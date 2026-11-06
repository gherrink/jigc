#!/usr/bin/env bash
# build-image.sh <sha> [tag]
# build-image.sh --registry <version> [tag]
#
# Build the trial image for ONE pinned jigc: a source commit, or a published version.
#
# Source mode: the source is a `git archive` of that exact sha, never the working tree: it
# excludes target/ (which would make the build context many GB), and it makes the version
# under test a build argument rather than something an executor has to search for and can
# get wrong. protocol.md §5 arm 2's whole hazard is that two commits stamp 1.0.0-rc.10.
#
# Registry mode (M54 S14): `cargo install jigc --version <version> --locked` from
# crates.io, with no tree of ours in the image — what an adopter can install is exactly
# what the session runs. That makes it the blind trial's instrument. A bare version is
# cargo's exact match, and the stamp assertion below compares against it, so the mode
# takes a version and refuses a requirement (`^1.0.0-rc.1` would install whatever the
# registry resolves today).
#
# The kept base (K7): source mode builds on the image `jigc-gate-base:<key>` — the toolchain
# and the fetched crates for one toolchain file, one lockfile, one base recipe and one
# platform — building it first, with `--target base`, only when no image of that tag exists,
# and keeping it. After the build it asserts that the image records that key and that tag's
# layer, and fails otherwise (README → The kept base layer, which also says how to list and
# remove kept bases; `docker builder prune` does not).
#
# The repository the commits are read from is JIGC_REPO when it is set and not empty, else
# the repository this script sits in — never a path of one machine.
set -euo pipefail

# stamp_of <repo> <sha> — the version the tree at <sha> stamps into `jigc --version`.
# From M54 it is `jigc`'s own `[package]` version in crates/cli/Cargo.toml; an older sha
# carries `version.workspace = true` there, so the root `[workspace.package]` is read
# instead — and ONLY then, and only inside that section: once the root lost its version,
# the root's first `version = ` line is a dependency's (insta's "1").
# Sourcing this file defines the function and runs nothing, so it can be driven without
# docker: `bash -c '. build-image.sh; stamp_of <repo> <sha>'`.
stamp_of() {
  local repo="$1" sha="$2" v
  v="$({ git -C "$repo" show "${sha}:crates/cli/Cargo.toml" 2>/dev/null || true; } |
    sed -n '/^\[package\]/,/^\[/s/^version = "\(.*\)"$/\1/p')"
  if [ -z "$v" ]; then
    v="$(git -C "$repo" show "${sha}:Cargo.toml" |
      sed -n '/^\[workspace\.package\]/,/^\[/s/^version = "\(.*\)"$/\1/p')"
  fi
  printf '%s\n' "$v"
}

# The kept base layer (K7): the toolchain and the fetched crates, built once and kept as the
# image `jigc-gate-base:<key>` (the Dockerfile's `toolchain` and `base` stages). The key is
# what the layer is a function of, and nothing else, so a candidate with the same key builds
# on the kept layer and one with another key builds a new one:
#   * rust-toolchain.toml at the sha — the toolchain rustup installs (`absent` on a tree
#     without one, whose build takes the base image's own);
#   * Cargo.lock at the sha — the crates `cargo fetch` downloads. The manifests are not in
#     the key: under `--locked` a tree's resolve is its lockfile, so two trees with one
#     lockfile need one set of crates whatever their manifests say;
#   * the Dockerfile down to its `build` stage, comments and blank lines dropped but its
#     parser directives (`# syntax=`, `# escape=`, `# check=`) kept, since they change how
#     the recipe is read — the recipe of the layer itself. Below that line nothing reaches
#     the layer;
#   * the platform the layer is built for (`linux/arm64`), which both builds are then given
#     with `--platform`, so a kept base of another platform is never the one taken.
# Each input is hashed alone and the lines hashed together, so no two inputs can be
# confused by where one ends. Sourcing this file defines these and runs nothing, so the key
# is driven without docker — `tooling-tests/trial_image_base_key.rs` drives it in the gate.
sha256_stdin() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum; else shasum -a 256; fi | cut -d' ' -f1
}

