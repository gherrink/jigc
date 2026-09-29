I drove the debug binary (`target/debug/jigc`, built bare at HEAD `21665da0`) against six `dev/jigc-rig` corpora plus one hand-built repo under a path containing a space. Findings below; each is marked **DRIVEN** or **READ**.

---

# Findings

## HIGH 1 — the emitted `Spawn:` line is now unquoted and breaks for any repository path containing a space (new in `a8318211`)

**Location:** `crates/engine/src/compose.rs:1783` (`emit_fan_out_spawns`), operand fed at `crates/cli/src/milestone.rs:2648` (`recorded_workflows` → `SubTask::in_worktree`).

```rust
let cmd = format!("cd {worktree} && jigc workflow {workflow} --task {id}");
```

**DRIVEN.** A repo at `/private/var/.../T/jigc space.2RgM67/repo`, `jigc setup` → `milestone create/add-task/provision` → `milestone execute`:

```
Spawn: `cd /private/var/.../T/jigc space.2RgM67/repo/.jigc/worktrees/area-one && jigc workflow sub-task --task area-one`
$ sh -c "$cmd"
sh: line 0: cd: /private/var/.../T/jigc: No such file or directory      rc=1
```

Before this range the operand was the repo-relative `.jigc/worktrees/<id>` — a slug, never spaced — so **this failure mode is new**, and it lands on the line the census itself ranks #3 and calls "the highest-leverage line in the product." The sub-agent cannot enter its worktree at all.

Two things make this a sweep miss rather than bad luck:
- **The sibling one commit away got it right.** `crates/cli/src/start.rs:2189` renders the *same* directory as `cd {cd}` with `cd` passed through `shell_token`. DRIVEN on the same spaced repo: `jigc start --task area-one` → ``run this from that worktree — `cd '/private/var/.../jigc space.2RgM67/repo/.jigc/worktrees/area-one'` ``. `a8318211`'s own commit message says the refusal "matches" the spawn line; in the spaced cell they now disagree.
- **Neither fence sees it.** `Spawn:` is a plain `format!`, not a `Route`, so `command_spans_are_shell_safe` and the new `unaimed_git_span` are both blind to it — and the acceptance arm that would have caught it asserts the *unquoted* spelling (`crates/cli/tests/cwd_verb_subject.rs:643-651`, `seen[0].contains(&format!("cd {}", …))`) over a fixture root built by `std::env::temp_dir()`, which has no space. Input-coverage masking: the only cell driven is the canonical one.

**Smallest correction:** wrap the operand at the one place it is composed — `format!("cd {} && …", crate::finding::shell_operand(&worktree))` at `compose.rs:1783`. `shell_operand` returns the bare path when no quoting is needed (DRIVEN: the aimed `git -C` spans are bare on space-free roots and quoted on the spaced one), so `cwd_verb_subject.rs:643` stays green unchanged; add a spaced-root cell to that arm.

---

## HIGH 2 — every surface that *prints* a repo-relative path into a `jigc migrate <PATH>` route now emits a route that runs only from the repository root (new in `4f61c80a`)

**Location (driven):** `crates/engine/src/validate.rs:1209-1216` — `adoption_route`, `token = crate::finding::shell_token(rel_key)` where `rel_key` is the repo-relative store key.

**DRIVEN**, `fresh` rig with a foreign `CHANGELOG.md` staged, run from `$REPO/docs/deep`:

```
route: adopt — run `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` to
       rewrite it into the managed `changelog` shape; …

$ jigc migrate CHANGELOG.md --as changelog          # from $REPO/docs/deep
could not read the foreign `changelog` source at `CHANGELOG.md`
  route: check the path, then re-run `jigc migrate <path> --as changelog` with a readable file   rc=1

$ jigc migrate CHANGELOG.md --as changelog          # from $REPO                                  rc=0
```

