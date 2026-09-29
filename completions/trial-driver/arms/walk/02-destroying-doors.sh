#!/usr/bin/env bash
# 02-destroying-doors.sh — protocol.md §5 arm 02 (M46 Increment 2).
#
# WHICH SET THIS ARM ITERATES, said in the arm as flow 49 requires:
# a **code-side registry** — `crates/cli/src/milestone.rs`'s `DESTROYING_DOORS`,
# four members, whose own doc-comment splits the matrix this arm walks:
#
#     "The refusal cells iterate the members whose `DestroyingDoor::code` is
#      `Some`; the narration cells iterate all four."
#
#   PROVISION_DOOR  jigc milestone provision   code: milestone.leftover-holds-work
#   DISCARD_DOOR    jigc milestone discard     code: milestone.dirty-worktree
#   UNINSTALL_DOOR  jigc uninstall             code: uninstall.dirty-worktree
#   FINALIZE_DOOR   jigc milestone finalize    code: None  <- narration only
#
# `FINALIZE_DOOR` carries no code by design, and the reason is M46's declared
# bound rather than an oversight: at that boundary the staged set is already
# committed, so a refusal would fire on the ordinary fan-out SUCCESS path and
# train `--force` into reflex. **At that door the loss is made visible, not
# prevented** — and this arm's job is to show that it is at least visible.
#
# THE FENCE IS NOT THIS ARM. `flow49_acceptance.rs` iterates the real registry in
# code and will redden if a fifth door lands. This arm drives the same doors LIVE,
# and states the count it was written against so it is visibly stale if that
# number moves. A walk arm is the live companion to a fence, never a substitute.
#
# ORDER TRAP, learned by getting it wrong: the `--ignored` narration only fires
# when the ignore rules exist **at the worktree's base commit**. A `.gitignore`
# committed after `provision` leaves the worktree detached at an earlier base, git
# inside it does not consider the files ignored, and the narration says
# "(never staged)" instead of "(ignored by git)" — the arm passes while testing
# the wrong cell. So the ignore rules are committed FIRST, below, deliberately.
set -uo pipefail
cd /work

DOORS_EXPECTED=4
REFUSING_DOORS_EXPECTED=3

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }

say "0 · adopt, and commit the ignore rules BEFORE provisioning (see the order trap)"
jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
printf 'build/\n*.env\n' >> .gitignore
git add .gitignore
git -c user.name='Corpus Owner' -c user.email='owner@example.invalid' \
    commit -qm 'chore: ignore build output and env files'
jigc --version

# ---------------------------------------------------------------------------
say "A · THE REFUSAL CELLS — the three doors whose DestroyingDoor::code is Some"
# The subject is a leftover git cannot read a repository in: the shape a `cp -R`
# or `mv` of a repo leaves at a worktree path. M48 replaced the *registered-set*
# subject with a fail-closed classifier, and M46 re-derived the subject as
# destruction; this is the verdict neither can prove disposable.
jigc milestone create "bound the store" >/dev/null 2>&1
jigc milestone add-task bound-the-store "cap distinct series" >/dev/null 2>&1
jigc milestone add-task bound-the-store "prune on overflow" >/dev/null 2>&1
jigc milestone provision bound-the-store >/dev/null 2>&1

W=".jigc/worktrees/cap-distinct-series"
( cd "$W" && echo "staged"   > staged.ts && git add staged.ts \
           && echo "unstaged" >> src/store.ts \
           && echo "untracked" > untracked.ts )
echo 'SECRET=1' > "$W/secrets.env"
mkdir -p "$W/build" && echo 'x' > "$W/build/out.js"
# De-register WITHOUT deleting. `git worktree remove` deletes the directory, so it
# cannot produce this state; removing the admin dir can.
rm -rf .git/worktrees/cap-distinct-series
git worktree prune

step jigc milestone provision bound-the-store
step jigc milestone discard bound-the-store
step jigc uninstall

say "A · bars — the §5 standard: names the path · says what is lost · names the consent"
P1="$(jigc milestone provision bound-the-store 2>&1)"; P1RC=$?
P2="$(jigc milestone discard bound-the-store 2>&1)";   P2RC=$?
P3="$(jigc uninstall 2>&1)";                           P3RC=$?

bar "provision refuses (exit non-zero)"            "test $P1RC -ne 0"
bar "provision carries milestone.leftover-holds-work" \
    "printf '%s' \"\$P1\" | grep -q 'milestone.leftover-holds-work'"
bar "provision names the path"                     "printf '%s' \"\$P1\" | grep -q 'cap-distinct-series'"
bar "provision names the consent"                  "printf '%s' \"\$P1\" | grep -q -- '--force'"

