## Verdict

**HOLDS WITH FINDINGS.** The deliverable's core is real and well built: *re-author, never parse* is the shipped shape; the driven index hazard is closed over its whole index axis (add/modify/delete/rename, from a linked worktree, previewed at `task validate` and `--dry-run`, shell-safe under hostile filenames); the tree, parent and author date survive byte-exact; the reflog carries the superseded sha; posture refuses at the commit arm under all 8 `InProgress`/detached members I drove with HEAD unmoved; teardown and the leftover residual rule hold; the 20 fences I named are green and the acceptance suite is 10/10.

One **HIGH** must close before the 1.0.0 call: it is exit-0 divergence behind a committing door **plus** a false green on the CI-gated verb — the class that blocked the call after RC-m50.

All drives on the DEBUG binary at `48d1d529`, `dev/jigc-rig` corpora.

---

## HIGH-1 — an amend finalize promotes a staged managed doc into the worktree, commits nothing, baselines it anyway, and then every surface lies about it

**Location:** `/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3233` (`let stage = if amend.is_some() { StagePolicy::Amend }`), `:4533` (`StagePolicy::Amend => git_commit_amend(&live, &msg_path)` — stages nothing, `gate_owner_artifacts_post_stage` skipped), `:3317` (the landed manifest forced to `(Vec::new(), Vec::new())` on the amend arm). The promote step of `try_execute_finalize_plan` still runs over `plan.promotions`.

**Driven, on `committed-singletons`:**

```
jigc task amend "doc edit probe"            # + author the commit doc
jigc doc set-slot vision:vision#thesis --from-file - --task doc-edit-probe
  → exit 0: "(copied in for update — the committed doc is now this task's
             staged copy, re-promoted at finalize)"
jigc task finalize doc-edit-probe --dry-run → "  promoted VISION.md"
jigc task finalize doc-edit-probe           → exit 0
     amended 9ae0b72 → 03f3cb8 — fix: repair with a doc edit
       the commit's tree is unchanged; only its message was rewritten
```

Then:

- `git status --porcelain` → ` M VISION.md` — **the finalize wrote it** (the tree was clean immediately before; verified in the rollback probe).
- `git show HEAD:VISION.md` → still the **old** thesis. No commit carries the edit; no clone will.
- The landed ack's manifest is **empty** — `VISION.md` is named on the forecast (`promoted VISION.md`) and then never again.
- `jigc doc show vision:vision` — the 1.0-pinned **committed**-doc read contract — returns `A NEW THESIS the amend task authored`.
- `jigc validate` → **exit 0, no `file-state` finding.** Control proving the baseline was advanced to bytes no commit carries: after `git checkout -- VISION.md` (restoring the *committed* content) `jigc validate` raises `blocking (gates at finalize) · file-state.hash-matches — on-disk content of VISION.md differs from the recorded state`. The polarity is inverted from a pre-task baseline, so the finalize baselined its own uncommitted promotion.

**The asymmetry that shows it is unintended:** on a **rejected** amend the promotion *is* rolled back — driven with a rejecting `commit-msg` hook, `VISION.md` restored and `git status` clean. The transaction rolls back a promotion it refuses to commit and keeps it when it succeeds.

**Class, and how the count was derived:** `command grep -n '(&\["doc", "[a-z-]*"\], VerbKind::' crates/cli/src/cli.rs` → **8** `Write` doc leaves. `doc create` is refused by the amend workflow's `allows-create: []`. The remaining seven share **one** copy-on-write seam — one ack string, `copied in for update … re-promoted at finalize`. Driven on 3 of them: `set-slot` (landed, above), `retitle-item` (landed: committed `docs/roadmap.md` keeps `### M-Alpha`, worktree and `doc show` both read `### M Alpha renamed`, ` M docs/roadmap.md`, `jigc validate` exit 0), `add-item` (acked the same promise). The fix point is one site, not seven.

