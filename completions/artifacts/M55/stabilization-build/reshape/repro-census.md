# Reproduction census — the canary's committed reports

Read-only, 2026-10-10, branch `fix/rc24-tier1`. Evidence: `completions/artifacts/M55/stabilization-build/canary-one/`
(100 report files: 78 under `run/r1/reports/test/`, 22 under `run-pending-a2/`; `run/ledger.md`; `data/entries.tsv`,
`data/verified-attempt-1.tsv`, `data/verified-attempt-2-not-on-record.tsv`). Nothing was driven, built or edited.
Scripts and per-key tables are beside this file (`blocks.py`, `cls.py`, `cls2.py`, `keys.py`, `keys2.py`, `final2.py`,
`final3.py`, `best.json`).

## 1. The format in force

| where | line | what it says |
|---|---|---|
| `implementation/pinning.md` | 34–54 | §3 owns the block: a YAML mapping `claim / verdict / setup / repro / expect / pinned-by`, headed *"illustrative — flag: notation not frozen"*. `setup` is "a state name, or argv steps". |
| `implementation/pinning.md` | 54 | *"No script-runner harness: blocks are converted to Rust tests by hand … the block is a specification for a test, not a test format."* |
| `implementation/pinning.md` | 108 | *"Deliberately out: a YAML-repro runner."* |
| `implementation/milestone-completion-workflow.md` | 22 | Every verdict, CONFIRMED and REFUTED, arrives with a repro block (setup · argv sequence · expected exit/JSON assertions). |
| `implementation/stabilization-workflow.md` | 40 | Points at the two rows above; adds nothing of its own on shape. |
| `.claude/agents/milestone-code-reviewer.md` | 32 | A finding returns `repro` = *the heading of its repro block in your report*; what was not pursued is returned marked `lead`. |
| `.claude/agents/milestone-e2e-tester.md` | 11, 15, 27 | "Exact repro commands + observed output" per scenario; the same `repro` field. Mints roots with `mktemp -d`, no teardown. |
| `.claude/agents/finding-verifier.md` | 12, 16, 21, 25, 27 | Reads the block in the reporter's report; re-drives "from nothing"; *"if the block cannot be run as written, reconstruct the smallest setup"*; leaves a block in §3's schema and says whether it is pinnable; `left_open` is free text. |
| `.claude/agents/finding-triage.md` | 20, 37, 41 | Every `left_open` entry is triaged like a finding; a finding without a block is graded `unclear` and said to lack one; an entry carries `repro`. |
| `.claude/workflows/stabilize.js` | 1589–1603 | `FINDING.repro`: required **string**, "the heading of its repro block in your report". |
| `.claude/workflows/stabilize.js` | 1584, 1675, 1692, 1706 | `left_open` is an array of plain strings — no `repro` at all. |
| `.claude/workflows/stabilize.js` | 2983–2987 | A finding reaches triage as one line ending `· repro: <string>`; a left-open item as `left open N — <text>`. |
| `.claude/workflows/stabilize.js` | 1630–1638, 1465, 1973 | Triage returns `repro` (string) per entry; it becomes the ledger row; the verifier's prompt is handed `its repro block: <that string>`. |
| `dev/stabilize-record` | 92, 1303, 2133, 2146 | `repro` is a required **text** cell of a ledger row, column `repro pointer`; nothing parses or resolves it. |

**Plainly:** a reproduction today is a YAML block inside a markdown report, reached through a free-text pointer.
Nothing requires it to be a self-contained runnable script; `pinning.md` rules a runner out by name. Nothing checks
that the pointer resolves, and a left-open item carries no reproduction field at all.

## 2. The count

### 2a. Blocks (exact, scripted)

288 fenced blocks in the 100 reports. **None is a shell block** (`sh`/`bash`/`zsh`/`console`: 0). 157 are §3 blocks
(a `claim:` key).

| role (report files) | argv-pure | mixed | no product argv in `repro` | all |
|---|---:|---:|---:|---:|
| instrument reporters (7) | 5 | 24 | 26 | 55 |
| verifiers (80) | 22 | 57 | 19 | 98 |
| advocate (1) | 0 | 3 | 0 | 3 |
| independent drive (1) | 0 | 1 | 0 | 1 |
| **all** | **27** | **85** | **45** | **157** |

