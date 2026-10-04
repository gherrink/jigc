# Row 9 · adopter docs & help — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. This row is **numbered axis 8
(adopter docs & help), scoped** to the sentences M54, M55 and the trailer wrote or moved. Baseline:
**rc.16**.

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **`CLAUDECODE` is set in your session.** Every commit jigc makes in a rig therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it. On this row that is
  **a sentence to check, not noise**: where a guide, a step or a help text says what a commit jigc
  makes looks like, drive it with the variable set *and* cleared and grade the sentence against both.
- **Keys:** a baseline row keeps its `(axis, id)` key verbatim; a new finding is keyed `(R9, <id>)`.
- Under `.jigc/` use `command grep` with a before-control.

## READ FIRST

- **The baseline:** `completions/artifacts/M52/per-axis-review/README.md` — the *Axis 8 · adopter
  docs & help* comparison table (5 M51 rows, all CLOSED), §A's `(8, N-1)` and `(8, N-2)` — and
  `completions/artifacts/M52/per-axis-review/axis-8.md`. Axis 8 recorded **zero** open leads. No M53
  partial run re-drove it.
- The contract: `design/surface-contract.md` → *The three laws* (law 1 is this row's predicate: a
  surface says what the binary does); `design/assistant-adapter.md` → *The adapter's owned artifacts*
  (the installed guide: its stamp, its refuse-to-clobber, *One batch, one hash move*);
  `implementation/release.md` → *Installing* and → *What the package carries*;
  `design/findings-channel.md` §2 (why `report-inconsistency` is visible) and §8 (the crate README);
  `completions/artifacts/M51/acceptance-design.md` Part 2, the axis-8 row.
- What changed: `completions/artifacts/M54/VERDICT.md` (scenario 7, the README-link bound),
  `completions/artifacts/M55/VERDICT.md` (scenario 22; *Not run* → the packaged-bytes comparison),
  `completions/artifacts/M55/crates-page-reread.md`, DECISIONS.md → *M54 settled* S13 and S21.

## DERIVE THE DOOR SET FROM — and state the count you read

The door set is a **doc-sentence batch**, derived, not a registry: every sentence the range wrote or
moved that claims what the binary does, and the verb it names.

| source of sentences | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| the two guides | `crates/cli/guides/QUICKSTART.md`, `crates/cli/guides/MIGRATING.md` | embedded by `include_str!` (`crates/cli/src/setup.rs`) and installed as `.claude/skills/jigc/SKILL.md`; QUICKSTART owns the install line |
| the crate README | `crates/cli/README.md` | generated from `README.md` by `dev/crate-readme`; M55's crates.io re-read counted **6** README links |
| the install line's homes | `crates/cli/guides/QUICKSTART.md` (owner), `README.md` (the one allowed copy), `crates/cli/README.md` (derived) | 3 |
| the generated help texts | `task finalize --help` (`whats_left_coverage`, the two commit clauses), `validate --help` (the probe-family registry), `migrate-corpus --help` (`SchemaChangeKind::ALL`), `doc show --help` (`WHOLE_DOC_KEYS`) | 4 generated texts — **re-derive the list**: grep the long-help builders in `crates/cli/src/doc.rs`, `task.rs`, `milestone.rs` |
| `COMMITTING_DOORS`' `commits` clauses, each rendered or quoted in its door's `--help` | `crates/cli/src/invocation_log.rs` | 11 rows over 9 verbs |
| `VERB_KINDS` | `crates/cli/src/cli.rs` | 48 leaves; `jigc <leaf> --help` exists for each |
| the router catalog | `jigc start` (bare), `jigc start "<intent>"`, `jigc describe` | the visible set follows each workflow's `selectable` (explicit or defaulted) and `suppressed` front-matter — re-derive it from both packs' 39 workflows; `report-inconsistency` is the one member the range added, and five methodology workflows state `selectable: true` explicitly (`decided-task`, `do-research`, `form-vision`, `park-idea`, `report-inconsistency`) |

## CELL SET

The numbered axis's three cells — **the sentence's claim driven at the verb** · **the generated help
equals the registry that generates it** · **the guide sentence is true after install** — over this
batch:

1. **The installed guide** — `jigc setup` on a `bare` rig, then read
   `.claude/skills/jigc/SKILL.md` line by line: every sentence about *install* (one line; one binary;
   the prerequisites; where cargo puts it; the upgrade being the same line), about *setup* (what it
   writes, its one commit, the hook), about *start / finalize*, and MIGRATING's about adoption,
   upgrade and the orphaned `doc-code` — each driven at the verb it names, in that same rig. In-repo
   links are unlinked in the installed copy: none dangles.
