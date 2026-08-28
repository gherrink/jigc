#!/usr/bin/env bash
# 09-increment-8-doors.sh — protocol.md §5 arm 09 (M46 Increment 8).
#
# WHICH SET THIS ARM ITERATES: a **derivation stated as one**. Increment 8 is a
# surface batch — seven repairs at seven doors, sharing no registry and no code
# path. The set is therefore the increment's own task list, and this arm is honest
# that it is a hand-enumerated set rather than dressing it up as an axis.
#
#   T1  the survivable hook-rejection frame on a docs-only finalize   -> arm 03/R2
#   T2  jigc milestone finalize --help
#   T3  a write verb at an unstaged address (three ProvisionRoute shapes)
#   T4  the sub-task base-pin refusal
#   T5  jigc describe <positional>
#   T6  the router's closing text
#   T7  step:locate-from-spec's read-back
#
# T1 is driven by R2's rehearsal and by the B1 blind arm, not here — a hook plant
# needs a corpus of its own, and duplicating it would double-count the coverage.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z]/{print $1; exit}'; }

jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

say "T5 · jigc describe <positional> — a tip where rc.11 printed a bare clap error"
step jigc describe adr
D="$(jigc describe adr 2>&1)"
bar "it still refuses the single-item form"       "printf '%s' \"\$D\" | grep -q 'unexpected argument'"
bar "…but now says the form is not built"         "printf '%s' \"\$D\" | grep -q 'single-item form is not built'"
bar "…and names the three narrowing flags"        "printf '%s' \"\$D\" | grep -q -- '--workflows' && printf '%s' \"\$D\" | grep -q -- '--doctypes' && printf '%s' \"\$D\" | grep -q -- '--commands'"
bar "…and points at the resolution trace for one workflow" \
    "printf '%s' \"\$D\" | grep -q 'jigc start --explain'"

say "T2 · jigc milestone finalize --help says what the door actually does"
step jigc milestone finalize --help
H="$(jigc milestone finalize --help 2>&1)"
bar "it names BOTH halves the boundary lands" \
    "printf '%s' \"\$H\" | grep -q 'the join.s suffix-resolved doc bodies' && printf '%s' \"\$H\" | grep -q 'code staged in each sub-task worktree'"
bar "…and says a blocking join finding commits nothing" \
    "printf '%s' \"\$H\" | grep -q 'commits nothing'"

say "T3 · a write verb at an unstaged address — the ProvisionRoute shapes"
jigc start --workflow record-decision "probe the write path" >/dev/null 2>&1
T="$(newtask)"
step sh -c "echo x | jigc doc set-slot 'adr:not-created-yet#context' --from-file - --task $T"
P1="$(echo x | jigc doc set-slot 'adr:not-created-yet#context' --from-file - --task "$T" 2>&1)"
bar "an un-created address says so, rather than writing nowhere" \
    "printf '%s' \"\$P1\" | grep -q 'no staged instance'"
bar "…and routes at the verb that provisions it" \
    "printf '%s' \"\$P1\" | grep -q 'jigc doc create'"
bar "…and warns the id comes from the TITLE, not the task id — the M44 footgun" \
    "printf '%s' \"\$P1\" | grep -q 'derives the id from the title'"

# The compose-provisioned doc is the contrasting cell: `commit` exists from the
# mint, so the same verb at the same shape of address WRITES rather than routes.
step sh -c "echo 'body text' | jigc doc set-slot 'commit:$T#body' --from-file - --task $T"
P2="$(echo 'body text' | jigc doc set-slot "commit:$T#body" --from-file - --task "$T" 2>&1)"
bar "a compose-provisioned doc accepts the write — the contrast that makes T3 legible" \
    "printf '%s' \"\$P2\" | grep -q 'set slot commit:'"

