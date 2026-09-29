# M51 — the acceptance design

**What this is.** The two things the `acceptance-spiked` gate cell halts on: **flow 52's arm set**
(§17 rules that M51 ships a `design/worked-examples.md` chapter and `crates/cli/tests/flow52_acceptance.rs`,
and states in the same sentence that the arm set is owed at decompose) and **the eight per-axis review
matrices** (D15 + §17: an axis's cell matrix is *a written table of `(door, cell)` rows whose cells are
driven*, and a verb is covered iff it is the door of **≥1 driven row**). Nothing here is driven — the drives
happen on the built and installed `1.0.0-rc.15`, **after** the audit's fixes land (D15, EC-10). Every set
named below either exists at `HEAD = 74627547` (cited by symbol and file) or is minted by a named decision.

Sources: [settle-record.md](settle-record.md) (D1–D15, amendments §1–§17), [charter.md](charter.md) → Acceptance,
[envelope-key-census.md](envelope-key-census.md), `crates/cli/src/cli.rs` → `VERB_KINDS` (47 leaves, line 1412),
and the mold: `design/worked-examples.md` → flow 51 + `crates/cli/tests/flow51_acceptance.rs`.

**Increment numbers are provisional.** Decompose has not run; each arm is keyed to the **decision** whose
increment it lands with, with a risk-first expected index in parentheses.

---

## Part 1 — flow 52: the arm set

**The mold, unchanged since M40:** one arm per new registry or class the wave mints; each arm **names the
kind of set it iterates** — a code-side registry · the class's defining case-set matched exhaustively · a
derivation stated as one · a manufactured shape space **with the reason it is manufactured** — and each
states what it adds over the axis suite beside it, because an arm that re-runs a shipped axis proves the
axis twice and the wave once.

**The wave's claim the flow proves** (settle → The claim, with §3's symmetric bound): *no caller-supplied
token and no repository posture reaches a door that destroys, commits or moves without that door having
adjudicated it — the two M50 exemptions are closed as classes — and every surface 1.0.0 pins says what the
binary does.* Two honest bounds ride the chapter's prose, both already on the record: the `Plain` family
ships as a **classified registry with one stated rule or one stated no-rule-and-why per member**, and the
posture family closes over **three of four** driven members with the `GIT_DIR` redirect declared out.

