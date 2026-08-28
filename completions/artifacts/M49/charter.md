# M49 — the surface-completion wave · CHARTER

**Preparation only.** Scope, claim, razor, forks and exclusions are here. **The decomposition is
not** — that is the next session's Settle and plan, against a baseline that exercises the real
binary rather than this document.

**Chartered on** [the 1.0.0 trial](../RC-1.0-final/trial-record.md): six sessions and an 11-arm
walk on `1.0.0-rc.12`, **zero data loss, zero corruption, zero regressions, no blocking findings**.
The 1.0.0 call is the human's and nothing here blocks it; this wave is the human's decision to
hold v1 to a higher prose-and-routing bar than "nothing blocks".

---

## The claim

> **Every fence M43–M48 built is applied at the sites its wave named, and nowhere else. This wave
> applies each over its axis.**

Not *"fix these N prose bugs."* The distinction is the whole point, and it is **falsifiable**:
if the seams turn out to be applied over their classes already, the claim is wrong and the wave
should shrink.

**The evidence it rests on, three independent instances:**

- **The clap-tip seam covers 2 of N error kinds.** `main.rs` says so in its own words:
  *"Every other clap error kind still prints clap's own render, did-you-mean included."* M48
  covered unknown-subcommand; M46 covered unexpected-positional; **S-4 is unexpected-argument, the
  third.**
- **The route floor binds `Finding`s only.** `is_route_exempt(code: &str)` takes a finding code, so
  **PT-A's anyhow error is outside it by construction** — its sibling one argument away routes.
- **The pack names what a fence made it name.** Measured at HEAD:
  `doc show` **34** · `set-field` 32 · `set-slot` 29 · `author` 26 · `add-item` 12 · `create` 2 ·
  **`doc schema` 0 · `doc list` 0.** M48's read-back fence pointed at `doc show`, and the pack
  names `doc show`.

**Why this matters more than the item count:** the discoverability lens has now landed **seven
consecutive times** across three dedicated surface waves. A fourth wave of one-off fixes predicts
an eighth. Applying the seams over their axes is the only version of this wave that could stop it
— and if it does not, that is a result worth having.

---

## Scope

Every row cites where it is recorded. **Nothing here is a new verb, flag, doctype, schema or
migration.**

### Tier 1 — the record already names the fix

| | fix | kind | needs |
|---|---|---|---|
| **B1** | `doc schema` / `doc list` named **0×** in either pack; `AGENT.md` frames `doc schema` under *"to learn how `jigc` itself behaves"* — tool-introspection, not *"learn a doctype's shape before writing to it"* | prose | pack text + 1 adapter string |
| **S-1** | the permitted heredoc-on-`jigc` form nobody names, while the pack prints `--from-file -` ~30× | prose | pack text |
| **S-4** | the clap tip misdirects (`-- --task`), never names `jigc doc rename` — the third error kind | both | code (the M46 Inc 8 T5 seam) |
| **PT-A** | occupancy refusal carries no route; its sibling on the same verb does | routing | code |
| **C2** | `validation.md:73` states `write.unknown-section` universally; two address forms falsify it, and section-level forms exit 1 **code-less and route-less** | both | docs + code |
| **E1** | `doctype-authoring.md` is five waves stale — names **none** of the M43–M48 pack-load fences (verified: 0 hits) while opening *"This is that list."* | prose | docs |
| **E2** | `methodology-docs.md:40` justifies the roadmap's prose decomposition with *"repeatable-inside-repeatable is not expressible"* — **falsified at M22** (`Leaf::Repeatable` shipped; the changelog uses it). The conclusion survives on other grounds; the stated reason rotted | prose | docs |
| **F3** | `module-layout.md:58` calls a question settled 2026-05-28 *"an open question"* | prose | docs |

### Tier 2 — a rule must be written before the fix, by their own refusals' terms

| | fix | refused on | the rule owed first |
|---|---|---|---|
| **D1** | the text renderer drops `location.address`, so a text finding cannot be identified by its subject | **scope** — *"No rule."* The near-miss citation is killed by *"Notation is illustrative"* | a stated rule that text findings carry identity |
| **D5** | up to **13** blocking `conformance.*` findings ship route-less | **merit** — the narrowing mechanism was the wrong instrument | re-pose as a route batch, not as the refused narrowing |

### Tier 3 — decisions with a fix attached

| | | |
|---|---|---|
| **A4** | what *"hidden"* means in the router's closing claim — 12 catalog / 21 absent / **3 with no stated reason** (`increment`, `ingest-existing`, `router`, exactly the `creates-task: false` set) | prose |
| **F7** | should the **surface** say what the trial protocol briefs an observer about (`validate`'s exit flip, `task validate`'s advisory)? An adopter is not an unbriefed observer with a protocol — the ambush shape, one audience over | prose |
| **D3 · D4** | the `milestone add-task` `next:` line; the `bind` ack's not-staged clause — both refused **as preference** | prose / code |