say "T4 · the sub-task base-pin refusal routes at provision, not at discard"
jigc milestone create "bound the store" >/dev/null 2>&1
jigc milestone add-task bound-the-store "cap distinct series" >/dev/null 2>&1
step jigc workflow sub-task --task cap-distinct-series
B="$(jigc workflow sub-task --task cap-distinct-series 2>&1)"
bar "it explains the pin rather than just refusing" \
    "printf '%s' \"\$B\" | grep -q 'pinned to base'"
bar "…names the worktree the work belongs in" \
    "printf '%s' \"\$B\" | grep -q '.jigc/worktrees/cap-distinct-series'"
bar "…and routes at provision — NOT at task discard, which used to exit 0 on a live record" \
    "printf '%s' \"\$B\" | grep -q 'jigc milestone provision bound-the-store'"
bar "…and does not offer discarding the sub-task" \
    "! printf '%s' \"\$B\" | grep -q 'jigc task discard cap-distinct-series'"

say "T6 · the router's closing text — the increment's own audit finding, re-measured"
jigc start "add a rate limiter to the ingest path" > /tmp/router.txt 2>&1
jigc describe --workflows > /tmp/allwf.txt 2>&1
step tail -6 /tmp/router.txt
R="$(cat /tmp/router.txt)"
bar "the closing text points at describe --workflows for the fuller set" \
    "printf '%s' \"\$R\" | grep -q 'jigc describe --workflows'"

# The measurement, printed rather than asserted. Increment 8's own audit finding
# was that this sentence overclaimed — "each with the reason it sits off the
# catalog" is false for the `creates-task: false` workflows, because M43's
# `suppressed:` fence binds only `selectable: false`. It was repaired as a
# law-1 scope fix. This re-derives the population against the shipped binary so
# the trial reads a number rather than a memory.
echo
echo "  MEASURED · workflows absent from the router catalog, and whether each states why:"
node -e '
const fs=require("fs");
const router=fs.readFileSync("/tmp/router.txt","utf8"), all=fs.readFileSync("/tmp/allwf.txt","utf8");
const cat=new Set([...router.matchAll(/^\s*[-*]\s+([a-z][a-z0-9-]+)\s+—/gm)].map(m=>m[1]));
// Split into blank-line-separated blocks instead of matching a body with a lazy
// quantifier: under the "m" flag `$` matches at EVERY line end, so `[\s\S]*?`
// stops at the first newline and every block looks empty. That bug made this
// print "21 without a reason" where the true answer is 3 — a precise-looking
// number that was false, produced by the instrument rather than the product.
const blocks=all.split(/\n\s*\n/);
const byName={};
for(const b of blocks){ const m=b.match(/^([a-z][a-z0-9-]+) is /); if(m) byName[m[1]]=(byName[m[1]]||"")+b; }
const absent=Object.keys(byName).filter(n=>!cat.has(n)).sort();
const noReason=absent.filter(n=>!byName[n].includes("It is hidden from the router catalog"));
console.log("           catalog: "+cat.size+" selectable · absent: "+absent.length+
            " · of those, WITHOUT a stated reason: "+noReason.length);
console.log("           without a reason: "+(noReason.join(", ")||"(none)"));
'
echo "           -> recorded in pre-trial-findings.md (PT-C) with both readings."
echo "           -> NOT adjudicated here: this arm builds the instrument."

say "T7 · step:locate-from-spec names the commit doc, not the sanctioned-optional spec write"
step jigc workflow implement-from-spec --preview
L="$(jigc workflow implement-from-spec --preview 2>&1)"
bar "the composed step text is readable without minting a task" \
    "test -n \"\$L\""
bar "…and its read-back names the task's commit doc" \
    "printf '%s' \"\$L\" | grep -q 'commit:'"

say "SUMMARY"
echo "  six of Increment 8's seven doors driven here; T1 (the hook frame) is R2's and B1's."
echo "  T6 is reported as a measurement against the shipped binary, not as a verdict."
if [ "$FAIL" -eq 0 ]; then echo "ARM 09 PASS"; else echo "ARM 09 FAIL"; fi
exit "$FAIL"
