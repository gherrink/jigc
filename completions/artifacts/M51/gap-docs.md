<!-- 2026-09-10 · Opus gap-detector · dimension: docs · HEAD 74627547 · target/release/jigc 1.0.0-rc.14 · no cargo, no repo edits · copied verbatim -->

# M51 gap-detection — dimension: **docs** (`design/` · `implementation/` · CLAUDE.md · MIGRATING · QUICKSTART · VISION · the shipped SKILL.md)

Repo at HEAD `74627547`. Everything marked **driven** below I ran myself against
`target/release/jigc` (`1.0.0-rc.14`) on a `dev/jigc-rig committed-singletons` corpus under
`$SCRATCH/gap-docs`; everything else is a read at HEAD with `file:line`. No repo file edited,
no cargo run. **Nothing here is settled** — this is a gap list for Settle, not a decision.

Ranked: **blocking** first (a fix cannot be planned without settling the doc question), then
**forks** (the three hunts), then **advisory**, then the **record-correction tier** as a strike
table, then **corrections to the inputs I was handed**.

---

## BLOCKING

### B1 · No doc owns "a path-taking argument", and the one doc that speaks to `migrate <path>` warrants the **opposite** (EC-1, fork F1)

Three doc facts, and they do not compose into a rule a build could enforce:

- **`design/auto-migration.md:39`** — the verb's founding rationale, verbatim: *"An explicit verb
  taking `path + --as <doctype>` addresses an **arbitrary foreign file** directly, sidestepping
  location-based classification."* This is the **doc-side warrant for the unbounded token**. Any
  guard at the `migrate` door narrows what this sentence sells as the verb's reason to exist.
  A fixer citing it will conclude the current behaviour is *designed*.
- **`design/auto-migration.md:68`** — *"This is jigc's **first byte-destructive operation on a
  repo file**"*. The scope word is *repo file*; EC-1 deletes a file **outside** the repository.
  So the doc's own scoping sentence is the rule the binary exceeds — but it is a description,
  not a predicate, and nothing derives from it.
- **`design/write-commands.md:127`** — the **destination**-side rule already exists and names its
  predicate: *"a destination inside another repository …, inside git's own directory, or outside
  the repository root is refused at this gate, **before** the transaction"*, `crates/cli/src/trackable.rs`,
  code `write.untrackable-destination`. **There is no source-side sibling anywhere in `design/`.**

**The gap:** the wave must *mint a home* for a source-side path rule. Candidate homes, with the
argument for each: **`write-commands.md`** beside `:127` (the predicate is already this doc's,
and the doc already owns the verb-argument convention at `:102`) — my recommendation for the
*rule*; **`auto-migration.md`** for the *retire's* trust model (see B2), because the retire is
that doc's own mechanism (`:68`, `:70`); **`storage.md` → Placement/home predicates** (`:176-178`)
only if the wave decides the rule is a *home* rule rather than an *argument* rule. Whichever is
picked, `auto-migration.md:39` must be **struck with its falsifying datum**, not silently
narrowed — it is the sentence a later reader will cite back.

**Also blocking, same row:** `design/worked-examples.md:3356-3358` is the **flow 51 acceptance
chapter** and states M50's claim as achieved — *"no caller-supplied token becomes a path
component without the door validating it against the grammar, home or value rule that token's
own family already declares"* over *"**Four** token families"*. EC-1 falsifies the claim and the
count. A locked acceptance chapter asserting a property the binary does not have is exactly the
shape M46's razor calls *stated-and-violated*; it needs a dated correction in the same wave that
fixes the behaviour, and the flow-52 chapter must not restate the claim without the fifth family.

### B2 · `storage.md`'s source-of-truth model has no room for a working-area file that authorizes a permanent deletion (EC-1, the sink half)

`design/storage.md:7`: *"The committed Markdown at type locations is the only source of truth.
Every other artifact — **the staging working area**, the edge index, the file↔state hashes — is
either a rebuildable cache or a transient working copy."* And `:276-290` (*The per-task working
area*) describes it as *"a gitignored scratch dir of working copies"*.

Neither is true of `.jigc/tasks/<id>/source-path`: baseline1 §4 traces it from
`migrate.rs:295 state::persist` → `engine/finalize.rs:400 plan_retirements` → `cli/task.rs:3016
retire` (`fs::remove_file`) with **no re-validation**, and drove the tamper repro (benign mint,
one-line rewrite of the file, `--approve` → exit 0, an arbitrary host file gone while the ack
named `foreign.md`). The same raw token is a **second** authority at `task.rs:1426`/`:1794`,
where it buys a **carryover-gate exemption** for an arbitrary path.

**The gap:** no doc states the trust model of the retire's input. Fork F1's robust arm
(*re-validate at the sink*) has **no doc to amend** — it must add one. Concretely owed:
(a) `storage.md:276` gains a sentence that the working area holds one file that is *authority*,
not a *copy*, and what constrains it; (b) `auto-migration.md:70` (the transaction mechanism
paragraph) gains the re-validation clause; (c) `finalize.md` → *Rollback discipline* row
"4 promote (+ the migration retire)" (`:174`) is the third site and currently describes only
byte-capture, not admissibility.

### B3 · No design doc states **any** HEAD or git-posture rule — and the nearest one is already false (EC-2, fork F2)

- `grep -rn "detached\|on a branch\|branchless" design/ implementation/` returns **no rule** —
  only the fan-out's *"N detached worktrees"* mechanics (`workflow-dialect.md:256`,
  `roadmap.md:715-719`). The posture EC-2 needs a home for has none.
- The natural home is **`design/finalize.md:31`**, *Preflight*: *"**Git in a committable state.**
  No in-progress merge/rebase/bisect. The CLI does *not* require a clean working tree."*
  **That bullet is unimplemented.** `grep -rn "MERGE_HEAD\|REBASE_HEAD\|rebase-apply\|CHERRY_PICK\|BISECT" crates/`
  → **zero hits** (driven).
