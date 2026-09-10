#!/usr/bin/env bash
# 23-m50-audit-findings.sh — M50's four completion-audit findings, driven.
#
# WHY THIS ARM EXISTS: protocol.md §5 chartered arms J (F1), K (F3) and M (Increment
# 13's SKILL.md re-clobber), plus the F4 cell that "must run outside the agent". The
# first pass of the walk ran arms 01-21, which were derived for M49's surface and
# predate all four. Chartering an arm and not running it is the failure the walk's own
# record format exists to make visible, so it is closed here rather than declared.
#
# WHICH SET THIS ARM ITERATES: the four findings of [M50/VERDICT.md], hand-enumerated
# and said to be one — there is no registry of "what an audit found".
#
# ON F4 AND THE DENY FLOOR: `Bash(jigc milestone discard:*)` is on the Claude Code
# adapter's deny list, so an AGENT cannot reach that door — B4-h met exactly this and
# halted. A scripted `--exec` arm is not an agent and the profile does not bind it,
# which is what protocol.md §0.3 means by "must run it outside the agent".
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }

say "0 · adopt"
jigc setup >/dev/null 2>&1 || { echo "setup failed"; exit 1; }
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

say "F1 (HIGH) · an address whose <slug> head reaches OUTSIDE the repository is refused"
# CORRECTED after driving: `add-from-spec` takes a `<type>:<slug>` ADDRESS, not a path.
# The arm's first draft passed an outside *path* and its bars went green on
# `store.unparseable — missing ':' between type and slug`, which is a different door
# answering a different mistake. A bar that passes for the wrong reason is a false green
# in the instrument, and this one was caught only by driving the shape afterwards —
# the discipline the m50 pair probes followed and this arm initially did not.
mkdir -p /tmp/outside/specs
printf '# Outside spec\n\n## Criteria\n\n### Bounded {#bounded}\nIt caps.\n' > /tmp/outside/specs/foreign.md
jigc milestone create "Bound the store" >/dev/null 2>&1
MS=bound-the-store
bar "the milestone exists to add to"  "jigc milestone list-tasks '$MS' >/dev/null 2>&1"
for DEPTH in '../../outside/specs/foreign' '../../../outside/specs/foreign' '../../../../outside/specs/foreign'; do
  step jigc milestone add-from-spec "$MS" "spec:$DEPTH"
  OUT="$(jigc milestone add-from-spec "$MS" "spec:$DEPTH" 2>&1)"; RC=$?
  bar "F1 [$DEPTH]: refused (exit 0 before the audit)" "test $RC -ne 0"
  bar "F1 [$DEPTH]: store.malformed-slug"              "printf '%s' \"\$OUT\" | grep -q 'store.malformed-slug'"
done
bar "F1: nothing was seeded from any of them" \
    "! jigc milestone list-tasks '$MS' 2>/dev/null | grep -qi 'bounded'"

say "F4 (LOW) · milestone discard must refuse over a sub-task's staged prose — run OUTSIDE the agent"
jigc milestone add-task "$MS" "cap distinct series" >/dev/null 2>&1
# CORRECTED after driving: `list-tasks` emits ONE line —
#   `milestone:<id> tasks (N): a, b, c`
# — not indented rows. The first draft's `awk '/^  [a-z0-9]/'` matched nothing, SUB came
# back empty, and the F4 cell SKIPped. It reported the skip honestly, which is the only
# reason this was visible at all.
SUB="$(jigc milestone list-tasks "$MS" 2>/dev/null \
        | sed -n 's/^milestone:[a-z0-9-]* tasks ([0-9]*): //p' | tr ',' ' ' | awk '{print $1}')"
echo "sub-task: ${SUB:-<none>}"
if [ -n "$SUB" ]; then
  jigc workflow single-task --task "$SUB" >/dev/null 2>&1
  A="$(jigc doc create adr --title 'Cap distinct series' --task "$SUB" 2>&1 | grep -oE '^adr:[a-z0-9-]+$' | head -1)"
  [ -n "$A" ] && printf 'Authored prose no commit has a copy of.\n' \
      | jigc doc set-slot "${A}#context" --from-file - --task "$SUB" >/dev/null 2>&1
  echo "staged in the sub-task: $(ls ".jigc/tasks/$SUB/docs" 2>/dev/null | tr '\n' ' ')"
  step jigc milestone discard "$MS"
  DOUT="$(jigc milestone discard "$MS" 2>&1)"; DRC=$?
  bar "F4: the abandon REFUSES over staged prose (exit 0 before the audit)" "test $DRC -ne 0"
  bar "F4: it names the code"                "printf '%s' \"\$DOUT\" | grep -q 'milestone.staged-prose'"
  bar "F4: --force is named as the consent"  "printf '%s' \"\$DOUT\" | grep -q -- '--force'"
  bar "F4: the prose is still there"         "test -n \"\$(ls .jigc/tasks/$SUB/docs 2>/dev/null)\""
  step jigc milestone discard "$MS" --force
  jigc milestone discard "$MS" --force >/dev/null 2>&1
  bar "F4: the consent route runs verbatim"  "! jigc milestone list 2>/dev/null | grep -q '$MS'"
else
  echo "  SKIP  F4 — no sub-task minted; the cell is unreached, not passed"
fi

say "Inc 13 · re-setup over a LOCALLY EDITED SKILL.md refuses to clobber"
SK=".claude/skills/jigc/SKILL.md"
bar "the adapter guide shipped" "test -f '$SK'"
if [ -f "$SK" ]; then
  BEFORE="$(md5sum "$SK" 2>/dev/null | awk '{print $1}')"
  step jigc setup
  jigc setup >/dev/null 2>&1
  AFTER="$(md5sum "$SK" 2>/dev/null | awk '{print $1}')"
  bar "an unedited guide re-installs idempotently" "[ \"$BEFORE\" = \"$AFTER\" ]"
  printf '\n<!-- a human edited this -->\n' >> "$SK"
  EDITED="$(md5sum "$SK" 2>/dev/null | awk '{print $1}')"
  step jigc setup
  SOUT="$(jigc setup 2>&1)"
  NOW="$(md5sum "$SK" 2>/dev/null | awk '{print $1}')"
  bar "Inc 13: a user-edited guide is NOT clobbered" "[ \"$EDITED\" = \"$NOW\" ]"
  bar "Inc 13: and setup says so"                    "printf '%s' \"\$SOUT\" | grep -qiE 'user-modified|edited|refus|kept'"
fi

say "F3 (LOW) · a read-path finding prints a repo-relative path, not the host's"
step jigc doc show vision
VOUT="$(jigc doc show vision 2>&1)"
bar "F3: the miss names no absolute host path" "! printf '%s' \"\$VOUT\" | grep -qE '/(private/)?(var|tmp|home|Users)/'"
bar "F3: it carries a code"                    "printf '%s' \"\$VOUT\" | grep -qE 'blocking · store\.'"

echo
if [ "$FAIL" -eq 0 ]; then echo "ARM 23 PASS"; else echo "ARM 23 RED"; fi
exit 0
