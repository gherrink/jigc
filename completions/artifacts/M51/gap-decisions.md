<!-- 2026-09-10 · Opus gap-detector · dimension: decisions · HEAD bd348a83 · target/release/jigc 1.0.0-rc.14 · no cargo, no repo edits · copied verbatim -->

# M51 gap detection — dimension: DECISIONS

Probed at HEAD `bd348a83` against the release binary `target/release/jigc` = `1.0.0-rc.14`.
Spikes driven in a throwaway `dev/jigc-rig committed-singletons` corpus under the session
scratchpad. No repo file edited, no cargo run. Every claim below carries `file:line` or a
driven command. **I settle nothing** — each item names the decision the Settle must take.

Ranked, blocking first.

---

## BLOCKING

### B1 · EC-9 re-opens a decision the human took twice, and the charter's own exclusion list already excludes it
`blocking · prior-art-contradiction`

EC-9's premise is *"the recorded publishing-floor deferral's trigger — first external adopter,
public release, or opening the repo — **fires by definition** when 1.0.0 ships"*
([charter.md:126](../../../completions/artifacts/M51/charter.md), VERDICT ledger EC-9).

The record says the opposite, in the human's own words:

- `implementation/decisions-pending.md:500` — *"~~Decide at the 1.0.0 call, or charter it.~~
  **DECIDED 2026-07-16 (human): post-v1.** 1.0 ships internal; the floor's original trigger
  stands unchanged (*first external adopter, public release, or opening the repo*) and fires
  **after** the 1.0.0 call, not before it."*
- Re-affirmed at the M46 Settle: `decisions-pending.md:419` — *"**S12**, the repo publishing
  floor (**discharged by citation**, not re-keyed — already DECIDED post-v1 by the human
  2026-07-16 …)"*.

So the trigger does **not** fire by definition: the decision of record is that 1.0.0 *is not*
a public release. And S12 is a member of the M46 capability ledger, which this charter's own
exclusions place out of scope — *"the capability wave's remaining ledger … adjudicated entry
by entry at the M48 Settle; an evidence check does not re-open it"* (charter → Decided OUT).
EC-9 is therefore **simultaneously Tier 1 and excluded** by the same document.

Two of EC-9's four facts are also stale: `Cargo.toml:15` already carries `publish = false`
and `:13` `license = "MIT OR Apache-2.0"`. What is undecided is whether to *publish*, not the
manifest value. (`README.md` absent and no `description`/`readme`/`keywords` — both confirmed.)

**The decision owed:** either the human states that 1.0.0 is now a public/outward release —
in which case this is a **basis-has-changed reversal recorded as one**, the repo's standing
form — or EC-9 is refused with the citation. It cannot be taken silently as a fresh scope item.

### B2 · EC-2 and EC-26 key their fix on `COMMITTING_DOORS`, and that registry structurally cannot hold `setup`
`blocking`

Both rows name `jigc setup` as a door needing the fix, and both name the same registry
(EC-26 explicitly: *"Note this is the same registry as EC-2"*). Driven/read at HEAD:

- `crates/cli/src/invocation_log.rs:95-97` — the member predicate is *"a production door that
  runs a **hook-capable** `git commit` on the user's behalf"*.
- `:119-121` — *"the same one exclusion: **`jigc setup`'s install commit passes `--no-verify`
  by recorded design**, so no hook runs and there is nothing to reject."* Confirmed in code at
  `crates/cli/src/setup.rs:1709` (`vec!["commit","--no-verify", …]`) with its rationale at
  `setup.rs:1602-1608`.
- `:130-171` — **ten** members; `setup` is not one.
- `:108-113` — `ERROR_CODE_REGISTRY` is **derived from** `COMMITTING_DOORS`
  (`registry_mirrors_the_declared_members`), so every member must carry a distinct
  route-exempt error identity, and `:174-180` states the anti-collision rule: no member may
  collide with the finding-code inventory. `setup`'s commit rejection already travels as the
  **finding** `setup.install-commit` (driven at baseline2 §2c: no git identity → exit 1
  `setup.install-commit`). Adding `setup` to the registry therefore forces a second identity
  for a rejection that already has one, and a `commit_rejected_axis.rs` cell that **cannot
  exist** (no hook runs under `--no-verify`).

