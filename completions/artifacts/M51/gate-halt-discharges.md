# M51 — the gate-record's HALT drives, discharged

**Date:** 2026-09-11 · **Binary:** `1.0.0-rc.14`, the repo's `target/release/jigc` built at
`bd348a83` (`jigc --version` → `jigc 1.0.0-rc.14`) · **Repo HEAD:** `bd348a83`, working tree
carrying only the uncommitted planning files.

This file discharges the drives the [planning gate-record](planning-gate-record.md) → *HALT cells*
names as owed before decompose. **Nothing here is a plan and nothing here is relayed** — every
section names the script that produced it, quotes the output that matters, and states a verdict
that the output can falsify. Where a drive left a cell un-reached, the section says so in its own
words rather than generalizing over it.

**Provenance.** Every script, its full captured output, and the intermediate probes live in the
session scratchpad at
`…/scratchpad/halt-drives/` (`drive2b`/`2c`/`2d`/`2e-ec30`, `drive3-d1`, `drive3b-d1`,
`drive4-d3`, `drive4b-d3`, `drive5-d4`, `drive5b-d4`, `drive5c-d4`, `cite-sweep2`, and the six
`b*`/`m*` cargo logs). Rigs were built with
`dev/jigc-rig <state> --binary /Users/maurice/projects/gherrink-jigc/target/release/jigc`, evaluated
in two steps, never `eval "$(…)"`. Cargo ran under a private `CARGO_TARGET_DIR`; tests ran **bare**
with `$?` captured directly, never through a pipe.

**One honesty note against my own instrument.** In §3's door-B control I wrote
`… 2>&1 | head -3; echo "RC=$?"`, which reports *head's* status. That control's `RC=0` is **void**
and is not used for anything below. It is recorded rather than deleted because it is the exact trap
`SHELL.md` names, committed by the agent running the drives.

---

## §1 · EC-7 — the applied mutation (claim-driven)

### What was asked

Delete one deny pattern (`Read(./**/*.key)`) from `crates/cli/adapters/claude-code.yaml`; run the
three tests whose job touches the deny floor; record which redden and which stay green; restore the
file byte-exact. Verdict: **is EC-7's residual — *accidental drop is caught; deliberate regeneration
is not* — correct as amended?**

### Script

```
# baseline
CARGO_TARGET_DIR=<private> cargo test -p cli --lib claude_code_profile_bytes_are_canonical
CARGO_TARGET_DIR=<private> cargo test -p cli --lib deny_floor_added_then_idempotent
CARGO_TARGET_DIR=<private> cargo test -p cli --test g_flow \
    e2e_audit::scenario_1b_deny_floor_merges_never_clobbers
# mutation
grep -v '^    - "Read(\./\*\*/\*\.key)"$' crates/cli/adapters/claude-code.yaml > mutated.yaml
cp mutated.yaml crates/cli/adapters/claude-code.yaml    # 49 lines → 48; the `Bash(cat …*.key:*)` twin survives
# same three runs
# restore
cp claude-code.yaml.orig crates/cli/adapters/claude-code.yaml
```

### Output

Baseline, all three green:

```
test adapter::tests::claude_code_profile_bytes_are_canonical ... ok
test adapter::tests::deny_floor_added_then_idempotent ... ok
test e2e_audit::scenario_1b_deny_floor_merges_never_clobbers ... ok
```

Under the mutation:

| test | home | exit | verdict |
|---|---|---|---|
| `claude_code_profile_bytes_are_canonical` | `crates/cli/src/adapter.rs:1495` | **101** | **REDDENS** |
| `deny_floor_added_then_idempotent` | `crates/cli/src/adapter.rs:2623` | **101** | **REDDENS** |
| `e2e_audit::scenario_1b_deny_floor_merges_never_clobbers` | `crates/cli/tests/e2e_audit.rs:290` | **0** | **STAYS GREEN** |

The two failures are inline `insta` snapshot diffs and they name the deleted line:

```
   21    21 │     - "Bash(cat ./.env.*:*)"
   22       │-    - "Read(./**/*.key)"                 (profile bytes golden)
   23    22 │     - "Bash(cat ./**/*.key:*)"
...
   14       │-      "Read(./**/*.key)",                (produced settings.json golden)
   15    14 │       "Bash(cat ./**/*.key:*)",
```

### Restore, verified

```
$ shasum -a256 crates/cli/adapters/claude-code.yaml
e61dfddac4aca2adf5db99ad18cd8e5699305b210630312f75ae7f6d4e4b4018   (identical to the pre-mutation sha)
$ git diff --stat -- crates/cli/adapters/claude-code.yaml
(empty)
$ git status --short
 M DECISIONS.md
 M implementation/decisions-pending.md
?? completions/artifacts/M51/
?? completions/artifacts/evidence-check-1.0/      (the pre-drive state, unchanged)
```