- **Driven by me, end to end:** on a repo with `MERGE_HEAD` present and `side.txt` staged by the
  merge, `jigc start --workflow single-task` minted at exit 0; `jigc task finalize <id>` blocked
  at exit 3 — but with **`finalize.carried-staged`**, the carryover gate, not a posture check;
  and `jigc task finalize <id> --carry-staged` **concluded the merge at exit 0**:
  `git log -1 --pretty='%h %p %s'` → `dc2b933 ba82872 10559ca chore(ui): tweak the greeting` —
  **two parents**, `MERGE_HEAD` consumed, subject jigc's. The ack read
  `finalized dc2b933 … **0 files committed**` while landing a merge commit.

**The gap, three parts:** (i) `finalize.md:31` is a stated-and-violated rule that must be struck
with this datum or built; (ii) there is no home for a *HEAD-on-a-branch* rule, and F2's
exemption question (*is the fan-out worktree exempted by a probe or by a flag the boundary
passes?*) is a **design** question no doc frames — `finalize.md` → `fan-out` finalize (`:189-206`)
is where it belongs, since `:204` already reasons about the boundary's subject; (iii) the
`--carry-staged`-concludes-a-merge cell is **in no ledger row** and is a capability question
(does a declared carry-over consent cover *concluding someone else's merge*?), not a wording one.

**New, in no ledger row:** the landed ack said **`0 files committed`** for a commit that carried
the carried-over path its own manifest had just listed (`carried-over side.txt`). One screen,
two counts.

### B4 · `command-output-contract.md`'s *"The three surfaces this pins"* is not a list of pinned envelopes — and the doc's membership sentence is falsified by the code's own census (EC-3, EC-5, fork F7)

- **`design/command-output-contract.md:13`** — the heading *"The three surfaces this pins"* is the
  whole pinned-ness declaration: composed output, write-acks, the findings envelope. Baseline3 §1
  drove **all 47 `VERB_KINDS` leaves** speaking `--format json` and found **17 keys named in no
  design document** across `uninstall`, `ingest`, `unmanage`, `rename`, `relocate`, `doc rename`,
  `config get/list/insert-step` and `milestone join`. **Nothing — doc-side or code-side — says
  which envelopes are pinned at all.** `text_json_parity_axis::REGISTRY` classifies
  *fence-ability* (`Tier::Fenced`/`Judgment`), never *pinned-ness*.
- **`command-output-contract.md:19`** states: *"**Three verbs emit this composed shape** …
  `jigc start`, `jigc workflow`, and `jigc migrate`. All three render through the one
  `render::composed` json arm."* The code-side census disagrees **in its own words** —
  `crates/cli/tests/text_json_parity_axis.rs:385-390`, the `["milestone","execute"]` row:
  *"`execute` renders through `render::composed`, not `render::milestone`: **stdout IS the pinned
  `{task, text}` contract**"*. A locked contract doc and a shipped registry contradict each other
  about the membership of a 1.0-pinned surface. `blocking · prior-art-contradiction` — the plan
  must not pick; the registry already settled it and the doc was never updated.
- **`design/doc-read-surface.md:185`** — *"`schema_version`, **a top-level key on every result
  envelope**"*. Driven (baseline3): it rides **7 of 48** arms. Two cells the doc cannot express:
  `task finalize` **landed** carries it and `task finalize --dry-run` does not (one verb, one
  door), and `milestone finalize` landed emits `{committed}` alone against `task finalize`'s
  `{committed, findings, schema_version}`.

**Design-completeness gap for F7:** *declare or delete* is not decidable until the wave says
**what "pinned" means** and **where the list lives**. Options with their doc consequence:
a per-envelope list inside `command-output-contract.md` (then `setup`/`uninstall`/`config`/
`relocate` join a doc that today owns three surfaces — its title and its `:399` "this doc owns
command-output contract v1" scope both move); or a code-side `declared_keys` column on
`text_json_parity_axis::REGISTRY` with the doc naming the registry (the `doctype_map_versions.rs`
shape). **Neither is stated in the charter, and the two produce different waves.**

### B5 · The 1.0 read contract's own **declared conformance witness** states a shape the binary does not emit (EC-13)

`design/doc-read-surface.md:5` names the milestone record as *"Its conformance witness (the
machine-consumed record whose shape pins the contract)"*, and `:57` repeats *"The **milestone
record is the conformance witness**"*. The witness is enumerated at
**`design/team-ready-state.md:195`**: `{ type, slug, fields: { base, status, schema-version },
sections: { tasks: [ { task-id, intent, status }, … ] } }`.

**Driven by me** (`jigc milestone create "Padding Wave"` → `milestone add-task` →
`jigc doc show milestone-record:padding-wave --format json`):
top-level `['fields','item-count','schema-version','sections','slug','type']` (**6**, witness
says 4) and item `['id','intent','status','task-id','workflow']` (**5**, witness says 3).
All four extra keys are correctly declared at `doc-read-surface.md:59/68/69` and at `:88`'s
additive ledger — **only the witness is stale.** The pin's *proof* is the thing that is wrong.

### B6 · `finalize.md` contradicts itself about what the transaction promises, and **neither** statement declares the residue (EC-29, fork F3)

- `design/finalize.md:178` (the config-layer rollback row): *"on failure restore the present ones
  (`update-index --cacheinfo`, **worktree untouched**) and **drop** any the stage newly added"*.
- `design/finalize.md:183`, five lines below the same table: **"Before phase 6, all-or-nothing."**

**The charter's cheap arm rests on a false premise.** F3's cheap side is *"keep `design/finalize.md`'s
declared index-only fidelity and correct the *all-or-nothing* header"* — but `:178` declares the
**restore's mechanism** (*the restore does not touch the worktree*), **not the residue** (*the
worktree keeps the rewritten `.jigc/version`*). Read plainly, `:178` says the worktree was never
modified. So EC-29's grade *"design-declared, not a violated promise"* is the **lenient** reading;
under the plain reading `:178` + `:183` jointly promise all-or-nothing and the residue breaks it.
Settle must adjudicate **which sentence is the promise** before the fork is even answerable, and
whichever way it goes the doc gains a sentence it does not have today: *what the worktree is
allowed to carry across a refused commit*.