**The decision owed:** the wave must decide whether to (a) mint a *second* subject — "doors
that commit on the user's behalf", superset of `COMMITTING_DOORS`, which is what EC-2's HEAD
probe and EC-26's staged-set guard actually want — or (b) fix `setup` outside any registry and
say so. *"Widen `COMMITTING_DOORS`"* is refused by construction, and the charter's axis line
(*"HEAD posture × the ten `COMMITTING_DOORS`" — "the registry exists; the cell does not"*) is
false for the one door both rows lead with.

### B3 · EC-26's robust arm revises a locked statement in two homes, and the gate's mechanism does not transfer to `setup`
`blocking`

Fork 4's robust arm is *"`setup` **joins** the carryover gate — the gate's subject becomes the
door, not the task"*. The gate's subject is **stated** as a task-minting door in two locked
homes, verbatim:

- `design/surface-contract.md:123` — *"**Every task-minting door** (`jigc start` compose forms,
  `jigc migrate`, `jigc milestone create` …) snapshots the staged state into the workbench"*.
- `design/finalize.md:159` — the same sentence, plus *"**Missing snapshot ⇒ fail-open**"*.

And the mechanism is **working-area-scoped**: the snapshot is written into the task/milestone
dir at mint and compared at finalize — `engine::state::MINT_DOORS` (`crates/engine/src/state.rs:808`),
whose doc-comment at `:860-863` records that this very sentence was corrected at M49 for naming
three doors while five mint. `jigc setup` mints no working area and has no before/after pair, so
it cannot "join the gate": a `setup` guard is a **dirty-pathspec refusal at one door**, a
different mechanism with a different consent shape (`--force`, the destroying-door mold), not a
snapshot/compare.

**The decision owed:** which registry owns setup's staged-set question. `MINT_DOORS` (subject:
a working-area mint) and `COMMITTING_DOORS` (subject: hook-capable commit) both structurally
exclude it — so the robust arm needs a third home, and the fork as posed ("joins the gate" vs
"narrates") conceals that. Also owed: whether the two locked sentences are **revised** (with
their basis) or left as-is with `setup` declared outside the gate.

### B4 · EC-26 has a razor leg-1 citation the charter does not carry — which changes its tier
`blocking` (mis-tiered)

`design/assistant-adapter.md:52` — *"**The install commit's pathspec — `setup` commits its own
install, and only its own install (M48).**"*, *"the pathspec is **enumerated, never a blanket
`git add -A`**"*, and the purpose: *"so the scaffolding never rides the user's first
`finalize`"*.

At HEAD a pathspec-limited `git commit -- <paths>` commits the **worktree bytes** of those
paths, so a user's uncommitted `CLAUDE.md` edit (a pathspec member) lands inside
`chore(jigc): install jigc workspace config` (EC-26, driven by reviewC). The doc's claim
*"only its own install"* is true of the **paths** and false of the **bytes** — a rule stated in
a locked artifact and violated at HEAD, i.e. the razor's leg 1, fully available.

**The decision owed:** the charter files EC-26 in **Tier 3 (capability cells)**, whose admission
argument is weaker. With this citation it is a *hole in a declared surface* (leg 0) at a
committing door that swallows user work at exit 0. Its tier should be adjudicated on the
citation, not on the ledger's "capability" label.

### B5 · EC-3's premise is falsified in one member, understates the class ~4×, and the real decision — *which envelopes 1.0 pins* — is named nowhere
`blocking · prior-art-contradiction`

**Falsifying datum.** EC-3 names four undeclared keys, one of which **is** declared:
`design/assistant-adapter.md:56` — *"the install summary says at the door: the hook line gains a
clause naming the consequence … and **`--format json` carries the same fact as `hook_committed`**"*.
It is declared there and nowhere in `command-output-contract.md` — whose own rule is that
*"every addition is declared **here**, in its own paragraph, as it ships"*
(`design/command-output-contract.md:446`).