The stale `crates/cli/src/.adapter.rs.pending-snap` (gitignored) was copied aside before the run and
copied back after it.

### Verdict — **EC-7's residual is CORRECT as amended**

Two independent inline goldens catch an **accidental** drop, so that half of the residual is proven,
not asserted. The other half is proven the harder way: the one test whose *stated job* is the deny
floor is **structurally incapable** of noticing, because its subject is scraped from the very file
that was mutated —

```rust
// crates/cli/tests/e2e_audit.rs:262
fn floor_patterns() -> Vec<String> {
    let yaml = include_str!("../adapters/claude-code.yaml");
    …                       // parses the `deny:` block out of the shipped profile
}
```

— and the assertion is `for p in floor_patterns() { assert!(settings.contains(…)) }`. Delete a
pattern and the assertion set shrinks with it. The same shape sits **inside** the reddening
`deny_floor_added_then_idempotent`: its `for p in &profile.allowlist.deny` loop is derived from the
loaded profile and is equally tautological; only the terminal `assert_snapshot!` over the produced
`settings.json` is a real fence. So of the three assertions across the two deny-floor-purposed
tests, **two are derived and one is a literal** — and a deliberate regeneration (`cargo insta
accept`) moves the two literals with the mutation and leaves nothing standing.

**Two riders the drive establishes beyond the residual's wording:**

1. The M50 entries `Bash(jigc uninstall:*)` and `Bash(jigc milestone discard:*)` — the human-owned
   destroying doors — are asserted by **the two inline goldens only**. The e2e test that exists to
   check the floor cannot see them, which is EC-27/D1a's scraper repair in one sentence.
2. The fix shape the Settle already picked (G-43 — *extend the test asserting literals against the
   **loaded** profile, never mint a production const*) survives this drive unchanged: the problem is
   not that a literal is missing, it is that the literal and the source are the same file.

---

## §2 · EC-30's milestone arm (claim-driven)

### What was asked

Drive the milestone arm: `milestone create` + `add-task` + `provision`; author in the sub-task area
and stage a code file in its worktree; in the MAIN checkout rewrite history so the milestone's
recorded `base` sha is unreachable; then `milestone join` and `milestone finalize`. **Does the store
lie (a success over an unreachable base), refuse honestly, or not care?** Confirmed or refuted?

### Script

`drive2d-ec30.sh` (the final form; `drive2b`/`2c` are its two corrected predecessors, kept because
each corrected a real mistake of mine — see *Bounds* below). `drive2e-controls.sh` carries the two
controls.

```
rig = dev/jigc-rig committed-singletons --binary <release>
jigc milestone create "EC30 probe"          # base pinned at HEAD, then a record commit lands
jigc milestone add-task ec30-probe "widen the cache"   # a second record commit lands
BASE=$(sed -n 's/.*"sha": "\([0-9a-f]*\)".*/\1/p' .jigc/milestones/ec30-probe/base.json)
jigc milestone provision ec30-probe
( cd .jigc/worktrees/widen-the-cache && jigc start --task widen-the-cache )
jigc doc create adr --title "Widen the cache" --task widen-the-cache
  + set-slot context / decision / consequences          # a fully-authored staged doc
( cd .jigc/worktrees/widen-the-cache && echo … > cache.ts && git add cache.ts )
git reset --soft "${BASE}~1" && git commit -q -m rewritten   # squashes the base away
jigc milestone join ec30-probe ; jigc milestone finalize ec30-probe
```

Note the correction to the literal instruction: `git reset --soft HEAD~1` is **not enough** here,
because `milestone create` and `add-task` each land a record commit *above* the base pin, so `HEAD~1`
rewrites a record commit and leaves the base reachable. `${BASE}~1` is what actually orphans it, and
the drive uses that.

### Output

Reachability after the rewrite:

```
commit                                   # git cat-file -t $BASE  → the object is still alive (reflog)
cat-file rc=0
is-ancestor-of-HEAD rc=1                 # git merge-base --is-ancestor $BASE HEAD → NOT reachable
```

`milestone join`:

```
joined milestone:ec30-probe — 1 doc(s) merged
  - adr:widen-the-cache  (created · from widen-the-cache)
JOIN_RC=0
```

`milestone finalize`:

```
blocking · finalize.base-mismatch — the milestone was pinned to base `84c0324…` but HEAD is now
  `d3f5d2f…`, and the commits landed since move more than milestone-record bookkeeping — the
  sub-task worktrees were cut from `84c0324…`, so combining them onto HEAD cannot be proven sound
  at: milestone:ec30-probe
  route: land this milestone's work first (out-of-band git: return HEAD to `84c0324…`, run
  `jigc milestone finalize`, then re-land the newer commits on top), or re-cut this milestone's
  work onto the new base (out-of-band git: re-provision the sub-task worktrees from HEAD and
  re-apply each sub-task's staged changes)
FINALIZE_RC=3
```

Nothing was committed (`git log` unchanged at `d3f5d2f rewritten`), the working tree is clean, and
the committed record still reads `status: active` with its original `base:` line.

### The two controls (`drive2e-controls.sh`)

| cell | base reachable? | delta `base..HEAD` | result |
|---|---|---|---|
| **A** | **yes** | one ordinary code commit | `finalize.base-mismatch`, **exit 3** — *the identical finding* |
| **B** | **yes** | the two milestone-record commits only | **exit 0**, `finalized c453565 — Finalize milestone ctl-probe (1 sub-task)`, `added cache.ts`, `promoted docs/decisions/widen-the-cache.md`, `modified docs/milestone-records/ctl-probe.md`, `3 files committed` |

### Verdict — **EC-30's milestone arm is REFUTED**

The store neither lies nor is indifferent: `jigc milestone finalize` **refuses honestly**, exit 3,
with a located finding and two runnable recovery routes, and commits nothing. Per the Settle's own
rule (*"if it refutes, the row is recorded refuted with its datum and nothing is built"*), the datum
is the `finalize.base-mismatch` block above.

**What the refutation is actually keyed on, stated precisely so the row is not re-opened on a wrong
premise.** The guard is **not** a reachability probe. `crates/cli/src/milestone.rs:989`
(`record_only_range`) asks two questions — *is `base` a linear ancestor of `HEAD`* and *does every
commit in `base..HEAD` touch only milestone-record paths* — and the unreachable-base case simply
fails the first. Control A proves this: a perfectly **reachable** base with one ordinary commit on
top yields the same code at the same exit. There **is** a separate reachability probe —
`base_commit_missing` / `guard_base_live` (`milestone.rs:1065`, `:1086`), minting
`engine::milestone::stale_base_finding` — but it fires only when the base **object is gone**, and it
did **not** fire here: a `git reset --soft` leaves the object in the reflog, and a provisioned
worktree's detached `HEAD` pins it besides. That path is therefore **not driven** (see *Bounds*).

### Bounds of §2

- **`milestone join` exits 0 over a divergent base** and prints `joined … — 1 doc(s) merged` with no
  mention of the base. It commits nothing by design, so no lie is landed — but the join's success
  line is the one surface in this sequence that says nothing true about the state. This is a
  surface-tier observation, **not** EC-30, and it is recorded here rather than promoted.
- **`guard_base_live`'s stale-base finding was not reached.** Reaching it needs the base *object*
  destroyed (reflog expiry + `gc --prune`, with the pinning worktree removed first), which this
  drive did not do.
- **A sub-task under a milestone stages no `commit` doc.** `drive2c` established this by driving it:
  after composing `jigc start --task widen-the-cache` from inside the provisioned worktree,
  `jigc doc list --task` reports *no docs staged*, and every `doc set-field commit:…` answers
  *"task `widen-the-cache`'s workflow provisions its `commit` doc at compose and grants no in-task
  create for it"*. The milestone boundary synthesizes its own message. The brief's *"author a commit
  doc"* is therefore not a thing that can be done at that door; a fully-authored `adr` was used as
  the staged subject instead, which is what makes the join's *1 doc(s) merged* real.

---

## §3 · D1 row 1 — the amended composition, exercised at every *shipped* caller (reuse-exercised)

### What was asked

The amended rule (*three predicates plus a new resolve-or-refuse step*) cannot be driven at the
`migrate` door without code. So: drive the three escape shapes — **absolute**, **`../`**,
**`.git/x`** — through `jigc config set docs-root`, through `jigc rename`'s
`write.untrackable-destination` door, and through `jigc setup`'s `core.hooksPath` path, and show
**which predicate answers which shape at each shipped caller**.

### Scripts

`drive3-d1.sh` (doors A, B, D) and `drive3b-d1.sh` (doors C, D-remaining).

### Door A — `jigc config set <root-knob> <shape>`

