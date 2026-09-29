<!-- 2026-09-10 · consolidated from four Opus gap-detector reports (decisions · docs · doctypes · capabilities) · HEAD 74627547 · target/release/jigc 1.0.0-rc.14 · no cargo, no repo edits -->

# M51 — consolidated gap findings

Four `gap-detector` subagents, one per dimension, each **driving the release binary**
`target/release/jigc` (`1.0.0-rc.14`) at `HEAD = 74627547` on `dev/jigc-rig` corpora under the
session scratchpad. **None ran cargo; none edited a repo file; no mutation was applied to any
fence.** Their reports are copied verbatim beside this file:
[gap-decisions.md](gap-decisions.md) · [gap-docs.md](gap-docs.md) ·
[gap-doctypes.md](gap-doctypes.md) · [gap-capabilities.md](gap-capabilities.md).

Context: [charter.md](charter.md) (scope, claim, razor, nine forks, exclusions) and
[baseline-ledger.md](baseline-ledger.md) with its four companions.

**What this file is.** One ranked, deduped gap list for the M51 Settle, plus the forks the Settle
must take, the corrections the probes made to the charter and the baseline, the contradictions
between locked docs, and the gate-record slots this wave will strain on. **It settles nothing and
carries no recommendation** — each row names the decision owed, not its answer.

**Provenance rule, inherited from the evidence check and binding here first: an agent's report is
a lead, not a measurement.** Rows below marked *driven* were produced by a detector running the
binary and are quoted with their command; rows marked *source-read* or *read at HEAD* are leads
and say so. Relayed is not verified by this file's author — the rows the Settle turns on are
re-driven at Settle, and the gate-record records which.

---

## The ranked gap list

Ranked **blocking** (a fix cannot be planned without settling it) → **fork** (a cheap and a robust
arm, both defensible) → **advisory** (still owed, lower blast radius). One row per distinct gap;
the four reports' duplicates are merged and every raising item id is carried.

