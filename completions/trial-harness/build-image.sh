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
if [ "${BASH_SOURCE[0]}" != "$0" ]; then return 0; fi

USAGE="usage: build-image.sh <sha> [tag] | build-image.sh --registry <version> [tag]"
REPO="${JIGC_REPO:-/Users/maurice/projects/gherrink-jigc}"

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
  BUILD_ARG="JIGC_SHA=${FULL_SHA}"

  echo "building ${TAG}"
  echo "  sha     : ${FULL_SHA}"
  echo "  stamped : ${STAMP}"
  echo "  subject : $(git -C "$REPO" log -1 --format=%s "$FULL_SHA")"
fi

# Created only after the arguments are settled: a usage error raised while the EXIT trap
# is set leaves the script at exit 0.
CTX="$(mktemp -d)"
trap 'rm -rf "$CTX"' EXIT
mkdir -p "$CTX/src"
if [ -n "${FULL_SHA:-}" ]; then   # source mode only; registry mode's src/ stays empty
  git -C "$REPO" archive "$FULL_SHA" | tar -x -C "$CTX/src"
fi
cp "$(cd "$(dirname "$0")" && pwd)/Dockerfile" "$CTX/Dockerfile"

docker build -t "$TAG" --build-arg "$BUILD_ARG" "$CTX"

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
