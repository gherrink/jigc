# M51 — the Settle record

Settled with the human across **2026-09-10/11**, against [charter.md](charter.md),
[baseline-ledger.md](baseline-ledger.md) (four Opus capability-auditors, all driving the release
binary) and [gap-findings.md](gap-findings.md) (four gap-detectors, 69 ranked gaps). **Every fork was
decided on two honestly-argued cases; none was self-framed by the proposer** — six independent
`robust-advocate` runs argued the robust side (`F1`, `F2`, `F3`, `F4`, `F7`, and one covering the
three smaller forks), each read-only, each settling nothing. **Their cases are archived verbatim
beside this file** — [advocates/](advocates/) — because they were written in the planning session's
scratchpad and would otherwise exist on one machine only.

**Every fork resolved robust.** That is a result, not a posture: the advocates were run precisely so
a cheap arm could win, and three of them **conceded scope against their own interest** (F7 refuses
`schema_version` on ~40 envelopes and refuses the doc-side markdown parser; `small-forks` argues the
*cheaper-looking* arm on EC-6; F1 concedes that a named list with a fence over the list is an
acceptable landing). What the cheap arms kept failing on was one shape: **each of them shipped a
false completeness claim on the record at the 1.0 pin** — the exact failure this wave was chartered
to correct.

**Posture.** Every fact this record turns on was driven against the **release** binary
`target/release/jigc` (`1.0.0-rc.14`) at `HEAD = bd348a83`. The advocates drove their own spikes
(F1's two-door table, F2's three-posture probe table, F3's [`advocates/f3-spike3.sh`](advocates/f3-spike3.sh) unrecoverable-loss cell,
F4's cells A and B on `dev/jigc-rig bare`, F7's key census); everything else is relayed from the
baseline or a gap-detector and is marked as such in those artifacts. **An agent's report is a lead,
not a measurement** — the rows this Settle turns on were re-driven, and the gate-record records
which.

**Reviewed before decompose, and amended.** Two independent readers — an Opus `design-reviewer`
driving the release binary ([design-review.md](design-review.md)) and an unseeded Codex source review
([codex-design-review.md](codex-design-review.md), prompt at [codex-design-prompt.md](codex-design-prompt.md)) — both returned *not ready to
decompose*. **The human accepted every finding on 2026-09-11.** The corrections live in
[Review amendments (2026-09-11)](#review-amendments-2026-09-11) below, with a dated bracket at each
decision they touch; no decision above was rewritten in place. **Read a decision and its bracket
together** — four of them claimed a shipped predicate or mechanism transfers, and the claim was false
when driven.

---

## The claim, unchanged in substance

> **No caller-supplied token and no repository posture reaches a door that destroys, commits or
> moves without that door having adjudicated it — the two exemptions the M50 registries carried
> (`ArgToken::Plain`'s path family; HEAD posture at `COMMITTING_DOORS`) are closed as classes — and
> every surface 1.0.0 pins says what the binary does, so that the pin closes over declared keys,
> true counts and a stated evolution rule.**

**Honest bound taken at this Settle, and stated in the wave's own terms:** the `Plain` path family is
shipped as a **classified registry with one stated rule — or one stated no-rule-and-why — per
member**, not as one shared predicate. The five exempted argument names were driven and ask four
different questions; folding unlike rules under one banner is the false-completeness shape M50's
Settle refused for family 3. The charter's falsifier anticipated this and it fired: the wave ships a
named list with a fence over the list, **and says so**.

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §3 — the claim gains the **symmetric posture bound**: the posture family closes over three of four driven members, and the `GIT_DIR` redirect is declared out with its rationale and its reopenable scope. Stating the bound for one half only was the false-completeness shape this wave exists to correct.]*

---

## Decisions

### D1 — EC-1 (fork 1): the registry arm, door **and** sink **and** the classified `Plain` family

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §2 · §10]*

**The fork.** *Cheap (as the baseline left it):* door + sink — guard `migrate`'s door and re-validate
the recorded token before `retire`. *Robust:* door + sink **+ the classified `Plain`-family
registry*, one stated rule per member, ⇔-fenced against the clap tree with an axis suite.

**The advocate's decisive argument** ([advocates/F1.md](advocates/F1.md) §2): the precedent is this repo's own. M50 derived
`DOCTYPE_DOORS` from a hand-kept `*_ARG_IDS` allowlist, `spec_addr` was not on it, and
`milestone add-from-spec spec:../../../<outside>/planted` **read outside the repo at exit 0** — the
M50 audit's HIGH. The fix was **totality**: erase the complement, so a new argument reddens until
someone answers it. `ArgToken::Plain` is total over *ids* and silent about *path-ness*, and
`cli.rs:1545` says so in its own words (*"nothing can check `Plain` from the outside"*). Two door
guards do not move that; a ⇔-fenced registry does. F1 also drove the table that kills the cheap
framing: `config set docs-root` refuses all three escape shapes with a code and a route, and
`jigc migrate` accepts all three at exit 0 — **the predicate is shipped and `migrate.rs` never asks
it**.

**Decided (the human, 2026-09-10/11) — the robust arm, in five parts:**

1. **The doors ask the shipped predicate.** `trackable::untrackable_reason` + `is_workbench_root` at
   `jigc migrate <path>` and at `jigc config insert-step/replace-step <file>` — three call sites,
   shipped predicates, no new capability.
2. **The sink is validated CLI-side.** `plan.retirements` is validated **before** `retire`. The
   engine cannot host the predicate — zero `Command::new` in `crates/engine`, no symlink syscall —
   and that is recorded as the reason rather than left implicit. The sweep covers **all five** raw
   `read_source_path` consumers, `retire_exempt` included, because that one also buys an arbitrary
   path a pass through the carryover gate.
3. **The registry.** A code-side path-argument registry over every `ArgToken::Plain` argument that
   becomes a path component — **six members**: `path`, `file`, `from_file`, `from`, `target`,
   `value` — each carrying **one stated rule or a stated no-rule-and-why**, ⇔-fenced against the
   clap tree, with an axis suite over `{absolute, ../, symlink, .git/}`.
4. **The human gate names what it will delete.** The exit-4 review hold names the file `--approve`
   retires, in text and as an **additive `retires` key**, declared.
5. **EC-27 (`value`) moves into this registry** rather than sitting in Tier 3 under an unshared
   predicate — the scope-coherence gap G-30, closed by membership.

**The adapter deny floor KEEPS the blanket permit** for `migrate` + `finalize --approve`, **with the
reason recorded rather than left as silence**: a guarded migrate destroys only a *reviewed, in-repo,
git-recoverable* file, so the asymmetry against `uninstall`/`milestone discard` — which are human
acts — is right once part 1 lands. The record states this because silence here is a by-omission
blessing. **`e2e_audit::floor_patterns()`'s comment-truncation is fixed in the same increment** — it
`break`s on the profile's comment and therefore scrapes 20 of 22 entries, so the one test whose job
is to check the floor cannot see the two M50 entries.

**Doc consequences.** `auto-migration.md:39` (*"an arbitrary foreign file"*) **struck with its
falsifying datum and narrowed to in-repo** · `write-commands.md:127` gains the **source-side sibling**
of its destination rule, in the same home · `storage.md` names `source-path` as **authority**, the
exception its source-of-truth model has no room for · `finalize.md:174`'s rollback row states
**admissibility**, not only byte-capture · `worked-examples.md:3358` takes a **dated correction**
(four token families → five).

**Discharges:** G-1 · G-2 · G-3 · G-4 · G-6 · G-30 · G-40 · G-49, and G-43's new half (the scraper).
**G-5 is discharged in part** — the fenced `migration-finalize` step sentence (*"the deletion
staged"*) becomes true in every escaping cell once the door refuses them; whether it is still false
for the **in-repo-untracked** cell is a `stage_migration`/`path_in_index` question re-driven at
decompose, and repaired in the pack if it is.

---

### D2 — EC-2 (fork 2): refuse, as a **posture family**

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §3 · §4 · §10]*

**The fork.** *Cheap:* narrate the detached HEAD and proceed under a stated consent. *Robust:* refuse
at the shared seam — with the fan-out exemption answered.

**The advocate's decisive argument** ([advocates/F2.md](advocates/F2.md) §1): the cheap probe was
**measured**, not argued. Across three synthetic repos, `symbolic-ref -q HEAD` answers **one of the
three implementable cells** and returns a false green on the two the baseline drove as live damage —
`milestone create` pinning git's **empty-tree hash** into a *committed* record, and `--carry-staged`
concluding the user's merge under jigc's own subject at exit 0. And `design/finalize.md:31`
**already promises** the third cell verbatim (*"No in-progress merge/rebase/bisect"*) with **zero**
probes in either crate: razor legs 0 and 1 fully available. So the cheap cut would ship a wave
claiming *no repository posture reaches a door without that door having adjudicated it* while leaving
a **declared** posture rule unimplemented.

**Decided (the human, 2026-09-10/11) — refuse, over a family of three predicates:**

- **HEAD detached** · **HEAD unborn** · **an operation in progress** (`MERGE_HEAD` / rebase /
  bisect). `finalize.md:31`'s promise is **built, not struck**.
- **The `GIT_DIR` redirect is declared out**, with `repo.rs:1-16`'s no-shell-out rationale **quoted**
  (*"so a fake `.git` can never walk up to, and bind against, a real ancestor repo"*; ~15 fixtures
  depend on it). Its **scope is reopenable** and is recorded as such; the
  `dirname(common-dir) != toplevel` shortcut is **not a discriminator** — a legitimate linked
  worktree has the identical asymmetry. Leaving the cell unmentioned is the one arm the record
  forbids.
- **A new registry — "doors that commit or move on the user's behalf"** — a **superset** of
  `COMMITTING_DOORS` (`COMMITTING_DOORS ⊆ …` asserted) that **holds `jigc setup`**. It carries no
  error identity, so it triggers no `ERROR_CODE_REGISTRY` derivation and duplicates no existing
  fence. **Three consumers read it**: EC-2's posture probe, EC-26's staged-set guard (D3), and the
  survivable frame's cause vocabulary (N20). Without one home each hand-enumerates, and every
  hand-listed set this repo has audited grew.
- **The probe sits at the door before work and at `overlay_docs_commit_and_ff`** — the landing act,
  with the live checkout as its subject — because `jigc setup` bypasses `git_commit_capture`
  entirely and the fast-forward is downstream of the seam.
- **`DedicatedWorktree` becomes a typed argument at the commit seam, never inferred from a path.**
  The exemption is **measured as un-sniffable**: every fan-out worktree answers `symbolic-ref -q
  HEAD` with exit 1, exactly like a user's detached HEAD. A path-shape guess is the class M50's audit
  condemned (`fanout_worktree_paths`' `is_dir()`).
- **`--force` is the single consent**, matching the destroying-door mold.
- **The linked-worktree ack names the repo and the branch it landed on** — posture is not only *is
  HEAD attached* but *which repo is this door acting on*.
- **`--carry-staged` stops concluding a merge**: the operation-in-progress predicate refuses first.
  The flag's documented consent is *carry my staged work*, and it never covered concluding someone
  else's merge.

**Decision-record consequence.** The consent shape is a 1.0 contract and the doors are asymmetric:
*narrate → refuse* is breaking after the pin, *refuse → add a consent flag* is additive. This spends
the one-way door in the reversible direction.

**Discharges:** G-7 · G-8 · G-9 · G-10 · G-11 · G-19 (the definitional half) · G-32 · G-41 · G-67,
and supplies N20's vocabulary home (G-47).

---

### D3 — EC-26 (fork 4): `setup` **refuses** its install commit over bytes it did not write

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §1 · §4 · §5 · §10 · §13]*

**The fork.** *Cheap:* `setup` narrates what it swept into its commit. *Robust:* `setup` joins the
carryover gate — the gate's subject becomes the **door**, not the task.

