# Second re-review: `02699199` on `fix/rc24-link-on-the-way` (parent `26474b80`)

This review is read-only. I edited nothing in the repository and ran no gate.

- **The binary.** I built a debug `jigc` from `02699199` in my own detached worktree, with a private `CARGO_TARGET_DIR`. Both live under `…/scratchpad/rr3.QkO7ck/`, and the worktree was removed at the end with `git worktree remove`.
- **The comparison binary.** The previous re-review's debug build of `26474b80` (`…/scratchpad/rr.ix8f9W/target/debug/jigc`) stands in for the parent.
- **How each verdict was reached.** Every verdict comes from driving one of these binaries through `dev/jigc-rig --binary`, in rigs minted by `mktemp -d`.
- **Tests run.** I ran only two ignored byte-table arms (§4).

## Verdict

**No blocking finding. Every re-review item is closed when re-driven. The commit introduces two regressions, and one older gap in the rule now has a driven repro. All three are should-fix.**

- **Re-review items.** NB1 and NS1–NS5 are closed, and so are the notes the commit took on.
- **NB1 holds.** No read form wrote past a link, with a task minted: 15 read forms × 6 workbench places (90 runs); 5 forms, the two dry runs among them, × 6 places (`.jigc` whole, `index`, `state`, `tasks`, `config`, `logs`) after HEAD moved (30 runs); and 18 forms over a linked home.
- **NS1 holds.** Every JSON stream of every read form parses as one document.
- **The three should-fix findings:**
  - **NS-A:** `task finalize --dry-run` and `migrate-corpus --dry-run` now forecast success while the real command refuses.
  - **NS-B:** `jigc uninstall` now refuses outright, even with `--force`, when `CLAUDE.md` or the settings file is a link to nothing. It wrote nothing there before.
  - **NS-C:** a writing form moves bytes past a link inside the parking home (`.jigc/displaced/<id>`). The guard deliberately does not look there, and its comment's premise is false.
- **None of the three meets the blocking bar this cycle.** Nothing is lost. Nothing is written into a commit, or out of the repository, that the user's own link does not lead to. Each fix is a few lines.
- **NS-A is the one I would put first.** It reverses a pinned rule: *the forecast obeys the gate it forecasts*, `crates/cli/src/task.rs:3907`.

## 1. The re-review's items, re-driven on `02699199`

| Item | Status | Evidence (driven unless marked) |
|---|---|---|
| **NB1** | **closed** | `readforms-all.sh index state tasks` and `… config logs milestones`: 90 runs, 15 read forms × 6 places, and no file changed outside. `readforms2.sh` adds `.jigc` whole, `config` and `logs`, plus the two dry runs, `start`, `task validate` and `validate`, run **after an ordinary commit** (stale index): 30 runs, nothing written outside. `orientwb.sh fresh` / `committed-singletons` (the whole `.jigc` linked, a task open, HEAD moved): `jigc start` exits 0 and the outside tree is unchanged. Mechanism: `refuse_past_a_workbench_link` (`crates/engine/src/state.rs:1345-1356`), asked at `write_atomic` (`:1374`) and `SaveLock::acquire` (`:1596`); `load_committed`'s save is best-effort (`crates/engine/src/index.rs:254`), so the read still answers. Exit codes are unchanged from the parent (`start` 0, `task validate` 3, `validate` 0). |
| **NS1** | **closed** | `jsonreject.sh`: all 6 rejects exit 1 with empty stdout, and stderr parses as one document. `task validate`, `task diff`, `doc schema`, `doc show adr:nope`, `doc list --task nope` and `milestone list-tasks nope` carry `store.link-on-the-way` inside `findings`; `doc show no-such-doc`, `workflow … --preview` and `config get` reply with a single-key `{"error"}` and withhold it. `jsonok.sh`: all 18 read forms' success documents parse as one document per stream. The advisory rides in `start` (task row), `upgrade`, `validate`, `task validate` and `task finalize --dry-run`. Notes N-1 and N-2 cover the gaps. |
| **NS2** | **closed** | `movein.sh ../vendor/dec vendor-outside` (`vendor` a link out) and `… vendor-inner` (`vendor` a link inside) now give *copy it in*; `… realdir` (`vendor/dec` real) gives *move `vendor/dec` into its place*. The ancestor case `.` → *remove*. Note N-3 covers the root target, which falls through. |
| **NS3** | **closed** | `jigc setup` over a link at `docs/research`, `docs/ideas`, `docs/completions` or `docs/milestone-records` exits 1 with `store.link-on-the-way`, and nothing is written (the outside dir keeps 1 entry and `.jigc` is not created). See §3 for whether this refuses anything legitimate. |
| **NS4** | **closed for the two files it names** | `uleaf.sh` / `uleaf-plain.sh`: `uninstall --force` and `uninstall` over `.claude/settings.json` → a link to an outside file both refuse with `uninstall.remove-allowlist`, and the outside sha `8e60bcfd6fa6` is unchanged. The third file edited in place, the `pre-commit` hook, is still written through (NS-D). |
| **NS5** | **closed** | Case `repoint-failed-a-link-at-the-docs-new-home-moved-out-and-re-run` (`crates/cli/tests/route_catalog/config.rs:533-548`); readings count 32 → 34 (`doc_read.rs:1373`) and 14 → 15 (`config.rs` reading). Re-driven (`s1route.sh`): refuse, move the link out, re-run exit 0, both docs `R` to `notes/`. |
| N-a | closed | Both dry runs are read forms (`crates/cli/src/cli.rs:837-840`), which opened NS-A. |
| N-c | closed | `make_pack_ungated` sits before `discover_project_config`'s doc comment (`crates/cli/src/pack.rs:2131-2149`). |
| N-d | closed (classification text) | `route_catalog.rs:860-867` now says *by the outer code only*. |
| N-e | closed | A link at `.jigc/index` or `.jigc/worktrees/do-a-thing` gets a route ending *"…, then re-run this command"*, with no commit clause (`link_way.rs:196-227`). `.jigc` itself and `.jigc/config` keep the commit clause; both are tracked, so that is correct. |
| B1, S1, S2, S5, S6, S7 (older) | still closed | I re-ran `probe2/3/5/6/7`, `s2revert`, `s5setup`, `s6`, `s6c` and `retired`. Each gives its closed verdict, `s6c` now on the copy route (NS2). |