| id | gap | severity | raised by | the decision Settle must take | evidence |
|---|---|---|---|---|---|
| **G-1** | No doc owns "a path-taking argument", and the one doc that speaks to `migrate <path>` warrants the **opposite** — its founding rationale sells the unbounded token as the verb's reason to exist, while its scoping sentence says *repo file* and the binary exceeds it. | blocking | docs B1 · docs F-a · decisions B11 · capabilities H2 | Which doc mints the **source-side** path rule (`write-commands.md` beside its destination-side sibling · `auto-migration.md` as the retire's trust model · `storage.md` as a home rule), and whether `auto-migration.md:39`'s *"arbitrary foreign file"* is **struck with its falsifying datum and narrowed to in-repo** or kept. | `design/auto-migration.md:39`, `:68`; `design/write-commands.md:127` (destination rule + `write.untrackable-destination`); read at HEAD |
| **G-2** | EC-1's sink guard has no engine-side predicate: `plan_retirements` is in `crates/engine`, which shells out to nothing, and `untrackable_reason`'s legs 3/4/5 are `git_output` shell-outs — and the untrusted `source-path` has **five** raw consumers, not two. | blocking | capabilities G1 · decisions B11 · baseline §0 | Lexical engine predicate at `plan_retirements` (cannot answer the symlink cell) **vs** the CLI validating `plan.retirements` before `retire` over all five legs; and whether the sweep covers all five raw consumers or only the sink. | `crates/engine/src/finalize.rs:400`, `:131`; `crates/cli/src/trackable.rs:55`; consumers at `task.rs:1296/:1426/:1794/:2491` + `engine/finalize.rs:401`; `grep -c "Command::new" crates/engine/src/` → 0 (driven) |
| **G-3** | `storage.md`'s source-of-truth model has **no room for a working-area file that is authority**: it calls the staging area a rebuildable cache or transient working copy, while `.jigc/tasks/<id>/source-path` authorizes a permanent deletion and buys a carryover-gate exemption. | blocking | docs B2 | Which homes gain the trust-model sentence — `storage.md:276` (the working area holds one file that is authority, and what constrains it) · `auto-migration.md:70` (the re-validation clause) · `finalize.md:174` (the rollback row describes byte-capture, not admissibility). | `design/storage.md:7`, `:276-290`; `design/auto-migration.md:70`; `design/finalize.md:174`; tamper repro driven at baseline1 §4 |
| **G-4** | The exit-4 review hold — **the only human gate on jigc's only byte-destructive op** — names **no path**, the pinned JSON hold carries no retire key, and `jigc task validate` is silent about it. This is a surface **addition**, not a violated rule, so it needs its own scope decision rather than silent inclusion under EC-1. | blocking | decisions B11 · docs A4 · doctypes D5 · capabilities G4 | Does the hold name the retire target, on which surfaces (text · an additive `retires` key on the ad-hoc `json!` · `GATE_COVERAGE ▸ Tier::Previewed` so `task validate` previews it), and is it admitted as a hole in a declared surface or as an addition priced on the human's criterion. | `design/auto-migration.md:64`; `crates/cli/src/render.rs:4176`, `:4183-4192` (no test, no golden asserts its key set); `grep -c "$EXT" hold.txt` → 0, hold JSON = `["review","rewrites","source","task"]` (driven, baseline1 §4); `gate_coverage.rs:43-49` |
| **G-5** | Both packs' `migration-finalize` step states *"the deletion staged … a `git rm` in effect"* — **false in every EC-1 cell** — and it is a **fenced named fact**, so any repair must keep all three required tokens. | blocking | doctypes D5 | Whether F1's landing makes the sentence true again with no pack edit (robust reading) or the step's prose is repaired (cheap reading leaves both packs shipping a false statement about the one destructive op under a fence that certifies its facts). | `crates/cli/pack/steps/migration-finalize.yaml` + `packs/methodology/steps/migration-finalize.yaml`; `cli::task::stage_migration` `crates/cli/src/task.rs:3064` (`path_in_index` guard); `CONSTRAINT_REQUIRED_TOKENS` `crates/cli/src/pack.rs:1242`; fence teeth driven via `JIGC_PACK_DIR` |
| **G-6** | The **flow 51 acceptance chapter** states M50's claim as achieved over *"four token families"* — EC-1 falsifies the claim and the count, in a locked acceptance chapter. | blocking | docs B1 | Whether the chapter takes a dated correction in the same wave that fixes the behaviour, and whether the flow-52 chapter may restate the claim without the fifth family. | `design/worked-examples.md:3356-3358` (read at HEAD) |
| **G-7** | EC-2 asks for "one probe at the shared seam" and **there is no shared pre-work seam**: `discover_repo_root` is copy-pasted into six modules, the six milestone doors call it directly, and `jigc setup` bypasses `git_commit_capture` entirely. | blocking | capabilities G2a · decisions B2/B3 | Where the posture probe lives — per door · at `git_commit_capture` (fires *after* the door acted) · at `overlay_docs_commit_and_ff` (the landing act, live checkout as subject) · a new shared pre-flight — and what covers `setup`, which no seam-placed probe reaches. | `crates/cli/src/locate.rs:165`, `repo.rs:41`, `milestone.rs:5411`, `task.rs:4911`, `start.rs:3826`, `ingest.rs:632`; `task::git_commit_capture` `task.rs:4255`; `setup.rs:1709`; source-read at HEAD |
| **G-8** | The honest subject is **repository posture — a predicate family of four**, not one `symbolic-ref`: driven, the charter's probe answers **1 of 4** cells and false-greens unborn HEAD and a `GIT_DIR` redirect. | blocking | capabilities G2b · decisions F1 · baseline cells 4/5 | Is the subject *HEAD posture* or *repository state*, and which members are in scope: detached · unborn (`rev-parse --verify HEAD`) · operation-in-progress (`MERGE_HEAD`/`rebase-merge`/`CHERRY_PICK_HEAD`) · git-dir redirect · **subject repo** (the linked-worktree split). | driven table in capabilities G2b; `grep -rn "MERGE_HEAD\|rebase-merge\|CHERRY_PICK" crates/` → 0; `GIT_DIR` and unborn-HEAD repros at baseline `[doors §2(c),(d)]` |
| **G-9** | The fan-out exemption **cannot be inferred at the seam**: the boundary passes only `wt.path()`, so `git_commit_capture` sees a bare `&Path` and a path-shape guess is the exact class M50's audit condemned; under `squash: false` one `DedicatedWorktree` reaches the seam **N+1** times. | blocking | capabilities G2c · decisions F2 · baseline §4 | Whether the exemption becomes a **typed argument** at the seam (every call site states its posture; a new one cannot compile without deciding) or a per-door guard — and whether it is per-call or per-boundary. | `crates/cli/src/task.rs:4573` (`DedicatedWorktree`), `:4496`, `:4225`, `chain_commit` `:4477`; source-read at HEAD |
| **G-10** | **No design doc states any HEAD or git-posture rule**, and the nearest one — `finalize.md`'s *"No in-progress merge/rebase/bisect"* preflight — is **unimplemented**: zero probes exist in either crate. | blocking | docs B3 · capabilities G2b | Strike `finalize.md:31` with the datum or build it; and where a *HEAD-on-a-branch* rule and F2's exemption question are homed (`finalize.md` → `fan-out` finalize already reasons about the boundary's subject). | `design/finalize.md:31`; `grep -rn "MERGE_HEAD\|REBASE_HEAD\|rebase-apply\|CHERRY_PICK\|BISECT" crates/` → 0 (driven) |
| **G-11** | `jigc task finalize --carry-staged` **concludes an in-progress merge** at exit 0 — HEAD gains two parents under jigc's own subject, `MERGE_HEAD` consumed, ack `1 file committed`, no mention of a merge. In no ledger row. | blocking | docs B3(iii) · doctypes A7 · capabilities G2b · baseline cell 15 | Does the flag's documented consent (*carry my staged work*) cover **concluding someone else's merge** — refuse, narrate, or declare it in scope — and does the fenced `finalize` step then owe the new fact plus a `CONSTRAINT_REQUIRED_TOKENS` token. | driven end to end in docs B3 (`git log -1 --pretty='%h %p %s'` → two parents) and baseline `[doors §2(e)]`; `packs/methodology/steps/finalize.yaml` |
| **G-12** | `jigc setup` fits **neither registry both EC-2 and EC-26 name**: `COMMITTING_DOORS`' member predicate is *a hook-capable commit* and setup passes `--no-verify` by recorded design (its rejection already travels as the finding `setup.install-commit`), while `MINT_DOORS`' subject is a working-area mint and setup mints none. | blocking | decisions B2 · decisions B3 · capabilities G2a · capabilities G5 | Mint a **third subject** ("doors that commit on the user's behalf", a superset of `COMMITTING_DOORS`) **vs** fix `setup` outside any registry and say so — and whether the two locked sentences that state the gate's subject as a task-minting door are **revised with their basis** or left with `setup` declared outside the gate. | `crates/cli/src/invocation_log.rs:95-97`, `:108-113`, `:119-121`, `:130-171`, `:174-180`; `setup.rs:1709`, `:1602-1608`; `engine::state::MINT_DOORS` `crates/engine/src/state.rs:808`, `:860-863`; `design/surface-contract.md:123`; `design/finalize.md:159` |
| **G-13** | EC-26 has a **razor leg-1 citation the charter does not carry**: `assistant-adapter.md` declares setup commits *"only its own install"* with an enumerated pathspec — true of the **paths**, false of the **bytes**, at a committing door that swallows user work at exit 0. The charter files it in Tier 3, whose admission argument is weaker. | blocking (mis-tiered) | decisions B4 · docs B7 · docs F-d row 8 | Adjudicate EC-26's tier **on the citation** rather than on the ledger's "capability" label; and whether `:52`'s sentence is re-derived (a pathspec bounds paths, never authorship) as part of the fix. | `design/assistant-adapter.md:52`; EC-26 driven by reviewC (uncommitted `CLAUDE.md` edit lands inside `chore(jigc): install jigc workspace config`) |
| **G-14** | EC-26's fix needs **two capabilities nothing supplies**: a third `CarryoverBoundary` variant (whose task-shaped finding code and route would lie at setup) and a **pathspec-scoped** staged snapshot — `decide_carryover` compares whole snapshots while setup's commit is pathspec-limited, so an unscoped gate blocks setup on every unrelated staged file. | blocking | capabilities G5 | Pathspec argument on `git_staged_snapshot` vs a caller-side filter; and whether a `Setup` variant reuses `finalize.carried-staged`'s identity/route or mints its own. | `crates/engine/src/finalize.rs:735`, `:678`, `:697`; `crates/cli/src/task.rs:3925`; `setup.rs:1709`; unrelated staged `feature.txt` correctly untouched, driven at baseline `[doors §4]` |
| **G-15** | **EC-3's premise is falsified in one member and understates the class ~4×**: `hook_committed` **is** declared — in `assistant-adapter.md`, not in the contract doc whose own rule is that every addition is declared *there* — and the driven count is ~17 undeclared keys across ~10 verbs. | blocking · prior-art-contradiction | decisions B5 · docs correction 1 · docs B4 · capabilities H5 · baseline cell 6 | The question fork 7 cannot be answered before: **is an undeclared key on `unmanage` a defect, or is `unmanage`'s envelope simply not pinned?** — i.e. which envelopes 1.0 pins. "Declare the four vs delete the four" presupposes the four are the set. | `design/assistant-adapter.md:56`; `design/command-output-contract.md:13`, `:446`; driven envelopes (`unmanage` → `{path,identity,dropped}`; `config get` → `{key,layer,op,rejected,value}`; `config list` → `{knobs,op}`; `ingest` → `{rows,summary}`) |
| **G-16** | `command-output-contract.md`'s own claim — that the flattened `{"error": …}` text *"since M50 Increment 10 carries the code and the locus inside it exactly as the agent-text surface does"* — is **false at those 22 doors**: a stated rule violated at HEAD on the pinned write-side contract, in no ledger entry. | blocking | decisions B8 | Whether this is admitted as a Tier-2 law-1 row inside fork 5, or separately — and either way that the contract sentence moves with the fix. | `design/command-output-contract.md:202`; driven: `jigc --format json task finalize no-such-task` → `{"error":"no task `no-such-task` — list live tasks with `jigc task list`"}` rc=1, same for `task validate`/`doc list --task`; `milestone list-tasks` likewise |
| **G-17** | **Fork 5's cheap arm cites a precedent whose load-bearing premise does not transfer.** The M50 refusal is grounded on `(code, null)` for *a string that names nothing*; F-5's subject is a **work unit**, a target form the same contract already declares, and the engine already ships `finalize.no-task` — the CLI short-circuits it one layer above, in one shared home, as a code-less `anyhow`. | blocking · foreclosed-by-doc | decisions B7 | Re-pose fork 5 as *"make the declared contract true at these doors"* vs *"declare the doors permanently outside the envelope"* — **not** as string-vs-envelope. | `design/command-output-contract.md:215`, `:167`; `crates/engine/src/finalize.rs:1146` (`task_missing_finding`); `crates/cli/src/task.rs:971` (`no_such_task`), `start.rs:2131`; driven above |
| **G-18** | `design/finalize.md` **contradicts itself** about what the transaction promises — the config-layer row's *"worktree untouched"* is a clause about the **restore's mechanism**, five lines above *"Before phase 6, all-or-nothing"* — and a second, sharper contradiction is unnamed: `DECISIONS.md` records the M45 fix as rolling back *"every path jigc's own staging contributed"*, false of the worktree half at the very site EC-29 drove. | blocking · prior-art-contradiction | decisions B9 · docs B6 · capabilities H6 | **Which sentence is the promise**, before fork 3 is answerable; and that the cheap arm is not *keeping* a declared decision but **declaring a new one** (index-only fidelity, worktree residue accepted, reachable by *upgrade → finalize → hook rejects*) — recorded as such, with `:183` and `DECISIONS.md:4788` both moving. | `design/finalize.md:178`, `:183`, `:174`, `:175`; `DECISIONS.md:4788`; residue driven at baseline `[doors §5]` (` M .jigc/version`, ` M .jigc/.gitignore`) |
| **G-19** | `COMMITTING_DOORS` is **10** and six shipped surfaces say **nine** — a stale count with one authority — but underneath it sits a **genuine definitional disagreement**: `jigc setup` is a committing door that is not a member, and both EC-2 and EC-26 widen "the registry". | blocking | capabilities H7 · docs A9 · docs B12 (sibling) | Settle *what a committing door is* before either row widens the registry; then the count sweep is mechanical. | `crates/cli/src/invocation_log.rs:130`; `crates/cli/tests/flow47_acceptance.rs:18` vs `:21`; `MIGRATING.md:39`, `QUICKSTART.md:180`, `CLAUDE.md:7`, `design/worked-examples.md:3005`, `decisions-pending.md:320` |
| **G-20** | **Every new finding code this wave mints has two doc homes and the charter names neither** — `validation.md`'s severity inventory + per-wave registration sections (default severity · intrinsic-or-tunable · keyed/unkeyed) and `surface-contract.md`'s 11-row error-code namespace (anything that becomes an `Outcome` identity). At least seven codes are at risk. | blocking | docs B11 | For each minted code: which **surface** it fires against (task-scope gate · store-scope report-only · an operational funnel outside `Finding`), its severity class, and whether it takes a door identity. | `design/validation.md:489`, `:607`, `:620`, `:19`, `:88`; `design/surface-contract.md:125`, `:131-142` |
| **G-21** | **`AMBUSH_CLASS_CODES` is a hand-listed const derived from nothing**, and M51's Tier-0 mints exactly the shape it encodes (a contract that first appears in its own block message). Both fences have teeth — **driven** — but nothing reddens when a new ambush-class contract stays off the hand-list. **N26's recorded trigger fires verbatim at this Settle.** | blocking (and a fork — see G-35) | doctypes D1 | Per new blocking contract: (a) joins the class or not, (b) which step declares it, (c) which tokens join `CONSTRAINT_REQUIRED_TOKENS`, (d) whether the set becomes **derived**. Plus N26's own question: is the fence's subject *packs that ship steps* rather than *packs that ship a manifest*. | `crates/cli/src/pack.rs:779`, `:802`, `:1242`, `:1303`; both fences driven red via `JIGC_PACK_DIR`; `implementation/decisions-pending.md:601` |
| **G-22** | **No schema-shape or home change is admissible anywhere in this wave** — and the freeze covers **17 manifest entries / 16 doctypes across both packs**, not the six "frozen v1" evokes. Driven: reworded `hint:`/`description:`/comments keep pack-load green; a `location:` change reddens it. One trap inside the free set: `optional:`, `set:`, `default:`, `of:`, `check:`, `title-names-symbol` are all **inside** the hash, so a Tier-2 "reword" that flips `optional:` is a version-gated change in a prose costume. | blocking | doctypes D2 | Record the wave's honest statement — **zero schema-hash movement, zero `schema-version` bumps, zero corpus migrations** — and that a `milestone-record` posture field (the one candidate that moves a hash) is refused by the razor and unnecessary, since a posture probe is a refusal, not a record. | driven table in doctypes D2 (`JIGC_PACK_DIR` copies, `jigc describe` rc); `crates/engine/src/manifest.rs:52`, `:76`; `crates/cli/pack/config/schema-manifest.yaml`; `packs/methodology/config/schema-manifest.yaml`; `packs/methodology/schemas/milestone-record.yaml` |
| **G-23** | **EC-17's axis is wrong in one cell and short by a dimension**: the four `DESTROYING_DOORS` run **two subject derivations** (on-disk walk vs `git worktree list`'s registered set), the fourth cell **cannot exist**, and `finalize.md:204`'s *"the subject is the on-disk path, never the registered set"* — a universal whose *by construction* warrant was explicitly withdrawn at M46 — is violated by the teardown at HEAD. A third derivation (`narrate_taken`'s `symlink_metadata(...).is_err()`) is what turns a wrong enumeration into the *"not recoverable"* lie. | blocking | docs B12 · capabilities G6 · baseline §3 | Does `:204`'s universal **bind the teardown** or take an explicit carve-out with its reason; is the axis `{shape} × {on-disk walk, registered set}`; and does `doomed_at` collapse onto `probe_leftover` (feasible with **one** widening — the richer per-entry form, refusal rendering a subset). | `design/finalize.md:204`; `crates/cli/src/milestone.rs:4348`, `:4356`, `:2645`, `:5091`, `:2514`; driven at baseline `[prose §4, §9.3]` |
| **G-24** | **The 1.0 read contract's own declared conformance witness states a shape the binary does not emit** — 6 top-level keys against the witness's 4, 5 item keys against 3, all four extras correctly declared elsewhere. **The pin's proof is the wrong thing.** | blocking | docs B5 · docs F-d row 2 · capabilities G3 | Is the witness **regenerated from or fenced against** the shape it witnesses (`doctype_map_versions.rs` is the shipped mold, but it needs the shape to be a code-side value rather than an ad-hoc `json!`), or is the witness role reassigned. | `design/doc-read-surface.md:5`, `:57`, `:59/:68/:69`, `:88`; `design/team-ready-state.md:195`; driven (`jigc doc show milestone-record:padding-wave --format json`) |
| **G-25** | **EC-12's fix is a qualifier, not a flip** — two locked docs already settle it (a **sub-task** discard is a record-only committing door; an ordinary discard writes no commit), so a build that rewrites the guides to *"`task discard` commits"* ships a **new** falsehood for the case the guides' own narrative is about. | blocking | docs B8 · docs F-d row 6 | The sub-task qualifier's exact wording in both shipped guides, sourced from `team-ready-state.md:102/:106` rather than from the registry count. | `design/team-ready-state.md:102`, `:106`; `MIGRATING.md:40`; `QUICKSTART.md:180`; `design/surface-contract.md:131-142` |
| **G-26** | **Both shipped adopter guides carry an unscoped *"jigc never passes `--no-verify`"* universal the binary breaks** — widened in QUICKSTART to *"Every jigc verb that commits on your behalf"* — while the design docs are internally correct and **scoped**. The code's own rationale (*"the only hook present is the warn-only `pre-commit` setup just installed"*) is false for exactly the adopter who already had hooks — the same adopter EC-26 swallows work from. | blocking | docs B7 · docs N-a · docs F-d row 7 | Scope the guide sentence, or change `setup`; and whether setup's recorded `--no-verify` rationale is struck with the datum. One paragraph's worth of fix, to be planned with G-13/G-14. | `MIGRATING.md:39`; `QUICKSTART.md:160-161`, `:165`; `crates/cli/src/setup.rs:1709`, `:1603-1607`; `design/finalize.md:89`, `:107`, `:250` |
| **G-27** | **EC-15: no doc states whether the join's overlay is computed on a blocking path.** The per-sub-task staged set **exists at the derivation site and is thrown away** — `join` loads each `ProvenanceRecord` and builds groups carrying `source_task`, then `continue`s on the clash arm before anything is claimed — so `no_docs_from` names two sub-tasks that both staged the clashing doc, and the text and the wire agree with each other while both disagree with the disk. | blocking | docs B13 · capabilities G7 | Populate the overlay on every blocking path **vs** declare it absent and derive the summary from the per-sub-task `provenance.json`; and — separately — whether `overlay` stays `{}` on a block, so the fix does not silently change one key while fixing another. `no_docs_from`'s **shape** does not move; only its values become true. | `design/command-output-contract.md:403`; `crates/engine/src/milestone.rs:1947`, `:2002`, `:2094-2096`, `:1701`; `crates/cli/src/render.rs:3841-3844`; both blocking classes driven at baseline4 §4 |
| **G-28** | **EC-9 re-opens a decision the human took twice**, and the charter's own exclusion list already excludes it: the publishing floor is `DECIDED 2026-07-16 (human): post-v1`, re-affirmed at the M46 Settle by citation, and its trigger fires **after** the 1.0.0 call. Two of EC-9's four facts are also stale (`publish = false` and the license are already set). | blocking · prior-art-contradiction | decisions B1 · docs B9 · docs correction 2 | Either the human states that 1.0.0 **is** a public/outward release — a **basis-has-changed reversal recorded as one** — or EC-9 is refused with the citation. It cannot be taken silently as a fresh scope item. | `implementation/decisions-pending.md:500-502`, `:419`, `:655`; `Cargo.toml:13`, `:15`; charter → *Decided OUT* |
| **G-29** | **EC-8 has no home at all, and any `design/`-only home reaches no adopter.** Nothing in the tree states the **binary's** versioning; what exists governs *contracts*. And the adopter-reach constraint is structural: the shipped guide body is `QUICKSTART.md` + `MIGRATING.md` `include_str!`'d, so `design/` never ships — while the guide preamble already tells adopters to re-run `jigc setup` after upgrading, with no compatibility statement behind it. | blocking | docs B10 · decisions F6/F7 · capabilities G3 | Which internal home (a new `design/` part-doc · `command-output-contract.md` → *Evolution posture* widened · `implementation/`), **and** what sentence lands in the guide body — i.e. is the policy adopter-facing or internal. | `grep -rn "semver\|SemVer\|1\.x\|major version" design/ implementation/ MIGRATING.md QUICKSTART.md VISION.md CLAUDE.md` → nothing for the binary; `crates/cli/src/setup.rs:69-70`, `:95-96`, `:79-84`; `design/command-output-contract.md:399`; `design/doc-read-surface.md:88` |
| **G-30** | **The wave's claim spans a token family the charter splits across two tiers with two rules.** `cli.rs` classifies `value` as `ArgToken::Plain`, and `config set docs-root <value>` turns it into a path component at a **moving** door (`git mv` of the committed corpus) — while EC-27 sits in Tier 3 with **no fork** and a different predicate (*"a value that reads back as itself"*), driven incomplete (`"   "`, `" "`, `"  x  "`, `"-"` all exit 0). | blocking (scope coherence) | decisions B10 · docs A2 · capabilities G15 | Is the family *the five ids the `ArgToken::Plain` doc-comment names* or *every `Plain` argument that becomes a path component* (six, incl. `value`)? If the former, say what closes the claim for `value`; if the latter, EC-27 moves into fork 1. | `crates/cli/src/cli.rs:1552`, `:1500-1508`, `:1545`; `crates/cli/src/config.rs:677`; `design/storage.md:176-178`; EC-27 widened at baseline `[tokens §2h]` |
| **G-31** | **The pack-step-count fence bans the number that is now true.** The fence fails any count claim equal to **69** outside a dated bracket, with a message asserting 69 *"was never the number"* — and the tree is now exactly 69 (30 dev + 39 methodology). Its own prose is stale in two places. If EC-10's fix is a new pack step the tree goes to 70 and the cell disappears **by accident**, leaving the fence wrong and unnoticed. | blocking | doctypes D4 | Whether the fence is re-aimed (and how the historical measurement point is kept) in the same motion as any M51 doc, record or fold-back that states the current count — and that the rc.11 charter row it also pins is inside EC-14's sweep subject. | `crates/cli/tests/foldback_truth.rs:454-469`, `:458`, `:311`, `:42`, `:534`, `:472`; `implementation/decisions-pending.md:355`; counts driven (`ls … *.yaml`) |
| **G-32** | **Git-env scrubbing is invariant-adjacent, and naming it out is itself a decision.** No production `env_remove`/`env_clear` exists, so `GIT_DIR`/`GIT_WORK_TREE` flow straight through — but jigc integrates with git **by shell-out by design**, and scrubbing changes behaviour for legitimate worktree and env users. Three further posture cells the charter names no fork for sit beside it: unborn HEAD pinning git's empty-tree hash into a **committed** record; the linked-worktree split (`milestone create`/`add-task` write into and commit onto the **main** checkout while the operator stands on `feature` — design-consistent per `locate.rs`, but the ack names neither checkout nor branch); and `--carry-staged` concluding a merge. | blocking (scope) | decisions F1 · capabilities G2b · baseline cells 4/5/10 | Is the subject *HEAD posture* or *repository state* — and specifically, **is git-env scrubbing in scope?** Leaving it unnamed is not a decision; naming it out is. | `grep -rn "env_remove\|env_clear" crates/cli/src` → no production hit (driven); `crates/cli/src/locate.rs:23-31`; `CLAUDE.md` → Code architecture (git by shell-out, never a git library); cells driven at baseline2 §2 |
| **G-33** | Nothing enumerates **which envelopes are pinned at all** — the contract doc names three surfaces while ~14 shipped envelopes ride in none of them, and the code-side registry that bijects the verb tree classifies *fence-ability*, never *pinned-ness*. Whatever the wave writes here **is the 1.0 contract**, not a test over it. | fork · cheap-vs-robust · one-way door at the pin | decisions B5/B6 · docs B4/F-b · capabilities G3/H1/H5 · baseline cell 6 | See **fork 7 (amended)** below. Also owed inside it: the recipe table's bijection is *one recipe per leaf verb*, so verb×**arm** splits (`task finalize` landed vs `--dry-run`; `milestone finalize` landed vs blocked) require reshaping the duplicate-path guard that is that test's load-bearing half. | `design/command-output-contract.md:13`, `:446`; `crates/cli/tests/text_json_parity_axis.rs` (`Tier::Fenced`/`Judgment`); `crates/cli/tests/format_json_success_axis.rs:389`, `:884`, `:918`; 47 leaves driven at baseline `[envelopes §1]` |
| **G-34** | **EC-6's cited basis is a misreading, and its fix could mint the obligation it claims to protect.** *"Independently versioned surfaces"* means independent **of each other**, not *carrying a version*; the M44 `task_id` deferral rests on *rebuildable + purely additive*, not on an integer. The log is opt-in, gitignored, rebuildable and consumed by our own scripts — it fails leg 0 as a one-way door — while **adding** a version integer creates a pinned surface at the moment the wave is closing the pin. | fork · cheap-vs-robust | decisions F4 · docs F-d row 11 · capabilities H7 | See **the EC-6 fork** below: version the log vs **declare it unversioned** and close the key set. | `design/measurement.md:75`; `crates/cli/src/invocation_log.rs:379`; `implementation/decisions-pending.md:455`; `grep -rn "log_version\|LOG_VERSION\|record_version"` → 0 |
| **G-35** | **`AMBUSH_CLASS_CODES` derived vs listed** — the mechanism half of G-21, posed as its own fork because it is the difference between this wave's contracts being fenced and being remembered. | fork · cheap-vs-robust | doctypes D1 | See **the `AMBUSH_CLASS_CODES` fork** below. | as G-21 |
| **G-36** | **EC-10's home: a fence, a pack step, or neither.** The recorded refusal in `foldback_truth.rs` declines the version assertion **by name** — sound for choosing a **numeral**, silent on the weaker checkable claim *the version this fold-back names is the version `Cargo.toml` carries*. And a **pack step is the wrong home, by the pack layer's own register**: `completion.yaml` ships into every adopter repo and the version it would name is jigc's own `Cargo.toml` — a law-1 lie for every reader who is not this repo. | fork · foreclosed-by-doc | decisions F3 · doctypes F-c | See **the EC-10 fork** below: narrow the recorded refusal with its rationale engaged (the repo's standing form) vs accept that EC-10 ships as prose a sixth time. | `crates/cli/tests/foldback_truth.rs:223`; `packs/methodology/workflows/completion.yaml`; `packs/methodology/steps/re-verify.yaml`; `crates/cli/tests/release_smoke.rs:100`; `grep -rl "rc\.14" crates/cli/tests/goldens/` → exactly 10 |
| **G-37** | **EC-14 is partly falsified, and shipping it as written would land a wrong "correction"**: *"eleven"* is **current** (the defect there is that the mirror is unfenced) and the two *"the window closes here — (M48)"* headings are **presentation, not a lie** (both paragraphs key the close to the 1.0 pin and each later spend is explicitly declared). Meanwhile two members are genuinely one-way: the frozen-schema doc a doctype author reads at the moment of bumping says *"ten shipped schemas"* against 17 entries, and both manifest headers restate one historical fact in two homes. | fork · cheap-vs-robust | decisions F10 · docs corrections 3/4/5 · doctypes F-c · doctypes A4 · baseline §3 | See **the EC-14 fork** below: rewrite the numerals vs fence-or-iterate them; and that **historical** counts are **dated-bracketed, not re-pinned**, which is the instruction a fixer told to "fix the counts" will otherwise violate. | `design/surface-contract.md:129`, `:131-142`; `crates/cli/src/invocation_log.rs:191`; `design/command-output-contract.md:446`; `design/doc-read-surface.md:90`; `design/corpus-migration.md:83`, `:281`; both `schema-manifest.yaml` headers; `crates/cli/tests/doctype_map_versions.rs`; `foldback_truth::dated_correction_spans` |
| **G-38** | **N23 / EC-38 — a doctype leaving the resolved set orphans its committed corpus.** Driven and re-confirmed: `validate` prints *"validates clean"* at exit 0, `doc list` drops the rows entirely, `git ls-files` carries every file. The consequence that prices it: a **false green on the verb `MIGRATING.md` tells adopters to CI-gate on**, and after 1.0 the reachable population stops being narrow, because PB-1 is the shipped, documented way for an adopter to own doctypes. | fork · cheap-vs-robust | doctypes F-b · capabilities G15 · charter Tier 1 | See **the N23 fork** below: leave it (narrow population today) vs a **deregistration** detect+route floor — the M38/M39 relocation floor applied to a doctype leaving the resolved set, joining `orphan.rs` and the M42 managed-vs-foreign discriminator. Argue it on leg 0's *known hole in a declared surface*, not on reversibility — a new code + route is additive. | driven at baseline4 §6, re-driven EC-38; control recorded (a project-*listed* pack defining no doctypes does **not** reproduce it); `crates/cli/src/orphan.rs` |
| **G-39** | **The razor's blanket refusal of frozen-schema changes may invert the human's own criterion.** For a **new** doctype the inference holds (*free at the freeze*); for a **shape change to an existing frozen doctype** it runs the other way — a bump today ships a migration over **zero** adopter corpora, the same bump after 1.0.0 runs over every adopter's store and flips their CI red until it does. | fork · foreclosed-by-doc | doctypes F-a | See **the razor-inversion fork** below. **This is a correction to the charter's stated ground, not a re-admission of F-3/F-8** — both may still be refused on the necessity leg; what must not stand is *"additive after the pin"* as the reason, because a future wave will cite it as precedent. | charter → *What it refuses by construction*; `design/corpus-migration.md`; `STORE_EXIT_FLIPS`; the methodology manifest header's own *"A NEW doctype is FREE AT THE FREEZE"* |
| **G-40** | **The adapter deny floor permits the one exit-0 byte-destructive path it exists to hold.** `permit: ["Bash(jigc:*)", …]` blanket-permits `jigc migrate <path> --as <T>` and `jigc task finalize <id> --approve` — the pair EC-1 proves deletes an unversioned external file and `.git/config` — while `uninstall` and `milestone discard`, which **refuse before destroying**, are denied. Separately, `e2e_audit::floor_patterns()` scrapes **20 of 22** entries because the loop `break`s on a **comment**, so the two M50 deny entries are invisible to the one test whose job is to check them. | fork · cheap-vs-robust | doctypes D3 · decisions F5 · baseline §3 | See **the deny-floor fork** below: whether the asymmetry is wrong (a third deny entry — profile bytes move, both inline snapshots redden, the entry must sit **above** the comment or the scraper still cannot see it) or right (in which case the reasoning belongs in the record; silence is a by-omission blessing). And whether the scraper is repaired rather than replaced by a `const`. | `crates/cli/adapters/claude-code.yaml:10`, `:32-34`, `:35`, `:36`; `crates/cli/tests/e2e_audit.rs:261-281` (scrape replicated, driven); `crates/cli/src/adapter.rs:1495-1547`, `:1569`, `:2622-2686` |
| **G-41** | **The `GIT_DIR` cell is foreclosed by a stated design constraint.** `repo.rs` deliberately refuses to shell out to git when `.git` is a *directory*, so *"a fake `.git` can never walk up to, and bind against, a real ancestor repo"*; ~15 unit fixtures depend on it. The discriminator that works needs exactly that query. The `dirname(common-dir) != toplevel` shortcut is **not** a discriminator — a legitimate linked worktree has the same asymmetry. | fork · foreclosed-by-doc | capabilities G2b · capabilities H4 | See **the GIT_DIR fork** below: re-scope `repo.rs`'s rationale (discriminate the fixtures by a marker) vs **declare the cell out with the quote**. Leaving it unmentioned is the one arm the record forbids. | `crates/cli/src/repo.rs:1-16`, `:26`; driven `rev-parse --git-dir` / `--show-toplevel` asymmetry in capabilities G2b |
| **G-42** | **Three unpinned-shape cells that the pin will freeze by omission**: `task list --format json` is a **top-level array** (no `schema_version`, no `findings`, no discriminator); `milestone execute` is a **fourth** producer of the composed `{task, text}` shape against the contract's *"Three verbs"* — the code-side census says so **in its own words** while the doc does not; and `milestone list-tasks`, a `VerbKind::Read` verb, ships `hook_output`, structurally always `""`, outside that key's declared scope. Neither `migrate` nor `milestone execute` carries a closed-key assertion. | fork | decisions B6 · decisions F9 · docs B4 · docs F-d row 1 · capabilities H5 · baseline cells 7/8/9 | See **the three-cells fork** below. Each needs its own answer: pin the array shape or state it unpinned · the doc takes the registry's membership (the registry already settled it; this is not a free choice) · the read verb drops the key or its declaration widens. | `crates/cli/src/render.rs:3285`; `crates/cli/tests/text_json_parity_axis.rs:385-390`; `crates/cli/src/cli.rs:1459`; `design/command-output-contract.md:19`; all three driven at baseline `[envelopes §1]` |
| **G-43** | **EC-7 is narrowed in one direction and widened in another, and neither probe applied a mutation.** The whole floor **is** independently pinned twice as inline `insta` literals in production source (22 patterns, not 20), with live evidence of the coupling firing — so the residual gap is **deliberate regeneration** (`cargo insta accept`), the compose-goldens posture, not an accidental drop. The new half is the scraper truncation in G-40. | advisory | decisions F5 · doctypes D3 · baseline §3 | Whether EC-7 survives at all after one applied mutation each (the baseline says in its own words that this row deserves one before it is acted on); and if it does, that the fix shape is the **shipped** one — extend the test that asserts literals against the **loaded** profile, rather than minting a production const that becomes a second source of truth. | `crates/cli/src/adapter.rs:1495-1547`, `:1569`, `:2622-2686`; `.adapter.rs.pending-snap` (gitignored, 318 KB); `crates/cli/tests/e2e_audit.rs:261` |
| **G-44** | **EC-20's fix is an additive key with the value already in scope**, and the order is key-then-fence: the findings are computed **above** the `if dry_run` branch and discarded, and `GATE_COVERAGE`'s fence says in its own words that it checks what surfaces *say*, never what a door *emits*. Moving the previewed set touches **eight** enumerating sites, **two of them composed pack steps**, and the composed *what's-left:* line is already generated. | advisory | capabilities G9 · doctypes A8 · baseline §3 | Whether `--dry-run` gains `findings` (an additive key inside the open window) and whether the behavioural arm — drive each `Tier::Previewed` member, assert the emitted `code` set is equal across `task validate` / `--dry-run` / landed — ships with it. | `crates/cli/src/render.rs:1887`, `:1894-1899`; `crates/cli/src/task.rs:1827`, `:1677`; `crates/cli/src/gate_coverage.rs:43-49`, `:268`; `crates/cli/tests/gate_coverage_fence.rs` |
| **G-45** | **EC-4/EC-16: the relocation is not a value of the acked enum, and a test pins the false phrase.** `route_docs_root_repoint_orphans` returns `()` and narrates via `eprintln!`, while `config_ack_parity` destructures `ConfigAck::Set` **exhaustively** over `{key, value}` — structurally blind. The over-claim has **five** homes, one of them a test asserting the phrase is present, so the fix is a five-site sweep including a red-then-green on a passing test. | advisory | capabilities G14 · docs A3 · baseline §4 | Whether `ConfigAck::Set` gains `relocated` (the exhaustive destructure then forces the key) and whether the honest key set is `{relocated, staged}`, since the door also stages an index change. | `crates/cli/src/config.rs:751`, `:781`, `:800-804`, `:846`, `:317`, `:66`, `:782`, `:878`; `crates/cli/src/render.rs:2541`, `:3351`; `crates/cli/tests/unmanage.rs:290-293` |
| **G-46** | **EC-18: no line-merge primitive exists**, the amend has **four** production callers, and the repo's three amend-or-refuse precedents are all other shapes. A union is ~6 lines but must be byte-idempotent across all four callers (or `finalize` starts committing a churning file), and today the file is exactly `ENTRIES` in order, so any fixture asserting the bytes is re-derived. | advisory | capabilities G10 · docs F-d row 10 | Amend or refuse-and-route for a file the user legitimately co-owns; and — independent of the merge — that the ack currently names no content change at all, which is EC-18's law-1 half. Note `project-setup.md` states the principle at the **sibling** root file and the workbench file has no rule. | `crates/cli/src/gitignore.rs:28`, `:32`; callers `adapter.rs:1108`, `task.rs:2778`, `milestone.rs:472`, `milestone.rs:2066`; `design/project-setup.md:103-106`; `design/storage.md:116` |
| **G-47** | **N20's routing fix is two lines; the `survived:` clause is the real work.** A fully-populated `RejectionFrame` is already in the caller's hand at all seven construction sites and is discarded at exactly one branch — but the clause is written for the *commit-rejected* case, and a **non-hook** failure can occur earlier or later in the same door, where it may be false. | advisory | capabilities G11 · decisions (charter N20) · docs correction 10 | The axis is `COMMITTING_DOORS × {hook rejection, non-hook failure} × {is the survived clause true?}` — ten doors × two — and a naive reuse of one clause for both cells ships a law-1 lie at the moment a user is recovering. | `crates/cli/src/task.rs:3533`, `:3502`; sites `cli.rs:796`, `migrate_corpus.rs:303`, `milestone.rs:408`, `:4094`, `:4166`, `task.rs:554`, `:2063`; driven at baseline cell 11 and `[doors §2(c)]` |
| **G-48** | **`jigc ingest`'s identity leg: the predicate exists, the route has no verb behind it.** The classifier asks `is_slug` and the **gate never does** — the two run in different modules over the identical derivation — so `ingest` claims adoption, `doc list` says `unregistered`, `doc show` refuses, and `validate` exits 1 routed at **`jigc ingest`**: a route that, followed exactly, changes nothing. | advisory | capabilities G12 · baseline cell 14 | One `is_slug` call at the ingest gate (engine-side, no new capability) plus a finding code and a route — and the honest route is a **`Human` route naming `git mv`**, since `doc rename` requires the doc be managed and this one by definition is not (M45's owner-artifact `git add` route is the precedent). The carried entry names only the registration. | `crates/engine/src/validate.rs:909-918`; `crates/engine/src/ingest.rs:226-229`; `implementation/decisions-pending.md:593`; driven at baseline `[tokens §5]` |
| **G-49** | **`jigc config insert-step … <file>` / `replace-step <target> <file>` read an arbitrary host file into a committable, composed step.** An absolute path, `../rel.yaml` and `.git/config` all land at exit 0 in `.jigc/config/steps/<stem>.yaml` and then render into `jigc start`'s step text. `untrackable_reason` asked of the **source** answers exactly this question and is already CLI-side in `config.rs` — one call, **verified-reuse**. | advisory (Tier-0 by the baseline; the tier is the Settle's) | decisions F14 · capabilities G1 · baseline Tier-0 cell 2 | Whether fork 1's predicate binds `file`/`from_file` — a **read escape into the agent's context**, not a deletion — which is exactly fork 1's unstated content; and whether the row is priced as escape (Tier 0) or as surface (Tier 1), but not as a different *fix*. | driven at baseline `[tokens §2d]` (git's own `repositoryformatversion = 0` rendered into a workflow); `crates/cli/src/trackable.rs:55` |
| **G-50** | **The sub-task `resume:` line promises a resume the state cannot serve** — a law-1 wording item, not a capability, and the composed footer names no worktree while the `Spawn:` line does. Three real states, not one (see *Corrections*). | advisory | capabilities G13 · doctypes A6 · baseline cell 13 | What the `resume:` and `Run:` lines say, and whether a `partial_worktree_advisories`-shaped per-sub-task advisory rides the `base == HEAD` compose. The layer is `render.rs::task_state_lines` + `start.rs`'s guard predicate — **not** a pack change and **not** a schema change; no pack-load fence has to learn the provisioning fact. | `crates/cli/src/render.rs:480`, `:466-470`, `:119`; `crates/cli/src/start.rs:1937`; `crates/cli/src/milestone.rs:2895`; all three states driven (capabilities G13, doctypes A6, baseline `[doors §3]`) |
| **G-51** | **`jigc task discard`'s own ack names no commit** — HEAD moves and the success line says only *"discarded task …"* where every other committing door prints its sha. EC-12 fixes the **guides**; the **door** is unlisted. | advisory | decisions F14 · capabilities G15 · baseline cell 12 | Whether the door joins the fix or the wave states that the guides are corrected and the ack is not. | driven at baseline4 §3 / cell 12 (`520283b → 1152124`); the sha is produced by the same `git_commit_pathspec` → `git_commit_capture` chain (`milestone.rs:820`) and discarded |
| **G-52** | **EC-11's truth-fence menu: two of three cases have no code-side set to generate from.** `migrate-corpus --help` can ride `SchemaChangeKind::ALL` (**verified-reuse**); `validate --help`'s *"code anchors"* names the **five store probe families**, which exist only as numbered comments and prose (`STORE_EXIT_FLIPS` is the wrong set); `doc show --help`'s four-key shape needs a const minted **and made load-bearing at the render site** or it is a second home for one fact. `{{schema:<T>}}` and the slot-ceiling statements reach **no** Tier-2 row — the available seams are generated `long_about()` and the M7 fence mold. | advisory | capabilities G8a · doctypes A2 | Whether the wave mints the probe-family registry (a new registry, not a reuse) and the `doc show` key const, or states those two help texts as un-fenced prose; and that `task finalize --help`'s three-gate claim can ride `whats_left_coverage()`, which **already generates that exact sentence**. | `crates/engine/src/slug.rs:139` + `crates/cli/src/doc.rs:169` (the proven mold); `crates/engine/src/schema_diff.rs:757`; `crates/engine/src/validate.rs:477/:496/:516/:580/:607`; `crates/cli/src/doc.rs:5070-5073`; `crates/cli/src/gate_coverage.rs:268`; `grep -n "PROBE_FAMILIES\|ProbeFamily"` → 0 |
| **G-53** | **EC-22/EC-23 mint before they fence.** `ManifestKind` has six members and **no `::ALL`**, so the fix's first act is minting the registry; `CorpusMigrationReport`'s field names ≠ wire keys (`no_commit`/`unlanded` are `#[serde(skip)]`), so the fence needs a stated per-field disposition — which `text_json_parity_axis`'s `Disposition` already models. | advisory | capabilities G8b · doctypes A2 | Whether both registries are minted in this wave, and that the guide sentences they fence are the shipped-guide ones (**MIGRATING/QUICKSTART are the SKILL.md body by construction**). | `crates/cli/src/render.rs` (`ManifestKind`, `ConfigAckArm::ALL` at `:2589`); `crates/cli/src/migrate_corpus.rs:120`; `crates/cli/src/lib.rs:41`; `MIGRATING.md:35` |
| **G-54** | **Every Tier-1 fix that adds a key owes a declared paragraph**, and four fixes in this wave add keys with none enumerated in the charter: `findings` on `--dry-run`, `relocated` (and possibly `staged`) on `ConfigAck::Set`, EC-3's declarations, and any log version. Separately, **EC-5's robust arm reads two ways** — *state the rule* vs *add `schema_version` to the ~40 envelopes that lack it* — which differ by an order of magnitude in cost. | advisory | decisions F11 · capabilities G3 | Which reading of EC-5's robust arm the wave takes, and that each added key ships with its declared paragraph in the form the contract requires. | `design/command-output-contract.md:446`; `crates/cli/src/task.rs:1827`; `crates/cli/src/config.rs` (`ConfigAck::Set`); 7-of-48 versioned arms driven at baseline `[envelopes §1]` |
| **G-55** | **Every Tier-2 guide edit has an adopter cost the charter does not price**: both guides are `include_str!`'d into the installed `SKILL.md`, so **any** guide byte moves `jigc-body-blake3`; `jigc setup` refuses to clobber an edited guide and `jigc upgrade` never re-installs — so the batch reaches only adopters who re-run setup and never edited it. And `unlink_in_repo_links` **flattens relative links**, so a fix paragraph cannot cite `design/` — it must be self-contained prose. | advisory | decisions F7 · doctypes A3 | Whether the Tier-2 edits are batched into **one** hash move, whether anything tells an adopter it moved, and that every added paragraph is written self-contained. | `crates/cli/src/setup.rs:63`, `:69-70`, `:95-96`, `:111-148`, `:243`; `crates/cli/src/upgrade.rs:119-136`; `crates/cli/tests/adapter_artifact.rs:449`; `implementation/decisions-pending.md:260` |
| **G-56** | **`design/measurement.md`'s invocation-log record shape is wrong in two fields** — `duration` vs `duration_ms`, and *"finding_codes (on failure)"* against a key that is **always present** (`[]` on success). It is the one doc paragraph EC-6's fence would be written against. | advisory | docs A1 · docs N-c | Whether the paragraph is corrected in the same motion as the EC-6 decision, since the fence's subject is that sentence. | `design/measurement.md:62`; driven (knob on, `jigc describe`): `['argv','binary_version','duration_ms','error_code','exit_code','finding_codes','output_bytes','timestamp']` |
| **G-57** | **The remaining Tier-3 capability cells, each with its predicate status**: EC-28's OS name ceiling is **absent** as a predicate and the class is **3 doors** with a false route (`doc create --slug` was **refuted** as refusing correctly); N15's correct shape is *three inches away* in its sibling `store.not-staged`; EC-25's echoed-vs-parsed token is **≥3 doors**, and only the message renders the normalized form while `at:` already carries the typed one. | advisory | capabilities G15 · baseline §3 · baseline §4 | For each: whether it is fixed here and at which door set — in particular whether `SLUG_DOORS` gains a filesystem-length predicate (a filesystem fact the slug grammar does not encode) with a code and a route. | `crates/cli/src/cli.rs:1978` (`SLUG_DOORS`); `crates/cli/src/config.rs:677`; driven at baseline `[tokens §2g, §2h, row 16]`, `[prose §6]` |
| **G-58** | **The razor's stated refusal class names verbs, schemas and execution shapes — not flags**, while **both** robust arms in scope add a consent flag. A reader could import the class and refuse the robust arm on a mis-read razor; the precedent runs the other way (M48 fork 1 flagged *"add a flag"* as *precisely the cheap framing* and demanded an independent robust-advocate). | advisory | decisions F12 | One sentence at the Settle stating that a flag is admissible **and owed the robust-advocate treatment**. | charter → *What it refuses by construction*; M48 fork 1 in `DECISIONS.md` |
| **G-59** | **F-9's admission is a fired trigger, not a fresh choice.** Its recorded trigger is *"the next wave that opens `doc rename`'s ack or `finalize`'s pre-commit manifest — or a second trial arm observed leaving the adapter"*, and this wave opens both. | advisory | decisions F13 | Record it as a **fired trigger** rather than as a scope call, so the ledger's discipline holds and F-10's parked trigger stays legible beside it. | `implementation/decisions-pending.md:223`, `:14`, `:5` |
| **G-60** | **The *"Notation is illustrative"* header covers 16 of 28 `design/` docs and disclaims notation only** — every rule this wave cites is prose and citable — but **nothing says so at the point of use**, so a fixer handed *"correct `finalize.md:31`"* can reach for the header. The three pinned-contract docs carry **no** disclaimer, which is the asymmetry worth naming once. | advisory | docs A8 | One sentence at the Settle so the razor's leg 1 is not argued per row. | `CLAUDE.md` → *How we work together*; the 16 disclaimered docs enumerated in docs A8 |
| **G-61** | **The adopter-doc edit set, consolidated** — must **gain**: the ahead-stamp paragraph (a `STORE_EXIT_FLIPS` member that flips an adopter's CI, documented only where `design/` never ships) · the `--no-verify` scope · the `task discard` sub-task qualifier · a versioning/compat sentence · the two missing `migrate-corpus` triage keys (`unadopted`'s emptiness **is** the exit rule) · `ManifestKind`'s fifth and sixth members. Must **lose or be qualified**: QUICKSTART's *"prints the manifest and stops, changing nothing"* (the correct sentence exists in the **other** file) · its presentation of `task validate` and `--dry-run` as one preview surface · MIGRATING's *"All nine committing doors"* · the duplicate install stanza (three paths across two documents, breaking *one fact, one home* three ways). | advisory | docs A9 · docs B10 · docs B7 · docs B8 | The batch's membership and its single hash move (G-55). | `MIGRATING.md:35`, `:39`, `:40`, `:47`; `QUICKSTART.md:19-20`, `:160-161`, `:165`, `:174-175`, `:180`; `CLAUDE.md:10`; `design/validation.md:453` |
| **G-62** | **`foldback_truth.rs`'s M50 fence must be re-aimed and inverted twice** — to `**M51 —` and back to the pre-audit direction at the build close, then inverted again at the completion fold-back — and its own doc-comment records that this was done **late** last time, staying red through two commits while the handover recorded a green gate. Its `1.0.0 is called` / `1.0.0 shipped` prohibition is the one to watch if the human takes the call after M51. | advisory | doctypes A5 | Name the re-aim in the close increment rather than discovering it at the gate. | `crates/cli/tests/foldback_truth.rs:242` and its doc-comment |
| **G-63** | **Three internal docs a session reads at the moment of doing the thing they describe are stale**: `doctype-authoring.md` still carries a *"Reading this before rc.6 ships"* caveat (seven waves ago) directly above the transform-kind matrix; `pinning.md` still claims `lib.rs` exposes neither the pack registry nor the clap tree (both `pub` since M45); `dev-workflow.md` heads *"all four pass"* over five bullets and states an order the gate does not run. | advisory | docs A5 · docs A6 | Whether they ride the Tier-2 batch (they are carried as (j)/(k)/(m) already). | `implementation/doctype-authoring.md:30`; `implementation/pinning.md:15`; `implementation/dev-workflow.md:13`, `:22`; `implementation/decisions-pending.md:144-156` |
| **G-64** | **`MIGRATING.md` ships a machine-dependent performance bound into every adopter repo** (*"the cost is quadratic in the item count"* plus wall-clock figures) — unpinnable by construction. | advisory | docs A7 | Whether machine-dependent numbers belong in the shipped artifact. Not a fence question. | `MIGRATING.md:47` |
| **G-65** | **EC-9's root `CHANGELOG.md` sits at a frozen placement doctype's canonical home — and the two probes disagree about whether that matters.** `changelog` is frozen at `placement: { file: CHANGELOG.md }`; the decisions probe reads a hand-authored root changelog as a foreign file at jigc's own managed home, the doctypes probe drove that **this repo does not self-host**, so the home is never met and no fence reddens (though a root `README.md` explaining the `code-anchor` grammar **would** become an undeclared site of it). | advisory | decisions F8 · doctypes A9 | If EC-9 survives G-28: author the changelog in the doctype's conformant shape, or knowingly foreign — **stated either way**, since the file's home is not a free choice; and whether a README that explains anchors joins `SOURCE_SITES`. | `CLAUDE.md` → M38; `design/storage.md` → Placement; `crates/cli/pack/schemas/changelog.yaml`; `code_anchor_grammar_sites::SCANNED_ROOTS`; `implementation/decisions-pending.md` → *The doctype-completeness milestone* |
| **G-66** | **The Tier-4 strike sites are derived, and three of the charter's own cites are wrong** — EC-14's *"ten"* is at a different line than cited, the N20 entry does not resolve at the cited line, and the arm count disagrees with itself (23 vs 24). Any strike must also be checked against `foldback_truth.rs`, which fences the `**M50 —` span of `CLAUDE.md`, the MIGRATING back-out ladder's numbering and per-prose-unit token presence in both shipped guides — **the suite will redden on adopter-guide edits and on the CLAUDE.md span, by design**. | advisory | docs strike table (EC-31…EC-43) · docs corrections 3/10 | That the Tier-4 edits re-derive their line cites before editing, and that the `CLAUDE.md`/`roadmap.md` restatements of the same claims are swept with them. | full strike table in [gap-docs.md](gap-docs.md); `crates/cli/tests/foldback_truth.rs` |
| **G-67** | **Fork 2's exemption question is already answered by the baseline — as a flag, not a probe.** Every fan-out sub-task worktree and the `DedicatedWorktree` both boundary arms commit in answer `git -C <wt> symbolic-ref -q HEAD` with **exit 1**, so a HEAD probe cannot distinguish the by-design detachment from a user's. | advisory | decisions F2 · baseline §4 | Record it as **measured** rather than re-posing it in the fork. | driven in decisions F2 / baseline `[doors §3]`; `crates/cli/src/task.rs:4581` (`git worktree add --detach`) |
| **G-68** | **EC-8's probe-wire half is a recorded deferral whose basis still holds**, so it is a deferral to **re-affirm with its citation**, not a hole to fill: the versioning policy is deferred under the trusted-pack model, and the cross-pack probe-collision trigger (*a second pack ships its own `field-types.yaml` or probe*) is **unfired** after PB-1. Only the **binary** semver policy is genuinely absent (G-29). | advisory | decisions F6 | Re-affirm with the citation rather than re-deciding. | `design/validation.md:167`, `:187`; `design/multi-pack.md:155` |
| **G-69** | **A workflow `description:`/`usage:` edit moves the six `describe--*` and ten `start-orient*` goldens**, and a new pack step would move six `workflow-preview--completion--*` goldens and the step count. Routine — but discovered at the gate if not named. | advisory | doctypes A9 · doctypes F-c | Name the regeneration in the close increment. | `crates/cli/tests/goldens/compose/methodology/workflow-preview--completion--*.txt` (6); `grep -rl "rc\.14" crates/cli/tests/goldens/` → 10 |

