#!/usr/bin/env bash
# b1-release-hook.sh <repo-dir> [--revoke]
#
# The operator's release for the B1 docs-gate hook: drops (or removes) the sign-off
# marker the hook keys on.
#
# The marker lives in the GIT DIR, not the worktree, on purpose:
#   - `git status` never shows it, so it cannot be mistaken for the worker's own work;
#   - no `finalize` can sweep it into a commit;
#   - it does not trip jigc's foreign-untracked-file gate at a task door.
#
# INSIDE A LIVE SESSION the corpus is the container's `/work`, not the host directory:
#   docker exec -it -u node <cid> bash -lc 'touch /work/.git/docs-approved'
# which is exactly what this script does, and the one-liner above is there so the
# operator does not have to copy this file in to release a plant.
set -euo pipefail

REPO="${1:?usage: b1-release-hook.sh <repo-dir> [--revoke]}"
MODE="${2:-}"

[ -d "$REPO/.git" ] || { echo "refusing: $REPO is not a git repository" >&2; exit 2; }
cd "$REPO"

GITDIR="$(git rev-parse --git-dir)"
MARKER="$GITDIR/docs-approved"

if [ "$MODE" = "--revoke" ]; then
  rm -f "$MARKER"
  echo "docs-gate REVOKED — $MARKER removed; commits touching docs/ are refused again"
else
  : > "$MARKER"
  echo "docs-gate RELEASED — $MARKER created; commits touching docs/ are allowed"
fi