### B7 · `assistant-adapter.md:52` states the sentence EC-26 falsifies, and **both shipped guides carry an unscoped `--no-verify` universal the binary breaks** (EC-26 · new)

- **`design/assistant-adapter.md:52`** — *"**`setup` commits its own install, and only its own
  install** (M48). … The pathspec is **enumerated, never a blanket `git add -A`**"*. EC-26 breaks
  the *"and only its own install"* half without breaking the pathspec half: the enumerated paths
  are `CLAUDE.md`, `.claude/settings.json`, `.jigc/.gitignore`, `.jigc/config/` — **exactly where a
  user's pre-existing uncommitted edits live**. The pathspec bounds *paths*, never *authorship*,
  and the doc's sentence conflates the two. F4 (`setup` narrates vs joins the carryover gate)
  cannot be settled without this sentence being re-derived.
- **New, in no EC row — a law-1 lie on the artifact that ships into every adopter repo.**
  `MIGRATING.md:39`: *"**jigc never passes `--no-verify`** — your `pre-commit`/`commit-msg` hooks
  are policy"*. `QUICKSTART.md:160-161` says the same, and `QUICKSTART.md:165` widens it:
  *"**Every jigc verb that commits on your behalf** answers a rejection that way."*
  `crates/cli/src/setup.rs:1709` — `git commit --no-verify -m INSTALL_COMMIT_MESSAGE`.
  The design docs are internally correct and **scoped** (`finalize.md:89` *"The CLI never passes
  `--no-verify`"* is inside `finalize`; `finalize.md:107` and `:250` name setup's exclusion by its
  *"recorded `--no-verify` rationale"*) — the **shipped** guides dropped the scope. And the code's
  own rationale (`setup.rs:1603-1607`: *"the only hook present is the warn-only `pre-commit` setup
  just installed"*) is false for exactly the adopter who already had hooks — the same adopter
  EC-26 swallows work from. These two are one paragraph's worth of fix and should be planned
  together.

### B8 · EC-12's fix is a **qualifier, not a flip** — two locked docs already settle it the other way

- `design/team-ready-state.md:102` settles it: *"a sub-task discard is a **record-only door**
  ([The commit model]), with the family's whole discipline"*, and `:106` states the complement:
  *"**Inert where there is no record to settle.** An ordinary task is named by no record … both
  discard exactly as before, writing no [commit]"*.
- The shipped guides state an **unqualified negative**: `MIGRATING.md:40` *"no commit, the
  committed store exactly as it was"*; `QUICKSTART.md:180` *"no commit is made and the committed
  store is untouched"*.
- `design/surface-contract.md:131-142` carries the **correct** 11-row identity table including
  `task-discard.commit-rejected` → *"`jigc task discard` of a milestone sub-task (M49)"*.

So the ledger's framing (*"it is the tenth committing door"*) is right about the registry and
**wrong as a guide edit**: a build that rewrites the guides to *"`task discard` commits"* ships a
new falsehood for the ordinary-task case the guides' own narrative is about. The owed edit is the
sub-task **qualifier**, sourced from `team-ready-state.md:102/:106`.

### B9 · EC-9's premise is contradicted by a **dated human decision** already in the record

The charter (`completions/artifacts/M51/charter.md:126-128`) says the publishing-floor deferral's
trigger *"**fires by definition** when 1.0.0 ships"*. `implementation/decisions-pending.md:502`
says otherwise, in its own words: *"**DECIDED 2026-07-16 (human): post-v1.** 1.0 ships internal;
the floor's original trigger stands unchanged (*first external adopter, public release, or opening
the repo*) and **fires after the 1.0.0 call, not before it.**"* (restated at `:655`).

**Blocking because it is a fork the wave would otherwise resolve by omission**: either the human
re-opens the 2026-07-16 decision (in which case the record is struck with a datum, per the repo's
rule), or EC-9 is refused with `:502` quoted. Building the floor without adjudicating it
overturns a dated human decision silently.

### B10 · EC-8 has **no home at all**, and any `design/`-only home reaches no adopter

`grep -rn "semver\|SemVer\|1\.x\|major version" design/ implementation/ MIGRATING.md QUICKSTART.md VISION.md CLAUDE.md`
returns nothing about the **binary's** versioning. What exists governs *contracts*, not the
product: `command-output-contract.md:399` (*"This doc owns command-output contract v1 … additive
keys pre-1.0 only; from the 1.0 pin … an explicitly versioned extension"*) and its read-side twin
`doc-read-surface.md:88`, plus the regime map at `doc-read-surface.md:175`.

**And the adopter-reach constraint is structural**, verified at HEAD:
`crates/cli/src/setup.rs:69-70` — `include_str!("../../../QUICKSTART.md")` and
`include_str!("../../../MIGRATING.md")` — and `:95-96` `guide_body()` = preamble + those two,
nothing else. **`design/` never ships.** So a versioning policy written only into a part-doc
satisfies EC-8's letter and none of its purpose. Owed, and un-decided: which of
{a new `design/` part-doc · `command-output-contract.md → Evolution posture` widened ·
`implementation/`} is the internal home, **and** what sentence lands in the guide body. Note the
guide preamble already says *"re-run `jigc setup` after upgrading the binary"* (`setup.rs:79-84`)
— an upgrade instruction with no compatibility statement behind it.

### B11 · Every new finding code the wave mints has **two** doc homes and the charter names neither

- **`design/validation.md` → *Severity inventory* (`:489`)** + the per-wave registration sections
  (*The M45 registrations* `:607`, *The M49 registration* `:620`) — where a code's **default
  severity**, its **intrinsic-or-tunable** class and its **keyed/unkeyed** disposition are stated.
  `:620`'s own words: *"Recorded here so the inventory stays exhaustive about what the engine
  *emits*, not only about what it *keys*."*
- **`design/surface-contract.md:125` → *The error-code namespace*** — the **11-row door-identity
  table** (`:131-142`), for anything that becomes an `Outcome` identity in the invocation log.

The charter's tier list names surfaces to fix and no doc obligations. Concretely at risk, one row
each: a `migrate` source guard code (B1), a HEAD-posture code (B3), the `SLUG_DOORS` OS-ceiling
code (EC-28), a whitespace root-knob code (EC-27), F-5's 22-door unknown-id code, F-11's payload
code, N20's boundary identity. **Which surface each fires against** (task-scope gate · store-scope
report-only · an operational funnel outside `Finding` entirely) is a design decision with a
severity consequence, and `validation.md:19` (*the engine/probe boundary*) plus `:88` (*Scope =
effective state*) are the sections that constrain it. Not one of them is settled.