## 2. Raw writes into the workbench and the homes — the sweep

### How I counted

`rawwrites.py` lists every call of the following in `crates/{cli,engine}/src`, excluding `#[cfg(test)] mod` blocks and comment lines:

- `fs::write(`, `File::create(`, `OpenOptions::new()`, `create_dir_all(`, `fs::rename(`, `fs::copy(`, `set_permissions(`, `.write_all(`;
- the removals `remove_file(`, `remove_dir_all(`, `fs::remove_dir(`;
- `git` with a writing subcommand.

It found **213 hits**:

| Kind | Hits |
|---|---|
| `fs::write` | 33 |
| `create_dir_all` | 31 |
| `remove_file` | 24 |
| `.write_all` | 10 |
| `remove_dir_all` | 8 |
| `fs::remove_dir` | 6 |
| `OpenOptions` | 5 |
| `fs::rename` | 4 |
| `fs::copy` | 1 |
| `set_permissions` | 2 |
| `git` with a writing subcommand | 89 |

The `git` hits break down as `config` ×51 (nearly all reads), `add` ×10, `worktree` ×5, `read-tree` ×5, `restore` ×4, `update-index` ×3, `hash-object` ×3, `commit` ×3, `apply` ×2, and `stash`, `mv`, `merge` ×1 each.

`enclosing.py` maps each fs write site to its function. I then removed:

- **the primitives' own bodies:**
  - `write_atomic`, `state.rs:1376-1383`;
  - `SaveLock::acquire`, `:1609-1612`;
  - `invocation_log::append_record`, `:663-667`;
  - `regular_file::replace` and `open_no_follow`, `:191-211`, which ask `blocker` and so `link_on_the_way`;
  - `store::rewrite_home`, `:272-286`, which asks `home_entry` before and after the open;
- **the 6 `write_all` calls into a child's stdin.**

That leaves **58 fs write sites in 37 functions**. 22 of those functions write into `.jigc/`:

| Function(s) | Writes to | Reached by | Past a link? |
|---|---|---|---|
| `config::run_insert_step` / `run_replace_step` / `run_fill` / `run_fork` / `write_scalar` / `append_delta_entry` (6) | `.jigc/config/**` | writing leaves (`config …`) | no — `config`, `config/steps`, `config/fills`, `config/schemas`, `config/workflows` are `WORKBENCH_DIRECTORIES`, asked at dispatch |
| `doc::move_staged_identity` | `.jigc/tasks/<id>/docs/**` | writing | no — `tasks/*/docs` is asked, and so is any deeper link that leads to a directory. A deeper dangling link makes `create_dir_all`/`write` fail, so nothing is written |
| `gitignore::ensure` / `write_ignore` | `.jigc/.gitignore` | writing | no — `.jigc` is asked |
| `milestone::commit_record_only`, `engine::milestone::{mint_milestone, reseed_cache_from_record, materialize}` | `.jigc/milestones/**` | writing (`list-tasks` does not reseed, `milestone.rs:2877`) | no — `milestones/*/merged/docs` is asked |
| `milestone::provision_worktrees`, `task::add` (dedicated worktree, `.combine-<pid>-<nanos>`) | `.jigc/worktrees` | writing | no — `worktrees` and `worktrees/*` are asked |
| `setup::record_install_footprint` / `record_ignored_footprint` | `.jigc/state/…` | `setup` | no — setup's guard asks the workbench |
| `task::try_execute_finalize_plan` (message), `engine::state::{mint_task, write_amend_pin, write_staged_snapshot, StagedRollback::rollback}` | `.jigc/tasks/<id>/…` | writing | no — `tasks` and `tasks/*` are asked |
| `relocate::displace_foreign_squatter` | `.jigc/displaced/<free name>` | writing | no — `displaced` itself is asked and the leaf name is free |
| **`task::displace_foreign_area`** (`task.rs:7797`, `:7808`), callers `task.rs:7993` (finalize/discard post-commit) and `milestone.rs:7597` (boundary) | `.jigc/displaced/<unit>/<entry>` | writing, **after** the guard passed | **yes — driven** (NS-C) |
| **`rollback::park`** (`rollback.rs:804`, `:810`) | `.jigc/displaced/<door>/<identity dirs>/<name>.pre-image.<nanos>` | writing, on a raced rollback | **yes by reading** (same mechanism, not driven: reaching it takes a rollback that races a concurrent edit) |

The other 15 functions write elsewhere:

- **Homes:** `task::promote`/`copy_regular`, `milestone::materialize_and_commit_record`, `relocate::relocate_one`, `config::route_docs_root_repoint_orphans`, and `rollback::restore` (any path its transaction wrote). These are writing forms only, the homes are asked at dispatch, and what remains is the declared check-to-act window.
- **`std::env::temp_dir()`:** `task::materialize_*` (`ScratchTree`), `milestone_boundary_gate`, `rename::apply_and_commit`'s message, and `combine`'s `TempIndex`.
- **Git's hooks directory:** `setup::install_precommit_hook`, `remove_precommit_hook`, `make_executable`.
- **The root `.gitignore` and the adapter members:** `setup::seed_secrets_gitignore`, `adapter::{inject_reference, unwire_reference, write_settings}`.

On the git side:

- **`GIT_INDEX_FILE`** is redirected only to `temp_dir()` (`combine.rs:298`, `task.rs:8909`) or to the dedicated worktree's own index.
- **`git worktree add`** writes only under `.jigc/worktrees`, which is asked.
- **`git add`/`mv`** of a path past a link is refused by git itself, and jigc asks before git is asked.

**Removals:**

- The 38 removal sites are all in writing forms or in `uninstall`. The workbench removals are behind the dispatch guard; `uninstall` removes `.jigc` as the entry it is (`remove_dir_all` does not follow a top-level link).
- `engine::index::invalidate` (a `remove_file` of `edges.json`) has two callers, both writing (`rename.rs:990`, `task.rs:7984`).

**Read forms reach none of the raw workbench writes.** The functions above are all writing acts, and the driven sweep confirmed it (138 runs, §1 NB1 and NS1): no outside byte changed.

**Result:**