bar "discard refuses (exit non-zero)"              "test $P2RC -ne 0"
bar "discard carries milestone.dirty-worktree"     "printf '%s' \"\$P2\" | grep -q 'milestone.dirty-worktree'"
bar "discard names the path"                       "printf '%s' \"\$P2\" | grep -q 'cap-distinct-series'"
bar "discard names the consent"                    "printf '%s' \"\$P2\" | grep -q -- '--force'"

bar "uninstall refuses (exit non-zero)"            "test $P3RC -ne 0"
bar "uninstall carries uninstall.dirty-worktree"   "printf '%s' \"\$P3\" | grep -q 'uninstall.dirty-worktree'"
bar "uninstall names the path"                     "printf '%s' \"\$P3\" | grep -q 'cap-distinct-series'"
bar "uninstall names the consent"                  "printf '%s' \"\$P3\" | grep -q -- '--force'"

say "A · and the consent, run verbatim, WORKS — criterion (d) of the §5 standard"
# This is ALSO the provision door's narration cell, and the first run of this arm
# proved why it has to be: `provision` is idempotent over a LIVE worktree — it
# "reuses a live worktree untouched" — so pointing the narration bar at a healthy
# worktree asserts that a door narrates a destruction it correctly is not doing.
# The provision door destroys exactly one thing, a LEFTOVER, so that is where its
# narration lives.
FORCED="$(jigc milestone provision bound-the-store --force 2>&1)"
printf '%s\n' "$FORCED"
bar "the leftover is gone after --force"     "test ! -e '$W/secrets.env'"
bar "provision --force narrates what it took" \
    "printf '%s' \"\$FORCED\" | grep -qi 'discards work that is not in git'"
bar "provision names it as a leftover, not a worktree" \
    "printf '%s' \"\$FORCED\" | grep -q 'leftover directory'"
bar "and says the bytes are unrecoverable" \
    "printf '%s' \"\$FORCED\" | grep -q 'not recoverable'"
# NOTE, recorded rather than asserted: a leftover's enumeration carries NO
# "(ignored by git)" labels, and that is coherent — git cannot read a repository
# there, which is the whole reason the door refused. The --ignored axis is
# therefore only reachable at the doors that act on a LIVE worktree, below.

# ---------------------------------------------------------------------------
say "B · THE NARRATION CELLS — all four doors, including the two that were silent"
# M46: narration is NOT gated on --force, and `provision --force` / `uninstall
# --force` were the two silent ones. Each cell gets a live worktree holding
# ignored + never-staged bytes.
narrate_setup() { # <sub-task path>
  echo 'SECRET=1' > "$1/secrets.env"
  mkdir -p "$1/build" && echo 'x' > "$1/build/out.js"
  echo 'untracked' > "$1/untracked.ts"
}

say "B1 · discard --force over a LIVE worktree — and the whole --ignored axis"
narrate_setup ".jigc/worktrees/prune-on-overflow"
N2="$(jigc milestone discard bound-the-store --force 2>&1)"
printf '%s\n' "$N2"
bar "discard --force narrates"        "printf '%s' \"\$N2\" | grep -qi 'discards work that is not in git'"
bar "the --ignored axis is labelled"  "printf '%s' \"\$N2\" | grep -q 'ignored by git'"
bar "a never-staged file is labelled differently" \
    "printf '%s' \"\$N2\" | grep -q 'never staged'"
bar "an ignored DIRECTORY is named by its container, not its members (--ignored=matching)" \
    "printf '%s' \"\$N2\" | grep -q 'build/ (ignored by git)'"
bar "M46's declared bound holds: the loss is VISIBLE, not prevented (exit 0)" \
    "test ! -e '.jigc/worktrees/prune-on-overflow/secrets.env'"

say "B2 · milestone finalize — the door with NO refusal code, where loss is visible not prevented"
# The first run of this arm never reached this door's teardown: with nothing to
# land, `finalize` refuses up front ("would land no work", exit 3) and the
# narration is never reached. So the sub-task worktree gets REAL staged code —
# which is also the honest subject, since this door's declared bound is precisely
# that it fires on the ordinary fan-out SUCCESS path.
jigc milestone create "bound the store again" >/dev/null 2>&1
jigc milestone add-task bound-the-store-again "cap distinct series" >/dev/null 2>&1
jigc milestone provision bound-the-store-again >/dev/null 2>&1
WT=".jigc/worktrees/cap-distinct-series"
( cd "$WT" && echo 'export const cap = 10_000;' > src/cap.ts && git add src/cap.ts )
narrate_setup "$WT"
N3="$(jigc milestone finalize bound-the-store-again 2>&1)"
printf '%s\n' "$N3"
# `grep -qv` is NOT "the phrase is absent" — under GNU grep it selects lines that
# do not match, so on multi-line output it succeeds almost always and the bar is
# inert. (ugrep, which this repo's host uses, happens to read it the other way,
# so the bar would look fine when driven on the host and be dead in the container
# where it actually runs.) The unambiguous form is a negated plain match.
bar "finalize lands the sub-task code rather than refusing" \
    "! printf '%s' \"\$N3\" | grep -qi 'would land no work'"
