# row-doc-list — the reconciliation (run canary-one, round 1, stage test, attempt 1)

Reporter `row-doc-list-reconciler`. Item `row-doc-list` (review-row, clause `no-lost-files`). The one
door of the round's test set this item covers, in the door list's own words: `jigc doc list`.

What was handed over: the source pass (`row-doc-list-source.a1.md`) and the driver's table
(`row-doc-list-driver.a1.md`), and no other report. What this report is: every defect either of
them reports and every claim one makes that the other does not, **driven again** on the binary of
the `BINARY:` line, in throwaway repositories of this reporter's own. Nothing below is taken from
either report's evidence files; one read-only count of the driver's label files is the single
look into another reporter's directory, and it is marked where it is used.

## Verdict in one paragraph

**The two reports agree with each other and with the binary on everything that bears on the row's
question.** On the candidate, `jigc doc list` wrote, removed and committed nothing in **121 of 121**
invocations driven here with the `invocation-log` knob off — exit 0, 1, 2 and one killed by an
alarm alike — the whole root (working tree, `.git/`, `.jigc/`, linked worktrees, the home
directory) compared entry by entry, **mtime included**, around the verb and nothing else, and
`git status --porcelain --untracked-files=all --ignored` identical before and after. **The seeded
claim `canary-seeded-claim` is REFUTED a third time**, on the candidate and on the previous
release (Repro RC-0). The one write reachable from the door is the one both reports name: the
opt-in invocation log. **No claim of either report failed to reproduce.** Three statements were
narrower or wider than what the binary does, and each is corrected below with its drive:
the source's "the door's two other hard reads are guarded" (one of them is not, RC-2), the
source's "at a located home" (a placement home too), and the driver's invocation count (its
operands, not its total). **Two leads became driven findings that neither report had driven:**
the source's L2 — one tracked path whose name is not UTF-8 empties the orphan walk — is
CONFIRMED, and reaches further than the lead said: over the same state `jigc validate` goes from
exit 1 with a blocking finding to exit 0 and *the committed store validates clean* (RC-8, RC-9);
and the staged arm, which the driver left unbounded, dies on two shapes of its own (RC-2).
**None of the thirteen findings is a regression against the previous release**, each was driven on
both binaries, and none breaks `no-lost-files` as this reporter reads its scope.

**No spawn or launch is part of this unit.** Nothing here is a sub-agent spawn: the fan-out cell
(M1) reads from a worktree `jigc milestone provision` cut, in one process. **The genuine
concurrent-spawn artifact is the orchestrator's main-session half and was NOT run** — nothing in
this report is evidence about it.

## The binary, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- The candidate's directory went first on `PATH`; `command -v jigc` printed
  `<scratch>/bin/c1.a1/jigc`. Both binaries print `jigc 1.0.0-rc.24`; nothing is keyed by it.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<W> dev/jigc-rig --binary <binary> refs-post-hoc`,
  stdout captured alone. The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/`
  at the start and at the end, `HEAD` eeffe347; nothing was committed.

## How every row was measured

Host: macOS 26.6.2, git 2.54.0 (Apple Git-157). One directory of this reporter's own, minted with
`mktemp -d` under the scratch root: `<scratch>/row-doc-list-reconciler.nbC73u` (written `<W>`).
Every root under it is from `mktemp -d` and still there; nothing was torn down.

Environment of every invocation: `HOME=<root>/home` (the row's own, empty before `setup`),
`GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four
`GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables.

One driver, `<W>/tools/drive.sh <root> <cwd> <label> <binary> <args…>`, in this order:

1. `git status --porcelain --untracked-files=all --ignored` → `<root>.runs/<label>.porc.before`
2. a snapshot of every entry under `<root>` — kind, mode, size, sha256 (link target for a link),
   **mtime in nanoseconds**, path → `<label>.man.before`
3. the invocation, stdin from `/dev/null`, stdout and stderr to their own files, exit status read directly
4. the snapshot again → `<label>.man.after`
5. the porcelain again → `<label>.porc.after`

The snapshots bracket the verb and nothing else — the order the source corrected its own
instrument to, taken from the start here, and stricter than the driver's manifest (which carried
no mtime and took its second porcelain before its second manifest). *Tree IDENTICAL* means `cmp`
of 2 and 4. 274 before/after pairs were taken in all; the tally by binary and verb is under
*The tally*.

## The seeded claim

### Repro RC-0 — the seeded claim, refuted on both binaries

```yaml
claim: "in a repository `jigc setup` has run in, `jigc doc list` exits 0 and an untracked file in the repository's root is gone afterwards (ledger key canary-seeded-claim)"
verdict: REFUTED
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
setup:            # the opening record's block, step for step; HOME a fresh empty directory
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["jigc", "setup"]                       # in repo; exit 0, install commit
  - "write notes.md = `one line\n` and do not add it"
repro:
  - ["jigc", "doc", "list"]                 # in repo
  - ["cat", "notes.md"]
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs\n"   # 36 bytes
  stderr: ""
  notes_md: "present; `one line`; sha256 3887c2cd3bec16420dc71507a74cf7f0a5effdd361f6d9cbe27b77831de8f65f before and after"
  porcelain_before_and_after: "?? notes.md"
  tree: "IDENTICAL, mtime included — 91 entries (repo with .git/ and .jigc/, HOME)"
  git_log: "two commits (the install commit, base); `git stash list` empty"
observed: "exactly the expectation — <W>/r2-setup-cand.rnxjbo, label 2.1; again in a second root, <W>/r2-setup-cand2.*"
also-on-previous-release: "the same: exit 0, the file stands with the same sha256, tree IDENTICAL, 89 entries (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d) — <W>/r2-setup-prev.WgsvNG"
pinned-by: "UNPINNED: three reporters of this round drove it by hand; the nearest standing test, read_verb_acts_nothing::no_read_verb_acts_over_the_revealing_state, was read by the source pass as holding no untracked file at the repository's root — this reporter ran no test and did not re-read its fixture"
```

## The reconciliation, claim by claim

**Legend.** *both* = stated by both reports; *source* / *driver* = stated by one. *Re-driven* gives
this reporter's root and label. Every row was driven on the candidate **and** on the previous
release unless it says otherwise; "prev: same" means stdout and stderr byte-identical after the
root's path is normalised, and the same exit status.

### 1. The brief's table — never set up, and set up with an untracked file

| claim | by | re-driven | result |
|---|---|---|---|
| never set up, `doc list`: exit 1, stdout empty, ``this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)``; `?? notes.md` both; tree identical | both (source A; driver 1.1) | `<W>/r1-bare-cand.658WdO` 1.1 | **holds** — 47 entries IDENTICAL; `notes.md` reads back unchanged; no `.jigc/` created. prev: same |
| the same with `--format json` (the `{ "error": … }` envelope on stderr), `adr`, from `sub/dir`, from a linked worktree of it | driver 1.2–1.4, 1.7 | 1.2, 1.3, 1.4, 1.7 | **holds**, exit 1 each, IDENTICAL. prev: same |
| never set up, `doc list --task nope`: exit 1, `finalize.no-task`, route `jigc task list`; the route run as printed exits 0, `jigc task list — no active tasks` | driver 1.5, 1.6, lead L2 | 1.5, 1.6, 1.5route | **holds** — and see RC-11 |
| a directory that is no repository: exit 1, ``not inside a git repository (no `.git` found from <cwd>) — …`` | both (source N; driver 1.8) | 1.8 | **holds**. prev: same |
| set up + untracked `notes.md`, `doc list`: exit 0, `jigc doc list — no committed docs`, stderr empty, `?? notes.md` both, tree identical | both (source B; driver 2.1) | `<W>/r2-setup-cand.rnxjbo` 2.1 | **holds** — Repro RC-0 |
| `--format json` → `{ "docs": [] }` (17 bytes); `adr` → ``no committed `adr` docs``; an unknown doctype → exit 1 `store.unknown-type`, route `jigc describe`; `--task nope` → exit 1 `finalize.no-task` | both (source B3–B6; driver 2.2–2.5) | 2.2–2.5 | **holds**, IDENTICAL each. prev: same |

The entry counts differ between the three reporters (87 / 94 / 91 for the set-up root) because
each instrument puts different files of its own inside the root; they are not a disagreement
about the binary.

