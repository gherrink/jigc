# row-doc-list — the driver's table (run canary-one, round 1, stage test, attempt 1)

Reporter `row-doc-list-driver`. Item `row-doc-list` (review-row, clause `no-lost-files`). The one
door of the round's test set this item covers, in the door list's own words: `jigc doc list`.

## Verdict in one paragraph

On the candidate, `jigc doc list` wrote, removed and committed **nothing** in every one of the 86
invocations whose before and after manifests were compared one by one (the count is the
`*.man.before` files under `<W>/*.runs` — 120 labels, less the 34 on the previous release, less
one `jigc task list`, plus Row 1.1, compared by hand) and in the 24 more run in three groups
of eight, with one declared exception: where the opt-in
`invocation-log` knob is switched on, each invocation appends one JSONL record to the gitignored
`.jigc/logs/invocations.jsonl` (the measurement log, created on demand, earlier bytes kept as a
prefix). `git status --porcelain` was identical before and after every invocation with the knob
off. **The seeded claim `canary-seeded-claim` is REFUTED**: the untracked `notes.md` stands,
byte-identical, after an exit-0 `jigc doc list` in a set-up repository. Three findings and four
leads follow; none of them is a loss of bytes by the listed door, none is a regression against
the previous release (each was driven on both binaries and behaves the same), and two of the
three findings are already on record in `implementation/decisions-pending.md` → (D).

What I did **not** run is listed under *Not driven*, at the end.

## The binary, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch root> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash of the BINARY line (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- The candidate's directory went first on `PATH`; `command -v jigc` printed the candidate's path.
  `jigc --version` prints `jigc 1.0.0-rc.24` on both binaries, as the contract says it does —
  nothing below is keyed by it.
- No `cargo build`, nothing under `target/`. The one rig was built with
  `dev/jigc-rig --binary <candidate> refs-post-hoc`, stdout captured alone, two-step eval.

## How every row was measured

Host: macOS 26.6.2, git 2.54.0 (Apple Git-157). Every repository is a throwaway under my own
directory, minted with `mktemp -d`:
`<scratch>/row-doc-list-driver.tyYnVh`
(written `<W>` below). Nothing was torn down; every root is still there.

Environment of every invocation: `HOME=<root>/home` (an empty directory of the row's own),
`GIT_CONFIG_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL=/dev/null`, author and committer identity from the
four `GIT_*` variables — the faithful mask of `implementation/dev-workflow.md` → Gate.

One driver, `<W>/tools/drive.sh <root> <cwd> <label> <binary> <args…>`, ran every row. Around the
one invocation it takes, in this order:

1. `git status --porcelain --untracked-files=all --ignored` (the stricter form: untracked files
   one by one, ignored files too) → `<root>.runs/<label>.porc.before`
2. a **manifest of every entry under `<root>`** — the working tree, the whole `.git`, the whole
   `.jigc`, the row's `HOME`, every linked worktree — one line per file (mode, size, sha256,
   path), per directory (mode, path) and per symlink (target) → `<label>.man.before`
3. the invocation, stdout and stderr to their own files, exit status read directly
4. the same `git status` → `<label>.porc.after`
5. the same manifest → `<label>.man.after`

*Porcelain identical* below means `cmp` of 1 and 4; *bytes identical* means `diff` of 2 and 5
came back empty — which is stronger than the porcelain, since it also holds git's own files,
the gitignored workbench and the home directory. The manifest's entry count is given per group.

## The table the brief asks for

### Row 1 — the verb in a repository `jigc setup` never ran in

Root `<W>/r1-bare.ilqxin`. Construction: `git init -q .`; one commit of `README.md`; then an
untracked `notes.md` (one line), never added. No `jigc setup`.

| # | cwd | invocation | exit | stdout | stderr | porcelain before → after | bytes under root |
|---|---|---|---|---|---|---|---|
| 1.1 | repo root | `jigc doc list` | **1** | empty | `this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)` | `?? notes.md` → `?? notes.md` | identical (46 entries) |
| 1.2 | repo root | `jigc doc list --format json` | 1 | empty | `{ "error": "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)" }` (3 lines) | identical | identical |
| 1.3 | repo root | `jigc doc list adr` | 1 | empty | the 1.1 line | identical | identical |
| 1.4 | `sub/dir` | `jigc doc list` | 1 | empty | the 1.1 line | identical | identical (48) |
| 1.5 | repo root | `jigc doc list --task nope` | 1 | empty | `blocking · finalize.no-task — no task `nope`` / `at: task:nope` / `route: `jigc task list` lists the live tasks` / footer | identical | identical |
| 1.6 | repo root | `jigc doc list --task nope --format json` | 1 | empty | the findings envelope, `schema_version` 3, one `finalize.no-task` finding | identical | identical |
| 1.7 | a linked worktree of it (`git worktree add`), holding an untracked file | `jigc doc list` | 1 | empty | the 1.1 line | identical | identical (64) |
| 1.8 | a directory that is no git repository | `jigc doc list` | 1 | empty | `not inside a git repository (no `.git` found from <cwd>) — run jigc from inside the target git repository; if this project isn't one yet, `git init` here first` | (git: not a repository) | identical (4) |