**The advocate's decisive argument** ([advocates/F4.md](advocates/F4.md) §2–§3): two driven cells, and the
second is in **no EC row**. Cell A — a user with `MM CLAUDE.md` (staged version A, worktree B) runs
`jigc setup`: **exit 0**, both the staged intermediate and the unstaged WIP land inside
`chore(jigc): install jigc workspace config`, and **the tree ends clean**, so nothing prompts
recovery. Cell B — an adopter with a pre-existing `pre-commit` hook rejecting `AWS_SECRET` has an
uncommitted `AWS_SECRET=hunter2` line: `setup` commits it at exit 0, because `setup.rs:1709` passes
`--no-verify` on a rationale (`:1603`, *"the only hook present is the warn-only pre-commit setup just
installed"*) that is **driven false** — setup preserved the user's hook and then skipped it. And in
the **same commit** it installs a `SKILL.md` saying *"jigc never passes `--no-verify` — your hooks
are policy."* One invocation authors a law-1 lie and violates it. `finalize.md:159` already retired
narration **for exactly this direction**: a foreign pre-staged change riding a jigc commit earned *"a
second sanctioned refusal."*

**Decided (the human, 2026-09-10/11) — refuse:**

- `setup` **refuses its install commit** when any of its own paths carries bytes it did not write.
  **The files stay written and staged, with a route** — so the guard binds the *commit*, not the
  *install*, and M49's non-circular-bootstrap bound is untouched by construction.
- **`--force` is the single consent** (the destroying-door mold).
- **An in-memory pathspec-scoped `StagedSnapshot` pair** — `decide_carryover` already takes in-memory
  values and setup mints and commits in one invocation, so nothing persists and **`MINT_DOORS` is
  untouched**. Unscoped would be wrong: an unrelated staged `feature.txt` is correctly untouched
  today, driven.
- **A `CarryoverBoundary::Setup` variant with its own code and route** — a task-shaped finding code
  would lie at this door.
- **Docs, each with its basis:** `surface-contract.md:123` and `finalize.md:159` (*"every
  task-minting door"*) are **revised with their basis** — the subject widens from *task-minting door*
  to *any door committing paths it does not own* · `assistant-adapter.md:52` is **re-derived to say
  bytes** (a pathspec bounds paths, never authorship) · `setup.rs:1603`'s false `--no-verify`
  rationale is **corrected**, and the two shipped guides' *"jigc never passes `--no-verify`"*
  universal is **scoped** — a half that is owed under either arm.

**Re-tiering, recorded:** **EC-26 moves from Tier 3 to Tier 1 on its leg-1 citation**
(`assistant-adapter.md:52`), which the charter did not carry. It is a hole in a **declared** surface,
not a capability addition.

**Honest weakness, carried from the advocate:** `--force` at a first-run door risks reflex-training
(M46's measured ground for refusing the `--ignored` refusal). The answer on the record: unlike build
output in a worktree, user bytes at exactly those eight paths are *not* the ordinary success path, so
the guard is rare rather than routine.

**Discharges:** G-12 (the `setup`-shaped half) · G-13 · G-14 · G-26.

---

### D4 — EC-29 (fork 3): the transaction **restores the worktree**

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §6]*

**The fork.** *Cheap:* keep `finalize.md`'s declared index-only fidelity and correct the
*"all-or-nothing"* header two rows above it. *Robust:* rollback restores the worktree too, over the
fifth un-swept dimension of the rollback matrix.

**The advocate's decisive argument** ([advocates/F3.md](advocates/F3.md) §1–§2): the cheap arm's premise is
**false in the doc it cites**, and the driven cell is unrecoverable loss. `finalize.md:178`'s
*"worktree untouched"* sits **inside the restore mechanism's parenthesis** (`update-index --cacheinfo`
does not touch the worktree) while every sibling row restores worktree bytes explicitly, and
`DECISIONS.md:4788` — written by the M45 audit that *made* this fix — says it rolls back *"every path
jigc's own staging contributed."* So cheap is not *keeping* a declared decision; it is **declaring a
new one** and editing two statements to say less at the 1.0 pin. Driven ([`advocates/f3-spike3.sh`](advocates/f3-spike3.sh)): a user's
uncommitted private line in `.jigc/.gitignore`, a hook-rejected `task finalize` → the file is
rewritten to canonical `ENTRIES`, **`git status` reads CLEAN**, and the bytes exist in **no git
object**. The transaction that reports *"nothing was committed"* destroyed user content and left no
signal. Reach is the ordinary **upgrade → finalize → hook rejects** sequence, not an edge.

**Decided (the human, 2026-09-10/11) — restore the worktree:**

- **A config-layer captured-pre-image family**, on `RecordPreImage`'s shipped
  *"absent means absent, never unreadable"* discipline: **capture before `gitignore::ensure` and the
  version refresh, restore at the existing failure site** — one capture point, one restore point,
  both already there, covering every arm including the fan-out `--ff-only`.
- **`gitignore::ensure` becomes amend-to-union (EC-18)**, **byte-idempotent across its four callers**
  — or `finalize` starts committing a churning file — with **the ack naming the content change**,
  which is EC-18's law-1 half and is owed independently of the merge.
- **Docs:** `finalize.md:178`'s clause and `:183`'s header are **reconciled** (which sentence is the
  promise), and **`DECISIONS.md:4788` is kept true** rather than narrowed.
- **Any per-path exemption — `.jigc/version` is the candidate — is stated with its reason**
  (`Snapshot::Exempt(<reason>)` already models it), **never left un-swept**. A stamp that is
  arguably correct-to-keep is an exemption, not a licence to leave the dimension open while
  `.gitignore` sits in the same pathspec set.

**Discharges:** G-18 · G-46.

---

### D5 — EC-3/EC-5 (fork 7): **mint the pinned-envelope registry first**

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §7 · §8 · §9 · §12]*

**The fork.** *Cheap:* declare the four undeclared keys and state the versioned/unversioned rule as a
sentence. *Robust:* delete what should not have shipped and close the partition against the clap leaf
tree.

**The advocate's decisive argument** ([advocates/F7.md](advocates/F7.md), the hole and the one-way-door
tell): `command-output-contract.md:446` states the rule — *"an undeclared key on a pinned envelope is
a defect, not an addition, whichever wave mints it"* — and **the rule quantifies over a set that
exists nowhere.** `Tier` in `text_json_parity_axis.rs` has two members and classifies
*fence-ability*, never *pinned-ness*; `grep -c "as_object\|\.keys()"` in that file returns **0**. So
on 1.0.0 day the rule is unanswerable in both directions. Worse, the cheap arm's `keys:` column
*derived from what the binary emits today* **blesses 17 accidents into contract by construction**,
converting *arguably not pinned, therefore deletable* into *asserted as the key set* — and deletion
then costs a v2. F7 re-derived the class independently: of 12 sampled keys, **11 appear in no
`design/` file at all**. EC-3's own premise falls in the same motion — `hook_committed` **is**
declared, at `assistant-adapter.md:56`, i.e. in the wrong home — and the class is ~4× the stated
four.

**Decided (the human, 2026-09-10/11) — the registry, code-side, before the pin:**

- **Every leaf verb × arm**, with the recipe table's one-per-verb bijection **reshaped to
  `(path, arm)`** — needed anyway for `task finalize` landed vs `--dry-run` and `milestone finalize`
  landed vs blocked. Each row is **`Pinned(<declared key set>)` or `Unpinned(<reason>)`**. The doors
  are asymmetric: `Unpinned(reason)` is reversible, a blessed key is not.
- **The 17 undeclared keys across 10 verbs are disposed *before* the pin** — declared in the contract
  doc's own paragraph form, or deleted. **`hook_output` on the read verb `milestone list-tasks` is
  deleted** (structurally always `""`, outside that key's declared scope); **`hook_committed`'s
  declaration moves** to the contract doc.
- **The doc's *"three surfaces"* section names the registry** as the list.
- **`schema_version`'s partition is fenced against the registry** — typed result values carry it,
  ad-hoc `json!` envelopes do not — and **the *"add it to ~40 envelopes"* reading is refused on the
  record**, the advocate's own concession, as real gold-plating.
- **The two remaining shape cells are decided one row each:** `task list --format json`'s top-level
  array, and `milestone execute`'s fourth-producer status — where **the doc takes the registry's
  membership**, because the code-side census names `render::composed` explicitly and settled it; this
  is not a free choice.
- **The doc-side markdown-parser half is refused** per `pinning.md` §3, which refuses that class by
  name.
- **Riders that ship with it, each additive inside the open window:** `task finalize --dry-run` gains
  **`findings`** (EC-20) with the **behavioural equal-set fence** — drive each `Tier::Previewed`
  member and assert the emitted `code` set is equal across `task validate` / `--dry-run` / landed —
  and `ConfigAck::Set` gains **`relocated`** (EC-4), which the exhaustive destructure then forces.
- **Every added key ships with its declared paragraph**, in the form the contract requires.

**Discharges:** G-15 · G-33 · G-42 · G-44 · G-45 · G-54.

---

### D6 — EC-9 **refused**; EC-8 **taken**, in two homes

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §7]*

**The fork.** The content of the release-versioning policy and the membership of the publishing
floor. **These are human decisions, not code.**

**The advocate's decisive argument** ([advocates/small-forks.md](advocates/small-forks.md)'s framing, sharpened by G-28/G-29):
EC-9 **re-opens a decision the human took twice** — `DECIDED 2026-07-16 (human): post-v1`, re-affirmed
by citation at the M46 Settle — and the floor's own trigger (*"first external adopter, public
release, or opening the repo"*) fires **after** the 1.0.0 call, not before it. Two of EC-9's four
facts are stale besides (`publish = false` and the license are already set). Meanwhile EC-8's **home
is forced**: `design/` never ships, and the shipped guide body is `QUICKSTART.md` + `MIGRATING.md`
`include_str!`'d into `SKILL.md`.

**Decided (the human, 2026-09-10/11):**

- **EC-9 is REFUSED**, on the 2026-07-16 human decision re-affirmed at the M46 Settle. **The
  publishing floor stays on its trigger**, with **going public kept in view for later** — recorded,
  not silently dropped.
- **EC-8 is taken**, in two homes: **one policy paragraph in an internal home** — the command-output
  contract's *Evolution posture* section **widened to the binary** — plus **a compact adopter-facing
  version in the guide body**, written self-contained (relative links are flattened on install).
- **The default policy:** inside **1.x** nothing pinned is removed or reshaped; contract-versions and
  schema-versions only **increase**, and each ships its migration; a removal or reshape is **2.0**.
  Written so that **nothing forecloses a later public release**.
- **The probe-wire deferral is re-affirmed by citation**, not re-decided: `validation.md:167`/`:187`
  and `multi-pack.md:155` — the trusted-pack basis holds and the cross-pack collision trigger is
  unfired after PB-1.

**Discharges:** G-28 · G-29 · G-65 (it falls with EC-9) · G-68, and the policy half of G-55.

---

### D7 — F-5 and F-11: make the **declared contract true** at the doors

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §8]*

**The fork, re-posed.** The charter posed *string vs envelope*; G-17 showed the cheap arm's cited
precedent does not transfer. The M50 refusal was grounded on `(code, null)` for **a token that names
nothing**; F-5's subject is a **work unit**, whose target form the contract **already declares**, and
the engine **already ships `finalize.no-task`** — the CLI short-circuits it one layer above, in one
shared home, as a code-less `anyhow`. So the real fork is *make the declared contract true at these
doors* vs *declare the doors permanently outside the envelope*.

**The decisive datum** (gap-findings G-16): `command-output-contract.md:202` claims the flattened
`{"error": …}` *"since M50 Increment 10 carries the code and the locus inside it exactly as the
agent-text surface does"* — **driven false at all 22 doors**. A stated rule violated at HEAD on the
pinned write-side contract, carried in no ledger entry.

**Decided (the human, 2026-09-10/11):**

- **F-5:** the declared contract is **made true at the 22 unknown-id doors**, through the shared home
  (`task.rs:971`, `start.rs:2131`), using the engine's shipped **`finalize.no-task`** and M50
  Increment 9's typed **`Result<_, Finding>`** shape — an error type a bare `anyhow` cannot inhabit,
  so the escape closes at the **type** level. `command-output-contract.md:202`'s false sentence is
  corrected in the same edit.
