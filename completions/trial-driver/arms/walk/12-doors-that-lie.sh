#!/usr/bin/env bash
# 12-doors-that-lie.sh — protocol.md §5 arm 12 (M49 Increment 2).
#
# WHICH SET THIS ARM ITERATES: a **code-side registry** — the `VerbKind::Read`
# rows of `crates/cli/src/cli.rs`'s `VERB_KINDS`, twelve leaves on 1.0.0-rc.13:
#
#   upgrade · describe · validate · doc show · doc schema · doc list ·
#   task list · task diff · task validate · config get · config list ·
#   milestone list-tasks
#
# plus the two MINT doors that take `--workflow`: `milestone add-task` and
# `milestone add-from-spec`.
#
# THE FLIP, stated so the arm reads as a change and not a fact of nature:
#   rc.12  `milestone add-task … --workflow <unknown>` minted the sub-task, wrote
#          the record and committed at exit 0 (the id was only checked on
#          re-entry); `milestone list-tasks` on a fresh clone REBUILT every absent
#          sub-task working area — six files — inventing the pack-default workflow
#          for a sub-task minted under another one; and `task discard <sub>` left
#          the committed record saying `active`.
#   rc.13  the `--workflow` id is checked against the loaded packs BEFORE anything
#          mints, at both doors; `list-tasks` reads the record and writes nothing;
#          `task discard` settles the item to `discarded` in a record-only commit.
#
# THE FENCE IS NOT THIS ARM. `crates/cli/tests/read_verb_acts_nothing.rs` iterates
# the registry in code over a fresh-clone state built to reveal a write, and
# `flow50_acceptance.rs` arm 2 drives the mint doors. This arm drives the same
# doors LIVE in the walk corpus and states the count it was written against, so it
# is visibly stale if that number moves. A walk arm is the live companion to a
# fence, never a substitute.
#
# PASS CONDITION, stated before the run:
#   (a) the unknown-workflow refusal blocks non-zero with its finding code, names
#       the loaded set, leaves no working area / no commit, and its route, filled
#       in, runs;
#   (b) a discarded sub-task reads `discarded` in the COMMITTED record, on a
#       record-only commit, and leaves `list-tasks`' live set;
#   (c) `list-tasks` on a fresh clone writes nothing under `.jigc/tasks/` or
#       `.jigc/milestones/`, while an operating door (`add-task`) re-seeds;
#   (d) no read verb creates anything under `.jigc/tasks/` or `.jigc/milestones/`,
#       moves HEAD, or moves the git index. Any other change is REPORTED in the
#       per-verb table as an observation, never asserted away — the one write the
#       registry's own doc-comment admits is the derived cache under `.jigc/index/`,
#       whose two safety properties (HEAD-stamped · a fixed point) are checked.
set -uo pipefail
cd /work

READ_VERBS_EXPECTED=12

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }

say "0 · adopt"
jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

# ---------------------------------------------------------------------------
say "A · THE MINT DOORS — an unknown --workflow is refused BEFORE anything mints"
step jigc milestone create "bound the store"
HEAD_A="$(git rev-parse HEAD)"
TASKS_A="$(ls .jigc/tasks 2>/dev/null | sort | tr '\n' ' ')"

step jigc milestone add-task bound-the-store "cap distinct series" --workflow no-such-workflow
A1="$(jigc milestone add-task bound-the-store "cap distinct series" --workflow no-such-workflow 2>&1)"; A1RC=$?
TASKS_B="$(ls .jigc/tasks 2>/dev/null | sort | tr '\n' ' ')"

bar "add-task refuses (exit non-zero)"                      "test $A1RC -ne 0"
bar "…carrying workflow-refs.unknown-workflow"              "printf '%s' \"\$A1\" | grep -q 'workflow-refs.unknown-workflow'"
bar "…enumerating the loaded set (names sub-task AND dev-task)" \
    "printf '%s' \"\$A1\" | grep -q 'sub-task' && printf '%s' \"\$A1\" | grep -q 'dev-task'"
