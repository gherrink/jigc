# Razor ledger — M46, the adjudicated IN/OUT set

The **guarded application** of M46's razor, run 2026-08-18 at HEAD `4f0b9e6` on `1.0.0-rc.11`. The
human's instruction was *"guard this with agents and combine findings"*; this is the combination.

**Instrument:** three independent appliers over disjoint slices — the correctness core · the 12
ledger entries + the trial surface batch · the doc contradictions under the bound — followed by an
**adversary over their combined output**, tasked with finding stretched admissions, wrongly-refused
items, and inter-applier conflicts. The orchestrator drove the one finding the appliers disagreed on.

**Inputs:** [scope-audit.md](scope-audit.md) (Scope) · [gap-findings.md](gap-findings.md)
(Detect gaps, §1 carries the razor as decided).

---

## 0 · The razor, as amended by its own guard

**Claim: every fix reconciles a contradiction the codebase already carries — a rule stated in one
place and violated in another.**

Test 1 (a rule is **stated** in a locked artifact) now carries a clause the adversary earned by
hitting all four failure modes in one pass:

> **Citation qualification.** A citation qualifies only if it is **normative and current**. It does
> **not** qualify if it is (i) a summary that forward-references its own fuller statement, (ii)
> notation the doc declares illustrative, (iii) a bound the doc declares *and schedules*, or (iv) a
> rationale another artifact has retracted. **The sweep is over every doc that touches the thing:**
> where a universal-quantifier sentence and an owning doc's carve-out disagree, **the carve-out
> governs** and the universal is a doc fix — and where a **declared exclusion rests on a predicate**,
> falsifying that predicate admits the item **at the predicate, never at the exclusion**.

That final sentence is load-bearing and is the single most valuable output of the guard — see §2.

**The adversary's verdict on the claim itself:** *"The claim is right; the 'three of six refused'
premise is false — an artifact of an incomplete citation sweep, not of the razor."* With qualified
citations, clause 1 admits **five of the six** correctness defects. **No second admitting clause is
recommended**; a "data-truth" clause would buy only N-6-adjacent cases and re-open the capability
tier the razor is refusing cleanly.

---

## 1 · The IN set