- **F-11:** **code + route inside the flattened string now.** Its **envelope** disposition is **one
  row of D5's registry** — which is exactly what the registry exists to answer, rather than a
  separate guess.

**Discharges:** G-16 · G-17.

---

### D8 — counts are **fenced, not rewritten**

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §11]*

**The fork.** *Cheap:* rewrite the numerals. *Robust:* fence or iterate them.

**The advocate's decisive argument** ([advocates/small-forks.md](advocates/small-forks.md), fork A — three spikes):
CLAUDE.md's *"thirteen `CELLS` rows"* was **63** when written and is **64** at HEAD — it moved again
between the evidence check and this Settle: a 100 % defect rate with a half-life shorter than the
wave. The sync mechanism actually in use is a **comment addressed to a human**
(`invocation_log.rs:513`), and nothing reads the file it names. The door count moved 9→10 at M49 and
**five prose homes still say nine**, including `MIGRATING.md:39`, which ships to every adopter, and
`flow47_acceptance.rs`, which contradicts itself four lines apart. And the fence built for this class
**became an instance of it**: `foldback_truth.rs` fails any claim `== 69` as *"never the number"* —
and the tree is now exactly **69** (30 dev + 39 methodology).

**Decided (the human, 2026-09-10/11):**

- **`ManifestKind::ALL` is minted**, with a **per-prose-unit `foldback_truth` arm**.
- **The `migrate-corpus` triage keys** are fenced via the report struct's **exhaustive destructure**,
  with a **per-field disposition** (field names ≠ wire keys; `text_json_parity_axis`'s `Disposition`
  already models it).
- **`COMMITTING_DOORS`' count and `ERROR_CODE_REGISTRY`'s doc mirror are fenced on the
  `doctype_map_versions` mold** — the shipped mechanism that reads a registry and asserts the doc's
  rows.
- **Historical counts are dated-bracketed, not re-pinned** — `dated_correction_spans` is the shipped
  discipline, and this is the instruction a fixer told to *"fix the counts"* will otherwise violate.
- **`foldback_truth.rs`'s `== 69` fence is re-keyed**, with its historical measurement point kept.
- **The two refuted EC-14 members are recorded as corrected, not fixed:** *"eleven"*
  (`surface-contract.md:129`) is **current** — the defect there is the **unfenced mirror** — and the
  two *"the window closes here"* headings are **presentation, not a lie**, since both paragraphs key
  the close to the 1.0 pin and each later spend is explicitly declared.

**Discharges:** G-31 · G-37 · G-53 · G-19's count half · G-24 (the enumerated-JSON-shape family — the
1.0 read contract's conformance witness is regenerated from or fenced against the shape it witnesses,
on the same mold; the exact mechanism is decompose-owed, because the witness needs a code-side value
rather than an ad-hoc `json!`) · G-62 · G-66.

---

### D9 — EC-6: the invocation log is **declared UNVERSIONED** and additive-only

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §14]*

**The fork.** *Cheap-looking:* add a version integer and close the key set. *Robust, in the other
direction:* **declare it unversioned** and close the key set.

**The advocate's decisive argument** ([advocates/small-forks.md](advocates/small-forks.md), fork B — arguing against the
thorough-looking option): versioning **answers a question nobody asked** — `measurement.md:75`'s *"independently
versioned surfaces"* means independent **of each other**, not *carrying a version*. It is wrong three
ways: it **mints a pinned surface in the wave whose job is closing the pin**; it adds a bump
obligation to the same prose-not-fence mechanism EC-10 records failing five consecutive waves; and it
is **weaker than what ships** — every record already carries `binary_version` (driven,
`"1.0.0-rc.14"`), and on an append log fed by successive binaries a per-record stamp maps each record
to the format that wrote it, where one file-level integer cannot. The **real** hole is elsewhere:
`Record` has 8 fields, the suite asserts presence/type on 4, **no key-set equality exists anywhere**,
and the one doc stating the shape is wrong in two fields.

**Decided (the human, 2026-09-10/11):** the log is **declared UNVERSIONED and additive-only**, with
**`binary_version` the discriminator**; its **key set is closed by an exhaustive-destructure fence**
over `Record` driving key-set equality on the emitted JSON; **`measurement.md:62`'s two wrong fields
are corrected** (`duration` → `duration_ms`; `finding_codes` is always present, `[]` on success, not
*"(on failure)"*); and **the M44 `task_id` deferral's basis is stated** — *rebuildable + purely
additive* — rather than assumed.

**Discharges:** G-34 · G-56.

---

### D10 — `AMBUSH_CLASS_CODES` becomes **derived**; N26's two questions answered

*[Amended 2026-09-15 at the build: see [Review amendments](#review-amendments-2026-09-11) §20 — the `orphaned` row's `id` is **`null`** (the stamp names no type), and the derived owe-set **unions a declared-identifier const** so `finalize.left-out` survives without a hand-stated row.]*

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §4 · §9 · **§18**, which settles N26's second question as its own code, `schema-conformance.unversioned-doctype`]*

**The fork.** *Cheap:* hand-add this wave's new blocking contracts to the const. *Robust:* derive the
owe-set.

**The decisive argument** ([advocates/small-forks.md](advocates/small-forks.md) fork A's closing scope item, on
gap-findings G-21/G-35):
**nothing reddens when a new ambush-class contract stays off a hand-list** — silent by construction,
and M51's Tier 0 mints exactly the shape the const encodes (a contract that first appears in its own
block message). Both pack-load fences were **driven red** under `JIGC_PACK_DIR`, so the teeth are
real; the membership is what is unfenced. **N26's recorded trigger fires verbatim at this Settle**
(*"the stated-at fence's owe-set is next opened"*).

**Decided (the human, 2026-09-10/11):** `AMBUSH_CLASS_CODES` is **derived rather than hand-listed**,
and **N26's two questions are answered**: the stated-at fence's **subject is packs that ship steps**
(not packs that ship a manifest), and **an unstamped managed corpus gets a store-surface answer** —
its exact shape owed at decompose.

**Structural constraint the plan inherits:** `finalize.carried-staged`'s declarer is `step:finalize`,
and **no pack step of either pack solicits `jigc setup`**, so D3's setup-side carryover contract has
no home the fence's own route can name today — which is part of why the owe-set is derived rather
than extended by hand.

**Discharges:** G-21 · G-35, and N26 graduates out of its carried entry.

---

### D11 — EC-10: a fence on the **weakest checkable claim**

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §15]*

**The fork.** *Cheap:* prose again — for the sixth consecutive wave. *Robust:* a fence.

**The decisive argument** ([advocates/small-forks.md](advocates/small-forks.md) fork A; gap-findings G-36): `foldback_truth.rs:223`
declines the version assertion **by name** — and that rationale holds for choosing a **numeral** and
is **silent on comparing two homes**. So the robust arm is a **narrowing of a recorded refusal with
its rationale engaged** — the repo's standing form — not an override.

**Decided (the human, 2026-09-10/11):** a fence on the claim *the version the fold-back names is the
version `Cargo.toml` carries*, landed as that narrowing. **The pack-step home is refused with its
ground:** `completion.yaml` ships into every adopter repo, and the version it would name is jigc's
own `Cargo.toml` — a law-1 lie for every reader who is not this repo (`re-verify.yaml` already models
the right register). Side effect priced: a new pack step would take the tree to 70 and make G-31's
fence cell disappear by accident.

**Discharges:** G-36.

---

### D12 — N23: the **deregistration** detect+route arm

*[Amended 2026-09-15 at the build: see [Review amendments](#review-amendments-2026-09-11) §20 — the `orphaned` row's `id` is **`null`** (the stamp names no type), and the derived owe-set **unions a declared-identifier const** so `finalize.left-out` survives without a hand-stated row.]*

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §9 · §10 · **§18**, which names the code `schema-conformance.orphaned-instance` and partitions it against D10's]*

**The fork.** *Cheap:* leave it — today's reachable population needs `JIGC_PACK_DIR` or a PB-1 project
pack dropping a doctype it owns. *Robust:* a doctype leaving the resolved set **routes its orphaned
instances**.

**The advocate's decisive argument** ([advocates/small-forks.md](advocates/small-forks.md), fork C): the hole is a **false green in the
adopter's CI**. Driven twice: with a doctype out of the resolved set, `jigc validate` prints *"no
findings — the committed store validates clean"* at **exit 0**, `jigc doc list` **drops the rows
entirely**, and `git ls-files` still carries every file — while `MIGRATING.md` ships into every
adopter repo telling them to CI-gate on that verb. *"A green meaning **I stopped looking at these
files** is the sharpest law-1 lie available, because the reader delegated the check to it."* And
**narrow reachability is a claim about the calendar**: PB-1 shipped at M49 as *the documented way an
adopter owns doctypes*, with a declared bound making divergence expected; the population is narrow
because there are no adopters yet — the population 1.0.0 exists to create.

**Decided (the human, 2026-09-10/11):** the arm ships **beside `orphaned_docs`** — a committed `.md`
**carrying a jigc stamp at a home no resolved doctype claims** (computable at HEAD:
`schema_version_from_front_matter` reads the stamp with no parse and no schema; `is_unadopted_foreign`
cannot be reused, it takes a `&Schema` that by construction no longer exists) — with a **`Human`
route** (re-add the pack defining the type, or `jigc unmanage`) and **`doc list` printing the row
instead of dropping it**. Argued on **leg 0's second clause** — a hole in a declared surface — since a
new code plus a route is additive, not one-way. **N23's trigger is recorded as fired.**

**Discharges:** G-38.

---

### D13 — the razor's **ground** is corrected; F-3 and F-8 stay out on the **necessity** leg

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §16]*

**The fork** (G-39): the charter refuses *any schema-shape change to a frozen doctype* as a class on
the ground that it is **additive after the pin**. For a **new** doctype that inference holds — the
methodology manifest says so in its own words (*a new doctype is free at the freeze*). For a **shape
change to an existing** frozen doctype it **runs the other way**: a bump today ships a migration over
**zero** adopter corpora; the same bump after 1.0.0 runs over **every adopter's store** and flips
their CI red until it does — textbook *expensive after the pin*.

**Decided (the human, 2026-09-10/11):** **the ground is corrected, the exclusion stands.** **F-3 and
F-8 stay OUT on the necessity leg** — no adopter needs them, and the M50 Settle refused the same edge
on `methodology-docs.md:42`'s universe rule — **not** on *"additive after the pin"*, which is
**inverted** for a shape change to an existing frozen doctype. Left unstated, a future wave would
cite the inversion as precedent.

**Flagged for later, with the pricing:** **F-8 is the natural first post-1.0 migration** — an
`AddedOptionalField`, a **stamp-only** migration — whenever its trigger fires; **F-3 is the expensive
one**, a change to the **schema-definition format**, itself frozen v1.

**Discharges:** G-39, and it is written into [charter.md](charter.md) as a dated
`[Corrected 2026-09-11 at the Settle: …]` bracket under the razor.

---

### D14 — the ledger question (fork 8): **closed under the stated criterion**

**The fork.** Read the conversion ledger **closed** under the stated criterion — every row carries
`pinned-by:` or a stated `UNPINNED:`, the test both prior closures used — or **substantively open**,
because F-3, F-5, F-9 and F-11 are CONFIRMED defects with no standing test. **The gate is the human's
by the record's own words.**

**Decided (the human, 2026-09-10/11): closed, under the stated criterion** — and **the criterion is
named for what it is**: *has a citation or a reason*, not *has a test*. Recorded alongside: **F-5,
F-9 and F-11 convert in M51 by their own fixes' red tests**, and **F-3 stays on its trigger** with its
`UNPINNED` reason standing.

---

### D15 — acceptance (fork 9): the **per-axis review**, no full blind trial

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §17 · **§19**, which adopts the acceptance design — nine flow-52 arms, the eight matrices' door-set derivations, and the strengthened coverage fence]*

**The fork.** The per-axis review is the acceptance instrument (decided by the human 2026-09-10);
**open** was whether a blind trial *also* runs before the 1.0.0 call, and if so at what scale.

**Decided (the human, 2026-09-10/11):**

- **The per-axis review on the built and installed `1.0.0-rc.15`** — not on the source, and **not
  before the audit's fixes land**, per the convention five consecutive waves have needed (EC-10).
  **Eight axes**, each a cell matrix driven end to end, each staffed as **one Opus driver plus one
  Codex source pass**, under the review's own fence: **every leaf in `VERB_KINDS` appears in at least
  one axis's cell matrix, or is named uncovered.**
- **No full blind trial.**
- **One blind re-measure of the duress cell only if the wave changes a surface a worker reads** —
  orientation, or the read-back fence.
- **Then the 1.0.0 call, which is the human's.**

**Consequence for the gate-record:** `acceptance-spiked` is filled against the eight matrices and that
fence, not against a flow with commands; fork 9 being settled is what lets that cell close.

---

### Review amendments (2026-09-11)

**What this is.** The fifteen decisions above were reviewed before decompose by two independent
readers, neither of whom authored what they reviewed: an Opus `design-reviewer` **driving the release
binary** ([design-review.md](design-review.md) — B1–B10 blocking, S1–S11 should-fix, A1–A8 advisory) and an unseeded
**Codex** source review ([codex-design-review.md](codex-design-review.md) — findings 1–10 plus a contradictions section; its
prompt, with the seven directed questions, at [codex-design-prompt.md](codex-design-prompt.md)). Both returned the same
verdict — *not ready to decompose* — and **the human accepted every finding and every recommendation
on 2026-09-11**.

The corrections are recorded **here**, dated, with a bracket at each decision they touch, rather than
folded silently into D1–D15: what the Settle decided and what the review corrected must stay
separable, because the next wave will cite both. **No decision reverses.** What moves is *mechanism*,
*scope* and *identity* — with one genuine scope narrowing, named as such (§3, `setup`'s exemption
from the unborn member).

**Four of the corrections are one shape, and it is the shape the reviewers both led with:** *a
decision that says "reuse the shipped predicate / the shipped mechanism" had not been checked against
the exact new shape, and the claim is false when driven* (§1, §2, §6, §7). Two of them — §1 and §2 —
meant the decided fix **did not cover the cell it was decided on**. That is the same class the repo
recorded at the M46 planning corrections (*a static read reaching a cleaner conclusion than the binary
supports*) and at M50's two planning halts (*a claim about how the composed product behaves is not
established by reading the files it is composed from*), landing this time inside a **Settle** rather
than a draft — which is the argument for the pre-decompose review existing at all.

