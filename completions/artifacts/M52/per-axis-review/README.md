# M52 — the eight per-axis review matrices (the re-run)

**What this is.** M52's acceptance instrument, the second of the two things its acceptance is made of.
[settle-record.md](../settle-record.md) → **D12** rules that the wave ships flow 53's arm set *and* a
**re-run of M51's eight-axis per-axis review**; [acceptance-design.md](../acceptance-design.md) →
*The per-axis review re-run* fixes its shape: the instrument preserved at
[M51/per-axis-review/instrument/](../../M51/per-axis-review/instrument/) — the eight Codex prompts and
the Workflow script — re-pointed at this directory, **staffed as before** (one Opus driver · one Codex
source pass · one reconciler per axis), driven on the **installed** binary after the completion audit's
fixes, with three obligations stated up front:

> **The comparison is row by row against M51's**: every §A row of M51's ledger is re-driven and recorded
> CLOSED (with the argv) or STILL-OPEN (with the datum); every new finding is tiered on the charter's
> predicate; the coverage table is diffed (**no leaf may lose an axis**).

This directory is that instrument's **persisted record**: what was driven, what it found, what it refused
to claim, and which of the 47 `VERB_KINDS` leaves each axis reached.
**M51's record at [M51/per-axis-review/](../../M51/per-axis-review/README.md) is the baseline and is not
edited by this re-run** — it is a dated record of what was driven on `1.0.0-rc.15`, and this file is a
dated record of what was driven on its successor.

**The binary.** Every row in every file here was driven on the installed release **`jigc 1.0.0-rc.16`**,
built from commit **`e519e4eb`** (*"docs(m52): completion audit closed — seven findings, seven fixed,
rc.16 stamped after"*), on **2026-09-21**. The version was asserted first, before anything else, in each
axis file. The binary is the **release** build, so the debug-only `debug_assert!` route fences do not
exist in it — a route-fence violation shows up here as a *bad emitted command*, never as a panic, which
is the posture an adopter's binary is in. Fixtures were built with [`dev/jigc-rig`](../../../../dev/jigc-rig)
(every root from `mktemp -d`, so nothing needed teardown and the `rm -rf $V/$D` shape never appears) and
by driving the binary; **nothing was written into the working repository**, no fix was applied, and no
commit was made by any agent in this review.

**An axis matrix row** is `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted,
verdict}`. A row is **driven** iff its argv ran on that binary and its verdict carries a **repro block**.
**A verb is covered iff it is the door of ≥1 driven row.** A classification-only row — a leaf proven by a
⇔ fence rather than by driving — confers **no** coverage, or the fence would measure a table instead of
the binary.

## The staffing

Per axis, three agents, unchanged from M51 §19:

- **One Opus driver** owns the `(door, cell)` table. It drives every row on the installed binary and
  records exit, code, route, surface and a repro block per row. **It may not mark a row driven from a
  source read.** Each driver was told not to read its axis's Codex pass, and each says so in its file.
- **One Codex source pass** ([codex/axis-N-source-pass.md](codex/), verbatim, so every lead is auditable)
  owns **completeness of the row set** — the question driving cannot answer: *is there a door the registry
  does not carry, or a bypass of the seam this axis is about?* It reads; it drives nothing.
- **One reconciler** (a separate Opus agent, which did **not** author the driver file it reconciles) merges
  the two: it re-drives every driver defect once, enters every Codex claim as a lead and drives it, and
  audits the driver's table for rows marked driven that carry no repro block (**demotion pass**).

## The reconciliation rule

> *A claim by one that the other cannot reproduce is a **lead, not a finding**.*

A Codex claim with no driven repro enters the table as `lead(codex, <claim>)` and is either **driven to a
repro block** — at which point it is a finding — or **recorded REFUTED with its falsifying datum**. An Opus
row the source pass says cannot happen **stays a finding** (it was driven), and the source claim is
recorded refuted with the datum that refutes it. Silence is not refutation. **No fix ships on a source read
alone, and no completeness claim ships on driving alone.**

The rule earned its keep again this run, in both directions and visibly:

- **Axis 3.** The Codex pass stated *"refusing doors guard foreign bytes before removal, while both
  finalizing doors **displace** them … in every area it removes"*. Two independent drives falsified it —
  `milestone finalize` destroys the **milestone's own** area's complement at exit 0, and both `Displace`
  doors remove the area when the displacement **failed**. The source claim is REFUTED with those repros;
  the driver's findings stand.
- **Axis 8.** The Codex pass read the help strings and reported *"no contradictory numeral"*. The binary
  prints *"**Four** states it refuses"* and *"Inert when all **three** guards are already clean"* in one
  `--help`, and `--force` in the state the smaller numeral describes is **not** inert. As axis 8's
  reconciler puts it: *a source read cannot see a predicate that is false only when run, which is the
  reconciliation rule's whole warrant.*
- **Axis 5, the other direction.** The Codex pass claimed a declared key set that the driver's 64-row sweep
  had not contradicted; driving a reachable pack composition showed `next_steps` **absent on the wire**
  while both registry rows declare it. A Codex lead became a finding by being driven, not by being read.

---

## Roll-up

| axis | subject | CONFIRMED | REFUTED | OPEN | doors covered (M51 →) |
|---|---|---|---|---|---|
| 1 | caller tokens | 17 | 0 | 5 | **40** (39) |
| 2 | posture | 4 | 1 | 0 | **47** (22) |
| 3 | destroying doors | 4 | 2 | 0 | **7** (6) |
| 4 | transaction / rollback | 14 | 1 | 4 | **16** (13) |
| 5 | pinned contracts | 15 | 1 | 0 | **47** (47) |
| 6 | composed surfaces | 13 | 2 | 9 | **24** (23) |
| 7 | freeze & migration | 8 | 2 | 9 | **10** (10) |
| 8 | adopter docs & help | 18 | 4 | 0 | **28** (23) |
| **total** | | **93** | **13** | **27** | **47 / 47 union · uncovered: none** |

**The counting basis differs per axis, and this is stated rather than smoothed.** The CONFIRMED/REFUTED/OPEN
triples above are **each reconciler's own roll-up as handed off**, and they do not all count the same kind
of thing: some axes count *ledger rows* (every Codex lead driven, every driver defect re-driven, every
observation carried), others count *findings* only. Two axis files' own internal roll-ups differ from the
hand-off figure by the same ambiguity — axis 6's roll-up table states `CONFIRMED 10` and then lists
fourteen members (ten Codex claims **plus** its four driver defects), and axis 1 reports its three groups
(13 Codex · 3 driver defects · 4 observations) without summing them. **The auditable set is the enumeration
below, not the table above**: §A lists every CONFIRMED entry that names a **defect or a contradiction**,
counted once, keyed `(axis, id)` — **27 entries** — and it is the set a next wave would be chartered on.
§B, §C and §D carry the positive completeness claims, the refutations and the open leads. Where a number
here disagrees with a number in an axis file, **the axis file governs**, because it carries the repro.

**The three headline facts of the re-run:**

1. **35 of M51's 39 §A rows are CLOSED**, each re-driven with its argv on rc.16. The four that are not are
   *partial*: two are a still-open **half** of a row whose other half closed (axis 2's `D3`, axis 4's `C4`),
   one is a **declared residual carried by decision on a written trigger** (axis 7's `C-2`), and one is the
   second half of axis 7's `D-4`. No M51 row regressed, and no closed row re-opened.
2. **27 new findings**, tiered on the charter's predicate: **4 tier-1** (exit-0 loss or repository harm
   through a committing or destroying door) · **8 tier-2** (posture and route dead ends) · **15 tier-3**
   (a surface says something the binary does not do). Two of the four tier-1 findings are **permanent,
   unrecoverable byte loss at exit 0**, both at a `Displace` door, and a third destroys a user's own
   authored commit message.
3. **The complete-fix lens lands on M52's own work for the sixth consecutive wave.** Axis 2's `DEFECT A`
   is M51's `D3` on an un-swept cell (the *clean* `--no-commit` cherry-pick, which writes no marker any
   member reads); axis 3's `A3-1` is M52's own `DESTROYING_DOORS` widening applied to five of the six areas
   a door can remove and not the sixth; axis 6's `D-2` is M52 Increment 9's `suppressed.door` class closed
   over one of its two members; axis 8's `N-1` is M52 Increment 4's own new fourth guard missing from the
   flag help that enumerates guards.

---

# THE ROW-BY-ROW COMPARISON — M51 §A, all 39 rows

