<!-- persisted verbatim 2026-09-10 from the evidence-check-1.0 session; agent: Claude Opus 5 (1M context); see VERDICT.md -->
# reviewB — the five surfaces no rc.14 trial reached

Binary: `/Users/maurice/projects/gherrink-jigc/target/release/jigc`, `jigc 1.0.0-rc.14`
(repo HEAD `bd348a83`). Every corpus built with `dev/jigc-rig <state> --binary <that path>`,
two-step eval, roots under
`/private/tmp/claude-501/.../scratchpad/reviewB`. No cargo was run. Nothing in the repo
was modified.

Coverage rows under test: `completions/artifacts/RC-rc14/coverage.md` rows **4**, **10b**,
**11a**, **11b** and **`1799a2d`**, plus side probe **F** (hook rejection at the milestone
boundary + the N20 repro).

## Verdict table

| cell | reached? | exit | matches contract? | note |
|---|---|---|---|---|
| **A** `1799a2d` — milestone boundary gates the transient commit doc | yes, end to end | **3** blocking / **0** positive | **yes** | Both author-required leaves named in one block, both sub-tasks, routes carry `--task <sub>`; HEAD 6→6. `feat:` with an empty subject never landed. `jigc task finalize` on the identical doc shape gives the identical code, message and exit 3. |
| **B** row 10b — a blocked `milestone join` | yes, both blocking classes | **1** | **yes**, with one defect | Block renders before the routing footer, never narrates a success; `--format json` carries `findings` + `schema_version: 3`. **But** on a `join.same-doc-clash` the `no docs staged from:` line (and the `no_docs_from` key) names sub-tasks that *did* stage the clashing doc — see Defect 1. |
| **C** row 11b — `squash: false` landing ack names every sha | yes, twice | **0** | **yes** | Ack listed all 3 shas with the paths each landed and a per-sub-task attribution line; verified byte-for-byte against `git rev-list`/`git show --name-only`. |
| **D** row 11a — the Fix-phase fan-out | yes, whole round, no LLM | **0** | **yes** | `config set … squash false` → `create` → `add-task --workflow fix-task` ×2 → `provision` → `execute` → both fixers on their own emitted `Spawn:` lines → `join` → `finalize`. One commit per fix, each owning exactly its partition's path; the config layer rode the aggregate; zero residue. |
| **E** row 4 — `ROOT_KNOBS` + `uninstall`'s third subject | yes, 2 knobs × 12 values, + uninstall | **1** refused / **0** legal | **yes**, with one wording over-claim | 7 shapes refused on both knobs with 3 distinct codes; manifest byte-unchanged after every refusal; `validate` exit 0 and `doc list` == `git ls-files` after **every** step. `uninstall.untracked-workbench-file` refuses, names each file, destroys nothing, and clears after remediation. Over-claim: see Defect 2. |
| **F** (side) hook rejection at `milestone finalize` | yes | **1** | **yes** | Survivable frame intact: verbatim hook stderr, state-truth clause, re-run argv; HEAD unmoved, both worktrees + staged code intact, `error_code: "milestone-finalize.chain-commit-rejected"` in the invocation log. Recovery re-run lands at exit 0. |
| **F/N20** the non-hook milestone refusal | yes | **1** | **reproduces, and is wider than recorded** | Bare `git merge --ff-only <sha>` failed: …`, no code, no route, no state clause, `error_code: null`. State-safe. Reproduces on **`squash: false` as well as `squash: true`** — the recorded entry scopes it to `squash: true`. |

---

## A — `1799a2d`, the milestone boundary gate

Corpus: `dev/jigc-rig fresh`, plus `src/alpha.rs` + `src/beta.rs` committed.
`finalize.fan-out.squash` set `false`.

```
jigc config set finalize.fan-out.squash false          exit 0
jigc milestone create "Cache rework"                   exit 0   → milestone:cache-rework, base ae5ae5e
jigc milestone add-task cache-rework "Doc area"        exit 0   → task:doc-area
jigc milestone add-task cache-rework "Code area"       exit 0   → task:code-area
jigc milestone list-tasks cache-rework                 exit 0   → code-area, doc-area
jigc milestone provision cache-rework                  exit 0   → 2 worktrees at base ae5ae5e
jigc milestone execute cache-rework                    exit 0   → 2 Spawn: lines
```

Then in each worktree (`cd .jigc/worktrees/<sub>`), `jigc workflow sub-task --task <sub>`
to provision the transient commit doc, real code `git add`ed, and **one** author-required
leaf deliberately left unfilled per sub-task:

* `code-area` — `scope` + `summary` set, **`type` left unset**
* `doc-area` — `type` set, **`summary` left unset**

`jigc task validate code-area` → exit **3**, `schema-conformance.field-value-conformant`.
`jigc task validate doc-area` → exit **3**, `schema-conformance.required-slot-present`.

```
$ jigc milestone join cache-rework
joined milestone:cache-rework — 2 doc(s) merged
  - commit:code-area  (created · from code-area)
  - commit:doc-area  (created · from doc-area)