| Item | Qualified citation | The violation |
|---|---|---|
| **PT-1** | `validation.md:418` — *"it routes to `jigc ingest` / `jigc migrate <path> --as <doctype>` — **never** to `migrate-corpus`"* | `migrate-corpus` claims the file at exit 1 with an unrunnable route; `is_unadopted_foreign` has **zero** call sites in `migrate_corpus.rs`. **Axis is larger than recorded:** a *second* foreign file yields `migrate-corpus.deferred`, also unrunnable — size over `{placement, located} × {first-blocked, deferred}`, and plant **two** foreign files in acceptance or the deferred arm is never reached |
| **F-1** | `validation.md:568` — *"It **keys on the gate, never on the diff**"* | `task.rs:1880` compares repeatable-**item** counts. Driven: a task that promoted `CHANGELOG.md` in its own commit still drew *"this task recorded no changelog entry"* |
| **F-1 rider (S-4)** | law 1 + `surface-contract.md:36` | The printed route's in-task arm is dead **at the moment it is offered** — running it answers `no task <id>`, exit 1. `task.rs:1896`'s *"live on the `task validate` preview"* is the false premise; one call site, inside finalize |
| **N-1 (narration half)** | `team-ready-state.md:92` — *"before removing each worktree the teardown **names every path it discards**"*, and *"for the remainder **it narrates**"* | Driven: `milestone finalize` at exit 0 enumerated `notes.txt` while destroying `secrets.env` **and** `build/out.bin`, named nowhere — inside an enumeration claiming completeness. `milestone.rs:3620` declares the blindness itself |
| **N-2** | `finalize.md:153` (the retired whole-tree Sweep *"dropped every line of sub-agent code, the data-loss the redesign fixes"*) · `team-ready-state.md:92` (*"**the registered set cannot answer it**"*) · law 1 | **Driven by the orchestrator** after the appliers disagreed: `cp -R` → `milestone finalize` exit 0, two docs committed, the sub-agent's content **in no commit on any ref**, manifest printing `no worktree provisioned` over a directory that exists and holds it, record flipped terminal, no jigc route recovers it. Seam: `provisioned_worktrees` filtering the **registered** set |
| **N-3** — see §2c, admitted at the falsified predicate | **`CLAUDE.md:50`** — *"out-of-band edits are **detected and routed** … conflicts blocked and routed to a human, **never silently merged**"* (a locked architectural invariant) | 6 concurrent writers, 5/5 trials lost 3–5 baselines, **every process exit 0** printing *"dropped its file-state baseline"*. And a lost baseline **silently disables `reconciliation.conflict-block`**: with it absent, an uncommitted human edit to a committed managed doc is destroyed at exit 0, on zero refs |
| **N-4 / entry 7 (route repair)** | `finding.rs:262`'s rationale — *"**no CLI verb repairs a hand-broken byte**"* — **falsified at HEAD, but only on one arm** | See the two-arm result below. A shipped verb chain *does* repair it, and **no surface names it** — but it is reachable only from a fresh clone. Cleanest citation: `reconciliation.conflict-block`'s route commands *"revert the external edit on disk"* — a routed, **non-exempt** blocking finding ordering the act `adapter.rs:1136` forbids |
| **N-5 (route + diagnosis only)** | law 1 + `surface-contract.md:36` + the pack's own designated recovery at `locate-from-spec.yaml:26-29` | Reproduced on **three frameworks by three parties** (PHP/Pest, TypeScript/vitest, node:test): the message asserts a symbol *"is absent from"* a file whose **indexed blob** contains it; all three offered repairs disproved. The working repair is named by the pack step and not by the finding |
| **N-5 sibling** | same rule | `author-migration-spec.yaml:32-36` solicits `<path>#<test-fn>` and says *"carry over verbatim"* with no closure caveat — M45's rider landed in **one of two** soliciting steps |
| **F-I** | `corpus-migration.md:131`/`:264`, `doctype-authoring.md:35`/`:41` (*"the deterministic value is spliced into every item"*) + `migrate_corpus.rs:632-638` (the code's own *"would be a lie"*) | `transform.rs:362-365` / `:431-434` read `decl.default` **only**. No `set:` kind is migration-time-deterministic on its own; the route is unfollowable **even when followed exactly** |
| **F-I rider** | same | `migrate-corpus.prose-needed` (`migrate_corpus.rs:864`) asserts *"mints a new **required** prose slot"* for **any** halted doc — the shared rendering half of PT-1 **and** F-I |
| **B1-1** | `surface-contract.md:136` — *"A statement about a surface is **quantified over what that surface actually serves** … a law-1 lie **even when the behaviour underneath is right**. The repair is **scope**"* | Driven: finalize prints git-vocabulary *"left-out (… `git add` to include)"* then *"your **staged changes are still staged**"* while `git diff --cached` is empty. One block, two meanings of "staged" |
| **B2-1** | `surface-contract.md:36` | Under `quick-fix` (`allows-create: []`) the refusal offers `jigc doc create <type>`; running it → `create.gate-blocked … allowed doctypes: []` |
| **B2-2 (route half)** | same + law 1 | `jigc task discard <sub>` exits 0 while the **committed** record still reads `"status":"active"` and `list-tasks` still lists it. *Rider:* the fix must **revise** `start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal`, which pins the current text |
| **B2-3** | law 1 + `help_truth.rs` | The contradiction is **inside one help output**: `about` says *"doc bodies … and commit them"*; `--carry-staged`, three lines below, says *"the aggregate commit is built from the sub-task worktrees"* |
| **B3b-1** | `surface-contract.md:136` second clause — name the scope *"**and name what the members outside it do instead**"* | The router names its scope and stops |
| **B3b-2** | law 1 | **Cause misattributed in the brief.** Not the `bind` ack: `locate-from-spec.yaml:43-48` prints *"Read your write back … **the write you just made**"* **unconditionally**, while `:34-36` sanctions leaving the write off. Driven both arms: with it exit 0, without it `store.not-staged` exit 1 |
| **S-2** | law 2 — *"Every affordance that is the **designated recovery** … is named by the surfaces that produce that state"* | `cli.rs:317` already records the answer (*"`jigc start --explain` is the resolution trace"*); `jigc describe adr` gives a bare clap exit 2. The brief's read-intent framing **fails** — that axis is scoped to parent nodes and `describe` is a leaf |
| **Doc riders** (10, all ≤ a paragraph) | per [gap-findings.md](gap-findings.md) §2 slice C | C1+C3/S4 → PT-1 · C2+D5 → F-1/Fork 3 · C4 → F-F · D1+S14 → Fork 4 · D2 → F-D · D3 → Fork 7. **Zero standalone doc increments** |
| **S13** | `doctype-authoring.md:1-3` — *"**This is that list.**"* | Five waves stale; names none of the M43–M48 pack-load fences, each a hard `bail!`. **It already produced a wrong estimate inside this wave's own planning.** On entry 2's critical path if entry 2 is IN |

---

## 2 · The re-cut that proves the claim

The adversary's sharpest single move, and the reason the citation clause is worth its words:

`team-ready-state.md:92` **excludes** `milestone finalize` from the refusal rule by name —
*"The landed-finalize teardown **stays outside the rule by satisfying it**: the commit landed first,
so the set it removes is already proved disposable, and **for the remainder it narrates** (visible,
not prevented; **M46**)."*

Read as a boundary, that refuses Fork 7: a refusal at the fourth door is new surface the doc
schedules. **But the exclusion rests on a predicate — *"for the remainder it narrates"* — and B2
falsifies it.** So the item is admitted **at the predicate**:

> **Fork 7 is re-cut from *"build a blocking refusal + `--force` at the fourth door"* (refused: new
> surface, and F-D's measured guard-killer — `--ignored` on a *refusal* fires on the ordinary fan-out
> success path, training `--force` into reflex) to *"make the narration true over the `--ignored`
> axis"* (admitted, cheaper, and it restores the exclusion the doc already relies on).**

Same defect, same evidence, a fix that is smaller, safer, and leaves the design of record intact.

---

### 2b · N-4 / entry 7 — the repair path is real, and reachable from one place only

**Orchestrator-driven, both arms, after the adversary's headline reported only the working one.**
A committed spec built through the tool, then hand-broken with `### Rationale for the cap` inside item
prose and committed (the pre-guard shape):

```console
$ # ARM 1 — the repo where the corruption happened (a file-state baseline exists)
$ jigc migrate docs/specs/rate-limiter.md --as spec        → EXIT=0, task minted
$ jigc doc author spec --from-file payload.yaml --task <id> → EXIT=0
$ jigc task finalize <id>                                   → EXIT=3
blocking · reconciliation.conflict-block — conflict on `docs/specs/rate-limiter.md`: an external edit
  and this task's staged writes both changed it
  route: `jigc task discard <id>` …, or **revert the external edit on disk** to keep them
$ jigc task finalize <id> --approve                         → EXIT=3   (identical)

$ # ARM 2 — a fresh clone (`.jigc/state/` is gitignored, so no baseline exists by definition)
$ jigc migrate … --as spec → EXIT=0 · jigc doc author … → EXIT=0
$ jigc task finalize <id>            → EXIT=4  "migration review required — nothing committed"
$ jigc task finalize <id> --approve  → EXIT=0
$ jigc validate                      → EXIT=0  "no findings — the committed store validates clean"
```

**So the disposition is sharper than either input said.** My Scope audit's flat *"no verb can repair
it"* is **wrong** (arm 2 repairs it). The adversary's *"a shipped verb repairs it"* is **incomplete**
(arm 1, the ordinary case — you fix the corpus where it broke — dies at exit 3 and routes you to the
forbidden hand edit).

**What this changes for the fix:** it is not "name the working path in the four refusals". The route
must name a path that is *reachable from where the user is standing*, and on arm 1 that means either
the `conflict-block` route naming the migrate entry, or a sanctioned way to drop the stale baseline
first. Ledger entry 7 **RETIRES** (the repair verb is not what is missing); what ships is the route,
and it has two arms, not one.

---

### 2c · N-3 — admitted at the falsified predicate, and DECIDED (the human, 2026-08-18)

**The item was refused at first pass** on the ground that `decisions-pending.md:326` defers lost-update
semantics by that exact description with a named trigger, and *a fired trigger is an expiry, not a
contradiction*. The independent robust-advocate overturned that, and the razor's **own guard clause**
is what overturns it:

Entry 9's M48 disposition (`decisions-pending.md:127`) rests on two predicates — **"untouched"** and
**"contention, not corruption."** The first was falsified 2026-08-18. The second the advocate
falsified by experiment: with the baseline present, an out-of-band worktree edit draws
`reconciliation.conflict-block` at exit 3; with the baseline **absent** — the residue a lost write
leaves — the same state finalizes at **exit 0**, and the human's prose is gone from the worktree, gone
from the commit, and on zero refs. **That is not contention.** So the item is admitted *at the
predicate, never at the exclusion* — the identical move that re-cut Fork 7 at §2. Declining to apply
it here while applying it there would be applying the razor selectively.

**The qualified citation is not the deferral ledger.** It is `CLAUDE.md:50`, a locked architectural
invariant, violated at HEAD, driven. What is admitted is therefore **not "a locking primitive"** but:
*the reconciliation state machine must not be silently disabled by a concurrent write.*

**One advocate claim was tested and does not hold.** Its topology argument —
*"two sub-agents finalizing concurrently lose one set of those adds by construction"* — was driven by
the orchestrator and **fails**: git's `index.lock` serialises the same-repo case cleanly
(`alpha exit=3 finalize.stage-failed`, promotions rolled back, retry lands, both baselines recorded),
and in a real fan-out per-sub-task finalize is refused by design. What survives is the **removal**
direction (5/5 driven; `unmanage`/`ingest`/`relocate`/`rename` take no `index.lock`), the arm-A/arm-B
result *however the baseline was lost*, and `jigc validate`'s own `edges.json` write (`index.rs:238`)
— a read-looking verb that is a writer to shared state.

**DECIDED — the third shape: base-relative three-way merge behind a save-scoped lock.**

Neither posed arm was taken. The cheap arm **as specified is not buildable**: `state::persist(path,
bytes)` receives opaque serialized bytes with no delta and no base, so it cannot re-apply anything; it
degrades to detect-and-error, which at `advance_file_state` fires *after* the commit lands and thus
**relocates** the false ack to a post-commit seam with no re-run. The robust arm as posed is
over-built — its 3–4 increments are bought almost entirely by a coarse lock whose deadlock constraint
(the pre-commit hook runs a nested `jigc validate`; `File::lock` is per-fd advisory) disappears once
the delta is *derived* rather than *threaded*.

The shape: stash the loaded base on `FileStateRecord::load`; on `save`, take the lock **for the
duration of `save` only**, re-read, three-way merge per key (`base` = loaded, `ours` = self,
`theirs` = on-disk), persist, release. Verified sound rather than assumed: the record is a bare
`BTreeMap<String, String>` and **every** production mutation is a per-key `record()` / `forget()` —
no whole-map substitution on any production path — so the merge is total and lossless. `EdgeIndex`
takes the **lock only**; it is stamp-rebuildable by design, so merge semantics there would be
gold-plating. A hand-constructed record (`base: None`, ~15 test seed sites) degrades to a safe union,
so no test churn.

**Rides with it, and is owed regardless of arm** under M46's own reconciliation claim: the
`storage.md` *Concurrent writers* design home (S6 — the doc says nothing today), and the
**`VISION.md:55`/`:168` + `CLAUDE.md:53` carve-out**, three locked artifacts stating an isolation that
`team-ready-state.md:172` already contradicts.

**Named bound, not buried:** `tasks.json` (S1) is written with plain `std::fs::write` at four sites in
`milestone.rs`, is named *"shared by all N sub-agents"*, and is a **registry, not a flat map** — the
merge does **not** transfer to it unexamined. It must be explicitly scoped in or excluded with a
reason; losing a sub-task registration is the N-2 class.

---

## 3 · The OUT set — the cost of the boundary, stated

Real, several cheap, none with a qualified rule behind it. Any of these must ride as a **declared
exception to the claim**, never smuggled in.

| Refused | Why it fails the test |
|---|---|
| **N-6** — text drops `location.address` | **No rule.** The near-miss (`worked-examples.md:813`) is killed by `worked-examples.md:4`: *"Notation is **illustrative**."* M48's parity fence is one-directional **by construction**. The human already declared this the boundary's stated cost |
| **N-1's refusal half** | `team-ready-state.md:92` blesses `git status --porcelain` for `OwnWorktree` **by name**. A policy to make. Superseded anyway by §2's re-cut |
| **N-3's locking primitive** | `decisions-pending.md:326` defers it *by that exact description with a named trigger*. **A fired trigger is an expiry, not a contradiction** |
| **N-5's blocking verdict** | `validation.md:204` sanctions non-resolution of a string-only name |
| **Entry 3** — precision fix / ack-ledger / `off` severity | The message is honest (it states the comparison it made); `validation.md:198` names the false-positive rate a *"conscious cost, recorded"*; the sub-defect sits inside a **declared bound** at `target_surface.rs:434` |
| **Entry 4** — search verb | Refused on better grounds than "capability": AGENT.md states an `unregistered` row is *"readable directly until adopted"*, and a conformant managed doc reads clean. **There is no unreadable-managed-doc contradiction to hang search on** |
| **Entry 2** — `planning-record` doctype | Capability. (One textual sliver — 13 vs 12 gate rows across two locked docs — is IN-with-narrowing at most) |
| **Entry 11c** — landed-teardown refusal | Deferred **to M46 by name**, but new surface. Re-cut at §2 |
| **B2-4** — `milestone finalize --dry-run` | `finalize.md:248` records `--dry-run` resolved for the **task** verb; no milestone twin is promised |
| **B3b-3 / Fork 6** | **The razor inverts:** suppression would *create* the contradiction M48 removed. A cap or collapse preserving the per-file answer is a preference |
| **Entries 1, 6, 8, 10** · **B2-2's `next:` half** · **the `bind` ack** | Capabilities or preferences; no qualified rule |
| **S12** — publishing floor | Already **DECIDED** post-v1 (`decisions-pending.md:209`). Discharge by citation |
| **Entry 12** — `hook_file` | **Discharged, verified.** Its declared bound is **stale and should be struck, not carried** |

---

## 4 · Adjudicated conflicts

- **`validation.md:39` vs `:66` — slice B wins; my own framing was wrong.** `:39` does not stand 27
  lines from its contradiction, it **cites** it: *"always present (**the route floor below** — never
  `null`)"*. General rule then stated exceptions, inside one section. *"27 lines apart"* was an
  artifact of counting lines instead of reading the parenthetical. It lands **only if** F-F narrows
  `is_route_exempt`, in which case both sentences move together.
