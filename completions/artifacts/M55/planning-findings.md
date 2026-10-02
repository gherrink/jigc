# M55 planning — findings register

Every defect, gap and stale record the M55 planning run surfaced on 2026-10-02, driven against the rc.22 debug binary at `62c76009` (four capability-auditors, four gap-detectors, two robust-advocates, two spikes, one gate-speed measurement). **This file is the durable record until the jigc-feedback and inconsistency doctypes exist; it is a seed source for them, not a second home** — when a row is seeded, it gets a pointer to its doc and is not edited again. Status is as of planning; the Settle's decision for each row is in [settle-log.md](settle-log.md). Working evidence: [baseline-ledger.md](baseline-ledger.md), [gap-list.md](gap-list.md).

Columns: **id** (planning-local) · **what** · **evidence** · **planning disposition**.

## Defects in the binary (jigc-feedback candidates)

| id | what | evidence | disposition |
|---|---|---|---|
| F1 | **Cross-task sweep.** A code-less task's finalize commits code another open task staged *after* the code-less task started — exit 0, no warning, under the code-less task's commit message; the code task then fails `finalize.empty-commit` (only route: discard). | spike, states 1–3: dev-task open + `park-idea` report; state 3 sweeps `README.md` + `greet.sh` into `docs(ideas): …` | Fixed for the report workflows by S4 (`step:finalize-doc-only` + `finalize.foreign-staged`). Two *code* tasks open in one checkout stay a declared bound (needs per-task path claims). |
| F2 | **Silent overwrite on create.** `doc create <ty> --title X` when a committed doc already has X's slug acks `(already existed — copied in for update)`, exit 0; the next `set-slot` overwrites the earlier doc. `validate` stays clean. | baseline (fan-out) + decisions gap-detector (sequential, two `park-idea` tasks) | Fixed for reports by S7 (`doc create --new`). |
| F3 | **Misroute on slug collision.** A different title slugging to an existing id is refused `write.title-ignored`, but the route tells the reporter to `jigc doc rename` the **existing** doc. | doctypes gap-detector | Fixed with S7. |
| F4 | **False conflict-block after a pull (L1).** After a pull/merge that changed a committed doc, the next task editing it is blocked `reconciliation.conflict-block`; the task's copy already holds the pulled bytes. `start`'s absorb is in-memory only. The route says to revert the external edit — i.e. the teammate's commit. Workarounds (`jigc ingest`, or any unrelated landed finalize) are not named. | baseline concurrency auditor; capabilities gap-detector (`crates/engine/src/file_state.rs:533`, fix sketched: absorb when on-disk bytes equal the blob at the task's base pin, ~50–100 lines) | Open — Settle item (latent defects). |
| F5 | **Store-scope `validate` exits 1 after a branch switch (L2).** A doc that exists only on the other branch reports blocking `reconciliation.rename … is missing`; the route offers `jigc unmanage`, which drops the doc from the index. Task scope downgrades the same case to advisory. Deliberate M45 boundary (`file_state.rs:986-992`, pinned by `store_scope_stays_blocking_where_task_scope_is_advisory`). CLAUDE.md says store validate exits 0. | baseline + capabilities gap-detector | Open — Settle item. Hits branch-per-milestone directly. |
| F6 | **User-created git worktree (L3).** After one report lands in a worktree, `doc show` there gives `store.not-found` and every later finalize exits 3 (`reconciliation.rename … missing`); the main checkout's `validate` exits 1. Committed-store reads bind to the main checkout, the history check to the worktree's HEAD. | baseline concurrency auditor (12/12 later pairs blocked) | Open — likely a declared bound (fix = per-worktree store root). |
| F7 | **Fan-out sub-task told to run a refused door (S2).** A sub-task composed from a workflow that includes `step:finalize` is told `Run: jigc task finalize <id>`, which exits 3 `finalize.milestone-sub-task`. `render.rs:138-145` switches orientation to the milestone door; the composed `{{ cli.finalize-task }}` ref doesn't. | baseline + capabilities gap-detector | Open — Settle item. |
| F8 | **Union-resolved singleton silently loses a field (S3).** Parallel branches each appending to one singleton; a naive union resolve dropped one item's `date` (its trailing fields block went to one side); `jigc validate` exited 0. | baseline concurrency auditor | Open — record; storage.md is silent on managed docs merged by git across branches. |
| F9 | **A `set: on-create` date can be deleted silently.** Deleting `date:` by hand and committing: `validate` reports only `file-state.hash-matches`, no conformance finding (deleting `trigger:` is reported). | decisions gap-detector | Open. |
| F10 | **A defaulted field deleted by hand is omitted from `doc show` JSON**, so a client filtering on it (e.g. `status == "open"`) misses rows silently. | capabilities gap-detector | Open. |
| F11 | **`card:` on a non-ref field is accepted and ignored** (on a `code-anchor`: no card in `doc schema` JSON; a list payload is rejected `expected a string`). Loader gap. | capabilities gap-detector | Open. |
| F12 | **A string→code-anchor type change rides along unclassified** when combined with an optional-flag change: `migrate-corpus` stamps it ("3 would migrate, 0 blocked") and leaves blocking anchors; a pure type change has no transform kind (`crates/engine/src/schema_diff.rs:13-15`). | doctypes gap-detector | Open. |
| F13 | **A listed-pack doctype is silently demoted by a same-id built-in doctype** — no warning; `doc schema` shows the built-in shape. | doctypes gap-detector (copy-built binary embedding `finding`) | Open; drives the namespaced-id choice for the new doctypes. |
| F14 | **No edit gate on committed docs.** Any task can `add-item` / `remove-item` / `set-field` / `set-slot` on any committed doc regardless of its workflow's `allows-create`; only `doc create` and `doc author` are gated. | baseline (C7) | Parked as an idea (S5). |
| F15 | **`jigc setup` reverts a hand opt-out of the methodology pack** (`compose-embedded-methodology` is always rewritten `true`; no opt-out flag). | baseline router auditor | Open. |
| F16 | **Project-layer workflow shadow goes stale silently** — whole-file copy with no recorded base, no CLI writes it (delta invariant). | baseline router auditor | Open; related to the project-pack idea (S3). |
| F17 | **The rig cannot prototype a methodology doctype composed with dev** — `JIGC_PACK_DIR` disables methodology composition (`pack.rs:2183`); a listed project pack must copy `commands.yaml` + finalize/author-commit/migration-finalize steps. | capabilities gap-detector | Open; feeds the project-pack idea. |
| F18 | **`doc show --format json` on a per-instance doc has no title key**; `doc list` carries no field values — "all open findings" costs list + N shows and markdown parsing for titles. | doctypes gap-detector | Open — Settle item (read surface for the new doctypes). |
| F19 | **One created instance per doctype per task** (`write.identity-change`) — seeding N per-doc findings needs N tasks. | baseline | Accepted cost of S1 (one doc per finding). |
| F20 | **`ingest` leaves adopted files untracked**, and a later unrelated `migrate-corpus` swept two such files into its own commit. | doctypes gap-detector | Open. |