**The scope contradiction, inside one doc.** `command-output-contract.md:13` heads
*"**The three surfaces this pins**"* — composed output, write-acks, the findings envelope.
`setup`, `uninstall`, `ingest`, `unmanage`, `rename`, `relocate` and the eight `config`
envelopes are in **none** of the three — yet later paragraphs in the same doc declare keys on
`setup` (`hook_file`), `upgrade` (`checked`), all six `ConfigAck`s (`committed`) and
`migrate-corpus` (`unadopted`, `unfilled`). So the doc pins three surfaces and declares keys on
at least four more.

Driven at HEAD in the rig (release binary), each an envelope key named in no design doc:

```
jigc --format json unmanage CHANGELOG.md   → {"path","identity","dropped"}
jigc --format json config get docs-root    → {"key","layer","op","rejected","value"}
jigc --format json config list             → {"knobs","op"}
jigc --format json ingest                  → {"rows","summary"}
```

**The decision owed, and fork 7 cannot be answered before it:** *is `identity` on `unmanage` a
defect ("an undeclared key on a pinned envelope is a defect") or is `unmanage`'s envelope simply
not pinned?* The contract cannot answer today. "Declare the four vs delete the four" presupposes
the four are the set; the baseline drove ~17 undeclared keys across ~10 verbs.

### B6 · `PINNED_ENVELOPES` is the wave's real one-way door and the charter treats it as a fence
`fork · cheap-vs-robust`

If the wave mints a registry of pinned envelopes (baseline3 §2's option 1/2), that registry
**is the 1.0 contract**, not a test over it: it decides, for 47 leaf verbs, which shapes freeze
at the pin and which stay free. Three consequences the charter does not price:

- `jigc task list --format json` is a **top-level JSON array** (`render.rs:3285` `json(&rows)`),
  so it can carry no `schema_version`, no `findings` and no discriminator. Pinning it freezes an
  envelope shape a driver cannot deserialize uniformly; *not* pinning it must then be said.
- `milestone execute --format json` emits `{task, text}` — a **fourth** composed producer against
  `command-output-contract.md`'s *"**Three verbs emit this composed shape**"* (§1). In no EC row.
- `milestone list-tasks` is `VerbKind::Read` (`cli.rs:1459`) and ships `hook_output`, whose
  declaration is scoped to landed-commit envelopes and record-only acks.

By contrast `ManifestKind::ALL` (EC-22) is **not** a one-way door — an internal enumeration whose
only consumer is a prose fence. The two should not be priced the same.

**Cheap:** declare four keys, leave the pinned set undefined (ships the ambiguity into 1.0, where
"an absent key is a decision" becomes unanswerable because nobody can say which envelopes the
rule binds). **Robust:** decide the pinned set now and fence it — one row per leaf verb, each
either *pinned with its declared key set* or *explicitly unpinned with its reason*. The vision's
own committed trajectory makes this the minimal-correct path: the pin is a one-way door and the
contract's stated posture (*"an undeclared key on a pinned envelope is a defect"*) is unenforceable
without the set.

### B7 · Fork 5's cheap arm cites a precedent whose load-bearing premise does not transfer
`fork · foreclosed-by-doc`

The charter's cheap arm is *"a code and a route inside the flattened `{"error": …}` string,
… which the M50 audit already chose once with measurements (`location: None` ⇒ a `(code, null)`
key strictly less informative than the flattened message)"*.

That refusal is homed at `design/command-output-contract.md:215` and its ground (1) is verbatim:
*"**It would have to be keyed `(code, null)`** — the malformed token is not a URI, no form in the
table above addresses *a string the user typed that names nothing*."* That premise is about
**`store.malformed-slug`**, a token that names nothing.

F-5's subject is a **work unit**, and the same contract already declares that target form:
`command-output-contract.md:167` table — *"a **work unit** → the work-unit ref `task:<id>` /
`milestone:<id>`"*, whose listed members include **`finalize.no-task`**. The engine ships that
finding: `crates/engine/src/finalize.rs:1146 task_missing_finding`, doc-commented *"Its subject is
the work unit whose area is absent, so it keys at the work unit"*. The CLI short-circuits it
first, in one shared home — `crates/cli/src/task.rs:971 no_such_task` — as a code-less `anyhow`.

Driven at HEAD:

```
jigc --format json task finalize no-such-task
  → {"error": "no task `no-such-task` — list live tasks with `jigc task list`"}   rc=1
jigc --format json task validate no-such-task   → same
jigc --format json doc list --task no-such-task → same
jigc --format json milestone list-tasks no-such-ms
  → {"error": "milestone `no-such-ms` does not exist\n  route: …"}                rc=1
```

So F-5 is **not** a cheap-vs-robust choice: the code, the target form and the shared producer all
already exist; the door refuses one layer above the finding that was written for it. The fix has
one home (`task.rs:971` + `start.rs:2131`) and the shipped M50 Increment 9 shape (`Result<_, Finding>`,
so a bare `anyhow` cannot inhabit it).

**The decision owed:** re-pose fork 5 as *"make the declared contract true at these doors"* vs
*"declare the doors permanently outside the envelope"*, not as *string-vs-envelope*.

### B8 · The contract's own claim about the flattened envelope is false at those 22 doors — a razor leg the charter does not cite
`blocking`

`command-output-contract.md:202` — *"the flattened *text* … **since M50 Increment 10 carries the
code and the locus inside it exactly as the agent-text surface does**."*

Driven above: no code, no locus, at four of the 22 doors. That is a stated rule violated at HEAD
(leg 1) on the pinned write-side contract, and it is a Tier-2 law-1 row in no ledger entry.

### B9 · Fork 3 rests on a reading `design/finalize.md` may not support, and the sharper contradiction is unnamed
`blocking · prior-art-contradiction`

EC-29 asserts the worktree residue is *"design-declared (`design/finalize.md`'s config-layer row
says **worktree untouched** on purpose)"*, and fork 3's cheap arm is *"keep the declared index-only
fidelity and correct the all-or-nothing header"*.

