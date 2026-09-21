<!-- M52 per-axis review (re-run) — axis 6 · pinned contracts — RECONCILED · driven on the installed `jigc 1.0.0-rc.16` (built from commit a3eb026b), 2026-09-21. -->

<!-- M52 per-axis review RE-RUN — axis 6 · composed surfaces · RECONCILED (driver × codex source pass) · 2026-09-21 -->

# M52 per-axis review (re-run) — AXIS 6 · composed surfaces — RECONCILED

**Inputs.** The Opus driver's table (`driver/axis6.md`, read at md5 `e1ac36f5049e858aa8fd2dca55bee123`,
stable across a 20 s re-hash before it was read) and the independent Codex source pass
(`codex/axis6-codex.md`, its prompt at `codex/axis6-prompt.md`). Both files existed and were
non-empty; no wait loop was needed.

**Binary.** `/Users/maurice/.local/bin/jigc`, asserted **`jigc 1.0.0-rc.16`** before any drive.
Every reconciliation drive ran in its own throwaway `dev/jigc-rig` repo
(`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"` — two-step
eval, `mktemp -d` roots, no teardown). Nothing was written into the working repository; no fix, no
commit, no repo edit.

**The rule applied** (`acceptance-design.md` → the reconciliation rule): a claim by one side the other
cannot reproduce is a **lead**, not a finding. Every Codex claim was entered as `lead(codex, …)` and
then **driven** — to a repro or to a refutation carrying the falsifying datum. Every driver defect was
**re-driven once by me** on the same binary; all four reproduce.

**Demotions: none.** Every row in §3 of the driver table states a literal argv and an observable
(exit code, a grep count, a verbatim surface line, a JSON key set); the three M51 closures (R1–R3)
and all four defects carry fenced repro blocks. The one row in §3.2 that is not driven
(`cwd-unreadable`) is **already** marked *not driven* by the driver, and §4 states the not-driven set
explicitly. My reading of the rule and the rows that came closest are in *Notes*.

---

# PART I — the driver table (unchanged)

<!-- M52 per-axis review RE-RUN — axis 6 · composed surfaces · OPUS DRIVER · driven on the installed `jigc 1.0.0-rc.16`, 2026-09-21 -->

# M52 per-axis review (re-run) — AXIS 6 · composed surfaces — the OPUS DRIVER

**Binary.** `/Users/maurice/.local/bin/jigc`, asserted **`jigc 1.0.0-rc.16`** before anything
else (`jigc --version` → `jigc 1.0.0-rc.16`). **RELEASE posture** — the `#[cfg(debug_assertions)]`
route-fence panics do not exist here. The binary carries M52's seven audit fixes
(`79e54c75 6d95756c fe8f29c4 c96137e4 b9ab6a70 1b036264`, folded at `a3eb026b`); every row below
is a fact about **that** binary.

**Method.** Every row ran in a throwaway `dev/jigc-rig` repo
(`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"` — two-step
eval, `mktemp -d` roots, no teardown, no `rm -rf` on a variable path). Nothing was written into the
working repository. **The Codex source pass for this axis was not read** (per brief); reconciliation
is a separate agent. A row is **driven** iff its argv ran on that binary and its verdict carries a
repro block or a measurable predicate stated in the row.

**Shell note carried for the reconciler.** zsh does **not** word-split an unquoted parameter, so
`jigc $argv` passes one argument; and a pipeline reports only its last command's status. Both bit
once early and every row below was re-run in the corrected form (argv spelled literally, exit read
bare).

---

## 1. The door set, derived from the code

Counts read at HEAD (`a3eb026b`) by parsing each slice literal with comments stripped — **not**
taken from the design doc's prose. The three that **moved since M51** are marked.

| registry | file | count I read | vs M51 |
|---|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1834` | **47** leaves | = |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:2003` | **47** rows | = |
| `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs:3110` | **14** rows | = |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs:2522` | **16** rows | = |
| `SLUG_DOORS` | `crates/cli/src/cli.rs:2829` | **6** rows | = |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs:2589` | **25** rows | = |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:171` | **10** rows | = |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3275` | **6** (`[&DestroyingDoor; 6]`) | **4 → 6** (M52 D3/§18) |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs:6030` | **64** rows | **60 → 64** |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:968` | **7** rows | **6 → 7** |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs:5415` | **4** codes | minted at M52 (F4) |
| `ROLLBACK_POPULATIONS` | `crates/cli/src/rollback.rs:169` | **11** rows | minted at M52 (D1) |
| `RelocateRefusal::ALL` | `crates/cli/src/relocate.rs:91` | present | minted at M52 (F5) |
| `InProgress` | `crates/cli/src/repo.rs:173` | present (9 git states via the rig) | minted at M52 (D2) |
| `Disposition` | `crates/cli/src/milestone.rs:3061` | present | minted at M52 (D3) |
| `ManifestKind::ALL` | `crates/cli/src/render.rs:1985` | **6** members | = |
| `PRE_DISPATCH_FAULTS` | `crates/cli/tests/pre_dispatch_faults.rs:145` | **3** faults × 3 phases | minted at M52 (D6) |
| `TASK_AREA_FILES` | `engine::state` (read via `crates/cli/tests/task_area_writer_registry.rs`) | present | minted at M52 (D3) |

**This axis's own door set is derived from the render seam, not from a door table** — unchanged in
shape from M51 and re-read at this HEAD:

| render site | dispatch fn | clap leaf |
|---|---|---|
| `cli.rs:1637` | `run_compose_named_no_intent` | `start` |
| `cli.rs:1660` | `run_compose` | `start` |
| `cli.rs:1682` | `run_compose_named` | `start` |
| `cli.rs:1724` | `run_resume` | `start` |
| `cli.rs:1746` | `run_reenter` | `workflow` |
| `cli.rs:1767` (`composed_preview`) | `run_preview` | `workflow` |
| `migrate.rs:76` | `dispatch_migrate` | `migrate` |
| `milestone.rs:4501` | `dispatch_execute` | `milestone execute` |

**8 render sites over 4 clap leaves**, ∪ `describe` ∪ the read verbs the read-back fence's owe-set
names (`doc show`, `doc list`, `task validate`).

**Door set = 8:** `start` · `workflow` · `migrate` · `milestone execute` · `describe` · `doc show` ·
`doc list` · `task validate`.

**Cell set = 5:** C1 the step text names a verb that answers · C2 the `resume:` line's three real
states · C3 orientation's three states · C4 the catalog one-liner matches the verb's behaviour ·
C5 the off-catalog reason is stated.

8 × 5 = **40 cells · 25 applicable and driven · 15 n/a with reasons** (the n/a set is M51's, re-derived
at this HEAD and unchanged — §3.1).

**The M52-minted registry this axis touches, and how it is iterated here.** The **`suppressed.door`
set** is the axis's own new subject: derived from both packs' workflow front matter (14 members —
`migrate-adr · migrate-arch-doc · migrate-changelog · migrate-completion-record · migrate-decisions-log ·
migrate-deferral-ledger · migrate-idea · migrate-prd · migrate-research · migrate-roadmap ·
migrate-spec · migrate-vision · milestone-execution · sub-task`) and driven **at both compose doors ×
three argv forms = 42 drives** (row 5/25). `PRE_DISPATCH_FAULTS` is crossed with this axis's doors in
§3.2. `ENVELOPE_OWED_CODES` is driven where this axis reaches it (`task bind`, `doc show`, `doc list`
— rows 40, 46). `InProgress` is driven at `task validate` (row 45) and as the **control** at the
composed doors (row 13). `ROLLBACK_POPULATIONS`, `TASK_AREA_FILES`, `RelocateRefusal::ALL` and
`DESTROYING_DOORS ▸ Disposition` do not intersect a composed surface and are **not** driven here —
they are axes 3 and 4's subject, stated in §4.

---

## 2. M51 rows: CLOSED / STILL-OPEN

**This is the re-run's first deliverable.** M51's axis-6 ledger carried **9 CONFIRMED · 1 REFUTED ·
4 OPEN**. Every CONFIRMED row is re-driven below on rc.16.

### 2.1 The three §A defects

| M51 row | status on rc.16 | the drive that says so |
|---|---|---|
| **A6-1** — a sub-task's composed surface is byte-identical inside and outside its worktree, and the milestone boundary then drops its staged code at exit 0 | **CLOSED (both halves)** | repro R1 |
| **A6-2** — `jigc start --workflow milestone-execution` composes three unrunnable `Run:` lines at exit 0, and jigc's own preview refusal routes there | **CLOSED** | repro R2 |
| **A6-3** — `implement-from-spec` over a corpus with no committed spec claims a list that is not there, then names an unanswerable step | **CLOSED (both cells, plus the dead-end tail)** | repro R3 |

**A6-1 — CLOSED.** The `resume:` line now **discriminates the posture** it is composed in, and the
milestone boundary **names the shared-checkout staged work it did not commit**.

```
# R1 · rig: fresh
jigc milestone create "Axis six probe"; jigc milestone add-task axis-six-probe --workflow sub-task "alpha work"
jigc milestone add-task axis-six-probe --workflow sub-task "beta work"
jigc milestone provision axis-six-probe        # -> "at base ae90150"   (BASE read from this line)
(cd .jigc/worktrees/beta-work && jigc workflow sub-task --task beta-work > /tmp/b.txt)   # exit 0
git checkout --detach ae90150                  # shared checkout AT the pin  => state (c)
jigc workflow sub-task --task beta-work > /tmp/c.txt                                     # exit 0
diff /tmp/b.txt /tmp/c.txt                     # exit 1 — ONE line differs, line 116:
< resume: … provisions this sub-task's write-ready docs area on first entry; YOU ARE IN this
    sub-task's worktree at `.jigc/worktrees/beta-work`, where its work happens — run it here
> resume: … provisions this sub-task's write-ready docs area on first entry; THIS CHECKOUT IS NOT
    THAT WORKTREE — run it from this sub-task's own worktree at `.jigc/worktrees/beta-work`, …
