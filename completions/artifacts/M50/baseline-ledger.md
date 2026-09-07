# M50 — the baseline ledger

**Verified at `HEAD = d9e91f1`** (branch `main`, clean tree), driven against the **release** binary
`target/release/jigc` (`1.0.0-rc.13`, rebuilt at HEAD before probing; the host `~/.local/bin/jigc`
was reinstalled from the same tree, closing the handover's stale-binary trap). Fixtures from
`dev/jigc-rig <state> --binary target/release/jigc`. Debug posture was used nowhere.

**Provenance, per the `claim-driven` gate.** Two facts were driven by the **orchestrator**
personally: W-13 (`jigc task discard ""`) and the repository-destruction repro
(`jigc task discard "../.."`). **Everything else in this file is relayed from five
`capability-auditor` subagents that drove it**, each returning the command and its output. Relayed
is not the same as verified by me; the rows the Settle turns on are re-driven at Settle, and the
gate-record records which.

**A map, not gospel.** Verified at this sha; the gap-detectors still spike the specific new shapes.

---

## 0 · The headline — Tier 0 is a different, larger class than chartered

The charter's Tier 0 is *"the empty-id class over its one seam."* All three nouns are wrong.

### 0.1 It is not the empty-id axis — it is the unvalidated-path axis, and it destroys repositories

The id is joined as a raw path component with **no validation of any kind**. `""` is the mildest
member. Driven by the orchestrator, from a fresh rig, release binary:

```
$ jigc task discard "../.."
discarded task ../.. — dropped staged edits to: decisions-log, roadmap
$ echo $?
0
$ ls -a "$REPO"
.  ..
```

25 files, 4 commits, `.git` included — gone, from one CLI call, at exit 0, behind a success ack.
Nothing recovers it. Relayed siblings, each driven by the auditor:

| id | resolves to | outcome |
|---|---|---|
| `""` | `.jigc/tasks/` | every task + all staged prose destroyed, exit 0 |
| `..` | `.jigc/` | the whole workbench destroyed, exit 0 |
| `../..` | the repo | **total repository destruction incl. `.git`**, exit 0 |
| an absolute path | anywhere | an arbitrary directory outside the repo removed, exit 0 (planted canary confirmed) |
| `.` | `.jigc/tasks/` | `task validate` exit 0 "validates clean" |
| `/`, `/etc` | those paths | `task validate` exit 0 "validates clean" |
| `../../` (validate) | the repo root | exit 3, parsed `docs/*.md` as staged docs |
| `<task>/`, `../tasks/<task>` | the task | accepted under an **aliased identity** |

`Path::join` with an absolute argument **replaces** the base — that is the mechanism that reaches
outside the repo. `remove_dir_all` is what makes it terminal.

### 0.2 It is not one seam — it is five

| seam | file:line | doors |
|---|---|---|
| `TaskArea::resolve` | `crates/cli/src/task.rs:763` | 5 (`task diff/validate/discard/finalize/bind`) |
| `start::resume_in_repo` | `crates/cli/src/start.rs:1915` | 1 (`start --task`) |
| `start::reenter_in_repo` | `crates/cli/src/start.rs:2034` | 1 (`workflow … --task`) |
| `doc::ActiveTask::resolve` | `crates/cli/src/doc.rs:5382` | 12 (`doc … --task`) |
| `milestone_dir` + 4 copied `is_dir()` checks | `crates/engine/src/milestone.rs:243` | 8 (`milestone …`) |

`crates/cli/tests/no_such_task_route.rs`'s **own module doc names three of them**. W-13's sentence
is falsifiable in one read: `doc list --task ""`, cited there as passing through `TaskArea::resolve`,
goes through `ActiveTask::resolve`. Four different texts answer `--task ""` across the seams.

### 0.3 `42 of 50` is not a property of the doors

It is fixture-dependent. For 22 of 25 doors the **address** argument fails before the id is
consulted, so which cells "carry a code" changes with the fixture — the auditor reproduced the
walk's shape with **different cells passing**. It must not be the fix's test target.

### 0.4 The rule is already stated, and violated at the other end

`engine::slug::is_slug` (`crates/engine/src/slug.rs:193`) is applied at **three mint boundaries**
(`start.rs:99`, `doc.rs:3426`, `migrate.rs:251`) and at **zero resolve seams**. Driven:
`jigc start --slug "../.."` is refused; `jigc task discard "../.."` is not. A clean razor triple —
stated in code, violated at HEAD, demonstrable in one command.

And the asymmetry is on the same bytes: `jigc uninstall` **blocks** with `uninstall.staged-prose`
over a task's staged docs and hands the reader `jigc task discard <task-id>` as the *safe* exit.

### 0.5 Three further latent defects on the same unguarded id, in no ledger

- **`doc set-slot … --task ""` acks a write at exit 0 into `.jigc/tasks/docs/`, which no finalize
  can reach** — a law-1 lie on the success path. It then makes `docs` appear as an active task in
  the 1.0-pinned `task list --format json`, so every later task-less write is permanently
  ambiguous. The refusal's own printed recovery is `jigc task discard ` — routing the reader into
  the destroying call.
- **`doc show --task <absolute path>` serves content from outside the repo** through the pinned
  read contract; `doc list` prints a repo-relative `path` the bytes did not come from.
- **`task discard ""` over a milestone sub-task bypasses `settle_discarded_sub_task`**, leaving
  `status: active` in the committed record for a task whose bytes are gone — re-opening the
  lying-record defect M49 closed for the named-id path.

### 0.6 The registry exists and need not be manufactured

No registry of id-taking doors exists **but the derivation mechanism is shipped and proven**:
`cli.rs:1530 DOCTYPE_ARG_IDS` → `DOCTYPE_DOORS`, fenced bijectively against the real clap tree.
The identical derivation over the work-unit id arg names — `id` (5), `task` (12), `milestone_id`
(8) — yields **exactly 25**, matching walk 17's set, with no other leaf using those names.
`VERB_KINDS` has 47 leaves; 25 take a work-unit id. `not_in_repo_axis.rs` is the shape such a
fence would take (an `ARMS` table fenced total against `VERB_KINDS`).

---

## 1 · Charter and trial claims found WRONG

| # | claim | verdict |
|---|---|---|
| 1 | *"`TaskArea::resolve` is the one seam"* (charter claim; W-13) | **WRONG** — five seams; the repo's own test module doc names three |
| 2 | *"the empty-id axis"* | **WRONG / understated** — the axis is every path shape; `../..` destroys the repository |
| 3 | *"42 of 50 cells"* | **WRONG as a target** — fixture-dependent, not a property of the doors |
| 4 | *"No read verb shows a task's whole staged area"* (`trial-record.md:55-57`) | **WRONG** — `jigc task diff <id>` prints the code diff **and every staged doc's full body** in one call |
| 5 | W-14's *"gitignored"* framing | **WRONG on one word** — `.jigc/` is tracked at its root (`gitignore.rs:24` ignores seven **subdirectories**); the test admitting `.jigc` is named `a_gitignored_root_still_relocates_…` and reasons from a property `.jigc` does not have |
| 6 | W-15's prescribed fix (*mint `write.unknown-section`*) | **WRONG as written** — `design/validation.md:77` declares this cell as M49's deliberate bound, and the one-hop form fires on **declared** sections too, so the prescribed code would be a law-1 lie on 3 of 4 shapes. Fails the razor's leg 1. |
| 7 | the charter citing M49's refusal of `AddedNestedRepeatable` as *"the port does not need it"* | **citing a retracted rationale** — M49 identified that as port-as-requirements-source and replaced it before the Settle closed; the shipped refusal is at leg 0. Leg number also inconsistent between `decisions-pending.md:174` (leg 1) and `settle-record.md` (leg 0). |
| 8 | the fan-out cut's *"`worktree` appears once in the methodology pack"* | **stale by one** — twice (`migration-finalize.yaml:10`, `planning-record.yaml:64`). Conclusion holds; the count does not. |

### Two locked-doc statements falsified at HEAD (razor leg 1 material)

- **`design/design-altitude-doctypes.md:43-47`** — *"They sit outside the frozen-v1 gate (the
  methodology pack ships no `schema-manifest.yaml`) … a later shape change needs no corpus
  migration."* **False since M40**: the methodology manifest lists `research`/`vision`/`idea` at
  schema-version 1. A planner pricing fork 5 off this paragraph concludes the `research` side is
  free. `implementation/doctype-map.md:42` is correct and machine-fenced.