Read at `design/finalize.md:178`, the phrase sits **inside the restore mechanism's parenthesis**:
*"on failure restore the present ones (`update-index --cacheinfo`, worktree untouched) and **drop**
any the stage newly added"* — it describes how the *index* entry is restored without disturbing the
file, not a decision that a rewritten `.jigc/version` stays modified. Every sibling row in the same
table explicitly restores the worktree: *"undo the promote's worktree write … working area intact"*
(`:174`, `:175`), *"the worktree never reset to HEAD"* (the record-only row).

And a second, sharper contradiction the charter does not name: `DECISIONS.md:4788` records the M45
audit fix as *"so a rejected finalize now rolls back **every path jigc's own staging contributed**"* —
false of the worktree half, at the very site EC-29 drove.

**The decision owed:** the cheap arm is not *keeping* a declared decision, it is **declaring a new
one** (index-only fidelity, worktree residue accepted, with the ordinary reach *upgrade → finalize →
hook rejects*). Say so, or take the robust arm. Either way `:183`'s *"Before phase 6, all-or-nothing"*
and `DECISIONS.md:4788`'s sentence both move.

### B10 · The wave's own claim spans a token family the charter splits across two tiers with two rules
`blocking` (scope coherence)

The claim: *"no caller-supplied token … reaches a door that destroys, **commits or moves** without
that door having adjudicated it."*

`crates/cli/src/cli.rs:1552` classifies **`value`** as `ArgToken::Plain`, and `jigc config set
docs-root <value>` turns it into a **path component** at a **moving** door — `git mv` of the
committed corpus (EC-4, driven). Its adjudication is three predicates
(`config::unusable_root_reason`, `is_workbench_root`, `trackable::untrackable_reason`), and EC-27
drove that set incomplete: `"   "`, `" "`, `"  x  "`, `"-"` all land at exit 0 and `config get`
reads back a value visually identical to unset.