| shape | code | leg that answered |
|---|---|---|
| `/…/d1-outside.t1SDn7` (absolute) | **`config.unusable-root`** | the M50 **value** rule, *not* `untrackable_reason` |
| `../escape` | **`config.untrackable-root`** | `untrackable_reason` leg 1 (*resolves outside the repository root*) |
| `docs/../../escape` | **`config.untrackable-root`** | folded to `../escape` first, then leg 1 |
| `.git/x` | **`config.untrackable-root`** | `untrackable_reason` leg 2 (*`.git` component*) |
| `.git` | **`config.untrackable-root`** | leg 2 |

All five exit **1**, nothing is written (`jigc config get docs-root` still reads
`docs/  (pack-default)` after each). The same five were re-driven at `placement-root` with identical
codes.

**The absolute row is the whole of D1's falsification, and the shipped message states it in its own
words:**

```
blocking · config.unusable-root — `/…/d1-outside.t1SDn7` cannot be the `docs-root`: an absolute path
  — every door resolves a root against the repository root, so the docs would move to
  `var/folders/…/d1-outside.t1SDn7` while `jigc config get` reads back `/…/d1-outside.t1SDn7`
```

That rendered destination — the leading `/` gone, the rest re-rooted **inside** the repo — is
exactly what `crate::trackable::untrackable_reason` does to it
(`crates/cli/src/trackable.rs:56`: `let relative = relative.trim_matches('/')`), which is why the
predicate answers **trackable** for an absolute path and a *second, separate* rule has to refuse it.

### Door B — `jigc rename` (`write.untrackable-destination`)

All three shapes, delivered through the only caller-supplied token that reaches the destination
(`--slug`), are refused **upstream**:

```
blocking · write.malformed-slug — `--slug "/…/d1-outside.t1SDn7"` is not a valid slug …   RC=1
blocking · write.malformed-slug — `--slug "../escape"`            … RC=1
blocking · write.malformed-slug — `--slug ".git/x"`               … RC=1
```

So at this door the answer is: **`write.untrackable-destination` is not reachable by any of the
three shapes through a caller token.** M50's slug grammar is the gate; the
`untrackable_reason` call at `rename.rs:532` is a *computed-destination* backstop behind it, which
is a different (and correct) job. The control on this door is **void** — I read its exit code
through a pipe (see the honesty note at the top) — so nothing is claimed from it.

### Door C — `jigc setup` under `core.hooksPath`

Four `bare` rigs, one per shape; each sets `git config core.hooksPath <shape>` before `jigc setup`:

| `core.hooksPath` | setup exit | hook in the install commit? | narration |
|---|---|---|---|
| `/…/d1-hooks.A2qd41` (absolute, outside) | 0 | **no** | *"local to this checkout — git cannot track this path"* |
| `../outhooks` | 0 | **no** | same |
| `.git/myhooks` | 0 | **no** | same |
| `myhooks` (in-repo, benign) | 0 | **yes** — `myhooks/pre-commit` is listed in the commit | the *"cannot track"* line is correctly **absent** |

Every escape shape is answered correctly here, **including the absolute one** — and the reason is
the load-bearing datum for D1's amendment. `committable_hook_path`
(`crates/cli/src/setup.rs:1495`) does not hand the raw token to the predicate:

```rust
let root = std::fs::canonicalize(repo_root).ok()?;
let hook = std::fs::canonicalize(hook_file).ok()?;
let relative = hook.strip_prefix(&root).ok()?.to_str()?.to_string();
crate::trackable::untrackable_reason(repo_root, &relative).is_none().then_some(relative)
```

The `strip_prefix(&root)` **is** a resolve-or-refuse step: an absolute path outside the root fails it
and returns `None` *before* `untrackable_reason` is asked anything.

### Door D — `jigc migrate <path>` (the un-guarded door, driven at this record's date)

| shape | exit | what landed |
|---|---|---|
| `/…/d1-out.2CgRVE/FOREIGN.md` (absolute, outside the repo) | **0** | task minted; the file's bytes (`# Old / - something`) rendered into the composed step text |
| `../SIBLING.md` (a sibling directory) | **0** | task minted; `# Sib` rendered into the step text |
| `.git/config` | **0** | task minted; git's own `[core] repositoryformatversion = 0 …` rendered into the step text |

`jigc task list` afterwards: **3 active task(s)**, one per shape.

### Verdict — the amended claim holds, and the shape of the fix is now driven rather than argued

*"Shipped predicates, **no new capability**"* stays **struck**. What the three doors show is a
consistent rule the wave can build to:

> `untrackable_reason` answers **`../` and `.git/`** and nothing else. An **absolute** token needs a
> separate, caller-side **resolve-or-refuse** step in front of it. Every shipped caller that is safe
> has one — `config set` has `config.unusable-root`, `setup` has `canonicalize + strip_prefix`. The
> caller that is not safe, `jigc migrate`, has **neither**, and accepts all three shapes at exit 0.