---

#### §1 — D3: the predicate is **worktree-vs-HEAD, per path**; the `StagedSnapshot` pair is withdrawn

*(B1, driven; Codex 3.)*

D3's third bullet decided *"an in-memory pathspec-scoped `StagedSnapshot` pair"*. **That mechanism
cannot see the cell D3 was decided on**, on three checked facts: `git_staged_snapshot`
(`crates/cli/src/task.rs:3925`) runs `git diff --cached --raw -z`, whose subject is **the index vs
HEAD**; `setup` commits with `git commit --no-verify -m … -- <paths>` (`setup.rs:1709`), and a
**pathspec commit takes the worktree contents** of those paths; and `decide_carryover`
(`crates/engine/src/finalize.rs:~735`) carries a path only when its `(path, blob)` pair matches a
snapshot entry — *a restaged different blob is the door's own work*. So the pair produces **zero
findings** here. Driven on `dev/jigc-rig bare`, with an edit that was never staged at all — sharper
than the advocate's cell A, and carried by no artifact:

```
printf 'user line one\n' > CLAUDE.md; git add CLAUDE.md; git commit -m "add CLAUDE.md"
printf 'user line one\nUNSTAGED WIP SECRET\n' > CLAUDE.md     # index CLEAN, worktree dirty
jigc setup                                                     # EXIT=0
git show HEAD:CLAUDE.md   ->  user line one / UNSTAGED WIP SECRET   # rode the install commit
git status --short        ->  (empty — the tree ends clean, nothing prompts recovery)
```

**Amended.** The predicate is the one D3's own prose names: **refuse when any path in `setup`'s
pathspec is dirty relative to HEAD *before* `setup` writes** — worktree-vs-HEAD, asked **per path**.
The `StagedSnapshot` pair is **withdrawn**; `decide_carryover` is not this door's predicate.

**Idempotence and upgrade are clean by construction**, which is Codex 3's question answered without
its four-state attribution record: the question is asked **before any write**, so the pre-run
worktree-vs-HEAD comparison is the whole subject, and nothing `setup` subsequently writes can enter
it. A fresh clone of a repo where `setup` already ran is clean (every install path matches HEAD); an
upgrade is clean (same); a host file the user edited and did not commit is **exactly** the refusal
D3 exists to mint. The four-state model is refused as answering a question this ordering removes.

**The `MINT_DOORS`-is-untouched sentence stays** — it is true under the amended predicate as it was
under the withdrawn one — but it is recorded as **not load-bearing**: it never was what made the
mechanism correct.

---

#### §2 — D1: three predicates, not one; a typed sink value; occurrence-keyed registry; a **source** rule; and the trackedness leg

*(B4 driven + doc-cited; B5 driven; Codex 1, 6, 10; S11.)*

**The door (B4).** D1 part 1 said *"`trackable::untrackable_reason` + `is_workbench_root` … shipped
predicates, **no new capability**"*, and F1's decisive table rested on *"`config set docs-root`
refuses all three escape shapes … the predicate is shipped and `migrate.rs` never asks it"*. The
absolute cell — EC-1's own Tier-0 cell — **does not transfer**: `crates/cli/src/trackable.rs:56` opens
with `let relative = relative.trim_matches('/')`, its doc-comment types the parameter as
*repo-root-relative*, and `resolve()` joins the trimmed value onto the root, so
`/private/tmp/victim/keepme.md` is re-read as `<repo>/private/tmp/victim/keepme.md` and the predicate
answers **`None` (trackable)**. A locked design doc says so in its own words
(`design/storage.md:177`: *"`crate::trackable` **trims the leading `/`**"*). The `config set` door
refuses that shape through a **third** predicate D1 never named — `unusable_root_reason`.

**Amended.** The door **resolves the caller token to a repo-relative path, or refuses an absolute one
outright**, and then asks **all three** predicates: `untrackable_reason` · `is_workbench_root` ·
`unusable_root_reason`'s **absolute and symlink legs**. ***"No new capability" is struck*** — the
resolve-or-refuse step is a new rule, small and stated, and three legs transfer rather than two.

**The sink (Codex 1, Codex 10).** `plan.retirements` is validated CLI-side, as decided — with three
additions:

- **A typed `ValidatedRetirement`** carrying the **normalized literal path** and the **bound
  repository identity**, which **all five** `read_source_path` consumers must accept; the raw read
  stays **private to the loader**, so a raw string cannot substitute for a validated one at any
  consumer. `retire_exempt` (`crates/engine/src/finalize.rs:738`) derives from that same value and
  never re-normalizes independently.
- **Validation immediately before the unlink, in the same function** — not once before the commit
  closure is entered, which would span promotion, staging and hooks and widen the substitution
  window.
- **Git pathspec magic is refused**: a leading `:` in any recorded path that would reach git is
  rejected. `--` prevents option parsing, never pathspec magic such as `:(top)…`.

**A descriptor-held traversal is REFUSED, as a declared bound.** Codex 1's race-resistant primitive
(open the repo root and walk components without following symlinks, then unlink relative to the held
descriptor, failing closed where unsupported) is **not built**: the substitution it closes requires a
**second writer inside the repository during the commit closure**, and jigc's model is single-writer
at a work unit ([storage.md](../../../design/storage.md) → Concurrent writers bounds the one exception, and it is the shared
`.jigc/` caches, not the worktree). The bound is **recorded rather than left as silence**, with its
reopening condition named: if a door ever writes the worktree concurrently with `finalize`, the
narrowing becomes insufficient and the primitive is owed.

**The registry (Codex 6).** Keyed by **`(leaf, argument id, conditional arm)` occurrences, not
deduplicated ids** — one id appears at semantically different leaves (`path` is `migrate`'s input
*and* `unmanage`'s; `file` occurs at `insert-step` and `replace-step`; `value` is path-like only when
`key ∈ ROOT_KNOBS`; `from_file` carries the `-` stdin sentinel, which is not a path). **The ⇔ fence
covers occurrences**, so a new leaf reusing an existing id reddens.

**`file` and `from_file` get a SOURCE rule (S11).** D1 part 1 had pre-committed them to a
*destination* predicate, which is the wrong question and narrows a shipped affordance twice over:
`untrackable_reason` would refuse a team steps library at `~/steps/foo.yaml`, and `is_workbench_root`
refuses any path under `.jigc` — precisely where `insert-step` **writes**
(`.jigc/config/steps/<basename>.yaml`). Their shared rule is: **readable · no `.git` component · not
reached through the workbench**. **An out-of-repo source is allowed**, with the reason stated: the
caller names it and the bytes land **visibly** (copied in, reviewable in the diff) — the driven harm
was reading `.git/config` into composed step text, not out-of-repo-ness.

**The trackedness leg (B5), and with it the G-5 re-drive discharged.** Both predicates D1 named are
**location** predicates; neither asks whether the source is **tracked**. Driven end to end on
`dev/jigc-rig fresh`: an untracked in-repo `HISTORY.md`, `jigc migrate HISTORY.md --as changelog`,
authored, `jigc task finalize <id> --approve` → `1 file committed` (the promoted `CHANGELOG.md`), the
source **gone from disk**, and `git log --all -- HISTORY.md` **empty** — the bytes exist in no git
object, and the deletion is named on **no** surface. That falsifies the recorded warrant for keeping
the deny floor's blanket permit (*"a guarded migrate destroys only a reviewed, in-repo,
**git-recoverable** file"*).

**Amended.** An **untracked in-repo migrate source is refused**, with a **`Human` route naming
`git add`** — M45's owner-artifact route is the precedent. The permit **stays**, and its warrant
becomes **true** rather than restated: after the leg, every admissible source is tracked and
therefore recoverable.

**This is recorded as the discharge of the Settle's own owed G-5 re-drive.** The fenced
`migration-finalize` step sentence (*"the deletion staged … a `git rm` in effect"*,
`crates/cli/pack/steps/migration-finalize.yaml:10-11` and its methodology twin) is **false today for
the in-repo-untracked cell** — driven above: nothing is staged and the removal rides no commit — and
the trackedness leg **makes it true**, because the only cell that falsified it is now refused. No
pack repair is owed; the re-drive is answered and leaves *Also owed, smaller*.

**Rider, so *"reuse"* stays accurate:** `is_workbench_root` is a private `fn`
(`crates/cli/src/config.rs:622`) and `engine::validate::schema_version_from_front_matter` is
`pub(crate)`; both need a visibility change to be reused as D1 and D12 describe.

---

#### §3 — D2: `setup` is **exempt from the unborn member**; the registry is a **total** classification; the posture subject is typed and re-probed; refusals carry **no override**

*(B2 driven; S1, S2, S3; Codex 2, 7.)*

**`setup` is EXEMPT from the unborn member**, with the M30 audit rationale **quoted** rather than
paraphrased — the repo's basis-unchanged form (`crates/cli/src/setup.rs:1617-1625`):

> Require a git work tree — but **DO mint on an unborn HEAD** (a brand-new repo with no commits).
> Setup owns committing its own install footprint regardless of HEAD state (**M30 audit finding 1**):
> … if setup skipped the install here, `CLAUDE.md`/`.claude/settings.json`/`.jigc/AGENT.md` would be
> left untracked after the first managed commit.

Driven, `git init -q . && jigc setup` lands `chore(jigc): install jigc workspace config` at exit 0
today. Applying the family uniformly would turn **the QUICKSTART on-ramp — the first command an
adopter runs** — into a refusal routed at `jigc setup --force`, training the exact reflex D3 itself
prices as its honest weakness. **The detached and operation-in-progress members still apply at
`setup`**; only *unborn* is exempt, and the exemption is a stated `Exempt(reason)` row rather than a
silent hole.

