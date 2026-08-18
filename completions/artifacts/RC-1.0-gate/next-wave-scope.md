# Scope brief — the combined pre-1.0 wave (M46 + PT-1 + F-1)

**Status: scope brief, not a plan.** It says what is in, what is out, why, and which decisions are
genuinely open. It deliberately does **not** decompose into increments — that is the
[milestone-planning workflow](../../../implementation/milestone-planning-workflow.md)'s job, after
the Settle this brief exists to feed.

**The human's call, 2026-08-18:** M46 + PT-1 + F-1 in one wave → another trial → the 1.0.0 call.

**Inputs read in full:** [decisions-pending.md](../../../implementation/decisions-pending.md) → the
capability wave (M46) and the condition-keyed section · [roadmap.md](../../../implementation/roadmap.md)
· [pre-trial-findings.md](pre-trial-findings.md) · [findings-verification.md](findings-verification.md)
· [session-findings.md](session-findings.md) · [trial-record.md](trial-record.md) ·
[coverage.md](coverage.md) · [protocol.md](protocol.md) §1 · [CLAUDE.md](../../../CLAUDE.md).

---

## 0 · The number: **M46**, kept

The roadmap runs 45 → 47 → 48; **46 was reserved for the capability wave and never spent**. Twelve
ledger entries in `decisions-pending.md` name *"the M46 Settle"* as their literal trigger, and the
M48 adjudication table is keyed to that name. Renumbering to M49 breaks every one of those triggers
and buys nothing.

Out-of-order numbering is already the norm here (the roadmap carries Milestone 10's completion record
after Milestone 13). **Recommendation: build this as Milestone 46**, with the roadmap section placed
after Milestone 48 and a one-line note saying why the number is out of sequence. If the human prefers
strict monotonicity, M49 works — but the twelve triggers must then be re-keyed in the same commit,
and that is churn with no gain.

---

## 1 · The wave's claim

> **No entry leaves this wave deferred on a count.** Every one of the twelve M46 ledger entries
> either ships, is **retired as dead** (its motive gone, stated so it never returns), or is
> **re-keyed to a condition that can fire on its own** — and the two defects that can misroute a
> project-carrying file are fixed **at the predicate, not at the message**.

Judgeable against a build in the same way the prior claims were:

| Wave | Claim |
|---|---|
| M43 | the surface contract — three laws, fenced where the surface is generated |
| M44 | the pull tier — capabilities made to behave like gates |
| M45 | the complete-fix contract — a fix is complete over its class's axis, and completeness is machine-checkable |
| M47 | the surface-fundament wave — what surfaces *say*, never what surfaces *exist* |
| M48 | the pre-1.0 reliability + discoverability wave — generous scope under a razor that can still refuse |
| **M46** | **the disposition wave — a count is not a disposition** |

The claim's teeth: at close, **`decisions-pending.md` → the capability wave contains no entry whose
only recorded justification is that it was deferred before**, and neither `migrate-corpus` nor the
changelog gate can be shown a state where its own neighbouring surface contradicts it.

The claim deliberately does **not** say "every capability ships." Three waves have counted these
entries and refused them on their merits; the failure this wave fixes is that the *counting* never
ends, not that the refusals were wrong.

---

## 2 · In scope

### Group A — the two named defects

Both fixed over their class axis (the M45 contract), not at their reported repro.

---

**A1 · PT-1 — `migrate-corpus` claims a never-adopted foreign file and prints a route that cannot run**

- **§1 class:** *a blocking dead end — a refusal whose route cannot run* → **BLOCKS**, under the
  human's blast-radius qualifier. The reachable set was **demonstrated, not assumed**: any
  never-adopted file at a managed doctype's home, and two of those homes are repo-root files
  (`CHANGELOG.md`, `VISION.md`) plus `docs/decisions/*`
  ([findings-verification](findings-verification.md) §11, verified on a foreign root `CHANGELOG.md`).
- **Evidence:** CONFIRMED with live repros on rc.11 **and** rc.10 (not a regression). `migrate-corpus`
  blocks at exit 1 with `migrate-corpus.prose-needed` and a route to *"author the new required prose …
  through the write verbs"*; the doc id that route implies does not exist. `jigc validate` on the same
  file emits `schema-conformance.unadopted-instance` at exit 0 and states the other surface's premise
  is wrong in so many words — *"it is a foreign file, not an unmigrated managed doc."*
- **Root cause, located:** M42's managed-vs-foreign discriminator ships as
  `engine::validate::is_unadopted_foreign` and has **five call sites** — two in `crates/cli/src/doc.rs`
  (`ingest`, `doc list`), two in `crates/engine/src/validate.rs`, one in
  `crates/engine/src/file_state.rs`. **`crates/cli/src/migrate_corpus.rs` has zero.** The fix is
  *reuse of a shipped predicate*, not new design.