### 2. Committed docs, a live task, linked worktrees, misuse (rig `refs-post-hoc`)

Root `<W>/jigc-rig-refs-post-hoc-5jGyCP` (candidate) and `…-PFxdN3` (previous release). Added by
hand: untracked `notes.md`, `scratchdir/untracked.md`, `docs/research/draft-untracked.md`.

| claim | by | re-driven | result |
|---|---|---|---|
| main checkout: header + five `managed` rows (+ the untracked draft as `unregistered`), the staged-listing note on stderr | both (source C1; driver C1, C2) | C1, C2 | **holds**, 150 entries IDENTICAL |
| `--task <id>` lists the two staged rows, stderr empty; with `--format json` too; the doctype filter narrows the note and its route; that route as printed lists the one row | both (source C2–C4; driver C3–C7) | C3, C4, C5, C5j, C6, C7 | **holds** |
| from a sub-directory: byte-identical to C1 | driver C8 | C8 | **holds** (`cmp`) |
| `.jigc/index/` holds only `edges.json.lock` before and after — no derived cache appears | source | `ls -a` after C8 | **holds** |
| three out-of-band edits left uncommitted (append to `VISION.md`, `docs/roadmap.md` moved away, `docs/decisions-log.md` overwritten): exit 0, `vision` still `managed`, `roadmap` gone from the listing, `decisions-log … unregistered`; `.jigc/state/file-state.json` same hash before and after | driver C9 | C9 | **holds** — hash prefix `7051af5aa2c70338` both; 416 entries IDENTICAL |
| linked worktree: stdout byte-identical to the main checkout's (the main checkout's untracked draft listed, the branch's own `branch-only.md` not); stderr carries the staged note, then ``note: served from the main checkout at `<rig>/repo` — jigc's doc store has one home, not the linked worktree at `<rig>/wt` on branch `feature` you are standing in`` | both (source D1, D3; driver D1–D3) | D1, D2, D3 | **holds** (`cmp` against C1 and C2) |
| the doctype filter there: the served-from line alone | driver D4 | D4 | **holds** |
| `--task` there: the staged listing, stderr **empty** | both (source D2; driver D5) | D5 | **holds** (`cmp` against C3) |
| previous release there: same stdout, the staged note alone — no served-from line | driver D6 | previous rig, D1 | **holds** |
| a detached worktree: the line without a branch | driver D7 | D7 | **holds** |
| a worktree nested below the main checkout: the worktree spelled `.trees/nested` | driver D8, lead L1 | D8 | **holds** — RC-10 |
| the main checkout reached through a symlink: no served-from line; the worktree reached through one: both lines, resolved spelling | driver D9, D10 | D9, D10 | **holds** |
| a `git clone` of the rig repository: its own five rows, stderr empty | driver D11 | D11 | **holds** |
| misuse: `--format bogus` and a second positional → clap, exit 2; `''`, `adr:foo`, `../../etc` → `store.unknown-type`, exit 1; `--task ''`, `../config`, `../../docs`, `.` → `work-unit.malformed-id`, exit 1; `commit` → exit 0, the scoped note | driver G1–G8 | G1–G8 (G7 as three rows) | **holds**, every one IDENTICAL |
| `--format human` (the driver's *not driven*) | — | G9 | exit 0, byte-identical to C1 |

**Candidate against previous release, all 33 rows of this group:** every stdout byte-identical;
stderr differs in exactly seven rows — D1, D2, D3, D4, D7, D8, D10, the non-`--task` reads from a
linked worktree — and in each by the served-from line and nothing else. That is the round's change
in `fn run_list`, seen from outside, and its whole footprint on this door.

**The design's five statements about the note** (`design/doc-read-surface.md`, *The served-from
note*): on stderr; stdout byte-identical to the main checkout's; nothing from the main checkout;
nothing on a `--task` read — all four held as the driver said. **The fifth, which neither report
drove — nothing from one of jigc's own fan-out worktrees — holds too (M1):** `jigc milestone create
"Cache rework"`, `milestone add-task cache-rework "Area one"`, `milestone provision cache-rework`
(exit 0), then `jigc doc list` from `<repo>/.jigc/worktrees/area-one`: exit 0, stdout byte-identical
to the main checkout's, **stderr empty**, 155 entries IDENTICAL; `--task area-one` there: `jigc doc
list — no docs staged in task area-one`. Root `<W>/rM-fan-cand.QtQSn5`. prev: same.

### 3. Untracked files at managed homes; ordinary git configuration

| claim | by | re-driven | result |
|---|---|---|---|
| six untracked files at six managed homes (a Keep-a-Changelog `CHANGELOG.md`, `VISION.md`, `docs/roadmap.md`, an ADR, a spec of bytes that are no UTF-8, an empty idea) plus an ignored directory, an ignored file, an uncommitted edit to the tracked `CLAUDE.md` and a staged file: six rows, all `unregistered`; json `"item-count": 0`, `"fields": null`, `title` the H1 or `null`; nothing adopted, stamped, moved or staged | driver A1–A4 | `<W>/rA-cand.i59kDH` A1–A4 | **holds** — porcelain the same 11 lines both sides, 112 entries IDENTICAL |
| the same under `status.showUntrackedFiles=no` and `core.autocrlf=true`: byte-identical | driver A5 | A5a, A5b | **holds** (`cmp`) |
| `CLAUDE.md` a tracked link to a tracked `AGENTS.md`, `core.autocrlf=true`, `status.showUntrackedFiles=no`, setup, an untracked CRLF `VISION.md`: one `unregistered` row, `"title": "Vision"`; `CLAUDE.md` still the link | driver B1, B2 | `<W>/rB-cand.1ur85i` B1, B2 | **holds**. prev: same |
| `CLAUDE.md` linked to a file *outside* the repository: `jigc setup` refuses, `setup.inject-reference`, exit 1; `doc list` there is the not-set-up refusal | driver (aside to B) | `<W>/rM-out-cand.XAABbz` M4 | **holds on the candidate**, tree IDENTICAL around the refusal. **The previous release does not refuse:** exit 0, an install commit, and the outside file grows from 17 to 56 bytes. The driver's "Previous release: same" is said of B1 and B2, where it holds; for this aside the two binaries differ, and the candidate's is the safer answer. `jigc setup` is not this row's door. |

### 4. One doc set, two creation orders, concurrent readers

Three `research` docs, each minted, authored and finalized through the binary (`start --workflow
do-research` → `doc create research --title …` → three `doc set-slot` → the commit doc's `type`,
`scope`, `summary`, `body` → `task finalize`), an untracked file planted at the research home and
at the root after each. Order *Alpha, Beta, Gamma* in `<W>/rE-fwd.*`, *Gamma, Beta, Alpha* in
`<W>/rE-rev.*` (builder: `<W>/tools/order.sh`).

| claim | by | re-driven | result |
|---|---|---|---|
| `doc list` byte-identical across the two orders | driver E1 | E1 | **holds** — 393 bytes, sha256 `5bc8f3caa171e77a…` in both |
| `--format json` and `research --format json` byte-identical | driver E2, E3 | E2, E3 | **holds** — 1355 bytes, sha256 `4bec966f8f154eb0…` in both |
| eight `doc list --format json` started together: exit 0 × 8, each stdout byte-identical to E2, nothing under the root changed | driver E4 | E4 | **holds** |
| previous release: json byte-identical to the candidate's | driver | E1–E4 with the previous binary | **holds** (`cmp`) |

### 5. The invocation log — the one write

Root `<W>/rF-cand.Qm3M2L` (and `<W>/rF-prev.wk1wF7`): a fresh set-up repository, `jigc config get
invocation-log` → `invocation-log = false  (pack-default)`, then `jigc config set invocation-log
true` (exit 0, left uncommitted, as the source drove it).

| claim | by | re-driven | result |
|---|---|---|---|
| knob off: no write | both | F0-knob-off | **holds** — IDENTICAL |
| knob on, first listing: `.jigc/logs/` and `.jigc/logs/invocations.jsonl` (176 bytes, one record) are new, `.jigc`'s mtime moves, nothing else; plain porcelain unchanged (`?? .jigc/config/manifest.yaml`, `?? notes.md`); with `--ignored` it gains `!! .jigc/logs/invocations.jsonl` | both (source E1; driver F1, F6) | E1, F6 | **holds**, to the byte count |
| an exit-1 listing appends too: 176 → 380 bytes, `"finding_codes":["store.unknown-type"]` | both (source E2; driver F4) | E2 | **holds**, to the byte count |
| eight together: 2 → 10 records, the earlier file a byte prefix of the later, every line parses, nothing else changed, eight stdouts identical | driver F2 | F2 | **holds** |
| from a linked worktree the record lands in the **main checkout's** log; the worktree's `.jigc` gains no `logs` | driver F5 | F5 | **holds** |
| the log file mode 444: exit 0, no record, no message | driver F7 | F7 | **holds** — IDENTICAL, stderr empty; and with `.jigc/logs` mode 555 and no file (F7b): the same |
| the previous release appends one record of the same shape | driver F8 | every row above on the previous binary | **holds**. One byte-level difference, the round's change showing in the record: from the linked worktree the candidate logs `"output_bytes":542`, the previous release `36` — the 506 bytes are the stderr the candidate now prints there |
| a link at the log's path is written through | source S2 | S2 | **holds** — RC-4, with two more shapes |
| eight one after another, 10 → 18 records (driver F3) | driver | not driven as its own row | the sequential appends E1 → E2 → F2 → F5 are the same path; the prefix property held across all of them (380 bytes of 2110) |

The source files this write as a finding (S1) and the driver as a declared exception with no
finding. **Both observed the same bytes; they differ in classification only**, which is triage's.
It is returned once, as RC-3.

### 6. Entries that cannot be read as a file

Root `<W>/rH-cand.hwt69x` (and `<W>/rH-prev.*`): a fresh set-up repository with an untracked
`notes.md`; one shape at a time, each alone, removed before the next.

| claim | by | re-driven | result |
|---|---|---|---|
| a dangling link `docs/decisions/dangling.md`: exit 1, stdout empty, `reading the committed doc at "<abs>/docs/decisions/dangling.md": No such file or directory (os error 2)`; json: the same in an `{ "error": … }` envelope | both (source S3; driver H1, DL-1) | H1, H1j | **holds** |
| the filter for that doctype fails the same way; another doctype's filter does not walk the home and answers | source | H1adr, H1vision | **holds** — exit 1 / exit 0 |
| `jigc validate` over the same store: exit 0, the entry not named | source | H1validate | **holds** — `no findings — the committed store validates clean` |
| a directory at a **placement** home (`CHANGELOG.md`): exit 1, `Is a directory (os error 21)` | driver H2 | H2 | **holds** — the source's title says "at a located home"; the defect is at a placement home too |
| a directory at a located home; a mode-000 file | both | H3, H4 | **holds** — os error 21, os error 13 |
| a named pipe: the verb blocks; killed by an 8-second alarm, status 142, stdout and stderr empty | source | Hpipe | **holds** — exit 142, the verb printed nothing |
| a link to a file outside the repository is read through: listed `unregistered`, json `"title": "Heading outside the repository"` | driver H5, L4 | H5, H5j | **holds** — RC-13 |
| a name with a space; an upper-case name: listed with ids outside the slug grammar | driver H6, H7, L3 | H6, H7 | **holds** — RC-12 |
| every plant removed: byte-identical to the baseline | source | Hend | **holds** (`cmp`) |
| previous release: identical in every row | both | all rows on the previous binary | **holds** — every stdout and stderr byte-identical |
| "the door's two other hard reads … are guarded" (`doc.rs:4953`, `doc.rs:5040`) | source | groups O and T, below | **half holds — see RC-2**: the orphan read is guarded (and drops its row silently); the staged read is not |
| "Not driven: the orphan-row site … and the staged arm" | driver | groups O and T | **now driven** — RC-2, RC-8 |

### 7. The layouts whose `.git` is a file with no main checkout behind it

The source drove two of the three the doc comment of `repo::home_is_a_checkout` names (a
submodule, a `--separate-git-dir` checkout); the driver drove the third (a worktree of a bare
repository). **All three were driven here, on both binaries** (`<W>/rL-{sub,sep,bare}-{cand,prev}.*`,
builder `<W>/tools/groupL.sh`).

| claim | by | re-driven | result |
|---|---|---|---|
| submodule: `doc list` before setup exits 1 (*not set up*); `jigc setup` typed in the submodule exits 0; `doc list` then exits 0, `no committed docs`, stderr empty; the store is at `<super>/.git/modules/vendor/` | source S4 | SUB1, SUB2, SUB3 | **holds** |
| that `setup` writes `.claude`, `.jigc`, `CLAUDE.md` into `<super>/.git/modules/vendor/` and a hook at `<super>/.git/hooks/pre-commit`; the submodule's work tree holds only `.git`; both porcelains empty | source L1 | SUB2 | **holds** — and the lead's open question is answered: RC-6 |
| `--separate-git-dir`: `setup` exits 1 at `setup.install-hook`, leaving its install in the git directory's parent; `doc list` then exits 0 from there | source S4, L1 | SEP1–SEP3 | **holds** — thirteen new entries under `<dir>/store/` (`.jigc/` five files, `.claude/` two, `CLAUDE.md`, their directories) |
| bare worktree: `doc list` exits 1; `setup` exits 1 at `setup.install-hook` **after** writing its install into the folder that holds the bare repository and appending `## Project interface` / `@.jigc/AGENT.md` to a `CLAUDE.md` already there; the route re-run as printed exits 1 again (the hooks directory is mode `drwxr-xr-x`) | driver I1–I3, DL-3 | I1, I2, I3 | **holds** — `CLAUDE.md` 80 → 119 bytes, the three original lines kept as a prefix |
| then `doc list` exits 0, `no committed docs`; with an untracked `docs/research/outside.md` in that folder it lists `research:outside  docs/research/outside.md  unregistered`, stderr empty, and the file does not exist in the worktree | driver I4, I5, DL-2 | I4, I5 | **holds** |
| previous release: the same in all three layouts | both | `diff` of the two whole logs | **holds** — the logs differ only in the size of the installed skill file and of the hook (the binary's path is spelled into it) |

### 8. The source's counts and citations, read against the tree at eeffe347

| claim | checked by | result |
|---|---|---|
| `log_invocation(` — the definition and one call, `main.rs:117` | `grep -rn "log_invocation(" crates/cli/src` | **2 hits**: `invocation_log.rs:487` (definition), `main.rs:117` |
| `VERB_KINDS`: 48 rows, 12 `VerbKind::Read`; `(&["doc", "list"], VerbKind::Read)` at `cli.rs:2059` | `awk` over the const's body, counted for `VerbKind::` and `VerbKind::Read` | **48 and 12**; the row is at line 2059 |
| `LogWrite::for_leaf` answers `AppendOnly` for the teardown leaf alone | read `invocation_log.rs:470-479` | **holds** — one comparison, against `UNINSTALL_DOOR.verb` |
| `append_record`: `create_dir` then `.create(mint).append(true).open(…)`; the never-written-through sentence is said of `AppendOnly` only | read `invocation_log.rs:606-662` | **holds** |
| the read-verb rule and its one carve-out, `cli.rs:1992-1999` | read | **holds** — "mutates nothing — neither repo files, nor the git index/history, nor the `.jigc/` workbench", then "One carve-out": the derived cache |
| the fence never switches the knob on; one row for this leaf at `read_verb_acts_nothing.rs:398`; `.git` left out of its byte snapshot | `grep -n "invocation-log\|invocation_log\|logs/"` (exit 1, no hit); `grep -n '"doc", "list"'` (line 398); read lines 316-326 | **holds as text**. No test was run by this reporter: a stabilization reporter builds nothing |
| `committed_instances(` — 14 textual call sites | `grep -rn` over `crates/cli/src crates/engine/src`, per file | **15 hits = 14 calls + the definition** (`engine/index.rs` 4, of which one is `pub fn`) — the source's 14 stands |
| `fn run_list` holds two `std::fs::read` (`doc.rs:4866`, `:4953`); `reading the committed doc at` has one hit | read `doc.rs:4801-4977`; `grep -rn` | **holds** — driver's count; the staged arm adds one more, `doc.rs:5040` |
| `served_from_home_note`: no hit at the previous release's commit, 7 in `crates/cli/src` at tip | `git grep -c … 91834b5e -- crates/` (exit 1); `grep -rn … \| wc -l` | **holds** — the door list's derivation |
| the four git subprocesses, the pack's scan for write primitives, the engine's two `Command::new` hits | — | **not re-derived**: the source's own reading. What was driven agrees with it (no entry under `.git/` moves in 121 rows), with one bound — *Bounds*, the fsmonitor cell |

### 9. The driver's invocation count

The driver's verdict counts **86** invocations compared one by one: "120 labels, less the 34 on
the previous release, less one `jigc task list`, plus Row 1.1, compared by hand". A read-only
count of `*.man.before` under the driver's own directory (the one look outside this reporter's
scratch; nothing was opened but file names) gives **122** labels, **35** of them carrying `prev`
in the name, and one `task list` label. 122 − 35 − 1 = 86 without the "+1". The total is the
same; two operands are off by two and by one, and the hand-compared row would make 87. Nothing
rests on it: the count is not a verdict, and this report's own tally stands beside it.