### B12 · `finalize.md:204` already declares the rule EC-17 needs, and the code violates it (EC-17)

`design/finalize.md:204`, verbatim: *"**The subject is the on-disk path, never the registered
set** — and the *by construction* warrant this paragraph carried until M46 is **withdrawn** …
Membership is the shipped `OwnWorktree` verdict on a path that **exists** — never registration,
and never mere existence either."*

`crates/cli/src/milestone.rs:4348` — `let registered = registered_worktrees(repo_root).unwrap_or_default();`
— and `:4356` `if !registered.iter().any(|w| w == &path) { continue; }`. The teardown subject at
`milestone finalize` **and** `milestone discard` (they share `remove_worktrees`) **is** the
registered set. That is why baseline4 §4 drove door 3 printing nothing at all over the identical
symlink state and door 4 being structurally unreachable.

**The design-completeness gap:** EC-17's stated axis is *`LeftoverShape` × 5 shapes at all four
`DESTROYING_DOORS`*, and baseline4 §9.3 refutes it — **the four doors run two different subject
derivations** (on-disk walk at `provision`/`uninstall`; `git worktree list` at
`discard`/`finalize`). A fix scoped to the registry sweeps half the class. The doc owes a
sentence per door naming **which derivation it uses and why**, and `finalize.md:204`'s universal
must either bind the teardown too or take an explicit carve-out with its reason.

### B13 · `command-output-contract.md:403` declares `no_docs_from`'s meaning; no doc says what it means on a **blocked** join (EC-15)

`:403` declares the key as naming *"every sub-task that staged no doc"*. Baseline4 §4 drove both
blocking classes: on `join.same-doc-clash` the key names two sub-tasks that **both** staged the
clashing doc (verified on disk and through `doc list --task`) with `overlay: {}`, while
`combine.code-collision` computes the overlay truthfully. `render.rs:3841-3843`'s own doc-comment
— *"the text and the wire cannot disagree about who staged nothing"* — is satisfied and both
disagree with the disk.

**Design-incomplete:** no doc states whether the overlay is computed on a blocking path.
`finalize.md` → *`fan-out` finalize* (`:189`) and `team-ready-state.md` → the join are the
candidate homes. The build must otherwise choose between *"populate the overlay on every blocking
path"* and *"declare it absent and derive the summary from the per-sub-task `provenance.json`"* —
a design fork, and the second is what baseline4 recommends because the join already reads that set
to build its groups.

---

## FORKS (the three hunts)

### F-a · `fork · foreclosed-by-doc` — `auto-migration.md:39` forecloses the robust EC-1 fix

Quoted rationale, verbatim: *"the alternatives **cannot reach the target**: a foreign off-canonical
changelog … classifies `Unmanaged` … so extending the `needs-reconcile` arm of `ingest` is
structurally unable to reach it, and `jigc start` has no source-path parameter. An explicit verb
taking `path + --as <doctype>` addresses an **arbitrary foreign file** directly, sidestepping
location-based classification."*

**Does the better path honour the rationale?** Yes — the rationale is about *location-based
classification*, not about *repository membership*. Every target it names (a root `HISTORY.md`,
`docs/changelog/changelog.md`, an in-repo squatter) is **inside the repo**. So
`trackable::untrackable_reason(repo_root, path)` at the door refuses **nothing the rationale
argues for**, and baseline1 §3 confirms the predicate already refuses every destructive cell in
§2a. This is a candidate **revise** (narrow "arbitrary" to "any in-repo path, wherever it sits"),
not a conflict. Flagging it because a fixer who reads `:39` and stops will conclude the
unbounded token is designed. **The human's call, not mine.**

### F-b · `fork · cheap-vs-robust` — the pinned-envelope question is a **one-way door at the pin**, and the cheap read of F7 leaves a known hole

`command-output-contract.md:446` states the rule that prices this: *"**an undeclared key on a
pinned envelope is a defect, not an addition**, whichever wave mints it"*, and *"From that pin
this contract's shapes evolve solely by an **explicitly versioned extension**."*

- **Cheap:** declare the 3–4 keys EC-3 names and state the versioned/unversioned envelope split
  as a sentence. Long-run cost: the other **13** undeclared keys baseline3 §1 found
  (`uninstalled`, `rows`, `summary`, `identity`, `new_path`, `old_path`, `prose_mentions`,
  `referrers`, `moved`, `displaced`, `reslugged`, `layer`, `rejected`, `knobs`, `anchor`, `side`,
  `step`, `overlay`) become, on the day 1.0.0 lands, **either** silently-pinned-by-accident **or**
  a versioned-extension chore — and the doc's own rule says a driver may not be told which.
- **Robust:** name the pinned set (doc-side or as a `declared_keys` column on the registry that
  already bijects the clap tree) so *pinned* is a property with a home, and close the key set per
  envelope. Both halves are cheap **only before the pin**.
- **Vision test:** the vision commits to *"the CLI is the sole channel"* and to a machine contract
  a driver reads; an envelope whose pinned-ness nobody can look up is a hole in a **declared**
  surface, not premature generality. Tag: **the robust path is the minimal-correct one.**