**Stated explicitly, as the gate cell requires:** *the amended composition at the `migrate` door —
the resolve-or-refuse step plus the three predicates — is **the increment's red test**.* It cannot be
driven here because the composition does not exist in the binary; §3 exercises each of its three legs
at the shipped caller that already carries it, and drives the `migrate` door to the exit-0 acceptance
the red test must invert. The rider stands unchanged: `is_workbench_root` is a private `fn`
(`config.rs:622`) and needs a visibility change.

---

## §4 · D3 row 5 — the amended predicate, driven as a git query (reuse-exercised)

### What was asked

The withdrawn `StagedSnapshot`/`decide_carryover` pair is an *index* probe. Drive the **amended**
predicate — worktree-vs-HEAD per path, asked before any write — over `jigc setup`'s **actual**
pathspec, in three states plus the unborn case.

### `jigc setup`'s actual pathspec

Read off `install_tracked_paths` (`crates/cli/src/setup.rs:1413-1467`), not paraphrased:

```
CLAUDE.md  .claude/settings.json  .jigc/AGENT.md  .jigc/.gitignore  .jigc/version
.jigc/config/.gitkeep  .jigc/config/packs.yaml
[+ .gitignore iff setup seeded it]  [+ .claude/skills/jigc/SKILL.md iff the profile declares a guide]
[+ the pre-commit hook iff committable_hook_path says so]
```

### Scripts

`drive4-d3.sh` (states a/b/c + unborn) and `drive4b-d3.sh` (the never-staged cell and the unborn
substitute). The probe is, per state:

```
git diff --quiet HEAD    -- <pathspec>   # the AMENDED predicate
git diff --cached --quiet -- <pathspec>  # the WITHDRAWN pair's question, for contrast
```

### Output

| state | worktree-vs-HEAD | index-vs-HEAD | `git status --short` |
|---|---|---|---|
| origin, after the first `setup` | **0 clean** | 0 clean | — |
| **(a)** fresh clone of that repo, before any setup | **0 clean** | 0 clean | — |
| **(b1)** the clone, immediately **before** a `setup` re-run | **0 clean** | 0 clean | — |
| **(b2)** the clone, immediately **after** that re-run | **0 clean** | 0 clean | — (`git diff HEAD -- <pathspec>` is empty) |
| **(b3)** after a **third** `setup` | **0 clean** | 0 clean | — |
| **(c1)** an **unstaged** edit to `CLAUDE.md` | **1 DIRTY** | **0 clean** | ` M CLAUDE.md` |
| **(c2)** the same edit **staged**, uncommitted | **1 DIRTY** | **1 dirty** | `M  CLAUDE.md` |

The unborn case (`git init`, no HEAD):

```
git diff --quiet HEAD   -- CLAUDE.md   → fatal: bad revision 'HEAD'        rc=128
git diff --cached --quiet -- CLAUDE.md  → (silent, diffs the empty tree)   rc=0
git rev-parse --verify -q HEAD                                            rc=1
git diff --quiet 4b825dc…(empty tree) -- CLAUDE.md, file UNTRACKED         rc=0
git diff --quiet 4b825dc…(empty tree) -- CLAUDE.md, after `git add`        rc=1
```

### The harm, re-driven at this record's date (`drive4b-d3.sh`)

On the (c1) cell — an edit **never staged at all**, the sharper of the two:

```
 M CLAUDE.md                       (before)
jigc setup …  install commit → dd5d458
SETUP_RC=0
git show HEAD:CLAUDE.md | grep -c "UNSTAGED WIP SECRET"   →  1
git status --short                                        →  (empty)
```

The user's uncommitted line is now inside a commit whose message is
`chore(jigc): install jigc workspace config`, and **nothing in the working tree prompts recovery**.

### Verdict — the amended predicate is now exercised, and it is the right one

- **The discriminating cell is (c1), and the two probes disagree on it.** Worktree-vs-HEAD says
  **dirty**; index-vs-HEAD — the withdrawn pair's question — says **clean**. That is the withdrawal's
  justification, driven rather than reasoned: `git_staged_snapshot` is an index-vs-HEAD probe, a
  pathspec commit takes the **worktree** contents, and the cell that loses bytes has nothing in the
  index at all. The pair yields zero findings exactly where the harm is.
