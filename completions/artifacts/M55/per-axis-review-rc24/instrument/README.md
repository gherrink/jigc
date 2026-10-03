# The per-axis review instrument — the rc.24 partial re-review's copy

This is the M51 → M53 instrument (`completions/artifacts/M53/per-axis-review-rc20/instrument/`,
and before it M52's and M51's) re-pointed for the **partial per-axis re-review on `jigc 1.0.0-rc.24`** that M54's S18 and M55's S16
owe before the 1.0.0 call (`design/findings-channel.md` §12; DECISIONS.md → *the road to the 1.0.0
call*). It holds **the briefs only**: ten Codex source-pass prompts (`axisN-prompt.md`) and ten
driver scopes (`axisN-driver-scope.md`). It reviews nothing, drove nothing for findings, and edits
nothing in the repository. Every path below is repo-relative.

**Read by symbol at** `aa6666cb` (the merge of the rc.24 release PR; the tag `jigc-v1.0.0-rc.24` peels
to `91834b5e`, the release-prepare commit that merge brings in), 2026-10-03. The installed `~/.local/bin/jigc` printed
`jigc 1.0.0-rc.24` when this was written.

**"Partial" means scoped.** Each row covers what M54, M55 and the co-author trailer changed inside
its subject — not the whole historical axis. A row with a baseline additionally re-drives that
baseline's tier-1 rows and its STILL-OPEN rows that fall inside the row's subject.

## Read this first: `axisN` is ROW N, not numbered axis N

The files are named `axis1` … `axis10` because the workflow script's shape expects that, but **N is
the row number of the approved table below, not the historical axis number.** Four of them collide
with a numbered axis of a different subject:

| file | this run's row | numbered axis of the same N (M51 → M53) |
|---|---|---|
| `axis4-*` | pack-load / manifest freeze · migration (= numbered axis **7**, scoped) | axis 4 was transaction / rollback |
| `axis5-*` | finalize / transaction (= numbered axis **4**, scoped) | axis 5 was pinned contracts |
| `axis7-*` | pinned read contracts (= numbered axis **5**, scoped) | axis 7 was freeze & migration |
| `axis8-*` | composed surfaces (= numbered axis **6**, scoped) | axis 8 was adopter docs & help |

Every baseline key in every record is `(numbered axis, id)` — `(7, A7-F3)`, `(5, DEFECT 4)`. **So
that nothing collides, the scopes tell each driver: a baseline row keeps its key verbatim, and a new
finding of this run is keyed `(R<row>, <id>)`** — `(R4, …)`, never `(4, …)`. Each prompt's fifth line
opens `ROW N · <name> (numbered axis M …)` for the same reason. This convention is the instrument
author's, not the human's — see *Not pinned down*, item 4.

## The ten rows

Approved by the human; none added, dropped or merged.

| # | Row | Scope on rc.24 | Basis | Baseline |
|---|---|---|---|---|
| 1 | setup · the hook · install · release | M54's setup, the pre-commit hook, install (QUICKSTART's line, the single binary), the release pipeline and packaged crate contents | NEW | none — first drive |
| 2 | probe integrity · measurement / the invocation log | the `doc-code` probe inside `jigc` (self-exec), the probe override, measurement and the invocation log incl. its committing-doors table | NEW | none — first drive |
| 3 | store exit codes / reconciliation | M54's store exit flips; M55's L1, L2 and the `unmanage` route; carries open gap **F21** as STILL-OPEN(expected) | NEW | none of its own; **three rows + one lead of numbered axis 7 (rc.16) carried** |
| 4 | pack-load / manifest freeze · migration | M54's pack move behind `pack_builtin.rs`; M55's two new methodology doctypes at v1, the `cheap-vs-robust` hint | numbered axis 7 | rc.16 — `completions/artifacts/M52/per-axis-review/` |
| 5 | finalize / transaction | M55's doc-only step: path-scoped commit, its left-out narration, the carryover gate's exemption | numbered axis 4 | rc.16 — `completions/artifacts/M52/per-axis-review/` |
| 6 | write surface | the `new: true` create-gate entry at both create doors, the `title-ignored` route | NEW (nearest: axis 1) | none — first drive |
| 7 | pinned read contracts | `title` + `fields` keys, the default projection | numbered axis 5 | rc.20 — `completions/artifacts/M53/per-axis-review-rc20/` |
| 8 | composed surfaces | the S2 fix; the two report and two triage workflows, hidden and visible; carries M55's declared bound (`jigc task validate <sub>` exits 3 on the omitted commit doc under `squash=true`) | numbered axis 6 | rc.19 — `completions/artifacts/M53/per-axis-review-rc19/` |
| 9 | adopter docs & help | the generated crate README, QUICKSTART/MIGRATING in the crate, `report-inconsistency` in the router, help text | numbered axis 8 | rc.16 — `completions/artifacts/M52/per-axis-review/` |
| 10 | co-author trailer | every member of `COMMITTING_DOORS` plus setup's install commit × the cell set the human listed | NEW | none — first drive |