bar "…and says what it left behind"                         "printf '%s' \"\$A1\" | grep -q 'nothing was minted, recorded or committed'"
bar "no working area was minted"                            "test ! -e .jigc/tasks/cap-distinct-series"
bar ".jigc/tasks/ is unchanged"                             "test \"\$TASKS_A\" = \"\$TASKS_B\""
bar "no commit landed"                                      "test \"\$(git rev-parse HEAD)\" = \"\$HEAD_A\""
bar "the record does not name the refused sub-task" \
    "! jigc doc show milestone-record:bound-the-store --format json 2>/dev/null | grep -q 'cap-distinct-series'"

# The route is a TEMPLATE (`"<intent>"` and `<workflow-id>` are placeholders), so
# "followable" means: fill the two placeholders and it runs. Filled with a real
# workflow it is also how this arm mints the real sub-task cell B needs.
ROUTE="$(printf '%s' "$A1" | sed -n 's/.*route: `\(jigc [^`]*\)`.*/\1/p' | head -1)"
echo "  route as printed : $ROUTE"
FILLED="${ROUTE//\"<intent>\"/\"cap distinct series\"}"
FILLED="${FILLED//<workflow-id>/sub-task}"
echo "  route as filled  : $FILLED"
bar "a route was printed"                 "test -n \"\$ROUTE\""
bar "…and carries both placeholders"      "printf '%s' \"\$ROUTE\" | grep -q '<intent>' && printf '%s' \"\$ROUTE\" | grep -q '<workflow-id>'"
step bash -c "$FILLED"
bar "the filled route mints the sub-task" "test -d .jigc/tasks/cap-distinct-series"

say "A · add-from-spec — the same check, and it precedes the spec read"
# No spec exists in this corpus. That is the point of the pair below: with an
# unknown workflow the door blocks on the WORKFLOW, never reaching the spec; with
# a valid one the very same address reaches `store.not-found`. The check order is
# therefore driven without authoring a spec.
#
# DECLARED, NOT DRIVEN: a seeded run over a committed spec. A `spec` criterion
# carries a `maps-to-test` code-anchor, so a committed spec that passes finalize
# is not cheap to make in a naive corpus; flow50 arm 2 covers the mint side.
step jigc milestone add-from-spec bound-the-store spec:nothing --workflow no-such-workflow
A2="$(jigc milestone add-from-spec bound-the-store spec:nothing --workflow no-such-workflow 2>&1)"; A2RC=$?
A3="$(jigc milestone add-from-spec bound-the-store spec:nothing 2>&1)"; A3RC=$?
bar "add-from-spec refuses the unknown workflow (exit non-zero)" "test $A2RC -ne 0"
bar "…carrying workflow-refs.unknown-workflow"                  "printf '%s' \"\$A2\" | grep -q 'workflow-refs.unknown-workflow'"
bar "…and says nothing was minted or seeded"                    "printf '%s' \"\$A2\" | grep -q 'nothing was minted, recorded or committed'"
bar "…while the same address under a VALID workflow reaches the spec read (store.not-found)" \
    "test $A3RC -ne 0 && printf '%s' \"\$A3\" | grep -q 'store.not-found'"
bar "the workflow check precedes the spec read (no store.not-found on the unknown-workflow path)" \
    "! printf '%s' \"\$A2\" | grep -q 'store.not-found'"

# ---------------------------------------------------------------------------
say "B · task discard <sub> — the COMMITTED record tells the truth"
step jigc milestone add-task bound-the-store "prune on overflow" --workflow dev-task
HEAD_B="$(git rev-parse HEAD)"
step jigc task discard cap-distinct-series
HEAD_C="$(git rev-parse HEAD)"
RECORD="$(jigc doc show milestone-record:bound-the-store --format json 2>/dev/null)"
printf '%s\n' "$RECORD"
STATUSES="$(printf '%s' "$RECORD" | node -e '
const j=JSON.parse(require("fs").readFileSync(0,"utf8"));
for (const t of j.sections.tasks) console.log(t.id+"="+t.status+" workflow="+t.workflow);
')"
echo "  record items:"; printf '%s\n' "$STATUSES" | sed 's/^/    /'
bar "the discarded sub-task reads discarded in the committed record" \
    "printf '%s' \"\$STATUSES\" | grep -qx 'cap-distinct-series=discarded workflow=sub-task'"
