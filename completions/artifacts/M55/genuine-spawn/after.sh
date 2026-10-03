#!/usr/bin/env bash
# M55 genuine-spawn — run by the orchestrator AFTER the three real sub-agents return.
#
#   after.sh [env-file]      (default: genuine.env, written by prepare-genuine.sh)
#
# 1. Pre-join witness: each sub-task worktree's `git status --porcelain` (must be
#    empty — the sub-agents write only through jigc, never into their worktree)
#    and each sub-task's staged doc list through `jigc doc list --task`.
# 2. The join + the one commit: `jigc milestone join` then `jigc milestone finalize`.
# 3. The tree-hash of the join commit, against
#      (a) the golden recorded at preparation (golden.txt), and
#      (b) a golden RE-DERIVED now by sim.sh in a fresh rig — the landed docs carry
#          a CLI-set UTC `date:` that nothing pins, so (b) is the binding comparison
#          and (a) only says whether the UTC day rolled since preparation.
#    MATCH iff the genuine tree equals (b). On MISMATCH, a per-file diff.
# It never edits the jigc repository.
set -u
. <scratchpad>/spawn/common.sh
envfile=${1:-$SPAWN_DIR/genuine.env}
. "$envfile" || exit 1
[ -n "${REPO:-}" ] && [ -d "$REPO/.jigc" ] || { echo "after: no rig at REPO=${REPO:-}" >&2; exit 1; }
export HOME="$RIG_HOME"; unset JIGC_PACK_DIR

echo "== binary: $("$JIGC_BIN" --version)  sha256 $(shasum -a 256 "$JIGC_BIN" | cut -c1-16)…"
echo "== UTC date now: $(date -u +%F)"

subject=$(git -C "$REPO" log -1 --format=%s)
case $subject in
  "Finalize milestone $MILESTONE"*)
    echo "== already finalized (HEAD: $subject) — comparing only" ;;
  *)
    echo "== pre-join witness"
    for s in "$SUB_FB1" "$SUB_FB2" "$SUB_INC"; do
        wt="$REPO/.jigc/worktrees/$s"
        st=$(git -C "$wt" status --porcelain --untracked-files=all)
        if [ -z "$st" ]; then echo "  $s worktree: clean"; else echo "  $s worktree: DIRTY"; printf '%s\n' "$st" | sed 's/^/    /'; fi
        case $s in "$SUB_INC") ty=inconsistency ;; *) ty=jigc-feedback ;; esac
        echo "  $s staged:"
        (cd "$wt" && "$JIGC_BIN" doc list "$ty" --task "$s" 2>&1) | sed 's/^/    /'
    done
    echo "== join"
    out=$(cd "$REPO" && "$JIGC_BIN" milestone join "$MILESTONE" 2>&1); rc=$?
    printf '%s\n' "$out" | sed 's/^/  /'
    [ "$rc" -eq 0 ] || { echo "MISMATCH: join exited $rc"; exit 1; }
    echo "== finalize"
    out=$(cd "$REPO" && "$JIGC_BIN" milestone finalize "$MILESTONE" 2>&1); rc=$?
    printf '%s\n' "$out" | sed 's/^/  /'
    [ "$rc" -eq 0 ] || { echo "MISMATCH: finalize exited $rc"; exit 1; }
    ;;
esac

genuine=$(git -C "$REPO" rev-parse 'HEAD^{tree}')
echo "== join commit: $(git -C "$REPO" log -1 --format='%h %s')"
echo "== join commit files:"
git -C "$REPO" show --name-only --pretty=format: HEAD | sed '/^$/d; s/^/  /'
echo "== main checkout status (expect empty):"
git -C "$REPO" status --porcelain | sed 's/^/  /'

recorded=$(sed -n 's/^tree=//p' "$SPAWN_DIR/golden.txt" 2>/dev/null)
echo "== re-deriving the golden now (sim.sh by-id exact join, fresh rig)…"
sim_out=$("$SPAWN_DIR/sim.sh" by-id exact join 2>"$SPAWN_DIR/after-sim.err") || { echo "sim failed:"; cat "$SPAWN_DIR/after-sim.err"; exit 1; }
fresh=$(printf '%s\n' "$sim_out" | sed -n 's/^tree=//p')
sim_repo=$(sed -n 's/^rig=\([^ ]*\) .*/\1/p' "$SPAWN_DIR/after-sim.err")/repo

echo
echo "genuine-spawn tree      = $genuine"
echo "sim tree (re-derived)   = $fresh"
echo "sim tree (golden.txt)   = ${recorded:-<none>}"
[ "$fresh" = "$recorded" ] || echo "  note: re-derived != recorded — the UTC day (docs' date:) or the binary moved since preparation; the re-derived one binds"
if [ "$genuine" = "$fresh" ]; then
    echo "MATCH"
    exit 0
fi
echo "MISMATCH"
for f in $(git -C "$REPO" show --name-only --pretty=format: HEAD) $(git -C "$sim_repo" show --name-only --pretty=format: HEAD); do echo "$f"; done | sort -u |
while read -r f; do
    a=$(git -C "$REPO" rev-parse "HEAD:$f" 2>/dev/null); b=$(git -C "$sim_repo" rev-parse "HEAD:$f" 2>/dev/null)
    [ "$a" = "$b" ] && continue
    echo "--- differs: $f (genuine ${a:-absent} vs sim ${b:-absent})"
    diff <(git -C "$sim_repo" show "HEAD:$f" 2>/dev/null) <(git -C "$REPO" show "HEAD:$f" 2>/dev/null) | sed 's/^/  /'
done
exit 1