bar "finalize actually committed the sub-task's code" \
    "printf '%s' \"\$N3\" | grep -q 'src/cap.ts'"
bar "finalize narrates the ignored bytes its teardown takes" \
    "printf '%s' \"\$N3\" | grep -qi 'discards work that is not in git'"

say "B3 · uninstall --force over a LIVE worktree (the other formerly-silent door)"
jigc milestone create "bound the store thrice" >/dev/null 2>&1
jigc milestone add-task bound-the-store-thrice "cap distinct series" >/dev/null 2>&1
jigc milestone provision bound-the-store-thrice >/dev/null 2>&1
narrate_setup ".jigc/worktrees/cap-distinct-series"
N4="$(jigc uninstall --force 2>&1)"
printf '%s\n' "$N4"
bar "uninstall --force narrates" "printf '%s' \"\$N4\" | grep -qi 'discards work that is not in git'"

# ---------------------------------------------------------------------------
say "C · the NON-DIRECTORY leftover at the uninstall door (M49's completion audit)"
# M49's audit found `jigc uninstall` destroying planted files at exit 0:
# `fanout_worktree_paths` filtered its subject with `path.is_dir()` — a claim about
# SHAPE where the door's question is about BYTES — so a regular file at a fan-out
# worktree path was invisible to M48's destroying-door guard AND to the loss
# narration, while `remove_dir_all(.jigc)` took it anyway. The fix dropped the
# filter and left the shape to `probe_leftover`'s fail-closed verdict.
#
# Both shapes are planted AT ONCE — a regular file and a real directory leftover —
# so the door has to answer for the FILE by name; a refusal that only names the
# directory would be the pre-fix behaviour wearing a green bar. B3 above tore the
# workbench down with `--force`, so the door is re-armed with a fresh `setup`.
jigc setup >/dev/null 2>&1
mkdir -p .jigc/worktrees/leftover-dir && echo 'sole copy' > .jigc/worktrees/leftover-dir/notes.txt
echo 'sole copy of a file' > .jigc/worktrees/leftover-file
ls -la .jigc/worktrees/
step jigc uninstall
C1="$(jigc uninstall 2>&1)"; C1RC=$?
# MEASURED on 1.0.0-rc.13 (f266770), 2026-09-04, and left as FAILing bars rather than
# re-worded to pass: the door refuses at exit 1 and the file survives — the audit's
# data-loss hole is closed — but the refusal is the fail-closed PROBE-ERROR shape
# (`could not read the leftover directory ".../leftover-file": Not a directory (os
# error 20)`, routed at "make sure git is on PATH … git worktree remove"), so it
# names neither the directory leftover beside the file nor `--force` as the consent,
# which cell A above asserts for the same door over a directory-shaped leftover.
bar "uninstall (no --force) refuses over the planted shapes (exit non-zero)" "test $C1RC -ne 0"
bar "…carries uninstall.dirty-worktree" "printf '%s' \"\$C1\" | grep -q 'uninstall.dirty-worktree'"
bar "…names the regular FILE at the worktree path" "printf '%s' \"\$C1\" | grep -q 'leftover-file'"
bar "…and the directory leftover beside it" "printf '%s' \"\$C1\" | grep -q 'leftover-dir'"
bar "…and names the consent" "printf '%s' \"\$C1\" | grep -q -- '--force'"
bar "the FILE still exists afterwards" "test -f .jigc/worktrees/leftover-file"
bar "…with its bytes intact" "grep -q 'sole copy of a file' .jigc/worktrees/leftover-file"
bar "the directory leftover still exists afterwards" "test -f .jigc/worktrees/leftover-dir/notes.txt"
bar ".jigc/ itself was not removed" "test -d .jigc/config"

say "SUMMARY"
echo "  doors this arm was written against : $DOORS_EXPECTED ($REFUSING_DOORS_EXPECTED refusing + 1 narrate-only)"
echo "  the registry fence lives in flow49_acceptance.rs, not here"
echo "  cell C (M49 audit): a regular FILE at a fan-out worktree path is a subject, not a shape"
if [ "$FAIL" -eq 0 ]; then echo "ARM 02 PASS"; else echo "ARM 02 FAIL"; fi
exit "$FAIL"