**The claim gains the symmetric posture bound (S1).** The honest bound under the claim covered the
`Plain` family only, while D2 declares the **`GIT_DIR` redirect out** — a repository posture that
reaches a committing door unadjudicated (the baseline drove `milestone create` writing the record into
repo A and landing the commit in repo B at exit 0). The claim now carries both halves: **the posture
family closes over three of four driven members; the `GIT_DIR` redirect is declared out with its
rationale, and its scope is reopenable.** Stating one half and not the other is the
false-completeness shape this wave exists to correct.

**The registry is a TOTAL classification of every clap leaf (S2, Codex 7)** — the `VERB_KINDS` mold,
so a new verb **reddens until someone answers it** — into **commit-on-behalf** / **move-on-behalf** /
**neither**, with **`COMMITTING_DOORS ⊆ commit-on-behalf` asserted**. A lower bound plus one named
element is not a membership rule, and three consumers read this set.

**The two classes take different postures**, because *"or move"* widens the subject past committing
and N20's cause vocabulary is meaningless at a non-committing mover:

- **Movers** (`config set docs-root`/`placement-root`, `jigc relocate`, `jigc rename`,
  `migrate-corpus`'s relocation arm) refuse **only operation-in-progress**.
- **Committing doors** refuse the **full family** (detached · unborn · operation-in-progress), with
  `setup`'s unborn exemption above.

**The posture subject is typed and re-probed at the seam (Codex 2).** `LiveCheckout { repo, branch }`
/ `DedicatedWorktree`, **re-probed immediately before every `git commit`, every `git mv` batch and
the live-checkout `merge --ff-only`** — not only at the door, because an ordinary commit still reaches
`git_commit_capture` (`crates/cli/src/task.rs:4255`) and a hook, a concurrent process or an earlier
phase can move HEAD in between. The probe verifies **repository identity and expected ref**, not
merely *HEAD is attached*. **`DedicatedWorktree`'s constructor is private to the worktree type** — a
public variant or a caller-supplied boolean is forgeable inside the codebase, which is the class M50's
audit condemned.

**Posture refusals carry NO override (S3).** D2's *"`--force` is the single consent"* is **withdrawn
for the posture family**: the route **names the git command** that resolves the posture (`git switch
<branch>`, `git merge --abort`, a first commit), because a posture is a repository state the user can
resolve, not bytes only they can value. So **`--force` at `jigc setup` carries exactly one meaning —
D3's consent** — and the consent-collapse S3 identified cannot occur.

---

#### §4 — D3 × D10: the ambush owe-set's **derivation is stated**, and `setup`'s code is its first exempt row

*(B3; B10; the Codex contradictions section.)*

D10 decided `AMBUSH_CLASS_CODES` is *"derived rather than hand-listed"* and named no source, while
D3 mints a blocking setup-side contract **no pack step can declare** — `assert_stated_at`
(`crates/cli/src/pack.rs:803-830`) requires at least one declarer among **each manifest-shipping
pack's own steps**, checked in isolation, and no step of either pack solicits `jigc setup`. A
derivation that picks the new code up therefore **reddens pack-load at every door**, exit 1, with no
legal declarer — deriving the owe-set does not solve that, it causes it.

**Amended — the derivation is written down, in three parts:**

- **Source set:** *blocking codes minted by a door in the **commit-on-behalf** class of D2's
  registry* (§3). Not *every blocking code* (far too wide — `conformance.*`, `write.*` are not
  ambush contracts), not *codes some step already declares* (circular and toothless).
- **Exclusion rule:** minus rows carrying a stated **`Exempt(<reason>)`** — the repo's standing
  shape.
- **The first exempt row is D3's setup-side carryover code**, with its reason: *no pack step of
  either pack solicits `jigc setup`, so the fence's own route has no home to name; declaring it on
  `step:finalize` would be a law-1 lie and would additionally have to buy tokens under
  `CONSTRAINT_REQUIRED_TOKENS` (`pack.rs:1242`).*

With the source set and the exclusion rule stated, a red test is writable and both decisions land
together — which is what the Codex contradictions section asked for: the store-surface answer and the
setup route are settled **together**, not left to separate increments.

---

#### §5 — D3: `--no-verify` is **kept**, on a new recorded basis

*(S4.)*

D3 corrected `setup.rs:1603`'s rationale (*"the only hook present is the warn-only `pre-commit`
setup just installed"* — driven false: setup **preserves** a pre-existing hook and then skips it) and
kept the behaviour, leaving it with a **struck basis and no replacement**. Under this repo's own
discipline that needs a re-recorded basis or a reversal.

**Amended — kept, with the basis written:** after D3's refusal lands, **the install commit carries
only bytes `setup` itself wrote** (that is precisely what the refusal enforces), and **jigc's own
just-installed hook must not self-trigger on the commit that installs it**. The user-hook half of the
old rationale is **struck with its falsifying datum**; the self-trigger half survives and now carries
the whole behaviour, which it can only do *because* the refusal closed the cell the first half was
covering. **The two shipped guides' universal is scoped to the task and milestone doors** — the doors
where a user hook is policy over the user's own work.

---

#### §6 — D4: **compare-and-swap** restore, two named files, and a byte algorithm for amend-to-union

*(Codex 4, 8; A1.)*

- **Compare-and-swap restore (Codex 4).** Every worktree rollback entry carries the **pre-image**,
  the **exact post-write image jigc produced**, and file identity where available. **Restore only if
  the current bytes still equal jigc's post-write image**; if they differ, **do not overwrite** —
  emit a **rollback-conflict finding** and preserve **both** versions. Absence restoration takes the
  identical rule. The interval spans promotion, retirement, ignore maintenance, staging and hooks, so
  an unconditional restore can destroy a concurrent edit — the same loss D4 exists to prevent, in the
  other direction.
- **The subject is `.jigc/.gitignore` and `.jigc/version` (A1)** — the two files `finalize`
  rewrites — **not** the config *directory*. `jigc_config_layer_pathspecs()` is three specs, one of
  which (`.jigc/config`) is a directory, and a worktree pre-image family over a directory is a
  different shape (absent-means-delete over N files). Named now so the build does not capture a
  directory.
- **Amend-to-union gets a byte algorithm (Codex 8).** **Preserve all existing bytes exactly; append
  only missing canonical entries in fixed order; insert exactly one separator newline only when
  required; never normalize or deduplicate existing content; reject non-regular, symlinked or
  undecodable files.** Comments, blank lines, CRLF, a missing final newline and pre-existing
  duplicates therefore survive untouched, and the result is byte-idempotent by construction rather
  than by inspection.
- **The *"one capture point covers every arm"* claim is narrowed to the finalize arms (A1).**
  `gitignore::ensure` has **three other production callers** outside that transaction —
  `adapter.rs:1108` (setup), `milestone.rs:472` (`milestone create`) and `milestone.rs:2066`
  (`milestone provision`, which **never commits**, so on an `ENTRIES` upgrade a private line would
  die there with no transaction at all). Those three are covered by **the amend**, not by the
  capture, and the record says so.
- **`RecordPreImage` transfers its *discipline*, not its type** (`milestone.rs:705` is module-private
  and single-path) — one word, so a plan does not go looking for a reusable primitive.

---

#### §7 — D5: a production `EnvelopeArm`, four proofs, the census adopted as the table, and the numeral struck

*(B8; Codex 5, 9; A6, A7; S5; [envelope-key-census.md](envelope-key-census.md).)*

- **A production `EnvelopeArm` enumeration (Codex 5).** *Arm* was undefined, so the `(path, arm)`
  bijection was not sound — clap enumerates **syntax**, never runtime result variants. The
  enumeration is **60 arms** per the census (58 success arms over 47 leaf verbs, plus the two
  cross-cutting reject arms), **derived from the result enums where they exist** —
  `engine::result::OrientationView`, `cli::render::DocAck`, `TaskAck`, `ConfigAck` — and
  **hand-enumerated with a stated reason for the 11 arms no enum can generate**, `jigc task
  finalize`'s **four** (landed · `--dry-run` · blocked · the exit-4 review hold) among them.
- **Four proofs**, which is what makes *"every verb × arm"* an implementable completeness claim:
  **(1)** every clap leaf has **≥ 1** arm; **(2)** every production arm has **exactly one** registry
  row; **(3)** every registry row is **driven**; **(4)** the **driven** key set equals the
  **declared** key set.
- **The fence is hosted at `crates/cli/tests/format_json_success_axis.rs`** — the one suite that
  already drives all 47 leaves to a real success through the real binary. `text_json_parity_axis.rs`
  renders a witness and therefore **cannot see an arm the dispatch chooses**.
- **The registry is production-side (A6)**, not test-side: so the contract doc may **name** it as the
  list, and the binary may **read** it. A6's question was not a filing question — the two sides differ
  in what can read them, and D5 pointed a locked 1.0 contract doc at the artifact.
- **The *"17 undeclared keys"* numeral is STRUCK (B8).** It was already inconsistent with the list it
  summarized, and D5's whole argument is that disposition must precede the pin — a table sized at 17
  would ship the remainder **blessed by omission on 1.0.0 day**, the precise failure D5 exists to
  prevent, in the wave whose D8 fences hand-counts. Driven, the class is **57 (verb, key) pairs
  across 23 verbs** (census §3), and **the set is derived at build**, not carried as a numeral.
- **The census's dispositions are adopted as D5's table** — it is no longer decompose-owed, because
  *declaring a key is the one-way act D5 exists to govern* (Codex 9) and an increment author must not
  settle it:
  - **Declare 53**, each with the paragraph the contract requires, in the home the census names.
  - **Delete four constants**: `installed` (`jigc setup`) · `uninstalled` (`jigc uninstall`) ·
    `review` (`task finalize`'s exit-4 hold) · `hook_output` on **`milestone list-tasks`** (a
    `VerbKind::Read` verb commits nothing, so no hook can ever speak). The first three duplicate a
    fact the **exit code** already carries — the retired-`discarded` precedent, verbatim.
  - **7 arms `Unpinned(<reason>)`** — and they are exactly the ones whose value is **composed
    prose**: `describe`'s menu and the six `render::milestone` acks.
  - **The constant rule, stated:** *a constant ships iff it is declared as one and its reason is
    written down* — which is what separates the four deletes from the two deliberate constants
    already on the record (`committed: false` on all six `ConfigAck`s; `findings: []` on
    `task diff`/`task bind`/`task discard`).
- **D6's policy paragraph states the pre-pin removal rule (A7).** Both posture homes authorize
  **additive** keys pre-1.0 and D6's new policy governs **inside 1.x**; **nothing stated the rule for
  a removal *before* the pin** — which is the sentence that authorizes the four deletes above. D6
  writes it rather than the wave relying on an unwritten rule.
- **The `--dry-run` equal-set fence is restated (S5).** `Tier::Previewed` holds **blocking** members
  (`finalize.carried-staged` among them), and in the state that makes one fire, `task finalize` does
  not land — so a three-way equality including *landed* has no value for exactly the members that
  matter. The fence is: **`validate` == `--dry-run` == the committing door's emission (landed *or*
  blocked)**.

---

#### §8 — D7: the 22 doors emit the **findings envelope**, and `coc:202`'s sentence is split

*(B7; A8.)*

D7 was ambiguous between its own fork's two arms, and only one discharges F-5's stated harm: the
contract lists `finalize.no-task` under the work-unit target form (`design/command-output-contract.md:172`), i.e.
as a finding that **projects a key**, while `:202` is the **flattened-string** paragraph. A code
inside a message is not a key, so the flattened arm leaves the charter's harm — *a driver keying on
the stable `(code, target)` pair gets nothing at 22 doors* — unrepaired.

**Amended.** The 22 doors emit **the findings envelope**. This is a **wire change on 22 verbs**, and
it is recorded as such: it ships as **a declared row of D5's registry**, inside the open pre-pin
window, which is what *"every fork resolved robust"* implies here.

**`command-output-contract.md:202`'s sentence splits in two (A8)**, so the edit is not a silent
rewrite: the **behaviour** half (*the flattened error carries the code and the locus*) is **made
true** at these doors; the **provenance** half (*"since M50 Increment 10"* — the clause that makes it
read as settled) is **struck with its falsifying datum**.

---