## Door-set registries and the counts read

So a driver's count can be compared. Every count was read off the symbol at the commit above; a
count marked *(grep)* has **no** code-side registry and is a census of string literals or files.

| # | the registry the door set is derived from | file | count read |
|---|---|---|---|
| 1 | `install_tracked_paths` (the install commit's pathspec) | `crates/cli/src/setup.rs` | 7 unconditional + 3 conditional members |
| 1 | `install_path_dispositions` | `crates/cli/src/setup.rs` | 2 dispositions; 2 exempt-when-jigc-owned members |
| 1 | `BEHALF_DOORS` → `setup` | `crates/cli/src/cli.rs` | 48 rows; `setup` commits-on-behalf, 1 posture exemption |
| 1 | `IGNORE_DOORS` | `crates/cli/src/gitignore.rs` | 4 |
| 1 | `setup.*` / `uninstall.*` codes *(grep)* | `crates/cli/src/*.rs` | 20 / 13 distinct literals |
| 1 | `include` + `readme` · `PUBLISHED_DIRS` | `crates/cli/Cargo.toml` · `crates/cli/tests/package_contents.rs` | 6 patterns + README · 4 |
| 2 | production callers of `doc_code_invoker` *(derivation — no registry of probe doors)* | `crates/cli/src/cli.rs`, `task.rs`, `milestone.rs` | 3 call sites → 6 doors |
| 2 | `ProbeArgv` | `crates/cli/src/invoke.rs` | 3 variants |
| 2 | `COMMITTING_DOORS` · `ERROR_CODE_REGISTRY` | `crates/cli/src/invocation_log.rs` | 11 rows over 9 verbs · 12 |
| 2 | the log record's key set | `crates/cli/src/invocation_log.rs` | 8 keys |
| 3 | `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs` | 7 |
| 3 | `STORE_FAMILIES` | `crates/engine/src/validate.rs` | 7 |
| 3 | `EXIT_CODES` · `ENVELOPE_OWED_CODES` | `crates/cli/src/task.rs` · `crates/cli/src/render.rs` | 5 · 4 |
| 4 | the embed seam | `crates/cli/src/pack_builtin.rs` | 2 packs, 1 module |
| 4 | manifest entries | `crates/cli/packs/{dev,methodology}/config/schema-manifest.yaml` | 6 · 13 |
| 4 | `SchemaChangeKind::ALL` × `LOCI` | `crates/engine/src/schema_diff.rs` | 18 × 3 = 54 |
| 4 | the numbered axis's door set | `completions/artifacts/M51/acceptance-design.md` Part 2 | 8 leaves (+ `relocate` here) |
| 5 | `COMMITTING_DOORS` ∪ `setup` ∪ `milestone join` ∪ `milestone provision` | `crates/cli/src/invocation_log.rs` | 11 rows + 3 |
| 5 | `CommitModel` · `ManifestKind::ALL` · `GATE_COVERAGE` | `crates/cli/src/render.rs` · `crates/cli/src/gate_coverage.rs` | 3 · 7 · 13 |
| 5 | `ROLLBACK_POPULATIONS` | `crates/cli/src/rollback.rs` | 11 |
| 5 | workflows composing `step:finalize-doc-only` *(grep)* | `crates/cli/packs/methodology/workflows/` | 4 |
| 6 | `VERB_KINDS`, `doc` × `Write` | `crates/cli/src/cli.rs` | 8 leaves; 2 create doors |
| 6 | `allows-create` entries *(front-matter parse)* | `crates/cli/packs/{dev,methodology}/workflows/*.yaml` | 34 entries over 29 of 39 workflows; 2 carry `new: true` |
| 6 | `AllowsCreate` keys · `DOCTYPE_DOORS` · `SLUG_DOORS` · `MINT_DOORS` | `crates/engine/src/compose.rs` · `crates/cli/src/cli.rs` · `crates/engine/src/state.rs` | 3 · 16 · 6 · 6 |
| 6 | `create.*` codes *(grep)* | `crates/cli/src`, `crates/engine/src` | 6 distinct literals |
| 7 | `ENVELOPE_ARMS` | `crates/cli/src/render.rs` | 66 arms over 48 leaves (59 Pinned · 7 Unpinned); `doc show` 8 · `doc schema` 1 · `doc list` 1 |
| 7 | `DOC_READ_VERBS` · `WHOLE_DOC_KEYS` · `DocRow` | `crates/cli/src/doc.rs` | 3 · 7 (+ `staged`) · 6 keys |
| 7 | `doc schema` `contract-version` | `crates/cli/src/doc.rs` | 7 |
| 8 | callers of `render::composed` | `crates/cli/src/cli.rs`, `migrate.rs`, `milestone.rs`, `task.rs` | 8 call sites over 5 verbs |
| 8 | workflows · the `suppressed.door` set · steps *(files / grep)* | `crates/cli/packs/{dev,methodology}/` | 18 + 21 = 39 · 15 · 31 + 44 |
| 8 | steps carrying `{{ cli.finalize-task }}` directly *(grep)* | the two step trees | 4 (the derived omission set is wider) |
| 8 | `OFF_CATALOG_VERBS` | `crates/cli/src/orient.rs` | 2 |
| 9 | the doc-sentence batch *(derivation)* | the two guides, the two READMEs, the help of the range's verbs | 2 guides · 3 homes of the install line · 4 generated help texts · 48 `--help` leaves |
| 10 | `COMMITTING_DOORS` + the install commit | `crates/cli/src/invocation_log.rs` · `crates/cli/src/setup.rs` | 11 rows + 1 = 12 doors |
| 10 | `CoAuthor` fields · `CoAuthorBasis` | `crates/cli/src/adapter.rs` · `crates/cli/src/task.rs` | 3 · 2 |

**The registries the rc.20 record tabulated, re-read here:** `VERB_KINDS` / `BEHALF_DOORS` 48 / 48 ·
`ENVELOPE_ARMS` 66 · `COMMITTING_DOORS` 11 · `ERROR_CODE_REGISTRY` 12 · `TASK_AREA_FILES` 15 ·
`MINT_DOORS` 6 · `AMBUSH_CONTRACTS` 6 · `STORE_EXIT_FLIPS` 7 · `WORK_UNIT_ID_DOORS` 25 ·
`SLUG_DOORS` 6 · `DOCTYPE_DOORS` 16 · `PATH_ARG_OCCURRENCES` 14 · `ROLLBACK_POPULATIONS` 11 ·
`DESTROYING_DOORS` 6 — **all unmoved since rc.20.** What the range moved: `ManifestKind::ALL` 6 → 7
(`left-staged`) · `CommitModel` 2 → 3 (`DocOnly`) · the methodology manifest 11 → 13 · shipped
workflows 35 → 39 · the `suppressed.door` set 14 (rc.19) → 15.

## Baseline rows each row re-drives

Keys only; each scope file quotes the row's one-line statement from its record.

| # | baseline record | tier-1 rows | STILL-OPEN rows inside the subject | leads re-dispositioned | listed as NOT re-driven (outside the subject) |
|---|---|---|---|---|---|
| 1 | — | — | — | — | — |
| 2 | — | — | — | — | — |
| 3 | `completions/artifacts/M52/per-axis-review/README.md` (axis 7) | none on axis 7 | `(7, A7-F1)` · `(7, A7-F2)` · `(7, C-2)` (declared residual) | the `probe-unreliable` member of §D's axis-7 row; and **F21** (`completions/artifacts/M55/VERDICT.md`) as STILL-OPEN(expected) | — |
| 4 | `completions/artifacts/M52/per-axis-review/README.md` (axis 7) | none on axis 7 | `(7, A7-F3)` | §D's axis-7 row, its freeze/migration members (the methodology manifest · the un-driven `SchemaChangeKind × LOCI` cells · `cwd-unreadable` / `pack-resource-missing` · CX-8 · CX-9 · CX-13) | `(7, A7-F1)` · `(7, A7-F2)` · `(7, C-2)` → row 3 |
| 5 | `completions/artifacts/M52/per-axis-review/README.md` (axis 4) | none on axis 4 | `(4, DEFECT 1)` · `(4, DEFECT 2)` · `(4, DEFECT 3)` (= the open half of M51 `(4, C4)`) | §D's four axis-4 leads | — |
| 6 | — | — | — | — | — |
| 7 | `completions/artifacts/M53/per-axis-review-rc20/README.md` (axis 5) | **`(5, DEFECT 1)`** | `(5, DEFECT 4)` · rc.17 `DEFECT A` · `(5, DEFECT 3)` (its `doc show` half) | `store.no-such-leaf` (M52 §D axis-7 row) | `(5, DEFECT 2)` · `(5, C1)` · `(5, D1)` · `(5, DEFECT 1 · rc.20)` = `(2, A2-2)` · `(5, DEFECT 2 · rc.20)` · `(5, C-13)` · `(5, C-18)` |
| 8 | `completions/artifacts/M53/per-axis-review-rc19/README.md` (axis 6) | none on axis 6 | `(6, D-1)` · `(6, D-2)` · `(6, D-3)` · `(6, D-4)`; plus `(2, N-1)` = `(6, A6-R1)` to carry rc.20's closure onto this axis | `(6, L-1)` … `(6, L-7)`; and M55's **declared bound** as HOLDS-AS-DECLARED / differs | `(2, A2-2)` named so it is not conflated |
| 9 | `completions/artifacts/M52/per-axis-review/README.md` (axis 8) | none on axis 8 | `(8, N-1)` · `(8, N-2)`; and M51 `CX-3` re-driven because M54 rewrote the sentence that closed it | none (axis 8 recorded zero) | — |
| 10 | — | — | — | — | `(2, DEFECT C)` named as a neighbour on the same seam |

**The only tier-1 baseline row inside any of the ten rows' baselines is `(5, DEFECT 1)`.** The other
three tier-1 rows the M52 record carries — `(3, A3-1)`, `(3, A3-2)`, `(2, DEFECT A)` — and rc.17's
`(2, DEFECT 1)` sit on numbered axes 2 and 3, which are **not among the ten rows**. They were last
driven CLOSED on rc.20. **This instrument does not re-drive them, and nothing in it says they hold on
rc.24.**

## The tier predicate — verbatim

From `completions/artifacts/M53/per-axis-review-rc20/README.md` → *FINDINGS* → *A · CONFIRMED*:

```text
The predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying or
moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the binary
does not do.
```

Its home, which that sentence quotes: `implementation/decisions-pending.md` → *The rc.16 wave (M52)*
(the three tier headings) and → *The rc.17 fix pass (M53)* → *The exit rule* (which uses the scale).

## The row schema — verbatim

From `completions/artifacts/M53/per-axis-review-rc20/README.md`:

```text
**An axis matrix row** is `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted,
verdict}`. A row is **driven** iff its argv ran on that binary. **A verb is covered iff it is the door of
≥1 driven row.** A classification-only row — a leaf proven by a ⇔ fence rather than by driving — confers
**no** coverage.
```

