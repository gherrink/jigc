# Row 10 · co-author trailer — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. Source of the row: the
2026-10-03 decision *Commits jigc makes carry the coding agent's co-author trailer*. **A new brief
— no baseline; first drive.** This code landed after M55's completion audit and after every prior
per-axis review, so neither covered it; with one methodology slot-hint reword it is the whole of the
rc.23→rc.24 product diff.

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **Keys:** a new finding is keyed `(R10, <id>)`.
- Under `.jigc/` use `command grep` with a before-control.

## THE VARIABLE IS THE ROW — read this before driving anything

- **You run inside a Claude Code agent session, where `CLAUDECODE` is set.** Left alone, *every*
  commit jigc makes for you is the **agent** arm. The **human** arm exists only when a cell removes
  the variable. So every cell is driven as an explicit triple, never by inheritance:
  - **set, non-empty** — `CLAUDECODE=1 jigc …` (set it explicitly; do not rely on the ambient value);
  - **unset** — `env -u CLAUDECODE jigc …`;
  - **set, empty** — `CLAUDECODE= jigc …`.
- **The rig construction itself commits** (`jigc setup`, and the finalizes a corpus state is built
  from) **under your ambient variable.** A rig is therefore born with agent-signed commits in its
  history. For any cell whose question is *what does `HEAD` carry before the act* — every amend cell —
  build the predecessor commit yourself with the variable in the state the cell names, and verify it
  (`git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)'`) **before** the act. For the
  install commit, construct the `bare` rig and run `jigc setup` yourself under each of the three
  states; do not read the trailer off a rig's own setup commit and call it a cell.
- **Never print the variable's value, or any other environment value.** Print only that it is set,
  unset or empty (`[ -n "${CLAUDECODE+x}" ]`, `[ -z "$CLAUDECODE" ]`).
- **Count with git's parser, not with a substring:**
  `git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)' <sha>` gives one line per trailer
  git reads; `git log -1 --format=%B <sha>` gives the raw message for placement. A row asserts both.

## READ FIRST