- **N-3 — both appliers right.** The expiry reading and the law-1 reading coexist; the minimum honest
  ack **is** detection at the write seam, which is the cheap CAS arm. The ledger row buys the seam
  obligation, not the primitive — and should say so.
- **`storage.md:121` reads as a false universal.** A three-member parenthetical governing "every
  door", while the owning doc excludes the fourth by name. An applier grepping `storage.md` admits
  entry 11; one grepping `team-ready-state.md` refuses it. **Needs the exclusion or a pointer either
  way.**
- **Entry 7's verdict was unstable three ways** — scope-audit said REFUTED, the trigger's literal
  words say IN, the binary says the need is already served by an unnamed verb. **Resolved: RETIRE the
  entry, ship the route repair.** The record's enumeration of doors is short by two.

---

## 5 · The Settle — decisions taken (the human, 2026-08-18)

All four are **owed to `DECISIONS.md`**, dated, when the Settle closes.

**D1 · The claim and razor.** M46 is **the reconciliation wave** — *every fix reconciles a
contradiction the codebase already carries.* Three-leg test + the citation-qualification clause (§0).
Guarded by three appliers over disjoint slices and an adversary over their combined output. **No
second admitting clause** — the "refuses three of six" premise proved false under qualified citation.

**D2 · N-3 takes the third shape** — base-relative three-way merge behind a save-scoped lock (§2c).
Neither posed arm: the cheap arm is not buildable as specified, the robust arm is over-built by a
coarse-lock design the third shape does not need. ~1 increment. `tasks.json` must be explicitly
scoped in or excluded with a reason.