- **The idempotence leg is no longer only an ordering argument.** (b2) and (b3) show `jigc setup` is
  **byte-idempotent over its own pathspec**: after a re-run `git diff HEAD -- <pathspec>` is empty, so
  the *next* invocation's pre-write question is clean too. A same-binary re-run can never
  self-trigger the refusal.
- **The fresh-clone leg is clean and needs no carve-out.** (a) is exit 0 with no setup having run in
  that checkout.
- **The unborn exemption is forced by `git diff HEAD` itself, not by a false refusal.** On an unborn
  HEAD the query is **fatal at exit 128**, which a predicate must not read as either verdict, so it
  needs an explicit `git rev-parse --verify -q HEAD` leg (or the empty-tree substitution). What it is
  *not* forced by is over-refusal: with the empty-tree substitute, a pre-existing **untracked**
  `CLAUDE.md` reads **clean** (rc=0) and only a **staged** one reads dirty. So the exemption is a
  decision about the bootstrap door, and the record can now state it with the exit codes rather than
  with a guess.

### Bounds of §4

- **The upgrade leg is driven for one binary only.** Only `1.0.0-rc.14` exists on this machine, so
  *"a later binary re-runs setup over an install an earlier binary committed"* — where `.jigc/version`
  and `SKILL.md`'s `jigc-version:` stamp differ — was **not** driven. The ordering argument covers it
  (the question is asked before any write, over a tree the previous setup committed), but that is an
  argument, and it is marked as one.
- Codex 3's four-state attribution model is still **refused on the ordering argument**; §4 does not
  bear on it either way.

---

## §5 · D4 row 6 — the non-finalize `gitignore::ensure` callers, driven (reuse-exercised)

### What was asked

A1's widening (*"three other production callers outside the transaction … the last never commits, so
on an `ENTRIES` upgrade a private line dies there with **no transaction at all**"*) was a **source
read**. Drive it: remove one canonical entry (`displaced/`) from the committed `.jigc/.gitignore`,
add a user line, commit; then run each caller and ask — was the user line destroyed, was it
committed, was there any transaction or rollback?

### The mechanism, quoted (`crates/cli/src/gitignore.rs:24-53`)

```rust
pub(crate) const ENTRIES: &str = "tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\ndisplaced/\n";
…
let needs_write = … !ENTRIES.lines().all(|entry| lines.contains(&entry)) …;
if needs_write { std::fs::write(&path, ENTRIES)?; }          // ← whole-file clobber
```

### Scripts

`drive5-d4.sh` (all four callers), `drive5b-d4.sh` (`milestone provision` **isolated**, after
`drive5-d4` let `milestone create` destroy the line before provision could be measured — that arm's
first result is contaminated and is superseded here), `drive5c-d4.sh` (`task finalize`).

Each rig is seeded identically, then committed:

```
tasks/ index/ state/ milestones/ worktrees/ logs/ my-private-notes/      # `displaced/` removed, user line added
```

### Output

| caller (site) | user line | left in the worktree? | **committed away?** | `git status` after | transaction / rollback |
|---|---|---|---|---|---|
| `jigc milestone create` (`milestone.rs:472`) | **DESTROYED** | yes — ` M .jigc/.gitignore`, unstaged | no (the record commit is pathspec-limited) | ` M .jigc/.gitignore` | **none** |
| `jigc milestone provision` (`milestone.rs:2066`), **isolated** | **DESTROYED** | yes — ` M .jigc/.gitignore`, unstaged | no (commits nothing) | ` M .jigc/.gitignore` | **none** |
| `jigc setup` re-run (`adapter.rs:1108`) | **DESTROYED** | **no** | **YES** — install commit `33a7524` | **clean** | **none** |
| `jigc task finalize` (`task.rs:2778`) | **DESTROYED** | **no** | **YES** — the task commit `ccca8c1` | **clean** | the D4 capture (restores on **failure**, not on success) |

`git show HEAD:.jigc/.gitignore | grep -c my-private-notes` after each: **1 · 1 · 0 · 0**.

The finalize arm narrates the change without naming what it cost:

```
finalized ccca8c1 — docs(notes): note the cache change
  modified .jigc/.gitignore
  added note.txt
  2 files committed
```

### Verdict — A1's claim is **CONFIRMED and sharpened**, and the priority inside it inverts

- *"No transaction at all"* is **true** of `milestone provision` and of `milestone create`: the
  clobber happens, nothing captures a pre-image, nothing rolls back.
- **But those two are the *least* harmful of the set on the success path.** They leave the loss
  **visible** (` M .jigc/.gitignore`) and **recoverable** (the line is still in `HEAD`; one
  `git checkout` restores it).