The path jigc printed is correct; the base is not — and the dead-end's own remedy ("check the path") is exactly the unfollowable route C2-03 described *before* the fix. The census names this row (C1-17) as **"already correct — the model for the fix."** `4f61c80a` inverted its base and swept only the one producer that echoes the token the operator typed (`migrate.rs:300`).

Second instance **DRIVEN** by substitution: `jigc ingest` prints `unmanaged docs/deep/ — 1 file(s)…` and a footer routing `` `jigc migrate <path> --as <doctype>` `` (`crates/cli/src/render.rs:4163`); from `docs/deep`, `jigc migrate docs/deep/x.md --as adr` → rc=1, same message.

**How the count was derived:** `command grep -rn 'jigc migrate [^-c]' crates/cli/src crates/engine/src`, minus comment lines, `#[cfg(test)]` assertions and `.pending-snap` files → **6 production sites**. One (`crates/cli/src/migrate.rs:300`) echoes the typed token and is correct by the recorded decision. The other **five** interpolate or invite a repo-relative store path:
- `crates/engine/src/validate.rs:1212` — `adoption_route` (reached by the store sweep **and** `doc show`'s `reroute_unadopted`) — DRIVEN broken
- `crates/cli/src/orphan.rs:1073` — `unregistered_route` (called from `crates/cli/src/cli.rs:1490`, and the ingest door's sibling) — READ
- `crates/engine/src/finalize.rs:538` — `jigc migrate {source_token} --as <doctype> --slug …` — READ
- `crates/engine/src/finalize.rs:556` — `jigc migrate {destination_token} --as <doctype>` — READ
- `crates/cli/src/render.rs:4163` — the ingest footer placeholder beside a repo-relative listing — DRIVEN broken

**Smallest correction:** at those five, render the operand absolute (`repo_root.join(rel)` through `shell_operand`) — the same *pasteable shell bytes* disposition this range applied to every `git` span — or state the base in the route text. One home would be a `migrate_at(repo_root, rel, ty)` helper beside `git_at`.

---

## MEDIUM 3 — the installed pre-commit hook's out-of-band-rename block silently fails open on a repository path containing a space (new in `65de53f5`)

**Location:** `crates/cli/src/setup.rs:539` (extraction) and `:543` (the awk), inside `PRECOMMIT_RENAME_BLOCK`.

```sh
moves="$(printf '%s' "$report" | grep -o 'git -C [^`]*')"
… awk '… NF >= 6 && $4 == "mv" && ($5 in S) && ($6 in S) { hit = 1 } …'
```

`git_at` renders the home through `shell_operand` (`crates/engine/src/finding.rs:894`), so a spaced checkout becomes `git -C '/a repo' mv <new> <old>` — **two** awk fields for the home, shifting `mv` to `$5`. **DRIVEN** at the shell against the shipped hook text with the exact route shape:

```
plain path : moves=[git -C /private/var/repo mv docs/new.md docs/old.md]           BLOCKS
spaced path: moves=[git -C '/private/var/my repo' mv docs/new.md docs/old.md]      does not block
```

The M35 guard disappears with no message, in the fail-**open** direction. The ordinary case is fine — `cargo test -p cli precommit_hook_acceptance::` is green, 11 passed, including `commit_blocks_on_this_commit_oob_rename` (DRIVEN) — so only the spaced cell regresses, and no test covers it.

**Smallest correction:** make the awk positionally independent, e.g. `for (i=3;i<=NF-2;i++) if ($i=="mv" && ($(i+1) in S) && ($(i+2) in S)) hit=1`, and add a spaced-root arm to `precommit_hook_acceptance.rs`.

---

## MEDIUM 4 — `milestone create`'s base pin was left on the standing checkout while the boundary's subject moved to `jigc_home`, so a milestone created in a linked worktree is born un-finalizable

**Location:** `crates/cli/src/milestone.rs:671` (`discover_repo_root`), `:714` (`read_head(&repo_root)`), `:725` (`git_staged_snapshot(&repo_root)`) — against `:5292` (`git_head(&jigc_home)`) and `:5395` (`git_staged_snapshot(&jigc_home)`) at the boundary.

**DRIVEN.** `committed-singletons` rig, ordinary branch-attached worktree `feat` at `9e7adc4`, main advanced to `7c6b034` by a real (non-record) commit:

```
$ cd $RIG/feat && jigc milestone create "Worktree born"
minted milestone:worktree-born (shared base 9e7adc4)      # feat's HEAD
record commit: 76dcaf3                                     # landed on MAIN (main: d8c496f; feat HEAD unmoved at 9e7adc4)

$ jigc milestone finalize worktree-born      # from $REPO   → exit 3  finalize.base-mismatch
$ jigc milestone finalize worktree-born      # from feat    → exit 3  finalize.base-mismatch
  route: … return HEAD to `9e7adc4…`, run `jigc milestone finalize`, then re-land the newer commits on top
```

The milestone refuses on its **first** finalize, from every cwd, and the route asks the operator to rewind the main checkout's HEAD onto another branch's commit. `f919ea95`'s own reasoning — *"There was never a second subject to choose. Every read and write below is about the milestone"* — applies verbatim to the door that *sets* what the gate compares against, and was not applied there; `milestone.rs:669-670`'s comment still reads *"The base pin is the worktree HEAD."* This is not a clean regression (pre-range the pair agreed and then died on C2-06's git fault), but it is an un-swept member of the class `f919ea95` claims to have closed.

**Smallest correction:** `read_head(&jigc_home)` and `git_staged_snapshot(&jigc_home)` at `run_create`, with the comment restated; add a `create-in-a-linked-worktree → finalize` cell to `cwd_verb_subject.rs`.

---

## LOW 5 — `jigc setup` in a linked worktree now writes a `.jigc/config/` layer that nothing reads

**Location:** `crates/cli/src/pack.rs:2038-2042` (`discover_project_config` → `jigc_home`) against `crates/cli/src/setup.rs` (binds `repo_root`, declared out of the class in `GIT_SPAN_SITES`' preamble).

**DRIVEN.** `jigc setup` from `$RIG/feat` exits 0 and creates `$RIG/feat/.jigc/{AGENT.md,config,state,version}`. Planting `compose-embedded-methodology: false` in `$RIG/feat/.jigc/config/packs.yaml` changes nothing: `jigc describe --format json` from `feat` is md5-identical to the main checkout's (`58b18047…`), and `jigc config set default-workflow single-task` from `feat` lands in `$REPO/.jigc/config/manifest.yaml`. Pre-range the worktree-local layer *was* the one read from there. No loss, no surface says so.

**Smallest correction:** either refuse/narrate `jigc setup` inside a linked worktree, or have it install at `jigc_home` — a decision, not a swap; at minimum state it in the ack.

## LOW 6 — the new AGENT.md paragraph calls the `git` span "the one exception" while jigc prints at least two other absolutes

**Location:** `crates/cli/src/adapter.rs:1252` (`BOOTSTRAP_PATHS_AND_CWD`), goldens `crates/cli/tests/goldens/compose/composite/agent-md--*.txt`.

The code's own doc-comment two lines above says ***"Two** spellings are absolute and say so: the emitted `cd` of a fan-out `Spawn:` line, and … any backticked `git` command"* — the shipped prose names one. **DRIVEN** third case: `jigc task finalize` from a linked worktree prints ``committed in the linked worktree at `/private/var/.../feat` on branch `feat` `` (`crates/cli/src/render.rs:2537`), an absolute on a plain manifest line. The paragraph exists to close a law-1 gap and reopens a smaller one.

**Smallest correction:** say "a `git` command jigc prints, and the `cd` of a fan-out spawn line, are the exceptions" (and either fold `CommitSite` into that clause or leave it, since it is a display path, not pasteable bytes).

## LOW 7 — a doc-comment cites a test that does not exist

**Location:** `crates/cli/src/migrate.rs:460-463`: *"What the base does is driven from real subdirectories in `crates/cli/tests/cwd_verb_subject.rs`."* **READ:** that suite has six tests, none of which invokes `migrate`; the real pin is the new arm in `crates/cli/tests/path_arg_occurrence_axis.rs`. **Correction:** re-point the citation.

## LOW 8 — two shape gaps in `unaimed_git_span`

**Location:** `crates/engine/src/finding.rs:975-1035`. **READ:**
- The loop `continue`s unless the span's **first** token is `git`, so a composite span is never checked. `crates/cli/src/ingest.rs:719-736` emits exactly that shape (`` `mkdir -p <abs> && git -C <abs> mv …` ``) — the producer is correct, but the fence is structurally unable to verify it, and a future composite would slip.
- False positives: `git commit -m "docs/x.md"` and `git log <rev>..<rev>` tokenize to `command=commit|log`, `operand=<second non-flag token>`, are not in `GIT_NON_PATH_COMMANDS`, and would panic the debug binary at a `Route` constructor. No production site emits either today.

**Correction:** scan every `git` token in a span rather than only the head; add `commit`/`log`/`show` (or a `-m`-aware flag-with-value table) to the declared set with reasons.

## LOW 9 — `adapter::render_spawn` now renders a second, divergent spelling of the spawn line

**Location:** `crates/cli/src/adapter.rs:309-316` — `#[allow(dead_code)]`, still `engine::milestone::worktree_path(task_id)` (repo-relative), i.e. the exact form `a8318211` declared broken; its doc-comment still says *"Consumed by the launch path (this increment's later tasks)."* Inert today (confirmed dead by the attribute), but it is the code a future launch path would reach for. **Correction:** delete it, or take the absolute the way `recorded_workflows` does.

## LOW 10 — C2-07 restated: `jigc uninstall` from a fan-out worktree is unchanged, and still destroys the repository-wide hook while claiming completion

**DRIVEN** on a fresh fan-out rig, from `$REPO/.jigc/worktrees/area-one`: rc=0, prints *"jigc uninstall — repo-local install removed"* with 7 removal lines. After: `$REPO/.git/hooks/pre-commit` **gone** (the only copy for every checkout, and the one `setup` says git cannot track), while the main checkout's `.jigc/{AGENT.md,config,index,milestones,state,tasks,version,worktrees}`, `.claude/skills/jigc/SKILL.md` and `CLAUDE.md`'s `@.jigc/AGENT.md` all stand. Not made worse by this range (its aimed route correctly targets the worktree whose files it did remove), and the range's own claim is that `uninstall` binds no `jigc_home` — but it is a destroying door reporting a completion it did not perform, and the human asked whether it should be adjudicated here.

## LOW 11 — one member of the "five worktree-set helpers" sweep was left on the walk-up

**Location:** `crates/cli/src/milestone.rs:2880-2910` — `provision_worktrees` still takes `repo_root` and runs `git worktree prune` / `registered_worktrees` from it, while `subtask_worktrees`, `held_subtask_worktrees`, `provisioned_worktrees`, `partial_worktree_advisories` and `remove_worktrees` all moved to `jigc_home`. **READ** — behaviourally equivalent (`git worktree list/prune` answer for the whole repository from any checkout, and the compared paths are already built from `canonical_home`), so not a defect; named because it is the one remaining split in a set the commit message says was made uniform.

---

# What I verified holds (all DRIVEN unless noted)

- **`18c4655e` is the pure refactor it claims.** One `discover_repo_root` (`repo.rs:118`); 17 call sites remain across `start.rs` (8), `locate.rs` (2), `milestone.rs` (3), `ingest.rs`, `task.rs`, `cli.rs`, `repo.rs::jigc_home`. Every removed copy was byte-identical (READ, diff).
- **C2-06 closed.** `milestone finalize` from inside a provisioned worktree lands: exit 0, the boundary commit on the **main** checkout's HEAD (`160b0bf`), both sub-tasks' code in it, both worktrees torn down — run from the worktree it removes.
- **C2-11 closed.** With a real external commit on main, `finalize.base-mismatch` fires at **exit 3** (measured bare, not through a pipe) from root, subdir *and* the fan-out worktree, identically; HEAD unmoved.
- **The pack-loader fix is real and is what makes the record-only carve-out work from a worktree** — the boundary advanced over two `chore(milestone): record …` commits from inside the worktree; and `jigc describe --format json` / `jigc start` are byte-identical from a linked worktree and the root.
- **C2-02 closed.** `jigc task diff area-one` is md5-identical from root / `docs/deep` / its own worktree / a sibling worktree, reports `area-one.txt`, carries no `milestone-records` and no sibling's code; `--format json` keys unmoved (`base code_diff findings op staged_docs task`).
- **C2-09.** The site line appears on both `--dry-run` and the ack with the branch, and the pinned forecast envelope's keys are unmoved (`dry_run findings left_out manifest subject`).
- **The spawn `cd` is absolute and byte-identical** from root, subdir and a sibling worktree (unspaced roots).
- **`migrate <PATH>` is cwd-based**, the persisted `source-path` is the clean repo-relative spelling, and `jigc start --task <migration-id>` resumes correctly from the root and from a sibling worktree. `--from-file -` and a cwd-relative `--from-file ./b.txt` both work from a subdirectory; `jigc unmanage docs/decisions-log.md` (a `RepoRoot` row) works from a subdirectory.
- **The aimed spans run.** `finalize.carried-staged` aims at the **standing** checkout (the index that actually holds the carried path — the fixer's refinement is right, driven: planted in `feat`, routed at `feat`, run verbatim **from the main root**, rc=0, finding cleared); `home-vacated`'s staged arm run verbatim from `docs/deep`, rc=0, right file restored; `repo.operation-in-progress` at the milestone boundary (`aim_at`) run verbatim from `docs/deep`, rc=0.
- **The split holds on the wire.** `milestone finalize --format json` over a bisecting worktree: `key.target` and `location.address` = `.jigc/worktrees/area-one`, message host-path-free, route absolute.
- **`milestone discard --force` from inside the worktree it removes** completes at exit 0, removes both worktrees, settles the record on main.
- **C2-13** fixed: `unmanaged . (the repository root)`.
- **Goldens:** 6 files, `+2` lines each, the AGENT.md paragraph and a blank line — nothing else. No golden widened to admit output.
- **Suites green** (run bare, exit codes captured directly): `count_fences::` 0, `posture_member_inventory::` 0, `git_span_aim::` 0, `repo_relative_paths::` 0, `path_arg_occurrence_axis::` 0, `precommit_hook_acceptance::` 0 (11 passed).

---

# Verdict

**HOLDS WITH FINDINGS.**

The verb-class deliverable genuinely holds: the seven root-discovery copies really are one function, the jigc_home/Here partition is right at every door I drove, C2-06 / C2-11 / C2-02 / C2-08 / C3-01 are closed on the real binary, and the route class's aimed-span rule works, keeps the key/locus/message repo-relative, and is fenced. The two new suites drive the real binary and are not tautological.

What stops it being clean is the same shape three times: **the range moved paths from relative to absolute and did not re-ask the two questions an absolute raises** — *does it survive a shell?* and *does everything that consumes it still parse it?* HIGH 1 (the unquoted `Spawn:` `cd`) and MEDIUM 3 (the hook's awk field shift) are both new, both fail on a repository path with a space, and both are masked by fixtures that have none. HIGH 2 is the mirror: `migrate`'s base flipped from root to cwd and the five producers that *print* a repo-relative path into that verb were not swept, turning a route the census called "already correct" into a driven dead end. MEDIUM 4 is the classic half-swept pair — the boundary's subject moved to `jigc_home` and the door that sets what the boundary gates on did not.

None of the findings is data loss, and none of them is reachable without either a spaced repository path or a non-root cwd. All four of HIGH 1 / HIGH 2 / MEDIUM 3 / MEDIUM 4 are small, local corrections with an obvious test cell each.