## The tally

274 before/after pairs, by binary and verb (classified by a script over the label files,
`<W>/*.runs/`):

| binary | verb | condition | tree IDENTICAL | tree DIFFERS |
|---|---|---|---|---|
| candidate | `jigc doc list` | knob off | **121** | **0** |
| candidate | `jigc doc list` | `invocation-log` on | 3 | 7 — the log, or the link's target, and nothing else (RC-3, RC-4) |
| candidate | `jigc doc list` | `core.fsmonitor=true` | 0 | 1 — one directory mtime under `.git/`, git's own (*Bounds*) |
| candidate | other verbs (`setup`, `validate`, `task list`, `doc show`, `task validate`) | | 12 | 4 — the four `setup` rows of section 7 |
| previous | `jigc doc list` | knob off | **100** | **0** |
| previous | `jigc doc list` | `invocation-log` on | 3 | 7 |
| previous | `jigc doc list` | `core.fsmonitor=true` | 0 | 1 |
| previous | other verbs | | 10 | 5 — the four `setup` rows, and the `setup` that followed a link out of the repository |

Beside the pairs: two groups of eight concurrent readers on each binary (E4 with the knob off —
tree identical around the group; F2 with it on — only the log moved).

## Findings

Severity is information for triage. `door` is in the door list's words where the finding is this
row's door; any other door is named plainly and marked `unlisted`. **Every finding below behaves
the same on the previous release** (sha256 accf3996…) unless it says otherwise — none is a
regression. Each names the reports it reconciles.