- *argv-pure*: every `setup` step is an argv array, `fixture: <state>`, `env:` or `cwd:`; every `repro` step is an argv
  array; a numeric exit is expected; no placeholder beyond `<repo>`/`<task>`/`<rig>`.
- *mixed*: argv steps beside prose strings or named acts (`write:`, `symlink:`, `plant:`, `remove:`, `shell:`).
- *no product argv*: the `repro` steps are shell lines on the agent's own variables (`$W`, `$ARM`, `$RUN`, `$OUT`, `$T`),
  prose, or the block's verdict is `NOT DRIVEN`.
- Every one of the 80 verifier reports leaves at least one block; 42 say *pinnable as it stands: yes*, 24 no, 4 partly,
  10 do not say.
- **The scripts existed and were not kept:** 50 of 80 verifier reports name a driving script of their own under the
  scratch root (`drive.sh` in 37, `lib.sh`, `mkrig.sh`, `cell.sh` …). No `.sh` file is committed under `canary-one/`.

### 2b. Keys (372 ledger rows, by the role that produced the item)

Unit: one ledger key (`run/ledger.md`), role from `data/entries.tsv` → `source_kind`. The 99 items the second
invocation's verifiers left open have no key and are not counted.

| role | R | N | L | all |
|---|---:|---:|---:|---:|
| instrument reporter (review row 16, trial arm 9, cross-cutting 2) | 0 | 20 | 7 | 27 |
| scope step | 0 | 0 | 6 | 6 |
| verifier (left-open) | 0 | 78 | 242 | 320 |
| advocate (left-open) | 0 | 3 | 8 | 11 |
| independent drive (left-open) | 0 | 2 | 5 | 7 |
| seeded at the opening | 0 | 1 | 0 | 1 |
| **all** | **0** | **104** | **268** | **372** |

**Which is which.** R = 0 is exact (no shell block exists). The N/L split is *scripted, with 129 keys read*: the
table uses my read where I read and the script elsewhere (script alone: N 103, L 269).

- The script's rule. **N** when the key's `repro pointer` names a §3 block (resolved to a block in the named report,
  or naming a cell/step/line of one) and does not say *no block*; **L** when the key says *not-driven / undriven /
  not-run / unexamined / not-enumerated …*, when the named block's verdict is `NOT DRIVEN`, when the key is about a
  pin, or when the pointer is `Left open, item N`, a heading, a paragraph or a table row.
- Read: a random sample of 46 stratified by role (12 · 3 · 24 · 3 · 3 · 1), all 79 keys a verifier was handed, and 16
  more verifier keys the script called N. Script against read: **124 of 129**; on the random 46: 44 of 46, read 13 N /
  33 L (28% / 72%), script 15 / 31.
- Under a strict reading of N ("commands are there") the 37 prose bullets that name one product command and an exit
  are still L: no setup command is written, the state is a rig or a cell of the agent's scratch root.

**L, by what it is** (268):

| kind | keys |
|---|---:|
| says something was not driven | 110 |
| prose claim, no command | 49 |
| prose bullet naming a product command and an exit, no block | 37 |
| about a pin, a test or a sentence of a doc | 38 |
| a section, a table row or a paragraph; no block | 32 |
| a block whose verdict is `NOT DRIVEN` | 2 |

**N, by what a tool cannot supply** (99 of the 104 N keys resolve to a block; a key has several):

| missing | keys |
|---|---:|
| it is not a script: a YAML spec with no runner | 99 |
| state built in prose or a named act (`write:`, `symlink:`, `plant:`) | 83 |
| a `repro` step in prose | 41 |
| a stdin payload carried in a comment or prose | 38 |
| no numeric exit expectation | 37 |
| a value an earlier command mints (`<task>`, an id read off stdout) | 30 |
| the working directory by a comment (`# in repo`) or `cwd:` | 26 |
| follows a printed route — the output must be read to know the next command | 25 |
| the agent's shell variables, or another block's state ("the setup of Repro RC-8") | 21 |
| another placeholder (`<blob>`, `<byte 0xFF>`, `<abs>`) | 14 |
| a platform or root (a case-folding volume, Linux, uid 0) | 8 |
| the trial image (docker) | 4 |
| a modified binary or script (the proposal applied, a mutant arm) | 4 |