- **Baseline: none — first drive.**
- The contract: `design/assistant-adapter.md` → *The co-author trailer* (six bullets and a *Declared
  bounds* paragraph — each sentence is a cell or a bound); `design/finalize.md` → *6. Commit*, → *The
  amend arm*, → *Commit-doc rendering* (the paragraph opening *One trailer the commit doc does not
  author*); DECISIONS.md → the 2026-10-03 entry *Commits jigc makes carry the coding agent's
  co-author trailer*; `design/surface-contract.md` → *The three laws* (law 1: a human's hand-run
  finalize carrying an agent's name would be a false statement).
- The shipped profile: `crates/cli/adapters/claude-code.yaml` (the `co-author:` block);
  the step that still invites an agent to record a co-author by hand:
  `crates/cli/packs/dev/steps/author-commit.yaml`.

## DERIVE THE DOOR SET FROM — and state the count you read

| registry | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | **11 rows over 9 verbs**: `task finalize` · `task finalize (amend)` · `milestone finalize (squash: true)` · `milestone finalize (squash: false)` · `rename` · `migrate-corpus` · `milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone discard` · `task discard` |
| the one exclusion | `crates/cli/src/setup.rs` → `commit_install` | `jigc setup`'s install commit (`--no-verify`), which signs its own message |
| the seam | `crates/cli/src/task.rs` → `git_commit_capture` / `commit_through_seam` | every row above funnels through it; the doc-only commit reaches it through `crates/cli/src/milestone.rs` → `git_commit_paths` |
| the profile key | `crates/cli/src/adapter.rs` → `CoAuthor` | 3 fields: `name`, `email`, `when-env`; unknown fields refused |
| the co-author basis | `crates/cli/src/task.rs` → `CoAuthorBasis` | 2: the session · the session **or** `HEAD` (the amend arm only) |

**Doors: 12** — the eleven rows and the install commit — **plus two named commit shapes that are
cells of their own:** the **doc-only path-scoped commit** (a `task finalize` of a report/triage
task) and **each per-sub-task commit of the `squash: false` chain** (one `milestone finalize` lands
several commits; every one is read).

## CELL SET

1. **The door sweep** — all 12 doors × {set · unset · empty}: every commit the door lands carries
   the trailer **exactly once** under *set*, and **none** under *unset* and *empty*. Read every
   commit landed since the pre-act `HEAD`, not only the tip.
2. **Amend** — `jigc task amend` → `jigc task finalize` over a `HEAD` that {carries the profile's
   trailer · does not} × the amending session {agent · human}: four cells. Record the trailer count
   before and after, and that the tree and parent are unchanged.
3. **Dedupe by address** — the commit doc already carries, through `jigc doc add-item
   commit:<id>#trailers --title Co-Authored-By` + `set-field …/value`:
   the identical value · the **same address under a different display name** · the same address with
   the key in another case · a **different** co-author (both must survive) · the address only in the
   *body prose*, outside the trailer block. One trailer per identity, by git's count.
4. **Message shapes** — subject only · subject + body · body ending in an existing trailer block
   (a `Refs:` trailer) · a body whose last paragraph mixes prose and a trailer-shaped line: where the
   trailer lands (`%B`), and that git reads it (`%(trailers)`).
5. **The profile, through `JIGC_ADAPTERS_DIR`** — a copy of the shipped profile in a `mktemp -d` with
   the `co-author:` key **absent** · `name` empty or carrying `<` · `email` carrying whitespace ·
   `when-env` not an identifier · an **unknown key** inside the block · the directory set but the
   file unreadable — at `jigc setup` (what does the install say, and under which code?) and at a
   committing door (does the commit land, and with what?). And `when-env` naming **another**
   variable: the trailer follows that variable, not `CLAUDECODE`.
6. **Fan-out, both squash modes** — a milestone with ≥ 2 sub-tasks, one code-carrying and one
   docs-only, under `finalize.fan-out.squash` = `true` (one combine commit) and `false` (the
   per-sub-task chain plus its aggregate), each × {set · unset}: every landed commit read. And the
   declared bound, as a measured row: the boundary run by a *human* over work sub-agents staged lands
   no trailer.
7. **The doc-only path-scoped commit** — a report task beside a code task with staged files: the
   trailer on the doc-only commit × {set · unset}, and the path scope unchanged by the signing (the
   staged index byte-identical before and after — row 5 owns the scope; here it is the control that
   signing did not disturb it).
8. **The record-only doors** — `milestone create`, `add-task`, `add-from-spec`, `discard`, sub-task
   `task discard`: their messages are jigc-authored, not from a commit doc — same triple.
9. **Rejection, then re-run** — a rejecting `pre-commit` hook under *set*: no commit; `HEAD`
   unchanged; then remove the hook and re-run the same door: exactly one trailer. Then the mixed
   sequence: rejected under *set*, re-run under *unset*, and the reverse — record what each landed
   commit carries.
10. **Pinned keys untouched** — `--format json` of `task finalize` (`committed.subject`, `subject`)
    and of the other doors' landed envelopes: byte-equal between *set* and *unset* apart from the sha;
    the Conventional-Commits header (`git log -1 --format=%s`) identical.
11. **The hook stream** — a `commit-msg` hook that echoes the message file it is handed: the message
    git gives the hook already carries the trailer (or not) — record which; and `chatty-hooks`' output
    relay unchanged.
12. **Run through cargo** — see the environment note; this cell has a fence form and an
    equivalent-environment drive, and is recorded as exactly that.

## BASELINE ROWS TO RE-DRIVE

None — nothing predates this code. Two neighbours, so they are not re-filed here:
`(2, DEFECT C)` (a posture breach at the commit seam prints no state-truth clause; tier 2,
STILL-OPEN(1.x) on rc.20) lives on the same seam and is **not** this row's; and the amend arm's own
contract (F-10) was reviewed on rc.20 — this row asks only what the trailer does on it.

## RIG STATES

- **`bare`** — the install commit: run `jigc setup` yourself, three times in three rigs, one per
  variable state.
- **`fresh`** — `task finalize` (ordinary, amend, doc-only), `milestone *`, `task discard`.
- **`committed-singletons`** — `rename` (a committed doc to re-slug) and the amend cells over a
  `HEAD` you built.
- **`migrate-corpus`'s commit** needs a corpus below its schema-version: read
  `crates/cli/tests/support/committing_doors.rs` for how the shared fixture brings **each** of the
  eleven doors to its commit, and reproduce that construction with the binary
  (`--pack-from-dev --schema … --repin` is the rig's way to a reshaped, still-frozen pack).
- **`chatty-hooks`** — cell 11's relay half.
- **Non-rig fixtures, named at their cells:** the profile copies under `mktemp -d` for
  `JIGC_ADAPTERS_DIR` (copy `crates/cli/adapters/claude-code.yaml`, edit the copy — never the repo's
  file); rejecting and echoing hooks behind `core.hooksPath` in a `mktemp -d`.

## ENVIRONMENT NOTES

- **Run through cargo (cell 12).** `.cargo/config.toml` in this repository forces `CLAUDECODE` to the
  empty string for every process **cargo** runs, so the test suite behaves the same inside an agent
  session as on a CI runner. The binary under review is the installed one and `cargo run` is
  forbidden, so this cell has **no drive on the installed binary through cargo**. Record it as two
  rows and say which is which:
  - a **fence row** (confers no coverage): `cargo test -p jigc --test g_finalize agent_co_author::`
    run from the repository root — it builds and runs the *debug* target, under cargo's forced-empty
    variable, and the suite sets the variable per child. Run it only if no other build holds
    `target/`; record pass/fail and the test count, nothing more.
  - the **equivalent-environment drive**: the *set, empty* arm of cell 1 on the installed binary is
    the state cargo's `[env]` produces — cite it as the driven half.
  Do not write, copy or edit a `.cargo/config.toml` anywhere to simulate it.
- **`JIGC_ADAPTERS_DIR` must be unset for every cell that is not cell 5** — check it once at the top
  (set or unset only, never the value).
- **A human typing into an agent's session** inherits its environment and is signed as the agent —
  a declared bound (*not driven* in the design's own words). It stays not driven: you cannot be a
  human. Say so.
- The trailer's text is the product's own string, `Co-Authored-By: Claude <noreply@anthropic.com>`;
  it is the only real address that may appear in your record. For the *different co-author* cells
  use a synthetic name at the reserved `example.com` domain, as the repository's own unit tests do.
- Commits you make with plain `git commit` to build a predecessor carry no trailer unless you write
  one — that is how the *HEAD does not carry it* cells are built, and how a hand-written trailer in
  `HEAD` (a human-typed one, different display name) is built for the dedupe-on-amend corner.