- **`design/bootstrap.md:27-94`** declares **four** orientation states in prose, including
  *Active task* with a named shape. **Two exist.** The code's own deferral note
  (`render.rs:94-96`) rests on *"no task store yet"* — false since M1 — and carries no trigger
  anywhere in `decisions-pending.md`.

---

## 2 · New defects, in no ledger, brief or trial row

| # | defect | severity shape |
|---|---|---|
| N1 | `jigc task discard <traversal/absolute id>` destroys the workbench, the repository, or any directory on the machine, at exit 0 | **data loss, unrecoverable** |
| N2 | `jigc uninstall` destroys any path under `.jigc/` outside `gitignore::ENTRIES` — an untracked hand-written file is unrecoverable, narrated by nothing | **data loss** |
| N3 | `doc set-slot … --task ""` acks a write nothing can commit, and poisons the pinned `task list` roster with a phantom task `docs` | law-1 lie + permanent ambiguity |
| N4 | `doc show/list --task <abs path>` serves bytes from outside the repo through the pinned read contract | contract violation |
| N5 | `config set placement-root README.md` lands the knob at **exit 0 with every move failed** — two managed docs become unreadable and unlisted, in the state `config.rs:350-363`'s own rationale says must not exist | store lies; all three razor legs |
| N6 | `config set placement-root <absolute>` is silently reinterpreted as repo-relative; the knob reads back absolute, the store resolves relative | law-1 |
| N7 | a symlinked `placement-root` leaves `jigc doc list`'s path and git's recorded path permanently disagreeing | shape-limited |
| N8 | `milestone provision --force` over a file-shaped leftover **narrates an irreversible destruction it then fails to perform**, and routes to itself forever | law-1 lie in the *other* direction; infinite route loop |
| N9 | a file-shaped leftover at `uninstall` **short-circuits the loop**, so a sibling directory leftover's good refusal is never printed | masking |
| N10 | `milestone join` renders its blocking finding headless, **after** a success-shaped narration line and the routing footer; its `--format json` carries **no `schema_version`**, unlike every other findings envelope | law-1 + contract inconsistency |
| N11 | `start --workflow <broken>`'s JSON arm is `{"error": "<flattened text>"}`, not the findings envelope — the `workflow-refs.*` family reaching this funnel is **not** covered by `command-output-contract.md:326`'s declared exception | contract |
| N12 | the `squash: false` milestone finalize ack names one sha and attributes another commit's files to it (JSON envelope carries the same wrong pairing, and no sha for any sub-task commit) | law-1; **gated to exactly the posture fork 8 prescribes** |
| N13 | no pack-load fence checks that a `ref`'s `to:` doctype is in the loaded pack set — pack-load, `doc schema`, `set-field` and `validate` all pass at exit 0; only `task validate` blocks, with the unfollowable route *"create the target in this task"* | latent until a cross-pack ref ships — i.e. until **fork 5** |
| N14 | `milestone join <unknown>` materializes `.jigc/index/edges.json` **before** rejecting the id | minor |
| N15 | on a `--task` read, the singleton-miss message is byte-identical to the task-less one, says *"names no **committed** doc"*, and its route drops `--task` | law-1 |
| N16 | `jigc doc show --task <id>` with no address is a bare clap exit-2 with no code and no route, unlike every other miss shape on that verb | route floor |
| N17 | `jigc start --workflow X "<intent>"` mints a **second** task silently over a live one — a third door on F-5's axis, and the only one that changes the repo | F-5's class |
| N18 | W-15's real class is ~10 bare cells across **5 verbs**; `remove-item`/`retitle-item` answer a section-only address headless while `add-item` answers the *same address* with a proper `write.unknown-section` finding | incomplete sweep |
| N19 | `read_pack`'s literal is route-less and code-less at all four sites (bare `anyhow`), and two doors beyond `start` (`validate`, `describe`) hit it | route floor |
| N20 | the ff-only merge refusal at the milestone boundary has no code, no route and no survivable frame — `surface_commit_rejection` (`task.rs:3146`) frames only a `CommitRejected` downcast, while `milestone.rs:3737-3757`'s doc-comment claims the arm catches **every** way the boundary refuses | the cut's declared bound, mechanism now named |