## Test-suite and tooling defects

| id | what | evidence | disposition |
|---|---|---|---|
| T1 | **Flaky assertion** `validate_previews_posture.rs:602` asserts stderr has no `"cd "`; any short SHA ending in `cd` trips it (~1 in 256 runs). | gate-speed measurement (nextest run failure: `base d80f6cd but`) | Fix in the `work/gate-speed` PR. |
| T2 | **BrokenPipe panic** — `trial_corpus.rs:520` `expect`s writing jigc's stdin; panics when jigc exits before reading. Same pattern at 143 sites; faster runs trigger it more. | gate-speed measurement | Fix in the `work/gate-speed` PR. |
| T3 | **`$TMPDIR` leak** — every gate run leaks ~249 entries (`jigc-trial-rig-fence*` dominant: 54k of ~111k entries, ~38 GB accumulated). | gate-speed measurement | Fix in the `work/gate-speed` PR. |
| T4 | `/usr/bin/git` (xcrun trampoline) costs ~11 ms per call × ~174k calls per gate; real git on PATH first breaks tree-sitter's C build unless `SDKROOT` is exported. | gate-speed measurement | Fixed by the `work/gate-speed` PR. |
| T5 | **Timing flake** `author_batch_scaling::the_batch_apply_growth_ratio_stays_within_its_stated_bound` measured a growth ratio of 6.16 against its 5.5 bound under full-suite load; passed alone and on the re-run. | R5 fold's first gate run, 2026-10-02 | Open — seed as jigc-feedback (test tooling). |

## Stale or contradicting records (inconsistency candidates)

| id | what | evidence | disposition |
|---|---|---|---|
| D1 | "Record-only door" (five fenced milestone doors) vs "record-only commit" (decisions-pending port paragraph) — one term, two meanings. | `design/finalize.md:218`, `design/team-ready-state.md:73`, `design/validation.md:755`, `count_fences.rs` RECORD_DOOR_HOMES vs `decisions-pending.md:104` | Settled by S4: the new thing is "the doc-only finalize step". |
| D2 | `pinned-by` defined as a test path in `implementation/pinning.md` §3 vs a code anchor in `ideas/finding-doctype.md`. | `pinning.md:51,106` | Settled by S6 (string, pinning.md's grammar); code-anchor parked. |
| D3 | Three finding vocabularies (port paragraph · `ideas/finding-doctype.md` · `completion-record`), "tier" meaning three things. | gap-list G9 | Open — field-set Settle item. |
| D4 | The charter's seed description is stale: three of the rc.16 23 rows are already fixed ((6,D-1), (7,A7-F3), (4,DEFECT 1) — DECISIONS 2026-09-23 batch); "rows added on 2026-09-27" matches nothing; M53's four re-reviews' tier-2/3 rows are uncounted. | read-back auditor | Open — seed Settle item. |
| D5 | `design/methodology-docs.md:108` says `remove-item` is "unwired"; it ships. | decisions gap-detector | Open. |
| D6 | `design/methodology-docs.md:31` shows deferral-ledger `kind` as `D/I`; it has been `Decision/Idea` since M41 (v2). | docs gap-detector | Open. |
| D7 | `CLAUDE.md` says the methodology manifest freeze-asserts "all ten shipped schemas"; there are eleven. | docs gap-detector | Open. |
| D8 | `crates/cli/tests/methodology_staging_contract.rs:131` hand-lists four workflows — stale. | docs gap-detector | Open. |
| D9 | `implementation/doctype-authoring.md`'s checklist names none of the hand-listed registration fences a new methodology doctype trips (14 real reds measured). | doctypes gap-detector census | Open — fixed while registering the new doctypes. |
| D10 | `design/reconciliation.md:51` "start is a pure reader" — but `start` mints a task (writes); the rationale does not cover it. | docs gap-detector | Open — bears on F4. |
| D11 | `design/worked-examples.md`'s flows index lists 16 of 54 flows. | docs gap-detector | Open. |
| D12 | release.md's `release` environment branch-policy id is stale (61320798 → 61391261). | M54 build memory | Open. |

## Machine observations (not repository defects)

- `fseventsd` at ~100% of one core for 18 days (~259 CPU-hours); unaffected by our tests; a reboot should reset it.
- Developer Tools exemption: the terminal here is Ghostty; expected gate gain ≤1%; optional.