| # | the set it iterates | kind of set | the done-picture it proves through the real binary | lands with |
|---|---|---|---|---|
| 1 | the **occurrence-keyed path-argument registry** (D1 §2 — keyed by `(leaf, argument id, conditional arm)` over the six `ArgToken::Plain` members `path`·`file`·`from_file`·`from`·`target`·`value`, ⇔-fenced against the clap tree; `ARG_TOKENS`, `cli.rs:1552`, is its parent classification) | **code-side registry** (minted by D1) | one traversal, one absolute outside-repo path, one symlink escape, one `.git/`-component and one untracked in-repo source go to **every occurrence** in one repository; each refuses with its own code (`migrate.source-untrackable` · `migrate.source-untracked` · `config.step-source-untrackable`) or, for a stated no-rule row, is driven to show the token becomes **no path component**; afterwards `HEAD` is unmoved, `git ls-files` is byte-identical, and the canary planted outside the repository reached **no stream**. *Adds over the per-door axis suite:* the composite cost, asserted once, across all occurrences | D1 (Inc 1) |
| 2 | the **posture family × the commit-on-behalf class** of D2's total leaf classification (§3 — every clap leaf classified commit-on-behalf / move-on-behalf / neither, `COMMITTING_DOORS ⊆ commit-on-behalf` asserted; `COMMITTING_DOORS`, `invocation_log.rs:130`) | **total classification** on the `VERB_KINDS` mold (minted by D2), crossed with the posture family's **defining case-set** (detached · unborn · operation-in-progress) | on a detached HEAD, an unborn HEAD and a live `MERGE_HEAD`, every commit-on-behalf door refuses with `repo.head-detached` / `repo.head-unborn` / `repo.operation-in-progress` and a `Human` route **naming the git command**; movers refuse operation-in-progress only; `jigc setup` on an unborn HEAD **still installs** (the stated `Exempt(reason)` row); a fan-out worktree — typed `DedicatedWorktree`, never sniffed — finalizes clean; and `--carry-staged` no longer concludes a user's merge | D2 (Inc 2) |
| 3 | **`setup`'s own install pathspec** — the eight paths it writes — asked **worktree-vs-HEAD, per path, before any write** (§1) | a **derivation stated as one** (the pathspec is the subject; the predicate is a git query, not a registry) | a repo whose `CLAUDE.md` carries an edit that was **never staged** (§1's driven red): `jigc setup` refuses its **commit** with `CarryoverBoundary::Setup`, the files stay **written and staged**, the route names `--force` as the single consent, and the user's bytes are still on disk; a fresh install, a re-run and an upgrade all pass clean; an unrelated staged `feature.txt` is untouched | D3 (Inc 3) |
| 4 | the **config-layer CAS pre-image** — `{stage failure, hook rejection} × {unchanged since jigc's post-write image, concurrently edited}` over the two named files `.jigc/.gitignore` and `.jigc/version` (§6, A1) | **manufactured shape space, and it says so**: the failure points are decided, not enumerable from any registry, and the second axis is a *race*, which no code-side set carries | a user's uncommitted private line in `.jigc/.gitignore` + a rejecting `pre-commit` → the transaction restores the **pre-image**, `git status` is *not* falsely clean, and the line is still there (flow 51's base-red is the inverse: the bytes existed in no git object); in the concurrently-edited cell the restore **does not overwrite** — a rollback-conflict finding is emitted and **both** versions survive; and `gitignore::ensure` is byte-idempotent across its four callers, its ack naming the content change | D4 (Inc 4) |
| 5 | the **`EnvelopeArm` registry** with its **four proofs** (§7 — 60 arms over 47 leaves, part-derived from `OrientationView`/`DocAck`/`TaskAck`/`ConfigAck`, 11 rows hand-enumerated with stated reasons; hosted at `format_json_success_axis.rs`) | **code-side registry, production-side**, with a derivation for part of it and stated reasons for the rest | every clap leaf has ≥1 arm; every production arm has exactly one row; every row is **driven**; the driven key set **equals** the declared key set — and the four deletes are gone from the wire (`installed`, `uninstalled`, `review`, `hook_output` on `milestone list-tasks`), the 7 `Unpinned(<reason>)` arms are the composed-prose ones, `ConfigAck::Set` carries `relocated`, `task finalize --dry-run` carries `findings` under the **`validate == --dry-run == the committing door`** equal-set fence, and `doc list` renders the declared third state `orphaned` with `item-count: null` | D5 (Inc 5) |
| 6 | the **22 unknown-id doors**, read off **`WORK_UNIT_ID_DOORS`** (`cli.rs:1738`, 26 rows / 25 doors, ⇔-fenced against the clap tree) filtered to the unknown-id cell | **code-side registry** (existing at HEAD), filtered to one cell of its own token axis | an id that is well-formed but names nothing is handed to every door: each answers the **findings envelope** carrying `finalize.no-task` with its route — not a flattened `{"error": …}` — so a driver keying on the stable `(code, target)` pair gets an answer at **all 22**; M50's malformed-id and empty-id columns stay green beside it, and a task minted after the sweep still finalizes to a real commit | D7 (Inc 6) |
| 7 | **`ManifestKind::ALL`** (`render.rs:1717` gains `ALL`) and the fenced-count family — the `migrate-corpus` triage keys by exhaustive destructure, `COMMITTING_DOORS`' count and `ERROR_CODE_REGISTRY`'s doc mirror on the `doctype_map_versions` mold | the **class's defining case-set, matched exhaustively** (two manifests; the compiler is the fence) | each manifest kind is driven to the surface that names it, and the numeral every prose home states is the numeral the registry carries — a count that moves reddens `foldback_truth` per prose unit instead of being discovered a wave later; the historical counts stay **dated-bracketed**, not re-pinned | D8 (+D9's `Record` key-set fence) (Inc 7) |
| 8 | the **derived ambush owe-set** (§4 — *blocking codes minted by a door in the commit-on-behalf class*, **minus** rows carrying a stated `Exempt(<reason>)`; replaces `AMBUSH_CLASS_CODES`, `pack.rs:779`) | a **derivation stated as one**, with a stated exclusion rule | a fixture pack under `JIGC_PACK_DIR` whose steps drop the required statement reddens **pack-load at every door** with the owe-set's own message; D3's setup-side carryover code is the **first exempt row**, with its reason (*no pack step of either pack solicits `jigc setup`*), and pack-load stays green with it absent — which is the cell a hand-list could not express | D10 (Inc 8) |
| 9 | the **D12 orphan arm over `STORE_EXIT_FLIPS`** (`render.rs:909` — the orphan code joins as its **sixth** member; §9) | **code-side registry** (membership is the assertion) | a doctype leaves the resolved set: `jigc validate` **exits non-zero** naming each orphaned instance with a `Human` route (re-add the pack, or `jigc unmanage`), `jigc doc list` **prints the row** in state `orphaned` with the stamp's identity and `item-count: null`, and the D10 sibling cause — an unstamped managed corpus — answers under its own code at the same surface, the partition named. Red at the base: *"no findings — the committed store validates clean"* at exit 0 while `git ls-files` still carried every file | D12 (Inc 8) |