---

## 3 · Fork-by-fork — what the baseline changed

### Fork 1 — `task discard`'s guard
`DESTROYING_DOORS`' subject **does not transfer**. All four members remove a *worktree-shaped path
under `.jigc/worktrees/`* and ask one git-linkage probe; a task working area is gitignored and holds
staged docs by design, so `probe_leftover` would refuse every live task. The shipped precedent that
*does* fit is `uninstall`'s `uninstall.staged-prose` guard — same bytes, and today the two doors
give opposite answers. The fork is now **id-guard at the seams** (which is Tier 0 regardless)
**plus** whether the *valid-id* destruction joins the staged-prose guard.

### Fork 2 — the read verb
Reframed by claim 4. `jigc task diff <id>` already answers most of *"what is here"*. What no verb
covers: `docs/provenance.json` (created vs edited-from-base) and `roles.json` (bound context roles).
Discoverability is the measured gap — `task diff` is named in **0** pack step files, **0** times in
`.jigc/AGENT.md`, once in `SKILL.md` under a heading about the finalize gate; `jigc task list` is
named in **0** of `QUICKSTART.md`/`MIGRATING.md`/`AGENT.md`, reachable only from `--help` or a
wrong-id error — and `task_list.rs`'s own header records it was built at M26 *for this exact
failure*. The shipped precedent for the cheap arm is `doc.rs:4191 staged_listing_hint`: `jigc doc
list` **already names the open task** on stderr, costing **zero** contract. Costs, driven: a
text-only orientation mention moves **2 of 624** goldens; a structured orientation state costs
`engine::result::SCHEMA_VERSION` 2→3 across four envelopes; a key on the composed envelope and
bodies in `task diff --format json` are both blocked by **prior recorded decisions**, not by the
additive-key window.