### Tier 1 additions — from the self-hosting port question (2026-08-28)

Asked: *does the Claude Code workflow keep working when this repo ports to jigc, and does anything
have to move pre-1.0?* The port analysis found **two surfaces of exactly this wave's shape**.

| | fix | kind | needs |
|---|---|---|---|
| **P1** | **Two shipped packs describe the same phase incompatibly.** `packs/methodology/steps/execute.yaml`: *"Run the tasks **strictly serially**: they share **one working tree**, each builds on the last."* `crates/cli/pack/steps/{provision-worktrees,implement-tasks}.yaml`: one **isolated detached worktree per sub-task**, `fan-out: over: {{milestone.tasks}}`, launched together. Both ship; both describe milestone execution | prose | pack text |
| **P2** | **`robust-advocate` is named in ZERO pack files** (verified: 0 hits across both packs) while `milestone-planning-workflow.md` and `design/methodology-docs.md` treat it as a mandatory instrument before any defer recommendation | prose | pack text (`settle.yaml`, `triage.yaml`) |
| **P3** | `validate.yaml` never states the **independence** requirement that is the point of the phase — *"by an agent that did not build it and cannot commit"* | prose | pack text |

**P1 is the sharper one.** It is not a gap; it is a **contradiction between two shipped surfaces**,
and `milestone-completion-workflow.md` records **two production incidents (M38, M42)** from
violating the serial rule. A reader of one pack is misled about the other.

**Verify at the Settle, do not assume — one claim from the port analysis is unverified.** The map
asserts that `milestone add-task --workflow <other>` followed by `milestone execute` emits
`Spawn: … jigc workflow sub-task --task <id>`, which the W-equality re-entry guard then refuses —
*"two features individually correct and mutually unusable."* **This was not driven.** If true it is
a routing dead end and belongs here; if it is a capability gap it does not. **Drive it before
scoping it.**

### The now-or-never subset — decided in this wave or never

The declared posture: *"additive keys are permitted **pre-1.0 only**; from the 1.0 pin the shape
evolves solely by an explicitly versioned extension."* The command-output contract **has no
version integer, by declaration**, so a post-1.0 change means minting a v2.

| | | |
|---|---|---|
| **N1 (C1)** | an undeclared **nested**-section hop returns **four codes across six item-addressing doors** for one defect — and the `--unset` cell is verbatim the sentence a prior commit's own message named as the defect it removed. A **stable-key** change: a driver keys on `(code, target)` | **Settle fork** |
| **N2** | `schema-version` is a **number** on `jigc doc schema` and a **string** on `jigc doc show` — two pinned 1.0 contracts, one name, two types. Verified live. The record parks it as *"recorded, not repaired"* and says why: *"a type change is not an additive key, so the pre-1.0 window does not cover it… and a fix wave does not get to make that call by accident"* — while naming the cost: *"comparing the two is exactly the upgrade-path check a driver automates, and today that comparison needs a cast"* | **Settle fork** |

**Both are decisions with a fix attached, not fixes.** N2 in particular was parked *for* a wave
that would decide it deliberately; deciding it by accident is what the record warns against.

---

## Decided OUT, with the ground — so the one-way-door ledger closes rather than goes quiet

**N3 · F15's structural half** — a managed `roadmap-entry → milestone-record` edge. **Out.** It is
a **feature**, not a fix, and M48 refused it on three independent grounds: a one-way door on two
manifest-frozen doctypes (and the *second* candidate bump on `milestone-record` in one cycle) · it
rests on a per-item `ref` shape **zero shipped doctypes exercise** (five `type: ref` declarations
pack-wide, all doc-level) · and `team-ready-state.md:185` settles it the other way outright.

**Its residual pain is real and ships recorded**: `add-from-spec` seeds from a spec's *criteria*
while planning produces prose in a *slot*, so the halves cannot meet and intents are hand-retyped.
That is a **capability gap** under §1. **Its prose half is IN as E2.**

**Also out, and each already refused with a citation:** the search verb · the file-state read verb
· the checkpoint record · `describe <name>` · F7's canonical-form change · F14's unbundling · the
per-instance acknowledged-findings ledger (deferred to 1.1; *"purely additive… no one-way door"*).

---

## The self-hosting port — analysed, and it changes nothing structural in this wave

**The question:** this repo ports to jigc after 1.0, the Claude Code workflow keeps working, and
agent prose shrinks because the pack carries it. Does that work, and does anything have to move
**pre**-1.0?