Fork 1 scopes the family to five argument ids (`path`, `from`, `file`, `from_file`, `target`);
EC-27 sits in **Tier 3** with no fork and a different predicate (*"a value that reads back as
itself"*). So the claim is not closed by fork 1's robust arm, and two tiers hold two halves of one
class with no shared decision.

**The decision owed:** is the family *the five ids the `ArgToken::Plain` doc-comment names*
(`cli.rs:1500-1508`) or *every `Plain` argument that becomes a path component* (six, incl. `value`)?
If the former, say what closes the claim for `value`; if the latter, EC-27 moves into fork 1.

### B11 · EC-1's leg-1 citation exists and is not carried; its review-gate half is a *new* requirement the razor may refuse
`blocking` (sink half) + `advisory` (review gate)

**The citation the charter does not use:** `design/auto-migration.md:66` declares the retire
*"jigc's **first byte-destructive operation on a repo file**"* and *"scoped to migration,
human-authorized through the review gate, and transactional"*. At HEAD it deletes an arbitrary
**host** file outside the repository at exit 0 (EC-1, driven ×4 by reviewE, re-driven at baseline1
§2a). Stated subject: *a repo file*. That is leg 1 for the door **and** the sink.

**Also declared:** *"An explicit verb taking `path + --as <doctype>` addresses an **arbitrary
foreign file** directly"* (`:39`) — the phrase a door-only fix will be argued against. The two
sentences are compatible (arbitrary *within the repo*), but the Settle should quote both, because
`:39` is the sentence that reads as licensing an outside path.

**The un-rowed half:** baseline1 §4 drove that the exit-4 review hold — *the only human gate on
jigc's only byte-destructive op* — **names no path**: `grep -c "$EXT" hold.txt` → `0`, and the
pinned JSON hold carries exactly `["review","rewrites","source","task"]` (no retire key);
`jigc task validate <id>` previews only the staged-copy advisory. That is in **no EC row**, and
`auto-migration.md:64` specifies the gate's content as the *fidelity diff* only — so naming the
retire target is a surface **addition**, not a violated rule. It needs an explicit scope decision
(leg 0: is the review gate a *declared surface with a hole*?), not silent inclusion under EC-1.

---

## FORKS AND ADVISORIES

### F1 · EC-2's posture set is under-scoped, and one cell touches an invariant
`fork` (scope) + `advisory`

The charter's axis is *HEAD posture*. Baseline2 §2 drove four further cells no fork names:

- **`GIT_DIR=<other repo>/.git` + `milestone create`** → exit **0**; the record file is written into
  repo A's tree and the commit lands **in repo B**. `grep -rn "env_remove\|env_clear" crates/cli/src`
  → no production hit.
- **Unborn HEAD** → `milestone create` exit 0, pinning `base: 4b825dc…` (git's empty-tree hash) into
  the **committed** record; `provision` then fails *"object … is a tree, not a commit"*.
- **Linked worktree on another branch** → `milestone create`/`add-task` write into and commit onto
  the **main checkout** while the operator stands on `feature`. This is design-consistent
  (`crates/cli/src/locate.rs:23-31`: `jigc_home` is *"the **main** checkout the committed doc-store
  and `.jigc/` bind to"*, M31 Inc 2 / WF3) but **unstated at the surface** — the ack says
  `record commit: <sha>` and names neither checkout nor branch.
- **`task finalize --carry-staged` during a merge** → exit 0, **concludes the merge**: HEAD gains two
  parents under jigc's own subject, `MERGE_HEAD` consumed, ack says `1 file committed`.

**The decision owed:** is the subject *HEAD posture* or *repository state*? And specifically —
**is git-env scrubbing (`GIT_DIR`/`GIT_WORK_TREE`) in scope?** That is invariant-adjacent: jigc
integrates with git by shell-out by design (CLAUDE.md → Code architecture), and scrubbing changes
behaviour for legitimate worktree/env users. Naming it out is a decision; leaving it unnamed is not.

### F2 · Fork 2's exemption question is already answered by the baseline — as a flag, not a probe
`advisory`

Fork 2 asks *"whether that exemption is a probe of the worktree or a flag the boundary passes."*
Baseline2 §3 drove it: every fan-out sub-task worktree and the `DedicatedWorktree` both boundary
arms commit in (`crates/cli/src/task.rs:4581`, `git worktree add --detach`) answer
`git -C <wt> symbolic-ref -q HEAD` with **exit 1**. A HEAD probe therefore cannot distinguish the
by-design detachment from a user's, so the exemption **must** be a flag/context the boundary passes.
The Settle should record that as measured rather than re-posing it.

### F3 · EC-10's fix must engage a recorded refusal by name
`fork · foreclosed-by-doc`

`crates/cli/tests/foldback_truth.rs:223` **declines the version assertion by name**: *"The owed
version bump is deliberately outside this fence… it carries no numeral: the roadmap's M50 section
names no target version and the 1.0.0 call is the human's, so asserting a version string would be
this fence choosing it."*

That rationale is sound for the **numeral** and silent on the weaker checkable claim baseline3 §7
names — *the version this fold-back names is the version `Cargo.toml` carries* — which is what has
been missed five consecutive waves. Also measured there: the only mechanical forcing function is the
ten version-bearing goldens (`grep -rl "rc\.14" crates/cli/tests/goldens/` → exactly 10, all
`goldens/compose/composite/start-orient*`), and they force the **regeneration**, never the bump;
`release_smoke.rs:100` derives from the same constant and is green at any version.

**The decision owed:** narrow the recorded refusal (it holds for choosing a numeral, not for
asserting agreement between two homes) — recorded as a narrowing with its rationale engaged, the
repo's standing form — or accept that EC-10 ships as prose again.

### F4 · EC-6's fix could mint the obligation it claims to protect, and its cited basis is a misreading
`fork · cheap-vs-robust`

EC-6 rests on *"a surface `design/measurement.md` calls **an independently-versioned surface** that
carries no version integer."* Read at `design/measurement.md:75`: *"where it and the dogfood hook's
v1 JSONL schema overlap in fields, they are **independently versioned surfaces — no shared format is
implied or required**."* The sense is *independent **of each other***, not *carrying a version*.
`crates/cli/src/invocation_log.rs:379` repeats exactly that sense (*"no format is shared with the
dogfood hook's JSONL schema"*).

The M44 `task_id` deferral (`decisions-pending.md:455`) rests on **rebuildable + purely additive**,
not on an integer, so its basis is untouched by EC-6.

**The decision owed:** the log is opt-in, gitignored (`.jigc/logs/`), rebuildable and consumed by
our own scripts — it fails razor leg 0 as a one-way door. Meanwhile *adding* a version integer
**creates** a pinned surface at the moment the wave is closing the pin. The robust move may be to
**declare it unversioned** (and close the key set) rather than version it. Either way it is a
decision, and the charter's Tier-1 framing (*"cheap now, expensive after the pin"*) presumes the
opposite without argument.

### F5 · EC-7 is corrected by the baseline and unproven; its fix shape has a shipped precedent
`advisory`

EC-7 claims deleting a deny pattern keeps the suite green. Baseline3 §5 read the tree and found
**two independent inline `insta` literals carrying the whole floor** in production source —
`crates/cli/src/adapter.rs:1495-1547` (`claude_code_profile_bytes_are_canonical`) and `:2622-2686`
(`deny_floor_added_then_idempotent`) — plus live evidence of the coupling firing (a gitignored
`.adapter.rs.pending-snap` recording a real pending diff). The residual gap is narrower: an inline
snapshot is re-acceptable via `cargo insta accept`, the same posture as the 634 compose goldens.
Counts also disagree (EC-7 says 20 patterns; baseline3 counts 22).

**No mutation was applied by any reviewer.** Per the check's own closing rule this row is a lead.
If it survives, the fix shape already ships: `adapter.rs:1569`
`the_deny_floor_carries_the_two_human_owned_destroyers` asserts literals against the **loaded**
profile — extend that, in the test, rather than mint a production const (which would be a second
source of truth for the profile, against *one fact, one home*).

### F6 · EC-8's probe-wire half is a recorded deferral whose basis still holds
`advisory`

`design/validation.md:167` — *"the *versioning policy* itself is deferred (below)"* — and `:187`
ties it to OS sandboxing under the **trusted-pack** model (*"the only pack composed is our own;
safely running an untrusted third-party probe is the post-MVP 'not a public API yet' line"*).

Basis check after PB-1 (M49 project packs): `design/multi-pack.md:155` records that cross-pack probe
collision is deferred and *"there is **no probe-program registry across packs**"*, trigger *"a second
pack ships its own `field-types.yaml` (same-id type) or probe"* — **unfired**. So the basis holds and
this half is a deferral to **re-affirm with its citation**, not a hole to fill. Only the **binary**
semver policy is genuinely absent (grep across `design/`, `implementation/`, root docs: no semver
policy for the binary).

### F7 · The versioning policy's *home* is forced by an existing rule, and every guide edit has an adopter cost the charter does not price
`advisory`

The rule already exists in EC-24's own axis: `design/` **never ships**, so anything an adopter must
read lives in `QUICKSTART.md`/`MIGRATING.md`. Those two are `include_str!`'d into the installed
guide — `crates/cli/src/setup.rs:69-70` — so **every Tier-2 guide edit moves `jigc-body-blake3`**,
and an adopter with a locally edited `SKILL.md` meets M48's `adapter-guide.user-modified`
refuse-to-clobber path on their next `jigc setup` (the exact consequence `decisions-pending.md:260`
records for M50's own guide edits).

**Decisions owed:** (a) is the versioning policy adopter-facing (→ shipped guide) or internal
(→ `design/` + `CLAUDE.md`)? (b) are the Tier-2 guide edits batched into **one** hash move, and does
anything tell an adopter it moved?

### F8 · EC-9's CHANGELOG item collides with a second recorded decision
`advisory`

`changelog` is a frozen **placement** doctype homed at root `CHANGELOG.md`
(CLAUDE.md → M38; `design/storage.md` → Placement). A hand-authored root `CHANGELOG.md` therefore
puts a foreign file at jigc's own canonical managed home for a frozen doctype — while self-hosting
is post-1.0 by the human's 2026-07-10 decision (`decisions-pending.md` → *The doctype-completeness
milestone (post-1.0)*, cited in the charter's own exclusions).

**Decision owed:** author it in the doctype's conformant shape, or knowingly foreign — stated either
way, since the file's home is not a free choice in this repo.

### F9 · Two contract rows the Tier-2 batch does not carry
`advisory`

- **A fourth composed producer.** `command-output-contract.md` §1: *"**Three verbs emit this composed
  shape** … `jigc start`, `jigc workflow`, and `jigc migrate`."* Driven: `jigc --format json
  milestone execute <id>` → `{"task": …, "text": …}`. The code-side census already knows
  (`text_json_parity_axis.rs`'s `milestone execute` row says so in its own words); the contract does
  not. Neither `migrate` nor `milestone execute` has a closed-key assertion — all four
  `assert_eq!(keys, ["task","text"])` sites drive `start`/`workflow`.
- **A read verb ships a commit-hook key.** `milestone list-tasks` is `VerbKind::Read`
  (`crates/cli/src/cli.rs:1459`) and emits `hook_output`, whose declaration is scoped to
  landed-commit envelopes, the `rename`/`migrate-corpus` reports and record-only acks.

### F10 · EC-14 is partly falsified; shipping it as written would land a wrong "correction"
`advisory`

Baseline4 §9 re-derived two of EC-14's members and both are wrong as stated:

- `design/surface-contract.md:129`'s *"member-for-member — **eleven**"* is **correct** (11 table rows
  = 11 `ERROR_CODE_REGISTRY` members, read at `invocation_log.rs:191`). The real defect is carried
  item **(h)** — it is *unfenced*, not stale.
- The two *"The window closes here — (M48)"* headings are **presentation, not a lie**: both
  paragraphs state in their own text that the close is keyed to **the 1.0 pin, not a wave name**
  (`command-output-contract.md:446`, quoted above), and each later spend is explicitly declared
  *"The last spend before the pin"*.

The repo's rule — *a stale claim is struck with the datum that falsifies it* — cuts both ways here.

### F11 · Every Tier-1 fix that adds a key owes a declared paragraph, and EC-5's robust arm may be a ~40-key spend
`fork · cheap-vs-robust`

`command-output-contract.md:446` fixes the **form** of a spend: *"every addition is declared here, in
its own paragraph, as it ships."* Four fixes in this wave add keys and none is enumerated in the
charter: `findings` on `task finalize --dry-run` (EC-20's mechanism, baseline3 §6 — the report is
computed then discarded at `crates/cli/src/task.rs:1827`), `relocated` (and possibly `staged`) on
`ConfigAck::Set` (EC-4, baseline3 §3), the four EC-3 declarations, and any log version (EC-6).

EC-5's robust arm — *"close the partition against the clap leaf tree"* — reads either as *state the
rule* (typed result values carry `schema_version`; ad-hoc `json!` envelopes do not) or as *add
`schema_version` to the ~40 envelopes that lack it*, which is ~40 additive keys each owed a
paragraph, on the last wave before the pin. The charter does not say which, and the two differ by an
order of magnitude in cost.

### F12 · The razor's stated refusal class names verbs, schemas and execution shapes — not flags
`advisory`

Both robust arms in scope add a **consent flag** (`--force`-shaped at `setup`; a refusal +
override at the path family). The razor's *"What it refuses by construction"* paragraph refuses
*any new verb, any schema-shape change to a frozen doctype, and any second execution shape*.
A flag is none of those, but a reader could import the class. Worth one sentence at the Settle so a
fixer does not refuse the robust arm on a mis-read razor. Note the precedent runs the other way:
M48 fork 1 explicitly flagged *"add a flag"* as *"precisely the cheap framing"* and demanded an
independent robust-advocate — so a flag here is admissible but owed that treatment.

### F13 · F-9's admission is a fired trigger, not a fresh choice — record it as one
`advisory`

`decisions-pending.md:223` keys F-9 at *"the next wave that opens `doc rename`'s ack or `finalize`'s
pre-commit manifest — **or** a second trial arm observed leaving the adapter to repair a commit."*
This wave opens both (EC-11/EC-22/EC-23 touch the manifest vocabulary; Tier 2 opens the rename ack).
The admission is clean; recording it as a **fired trigger** rather than as a scope call keeps the
ledger's discipline (`:14`, `:5`) intact and keeps F-10's parked trigger legible beside it.

### F14 · Two rows in the baselines are live defects in no EC row, and both are decisions about scope
`advisory`

- **`jigc task discard`'s own ack names no commit** (baseline4 §3, driven: HEAD moves
  `520283b → 1152124` while the success line says only `discarded task …`). Every other committing
  door prints its sha. EC-12 fixes the *guides*; the *door* is unlisted.
- **`jigc config insert-step … <file>` reads any host file into a committable, composed step**
  (baseline1 §2d, driven: absolute path, `../rel.yaml`, and `.git/config` all land at exit 0 in
  `.jigc/config/steps/<stem>.yaml` and then render into `jigc start`'s step text). EC-1's axis names
  `file` as a member of the exempt set, but the ledger's repro is `migrate`-only; whether the fix's
  predicate binds `file`/`from_file` (a *read* escape into the agent's context, not a deletion) is
  exactly fork 1's unstated content.

---

## What I checked and found clean (a clean check stated is a result)

- **`ArgToken::Plain`'s premise, read in full** (`cli.rs:1491-1553`): the declared bound is explicit —
  *"nothing can check `Plain` from the outside"* (`:1545`) — and the doc-comment's own justification
  (*"a path argument is a path the caller means as one, adjudicated by the filesystem and by their own
  doors"*) is a **premise about the doors**, not a claim that no door needs a guard. So fork 1's
  robust arm **narrows** that exemption rather than overturning it; the M50 record's own gloss
  (*"catches forgetting, never mis-answering"*) is quoted correctly in the charter.
- **The additive window is open and its close is correctly keyed** to the 1.0 pin, not to a wave name
  (`command-output-contract.md:446`, `doc-read-surface.md` → *The window closes here*). Nothing in
  this wave has to re-argue the window; it has to spend it in the declared form (F11).
- **`ERROR_CODE_REGISTRY` mirrors `surface-contract.md`'s table accurately today** (11 = 11).
- **The probe-wire deferral's basis holds** (F6).
- **`finalize.no-task`'s target form is declared and correct** — the contract is not missing a form
  for F-5; the door is missing the finding (B7).