**Smallest correction:** beside the two existing amend gates at `task.rs:3316`, refuse before `plan_finalize` promotes anything when the task stages any managed doc other than the transient `commit` — one blocking finding per staged doc, routed at re-authoring it in an ordinary task. Refusing at the write doors instead would need a seven-verb sweep for the same guarantee.

---

## MEDIUM-2 — the resume and `--format json` arms drop the `amending:` block while the step text still points at it

**Location:** `crates/cli/src/start.rs:1838` (`compose_core`) and `:2910` (`compose_task_workflow`) hard-code `amend: None`; `crates/cli/src/render.rs:2231` (`amending_block`) renders nothing for `None`; the dangling pointer is `crates/cli/pack/steps/amend-message.yaml:14`.

**Driven:**

```
jigc start --task open-amend        → exit 0; grep -c "amending:" → 0
                                      grep -c "already been pushed" → 0
                                      line 11 still reads:
    "…jigc cannot tell them apart from an ordinary task commit — read HEAD's
     subject line in the ack above and stop if it is one of them"
jigc workflow amend --task open-amend → exit 0; "amending:" → 0
jigc task amend "from a subdir" --format json
     → {"task": …, "text": …} — the block absent, the same pointer inside `text`
```

This is the settle's **only** mitigation for the cell it deliberately refused to refuse. `design/finalize.md:131`: *"the mint ack prints `HEAD`'s current subject line **above** the instructions so the reader sees what is about to be replaced."* On the resume path — the one five consecutive trials show agents taking when context is lost, and the one M48's read-back fence exists for — the block is gone and the instruction points at nothing.

**Class, derived:** `command grep -n "amend: None\|amend: Some" crates/cli/src/start.rs` → 4 sites, one `Some` (`:443`). Of the three `None`s, `compose_minted_in_repo` (`:1507`) is reached only by `jigc migrate` and by `amend_in_repo`, which overwrites it — so **2 live offenders, both driven**.

**Smallest correction:** `state::read_amend_pin` in the two re-compose paths, attaching the same `AmendTarget` (the marker already holds the full sha; the subject is one `git log -1 --format=%s <sha>`), and reword the step's sentence to name a command rather than "the ack above".

---

## MEDIUM-3 — the finalize door's left-out / dry-run / flag surfaces still describe the ordinary commit model, and one of them instructs the agent to do what the arm refuses

**Locations:** `crates/cli/src/render.rs:2288` (`left_out_lines`' header), `:2312` (`left_out_advisory`'s header), `crates/cli/src/task.rs:3218` (the unconditional pre-commit emit), the two arg doc-comments in `TaskCommand::Finalize` (`task.rs` ~`--dry-run` / `--carry-staged`), `crates/cli/src/gate_coverage.rs:206` (`fragment: "the carryover gate"`).

**Driven** (amend task, clean index, dirty tree):

```
jigc task finalize leftout-probe --dry-run
  would rewrite 2a9759a "…" → "fix: second repair"
    the commit's tree is unchanged; only its message is replaced
    left-out (unstaged/untracked — git add to include):   ← the misdirection
      README.md / staged.txt / untracked.txt
jigc task finalize leftout-probe            (landing run, stderr)
  finalize — about to commit the index; leaving out:      ← the index must be EMPTY here
    left-out (unstaged/untracked — git add to include):
```

Following that instruction — `git add h.txt` — makes the same finalize refuse at **exit 3** with `finalize.amend-index-dirty` (driven). A route the same binary refuses.