Five N keys lack only a runner and a working-directory convention.

### Examples

**R** — none. The nearest:
- `r1-doc-list-unreadable-entry-fails-listing` → `run/r1/reports/test/row-doc-list-reconciler.a1.md`, `Repro RC-1`: all
  argv from `git init`, three exits expected; the working directory is the comment `# in repo`.
- `r1-doc-list-staged-arm-unreadable-entry` → same file, `Repro RC-2`: `fixture: refs-post-hoc`, one `ln -s`, two argv.
- `r1-p4-not-found-not-staged-routes-loop-at-resume-door` → `verify-p3-r1-p3-unreadable-entry-undriven-platform-shapes-and-doors.a1.md`, `Repro V-3c`.

**N**
- `r1-p3-setup-writability-route-printed-for-other-causes` → `verify-p2-r1-setup-install-hook-route-other-causes-unexamined.a1.md`,
  `Repro VR-2`: state by `write:`/`symlink:` acts with `<abs>` paths. Its sibling `Repro VR-1` has the step
  `remove: "what is at the path run 1's route prints"`.
- `r1-control-pass-counts-failed-read` → `arm-control-score.a1.md`, `S5`: shell lines on `$W`, `$ARM`, `$RUN`, `$T`; a
  mutant of the arm script; the trial image.
- `r1-setup-installs-outside-work-tree` → `row-doc-list-reconciler.a1.md`, `Repro RC-6`: setup is "the first four steps
  of Repro RC-5, with proj/CLAUDE.md … written before them".

**L**
- `r1-uninstall-log-append-through-live-link` → `cross-cutting-review.a1.md`, `Lead L3`: prose; the ledger's own pointer
  says *NO repro block*.
- `r1-p4-unregistered-doc-route-undriven-cells` → `verify-p3-r1-p3-unregistered-doc-route-after-unmanage-not-run.a1.md`,
  `Left open` item 5: four things, "none driven".
- `r1-doc-list-cells-driven-on-one-platform` → `row-doc-list-reconciler.a1.md`, `Repro LD-3`: a block,
  `verdict: NOT DRIVEN`, `expect: "not established"`.

## 3. What replay would have removed

Sorted by each verifier's own basis word (`does-not-reproduce` · `breaks-no-clause` · `intended`), read from the
`basis` column of the two `verified-*.tsv` (all 58 refutations read) and held against the verdict line of each of the
80 reports (64 by a `Verdict` heading, 16 by their verdict line; no disagreement). "Handed" is my reading of each
key's ledger pointer (79 keys).

| | first invocation (61) | handed N | handed L | second (19) | handed N | handed L |
|---|---:|---:|---:|---:|---:|---:|
| confirmed | 12 | 5 | 7 | 6 | 2 | 4 |
| refuted — did not reproduce | 10 | 1 | 9 | 0 | 0 | 0 |
| refuted — reproduced, not a break: `breaks-no-clause` | 33 | 10 | 23 | 9 | 0 | 9 |
| refuted — reproduced, not a break: `intended` | 5 | 2 | 3 | 1 | 0 | 1 |
| could not be driven (halted) | 1 | 0 | 1 | 3 | 0 | 3 |
| other | 0 | | | 0 | | |
| **all** | **61** | **18** | **43** | **19** | **2** | **17** |

- **Did not reproduce, with a block to replay: 1 of 80** — the planted `canary-seeded-claim`. The other 9 had no
  reproduction: the verifier drove cells nobody had driven (7 keys say *not driven*) and found no defect.