- **The class is 3 writers, an enumeration.** One raw-write mechanism reaches past a link for a writing form after the dispatch guard: the parking home's interior. Its writers are `displace_foreign_area` (2 call sites, one driven) and `rollback::park` (1 call site, read).
- **What makes it reachable:** `link_way::in_the_workbench` deliberately does not descend into `.jigc/displaced/` (`link_way.rs:418-421`, the `rel != DISPLACED` arm). Its stated premise, *"jigc writes into it only at a name that is free"* (`link_way.rs:356-357`), is false for both writers. They create subdirectories (`<unit>/`, `<door>/<dirs>/`) that may already exist from an earlier displacement.
- **The commit's claim** (DECISIONS: *"wrote nothing is true by construction, whatever a command's classification"*) holds for the read forms. The `storage.md` wording *"every atomic write … a task area's docs and state"* is broader than the code: `mint_task`, `write_staged_snapshot`, `write_amend_pin` and the `StagedRollback` restore write raw. None of those is reachable past a link, because the guard asks `tasks/*`.
- **The commit's count is test code.** *"67 call sites"* of `persist`: `grep -rn 'persist('` gives 65 including test modules, and `callcount.py` gives 26 outside them. The figure counts test code; it is not wrong about the mechanism.

## 3. What this commit newly broke, item by item

- **`uninstall`'s new link rule** (`setup.rs:3119-3167`, called at `:4978`) is sound for a link to a regular file inside the checkout. `reflink.sh AGENTS.md "# mine"` (setup merged into `AGENTS.md`) tears down and restores `AGENTS.md` to `# mine`. Into `.git` it is refused, because `link_end` refuses `.git/` (`setup.rs:2989-2994`).
  - **Into `.jigc`** (`CLAUDE.md -> .jigc/mine.md`, tracked): setup follows it and merges the reference into a file inside the workbench, and the install commit carries it. `uninstall` then removes `.jigc` (step 1) before step 2 unwires, so the user's tracked `.jigc/mine.md` goes with the workbench and `CLAUDE.md` is left dangling. The *removes N tracked file(s)* warning names it, and the file stays in HEAD. Note N-4.
  - **Two real defects:**
    - **NS-B:** a dangling link at either file is now refused, and `--force` does not help. This is a regression.
    - **NS-D:** the hook, the third file edited in place, is not covered. This is pre-existing.
  - **One older defect in the followed arm** (N-5): over an initially empty target, `unwire_reference` removes the user's **link** and leaves jigc's section in the target.
- **`store.link-on-the-way` as a declared singleton** (`crates/engine/src/finding.rs:265-285`, contract `command-output-contract.md:431`). In JSON, `carrying_the_held_advisory` (`render.rs:2049-2063`) dedups by code, so at most one instance per findings collection. Orientation repeats it once per active-task row, which is the posture family's precedent (*N keys in N slices, never a duplicate within one*, `orient.rs:166-171`).
  - **Gap (N-2):** `task diff`'s success document has a `findings` array (`render.rs:7525`) and does not carry it. That contradicts the contract bullet *"rides in the one document that has a `findings` array"*.
  - **Text mode:** `migrate-corpus --dry-run` over a link at `docs/decisions` prints two `store.link-on-the-way` findings, the advisory (*"This one only reads, and wrote nothing"*) then the blocking one from its own prior-homes check (N-6).
- **The four `render.rs` sites** (`:910`, `:976`, `:2081`, `:3315`) are the only findings envelopes that carry it. `finalize_landed` is reached from a read form only through `task finalize --dry-run`, where carrying is correct.
- **The single-key `{"error": …}`** withholds the advisory. Driven: `doc show no-such-doc`, `workflow no-such-wf --preview`, `config get no-such-key`, and `migrate-corpus --dry-run` over a link at a prior home. In the last, the guard's advisory named `.jigc/index` too, and that link goes unmentioned in the JSON. This is declared in DECISIONS as *left open, for the ledger*. Note N-1.
- **The move-in route's every-component-real check** (`link_way.rs:161-173`) is correct for the shapes driven (§1, NS2). New-code defect N-3: the repository root as the target falls past the new ancestor arm into *copy it in*.
- **Setup with the methodology pack:** it **refuses nothing legitimate**.
  - `setup` always writes `compose-embedded-methodology: true` (`setup.rs:2244` → `compose_marker_rendering`, `:6908`). It even flips a hand-set `false`, as it did before this commit. So after `setup` every project composes the methodology pack, and a link at one of its homes is refused by the next writing command anyway: setup now says it first.
  - Under an explicit `JIGC_PACK_DIR`, `make_pack_from_marker` ignores the marker (`pack.rs:2246-2247`), so setup asks only that pack's homes.
  - Driven: at `docs/research`, `docs/ideas`, `docs/completions` and `docs/milestone-records` it refuses with nothing written. At root-level `ideas` and `completions`, which are not homes under the default `docs-root`, it passes correctly.
- **The moved route-catalog case at `config.repoint-failed`** (`route_catalog/config.rs:526-548`): green as driven by hand (`s1route.sh`, §1).
  - The `EXPECTED_RED` rename `…-a-link-to-a-read-only-file-outside` → `…-inside` (`route_catalog.rs:694`) is not a masking widening. The outside state now refuses earlier, at the new site, and has two green cases of its own (`uninstall.rs:1229-1246`). The inside state is red for the pre-existing reason `THE_BIT_NOT_THE_LINK` names.
  - Doc-read reading `doc_read.rs:1432-1437` labels *"`jigc relocate`: a link at the new home of a doc it moves"* `Why::NotConstructible` (*"it takes a pack of the reader's own"*). The byte table constructs that state (its three `relocate-…-at-the-notes-new-home` cells). By N3's rule that is `Why::NotBuilt`. It copies its sibling row's wording, which has the same issue. Note N-7.

## 4. The byte table — *"785 rows, no cell moved"*

`byte-table-record-cells.tsv` has 785 lines: a header and 784 cells. This commit does not touch it.

- **On-demand cells: the 180 printed byte-for-byte as committed.** I ran the ignored arm `byte_table::the_cells_on_demand_are_run_and_printed` on this commit (107 s). Its `== cells on demand ==` block, sorted, is **identical** to the 180 rows of `byte-table-on-demand-cells.tsv`.
- **Every cell on both binaries: no cell moved.** I ran the ignored arm `byte_table::the_cells_are_run_on_the_binary_a_variable_names` with `JIGC_BYTE_TABLE_BINARY` set to the `26474b80` build (803 s, `test result: ok`). It ran 784 cells on each binary, the gate's and the on-demand ones.
  - Parent against this commit: of the 784 cells, **0** differ in class or exit (awk over columns 7/8 against 10/11 of the printed `== cells ==` block).
  - This commit against the committed record: of the 784 cells, **0** differ in command, state, *listed as*, marks, key, class or exit. Every cell is present, and none is new.
  - **So the claim *"785 rows, no cell moved"* holds exactly.** The 785 rows are a header and 784 cells.
- **Why nothing should move, by reading.** No cell plants a state this commit changed. That rests on a grep of the record:
  - no `uninstall` cell has a link at `CLAUDE.md`, the settings file or the hook (27 `uninstall` cells read);
  - no `setup` cell has a link at a methodology home (the 30 `setup … link` cells are all at install members or the workbench);
  - no dry-run cell has a link at a directory on the way: the four `migrate-corpus-dry-run-…-link-…` cells plant at the doc's own file;
  - no cell plants inside `.jigc/displaced/`.

## 5. Findings, ranked

### Blocking

None.

### Should-fix

**NS-A — The two dry runs reclassified as read forms now forecast success over a link, where the command they forecast refuses.** This is a regression, new in this commit.

- **Location:** `crates/cli/src/cli.rs:837-840` (`Command::reads_only` gains `MigrateCorpus { dry_run: true }` and `Finalize { dry_run: true }`).
- **The rules it breaks:**
  - `crates/cli/src/task.rs:3907-3913` (*"the forecast obeys the gate it forecasts … forecasting it at exit 0 was a false green"*);
  - `crates/cli/src/migrate_corpus.rs:272-274` (*"`--dry-run` … printing the identical report an applying run would print"*) and `:970` (*"That falsifies dry-run's whole contract"*).
- **Evidence:**
  - `dryforecast.sh`, an authored `fix-a-typo`, `README.md` staged:
    - link at `docs/decisions`: `task finalize --dry-run` → exit 0, *"would commit — fix: Fix a typo in the readme."*; `task finalize` → exit 1, `store.link-on-the-way`;
    - link at `.jigc/index`: the same pair.
    - On `26474b80` all four runs exit 1.
  - `migrate-corpus --dry-run` over a link at `.jigc/index`, one research doc below its version: exit 0, *"1 would migrate"*; `migrate-corpus` → exit 1. On `26474b80` both exit 1. Over a link at `docs/decisions` (`adr`'s prior home), its own prior-homes check still refuses.
- **Masked:** `link_on_the_way_axis.rs`'s new `no_read_form_writes_past_a_linked_workbench_place` checks only that nothing is written outside. No test compares a dry run's exit with its command's.
- **Class and count derivation:** the `reads_only` additions in this diff are exactly 2, and both forecast a writing act. Both were driven.
- **Robust end state:** keep both dry runs refusing (the parent's behaviour; N-a was "errs on the safe side"), or have the forecast carry the refusal at the committing exit. The second is what M47's rule asks of every other refusal the forecast predicts.

**NS-B — `jigc uninstall` now refuses outright, `--force` included, when `CLAUDE.md` or the settings file is a link to nothing, and its route's first way cannot be followed.** This is a regression, new in this commit.

- **Location:** `crates/cli/src/setup.rs:3125-3167` (`edited_through_links` takes every `LinkEnd::Refused`, the dangling end included, from `link_end` at `:2966-2971`), called at `:4978`.
- **Evidence:**
  - `danglingref.sh`, a `fresh` rig, the file replaced by `ln -s gone.md` and committed: on `02699199`, `uninstall` exits 1 for both `CLAUDE.md` (`uninstall.unwire-reference`) and `.claude/settings.json` (`uninstall.remove-allowlist`), and `.jigc` is kept.
  - On `26474b80` both exit 0, `.jigc` is removed and no `gone.md` is created. `unwire_reference` reads first and treats `NotFound` as nothing to do (`adapter.rs:1007-1010`). So the parent wrote nothing through the dangling link.
  - The refusal's reason is setup's: *"merging through it would create a file wherever it points"*. Uninstall never creates there.
  - The route's first way, *"replace the link … with the file it points at — a copy"*, has no file to copy. The site's reading admits this (`route_catalog/uninstall.rs` `ONE_ROUTE_EVERY_END`: *"over a link to nothing it has no bytes to copy"*).
- **Why it matters:** `storage.md` makes `uninstall` the one door the guard stands down at so that a link never keeps a user from taking jigc out. A dangling bootstrap link (its target moved or deleted) is an ordinary state.
- **Class and count derivation:** 2 files × the 1 end (`leads to nothing`) where the parent wrote nothing. I read the four `LinkEnd::Refused` arms of `link_end`. A link to a directory failed at the parent too (`EISDIR`, after `.jigc` was removed), so refusing that end first is an improvement, not a regression.
- **Robust end state:** for the teardown, a link to nothing has nothing to edit. Skip it (or name it in a note), and refuse only the ends a read would follow out of the checkout.

**NS-C — A writing form moves bytes past a link inside the parking home `.jigc/displaced/`, which the guard deliberately does not walk.** This is older, from `26474b80`'s S1 carve-out, and driven here for the first time.

- **Location:**
  - `crates/cli/src/link_way.rs:418-421` (no descent into `displaced`), and its premise at `:356-357` (*"jigc writes into it only at a name that is free, so what it holds … is on the way to nothing jigc writes"*);
  - the writers: `crates/cli/src/task.rs:7790-7810` (`displace_foreign_area`: `home = displaced/<unit_id>`, `create_dir_all(parent)`, `fs::rename`) and `crates/cli/src/rollback.rs:794-811` (`park`: `displaced/<door>/<identity dirs>/…`).
- **Evidence:** `displaced.sh`, `committed-singletons`:
  - Setup: an authored `fix-a-typo`, `.jigc/tasks/fix-a-typo/NOTES.md` planted, and `.jigc/displaced/fix-a-typo` a link to an outside directory.
  - `jigc task finalize fix-a-typo` exits 0 and prints *"…NOTES.md → .jigc/displaced/fix-a-typo/NOTES.md"*. `NOTES.md` is now in the outside directory, and `.jigc/displaced/fix-a-typo/` in the repository is only the link.
  - Control (`nolink`): the file lands in the repository's `.jigc/displaced/fix-a-typo/`.
- **Severity reasoning:** nothing is lost, the destination is the user's own link, and the parking home is gitignored. But it is a write past a link on the way at a door the ruling covers, and the code comment that excused it is false.
- **Class and count derivation:** §2's enumeration. 3 writers reach into the parking home's interior: `displace_foreign_area` at 2 call sites (`task.rs:7993`, `milestone.rs:7597`; one driven) and `rollback::park` at 1 (read). `relocate::displace_foreign_squatter` writes only at a free leaf directly under `displaced` and is safe.
- **Robust end state:** ask `workbench_link_on_the_way` (or `regular_file::link_on_the_way`) at those three writers before the `create_dir_all`, and on a link fall to the *could not park it* arm (it does not move the bytes, so the area keeps them). Alternatively, let the walk descend into `displaced/` for directories only.

**NS-D — `jigc uninstall` still rewrites an outside file through a link at the `pre-commit` hook, the third file it edits in place.** This is older; it is NS4's class at its third member.

- **Location:** `crates/cli/src/setup.rs:1209-1231` (`remove_precommit_hook`, `RestoreForeign` → `fs::write(&hook, …)` at `:1228`). `edited_through_links` is given only the reference file and the settings file (`:4978-4990`).
- **Evidence:** `hooklink.sh`, a `bare` rig:
  - A foreign `pre-commit` hook; `jigc setup` wraps it.
  - The hook is moved outside and linked back.
  - `jigc uninstall` exits 0, and the outside file goes from sha `bdad46da6ea7` (66 lines) to `ffe57f281819` (2 lines).
  - `jigc setup` refuses the same shape when the link leads out of both the repository and the hooks directory (`merged_through_links`'s fifth member, `setup.rs:3169` onward).
- **Class and count derivation:** I read `uninstall` steps 1–7 (`setup.rs:4945-5200`). Three files are edited in place (reference, settings ×3 steps, hook), and 2 are covered. The guide is removed with `remove_file`, which removes a link and not its target, so it is safe.
- **Robust end state:** add the hook to `edited_through_links` with setup's hook rule (follow a link only where `setup` would).

### Notes

- **N-1 — JSON read forms that withhold the advisory.** A successful read whose document has no `findings` array withholds it:
  - `doc show`, `doc list`, `doc schema`, `task list`, `describe`, `config get`/`list`, `start --explain`, `start --workflow router`, `workflow --preview`;
  - orientation **with no active task**, which has no row to carry it;
  - every single-key `{"error"}` reject.

  A JSON-only driver learns of the link only when a write refuses. This is declared (DECISIONS, *Left open*). Text mode prints it everywhere.
- **N-2 — `task diff --format json` has `findings: []` and does not carry the advisory.** Location: `render.rs:3966`, `:7525`. This contradicts the new contract bullet (`command-output-contract.md:431`, *"rides in the one document that has a `findings` array"*) and DECISIONS (*"every findings report a read form prints carries it"*). Derivation: I walked every key of the 16 read forms' success documents (`jsonok.sh` plus a key walk), and `task diff` is the only one with a `findings` array that does not carry it.
- **N-3 — A link whose target is the repository root gets the *copy it in* route.** Location: `link_way.rs:153-165`; the empty `rel` is filtered out before the new ancestor arm. Driven: `docs/decisions -> ..` gives *"replace the link … with a real directory holding a copy of what it leads to"*. Followed literally, that copies the whole checkout, `.git` included, into `docs/decisions`. The root is the ancestor of every link, so by the arm's own comment the answer is *remove*. `docs/decisions -> ../.git` likewise gives *copy* (documented, but a nested repository).
- **N-4 — A bootstrap link into `.jigc` is followed.** `CLAUDE.md -> .jigc/mine.md`: `setup` merges jigc's reference into a file inside the workbench, and `uninstall` deletes it with `.jigc` before unwiring, leaving `CLAUDE.md` dangling. The file stays in HEAD and the warning names it. Consider having `link_end` refuse `.jigc/`, as it refuses `.git/`.
- **N-5 — `unwire_reference` removes the link, not the section, when the target held only jigc's section.** Location: `adapter.rs:1032-1037`. `reflink.sh AGENTS.md EMPTY`: `CLAUDE.md -> AGENTS.md` (empty) gets the section after `setup`; `uninstall` exits 0, **deletes the user's `CLAUDE.md` link**, and leaves `## Project interface … @.jigc/AGENT.md` in `AGENTS.md`. This is older than the commit, but it is the "followed" arm of NS4's rule.
- **N-6 — Text mode, `migrate-corpus --dry-run` over a link at a prior home, prints two `store.link-on-the-way` findings.** First the advisory (*"This one only reads, and wrote nothing"*), then the door's blocking refusal. In JSON the reject is a single `{"error"}` naming only the prior-home link, not every link the guard found (N1's *every link named in one refusal*).
- **N-7 — `Why::NotConstructible` where the state is built elsewhere.** Location: `route_catalog/doc_read.rs:1432-1437` (and its sibling `:1439-1444`). The byte table constructs a link at `relocate`'s new home, so by N3's own rule this is `Why::NotBuilt`.
- **N-8 — Count wording.** DECISIONS' *"`persist` (67 call sites)"* includes test modules; production code has 26 (`callcount.py`).
- **Leads, not pursued:**
  - A link at a workbench file's **own** place (the leaf, not the way) is outside this rule. Example: `.jigc/config/manifest.yaml` → an outside file under `config set` (`config.rs:2332`, a raw `fs::write`).
  - A linked hooks **directory** (`.git/hooks` → elsewhere) is written through by `install_precommit_hook`. It is git's directory and probably intended, but no doc says so.
  - The `uninstall.*` link refusal prints the outside target's host path (`link_end`, `setup.rs:2979-2982`). This is the existing absolute-path rule for outside paths, already a lead in the re-review.

## Repro blocks

### NS-A — the dry run forecasts what the command refuses

```bash
BIN=<debug jigc of 02699199>; WT=<a worktree of it>
rig=$("$WT"/dev/jigc-rig --binary "$BIN" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
$JIGC start --workflow quick-fix "fix a typo" >/dev/null 2>&1
$JIGC doc set-field commit:fix-a-typo#header/type --task fix-a-typo --value fix >/dev/null
printf 'Fix a typo in the readme.\n' | $JIGC doc set-slot commit:fix-a-typo#summary --task fix-a-typo --from-file - >/dev/null
echo "typo fixed" >> README.md; git add README.md
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p docs; ln -s "$O" docs/decisions
$JIGC task finalize fix-a-typo --dry-run; echo "exit=$?"   # exit=0, "would commit — fix: …" (26474b80: exit=1)
$JIGC task finalize fix-a-typo; echo "exit=$?"             # exit=1, store.link-on-the-way
```

### NS-B — uninstall over a dangling bootstrap link

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
rm CLAUDE.md; ln -s gone.md CLAUDE.md; git add -A; git commit -q -m "a link to nothing"
$JIGC uninstall --force; echo "exit=$?"   # exit=1, uninstall.unwire-reference … "leads to nothing" (26474b80: exit=0, nothing written)
```

### NS-C — a finalize parks past a link in the parking home

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
$JIGC start --workflow quick-fix "fix a typo" >/dev/null 2>&1
$JIGC doc set-field commit:fix-a-typo#header/type --task fix-a-typo --value fix >/dev/null
printf 'Fix a typo.\n' | $JIGC doc set-slot commit:fix-a-typo#summary --task fix-a-typo --from-file - >/dev/null
echo x >> README.md; git add README.md; printf 'my own notes\n' > .jigc/tasks/fix-a-typo/NOTES.md
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p .jigc/displaced; ln -s "$O" .jigc/displaced/fix-a-typo
$JIGC task finalize fix-a-typo; echo "exit=$?"   # exit=0; "NOTES.md → .jigc/displaced/fix-a-typo/NOTES.md"
find "$O" -type f                                # <outside>/NOTES.md
```

### NS-D — uninstall through a linked pre-commit hook

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" bare 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
H=$(git rev-parse --git-path hooks)/pre-commit; mkdir -p "$(dirname "$H")"
printf '#!/bin/sh\necho my own hook\n' > "$H"; chmod +x "$H"; $JIGC setup >/dev/null 2>&1
O=$(mktemp -d "$RIG/outside.XXXXXX"); cp "$H" "$O/pre-commit"; rm "$H"; ln -s "$O/pre-commit" "$H"
shasum "$O/pre-commit"; $JIGC uninstall; echo "exit=$?"; shasum "$O/pre-commit"   # exit=0; sha changed
```

### N-3 — a link to the repository root

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
mkdir -p docs; ln -s .. docs/decisions; git add -A; git commit -q -m l
$JIGC start --workflow quick-fix "fix a typo"   # route: "replace the link … with a real directory holding a copy of what it leads to"
```

### N-2 — `task diff` carries no advisory in its `findings`

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
$JIGC start --workflow quick-fix "fix a typo" >/dev/null 2>&1; O=$(mktemp -d "$RIG/o.XXXXXX"); mkdir -p docs; ln -s "$O" docs/decisions
$JIGC --format json task diff fix-a-typo   # "findings": [] — the advisory is not there
```

## Tools

All the tools are in `<scratch>/rr3.QkO7ck/tools/`. Each is one rig and one question against a named binary.

**Copied from the re-review**, re-pointed at this binary with a `sed` of the `R=` / `WT=` / `BIN=` lines: `readforms.sh`, `readforms-all.sh`, `jsonreject.sh`, `orientwb.sh`, `s1route.sh`, `uleaf*.sh`, `probe*.sh`, `s2revert.sh`, `s5setup.sh`, `s6*.sh`, `retired.sh`.

**New:**

- **`readforms2.sh <state> <place|.> <argv…>`**: does a read form write past a link at `.jigc` (whole) or `.jigc/<place>`, after an ordinary commit has moved HEAD? It extends `readforms.sh` with the whole-`.jigc` case and the stale-cache commit.
- **`jsonok.sh`**: do all 18 read forms keep one document per stream under `--format json` with a link at a home, and which documents carry the advisory?
- **`dryforecast.sh <bin> <place>`** and **`migdry.sh <bin>`**: does a dry run's exit match its command's over a link? This is the check NS-A lacks, and a candidate for the axis suite: for every read form that forecasts a writing act, its exit equals the act's.
- **`danglingref.sh <bin> <file>`**: does `uninstall` tear down over a link to nothing at `CLAUDE.md` or the settings file?
- **`displaced.sh [nolink]`**: does a finalize park past a link inside `.jigc/displaced/`?
- **`hooklink.sh`**: does `uninstall` rewrite an outside hook through a link?
- **`reflink.sh <target> <content|EMPTY> [--force]`**: what do `setup` and `uninstall` do with a `CLAUDE.md` linked inside the checkout?
- **`movein.sh <link-target> <variant>`**: which way out is printed for a given link spelling?
- **`rawwrites.py <root>`**: every raw fs/git write site in `crates/{cli,engine}/src`, outside `#[cfg(test)] mod` blocks. Paired with **`enclosing.py <root> <file:line>…`**, which names each site's function, it answers *which writes bypass the primitives*. It is a candidate for `dev/`, as the census the brief asked the builder for.
- **`callcount.py <root> <regex>`**: production-only call counts.

**Instruments run:** the byte table's two ignored arms (`the_cells_on_demand_are_run_and_printed`, and `the_cells_are_run_on_the_binary_a_variable_names` with `JIGC_BYTE_TABLE_BINARY` set to the parent's build). Running the parent as the *named* binary turns that arm into a per-commit differ: it reports which cells this commit moved.

**Re-running them:** the scripts read `$R/wt/dev/jigc-rig` and `$R/target/debug/jigc`. The binary and the target directory are kept. The worktree was removed, so re-create it first with `git worktree add --detach <R>/wt 02699199`.

**Missing tool, again:** a rig option to plant a link at a named place, inside or outside. Every probe here builds one by hand.

**Trap, recorded:** in zsh, `echo ====` fails (`=` expansion), and an unquoted `$var` holding several paths is not word-split. Use `xargs`, or `${=var}`.

<!-- end of report -->
