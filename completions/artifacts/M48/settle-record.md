# M48 Settle — running record (drafts for DECISIONS.md at Settle close)

## F7 — the hollow optional heading. SETTLED 2026-08-13: arm 2 (fix the guidance).

**Decision.** The template stops instructing an omission the format does not have. F7 is
reclassified as a **law-1 "nothing lies" defect in pack prose, not a format defect** — the cause
chain is *template says "omit" → agent omits → the format renders the heading regardless → hollow
`## Options` ships*, and the format is behaving exactly as designed and documented. The lying link
is the template.

**Why arm 1 was refused, against the wave's razor** (*in scope if it makes an existing surface more
reliable / discoverable / cheaper to use correctly; out if it opens a new domain capability*): arm 1
fails it twice — it makes the store *less* reliable (it forks the canonical byte form, so an
rc.11-authored ADR is a hard parse failure on rc.10 **with no version signal**, since `adr.yaml` is
untouched and no `schema_hash` moves), and normalizing existing corpora requires a **section-removal
transform kind that does not exist** — new domain capability in the most literal sense.

**Prior art it would have overturned** (found independently by all four gap-detectors; cited by
none of the charter): `design/corpus-migration.md:61` rejects parser-tolerance **by name** (*"two
valid byte forms for one schema — forking the frozen canonical form and touching the byte-stable
parse core, the #1-retired risk"*) · `design/document-type-schema.md:173` (*"heading is still
rendered … optionality governs prose, not heading presence"*) · and `schema_diff.rs:149`, whose
invariant (*"the v2 writer emits the heading unconditionally, so a historical doc lacking it is
non-canonical"*) is the **entire justification for the shipped M36 `adr` v1→v2 migration** — arm 1
would make that migration's own output the artifact F7 calls a defect.

**Tier correction.** F7 **leaves tier 1**. The charter filed it now-or-never because it *"changes
rendered bytes of committed docs"* — true only of arm 1, and arm 1 is no cheaper now than later (the
v1 freeze already binds and the change is invisible to the freeze in either era). Arm 2 changes no
bytes of any committed doc, and the offending string is a `hint:` key, which `erase_presentation`
strips — so nothing at the freeze moves. **F7 is a tier-2 understandability item.**

**Riders, both owed:**
1. The conversion-ledger row is rewritten. It currently reads *"red test: omitted optional section
   renders no heading"* — pre-committing to arm 1. Under arm 2 the red test is the opposite: the
   template no longer instructs an omission, fenced where the guidance is generated.
2. `implementation/decisions-pending.md:275` (*Parser tolerance for a genuinely-absent optional
   section*) gains an explicit **"counted, still deferred"** line. It was filed under *No firm
   trigger yet* rather than keyed to a milestone, which is exactly why the charter drafted F7
   without it.

**Declared non-defect:** `doc show commit:<id> --task` rendering a hollow `## Body` is **correct**
under the locked record. The commit *sink*'s stripping is the special case, and it stays special.

---

## F3 — the leftover destroyer. SETTLED 2026-08-13: robust — the classifier at every destroying door.

**Decision.** The guard's **subject changes from "the registered set" to "every path this call is
about to remove."** One fail-closed classifier — `git -C <path> rev-parse --show-toplevel` yielding
three verdicts (`rc!=0` ⇒ *unverifiable, refuse if non-empty* · `toplevel == path` ⇒ live linked
worktree, reuse the existing dirty probe · `toplevel != path` ⇒ no own linkage, refuse if non-empty)
— adopted at **`provision`, `discard` and `uninstall`**, plus a second arm reusing `task.rs`'s
existing `dropped_staged_docs` enumerator so `uninstall` also guards `.jigc/tasks/<id>/docs/`
authored prose. `--force` is the escape hatch at each. `finalize`'s landed teardown **unifies the
probe only**; its policy stays M46 entry 11, now one call site from closure.

**Argued by an independent robust-advocate before the human chose** (the M34 discipline). Two of its
findings refuted charter arms on evidence:
- **`git worktree prune` is a dead end** in the ordinary `cp -R` shape — the copy's admin record
  points at the *source's* still-existing directory, so `prune` removes nothing. A refusal routed
  there would be a **law-1 lie**. Fork 1's second arm is refuted, not merely declined.
- **The discriminator separates *linkage classes*, not the three shapes.** `rc=128` covers *both*
  deleted-record and copied-then-source-moved, and in that class git cannot say whether the
  directory is dirty — so the probe is **fail-closed on non-empty**, never a dirty check. That class
  contains the walk's own repro.

**The advocate declined to overreach, and that shaped the scope:** **M46 entry 10's durable
"was provisioned" fact is NOT required** — the guard's question is *"can I prove this path holds
nothing precious?"*, never *"is it ours?"*; ownership only matters if you want to be **more**
destructive. This keeps a frozen-doctype `schema-version` 2→3 + corpus migration out of the wave.

**Why the property and not the instance.** The doors **compose into** the loss: `discard` politely
**skips** a non-registered leftover, orphaning it; the next `provision` steps on it. Two doors each
individually "safe" produce the destruction. `design/team-ready-state.md:92` states the property as
an enumeration — *"The un-landed teardowns left are `discard`'s and `jigc uninstall`'s, and both
refuse on a dirty sub-task worktree"* — now false in two independent ways, and this is the **second
time** that sentence has been falsified (M47's audit broke its predecessor). `uninstall --help`
claims *"Idempotent and non-destructive: … your own file content is preserved byte-for-byte"* over
reproduced unrecoverable loss.

**Owed:** `crates/cli/tests/milestone.rs:2129-2141` currently **pins the loss as expected output**
(plants a junk file in a non-registered leftover, asserts clear-and-exit-0). It is **revised**, not
appended to — and the junk-dir clear must keep working. Whatever assertion lands there is the
pinned 1.0 contract. `design/team-ready-state.md` gains the **provision** door (today no `design/`
doc states what provision does to a pre-existing directory at its target path).

**Honest cost:** ≈ half an increment over the cheap arm. The fixture tax is paid either way —
`trial_corpus::copy_state()` hard-refuses any corpus with `.git/worktrees`, so leftover states build
fresh per arm; the cheap arm just pays it for one shape instead of five.

---

## F1 — the read-back fence. SETTLED 2026-08-13: the union derivation + the read-surface doors.

**Decision.** The pack-load fence keys on the **union of two structural signals** — a `{{cli.<id>}}`
ref resolving to a `doc <write-verb>` (18 steps) ∪ a `{{schema:<T>}}` ref (13 steps) — which covers
**30 of the 31** write-soliciting steps. The single miss (`locate-from-spec.yaml`, literal-prose-only)
is the one step that already names `doc show`. **The fence iterates the derivation, never a
hand-copied list.** `doc show` joins **both** packs' command catalogs (it is in neither today).
Beyond steps, the read-surface doors an agent hits first are included: **`doc list`'s empty-set line
gains a route** to the staged read, and **`doc list --task <id>` is accepted** (today it dies at clap
with a bare parse error and no route).

**Why the union and not either signal alone:** catalog-only silently exempts all twelve
`author-migration-*` steps; schema-only misses every ordinary `author-adr`/`author-spec`/`author-commit`
and all eleven methodology `author-*`. M44's D5 fence is precedent for the **pipeline**, not the
**set** — a correction to the charter, which cited it as the mold for both.

**Why iterate the derivation:** `design/surface-contract.md:74` records the trap live — *"A
declaration is a receipt, and a receipt outlives what it stands for: the M47 pre-decompose review
demonstrated it live, deleting 590 characters of copy-in/append contract prose from a migrate step
while keeping its front-matter code — every fence stayed green."* The existing sibling-tip fence
(`cli.rs:1364`) hand-copies its pairs; F1's must not.

**Reuse verified, not inferred:** per-origin-pack catalog resolution is sound
(`enumerate_store_workflows` resolves each catalog against the workflow's origin pack;
`CompositePack::read` is winner-take-all whole-file, so a naive composed read would break).

**Home:** a **fourth tier** of `design/surface-contract.md` → The stated-at fence — it shares the
pack-load pipeline, `CONSTRAINT_REQUIRED_TOKENS`, and the presence-never-content A-3 bound with the
three tiers already there. **No new part-doc.** The path-local rule at `surface-contract.md:69`
(*"stated once per step that solicits slot prose … never hoisted into one home a later step could
sit below"*) is the governing precedent and gains the union tier beside it.

**Known tension, accepted:** this adds a line to ~30 steps in a wave that also carries composed
density as a finding (the `{{schema:changelog}}` block is already 58 of 154 lines). ~612 compose
goldens regenerate, **after** the surface-changing fixes, per `pinning.md`'s sequencing.

---

## F2 — the dropped title. SETTLED 2026-08-13: reject, and ship the in-task title change.

**Decision.** `doc author` / `doc create` **reject** a payload whose `title:` diverges from — or
would silently no-op against — the bound identity, and the rejection **routes to a supported in-task
doc-level title change**, so the correction has somewhere to go.

**The class is three cells, and the charter's two arms answer one.** Cell A (reported): same-slug
re-author drops the title, stale H1, success ack. **Cell B (verified through finalize):** a
*divergent* title is not ignored — it **mints a second doc**, and `task finalize` exits 0 committing
both (`docs/decisions/adopt-redis.md` **and** `adopt-valkey.md`). Cell C: `doc create` is the same
door, byte-identical ack whether the title applied or not. The *"ack `title ignored — create-only`"*
arm **cannot** cover cell B — there the title was not ignored, it was used, to mint.

**Cost, established:** `title` is **not a `Field`**, has no `set:` kind, and is outside
`schema_hash` (the H1 renders from the doc id) — so this is CLI/engine work on the identity op:
**no schema change, no version bump, no corpus migration, no manifest touch.**

**The invariant is honoured, not breached.** CLAUDE.md's *"a doc-level identity change is allowed
only through the atomic `jigc rename` op — never an untracked move"* targets the **untracked move**;
`write-commands.md` scopes `rename` as *"task-less and self-committing, mutating the committed store
directly"*, which **carves out** the staged case rather than forbidding it.

**Owed:** a **three-cell doctype census**, or the fix repeats the incomplete-sweep shape M45 exists
to prevent — (a) slug-identity doctypes, (b) placement / `display-title` singletons where there is
no slug identity and the op must **refuse**, (c) enum-`id-from` repeatable items where
`write.identity-change` already refuses. And `design/write-commands.md:40` is **revised, not
extended**: it states `author` over a committed instance *"does **exactly three things**"* — a closed
set of which `title:` is none.

---

## F16 / M46 entry 2 — the checkpoint record. SETTLED 2026-08-13: stay deferred, re-counted at 7.

**Decision.** The charter contradicts itself — tier 2 (`:96`) lists F16 as in-scope while fork 4
(`:104`) defers entry 2, and **entry 2 *is* the checkpoint gate-record**. Fork 4 governs: **F16
formally leaves tier 2** and routes to M46 entry 2, re-counted at **seven** demands and still
deferred, in the demand-counter rule's own blessed form.

**The reason, stated:** a record where none exists is **new domain capability**, which the razor
refuses — a checkpoint record is not the reliability or discoverability of an *existing* surface.
Entry 2's own trigger (*"the planning workflow is (re-)encoded into the methodology pack, or the v1
in-place migration"*) is **verifiably still unfired at HEAD**: `planning.yaml`'s last touch is M47's
shared-`author-commit` split (a step-composition edit), `settle.yaml`'s is M16, the original encode.

**The charter priced absorption at zero and it is not zero:** it lands either a new methodology
doctype — an **eleventh** manifest entry, all ten re-verified under the strict set-equality assert —
or a new field on `decisions-log`/`milestone-record` (version bump + snapshot + migration).

**Carried, not fixed:** `Checkpoint:` remains pure emission (a single text prepend in `compose.rs`;
no verb, state, field or gate consults it, and it does not halt the binary), and
`design/methodology-docs.md:77` remains a **design of record for a `planning-record` doctype that
does not exist**. Both are recorded against entry 2 rather than silently carried.


---

## F8 — the near-miss tip. SETTLED 2026-08-13: jigc owns the unknown-subcommand error surface.

**Decision.** Three parts. (1) **`jigc config get` / `config list` ship** — chartered explicitly by
the razor (*"the generosity is in admitting new surface (a `config get`, …)"*), and small: 4 primary
knobs plus the 37 `validation.<probe>.<check>.severity` keys. (2) **Curated tip entries** for
read-shaped misses join `unknown_subcommand_tip`. (3) **jigc renders its own message for the
unknown-subcommand clap error kind**, instead of letting clap render its suggestion — keeping
did-you-mean intact for every other error kind.

**Why (3) rather than the map alone:** jigc's tip appends *after* clap's whole error block, so
`jigc config get` prints clap's `tip: a similar subcommand exists: 'set'` **above** jigc's correction
— answering a read intent with a write verb, a law-1 lie the wave's own razor forbids. Third shape:
`jigc config list` produces **no tip at all**.

**Declared build-time spike, with a stated fallback.** The interception point for the
unknown-subcommand error kind is **not verified**. If it is not clean, fall back to
`default-features = false` minus `suggestions` (verified surgical at the Cargo level — the repo uses
only `derive` plus defaults) and grow the curated map, accepting that did-you-mean goes globally.

---

## F14 — the manufactured advisory. SETTLED 2026-08-13: state the gate as a discriminating axis.

**Decision.** The catalog entry for `single-task` names that it grants the changelog gate and what
that means when the change is not user-facing — so `changelog-recording.gate-granted-unused` is
**expected rather than an ambush**. Tier-2 wording under the style guide's routing-surface rule
(*"workflow descriptions are routing surfaces — state when-to-use, when-NOT-to-use, and the
discriminating trigger axes"*). No structural change.

**The charter's premise is refuted:** the catalog states the intent verbatim — `single-task —
implement one scoped change end-to-end, recording its decisions as ADRs`. The defect is **bundling**
(`allows-create: [{adr, as: decision}, {changelog, as: change}]`; it is the only catalog workflow
pairing code + ADR without requiring a spec), not routing.

**Two arms refused with reasons.** *Suppression* is twice-declined — M42 fork 6 settled it and M43
fork 4 re-posed it as the `#type == docs` carve-out and **declined** (*"it hard-codes 'docs commits
are never user-facing' as CLI policy"*); taking it now is a third pass at a settled question. *A
conditional gate* is barred outright by the non-goals (*"no DAGs, conditionals, or a runtime"*).
*Unbundling* is a workflow-shape change on a shipped surface — arguably new domain.

---

## F10 — the lying advisory name. SETTLED 2026-08-13: converge on the store-scope code.

**Decision (human's call, then refined).** Do **both** halves the charter asks for — but via
convergence rather than a split, which delivers both at a fraction of the contract cost.

The code is emitted at two genuinely different conditions: `file_state.rs:1051` **blocking** (a
managed doc drifted and is non-conformant — here `-block` is *honest*) and `:1084` **advisory** (an
unvetted foreign file in a managed location — here `-block` lies). The advisory condition **already
has a correct, shipped, pinned code at store scope**: `schema-conformance.unadopted-instance`,
carrying M42's managed-vs-foreign discriminator route. So the task-scope advisory **converges** on
it; the blocking member keeps `reconciliation.conformance-block`, which is true of it.

**What that buys:** the name stops lying · one foreign file answers **one** code and one route at
both doors · **no new check id is minted**, so the M21 invariant (*"No new doctype, no new check
ids, no engine knob-merge primitive"*) is **honoured, not overturned** · one stable-key move instead
of a code split.

**The real defect behind the charter's item:** M42's managed-vs-foreign discriminator swept the
**store** family and never reached the **task-scope** gate — an incomplete sweep, M45's lens turned
on M42's work, and the same class this wave exists to close.

**Now-or-never, correctly:** a finding-key change is breaking after 1.0. This is the last wave in
which the name can stop lying for free. **Owed before building:** verify no advisory-arm caller
needs a code the store-scope one cannot express.

**Routed, not fixed:** that it fires at all on a task which never touched the file is the
advisory-habituation floor — M46 entry 3, arriving as a **counted datum** (its recorded disposition
stands: drain with the precision fix first, ledger only if the floor survives).

---

## The pre-1.0 additive-key window. SETTLED 2026-08-13: sweep every machine-output surface, fenced by parity.

**Decision (human's call — wider than the proposer's recommendation, then given an axis).** Two
locked contracts state the same rule — `design/doc-read-surface.md:87` (*"Additive keys are
permitted **pre-1.0 only**; from the 1.0 pin, the shape evolves only by an **explicitly versioned
extension** — never a silent additive key"*) and `design/command-output-contract.md:367` for the
write/compose side — and **neither has a discharge sentence**; `:87` is an open ledger of waves
rc.4→rc.8. **M48 is the last pre-1.0 wave, so it closes them.**

**The enumeration rule, so the sweep is plannable and testable:**

> **A value the human/agent text already prints, but the `--format json` envelope withholds, is a gap.**

Derived from a code-side axis the repo already has — `format_json_success_axis.rs` bijects against
the clap verb tree, so *"every machine-output surface"* enumerates as *every leaf verb that speaks
`--format json`*. **Ships as a standing parity fence, not a one-time census**, so a future verb
cannot regress.

**Known members** (each with a recorded demand): `doc schema`'s id-source / write-key marker
(F12 — `contract-version` **4→5**; a reader has no way to learn the payload key is `--title`) ·
`setup --format json`'s `hook_file` (M46 entry 12 — computed, carried, key withheld; **entry 12 is
ABSORBED**) · `ConfigAck`'s uncommitted-state key (F5, three measured instances). The fence will
find the rest.

**Deliberately excluded:** keys for values **nothing currently computes** — re-derivable state,
counts, internal identifiers never rendered. No demand, no witness; adding them is inventing
contract surface.

**Cost, stated:** larger than the three known members. Some envelopes carry pinned
`contract-version` fields that move, each with docs and goldens following. **Plausibly its own
increment** rather than a rider on F12.

**Refused arm:** the `category`→`title` schema rename for F12 is **not available** — it classifies
`RemovedField`, a recorded refusal whose strip arm destroys committed values. Projection-only.

---

## M46's ledger — SETTLED 2026-08-13, entry by entry (fork 4, the agenda item that closes it).

| # | Disposition | Basis |
|---|---|---|
| 1 gate-command / evidence | **stay deferred**, count unmoved | untouched by the trial; the corrected arms (attested-prose vs CLI-observed) still unposed |
| 2 checkpoint / gate-record | **stay deferred, re-counted at 7** | trigger verifiably unfired at HEAD; a record where none exists is new domain capability |
| 3 acknowledged-findings ledger | **drain first** — disposition unchanged, **new counted datum** | F10's firings on unrelated tasks join the count; the F5 option-(a) precision fix is still untried |
| 4 managed-doc read-side | **SPLIT** — own-work subclass **closed by F1**; **search subclass stays deferred** | the charter's prediction tested **half-right**; `ideas/doc-search.md` stays parked, its shape is design-workflow work |
| 5 Pest is-a-test tier | **stay deferred** | untouched |
| 6 symbol-mention-sweep | **stay deferred** | untouched; the three-shape fork still unresolved |
| 7 item-slot repair verb | **stay deferred** | untouched; motive still largely gone |
| 8 file-state introspection | **stay deferred** | F10 supplies one live diagnosis, but the fix does not need it; a read verb with no designed shape follows entry 4's reasoning |
| 9 store locking | **stay deferred** | untouched; contention, not corruption |
| 10 durable "was provisioned" | **stay deferred, motive drained** | the robust-advocate established F3's classifier does **not** need it — *"can I prove this path holds nothing precious?"*, never *"is it ours?"* |
| 11 dirty-worktree guard | **PARTIALLY ABSORBED** | provision/discard/uninstall taken by F3; `finalize`'s landed-teardown **policy** stays deferred, now **one call site from closure** (the probe unifies) |
| 12 `setup --format json` `hook_file` | **ABSORBED** | into the machine-output parity sweep |

**The razor refused something, deliberately.** Entry 4's search half and entry 8's read verb were
both declined on the same ground — *a missing capability, not an unnamed one*; the lens this wave
attacks is **"the capability exists, no composed surface names it,"** and search has nothing to
name. Recorded because a wave that cannot refuse cannot halt.

---

## F15 — "milestone" names two objects. SETTLED 2026-08-13: the non-schema half only.

**Decision.** Planning's finalize step **names `jigc milestone create <the title you just used>`**.
Pure pack prose, zero doctype reach.

**The structural half is refused on three independent grounds:** it is a **one-way door** on two
manifest-frozen doctypes (`roadmap` v1 `f02a981…`, `milestone-record` v2 `4c725b8…` — a bump plus a
corpus migration plus a manifest re-pin, and it would be the *second* candidate bump on
`milestone-record` this cycle) · it rests on an **item-block `ref` shape zero shipped doctypes
exercise** (five `type: ref` declarations pack-wide, **all doc-level header fields**; the engine hook
exists but has never carried a shipped instance, and `methodology-docs.md:42` explicitly defers the
per-item edge-identity question) · and `design/team-ready-state.md:185` settles it the other way:
*"**No managed edge** — `milestone-record → task`/`roadmap-entry` are **not minted**."*

**Stated as a remaining bound, not carried silently by the verb-naming fix:** `add-from-spec` seeds
sub-tasks per spec *criterion* while planning produces prose in a `decomposition` **slot**, so the
two halves still cannot meet and intents are hand-retyped.

**Changed basis, recorded, built on by nothing:** `design/methodology-docs.md:40` justifies the
roadmap's prose decomposition with *"repeatable-inside-repeatable is **not expressible** in the
schema model"* — **falsified at M22** (`Leaf::Repeatable` shipped; `changelog.releases.changes` uses
it today). The better path stays shut mechanically regardless: `diff_item_fields` filters item-block
leaves to `Leaf::Field`, so adding a nested repeatable classifies nothing and the M42 empty-diff
backstop refuses the migration.

---

## The adapter's own instruction files — SETTLED 2026-08-13: take a first slice (human's call).

**Provenance — a VISION commitment never built.** *"For Claude Code that adapter is a one-line
pointer in root `CLAUDE.md` plus **a small set of skill/command files that each just call the
CLI**. … That routing pointer plus the thin file layout **is** the entire integration surface."*
`jigc setup`'s install commit carries exactly **seven** files (`CLAUDE.md`, `.jigc/AGENT.md`,
`.jigc/version`, `.jigc/.gitignore`, `.jigc/config/{.gitkeep,packs.yaml}`, `.claude/settings.json`)
— **no skill or command files**. The pointer and preload shipped; this half never did.

**The principle the human set, and it dissolves the original three options:** guidance belongs in
**adapter-owned artifacts jigc stamps and replaces on update**, never in repo prose copied into the
user's tree — which would make the user's repo the owner of content jigc should own.

**First slice: A — the guides as an adapter artifact.** The content today in
`QUICKSTART.md`/`MIGRATING.md` ships as a version-stamped Claude Code skill. Chosen over *B* (a
command file per workflow — needs a rule for which of 33 workflows qualify, 18 being router-hidden)
and *C* (the read/write contract as a skill — overlaps F1's composed-step fence, two homes for one
statement).

**The load-bearing part is the mechanism, not the prose** — and it cannot be skipped: the artifact
is **stamped with the binary version**, **replaced on `setup`/`upgrade`**, and **refuses to clobber
a user-modified copy, routing instead**. That is the minimal honest form of principle #5 (*every
customization is a recorded delta against a known base version, never an untracked fork*); full
reconciliation is earned when a second artifact appears. It also joins `uninstall`'s enumerated
removal set — which F3 is already touching.

**Proposer's dissent, recorded:** I recommended parking this, on the same ground the search verb was
refused (new surface, undesigned shape). The human took the slice. The distinguishing argument is
that VISION already commits to it and the content already exists, so the work is the ownership
mechanism rather than a new design.

---

## `describe` — SETTLED 2026-08-13: the filter only, not the positional form.

**Decision.** `jigc describe --workflows` and kin — a **filter flag** selecting which entries the
menu returns. The positional single-item form (`jigc describe <name>`) is **not** built.

**Why the split honours the record rather than overturning it.** `design/introspection.md:112`
forecloses exactly one shape: *"**`jigc describe`** — whole-menu, **no positional argument**. … A
single-item form (`jigc describe <id>`) is **not** built — it would blur the `describe` / `--explain`
boundary; **revisit only if a real need appears**."* The foreclosure is a **boundary** argument
(`describe` is the menu, `--explain` is the resolution trace) and a filter does not cross it — it
still returns the menu. The record is **silent** on filtering.

**The measured need** (which the filter may itself drain, deferring the revisit question): 101 lines
/ **24,407 bytes**, longest line **1,167 chars**, all 17 commands on one line, **18 of 33** workflows
carrying *"It is hidden from the router catalog"* inline, **12 of 33** being `migrate-*`. The router
offers 12; `describe` shows 33.

**Constraint on the fix:** `describe`'s prose tier is fenced **non-contractual by design**
(`describe_real_output_is_non_contractual_prose`), so a filter selects **which entries** appear and
**never** pins what they say.

**Handled by the parity sweep, not separately:** the router-hidden flag exists today **only as a
substring of `prose`** even under `--format json` — a value the text conveys and the envelope
withholds, so the parity rule catches it automatically.

---

## The manifest-hash fence — SETTLED 2026-08-13: build mechanism (b), the CI git-diff check.

**Decision.** CI gains `fetch-depth: 2` and a step diffing `git show HEAD~1:<manifest>` against the
working copy for **both** manifests, failing when a `schema-hash` moves without its **co-located**
version field moving (`schema-version` per doctype; `slug-rule.hash` with `slug-rule.version`), with
a **named escape** for legitimate re-pins.

**Why now.** The problem is real and **spike-proven live on rc.9**: while the freeze is
comment-guarded, *"re-pin the hash"* is one keystroke with two meanings — a legitimate prose fix, or
a silent freeze breach shipping an un-migrated structural change **past every gate at exit 0**. Same
class as M42's permanent dead end, which survived six milestones, two RC trials, three completion
audits and a cross-model review while corrupting real adopter corpora. **Its precondition landed at
M47** (the schema-hash presentation projection: with only contract keys hashed, every hash move is a
real contract change), and 1.0 is the moment the freeze becomes load-bearing.

**The other two mechanisms stay refused:** (a) the append-only `(type, version) → hash` ledger is a
**new stored format** — out under the razor; (c) the successor rule is unimplementable in-process
for the reason M47 recorded (a pack-load assert cannot observe a *change* without a recorded prior).

**CORRECTED at the increment-11 plan halt (2026-08-15) — the comparison WINDOW, not the mechanism.**
The settled form (`fetch-depth: 2` + `git show HEAD~1:<manifest>`) inspects only the **last commit of
a push**, because GitHub Actions fires one run per push, at the tip — and this repo pushes in large
batches (**53** commits in one push during this very build; the historically relevant one carried
**37**). **The instance that decides it:** `2c5eee5` (M47 Increment 1, *"the schema-hash becomes a
presentation projection"*) re-pinned **all 16 doctype hashes at unchanged `schema-version`s, in both
manifests, in one commit** — the exact shape this fence exists to catch, and the one both manifest
headers name as *"the declared genesis exemption and the ONLY one."* It landed **34 commits from its
push tip**, and the settled check is **provably clean** over that push (the two-manifest diff between
the tip and its parent is empty). **The fence as settled would have missed the only real instance in
the repo's history.**

**Settled at the halt: widen the base ref, keep the mechanism.** Same one-shot git-diff check;
base = `github.event.before` for pushes, the PR base for pull requests, **falling back to `HEAD~1`**
when absent or all-zeros (new branch, force-push, `workflow_dispatch`) — so the originally-approved
shape remains the **floor** rather than being replaced. Cost: `fetch-depth: 0`, **9.89 MiB** over
1800 commits. Taken to the human rather than absorbed, because it changes human-approved text.

**Two sub-decisions taken at the resumed plan, recorded so they are not re-opened:** *(i)* the fence
is **CI-only**, not part of the local four-command gate — the escape's natural shape is a
commit-message trailer (per-commit, reviewable, not retro-addable without rewriting history), and the
dev-workflow gate runs **before the commit message exists**, so a fence riding `cargo test` locally
could never be made green for a legitimate re-pin; the live arm is `#[ignore]`d and invoked by the
named CI step, while the **verdict-axis tests** (fabricated manifest texts + throwaway git repos)
stay in the local gate so the fence's own logic is standing-tested. *(ii)* it adds **no product
surface** — `Manifest`, `ManifestEntry` (`ty`/`schema_version`/`schema_hash`) and `SlugRule`
(`version`/`hash`) are already `pub`, so both manifest texts are parsed and compared entirely inside
a test file; no new engine or CLI function ships.

**Declared bounds:** it protects **this repo only, not adopters** — honest scope, since this repo is
the only party that edits a manifest — and it is a **build fence, not a surface**, so it sits on the
razor's edge rather than inside it. Recorded as a deliberate edge case, not an oversight.

---

## Flow 48's acceptance — SETTLED 2026-08-13: mint the missing registries; every arm iterates.

**Decision.** No arm is hand-listed. Where a class has no code-side registry, the wave **mints one**
— and in every case the mint repairs an existing weakness rather than adding scaffolding:

| item | registry | what the mint also fixes |
|---|---|---|
| F8 | `unknown_subcommand_tip`'s `match` → a table | its fence currently **hand-copies** the pairs — the same receipt-outlives-contract failure F1 guards against |
| AGENT.md exit clause | `STORE_EXIT_FLIPS` `pub(crate)` → `pub` | unreachable from `tests/` today; iterated only by in-crate unit tests |
| F5 | `ConfigAck` gains `ALL` | exposes the real **six**-write-verb axis instead of the single verb the charter assumed |
| F3 | the classifier's verdicts + a destroying-door list | the fix produces the shape enumeration by construction |
| F6 | rides the shipped `COMMITTING_DOORS` | the class is *doors × the empty-commit outcome* — a registry that already exists |
| F1 · F7 | derivable already | F7's arm iterates the **derived** `optional:` set (only one live persisted cell today, `adr#options`) so a future optional section joins free |

**Why.** The wave's own claim is M45's complete-fix contract — *completeness is machine-checkable* —
and F1's fence was already settled as *iterating its derivation, never a list*. Hand-listing three
acceptance arms would make the acceptance the one place the wave does not hold itself to its own
contract, and it would leave the arms pinned to instances precisely where the newest fixes are.

---

## Determined without a fork (recorded so the build does not re-open them)

- **F13** — **reword, never move.** `design/finalize.md:157` settles the placement: *"**The
  `left-out` advisory prints BEFORE the commit too (M42).** … `finalize` **says what it is about to
  leave out, before it commits**."* Moving the header undoes that decision; rewording preserves it.
  **Two sites, not one** — the second present-tense stem is the carry-over render (`render.rs:1232`);
  a complete fix takes both, and `finalize.md:155` names four render sites the labels move across
  together. **F11** is the same class through a third door (`migrate-corpus`'s `already current`
  headline describing the scan while the action sits in the trailing commit line).
- **F9** — fix the **sentence**, never `slugify`. The hyphen exemption lives in a *test*
  (`slug.rs:1140`, inside `mint_statement_states_the_whole_mint_rule`'s separator-derivation loop:
  `if ch == '-' { continue; }`), and deleting it forces the statement to name `-` as a boundary while
  its glue clause says hyphens bind — so the sentence must assert **two** properties. `SLUG_RULE_VERSION`
  stays 3; neither manifest's `slug-rule` block moves. **Build-note:** a builder who "fixes"
  `renormalize` instead lands `slug-rule-version` 3→4, re-pins **both** manifests, and is
  **permanently unmigratable** (*"no transform kind re-mints an id — a corpus that spans one carries
  two id generations permanently"*). ~84 compose goldens carry the sentence.
- **F4** — the root cause is `setup.rs:999 install_tracked_paths()`, a hardcoded 7-path list whose
  doc comment states the now-stale premise *"the pre-commit hook … **never a tracked file**"* — true
  for `.git/hooks`, false under in-repo `core.hooksPath`. **Fix the premise in the comment, not only
  the pathspec**, or the next hooks-path change re-opens it. A **declared bound** is owed: when the
  hooks dir is outside the repo the hook is not committable at all, so *"a clone gets the backstop"*
  is conditional. `QUICKSTART.md:64-66` is a live law-1 lie this falsifies (*"commits its own
  install … **only the files above**"*, where the hook is in that list) and is revised **with** the
  fix, not independently.
- **F5's text half** — all **six** `ConfigAck` variants state their uncommitted state, not just
  `config set`. The reuse claim is **prose reuse, not code reuse**: the *"each move is a staged
  `git mv` — commit it with your next commit"* line is a bare `eprintln!` inside
  `route_docs_root_repoint_orphans`, firing only for `docs-root` and only on a non-empty stranded
  set. There is no shared function to extend.
- **The `set-slot` marker corruption** (found outside the charter): `set-slot` given `<<…>>`-wrapped
  prose exits 0 and writes the **literal markers** into committed prose. Treated as a **law-1/law-3
  surface fix** (the asymmetry is stated only in `--help`), **not** a CLI content check — the
  determinism boundary gives the LLM *"the prose inside slots"*, and `document-type-schema.md:53`
  blesses a write-time shape check for pack-declared **field** types only. No precedent for the CLI
  adjudicating **slot** payload content, and this wave does not set one.
- **Docs homes** (no new part-doc): F1's fence → a **fourth tier** of `design/surface-contract.md` →
  The stated-at fence · F3 → `design/team-ready-state.md` (which today has **no statement anywhere in
  `design/`** about what `provision` does to a pre-existing directory) + `storage.md`'s teardown
  enumeration · F4 → `design/assistant-adapter.md` → Install discipline · F12 + the parity sweep →
  `doc-read-surface.md` / `command-output-contract.md` (both gain the **discharge sentence** closing
  the pre-1.0 additive window) · F2 → `design/write-commands.md:40`, **revised not extended**.
- **Two falsified closed sets, re-derived rather than appended to:** `write-commands.md:40`
  (*`author` … "does **exactly three things**"* — `title:` is none of them) and
  `team-ready-state.md:92` (*"The un-landed teardowns left are `discard`'s and `jigc uninstall`'s,
  and both refuse on a dirty sub-task worktree"* — false in two independent ways, and the **second**
  time that sentence has been falsified).
- **The 69→66 correction** lands in **one** home (the charter row) with the other three restatements
  (`CLAUDE.md`, `findings-verification.md`, `M48/handover.md`) cross-referencing — and the claim is
  restated precisely: the one mention is a **committed-doc id lookup, not a read-back of in-flight
  work**.
- **The test-registration fence hole**, since every M48 red test lands through it:
  `test_target_registration.rs::registrations()` reads group roots from `tests/groups/*.rs` **on
  disk**, not from `Cargo.toml`'s `[[test]]` list, so a group root added without its `[[test]]` entry
  looks registered while compiling nowhere; both the `Cargo.toml` comment and the fence's module doc
  still say **"ten"** group binaries (there are **twelve**); `pinned_facts/` submodules are unscanned.

---

## Check-scope pins — every new check the design introduces

*(The Settle's own exercise obligation: "a new finding is under-specified until you state the surface it fires against." Pinned here so the build cannot resolve them silently.)*

| Check | Surface it fires against | Severity / effect | Code | Basis |
|---|---|---|---|---|
| **F1 read-back fence** | **pack-load**, per origin pack — the *derived* owe-set (a step with a `{{cli.*}}` ref resolving to a `doc <write-verb>` **or** a `{{schema:<T>}}` ref) | **bail, exit 1** before any composition — not a `validate` finding | a new `states-constraints:` constraint code + its row in `CONSTRAINT_REQUIRED_TOKENS` (tokens naming `jigc doc show` and `--task`) | the three-tier `stated_at_fence` mold, proven by applied mutation at the baseline |
| **F3 destroying-door guard** | the **paths a call is about to remove**, at `provision` · `discard` · `uninstall` | **blocking**, with `--force` as the hatch | door-scoped, following the mold M47 established when it minted `uninstall.dirty-worktree` | M21's *"no new check ids"* governs **validation check ids**, not CLI door error identities — `uninstall.dirty-worktree` is the precedent |
| **F3 authored-prose guard** | `.jigc/tasks/<id>/docs/` — uncommitted authored content with **no worktree involved** | **blocking**, same hatch | rides the same door code as `uninstall`'s | the outside-charter HIGH loss; bounded deliberately — `config/`, the milestone JSON cache and task metadata are declared safe by construction, not probed |
| **F2 title rejection** | the **write path**, task scope — `doc author` / `doc create` payloads | **blocking**, exit 1, with a route to the in-task title change | **proposed: converge on the shipped `write.identity-change`** rather than mint — it already means *"you are trying to change identity through a verb that cannot"* (today: `retitle-item` on an enum `id-from`). Same move as F10; **owed a caller sweep before building**, exactly as F10's convergence is | the F10 precedent set in this Settle |
| **Adapter-artifact clobber** | `setup` / `upgrade`, over a user-modified shipped artifact | **advisory + route, never blocking** — refusing to clobber must not block `setup` | follows `upgrade`'s clean / conflict / orphaned reporting | principle #5's delta discipline; blocking setup on a modified skill would be hostile, silently overwriting would be the untracked fork the principle forbids |
| **Machine-output parity fence** | **test-level**, a standing suite over the clap verb tree | red test | none — not a runtime finding | M45's contract-property-suite mold |
| **Manifest-hash fence** | **CI**, diffing both manifests against `HEAD~1` | CI failure | none — not a runtime finding | declared bound: this repo only, and a build fence rather than a surface |
| **F10 convergence** | unchanged | unchanged | **no new code — that is the point** | M21 honoured |

**One genuinely open build-shape item, flagged rather than silently resolved:** the **in-task
doc-level title change's verb shape** — a `--task` arm on the existing `rename`, versus a distinct
`doc` leaf. The *behaviour* is settled (reject the silent path; route to a supported correction); the
surface shape is a build decision, and it must be taken **against the three-cell doctype census**
(slug-identity doctypes · placement / `display-title` singletons where the op must **refuse** ·
enum-`id-from` items where `write.identity-change` already refuses), not for the adr case alone.

---

## The independent pre-decompose review — 2026-08-13. Two settled claims refuted; four re-settles.

*(The M47 precedent held: that review refuted two Settle claims against the code and forced four
re-settles. This one did the same. Reviewer did not author what it reviewed.)*

### Re-settled

**F10 — the convergence collapsed on its only caller. RE-SETTLED: discriminate, then split.**
The owed caller sweep came back **negative**. `conformance_advisory_finding` has **exactly one**
caller — `reconcile_committed`'s UNKNOWN + non-conformant arm (`file_state.rs:260`) — and that arm
has **no managed-vs-foreign discriminator** (unlike `detect_committed_store:617`, which calls
`is_unadopted_foreign`). So it fires over **managed** docs too, pinned by a standing suite
(`managed_vs_foreign.rs:944`) that drives a **v2-stamped, fully-migrated ADR** with `## Consequences`
deleted and asserts one advisory `reconciliation.conformance-block`. Under the settled convergence
that would move a **managed** doc onto a foreign-file code, and the un-baselined state is that
suite's own *"dominant `jigc validate` corpus"*, i.e. the **common** cell. Convergence would also
falsify `render.rs:718`'s `GATES_NOWHERE` membership derivation (*"emitted only on a store-scope path
and by no task-scope path"*).
**CORRECTED at the increment-4 plan halt (2026-08-14) — this re-settle carried a misattributed
quote, and the correction changes what the increment must build.** The string *"ingest, migrate, or
move it out of the managed location"* is **`conformance_advisory_finding`'s OWN route**
(`file_state.rs:1088`) — **not** `unadopted_instance`'s, which routes to *"adopt — run `jigc ingest`
… it is a foreign file, not an unmigrated managed doc"* (`validate.rs:970`). The reviewer
misattributed it and the proposer propagated it without checking. **Consequence:** the lie does not
*arrive* with convergence — it is **already on the managed cell today** (live-reproduced on a
v2-stamped, fully-migrated ADR with `## Consequences` deleted), and the settled split **keeps** it
there. It is **worse after the fix than before**: today the arm serves foreign ∪ managed and the
route is correct for the foreign majority; after the split it serves **managed only**, so all three
clauses — `jigc ingest`, `jigc migrate --as <ty>`, *"move it out of the managed location"* — are
adoption-or-removal advice about a doc jigc wrote and owns, i.e. wrong for **100%** of its remaining
population. Increment 4's *Proves* (*"no advisory tells a stamped managed doc to migrate itself"*)
was undeliverable by the settled mechanism. **The split was right and incomplete: it fixed the code
and left the route, and the route is the actual lie.**

**Settled at the halt (2026-08-14): route on the stamp, reusing two already-shipped strings.**
Below-version or stamp-absent → the corpus-migration route (`route_schema_conformance`,
`validate.rs:1348-1356`, already shipped store-side); at-version → the blocking twin's hand-repair
sanction (`file_state.rs:1057-1062`, *"fix the file to restore conformance, or revert the edit — this
is the one case a managed file is yours to hand-edit"*). The stamp split is load-bearing rather than
decorative: **hand-repair advice over a *stale* managed doc** (a v1-stamped ADR under v2, which lands
in this same arm) **is itself wrong** — it tells the operator to hand-fix what `migrate-corpus` must
rewrite. No new mechanism, no new check id, both strings already ship, and `versions` is at the emit
site by construction of this increment. No locked doc answered this: `project-setup.md:108,152`
designs this arm **exclusively** as the foreign-squatter G4 gate — the population the increment
removes from it.

**Two costs the Settle had not priced, verified at the halt:** `is_unadopted_foreign` needs
`versions` + `priors`, and **none** of `reconcile_committed`, `reconcile_committed_store` or
`validate_task` carries them — they thread from **three CLI doors** (`task.rs:862`,
`milestone.rs:3333`, `milestone.rs:887`, the last needing its own `make_pack()`). And
**`GATES_NOWHERE`'s stated derivation becomes false** — *"emitted only on a store-scope path and by
no task-scope path"* (`render.rs:722-726`) must be re-derived, mirrored at `validation.md:378,385`;
its **membership** stays correct, since the task-scope emission is advisory and still gates nowhere.
*(The planning gate-record's F10 row said the derivation was "preserved" — that cell was wrong.)*

**Settled:** condition the convergence on `is_unadopted_foreign` at the emit site — the discriminator
the Settle already names as the real defect. **Foreign** converges on `unadopted-instance`; the
**managed** cell keeps `reconciliation.conformance-block`, which is honest for a doc that genuinely
drifted. The name stops lying where it lied, and **no new check id** is minted.

**F3 — the classifier and the junk-dir clause were jointly unsatisfiable. RE-SETTLED: fail-closed wins.**
Spiked: `milestone.rs:2129`'s plant is a plain directory with **no `.git`**, and
`rev-parse --show-toplevel` there returns **rc=0 printing the main repo root** (git walks up;
gitignoring `.jigc/` is irrelevant) ⇒ verdict 3 ⇒ non-empty ⇒ **refuse**. The Settle's *"the junk-dir
clear must keep working"* clause contradicted its own fail-closed rule and would have halted the
build at F3's first red test.
**Settled: the junk-dir clear stops working.** `milestone.rs:2129` is revised to expect a refusal
naming the path, `--force` as the hatch. **The decisive argument is the repo's own:** the binary
cannot distinguish `junk.txt` from `precious.txt` — the old assertion passes only because *the test
author* knew, which is pinning data loss as expected output, the exact reason the leftover
re-`provision` left the latent-surface sweep (`0aa38bd`) rather than being fenced. Verdict 3 is not
reached only by junk: **any** non-empty directory at the target path lands there, including untracked
user work that was never a worktree. **Declared behaviour change:** provision refuses where it used
to succeed — breaking now rather than after 1.0.

**F2 — the in-task title change was named, not designed. RE-SETTLED: split on committed-store identity.**
The reviewer refuted the Settle's carve-out argument: *"`rename` is task-less and self-committing"*
scopes the **verb**, not the **invariant**, and `storage.md:66` is the invariant (*"A **path/slug
rename** *is* an **identity change** … allowed **only** through one explicit, atomic CLI op"*).
Nothing settled said what finalize does with the old committed path, its referrers, or the
mid-fan-out guard (`write-commands.md:96-99`) — so the acceptance test could not be written.
**Settled (human's call, after the proposer argued it):** the boundary is **committed-store
identity**, which is the identity model's own subject (*"identity is the path"*).
- **A doc never committed** — re-slug freely in-task, with the **in-task referrer set derived, not
  hand-listed** (role binding + staged refs; all inside the CLI-owned task area). There can be no
  committed referrers **by construction**. Re-slugging it is the same act as minting it correctly.
- **A committed doc copied in** — **retitle-only**; a genuine re-slug routes to `jigc rename`, which
  exists precisely for it, atomic and transactional with rollback.
- **Placement / `display-title` singletons refuse** (no slug identity to change); **enum-`id-from`
  items already refuse** via `write.identity-change`.
**Why not retitle-only everywhere** (the reviewer's smaller recommendation): it manufactures the very
drift the identity model calls worse — VISION's rationale for `rename` existing is *"a slug that
tracks its title beats one frozen out of sync"* — so every in-task correction would commit a stale
slug and charge a second atomic op to undo a mistake the tool made. **It introduces no new concept**:
the discriminator is already computed at that exact branch (it is what makes `create` ack `existed`,
and what `write.already-present` / `create.serial-collision` key on). **And it closes a live bug
retitle-only would leave standing:** `doc create --slug <other>` over a staged doc mints a **third**
doc at exit 0.

### Accepted as corrections (no change of direction)

- **F3's subject was too broad.** Literally *"every path this call is about to remove"* **bricks
  `uninstall`**, which removes `.jigc` wholesale (`rev-parse` there returns the repo root ≠ `.jigc`,
  always non-empty ⇒ refuse every teardown). Scoped to **worktree-shaped paths under
  `.jigc/worktrees/`** plus the separate `dropped_staged_docs` arm.
- **F7's fence cannot be axis-complete, and the Settle claimed it could.** The omission instruction
  lives in **four** hand-authored places — `adr.yaml:26` (`hint:`), `author-adr.yaml:20`,
  `implement.yaml:23`, `author-migration-adr.yaml:18` — and only the `hint:` reaches steps through a
  seam (`{{schema:<T>}}`, whose 13 steps include none of the ordinary ones). A fence over the other
  three is a **grep over prose**, which the M42 lesson forbids. **The fence scopes to the derived
  `optional:` set; the three step strings are an unfenced tier-2 edit, stated as a bound.**
- **The adapter slice mis-framed principle #5.** Refuse-to-clobber is an untracked-fork **detector
  with no delta**, not *"the minimal honest form"* of the principle. Recorded as a **declared
  deviation with its own trigger** (when a second artifact appears). **Sequencing owed:** it needs an
  entry in `install_tracked_paths` — the **same hardcoded 7-path list F4 rewrites** — so **F4
  sequences before it**, plus a 7th step in `uninstall`'s enumerated removal set and a field on
  `RemovedArtifacts`.
- **F15's bound belongs on the surface that creates the adjacency.** The planning finalize step names
  `milestone create`; it must also name `milestone add-task` as the manual seeding path, or an agent
  finds `add-from-spec` and is stopped by a gap the fix itself surfaced.
- **F2's rejection needed more than a code.** Pinned: **surface** = both `doc author` and `doc
  create` at the copy-in branch · **severity** = blocking · **atomicity** = whole payload at **both**
  verbs (the Settle had stated it only for `author`) · and its **cell joins
  `write_miss_shape_axis.rs`**, whose declared exempt list currently carries `create`/`author`.
- **`roadmap.md` has no Milestone 48 section** while `DECISIONS.md` and `decisions-pending.md:77`
  both point at it — the identical dangling-pointer failure M47's Settle catalogued, one wave later.
  Lands in Decompose.

### Confirmed against the code — load-bearing premises that hold

**F1's union derivation is exact**: `{{cli.*}}`→write-verb = **18** (8 dev + 10 methodology),
`{{schema:<T>}}` = **13**, union = **30** distinct with one overlap (`record-changelog.yaml`), of
**66** step files; **exactly one** names `doc show` (`locate-from-spec.yaml:21`, a committed-doc id
lookup) and it is the single miss. · **F7 arm 2 moves no hash** — `manifest.rs:76 erase_presentation`
erases every `Slot.hint` at both loci before `schema_hash`. · **`STORE_EXIT_FLIPS` is `pub(crate)`**
(`render.rs:622`). · **`ConfigAck` has six variants and no `ALL`** (`render.rs:1838`). · **The
sibling-tip fence hand-copies its pairs** (`cli.rs:1385` re-types the `match` arms at `:1344-1367`).
· **F14's premise refutation holds** — `single-task.yaml:2`'s `when:` states the intent verbatim.
*Cost added:* that string is golden-pinned in five suites, so F14's reword regenerates all of them.
· **F3's classifier is sound on the `cp -R` shape** — verdict 2, and the copy's own dirt reads
correctly while the source reads clean.
