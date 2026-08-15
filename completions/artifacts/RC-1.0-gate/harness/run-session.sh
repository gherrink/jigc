#!/usr/bin/env bash
# run-session.sh <corpus-dir> <out-dir> [tag]
#
# Drive ONE interactive blind session in isolation.
#
#   1. create a container from the pinned image, with the corpus copied in
#   2. hand you an interactive Claude Code session inside it
#   3. on exit, copy the whole /work back out for analysis, then destroy the container
#
# The corpus is copied, never mounted: colima serves host mounts read-only, which
# silently produces sessions that appear to change nothing — a result this trial would
# otherwise read as a finding about jigc rather than about the mount.
#
# The ORIGINAL corpus directory is never touched, so a session can be re-run from a
# clean start and the operator cannot accidentally analyse a corpus a worker mutated.
set -euo pipefail

CORPUS="${1:?usage: run-session.sh <corpus-dir> <out-dir> [tag]}"
OUT="${2:?usage: run-session.sh <corpus-dir> <out-dir> [tag]}"
TAG="${3:-jigc-gate:rc11}"

[ -d "$CORPUS/.git" ] || { echo "refusing: $CORPUS is not a git repository" >&2; exit 2; }
[ -e "$OUT" ] && { echo "refusing: $OUT already exists — pick a fresh out-dir" >&2; exit 2; }

TOKEN="${CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING:-${CLAUDE_CODE_OAUTH_TOKEN:-}}"
[ -n "$TOKEN" ] || { echo "refusing: no token in CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING" >&2; exit 2; }

ENVFILE="$(mktemp)"; chmod 600 "$ENVFILE"
printf 'CLAUDE_CODE_OAUTH_TOKEN=%s\n' "$TOKEN" > "$ENVFILE"

CID=""
cleanup() {
  # Copy out BEFORE removing, and only then remove. A crash mid-session still yields
  # the evidence; the credentials outlive neither the run nor a crash during it.
  if [ -n "$CID" ]; then
    if [ ! -e "$OUT" ]; then
      mkdir -p "$OUT"
      docker cp "$CID:/work/." "$OUT/" >/dev/null 2>&1 || echo "WARNING: copy-out failed" >&2
    fi
    docker rm -f "$CID" >/dev/null 2>&1 || true
  fi
  rm -f "$ENVFILE"
}
trap cleanup EXIT

echo "image  : $TAG  ($(docker run --rm --entrypoint /usr/local/bin/jigc "$TAG" --version))"
echo "corpus : $CORPUS"
echo "out    : $OUT"
echo

# -it so the session is a real terminal. bypassPermissions because a permission prompt
# on every Edit is an operator touch that has to be logged and is not what this trial
# measures — the container is throwaway, so there is nothing here to protect.
CID="$(docker create -it --env-file "$ENVFILE" "$TAG" \
        claude --permission-mode bypassPermissions)"
docker cp "$CORPUS/." "$CID:/work/" >/dev/null

echo "starting the session — exit it normally when the work is done"
echo "-------------------------------------------------------------"
docker start -ai "$CID"
echo "-------------------------------------------------------------"
echo "session ended; copying /work out to $OUT"

mkdir -p "$OUT"
docker cp "$CID:/work/." "$OUT/" >/dev/null

echo
echo "invocation log:"
if [ -f "$OUT/.jigc/logs/invocations.jsonl" ]; then
  wc -l < "$OUT/.jigc/logs/invocations.jsonl" | sed 's/^/  records: /'
  echo "  read-back verb used at any point:"
  grep -c '"doc","show"' "$OUT/.jigc/logs/invocations.jsonl" 2>/dev/null | sed 's/^/    doc show calls: /' || echo "    doc show calls: 0"
else
  echo "  NONE — the log was never enabled, so §3's primary channel is missing"
fi