- **The silent, landed losses are at the two *committing* callers.** `jigc setup` and
  `jigc task finalize` each rewrite the file to canonical `ENTRIES` **and commit it**, leaving
  `git status` clean and the line present only in an earlier commit. `setup` does not even mention
  the file; `finalize` mentions it as `modified .jigc/.gitignore`, which is true and tells the reader
  nothing about the deletion.
- **This is the argument for the amend being a precondition rather than a rider**, exactly as the
  Settle has it: the capture's subject is the finalize transaction, and three of the four callers are
  outside it — but the caller whose loss is worst (`setup`) is outside it too, so only the
  amend-to-union reaches it. The drive therefore does not move the decision; it moves which caller
  the decision has to be *justified* by.

### Bounds of §5

- The **succeeding** finalize is what §5 drove. F3's `advocates/f3-spike3.sh` drove the
  **hook-rejected** finalize, where the restore leaves the bytes in **no** git object — a strictly
  worse cell that §5 does not re-drive and does not need to.
- The `milestone provision` row in `drive5-d4.sh` is **contaminated** (`milestone create` ran first
  and had already canonicalized the file). It is superseded by the isolated run in `drive5b-d4.sh`,
  and both are kept so the correction is on the record.

---

## §6 · The two remaining relayed rows, driven

### §6a · EC-14 — *"three `DECISIONS.md` citations point at blank lines"*

**Script** (`cite-sweep2.sh`): enumerate every `<file>.md:<line>` citation in `DECISIONS.md`, resolve
each against the repo (`./`, `design/`, `implementation/`, `completions/artifacts/`), and report
whether the cited line is blank, past EOF, or unresolvable.

**Repo-wide result:**

```
274 distinct citation strings  →  261 distinct (file, line) targets
    199 OK
     62 BLANK
      0 PAST-EOF
      0 UNRESOLVED
```

