#!/usr/bin/env bash
# run-session.sh [--shell|--exec F|--headless] [--strict-permissions] [--cid-file P]
#                <corpus-dir> <out-dir> [tag]
#
# Drive ONE session in isolation.
#
#   1. create a container from the pinned image, with the corpus copied in
#   2. hand you an interactive Claude Code session inside it (or a shell, with --shell,
#      or an unattended `claude -p` turn, with --headless)
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
PROMPT_FILE=""
HOME_DIR=""
EXTRA=()
CID_FILE=""
# A headless turn has no terminal, so its stdout IS the stream-json transcript and
# its stderr is the only place a CLI-level failure appears. Captured to temps and
# moved into $OUT after copy_out, because $OUT must not exist when the run starts.
#
# Declared empty here and allocated only after the EXIT trap is armed: eight
# refusal paths sit between this line and that trap, and allocating up here leaked
# two temp files on every one of them.
CAP_OUT=""; CAP_ERR=""

while true; do
  case "${1:-}" in
    --shell)              MODE=shell; shift ;;
    # --exec runs a script through the SAME copy-in / copy-out / provenance path a blind
    # session uses. That is the point: arm 0's job is to prove the containerised chain,
    # and a control driven by some other mechanism would not validate the mechanism the
    # blind sessions actually run on.
    --exec)               MODE=exec; EXEC_FILE="${2:?--exec needs a script}"; shift 2 ;;
    # --headless drives ONE unattended `claude -p` turn through the identical
    # copy-in / copy-out / provenance path, for the same reason --exec does. It is
    # how a plant or a prompt gets rehearsed without buying an operator session —
    # the act cue-cards.md named as its own largest untested assumption and never
    # paid for.
    --headless)           MODE=headless; shift ;;
    --prompt-file)        PROMPT_FILE="${2:?--prompt-file needs a file}"; shift 2 ;;
    # A staged `.claude` tree copied in as ~/.claude BEFORE the CLI starts. Without
    # it `--resume <id>` resumes nothing — and does not fail: it silently starts a
    # FRESH conversation, which is the one apparatus failure that manufactures a
    # plausible arm out of a dead fixture.
    --home)               HOME_DIR="${2:?--home needs a directory}"; shift 2 ;;
    # Appended to the `claude` command line, in order. Carries --session-id /
    # --resume / --fork-session without this script needing to know about them.
    --arg)                EXTRA+=("${2:?--arg needs a value}"); shift 2 ;;
    # Where to write the container id, as soon as it exists. A mid-stream plant has
    # to reach INTO the live container, and the id is printed to a terminal nobody
    # is watching on an unattended run. Written before `docker start`, so a poller
    # can be waiting before the session has done anything.
    --cid-file)           CID_FILE="${2:?--cid-file needs a path}"; shift 2 ;;
    --strict-permissions) PERMISSION_MODE=default; shift ;;
    -*) echo "unknown option: $1" >&2; exit 2 ;;
    *) break ;;
  esac
done

if [ "$MODE" = headless ] && [ -z "$PROMPT_FILE" ]; then
  echo "refusing: --headless needs --prompt-file (a turn with no prompt is not a turn)" >&2
  exit 2
fi
if [ -n "$PROMPT_FILE" ] && [ ! -f "$PROMPT_FILE" ]; then
  echo "refusing: no such prompt file: $PROMPT_FILE" >&2; exit 2
fi
if [ -n "$HOME_DIR" ] && [ ! -d "$HOME_DIR/.claude" ]; then
  echo "refusing: --home $HOME_DIR has no .claude/ — \`--resume\` would resume nothing" >&2
  exit 2
fi

CORPUS="${1:?usage: run-session.sh [--shell|--exec F|--headless] [--strict-permissions] [--cid-file P] <corpus-dir> <out-dir> [tag]}"
OUT="${2:?usage: run-session.sh [--shell|--exec F|--headless] [--strict-permissions] [--cid-file P] <corpus-dir> <out-dir> [tag]}"
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
  rm -f "${CAP_OUT:-}" "${CAP_ERR:-}" 2>/dev/null || true
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
# Safe to allocate from here: every exit below runs cleanup.
CAP_OUT="$(mktemp)"; CAP_ERR="$(mktemp)"

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
elif [ "$MODE" = headless ]; then
  # A BRANCH of this chain, not a second `if` after it. Written as a separate `if`
  # it fell through the `else` first, created an interactive container, then created
  # the real one and overwrote CID — so every headless run orphaned a container that
  # was never started, never removed, and carries the OAuth token in its config.
  # Eleven had accumulated before this was found.
  #
  # `-p` with the prompt as an argument, and stream-json so the transcript is
  # parseable. No -t: a headless turn has no stdin, and attaching one that never
  # closes hangs the run.
  PROMPT="$(cat "$PROMPT_FILE")"
  CID="$(docker create --env-file "$ENVFILE" "$TAG" \
          claude -p "$PROMPT" --model "$MODEL" --permission-mode "$PERMISSION_MODE" \
          --output-format stream-json --verbose ${EXTRA[@]+"${EXTRA[@]}"})"
else
  CID="$(docker create -it --env-file "$ENVFILE" "$TAG" \
          claude --model "$MODEL" --permission-mode "$PERMISSION_MODE")"
fi
docker cp "$CORPUS/." "$CID:/work/" >/dev/null
[ "$MODE" = exec ] && docker cp "$EXEC_FILE" "$CID:/tmp/arm.sh" >/dev/null
# ONCE, and of the `.claude` directory itself: `docker cp DIR CONTAINER:DEST` nests
# when DEST/DIR already exists, so a second copy lands at ~/.claude/.claude where the
# CLI sees neither the transcript nor anything else staged.
[ -n "$HOME_DIR" ] && docker cp "$HOME_DIR/.claude" "$CID:/home/node/" >/dev/null

