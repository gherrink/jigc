I drove the installed `1.0.0-rc.16` and read the cited code at the line. Findings below; every behavioural claim is marked **DRIVEN** or **READ**.

---

# M53 pre-decompose review — findings

## BLOCKING

### B1 · D1.3's safety premise is false at one of the two consumers it names — the fix introduces a new exit-3 wedge on the milestone boundary
**Settle sentence (D1.3):** *"Safe because a left-behind foreign file reaches nothing (READ by the advocate: `plan_promotions` `finalize.rs:1141` and the gate copy loop `milestone.rs:5843` both skip a stem with no `<ty>:<slug>` split)."*

**Falsifying datum.** The two consumers use **different** predicates.
- `plan_promotions` — `crates/engine/src/finalize.rs:1142`: `let Some((ty, slug)) = stem.split_once(':') else { continue; }` → skips. Claim holds. (READ)
- The gate copy loop — `crates/cli/src/milestone.rs:5855`: `let ty = address.split(':').next().unwrap_or(address);` → for a foreign `merged/docs/adr.md` this yields `ty = "adr"`, `schemas.get("adr")` is `Some` with a `location`, so `persisted == true` and `milestone.rs:5862` **copies it into the gate staging area**. (READ)

The gate then runs `conformance_for`, whose own type split is `filename.split(':').next()` over the *filename* including `.md` (`crates/engine/src/validate.rs:2218`), so the type resolves to `adr.md` → blocking `schema-conformance.unknown-type`. **DRIVEN** on the identical seam at the task door, rig `fresh`:

```
$ printf '# Not a jigc doc\n' > .jigc/tasks/tidy-the-readme/docs/adr.md
$ jigc task validate tidy-the-readme
blocking · schema-conformance.unknown-type — staged doc `adr` has type `adr.md`, which the resolved cascade does not define
rc=3
```

Today `materialize`'s unconditional `remove_dir_all(merged/docs)` wipes such a file before the gate, so the cell does not exist. D1.3 removes the wipe, and the file then survives into the gate — which runs **before** the commit and therefore before phase 7's displacement, so the milestone boundary blocks at exit 3 and the displacement that was supposed to rescue the bytes never runs. Every re-run blocks identically. The trigger set is every foreign `.md` under `merged/docs/` whose stem's first `:`-segment names a resolved doctype — which includes every colon-less name equal to a doctype (`adr.md`, `spec.md`, `vision.md`, `roadmap.md`, `changelog.md`).

**Smallest correction.** Make `milestone.rs:5855` ask `split_once(':')`, the predicate `plan_promotions` and `staged_doc_id` already use (one condition on an existing guard — inside the boundary), and restate D1.3's premise with the *three* consumers and their actual predicates rather than two. Then add the cell to D1's acceptance axis: *a foreign `<doctype>.md` under `merged/docs/` survives materialize, is not gated, and is displaced at phase 7.*

---

### B2 · D2.3 names a line shared by a door whose disposition is *Take* — the replacement silently re-decides `jigc milestone discard`
**Settle sentence (D2.3):** *"`post_commit` (`task.rs:5981`) and `cleanup_subtask_areas` (`milestone.rs:5538`) replace `remove_dir_all` with the shipped `engine::state::unwind_area(area, kind)`."*

**Falsifying datum (READ).** `milestone.rs:5538` is inside `cleanup_subtask_areas`, which has **three** callers, and one of them is the abandon path:
- `crates/cli/src/milestone.rs:4009` — `cleanup_subtask_areas(&jigc_root, &list, SubtaskComplement::Take)` (`run_discard`)
- `:5112` and `:5241` — the two landed `finalize` arms, `SubtaskComplement::Displace`

The function's own doc-comment states the rule D2.3 would break, verbatim (`milestone.rs:5500-5505`):

> *"**What happens to each area's *complement* is the caller's**… The removal itself is the same at every call; only this differs, and it differs by *door*. A disposition read off the function instead of the call would silently re-decide `jigc milestone discard`."*

