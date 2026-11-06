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
#   * the Dockerfile down to its `build` stage, comments and blank lines dropped — the
#     recipe of the layer itself. Below that line nothing reaches the layer.
# Each input is hashed alone and the three lines hashed together, so no two inputs can be
# confused by where one ends. Sourcing this file defines these and runs nothing, so the key
# is driven without docker: `check-base-layer.sh a_changed_lockfile_or_toolchain_file_builds_a_new_layer`.
sha256_stdin() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum; else shasum -a 256; fi | cut -d' ' -f1
}

# base_stage_text <dockerfile> — the Dockerfile above its `build` stage, as the key reads it.
# A Dockerfile without that line has no base to key, and says so rather than keying all of it.
base_stage_text() {
  local df="$1"
  if ! grep -q '^FROM \${JIGC_BASE} AS build$' "$df"; then
    echo "no 'FROM \${JIGC_BASE} AS build' line in $df — the base layer cannot be keyed" >&2
    return 1
  fi
  sed '/^FROM \${JIGC_BASE} AS build$/,$d' "$df" | grep -v -e '^[[:space:]]*#' -e '^[[:space:]]*$'
}

# base_key_of <repo> <sha> <dockerfile> — the base layer's key for the tree at <sha>.
base_key_of() {
  local repo="$1" sha="$2" df="$3" text f lines
  text="$(base_stage_text "$df")" || return 1
  git -C "$repo" cat-file -e "${sha}^{commit}" 2>/dev/null || {
    echo "no commit '${sha}' in ${repo}" >&2
    return 1
  }
  lines="dockerfile-above-the-build-stage $(printf '%s\n' "$text" | sha256_stdin)"
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
  BASE_KEY="$(base_key_of "$REPO" "$FULL_SHA" "$HERE/Dockerfile")"
  BASE_TAG="jigc-gate-base:${BASE_KEY}"
  BUILD_ARG="JIGC_SHA=${FULL_SHA}"

  echo "building ${TAG}"
  echo "  sha     : ${FULL_SHA}"
  echo "  stamped : ${STAMP}"
  echo "  subject : $(git -C "$REPO" log -1 --format=%s "$FULL_SHA")"
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
    docker build --target base -t "$BASE_TAG" --build-arg "JIGC_BASE_HASH=${BASE_KEY}" "$CTX"
    echo "base built in $((SECONDS - T0)) s"
  fi
  T0=$SECONDS
  docker build -t "$TAG" --build-arg "$BUILD_ARG" --build-arg "JIGC_BASE=${BASE_TAG}" "$CTX"
  echo "candidate built on ${BASE_TAG} in $((SECONDS - T0)) s"
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