Its original, `completions/artifacts/M51/acceptance-design.md` → Part 2 (the binary named there is
rc.15; read *the installed `1.0.0-rc.24`*):

```text
**Row schema, common to all eight:** `(door, cell) → {argv driven, exit, code|none, route kind, surface
asserted, verdict}`. A row is **driven** iff its argv ran on the installed `1.0.0-rc.15` and its verdict was
recorded with a repro block. **A verb is covered iff it is the door of ≥1 driven row.** Classification-only
rows (a leaf classified *neither*, a registry row proven by a ⇔ fence rather than by driving) are **not**
driven rows and confer **no** coverage — otherwise the fence measures a table instead of the binary.
```

## The reconciliation rule — verbatim

From `completions/artifacts/M53/per-axis-review-rc20/README.md` → *The reconciliation rule*:

```text
> *A claim by one that the other cannot reproduce is a **lead, not a finding**.*

A Codex claim with no driven repro enters the table as `lead(codex, <claim>)` and is either **driven to a
repro block** — at which point it is a finding — or **recorded REFUTED with its falsifying datum**. An
Opus row the source pass says cannot happen **stays a finding** (it was driven), and the source claim is
recorded refuted with the datum. **Silence is not refutation.** A row a reconciler cannot drive at all
stays an **OPEN LEAD** with the reason.
```