**Deliberately unrepresented, recorded as a decision** (the M46 Inc 9 / M48 Inc 11 / M49 Inc 12 precedent —
*an increment that mints no verb, finding or route carries nothing for a done-picture walk to reach, and
manufacturing an arm would be a walk written to have an arm rather than to prove a claim*):

- **D6 — the release-versioning policy** (two prose homes plus the guide body) and **§13's batched adopter-doc
  hash move**: doc-only. The *behaviour* they describe is proven by arms 5 and 9.
- **D11 — the version-stamp fence** and **D8's `foldback_truth` re-keying**: build-time fences over this
  repo's own prose. §15 records D11 as admitted by the human's boundary and **refused by razor leg 2** — it
  benefits our process only, which is precisely why no adopter-facing walk can reach it.
- **D9's invocation-log key-set fence**: the log is a gitignored measurement artifact, not a surface; its
  closure is an exhaustive destructure over `Record`, asserted in its own suite and ridden by arm 7.
- **EC-7's floor-scraper repair** (`e2e_audit::floor_patterns()`'s comment truncation): a test-harness fix;
  the floor's *behaviour* already has flow 51 arm 7.
- **The Tier-2 law-1 wording batch and the Tier-4 record corrections**: surfaces whose only change is what
  they *say*; each is keyed to its findings-verification row, and none mints a verb, code or route.
- **The close increment** (goldens, the conversion and disposition ledgers, flow 52 itself).

Two in-scope rows ride existing arms rather than earning their own, so they are not lost: **EC-28**'s
name-ceiling code at the three `SLUG_DOORS` rides **arm 1** (a `SLUG_DOORS` occurrence is a registry row),
and **N15**'s `--task` arm of `store.not-found` rides **arm 5** (it ships as a D5 row on the pinned read
surface).

---

## Part 2 — the eight per-axis review matrices

**Row schema, common to all eight:** `(door, cell) → {argv driven, exit, code|none, route kind, surface
asserted, verdict}`. A row is **driven** iff its argv ran on the installed `1.0.0-rc.15` and its verdict was
recorded with a repro block. **A verb is covered iff it is the door of ≥1 driven row.** Classification-only
rows (a leaf classified *neither*, a registry row proven by a ⇔ fence rather than by driving) are **not**
driven rows and confer **no** coverage — otherwise the fence measures a table instead of the binary.

