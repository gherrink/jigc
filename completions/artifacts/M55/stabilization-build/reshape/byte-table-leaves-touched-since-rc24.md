# The write leaves the fix pass touched since `jigc 1.0.0-rc.24`

*The count the byte table's bound stands on, as the plan's task `B3` asks ([byte-table-plan.md](byte-table-plan.md) → §1.4, §8): which of the product's 36 writing commands the fix pass touched, under each reading of *touched* the plan names, each leaf the plan's script called untouched read against its handler. Made on 2026-10-13 on the branch `fix/rc24-route-catalog`. The suite reads the table of §2 (`byte_table::every_scheduled_leaf_is_one_the_count_holds_untouched`): a line a write leaf, and no leaf the count holds touched under the plan's rule may stand on the byte table's scheduled list.*

**A count, not a ruling.** What *touched* means is the human's to say at the pilot's stop; this file gives the number under each reading and says how each was established.

- **Previous release:** `91834b5e011de2c36e2be2b79e96c0b9f60a803c` (`jigc 1.0.0-rc.24`).
- **Read at:** `aeeb0839089626041aab8af99185a8ef76e70900` (branch `fix/rc24-route-catalog`).
- **The change:** `git diff --stat 91834b5e HEAD -- crates/cli/src crates/engine/src` — 29 files, 18,477 insertions, 1,388 deletions.
- **A write leaf** is one row of `VERB_KINDS` (`crates/cli/src/cli.rs`) classed `VerbKind::Write`: 36 of its 48 rows.

## 1. The numbers, by reading

| the reading of *touched* | touched | untouched | how it was established |
|---|---:|---:|---|
| **every changed function** — a leaf is touched when the process that runs it reaches any function that changed | **36** | 0 | read: every leaf runs through `fn main` (`crates/cli/src/main.rs`), which changed, and through the invocation log's `LogWrite::for_leaf` and `enabled_logs_dir`, which changed with it (§4) |
| **the plan's rule** — its handler reaches a function that changed and that not every write leaf reaches, whatever the change was | **30** | 6 | the plan's script for 28; by reading for the ten the script called untouched or touched only through what a command prints: `ingest` and `unmanage` join the 28, and six stay out (§3) |
| **a change that moves a byte or a refusal** — the same, counting only a change to what the command writes, where, or when it refuses | **at most 26** | at least 10 | by reading for the ten: none of them reaches a change of that kind. **The other 26 were not read under this reading**, leaf by leaf — the script counts a changed function whatever the change was (§5) |

**The plan's range was 26 to 30 by reading, with six leaves read by nobody.** All ten are read now. Under the rule the pilot is built under the number is **30 of 36**, and the six that stand outside it are `task bind` and the five `config` leaves that are not `config set`.

**What the pilot's lists do with it.** The byte table's scheduled list held eight leaves, on the script's word. Two of them — `ingest` and `unmanage` — are touched under the plan's rule, so they leave the scheduled list and stand on the owed list, under the task of the plan that would bring the install family's cells (`B5`); six stay scheduled, each with this file as its reason. **Under the narrowest reading the two would go back, and `workflow` and `task amend` would join them.**

## 2. The table

