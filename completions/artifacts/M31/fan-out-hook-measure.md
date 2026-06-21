# M31 — the fan-out finalize hook contract (both commit paths run the user's hooks, WIP-safely)

**Run 2026-06-21.** The M31 done-bar mandated by [finalize.md](../../../design/finalize.md)
→ `fan-out` finalize / never-bypass-hooks, [worked-examples.md](../../../design/worked-examples.md)
→ flow 9 + flow 33, [DECISIONS.md](../../../DECISIONS.md) → 2026-06-21 M31 Inc 5 (squash:true
hook restoration, WIP-safe) + 2026-06-20 M31 planning (squash:false honest rework), and
[roadmap.md](../../../implementation/roadmap.md) → M31 Increment 5. M31 makes the milestone
fan-out finalize fire the user's `pre-commit`/`commit-msg` hooks on **both** commit paths while
WIP-safety holds: on **squash:true** the aggregate combine runs the hooks against the combined
tree **without touching the main checkout**; under **squash:false** each per-sub-task commit
carries **that sub-task's worktree-attributed code** and relays its hooks. The hook restoration
+ honest rework shipped Inc 1–T3 (the off-line worktree combine engine, the WIP-safe
dedicated-worktree hook commit, the per-sub-task code commits, the one-hook-story doc fold-back);
this T4 artifact records the **four measured facts**, each **observed by running the
ACTUALLY-PINNED `cargo install`-built binary** (NOT `cargo test`) over real git repos with **no
environment overrides** — the production path a real install hits, with the embedded dev pack and
the sibling `doc-code` probe.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `cb0fb59b14f59e7f7d2465daf94e0f88a2cc0afe9a11863cfc41162dcfbd2f52` |
| `doc-code` probe sha256 | `530dd89f32d7c6a6afbb617b7672744897316d9ecb243a6cd06710ec22302398` |
| HEAD commit | Built at `c675e1a` (M31 Inc 5 T3 — the one-hook-story doc fold-back; T1 restored the squash:true WIP-safe hook commit, T2 the squash:false per-sub-task honest rework). This re-pin commit (T4) adds **only** `completions/artifacts/M31/` (this file + `evidence/`) — no Rust source change — so the pinned binary stays **byte-identical to a fresh `cargo install` at final HEAD**. |
| Built | `cargo install --path crates/cli --force` (release; `build.rs` builds the eight-grammar `doc-code` probe — Rust + TypeScript/TSX + JavaScript/JSX + Python + PHP + bash + CSS + YAML — and embeds it via `OUT_DIR/doc-code`, plus the embedded dev pack); `rustc 1.95.0` |
| Pinned to | `jigc` **and** its `doc-code` probe sibling to **both** `~/.local/bin/` and `~/.cargo/bin/`. `cargo install` placed `jigc` at `~/.cargo/bin/jigc`; it was copied to `~/.local/bin/jigc`; a `jigc setup` run from **each** bin dir confirmed that dir's `doc-code` sibling (the extract is a **no-op** when byte-identical — T1–T3 changed only `milestone.rs`, not the `doc-code` probe's grammar set, so the sibling stays the M29 eight-grammar probe). **All four sha256 identical** — `jigc` identical across both dirs (`cb0fb59b…`), `doc-code` identical across both dirs (`530dd89…`). `which jigc` → `~/.local/bin/jigc`, `which doc-code` → `~/.local/bin/doc-code` (a pinned dir). |
| Fresh-build cross-check | The relocated `jigc` is **byte-identical** to the freshly-`cargo install`-built `target/release/jigc` (same `cb0fb59b…`); the `doc-code` sibling is byte-identical to the `build.rs`-embedded `OUT_DIR/doc-code` and to `target/release/doc-code` (`530dd89…`) — i.e. the pinned binaries match a fresh `cargo install` at this HEAD. |
| Size | release `jigc` **13.80 MB** (13 801 600 B), `doc-code` **8.49 MB** (8 486 360 B). The size guard's **90 MiB** ceiling ([`crates/cli/tests/cargo_install_probe.rs`](../../../crates/cli/tests/cargo_install_probe.rs)) protects the **debug** `CARGO_BIN_EXE_jigc` (**73.02 MiB**, 76 568 336 B); still well under. |
| Invocation | `jigc` from `PATH`, with the **embedded** dev pack (**no** `JIGC_PACK_DIR`) and the `doc-code` probe resolved **as the sibling beside the installed binary** (**no** `JIGC_DOC_CODE_PROBE` — the driver `unset`s both) — the production probe-resolution path (the M20 embed/sibling). |
| Driver | [`evidence/drive.sh`](evidence/drive.sh) — a self-cleaning scratch git repo + isolated `$HOME` per arm; logs in [`evidence/`](evidence/). |