### F-c · `fork · cheap-vs-robust` — EC-14's "hand-written numbers" is being priced as wording, and two of its members are one-way

Most of EC-14 is genuinely tier-2. Two are not:
- **`design/corpus-migration.md:83`** — *"listing all **ten** shipped schemas"*. Driven at HEAD:
  `grep -c "schema-version:"` → dev **6**, methodology **11** = **17 entries**. This is the doc a
  doctype author reads *at the moment of bumping a frozen schema*, and the count is the freeze's
  own scope statement.
- **Both manifest headers** (`crates/cli/pack/config/schema-manifest.yaml:48` and
  `packs/methodology/config/schema-manifest.yaml:79`) carry the identical sentence *"the narrowing
  re-pinned **all 16 doctype hashes**"* — one historical fact restated in two homes, in the two
  files an author edits when the count moves.
The repo already ships the robust mechanism for exactly this
(`crates/cli/tests/doctype_map_versions.rs` fencing `doctype-map.md`'s rows against both
manifests). Choosing *"rewrite the numeral"* over *"fence it or iterate it"* re-buys the same
defect next bump. **Note the historical members must be dated-bracketed, not re-pinned** — the
`foldback_truth.rs::dated_correction_spans` discipline; a fixer told to "fix the counts" will
mechanically re-pin history.

### F-d · `prior-art-reconciled` — contradictions between locked docs, quoted

| # | doc A | doc B / code | the disagreement | who settled it |
|---|---|---|---|---|
| 1 | `command-output-contract.md:19` *"**Three verbs** emit this composed shape"* | `crates/cli/tests/text_json_parity_axis.rs:385-390` *"stdout IS the pinned `{task, text}` contract"* for `["milestone","execute"]` | membership of a 1.0-pinned surface: 3 vs 4 | **the registry** — it names `render::composed` explicitly; the doc was never updated. Not a free choice. |
| 2 | `doc-read-surface.md:5`, `:57` *"the milestone record is the conformance witness"* | `team-ready-state.md:195`'s enumeration | 4 vs **6** top-level keys, 3 vs **5** item keys (**driven**) | **`doc-read-surface.md:59/68/69`** — the keys are correctly declared there; only the witness is stale |
| 3 | `finalize.md:204` *"never registration"* | `milestone.rs:4348/:4356` (registered set) | which set a destroying/tearing-down door's subject is | **`finalize.md:204`** settled it for the boundary; the teardown never joined |
| 4 | `finalize.md:178` *"worktree untouched"* | `finalize.md:183` *"Before phase 6, all-or-nothing"* | what the transaction promises | **unsettled — this is fork F3** |
| 5 | `finalize.md:31` *"No in-progress merge/rebase/bisect"* | zero `MERGE_HEAD`/rebase/bisect probes in `crates/`; `--carry-staged` **concludes** a merge (**driven**) | whether a posture preflight exists | **the binary** — the bullet was never built |
| 6 | `team-ready-state.md:102`/`:106` (sub-task discard commits; ordinary discard does not) | `MIGRATING.md:40`, `QUICKSTART.md:180` (*"no commit"*, unqualified) | whether `task discard` commits | **`team-ready-state.md`** — the guides need the qualifier, not a flip |
| 7 | `MIGRATING.md:39` / `QUICKSTART.md:160-161,165` *"jigc never passes `--no-verify`"* | `setup.rs:1709`; `finalize.md:89/:107/:250` (correctly **scoped** to finalize) | scope of the never-`--no-verify` promise | **`finalize.md:107`** — it names setup's exclusion by its recorded rationale; the guides dropped the scope |
| 8 | `assistant-adapter.md:52` *"setup commits its own install, **and only its own install**"* | EC-26 (driven: an uncommitted `CLAUDE.md` edit lands inside the install commit) | whether a pathspec bounds authorship | **unsettled — this is fork F4**, and the sentence is its premise |
| 9 | `storage.md:7` *"the staging working area … a transient working copy"* | `.jigc/tasks/<id>/source-path` is authority for an irreversible delete (baseline1 §4) | the source-of-truth model's completeness | **unsettled — B2** |
| 10 | `project-setup.md:103-106` *"never touch an existing project's `.gitignore`"* / *"still merge — never clobber"* (the **root** file) | `gitignore.rs:32` whole-file replace of **`.jigc/.gitignore`**; its own doc-comment says *"amended once to the union"* | whether jigc-owned tracked files are amended or replaced | **`project-setup.md`** states the principle at the sibling file; the workbench file has no rule (`storage.md:116` describes it and states no ownership) |
| 11 | `measurement.md:76` *"independently versioned surfaces"* | no version integer exists (`grep -rn "log_version\|LOG_VERSION\|record_version"` → 0) | whether the log is versioned | **unsettled — EC-6**; and `decisions-pending.md:455` defers `task_id` **on the ground that the log is independently evolvable**, i.e. on the same missing carrier |
| 12 | `doc-read-surface.md:175` heading *"five regimes"* / `:177` *"Five independently-governed … regimes"* | the table at `:181-186` — **six** rows | a count contradicted two lines below itself | the table |

---

## ADVISORY (still owed, lower blast radius)

- **A1 · `measurement.md:62`'s record shape is wrong in two fields.** It states
  `{ timestamp, argv, exit_code, **duration**, output_bytes, binary_version, **finding_codes (on
  failure)**, error_code }`. **Driven by me** (knob on, `jigc describe`):
  `['argv','binary_version','duration_ms','error_code','exit_code','finding_codes','output_bytes','timestamp']`
  with `finding_codes: []` **present on success**. Two errors in the one doc paragraph EC-6's
  fence would be written against.
