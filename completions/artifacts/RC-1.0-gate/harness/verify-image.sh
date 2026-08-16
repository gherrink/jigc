#!/usr/bin/env bash
# verify-image.sh [tag] [expected-version]
#
# Prove the rig before it carries a trial. Seven checks, each one a thing that would
# otherwise fail silently and be read as a result about jigc rather than about the
# apparatus.
#
# Check 4 is the one the whole isolation exists for, and it is stated as a DIFFERENCE:
# the same probe answers YES on this host and must answer NO in here. A probe that is
# null on both sides proves nothing — that was this rig's first attempt, and it would
# have certified a machine with no isolation at all.
set -uo pipefail

TAG="${1:-jigc-gate:rc11}"
WANT_VERSION="${2:-1.0.0-rc.11}"
PASS=0; FAIL=0

ok()   { echo "  PASS  $1"; PASS=$((PASS+1)); }
bad()  { echo "  FAIL  $1"; FAIL=$((FAIL+1)); }
skip() { echo "  SKIP  $1"; }

TOKEN="${CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING:-${CLAUDE_CODE_OAUTH_TOKEN:-}}"
[ -n "$TOKEN" ] || { echo "refusing: no token in CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING" >&2; exit 2; }
ENVFILE="$(mktemp)"
chmod 600 "$ENVFILE"
# Pre-declared so the trap cannot abort on an unbound variable when an interrupt lands
# during checks 1–4, before check 5 assigns them. A rig whose whole argument is that
# apparatus failures must not read as results should not have a failing cleanup path.
CORPUS=""; OUT=""
trap 'rm -f "$ENVFILE"; [ -n "$CORPUS" ] && rm -rf "$CORPUS"; [ -n "$OUT" ] && rm -rf "$OUT"; true' EXIT
# --env-file rather than -e: -e puts the token in the PROCESS LIST. It does NOT keep it
# out of `docker inspect` — --env-file is parsed client-side and the value lands in the
# container config verbatim (verified with a canary). Declared bound, not a guarantee:
# anything that can reach the docker socket can read the token for the container's life.
printf 'CLAUDE_CODE_OAUTH_TOKEN=%s\n' "$TOKEN" > "$ENVFILE"

echo "== 1. the binary under test reports the version its tree carries"
GOT="$(docker run --rm --entrypoint /usr/local/bin/jigc "$TAG" --version 2>&1)"
[ "$GOT" = "jigc $WANT_VERSION" ] && ok "$GOT" || bad "wanted 'jigc $WANT_VERSION', got '$GOT'"

echo "== 2. the doc-code probe loads (a dead probe reads as a validation family finding nothing)"
docker run --rm --entrypoint sh "$TAG" -c '/usr/local/bin/doc-code --help >/dev/null 2>&1; [ $? -ne 127 ]' \
  && ok "doc-code executes" || bad "doc-code did not execute"

echo "== 3. no host INSTRUCTION files are present (the seeded onboarding state is not one)"
# Deliberately not "the home is empty": the image seeds ~/.claude.json so interactive
# mode does not open on the auth screen. That file carries no instructions. What must
# be absent is anything that could speak to the model — CLAUDE.md, memory, skills,
# agents, or an imported dotfile.
LEAK="$(docker run --rm --entrypoint sh "$TAG" -c '
  find /home/node /root -maxdepth 3 \( -name "CLAUDE.md" -o -name "*.memory" -o -name "PRINCIPLES.md" -o -name "LACON.md" \) 2>/dev/null
  ls -d /home/node/.claude/skills /home/node/.claude/agents /home/node/.claude/CLAUDE.md 2>/dev/null
' 2>&1)"
[ -z "$LEAK" ] && ok "no CLAUDE.md, no memory, no skills, no agents" || bad "found: $LEAK"

echo "== 4. the discriminating probe flips (must be YES on the host, NO in here)"
PROBE='Answer with one word only, YES or NO: do your loaded instructions mention a bash output filter, or a rule about asking the user only one question at a time?'

# BOTH sides are run, every time. The container half alone cannot distinguish "isolation
# works" from "the probe went null" — and a null on both sides is exactly what this rig's
# first probe did, which would have certified a machine with no isolation at all. The
# probe is keyed to the contents of the operator's own global instructions, so the day
# either is reworded this check must fail loudly rather than pass vacuously.
HOST_ANS="$(cd /tmp && env -u CLAUDECODE claude -p "$PROBE" --model claude-sonnet-5 2>&1 \
             | tr -d '[:space:]' | tr 'a-z' 'A-Z')"
ANS="$(docker run --rm --env-file "$ENVFILE" --entrypoint claude "$TAG" \
        -p "$PROBE" --model claude-sonnet-5 2>&1 | tr -d '[:space:]' | tr 'a-z' 'A-Z')"

