# Scope audit — M46, the combined pre-1.0 wave

The **Scope phase** of the [milestone-planning workflow](../../../implementation/milestone-planning-workflow.md)
for M46, run 2026-08-18 against HEAD `b06bf72` on binary `1.0.0-rc.11`.

Its subject is the [scope brief](../RC-1.0-gate/next-wave-scope.md), whose own
[handover](handover.md) says plainly: *"its ledger dispositions are reasoned but unverified by me.
Re-derive any disposition you are about to act on."* This document is that re-derivation, plus what
exercising the binary turned up that no ledger, brief or trial anticipated.

**Method.** The workflow requires the Scope phase to produce a *verified capability ledger* —
subagents exercising the real binary, not the orchestrator reading code — and states that an
existing recent audit does not discharge it, *even the audit that chartered the milestone*. Four
`capability-auditor` agents took one slice each, in throwaway repos under the session scratchpad; the
project repo was untouched throughout (`git status --porcelain` empty at `b06bf72`, verified after).
Every exit code was measured **unpiped** — redirected to a file, `$?` read directly.

**The gate at HEAD was re-measured, not inherited: 2735 passed / 0 failed, exit 0** — identical to
M48's recorded figure, so the baseline is a measurement rather than a claim.

**Provenance is split explicitly below.** Findings marked **[self]** were driven by the orchestrating
session directly; **[auditor]** were driven by a delegated auditor and are relayed with their repro.
Four of the six new defects were reproduced independently by the orchestrator after the auditor
reported them; those carry both marks. This split exists because the handover's first lesson is
*"reading a gate declaration is not running the command it gates"* — and it applies to reading an
agent's report too.

---

## 1 · Headline

**The brief's defect diagnoses are sound. Its cost estimates are systematically wrong in both
directions, and its scope is missing six defects — four of them exit-0 correctness, two inside code
M48 shipped to prevent that exact class.**