| leaf | every changed function | the plan's rule | a change that moves a byte or a refusal | how it was established |
|---|---|---|---|---|
| `jigc start` | touched | touched | not read | the plan's script: 5 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: 2 |
| `jigc workflow` | touched | touched | untouched | read: both of its forms reach changed functions that fill and print one presentation field (`Composition.code_only`), a new read of git (`repo::home_is_a_checkout`), and `state::record_doc_provenance`, whose new parameter this caller passes as `None` (§3) |
| `jigc setup` | touched | touched | not read | the plan's script: 57 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: `setup::install`, 1 call from the handler |
| `jigc uninstall` | touched | touched | not read | the plan's script: 74 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: `setup::uninstall`, 1 |
| `jigc ingest` | touched | touched | untouched | read: its handler `ingest::run` calls `FileStateRecord::load` and, where it adopted something, `::save` — both changed, neither reached by every write leaf. The change to `load` is the text of an existing refusal, over a record that is there and cannot be read; the change to `save` moved its body into `save_locked` and writes what it wrote (§3) |
| `jigc migrate` | touched | touched | not read | the plan's script: 5 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: 1 |
| `jigc migrate-corpus` | touched | touched | not read | the plan's script: 10 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: 2 |
| `jigc unmanage` | touched | touched | untouched | read: its handler `unmanage::run` calls the same `load` and, where it dropped something, `save`; and `render::unmanage` gained a sentence of its text ack (§3) |
| `jigc rename` | touched | touched | not read | the plan's script: 16 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: 1 |
| `jigc relocate` | touched | touched | not read | the plan's script: 10 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: 4 |
| `jigc doc create` | touched | touched | not read | the plan's script: 17 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler itself |
| `jigc doc add-item` | touched | touched | not read | the plan's script: 10 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler; `doc::read_or_copy_in`, 1 |
| `jigc doc remove-item` | touched | touched | not read | the plan's script: 10 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler; `doc::read_or_copy_in`, 1 |
| `jigc doc retitle-item` | touched | touched | not read | the plan's script: 10 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler; `doc::read_or_copy_in`, 1 |
| `jigc doc rename` | touched | touched | not read | the plan's script: 14 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler itself |
| `jigc doc set-field` | touched | touched | not read | the plan's script: 11 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler; `doc::read_or_copy_in`, 1 |
| `jigc doc set-slot` | touched | touched | not read | the plan's script: 10 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler; `doc::read_or_copy_in`, 1 |
| `jigc doc author` | touched | touched | not read | the plan's script: 18 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler itself |
| `jigc task amend` | touched | touched | untouched | read: `start::amend_in_repo` reaches the same field, the same read and the same inert parameter (§3) |
| `jigc task discard` | touched | touched | not read | the plan's script: 14 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: `milestone::discard_sub_task_item`, 2 |
| `jigc task finalize` | touched | touched | not read | the plan's script: 46 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: `task::finalize`, 1 |
| `jigc task bind` | touched | untouched | untouched | read: `task::run_bind`, `TaskArea::resolve`, `TaskArea::bind` and `render::task_ack` hold no changed line; the script's one edge is a local closure named `head` (§3) |
| `jigc config set` | touched | touched | not read | the plan's script: 14 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: 1 |
| `jigc config insert-step` | touched | untouched | untouched | read: `config::run_insert_step` and what it calls hold no changed line — the file's four hunks are in `reject_repoint` and `repoint_failed`, which `config set` alone calls (§3) |
| `jigc config replace-step` | touched | untouched | untouched | read: `config::run_replace_step`, as above (§3) |
| `jigc config remove-step` | touched | untouched | untouched | read: `config::run_remove_step`, as above (§3) |
| `jigc config fill` | touched | untouched | untouched | read: `config::run_fill`, as above (§3) |
| `jigc config fork` | touched | untouched | untouched | read: `config::run_fork`, as above (§3) |
| `jigc milestone create` | touched | touched | not read | the plan's script: 7 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: `guard_record_free` · `append_and_commit_record`, 1 |
| `jigc milestone add-task` | touched | touched | not read | the plan's script: 7 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: `guard_record_free` · `append_and_commit_record`, 1 |
| `jigc milestone add-from-spec` | touched | touched | not read | the plan's script: 7 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: `guard_record_free` · `append_and_commit_record`, 1 |
| `jigc milestone provision` | touched | touched | not read | the plan's script: 30 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler, or 1 |
| `jigc milestone execute` | touched | touched | not read | the plan's script: 17 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler, or 1 |
| `jigc milestone join` | touched | touched | not read | the plan's script: 9 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler, or 1 |
| `jigc milestone finalize` | touched | touched | not read | the plan's script: 80 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler, or 1 |
| `jigc milestone discard` | touched | touched | not read | the plan's script: 48 changed functions it reaches that not every leaf reaches; the nearest, and calls from the handler: the handler, or 1 |

## 3. The ten leaves, read against their handlers

Read on 2026-10-13 by a read-only helper of the builder — who ran nothing — and checked by the builder against the diff at the places named: every hunk of `crates/cli/src/config.rs`, the spans of `task::run_bind`, `TaskArea::resolve` and `TaskArea::bind`, `crates/cli/src/unmanage.rs` and `crates/cli/src/ingest.rs` whole (neither has a hunk), and the hunks the changes below stand in. A line number is this tree's at the commit above.