Its original, `completions/artifacts/M51/acceptance-design.md` → *The reconciliation rule*:

```text
**The reconciliation rule.** *A claim by one that the other cannot reproduce is a **lead, not a finding**.*
A Codex claim with no driven repro enters the axis table as `lead(codex, <claim>)` and is either **driven to
a repro block** — at which point it is a finding — or **recorded refuted with its falsifying datum**. An
Opus row that the source pass says cannot happen stays a finding (it was driven), and the source claim is
recorded refuted with the repro that refutes it. **No fix ships on a source read alone, and no completeness
claim ships on driving alone** — the standing rule from M46's planning corrections (*an agent's report is a
lead, not a measurement*) and M50's two planning halts (*a claim about how the composed product behaves is
not established by reading the files it is composed from*).
```

## The files, and what each is

| file | what it is |
|---|---|
| `axisN-prompt.md` (N = 1 … 10) | the **Codex source-pass prompt** for row N — exactly five lines: (1) the per-run lead-in, (2) blank, (3) the standing source-pass brief, **byte-identical** to line 3 of all eight `completions/artifacts/M52/per-axis-review/instrument/axisN-prompt.md` (copied by a script, never retyped; the ten copies and the M52 original hash the same), (4) blank, (5) the row paragraph: door set, cell set, Read list, Hunt list. **Unseeded:** a Hunt list names classes of defect; none asserts a defect exists. Run as `codex exec -s read-only -o <out> - < axisN-prompt.md`. |
| `axisN-driver-scope.md` (N = 1 … 10) | what the **Opus driver** for row N is told beyond the standing driver brief: READ FIRST · the registries to derive the door set from, with the count the instrument's author read · the cell set · the baseline rows to re-drive, by key · the useful `dev/jigc-rig` states · the row's environment notes. |
| `README.md` | this file. |