| | |
|---|---|
| Ledger dispositions re-derived | 12 of 12 |
| Dispositions confirmed as written | 5 |
| Confirmed with a material correction | 4 |
| **Refuted** | **3** (entries 5, 7, 10) |
| New defects, in no ledger/brief/trial | **6** |
| Recorded triggers found **fired** | **1** (entry 9) |
| Brief claims found false | 4 (Fork 4's framing ×2, Fork 6's blast radius, Fork 7's prescription) |

---

## 2 · The six new defects

Each is a defect nothing in the record anticipated. Classes are the trial protocol's §1 rows.

### N-1 · The `--ignored` axis: all four destroying doors delete gitignored bytes at exit 0
**[auditor] + [self, independently reproduced]** · *Class: data loss on a destroying path* ·
**In shipped M48 code.**

M48's centrepiece was a fail-closed leftover classifier over three destroying doors, guarded behind
`--force`. The guard is real and works — and it is blind on exactly one axis. Same door, same state
shape, opposite outcome:

```console
$ # ARM A — a gitignored file is the worktree's only content
$ printf 'API_KEY=real-production-key\n' > .jigc/worktrees/do-alpha/secrets.env
$ jigc milestone discard ign-probe ; echo "EXIT=$?"
discarded milestone:ign-probe (1 sub-task(s); workbench removed)
EXIT=0
$ [ -f .jigc/worktrees/do-alpha/secrets.env ] && echo "STILL THERE" || echo "GONE"
GONE

$ # ARM B (control) — a plain untracked file, same door, same shape
$ printf 'plain untracked\n' > .jigc/worktrees/do-beta/scratch.txt
$ jigc milestone discard ctrl ; echo "EXIT=$?"
blocking · milestone.dirty-worktree — milestone:ctrl: 1 sub-task worktree path(s) hold content, and
  `jigc milestone discard` would settle the record and tear the workbench down over them:
  …/do-beta: ?? scratch.txt — registered here, so the teardown removes it and this content is destroyed
  route: look inside those paths and get out what you need …— or re-run with `--force` …
EXIT=1
$ [ -f .jigc/worktrees/do-beta/scratch.txt ] && echo "STILL THERE (refused)" || echo "GONE"
STILL THERE (refused)
```

**Where the blindness lives.** `probe_leftover` reasons that for the `Unverifiable` / `NoOwnLinkage`
verdicts *"any content refuses — ignored files included: an ignored file is not work to git, but the
binary is not the one who gets to decide that about a directory it cannot even place."* The
`OwnWorktree` branch delegates instead to `dirty_worktrees` (`crates/cli/src/milestone.rs:2153-2177`),
which is `git status --porcelain` default and therefore ignore-blind. `discarded_work` adds
`--untracked-files=all` but not `--ignored`.

> **CORRECTED after the gap pass, twice over.** (i) The audit first wrote that *"the codebase
> declares the opposite policy in its own doc comment."* That overstates it: the comment is
> **scoped to the `Unverifiable`/`NoOwnLinkage` verdicts and is honored there** (`child_names` is a
> plain readdir, ignored files included). So this is a policy **never made for the `OwnWorktree`
> verdict**, not a decision made and lost — which changes it from "fix the contradiction" to
> "make the decision".
>
> (ii) **The cheap fix is not safe.** `design/team-ready-state.md:167` states a provisioned worktree
> arrives **tracked-only** *"while the sub-task walk tells the agent to build the code and run the
> tests"* — so after any sub-agent build, `--ignored` reports `target/` / `node_modules/`. Adding the
> flag at every door would make the guard **refuse on the success path of the ordinary fan-out**,
> training `--force` into muscle memory, which is how a guard dies. The two verdicts need separating
> before pricing: the `provision`-on-`cp -R` loss sits in `NoOwnLinkage`, the live-worktree loss in
> `OwnWorktree`, and they may take different answers.
>
> Rider owed with the decision: `DESTROYING_DOORS` (`milestone.rs:1817`) has **three** members and
> `design/storage.md:121` enumerates the same three by name. If `milestone finalize` joins the axis,
> the const and that locked sentence are revised in the same motion — the const is what acceptance
> iterates.

**[auditor]** measured the same axis at all four doors — `milestone discard`, `uninstall`,
`milestone provision` (on a non-registered leftover) and `milestone finalize` — destroying at exit 0
in each. The `provision` arm deleted `secrets.env` **and** `build/out.bin` on a `cp -R`-shaped leftover.

**The axis is one flag** (`--ignored`) **under four doors.** That is the M45 complete-fix lens
pointed at M48's own centrepiece.

---

### N-2 · `milestone finalize` silently degrades to a docs-only commit when worktrees are unreachable
**[auditor]** · *Class: wrong result on a destructive path, terminal* · **Refutes ledger entry 10's
disposition.**

`provisioned_worktrees` filters on **the registered set** — precisely the subject M48 retired at the
other three doors as *"structurally blind to the ordinary trigger: a `cp -R` or `mv` of the repo
leaves the copy's worktrees registered at the source's path."* That subject survives at `finalize`.

Two instances, both on ordinary user actions (`cp -R` is how every RC trial corpus is made):

```console
$ (in the copy) cd .jigc/worktrees/rework-the-low-cache-path && git diff --cached --name-only
README.md
src/low.rs                            # real, git-added sub-agent work
$ jigc milestone finalize cache-rework ; echo "EXIT=$?"
finalized 3cb8698 — Finalize milestone cache-rework (2 sub-tasks)
  promoted docs/decisions/lru-eviction.md
  2 files committed
  sub-tasks: rework-the-low-cache-path: 2 docs, no worktree provisioned · …
EXIT=0
$ git show HEAD --name-only --format=
docs/decisions/lru-eviction.md
docs/milestone-records/cache-rework.md     # three staged code files silently dropped
```

Blast radius: exit 0; the sub-agents' `git add`ed code never lands; the manifest **states a
falsehood** (`no worktree provisioned` while the directory sits on disk holding the work — a law-1
lie, not a warning); the record flips to terminal `joined`, so `milestone finalize` and
`milestone provision` both answer `milestone.terminal` at exit 1 afterwards — **no jigc route
recovers it.** The bytes survive on disk, so this is not data loss; it is unreachable work plus a
lying record.

M47's substitutes do close the arm they were built for — the zero-contribution shape refuses
correctly at exit 3. The gap is exactly the arm entry 10 names: **staged *persisted* docs survive, so
`milestone.zero-contribution` cannot fire, and the boundary degrades to a partial commit.**

---

### N-3 · Concurrent state writes are lost, each reporting success at exit 0
**[auditor] + [self, independently reproduced]** · *Class: law-1 lie on the success path* ·
**Ledger entry 9's recorded trigger has fired.**

Entry 9's trigger reads: *"two sub-agents' state writes demonstrably interleave and one is lost."*

```console
$ # six adopted ADRs, six concurrent `jigc unmanage`, state restored between trials
$ for trial in 1 2 3 4 5; do … done
trial 1: remaining=4 (expected 0)  exits=[0 0 0 0 0 0 ]
trial 2: remaining=5 (expected 0)  exits=[0 0 0 0 0 0 ]
trial 3: remaining=3 (expected 0)  exits=[0 0 0 0 0 0 ]
trial 4: remaining=5 (expected 0)  exits=[0 0 0 0 0 0 ]
trial 5: remaining=3 (expected 0)  exits=[0 0 0 0 0 0 ]
```

The ack printed by a process whose write was discarded:

```console
$ cat /tmp/u2.txt
unmanaged docs/decisions/decision-2.md (adr:decision-2) — dropped its file-state baseline + forward
edges; the file is left on disk. …
$ cat /tmp/e2
0
```

The baseline is still in `file-state.json`. **[auditor]** additionally reproduced it in the genuine
fan-out topology (three concurrent `jigc` runs, each from inside a different provisioned worktree:
1 of 5 trials lost an update) and confirmed the premise that makes it reachable — a `jigc` process
run from inside `.jigc/worktrees/<id>/` writes the **main** repo's `.jigc/state/file-state.json`.

**Why this changes the disposition rather than merely counting against it.** Entry 9's deferral rests
on *"contention, not corruption … no writer produces bytes nobody authored (T1 clear)."* That
statement is true and is **not the failure mode observed**. The failure is a writer producing an
**ack nobody earned**. This project has three times treated a law-1 lie on a success path as
blocking.

*Bound, stated:* `unmanage` was chosen as the seam because it is fast and has no git serialization.
The identical unlocked `load → mutate → save` shape exists at `ingest.rs:188/245`,
`rename.rs:391/397`, `task.rs:835`, `milestone.rs:879/914`, `relocate.rs:49/54`,
`unmanage.rs:59/77` — but one seam was priced, not all.

> **RETRACTED — this audit's own error, caught by the gap pass.** An earlier revision of this section
> stated: *"`tasks.json`, named in entry 9, does not exist at rc.11; task state is per-task under
> `.jigc/tasks/<id>/`, so that third of the entry's claim is stale."* **That is false.** It exists at
> `.jigc/milestones/<id>/tasks.json` — `pub const TASKS_FILE: &str = "tasks.json"`
> (`crates/engine/src/milestone.rs:41`), written at `:218` / `:286`, read by `read_task_list` (`:692`)
> — and `design/team-ready-state.md:172` documents the sharing as designed: *"shared by all N
> sub-agents — `.jigc/state/*`, `.jigc/index/*`, and the milestone's `tasks.json`."*
> **Ledger entry 9 is accurate on all three surfaces; do not strike the clause.**
>
> The provenance failure is worth naming, because it is the one this session's handover warned about:
> the claim came from a delegated auditor, was relayed into this document without being driven, and
> was then reported onward as fact. It survived one full reporting cycle. *An agent's report is a
> lead, not a measurement* — the same rule this document applies to doc statements now applies to
> its own sources.

---

### N-4 · Pre-guard item-slot corruption is repairable by no verb, and its blocking finding carries no route
**[auditor]** · *Class: blocking dead end + M43 route-floor violation* · **Refutes ledger entry 7's
retirement.**

M45's item-slot write guard holds — confirmed over depth × nesting × `set-slot`/`author`. But entry
7's retirement rests on *"a plain re-author through the write verbs fixes a pre-guard corpus"*, and
every door refuses:

```console
$ jigc doc show spec:pre-guard-spec ; echo $?
blocking · store.unparseable — … `### Rationale for the cap` sits at `###`, the schema-reserved item depth here
1
$ jigc doc set-slot spec:pre-guard-spec#criteria/burst-limit/statement --from-file clean.txt --task repair ; echo $?
blocking · write.non-reparseable — write rejected: the source does not conform to the schema
  route: nothing was persisted — revise the payload … or `jigc task discard <task-id>` and start over
1
$ jigc doc add-item spec:pre-guard-spec#criteria --title "Rationale for the cap" --task repair ; echo $?
blocking · write.wrong-shape — write rejected: section "criteria" last item not present
1
```

The payload is already conformant, so the `non-reparseable` route is a dead end; and the route the
*diagnosis* names — `conformance.item-heading-unanchored` says *"if it is a new item, mint it with
`jigc doc add-item` (which writes the anchor)"* — answers an unrelated `write.wrong-shape`. The only
repair is an out-of-band hand edit, the one action the contract forbids.

Additionally: the blocking `conformance.item-heading-unanchored` carries **`route: null`** in
`--format json` and prints no `route:` line.

> **CORRECTED after the gap pass — the route-floor half of this finding is REFUTED.** The audit first
> read this as *"a straight violation of M43's route floor."* It is not: the floor carries a
> **declared exemption**, and this code is inside it by construction.
> `engine::finding::is_route_exempt` (`crates/engine/src/finding.rs:262`) is literally
> `code.starts_with("conformance.")`, and `design/surface-contract.md` → The route fence re-affirms
> it on its own rationale — *"purely-positional parser conformance diagnostics (the located message
> **is** the repair)"*. The doc comment goes further and anticipates precisely this state:
> *"fix the named line; **no CLI verb repairs a hand-broken byte**, and any at-parse route would be
> a guess."* So *"only an out-of-band hand edit works"* is **declared design, not a defect**.
>
> **What survives, and is still real:** (i) the diagnosis's own prose names a repair that does not
> work — `conformance.item-heading-unanchored` says *"mint it with `jigc doc add-item` (which writes
> the anchor)"*, and `add-item` answers an unrelated `write.wrong-shape`; (ii) **entry 7's retirement
> is still refuted**, because its trigger fires on the state existing at all, not on the route; and
> (iii) `design/validation.md:39` still states the floor flatly as *"always present … never `null`"*
> while `surface-contract.md` carves the exemption — **two statements in one design set disagree in
> letter**, which is a docs fork for the Settle.

**Entry 7's own trigger** is *"a corpus carrying pre-guard item-slot corruption that a plain
re-author through the write verbs cannot fix."* That is not a hypothetical condition; it is rc.11's
reproducible behaviour.

---

### N-5 · `doc-code.criterion-maps-to-test` asserts a symbol is absent from a file that contains it
**[auditor, PHP/Pest] + [self, independently reproduced on TypeScript/vitest]** ·
*Class: law-1 lie on a blocking path* · **Refutes ledger entry 5's retirement.**

Reproduced on **two languages by two parties**, which lifts this from an instance to an axis:
closure-registered tests in general, not one framework.

```console
$ grep -n "rejects a burst beyond the cap" tests/rate_limit.test.ts
3:it('rejects a burst beyond the cap', () => {
$ jigc task validate spec-the-limiter ; echo "EXIT=$?"
blocking · doc-code.criterion-maps-to-test — anchor `tests/rate_limit.test.ts#rejects a burst beyond the cap`
  resolves to no symbol (`rejects a burst beyond the cap` is absent from `tests/rate_limit.test.ts` in the staged index)
  route: if the cited code is on disk but unstaged, `git add` it — finalize adjudicates the staged index,
  not the working tree; otherwise update the citation to match the renamed/moved code, or restore the
  cited symbol (e.g. revert the change)
EXIT=3
```

All three routes disproved against the exact tree the message names:

```console
$ git ls-files --cached -- tests/rate_limit.test.ts        # route 1: "if unstaged, git add it"
tests/rate_limit.test.ts                                   # → already in the staged index
$ git show :tests/rate_limit.test.ts | grep -n "rejects a burst beyond the cap"
3:it('rejects a burst beyond the cap', () => {              # → present in the INDEXED blob
$ git log --oneline --diff-filter=RD -- tests/rate_limit.test.ts    # routes 2 and 3: renamed/moved/deleted
                                                           # → no rename, move or delete in history
```

And the repair that works produces **no finding at all**, while being named by the pack step and not by
the finding:

```console
$ jigc doc set-field …/maps-to-test --value 'tests/rate_limit.test.ts' --task spec-the-limiter
$ jigc task validate spec-the-limiter | grep doc-code
                                                           # → no doc-code finding
```

```console
$ jigc task validate implement-burst-rejection ; echo $?
blocking · doc-code.criterion-maps-to-test — anchor `tests/php/RateLimitPest.php#rejects a burst beyond the cap`
  resolves to no symbol (`rejects a burst beyond the cap` is absent from `tests/php/RateLimitPest.php` in the staged index)
  route: if the cited code is on disk but unstaged, `git add` it … otherwise update the citation to match the
  renamed/moved code, or restore the cited symbol (e.g. revert the change)
3
```

The file contains `it('rejects a burst beyond the cap', function () {…});` verbatim. All three
offered repairs are inapplicable (the file is staged; nothing was renamed, moved or deleted). The one
repair that works — the file-only fallback — is named by the pack step and **not by the finding**.

Two riders: M45's integrity rider landed in `crates/cli/pack/steps/locate-from-spec.yaml` only —
`author-migration-spec.yaml` still prescribes `<path>#<test-fn>` with no closure caveat and tells a
migrating agent to carry a cited test *"over verbatim"*, manufacturing exactly this block on a
Pest/vitest corpus. And the sanctioned fallback is the quietest path: a file-only anchor pointed at
**production code with no test in it** produces no finding at all.

---

### N-6 · The text renderer drops finding identity that the JSON seam preserves
**[auditor] + [self, verified at the seam]** · *Class: surface defect on a blocking path* ·
**An axis, not an instance.**

Renaming one symbol that three managed components anchor produces three **byte-identical** blocking
lines in both `--format agent` and `--format human`:

```console
blocking · doc-code.symbol-exists — anchor `src/feed.ts#WebSocketFeed` resolves to no symbol (…)
blocking · doc-code.symbol-exists — anchor `src/feed.ts#WebSocketFeed` resolves to no symbol (…)
blocking · doc-code.symbol-exists — anchor `src/feed.ts#WebSocketFeed` resolves to no symbol (…)
```

while the JSON carries three distinct addresses (`arch-doc:ingest-pipeline#components/…`,
`arch-doc:rollup-surface#components/…` ×2). The worker cannot tell how many docs are affected, or
which.

**[self]** The cause is general, not specific to this code. `finding_line`
(`crates/cli/src/render.rs:2312`) renders only `severity · code — message` plus the route; it **never
prints `finding.location.address`**. Identity therefore reaches the text surface only when the
*message* happens to carry it. `doc-code.symbol-exists`'s message is built from the anchor value
(`crates/cli/probes/doc-code/src/main.rs:192`), not the owning doc.

So the class is: **any finding whose message does not self-identify is indistinguishable in text
while remaining distinct in JSON.** A fix at the instance would leave the axis open. Note the
contrast inside the same probe — `title-names-symbol`'s route *does* name its doc.

---

## 3 · The twelve ledger entries, re-derived

| # | Brief's disposition | Verdict | Basis |
|---|---|---|---|
| 1 | RE-KEY post-1.0 | **CONFIRMED** | Quote at `design/self-hosting.md:79` verified verbatim. Nothing at rc.11 records a cleared gate: the `Checkpoint:` string is read-path only, the task area holds no gate field, no verb could record one, and the invocation-log record shape has no field that could carry it. The corrected arms (attested-prose vs CLI-observed) hold. |
| 2 | RE-KEY post-1.0 | **CONFIRMED, basis wrong** | **Not a frozen-schema event and does not expire** — see §4. Its real cost is ordinary doctype registration. |
| 3 | SPLIT (noise IN, ledger RE-KEY) | **CONFIRMED, basis expired** | Floor now measured on rc.11: **9 firings / 16 components, on one surface only** (`jigc validate`). The unrelated task gate, unrelated finalize and pre-commit hook print **zero** — M47 Inc 7's `blocking_probes` re-key works. The habituation mechanism the entry tracked for four trials is already gone; the 21-firing datum was rc.9. The F5 precision fix's outcome is now determinate without building it: **drains 5 of 9; 4 survive**, and those 4 are exactly the honest-title class. |
| 4 | RE-KEY post-1.0 (both halves) | **CONFIRMED, size refuted** | Gap is real and counted: **0 occurrences** of search/grep/filter/references across **29,886 bytes** of the entire help tree; an unregistered doc cannot be read at all. But *"a wave of its own"* is an overestimate — see §5. |
| 5 | **RETIRE** | **REFUTED** | N-5. *"Nothing false remains standing"* is false on a blocking path. |
| 6 | RE-KEY post-1.0 | **CONFIRMED, one shape re-priced** | The *"80% already shipped"* hypothesis is refuted: the middle shape is **0% shipped as a surface, ~100% shipped as data** (`doc show --format json` returns `implemented-by` per component). A behaviour-only change to a symbol two docs anchor yields **exit 0, zero doc-code rows** — the identity-not-truth bound, live for the fourth time. |
| 7 | **RETIRE** | **REFUTED** | N-4. Refuted by the entry's own trigger. |
| 8 | RETIRE | **EARNED, bounded** | Five real file-state states each named their path and a route that closed it; the JSON holds only `{path: sha256}` and answers nothing the findings do not. *Bounds:* fan-out contention (entry 9) unexercised — if N-3's residue ever produces a baseline nobody wrote, the motive returns; and `jigc unmanage` on a path that no longer exists acks *"the file is left on disk"* unconditionally, on the very verb its dangling-baseline route sends you to. |
| 9 | RE-KEY post-1.0 | **REFUTED — trigger fired** | N-3. |
| 10 | OUT by its own trigger | **CONFIRMED, far stronger basis** | N-2 supplies the measured silent-degrade instances the disposition said did not exist — *and* §4 shows the entry's natural shape is **unmigratable**, which is a better reason for OUT than the one recorded. |
| 11 | IN (Fork 7a) | **CONFIRMED, prescription wrong** | See §5. |
| 12 | ALREADY CLOSED | **CONFIRMED by exercise** | `hook_file` emitted and **correct on the branch where the naive path would lie**: under `git config core.hooksPath .githooks` it emits `.githooks/pre-commit` with `hook_committed: true`, and `.git/hooks` holds nothing. The entry's declared bound (*"a driver … still cannot learn where the hook landed"*) is **stale and should be struck, not carried.** |

---

## 4 · Fork 4 — mis-framed in both records

Both the ledger header and the brief's Fork 4 say **two** entries need a shape change to a frozen
methodology doctype. **That is false for entry 2.**

**Entry 2 — a new `planning-record` doctype — is not a frozen-schema event.** Exercised by adding a
spike schema to an export of HEAD and rebuilding:

- Shipping the schema alone blocks every door at exit 1 (`ManifestError::ExtraEntry`, strict
  set-equality over declared ∪ shipped, `crates/engine/src/manifest.rs:311-354`).
- Adding the 11th manifest entry is **3 YAML lines**; the other ten are untouched and re-verify clean.
- **Corpus migration: none.** `jigc migrate-corpus` → `0 migrated, 0 already current, 0 blocked`,
  exit 0 — a doctype that never existed has no instances.
- **No `schema-snapshots/` file** is required (snapshots are sourced only for a v*k*→v*k+1* migration).
- **CI manifest fence: clean by construction** — its own verdict table reads *"entity absent at base
  (added) → clean"* (`crates/cli/tests/manifest_freeze_fence.rs:31`). No `Manifest-Repin:` trailer.

**So entry 2 is not a one-way door and does not expire at the 1.0.0 call** — a new doctype is exactly
as cheap in 1.1, because it migrates nothing and no adopter corpus can hold an instance of a doctype
that does not exist. Its genuine cost is ordinary doctype-registration work
([doctype-authoring.md](../../../implementation/doctype-authoring.md)): the round-trip registry fence,
the read-back / stated-at / catalog-shape / suppression / allows-create fences once it ships an
author step, and a golden regeneration. **Entry 2 comes off Fork 4 entirely.**

**Entry 10 — `milestone-record` 2→3 — is a frozen-doctype event, and as naturally shaped it cannot
be migrated at all.** A required `set: on-transition` item field classifies as `AddedItemField`
(✅ in the authoring matrix, `placeable_without_prose` true because `field.set.is_some()` —
`crates/engine/src/schema_diff.rs:795-797`), but has **no deterministic value at migration time**, so
the splice writes nothing and the doc fails its own conformance gate:

```console
$ jigc migrate-corpus ; echo $?                      # required-field shape, real v2 instance
blocking · migrate-corpus.prose-needed — … author the new required prose … through the write verbs
1
$ jigc doc set-field milestone-record:cache-rework#tasks/do-the-thing/provisioned --value yes ; echo $?
blocking · write.machine-maintained — the record is machine-maintained (every leaf is CLI-`set:`)
  … and no `jigc doc` write applies to it
1
```

A mutual dead end, on the one doctype where *every* leaf is `set:`-bearing. The `optional: true`
shape migrates clean (`1 migrated`, byte-stable, self-committing) — but makes the "durable fact"
absent-by-default, and so weaker as a guarantee.

> **CORRECTED after the gap pass — "unmigratable" is true of one shape, not of the entry.** A third
> arm exists and is proven by a standing test at HEAD.
> `placeable_without_prose` (`crates/engine/src/schema_diff.rs:790-797`) is
> `optional || default.is_some() || set.is_some()`, and `apply_added_item_field`
> (`crates/engine/src/transform.rs:430-434`) splices **`decl.default` first**, erroring only when
> there is no default *and* the field is not optional. So a **required, deterministic,
> defaulted** field — e.g. `{ id: provisioned, type: enum, of: [yes,no,unknown], default: unknown,
> set: on-transition }` — migrates byte-stable into every existing item:
>
> ```console
> $ cargo test -p engine added_item_field_with_a_default ; echo "EXIT=$?"
> test schema_diff::tests::an_added_item_field_with_a_default_classifies_added_item_field ... ok
> test transform::tests::added_item_field_with_a_default_splices_the_value_into_every_item ... ok
> test result: ok. 2 passed; 0 failed
> EXIT=0
> ```
>
> **Consequence for Fork 4:** its arm list must be re-cut. *Required-with-default* is neither
> "unmigratable" nor "absent-by-default and therefore weak", so the fork can no longer be posed as
> a choice between a dead end and a weakened guarantee.

**Consequence for the Settle:** Fork 4 is a **single-entry decision about entry 10**, and it should
be posed carrying the `prose-needed` / `write.machine-maintained` dead end, which nothing in the
record mentions. The adopter-facing cost the window closes on is real and was exercised: stock rc.11
against a v3 corpus gives `schema-conformance.schema-version-ahead` at **exit 1 store-wide** until
the adopter upgrades. Pre-1.0 that population is zero.

---

## 5 · Four brief claims that do not survive exercise

**Fork 7's prescription is wrong, and the fix is cheaper than priced.** The brief says reuse M48's
fail-closed leftover classifier at `milestone finalize`. That classifier's subject (`probe_leftover`
→ `dirty_worktrees`) is the whole porcelain union — at `finalize` the staged set **is the payload**,
so it would refuse every legitimate fan-out finalize. The correct subject already exists:
`discarded_work` (`crates/cli/src/milestone.rs:3621`) partitions on the index column and `continue`s
on wholly-staged files — and is **already called pre-commit** at `milestone.rs:3556` inside
`subtask_contributions`, on the exact worktree list finalize will tear down. The refusal is a branch
on a value the boundary already holds. The M48 adjudication's *"the probe unifies"* is the part to
correct: the two probes must stay **split by door**.

**Fork 6's blast radius is one finding code, not a class.** The brief says Fork 6 *"changes what the
task gate prints for **all** store-scope findings"* and therefore re-opens two headline trial
results. Measured: **`doc-code` store rows are already task-scoped**; only
`schema-conformance.unadopted-instance` rides an unrelated task gate. It is advisory and never flips
an exit, so the 17-non-zero-exit census and the route-floor result are untouched. Fork 6(a) is a
one-code fix following an established precedent.

The noise itself is worse than *"repeats"*: growth is **1:1 with foreign file count, uncapped** —
25 foreign files produce 25 findings on both `validate` and `finalize`, **23.6 KB** of findings about
files the task never touched, at exit 0, with no mute available.

**Entry 4's search verb is 1–2 increments, not a wave.** `run_list` (`crates/cli/src/doc.rs:3476`)
already enumerates every committed instance of every doctype, reads its bytes, parses it and renders
through a pinned JSON shape. `scan_prose_mentions` + `line_has_token` (`crates/cli/src/rename.rs:510`)
is a shipped, unit-tested, gitignore-respecting, word-boundary repo-wide scan emitting `path:line`,
welded to `rename` and needing extraction. The remaining cost is contract joins M48 performed twelve
times in one wave. The brief also **double-counts the latent-surface-sweep trigger**: the trigger of
record is *"≥3 user-facing verbs or ≥3 composed workflows"*, and one leaf verb does not fire it.

**Entry 3 has a cheaper drain than any fork lists.** The severity knob has no off member:

```console
$ jigc config set validation.doc-code.title-names-symbol.severity off ; echo "exit=$?"
blocking · config.value-rejected — `off` is not a valid value … (allowed: blocking, warning, advisory)
exit=1
```

A fourth enum member is materially cheaper than either the precision fix or the acknowledged-findings
ledger, and is on no arm list. Separately, a **sub-defect inside the check**: `TypeScript sample
shape` → `Sample` fires *while the title literally contains the symbol*, because non-compound tokens
are filtered out before comparison, making the exact-match escape hatch unreachable. Fixing only that
drains 3 of the 9.

---

## 6 · Two brief-flagged items, discharged

**The unverified carryover observation — reproduced, and it is not a defect. [self]** The brief flags
trial-record finding #4 (*"the carryover gate protects staging that exists at the mint, not staging
that existed before the session"*) as owed verification **before the Settle**. The axis is: staged →
unstaged before the mint → re-staged after it.

```console
$ # ARM 1 — the axis
$ git add foreign.md && git restore --staged foreign.md    # staged, then unstaged, BEFORE the mint
$ jigc start --workflow quick-fix "make a change"           # mint
$ git add foreign.md                                        # re-staged AFTER the mint
$ jigc task validate make-a-change
no findings — the task validates clean
$ jigc task finalize make-a-change --dry-run
finalize --dry-run — pre-commit manifest (nothing committed)
  added foreign.md
  modified seed.md

$ # ARM 2 (control) — staged AT the mint
blocking · finalize.carried-staged — `foreign.md` was already staged before this task existed …
blocking · finalize.carried-staged — `seed.md` was already staged before this task existed …
```

The gate behaves exactly as its own composed text states — *"anything still staged from **before this
task was minted**"*. `foreign.md` was not staged at the mint, so no refusal, and **the tool tells no
lie**. Re-staging after the mint is a deliberate `git add`; there is no data loss. **Recommendation:
record verified-not-a-defect and do not scope it.**

**Fork 3's hidden axis — the finalize-only set is larger than one, and contains a blocking member. [self]**
The brief hoped the set might be `{the changelog advisory}`, making Fork 3 a one-call-site decision.
`preview_gates` (`crates/cli/src/task.rs:936`) previews exactly two things: the carryover probe and
the six staging-independent `owner-artifact` causes. Driven live:

```console
$ jigc task validate base-probe ; echo "exit=$?"
no findings — the task validates clean
exit=0
$ jigc task finalize base-probe --dry-run ; echo "exit=$?"
blocking · finalize.base-mismatch — the task was started at base `070748e…` but HEAD is now `b1fab2f…`,
  and the moved history overlaps the task's work on `seed.md`
exit=3
```

So there is a **second, blocking** member of the class F-1's advisory belongs to, staging-independent
and previewable in exactly the way the carryover probe already is. No trial found it because no
session moved HEAD under a live task.

> **CORRECTED after the gap pass — `finalize.base-mismatch` is a recorded exclusion, not an
> oversight.** `design/command-output-contract.md:363` pins the preview contract and enumerates its
> exceptions: four items *"unpreviewable in principle"* plus *"a handful more (**the preflight's base
> pin**, promotion, the migration review hold) … **deliberately left out** of the preview"*, citing
> DECISIONS 2026-07-26 → M47 Settle Decision 1, and mirrored at `finalize.md:41`,
> `validation.md:610`, `write-commands.md:232`. The live repro above stands as a *measurement* of
> what a worker sees; it is **not** evidence of an unnoticed hole.
>
> **What the fork actually is, restated:** `changelog-recording.gate-granted-unused` is in
> **neither** enumerated half — neither previewed nor declared excluded — so the contract's
> enumeration is *incomplete*, and the narrow arm is that the advisory joins the preview as a missing
> member of the existing rule while the base pin stays excluded per M47. Anything wider **revises a
> pinned contract**: surfacing a *blocking* finalize-only finding at `task validate` flips that verb
> 0 → 3 on a shipped state, which touches the pinned exit-code taxonomy — so the brief's claim that
> *"nothing in this scope needs a contract change"* is false on the wide arm.
>
> The candidate list below is therefore a *census*, not a proposal; five of its six members may be
> declared exclusions.

**The precision that keeps this honest:** the composed surface does **not** over-promise. It says
*"previews **part of** the finalize gate: this task's content findings, the carryover gate, and the
owner-artifact causes that need no staging; the staged set, promotion and the commit surface at
finalize."* M47 scoped that truthfully. Two narrower defects remain: `finalize.base-mismatch` falls
into **neither** enumerated half, so the enumeration is incomplete rather than false; and the genuine
law-1 break is F-1's **route**, which offers an in-task option at a moment when in-task verbs are
dead. The source comment at `crates/cli/src/task.rs:1893` is the false premise that produced it.

**Candidate previewable set** (staging-independent finalize-only findings):
`changelog-recording.gate-granted-unused` · `finalize.base-mismatch` · `finalize.milestone-sub-task` ·
`finalize.promote-clobber` · `finalize.migration-no-replacement` · the `migrate.review-pending` gate.
`finalize.empty-commit` is genuinely staging-dependent and should stay put.

---

## 7 · The two named defects, root causes confirmed [self]

**PT-1.** `engine::validate::is_unadopted_foreign` has call sites in `crates/cli/src/doc.rs` (×2),
`crates/engine/src/validate.rs` (×2) and `crates/engine/src/file_state.rs` (×1).
`crates/cli/src/migrate_corpus.rs`: **zero**. The fix is reuse of a shipped predicate.

**F-1.** The predicate is `changelog_entry_count` (`crates/cli/src/task.rs:558`), a nested repeatable
**item** count staged-vs-committed — blind to any prose edit. `changelog_gate_advisory` has exactly
one call site, `task.rs:1190`, inside `finalize`. The comment at `task.rs:1893` states the advisory
*"is live on the `task validate` preview"*; it is not.

**One correction to a related claim. [self]** A refused write **does** leave a staged copy — verified:
a `write.slot-heading-depth` refusal at exit 1 left `spec:alpha-spec.md` in a task docs dir that held
only `commit:*.md` and `provenance.json` before. But the copy is **byte-identical to the committed
file**, and the *"nothing was persisted"* phrasing lives at `crates/engine/src/write.rs:5352/5358`,
attached to `write.non-reparseable` — **not** to the depth refusal. The copy-in is a side effect of
copy-on-write binding happening before validation. So the law-1 contradiction is real but narrower
than *"any refused write lies"*, and its downstream harm (a `reconciliation.conflict-block` at
finalize asserting writes the task never made) is **[auditor]**-observed, not reproduced here.

---

## 8 · Declared bounds on this ledger

- **This is a verified-at-`b06bf72` map, not gospel.** Every disposition above was exercised at one
  sha on one binary.
- **Provenance is mixed.** Four of the six new defects (N-1, N-3, N-5, N-6) were independently
  reproduced by the orchestrating session — N-5 on a *second language*, which is why it is stated as
  an axis. **N-2 and N-4 are relayed from auditors with their repros and were not re-driven**; they
  should be re-driven before their fixes are specified.
- **A near-miss worth recording, in the family of *"measure exit codes unpiped"*.** Checking whether
  the cited test file was staged, the first probe used `git diff --cached --name-only | grep` and read
  back `NOT STAGED` — which would have put a false fact in this record. `git diff --cached` shows
  *changes against HEAD*, not index membership; a tracked, unmodified file is in the index and absent
  from that output. The correct probes are `git ls-files --cached -- <path>` for membership and
  `git show :<path>` for indexed content. **`git diff --cached` is not the index.**
- **N-3 priced one seam, not all of them.** Six other call sites share the unlocked
  `load → mutate → save` shape; none were exercised.
- **Entry 3's 9/16 is a fire rate over an enumerated shape matrix**, deliberately constructed, not a
  naturalistic sample. The seven shape classes and their per-class drain verdicts transfer; the ratio
  does not.
- **The F5 drain scoring (5 of 9) is arithmetic** over the nine observed messages under a stated
  predicate, not an exercise of a built fix — the fix does not exist to run.
- **Entry 11's destruction was driven on the `squash: true` arm only.**
- **N-1's four-door measurement** came from one auditor; the orchestrator independently reproduced
  the `milestone discard` door and its control, not the other three.
- Not measured: whether `title-names-symbol` or `unadopted-instance` ride `jigc milestone finalize`.
- **Nothing here is settled.** The Scope phase establishes facts; the Settle decides.