- **The axis** (what makes the fix complete): every surface that classifies a **committed file sitting
  at a managed home** must consult the one discriminator. That set is enumerable — the store walkers —
  so the acceptance is a registry/enumeration assertion in the M45/M47/M48 style, not a repro pin. It
  must iterate the **placement branch** as well as `docs-root` (root `CHANGELOG.md` / `VISION.md`,
  `docs/roadmap.md`), which is where the blast radius lives.
- **The trap to avoid, stated up front:** the fix must not become a **false green**. Excluding foreign
  files from the *blocking* set is right; excluding them from the *report* recreates exactly the class
  M42's placement fix and the confidence audit's above-current fix both closed. See **Fork 1**.
- **Likelihood, honestly:** 0 of 4 blind sessions ran `migrate-corpus`; the one worker that met the
  triggering file went `validate` → `ingest` → `migrate … --as adr` and adopted it correctly. The
  disposition is BLOCKING on **reach**, not on frequency, and the record should keep saying so.

---

**A2 · F-1 — `changelog-recording.gate-granted-unused` fires on a task that DID record an entry**

- **§1 class:** *a wrong result on a non-destructive path* → **SHIPS RECORDED**, and the human has
  pulled it into the wave anyway. Correctly so: its route actively instructs a duplicate of
  user-facing history.
- **Evidence:** CONFIRMED, boundary established on three tasks. The predicate is keyed on the **count
  of repeatable items** staged vs committed (`crates/cli/src/task.rs:1858`), so any prose-only edit to
  an existing item — `set-slot`, `retitle-item`, a `set-field` on a release date — falls on the wrong
  side. The advisory asserts nothing was recorded **in the same output that prints
  `promoted CHANGELOG.md`**. Identical on rc.10; the defect is M42's, carried through four waves.
- **Three distinct halves, and they should not be conflated:**
  1. **The predicate** — count-based, blind to prose edits. See **Fork 2**.
  2. **The routes point at each other** — the advisory's in-task option earns `write.already-present`,
     whose own route sends the worker back to `set-slot`, the verb that fired the advisory. The other
     option (`jigc start --workflow record-change`) produces the duplicate entry.
  3. **It is a post-commit ambush** — one call site, inside `finalize` (`task.rs:1190`);
     `jigc task validate` on the identical state exits 0 without it. That is a **sibling of M47
     Increment 4's own claim** (*"`jigc task validate` previews what finalize gates on"*) through an
     un-swept axis: advisories. See **Fork 3**.
- **S-4 rides with it:** the source comment at `crates/cli/src/task.rs:1893` states the advisory *"is
  live on the `task validate` preview"*. It is not. Not a §1 finding (internal comment, not a printed
  surface) — but it is the **false premise that produced the unrunnable route half**, so it is
  corrected in the same increment, not separately. This is the same failure shape the project has
  fixed twice at M47 and M48: a doc-comment premise outliving the behaviour it described.

---

### Group B — the M46 ledger, disposed entry by entry

The wave's claim binds here. Each entry leaves the ledger; the table below is this brief's
**recommendation**, and the Settle decides.

| # | Entry | Recommendation | Basis |
|---|---|---|---|
| 1 | Gate-command / evidence rung | **RE-KEY, post-1.0** | 3 demands, no new datum this trial. Its arms are still mis-posed (attested-prose vs CLI-observed); the corrected fork engages the determinism boundary and deserves its own decision, not a slot in a defect wave. Re-key to: *a trial produces a case where an unrecorded gate cost correctness, not discipline.* |
| 2 | Checkpoint / planning gate-record | **RE-KEY, post-1.0 — but see Fork 4** | Recorded trigger verifiably unfired at HEAD. Costs an 11th manifest entry or a frozen-doctype version bump + snapshot + migration. 5 demands and none of them a byte. |
| 3 | Acknowledged-findings ledger | **SPLIT: the noise half IN, the ledger half RE-KEY** | The trial adds a *new* noise instance in a different code (B3b-3, below), same class: a store-scope row riding a task gate. Take the scope fix; leave the ledger keyed to the F5 precision fix, unbuilt and untried. |
| 4 | Managed-doc read-side (search subclass) | **RE-KEY, post-1.0** | `ideas/doc-search.md` is a missing capability with a real design, and it engages VISION principle #3 head-on. Building a *guard* over a gap punishes the gap; building the *search verb* is a wave of its own. |
| 5 | Pest / non-Rust is-a-test tier | **RETIRE** | Untouched across four re-counts; the integrity rider landed at M45 and nothing false remains standing. Retire with the shape preserved in `ideas/multi-language-doc-code.md`. |
| 6 | symbol-mention-sweep de-park | **RE-KEY, post-1.0** | One adopter report, three unresolved shapes with very different costs. A defect wave has no room to choose well. |
| 7 | Item-slot repair verb | **RETIRE** | M45 closed the class over the write path; the verb would serve only pre-guard corpora, and none exist outside this repo. Its own trigger (*a corpus carrying pre-guard corruption*) is the retirement condition. |
| 8 | File-state introspection | **RETIRE** | Re-counted at **0 live diagnoses** post-M47; the demand that produced it was drained by M47 Increment 2. A capability with nothing demanding it. |
| 9 | Store locking / lost-update | **RE-KEY, post-1.0** | Contention, not corruption; the corruption subset shipped at M45. Its trigger is already a fact-shaped condition and can stay. |
| 10 | Durable "was provisioned" fact | **OUT by its own trigger — see Fork 4** | The recorded trigger was *"re-price only if the M47 substitutes leave a **measured** silent-degrade instance."* This trial drove the milestone arc (B2, live worktrees, join, finalize) and produced none. Keeps a frozen `milestone-record` 2→3 out of the wave. |
| 11 | `finalize`'s landed-teardown dirty-worktree guard | **IN — see Fork 7** | The one destroying door that does **not** satisfy the rule M48 made the other three satisfy. Recorded as data loss at **exit 0 on the success path**; under §1 row 1, a trial that found it would BLOCK. Shipping 1.0.0 with three guarded doors and one unguarded one is the asymmetry an adopter finds. |
| 12 | `setup --format json` `hook_file` | **ALREADY CLOSED — verified** | Shipped at M48: `crates/cli/src/render.rs:2365` emits `"hook_file"` in the envelope, with a test asserting it. Record it discharged; it is not scope. |

