#!/usr/bin/env bash
# check-corpus.sh <corpus-dir> [--clean-prose]
#
# Assert a freshly instantiated corpus is in its expected STARTING state, before a
# blind session is ever pointed at it.
#
# Why this exists: RC-pre-1.0 stated exactly this checklist as prose — "Verified
# pre-trial, all six: 7 commits, clean tree, `.jigc` absent, no `.claude/` or
# `CLAUDE.md` residue, `README.md` the only tracked `.md`, `npm test` green 23/23" —
# and nothing in the repo ever checked it. Every `git rev-list --count` in this
# repository is a *delta* measurement inside an evidence driver; none is a precondition
# gate. This is "a plant assumed to fire is not a plant", applied to the corpus itself.
#
# A corpus that fails any bar below is not a trial corpus. A worker pointed at one that
# already carries `.jigc/` is not doing a cold start, and nothing it does afterwards
# measures what the protocol says it measures.
#
# Shape follows the in-repo idiom (M35/harness/check-seed.sh): ok/fail counters, a
# skip branch for an optional tool, non-zero exit on any failure.
set -uo pipefail

CORPUS="${1:?usage: check-corpus.sh <corpus-dir> [--clean-prose]}"
WANT_CLEAN_PROSE=0
[ "${2:-}" = "--clean-prose" ] && WANT_CLEAN_PROSE=1

EXPECT_COMMITS=7
EXPECT_TESTS=23

PASS=0; FAIL=0
ok()   { echo "  PASS  $1"; PASS=$((PASS+1)); }
bad()  { echo "  FAIL  $1"; FAIL=$((FAIL+1)); }
skip() { echo "  SKIP  $1"; }

[ -d "$CORPUS/.git" ] || { echo "not a git repository: $CORPUS" >&2; exit 2; }
cd "$CORPUS"

echo "checking corpus at $CORPUS"

# --- the history is the one the template builds -------------------------------------
N="$(git rev-list --count HEAD 2>/dev/null || echo 0)"
[ "$N" = "$EXPECT_COMMITS" ] && ok "$EXPECT_COMMITS commits" || bad "expected $EXPECT_COMMITS commits, found $N"

DIRT="$(git status --porcelain)"
[ -z "$DIRT" ] && ok "working tree clean" || bad "working tree dirty:
$DIRT"

# --- it has never met jigc ----------------------------------------------------------
# The "from nothing" premise is exact, not approximated. A leftover .jigc/ from a
# rehearsal is the single most likely way to hand a worker a corpus that is not naive.
RESIDUE=""
for p in .jigc .claude CLAUDE.md AGENT.md; do
  [ -e "$p" ] && RESIDUE="$RESIDUE $p"
done
[ -z "$RESIDUE" ] && ok "no jigc/adapter residue" || bad "corpus is not naive — found:$RESIDUE"

# --- README.md is the only tracked markdown -----------------------------------------
# A tracked .md is an ingest candidate; a stray one changes what `jigc ingest` reports
# and silently changes the adoption arm's subject.
MDS="$(git ls-files '*.md' '*.markdown')"
[ "$MDS" = "README.md" ] && ok "README.md is the only tracked .md" || bad "tracked .md files are:
$MDS"

# --- the suite runs, and runs green --------------------------------------------------
# Not decoration: `maps-to-test` has 0 writes in its entire history, and these files are
# the only place it has ever had to point at.
if command -v node >/dev/null 2>&1; then
  # --test-reporter=tap is pinned deliberately: node's DEFAULT reporter differs by
  # version (24 emits "ℹ pass 23", older emits TAP's "# pass 23"), and this script runs
  # both on the host and inside the trial container, which carry different nodes.
  # Parsing whatever the local default happens to be would make the check pass or fail
  # on the runner rather than on the corpus.
  OUT="$(node --test --test-reporter=tap test/*.test.ts 2>&1)"
  GOT_PASS="$(printf '%s\n' "$OUT" | sed -nE 's/^# pass[[:space:]]+([0-9]+)$/\1/p' | tail -1)"
  GOT_FAIL="$(printf '%s\n' "$OUT" | sed -nE 's/^# fail[[:space:]]+([0-9]+)$/\1/p' | tail -1)"
  if [ "$GOT_PASS" = "$EXPECT_TESTS" ] && [ "$GOT_FAIL" = "0" ]; then
    ok "node --test: $GOT_PASS passed, 0 failed"
  else
    bad "node --test: expected $EXPECT_TESTS passed / 0 failed, got ${GOT_PASS:-?} / ${GOT_FAIL:-?}"
  fi
else
  skip "node not on PATH — suite not run (the corpus still ships it)"
fi

# --- the symbols the doc-code probes bind against ------------------------------------
# The corpus exists so `symbol-exists` has real targets. If these moved, an anchor a
# worker writes will dangle for a reason that has nothing to do with the worker.
MISSING=""
for sym in "class IngestQueue" "class MemoryStore" "prune(" "export function rollup"; do
  grep -rqF -- "$sym" src/ || MISSING="$MISSING '$sym'"
done
[ -z "$MISSING" ] && ok "doc-code anchor symbols present" || bad "missing symbols:$MISSING"

# --- prose/code consistency, only when it was asked for ------------------------------
if [ "$WANT_CLEAN_PROSE" = 1 ]; then
  HITS="$(grep -rniE "in front of|long-term store" README.md package.json src/ 2>/dev/null || true)"
  [ -z "$HITS" ] && ok "prose matches the code (no forwarding claim)" || bad "--clean-prose was expected, but the contradiction survives:
$HITS"
else
  skip "prose contradiction not checked (corpus instantiated without --clean-prose)"
fi

echo
echo "$PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ] || exit 1
