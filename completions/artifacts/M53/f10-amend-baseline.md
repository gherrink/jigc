## F‑10 baseline — what an amend would have to stand on

Verified at `834772b6`, driven on the **installed** `1.0.0-rc.19` via `rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`. Repro roots: `/var/folders/.../jigc-rig-fresh-6uXgtO/repo` (task arm), `/private/var/.../jigc-rig-fresh-CGDtTK/repo` (milestone arm). A map at this sha, not gospel.

---

## 1 · What survives a landed `task finalize` — exactly

Drive: `start --workflow quick-fix "fix the readme typo"` → `doc set-field #type` / `set-slot #summary` / `add-item #trailers` + `set-field …/value` → `git add README.md` → `task finalize`.

| Artefact | After finalize | Evidence |
|---|---|---|
| `.jigc/tasks/<id>/` (base.json · intent · workflow · staged-snapshot.json · docs/`commit:<id>.md` · docs/provenance.json) | **Gone, whole tree** | DRIVEN — `find .jigc -type f` before: 6 task files; after: `.jigc/{.gitignore,AGENT.md,config/manifest.yaml,index/edges.json.lock,logs/invocations.jsonl,state/file-state.json,version}`. `ls .jigc/tasks` empty. |
| `.jigc/state/file-state.json` | Survives — **`{path → content-sha256}` only**. No task id, no commit sha, no doctype. | DRIVEN — `{"hashes":{".jigc/config/manifest.yaml":"7007be…","README.md":"dcd093…"}}` |
| `.jigc/index/edges.json` | **Never created** on this corpus (only `edges.json.lock`). Edges are forward-refs between *committed* docs; a transient `commit` doc is never committed. | DRIVEN |
| `renames.json` (M51 Inc 11 T1) | Lives at `.jigc/tasks/<id>/renames.json` — a `TASK_AREA_FILES` member → **dies with the area**. `find .jigc -name renames.json` → none. | DRIVEN + READ (`engine::state::TASK_AREA_FILES`, 14 members) |
| Invocation log | Records `{timestamp, argv, exit_code, duration_ms, finding_codes, output_bytes, binary_version, error_code}`. **`binary_version` yes (`1.0.0-rc.19`). No sha. No task id field** — the id is only inside `argv`. Knob `invocation-log` defaults **OFF**. | DRIVEN (record verbatim) + READ `crates/cli/src/invocation_log.rs:12–19` |
| The commit itself | Message = `docs: corect the readme typoo\n\nCo-Authored-By: …`. **No jigc trailer, no marker, no task id.** `%(trailers)` carries only what the author added. | DRIVEN |
| Milestone sub-task | The committed `milestone-record` **survives** at `docs/milestone-records/<id>.md` with `status: joined` and per-task `task-id` / `intent` / `workflow` / `status`. **No sha anywhere in the schema.** `.jigc/milestones/<id>/` cache removed at the join. | DRIVEN (record file + `find`) + READ `packs/methodology/schemas/milestone-record.yaml` |

**Is there any persisted link from HEAD's sha back to the task id / commit doc? No — not one.** The only durable id→work link is the milestone record (task id ↔ milestone ↔ workflow), and it names no commit. The boundary ack prints `committed.commits[].hash` and `committed.sub_tasks[].hash` and **persists neither** (READ `design/command-output-contract.md` → `jigc milestone finalize`; DRIVEN envelope).

**And task ids are reused.** After the finalize I re-ran `start --workflow quick-fix "fix the readme typo"` → **minted `fix-the-readme-typo` again**, `base.json` pinned at the current HEAD, exit 0 (DRIVEN). A task id therefore does not identify a commit even in principle — it is not a key.

**Read surfaces for a landed commit doc dead-end honestly:**
- `jigc doc show commit:fix-the-readme-typo` → exit **1**, `store.transient-type`, route `… --task <task-id>` (DRIVEN)
- `jigc doc show commit:fix-the-readme-typo --task fix-the-readme-typo` → exit **1**, `finalize.no-task`, route `jigc task list` (DRIVEN)
- `jigc task list` → `no active tasks` (DRIVEN)

