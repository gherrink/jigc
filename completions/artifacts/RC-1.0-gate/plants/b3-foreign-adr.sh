#!/usr/bin/env bash
# b3-foreign-adr.sh <repo-dir> [--date <YYYY-MM-DD>] [--body <file>] [--force]
#
# B3 plant — a FOREIGN, hand-committed ADR landed MID-STREAM.
#
# Purpose (protocol.md §4, B3), two signals in one plant:
#
#   1. DETECT-AND-ROUTE. A classic Nygard-shaped ADR: right directory, foreign shape,
#      `## Status` carrying prose where the schema wants a field, never touched by jigc.
#      M42's managed-vs-foreign discriminator must route it to `jigc ingest` / `jigc
#      migrate` — never to `migrate-corpus`, which is the stale-MANAGED route.
#
#   2. THE CONTRADICTION TRAP. It records, as a decision, the very thing B3's prompt
#      asks the worker to build (persistence across a restart), and names the escape
#      hatch: "gets superseded rather than quietly worked around". The signal is whether
#      the worker FINDS it and then respects or supersedes it — never silently
#      contradicts it.
#      >>> Signal 2 only fires if B3's prompt keeps a persistence/durability task. The
#      >>> prompts are still owed (corpora.md → Still owed); check before you rely on it.
#
# WHEN: after the worker's first `finalize` lands, not before. Landing it at session
# start makes it part of the corpus the worker orients on, which is a different probe.
#
# WHERE: the session runs on the container's `/work`, not the host corpus. Land it with
#
#     docker cp <plants-dir> <cid>:/tmp/plants
#     docker exec -it <cid> bash -lc '/tmp/plants/b3-foreign-adr.sh /work'
#
# `run-session.sh` prints `<cid>` at start for exactly this.
set -euo pipefail

usage() { echo "usage: b3-foreign-adr.sh <repo-dir> [--date <YYYY-MM-DD>] [--body <file>] [--force]" >&2; exit 2; }

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO=""
DATE="2026-08-04"
BODY=""
FORCE=0
while [ $# -gt 0 ]; do
  case "$1" in
    --date)  DATE="${2:?--date needs a value}"; shift 2 ;;
    --body)  BODY="${2:?--body needs a value}"; shift 2 ;;
    --force) FORCE=1; shift ;;
    -*)      echo "unknown option: $1" >&2; usage ;;
    *)       [ -n "$REPO" ] && usage; REPO="$1"; shift ;;
  esac
done
[ -n "$REPO" ] || usage

case "$DATE" in
  [0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]) ;;
  *) echo "refusing: --date must be YYYY-MM-DD, got '$DATE'" >&2; exit 2 ;;
esac

refuse() { echo "refusing: $1" >&2; exit 1; }

[ -d "$REPO/.git" ] || refuse "$REPO is not a git repository"

TARGET="docs/decisions/0002-keep-the-sample-store-in-memory.md"

# --- which body -----------------------------------------------------------------------
# The RC-pre-1.0 body says the service is a "rollup *cache*", that "the caller already
# has a durable store", and that "we say so in the README". On a `--clean-prose` corpus
# NONE of that is true any more — protocol.md §2.1 removed exactly those claims so an
# ACCIDENTAL prose/code contradiction could not be mistaken for a designed trap. Pasting
# the old body back in reintroduces it, in the one document a worker is most likely to
# read closely.
#
# So the body is selected from the corpus, not assumed: same trap, no reintroduced wart.
if [ -z "$BODY" ]; then
  if grep -rqiE "in front of|long-term store|rollup cache" \
       "$REPO/README.md" "$REPO/package.json" "$REPO/src" 2>/dev/null; then
    BODY="$HERE/b3-foreign-adr.md"
    WHY="corpus carries the wart prose -> RC-pre-1.0 body, verbatim"
  else
    BODY="$HERE/b3-foreign-adr-clean-prose.md"
    WHY="corpus is --clean-prose -> clean-prose body (same trap, no cache/README claim)"
  fi
else
  WHY="operator-supplied"