---

## The forks for Settle

The charter's **nine forks**, restated as the probes amended them, then the **new** forks the probes
raised. Each states the cheap and the robust arm **neutrally**, with the one-way-door or
declared-hole tell named. **No recommendation is carried here** — a robust-advocate argues the
robust side at Settle against a proposer who did not author it.

### The charter's nine, as amended

**Fork 1 — EC-1, the shape of the path guard.** The charter already recorded the cheap arm as
falsified by the baseline (a door-only guard cannot close the class: a benign token at the door,
a rewritten `source-path`, `--approve` at exit 0, an arbitrary host file gone). The probes amend it
three further ways. **Cheap (as it now stands):** *door + sink* — the door adjudicates and
`plan_retirements`/`task::retire` re-validate the recorded token. **Robust:** *door + sink + the
classified `Plain`-family registry*, one stated predicate per member (the five members were driven
and ask **four different questions**). **Amendments:** (a) the sink predicate cannot live in the
engine as stated — `untrackable_reason`'s legs are shell-outs and the engine has no subprocess and
**no symlink check at all** — so the fork carries a sub-choice, *lexical engine predicate (cannot
answer the symlink cell)* vs *the CLI validating `plan.retirements`* (G-2); (b) the untrusted token
has **five** raw consumers, two of which buy a **carryover-gate exemption**, so a guard at
`plan_retirements` alone leaves four sites reading raw (G-2); (c) `file`/`from_file` — the
`config insert-step` read escape — is the fork's unstated content and must be answered inside it
(G-49). **Tell — declared hole:** leg 0's second clause admits the robust arm **on the evidence**,
not on preference: the cheap cut ships a known data-loss cell reached through a door whose argument
was benign. **Not a one-way door at the pin.**