if [ -n "$CID_FILE" ]; then mkdir -p "$(dirname "$CID_FILE")"; printf '%s\n' "$CID" > "$CID_FILE"; fi
echo "container  : $CID"
# -u node is required, not cosmetic: `docker exec` bypasses the ENTRYPOINT's gosu, so it
# lands as root, and every git call in /work then dies on "detected dubious ownership".
echo "  (a mid-stream plant runs with: docker exec -it -u node $CID bash -l)"
echo
if [ "$MODE" = exec ]; then echo "running $EXEC_FILE in the container";
elif [ "$MODE" = headless ]; then echo "driving one headless turn from $PROMPT_FILE";
else echo "starting — exit normally when the work is done"; fi
echo "-------------------------------------------------------------"
# -ai for the interactive modes (stdin attached, or you cannot type); -a for a scripted
# arm, which has no stdin. Getting this wrong disables input on a blind session silently.
RUN_RC=0
if [ "$MODE" = exec ] || [ "$MODE" = headless ]; then
  # Captured rather than streamed, so both channels survive into the evidence. A
  # scripted arm is then replayed to the terminal; a headless turn is not, because
  # its stdout is a stream-json transcript and dumping it buries the summary.
  # The instant the session begins, in UTC — the clock `.jigc/logs/invocations.jsonl`
  # stamps its records with. Anything in that log OLDER than this was written by
  # something other than this session: the adoption arm, or a plant. A rehearsal
  # measured a plant contributing 4 of a reported 6 VERB records, so this is not
  # hypothetical bookkeeping — it is the difference between the worker's number and
  # the rig's. See driver/observe.py's pre_session_records.
  SESSION_START="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  docker start -a "$CID" >"$CAP_OUT" 2>"$CAP_ERR" || RUN_RC=$?
  if [ "$MODE" = exec ]; then cat "$CAP_OUT"; cat "$CAP_ERR" >&2; fi
  [ "$RUN_RC" -ne 0 ] && echo "(the arm exited $RUN_RC — that is data, not necessarily failure)"
else
  SESSION_START="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  docker start -ai "$CID"
fi
echo "-------------------------------------------------------------"

copy_out || { echo "copy-out failed; the trap will retry and keep the container" >&2; exit 1; }

# The headless channels join the evidence. `stream.jsonl` is stdout verbatim — the
# transcript a parser reads; `stderr.txt` is where a dead CLI says so, and a run that
# produced no stream but exited 0 is an apparatus failure, not a worker that did nothing.
if [ "$MODE" = headless ]; then
  cp "$CAP_OUT" "$OUT/stream.jsonl"
  cp "$CAP_ERR" "$OUT/stderr.txt"
  echo "  stream events      : $(wc -l < "$OUT/stream.jsonl" | tr -d ' ')"
  if [ ! -s "$OUT/stream.jsonl" ]; then
    echo "  WARNING: empty stream — the turn produced no events. Read stderr.txt before" >&2
    echo "           reading anything else; this is an apparatus failure, not a result." >&2
  fi
fi

# Provenance travels with the evidence, so the record never has to reconstruct it.
cat > "$OUT/PROVENANCE.txt" <<EOF
image        $TAG
image-id     $(docker image inspect "$TAG" --format '{{.Id}}' 2>/dev/null || echo unknown)
jigc-version $STAMP
jigc-sha     $SHA
model        $MODEL
permissions  $PERMISSION_MODE
corpus-src   $CORPUS
session-start ${SESSION_START:-unknown}
exit-code    $RUN_RC
EOF

echo "evidence in $OUT (corpus + .session-transcript/ + PROVENANCE.txt)"
LOG="$OUT/.jigc/logs/invocations.jsonl"
# A QUICK LOOK, NOT THE MEASUREMENT. `completions/trial-driver/run.py observe "$OUT"`
# is the authoritative reader: it normalises leading global flags (`jigc --format json
# doc show …` is accepted and logged verbatim, and a raw grep matches none of it),
# separates attempts from `VERB-effective` (exit 0), and reads the arm's real exit code
# out of PROVENANCE.txt. Two implementations of one registered measurement is how the
# 1.0.0-gate undercount happened; this one is deliberately the non-authoritative half.
# The counts below were aligned to protocol §3.3 on 2026-08-28 (`task validate` joins;
# a `doc list` without `--task` is a committed-store index read and leaves).
count() {
  # `grep -c` prints 0 AND exits 1 on no match, so `|| echo 0` used to emit two zeros.
  local n
  n="$(grep -cE "$1" "$LOG" 2>/dev/null)" || n=0
  printf '%s' "${n:-0}"
}
if [ -f "$LOG" ]; then
  echo "  invocation records : $(wc -l < "$LOG" | tr -d ' ')"
  # protocol.md §3.3 scores the STAGED read-back, `doc show … --task …` — not any
  # `doc show`, which includes the committed-store lookup that has existed since M39
  # and is not what the wave's claim is about. Both are printed; only one is the claim.
  echo "  doc show --task    : $(count '"doc","show".*"--task"')"
  echo "  doc show (any)     : $(count '"doc","show"')"
  echo "  §3.3 adjacent      : $(count '"task","diff"|"task","validate"|"doc","list".*"--task"')   (task diff · task validate · doc list --task)"
  echo "  ^ indicative only — score with: completions/trial-driver/run.py observe \"$OUT\""
else
  echo "  NO INVOCATION LOG — §3.3's primary channel is missing for this session"
fi