#### §9 — D12: the orphan code joins **`STORE_EXIT_FLIPS`**, and `doc list` gains a declared third state

*(B6; A5.)*

- **Exit semantics.** D12 was admitted on *"a **false green** on the verb `MIGRATING.md` tells
  adopters to CI-gate on"*, but store-scope content findings are report-only at exit 0 unless the code
  is a member of `cli::render::STORE_EXIT_FLIPS` (`crates/cli/src/render.rs:909`). **The orphan code
  joins `STORE_EXIT_FLIPS`** — otherwise `jigc validate` still exits 0, the adopter's CI is still
  green, and the decision's own justification is undischarged. **Precedents:** M42's
  `schema-conformance.schema-version-current` and M46's `schema-conformance.unadopted-instance`, both
  members. (D12's *"additive, not one-way"* argument is about **minting a code plus a route**, and it
  stands; it was never an argument for leaving the exit rule unmoved.)
- **`doc list` gains a declared third `state` value: `orphaned`.**
  `design/doc-read-surface.md:159` declares the enumeration **exhaustively** (*`managed` (stamped)*
  or *`unregistered` (no stamp…)*), and an orphaned instance **is** stamped (so not `unregistered`)
  and managed by nothing (so `managed` is a law-1 lie). Changing a declared enumeration on a pinned
  contract therefore ships as **a D5 row with its own declared paragraph**.
- **Its two remaining answers are stated, not left to the plan.** `id` carries the identity **as
  recorded in the stamp**, named in the paragraph as *read from the front matter, not resolved
  through a schema* — the type it names is defined by no resolved schema, and saying so is the honest
  render. `item-count` is **`null`**, and the reason is that the shipped rule does not reach this
  population: `doc-read-surface.md:159` makes the count **best-effort, `0` for an instance that
  does not parse against its current schema** — and an orphan has **no schema at all**, so `0`
  would be indistinguishable from *parsed, and empty*. The `0` rule stands unchanged for the
  population it was written for (unregistered foreign files, stale-shape managed docs); `null` is a
  **third** answer for a third population, declared in the same D5 paragraph as the `orphaned`
  state — because a number invented here would be the law-1 lie D12 was admitted to close.
- **D10's unstamped-managed answer and D12's orphan finding are ONE condition with two causes
  (A5)** — *jigc's store surface is silent about a file it should speak for*. **The partition is
  named at decompose** (which cause each surface answers, under which code), so the plan does not
  mint two codes for one condition or leave the seam between them open.

---

#### §10 — the new codes: every refusal this wave mints gets its identity **now**

*(B9 — the census of eleven under-specified mints, each previously missing its surface, severity,
route kind and registry identity.)*

**The shape, decided once for the family:** every refusal this wave mints is a **blocking `Finding`
on the destroying-door mold** — a **code**, a **`Human` route naming the git command** (or the act)
that resolves the state, and **exit 1**. **None joins `ERROR_CODE_REGISTRY`**: that registry mirrors
**door identities** derived from `COMMITTING_DOORS` (`design/surface-contract.md:131-142`), and a
blocking `Finding` is not an `Outcome` identity — so D2's wider registry does not move the mirror's
subject. **Each is registered in `validation.md`'s inventory** (severity class ·
intrinsic-or-tunable · keyed/unkeyed), which is the home the Scope-confirmed batch already named.

| mint | code |
|---|---|
| D2 — HEAD detached | `repo.head-detached` |
| D2 — HEAD unborn | `repo.head-unborn` |
| D2 — an operation in progress | `repo.operation-in-progress` |
| D1 — `jigc migrate <path>`, the location legs | `migrate.source-untrackable` |
| D1 — `jigc migrate <path>`, the trackedness leg (§2) | `migrate.source-untracked` |
| D1 — the sink, before `retire` | `finalize.retire-untrackable` |
| D1 — `config insert-step` / `replace-step` `<file>` | `config.step-source-untrackable` |
| D12 — the orphaned-instance finding | D12's orphan code (§9) |
| Scope confirmed — `jigc ingest`'s identity leg | `ingest`'s identity refusal |
| EC-28 — the OS name ceiling at three `SLUG_DOORS` (§12) | EC-28's name-ceiling code |

**Two riders on the table.** `finalize.retire-untrackable` fires **inside the commit closure**, so it
**rolls back the promote** — it is a refusal of the whole transaction, not of one step, and the
rollback rides the captured-pre-image discipline the wave is already extending (§6). **D12's orphan
code is the one row whose exit rule is not the mold's exit 1**: it is a store-scope finding, and its
exit behaviour is its `STORE_EXIT_FLIPS` membership (§9).

Rows 7 and 8 of B9's census — D3's `CarryoverBoundary::Setup` and D10's store-surface answer — were
already carried as **owed shapes** and stay owed, now under this family's decided mold.

---

#### §11 — D8: the `CELLS` datum is struck, and the no-code-side-set residue is named

*(S6; A3.)*

- **`CELLS` is 63, not 64.** D8's headline datum — *"was **63** when written and is **64** at HEAD …
  a 100 % defect rate with a half-life shorter than the wave"* — is itself a miscount, inside the
  decision **about** false counts. Driven at HEAD:
  `awk '/pub const CELLS/,/^\];/' crates/cli/tests/support/write_miss_cells.rs | grep -c "^    Cell {"`
  → **63**; the 64th `Cell {` match is the **`struct Cell` declaration** at
  `crates/cli/tests/support/write_miss_cells.rs:108`. **The count has not moved since the evidence
  check.** The datum is **struck with its falsifying datum**, here and in the DECISIONS entry that
  repeats it — the decision (*fence, don't rewrite*) stands without it.
- **The residue class joins D8 (A3).** Some EC-14 members have **no code-side set to fence against**
  (`design/doc-read-surface.md:175` heads *"five regimes"* over a six-row table; the dev and
  methodology manifest headers restate one historical fact in two files), and a fixer told *"fence,
  don't rewrite"* has nothing to reach for — the instruction would halt them. **Counts over sets the
  code cannot move are corrected in place, and historical ones are dated-bracketed**, with that
  one-line reason on the record.

---

#### §12 — EC-28 and N15: two in-scope charter rows get their dispositions

*(S8 — both had zero mentions anywhere in the Settle, and both are more than wording.)*

- **EC-28 → a name-ceiling predicate at `SLUG_DOORS`, with a code and a route.** Three doors
  (`doc rename`, `doc create`, `start … --slug <300>`) fail on the OS name ceiling and answer
  `task.working-area-io` routed *"resolve the underlying I/O condition (a disk or permissions
  problem…)"* — **a law-1 lie on a door the wave's claim covers**. `SLUG_DOORS` gains the
  filesystem-length predicate; the code is §10's table row; the route names the ceiling and what to
  do about it. G-57's question is **answered**, not left silent.
- **N15 → the `store.not-found` `--task` arm names its copy and keeps `--task` in the route.** A
  `--task` read of an address resolving nowhere is byte-identical to the task-less one, says
  *committed*, and its route **drops `--task`**. The arm says which copy it looked in (staged vs
  committed) and the route **retains `--task <id>`**. It touches the **1.0-pinned read surface's**
  route and value, so it ships as **a D5 row**.

---

#### §13 — D3: the widened carryover sentence is **scoped**, and the two shipping guides join the batch

*(S9.)*

D3 widens `surface-contract.md:123` and `finalize.md:159` from *"every task-minting door"* to *"any
door committing paths it does not own"*. **As a universal that is false at HEAD** for every committing
door that does not snapshot — `jigc rename`, `migrate-corpus`, the milestone record-only doors — so
the repair would ship **a fresh law-1 overclaim in the wave whose Tier 2 is law-1 overclaims.**
**Amended: the widened sentence is scoped to the commit-on-behalf class of D2's registry** (§3).

**And two shipping guides join the single batched hash move**, neither of which was in the
doc-consequence set or in G-61's consolidated batch: **`MIGRATING.md:38`** and
**`QUICKSTART.md:176`**, both stating the gate's subject, both `include_str!`'d into the installed
`SKILL.md` — while D3 adds a refusal at **the first command an adopter runs**.

---

#### §14 — D9: the *"independently versioned"* phrase moves in the same edit

*(A2.)*

D9's reading of `design/measurement.md` is correct and was verified in context (*independent of each
other*), but after the log is **declared unversioned**, the closing sentence still says *versioned* of
that surface. **It moves in the same edit** — e.g. *"independently **governed** surfaces; the
invocation log is declared unversioned and additive-only, with `binary_version` the per-record
discriminator"* — rather than being left for a later reader to trip over.

---

#### §15 — D11: admitted by the human's boundary, with razor leg 2 **answered explicitly**

*(A4.)*

The razor's leg 2 is *"right for **any adopter** — if the only beneficiary is our layout, our naming
or our process, it is refused."* A fence asserting that CLAUDE.md's fold-back paragraph names the
version `Cargo.toml` carries benefits **this repo's process only**, and the charter excludes the
harness-surface wave on exactly that ground. D11 argued the foreclosed-by-doc narrowing
(`foldback_truth.rs:223`) and never leg 2 or leg 0.

**Amended, and written rather than assumed: D11 is admitted by the human's boundary** — the wave takes
**every** ledger row, EC-10 included — **and leg 2 is answered explicitly: it would refuse D11 on its
face.** The admission rests on the boundary decision, **not** on the razor. Recorded so no future wave
cites D11 as a razor precedent — which matters here because D13 makes razor consistency this wave's
own subject.

---

#### §16 — D13: the razor gains **leg 0b**, shown to refuse

*(S7.)*

D13 corrected the razor's ground and then kept F-3/F-8 out on *"the **necessity** leg — no adopter
needs them"* — **a test the razor as written does not contain**, and one that also dissolves the M50
precedent D13 leans on (M50 refused the same edge partly because *"`AddedOptionalField` makes the
edge a one-byte-line 1.1 addition"*, the premise D13 has just falsified). As written, a future wave
inherits a razor whose refusal is **unciteable**.

**Amended — the leg is added, in the charter, beside D13's dated bracket:**

> **Leg 0b** — *needed by a real adopter today; an expensive-but-bounded, trigger-recorded cost is
> deferrable.*

**And it is shown to bite in both directions**, which is the repo's test for a razor leg being real:
it **refuses F-3 and F-8** (both expensive, both bounded, both carrying a recorded trigger), and it
**admits nothing new in scope** — every in-scope row is already admitted by leg 0's first clause (a
one-way door at the pin) or its second (a hole in a declared surface), so leg 0b changes no admission
this wave makes.

---

#### §17 — D15: the coverage fence requires a **reason**, *cell matrix* is defined, and **flow 52 ships**

*(S10.)*

- **`uncovered(<reason>)` is required.** *"Named uncovered"* carried no reason requirement, so the
  fence was satisfiable by listing all 47 `VERB_KINDS` leaves as uncovered. Every sibling exemption
  in this repo carries one — `Snapshot::Exempt(<reason>)`, `Unpinned(<reason>)`,
  `suppressed: {reason, expires}`, `is_route_exempt`'s enumeration — and this one now does too.
- **"Cell matrix" is defined**, so *"a verb appears in an axis's matrix"* is falsifiable: **an axis's
  cell matrix is a written table of `(door, cell)` rows whose cells are driven, and a verb is covered
  iff it is the door of at least one driven row.**
- **M51 ships a flow-52 chapter in [design/worked-examples.md](../../../design/worked-examples.md) and a
  `crates/cli/tests/flow52_acceptance.rs` suite**, on the mold every wave since M40 has used — each
  arm **naming the kind of set it iterates**. The review is the *acceptance instrument*; it leaves
  **nothing standing behind it**, and a suite does. The arm set is owed at decompose.

---

**§18 and §19 post-date the review bake-in.** The sixteen entries above are *corrections* the two
reviewers produced and the human accepted. The two below are **decisions the human took on
2026-09-11 after that bake-in had settled** — each closing one of the two cells the
[planning gate-record](planning-gate-record.md) recorded as **STILL HALTED**, and each therefore
removing an item from *Owed at decompose* rather than adding one. They are numbered into this section
because they are dated the same day and because a reader of D10/D12/D15 must reach them from the
decision they move.