**Fork 2 — EC-2, refusal vs consent, and the exemption.** **Cheap:** narrate the detached HEAD and
let the door proceed under a stated consent. **Robust:** refuse at the shared seam. **Amendments:**
(a) *there is no shared seam* — `discover_repo_root` is copy-pasted six times and `setup` bypasses
`git_commit_capture` entirely, so "the shared seam" is a premise, not a location (G-7); (b) the
subject is a **family of four predicates**, of which one probe answers one cell and false-greens two
(G-8); (c) the exemption **cannot be sniffed** — `DedicatedWorktree` reaches the seam as a bare
`&Path`, and a path-shape guess is the class M50's audit condemned, so the exemption must be a
**typed argument** (G-9); (d) the exemption question itself is **already measured** — a HEAD probe
cannot distinguish by-design detachment from a user's, so it must be a flag (G-67); (e) posture is
not only *is HEAD attached* but *which repo is this door acting on* (the linked-worktree split); (f)
the `GIT_DIR` member is foreclosed by `repo.rs` (its own fork below). **Tell:** shipping the
one-liner and calling the exemption class closed puts a **false completeness claim** on the record —
the exact failure the wave exists to correct.

**Fork 3 — EC-29, what the transaction promises.** **Cheap:** keep the declared index-only fidelity
and correct the *"all-or-nothing"* header. **Robust:** rollback restores the **worktree** too, over
the fifth un-swept dimension. **Amendment:** the cheap arm's premise is false as the charter states
it — *"worktree untouched"* is a clause about the **restore's mechanism**, not a declaration that
the worktree keeps a rewritten `.jigc/version`, and every sibling row explicitly restores the
worktree — so **the cheap arm is not *keeping* a declared decision, it is declaring a new one**, and
`DECISIONS.md`'s M45 sentence (*"every path jigc's own staging contributed"*) moves with it (G-18).
**Capability asymmetry to price:** index rollback rides captured pre-images across four aligned
families; a **worktree** rollback for the config layer has **no capture at all** today, so "robust"
here is a new capture family, not a reuse. **Tell:** not a one-way door at the pin; the decision is
which sentence 1.0 ships as the promise.