# base_stage_text <dockerfile> — the Dockerfile above its `build` stage, as the key reads it.
# A Dockerfile without that line has no base to key, and says so rather than keying all of it.
# A parser directive is a `# <name>=<value>` line of a known name in the file's leading run of
# directives — docker stops reading them at the first other line, comment or blank — so only
# there is one kept; every other comment, a directive-shaped one further down included, and
# every blank line is dropped.
base_stage_text() {
  local df="$1"
  if ! grep -q '^FROM \${JIGC_BASE} AS build$' "$df"; then
    echo "no 'FROM \${JIGC_BASE} AS build' line in $df — the base layer cannot be keyed" >&2
    return 1
  fi
  awk '
    BEGIN { head = 1 }
    /^FROM \$\{JIGC_BASE\} AS build$/ { exit }
    head && tolower($0) ~ /^#[ \t]*(syntax|escape|check)[ \t]*=/ { print; next }
    { head = 0 }
    /^[ \t]*#/ || /^[ \t]*$/ { next }
    { print }
  ' "$df"
}

# docker_platform — the platform a build here is for: DOCKER_DEFAULT_PLATFORM when it is set
# and not empty, else the docker server's own `<os>/<arch>`.
docker_platform() {
  if [ -n "${DOCKER_DEFAULT_PLATFORM:-}" ]; then
    printf '%s\n' "$DOCKER_DEFAULT_PLATFORM"
  else
    docker version --format '{{.Server.Os}}/{{.Server.Arch}}'
  fi
}

# base_key_of <repo> <sha> <dockerfile> <platform> — the base layer's key for the tree at
# <sha>, built for <platform>. A key with no platform is refused rather than shared.
base_key_of() {
  local repo="$1" sha="$2" df="$3" platform="${4:-}" text f lines
  if [ -z "$platform" ]; then
    echo "no platform — the base layer is keyed by the platform it is built for" >&2
    return 1
  fi
  text="$(base_stage_text "$df")" || return 1
  git -C "$repo" cat-file -e "${sha}^{commit}" 2>/dev/null || {
    echo "no commit '${sha}' in ${repo}" >&2
    return 1
  }
  lines="dockerfile-above-the-build-stage $(printf '%s\n' "$text" | sha256_stdin)"
  lines+=$'\n'"platform ${platform}"
  for f in rust-toolchain.toml Cargo.lock; do
    if git -C "$repo" cat-file -e "${sha}:${f}" 2>/dev/null; then
      lines+=$'\n'"${f} $(git -C "$repo" show "${sha}:${f}" | sha256_stdin)"
    else
      lines+=$'\n'"${f} absent"
    fi
  done
  printf '%s\n' "$lines" | sha256_stdin
}

# default_repo — the repository the commits are read from: JIGC_REPO when it is set and not
# empty, else the repository this script sits in. Never a path of one machine.
default_repo() {
  if [ -n "${JIGC_REPO:-}" ]; then
    printf '%s\n' "$JIGC_REPO"
  else
    git -C "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" rev-parse --show-toplevel
  fi
}
if [ "${BASH_SOURCE[0]}" != "$0" ]; then return 0; fi

HERE="$(cd "$(dirname "$0")" && pwd)"

USAGE="usage: build-image.sh <sha> [tag] | build-image.sh --registry <version> [tag]"
REPO="$(default_repo)"

if [ "${1:-}" = "--registry" ]; then
  VERSION="${2:?$USAGE}"
  if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then
    echo "refusing: --registry takes one exact version, not '${VERSION}'" >&2
    echo "$USAGE" >&2
    exit 2
  fi
  TAG="${3:-jigc-gate:registry-${VERSION}}"
  STAMP="$VERSION"
  # The build context carries no source tree: the Dockerfile installs from the registry.
  BUILD_ARG="JIGC_REGISTRY=${VERSION}"

  echo "building ${TAG}"
  echo "  source  : crates.io, jigc ${VERSION} (cargo install --locked)"
  echo "  stamped : ${STAMP}"
else
  SHA="${1:?$USAGE}"
  TAG="${2:-jigc-gate:${SHA}}"

  FULL_SHA="$(git -C "$REPO" rev-parse "$SHA")"
  STAMP="$(stamp_of "$REPO" "$FULL_SHA")"
  PLATFORM="$(docker_platform)"
  BASE_KEY="$(base_key_of "$REPO" "$FULL_SHA" "$HERE/Dockerfile" "$PLATFORM")"
  BASE_TAG="jigc-gate-base:${BASE_KEY}"
  BUILD_ARG="JIGC_SHA=${FULL_SHA}"

  echo "building ${TAG}"
  echo "  sha     : ${FULL_SHA}"
  echo "  stamped : ${STAMP}"
  echo "  subject : $(git -C "$REPO" log -1 --format=%s "$FULL_SHA")"
  echo "  platform: ${PLATFORM}"
  echo "  base    : ${BASE_TAG}"
