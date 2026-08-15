#!/usr/bin/env bash
# build-image.sh <sha> [tag]
#
# Build the trial image for ONE pinned jigc source commit.
#
# The source is a `git archive` of that exact sha, never the working tree: it excludes
# target/ (which would make the build context many GB), and it makes the version under
# test a build argument rather than something an executor has to search for and can get
# wrong. protocol.md §5 arm 2's whole hazard is that two commits stamp 1.0.0-rc.10.
set -euo pipefail

SHA="${1:?usage: build-image.sh <sha> [tag]}"
TAG="${2:-jigc-gate:${SHA}}"
REPO="${JIGC_REPO:-/Users/maurice/projects/gherrink-jigc}"

FULL_SHA="$(git -C "$REPO" rev-parse "$SHA")"
STAMP="$(git -C "$REPO" show "${FULL_SHA}:Cargo.toml" | sed -n 's/^version = "\(.*\)"/\1/p' | head -1)"

echo "building ${TAG}"
echo "  sha     : ${FULL_SHA}"
echo "  stamped : ${STAMP}"
echo "  subject : $(git -C "$REPO" log -1 --format=%s "$FULL_SHA")"

CTX="$(mktemp -d)"
trap 'rm -rf "$CTX"' EXIT
mkdir -p "$CTX/src"
git -C "$REPO" archive "$FULL_SHA" | tar -x -C "$CTX/src"
cp "$(cd "$(dirname "$0")" && pwd)/Dockerfile" "$CTX/Dockerfile"

docker build -t "$TAG" --build-arg "JIGC_SHA=${FULL_SHA}" "$CTX"

echo
echo "built ${TAG} — verifying the binary reports the stamp the tree carries"
docker run --rm --entrypoint /usr/local/bin/jigc "$TAG" --version