That chain is law-2 clean and is the F‑10 symptom on the read side: every route is followable and the last one answers *nothing here*.

### Can the rendered message be parsed back into a conformant commit doc?

**No parser exists.** `command grep -rniE "parse.*commit.?message|conventional.?commit"` over both crates returns only `render_commit_message` and composed-prose hits (READ). `write::instance_from_source` parses the **doc markdown**, not the git message.

**And a parser would be wrong, not merely missing — the render is lossy and ambiguous:**

- **Ambiguity, DRIVEN.** A body whose last paragraph is `Refs: this-looks-like-a-trailer` lands as
  ```
  fix: a second fix

  Motivation prose here.

  Refs: this-looks-like-a-trailer
  ```
  (`od -c` verbatim). The doc had **zero** trailer items. Any parser splitting on the last blank-line-delimited `key: value` block re-authors that prose into `#trailers`. Round-trip render → parse → render is **not byte-equal** for this shape.
- **Loss, READ.** `render_commit_message` (`crates/engine/src/write.rs:2733`) reads exactly `type`, `scope` (header), `summary`, `body`, `trailers`. The commit schema's header also declares **`implements: {type: ref, to: spec, card: "0..1", inverse: implemented-by}`** — **never projected into the message**, so it is unrecoverable from HEAD. The H1 (`# <task-id>`) and the trailer items' `{#id}` anchors are likewise unprojected.
- **Under `finalize.fan-out.squash: true` (the default) there is no commit doc behind the commit at all.** DRIVEN: with a sub-task commit doc authored `docs` / *"tidy the readme"*, the landed boundary message was `Finalize milestone test-milestone (1 sub-task)\n\n- tidy-the-readme` — the **CLI-synthesized** structural message (READ `engine/finalize.rs:990`). Only the `squash: false` chain arm renders authored commit docs (`render_subtask_messages`).

---

## 2 · The seams an amend would stand on