**Also in from the ledger's neighbourhood:**

- **N2 · `milestone provision`'s mid-mutation failure** ([coverage.md](coverage.md) Column 3). M48's
  audit MEDIUM made the *refusal* two-phase and transactional; a `git worktree add` failing **inside**
  phase 2 still leaves earlier paths provisioned with no rollback. **IN, if cheap** — the
  captured-pre-image discipline exists and was aligned across four staged-path families at the
  confidence audit; this is a fifth family left unaligned. Recoverable by idempotent re-run, which is
  why M48 took it as a bound — but "recoverable" is what entry 11 was told too.

---

### Group C — the trial's surface batch

Every CONFIRMED and PARTIAL finding gets a disposition here. All are §1 *surface/wording* or
*wrong-result* rows, all reversible, none touching a pinned `--format json` contract — so the
one-way-door qualifier promotes none of them.

| ID | Finding | §1 class | Disposition | The fix, in one line |
|---|---|---|---|---|
| **B1-1** | *"your staged changes are still staged"* in the hook-rejection frame reads false against `git diff --cached` on a docs-only task | surface | **IN** | Name the staging area: *"your task's staged docs are intact in `.jigc/tasks/<id>/`; your git index is unchanged."* Sweep over `COMMITTING_DOORS` — M47 built that enumeration; this is its un-swept vocabulary axis |
| **B2-1** | `no staged instance … provision it first (jigc doc create <type>)` names a verb the raising workflow's `allows-create` forbids | surface (PARTIAL) | **IN** | Drop the create option from the route when the active workflow's gate does not carry the doctype. **Note the correction:** the original "the pack instructs what its gate refuses" reading was **wrong** and is retracted in [session-findings.md](session-findings.md) Correction 3 — the printed step is runnable; only the *refusal's* route is not |
| **B2-2** | Seeding sub-tasks makes them un-resumable from the main tree; the blanket base-pin refusal mis-routes on its only subject | wrong result | **IN** | Route it at `jigc milestone provision <m>` / `cd .jigc/worktrees/<id>` — the mechanism it never names — and give `milestone add-task` the `next:` line it is missing. The `jigc task discard` option must stop running at exit 0 while leaving the milestone naming the discarded task |
| **B2-3** | `milestone finalize --help` enumerates only doc bodies; it commits code from every worktree | surface (PARTIAL) | **IN** | One sentence in `finalize`'s `about`. `join`'s help is **truthful** — the finding was aimed at the wrong verb |
| **B2-4** | No `--dry-run` for `milestone finalize` | capability gap | **IN — see Fork 5** | The direct measured cause of a destructive workaround (B2 `git reset --hard` on a sub-task worktree). Shape is settled by `task finalize --dry-run` |
| **B3b-1 / B3a-1** | The router's catalog is a closed list with no exit; `record-change` is correctly hidden and unreachable from it | surface | **IN** | One addendum to the router's closing text naming `jigc describe --workflows` as the fuller catalog. Reported independently by **2 of 4** sessions. Does not re-list hidden workflows — the suppression stays clean |
| **B3b-2** | `doc show --task` refuses `store.not-staged` immediately after `task bind` | surface | **IN** | Fix the **`bind` ack**, not the refusal: *"…bound for resume; it is not staged — read it with `jigc doc show <id>`."* Recorded because it is the read-back verb this trial measures, refusing where a worker reaches for it |
| **B3b-3** | An unadopted foreign file's advisory repeats on every unrelated task's `validate` **and** `finalize` | surface | **IN — see Fork 6** | Scope-aware suppression. Ties directly to ledger entry 3 |
| **D-1** | `jigc doc schema` is named by **0** pack steps, **0** times in `SKILL.md`, and once in `AGENT.md` filed under *"how jigc behaves"* rather than *"what shape is this doc"* | surface | **IN — see Fork 8** | The capability half is **REFUTED** — `doc schema` answers B1's stated need exactly. **Read the corrected evidence:** 3 of 4 workers found and used it 11 times; the one that missed it is the one session without the adapter preloaded ([session-findings.md](session-findings.md) Correction 2). This is *weaker* than a seventh discoverability landing and must not be scoped as one |
| **S-2** | `jigc describe <item>` answers with a bare clap exit-2 and no tip | surface | **IN** | M48's read-intent rule gives `jigc doc read` a helpful tip; `describe <item>` gets nothing. That is the read-intent rule's own **un-swept axis** — the `VERB_KINDS` work exists, this is a missing member |
| **S-4** | Source comment claims the changelog advisory is live on the `task validate` preview | unclassified (internal) | **IN, with A2** | Not a §1 finding; it is A2's false premise |
| **B3b-5** | No bulk/patch edit of existing items; `set-slot` is overwrite-only (no append) | capability gap | **OUT** — §3 | |
| **S-1 / B1-3** | No post-commit redo/narrow/split of a finalize | capability gap (PARTIAL) | **OUT** — §3 | |