Replacing the removal unconditionally means `milestone discard --force` — where `--force` **is** the operator's consent to take the foreign bytes — stops taking them, `all_gone` goes false, and the ack flips from `workbench removed` to `workbench NOT fully removed` while `prose.narrate_taken` / `foreign.narrate_taken` (`milestone.rs:4010`, `:4013`) narrate bytes as taken that are still on disk. That is a law-1 lie plus a consent the door honoured and then ignored.

**Smallest correction.** Condition the sink on the `SubtaskComplement` arm — `Displace ⇒ unwind_area`, `Take ⇒ remove_dir_all` — and say so in D2.3 with the doc-comment's sentence quoted. Add the cell to D2's axis: *`milestone discard --force` over a sub-task area holding a foreign byte still removes it and still acks `workbench removed`.*

---

### B3 · D2's check scope puts the new code in a registry that is asserted `==` a derived set with a hard count
**Settle sentence (D2, Check scope):** *"`finalize.foreign-bytes`: … joins `FINALIZE_FAMILY` and `ERROR_CODE_REGISTRY`."*

**Falsifying datum (READ).** `ERROR_CODE_REGISTRY` is not a finding-code registry. `crates/cli/src/invocation_log.rs:602-625`:

```rust
let mut declared: Vec<&str> = COMMITTING_DOORS.iter().map(|d| d.error_code).collect();
declared.push(ERROR_REVIEW_PENDING);
assert_eq!(ERROR_CODE_REGISTRY, declared, "the registry, the committing-door axis, and design/surface-contract.md mirror each other member-for-member");
assert_eq!(declared.len(), 11, "the declared vocabulary is the 10 committing doors + the review hold…");
```

Adding a member reddens **both** asserts. `invocation_log.rs:103` types it as *"the `ERROR_CODE_REGISTRY` member this door logs on a commit-phase rejection"* — an `Outcome` identity, not a `Finding`. And `crates/cli/src/task.rs:830-833` already states the rule for exactly this family: *"A door refusal, not a probe result and not a commit-phase rejection, so it joins neither `engine::result::CHECK_INVENTORY` nor `crate::invocation_log::ERROR_CODE_REGISTRY`."*

`FINALIZE_FAMILY` is correct (`render.rs:1432`; its predicate is namespace + production-constructor, ⇔-fenced by `finalize_family_registry.rs`).

**Smallest correction.** Strike `ERROR_CODE_REGISTRY` from D2's check scope. The finding still reaches the log through `render::BlockedFinding`/the findings channel, which is the shipped path `DISCARD_FOREIGN_BYTES` takes.

---

### B4 · D2.6 reuses a producer whose message and route are both fixed, and whose message is false about a sub-task
**Settle sentence (D2.6):** *"`engine::milestone::discard_sub_task_item` asks the shipped `item_is_settled(item)` before the splice and **refuses with the shipped `milestone.terminal`**, its route naming the leftover area to clear by hand."*

**Falsifying datum (READ).** `crates/engine/src/milestone.rs:1176-1191`:

```rust
pub fn terminal_milestone_finding(milestone_id: &str, status: &str) -> Finding {
    Finding::graded(Severity::Blocking, "milestone.terminal",
        format!("milestone `{milestone_id}` is `{status}` — a settled milestone is over and has no workbench"),
        Some(Location::addressed(format!("milestone:{milestone_id}"), 1, 1)),
        Some(format!("read the settled record with `jigc doc show milestone-record:{milestone_id}`; new work starts a new milestone (`jigc milestone create \"<title>\"`)").into()))
}
```

Both the message and the route are composed inside the producer; **the route is not a parameter** (unlike `no_such_task_finding`, which the record correctly notes takes one). Called for a settled *sub-task* it emits `milestone \`first-sub\` is \`joined\`` at target `milestone:first-sub` with a route pointing at a `milestone-record:first-sub` that does not exist. Law 1 twice over, plus a degenerate `(code, target)`.

**Smallest correction.** D2.6 must state that it mints a **second producer** (or parameterizes message + route) for the settled-sub-task-item condition, keeping the `milestone.terminal` *code*; and the "What M53 mints" ledger gains that producer. Today the ledger reads *"finding codes: 1"* and is silent about producers, so this arrives at decompose as an unbudgeted surface.

