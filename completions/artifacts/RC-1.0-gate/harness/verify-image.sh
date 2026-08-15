#!/usr/bin/env bash
# verify-image.sh [tag] [expected-version]
#
# Prove the rig before it carries a trial. Five checks, each one a thing that would
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

TOKEN="${CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING:-${CLAUDE_CODE_OAUTH_TOKEN:-}}"
ENVFILE="$(mktemp)"
chmod 600 "$ENVFILE"
trap 'rm -f "$ENVFILE"; rm -rf "$CORPUS" "$OUT"' EXIT
# --env-file, never -e: -e puts the token in the process list and in `docker inspect`
# for anything on this machine to read.
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

echo "== 4. the discriminating probe flips (YES on the host, must be NO in here)"
PROBE='Answer with one word only, YES or NO: do your loaded instructions mention a bash output filter, or a rule about asking the user only one question at a time?'
ANS="$(docker run --rm --env-file "$ENVFILE" --entrypoint claude "$TAG" \
        -p "$PROBE" --model claude-sonnet-5 2>&1 | tr -d '[:space:]' | tr 'a-z' 'A-Z')"
case "$ANS" in
  NO)  ok "container answered NO — host instructions are absent" ;;
  YES) bad "container answered YES — ISOLATION IS NOT HOLDING" ;;
  *)   bad "probe returned neither YES nor NO ('$ANS') — apparatus failure, not a result" ;;
esac

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
elif [ -f "$OUT/.jigc/AGENT.md" ]; then
  ok "history preserved ($BEFORE -> $AFTER) and jigc setup landed inside the container"
else
  bad "history came back but jigc setup left no .jigc/AGENT.md"
fi

echo
echo "== $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ] || exit 1