| axis | door set (registry it is derived from) | cell set |
|---|---|---|
| **1 · caller tokens** | the D1 **occurrence-keyed path-argument registry** ∪ `DOCTYPE_DOORS ▸ DoctypeArg::Address` (10) ∪ `SLUG_DOORS` (6) ∪ `WORK_UNIT_ID_DOORS` (26 rows) — all four ⇔-fenced against the clap tree from `ARG_TOKENS` | absolute · `../` escape · symlink escape · `.git/` component · workbench root · untracked in-repo · leading-colon pathspec magic · the `-` stdin sentinel · OS name ceiling · well-formed control |
| **2 · posture** | D2's **commit-on-behalf** and **move-on-behalf** classes (`COMMITTING_DOORS ⊆ commit-on-behalf`); the *neither* class is fenced, not driven | `{HEAD detached · HEAD unborn · merge in progress · rebase in progress · bisect in progress · dedicated worktree (typed) · GIT_DIR redirect (declared out — recorded as a stated row, with its reopening condition)}` |
| **3 · destroying doors** | `DESTROYING_DOORS` (`milestone.rs:2514`, 4) ∪ `jigc task discard` (M50's staged-prose guard, shared home) | `LeftoverShape::{Directory, File, Unreadable}` × `{staged prose · untracked workbench file · clean}` × `{no --force, --force}` |
| **4 · transaction/rollback** | `COMMITTING_DOORS` (10 rows / 9 verbs) ∪ `setup` ∪ `milestone join` ∪ `milestone provision` (a `gitignore::ensure` caller that never commits — §6/A1) | failure points `{stage failure · hook rejection · retire-untrackable refusal inside the closure · empty commit · promote after retire}` × `{worktree unchanged, worktree concurrently edited}` |
| **5 · pinned contracts** | the **`EnvelopeArm` registry** — 60 arms over all 47 leaves, with the four proofs | `{declared key set == driven key set · Unpinned(<reason>) · the four deletes absent · the reject funnels (`error` vs findings envelope) · exit code}` |
| **6 · composed surfaces** | the four `render::composed` producers (`start`, `workflow`, `migrate`, `milestone execute`) ∪ `describe` ∪ the read verbs the read-back fence's owe-set names (`doc show`, `doc list`, `task validate`) | `{the step text names a verb that answers · the resume: line's three real states · orientation's three states · the catalog one-liner matches the verb's behaviour · the off-catalog reason is stated}` |
| **7 · freeze & migration** | the freeze/migration surfaces: `describe`, `doc schema`, `validate`, `doc list`, `migrate-corpus`, `migrate`, `ingest`, `unmanage` | `SchemaChangeKind::ALL × LOCI` ∪ `ManifestKind::ALL` × `{hash moved without a version bump · schema-version ahead · unstamped managed · orphaned doctype · unadopted foreign · project-layer shadow}` |
| **8 · adopter docs & help** | the **doc-sentence batch** — derived at build from M51's Tier-2/Tier-4 edit set + the three EC-11 generated help texts + the two `include_str!`'d guides; its doors are the verbs those sentences name | `{the sentence's claim driven at the verb · the generated help text equals the registry that generates it · the guide sentence is true after install}` |

### Coverage — all 47 `VERB_KINDS` leaves

**Honest statement about the fence first.** Axis 5's registry drives **every** leaf to a success by proof (1)
and proof (3), so *"every leaf appears in ≥1 axis matrix"* is satisfied by axis 5 **alone**. The fence as
§17 writes it is therefore true but weak, and the table below reports what it cannot: **which axes beyond
axis 5 reach each leaf**, and marks a leaf reached by axis 5 only as `only-5(<reason>)` — the honest
analogue of `uncovered(<reason>)`, with the same reason obligation. `uncovered(<reason>)` is empty by
construction, and that is stated rather than presented as a result.

| # | leaf | axes | note |
|---|---|---|---|
| 1 | `start` | 1, 5, 6, 8 | slug + work-unit-id doors; orientation's three states |
| 2 | `workflow` | 1, 5, 6 | |
| 3 | `setup` | 2, 4, 5, 8 | D3's refusal; unborn `Exempt(reason)` |
| 4 | `uninstall` | 3, 5, 8 | |
| 5 | `upgrade` | 5, 8 | a read sweep; its only token is the guide it checks |
| 6 | `ingest` | 5, 7 | the identity leg is about bytes found on disk, not a caller token |
| 7 | `migrate` | 1, 5, 6, 7, 8 | EC-1's headline door |
| 8 | `migrate-corpus` | 2, 4, 5, 7, 8 | committing door **and** mover (relocation arm) |
| 9 | `unmanage` | 1, 5, 7 | `path`; D12's route target |
| 10 | `rename` | 1, 2, 4, 5 | committing door and mover |
| 11 | `relocate` | 1, 2, 5 | `from`; mover — operation-in-progress only |
| 12 | `describe` | 5, 6, 7 | the catalog menu, and the door pack-load fences surface at |
| 13 | `validate` | 5, 7, 8 | `STORE_EXIT_FLIPS`; EC-11's probe-family help text |
| 14 | `doc create` | 1, 5 | |
| 15 | `doc add-item` | 1, 5 | address **and** slug door |
| 16 | `doc remove-item` | 1, 5 | |
| 17 | `doc retitle-item` | 1, 5 | |
| 18 | `doc rename` | 1, 5, 8 | F-9's ack sentence |
| 19 | `doc set-field` | 1, 5 | |
| 20 | `doc set-slot` | 1, 5 | address + `from_file` occurrence |
| 21 | `doc author` | 1, 5 | `from_file`; F-11's payload parse |
| 22 | `doc show` | 1, 5, 6, 8 | N15's `--task` arm; EC-11's key-const help text |
| 23 | `doc schema` | 5, 7 | the projection the freeze governs |
| 24 | `doc list` | 1, 5, 6, 7 | the `orphaned` third state |
| 25 | `task list` | 5 | `only-5(no path, id, posture or destruction reaches it — its whole contract is its envelope, and it is the census's single top-level-array anomaly, §4.1)` |
| 26 | `task diff` | 1, 5 | |
| 27 | `task validate` | 1, 5, 6 | the previewed-tier equal-set fence |
| 28 | `task discard` | 1, 2, 3, 4, 5, 8 | the tenth committing door; silent-sha ack |
| 29 | `task finalize` | 1, 2, 4, 5, 8 | hosts D1's sink (`finalize.retire-untrackable`) |
| 30 | `task bind` | 1, 5 | address door |
| 31 | `config set` | 1, 2, 5, 8 | `value` × `ROOT_KNOBS`; mover |
| 32 | `config insert-step` | 1, 5 | `file` — the **source** rule (§2) |
| 33 | `config replace-step` | 1, 5 | `file` + `target` occurrences |
| 34 | `config remove-step` | 1, 5 | `target` — a stated **no-rule-and-why** row, driven to show no path is created |
| 35 | `config fill` | 1, 5 | `target` + `from_file` |
| 36 | `config fork` | 1, 5 | `target`; its ack emits a `path` |
| 37 | `config get` | 5 | `only-5(a read of the cascade; its argument is a knob name from the knob vocabulary, never a path — and the knob value rule is driven at `config set`)` |
| 38 | `config list` | 5 | `only-5(takes no argument at all; no posture, no write, no destruction — its contract is its envelope)` |
| 39 | `milestone create` | 2, 4, 5 | mints its id (no resolve door); posture bricked its record at the baseline |
| 40 | `milestone add-task` | 1, 2, 4, 5 | |
| 41 | `milestone add-from-spec` | 1, 2, 4, 5 | the M50 audit HIGH's door |
| 42 | `milestone list-tasks` | 1, 5 | axis 5 carries the `hook_output` delete |
| 43 | `milestone provision` | 1, 3, 4, 5, 8 | destroying door; non-committing `gitignore::ensure` caller |
| 44 | `milestone execute` | 1, 5, 6, 8 | the fourth composed producer |
| 45 | `milestone join` | 1, 2, 4, 5, 8 | `overlay_docs_commit_and_ff` is a posture seam |
| 46 | `milestone finalize` | 1, 2, 3, 4, 5, 8 | destroys on the ordinary success path |
| 47 | `milestone discard` | 1, 2, 3, 4, 5, 8 | |

**`uncovered(<reason>)`: none.** Three leaves are `only-5`, each with its reason above. If a leaf is added
before the review runs, it inherits axis 5 automatically (the registry is ⇔-fenced against the clap tree)
and must be given a reason here or an axis row — the same obligation, stated the same way.

### Staffing: the Opus driver and the Codex source pass

Each axis is staffed as **one Opus driver + one Codex source pass** (D15). They answer different questions
and must not be collapsed:

- **The Opus driver** owns the `(door, cell)` table: it drives every row on the installed `1.0.0-rc.15` in
  throwaway repos (`dev/jigc-rig <state>`, never a variable-path `rm -rf`), and records **exit, code, route,
  surface and a repro block** per row. It may not mark a row driven from a source read.
- **The Codex source pass** owns **completeness of the row set** — the question driving cannot answer: *is
  there a door the registry does not carry, or a bypass of the seam the axis is about?* It reads, per axis:
  **1** `ARG_TOKENS` + the path-argument registry + every consumer of its argument ids, `trackable.rs`,
  `config.rs::unusable_root_reason`, the `retire` path · **2** the posture registry + every
  `Command::new("git")` site whose subcommand is `commit`, `mv`, `merge` or `rm` · **3** `milestone.rs`'s
  destroying doors, the leftover classifier, every `remove_dir_all`/`remove_file` site · **4** every
  pre-image capture and restore site, the commit closures, all four `gitignore::ensure` callers · **5** every
  serialization that reaches stdout, hunting an envelope the registry does not carry · **6**
  `render::composed`'s callers, the pack step bodies, the `describe` projection · **7** `pack.rs`'s freeze
  and manifest resolution, `schema_diff`, `migrate-corpus`, the store sweep · **8** the `include_str!` guide
  seam and every `about`/`long_about` string against the behaviour it names.

**The reconciliation rule.** *A claim by one that the other cannot reproduce is a **lead, not a finding**.*
A Codex claim with no driven repro enters the axis table as `lead(codex, <claim>)` and is either **driven to
a repro block** — at which point it is a finding — or **recorded refuted with its falsifying datum**. An
Opus row that the source pass says cannot happen stays a finding (it was driven), and the source claim is
recorded refuted with the repro that refutes it. **No fix ships on a source read alone, and no completeness
claim ships on driving alone** — the standing rule from M46's planning corrections (*an agent's report is a
lead, not a measurement*) and M50's two planning halts (*a claim about how the composed product behaves is
not established by reading the files it is composed from*).