| Seam | State | Evidence |
|---|---|---|
| `engine::write::render_commit_message` / `trailer_lines` | Pure `(schema, instance) → String`, no disk. Directly reusable for an amend's message render. | READ `write.rs:2733/2796` |
| `task finalize` transaction | posture → validate → render (phase 3) → promote (copy) → stage → `git commit -F` → post-commit → teardown. `--approve` / `--dry-run` / `--carry-staged`; **no `--amend`**. | DRIVEN `--help`; READ `design/finalize.md` |
| `COMMITTING_DOORS` | **10 rows** (`milestone finalize` is two arms). Read by **17 test files** (`commit_rejected_axis`, `hook_output_axis`, `rejection_frame_outcome`, `repo_posture`, `help_truth`, `format_json_success_axis`, `reject_document_axis`, `finalize_family_registry`, `invocation_log`, `rollback_population_registry`, `config_layer_preimage`, `migrate_source_rules`, `migrate_route_family`, `count_fences`, `flow47/48_acceptance`, `support/committing_doors`). Each row owes a `commits:` help clause fenced through the real binary. | READ (counted) |
| `ERROR_CODE_REGISTRY` | **11 members**, mirrored member-for-member **and by count word (“eleven”)** in `design/surface-contract.md` → *The error-code namespace*, fenced by `count_fences.rs`. | READ |
| `count_fences.rs` `HOMES` | **9 homes** state the committing-door count: `CLAUDE.md`, `implementation/decisions-pending.md`, `MIGRATING.md`, `design/worked-examples.md`, `design/command-output-contract.md`, `design/surface-contract.md`, `flow47_acceptance.rs`, `commit_rejected_axis.rs`, `pinned_facts.rs`. | READ |
| `BEHALF_DOORS` / `VERB_KINDS` | Both **47 rows**, each a **bijection with the clap leaf tree** (`every_leaf_verb_is_classified`, `every_leaf_verb_says_what_it_acts_on`). `BEHALF_DOORS`: 10 `CommitsOnBehalf` · 2 `MovesOnBehalf` · 35 `Neither`. `COMMITTING_DOORS ⊆ commit-on-behalf` is **asserted**. A new leaf reddens three fences until classified, and owes a runnable `argv` checked against the real clap tree. | READ (counted) |
| `ENVELOPE_ARMS` | **64 rows** (57 `Pinned`, 7 `Unpinned`), keyed `(path, arm)`, fenced by four proofs in `format_json_success_axis.rs` incl. *every clap leaf owns ≥1 row* and *driven key set == declared key set*. A new leaf **must** declare arms. | READ (counted) |
| **The pre-pin window** | **OPEN.** *"It ends **at the 1.0 pin**"* — not at a wave. Version is `1.0.0-rc.19`; the 1.0 call has not been taken. Additive keys, pre-pin removals, and (three declared precedents) reshapes are all admissible **now**, each declared in its own paragraph as it ships. Post-pin the same act is a `2.0`. | READ `design/command-output-contract.md` → *Evolution posture*, lines 489–575 |
| Survivable hook-rejection frame | Per-door state-truth clause + shell-safe re-run argv + one log identity, over the committing-door axis (10 doors). | READ `invocation_log.rs`, `render.rs:2427` |
| `DESTROYING_DOORS` | **6 members**, subject = *a path removed that the door did not write*, axis `Disposition::{Refuse, Narrate, Displace}`. An amend removes **no path**; the superseded commit stays reachable via reflog. **An amend is not a destroying door under the shipped subject.** | READ `milestone.rs:3430` |
| `ROLLBACK_POPULATIONS` | **11 rows** (`FileCas` · `MintedSet` · `DoorGuard` · `Declared`), count-fenced over restore sites. | READ |
| `MINT_DOORS` | **5 doors**, each `Snapshot::Written` or `Snapshot::Exempt(<reason>)` under a source-level completeness fence. Any new minting door joins it. | READ |
| `WORK_UNIT_ID_DOORS` | **25 rows** × 4 cells, ⇔-fenced against the clap tree; each row carries its own runnable argv. Any verb taking a work-unit id joins. | READ |
| `TASK_AREA_FILES` | **14 members**, count-fenced in both crates (`task_area_writer_registry.rs` counts production `<dir>.join(<name>)` sites). A new area file reddens until registered. | READ |
| `STORE_EXIT_FLIPS` | 7 (`probe-unreliable`, `oob-rename`, `unmigrated-corpus`, `ahead-corpus`, `orphaned-instance`, `home-vacated`, `foreign-squatter`). Not touched by an amend. | READ (ids listed) |
| M43 `Route` fence | `Route::mechanical`'s argv must parse against the real clap tree; placeholders must be in the declared `DUMMY_SUBSTITUTIONS` table (14 entries). A new route with a new placeholder grows that table. | READ `route_fence.rs` |
| Posture family | `InProgress` (9 variants) refused at every commit-on-behalf door, re-probed immediately before the commit; `jigc task validate` previews it at exit 1. Transfers to an amend door unchanged. | READ |
| **`amend` in the design corpus** | `command grep -rn -- "--amend\|amend a \|amending" design/ implementation/ packs/ crates/cli/pack/` → **zero hits.** The path is entirely undesigned. | DRIVEN |

---

## 3 · What git does (all DRIVEN, on the rig, jigc's own installed hook)