**One unverified observation, flagged rather than scoped.**
[trial-record.md](trial-record.md) finding #4 — *"the carryover gate protects staging that exists at
the mint, not staging that existed before the session"* — is **not in the 17 verified claims** and has
no repro block. Its first reading was one of the three corrections the observer had to make (B1's
plant was defused before the mint, so the gate was never given the case). What remains unsettled is a
**real axis question**: paths staged, unstaged before the mint, then re-staged after it. **Verify it
before the Settle**; if it reproduces, it is a carryover-gate axis gap and its class is
*wrong result on a non-destructive path* at minimum. Do not scope it on the current evidence, and do
not drop it.

---

### Group D — the obligations that bind the human's own gate

**D1 · The RC-1.0-gate conversion ledger is open, and it is the stated gate on the 1.0.0 call.**
[CLAUDE.md](../../../CLAUDE.md) records it as the human's gate: *"the 1.0.0 call is not taken until
every trial repro block carries `pinned-by:` or a stated `UNPINNED: <why>`."* Measured today:

```
RC-pre-1.0/findings-verification.md   35  pinned-by:/UNPINNED   (closed at M48)
RC-1.0-gate/findings-verification.md   2  pinned-by:            (D-1 and S-3 only)
```

The new trial opened a new ledger and it is **~2 of 17 rows closed**. Closing it is in scope, is not
optional, and is not mechanically checkable — [pinning.md](../../../implementation/pinning.md) §3
refuses a symbol parser by name, so each citation is verified by reading what the cited test asserts.
See **Fork 9** for when in the wave it closes.

**D2 · Acceptance in the established shape.** Flow 49 in
[worked-examples.md](../../../design/worked-examples.md) + a `crates/cli/tests/flow49_acceptance.rs`,
each arm **enumerating its class's axis from a code-side registry** rather than pinning the reported
repro — the discipline M45 introduced and M47/M48 held. PT-1's axis (store-walking surfaces × managed
homes incl. the placement branch) and B1-1's (`COMMITTING_DOORS` × vocabulary) are the two that
genuinely have registries; the tier-2 wording batch does not and should not pretend to.

**D3 · The fold-back and the record.** DECISIONS entries per increment, the roadmap section, the
`decisions-pending.md` ledger rewritten to its post-disposition state (this is where the wave's claim
is checked), and the CLAUDE.md project-state paragraph.

---

## 3 · Explicitly out of scope

