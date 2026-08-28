#!/usr/bin/env bash
# 06-pre-guard-repair.sh — protocol.md §5 arm 06 (M46 Increment 5, and Inc 1's
# observable consequence).
#
# WHICH SET THIS ARM ITERATES: a **derivation, stated as one, with a declared
# hole**. Increment 5's deliverable was "the pre-guard repair route, both arms":
#
#   arm 1  conformance.item-heading-unanchored stops naming a verb the state
#          refuses, and prints the repair itself
#   arm 2  reconciliation.conflict-block gains a migration arm routing at
#          `jigc unmanage <source>`
#
# **Arm 2 is NOT reached here, and this file says so rather than implying it is.**
# See the DECLARED GAP block below. An unreached probe that is visibly blank is
# the whole reason walk arms became scripts (cue-card-postmortem.md §6 step 4);
# one that is quietly absent is the failure that postmortem exists for.
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

say "0 · land a committed spec — a doctype with a repeatable item section"
# `adr` cannot host this finding: with no repeatable section a stray `###` lands in
# slot prose and draws `conformance.slot-heading-depth` instead. The finding under
# test is about an ITEM boundary, so the subject has to be a doctype that has one.
jigc start --workflow plan "bound the distinct series count" >/dev/null 2>&1
T="$(newtask)"
S="$(jigc doc create spec --title 'Bound the distinct series count' --task "$T" 2>&1 \
      | grep -oE '^spec:[a-z0-9-]+$' | head -1)"
echo "spec: $S"
echo 'Cap the number of distinct series the store will hold.' \
  | jigc doc set-slot "$S#goal" --from-file - --task "$T" >/dev/null 2>&1
echo 'Unbounded series counts let one noisy writer evict everyone else.' \
  | jigc doc set-slot "$S#context" --from-file - --task "$T" >/dev/null 2>&1
ITEM="$(jigc doc add-item "$S#criteria" --title 'The store caps distinct series' --task "$T" 2>&1 \
        | grep -oE '^spec:[a-z0-9-]+#criteria/[a-z0-9-]+$' | head -1)"
echo 'The store rejects a new series past the ceiling.' \
  | jigc doc set-slot "$ITEM/statement" --from-file - --task "$T" >/dev/null 2>&1
jigc doc set-field "commit:$T#header/type" --value docs --task "$T" >/dev/null 2>&1
printf 'bound the distinct series count\n' \
  | jigc doc set-slot "commit:$T#summary" --from-file - --task "$T" >/dev/null 2>&1
jigc task finalize "$T" >/dev/null 2>&1
F="$(git ls-files docs/specs/ | head -1)"
bar "the spec is committed" "test -n '$F' && test -f '$F'"

say "1 · a human hand-adds an item heading with no anchor — the pre-guard state"
printf '\n### A hand-added criterion\n\nWith no anchor.\n' >> "$F"
git add -A
git -c user.name='A Human' -c user.email='human@example.invalid' \
    commit -qm 'docs: add a criterion by hand'
step jigc validate
V="$(jigc validate 2>&1)"

bar "the finding fires"                    "printf '%s' \"\$V\" | grep -q 'conformance.item-heading-unanchored'"
bar "it says WHY the parser cares — it reads as an item boundary" \
    "printf '%s' \"\$V\" | grep -q 'reads it as an item boundary'"
bar "it prints the paste-able REPAIR rather than describing one" \
    "printf '%s' \"\$V\" | grep -q '{#<id>}'"
bar "it covers the prose branch too — demote, do not anchor" \
    "printf '%s' \"\$V\" | grep -q 'demote it to'"
bar "…and says what a valid id looks like, so the paste can be completed" \
    "printf '%s' \"\$V\" | grep -q 'lowercase-kebab slug unique among'"

# M46's fix was to stop naming a verb the state refuses. `jigc doc add-item` cannot
# run here — the doc is committed and no task holds it — so naming it was a route
# that could not be followed.
bar "it does NOT name jigc doc add-item, the verb this state refuses" \
    "! printf '%s' \"\$V\" | grep -q 'jigc doc add-item'"
bar "conformance.* is route-exempt by declaration, so the MESSAGE carries the repair" \
    "! printf '%s' \"\$V\" | grep -A1 'item-heading-unanchored' | grep -q '^  route:'"

say "2 · the same edit at a task door — where it actually gates"
# Store scope is report-only (exit 0); the pre-guard state blocks at a task door.
# That split is the reconciliation model's, and it is what makes the store sweep
# safe to run continuously.
bar "the store sweep is report-only about it" \
    "printf '%s' \"\$V\" | grep -q 'report-only at store scope'"
bar "…and says which doors it DOES gate at" \
    "printf '%s' \"\$V\" | grep -q 'these gate at'"
bar "…naming all three of them" \
    "printf '%s' \"\$V\" | grep -q 'jigc task validate' && printf '%s' \"\$V\" | grep -q 'jigc task finalize' && printf '%s' \"\$V\" | grep -q 'jigc milestone finalize'"

say "3 · the out-of-band edit is detected, not silently absorbed (Inc 1's consequence)"
bar "the hand edit is seen as drift against the recorded baseline" \
    "printf '%s' \"\$V\" | grep -q 'file-state.hash-matches'"
bar "…and routes at re-authoring through the owning workflow" \
    "printf '%s' \"\$V\" | grep -q 're-author it through the owning workflow'"

cat <<'GAP'

=== DECLARED GAP — arm 2 of Increment 5 is NOT reached by this arm

`reconciliation.conflict-block`'s new migration arm — the one routing at
`jigc unmanage <source>` — needs a migration task whose source is a MANAGED doc
carrying a baseline, then an out-of-band edit to that same path. Driven here, a
`jigc migrate <path> --as adr` over a FOREIGN file does not reach it: the source
has no baseline, so the reconciliation guard never engages and the task door
reports `schema-conformance.unadopted-instance` instead.

Reaching it needs the same-path carve-out (M43) over an already-managed doc. That
is a longer setup than this arm should carry silently, so it is declared rather
than faked, and it is NOT counted as covered anywhere.

Its standing fence is the Increment 5 suite; the trial's coverage table (§6) must
record this cell as test-fenced, not trial-reached.

=== ALSO NOT REACHED — Increment 1's lock and merge

Only its observable consequence (section 3 above) is drivable. The base-relative
merge and the save-scoped lock need CONCURRENT jigc processes; the wave itself
reproduced the symptom with a 2000-round harness. Test-fenced, and §6 says so.
GAP

say "SUMMARY"
echo "  Increment 5 arm 1 driven; arm 2 declared unreached with its reason."
echo "  Increment 1 reached only at its observable consequence, declared likewise."
if [ "$FAIL" -eq 0 ]; then echo "ARM 06 PASS"; else echo "ARM 06 FAIL"; fi
exit "$FAIL"