**D3 · ~~The `validation.md:433` expiry is AFFIRMED~~ — WITHDRAWN the same day, on a falsified
premise.** The affirmation rested on the bound's own reasoning that the unstamped-managed population
is *"finite pre-1.0 and empty at the 1.0 pin"*. The independent pre-decompose review falsified it and
the orchestrator confirmed it by driving the binary:

```console
$ grep -c schema-version docs/decisions/adopted-me.md   → 0     # conformant foreign ADR, no stamp
$ jigc ingest                                           → adopted — indexed + baselined, no file moved
$ head -4 docs/decisions/adopted-me.md                  → byte-identical; still no stamp
$ jigc doc list | grep adopted-me                       → adr:adopted-me  …  managed
$ jigc migrate-corpus                                   → 1 migrated       # the v0 arm is LIVE
```

**`jigc ingest` is register-only** — it indexes and baselines and *"never moves or rewrites the
file"* — so the product's **own primary brownfield front door refills the population continuously**.
Under *"stamp-absent means foreign, full stop"*, that doc reads `unregistered` at `doc list` while
`ingest` calls it adopted: **the wave would create a door-dependent contradiction — the class M48
removed and the one this razor exists to refuse.** Both readings also regress shipped behaviour
(deleting `classify_provenance`'s parse-against-prior arm kills two engine tests naming it
load-bearing and orphans the `priors` threading through four doors; `migrate-corpus`'s v0 arm is the
whole subject of `methodology_corpus_stamp.rs`).