**Fork 4 — EC-26, `setup` and the carryover gate.** **Cheap:** `setup` narrates what it swept.
**Robust:** `setup` **joins the carryover gate** — the gate's subject becomes the door.
**Amendments:** (a) the robust arm as posed is **refused by construction** — `COMMITTING_DOORS`'
predicate is a hook-capable commit and `setup` commits `--no-verify` by recorded design (its
rejection already has an identity, `setup.install-commit`), while `MINT_DOORS`' subject is a
working-area mint; so the robust arm needs a **third home** and the fork as posed conceals that
(G-12); (b) the gate's subject is **stated as a task-minting door in two locked homes**, so the
robust arm revises them or declares `setup` outside the gate (G-12); (c) two capabilities are
genuinely needed — a third `CarryoverBoundary` variant and a **pathspec-scoped** snapshot, without
which the gate blocks `setup` in any dirty tree (G-14); (d) the row carries a **leg-1 citation the
charter does not use**, which moves it out of Tier 3 (G-13). **Tell:** a committing door that
swallows uncommitted user work at exit 0 is a hole in a **declared** surface, not a capability
addition.

**Fork 5 — F-5 / F-11, code and route vs the envelope.** **Cheap:** a code and a route inside the
flattened `{"error": …}` string. **Robust:** the findings envelope at those doors — a wire change on
a de-facto-pinned shape. **Amendment — the cheap arm's cited precedent does not transfer:** the M50
refusal is grounded on `(code, null)` for *a token that names nothing*; F-5's subject is a **work
unit**, whose target form the contract **already declares**, and the engine **already ships**
`finalize.no-task` — the CLI short-circuits it one layer above in one shared home as a code-less
`anyhow`, with M50 Increment 9's `Result<_, Finding>` shape available so a bare `anyhow` cannot
inhabit it. **Re-posed:** *make the declared contract true at these doors* vs *declare the doors
permanently outside the envelope* (G-17). **And the contract's own claim about the flattened text is
false at those doors** (G-16), which is a razor leg the charter does not cite. **Tell:** robust is
cheap only before the pin; but the re-posed cheap arm is no longer *string-vs-envelope*.