exit 0

$ jigc milestone finalize cache-rework
blocking · schema-conformance.field-value-conformant — `commit:code-area`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  at: commit:code-area#header/type
  route: `jigc doc set-field commit:code-area#header/type --task code-area --value <value>` to correct the value
blocking · schema-conformance.required-slot-present — `commit:doc-area`: required slot in section `summary` is empty
  at: commit:doc-area#summary · line 9
  route: `jigc doc set-slot commit:doc-area#summary --task doc-area --from-file -` to fill the empty slot
exit 3
```

`git rev-list --count HEAD` = **6 before, 6 after**. `git log` unchanged. No `feat:` with an
empty subject, and no commit at all.

**Positive case**, same milestone, both leaves filled through the routes the block printed:

```
$ jigc milestone finalize cache-rework
finalized 2fd9d87 — Finalize milestone cache-rework (2 sub-tasks)
  ...
exit 0
```
6 → 9 commits (2 per-sub-task + 1 aggregate).

**Comparison with `jigc task finalize` on the same doc shape** (fresh corpus, plain
`dev-task`, `src/a.rs` staged):

```
$ jigc task finalize tweak-alpha          # type unset
blocking · schema-conformance.field-value-conformant — `commit:tweak-alpha`: field `type` … is not a member of enum "type" …
exit 3 · HEAD unmoved

$ jigc task finalize tweak-alpha          # summary emptied
blocking · schema-conformance.required-slot-present — `commit:tweak-alpha`: required slot in section `summary` is empty
exit 3 · HEAD unmoved
```

Identical code, identical message shape, identical exit at both doors.
`design/validation.md`'s "gates as hard as the two task doors" holds on this binary.

---

## B — row 10b, a blocked `milestone join`

### B.1 `combine.code-collision` (both worktrees stage `src/shared.rs`)

```
$ jigc milestone join collide            # streams merged
blocking · combine.code-collision — code collision — `src/shared.rs` (sub-tasks [code-area, doc-area]) staged by more than one worktree; the combine disjoint-applies code and never text-merges a shared file
  at: src/shared.rs
  route: have the contending sub-tasks touch distinct files, or combine their overlapping changes by hand
join blocked: milestone:collide — 1 blocking finding(s); 0 doc(s) would merge, nothing committed
  no docs staged from: code-area, doc-area
— jigc · run `jigc start` for orientation; all writes through `jigc`.
exit 1
```

Order on the merged stream: **finding block → `join blocked:` verdict → routing footer.**
The headline never opens `joined milestone:…`. Identical in `--format human`.
Stream discipline: the finding rides stderr, the verdict + footer stdout.

`--format json` (stdout) carries `findings[]` with `check` / `code` / `key{code,target}` /
`location` / `message` / `probe` / `route` / `severity`, plus `milestone`, `no_docs_from`,
`overlay`, `schema_version: 3`. Exit 1.

`jigc milestone finalize collide` over the same state: same finding on stderr, **empty
stdout**, exit 1, HEAD 6 → 6.

### B.2 `join.same-doc-clash` (both sub-tasks edit committed `vision`)

```
blocking · join.same-doc-clash — same-doc clash — sub-tasks [alpha-area, beta-area] each write `vision:vision` at the milestone base; the join never blind-merges a shared managed doc
  at: vision:vision
  route: have the contending sub-tasks edit distinct docs, or merge their intent by hand