fi

# Created only after the arguments are settled: a usage error raised while the EXIT trap
# is set leaves the script at exit 0.
CTX="$(mktemp -d)"
trap 'rm -rf "$CTX"' EXIT
mkdir -p "$CTX/src"
if [ -n "${FULL_SHA:-}" ]; then   # source mode only; registry mode's src/ stays empty
  git -C "$REPO" archive "$FULL_SHA" | tar -x -C "$CTX/src"
fi
cp "$HERE/Dockerfile" "$CTX/Dockerfile"

if [ -n "${FULL_SHA:-}" ]; then
  # A kept base is used as it is; none is built from this tree, with --target base, and
  # kept under its key. Either way the line says which, and the build's own wall time.
  if docker image inspect "$BASE_TAG" >/dev/null 2>&1; then
    echo "base kept: ${BASE_TAG}"
  else
    echo "base not kept: building ${BASE_TAG} from this tree (toolchain and crates)"
    T0=$SECONDS
    docker build --platform "$PLATFORM" --target base -t "$BASE_TAG" \
      --build-arg "JIGC_BASE_HASH=${BASE_KEY}" "$CTX"
    echo "base built in $((SECONDS - T0)) s"
  fi
  T0=$SECONDS
  docker build --platform "$PLATFORM" -t "$TAG" --build-arg "$BUILD_ARG" \
    --build-arg "JIGC_BASE=${BASE_TAG}" "$CTX"
  echo "candidate built on ${BASE_TAG} in $((SECONDS - T0)) s"

  # The base the build used is the one keyed: the image records the key this script computed
  # and the mark of the layer the tag names now, and is of the platform keyed. Two builds of
  # one key at once, or a tag removed or replaced between the inspect above and the build,
  # would otherwise pass unseen.
  GOT_BASE="$(docker run --rm --entrypoint cat "$TAG" /usr/local/share/jigc-image/base 2>/dev/null || true)"
  KEPT_BASE="$(docker run --rm --entrypoint cat "$BASE_TAG" /usr/local/share/jigc-base/record 2>/dev/null || true)"
  GOT_KEY="$(sed -n 's/^key //p' <<<"$GOT_BASE")"
  GOT_LAYER="$(sed -n 's/^layer //p' <<<"$GOT_BASE")"
  KEPT_LAYER="$(sed -n 's/^layer //p' <<<"$KEPT_BASE")"
  GOT_PLATFORM="$(docker image inspect "$TAG" --format '{{.Os}}/{{.Architecture}}')"
  WANT_PLATFORM="$(cut -d/ -f1-2 <<<"$PLATFORM")"
  if [ "$GOT_KEY" != "$BASE_KEY" ] || [ -z "$GOT_LAYER" ] || [ "$GOT_LAYER" != "$KEPT_LAYER" ] \
    || [ "$GOT_PLATFORM" != "$WANT_PLATFORM" ]; then
    echo "MISMATCH: ${TAG} was not built on ${BASE_TAG} for ${WANT_PLATFORM}:" >&2
    echo "  the image records key '${GOT_KEY}' and layer '${GOT_LAYER}', for ${GOT_PLATFORM}" >&2
    echo "  the tag holds layer '${KEPT_LAYER}'" >&2
    exit 1
  fi
  echo "base checked: ${TAG} records ${BASE_TAG}, layer ${GOT_LAYER}, ${GOT_PLATFORM}"
else
  docker build -t "$TAG" --build-arg "$BUILD_ARG" "$CTX"
fi

echo
echo "built ${TAG} — verifying the binary reports the stamp its source carries"
GOT="$(docker run --rm --entrypoint /usr/local/bin/jigc "$TAG" --version 2>&1)"
echo "$GOT"
# Asserted, not printed. The sentence above claimed a verification the script never
# performed: STAMP was extracted, echoed, and compared to nothing.
if [ "$GOT" != "jigc ${STAMP}" ]; then
  echo "MISMATCH: the source stamps '${STAMP}' but the built binary says '${GOT}'" >&2
  exit 1
fi
