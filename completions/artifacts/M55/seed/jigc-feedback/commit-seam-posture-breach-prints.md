---
kind: bug
found-in: review:M52-per-axis/(2,DEFECT C)
about: jigc task finalize
jigc-version: 1.0.0-rc.16
status: open
tier: tier-2
date: 2026-10-03
schema-version: 1
---

# Commit-seam posture breach prints no state clause or re-run

## Description

Suppose a repository posture breach (here a bisect) arrives after `jigc task finalize` has started its transaction and is caught at the commit seam. The door then prints one `repo.operation-in-progress` line and a route ending in *then re-run this command*. That is all it prints. Every other in-transaction failure at the same door prints a state-truth clause (*task … is intact — nothing was committed, your task's staged docs are still in …*) and the door's own copy-runnable re-run (`jigc task finalize <id>`). `design/finalize.md` → *6. Commit* requires both of them. Meanwhile `task.rs`'s `already_typed` passes the `repo.*` posture refusals through unchanged on purpose. So two stated homes contradict each other. The review drove it identically at `milestone finalize`'s fan-out seam. The M53 re-reviews kept it open through `1.0.0-rc.20` and widened it to the `--amend` arm.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. A git shim on `PATH` starts a bisect on the first `git add` of the run. Finalize then exits 1 with the posture code and the bare route, and states none of the state that is true: HEAD is unmoved, `work.txt` is still staged, and the task's working area is intact.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons --start quick-fix "seam probe") || exit; eval "$rig"
$JIGC doc set-field commit:seam-probe#header/type --value fix
printf 'seam\n' | $JIGC doc set-slot commit:seam-probe#summary --from-file -
echo w > work.txt; git add work.txt
$JIGC task validate seam-probe                # exit 0
REAL_GIT=$(command -v git); mkdir -p "$RIG/shim"
cat > "$RIG/shim/git" <<EOS
#!/bin/sh
if [ ! -e "$RIG/shim/fired" ]; then
  case " \$* " in *" add "*) touch "$RIG/shim/fired"
    "$REAL_GIT" -C "$REPO" bisect start; "$REAL_GIT" -C "$REPO" bisect bad;; esac
fi
exec "$REAL_GIT" "\$@"
EOS
chmod +x "$RIG/shim/git"
PATH="$RIG/shim:$PATH" $JIGC task finalize seam-probe     # exit 1
#   blocking · repo.operation-in-progress — a bisect is in progress — the repository is
#     not in a committable state
#     route: conclude it, or abandon it with `git bisect reset`, then re-run this command
git rev-parse HEAD                            # unmoved
git diff --cached --name-only                 # work.txt, still staged
ls .jigc/tasks/seam-probe/docs/               # commit:seam-probe.md provenance.json
```

## Resolution