### Fork 3 — `placement-root .jigc`
Both options survive, but the axis is wrong: the charter says *tracked* files under `.jigc/`; the
class that **loses bytes** is the **untracked** one (N2). The tracked docs are `git checkout`-
recoverable. And the guard admits three further shapes that leave the store lying (N5/N6/N7). No
`ROOT_KNOBS` registry exists — the subject is a hand-written `matches!` at two places.

### Fork 4 — `AddedNestedRepeatable`
`changelog.releases/changes` is the **only** nested repeatable in the whole shipped 16-doctype set
(enumerated from `doc schema --format json`, not prose). Every change to it — add, remove, reshape
a leaf inside — blocks at `migrate-corpus.unclassified-change`, routed into
`crates/engine/src/schema_diff.rs` **+** `transform.rs` (two files, not one). In the *required*
case it is a **mutual dead end**: `validate` says run the migration, `migrate-corpus` says build
the kind in the source tree. Measured cost asymmetry: a supported kind is **3 commands, one
auto-commit, ~0.6 s, zero authoring**; an unsupported one is unfollowable, forever. Additive today
because both consumer matches are **exhaustive with no wildcard**, so the compiler forces the arms.
No `SchemaChange::ALL` exists — a completeness fence over transform kinds has no set to read.

### Fork 5 — `adr → research`
**Driven green end to end** by the auditor: pack loads, `doc schema` projects the address,
`set-field` accepts, `finalize` promotes, `validate` exits 0, and the dangling arm blocks correctly
at `ref-resolves`. That retires a recorded unexercised-surface bound. Of M48's three grounds for
refusing the analogous edge, **only one carries over** (a one-way door on frozen doctypes) — the
per-item-ref ground does not apply, since this is a doc-level header field like all five shipped
refs. **Optional** ref = `AddedOptionalField`, stamp-only byte delta. **Required** ref = permanent
dead end (`migrate-corpus.fold-refused`; the prose-needing *field* arm is unbuilt). Blocker to
weigh: **N13** — shipping a cross-pack ref in the dev pack makes the dev pack depend on a doctype
it does not own, and `JIGC_PACK_DIR` supersedes the compose-methodology marker.