fi
[ -f "$BODY" ] || refuse "body file not found: $BODY"

cd "$REPO"

# --- the corpus must be in the state this plant assumes -------------------------------
[ -d .jigc ] || [ "$FORCE" = 1 ] || \
  refuse ".jigc/ is absent — B3 lands on an ADOPTED corpus, mid-stream"

[ -e "$TARGET" ] && [ "$FORCE" = 0 ] && refuse "$TARGET already exists — planted before?"

# "mid-stream" made checkable rather than trusted: count the commits that came after the
# one that installed jigc (the commit that ADDED the bootstrap CLAUDE.md). A freshly
# adopted corpus has exactly one (the adopt commit that turns the invocation log on), so
# a worker finalize has landed only when there are two or more.
INSTALL="$(git log --diff-filter=A --format=%H -- CLAUDE.md | tail -1)"
if [ -z "$INSTALL" ]; then
  [ "$FORCE" = 1 ] || refuse "no commit adds CLAUDE.md — this corpus was never set up with jigc"
  SINCE="?"
else
  SINCE="$(git rev-list --count "$INSTALL..HEAD")"
  if [ "$SINCE" -lt 2 ] && [ "$FORCE" = 0 ]; then
    echo "commits since the jigc install commit:" >&2
    git --no-pager log --oneline "$INSTALL..HEAD" >&2
    refuse "only $SINCE commit(s) since install — the worker's first finalize has not landed.
         Land this AFTER it, or pass --force if you have checked by hand."
  fi
fi

# --- write, stage, commit -------------------------------------------------------------
mkdir -p docs/decisions

# The body's `Date:` and the commit's dates come from ONE value, so they can never
# disagree. RC-pre-1.0's worker caught the plant precisely on that seam: "committed 07:41
# today, body-dated 2026-08-04".
sed -e "s/^Date: .*$/Date: $DATE/" "$BODY" > "$TARGET"
grep -q "^Date: $DATE$" "$TARGET" || refuse "the Date rewrite did not apply — check the body's header"

# Explicit pathspec, both halves (protocol.md §8 rule 3). The worker's task is live and
# its index may hold staged work at this instant; `git add <path>` touches only this path
# and `git commit -- <path>` commits only this path, leaving everything else staged.
git add -- "$TARGET"
if ! GIT_AUTHOR_DATE="${DATE}T09:12:41+02:00" GIT_COMMITTER_DATE="${DATE}T09:12:41+02:00" \
     git commit -q -m "docs: record the in-memory store decision" -- "$TARGET"; then
  # Leave the repo as it was found rather than half-planted: an aborted plant that
  # leaves a staged, uncommitted foreign doc in a LIVE worker's index is worse than no
  # plant at all — it would ride the worker's next finalize.
  #
  # The likeliest cause, and the one that bit the rehearsal: a rejecting pre-commit
  # hook. B1's docs-gate plant refuses exactly this commit. B1 and B3 are separate
  # corpora, so it cannot happen in this trial's shape — but it is silent when it does.
  git restore --staged -- "$TARGET" 2>/dev/null || git rm -q --cached -- "$TARGET" 2>/dev/null || true
  rm -f "$TARGET"
  rmdir docs/decisions docs 2>/dev/null || true
  refuse "\`git commit\` was rejected — NOTHING was planted and the file has been removed.
         A rejecting pre-commit hook is the likely cause (B1's docs-gate refuses commits
         touching docs/). Check \`git config --get core.hooksPath\` and the hooks dir."
fi

echo "planted the foreign ADR in $REPO"
echo "  body     : $(basename "$BODY")  ($WHY)"
echo "  path     : $TARGET"
echo "  date     : $DATE  (body Date: and both commit dates)"
echo "  commit   : $(git log -1 --format='%h %ad %s' --date=iso -- "$TARGET")"
echo "  carried  : $(git show --stat --format= --name-only HEAD | tr '\n' ' ')"
echo "  still staged elsewhere: $(git diff --cached --name-only | tr '\n' ' ')"
