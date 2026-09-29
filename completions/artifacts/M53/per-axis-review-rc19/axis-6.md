<!-- M53 THIRD PARTIAL per-axis review — axis 6 — the reconciled file, copied verbatim. Driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.19` (repo HEAD `a8904637`), 2026-09-23. -->

<!-- M53 THIRD PARTIAL per-axis review — axis 6 · composed surfaces · OPUS DRIVER · driven on the installed `jigc 1.0.0-rc.19`, 2026-09-23 -->

# M53 third partial per-axis review — AXIS 6 · composed surfaces — the OPUS DRIVER

**Binary.** `/Users/maurice/.local/bin/jigc`, asserted **`jigc 1.0.0-rc.19`** before anything else
(`jigc --version` → `jigc 1.0.0-rc.19`, rc=0). **RELEASE posture** — the `#[cfg(debug_assertions)]`
route-fence panics do not exist here. Repo HEAD at read time: `a8904637` (*chore(release): 1.0.0-rc.19
— the third M53 stamp, after the cwd-dependence arc*).

**Method.** Every row ran in a throwaway `dev/jigc-rig` repo
(`rig=$(SCRATCH=<dir> dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
— two-step eval, `mktemp -d` roots, no teardown, no `rm -rf` on a variable path). Nine rigs were built:
six `fresh`, two `committed-singletons`, one `bare`, plus two whose **`SCRATCH` root carries a space**
(`…/jigc space.GZUMb3`) or an **apostrophe and a `#`** (`…/it's #odd.ygoK35`). Nothing was written into
the working repository; no fix, no commit, no repo edit. **The Codex source pass for this axis was not
read** (per brief); reconciliation is a separate agent.

**Shell notes carried for the reconciler.** zsh does **not** word-split an unquoted parameter — a
`for w in $DOORS` loop over a space-joined string iterates **once** with the whole string and every
drive silently becomes one nonsense argv (it bit here on the first `suppressed.door` sweep; all 42
drives were re-run with an array). A pipeline reports only its last command's status, so every exit
code below was read bare.

**The cwd dimension.** This re-review adds a sixth cell to M51/M52's five, because the cwd arc is what
changed. Every applicable row was driven from **four cwds** — the repo root · an ordinary subdirectory
(`docs/deep`) · a **provisioned fan-out worktree** (`.jigc/worktrees/<sub>`) · an **ordinary linked
worktree** outside `.jigc/` — and the emitting/consuming cells again on a **spaced** and on an
**apostrophe+`#`** repository path.

---

## 1. The door set and the registry counts, read from the code

Counts read at `a8904637` by parsing each slice literal with comments stripped (a small Python
top-level-comma counter), **not** taken from any design doc's prose. Every count is **unmoved from
M52's re-run**.

| registry | file | count I read | vs M52 |
|---|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1835` | **47** leaves | = |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:2004` | **47** rows | = |
| `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs:3138` | **14** rows | = |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs:2523` | **16** rows | = |
| `SLUG_DOORS` | `crates/cli/src/cli.rs:2830` | **6** rows | = |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs:2590` | **25** rows | = |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:171` | **10** rows | = |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3430` | **6** (`[&DestroyingDoor; 6]`) | = |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs:6254` | **64** rows | = |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:968` | **7** rows | = |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs:5639` | **4** codes | = |
| `PRE_DISPATCH_FAULTS` | `crates/cli/tests/pre_dispatch_faults.rs:145` | **3** faults | = |

**This axis's own door set is derived from the render seam, re-read at this HEAD** (the eight production
`render::composed` / `render::composed_preview` call sites, `crates/cli/src/render.rs:330` / `:382`):

| render site | dispatch fn | clap leaf |
|---|---|---|
| `cli.rs:1638` | `run_compose_named_no_intent` | `start` |
| `cli.rs:1661` | `run_compose` | `start` |
| `cli.rs:1683` | `run_compose_named` | `start` |
| `cli.rs:1725` | `run_resume` | `start` |
| `cli.rs:1747` | `run_reenter` | `workflow` |
| `cli.rs:1768` (`composed_preview`) | `run_preview` | `workflow` |
| `migrate.rs:76` | `dispatch_migrate` | `migrate` |
| `milestone.rs:4884` | `dispatch_execute` | `milestone execute` |

**8 render sites over 4 clap leaves**, ∪ `describe` ∪ the three read verbs M48's read-back fence's
owe-set names.

**Door set = 8:** `start` · `workflow` · `migrate` · `milestone execute` · `describe` · `doc show` ·
`doc list` · `task validate`.

**Cell set = 6** (M51/M52's five, plus the arc's own): C1 the step text names a verb that answers · C2
the `resume:` line's real states · C3 orientation's states · C4 the catalog one-liner matches the verb's
behaviour · C5 the off-catalog reason is stated · **C6 cwd-invariance — the surface reads identically,
and every command it emits runs, from all four cwds and from a shell-hostile repo path.**

8 × 6 = **48 cells · 33 applicable and driven · 15 n/a with reasons** (§3.1 — M51's n/a set, re-derived
at this HEAD and unchanged; C6 is applicable at every door).

**The `suppressed.door` set is re-derived, not carried**: `grep -rln '^  door:'` over both packs'
workflow dirs → **14** members, and the full `(creates-task, selectable, suppressed, door)` partition
over all **34** workflows is in §3, row 5.

---

## 2. M52 §A rows: CLOSED / STILL-OPEN

M52's axis-6 ledger carried **4 defects** (`(6, D-1)` … `(6, D-4)`), all tier-2/tier-3 and all
**triaged to the 1.x findings ledger** ([decisions-pending.md](../../../../implementation/decisions-pending.md)
→ *Tier 2 and tier 3 — 23 rows*). **M53 built no fix for any of them**, so all four are **expected
STILL-OPEN**, and all four reproduce byte-for-byte on rc.19.

| M52 §A row | tier | status on rc.19 | the drive |
|---|---|---|---|
| `(6, D-1)` orientation reports a live task's findings without the repository posture | 2 | **STILL-OPEN** (triaged) | R-D1 |
| `(6, D-2)` `fix-task` composable by name, and its forbidden door is the only one that works | 2 | **STILL-OPEN** (triaged) | R-D2 |
| `(6, D-3)` the orientation `Preview:` footer states the pre-M52 rule `milestone-execution` falsifies | 3 | **STILL-OPEN** (triaged) | R-D3 |
| `(6, D-4)` a legitimately-empty `milestone execute` walk does not state its empty case | 3 | **STILL-OPEN** (triaged) | R-D4 |

```
# R-D1 · rig: fresh
jigc start --workflow single-task "posture parity"   # 0 — task minted
git bisect start                                     # 0 — BISECT_LOG BISECT_NAMES BISECT_START, HEAD unmoved
jigc start                                           # 0
>   findings: 2 blocking, 1 advisory
>     blocking · schema-conformance.field-value-conformant … commit:posture-parity
>     blocking · schema-conformance.required-slot-present  … commit:posture-parity#summary
>     advisory · changelog-recording.gate-granted-unused   … task:posture-parity
>   Run: `jigc task validate posture-parity` — previews … THE REPOSITORY POSTURE FINALIZE REFUSES UNDER …
jigc start --format json  →  tasks[0].findings = the same three codes;
                             repo.operation-in-progress ABSENT from the pinned wire
jigc task validate posture-parity                    # 1
>   blocking · repo.operation-in-progress — a bisect is in progress …
>   route: conclude it, or abandon it with `git bisect reset`, then re-run this command
```

```
# R-D2 · rig: fresh
jigc start --workflow fix-task "fix the thing"       # 0 — task minted: fix-the-thing
grep -n 'THIS worktree|task finalize' → :20 "`git add` … inside THIS worktree"
                                        :25 "Never `git commit` and never `jigc task finalize` here."
jigc task list → fix-the-thing [fix-task]   (no milestone, no worktree)
jigc doc set-field commit:fix-the-thing#type --value fix --task fix-the-thing     # 0
printf 'fix the thing\n' | jigc doc set-slot commit:fix-the-thing#summary --from-file - --task …  # 0
printf 'x\n' > fixed.txt && git add fixed.txt
jigc task finalize fix-the-thing                     # 0   <-- the door line 25 forbids
> no findings — the task validates clean
> finalized 416eb35 — fix: fix the thing   ·   added fixed.txt   ·   1 file committed
```

```
# R-D3 · rig: fresh
jigc start | grep '^Preview:'
> Preview: `jigc workflow <id> --preview` — … a workflow that mints nothing has no preview —
>   `jigc start --workflow <id>` COMPOSES IT DIRECTLY, and mints nothing either
jigc describe --workflows | head -4          # the SWEPT sibling sentence:
> … NEITHER REACHES A WORKFLOW WHOSE LINE BELOW SAYS IT IS REACHED ONLY THROUGH A VERB … and both
>   of these refuse it by name.
jigc start --workflow milestone-execution    # 1 · workflow.verb-routed   <-- the falsifier
jigc start --workflow router                 # 0                          <-- the control
```

```
# R-D4 · rig: fresh
jigc milestone create "Empty probe"          # 0
jigc milestone execute empty-probe           # 0
> Spawn a sub-agent per sub-task (one per `Spawn:` line below) and implement it. …
grep -c '^Spawn:' → 0 ;  stderr → 0 bytes ;  nothing states the empty case
jigc milestone finalize empty-probe          # 3 · milestone.zero-contribution, two-armed route (terminal safe)
```

**M51's three defects, which M52 closed, are re-verified holding on rc.19** (they are the cells the arc
was most likely to disturb):

- **A6-1 — the `resume:` line discriminates the posture it is composed in.** Driven from five cwds for
  one sub-task (`jigc workflow sub-task --task beta-work`): from `beta-work`'s own worktree, *"you are
  in this sub-task's worktree at `.jigc/worktrees/beta-work`, where its work happens — run it here"*
  (exit 0); from the **sibling** worktree `alpha-work` (which stands at the base pin), *"this checkout
  is not that worktree — run it from this sub-task's own worktree at `.jigc/worktrees/beta-work`"*
  (exit 0); from the root, the subdirectory and the linked worktree — all off the pin — the
  base-pin refusal, **byte-identical across the three** (§3, rows 7/21/22).
- **A6-2 — `workflow.verb-routed`.** Re-driven over the whole derived 14-member `suppressed.door` set ×
  3 argv forms = **42 drives, 42 exits of 1, 42 `workflow.verb-routed`, zero mints** (`jigc task list`
  line-count unchanged before and after).
- **A6-3 — the empty-case clauses.** `implement-from-spec` over a zero-spec corpus still states both
  empty cases; `jigc task bind`'s `store.not-found` / `store.unknown-type` still ride the findings
  envelope (checked at the constructor and through `ENVELOPE_OWED_CODES`, 4 members, unmoved).

**M52's open leads** — all four re-driven, all unchanged (§6).

---

## 3. The (door, cell) table — rc.19

Route kind: **M**echanical / **H**uman / **I**nformational / **none**.

| # | door | cell | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `start` | C1 | `start --workflow single-task "add a greeting helper"` **run entirely from `docs/deep`**, then every verb its text names: `doc set-field` → `doc set-slot` → `doc show … --task` → `doc list --task` → `task validate` → `task finalize` | 0/0/0/0/0/0/0 | advisory `changelog-recording.gate-granted-unused` | M ×4 | the chain lands `13fbc46 — feat: add a greeting helper`, `added greet.rs`, 1 file committed — **from a subdirectory** | matches contract |
| 2 | `start` | C1 | `start --workflow implement-from-spec "build the thing"` over a **zero-spec** corpus | 0 | none | I | *"nothing is listed until a `spec` is committed; with none there is nothing to bind here — author one first through the `plan` workflow"* | matches contract (A6-3 holds) |
| 3 | `start` | C1/C5 | the derived `suppressed.door` set (**14**) × {`--workflow <X>`, `--workflow <X> "an intent"`, `workflow <X> --preview`} = **42 drives** | 1 ×42 | `workflow.verb-routed` ×42 | M (the declared door) | one blocking finding keyed `workflow:<id>`; `jigc task list` unchanged before/after | matches contract |
| 4 | `start` | C1 | `start --workflow fix-task "fix the thing"` → fill → `task finalize` | 0/0/0/**0** | none | none | the body forbids `jigc task finalize`; the forbidden door **lands `416eb35`** | **STILL-OPEN `(6, D-2)`** |
| 5 | `start` | C2 | `start --workflow router` · bare `start` · `workflow single-task --preview` (the id-less forms) | 0 ×3 | none | M | `grep -c '^resume:'` = **0 · 0 · 0** | matches contract |
| 6 | `start` | C2 | `start --task <migrate-id>` from all four cwds | 0 ×4 | none | M | byte-identical across the four; `resume:` re-feeds the persisted source | matches contract |
| 7 | `start`/`workflow` | C2/C6 | `workflow sub-task --task beta-work` with HEAD off the pin, from root · `docs/deep` · linked worktree | 1 ×3 | none (flattened, routed) | M | the refusal is **byte-identical** across the three and its route is an **absolute, shell-quoted** `` `cd '<abs>/.jigc/worktrees/beta-work'` `` | matches contract |
| 8 | `start` | C3 | `start` / `--format json` in a `bare` repo, from the root and from `src/deep` | 0 ×4 | none | M (`jigc setup`) | text *"This project isn't set up."*; JSON `{"state":"unset-project","schema_version":3}`; **subdir byte-identical** | matches contract |
| 9 | `start` | C3 | `start` / `--format json` with one live task | 0 | 2 blocking + 1 advisory | M ×4 | `state:"active-task"`, `tasks[0].findings` = the three codes | matches contract |
| 10 | `start` | C3 | `chmod 000 .jigc/tasks/<id>/docs` then `start` / `--format json` | 0/0 | none | M | text `findings: unknown — …`; JSON `findings: null` + `findings_unavailable: "<reason>"` — *unknown* never rendered as *none* | matches contract (see O-3) |
| 11 | `start` | C3 | one live task + `git bisect start`, then `start` vs the `task validate` it routes to | 0 vs **1** | orientation omits `repo.operation-in-progress`; validate raises it | M on both | the divergence M52 found, unchanged | **STILL-OPEN `(6, D-1)`** |
| 12 | `start` | C4 | `start` (orientation catalog) vs `start --workflow router` (router catalog) | 0/0 | none | M | **12 vs 12** one-liners; `diff` of the sorted, indent-stripped sets is **empty** — one generator | matches contract |
| 13 | `start` | C4 | all **12** catalog one-liners vs the `create-gates:` each workflow composes | 0 ×12 | none | I | `dev-task`/`quick-fix` → **no** `create-gates:` · `decided-task` → `decisions-log` · `single-task` → `adr, changelog` · `implement-from-spec`/`record-decision` → `adr` · `plan` → `spec` · `project-setup` → `prd` · `architecture-documentation` → `arch-doc` · `do-research` → `research` · `form-vision` → `vision` · `park-idea` → `idea`. **12/12** | matches contract |
| 14 | `start` | C5 | the orientation footer's `Preview:` line vs `start --workflow milestone-execution` | 0 then **1** | `workflow.verb-routed` | I vs M | the footer still states the pre-M52 rule the refusal falsifies | **STILL-OPEN `(6, D-3)`** |
| 15 | `start` | C6 | bare `start` / `start --format json` from all four cwds | 0 ×8 | — | — | stdout **and** stderr byte-identical from root, `docs/deep`, the fan-out worktree and the linked worktree | matches contract |
| 16 | `start` | C6 | `start --workflow quick-fix "<intent>"` **minted from an ordinary linked worktree** | 0 | none | M | `task minted:`; the working area lands at **jigc_home** (`<main>/.jigc/tasks/<id>/`), the worktree's own `.jigc/tasks/` stays empty, and `jigc start --task <id>` from the **root** resumes it at exit 0 | matches contract |
| 17 | `workflow` | C1 | `workflow <w> --preview` for the 12 catalog workflows + `planning`/`completion`/`record-change`/`record-dogfood`/`fix-task` | 0 ×17 | none | I | every preview composes; `jigc task list` unchanged | matches contract |
| 18 | `workflow` | C1 | **the mechanical sweep**: 17 previews + `router`/`ingest-existing`/`increment` + `milestone execute` + a real `migrate` compose = **3026 lines** of composed text; every distinct `jigc …` form extracted and driven `--help` | 0 | none | — | **30 distinct forms · 25 real clap leaves · 5 prose fragments** (`jigc can manage`, `jigc checks only`, `jigc has adopted`, `jigc itself accepts`, `jigc never auto-migrates`) — the same partition M52 measured at 3025 lines (the delta is the version token) | matches contract |
| 19 | `workflow` | C5 | `workflow {router, ingest-existing, increment} --preview` | 1 ×3 | none | M | the mints-nothing refusal; all three direct forms then run at exit 0 | matches contract |
| 20 | `workflow` | C6 | `workflow single-task --preview` from all four cwds | 0 ×4 | — | — | byte-identical stdout+stderr | matches contract |
| 21 | `workflow` | C1/C6 | the **spawn line run verbatim**: `sh -c "$SPAWN"` from root · `docs/deep` · a *sibling* fan-out worktree · a linked worktree, on a plain root | 0 ×4 | none | M | each lands in the named worktree and composes (`resume:` present, stderr empty) | **matches contract — census C1-14/C3-01 CLOSED** |
| 22 | `workflow` | C1/C6 | the same on a **spaced** repo root (`…/jigc space.GZUMb3/…`) and on an **apostrophe+`#`** root (`…/it's #odd.ygoK35/…`) | 0 ×5 | none | M | the emitted operand is `cd '…/jigc space…/.jigc/worktrees/alpha-work'` and `cd '…/it'\''s #odd…/eps-area'` — POSIX single-quote escaping; `sh -c` rc=0 from every cwd | **matches contract — cwd-review HIGH 1 CLOSED** |
| 23 | `migrate` | C1/C6 | from `docs/deep`, `migrate old-decision.md --as adr` over an **untracked** source, then the whole route **run verbatim from that same subdirectory** | 1 then 0/0 | `migrate.source-untracked` | H | route: ``stage it with `git -C <ABSOLUTE> add -- docs/deep/old-decision.md`, then re-run `jigc migrate old-decision.md --as adr` `` — the git half **aimed absolute**, the jigc half **echoing the caller's own token**; both ran at rc=0 from `docs/deep` and the migrate minted | **matches contract — census C1-03 + C2-03 CLOSED** |
| 24 | `migrate` | C2 | the composed footer, then `start --task <migrate-id>` from all four cwds | 0 ×5 | none | M | `resume:` / `what's-left:` / `task scope:` / `create-gates:`; the four resumes byte-identical | matches contract |
| 25 | `migrate` | C1 | the full migrate arc from `docs/deep`: `doc create adr` → 3 × `doc set-slot` → commit doc → **plain** `task finalize` | 0/0/0/**4** | the exit-4 review hold | M (`--approve`) | *"migration review required — nothing committed … Re-run `jigc task finalize <id> --approve` … DELETE the foreign original `docs/deep/old-decision.md`"*; `git log` unmoved | matches contract |
| 26 | `migrate` | C4/C5 | all **12** `migrate-*` declared doors run for real (`migrate legacy/src-<ty>.md --as <ty>`) | 0 ×12 | none | M | every verb-routed refusal's declared door is an argv that exists and mints (`task minted:` on all 12) | matches contract |
| 27 | `milestone execute` | C1/C6 | the **whole fan-out chain run from inside a fan-out worktree**: `execute` → the two `Spawn:` lines → sub-task authoring → `join` → `finalize` | 0/0/0/0/**0** | none | M ×3 | the boundary lands `77ca9c7 — Finalize milestone fan-chain (2 sub-tasks)`, 3 files, on the **main** checkout's HEAD — from the cwd jigc's own spawn line creates | **matches contract — census C2-06 (rank 2) CLOSED** |
| 28 | `milestone execute` | C6 | `milestone execute <m>` from all four cwds, plain root and spaced root | 0 ×8 | none | M | byte-identical stdout in all eight | matches contract |
| 29 | `milestone execute` | C1 | the **partial**-provision state (1 of 2 sub-tasks provisioned, confirmed by `git worktree list`) | 0 | advisory `milestone.worktrees-partial` | M (`jigc milestone provision <m>`) | the advisory prints on **stdout, before the walk**, names `delta-area`, its locus `.jigc/worktrees/delta-area`, and *"its `Spawn:` line below would run in a working area that does not exist"* | matches contract (see O-4) |
| 30 | `milestone execute` | C1 | zero sub-tasks | 0 | none | M ×3 | `grep -c '^Spawn:'` = 0, stderr 0 bytes, no statement of the empty case | **STILL-OPEN `(6, D-4)`** |
| 31 | `milestone execute` + the other 3 compose leaves | C1/C2 | all **6** composed render forms × `--format json` | 0 ×6 | advisory on stderr where raised | M | every one is exactly `['task','text']`; `task: null` for `--preview`, `milestone execute` and the router selection; `task` non-null for the mint, the resume, the worktree re-entry and `migrate`; `resume:` present on the text arm and in **no** JSON arm's `text` | matches contract |
| 32 | `milestone execute`/`finalize` | C6 | the **base-mismatch gate**: pin at `564b8af`, main advanced to `e78483c`, `milestone finalize` from the root vs from inside the fan-out worktree | 3 vs **3** | `finalize.base-mismatch` on both | H on both | the two refusals are **byte-identical** | **matches contract — census C2-11 CLOSED** |
| 33 | `describe` | C1/C4 | `describe` · `--workflows` · `--commands` · `--format json`, each named by a composed step | 0 ×4 | none | I | `['commands','definitions','schema_version']`; **31** command entries; `origin_pack` on every workflow row | matches contract |
| 34 | `describe` | C4 | four command hints driven against behaviour | 0 | none | I | `milestone-provision` *"one detached base-pin worktree per sub-task"* → `git worktree list` shows `(detached HEAD)` at the pin · `milestone-join` *"commits nothing"* → HEAD byte-equal before/after (`e78483c…` → `e78483c…`) · `show-doc` *"with `--task`, the staged copy"* → staged bytes served (row 1) · `validate-task` *"the repository posture …"* → really raised (row 11) | matches contract |
| 35 | `describe` | C5 | `describe --workflows --format json`, all entries partitioned | 0 | none | I | **34 = 22 + 12**; **22/22** hidden rows carry a non-empty `router_hidden` reason and the text arm prints *"It is hidden from the router catalog"* exactly **22** times; the 12 non-hidden **equal** the orientation catalog | matches contract |
| 36 | `describe` | C5 | three manufactured packs (`--pack-from-dev --workflow <id> <f> --repin`) | rig 1 ×2, rig 0 ×1 | pack-load fences | M | (a) `creates-task: false` with no `suppressed:` → *"pack-load suppression fence failed"* · (b) `suppressed.door: jigc nosuchverb <x>` → **`workflow-refs.suppressed-malformed`** naming the pack, *"which does not parse against the real CLI"* · (c) the same probe in a **manifest-less** pack loads clean and narrates with **no** reason | (a)/(b) match; (c) **STILL-OPEN lead**, declared (O-6) |
| 37 | `describe` | C6 | `describe` / `--workflows` / `--commands --format json` from all four cwds | 0 ×12 | — | — | byte-identical | matches contract |
| 38 | `doc show` | C1 | `doc show commit:<id> --task <id>` run from the read-back line that named it, from `docs/deep` | 0 | none | none | the staged bytes just written, front matter included | matches contract |
| 39 | `doc show` | C1 | `doc show commit:<live-id>` (no `--task`) · `doc show vision:bogus` · `doc show vision --task no-such-task` | 1 ×3 | `store.transient-type` · `store.fixed-identity` · `finalize.no-task` | M ×3 | each names its own recovery | matches contract |
| 40 | `doc show` | C6 | `doc show vision` / `--format json` from all four cwds | 1 ×4 / 0 ×4 | — | — | byte-identical; a `doc show` from a fan-out worktree whose HEAD predates the file still serves the **main** checkout's committed bytes | matches contract |
| 41 | `doc list` | C1/C6 | `doc list` · `doc list --task <id>` · `doc list --task no-such-task` · `doc list padding`, and the first two from all four cwds | 0/0/1/1 | — / — / `finalize.no-task` / `store.unknown-type` | I + M | `id · path · state` rows; the empty set prints the `note:` naming the open task and the `--task` form; all cwds byte-identical | matches contract |
| 42 | `task validate` | C1/C6 | `task validate <id>` from the `what's-left:` line, from all four cwds, over a project-layer severity delta | 3 ×4 | `schema-conformance.*`, `changelog-recording.*` | M on every finding | byte-identical across the four — the **project config layer set from the root is read identically from a linked worktree and a fan-out worktree** | matches contract |
| 43 | `task validate` | C4 | the clause's repository-posture claim, driven | 1 | `repo.operation-in-progress` | H | the posture refusal is byte-identical to `task finalize`'s | matches contract |
| 44 | `describe`/`doc show`/`doc list`/`task validate` | C4 | each door's `--format json` key set | 0/0/0/3 | — | — | `['commands','definitions','schema_version']` · `['fields','item-count','schema-version','sections','slug','type']` · `['docs']` · `['findings','schema_version']` — **all unmoved from M52** | matches contract |
| 45 | all 8 | C6 | the **pack-set seam**: a project-layer knob written from the root (`config set default-workflow single-task`) read back from all four cwds; then the same knob **written from inside the fan-out worktree** and read back from the root | 0 ×8 | — | — | `default-workflow = single-task  (project)` from every cwd; the worktree's write lands in the **main** checkout's `.jigc/config/manifest.yaml` while the worktree's own `.jigc/config/` is untouched | matches contract (see O-1) |

### 3.1 n/a rows (15), each with its reason — re-derived at this HEAD, unchanged from M51/M52

| door | cell | reason |
|---|---|---|
| `describe`, `doc show`, `doc list`, `task validate` | **C2** | none of the four renders `render::task_state_lines` — they carry no `resume:` line to be in a state |
| `workflow`, `migrate`, `milestone execute`, `describe`, `doc show`, `doc list`, `task validate` | **C3** | orientation is `run_orient`, reached only by bare `jigc start`; no other leaf produces an `OrientationView` |
| `doc list` | **C4** | the 31-entry command catalog ships **no** `list-docs` ref (verified in this run's full listing), so `doc list` is the door of no catalog one-liner |
| `doc show`, `doc list`, `task validate` | **C5** | off-catalog-ness is a property of a **workflow**, not of a verb |

### 3.2 `PRE_DISPATCH_FAULTS` × this axis's doors — including the cell M52 could not drive

| fault (fixture) | doors | exit | stdout | stderr | verdict |
|---|---|---|---|---|---|
| **`cwd-unreadable`** (`mkdir docs/gone; cd docs/gone; rmdir <abs>/docs/gone`) | `start` · `describe` · `doc list` · `task validate <id>` · `workflow --preview` | 1 ×5 | **empty** | one line, identical at all five: *"cannot determine the current directory: No such file or directory (os error 2)"*; under `--format json`, one document `{"error": …}` on stderr with stdout empty | **matches contract — this is the one cell M52 declared *not driven*, now driven** |
| `packs-yaml-malformed` (`.jigc/config/packs.yaml` invalid YAML in the **main** checkout) | `start` · `describe` · `doc list` · `workflow --preview`, each **× 4 cwds = 16 drives** | 1 ×16 | empty | *"`.jigc/config/packs.yaml` is not a valid pack-set list: mapping values are not allowed in this context at line 1"* — **identical from the linked worktree and the fan-out worktree**, i.e. the pack loader binds jigc_home | matches contract (the seventh-walk-up fix, driven) |
| `pack-resource-missing` | — | — | — | — | not re-driven (M52 drove it at two doors; nothing in this arc touches `JIGC_PACK_DIR` resolution). Stated in §4 |

---

## 4. What I did NOT drive, stated plainly

- **A project *pack* (as opposed to the project config layer) listed in `packs.yaml`.** Reaching it
  means hand-writing `.jigc/config/packs.yaml` plus a pack tree, and the brief's rule is to build
  fixtures with the binary. The **same seam** is driven instead, both directions, at row 45 and §3.2
  (the config layer and `packs.yaml` are both read from `<jigc_home>/.jigc/config`, and the malformed
  file is seen from every cwd). The pack **listing** cell is therefore not driven.
- **`pack-resource-missing` × this axis's doors** — not re-driven; M52 drove it at `start` and
  `describe` and nothing in the cwd arc touches `JIGC_PACK_DIR` resolution.
- **C1 end-to-end to a landed commit for 8 of the 20 composed workflows.** Driven to a real outcome:
  `single-task` (from a subdirectory, to a commit), `implement-from-spec`, `fix-task` (to a commit),
  `sub-task` (×4, to a milestone boundary), `record-decision` (to a promoted adr, twice, once under a
  spaced `docs-root`), `quick-fix` (minted from a linked worktree), the 12 `migrate-*` (to a mint, and
  `migrate-adr` to the exit-4 hold), plus `router`/`ingest-existing`/`increment` compose-only by design.
  For `architecture-documentation`, `decided-task`, `dev-task`, `do-research`, `form-vision`,
  `park-idea`, `project-setup`, `plan`, `planning`, `completion`, `record-change` and `record-dogfood` I
  composed each and checked **mechanically** that every `jigc <token>` in the text is a real clap leaf
  (row 18); their chains were **not** run to a commit.
- **C4 at `describe` for the 34 narration bodies and 27 of the 31 command hints.** Four hints were
  driven against behaviour (row 34); all 22 off-catalog reasons were driven for presence and partition
  (row 35); the rest were not.
- **`migrate … --approve`** — row 25 stops at the declared exit-4 hold, which *is* the cell (*nothing
  committed*). The approve arm is axis 7's.
- **`task diff`'s pinned envelope and the `PATH_ARG_OCCURRENCES` bases** — the brief assigns those to
  axis 5. Not driven here.
- **`ROLLBACK_POPULATIONS`, `TASK_AREA_FILES`, `DESTROYING_DOORS ▸ Disposition`, the `InProgress`
  family beyond `bisect`** — none intersects a composed surface; axes 2/3/4's subject.
- **The Codex source pass for axis 6** — deliberately not read (per brief).

---

## 5. Defects

### DEFECT A6-R1 (MEDIUM) — the installed pre-commit hook now calls *any* aimed `git` span a rename, and prints, on every commit, that an out-of-band rename exists when none does

**This is a regression the cwd arc introduced**, and it is the one new defect this axis found.

**Contract contradicted.** `design/surface-contract.md` → **Law 1 — nothing lies**, applied to a
committing door's own backstop. The hook's sentence asserts a fact about the repository (*"an
out-of-band managed-doc rename exists in the committed tree"*) that is false, and its parenthetical
(*"not staged in this commit"*) is false in the second cell below, where the move **is** staged in the
commit being made.

**The mechanism, read at HEAD.** The hook's rename block (`crates/cli/src/setup.rs:620`, the
`PRECOMMIT_RENAME_BLOCK` template) extracts candidate rename routes from the `jigc validate --format
json` report:

```
moves="$(printf '%s' "$report" | grep -o 'git -C [^`]*')"      # HEAD (rc.19)
moves="$(printf '%s' "$report" | grep -o 'git mv [^`]*')"      # HEAD~ of 65de53f5 (rc.18 and before)
```

`65de53f5` (*fix(route): every operator-facing `git` span names the checkout it runs in*) rendered the
`reconciliation.rename` revert route as `git -C <abs> mv <new> <old>`, so the grep had to widen. It was
widened to **`git -C`**, which is now the prefix of **every operator-facing git span in the whole
report** — `home-vacated`'s `git -C <abs> show <sha> -- <path>`, `owner-artifact`'s `git -C <abs> add`,
`file_state`'s restores. The gate `[ -n "$moves" ]` therefore passes for reports that contain no rename
route at all; the awk then finds no `mv` pair and falls through to the `else` branch, which prints the
sentence unconditionally.

**Cell 1 — a committed *deletion* makes every later commit lie. Driven, on a clean corpus.**

```
# rig: committed-singletons (fresh; `jigc validate` exits 0 and the first commit is silent)
jigc validate                                   # 0
printf 'a\n' > f1.txt; git add f1.txt; git commit -m "unrelated 1"     # 0, stderr EMPTY  (the control)

git rm -q docs/roadmap.md
git commit -m "delete the roadmap out of band"  # 0
> jigc: an out-of-band managed-doc rename exists in the committed tree — run `jigc validate` for
>   details (not staged in this commit; commit not blocked).

printf 'b\n' > f2.txt; git add f2.txt
git commit -m "unrelated 2"                     # 0   <-- NOTHING renamed, NOTHING relevant staged
> jigc: an out-of-band managed-doc rename exists in the committed tree — …          <-- and again,
>                                                                    on every commit from here on.
```

and the report the hook read, in full — **one** `git -C` span, and it is a `show`:

```
jigc validate --format json     # exit 1
  reconciliation.rename        route: restore docs/roadmap.md, or confirm the deletion by dropping it
                                      from the index: `jigc unmanage docs/roadmap.md`     <- NO `mv`
  schema-conformance.home-vacated
                               route: restore the document at `docs/roadmap.md` … (`git -C <ABS>
                                      show cc1b102 -- docs/roadmap.md`)                   <- the match
grep -c 'git mv'  <report>  →  0        # what the pre-M53 pattern would have matched: nothing
grep -o 'git -C ' <report>  →  1        # what HEAD's pattern matches
```

**Cell 2 — a *staged* placement-doc `git mv` is told it is not staged. Driven, two homes.**

```
# rig: committed-singletons (fresh)
git mv docs/roadmap.md docs/plan.md
git commit -m "bare git mv of a placement doc"  # 0
> jigc: an out-of-band managed-doc rename exists in the committed tree — … (NOT STAGED IN THIS
>   COMMIT; commit not blocked).            <-- it is staged in this commit, and it just landed

# same, at a root-level placement home:
git mv VISION.md THESIS.md ; git commit -m "…"  # 0, the same false sentence
# same, under a spaced placement-root (`jigc config set placement-root "my root"`):
git mv "my root/roadmap.md" "my root/plan.md" ; git commit -m "…"   # 0, the same false sentence
```

**Controls, driven — the genuine block is intact, and the space axis really is closed.**

```
# a LOCATION doctype at the DEFAULT docs-root:
jigc start --workflow record-decision "pick ring buffers" → … → jigc task finalize
  → promoted docs/decisions/ring-buffers.md
git mv docs/decisions/ring-buffers.md docs/decisions/rings.md
git commit -m "bare git mv, default docs-root"      # rc=1
> jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc
>   identity tracking; use `jigc rename` instead (commit blocked).

# the same under a SPACED docs-root (`jigc config set docs-root "my docs"`), the cell the
# confirmation pass's MEDIUM 1 closed:
  → promoted my docs/decisions/use-ring-buffers.md
git mv "my docs/decisions/use-ring-buffers.md" "my docs/decisions/new-cache.md"
git commit -m "…"                                   # rc=1, RENAME_BLOCK on stderr   <-- CLOSED
```

**Severity.** Surface-tier, **not tier 1**: no byte dies, no commit is wrongly blocked, and the
correct block still fires on every location-doctype rename including under a spaced `docs-root`. What
fails is law 1, on the one artifact jigc installs into the developer's own commit path, in the
**fail-loud-and-wrong** direction — it names a rename that does not exist, and in cell 2 denies one
that does, on every commit until the underlying finding is cleared.

**The axis, derived rather than reported.** The class is *every finding whose route carries an aimed
`git` span and is not a rename revert*. At HEAD that is at least `schema-conformance.home-vacated`
(three git-observable removal states), `owner-artifact.present`'s untracked cause, `finalize`'s
carried/stale `git restore --staged` siblings, `file_state`'s out-of-band-rename `git mv`/`git
checkout` arms and `milestone`'s record-conflict restore — i.e. most of census 1. The smallest correct
predicate is the one the pre-M53 grep had: match the **`mv` subcommand**, not the `-C` prefix
(`grep -oE 'git -C [^\`]* mv [^\`]*'`, or keep the wide grep and move the `[ -n "$moves" ]` gate
**after** the awk so the `else` branch is only reached when a rename route was actually present).

---

## 6. Observations and leads (driven, not defects — no stated contract is contradicted)

1. **O-1 — a store door that binds jigc_home prints a repo-relative path the reader's own checkout also
   has.** Driven: from a fan-out worktree, `jigc config set invocation-log true` acks *"written to
   `.jigc/config/`"* while the write lands in the **main** checkout; the standing worktree has its own
   `.jigc/config/` (holding `.gitkeep` + `packs.yaml` and **not** the written `manifest.yaml`). Same
   shape at §3.2's pack fault: *"`.jigc/config/packs.yaml` is not a valid pack-set list"* read from a
   **linked** worktree whose own copy is valid. This is law 1's declared convention (repo-relative
   against jigc_home, *"not portable across the two checkouts a fan-out is made of"* being the reason
   the locus stays relative), and `render::InstallSite` — the sentence that names the site when the
   reader stands elsewhere — is scoped **by its own doc-comment** to `jigc setup` / `jigc uninstall`,
   which names `jigc config set` as a jigc_home writer in the same breath. Recorded because the arc
   *created* this readership: before it, those doors acted on the checkout you stood in.
2. **O-2 — the route-span carve-out enumerates two span kinds; the arc minted a third.**
   `crates/cli/tests/support/route_spans.rs` redacts *"a backticked span whose first two tokens are
   `git -C`, and one whose first two are `jigc migrate` followed by an absolute operand"* and states
   that *"any other `` `jigc …` `` line"* stays subject to the full law-1 host-path scan. The arc's
   `cd <abs> && jigc workflow <W> --task <id>` span — two producers,
   `engine::compose::emit_fan_out_spawns` and `cli::start::blanket_base_pin_refusal` — is neither. It
   does not redden anything only because `repo_relative_paths.rs`' driven door table is scoped to the
   milestone destroying/provisioning doors and reaches neither producer. The **binary is right** (the
   rule and its reason are stated at the site, `compose.rs:1771-1793`); what is missing is that the
   disposition table and the carve-out do not carry the third kind, so the next producer that emits an
   absolute into a composed line is fenced by nothing.
3. **O-3 — `findings: unknown — <reason>` still prints an absolute host path**, on the text arm and
   inside the pinned JSON key `findings_unavailable` (producer `crates/cli/src/task.rs`, a `{:?}`
   format). Re-driven on rc.19, unchanged from M52's O-3; inside the declared `UNSWEPT_PRODUCERS`
   bound, which fences `.display()` only.
4. **O-4 — `milestone execute` in the partial state names one worktree two ways on one stdout.**
   Driven: locus `at: .jigc/worktrees/delta-area` three lines above ``Spawn: `cd
   /private/…/repo/.jigc/worktrees/delta-area && …` ``. This is the trade
   `design/surface-contract.md` takes **by name** (*"where the subject genuinely coincides — a fan-out
   worktree named in the message and aimed at in the route — the trade is taken with this reason"*),
   so it is not a defect; recorded so it is not re-derived as one, and because it is the exact shape
   M50 Increment 12 called *the break* in the other direction.
5. **O-5 — `jigc task bind`'s `task-bind.undeclared-role` flattens to `{"error": …}` under
   `--format json`** while the same door's `store.*` refusals ride the findings envelope. Driven, then
   **checked and cleared**: `design/validation.md`:887-888 declares both `task-bind.*` codes *"neither
   — un-keyed"*, and `ENVELOPE_OWED_CODES` (4 members, all `store.*`) is the obligation set. Not a
   defect; recorded so the divergence is not re-found.
6. **O-6 — the `suppressed:` guarantee is manifest-scoped.** Re-driven (row 36c): a manifest-less pack's
   `creates-task: false` workflow loads clean, `describe` narrates it with **no** reason and no
   `router_hidden`, and orientation lists it 0 times. Declared at `design/surface-contract.md`:127 /
   `design/introspection.md`:90. **STILL-OPEN, unchanged** (M52's OPEN lead 1).
7. **O-7 — `jigc migrate` mints silently over open work.** Driven with **15** tasks already live:
   `jigc migrate legacy/yet.md --as adr` → exit 0, `task minted:`, **0** `also open:` bytes; the very
   next `jigc start --workflow quick-fix "another one"` over the same state renders *"also open: 16
   other tasks were already open before this call"*. Declared since M52 at
   `design/write-commands.md`:189. **STILL-OPEN, unchanged** (M52's O-1).
8. **O-8 — the malformed-address refusal carries no code on either arm.** Driven: `jigc doc show
   padding` → *"malformed address `padding`: missing ':' between type and slug …"* with no `blocking ·
   <code>` prefix, and `--format json` → `{"error": …}`. The declared flatten posture
   (`design/command-output-contract.md`:294). **STILL-OPEN, unchanged** (M52's O-2).

### 6.1 Census / review rows this axis re-drove and found CLOSED on rc.19

Recorded here because the brief asks each cwd-arc finding to be verified closed, and these are the ones
that live on a composed surface:

| row | source | status on rc.19 | the drive |
|---|---|---|---|
| C1-14 / C3-01 — the `Spawn:` `cd` ran only from the repo root | cwd-census rank 3 | **CLOSED** | rows 21/22 — absolute, and quoted on spaced / `'`+`#` roots; `sh -c` rc=0 from four cwds |
| C2-06 — `jigc milestone finalize` unreachable from any worktree | cwd-census rank 2 | **CLOSED** | row 27 — the whole chain from inside the worktree lands `77ca9c7` |
| C2-11 — the base-mismatch gate did not fire from a worktree | cwd-census | **CLOSED** | row 32 — byte-identical exit-3 refusal from both |
| C2-03 / C1-03 — `migrate <PATH>` root-based, its route half-runnable | cwd-census rank 4/5 | **CLOSED** | row 23 — cwd-based token, aimed `git -C` half, both run verbatim from `docs/deep` |
| the pack loader's seventh walk-up (the project pack-set vanishing in a worktree) | VERDICT addendum 2 | **CLOSED** | row 45 + §3.2 — knob and malformed-`packs.yaml` both bind jigc_home from all four cwds |
| C2-07 / review LOW 10 — `uninstall` from a worktree half-uninstalled and said otherwise | cwd-census rank 8 | **CLOSED** | §6.2 |
| confirmation-pass MEDIUM 1 — the hook's operand axis under a spaced `docs-root` | audit/cwd-fix-code-review-2 | **CLOSED (its own axis)** | §5 controls — `git commit` rc=1 with `RENAME_BLOCK` under `docs-root "my docs"` |
| confirmation-pass MEDIUM 2 — the install-site line in the fan-out cell | audit/cwd-fix-code-review-2 | **CLOSED** | §6.2 |
| confirmation-pass LOW 7 — `uninstall` left git's worktree admin stale | audit/cwd-fix-code-review-2 | **CLOSED** | §6.2 |

### 6.2 `jigc uninstall` from inside a fan-out worktree — the three closures in one drive

```
# rig: fresh + a milestone with one provisioned sub-task worktree `gamma-area`
git worktree list           →  2 entries ;  ls .git/worktrees  →  gamma-area
cd "$REPO/.jigc/worktrees/gamma-area" && jigc uninstall        # rc=0
> jigc uninstall — repo-local install removed
>   - removed .jigc/
>   - pruned git's worktree registrations for the fan-out worktrees `.jigc/` held    <- LOW 7 closed,
>                                                                                      and narrated
>   … (bootstrap reference, allowlist, SessionStart hook, deny floor, pre-commit hook, guide artifact)
>   removed at `/…/repo` — the main checkout this repository's jigc install and `.jigc/` workbench
>     bind to, AND THAT WORKBENCH HELD THE WORKTREE YOU ARE STANDING IN, WHICH THIS REMOVED
>                                                                                    <- MEDIUM 2 closed
> [stderr] warning: removing `.jigc/` also removes 5 tracked file(s) under it: …
>   note: each is in the index, so `git -C /…/repo checkout -- <path>` brings it back.   <- aimed
after:  ls .git/worktrees → (empty) ;  git worktree list → 1 entry ;  <main>/.jigc gone ;  hook gone
```

---

## 7. What this adds over flow-54 arm N

M53's acceptance design ships **five** arms and **none of them is this axis's**: arm 1 is the milestone
area's foreign-byte disposition, arm 2 the rollback/unwind matrix, arm 3 the residual-area class, arm 4
the four new `GitState` variants × `BEHALF_DOORS`, arm 5 `MINT_DOORS` × unslugable titles. The composed
surface is pinned by **flow-53 arm 7** (M52 — *the composed doors*: the verb-routed set derived from
both packs' `suppressed` blocks × the two compose doors, plus the empty-enumeration set) and by
**flow-53 arm 6** (`PRE_DISPATCH_FAULTS × VERB_KINDS`, `ENVELOPE_ARMS`). Neither moved in M53, and the
cwd arc shipped **no** new acceptance arm at all — its proof is `cwd_verb_subject.rs`,
`git_span_aim.rs`, `path_arg_occurrence_axis.rs`, `precommit_hook_acceptance.rs` and
`repo_relative_paths.rs`, which are seam suites, not done-picture walks.

This review drives the half those arms structurally cannot reach, in four ways:

1. **It varies the cwd under a fixed argv, which no arm does.** Every suite above builds its fixture
   and drives from one place. A6-R1, C2-06's closure, C2-11's closure and the row-45 pack-seam
   confirmation all exist only because the *same* argv was run from four directories and two
   shell-hostile roots. The arc's own e2e is headless by construction; `repo_relative_paths.rs`' door
   table is keyed by **door**, not by cwd, so a door that answers correctly from the root and wrongly
   from a worktree is invisible to it.

2. **It follows the emitted bytes into a shell instead of asserting their text.**
   `precommit_hook_acceptance.rs` drives the shipped awk over a quoting axis; it does not run
   `git commit` over a **placement** doc or over a repository whose validate report carries a
   non-rename aimed span — which is exactly where A6-R1 lives. Likewise rows 21/22 do not assert that
   the `Spawn:` line *contains* an absolute; they `sh -c` it from four cwds on three different root
   spellings, and row 27 then runs the composed walk it starts to a **landed boundary commit**.

3. **It derives the axis's complement rather than its membership.** A6-R1 is not a defect in any span
   the fences enumerate — it is a defect in what the **consumer** of those spans now matches, and the
   consumer is a shell script embedded in a Rust literal that no `Route` fence, no `GIT_SPAN_SITES`
   row and no `ENVELOPE_ARMS` row covers. Finding it required reading the *old* grep out of
   `65de53f5^` and asking what the new one also matches. O-2 is the same shape one layer up: the
   carve-out enumerates two span kinds and the arc minted a third.

4. **It crosses two surfaces the arms measure separately.** Rows 12/13/35/44 assert agreement where it
   holds — orientation's catalog **is** the router's (empty `diff`), all 12 one-liners **are** their
   `create-gates:`, 34 = 22 + 12 with 22/22 reasons non-empty, four JSON key sets unmoved — and rows
   11/14 record the two places where M52 found it broken and it is still broken. An arm keyed per door
   cannot see a divergence between two doors.

**Net for this axis on rc.19:** M52's four §A rows are all **STILL-OPEN as triaged** (no fix was built
for them) and all reproduce; M51's three closures and all six of M52's Codex claims still hold; the
cwd arc's own composed-surface promises — the quoted absolute spawn line, the worktree-reachable
milestone boundary, the jigc_home-bound pack set, the aimed migrate route, the uninstall site line and
prune — are **CLOSED, driven, including on a spaced and an apostrophe-bearing repository path**; and
**one new defect**, A6-R1, a law-1 regression the arc introduced in the installed pre-commit hook.
**Zero data loss, zero corruption, zero tier-1 rows on this axis.**

---

## 8. Doors covered

Every clap leaf that is the door of ≥1 **driven** row above, in `VERB_KINDS` spelling. A
classification-only row confers no coverage.

| leaf | rows |
|---|---|
| `start` | 1–16, 31; R-D1, R-D2, R-D3 |
| `workflow` | 3, 7, 17–22, 31 |
| `setup` | 36 (the `setup.pack-load` advisory over each manufactured pack, emitted by the installed binary at rig construction); §5 (the installed hook is `setup`'s artifact) |
| `uninstall` | §6.2 |
| `migrate` | 23–26, 31; O-7 |
| `describe` | 33–37 |
| `validate` | §5 (the report the hook reads, driven for its own sake at three states) |
| `doc create` | 1, 25; §5 controls |
| `doc set-field` | 1, 4, 25, 27; §5 controls |
| `doc set-slot` | 1, 4, 25, 27; §5 controls |
| `doc show` | 1, 38–40; O-8 |
| `doc list` | 1, 41, 44 |
| `task list` | 3, 4, 17; R-D2 |
| `task validate` | 42–44; R-D1 |
| `task finalize` | 1, 4, 25, 27; §5 controls; R-D2 |
| `task bind` | O-5 (four refusal drives) |
| `config set` | 42, 45; §5 (`docs-root "my docs"`, `placement-root "my root"`) |
| `config get` | 45 |
| `config list` | 45 |
| `milestone create` | 27–30, 32; R-D4 |
| `milestone add-task` | 27–29, 32 |
| `milestone list-tasks` | 29 |
| `milestone provision` | 27–29, 32, 34; §6.2 |
| `milestone execute` | 21, 22, 27–32 |
| `milestone join` | 27, 32, 34 |
| `milestone finalize` | 27, 32; R-D4 (`milestone.zero-contribution`) |

**26 leaves driven to behaviour.** The other 21 `VERB_KINDS` leaves — `upgrade`, `ingest`,
`migrate-corpus`, `unmanage`, `rename`, `relocate`, `doc add-item`, `doc remove-item`,
`doc retitle-item`, `doc rename`, `doc author`, `doc schema`, `task diff`, `task discard`,
`config insert-step`, `config replace-step`, `config remove-step`, `config fill`, `config fork`,
`milestone add-from-spec`, `milestone discard` — are other axes' subjects and were not reached here.

---

# Reconciliation ledger — AXIS 6 · composed surfaces

**Reconciler.** A third agent, holding both the Opus driver table above (§1–§8, reproduced
**unchanged**) and the Codex source pass
(`scratchpad/axis-review-rc19/codex/axis6-codex.md`). Same binary, asserted before anything
else: `/Users/maurice/.local/bin/jigc --version` → **`jigc 1.0.0-rc.19`**, rc=0. Same rig
discipline (`rig=$(SCRATCH=<dir> dev/jigc-rig <state> --binary …) || exit; eval "$rig"`, two-step
eval, `mktemp -d` roots, no teardown, no `rm -rf` on a variable path). **Seven rigs built here**
— four `fresh`, three `committed-singletons`, one of them under a `SCRATCH` root carrying a space
(`…/recon-work/jigc space/…`). Nothing written into the working repository; no fix, no commit, no
repo edit.

**The rule applied** (`acceptance-design.md` → *The reconciliation rule*): a claim by one party
that the other cannot reproduce is a **lead**, not a finding. Every Codex claim was entered as
`lead(codex, …)` and then **driven** — to a repro block (CONFIRMED, origin codex), to a recorded
refutation with the falsifying datum (REFUTED), or, where the state cannot be built in this
environment, left **OPEN** with the reason. Every driver defect was **re-driven here** before its
status was carried.

## A. Demotions — driver rows marked driven that carry no repro

**None.** Every one of the 45 rows in §3 carries an argv, an exit code and the asserted surface
inline — that *is* its repro — and the driver's artifact tree
(`scratchpad/axis-review-rc19/driver/work/`, **336** entries: 9 rig roots including the two
shell-hostile ones, plus per-drive `.out`/`.err`/`.json` captures) backs them. I re-drove a
**12-row sample** independently (rows 3, 7, 11, 12, 19, 21, 22, 29, 30, 31, 41 and the §5 cells)
and every one reproduced. Two coverage qualifications, recorded rather than demoted:

1. **`setup`'s coverage is by fixture construction, not by a table row.** The driver's §8 credits
   `setup` to row 36 and §5. Row 36 is honest — the manufactured packs are installed by the
   *installed binary's own* `jigc setup`, and its `setup.pack-load` fence is what rows 36(a)/(b)
   assert. §5's hook is `setup`'s artifact, but §5 drives `git commit`, not `setup`. Coverage
   stands; the ground is row 36.
2. **§2's `A6-3` second half was a source read in the driver, and is now driven.** The driver wrote
   that `jigc task bind`'s `store.not-found` / `store.unknown-type` "still ride the findings
   envelope (**checked at the constructor and through `ENVELOPE_OWED_CODES`**)" — a read, not a
   drive, inside a "re-verified holding" bullet. Driven here (§C, R-6): both ride the envelope.
   **Upgraded, not demoted.**
3. §3.2's third row (`pack-resource-missing`) and every item in §4 are **declared** not-driven by
   the driver itself. They confer no coverage and are not demotions.

## B. Codex claims

### CLAIM 1 — the non-UTF-8 root escape hatch silently restores the cwd-dependent relative `Spawn:` form → **OPEN LEAD**

*Codex:* production `milestone execute` builds the absolute worktree from `jigc_home`, but when
`Path::to_str()` returns `None` it calls `SubTask::new` (`milestone.rs:2677`), which stores
`worktree: None` (`data_value.rs:232`), and the fan-out renderer then emits the repo-relative
`.jigc/worktrees/<id>` (`compose.rs:1779`) — a `cd` that runs from the repository root and nowhere
else. Confidence high from source; runtime confirmation needed.

**Status: OPEN LEAD — the trigger state cannot be built in this environment.** The reason, driven:

```
# the filesystem refuses the name the claim needs
python3 -c "import os; os.mkdir(b'nonutf8-\xff-dir')"
  → OSError: [Errno 92] Illegal byte sequence: b'nonutf8-\xff-dir'     # APFS, EILSEQ

# every mount on this machine is APFS (one SMB share aside)
mount | grep -c apfs   → 8

# and the alternate-filesystem escape is denied to this session
hdiutil create -size 20m -fs "MS-DOS FAT32" -volname NUTF <dir>/nutf
  → hdiutil: create failed - Operation not permitted
```

A repository whose pathname carries a non-UTF-8 byte is therefore unbuildable here. **Never
promoted on the source read, never dropped.** Three things I *could* drive, which bound the lead
and are what the next reviewer should start from:

1. **The source leg is exact, and the branch is uniquely reachable.** `SubTask::new` /
   `SubTask::in_worktree` over `crates/`:

   ```
   crates/cli/src/milestone.rs:2674   SubTask::in_worktree(id, workflow, path)   # production
   crates/cli/src/milestone.rs:2677   SubTask::new(id, workflow)                 # production — the branch
   crates/cli/src/milestone.rs:7547,7548,7591,7592   # unit tests
   crates/engine/src/compose.rs:8937,9124            # unit tests
   crates/engine/src/data_value.rs:1122,1123         # unit tests
   ```

   So `compose.rs:1779`'s repo-relative fallback is reachable in **production** only through
   `Path::to_str() == None`. There is no second production producer of a `worktree: None` sub-task.

2. **The one drivable neighbouring degradation does *not* trigger it.** `recorded_workflows`
   canonicalizes and keeps the raw path on failure (`.canonicalize().unwrap_or(worktree)`), so an
   **unprovisioned** sub-task — the state where canonicalization fails — still emits an absolute:

   ```
   # rig: fresh · milestone fan-chain with alpha/beta provisioned, gamma added after provision
   jigc milestone add-task fan-chain "gamma work"      # 0
   jigc milestone execute fan-chain                    # 0
   > advisory · milestone.worktrees-partial — 2 of 3 sub-task worktrees are provisioned —
   >   sub-task `gamma-work` has none (nothing is there) …
   > Spawn: `cd <REPO>/.jigc/worktrees/alpha-work && jigc workflow sub-task --task alpha-work`
   > Spawn: `cd <REPO>/.jigc/worktrees/beta-work  && jigc workflow sub-task --task beta-work`
   > Spawn: `cd <REPO>/.jigc/worktrees/gamma-work && jigc workflow sub-task --task gamma-work`   <- ABSOLUTE
   ls .jigc/worktrees  →  alpha-work  beta-work        # gamma's directory does not exist
   ```

   The absolute guarantee survives the degradation that *is* reachable. (This also re-drives the
   driver's row 29 — advisory on stdout, before the walk, naming the locus — CONFIRMED.)

3. **The "contradicts the adjacent contract" leg is partly falsified by the cited lines
   themselves.** `data_value.rs`'s `worktree` doc-comment declares the fallback in the same breath
   as the guarantee — *"**The absolute path of the worktree the spawn line `cd`s into**, when the
   caller knows it — `None` falls back to the repo-relative [`worktree_path`] convention"* — and
   `milestone.rs:2675-2676` states the degradation by name: *"A non-UTF-8 worktree path cannot ride
   a composed line at all; the repo-relative convention is what the emit falls back to."* So what is
   open is **a declared degradation that is silent at runtime** (nothing is printed, no advisory is
   raised, the emitted line looks ordinary), not an undeclared contract contradiction. The lead
   should be re-posed on that axis — *should a degraded `Spawn:` line say it is degraded?* — on a
   machine whose filesystem admits the name.

### CLAIM 2 — "No other grounded completeness defect was found" (the pass's blanket negative) → **REFUTED**

**Falsifying datum: A6-R1, re-driven here in full (§C-1).** The driver's one new defect lives in
the shipped **pre-commit hook** — a shell script embedded in a Rust string literal
(`crates/cli/src/setup.rs`, `PRECOMMIT_RENAME_BLOCK`) whose *consumer* behaviour, not whose text,
is wrong. This is recorded as a refutation of the blanket negative rather than of a targeted
denial: the pass's own stated bound is *"This pass establishes source completeness, not runtime
prose/help equivalence"*, and the defect is exactly a runtime-consumer fact. Read at HEAD, the
grep the pass would have had to evaluate:

```
crates/cli/src/setup.rs (installed at .git/hooks/pre-commit, line 41):
  moves="$(printf '%s' "$report" | grep -o 'git -C [^`]*')"
```

### CLAIM 3 — A6-1 CLOSED (argv): all three `resume:` states remain expressible → **CONFIRMED**

Driven here, four cwds, one sub-task, HEAD off the pin:

```
# rig: fresh · milestone fan-chain, alpha-work + beta-work provisioned at base 67d24a0, main at c324cc0
cd .jigc/worktrees/beta-work  && jigc workflow sub-task --task beta-work     # 0
> resume: `jigc workflow sub-task --task beta-work` — … you are in this sub-task's worktree at
>   `.jigc/worktrees/beta-work`, where its work happens — run it here
cd .jigc/worktrees/alpha-work && jigc workflow sub-task --task beta-work     # 0
> resume: … this checkout is not that worktree — run it from this sub-task's own worktree at
>   `.jigc/worktrees/beta-work`, where its work happens
cd <REPO>        && jigc workflow sub-task --task beta-work                  # 1
cd <REPO>/docs/deep && jigc workflow sub-task --task beta-work               # 1
> task `beta-work` is pinned to base 67d24a0 but you're on c324cc0 — … run this from that
>   worktree — `cd /private/…/.jigc/worktrees/beta-work`
cmp <root stderr> <docs/deep stderr>   → identical      # byte-identical across the two off-pin cwds
```

Agrees with driver row 7. The `Spawn:`-production half Codex flagged "subject to Claim 1" is
driven CONFIRMED independently (§C-2).

### CLAIM 4 — A6-2 CLOSED (argv): named composition refuses verb-routed workflows → **CONFIRMED**

```
# rig: fresh
jigc start --workflow milestone-execution     # 1
> blocking · workflow.verb-routed — workflow `milestone-execution` is not composed by name:
>   composes a degenerate walk — zero `Spawn:` lines and an unresolved `<MILESTONE_ID>` off-verb,
>   so a router pick could never land
>   at: workflow:milestone-execution
>   route: `jigc milestone execute <milestone-id>`
jigc start --workflow router                  # 0        <- the control
```

Agrees with driver row 3 (the full 14-member `suppressed.door` set × 3 argv forms = 42 drives,
42 exits of 1). I re-drove one member and the control; the remaining 41 rest on the driver's row.

### CLAIM 5 — A6-3 CLOSED (datum): the empty-case clauses stand → **CONFIRMED**

Both halves driven. The `implement-from-spec` half agrees with driver row 2. The `task bind` half
— the one the driver established by a **source read** — is driven here (R-6):

```
# rig: fresh · jigc start --workflow implement-from-spec "build the thing"  → task minted
jigc task bind spec spec:nope build-the-thing            # 1
> blocking · store.not-found — no committed doc `spec:nope` to bind (expected at `docs/specs/nope.md`) …
jigc task bind spec spec:nope build-the-thing --format json   # 1
> stdout EMPTY; stderr {"schema_version":3,"findings":[{… "code":"store.not-found",
>   "key":{"code":"store.not-found","target":"spec:nope"} …}]}          <- the findings envelope
jigc task bind spec nosuch:x build-the-thing --format json    # 1
> … "code":"store.unknown-type","key":{"code":"store.unknown-type","target":"nosuch"} …
# and the divergence O-5 records, driven on the same door:
jigc task bind decision adr:nope posture-parity --format json  # 1
> stderr {"error":"blocking · task-bind.undeclared-role — …"}          <- flattened, as declared
```

**One qualification the driver's O-5 does not carry:** the role check fires **first**, so the
`store.*` arms are unreachable on a workflow that declares no read role (`single-task` →
`task-bind.undeclared-role — … (declared: none)`). The envelope claim holds only for a task whose
workflow declares the role — `implement-from-spec`'s `spec`. Not a defect; the ordering is the
door's own precedence.

### CLAIM 6 — catalog membership derives from `creates-task && selectable`; `describe` projects the reason through one predicate; no catalog/help contradiction → **CONFIRMED**

```
# rig: fresh
jigc start           | orientation "Available workflows:" block  →  12 entries
jigc start --workflow router | the selectable listing            →  12 entries
diff <(orientation, indent-stripped, sorted) <(router, sorted)    →  0 bytes, rc=0
```

Agrees with driver rows 12 and 35 (34 = 22 + 12, 22/22 hidden rows carrying a non-empty
`router_hidden`). **Bound carried:** the guarantee is manifest-scoped — driver row 36(c) / O-6
shows a manifest-less pack's `creates-task: false` workflow loading clean with **no** reason. That
is a declared scope (`design/surface-contract.md`:127), not a contradiction of this claim, and it
stays an OPEN lead in §D.

### CLAIM 7 — orientation retains unset / clean / active-task arms; no unavailable-findings sweep is converted to `none` → **CONFIRMED**

The active-task arm driven here (`state: "active-task"`, `tasks[0].findings` = three codes, keys
`['header','next_steps','schema_version','state','tasks','workflows']`); the unset arm and the
`findings: unknown` arm rest on driver rows 8 and 10, both of which carry argv + exit + the
asserted JSON.

### CLAIM 8 — every executable `jigc …` form in both step trees is a clap leaf with the named flags; no stale verb or flag → **CONFIRMED**

Rests on driver row 18 (3026 composed lines → 30 distinct forms → 25 real clap leaves + 5 prose
fragments, each driven `--help`), whose artifacts are `work/allcomposed.txt` and `work/forms.txt`.
Not independently re-derived here.

### CLAIM 9 — the four `render::composed` producers and their registry rows are present; the read-back owe-set carries `describe`, every `doc show` arm, `doc list`, `task validate` → **CONFIRMED**

Agrees with the driver's §1 render-seam derivation (**8** render sites over **4** clap leaves —
the driver reads six `start`/`workflow` sites where Codex names the dispatch functions; same set,
finer granularity) and with driver row 31 (all 6 composed render forms × `--format json` are
exactly `['task','text']`), which I re-drove on four of the six:

```
jigc start --format json --workflow router          # 0  keys=['task','text']  task=None
jigc workflow single-task --preview --format json   # 0  keys=['task','text']  task=None
jigc milestone execute fan-chain --format json      # 0  keys=['task','text']  task=None
jigc start --task build-the-thing --format json     # 0  keys=['task','text']  task='build-the-thing'
# `resume:` present on the text arm, absent from every JSON arm's `text`
```

### CLAIM 10 — zero schema-hash movement: neither frozen manifest changed across the cwd arc → **CONFIRMED**

```
git log --oneline 271b0cb7..HEAD | wc -l                  → 35        # rc.18 → rc.19, the cwd arc
git diff --stat 271b0cb7..HEAD -- crates/cli/pack/config/schema-manifest.yaml \
                                  packs/methodology/config/schema-manifest.yaml
  → 0 bytes of output                                                 # no movement
```

## C. Driver defects

### C-1 · DEFECT A6-R1 (MEDIUM) — **CONFIRMED, re-driven in full**

The source pass is silent on it (see Claim 2). Re-driven here on a fresh rig, both cells, plus
both controls. Every line below is this reconciler's own run, not a copy of the driver's.

**Cell 1 — a committed deletion makes every later commit lie.**

```
# rig: committed-singletons (fresh)
jigc validate                                        # 0
printf 'a\n' > f1.txt; git add f1.txt
git commit -m "unrelated 1"                          # 0, stderr 0 bytes            <- the control
git rm -q docs/roadmap.md
git commit -m "delete the roadmap out of band"       # 0
> jigc: an out-of-band managed-doc rename exists in the committed tree — run `jigc validate`
>   for details (not staged in this commit; commit not blocked).
printf 'b\n' > f2.txt; git add f2.txt
git commit -m "unrelated 2"                          # 0   <- nothing renamed, nothing relevant staged
> jigc: an out-of-band managed-doc rename exists in the committed tree — …    <- and on every commit after
```

and the report the hook read:

```
jigc validate --format json                          # 1
grep -c 'git mv'  <report>   → 0        # what the pre-M53 pattern matched: nothing
grep -o 'git -C ' <report>   → 1        # what HEAD's pattern matches
the single span:  git -C /private/…/repo show e3c759a -- docs/roadmap.md     <- a `show`, not a rename
```

**Cell 2 — a staged placement-doc `git mv` is told it is not staged.**

```
# rig: committed-singletons (fresh)
git mv docs/roadmap.md docs/plan.md
git commit -m "bare git mv of a placement doc"       # 0
> jigc: an out-of-band managed-doc rename exists in the committed tree — … (NOT STAGED IN THIS
>   COMMIT; commit not blocked).
git show --name-status --find-renames --format= HEAD
> R100  docs/roadmap.md  docs/plan.md                 <- it IS staged in that very commit
# the same shape on the other fixed-identity singleton:
git mv docs/decisions-log.md docs/dlog.md ; git commit -m "…"   # 0, the same false sentence
```

**Controls — the genuine block is intact, at the default docs-root and under a spaced one.**

```
# a LOCATION doctype at the DEFAULT docs-root:
jigc start --workflow record-decision "pick ring buffers" → doc create adr → 3 × set-slot
  → set-field status=accepted → commit doc → jigc task finalize        # 0
  → promoted docs/decisions/use-ring-buffers.md
git mv docs/decisions/use-ring-buffers.md docs/decisions/rings.md
git commit -m "bare git mv, default docs-root"                          # rc=1
> jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc
>   identity tracking; use `jigc rename` instead (commit blocked).
git diff --cached --name-status --find-renames → R100 …/use-ring-buffers.md …/rings.md  (refused)

# the same under a SPACED docs-root — confirmation-pass MEDIUM 1's own cell:
jigc config set docs-root "my docs"                                     # 0
  → promoted my docs/decisions/use-ring-buffers.md
git mv "my docs/decisions/use-ring-buffers.md" "my docs/decisions/new-cache.md"
git commit -m "…"                                                       # rc=1, RENAME_BLOCK   <- CLOSED
```

**The mechanism, read at HEAD and quoted rather than paraphrased** (`.git/hooks/pre-commit`,
lines 41–42, installed by `jigc setup`):

```sh
moves="$(printf '%s' "$report" | grep -o 'git -C [^`]*')"
if [ -n "$moves" ]; then
        …awk scans the route's shell-words for an `mv` whose two operands are both staged…
        echo 'jigc: an out-of-band managed-doc rename exists in the committed tree — … (not staged
          in this commit; commit not blocked).' >&2
fi
```

The gate `[ -n "$moves" ]` is satisfied by **any** aimed git span in the report; the awk then finds
no `mv` pair and falls through to the unconditional `else` sentence. **The driver's severity call,
its derived axis (*every finding whose route carries an aimed `git` span and is not a rename
revert*) and its two proposed predicates are carried unchanged** — I confirmed only that the defect
reproduces and that the block it sits beside still fires.

### C-2 · the cwd arc's composed-surface closures — **CONFIRMED (re-driven)**

| closure | re-driven here | result |
|---|---|---|
| C1-14 / C3-01 — the `Spawn:` `cd` ran only from the repo root | `milestone execute` on a plain root; `sh -c "$SPAWN"` from the root, `docs/deep` and a **sibling** worktree | absolute operand; **rc=0 ×3**, `resume:` present, stderr 0 bytes |
| cwd-review HIGH 1 — an absolute operand on a shell-hostile root | the same on a root under `…/recon-work/jigc space/…` | emitted `` cd '/…/jigc space/…/.jigc/worktrees/alpha-work' `` — POSIX single-quoted; `sh -c` **rc=0** from the root and from `docs/deep` |
| confirmation-pass MEDIUM 1 — the hook under a spaced `docs-root` | §C-1's second control | `git commit` **rc=1** with `RENAME_BLOCK` |

### C-3 · the four M52 §A rows, all **STILL-OPEN as triaged** — each re-driven here

| row | re-driven | the datum |
|---|---|---|
| `(6, D-1)` orientation reports a live task's findings without the repository posture | yes | `git bisect start` (BISECT_LOG/NAMES/START present) → `jigc start` **0**, three findings, `grep -c 'operation-in-progress'` = **0** on text **and** on the pinned wire; `jigc task validate posture-parity` **1** with `blocking · repo.operation-in-progress — a bisect is in progress …`. Orientation's own route line promises it: *"previews part of the finalize gate: **the repository posture finalize refuses under**, …"* |
| `(6, D-2)` `fix-task` composable by name, its forbidden door the only one that works | yes | text line 25 *"Never `git commit` and never `jigc task finalize` here."* → fill `commit:fix-the-thing` → `git add fixed.txt` → `jigc task finalize fix-the-thing` **rc=0**, *"finalized 8c45315 — fix: fix the thing · added fixed.txt · 1 file committed"* |
| `(6, D-3)` the `Preview:` footer states the pre-M52 rule `milestone-execution` falsifies | yes | footer: *"a workflow that mints nothing has no preview — `jigc start --workflow <id>` **composes it directly, and mints nothing either**"*; `jigc start --workflow milestone-execution` **1 · workflow.verb-routed**; `--workflow router` **0** |
| `(6, D-4)` a legitimately-empty `milestone execute` walk does not state its empty case | yes | `milestone create "Empty probe"` **0**; `milestone execute empty-probe` **0**; `grep -c '^Spawn:'` = **0**; stderr **0 bytes**; no sentence names the empty case |

## D. Open leads carried out of this axis

| # | lead | why it stays open |
|---|---|---|
| L-1 | **codex claim 1** — a non-UTF-8 repository path silently restores the relative `Spawn:` form | the trigger state is unbuildable in this environment (APFS `EILSEQ`; `hdiutil create` *Operation not permitted*). Source leg verified exact and uniquely reachable; the drivable neighbour (unprovisioned worktree) does **not** trigger it; the "contradicts the adjacent contract" leg is partly falsified — the fallback is declared at both cited sites, so the open question is *silence*, not contradiction |
| L-2 | **O-6** — the `suppressed:` guarantee is manifest-scoped (driver row 36c) | driven and **STILL-OPEN, unchanged** from M52's open lead 1; declared at `design/surface-contract.md`:127 / `design/introspection.md`:90 |
| L-3 | **O-2** — the route-span carve-out enumerates two span kinds; the arc minted a third (`cd <abs> && jigc workflow …`) | the binary is right and states its reason at the site; what is missing is a fence, so the next producer of an absolute in a composed line is covered by nothing. Not a defect; carried |
| L-4 | **O-1** — a store door that binds jigc_home prints a repo-relative path the reader's own checkout also has | law 1's declared convention; the arc *created* the readership. Carried, not a defect |
| L-5 | **O-3 / O-7 / O-8** — the absolute host path in `findings_unavailable`; `jigc migrate` minting silently over open work; the malformed-address refusal carrying no code | all three driven and unchanged from M52, each inside a declared bound (`UNSWEPT_PRODUCERS` · `design/write-commands.md`:189 · `design/command-output-contract.md`:294) |
| L-6 | **`pack-resource-missing` × this axis's doors** | declared not re-driven by the driver (§4); nothing in the cwd arc touches `JIGC_PACK_DIR` resolution |
| L-7 | **a project *pack* listed in `packs.yaml`** (as opposed to the project config layer) | declared not driven by the driver (§4); the same seam is driven both directions at row 45 and §3.2 |

## E. Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling — the union of the
driver's §8 and this reconciliation's own drives. A classification-only row confers no coverage.

`start` · `workflow` · `setup` · `uninstall` · `migrate` · `describe` · `validate` ·
`doc create` · `doc set-field` · `doc set-slot` · `doc show` · `doc list` ·
`task list` · `task bind` · `task validate` · `task finalize` ·
`config set` · `config get` · `config list` ·
`milestone create` · `milestone add-task` · `milestone list-tasks` · `milestone provision` ·
`milestone execute` · `milestone join` · `milestone finalize`

**26 leaves.** The other 21 `VERB_KINDS` leaves — `upgrade`, `ingest`, `migrate-corpus`,
`unmanage`, `rename`, `relocate`, `doc add-item`, `doc remove-item`, `doc retitle-item`,
`doc rename`, `doc author`, `doc schema`, `task diff`, `task discard`, `config insert-step`,
`config replace-step`, `config remove-step`, `config fill`, `config fork`,
`milestone add-from-spec`, `milestone discard` — are other axes' subjects and were reached by
neither pass on this axis.

## F. Net

**One defect, origin driver: A6-R1**, re-driven here in full with both controls — a law-1
regression the cwd arc introduced in the installed pre-commit hook, surface-tier, **no byte dies,
no commit is wrongly blocked, and the genuine block still fires** including under a spaced
`docs-root`. **Zero claims from the Codex pass were promoted to findings**: one is an OPEN lead the
environment cannot reach, one blanket negative is refuted by A6-R1, and the remaining eight agree
with driven driver rows. **M52's four §A rows are all STILL-OPEN as triaged**, each re-driven here.
**M51's three closures hold.** The arc's own composed-surface promises are **CLOSED, driven,
including on a space-bearing repository path.** **Zero data loss, zero corruption, zero tier-1 rows
on this axis.**