Beside it: `--dry-run`'s help says it refuses on *"the empty-commit guard"* (bypassed on this arm — `task.rs:3130`, `has_diff = true`) and on *"the carryover gate … (add `--carry-staged` to forecast the carry)"*; `--carry-staged` is driven **accepted and inert** on the amend arm both with a dirty index (refuses regardless) and with a clean one (lands), and its own arg help carries no carve-out. The composed `what's-left:` line and the coverage sentence in `task finalize --help` both name *"the carryover gate"* on an amend task, where the answering check is `finalize.amend-index-dirty` — the row's own comment already says the member *"is the index gate"*.

**Class, derived:** I read the door's whole help (`finalize_long_about`, `task.rs:208`, plus both arg doc-comments), the `carryover` row, and every `left_out_lines` consumer — `command grep -n "left_out_lines" crates/cli/src/render.rs` → 4 hits: `:2284` def, `:2308` pre-commit advisory, `:2662` forecast, `:2781` landed residual. On the amend arm **2 of 3 consumers are live** (the landed residual is inert because the manifest is forced empty at `task.rs:3317`). So **4 clause families across 3 files, 2 driven live render sites.**

**Smallest correction:** give `left_out_lines` the commit model the way `ForecastSubject` already has it, so the amend arm drops "git add to include"; add the arm's carve-out to the two arg helps; let the `carryover` fragment read the model.

---

## MEDIUM-4 — `finalize.amend-index-dirty` joined no `AMBUSH_CONTRACTS` row and bought no `CONSTRAINT_REQUIRED_TOKENS` facts

**Location:** `crates/cli/src/pack.rs:1037` (`AMBUSH_CONTRACTS`), `:1177` (`CONSTRAINT_REQUIRED_TOKENS`), declarer at `crates/cli/pack/steps/amend-message.yaml:2`.

`git diff 9fa66338..48d1d529 -- crates/cli/src/pack.rs` is **10 lines, all inside `mod tests`** — the production registries are untouched. `command grep -c "AmbushContract {" crates/cli/src/pack.rs` → 5 (one struct definition + **4 rows**), none for amend. `CONSTRAINT_REQUIRED_TOKENS` is still `[(&str, &[&str]); 6]` with no amend row.

The registry's stated source-set rule (pack.rs, `AMBUSH_CONTRACTS`' doc-comment): *"blocking codes minted by a door in the **commit-on-behalf** class … an ambush contract is a rule about what the door will refuse to do with the user's own work."* `finalize.amend-index-dirty` is blocking, minted at `cli::task::amend_index_dirty_finding` (`task.rs:89`), reached through `jigc task finalize` (`CommitsOnBehalf`), and is the direct sibling of the `Owed` row `finalize.carried-staged`. Its recorded reason for existing is *"nothing reddened when a new ambush-class contract stayed off one"* — and nothing did. Consequently the step's voluntary `states-constraints:` entry buys nothing: delete the dirty-index paragraph from `amend-message.yaml` and no fence notices.

**Smallest correction:** an `AMBUSH_CONTRACTS` row — `Owed` with a methodology-side declarer, or `Exempt("<reason>")` on `setup.dirty-install-path`'s precedent (the methodology pack has no amend step) — plus a `CONSTRAINT_REQUIRED_TOKENS` row so the declared statement buys its facts.

---

## LOW-5 — `amend.head-shape`'s locus names a task id the call would never have minted

`crates/cli/src/start.rs:180` passes `fallback_id` into `head_shape_refusal` unconditionally. Driven over a root HEAD with an intent supplied:

```
jigc task amend "root probe"
  blocking · amend.head-shape — … HEAD is a root commit … nothing was minted
    at: work-unit:amend-b219d05        ← the id would have been `root-probe`