**PT-1 needs none of it** and reverts to the straightforward shape the affirmation had forbidden:
*call the shipped 5-site `is_unadopted_foreign` discriminator from `migrate_corpus.rs` before the
fold* — a filter in front of a live arm.

**What survives:** the *finding* that `validation.md:433` schedules an expiry at this wave's gate and
no planning input carried it. The bound stands as written and owes an amendment naming `ingest`'s
register-only adopt as a **continuing** source of unstamped-managed docs.

**Recorded as a basis-has-changed withdrawal, not an override** — and the honest note: the premise was
checkable in one command, and the check was not run before the recommendation was made.

**D4 · Fork 4 closes as TAKE NEITHER — on evidence, not on a count.** The expiring decision, deferred
to this Settle with both arms open, is closed on three independent grounds:

1. **Entry 2 was never on the window.** A new doctype moves no hash, needs no snapshot, and returns
   `0 migrated, 0 already current, 0 blocked` at exit 0 — exactly as cheap in 1.1.
2. **Entry 10's residue is empty, driven both arms today.** The `cp -R` / `mv` arms fall to N-2's
   path-subject fix at **zero stored state**; the fresh-clone arm **already refuses** at exit 3 with a
   route that names the cause — *"a fresh clone has none"*. Nothing is left for a durable
   "was provisioned" fact to serve.