---

#### §18 — D10 × D12: the partition is **two codes, split on whether the doctype resolves**

*(Discharges [planning gate-record](planning-gate-record.md) → HALT cell 4 (`design-complete` row 12)
and *Owed at decompose* item 3. Settles what §9/A5 named as **one condition with two causes** —
*jigc's store surface is silent about a file it should speak for* — without minting two codes for one
cause or leaving the seam between them open.)*

**The discriminator, decided:** *does the file's declared doctype resolve to a schema in the composed
set?* It is asked once, and each answer has exactly one owner.

**D12 — the doctype does NOT resolve: `schema-conformance.orphaned-instance`.**

- **Subject:** a **committed `.md` carrying a jigc stamp whose doctype no resolved schema defines** —
  computable at HEAD by `schema_version_from_front_matter`, which reads the stamp with no parse and
  no schema (D12; the `pub(crate)` visibility rider of §2 applies).
- **Severity:** **blocking at store scope**, and it **joins `cli::render::STORE_EXIT_FLIPS` as its
  sixth member** (§9), so `jigc validate` exits non-zero rather than printing *"the committed store
  validates clean"* over files `git ls-files` still carries. Precedents unchanged: M42's
  `schema-conformance.schema-version-current`, M46's `schema-conformance.unadopted-instance`.
- **Route: `Human`** — *re-add the pack that defines the type, or `jigc unmanage <path>`*. Two exits,
  both runnable, neither inventing a schema jigc does not have.
- **`jigc doc list`** prints the row instead of dropping it, in the declared third `state`
  **`orphaned`**, with **`id` read from the stamp** (named in the paragraph as *read from the front
  matter, not resolved through a schema*) and **`item-count: null`** — the third answer for a third
  population, because `0` would be indistinguishable from *parsed, and empty* (§9).

**D10 — the doctype DOES resolve: `schema-conformance.unversioned-doctype`.**

- **Subject:** a doc **whose doctype is resolved** but **whose owning pack ships no manifest entry for
  it**, so no stamp is ever injected and `schema-conformance.schema-version-current` — the M42 check
  that exists to catch an unmigrated corpus — **can never fire against it**. This is D10's second
  N26 question (*an unstamped managed corpus gets a store-surface answer*), and the answer is a code
  of its own rather than a re-use of D12's: the type is defined, so *orphaned* would be a law-1 lie.
- **Severity: advisory, report-only. It does NOT join `STORE_EXIT_FLIPS`** and flips no exit.
- **Route:** *add a manifest entry for the doctype, or state that the doctype is deliberately
  unfrozen* — the second exit being a real one, not a courtesy.

**The reason the two severities differ, quoted from the human's confirmation:**

> a manifest-less pack means "unchecked" by the manifest header's own stated design, and flipping
> would fail every manifest-less project pack's CI for a permitted choice

So D12's exit flip is **required by its own admission argument** (a green meaning *I stopped looking
at these files*, on the verb `MIGRATING.md` tells adopters to CI-gate on), while D10's would punish a
pack author for taking a choice the manifest design grants — the same asymmetry, read off the two
causes rather than assumed from their shared surface.

**Both codes are registered in [validation.md](../../../design/validation.md)'s inventory** (severity
class · intrinsic-or-tunable · keyed/unkeyed), per the *Scope confirmed* rule that no code is minted
homeless, and both take §10's mold in every other respect. Neither joins `ERROR_CODE_REGISTRY`: they
are `Finding`s, not door identities.

---

#### §19 — D15: the acceptance design is **adopted**, and the four HALT drives are folded in

*(Discharges *Owed at decompose* item 6 and the flow-52 half of
[planning gate-record](planning-gate-record.md) → HALT cell 1, at the **design** level; the drives
themselves run on the built and installed `1.0.0-rc.15`, after the audit's fixes, exactly as D15
rules. Source: [acceptance-design.md](acceptance-design.md), read and adopted in full.)*

**Adopted as decided, not as a draft:**

- **The nine flow-52 arms**, each naming the kind of set it iterates — D1's occurrence-keyed
  path-argument registry (1) · D2's total leaf classification × the posture family's defining
  case-set (2) · `setup`'s own install pathspec as a **derivation stated as one** (3) · D4's
  config-layer CAS pre-image as a **manufactured shape space that says so** (4) · D5's `EnvelopeArm`
  registry with its four proofs (5) · `WORK_UNIT_ID_DOORS` filtered to the unknown-id cell (6) ·
  `ManifestKind::ALL` and the fenced-count family as a **case-set matched exhaustively** (7) · the
  derived ambush owe-set (8) · the D12 orphan arm over `STORE_EXIT_FLIPS` (9). The **deliberately
  unrepresented** set is adopted with it, on the M46 Inc 9 / M48 Inc 11 / M49 Inc 12 precedent: D6's
  policy prose, §13's batched hash move, D11's version fence, D9's log key-set fence, EC-7's scraper
  repair, the Tier-2 and Tier-4 batches, and the close increment mint no verb, finding or route for a
  done-picture walk to reach, and manufacturing an arm for them would be a walk written to have an
  arm. **EC-28 rides arm 1 and N15 rides arm 5**, so neither is lost by not having its own.