```

and the data-loss half, driven to the commit on a second fresh rig:

```
# alpha works inside its worktree; beta works in the SHARED checkout at the pin
printf 'beta work output\n' > beta-code.txt && git add beta-code.txt
jigc milestone join axis-six-probe      # exit 0 — 2 doc(s) merged
git switch main
jigc milestone finalize axis-six-probe  # exit 0
> finalized 20f3cb6 — Finalize milestone axis-six-probe (2 sub-tasks)
>   added alpha-code.txt
>   modified docs/milestone-records/axis-six-probe.md
>   2 files committed
>   sub-tasks: alpha-work: 1 doc, 1 code file · beta-work: 1 doc
>   staged in the shared checkout (not committed by this boundary — the aggregate commit is
>   built from the sub-task worktrees; these stay staged):
>     beta-code.txt                              <-- THE LINE M51 SAID DID NOT EXIST
git ls-tree -r --name-only HEAD | grep -c beta-code.txt   # 0   (unchanged — narration, not a fold)
git status --short                                        # A  beta-code.txt   (bytes survive)
```

The bytes are still uncommitted — the fix is **narration**, exactly what the cited pin
(`milestone_teardown_loss::a_landed_boundary_names_the_shared_checkout_work_it_did_not_commit`)
asserts. The *silence* is gone, which is what the row named.

**A6-2 — CLOSED.** The composition is refused at both compose doors with a code, a stable key and a
runnable route; nothing mints.

```
# R2 · rig: fresh
jigc workflow milestone-execution --preview                       # exit 1
jigc start --workflow milestone-execution                         # exit 1
jigc start --workflow milestone-execution "an intent"             # exit 1
> blocking · workflow.verb-routed — workflow `milestone-execution` is not composed by name:
>   composes a degenerate walk — zero `Spawn:` lines and an unresolved `<MILESTONE_ID>` off-verb,
>   so a router pick could never land
>   at: workflow:milestone-execution
>   route: `jigc milestone execute <milestone-id>`
jigc task list                                                    # no active tasks
```

Widened over the **whole derived `suppressed.door` set** — 14 members × 3 argv forms × 2 doors,
**42 drives, 42 refusals at exit 1, one `workflow.verb-routed` each, zero mints** (row 5/25), and
every one of the 12 `migrate-*` members' declared door **run for real** (row 33): all 12 mint at
exit 0 through `jigc migrate <path> --as <ty>`.

**A6-3 — CLOSED.** Both renders now state their empty case, and the clause is **conditional, not
state-dependent** — it is byte-identical over a populated corpus.

```
# R3 · rig: fresh (zero committed specs)
jigc start --workflow implement-from-spec "build the thing"       # exit 0
>  6 Pick the spec this work implements from the committed specs below and bind it by
>    its FULL address exactly as listed … then re-run to read its criteria
>    (NOTHING IS LISTED UNTIL A `spec` IS COMMITTED; WITH NONE THERE IS NOTHING TO BIND
>     HERE — AUTHOR ONE FIRST THROUGH THE `plan` WORKFLOW, THEN START THIS WORK AGAIN):
> 18 The bound spec's criteria … (NOTHING APPEARS UNTIL YOU BIND A SPEC ABOVE AND RE-COMPOSE,
>     AND NOTHING APPEARS WHEN THE BOUND SPEC CARRIES NO CRITERIA YET):
# populated control (same rig, after plan → doc create spec → finalize a144b13):
jigc start --workflow implement-from-spec "wire the parser"       # exit 0
> …same clause, byte-identical… followed by:  > spec:padding-rules
jigc task bind spec spec:padding-rules wire-the-parser            # exit 0 — bound
jigc start --task wire-the-parser                                 # exit 0 — criteria render:
> ### Pads to eight  {#pads-to-eight} / It pads to eight.
```

and the dead-end **tail** M51 recorded as "the last inch" is closed too (M52 F4 —
`ENVELOPE_OWED_CODES`):

```
jigc task bind spec spec:padding build-the-thing --format json     # exit 1, stdout EMPTY, stderr:
> {"schema_version":3,"findings":[{"code":"store.not-found",
>   "key":{"code":"store.not-found","target":"spec:padding"}, …,
>   "route":"`jigc doc list spec` — the committed docs this doctype has"}]}
jigc task bind spec nosuchtype:x build-the-thing --format json     # exit 1 — store.unknown-type, enveloped
```

### 2.2 The six Codex claims M51 drove and CONFIRMED

| M51 claim | status on rc.16 | the drive |
|---|---|---|
| **C1** — all composed producers use the shared seam; 8 sites / 4 leaves; the wire is `{task, text}` | **CLOSED (holds)** | row 37: all **6** render forms driven — `start` mint / `start` resume / `workflow --preview` / `workflow --task` / `migrate` / `milestone execute` — each `['task','text']`, `task: null` on the two that mint nothing, and `resume:` present on the text arm and **absent** from every JSON arm (the declared seam bound) |
| **C2** — the three resume states are expressible (id-less omits the line; top-level routes `jigc start --task`; sub-tasks route `jigc workflow <W> --task` from their worktree) | **CLOSED (holds, and now discriminates posture too)** | rows 6, 8, 21: id-less forms render **0** `resume:` lines; top-level renders `jigc start --task <id>`; the sub-task line now carries the posture clause A6-1's fix added |
| **C3** — orientation covers exactly unset / clean / active-task; an unavailable sweep is `findings: unknown — <reason>`, never `none` | **CLOSED (holds)** | rows 9–12: `state` ∈ {`unset-project`, `clean`, `active-task`}, `schema_version: 3`; `chmod 000` on the task dir → text `findings: unknown — …`, JSON `findings: null` + `findings_unavailable: "<reason>"`, **exit 0** |
| **C4** — `describe`/`doc show`/`doc list`/`task validate` all have registry rows | **CLOSED (holds)** | row 46: `describe` → `['commands','definitions','schema_version']` · `doc show vision` → `['fields','item-count','schema-version','sections','slug','type']` · `doc list` → `['docs']` · `task validate` → `['findings','schema_version']` |
| **C5** — every executable `jigc …` form in both step trees resolves to a real clap leaf | **CLOSED (holds, re-swept over a wider corpus)** | row 26: **3025 lines** of composed text (17 workflow previews + router + ingest-existing + increment + `milestone execute` + `migrate`), 30 distinct `jigc …` forms extracted, each driven `--help`: **25 real leaves, 5 prose fragments** (`jigc can manage`, `jigc checks only`, `jigc has adopted`, `jigc itself accepts`, `jigc never auto-migrates`) — the same class M51 recorded, **zero** non-leaf command forms |
| **C6** — catalog membership and narration share front matter; the fences fire | **CLOSED (holds, and the fence set has grown by one)** | row 29: all three M51 causes still redden pack-load, **plus M52's new `workflow-refs.suppressed-malformed`** door-parse fence |

### 2.3 M51's REFUTED and OPEN rows

- **C0 (REFUTED at M51 — "no grounded completeness defects found")**: not re-litigated; its three
  falsifiers are all CLOSED above, which is the honest outcome for it.
- **OPEN lead 1 — C5 is a manifest-scoped guarantee.** **STILL-OPEN, unchanged and still declared.**
  Driven: `dev/jigc-rig fresh --pack-from-dev --workflow probe-nosupp <f>` (manifest dropped) loads
  clean and `jigc describe --workflows` narrates `probe-nosupp is A probe workflow that mints no task
  and declares no suppressed block.` with **no** *"It is hidden from the router catalog: …"* clause,
  while orientation lists it **0** times. Declared at `design/surface-contract.md`:127 /
  `design/introspection.md`:90 — a wave decision, not a defect.
- **OPEN lead 2 — the JSON arm of every composed door carries no task-state lines.** **STILL-OPEN,
  declared at the seam.** Driven at row 37: `resume:` / `what's-left:` / `task scope:` /
  `create-gates:` / `also open:` exist on the text arm of all six render forms and on **none** of
  their JSON arms.
- **OPEN lead 3a — `jigc migrate` mints a task and is silent about the work already open.**
  **STILL-OPEN — and the record that made it a match is now corrected rather than the behaviour.**
  Driven with **3** tasks already live: `jigc migrate legacy/old.md --as adr` → exit 0,
  `task minted: …`, **0** `also open:` bytes, while `jigc start --workflow quick-fix "third one"` over
  the same state renders `also open: 2 other tasks were already open …`.
  `design/write-commands.md`:189 now carries a dated M52 correction naming this exact divergence,
  scoping the claim to the two `jigc start` intent forms and keeping the lead open on its own trigger
  (*the wave that next touches the `MINT_DOORS` enumeration*). **Not a defect: the surface now says
  what the binary does.**
- **OPEN lead 3b — `jigc task bind`'s bare refusals carry no code and no route.** **PARTLY CLOSED.**
  `store.not-found` and `store.unknown-type` now carry code, key, route **and** the findings envelope
  (M52 F4, driven in R3); the **malformed-address** arm (`missing ':' between type and slug`) still
  carries **no code on either arm** — see O-2, which places it inside M50's declared flatten posture
  rather than calling it new.