| Question | Answer |
|---|---|
| Does `pre-commit` run on `--amend`? | **Yes.** Instrumented the installed hook at line 2: `HOOK-RAN: pre-commit staged=[]`. |
| Does `commit-msg` run on `--amend`? | **Yes.** `HOOK-RAN: commit-msg on .git/COMMIT_EDITMSG` on both `--no-edit` and `-F`. |
| Does the M35 rename guard fire on a message-only amend? | **No, and safely so.** The guard blocks only when a `reconciliation.rename` route's **both** paths appear in `git diff --cached --name-status` for *this* commit — and on a message-only amend that set is **empty** (driven above). |
| Does a rejected hook damage HEAD? | **No.** `pre-commit exit 1` → rc 1, HEAD `175ca1b…` before **and** after. `commit-msg exit 1` → same. `git commit --amend` is atomic w.r.t. HEAD. A survivable frame for an amend door has the easiest state-truth clause in the registry: *HEAD is unchanged.* |
| `--no-edit` vs `-F`? | Both work; both re-run both hooks; **both mint a new sha every time** (`df81392` → `392d4e2` → `175ca1b`), even with `--no-edit` and nothing staged. |
| **The hazard.** | `git commit --amend` **silently folds the entire current index into the rewritten commit.** Driven: HEAD held `README.md` (1 file); I staged an unrelated `unrelated.txt`, ran `git commit --amend -F <msg>`, exit 0 — the amended commit now carries **`README.md` + `unrelated.txt`, 2 files**. A door that shells out to `git commit --amend` without first refusing a non-empty index re-enacts `finalize.carried-staged` **on a commit that already landed**. |
| Is HEAD's push status knowable? | **Not reliably.** `git branch -r --contains HEAD` → empty on the rig (no fetched remote refs), and it is empty for *never-fetched* as well as *never-pushed*. A push guard here is a heuristic, not a fence. |
| Does jigc notice a raw amend? | **No — and correctly.** `jigc validate --format json` after the raw amend: `{"blocking_probes":[],"findings":[],"report_only":true,"scope":"store"}`, exit 0. The file-state baseline is **content-keyed**, so a message-only amend is genuinely invisible. This is the driven confirmation of the rc.14 record's claim that B1's bypass corrupted nothing. |

---

## 4 · The adapter side

- **`.jigc/AGENT.md`** (11 lines, quoted in full in my transcript) says *"`jigc` … owns every structural write — placement, cross-references, **commits**. You author only the prose."* and *"write every change back through `jigc`."* It says **nothing** about a landed commit being wrong. DRIVEN.
- **`.claude/skills/jigc/SKILL.md`** (388 lines): `grep -niE "amend|wrong|mistake|already (committed|landed)|fix the commit"` returns **one** hit, an unrelated schema-version-ahead paragraph. Its recovery guidance covers carryover, hand edits, `home-vacated`, hook rejection — **not a landed-but-wrong commit.** DRIVEN.
- **The deny floor does not mention git commit at all.** Installed `.claude/settings.json`: `allow: ["Bash(jigc:*)", "Bash(git add:*)"]`; `deny:` = `rm -rf`, `curl`, `wget`, **`git push --force`**, the secrets globs, and the two human-owned destroying doors `Bash(jigc uninstall:*)` / `Bash(jigc milestone discard:*)`. `Bash(git commit --amend…)` is **neither permitted nor denied** — it falls through to the harness default. DRIVEN.

So the product tells the agent jigc owns commits, ships no jigc path when one is wrong, and leaves raw `git commit --amend` un-denied. B1's bypass was the only move on the board.

---

## 5 · The shapes, with honest cost

Registry deltas below are counted against the numbers in §2. "Prose homes" = the 9 `count_fences.rs` `HOMES` that must move a numeral.

### (A) `jigc task finalize <id> --amend`
**Impossible as specified, and not rescuable.** The area is gone (driven), the id is **reusable** (driven — the same id re-minted at exit 0), the invocation log carries no sha and is off by default, and no record maps id → commit. There is nothing to resolve *through*. **Refuse this shape on the driven facts, not on taste.**

### (B) `jigc commit amend` — parse HEAD's message into a fresh commit doc
**Cost:** +1 leaf (`VERB_KINDS`/`BEHALF_DOORS` 47→48, `CommitsOnBehalf` 10→11) · `COMMITTING_DOORS` 10→11 · `ERROR_CODE_REGISTRY` 11→12 + the `surface-contract.md` mirror row **and its count word** · 9 prose homes · ~17 suites gain a cell · `ENVELOPE_ARMS` 64→66 (mint arm + landed arm) · `MINT_DOORS` 5→6 · `WORK_UNIT_ID_DOORS` 25→26 · **plus a commit-message parser that does not exist and cannot be correct** (driven: the body/trailer split is ambiguous; `implements` is unrecoverable; under `squash: true` there is no authored doc behind the message at all).
**Verdict: the parser is the whole cost and it is unbuildable-correct.** Reject.