### RC-1 — one entry named `*.md` that cannot be read fails the whole committed listing with a raw OS error and no route; a named pipe hangs it; `jigc validate` over the same store says nothing

- **reconciles:** source S3 · driver DL-1 — the same defect, found twice. Agreed in every cell both drove; each drove cells the other did not, and all of them hold.
- **door:** `jigc doc list`
- **clause:** `none` — nothing is written or lost (tree IDENTICAL in all eleven cells); the refusal prints no route, so no route fails as printed; not a regression.
- **severity:** medium for the verb's own job, low against the closing condition. A dangling link or a directory named `x.md` in a docs folder is an ordinary accident, and it turns the one index read an agent is told to use into exit 1 with zero rows, the machine's absolute path, and the words "the committed doc" for an entry git never held.
- **class:** an enumerator admits an entry by name, and its consumer reads it with a bare `?`.
- **count_derivation:** site `doc.rs:4866` (`std::fs::read(&path)…?` over `engine::index::committed_instances`, which filters on the extension `md` and — on the placement branch — on `path.exists()`), driven in **eleven cells**: a dangling link at a located home (unfiltered, json, that doctype's filter), a directory at a placement home, a directory at a located home, a mode-000 file, a named pipe; plus the two controls (another doctype's filter, every plant removed) and `validate`. A dangling link at a *placement* home was not driven: by the source, `exists()` is false for it and the row is absent. The door's other read sites are RC-2. Beyond this door: `committed_instances(` has 14 call sites (section 8); the source read four, this reporter one — **an instance in this door, unbounded across the mechanism's consumers.**

#### Repro RC-1

```yaml
claim: "one untracked entry named *.md that cannot be read as a file, at a home `jigc doc list` enumerates, makes the listing exit 1 with no rows and no route; `jigc validate` over the same store exits 0 without naming it"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "byte-identical stdout and stderr in every cell (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
setup:
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["jigc", "setup"]                       # in repo; exit 0
  - ["mkdir", "-p", "docs/decisions"]
  - ["ln", "-s", "/nonexistent/nowhere.md", "docs/decisions/dangling.md"]
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "list", "vision"]
  - ["jigc", "validate"]
expect:
  - exit: 1
    stdout: ""
    stderr: "reading the committed doc at \"<abs repo>/docs/decisions/dangling.md\": No such file or directory (os error 2)"
  - exit: 0
    stdout: "jigc doc list — no committed `vision` docs"
  - exit: 0
    stdout_contains: "no findings — the committed store validates clean"
variants:        # each alone, in place of the `ln`
  - "mkdir CHANGELOG.md                      -> exit 1, `Is a directory (os error 21)`   (a placement home)"
  - "mkdir -p docs/research/a-dir.md         -> exit 1, `Is a directory (os error 21)`"
  - "a mode-000 docs/ideas/locked.md         -> exit 1, `Permission denied (os error 13)`"
  - "mkfifo docs/research/pipe.md            -> no exit: killed by an 8 s alarm, status 142, nothing printed"
control: "every plant removed -> exit 0, byte-identical to the listing before any plant"
observed: "<W>/rH-cand.hwt69x, labels H0, H1, H1j, H1adr, H1vision, H1validate, H2, H3, H4, Hpipe, Hend"
pinned-by: "UNPINNED: no test found for an unreadable entry at a managed home on this door (the source's search; not repeated here)"
```

### RC-2 — the staged arm has two failing sites of its own, and a broken entry in a task's staging area silently removes the committed listing's staged note

- **reconciles:** the source's "the door's two other hard reads were read and are guarded" · the driver's "for the staged arm this is `instance, unbounded`". **The source's sentence does not reproduce for `doc.rs:5040`.** Its guard is a `metadata(...).is_file()` in `task::staged_doc_ids` (`task.rs:1227`): a stat, not a read. A mode-000 staged copy passes the stat and fails the read; and a dangling link fails the stat itself, which that function returns as an error by design. For `doc.rs:4953` the guard *is* a read (`orphan::carries_stamp`) and holds — RC-8 has what it costs.
- **door:** `jigc doc list`
- **clause:** `none` — nothing written; the states sit inside the gitignored workbench, where only jigc writes in documented use.
- **severity:** low.
- **class:** as RC-1 — enumerate by name, then read or stat with a bare `?`.
- **count_derivation:** the staged arm, `fn run_list_staged` (`doc.rs:5006-5064`), read end to end: two fallible filesystem steps, `staged_doc_ids` at `:5023` and `std::fs::read` at `:5040`. Four shapes planted in `.jigc/tasks/<id>/docs/`, each alone: a directory `research:a-dir.md` → **skipped**, exit 0; a named pipe → **skipped**, exit 0; a dangling link → **exit 1** at `:5023`; a mode-000 file → **exit 1** at `:5040`. So in this door **three sites** end the listing on an entry they cannot read — `doc.rs:4866`, `doc.rs:5023`, `doc.rs:5040` — and one (`doc.rs:4953`) does not. `staged_doc_ids(` has other callers (`task discard`'s ack, the `uninstall` guard, `staged_listing_hint`); only the last was driven: **unbounded beyond this door.**
- **the third thing seen:** with the dangling link in the task's staging area, the **committed** arm still exits 0 with its five rows, and its stderr is **empty** — the `note: docs are also staged in open task …` line that the same rig prints without the plant is gone (`staged_listing_hint` takes the enumerator's error as an empty list).

#### Repro RC-2

