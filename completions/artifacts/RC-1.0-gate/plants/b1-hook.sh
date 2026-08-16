#!/usr/bin/env bash
# b1-hook.sh <corpus-dir> [--date <git-date>] [--force]
#
# B1 plant 1 — a REJECTING `pre-commit` hook under `core.hooksPath`.
#
# Purpose (protocol.md §4, B1): exercise M47's survivable hook-rejection frame on a
# doc-promoting `jigc task finalize`. The hook refuses any commit whose staged set
# touches `docs/`, which is where a promoted ADR lands, and is released by the
# operator with `b1-release-hook.sh`.
#
# WHY `core.hooksPath` AND NOT `.git/hooks`: `resolve_hooks_dir`
# (crates/cli/src/setup.rs) resolves the hooks dir through `git rev-parse --git-path
# hooks`, so `jigc setup` installs its own warn-only block into THIS directory and
# `SetupSummary.hook_file` prints what it resolved. A plant in `.git/hooks` would be
# bypassed entirely by a corpus whose `core.hooksPath` points elsewhere, and would not
# exercise the resolution M47/M48 changed.
#
# The hook is a FOREIGN hook. `jigc setup` splices its own warn-only block in after the
# shebang and leaves this body verbatim below it, so the foreign body still owns the
# final exit (crates/cli/src/setup.rs → wrapped_managed_block). That is deliberate: the
# rejection under test is this hook's, not jigc's.
#
# RUN THIS ON THE HOST CORPUS DIRECTORY BEFORE `run-session.sh` COPIES IT IN.
# `.git/config` travels with the corpus, so `core.hooksPath` survives the copy.
#
# It is applied AFTER `check-corpus.sh` has gated the corpus: the plant deliberately
# breaks three of that script's naive bars (commit count, reflog count, hooks-dir
# residue, `core.hooksPath` unset). Gate first, plant second, never the other way round.
set -euo pipefail

usage() { echo "usage: b1-hook.sh <corpus-dir> [--date <git-date>] [--force]" >&2; exit 2; }

CORPUS=""
DATE=""
FORCE=0
while [ $# -gt 0 ]; do
  case "$1" in
    --date)  DATE="${2:?--date needs a value}"; shift 2 ;;
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
# Each bar is a way the plant would silently not be the plant.
git rev-parse HEAD >/dev/null 2>&1 || refuse "no commits in $CORPUS"

[ -e .jigc ] && [ "$FORCE" = 0 ] && \
  refuse ".jigc/ is present — B1 is a COLD START and this hook must predate jigc setup"

HP="$(git config --get core.hooksPath 2>/dev/null || true)"
[ -n "$HP" ] && [ "$FORCE" = 0 ] && \
  refuse "core.hooksPath is already set to '$HP' — this corpus has been planted before"

[ -e .githooks ] && [ "$FORCE" = 0 ] && \
  refuse ".githooks/ already exists — this corpus has been planted before"

if DIRT="$(git status --porcelain 2>&1)"; then
  [ -n "$DIRT" ] && [ "$FORCE" = 0 ] && refuse "working tree is dirty:
$DIRT"
else
  refuse "git status failed — cannot certify the tree: $DIRT"
fi

# The plant commit sits flush with the corpus's own history by default. The template
# commits carry the instantiation time, so a plant back-dated to some earlier "realistic"
# day would land as a commit whose author date PRECEDES its parents — visible in
# `git log --format='%ad %s'` and exactly the kind of seam RC-pre-1.0's worker spotted.
# Defaulting to HEAD's own author date makes the plant indistinguishable from the rest of
# the history; `--date` overrides it if a trial wants otherwise.
[ -n "$DATE" ] || DATE="$(git log -1 --format=%aI HEAD)"

# --- the hook -------------------------------------------------------------------------
mkdir -p .githooks
cat > .githooks/pre-commit <<'HOOK'
#!/bin/sh
# Docs gate.
#
# Everything under docs/ is reviewed before it lands. Until the reviewer records a
# sign-off, a commit that touches docs/ is refused. The sign-off marker lives in the
# git dir so it is never committed by accident and never shows up in `git status`.

approved="$(git rev-parse --git-dir)/docs-approved"

if [ ! -f "$approved" ] && \
   git diff --cached --name-only --diff-filter=ACMRD | grep -q '^docs/'; then
	echo 'docs-gate: refusing this commit — it touches docs/ and the docs review has not signed off.' >&2
	echo 'docs-gate: nothing was committed. Ask the docs reviewer to sign off, then commit again.' >&2
	exit 1
fi

exit 0
HOOK
chmod +x .githooks/pre-commit

git config core.hooksPath .githooks

# Explicit pathspec (protocol.md §8 rule 3): a partial commit can never sweep another
# process's in-flight index into the operator's commit. Here the tree is clean, but the
# rule is applied uniformly so no plant is the exception that gets it wrong.
git add -- .githooks/pre-commit
GIT_AUTHOR_DATE="$DATE" GIT_COMMITTER_DATE="$DATE" \
  git commit -q -m "chore: refuse doc commits until the docs review signs off" \
  -- .githooks/pre-commit

# --- what landed ----------------------------------------------------------------------
echo "planted the docs-gate hook in $CORPUS"
echo "  hooks dir        : $(git rev-parse --git-path hooks)"
echo "  release marker   : $(git rev-parse --git-dir)/docs-approved  (absent = rejecting)"
echo "  commit           : $(git log -1 --format='%h %ad %s' --date=iso)"
echo "  working tree     : $(git status --porcelain | wc -l | tr -d ' ') entries (expect 0)"
echo
echo "release it with: b1-release-hook.sh <repo-dir>"
echo "NOTE: the session runs on the COPY inside the container — release it there:"
echo "      docker exec -it <cid> bash -lc 'touch /work/.git/docs-approved'"
