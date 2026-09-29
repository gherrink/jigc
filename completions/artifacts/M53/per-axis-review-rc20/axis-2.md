<!-- Reconciled AXIS 2 file, copied verbatim. Driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.20` (repo HEAD `4d3175c3`), 2026-09-27. -->

<!-- M53 FOURTH PARTIAL per-axis review — AXIS 2 · posture — the RECONCILED axis file.
     The Opus driver table below is reproduced VERBATIM (no demotions were required: every
     row lacking a fenced repro block in the driver file was re-driven by the reconciler and
     stands on the reconcilers own repro — see the ledger's "Rows filed without a repro block").
     The Codex source pass (codex/axis2-codex.md) is entered claim-by-claim in the ledger.
     Reconciler drove on /Users/maurice/.local/bin/jigc -> jigc 1.0.0-rc.20, repo HEAD 4d3175c3,
     2026-09-27. Rigs from dev/jigc-rig only (every root mktemp -d; no teardown; no rm -rf on a
     variable path anywhere), plus three non-rig fixtures a real repository can have: a rejecting
     hook behind core.hooksPath, a deterministic git shim on PATH, and a second repository.
     Nothing was fixed, committed or edited in the working repository. -->

<!-- M53 FOURTH PARTIAL per-axis review — axis 2 · posture — the OPUS DRIVER. Driven on the installed
     `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.20` (repo HEAD `4d3175c3`), 2026-09-27. -->

# M53 — the FOURTH PARTIAL per-axis review — AXIS 2 · posture — the OPUS DRIVER

**Binary.** Asserted **first, before anything else ran**:

```
argv : /Users/maurice/.local/bin/jigc --version
     -> jigc 1.0.0-rc.20
