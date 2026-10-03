#!/usr/bin/env bash
# M55 genuine-spawn — the automated N-process sim of flow 58 (D), in its own fresh
# rig: every Spawn span run verbatim through the jigc shim, each sub-task filing
# from its own worktree with the test's exact values, then the join + finalize.
#
#   sim.sh [by-id|reverse] [exact|heredoc] [join|nojoin]
#
# Prints `tree=<hash>` and the HEAD commit's file list on stdout (construction
# chatter on stderr). Defaults: by-id exact join.
set -u
. <scratchpad>/spawn/common.sh

order=${1:-by-id}; form=${2:-exact}; join=${3:-join}
prep_rig || exit 1
case $order in
    by-id)   subs="$SUB_INC $SUB_FB1 $SUB_FB2" ;;   # sorted task ids
    reverse) subs="$SUB_FB2 $SUB_FB1 $SUB_INC" ;;
    *) echo "order: by-id|reverse" >&2; exit 2 ;;
esac
for s in $subs; do sim_sub "$s" "$form" || exit 1; done
if [ "$join" = join ]; then
    join_and_finalize || exit 1
else
    (cd "$REPO" && HOME="$RIG_HOME" "$JIGC_BIN" milestone finalize "$MILESTONE" >/dev/null 2>&1) || exit 1
fi
echo "rig=$RIG order=$order form=$form join=$join" >&2
echo "tree=$(git -C "$REPO" rev-parse 'HEAD^{tree}')"
echo "base-tree=$(git -C "$REPO" rev-parse 'HEAD~1^{tree}')"
git -C "$REPO" show --name-only --pretty=format: HEAD | sed '/^$/d'