if [ "$HOST_ANS" != "YES" ]; then
  bad "host answered '$HOST_ANS', not YES — the probe no longer discriminates, so a NO
        below would prove nothing. Reword the probe against the operator's current
        global instructions before trusting any isolation claim."
else
  case "$ANS" in
    NO)  ok "host YES / container NO — host instructions are absent" ;;
    YES) bad "container answered YES — ISOLATION IS NOT HOLDING" ;;
    *)   bad "container returned neither YES nor NO ('$ANS') — apparatus failure, not a result" ;;
  esac
fi

echo "== 5. a corpus round-trips with its git history intact"
CORPUS="$(mktemp -d)"; OUT="$(mktemp -d)"
git init -q "$CORPUS"
git -C "$CORPUS" config user.email t@t.t; git -C "$CORPUS" config user.name T
echo "# service" > "$CORPUS/README.md"
git -C "$CORPUS" add -A; git -C "$CORPUS" commit -qm "init"
BEFORE="$(git -C "$CORPUS" rev-parse HEAD)"

CID="$(docker create --env-file "$ENVFILE" "$TAG" \
        sh -c 'cd /work && jigc setup >/dev/null 2>&1 && jigc config set invocation-log true >/dev/null 2>&1 && git log --oneline | tail -1')"
docker cp "$CORPUS/." "$CID:/work/" >/dev/null
docker start -a "$CID" >/dev/null 2>&1
rm -rf "$OUT"; mkdir -p "$OUT"
docker cp "$CID:/work/." "$OUT/" >/dev/null
docker rm -f "$CID" >/dev/null

AFTER="$(git -C "$OUT" rev-parse HEAD 2>/dev/null || echo none)"
if [ "$AFTER" = "none" ]; then
  bad "no git history came back out"
elif ! git -C "$OUT" merge-base --is-ancestor "$BEFORE" "$AFTER" 2>/dev/null; then
  # "history preserved" is the claim; `AFTER != none` is not that claim. A container
  # that wiped and re-inited the repo satisfies the weaker test and destroys the corpus.
  bad "the returned history does not descend from $BEFORE — the corpus was not preserved"
elif [ -f "$OUT/.jigc/AGENT.md" ]; then
  ok "history descends from $BEFORE -> $AFTER, and jigc setup landed inside the container"
else
  bad "history came back but jigc setup left no .jigc/AGENT.md"
fi

echo "== 6. the workspace is trusted, so jigc's allowlist is honoured — and a transcript survives"
# Two things that would each silently change or erase the headline measurement.
#
# Trust: `jigc setup` writes .claude/settings.json allowlisting Bash(jigc:*) and
# Bash(git add:*). Claude Code IGNORES that file in an untrusted workspace, printing
# "Ignoring N permissions.allow entries ... this workspace has not been trusted". If the
# seeded projects["/work"].hasTrustDialogAccepted ever stops applying, every jigc call
# prompts, and the ergonomic asymmetry §3 measures is gone in the other direction.
# NOTE: this must run through the ENTRYPOINT (gosu node, HOME=/home/node). Probing with
# `--entrypoint claude` runs as root against an unseeded /root/.claude.json and
# reproduces the warning spuriously — which is how this check came to exist.
#
# Transcript: protocol.md §3.3's FILESYSTEM outcome has the session transcript as its
# ONLY evidence, and it lives in the session HOME rather than /work.
if [ -d "$OUT/.claude" ]; then
  CID2="$(docker create --env-file "$ENVFILE" "$TAG" \
           claude -p "reply with the single word ok" --model claude-sonnet-5)"
  docker cp "$OUT/." "$CID2:/work/" >/dev/null
  TRUST_OUT="$(docker start -a "$CID2" 2>&1)"
  # docker cp works on a stopped container; docker exec does not, and the -p run has
  # already exited by here.
  TDIR="$(mktemp -d)"
  docker cp "$CID2:/home/node/.claude/projects" "$TDIR/" >/dev/null 2>&1 || true
  NJSONL="$(find "$TDIR" -name '*.jsonl' 2>/dev/null | wc -l | tr -d ' ')"
  docker rm -f "$CID2" >/dev/null 2>&1
  rm -rf "$TDIR"

  case "$TRUST_OUT" in
    *"has not been trusted"*|*"Ignoring"*permissions*)
      bad "the workspace is NOT trusted — jigc's allowlist is being ignored: $TRUST_OUT" ;;
    *) ok "workspace trusted; jigc's allowlist is honoured" ;;
  esac
  [ "$NJSONL" -gt 0 ] && ok "a session transcript is recoverable ($NJSONL jsonl)" \
    || bad "no transcript came out of the session HOME — §3.3's FILESYSTEM channel is unmeasurable"
else
  skip "no .claude/ came back from check 5 — trust not exercised"
fi

echo
echo "== $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ] || exit 1