- **A2 · `storage.md:176-178` has three **home** predicates and no **value** predicate** (EC-27).
  The three are `config.untrackable-root`, `config.workbench-root`, `config.unusable-root`, and
  the third's own framing — *"the store could not go on describing its docs there"* — is exactly
  the generalization EC-27 wants (*"a value that reads back as itself"*). The doc has the right
  frame at the wrong scope; a fourth bullet is the shape, and it needs a code (see B11).
- **A3 · `unmanage.rs:290-293` is a test that pins the lie EC-16 names.** It asserts
  `stderr.contains("managed or not")` with the message *"the relocation header must name the
  sweep class"*. The phrase lives in **four** production sites (`config.rs:66` the `--help`
  doc-comment, `:782` the `docs-root` ack, `:878` the `placement-root` ack, `render.rs:3351` the
  `unmanage` ack). Any EC-16 fix reddens that test; the plan must expect it.
- **A4 · `auto-migration.md` never states that the review hold names its retire target.** `:64`
  describes the gate as rendering the fidelity diff and exiting non-zero; baseline1 §4 drove the
  hold naming **no path** (`grep -c "$EXT" hold.txt` → 0) and the pinned JSON carrying
  `["review","rewrites","source","task"]` with no retire key, while `jigc task validate` — M47's
  *"preview what finalize gates on"* — is silent about it. **The only human gate on jigc's only
  byte-destructive op does not name the file it will delete, and no doc says it should.** Owed:
  one sentence at `auto-migration.md:64`, and a `GATE_COVERAGE ▸ Tier::Previewed` consequence
  (`gate_coverage.rs:43-49` says membership is owned by *"the checks `TaskArea::preview_gates`
  actually runs"*).
- **A5 · `doctype-authoring.md:30` still carries a forward-looking *"Reading this before rc.6
  ships"* caveat** (rc.6 = M42, seven waves ago) directly above the transform-kind matrix; and
  `pinning.md:15` still claims `lib.rs` exposes neither the pack registry nor the clap tree (both
  `pub mod` since M45). Both are internal, both are read *at the moment of doing the thing they
  describe*. Carried as (j)/(k) at `decisions-pending.md:144-156`.
- **A6 · `dev-workflow.md:13`** heads *"all four pass"* over five bullets and `:22` says *"the five
  commands above, in order"* — false on the order too (`dev/gate` runs probe · fmt · clippy ·
  build · test; the doc lists fmt · clippy · probe · test · build). Carried as (m). It is the
  doc every build session reads first.