**Both recorded lessons of the M52 instrument are kept.** (1) No prompt tells Codex to write a file:
every lead-in carries *do NOT write any file … your ENTIRE report must be your final message*. (2) The
reconcile phase must wait for its Codex file (`until [ -s … ]`) — that guard lives in the workflow
script, which this directory does **not** contain.

**What the orchestrator re-points in the workflow script**
(`completions/artifacts/M53/per-axis-review-rc20/instrument/per-axis-review.workflow.js`): the scratch
root and the binary path; the expected `--version` (`jigc 1.0.0-rc.24`); `AXES` → the ten rows; the
driver prompt's READ FIRST / PROBE HARDEST lines → the row's `axisN-driver-scope.md`; the rig-state
list (it now has `refs-post-hoc`, and `bare` is the only state `setup` can be probed from); the
record directory; and the coverage table — **on a scoped run, union coverage of all 48 `VERB_KINDS`
leaves is not a goal**: the assembler should report which leaves the ten rows reached and compare
each baseline-bearing row against its own baseline's coverage restricted to the row's subject, not
demand the 48.

## Standing notes for every driver (repeated at the head of each scope file)

- **The binary** is the installed registry build `~/.local/bin/jigc`; it must report
  `jigc 1.0.0-rc.24`. Release posture: the debug-only fences do not exist in it, so a fence violation
  shows as a bad emitted command, never a panic. Never `target/debug/jigc`, never `cargo run`.
