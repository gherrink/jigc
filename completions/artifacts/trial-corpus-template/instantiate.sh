#!/usr/bin/env bash
# Instantiate one trial corpus from this template, with a plausible 7-commit history.
#
#   ./instantiate.sh [--clean-prose] <dest-dir> <product-name> [tagline]
#
# `--clean-prose` removes the known prose↔code contradiction (README.md → A known
# wart) in **all three** places it lives. Without the flag the corpus is
# byte-identical to what the pre-1.0.0 trial ran on, so that trial stays reproducible.
#
# Why a flag rather than "pass a different tagline": the tagline argument reaches
# `package.json` and `README.md` only, while the same claim is also hard-coded in
# `src/store.ts` — the file a worker actually reads before writing a spec, and the one
# G3 read to find the contradiction. A caller who followed the old advice got a corpus
# that still contradicted itself in the place that mattered most.
#
# See README.md in this directory for why the template exists and what its
# properties are load-bearing for.
set -euo pipefail

CLEAN_PROSE=0
if [ "${1:-}" = "--clean-prose" ]; then
  CLEAN_PROSE=1
  shift
fi

TEMPLATE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST="${1:?usage: instantiate.sh [--clean-prose] <dest-dir> <product-name> [tagline]}"
PRODUCT="${2:?usage: instantiate.sh [--clean-prose] <dest-dir> <product-name> [tagline]}"

WART_TAGLINE="A rollup cache for time-series samples — windowed aggregates in front of whatever long-term store you already have."
CLEAN_TAGLINE="An in-memory rollup buffer for time-series samples — windowed aggregates over a bounded recent window."

if [ "$CLEAN_PROSE" = 1 ]; then
  TAGLINE="${3:-$CLEAN_TAGLINE}"
else
  TAGLINE="${3:-$WART_TAGLINE}"
fi

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

if [ "$CLEAN_PROSE" = 1 ]; then
  # The third site: src/store.ts's header comment. Rewritten rather than templated so
  # this directory stays a runnable project in its own right (`node --test test/*.ts`).
  sed -i.bak \
    -e 's|^ \* Deliberately not durable: this service is a rollup cache in front of whatever$| * Deliberately not durable: this is a bounded in-memory buffer over a recent|' \
    -e 's|^ \* long-term store the caller already has\.$| * window — nothing is forwarded anywhere, and nothing reads it back.|' \
    "$DEST/src/store.ts"
  rm -f "$DEST/src/store.ts.bak"

  # Fenced, not hoped for — and deliberately AFTER the two rendered files exist, so it
  # actually sees all three sites. A silent no-op here (someone reworded the header, or
  # the fence ran too early) would hand a trial a corpus it believes is clean and is
  # not, which is the exact failure this flag exists to fix.
  if grep -rniE "in front of|long-term store" \
       "$DEST/src" "$DEST/README.md" "$DEST/package.json" >&2; then
    echo "--clean-prose left the contradiction in place (matches above) — fix the sed" >&2
    exit 1
  fi
fi

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
