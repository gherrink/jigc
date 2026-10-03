---
kind: bug
found-in: review:M52-per-axis/(4,DEFECT 2)
about: config.repoint-failed
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Repoint-failed message prints an absolute host path

## Description

`config.repoint-failed` renders an absolute host path in the `message` that the pinned findings envelope carries. That happens when `write_scalar` fails after the `git mv` batch has landed. The path is inside the repository, and the repo-relative spelling is already in hand in the finding's own `location.address`. Law 1's printed-path fence has one home, `render::repo_relative`, which keeps the honest absolute only for *a path genuinely outside the repository*. The second half is in the record. `repo_relative_paths.rs`' `UNSWEPT_PRODUCERS` row for `config.rs` gives its reason as *an error channel, not a finding surface*. Yet one of those sites' text **is** the `message` of a blocking finding with a `(code, target)` key, on the text arm and the 1.0-pinned JSON arm alike. The rollback itself is clean.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. With `.jigc/config` made read-only, `jigc config set docs-root documentation --format json` exits 1. Its `message` reads *could not write /…/repo/.jigc/config/manifest.yaml: Permission denied*, with the host's absolute path, while `key.target` and `location.address` read `.jigc/config/manifest.yaml`. The `UNSWEPT_PRODUCERS` reason still says *not a finding surface*.

## Repro

```sh
rig=$(dev/jigc-rig vendored) || exit; eval "$rig"      # docs/specs/ and docs/architecture/ to move
chmod 0555 .jigc/config
$JIGC config set docs-root documentation --format json  # exit 1
#   "code": "config.repoint-failed",
#   "message": "`docs-root` was not set to `documentation`: could not write
#               <absolute path of $REPO>/.jigc/config/manifest.yaml: Permission denied … —
#               the re-point was undone",
#   "location": { "address": ".jigc/config/manifest.yaml", … }
chmod 0755 .jigc/config; git status --short             # empty: the rollback is clean
```

## Resolution