- **`CLAUDECODE` is set in a driver's session**, so every commit jigc makes in a rig carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it (`env -u CLAUDECODE`, or
  `CLAUDECODE=`). Row 10 drives both arms explicitly, and is told that a rig is *born* with
  agent-signed commits. Every other row is told why a trailer appears and to hold the variable
  constant across compared cells. Rows 5 and 9 have one cell each where it is a datum.
- **Docker.** The orchestrator recorded Docker as not running. **A probe made while this was written
  answered** (`docker version` returned a server version). Row 1's scope therefore tells the driver to
  probe once and either drive the `dev/runner-faithful` cells or mark them NOT DRIVEN (Docker not
  running) — never to guess. See *Not pinned down*, item 1.
- **`cargo publish` is dry-run only**, and the bare install line is never run on the host (it would
  replace the binary nine other drivers are using) — row 1's scope gives the private-root form.
- **Rigs:** two-step eval, stdout only, `--binary ~/.local/bin/jigc` always (the rig's default is the
  debug binary), no teardown, never `rm -rf` a variable path. Under `.jigc/`, `command grep` with a
  before-control.

## Not pinned down — discrepancies, unknowns and calls the instrument's author made

1. **Docker's state contradicts the brief.** Told *not running*; observed answering at authoring
   time. Written as a probe-then-decide instruction, not as either assumption.
2. **"M54's store exit flips" has no code home as a change to the table.** `STORE_EXIT_FLIPS` carries
   the same seven ids it carried on rc.20, and the only commit in the M54 → rc.24 range that touches
   the table is M55's L2 narrowing of `oob-rename` (`edaa428c`). What M54 changed on that surface is a
   **producer**: a probe that cannot start, or runs at another build, now reaches
   `pack-probe-integrity.probe-failure` and through it the `probe-unreliable` member. Row 3 is written
   against the registry as it is (all seven members, `oob-rename` and `probe-unreliable` mandatory);
   row 2 owns why the probe did not run. If the human meant something else by the phrase, it is not
   in the code under that name.
3. **S16's parentheticals were withdrawn inside their own Settle.**
   `completions/artifacts/M55/settle-log.md`'s S16 line reads *create --new*, *foreign-staged* and
   *contract-version bump*; R1 replaced the refusal with a path-scoped commit (there is no
   foreign-staged finding), R2 moved create-only onto the gate entry (there is no `--new` flag), and
   the design-draft resolutions struck the bump (`doc schema`'s `contract-version` is still 7).
   DECISIONS.md's own S16 line carries no parentheticals. Rows 5, 6 and 7 are written against
   `design/findings-channel.md` §12 and the code, and every prompt's lead-in says so.
4. **The `(R<row>, <id>)` key convention and the `ROW N` labelling are this instrument's**, added
   because `axisN` now means two things (above). If the orchestrator prefers another scheme, it is one
   sentence per scope file and one phrase per prompt's fifth line.
5. **Numbered axis 7's baseline is split across three rows**, by subject, so that no open row is
   dropped: the store-exit rows `(7, A7-F1)`, `(7, A7-F2)`, `(7, C-2)` and the `probe-unreliable` lead
   → row 3; `(7, A7-F3)` and the freeze/migration leads → row 4; the `store.no-such-leaf` lead → row
   7. Row 3 was approved as a NEW brief with *"reuse what fits"*; this is the reading of that.
6. **How "tier-1 rows and STILL-OPEN rows that fall inside the row's subject" was read:** every
   tier-1 row of a row's baseline axis is re-driven regardless of subject (there is exactly one,
   `(5, DEFECT 1)`); STILL-OPEN rows are filtered by subject, and the ones filtered **out** are listed
   in the scope file as *not re-driven* so their absence is not read as closure (row 7 lists seven).
   For rows 5, 8 and 9 every open row of the baseline axis was taken as inside the subject — which
   puts `(4, DEFECT 2)` (`config set`'s rollback) and `(4, DEFECT 3)` (`milestone create`'s) in row 5
   although neither is on `task finalize`. Three cheap drives; drop them if the narrower reading was
   meant.