**Fork 6 — EC-8 / EC-9, the versioning policy and the publishing floor.** **Amendments:** (a) EC-9's
premise is **contradicted by a dated human decision taken twice**, and the charter's own exclusions
place it out of scope — so the fork's first question is whether the human **reverses** that decision
on a changed basis (G-28); (b) EC-8's **home is forced** by an existing rule — `design/` never
ships, so anything an adopter must read lives in the two guides that are the `SKILL.md` body, and
**every** guide edit moves `jigc-body-blake3` (G-29, G-55); (c) the probe-wire half is a **recorded
deferral whose basis still holds** and is to be re-affirmed with its citation, not filled (G-68).
**Tell:** these are human decisions, not code; what is fenceable is the **home**, not the content.

**Fork 7 — EC-3 / EC-5, declare or delete.** **Cheap:** declare the four keys and state the
versioned/unversioned rule as a sentence. **Robust:** delete what should not have shipped and close
the partition against the clap leaf tree. **Amendment — neither arm can range over anything until
the pinned set exists** (G-33): the contract names **three** surfaces while ~14 shipped envelopes
ride in none of them, so `guide_file` is "a defect" only because `hook_file` beside it happened to
get a paragraph while `identity` on `unmanage` is unremarked — **the doc's framing, not the binary,
is what makes EC-3 a four-key finding instead of a twenty-one-key one**. Two structural costs the
charter does not price: the recipe table's bijection is *one recipe per leaf verb* and the real
splits are per **arm**, so the duplicate-path guard — that test's load-bearing half — must be
reshaped; and a `keys:` column written from driven output **blesses all 17 undeclared keys** on the
day 1.0.0 lands, converting *delete them* into *a versioned extension to delete them*, by the
contract's own rule. **Tell — one-way door at the pin, by the charter's own leg-0 test.** Whatever
is written here **is** the 1.0 contract, not a test over it; and `ManifestKind::ALL` by contrast is
**not** a one-way door (an internal enumeration whose only consumer is a prose fence) — the two must
not be priced the same.

**Fork 8 — the ledger question.** Unamended by the probes. Read closed under the stated criterion
(every row carries `pinned-by:` or a stated `UNPINNED:`) or substantively open (F-3, F-5, F-9, F-11
are CONFIRMED defects with no standing test). **The gate is the human's by the record's own words.**

**Fork 9 — the acceptance instrument.** Unamended in substance; **one consequence surfaced**: the
per-axis review's own fence (*every leaf in `VERB_KINDS` appears in at least one axis's matrix or is
named uncovered*) is what the `acceptance-spiked` gate-record slot must be filled against, and
fork 9 being open means **that cell cannot close before the Settle resolves it** (see *Owed at the
gate-record*).

### The new forks

**EC-6 — version the log vs declare it unversioned.** **Cheap:** add a version integer and close the
key set. **Robust (in the other direction):** **declare the log unversioned** and close the key set,
because the log is opt-in, gitignored, rebuildable and consumed by our own scripts — it fails leg 0
as a one-way door — while *adding* an integer **creates** a pinned surface at the moment the wave is
closing the pin. **Amendment:** EC-6's cited basis is a misreading (*independently versioned* means
independent **of each other**), and the M44 `task_id` deferral rests on *rebuildable + purely
additive*, not on an integer, so its basis is untouched either way (G-34). **Tell — the charter's
Tier-1 framing (*cheap now, expensive after the pin*) presumes the opposite without argument.** The
doc paragraph the fence would be written against is itself wrong in two fields (G-56).