- **The eight axis matrices' door-set derivations** — axis 1 from the D1 registry ∪ `DOCTYPE_DOORS ▸
  DoctypeArg::Address` ∪ `SLUG_DOORS` ∪ `WORK_UNIT_ID_DOORS`, all four ⇔-fenced from `ARG_TOKENS`;
  axis 2 from D2's commit-on-behalf and move-on-behalf classes; axis 3 from `DESTROYING_DOORS` ∪
  `task discard`; axis 4 from `COMMITTING_DOORS` ∪ `setup` ∪ `join` ∪ `provision`; axis 5 from the
  `EnvelopeArm` registry; axis 6 from `render::composed`'s four producers ∪ `describe` ∪ the
  read-back fence's owe-set; axis 7 from the freeze/migration surfaces; axis 8 from the Tier-2/Tier-4
  edit set plus the three generated help texts and the two `include_str!`'d guides. **A
  classification-only row confers no coverage** — otherwise the fence measures a table instead of the
  binary.
- **The driver/source-pass split** — one Opus driver owning the `(door, cell)` table (a row is driven
  iff its argv ran on the installed binary and its verdict carries a repro block; **it may not mark a
  row driven from a source read**) plus one Codex source pass owning **completeness of the row set**,
  with the per-axis reading list as written.
- **The reconciliation rule**, adopted verbatim as the wave's standing form: ***a claim by one that
  the other cannot reproduce is a lead, not a finding.*** A Codex claim with no driven repro enters
  the table as `lead(codex, <claim>)` and is either driven to a repro block — at which point it is a
  finding — or recorded refuted with its falsifying datum; an Opus row the source pass says cannot
  happen stays a finding, and the source claim is recorded refuted. **No fix ships on a source read
  alone, and no completeness claim ships on driving alone** (M46's planning corrections; M50's two
  planning halts).

**The coverage finding is recorded, and the fence is strengthened rather than restated.** Driven
against the design: **axis 5 alone satisfies §17's fence**, because its proofs (1) and (3) drive
*every* leaf to a success — so *"every leaf appears in ≥1 axis matrix"* is true and **weak**, and a
fence that cannot fail is the shape this wave exists to correct. **Amended: the coverage table
reports which axes beyond axis 5 reach each leaf, and a leaf reached by axis 5 only is marked
`only-5(<reason>)` under the same reason obligation `uncovered(<reason>)` carries.** Today that is
exactly three leaves, each with its reason on the record: **`task list`** (no path, id, posture or
destruction reaches it — its whole contract is its envelope, and it is the census's single
top-level-array anomaly), **`config get`** (a read of the cascade; its argument is a knob name from
the knob vocabulary, never a path — and the knob **value** rule is driven at `config set`), and
**`config list`** (takes no argument at all). `uncovered(<reason>)` is **empty by construction**, and
that is stated rather than presented as a result; a leaf added before the review runs inherits axis 5
automatically and must be given a reason here or an axis row.

**The four HALT drives are folded in** ([gate-halt-discharges.md](gate-halt-discharges.md), all run
against the release `1.0.0-rc.14` at `bd348a83`, each section naming its script and quoting the
output that matters):

- **EC-30's milestone arm is REFUTED, with its datum, and nothing is built for it.** `jigc milestone
  finalize` over a `git reset --soft`-ed base **refuses honestly** — `finalize.base-mismatch`, **exit
  3**, a located finding and two runnable recovery routes, zero commits — and control A proves the
  guard is not a reachability probe at all: a perfectly *reachable* base with one ordinary commit on
  top yields the same code at the same exit. Per the Settle's own rule the row is **recorded refuted
  and leaves Tier 3**. Two bounds ride it: `guard_base_live`'s stale-base finding was **not reached**
  (it needs the base *object* destroyed), and `milestone join` exits 0 over a divergent base with a
  success line that says nothing true about the state — a **surface-tier observation, recorded here,
  not promoted into EC-30**.
- **EC-7's residual is CORRECT as amended, and it survives its applied mutation.** The two inline
  `insta` goldens redden on an accidental drop; the one e2e test whose *stated job* is the deny floor
  **stays green**, because `floor_patterns()` `include_str!`s the very file that was mutated, so the
  assertion set shrinks with the deletion. Of three assertions across the two deny-floor-purposed
  tests, **two are derived and one is a literal** — and the two M50 human-owned destroying-door
  entries are asserted by the inline goldens **only**. The fix shape stays G-43's: extend the test
  asserting literals against the **loaded** profile; never mint a production const.
- **The three falsified reuse rows are now exercised** — D1's three-leg composition at the shipped
  callers that carry each leg (`config set` has `config.unusable-root`, `setup` has `canonicalize +
  strip_prefix`, `jigc migrate` has **neither** and takes all three shapes at exit 0), with the
  residual stated plainly: **the amended composition at the `migrate` door is the increment's red
  test** and cannot be driven before it exists · D3's predicate driven **as a git query**, where
  worktree-vs-HEAD says *dirty* on the discriminating cell and index-vs-HEAD says *clean*, which is
  the withdrawal's justification driven rather than reasoned, with the upgrade leg bounded to one
  binary · D4's three non-finalize `gitignore::ensure` callers driven, **confirming A1 and inverting
  the priority inside it**: `milestone provision`/`create` lose the line **visibly and recoverably**,
  while the two *committing* callers (`setup`, `task finalize`) lose it **silently and landed** — so
  the amend-to-union is a precondition, and the caller that justifies it is `setup`.
- **EC-14's *"three `DECISIONS.md` citations point at blank lines"* is struck and widened**: into the
  two posture homes the count is **12**, and repo-wide **62 of 261** line-anchored citations in
  `DECISIONS.md` land on a blank line (**23.8 %**) — the claim understates its own class by a factor
  of four in the homes it names and by twenty overall, *the same shape four consecutive completion
  audits have recorded*. One caveat is kept with it: a blank-line citation is a **stale or off-by-N
  pointer**, not proof of absent content; what it establishes is that these citations are fenced
  against nothing.
- **Codex 6's *"no seventh `Plain` argument id"* is CONFIRMED** by a per-row enumeration of all 25
  ids at HEAD — 11 booleans, 14 string-valued, **six** reaching a path component or a filesystem-op
  argument (the registry's members), and `workflow` named as the **near-miss** that closes on two
  facts (neither `PackSource::read` impl joins the id; the one `join`-based read is gated behind a
  directory listing of file stems). The bound stands and is the point of the totality fence rather
  than of the search: *an enumeration by a reader can miss; what makes a miss survivable is the ⇔
  fence forcing the question of every argument that ships.*

**What this does not discharge.** The eight matrices' rows are **not driven** and deliberately cannot
be: they run on `1.0.0-rc.15`, a binary that does not exist until after the audit's fixes land. HALT
cell 1 therefore closes **at the design level only**, and the gate-record says so in those words.

---

#### §20 — D10 × D12, at the build (2026-09-15): the `orphaned` row's `id` is **`null`**, and the derived owe-set unions a **declared-identifier** const

*(Decided by the human on 2026-09-15 at the Increment 8 plan halt — the build-planner halted rather
than resolving it — against an independent `robust-advocate` case. The planner's arm was the cheap
one on both forks and lost on both. No decision above is rewritten; this bracket is reached from D10,
D12, §9 and §18.)*

**Fork 1 — the row's identity. §9/§18's *"`id` is read from the stamp"* is driven false:** a committed
managed doc's whole front matter is `---\nschema-version: N\n---` (`dev/jigc-rig committed-singletons`:
`CHANGELOG.md` → `schema-version: 2`, `VISION.md` → `schema-version: 1`);
`engine::validate::schema_version_from_front_matter` (`validate.rs:1383`) returns `Option<u32>`;
`instance_slug` (`index.rs:823`) needs the `&Schema` in both branches; and no HEAD source holds a
type for an orphan (`file-state.json` is path→hash, finalize commits carry no doctype trailer). There
is nothing honest to report. **Decided: `id: null`.** An orphan has no address, so the row says so
structurally, `state: "orphaned"` being the discriminator; `item-count: null` stands. This is a
**reshape** of `DocRow.id` (string → string|null, on the wire since M42), admissible only while the
pre-1.0 window is open under the posture Increment 5 landed
([command-output-contract.md](../../../design/command-output-contract.md) → the third pre-pin case,
the act Increment 6 took at twenty-five doors) — and taken now precisely because after the pin
`id`'s **meaning** is what freezes: under the path arm it would read *address-or-path* for the life
of 1.x with no repair short of 2.0. Precedent: `jigc unmanage` pins `identity: Option<String>`
(`unmanage.rs:35`), null when no schema owns the path, and substitutes the path only into a finding's
`from` (`:67`), never the wire key — the same file adjudicated both choices. **Refused, with their
tells:** the repo-relative path (a meaning change on a pinned key, borrowing a finding-*target*
licence for an identity key; a driver keying on `row.id` builds an address from a path and is refused
only at `doc show`) · a synthesized `<x>:<stem>` (an address `doc show` refuses — and wrong in
derivation, since `instance_slug` yields the *type* as slug for a placement doctype, so `VISION.md`
is `vision:vision`, not `?:VISION`) · dropping the row (reversible, but it re-opens `doc list`'s
founding *"an omitted file sends the agent to `cat`"* argument on the very file `validate` now
exit-flips on). **`design/doc-read-surface.md:160` is corrected in Increment 8 with its falsifying
datum**, not reworded, and the `DocRow.id` doc-comment states the null case.

**Fork 2 — `finalize.left-out` under the derivation.** §4's source set (*blocking codes minted by a
commit-on-behalf door, minus `Exempt(reason)` rows*) cannot produce `finalize.left-out`: a current
`AMBUSH_CLASS_CODES` member (`pack.rs:779`) that **no producer mints** (`render::FINALIZE_NON_MEMBERS`,
M42's print-over-refuse), declared by both packs' finalize steps and buying three tokens under
`CONSTRAINT_REQUIRED_TOKENS` (`pack.rs:1242`) — so a literal derivation silently retires a shipped
fence member. **Decided: the derivation stays pure, and the owe-set is
`derive(commit-on-behalf blocking codes) − Exempt(reason) ∪ DECLARED_CONTRACT_IDENTIFIERS`**, the
second operand a one-row const beside the derivation, each row carrying its non-mint reason,
⇔-bijected against `FINALIZE_NON_MEMBERS` (which already names the row) and
`CONSTRAINT_REQUIRED_TOKENS`. **Refused:** a hand-stated non-mint row *inside* the derived set —
that is the hand-list §4 was written to kill, re-admitted behind a derivation-shaped wrapper, and the
next print-over-refuse contract would join by hand on its precedent.

---

## Scope confirmed

*[Amended 2026-09-11: see [Review amendments](#review-amendments-2026-09-11) §10 — `jigc ingest`'s identity refusal takes its code and the family's mold — and §12, which gives **EC-28** and **N15** the dispositions neither had anywhere.]*

By the human's earlier boundary and **confirmed at this Settle**: **every baseline-amendment cell
joins its tier**. Recorded here so no cell is admitted by silence.

- **The sub-task `resume:` line's three real states** — pre-provision refusal · post-provision
  compose-without-provisioning · `base == HEAD` silent compose — all in, as **one law-1/advisory item
  on the render layer** (`render::task_state_lines` + `start.rs`'s guard predicate). **Not a pack
  change, not a schema change**; no pack-load fence has to learn the provisioning fact. *(G-50)*
- **`jigc task discard`'s silent commit ack joins** — the tenth committing door, whose success line
  names no sha where every sibling prints one. *(G-51)*
- **`jigc ingest`'s identity leg joins**, with a **`Human` route naming `git mv`** — `doc rename`
  requires the doc be managed and this one by definition is not; M45's owner-artifact `git add` route
  is the precedent. *(G-48)*
- **The three EC-11 help texts:** `migrate-corpus` generated from **`SchemaChangeKind::ALL`**;
  `task finalize --help` from **`whats_left_coverage()`**, which already generates that sentence; and
  `validate --help` / `doc show --help` need a **minted probe-family registry** and a **load-bearing
  key const** respectively — **both minted**, because an unfenced second home for one fact is the
  defect, not the fix. *(G-52)*
- **EC-12 is a sub-task QUALIFIER, not a flip**, sourced from `team-ready-state.md:102`/`:106` — a
  build that rewrites the guides to *"`task discard` commits"* would ship a **new** falsehood for the
  case the guides' own narrative is about. *(G-25)*
- **EC-17 over `{shape} × {on-disk walk, registered set}`**, with **`doomed_at` collapsed onto
  `probe_leftover`** and **`finalize.md:204` binding the teardown**. The fourth `DESTROYING_DOORS`
  cell cannot exist. *(G-23)*
- **EC-15's overlay is derived from `provenance.json` per sub-task**, with **`overlay` unchanged on a
  block** — so the fix does not silently change one key while fixing another. `no_docs_from`'s
  **shape** does not move; only its values become true. *(G-27)*
- **F-9 is recorded as a fired trigger**, not a fresh scope call — its recorded trigger names *the
  next wave that opens `doc rename`'s ack or `finalize`'s pre-commit manifest*, and this wave opens
  both. *(G-59)*
- **Every new finding code is registered in `validation.md`'s inventory** (severity class ·
  intrinsic-or-tunable · keyed/unkeyed) **and, where it is a door identity, in
  `surface-contract.md`'s namespace**. At least seven codes are at risk of being minted homeless.
  *(G-20)*
- **Zero schema-hash movement, zero `schema-version` bumps, zero corpus migrations in this wave** —
  **stated as the wave's honest bound.** The freeze covers **17 manifest entries / 16 doctypes across
  both packs**, and `optional:`/`set:`/`default:`/`of:`/`check:`/`title-names-symbol` are all *inside*
  the hash, so a Tier-2 "reword" that flips `optional:` is a version-gated change in a prose costume.
  A `milestone-record` posture field is refused and unnecessary: **a posture probe is a refusal, not a
  record.** *(G-22)*
- **A consent flag is admissible and is owed the robust-advocate treatment** — the razor's refusal
  class names verbs, schemas and execution shapes, **not flags**, and both robust arms in scope add
  one. Stated once, here, so it is not re-argued per row. *(G-58)*
- **The *"Notation is illustrative"* header disclaims notation only** — every rule this wave cites is
  prose and citable. Stated once so leg 1 is not argued per row. *(G-60)*
- **The adopter-doc edit set is one batch and one hash move** — both guides are `include_str!`'d into
  the installed `SKILL.md`, every byte moves `jigc-body-blake3`, and relative links are flattened, so
  every added paragraph is written **self-contained**. *(G-55, G-61, G-64)*
- **The three stale internal docs ride the Tier-2 batch** (`doctype-authoring.md`'s pre-rc.6 caveat ·
  `pinning.md:15`'s falsified prerequisite · `dev-workflow.md`'s *"all four"* over five bullets).
  *(G-63)*
- **EC-30's milestone arm is driven first.** If it refutes, the row is recorded **refuted with its
  datum** and nothing is built. *(unchanged from the charter)*
- **EC-7 survives only after one applied mutation**, and if it does, the fix shape is the **shipped**
  one — extend the test asserting literals against the **loaded** profile, never mint a production
  const that becomes a second source of truth. The scraper repair itself is D1's. *(G-43)*
- **The close increment inherits two mechanical obligations, named now rather than discovered at the
  gate:** `foldback_truth.rs:242` **re-aimed to `**M51 —` and inverted twice**, and the golden
  regeneration (`describe--*` / `start-orient*` / `workflow-preview--*` and the 10 version-bearing
  goldens). *(G-62, G-69)*

---

## Owed at decompose

*[Re-cut 2026-09-11 at the review — see [Review amendments](#review-amendments-2026-09-11). Four items were **decided** by the amendments and leave this list: D1's sink validation point (§2 — immediately before the unlink, in the same function, behind a typed `ValidatedRetirement`), D5's registry home and key table (§7 — hosted at `format_json_success_axis.rs`, production-side, the census's dispositions adopted), D2's membership rule (§3 — a total classification of every clap leaf), and the G-5 re-drive (§2 — driven, and discharged by the trackedness leg). Three joined it: the D10/D12 partition, the `EnvelopeArm` hand-enumerated rows' reasons, and flow 52's arm set.]*

*[**Re-cut again 2026-09-11, after the bake-in** — §18 and §19. **Two of those three leave again**: the D10/D12 partition is **decided** (§18 — two codes split on whether the doctype resolves, with their severities, routes, `doc list` answers and the quoted reason for the exit-flip asymmetry), and flow 52's arm set is **decided** (§19 — the nine arms, plus the deliberately-unrepresented set). Nothing joins. **Three shapes remain**, plus the smaller list below, whose EC-30 and EC-7 rows are discharged by [gate-halt-discharges.md](gate-halt-discharges.md).]*

The exact shapes this Settle deliberately left to the plan. Each is a **shape**, not a re-decision —
the decision is above; what is owed is where it lands.

1. **D2 — the posture registry's identifier and home.** Its **membership rule is decided** (§3: a
   total classification of every clap leaf into commit-on-behalf / move-on-behalf / neither); what is
   owed is the identifier, the file it lives in, and the exact form of its
   `COMMITTING_DOORS ⊆ commit-on-behalf` assertion and its clap-tree ⇔ fence.
2. **D3 — `CarryoverBoundary::Setup`'s code spelling and route text.** The **mold is decided**
   (§10: a blocking `Finding`, a `Human` route, exit 1, registered in `validation.md`'s inventory,
   outside `ERROR_CODE_REGISTRY`); what is owed is the code's spelling and the route's wording, which
   must name `--force` as the single consent without training reflex.
3. **D5 — the `EnvelopeArm` hand-enumerated rows' reasons.** The **11 arms no result enum can
   generate** (§7 — `jigc task finalize`'s four among them) each need the stated reason that stands
   in for a derivation, in the form the registry requires.
4. **D6 — the policy text**, now including **the pre-pin removal rule** (§7/A7) that authorizes D5's
   four deletes. The internal paragraph's wording in the widened *Evolution posture* section, and the
   compact adopter-facing sentence in the guide body — self-contained, and batched into the single
   hash move.

**Also owed, smaller:** the `claim-driven` gate cell written as **relayed** for the four source-read
rows that remain relayed (**EC-6**, **EC-8**, **EC-9**, **EC-24**) — none of which gates a scope
decision, which is what the cell already permits — and the Tier-4 strike sites' **line cites
re-derived before editing**, three of the charter's own being wrong (G-66).

**Discharged from this list by driving, 2026-09-11** ([gate-halt-discharges.md](gate-halt-discharges.md)):
**EC-30's milestone arm** — driven and **REFUTED with its datum**, so the row leaves Tier 3 and
nothing is built · **EC-7** — the applied mutation run, the residual **correct as amended** · and the
two further relays, **EC-14's citation count** (struck and widened to 12 into the posture homes, 62
of 261 repo-wide) and **Codex 6's *"no seventh `Plain` argument id"*** (confirmed by a per-row
enumeration of all 25 ids at HEAD).