```

The function's own doc-comment (`start.rs` ~156) claims *"both are read before the shape gate so the refusal can key its target on the id the mint **would** have taken"* — false for the intent-supplied branch. **Class: 1 producer, 1 call site** (`command grep -n "head_shape_refusal" crates/cli/src/start.rs` → 2 hits, definition + call). Instance, bounded at 1.

---

## LOW-6 — the acceptance suite overstates its posture coverage

`crates/cli/tests/task_amend.rs:159-161`: *"…rather than to re-run `posture_door_axis`, which sweeps all seventeen against every acting door **and now includes this arm through `BEHALF_DOORS`**."* Read: `command grep -c amend crates/cli/tests/posture_door_axis.rs` → **0**. That suite filters `BEHALF_DOORS` to acting rows (`runnable_argv`, `posture_door_axis.rs:175`) and the new row is `task amend` = `Neither`; its `task finalize` row is driven over an **ordinary** task. The amend arm's posture coverage is the 4 `AMEND_TABLE` rows, not seventeen. (Behaviour holds — I drove merge, squash-merge, unmerged-index, uncommitted-pick, rebase-merge, revert, bisect and detached; all 8 refuse at the finalize arm, HEAD unmoved. Only the claim is wrong.)

---

## LOW-7 — two settle rows are not real at the mint door

- *"repository posture … transfers unchanged at **both** doors"* — driven, `jigc task amend` mints at exit 0 under all 8 members. `design/finalize.md:265` records the truth (*"the mint door is `BEHALF_DOORS`' `Neither` and adjudicates none of it"*), so the design home is right and the settle row is stale — but the settle is what the workflow names as this feature's design of record, and `head_shape_refusal`'s own doc-comment articulates the principle it does not apply here (*"a mint that cannot land is a task the agent must then discard"*).
- *"`task amend` from a worktree … the ack names the checkout when it is not the workbench's"* — driven from a linked attached worktree, the **mint** ack names no checkout; the `committed/would commit in the linked worktree at …` line appears only on the forecast and the landed ack.

---

## LOW-8 — "only its message was rewritten" omits the committer rewrite

`crates/cli/src/render.rs:2411` (`landed_summary`'s amend lead) and its forecast twin at `:2368`. Driven: author date preserved (`AD=Sat Sep 26 22:06:56 2026` before **and** after), committer date reset (`CD=…22:06:56` → `…22:07:40`); `git commit --amend` also resets the committer *identity* to the running user. On a shared repo an amend silently re-attributes a third party's commit and no surface says so.

---

## Informational — the golden count moved and `pinning.md`'s running accounting has no paragraph yet

Measured: `find crates/cli/tests/goldens -type f -name '*.txt' | wc -l` → **646** (634 at M52's close). `git diff 9fa66338..48d1d529 --stat -- crates/cli/tests/goldens` → exactly **24 files: 12 added + 12 modified** (`start--amend--*` ×6, `workflow-preview--amend--*` ×6 new; `agent-md` ×6 and `describe` ×6 re-blessed) — **nothing else moved**. Both new families correctly capture `exit: 1` refusals (`workflow.verb-routed`), which I confirmed live. `DECISIONS.md` accounts for `+12 / 12 re-blessed`; the arithmetic reconciles (dev 17→18 × 2 × 6). `implementation/pinning.md`'s running fixed-point paragraph still ends at 634 — a close-increment obligation, not a defect in this range.

**Also worth recording:** the golden harness enumerates workflows only through `workflow --preview` and `start --workflow` (`crates/cli/tests/compose_goldens.rs:286,305`), and **both refuse `amend`** — so `amend-message.yaml`'s step body, the whole surface an agent reads, is pinned by no golden. Every other workflow's body is. And `amend-message.yaml` duplicates ~35 lines of `author-commit.yaml` verbatim (heading depths, the heredoc paragraph, the schema paragraph, scope/body, read-back) with no include, against the one-home rule.

**Baseline premise falsified, stated because it was load-bearing for the settle's record-only row:** the baseline's *"No sha anywhere in the [milestone-record] schema"* is wrong — `docs/milestone-records/<id>.md:2` carries `base: <40-char sha> <short>` (driven: `command grep -rn "7012064" docs/milestone-records/` hits). I could not reach a corrupting shape (every `milestone` door commits its record on top of the base, so no CLI sequence puts HEAD at a live milestone's `base`), and amending a record-only commit is driven harmless — record intact, `base:` still a live ancestor, `milestone add-task` and `validate` unaffected. Stating the falsified premise, not a defect.

---

## Closure table — the settle's refusal table, driven

| Settle row | mint door | finalize arm | text | json | verdict |
|---|---|---|---|---|---|
| repository posture (`InProgress`, detached, unborn) | **mints, exit 0** (8 members driven) | refuses; HEAD **and tree** unmoved; `repo.operation-in-progress` / `repo.head-detached` | ✅ | ✅ flattened | **PARTIAL** — LOW-7 (design home right, settle row stale) |
| non-empty index at finalize | n/a | `finalize.amend-index-dirty`, exit 3, one per path; add/modify/**delete**/**rename** all caught; the *worktree's own* index asked from a linked worktree; previewed at `task validate` and `--dry-run`; hostile filenames single-quoted correctly | ✅ | ✅ envelope, `(code,target)` | **HOLDS** (a staged rename needs two rounds of the route — `--name-only` reports only the destination; the gate re-fires, no hole) |
| root / merge HEAD (+ unborn) | `amend.head-shape`, exit 1, **nothing minted** | n/a | ✅ | ✅ flattened | **HOLDS** (locus wobble: LOW-5) |
| HEAD moved between amend and finalize | n/a | `finalize.base-mismatch`, exit 3, amend-specific producer's words (driven via `milestone create`'s record commit) | ✅ | ✅ envelope | **HOLDS** |
| which door made HEAD (boundary / record-only / foreign) | no refusal by design; subject line printed above the step text | — | ✅ mint text only | ❌ | **PARTIAL** — MEDIUM-2 (absent on resume and on json; the step still says "the ack above") |
| pushed status | advisory in the mint ack + `--help` | — | ✅ mint text only | ❌ | **PARTIAL** — MEDIUM-2 |
| `squash: true` boundary / `squash: false` chain | allowed; ack printed `Finalize milestone boundary (1 sub-task)`; record + `validate` unharmed after | lands | ✅ | ✅ | **HOLDS as designed** |
| a promoted doc in HEAD's tree | — | tree untouched, file-state valid **for a doc promoted by an earlier commit** | — | — | **FAILS for a doc this task stages** — HIGH-1 |
| amend twice | fresh mint against the new HEAD | commit count unmoved | ✅ | ✅ | **HOLDS** |
| an open task at amend time | `also open:` names it (`workflow amend`), both directions | — | ✅ | — | **HOLDS** |
| inside a fan-out worktree | pins the standing checkout's HEAD | detached refused at the commit arm | ✅ | ✅ | **HOLDS**; mint-ack checkout naming not real (LOW-7) |
| `--dry-run` | — | `would rewrite <sha7> "<old>" → "<new>"` + tree clause; dirty-index probe included; `task validate` previews the same | ✅ | ✅ `subject` | **HOLDS**; its left-out wording is MEDIUM-3 |

**Also driven clean, so triage need not re-drive:** happy path (new sha, `committed.amended` = superseded short sha, tree byte-identical, parent unchanged, message rendered from the doc down to its `Refs:` trailer, area torn down, `task list` empty, `jigc validate` clean); `finalize.amend-rejected` in the invocation log under a **`commit-msg`** hook with HEAD byte-identical and the promotion rolled back; `task discard` of an amend task (no commit, HEAD unmoved); `task diff` / `doc list --task`; `--format json` from a subdirectory; the second mint refused `task.serial-collision`; the teardown-fault residual (`base.json` first → `task list` clean, `finalize.no-task` with the leftover route); empty-subject HEAD → `(no subject line)`; no-setup and outside-a-repo answers; `start --workflow amend` and `workflow amend --preview` both refuse `workflow.verb-routed` with a runnable route; `task finalize --help` carries the amend `commits:` clause; `describe` lists the workflow off-catalog with its reason; and 20 named fences green (`count_fences`, `gate_coverage_fence`, `format_json_success_axis`, `text_json_parity_axis`, `task_area_writer_registry`, `mint_doors`, `suppression_fence`, `help_truth`, `compose_goldens`, `commit_rejected_axis`, `rejection_frame_outcome`, `work_unit_id_axis`, `posture_door_axis`, `freeze_enforcement`, `ref_target_fence`, `pre_dispatch_faults`, `mint_door_base_pin`, `git_span_aim`, `reject_document_axis`, `dev_rig_parity`) plus `task_amend::` 10/10.