## Honest posture — protocol-observed, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23–M30):
this is an **owner-artifact recording** (the recorded-alongside measure run on the actually-pinned
binary), **not** a TDD red→green test — its done-criterion is this committed artifact plus the
clean end-to-end runs on the installed binary. The squash:true WIP-safe hook commit, the
squash:false per-sub-task honest rework, the cross-worktree same-file block, and the WIP-safety
guarantee are independently **binary-proven** by
[`crates/cli/tests/milestone.rs`](../../../crates/cli/tests/milestone.rs) (the
`milestone_finalize_relays_every_fan_out_commit_hook_output`,
`…_squash_false_lands_n_plus_one_commits_in_id_order`,
`…_squash_false_blocks_a_cross_worktree_code_collision` tests) and
[`crates/cli/tests/flow33_acceptance.rs`](../../../crates/cli/tests/flow33_acceptance.rs)
(`flow33_same_file_collision_blocks_and_wip_survives`) against this same binary's source. This
artifact's job is to show those facts *on the binary a real install actually resolves*, with the
production sibling probe and no overrides.

## Corpus — four scratch git repos (mirrors the M31 acceptance suite)

Each arm stands up a throwaway `git init` repo with one base commit, mints
`milestone:cache-rework` + two sub-tasks (`area-low`, `area-zed`, added NON-id order so id-order
is no accident of insertion), stages each sub-task's docs into its `.jigc/tasks/<sub>/docs/` area
and (where the arm needs code) provisions the base-pin worktrees and stages disjoint code in each
`.jigc/worktrees/<sub>/`. The four arms:

- **squash-true-hooks** — default `squash: true`; two disjoint-code sub-agents; a non-blocking
  `pre-commit` hook installed. Proves the combine runs the hook against the combined tree.
- **squash-false** — `finalize.fan-out.squash: false`; each sub-task stages a disjoint ADR + its
  authored `commit:<sub>` doc + disjoint worktree code; the same non-blocking hook. Proves N
  per-sub-task code commits + the parent aggregate, hook relayed per commit.
- **wf2-block** — `squash: false`; both sub-agents stage the **same** path `src/shared.rs`. Proves
  the cross-worktree collision blocks up front.
- **wip-survives** — default `squash: true`; a same-file collision (`area-low` edits `shared.txt`,
  `area-zed` renames it — the rename old-path collides) **plus** unrelated main-checkout WIP.
  Proves the blocked combine leaves the live checkout byte-identical.

Full logs per arm in [`evidence/`](evidence/).

## The four measured facts — each observed on the installed binary

All four observed by running `~/.local/bin/jigc` (sha256 `cb0fb59b…`) from `PATH`, **no
`JIGC_PACK_DIR`, no `JIGC_DOC_CODE_PROBE`**, the embedded dev pack, resolving the `doc-code`
sibling (`530dd89…`) beside it.

### Fact 1 — squash:true fires the user's hooks on the aggregate ([`evidence/squash-true-hooks.log`](evidence/squash-true-hooks.log))

`jigc milestone finalize cache-rework` (default `squash: true`): `FINALIZE_EXIT=0`, HEAD **delta
+1** (exactly one aggregate commit). The installed binary relays the non-blocking hook's warning
**exactly once**, in the delimited section:

```
--- hook output ---
NON-BLOCKING-MILESTONE-HOOK-WARNING
```

The Inc-4 `git commit-tree` form ran **zero** hooks; the Inc-5 WIP-safe dedicated-worktree commit
(`git commit -F` from a clean worktree at the combined tree, then fast-forward main) runs the
user's `pre-commit`/`commit-msg` hooks against the combined tree. The single commit carries
**both** sub-agents' staged code **and** the merged docs (drops none):

```
.jigc/.gitignore
docs/decisions/low-policy.md
docs/decisions/zed-policy.md
src/low.rs
src/zed.rs
```

`git status --porcelain` post-finalize is **empty** — the combine + worktree teardown leave the
main checkout clean (no ` D` drift).

### Fact 2 — squash:false lays N per-sub-task code commits, each relaying its hook ([`evidence/squash-false.log`](evidence/squash-false.log))

`jigc milestone finalize cache-rework` with `finalize.fan-out.squash: false`: `FINALIZE_EXIT=0`,
HEAD **delta +3** = **N+1** (2 per-sub-task code commits + 1 parent aggregate). The non-blocking
hook is relayed **3 times** (one per fan-out commit) — every fan-out commit now runs the user's
hooks. Commit subjects, oldest-first (the id-sorted sub-task sequence then the aggregate):

```
feat: rework the low cache path
feat: rework the zed cache path
Finalize milestone cache-rework (2 sub-tasks)
```