---

### B5 · D4's route mold is false on the very cell the row reports
**Settle sentence (D4):** *"**Route:** conclude `git commit` (*once its conflicts are resolved*, the shipped mold)…"*

**Falsifying datum (READ + DRIVEN).** The mold is a single `format!` at `crates/cli/src/repo.rs:~404`:

```rust
Some(conclude) => format!("conclude it with `{conclude}` once its conflicts are resolved, or abandon it with `{}`, then re-run this command", operation.abandon()),
```

Every existing member that supplies a `conclude_command` (`Merge`, `Rebase`, `Am`, `CherryPick`, `Revert`) is detected only in a *stopped/conflicted* state, so the qualifier is true for all five. The new member spans both cells — **DRIVEN**, git 2.54.0:

```
clean    `git cherry-pick -n <c>`      → rc 0, MERGE_MSG + AUTO_MERGE, git ls-files -u = 0
clean    `git cherry-pick -n <a>..<b>` → rc 0, MERGE_MSG + AUTO_MERGE, no sequencer/, ls-files -u = 0
conflict `git cherry-pick -n <c>`      → rc 1, MERGE_MSG + AUTO_MERGE, no CHERRY_PICK_HEAD, ls-files -u = 3
```

The charter's reported cell is the **clean** one, where there are no conflicts to resolve; the route as settled tells the operator to resolve conflicts that do not exist.

**Smallest correction.** Either give the new member a conclude phrase of its own (*"conclude it with `git commit`, which uses the pick's own message"*) or make the qualifier a per-member predicate beside `conclude_command()`. Either way D4 must stop claiming the shipped mold transfers unchanged, and name both cells in its acceptance.