2. **The install line, byte-for-byte** in its three homes, and equal to what `jigc`'s own surfaces
   print wherever one prints it (grep the help and the findings' routes for `cargo install`).
3. **The crate README** — `dev/crate-readme --stdout` equals the committed `crates/cli/README.md`
   (a read-only run); every rewritten link's target **exists in this repository at the path the URL
   names**; no relative link survives; the install line and every command in it run as written in a
   rig (or are named as not runnable there, with the reason).
4. **`report-inconsistency` in the router** — it is in bare `jigc start`'s catalog and in the
   router's composed text with its `when:` hint; an intent that matches the hint, routed by
   following the printed re-run line, mints the task and composes the workflow; the one-liner says
   what the workflow then does (one new doc, landed alone, other staged paths left staged) — drive
   the workflow to its commit and compare. The three hidden workflows appear in neither.
5. **Help text, the range's verbs** — `--help` of `setup`, `uninstall`, `validate`, `unmanage`,
   `doc create`, `doc author`, `doc show`, `doc list`, `task finalize`, `task validate`, `workflow`,
   `milestone execute`, `milestone finalize`: every sentence stating a count, a default, a gate
   subject, an exit code, the commit the door makes or its scope, a refusal code, or *never* — driven
   at the verb. Top-level `jigc --help` one-liners for the same verbs.
6. **Generated help == its registry** — the four generated texts above against the registry each is
   generated from (read the registry by symbol; compare to the emitted bytes).
7. **The commit sentences** — each `COMMITTING_DOORS` verb's help states the commit it lands; and
   wherever a guide, a step (`step:author-commit`) or a help says something about authorship or
   trailers, what the landed commit actually carries with `CLAUDECODE` set and cleared.

## BASELINE ROWS TO RE-DRIVE

Keys quoted from `completions/artifacts/M52/per-axis-review/README.md`:

| key | tier there | what it said on rc.16 | note |
|---|---|---|---|
| `(8, N-1)` | 3 | `jigc uninstall --help` contradicts itself about its own guard count, and the `--force` clause is the half that lies | re-drive: read the help, count the guards each clause names against the guards the door has — CLOSED (argv) or STILL-OPEN(1.x, expected) |
| `(8, N-2)` | 3 | every retitle-without-reslug at `jigc rename` acks a no-op and commits one, while the binary knows the title it just moved | re-drive |
| M51 `CX-1` · `CX-2` · `CX-3` · `D-1` · `D-2` | — | all CLOSED on rc.16 | **`CX-3` must be re-driven** — it was closed by scoping the guide's install command to *a clone of the jigc repository*, and M54 rewrote that story to crates.io; `CX-2` (the guide's refuse-to-clobber sentence) and `D-2` (every committing door's help states its commit — the door set is 11 rows now) where cells 1 and 7 pass through them; `CX-1`, `D-1` only if a cell reaches them |

There is **no tier-1 row** and **no open lead** on numbered axis 8.

## RIG STATES

- **`bare`** — the installed-guide cell: `jigc setup` is the act under test.
- **`fresh`** — the router, the workflow, the help-claim drives.
- **`committed-singletons`** — `(8, N-2)` (a committed doc to retitle) and help claims about
  populated stores.
- **No rig** for cells 2, 3 and 6: they read this repository's files and the binary's `--help`.
  `dev/crate-readme --stdout` writes nothing.

## ENVIRONMENT NOTES

- **Links: existence in the tree is drivable; HTTP is a different question.** A rewritten README
  link's *target path* exists or does not at this commit — check that. Whether the URL answers 200
  on github.com is already on the record for rc.23 (`completions/artifacts/M55/crates-page-reread.md`)
  and the README did not change in the rc.23→rc.24 range (verify with `git diff` over the two tags);
  if you re-fetch, record it as a network observation, not a product row. The **rc.24 crates.io page
  itself has not been re-read** — say so; do not infer it from rc.23's.
- **The packaged README bytes** (M55's *Not run*): `cargo package -p jigc --list` shows the file is
  listed; comparing the packaged *bytes* needs `cargo package` to build the tarball — drivable with a
  private `CARGO_TARGET_DIR` under your scratch root, never with `--allow-dirty`, never `publish`.
  Row 1 owns the package; coordinate by not duplicating — cite its row if it drove it, drive it only
  if it did not.
- The install line itself is **row 1's** to run; here it is a sentence to compare and a guide claim
  to check, not a command to execute on this host.
- A help text is read from the installed binary (`jigc <leaf> --help`), never from the source — the
  source is the reconciler's and the Codex pass's.
