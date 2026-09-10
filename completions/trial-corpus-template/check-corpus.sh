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
EXPECT_TESTS=24

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

# `git status` failing must not read as "clean". Empty output and a broken index /
# dubious-ownership refusal are the same empty string, and this script runs both on the
# host and inside the container, where `detected dubious ownership in repository at
# '/work'` is reachable in normal use.
if DIRT="$(git status --porcelain 2>&1)"; then
  [ -z "$DIRT" ] && ok "working tree clean" || bad "working tree dirty:
$DIRT"
else
  bad "git status failed — cannot certify the tree: $DIRT"
fi

# --- it has never met jigc ----------------------------------------------------------
# The "from nothing" premise is exact, not approximated. A leftover from a rehearsal is
# the single most likely way to hand a worker a corpus that is not naive.
#
# `.git/hooks/pre-commit` is the one that matters most and the one a human cleanup
# misses: `jigc setup` installs it (crates/cli/src/setup.rs → install_precommit_hook),
# it lives INSIDE .git/, so it survives `git reset --hard` and `git clean -fdx`, and it
# is invisible to `git status`. A corpus carrying it fires `jigc validate` on the
# worker's first commit and prints doc↔code findings — contamination on the exact
# conduct axis this trial measures, in a corpus otherwise certified naive.
RESIDUE=""
for p in .jigc .claude CLAUDE.md AGENT.md .mcp.json CLAUDE.local.md AGENTS.md; do
  [ -e "$p" ] && RESIDUE="$RESIDUE $p"
done
HOOKS_DIR="$(git rev-parse --git-path hooks 2>/dev/null || echo .git/hooks)"
for h in "$HOOKS_DIR"/*; do
  [ -f "$h" ] || continue
  case "$h" in *.sample) continue;; esac
  RESIDUE="$RESIDUE $h"
done
[ -z "$RESIDUE" ] && ok "no jigc/adapter residue (hooks dir included)" \
  || bad "corpus is not naive — found:$RESIDUE"

# --- no path back to a real repository, and no redirected hooks ----------------------
# A remote means a worker's `git push` can reach something real. `core.hooksPath` is how
# a rehearsal's hook survives even an empty .git/hooks.
# Local remote URLs only — NOT `git remote -v`, which lists a bare name for any
# `remote.<name>.*` key inherited from the operator's ~/.gitconfig. A global
# `remote.origin.prune = true` makes every fresh repo report an "origin" with no URL,
# which is not a remote and cannot be pushed to.
REMOTES="$(git config --local --get-regexp '^remote\..*\.url' 2>/dev/null || true)"
[ -z "$REMOTES" ] && ok "no git remote URL" || bad "corpus has a real remote — a worker could push:
$REMOTES"

HP="$(git config --get core.hooksPath 2>/dev/null || true)"
[ -z "$HP" ] && ok "core.hooksPath unset" || bad "core.hooksPath is set to '$HP'"

# --- the history is the template's, not a rehearsal's ---------------------------------
# `git reflog` shows a rehearsal's setup commit even after `git reset --hard`, and a
# worker that runs `git reflog` for orientation would see it.
REFLOG_N="$(git reflog 2>/dev/null | wc -l | tr -d ' ')"
[ "$REFLOG_N" = "$EXPECT_COMMITS" ] && ok "reflog carries $EXPECT_COMMITS entries (no rehearsal residue)" \
  || bad "reflog has $REFLOG_N entries, expected $EXPECT_COMMITS — this corpus has been worked in"

BRANCH="$(git rev-parse --abbrev-ref HEAD 2>/dev/null)"
[ "$BRANCH" = "main" ] && ok "on main" || bad "branch is '$BRANCH', expected main"

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

# --- and they are REACHED, not merely present (PT-D) ---------------------------------
# Presence is not the property a plant needs. Through three trials `IngestQueue.push()`
# was called from no live path and `tick()` from nothing at all: `MemoryStore.prune` was
# dead for the same reason. Four workers in the RC-m50 trial found it and two filed it as
# a deferral — worker budget spent on the fixture instead of on the product, and worse, a
# plant whose subject is dead code has "the worker fixes the code instead" as its
# falsifier. Plant E's whole subject is this queue's overflow policy.
#
# Driven, not grepped: a grep proves a call site is written down, and what a plant needs
# is that the behaviour happens. The probe POSTs through the router, asserts the store is
# still EMPTY (the sample is buffered, not stored), ticks, and asserts it landed — so it
# fails both ways, when the queue is bypassed and when the drain is severed.
#
# The probe file lives OUTSIDE the corpus, in its own mktemp dir: writing it inside would
# dirty the tree that bar 2 above just certified clean.
if command -v node >/dev/null 2>&1; then
  PROBE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/corpus-reach.XXXXXX")"
  ABS="$(pwd -P)"
  cat > "$PROBE_DIR/reach.ts" <<PROBE
import { createService } from "$ABS/src/index.ts";
const svc = createService(
  { windowMs: 1000, retentionMs: 60000, maxSamples: 8 } as any,
  { now: () => 1000 } as any,
);
svc.router.dispatch("POST", "/samples", "cpu.load 0.5 1000");
console.log("before:" + JSON.stringify(svc.store.series()));
svc.tick();
console.log("after:" + JSON.stringify(svc.store.series()));
PROBE
  REACH="$(node "$PROBE_DIR/reach.ts" 2>&1)"
  if printf '%s\n' "$REACH" | grep -qx 'before:\[\]' \
     && printf '%s\n' "$REACH" | grep -qx 'after:\["cpu.load"\]'; then
    ok "the queue is on the live write path (POST buffers; tick lands it)"
  else
    bad "a plant symbol is unreachable — POST/tick did not go through the queue:
$REACH"
  fi
else
  skip "node not on PATH — reachability not driven (the corpus still ships the wiring)"
fi

# --- prose/code consistency, only when it was asked for ------------------------------
if [ "$WANT_CLEAN_PROSE" = 1 ]; then
  # Wider than the two phrases instantiate.sh's sed targets, so this is an independent
  # check rather than a restatement of the fix — a header reworded to claim forwarding
  # in different words would no-op the sed and pass its fence, and must still fail here.
  #
  # But NOT so wide that it matches the replacement text. The clean header itself reads
  # "Deliberately not durable … nothing is forwarded anywhere", so "durable"/"forward"
  # as bare terms make this check fail on a correct corpus — which it did, on the first
  # attempt. Match claim-shaped phrases, not their negations.
  HITS="$(grep -rniE "in front of|long-term store|rollup cache|refill|upstream (store|reader)" \
            README.md package.json src/ 2>/dev/null || true)"
  [ -z "$HITS" ] && ok "prose matches the code (no forwarding/durability claim)" || bad "--clean-prose was expected, but a forwarding-shaped claim survives:
$HITS"
else
  skip "prose contradiction not checked (corpus instantiated without --clean-prose)"
fi

echo
echo "$PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ] || exit 1