### (C) `finalize` keeps a shadow of the commit doc for one amend
**Cost:** a new durable population (`.jigc/state/last-commit/` or a `TASK_AREA_FILES`-shaped survivor) — a new lifecycle (when does it expire? HEAD moves? a second finalize? a `milestone finalize`?), a new destroying-door subject (the shadow is bytes nobody wrote), a `ROLLBACK_POPULATIONS` row, and it **re-opens the M53 teardown** the current design closed. It also answers only the *most recent* finalize.
**Verdict: the most new mechanism for the least generality.** Reject.

### (D) The narrow shape — **re-author, never parse**
`jigc task amend` mints a transient area whose `base.json` pins **HEAD**, provisions an **empty** `commit` doc (identity `commit:<amend-task-id>`), and the agent authors it through the ordinary `set-field`/`set-slot`/`add-item` verbs and reads it back with `doc show --task`. `jigc task finalize <id>` sees an `amend` marker file in the area and takes its **second commit model**: render → `git commit --amend -F` with the **tree untouched**.

Why this is the right axis: it keeps the Conventional-Commits **rendering** inside the CLI. A doc-only answer hands `<type>(<scope>): <summary>` back to the LLM, which is precisely the structural op the product exists to take away.

**Cost, counted:**
- `VERB_KINDS` / `BEHALF_DOORS` 47→48 — `task amend` classifies **`Neither`** (it only mints); the commit stays at `task finalize`.
- `COMMITTING_DOORS` 10→**11** (a second arm of `jigc task finalize`, the `milestone finalize` two-arm precedent) → `ERROR_CODE_REGISTRY` 11→**12** (`finalize.amend-rejected`) → `surface-contract.md` mirror +1 row **and “eleven”→“twelve”** → 9 prose homes → ~17 suites gain a cell.
- `ENVELOPE_ARMS` 64→**66**: `task amend`'s composed `{task, text}` arm (rides §1 unchanged) + `task finalize`'s **amend-landed** arm. **Admissible: the pre-pin window is open** — but it must be declared in its own paragraph in `command-output-contract.md` as it ships, and `text_json_parity_axis` + `format_json_success_axis` must both cover the new leaf.
- `MINT_DOORS` 5→6 (`Snapshot::Written`). `TASK_AREA_FILES` 14→15 (the `amend` marker) + its writer-count fence in both crates.
- `WORK_UNIT_ID_DOORS` 25→26 (`task amend` takes no id; `task finalize` already has its row — so possibly **+0**).
- **`ROLLBACK_POPULATIONS`: +0.** Driven: a rejected amend leaves HEAD byte-identical, the tree is untouched, nothing is promoted. The transaction is degenerate — the cheapest survivable frame in the registry.
- **`DESTROYING_DOORS`: +0.** No path is removed; the superseded commit stays in the reflog.
- **New refusals (each owes a code + a route through the M43 fence):** `repo.operation-in-progress` (**existing**, transfers) · `work-unit.malformed-id` (**existing**) · **the index guard** — refuse unless `git diff --cached --quiet HEAD`, because the driven fold-in hazard is a data-loss cell, not a papercut · a HEAD-shape guard (root commit / merge commit) · and the **carryover gate must be explicitly exempt with a stated reason**, or it fires on every amend taken mid-work (an amend mints an area but stages nothing).

### (E) The smallest shape — **zero binary change**
One paragraph in the adopter guides + one line in the `task finalize` landed ack: *the commit has landed; a wrong message is repaired with `git commit --amend` on an unpushed commit, and jigc does not own that act.* Cost: **0 registries, 0 codes, 0 envelope arms, 0 count fences** — one guide-hash move under M52's one-batch rule, plus the law-1 obligation to strike AGENT.md's unqualified *"owns … commits"*.