3. **M42 already refused the premise** (`decisions-pending.md:207`): *"if the window is real, the
   migration machine has failed its declared purpose"*, citing `corpus-migration.md:3`'s promise that
   an adopter's lock-in cost is **bounded**.

*Recorded for the avoidance of doubt:* the bump is **buildable** — a required, defaulted,
deterministic field migrates byte-stable (2 engine tests pass at HEAD) and stays machine-maintained,
since `is_machine_maintained_absolute` keys on the `set:` kind, not on `default:`. It is refused
because it has **nothing to serve**, not because it is impossible. This is the disposition wave's own
claim discharged on its hardest entry.

**Owed to `decisions-pending.md`:** every OUT-set item re-keyed to a condition that can fire on its
own, or retired as dead — no entry may leave M46 deferred on a count. The two with the strongest
measured evidence behind them are `milestone finalize --dry-run` (a measured destructive workaround)
and a cap/collapse on the `unadopted-instance` volume (1:1, uncapped, no mute) — both refused here,
both owed an honest condition.

---

## 6 · Bounds

- One sha, one binary. Everything marked driven was exercised; code-seam and doc reads are stated as
  such in the source probe outputs.
- **N-2 is now orchestrator-driven** — the appliers disagreed, and both prior documents' "unverified"
  declarations can be struck.
- **F-F's narrowing does not resolve F-F.** `is_route_exempt(code: &str)` is code-only **by declared
  design** (*"never a census of call sites"*), while the contradiction is about **managed vs
  foreign** — a property of the *document*. Three arms, not two. The route-owing ceiling if it
  narrows is **13**, all produced in `parse.rs`, one route per code.
- N-3 priced one seam of ~9; the other `load → mutate → save` sites were enumerated by reading.
- N-1's four-door claim rests on two doors driven; `uninstall` and `provision` were not re-driven.
- **Nothing here is settled.** This is the razor's output; the Settle is the human's.