- **Reproduced, but not a break: 48 of 80** (38 + 10). In 42 of them the basis cites the same behaviour on the
  previous release (first: 29 of 33 and 4 of 5; second: 8 of 9 and 1 of 1) — the part a tool does. The verdict itself
  rests on a reading: no route is printed, the route works, the state is outside the clause's scope, a decision intends it.
- **No block was handed to 60 of 80 verifiers** (43 + 17). They returned **11 of the 18 confirmed** (7 + 4), **both
  regressions among them**, and all 4 halts.
- A block was handed to 20; 7 confirmed, 1 did not reproduce, 12 reproduced and were judged no break.
- This differs from the figure in the `DECISIONS.md` entry of 2026-10-10 (*6 of the 31 leads the canary verified were
  confirmed*). That figure sorts by the key's wording (`key_shape_heuristic` = coverage) over the first invocation;
  sorted by whether a block was handed, it is 7 of 43 there and 11 of 60 over both, with both regressions inside.

## 4. What a reproduction presumes (157 blocks)

| | blocks |
|---|---:|
| the binary as a bare `jigc` in argv, resolved by `PATH` | 131 |
| `$JIGC` | 0 |
| an absolute path to a binary in a step | 1 (inside the trial image) |
| a `binary:` key naming the candidate's sha256 — prose, not a step | 115 |
| no `jigc` call in the steps at all (trial tooling, greps, reads) | 16 |
| state from a rig: `refs-post-hoc` 46 · `fresh` 38 · `bare` 11 · `committed-singletons` 2 · `vendored` 2 | 99 |
| state from `git init` in the steps | 26 |
| neither a rig state nor `git init` (an earlier block's state) | 36 |
| names the isolation (`HOME`, `GIT_CONFIG_*`) | 43 |
| carries the previous release's result as a key | 84 |
| names the previous release's binary in a step | 13 |
| network (curl, clone, fetch, push, crates.io) | 0 |
| docker / the trial image | 12 (all trial-arm) |
| a `requires:` or `platform:` key | 35 |
| needs a case-folding or case-sensitive volume | 10 |
| names Linux or root as needed or not driven | 33 |
| `sudo` in a step | 0 |

- A rig is built by driving the binary, and `jigc setup` writes the binary's absolute path into the hook: a rig is per
  binary (`dev/jigc-rig --binary`). The rig's default is this tree's debug build.
- A printed route contains a bare `jigc`; with the wrong `PATH` it resolves to the installed release
  (`.claude/agents/milestone-e2e-tester.md:23`).

**What makes an unattended run on two binaries unsafe** (blocks):

| hazard | blocks |
|---|---:|
| follows a route the product prints (the next command comes from output) | 43 |
| `remove:` a path read off the product's output | 2 (both regressions' blocks) |
| `git add/commit/mv/rm/update-index` with the working directory given by a comment or a fixture | 76 |
| `--force`, `discard` or `uninstall` in the steps | 25 |
| a link leading out of the repository (`ln -s`, `symlink:`) | 28 |
| an absolute path outside any temp root: `/dev/null` 22 · `/nonexistent` 9 · `/usr` 1 · `/work` 1 | 32 |
| `chmod 000` / a mode-000 plant (a temp root that cannot be read back) | 16 |
| a hook planted in `.git/hooks` | 11 |
| `git worktree add` (all inside the rig) | 14 |
| a named pipe — the product blocks until killed ("an 8 s alarm") | 8 |
| this repository's path (`$C`) in a step — trial tooling read and run | 8 |
| `rm -r` in a step · `git config --global` · `sudo` | 0 · 0 · 0 |

- No block removes a variable path. The run's transcripts did: three verifiers were stopped on a driving script that
  ran `rm` (canary `README.md` §2).
- One unverified finding's route tells the reader to remove the link at `/dev/null`
  (`r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null`).
- `jigc setup` in a worktree of a bare repository writes into the parent folder (`Repro RC-6`): a root one level deep
  is written outside.

## 5. Stake

The 18 confirmed, from `findings.md` §1, §2, §4 and the two regressions' blocks:

| kind | count | keys |
|---|---:|---|
| **loss**, by a printed route followed — and **regression** | 2 | `r1-setup-install-hook-route-other-causes-unexamined`, `r1-p3-setup-install-hook-undriven-producers-and-causes` |
| **stuck** | 16 | the other ten of the first invocation, the six of the second |

- The two: the route names an outside script as the link; done as printed, a file "beside the repo, in no repository"
  is deleted (`Repro VR-1`: `after_the_route: "outside/shared-hook no longer exists"`); `1.0.0-rc.24` exits 0.
- Borderline among the 16: `r1-setup-installs-outside-work-tree` writes seven files and appends to a `CLAUDE.md`
  outside the work tree, and in a submodule rewrites the superproject's hook at exit 0 with the user's body kept.
  Nothing destroyed, nothing committed: stuck.
- `r1-p3-lowercase-address-other-doors-not-driven`: a write accepted at exit 0 that cannot land. Nothing destroyed: stuck.

**Does triage's output already tell the kind? No.** `data/entries.tsv` has `clause` (`no-lost-files` 83 ·
`working-product` 242 · `migration-works` 1 · `none` 46), `triage_grade` and a free-text `triage_why`.

- *Loss*: all 18 confirmed carry `working-product`, the two loss findings included. Of the 83 `no-lost-files` rows, 21
  were verified: 20 refuted, 1 halted, none confirmed.
- *Regression* against *stuck*: both are measures of the one clause `working-product`; no field separates them.
  `regression` is the verifier's cell, filled for 12 rows.
- `triage_why` of the two loss findings holds no word of deletion (searched: destroy, delet, lost, gone, overwrit,
  replac). The deletion was established by the verifier.
- Inside or outside the round's scope is already derivable: `door`, `names_the_round_door`.

## Not established

- Whether any block runs: nothing was driven. *Argv-pure* is a statement about its shape.
- The class of 243 of the 372 keys by reading — they are the script's; the 99 unkeyed left-open items of the second
  invocation were not classified.
- The scratch scripts (`drive.sh` …): only their names in the reports. Whether they were self-contained is unknown.
- The 47 raw findings of the seven instrument reporters before triage merged them into 27 keys were not classified
  one by one; the 55 instrument blocks are in 2a.
- Whether a `breaks-no-clause` verdict would have been reached without a model from the two binaries' outputs alone:
  42 of 48 cite the comparison, none rests on it alone as read from one line of basis.
- Verifier tokens per class: not computed.

---

# Follow-up: what predicts a verdict

Same data, same rules; nothing driven. Unit: the 80 verifier runs (61 of the first invocation, 19 of the second;
79 keys — one was run twice and halted twice). "Confirmed" is the verifier's verdict. Scripts: `fu.py`, `fuB.py`;
per-run rows: `runs.json`, `groups.json`.

## A. Parent

Parent = the report the item's ledger pointer names first; a verifier parent's verdict is its first-invocation verdict.

| what produced the item | confirmed / runs |
|---|---:|
| an instrument's own finding (6 with a block, 2 prose leads) | 1 / 8 |
| the scope step's left-open item | 0 / 1 |
| seeded at the opening | 0 / 1 |
| a verifier's left-open item | 17 / 70 |
| an advocate's or the drive's left-open item | 0 / 0 — none was verified |

| the parent verifier's own verdict | confirmed / runs |
|---|---:|
| confirmed | 9 / 18 |
| refuted — reproduced, not a break | 8 / 41 |
| refuted — did not reproduce | 0 / 11 |
| halted | 0 / 0 |

| door, against the parent verifier's item | confirmed / runs |
|---|---:|
| the same door | 7 / 23 |
| a different door | 10 / 47 |

By root instrument finding (the chain followed back): three roots — `RC-1`, `RC-6`, `RC-12` of
`row-doc-list-reconciler` — carry 16 / 34; the other seven roots 2 / 46 (`RC-2` alone: 0 / 19).

## B. About a route or not

Sorted by me from the handed text — the key and triage's one line in `data/entries.tsv` (`triage_why`) — for all 79
keys, every one read. **Not blind:** I had read the verdicts before sorting. Two blind proxies are given below it.