### Fork 6 — `describe`'s origin pack
The fork's framing is **undecidable from the record as it stands**, and *that contradiction is the
fork*: `design/introspection.md:56` says *"no version governs it"*; the code stamps
`describe --format json` with `engine::result::SCHEMA_VERSION` 2, whose own module doc calls it the
stable external surface. The kinds axis is code-side and enumerable.

### Fork 7 — `setup` over a shape-changing shadow
Driven: `setup` exit 0, silent; `describe`/`doc schema`/`validate`/`start`/`doc list` all exit 1
with the full freeze block and a followable route. The concrete fact the fork needs: **`setup
--format json` already ships a `findings` key, and it is empty** — a warning is an existing slot,
not new contract surface.

### Fork 8 — the fan-out Fix phase
**The primitive is 100 % built and driven-proven** (create → add-task → provision → execute → join
→ finalize, `squash: false` landing one conventional commit per sub-task in id order; the collision
arm blocks at both doors with nothing committed). **The pack composition is 0 % built**:
`fix-task.yaml` and `fix-finding.yaml` do not exist, `fix-gate.yaml` still carries the verbatim
serial rule, `triage.yaml` has no `add-task` line. Two riders the cut does not price: **N12** (the
ack a fix-round orchestrator reads is wrong on exactly the prescribed knob) and **N20** (a dirty
main is the *normal* state a fix round runs in, and that refusal has no code, no route, no frame).
Also: `milestone_boundary_gate.rs` **pre-stages** the commit doc, so `1799a2d`'s acceptance runs on
a fixture that skips the provisioning barrier every real fan-out must cross.

---

## 4 · Genuinely built + proven (do not rebuild)

- The nonexistent-id column at **all 25 doors** — exit 1, a route, nothing on disk changes.
- The converged `no_such_task` message + runnable route (now **five** doors, not M43's three).
- `DESTROYING_DOORS` × leftover-verdict matrix for **directory** leftovers, incl. staged /
  untracked / partly-staged / empty / `--ignored` narration — registry-driven, a fifth door cannot
  land silently.
- The freeze at **47 doors** bijecting `VERB_KINDS`, incl. M49's project-shadow arm; the manifest
  CI fence; the above-current stamp block.
- The migration path for **supported** kinds: 3 commands, ~0.6 s, zero authoring, clean tree.
- Cross-pack `ref` resolution end to end, incl. the dangling-target block.
- The fan-out primitive incl. N-process join determinism and code-collision blocking.
- `jigc task diff` as a whole-staged-area read (text form).
- `doc list`'s open-task note — the zero-contract precedent for fork 2's cheap arm.

## 5 · Registries a completeness fence can read (asked for explicitly)

| axis | registry | state |
|---|---|---|
| id-taking doors | derivable from `id`/`task`/`milestone_id` via the shipped `DOCTYPE_ARG_IDS` pattern → exactly 25 | **derivable, not built** |
| finding text renders | `located_finding_text.rs:250 MESSAGE_SITES` + its source-scanning fence — **already enumerates the whole class** | **built + green**; one extra checked token closes W-1 |
| destroying doors | `DESTROYING_DOORS` × `LEFTOVER_VERDICTS` | **built**; subject excludes `task discard` by definition |
| `.jigc/` non-transient content | `gitignore::ENTRIES` complement — precisely N2's axis | **exists, unused as an axis** |
| write-miss shapes | `VERB_KINDS ▸ doc ▸ Write` (verb axis only); `engine::address::Fragment` could supply the shape half | **partial** — declaredness must be manufactured |
| transform kinds | consumer matches are exhaustive (compiler-fenced); **no `SchemaChange::ALL`** | **absent** — prerequisite for a kind-completeness fence |
| root knobs | hand-written `matches!` at two places; `knobs.yaml` types both as bare `string` | **absent** |
| pack resources a door requires | hand-list of `read_pack`'s six callers + three inline sites | **absent** |
| fan-out abort causes | `RejectionCause` is a **manufactured test-side** enum | **absent** |

## 6 · Declared bounds on this ledger

Not driven anywhere: symlinked `--task` targets; concurrent/fan-out interaction with degenerate
ids; the invocation log's record of degenerate-id calls; all five `MINT_DOORS` id shapes (two
driven); the pack-author cost of omitting a `schema-snapshots/` entry on a bump; the
`ProseNeeding`-slot and `ValueRemapped` authoring costs end to end (branches confirmed in source
and suites). Every destruction repro was run **only** inside `mktemp -d` rig roots; no `rm` was
issued against a variable path at any point.

