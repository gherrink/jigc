# M31 — the fan-out finalize hook contract (both commit paths run the user's hooks, WIP-safely)

**Run 2026-06-21 (re-pinned at HEAD `eb3c706`).** The M31 done-bar mandated by [finalize.md](../../../design/finalize.md)
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
the **WIP-safe combine landing** (`git merge --ff-only`, never `git reset --hard`) shipped at
`eb3c706` — the commit that prompted this re-pin, because the earlier T4 pin (`cb0fb59b…`,
built at `b91b3b2`) predated it and still carried the destructive-reset WIP-loss bug.
This artifact records the **five measured facts**, each **observed by running the
ACTUALLY-PINNED `cargo install`-built binary** (NOT `cargo test`) over real git repos with **no
environment overrides** — the production path a real install hits, with the embedded dev pack and
the sibling `doc-code` probe.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `c9e7eb72100df899af1b48cc5030e9d2337f0af7b348d1a30ca16d88b0263ec8` |
| `doc-code` probe sha256 | `530dd89f32d7c6a6afbb617b7672744897316d9ecb243a6cd06710ec22302398` |
| HEAD commit | Built at `eb3c706` (`fix(finalize): land squash:true combine via ff-only merge, not destructive reset`) — the WIP-safe-landing fix. **The prior pin (`cb0fb59b…`, built at `b91b3b2`) was STALE**: `eb3c706` changed `crates/cli/src/task.rs` (the combine land step) *after* that pin, so the installed binary still wiped unrelated unstaged WIP on a successful `squash:true` combine. This pin rebuilds at `eb3c706` and re-copies into both bin dirs. The binary is **not** byte-reproducible across builds (release binaries are not stripped — embedded build paths / debug-info differ build-to-build), so the sha recorded here is the exact output of the `cargo install` that produced the pinned bytes, identical across `target/release`, `~/.cargo/bin`, and `~/.local/bin`. `eb3c706` touches no grammar, so `doc-code` stays the M29 eight-grammar probe (`530dd89…`). |
| Built | `cargo install --path crates/cli --force` (release; `build.rs` builds the eight-grammar `doc-code` probe — Rust + TypeScript/TSX + JavaScript/JSX + Python + PHP + bash + CSS + YAML — and embeds it via `OUT_DIR/doc-code`, plus the embedded dev pack); `rustc 1.95.0` |
| Pinned to | `jigc` **and** its `doc-code` probe sibling to **both** `~/.local/bin/` and `~/.cargo/bin/`. `cargo install --path crates/cli --force` placed `jigc` at `~/.cargo/bin/jigc`; it was copied to `~/.local/bin/jigc`; the `doc-code` sibling (unchanged by `eb3c706`, still the M29 eight-grammar probe) is the same `530dd89…` in both dirs. **All four sha256 identical** — `jigc` identical across both dirs (`c9e7eb72…`), `doc-code` identical across both dirs (`530dd89…`). `which jigc` → `~/.local/bin/jigc`, `which doc-code` → `~/.local/bin/doc-code` (a pinned dir). |
| Fresh-build cross-check | The pinned `jigc` IS the output of this `cargo install` at `eb3c706` — byte-identical to the `target/release/jigc` it co-produced (same `c9e7eb72…`); the `doc-code` sibling is byte-identical to the `build.rs`-embedded `OUT_DIR/doc-code` and to `target/release/doc-code` (`530dd89…`). (Release `jigc` is not stripped, so a *separate* later rebuild yields a different sha — the pin is the bytes this install wrote, not a reproducibility claim.) |
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

## Corpus — five scratch git repos (mirrors the M31 acceptance suite)

Each arm stands up a throwaway `git init` repo with one base commit, mints
`milestone:cache-rework` + two sub-tasks (`area-low`, `area-zed`, added NON-id order so id-order
is no accident of insertion), stages each sub-task's docs into its `.jigc/tasks/<sub>/docs/` area
and (where the arm needs code) provisions the base-pin worktrees and stages disjoint code in each
`.jigc/worktrees/<sub>/`. The five arms:

- **squash-true-hooks** — default `squash: true`; two disjoint-code sub-agents; a non-blocking
  `pre-commit` hook installed. Proves the combine runs the hook against the combined tree.
- **squash-false** — `finalize.fan-out.squash: false`; each sub-task stages a disjoint ADR + its
  authored `commit:<sub>` doc + disjoint worktree code; the same non-blocking hook. Proves N
  per-sub-task code commits + the parent aggregate, hook relayed per commit.
- **wf2-block** — `squash: false`; both sub-agents stage the **same** path `src/shared.rs`. Proves
  the cross-worktree collision blocks up front.
- **wip-survives** — default `squash: true`; a same-file collision (`area-low` edits `shared.txt`,
  `area-zed` renames it — the rename old-path collides) **plus** unrelated main-checkout WIP.
  Proves the **blocked** combine leaves the live checkout byte-identical.