```yaml
claim: "`jigc doc list --task <id>` exits 1 with a raw OS error when the task's staging area holds a staged-doc-named entry that is a dangling link or a mode-000 file; a directory or a named pipe there is skipped"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "byte-identical stdout and stderr in every cell"
setup:
  - fixture: refs-post-hoc                  # live task ground-the-vision-in-research staging two docs
  - ["ln", "-s", "/nonexistent/x.md", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md"]
repro:
  - ["jigc", "doc", "list", "--task", "ground-the-vision-in-research"]
  - ["jigc", "doc", "list"]
expect:
  - exit: 1
    stdout: ""
    stderr: "listing the docs staged in task `ground-the-vision-in-research`: No such file or directory (os error 2)"
  - exit: 0
    stdout: "header + five `managed` rows"
    stderr: ""                              # the staged-listing note is absent
variants:        # each alone, in place of the `ln`
  - "a mode-000 research:locked.md           -> exit 1, `reading the staged doc at \"<abs>/.jigc/tasks/<id>/docs/research:locked.md\": Permission denied (os error 13)`"
  - "mkdir research:a-dir.md                 -> exit 0, the two staged rows (skipped)"
  - "mkfifo research:pipe.md                 -> exit 0, the two staged rows (skipped; no hang)"
observed: "<W>/jigc-rig-refs-post-hoc-* named in <W>/rigT-cand.rig, labels T0, T1-dir, T2-dangling, T2-dangling-committed-arm, T3-locked, T4-pipe, Tend"
pinned-by: "UNPINNED: found this round"
```

### RC-3 — under the opt-in `invocation-log` knob the read verb creates `.jigc/logs/` and appends to its log, and the read-verb rule names no such write

- **reconciles:** source S1 (a finding) · the driver's group F (a declared exception, no finding). Same bytes, two classifications; returned once.
- **door:** `jigc doc list`
- **clause:** `none` — opt-in, gitignored, append-only as driven (the earlier log a byte prefix of the later across every append), nothing destroyed, nothing committed, plain porcelain unchanged.
- **severity:** low — the behaviour is designed (`design/measurement.md`, per both reports); what is off is that the registry's rule text and its fence do not admit it and cannot see it.
- **class:** a write a `Read` leaf makes that the read-verb rule does not name.
- **count_derivation:** the source's, re-derived: one call site of the writer (`main.rs:117`), reached by every leaf; `LogWrite::for_leaf` is `AppendOnly` for one leaf; `VERB_KINDS` holds 48 rows, 12 of them `Read` (section 8). **The class is the 12 `Read` rows; one was driven.**

#### Repro RC-3

```yaml
claim: "with the invocation-log knob on, `jigc doc list` at exit 0 creates .jigc/logs/ and .jigc/logs/invocations.jsonl; plain `git status --porcelain` does not move"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same entries, the same record shape"
setup:
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["jigc", "setup"]
  - ["jigc", "config", "set", "invocation-log", "true"]      # exit 0, left uncommitted
  - "write notes.md, one line, and do not add it"
repro:
  - ["jigc", "doc", "list"]
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs"
  porcelain_before_and_after: ["?? .jigc/config/manifest.yaml", "?? notes.md"]
  new_entries: [".jigc/logs/", ".jigc/logs/invocations.jsonl — 176 bytes, one JSON line: argv [\"doc\",\"list\"], exit_code 0, output_bytes 36"]
  also_moved: ".jigc (mtime)"
  nothing_else_changed: true
observed: "<W>/rF-cand.Qm3M2L, labels F0-knob-off (control), E1, E2, F2 group, F5, F6, F7, F7b"
pinned-by: "UNPINNED: read_verb_acts_nothing drives every Read leaf with the knob off (no hit for `invocation-log` in the file), so no standing test observes this write"
```

### RC-4 — the log's opening write follows a link: it appends to a link's target, creates a dangling link's target, and writes into a directory `.jigc/logs` links to

- **reconciles:** source S2, which the driver has no counterpart for. **It reproduces, and two more shapes of the same site do what the source did not drive.**
- **door:** `jigc doc list`
- **clause:** `none` — as driven, no byte is destroyed in any shape: an existing target keeps its bytes as a prefix, and the other two shapes create a file that was not there. The states are links placed inside the gitignored workbench. The run's opening declares no bound, so this is returned and not dropped; the reading does not lean on one.
- **severity:** low. What it is: with the knob on, a verb that only reads writes **outside the repository** at exit 0, wherever a link under `.jigc/logs` points.
- **class:** an opening write (`create(true).append(true)`) preceded by a `create_dir` that treats *already exists* as success, with no check of what exists.
- **count_derivation:** one site, `invocation_log.rs:638-659`, the `Mint` arm; **four shapes driven** — a link at the file to an existing file (appended), a dangling link at the file (**target created**, 176 bytes), `.jigc/logs` a link to an existing directory outside the repository (**`invocations.jsonl` created there**), `.jigc/logs` a dangling link (nothing written, exit 0, no message). The site is reached by 47 of the 48 leaves (RC-3's derivation). The crate's other writers were not enumerated against this shape — the run's range carries a suite named `replacing_writers_never_follow`, which neither the source nor this reporter read against this site: **an instance, unbounded.**

#### Repro RC-4

```yaml
claim: "with the knob on and a link under .jigc/logs, `jigc doc list` at exit 0 writes its record where the link points — outside the repository"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "identical in all four shapes"
setup:
  - "the setup of Repro RC-3, then one `jigc doc list` to start the log"
  - "move .jigc/logs/invocations.jsonl out of the repository"
  - "write <root>/target.txt = `user bytes\n`"
  - ["ln", "-s", "<root>/target.txt", ".jigc/logs/invocations.jsonl"]
repro:
  - ["jigc", "doc", "list"]
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs"
  stderr: ""
  target_txt: "11 -> 187 bytes: line 1 `user bytes` unchanged, line 2 the record for argv [\"doc\",\"list\"]"
  nothing_else_changed: true
variants:        # each alone
  - "the link dangling (-> <root>/created-by-jigc.txt, absent)   -> exit 0; that file now exists, 176 bytes, one record"
  - ".jigc/logs itself a link to <root>/outside-dir (existing)    -> exit 0; <root>/outside-dir/invocations.jsonl created, 176 bytes"
  - ".jigc/logs a dangling link                                   -> exit 0; nothing written, nothing printed"
observed: "<W>/rF-cand.Qm3M2L, labels S2, S2b, S2c, S2d"
pinned-by: "UNPINNED: the source names invocation_log::tests::an_append_only_write_creates_nothing_and_still_appends_to_a_log_that_is_there as the pin of the AppendOnly arm — a different arm; not opened here"
```

### RC-5 — where `.git` is a file and no main checkout stands behind it, the listing is read at a directory that is no checkout, at exit 0 and with no note

- **reconciles:** source S4 (submodule, `--separate-git-dir`) · driver DL-2 (a worktree of a bare repository). One class, three layouts, two of three driven by one and the third by the other. **All three driven here; every cell of both reports holds.**
- **door:** `jigc doc list`
- **clause:** `none` for this door — it reads, and what it reads at the wrong place it only reports.
- **severity:** low here. Already on record, per both reports: `implementation/decisions-pending.md` → (D), trigger M57, with the open question whether these layouts are supported. This reporter did not re-read that entry.
- **class:** `repo::jigc_home` answers the parent of the git common directory wherever `.git` is a file; the round's served-from note is silent there by design (`home_is_a_checkout` reads a probe that cannot answer as `false`).
- **count_derivation:** the three layouts the doc comment of `home_is_a_checkout` enumerates (the driver's read; the source cites `repo.rs:148-155`) — **3 of 3 driven**, on both binaries. The door is one of 48 leaves that resolve their home the same way; no other leaf was driven for this finding.

#### Repro RC-5

```yaml
claim: "in a worktree of a bare repository, after `jigc setup` was typed there, `jigc doc list` exits 0 serving the folder that holds the bare repository, and prints no served-from line"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "identical"
setup:
  - ["mkdir", "proj"]
  - ["git", "init", "-q", "--bare", "store.git"]                              # cwd proj
  - ["git", "-C", "store.git", "worktree", "add", "-q", "../wt", "-b", "main"]
  - "in proj/wt: write README.md, `git add README.md`, `git commit -q -m base`"
  - ["jigc", "setup"]                       # cwd proj/wt; exit 1 — Repro RC-6
  - "write proj/docs/research/outside.md = `# Outside any repository\n`"
repro:
  - ["jigc", "doc", "list"]                 # cwd proj/wt
expect:
  exit: 0
  stdout: "id  path  state\nresearch:outside  docs/research/outside.md  unregistered\n"
  stderr: ""
  note: "proj/wt/docs/research/outside.md does not exist; proj/wt holds README.md alone"
  tree: IDENTICAL