The first deliverable. Every §A row of [M51's ledger](../../M51/per-axis-review/README.md) → §A,
re-driven on `1.0.0-rc.16`. **CLOSED** carries the argv that settles it; **STILL-OPEN** carries the datum.
A row is keyed **(axis, id)** — an id repeats across axes — exactly as M51's own conversion ledger keys it.

## Axis 1 · caller tokens — 5 rows, **5 CLOSED**

| M51 row | verdict | the argv + what it shows |
|---|---|---|
| **A1-D1** (HIGH) — a bogus `<slug>` head on a singleton accepted by five write doors; `finalize` drops a payload | **CLOSED** | `printf 'PAYLOAD-ALPHA\n' \| jigc doc set-slot "vision:alpha#thesis" --from-file -` → exit 1, `store.fixed-identity`, one route. Driven over the **whole class**, with the fixed-identity doctype set derived from the binary (`doc schema <ty> --format json` → `identity.kind == "fixed"`: `vision` · `roadmap` · `decisions-log` · `changelog` · `deferral-ledger`) × the 9 reachable address doors — **45/45** exit 1 |
| **A1-D2** (MEDIUM-HIGH) — `relocate <freeze-exempt> --from .jigc` moves jigc's own install artifacts | **CLOSED** | `jigc relocate adr --from .jigc` → exit 1 `config.workbench-root`; `--from .claude` → exit 1 `config.unusable-root`, the installed roots derived from the adapter profile rather than a `.claude` literal |
| **A1-D3** (MEDIUM) — the `relocate`/`from` registry row's declared argv cannot reach its own arm | **CLOSED** | the row now names its manifest-less witness; on a stock pack `relocate adr --from docs/old` → exit 1 `relocate.frozen-doctype` (**15/15** doctypes), and on `--pack-from-dev` the registered argv reaches its caller-token arm |
| **A1-D4** (MEDIUM) — the OS-name-ceiling cell at the address `<slug>` head leaks a bare OS error at five write doors | **CLOSED** | `write.slug-name-ceiling` at every user-address boundary **before** filesystem resolution — `doc`, `task bind` (8/8 cells with a declared role), `rename`, `milestone add-from-spec` |
| **A1-D5** (MEDIUM) — `config set docs-root <component over NAME_MAX>` accepted at exit 0 | **CLOSED** | `unusable_root_reason` checks every component against the ceiling before the filesystem walk, driven at **both** `ROOT_KNOBS` members |

## Axis 2 · posture — 5 rows, **4 CLOSED · 1 partially still-open**

| M51 row | verdict | the argv + what it shows |
|---|---|---|
| **codex-1** — `jigc relocate` mutates the index with `git rm --cached` under a live merge, no re-probe before the mutation | **CLOSED** | a `git` shim firing one real merge **after** `ls-files -z`: `jigc relocate adr --from docs/old` → exit 1, `repo.operation-in-progress`, route `git merge --abort`; `git ls-files --stage` **byte-identical** before and after, and the shim's trace contains **no `rm --cached` at all** |
| **D1** — a commit-on-behalf door never reports `repo.operation-in-progress` under a rebase or a bisect | **CLOSED** | `rebase-merge`: all **10** commit doors → `a rebase is in progress`, route run verbatim → exit 0, state gone, door then exit 0. `bisect`: all 10 → `git bisect reset` → exit 0. `posture()` asks the operation **first**, so `HeadDetached` no longer masks it |
| **D2** — `rebase-apply` is `git am`'s marker too, and jigc names *a rebase* and routes at a command git refuses | **CLOSED** | `am` state: all **12** doors → `` a `git am` is in progress ``, route `git am --abort` run verbatim → exit 0. The sibling `rebase --apply` state still answers *a rebase* — the discriminator is real in both directions |
| **D3** — an un-concluded cherry-pick or revert is not a member of the family, and `task finalize` concludes it at exit 0 | **CLOSED for the conflicted cells · STILL-OPEN for the clean `--no-commit` cherry-pick** | *Closed:* cherry-pick and revert states → all 12 doors refuse at exit 1 with their own nouns and routes, markers still on disk after a full sweep. *Still open:* **A2-DEFECT A** — `git cherry-pick -n` that applies cleanly writes **no marker any member reads** and leaves the index fully merged, so `jigc task finalize` committed the user's picked payload at exit 0 and `MERGE_MSG` is gone |
| **D3b** — git's own refusal during a cherry-pick is dressed as a pre-commit hook rejection | **CLOSED** | reached with `commit.gpgsign` + `gpg.program /bin/false`, **no hook anywhere**: `` `git commit` failed (no commit was made): `` + git's bytes verbatim, **zero** occurrences of the word *hook*, then the door's own state clause and copy-runnable re-run |

*Also closed:* M51's axis-2 **OPEN lead** — the fan-out `git merge --ff-only` re-probe, *"not driven at the
seam by either pass"* — was driven at the seam with a shim firing a real conflicting merge inside
`overlay_docs_commit_and_ff`: exit 1, `repo.operation-in-progress`, `grep -c ff-only trace.log` → **0**
(the fast-forward never ran), HEAD unmoved.

## Axis 3 · destroying doors — 5 rows, **5 CLOSED**

| M51 row | verdict | the argv + what it shows |
|---|---|---|
| **C-1 class** — a non-`.md` under a task's working area destroyed at exit 0 by five doors, no refusal, no narration | **CLOSED (all five doors)** | `task discard` · `milestone discard` · `uninstall` refuse at exit 1 under their own `*.foreign-bytes` code naming **every** planted path with a `--force` route; `task finalize` · `milestone finalize` displace to `.jigc/displaced/<id>/<relative>`, name each `<from> → <to>` and carry the pairs on `committed.displaced`. Bytes byte-intact in all five |
| **D-1** (MEDIUM) — the staged-prose route at two destroying doors names a landing verb the state refuses | **CLOSED** | both routes now name `jigc milestone finalize <id>` and state *so `jigc task finalize` refuses it*; the ordinary-task route is unchanged and lands at exit 3 for the **stated** reason |
| **D-2** (LOW) — `milestone discard --force` acks `workbench removed` over a failed teardown | **CLOSED** | with `chmod 500 .jigc/worktrees` the ack reads `… workbench NOT fully removed — the warnings above name what is left`; both worktrees still on disk, both named |
| **D-3** (LOW) — the fail-closed staged-prose refusal prints the host path | **CLOSED** | with `chmod 000 …/docs`, no absolute path on either surface; both routes repo-relative and naming `--force` |
| **D-4** (LOW) — the worktrees-root fail-closed route blames `git` on PATH and omits the consent | **CLOSED** | route reads *"this failure is a `read_dir` of that directory, **not a git fault**"* and names `jigc uninstall --force` |

## Axis 4 · transaction / rollback — 6 rows, **5 CLOSED · 1 partially still-open**

| M51 row | verdict | the argv + what it shows |
|---|---|---|
| **C1** — the promote rollback is unconditional and emits no conflict finding | **CLOSED** | `task finalize … --approve` under a hook appending to `VISION.md` → exit 1, **the racer's line stands**, one blocking `finalize.rollback-conflict`, pre-image parked at `.jigc/displaced/finalize/VISION.md.pre-image.<nanos>` and named in the route, HEAD unmoved. Also at `milestone finalize` on a **new** destination, whose route states that case in its own words |
| **C2** — `RecordPreImage` rollback restores the milestone record unconditionally | **CLOSED, at all five doors the registry names** | `milestone add-task` under a record-appending hook → exit 1, racer's line still in the record, `milestone.rollback-conflict` keyed at the record, pre-image parked. Same at `milestone create` (absent pre-image), `milestone discard --force`, `milestone add-from-spec`, and `task discard --force` (its **own** `task-discard.rollback-conflict`) |
| **C3** — `RecordFlipGuard`'s `Drop` is a second unconditional restore | **CLOSED, on both commit-model arms** | `milestone finalize` at `finalize.fan-out.squash` = true **and** false → exit 1 both, racer's line survives both, and the two frames differ correctly |
| **C4** — `milestone provision` amends `.jigc/.gitignore` then fails without acknowledging it | **the guard arm CLOSED · the hook-rejected `create` arm STILL-OPEN** | *Closed:* `milestone provision nonexistent-milestone` → exit 1 `milestone.unknown`, file **byte-identical**; a taken-slug `milestone create` → exit 1, byte-identical. *Still open:* **A4-DEFECT 3** — `milestone create` under a rejecting hook → exit 1 with the file **amended**, **zero** mentions of `gitignore` on either stream, and the frame reading *"nothing … survives"* |
| **DEFECT 1** — a rollback conflict breaks `--format json` stream discipline | **CLOSED** | `task finalize --format json` with a hook racing `.jigc/.gitignore` → stdout **0 bytes**, stderr parses as **exactly one** document carrying both `finalize.commit-rejected` and `finalize.rollback-conflict`. M51's `JSONDecodeError: Extra data` does not reproduce |
| **DEFECT 2** — the promote axis silently destroys the racer's bytes, and the retire axis silently declines to restore | **CLOSED, both halves** | promote: see `C1`. Retire: a hook re-creating the retired `direction/plan.md` → exit 1, the racer's file stands, the message states the **deletion** shape, pre-image parked **directory-preserving** |

## Axis 5 · pinned contracts — 4 rows, **4 CLOSED**

| M51 row | verdict | the argv + what it shows |
|---|---|---|
| **DEFECT D** (codex-origin) — a pre-dispatch `current_dir()` failure bypassed both funnels at 47/47 leaves | **CLOSED** | the 47-leaf deleted-cwd sweep: **47/47** exit 1 · stdout 0 bytes · stderr `{"error": "cannot determine the current directory: …"}`. 0/47 non-JSON |
| **DEFECT A** — `setup`/`uninstall` reject with a third, undeclared envelope shape | **CLOSED** | the 47-leaf outside-repo sweep: **45 `{error}` + 2 `{findings, schema_version}` = 47/47 in a declared arm**, zero bare-`Finding` roots, driven at all four of M51's codes |
| **DEFECT B** — `doc show <addr>#<repeatable>` is a second top-level array, declared nowhere | **CLOSED** | the registry now declares **eight** `doc show` rows over **six** `ArmShape` members, all eight driven; `ArrayOf`'s doc-comment no longer claims *"the surface's one array"* |
| **DEFECT C** — `ArmOutcome::Success` claims exit 0 at three rows that ship non-zero by design | **CLOSED by declaration, and re-driven** | the doc-comment now carries *"Not a claim that the verb can never exit non-zero"* and names the three cells; `validate` exit 1, `migrate-corpus --dry-run` exit 1, `task validate` exit 3, key sets exact, stderr 0 bytes, and **no fourth cell found** |

## Axis 6 · composed surfaces — 3 rows, **3 CLOSED**

| M51 row | verdict | the argv + what it shows |
|---|---|---|
| **A6-1** — a sub-task's composed surface is byte-identical inside and outside its worktree, and the boundary then drops its staged code at exit 0 | **CLOSED (both halves)** | the `resume:` line now **discriminates the posture** it is composed in, and the milestone boundary **names the shared-checkout staged work it did not commit** |
| **A6-2** — `start --workflow milestone-execution` composes three unrunnable `Run:` lines at exit 0 | **CLOSED** | `suppressed.door` is parsed and clap-checked at pack load; named composition returns blocking `workflow.verb-routed`. Driven over the **14-member** door set derived from both packs (`grep -rl '^  door:'`), not a hand list, plus a non-member control and a `door: jigc nosuchverb` fence |
| **A6-3** — `implement-from-spec` over a corpus with no committed spec claims a list that is not there | **CLOSED (both cells, plus the dead-end tail)** | the empty case is stated and routed at `plan`; the criteria projection states both its unbound and empty-criteria cases |

## Axis 7 · freeze & migration — 6 rows, **4 CLOSED · 1 STILL-OPEN (declared residual) · 1 half still-open**

| M51 row | verdict | the argv + what it shows |
|---|---|---|
| **C-1** — `ingest` and `unmanage` disagree on a placement document's identity | **CLOSED, both halves** | `ingest` on a committed `VISION.md` writes `"from": "vision:vision"` (rc.15 wrote `vision:VISION`); the following `unmanage` leaves `"edges": []`; runs 2 and 3 are honest no-ops |
| **C-2** — the orphan territory bound misses a root-placement orphan | **STILL-OPEN, as declared** | departed-doctype fixture: mentions of `VISION.md` — `validate` **0**, `doc list` **0**, `ingest` **0**, `migrate-corpus` **0**, while the file still carries `schema-version: 1`. This is `orphan.rs`'s **declared residual 2**, carried by decision on the namespaced-stamp trigger and recorded `UNPINNED` in M51's own ledger. **Not re-reported as a new finding** |
| **D-1** — `jigc ingest` durably absorbs an out-of-band edit and surfaces nothing | **CLOSED** | `ingest` → exit 0 and now prints `advisory · file-state.absorbed` with an Informational route naming the finding it retired; `--format json` carries it with key `{code, target}`; the undrifted control prints nothing |
| **D-2** — a `Relocated`-only bump is invisible to `migrate-corpus`, and the refusal that routes there is a dead end | **CLOSED on all three legs** | placement arm: `migrate-corpus` → `1 migrated HISTORY.md`, committed, `CHANGELOG.md` gone. Location arm: `1 migrated docs/adrs/use-sqlite.md`. Route leg: `relocate adr --from docs/decisions` → exit 1 **`relocate.frozen-doctype`** (a code, where rc.15 had none) routing at `migrate-corpus`, which **acts** |
| **D-3** — `ValueRemapped` on an `id-from` enum falls through to `migrate-corpus.prose-needed` | **CLOSED** | → exit 1 **`migrate-corpus.fold-refused`** (never `prose-needed`), the cause naming the `id-from` role verbatim and stating *"no prose … and no re-run changes it"*; `git status` empty, bytes untouched |
| **D-4** — the orphan finding's false clause · and `ingest` contradicting `validate` at the same commit | **half 1 CLOSED · half 2 STILL-OPEN** | *Half 1:* the message now reads *"no resolved doctype claims this path"* — the false clause is gone. *Half 2:* at **one commit**, `validate` → exit 1 with two `orphaned-instance` rows and `ingest` → exit 0 filing both under *"fine to stay plain … **no action needed**"*. Re-filed as **A7-F2** |

## Axis 8 · adopter docs & help — 5 rows, **5 CLOSED**

| M51 row | verdict | the argv + what it shows |
|---|---|---|
| **CX-1** — `migrate-corpus --help`'s *"`--dry-run` writes nothing at all"* falsified by the invocation log | **CLOSED** | all three homes carry `cli::NO_WRITE_EXCEPTION` verbatim — `migrate-corpus --help`, `validate --help`, and the **driven** `--dry-run` ack |
| **CX-2** — the installed guide's *"every `jigc setup` rewrites it"* false for a user-modified guide | **CLOSED** | the installed `SKILL.md:8` scopes the rewrite (*"replaces it while it is still jigc's"*) and names `adapter-guide.user-modified`; driven, the advisory fires and the file is byte-untouched |
| **CX-3** — the guide's one install command cannot run from the repo it is installed into | **CLOSED** | `SKILL.md:27`/`:33` scope it to *"a **clone of the jigc repository**"*, and the code block carries the `# cwd:` comment |
| **D-1** — `commit-recording.stale-title` silent in the default format at the two surfaces F-9 named | **CLOSED** | `doc rename --task` prints the advisory in agent text, and `task finalize --dry-run`'s text arm names **every** code its JSON sibling carries (2/2) |
| **D-2** — `jigc task discard --help` never says it commits | **CLOSED** | all **9** `COMMITTING_DOORS` verbs' helps state the commit they land; `milestone add-task`'s and `add-from-spec`'s acks name their sha |

**Score across all eight axes: 35 of 39 CLOSED · 4 partial**, and the four partials are
`(2, D3)`'s clean-`-n` cell → **A2-DEFECT A**; `(4, C4)`'s hook-rejected `create` arm → **A4-DEFECT 3**;
`(7, C-2)`, a **declared residual on a written trigger**, deliberately not re-reported as new; and
`(7, D-4)`'s half 2 → **A7-F2**. Nothing regressed; no closed row re-opened.

---

# M51's §D OPEN LEADS (7) — re-dispositioned

| # | axis | M51's lead | disposition on rc.16 |
|---|---|---|---|
| 1 | 1 | **C10's fence half** — all route constructors invoke the command-span fence in debug builds | **STILL-OPEN, same reason.** Axis 1's `C9`: the fence is a **compile/test-time** assertion, not a behaviour any argv against the installed release binary can exhibit. A source read agreeing with a source pass is not a drive, so it stays a lead. The source read is consistent (the symbol iterates the real clap tree in both directions) |
| 2 | 1 | **O5** — a tampered `.jigc/tasks/<id>/source-path` makes `--approve` silently retire nothing | **CLOSED.** Driven at axis 4 cell E1: with `source-path` rewritten to an absolute path in a `mktemp -d` outside the repo, `task finalize … --approve` → exit **1**, blocking `finalize.retire-untrackable` (*"resolves outside the repository"*), **the canary outside the repo intact**, the promote rolled back, `git ls-files -s` identical, HEAD unmoved. Axis 1's `C11` independently confirms the unlink sink re-adjudicates the recorded source immediately before `remove_file` |
| 3 | 2 | the fan-out **`git merge --ff-only` re-probe**, *"not driven at the seam by either pass"* | **CLOSED.** Driven at the seam with a `git` shim firing one real conflicting merge on `git write-tree` inside `overlay_docs_commit_and_ff` — after `SeamSubject::live`, before the fast-forward: exit 1, `repo.operation-in-progress`, `grep -c ff-only` → **0**, HEAD unmoved, the sub-task worktree still holding its staged file. Also driven on its **pass** arm (the same fan-out with no racer finalizes clean at exit 0) |
| 4 | 6 | **C5 is a manifest-scoped guarantee** — a manifest-less project-local pack can ship an off-catalog workflow `describe` narrates with no reason | **STILL-OPEN, unchanged and still declared.** Driven: a manifest-dropped pack loads clean and `describe --workflows` narrates the workflow with **no** *"It is hidden from the router catalog"* clause, while orientation lists it 0 times. Declared at `surface-contract.md`:127 / `introspection.md`:90 — a wave decision |
| 5 | 6 | the **JSON arm of every composed door carries no task-state lines** | **STILL-OPEN, declared at the seam — and now agreed by both passes.** Driven at all **six** render forms: `resume:` / `what's-left:` / `task scope:` / `create-gates:` / `also open:` exist on the text arm of each and on **none** of their JSON arms. The Codex pass states the same from the source side (`CX-10`), so it is a decision both passes now name |
| 6 | 6 | the driver's four §4 leads (a) `migrate` mints silently over open work | **STILL-OPEN — and the record that made it a match is now corrected rather than the behaviour.** Driven with 3 tasks live: `jigc migrate legacy/old.md --as adr` → exit 0, `task minted:`, **0** `also open:` bytes, while `jigc start --workflow quick-fix` over the same state renders `also open: 2 other tasks were already open`. `design/write-commands.md`:189 now carries a dated M52 correction naming this divergence and keeps the lead open on its own trigger (*the wave that next touches `MINT_DOORS`*) |
| 6b | 6 | (b) `task bind`'s bare refusals carry no code and no route | **PARTLY CLOSED.** `store.not-found` and `store.unknown-type` now carry code, key, route **and** the findings envelope (M52 F4, driven); the **malformed-address** arm (`missing ':' between type and slug`) still carries no code on either arm, inside M50's declared flatten posture rather than as a new shape |
| 6c | 6 | (c) `describe --commands --format json` carries no argv · (d) the none-provisioned advisory is deliberately silent | **STILL-OPEN, unchanged, both declared** (`introspection.md`:100; `milestone.rs`'s own comment). Re-read, not re-driven further |

**Net: 3 CLOSED · 1 partly closed · 5 still open, every one of the five declared somewhere** (four in a
design doc, one in a dated record correction with a written trigger). None was promoted on a source read.

---

# FINDINGS

Each entry carries its repro in **condensed** form — the setup, the argv, and the observed lines that carry
the verdict. The **full** block, with every cell of the surrounding table, is in the axis file cited on the
row. **The tier is this README's**, applied from the charter's own three-way predicate
([decisions-pending.md](../../../../implementation/decisions-pending.md) → *The rc.16 wave (M52)*: *exit-0 loss
or repository harm through a committing or destroying door* · *posture and route dead ends* · *surfaces that
say something the binary does not do*); each entry also quotes the **axis's own severity grade**, and where
the two pull apart the axis's grade is printed rather than overwritten.

## A · CONFIRMED defects and contradictions (27 entries)

### Tier 1 — exit-0 loss or repository harm through a committing, destroying or moving door (4)

**The conversion ledger — CLOSED for these four rows** (M53 Increment 6 / T2, 2026-09-22). Each entry
below carries one `pinned-by:` — or a stated `UNPINNED: <why>` — for **the row**, and one more for
**every widening cell the M53 baseline added to it** ([baseline-ledger.md](../../M53/baseline-ledger.md)
§2, *Every class came back wider than its row*), so a cell the fix had to sweep cannot ride the row's
citation. Every citation is **verified by reading what the cited test asserts**, never from its name:
[pinning.md](../../../../implementation/pinning.md) §3 and its 2026-08-18 addendum refuse a `pinned-by:`
symbol parser **by name**, so no command checks the reading — three citations that read apt and asserted
something else are what bought that refusal. **§A's tier-2 and tier-3 rows are not dispositioned here**:
the charter triaged them, and M53 is a fix pass over these four alone
([decisions-pending.md](../../../../implementation/decisions-pending.md) → *The rc.17 fix pass (M53)*).

**The closure check, stated and re-runnable.** It fences the *form* — four entries, each with exactly
one `**the row**` disposition and one labelled `**cell (…)**` line per widening cell — and nothing else:

```
awk '
  /^### Tier 1 —/ { t = 1; next }
  /^### Tier 2 —/ { t = 0 }
  t && /^\*\*`\(/ { row = ++n; r[n] = 0; c[n] = 0; next }
  t && /^`(pinned-by|UNPINNED):`/ {
    if (!row) { printf "STRAY disposition, line %d\n", NR; bad = 1; next }
    if ($0 ~ /\*\*the row\*\*/) r[row]++
    else if ($0 ~ /\*\*cell \(/) c[row]++
    else { printf "UNLABELLED disposition, line %d\n", NR; bad = 1 }
  }
  END {
    if (n != 4) { printf "tier 1 holds %d entries, not 4\n", n; bad = 1 }
    for (i = 1; i <= n; i++) {
      if (r[i] != 1) { printf "entry %d: %d row-dispositions (want 1)\n", i, r[i]; bad = 1 }
      printf "entry %d: the row + %d cells disposed\n", i, c[i]
    }
    exit bad ? 1 : 0
  }' completions/artifacts/M52/per-axis-review/README.md
```

Driven 2026-09-22, exit **0**: `entry 1: the row + 2 cells disposed` · `entry 2: the row + 3` ·
`entry 3: the row + 2` · `entry 4: the row + 3` — **fourteen dispositions over four rows and ten
widening cells, zero `UNPINNED`**. The counts are checked by the command; that they are *the* ten cells
§2 names is checked by reading §2 beside them, which is the half no command can do.

**`(3, A3-1)` · HIGH · `jigc milestone finalize` destroys the milestone's own working-area complement at
exit 0, named by nothing.** — [axis-3.md](axis-3.md) §5, §3 R-H

The landed boundary removes `.jigc/milestones/<id>/` with `remove_dir_all` and takes every byte in it that
jigc did not write. `.jigc/` is gitignored whole, so nothing else has a copy.

```
setup: milestone rig; jigc milestone provision axis-three-probe; work staged in both worktrees
       printf 'MILESTONE-AREA PRECIOUS\n' > .jigc/milestones/axis-three-probe/mnotes.txt
       mkdir -p .jigc/milestones/axis-three-probe/sub; printf 'DEEP\n' > …/sub/deep.txt
       git status --porcelain --ignored …/mnotes.txt  ->  !! .jigc/milestones/   (no other copy)
$ jigc milestone finalize axis-three-probe
  exit 0 — finalized c48cc91 … 3 files committed
  *** no note, no warning, no finding, on either stream ***
  --format json:  "committed": { "displaced": [], … }
after: find .jigc/milestones  -> .jigc/milestones        # the whole area is gone
       find .jigc/displaced   -> No such file or directory
# the three sibling doors, on the identical plant:
$ jigc milestone discard axis-three-probe          -> 1  milestone.foreign-bytes  (names it)
$ jigc milestone discard axis-three-probe --force  -> 0  warning: … not recoverable. (names it)
$ jigc uninstall                                   -> 1  uninstall.foreign-bytes  (names it)
```

Contradicts `DESTROYING_DOORS`' own doc-comment — *"Every door **answers for what it removes** … a door
that destroys what it never named is the law-1 half-truth"*. Three of the four doors over that area answer
for it; the landed boundary does not. **Re-driven by the reconciler**, which also refuted the Codex pass's
two contradicting claims with this repro.

`pinned-by:` **the row** — `milestone_boundary_displacement::a_landed_milestone_boundary_keeps_every_byte_of_its_own_area_it_did_not_write`
— **verified by reading**: over `{squash, chain} × {agent, json}` it drives a real fan-out
`jigc milestone finalize` to exit 0 (the **success** path, so a refusal could not stand in for the
claim), asserts the milestone area is still removed, and then requires every `MILESTONE_PLANTS` byte to
be readable at `.jigc/displaced/<milestone>/<rel>` **byte-equal** to what was planted — `merged/scratch/perf.txt`
included, riding its parent entry's move — each move named on stderr as `from` → `to` under **both**
formats, `committed.displaced` to equal the sorted **union** over this area and every sub-task area, and
jigc's own `tasks.json` and a materialized body never to be parked. Assertion (3) is the exact negation
of this row, and the envelope assertion of the `"displaced": []` the repro quotes.

`pinned-by:` **cell (i) · the `merged/**` region** — `milestone_merged_complement::every_plant_locus_under_merged_is_foreign_and_returned_whole`
+ `milestone_merged_complement::milestone_discard_refuses_over_every_merged_plant_and_narrates_under_force`
+ `milestone_merged_complement::uninstall_refuses_over_every_merged_plant_and_narrates_under_force`
+ `milestone_merged_complement::every_merged_plant_survives_the_unwind_and_leaves_the_area_foreign`
— **verified by reading**: the first calls `engine::state::foreign_area_paths(area, WorkArea::Milestone)`
over six planted loci under `merged/` and asserts the returned complement **equals** those six sorted,
that a foreign *directory* comes back whole (`merged/sub/nested.txt` is asserted **absent** — the parent
is the unit a door names and moves), that no `MILESTONE_AREA_FILES` member written in the shape jigc
writes it joins the complement, and that no colon-bearing materialized body does either; the two door
arms drive the real binary and require the refusal to name **every one** of the six paths under the
shipped `milestone.foreign-bytes` / `uninstall.foreign-bytes` identities, take nothing, and under
`--force` narrate every path on stderr before the area goes; the unwind arm copies the pristine area once
per locus and requires `engine::state::unwind_area` to return `AreaUnwind::Foreign` with that locus
byte-intact (a foreign directory never descended) **while** every `<type>:<slug>.md` body and every
registry member jigc wrote is gone — one file, one answer, so a walk that refuses too much would strand
the mint. The M52 bound —
*`merged/` is jigc's wholesale, never walked* — is negated by the first test's equality.

`pinned-by:` **cell (ii) · `materialize` clearing `merged/docs/` on a finalize that does *not* land** —
`merged_area_selective_clear::a_blocked_boundary_clears_the_stale_body_and_keeps_every_foreign_plant`
— **verified by reading**: it stages an `adr` missing its required `## Decision` so the boundary blocks
at **exit 3** on the boundary's own `conformance.section` with `head_count` unmoved (a blocked run, which
is the arm this cell is about and the row's repro never reached), and then asserts the clear is
*selective*: the stale body whose name `staged_doc_id` recognises is gone **and** its marker is found
nowhere by the same `command grep` instrument that found it before, while every `PLANTS` and
`SHAPE_PLANTS` entry under `merged/docs/` is still on disk **byte-intact** and named by no finding.

**`(3, A3-2)` · HIGH · when the displacement fails, both `Displace` doors remove the area anyway, and say
only that the *move* failed.** — [axis-3.md](axis-3.md) §5, §3 R-I

The removal is **not** conditioned on the move having succeeded.

```
setup: fresh rig; jigc start --workflow quick-fix "tidy the readme"; a landable commit doc
       printf 'NOT A DIR\n' > .jigc/displaced                       # cell (a)
       printf 'PRECIOUS-A3-2\n' > .jigc/tasks/tidy-the-readme/notes.txt
$ jigc task finalize tidy-the-readme --format json
  exit 0
  stderr: note: could not open .jigc/displaced/tidy-the-readme to move … aside: Not a directory (os error 20)
  stdout: "committed": { "displaced": [], … }
after: ls -a .jigc/tasks -> . ..          git grep -l 'PRECIOUS-A3-2' HEAD -> (nothing)
       grep -rl 'PRECIOUS-A3-2' .        -> (nothing)      # permanently gone, at exit 0
# cell (b), an ordinary read-only `.jigc/displaced/`: identical, EACCES
# both cells driven at `milestone finalize` too
```

`Disposition::Displace` is *"**Keep it**"*, and `DestroyingDoor`'s rule is that `PendingLoss` is *"read
before the removal and printed after it, so neither half of the claim can be false"*. Here the printed half
is *the move failed* and the unprinted half is *and then it was deleted*. **The safe cell is recorded too**
(when the *source* is unreadable both operations fail on the same `EACCES` and the bytes survive) — by
accident of a shared cause, not by design, which is why the axis is `{move succeeds, move fails} × {removal
succeeds, removal fails}` and only three of its four cells are currently safe.

`pinned-by:` **the row** — the seven `finalize_displacement::task_area_*` coordinate tests (through
`assert_task_cell`) and the seven `milestone_boundary_displacement::boundary_areas_*` ones (through
`assert_boundary_cell`, each run at **both** `finalize.fan-out.squash` arms) — **verified by reading**:
each drives a landed finalize, takes a **before-control** by invoking the `grep -rlE` binary directly
over the planted `JIGC-M53-KEEP` markers and asserting the before-count **is** the plant count (*a scan
that finds nothing before proves nothing after*), then requires the after-count to **equal** it — *every planted byte is
still on disk, moved aside or left standing, never taken* — asserts the cell's own move coordinate
against the length of `committed.displaced` so a manufacture that silently stopped blocking cannot pass
while testing a different cell, and asserts that an area jigc could **not** empty of a third party's
bytes is **left standing** while one it emptied is gone. The row is the `Moves::None` column: the move
fails and the area is now kept, where M52 removed it anyway.

`pinned-by:` **cell (i) · the partial move, the axis's third value** —
`finalize_displacement::a_partial_move_names_what_it_could_not_move_and_counts_the_whole_area`
— **verified by reading**: it occupies one entry's parent under the parking home so exactly one of two
entries can move, then locates the single narration block beginning `note: the working area held` and
requires **both halves inside that one block** — `held 2 entries`, which is the *complement's* count and
not `moved.len()`; the move it made, named `from` → `to`; the entry it could **not** move; and
`File exists`, the reason, so the operator need not guess which of the parking home's two failure points
it hit. `committed.displaced` is asserted to carry the moved pair **alone**, the key being declared as
what moved.

`pinned-by:` **cell (ii) · the non-atomic `remove_dir_all` skeleton** — the four `*_fault_on_a_later_member`
and `*_fault_on_the_pin` coordinates at both doors, plus
`finalize_displacement::the_unreachable_coordinates_are_named_with_their_reason` and
`milestone_boundary_displacement::the_boundary_axis_names_its_unreachable_coordinates`
— **verified by reading**: an in-transaction `pre-commit` hook `chmod 0555`s either the area's `docs/`
(`merged/docs/` at the milestone) so the removal faults on a **later** registry member, or the area
directory itself so it faults on member 0, the base pin; the shared cell assertion then requires the area
to be **left standing** rather than reduced to a skeleton, exactly one `finalize.foreign-bytes` advisory per
standing area, and `base.json` present **iff** the fault was on the pin — so a clean unwind and a fault
on a later member both take the pin first, which is what a by-id door reads. The two gap tests assert
`CELLS + UNREACHABLE == 9` over the manufactured `{all · partial · none} × {clean · fault on the pin ·
fault on a later member}` space, each unreachable coordinate carrying a non-empty reason and appearing in
exactly one of the two lists.

`pinned-by:` **cell (iii) · `cleanup_subtask_areas`' ignored `all_gone`** — assertion (4) of
`milestone_boundary_displacement::assert_boundary_cell` (driven by its seven `boundary_areas_*` tests)
+ `milestone_boundary_displacement::milestone_discard_force_still_takes_a_sub_task_areas_foreign_byte`
— **verified by reading**: the boundary cells count the standing areas across the milestone area **and
every sub-task area**, require `stderr.matches("finalize.foreign-bytes").count()` to equal that number,
and require each advisory to be keyed at its **own** work unit (`task:<id>` / `milestone:<id>`), so a
boundary settling three areas hands a reader three keys — the return value both landed arms dropped is
now one finding per area. The third caller is disposed the other way and **driven**: `jigc milestone
discard --force` still takes a sub-task area's foreign byte, still acks `workbench removed`, parks
nothing, and mints **no** `finalize.foreign-bytes` — the disposition read off the call, never off the
function.

**`(2, DEFECT A)` · tier-1 by consequence, graded *the clean-`-n` cell of M51's D3* by the axis · a clean
`git cherry-pick --no-commit` is a member of no `InProgress::ALL` row, and `jigc task finalize` concludes it
at exit 0, destroying the picked commit's authored message.** — [axis-2.md](axis-2.md) §4, ledger §B

```
setup: dev/jigc-rig committed-singletons --start decided-task "axis intent"
$ git cherry-pick -n "$P"                                             -> exit 0
$ ls -A .git | grep -E 'CHERRY_PICK_HEAD|sequencer|REVERT_HEAD|MERGE_HEAD|SQUASH_MSG|MERGE_MSG'
  MERGE_MSG                                   <- the ONLY thing git left
$ git ls-files -u | wc -l                     -> 0            # index fully merged
$ cat .git/MERGE_MSG | head -1
  the users cherry-pick target                <- the picked commit's authored message
$ jigc task finalize axis-intent              -> exit 0, the picked payload committed under jigc's subject
$ git cherry-pick --continue                  -> "no cherry-pick or revert in progress"
```

Three homes state the rule this falsifies — `design/finalize.md:31` (*"**any operation git can leave
un-concluded** rather than a list of the markers git happens to write"*), `design/validation.md:664` as
corrected at M52 Increment 3, and `InProgress`'s own doc-comment. Driven, the member set is still
**marker-keyed for eight of nine**, and the ninth (`git ls-files -u`) answers no on a clean pick.
**The identical damage class was made a member at M52 precisely because jigc swallowed it** — `validation.md`'s
`a squash merge` row records *"the entire squashed payload landed inside jigc's own commit at exit 0 and the
merge's authored message was destroyed with `SQUASH_MSG`"*. **Bound, stated by the axis:** this is a fact
about `git cherry-pick -n`'s marker behaviour on the installed git; a git that wrote `CHERRY_PICK_HEAD` on a
clean `-n` would close it, and none is known.

`pinned-by:` **the row** — `repo_posture::every_git_state_names_its_own_operation_and_a_route_git_accepts`
+ `repo_posture::an_uncommitted_cherry_pick_is_abandoned_by_git_reset_in_every_cell`
— **verified by reading**: the first iterates `GitState::ALL` and, for every fixture that declares an
operation, requires exactly one `repo.operation-in-progress` breach whose `operation()` **is** the member
the fixture declares and whose rendered text names that member's own noun, then **extracts the argv out
of the rendered route** — never rebuilds it from `abandon()` — runs it verbatim in that repository, and
requires git to accept it at exit 0 and the probe to answer **no operation** afterwards; it also asserts
in the other direction that no `InProgress::ALL` member is left unreached by the fixture set. The second
builds each uncommitted-pick cell, asserts `MERGE_MSG` present with `CHERRY_PICK_HEAD`, `MERGE_HEAD`,
`REVERT_HEAD`, `SQUASH_MSG`, `rebase-merge` and `rebase-apply` all **absent** (the conjunction that makes
the state distinguishable, and the reason every marker-keyed member missed it) and the `git ls-files -u`
count the cell is named for, then runs `git reset` and requires it to be accepted, `MERGE_MSG` gone in
**that worktree**, the index free of unmerged paths, and the picked bytes still in the working tree.

`pinned-by:` **cell (i) · the three damage shapes (swallow · message-only kill · index contamination)** —
`flow54_acceptance::arm4_an_uncommitted_pick_is_refused_and_nothing_of_it_is_consumed`
+ `posture_door_axis::every_acting_door_adjudicates_the_posture_family`
— **verified by reading**: arm 4 crosses the uncommitted-pick cells with the acting rows of
`BEHALF_DOORS` — **both** `CommitsOnBehalf` and `MovesOnBehalf`, counted off the registry and asserted to
be 12, so a row added anywhere reddens until someone answers what it acts on — and after **every** such
invocation asserts the door refused under `repo.operation-in-progress` naming `cherry-pick`, that
`MERGE_MSG` is **byte-identical** to the bytes read before (closing *swallow* and *message-only kill*
together, since both are observed as that file changing or vanishing) and that `git ls-files --stage` is
identical to its pre-image (closing *index contamination*, the mover-only shape). The door-axis test adds
the complement: a cell that legitimately **proceeds** gets its own repository and is required to raise no
`PostureMember::ALL` code at all, so the sweep cannot green by refusing everywhere.

`pinned-by:` **cell (ii) · the two byte-identical sibling states (clean multi-commit `-n` range;
conflicted `-n` after `git add`)** — `flow54_acceptance::uncommitted_pick_states` feeding `arm4_…`, over the cells
`crates/cli/tests/support/git_state.rs` builds — **verified by reading**: the cell set is **derived**, not
listed — `GitState::ALL` filtered by `state.in_progress() == Some(InProgress::UncommittedCherryPick)` —
and the arm asserts it holds exactly **four**: the clean single-commit pick, the multi-commit `-n`
**range** (built by `git cherry-pick -n <branch>..<branch>`, whose own fixture doc records the driven
datum that it queues **no `sequencer/`**, which is what makes it a cell rather than a variation), the
conflicted `-n`, and the conflicted `-n` **after `git add`** (index clean again, `MERGE_MSG` the only
thing on disk still saying the pick is un-concluded). `is_clean_cell` reads `unmerged == 0` off each
fixture's own driven expectation rather than deciding it, and the arm additionally forbids the
conflict-shaped qualifier *once its conflicts are resolved* in exactly the cells that have none.

**`(5, DEFECT 1)` · tier-1 as repository harm (no byte loss) · two minting doors accept a title that slugs to
nothing and **commit** a record at a fabricated identity at exit 0.** — [axis-5.md](axis-5.md) §11, ledger §B

`engine::milestone::mint_id` and `mint_sub_id` apply an *empty → type-name fallback* with no guard in front
of it, so `jigc milestone create ""` / `--title "!!!"` mints and commits the degenerate `<ty>:<ty>` that
`engine/src/state.rs:2041`'s own doc-comment says the guard exists to prevent (*"left unguarded it mints a
degenerate `<ty>:<ty>`"*). Both doors are `COMMITTING_DOORS` members. Driven at exit 0 with the record
committed; re-driven by the reconciler.

`pinned-by:` **the row** — `work_unit_id_axis::every_mint_door_produces_an_id_every_door_accepts`
+ `flow54_acceptance::arm5_no_door_mints_a_work_unit_at_a_fabricated_identity`
— **verified by reading**: both iterate `MINT_DOORS`, dispatched by the row's `site` with a **hard panic
for an undriven member**, so a sixth row reddens rather than being skipped. The three rows whose id is
slugged from caller prose — `start.rs::mint_in_repo`, `milestone.rs::run_create` and
`engine::milestone::add_task` — are driven over the whole degenerate title set through
`refuses_before_any_write`, which requires the refusal to carry `write.unslugable-title` **and** the
class's sentence, and then asserts the refusal **precedes every write**: `git rev-parse HEAD` unmoved
(*the record commit included*), the working-area name set unchanged, and `git status --porcelain` empty.
The two rows whose id comes from something that is not a title are `Exempt(reason)` on the registry's own
mold and **drive** their stated reason instead of asserting it — the migrate row mints exactly one area
from a `blake3` of the source path whose id satisfies `engine::slug::is_slug`, and the re-seed row
rebuilds an area from the **committed record** after the gitignored workbench is emptied. The hostile
half of the same test keeps the other direction true: every id a mint door does produce is one every
by-id door accepts, so the guard cannot strand a live work unit.

`pinned-by:` **cell (i) · the non-Latin-script and stopword-only titles** — the `UNSLUGABLE_TITLES` /
`DEGENERATE` constant driven by both tests above — **verified by reading**: the set is
`["", "   ", "!!!", "日本語", "the of a"]`, so the two shapes the M53 baseline added beyond the row's
punctuation-only cell are members of the same loop rather than a separate arm, and the assertion is not
only on the code but on the class's own sentence — *ids are built from ASCII letters and digits, so a
title in another script, or of stopwords only, yields none* — which is the half that stays true for a
title in **any** script, the half `work-unit.malformed-id`'s *use lowercase letters, digits, and single
hyphens* would have got wrong at a door whose convention is free prose.

`pinned-by:` **cell (ii) · the third committing door, `jigc milestone add-from-spec`** —
`work_unit_id_axis::add_from_spec_refuses_a_criterion_that_yields_no_id` — **verified by reading**: it
commits a two-criteria spec whose **second** criterion is `日本語` — deliberately second, so the pass has
already minted one sub-task before it aborts — drives the real door, and asserts the pass refuses under
`write.unslugable-title` with the class's sentence, `head_sha` **unmoved** (*no record commit — not for
the degenerate criterion, and not for the ordinary one it had already minted*), the working-area name set
unchanged (the mid-loop unwind carried the earlier mint out) and the working tree clean. That is the seam
the row's two-door prescription missed: `add_from_spec` consults the resume skip set **before** calling
`add_task`, so the fabricated `task` id was skipped at exit 0 and never reached that function's guard.

`pinned-by:` **cell (iii) · `add-task ""` committing a record `jigc validate` itself calls corrupt** —
`flow54_acceptance::arm5_…` at the `crates/engine/src/milestone.rs::add_task` row (via
`refuses_before_any_write`) + `work_unit_id_axis::outside_a_repository_the_degenerate_title_still_answers_not_in_repo`
— **verified by reading**: `""` is the first member of the degenerate set, so the `add-task` row is driven
with an empty intent and the shared helper requires HEAD **unmoved** — and since the record commit is the
write the refusal precedes, no `intent`-empty record is ever written, which is what makes the corrupt
record unreachable rather than merely reported. The companion arm keeps the guard **after**
`discover_repo_root` and `jigc_home_or_repo`: outside a repository, `jigc milestone create ""` is
asserted to answer *not inside a git repository* and to **not** contain `write.unslugable-title`, so the
title guard cannot pre-empt the precondition every door shares with a true sentence about a fact that is
not the caller's problem.

### Tier 2 — posture and route dead ends (8)

**`(1, A1-N1)` · MEDIUM-HIGH · a route whose operand's first byte is `-` is re-read as an option by the
receiving CLI and dead-ends at exit 2.** — [axis-1.md](axis-1.md) §3, ledger §B

```
$ jigc config set docs-root -            -> exit 0     # the Adjudicated row admits the value
… author + finalize an adr, then an ordinary out-of-band `git rm` …
$ jigc validate
  blocking · reconciliation.rename — … is missing
    route: … confirm the deletion by dropping it from the index: `jigc unmanage -/decisions/dash-probe.md`
$ jigc unmanage -/decisions/dash-probe.md          # the route, verbatim, BARE
  error: unexpected argument '-/' found            ROUTE EXIT=2
$ jigc unmanage '-/decisions/dash-probe.md'        # quoting does not help — not a shell fault
  exit=2
$ jigc unmanage -- -/decisions/dash-probe.md       # the form that works, which no surface prints
  exit=0
```

Contradicts the M51 completion audit's own closing assertion for this axis — *"asserts of **every** cell
that the command a door prints **can be run**"*. The cause is one byte in a shared predicate:
`engine::finding::shell_safe`'s inert alphabet includes `-`, and its stated subject is *whether one emitted
token survives a real shell as exactly itself* — which a leading-`-` token does. **The shell is not the
problem; the receiving CLI is.** This is M51's space class one byte over. **No bytes are lost**; what is
lost is the recovery. **Widened by the reconciler** to a second producer reachable with no knob preimage.

**`(1, A1-N2)` · MEDIUM · a `ROOT_KNOBS` value the door accepts makes the door's own `git mv` unparseable,
and the refusal's route prescribes re-running it.** — [axis-1.md](axis-1.md) §3

```
$ jigc config set placement-root -
  blocking · config.repoint-failed — `placement-root` was not set to `-`:
    `git mv docs/decisions-log.md -/decisions-log.md` failed: error: unknown switch `/'
    — the re-point was undone
    route: … Fix what this message names, then re-run `jigc config set placement-root -`
  exit=1     (transaction sound: git status empty, config unchanged — driven)
```

Three things are true at once and the third is the defect: the transaction is sound; the failing argv is
**jigc's own** (built with no `--` separator); and the route prescribes a repeat of a deterministic failure
— there is nothing the operator can fix, the value **is** the fault. **And the knob's admissibility depends
on corpus shape, not on a value rule**: the same `config set docs-root -` lands at exit 0 on a corpus with
no `location:` instance to move and exit 1 on one that has. None of the row's three declared predicates
covers *a value the door's own git invocation will read as an option*.

**`(2, DEFECT C)` · a posture breach raised at the commit seam prints no state-truth clause and no
copy-runnable re-run, while every other in-transaction failure at the same door prints both.** —
[axis-2.md](axis-2.md) §4, ledger §B

```
A · a rejecting pre-commit hook, same rig, same task:
  `git commit` was rejected (no commit was made): … task axis-intent is intact — nothing was
  committed, your task's staged docs are still in `.jigc/tasks/axis-intent/docs/` … then re-run
  `jigc task finalize axis-intent`.                               exit 1 · log error_code set
B · a posture raced in at the same point (a git shim firing `git bisect start` after the stage):
  blocking · repo.operation-in-progress — a bisect is in progress …
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
  exit 1 · log finding_codes:[repo.operation-in-progress], error_code: null
  (verified on disk and unsaid: HEAD unmoved, task still open, work.txt still staged,
   jigc's own .jigc/* staging rolled back)
```

The axis quotes **both** sides rather than hiding the counter-citation: `design/finalize.md` → *6. Commit*
requires a state-truth clause, the door's own copy-runnable re-run and its own error identity, and states
the composition is *"one function serving all three render arms … so no arm can take the constant and skip
the outcome"*; `task.rs`'s `already_typed` lists the `repo.*` posture refusals as a deliberate passthrough.
It is a contradiction **between two stated homes**. Driven identically at `milestone finalize`'s fan-out
seam, where the transaction had gone further and the surface was still those two lines.

**`(4, DEFECT 1)` · LOW · `finalize.stage-failed`'s route is not copy-runnable, while the same door's other
frame is.** — [axis-4.md](axis-4.md) §6, §4 R-F

```
setup: … a landable task; then  : > .git/index.lock
$ jigc task finalize axis-four-probe                        -> exit 3
  blocking · finalize.stage-failed — … `git add -- …` failed: …
    route: resolve the embedded git failure …, then re-run `jigc task finalize`
$ rm -f .git/index.lock; jigc task finalize                 # the route, verbatim
  error: the following required arguments were not provided: <ID>      -> exit 2
# the SAME door's hook-rejection frame, for contrast:
  re-run `jigc task finalize axis-four-probe`
```

The task id is not missing: `stage_failed_finding(task_id, git_error)` **takes** it and spends it on the
locus while the route is a bare `&'static str`. **Checked and not overclaimed:** the finding's
`gate_coverage` row is `Door::FinalizeOnly(NotPreviewable::NotYetExistent)`, so `task validate`
deliberately does not preview it — the route reaches a reader at the finalize door only, which is where it
was driven.

**`(5, DEFECT 2)` · `config remove-step` and `config replace-step` refuse a step that **is** in the include
list, name the wrong verb, and print a route the caller has already satisfied.** — [axis-5.md](axis-5.md) §11

`resolve_fork_bytes` is reached by three callers and its refusal message is written for one of them: a step
inserted by `jigc config insert-step` exists only in the project layer, so the pack read fails and both
sibling verbs raise *"no step `X` body **to fork**"* — while the very next invocation of the same binary
lists that step in the resolved include list. Re-driven by the reconciler.

**`(6, D-1)` · MEDIUM · `jigc start` orientation reports a live task's findings without the repository
posture that blocks every door it then routes to.** — [axis-6.md](axis-6.md) §5

```
rig: fresh;  jigc start --workflow single-task "posture parity";  git bisect start
$ jigc start                       -> exit 0   "findings: 2 blocking, 1 advisory"
   Run: `jigc task validate posture-parity` — previews … THE REPOSITORY POSTURE FINALIZE REFUSES UNDER …
$ jigc start --format json         -> the three content codes; repo.operation-in-progress ABSENT from the wire
$ jigc task validate posture-parity -> exit 1  repo.operation-in-progress — a bisect is in progress
$ jigc task finalize posture-parity -> exit 1  identical refusal
```

Contradicts the M50 design of record for the orientation variant — findings come *"from the shipped
task-scope sweep **so orientation and its own `task validate` route cannot diverge**"*. **The control is
driven and they agree everywhere else** (a carried-staged probe: the same 3 blocking + 1 advisory, same
order). The divergence is only on the leg M52 added at the door, which the front door did not join.
Surface-tier, no loss: the posture is stated at the door where it binds.

**`(6, D-2)` · MEDIUM · `fix-task` is composable by name, and the composed walk's forbidden door is the only
one that works.** — [axis-6.md](axis-6.md) §5

```
rig: fresh
$ jigc start --workflow fix-task "fix the thing"      -> exit 0, task minted (no milestone, no worktree)
   composed body line 25: "Never `git commit` and never `jigc task finalize` here."
$ … fill the commit doc, git add fixed.txt …
$ jigc task finalize fix-the-thing                    -> exit 0   <- the door line 25 forbids
$ (the declared shape, for contrast) milestone add-task … --workflow fix-task; provision; execute
   Spawn: `cd .jigc/worktrees/finding && jigc workflow fix-task --task finding`   -> exit 0
```

The pack's own `suppressed.reason` says *"it has no commit boundary of its own, so a router pick could
never land"*; driven, a by-name composition lands one. **The axis is derived, not reported**: partition both
packs by `(creates-task, selectable, suppressed.door)` and the `selectable: false ∧ creates-task: true ∧ no
door` set is **five**; four of them say *invoked/reached **by name*** in their own reason and compose
correctly, and `fix-task` is the one member whose reason says it is *spawned*, sharing its operative clause
verbatim with `sub-task` — which **is** declared. M52 Increment 9 closed that class over one of its two
members.

**`(7, A7-F3)` · `doc show` refuses a relocated managed doc with a route that names none of the repair paths
the same commit's other doors name.** — [axis-7.md](axis-7.md) §3, ledger §R-R2

```
one commit, five doors, one document (driven on BOTH home kinds — location and placement):
$ jigc validate   -> 1  schema-conformance.schema-version-current   route: run `jigc migrate-corpus`   CORRECT
$ jigc doc list   -> 0  changelog:changelog  CHANGELOG.md  managed        <- stale home, "managed"
$ jigc ingest     -> 0  ingest.wrong-location  route: move it by hand, then re-run `jigc ingest`
$ jigc doc show changelog -> 1
    store.not-found — could not read `changelog:changelog` at `HISTORY.md`: No such file or directory
    route: create the referenced doc, or fix the reference …, or read it with `--task <task-id>`
$ jigc migrate-corpus  -> 0  "1 migrated HISTORY.md"    # the route validate named, run verbatim
$ jigc doc show changelog -> 0  renders
```

The document is tracked, on disk, and `doc list` calls it `managed` in the same second that `doc show` says
it *could not read* it, offering three exits, none of which is the one `validate` names one command earlier.
**Widened by the reconciler** from the driver's `location` arm to the `placement` arm.

### Tier 3 — a surface says something the binary does not do (15)

**`(1, A1-N3)` · MEDIUM (a record defect) · a carried deferral's enumeration of the flatten exemptions is
falsified by a fourth family, which diverges from its own sibling inside one door.** —
[axis-1.md](axis-1.md) §3

`decisions-pending.md` carries M52 **(b)**: the contract's rule *a reject that carries a finding takes the
findings arm* is broader than the binary, with `store.malformed-slug`, `pack.resource-missing` and the
`workflow-refs.*` flatten path as the deliberate exemptions. Driven, there is a fourth, and it is the one
family the contract's **own target-form table** lists under a declared URI target: at one door, one
argument, one `write.*` family, `write.not-present` takes `{findings, schema_version}` with a key and
`write.slug-name-ceiling` takes `{error}` — and the latter carries a locus in the declared normal form plus
a route, so the exemption's two measured M50 grounds do not transfer. **Not a new binary behaviour — a
false enumeration in the record**, which is the false-completeness shape this review exists to catch.

**`(2, DEFECT B)` · a conflicted `git merge --squash` is answered by the wrong member, with a predicate that
is false of the state.** — [axis-2.md](axis-2.md) §4

```
$ git merge --squash cb            -> exit 1;  .git holds MERGE_MSG + SQUASH_MSG;  git ls-files -u -> 3
$ jigc milestone create CS1
  blocking · repo.operation-in-progress — a squash merge is staged and not committed —
    the repository is not in a committable state
    route: conclude it, or abandon it with `git reset --merge`                      exit 1
```

`design/validation.md` → *The M52 widening* attributes exactly this cell to `UnmergedIndex` (*"unmerged
paths with no operation marker at all — … **or a conflicted `git merge --squash`**"*), and `repo.rs`'s own
doc-comment says the same. Driven, `SQUASH_MSG` **is** a marker, so `SquashMerge` answers first and tells
the user the squash *is staged and not committed* when it is conflicted with three unmerged entries.
**The route is effective** (`git reset --merge` cleared it and the door then exited 0) — what is wrong is
the claim, on the member whose whole purpose is naming which of nine things the user is in.

**`(3, A3-3)` · LOW · `milestone_boundary_displacement`'s subject is the sub-task areas only, so the
boundary's `displaced: []` assertion is true of a tree it did not look at.** — [axis-3.md](axis-3.md) §5

`grep -c 'milestones' crates/cli/tests/milestone_boundary_displacement.rs` → **0**: every plant in that
suite is under `.jigc/tasks/`, so the boundary's **own** area is outside the suite's subject and A3-1 is
green under the suite that exists to fence this door's displacement. A defect against the acceptance
design's own rule that an arm *iterates its class's axis* — the class is *working areas this door removes*,
and the axis has two members. **One reconciler correction, recorded rather than hidden:** the driver's
parenthetical *"no suite in `crates/cli/tests/` contains `could not open`/`could not move`"* is over-broad
(a tree-wide grep returns 3 hits, all about `relocate`); the substantive claim — the move-failure cell has
no standing test at either `Displace` door — holds.

**`(4, DEFECT 2)` · LOW · `config.repoint-failed` renders an absolute host path in the message a pinned
envelope carries, and the declared reason for leaving it says it is not a finding surface.** —
[axis-4.md](axis-4.md) §6, §4 R-G

```
… chmod 0555 .jigc/config  (so write_scalar fails AFTER the git mv batch lands) …
$ jigc config set docs-root documentation --format json   -> exit 1; stdout 0 bytes
  stderr, one {findings, schema_version} document:
    "code": "config.repoint-failed",
    "message": "… could not write /private/var/folders/nj/…/repo/.jigc/config/manifest.yaml: … "
    "location": { "address": ".jigc/config/manifest.yaml" }     <- the repo-relative spelling is in hand
  (the rollback itself is clean: doc byte-identical, `git status --short` EMPTY)
```

Law 1's printed-path fence has one home, `render::repo_relative`, for *"the honest absolute for a path
genuinely outside the repository"* — and this path is inside it. **The second half is the record**:
`repo_relative_paths.rs`' `UNSWEPT_PRODUCERS` row for `config.rs` (20 sites) gives as its reason *"an error
channel, **not a finding surface**"*; driven, one of those sites' text **is** the `message` of a blocking
finding with a `(code, target)` key, on the text arm and the 1.0-pinned JSON arm alike. That row was re-read
at M52 Increment 10 and **kept** the clause this repro falsifies.

**`(4, DEFECT 3)` · LOW · a hook-rejected `milestone create` leaves its `.jigc/.gitignore` amend on disk,
names it nowhere, and says *"nothing … survives"*.** — [axis-4.md](axis-4.md) §6, §4 R-E

```
printf 'tasks/\n# my private line\n' > .jigc/.gitignore;  a rejecting pre-commit hook
$ jigc milestone create 'Amend probe'      -> exit 1
  "nothing was committed — the record write and the milestone workbench were both rolled back, so
   nothing of milestone:amend-probe survives."
$ diff /tmp/ign-pre.txt .jigc/.gitignore   -> the amend SURVIVED (6 added lines)
$ grep -c gitignore <stdout> <stderr>      -> 0 0        $ git status --short -> ` M .jigc/.gitignore`
```

This is **M51 §A's C4 narrowed to what rc.16 still does**. `IGNORE_DOORS`' row for this door declares its
ack to be *"the `minted milestone:` ack itself"* — on a rejected run that ack does not exist, so the
declared channel never opens while the write survives; and the frame composes *"nothing … survives"* with no
*"apart from"* opener because the surviving path is outside `ROLLBACK_POPULATIONS`. **No data loss** — the
amend is a union and the user's own lines are kept, verified. **What M52 did fix is driven too**, so the
finding is not overstated: both guard-refusal arms are closed and the succeeding control still acks.

**`(5, DEFECT 3)` · `jigc rename` has a refusal outside its own declared-complete refusal registry, and it
carries no code.** — [axis-5.md](axis-5.md) §11

`rename.rs:40-42` claims *"**Every one of those refusals** is declared and disposed in one place —
`RefusalKind`, which carries the finding code each raises"*. Driven, `parse_addr` bails with a bare
`anyhow!` before any `RefusalKind` is reached, so the bare-slug form — *the natural first guess, and the
form the module's own comment names as such* — refuses with no code, no `key` and no error-code-registry
entry, and it is the one form the two sibling doors **accept**.

**`(5, DEFECT 4)` · `doc show` blocks a **declared but unpopulated** optional leaf with `store.no-such-leaf`,
while the design doc states that rule over *undeclared* leaves only.** — [axis-5.md](axis-5.md) §11

`design/doc-read-surface.md:86` scopes the block to an **undeclared** leaf; `doc schema` at
contract-version 7 advertises the address (`adr:<slug>#status/supersedes`, `required: false`). Driven,
`resolve_section_leaf` blocks on a declared, optional, absent field with a message asserting the leaf does
not exist. Re-driven by the reconciler, who confirmed the two-homes reading in both homes.

**`(5, C1)` · origin codex, driven · `start`'s two orientation rows declare `next_steps`, and a reachable
composition omits it.** — [axis-5.md](axis-5.md) ledger §A

```
rig: fresh --pack-from-dev, then MOVE the one remaining off-catalog verb out of the throwaway pack
$ jigc --format json start                    -> exit 0, stderr 0 bytes
  driven keys:   ['header', 'schema_version', 'state', 'workflows']
  DECLARED:      ['header', 'next_steps', 'schema_version', 'state', 'workflows']
$ … the ActiveTask arm: identical, next_steps absent on the wire
```

`next_steps` is `skip_serializing_if = "Vec::is_empty"` and the producer returns empty when the composed
pack-set ships neither `planning` nor `ingest-existing` — a pack the binary loads without complaint. The
registry's declared bound covers a **missing row**; here the row exists and its declared key set is **false**
for a reachable state, which is proof 4's own cell failing rather than the bound absorbing it.

**`(5, D1)` · origin reconciler · `jigc start --explain` emits a production `--format json` stdout arm that
`ENVELOPE_ARMS` does not carry.** — [axis-5.md](axis-5.md) ledger §C

The same class as M51's DEFECT B, and it survived **both** passes: `Command::Start { explain: true, .. }`
short-circuits before compose-or-orient into `run_explain`, which prints `json(tree)` over
`engine::result::ResolutionTree`; `ENVELOPE_ARMS` carries exactly four `start` rows and none is it. The
driver's 64-row sweep never reached it (`MINIMAL_ARGV` for `start` is the orient form, so none of the four
uniform 47-leaf sweeps reaches `--explain` either), and the source pass listed the print site inside a
family it declared covered.

**`(6, D-3)` · LOW · the orientation footer's `Preview:` line still states the pre-M52 rule, which
`milestone-execution` falsifies.** — [axis-6.md](axis-6.md) §5

`jigc start`'s footer: *"a workflow that mints nothing has no preview — `jigc start --workflow <id>`
composes it directly, and mints nothing either"*. `jigc describe --workflows` carries the **swept** sentence
(*"neither reaches a workflow whose line below says it is reached only through a verb … and both of these
refuse it by name"*). Driven: `jigc start --workflow milestone-execution` → exit 1 `workflow.verb-routed`;
`--workflow router` → exit 0 (the control). One claim, two surfaces, M52 Increment 9 swept one.

**`(6, D-4)` · LOW · a legitimately-empty `milestone execute` walk does not state its empty case.** —
[axis-6.md](axis-6.md) §5

```
$ jigc milestone create "Empty probe";  jigc milestone execute empty-probe    -> exit 0
  "Spawn a sub-agent per sub-task (one per `Spawn:` line below) …"
  grep -c '^Spawn:' -> 0      stderr -> empty        <- zero spawn lines, and nothing says so
```

The sibling empty enumeration (`implement-from-spec`'s spec list) got exactly that clause at M52; this one
did not. **The terminal is safe, which bounds the severity** — the walk's last verb refuses with a code and
a two-armed route.

**`(7, A7-F1)` · the exit-flip trailer asserts a commit and a `git mv` that never happened.** —
[axis-7.md](axis-7.md) §4 R-F1, ledger §R-R8

```
$ rm CHANGELOG.md                       # uncommitted, unstaged worktree deletion
$ jigc validate                         -> exit 1
  … reconciliation.rename … route: `jigc unmanage CHANGELOG.md`          <- fine
  … schema-conformance.home-vacated … route: `git checkout -- CHANGELOG.md`  <- fine
  TRAILER: "out-of-band rename detected — a structural-identity change THIS COMMIT INTRODUCED;
            the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`)."
$ git rev-parse HEAD -> unchanged   git status --short -> " D CHANGELOG.md"
$ git log -1 --name-status --diff-filter=R -> (no rename rows in HEAD at all)
```

**Re-driven over all three `orphan::Removal` cells** — committed, staged and worktree-only — and the same
trailer appears verbatim in the two where the act was a `git rm`. Its reach is explained by a declared
mechanism the axis records as an observation rather than a second defect: `first_store_exit_flip` documents
*"table order **is** precedence"*, so where two members match only the first trailer renders, and that is
the one the reader gets.

**`(7, A7-F2)` · `ingest` says *no action needed* about the files `validate` blocks on** (M51 `(7, D-4)`
half 2). — [axis-7.md](axis-7.md) §4 R-F2, ledger §R-R6

```
one commit:
$ jigc validate --format json -> exit 1
   [("schema-conformance.orphaned-instance","docs/decisions-log.md"),
    ("schema-conformance.orphaned-instance","docs/roadmap.md")]
   each route's FIRST exit: "ask `jigc ingest`, which re-reads the file …"
$ jigc ingest                 -> exit 0
   unmanaged docs/ — 2 file(s) parse against no schema (left untouched — fine to stay plain)
   "staying a plain file is a legitimate end-state — no action needed."
$ jigc ingest --format json   -> both rows {'verdict':'unmanaged','finding':None,'annotations':[]}
```

The axis records the discriminator that keeps its sibling out of the finding set: at the *ahead* and *below*
cells `ingest` is currency-blind **by declared design** and makes no positive claim, whereas here it makes
one (*no action needed*) about a path another door blocks on, and it is the door `validate`'s own route
hands to first.

**`(8, N-1)` · LOW · `jigc uninstall --help` contradicts itself about its own guard count, and the `--force`
clause is the half that lies.** — [axis-8.md](axis-8.md) §3

```
one --help, two numerals:
  about:      "Four states it refuses instead of destroying … --force … deletes all four"
  --force:    "… a fan-out worktree with content, an open task's staged docs, or a workbench file
               no index has a copy of … Inert when all three guards are already clean."
driven, in a state where all three ENUMERATED guards are clean and only the fourth is dirty:
$ jigc uninstall          -> 1  uninstall.foreign-bytes  .jigc/displaced/u-four/notes.txt
$ jigc uninstall --force  -> 0  warning: … the relocation workbench is the only copy of these bytes
                                — they are not recoverable.            <- NOT inert
```

The fourth guard is M52 Increment 4's **own** new member. **The sweep's sibling shows this is an omission,
not a decision**: `milestone discard`'s flag help *was* swept in the same wave and names all three of its
guards. **Two supporting facts are recorded as bounds rather than widenings**: the refusal reports one guard
at a time (consistent with the about's *"Any of them removes nothing until you re-run"*, so **not** graded),
and the `--force` narration is complete — four warning blocks over a four-dirty state, which is what makes
the flag help's three-population enumeration the outlier.

**`(8, N-2)` · LOW · every retitle-without-reslug at `jigc rename` acks a no-op and commits one, while the
binary knows the title it just moved.** — [axis-8.md](axis-8.md) §3

M52 Increment 6 mints `write.identity-change` and routes it at a **specific argv** whose stated effect is a
title rewrite. Driven, that argv rewrites the title (`VISION.md:5` → `# New Vision`) and reports
`renamed vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)`, with the commit
subject `rename VISION.md -> VISION.md` — **the title, the only thing that moved and the thing the route
promised, is on neither surface**, nor in the permanent git record the door lands. The binary *has* the
datum: re-run the identical argv and the no-op arm prints the title. So the *failure to act* ack is fully
informative and the *successful act* ack is not — the inverse of the intended asymmetry. **The class is
wider than the fixed-identity route**, driven on a slugged doctype too.

## B · CONFIRMED source-pass claims driven and held

The Codex passes' **positive completeness claims** that the reconcilers drove and confirmed. They are listed
in aggregate because none names a defect; each axis file carries them individually with its repro.

| axis | what was driven and held |
|---|---|
| 1 | `ARG_TOKENS`' totality over the clap vocabulary and its six path-bearing members · `DOCTYPE_DOORS`/`WORK_UNIT_ID_DOORS`/`SLUG_DOORS` as projections of one classification · migration admission and destructive retirement sharing `resolve_source_token` (8 cells) · **the strongest positive on the axis**: the unlink sink re-adjudicates the recorded source immediately before `remove_file`, so no raw source-path read bypasses the typed sink · zero schema-hash movement |
| 2 | all five M51 dispositions, each re-driven at the seam (16 of 17 leads confirmed) |
| 3 | M51's five §A dispositions · `LeftoverAt` dispatch shape-complete with no surviving `is_dir()` bypass · `--force` attached only to the four refusing rows' consent, so it cannot silently widen finalize · `TASK_AREA_FILES`' complement · the writer rollback's **non-recursive** deletion, which closes the hook-created-file bypass adjacent to this axis |
| 4 | all four M51 codex rows · `ROLLBACK_POPULATIONS`' eleven populations with explicit discipline · the shared finalize closure capturing five pre-image families **before** promotion/retirement/ignore-amend/stage/commit · `COMMITTING_DOORS`' ten identities with `setup` deliberately outside (`--no-verify`) · all four `gitignore::ensure` writers · `TASK_AREA_FILES` = 13 · `STORE_EXIT_FLIPS`' seventh member · zero schema-hash movement |
| 5 | nine leads, all confirmed: the four M51 closures · the four pre-pin deletes still absent · every `Unpinned` row carrying a stated reason · the 64-row/47-leaf serialization inventory **in its counts** (its completeness half refuted → D1) · zero schema-hash movement |
| 6 | ten: the three M51 §A closures · the shared composed seam (`{task, text}` at all six render forms) · the three resume states · orientation's three states with `findings: unknown` never collapsing to `none` · the read-back owe-set · **3025 lines** of composed text re-swept for executable `jigc` forms (25 real leaves, 5 prose fragments, zero non-leaf commands) · the catalog membership partition · zero manifest movement |
| 7 | eight: the root-placement residual · four M51 closures · the fixed-identity predicate as `placement \|\| singleton` consumed by four call sites · the project-layer freeze bypass **refuted as a possibility** at 8/8 doors · zero schema-hash movement |
| 8 | sixteen: the five M51 closures · `NO_WRITE_EXCEPTION` at all three homes · the derived help projections (`SchemaChangeKind::ALL` 18 kinds, `STORE_FAMILIES`, `whats_left_coverage()`, `WHOLE_DOC_KEYS`) · the installed `SKILL.md` built from both guides with `include_str!` and stamped · `DESTROYING_DOORS`' six disposition-bearing members · the `suppressed.door` help · `COMMITTING_DOORS` = 10 · zero schema-hash movement |

**The freeze boundary, asserted on every axis.** All eight source passes ran the same check independently —
`git diff` over both schema trees, both manifests and both snapshot trees from M51's `577a0099` to `e519e4eb`
— and all eight report **no paths**. M52's stated boundary (zero schema-hashes, zero `schema-version`s,
zero corpora moved) holds on the shipped binary.

## C · REFUTED claims — each with its falsifying datum

*Granularity, stated:* the rows below are the refutations **that name a claim and its datum**, one row each.
They are fewer than the roll-up's 13, because two reconcilers counted a partial refutation and a datum
correction as separate refuted rows in their own hand-off (axis 8 reports 4 where the two claim-level
refutations are listed here, its other two being the datum corrections its §R.5 records against rows that
otherwise hold). **The axis file governs**; this table is the claim-level view.

| axis | claim | falsifying datum |
|---|---|---|
| 2 | **CX-4**: *"`InProgress::ALL` contains all nine declared git states … operation-first posture prevents any committing door from reaching its commit seam under them"* | **A2-DEFECT A**: a clean `git cherry-pick -n` is a member of no row — it writes `MERGE_MSG` only, leaves the index fully merged, and `task finalize` reached its commit seam and committed at exit 0 |
| 3 | **CX-1**: *"no new source-grounded completeness defect found … both finalizing doors **displace** [foreign bytes] … in every area it removes"* | **A3-1** and **A3-2**, two independent drives: exit 0, both streams silent, `committed.displaced == []`, `.jigc/displaced` never created, bytes in no commit and nowhere on disk |
| 3 | **CX-11**: the census aggregate *"I found **no unguarded production removal of adopter bytes**"*, with `task.rs:5981` annotated *"foreign complement displaced first"* | the same two drives, plus the source coordinate the census mis-reads: at `task.rs:5957-5983` the `remove_dir_all` is **not** conditioned on the displacement's outcome |
| 4 | **CL-0**: *"no source-grounded completeness defect found — no omitted transaction door, no uncaptured in-closure write"* | **DEFECT 3**: `milestone create`'s `.jigc/.gitignore` amend survives a rejected commit, is named nowhere, and the frame says *"nothing … survives"* |
| 5 | **C8**, in its **completeness** half: *"`ENVELOPE_ARMS` is 64 rows over all 47 clap leaves, and no stdout serialization escapes the central renderer"* (its **counts** are confirmed) | **D1**: `jigc start --explain --format json` prints `json(tree)` from `run_explain`, an arm the registry does not carry |
| 6 | **CX-0**: *"no grounded completeness defects found: no omitted door, missing composed-output state, invalid executable step command, orientation null/unknown collapse, or catalog/help contradiction"* | D-1…D-4, **two of them inside the categories it names** |
| 6 | **CX-9**, in its **completeness** half: catalog membership is exactly `creates-task && selectable` and every complement member must carry `suppressed: {reason, expires}` (the membership/partition half is **confirmed**) | D-2's un-fenced `door:` key and D-3's un-swept sibling sentence |
| 7 | **CX-5**: M51's `D-4` is CLOSED | **REFUTED IN PART** — half 1 is closed, half 2 is **A7-F2**, re-driven by the reconciler at `R-R6` |
| 7 | the driver's forward-reference to a defect **`A7-F4`** | **a dangling pointer**: no such defect is stated or reproduced anywhere in the file. The `a8` row it hangs off **is** driven; the defect it forward-references does not exist. Nothing was entered in the ledger for it, and the reconciler marked it in place |
| 8 | **C-16**: *"I found no contradictory numeral, default, gate subject, exit-code claim, or 'never' assertion in the inspected help strings and guides"* | **N-1**'s live repro — *"Four states it refuses"* and *"Inert when all three guards are already clean"* in one `--help`, with `--force` demonstrably not inert |
| 8 | **C-13**, in its **ordinal**: *"`schema-conformance.home-vacated` is the **seventh** `STORE_EXIT_FLIPS` member"* (confirmed **in substance** — it is a member, and the set is 7) | it is member **6** by declaration order, deliberately ahead of `foreign-squatter` |

Axis 1 and axis 5 recorded **no** refutation of a Codex claim (axis 5's single refutation is the scoped half
of `C8` above); axis 1's pass made no completeness claim this review could falsify, and its bound (`C15`) was
looked for and **not found** — see §D.

## D · OPEN leads — driven as far as the state allows, promoted by nothing

*Granularity, stated:* axes 6 and 7 carry their leads as **one row per cluster** below where their own files
enumerate them singly (axis 6: eight leads in one row; axis 7: nine in one row), which is why the rows here
number fewer than the roll-up's 27. Every member is named inside its row, and **no lead is dropped**.

| axis | lead | why it is open, not a finding |
|---|---|---|
| 1 | **C9** — the occurrence fence is bidirectional over every real `(leaf, argument)` pair | a **compile/test-time** assertion no argv against the installed release binary can exhibit; confirming it means running the multi-minute gate, not a probe. Source read consistent |
| 1 | **C15** — a declared bound: an argument mis-classified `PlainValue::Other` that does reach a path could evade the occurrence registry | a *possibility* statement about a future mis-answer, which no drive can close. The reconciler looked for an instance today: the three `Other`-classified arguments whose values reach a path component (`intent`, `title`, `to`) were driven over five escape shapes each and **all sanitize through the slug rule**. **No instance found — the bound is unfalsified and remains, correctly, a bound** |
| 4 | **CL-4's stale-base ordering leg** | no corpus state builds a stale base: a HEAD-moved-past-base `milestone provision` exited **0** at the pinned base. Neither confirmed nor refuted on behaviour; the reconciler will not promote an ordering claim on a source read |
| 4 | **cell D in full** (stage failure × worktree concurrently edited, 15 rows) | no deterministic in-transaction racer exists before the stage — the hook is the only controllable execution point and it runs after. Kept an **open lead rather than an `n/a`**, because *no instrument exists* is a statement about the fixture library, not about the binary. Both passes and M52's own VERDICT reach the same conclusion |
| 4 | **a genuine concurrent *process* racing a pre-image** | every race here is the `pre-commit` hook, which is the instrument `design/finalize.md` names; a second process would test the scheduler |
| 4 | **`milestone provision`'s post-`ensure` failure point** | M52's T9 moved the `ensure` behind the guards; whether any failure point now lies *after* it is unestablished. Two arms probed (exit 1 pre-write, exit 0), neither reaching |
| 6 | **O-1** `migrate` mints silently over open work · **O-2** the malformed-address refusal's code-lessness · **O-3** the orientation `findings: unknown` reason printing an absolute host path (re-observed by the reconciler in its own drive, inside a declared bound) · **O-4** `milestone-execution`'s present-tense `suppressed.reason` · **O-5** `task validate` not previewing the migration review hold · **O-6** the manifest-scoped C5 guarantee · **O-7** `provision`'s empty parenthesised list over a zero-sub-task milestone (re-observed) · **the JSON arm carrying no task-state lines** | eight leads, **every one declared somewhere**, none promotable on a source read, none droppable. Two were re-observed by the reconciler in drives it ran for other reasons, which is recorded rather than folded in |
| 7 | the **methodology pack's own `schema-manifest.yaml`** (no rig builds a reshapeable methodology pack) · **40 of the 54 `SchemaChangeKind × LOCI` cells** · `ManifestKind::ALL` behaviour (**n/a with a driven reason** — no axis-7 door renders a manifest) · the registries no axis-7 door is a member of (`ROLLBACK_POPULATIONS`, `TASK_AREA_FILES`, `DESTROYING_DOORS × Disposition`, `suppressed.door`, 9 of 10 `RelocateRefusal::ALL`) · `probe-unreliable`, the 7th `STORE_EXIT_FLIPS` member · `store.no-such-leaf`, the 2nd `ENVELOPE_OWED_CODES` member · `cwd-unreadable` and `pack-resource-missing` · **CX-8's third origin pack** · **CX-9 as behaviour** | nine, each with the state or fixture it would need. `CX-13` is **partially confirmed** where drivable and open for the rest |

Axes 2, 3, 5 and 8 each record **zero** open leads, explicitly rather than by omission: *"every Codex claim
was driven to a repro or to a falsifying datum on this binary; no claim required a state the rig cannot
build"*. Axis 3 adds the reason a reader would want: *"recorded explicitly so a reader can tell an empty
list from an unwritten one."*

---

# HONEST BOUNDS — what each axis states it did **not** drive

Each axis file carries its own *"what I did NOT drive, and why"* section; this is the roll-up. Nothing here
is presented as driven, and **no bound is inferred** — each is quoted or paraphrased from the axis that
declared it.

**Axis 1 · caller tokens.** 191 (door, cell) pairs `n/a` in five stated groups: `relocate` × the 9
non-control cells on a *stock* corpus (the door refuses at the doctype before `--from` is read; the cells
were driven on the manifest-less pack instead) · address doors × *untracked in-repo* and *stdin sentinel*
(an address head is an identity, never a filesystem path) · `SLUG_DOORS` × the same two · the 25
`WORK_UNIT_ID_DOORS` × the six filesystem-shaped cells (the guard is one equivalence class; three members
of it answer identically at all 25 doors) · `RelocateRefusal::UntrackableDestination`, **1 member of 10**,
which needs a hand-authored schema homing inside `.git/` — the engine-side axis M49 declared open. Also not
driven: the `owned-location` gate's presence and trackedness legs, and **any concurrent drive** — every row
is single-process.

**Axis 2 · posture.** `GIT_DIR` at 11 of the 12 acting doors (a property of the probe and the ambient
environment, declared out with a written reopening condition) · `SeamSubject::verify`'s identity-drift and
expected-ref-drift legs · `milestone finalize`'s two `COMMITTING_DOORS` rows as distinct posture rows (the
guard is leaf-keyed) · **the `task finalize` seam raced by a *merge*, attempted and not reachable** — git
refuses to begin a merge over the index jigc has just staged, measured, and the cell was reached with
`git bisect start` instead, *"recorded so the failed attempt is not mistaken for a passing one"* · a genuine
concurrent process rather than a shim racer · non-`main` default branches, submodule worktrees,
`core.worktree` redirects · **git versions other than the one installed** — *"every marker fact here is that
git's on-disk contract"*, which is what bounds DEFECT A.

**Axis 3 · destroying doors.** Enumerated per (door, cell) in its §4. Its fixture honesty note records which
plants were written into `.jigc/` directly rather than reached by driving.

**Axis 4 · transaction / rollback.** Nine stated groups, the load-bearing ones being **cell D in full**
(no deterministic in-transaction racer exists before the stage — reached *by driving* rather than inherited
from M52's VERDICT) · cell B at `doc author` and `config set` (neither window spawns a subprocess a fixture
can put user code into; both rows' *unraced* rollbacks are driven) · a genuine concurrent process · the
`chmod 0555` cell carrying M52's platform bound (**it passes vacuously as root, and this run was not root**).
**Fixture honesty:** two fixtures wrote into `.jigc/` rather than reaching a state by driving — the
`source-path` rewrite (the only way to reach a guard that exists *because* that value can be corrupted; the
caller-typed route in is refused at the door, driven) and the `.jigc/.gitignore` trims, which are the
user-co-owned file the cell is about.

**Axis 5 · pinned contracts.** Nine classes, each handed to the axis that owns it: 3 of 10
`RelocateRefusal` members · 9 of 11 `ROLLBACK_POPULATIONS` rows · 2 of 6 `DESTROYING_DOORS` · 5 of 9
`InProgress::ALL` members · `ManifestKind::ALL` and `SchemaChangeKind::ALL × LOCI` (they shape values
*inside* two envelopes, both of which **are** driven) · `ENVELOPE_ARMS` under a project-pack composition ·
`hook_output`'s value across the whole producer axis · **a concurrent / fan-out envelope** (`join` and
`milestone finalize | Landed` driven single-process through a real provisioned worktree; the genuine
Task-tool spawn is the standing honest bound M51's VERDICT carries, **unchanged by this review**) ·
`store.no-such-leaf` at a second producer (there is only one, *"stated rather than presented as a 1-of-N
sweep"*).

**Axis 6 · composed surfaces.** `cwd-unreadable` of `PRE_DISPATCH_FAULTS` (a harness manoeuvre the two-step
rig eval does not support cleanly — flagged for the reconciler and driven at axis 5's 47-leaf sweep
instead) · the registries no composed surface intersects · `migrate … --approve` (axis 7's) · **C1
end-to-end for 8 of the 17 composed workflows** — those eight were composed and *mechanically* checked
against clap, and their chains were **not run to a commit** · `describe`'s 34 narration bodies and 27 of 31
command hints · the `doc list` `orphaned` state and the store sweeps (axis 7) · the `GIT_DIR` redirect and
the posture family proper (axis 2).

**Axis 7 · freeze & migration.** Ten groups, the load-bearing ones being **40 of the 54
`SchemaChangeKind × LOCI` cells** (7 distinct kinds driven, chosen to hit every *disposition* and both
`Relocated` home kinds; each remaining cell needs its own manufactured pack + snapshot + re-pin) · all 6
`ManifestKind` cells × all 8 doors, **n/a with a driven reason** · three doors at the pack-layer hash-move
cell (*"stated rather than extrapolated into driven rows"*) · **the methodology pack's own manifest** — every
hash-move moved the **dev** pack's, so per-origin manifest resolution was not independently drifted.

**Axis 8 · adopter docs & help.** Nine groups, including: `migrate-corpus`'s per-door state clause and the
`unfilled` non-empty arm (axis 7's fixture — *"driving it here would prove axis 7 twice"*) · **the fan-out
doors' behaviour** — their `--help` and count claims were driven, **not a live fan-out**, so `milestone
finalize` as the second `Displace` door is un-driven here · `setup --force` over a dirty
`.git/hooks/pre-commit` (the declared bound at `install_candidate_paths`, unchanged since M51) ·
`cargo install` from an adopter repo (**not re-run**, because it would replace the `1.0.0-rc.16` this review
is measured against) · a recorded config delta actually drifting · the Tier-4 record corrections (no verb to
drive) · 12 of the 14 `suppressed.door` members individually (the **set-level** claims are driven).

**Two bounds hold across the whole review, both carried from M51 and unchanged by it:**

1. **No genuine concurrent process** raced any seam or any pre-image at any axis. Every race here is a
   deterministic `git` shim or a `pre-commit` hook — the instruments the design names. The live Task-tool
   fan-out spawn remains the orchestrator's main-session artifact, which this review did not run.
2. **`chmod 000` / `chmod 0555` cells pass vacuously as root.** This run was not root, and the axes that
   used them say so.

---

# COVERAGE — all 47 `VERB_KINDS` leaves

**The count, read from the code at `e519e4eb`.** `crates/cli/src/cli.rs:1834` → `VERB_KINDS` carries
**47** leaf rows — extracted by reading the `(&[…], VerbKind::…)` rows of the const itself, **47**, matching
the acceptance design's number and M51's. The leaf spellings and their order are byte-identical to M51's
table, so the two tables are directly comparable row by row.

**What the table reports.** §17's fence — *every leaf appears in ≥1 axis matrix* — is satisfiable by one
axis alone, which M51's §19 recorded as *true but weak* and strengthened: the column below reports **which
axes reach each leaf**, a leaf reached by no axis is `uncovered(<reason>)`, and a leaf reached only by
axis 5 is `only-5(<reason>)` carrying the same reason obligation. **`uncovered`: none. `only-5`: none.**

| # | leaf | M52 axes (driven rows) | M51 | Δ |
|---|---|---|---|---|
| 1 | `start` | 1, 2, 5, 6, 8 | 1, 2, 5, 6, 8 | — |
| 2 | `workflow` | 1, 2, 5, 6, 8 | 1, 5, 6, 8 | +2 |
| 3 | `setup` | 2, 4, 5, 6, 8 | 2, 4, 5, 6, 8 | — |
| 4 | `uninstall` | 2, 3, 5, 8 | 2, 3, 5, 8 | — |
| 5 | `upgrade` | 2, 5, 8 | 5, 8 | +2 |
| 6 | `ingest` | 1, 2, 5, 7, 8 | 2, 5, 6, 7 | +1, +8 · **−6** |
| 7 | `migrate` | 1, 2, 4, 5, 6, 7, 8 | 1, 2, 4, 5, 6, 7, 8 | — |
| 8 | `migrate-corpus` | 2, 4, 5, 7, 8 | 2, 4, 5, 7, 8 | — |
| 9 | `unmanage` | 1, 2, 5, 7 | 1, 2, 5, 7 | — |
| 10 | `rename` | 1, 2, 4, 5, 8 | 1, 2, 4, 5, 8 | — |
| 11 | `relocate` | 1, 2, 5, 7 | 1, 2, 5, 7 | — |
| 12 | `describe` | 2, 5, 6, 7, 8 | 5, 6, 7 | +2, +8 |
| 13 | `validate` | 1, 2, 5, 7, 8 | 1, 2, 5, 7, 8 | — |
| 14 | `doc create` | 1, 2, 5, 6 | 1, 5 | +2, +6 |
| 15 | `doc add-item` | 1, 2, 5, 6 | 1, 5 | +2, +6 |
| 16 | `doc remove-item` | 1, 2, 5 | 1, 5 | +2 |
| 17 | `doc retitle-item` | 1, 2, 5 | 1, 5 | +2 |
| 18 | `doc rename` | 1, 2, 5, 8 | 1, 2, 5, 8 | — |
| 19 | `doc set-field` | 1, 2, 5, 6 | 1, 5, 6 | +2 |
| 20 | `doc set-slot` | 1, 2, 5, 6 | 1, 5, 6 | +2 |
| 21 | `doc author` | 1, 2, 4, 5, 6 | 1, 5, 6 | +2, +4 |
| 22 | `doc show` | 1, 2, 5, 6, 7, 8 | 1, 5, 6, 7, 8 | +2 |
| 23 | `doc schema` | 1, 2, 5, 6, 7, 8 | 1, 5, 6, 7, 8 | +2 |
| 24 | `doc list` | 1, 2, 5, 6, 7, 8 | 1, 2, 5, 6, 7, 8 | — |
| 25 | `task list` | 2, 5, 6, 8 | 5, 6 | +2, +8 |
| 26 | `task diff` | 1, 2, 5 | 1, 5 | +2 |
| 27 | `task validate` | 1, 2, 5, 6, 8 | 1, 5, 6, 8 | +2 |
| 28 | `task discard` | 1, 2, 3, 4, 5, 8 | 1, 2, 3, 4, 5, 8 | — |
| 29 | `task finalize` | 1, 2, 3, 4, 5, 6, 8 | 1, 2, 3, 4, 5, 6, 8 | — |
| 30 | `task bind` | 1, 2, 4, 5, 6, 8 | 1, 5, 6 | +2, +4, +8 |
| 31 | `config set` | 1, 2, 4, 5, 6, 8 | 1, 2, 5, 6, 8 | +4 |
| 32 | `config insert-step` | 1, 2, 5 | 1, 5 | +2 |
| 33 | `config replace-step` | 1, 2, 5 | 1, 5 | +2 |
| 34 | `config remove-step` | 1, 2, 5 | 1, 5 | +2 |
| 35 | `config fill` | 1, 2, 5 | 1, 5 | +2 |
| 36 | `config fork` | 1, 2, 5 | 1, 5 | +2 |
| 37 | `config get` | 1, 2, 5 | 1, 5 | +2 |
| 38 | `config list` | 2, 5 | 5 (**`only-5`**) | +2 — **the last `only-5` leaf closes** |
| 39 | `milestone create` | 1, 2, 4, 5, 6, 8 | 1, 2, 4, 5, 6, 8 | — |
| 40 | `milestone add-task` | 1, 2, 3, 4, 5, 6, 8 | 1, 2, 4, 5, 6 | +3, +8 |
| 41 | `milestone add-from-spec` | 1, 2, 4, 5, 8 | 1, 2, 4, 5 | +8 |
| 42 | `milestone list-tasks` | 1, 2, 5, 8 | 1, 5 | +2, +8 |
| 43 | `milestone provision` | 1, 2, 3, 4, 5, 6, 8 | 1, 2, 3, 4, 5, 6, 8 | — |
| 44 | `milestone execute` | 1, 2, 5, 6 | 1, 5, 6, 8 | +2 · **−8** |
| 45 | `milestone join` | 1, 2, 4, 5, 6 | 1, 2, 4, 5, 6, 8 | **−8** |
| 46 | `milestone finalize` | 1, 2, 3, 4, 5, 6, 8 | 1, 2, 3, 4, 5, 6, 8 | — |
| 47 | `milestone discard` | 1, 2, 3, 4, 5, 8 | 1, 2, 3, 4, 5, 8 | — |

**`uncovered(<reason>)`: EMPTY** — every one of the 47 leaves is the door of ≥1 driven row in ≥2 axes.
**`only-5`: EMPTY** — down from M51's one (`config list`) and the design's planned three. The three the
acceptance design expected (`task list` · `config get` · `config list`) are each now reached by **3–4**
axes, because axis 2 drove **all 35 `Neither` leaves as controls** rather than M51's 10, and axis 8's door
set grew from 23 to 28. The `only-5` reason column is therefore empty rather than inherited; M51's reason
for `config list` (*"takes no argument at all; no posture, no write, no destruction — its contract is its
envelope"*) is recorded here only as the explanation of why it was the last to close.

## Differences against M51's driven coverage table

**Per-axis door counts, as each axis file states them:**

| axis | M51 | M52 | what moved |
|---|---|---|---|
| 1 | 39 | **40** | `+ingest` (the adoption route over a leading-dash filename) |
| 2 | 22 | **47** | all 12 acting doors across every cell **and all 35 `Neither` leaves as controls**, where M51 drove 10 |
| 3 | 6 | **7** | `+milestone add-task` (the door of a Codex rollback lead — *"the rule is 'door of ≥1 driven row', not 'door in the registry'"*). `DESTROYING_DOORS` is now **6** members and **all six are covered** |
| 4 | 13 | **16** | `+doc author`, `+config set`, `+task bind`, all reached through `ROLLBACK_POPULATIONS` and `ENVELOPE_OWED_CODES` |
| 5 | 47 | **47** | unchanged — the four uniform sweeps drive every leaf |
| 6 | 23 | **24** | `+config set` (added by the reconciliation: `jigc config set finalize.fan-out.squash false` driven at exit 0 as a step-command check; the driver drove `--help` shapes only) |
| 7 | 10 | **10** | unchanged — 8 door-set members + the same 2 *evidence doors* (`doc show`, `relocate`), listed so the diff can see them rather than to claim them |
| 8 | 23 | **28** | `+start`, `+workflow`, `+upgrade`, `+doc list`, `+milestone list-tasks` and others from the widened help/guide sweep; `−milestone execute`, `−milestone join` |

**Three leaves lose an axis, and none loses coverage. Each is stated by the axis that dropped it:**

1. **`ingest` loses axis 6.** Axis 6's door list names it among *"another axis's subject … not reached
   here"*; the axis's cell set this run was `PRE_DISPATCH_FAULTS` × the composed doors and the three read
   verbs, and `ingest` is neither. **`ingest` is driven at four axes (1, 2, 5, 7)** — one more than M51's
   three — and axis 7 drives it across **seven** freeze/adoption conditions, where A7-F2 lives.
2. **`milestone execute` loses axis 8.** Axis 8 drove the fan-out doors' `--help` and the count claims that
   name them, *"not a live fan-out"*, and records that explicitly in its *what I did NOT drive* list.
   **Still driven at four axes (1, 2, 5, 6)**, including axis 6's D-4.
3. **`milestone join` loses axis 8.** Axis 8 names it in its own words — *"`milestone join` is explicitly
   NOT covered — no row of either pass drove it, and it is named here only to say so"*. **Still driven at
   five axes (1, 2, 4, 5, 6)**, including axis 4's cell C4 and axis 6's fan-out drives.

**Assessment against the rule.** The acceptance design's obligation is *no leaf may lose an axis*, and read
at the letter **three leaves did**. Read against what the fence protects — that no leaf becomes unexamined,
and that a leaf's examination is not reduced to one axis's sweep — the re-run is **strictly stronger than
M51's**: `uncovered` is empty, `only-5` is now empty where M51 had one and the design planned three, **44 of
47 leaves gained at least one axis**, the three that lost one each retain **four or five**, and every drop is
declared in the dropping axis's own *not driven* section rather than being silent. The three drops are
recorded here as **differences, not as satisfied**, so a reader can see the letter of the rule and its
substance separately and judge both.

---

# Files

| file | what it is |
|---|---|
| [axis-1.md](axis-1.md) … [axis-8.md](axis-8.md) | the eight **reconciled** axis files, verbatim as their reconcilers wrote them, each prefixed with one header line naming the binary and the date. Every file is the Opus driver's table **unchanged**, followed by the reconciliation ledger |
| [codex/axis-1-source-pass.md](codex/axis-1-source-pass.md) … [codex/axis-8-source-pass.md](codex/axis-8-source-pass.md) | the eight **Codex source passes**, verbatim, so every lead is auditable against the verdict the reconciler drove it to |
| this README | the roll-up: the row-by-row comparison against M51's §A, the re-disposition of its open leads, the tiered findings, the bounds, and the coverage table |

**The instrument itself** (the eight prompts and the Workflow script) is not re-copied here: it is preserved
once, at [M51/per-axis-review/instrument/](../../M51/per-axis-review/instrument/), and the acceptance design
names that directory as the thing re-pointed at this one. A tool nothing points at is a tool that gets
rebuilt.
