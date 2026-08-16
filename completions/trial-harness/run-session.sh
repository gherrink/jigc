#!/usr/bin/env bash
# run-session.sh [--shell] [--strict-permissions] <corpus-dir> <out-dir> [tag]
#
# Drive ONE session in isolation.
#
#   1. create a container from the pinned image, with the corpus copied in
#   2. hand you an interactive Claude Code session inside it (or a shell, with --shell)
#   3. on exit, copy /work AND the session transcript out, verify the copy, then destroy
#
# The corpus is copied, never mounted: colima serves host mounts read-only, which
# silently produces sessions that appear to change nothing — a result this trial would
# otherwise read as a finding about jigc rather than about the mount.
#
# The ORIGINAL corpus directory is never touched, so a session can be re-run from a
# clean start and the operator cannot accidentally analyse a corpus a worker mutated.
set -euo pipefail

MODE=claude
PERMISSION_MODE=bypassPermissions
EXEC_FILE=""

while true; do
  case "${1:-}" in
    --shell)              MODE=shell; shift ;;
    # --exec runs a script through the SAME copy-in / copy-out / provenance path a blind
    # session uses. That is the point: arm 0's job is to prove the containerised chain,
    # and a control driven by some other mechanism would not validate the mechanism the
    # blind sessions actually run on.
    --exec)               MODE=exec; EXEC_FILE="${2:?--exec needs a script}"; shift 2 ;;
    --strict-permissions) PERMISSION_MODE=default; shift ;;
    -*) echo "unknown option: $1" >&2; exit 2 ;;
    *) break ;;
  esac
done

CORPUS="${1:?usage: run-session.sh [--shell] [--strict-permissions] <corpus-dir> <out-dir> [tag]}"
OUT="${2:?usage: run-session.sh [--shell] [--strict-permissions] <corpus-dir> <out-dir> [tag]}"
TAG="${3:-jigc-gate:rc11}"

# Pinned, and recorded. The CLI version is pinned in the Dockerfile with the argument
# that "a trial whose CLI version drifts under it cannot be compared to its own other
# sessions" — that applies with more force to the model, which is the single largest
# behavioural variable in a behavioural measurement. Three blind sessions may run days
# apart; nothing else in $OUT would say which model any of them used.
MODEL="${JIGC_GATE_MODEL:-claude-sonnet-5}"

[ -d "$CORPUS/.git" ] || { echo "refusing: $CORPUS is not a git repository" >&2; exit 2; }
[ -e "$OUT" ] && { echo "refusing: $OUT already exists — pick a fresh out-dir" >&2; exit 2; }

TOKEN="${CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING:-${CLAUDE_CODE_OAUTH_TOKEN:-}}"
[ -n "$TOKEN" ] || { echo "refusing: no token in CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING" >&2; exit 2; }

# The image must be the one you think it is. verify-pair.sh exists because a
# wrong-binary run is invisible in the output; this is the point where it matters.
STAMP="$(docker run --rm --entrypoint /usr/local/bin/jigc "$TAG" --version 2>&1)"
SHA="$(docker inspect "$TAG" --format '{{range .Config.Env}}{{println .}}{{end}}' \
        | sed -n 's/^JIGC_SHA=//p' | head -1)"
[ -n "$SHA" ] || { echo "refusing: $TAG carries no JIGC_SHA — not built by build-image.sh" >&2; exit 2; }

# --env-file rather than -e: -e puts the token in the PROCESS LIST. It does NOT keep it
# out of `docker inspect` — --env-file is parsed client-side and the value lands in the
# container config verbatim (verified with a canary). Declared bound, not a guarantee,
# and it matters most here: this container lives for the whole session.
ENVFILE="$(mktemp)"; chmod 600 "$ENVFILE"
printf 'CLAUDE_CODE_OAUTH_TOKEN=%s\n' "$TOKEN" > "$ENVFILE"

CID=""
COPIED=0

copy_out() {
  # /work is the corpus. The transcript lives in the session HOME, NOT under /work —
  # `/home/node/.claude/projects/-work/<uuid>.jsonl` — and protocol.md §3.3 names the
  # transcript as one of its two evidence channels, the ONLY evidence for a FILESYSTEM
  # outcome. Destroying the container without it left that channel unmeasurable.
  mkdir -p "$OUT"
  docker cp "$CID:/work/." "$OUT/" >/dev/null 2>&1 || return 1
  mkdir -p "$OUT/.session-transcript"
  for home in /home/node /root; do
    docker cp "$CID:$home/.claude/projects" "$OUT/.session-transcript/" >/dev/null 2>&1 || true
  done
  # The corpus must have come back with its history, or there is nothing to analyse.
  [ -d "$OUT/.git" ] || return 1
  COPIED=1
  return 0
}

