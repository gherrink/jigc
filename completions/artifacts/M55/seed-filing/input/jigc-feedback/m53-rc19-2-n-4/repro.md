```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
L=$(mktemp -d)/lw; git worktree add -q -b lwbr "$L"; cd "$L"
$JIGC start "record the linked decision" --workflow decided-task > /dev/null
$JIGC doc add-item decisions-log#entries --title "Linked branch decision"
printf 'Why.\n' | $JIGC doc set-slot decisions-log#entries/linked-branch-decision/why --from-file -
$JIGC doc set-field commit:record-the-linked-decision#header/type --value docs
printf 'record it\n' | $JIGC doc set-slot commit:record-the-linked-decision#summary --from-file -
$JIGC task finalize record-the-linked-decision; echo "exit $?"   # 0, lands on lwbr, not main
cd "$REPO"
git merge-base --is-ancestor HEAD lwbr && echo "main's HEAD is an ancestor of lwbr"
$JIGC validate; echo "exit $?"
#   0; advisory · file-state.hash-matches — on-disk content of `docs/decisions-log.md` differs
#   from the recorded state;  route: the baseline lags `HEAD`; absorbed at the next finalize
#   (the baseline holds lwbr's bytes, a commit AHEAD of this HEAD: it does not lag it)
```