Each per-sub-task commit carries **that** sub-task's worktree code — a real tree, **not** the
retired `--allow-empty` tree-empty form (`git show --stat`):

```
[HEAD~2] feat: rework the low cache path      src/low.rs | 1 +
[HEAD~1] feat: rework the zed cache path      src/zed.rs | 1 +
```

The **merged docs land in the parent aggregate** (review B1/B2 — the by-task-id doc-join is a
cross-sub-task merge artifact, not partitionable per-sub-task) — `git show --name-only HEAD`:

```
.jigc/.gitignore
.jigc/config/manifest.yaml
docs/decisions/low-policy.md
docs/decisions/zed-policy.md
```

`git status --porcelain` post-finalize is **empty** — the boundary leaves a clean tree.

### Fact 3 — the WF2 same-file block fires across the fan-out ([`evidence/wf2-block.log`](evidence/wf2-block.log))

Both sub-agents staged `src/shared.rs` in their isolated worktrees.
`jigc milestone finalize cache-rework` (`squash: false`): `FINALIZE_EXIT=1`. The block fires
**up front** (before any per-sub-task commit), naming the contended path and a route:

```
code collision — `src/shared.rs` (sub-tasks [area-low, area-zed]) staged by more than one
worktree; the combine disjoint-applies code and never text-merges a shared file
  route: have the contending sub-tasks touch distinct files, or combine their overlapping changes by hand
```

HEAD is **unchanged** (commit count `1 == 1` before/after — nothing committed) and **no ADR doc
file** is promoted.

### Fact 4 — unrelated main-checkout WIP survives a blocked combine ([`evidence/wip-survives.log`](evidence/wip-survives.log))

With default `squash: true`, a same-file collision (`area-low` edits `shared.txt`; `area-zed`
**renames** it — the rename old-path collides), plus unrelated main-checkout WIP seeded before the
finalize: an untracked `wip-untracked.txt` and an unstaged-modified tracked `README.md`.
`jigc milestone finalize cache-rework`: `FINALIZE_EXIT=1`, HEAD **unchanged**. The unrelated WIP
survives **byte-identically** — the off-line temp-index combine never touches the live checkout
(review S2):

```
wip-untracked.txt = [scratch]                (intact)
README.md         = [hello|local WIP]        (intact)
git status --porcelain before == after       byte-identical: yes
```

No ADR doc file is promoted (the rolled-back promotions leave no doc).

## Bounds (carried in honestly)

- **n = 4 arms** over hand-built fixtures — a done-bar shape check, not a distribution. The hook
  restoration, the honest rework, the cross-worktree block, and WIP-safety are binary-proven by
  `milestone.rs` + `flow33_acceptance.rs` against this same binary's source; this artifact is the
  *observed-on-the-installed-binary* shadow.
- **Owner-artifact recording**, not a CLI-emitted metric — the recorded-alongside posture. Only
  the finalize exit / landed commit count / committed trees / promotion is the binary's own
  yes/no; the hook-relay count + WIP-survival reading is the owner's inspection of the emitted
  output and the working tree.
- **A blocked combine leaves an empty `docs/decisions/` directory** in the main checkout (the
  rollback removes the promoted doc *files* but does not prune the now-empty parent dir). git
  does not track empty directories, so it never reaches the byte-identical `git status` above and
  affects no WIP file — "nothing promoted" is measured at the doc-**file** level (the flow33
  idiom), and WIP-safety holds for every WIP file. Harmless cosmetic leftover, accepted by the
  committed `flow33_same_file_collision_blocks_and_wip_survives` test.
- **The scratch repos + `$HOME`s under `/tmp` are detached and discarded**; no developer repo was
  mutated. jigc's own source was not modified.

## Verdict — done-bar met

The milestone fan-out finalize fires the user's `pre-commit`/`commit-msg` hooks on **both** commit
paths while WIP-safety holds — **measured on the actually-pinned, `cargo install`-built binary**
(`jigc` `cb0fb59b…`, `doc-code` `530dd89…`, both identical across `~/.local/bin` + `~/.cargo/bin`
and matching a fresh build at HEAD), resolving the probe through the **production sibling path**
with **no env overrides**. (1) A **squash:true** combine fires the user's hook on the single
aggregate (relayed exactly once) and carries both sub-agents' code + the merged docs; (2) a
**squash:false** fan-out lays **N+1** commits — each per-sub-task commit carrying **its own
worktree's code** (a real tree, not an empty one) and relaying its hook, with the merged docs in
the parent aggregate; (3) a cross-worktree same-file collision **blocks** up front (exit 1, HEAD
unchanged, nothing promoted), naming the contended path; and (4) unrelated main-checkout WIP
**survives a blocked combine byte-identically**. **One hook story, both paths, WIP-safe. The M31
done-bar is met.**
