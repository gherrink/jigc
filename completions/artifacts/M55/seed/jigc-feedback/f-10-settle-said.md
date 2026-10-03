---
kind: feedback
found-in: review:M53-per-axis-rc20/(2,A2-3)
about: jigc task amend
jigc-version: 1.0.0-rc.20
status: resolved
tier: tier-3
pinned-by: UNPINNED: a record row; the fix struck a sentence in completions/artifacts/M53/f10-amend-settle.md, which no test reads, and the binary's side was already the text_json_parity_axis census row's DeclaredOut
date: 2026-10-03
schema-version: 1
---

# The F-10 settle said task amend JSON carries the pinned sha

## Description

`f10-amend-settle.md`, F-10's design of record, said that `jigc task amend --format json` *carries the pinned sha under `text`, no new key needed — **verify***. Driven on `1.0.0-rc.20`, the envelope is `{task, text}` and no run of 7 to 40 hex digits appears anywhere in it. The binary was right. The verification had come back negative and was acted on (`c65d3495`), and the authority is a `Disposition::DeclaredOut` census row in `crates/cli/tests/text_json_parity_axis.rs`: the sha is a fact about the repository, which any driver reads from `git log -1 HEAD`. But the settle row was never struck, while two rows of the same table took dated corrections from the same pass. A smaller half rode with it: that census row's subject called the arm *led by the `amending:` block*, which the disposition beneath it declares out of the arm. It is a record row, not a behaviour row, so its kind is `feedback`. It was found independently as `(3, F-D)`.

## Repro

```sh
src=$PWD                                          # the jigc checkout
rig=$("$src"/dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC --format json task amend "json probe" > amend.json; echo "exit $?"   # 0
grep -c '"task"' amend.json                       # 1: the {task, text} arm
grep -Eo '[0-9a-f]{7,40}' amend.json | wc -l      # 0: no sha in the envelope, by declaration
grep -c 'carries the pinned sha under `text`, no new key needed — \*\*verify\*\*.*~~' \
  "$src"/completions/artifacts/M53/f10-amend-settle.md           # 1: the settle row is struck
```

## Resolution

Fixed in M53's last batch (`3e362bf8`), stamped `1.0.0-rc.21`. The settle row is struck, with its driven datum beside it, and the census row's subject names the arm only.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). `jigc --format json task amend "json probe"` exits 0 with the `{task, text}` envelope, and no run of 7 to 40 hex digits appears in it. The settle row reads struck, followed by its dated correction bracket.