- **wip-survives-success** — default `squash: true`; two **disjoint**-code sub-agents (the combine
  succeeds) **plus** unrelated unstaged tracked WIP in the main checkout. Proves a **successful**
  combine leaves the live checkout byte-identical — the regression this re-pin closes (the success
  path is exactly where the stale binary's `git reset --hard` wiped WIP). Mirrors
  `flow33_acceptance.rs::flow33_unrelated_wip_survives_successful_combine`.

Full logs per arm in [`evidence/`](evidence/).

## The five measured facts — each observed on the installed binary

All five observed by running `~/.local/bin/jigc` (sha256 `c9e7eb72…`) from `PATH`, **no
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

### Fact 5 — unrelated main-checkout WIP survives a SUCCESSFUL combine ([`evidence/wip-survives-success.log`](evidence/wip-survives-success.log))

The success-path twin of Fact 4 — and the fact the stale pin **failed**. With default
`squash: true` and two **disjoint**-code sub-agents (so the combine succeeds), unrelated unstaged
tracked WIP is seeded in the main checkout before the finalize:
`README.md = hello\nUNRELATED WIP THE USER IS EDITING\n`. `jigc milestone finalize cache-rework`:
`FINALIZE_EXIT=0`, HEAD **delta +1** (one aggregate). The unrelated WIP survives
**byte-identically** — the `eb3c706` fix lands the combine via `git merge --ff-only`, which carries
the unstaged WIP across the fast-forward instead of the prior `git reset --hard` that discarded it:

```
README.md = [hello|UNRELATED WIP THE USER IS EDITING]   byte-identical before == after: yes
```

The WIP stays **out** of the aggregate commit — `git show --name-only HEAD` is exactly the two
sub-agents' code + the merged docs (`src/low.rs`, `src/zed.rs`, `docs/decisions/{low,zed}-policy.md`,
`.jigc/.gitignore`), no `README.md`. (Re-run against the **stale** `cb0fb59b…` pin, this same arm
gave `FINALIZE_EXIT=0` but `README.md` wiped back to `hello\n` — the data loss `eb3c706` fixes.)

## Bounds (carried in honestly)

- **n = 5 arms** over hand-built fixtures — a done-bar shape check, not a distribution. The hook
  restoration, the honest rework, the cross-worktree block, and WIP-safety on **both** the blocked
  and the successful path are binary-proven by `milestone.rs` + `flow33_acceptance.rs`
  (incl. `flow33_unrelated_wip_survives_successful_combine`) against this same binary's source; this
  artifact is the *observed-on-the-installed-binary* shadow.
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
paths while WIP-safety holds — **measured on the actually-pinned, `cargo install`-built binary at
`eb3c706`** (`jigc` `c9e7eb72…`, `doc-code` `530dd89…`, both identical across `~/.local/bin` +
`~/.cargo/bin` and the `target/release` the install co-produced), resolving the probe through the
**production sibling path** with **no env overrides**. (1) A **squash:true** combine fires the
user's hook on the single aggregate (relayed exactly once) and carries both sub-agents' code + the
merged docs; (2) a **squash:false** fan-out lays **N+1** commits — each per-sub-task commit carrying
**its own worktree's code** (a real tree, not an empty one) and relaying its hook, with the merged
docs in the parent aggregate; (3) a cross-worktree same-file collision **blocks** up front (exit 1,
HEAD unchanged, nothing promoted), naming the contended path; (4) unrelated main-checkout WIP
**survives a blocked combine byte-identically**; and (5) unrelated main-checkout WIP **survives a
SUCCESSFUL combine byte-identically** (the regression the stale `cb0fb59b…` pin failed — the
`git reset --hard` data loss `eb3c706` replaces with an ff-only merge). **One hook story, both
paths, WIP-safe on the actually-pinned binary. The M31 done-bar is met.**

## Completion addendum — 2026-06-21 (post-audit fix + re-pin at `bdb1523`)

The milestone-completion audit (e2e 13/13 on the installed binary + code-review) found one HIGH the
above run missed: the **squash:false *abort* path** still ran `git reset --hard` on the live main
checkout (`commit_subtask_code` applied each sub-task's patch directly onto main, and any abort —
a per-sub-task hook rejection or the aggregate rejection — reset it), **destroying unrelated
unstaged tracked WIP** — the symmetric twin of the bug `eb3c706` fixed for squash:true. Fixed at
**`bdb1523`** (`fix(finalize): squash:false fan-out abort is WIP-safe — never reset --hard the main
checkout`): squash:false now builds its N+1 commit chain in a dedicated worktree and lands via
`git merge --ff-only`, so an abort tears down the worktree and **never touches main**.

**Re-pinned at `bdb1523`** (replaces the `eb3c706`/`c9e7eb72…` pin above):

| | |
|---|---|
| `jigc` sha256 | `8d33bb4ffb0c35c2669dda04c1622f05453ea0c909e3165f416a1454d1d492f4` (identical across `~/.local/bin` + `~/.cargo/bin`) |
| `doc-code` probe sha256 | `530dd89f32d7c6a6afbb617b7672744897316d9ecb243a6cd06710ec22302398` (unchanged — `bdb1523` touches no grammar) |
| HEAD | `bdb1523` · built `cargo install --path crates/cli --force`, copied to both bin dirs |

**New measured fact (6) — squash:false ABORT is WIP-safe** (verified on the re-pinned installed
binary, no env overrides, via the milestone-completion re-verify): with `finalize.fan-out.squash`
false, a seeded unrelated **unstaged tracked** edit in the main checkout (`important.txt` =
`PRECIOUS UNSAVED HUMAN WORK`), and an aggregate-rejecting `pre-commit` hook → `jigc milestone
finalize` exits 1, **HEAD is unchanged** (no orphan per-sub-task commits), the **human edit
survives byte-for-byte** (would be reverted to its committed content if `reset --hard` had run),
and the worktrees are torn down. The success path re-confirmed: N+1 commits land, unrelated WIP
preserved, and the committed sequence is **byte-identical across divergent sub-task add-orders**
(by-task-id, not feed order). **The M30/M31 data-loss class is now retired on BOTH commit paths.**
