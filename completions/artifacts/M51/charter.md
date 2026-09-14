# M51 — the evidence-check wave (the unadjudicated-axis wave) · CHARTER

**Preparation only.** Scope, claim, razor, forks and exclusions are here. **The decomposition is
not** — that is the planning session's Settle and plan, against a baseline that exercises the real
binary rather than this document. Written 2026-09-10 by the session that orchestrated the evidence
check, which is exactly the position that produces confident wrong premises; **every row below is a
lead until the baseline drives it**, and the check's own closing rule binds this charter first: *an
agent's report is a lead, not a measurement* ([VERDICT](../evidence-check-1.0/VERDICT.md) → Honest
bounds). Twenty-two of the forty-three rows are marked `reproduced-live? yes`; the rest are
source- or repo-reads and are named as such in the ledger — **drive them before fixing them.**

**Chartered on** [the 1.0.0 evidence check](../evidence-check-1.0/VERDICT.md), run 2026-09-10 on
**`1.0.0-rc.14`** built from HEAD **`bd348a83`** — five Opus reviewers plus one independent,
unseeded Codex source review, every driving reviewer on the release binary, **no reviewer running
cargo and none editing a repo file**. The gate passed at that HEAD: **3341 passed / 0 failed**
([gate-rc14-at-bd348a83.log](../evidence-check-1.0/gate-rc14-at-bd348a83.log)). Its verdict is two
independent conclusions:

- **Not 1.0-ready as rc.14 stands** — **two data-loss-at-exit-0 defects that no trial reached, both
  driven live here, and both outside every M50 registry**: a path-taking argument that no registry
  classifies (EC-1) and a git posture that no registry has a cell for (EC-2).