| group | confirmed / runs |
|---|---:|
| names a route that is said to fail, loop or misroute | 9 / 15 |
| names a route that was not run, or a refusal's other causes | 6 / 15 |
| **names a printed route — both** | **15 / 30** |
| a refusal that prints no route at all | 0 / 3 |
| states a behaviour, no route in it | 1 / 18 |
| says something was not driven, no route named | 2 / 28 |
| about a registry / record | 0 / 1 |
| **everything else — the four rows above** | **3 / 50** |

- Blind proxies: the key contains `route` 11 / 20, does not 7 / 60; triage's line contains `route` 12 / 27, does not 6 / 53.
- Route × parent: route and parent confirmed 7 / 10 · route and parent not-a-break 7 / 12 · route and parent
  did-not-reproduce 0 / 6 · no route and parent confirmed 2 / 8 · no route and parent not-a-break 1 / 29.
- Route × handed: route with a block 6 / 9, route without 9 / 21; no route with a block 1 / 11, without 2 / 39.
- Other fields available before launch: clause `working-product` 18 / 57, `no-lost-files` 0 / 22, `migration-works`
  0 / 1 · grade *breaks* 2 / 3, *unclear* 16 / 77 · names the round's door 0 / 12, does not 18 / 68.
- The three confirmed outside the route groups: `r1-p3-lowercase-address-other-doors-not-driven`,
  `r1-p3-doc-show-task-no-committed-copy-of-stranded-doc`, `r1-p4-not-staged-block-other-unservable-states-not-enumerated`.

## C. Cost by what was handed

Join key present: the `label` column, `verify:<key>`, in `data/agents-attempt-1.tsv` and
`data/agents-attempt-2-returned.tsv`; 80 of 80 joined.

| | runs | output tokens: median · mean · sum | minutes: median · mean · sum |
|---|---:|---|---|
| all | 80 | 44.9 k · 50.6 k · 4,051 k | 8.7 · 10.5 · 841 |
| handed a block (N) | 20 | 40.9 k · 39.5 k · 791 k | 7.9 · 7.9 · 158 |
| handed no block (L) | 60 | 47.5 k · 54.3 k · 3,260 k | 9.7 · 11.4 · 683 |
| confirmed | 18 | 44.7 k · 45.1 k · 812 k | 8.6 · 9.3 · 167 |
| did not reproduce | 10 | 40.2 k · 45.6 k · 456 k | 9.0 · 9.5 · 95 |
| reproduced, not a break | 48 | 45.2 k · 54.9 k · 2,637 k | 9.4 · 11.5 · 550 |
| halted | 4 | 35.8 k · 36.4 k · 146 k | 7.3 · 7.2 · 29 |
| N · confirmed | 7 | 40.4 k · 39.9 k · 279 k | 8.4 · 8.2 · 58 |
| N · did not reproduce (the plant) | 1 | 12.4 k | 2.5 |
| N · not a break | 12 | 41.5 k · 41.6 k · 499 k | 7.9 · 8.1 · 97 |
| L · confirmed | 11 | 46.6 k · 48.4 k · 533 k | 9.8 · 9.9 · 109 |
| L · did not reproduce | 9 | 46.8 k · 49.3 k · 444 k | 10.4 · 10.3 · 92 |
| L · not a break | 36 | 49.8 k · 59.4 k · 2,138 k | 9.9 · 12.6 · 453 |
| route named (B) | 30 | 43.4 k median · 1,308 k sum | 268 sum |
| no route named (B) | 50 | 45.6 k median · 2,742 k sum | 572 sum |

A verifier costs about the same whatever it was handed and whatever it returns; only the plant was cheap.

## D. The 48 "reproduced, not a break" verdicts: what settled each

Read: all 48, each verifier's one-line `basis` in full (384–895 characters) — not the report sections. One primary
leg per verdict; most carry a second.