*(Everything else in D4 checks out and is DRIVEN: the position-before-`UnmergedIndex` is load-bearing — a conflicted `-n` carries no `CHERRY_PICK_HEAD`; `git reset` on the conflicted cell exits 0, clears `MERGE_MSG`+`AUTO_MERGE` and takes `git ls-files -u` to 0, so `repo_posture.rs`'s route-runnability fence passes; `MERGE_MSG` is already in `MARKER_UNIVERSE` (`git_state.rs:246`) so the four new `EXPECTATIONS` rows are expressible; `GitState::in_progress` is already many-to-one (`RebaseMerge|RebaseApply → Rebase`, `CherryPick|Sequencer → CherryPick`, `git_state.rs:192-206`), so four states → one member breaks no fence, and neither `dev_rig_parity.rs:254` nor `flow53::state_producing` assumes a bijection; `posture_member_inventory.rs:160` fences both prose homes member-for-member. A clean `git revert -n` leaves `REVERT_HEAD` (DRIVEN), so the noun "an uncommitted cherry-pick" is not ambushed by revert.)*

---

## SHOULD-FIX

### S1 · D2.4's message needs a path set the return type does not carry, and the `Err` arm of the axis has no stated disposition
**Settle sentence (D2.4):** *"On `Foreign`: **one advisory `finalize.foreign-bytes` per area** — located at the area, message *kept, not taken* (the N paths, the failing move's reason…)."*

`AreaUnwind::Foreign` is a **unit variant** (`engine/state.rs:309`) — it carries no paths. The shipped precedent for this exact arm, `cli::milestone::unwind_mint` (`milestone.rs:1063-1072`), calls `mint_foreign_bytes_finding(jigc_home, area)`, which names **the area only**. So "the N paths" is a new probe (`foreign_area_paths` re-read after the unwind), not a reuse.

Worse: the record's *motivating* cell cannot be described from the displacement's failure set. The F1 advocate withdrew its pre-commit arm because *"a succeeding hook writes into the area **during** the commit, where no probe can see it"* — in that cell the displacement reports zero failures and the unwind still returns `Foreign`, so a message composed from "the failing move's reason" is empty or wrong.

And D2's axis carries `× {unwind ok · unwind faults}` while D2.4 specifies only the `Foreign` disposition. The shipped precedent has an `Err`-arm finding (`mint_unwind_failed_finding` → `milestone.unwind-failed`, `milestone.rs:1071`); D2 states nothing for it.

**Correction.** State that the finding's subject is a post-unwind `foreign_area_paths` re-read (the cell where nothing failed to move is real), and state the `Err` arm's disposition explicitly — reuse `milestone.unwind-failed`'s shape or declare it stays the self-heals note.

### S2 · D2's declared bound under-enumerates the unwind-fault cell, and its other half is a dead end D3 creates
**Settle sentence (D2, Declared bounds):** *"an `unwind_area` I/O fault can leave a whole area standing with its pin — listed as active … cleared by `task discard --force` (plain) or by hand (sub-task, D2.6)."*

`unwind_area` removes members in registry order and `BASE_PIN_FILE` is `TASK_AREA_FILES[0]` (`state.rs:130`), returning `Err` **at the point of failure, leaving the rest as found** (`state.rs:385` doc-comment). So there are two fault cells, not one: fail on the pin (pin intact → the bound as written) and fail on any **later** member (pin already gone → after D3 the area is a *residual*, resolves at no door, and `jigc task discard --force` is unavailable — the one recovery the bound names). The bound names only the first.

**Correction.** Enumerate both cells; for the pin-already-removed cell, say which surface names the area (D2.4's finding does, if it fires on `Err` — see S1) and that the recovery is by hand.

### S3 · D3 mischaracterises two of the three producers it deletes, and the third is still reachable on a legitimate area
**Settle sentence (D3):** *"This **deletes** the three code-less, host-absolute-path producers (`start.rs:2220`, `:2345`, `task.rs:1928`) — a residual never reaches the pin read."*

READ, all three:
- `start.rs:2220` and `:2345` — `"could not read the base pin for task \`{id}\`"`. **No host path.** Only `task.rs:1928` (`"could not read the base pin at {path:?}"`) carries one, plus its `"malformed base pin at {path:?}"` sibling one line down.
- More importantly, D3's own predicate is **existence, never a parse**, chosen precisely because *"`mint_task` writes the pin with a plain `fs::write`, so a parse-keyed predicate would have a torn-read window."* In that window — and on a corrupt or unreadable pin generally — a **legitimate** area passes the predicate and the pin read still fails. Deleting all three drops the handling for the exact state D3 argues exists.

**Correction.** Keep the producers, narrow the claim to *"a residual no longer reaches them"*, and fix `task.rs:1928`/`:1929`'s host path through `render::repo_relative` (law 1) rather than deleting the arm.

### S4 · D5's justification for keeping the fallback expressions is false at the line, and the acceptance predicate has no exemption row
**Settle sentence (D5):** *"the fallback expressions stay — `mint_migration_in_repo` relies on the task-side one by design."*

READ, `crates/cli/src/start.rs:212-213`:
```rust
let id = migration_task_id(doctype, source_path);
let minted = state::mint_task(&jigc_root, "", doctype, workflow_id, base, Some(&id))
```
`mint_task` takes the override branch (`state.rs:990-993`) and **never calls `mint_id`**. So `migrate` does not rely on that fallback. After D5's two guards land, the `None` callers are `mint_in_repo` (guarded at `start.rs:92`) and `add_task` (guarded by D5) — the `mint_id`/`mint_sub_id` empty→type-name fallbacks become production-dead.

Separately: D5's acceptance predicate is *"the id is derived from the caller's prose, never fabricated"*, iterated over `MINT_DOORS`. The migrate row derives its id from a **path hash**, not prose, and the reseed row takes a recorded id — so two of five rows falsify the predicate as stated. The repo's own mold for this is an `Exempt(reason)` row (`MINT_DOORS` already uses `Snapshot::Exempt`).

**Correction.** Strike the false justification; state the real reason to keep the expressions (or delete them); and give the fence its two stated exemption rows.

### S5 · D5's "tests that must move" list omits a standing assertion on the literal it replaces
`crates/cli/src/start.rs:4409` asserts `msg.contains("intent must contain at least one letter or digit")` — the exact bail D5 replaces. D5 lists only `engine/milestone.rs:4801`. Add it.

### S6 · D2 mints a shared `*.foreign-bytes` code without engaging the two recorded rules against it
`crates/cli/src/task.rs:826-828`: *"The **task** door's foreign-byte code — **its own, never a sibling door's** … the two other doors that refuse over this subject destroy different things and their routes lead different ways."* And `milestone.rs:~1080`: `mint_foreign_bytes_finding` is deliberately `DISCARD_FOREIGN_BYTES_CODE`'s **third producer**, because *"a second spelling of it would make one state answer two ways."*

D2 mints **one** new code used at **two** doors. That may well be right (the disposition and route are identical at both, and the state — *kept after a landed commit* — is genuinely not the mint-unwind state), but the record argues only from `DESTROYING_DOORS`' `codes: &[]`, which is a different field. Per *the record is rebuttable*, write the engagement: quote both rules and say why this state is one state.

### S7 · D1.2's `merged/docs/` rule must not inherit `TASK_DOCS_FILES`
**Settle sentence (D1.2):** *"under `merged/docs/` an entry the shipped `staged_doc_id` predicate recognises is jigc's, **every other entry under `merged/` is foreign**."*

`materialize` writes **only** `<ty>:<slug>.md` bodies into `merged/docs/` (`engine/milestone.rs:1909`) — no `provenance.json`. The Task branch's predicate is `TASK_DOCS_FILES.contains(name) || staged_doc_id(name).is_some()` (`state.rs:284-287`); copying it wholesale into the Milestone branch would classify a foreign `merged/docs/provenance.json` as jigc's and destroy it. State the Milestone rule as `staged_doc_id` **alone**.

### S8 · "`run_create`'s top" taken literally changes what jigc answers outside a git repository
**Settle sentence (D5):** *"**Seams:** `run_create`'s top (`cli/milestone.rs:629`) — before `guard_record_free`, `gitignore::ensure` and `mint_milestone`, so **no write precedes it**."*

`run_create`'s first two statements are `discover_repo_root(cwd).ok_or_else(|| crate::locate::not_in_repo(cwd))?` and `jigc_home_or_repo(cwd)?` (`milestone.rs:631-633`) — both reads. Placing the title guard literally at the top would make `jigc milestone create ""` outside a repository answer `write.unslugable-title` instead of the one not-in-repo answer M49 built (*"jigc outside a git repository answers once, with one text and a route"*). The stated requirement — *no write precedes it* — is satisfied after those two reads.

### S9 · D2's 18-cell axis has no zero-false-fire control
D1 carries one (*"an ordinary post-join area displaces **nothing** and `displaced` is `[]`"*). D2's axis is `{all · some · none move} × {unwind ok · faults} × {3 areas}` — every cell presupposes foreign bytes. The single most dangerous regression D2 can ship is `unwind_area` returning `Foreign` over an **ordinary** finalized area, which would leave every ordinary task area standing and mint a spurious advisory on every landed finalize. Evidence that it does not exists (`task_area_writer_registry.rs`, and I confirmed by inspection that a `start → doc create → validate` area holds only `base.json · docs/{commit:<id>.md, provenance.json} · intent · staged-snapshot.json · workflow`, all registry members — **DRIVEN**, rig `fresh`), but D2 should name the control cell for **both** kinds: an ordinary `task finalize` and an ordinary post-join milestone area (incl. `merged/` after D1.4) unwind to `Removed` and mint nothing.

---

## NOTE

- **N1 · D4 route prose.** *"The route says the pick is unstaged, not lost."* True on the clean cell. On the conflicted cell, `git reset` leaves the file carrying `<<<<<<<`/`>>>>>>>` markers in the worktree (**DRIVEN**), which is not "unstaged changes". Word the route so both cells read true.
- **N2 · D1.3's doc list.** `engine::milestone::materialize`'s own doc-comment (*"The `merged/docs/` folder is **truncated** before writing (a re-materialize is a clean rebuild…)"*, `milestone.rs:1855`) is a claim the fix falsifies and is not among D1's "docs that move". Byte-determinism itself is fine: every entry `materialize` can write is `<ty>:<slug>.md`, so a selective clear still removes every stale body (READ).
- **N3 · `FinalizeCode`'s doc-comment** (`render.rs:1385`) reads *"a **blocked**-finalize finding code"*; D2's member is a landed-arm advisory. One sentence.
- **N4 · Census completeness.** The E-table omits three production `join("tasks")` sites — `milestone.rs:2563` (`recorded_workflows`), `:4714` (`staged_by_sub_task`), `:6000`. All are already-resolved-`&Path` consumers and are covered by §1.1's stated bound, but since D3's table claims *"where membership is decided, not a list of sites"*, naming them as consumers would close the reader's obvious question.
- **N5 · D2.4's `Human` route.** It embeds arbitrary foreign filenames. M51's `fence_command_spans` only inspects backticked `git …`/`jigc …` spans (`finding.rs:1069-1080`), so a spaced path outside a command span is unfenced — but also unquoted and ambiguous. If the route names a command, render the path through `engine::finding::shell_token`.
- **N6 · Ledger.** *"What M53 mints, in full"* undercounts by at least: one route/finding producer (B4), one post-unwind probe (S1), and a widened `no_such_task_finding` message channel plus a per-door foreign-path count at the shared resolve predicate (D3). The *codes* count of 1 is right; the *producers* count is not stated and should be.

---

## What I checked and found sound (so it is not re-litigated)

- `displace_foreign_area` handles a two-level relative entry: `home.join(&entry)` + `create_dir_all(to.parent())` (`task.rs:5872-5882`) → `.jigc/displaced/<mid>/merged/docs/x.txt`. **TRANSFERS** (READ).
- `foreign_area_paths` can be made kind-matched without touching the Task path: the descent is already gated `if kind == WorkArea::Task && name == DOCS_DIR` (`state.rs:279`). **TRANSFERS** (READ).
- `staged_doc_id` is the exact inverse of `instance_filename`, and `materialize` writes bodies through `instance_path` **only** under `merged/docs/` — nothing under `merged/` outside it (`milestone.rs:1890-1912`). No materialized body is ever classified foreign. **TRANSFERS** (READ).
- `foreign_areas` (`task.rs:5713`) already dispatches `AreaKind::Milestone → foreign_area_paths(dir, WorkArea::Milestone)`, so D1.2's widening reaches `task discard` / `milestone discard` / `uninstall` at their shipped identities without new call sites. **TRANSFERS** (READ).
- D3's load-bearing premise: `base.json` is the first content write of `mint_task` (`state.rs:1005-1009`), is written by all five `MINT_DOORS` rows, is `TASK_AREA_FILES[0]` / `MILESTONE_AREA_FILES[0]`, has **no production rewriter or remover** outside `unwind_area` and `reseed_cache_from_record`'s absent-cache re-seed, and has existed since the first mint commit (`9679648a`, 2026-05-31) — so there is no pin-less legacy area. **TRANSFERS** (READ + census DRIVEN).
- `engine::milestone::add_task` returns `Result<AddedTask, Finding>` (`milestone.rs:348-353`), so D5's engine seam can carry a coded finding to the CLI. **TRANSFERS** (READ).
- `task finalize`'s landed arm carries `findings` (`ENVELOPE_ARMS`, `render.rs:6502`); `milestone finalize`'s is `Object(&["committed"])`, Pinned (`render.rs:6728`). D2.5's stderr-only split is correct. **TRANSFERS** (READ).
- D4's fixture/parity/flow53 claims — no bijection assumption anywhere (see B5's parenthetical). **TRANSFERS** (READ).

---

**Verdict: NOT READY to decompose.** Five blocking items. B1, B2 and B5 each ship a defect if built as written; B3 halts the build on a red fence; B4 ships two law-1 lies. All five are the M52 signature — *a claim that a shipped predicate, producer or registry transfers, false when read at the line.*