bar "…and its sibling stays active"                        "printf '%s' \"\$STATUSES\" | grep -q '^prune-on-overflow=active'"
bar "exactly one commit landed"                            "test \"\$(git rev-list --count $HEAD_B..$HEAD_C)\" = 1"
bar "…named as the discard"                                "git log -1 --format=%s | grep -q 'discard task:cap-distinct-series'"
bar "…and it is record-only (touches just the record file)" \
    "test \"\$(git show --stat --format= --name-only HEAD | tr -d ' ')\" = 'docs/milestone-records/bound-the-store.md'"
bar "the working area is gone"                             "test ! -e .jigc/tasks/cap-distinct-series"
LT="$(jigc milestone list-tasks bound-the-store 2>&1)"; LTRC=$?
printf '%s\n' "$LT"
bar "list-tasks no longer shows it active"                 "test $LTRC -eq 0 && ! printf '%s' \"\$LT\" | grep -q 'cap-distinct-series'"
bar "…and still shows the live one"                        "printf '%s' \"\$LT\" | grep -q 'prune-on-overflow'"

# ---------------------------------------------------------------------------
say "C · FRESH CLONE — list-tasks is a read; add-task is the door that re-seeds"
C="$(mktemp -d /tmp/clone-XXXXXX)"
git clone -q /work "$C/repo"
(
  cd "$C/repo"
  git config user.email 'clone@example.invalid'; git config user.name 'Clone'
  jigc setup >/dev/null 2>&1
  echo "  clone: $C/repo"
  echo "  .jigc/tasks before      : $(ls .jigc/tasks 2>&1 | tr '\n' ' ')"
  step jigc milestone list-tasks bound-the-store
  echo "  .jigc/tasks after read  : $(ls .jigc/tasks 2>&1 | tr '\n' ' ')"
  echo "  .jigc/milestones        : $(ls .jigc/milestones 2>&1 | tr '\n' ' ')"
  test ! -e .jigc/tasks && test ! -e .jigc/milestones; echo $? > "$C/read-wrote-nothing"
  jigc milestone list-tasks bound-the-store 2>&1 | grep -q 'prune-on-overflow'; echo $? > "$C/read-answered"
  step jigc milestone add-task bound-the-store "evict cold entries"
  echo "  .jigc/tasks after add   : $(ls .jigc/tasks 2>&1 | tr '\n' ' ')"
  test -d .jigc/tasks/prune-on-overflow; echo $? > "$C/door-reseeded"
  cat .jigc/tasks/prune-on-overflow/workflow 2>/dev/null > "$C/reseeded-workflow"
)
bar "the fresh clone's read answers from the record"           "test \"\$(cat $C/read-answered)\" = 0"
bar "…and writes nothing under .jigc/tasks/ or .jigc/milestones/" "test \"\$(cat $C/read-wrote-nothing)\" = 0"
bar "an operating door (add-task) DOES re-seed the absent area"   "test \"\$(cat $C/door-reseeded)\" = 0"
bar "…sourcing the workflow from the record (dev-task), not the pack default" \
    "test \"\$(cat $C/reseeded-workflow)\" = dev-task"

# ---------------------------------------------------------------------------
say "D · THE READ-VERB CARVE-OUT — all $READ_VERBS_EXPECTED VerbKind::Read leaves, snapshot before/after"
# A live ordinary task, so `task diff` / `task validate` address real state — with
# a fixed slug so the sweep never depends on `task list`'s order.
jigc start --workflow single-task "tidy the read path" --slug tidy-the-read-path >/dev/null 2>&1
T=tidy-the-read-path
bar "the live ordinary task exists" "test -d .jigc/tasks/$T"