`notes.md` read back after 1.1: `untracked bytes no git object holds` — present, unchanged. No
`.jigc/` was created by any of the eight. Previous release, rows 1.1, 1.2, 1.5, 1.8: same exit,
same stdout, same stderr, bytes identical.

The route 1.1 prints, run as printed (`jigc setup`, in the Row 2 construction): exit 0, an
install commit, and `jigc doc list` then answers at exit 0. The route 1.5 prints, run as
printed in the never-set-up repository (`jigc task list`): exit 0, `jigc task list — no active
tasks`, bytes identical — see lead L2.

### Row 2 — the verb in a repository setup ran in, which holds an untracked file

Root `<W>/r2-setup.ay8WZN`. Construction — the seeded finding's block, step for step, with the
candidate as BINARY and a home directory that is not the machine's:

    git init -q repo
    git -C repo commit -q --allow-empty -m base
    (in repo)  jigc setup                       exit 0, install commit 19e97f7
    (in repo)  printf 'one line of notes that no git object holds\n' > notes.md      never added
    (in repo)  jigc doc list
    (in repo)  cat notes.md

| # | invocation | exit | stdout | stderr | porcelain before → after | bytes under root |
|---|---|---|---|---|---|---|
| 2.1 | `jigc doc list` | **0** | `jigc doc list — no committed docs` (36 bytes) | empty | `?? notes.md` → `?? notes.md` | identical (94 entries) |
| 2.2 | `jigc doc list --format json` | 0 | `{ "docs": [] }` (17 bytes) | empty | identical | identical |
| 2.3 | `jigc doc list adr` | 0 | `jigc doc list — no committed `adr` docs` | empty | identical | identical |
| 2.4 | `jigc doc list no-such-doctype` | 1 | empty | `blocking · store.unknown-type — unknown doctype `no-such-doctype`` / `route: list the available doctypes with `jigc describe`` | identical | identical |
| 2.5 | `jigc doc list --task nope` | 1 | empty | `finalize.no-task`, as 1.5 | identical | identical |

After 2.1: `cat notes.md` exits 0 and prints `one line of notes that no git object holds`;
sha256 `2af6182885a710b2b4342810173ec3fa1e6765f34c6caf22d4581b517ded5a71` before and after;
`git log --oneline` still two commits (`19e97f7`, `eab68c5`); `git stash list` empty. Previous
release, 2.1 and 2.2: same exit, same stdout, same stderr, bytes identical.

**The seeded claim is refuted — Repro 1.**

## The rest of what was driven on the door

Every row below: candidate, exit status as given, **porcelain identical, bytes identical** unless
the row says otherwise.

### A. Untracked files squatting at managed-doc homes (`<W>/r3-homes.4BswBh`)

Set-up repository; then, none of them added: a Keep-a-Changelog `CHANGELOG.md`, a `VISION.md`,
`docs/roadmap.md`, `docs/decisions/0001-use-postgres.md`, `docs/specs/weird.md` (bytes that are
no UTF-8), an empty `docs/ideas/empty.md`, `notes.md`; plus an ignored directory with a file, an
ignored `local.secret`, an uncommitted edit to the tracked `CLAUDE.md` and a file staged in the
index and never committed. Porcelain, 11 lines, before and after each row:
` M CLAUDE.md` · `A  staged.txt` · six `??` doc-home files · `?? notes.md` ·
`!! ignored-dir/keep.txt` · `!! local.secret`.

| # | invocation | exit | stdout |
|---|---|---|---|
| A1 | `jigc doc list` | 0 | header `id  path  state` and six rows, every one `unregistered`: `adr:0001-use-postgres` · `changelog:changelog` · `idea:empty` · `roadmap:roadmap` · `spec:weird` · `vision:vision` |
| A2 | `jigc doc list --format json` | 0 | the same six, each `"state": "unregistered"`, `"item-count": 0`, `"fields": null`, `title` the file's H1 or `null` |
| A3 | `jigc doc list adr` | 0 | the one `adr` row |
| A4 | `jigc doc list changelog --format json` | 0 | the one `changelog` row |
| A5 | A1 and A2 again under `status.showUntrackedFiles=no` and `core.autocrlf=true` (repository config) | 0 | byte-identical to A1 and A2 |

112 manifest entries, identical across every row. Nothing was adopted, stamped, moved or staged.

### B. Ordinary git configuration in one repository (`<W>/r5-crlf-symlink.UCITNS`)