join blocked: milestone:doc-clash — 1 blocking finding(s); 0 doc(s) would merge, nothing committed
  no docs staged from: alpha-area, beta-area
exit 1
```

Block-before-footer holds. The `no docs staged from:` line does not — see Defect 1.

### B.3 the mixed control (one sub-task stages a doc, a *code* collision blocks)

```
join blocked: milestone:mixed — 1 blocking finding(s); 1 doc(s) would merge, nothing committed
  - commit:aaa-area  (created · from aaa-area)
  no docs staged from: bbb-area, ccc-area
```

Truthful here. So the lie is specific to the same-doc-clash short-circuit, not to blocking
joins generally.

---

## C — row 11b, the `squash: false` landing ack

Two independent runs. Run 1 (cell A's positive finalize):

```
$ jigc milestone finalize cache-rework
finalized 2fd9d87 — Finalize milestone cache-rework (2 sub-tasks)
  added .jigc/config/manifest.yaml
  modified docs/milestone-records/cache-rework.md
  modified src/alpha.rs
  modified src/beta.rs
  4 files committed
  commits (oldest first, each with the paths it landed):
    29fd113 feat(cache): rework the cache path
      src/beta.rs
    dbe61d5 feat: add the alpha helper
      src/alpha.rs
    2fd9d87 Finalize milestone cache-rework (2 sub-tasks)
      .jigc/config/manifest.yaml
      docs/milestone-records/cache-rework.md
  sub-tasks: code-area: 1 doc, 1 code file, committed as 29fd113 · doc-area: 1 doc, 1 code file, committed as dbe61d5
exit 0
```

Read back off git, not off the ack:

```
$ git log --format='%h %s' -3
2fd9d87 Finalize milestone cache-rework (2 sub-tasks)
dbe61d5 feat: add the alpha helper
29fd113 feat(cache): rework the cache path
$ for s in $(git rev-list --reverse -n 3 HEAD); do git show --name-only --format= $s; done
src/beta.rs
src/alpha.rs
.jigc/config/manifest.yaml  docs/milestone-records/cache-rework.md
```

Every sha in the ack is on HEAD, in the stated order, owning exactly the stated paths.
`git status --porcelain` empty. Run 2 (cell D) reproduced the same shape with different
shas. Contract met.

---

## D — row 11a, the Fix-phase fan-out

The entry point is the composed methodology walk, not a verb. `jigc workflow completion
--preview` (exit 0) emits the round verbatim at lines 65–71:

```
jigc config set finalize.fan-out.squash false
jigc milestone create "<the fix round>"
jigc milestone add-task <fix-milestone-id> "<the finding>" --workflow fix-task
jigc milestone provision <fix-milestone-id>
jigc milestone execute <fix-milestone-id>
jigc milestone join <fix-milestone-id>
jigc milestone finalize <fix-milestone-id>
```

with the partition rule as prose above it ("Partition the confirmed findings by each fix's
projected write-set… A finding spanning two files is one sub-task; two findings landing in
one file are one sub-task… Keep shared artifacts out of the partition and apply them
serially after the join… If a round's write-sets cannot be made disjoint… fall back to
running the fixers one at a time").

Driven verbatim, substituting only the two placeholders, over a two-partition fixture
(`src/low.rs`, `src/zed.rs`):

```
jigc config set finalize.fan-out.squash false                                     exit 0
jigc config get finalize.fan-out.squash        → finalize.fan-out.squash = false  (project)
jigc milestone create "Audit fix round"                                           exit 0
jigc milestone add-task audit-fix-round "the low cache path truncates" --workflow fix-task   exit 0
jigc milestone add-task audit-fix-round "the zed cache path truncates" --workflow fix-task   exit 0
jigc milestone provision audit-fix-round                                          exit 0
jigc milestone execute audit-fix-round                                            exit 0
  Spawn: `cd .jigc/worktrees/low-cache-path-truncates && jigc workflow fix-task --task low-cache-path-truncates`
  Spawn: `cd .jigc/worktrees/zed-cache-path-truncates && jigc workflow fix-task --task zed-cache-path-truncates`