variants:
  - "a submodule (super/vendor/lib), after `jigc setup` there (exit 0)       -> exit 0, `no committed docs`, stderr empty; the store is super/.git/modules/vendor/.jigc"
  - "`git init --separate-git-dir <dir>/store/sep.git work`, after `jigc setup` (exit 1) -> exit 0, `no committed docs`, served from <dir>/store/"
observed: "<W>/rL-bare-cand.pWoLXM (I4, I5), <W>/rL-sub-cand.jJPSY2 (SUB1, SUB3), <W>/rL-sep-cand.Y4RVOS (SEP1, SEP3)"
pinned-by: "UNPINNED here: the source names linked_worktree_doc_home::a_layout_whose_doc_home_is_no_checkout_lands_a_doc_as_it_did_before_the_guard and did not open it; neither did this reporter"
```

### RC-6 — `jigc setup` in those three layouts: it installs outside any work tree, and in two of them it exits 1 after writing, with a route that loops

- **reconciles:** driver DL-3 (bare worktree) · source lead L1 (submodule, and the `--separate-git-dir` sibling). Both hold. **The lead's open question — what happens to a `pre-commit` hook the superproject already has — is answered: its body survives.** A 41-byte hook of the superproject's (`#!/bin/sh` and one `echo`) became a 5187-byte file: jigc's managed block first, then the user's `echo` line, after the block's end marker. No byte of the user's hook body is lost; the superproject's commits now run `jigc validate` first.
- **door:** `jigc setup` — **unlisted** (an excluded door of the round; reached by running this row's printed route, `jigc setup`, in these layouts).
- **clause:** `working-product`, as the driver read it and this reporter does — "every refusal's route works as printed": in two layouts `doc list`'s refusal routes at `jigc setup`, which exits 1; that refusal routes at "ensure the repo's git hooks directory is writable, then re-run `jigc setup`", the hooks directory is writable, and the re-run exits 1 again. **In scope only if the layout is a supported one — the open question on record.** Not `no-lost-files` as scoped: the writes land at exit 1, and the one user file touched (`CLAUDE.md` beside the bare repository) is appended to, its three lines kept.
- **severity:** medium if the layouts are supported; none if a refusal is to say they are not.
- **class / count_derivation:** the same three layouts as RC-5 — **3 of 3 driven** for this door: submodule → exit 0, thirteen new entries under `<super>/.git/modules/vendor/` and the superproject's hook rewritten; `--separate-git-dir` → exit 1 after thirteen new entries under the git directory's parent; bare worktree → exit 1 after twelve new entries and one appended file, and the re-run exits 1 having rewritten five of them in place. The driver's "(D) says 2 of them exit 1 after writing" matches what was driven.

#### Repro RC-6

```yaml
claim: "`jigc setup` typed in a worktree of a bare repository writes its install into the folder that holds the bare repository, appends to a CLAUDE.md there, exits 1, and its route re-run as printed exits 1 again"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "identical but for the size of the installed skill file"
setup:
  - "the first four steps of Repro RC-5, with proj/CLAUDE.md = `# my folder-level agent rules\n\nkept beside the bare store, in no git repository\n` written before them"
repro:
  - ["jigc", "setup"]                       # cwd proj/wt
  - ["jigc", "setup"]                       # the route, as printed
expect:
  exit: 1                                   # both times
  stdout: ""
  stderr: "blocking · setup.install-hook — cannot install the `pre-commit` hook into the repo's hooks dir: git could not resolve the hooks dir for `<proj>`: fatal: not a git repository (or any of the parent directories): .git\n  route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`\n…"
  created_in_proj: [".jigc/.gitignore", ".jigc/AGENT.md", ".jigc/config/.gitkeep", ".jigc/config/packs.yaml", ".jigc/version", ".claude/settings.json", ".claude/skills/jigc/SKILL.md"]
  proj_CLAUDE_md: "80 -> 119 bytes: the three original lines, then `\n## Project interface\n\n@.jigc/AGENT.md\n`"
  proj_wt: "porcelain empty, one commit, README.md alone"
variants:
  - "a submodule: exit 0; .claude, .jigc, CLAUDE.md land in super/.git/modules/vendor/; super/.git/hooks/pre-commit is rewritten with the managed block first and the superproject's own hook body after it"
  - "`--separate-git-dir`: exit 1 at the same code, after the same install lands in the git directory's parent"
observed: "<W>/rL-bare-cand.pWoLXM (I2, I3), <W>/rL-sub-cand.jJPSY2 (SUB2; hook.before beside it), <W>/rL-sep-cand.Y4RVOS (SEP2)"
pinned-by: "UNPINNED: recorded, per the driver, at implementation/decisions-pending.md → (D), trigger M57"
```

### RC-7 — the standing read-verb fence observes this door in one form only

- **reconciles:** source S5, a reading of a test file; the driver has no counterpart. **The citations hold as text** (section 8): one table row for the leaf, the knob never on, `.git` out of the byte snapshot. No test was run.
- **door:** `jigc doc list`
- **clause:** `none` — no defect is hidden behind it as far as three reporters' drives reach.
- **severity:** low.
- **class:** input coverage of a fence over a multi-arm leaf.
- **count_derivation:** the source's: one row (`read_verb_acts_nothing.rs:398`) for a leaf with two arms. The other eleven `Read` rows' tables were not read by anyone: **an instance, unbounded.** What this pass adds to the source's own drives of the unfenced forms: the staged arm, the filter, json, a linked worktree, a detached one, a nested one, a clone, a fan-out worktree, and the three no-checkout layouts — every one write-free with `.git/` and the home directory inside the snapshot.

#### Repro RC-7

```yaml
claim: "`jigc doc list --task <id>` — the arm the read-verb fence does not drive — writes nothing, .git/ and HOME included"
verdict: CONFIRMED (the arm is write-free; that the fence does not hold it is the source's reading of the test file, verified as text only)
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
setup:
  - fixture: refs-post-hoc
  - "write notes.md, scratchdir/untracked.md and docs/research/draft-untracked.md, and do not add them"
repro:
  - ["jigc", "doc", "list", "--task", "ground-the-vision-in-research"]
expect:
  exit: 0
  stdout: "id  path  state\ncommit:ground-the-vision-in-research  commit:ground-the-vision-in-research  managed\nvision:vision  VISION.md  managed\n"
  stderr: ""
  tree: "IDENTICAL, mtime included — 150 entries"
observed: "<W>/jigc-rig-refs-post-hoc-5jGyCP, label C3; from a linked worktree, D5 (byte-identical stdout)"
pinned-by: "UNPINNED: read_verb_acts_nothing::no_read_verb_acts_over_the_revealing_state drives the bare form only (the source's reading)"
```

### RC-8 — one tracked path whose name is not UTF-8 removes every `orphaned` row from the listing, at exit 0 and without a word

- **reconciles:** source lead L2 — *not driven* there, "the filesystem these drives ran on refuses a file name that is not UTF-8". **Driven here and CONFIRMED.** The state needs no such file on disk: one index entry is enough (`git update-index --add --cacheinfo`), because the walk reads `git ls-files -z`.
- **door:** `jigc doc list`
- **clause:** `none` for this door — a read that reports less than is there; nothing written.
- **severity:** low for this door; see RC-9 for where the same state costs more.
- **class:** `orphan::committed_markdown` asks `git ls-files -z` through `task::git_capture`, which fails the whole capture unless git's output is UTF-8, and turns that failure into an empty listing.
- **count_derivation:** `grep -rn "committed_markdown(" crates/cli/src` — **6 hits = the definition + 5 call sites**: `orphan.rs:222` (the strand walk, `orphaned_docs`) and `orphan.rs:616` (`orphaned_instances`) are this door's two; `config.rs:1512`, `orphan.rs:1116` and `relocate.rs:708` were not read or driven (lead LD-2). `git_capture(` itself has 37 textual hits in `crates/cli/src`, every one sharing the UTF-8 step; none but this one was examined. **Driven: this door's two consumers, through one state. Unbounded beyond them.**
- **reach, stated plainly:** built with git plumbing on macOS. On a file system that admits such a name it is an ordinary committed file (a legacy Latin-1 name); that was **not** driven. The sibling shape that needs no odd name: `chmod 000` on the tracked orphan itself drops its row the same way (`carries_stamp` reads an unreadable file as unstamped) — driven, label O2.

#### Repro RC-8

```yaml
claim: "with one index entry whose name is not valid UTF-8, `jigc doc list` no longer prints the `orphaned` row of a stamped committed file no doctype claims"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "byte-identical stdout and stderr in every cell"
setup:
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["jigc", "setup"]
  - "write docs/zzz/orphan.md = `---\nschema-version: 1\n---\n# An orphan nobody claims\n\nbody\n`"
  - ["git", "add", "docs/zzz/orphan.md"]
  - ["git", "commit", "-q", "--no-verify", "-m", "an orphan"]
  - control: ["jigc", "doc", "list"]        # exit 0: `(none)  docs/zzz/orphan.md  orphaned`
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]     # exit 0
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "list", "--format", "json"]
expect:
  - exit: 0
    stdout: "jigc doc list — no committed docs"
    stderr: ""
  - exit: 0
    stdout: "{ \"docs\": [] }"
variants:
  - "instead of the index entry: chmod 000 docs/zzz/orphan.md   -> exit 0, `no committed docs`"
observed: "<W>/rO-cand.z6EWfu, labels O1, O1j (control), O2, O3, O3j"
pinned-by: "UNPINNED: found this round, from the source's lead"
```

### RC-9 — the same state turns `jigc validate` from exit 1 with a blocking finding into exit 0 and *the committed store validates clean*

- **reconciles:** the source's L2 says of the store sweep only that the same enumerator feeds its blocking finding, "not this row's door, and not read further". **Driven here, on the way.**
- **door:** `jigc validate` — **unlisted** (not a door of this item or of the round's test set).
- **clause:** `none` as this reporter reads the four — no bytes lost, nothing committed, the previous release does the same. It is returned for its tier: a blocking verdict that an unrelated file name switches off, with a message that says the store is clean.
- **severity:** medium. The pre-commit hook `jigc setup` installs runs this same sweep.
- **class / count_derivation:** as RC-8 — the same enumerator, `orphan.rs:616`, reached from the store sweep. One state, one door, both binaries. The other blocking findings of the sweep that read `committed_markdown` (the strand walk at `orphan.rs:222`) were not driven under this state: **an instance, unbounded.**

#### Repro RC-9

```yaml
claim: "over a store holding a stamped committed file no doctype claims, `jigc validate` exits 1 with a blocking `schema-conformance.orphaned-instance`; add one index entry whose name is not UTF-8 and it exits 0 with `no findings`"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "byte-identical"
setup:
  - "the setup of Repro RC-8 up to and including the commit of docs/zzz/orphan.md"
  - control: ["jigc", "validate"]           # exit 1; stdout opens `blocking · schema-conformance.orphaned-instance — committed doc `docs/zzz/orphan.md` sits at a jigc-managed home and carries a `schema-version:` stamp, but no resolved doctype claims this path`
  - "the `git update-index --add --cacheinfo` step of Repro RC-8"
repro:
  - ["jigc", "validate"]
expect:
  exit: 0
  stdout: "no findings — the committed store validates clean\n— jigc · run `jigc start` for orientation; all writes through `jigc`.\n"
  stderr: ""
variants:
  - "instead of the index entry: chmod 000 docs/zzz/orphan.md   -> exit 0, the same `no findings` line"
observed: "<W>/rO-cand.z6EWfu, labels O1v (control, exit 1, 1191 bytes), O2v, O3v"
pinned-by: "UNPINNED: found this round"
```

### RC-10 — from a worktree nested below the main checkout, the served-from line spells the worktree relative to the main checkout, not to the reader

- **reconciles:** driver lead L1; the source has no counterpart. **Reproduces.** This is the one finding inside code the round added: the previous release prints no such line.
- **door:** `jigc doc list`
- **clause:** `none`.
- **severity:** low — a path the reader cannot resolve from where the line says they are standing.
- **class / count_derivation:** `instance, unbounded` — one cell driven (`git worktree add <repo>/.trees/nested -b nested`). The design names three verbs that print the line (`doc show`, `doc list`, `validate`); the other two were not driven from a nested worktree. Whether the surface contract's repo-relative rule intends this spelling was not established by the driver, and not by this reporter.

#### Repro RC-10

```yaml
claim: "from a linked worktree created below the main checkout, `jigc doc list` names the main checkout absolute and the worktree as `.trees/nested`"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "no such line at all — the note is the round's"
setup:
  - fixture: refs-post-hoc
  - ["git", "worktree", "add", "-q", "<repo>/.trees/nested", "-b", "nested"]
repro:
  - ["jigc", "doc", "list"]                 # cwd <repo>/.trees/nested
expect:
  exit: 0
  stdout: "byte-identical to the main checkout's"
  stderr_last_line: "note: served from the main checkout at `<abs repo>` — jigc's doc store has one home, not the linked worktree at `.trees/nested` on branch `nested` you are standing in"
observed: "<W>/jigc-rig-refs-post-hoc-5jGyCP, label D8"
pinned-by: "UNPINNED: found this round"
```

### RC-11 — in a repository that was never set up, a read with `--task` answers *no such task* and routes at `jigc task list`; nothing says the project is not set up

- **reconciles:** driver lead L2 ("instance, unbounded — I did not enumerate the other `--task` read arms"). **Reproduces, and the two other `--task` reads driven here do the same.**
- **door:** `jigc doc list`
- **clause:** `none` — the refusal is true as far as it goes and its route runs (exit 0, `jigc task list — no active tasks`).
- **severity:** low.
- **class:** a `--task` arm that resolves the task before it asks for the project layer (`fn run_list` returns into `run_list_staged` on its first lines, before `require_project_layer`).
- **count_derivation:** **3 of 3 driven** `--task` reads answer `finalize.no-task` in a never-set-up repository — `jigc doc list --task nope`, `jigc doc show adr:x --task nope`, `jigc task validate nope` — while the same `doc show` without `--task` answers the not-set-up line. The registry's other task-addressed leaves were not enumerated: **unbounded.**

#### Repro RC-11

```yaml
claim: "in a git repository `jigc setup` never ran in, `jigc doc list --task nope` exits 1 with `finalize.no-task` and a route at `jigc task list`, and that route exits 0 with an empty roster"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "byte-identical"
setup:
  - ["git", "init", "-q", "repo"]
  - "one commit of README.md"
repro:
  - ["jigc", "doc", "list", "--task", "nope"]
  - ["jigc", "task", "list"]
  - ["jigc", "doc", "list"]
expect:
  - exit: 1
    stderr: "blocking · finalize.no-task — no task `nope`\n  at: task:nope\n  route: `jigc task list` lists the live tasks\n— jigc · run `jigc start` for orientation; all writes through `jigc`.\n"
  - exit: 0
    stdout: "jigc task list — no active tasks\n— jigc · run `jigc start` for orientation; all writes through `jigc`.\n"
  - exit: 1
    stderr: "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
variants:
  - "jigc doc show adr:x --task nope   -> exit 1, finalize.no-task"
  - "jigc task validate nope           -> exit 1, finalize.no-task"
  - "jigc doc show adr:x               -> exit 1, the not-set-up line"
observed: "<W>/r1-bare-cand.658WdO (1.5, 1.6, 1.5route, 1.1); <W>/rM-l2-cand.Jofa1A (M3-show-task, M3-show, M3-taskvalidate)"
pinned-by: "UNPINNED: found this round"
```

### RC-12 — the listing prints an identity `jigc doc show` refuses as malformed, and that refusal's route points back at the listing

- **reconciles:** driver lead L3 ("not pursued: whether those ids are accepted …"). **Reproduces; the follow-up was driven for the read door.**
- **door:** `jigc doc list`
- **clause:** `none` as read — the route (`jigc doc list`) runs and exits 0. It is a loop all the same: the listing is where the reader got the identity the refusal sends them back to look up. Triage may read the second clause's "every refusal's route works as printed" more strictly than this.
- **severity:** low. The state is ordinary: any untracked `README.md` or `My Notes.md` in a located doctype's folder.
- **class / count_derivation:** `instance, unbounded` — two names driven (a space, upper case) at one located home, through `doc list` and `doc show`. The plain listing separates columns with two spaces, so a name with a space also makes the row ambiguous (the driver's point). The adoption verbs the `unregistered` state points at were not driven with such a name.

#### Repro RC-12

```yaml
claim: "an untracked docs/research/UPPER.md is listed as `research:UPPER`; `jigc doc show research:UPPER` exits 1 `store.malformed-slug` with a route at `jigc doc list`"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "byte-identical"
setup:
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["jigc", "setup"]
  - "write docs/research/UPPER.md = `# Upper\n` and docs/research/has space.md = `# Spaced\n`; add neither"
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "show", "research:UPPER"]
expect:
  - exit: 0
    stdout: "id  path  state\nresearch:UPPER  docs/research/UPPER.md  unregistered\nresearch:has space  docs/research/has space.md  unregistered\n"
  - exit: 1
    stderr: "blocking · store.malformed-slug — \"UPPER\" is not a valid doc slug — the `<slug>` head of address `research:UPPER`\n  route: `jigc doc list` lists the committed docs and the identity each one carries; use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)\n"