# The whole workbench minus the log jigc itself writes on every invocation, plus
# HEAD and the index. `cksum` because the runtime image has no md5sum promise and
# the host has no md5sum at all.
snap() {
  find .jigc -type f -not -path '.jigc/logs/*' -exec cksum {} + | sort -k3
  printf 'HEAD %s\n' "$(git rev-parse HEAD)"
  git status --porcelain | sed 's/^/STATUS /'
}
SWEEP_BAD=0
printf '\n  %-24s %-6s %s\n' "verb" "exit" "changed under .jigc/ (excluding logs), HEAD, index"
sweep() { # <label> <argv…>
  local label="$1"; shift
  local before after rc changed bad
  before="$(snap)"
  "$@" >/dev/null 2>&1; rc=$?
  after="$(snap)"
  changed="$(diff <(printf '%s\n' "$before") <(printf '%s\n' "$after") | grep '^[<>]' | sed 's/^[<>] //' | awk '{ if ($1=="HEAD") print "HEAD"; else if ($1=="STATUS") print "index:" $NF; else print $NF }' | sort -u | tr '\n' ' ')"
  printf '  %-24s %-6s %s\n' "$label" "$rc" "${changed:-(nothing)}"
  bad="$(printf '%s' "$changed" | tr ' ' '\n' | grep -E '^(\.jigc/(tasks|milestones)/|HEAD$|index:)' || true)"
  if [ -n "$bad" ]; then echo "     ^ FORBIDDEN for a read verb: $bad"; SWEEP_BAD=1; fi
}
sweep "upgrade"              jigc upgrade
sweep "describe"             jigc describe
sweep "validate"             jigc validate
sweep "doc show"             jigc doc show milestone-record:bound-the-store
sweep "doc schema"           jigc doc schema adr
sweep "doc list"             jigc doc list
sweep "task list"            jigc task list
sweep "task diff"            jigc task diff "$T"
sweep "task validate"        jigc task validate "$T"
sweep "config get"           jigc config get docs-root
sweep "config list"          jigc config list
sweep "milestone list-tasks" jigc milestone list-tasks bound-the-store
echo
bar "no read verb created anything under .jigc/tasks/ or .jigc/milestones/, moved HEAD, or moved the index" \
    "test $SWEEP_BAD -eq 0"

say "D · the one admitted write — .jigc/index/ is a self-healing derived cache, checked not assumed"
if [ -f .jigc/index/edges.json ]; then
  STAMP="$(node -e 'console.log(JSON.parse(require("fs").readFileSync(".jigc/index/edges.json","utf8")).stamp||"")')"
  echo "  edges.json stamp : $STAMP"
  echo "  HEAD             : $(git rev-parse HEAD)"
  bar "the cache is stamped with the committed HEAD it was built from" "test \"\$STAMP\" = \"\$(git rev-parse HEAD)\""
  B2="$(snap)"; jigc task validate "$T" >/dev/null 2>&1; A2S="$(snap)"
  bar "a second run of the materializing verb is a fixed point (changes nothing)" "test \"\$B2\" = \"\$A2S\""
else
  echo "  MEASURED · no .jigc/index/edges.json was materialized by any read verb on this corpus"
  echo "             (the carve-out went unexercised here; read_verb_acts_nothing.rs keeps it exercised)"
fi

say "SUMMARY"
echo "  read verbs this arm was written against : $READ_VERBS_EXPECTED (the registry fence lives in read_verb_acts_nothing.rs)"
echo "  the per-verb change table above is an observation; only .jigc/tasks/, .jigc/milestones/, HEAD and the index are asserted"
if [ "$FAIL" -eq 0 ]; then echo "ARM 12 PASS"; else echo "ARM 12 FAIL"; fi
exit "$FAIL"