| Item | Why out |
|---|---|
| **B3b-5 · bulk/patch edit + append-to-slot** | A real design with a parked shape ([ideas/batch-authoring-ergonomics.md](../../../ideas/batch-authoring-ergonomics.md)). `doc author` refusing the whole payload on first collision is **documented behaviour the help text states up front** — it ambushes nobody. B3b's own judgment (*"might well be deliberate"*) is right. Post-1.0. |
| **S-1 · post-commit redo / narrow / split** | Collides with the finalize transaction model, which is a load-bearing invariant (*writes are transactional; integrity holds at the commit boundary*). The **pre**-commit half is already served and was in front of the worker (`--dry-run`, the `left-out` block). Post-commit surgery is git's job and the worker did it correctly. Post-1.0. |
| **`ideas/doc-search.md` — a search verb** | Ledger entry 4's search subclass. The largest genuine capability gap in the ledger and the named *cause* of read-side bypasses — and precisely for that reason it deserves a wave, not a slot. |
| **Ledger entries 1, 2, 6, 9** | Re-keyed post-1.0 with conditions that can fire on their own (see the Group B table). None is a byte. |
| **Ledger entries 5, 7, 8** | **Retired.** Motives measurably drained; retiring beats a fifth re-count. |
| **Ledger entry 10 — the durable "was provisioned" fact** | Out **by its own recorded trigger**: it re-prices only on a *measured* silent-degrade instance, and this trial drove the milestone arc without producing one. Keeps a frozen-doctype `schema-version` 2→3 out of the wave. Fork 4 is the only thing that changes this. |
| **N1 · F7's novel omission phrasing** | A declared bound in [M48/VERDICT.md](../M48/VERDICT.md), and unreachable **by construction** — a trial cannot drive the absence of a phrasing, and a general fence would be the grep the M42 lesson forbids. |
| **N3 · adapter-artifact delta discipline** | A declared deviation from principle #5 with its own trigger: *a second adapter artifact appears.* Unfired. |
| **The repo publishing floor (LICENSE, root README, `publish=false`)** | Its trigger is *first external adopter / public release / opening the repo* — none of which the 1.0.0 call by itself is. Flagged here because "1.0.0" reads like publication and this entry is what would be owed if it is. **Ask the human whether 1.0.0 means outward.** |
| **The F5 `title-names-symbol` precision fix** | Ledger entry 3's build half. The measured 21-firing floor is from project-alpha-4.0 and was never re-measured on rc.11; the fresh trial corpora are too small to show it. Fixing an unmeasured floor is the wrong order — the recorded trigger already says *precision fix first, ledger only if the floor survives*, and the precision fix needs a floor to aim at. |
| **Anything that changes a pinned `--format json` contract** | M48 closed the additive-key window and took `doc schema` to contract-version 5. Nothing in this scope needs a contract change, and the §1 one-way-door qualifier says a wrong frozen shape **BLOCKS** rather than ships. Keep it that way. |

---

## 4 · The open forks — what a Settle must decide

Nine. Six are cheap-vs-robust and must go to an independent **robust-advocate** before the cheap
option is taken, per the project's own rule.

---

**Fork 1 · PT-1's shape: what does `migrate-corpus` say about a file that is not its subject?**
*Cheap-vs-robust: **yes**.*

- **(a) Silent exclusion.** Consult `is_unadopted_foreign`, drop foreign files from the candidate set,
  say nothing. *Cost:* cheapest, and it recreates a **false-green class this project has closed
  twice** — `0 blocked` reads as *nothing to do* over a corpus that has un-adopted files.
- **(b) Exclude from blocking, report as an advisory with the adoption route** — mirroring `validate`'s
  `unadopted-instance` verbatim so the two surfaces share one vocabulary. Exit status computed over
  the managed set alone. *Cost:* one more row type in `migrate-corpus`'s report; the two surfaces must
  be kept in sync, which the shared predicate does for free.
- **(c) Route repair only.** Leave it blocked, fix the printed route to name `ingest`/`migrate --as`.
  *Cost:* cheapest of all and **does not fix the classification** — the tool still claims a subject
  that is not its own, and the next surface to walk the store repeats the bug.

**Recommendation: (b).** It is the only option under which the two surfaces cannot disagree again.

---

**Fork 2 · F-1's predicate: what counts as "recorded a changelog entry"?**
*Cheap-vs-robust: **yes**.*

- **(a) Write-touch.** Any staged write to the changelog doc in this task suppresses the advisory.
  *Cost:* simplest and unambiguous; a task that only fixes a typo in an existing entry also suppresses
  it — arguably correct (the doc was tended and the tool cannot read intent), arguably a hole.
- **(b) Structural.** Item-count increase **or** any slot/field write under a release / change-group.
  *Cost:* more precise, more predicate surface to keep aligned with the schema as it evolves.
- **(c) Keep the count, fix only the routes.** *Cost:* leaves a **printed lie** standing — the advisory
  still contradicts the `promoted CHANGELOG.md` line printed beside it. Fails law 1.

**Recommendation: (a).** The gate's honest subject is *"did this task tend the changelog"*; it has
never been able to judge whether a change is user-facing, which is why its own route ends *"if it is
not user-facing, no action is needed."*

---

**Fork 3 · Does the changelog advisory join `task validate` — and if so, what about the rest?**
*Cheap-vs-robust: **yes**. This is the fork with the biggest hidden axis.*

- **(a) Surface it at `task validate`.** Makes the in-task route runnable and discharges M47
  Increment 4's contract (*"`task validate` previews what finalize gates on"*) over its advisory axis.
  *Cost:* **the axis must be enumerated first** — if other finalize-only findings exist, (a) on this
  one alone fixes an instance while M45's own contract demands the class. That enumeration is real work
  and may be a whole increment.
- **(b) Keep it finalize-only, drop the in-task option from the route.** *Cost:* cheap, honest, and
  leaves M47's preview contract stated more broadly than it holds.