**EC-14 — fence-vs-rewrite.** **Cheap:** rewrite the numerals. **Robust:** fence or iterate them —
the repo already ships the mechanism (`doctype_map_versions.rs` reading the registry and asserting
the doc's rows). **Tell:** choosing *rewrite* re-buys the same defect at the next bump, and two
members are one-way in practice — the doc a doctype author reads **at the moment of bumping a frozen
schema** states the freeze's own scope wrongly, and both manifest headers restate one historical
fact in the two files an author edits when the count moves. **Constraint on either arm:** the
**historical** counts must be **dated-bracketed, not re-pinned** (`dated_correction_spans` is the
shipped discipline) — a fixer told to "fix the counts" will mechanically re-pin history. **And two
of EC-14's members are falsified as stated** (G-37), so shipping it unamended lands a wrong
"correction".

**N23 — the deregistration route.** **Cheap:** leave it; the reachable population today needs
`JIGC_PACK_DIR` or a PB-1 project pack dropping a doctype it owns. **Robust:** a doctype leaving the
resolved set **routes its orphaned instances** — the M38/M39 detect-route floor applied to
deregistration rather than relocation, joining `orphan.rs` and the M42 managed-vs-foreign
discriminator. **Tell — a known hole in a declared surface (leg 0's second clause), not a one-way
door**: a new code plus a route is additive, so it must be argued on the hole, and the hole is a
**false green on `jigc validate`** — the verb `MIGRATING.md` ships into every adopter repo telling
them to CI-gate on it. **The long-run half:** after 1.0.0 the population stops being narrow, because
PB-1 is the shipped, documented way for an adopter to own doctypes (G-38).

**`AMBUSH_CLASS_CODES` — derived vs listed.** **Cheap:** hand-add this wave's new blocking contracts
to the const. **Robust:** derive the owe-set, so a new ambush-class contract cannot stay off it.
**Tell — silent by construction:** nothing reddens when a code stays off a hand-list, and **N26's
recorded trigger fires verbatim at this Settle** (*"the stated-at fence's owe-set is next opened"*),
carrying its own two questions: is the fence's subject *packs that ship steps* rather than *packs
that ship a manifest*, and does an unstamped managed corpus deserve a store-surface answer. **A
structural constraint on the cheap arm:** `finalize.carried-staged`'s declarer is `step:finalize`
and **no pack step of either pack solicits `jigc setup`**, so a setup-side carryover contract has no
home the fence's own route can name (G-21, G-35).

**EC-10 — a fence vs a pack step.** **Cheap:** prose again (five consecutive waves). **Robust:** a
fence on the weakest checkable claim — *the version this fold-back names is the version `Cargo.toml`
carries*. **The pack-step arm is refused with its ground:** `completion.yaml` ships into every
adopter repo and the version it would name is jigc's own `Cargo.toml`, a law-1 lie for every reader
who is not this repo; `re-verify.yaml` already models the right register. **Tell —
`foreclosed-by-doc`, narrowly:** `foldback_truth.rs:223` declines the version assertion **by name**,
and that rationale holds for choosing a **numeral** and is silent on comparing two homes — so the
robust arm is a **narrowing of a recorded refusal with its rationale engaged**, the repo's standing
form, not an override (G-36). **Side effect to price:** if the fix were a new pack step the tree goes
to 70 and G-31's fence cell disappears by accident.

**The razor's *"additive after the pin"* — an inversion for frozen-shape changes.** The charter
refuses *any schema-shape change to a frozen doctype* as a class on the ground that it is *additive
after the pin*. For a **new** doctype that holds (the methodology manifest says so in its own words);
for a **shape change to an existing** frozen doctype it runs the other way — the bump today ships a
migration over **zero** adopter corpora, the same bump after 1.0.0 runs over every adopter's store
and flips their CI red until it does, which is textbook *expensive after the pin*. **This is a
correction to the charter's stated ground, not a re-admission of F-3/F-8** — both may still be
refused on the necessity leg (*no adopter needs it today* is a genuine leg-0 refusal, and the M50
Settle refused the same edge once on `methodology-docs.md:42`'s universe rule). **Tell:** left
unstated, a future wave cites the inversion as precedent (G-39).

**The deny floor's asymmetry.** **Cheap:** leave the profile as it is. **Robust:** a third deny entry
covering the one exit-0 byte-destructive path — `jigc migrate` + `jigc task finalize --approve` —
which today rides the blanket `Bash(jigc:*)` permit while `uninstall` and `milestone discard`, both
of which **refuse before destroying**, are denied. **Cost of the robust arm, measured:** profile
bytes move, both inline `insta` snapshots redden (inline, re-acceptable), the SKILL.md/settings.json
e2e arms re-run, and the entry must sit **above** the profile's comment or the scraper still cannot
see it. **Tell:** the floor's stated rule is *"a refusal an agent can consent past is not a floor"*
— if the Settle decides the asymmetry is right, **that reasoning belongs in the record**, because
silence here is a by-omission blessing of the cheap read (G-40). **Adjacent and independent:**
`floor_patterns()` scrapes 20 of 22 because it `break`s on a comment, so any entry added below a
comment is invisible to the one test whose job is to check it.

**The `GIT_DIR` cell.** **Cheap:** declare it out **with `repo.rs`'s rationale quoted**. **Robust:**
re-scope that rationale — discriminate the ~15 fake-`.git` fixtures by a marker rather than by *never
ask git when `.git` is a directory* — and close the cell. **Tell — `foreclosed-by-doc`:** the
rationale still holds (test isolation is real), but its **scope** is reopenable, and the shortcut
that avoids reopening it (`dirname(common-dir) != toplevel`) is **not a discriminator**, because a
legitimate linked worktree has the same asymmetry. **The one arm the record forbids is leaving it
unmentioned** (G-41).

**The three unpinned-shape cells.** Three separate decisions, posed together because the pin closes
over all three at once. **`task list --format json` is a top-level array** — pinning it freezes an
envelope a driver cannot deserialize uniformly; *not* pinning it must then be **said**.
**`milestone execute` is a fourth composed producer** — and this one is **not a free choice**: the
code-side registry names `render::composed` explicitly and settled it; the doc was never updated
(G-42, and *Contradictions* row 1). **`milestone list-tasks` ships `hook_output` on a read verb**,
structurally always `""`, outside that key's declared scope — drop the key or widen the declaration.
**Tell:** all three are one-way at the pin in the same way fork 7 is, and none carries a closed-key
assertion today.

---

## Corrections to the inputs

Every correction the probes made to the charter or the baseline ledger, deduped, each with the datum.
**The repo's rule binds: a stale claim is struck with the datum that falsifies it, never silently
rewritten.**

**To the charter**

1. **EC-9's *"fires by definition"* is contradicted by a dated human decision**, taken 2026-07-16 and
   re-affirmed by citation at the M46 Settle: *"**DECIDED 2026-07-16 (human): post-v1.** 1.0 ships
   internal … fires **after** the 1.0.0 call, not before it."* Two of EC-9's four facts are also
   stale — `Cargo.toml:15` already carries `publish = false` and `:13` the license.
   `implementation/decisions-pending.md:500-502`, `:419`, `:655`. *(decisions B1, docs B9, docs
   correction 2)*
2. **EC-3 says four keys are undeclared; `hook_committed` is declared** — at
   `design/assistant-adapter.md:56`, i.e. in the wrong home, which is the real gap. `guide_file` and
   `install_commit` have no declaration anywhere (`grep -rn "guide_file\|install_commit" design/
   implementation/` → none). **The corrected shape is sharper, not softer.** *(docs correction 1,
   decisions B5)*
3. **EC-2's axis line — *"HEAD posture × the ten `COMMITTING_DOORS`" / "the registry exists; the cell
   does not"* — is false for the one door both EC-2 and EC-26 lead with.** `setup` is not a member and
   cannot become one: the predicate is a **hook-capable** commit and setup passes `--no-verify` by
   recorded design. `crates/cli/src/invocation_log.rs:95-97`, `:119-121`, `:130-171`;
   `setup.rs:1709`, `:1602-1608`. *(decisions B2)*
4. **EC-26's robust arm ("`setup` joins the carryover gate") revises a locked statement in two homes
   and cannot reuse the gate's mechanism.** The gate is snapshot/compare inside a working area
   (`MINT_DOORS`); `setup` mints none. `design/surface-contract.md:123`; `design/finalize.md:159`;
   `crates/engine/src/state.rs:808`, `:860-863`. *(decisions B3, capabilities G5)*
5. **EC-26 is mis-tiered.** Its leg-1 citation exists and is uncarried:
   `design/assistant-adapter.md:52`. *(decisions B4, docs B7)*
6. **EC-29's *"design-declared"* grade is the lenient reading.** `finalize.md:178`'s *"worktree
   untouched"* is a clause about the **restore's mechanism**; every sibling row explicitly restores
   the worktree. Under the plain reading `:178` + `:183` jointly promise all-or-nothing and the
   residue breaks it. *(docs correction 7, decisions B9)*
7. **EC-12 as written would produce a false guide edit.** `design/team-ready-state.md:102`/`:106`
   already settle it: the **sub-task** discard commits; the ordinary one does not. *(docs correction 8)*
8. **EC-14's citation for the *"ten"* count is wrong** — the stale sentence is at
   `design/corpus-migration.md:83`, not `:281` (`:281` is the *"Hashed, not refused by name"*
   bullet). The count itself is confirmed stale: dev **6** + methodology **11** = **17** entries,
   **16 distinct doctypes** (`commit` is in both manifests) — and `:281`'s own *"6/6 dev + 10/10
   methodology"* is separately wrong, and never says whether *sixteen* means entries or types.
   *(docs correction 3, doctypes A4, doctypes D2)*
9. **EC-14's *"eleven"* is current, not stale** — `ERROR_CODE_REGISTRY` has 11 members and the table
   has 11 rows; the defect is that the mirror is **unfenced** (carried defect (h)). *(docs
   correction 4, decisions F10, baseline §3)*
10. **EC-14's *"the window closes here"* pair is presentation, not a lie** — both paragraphs state in
    their own text that the close is keyed to the **1.0 pin, not a wave name**, and each later spend
    is explicitly declared. *"Fix the heading"* and *"the doc lied"* are different edits. *(docs
    correction 5, decisions F10)*
11. **EC-17's axis is wrong in one cell and short by a dimension** — the fourth `DESTROYING_DOORS`
    cell cannot exist (`remove_worktrees` filters on the **registered** set and `continue`s), and the
    doors run **two** subject derivations; a third derivation (`narrate_taken`) is what produces the
    *"not recoverable"* lie. `crates/cli/src/milestone.rs:4348`, `:4356`, `:2645`, `:5091`. *(docs
    correction 6, capabilities G6, baseline §3)*
12. **EC-6's cited basis is a misreading.** `design/measurement.md:75` says the log and the dogfood
    hook's JSONL are *"independently versioned surfaces"* meaning independent **of each other**;
    `invocation_log.rs:379` repeats that sense. The M44 `task_id` deferral rests on *rebuildable +
    purely additive*, not on an integer. *(decisions F4)*
13. **EC-7's counts and grade are both off** — 22 patterns, not 20; the floor **is** independently
    pinned twice in production source; the residual gap is deliberate regeneration. **And a new
    defect the charter's narrowing does not carry:** `e2e_audit::floor_patterns()` scrapes only
    **20 of 22** because it `break`s on a comment. *(decisions F5, doctypes D3, baseline §3)*
14. **EC-20's framing needs one correction** — `task validate` and the **landed** `task finalize`
    emit the *identical* findings, so the contract's *"same check, same severity"* is **not**
    falsified by the committing door; the divergence is `--dry-run`, whose envelope has **no
    `findings` key at all**. The law-1 problem is QUICKSTART presenting the two as one surface.
    *(docs correction 9, baseline §3)*
15. **EC-30's milestone arm remains undriven** by any probe; the charter's own instruction stands —
    drive it first, and if it refutes, record it refuted with its datum and build nothing.
    *(baseline §5, unchanged)*
16. **The razor's ground for excluding frozen-shape changes is stated backwards** for *changes to
    existing* frozen doctypes (G-39). This corrects the **ground**, not the exclusion.
17. **`{{schema:<T>}}` and the slot-ceiling statements reach no Tier-2 row** — the `{{schema:}}` seam
    renders only inside pack **step bodies**, and not one EC-11/EC-14/EC-22/EC-23 sentence lives in a
    step body; they live in `--help` texts and the two shipped guides, which that seam cannot see.
    *(doctypes A2)*

**To the baseline ledger**

18. **The sub-task `resume:` line resolves into three states, all real — not a refutation.** The
    baseline recorded one; the probes drove three:
    (a) **PRE-provision, HEAD moved** — `jigc start --task <id>` and `jigc workflow <wf> --task <id>`
    give a **byte-identical** refusal from an upstream base-pin guard, so naming `workflow` in the
    footer buys nothing and the reuse claim is **refuted** (capabilities G13, driven);
    (b) **POST-provision** — `jigc start --task alpha-task` composes at exit 0 with **no
    `.jigc/tasks/<id>/docs/` at all**, while the `Spawn:` line `jigc workflow sub-task --task
    beta-task` provisions `commit:beta-task.md` + `provenance.json`: a resumed sub-agent gets a
    workflow it cannot author, and the refusal's route prints an empty list and never names the
    provisioning verb (baseline cell 13 / `[doors §3]`, driven end to end, two arms in one corpus);
    (c) **`base == HEAD`** (after `git reset --hard <base>`, from a fresh clone at the pin, or after a
    squash fold) — the sub-task workflow **composes in the shared checkout at rc=0 with no worktree
    mention and no advisory**, because the guard is conditioned on moved history (doctypes A6,
    driven). **The consequence in (c):** `milestone finalize` folds each sub-task's code from **that
    worktree's** staged index, so code staged in the shared checkout is outside the boundary — read
    off the composed `execute` text plus the documented fold, **not** demonstrated by a full fan-out
    landing.
19. **The charter's N20 line cite does not resolve at HEAD** — `decisions-pending.md:563` is the
    `adr.cites-code` migration row; `:597` is a bootstrap-facts row. Re-derive the line before
    editing. *(docs correction 10)*
20. **`COMMITTING_DOORS` is 10 and the tenth door's ack is silent about its own commit** — a row in
    no ledger entry, beside the count sweep. *(baseline cell 12, decisions F14, G-51)*
21. **The `Plain` family is a list, not a class** — confirmed by the probes and extended: `value` is
    a sixth `Plain` argument that becomes a path component at a **moving** door, and it sits in a
    different tier with a different predicate (G-30). *(decisions B10)*
22. **`e2e_audit`'s scrape and `AMBUSH_CLASS_CODES`' membership are two mechanisms the baseline
    graded from reading; both were driven here** — the pack-load fences **have teeth** (both reddened
    under `JIGC_PACK_DIR`), and the scrape **is truncated**. *(doctypes D1, D3)*
23. **The pack-step count is now 69, the number a shipped fence bans** — and the fence's own prose is
    stale in two places. *(doctypes D4)*

---

## Contradictions between locked docs

`docs F-d`'s twelve rows, merged with `capabilities H7`. **Where a settler is named, the plan must
not pick freely.**

| # | doc A | doc B / code | the disagreement | who settled it |
|---|---|---|---|---|
| 1 | `command-output-contract.md:19` *"**Three verbs** emit this composed shape"* | `crates/cli/tests/text_json_parity_axis.rs:385-390` — *"stdout IS the pinned `{task, text}` contract"* for `["milestone","execute"]` | membership of a 1.0-pinned surface: 3 vs 4 | **the registry** — it names `render::composed` explicitly; the doc was never updated. **Not a free choice.** |
| 2 | `doc-read-surface.md:5`, `:57` *"the milestone record is the conformance witness"* | `team-ready-state.md:195`'s enumeration | 4 vs **6** top-level keys, 3 vs **5** item keys (**driven**) | **`doc-read-surface.md:59/68/69`** — the keys are correctly declared there; only the witness is stale |
| 3 | `finalize.md:204` *"the subject is the on-disk path, **never registration**"* | `crates/cli/src/milestone.rs:4348`/`:4356` (registered set) | which set a destroying/tearing-down door's subject is | **`finalize.md:204`** settled it for the boundary; the teardown never joined |
| 4 | `finalize.md:178` *"worktree untouched"* | `finalize.md:183` *"Before phase 6, all-or-nothing"* | what the transaction promises | **unsettled — fork 3** (and `DECISIONS.md:4788` moves with it) |
| 5 | `finalize.md:31` *"No in-progress merge/rebase/bisect"* | zero `MERGE_HEAD`/rebase/bisect probes in `crates/`; `--carry-staged` **concludes** a merge (**driven**) | whether a posture preflight exists | **the binary** — the bullet was never built |
| 6 | `team-ready-state.md:102`/`:106` (sub-task discard commits; ordinary discard does not) | `MIGRATING.md:40`, `QUICKSTART.md:180` (*"no commit"*, unqualified) | whether `task discard` commits | **`team-ready-state.md`** — the guides need the **qualifier**, not a flip |
| 7 | `MIGRATING.md:39` / `QUICKSTART.md:160-161,165` *"jigc never passes `--no-verify`"* | `setup.rs:1709`; `finalize.md:89/:107/:250` (correctly **scoped** to finalize) | scope of the never-`--no-verify` promise | **`finalize.md:107`** — it names setup's exclusion by its recorded rationale; the **shipped** guides dropped the scope |
| 8 | `assistant-adapter.md:52` *"setup commits its own install, **and only its own install**"* | EC-26 (driven: an uncommitted `CLAUDE.md` edit lands inside the install commit) | whether a pathspec bounds authorship | **unsettled — fork 4**, and the sentence is its premise |
| 9 | `storage.md:7` *"the staging working area … a transient working copy"* | `.jigc/tasks/<id>/source-path` is authority for an irreversible delete (driven) | the source-of-truth model's completeness | **unsettled — G-3** |
| 10 | `project-setup.md:103-106` *"never touch an existing project's `.gitignore`"* / *"still merge — never clobber"* (the **root** file) | `gitignore.rs:32` whole-file replace of **`.jigc/.gitignore`**; its own doc-comment says *"amended once to the union"* | whether jigc-owned tracked files are amended or replaced | **`project-setup.md`** states the principle at the sibling file; the workbench file has **no rule** (`storage.md:116` describes it and states no ownership) |
| 11 | `measurement.md:75-76` *"independently versioned surfaces"* (and `invocation_log.rs:378-380` repeating it) | no version integer exists (`grep -rn "log_version\|LOG_VERSION\|record_version"` → 0); `decisions-pending.md:455` defers `task_id` **on that very premise** | whether the log is versioned | **unsettled — the EC-6 fork**; the deferral's ground and EC-6's finding are the same claim read opposite ways |
| 12 | `doc-read-surface.md:175` heading *"five regimes"* / `:177` *"Five independently-governed … regimes"* | the table at `:181-186` — **six** rows | a count contradicted two lines below itself | **the table** |
| 13 | `invocation_log.rs:130` (`COMMITTING_DOORS` = **10**), `flow47_acceptance.rs:18` | `MIGRATING.md:39`, `QUICKSTART.md:180`, `CLAUDE.md:7`, `worked-examples.md:3005`, `flow47_acceptance.rs:21`, `decisions-pending.md:320` — **nine** | how many committing doors there are | **the code registry** — a stale count with one authority. **But underneath it is a real definitional disagreement: `jigc setup` is a committing door that is not a member** (G-12, G-19) |
| 14 | `design/validation.md:324/:354/:426/:435` — **five** store probe families, matched by `crates/engine/src/validate.rs:477/:496/:516/:580/:607` | `jigc validate --help` names **one** (*"code anchors"*) | what `validate` checks | **the doc and the code agree; the help text is the liar** — one authority, one liar, and no registry to generate from (G-52) |

---

## Owed at the gate-record

The fourteen `planning-record` gates are one required slot each, no `optional:` anywhere, and the
doc and the schema are held in lockstep by two suites that derive the gate ids from the **shipped**
schema. **Writing `completions/artifacts/M51/planning-gate-record.md` is safe** — the gate-home
fence's `GOVERNED_TREES` is `["design", "implementation", ".claude"]` and its own module doc states
that `completions/` holds the **filled instances**. Schema order and lines:
`reuse-exercised` `:53` · `cheap-vs-robust` `:57` · `foreclosed-by-doc` `:61` ·
`prior-art-reconciled` `:65` · `census` `:69` · `integration-seam` `:73` · `check-scope-pinned` `:77`
· `design-complete` `:81` · `acceptance-spiked` `:85` · `value-flow-exercised` `:89` ·
`deliverable-reachable` `:93` · `strategic-claim-fresh` `:97` · `quote-attributed` `:101` ·
`claim-driven` `:105` (`packs/methodology/schemas/planning-record.yaml`;
`crates/cli/tests/planning_gate_home.rs`, `planning_record_schema.rs`).

**The four slots this wave will strain on** *(doctypes A1)*:

- **`acceptance-spiked`** — *"every acceptance flow's behaviour and commands spiked on the real
  binary — driven through the shipped verb's entry point"*. M51's acceptance is the **per-axis
  review**, which is a cell matrix, not a flow with commands. Filling it honestly means spiking the
  eight axes' matrices and satisfying the review's own fence — every leaf in `VERB_KINDS`
  (`crates/cli/src/cli.rs:1412`, 47 leaves) appears in at least one matrix or is named uncovered.
  **Fork 9 is open, so this cell cannot close before the Settle resolves it.**
- **`claim-driven`** — *"filled by naming what was driven"*. Six ledger rows are source- or repo-read
  only and must be written in **as relayed** or driven first: EC-6, EC-7 (fence grades from reading
  test bodies), EC-8 and EC-9 (absence by grep), EC-24 (absence by grep), and EC-30's **milestone**
  arm, explicitly not driven. EC-14's *"three `DECISIONS.md` citations point at blank lines"* was
  **not checked** by any probe. Cheapest discharge: drive EC-6/EC-7 with **one applied mutation
  each** — the baseline says so in its own words, and the doctypes probe showed the reported EC-7
  grade was wrong in **both** directions.
- **`value-flow-exercised`** — a fix wave has no net-new value flow. Answer it as the pre-1.0 loop
  re-driven at baseline, or `N/A — <why>`; the schema treats an honest `N/A` as a complete answer.
- **`prior-art-reconciled`** — EC-8 (release-versioning policy) and EC-9 (publishing floor) have **no
  design doc at all**, so there is no prior art to reconcile and the cell must **say so** rather than
  be left thin. Everything else on this axis has prior art in quantity: the fourteen-row contradiction
  table above is that cell's raw material.

**Two mechanical obligations the close increment inherits**, so they are not discovered at the gate:
`foldback_truth.rs:242` must be **re-aimed to `**M51 —` and inverted twice** (it was late last time,
and its own doc-comment records that the lateness was the finding) — G-62; and the pack-step-count
fence bans the number that is now true, so any count claim this wave writes reddens the gate until
that fence moves — G-31.
