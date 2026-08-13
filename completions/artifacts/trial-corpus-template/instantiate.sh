#!/usr/bin/env bash
# Instantiate one trial corpus from this template, with a plausible 7-commit history.
#
#   ./instantiate.sh <dest-dir> <product-name> [tagline]
#
# See README.md in this directory for why the template exists and what its
# properties are load-bearing for.
set -euo pipefail

TEMPLATE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST="${1:?usage: instantiate.sh <dest-dir> <product-name> [tagline]}"
PRODUCT="${2:?usage: instantiate.sh <dest-dir> <product-name> [tagline]}"
TAGLINE="${3:-A rollup cache for time-series samples — windowed aggregates in front of whatever long-term store you already have.}"

if [ -e "$DEST" ]; then
  echo "refusing to overwrite existing $DEST" >&2
  exit 1
fi

mkdir -p "$DEST/src" "$DEST/test"
cp "$TEMPLATE"/src/*.ts "$DEST/src/"
cp "$TEMPLATE"/test/*.ts "$DEST/test/"
cp "$TEMPLATE/gitignore.template" "$DEST/.gitignore"

for f in package.json README.md; do
  sed -e "s|__PRODUCT__|$PRODUCT|g" -e "s|__TAGLINE__|$TAGLINE|g" \
    "$TEMPLATE/$f.template" > "$DEST/$f"
done

cd "$DEST"
git init -q -b main

commit() { # commit <message> <path>...
  local msg="$1"; shift
  git add -- "$@"
  git commit -q -m "$msg"
}

# Every file is already on disk; the history is built by staging them in
# dependency order, so each commit is a coherent working tree.
commit "chore: project skeleton" package.json README.md .gitignore
commit "feat: sample validation with a named offending field" src/validate.ts test/validate.test.ts
commit "feat: injectable clock and env-read config" src/clock.ts src/config.ts
commit "feat: bounded ingest queue and wire-line parser" src/ingest.ts test/ingest.test.ts
commit "feat: in-memory per-series sample store" src/store.ts
commit "feat: aligned window rollups and summaries" src/rollup.ts src/summary.ts test/rollup.test.ts
commit "feat: router and service wiring" src/router.ts src/index.ts

echo "--- $PRODUCT at $DEST"
git log --oneline
echo "--- working tree (expect empty) ---"
git status --short