cleanup() {
  if [ -n "$CID" ]; then
    if [ "$COPIED" = 0 ]; then
      # Retry unconditionally. The previous version skipped the retry whenever $OUT
      # existed — which its own mkdir had just guaranteed — so a failure during the
      # main copy destroyed the only copy of a multi-hour session.
      if copy_out; then
        echo "evidence recovered to $OUT" >&2
      else
        echo "WARNING: copy-out FAILED — keeping container $CID for manual recovery" >&2
        echo "         docker cp $CID:/work/. <somewhere>" >&2
        rm -f "$ENVFILE"
        return
      fi
    fi
    docker rm -f "$CID" >/dev/null 2>&1 || true
  fi
  rm -f "$ENVFILE"
}
trap cleanup EXIT

echo "image      : $TAG ($STAMP)"
echo "jigc sha   : $SHA"
echo "model      : $MODEL"
echo "permissions: $PERMISSION_MODE"
echo "corpus     : $CORPUS"
echo "out        : $OUT"
[ "$PERMISSION_MODE" = bypassPermissions ] && cat >&2 <<'WARN'

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.

WARN

if [ "$MODE" = shell ]; then
  CID="$(docker create -it --env-file "$ENVFILE" "$TAG" bash -l)"
elif [ "$MODE" = exec ]; then
  [ -f "$EXEC_FILE" ] || { echo "refusing: no such script: $EXEC_FILE" >&2; exit 2; }
  CID="$(docker create --env-file "$ENVFILE" "$TAG" bash -lc 'bash /tmp/arm.sh')"
else
  CID="$(docker create -it --env-file "$ENVFILE" "$TAG" \
          claude --model "$MODEL" --permission-mode "$PERMISSION_MODE")"
fi
docker cp "$CORPUS/." "$CID:/work/" >/dev/null
[ "$MODE" = exec ] && docker cp "$EXEC_FILE" "$CID:/tmp/arm.sh" >/dev/null

echo "container  : $CID"
# -u node is required, not cosmetic: `docker exec` bypasses the ENTRYPOINT's gosu, so it
# lands as root, and every git call in /work then dies on "detected dubious ownership".
echo "  (a mid-stream plant runs with: docker exec -it -u node $CID bash -l)"
echo
if [ "$MODE" = exec ]; then echo "running $EXEC_FILE in the container"; else
echo "starting — exit normally when the work is done"; fi
echo "-------------------------------------------------------------"
# -ai for the interactive modes (stdin attached, or you cannot type); -a for a scripted
# arm, which has no stdin. Getting this wrong disables input on a blind session silently.
if [ "$MODE" = exec ]; then
  docker start -a "$CID" || echo "(the arm exited non-zero — that is data, not necessarily failure)"
else
  docker start -ai "$CID"
fi
echo "-------------------------------------------------------------"

copy_out || { echo "copy-out failed; the trap will retry and keep the container" >&2; exit 1; }

# Provenance travels with the evidence, so the record never has to reconstruct it.
cat > "$OUT/PROVENANCE.txt" <<EOF
image        $TAG
jigc-version $STAMP
jigc-sha     $SHA
model        $MODEL
permissions  $PERMISSION_MODE
corpus-src   $CORPUS
EOF

echo "evidence in $OUT (corpus + .session-transcript/ + PROVENANCE.txt)"
LOG="$OUT/.jigc/logs/invocations.jsonl"
if [ -f "$LOG" ]; then
  echo "  invocation records : $(wc -l < "$LOG" | tr -d ' ')"
  # protocol.md §3.3 scores the STAGED read-back, `doc show … --task …` — not any
  # `doc show`, which includes the committed-store lookup that has existed since M39
  # and is not what the wave's claim is about. Both are printed; only one is the claim.
  echo "  doc show --task    : $(grep -c '"doc","show".*"--task"' "$LOG" 2>/dev/null || echo 0)"
  echo "  doc show (any)     : $(grep -c '"doc","show"' "$LOG" 2>/dev/null || echo 0)"
  echo "  task diff/doc list : $(grep -cE '"task","diff"|"doc","list"' "$LOG" 2>/dev/null || echo 0)   (VERB-ADJACENT, §3.3)"
else
  echo "  NO INVOCATION LOG — §3.3's primary channel is missing for this session"
fi
