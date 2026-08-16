#!/usr/bin/env bash
# b1-staged.sh <corpus-dir> [--force]
#
# B1 plant 2 — TWO FILES STAGED BEFORE THE FIRST MINT, so the carryover gate has
# something to catch.
#
# Purpose (protocol.md §4, B1): M43's carryover gate snapshots the staged set at every
# task-minting door and `finalize` refuses to carry it, one blocking
# `finalize.carried-staged` PER PATH, each with its own unstage route. Two paths, of two
# different shapes:
#
#   scripts/retention-sweep.sh   a NEW file      (index status A)
#   src/router.ts                a MODIFIED file (index status M)
#
# Two shapes rather than two of one, because the gate's finding is emitted per path and
# the routes differ in what they restore — a per-path claim proven on one shape is not
# proven.
#
# RUN THIS ON THE HOST CORPUS DIRECTORY, AFTER `b1-hook.sh`, BEFORE `run-session.sh`.
# Nothing here is committed: the plant IS the index state, and `docker cp` of the corpus
# carries `.git/index` with it.
set -euo pipefail

usage() { echo "usage: b1-staged.sh <corpus-dir> [--force]" >&2; exit 2; }

CORPUS=""
FORCE=0
while [ $# -gt 0 ]; do
  case "$1" in
    --force) FORCE=1; shift ;;
    -*)      echo "unknown option: $1" >&2; usage ;;
    *)       [ -n "$CORPUS" ] && usage; CORPUS="$1"; shift ;;
  esac
done
[ -n "$CORPUS" ] || usage

refuse() { echo "refusing: $1" >&2; exit 1; }

[ -d "$CORPUS/.git" ] || refuse "$CORPUS is not a git repository"
cd "$CORPUS"

# --- the corpus must be in the state this plant assumes ------------------------------
[ -e .jigc ] && [ "$FORCE" = 0 ] && \
  refuse ".jigc/ is present — the staged set must predate the first mint, and it does not"

STAGED="$(git diff --cached --name-only)"
[ -n "$STAGED" ] && [ "$FORCE" = 0 ] && refuse "the index is already dirty:
$STAGED"

[ -e scripts/retention-sweep.sh ] && [ "$FORCE" = 0 ] && \
  refuse "scripts/retention-sweep.sh already exists — this corpus has been planted before"

git ls-files --error-unmatch src/router.ts >/dev/null 2>&1 || \
  refuse "src/router.ts is not tracked — this is not a trial-corpus-template corpus"

git diff --quiet -- src/router.ts || [ "$FORCE" = 1 ] || \
  refuse "src/router.ts is already modified — this corpus has been planted or worked in"

grep -q '/healthz' src/router.ts && [ "$FORCE" = 0 ] && \
  refuse "src/router.ts already carries the planted route"

# --- path 1: a new, untracked-then-staged file ----------------------------------------
mkdir -p scripts
cat > scripts/retention-sweep.sh <<'SWEEP'
#!/bin/sh
# Drop series whose newest sample is older than the retention window.
#
# Meant for cron. Reads the same RETENTION_MS the service reads, so the two cannot
# drift apart, and prints what it would drop unless --apply is passed.
set -eu

retention="${RETENTION_MS:-3600000}"
apply=0
[ "${1:-}" = "--apply" ] && apply=1

echo "retention window: ${retention}ms (apply=${apply})"
echo "TODO: call the summary endpoint per series and drop the empty ones"
SWEEP
chmod +x scripts/retention-sweep.sh

# --- path 2: an in-flight modification to a tracked file ------------------------------
# Inserted with awk rather than sed so the multi-line insert stays readable and portable
# (BSD sed's `i\` differs from GNU's). The anchor is the 404 fallthrough, which is the
# last branch of `dispatch`.
awk '
  /return \{ status: 404, body: `no route for/ && !done {
    print "    if (method === \"GET\" && path === \"/healthz\") {"
    print "      return { status: 200, body: \"ok\" };"
    print "    }"
    done = 1
  }
  { print }
' src/router.ts > src/router.ts.plant && mv src/router.ts.plant src/router.ts

grep -q '/healthz' src/router.ts || refuse "the router edit did not apply — check the awk anchor"

# --- stage both, commit nothing -------------------------------------------------------
git add -- scripts/retention-sweep.sh src/router.ts

echo "planted the pre-mint staged set in $CORPUS"
git status --porcelain
echo
echo "  staged paths     : $(git diff --cached --name-only | tr '\n' ' ')"
echo "  HEAD             : $(git log -1 --format='%h %s')"
echo "  (nothing was committed — the index IS the plant)"
