# Gap findings — M46, the Detect-gaps pass

The **Detect gaps** phase of the [milestone-planning workflow](../../../implementation/milestone-planning-workflow.md)
for M46, run 2026-08-18 at HEAD `b06bf72` on `1.0.0-rc.11`. Four adversarial probes — decisions,
docs, doctypes, capabilities — each carrying the three cross-cutting hunts (cheap-vs-robust against
the vision's committed trajectory · foreclosed-by-doc · prior-art-reconciled).

Input: [scope-audit.md](scope-audit.md). **The gap pass corrected that document five times**, four of
them material; every correction is folded back inline there, marked, with its evidence. This is the
phase working as designed, and it is the reason the full four-probe pass was run rather than skipped.

**Nothing here is settled.** The Settle that follows is the human's gate; this is its agenda.

---

## 0 · The cross-cutting judgment

Three of the six new defects are larger than the Scope audit priced, and **all three for the same
reason: the fix's subject is a rule the codebase states in one place and violates in another.**

| Defect | The contradiction |
|---|---|
| **N-3** | ~~`.jigc` state is a single-writer model in the code and a five-writer model in `team-ready-state.md:172`~~ |
| **N-1** | ~~`probe_leftover`'s doc comment forbids exactly what `dirty_worktrees` does — in the same file~~ |
| **N-4** | ~~`is_route_exempt`'s rationale sanctions the hand edit that *"the LLM writes only through the CLI"* forbids~~ |

> **⚠ CITATION HAZARD — this table is struck. Two of its three rows do not survive their own sources,
> and the razor decision below cites this paragraph as its justification, so anything admitted off it
> inherits the defect.** Caught by the razor adversary, 2026-08-18.
>
> - **N-1's row is retracted by its own input.** [scope-audit.md](scope-audit.md) §N-1 already carries
>   the correction: the comment is *scoped to the `Unverifiable`/`NoOwnLinkage` verdicts and honored
>   there*, so this is *"a policy **never made** for the `OwnWorktree` verdict"* — a decision to make,
>   not a contradiction to fix.
> - **N-3's row is not a contradiction.** `team-ready-state.md:172` states the shared topology and
>   `:174`, in the next sentence, **schedules the gap** (*"Lost-update semantics defer to the
>   capability wave with a real trigger"*). A doc that states a topology and schedules its gap in
>   consecutive lines does not contradict itself.
> - **N-4's row understates it.** The rationale is not merely in tension with an invariant — it is
>   **factually false at HEAD**: a shipped verb chain does repair the "unrepairable" doc
>   ([razor-ledger.md](razor-ledger.md) → B1).
>
> **The claim survives** — the adversary's sweep found *better* citations for five of the six
> correctness defects, not fewer. What fails here is citation quality, which is why the razor's test 1
> now carries a **citation-qualification** clause. The adjudicated ledger, not this table, is the
> record: [razor-ledger.md](razor-ledger.md).

---

## 1 · The razor — decide first, or nothing below can halt

`implementation/decisions-pending.md:83` records why, in the project's own words:

> *"M47's razor is what let its planners **refuse** things and halt twice on false premises, and a
> wave scoped as 'everything that improves quality' can produce no refusal and therefore no halt."*

M46 today carries: six new defects · PT-1 + F-1 · a 12-row ledger disposition · a ~12-item surface
batch · three capability items · the conversion ledger — and **no stated boundary that excludes
anything**. Prior waves each had one (M47: *"changes what surfaces say, never what surfaces exist"*;
M48: *"generous scope under a razor that can still refuse"*).

**Owed at the Settle:** the claim and the razor, in the M47/M48 form — *in scope if X, out if Y* —
**before decomposition**, so a build halt has a test to be adjudicated against.

### DECIDED (the human, 2026-08-18) — M46 is **the reconciliation wave**

> **Claim: every fix reconciles a contradiction the codebase already carries — a rule stated in one
> place and violated in another.**

Chosen because it is the shape the evidence actually took: three of the six new defects, the
migration dead-end class, `is_route_exempt`, and the `doctype-authoring` matrix all have that
structure (§0).

**The razor, as a test. An item is IN only if all three hold:**

1. A rule is **stated** in a locked artifact — a `design/` part-doc, a pinned contract, an
   architectural invariant (CLAUDE.md / VISION.md), a code-side fence, or a doc comment that declares
   policy — quotable with `file:line`.
2. Shipped behaviour at HEAD **violates** it, quotable with `file:line` or reproducible.
3. The violation is **demonstrable by driving the binary**, or by a code-seam read declared as such.

**An item is OUT if:** it is a missing capability (nothing is violated because no rule covers it) · it
is a preference or improvement with no stated rule behind it · the rule must be **invented or
stretched** to admit it.

**The bound on doc reconciliation** — the elastic risk this razor carries, fenced rather than left to
judgment: *a doc is IN only when the contradiction would mislead **this wave's own** build or
acceptance.* Drift unrelated to the wave's fixes is **recorded, not fixed**.

**What the razor is expected to refuse** (the cost of the boundary, stated up front so it is visible
rather than discovered): `doc search` · `milestone finalize --dry-run` · the acknowledged-findings
ledger · the `off` severity member · the symbol-mention sweep — all missing capabilities, not
contradictions. **N-6 is refused too** until a rule is stated, which matches §3's finding that it
violates nothing today.

**The razor is guarded, not self-applied** (the human's instruction, 2026-08-18): three independent
appliers over disjoint slices — the correctness core · the ledger + surface batch · the doc
contradictions under the bound — followed by an **adversary over their combined output**, whose job
is to find items admitted by stretching "contradiction" and items wrongly refused. The verdict ledger
is [razor-ledger.md](razor-ledger.md).

**Owed to `DECISIONS.md`** when the Settle closes: this decision, dated, with the claim and the test.

---

## 2 · Blocking forks, ranked

### F-A · `validation.md:433`'s declared expiry fires **at this wave's gate**, and no planning input carries it

> *"an unstamped **managed** doc exists only in a corpus that has never run that migration — a set
> that is finite pre-1.0 and **empty at the 1.0 pin**, after which **stamp-absent means foreign,
> full stop**. This is a **declared bound with an expiry**, on the record, **revisited at the 1.0
> call**."*

The 1.0.0 call is the next act after this wave and its trial. Named in neither
[decisions-pending.md](../../../implementation/decisions-pending.md) → the capability wave, nor the
[scope brief](../RC-1.0-gate/next-wave-scope.md), nor [scope-audit.md](scope-audit.md).

**It reshapes PT-1.** Under that rule `migrate-corpus`'s v0 arm (`corpus-migration.md:90`) becomes a
**pre-1.0-only** code path for foreign files *by rule*, not by adding a predicate — so the honest fix
is to retire or gate it, which is none of Fork 1's three arms. **One-way-door tell fires:** stamp
semantics are a stored-format contract over adopter corpora.

**Recommendation:** take this first. Everything in PT-1 is downstream of it.

---

### F-B · Fork 4's window premise is contradicted by a locked decision

`decisions-pending.md:207` (M42, 2026-07-13) says the *"last cheap window before adopter corpora
multiply the migration cost"* framing **self-refutes**:

> *"if the window is real, the migration machine has failed its declared purpose"* — citing
> `corpus-migration.md:3`: *"so the lock-in cost a productive adopter inherits is **bounded** rather
> than open-ended."*

The brief re-imported that premise without quoting the refusal.

**Re-pose as one question:** *do we believe the migration machine bounds the cost (M42) or not?*
- **If yes** → Fork 4 collapses to "no", and entry 2 was never on the window anyway
  ([scope-audit.md](scope-audit.md) §4).
- **If no** → the decision is a **sweep over all sixteen frozen entries**, not two. M42's own
  precedent scoped it as a sweep (*"the sweep found no third such hole"*) and that sweep predates
  M37/M38/M39/M40's doctypes and the ten-schema methodology manifest. Nobody has re-run it. The
  deferred **`spec` criterion-status / open-decisions** shapes are inside the window and larger than
  either ledger entry.

**Also owed either way:** `implementation/doctype-authoring.md:35/41` reads ✅ for a `set:`-bearing
field, but `transform.rs:430-434` and `:359-365` read **`decl.default` only** — reproduced
end-to-end. `doctype-authoring.md:23`'s own rule is *"a bump without a kind strands the corpus.
**Build the kind first, then bump.**"* That matrix cell is what would have told the author, and it
says ✅. **Correct it in this wave regardless of Fork 4's outcome.**

---

### F-C · N-3 — one primitive, but ~9 seams, three refactors, and a deadlock constraint

**The one-or-seven verdict: ONE primitive. It does not split the wave.** Both `FileStateRecord::save`
(`file_state.rs:109`) and `EdgeIndex::save` (`index.rs:101`) funnel through the same
`state::persist` → `write_atomic` (`state.rs:387,397`). `std::fs::File::lock/try_lock` is std-stable
at 1.95 — no new crate, reachable from `engine`. The primitive is ~40 lines.

**But the audit's price was off by ~9×, and the obvious design self-deadlocks.**

- **~9 mutating seams, not 6**, plus five opportunistic writers. Three carry a *whole loaded record*
  across seconds of git/FS work: `ingest` (whole-corpus walk), `advance_file_state`
  (`task.rs:3192`, spans N `git show` subprocesses), `migrate_corpus` (walk + self-commit). A narrow
  critical section requires refactoring each so what crosses the git call is the **delta** — all
  already commutative (`record.record`, `record.forget`, `index.drop_doc`) — not the record.
- **A coarse lock deadlocks against jigc's own hook.** Verified live: `git commit` inside
  `jigc task finalize` runs the installed pre-commit hook, which runs a **nested `jigc validate`
  process**. `File::lock` is per-fd advisory — a second handle *in the same process* already fails
  `try_lock`. Any lock design owes an explicit re-entrancy carve-out.
- **`unmanage`/`ingest` save two files non-atomically** (`unmanage.rs:77` then `:80`). A lock fixes
  interleaving, not a torn pair — a second decision.

**Fork.** *Cheap:* lock-free CAS at the write seam — re-read, verify the hash is unchanged, re-apply
the delta. No lock file, no deadlock, no refactor — but it **narrows** the window rather than closing
it, so a wave claiming *"a fix is complete over its class's axis"* would ship a probability reduction
under a correctness claim. *Robust:* std flock + re-read-under-lock + delta re-apply + the three
refactors + the re-entrancy carve-out. **Long-run cost of cheap:** the divergence stays possible
under the topology the product's own fan-out primitive creates, and it does not get cheaper later —
the three wide seams' shape is what makes it expensive, and that shape does not improve.

**Nothing forbids a lock file.** `decisions-pending.md:326` is a deferral, not a prohibition, and its
stated basis is already corrected in writing at `team-ready-state.md:174`.

**Reconciliation owed:** `VISION.md:55` and `:168` still state *"each writer gets an isolated working
area keyed by task ID"* with no carve-out, and CLAUDE.md restates it as an architectural invariant.

---

### F-D · N-1 — the `--ignored` false positive is the *normal* case, not the edge case

Measured in a real provisioned worktree holding `target/`, `node_modules/`, `build.log`,
`secrets.env`:

```console
git status --porcelain                    → 1 entry    (secrets.env)
git status --porcelain --ignored          → 4 entries  (+ build.log, node_modules/, target/)
git status --porcelain --ignored -uall    → 43 entries
```

`probe_leftover` → `dirty_worktrees` uses the 1-entry form; `discarded_work` uses `-uall` and would
become the **43-entry** form. And `team-ready-state.md:167` says a worktree arrives tracked-only
*"while the sub-task walk **tells the agent to build the code and run the tests**"* — so **a worktree
that did its job holds build output.** A refusal on `--ignored` fires on the success path of the
ordinary fan-out, and `--force` becomes the routine answer, which is how a guard dies. The M48 doors
were judged against a **protection-not-obstruction** standard; the cheap fix fails it.

**There is no mechanical discriminator** between "ignored build artifact, disposable" and "ignored
secret, irreplaceable". That is a design decision M46 must settle and nothing in the record prices.

**The robust split:** *narration* takes ignored files (a destroyer must name everything it destroys —
`milestone finalize` today destroys `target/`/`node_modules/`/`build.log` **unmentioned**, which is a
law-1 problem in its own right, separate from Fork 7's refusal); *refusal* needs a stated policy for
build output, with `--force` as the single consent. **`milestone finalize --help` has no `--force`
today.**

**Rider:** `DESTROYING_DOORS` (`milestone.rs:1817`) has **three** members and `storage.md:121`
enumerates the same three by name. Adding `finalize` widens the array the M48 acceptance already
iterates — the new member joins the axis test for free — but revises a locked sentence in the same
motion.

**Also:** the two verdicts need separating before pricing. The `provision`-on-`cp -R` loss sits in
`NoOwnLinkage`; the live-worktree loss in `OwnWorktree`. They may take different answers, and the
"four doors, one flag" framing merges them.

---

### F-E · Fork 3 is foreclosed by a locked doc **and** blocked by a fail-fast planner

`design/finalize.md:41`, verbatim:

> *"It does **not** preview the phases around it: the preflight above (the base pin) and phases 3–6
> below each adjudicate their own failures — the empty-commit guard, `finalize.render-io`,
> `promote-clobber`, `stage-failed`, the untracked `owner-artifact` cause, and the commit hook's
> rejection. That is why a clean `task validate` says nothing this side of the commit blocks it, not
> this will commit."*

**Four of the six Fork-3 candidates are named there as deliberately not previewed.** The rationale is
real — a preview must not promise *"this will commit"* — and M47's carve-out honours it.

Three further constraints, none of them in the record:

- **`plan_finalize` is fail-fast** (`finalize.rs:186-270`, every gate `return Err(vec![…])`). A
  preview built on it shows **only the first** failure, and `base-mismatch` is checked at phase 1, so
  it masks every other member. A real preview means restructuring to collect-all, or writing a second
  path — which `finalize.md:41` and four code sites forbid **by name** (*"no private check path"*).
- **No code-side classification of staging-dependence exists.** `preview_gates` hand-picks its two
  members; findings carry no such bit. The wave's acceptance discipline wants a registry to
  enumerate; one would have to be minted.
- **`migrate.review-pending` is not a `Finding`** — it is `ERROR_REVIEW_PENDING` in the anyhow
  error-code registry (`invocation_log.rs:89`). Promoting it pulls it under the route floor and the
  finding-key contract. Not a member of the same set.

**Also a contradiction:** `finalize.md:41` says finalize has no private check path *at this phase*,
while `validation.md:506` documents `gate-granted-unused` as *"surfaced at `finalize`"* — and the
code merges it into the phase-2 report at `task.rs:1190`, **promotable to blocking**, so the
divergence is a *gate* divergence.

**Recommendation:** narrow arm — the changelog advisory joins the preview as a missing member of the
existing rule; the base pin stays excluded per M47 — plus one sentence so the exclusion list is
*generated* rather than hand-listed. **Anything wider is a contract revision and must be posed as
one:** surfacing a *blocking* finalize-only finding at `task validate` flips that verb 0 → 3 on a
shipped state, touching the pinned exit-code taxonomy. The brief's claim that *"nothing in this scope
needs a contract change"* is false on the wide arm. **This is the item most likely to force a
mid-build halt.**

---

### F-F · N-4 — `is_route_exempt` is a blanket prefix whose rationale contradicts an invariant

```rust
pub fn is_route_exempt(code: &str) -> bool {
    code.starts_with("conformance.")
}
```

The **entire `conformance.*` family** is exempt by prefix — including blocking *gate* findings, not
just parser diagnostics. So M45's newly-minted `conformance.item-heading-unanchored` inherited
exemption **silently**, the seam-sweep test is green because the exemption swallows the family, and
**the fence structurally cannot catch the next one either.**

The rationale (`finding.rs:252`, restated `validation.md:66`): *"the located message **is** the repair
(fix the named line; **no CLI verb repairs a hand-broken byte**, and an at-parse route would be a
guess)."*

**N-4 falsifies it for managed docs.** "Fix the named line" is the one action
*"the LLM writes only through the CLI"* forbids. Either the exemption is scoped (it holds for a
*foreign* file, not a managed one) or some doc must state that hand-editing is the sanctioned repair
for parse-level corruption of a managed doc — which nothing says, and `storage.md`/CLAUDE.md say the
opposite. Note the doc says *may* stay route-less — so **narrowing is available inside the existing
rule**, not an override.

**Delta:** narrowing from a prefix to an enumerated list is small; the consequence is that every
non-exempt `conformance.*` blocking producer now owes a route, and the sweep names them all at once.
**That count is unmeasured — measure it before the Settle concludes.**

**Second contradiction:** `validation.md:39` states the floor flatly as *"always present … never
`null`"* while `surface-contract.md:28` and `validation.md:51` carve the exemption.

---

### F-G · PT-1 — after the recommended fix, **nothing** exits non-zero on an unadopted foreign file

`schema-conformance.unadopted-instance` is advisory and is **not** a member of `STORE_EXIT_FLIPS`
(`render.rs:676`). Today `migrate-corpus`'s exit-1 is wrong *about its subject* but is the only
non-zero signal that exists. Fork 1(b) removes it.

**Decide explicitly:** does `unadopted-instance` join `STORE_EXIT_FLIPS` (a fifth member — the table
is built for exactly this and the AGENT.md exit-flip clause is derived from it, so it is cheap and
fenced), or is the pre-1.0 answer *"adoption is advisory everywhere"*? **Not deciding it is the
false-green the brief warns against, one layer up.**

**Axis note:** `storage.md:166+` carries a standing placement census with a grep recipe and an
inversion rule — but it enumerates *`schema.location`/`placement` consumers*, **not** *"surfaces that
classify a committed file at a managed home"*, which is PT-1's actual axis. The sets overlap and are
not equal; planning must say which rows it covers rather than treating the census as PT-1's registry.

---

### F-H · Fork 6 would overturn a design of record M48 settled one wave ago

`validation.md:435`: the task-scope reconciler emits `unadopted-instance` *"byte-identical to the
store door's"* route, because *"one file answered **two codes** depending on which door asked, and
the stable `(code, target)` key … was not stable across the doors of one binary."*
`worked-examples.md:3163` (flow 48 arm 4): *"One file answers one code and one route at every door …
the split is proven per **file**, never per report."* `project-setup.md:156` records the same.

Suppressing it at the task gate re-introduces the door-dependent answer M48 removed, and **moves a
shipped acceptance arm**. The cost is real — 25 foreign files → 25 rows × two surfaces, 23.6 KB,
uncapped, no mute — but arms that keep the guarantee exist: a per-file cap, a collapse-with-count
line, or first-encounter-per-store-state. **Arm (a) as written does not.**

*Reproduced unprompted during the gap pass:* an ordinary `task finalize` on a `README.md` tweak
printed output that was **entirely** the foreign `CHANGELOG.md` advisory — two lines, zero about the
task.

---

### F-I · A **three-member permanent-dead-end class** in the migration transform — proved end to end

The Scope audit found one instance (`milestone-record`'s required `set: on-transition`). The class is
wider and materially worse: **the route the refusal prints does not work even when followed exactly.**

Driven through the real binary (methodology `idea` bumped v1→v2, manifest re-pinned, release build,
real repo, committed v1 instance):

```console
$ jigc migrate-corpus ; echo "EXIT=$?"        # field = { id: reviewed, type: date, set: on-create }
blocking · migrate-corpus.prose-needed — `ideas/probe-idea.md`'s migration mints a new **required**
  prose slot, which no transform can fill …
  route: author the new required prose in `ideas/probe-idea.md` through the write verbs, then re-run
EXIT=1

$ # …the agent then does exactly what the route says, authoring the value into the committed doc…
$ jigc migrate-corpus ; echo "EXIT=$?"
blocking · migrate-corpus.prose-needed — …same message, same route…
EXIT=1                                         # forever
```

Identical for a plain required field with no `default:`/`set:`. **Three named sites:**
`transform.rs:262-269` (the `ProseNeeding{leaf: Some(_)}` branch is unbuilt and returns `Unsupported`
before any presence check) · `transform.rs:364-365` and `:432-433` (a `set:`-bearing field with no
`default:` falls in the `Err` arm at **both** loci; `with_stamp_default` rescues only the field
literally named `schema-version`, and only in `Simple` sections) · `transform.rs:638`
(`try_migrate_doc` does `transform(...).ok()?`, collapsing `Unsupported`, `Unclassified`, parse
failure and gate failure into one `None`, which `migrate_corpus.rs:866` then renders as
`prose_needing_finding`).

**This is a law-1 lie on a blocking path with an unfollowable route** — the N-4 / N-5 class again. The
deliberate contrast is damning: `Unclassified`, `NarrowedCardinality` and `RemovedField` are all
refused **pre-fold** with honest schema-authoring routes (`migrate_corpus.rs:640/657/674`), precisely
because *"the fold's halt route ('author the prose, then re-run') would be a lie."* Three refusal
classes got that treatment; these three did not.

**The transferable enumeration — no `set:` kind is migration-time-deterministic on its own:**

| added leaf shape | classifies to | transform | migratable? |
|---|---|---|---|
| `default:` (no `set:`), either locus | `Added{Optional,Item}Field` | value spliced | ✅ |
| `optional: true`, no default | `Added*Field` | byte no-op | ✅ |
| **`set: on-create` / `on-transition`**, required, no default | `Added*Field` | `Err(Unsupported)` | ⛔ **permanent** |
| **`set: schema-version`**, required, no default | `Added*Field` | `Err(Unsupported)` | ⛔ except the one CLI special case |
| **`set:` + `default:`** (any kind, either locus) | `Added*Field` | **value spliced** | ✅ |
| `set:` + `optional: true` | `Added*Field` | byte no-op | ✅ |
| required, no default, no set | `ProseNeeding{leaf: Some}` | `Err(Unsupported)` | ⛔ **permanent** at header locus |

**The fix is provably a one-liner, not a deriver.** Measured on the conformance side: a `set:`-bearing
field **absent** from a doc yields `author_required=false` and **zero** conformance findings. The doc
would gate clean if the transform simply no-op'd — so `transform.rs:364/432` refusing is a **false
refusal**, exactly what `SchemaChange::PresentationOnly`'s own doc comment exists to prevent, and
`corpus-migration.md:162` already states the fact (*"mint-time derivers; **a doc lacking the value
already conforms**"*).

**Root cause is a familiar shape:** `placeable_without_prose` (`schema_diff.rs:795`) is a **third,
private re-derivation** of a predicate `corpus-migration.md:176` claims is shared (*"The classifier
calls that same predicate rather than re-deriving it, so the two verdicts cannot drift"*). The
tightening path does call `validate::is_author_required`; the add path does not; the transform driver
uses a **fourth** rule (`decl.optional` alone). M45's census third axis, recurring inside one file.

**Docs owed either way:** `corpus-migration.md:131` and `:264` state the `set:` splice as fact and are
falsified by the binary; `doctype-authoring.md:35` and `:41` inherit the same false claim — and `:41`
bills itself as a *correction* of an earlier draft that was *"wrong for the optional case"*. It was
corrected for the optional case and left wrong for the `set:` case.

**Blocks the build because** any M46 increment that bumps *any* frozen doctype can walk into this and
meet a surface that reports a false cause with a route that cannot work.

---

### F-J · Entry 2's `planning-record` cannot be specified from the record, and its designed growth path strands its own corpus

**Three locked sources, three different gate sets:** `methodology-docs.md:58-70` has **13** rows;
`milestone-planning-workflow.md:63` enumerates **12**, omitting `quote-attributed` (which the same
file names as a gate row at `:46`); `M48/planning-gate-record.md:24` carries an **`overload valve`**
gate in neither enumeration and omits two others. A required-slot set **is** the frozen shape, so
picking wrong costs a bump + migration to correct.

**Worse, the doc's declared evolution mode is corpus-blocking.** `methodology-docs.md:80`: *"a new
lesson is a **new gate-row**."* Under the freeze a new row = a new required slot =
`ProseNeeding{leaf: None}`, which `doctype-authoring.md:42` says **⛔ blocks by design** — so every
historical planning-record fails its own conformance gate until someone authors prose into a past
record for a gate invented after it was written. **The doctype's only sanctioned maintenance
operation strands its corpus, every time.**

**And as specified it cannot express what M47 and M48 actually recorded.** Both hand-filled records
are *two-dimensional*: a milestone-level gate table **plus** a per-scope-item table (M47: 13 items ×
5 gates; M48: 20 × 5), plus declared bounds. Five of the thirteen gates are **per-scope-item**.
Collapsing them into one prose slot destroys exactly the structure that did the work — M48's per-item
cells are where the corrections live (`⚠️ CORRECTED 2026-08-14`), and M47's row 8 is where an
under-designed item was caught. Cells also carry a **verdict** distinct from the evidence, which one
required slot cannot hold.

**Cheap-vs-robust, and the robust arm is cheaper to build:** a **repeatable `gates` section** (item =
`title` + `verdict` enum + `evidence` slot) plus a repeatable `items` section makes adding a gate an
*authoring* act with zero schema change, and matches the `deferral-ledger` / `completion-record`
shapes already shipped. Verified non-blocker: hyphenated section ids are legal (`open-questions`,
`unreleased-changes` both ship).

---

## 3 · The most leveraged single artifact

**A finding-code registry does not exist.** `ERROR_CODE_REGISTRY` (`invocation_log.rs:183`) covers
only anyhow error identities; `is_declared_singleton` / `is_declared_non_unique` / `is_route_exempt`
are *predicates over code strings*, not enumerations.

So the class *"every finding whose message does not self-identify"* (N-6) **cannot be enumerated
mechanically at HEAD** — and the same absence blocks F-E's staging-dependence classification and
F-F's route-owing count. Minting one artifact serves the acceptance of **three** blocking forks.

**Two pieces of good news on N-6, both measured:**
- **Goldens are safe** — `crates/cli/tests/goldens/` has one subtree (`compose`) and **zero** golden
  files contain a finding line.
- **M48's parity fence does not block it** — `text_json_parity_axis.rs:8-10` states the rule
  one-directionally (*"a value the text prints but the JSON envelope withholds is a gap"*). N-6 is
  the **converse**, and the fence is silent on it.

That silence is itself a finding: **the fence's rule has an un-swept direction.** Whether M46 makes
it bidirectional is a genuine fork — the naive symmetric rule would demand every JSON key be
printable in text, which is almost certainly wrong.

*Constraints:* `Location.address` is `Option<String>` and `is_declared_singleton` codes carry `None`
deliberately, so the fix needs an absent branch; and ~68 test sites assert finding text, whose
survival depends on whether the address is **appended** or **inserted** — a decision, not an
implementation detail.

---

## 4 · Smaller items that are nonetheless owed

| # | Item | Kind |
|---|---|---|
| S1 | **`tasks.json` is written non-atomically** (`milestone.rs:218/286/465/885`, plain `std::fs::write`) while `team-ready-state.md:172` names it **shared by all N sub-agents** — an un-swept sibling of M45 Decision 9, which closed exactly this for `file-state.json`. One line per site. | blocking (small) |
| S2 | `migrate-corpus.prose-needed` fires for **any** halted doc (`migrate_corpus.rs:864`) and asserts *"mints a new **required** prose slot"* — a law-1 lie when the halt was an unimplemented field transform. | advisory |
| S3 | `write.machine-maintained-field`'s route **hardcodes the milestone-record** (*"change only through the milestone verbs"*) while firing on `spec:`. Axis: any doctype × any `set:` absolute. | advisory |
| S4 | `storage.md:124` lists `changelog → changelog/`; `storage.md:143` (its own Placement section) and `doctype-map.md:21` say root `CHANGELOG.md`. **Inside PT-1's axis** — a planner deriving the axis from the location list misses the home the defect reproduces on. | blocking |
| S5 | `corpus-migration.md` has **no foreign-file arm** anywhere; PT-1's fix has nowhere to be recorded that the verb's owner reads. | blocking |
| S6 | No design home for **store concurrency** (F-C) — `storage.md` § Derived caches says nothing about concurrent writers and its Open questions omits it. | blocking if F-C lands |
| S7 | No shared home for the **four-door destroying policy** — spread across `team-ready-state.md:92`, a code comment inside `storage.md:121`'s directory fence, and `project-setup.md:150`. | blocking |
| S8 | `write-commands.md` documents **five** of `milestone`'s nine verbs — never `provision`, `list-tasks` or `join`. N2's transactionality has no home. | advisory |
| S9 | `index::load_committed` **writes** on read-looking paths (`index.rs:238`); `jigc start --task` **from inside a worktree** wrote `.jigc/index/edges.json` in the main checkout. The compose path every sub-agent runs first is a writer to shared state. | advisory (feeds F-C) |
| S10 | `reconciliation.md:189` open question — *"concurrent OOB edits during a task … the window between two writes is unprotected"*. Same family as F-C, different subject (human vs CLI). The locking decision should say which window it closes. | advisory |
| S11 | `finalize.md:246` cites `setup.rs:1002`; actual `:1755`. Substance holds. A `file:line` staleness **class** post-M48. | advisory |
| S12 | The **publishing floor is already decided** — `decisions-pending.md:209`: *"DECIDED 2026-07-16 (human): post-v1. 1.0 ships internal"*. The brief's *"ask whether 1.0.0 means outward"* re-opens a settled call; affirm it or state a basis change. | advisory |
| S13 | **`implementation/doctype-authoring.md` is five waves stale** — 0 occurrences each of `M43`/`M44`/`M45`/`M48`, `states-constraints`, `staged-read-back`, `suppressed`, `{{schema:<…>}}`, `CONSTRAINT_REQUIRED_TOKENS`, `Manifest-Repin`, `surface-contract`. Every one is a **hard pack-load bail** for a doctype shipping an author step (`pack.rs:722-767` derives the read-back owe-set automatically; `:784+` requires the declaration be *bought* by a phrase in the step's prose). Its "free of charge" list also omits `doc list` and `doc show --task`. **This is the artifact the Scope audit priced entry 2 against** — costing off it walks a plan to completion, then bails at pack-load on obligations it never named. | **blocking** |
| S14 | **`implementation/doctype-map.md` records the current `schema-version` for one of four bumped doctypes** — `adr` is manifest **v2** (M36 `options`) but the map says *"✅ frozen v1 (M33)"*; `deferral-ledger` (v2) and `milestone-record` (v2) are version-blind; `:41`'s scope pin still describes the M40 ten-at-v1 state. And **`planning-record` appears nowhere in the map** — the one doctype M46 might build is invisible in the artifact whose declared job is to tell *detect gaps* what doctypes a milestone needs. | **blocking** |
| S15 | **Two candidate M46 fixes sit on frozen *dev-pack* schema keys.** `spec.yaml:38` carries `check: criterion-maps-to-test`; `arch-doc.yaml:41` carries `title-names-symbol: true` — both semantics keys **inside the `schema-hash`**. So a schema-side fix for **N-5** or for entry 3's `title-names-symbol` drain is a dev-pack frozen-doctype event (`spec` or `arch-doc` 1→2 + snapshot + migration + `Manifest-Repin:` trailer) on *adopter-facing* doctypes — and **Fork 4 is framed entirely around the two methodology entries and would not see it.** The probe-side fix (`crates/cli/probes/doc-code/src/main.rs`) is schema-free. State which side is taken, knowingly. | **blocking** |
| S16 | **N-1, N-3, N-4 and N-6 need no doctype and no recorded state** (checked individually). N-3's state is correctly gitignored workbench under the settled `.jigc`-is-the-workbench split — any proposal to move it to a committed record should be refused on that split. **N-2 is the only real candidate, and it may not need one either:** `subtask_contributions` (`milestone.rs:3536-3568`) derives `provisioned` / `code_files` / `discarded` from the *registered* worktree list, which is exactly what `cp -R` invalidates. Switching that subject to the **path on disk** — the fail-closed subject M48 already uses at the other three doors — fixes N-2 with **zero stored state on every arm except the fresh clone**. So entry 10's honest question is *"is the fresh-clone arm worth a frozen-doctype bump?"*, not *"is the fix impossible without one?"* | advisory (re-prices F-B) |

---

## 5 · Sequencing owed at the Settle, not at trial setup

- **Declared behaviour changes must be pre-registered.** The trial protocol §1 exempts exactly
  **one** regression. M46 could ship four or more (a `finalize` refusal · an ignored-content refusal
  at up to four doors · `migrate-corpus`'s exit flip · the changelog predicate). Decide the list now,
  while the reasons are live, or each reads as a regression by the rule's own letter.
- **Fork 4 before the trial budget.** If any bump is taken, the next trial's headline subject becomes
  the migration — the brief's own #1 risk.
- **Fork 5 inherits M47's preview contract.** If `milestone finalize --dry-run` ships *and* F-D/F-E
  add refusals at that door, the dry-run must preview them or it re-mints the exact
  *"previews what finalize gates on"* gap M47 closed for the task twin. `finalize.md:248` records
  `--dry-run` as **resolved for the task verb only** — the milestone twin revises a locked
  resolved-open.
- **Two pinning riders** (`implementation/pinning.md:126`): `changelog_gate_advisory.rs`
  *"reads apt by name and genuinely pins three arms of the correct behaviour — while touching none of
  the cells the finding is about"* (F-1's fix must **not** cite it), and
  `start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal` *"asserts the emitted route
  text, so it pins a route a later wave wants to change as expected output"* (B2-2's repair must
  **revise** it, not add beside it).

---

## 6 · The M48 adjudication table is falsified in four rows

`decisions-pending.md:117-131` is a recorded Settle output. This pass refutes rows **5, 7, 9** and
materially re-bases **10** and **11**. The project's own rule (M47's *"a basis-has-changed rebuttal,
not an override"*) requires each to be re-disposed **with the falsifying datum quoted**, not quietly
rewritten. The **M46 demand-counter re-count** owed at this Settle (`decisions-pending.md:38`) was
discharged by that table and is now partly void — entries 2, 3 and 4 each need a fresh
*"we counted N and still say no, because…"* or a yes.

---

## 7 · Bounds

- Four probes, one sha, one binary. Findings marked as exercised were driven; doc-vs-doc readings are
  leads until driven, and are marked as such in the probe outputs.
- **N-2 and N-4's live repros were relayed from the Scope auditors and re-driven by no one** —
  including this pass. N-2 is the one whose disposition moves F-B, F-D and entry 10 simultaneously;
  drive it before its fix is specified.
- The route-owing count behind F-F (how many blocking `conformance.*` producers lose exemption if the
  prefix narrows) is **unmeasured**.
- The six `load → mutate → save` seams behind F-C other than `unmanage` were enumerated by reading,
  not by driving a loss at each.