observed: "<W>/rM-l3-cand.s5W4tG (M3-list, M3-show-space, M3-show-upper); <W>/rH-cand.hwt69x (H6, H7)"
pinned-by: "UNPINNED: found this round"
```

### RC-13 — a link at a managed home that leads out of the repository is read through, and the outside file's heading is printed as the row's title

- **reconciles:** driver lead L4. **Reproduces.**
- **door:** `jigc doc list`
- **clause:** `none` — a read; nothing written. A link placed at a managed home is a planted state; the opening declares no bound, so it is returned.
- **severity:** low. The contrast the driver drew holds on the candidate: `jigc setup` refuses to follow a `CLAUDE.md` link out of the repository for a write (section 3, M4); the listing follows one for a read.
- **class / count_derivation:** `instance, unbounded` — one cell (a spec home), two formats. `doc show` over such a row was not driven.

#### Repro RC-13

```yaml
claim: "an untracked link docs/specs/out-link.md -> a file outside the repository is listed `unregistered`, and --format json carries the outside file's H1 as `title`"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "byte-identical"
setup:
  - "the first three steps of Repro RC-12"
  - "write <root>/outside.md = `# Heading outside the repository\n`   (beside the repository, not in it)"
  - ["ln", "-s", "<root>/outside.md", "docs/specs/out-link.md"]