**It works, and the planning→execution handoff closes without a managed edge.** The chain is:
`planning` authors the roadmap entry (prose decomposition) → `planning-finalize` names
`milestone create` + `add-task` (**shipped at M48**) → an agent reads the decomposition and yields
intents → `add-task` mints identity → `milestone-execution` + `sub-task` fan out → `milestone
finalize` joins. **The prose→intent step is legitimately the LLM's** under the determinism
boundary: the CLI owns structure, placement and identity; the LLM owns the prose. This repo
already does exactly that with its `milestone-reader` agent.

**Four shape items exist, and none of them belongs in this wave:**

| | | why it is OUT |
|---|---|---|
| **S1** `roadmap.decomposition` slot → nested repeatable | the only one needing a **new engine transform kind** — *"Edit a nested repeatable … ⛔ **build the kind first.** No driver today; the backstop refuses the migration"* | a **feature**, and *build the kind first* is the documented working process — M41 built `ValueRemapped`, M42 built three, each when a bump needed one |
| **S2** `milestone-record.tasks` gains a per-task `workflow` | the code prices it itself: *"a frozen-doctype schema bump — a one-way door, out of charter"* | schema change |
| **S3** a task's `increment` · **S4** a milestone's `status` | both `AddedItemField` | schema changes |

**Three reasons they stay out, and the third is the load-bearing one:**

1. **They are schema changes**, and one needs a new engine kind. The wave excludes both by charter.
2. **The blast-radius argument is weaker than it looks.** This repo is **not self-hosted** — no
   `.jigc/`, no `docs/roadmap.md`, so **zero managed roadmap instances exist here**. There is no
   cheap-now window being spent; the freeze already binds pre-1.0, so the bump costs the same
   machinery either way. What grows post-1.0 is the adopter corpus that must migrate — and
   migrating a corpus is what jigc is *for*, on a path M38, M40, M41 and M42 have each proven.
3. **Taking them would break the razor.** They violate no stated rule, so they fail its first leg —
   which means the razor could not refuse them, and a razor that cannot refuse is the failure the
   claim is tested by. **Adding them would make the claim unfalsifiable**, which costs more than
   the items are worth.

**The honest counterweight, recorded because it argues against my own recommendation:** if the
port lands soon after 1.0, S1–S4 get done anyway in the next wave, and doing them here would save
one migration cycle over an adopter set that is small and self-selected. That is a real argument.
It loses to reason 3 — but it should lose in the open.

**What the pack does NOT do is lie about this.** `planning-finalize.yaml` states the gap verbatim:
*"this milestone's breakdown is prose in the roadmap entry's `decomposition` slot, which carries no
criteria — so there is nothing there for it to read, and the intents are retyped from your own
decomposition."* A surface that states its own bound is not a surface this wave fixes.

## The razor

**M46's precedent, and the reason it is here: a wave that cannot refuse cannot halt.** M46's
guarded three-leg razor refused **nine** items with citations, three carrying measured evidence.

An item is IN only if all three hold:

1. **The rule it violates is stated somewhere locked** — a design doc, a fence, a contract.
2. **HEAD violates it**, demonstrably.
3. **It is demonstrable by driving the binary**, not by reading.

**If the razor cannot refuse, the claim is wrong** — that is M46's own falsifiability test and it
is adopted verbatim. Tier 2's two items fail leg 1 today **by their own refusals' terms**; writing
the rule is what makes them eligible, and the rule is the deliverable, not a formality.

---

## Owed to the next session

- **Settle N1 and N2** — both one-way, both contract-touching, both parked *for* a deliberate call.
- **Write D1's and D5's rules**, or leave both out and say so.
- **Baseline against the binary**, not this file. Every wave since M45 has had charter premises
  corrected by a baseline that exercised HEAD — M48's corrected **19**.
- **Then decompose.** Not before.

## Two open items that are NOT prose — flagged so they are not lost a second time

1. **`milestone provision`'s mid-phase-2 non-rollback.** `next-wave-scope.md:149` marked it
   *"IN, if cheap"* and **no M46 increment took it.**
2. **An undischarged standing order.** *"The carryover gate protects staging that exists at the
   mint, not staging that existed before the session — **verify it before the Settle… do not drop
   it.**"* No M46 artifact records that verification.

## Bounds carried in

- **The trial's own instrument found more defects than the product did** — five instrument, four
  product — including an **incomplete fix to one of its own fixes**. Treat this wave's fixes the
  same way: swept over an axis, or they will be back.
- **Five fixes were already owed before this charter** ([trial-record.md](../RC-1.0-final/trial-record.md)
  → Owed), and three of them are Tier 1 rows here. The other two are the answer key's `scope`
  pattern and a during-session sibling to the `session-start` split — **apparatus, not product**,
  and they belong to the trial tooling rather than to this wave.