**Recommendation: enumerate first, then decide.** If the finalize-only set is {this}, take (a) — it is
one call site. If it is larger, the Settle is choosing between an instance fix and a class fix with
its eyes open, which is exactly what M45 exists to prevent doing by accident.

---

**Fork 4 · The pre-1.0 one-way-door window — the wave's most consequential decision.**
*Cheap-vs-robust: **yes**, and it is the one where "cheap" may be right.*

Two ledger entries need a **frozen, manifest-governed methodology doctype** to change shape: entry 2
(a `planning-record` doctype — an 11th manifest entry) and entry 10 (`milestone-record`
`schema-version` 2→3 for a durable provisioning fact). **A schema bump plus corpus migration is
cheapest now and gets monotonically more expensive after 1.0.0**, because after the call there are
adopter corpora in the wild.

- **(a) Take neither.** Both triggers are recorded as unfired, and this trial produced no instance of
  either. *Cost:* any post-1.0 need pays migration cost **plus** adopter cost, and the window closes
  today.
- **(b) Take entry 10 only.** M47's substitutes close the fresh-clone case but not the local
  post-teardown residue. *Cost:* a frozen-doctype bump + migration + manifest re-pin for a residue with
  zero measured instances — and it makes the **next trial's headline subject the migration**, not the
  fixes.
- **(c) Take both.** *Cost:* (b)'s, doubled, on a wave already at size.

**Recommendation: (a), in writing, with the window cost stated as a known bound** — the demand-counter
rule has held for three waves and this trial gave it nothing new. But this must be **posed to the
human, not assumed**: "cheapest now" is a real argument and it expires at the 1.0.0 call.

---

**Fork 5 · `milestone finalize --dry-run`: build it, or make the existing partial preview findable?**
*Cheap-vs-robust: **yes**.*

- **(a) Build the flag**, mirroring `task finalize --dry-run`: forecast the manifest including the code
  fold from each worktree, commit nothing. *Cost:* new surface at the last moment before 1.0.0;
  forecasting a multi-worktree fold without performing it is real work; contributes to the
  latent-surface-sweep trigger count (§6).
- **(b) Don't build.** Fix `finalize --help` (already in scope as B2-3) and teach the workflow text
  that `milestone join` *"commits nothing"* and prints the merged doc set. *Cost:* the **code** half
  stays unpreviewable — and that is precisely the uncertainty that produced a `git reset --hard` on a
  live sub-task worktree.

**Recommendation: (a).** A measured destructive workaround by a careful worker is the strongest kind
of evidence this project accepts, and the shape is settled by the task-level twin — this is not new
design, it is a missing member of an existing pair.

---

**Fork 6 · Store-scope findings riding the task gate (B3b-3 + ledger entry 3).**
*Cheap-vs-robust: **yes**.*

- **(a) Scope-aware routing.** Store-scope rows print on `jigc validate`; on the task gate they print
  only when they name a file the task touches. *Cost:* the honest fix
  [findings-verification](findings-verification.md) §10 names — but it changes what a **gate** prints,
  and a worker could stop seeing a real store problem at the moment it matters.
- **(b) Print once per store state.** *Cost:* new state (what was printed, when) for a cosmetic.
- **(c) Make the code cascade-tunable**, following the `changelog-recording.gate-granted-unused`
  precedent. *Cost:* does nothing by default; hands the adopter a knob instead of a fix.
- **(d) Nothing.** *Cost:* correct, correctly routed, non-blocking, and noisy enough that a worker
  re-checked every firing to see whether it was new — the habituation failure mode the ledger has been
  tracking for two trials.

**Recommendation: (a), bounded to the task gate only** — `jigc validate` keeps saying everything.

---

**Fork 7 · Ledger entry 11 — does `finalize`'s landed teardown get `discard`'s guard?**
*Cheap-vs-robust: **yes**.*

- **(a) Build the refusal + `--force`**, reusing M48's fail-closed leftover classifier. *Cost:* a new
  blocking refusal and a new flag on the milestone commit boundary, shipped immediately before 1.0.0;
  it can block a legitimate finalize; it is a **second declared behaviour change** and §1's regression
  row exempts only *one* (§6).
- **(b) Keep M47's warn-only bound.** *Cost:* one of four destroying doors does not satisfy the rule
  the other three now satisfy, and it destroys unstaged/untracked sub-agent work at **exit 0 on the
  success path**. The codebase's own comments record the asymmetry.

**Recommendation: (a).** The M47 justification for deferring — *"only unstaged/untracked scratch is
lost"* — is the same reasoning that was found **false for `uninstall`** at the M47 completion audit,
and the fix there was taken. Consistency across the four doors is worth more before 1.0.0 than after.

---

**Fork 8 · D-1: a fence, or prose?**
*Cheap-vs-robust: **yes**.*