| leaf | its handler | changed functions it reaches that not every write leaf reaches | what each change is |
|---|---|---|---|
| `ingest` | `ingest::run` (`crates/cli/src/ingest.rs:186`) | `FileStateRecord::load` (`crates/engine/src/file_state.rs:252`), called at `ingest.rs:205`; `FileStateRecord::save` (`file_state.rs:161`), called at `ingest.rs:288` where something was adopted; and the two new functions they call, `unreadable_record` and `save_locked` | `load`: a record that is there and cannot be read is still an error, and now says which file, why, and how to get past it — **the text of a refusal that stood**, its condition unchanged. `save`: its body moved into `save_locked`, under the same lock, with the same merge and the same persist — **what is written is what was written** |
| `unmanage` | `unmanage::run` (`crates/cli/src/unmanage.rs:60`) | the same `load`, at `unmanage.rs:70`, and `save`, at `unmanage.rs:90` where something was dropped; `render::unmanage` (`crates/cli/src/render.rs:5020`) | as above; and the text ack for a drop with no file on disk gained a sentence — **what the command prints** |
| `workflow` | `start::reenter_in_repo` and `start::preview_in_repo` (`crates/cli/src/start.rs:2906`, `:1452`) | `start::compose_task_workflow`, `start::compose_core`, `render::composed`, and the new `render::CodeOnlyCheckout`'s functions with `repo::home_is_a_checkout` (`crates/cli/src/repo.rs:167`); `state::record_doc_provenance` (`crates/engine/src/state.rs:1065`) | one field of what is composed, `code_only`, filled and printed as a block above the step text — **what the command prints**; a new **read** of git, two `git rev-parse` calls; and a new parameter of the provenance writer which this caller passes as `None`, so the manifest's bytes are what they were. The refusal `compose_core` gained stands in an arm a preview does not take |
| `task amend` | `start::amend_in_repo` (`crates/cli/src/start.rs:429`) | `start::compose_minted_in_repo`, `render::composed`, the same `CodeOnlyCheckout` functions and read, the same `record_doc_provenance` | as `workflow`: what is printed, a read, and an inert parameter. The new refusal is not reached: an amend composes through `compose_minted_in_repo` |
| `task bind` | `task::run_bind` (`crates/cli/src/task.rs:1614`) | **none** | — the functions read hold no changed line. The script's one edge is `head()`, a closure local to `doc::address_parse_guidance`, which shares its name with `setup::DirtyPaths::head` |
| `config insert-step` | `config::run_insert_step` (`crates/cli/src/config.rs:1695`) | **none** | — `config.rs` has four hunks, all in `reject_repoint` (`:625`) and `repoint_failed` (`:670`), which only `config::run_set` calls |
| `config replace-step` | `config::run_replace_step` (`config.rs:1775`) | **none** | — |
| `config remove-step` | `config::run_remove_step` (`config.rs:1843`) | **none** | — |
| `config fill` | `config::run_fill` (`config.rs:1893`) | **none** | — |
| `config fork` | `config::run_fork` (`config.rs:1939`) | **none** | — |

**How far `FileStateRecord::load` and `::save` reach**, since the two leaves' count turns on it. The six leaves of this table that reach no changed function reach neither, which the builder checked; so neither is a function every write leaf reaches, and `ingest` and `unmanage` are touched under the plan's rule. Beyond that, as the helper read the call sites of `load` outside tests and the builder did not check: 21 of the 36 write leaves reach it — 22 with `start`'s bare form — and 14 reach neither function: `workflow`, `setup`, `uninstall`, `migrate`, `task amend`, `task bind`, the five `config` leaves that are not `config set`, and `milestone provision`, `execute` and `join`.

## 4. What every write leaf reaches

`fn main` (`crates/cli/src/main.rs:33`) runs around every command's dispatch, and it changed: it now carries the invocation log's write policy beside the command's outcome. With it changed `invocation_log::enabled_logs_dir`, the new `LogWrite::for_leaf` — *append only* for `uninstall`, *mint* for every other leaf — and, reached only while the log's knob is on, `log_invocation` and `append_record`, which makes the log's directory with `create_dir` where it used `create_dir_all`. That is what the first reading counts, and it is why it comes to 36.

**A sentence of the plan found false against the tree, and left as it stands there.** The plan's §1.4 says its rule *"leaves out the 61 changed functions all 36 leaves reach"*. Six leaves' handlers reach no changed function at all (§3), so no changed function is reached from inside all 36 handlers: the 61 are what the script's graph by name gives, and a name two functions share is an edge it follows. What all 36 share is `main` and the functions it calls, above.

## 5. Not established

- **The 26 leaves the script and this reading both count touched, under the narrowest reading.** For each the script gives a number of changed functions and the nearest one; whether any of those changes moves a byte the leaf writes or a refusal it takes was read for none of them here. For the four leaves of the pilot's acceptance — `setup`, `uninstall`, `task finalize`, `milestone finalize` — the acceptance itself shows one: each refuses on this tree where the previous release acted ([byte-table-findings.md](byte-table-findings.md) → §3).
- **Edges no reading by name sees**: a call through a generic parameter's method was not resolved. The two changed trait implementations that are called implicitly — `Drop` for the boundary's dedicated worktree, and a conversion on the create path — are on none of the ten leaves' paths.
- **What a manifest that already carries the provenance writer's new member round-trips to** on `workflow` and `task amend`: established is only that this caller passes `None`.
- **Nothing was run for this count.** Every statement is from reading the tree and the diff.