- **A7 · `MIGRATING.md:47` carries a performance bound in a shipped adopter guide** (*"the cost is
  quadratic in the item count"* plus wall-clock figures elsewhere in the section). Unpinnable by
  construction; a decision about whether machine-dependent numbers belong in the artifact that
  ships into every adopter repo, not a fence.
- **A8 · The *"Notation is illustrative"* header — 16 of 28 `design/` docs carry it**
  (`changelog`, `finalize`, `assistant-adapter`, `multi-pack`, `command-catalog`, `self-hosting`,
  `document-type-schema`, `overrides`, `project-setup`, `validation`, `storage`, `reconciliation`,
  `structural-grammar`, `write-commands`, `workflow-dialect`, `worked-examples`). **Per
  `CLAUDE.md` → How we work together it disclaims *notation only*: "a rule, contract or invariant
  stated in the doc's prose is **citable** exactly as in an undisclaimered doc."** So it excuses
  **none** of the drift above — `finalize.md:31/:178/:183/:204`, `storage.md:7`,
  `write-commands.md:127`, `auto-migration.md:39/:68`, `worked-examples.md:3356` are all prose
  rules and all citable. **The gap is that nothing says so at the point of use**: a fixer handed
  "correct `finalize.md:31`" can reach for the header. Notably the three pinned-contract docs
  (`command-output-contract`, `doc-read-surface`, `surface-contract`) carry **no** disclaimer —
  which is right, and which is the asymmetry worth naming in the Settle so the razor's leg 1
  (*"a rule **stated** in a locked artifact"*) does not get argued per row.
- **A9 · Adopter-doc edits, consolidated** (each ships into every adopter repo via
  `setup.rs:69-70,95-96`):
  **must gain** — the ahead-stamp paragraph (EC-24; the behaviour is documented only at
  `design/validation.md:453` and `design/` never ships, while `schema-conformance.schema-version-ahead`
  is a `STORE_EXIT_FLIPS` member that flips an adopter's CI) · the `--no-verify` scope (B7) ·
  the `task discard` sub-task qualifier (B8) · a versioning/compat sentence (B10) ·
  the two missing `migrate-corpus` triage keys `unadopted`/`unfilled`, where **`unadopted`'s
  emptiness is the exit rule** (EC-23; `MIGRATING.md` step 4 names three of the eight envelope
  keys) · `ManifestKind`'s fifth and sixth members (`MIGRATING.md:35` names four; the enum has six
  and **no `::ALL` exists**, so the fix's first act is minting the registry).
  **must lose or be qualified** — `QUICKSTART.md:174-175` *"prints the manifest and stops, changing
  nothing"* (true only on a clean task; `MIGRATING.md:35` states the rule correctly —
  *"It forecasts no green it would refuse"* — so the correct sentence exists in the wrong file) ·
  `QUICKSTART.md`'s presentation of `task validate` and `finalize --dry-run` as one preview surface
  (EC-20) · `MIGRATING.md:39`'s *"All nine committing doors"* against a 10-member registry ·
  the duplicate install stanza (`QUICKSTART.md:19-20` two paths vs `CLAUDE.md:10` a third).
  **One home, one fact** is the repo's own convention and the install stanza breaks it three ways.

---

## The record-correction tier (EC-31…EC-43) — strike sites and the datum to quote

Repo rule: *a stale claim is struck with the datum that falsifies it, never silently rewritten.*

| id | file:line to strike | the datum to quote |
|---|---|---|
| **EC-31** | `completions/artifacts/RC-rc14/trial-record.md:70-74` (bullet 4) **and** `RC-rc14/pre-trial-findings.md:182-185` | **Verified by me from that arm's own evidence.** `evidence/B3-strict/invocations.jsonl`: rec 1 `start` @ `05:26:33Z`; rec 2 `doc show commit:… --task … --format json` @ `05:26:40Z`; **rec 3 `doc show adr:reject-the-newest-sample-when --task record-the-ingest-queue-overflow` @ `05:26:41Z`**. The `find … \| xargs -I{} sh -c 'echo ==={}===; cat {}'` is in `transcript.jsonl` at `05:26:53.490Z` and its tool_result is `is_error: true` — *"This Bash command contains multiple operations. The following part requires approval: xargs …"*. **Denied, and 12 s later.** B3-strict is a fourth VERB-first arm. |
| **EC-32** | `trial-record.md` (the mechanism prose asserting 3 of 3) | B3 read at invocation 3 and resumed at 6 — the record's own trace table shows it. `crates/cli/adapters/claude-code.yaml:7` binds `SessionStart → jigc start`, so record 1 is the harness's hook in all six arms. The adapter `SKILL.md` (tool call 1 in every arm) names `jigc doc show … --task <id>` verbatim — shipped at M48, present in RC-m50, so it cannot explain 1/3→3/3. |
| **EC-33** | `RC-rc14/coverage.md:7` — *"The subject is 18 sources: 13 increments + `d854e25` + 4 audit findings"* | **Driven by me:** `git merge-base --is-ancestor d854e25 f266770` → **exit 0**; `git log -1 d854e25a` → `2026-09-04 fix(cli): the milestone boundary gates the commit docs it commits`. It is an ancestor of the sha the **previous** trial ran, so it is not in the rc.13→rc.14 diff the table names as its subject. |
| **EC-34** | `RC-rc14/coverage.md:24` (row 12) **and** the table's missing row | **Verified by me:** `completions/trial-driver/arms/walk/18-surface-batch.sh:2` declares itself *"protocol.md §5 arm 18 (**M49 Increment 11**, T1–T8 …)"* — row 12 credits it with M50 Increment 12's surfaces. Separately, M50 Increment 10's `pack.resource-missing` is in **no** column of a table titled *every changed surface in exactly one column*; its correct column is **test-fenced** (`pack_resource_miss_axis.rs` exists), so nothing is unfenced. |
| **EC-35** | `RC-rc14/findings-verification.md:129` | The line reads `` `pinned-by:` `crates/cli/tests/…` — **UNPINNED: verified by reading, no suite asserts this.** `` — an **ellipsis where a citation goes**, immediately before its own `UNPINNED:`. Delete the `pinned-by:` clause; keep the `UNPINNED:`. |
| **EC-36** | the blanket rider *"none moves a pinned contract"* in `findings-verification.md` | F-8 is a frozen dev-pack `schema-version` 2→3 **plus a shipped corpus migration for every adopter**; F-3 is a change to the **schema-definition format** (itself frozen v1) plus `doc schema` `contract-version` 6→7; F-11 is **two** repairs the record does not distinguish. Price per **mechanism**, not per contract-touched. |
| **EC-37** | `implementation/decisions-pending.md:563` region — the N20 entry's scope clause | Driven at both knob settings: the same bare `git merge --ff-only <sha>` failure, exit 1, no code, no route, no state clause, `error_code: null`, **byte-identical under `squash: false`**. The entry records it as *"a `squash: true` fan-out whose fast-forward collides with ordinary main-checkout WIP"* — **knob-independent**, and `squash: false` is the setting a Fix round runs under. *(Note: my own read puts N20's entry near `:597`, not `:563` — the charter's line cite should be re-derived before the edit; see the corrections section.)* |
| **EC-38** | no strike — a **confirmation** | N23 re-driven: `jigc validate` → *"no findings — the committed store validates clean"* at exit 0 while `doc list` drops 3 of 4 managed rows; `git ls-files` carries every file, every sha unchanged. **Registration loss, not byte loss** — the classification is right. Its consequence is a **false green on the verb `MIGRATING.md` tells adopters to CI-gate on**. |
| **EC-39** | `RC-rc14/trial-record.md` headline | *"zero data loss, zero corruption, zero regressions, nothing blocking"* outruns the coverage table's **neither** column (rows 4, 10b, 11a, 11b, `d854e25`). Defensible wording: *"no data loss or corruption was observed in the reached trial paths."* The five unreached cells were later driven and all matched contract — **the correction is to the wording, not the result.** |
| **EC-40** | `RC-rc14/trial-record.md:25` (*"**23 arms**"*) vs `RC-rc14/coverage.md:58` (*"**24 arms.** 00 (control, first) · 01–21 … · 22 …"*) | Verified by me at HEAD; the arm directory `completions/trial-driver/arms/walk/` contains `00-…` through `23-…` = **24** scripts. |
| **EC-41** | the bypass-chain sizing clause in `findings-verification.md` | B1's manual commit left `scripts/retention-sweep.sh` and `src/router.ts` staged and uncommitted — **not lost**, and reported by **no jigc surface**. The chain's core claim re-drove and **holds**; *content-keyed, not sha-keyed* is structural (`crates/cli/src/file_state.rs:63-64`). |
| **EC-42** | `RC-rc14/trial-record.md`'s scored table (the bare `fs` column) | RC-m50 printed `fs (DOC/wkbn)`; the reader **still computes the split** — *"1 managed-document read(s), 2 workbench-bookkeeping read(s)"* for B3-strict — and the dropped column is exactly what hid EC-31: B3-strict's one DOC read is the **denied** pipeline. Restore the split column. |
| **EC-43** | no strike — a **disclosure to carry** | The corpus fixture moved between RC-m50 and rc.14 (the PT-D `IngestQueue` change, declared under `protocol.md` §2.2); **no arm isolates it.** The one legitimate soft spot in the 1/3→3/3 comparison, and the record already names it. |

**A doc obligation the tier does not carry:** these are trial-record edits, but `CLAUDE.md`'s M50
paragraph and `implementation/roadmap.md` → Milestone 50 restate several of the same claims. Any
strike must be checked against `crates/cli/tests/foldback_truth.rs` (it fences the `**M50 —` span
of CLAUDE.md, the MIGRATING back-out ladder's numbering, and per-prose-unit token presence in both
shipped guides) — the suite will redden on adopter-guide edits and on the CLAUDE.md span, by
design.

---

## Corrections to the inputs I was handed (each with its falsifying datum)

1. **EC-3 says four keys are undeclared; `hook_committed` is declared.**
   `design/assistant-adapter.md:56`: *"`--format json` carries the same fact as `hook_committed`."*
   `grep -rn "guide_file\|install_commit" design/ implementation/` → no declaration.
   **The corrected shape is sharper, not softer:** three keys are undeclared, one is declared **in
   the wrong home**, and that is the real gap — declarations scatter because no doc says which
   envelopes are pinned (B4).
2. **EC-9's *"fires by definition"* is contradicted by a dated human decision.**
   `implementation/decisions-pending.md:502`: *"**DECIDED 2026-07-16 (human): post-v1.** 1.0 ships
   internal … fires **after** the 1.0.0 call, not before it."* (B9.)
3. **EC-14's citation for the *ten* count is wrong.** The stale sentence is at
   `design/corpus-migration.md:83` (*"listing all **ten** shipped schemas"*), not `:281` — `:281`
   is the *"Hashed, not refused by name"* bullet about schema shadows. The count itself is
   confirmed stale: dev **6** + methodology **11** = **17** manifest entries (driven by grep).
4. **EC-14 lists `surface-contract.md:129`'s *"eleven"* as stale; it is correct.**
   `ERROR_CODE_REGISTRY` has 11 members and the table at `:131-142` has 11 rows. The defect there
   is that it is **unfenced** (carried defect (h)), not that it is wrong.
5. **EC-14's *"both posture homes headed 'The window closes here — (M48)' above two later declared
   spends"* reads as a lie and is not.** `command-output-contract.md:446` and
   `doc-read-surface.md:90` both state **in their own text** that the close is keyed to the **1.0
   pin, not a wave name**, and each later spend is explicitly declared. Presentation, not law 1 —
   which matters, because "fix the heading" and "the doc lied" are different edits.
6. **EC-17's axis is stated as four `DESTROYING_DOORS`; the fourth cell cannot exist and the
   other three do not share a subject.** `crates/cli/src/milestone.rs:4348/:4356` filters on the
   **registered** set, so `milestone finalize` and `milestone discard` `continue` past a symlink
   leftover. Two subject derivations, not one axis (B12).
7. **EC-29's *"design-declared"* grade is the lenient reading.** `finalize.md:178`'s *"worktree
   untouched"* is a clause about the **restore's mechanism**, not a declaration that the worktree
   keeps a rewritten `.jigc/version`. F3's cheap arm assumes the stronger reading (B6).
8. **EC-12 as written would produce a false guide edit.** `team-ready-state.md:102`/`:106` already
   settle it: the **sub-task** discard commits; the ordinary one does not (B8).
9. **EC-20's framing needs one correction.** Driven (baseline3 §6, one task one moment):
   `task validate` and the **landed** `task finalize` emit the *identical* two findings, so
   `command-output-contract.md`'s *"same check, same severity"* is **not** falsified by the
   committing door. What diverges is **`--dry-run`**, whose envelope has **no `findings` key at
   all** (`task.rs:1827`). The law-1 problem is QUICKSTART presenting the two as one surface.
10. **The charter's N20 line cite (`decisions-pending.md:563`) does not resolve to the N20 entry
    at HEAD** — `:563` is the `adr.cites-code` migration row; `:597` is a bootstrap-facts row.
    Re-derive the line before editing.

---

## New, in no ledger row (found while checking the docs)

- **N-a · Two shipped adopter guides carry an unscoped `--no-verify` universal the binary breaks**
  (`MIGRATING.md:39`, `QUICKSTART.md:160-161`, widened at `:165` to *"Every jigc verb that commits
  on your behalf"*) vs `setup.rs:1709`. The code's own rationale (`setup.rs:1603-1607`) rests on
  *"the only hook present is the warn-only `pre-commit` setup just installed"* — false for any
  adopter who already had hooks. (B7.)
- **N-b · A landed finalize that concludes a merge reports `0 files committed`** while its own
  pre-commit manifest printed `carried-over side.txt` (driven, B3). One screen, two counts.
- **N-c · `design/measurement.md:62` states the invocation-log record with the wrong field name
  and the wrong presence rule** (`duration` vs `duration_ms`; *"finding_codes (on failure)"* vs
  always present). Driven. (A1.)

---

## Bounds of this report

- I drove: the merge/`--carry-staged` cell; the milestone-record witness key sets; three `--help`
  texts (`validate`, `migrate-corpus`, `doc show`); the invocation-log record shape; the
  `MERGE_HEAD`/rebase/bisect grep; the manifest entry counts; `git merge-base --is-ancestor` for
  EC-33; and B3-strict's `invocations.jsonl`/`transcript.jsonl` for EC-31. Everything else is a
  read at HEAD or is attributed to a named baseline ledger.
- **I did not drive** EC-1's deletion, EC-2's three doors, EC-17's symlink cells, EC-15's join
  classes, EC-26 or EC-27 — those are baseline1/2/3/4's repros, quoted as leads with their
  citations, per the check's own closing rule.
- **No cargo was run**, so every claim about what a *test* asserts is from reading the test body
  at HEAD (`unmanage.rs:290-293`, `text_json_parity_axis.rs:385-390`).
- I did not audit `design/`'s remaining docs (`architecture-documentation`, `bootstrap`,
  `changelog`, `design-altitude-doctypes`, `introspection`, `methodology-docs`, `multi-pack`,
  `reconciliation`, `self-hosting`, `structural-grammar`) for tier-2 drift; the sweep above was
  scoped to the docs the wave's tiers touch.