repo HEAD at read time: 4d3175c3 ("chore(release): 1.0.0-rc.20 — the fourth M53 stamp, after the
                                   usability batch and F-10's review fixes")
```

This is the **release** posture: the debug-only `Route::mechanical` argv fence does not exist here, so a
route-fence violation shows up as a **bad emitted command**, never as a panic. Every emitted `git …` span
in this file was therefore **executed**, not asserted.

**Baseline re-driven.** [per-axis-review-rc19/README.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/per-axis-review-rc19/README.md)
§A (all tiers) + [per-axis-review-rc19/axis-2.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/per-axis-review-rc19/axis-2.md)
(rc.19, HEAD `a8904637`) — its four findings `N-1`…`N-4`, the two rc.18 rows `(2, F-1)`/`(2, F-2)`, the
three M52 rows `(2, DEFECT A/B/C)`, `(2, DEFECT 1)`, and M51's `D1`/`D2`. **What changed under them:**
[VERDICT.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/VERDICT.md)
→ Addendum 3 — the pre-v1 usability batch (`be40738e`…`b6b1a18f`) and **one new capability, F-10
`jigc task amend`** (`407ebf08`…`bb252c25`), whose design of record is
[f10-amend-settle.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/f10-amend-settle.md)
and whose eight review findings are in
[audit/f10-code-review.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/audit/f10-code-review.md).

**Fixtures.** `dev/jigc-rig` only (every root from `mktemp -d`; **`rm -rf` on a variable path appears
nowhere in this review** and nothing was torn down), plus the binary itself, plus three things a rig
cannot express and that are legitimate fixtures rather than jigc state: a rejecting hook behind
`core.hooksPath`, a deterministic `git` shim on `PATH` for the seam race, and a second repository for the
foreign-worktree cell. **Nothing was written into any `.jigc/` by hand. Nothing was fixed, committed or
edited in the working repository.** Roughly **fifty** rigs were built; one of them under a repository
path containing a **space**.

**The four cwds, used throughout:** **(a)** `$REPO` · **(b)** `$REPO/docs/deep` · **(c)**
`$REPO/.jigc/worktrees/<sub>` (a provisioned fan-out worktree, detached) · **(d)** an ordinary **linked**
worktree outside `.jigc/` (branch-attached *and* `--detach`, both). A fifth, **`/`** — outside the
repository entirely — wherever the row is about *a route the operator pastes into a shell of unknown cwd*.

**Instrument note honoured.** Every exit code below was measured **bare**, with output to `/dev/null`,
never through a pipe. Where a first reading came from a pipeline it is called out in §10. `command grep`
was used for every read under `.jigc/`.

---

## 1 · THE HEADLINE — the tier-1 count

> # TIER-1 ROWS FOUND: **0**
>
> Tier 1 = exit-0 loss or repository harm through a committing, destroying or moving door. **No driven
> row on this axis reached it.**
>
> **The new committing arm is safe over every cell I could reach.** `jigc task finalize`'s amend arm
> refused at exit 1 under **all sixteen** buildable git states with `HEAD` **byte-identical** each time;
> refused over a non-empty index at **exit 3** across add / modify / delete / rename with HEAD *and* tree
> unmoved; refused at **all eight** `doc` write leaves before any managed doc was staged (`git status
> --porcelain` **empty** in every cell — the review's HIGH-1 is closed over its whole leaf axis, including
> the two leaves the review's own class count did not reach); refused on a moved HEAD; and under a
> rejecting `pre-commit` **and** `commit-msg` hook left `HEAD`, its message and its tree **byte-identical**
> with the task still live and the survivable frame's state-truth clause intact. On the happy path the
> tree and the parent were **byte-identical** and the author date preserved.
>
> **Three findings, none tier 1: one tier 2, two tier 3.** **A2-1** — the `--dry-run` help's parity
> sentence, false over **two** of the four gates the forecast refuses on; I found it on the amend arm and
> then drove it **pre-existing** on the ordinary arm's empty-commit gate, so the axis is the refusal set,
> not the amend cell. **A2-2** (tier 2) — the fan-out-worktree preview/seam posture split, whose emitted
> route exits **128**; outside the F-10 arc and outside the usability batch, a pre-existing gap driven at
> a `(door, cwd)` pair rc.18 and rc.19 did not reach. **A2-3** — a stale sentence in F-10's design of
> record, the envelope row it marked ***verify***, driven false.
>
> **Both rc.19 findings the usability batch claimed are CLOSED, driven:** `N-1` (the hook crying wolf) and
> `N-2` (the placement family's inert backstop — now blocking at **4/4** singletons).

---

## 2 · The door set and the registry counts, read from the code at HEAD `4d3175c3`

Read from the code, not from the design doc's numbers.

| registry | file | count I read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1877` | **48** leaf rows (**36** `Write` / **12** `Read`) — 47→48, `task amend` is the new `Write` leaf |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:2047` | **48** rows — the total classification |
| ↳ `ActsOnBehalf::CommitsOnBehalf` | | **10** — `setup` · `migrate-corpus` · `rename` · `task discard` · `task finalize` · `milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone finalize` · `milestone discard` |
| ↳ `ActsOnBehalf::MovesOnBehalf` | | **2** — `relocate` · `config set` |
| ↳ `ActsOnBehalf::Neither` | | **36** — **`task amend` is one of them** (its own comment: *"`jigc start`'s class exactly"*) |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:188` | **11** rows — the new one is `"jigc task finalize (amend)"` with `error_code: ERROR_AMEND_REJECTED` |
| `ERROR_CODE_REGISTRY` | `crates/cli/src/invocation_log.rs:268` | **12** — `ERROR_AMEND_REJECTED` = `finalize.amend-rejected` (`:68`) |
| `PostureMember::ALL` | `crates/cli/src/repo.rs:167` | **3** |
| **`InProgress::ALL`** | `crates/cli/src/repo.rs:301` | **10** |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3430` | **6** |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs:2641` | **25** |
| `SLUG_DOORS` | `crates/cli/src/cli.rs:2881` | **6** |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs:2574` | **17** |
| `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs:3189` | **14** |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs:6572` | **66** — the two new rows are `("task amend", "Composed")` and `("task finalize", "LandedAmend")` |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:1049` | **7** |
| `AMBUSH_CONTRACTS` | `crates/cli/src/pack.rs:990` | **6** rows across **three** dispositions — 3 `Owed` · **2 `DeclaredWhereReachable`** (the new third disposition: `finalize.amend-index-dirty`, `finalize.amend-staged-doc`) · 1 `Exempt` |
| `CONSTRAINT_REQUIRED_TOKENS` | `crates/cli/src/pack.rs:1658` | **8** rows (6→8; the two amend rows) |
| `GATE_COVERAGE ▸ Tier::Previewed` | `crates/cli/src/gate_coverage.rs:206` | **5** members; exactly one (`carryover`) carries an `AmendSpelling` |
| `dev/jigc-rig --list-git-states` | | **17** (16 overlay + `unborn` standalone) |

**Axis 2's door set = the 12 acting `BEHALF_DOORS` rows**, plus the two subjects the fan-out mints (each
provisioned sub-task worktree the boundary commits from, and its preview at `jigc task validate <sub-id>`),
**plus the 11th `COMMITTING_DOORS` row** — `jigc task finalize`'s amend arm, which is the same clap leaf
and a different commit construction, so it is a *cell* of `task finalize` rather than a new door. The 36
`Neither` rows are driven as controls; `jigc task amend` is one of them and its silence is the contract
(§4.1).

**The new mechanism, read before it was probed.** `cli::start::amend_in_repo` (`start.rs:397`) gates in
one order — pack → intent → `head_shape_refusal` → mint — so a refused amend strands no task directory;
the area gains one `TASK_AREA_FILES` member, `amend`, holding the pinned sha, written **immediately**
after the mint. `cli::task::git_commit_amend` (`task.rs:7568`) delegates to the identical
`git_commit_capture` seam as `git_commit`, so the posture re-probe, the captured hook stream and the
rejection typing are shared; `StagePolicy::Amend` stages nothing.

---

## 3 · The (door, cell) table — the amend arm (the new code)

Exit / code / route are identical across the cells a repro block groups unless the table says otherwise.

| # | cell | door | argv | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | happy path | `task amend` | `task amend "repair the summary"` | **0** | none | Informational (`amending:` block) | `task minted: …` + `amending: <sha7> "<subject>"` | matches contract |
| 2 | happy path | `task finalize` (amend) | `--format json task finalize repair-the-summary` | **0** | none | none | `committed.amended` = `d421129`, `committed.hash` = `82356d8`, `files` 0, `manifest` `[]`, `promoted` `[]` | matches contract |
| 3 | tree / parent / author | — | `git rev-parse HEAD^{tree}`, `HEAD^`, `%ad`/`%cd` | — | — | — | tree **byte-identical**, parent unchanged, `AD` preserved, `CD` reset | matches contract |
| 4 | reflog | — | `git reflog -3` | — | — | — | `commit (amend)` + the superseded `d421129` reachable | matches contract |
| 5 | posture × **16** git states | `task amend` | `task amend "posture probe"` | **0** ×16 | none | none | mints under every member — `BEHALF_DOORS` `Neither` | matches contract (settle's corrected row) |
| 6 | posture × **16** git states | `task finalize` (amend) | `task finalize posture-probe` | **1** ×16 | `repo.operation-in-progress` ×15, `repo.head-detached` ×1 | Human/Mechanical per member | the member's own noun; **HEAD unmoved in all 16** | matches contract |
| 7 | index dirty — **add** | `task finalize` (amend) | `task finalize index-probe` | **3** | `finalize.amend-index-dirty` | Human (`git -C <abs> restore --staged -- <path>`) | `key {code, "newfile.txt"}`, `location.address` = the path | matches contract |
| 8 | index dirty — **modify** | ″ | ″ | **3** | ″ | ″ | `key {code, "README.md"}` | matches contract |
| 9 | index dirty — **delete** | ″ | ″ | **3** | ″ | ″ | `key {code, "docs/roadmap.md"}` | matches contract |
| 10 | index dirty — **rename** | ″ | ″ | **3** | ″ | ″ | `key {code, "docs/roadmap-moved.md"}` (git's `--name-only` destination) | matches contract |
| 11 | index dirty — preview | `task validate` | `task validate bare-exits` | **3** | ″ | ″ | the identical finding | matches contract |
| 12 | index dirty — forecast | `task finalize --dry-run` | ″ | **3** | ″ | ″ | ″ | matches contract |
| 13 | index dirty — **no consent** | `task finalize --carry-staged` | ″ | **3** | ″ | ″ | refuses identically; help declares the flag inert | matches contract |
| 14 | the route, executed | — | the emitted `git -C … restore --staged -- …` | **0** from (a) (b) `/` | — | — | after it, the same finalize **lands** at exit 0 | matches contract |
| 15 | staged managed doc × **8** `doc` write leaves | `doc create`/`add-item`/`remove-item`/`retitle-item`/`rename`/`set-field`/`set-slot`/`author` | see §3.3 | **1** ×8 | `finalize.amend-staged-doc` ×5, `create.gate-blocked` ×2, `write.identity-change` ×1 | Mechanical | `git status --porcelain` **empty** after every cell | matches contract |
| 16 | `jigc rename` inside an amend task | `rename` | `rename adr:single-node-cache --to "…"` | **1** | `rename.in-flight` | Mechanical | worktree clean — the *already-refused re-slug staged the doc* cell the review found is closed | matches contract |
| 17 | HEAD moved | `task finalize` (amend) | `task finalize moved-head` | **3** | `finalize.base-mismatch` | Mechanical (`task discard … --force`, then `task amend`) | `key {code, "task:moved-head"}`; pinned + current sha both printed | matches contract |
| 18 | HEAD moved — preview | `task validate` | `task validate moved-head` | **0** | **none** | — | `findings: []` | **DEFECT A2-1** |
| 19 | HEAD moved — forecast | `task finalize --dry-run` | ″ | **3** | `finalize.base-mismatch` | Mechanical | the finding on the envelope | **DEFECT A2-1** (the pair) |
| 20 | `amend.head-shape` — **unborn** | `task amend` | `task amend "unborn probe"` | **1** | `amend.head-shape` | Human (`jigc start "<intent>"`) | `at: work-unit:unborn-probe` — the id the mint *would* have taken (LOW-5 closed); `.jigc/tasks` **absent** | matches contract |
| 21 | `amend.head-shape` — **root** | ″ | `task amend "root probe"` | **1** | ″ | Human (`git commit --amend`) | `at: work-unit:root-probe`; nothing minted | matches contract |
| 22 | `amend.head-shape` — **merge** | ″ | `task amend "merge probe"` | **1** | ″ | Human (leave it / redo the merge) | *"a merge commit — it has 2 parents"*; `.jigc/tasks` **empty**; HEAD unmoved | matches contract |
| 23 | `amend.head-shape` — no intent | ″ | `task amend` (unborn) | **1** | ″ | Human | `at: work-unit:amend-4b825dc` — the fallback id, correct per the doc-comment | matches contract |
| 24 | hook-rejected — `pre-commit` | `task finalize` (amend) | `task finalize hook-probe` | **1** | `finalize.amend-rejected` | frame + copy-runnable re-run | `HEAD` **byte-identical**, message identical, tree identical, worktree clean, task live | matches contract |
| 25 | hook-rejected — `commit-msg` | ″ | ″ | **1** | ″ | ″ | byte-identical output to row 24 | matches contract |
| 26 | hook-rejected — the log | — | `.jigc/logs/invocations.jsonl` | — | — | — | `error_code: "finalize.amend-rejected"`, `finding_codes: []` — the 11th `COMMITTING_DOORS` row's own identity, distinct from `finalize.commit-rejected` | matches contract |
| 27 | hook-rejected — envelope | `task finalize --format json` | ″ | **1** | ″ | — | `key {code, "task:hook-probe"}` on the `findings` arm | matches contract |
| 28 | seam race (posture) | `task finalize` (amend) | `PATH=<shim> task finalize amend-reverify` | **1** | `repo.operation-in-progress` | Mechanical (`git bisect reset`) | HEAD unmoved, message identical, task live — the seam's immediate re-probe fires on the amend arm | matches contract, **and inherits `(2, DEFECT C)`** |
| 29 | content gate | `task finalize` (amend) | `task finalize incomplete-doc` | **3** | `schema-conformance.field-value-conformant` + `.required-slot-present` | Mechanical ×2 | HEAD unmoved | matches contract |
| 30 | cwd (b) subdir | `task amend` + `task finalize` | from `$REPO/docs/deep` | 0 / 0 | none | — | lands; route for the dirty cell runs from (a)(b)`/` | matches contract |
| 31 | cwd (d) linked, **attached** | `task amend` | from `$RIG/lw` | **0** | none | Informational | pins the **linked** HEAD `073c29d`; the ack names the checkout and the branch — *"not of the main checkout jigc's workbench binds to"* (LOW-7 built) | matches contract |
| 32 | cwd (d) — forecast | `task finalize --dry-run` | ″ | **0** | none | — | *"would commit in the linked worktree at `<abs>` on branch `lwbr`"* | matches contract |
| 33 | cwd (d) — index subject | `task finalize` (amend) | ″ | 3 then 0 | `finalize.amend-index-dirty` then none | Human | a dirty **linked** index refuses; a dirty **main** index does **not** — the standing worktree's own index is the subject, and main's staged plant survives untouched | matches contract |
| 34 | cwd (c) fan-out, detached | `task amend` | from `$REPO/.jigc/worktrees/aw-one` | **0** | none | Informational | pins `58f6538`; the ack names `.jigc/worktrees/aw-one` **repo-relative** | matches contract |
| 35 | cwd (c) — finalize | `task finalize` (amend) | ″ | **1** | `repo.head-detached` | Human (`git switch <branch>`) | HEAD unmoved | **DEFECT A2-2** (its route) |
| 36 | cwd (c) — preview | `task validate` | ″ | **0** | **none** | — | *no findings — the task validates clean* | **DEFECT A2-2** |
| 37 | spaced repository root | `task finalize` (amend) | under `…/axis2 spaced/…/repo` | **3** | `finalize.amend-index-dirty` | Human | `git -C '<abs with a space>' restore --staged -- 'sp file.txt'` — **both** operands single-quoted; run verbatim rc **0** from (a) (b) `/`; then lands | matches contract |
| 38 | boundary commit at HEAD | `task amend` + finalize | over a landed `milestone finalize` | 0 / 0 | none | Informational | allowed by design; tree identical, record intact, `base:` line intact, `validate` 0, `doc show` 0 | matches contract (the settle's *no discriminator exists* row) |
| 39 | record-only commit at HEAD | ″ | over `milestone create`'s record commit | 0 / 0 | none | ″ | `validate` 0, `milestone add-task` afterwards 0, `list-tasks` intact | matches contract |
| 40 | verb-routed | `start --workflow amend` | `start --workflow amend "x"` | **1** | `workflow.verb-routed` | Mechanical (`jigc task amend`) | names the reason | matches contract |
| 41 | verb-routed | `workflow amend --preview` | ″ | **1** | ″ | ″ | byte-identical | matches contract |
| 42 | `task diff` | `task diff` | `--format json task diff incomplete-doc` | 0 | none | — | keys `[base, code_diff, findings, op, staged_docs, task]` | matches contract |
| 43 | `doc list --task` | `doc list` | `doc list --task incomplete-doc` | 0 | none | — | `commit:incomplete-doc … managed` — one doc | matches contract |
| 44 | `doc show --task` | `doc show` | `doc show commit:<id> --task <id>` | 0 | none | — | serves the task's staged copy | matches contract |
| 45 | `also open:` | `start` | a second mint beside a live amend task | 0 | none | Informational | *"also open: 1 other task was already open before this call"* | matches contract |
| 46 | amend-task discard | `task discard` | `task discard <amend id> --force` | **0** | none | — | HEAD unmoved, area gone | matches contract |
| 47 | second mint | `task amend` | the same intent twice | **1** | `task.serial-collision` | Mechanical | resume-or-discard route | matches contract |
| 48 | resume carries the pin | `start --task` / `workflow amend --task` | both re-compose doors | 0 | none | Informational | `amending:` block present on **both** (MEDIUM-2 closed on the text arm) | matches contract |
| 49 | `--format json` mint | `task amend --format json` | ″ | 0 | none | — | keys `[task, text]`; `text` carries **neither** the pinned sha nor `amending:` — *and the ordinary `start --format json` omits its ack the same way* | matches contract; **the settle row is DEFECT A2-3** |
| 50 | the `left-out` header | `task finalize` (amend) | over a dirty **tree**, clean index | 0 | none | — | *"an amend commits no tree change, so none of it can join"* — MEDIUM-3's misdirection gone | matches contract |

### 3.1 · Repro — the happy path, the envelope, and the bytes that must not move

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
BEFORE: HEAD d421129 "docs(changelog): cut the first release"
        tree ee956c69…  parent 48b60013…  AD=Sun Sep 27 01:38:54 2026  CD=Sun Sep 27 01:38:54 2026
        git status --porcelain -> (empty)   commits=5

argv : jigc task amend "repair the summary"                                    -> exit 0
  task minted: repair-the-summary
  amending: d421129 "docs(changelog): cut the first release"
    `jigc task finalize repair-the-summary` replaces that message and leaves the commit's tree exactly
    as it is — so this repairs the message, never the change.
    If this commit has already been pushed, amending it rewrites shared history — jigc cannot tell
    whether it has.
  area: amend base.json docs intent staged-snapshot.json workflow
  .jigc/tasks/repair-the-summary/amend -> d421129865e99bd3e396953543767efb9d91ef37   (the full sha)

argv : jigc doc set-field commit:repair-the-summary#type --value docs --task …  -> 0
       jigc doc set-slot  commit:repair-the-summary#summary --from-file - …     -> 0
argv : jigc task validate repair-the-summary                                    -> exit 0 (bare)
argv : jigc task finalize repair-the-summary --dry-run                          -> exit 0 (bare)
  would rewrite d421129 "docs(changelog): cut the first release" → "docs: cut the first release, with
    the date corrected"
    the tree and the author are unchanged; the message is re-authored and the committer becomes you, now

argv : jigc --format json task finalize repair-the-summary                      -> exit 0 (bare)
  keys ['committed','findings','schema_version']
  "committed": {"amended":"d421129","displaced":[],"files":0,"hash":"82356d8","hook_output":"",
                "left_out":[],"manifest":[],"promoted":[],
                "subject":"docs: cut the first release, with the date corrected"}
  stderr: 0 bytes

AFTER : HEAD 82356d8   tree ee956c69…  IDENTICAL   parent 48b60013…  UNCHANGED
        AD=…01:38:54 (preserved)   CD=…01:39:35 (reset)   commits=5 (UNCHANGED)
        reflog: 82356d8 HEAD@{0}: commit (amend): …  |  d421129 HEAD@{1}: commit: …
        git status --porcelain -> (empty)      jigc task list -> no active tasks (area torn down)
```

### 3.2 · Repro — `InProgress::ALL` ∪ `{detached}` × the amend arm, one fresh rig per member

```
setup (×16): rig=$(dev/jigc-rig committed-singletons --git-state <S> --binary …) || continue; eval "$rig"
             jigc task amend "posture probe"; author type+summary
argv        : jigc task finalize posture-probe
```

| `--git-state` | mint door | finalize arm | code | noun printed | HEAD |
|---|---|---|---|---|---|
| `merge` | 0 | **1** | `repo.operation-in-progress` | *a merge is in progress* | unmoved |
| `squash-merge` | 0 | **1** | ″ | *a squash merge is staged and not committed* | unmoved |
| `rebase-merge` | 0 | **1** | ″ | *a rebase is in progress* | unmoved |
| `rebase-apply` | 0 | **1** | ″ | *a rebase is in progress* | unmoved |
| `am` | 0 | **1** | ″ | *a `git am` is in progress* | unmoved |
| `cherry-pick` | 0 | **1** | ″ | *a cherry-pick is in progress* | unmoved |
| `sequencer` | 0 | **1** | ″ | *a cherry-pick is in progress* | unmoved |
| `dangling-sequencer` | 0 | **1** | ″ | *a cherry-pick or revert left a queue of commits in `sequencer/`* | unmoved |
| `uncommitted-pick` | 0 | **1** | ″ | *an uncommitted cherry-pick is in progress* | unmoved |
| `uncommitted-pick-range` | 0 | **1** | ″ | ″ | unmoved |
| `uncommitted-pick-conflicted` | 0 | **1** | ″ | ″ | unmoved |
| `uncommitted-pick-resolved` | 0 | **1** | ″ | ″ | unmoved |
| `revert` | 0 | **1** | ″ | *a revert is in progress* | unmoved |
| `unmerged-index` | 0 | **1** | ″ | *a conflict left unmerged paths in the index* | unmoved |
| `bisect` | 0 | **1** | ″ | *a bisect is in progress* | unmoved |
| `detached` | 0 | **1** | `repo.head-detached` | *HEAD is detached* | unmoved |

**Precedence, driven:** five of these states carry a **non-empty index** (`merge`, `squash-merge`,
`cherry-pick`, `uncommitted-pick*`, `unmerged-index`) and every one of them answered with the **posture**,
not `finalize.amend-index-dirty` — the posture guard runs at dispatch, ahead of the arm's own gate. That
is the correct order and it is the order that keeps the cell safe: the door refuses before it can read a
half-merged index as a carry.

### 3.3 · Repro — the `finalize.amend-staged-doc` gate over **all 8** `doc` write leaves

The F-10 review's HIGH-1 derived its class as *8 `Write` doc leaves, one shared copy-on-write seam*, and
drove **3**. All eight are driven here, and the two the review's count left as *"the fix point is one
site"* are answered by **different** producers — which is why driving them was not redundant.

```
setup: rig committed-singletons; jigc task amend "doc leaf sweep"      -> task doc-leaf-sweep
       (a second rig lands a committed non-singleton `adr:single-node-cache` through
        `jigc start --workflow record-decision`, for the two leaves a singleton cannot reach)
BEFORE every cell: git status --porcelain -> (empty)
```

| leaf | argv | exit | code | worktree after |
|---|---|---|---|---|
| `doc create` | `doc create adr --title "A new decision" --task …` | 1 | `create.gate-blocked` (`allowed doctypes: []`) | **empty** |
| `doc add-item` | `doc add-item decisions-log#entries --title "A staged entry" --task …` | 1 | **`finalize.amend-staged-doc`** → `docs/decisions-log.md` | **empty** |
| `doc remove-item` | `doc remove-item roadmap#milestones/m-alpha --task …` | 1 | ″ → `docs/roadmap.md` | **empty** |
| `doc retitle-item` | `doc retitle-item roadmap#milestones/m-alpha --title "M Alpha renamed" --task …` | 1 | ″ → `docs/roadmap.md` | **empty** |
| `doc set-field` | `doc set-field changelog#releases/1-0-0/link --value … --task …` | 1 | ″ → `CHANGELOG.md` | **empty** |
| `doc set-slot` | `doc set-slot vision#thesis --from-file - --task …` | 1 | ″ → `VISION.md` | **empty** |
| `doc rename` (singleton) | `doc rename vision --to "A New Vision" --task …` | 1 | `write.identity-change` | **empty** |
| `doc rename` (**committed non-singleton**) | `doc rename adr:single-node-cache --to "…" --task …` | 1 | **`finalize.amend-staged-doc`** → `docs/decisions/single-node-cache.md` | **empty** |
| `doc author` | `doc author vision --task … --from-file -` (conforming payload) | 1 | `create.gate-blocked` — `author` implies `create`, which the amend workflow's `allows-create: []` refuses first, so the amend gate is **unreachable** through this leaf | **empty** |

The blocking message is one identity at both positions, as its producer's comment requires, and the write
door's route is the `Write` arm (`jigc start "<intent>"` mints an ordinary task), never the finalize arm's
`task discard … --force`:

```
blocking · finalize.amend-staged-doc — `vision:vision` is a managed doc, and it promotes to `VISION.md`
  — but this task's commit model is an amend, which changes no tree: its finalize would write `VISION.md`
  into the worktree and commit none of it, leaving the file diverged from the commit it just rewrote
  at: VISION.md
  route: `jigc start "<intent>"` mints an ordinary task, whose finalize commits the promoted doc; this
         amend task keeps its own job, repairing `HEAD`'s message
exit=1
AFTER the whole nine-cell sweep: git status --porcelain -> (empty)   HEAD f59ad2f (unmoved)
                                jigc doc list --task doc-leaf-sweep -> commit:doc-leaf-sweep   (one doc)
```

### 3.4 · Repro — the dirty-index gate, four index shapes, and the route executed

```
setup (×4): rig committed-singletons; jigc task amend "index probe"; author type+summary
            add    : printf 'new\n' > newfile.txt; git add newfile.txt
            modify : printf 'more\n' >> README.md; git add -A
            delete : git rm -q --cached docs/roadmap.md
            rename : git mv docs/roadmap.md docs/roadmap-moved.md
BEFORE: H0=git rev-parse HEAD ; T0=git rev-parse HEAD^{tree}

argv/exit (ALL MEASURED BARE, output to /dev/null):
  jigc task validate index-probe                -> 3
  jigc task finalize index-probe --dry-run      -> 3
  jigc task finalize index-probe                -> 3
  jigc task finalize index-probe --carry-staged -> 3          <- the flag is accepted and refuses anyway
  jigc --format json task validate index-probe  -> 3
  jigc --format json task finalize … --dry-run  -> 3

observed (the `add` cell; the other three differ only in the path):
  blocking · finalize.amend-index-dirty — `newfile.txt` is staged, and an amend rewrites `HEAD` from the
    index — finalizing now would fold it into the commit whose message this task is repairing, a change
    that commit never carried
    at: newfile.txt
    route: unstage it (`git -C <ABS repo> restore --staged -- newfile.txt`) and re-run the finalize —
           this arm takes no `--carry-staged`, because an amend that carried anything would change a tree
           it promised not to touch
  JSON: findings[0].key = {"code":"finalize.amend-index-dirty","target":"newfile.txt"}
        findings[0].location.address = "newfile.txt"
AFTER each cell: HEAD unmoved ; tree unmoved

THE EMITTED ROUTE, RUN VERBATIM:
  cwd=$REPO             -> rc=0
  cwd=$REPO/docs/deep   -> rc=0
  cwd=/                 -> rc=0
then: jigc task finalize bare-exits -> exit 0
  finalize — about to rewrite HEAD's message; the committed tree does not move, so it leaves out:
    left-out (unstaged/untracked — an amend commits no tree change, so none of it can join):
      newfile.txt
  amended 44a0fa5 → 2b54b61 — fix: measured bare
    the tree and the author are unchanged; the message is re-authored and the committer becomes you, now
    the superseded commit stays reachable in the reflog
```

The **rename** cell needs its own note: git's `diff --cached --name-only` reports only the destination, so
the first round of the route unstages `docs/roadmap-moved.md` and the gate **re-fires** on the source. Two
rounds, no hole — the same observation the F-10 review recorded, re-driven.

### 3.5 · Repro — the hook-rejected cell, both hooks, and the log identity

```
setup (×2): rig committed-singletons; jigc config set invocation-log true
            HD=$RIG/hooks; printf '#!/bin/sh\necho "the hook says no" >&2\nexit 1\n' > $HD/<pre-commit|commit-msg>
            git config core.hooksPath $HD
            jigc task amend "hook probe"; author type+summary
BEFORE: H0=git rev-parse HEAD ; M0=md5(git log -1 --format=%B) ; T0=git rev-parse HEAD^{tree}

argv : jigc task finalize hook-probe                    -> exit 1 (bare)
  `git commit` was rejected (no commit was made):
  the hook says no

  `HEAD` is unchanged — the commit this task is repairing still carries the message it had, and task
  hook-probe's authored commit doc is still in `.jigc/tasks/hook-probe/docs/`. Fix the hook's complaint,
  then re-run `jigc task finalize hook-probe`.

AFTER: HEAD BYTE-IDENTICAL · message identical · tree identical · git status --porcelain EMPTY ·
       jigc task list -> 1 active task(s)
BYTE-IDENTICAL output for the `commit-msg` hook.

argv : jigc --format json task finalize hook-probe       -> exit 1
  findings[0].code  "finalize.amend-rejected"
  findings[0].key   {"code":"finalize.amend-rejected","target":"task:hook-probe"}
  findings[0].route the state-truth clause + `jigc task finalize hook-probe`

the invocation log (invocation-log = true):
  {'argv':['task','amend','logged probe two'],   'exit_code':0,'finding_codes':[],'error_code':None,'binary_version':'1.0.0-rc.20'}
  {'argv':['task','finalize','logged-probe-two'],'exit_code':1,'finding_codes':[],'error_code':'finalize.amend-rejected','binary_version':'1.0.0-rc.20'}
  {'argv':['task','finalize','logged-probe-two'],'exit_code':0,'finding_codes':[],'error_code':None,'binary_version':'1.0.0-rc.20'}   <- after the hook was removed
```

The **11th `COMMITTING_DOORS` row is therefore confirmed behaviourally**: one leaf, two commit models, two
error identities, and the log discriminates them.

### 3.6 · Repro — the seam's immediate posture re-probe on the amend arm

```
setup: rig committed-singletons; jigc task amend "amend reverify"; author type+summary
       a deterministic `git` shim first on PATH that fires `git -C $REPO bisect start`+`bad` ONCE on the
       finalize transaction's `status --porcelain` probe, then execs /usr/bin/git
argv : PATH="$RIG/shim:$PATH" jigc task finalize amend-reverify
observed: exit=1
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
    committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
  shim log: "shim fired on: status --porcelain --untracked-files=all"
AFTER: HEAD unmoved · message identical · BISECT_LOG present · task LIVE
```

`git_commit_amend` shares `git_commit_capture`, and the shared re-probe fires — so the amend arm is inside
`(2, DEFECT C)`'s axis rather than outside it (§4.4): the refusal prints **one code and one route and
nothing else**, no state-truth clause, no copy-runnable re-run, while the hook arm one step away prints
both. That widens DEFECT C's door axis by one; it is a **measurement on an already-filed row**, not a new
finding.

**A narrower shim, fired on the `--amend` invocation itself** (i.e. *after* the re-probe), landed the amend
at exit 0 with HEAD moved. That is a genuine race window, not a gap: M52's own record declares *the hook is
the design's named racer*, and a bisect begun inside the same `git` process that performs the commit is
outside any predicate jigc can ask. Recorded, not filed.

---

## 4 · The rc.19 / M52 baseline rows — CLOSED / STILL-OPEN

**Every baseline row, all tiers.** `STILL-OPEN(1.x, expected)` marks a row the M53 charter triaged into
the 1.x ledger rather than fixing — a *measurement*, not a finding.

| baseline row | tier | verdict on rc.20 | the argv / the datum |
|---|---|---|---|
| **`(2, N-1)`** (rc.19) — the pre-commit hook announces an out-of-band **rename** on a commit with none, and calls a staged change *"not staged in this commit"* | 3 | **CLOSED** | §4.1 — `git rm -q VISION.md` then `git commit`: rc **0** and the hook prints **nothing**. rc.19's sentence is gone. Before-control (an ordinary commit) also silent |
| **`(2, N-2)`** (rc.19) — the hook's *blocking* rename backstop is structurally unreachable for the entire **placement** family | 3 | **CLOSED** | §4.2 — `git mv` of **all four** placement singletons now blocks: rc **1** ×4 with *"out-of-band managed-doc rename staged in this commit … (commit blocked)"*. The location-doctype control is byte-identical |
| **`(2, F-1)`** (rc.18) — the aimed route `git -C <repo-relative>` does not run from the checkout that printed it | 2 | **still CLOSED** | §4.3 — both `aim_at` callers × **4 doors** (`milestone finalize` 3 · `task validate <sub>` 3 · `milestone discard` 1 · `uninstall` 1) × cwds (a)(b)(d), one byte-identical span; run verbatim rc **0** from **five** cwds incl. `/` and the fan-out worktree; and rc **0** under a spaced root where `shell_operand` single-quotes it |
| **`(2, DEFECT A)`** (M52) — a clean `git cherry-pick --no-commit` is a member of no `InProgress` row | **1** | **still CLOSED** | §3.2 + §4.5 — all four `uncommitted-pick*` states answer `UncommittedCherryPick` at exit 1 across **8** doors + the amend arm; the discriminating cell (`uncommitted-pick-conflicted`, index genuinely conflicted) still answers the *pick*, not `UnmergedIndex`. No consent flag bypasses |
| **`(2, DEFECT 1)`** (rc.17) — the boundary commits a provisioned worktree's un-concluded operation at exit 0 | **1** | **still CLOSED** | §4.3 — the worktree subject refuses at `bisect` from (a)(b)(d) at both producers, the refusal placed **before** the record flip; main HEAD unmoved |
| M51 **`D1`** · **`D2`** | — | **still CLOSED** | §3.2 + §4.5 — `rebase-merge`/`rebase-apply` are HEAD-detached in git and still answer *a rebase*; only the bare `detached` state reaches `repo.head-detached`. `am` and apply-backend rebase keep two nouns and two abort commands |
| **`(2, DEFECT C)`** (M52) — a posture breach at the **commit seam** prints no state-truth clause and no copy-runnable re-run | 2 | **STILL-OPEN(1.x, expected)** — and its door axis is now **wider by one** | §4.4 — reproduces byte-for-byte on the ordinary arm with the shim, **and** on the amend arm (§3.6). The contrast arm (a rejecting `pre-commit`) prints the full clause **and** `jigc task finalize seam-probe` |
| **`(2, DEFECT B)`** (M52) — a conflicted `git merge --squash` is answered by `SquashMerge`, whose predicate is false of the state | 3 | **STILL-OPEN(1.x, expected)** | §4.5 — markers `MERGE_MSG` + `SQUASH_MSG`, `git ls-files -u` → **3** unmerged; `jigc milestone create` → exit 1, *"a squash merge is **staged and not committed**"* |
| **`(2, F-2)`** (rc.18) — a foreign repository's worktree parked at `.jigc/worktrees/<sub-id>` is committed as the sub-task's own work | 3 | **STILL-OPEN(1.x, expected)** | §4.6 — `join` 0, `finalize` 0, *"added bsecret.txt … sub-tasks: fw-area: 1 code file"*; `git cat-file -p HEAD:bsecret.txt` → `B-SECRET-PAYLOAD`. **No byte lost** — B's untracked plant intact, B's 2 worktree registrations intact, `.jigc/displaced/` absent |
| **`(2, N-3)`** (rc.19) — `milestone provision` acks *"at base &lt;pin&gt;"* over a worktree it **reused** at a different commit | 3 | **STILL-OPEN(1.x, expected)** | §4.7 — pin `825e612`, parked worktree HEAD `7226809`; `provision` → exit 0, *"at base 825e612"*; worktree HEAD **unchanged**; the boundary then repeats *"the sub-task worktrees were cut from 825e612…"* at exit 3 |
| **`(2, N-4)`** (rc.19, tier corrected 2→3 by the reconciler) — a promoting `task finalize` from a linked worktree leaves the main checkout with a finding naming an out-of-band edit that never happened | 3 | **STILL-OPEN(1.x, expected), exactly as corrected** | §4.8 — the doc lands on `lwbr`; main's `jigc validate` prints `blocking (gates at finalize) · file-state.hash-matches` with the route *"re-author it through the owning workflow"*; and the **escalation leg still does not fire** — main's next `task finalize` **absorbs** it (`advisory · reconciliation.absorb`, *no action needed*) and lands at exit 0 |

### 4.1 · `(2, N-1)` — CLOSED

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
BEFORE-CONTROL: printf 'ordinary\n' > ord.txt; git add ord.txt; git commit -m "probe: ordinary"
  -> rc=0, hook prints NOTHING (no `jigc:` line)
argv : git rm -q VISION.md ; git status --porcelain -> "D  VISION.md"   (a DELETION; no rename anywhere)
       git commit -m "probe: delete a managed singleton"
observed: rc=0, NO `jigc:` line at all
  [main 1d98038] probe: delete a managed singleton
(on rc.19 this printed: "jigc: an out-of-band managed-doc rename exists in the committed tree … (not
 staged in this commit; commit not blocked).")
```

### 4.2 · `(2, N-2)` — CLOSED at all four placement singletons

```
setup (×4, one fresh rig each): rig committed-singletons
argv : git mv <A> <B> ; git status --porcelain ; git commit -m "probe"   (rc measured bare)

  docs/roadmap.md       -> docs/roadmap-oob.md     | staged: R  … | rc=1  BLOCKED
  VISION.md             -> VISION-OOB.md           | staged: R  … | rc=1  BLOCKED
  CHANGELOG.md          -> CHANGELOG-OOB.md        | staged: R  … | rc=1  BLOCKED
  docs/decisions-log.md -> docs/dl-oob.md          | staged: R  … | rc=1  BLOCKED
each printing: "jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses
                jigc identity tracking; use `jigc rename` instead (commit blocked)."
CONTROL, a LOCATION doctype (docs/milestone-records/loc-wave.md -> …/zz-oob.md): rc=1, byte-identical line
(on rc.19 all four placement rows were rc=0 with the "not staged in this commit" sentence)
```

### 4.3 · `(2, F-1)` + `(2, DEFECT 1)` — still CLOSED

```
setup: rig committed-singletons; milestone cwd-wave + sub-task cw-area provisioned
       W=$REPO/.jigc/worktrees/cw-area; git -C $W bisect start; git -C $W bisect bad
       mkdir -p $REPO/docs/deep; git worktree add -q $RIG/linked -b linkedbr
BEFORE: 4 BISECT_* markers in .git/worktrees/cw-area ; git -C $REPO status --porcelain -> (empty)

door × cwd → exit, and the span as emitted (one string, byte-identical in all 12 cells):
  milestone finalize cwd-wave   (a)(b)(d) -> 3
  task validate cw-area         (a)(b)(d) -> 3
  milestone discard cwd-wave    (a)(b)(d) -> 1
  uninstall                     (a)(b)(d) -> 1
  span: git -C <ABS>/.jigc/worktrees/cw-area bisect reset

THE EMITTED COMMAND, RUN VERBATIM:
  cwd=$REPO            -> rc=0
  cwd=$REPO/docs/deep  -> rc=0
  cwd=$W               -> rc=0
  cwd=$RIG/linked      -> rc=0
  cwd=/                -> rc=0
AFTER the whole sweep: main HEAD unmoved, the milestone still finalizable
```

### 4.4 · `(2, DEFECT C)` — reproduces byte-for-byte, with its contrast arm

```
setup: rig=$(dev/jigc-rig committed-singletons --start quick-fix "seam probe" --binary …) || exit; eval "$rig"
       author type+summary; `jigc task validate seam-probe` -> exit 0 (bare); git add work.txt
       a deterministic `git` shim first on PATH that fires `git -C $REPO bisect start`+`bad` ONCE on the
       first `git add` of the run, then execs /usr/bin/git
argv : PATH="$RIG/shim:$PATH" jigc task finalize seam-probe     -> exit 1 (bare)
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
    committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
  shim log: "shim: fired [bisect start]"
STATE AFTER, all true and NONE of it stated by that surface:
  HEAD 66a0ece (unmoved) · work.txt still staged · .jigc/tasks/seam-probe/docs/ intact
  (commit:seam-probe.md + provenance.json) · BISECT_LOG present

CONTRAST — same door, same seam, a rejecting pre-commit hook instead:
  `git commit` was rejected (no commit was made):
  the hook says no

  task seam-probe is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/seam-probe/docs/`, and anything you had `git add`-ed is still in git's index.
  Fix the hook's complaint, then re-run `jigc task finalize seam-probe`.
```

### 4.5 · `(2, DEFECT A)` and `(2, DEFECT B)` — the 16 × 8 acting-door sweep

Every acting `BEHALF_DOORS` row is covered. **16 members × 8 doors**, one fresh rig per member:

| `--git-state` | `milestone create` | `migrate-corpus` | `config set docs-root` | `setup` | `relocate` | `task discard` | `rename` | noun |
|---|---|---|---|---|---|---|---|---|
| `merge` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a merge is in progress* |
| `squash-merge` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a squash merge is staged and not committed* |
| `rebase-merge` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a rebase is in progress* |
| `rebase-apply` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a rebase is in progress* |
| `am` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a `git am` is in progress* |
| `cherry-pick` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a cherry-pick is in progress* |
| `sequencer` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a cherry-pick is in progress* |
| `dangling-sequencer` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *… left a queue of commits in `sequencer/`* |
| `uncommitted-pick` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *an uncommitted cherry-pick is in progress* |
| `uncommitted-pick-range` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | ″ |
| `uncommitted-pick-conflicted` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | ″ |
| `uncommitted-pick-resolved` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | ″ |
| `revert` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a revert is in progress* |
| `unmerged-index` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a conflict left unmerged paths in the index* |
| `bisect` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | *a bisect is in progress* |
| `detached` | 1 | 1 | **0** ✔ | 1 | `relocate.frozen-doctype` ✔ | 1 | 1 | *HEAD is detached* |

Every non-`detached` cell's code is `repo.operation-in-progress`; every `detached` cell's is
`repo.head-detached`. The two ✔ cells in the `detached` row are the **declared mover rule**
(`repo.rs`: *a mover refuses only `PostureMember::OperationInProgress`*), and both movers **do** refuse
every operation row above.

**The remaining four acting doors**, at `bisect`, one rig, with a live plain task and a provisioned
milestone — plus the `Neither` controls and the no-override cross:

```
setup: rig committed-singletons; milestone remaining-wave + sub-task rm-one provisioned;
       jigc start "a live plain task" --workflow quick-fix; author; git add w.txt
       git bisect start; git bisect bad      (in the MAIN checkout)
argv/exit (bare):
  jigc milestone add-task remaining-wave 'rm two'      -> 1  repo.operation-in-progress
  jigc milestone add-from-spec remaining-wave spec:nope -> 1  repo.operation-in-progress
  jigc milestone finalize remaining-wave               -> 1  repo.operation-in-progress
  jigc milestone discard  remaining-wave               -> 1  repo.operation-in-progress
  jigc task finalize a-live-plain-task                 -> 1  repo.operation-in-progress
  jigc task validate a-live-plain-task                 -> 1  repo.operation-in-progress
CONTROLS (`Neither` rows — correctly silent):
  jigc milestone provision remaining-wave              -> 0
  jigc milestone join      remaining-wave              -> 0
  jigc milestone execute   remaining-wave              -> 0
NO-OVERRIDE CROSS:
  jigc task finalize a-live-plain-task --carry-staged   -> 1  repo.operation-in-progress
  jigc task finalize a-live-plain-task --dry-run        -> 1  repo.operation-in-progress
  jigc milestone discard remaining-wave --force        -> 1  repo.operation-in-progress
  jigc task discard a-live-plain-task --force          -> 1  repo.operation-in-progress
  jigc milestone finalize remaining-wave --format json -> 1  repo.operation-in-progress
AFTER: HEAD 6a183cf (unmoved) ; w.txt still staged
```

`(2, DEFECT B)`'s own repro:

```
setup: rig committed-singletons; branch `cb` and `main` both touch conf.txt; git merge --squash cb -> rc=1
state: markers -> MERGE_MSG, SQUASH_MSG · git ls-files -u | wc -l -> 3   <- the index IS conflicted
       git diff --cached --name-only -> conf.txt
argv : jigc milestone create "DefB probe"  -> exit 1 (bare)
  blocking · repo.operation-in-progress — a squash merge is staged and not committed — …
    route: conclude it, or abandon it with `git reset --merge` (which also discards anything else you had
           staged, from the index and from your working tree), then re-run this command
```

### 4.6 · `(2, F-2)` — reproduces exactly, no byte lost

```
setup: rig committed-singletons; repo B = a SECOND repository (mktemp -d, 2 commits)
       milestone foreign-wave + sub-task fw-area, NO `jigc milestone provision`:
       git -C $B worktree add -q "$REPO/.jigc/worktrees/fw-area" -b fwbranch
       printf 'B-SECRET-PAYLOAD\n' > $W/bsecret.txt; git -C $W add bsecret.txt
       printf 'B-UNTRACKED-KEEP\n'  > $W/keepme.txt
BEFORE: git -C $W rev-parse --show-toplevel -> the squatted path itself
        git -C $W symbolic-ref -q HEAD      -> refs/heads/fwbranch      (B's branch)
argv : jigc milestone join foreign-wave      -> exit 0
       jigc milestone finalize foreign-wave  -> exit 0
  finalized 2a2f7bb — Finalize milestone foreign-wave (1 sub-task)
    added bsecret.txt · modified docs/milestone-records/foreign-wave.md · 2 files committed
    sub-tasks: fw-area: 1 code file            <- B's bytes, acked as the sub-task's own work
AFTER: git -C $REPO cat-file -p HEAD:bsecret.txt -> B-SECRET-PAYLOAD
       $W/keepme.txt -> B-UNTRACKED-KEEP (intact) · B worktree registrations: 2 (intact) ·
       .jigc/displaced/ -> absent
```

### 4.7 · `(2, N-3)` — reproduces exactly

```
setup: rig committed-singletons
       jigc milestone create "Reuse wave"  -> "minted milestone:reuse-wave (shared base 825e612)"
       jigc milestone add-task reuse-wave "ru one"
       printf 'x\n' > extra.txt; git add extra.txt; git -c core.hooksPath=/dev/null commit -m "chore: advance"
       git worktree add -q --detach "$REPO/.jigc/worktrees/ru-one"
BEFORE: milestone pin = 825e612 · the parked worktree's HEAD = 7226809
argv : jigc milestone provision reuse-wave                        -> exit 0 (bare)
  provisioned 1 worktree(s) for milestone:reuse-wave at base 825e612 (ru-one)
AFTER : git -C $REPO/.jigc/worktrees/ru-one rev-parse --short HEAD -> 7226809    <- NOT 825e612
then  : jigc milestone finalize reuse-wave                        -> exit 3 (bare)
  blocking · finalize.base-mismatch — the milestone was pinned to base `825e612…` but HEAD is now
    `7226809…`, and the commits landed since move more than milestone-record bookkeeping — the sub-task
    worktrees were cut from `825e612…`, so combining them onto HEAD cannot be proven sound
```

Two law-1 claims the binary cannot support, unchanged from rc.19: the ack's *"at base 825e612"* and the
refusal's *"the sub-task worktrees were cut from 825e612"* — that worktree was cut from `7226809`.

### 4.8 · `(2, N-4)` — reproduces as the reconciler corrected it (tier 3)

```
setup: rig committed-singletons; git worktree add -q $RIG/lw -b lwbr; cd $RIG/lw
       jigc start "record the linked decision" --workflow decided-task
       jigc doc add-item decisions-log#entries --title "Linked branch decision" --task …
         -> "(copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)"
       slots + commit doc authored
BEFORE: md5 $REPO/docs/decisions-log.md == md5 $RIG/lw/docs/decisions-log.md == 41fd3323a5b7566764cb350ceeaa6130
argv : jigc task finalize record-the-linked-decision            -> exit 0
AFTER : main HEAD b3c258e [main] (UNMOVED)   ·   linked HEAD 1b1efd7 [lwbr]
        grep -c 'Linked branch decision' $REPO/docs/decisions-log.md  -> 0
        grep -c 'Linked branch decision' $RIG/lw/docs/decisions-log.md -> 1
then  : (cd $REPO && jigc validate)                              -> exit 0 (store scope), and prints:
  blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/decisions-log.md`
    differs from the recorded state
    at: docs/decisions-log.md
    route: review the out-of-band edit to `docs/decisions-log.md` and re-author it through the owning workflow
THE ESCALATION LEG STILL DOES NOT FIRE — main's next task finalize:
  advisory · reconciliation.absorb — external edit absorbed: `docs/decisions-log.md`
    route: no action needed — the external edit was absorbed into the baseline
  finalized 27baba5 — fix: the next task in main · 1 file committed · exit 0
```

---

## 5 · The `AMBUSH_CONTRACTS` third disposition, driven

The charter names this as a probe target. `AMBUSH_CONTRACTS` is **6 rows / 3 dispositions**: `Owed` (3) ·
**`DeclaredWhereReachable { reason, declarer }`** (2, both F-10's) · `Exempt` (1). The two sets it feeds
are `ambush_class_codes()` (the **owe-set** — `Owed` ∪ `DECLARED_CONTRACT_IDENTIFIERS`) and
`fenced_contract_codes()` (the **prose fence** — `Owed` ∪ `DeclaredWhereReachable` ∪ … ), and
`CONSTRAINT_REQUIRED_TOKENS` (8 rows) bijects against the second.

Driven with a throwaway pack that **keeps** its freeze manifest (`dev/jigc-rig fresh --repin`), one mutant
at a time, restored between:

```
setup: rig=$(dev/jigc-rig fresh --repin --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       S=$JIGC_PACK_DIR/steps/amend-message.yaml   (its front matter:
         states-constraints: [read.staged-read-back, finalize.amend-index-dirty, finalize.amend-staged-doc])
five doors asked per cell: describe · validate · start · task amend "x" · doc schema commit

CONTROL                                                          -> 0 0 0 0 0
MUTANT 1  delete "there is no flag that declares it deliberate    -> 1 1 1 1 1   REDDENS (named-fact tier,
          here."  (the `no flag` fact of finalize.amend-index-dirty)              finalize.amend-index-dirty)
MUTANT 2  "promotes to a file in the repository" -> "lives in     -> 1 1 1 1 1   REDDENS (named-fact tier,
          the repository"  (the `promotes` fact of …staged-doc)                   finalize.amend-staged-doc)
MUTANT 3  drop `finalize.amend-index-dirty` from states-constraints-> 0 0 0 0 0   does NOT redden
RESTORED                                                         -> 0 0 0 0 0
```

**MEDIUM-4's fix holds where it is claimed:** both named facts are now bought, at every door, and before
the fix the review measured that deleting the dirty-index paragraph reddened nothing.

**Mutant 3 is the disposition's declared residual, not a defect.** `pack.rs:1099`'s
`ambush_class_codes()` filters `matches!(row.disposition, AmbushDisposition::Owed)`, and
`fenced_contract_codes()`'s doc-comment states the consequence in so many words: *"An
`AmbushDisposition::DeclaredWhereReachable` code is in **this one only**: no pack owes it, and the pack
that does declare it must state it."* `design/surface-contract.md:118` and `:143` say the same. So the
binary does exactly what its own two homes state, and the residual — **a step can drop the declaration
and both fences fall silent together** — is a named consequence of holding the row out of the owe-set so
that a methodology-alone composition does not redden. Recorded as an observation in §8, deliberately not
filed: filing it would be filing the decision, not a divergence.

A second control worth stating because it changes what a `--pack-from-dev` probe can conclude:
**`--pack-from-dev` drops the freeze manifest, and with it the named-fact tier goes vacuous** — mutants 1
and 2 both exited **0** at all five doors on a `--pack-from-dev` rig, and **1** on the `--repin` rig. The
tier's own doc-comment says it keeps the *manifest* subject, so this is the documented behaviour; I record
it because a reviewer reaching for `--pack-from-dev` here would measure a false green.

---

## 6 · Findings

### A2-1 · **TIER 3** — `jigc task finalize --dry-run`'s help says *"Its `findings` are the set `jigc task validate <id>` reports"*, and **two** of the gates it refuses on are outside that set — one of them a gate the same paragraph lists, the other the amend arm's new `finalize.base-mismatch`

> **Class, derived and driven before it was written up.** I found this on the amend arm's moved-HEAD cell
> and assumed it was F-10's. It is not: driving the ordinary arm's **empty-commit** gate — which the same
> sentence's own list names — shows the parity claim was already false before F-10, so the axis is
> ***the set `--dry-run` refuses on vs the set `task validate` reports***, and the amend arm contributes
> the second member, not the first. Both are driven below.

The help text, quoted from the binary (`jigc task finalize --help`, the `--dry-run` arg):

> It *refuses* on **three gates**: this task's validation findings, the empty-commit guard, and the
> carryover gate … On an **amend** task the last two read differently … **Its `findings` are the set
> `jigc task validate <id>` reports**, the staging-independent `owner-artifact` causes included —
> reported here, decided at the real finalize.

Driven, on the amend arm with HEAD moved by jigc's own `milestone create` record commit:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       jigc task amend "moved head"; author type+summary
       jigc milestone create "Move head wave"      -> exit 0, HEAD b96b324 -> 1cf103e

argv : jigc task validate moved-head                       -> exit 0   (BARE)
  no findings — the task validates clean
argv : jigc --format json task validate moved-head         -> exit 0   (BARE)
  {"schema_version": 3, "findings": []}

argv : jigc task finalize moved-head --dry-run             -> exit 3   (BARE)
  blocking · finalize.base-mismatch — `HEAD` is no longer the commit this amend was minted against — it
    pinned `b96b324b…` and `HEAD` is now `1cf103e4…`, so the message this task authored would rewrite a
    different commit
    at: task:moved-head
    route: start the repair again against the commit that is there now (`jigc task discard moved-head
           --force`, then `jigc task amend`), or return `HEAD` to `b96b324b…` first
argv : jigc --format json task finalize moved-head --dry-run -> exit 3
  findings[0].key = {"code":"finalize.base-mismatch","target":"task:moved-head"}

argv : jigc task finalize moved-head                       -> exit 3, the identical finding
AFTER: HEAD 1cf103e (unmoved by any of the above)
```

Both halves of the claim are false of this cell: `--dry-run` refuses on a **fourth** gate the sentence
does not list, and its `findings` are a **strict superset** of `task validate`'s (`[base-mismatch]` vs
`[]`). The gap is one-sided and the sentence is the one that promises parity.

**Control 1 — the ordinary arm does not reach `base-mismatch`.** A plain `quick-fix` task with HEAD
advanced out of band finalized at exit **0**:

```
setup: rig committed-singletons --start quick-fix "ordinary base probe"; author; git add work.txt
       an out-of-band `git -c core.hooksPath=/dev/null commit` advancing HEAD
argv : jigc task validate ordinary-base-probe        -> 0
       jigc task finalize ordinary-base-probe --dry-run -> 0   ("would commit — fix: an ordinary control")
       jigc task finalize ordinary-base-probe        -> 0   (it LANDS)
```

**Control 2 — and this is the one that widens the class: the parity sentence is ALREADY false on the
ordinary arm, over a gate the same paragraph lists.** The empty-commit guard:

```
setup: rig=$(dev/jigc-rig committed-singletons --start quick-fix "empty commit probe" --binary …) || exit
       eval "$rig"; author type+summary; stage NOTHING
BEFORE: git diff --cached --name-only -> (empty) ; git status --porcelain -> (empty)

argv : jigc task validate empty-commit-probe                  -> exit 0   (BARE)
  no findings — the task validates clean
argv : jigc --format json task validate empty-commit-probe    -> {"schema_version":3,"findings":[]}

argv : jigc task finalize empty-commit-probe --dry-run        -> exit 3   (BARE)
  blocking · finalize.empty-commit — task validated but produced no diff — nothing to finalize
    at: task:empty-commit-probe
    route: make a change, then re-run `jigc task finalize empty-commit-probe` — or, if the task is done
           with nothing to show, abandon it with `jigc task discard empty-commit-probe --force`
argv : jigc --format json task finalize … --dry-run           -> exit 3
  findings[0].key = {"code":"finalize.empty-commit","target":"task:empty-commit-probe"}
```

So the sentence contradicts its own paragraph three lines above it: *"It refuses on three gates: this
task's validation findings, **the empty-commit guard**, and the carryover gate"* and then *"Its `findings`
are the set `jigc task validate <id>` reports"*. Of the four gates the forecast actually refuses on, two
(**`finalize.empty-commit`** on the ordinary arm, **`finalize.base-mismatch`** on the amend arm) are
outside `task validate`'s set; two (validation findings, the carryover / empty-index member) are inside it
and I drove both (§3, rows 11–12; §4.5's no-override cross).

So the divergence arrives with the amend arm, which is the arm that made `finalize.base-mismatch` reachable
at a task door.

**The preview's silence is a decision, not an oversight, and the code says so** — read *after* the cell was
driven, `crates/cli/src/task.rs:2644`:

> The amend arm's other refusal, `finalize.base-mismatch`, deliberately does **not** preview: the base pin
> is finalize-only by the M47 Settle, Decision 1, and the amend form is that same member (the pin *is* the
> commit it rewrites).

`GATE_COVERAGE` carries no `base-mismatch` row at any tier, and the composed `what's-left:` line names five
members of which this is not one. So `task validate` is behaving exactly as decided, documented and
enumerated. **The single thing that is false is the `--dry-run` arg help** — its *three gates* count, and
its sentence *"Its `findings` are the set `jigc task validate <id>` reports"*. This is a pure law-1 row
against one help text, and the correction is to that text, not to either door.

**Why not tier 2:** the finalize refuses, HEAD is unmoved, and the refusal's own route is Mechanical and
runnable (`jigc task discard <id> --force`). Nothing is a dead end; a surface says something the binary
does not do.

**Why not tier 1:** nothing lands and no byte moves in any of the four invocations.

**Smallest correction, for the triage's convenience:** the parity sentence is the thing to strike or
qualify — the honest statement is *"its `findings` are `jigc task validate <id>`'s set **plus** the
finalize-only gates the forecast itself reaches"*, and the *three gates* count needs the amend arm's
fourth. (The alternative — giving `empty-commit` and `base-mismatch` `GATE_COVERAGE` rows so the two doors
agree — would reverse two recorded decisions, which is a wave's call and not a help-text fix.) **The axis
the fix is owed over is the whole set `--dry-run` refuses on**, not either cell: I drove four gates and
two of them fall outside, so a fix that names only `base-mismatch` would ship the same incomplete sweep
one member over.

---

### A2-2 · **TIER 2** — *outside the F-10 arc and outside the usability batch* — inside a fan-out worktree, `jigc task validate` reports *validates clean* for a task whose `jigc task finalize` refuses `repo.head-detached`, and that refusal's route exits **128** from the checkout it was printed in

Two halves, one cell. The preview/door split first:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       jigc milestone create "Preview wave"; jigc milestone add-task preview-wave "pw one"
       jigc milestone provision preview-wave
       W=$REPO/.jigc/worktrees/pw-one ;  git -C $W symbolic-ref -q HEAD -> (none: DETACHED)
       cd $W

CELL B — an AMEND task minted there:
  jigc task amend "fanout preview"                  -> 0  (ack names `.jigc/worktrees/pw-one`)
  author type+summary
  jigc task validate fanout-preview                 -> exit 0   (BARE)
    no findings — the task validates clean
  jigc --format json task validate fanout-preview   -> exit 0 : {"schema_version": 3, "findings": []}
  jigc task finalize fanout-preview                 -> exit 1   (BARE)
    blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no branch, and
      the next checkout would leave it unreachable
      route: re-attach HEAD with `git switch <branch>`, then re-run this command

CELL C — an ORDINARY (quick-fix) task minted there, same worktree, so the row is NOT amend-specific:
  jigc start "plain in fanout" --workflow quick-fix  -> 0 ; author ; git add p.txt
  jigc task validate plain-in-fanout                 -> exit 0  "no findings — the task validates clean"
  jigc task finalize plain-in-fanout                 -> exit 1  repo.head-detached

CONTROLS — from an ordinary DETACHED LINKED worktree (rc.19 §3.5's shape), BOTH doors refuse:
  git worktree add -q --detach $RIG/det ; cd $RIG/det
  ordinary task : jigc task validate -> 1 repo.head-detached | jigc task finalize -> 1 repo.head-detached
  amend task    : jigc task validate -> 1 repo.head-detached | jigc task finalize -> 1 repo.head-detached
AFTER every cell: main HEAD unmoved, worktree HEAD unmoved
```

The two controls localize it: the divergence is the **`HeadDetached` × registered-fan-out-worktree** cell
and nothing else. The mechanism is visible in the code and consistent with what I drove:
`cli::finalize_posture_refusal` → `posture_breach_in` → `repo::adjudicated_breach` resolves
`repo::posture_subject(repo_root)`, whose three legs (`.git` is a file · the path is
`engine::milestone::worktree_path(<name>)` · `owning_milestone` answers) return `Checkout::Dedicated`,
which by `PostureSubject::adjudicates` is **exempt from `HeadDetached` and from that member only**. Both
the dispatch guard and `task validate` therefore stay silent. The refusal at `task finalize` comes from
the commit seam's own subject, which is built `live` at the standing checkout
(`milestone.rs:5166`'s comment says so in as many words: *"`SeamSubject::live(<the worktree>)`, which
refuses `repo.head-detached`"*). Two subjects, one cwd, opposite answers.

**This is precisely the class `GATE_COVERAGE`'s `posture` row was minted to close** — its own comment:
*"`finalize` refuses under an un-concluded merge, rebase or pick at exit 1, and until M52 the preview
reported the task's findings over that state and exited 0"* — and the `what's-left:` line every composed
task prints still claims it: *"previews part of the finalize gate: **the repository posture finalize
refuses under**, …"*. Driven, it does not, for this member at this cwd.

Now the second half, which is why this is tier 2 and not tier 3 — **the route cannot run**:

```
cwd = $REPO/.jigc/worktrees/pw-one      (the checkout that printed the route)
argv : git switch main                                            <- the emitted route, verbatim
observed: fatal: 'main' is already used by worktree at '<ABS>/repo'
          rc=128
git branch -a  ->  * (no branch)
                   + main
```

The repository has exactly one branch and the main checkout holds it, so there is **no `<branch>`** the
route's placeholder can be filled with from inside a fan-out worktree. The composed step meanwhile tells
the agent to run `jigc task finalize fanout-preview`, and `jigc task list` from there lists all three
tasks as live. A task minted in a fan-out worktree is therefore **unfinalizable from where it was minted**,
its preview says it is clean, and its only offered exit is a command that fails — the charter's tier-2
predicate (*a posture or route dead end*) on both legs.

**Why not tier 1:** nothing lands and nothing is destroyed — HEAD unmoved in both checkouts, the staged
`p.txt` intact, the task recoverable (`jigc task discard <id> --force` works, and the task *can* be
finalized from a cwd whose checkout is attached — I drove the amend cell from `$REPO` and it reached
`finalize.base-mismatch`, a different and correct refusal).

**Not new code.** `posture_subject`'s exemption is M51's and the seam's `live` subject is M52's; the
usability batch and F-10 touched neither. rc.18 and rc.19 both drove the detached cell from an **ordinary
linked** worktree (where it holds) and drove cwd (c) only for the *milestone*-shaped doors; this
`(door, cwd)` pair is the one neither reached. I state that rather than presenting it as a regression.

---

### A2-3 · **TIER 3** — F-10's design of record states an envelope fact the binary does not carry, on the one row it marked ***verify***

[f10-amend-settle.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/f10-amend-settle.md)
→ *Envelopes*, quoted:

> `jigc task amend` → the composed `{task, text}` arm (§1 of `command-output-contract.md`, unchanged
> shape); **`--format json` carries the pinned sha under `text`**, no new key needed — **verify**; if a
> key is needed it is declared in the additive-key paragraph.

Driven:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       HEAD short = 7a24bef ; full = <40 hex>
argv : jigc --format json task amend "sha probe"           -> exit 0
  keys ['task','text']            (the declared ArmShape::Object(&["task","text"]) — the SHAPE is right)
  short sha in text : False
  full  sha in text : False
  "amending" in text: False
  text bytes: 4170
CONTROL, the TEXT arm of the same door, same rig, same HEAD:
  short sha in text : True        "amending:" in text : True
CONTROL, the RESUME doors on the text arm (MEDIUM-2's fix — these DO carry it):
  jigc start --task <id>          -> 2 hits (`amending:` + the pushed advisory)
  jigc workflow amend --task <id> -> 1 hit
CONTROL, the same resume door on the JSON arm:
  jigc --format json start --task <id> -> text: amending 0, pushed 0
```

**The binary is internally consistent and the settle row is the false statement** — which is why this is
filed as a record row and not as a behaviour row. The composed `{task, text}` arm carries the *workflow
body* and not the door's ack, at **every** composing door, driven:

```
CONTROL, an ORDINARY start:
  jigc start "ordinary ack probe" --workflow quick-fix                -> text arm: "task minted:" ×1
  jigc --format json start "ordinary ack probe two" --workflow quick-fix
    -> keys ['task','text'] ; "task minted:" in text ×0
```

So the ack — `task minted:`, the `amending:` block, `also open:` — is a **text-arm-only header** on both
doors, and `ENVELOPE_ARMS`' `("task amend", "Composed")` row is honest about the keys. What the settle
asserts is that the *pinned sha* survives into the JSON, and a driver reading `--format json` gets neither
the sha nor the commit's subject line. The one mitigation that matters is present: the step body no longer
points at *"the ack above"* — MEDIUM-2's reword landed, and the JSON `text` tells the reader to run
`git log -1 --format=%s` themselves, which is runnable:

```
  from an ordinary task commit — check the subject line of the commit this task
  pinned (`git log -1 --format=%s`) and stop if it is one of them: a milestone …
```

**Why tier 3 and not tier 2:** nothing is a dead end — the JSON driver has a runnable command for the one
fact it is missing, and the `task` key it does get is what every subsequent call needs.

**Why it is worth a row at all:** the settle is what the F-10 workflow names as this feature's design of
record, the row carries an explicit **verify** marker, and the verification evidently did not happen — the
same instrument shape M50 and M51 both recorded (*a claim about how the composed product behaves is not
established by reading the files it is composed from*). The smallest correction is to strike the clause
with this datum, or to declare the additive key the row's own escape hatch already names.

---

## 7 · What I did NOT drive, and why

Stated plainly; none of these is presented as driven.

1. **`InProgress::ALL` × each of the 12 acting doors as a full 10 × 12 cross.** I drove **16 states × 8
   doors** (§4.5, one fresh rig per state) and **1 state (`bisect`) × all 12 doors** (§4.5, second block),
   plus **16 states × the amend arm** (§3.2). The un-driven cells are the 15 non-`bisect` states at the
   four doors of the second block. Reason: the member is decided by one shared `adjudicated_breach`
   composition and the door only filters on `PostureMember`, so the cross is not independent, and both
   projections of it are driven.
2. **`InProgress::ALL` × the fan-out-worktree subject, member by member.** Driven at `bisect` only
   (§4.3), across three cwds and both `aim_at` producers. The arc under review changed neither the
   detection nor the render there.
3. **The amend arm × the fan-out-worktree subject × each `InProgress` member.** Driven at the `detached`
   member (§6, A2-2) and at `bisect` in the main checkout (§3.2). The remaining cells are the same
   independence argument as (1).
4. **Both `finalize.fan-out.squash` commit models at the fan-out posture cell.** I drove the default
   model only; the arc moved no commit-model code, and the amend arm is `task finalize`'s second model,
   not `milestone finalize`'s.
5. **`(2, DEFECT C)` at the *milestone boundary's* seam.** I raced the `task finalize` seam on both the
   ordinary arm (§4.4) and the amend arm (§3.6).
6. **The `GIT_DIR` redirect** — the family's *declared-out* bound, unchanged by this arc.
7. **A genuinely concurrent racer** on any rollback population — out of this axis's scope and a declared
   bound in M52's own record. My shim is deterministic and single-process, which is a *timing fixture*,
   not concurrency.
8. **An apostrophe- or `#`-bearing repository root.** I drove the **space** axis on one rig (§3, row 37).
9. **A pushed commit.** The settle declares pushed-ness undetectable and ships advice rather than a
   fence; I did not build a remote to re-establish a fact the baseline already drove.
10. **Every `--format json` arm of every acting door under every member.** I drove the JSON arm at
    `milestone finalize`, `task finalize` (both models, four outcomes), `task validate`, `task diff` and
    `task amend`. The remaining leaves' envelope shapes are axis 5's table and I do not claim them.
11. **`PATH_ARG_OCCURRENCES`, `DOCTYPE_DOORS`, `SLUG_DOORS`, `WORK_UNIT_ID_DOORS`, `SchemaChangeKind`,
    `ManifestKind`.** I read the counts from the code (§2) as the charter asks, and drove only the rows
    axis 2 owns. Those registries' own tables are axes 5 and 7's.
12. **The `stated_at_fence.rs` / `count_fences.rs` suites themselves.** I drove the fences *through the
    binary* with mutated packs (§5); I did not run the Rust suites, and a green suite is not a driven row.

---

## 8 · Observations driven and carried — **not** defects

1. **The `DeclaredWhereReachable` residual** (§5, mutant 3): dropping `finalize.amend-index-dirty` from
   `amend-message.yaml`'s `states-constraints:` reddens nothing, because the code is deliberately held out
   of `ambush_class_codes()`. Two production homes and one design doc state this outcome in as many words,
   so the binary tells the truth; the residual is that **both** fences fall silent together, and the
   contract can go un-stated in the only composition that reaches its door. Carried, not filed.
2. **`--pack-from-dev` makes the named-fact tier vacuous** (§5): the tier keeps the *manifest* subject by
   its own recorded design, and `--pack-from-dev` drops the manifest. Two mutants that redden on a
   `--repin` rig exit 0 on a `--pack-from-dev` one. A measurement about the *instrument*, recorded so the
   next reviewer does not read a false green.
3. **A bisect begun inside the same `git` process that performs the amend lands at exit 0** (§3.6, the
   narrow shim). Outside any predicate jigc can ask; M52's record names the hook as the design's racer.
4. **Amending a milestone-boundary commit replaces a structural message with a prose one at exit 0**
   (§3, rows 38–39). By design — the settle refuses to invent a discriminator that does not exist, the
   composed step names the three shapes, and the mint ack prints HEAD's subject line. Driven harmless:
   tree identical, record intact, `base:` line intact, `validate` 0, `doc show` 0, and `milestone
   add-task` afterwards 0.
5. **`amend.head-shape`'s no-intent locus over an unborn repository is `work-unit:amend-4b825dc`**, where
   `4b825dc` is git's empty-tree sha. It *is* the id the mint would have taken, which is what the
   function's doc-comment promises, so the row is correct; it is a slightly odd name and I record it
   rather than file it.
6. **`task validate` on the amend arm previews the posture correctly for the `OperationInProgress`
   family** (§3, and driven mid-`bisect`: exit 1, `repo.operation-in-progress`, both on the text arm and
   under `--dry-run`). A2-2 is `HeadDetached`-specific.
7. **Two exit codes in this file were first read through a pipe and were wrong** — `task validate`'s and
   `--dry-run`'s on the dirty-index cell read `0` behind `| head`, and are **3** measured bare. Both were
   re-measured before being written down (§3.4 says *ALL MEASURED BARE*). No exit code in any table above
   comes from a pipeline.

---

## 9 · M52 §A rows: CLOSED / STILL-OPEN

| M52 §A row (axis 2) | tier | rc.20 |
|---|---|---|
| `(2, DEFECT A)` — the clean `cherry-pick --no-commit` cell | **1** | **CLOSED** (§3.2, §4.5) |
| `(2, DEFECT B)` — `SquashMerge`'s predicate is false of a conflicted squash | 3 | **STILL-OPEN(1.x, expected)** (§4.5) |
| `(2, DEFECT C)` — the commit seam prints no state-truth clause and no re-run | 2 | **STILL-OPEN(1.x, expected)**, door axis **+1** (§4.4, §3.6) |
| M51 `D1` / `D2` | — | **still CLOSED** (§3.2, §4.5) |
| rc.17 `(2, DEFECT 1)` — the boundary commits an un-concluded worktree at exit 0 | **1** | **still CLOSED** (§4.3) |
| rc.18 `(2, F-1)` — the aimed route | 2 | **still CLOSED** (§4.3) |
| rc.18 `(2, F-2)` — the foreign squatter's bytes acked as jigc work | 3 | **STILL-OPEN(1.x, expected)** (§4.6) |
| rc.19 `(2, N-1)` — the hook cries wolf | 3 | **CLOSED** (§4.1) |
| rc.19 `(2, N-2)` — the placement family's inert backstop | 3 | **CLOSED** (§4.2) |
| rc.19 `(2, N-3)` — `provision`'s false base clause | 3 | **STILL-OPEN(1.x, expected)** (§4.7) |
| rc.19 `(2, N-4)` — the linked-worktree promote's false diagnosis | 3 | **STILL-OPEN(1.x, expected)**, exactly as the reconciler corrected it (§4.8) |

**Counts.** 6 CLOSED (2 newly closed by the usability batch) · 5 STILL-OPEN, every one of them a row the
charter triaged into the 1.x ledger · **0** regressions · **0** tier-1.

---

## 10 · What this adds over flow-54 arm 4 (and over flow-55's amend arm)

`crates/cli/tests/flow54_acceptance.rs`'s posture arm iterates `InProgress::ALL × BEHALF_DOORS`' acting
rows from a built fixture root, in the **debug** binary, through the library seam; the F-10 acceptance
(`task_amend::`, 10/10) iterates the settle's refusal table the same way. Five things here are outside
what either can reach, and three of them are what a *driver* is for.

1. **Release posture.** Every row above ran on the installed `1.0.0-rc.20`, where the
   `Route::mechanical` argv fence does not exist — the only posture in which §4.3's central question
   (*does the emitted route run?*) can be asked at all.
2. **The route is executed, not asserted.** §4.3 runs `git -C <abs> bisect reset` from five cwds
   including `/`; §3.4 runs the amend arm's `git -C <abs> restore --staged -- <path>` and then finalizes;
   §3, row 37 runs both under a spaced root where `shell_operand` quotes two operands. And A2-2's second
   half is *only* reachable this way: `git switch <branch>` is a textually perfect route that exits
   **128**, and a text assertion cannot tell those apart.
3. **cwd is a dimension the suite does not have.** A `#[test]` runs from the crate's working directory.
   A2-2 exists only because the driver stood in `$REPO/.jigc/worktrees/pw-one`; and F-10's own review
   noted that `posture_door_axis.rs` contains **zero** occurrences of `amend` (its `BEHALF_DOORS` filter
   keeps acting rows, and `task amend` is `Neither`), so the amend arm's posture coverage in-suite is the
   4 `AMEND_TABLE` rows — against the **16 members × the arm** driven in §3.2 and the **16 × 8** in §4.5.
4. **The pack-load fences driven as a product, through mutated packs.** §5 deletes one authored *fact* at
   a time from a throwaway pack and asks **five** doors, which is how mutant 3's silence and
   `--pack-from-dev`'s vacuous green became measurements rather than assumptions. A green
   `stated_at_fence.rs` says the fence's own axis is swept; it does not say which rig a reviewer can
   trust.
5. **Shell-level and cross-process composition.** N-1 and N-2 live in a **shell script the binary
   writes**, invoked by **git**, greping a report the binary printed; the seam races in §4.4 and §3.6
   live in a `git` shim on `PATH`; `(2, F-2)` needs a *second repository*. No Rust-side fence spans those.

---

## 11 · Doors covered

Every clap leaf that is the door of at least one **driven** row above (`VERB_KINDS` spelling), **35** of
the registry's 48:

`setup` · `uninstall` · `migrate-corpus` · `rename` · `relocate` · `describe` · `validate` · `start` ·
`workflow` · `config set` · `doc create` · `doc add-item` · `doc remove-item` · `doc retitle-item` ·
`doc rename` · `doc set-field` · `doc set-slot` · `doc author` · `doc show` · `doc schema` · `doc list` ·
`task amend` · `task diff` · `task validate` · `task discard` · `task finalize` · `milestone create` ·
`milestone add-task` · `milestone add-from-spec` · `milestone list-tasks` · `milestone provision` ·
`milestone join` · `milestone execute` · `milestone finalize` · `milestone discard`

**All 12 acting `BEHALF_DOORS` rows are among them, and so is the 11th `COMMITTING_DOORS` row** (the
amend arm of `task finalize`, driven at nine distinct cells). Leaves used only to build fixtures or read
state — `task list`, `config get`, `config list`, `ingest`, `migrate`, `unmanage`, `upgrade`, the four
`config *-step`/`fill`/`fork` leaves, `task bind` — are deliberately **not** claimed: no verdict row is
keyed on them.

---

## 12 · Instrument honesty

- Every exit code in every table was measured **bare**. Two were first read through `| head`, were wrong,
  and were re-measured before being written down (§8.7).
- Every rig root came from `mktemp -d`; nothing was torn down; **`rm -rf` on a variable path appears
  nowhere in this review**, and no `--clean`/`--reset` was reached for.
- Nothing was written into any `.jigc/` by hand. The three non-rig fixtures are named in the header and
  each is a thing a real operator's repository can have (a hook, a `PATH`, a second repository).
- **The Codex source pass for this axis was not read.**
- `command grep` was used for every read under `.jigc/`.
- **A2-2 is the judgment I am least sure of and I have written both sides.** Which door is *wrong* is not
  obvious: a plain `task finalize` inside a jigc-detached worktree arguably *should* refuse (the commit
  would belong to no branch), in which case the preview is the offender; or the whole `Dedicated`
  exemption should reach the seam, in which case the seam is. I have filed the **divergence plus the
  unrunnable route**, which is true either way, and have not prescribed which side moves.
- **A2-1 was filed one member too narrow and corrected before it was written up.** My first reading was
  *"F-10's new code lies in its help"*; the ordinary-arm control (§6, Control 2) shows the parity sentence
  was already false over the empty-commit gate, so I re-derived the axis as *the whole set `--dry-run`
  refuses on* and drove all four gates. I record the correction because the wrong version would have sent
  a fixer at one cell of a two-cell class — the incomplete-fix shape M45 exists for.
- **A2-3 is a record row, not a behaviour row**, and I have said so explicitly with the control that
  proves the binary is self-consistent. A reviewer who wants only behaviour findings should read it as
  one item to strike from a doc.
- The three defect rows are one tier-2 and two tier-3. I looked specifically for the tier-1 shape at every
  cell of the new committing arm — a byte lost, a tree moved, a doc promoted without being committed, a
  false green on `jigc validate` — and drove a before/after control for each. None of them reached it, and
  the F-10 review's HIGH-1 is closed over **nine** cells of an eight-leaf class.

---

# RECONCILIATION — the Opus driver × the Codex source pass

**Reconciler's binary, asserted first.** `/Users/maurice/.local/bin/jigc --version` → `jigc 1.0.0-rc.20`;
repo HEAD `4d3175c3`. Release posture, so every emitted `git …` span quoted below was **executed**, not
asserted. Every exit code was measured **bare**, output to `/dev/null` or a file — never through a pipe.

**The rule applied** (`acceptance-design.md` → The reconciliation rule): a claim by one side the other
cannot reproduce is a **lead**, not a finding. Every Codex claim is entered as `lead(codex, …)` and then
**driven** — to a repro block (CONFIRMED, origin codex) or to a refutation carrying the falsifying datum.
Every driver **defect** the source pass is silent on **stays a finding** and was **re-driven once** by the
reconciler. A claim that cannot be driven at all stays an **OPEN LEAD** with its reason, never promoted on
the source read and never dropped.

**Headline: the two passes do not contradict each other anywhere.** Codex named no defect the driver
missed except `(2, F-2)`, which the driver had already driven; the driver named three defects
(`A2-1`/`A2-2`/`A2-3`) the source pass is **silent** on — none of them contradicted — and all three
reproduce on the reconciler's own rigs. **Tier-1 rows: 0.**

---

## A · Rows filed without a repro block — the demotion audit

The driver's §3 `(door, cell)` table carries **50** rows. Twenty of them are backed by no fenced repro
block anywhere in the driver file: rows **20–23** (`amend.head-shape` ×4), **29** (content gate),
**30–33** (cwd (b) and cwd (d)), **37** (spaced root), **38–39** (boundary / record-only commit at HEAD),
**40–41** (verb-routed), **42–44** (`task diff` / `doc list --task` / `doc show --task`), **45–47**
(`also open:` / discard / serial-collision). As filed, those rows are **argv-and-observation lines, not
repro blocks**.

Rather than demote and discard the evidence, the reconciler **drove all twenty**. Every one reproduced;
**none required demotion in the end**, and each now stands on the reconciler's repro rather than the
driver's prose. The repros are §B.11–§B.16 below. This is recorded because the *filing* was thin even
though the *facts* held: a reader must not take those twenty rows as driver-evidenced.

No other row was demoted. Rows 1–19, 24–28, 34–36, 48–50 each sit under a fenced block in the driver file
(§3.1 · §3.2 · §3.3 · §3.4 · §3.5 · §3.6 · §6 A2-1 · §6 A2-2 · §6 A2-3).

---

## B · The reconciliation ledger

### B.1 · `lead(codex, 1)` — a foreign repository's worktree parked at `.jigc/worktrees/<sub-id>` is accepted as a live sub-task worktree and contributes staged bytes to `milestone finalize` — **CONFIRMED (repro), origin codex, = rc.20 `(2, F-2)`**

Codex's proposed reproduction was run **exactly as proposed** and lands exactly as predicted. This is the
same row the driver drove at §4.6; the two repros are independent and agree.

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       repo B = a SECOND repository (mktemp -d, 2 commits, git init)
       jigc milestone create "Foreign wave"        -> 0   (record commit 9ff0787)
       jigc milestone add-task foreign-wave "fw area" -> 0 (record commit d6a774c)
       NO `jigc milestone provision`
       git -C $B worktree add -q "$REPO/.jigc/worktrees/fw-area" -b fwbranch
       printf 'B-SECRET-PAYLOAD\n' > $W/bsecret.txt ; git -C $W add bsecret.txt
       printf 'B-UNTRACKED-KEEP\n'  > $W/keepme.txt
BEFORE: git -C $W rev-parse --show-toplevel -> the squatted path itself
        git -C $W symbolic-ref -q HEAD      -> refs/heads/fwbranch     (B's branch, not jigc's)
        main HEAD                            -> d6a774c

argv : jigc milestone join foreign-wave        -> exit 0
  joined milestone:foreign-wave — 0 doc(s) merged
    no docs staged from: fw-area
argv : jigc milestone finalize foreign-wave    -> exit 0
  finalized 9ca48d3 — Finalize milestone foreign-wave (1 sub-task)
    added bsecret.txt
    modified docs/milestone-records/foreign-wave.md
    2 files committed
    sub-tasks: fw-area: 1 code file          <- B's bytes, acked as the sub-task's own work

AFTER : git -C $REPO cat-file -p HEAD:bsecret.txt -> B-SECRET-PAYLOAD
        $W/keepme.txt -> B-UNTRACKED-KEEP (intact)
        B worktree registrations: 2 (intact)      .jigc/displaced/ -> absent
```

**No byte is lost** — B's untracked plant survives, B's registrations survive, nothing is displaced — so
this is a **tier-3** *attribution* row, not a loss row, exactly as both passes tier it. Codex's mechanism
(`subtask_worktrees` classifies on `classify_leftover` → `OwnWorktree` without asking whether the git
common directory belongs to `jigc_home`) is consistent with what drove; the reconciler does **not** claim
the mechanism, only the behaviour.

### B.2 · `lead(codex, CLOSED (argv))` — all ordinary detached/unborn and all ten `InProgress::ALL` rows across acting doors; operation detection precedes detached-HEAD diagnosis, `UncommittedCherryPick` before the unmerged-index fallback — **CONFIRMED (repro)**

Sixteen `--git-state` members × a `CommitsOnBehalf` door (`milestone create`) × the `Neither` control
(`task amend` must mint) × the amend arm of `task finalize`, one fresh rig per member:

```
setup (×16): rig=$(dev/jigc-rig committed-singletons --git-state <S> --binary …) || continue; eval "$rig"
argv       : jigc milestone create "Probe wave"   |  jigc task amend "posture probe"  |  jigc task finalize posture-probe
```

| `--git-state` | `milestone create` | `task amend` (mint) | `task finalize` (amend arm) | noun printed | HEAD |
|---|---|---|---|---|---|
| `merge` | 1 `repo.operation-in-progress` | **0** | 1 `repo.operation-in-progress` | *a merge is in progress* | unmoved |
| `squash-merge` | 1 ″ | **0** | 1 ″ | *a squash merge is staged and not committed* | unmoved |
| `rebase-merge` | 1 ″ | **0** | 1 ″ | *a rebase is in progress* | unmoved |
| `rebase-apply` | 1 ″ | **0** | 1 ″ | *a rebase is in progress* | unmoved |
| `am` | 1 ″ | **0** | 1 ″ | *a `git am` is in progress* | unmoved |
| `cherry-pick` | 1 ″ | **0** | 1 ″ | *a cherry-pick is in progress* | unmoved |
| `sequencer` | 1 ″ | **0** | 1 ″ | *a cherry-pick is in progress* | unmoved |
| `dangling-sequencer` | 1 ″ | **0** | 1 ″ | *… left a queue of commits in `sequencer/`* | unmoved |
| `uncommitted-pick` | 1 ″ | **0** | 1 ″ | *an uncommitted cherry-pick is in progress* | unmoved |
| `uncommitted-pick-range` | 1 ″ | **0** | 1 ″ | ″ | unmoved |
| `uncommitted-pick-conflicted` | 1 ″ | **0** | 1 ″ | ″ ← **not** `UnmergedIndex` | unmoved |
| `uncommitted-pick-resolved` | 1 ″ | **0** | 1 ″ | ″ | unmoved |
| `revert` | 1 ″ | **0** | 1 ″ | *a revert is in progress* | unmoved |
| `unmerged-index` | 1 ″ | **0** | 1 ″ | *a conflict left unmerged paths in the index* | unmoved |
| `bisect` | 1 ″ | **0** | 1 ″ | *a bisect is in progress* | unmoved |
| `detached` | 1 **`repo.head-detached`** | **0** | 1 **`repo.head-detached`** | *HEAD is detached* | unmoved |

**The precedence legs Codex names are both driven.** `rebase-merge` and `rebase-apply` are HEAD-detached
in git and answered *a rebase*, not `repo.head-detached` — only the bare `detached` member reaches it;
and `uncommitted-pick-conflicted`, whose index is genuinely conflicted, answered the **pick**, not the
unmerged-index fallback. **HEAD unmoved in all 48 cells.** This also confirms M51 `D1`/`D2`/`D3`
(§B.10) and the `task amend` = `Neither` classification (§B.6).

### B.3 · `lead(codex, CLOSED (argv))` — `setup`'s unborn exemption remains setup-only — **CONFIRMED (repro)**

```
setup: rig=$(dev/jigc-rig --git-state unborn --binary …) || exit; eval "$rig"    (standalone: no jigc setup)
BEFORE: git rev-parse HEAD -> fatal: ambiguous argument 'HEAD'   ;  commits: 0

argv : jigc setup                       -> exit 0    install commit → 87c0a8d ; commits after: 1
       (the exemption: setup BIRTHS HEAD on an unborn repository)

a SECOND unborn rig, other doors BEFORE setup:
argv : jigc milestone create "Unborn wave"  -> exit 1
  blocking · repo.head-unborn — HEAD is unborn — this repository has no commits yet, so there is no base
    for jigc to commit against
    route: land the repository's first commit with `git commit`, then re-run this command
argv : jigc task amend "unborn probe"       -> exit 1   amend.head-shape (see §B.11)
```

The exemption is **setup's alone**: a second commit-on-behalf door over the identical shape refuses
`repo.head-unborn` at exit 1.

### B.4 · `lead(codex, CLOSED (argv))` — `(2, F-1)`'s aimed posture routes and the repo-relative-locus / absolute-route split — **CONFIRMED (repro)**; `(2, DEFECT 1)` still closed with it

```
setup: rig committed-singletons; milestone cwd-wave + sub-task cw-area PROVISIONED
       W=$REPO/.jigc/worktrees/cw-area ; git -C $W bisect start ; git -C $W bisect bad
       mkdir -p $REPO/docs/deep ; git worktree add -q $RIG/linked -b linkedbr
BEFORE: 4 BISECT_* markers in .git/worktrees/cw-area
```

| door | cwd (a) `$REPO` | cwd (b) `docs/deep` | cwd (d) linked |
|---|---|---|---|
| `milestone finalize cwd-wave` | 3 | 3 | 3 |
| `task validate cw-area` | 3 | 3 | 3 |
| `milestone discard cwd-wave` | 1 | 1 | 1 |
| `uninstall` | 1 | 1 | 1 |

All **12** cells emitted one byte-identical span:

```
git -C <ABS>/.jigc/worktrees/cw-area bisect reset

RUN VERBATIM:  cwd=$REPO -> 0 · cwd=$REPO/docs/deep -> 0 · cwd=$W -> 0 · cwd=$RIG/linked -> 0 · cwd=/ -> 0
AFTER the whole sweep: main HEAD unmoved
```

`(2, DEFECT 1)` rides the same repro: the boundary **refuses** over the un-concluded worktree at exit 3
rather than committing it at exit 0, and main HEAD never moved.

### B.5 · `lead(codex, CLOSED (argv))` — rc.20 `N-1` and `N-2` were changed by the usability batch — **CONFIRMED (repro)**, both

**N-1 — the hook no longer cries wolf on a plain deletion:**

```
setup: rig committed-singletons
BEFORE-CONTROL: printf 'ordinary\n' > ord.txt; git add ord.txt; git commit -m "probe: ordinary"
  -> rc=0 ; `^jigc:` lines: 0
argv : git rm -q VISION.md ; git status --porcelain -> "D  VISION.md"    (a DELETION, no rename anywhere)
       git commit -m "probe: delete a managed singleton"
observed: rc=0 ; `^jigc:` lines: 0   (no jigc line at all)
```

**N-2 — the blocking rename backstop now reaches the whole placement family (4/4):**

```
setup (×4, one fresh rig each): rig committed-singletons
argv : git mv <A> <B> ; git commit -m "probe"      (rc measured bare)

  docs/roadmap.md       -> docs/roadmap-oob.md     | staged: R  … | rc=1  BLOCKED
  VISION.md             -> VISION-OOB.md           | staged: R  … | rc=1  BLOCKED
  CHANGELOG.md          -> CHANGELOG-OOB.md        | staged: R  … | rc=1  BLOCKED
  docs/decisions-log.md -> docs/dl-oob.md          | staged: R  … | rc=1  BLOCKED
each printing: "jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses
                jigc identity tracking; use `jigc rename` instead (commit blocked)."

CONTROL, a LOCATION doctype (docs/milestone-records/loc-wave.md -> …/zz-oob.md, after `milestone create`):
  rc=1, byte-identical line
```

### B.6 · `lead(codex, new F-10 code)` — `task amend` is correctly `Neither`; `COMMITTING_DOORS` rises to 11 while the commit-on-behalf **leaf** set stays at ten — **CONFIRMED (registry read + repro)**

Registries read from the code at HEAD `4d3175c3`, parsed rather than eyeballed:

```
VERB_KINDS        : 48 rows  {Write: 36, Read: 12}
BEHALF_DOORS      : 48 rows  {Neither: 36, CommitsOnBehalf: 10, MovesOnBehalf: 2}
  CommitsOnBehalf : setup · migrate-corpus · rename · task discard · task finalize ·
                    milestone create · milestone add-task · milestone add-from-spec ·
                    milestone finalize · milestone discard
  MovesOnBehalf   : relocate · config set
  the amend row   : ("task amend", Neither)
COMMITTING_DOORS  : 11 rows — the 11th is verb "jigc task finalize (amend)",
                    error_code ERROR_AMEND_REJECTED = "finalize.amend-rejected"
AMBUSH_CONTRACTS  : 6 rows / 3 dispositions — Owed 3 (promote-clobber · nothing-staged · carried-staged) ·
                    DeclaredWhereReachable 2 (finalize.amend-index-dirty · finalize.amend-staged-doc) ·
                    Exempt 1 (setup.dirty-install-path)
```

Every count matches the driver's §2. The `Neither` classification is **behaviourally** confirmed by §B.2
(`task amend` mints at exit 0 under all sixteen git states), and the 11th row is **behaviourally**
confirmed by §B.7's log.

### B.7 · `lead(codex, new F-10 code)` — `git_commit_amend` supplies `--amend -F` to the same immediately re-probed seam — **CONFIRMED (source + repro)**

Source, `crates/cli/src/task.rs:7568`: `git_commit_amend` calls `git_commit_capture(subject, ["--amend",
"-F", message_file])` — the identical seam `git_commit` uses. Driven two ways:

```
--- the hook-rejection arm, both hooks, with the log identity ---
setup: rig committed-singletons ; jigc config set invocation-log true
       HD=$RIG/hooks ; printf '#!/bin/sh\necho "the hook says no" >&2\nexit 1\n' > $HD/<pre-commit|commit-msg>
       git config core.hooksPath $HD ; jigc task amend "logged probe two" ; author type+summary
argv : jigc task finalize logged-probe-two          -> exit 1 (bare)
  `git commit` was rejected (no commit was made):
  the hook says no

  `HEAD` is unchanged — the commit this task is repairing still carries the message it had, and task
  logged-probe-two's authored commit doc is still in `.jigc/tasks/logged-probe-two/docs/`. Fix the
  hook's complaint, then re-run `jigc task finalize logged-probe-two`.
AFTER: HEAD BYTE-IDENTICAL · message identical · tree identical · task still live (1 active task)
`commit-msg` hook: output byte-identical to `pre-commit` (modulo shas)
--format json: findings[0].key = {"code":"finalize.amend-rejected","target":"task:logged-probe-two"}

the invocation log:
  {'argv':['task','amend','logged probe two'],   'exit_code':0,'error_code':None,          'binary_version':'1.0.0-rc.20'}
  {'argv':['task','finalize','logged-probe-two'],'exit_code':1,'error_code':'finalize.amend-rejected','binary_version':'1.0.0-rc.20'}
  {'argv':['task','finalize','logged-probe-two'],'exit_code':0,'error_code':None,          'binary_version':'1.0.0-rc.20'}  <- hook removed
```

The 11th `COMMITTING_DOORS` row is therefore confirmed **behaviourally**: one leaf, two commit models,
two error identities, and the log discriminates them.

### B.8 · `lead(codex, new F-10 code)` — the amend arm's dirty-index and promoting-doc gates precede planning/promotion; both codes carry `DeclaredWhereReachable` ambush rows — **CONFIRMED (repro + registry read)**

The promoting-doc gate over the `doc` **write-leaf axis**, worktree asserted empty before and after every
cell (this is the F-10 review's HIGH-1 closure, re-driven):

```
setup: rig committed-singletons ; jigc task amend "doc leaf sweep"   -> task doc-leaf-sweep
BEFORE: git status --porcelain -> (empty)
```

| leaf | exit | code | worktree after |
|---|---|---|---|
| `doc add-item decisions-log#entries --title …` | 1 | **`finalize.amend-staged-doc`** | **empty** |
| `doc remove-item roadmap#milestones/m-alpha` | 1 | ″ | **empty** |
| `doc retitle-item roadmap#milestones/m-alpha` | 1 | ″ | **empty** |
| `doc set-field changelog#releases/1-0-0/link` | 1 | ″ | **empty** |
| `doc set-slot vision#thesis --from-file …` | 1 | ″ | **empty** |
| `doc create adr --title "A new decision"` | 1 | `create.gate-blocked` | **empty** |
| `doc rename vision --to "A New Vision"` | 1 | `write.identity-change` | **empty** |

```
AFTER the whole sweep: git status --porcelain -> (empty) ; HEAD c35f00d unmoved
                       jigc doc list --task doc-leaf-sweep -> commit:doc-leaf-sweep  (one doc)
```

The dirty-index gate is driven at §B.13 (four index shapes, the route executed) and §B.14 (the spaced
root). `AMBUSH_CONTRACTS` carries `DeclaredWhereReachable` rows for **both** codes (§B.6's read).

### B.9 · `lead(codex, STILL-OPEN)` — `(2, DEFECT B)`: `SquashMerge` remains exactly `SQUASH_MSG && !MERGE_HEAD` — **CONFIRMED (repro)**

```
setup: rig committed-singletons; branch `cb` and `main` both rewrite conf.txt
argv : git merge --squash cb                      -> rc=1
state: markers -> MERGE_MSG, SQUASH_MSG  (no MERGE_HEAD)
       git ls-files -u | wc -l -> 3               <- the index IS conflicted
       git status --porcelain -> "UU conf.txt"    <- nothing is staged
argv : jigc milestone create "DefB probe"         -> exit 1 (bare)
  blocking · repo.operation-in-progress — a squash merge is **staged and not committed** — the repository
    is not in a committable state
    route: conclude it, or abandon it with `git reset --merge` (which also discards anything else you had
           staged, from the index and from your working tree), then re-run this command
```

The door refuses — which is safe — but the noun it prints is **false of the state**: nothing is staged,
the index is conflicted. Tier 3, triaged into the 1.x ledger; unchanged by this arc.

### B.10 · `lead(codex, STILL-OPEN)` — `(2, DEFECT C)`: a seam-time posture failure exits through the raw commit error rather than the door's full state-truth / re-run envelope — **CONFIRMED (repro), on BOTH arms**

```
setup (ordinary arm): rig committed-singletons --start quick-fix "seam probe" ; author ; git add work.txt
                      jigc task validate seam-probe -> 0 (bare)
                      a deterministic `git` shim first on PATH that fires
                      `git -C $REPO bisect start` + `bad` ONCE on a `status --porcelain` probe, then execs /usr/bin/git
argv : PATH="$RIG/shim:$PATH" jigc task finalize seam-probe      -> exit 1 (bare)
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
    committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
  shim log: "shim fired on: status --porcelain --untracked-files=all"
STATE AFTER, all true and NONE of it stated by that surface:
  HEAD unmoved · work.txt still staged · .jigc/tasks/seam-probe/docs/ intact
  (commit:seam-probe.md + provenance.json) · BISECT_LOG present

CONTRAST — same door, same seam, a rejecting pre-commit hook instead:
  `git commit` was rejected (no commit was made):
  the hook says no

  task seam-probe is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/seam-probe/docs/`, and anything you had `git add`-ed is still in git's index.
  Fix the hook's complaint, then re-run `jigc task finalize seam-probe`.

--- the AMEND arm, same shim shape ---
setup: rig committed-singletons ; jigc task amend "amend reverify" ; author type+summary
argv : PATH="$RIG/shim:$PATH" jigc task finalize amend-reverify   -> exit 1 (bare)
  blocking · repo.operation-in-progress — a bisect is in progress — …
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
  shim log: "shim fired on: status --porcelain --untracked-files=all"
AFTER: HEAD unmoved · message identical · BISECT_LOG present · task LIVE (1 active)
```

The driver's measurement — **DEFECT C's door axis is now wider by one**, the amend arm inheriting the same
truncated envelope from the shared `git_commit_capture` seam — is **confirmed**. This also drives Codex's
`D3b` closure (*every hook-capable commit, amend included, converges on `git_commit_capture` whose
immediately preceding statement is `subject.verify(Commit)`*): the amend arm's seam **did** re-probe and
**did** refuse, HEAD unmoved.

### B.10b · `lead(codex, M51 rows)` — `D1` / `D2` / `D3` CLOSED — **CONFIRMED (repro)**, all three inside §B.2

`D1` (operation-first probing stops rebase/bisect falling through to detached HEAD): `rebase-merge` and
`rebase-apply` are HEAD-detached in git and answered *a rebase*; `bisect` answered *a bisect*; only the
bare `detached` member reached `repo.head-detached`. `D2` (`git am` and apply-backend rebase disjoint):
two nouns — *a `git am` is in progress* vs *a rebase is in progress*. `D3` (cherry-pick, revert,
sequencer, uncommitted-pick, unmerged-index explicit ordered members): five distinct nouns, and the
conflicted uncommitted pick answered the pick.

### B.11 · `lead(codex, STILL-OPEN)` — `(2, N-3)`: provisioning reuses a registered exact-path worktree untouched rather than proving its HEAD equals the milestone pin — **CONFIRMED (repro)**

```
setup: rig committed-singletons
       jigc milestone create "Reuse wave"   -> "minted milestone:reuse-wave (shared base cbc661c)"
       jigc milestone add-task reuse-wave "ru one"
       printf 'x\n' > extra.txt; git add extra.txt
       git -c core.hooksPath=/dev/null commit -m "chore: advance"      -> HEAD now 313b93d
       git worktree add -q --detach "$REPO/.jigc/worktrees/ru-one"      -> parked at 313b93d
BEFORE: milestone pin = cbc661c   ·   the parked worktree's HEAD = 313b93d

argv : jigc milestone provision reuse-wave                             -> exit 0 (bare)
  provisioned 1 worktree(s) for milestone:reuse-wave at base cbc661c (ru-one)
AFTER: git -C $REPO/.jigc/worktrees/ru-one rev-parse --short HEAD       -> 313b93d    <- NOT cbc661c

then : jigc milestone finalize reuse-wave                              -> exit 3 (bare)
  blocking · finalize.base-mismatch — the milestone was pinned to base `cbc661c…` but HEAD is now
    `313b93d…`, and the commits landed since move more than milestone-record bookkeeping — the sub-task
    worktrees were cut from `cbc661c…`, so combining them onto HEAD cannot be proven sound
```

Two law-1 claims the binary cannot support, unchanged from rc.19 and from the driver's §4.7: the ack's
*"at base cbc661c"* and the refusal's *"the sub-task worktrees were cut from cbc661c"* — that worktree was
cut from `313b93d`.

### B.12 · `lead(codex, STILL-OPEN)` — `(2, N-4)`: the shared-workbench-baseline diagnosis is unchanged by the usability/F-10 range — **CONFIRMED (repro), tier 3 exactly as the rc.19 reconciler corrected it**

```
setup: rig committed-singletons ; git worktree add -q $RIG/lw -b lwbr ; cd $RIG/lw
       jigc start "record the linked decision" --workflow decided-task
       jigc doc add-item decisions-log#entries --title "Linked branch decision" --task …
         -> "(copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)"
       the `why` slot + the commit doc authored
BEFORE: md5 $REPO/docs/decisions-log.md == md5 $RIG/lw/docs/decisions-log.md == 41fd3323a5b7566764cb350ceeaa6130

argv : jigc task finalize record-the-linked-decision            -> exit 0
AFTER: main HEAD cdb6903 (UNMOVED) · linked HEAD 52faa34 [lwbr]
       grep -c 'Linked branch decision' $REPO/docs/decisions-log.md   -> 0
       grep -c 'Linked branch decision' $RIG/lw/docs/decisions-log.md -> 1
then : (cd $REPO && jigc validate)                              -> exit 0 (store scope), and prints:
  blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/decisions-log.md`
    differs from the recorded state
    at: docs/decisions-log.md
    route: review the out-of-band edit to `docs/decisions-log.md` and re-author it through the owning workflow

THE ESCALATION LEG STILL DOES NOT FIRE — main's next task finalize:
  advisory · reconciliation.absorb — external edit absorbed: `docs/decisions-log.md`
    route: no action needed — the external edit was absorbed into the baseline
  finalized c41c0d8 — fix: the next task in main    -> exit 0
```

The diagnosis names an out-of-band edit that never happened, and the blocking finding is absorbed at the
next finalize rather than escalating — tier 3, triaged into the 1.x ledger.

### B.13 · `lead(codex, prior Codex displacement claim CLOSED)` — both `git mv` and displacement `git rm --cached` have an immediate `verify(Move)` — **OPEN LEAD (not driven)**

Source-only in the Codex pass, and the reconciler did **not** build a displacement cell on this axis: the
displacing doors are the M52 foreign-bytes arms of `task finalize` / `milestone finalize`, whose subject is
the *writer-set complement*, not a repository posture, and no rig state produces one without hand-writing
into `.jigc/`. Recorded as an open lead with its reason rather than promoted on the source read. **It is a
no-bypass claim, not a defect claim**, so leaving it open asserts no defect.

### B.14 · `lead(codex, new F-10 code)` — `DedicatedWorktree` remains unforgeable outside `task.rs`: private fields, private constructor, typed `SeamSubject::dedicated` — **OPEN LEAD (compile-time claim, no runtime cell) — source corroborated**

Read at HEAD: `pub(crate) struct DedicatedWorktree { repo_root: PathBuf, path: PathBuf }` — both fields
private, `fn add(..)` private (no `pub`), and `SeamSubject::dedicated(&crate::task::DedicatedWorktree)` at
`repo.rs:1014` is the only typed entry. This is a claim about what **cannot be written**, so it has no
behavioural cell a driver can reach; the reconciler records it as corroborated-by-source and **not driven**
rather than promoting a source read to a finding-grade CONFIRMED.

### B.15 · `lead(codex, new F-10 code)` — the fast-forward merge still has an immediate commit re-probe, and **no production HEAD-changing `switch` or ordinary `checkout` exists** — the absence half **CONFIRMED (datum)**, the re-probe half **OPEN LEAD (source-corroborated, not driven)**

The **absence** is a complete, deterministic measurement over both crates' sources and is therefore
recorded as a confirmed datum, not a lead:

```
argv : grep -rn '"switch"\|"checkout"' crates/cli/src crates/engine/src
observed: exactly TWO hits, both string literals in crates/engine/src/finding.rs:996 / :1004 —
          members of `GIT_NON_PATH_COMMANDS`, the route-TEXT table that exempts a git verb from the
          path-quoting rule. Neither is a git invocation. Zero production HEAD-changing switch/checkout.
```

The **re-probe** half (`task.rs:7871`: `live.verify(SeamAct::Commit)?;` immediately before
`git merge --ff-only <commit>`) is present in source with its own doc-comment stating the reason, but the
reconciler did **not** race the fan-out fast-forward seam — that needs a shim fired inside the boundary's
own combine transaction, and the M52 record declares a genuine concurrent racer out of this axis's scope.
Open lead with its reason.

### B.16 · `lead(codex, breadth)` — the named M52 registries and their consumers add no commit/move act and bypass no posture seam — **OPEN LEAD (breadth claim, no single cell)**

No one driven cell establishes or refutes it; the closest behavioural evidence is §B.2's total
classification (48 `BEHALF_DOORS` rows, 10 commit-on-behalf / 2 move-on-behalf, every one refusing every
posture member) plus §B.6's parsed registries. Recorded open rather than promoted.

### B.17 · `lead(codex, boundary)` — no schema or doctype-manifest file changed in `609da011..HEAD`; the M52 zero-schema-hash boundary is intact — **CONFIRMED (datum)**

```
argv : git diff --name-only 609da011..HEAD -- '*schema-manifest*' 'crates/cli/pack/schemas/*' 'packs/methodology/schemas/*'
observed: 0 files
argv : git diff --name-only 609da011..HEAD -- '*schema*' '*manifest*' 'crates/cli/pack/**' 'packs/methodology/**'
observed: crates/cli/pack/steps/amend-message.yaml
          crates/cli/pack/workflows/amend.yaml
```

Only a step and a workflow moved under the packs; no schema, no manifest.

---

## C · The driver's own defects — status after re-driving

The Codex source pass is **silent** on all three; per the rule each stays a finding, and each was
re-driven once by the reconciler. **All three reproduce.** None is contradicted by the source pass.

### C.1 · `A2-1` — **TIER 3 — CONFIRMED, re-driven over BOTH cells of its class**

The help text, quoted from the reconciler's own binary (`jigc task finalize --help`, the `--dry-run` arg):

> It *refuses* on **three gates**: this task's validation findings, the empty-commit guard, and the
> carryover gate … On an **amend** task the last two read differently … **Its `findings` are the set
> `jigc task validate <id>` reports**, the staging-independent `owner-artifact` causes included —
> reported here, decided at the real finalize.

**Cell 1 — the amend arm's `finalize.base-mismatch`:**

```
setup: rig committed-singletons ; jigc task amend "moved head" ; author type+summary
       jigc milestone create "Move head wave"   -> exit 0, HEAD 398ffeb -> 638cb9c

argv : jigc task validate moved-head                          -> exit 0  (BARE)  "no findings — the task validates clean"
argv : jigc --format json task validate moved-head            -> exit 0  {"schema_version": 3, "findings": []}
argv : jigc task finalize moved-head --dry-run                -> exit 3  (BARE)
  blocking · finalize.base-mismatch — `HEAD` is no longer the commit this amend was minted against — it
    pinned `398ffebd…` and `HEAD` is now `638cb9cf…`, so the message this task authored would rewrite a
    different commit
    at: task:moved-head
    route: start the repair again against the commit that is there now (`jigc task discard moved-head
           --force`, then `jigc task amend`), or return `HEAD` to `398ffebd…` first
argv : jigc --format json task finalize moved-head --dry-run  -> exit 3
  findings[0].key = {"code":"finalize.base-mismatch","target":"task:moved-head"}
AFTER: HEAD 638cb9c (unmoved by any of the four)
```

**Cell 2 — the ordinary arm's `finalize.empty-commit`, the member that makes this pre-existing rather
than F-10's:**

```
setup: rig committed-singletons --start quick-fix "empty commit probe" ; author type+summary ; stage NOTHING
BEFORE: git diff --cached --name-only -> (empty) ; git status --porcelain -> (empty)

argv : jigc task validate empty-commit-probe                  -> exit 0  (BARE)  "no findings — the task validates clean"
argv : jigc --format json task validate empty-commit-probe    -> {"schema_version":3,"findings":[]}
argv : jigc task finalize empty-commit-probe --dry-run        -> exit 3  (BARE)
  blocking · finalize.empty-commit — task validated but produced no diff — nothing to finalize
    at: task:empty-commit-probe
    route: make a change, then re-run `jigc task finalize empty-commit-probe` — or, if the task is done
           with nothing to show, abandon it with `jigc task discard empty-commit-probe --force`
argv : jigc --format json task finalize … --dry-run           -> exit 3
  findings[0].key = {"code":"finalize.empty-commit","target":"task:empty-commit-probe"}
```

**The driver's own class correction is upheld.** Two of the gates `--dry-run` refuses on lie outside
`task validate`'s reported set — one on each arm — so the axis the fix is owed over is *the whole set
`--dry-run` refuses on*, not the amend cell. Tier 3: nothing lands, HEAD unmoved in every invocation, and
each refusal's route is runnable.

### C.2 · `A2-2` — **TIER 2 — CONFIRMED, both halves, including the route exiting 128**

```
setup: rig committed-singletons
       jigc milestone create "Preview wave" ; jigc milestone add-task preview-wave "pw one"
       jigc milestone provision preview-wave   -> "provisioned 1 worktree(s) … at base 5d4a27e (pw-one)"
       W=$REPO/.jigc/worktrees/pw-one ; git -C $W symbolic-ref -q HEAD -> (empty: DETACHED)
       cd $W

CELL B — an AMEND task minted there:
  jigc task amend "fanout preview"                 -> 0   (ack names `.jigc/worktrees/pw-one`)
  jigc task validate fanout-preview                -> exit 0  (BARE)  "no findings — the task validates clean"
  jigc --format json task validate fanout-preview  -> exit 0  {"schema_version": 3, "findings": []}
  jigc task finalize fanout-preview                -> exit 1  (BARE)
    blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no branch, and
      the next checkout would leave it unreachable
      route: re-attach HEAD with `git switch <branch>`, then re-run this command

CELL C — an ORDINARY (quick-fix) task minted in the SAME worktree, so the row is not amend-specific:
  jigc task validate plain-in-fanout               -> exit 0  "no findings — the task validates clean"
  jigc task finalize plain-in-fanout               -> exit 1  repo.head-detached

THE EMITTED ROUTE, RUN VERBATIM from the checkout that printed it:
  cwd = $REPO/.jigc/worktrees/pw-one
  argv: git switch main
  observed: fatal: 'main' is already used by worktree at '<ABS>/repo'
            rc=128
  git branch -a -> * (no branch) / + main          <- there is no <branch> the placeholder can take

CONTROL — an ordinary DETACHED LINKED worktree (git worktree add --detach), same binary:
  ordinary task: jigc task validate -> exit 1 repo.head-detached    <- BOTH doors refuse there
AFTER every cell: main HEAD unmoved, worktree HEAD unmoved
```

The control localizes it to the **`HeadDetached` × registered-fan-out-worktree** cell: `task validate`
reports clean while `task finalize` refuses, and the refusal's only offered exit exits 128 from the
checkout that printed it. Tier 2 on both legs (a preview that disagrees with its door; a route that
cannot run). Nothing lands and nothing is destroyed.

### C.3 · `A2-3` — **TIER 3 — CONFIRMED (a record row, not a behaviour row)**

The settle row, quoted verbatim from `completions/artifacts/M53/f10-amend-settle.md:58`:

> `jigc task amend` → the composed `{task, text}` arm (§1 of `command-output-contract.md`, unchanged
> shape); **`--format json` carries the pinned sha under `text`**, no new key needed — **verify**; if a
> key is needed it is declared in the additive-key paragraph.

```
setup: rig committed-singletons ; HEAD short = 8c73310, full = 8c733106ece180b875dba96493ddd2af1054617f
argv : jigc --format json task amend "sha probe"        -> exit 0
  keys              : ['task', 'text']      (the declared ArmShape::Object — the SHAPE is right)
  short sha in text : False
  full  sha in text : False
  "amending" in text: False
  text bytes        : 4170

CONTROL, the TEXT arm of the same door (fresh rig, HEAD 059018e):
  short sha in text arm: 1 hit      'amending:' in text arm: 1 hit
CONTROL, the RESUME doors on the TEXT arm:
  jigc start --task <id>           -> 2 hits of `amending`
  jigc workflow amend --task <id>  -> 2 hits of `amending`
CONTROL, the same resume door on the JSON arm:
  jigc --format json start --task <id>  -> 0 hits
CONTROL, an ORDINARY start — the ack is a text-arm-only header at every composing door:
  jigc start "ordinary ack probe" --workflow quick-fix              -> TEXT: "task minted:" ×1
  jigc --format json start "ordinary ack probe two" --workflow …    -> keys ['task','text'] ;
                                                                       "task minted:" in text: False
```

The binary is internally consistent — the ack is a text-arm header on **both** doors — so the false
statement is the settle's, on the one row it marked ***verify***. Tier 3: the JSON driver has a runnable
command (`git log -1 --format=%s`) for the one fact it is missing.

**One immaterial deviation from the driver, recorded for honesty:** the driver measured
`jigc workflow amend --task <id>` at **1** hit of `amending`; the reconciler measured **2**. Both are
non-zero, so the driver's claim (*the resume doors DO carry it on the text arm*) holds either way; the
count difference is not load-bearing and is not filed.

---

## D · The twenty rows re-driven to replace their missing repro blocks

Each row below carried no fenced repro block in the driver file (§A). Every one reproduced.

**Rows 20–23 — `amend.head-shape`, its four loci:**

```
UNBORN   (dev/jigc-rig --git-state unborn): jigc task amend "unborn probe"  -> exit 1
  blocking · amend.head-shape — `jigc task amend` rewrites a single-parent commit, and HEAD is unborn —
    this repository has no commit yet — nothing was minted
    at: work-unit:unborn-probe
    route: there is no commit to repair; make one first — `jigc start "<intent>"` composes the ordinary
           task that lands it
  .jigc/tasks -> ABSENT
NO INTENT (same shape): jigc task amend                                      -> exit 1
    at: work-unit:amend-4b825dc      (git's empty-tree sha — the id the mint would have taken)
ROOT     (unborn rig + one commit):  jigc task amend "root probe"            -> exit 1
    "… and HEAD is a root commit — it has no parent — nothing was minted"
    at: work-unit:root-probe ; route: `git commit --amend` ; .jigc/tasks -> ABSENT
MERGE    (committed-singletons + a --no-ff merge; parents of HEAD = 2):
         jigc task amend "merge probe"                                       -> exit 1
    "… and HEAD is a merge commit — it has 2 parents — nothing was minted"
    at: work-unit:merge-probe ; .jigc/tasks -> [] ; HEAD unmoved
```

**Row 29 — the content gate on the amend arm:**

```
setup: rig committed-singletons ; jigc task amend "incomplete doc" (commit doc left UNauthored)
argv : jigc task finalize incomplete-doc                                     -> exit 3
  blocking · schema-conformance.field-value-conformant — `commit:incomplete-doc`: field `type` … "" is not
    a member of enum "type" (allowed: feat, fix, docs, …)
  blocking · schema-conformance.required-slot-present — `commit:incomplete-doc`: required slot in section
    `summary` is empty
AFTER: HEAD af33421 unmoved
```

**Row 30 — cwd (b), a subdirectory:**

```
cwd  : $REPO/docs/deep
argv : jigc task amend "subdir probe"        -> 0  "task minted: subdir-probe" / "amending: 55bd789 …"
argv : jigc task finalize subdir-probe       -> 0  "amended 55bd789 → a7fef0f — fix: from a subdir"
AFTER: tree IDENTICAL
```

**Rows 31–33 — cwd (d), an ATTACHED linked worktree:**

```
setup: git worktree add -q $RIG/lw -b lwbr ; cd $RIG/lw   (linked HEAD a7fef0f)
argv : jigc task amend "linked probe"        -> 0
  amending: a7fef0f "fix: from a subdir"
    that is the `HEAD` of the linked worktree at `<ABS>/lw` on branch `lwbr` — not of the main checkout
    jigc's workbench binds to                                     <- LOW-7's sentence, present
argv : jigc task finalize linked-probe --dry-run   -> 0
  would rewrite a7fef0f … → "fix: in the linked checkout"
    would commit in the linked worktree at `<ABS>/lw` on branch `lwbr` — not in the main checkout …
ROW 33 — which index is the subject:
  plant a staged file in MAIN  : git -C $REPO add mainplant.txt
  jigc task finalize linked-probe                  -> 0   "amended a7fef0f → 01de1fa"
  main plant still staged after: [mainplant.txt]          <- main's index is NOT the subject, untouched
  plant a staged file in LINKED: git add lplant.txt
  jigc task finalize linked-probe-two              -> 3   finalize.amend-index-dirty   at: lplant.txt
```

**Row 37 — a repository root containing a SPACE** (hand-built: `mktemp -d '…/axis2 spaced.XXXXXX'`, `git
init`, two commits, `jigc setup`):

```
repo root: /private/var/…/axis2 spaced.LfnXXX/repo
setup: jigc task amend "spaced probe" ; author type+summary ; printf 'x\n' > "sp file.txt" ; git add "sp file.txt"
argv : jigc task finalize spaced-probe                                       -> exit 3
  blocking · finalize.amend-index-dirty — `sp file.txt` is staged, and an amend rewrites `HEAD` from the
    index — …
    route: unstage it (`git -C '/private/var/…/axis2 spaced.LfnXXX/repo' restore --staged -- 'sp file.txt'`)
           and re-run the finalize — this arm takes no `--carry-staged`, …
THE EMITTED ROUTE (BOTH operands single-quoted), RUN VERBATIM:
  cwd=$REPO -> rc=0        cwd=/ -> rc=0
then : jigc task finalize spaced-probe   -> 0   "amended 43ffea1 → 4814987 — fix: measured under a space"
```

**Rows 38–39 — amending jigc's own structural commits at HEAD** (allowed by design; driven harmless):

```
ROW 39, a milestone RECORD-ONLY commit:
  HEAD subject: "chore(milestone): open record for milestone:record-wave"
  jigc task amend "record repair" -> 0 ; jigc task finalize record-repair -> 0
    "amended 271819a → ae90064 — docs: the record commit reworded"
  AFTER: tree IDENTICAL · jigc validate -> 0 · jigc milestone add-task record-wave "rw one" -> 0
         jigc milestone list-tasks record-wave -> "milestone:record-wave tasks (1): rw-one"

ROW 38, a landed milestone BOUNDARY commit (provision -> stage code -> join -> milestone finalize):
  HEAD subject: "Finalize milestone bound-wave (1 sub-task)"
  jigc task amend "boundary repair" -> 0 ; jigc task finalize boundary-repair -> 0
    "amended 5961bfb → f0ad6b1 — chore: the boundary commit reworded"
  AFTER: tree IDENTICAL · record md5 INTACT · `base: 745a472c…` line intact ·
         jigc validate -> 0 · jigc doc show milestone-record:bound-wave -> 0
```

**Rows 40–41 — verb-routed, byte-identical at both doors:**

```
argv : jigc start --workflow amend "x"       -> exit 1
argv : jigc workflow amend --preview         -> exit 1
  blocking · workflow.verb-routed — workflow `amend` is not composed by name: verb-routed — reached only
    through `jigc task amend`, which pins the commit at HEAD and provisions the empty commit doc the amend
    renders; a router pick would compose with no pinned commit and finalize would have nothing to rewrite
    route: `jigc task amend`
  diff of the two outputs: BYTE-IDENTICAL
```

**Rows 42–47 — the read surfaces and the task-lifecycle cells:**

```
42 : jigc --format json task diff incomplete-doc  -> 0 ; keys ['base','code_diff','findings','op','staged_docs','task']
43 : jigc doc list --task incomplete-doc          -> 0 ; "commit:incomplete-doc  commit:incomplete-doc  managed"
44 : jigc doc show commit:incomplete-doc --task incomplete-doc -> 0 ; serves the task's staged copy (front matter)
45 : jigc start "a second thing" --workflow quick-fix beside a live amend task -> 0
     "also open: 1 other task was already open before this call — nothing here touched it; several open
      tasks are legal, each addressed by its own `--task`:"
47 : jigc task amend "incomplete doc"  (the same intent twice)                 -> exit 1
     blocking · task.serial-collision — task `incomplete-doc` is already active
       route: resume with `jigc start --task incomplete-doc` or abandon with `jigc task discard
              incomplete-doc --force`
46 : jigc task discard incomplete-doc --force     -> 0 ; HEAD unmoved · the task area is gone
```

---

## E · Counts after reconciliation

| | |
|---|---|
| **Tier-1 rows** | **0** |
| Driver defects, re-driven | **3** — `A2-1` (t3) · `A2-2` (t2) · `A2-3` (t3), all CONFIRMED, none contradicted by the source pass |
| Codex claims **CONFIRMED** (driven or measured) | **13** |
| Codex claims **REFUTED** | **0** |
| Codex claims left **OPEN LEAD** | **4** — B.13 · B.14 · B.15 (the re-probe half) · B.16 |
| Driver rows **demoted** | **0** (20 filed without a repro block; all 20 re-driven by the reconciler — §A, §D) |
| Baseline rows CLOSED | **6** — `N-1` · `N-2` · `F-1` · `(2, DEFECT A)` · `(2, DEFECT 1)` · M51 `D1`/`D2`/`D3`/`D3b` |
| Baseline rows STILL-OPEN(1.x, expected) | **5** — `DEFECT B` · `DEFECT C` (door axis +1) · `F-2` · `N-3` · `N-4` |
| Regressions | **0** |

**The two passes agree on every row they both reached.** The only asymmetries are directional: Codex
reached one row the driver had already driven (`F-2`), and the driver reached three the source pass never
looked at (`A2-1`/`A2-2`/`A2-3`) — two of which are *only* reachable by driving (a help text measured
against four gates; a route that is textually perfect and exits 128).

---

## F · Doors covered

Every clap leaf that is the door of at least one **driven** row in this reconciled file (`VERB_KINDS`
spelling), **35** of the registry's 48:

`setup` · `uninstall` · `migrate-corpus` · `rename` · `relocate` · `describe` · `validate` · `start` ·
`workflow` · `config set` · `doc create` · `doc add-item` · `doc remove-item` · `doc retitle-item` ·
`doc rename` · `doc set-field` · `doc set-slot` · `doc author` · `doc show` · `doc schema` · `doc list` ·
`task amend` · `task diff` · `task validate` · `task discard` · `task finalize` · `milestone create` ·
`milestone add-task` · `milestone add-from-spec` · `milestone list-tasks` · `milestone provision` ·
`milestone join` · `milestone execute` · `milestone finalize` · `milestone discard`

All **12** acting `BEHALF_DOORS` rows are among them, and so is the **11th `COMMITTING_DOORS` row** (the
amend arm of `task finalize`, driven by the reconciler at nine distinct cells). Leaves used only to build
fixtures or read state — `task list`, `config get`, `config list`, `ingest`, `migrate`, `unmanage`,
`upgrade`, the four `config *-step`/`fill`/`fork` leaves, `task bind` — are deliberately **not** claimed.

---

## G · Reconciler's instrument honesty

- Every exit code above was measured **bare**, to a file or `/dev/null`, never through a pipe. `command
  grep` was used for reads under `.jigc/`.
- Every rig root came from `mktemp -d` via `dev/jigc-rig`, evaluated in two steps
  (`rig=$(…) || exit; eval "$rig"`), never `eval "$(…)"`. **Nothing was torn down; `rm -rf` on a variable
  path appears nowhere in this reconciliation.** Roughly forty rigs were built.
- Three fixtures are not rig states and are named as such: a rejecting hook behind `core.hooksPath`, a
  deterministic single-process `git` shim on `PATH` (a *timing* fixture, not concurrency), and a second
  repository. One repository root was hand-built under a path containing a space, because no rig option
  expresses that.
- **Nothing was written into any `.jigc/` by hand. Nothing was fixed, committed or edited in the working
  repository.**
- `§B.6`'s registry counts were **parsed** out of the source (`BEHALF_DOORS`, `VERB_KINDS`,
  `COMMITTING_DOORS`, `AMBUSH_CONTRACTS`), not eyeballed; the rest of the driver's §2 table was not
  re-read and is not claimed by the reconciler.
- **Four Codex claims are left OPEN rather than promoted on the source read** (§B.13–§B.16). Three of
  them are *no-bypass* claims, so leaving them open asserts no defect; the fourth (B.15's absence half)
  was split out and confirmed on a complete grep because an absence over the source is measurable, while
  the re-probe half it sits beside is not.
- **The driver's §5 `AMBUSH_CONTRACTS` mutant work and its `--pack-from-dev` instrument warning were not
  re-driven.** They are observations, not defects, the source pass is silent on them, and no reconciled
  verdict rests on them. The registry read in §B.6 corroborates their subject only.
- **A2-2 carries the driver's own uncertainty forward unchanged.** Which door is wrong — the preview that
  says clean, or the seam that refuses — is not settled here; what is driven is the *divergence* plus the
  *unrunnable route*, which is true whichever side moves.