- **The rc.14 trial's evidence holds and is *stronger* than its record states** — its single
  strongest stated caution against the 3/3 headline (B3-strict's *"first move"*) is **false as
  written**; corrected, B3-strict is a **fourth** VERB-first arm under the stricter posture (EC-31).

The human's criterion for the pre-1.0 waves, verbatim from
[the handover that preceded the RC-m50 trial](../RC-m50/handover.md) and reaffirmed here:

> everything that improves usability, routing, bug-fixes — everything that makes the product better
> and more acceptable at v1, and above all **everything that becomes impossible or expensive to
> change once people start using the product. Now it's cheap.**

**The boundary is decided, not open (the human, 2026-09-10).** This wave takes **every row of the
evidence-check ledger (EC-1 … EC-43)**, plus the **fix-shaped** rc.14 trial rows **F-5, F-9, F-11**
and the carried defects **N15, N20, N23**. It **leaves post-1.0** the four **capability additions**
— **F-3** (a schema-*format* change), **F-8** (a frozen `adr` schema bump + a shipped adopter
migration), **F-10** (a new amend verb), **F-13** (a second milestone execution shape) — on the
criterion above: **capability additions are additive after 1.0; surface and contract fixes are
not.** EC-36 is the pricing that makes the line checkable — cost is per *mechanism* (text · additive
key · envelope shape · schema-version bump + migration · schema-format change), never per *contract
touched/not touched*.

---

## The claim

> **No caller-supplied token and no repository posture reaches a door that destroys, commits or
> moves without that door having adjudicated it — the two exemptions the M50 registries carried
> (`ArgToken::Plain`'s path family; HEAD posture at `COMMITTING_DOORS`) are closed as classes — and
> every surface 1.0.0 pins says what the binary does, so that the pin closes over declared keys,
> true counts and a stated evolution rule.**

Two halves, both falsifiable.

**The first half is falsified** if the baseline drives either exemption and finds it is not one
class but a list. M50's own audit already recorded the shape: `ArgToken::Plain` *"catches
forgetting, never mis-answering"* ([DECISIONS.md](../../../DECISIONS.md) → 2026-09-09), and EC-1 is
mis-answering. If the five exempted argument names cannot share one predicate — because a `--from`
that enumerates git's committed set and a `--path` that takes an arbitrary host path are different
questions — then the wave ships a **named list with a fence over the list**, and says so rather than
calling it a class. Symmetrically for EC-2: if a `git symbolic-ref -q HEAD` probe cannot distinguish
the fan-out worktree (detached **by design**) from a user's detached HEAD, the exemption is not
declarable at the shared seam and the half shrinks to the doors that can carry it. And if either
blocker fails to reproduce on the baseline's own binary, it leaves the wave with its falsifying
datum quoted — the repo's rule for a corrected row.

**The second half is falsified** by any row where the binary turns out right and the check wrong: a
key EC-3 calls undeclared that is declared, a count EC-14 calls stale that is current, a fence EC-6
or EC-7 grades partial that a driven mutant proves closed. Those rows are **source-reads**, and the
check itself flags them as leads. It is falsified more sharply if a pinned surface cannot state its
post-1.0 evolution rule (EC-8) without minting a new contract version to say it — in which case the
pin does not close cheaply, and the wave has to say what it costs instead of claiming it is free.

## Scope

Every item carries its ledger id and the **axis** the ledger already derived. The axis column is
**copied, not re-derived** — re-deriving it in a charter is how the paraphrase drift this file's own
preamble warns about enters ([decisions-pending.md](../../../implementation/decisions-pending.md) →
*A charter fork that engages a record quotes the record's scope sentence verbatim*).

### Baseline amendments (2026-09-10)

**The baseline drove this charter's rows** — four Opus capability-auditors on the release
`1.0.0-rc.14` at HEAD `bd348a83`, none running cargo, none editing a repo file:
[baseline-ledger.md](baseline-ledger.md) (consolidated) with its four companions. Every cell below
is *added* to the tier named; nothing already in the tiers is removed. **Fifteen cells**, each
tracing to a companion line.

**Tier 0 gains** — five, all driven:

1. **The `source-path` sink re-validation — now measured, not argued.** A benign in-repo mint,
   `.jigc/tasks/<id>/source-path` overwritten in the mutable working area, then
   `task finalize --approve` → **exit 0**, the ack naming the benign source, and an arbitrary host
   file deleted. `retire_exempt` (`task.rs:1426`/`:1794`) is a **second** raw consumer of the same
   token, and `trackable::untrackable_reason` — which already refuses every destructive cell — is
   **never called by `migrate.rs`**.
2. **`jigc config insert-step … <file>` / `config replace-step <target> <file>` — a read escape.**
   An absolute outside path, a `../` escape and `.git/config` are accepted at **exit 0**, copied
   into the committable `.jigc/config/steps/<stem>.yaml`, and then **composed into the step text
   `jigc start` hands the agent** (driven: git's own `repositoryformatversion = 0` rendered into a
   workflow). **Classified Tier 0**, on this ground: it is the same `ArgToken::Plain` member family
   and the same missing home rule at the same seam as EC-1 — one predicate closes both — and while
   it is *a read, not a loss*, it lands arbitrary host bytes inside the repo as committable files
   and inside the agent's composed context, which is the boundary the adapter deny-floor exists to
   hold. The honest counter, for the razor: nothing is destroyed and every effect is recoverable, so
   if the Settle prices it as surface rather than escape it belongs in Tier 1 — but not in a
   different *fix*.
3. **`jigc migrate .git/*` sources.** `.git/config` → `--approve` **exit 0, deleted**; `.git/HEAD` →
   exit 1 with HEAD deleted mid-transaction and rollback leaving **the index carrying staged
   deletions of `.jigc/.gitignore`, `.jigc/config/*`, `.jigc/version` while the files are untracked
   on disk** (reproduced twice).
4. **The `GIT_DIR` redirect.** `GIT_DIR=<other repo>/.git jigc milestone create` → **exit 0**, the
   record written into repo A and **the commit landed in repo B**. No git-env scrubbing exists
   (`env_remove`/`env_clear`: no production hit).
5. **The unborn-HEAD milestone.** `milestone create` on an orphan branch exits 0 and writes git's
   **empty-tree hash** into the *committed* record; `provision` then fails
   `milestone.provision-failed` and `milestone finalize` fails with a **bare `anyhow`** —
   `error_code: null`, no code, route or state clause. Only `discard --force` exits.

**Tier 1 gains** — ten:

6. **The pinned-envelope *list* itself (EC-3 as a class).** All 47 leaf verbs speak `--format json`;
   beyond EC-3's four keys, **13 further envelope keys are named in no design document**
   (`uninstalled`, `rows`, `summary`, `identity`, `new_path`, `old_path`, `prose_mentions`,
   `referrers`, `moved`, `displaced`, `reslugged`, `layer`, `rejected`, `knobs`, `anchor`, `side`,
   `step`, `overlay`). **Nothing enumerates which envelopes are pinned at all** — the contract doc
   names three surfaces, `text_json_parity_axis::REGISTRY` classifies *fence-ability*, never
   *pinned-ness*. **Fork 7 needs this list before declare-or-delete can range over anything.**
7. **`milestone execute` is a fourth composed producer** of the pinned `{task, text}` shape, against
   the contract's *"three verbs"* — and the code-side census already says so while the doc does not.
8. **`task list --format json` is a top-level array**, so it can carry no `schema_version`, no
   `findings` and no discriminator; stated in no design doc.
9. **`milestone list-tasks` ships `hook_output`** on a `VerbKind::Read` verb, structurally always
   `""`, outside that key's declared scope.
10. **The user's linked-worktree split.** Driven from `git worktree add -b feature`: `task finalize`
    commits onto `feature` (correct) while `milestone create`/`add-task` write into the **main
    checkout** and commit onto **`main`**. **A posture cell in no EC row** — fork 2 must say what a
    committing door's *subject repo* is, not only whether HEAD is attached.
11. **The survivable frame's cause vocabulary — N20 widens to every non-`CommitRejected` boundary
    error.** Driven on a `git merge --ff-only` refusal (no hook involved): exit 1, bare `anyhow`,
    `error_code: null`, record left `active`; the unborn-HEAD `milestone finalize` is a second
    instance. Telling that reader to *fix the hook* is a law-1 lie about the cause.
12. **`jigc task discard`'s ack is silent about its own commit** — the tenth committing door, driven
    (`HEAD 520283b → 1152124`), and its success line names no sha where every sibling prints one.
13. **The sub-task workflow's own `resume:` line does not provision its commit doc** — **driven end
    to end, two arms in one corpus**: `jigc start --task alpha-task` (the footer's own resume line)
    exits 0 with **no `.jigc/tasks/alpha-task/docs/` at all**, so `doc set-field
    commit:alpha-task#type` is refused *"no staged instance … task alpha-task's workflow provisions
    its `commit` doc at compose and grants no in-task create for it"* and its route
    `jigc doc list --task alpha-task` prints *"no docs staged"*; while `jigc workflow sub-task
    --task beta-task` — the line `milestone execute` prints as `Spawn:` — exits 0 with
    `commit:beta-task.md` + `provenance.json` staged. A resumed sub-agent gets **a workflow it
    cannot author** and a refusal whose route **never names the provisioning verb**.
14. **`jigc ingest`'s self-contradiction, with a looping route** — `ingest` claims adoption at exit
    0 and persists it, `doc list` says `unregistered`, `doc show` refuses `store.malformed-slug`,
    and `validate` exits 1 *"never adopted"* **routed at `jigc ingest`**: a route that, followed
    exactly, changes nothing. The carried entry (`decisions-pending.md:593`) names only the
    registration.
15. **`task finalize --carry-staged` concludes a merge** — exit 0, a **two-parent** commit under
    jigc's own subject, `MERGE_HEAD` consumed, the ack saying `1 file committed` and never
    mentioning the merge. The flag's documented consent is *carry my staged work*.

**Rows corrected by the baseline** (each with its datum in the ledger):

- **EC-28's `doc create` claim is corrected — refuted as written.** `doc create --slug <300>` does
  **not** refuse at the create-gate: it fails on the same OS ceiling as `doc rename` and calls it
  `task.working-area-io` with a route blaming *"a disk or permissions problem"* — a **false route**.
  It is a 3-door class (`doc rename`, `start`, `doc create`); `doc add-item --slug <300>` exits 0
  and the `migrate --slug` cell is masked, not clear.
- **EC-14's *"eleven"* is corrected — it is current, not stale.** `surface-contract.md:129` matches
  `ERROR_CODE_REGISTRY`'s 11 members; the defect there is carried defect **(h)** — the mirror is
  **unfenced** while the registry test hardcodes `11` and names that file in its assert. (Same pass:
  the *"window closes here"* pair is presentation, not a lie; CLAUDE.md's *"thirteen `CELLS` rows"*
  is false and **was false when written** — 63 rows at that very commit.)
- **EC-7 is narrowed.** The `include_str!` self-reference in `e2e_audit.rs` is real, but
  `adapter.rs` carries the whole floor **twice as inline `insta` literals** (22 patterns) and both
  redden on a deletion. The residual gap is **deliberate regeneration** (`cargo insta accept`), not
  an accidental drop — the compose-goldens posture. (The charter's *"20 patterns"* is 22.)
- **EC-20 is narrowed to its real subject.** `task validate` and the **landed** `task finalize`
  emit **identical** findings; the divergence is `finalize --dry-run`, which has **no `findings` key
  at all** and drops **both** advisories. `GATE_COVERAGE`'s fence is a token guard over prose and by
  construction cannot compare what two doors *emit*.
- **EC-17's axis loses a cell and gains a dimension.** The fourth `DESTROYING_DOORS` cell **cannot
  exist** (`remove_worktrees` filters on the *registered* set), and the doors run **two subject
  derivations** — on-disk walk vs registered set — which is why two doors narrate a destruction they
  do not perform and a third is silent over the identical state.

### Tier 0 — blocks the 1.0.0 call

Both driven live; both exit-0 byte loss; both outside every M50 registry.

- **EC-1 — the path-taking argument family.** `jigc migrate <absolute-path-outside-the-repo> --as
  <doctype>` is accepted at exit 0 and `jigc task finalize --approve` **permanently deletes the
  external file**; the `../` and symlink spellings read-escape without deleting; `.git/config` is
  accepted as a source. *Axis:* `ArgToken::Plain`'s exemption set `path`, `from`, `file`,
  `from_file`, `target` (`cli.rs:1500`) × {read, record, retire/delete} × {absolute, `../` escape,
  symlink escape, `.git/` component} — **every exempted argument's door must adjudicate what the
  exemption assumes it adjudicates, and the recorded `source-path` must be re-validated at the
  destructive sink (`task.rs::retire`), because the working area is mutable.** Fork 1.
- **EC-2 — HEAD posture at the committing doors.** No committing door asks whether HEAD is on a
  branch; `task finalize` lands prose on a branchless commit and destroys the only other copy in the
  same act, `milestone create` bricks the record, `setup` silently un-installs on the next checkout,
  and `validate` then reports a **phantom** out-of-band edit. `grep -rn "symbolic-ref\|symbolic_ref\|rev-parse --abbrev-ref"`
  over both crates returns **zero** hits. *Axis:* **HEAD posture × the ten `COMMITTING_DOORS`** — one
  probe at the shared seam, with a declared exemption (or a `--force`-shaped consent) for the
  fan-out worktree path, which is detached by design. **The registry exists; the cell does not.**
  Fork 2.

### Tier 1 — cheap now, expensive after the pin (the contract tier)

- **EC-3** four undeclared keys on pinned envelopes (`guide` on `upgrade`; `guide_file`,
  `install_commit`, `hook_committed` on `setup`), where `command-output-contract.md:439` counts four
  facts and the binary emits eight. *Axis:* every `--format json` envelope × {declared,
  fenced-closed} — a closed-key-set assertion per envelope plus a doc paragraph per key. Fork 7.
- **EC-4** `config set docs-root` **`git mv`s the committed corpus** and the pinned ack says nothing
  about it. *Axis:* acks whose side effects are not fields of the acked value — a fence over the
  *door's* effects, not over the enum.
- **EC-5** the result envelope's `schema_version` is documented as being on **every** envelope and
  rides an unstated subset; `task diff` is the sharp case. *Axis:* the versioned/unversioned envelope
  split — state the rule or close it, then fence the partition against the clap leaf tree. Fork 7.
- **EC-6** the invocation-log record shape is partially fenced (presence/type for three keys, no
  key-set equality anywhere) on a surface `design/measurement.md` calls independently versioned and
  which carries no version integer. *Axis:* the log record's key set — one closed-set assertion, and
  a decision on whether the log gets a version integer or is declared unversioned.
- **EC-7** the adapter deny floor's 20 secrets patterns are fenced by the file they check
  (`include_str!`-scraped), so deleting a pattern keeps the test green. *Axis:* self-referential
  fences — every test whose expectation is derived from its own subject; a security floor needs an
  independent literal set. **Narrowed by the baseline:** `adapter.rs` carries the floor twice as
  inline `insta` literals (22 patterns, not 20) and both redden on a deletion — the residual gap is
  *deliberate regeneration*, not an accidental drop ([baseline-ledger.md](baseline-ledger.md) → §3).
- **EC-8** no document states a release-versioning policy for the binary, and the probe wire's
  versioning is explicitly deferred. *Axis:* every pinned surface × its post-1.0 evolution rule, in a
  home an adopter reads. Fork 6.
- **EC-9** no root `README.md`, no root `CHANGELOG.md`, `publish = false` undecided; the recorded
  publishing-floor deferral's trigger — *"first external adopter, public release, or opening the
  repo"* — **fires by definition** when 1.0.0 ships. *Axis:* the publishing floor as a checklist,
  each item named and owned. Fork 6.
- **EC-10** the version-stamp confirmation is prose, not a fence — which is why the owed bump has
  been caught unshipped **five consecutive waves**. *Axis:* owed-at-close acts that are prose (the
  bump, the golden regeneration, the fold-back) — a step that mints them or a fence that reddens on
  their absence.
- **F-5** — the unknown-id column: **22 doors** refuse correctly, with a route, **and no code**;
  M50's empty-id column is 53/53 green beside it. A driver keying on the stable `(code, target)`
  pair gets nothing at 22 doors. Fork 5.
- **F-11** — the `doc author` **payload parse** error carries no severity, no code, no `at:`, no
  route and no footer, while the same door answers a declared-address miss with `blocking ·
  write.unknown-field` + route; M50 Increment 9 closed the write-miss floor at the four target
  resolvers and the payload parse sits **upstream of every one of them**. Fork 5 prices the envelope
  half separately — EC-36 records that F-11 is *two* repairs the trial record does not distinguish.
- **N15** — a `--task` read of an address that resolves nowhere is byte-identical to the task-less
  one, says *committed*, and its route **drops `--task`**, silently serving the committed copy to a
  reader holding a staged one. On a **1.0-pinned** read surface; its sibling `store.not-staged` is
  already right.
- **N20** — the milestone boundary's non-hook refusal **discards** a fully-built `RejectionFrame`:
  no code, no route, no state clause, `error_code: null`, no identity in the invocation log.
  **EC-37 corrects its recorded scope: it is knob-independent** — byte-identical under `squash:
  false`, the setting a Fix round runs under.
- **N23** — retiring a doctype silently unmanages its committed, tracked corpus: `validate` prints
  *"validates clean"* at exit 0 while `doc list` drops the rows entirely. **EC-38 re-drove it and
  confirms the classification** — registration loss, not byte loss — and names the consequence that
  makes it Tier 1: a **false green on the verb MIGRATING tells adopters to CI-gate on**.

### Tier 2 — the law-1 surface batch

Every row is a surface that says something the binary does not do. Two of them (EC-16, EC-17) are
**narration lies at destroying and relocating doors**, which is why they sit at the top of the batch
rather than in a wording tail.

- **EC-11** two `--help` texts false, one understating (`validate`'s *"code anchors"*;
  `migrate-corpus`'s *"v0→v1 transform"*; `doc show`'s four-key shape against six emitted). *Axis:*
  verb help × the behaviour it describes, checked against the code-side set it names.
- **EC-12** the adopter-facing back-out ladder says `jigc task discard` makes no commit — it is the
  **tenth** committing door. *Axis:* the "nine doors" count wherever it is written, and the class
  beyond the count: every adopter-facing sentence that says a verb does not commit.
- **EC-13** the 1.0 read contract's own declared conformance witness states a shape the binary no
  longer emits (four keys). *Axis:* every doc that enumerates a JSON shape by hand — generated from,
  or fenced against, the shape it witnesses (`doctype_map_versions.rs` is the shipped mechanism).
- **EC-14** the stale-count family (eleven manifest entries counted as ten; *"all 16 doctype
  hashes"*; five regimes over six rows; the nine-door and thirteen-cell counts; three `DECISIONS.md`
  citations pointing at blank lines; both posture homes headed *"the window closes here"* above two
  later declared spends). *Axis:* hand-written numbers over sets the code can move. **Corrected by
  the baseline:** *"eleven"* (`surface-contract.md:129`) is **current, not stale** — the defect
  there is that the mirror is unfenced — and the posture-home pair is presentation, not a lie
  ([baseline-ledger.md](baseline-ledger.md) → §3).
- **EC-15** `milestone join`'s `no docs staged from:` line and its `no_docs_from` key are **false**
  on a `join.same-doc-clash`; the sibling blocking class computes the overlay truthfully. *Axis:* the
  blocking-join classes × the overlay — one derivation, populated on every blocking path or
  explicitly declared absent.
- **EC-16** the `docs-root` relocation ack over-claims its subject (*"every committed doc under the
  prior resolved root, managed or not"* — one of three moved, correctly). *Axis:* acks that describe
  their subject as a universal, checked against the set the code walks — here the placement branch
  `CLAUDE.md` already names as the cross-cutting gotcha.
- **EC-17** two destroying doors **narrate a destruction they do not perform** and name three of the
  repo's own committed docs as unrecoverable; the third door prints nothing at all over the identical
  state. *Axis:* `LeftoverShape` × {regular file, directory, symlink-to-file, symlink-to-dir,
  symlink-into-the-repo} at all four `DESTROYING_DOORS`.
- **EC-18** `gitignore::ensure` **replaces** `.jigc/.gitignore` with the canonical `ENTRIES` set and
  finalize commits the result; its own doc-comment says *"amended once to the union"*. *Axis:*
  jigc-owned files a user may have edited — amend-or-refuse, never replace; `SKILL.md` already has
  the right shape.
- **EC-19 / EC-20 / EC-21** QUICKSTART Q1 (`--dry-run` *"changes nothing"* is true only on a clean
  task — driven on a fresh one it prints two blocking findings, no manifest, exit 3), Q2 (`task
  validate` and `finalize --dry-run` do **not** preview the same set, against the contract's own
  *"same check, same severity"*), Q3 (three install paths across two documents for step zero).
  *Axes:* every guide sentence walked in the state its own narrative puts the reader in · the
  previewed set × the gated set at `GATE_COVERAGE`'s advisory tier · one fact, one home.
  **EC-20 narrowed by the baseline:** `task validate` and the **landed** `finalize` emit identical
  findings — the divergence is `--dry-run`, which carries **no `findings` key at all**
  ([baseline-ledger.md](baseline-ledger.md) → §3).
- **EC-22 / EC-23 / EC-24** MIGRATING's four-member manifest vocabulary against a fifth
  (`ManifestKind::Added`) · the three-key `migrate-corpus` triage list against six, where
  `unadopted`'s emptiness **is the exit rule** · the ahead-stamp direction never mentioned in the
  guide the adopter is handed. *Axes:* enumerations of a code-side enum in prose · the adopter guides
  against the pinned envelopes (**MIGRATING/QUICKSTART are the SKILL.md body by construction**) ·
  `STORE_EXIT_FLIPS` × the adopter guide.
- **EC-25** `doc retitle-item` mis-quotes a traversal address, dropping one component from the echo.
  *Axis:* the echoed token vs the parsed token at every address-refusing door.
- **F-9** — `jigc doc rename` re-slugs a doc and says nothing about the staged commit doc whose
  summary still names the **old** title; `--dry-run` prints the stale subject and the new path on
  adjacent lines without connecting them. It is **the cheapest link to cut** in the trial's only
  adapter-bypass chain and **not** a determinism-boundary violation — *noticing* is not *rewriting*.

### Tier 3 — the capability cells the check found

- **EC-26** `jigc setup` **commits the user's uncommitted work at exit 0**, with no carryover gate
  and no mention; the swallow is confined to setup's own pathspec, which is exactly where a user's
  edits live. *Axis:* the carryover gate × `COMMITTING_DOORS` — the gate's subject is a **door**, not
  a task. Note this is the same registry as EC-2. Fork 4.
- **EC-27** a **whitespace-only `docs-root`** is accepted at exit 0, an ADR finalizes under a
  directory literally named three spaces, and `config get` reads back a value **visually identical to
  unset**. *Axis:* the root-knob value rule × the whitespace class — the predicate is *"a value that
  reads back as itself"*, not *"no control characters"*.
- **EC-28** `doc rename --slug <300 chars>` fails with a bare `File name too long (os error 63)` — no
  code, no route, no `at:`; state intact. *Axis:* `SLUG_DOORS` × the OS-error class. **Corrected by
  the baseline:** the row's *"`doc create --slug` refuses correctly at the create-gate"* is
  **refuted** — it fails on the same ceiling and calls it `task.working-area-io` with a false
  disk/permissions route; the class is 3 doors ([baseline-ledger.md](baseline-ledger.md) → §3).
- **EC-29** a hook-rejected `task finalize` leaves `.jigc/version` rewritten in the worktree although
  no commit landed — **design-declared** (`design/finalize.md`'s config-layer row says *"worktree
  untouched"* on purpose) while the header two rows above reads *"Before phase 6, all-or-nothing"*.
  *Axis:* rollback fidelity, index vs worktree, over {promotions, retirements, owner-artifact,
  config-layer, milestone-record} × {worktree, index} — the four index families are covered; the
  worktree is the un-swept dimension. Fork 3.
- **EC-30** the residual, **unverified** row: a `git reset --soft` + re-commit makes a task's
  `base.json` pin unreachable. **Driven on the single-task path the hypothesis was refuted** — the
  store does not lie, it does not care. The **milestone** arm reads `base.sha` at the join under a
  `base == HEAD` guard and **was not driven**. *Drive the milestone arm first; if it refutes, the row
  is recorded as refuted with its datum and nothing is built.*

### Tier 4 — record corrections, each with its falsifying datum quoted

**EC-31 … EC-43.** Edits to [the rc.14 trial record](../RC-rc14/trial-record.md),
[findings-verification.md](../RC-rc14/findings-verification.md),
[coverage.md](../RC-rc14/coverage.md) and
[decisions-pending.md](../../../implementation/decisions-pending.md). The repo's rule binds every
one: **a stale claim is struck with the datum that falsifies it, never silently rewritten**
([DECISIONS.md](../../../DECISIONS.md) → the M46 record-truth increment; M49's *"each with the
falsifying datum quoted"*).

- **EC-31** B3-strict's *"first move"* is backwards — its first read of the plant was `jigc doc show
  … --task` at **invocation 3, tool call 3, 9 s in**; the `find | xargs cat` was **tool call 7** and
  was **denied**. It is a **fourth VERB-first arm**, and the correction **strengthens** the headline.
- **EC-32** the pre-registered mechanism is confirmed in **2 of 3**, not 3 of 3 (B3 read at
  invocation 3 and resumed at 6); **invocation 1 is the harness's `SessionStart` hook**, not the
  worker's; and the adapter's `SKILL.md` — tool call 1 in every arm — names the read verb
  **verbatim**, so the route was available directly (it shipped at M48 and cannot explain 1/3→3/3).
- **EC-33** `1799a2d` is **not in the rc.13→rc.14 diff** (`git merge-base --is-ancestor` → yes); the
  coverage table's stated subject is wrong about that member.
- **EC-34** coverage **row 12 is classified from the artifact, not the code** — the sixth instance of
  the failure `pinning.md` §5 catalogues five of; and **one M50 surface is in no column at all**
  (Increment 10's `pack.resource-missing`), whose correct column is *test-fenced*, so nothing is
  unfenced.
- **EC-35** F-3's row writes `pinned-by:` `crates/cli/tests/…` — **an ellipsis where a citation
  goes**, immediately before its `UNPINNED:`. Delete it.
- **EC-36** the blanket rider *"none moves a pinned contract"* is true and its implied gloss *"and is
  therefore cheap"* is false for F-8, F-3 and half of F-11. **This row is the boundary's pricing
  instrument** and is why F-3 and F-8 are excluded rather than tiered.
- **EC-37** N20 is **knob-independent** — driven byte-identically under `squash: false`. *Axis:*
  every `decisions-pending.md` entry whose scope is stated as a condition, re-driven with that
  condition negated.
- **EC-38** N23 re-driven and **confirmed** — the ledger's classification is right and it is not a
  misfiled loss cell.
- **EC-39** the categorical headline *"zero data loss, zero corruption, zero regressions, nothing
  blocking"* outruns the trial's coverage; the defensible wording is *"no data loss or corruption
  was observed in the reached trial paths"*. **The five unreached cells were subsequently driven and
  all matched contract** — the correction is to the wording, not to the result.
- **EC-40** the arm count disagrees with itself (23 vs 24 including arm 00).
- **EC-41** the bypass chain's sizing missed a residue — two files left staged and uncommitted, **not
  lost**, and reported by no jigc surface, which is the same gap F-6/N27 names from the other side.
  The chain's **core claim was re-driven and holds**, and *content-keyed, not sha-keyed* is
  structural (`file_state.rs:63-64`).
- **EC-42** the record's scored table prints a bare `fs` column where RC-m50 split `fs (DOC/wkbn)` —
  **the reader still computes the split**, and that dropped column is exactly what hid EC-31.
- **EC-43** the corpus fixture **moved** between RC-m50 and rc.14 (the PT-D `IngestQueue` change, a
  declared fixture change); no arm isolates it. **The one legitimate soft spot in the 1/3→3/3
  comparison, and the record names it** — this row is a disclosure to carry, not a defect to fix.

## The razor

M46's three legs for a defect, M49's fourth for a shape, with M46's falsifier kept verbatim
([M49/settle-record.md](../M49/settle-record.md) → D2):

0. **Necessary — on the pre-1.0 critical path**: a **one-way door at the pin**, or a **hole in a
   declared surface**. This is the leg that refuses.
1. **Demonstrated, not asserted** — a rule **stated** in a locked artifact and **violated** at HEAD,
   *or* a defect **driven** on the binary. Both require execution, never a source read.
2. **Right for any adopter** — if the only beneficiary is our layout, our naming or our process, it
   is refused and we change instead.
3. **Timing is named** — one-way at the pin ⇒ state the concrete post-1.0 cost; not one-way ⇒ it may
   be deferred without ceremony.
4. **Falsifier** — *if the razor cannot refuse, the claim is wrong.*

**[Corrected 2026-09-11 at the Settle: the ground below is stated backwards for a change to an
*existing* frozen doctype.** *"Additive after the pin"* holds for a **new** doctype — the methodology
manifest says so in its own words (*a new doctype is free at the freeze*). For a **shape change to an
existing** frozen doctype it **inverts**: the bump taken today ships a migration over **zero** adopter
corpora, while the same bump after 1.0.0 runs over **every adopter's store** and flips their CI red
until it does — textbook *expensive after the pin*. **The exclusions stand, on a different leg:
F-3 and F-8 are refused on the *necessity* leg** — no adopter needs either today, and the M50 Settle
refused the same edge on `methodology-docs.md:42`'s universe rule — **not** on additivity, which a
future wave would otherwise cite as precedent. F-8 is flagged as the natural **first post-1.0
migration** (`AddedOptionalField`, a stamp-only migration) when its trigger fires; F-3 is the
expensive one, a change to the schema-definition format, itself frozen v1.
[settle-record.md](settle-record.md) → D13.]**

**What it refuses by construction, with the citation.** **Any new verb, any schema-shape change to a
frozen doctype, and any second execution shape** fail leg 0 as a class: each is **additive after the
pin**, so none is a one-way door, and none is a hole in a *declared* surface. That is exactly
**F-10** (a new amend verb), **F-8** (`adr → research`: manifest `schema-version` 2→3 **plus a
shipped corpus migration for every adopter** — the freeze doing its job, EC-36), **F-3** (a
conditional field requirement no schema key expresses — a change to the **schema-definition format**,
itself frozen v1, plus `doc schema` `contract-version` 6→7, EC-36), and **F-13** (a second milestone
execution shape — the trial record itself classifies it *"a **design** question, not a fix"*). The
razor must also be able to refuse inside the ledger: any Tier-2 or Tier-4 row that survives only as
*nice* — the human's criterion supplies the tiebreak, *expensive after the pin* admits, *nice* does
not — and **any row the baseline cannot reproduce**, which leaves the wave as a corrected record
rather than as a fix.

## Forks for Settle

Each states the **cheap** and the **robust** reading and stops there. **No recommendation is carried
in this charter** — a robust-advocate argues the robust side at Settle, against a proposer who did
not author it.

- **F1 — EC-1, the shape of the path guard.** ~~*Cheap:* adjudicate at the `migrate` door only,
  where the loss was driven.~~ **The cheap option is falsified by the baseline** — a door-only guard
  cannot close the class: with a **benign** in-repo token at the door, overwriting the recorded
  `source-path` in the mutable working area lands `--approve` at **exit 0** with an arbitrary host
  file deleted and the ack naming the benign source ([baseline-ledger.md](baseline-ledger.md) → §0).
  So the fork is now: *door + sink* — the door adjudicates and `plan_retirements`/`task::retire`
  (plus `retire_exempt`, the second raw consumer) re-validate the recorded token — **vs.** *door +
  sink + the classified `Plain`-family registry*, because the five exempted argument names were
  driven and ask **four different questions** (`unmanage`'s `path` and `config`'s `target` are
  built + proven, `relocate`'s `from` is a git-enumerated filter, `from_file` is an unbounded read
  by design with a route-floor gap, and only `migrate`'s `path` and `config`'s `file` carry the
  escape) — so the robust arm is a **registry with one stated predicate per member**, not one shared
  predicate, and it is the half that buys the missing door registry and axis suite.
- **F2 — EC-2, refusal vs consent, and the exemption.** *Cheap:* narrate the detached HEAD and let
  the door proceed under a stated consent. *Robust:* refuse at the shared seam. Either way the fork
  must say **how the fan-out worktree — detached by design — is exempted**, and whether that
  exemption is a probe of the worktree or a flag the boundary passes. **The baseline measured the
  exemption's shape** ([baseline-ledger.md](baseline-ledger.md) → §4): 9 of 10 doors funnel into
  `task::git_commit_capture`, but **the two boundary arms pass a `DedicatedWorktree` through that
  same function**, so a naive seam probe is wrong in exactly the two members needing the exemption —
  the exemption has to be **carried by the caller**, not sniffed at the seam. The act that decides
  the boundary is `git merge --ff-only` in `overlay_docs_commit_and_ff`, **downstream of the seam**,
  and **`jigc setup` bypasses the seam entirely** (`git_output`), so **the probe belongs at the door
  and at the fast-forward**, with the live checkout at the landing act as its subject. The fork also
  now has to answer the **linked-worktree split** (cell 10 above): posture is not only *is HEAD
  attached* but *which repo is this door acting on*.
- **F3 — EC-29, what the transaction promises.** *Cheap:* keep `design/finalize.md`'s declared
  index-only fidelity and **correct the *"all-or-nothing"* header two rows above it**. *Robust:*
  rollback restores the **worktree** too, over the fifth un-swept dimension of the rollback matrix.
- **F4 — EC-26, `setup` and the carryover gate.** *Cheap:* `setup` **narrates** what it swept into
  its commit. *Robust:* `setup` **joins the carryover gate** — the gate's subject becomes the door,
  not the task, with a stated exemption for any door that must commit outside one.
- **F5 — F-5 / F-11, code and route vs the envelope.** *Cheap:* a code and a route **inside the
  flattened `{"error": …}` string**, which EC-36 prices as free and which the M50 audit already
  chose once with measurements (`location: None` ⇒ a `(code, null)` key strictly less informative
  than the flattened message). *Robust:* the **findings envelope** at those doors — a **wire change
  on a de-facto-pinned shape**, and therefore cheap only before the pin.
- **F6 — EC-8 / EC-9, the versioning policy and the publishing floor.** The **content** of the
  policy (what a 1.x may change, what triggers 2.0, whether a verb, flag, finding code or exit code
  may be removed inside 1.x, whether minting `command-output contract v2` is a 1.x act, and what the
  probe wire binds to) and the floor's membership (README · CHANGELOG · `publish` · crates.io
  metadata · tag/release notes). **These are human decisions, not code**; the wave writes down what
  is decided and fences what can be fenced.
- **F7 — EC-3 / EC-5, declare or delete.** *Cheap:* **declare** the four undeclared keys and state
  the versioned/unversioned envelope rule as it stands (typed result values carry `schema_version`;
  ad-hoc `json!` envelopes do not). *Robust:* **delete** what should not have shipped and close the
  partition against the clap leaf tree, so the rule is a fence rather than a sentence. The
  contract's own rule is the pressure: *"an undeclared key on a pinned envelope is a defect, not an
  addition, whichever wave mints it."*
- **F8 — the ledger question** ([VERDICT](../evidence-check-1.0/VERDICT.md) → *The ledger
  question*). Read **closed under the stated criterion** — every row carries `pinned-by:` or a stated
  `UNPINNED:`, the test both prior closures used — or **substantively open**, because F-3, F-5, F-9
  and F-11 are CONFIRMED defects with no standing test and `UNPINNED` is an audit exception rather
  than a substitute for conversion. **The gate is the human's by the record's own words**; the
  reading is recorded either way, and the wave's Tier-1/Tier-2 fixes convert three of those four
  rows by construction.
- **F9 — the acceptance instrument.** The **per-axis review** (below) is the wave's acceptance,
  **decided by the human 2026-09-10**. What is **open**: whether a **blind trial also runs before the
  1.0.0 call**, and if so whether it is a full protocol or a single re-measure. The datum on the
  table: the rc.14 duress cell is at **3/3** and the check found **no product regression** in it —
  but the two blockers were found by **driving axes no trial charted**, which is the argument for
  the review and, separately, an argument that a trial would not have found them either.

## Decided OUT, with the ground

- **F-3** (a conditional field requirement — a **schema-format** change, itself frozen v1, plus
  `doc schema` `contract-version` 6→7) and **F-8** (`adr → research` — a frozen dev-pack
  `schema-version` 2→3 **plus a shipped corpus migration for every adopter**). Both priced by
  **EC-36**; both **additive after the pin** and therefore not one-way; **F-8's trigger already
  stands** in [decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.14
  trial's findings* (*whichever wave first needs a cross-doctype ref*) and F-3's alongside it (*the
  first doctype that needs a conditional field requirement*). **The M50 Settle refused the same edge
  once already**, at `methodology-docs.md:42`'s universe rule and a failing necessity leg.
- **F-10** (a verb that amends a landed commit) — a **new verb**, additive after 1.0; its trigger
  stands with the bypass-chain entry, and **F-9 is the cheapest link in that chain** and is *in*.
- **F-13** (a second, sequential milestone execution shape) — the trial record classifies it *"a
  **design** question, not a fix"*, and its trigger stands: *whichever wave first takes the
  sequential-execution question*. It is also **why the `1799a2d` boundary was reached by nothing** —
  the uncovered arm and the gap are one event, and that is a trial-coverage fact, not a wave item.
- **The doctype-completeness milestone** ([decisions-pending.md](../../../implementation/decisions-pending.md)
  → *The doctype-completeness milestone (post-1.0)*) — explicitly post-1.0 by the human's decision of
  2026-07-10, and the reason this repo does not self-host (which is EC-9's context, not its fix).
- **The harness-surface wave** ([decisions-pending.md](../../../implementation/decisions-pending.md)
  → *The harness-surface wave*) — build infrastructure, reaching no adopter. Its own boundary
  paragraph already states this; M50 excluded it on the same ground.
- **The capability wave's remaining ledger** ([decisions-pending.md](../../../implementation/decisions-pending.md)
  → *The capability wave (M46)*) — adjudicated entry by entry at the M48 Settle and disposed at M46;
  what remains is deferred with its own dispositions and is not re-opened by an evidence check.

## Acceptance

**The per-axis review, run on the built and installed `1.0.0-rc.15` binary** — not on the source, and
not before the audit's fixes land, per the convention five consecutive waves have needed (EC-10).
**Eight axes**, each a cell matrix driven end to end, each staffed as **one Opus driver plus one
Codex source pass**:

1. **Caller-supplied tokens** reaching a path or git argument, **at every door**.
2. **Git and environment posture** at every **committing** door.
3. **Destroying doors × leftover shapes × consent.**
4. **Transaction and rollback** at every committing door **× failure point**.
5. **The pinned read and write contracts.**
6. **Composed surfaces and discoverability.**
7. **Freeze and migration.**
8. **Adopter docs and help against the binary.**

**The fence on the review itself: every leaf in `VERB_KINDS` (`crates/cli/src/cli.rs:1412`) must
appear in at least one axis's cell matrix, or be named uncovered.** A review that silently omits a
verb is the instrument failure this wave exists to correct — EC-34 is that failure inside the
*previous* instrument, and the fence is the same shape as the coverage rule it violated.

**Why this instrument and not a trial.** Both 1.0-blocking defects were found by driving an **axis**
— a path-taking argument family, a git posture — and neither is verb-shaped, which is why four
consecutive trials and every M50 registry missed them. A blind trial measures what a worker *does*;
it does not enumerate a cell matrix. Whether one **also** runs is fork 9.

**Then the 1.0.0 call, which is the human's.**