`CLAUDE.md` a tracked symlink to a tracked `AGENTS.md` inside the repository,
`core.autocrlf=true`, `status.showUntrackedFiles=no`, then `jigc setup` (exit 0), then an
untracked CRLF `VISION.md` and a CRLF `notes.md`.

| # | invocation | exit | stdout |
|---|---|---|---|
| B1 | `jigc doc list` | 0 | `vision:vision  VISION.md  unregistered` |
| B2 | `jigc doc list --format json` | 0 | the one row, `"title": "Vision"` |

96 entries, identical; `CLAUDE.md` still the symlink to `AGENTS.md`. Previous release: same.
(A first attempt with `CLAUDE.md` linked to a file *outside* the repository never reached the
door: `jigc setup` refused it, `setup.inject-reference`, exit 1 — `<W>/r4-crlf-symlink.Q4Ec21`;
`doc list` there is Row 1's refusal, bytes identical.)

### C. Committed docs, a live task that stages one, untracked files (`<W>/jigc-rig-refs-post-hoc-9hmHWR`)

The rig state `refs-post-hoc` on the candidate: five committed managed docs, and the live task
`ground-the-vision-in-research` holding a staged copy of the committed vision. Added by hand:
an untracked `notes.md` and an untracked `docs/research/draft-untracked.md`.

| # | cwd | invocation | exit | stdout | stderr |
|---|---|---|---|---|---|
| C1 | main checkout | `jigc doc list` | 0 | six rows: five `managed`, `research:draft-untracked … unregistered` (330 bytes) | `note: docs are also staged in open task ground-the-vision-in-research — this listing is the committed store; if that task is yours, list what it stages: `jigc doc list --task ground-the-vision-in-research`` |
| C2 | main checkout | `jigc doc list --format json` | 0 | the same six as json (1331 bytes) | the C1 note |
| C3 | main checkout | `jigc doc list --task ground-the-vision-in-research` (C1's route, as printed) | 0 | `commit:ground-the-vision-in-research` and `vision:vision`, both `managed` | empty |
| C4 | main checkout | the same with `--format json` | 0 | the two rows; the staged vision carries `"grounded-in": ["research:context-loss"]` | empty |
| C5 | main checkout | `jigc doc list vision` | 0 | the one row | the note, scoped: `` `jigc doc list vision --task ground-the-vision-in-research` `` |
| C6 | main checkout | that route, as printed | 0 | `vision:vision  VISION.md  managed` | empty |
| C7 | main checkout | `jigc doc list adr` | 0 | `jigc doc list — no committed `adr` docs` | empty (the task stages no adr) |
| C8 | `src/deep` | `jigc doc list` | 0 | byte-identical to C1 | the C1 note |
| C9 | main checkout, after three out-of-band edits left uncommitted: a line appended to `VISION.md`, `docs/roadmap.md` moved away, `docs/decisions-log.md` overwritten with a line of garbage | `jigc doc list` | 0 | `vision` still `managed`, `roadmap` no longer listed, `decisions-log:decisions-log … unregistered` | the C1 note |

148 to 274 entries (the root grew as worktrees were added, below); identical across every row.
In C9 the porcelain before and after is the same 18 lines (` M VISION.md`,
` M docs/decisions-log.md`, ` D docs/roadmap.md`, …) and `.jigc/state/file-state.json` has the
same sha256 before and after: the read absorbed, reconciled and restored nothing. Previous
release, C1, C2 and C9: same stdout, same stderr, bytes identical.

### D. Linked worktrees — the cell the round's change reaches (`served_from_home_note`)

Same rig. `git worktree add <root>/wt -b feature`; on that branch a commit that edits
`VISION.md` and adds `docs/research/branch-only.md`; then in the worktree an untracked
`wt-notes.md` and an uncommitted edit to `CHANGELOG.md`. Worktree porcelain, before and after
every row: ` M CHANGELOG.md` · `?? wt-notes.md`.

| # | cwd | invocation | exit | stdout | stderr |
|---|---|---|---|---|---|
| D1 | `wt` | `jigc doc list` | 0 | **byte-identical to the main checkout's** — the main checkout's untracked `draft-untracked.md` is listed, the branch's own `branch-only.md` is not | the C1 note, then one line: `note: served from the main checkout at `<root>/repo` — jigc's doc store has one home, not the linked worktree at `<root>/wt` on branch `feature` you are standing in` |
| D2 | `wt` | `jigc doc list --format json` | 0 | byte-identical to the main checkout's json | the same two lines |
| D3 | `wt/sub` | `jigc doc list` | 0 | byte-identical | the same two lines |
| D4 | `wt` | `jigc doc list research` | 0 | the two research rows | the served-from line alone |
| D5 | `wt` | `jigc doc list --task ground-the-vision-in-research` | 0 | the staged listing, as C3 | **empty** — no note on a `--task` read |
| D6 | `wt` | previous release, `jigc doc list` | 0 | byte-identical to D1 | the C1 note alone — no served-from line |
| D7 | a detached worktree (`git worktree add --detach`) | `jigc doc list` | 0 | byte-identical | the served-from line without a branch: `… not the linked worktree at `<root>/wt-detached` you are standing in` |
| D8 | a worktree nested below the main checkout (`git worktree add <root>/repo/.trees/nested -b nested`) | `jigc doc list` | 0 | byte-identical | `… at `<root>/repo` — … not the linked worktree at `.trees/nested` on branch `nested` you are standing in` — lead L1 |
| D9 | the main checkout reached through a symlink to it, and through the system temp directory's own symlinked spelling | `jigc doc list` | 0 | byte-identical | the C1 note alone — **no** served-from line |
| D10 | the worktree reached the same two ways | `jigc doc list` | 0 | byte-identical | the two lines, both paths in their resolved spelling |
| D11 | a `git clone` of the rig repository, holding an untracked file | `jigc doc list` | 0 | the clone's own five `managed` rows | empty — a clone is its own main checkout |

203 to 422 entries; identical across every row: the worktree's untracked file and uncommitted
edit, the main checkout's untracked files, `.git/worktrees/*` and the task's staged copy all
unchanged. The design's four statements about the note (`design/doc-read-surface.md`, *The
served-from note*) each held as driven: on stderr; stdout byte-identical to the main checkout's;
nothing from the main checkout; nothing on a `--task` read. Its fifth — nothing from one of
jigc's own fan-out worktrees — was not driven.

### E. One doc set, two creation orders, then concurrent readers

`<W>/r7-order-fwd.9qNFmX` and `<W>/r7-order-rev.on2eQS`, built by `<W>/tools/order.sh`: a set-up
repository and three `research` docs, each minted, authored and finalized through the binary
(`start --workflow do-research` → `doc create research` → three `doc set-slot` → the commit doc
→ `task finalize`), with one untracked file planted at the research home and one at the root
after each. Order *Alpha, Beta, Gamma* in the first, *Gamma, Beta, Alpha* in the second.

| # | invocation | exit (both) | stdout, first root vs second |
|---|---|---|---|
| E1 | `jigc doc list` | 0 | **byte-identical** (465 bytes, sha256 `cd8e4bd7112c3675…` in both) |
| E2 | `jigc doc list --format json` | 0 | **byte-identical** (sha256 `7e94e568543e8673…` in both) |
| E3 | `jigc doc list research --format json` | 0 | byte-identical |
| E4 | eight `jigc doc list --format json` started together in the first root | 0 × 8 | eight stdouts, each byte-identical to E2 |

132 entries, identical across E1–E3 in both roots and across the eight concurrent readers of E4.
Previous release on both roots: json byte-identical to the candidate's.

### F. The opt-in invocation log — the one write the door makes

`fn main` changed in the round (it now passes a `LogWrite` to `invocation_log::log_invocation`),
and every leaf verb runs through it. The log is off by default (`jigc config get invocation-log`
→ `false  (pack-default)`), which is why every row above shows no write. Switched on in
`<W>/r7-order-fwd.9qNFmX` with `jigc config set invocation-log true`, the change committed:

| # | state | invocation | exit | what changed under the root |
|---|---|---|---|---|
| F1 | a log holding one record (the pre-commit hook's own `validate`) | `jigc doc list` | 0 | **only** `.jigc/logs/invocations.jsonl`: 313 → 490 bytes, one record appended — `{"timestamp":"…","argv":["doc","list"],"exit_code":0,"duration_ms":64,"finding_codes":[],"output_bytes":465,"binary_version":"1.0.0-rc.24","error_code":null}`. Porcelain identical (with `--ignored` too). stdout byte-identical to E1 |
| F2 | 2 records | eight started together, `--format json` | 0 × 8 | 2 → 10 records; the earlier file is a byte prefix of the later; every line parses as JSON; nothing else under the root changed; eight stdouts byte-identical to E2 |
| F3 | 10 records | eight, one after another | 0 × 8 | 10 → 18 records, the earlier file a byte prefix; eight stdouts byte-identical to E2 |
| F4 | — | `jigc doc list no-such-doctype` | 1 | one record, `"exit_code":1`, `"finding_codes":["store.unknown-type"]` |
| F5 | a linked worktree of this repository, holding an untracked file | `jigc doc list` | 0 | the record lands in the **main checkout's** log (3841 → 4019 bytes); the worktree gains nothing — its `.jigc` holds no `logs` |
| F6 | `.jigc/logs` moved out of the repository | `jigc doc list` | 0 | `.jigc/logs/` and a one-record log are created (177 bytes); porcelain with `--ignored` gains `!! .jigc/logs/invocations.jsonl`, plain porcelain is unchanged |
| F7 | the log file mode 444 | `jigc doc list` | 0 | bytes identical — no record, no message, the read answers |
| F8 | — | previous release, `jigc doc list` | 0 | one record appended, same shape |

This is the write `design/measurement.md` declares; it is opt-in, lands in a gitignored file,
is append-only as driven, and is the only path the door wrote on in this report.

### G. Misuse shapes (rig root, main checkout; previous release: same stdout and stderr in every row)

| # | invocation | exit | answer |
|---|---|---|---|
| G1 | `jigc doc list --format bogus` | 2 | clap: `invalid value 'bogus' for '--format <FORMAT>'` |
| G2 | `jigc doc list adr spec` | 2 | clap: `unexpected argument 'spec' found` |
| G3 | `jigc doc list ''` | 1 | `store.unknown-type — unknown doctype ```` |
| G4 | `jigc doc list adr:foo` | 1 | `store.unknown-type` |
| G5 | `jigc doc list ../../etc` | 1 | `store.unknown-type` |
| G6 | `jigc doc list --task ''` | 1 | `work-unit.malformed-id — "" is not a valid work-unit id` |
| G7 | `jigc doc list --task ../config` · `--task ../../docs` · `--task .` | 1 | `work-unit.malformed-id` each — no path built from the id |
| G8 | `jigc doc list commit` | 0 | `no committed `commit` docs`, and the staged note scoped to `commit` |

422 entries, bytes identical in every row.

### H. Entries at a managed home that cannot be read as a file (`<W>/r6-odd.w0nVxB`)

A set-up repository holding an untracked `notes.md`; then one shape at a time, each alone:

| # | the one untracked entry planted | exit | stdout | stderr |
|---|---|---|---|---|
| H0 | none | 0 | `jigc doc list — no committed docs` | empty |
| H1 | `docs/decisions/dangling.md`, a symlink whose target does not exist | **1** | empty | `reading the committed doc at "<root>/repo/docs/decisions/dangling.md": No such file or directory (os error 2)` |
| H2 | `CHANGELOG.md`, a directory | **1** | empty | `reading the committed doc at "<root>/repo/CHANGELOG.md": Is a directory (os error 21)` |
| H3 | `docs/research/a-dir.md`, a directory | **1** | empty | `… "<root>/repo/docs/research/a-dir.md": Is a directory (os error 21)` |
| H4 | `docs/ideas/locked.md`, a file of mode 000 | **1** | empty | `… "<root>/repo/docs/ideas/locked.md": Permission denied (os error 13)` |
| H5 | `docs/specs/out-link.md`, a symlink to a file outside the repository | 0 | `spec:out-link  docs/specs/out-link.md  unregistered` | empty — lead L4 |
| H6 | `docs/research/has space.md` | 0 | `research:has space  docs/research/has space.md  unregistered` | empty — lead L3 |
| H7 | `docs/research/UPPER.md` | 0 | `research:UPPER  docs/research/UPPER.md  unregistered` | empty — lead L3 |

107 entries, bytes identical in every row. Previous release: same stdout and stderr in every
row. H1–H4 are finding DL-1.

### I. A worktree of a bare repository (`<W>/r8-barestore.07r8Lk`, `<W>/r9-barestore-cand.Sx0fSz`, `<W>/r9-barestore-prev.0Wph36`)

`git init --bare store.git`, `git -C store.git worktree add ../wt -b main`, one commit in `wt`.

| # | step, typed in `wt` | exit | what happened |
|---|---|---|---|
| I1 | `jigc doc list` | 1 | Row 1's refusal, routing at `jigc setup`; bytes identical |
| I2 | `jigc setup` — I1's route, as printed | **1** | `blocking · setup.install-hook — cannot install the `pre-commit` hook into the repo's hooks dir: git could not resolve the hooks dir for `<root>/proj`: fatal: not a git repository …` — **after** it had written `.jigc/` (five files), `.claude/settings.json`, `.claude/skills/jigc/SKILL.md` and `CLAUDE.md` into the folder that holds `store.git`, which is no git repository; a `CLAUDE.md` of the user's already in that folder had a `## Project interface` section and `@.jigc/AGENT.md` appended to it. Nothing in `wt`, no commit |
| I3 | `jigc setup` again — I2's route, as printed (the hooks directory is `store.git/hooks`, mode `drwxr-xr-x`) | 1 | the same refusal |
| I4 | `jigc doc list` | **0** | `jigc doc list — no committed docs`; stderr empty; bytes identical |
| I5 | `jigc doc list`, after an untracked `docs/research/outside.md` was put in the folder that holds `store.git` | **0** | `research:outside  docs/research/outside.md  unregistered`; stderr empty — no served-from line; `wt/docs/research/outside.md` does not exist; bytes identical |

Previous release: I1, I2 (same refusal, the same thirteen manifest lines) and I4 the same. I4
and I5 are finding DL-2; I2 and I3 are finding DL-3, on a door this item does not list. **Both
are already on record**: `implementation/decisions-pending.md` → *(D) A worktree of a bare
repository …*, corrected 2026-10-05 — "the home jigc resolves is a directory that is no work
tree, so store reads … are taken at the wrong place", "`jigc setup` exits 1 at its hook step in
the sibling-bare and `--separate-git-dir` layouts, after writing", trigger M57, with the open
question *whether these three layouts are supported*.

## Findings

### DL-1 — one unreadable entry at a managed home refuses the whole listing, with a raw OS error

- **door:** `jigc doc list`
- **clause:** none, as I read it. Nothing is written or lost (bytes identical); it is no
  regression; and the refusal prints no route, so no route fails as printed. It is a
  refusal-quality defect on the door `design/doc-read-surface.md` calls "a report … never a
  block" for an instance that does not parse.
- **what:** an untracked dangling symlink, a directory named `<x>.md`, or a file the user
  cannot read, at any home `doc list` enumerates, turns the listing of the *whole* store into
  exit 1 with one line: no finding code, no route, the absolute path of the machine, and the
  words "the committed doc" for an entry git has never held. `--format json` answers
  `{ "error": "reading the committed doc at …" }`.
- **class:** an enumerated instance whose bytes `fn run_list` reads with a bare
  `std::fs::read(…)?`.
- **count_derivation:** an `awk` over the body of `fn run_list` in `crates/cli/src/doc.rs` at
  eeffe347 for `std::fs::read` — **2 hits**: line 4866 (a resolved row, context "reading the
  committed doc at") and line 4953 (an orphan row, context "reading the orphaned doc at").
  `grep -rn "reading the committed doc at" crates/cli/src crates/engine/src` — 1 hit, the
  first. **Driven: the first site, in 4 cells** (3 shapes; a located home and a placement
  home). **Not driven: the orphan-row site**, and the staged arm (`fn run_list_staged`), whose
  reads I did not enumerate — so for the staged arm this is `instance, unbounded`.
- **severity:** low. A dangling link in a docs directory is an ordinary accident, not a planted
  state; the cost is that the agent's one sanctioned index read answers nothing until it is
  found, and the message does not say what to do.
- **regression:** no — the previous release prints the same bytes at the same exit, all four cells.
- **repro:** Repro 2.

### DL-2 — in a worktree of a bare repository the listing is served from a folder that is no checkout, and says nothing

- **door:** `jigc doc list`
- **clause:** none, as I read it — the read writes nothing. **Already on record** at
  `implementation/decisions-pending.md` → (D), trigger M57; reported here as found again.
- **what:** rows I4 and I5. At exit 0 the listing names `docs/research/outside.md`, a path that
  resolves to nothing from the checkout the reader stands in, and the round's served-from line
  is silent, by its own design (`crates/cli/src/repo.rs`, `home_is_a_checkout`: "a probe that
  cannot answer reads as `false`").
- **class:** the layouts whose `.git` is a file with no main checkout behind it.
- **count_derivation:** read off the doc comment of `home_is_a_checkout` in
  `crates/cli/src/repo.rs`, which enumerates **3**: a worktree of a bare repository, a
  `--separate-git-dir` checkout, a submodule. **Driven: 1 of the 3.** An instance of a recorded
  class; the other two layouts were not driven here.
- **severity:** low for this door (a read); the layout question is the human's and is open.
- **regression:** no — the previous release answers I4 the same.
- **repro:** Repro 3.

### DL-3 — `jigc setup` in that layout writes outside any repository, then exits 1, and its route loops

- **door:** `jigc setup` — **unlisted** (not a door of this item or of the round's test set; reached
  by running Row 1's printed route in layout I).
- **clause:** `working-product`, as I read its second half — "every refusal's route works as
  printed": I1 routes at `jigc setup`, which exits 1; that refusal routes at "ensure the repo's
  git hooks directory is writable, then re-run `jigc setup`", and the hooks directory is
  writable and the re-run exits 1 again. It also touches the first clause's sentence "where
  jigc cannot tell (git fails …) it refuses before writing": git failed and the writes had
  already happened — but at exit 1, and the user's `CLAUDE.md` was appended to, not destroyed,
  so I do not read it as a break of `no-lost-files` as scoped. Whether the layout is *supported*
  decides both, and that is (D)'s open question. **Already on record** there.
- **class / count_derivation:** as DL-2 — 3 layouts by the same doc comment; (D) itself says
  setup exits 1 after writing in **2** of them (sibling-bare, `--separate-git-dir`). Driven: 1.
  `instance` of a recorded class.
- **severity:** medium if the layout is supported, none if a refusal is to say it is not.
- **regression:** no — the previous release leaves the same thirteen manifest lines at the same exit.
- **repro:** Repro 4.

## Leads — noticed, not pursued

- **L1 (door `jigc doc list`, clause none).** Row D8: from a worktree nested *below* the main
  checkout the served-from line spells the main checkout absolute and the worktree
  `.trees/nested` — relative to the main checkout, not to the reader, who is standing in it.
  The doc comment on `InstallSite.home` (`crates/cli/src/render.rs`) reasons from "both
  reachable cells" (a worktree outside, a fan-out worktree below); a user's own worktree below
  the main checkout is a third. Whether law 1's repo-relative rule intends that spelling here I
  did not establish. One driven cell; instance, unbounded.
- **L2 (door `jigc doc list`, clause none).** Row 1.5: in a never-set-up repository the `--task`
  arm answers `finalize.no-task` and routes at `jigc task list`, which exits 0 with "no active
  tasks" — neither says the project is not set up. `fn run_list` returns into
  `run_list_staged` before `require_project_layer` runs (the first lines of the function).
  Same on the previous release. Instance, unbounded — I did not enumerate the other `--task`
  read arms.
- **L3 (door `jigc doc list`, clause none).** Rows H6 and H7: an untracked file whose name is
  no slug is listed with an id outside the slug grammar (`research:has space`,
  `research:UPPER`), and in the plain listing — columns separated by two spaces — the space
  makes the row ambiguous. Same on the previous release. Not pursued: whether those ids are
  accepted by the adoption verbs the `unregistered` state points at.
- **L4 (door `jigc doc list`, clause none).** Row H5: a symlink at a managed home that leads
  outside the repository is read through — `--format json` projects the outside file's H1 as
  the row's `title` (`"Heading outside the repository"`). `jigc setup` refuses to follow such a
  link for a write; the read follows it. A deliberately planted state, so most likely inside a
  declared bound; no bound is declared for this run, so it is returned rather than dropped.

## Repro blocks

### Repro 1 — the seeded claim, refuted

```yaml
claim: "in a repository `jigc setup` has run in, `jigc doc list` exits 0 and an untracked file in the repository's root is gone afterwards (ledger key canary-seeded-claim)"
verdict: REFUTED
binary: candidate, content sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc (also refuted on the previous release, accf3996…)
setup:
  - env: HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null, GIT_AUTHOR_NAME/EMAIL and GIT_COMMITTER_NAME/EMAIL set
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - cwd: repo
  - ["jigc", "setup"]                       # exit 0
  - write: notes.md = "one line of notes that no git object holds\n"   # never added
repro:
  - ["git", "status", "--porcelain", "--untracked-files=all", "--ignored"]   # "?? notes.md"
  - ["jigc", "doc", "list"]
  - ["git", "status", "--porcelain", "--untracked-files=all", "--ignored"]
  - ["cat", "notes.md"]
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs\n"
  stderr: ""
  porcelain_after: "?? notes.md\n"          # identical to before
  notes_md: present, sha256 2af6182885a710b2b4342810173ec3fa1e6765f34c6caf22d4581b517ded5a71 before and after
  every_file_under_the_root: identical (94 entries: working tree, .git, .jigc, HOME)
observed: exactly the expectation — <W>/r2-setup.ay8WZN, label setup-list
pinned-by: "UNPINNED: a verdict of this round; conversion to a standing test is the fix stage's"
```

### Repro 2 — DL-1, an unreadable entry refuses the whole listing

```yaml
claim: "one untracked entry at a managed home that cannot be read as a file makes `jigc doc list` exit 1 with a raw OS error and no route"
verdict: CONFIRMED
binary: candidate dded1fac… ; the previous release accf3996… prints the same bytes
setup:
  - env: as Repro 1
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - cwd: repo
  - ["jigc", "setup"]
  - ["mkdir", "-p", "docs/decisions"]
  - ["ln", "-s", "/nonexistent/nowhere.md", "docs/decisions/dangling.md"]
repro:
  - ["jigc", "doc", "list"]
expect:
  exit: 1
  stdout: ""
  stderr: 'reading the committed doc at "<absolute path of repo>/docs/decisions/dangling.md": No such file or directory (os error 2)'
  bytes_under_the_root: identical
variants:        # each alone, in place of the `ln`
  - "mkdir CHANGELOG.md"                                  -> exit 1, "… CHANGELOG.md\": Is a directory (os error 21)"
  - "mkdir -p docs/research/a-dir.md"                     -> exit 1, "Is a directory (os error 21)"
  - "printf '# x\n' > docs/ideas/locked.md; chmod 000 …"  -> exit 1, "Permission denied (os error 13)"
control: with nothing planted the same command exits 0, "jigc doc list — no committed docs"
observed: <W>/r6-odd.w0nVxB, labels s0-none, s1-dangling, s2-dir-placement, s3-dir-located, s4-unreadable (and the same labels with -prev)
pinned-by: "UNPINNED: found this round"
```

### Repro 3 — DL-2, the listing in a worktree of a bare repository

```yaml
claim: "in a worktree of a bare repository, after `jigc setup` was typed there, `jigc doc list` exits 0 serving the folder that holds the bare repository, with no served-from line"
verdict: CONFIRMED
binary: candidate dded1fac… ; the previous release answers the same
setup:
  - env: as Repro 1
  - ["mkdir", "proj"]
  - cwd: proj
  - ["git", "init", "-q", "--bare", "store.git"]
  - ["git", "-C", "store.git", "worktree", "add", "-q", "../wt", "-b", "main"]
  - cwd: proj/wt
  - write: README.md ; ["git", "add", "README.md"] ; ["git", "commit", "-q", "-m", "base"]
  - ["jigc", "setup"]                       # exit 1 — Repro 4
  - write: ../docs/research/outside.md = "# Outside any repository\n"
repro:
  - ["jigc", "doc", "list"]                 # cwd proj/wt
expect:
  exit: 0
  stdout: "id  path  state\nresearch:outside  docs/research/outside.md  unregistered\n"
  stderr: ""                                # no "note: served from …"
  note: proj/wt/docs/research/outside.md does not exist
  bytes_under_the_root: identical
observed: <W>/r9-barestore-cand.Sx0fSz, labels bs-after, bs-after2
pinned-by: "UNPINNED: the class is recorded at implementation/decisions-pending.md → (D)"
```

### Repro 4 — DL-3, `jigc setup` writes outside any repository and then exits 1

```yaml
claim: "`jigc setup` typed in a worktree of a bare repository writes its install into the folder that holds the bare repository, appends to a CLAUDE.md there, exits 1, and its route re-run as printed exits 1 again"
verdict: CONFIRMED
binary: candidate dded1fac… ; the previous release leaves the same thirteen manifest lines at exit 1
setup:
  - env: as Repro 1
  - ["mkdir", "proj"]
  - cwd: proj
  - ["git", "init", "-q", "--bare", "store.git"]
  - ["git", "-C", "store.git", "worktree", "add", "-q", "../wt", "-b", "main"]
  - write: CLAUDE.md = "# my folder-level agent rules\n\nkept beside the bare store, in no git repository\n"
  - cwd: proj/wt
  - write: README.md ; ["git", "add", "README.md"] ; ["git", "commit", "-q", "-m", "base"]
repro:
  - ["jigc", "setup"]
  - ["jigc", "setup"]                       # the route, as printed
expect:
  exit: 1                                   # both times
  stdout: ""
  stderr: "blocking · setup.install-hook — cannot install the `pre-commit` hook into the repo's hooks dir: git could not resolve the hooks dir for `<proj>`: fatal: not a git repository (or any of the parent directories): .git\n  route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`\n…"
  created_in_proj: [".jigc/.gitignore", ".jigc/AGENT.md", ".jigc/config/.gitkeep", ".jigc/config/packs.yaml", ".jigc/version", ".claude/settings.json", ".claude/skills/jigc/SKILL.md"]
  proj_CLAUDE_md: the three original lines, then "\n## Project interface\n\n@.jigc/AGENT.md\n"
  proj_wt: "git status --porcelain" empty, one commit
observed: <W>/r9-barestore-cand.Sx0fSz and <W>/r9-barestore-prev.0Wph36 (built by <W>/tools/barestore.sh), files *.man.before, *.man.after, *.setup.stderr, *.setup2.stderr
pinned-by: "UNPINNED: recorded at implementation/decisions-pending.md → (D), trigger M57"
```

## Not driven

- **The served-from line from one of jigc's own fan-out worktrees** (the design's fifth
  statement: nothing is printed there). It needs a milestone fan-out; I did not build one.
- **A `--separate-git-dir` checkout and a submodule** — the other two layouts of DL-2's class.
- **The orphan-row read** of `fn run_list` (a stamped doc whose doctype resolves to nothing),
  the second site of DL-1's class.
- **The door through a real assistant session.** I ran headless; nothing here is a spawn.
- **The debug-only route fences.** The candidate is a release binary; the gate runs those.
- **`jigc doc list --format human`** was not driven as a row of its own.
- **An older git, a moved repository** — two of the nine layouts of the regression set's part 2.

## Where the evidence is

Per root `<W>/<root>`: `<W>/<root>.runs/<label>.{stdout,stderr,porc.before,porc.after,man.before,man.after}`.
The drivers: `<W>/tools/{env.sh,snap.sh,drive.sh,order.sh,barestore.sh}`. I committed nothing,
and wrote nothing into the working tree but this report: `git status --porcelain` in the clone
read `?? completions/artifacts/canary-one/r1/` at the start and at the end.

<!-- end of report -->