- **(a) Extend M48's stated-at fence with a fifth tier** — a step that solicits an author payload must
  name the schema read it stands for, over a derived owe-set. *Cost:* fence design plus fence fatigue;
  and the corrected evidence is weaker than M48's was (3 of 4 workers found `doc schema` unaided).
- **(b) Prose:** re-file the `AGENT.md` mention under the write intent (*"before authoring, read the
  doctype's shape"*), name `doc schema` in the authoring steps and in `SKILL.md`. *Cost:* cheap,
  unfenced, will drift — which is the exact argument M48 used to build the fence rather than fix the
  prose.
- **(c) Nothing.** *Cost:* honest under the corrected evidence, and leaves the **preload** cause
  (B1 ran `jigc setup` inside the measured session) untouched — a cause the protocol already
  anticipated.

**Recommendation: (b), plus one honest sentence in the record** that the correlation is n=1 and the
cause may be preload rather than discoverability. Do **not** scope this as a seventh discoverability
landing; [session-findings.md](session-findings.md) Correction 2 retracts that framing and the wave
must not re-import it.

---

**Fork 9 · When does the RC-1.0-gate conversion ledger close?**

- **(a) Before the build.** *Cost:* ~14 rows get cited, then re-cited when their fixes land — duplicate
  work, and every citation the wave then rewrites is invalidated.
- **(b) As the wave's closing increment.** Each fixed finding cites the test its fix shipped; each
  out-of-scope finding cites a standing suite or states `UNPINNED: <why>`. *Cost:* the ledger's
  evidence arrives last, so a build problem surfaces late.

**Recommendation: (b), with one carve-out** — the **out-of-scope** set (§3) gets its `pinned-by:` or
`UNPINNED:` written **now, before the build**. That is how this brief's own refusals become auditable
rather than assertions.

---

## 5 · What the following trial must test that this one could not

**1 · The `doc rename` / identity surface — reached by nothing.** [coverage.md](coverage.md) is
unambiguous: `doc rename` appears **zero** times across all four blind invocation logs *and* the
control; `write.identity-change` / `write.title-ignored` appear in **no** session's `finding_codes`;
M48's second-largest increment is carried entirely by tests and one operator rehearsal. Two probes
§5 arm 4 chartered were never run — `doc rename` on a **committed** identity (expect the refusal, not
exit 0) and a typo'd `jigc config get`.

**The instrument, not just the arm, must change.** The designed occasion was §3.2's mid-task
correction, and it **never fired in 4 of 4 sessions** — a correction queued at a turn boundary cannot
be delivered into an arc that never pauses. Options: an operator-scripted walk arm (reliable, buys no
discoverability signal), or a corpus seeded so the identity change is unavoidable — a doc whose title
the task's own intent forces to move. Prefer the second, backed by the first.

**2 · The carryover gate's re-stage-after-mint axis.** B1's plant was defused before the mint, so the
gate was never given its case, and the observation in §2 above is unverified. A designed arm:
stage → mint → unstage → re-stage → finalize.

**3 · `migrate-corpus` on a genuinely mixed corpus.** No trial has ever built one store holding
foreign + stale-managed + ahead-stamped files at once — which is the real adopter shape and the exact
state PT-1's fix reclassifies. Every prior migrate-corpus result was measured on a single-class corpus.

**4 · Whatever this wave adds.** Under the standing rule *the acceptance must reach what the wave
changed*: a `milestone finalize --dry-run` arm and a `finalize` dirty-worktree refusal arm if forks 5
and 7 are taken. The dirty-worktree refusal must be judged against §5 arm 1's fixed
protection-not-obstruction standard, like M48's three doors were.

**5 · `provision`'s mid-mutation failure (N2), if taken.** Not blind-reachable — no session can make
`git worktree add` fail partway. Operator-scripted with a poisoned path.

**6 · The guide artifact's *use*.** The protocol asked for it in writing and the trial did not answer:
zero mentions of the skill, the guide or `SKILL.md` across all four feedback reports. Report it as
**unmeasured**, not as a null, and build a channel for it (transcript grep or an invocation-log
proxy).

**7 · A regression arm that is not rc.10.** Detection currently rests on **one** walk arm. This wave
changes predicates on `migrate-corpus` and the changelog gate — both of which act on **existing
corpora** — so the next trial needs a continuation arm on a corpus authored under **rc.11**, not only
the rc.10 one.

**8 · A corpus that is not the observer's.** Every honest bound says the same thing: one template,
`--clean-prose`, no `node_modules`, 23 tests. If 1.0.0 is a claim about adopters, at least one arm
should run somewhere purpose-built for something else.

---

## 6 · Sequencing risk — what could invalidate the 1.0.0-gate trial's results

Ranked by how much re-running they force.

1. **A frozen-doctype schema bump (Fork 4) invalidates the most.** Every corpus every prior trial
   produced becomes un-migrated, and the **next trial's headline subject becomes the migration** rather
   than the fixes. This one decision changes what the following trial is *for*. It is the strongest
   argument for (a).
2. **PT-1's fix changes `migrate-corpus`'s subject set.** Walk arm 2 — the only rc.10 → rc.11
   continuation evidence, which reported *"0 migrated, 3 already current, 0 blocked"* — was measured
   under the old classification and **must be re-run**. So must project-alpha-4.0's ahead-stamp result and
   RC-pre-1.0's migrate-corpus hook-rejection finding, if either is still load-bearing.
3. **F-1's predicate change alters what `finalize` prints on every changelog-touching task** — the most
   common task shape in every trial. Anything this trial concluded about finalize's output (noise,
   route floor, the exit-code census) is re-opened for that path.