| what settled it | verdicts | one agent with the handed text and both binaries' output: yes · needed more drives · cannot tell |
|---|---:|---|
| (i) the same on the previous release, and the second measure untouched — no route printed, or the route runs | 22 | 5 · 17 · 0 |
| (ii) documented or intended, a decision, doc or test cited | 7 | 1 · 6 · 0 |
| (iii) a sentence wider than the fact | 1 | 0 · 1 · 0 |
| (iv) outside the clause's scope: a planted state or an unsupported layout | 6 | 0 · 6 · 0 |
| (v) other — the first clause's terms not met on the facts: nothing destroyed, nothing committed unasked | 12 | 0 · 11 · 1 |
| **all** | **48** | **6 · 41 · 1** |

- *Needed more drives* is 36 times by construction: the item came with no block, so a tool had nothing to run. Of the
  12 that came with a block: 6 yes, 5 needed more (a control without the plant, the route run to its end, more arms),
  1 cannot tell.
- (iv): 4 cite the clause's own scope words (this repository's own migration; *deliberately planted states*), 2 rest
  on the verifier's reading of *supported layout* and say the run declares no bound. No verdict is settled by a
  declared bound of the run; 1, counted under (i), also cites a declared bound of `design/finalize.md`.
- 9 of the 48 say the verdict rests on a reading of the clause; 4 say that under the other reading it is confirmed
  (`r1-doc-list-unreadable-entry-undriven-shapes-and-consumers`, `r1-p3-doc-show-offers-unmanage-after-unmanage`,
  `r1-p4-milestone-staged-prose-route-not-staged-unreadable-doc`,
  `r1-p4-milestone-finalize-missing-provenance-route-no-command`).
- 42 of the 48 cite identical behaviour on the previous release.

## E. Depth

| triage pass the item entered in (= hops from an instrument finding + 1) | confirmed / runs |
|---|---:|
| 1 — instruments, scope, seed | 1 / 10 |
| 2 | 3 / 13 |
| 3 | 8 / 39 |
| 4 (second invocation) | 6 / 18 |

The 18 confirmed, each back to its instrument finding (all four roots are findings of `row-doc-list-reconciler`):

| root (its own verdict) | hops | confirmed |
|---|---:|---|
| `RC-6` `r1-setup-installs-outside-work-tree` (confirmed) | 0 | itself |
| | 1 | `r1-setup-install-hook-route-other-causes-unexamined` |
| | 2 | `r1-p3-setup-writability-route-printed-for-other-causes`, `r1-p3-uninstall-writability-route-not-driven`, `r1-p3-setup-install-hook-undriven-producers-and-causes` |
| `RC-12` `r1-doc-list-prints-id-doc-show-refuses` (refuted, intended) | 1 | `r1-doc-show-lowercase-address-prints-path-no-file-has`, `r1-adoption-route-for-non-doc-id-name-not-run` |
| | 2 | `r1-p3-migrate-case-variant-path-says-git-holds-no-copy`, `r1-p3-lowercase-address-other-doors-not-driven` |
| `RC-1` `r1-doc-list-unreadable-entry-fails-listing` (refuted, not a break) | 2 | `r1-p3-not-staged-route-task-less-read-refuses` |
| | 3 | the six of the second invocation |
| `LD-2` `r1-non-utf8-path-other-orphan-walk-consumers` (refuted, not a break) | 2 | `r1-p3-doc-show-task-no-committed-copy-of-stranded-doc`, `r1-p3-amend-refusal-route-spells-c-quoted-name-as-path` |

1 at 0 hops, 3 at 1, 8 at 2, 6 at 3. One instrument finding of the 47 was itself confirmed; 17 of 18 are a
verifier's left-open item: 9 of those 17 sit under a parent that was confirmed, 8 under one that was refuted.

## Not established (follow-up)

- A blind sort for B: mine was made after reading the verdicts. The two string proxies are blind and weaker.
- Whether "names a route" holds outside this run: one door, one reviewer's roots, 18 confirmed.
- The handed text for B is triage's line and the key, not the verifier's whole prompt (triage's *what to re-drive*
  is not in the committed tables).
- D's second column for the 36 without a block is a statement about what was handed, not about what a block would
  have allowed.
- D was sorted from the one-line basis; the report sections were not read for it.
- Cache-read and cache-write tokens per class: not computed (output tokens only).