repro:
  - ["jigc", "doc", "list", "--format", "json"]
expect:
  exit: 0
  stdout_json: { "docs": [ { "id": "spec:out-link", "path": "docs/specs/out-link.md", "state": "unregistered", "item-count": 0, "title": "Heading outside the repository", "fields": null } ] }
  tree: IDENTICAL
observed: "<W>/rH-cand.hwt69x, labels H5, H5j"
pinned-by: "UNPINNED: found this round"
```

## Leads — noticed, not pursued

### LD-1 — under `core.fsmonitor=true` the unfiltered listing moves one directory's mtime under `.git/`, through git's own daemon

- **door:** `jigc doc list` · **clause:** `none` · **severity:** none seen — a bound on the verdict's "mtime included", not a defect.
- **what was seen:** a set-up repository with `core.untrackedCache=true` and `core.fsmonitor=true`, the daemon running. Three trials: an idle second moves nothing; `jigc doc list` moves the mtime of `.git/fsmonitor--daemon/cookies` and nothing else. **Control:** `git ls-files -z` alone does the same; `jigc doc list adr` — the filtered form, which runs no `ls-files` — moves nothing. So it is the cookie handshake of the fourth subprocess the source lists, in git's own directory. Same on the previous release.
- **not pursued:** a filter driver, a sparse or split index, an index that needs a refresh. `instance, unbounded`. Both daemons were stopped and report *not watching*.

#### Repro LD-1

```yaml
claim: "with core.fsmonitor=true and the daemon running, `jigc doc list` changes the mtime of .git/fsmonitor--daemon/cookies and nothing else; `jigc doc list adr` changes nothing"
verdict: OBSERVED (a lead; git's own write, shown by the control)
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
setup:
  - "the first three steps of Repro RC-12"
  - ["git", "config", "core.untrackedCache", "true"]
  - ["git", "config", "core.fsmonitor", "true"]
  - ["git", "status", "--porcelain"]        # starts the daemon
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "list", "adr"]
  - ["git", "ls-files", "-z"]               # the control
expect:
  - changed_under_dot_git: ["fsmonitor--daemon/cookies (mtime)"]
  - changed_under_dot_git: []
  - changed_under_dot_git: ["fsmonitor--daemon/cookies (mtime)"]
observed: "<W>/rM-fsm-cand.tlDI5R — label M2-list, and files c.before / c.mid / c.after"
pinned-by: "UNPINNED: a lead"
```

### LD-2 — the three other consumers of the listing that a non-UTF-8 name empties

- **door:** unlisted — `config.rs:1512`, `orphan.rs:1116`, `relocate.rs:708` call `orphan::committed_markdown`; by its doc comment the last is the freeze-exempt relocation path, which "walks the same committed truth to find the instances stranded at a supplied prior home".
- **clause:** unknown — if a migration's strand walk goes empty over such a repository, the fourth clause is the one it would touch; **nothing here establishes that**. Not read, not driven.
- **class / count_derivation:** RC-8's grep — 5 call sites, 2 driven, **3 not examined**.

#### Repro LD-2

```yaml
claim: "with one index entry whose name is not UTF-8, the verbs behind config.rs:1512, orphan.rs:1116 and relocate.rs:708 see an empty committed-markdown listing"
verdict: NOT DRIVEN (a lead from RC-8's enumeration)
setup:
  - "the setup of Repro RC-8, including the index entry"
repro:
  - "the verb each call site serves — not identified here"
expect: "not established"
pinned-by: "UNPINNED: not driven"
```

### LD-3 — one platform

- **door:** `jigc doc list` · **clause:** `none`.
- Every cell of this report, and of the two it reconciles, ran on one macOS host and one git. The shapes of RC-1 and RC-2 that depend on permissions, and RC-8's file-name state as an ordinary file, were not driven on Linux; a repository moved after setup and an older git (the driver's *not driven*) were not driven here either.

#### Repro LD-3

```yaml
claim: "the cells of RC-1, RC-2 and RC-8 behave the same on Linux"
verdict: NOT DRIVEN
setup: "as each finding's block, on a Linux host; for RC-8 a real committed file whose name holds the byte 0xFF"
repro: "as each finding's block"
expect: "not established"
pinned-by: "UNPINNED: not driven"
```

## Bounds — what this reconciliation did not do

- **No spawn.** No assistant session, no sub-agent, no adapter launch: this unit has none. The fan-out cell is one process reading from a provisioned worktree. **The genuine concurrent-spawn half is the orchestrator's and was not run.**
- **No test was run and nothing was built.** Every statement about a test file is a statement about its text.
- **The source's trace was not re-derived line by line.** `fn run_list`, `fn run_list_staged`, the three enumerators and the log writer were read in full; the pack-load path, the cascade, `served_from_home_note`'s callees and the subprocess list are the source's reading, checked only by what the drives show.
- **`decisions-pending.md` → (D) and the fix pass's ledger rows** that both reports cite for RC-5 and RC-6 were not re-read.
- **Not driven:** `JIGC_PACK_DIR`, a project pack list, a project-layer schema shadow; concurrency against a writer; the debug-only route fences (the candidate is a release binary).
- **The driver's row F3** (eight sequential appends) was not driven as a row of its own.

## Where the evidence is

Per root `<W>/<root>`: `<W>/<root>.runs/<label>.{stdout,stderr,porc.before,porc.after,man.before,man.after}`.
The tools: `<W>/tools/{env.sh,snap.py,drive.sh,mkfresh.sh,mkrig.sh,rows12.sh,groupCDG.sh,groupH.sh,groupF.sh,groupO.sh,groupT.sh,groupL.sh,groupAB.sh,order.sh,groupE.sh,groupM.sh}`.
The previous-release runs' logs: `<W>/{rows12,groupCDG,groupH,groupF,groupO,groupT,groupL,groupAB,groupM}-prev.log`.
`<W>` is `<scratch>/row-doc-list-reconciler.nbC73u`. Nothing was committed, and the one file written
for the repository is this report.

<!-- end of report -->