7. **The rc.23 → rc.24 range carries one change the ten rows do not name:** a second
   `planning-record` slot-hint reword (`claim-driven`, commit `069d4b67`), which also moved the twelve
   `planning` compose goldens. It is the same class as the `cheap-vs-robust` hint (presentation text
   the schema-hash erases), so it is placed in row 4 beside it. It is not a new row.
8. **Row 10's "run-through-cargo" cell has no drive on the installed binary.** `.cargo/config.toml`'s
   forced-empty `CLAUDECODE` applies to processes cargo runs; the binary under review is not run by
   cargo and `cargo run` is forbidden. The scope records it as a **fence row**
   (`cargo test -p jigc --test g_finalize agent_co_author::`, debug target, no coverage) plus the
   *set, empty* arm on the installed binary as the equivalent-environment drive. If a different
   construction was meant, it is not derivable from the code.
9. **Project-layer shadows need a stated exception to the standing rule.** The standing driver brief
   says *build each fixture with the binary, never by writing into `.jigc/`*. No verb mints a schema or
   workflow whole-file shadow (`jigc config fork` forks a step only), and rows 4, 5, 6 and 8 each have
   cells that need one. Their scopes grant one exception — a hand-written file under `.jigc/config/`,
   the tracked project layer (`design/overrides.md` → *Authored metadata on a definition resolves by
   whole-file shadow*), committed with plain git and named at the cell. The gitignored workbench stays
   off limits.
10. **There is no code-side registry of probe doors.** Row 2's door set is a derivation: the three
    production callers of `doc_code_invoker`, mapped to six doors by the header of
    `crates/cli/tests/probe_failure_doors.rs` (which says *all five* and lists six — five paths, the
    task gate serving two leaves). Stated as a derivation in the scope.
11. **Three code families were counted by grep, not by registry** — `setup.*` (20), `uninstall.*`
    (13), `create.*` (6). A literal built by `format!` or held in a `const` under another spelling
    would be missed; each driver states its own count.
12. **Cells no driver on this host can reach, written as NOT DRIVEN (reason) in the scopes rather
    than left to be guessed:** the release workflow, a publish, a tag push, a merge (agents may not —
    `implementation/release.md`); the Linux replaced-binary `/proc/self/exe` arm (row 2); the
    absent-manifest-entry arm of the freeze, which needs an embedded-pack rebuild (row 4); any
    comparison against an older binary, i.e. *the existing corpus is read unchanged across the range*
    (row 4 — none is installed and none may be built); a case-sensitive filesystem (row 6); a human
    typing into an agent's session (row 10, the design's own *not driven*); and the **rc.24 crates.io
    page**, which nobody has re-read — rc.23's was (row 9).
13. **The baselines were read at README depth.** Every baseline key and one-line statement here is
    quoted from the three review READMEs; the per-axis files (`axis-N.md`, 75 – 150 KB each) and the
    Codex passes were not read in full. Each scope sends its driver to the axis file for the repro
    blocks and fixtures, and nothing here restates a repro.
14. **Numbered axes 1, 2 and 3 are not among the ten rows.** Their rows stand where the last run that
    drove them left them (axis 1: rc.16; axes 2 and 3: rc.20), and three of the four historical
    tier-1 rows live there. This is the approved scope, stated so the record does not read as though
    they were asked.
15. **Line 3's word budget against line 1's ask.** The standing brief (byte-identical by instruction)
    ends *under 1500 words*, while each lead-in asks for baseline dispositions, completeness claims and
    leads in full. The rc.19 and rc.20 prompts carried the same tension and their passes resolved it by
    exceeding the budget; left as it is, because line 3 may not be edited.
16. **No workflow script is written here.** The task was the briefs; the script is the orchestrator's
    to re-point (list above).