**The strongest case for (E), stated properly, because it is genuinely strong:**
1. **The bypass corrupted nothing, driven.** `jigc validate` reads clean after a raw amend because the baseline is content-keyed. There is no integrity defect here — only a discoverability one.
2. **This is the pull-tier lens for the seventh consecutive trial.** The mechanism named in the rc.14 record is *the model is reliable at push, unreliable at pull.* The pull-tier fix for a capability that already exists (git's) is to **name it**, not to re-implement it behind a new verb.
3. **Asymmetry of reversibility.** (E) is reversible after 1.0 at the cost of a paragraph. (D) moves a count word in 9 prose homes, adds a twelfth error identity to a registry whose mirror is fenced, and adds two arms to a 64-row envelope table that **freezes at the 1.0 pin** — after which the same shape costs a versioned extension.
4. **It is the honest law-1 repair.** AGENT.md today says jigc owns commits, full stop. That sentence is *false at exactly one point*, and (E) is the fix that makes it true.

Its real weakness, stated: it concedes the determinism boundary at the one place the product's thesis is sharpest — the agent re-types `<type>(<scope>): <summary>` by hand, and jigc's own guide tells it to.

---

## 6 · Sibling cells any shape must answer (each driven or read above)

1. **Amend after `milestone finalize` under `squash: true`** — the boundary message is CLI-synthesized, **not a commit doc** (driven). An amend door minting an empty `commit` doc would let an agent replace a structural boundary message with a Conventional-Commits one. Refuse, or define the milestone-boundary message as its own re-render.
2. **Amend after `squash: false`** — HEAD is the *last* per-sub-task commit or the docs aggregate; amending it repairs one of N.
3. **Amend after a record-only commit** (`milestone create` / `add-task` / `add-from-spec` / `milestone discard` / `task discard`-of-a-sub-task, and `setup`'s install commit) — these have no commit doc at all and their messages are jigc's own bookkeeping. Refuse, and say which door made HEAD — but note **jigc cannot tell**: no trailer, no state, no log sha (driven).
4. **HEAD moved since the finalize** — jigc has no record of which commit was "ours", so every amend door is really *amend HEAD, whatever HEAD is*. State that in the verb's own name and help.
5. **A staged index at amend time** — driven: git folds it in silently. Must refuse.
6. **Amend twice** — harmless to git; each amend mints a new sha; the amend area must not survive the first.
7. **Amend when a doc was promoted** — the promoted ADR is *in* HEAD's tree. Message-only amend leaves the tree and therefore `file-state.json` valid (driven: file-state unchanged across three amends). **Content** amend is a different verb and should not be built.
8. **A pushed commit** — undetectable in practice (driven: `git branch -r --contains HEAD` empty with a remote configured). Any guard here is advisory, and must say so rather than imply a fence.
9. **Amend inside a fan-out worktree** — a sub-task worktree's HEAD is detached at the milestone base; posture already refuses detached HEAD at committing doors (read).
10. **An open task at amend time** — orthogonal, but `jigc start`'s `also open:` block and the single-active-task default both need an answer.

---

## 7 · Recommendation (the human decides)

**Build (D), not (B) or (C): a mint door that provisions an *empty* commit doc pinned to HEAD, landed through `jigc task finalize`'s second commit model — re-author, never parse.** The one fact I would defend hardest is that **(B)'s parser cannot be made correct** — I drove a body paragraph re-classifying as a trailer, and `implements` is projected nowhere — so any shape that reads HEAD's message back into a doc ships a silent-corruption class into the one surface that rewrites history. (D) needs no parser, reuses the posture family and the hook frame verbatim, adds **no** rollback population and **no** destroying door, and its two envelope arms are admissible only because the pre-pin window is open *now* — which is exactly the argument for taking it before 1.0 rather than after.

If the wave's budget is the binding constraint, **(E) is a real answer and not a cop-out** — the bypass provably corrupted nothing, the lens is discoverability, and a guide paragraph is reversible where a twelfth error identity in a mirror-fenced registry is not. What I would **not** accept is (E) *silently*: if jigc does not own this act, AGENT.md's *"owns … commits"* must stop saying it does.

Either way, three driven facts should ride into the Settle as constraints rather than be rediscovered: **`git commit --amend` folds the whole index in at exit 0**; **a rejected amend leaves HEAD byte-identical**; and **a task id is re-mintable, so it is not a key to anything.**
