#!/usr/bin/env bash
# 05-milestone-boundary.sh — protocol.md §5 arm 05 (M46 Increment 2, T1).
#
# WHICH SET THIS ARM ITERATES: the **class's defining case-set** — the three
# states a repository's fan-out worktrees can be in at the milestone boundary,
# named from the code's own `LeftoverVerdict`/linkage discriminator rather than
# from a reported repro:
#
#   1. OWN linkage        a worktree this repo registered and git can read
#   2. FOREIGN linkage    a `cp -R` of a repo: the copy's worktrees resolve
#                         through the SOURCE's admin dir, which still exists
#   3. NO linkage         an `mv` of a repo: the admin dir it pointed at is gone
#
# M46 re-derived the boundary's subject as the **on-disk path** rather than the
# registered set. Before that, a copied repo landed a docs-only commit at exit 0
# over a directory that was holding the work — the sub-agent's code silently not
# committed, and the milestone record flipped to a terminal state on the way out.
#
# Cell 3 splits, and the split is the interesting part: whether the boundary has
# ANY other work to land decides whether it refuses or proceeds-and-reports. Both
# halves are driven, because "it refuses" and "it says what it could not count"
# are different promises and only one of them is about data loss.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }

jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

# Build a milestone with one provisioned sub-task holding STAGED CODE. Copies are
# plain recursive copies on purpose: that is the shape under test, and
# implementation/pinning.md §4 warns it is unsafe for exactly the reason this arm
# is about — a copied worktree's `.git` file holds an absolute path back into the
# source.
build() { # <dest> ; leaves a repo at <dest> with staged sub-task code
  rm -rf "$1"; cp -R /work "$1"
  ( cd "$1" \
    && jigc milestone create "bound the store" >/dev/null 2>&1 \
    && jigc milestone add-task bound-the-store "cap distinct series" >/dev/null 2>&1 \
    && jigc milestone provision bound-the-store >/dev/null 2>&1 \
    && cd .jigc/worktrees/cap-distinct-series \
    && echo 'export const cap = 10_000;' > src/cap.ts \
    && git add src/cap.ts )
}
# …and one that ALSO holds a staged doc, authored from inside the worktree (the
# only place a sub-task's own doors will run — a sub-task is pinned to the
# milestone's base, and running from the parent checkout is refused by design).
build_with_doc() { # <dest>
  build "$1"
  ( cd "$1/.jigc/worktrees/cap-distinct-series" \
    && jigc doc create adr --title "Cap distinct series at a ceiling" \
         --task cap-distinct-series >/dev/null 2>&1
    for s in context decision consequences; do
      echo 'Bounded memory beats an unbounded series count.' \
        | jigc doc set-slot "adr:cap-distinct-series#$s" --from-file - \
            --task cap-distinct-series >/dev/null 2>&1
    done )
}

# ---------------------------------------------------------------------------
say "1 · OWN linkage — the ordinary success path, as the baseline"
build /tmp/own
( cd /tmp/own && jigc milestone finalize bound-the-store 2>&1 ) > /tmp/own.out
cat /tmp/own.out
bar "the sub-task's staged code lands" "grep -q 'src/cap.ts' /tmp/own.out"

say "2 · FOREIGN linkage — a cp -R of a repo whose SOURCE still exists"
# This is the cell M46 fixed. The copy's worktree resolves through the source's
# admin dir, so the *registered set* says nothing useful about it; the on-disk
# path does.
build /tmp/src-live
cp -R /tmp/src-live /tmp/copied
( cd /tmp/copied && jigc milestone finalize bound-the-store 2>&1 ) > /tmp/copied.out
CRC=$?
cat /tmp/copied.out
bar "the copy lands the sub-agent's code rather than a docs-only commit" \
    "grep -q 'src/cap.ts' /tmp/copied.out"
bar "…and does not silently report having provisioned nothing" \
    "! grep -qi 'no worktree provisioned' /tmp/copied.out"

say "3a · NO linkage, nothing else to land — the boundary REFUSES"
# An mv leaves every worktree pointing at an admin dir that is gone. With no docs
# to promote either, landing would commit only jigc's own bookkeeping AND flip the
# record to a terminal state — after which the milestone can never be finalized.
# Refusing is the whole point.
build /tmp/to-move
mv /tmp/to-move /tmp/moved
( cd /tmp/moved && jigc milestone finalize bound-the-store 2>&1 ) > /tmp/moved.out
( cd /tmp/moved && jigc milestone finalize bound-the-store >/dev/null 2>&1 ); MRC=$?
cat /tmp/moved.out
bar "it refuses rather than landing a bookkeeping-only commit" "test $MRC -ne 0"
bar "the refusal is milestone.zero-contribution"  "grep -q 'would land no work' /tmp/moved.out"
bar "…and says what landing anyway would cost — the terminal record" \
    "grep -q 'could never be finalized again' /tmp/moved.out"
bar "…and routes at provision, the verb that repairs it" \
    "grep -q 'jigc milestone provision bound-the-store' /tmp/moved.out"

say "3b · NO linkage, but docs DO land — it proceeds and NAMES what it could not count"
# Docs live in `.jigc/tasks/<sub>/docs/` and travel with a move; worktree linkage
# does not. So this is the state where the boundary must both land the docs and
# be honest that the code was uncountable. M46's declared bound at this door is
# that the loss is made VISIBLE, not prevented — this is that promise, driven.
build_with_doc /tmp/to-move-2
mv /tmp/to-move-2 /tmp/moved-2
( cd /tmp/moved-2 && jigc milestone finalize bound-the-store 2>&1 ) > /tmp/moved2.out
cat /tmp/moved2.out
bar "the docs still promote" "grep -q 'promoted docs/decisions/' /tmp/moved2.out"
bar "the unreadable worktree is NAMED, with its path" \
    "grep -q 'unreadable worktree at .jigc/worktrees/cap-distinct-series' /tmp/moved2.out"
bar "…and it says plainly that no code was counted" \
    "grep -q 'no code counted' /tmp/moved2.out"

say "3b · and the same fact reaches the machine surface, not only the prose"
build_with_doc /tmp/to-move-3
mv /tmp/to-move-3 /tmp/moved-3
( cd /tmp/moved-3 && jigc milestone finalize bound-the-store --format json 2>&1 ) > /tmp/moved3.json
cat /tmp/moved3.json; echo
bar "the --format json envelope carries worktree_unreadable" \
    "grep -q 'worktree_unreadable' /tmp/moved3.json"
bar "…and it is valid JSON, not prose with a key in it" \
    "node -e 'JSON.parse(require(\"fs\").readFileSync(\"/tmp/moved3.json\",\"utf8\"))'"

say "SUMMARY"
echo "  three linkage states driven: own · foreign (cp -R) · none (mv), the last split"
echo "  on whether anything else was landing. M46's declared bound at this door —"
echo "  the loss is visible, not prevented — is cells 3a and 3b together."
if [ "$FAIL" -eq 0 ]; then echo "ARM 05 PASS"; else echo "ARM 05 FAIL"; fi
exit "$FAIL"