- **OPEN lead 3c — `describe --commands --format json` carries no argv** and **3d — the
  none-provisioned advisory is deliberately silent**: both re-read, neither re-driven further; both
  declared (`design/introspection.md`:100; `milestone.rs`'s own comment). **STILL-OPEN, unchanged.**

**Roll-up of the comparison: 9 of 9 M51 CONFIRMED rows CLOSED · 0 STILL-OPEN · 4 open leads carried
(1 now corrected in the record, 1 partly closed, 2 unchanged).**

---

## 3. The (door, cell) table — rc.16

Route kind: **M**echanical / **H**uman / **I**nformational / **none**.

| # | door | cell | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `start` | C1 | `start --workflow single-task "add a greeting helper"` → every verb its text names, run for real → `doc show commit:… --task` → `doc list --task` → `task validate` → `task finalize` | 0/0/0/0/0/0 | advisory `changelog-recording.gate-granted-unused` | M ×4 | the chain lands `92ff6f8 — feat: add a greeting helper`, `added greet.rs`, 1 file committed | matches contract |
| 2 | `start` | C1 | `start --workflow implement-from-spec "build the thing"` over a **zero-spec** corpus | 0 | none | I (the empty-case clause) | lines 9–11 carry *"nothing is listed until a `spec` is committed; with none there is nothing to bind here — author one first through the `plan` workflow"* | **matches contract — A6-3 CLOSED** |
| 3 | `start` | C1 | the same over a **populated** corpus → `task bind spec spec:padding-rules` → `start --task` | 0/0/0 | none | M | the clause is **byte-identical** and the list renders `> spec:padding-rules`; re-compose renders the criteria with their `{#id}` anchors | matches contract |
| 4 | `start` | C1 | `start --workflow fix-task "fix the thing"` → author the commit doc → `task finalize fix-the-thing` | 0/0/**0** | none | none | the body says *"Never `git commit` and never `jigc task finalize` here. The milestone's finalize…"* and *"`git add` … inside THIS worktree"*; the forbidden door **lands `9b1441f — fix: fix the thing`** | **DEFECT D-2** |
| 5 | `start` | C1/C5 | the derived `suppressed.door` set (14) × `{--workflow <X> "<i>"`, `--workflow <X>`} | 1 ×28 | `workflow.verb-routed` ×28 | M (the declared door) | one blocking finding keyed `workflow:<id>`, stable `(code, target)`, stdout empty, `task list` unchanged | matches contract |
| 6 | `start` | C2 | `start --workflow single-task …` then `start --task <id>` | 0/0 | none | M | `resume: \`jigc start --task <id>\`` on the mint **and** the resume; `task minted:` on the mint only | matches contract |
| 7 | `start` | C2 | `start --task <sub-id>` with HEAD off the pin | 1 | none (flattened, routed) | M (`jigc milestone provision <m>`) | *"task `beta-work` is pinned to base … but you're on … run `jigc milestone provision axis-six-probe` … then re-run this from that worktree"* | matches contract |
| 8 | `start` | C2 | `start --workflow router` (id-less) | 0 | none | M | `grep -c '^resume:'` = **0** | matches contract |
| 9 | `start` | C3 | `start` / `--format json` in a `bare` repo | 0 | none | M (`jigc setup`) | text *"This project isn't set up."*; JSON `{"state":"unset-project","schema_version":3}` | matches contract |
| 10 | `start` | C3 | `start` / `--format json` in a set-up repo, no task | 0 | none | M ×4 | 12-entry catalog + `Run:`/`Preview:` block; JSON `state:"clean"` + `header`/`workflows`/`next_steps`, no `tasks` key | matches contract |
| 11 | `start` | C3 | `start` / `--format json` with one live task | 0 | 2 blocking + 1 advisory | M ×4 | `Active task:` block (7 facts incl. `base:` as `{sha,short}` on the wire); JSON `state:"active-task"`, `tasks:[…]` | matches contract |
| 12 | `start` | C3 | `chmod 000 .jigc/tasks/<id>/docs` then `start` / `--format json` | 0 | none | M | text `findings: unknown — enumerating the task's code-anchor surface at "…": Permission denied (os error 13)`; JSON `findings: null` + `findings_unavailable: "<reason>"` — *unknown* never rendered as *none* | matches contract (see O-3) |
| 13 | `start` | C3 | one live task + `git bisect start` (HEAD unmoved), then `start` vs the `jigc task validate <id>` it prints | 0 vs **1** | orientation: 2 blocking + 1 advisory — **no** `repo.operation-in-progress`; validate: `repo.operation-in-progress` and none of orientation's three | M on both | orientation renders *"previews part of the finalize gate: **the repository posture finalize refuses under**, …"* while its own `findings:` omits it, and 3 of its 4 `Run:` directives refuse at exit 1 | **DEFECT D-1** |
| 14 | `start` | C4 | `start` (orientation catalog) vs `start --workflow router` (router catalog) | 0/0 | none | M | 12 vs 12 one-liners, `diff` clean after stripping the two-space list indent — one generator | matches contract |
| 15 | `start` | C4 | all 12 catalog one-liners vs the `create-gates:` each workflow composes (`workflow <w> --preview`) | 0 ×12 | none | I | `dev-task`/`quick-fix` *"recording no decision"* → **no** `create-gates:` line; `decided-task` → `decisions-log`; `single-task` *"ADRs and … changelog"* → `adr, changelog`; `record-decision`/`implement-from-spec` → `adr`; `plan` → `spec`; `project-setup` → `prd`; `architecture-documentation` → `arch-doc`; `do-research` → `research`; `form-vision` → `vision`; `park-idea` → `idea`. **12/12** | matches contract |
| 16 | `start` | C5 | `start` (clean) — the two off-catalog next-step directives | 0 | none | M ×2 | `Run: jigc start --workflow planning` / `… ingest-existing`, each with its gist, and both forms then run at exit 0 | matches contract |
| 17 | `start` | C5 | the orientation footer's `Preview:` line vs `start --workflow milestone-execution` | 0 then **1** | `workflow.verb-routed` | I vs M | the line claims *"a workflow that mints nothing has no preview — `jigc start --workflow <id>` composes it directly, and mints nothing either"*; `milestone-execution` mints nothing and is **refused**. `describe`'s sibling sentence carries the M52 carve-out; this one does not | **DEFECT D-3** |
| 18 | `workflow` | C1 | `workflow <w> --preview` for all 12 catalog workflows + `planning`/`completion`/`record-change`/`record-dogfood`/`fix-task` | 0 ×17 | none | I (banner) | every preview composes; `jigc task list` after all of them: *"no active tasks"* | matches contract |
| 19 | `workflow` | C1 | `workflow sub-task --task <id>` from the worktree → the verbs its text names | 0 | none | M | provisions the sub-task's docs area on first entry exactly as the `resume:` line claims | matches contract |
| 20 | `workflow` | C1 | `workflow fix-task --task <id>` from its provisioned worktree | 0 | none | M | composes, and its `resume:` names the worktree it is in — **the door D-2 says exists and is undeclared** | matches contract (the door), see D-2 |
| 21 | `workflow` | C2 | `workflow sub-task --task beta-work` from the **shared checkout at the pin** vs from **its own worktree** | 0/0 | none | M | `diff` of the two whole composed outputs: exactly **one** line differs, the `resume:` line, each naming its own posture | **matches contract — A6-1 CLOSED** |
| 22 | `workflow` | C2 | the same with HEAD off the pin | 1 | none | M | byte-identical to row 7 | matches contract |
| 23 | `workflow` | C4 | `workflow quick-fix --preview --format json` | 0 | none | I | `['task','text']`, `task: null`; `task list` unchanged | matches contract |
| 24 | `workflow` | C5 | `workflow {router, ingest-existing, increment} --preview` | 1 ×3 | none | M | *"workflow '<id>' mints no task, so there is nothing to preview — run it directly: jigc start --workflow <id>"*, and all three direct forms then run at exit 0 | matches contract |
| 25 | `workflow` | C5 | the derived `suppressed.door` set (14) × `--preview` | 1 ×14 | `workflow.verb-routed` ×14 | M | the refusal replaces the mints-nothing refusal M51 found routing into the degenerate compose | matches contract |
| 26 | `describe` | C1 | `describe --workflows` / `--commands` / `--format json`, each named by a composed step | 0 | none | I | non-empty projections; `schema_version` + `origin_pack` on the JSON arm; the 30 distinct `jigc …` forms across 3025 lines of composed text all resolve (25 leaves, 5 prose fragments) | matches contract |
| 27 | `describe` | C4 | `describe --commands` (31 entries) — 4 hints driven against behaviour | 0 | none | I | `milestone-provision` *"one detached base-pin worktree per sub-task"* → `git worktree list` shows `(detached HEAD)` at the pin · `milestone-join` *"commits nothing"* → HEAD byte-equal before/after · `show-doc` *"with `--task`, the staged copy"* → staged bytes served · `validate-task` *"the repository posture, content findings, carryover…"* → the posture refusal really is raised (row 45) | matches contract |
| 28 | `describe` | C5 | `describe --workflows --format json`, all 34 entries partitioned | 0 | none | I | **34 = 22 + 12**; **22/22** off-catalog entries carry `router_hidden` and the *"It is hidden from the router catalog: …"* clause; **0/12** catalog entries carry it; the non-hidden set **equals** the orientation catalog | matches contract |
| 29 | `describe` | C5 | three manufactured packs (`--workflow <id> <f> --repin`) | rig 1 ×3 | pack-load fences | M | (a) `creates-task: false` with no `suppressed:` → *suppression fence failed* · (b) selectable with `when: ""` → *catalog shape fence failed* · (c) **new at M52** `suppressed.door: jigc nosuchverb <x>` → *`workflow-refs.suppressed-malformed` … does not parse against the real CLI* | matches contract |
| 30 | `describe` | C5 | the same probe in a **manifest-less** pack (`--pack-from-dev`, no `--repin`) | 0 | none | none | `describe` narrates it with **no** reason and `router_hidden` absent; orientation lists it 0 times | **STILL-OPEN lead** (declared) |
| 31 | `migrate` | C1 | `migrate legacy/old-decision.md --as adr` → `doc author adr --from-file -` → `doc show adr:<slug> --task` → `task validate` → plain `task finalize` | 0/0/0/0/**4** | the exit-4 review hold | M (`--approve`) | *"migration review required — nothing committed …"* + the fidelity scan + the whole diff; `git log` unmoved, `git status` unchanged | matches contract |
| 32 | `migrate` | C2 | the composed output's footer, then `start --task <migrate-id>` | 0/0 | none | M | `resume:` / `what's-left:` / `task scope:`; the resume **re-feeds the persisted source** and is byte-identical to the mint composition minus the `task minted:` header | matches contract |
| 33 | `migrate` | C4/C5 | all **12** `migrate-*` declared doors run for real (`migrate legacy/src-<ty>.md --as <ty>`) | 0 ×12 | none | M | every verb-routed refusal's route is a command that exists and mints; the reason each `describe` entry gives (*"reached only through `jigc migrate <path> --as <ty>`"*) is the argv that answers | matches contract |
| 34 | `milestone execute` | C1 | `milestone execute <m>` (none provisioned) → `provision` → `workflow sub-task --task <each>` → `join` → `finalize` | 0/0/0/0/0 | none / advisory | M ×4 | 2 real `Spawn:` lines with real ids; the whole chain runs in order and lands the boundary commit | matches contract |
| 35 | `milestone execute` | C1 | the same in the **partial**-provision state | 0 | advisory `milestone.worktrees-partial` | M (`jigc milestone provision <m>`) | the advisory prints **before** the walk, names `delta-work` and its path, and says its `Spawn:` line *"would run in a working area that does not exist"* | matches contract |
| 36 | `milestone execute` | C1 | `milestone execute <m>` over a milestone with **zero** sub-tasks | 0 | none | M ×3 | the walk composes in full — *"Spawn a sub-agent per sub-task (one per `Spawn:` line below)"* — with `grep -c '^Spawn:'` = **0** and **no statement of the empty case** | **DEFECT D-4** |
| 37 | `milestone execute` + the other 3 leaves | C1/C2 | all **6** composed render forms × `--format json` | 0 ×6 | advisory on stderr where raised | M | every one is exactly `['task','text']`; `task: null` for `--preview` and `milestone execute`; `resume:` on the text arm and **never** in the JSON; `grep -c worktrees-partial` on stdout = **0** while stderr carries it | matches contract |
| 38 | `milestone execute` | C4/C5 | `describe`'s `milestone-execution` reason vs the off-verb door | 0 | none | I | the reason is honest about the **body** (the walk it would compose), but is in the present tense for a composition the binary now refuses | matches contract (see O-4) |
| 39 | `doc show` | C1 | `doc show commit:<id> --task <id>` / `doc show adr:<slug> --task <id>`, each run from the read-back line that named it, in 3 workflows | 0 ×5 | none | none (success) | the staged bytes just written, incl. the front-matter block | matches contract |
| 40 | `doc show` | C1 | `doc show commit:<id>` with **no** `--task` · `doc show vision:bogus` · `doc show vision --task no-such-task` | 1 ×3 | `store.transient-type` · `store.fixed-identity` · `finalize.no-task` | M ×3 | each names the exact recovery (`--task <task-id>` · `jigc doc show vision:vision` · `jigc task list`) | matches contract |
| 41 | `doc show` | C4 | the `show-doc` catalog hint | 0 | none | I | *"with `--task`, the staged copy of your own in-flight write"* — driven against a doc with **no** committed copy | matches contract |
| 42 | `doc list` | C1 | `doc list` · `doc list spec` (empty) · `doc list --task <id>` · `doc list --task no-such-task` · `doc list padding` | 0/0/0/1/1 | — / — / — / `finalize.no-task` / `store.unknown-type` | I + M | `id · path · state` rows in `managed`; the empty set prints *"jigc doc list — no committed `spec` docs"* + the `note:` naming the open task and the `--task` form | matches contract |
| 43 | `task validate` | C1 | `task validate <id>` from the `what's-left:` line, in 4 tasks | 0 / 3 | `schema-conformance.*`, `finalize.carried-staged`, `file-state.staged-copy` | M on every finding | exit 3 iff blocking; nothing committed | matches contract |
| 44 | `task validate` | C4 | the generated coverage clause vs `jigc task finalize --help` | 0 | none | I | the whole span is present **verbatim** in `task finalize --help` after whitespace normalisation — one generator (`whats_left_coverage()`), two surfaces | matches contract |
| 45 | `task validate` | C4 | the clause's two claims, driven: the carryover gate, and (new at M52) the repository posture | 3 / **1** | `finalize.carried-staged` · `repo.operation-in-progress` | M + H | both gates the clause claims to preview are actually raised; the posture refusal is byte-identical to `task finalize`'s | matches contract |
| 46 | `describe`/`doc show`/`doc list`/`task validate` | C4 | each door's `--format json` key set | 0/0/0/3 | — | — | `['commands','definitions','schema_version']` · `['fields','item-count','schema-version','sections','slug','type']` · `['docs']` · `['findings','schema_version']` | matches contract |

### 3.1 n/a rows (15), each with its reason — re-derived at this HEAD, unchanged from M51

| door | cell | reason |
|---|---|---|
| `describe`, `doc show`, `doc list`, `task validate` | **C2** | none of the four renders `render::task_state_lines` — they carry no `resume:` line to be in a state |
| `workflow`, `migrate`, `milestone execute`, `describe`, `doc show`, `doc list`, `task validate` | **C3** | orientation is `run_orient`, reached only by bare `jigc start`; no other leaf produces an `OrientationView` |
| `doc list` | **C4** | the 31-entry command catalog ships **no** `list-docs` ref (verified in this run's full listing), so `doc list` is the door of no catalog one-liner |
| `doc show`, `doc list`, `task validate` | **C5** | off-catalog-ness is a property of a **workflow**, not of a verb |

### 3.2 `PRE_DISPATCH_FAULTS` × this axis's doors (the M52 registry crossing)

One JSON document per cell, on its expected arm. Driven per fault × door:

| fault (fixture) | door | exit | stdout | stderr | verdict |
|---|---|---|---|---|---|
| `packs-yaml-malformed` (`.jigc/config/packs.yaml` not valid YAML) | `start` | 1 | **empty** | one doc, `{error}` | matches |
| " | `workflow --preview` | 1 | empty | one doc, `{error}` | matches |
| " | `describe` | 1 | empty | one doc, `{error}` | matches |
| " | `doc list` | 1 | empty | one doc, `{error}` | matches |
| " | `task validate <unknown id>` | 1 | empty | one doc, `{findings, schema_version}` | matches (short-circuits on the id before pack load — the declared arm) |
| `pack-resource-missing` (`JIGC_PACK_DIR` → an empty dir) | `start` | 1 | empty | one doc, `{error}` carrying `blocking · pack.resource-missing … route: restore \`config/defaults.yaml\` …` | matches |
| " | `describe` | 1 | empty | one doc, `{error}` carrying the same code for `config/knobs` | matches |
| `cwd-unreadable` | — | — | — | — | **not driven** (§4) |

---

## 4. What I did NOT drive, stated plainly

- **`cwd-unreadable`** of `PRE_DISPATCH_FAULTS`. Deleting the process cwd under a shell that has
  already `cd`-ed there is a harness manoeuvre the two-step rig eval does not support cleanly; the
  other two faults were driven at five doors each. Axis 5/6 overlap — flagged for the reconciler.
- **`ROLLBACK_POPULATIONS`, `TASK_AREA_FILES`, `RelocateRefusal::ALL`, `DESTROYING_DOORS ▸
  Disposition`, `InProgress::ALL`'s remaining eight members.** None intersects a composed surface:
  no `render::composed` producer and none of the three read verbs is a rollback, destroying or
  relocating door. `InProgress` was driven at **two** members (`merge`, `bisect`) because `task
  validate` is this axis's door and D-1 needs the cell; the other seven are axis 2's subject.
- **`migrate … --approve`.** Row 31 stops at the declared exit-4 hold (which is the cell: *nothing
  committed*). The approve arm is axis 7's.
- **C1 end-to-end for 8 of the 17 composed workflows.** I drove the full named-verb chain to a real
  outcome for `single-task`, `implement-from-spec`, `plan`, `migrate-adr`, `sub-task`, `fix-task`,
  `milestone-execution` and `router`/`ingest-existing`/`increment` (compose only, by design — they
  mint nothing). For `architecture-documentation`, `decided-task`, `dev-task`, `do-research`,
  `form-vision`, `park-idea`, `project-setup`, `quick-fix`, `record-decision`, `planning`,
  `completion`, `record-change` and `record-dogfood` I composed each and **mechanically** checked
  every `jigc <token>` against clap (row 26); their chains were **not** run to a commit.
- **C4 at `describe` for the 34 narration bodies and 27 of the 31 command hints.** Four hints were
  driven against behaviour (row 27); all 22 off-catalog reasons were driven for presence and
  partition (row 28); the rest were not.
- **`doc list`'s `orphaned` third state**, `doc schema`'s projection, and the `home-vacated` /
  `unadopted-instance` store sweeps: axis 7.
- **The `GIT_DIR` redirect and the posture family proper**: axis 2.
- **The Codex source pass for axis 6** — deliberately not read (per brief).

---

## 5. Repro blocks for the four defects

### DEFECT D-1 (MEDIUM) — `jigc start` orientation reports a live task's findings without the repository posture that blocks every door it then routes to

**Contract contradicted.** `design/command-output-contract.md`:480 (M52, D2 as amended by §4):
`jigc task validate` *"reports the one pre-commit phase a caller can resolve before finalizing, and
no other: the repository posture … which makes `finalize` refuse before it reads the task at all"*,
at exit **1**. And the M50 design of record for the orientation variant — findings come *"from the
shipped task-scope sweep **so orientation and its own `task validate` route cannot diverge**"*
(`CLAUDE.md` → M50). Driven, they diverge, and **only on the leg M52 added**: on every other leg they
agree exactly (the control below).

```
# rig: fresh
jigc start --workflow single-task "posture parity"      # exit 0 — task minted
git bisect start                                        # HEAD UNMOVED; markers BISECT_LOG BISECT_NAMES BISECT_START

jigc start                                              # exit 0
>   findings: 2 blocking, 1 advisory
>   blocking · schema-conformance.field-value-conformant  … commit:posture-parity#header/type
>   blocking · schema-conformance.required-slot-present   … commit:posture-parity#summary
>   advisory · changelog-recording.gate-granted-unused    … task:posture-parity
>   Run: `jigc start --task posture-parity`   — resume …
>   Run: `jigc task validate posture-parity`  — previews part of the finalize gate:
>        THE REPOSITORY POSTURE FINALIZE REFUSES UNDER, this task's content findings, …
>   Run: `jigc task finalize posture-parity`  — validate + commit
>   Run: `jigc task discard posture-parity --force` — abandon …

jigc start --format json > o.json   # then: python3 -c "import json;print([f['code'] for f in json.load(open('o.json'))['tasks'][0]['findings']])"
> "schema-conformance.field-value-conformant"
> "schema-conformance.required-slot-present"
> "changelog-recording.gate-granted-unused"          # repo.operation-in-progress ABSENT from the pinned wire

jigc task validate posture-parity                        # exit 1 — the route orientation just printed
> blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
>   committable state
>   route: conclude it, or abandon it with `git bisect reset`, then re-run this command
jigc task finalize posture-parity                        # exit 1 — identical refusal
```

**The control, same shape, driven — the two agree everywhere else:**

```
# rig: fresh; carry.txt staged BEFORE the mint
jigc start                    findings: 3 blocking, 1 advisory   (…, finalize.carried-staged, …)
jigc task validate carry-probe   exit 3 — the SAME 3 blocking + 1 advisory, same order
```

So the divergence is not the sweep's — it is that M52 added the posture leg **at the door** and the
front door that routes to that door did not join. Orientation answers *"2 blocking"* to a reader
whose next three commands all refuse for a third reason it never names, at exit 0.

**Severity.** Surface-tier, no loss: the posture is stated at the door where it binds, so law 3 holds
literally and the reader is not stranded. What fails is the front door's own answer to *what stands
between me and a commit*, on a pinned envelope key a driver reads.

### DEFECT D-2 (MEDIUM) — `fix-task` is composable by name, and the composed walk's forbidden door is the only one that works

**Contract contradicted.** The pack's own `suppressed.reason`
(`packs/methodology/workflows/fix-task.yaml`): *"spawned by a completion audit's Fix-phase fan-out and
joined at the milestone's finalize — **it has no commit boundary of its own, so a router pick could
never land**"*. Driven, a by-name composition lands one. And
`crates/cli/src/start.rs::read_named_workflow`'s own doc-comment: *"A workflow whose `suppressed:`
block declares a `door:` composes correctly **only** through that door, because the door is what
binds the input the body reads"* — `fix-task`'s body reads a worktree and a milestone binding, and
the workflow declares **no `door:`**.

```
# rig: fresh
jigc start --workflow fix-task "fix the thing" > ft.txt  # exit 0 — task minted: fix-the-thing
grep -n 'THIS worktree' ft.txt ; grep -n 'task finalize' ft.txt
> 18:`git add` the paths this fix touched, inside THIS worktree — your own paths and
> 25:Never `git commit` and never `jigc task finalize` here. The milestone's finalize
#   … the task is NOT a sub-task of any milestone and HAS no worktree:
jigc task list
>   fix-the-thing  [fix-task]  fix the thing
#   (no milestone: it was minted by the front door, not by `milestone add-task`, and
#    `.jigc/worktrees/` holds nothing)

jigc doc set-field commit:fix-the-thing#type --value fix --task fix-the-thing     # 0
printf 'fix the thing\n' | jigc doc set-slot commit:fix-the-thing#summary --from-file - --task fix-the-thing  # 0
printf 'x\n' > fixed.txt && git add fixed.txt
jigc task finalize fix-the-thing                        # exit 0   <-- the door line 25 forbids
> no findings — the task validates clean
> finalized 9b1441f — fix: fix the thing
>   added fixed.txt
>   1 file committed
```

**The door exists and is the sub-task shape exactly** — so the gap is the missing declaration, not a
missing capability:

```
jigc milestone create "Fix round"; jigc milestone add-task fix-round "the finding" --workflow fix-task
jigc milestone provision fix-round
jigc milestone execute fix-round | grep '^Spawn:'
> Spawn: `cd .jigc/worktrees/finding && jigc workflow fix-task --task finding`
(cd .jigc/worktrees/finding && jigc workflow fix-task --task finding)     # exit 0
> resume: `jigc workflow fix-task --task finding` … you are in this sub-task's worktree …
```

**The axis, derived rather than reported.** Partition both packs' workflows by
`(creates-task, selectable, suppressed.door)`: the `selectable: false ∧ creates-task: true ∧ no door`
set is **five** — `planning`, `completion`, `record-change`, `record-dogfood`, `fix-task`. The first
four say *invoked/reached **by name*** in their own `suppressed.reason` and composing them by name is
correct (driven: each mints at exit 0 with an intent, refuses without one, and previews cleanly).
**`fix-task` is the one member whose reason says it is *spawned*, verbatim sharing its operative
clause with `sub-task`** — which *is* declared. M52's Increment 9 closed that class over one of its
two members.

### DEFECT D-3 (LOW) — the orientation footer's `Preview:` line still states the pre-M52 rule, which `milestone-execution` falsifies

**Contract contradicted.** `design/surface-contract.md` → **Law 1 — nothing lies.** The same claim
exists on two surfaces; M52 Increment 9 swept one.

```
# rig: fresh
jigc start | grep '^Preview:'
> Preview: `jigc workflow <id> --preview`   — read a task-minting workflow's step text without
>   minting a task; a workflow that mints nothing has no preview — `jigc start --workflow <id>`
>   COMPOSES IT DIRECTLY, and mints nothing either                     (render.rs:293)

jigc describe --workflows | head -3
> … a workflow that mints nothing has no preview, and `jigc start --workflow <id>` composes it
>   directly without minting either. NEITHER REACHES A WORKFLOW WHOSE LINE BELOW SAYS IT IS
>   REACHED ONLY THROUGH A VERB … and both of these refuse it by name.        (render.rs:5681)

# the falsifier — a mints-nothing workflow that the front door refuses:
jigc start --workflow milestone-execution        # exit 1 · workflow.verb-routed
jigc start --workflow router                     # exit 0 (the control — composes directly)
```

`milestone-execution` is `creates-task: false` and carries a `door:`; it is reachable by name from
`describe`, which is the surface a reader consults after reading the orientation footer.

### DEFECT D-4 (LOW) — a legitimately-empty `milestone execute` walk does not state its empty case

**Contract contradicted.** The acceptance design's own cell for M52 arm 7 — *"a legitimately-empty
render states the empty case"* — and `design/surface-contract.md` law 1. The sibling empty
enumeration (`implement-from-spec`'s spec list) got exactly that clause at M52 (R3); this one did
not. The pack step is `crates/cli/pack/steps/implement-tasks.yaml`, whose front matter is
`fan-out: over: "{{ milestone.tasks }}"`.

```
# rig: fresh
jigc milestone create "Empty probe"              # exit 0 — record commit
jigc milestone execute empty-probe               # exit 0
> Spawn a sub-agent per sub-task (one per `Spawn:` line below) and implement it.
> Each runs in its own isolated working area keyed by task id. Wait for every
> spawned sub-task to report complete before moving on — nothing is merged yet.
>                                     <-- zero Spawn: lines, and nothing says so
grep -c '^Spawn:'   -> 0
stderr              -> empty   (no advisory: `partial_worktree_advisories` is silent for the
                                settled none-provisioned state, by design)
jigc milestone provision empty-probe  -> exit 0 "provisioned 0 worktree(s) … ()"
jigc milestone join empty-probe       -> exit 0 "0 doc(s) merged"
```

**The terminal is safe, which bounds the severity** — the walk's last verb refuses with a code and a
two-armed route:

```
jigc milestone finalize empty-probe              # exit 3
> blocking · milestone.zero-contribution — milestone:empty-probe would land no work …
>   route: `jigc milestone provision empty-probe` … — or settle the milestone as abandoned with
>          `jigc milestone discard empty-probe`
```

So the class is *the empty render does not state its empty case*, not *the empty walk lands
something*.

---

## 6. Observations and leads (driven, not defects — no stated contract is contradicted)

1. **O-1 — `jigc migrate` mints silently over open work.** STILL-OPEN; the record was corrected at
   M52 rather than the behaviour, and the lead carries its own trigger
   (`design/write-commands.md`:189). Driven in §2.3.
2. **O-2 — the malformed-address refusal carries no code on either arm.**
   `jigc task bind spec padding <task>` and `jigc doc show padding` both answer
   *"malformed address `padding`: missing ':' between type and slug … route: run `jigc describe` …"*
   — text with no `blocking · <code>` prefix, `--format json` as `{"error": …}`. This is the posture
   `design/command-output-contract.md`:294 declares for a caller-typed token that names nothing
   (*"a null-target key is strictly less informative than the flattened message"*), and the sibling
   `store.malformed-slug` **does** carry a code (driven: `jigc task bind spec 'spec:../../etc' <task>`
   → `blocking · store.malformed-slug`). Recorded because it is the last inch of A6-3's old dead end,
   and because the *missing-colon* arm is the one shape in that family with no identity at all.
3. **O-3 — the orientation `findings: unknown — <reason>` still prints an absolute host path**, on
   the text arm and inside the pinned JSON key `findings_unavailable`. Producer:
   `crates/cli/src/task.rs:1964`, `format!("enumerating the task's code-anchor surface at {:?}", …)`.
   M51 recorded this as a *declared, counted* member of `UNSWEPT_PRODUCERS`; M52 Increment 4 / T7
   rewrote that row to **2** with the reason *"one `with_context` promote fault, and the `git archive
   --prefix=`"*, which no longer names this site. The count is not wrong — the guarded-module fence
   reads `.display()` only and states the `{…:?}` half as a **declared bound** — so the site is
   inside a declared bound and **outside the reason that used to name it**. Recorded as a
   record-truth lead, not a defect.
4. **O-4 — `milestone-execution`'s `suppressed.reason` is in the present tense for a composition the
   binary now refuses.** The refusal renders it verbatim: *"is not composed by name: **composes** a
   degenerate walk — zero `Spawn:` lines and an unresolved `<MILESTONE_ID>` off-verb"*. Its twelve
   `migrate-*` siblings use the conditional (*"a router pick **would** compose with no staged
   source"*). Marginal, and the sentence is the pack's own statement of *why*, so no rule is
   contradicted — but D-4 shows the degenerate walk it describes **is still reachable at the real
   verb door**, which is where the tense is arguably right and the statement is missing.
5. **O-5 — `jigc task validate` does not preview the migration review hold.** Driven at row 31: a
   migrate task validates at exit 0 while plain `task finalize` holds at exit 4. **Declared** at
   `design/command-output-contract.md`:480 (*"the preflight's base pin, promotion, the migration
   review hold … deliberately left out of the preview"*). Not a defect; recorded so it is not
   re-derived.
6. **O-6 — C5 is a manifest-scoped guarantee.** Re-driven, unchanged (row 30). Declared at
   `design/surface-contract.md`:127 / `design/introspection.md`:90.
7. **O-7 — `jigc milestone provision` over a zero-sub-task milestone acks `provisioned 0 worktree(s)
   for milestone:<id> at base <sha> ()`** — an empty parenthesised list. Honest, and cosmetic; noted
   beside D-4 because the same state produces both.

---

## 7. What this adds over flow-53 arm 7 (and arm 6)

**Arm 7 is the arm that pins part of this axis.** `completions/artifacts/M52/acceptance-design.md` →
arm 7, *the composed doors (D9)*: it iterates **the verb-routed workflow set derived from both packs'
`suppressed` blocks × the two compose doors**, plus *the empty-enumeration set from the pack's
`{{@…}}` placeholders*, and asserts refusal with a runnable route, the empty-case statement, and the
exit-4 hold. Arm 6 (`PRE_DISPATCH_FAULTS × VERB_KINDS`, `ENVELOPE_ARMS`) pins the **wire** at every
leaf this axis uses.

This review drives the half those arms structurally cannot reach, in four ways:

1. **It derives the axis's *complement* rather than its membership.** Arm 7 iterates the set of
   workflows that **declare** a door. D-2 is a workflow that **should** declare one and does not —
   invisible to any iteration over the declared set, and reachable only by partitioning *all* 34
   workflows on `(creates-task, selectable, door, the reason's own words)` and asking which member
   of the *spawned* class is missing. The same shape for arm 7's empty-enumeration half: it iterates
   `{{@…}}` placeholders, and D-4's empty set is a `fan-out: over:` — a different placeholder family
   in the same class.

2. **It follows the prose to a landed commit instead of asserting a refusal.** Arm 7 asserts that
   `start --workflow <verb-routed>` exits 1. D-2's finding is that a workflow which *is not* in that
   set composes at exit 0 and then **lands `9b1441f`** through a door its own body forbids. Rows 1,
   3, 31, 34 do the same for the passing cells — a real commit (`92ff6f8`), a real bind-and-recompose,
   a real exit-4 hold, a real milestone boundary.

3. **It crosses two surfaces that the arms measure separately.** D-1 is `jigc start`'s finding set
   against `jigc task validate`'s finding set **in one repository state**; D-3 is `render.rs:293`
   against `render.rs:5681` **in one run**. Rows 14, 15, 28, 44 assert the same kind of agreement
   where it holds (orientation's catalog = the router's; all 12 one-liners = their `create-gates:`;
   the 34-entry partition; the coverage span = `task finalize --help`). An arm keyed per door cannot
   see a divergence between two doors.

4. **It varies the repository state under a fixed argv.** A6-1's closure is proven here the way the
   defect was found — `diff` of one argv's output in two postures, now **one line different** where
   it used to be zero. D-1 exists only because `git bisect start` changes the world without changing
   the argv or HEAD.

**Net for this axis:** M51's three defects and all six of its Codex claims are **CLOSED on rc.16**;
four new findings, all surface-tier, **zero data-loss and zero regression**; and three of the four
(D-1, D-3, D-4) are **incomplete sweeps of M52's own new work** — the wave's recurring signature,
landing here for the seventh consecutive time.

---

## 8. Doors covered

Every clap leaf that is the door of ≥1 **driven** row above, in `VERB_KINDS` spelling. A
classification-only row confers no coverage.

| leaf | rows |
|---|---|
| `start` | 1–17, 37; D-1, D-2, D-3 |
| `workflow` | 18–25, 37; A6-1 (R1), D-2 (the real door) |
| `describe` | 26–30, 38; D-3 |
| `migrate` | 31–33, 37 |
| `milestone execute` | 34–38; D-4 |
| `milestone create` | 34, 36; R1, D-2, D-4 |
| `milestone add-task` | 34; R1, D-2 |
| `milestone provision` | 27, 34, 35; R1, D-2, D-4 |
| `milestone join` | 27, 34; R1, D-4 |
| `milestone finalize` | 34; R1 (the landed boundary + its new narration), D-4 (`milestone.zero-contribution`) |
| `doc show` | 39–41, 46; R3 |
| `doc list` | 42, 46; R3 (the law-2 contrast sentence) |
| `doc schema` | 3 (`jigc doc schema spec`), 42 (`jigc doc schema padding` → `store.unknown-type`) |
| `doc create` | 3 (`doc create spec --title`) |
| `doc add-item` | 3 (`doc add-item spec:…#criteria --title`) |
| `doc set-field` | 1, 4; R1 |
| `doc set-slot` | 1, 3, 4; R1 |
| `doc author` | 31 (the migrate batch payload) |
| `task list` | 5, 18, 25; D-2 |
| `task validate` | 43–46; D-1 |
| `task bind` | 3; R3 (four refusal shapes) |
| `task finalize` | 1, 31, 44; D-2 (the forbidden door that lands) |
| `setup` | 29 (the `setup.pack-load` advisory over each manufactured pack, emitted by the installed binary at rig construction) |

**23 leaves driven to behaviour.** Every other `VERB_KINDS` leaf — `uninstall`, `upgrade`, `ingest`,
`migrate-corpus`, `unmanage`, `rename`, `relocate`, `validate`, `doc rename`, `doc remove-item`,
`doc retitle-item`, `task diff`, `task discard`, the six `config` leaves, `milestone add-from-spec`,
`milestone list-tasks`, `milestone discard` — is another axis's subject and was not reached here.

---

# PART II — Reconciliation ledger

The Codex pass for this axis reports **zero new claims** ("No grounded completeness defects found").
Its substantive content is therefore one **completeness claim**, three **M51-row dispositions**, six
**re-check assertions**, two **bounds** and one **boundary claim** — twelve claims, each entered as a
lead and driven below.

## II.a — the Codex claims

### CX-0 — "No grounded completeness defects found: no omitted door, missing composed-output state, invalid executable step command, orientation null/unknown collapse, or catalog/help contradiction that supports a new reproduction." → **REFUTED**

**Falsifying data — four, two of them landing inside the categories CX-0 names by name.**

- *"no catalog/help contradiction"* is falsified by **D-3**, driven below: the orientation footer's
  `Preview:` line states the pre-M52 rule verbatim while the binary refuses the exact form it
  prescribes.
- *"no omitted door"* is falsified by **D-2**: `fix-task` carries `suppressed: {reason, expires}` and
  **no `door:`**, while its reason is the verbatim twin of `sub-task`'s — which does declare one.
  Codex read the fence that requires `{reason, expires}` (`pack.rs:692`/`731`) and concluded the
  complement was complete; the fence does not reach the `door:` key, and the omission is only visible
  by partitioning *all 34* workflows on `(creates-task, selectable, door, the reason's own words)`.

```
# my re-drive of the partition, at HEAD
$ grep -rl '^  door:' packs/methodology/workflows/ crates/cli/pack/workflows/ | wc -l
      14
$ grep -n -A4 '^suppressed:' packs/methodology/workflows/fix-task.yaml
6:suppressed:
7-  reason: spawned by a completion audit's Fix-phase fan-out and joined at the milestone's finalize — it has no commit boundary of its own, so a router pick could never land
8-  expires: never
9----                                    <-- no `door:` key
$ grep -n -A4 '^suppressed:' crates/cli/pack/workflows/sub-task.yaml
6:suppressed:
7-  reason: spawned by a milestone execution's fan-out and joined at the parent's finalize — it has no commit boundary of its own, so a router pick could never land
9-  door: jigc workflow sub-task --task <task-id>
```

- **D-1** (orientation's finding set omits the posture that blocks all three doors it routes to) and
  **D-4** (an empty `milestone execute` walk states no empty case) are two further instances.

**Why the source read missed them, stated rather than implied.** Codex declares its own bound —
*"This was source-only: I did not execute the binary"* — and three of the four defects are invisible
to a source read by construction: D-1 needs the repository state varied under a fixed argv
(`git bisect start` changes the world without changing HEAD or the argv), D-3 is two producers
(`render.rs:293` vs `render.rs:5681`) compared **in one run**, and D-2 needs the composed prose
followed to a landed commit. CX-0 is refuted as a completeness claim, not as a reading of the files
it names.

### CX-1 — "A6-1 CLOSED: composition records its worktree posture; the milestone boundary reads the shared checkout's still-staged set." → **CONFIRMED (repro)**

```
# rig: fresh — BASE read from the provision line (see Notes: it is NOT `git rev-parse HEAD`)
jigc milestone create "Axis six probe"
jigc milestone add-task axis-six-probe --workflow sub-task "alpha work"
jigc milestone add-task axis-six-probe --workflow sub-task "beta work"
jigc milestone provision axis-six-probe        # -> "at base 4b061fc"      HEAD is now b72400c
(cd .jigc/worktrees/beta-work && jigc workflow sub-task --task beta-work) > b.txt   # exit 0
git checkout --detach 4b061fc                                                        # exit 0
jigc workflow sub-task --task beta-work > c.txt                                      # exit 0
diff b.txt c.txt | grep -c '^[<>]'   -> 2      # exactly one line, 116, differs:
116c116
< resume: … you are in this sub-task's worktree at `.jigc/worktrees/beta-work`, where its work happens — run it here
> resume: … this checkout is not that worktree — run it from this sub-task's own worktree at `.jigc/worktrees/beta-work`, where its work happens
```

and the narration half, driven to the boundary commit on a second rig:

```
jigc milestone finalize axis-six-probe          # exit 0
> finalized 780daef — Finalize milestone axis-six-probe (2 sub-tasks)
>   sub-tasks: alpha-work: 1 code file · beta-work: 1 doc
>   staged in the shared checkout (not committed by this boundary — the aggregate commit is built
>   from the sub-task worktrees; these stay staged):
>     beta-code.txt
git ls-tree -r --name-only HEAD | grep -c beta-code.txt   -> 0     # still uncommitted
git status --short                                        -> A  beta-code.txt   # bytes survive
```

Agrees with the driver exactly: the fix is **narration**, the silence is gone.

### CX-2 — "A6-2 CLOSED: `suppressed.door` is parsed and clap-checked at pack load; named composition returns blocking `workflow.verb-routed`; verb-routed workflows cannot mint or emit degenerate composed bodies through either named-compose door." → **CONFIRMED (repro)**

```
# rig: fresh — the 14-member set derived from both packs (`grep -rl '^  door:'`), NOT a hand list:
#   migrate-adr migrate-arch-doc migrate-changelog migrate-completion-record migrate-decisions-log
#   migrate-deferral-ledger migrate-idea migrate-prd migrate-research migrate-roadmap migrate-spec
#   migrate-vision milestone-execution sub-task
for w in $DOORED; do for form in "start --workflow $w 'an intent'" "start --workflow $w" "workflow $w --preview"; do …
drives=42 refusals_at_exit1=42 verb_routed_findings=42
jigc task list  ->  "no active tasks"            # zero mints across all 42
# the non-member control, same rig:
jigc start --workflow fix-task "control"   # exit 0 — task minted        <-- CX-0's falsifier
```

and the M52 fence itself, driven on a manufactured pack:

```
# a workflow declaring `door: jigc nosuchverb <x>`
dev/jigc-rig fresh --workflow probe-badsupp <f> --repin      # rig exit 1
> pack-load suppression fence failed (workflow-refs.suppressed-malformed): workflow `probe-badsupp`
>   (pack `dev`) declares `suppressed.door: jigc nosuchverb <x>`, which does not parse against the
>   real CLI …
# control, same file with `door: jigc milestone execute <milestone-id>`        -> rig exit 0
```

### CX-3 — "A6-3 CLOSED: `implement-from-spec` states its empty case and routes to `plan`; the criteria projection states both its unbound and empty-criteria cases." → **CONFIRMED (repro)**

```
# rig: fresh (zero committed specs)
jigc start --workflow implement-from-spec "build the thing"     # exit 0
 9: criteria (nothing is listed until a `spec` is committed; with none there is
    nothing to bind here — author one first through the `plan` workflow …
19: … (nothing appears until you bind a spec above and re-compose, and
20:   nothing appears when the bound spec carries no criteria yet)
```

### CX-4 — "Shared composed seam remains complete; `render::composed` owns the text/JSON projection; JSON composed output remains `{task,text}`." → **CONFIRMED (repro)**

```
jigc start --workflow single-task "seam probe" --format json   -> ['task','text']  task='seam-probe'
jigc start --task seam-probe --format json                     -> ['task','text']  task='seam-probe'
jigc workflow quick-fix --preview --format json                -> ['task','text']  task=None
grep -c 'resume:' j1.json j2.json j3.json                      -> 0 0 0    (and absent from `text`)
```

### CX-5 — "Resume states — strengthened and complete (id-less omits; top-level names `jigc start --task`; sub-tasks name the provisioning door and now distinguish own-worktree from elsewhere)." → **CONFIRMED (repro)**

```
jigc start --workflow router        # exit 0 — grep -c '^resume:' = 0
jigc workflow quick-fix --preview   # exit 0 — grep -c '^resume:' = 0
jigc start --workflow single-task "top level"   ->  resume: `jigc start --task top-level`
# the sub-task pair, both postures: CX-1's repro above
```

### CX-6 — "Orientation's three states — complete; I found no path converting an unavailable findings sweep to `none`." → **CONFIRMED on its stated scope (repro), with a bound**

```
jigc start --workflow single-task "unknown probe"; chmod 000 .jigc/tasks/unknown-probe/docs
jigc start                     # exit 0
>   findings: unknown — enumerating the task's code-anchor surface at "…/unknown-probe": Permission denied (os error 13)
jigc start --format json       # exit 0
> unknown-probe | findings= None | findings_unavailable= 'enumerating the task's code-anchor surface at "…": Permission denied (os error 13)'
>    keys: ['base','findings','findings_unavailable','id','intent','milestone','staged','workflow']
```

**Bound.** The claim is about the *state* enumeration and holds. **D-1 is about the finding set
*inside* the `active-task` state** — a leg the state enumeration does not cover, and which this claim
therefore neither asserts nor protects.

### CX-7 — "Read-back owe-set complete: `describe`, `doc show`, `doc list`, `task validate` rows remain present; no read verb was added to the owe-set." → **CONFIRMED (repro)**

```
jigc describe --format json         exit 0 -> ['commands','definitions','schema_version']
jigc doc list --format json         exit 0 -> ['docs']
jigc task validate <id> --format json exit 3 -> ['findings','schema_version']
jigc doc show commit:<id> --task <id>  exit 0    (driver row 39/46: ['fields','item-count','schema-version','sections','slug','type'])
```

### CX-8 — "Pack step commands consistent: every executable `jigc …` form checks out against clap, including `doc set-field`, `doc show --task`, `config set finalize.fan-out.squash` and the milestone sequence." → **CONFIRMED (repro)**

```
jigc doc set-field --help                          exit 0
jigc doc show --help                               exit 0
jigc config set --help                             exit 0
jigc config set finalize.fan-out.squash false      exit 0    # the real command, not just --help
jigc doc show commit:seam-probe --task seam-probe  exit 0
```
plus the driver's wider sweep (row 26: 30 distinct forms over 3025 lines of composed text — 25 real
leaves, 5 prose fragments, zero non-leaf command forms), which I did not re-run in full (*Notes*).

### CX-9 — "Catalog and off-catalog narration — strengthened and complete: membership is exactly `creates-task && selectable`; every complement member must carry `suppressed: {reason, expires}`; `describe` projects the suppression reason through one predicate." → **CONFIRMED on the membership/partition half · REFUTED as a completeness claim**

Confirmed half, driven:

```
jigc describe --workflows --format json   # exit 0
 total=34  router_hidden=22  catalog=12
 hidden carrying "It is hidden from the router catalog:" → 22/22 ; catalog → 0/12
 catalog ids = architecture-documentation decided-task dev-task do-research form-vision
               implement-from-spec park-idea plan project-setup quick-fix record-decision single-task
# and the two text catalogs agree with it and with each other:
jigc start | <one-liners>  vs  jigc start --workflow router | <one-liners>
 orientation=12  router=12  diff exit 0   (identical, and identical to describe's non-hidden set)
```

Refuted half — two falsifying data, both driven:

1. the fence reaches `{reason, expires}` and **not** `door:`, so `fix-task` sits in the complement
   with no door and composes by name (**D-2**, repro below);
2. the narration of the rule exists on **two** surfaces and M52 swept one — `describe` carries the
   carve-out, the orientation footer does not (**D-3**, repro below).

### CX-10 — bound: "JSON composed output deliberately remains `{task,text}` and excludes presentation-only task-state lines." → **CONFIRMED (repro)** — the same drive as CX-4. Matches the driver's OPEN lead 2; **declared, not a defect**.

### CX-11 — "Schema-hash boundary: no violation; the diff from `35195f56` to HEAD changes neither frozen `schema-manifest.yaml`." → **CONFIRMED (datum)**

```
git diff --stat 35195f56..HEAD -- crates/cli/pack/config/schema-manifest.yaml \
                                  packs/methodology/config/schema-manifest.yaml
(empty)
git diff 35195f56..HEAD -- <both manifests> | grep -c 'schema-hash'   -> 0
```

*(Out of this axis's cell set; recorded because the pass asserted it and it is checkable in one
command.)*

## II.b — the driver's defects, each re-driven by me

### D-1 (MEDIUM) — orientation reports a live task's findings without the repository posture that blocks every door it routes to → **CONFIRMED (re-driven, repro)**

Codex is **silent** on this cell (CX-6 asserts the *state* enumeration, not the finding set inside a
state). It stays a finding; it was driven twice.

```
# rig: fresh
jigc start --workflow single-task "posture parity"     # exit 0 — task minted
git bisect start                                       # exit 0 — HEAD UNMOVED (9498f1d)
jigc start                                             # exit 0
>   findings: 2 blocking, 1 advisory
>   Run: `jigc task validate posture-parity`   — previews part of the finalize gate: THE REPOSITORY
>        POSTURE FINALIZE REFUSES UNDER, this task's content findings, the carryover gate, …
jigc start --format json  ->  tasks[0].findings codes:
   ['schema-conformance.field-value-conformant','schema-conformance.required-slot-present',
    'changelog-recording.gate-granted-unused']          # repo.operation-in-progress ABSENT
jigc task validate posture-parity                      # exit 1
> blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
>   committable state
>   route: conclude it, or abandon it with `git bisect reset`, then re-run this command
jigc task finalize posture-parity                      # exit 1 — byte-identical refusal
```

Orientation answers *"2 blocking"* at exit 0 to a reader whose next three printed commands all refuse
for a third reason it never names — on the pinned envelope key a driver reads.

### D-2 (MEDIUM) — `fix-task` is composable by name, and the composed walk's forbidden door is the only one that works → **CONFIRMED (re-driven, repro)**

Codex's CX-9 implies this cannot happen ("every complement member must carry `suppressed:`"); the
fence it cites does not reach `door:`. **Refuted with this repro.**

```
# rig: fresh
jigc start --workflow fix-task "fix the thing"       # exit 0 — task minted: fix-the-thing
grep -n 'THIS worktree' ft.txt   -> 20:`git add` the paths this fix touched, inside THIS worktree …
grep -n 'task finalize' ft.txt   -> 25:Never `git commit` and never `jigc task finalize` here. …
jigc task list                   -> fix-the-thing  [fix-task]  fix the thing      (no milestone)
ls -A .jigc/worktrees            -> No such file or directory                     (no worktree)
jigc doc set-field commit:fix-the-thing#type --value fix --task fix-the-thing            # exit 0
printf 'fix the thing\n' | jigc doc set-slot commit:fix-the-thing#summary --from-file - --task fix-the-thing  # exit 0
printf 'x\n' > fixed.txt && git add fixed.txt
jigc task finalize fix-the-thing                     # EXIT 0   <-- the door line 25 forbids
> no findings — the task validates clean
> finalized f8ffecf — fix: fix the thing
>   added fixed.txt
>   1 file committed
```

The door the body *does* name exists and works (`jigc milestone execute` → `Spawn: cd
.jigc/worktrees/<id> && jigc workflow fix-task --task <id>`), so the gap is the **missing
declaration**, not a missing capability — the same shape `sub-task` already declares.

### D-3 (LOW) — the orientation footer's `Preview:` line still states the pre-M52 rule, which `milestone-execution` falsifies → **CONFIRMED (re-driven, repro)**

Codex's CX-0 explicitly denies a catalog/help contradiction. **Refuted with this repro.**

```
# rig: fresh
jigc start | <footer>
> Preview: `jigc workflow <id> --preview`   — read a task-minting workflow's step text without
>   minting a task; a workflow that mints nothing has no preview — `jigc start --workflow <id>`
>   composes it directly, and mints nothing either                        <-- no carve-out
grep -c 'Neither reaches' <orientation>  -> 0
grep -c 'Neither reaches' <describe --workflows>  -> 1
# the falsifier:
jigc start --workflow milestone-execution   # exit 1 · blocking · workflow.verb-routed
jigc start --workflow router                # exit 0 (the control — mints nothing, composes directly)
```

`milestone-execution` is `creates-task: false` **and** carries a `door:`; the sentence that tells the
reader what to do with a mints-nothing workflow is true for `router` and false for it.

### D-4 (LOW) — a legitimately-empty `milestone execute` walk does not state its empty case → **CONFIRMED (re-driven, repro)**

Codex is silent on this cell.

```
# rig: fresh
jigc milestone create "Empty probe"            # exit 0 — record commit 0b383a9
jigc milestone execute empty-probe             # exit 0
> Spawn a sub-agent per sub-task (one per `Spawn:` line below) and implement it.
> Each runs in its own isolated working area keyed by task id. Wait for every
> spawned sub-task to report complete before moving on — nothing is merged yet.
grep -c '^Spawn:'  -> 0        stderr -> empty        (no statement of the empty case anywhere)
jigc milestone provision empty-probe  # exit 0 — "provisioned 0 worktree(s) … at base 34b1877 ()"
jigc milestone join empty-probe       # exit 0 — "0 doc(s) merged"
jigc milestone finalize empty-probe   # exit 3
> blocking · milestone.zero-contribution — milestone:empty-probe would land no work …
>   route: `jigc milestone provision empty-probe` … — or settle the milestone as abandoned with
>          `jigc milestone discard empty-probe`
```

The terminal is safe, which bounds the severity; the class is *the empty render does not state its
empty case*, and its sibling (`implement-from-spec`'s spec list) got exactly that clause at M52.

## II.c — the driver's observations and open leads

Carried forward unchanged; none is contradicted by the source pass, and none was promoted.

| lead | status after reconciliation |
|---|---|
| **O-1** `jigc migrate` mints silently over open work | OPEN, declared + record corrected at M52; trigger carried (`design/write-commands.md`:189) |
| **O-2** the malformed-address refusal carries no code on either arm | OPEN, inside M50's declared flatten posture (`command-output-contract.md`:294) |
| **O-3** the orientation `findings: unknown` reason prints an absolute host path, on text **and** inside the pinned `findings_unavailable` key | OPEN — **re-observed in my own CX-6 drive** (`"/private/var/folders/…/repo/.jigc/tasks/unknown-probe"`); a record-truth lead, inside a declared bound |
| **O-4** `milestone-execution`'s `suppressed.reason` is in the present tense | OPEN, marginal; D-4 is the reason the tense is arguably right |
| **O-5** `task validate` does not preview the migration review hold | OPEN, declared (`command-output-contract.md`:480) |
| **O-6** C5 is a manifest-scoped guarantee | OPEN, declared (`surface-contract.md`:127 / `introspection.md`:90) |
| **O-7** `milestone provision` over a zero-sub-task milestone acks an empty parenthesised list | OPEN, cosmetic — **re-observed** in my D-4 drive: `provisioned 0 worktree(s) for milestone:empty-probe at base 34b1877 ()` |
| **driver OPEN lead 2** the JSON arm carries no task-state lines | OPEN, declared at the seam — and **CX-10 states the same thing from the source side**, so both passes now agree it is a decision |

## II.d — roll-up

| | count | members |
|---|---|---|
| **CONFIRMED** | **10** | CX-1, CX-2, CX-3, CX-4, CX-5, CX-6 (on scope), CX-7, CX-8, CX-10, CX-11 — all origin **codex**; plus the four driver defects D-1…D-4, origin **driver**, each re-driven |
| **REFUTED** | **2** | CX-0 (falsified by D-1…D-4, two of them inside the categories it names) · CX-9's completeness half (falsified by D-2's un-fenced `door:` key and D-3's un-swept sibling sentence) |
| **OPEN LEADS** | **8** | O-1…O-7 and the driver's JSON-arm lead — every one declared somewhere, none promotable on a source read, none droppable |
| **DEMOTED** | **0** | no row marked driven lacks an argv and an observable |

**Net for the axis, after reconciliation: 4 findings, all surface-tier, all driven twice** (driver,
then me) — **zero data-loss, zero corruption, zero regression**. All nine M51 CONFIRMED rows are
CLOSED on rc.16, confirmed independently by both passes. Three of the four new findings (D-1, D-3,
D-4) are **incomplete sweeps of M52's own new work**.

---

# PART III — Doors covered

Every clap leaf that is the door of ≥1 **driven** row in Part I or Part II, in `VERB_KINDS` spelling.
A classification-only row confers no coverage.

| leaf | where |
|---|---|
| `start` | driver rows 1–17, 37; D-1, D-2, D-3; my CX-1/2/3/4/5/6 and D-1/D-2/D-3/D-4 drives |
| `workflow` | driver rows 18–25, 37; my CX-1 (both postures), CX-2 (14 previews), CX-5 |
| `describe` | driver rows 26–30, 38; my CX-7, CX-9 |
| `migrate` | driver rows 31–33, 37 |
| `milestone execute` | driver rows 34–38; D-4; my D-4 drive |
| `milestone create` | driver rows 34, 36; my CX-1, D-4 |
| `milestone add-task` | driver row 34; my CX-1 |
| `milestone provision` | driver rows 27, 34, 35; my CX-1, D-4 |
| `milestone join` | driver rows 27, 34; my CX-1, D-4 |
| `milestone finalize` | driver row 34; my CX-1 (the landed boundary + its narration), D-4 (`milestone.zero-contribution`) |
| `doc show` | driver rows 39–41, 46; my CX-7, CX-8 |
| `doc list` | driver rows 42, 46; my CX-7 |
| `doc schema` | driver rows 3, 42 |
| `doc create` | driver row 3 |
| `doc add-item` | driver row 3 |
| `doc set-field` | driver rows 1, 4; my CX-1, CX-8, D-2 |
| `doc set-slot` | driver rows 1, 3, 4; my CX-1, D-2 |
| `doc author` | driver row 31 |
| `task list` | driver rows 5, 18, 25; my CX-2, D-2 |
| `task validate` | driver rows 43–46; D-1; my CX-7, D-1 |
| `task bind` | driver row 3; R3 |
| `task finalize` | driver rows 1, 31, 44; my D-1, D-2 (the forbidden door that lands `f8ffecf`) |
| `config set` | **added by this reconciliation** — `jigc config set finalize.fan-out.squash false` driven at exit 0 as the CX-8 step-command check (the driver drove `--help` shapes only) |
| `setup` | driver row 29; my CX-2 fence drive (the `setup.pack-load` advisory the installed binary emits at rig construction) |

**24 leaves driven to behaviour** (the driver's 23, plus `config set`). Every other `VERB_KINDS`
leaf — `uninstall`, `upgrade`, `ingest`, `migrate-corpus`, `unmanage`, `rename`, `relocate`,
`validate`, `doc rename`, `doc remove-item`, `doc retitle-item`, `task diff`, `task discard`, the
five remaining `config` leaves, `milestone add-from-spec`, `milestone list-tasks`,
`milestone discard` — is another axis's subject and was not reached here.

---

# PART IV — Notes

1. **On the demotion rule, applied honestly.** The brief says a row *"marked driven that carries no
   repro block"* is not driven. Read at the letter, that demotes ~42 of 46 rows, because the driver's
   table format carries fenced repro blocks only for the three M51 closures and the four defects, and
   states every other row as *argv + exit + the surface predicate it asserts*. I applied the rule as
   the driver's own stated definition does — **a row is driven iff its argv ran and its verdict
   carries a repro block or a measurable predicate stated in the row** — and then went looking for
   rows failing *that*. **There are none.** The four closest calls, and why each survives:
   row **38** (a prose-tense judgement — but the two surfaces it compares are both quoted from driven
   output), row **29** (evidenced by a rig-construction failure rather than a jigc exit — I re-drove
   its M52-new arm (c) myself and the fence fires), row **30** (a negative: no clause, 0 orientation
   listings — both countable), and row **20** (a "matches contract (the door)" verdict whose content
   is D-2's positive control, which I re-drove). §3.2's `cwd-unreadable` cell and all of §4 are
   already self-marked not-driven.
2. **An instrument note that bit me and will bite the next re-runner.** R1's `BASE` must be read from
   the `milestone provision` line (*"at base <sha>"*), **not** from `git rev-parse HEAD`: `milestone
   create` and `add-task` land record-only commits, so HEAD has moved past the pin by then. Detaching
   at HEAD instead of the pin puts the shared checkout *off* the pin, which is a **different cell**
   (the driver's rows 7/22, exit 1) and yields a whole-file diff instead of the one-line one. My first
   A6-1 re-drive did exactly that; the corrected drive is the one recorded above.
3. **What I did not re-drive**, stated plainly: the driver's row-26 aggregate (3025 lines of composed
   text, 30 extracted `jigc …` forms) — I re-drove five forms, including the four Codex names by
   name, and all resolve; the 17-preview sweep (row 18); the 12 `migrate-*` doors run for real (row
   33); the four `describe` behavioural hints (row 27); and rows 31/32's migrate chain. None of these
   is contradicted by the source pass, so under the reconciliation rule none needed driving to a
   verdict — but the reconciled file's confidence in them is the driver's, not mine.
4. **`cwd-unreadable` (of `PRE_DISPATCH_FAULTS`) stays undriven** on both sides — the driver flagged
   it for me, and I did not drive it either: deleting the process cwd under an already-`cd`-ed shell
   is a harness manoeuvre the two-step rig eval does not support cleanly. It is an **OPEN
   instrument gap**, not a finding, and it overlaps axis 5.
5. **The two passes' disagreement is structural, not adversarial.** Codex read the fences and found
   them sound; every one of them *is* sound over the set it fences. All four defects live where no
   fence has a subject: an un-fenced optional key (`door:`), a second producer of one sentence, a
   finding set assembled at a different door, and an empty-case clause that rides a different
   placeholder family (`fan-out: over:` rather than `{{@…}}`). That is the completeness question the
   source pass was asked and the answer it could not reach without driving.