```

Each fixer run in its own worktree on its own `Spawn:` line: `jigc workflow fix-task --task
<id>` (exit 0 — its composed body carries the axis-derivation instruction, the red-test
rule, the "`git add` … inside THIS worktree" rule and "Never `git commit` and never `jigc
task finalize` here"), then the fix applied, `git add`ed, and the commit doc authored.

```
$ jigc milestone join audit-fix-round
joined milestone:audit-fix-round — 2 doc(s) merged
  - commit:low-cache-path-truncates  (created · from low-cache-path-truncates)
  - commit:zed-cache-path-truncates  (created · from zed-cache-path-truncates)
exit 0

$ jigc milestone finalize audit-fix-round
finalized 73377df — Finalize milestone audit-fix-round (2 sub-tasks)
  commits (oldest first, each with the paths it landed):
    2ea7055 fix: src/low.rs stops truncating      src/low.rs
    15e22dd fix: src/zed.rs stops truncating      src/zed.rs
    73377df Finalize milestone audit-fix-round …  .jigc/config/manifest.yaml docs/milestone-records/audit-fix-round.md
exit 0
```

`git status --porcelain` empty; `jigc config get finalize.fan-out.squash` still `false
(project)`; the aggregate carries `.jigc/config/manifest.yaml`, so the knob write that
landed uncommitted was picked up by the round's own boundary.

**Where an LLM is needed and what I authored instead.** Exactly two things in the round are
prose: (1) the *partition decision* — which findings share a write-set — is stated as a rule
in the step body and is the agent's judgment, not a primitive the CLI computes; the CLI's
enforcement of a bad partition is `combine.code-collision` at the join, which cell B.1
drives directly. (2) each fix's `commit:<sub>#summary` slot. I authored (1) as the
two-partition split above and (2) as the one-line summaries shown. Nothing else in the
walk needed prose; every other step is a `jigc` command the composed bytes print.
`fix-finding` is a **step** (`packs/methodology/steps/fix-finding.yaml`) included by the
`fix-task` workflow, not a verb — there is nothing separate to drive.

---

## E — row 4, `ROOT_KNOBS`

Corpus: `dev/jigc-rig committed-singletons` + a real `adr:use-a-queue` created through
`record-decision` and committed to `docs/decisions/use-a-queue.md`, plus `realdir/`,
`plainfile.md` and `linked -> realdir`.

Baseline: `docs-root = docs/  (pack-default)`, `placement-root =   (pack-default)`.

Every value below driven on **both** `docs-root` and `placement-root`:

| value | exit | code | manifest after |
|---|---|---|---|
| `.jigc` | 1 | `config.workbench-root` | unchanged |
| `.jigc/foo` | 1 | `config.workbench-root` | unchanged |
| `.git` | 1 | `config.untrackable-root` | unchanged |
| `.git/foo` | 1 | `config.untrackable-root` | unchanged |
| `/tmp/elsewhere` (absolute) | 1 | `config.unusable-root` | unchanged |
| `linked` (symlink to a dir) | 1 | `config.unusable-root` | unchanged |
| `linked/sub` (through a symlink) | 1 | `config.unusable-root` | unchanged |
| `plainfile.md` (regular file) | 1 | `config.unusable-root` | unchanged |
| `..` | 1 | `config.untrackable-root` | unchanged |
| `""` | **0** | — | folds to `.`, docs relocated |
| `.` | **0** | — | legal; idempotent when already `.` |
| `nonexistent-dir` | **0** | — | legal; docs relocated, dir created |

Representative refusals, verbatim:

```
$ jigc config set docs-root .jigc
blocking · config.workbench-root — `.jigc` is inside jigc's own workbench (`.jigc/`) — `docs-root` cannot home managed docs in the tree `jigc uninstall` removes whole
  route: re-run with a root outside `.jigc/` — the workbench holds jigc's own state, not your documents; `jigc config list` shows the value in force and the layer it wins from
exit 1

$ jigc config set docs-root .git
blocking · config.untrackable-root — `.git` cannot be the `docs-root`: `.git` is inside git's own directory — git refuses to track any path with a `.git` component (`error: invalid path`), so the bytes would survive only in history
exit 1

$ jigc config set docs-root /tmp/elsewhere
blocking · config.unusable-root — `/tmp/elsewhere` cannot be the `docs-root`: an absolute path — every door resolves a root against the repository root, so the docs would move to `tmp/elsewhere` while `jigc config get` reads back `/tmp/elsewhere`
exit 1

$ jigc config set docs-root linked
blocking · config.unusable-root — `linked` cannot be the `docs-root`: `linked` is a symlink — git records the link, not a path through it, so the moved docs would stage as index entries the worktree has no path for (`RD`) and `jigc doc list` would name a home git does not
exit 1

$ jigc config set docs-root plainfile.md
blocking · config.unusable-root — `plainfile.md` cannot be the `docs-root`: `plainfile.md` is a file, not a directory — every move into it fails while the knob lands anyway, and the store then resolves its docs to homes nothing is at
exit 1

$ jigc config set docs-root ..
blocking · config.untrackable-root — `..` cannot be the `docs-root`: `..` resolves outside the repository root
exit 1
```

**Store agreement after every one of those refusals** (checked per step, not only at the
end): `jigc validate` exit **0**; `jigc doc list` paths ==
`CHANGELOG.md docs/decisions-log.md docs/decisions/use-a-queue.md docs/roadmap.md VISION.md`
== the same set from `git ls-files`; `git status --porcelain` **empty**; `.jigc/config/manifest.yaml`
**absent** (never written). Nothing moved, nothing staged, no partial write at any refusal.

Legal sets moved the docs and kept the store in agreement:

```
$ jigc config set docs-root ""
relocating 1 committed doc(s) stranded by the `docs-root` re-point to `.` (…):
  - docs/decisions/use-a-queue.md → decisions/use-a-queue.md
config: set `docs-root` = `.` — written to `.jigc/config/`, uncommitted …
exit 0
   doc list == git ls-files (decisions/use-a-queue.md), validate exit 0

$ jigc config set docs-root newhome     → decisions/use-a-queue.md → newhome/decisions/use-a-queue.md, exit 0, in agreement
$ jigc config set docs-root .           → newhome/… → decisions/use-a-queue.md, exit 0, in agreement
```

`placement-root ""` (folds to `.`) relocated `docs/decisions-log.md → decisions-log.md` and
`docs/roadmap.md → roadmap.md` as staged `git mv`s; `placement-root .` when already `.` was
a clean no-op with no moves; `placement-root nonexistent-dir` relocated both again. After
every one, `doc list` == `git ls-files` and `validate` exit 0.

### E.2 `uninstall.untracked-workbench-file`

The untracked workbench file planted through a real door (`jigc config set docs-root docs2`
writes an uncommitted `.jigc/config/manifest.yaml`), then a hand-written one added:

```
$ jigc uninstall
blocking · uninstall.untracked-workbench-file — `.jigc/` holds 1 file(s) that no index has a copy of — removing `.jigc/` would destroy them:
  .jigc/config/manifest.yaml
  route: put them where they can be recovered (`git add <path>` is enough — the index keeps a copy `git checkout -- <path>` restores) or delete the ones you do not need, then re-run `jigc uninstall`; `jigc uninstall --force` deletes them with the install
exit 1
```
`.jigc/` still present, `manifest.yaml` byte-intact.

With a second file planted it names **both** and says `2 file(s)`. `git add`ing the config
narrows it to `.jigc/scratch-note.txt` alone. Removing that file lets it through:

```
$ jigc uninstall
warning: removing `.jigc/` also removes 6 tracked file(s) under it:
    .jigc/.gitignore  .jigc/AGENT.md  .jigc/config/.gitkeep  .jigc/config/manifest.yaml  .jigc/config/packs.yaml  .jigc/version
  note: each is in the index, so `git checkout -- <path>` brings it back.
jigc uninstall — repo-local install removed
  - removed .jigc/ … - removed jigc guide artifact
exit 0
```