4. **Fork 6 changes what the task gate prints for *all* store-scope findings.** That is the same
   surface carrying two headline results: *"every block encountered carried a route"* and the
   17-non-zero-exits census. If taken, both need re-measuring.
5. **Fork 7 introduces a second declared behaviour change.** §1's regression row exempts exactly **one**
   (`provision` refusing a non-empty leftover). A second must be **written into the next protocol
   before it runs**, or the new refusal will read as a regression and BLOCK by the rule's own letter.
6. **The latent-surface-sweep trigger is a count** — ≥3 user-facing verbs or ≥3 composed workflows.
   `milestone finalize --dry-run` is a flag, not a verb, and the wave adds no workflow — so on the
   current scope the trigger **does not fire**. It fires if the Settle adds `doc search` or the
   evidence-rung command. Budget a whole increment if so; the record already notes the trigger as
   written fires on every wave M41–M47.
7. **Citation churn (Fork 9).** Any test cited as `pinned-by:` that this wave rewrites invalidates the
   citation. Closing the ledger last is the mitigation.
8. **The read-back measurement is not re-runnable as-is.** 4/4 VERB measured *instructed* compliance at
   N=4 with `bypassPermissions` as a declared directional confound. Changing which surfaces name which
   read verbs (Fork 8) changes the instrument. Keep the next trial's read-back arm identical to this
   one, or state plainly that the two numbers are not comparable — the same discipline this trial
   applied to the RC-pre-1.0 comparison.

---

## 7 · Size — honest

**As scoped above: roughly 10–12 increments** — the same size as M45 (11), M47 (11), M48 (12).
Two defect increments · one to three surface-batch increments · two to three capability increments
(dry-run, the finalize guard, provision rollback) · one discoverability increment · the ledger
disposition rewrite · the conversion ledger · flow 49 + goldens + fold-back. **Feasible as one wave.**

**It is feasible only because the ledger closes by *disposition*, not by build.** Three specific
decisions would break it:

- **Fork 4 taken either way but (a)** — a frozen-doctype bump drags a corpus migration, a manifest
  re-pin and a migration-shaped trial behind it. That is a wave.
- **`doc search` de-parked** (ledger entry 4). A whole capability with a real design, and it fires the
  latent-surface sweep on top.
- **Fork 3 answered (a) over a large finalize-only set** — a preview-contract sweep is its own
  increment at least, possibly two.

**If a split is forced**, the correct cut is **not** defects-then-capability. It is:

- **W1 (pre-1.0):** the two defects · the whole surface batch · **the `finalize` dirty-worktree guard
  (Fork 7)** · the ledger disposition rewrite · the conversion ledger.
- **W2 (post-1.0):** `milestone finalize --dry-run` · N2's rollback · whatever the ledger keeps.

The destroying-door guard belongs in W1 **even under a split**, because deferring it past the call
means shipping it later as a breaking change to an adopted binary — strictly worse than shipping it
now as the fourth member of a set adopters meet all at once.

---

## Recommendation

Build this as **M46**, claimed as *the disposition wave — a count is not a disposition*, and hold it to
that: the two named defects fixed at their predicates over enumerable axes (PT-1's is a shipped
discriminator with five call sites and a missing sixth; F-1's is one count-based predicate and two
routes that point at each other), the trial's nine surface findings taken as one batch because they
are all reversible prose and routes, three capability entries built because their shapes are already
settled by existing twins (`milestone finalize --dry-run`, the `finalize` teardown guard, provision's
rollback), the other nine ledger entries **retired or re-keyed to conditions that can fire on their
own — in writing, this wave, so the ledger is empty of counts when 1.0.0 is called** — and the
RC-1.0-gate conversion ledger closed as the final increment, since it is the human's own stated gate
and stands today at 2 of 17 rows. Take Fork 4 as **(a), take neither frozen-doctype bump**, and record
the closing window as a known bound rather than spending the wave on capability with no measured
demand; that single decision is what keeps this one wave instead of two, and it is the one I would
most want the human to overrule me on if they disagree, because it expires at the call. Everything
else here is reversible after 1.0.0, which is exactly why it can ship in one wave and why the two
irreversible-feeling questions — the schema window and the second declared behaviour change — are the
only ones this brief refuses to answer on its own.