---

## 7 · Dispositions — appended 2026-09-07 (M50 Increment 13 / T2)

**Appended, not merged.** §§0–6 above are a dated record of what was driven at `d9e91f1` and stay
one; this section is a second dated record of what the wave then *did* about each row. Where the two
disagree, the disagreement is stated here rather than edited out of the prose above.

**Posture.** Every verdict below was **driven at `HEAD = 061e7a3`** against the **release** binary
`target/release/jigc` (rebuilt at that HEAD) on `dev/jigc-rig` corpora, unless a cell says *source
read*. **Eleven** of D14's nineteen were disposed by earlier increments — the plan said twelve — and
each is cited to the increment **and task** that took it; each was also **re-driven here** rather
than taken from its own entry, per the M46 rule *an agent's report is a lead, not a measurement*,
which is how the count was found wrong (below).

**Two corrections to the Increment 13 plan, both found by driving:**

- **N16 was not disposed.** The plan (and `DECISIONS.md` → 2026-09-07 M50 Increment 13 planning)
  lists it as taken at Increment 9. Increment 9's own entry does the opposite — it **excludes** it
  (*"`doc show --task <id>` with no address (baseline N16) is a read verb; the deliverable names
  write verbs"*), which is a scoping decision, not a disposition of the defect. It is disposed here.
- **N20 was not disposed either.** It sits outside D14's 19 because the charter's fan-out cut called
  it a declared bound, and D11 named only N12 as its prerequisite. It reproduces at HEAD and is
  carried here with a trigger.

So the seven the plan expected are **nine**: N11 · N14 · N15 · **N16** · **N20** · N23 · N26 · N27 ·
N28.

| # | verdict | disposition |
|---|---|---|
| N1 | ADMITTED | Tier 0 (D1 + D2) — **Increment 1 / T1** (`work-unit.malformed-id` at the five resolve seams, above clap and below the filesystem op) and **Increment 3 / T2** (the valid-id half: `jigc task discard` refuses over staged prose, `--force` the single consent). Driven at HEAD: `jigc task discard "../.."` and `jigc task discard ""` both exit 1 with the code and the grammar route; the rig repo's `.git`, worktree and `HEAD` are untouched. |
| N2 | ADMITTED | Tier 1 (D8) — **Increment 4 / T4**. Driven: an untracked `.jigc/HANDNOTE.txt` blocks `jigc uninstall` at exit 1 with `uninstall.untracked-workbench-file`, naming the path and both exits (`git add` or `--force`). |
| N3 | ADMITTED | Tier 0 (D2), family 1 — **Increment 1 / T1**. Driven: `jigc doc set-slot 'vision:vision#thesis' --from-file f --task ""` exits 1 with `work-unit.malformed-id`; `.jigc/tasks/` stays absent (no `docs/` area is minted) and `jigc task list` reports **no active tasks**, so the poisoned-roster half cannot arise either. |
| N4 | ADMITTED | Tier 0 (D2) — **Increment 1 / T1** (the `--task <absolute path>` half) with **Increment 2 / T1** closing the address slug head (family 4). Driven: `jigc doc show 'vision:vision' --task /etc` and `jigc doc list --task /etc` both exit 1 with `work-unit.malformed-id`. |
| N5 | ADMITTED | Tier 1 (D8), the value rule — **Increment 4 / T3**. Driven: `jigc config set placement-root README2.md` exits 1 with `config.unusable-root`, naming the file-not-a-directory cause and the state it prevents. The knob does not land. |
| N6 | ADMITTED | Tier 1 (D8), the value rule — **Increment 4 / T3**. Driven: `jigc config set placement-root /tmp/elsewhere` exits 1 with `config.unusable-root`, naming the read-back divergence it prevents. |
| N7 | ADMITTED | Tier 1 (D8), the value rule — **Increment 4 / T3**. The symlink arm rides the same refusal (its route names all three rejected shapes: a file, an absolute path, a path through a symlink). |
| N8 | ADMITTED | **Increment 12 / T2** — the narrator and the removal now read one `LeftoverShape`, so `milestone provision --force` no longer narrates an irreversible destruction it then fails to perform, and the route no longer loops on the argv that just failed. |
| N9 | ADMITTED | **Increment 12 / T2** — the probe's failure is folded into a `LeftoverHold`, so all three refusing doors collect the whole set before refusing; the masked sibling is named. |
| N10 | ADMITTED | **Increment 10 / T2** — the block renders before the stdout ack and the headline states the verdict; `--format json` carries `engine::result::SCHEMA_VERSION`, read rather than retyped. **Declared residue, recorded at that task rather than closed:** the ledger's *"unlike every other findings envelope"* is too strong — the `DocAck` **success** acks also project `findings` with no `schema_version`. A different family (a write ack, not a report of what refused), outside that deliverable, and inside the pre-1.0 additive-key window M48 Increment 7 closed with its text/JSON parity fence. Recorded, not deferred. |
| N11 | REFUSED | **Leg 1 — the rule the row leans on is not stated; the shape is the declared complement.** Driven: `jigc start --workflow no-such-workflow "x" --format json` → `{"error": "blocking · workflow-refs.unknown-workflow — … route: …"}`, exit 1, from `start::read_workflow`'s `finding_to_err`. `command-output-contract.md:202-204` declares exactly this: *a finding consumed by `finding_to_err` projects no key at all* and rides `{"error": …}`, with the flattened text carrying the code and route since **Increment 10 / T1** — *"the contract's completeness claim is not falsified by it."* The baseline cited `:326`, which is the **declared singleton `target: null` exception** — a different rule about a different funnel. |
| N12 | ADMITTED | D11's prerequisite — **Increment 11 / T1**. `committed.commits` (every commit oldest first, each `{hash, subject, paths}`) and `committed.sub_tasks[].hash` land additively; `manifest`/`files` keep their meaning because the Settle prices a correction there as a break. |
| N13 | ADMITTED | D7's ships-regardless half — **Increment 8 / T1** (the pack-load ref-target fence over the composed set) and **/ T3** (the `to:` key on `doc schema`'s ref projection). Driven incidentally while driving N23: removing `adr` from a loaded pack now blocks **every** door at pack-load, naming the loaded pack-set and three exits. |
| N14 | REFUSED | **Leg 1 — no rule states it, and the refusal makes no claim it falsifies.** Driven: `jigc milestone join no-such-milestone` blocks `milestone.unknown` (exit 1, `at:` + a runnable route) **after** creating `.jigc/index/edges.json`. `index/` is a `gitignore::ENTRIES` member — a rebuildable derived cache that is a pure function of the committed store, the shape M46/M49's read-verb rule permits by name. The last clause was **asserted, not assumed**: the refusal names the id and routes to `jigc milestone create` and states nothing about what was or was not left behind, and `malformed_work_unit_id.rs`'s neighbouring survival assertion is over the **tracked** set, which the call leaves byte-identical (`git status --porcelain` empty). |
| N15 | CARRIED | Driven **exactly** as recorded, and the consequence driven too. With `vision:vision` genuinely staged in a live task, `jigc doc show 'vision:wrong-slug' --task <id>` is **byte-identical** to the task-less form, says *"names no **committed** doc"*, and its route (`jigc doc show vision:vision`, `--task` dropped) followed verbatim serves the **committed** prose, not the staged copy the reader is holding. A law-1/route wobble on a 1.0-pinned read surface, with no tier open to admit it. → `decisions-pending.md`, with a trigger. |
| N16 | REFUSED | **Leg 1 — and the row's own premise is refuted by driving.** The baseline says the bare clap exit-2 is *"unlike every other miss shape on that verb"*. Driven over six leaves at HEAD (`doc show`, `doc set-slot`, `doc add-item`, `task diff`, `doc schema`, `rename`), a **missing required argument** is answered identically everywhere: exit 2, the argument named, a usage line. It is not unlike anything; the baseline compared it to *address-shape* misses, which are the different question of an address that was supplied and did not resolve. And no locked artifact states the rule: `validation.md` → The route floor governs **findings**, and D2's clap-exit-2 reasoning is about a **supplied** value the door must adjudicate, never an **absent** argument clap already names. Taking arity off clap at every leaf is a design fork, not a repair. |
| N17 | ADMITTED | D3 / design-review B8 — **Increment 5 / T3**. Driven: a second `jigc start --workflow dev-task "second"` over a live task still mints (several open tasks are legal) and now prints *"also open: 1 other task was already open before this call — nothing here touched it"* with the `--task` addressing rule. |
| N18 | ADMITTED | D10 — **Increment 9 / T3**. `MISS_SHAPES` (39 rows, one per `(verb, address shape, leading-hop declaredness)`, each carrying the code it earns) crossed against `CELLS` in both directions; `validation.md:77`'s universal is replaced by the named fence that holds it. |
| N19 | ADMITTED | **Increment 10 / T3**. One producer (`start::pack_resource_missing`) answers for all four sites with `pack.resource-missing`, the searched pack-set, and a `Human` route. |
| N20 | CARRIED | **Driven at HEAD, and it is the ninth row, not a bound already discharged.** A milestone boundary whose fast-forward collides with main-checkout WIP prints `` `git merge --ff-only <sha>` failed: error: The following untracked working tree files would be overwritten by merge: …`` — **no finding code, no `route:` line, no survivable frame** (nothing states what survived, and there is no re-run argv), exit 1. Confirmed state-safe rather than assumed: `HEAD` is unmoved, and moving the WIP aside and re-running lands the boundary at exit 0. **The mechanism is one level in from where the baseline put it.** Both boundary arms *do* reach `surface_commit_rejection` with a fully-built `RejectionFrame` — `milestone.rs:3905`'s comment (*"this arm catches EVERY way the boundary refuses … a `git merge --ff-only` refusal over ordinary untracked main-checkout WIP"*) is **true of the match arm**. What drops the frame is `task.rs:3534`, which renders it only when the error downcasts to `CommitRejected`; a `git_run` failure is a plain `anyhow`, so it falls to `render::operational_error` and loses the code, the route, the survived-state clause *and* the door's error identity in the invocation log (`Outcome::failure()` rather than `Outcome::error(frame.code)`). Driven on the **default** `squash: true` arm; the `squash: false` arm was not reached in this fixture (it gates earlier on the sub-task's own commit doc) and is a source read. → `decisions-pending.md`, with a trigger. |