Refuse → name → route → remediate → proceed. Contract met.

---

## F — the survivable frame at the milestone boundary, and N20

### F.1 hook rejection, `squash: false`, 2 sub-tasks, both with code + a conformant commit doc

`.git/hooks/pre-commit` replaced with `echo "policy: commits are frozen for the release window" >&2; exit 1`.

```
$ jigc milestone finalize hooked
`git commit` was rejected (no commit was made):
policy: commits are frozen for the release window

milestone:hooked is intact — nothing was committed, HEAD is at its pre-finalize commit, and every provisioned sub-task worktree still holds its staged code. Fix the hook's complaint, then re-run `jigc milestone finalize hooked`.
exit 1
```

Every clause of the survivable-frame rule is present and **true when checked**:

* verbatim hook stderr — yes
* state-truth clause — HEAD `bda78e5` before and after; `git rev-list --count` 6 → 6
* worktrees intact — `aaa-area`, `bbb-area` both live in `git worktree list`
* staged code intact — `git -C .jigc/worktrees/aaa-area diff --cached --name-only` = `src/a.rs`; `bbb-area` = `src/b.rs`
* copy-runnable re-run argv — `jigc milestone finalize hooked`
* milestone record untouched — `status: active`, both sub-tasks `status: active`
* invocation log (after `jigc config set invocation-log true`, at `.jigc/logs/invocations.jsonl`, **not** `.jigc/log/`):

```json
{"timestamp":"…","argv":["milestone","finalize","hooked"],"exit_code":1,"duration_ms":1528,
 "finding_codes":[],"output_bytes":330,"binary_version":"1.0.0-rc.14",
 "error_code":"milestone-finalize.chain-commit-rejected"}
```

The identity is the per-door `milestone-finalize.chain-commit-rejected`, not a shared
`finalize.commit-rejected` — consistent with M47's "one error identity per door". Removing
the hook and re-running landed the boundary at exit 0 with all three commits.

### F.2 N20 reproduces — and is wider than recorded

`implementation/decisions-pending.md:563` records N20 as *"A `squash: true` fan-out whose
fast-forward collides with ordinary main-checkout WIP"*.

`squash: true` (default), 2 sub-tasks with staged code, plus `printf 'WIP\n' >> src/a.rs`
in the main checkout:

```
$ jigc milestone finalize ffwd
`git merge --ff-only 127e3227c6fba581d3ff866707850302d390c4fa` failed: error: Your local changes to the following files would be overwritten by merge:
	src/a.rs
Please commit your changes or stash them before you merge.
Aborting
exit 1
```

No finding code. No `route:`. No state-truth clause. No re-run argv.
`"error_code": null` in the invocation log — the door has no identity there either.
State-safe as recorded: HEAD unmoved, WIP intact, both worktrees and their staged code
intact; `git stash push -- src/a.rs` and a re-run landed the boundary at exit 0.

**The widening:** the same probe with `finalize.fan-out.squash false` produces the
byte-identical bare failure, exit 1, `error_code: null`:

```
$ jigc config set finalize.fan-out.squash false && jigc milestone finalize ffwd2
`git merge --ff-only 942b479ac9d30084249c4f2354e506d0044e90f7` failed: error: Your local changes to the following files would be overwritten by merge:
	src/a.rs
Please commit your changes or stash them before you merge.
Aborting
exit 1
```

So N20 is not a `squash: true` arm; it is the boundary's fast-forward step on **both**
knob settings — including the setting a Fix round runs under, which is the case the entry's
own last sentence gestures at without measuring.

---

## Defects found

**1. `milestone join`'s `no docs staged from:` line and its `no_docs_from` key are false on a
`join.same-doc-clash`.** Exit 1, not 0 — no data is lost — but the surface states the
opposite of what happened.

Driven: two sub-tasks each `jigc doc set-slot vision#thesis --task <sub>` on the committed
`vision`. Both stage a copy (`copied in for update` acked at exit 0), and the disk agrees:

```
$ find .jigc/tasks -maxdepth 3 -name '*.md'
.jigc/tasks/alpha-area/docs/vision:vision.md
.jigc/tasks/beta-area/docs/vision:vision.md
$ jigc doc list --task alpha-area
vision:vision  VISION.md  managed
$ jigc doc list --task beta-area
vision:vision  VISION.md  managed
```

Yet:

```
join blocked: milestone:doc-clash — 1 blocking finding(s); 0 doc(s) would merge, nothing committed
  no docs staged from: alpha-area, beta-area
```
and `"no_docs_from": ["alpha-area","beta-area"]`, `"overlay": {}` in `--format json`.

Contradicts, by name:

* `design/command-output-contract.md:403` — the declared meaning of the key it minted:
  *"its text echoes the milestone id and **names every sub-task that staged no doc**"*.
  Both named sub-tasks staged one.
* `design/surface-contract.md` law 1 (*nothing lies*).
* `crates/cli/src/render.rs:3841-3843`, the derivation's own doc-comment: *"the set …
  names and its envelope's `no_docs_from` key carries. One derivation for both surfaces, so
  the text and the wire cannot disagree about **who staged nothing**."* They agree with each
  other and both disagree with the disk.
* `crates/cli/src/render.rs` ~3790, the block-path comment: *"The overlay is still reported
  — under a block it is the merge the join *would* have produced, which is exactly the
  diagnostic the contending sub-tasks are read against."* On the same-doc-clash path the
  overlay is `{}`, so that stated intent does not hold and `doc_less_sub_tasks`
  (`render.rs:3844`, `outcome.overlay.values()`) degenerates to "every sub-task".

Scope: specific to the same-doc-clash short-circuit. The `combine.code-collision` path
computes the overlay and reports truthfully (control run B.3: `1 doc(s) would merge` /
`no docs staged from: bbb-area, ccc-area`, both correct). The two blocking join classes
therefore disagree about whether the overlay is populated under a block — which is the axis,
not the instance.

**2. `docs-root` relocation ack over-claims its subject.** Exit 0, behaviour correct, wording
wrong.

```
relocating 1 committed doc(s) stranded by the `docs-root` re-point to `.` (every committed doc
under the prior resolved root, managed or not; each move is a staged `git mv` …):
  - docs/decisions/use-a-queue.md → decisions/use-a-queue.md
```

The prior resolved root was `docs/`. Three committed `.md` files sat under it —
`docs/decisions/use-a-queue.md`, `docs/decisions-log.md`, `docs/roadmap.md`. One moved. The
other two are **placement** doctypes resolving through `placement-root`, so leaving them is
the *right* behaviour (`CLAUDE.md` → the placement branch; `design/storage.md` → Placement);
the parenthetical "every committed doc under the prior resolved root, managed or not" is
what is wrong, and it is exactly the sentence an operator reads to decide whether the move
was complete. Low severity, surface-tier, reversible.

**3. N20's recorded scope is too narrow** (see F.2). Not a new defect — a correction to an
already-carried one: the entry keys the loss of the survivable frame to `squash: true`, and
it is knob-independent.

Nothing found at exit 0 that loses data or corrupts a doc.

## Could not reach, and why

* **Nothing.** All five chartered cells and the side probe ran to completion on the real
  release binary.
* **One thing driven by hand rather than by an agent:** the *partition decision* in cell D
  (which findings share a write-set) is prose the fix-gate step instructs an agent to make;
  the CLI computes no partition. I made it (two disjoint single-file partitions) and drove
  everything downstream. Its negative — a partition that is *not* disjoint — is driven
  separately and mechanically in cell B.1 as `combine.code-collision`, so the enforcement
  half is measured even though the judgment half is not automatable.
* **Two commit-summary slots** (cell A `doc-area`, cell D ×2) needed prose; I authored
  one-line summaries. Every other step in every cell is a `jigc` invocation.
* **`.jigc/log/` does not exist**; the invocation log is opt-in at
  `jigc config set invocation-log true` and lands at `.jigc/logs/invocations.jsonl`. Cell F's
  first run was therefore unlogged and was re-run with the knob on to read the identity.
