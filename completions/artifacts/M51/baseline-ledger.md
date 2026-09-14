# M51 — the baseline ledger

**Driven 2026-09-10, at `HEAD = bd348a83`** (branch `main`, clean tree), against the **release**
binary `target/release/jigc` = **`1.0.0-rc.14`**. Fixtures from
`dev/jigc-rig <state> --binary target/release/jigc` (two-step eval, roots under the session
scratchpad, `committed-singletons` · `fresh` · `vendored` · `bare`). **Debug posture was used
nowhere** — the route-fence panics do not exist in the release build, and one row below is
explicitly out of reach because of it.

**Provenance.** Four **Opus `capability-auditor` subagents**, one per area, each **driving the
binary** and returning the command and its output. **None ran cargo; none edited a repo file.**
Their ledgers are appended as companions, verbatim, each with its own provenance line:

| companion | area | rows the wave leans on |
|---|---|---|
| [baseline-tokens.md](baseline-tokens.md) | caller-supplied tokens that become filesystem paths or git arguments | EC-1, EC-25, EC-27, EC-28, `ingest` |
| [baseline-committing-doors.md](baseline-committing-doors.md) | the committing-door seam and git posture | EC-2, EC-18, EC-26, EC-29, N20 |
| [baseline-envelopes.md](baseline-envelopes.md) | the pinned-envelope and fence machinery | EC-3, EC-4, EC-5, EC-6, EC-7, EC-10, EC-20 |
| [baseline-prose.md](baseline-prose.md) | the prose surfaces 1.0 ships, and the fences that keep them true | EC-11 … EC-17, EC-19, EC-21 … EC-24, EC-38, N15, N23 |

Citations below are `[tokens §n]`, `[doors §n]`, `[envelopes §n]`, `[prose §n]` and point at the
companion's own section. **Every claim in this section traces to a companion line**; the one place
where it does not — a transcript quote kept so the datum survives the scratchpad — says so in its
own sentence.

**A map, not gospel.** Verified at this sha, nothing settled: the Settle is the human's, and the
gap-detectors still spike the specific new shapes. **Relayed is not verified by the orchestrator** —
the rows the Settle turns on are re-driven at Settle, and the gate-record records which.

---

## What the baseline changed in the charter

### 0 · The headline — EC-1 is a **sink** defect, and fork 1's cheap arm is falsified

The charter's Tier 0 states EC-1 as a door problem with a sink rider (*"and the recorded
`source-path` must be re-validated at the destructive sink … because the working area is
mutable"*). The baseline **drove the rider** and it is not a rider: it is the defect.

Minted against a benign **in-repo** `foreign.md`, then `.jigc/tasks/<id>/source-path` overwritten
with an absolute path outside the repo — the working area is an ordinary mutable file —
`jigc task finalize <id> --approve` exits **0**, acks `finalized 64606da — docs(changelog): adopt
foreign.md as a managed changelog`, lists `foreign.md` as `left-out`, and the planted
`<victim>/keepme.txt` is **gone** [tokens §2a.1, §4].

So **a door-only guard cannot close the class**: the door saw a benign token, and the bytes died
anyway. `plan_retirements` (`engine/finalize.rs:400`) and `task::retire` (`cli/task.rs:3016`) read
the recorded token **raw**, and `retire` does `repo_root.join(retirement)` — an absolute value
replaces the root [tokens §4]. There is a **second raw consumer of the same untrusted token**: the
carryover-gate exemption `retire_exempt` (`task.rs:1426`, `:1794` → `finalize.rs:738`), so a
tampered `source-path` also buys an arbitrary path a pass through the gate [tokens §4].

Two facts make it worse and are new:

- **No surface names the file `--approve` deletes.** The exit-4 review hold — the **one human gate
  on jigc's only byte-destructive op** — prints the fidelity diff and *"retire the foreign
  original"* with **no path** (`grep -c "$EXT" hold.txt` → `0`); the pinned `--format json` hold
  carries exactly `["review","rewrites","source","task"]`, no retire key; and `jigc task validate`
  (M47's *"preview what finalize gates on"*) shows only the `file-state.staged-copy` advisory
  [tokens §2a.2, §4].
- **`trackable::untrackable_reason` — the predicate that already refuses every destructive cell —
  is never called by `migrate.rs`.** It ships at `cli/trackable.rs:55` with four callers
  (`config.rs:402`, `relocate.rs:58`, `rename.rs:532`, `setup.rs:1499`) [tokens §3].

**The `Plain` family is a list, not a class — the charter's own falsifier fired.** The claim's
first half says it is falsified *"if the five exempted argument names cannot share one predicate"*.
Driven, they cannot:

| member | door(s) | driven verdict |
|---|---|---|
| `path` | `migrate`, `unmanage` | **split** — `migrate` is the data-loss cell; `unmanage` is **built + proven** (7 shapes incl. `/etc/hosts`, `.git/config`, `../..`: all exit-0 no-ops, every file intact) [tokens §2a, §2b] |
| `file` | `config insert-step`, `config replace-step` | **latent defect (read escape)** — needs a *home* rule [tokens §2d] |
| `from_file` | `doc set-slot`, `doc author`, `config fill` | **by-design unbounded read** into a declared slot; the gap is the **route floor** (4 miss shapes, bare `anyhow` + errno) [tokens §2e] |
| `from` | `relocate` | **filter only** — sources come from `orphan::committed_markdown` (git-enumerated); 42 driven cells, `0 moved`, clean `git status` [tokens §2c] |
| `target` | `config fill/fork/replace-step/remove-step` | **built + proven** — fenced by *declared existence*, not by a grammar: `config.fill-point-absent` / `config.anchor-absent` with code + route, token echoed verbatim [tokens §2f] |

Five members, **four different questions**. So fork 1's robust arm is not *"one shared predicate at
every member"* — it is a **classified registry over the `Plain` family**, one stated predicate per
member (or a stated *no rule, and why*), with a fence over the list. And the gap the baseline makes
visible is structural: the other four token families each have a **code-side door registry and an
axis suite**; the path family has **neither** — no registry enumerates the path-taking doors, and
`untrackable_home_axis.rs` says in its own header *"the axis is the destination, not the knob"*, so
the **source** side is unswept [tokens §1].

### 1 · Tier 0 gains five cells

1. **The `source-path` sink re-validation — now measured, not argued** (§0 above) [tokens §2a.1,
   §4]. It changes what fork 1 decides, not only how much it costs.
2. **`jigc config insert-step … <file>` / `replace-step <target> <file>` — a read escape into the
   repo *and* into the agent's composed context.** An absolute outside path, a `../` escape and
   `.git/config` are all accepted at **exit 0** and copied verbatim into `.jigc/config/steps/<stem>.yaml`
   — an in-repo, committable file. Then `jigc start --workflow single-task "probe compose"` renders
   `Relative escape body.`, `Secret from outside.` **and `repositoryformatversion = 0` / `bare = false`**
   into the step text the agent reads [tokens §2d]. Traversal is impossible in the *destination*
   (`file_stem()` yields one component); the escape is the **read** and what it lands in-repo.
3. **`jigc migrate .git/*`.** `.git/config` as a source: mint exit 0, `--approve` **exit 0, the file
   deleted**. `.git/HEAD`: `--approve` exits 1 with HEAD deleted mid-transaction, and rollback
   restores HEAD but **leaves the index carrying staged deletions of `.jigc/.gitignore`,
   `.jigc/config/*`, `.jigc/version` with the files untracked on disk** (reproduced twice against a
   fresh rig whose `git status` is empty) [tokens §2a].
4. **The `GIT_DIR` redirect.** With `GIT_DIR=<other repo>/.git`, `jigc milestone create` exits **0**:
   the record file is written into repo A's tree and the commit `db89e71` **lands in repo B**,
   leaving B with ` D x` and all of A untracked. `grep -rn "env_remove\|env_clear" crates/cli/src`
   → no production hit: **no git-env scrubbing anywhere** [doors §2(d)].
5. **The unborn-HEAD milestone.** On an orphan branch, `milestone create` exits **0** and writes
   git's **empty-tree hash** into the *committed* record (`base: 4b825dc642cb…`); `add-task` accepts;
   `provision` then fails `milestone.provision-failed` (*"object … is a tree, not a commit"*) and
   `milestone finalize` fails with a **bare `anyhow`** — no code, no route, no state clause,
   `error_code: null` in the invocation log. The only exit is `milestone discard --force`
   [doors §2(c)].

### 2 · Tier 1 gains ten cells

6. **The pinned-envelope *list* itself — EC-3 is a class, not four keys.** All **47** leaf verbs
   speak `--format json` and were driven to a genuine success; the four keys EC-3 names are real,
   **and a grep of every driven key against all of `design/` returns 13 further envelope keys that
   no design document names** — `uninstalled` · `rows` · `summary` · `identity` · `new_path` ·
   `old_path` · `prose_mentions` · `referrers` · `moved` · `displaced` · `reslugged` · `layer` ·
   `rejected` · `knobs` · `anchor` · `side` · `step` · `overlay`, across `uninstall`, `ingest`,
   `unmanage`, `rename`, `relocate`, `doc rename`, `config get`, `config list`,
   `config insert-step` and `milestone join` [envelopes §1]. **The gap is that nothing enumerates
   which envelopes are pinned at all**: `command-output-contract.md`'s *"The three surfaces this
   pins"* covers composed output, write-acks and the findings envelope — `setup`, `uninstall`,
   `ingest`, `unmanage`, `rename`, `relocate` and the eight `config` envelopes are in **none** of
   the three, and `text_json_parity_axis::REGISTRY` classifies **fence-ability**
   (`Tier::Fenced`/`Tier::Judgment`), never **pinned-ness** [envelopes §1, §2]. So `guide_file` is
   "a defect" only because `hook_file` beside it happens to have a paragraph, while `identity` on
   `unmanage` is unremarked. **This decides fork 7's subject**: declare-or-delete needs a list to
   range over before it can be either.
7. **`milestone execute` is a *fourth* composed producer.** `command-output-contract.md` §1 says
   *"**Three verbs** emit this composed shape … `jigc start`, `jigc workflow`, and `jigc migrate`"*;
   driven, `jigc --format json milestone execute cache-rework` → `{"task": …, "text": …}`. The
   code-side census already knows — `text_json_parity_axis.rs`'s own registry entry says
   *"`execute` renders through `render::composed` … stdout IS the pinned `{task, text}` contract"*
   — and the contract doc does not. Neither `migrate` nor `milestone execute` has a closed-key
   assertion: all four `assert_eq!(keys, ["task","text"])` sites drive `start`/`workflow`
   [envelopes §1 · F-new-1].
8. **`task list --format json` is a top-level *array*.** Every other envelope is an object; this one
   is `[{id, workflow, intent}, …]` (`render.rs:3285`), so it can carry no `schema_version`, no
   `findings` and no discriminator, and a driver cannot deserialize one envelope shape. No design
   doc states it, and `format_json_success_axis` passes it because its predicate is *parses as
   exactly one JSON document* [envelopes §1 · F-new-3].
9. **`milestone list-tasks` — a `VerbKind::Read` verb ships `hook_output`.** It shares
   `render::milestone` with five write siblings and emits `{"hook_output": "", "text": …}`, while
   `hook_output`'s declaration is scoped to *"the landed-commit envelopes … and the milestone
   record-only op acks"*. The value is structurally always `""` [envelopes §1 · F-new-2].
10. **The user's linked-worktree split — the doors disagree about which repo they act on.** Driven
    from a `git worktree add -b feature <wt>`: `task finalize` commits onto **`feature`** (correct,
    the promoted ADR lands in the worktree and rides the commit), while `milestone create` writes
    `docs/milestone-records/probe.md` into the **main checkout** and commits `19567e2` onto
    **`main`** — the operator's worktree gets neither the file nor the commit; `milestone add-task`
    is the same [doors §2(b)]. This is a **posture cell in no EC row**, and it lands on fork 2: the
    exemption fork must say what a committing door's *subject repo* is, not only whether HEAD is
    attached.
11. **The survivable frame's cause vocabulary — N20 widens to every non-`CommitRejected` boundary
    error.** Driven, a boundary refused by `git merge --ff-only` (not by a hook) exits 1 with the
    frame's whole apparatus gone:
    ```
    argv: jigc milestone finalize probe-milestone      EXIT=1
    `git merge --ff-only 0273928…` failed: error: The following untracked working tree files
    would be overwritten by merge: src-alpha-task.txt … Aborting
    LOG: 1 None []          ← error_code: null, finding_codes: []
    record status: active
    ```
    *(Quoted from the doors auditor's own transcript `baseline2/n20.txt`; the same evidence block
    and the widening are in the re-saved companion's §2, which also names the mechanism —
    `task.rs:3534` renders the frame only on the `CommitRejected` downcast, so `git_run`/
    `git_worktree` failures are plain `anyhow` [doors §2].)* The
    unborn-HEAD `milestone finalize` (cell 5) is a **second** non-hook instance of the identical
    shape. So N20's subject is not *the hook-rejection frame at the boundary*: it is **every
    boundary error that is not a hook rejection**, and telling that reader to *fix the hook* is a
    law-1 lie about the cause.
12. **`jigc task discard`'s own ack is silent about its commit.** Both shipped guides say the door
    makes no commit (`MIGRATING.md:40`, `QUICKSTART.md:180`) and it is the **tenth**
    `COMMITTING_DOORS` member — driven on a milestone sub-task, `HEAD 520283b → 1152124`,
    `chore(milestone): discard task:measure-the-hit-rate on milestone:cache-rework`, exit 0. **New,
    in no ledger row: the success line names no sha**, where every other committing door prints one
    [prose §3 · EC-12].
13. **The sub-task workflow's own `resume:` line does not provision its commit doc — driven end to
    end, two arms in one corpus** [doors §3]. The composed footer prints *"resume: `jigc start
    --task <id>` — re-composes this workflow if context is lost"*; the spawn line `milestone
    execute` prints is *"Spawn: `cd .jigc/worktrees/<id> && jigc workflow sub-task --task <id>`"*.
    They do not do the same thing:
    ```
    # ARM 1 — the composed footer's own resume line
    cd .jigc/worktrees/alpha-task && jigc start --task alpha-task      → EXIT=0
    ls .jigc/tasks/alpha-task/docs/  → No such file or directory
    jigc doc set-field commit:alpha-task#type --task alpha-task --value feat
      → "no staged instance for `commit:alpha-task#type` — task alpha-task's workflow provisions
         its `commit` doc at compose and grants no in-task create for it; list what task
         alpha-task stages with `jigc doc list --task alpha-task`"
    jigc doc list --task alpha-task  → "no docs staged in task alpha-task"

    # ARM 2 — the spawn line milestone execute prints
    cd .jigc/worktrees/beta-task && jigc workflow sub-task --task beta-task  → EXIT=0
    ls .jigc/tasks/beta-task/docs/   → commit:beta-task.md   provenance.json
    ```
    **The finding:** `resume:` re-composes **without provisioning the transient `commit` doc**,
    while the spawn line provisions it. A sub-agent whose first or resumed entry is the `resume:`
    line gets **a workflow it cannot author**, and the refusal's route (`jigc doc list --task …`)
    **prints an empty list and never names the provisioning verb** — recoverable by running the
    workflow verb, and nothing tells you to. It is the M31→M32 lesson's exact shape, on the surface
    built to answer it.
14. **`jigc ingest`'s self-contradiction, with a looping route.** The carried entry
    (`decisions-pending.md:593`) names the *registration*; driven at HEAD it is three doors and two
    answers over one file: `jigc ingest` → exit 0 *"adoptable docs/research/OddName.md → research
    (adopted — indexed + baselined, no file moved)"* and it **does** persist into
    `state/file-state.json`; `jigc doc list research` → `unregistered`; `jigc doc show
    research:OddName` → exit 1 `store.malformed-slug`; `jigc validate` → **exit 1**, advisory
    `schema-conformance.unadopted-instance … never adopted by jigc`, **routed *"run `jigc ingest`
    to route it"*** — the verb that just claimed the adoption. A second `ingest` repeats the claim.
    **A route that, followed exactly, changes nothing** — PT-1's shape at the ingest door
    [tokens §5].
15. **`jigc task finalize --carry-staged` concludes a merge.** With a merge in progress and the
    resolution staged, the carryover gate correctly blocks (exit 3, `finalize.carried-staged`);
    the override then exits **0** and lands a commit with **two parents**, consuming `MERGE_HEAD`,
    under jigc's own subject `chore: tweak the greeting`, and the ack says `1 file committed` and
    **never mentions a merge** [doors §2(e)]. The consent the flag documents is *carry my staged
    work*, not *conclude my merge*.

### 3 · Corrections to charter rows — four narrowed or refuted, each with its datum

- **EC-28's `doc create` claim is REFUTED as written, and the row is a 3-door class.** The charter
  reads *"`doc rename --slug <300 chars>` fails with a bare `File name too long` … `doc create
  --slug` refuses correctly at the create-gate."* Driven: `doc create adr --title X --slug <300>`
  fails on the **same OS ceiling** and calls it `blocking · task.working-area-io — could not
  provision the instance for task 'adr:aaa…'`, `at: task:adr:aaa…` (a malformed address), with the
  route *"resolve the underlying I/O condition (a disk or permissions problem…)"* — **a false
  route**: there is no disk or permissions problem. `start --workflow single-task … --slug <300>`
  answers the same way. `doc rename` is the code-less bare-error cell EC-28 reports. `doc add-item
  --slug <300>` **exits 0** (an item id is not a filename), and the `migrate --slug` cell is
  **masked, not clear** (`changelog` is a fixed-slug singleton). State is intact in every cell; the
  defect is the surface [tokens §2g].
- **EC-14's *"eleven"* is correct — the defect there is that it is unfenced.**
  `surface-contract.md:129`'s *"member-for-member — eleven"* matches `ERROR_CODE_REGISTRY`'s 11
  members and the table's 11 rows. The real row is carried defect **(h)**: the registry test
  hardcodes `11` and **names that file in its assert message while no test reads it** — the twin of
  (g) on the next door added [prose §5, §9.1]. Two further EC-14 corrections in the same pass: the
  *"window closes here"* pair is **presentation, not a lie** (both paragraphs state the close is
  keyed to the 1.0 pin, not a wave name, and the M49 spend below each is explicitly declared)
  [prose §5, §9.2]; and CLAUDE.md's *"thirteen `CELLS` rows"* is **false and was false when
  written** — `write_miss_cells.rs::CELLS` had **63** rows at `32de1121`, the very commit that
  wrote the sentence [prose §5].
- **EC-7 is narrowed: right about `e2e_audit.rs`, wrong as a claim about the tree.** The
  self-reference is real (`e2e_audit.rs:261` `include_str!`s the profile and scrapes its `deny:`
  block), **but two independent literal sets exist in the crate's own source** —
  `adapter.rs:1495-1547` (`claude_code_profile_bytes_are_canonical`) and `adapter.rs:2622`
  (`deny_floor_added_then_idempotent`, whose inline snapshot lists all **22** patterns verbatim) —
  and deleting `Read(./**/*.key)` reddens both. Live evidence the coupling is genuine: the
  gitignored `.adapter.rs.pending-snap` records a real pending diff from a past profile edit. **The
  residual gap is real but narrower**: an inline `insta` snapshot is re-acceptable by `cargo insta
  accept`, so the floor is fenced against an **accidental** drop and not against a **deliberate
  regeneration** — the same posture as the 634 compose goldens. Only the two human-owned destroyers
  are pinned by a *reasoned* literal (`adapter.rs:1569`) [envelopes §5]. (Note also the count: the
  charter says *"20 secrets patterns"*, the profile carries **22**.)
- **EC-20 is corrected at its subject: the divergence is `--dry-run`, not the committing door.**
  The contract sentence — *"every check `task validate` runs … is the same check `finalize` runs, at
  the same severity"* — is **not** falsified by the landed door: driven on one task at one moment,
  `task validate` and the landed `task finalize` emit the **identical** two findings. What diverges
  is the **forecast arm**: `task finalize --dry-run` has **no `findings` key at all** and drops
  **both** advisories, not only the changelog one (`task.rs:1827` computes the full report *above*
  the `if dry_run` branch and returns only the manifest). And `GATE_COVERAGE` cannot see it by
  construction — `gate_coverage.rs:43-49` states that the **membership** of `Tier::Previewed` is
  owned elsewhere, and `gate_coverage_fence.rs` is a **named-fact token guard over prose surfaces**;
  **no test compares the finding sets two doors emit** [envelopes §6, prose §3 Q2]. The minimal
  fence needs `--dry-run` to gain a `findings` key — an **additive key on a pinned envelope**, and
  therefore inside the closing window.
- **EC-17's axis is wrong in one cell and short by a dimension.** The fourth `DESTROYING_DOORS`
  cell **cannot exist**: `remove_worktrees` (`milestone.rs:4337`) filters against `git worktree
  list`'s **registered** set and `continue`s on a miss, so `milestone finalize` — and
  `milestone discard`, which shares the function — neither narrates nor destroys a symlink; it
  survives the boundary in silence. **The doors run two different subject derivations** — on-disk
  walk (`provision`, `uninstall`) vs registered set (`discard`, `finalize`) — and the narration bug
  is two derivations of one subject disagreeing about symlinks: `probe_leftover` uses
  `symlink_metadata(...).is_dir()` (→ `LeftoverShape::File`, *"it is a file, not a worktree"*) while
  `doomed_at` uses `path.is_dir()`, which **follows** the link and enumerates *through* it. Driven,
  `milestone provision --force` and `uninstall --force` both name **three of the repo's own
  committed, tracked docs** as unrecoverable while their shas are unchanged, and the same run calls
  one path a *file* two lines up and a *directory* two lines down; `milestone discard --force`
  prints nothing at all over the identical state. **A fix scoped to the four-member registry will
  miss that half the doors ask a different question** [prose §4, §9.3].

### 4 · Confirmed as the charter states them (drive-once, do not re-derive)

EC-2's core (13 doors driven on a detached HEAD, **all exit 0**, every commit branchless, `main`
untouched, and after `task finalize` a `git checkout main` drops the prose while `jigc validate`
reports a **phantom** `file-state.hash-matches` routed *"review the out-of-band edit"* — there was
none; `grep -rn "symbolic-ref\|rev-parse --abbrev-ref" crates/` → **0 hits**) [doors §2(a)] ·
EC-4 [envelopes §3] · EC-5, **sharper** (the versioned set is **7 of 48** arms, and two new cells:
one verb with two envelopes split on a flag — `task finalize` landed carries `schema_version`,
`--dry-run` carries none — and two sibling committing doors disagreeing, `milestone finalize`
landing `{committed}` alone) [envelopes §1] · EC-6 (presence/type only, **no key-set equality
anywhere**, and **no version integer** on a surface its own doc-comment calls *"independently
versioned"*) [envelopes §4] · EC-10 (the ten version-bearing goldens are the **only** mechanical
forcing function, and they force the *regeneration*, never the *bump*; `release_smoke.rs:100`
derives its expectation from the same constant and is green at any version) [envelopes §7] ·
EC-11 D4/D5/D6 [prose §2] · EC-12 [prose §3] · EC-15, **per blocked group, not a global
short-circuit**, with a mixed control [prose §4] · EC-16, **and a test pins the lie**
(`unmanage.rs:291-293` asserts the over-claiming phrase is present) [prose §4] · EC-18 (whole-file
`fs::write(&path, ENTRIES)` against its own doc-comment's *"amended once to the union"*, 4
production callers) [doors §6] · EC-19 Q1/Q2, EC-21, EC-22, EC-23, EC-24 [prose §3] · EC-25,
**≥3 doors not 1** (`doc retitle-item`, `doc add-item`, `doc set-slot`) [tokens row 16] · EC-26
[doors §4] · EC-27, **widened** (`" "`, `"   "`, `"  x  "` and `"-"` all exit 0; `config get`
reads back indistinguishable from unset; the predicate wants *"a value that reads back as
itself"*) [tokens §2h] · EC-29 (**driven**: under a rejecting hook the config-layer family leaves
` M .jigc/version`, ` M .jigc/.gitignore`; the other four captured-pre-image families were clean)
[doors §5] · N15 (**and its mitigating `note:` rides the stream `doc show --help` tells the reader
to discard**) [prose §6] · N23/EC-38 (registration loss, not byte loss; **a control is recorded** —
a project-*listed* pack that defines no doctypes does not reproduce it) [prose §6].

**Fork-relevant measurement for fork 2, beyond the cells above.** 9 of the 10 committing doors
funnel into `task::git_commit_capture` (`task.rs:4255`) in the repo the operator stands in — but
the **two boundary arms pass a deliberately detached `DedicatedWorktree`** (`task.rs:4599`) through
that *same* function, so a naive probe at the seam is wrong in exactly the two members that need
the exemption, and it would still miss the act that matters — `git merge --ff-only` in
`overlay_docs_commit_and_ff` (`task.rs:4406-4430`), **downstream** of the seam. **`jigc setup`
bypasses the seam entirely** (`setup.rs:1709`, `git commit --no-verify` via `git_output`), so no
probe there reaches it. The sub-task worktrees never commit (`finalize.milestone-sub-task` fences a
per-sub-task finalize: driven, exit 3, routed at `jigc milestone finalize`). **The rule's real
subject is the live checkout at the landing act** [doors §1, §3].

### 5 · What the baseline did **not** reach

- **No mutation was applied.** Every *fence-quality* grade is from reading the test body plus
  driving the binary; no key was deleted to watch a suite redden. **EC-7's correction in particular
  deserves one applied mutation before it is acted on** [envelopes, closing section].
- **No cargo run**, by instruction; `text_json_parity_axis`, `format_json_success_axis` and
  `invocation_log` were read, not executed. The gate at this sha is the orchestrating session's
  (`gate-rc14-at-bd348a83.log`, 3341/0) [envelopes, closing section].
- **`engine::validate::adoption_route`'s unquoted-path-token panic is debug-only** and the rig
  drives release, so it was **not re-driven** [tokens §5].
- **`jigc migrate .jigc/<x>` was driven only to mint**; the `--approve` deletion of a workbench file
  is inferred from the same `retire` path [tokens, reachable-but-unmeasured].
- **The `LeftoverShape` axis was driven for symlink→directory only**; `{regular file, directory,
  symlink→file, symlink-into-the-repo}` were not. `DESTROYING_DOORS` door 4 was reached by **call
  path, not by drive** [prose §10].
- **`migrate-corpus --help`'s "v0→v1" falsity** is established from the manifests and
  `SchemaChangeKind::ALL`, not by driving a multi-kind migration; **N23 was reproduced through
  `JIGC_PACK_DIR`**, not through a project-pack doctype retirement [prose §10].
- **EC-30's milestone arm was not driven** by any auditor — the charter's instruction stands: drive
  it first; if it refutes, it is recorded as refuted and nothing is built.
- *(Persist note, 2026-09-10: the doors companion was re-saved complete — §1–§7 plus its own
  "Honest bounds" section — after this section was first written, so its own bounds are read there.
  The N20 widening's driven datum stays quoted in cell 11 above so it survives the scratchpad; it is
  also in the companion's §2.)*
