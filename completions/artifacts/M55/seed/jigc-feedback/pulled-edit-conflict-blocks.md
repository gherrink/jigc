---
kind: bug
found-in: milestone:M55-planning/F4
about: reconciliation.conflict-block
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: l1_pull_absorption::a_pulled_edit_lands_through_the_task_that_edits_the_doc
date: 2026-10-03
schema-version: 1
---

# A pulled edit conflict-blocks the next task that edits the doc

## Description

After a pull or merge that changed a committed doc, the next task editing it was blocked `reconciliation.conflict-block`, although the task's copy already held the pulled bytes. `start`'s absorb was in memory only. The route said to revert the external edit, which is the teammate's commit, and the two workarounds, `jigc ingest` or any unrelated landed finalize, were not named. It is the baseline ledger's **L1**, the first of the three latent defects `design/findings-channel.md` → 6 fixes, and it would hit every pull under the branch-per-milestone model. The capabilities gap-detector located it at the `DRIFTED + TOUCHED` arm of `crates/engine/src/file_state.rs` and sketched the fix.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
git clone -q --bare "$REPO" "$RIG/origin.git"; git remote add origin "$RIG/origin.git"
git fetch -q origin; git branch -q --set-upstream-to=origin/main main
git clone -q "$RIG/origin.git" "$RIG/mate"                              # a teammate's clone
sed -i.bak 's/^A deterministic CLI assembles exactly the context a task needs\.$/&\
A teammate'"'"'s pulled line./' "$RIG/mate/VISION.md"; rm "$RIG/mate/VISION.md.bak"   # a line in the thesis
git -C "$RIG/mate" -c user.name=Mate -c user.email=mate@example.com commit -qam "docs: teammate edit"
git -C "$RIG/mate" push -q origin main; git pull -q --ff-only            # the pull
T=$($JIGC start --workflow single-task "sharpen the open questions" | sed -n 's/^task minted: //p')
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value vision --task $T > /dev/null
printf 'sharpen the open questions\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
printf 'Seed probe prose, written by the task.\n' | $JIGC doc set-slot vision:vision#open-questions --from-file - --task $T > /dev/null
$JIGC task validate $T; echo "validate: exit $?"     # 0, advisory reconciliation.absorb at VISION.md
$JIGC task finalize $T > /dev/null 2>&1; echo "finalize: exit $?"           # 0
git show HEAD:VISION.md | grep -c -e "A teammate's pulled line." -e "Seed probe prose"   # 2: both sides land
```

## Resolution

Fixed in M55 Increment 4 (`8b4f012f`, *a touched doc whose drift is a pull lands instead of conflict-blocking*), with the store-scope arm in `2c284c48`. When a touched doc's on-disk bytes equal its blob at the task's base pin, the drift predates the task and is absorbed through the untouched arm's whole body, conformance check first. An edit made during the task still conflict-blocks. At store scope, `jigc validate` grades a baseline that lags `HEAD` advisory, routed *the baseline lags `HEAD`; absorbed at the next finalize* (`design/findings-channel.md` → 6, L1).

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`), over a real pull from a local bare `origin`. After the teammate's `VISION.md` commit is pulled, a `single-task` that edits `vision#open-questions` reads advisory `reconciliation.absorb` at `VISION.md` from `jigc task validate`, at exit 0. Its `jigc task finalize` exits 0, and the landed `VISION.md` carries both the teammate's line and the task's prose.