**Into the two posture homes specifically** (the claim's own subject) — every cited line, checked:

| doc | lines cited in `DECISIONS.md` | **blank** |
|---|---|---|
| `design/doc-read-surface.md` | 33, 68, 85, 86, 87, 92, 99, 107 | **85, 87, 107 → 3** |
| `design/command-output-contract.md` | 9, 11, 46, 59, 65, 67, 122, 154, 202, 204, 268, 321, 325, 337, 339, 343, 367, 376, 412 | **46, 59, 321, 325, 337, 339, 343, 367, 376 → 9** |

The two the brief named are both in the blank set:

```
doc-read-surface.md:87          ||     (blank — the content it means is at :86)
command-output-contract.md:367  ||     (blank)
```

**Verdict — CONFIRMED as to the two named citations, and the *number* is struck.** Into the posture
homes the count is **12, not three**; repo-wide **62 of 261** line-anchored citations in
`DECISIONS.md` land on a blank line — **23.8 %**. The claim understates its own class by a factor of
four in the homes it names and by twenty overall. Read together with the baseline's two earlier
corrections to EC-14 (*"eleven"* is current, not stale; the posture-home headings are presentation,
not a lie), EC-14's remaining live member is this one, and it is **larger than reported** — the same
shape four consecutive completion audits have recorded.

**One caveat on what "blank" means here, stated so the number is not over-read:** a citation landing
on a blank line is a *stale or off-by-N pointer*, not necessarily a claim about absent content — most
of the 62 sit one line off the paragraph they mean (`:85`/`:87` bracket `:86`). What it establishes
is that these citations are **not fenced against anything**, which is the axis EC-14 belongs to.

### §6b · Codex 6 — *"no seventh `Plain` argument id"*

**Method.** Enumerate every `ArgToken::Plain` entry in `ARG_TOKENS` (`crates/cli/src/cli.rs:1552`),
resolve each to its clap field type, and for each string-valued one trace whether the caller's value
becomes a `Path::join` component or a git argv element. This is a source read **executed here**, not
relayed: every row below was greped at HEAD.

**25 `Plain` ids. 11 are `bool`** and can carry no token at all: `approve`, `carry_staged`,
`commands`, `doctypes`, `dry_run`, `explain`, `force`, `no_commit`, `preview`, `unset`, `workflows`.

**14 are string-valued:**

| id | door(s) | becomes a path / argv element? | evidence |
|---|---|---|---|
| `path` | `migrate <path>`, `unmanage <path>` | **YES** — read, and the migrate subject | documented member |
| `from` | `relocate <type> --from <prior-home>` | **YES** — a directory or literal file | `cli.rs:312`, doc-comment names both shapes |
| `file` | `config insert-step <file>`, `replace-step <file>` | **YES**, twice — read as a `PathBuf`, and its **basename** becomes `.jigc/config/steps/<basename>.yaml` | `config.rs:984`, `:1051` |
| `from_file` | `config fill --from-file`, `doc author/set-slot --from-file` | **YES** — a path, or the `-` stdin sentinel | `config.rs:135`, `doc.rs:377`, `:445` |
| `target` | `config replace-step/remove-step/fill/fork <target>` | **YES** — its parsed `fill_id` / `step_id` becomes a path component | `config.rs:1135` (`fills_dir.join(format!("{}.md", parsed.fill_id))`), `:1190` (`steps_dir.join(format!("{step_id}.yaml"))`) |
| `value` | `config set <key> <value>` | **YES, conditionally** — path-like iff `key ∈ ROOT_KNOBS` | `config.rs:55`, `:402` — EC-27's member |
| `after` | `config insert-step --after` | no — an anchor step id; reaches `check_anchor_present` and the delta YAML only | no `join(`/`fs::` hit |
| `before` | `config insert-step --before` | no — same | no `join(`/`fs::` hit |
| `key` | `config set/get <key>` | no — a knob key adjudicated against `config/knobs.yaml`, written as a YAML map key | no `join(` hit in `config.rs` |
| `role` | `task bind <role> …` | no — a roles.json key | no `join(`/`PathBuf`/`args(` hit |
| `intent` | `start "<intent>"`, `milestone add-task <intent>` | no — an **identity** source, sanitized through `engine::slug::slugify` | no raw `join(` hit |
| `title` | `doc create --title`, `milestone create <title>` | no — same, `engine::slug::slugify` | `doc.rs:1954`, `:5537` |
| `to` | `doc rename --to`, `rename --to` | no — same, `engine::slug::slugify` | `doc.rs:2887` |
| `workflow` | `start --workflow`, `jigc workflow <id>`, `config insert-step --workflow` | **no — and this is the near-miss worth stating** | below |

**Why `workflow` is not the seventh.** It *looks* like one: `CascadeDefs::project_def`
(`start.rs:3467`) does `self.project_config.join(dir).join(format!("{id}.yaml"))`, an unvalidated
`ResourceId` newtype (`engine/src/packsource.rs:14`) sits between the token and the pack read, and
`jigc workflow ../../etc/passwd` is a syntactically legal invocation. It closes on two facts:

1. **Neither `PackSource::read` impl joins the id.** Both enumerate the directory and match on the
   *file stem*:
   `pack.rs:1465` — `dir.files().find(|f| f.path().file_stem() … == Some(id.as_str()))`;
   `pack.rs:1866` — `read_dir(…).find(|e| e.path().file_stem() … == Some(id.as_str()))`.
   A file stem can never contain `/`, so no escape shape can match.
2. **The one `join`-based read is gated behind a directory listing.** `project_def` runs only under
   `project_owns(id)` → `resolved.file_owner(id)` (`cascade.rs:783`), whose map is populated by
   `shadowed_definition_ids` → `project_native_ids` → `read_dir` file stems (`start.rs:3020`). The id
   must already *be* a listed file stem before the join happens.

**Verdict — CONFIRMED. There is no seventh.** Of 25 `Plain` ids, **six** become a path component or a
filesystem-op argument (`path`, `from`, `file`, `from_file`, `target`, and `value` under
`ROOT_KNOBS`); eleven are booleans; the remaining eight do not, and three of those eight (`intent`,
`title`, `to`) become *identities* only through `engine::slug::slugify`. **No string-valued `Plain`
id becomes a git argv element either** — the commit message is rendered from the commit doc, not from
`intent` or `title`.

**Bound, restated because it is the point of the totality fence rather than of this search.** This is
an enumeration by a reader, and a reader can miss. What makes a miss survivable is the ⇔ fence
(`cli_parse::every_clap_argument_is_classified`) forcing the question of every argument that ships —
the Settle's own words, and this drive does not replace them. What it *does* replace is the word
"relayed": the enumeration has now been run at HEAD, with a per-row citation.

---

## What is still owed

Two of the gate-record's four HALT cells are **untouched** by these drives, because neither was in
this brief and neither is a drive:

- **`acceptance-spiked` (HALT cell 1)** — the eight axes' `(door, cell)` matrices, the `VERB_KINDS`
  fence evaluated against them, and flow 52's arm set. The binary the per-axis review runs on
  (`1.0.0-rc.15`) does not exist and deliberately does not until after the audit's fixes land.
- **`design-complete` row 12 (HALT cell 4)** — D10's second answer: the code, its severity class, the
  surface it fires against, its route, and the D10/D12 partition. Still a name plus TBD.
