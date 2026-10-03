<!-- Reconciled ROW 9 file (adopter docs & help), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis9` in the body means ROW 9 of this run, not numbered axis 9. -->

# rc.24 partial per-axis re-review — ROW 9 · adopter docs & help — RECONCILED (driver table + Codex source pass; the source pass is present, exit 0)

This file is the reconciliation of two independent passes over the published `jigc 1.0.0-rc.24`:
the Opus **driver table** (reproduced below, unchanged except for demotions) and the Codex
**source pass** (source-only at tag `jigc-v1.0.0-rc.24`; every claim of it is entered as a lead in
R.3 and driven). The reconciler authored neither and built no code. Rule applied:
`completions/artifacts/M51/acceptance-design.md` → *The reconciliation rule* — a claim by one that
the other cannot reproduce is a lead, not a finding.

**Binary asserted before anything else:** `~/.local/bin/jigc --version` → `jigc 1.0.0-rc.24`, exit 0.
Every reconciler drive ran that binary through `dev/jigc-rig <state> --binary ~/.local/bin/jigc`
(two-step eval, stdout only, `[ -n "$REPO" ]` guarded, no teardown). Rigs used by the reconciler:
`fresh` ×4, `bare` ×2, `committed-singletons` ×1. `CLAUDECODE` is **set** in the reconciler's
session; a cell that clears it says `env -u CLAUDECODE`. No environment value is recorded.

**How to read it.** Part I is the driver's file. A verdict cell ending in **DEMOTED (reconciler)**
is a row the driver marked driven that carries no repro block — such a row is not driven on the
driver's evidence (R.2). Part II is the reconciliation ledger: R.1 the door-set count against the
registries · R.2 the demotions · R.3 every Codex claim · R.4 every driver defect, re-driven, with
its tier · R.5 the reconciler's repro blocks · R.6 the baseline rows · R.7 open leads · R.8 doors
covered.

**Result in one paragraph.** Six new findings are confirmed on rc.24, plus the two baseline rows
still open: **one proposed tier 1** — `(R9, F5)`, `jigc uninstall` without `--force` destroys
sole-copy files inside `.jigc/logs/`, `.jigc/state/` and `.jigc/index/` at exit 0, the opt-in
invocation log among them — and **five tier 3** (`(R9, F1)`–`(R9, F4)` from the driver, `(R9, C-1)`
from the Codex pass). Both baseline rows `(8, N-1)` and `(8, N-2)` are STILL-OPEN; the Codex pass's
*CLOSED* for `(8, N-1)` is refuted by a drive. One further Codex statement is refuted in part (the
guides carry no false dry-run claim — F1 and F4 are two). Coverage after reconciliation: **37 of 48**
`VERB_KINDS` leaves.

---

# PART I — the driver table (unchanged except demotions)

# rc.24 partial per-axis re-review — ROW 9 · adopter docs & help — DRIVER TABLE

Numbered axis 8 (adopter docs & help), scoped to the sentences M54, M55 and the co-author trailer
wrote or moved, plus the baseline rows the scope names. Baseline: rc.16
(`completions/artifacts/M52/per-axis-review/axis-8.md`).

**Binary asserted first, before anything else:**

```
$ ~/.local/bin/jigc --version
jigc 1.0.0-rc.24          # exit 0
```

Release posture. Every drive below ran this binary (`$JIGC` = `~/.local/bin/jigc`, passed to the rig
with `--binary`); one cell (1.13) ran a `cmp`-identical copy of it from a `mktemp -d` directory, and
says so. Nothing ran `target/debug/jigc` or `cargo run`.

**Environment held constant:** `CLAUDECODE` is **set** in this session. Every cell that clears it says
so (`env -u CLAUDECODE`). No environment value is recorded anywhere below.

**Rigs** (all from `dev/jigc-rig <state> --binary ~/.local/bin/jigc`, two-step eval, stdout only,
`[ -n "$REPO" ]` guarded): `bare` ×4, `fresh` ×5, `committed-singletons` ×1. No teardown. No file was
written into the gitignored workbench except the *plants* the cells themselves name (a scratch note
beside a task's staged docs; the files of finding F5) — those are the fixtures under test, not corpus
construction.

---

## 0 · The door set, derived — the counts I read beside the instrument author's

| registry / source | file | instrument's count | **my count** | datum |
|---|---|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs` | 48 leaves | **48** (13 top-level · 11 `doc` · 7 `task` · 8 `config` · 9 `milestone`) | agrees; `jigc <leaf> --help` ran for all 48, each exit 0 |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | 11 rows over 9 verbs | **11 rows over 9 verbs** | agrees. The doc-only commit (M55) is **not** a row: it is an arm of `jigc task finalize` with no row and no clause of its own |
| generated long-help builders | `crates/cli/src/{cli,doc,task,milestone}.rs` | "4 generated texts — re-derive the list" | **14** `*_long_about()` builders: `cli.rs` 2 (`migrate-corpus`, `validate`) · `doc.rs` 6 (`set-slot`, `set-field`, `add-item`, `create`, `rename`, `show`) · `milestone.rs` 3 (`create`, `add-task`, `add-from-spec`) · `task.rs` 3 (`finalize`, `amend`, `discard`) | **differs**: the four the instrument names are the four whose registry is a named code-side set; ten more texts are built by a function. rc.16's O-4 said 13; `task amend` is the fourteenth |
| `STORE_FAMILIES` | `crates/engine/src/validate.rs` | — | **7** | |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs` | 18 | **18** | agrees |
| `WHOLE_DOC_KEYS` (+ `STAGED_KEY`) | `crates/cli/src/doc.rs` | 7 (+ `staged`) | **7 + 1** | agrees |
| `CommitModel` | `crates/cli/src/render.rs` | 3 | **3** (`Index`, `Amend`, `DocOnly`) | agrees — and `task finalize --help` names **two** (finding F1) |
| shipped workflows | `crates/cli/packs/{dev,methodology}/workflows/` | 39 | **39** (18 + 21) | agrees |
| the router's visible set | front-matter: `creates-task: true` ∧ `selectable ≠ false` ∧ no `suppressed:` | — | **13** | equals the 13 members bare `jigc start` printed |
| explicit `selectable: true` | methodology pack | 5 | **5** (`decided-task`, `do-research`, `form-vision`, `park-idea`, `report-inconsistency`) | agrees |
| `suppressed:` blocks carrying `door:` | both packs | 15 | **15** (`amend`, 12 `migrate-*`, `milestone-execution`, `sub-task`) | agrees |
| homes of the install line | `QUICKSTART.md` (owner), `README.md`, `crates/cli/README.md` | 3 | **3**, plus a 4th copy on disk after install (`.claude/skills/jigc/SKILL.md`) and one embedded in the binary | |
| crate README links | `crates/cli/README.md` | 6 | **7 occurrences over 6 distinct targets** (`QUICKSTART.md` is linked twice) | M55's re-read counted targets |
| the two guides | `crates/cli/guides/` | 2 | **2**; installed as one 409-line file | |

**The doc-sentence batch is a derivation, not a registry** (as on rc.16). I derived it from
`git diff 8914f471^..HEAD` over the two guides (the M54 → rc.24 range), the commit that added the
trailer (`28ca86b4`), and the help of the thirteen verbs the scope lists. What the range wrote or
moved in the guides: the whole **Install** section; the removal of the probe bullet from §1 and the
*"one of the things above is not in that commit"* sentence; MIGRATING's **"One file an older install
leaves behind"** paragraph; MIGRATING gate 1's `left-staged` clause; the *"eleven committing doors"*
count. `28ca86b4` touched **no** guide, **no** help text and **no** pack step.

**Cells:** **C1** the sentence's claim driven at the verb · **C2** the generated help equals the
registry that generates it · **C3** the guide sentence is true after install.

---

## 1 · The `(door, cell)` table

Columns: # · door · cell · argv driven · exit · code\|none · route kind · surface asserted · verdict.
Route kind: M = Mechanical, H = Human, I = Informational, — = none.

### Cell 1 — the installed guide, line by line (`.claude/skills/jigc/SKILL.md` after `jigc setup`)

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1.1 | `setup` | C3 | `jigc setup` (bare rig) | 0 | none | — | writes `.jigc/AGENT.md`; `CLAUDE.md` gets a bare `@.jigc/AGENT.md` import (no marker fence); `Bash(jigc:*)` allowlisted; `SessionStart` → `jigc start`; `SKILL.md` opens with `jigc-version: 1.0.0-rc.24` + `jigc-body-blake3:`; one commit `chore(jigc): install jigc workspace config` of 8 files; hook at `.git/hooks/pre-commit` not in it | matches |
| 1.2 | `setup` | C3 | `jigc setup` again | 0 | none | — | five footprint files byte-identical, `HEAD` unmoved, ack carries no `install commit` line | matches |
| 1.3 | `setup` | C3 | append a line to `SKILL.md`; `jigc setup` | 0 | `adapter-guide.user-modified` | H | file byte-identical after; `HEAD` unmoved; `--format json` `guide_file: null`, `install_commit: null` | matches (= CX-2) |
| 1.4 | `upgrade` | C3 | `jigc upgrade` (same state) | 0 | `adapter-guide.user-modified` | H | reports the same state, replaces nothing | matches |
| 1.5 | `setup` | C3 | untracked `CLAUDE.md`; `env -u CLAUDECODE jigc setup` (bare) | 1 | `setup.dirty-install-path` | H | names the path; nothing installed (`.jigc/` absent, no hook), `HEAD` unmoved, file bytes intact | matches |
| 1.6 | `setup` | C3 | `env -u CLAUDECODE jigc setup --force` | 0 | `setup.forced-install-path` | H | install commit lands; advisory names `CLAUDE.md`; the user's line kept above the import | matches |
| 1.7 | `setup` | C1 | `jigc setup --force` on a clean footprint | 0 | none | — | no commit, `HEAD` unmoved (*"Inert when the footprint is clean"*) | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-B** |
| 1.8 | `setup` | C3 | `git config core.hooksPath .githooks`; `jigc setup` (bare) | 0 | none | — | install commit carries `.githooks/pre-commit` (9 files); the hook body names this machine's `jigc` path | matches |
| 1.9 | `validate` (through the installed hook) | C3 | break a cited symbol; plain `git commit` | 0 | none | — | stderr `jigc: doc<->code drift detected … (commit not blocked).`; commit lands | matches |
| 1.10 | `validate` (through the installed hook) | C3 | `git mv` a managed doc, both paths staged; plain `git commit` | 1 | none | — | `out-of-band managed-doc rename staged in this commit … use jigc rename instead (commit blocked).` | matches |
| 1.11 | (top level) | C3 | `jigc --version` | 0 | none | — | `jigc 1.0.0-rc.24` | matches |
| 1.12 | `task finalize` · `validate` | C3 | code-anchor finalize + `jigc validate`, no `doc-code` file beside the binary and none written by `setup` | 0 / 0 | none | — | *"work out of the box, with no second file"*; `doc-code.symbol-exists` fires once the symbol is removed | matches — **PARTLY DEMOTED (reconciler): the `doc-code.symbol-exists` arm carry no repro block. RD-G2, RD-S** |
| 1.13 | `setup` · `task finalize` · `validate` · `uninstall` | C3 | a sentinel `doc-code` (writes a marker when run) beside a `cmp`-identical copy of the binary; the four verbs run from that copy | 0 ×4 | none | — | marker never written; sentinel bytes unchanged and still present (*"nothing reads that file any more, and nothing removes it either"*) | matches |
| 1.14 | `start` | C3 | `jigc start "add a rate limiter"` | 0 | none | — | menu + `jigc start --workflow <chosen> "<intent>"`; `.jigc/tasks/` empty | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G1** |
| 1.15 | `start` | C3 | `jigc start --workflow single-task "add a rate limiter"` | 0 | none | — | `task minted: add-a-rate-limiter`; `.jigc/tasks/add-a-rate-limiter/` | matches |
| 1.16 | `start` | C3 | `jigc start` | 0 | none | — | orientation; nothing minted, tree unchanged | matches |
| 1.17 | `config get` · `config set` · `start` · `doc list` · `validate` | C3 | `config get invocation-log` → `false (pack-default)`; `config set invocation-log true`; three reads | 0 | none | — | one record per run appended to `.jigc/logs/invocations.jsonl` (3 lines after 3 reads); `git check-ignore` names `.jigc/.gitignore:6:logs/` | matches (CX-1's exception, reached) — **PARTLY DEMOTED (reconciler): `config get`, the three-reads count and `git check-ignore` carry no repro block. RD-F3** |
| 1.18 | `doc list` · `doc show` · `doc schema` | C3 | `doc list --task <id>`; `doc show adr:<slug> --task <id>`; `doc schema adr` | 0 | none | — | staged rows by address; the staged body; the schema with every write address | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G2** |
| 1.19 | `doc set-field` · `doc schema` | C3 | `doc set-field adr:token-bucket-limiter#status/cites-code --value src/limiter.rs#limit` | 0 | none | — | `doc schema adr` prints `code-anchor <repo-relative-path>[#<symbol>]` | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G2** |
| 1.20 | `task validate` | C3 | pre-task staged `server.conf`; `jigc task validate <id>` (single-task) | 3 | `finalize.carried-staged` | M | route ``git -C $REPO restore --staged -- server.conf`` or `--carry-staged` | matches |
| 1.21 | `task finalize` | C3 | `jigc task finalize <id> --dry-run` (same state) | 3 | `finalize.carried-staged` | M | findings instead of a manifest | matches |
| 1.22 | `task finalize` | C3 | `… --dry-run --carry-staged` | 0 | none | — | manifest rows `carried-over server.conf` · `added src/limiter.rs` · `promoted docs/decisions/…`; left-out `scratch.txt` | matches |
| 1.23 | `task finalize` | C3 | `… --carry-staged --format json`, a scratch note planted beside the staged docs | 0 | none | — | `committed.manifest[].kind` ∈ {`promoted`,`carried-over`,`added`}; `left_out[].kind = untracked`; `committed.displaced[{from,to}]` + the stderr note; the ADR at `docs/decisions/` | matches |
| 1.24 | `task diff` | C3 | `jigc task diff <id>` | 0 | none | — | code diff vs base + the staged docs | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G2** |
| 1.25 | `task amend` · `task finalize` | C3 | `jigc task amend "…"`; author `type`+`summary`; `finalize --dry-run`; `finalize` | 0 | none | — | mint prints `amending: <sha> "<subject>"`; `amended <old> → <new>`; tree hash unchanged | matches (the guide paragraph; the composed step's trailer sentence is **F2**) |
| 1.26 | `task discard` | C3 | `jigc task discard throwaway` ×2, then `--force` | 1 · 1 · 0 | `task-discard.staged-prose` · `task-discard.foreign-bytes` · none | H | each refusal names its population; `--force` narrates the foreign path; `HEAD` unmoved | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G3** |
| 1.27 | `task discard` | C3 | `jigc task discard first-sub-task` (a milestone sub-task; a bystander path staged) | 0 | none | — | `record commit: <sha>`; `HEAD` moves by one path-scoped commit; `A mine.txt` still staged | matches |
| 1.28 | `task finalize` | C3 | a rejecting `commit-msg` hook; `jigc task finalize <doc-only task>`; `invocation-log` on | 1 | `finalize.commit-rejected` (log `error_code`) | M | hook stderr verbatim; *"task … is intact … staged docs are still in `.jigc/tasks/<id>/docs/` …"*; re-run line; `HEAD` unmoved | matches |
| 1.29 | `config set` | C3 | `jigc config set docs-root documentation`; then `""` | 0 | none | — | staged `git mv` of both ADRs; `""` acks as `.`; `doc list` follows | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G3** |
| 1.30 | `task finalize` · `task validate` | C3 | the guides' carryover sentences on a **doc-only** task (pre-task staged `server.conf`) | 0 | none | — | QUICKSTART *"A change staged before the task existed refuses to ride the commit — one blocking `finalize.carried-staged` per carried path — unless you declare it with `--carry-staged`"*; MIGRATING gate 4 | **DEFECT — F1** |
| 1.31 | `task finalize` | C3 | `… --dry-run --format json` (doc-only) | 0 | none | — | `left_out: [{kind: left-staged, path: server.conf}, {kind: untracked, path: scratch.txt}]` (MIGRATING gate 1's two left-out tags) | matches |
| 1.32 | `ingest` · `migrate` · `task finalize` | C3 | foreign `docs/decisions/use-postgresql.md` committed; `jigc ingest`; `jigc migrate <path> --as adr`; author; `finalize`; `finalize --approve` | 0 · 0 · 4 · 0 | `conformance.section-missing` (ingest) · none | H | task id carries a 12-hex suffix; plain finalize renders the fidelity diff, **exit 4**, `HEAD` unmoved; `--approve` lands `M docs/decisions/use-postgresql.md` | matches — **PARTLY DEMOTED (reconciler): the `jigc ingest` arm carry no repro block. RD-F4** |
| 1.33 | `task finalize` | C3 | `jigc task finalize <migration task> --dry-run` vs the landed commit | 0 | none | — | MIGRATING gate 1 *"It prints the manifest the finalize would commit"* | **DEFECT — F4** |
| 1.34 | `migrate-corpus` | C3 | `jigc migrate-corpus --dry-run`; `jigc migrate-corpus --format json` | 0 | none | — | keys `migrated`, `already_current`, `blocked`, `unadopted`, `unfilled`, `commit`, `hook_output`, `dry_run`; tree unchanged | matches (the no-op arm only — see ND-4) — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G1** |
| 1.35 | `upgrade` | C3 | `jigc upgrade` on a clean install | 0 | none | — | reports, `git status` empty after | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G1** |
| 1.36 | `validate` | C3 | `jigc validate` over a committed doc↔code break | 0 | `doc-code.symbol-exists` | H | *"report-only at store scope (exit 0); these gate at …"* | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G2** |
| 1.37 | `setup` | C3 | read the installed `SKILL.md` | 0 | none | — | 0 markdown links, 0 URLs, 0 occurrences of `crates/cli` or `# cwd:`; the install block is the one crates.io line | matches (= CX-3's sentence) |
| 1.38 | `uninstall` | C3 | bytes parked under `.jigc/displaced/`; `jigc uninstall` | 1 | `uninstall.foreign-bytes` | H | names the parked file | matches |

### Cell 2 — the install line, byte for byte

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 2.1 | `setup` | C3 | `jigc setup`; hash the `^cargo install` line of the installed `SKILL.md` | 0 | none | — | `cargo install jigc --version '^1.0.0-rc.1' --locked` — same SHA-256 as the three repository homes | matches |
| 2.2 | *(repository read — no coverage)* | — | hash the line in `README.md`, `crates/cli/README.md`, `crates/cli/guides/QUICKSTART.md`; grep all 49 dumped help texts; `strings` on the binary | — | — | — | three homes byte-identical; **no** help text and no route string prints `cargo install` (the binary embeds it once, inside the guide) | matches |

### Cell 3 — the crate README

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 3.1 | *(repository tool — no coverage)* | — | `dev/crate-readme --stdout` · `cmp` against `crates/cli/README.md` | 0 · 0 | — | — | generator output equals the committed file; working tree unchanged | matches |
| 3.2 | *(repository read — no coverage)* | — | every `](…)` in `crates/cli/README.md`; `git cat-file -e jigc-v1.0.0-rc.24:<path>` per target; `git diff --stat jigc-v1.0.0-rc.23 jigc-v1.0.0-rc.24 -- README.md crates/cli/README.md crates/cli/guides/` | 0 | — | — | 7 links, all `https://github.com/gherrink/jigc/blob/HEAD/…`; 0 relative; all 6 targets exist at the tag; the range diff is empty | matches |
| 3.3 | `setup` · `start` · `task finalize` | C1 | the README's own claims: *"`jigc setup` installs them, concatenated, as the agent's guide (`.claude/skills/jigc/SKILL.md`)"*, and the loop `jigc setup` → `jigc start "<intent>"` → `jigc task finalize <id>` | 0 | none | — | rows 1.1, 1.14, 1.15, 1.23 | matches |

### Cell 4 — `report-inconsistency` in the router

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 4.1 | `start` | C1 | `jigc start` (fresh) | 0 | none | — | 13 catalog rows = the derived visible set; `report-inconsistency — code and a doc, or two docs, disagree and the disagreement is worth a record until it is reconciled`; `report-jigc-feedback`, `triage-inconsistency`, `triage-jigc-feedback` absent | matches |
| 4.2 | `start` | C1 | `jigc start "the README says the port default is 8080 but the code says 9090"` | 0 | none | — | the same 13 rows with the same hint; the re-run line; the three hidden workflows absent; nothing minted | matches |
| 4.3 | `start` | C1 | `jigc start --workflow report-inconsistency "<the same intent>"` | 0 | none | — | `task minted: readme-says-the-port`; composes the workflow; `create-gates: inconsistency` | matches |
| 4.4 | `doc create` · `doc set-field` · `doc add-item` · `doc set-slot` · `task finalize` | C1 | the composed steps, verbatim, with `server.conf` staged before the mint and `scratch.txt` untracked; `jigc task finalize <id>` | 0 | none | — | one commit, **one file** (`docs/inconsistencies/port-default-disagrees.md`); after: `A server.conf` still staged, `?? scratch.txt` | matches |
| 4.5 | `describe` | C1 | `jigc describe --workflows`; `jigc describe` | 0 | none | — | `report-inconsistency` carries no hidden clause; the three others each say *"It is hidden from the router catalog: invoked by name …"* with the reason | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G1** |
| 4.6 | `workflow` | C1 | `jigc workflow report-jigc-feedback --preview` | 0 | none | — | composes, `no task minted` (hidden, reachable by name) | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G1** |
| 4.7 | `doc create` · `doc author` | C1 | a second report task; `doc create inconsistency --title "Port mismatch"` and `doc author inconsistency` over an id already committed | 1 · 1 | `create.already-exists` | M / H | nothing staged (`doc list --task` shows only the commit doc); `--slug` mints beside | matches |

### Cell 5 — help text, the range's verbs

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 5.1 | all 48 leaves + top level | C1 | `jigc <leaf> --help` ×48; `jigc --help` | 0 ×49 | none | — | a help text exists for every `VERB_KINDS` leaf | matches (confers **no** door coverage — clap answers before dispatch) — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-H** |
| 5.2 | `setup` | C1 | `--help` sentences → rows 1.1, 1.2, 1.5–1.7 | — | — | — | *"Idempotent"*; *"Refuses its own install commit when …"*; `--force` *"Inert when the footprint is clean"* | matches (see O-1 for what it omits) |
| 5.3a | `uninstall` | C1 | `jigc uninstall` (fresh, knob off) twice | 0 · 0 | none | — | removes `.jigc/`, unwires `CLAUDE.md`, drops the permit; second run `(nothing to remove — no repo-local jigc install was present)`; `uninstall.untracked-workbench-file` fires over an untracked `.jigc/config/notes.txt` | matches |
| 5.3b | `uninstall` | C1 | the same, with `invocation-log` on | 0 · 0 | none | — | *"removes `.jigc/`"* · *"Idempotent: a second run is a clean no-op"* | **DEFECT — F3** |
| 5.3c | `uninstall` | C1 | files planted inside `.jigc/logs/`, `.jigc/state/`, `.jigc/index/`; `jigc uninstall` (no `--force`) | 0 | none | — | *"Four states it refuses instead of destroying, because `.jigc/` is their only copy — … and any other file under `.jigc/` that no index has a copy of … blocks with `uninstall.untracked-workbench-file`"* | **DEFECT — F5** |
| 5.4 | `validate` | C1 | `jigc validate` on clean, drifted and un-baselined stores | 0 | varies | — | task-less; repairs nothing; `git status` unchanged after; exit 0 over findings | matches — **PARTLY DEMOTED (reconciler): the drifted and un-baselined arms carry no repro block. RD-G2 (drifted); the un-baselined arm stays undriven** |
| 5.5 | `unmanage` | C1 | `jigc unmanage docs/decisions-log.md` ×2 | 0 · 0 | none | — | first drops the baseline and edges, bytes unchanged; second `no-op: … is not managed (nothing to drop)`; `HEAD` unmoved | matches |
| 5.6 | `doc create` | C1 | titles `The v1.1 cache!` · `Use in-memory store for sessions today` · `In-memory cache`; same-id / other-id / malformed-slug / committed-id re-creates | 0 ×3 · 1 ×4 · 0 | `write.title-ignored` · `write.identity-change` · none (slug grammar) · `write.title-ignored` | M | ids `v1-1-cache`, `use-in-memory-store`, `in-memory-cache`; staged-doc title → route at `jigc doc rename`; committed-doc title → *distinct `--title` or `--slug`* | matches |
| 5.7 | `doc author` | C1 | the same payload twice in one task; under `new: true` over a committed id | 0 · 1 · 1 | `write.already-present` · `create.already-exists` | M / H | whole payload rejected, nothing staged | matches (the `already existed — copied in for update` ack: ND-9) — **PARTLY DEMOTED (reconciler): the `write.already-present` arm carry no repro block. RD-G3** |
| 5.8 | `doc show` | C1 | committed `--format json`; staged `--task --format json`; a staged-only address without `--task`; `#sides`; `#sides/<id>/says` | 0 · 0 · 1 · 0 · 0 | `store.not-found` | M | 7 keys; staged adds `staged: <task id>`; refusal routes at `--task <task-id>`; item array; leaf string | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G2 + RD-D** |
| 5.9 | `doc list` | C1 | task-less with an open task; `--task`; `--format json` | 0 | none | — | stderr note hands over `jigc doc list --task <id>`, stdout unchanged; rows `{id, path, state, item-count, title, fields}`; a transient doc lists at `commit:<task-id>` | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-G2** |
| 5.10 | `task finalize` | C1 | `--dry-run` over a clean tree / an unstaged change; `--approve` on a non-migration task | 3 · 3 · 0 | `finalize.empty-commit` · `finalize.nothing-staged` · none | H / M | refusal instead of manifest; `--approve` inert | matches |
| 5.10b | `task finalize` | C1 | `--carry-staged` and `--dry-run` on a doc-only task with a pre-task staged path | 0 | none | — | `--carry-staged`: *"land index entries staged before this task existed instead of refusing … Inert when nothing is carried, and inert on an **amend** task in every state"*; `--dry-run`: *"refuses … the carryover gate, where an undeclared carry-over is reported (exit 3) instead of the manifest"*; the about's *"second commit model"* | **DEFECT — F1** |
| 5.11 | `task validate` | C1 | clean → advisory only → blocking | 0 · 0 · 3 | — | — | *"exit non-zero iff any blocks"*; `--carry-staged` accepted | matches — **PARTLY DEMOTED (reconciler): the clean (no-finding) arm carry no repro block. not re-driven** |
| 5.12 | `workflow` | C1 | `--preview` on `single-task`, `amend`, `migrate-adr`, `router`; neither flag; both flags | 0 · 1 · 1 · 1 · 2 · 2 | `workflow.verb-routed` ×2 · none | M | no task minted; the refusal names `jigc task amend` / `jigc migrate <path> --as adr`; *"mints no task, so there is nothing to preview"*; clap usage | matches |
| 5.13 | `milestone execute` · `milestone list-tasks` | C1 | two sub-tasks; `jigc milestone execute ship-limiter`; unknown id | 0 · 1 | `milestone.unknown` | M | 2 `Spawn:` lines, id-sorted; `HEAD` unmoved | matches — **PARTLY DEMOTED (reconciler): `milestone list-tasks` and the unknown-id arm carry no repro block. RD-M** |
| 5.14 | `milestone finalize` · `milestone provision` · `milestone join` | C1 | one sub-task run in its worktree; `join`; `finalize` with a bystander path staged, then `--carry-staged`; unknown id | 0 · 0 · 3 · 0 · 1 | `finalize.carried-staged` · `milestone.unknown` | M | subject `Finalize milestone second-wave (1 sub-task)`; record + code in one commit; *"these stay staged: mine.txt"* | matches — **PARTLY DEMOTED (reconciler): the unknown-id arm carry no repro block. RD-M** |
| 5.15 | top level | C1 | `jigc --help` (the five of the thirteen verbs that are top-level: `setup`, `uninstall`, `validate`, `unmanage`, `workflow`); `jigc doc --help`, `jigc task --help`, `jigc milestone --help` for the other eight | 0 | none | — | each one-liner equals the first paragraph of its verb's own help (compared by script for the five; read for the eight) | matches (they inherit F3's and F5's sentences where they quote them: `uninstall`'s one-liner is its whole about) — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-H (the five top-level one-liners; the eight sub-verb one-liners stay undriven)** |

### Cell 6 — generated help == its registry

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 6.1 | `validate` | C2 | `jigc validate --help` | 0 | none | — | the seven `STORE_FAMILIES` `name — checks` pairs, in registry order, each found once | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-H** |
| 6.2 | `migrate-corpus` | C2 | `jigc migrate-corpus --help` | 0 | none | — | the 18 `SchemaChangeKind::ALL` wire names, in order, as one comma-joined run | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-H** |
| 6.3 | `doc show` | C2 | `jigc doc show --help`; `jigc doc show vision:vision --format json` | 0 | none | — | `type, slug, title, item-count, schema-version, fields, sections` + `staged`; the emitted object's key set is those seven | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-H + RD-D** |
| 6.4 | `task finalize` | C2 | `jigc task finalize --help`; the `what's-left:` line of `jigc start --workflow single-task …` | 0 | none | — | the coverage sentence is byte-identical on both surfaces; the amend clause equals `TASK_FINALIZE_AMEND_COMMITS` | matches — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-H** |

### Cell 7 — the commit sentences

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 7.1 | the 9 `COMMITTING_DOORS` verbs | C2 | `--help` of each; grep each row's `commits` clause | 0 | none | — | all 11 clauses present in their door's help | matches (= D-2) — **DEMOTED (reconciler): no repro block in §2 — NOT driven on the driver's evidence. Re-driven: RD-H** |
| 7.2 | `task finalize` ×3 arms · `milestone finalize` · `rename` · `milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone discard` · `task discard` | C1 | each driven to its commit (rows 1.23, 1.25, 4.4, 5.14, B.2, 5.13, and the repro in §2.7) | 0 | none | — | `HEAD` moves as the help says; the path-scoped doors leave a staged bystander staged; `add-from-spec` lands one commit per seed and names each sha | matches — nine of the eleven rows driven to their commit (`finalize.fan-out.squash` resolves `true (pack-default)`, so the squash arm is the one driven); `migrate-corpus` and `squash: false`: ND-4, ND-5 |
| 7.3 | `setup` · `task finalize` · `milestone add-task` | C1 | the same door with `CLAUDECODE` set and cleared | 0 | none | — | set → exactly one `Co-Authored-By: Claude <noreply@anthropic.com>`; cleared → none. No help or guide sentence states either | matches the design; see O-2 |
| 7.4 | `doc add-item` · `doc set-field` · `task finalize` | C1 | `step:author-commit`'s invitation followed: a `Co-Authored-By` item valued `Claude Opus <noreply@anthropic.com>`, variable set | 0 | none | — | the landed message carries **one** trailer, in the authored spelling | matches (de-duplication by address) |
| 7.5 | `task amend` · `task finalize` | C1 | the composed `amend` step's sentence, driven with **no** trailer item authored, over three arms | 0 | none | — | *"Trailers are re-authored too — the amended message carries only the trailer items you add here"* | **DEFECT — F2** |

### Baseline re-drives

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| B.1 | `uninstall` · `task list` | C1 | only the parked-bytes guard dirty; `jigc uninstall --force` | 0 | none | — | `--force`: *"Inert when all three guards are already clean"* | **STILL-OPEN — `(8, N-1)`** |
| B.2 | `rename` | C1 | `jigc rename vision:vision --to 'New Vision' --slug vision` | 0 | none | — | ack `renamed vision:vision -> vision:vision (VISION.md -> VISION.md)`; subject `rename VISION.md -> VISION.md` | **STILL-OPEN — `(8, N-2)`** |

**Rows driven: 76** (38 + 1 + 1 + 7 + 18 + 4 + 5 + 2). **Repository-read rows, no coverage: 3**
(2.2, 3.1, 3.2). **Not driven: 17** (§4).

> **Reconciler's count (see R.2).** Of the 76 rows the driver marks driven, **55 carry a repro block** in §2 and **21 carry none**; 8 of the 55 have an arm with no block. A row without a repro block is not driven: the 21 are demoted above, each marked in its verdict cell. All 21 were then re-driven by the reconciler on the same binary (blocks in R.5); 20 hold in full, and row 5.15 holds for five of its thirteen one-liners.

---

## 2 · Repro blocks

Paths are written `$REPO` (the rig's repository), `$RIG` (its root), `<tmp>`. Output is trimmed to
the asserted lines; `EXIT=` is the driven process's own status, captured unpiped.

### 2.1 · Cell 1 — setup, the installed guide, the hook (rows 1.1–1.13, 1.37, 2.1)

```
rig=$(dev/jigc-rig bare --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit

$ jigc setup                                         # CLAUDECODE set
  - bootstrap reference → CLAUDE.md …
  - jigc allowlist → .claude/settings.json …
  - SessionStart hook → .claude/settings.json …
  - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
    local to this checkout — git cannot track this path, so the hook is not in the install commit; …
  - jigc guides → .claude/skills/jigc/SKILL.md   (jigc's own quickstart + migration notes, stamped with this build)
  - install commit → 4c30ee7 …
EXIT=0
$ git log -1 --format='%s | %(trailers:only)'
chore(jigc): install jigc workspace config | Co-Authored-By: Claude <noreply@anthropic.com>
$ git show --stat --format= HEAD        → 8 files: .claude/settings.json, .claude/skills/jigc/SKILL.md,
                                          .jigc/{.gitignore,AGENT.md,config/.gitkeep,config/packs.yaml,version}, CLAUDE.md
$ cat CLAUDE.md                         → "## Project interface" / "@.jigc/AGENT.md"
$ head -6 .claude/skills/jigc/SKILL.md  → jigc-version: 1.0.0-rc.24 / jigc-body-blake3: 94afec4c…
$ grep -c '\](' SKILL.md → 0 ; grep -c 'https\?://' → 0 ; grep -c 'crates/cli\|# cwd:' → 0
$ grep '^cargo install' SKILL.md        → cargo install jigc --version '^1.0.0-rc.1' --locked
                                          (SHA-256 equal to the line in README.md, crates/cli/README.md, QUICKSTART.md)
$ ls ~/.local/bin | grep -c doc-code    → 0

$ jigc setup                            → EXIT=0 ; five footprint files byte-identical ; HEAD unmoved
$ printf '\nmy own note\n' >> .claude/skills/jigc/SKILL.md ; jigc setup
advisory · adapter-guide.user-modified — `.claude/skills/jigc/SKILL.md` no longer carries the bytes jigc wrote, so jigc left it untouched …
EXIT=0                                  → file hash unchanged ; HEAD unmoved ; `--format json`: guide_file null, install_commit null
$ jigc upgrade                          → the same advisory ; EXIT=0

# the hook
$ … land adr:use-a-router citing src/router.rs#route through record-decision … ; jigc validate → clean, EXIT=0
$ printf 'pub fn other() {}\n' > src/router.rs ; git add src/router.rs ; git commit -m "refactor: rename route"
jigc: doc<->code drift detected in committed docs — run `jigc validate` for details (commit not blocked).
EXIT=0
$ git mv docs/decisions/use-a-router.md docs/decisions/use-router.md ; git commit -m "chore: move adr"
jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).
EXIT=1

# dirty footprint (a second bare rig)
$ printf '# my project rules\n' > CLAUDE.md ; env -u CLAUDECODE jigc setup
blocking · setup.dirty-install-path — 1 path(s) in the install footprint carried changes that were in no commit …  `CLAUDE.md`
EXIT=1                                  → no .jigc/, no hook, HEAD unmoved, CLAUDE.md bytes intact
$ env -u CLAUDECODE jigc setup --force
advisory · setup.forced-install-path — `--force` consented over 1 install path(s) …  `CLAUDE.md`
EXIT=0 ; git log -1 --format='%s | [%(trailers:only)]' → chore(jigc): install jigc workspace config | []

# in-tree hooks (a third bare rig)
$ git config core.hooksPath .githooks ; jigc setup → EXIT=0 ; the install commit lists .githooks/pre-commit (9 files)

# the orphaned doc-code (a fourth bare rig; B=$(mktemp -d …); cp ~/.local/bin/jigc "$B/jigc"; cmp → identical)
$ printf '#!/bin/sh\necho ran >> "<tmp>/doc-code.ran"\nexit 7\n' > "$B/doc-code"; chmod +x "$B/doc-code"
$ "$B/jigc" setup ; … a code-anchor finalize … ; "$B/jigc" validate ; "$B/jigc" uninstall      → EXIT=0 each
$ ls "$B" → doc-code  jigc           # no doc-code.ran: never executed ; sentinel hash unchanged
```

### 2.2 · Cells 1 and 4 — the loop and `report-inconsistency` (rows 1.14–1.25, 1.30, 1.31, 4.1–4.4, 5.10b)

```
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit

$ jigc start                            → 13 workflows, among them
  - report-inconsistency — code and a doc, or two docs, disagree and the disagreement is worth a record until it is reconciled
$ jigc start "the README says the port default is 8080 but the code says 9090"
  … the same 13 rows … / jigc start --workflow <chosen> "<intent>"            EXIT=0 ; .jigc/tasks empty

$ printf 'port = 9090\n' > server.conf ; git add server.conf ; printf 'scratch\n' > scratch.txt      # BEFORE the mint
$ jigc start --workflow report-inconsistency "the README says the port default is 8080 but the code says 9090"
task minted: readme-says-the-port
… Land this task's docs as exactly one commit. This finalize commits path-scoped: … Anything else
staged in this checkout … stays staged and stays out of this commit, so the carryover gate does not
fire here and `--carry-staged` changes nothing. …
EXIT=0
$ jigc doc create inconsistency --title "Port default disagrees" --task readme-says-the-port   → inconsistency:port-default-disagrees
$ … set-field #meta/kind code-doc ; add-item #sides ×2 (--slug readme, --slug conf) ; set-slot ×3 ; commit doc type/scope/summary/body …

$ jigc task validate readme-says-the-port                         → advisory file-state.staged-copy only ; EXIT=0
$ jigc task finalize readme-says-the-port --dry-run               # an UNDECLARED pre-task staged path is present
would commit — docs(findings): record the port default disagreement
  promoted docs/inconsistencies/port-default-disagrees.md
  left-out (this commit takes only this task's docs and the artifacts they record — a staged path stays staged for the task it belongs to):
    server.conf
    scratch.txt
EXIT=0
$ jigc task finalize readme-says-the-port --dry-run --format json → "left_out": [{"kind":"left-staged","path":"server.conf"},{"kind":"untracked","path":"scratch.txt"}]
$ git status --short            → A  server.conf / ?? scratch.txt
$ jigc task finalize readme-says-the-port --carry-staged
finalized b977a9e — docs(findings): record the port default disagreement
  promoted docs/inconsistencies/port-default-disagrees.md
  1 file committed
EXIT=0
$ git status --short            → A  server.conf / ?? scratch.txt          # --carry-staged landed nothing
$ git show --stat --format= HEAD → docs/inconsistencies/port-default-disagrees.md | 28 +

# the ordinary model in the same rig, same staged path — the control F1 is graded against
$ jigc start --workflow single-task "add a rate limiter" ; … src/limiter.rs staged, adr:token-bucket-limiter authored,
  #status/cites-code = src/limiter.rs#limit, a Co-Authored-By item valued "Claude Opus <noreply@anthropic.com>" …
$ jigc task validate add-a-rate-limiter
blocking · finalize.carried-staged — `server.conf` was already staged before this task existed …
  route: unstage it (`git -C $REPO restore --staged -- server.conf`) if it is not this task's work, or pass `--carry-staged` …
EXIT=3
$ jigc task finalize add-a-rate-limiter --dry-run                 → the same finding ; EXIT=3
$ jigc task finalize add-a-rate-limiter --dry-run --carry-staged  → carried-over server.conf / added src/limiter.rs / promoted docs/decisions/token-bucket-limiter.md ; EXIT=0
$ printf 'my analysis\n' > .jigc/tasks/add-a-rate-limiter/notes.txt
$ jigc task finalize add-a-rate-limiter --carry-staged --format json
  "committed": { "displaced": [{"from": ".jigc/tasks/add-a-rate-limiter/notes.txt", "to": ".jigc/displaced/add-a-rate-limiter/notes.txt"}],
                 "files": 3, "left_out": [{"kind":"untracked","path":"scratch.txt"}],
                 "manifest": [promoted …, carried-over server.conf, added src/limiter.rs], … }
EXIT=0
$ git log -1 --format='%(trailers:only)'  → Co-Authored-By: Claude Opus <noreply@anthropic.com>      # exactly one
```

### 2.3 · Cell 7 — the amend step's trailer sentence (row 7.5, finding F2)

```
# same fresh rig; HEAD is the commit above and carries `Co-Authored-By: Claude Opus <noreply@anthropic.com>`
$ env -u CLAUDECODE jigc task amend "fix the subject"
task minted: fix-the-subject
amending: 69d0f5f "feat: add a token bucket rate limiter"
…
Trailers are re-authored too — the amended message carries only the trailer items
you add here, so re-add any the old message had that still apply:
…
$ env -u CLAUDECODE jigc doc set-field commit:fix-the-subject#type --value feat --task fix-the-subject
$ printf 'add a token-bucket rate limiter' | env -u CLAUDECODE jigc doc set-slot commit:fix-the-subject#summary --from-file - --task fix-the-subject
$ env -u CLAUDECODE jigc doc show commit:fix-the-subject --task fix-the-subject | tail -3     → "## Trailers" and nothing under it
$ env -u CLAUDECODE jigc task finalize fix-the-subject
amended 69d0f5f → 9de6815 — feat: add a token-bucket rate limiter
EXIT=0
$ git log -1 --format='%b'
Co-Authored-By: Claude <noreply@anthropic.com>            # zero items authored, variable unset — and respelled from "Claude Opus"

# the other arms (each: amend mint, type + summary only, finalize; all exit 0)
[unset · HEAD had none]  chore: add the scratch file        | trailers: []
[set   · HEAD had none]  chore: add the scratch file again  | trailers: [Co-Authored-By: Claude <noreply@anthropic.com>]
```

### 2.4 · `(8, N-1)` and finding F3 — `uninstall` (rows B.1, 5.3a, 5.3b, 1.38)

```
# the fresh rig of 2.2, later: no open task, one worktree (the main checkout), a parked file from row 1.23
$ jigc uninstall
blocking · uninstall.foreign-bytes — `.jigc/` holds 1 path(s) jigc did not write …
  .jigc/displaced/add-a-rate-limiter/notes.txt
EXIT=1
$ git worktree list | wc -l → 1 ; jigc task list → "no active tasks" ; the one untracked config delta was `git add`-ed
$ cat .jigc/displaced/add-a-rate-limiter/notes.txt   → my analysis          # before-control
$ jigc uninstall --force
warning: removing the relocation workbench .jigc/displaced discards work that is not in git:
    .jigc/displaced/add-a-rate-limiter/notes.txt
  note: the relocation workbench is the only copy of these bytes — they are not recoverable.
warning: removing `.jigc/` also removes 6 tracked file(s) under it: …
EXIT=0                                                # NOT inert with the three enumerated guards clean
$ jigc uninstall --help | grep -c 'Four states it refuses'                 → 1
$ jigc uninstall --help | grep -c 'Inert when all three guards are already clean' → 1

# F3 — two further fresh rigs
[knob off]  $ jigc uninstall → EXIT=0, "removed .jigc/" + six further lines ; ls -a → . .. .claude .git README.md
            $ jigc uninstall → "(nothing to remove — no repo-local jigc install was present)" ; EXIT=0

[knob on]   $ jigc config set invocation-log true ; git add .jigc/config/manifest.yaml ; git commit -m "chore: turn the invocation log on"
            $ jigc validate ; wc -l < .jigc/logs/invocations.jsonl → 2 ; git status --short --ignored → !! .jigc/logs/
            $ jigc uninstall
              - removed .jigc/  … (seven lines)
            EXIT=0
            $ find .jigc -type f     → .jigc/logs/invocations.jsonl          # re-created by this run's own log append
                                                                               # (counted on the 2.2 rig after its `uninstall --force`:
                                                                               #  1 line, argv ["uninstall","--force"] — the earlier records gone)
            $ git status --short     → … ?? .jigc/logs/                      # no longer ignored: .jigc/.gitignore went with the install
            $ jigc uninstall
              - removed .jigc/
            EXIT=0                                                             # the second run acted
            $ find .jigc -type f     → (nothing)
```

### 2.5 · Finding F5 — `uninstall` over files inside the three un-owned gitignored areas (row 5.3c)

```
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$ jigc validate >/dev/null ; mkdir -p .jigc/logs .jigc/state .jigc/index
$ printf 'MARKER-R9-LOGS my own notes\n'  > .jigc/logs/notes.txt
$ printf 'MARKER-R9-STATE my own notes\n' > .jigc/state/notes.txt
$ printf 'MARKER-R9-INDEX my own notes\n' > .jigc/index/notes.txt
$ printf 'MARKER-R9-CONTROL\n'            > .jigc/config/notes.txt        # the guard's declared subject, as a control

$ command grep -rl "MARKER-R9" .jigc | sort                                # before-control: all four found
.jigc/config/notes.txt
.jigc/index/notes.txt
.jigc/logs/notes.txt
.jigc/state/notes.txt

$ jigc uninstall                                                           # the control refuses, and names ONLY the control
blocking · uninstall.untracked-workbench-file — `.jigc/` holds 1 file(s) that no index has a copy of — removing `.jigc/` would destroy them:
  .jigc/config/notes.txt
EXIT=1
$ mv .jigc/config/notes.txt "$RIG/control-notes.txt"                       # the control leaves the repository

$ jigc uninstall                                                           # no --force
jigc uninstall — repo-local install removed
  - removed .jigc/  … (seven lines)
[stderr] warning: removing `.jigc/` also removes 5 tracked file(s) under it: … (the five install files, nothing else)
EXIT=0
$ command grep -rl "MARKER-R9" . | sort     → (nothing)                    # after: all three gone
$ ls -a                                     → . .. .claude .git README.md
$ git stash list | wc -l                    → 0
```

### 2.6 · Finding F4 — the migration forecast (rows 1.32, 1.33)

```
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$ mkdir -p docs/decisions ; printf '# Use PostgreSQL\n\nWe picked postgres because of reasons.\n' > docs/decisions/use-postgresql.md
$ git add docs ; git commit -m "docs: old adr"
$ jigc migrate docs/decisions/use-postgresql.md --as adr     → task minted: migrate-adr-docs-decisions-use-postgresql-038c497bd8bf
$ jigc doc author adr --from-file - --task <id> <<'EOF' … title + context/decision/consequences … EOF      → adr:use-postgresql

$ git status --short -- .jigc/config .jigc/.gitignore | wc -l     → 0       # before-control: neither path is dirty
$ git diff --stat HEAD -- .jigc/config .jigc/.gitignore | wc -l   → 0

$ jigc task finalize <id> --dry-run --format json
  manifest: [promoted docs/decisions/use-postgresql.md, modified .jigc/config, modified .jigc/.gitignore]   left_out: []
EXIT=0
$ jigc task finalize <id>                 → "migration review required — nothing committed. …" ; EXIT=4 ; HEAD unmoved
$ jigc task finalize <id> --approve --format json
  committed.manifest: [promoted docs/decisions/use-postgresql.md]   files: 1
EXIT=0
$ git show --name-status --format='%s' HEAD
docs(adr): adopt docs/decisions/use-postgresql.md as a managed adr
M	docs/decisions/use-postgresql.md

# control, same rig: an ordinary docs-only task (record-decision)
  forecast manifest: [promoted docs/decisions/use-redis.md]   landed manifest: [promoted docs/decisions/use-redis.md]   files: 1
```

### 2.7 · `(8, N-2)`, `unmanage`, and the milestone doors (rows B.2, 5.5, 5.13, 5.14, 1.27, 7.2)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit

$ jigc rename vision:vision --to "New Vision"
blocking · write.identity-change — cannot reslug `vision:vision` … only a retitle is supported
  route: `jigc rename vision:vision --to 'New Vision' --slug vision` keeps the identity that cannot move and rewrites only the title
EXIT=1
$ jigc rename vision:vision --to 'New Vision' --slug vision              # the route, verbatim
renamed vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)
EXIT=0
$ grep -n '^# ' VISION.md → 5:# New Vision ; git log -1 --format=%s → rename VISION.md -> VISION.md ; one line changed
$ jigc rename vision:vision --to 'New Vision' --slug vision
no-op: vision:vision already holds the title "New Vision" at VISION.md — nothing renamed, nothing committed
EXIT=0
$ jigc rename vision:vision --to 'Newer Vision' --slug vision --format json  → { "from": "vision:vision", "to": "vision:vision", …, "title": "Newer Vision", "commit": "626762f", … }
                                                                               # the JSON envelope does carry the title; the text ack and the subject do not

$ jigc unmanage docs/decisions-log.md   → "unmanaged docs/decisions-log.md (decisions-log:decisions-log) — dropped its file-state baseline + forward edges; the file is left on disk. …" ; EXIT=0
$ jigc unmanage docs/decisions-log.md   → "no-op: docs/decisions-log.md is not managed (nothing to drop)" ; EXIT=0 ; bytes and HEAD unchanged

$ printf 'staged by me\n' > mine.txt ; git add mine.txt
$ jigc milestone create "Ship limiter"                      → record commit: 24f50ff … ; HEAD moves ; 1 file
$ jigc milestone add-task ship-limiter "first sub task"     → record commit: d9c17b9 … ; trailer present
$ env -u CLAUDECODE jigc milestone add-task ship-limiter "second sub task" → record commit: 0c871f1 ; trailers: []
$ jigc milestone execute ship-limiter                       → EXIT=0 ; 2 `Spawn:` lines ; HEAD unmoved
$ jigc task discard first-sub-task                          → "record commit: fbfdca6 — this sub-task's milestone record, settled to `discarded` and committed on its own" ; EXIT=0
$ jigc milestone discard ship-limiter                       → "discarded milestone:ship-limiter (2 sub-task(s); workbench removed)" ; HEAD moves by one
$ git status --short                                        → A  mine.txt       # through all five commits

$ jigc milestone create "Second wave" ; add-task … "write the widget" ; jigc milestone provision second-wave
$ (cd .jigc/worktrees/write-the-widget && jigc workflow sub-task --task write-the-widget ; widget.rs staged ; commit doc type + summary)
$ jigc milestone join second-wave                           → joined … 1 doc(s) merged ; EXIT=0
$ jigc milestone finalize second-wave
blocking · finalize.carried-staged — `mine.txt` was already staged before this milestone existed — the aggregate commit is built from the sub-task worktrees and cannot carry it …
EXIT=3
$ jigc milestone finalize second-wave --carry-staged
finalized 80ec3df — Finalize milestone second-wave (1 sub-task)
  modified docs/milestone-records/second-wave.md / added widget.rs / 2 files committed
  staged in the shared checkout (not committed by this boundary …; these stay staged):  mine.txt
EXIT=0

$ … land spec:widget-spec (two criteria) through `plan` … ; jigc milestone create "Fourth wave"
$ jigc milestone add-from-spec fourth-wave spec:widget-spec
seeded 2 sub-task(s) … : renders-a-widget, hides-a-widget
record commit: d3fa8ef …
record commit: a2006f9 …
EXIT=0                                                      # two commits added
```

### 2.8 · The remaining help drives (rows 4.5–4.7, 5.6–5.12, 1.26, 1.28, 1.29)

```
$ jigc doc create adr --title "The v1.1 cache!" --task slug-probe-one                          → adr:v1-1-cache
$ jigc doc create adr --title "Use in-memory store for sessions today" --task slug-probe-two   → adr:use-in-memory-store
$ jigc doc create adr --title "In-memory cache" --task slug-probe-three                        → adr:in-memory-cache
$ jigc doc create adr --title "v1.1 cache" --task slug-probe-one                → write.title-ignored, route `jigc doc rename adr:v1-1-cache --to 'v1.1 cache' --task …` ; EXIT=1
$ jigc doc create adr --title "Something else entirely" --task slug-probe-one   → write.identity-change ; EXIT=1
$ jigc doc create adr --title "…" --slug "Bad_Slug" --task slug-probe-one       → `--slug "Bad_Slug"` is not a valid slug … ; EXIT=1
$ jigc doc create adr --title "Use A Router!!" --task committed-title-probe     → write.title-ignored (committed), route: distinct `--title` or `--slug <slug>` ; EXIT=1

$ jigc doc create inconsistency --title "Port mismatch" --task second-report    → create.already-exists ; EXIT=1
$ jigc doc author inconsistency --from-file <payload> --task second-report      → create.already-exists ; EXIT=1
$ jigc doc list --task second-report                                            → commit:second-report only

$ jigc workflow single-task --preview        → "preview: workflow `single-task` — no task minted." ; EXIT=0 ; .jigc/tasks empty
$ jigc workflow amend --preview              → workflow.verb-routed, route `jigc task amend` ; EXIT=1
$ jigc workflow migrate-adr --preview        → workflow.verb-routed, route `jigc migrate <path> --as adr` ; EXIT=1
$ jigc workflow router --preview             → "mints no task, so there is nothing to preview" ; EXIT=1
$ jigc workflow single-task                  → clap: required `<--task <TASK>|--preview>` ; EXIT=2
$ jigc workflow single-task --preview --task x → clap: cannot be used with ; EXIT=2

$ jigc task finalize tiny-fix --dry-run      → finalize.empty-commit ; EXIT=3      (clean tree)
$ jigc task finalize tiny-fix --dry-run      → finalize.nothing-staged ; EXIT=3    (an unstaged edit)
$ jigc task finalize tiny-fix --approve      → finalized e3bff0f — fix: tiny ; EXIT=0

# the rejection frame on the doc-only arm, invocation log on
$ printf '#!/bin/sh\necho "policy: subject must carry a ticket" >&2\nexit 1\n' > .git/hooks/commit-msg ; chmod +x …
$ jigc task finalize hook-probe-doc-only
`git commit` was rejected (no commit was made):
policy: subject must carry a ticket
task hook-probe-doc-only is intact — nothing was committed, your task's staged docs are still in `.jigc/tasks/hook-probe-doc-only/docs/`, … re-run `jigc task finalize hook-probe-doc-only`.
EXIT=1 ; HEAD unmoved ; last log record: exit_code 1, error_code finalize.commit-rejected
```

---

## 3 · Defects

Tier predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying
or moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the
binary does not do.

### (R9, F1) — the commit boundary's help and both guides state the carryover gate as a universal; the doc-only commit model is on none of them

**Door:** `task finalize` (and `task validate`) · **cell:** C1 / C3 · **exit:** 0 · **code:** none.

**What is contradicted.** `jigc task finalize --help`, read from the installed binary:

* the about: *"On a task minted by `jigc task amend` it takes its **second commit model**"* — there
  are three (`CommitModel`: `Index`, `Amend`, `DocOnly`), and the third is named nowhere in the text;
* `--carry-staged`: *"land index entries staged before this task existed instead of refusing … Inert
  when nothing is carried, and inert on an **amend** task in every state"*;
* `--dry-run`: *"It refuses, rather than printing a manifest, on every gate decided before the
  transaction — … and the carryover gate, where an undeclared carry-over is reported (exit 3) instead
  of the manifest … On an **amend** task three of those read differently"*.

And the installed guide: QUICKSTART §3 — *"A change staged before the task existed refuses to ride the
commit — one blocking `finalize.carried-staged` per carried path — unless you declare it with
`--carry-staged`"*, and *"the file-set the commit carried (the git index: what you staged plus the
docs jigc promotes)"*; MIGRATING gate 4 — *"at finalize anything staged before the task existed
refuses"*.

**Driven (2.2).** On a `report-inconsistency` task with a path staged before the mint: `--dry-run`
prints a manifest at **exit 0** (no finding, no exit 3); `--carry-staged` is accepted and lands
**nothing** of the carried path (`A server.conf` before and after; the commit holds one file). The
same staged path in the same rig refuses with `finalize.carried-staged` at exit 3 on a `single-task`
task — the control.

**The binary knows.** The composed step says it correctly (*"the carryover gate does not fire here and
`--carry-staged` changes nothing"*), the left-out narration says it, MIGRATING gate 1 was swept for
`left-staged`, and `design/finalize.md` states *"`--carry-staged` is inert in every state"* on this
arm. The help and four guide sentences are the unswept siblings; the help's own text enumerates its
exceptions (*"and inert on an amend task"*) and the enumeration is one short.

**Proposed tier: 3** — a surface states a refusal and a carry the binary does not perform on this arm;
nothing is lost (the staged path stays staged) and no route dead-ends.

### (R9, F2) — the composed `amend` step says the amended message carries only the trailers you author; it carries the co-author trailer you did not

**Door:** `task amend` → `task finalize` (amend arm) · **cell:** C1 · **exit:** 0 · **code:** none.

**What is contradicted.** The composed `amend` workflow, emitted by `jigc task amend`: *"Trailers are
re-authored too — the amended message carries only the trailer items you add here, so re-add any the
old message had that still apply."*

**Driven (2.3) — three arms driven, zero trailer items authored in each; the fourth is stated, not driven:**

| `CLAUDECODE` | `HEAD` carried the trailer | the amended message's trailers | the sentence |
|---|---|---|---|
| unset | yes (`Claude Opus <noreply@anthropic.com>`) | `Co-Authored-By: Claude <noreply@anthropic.com>` | **false** |
| unset | no | none | true |
| set | no | `Co-Authored-By: Claude <noreply@anthropic.com>` | **false** |
| set | yes | *not driven on the amend arm* (row 7.4 drove the same de-duplication on the ordinary arm) | — |

`design/assistant-adapter.md` → *The co-author trailer* → *The amend arm* declares the behaviour
(*"re-derived — and carried over when `HEAD`'s message already carries the profile's trailer"*), so
the binary matches its design and the **step text** is what did not move: commit `28ca86b4` touched no
pack file.

**Two data that bound it.** (a) The carried trailer is the profile's rendering, not `HEAD`'s bytes:
`Claude Opus <…>` became `Claude <…>`. (b) Because the trailer is re-applied whenever `HEAD` carried
it, a reader who follows the sentence to **drop** a co-author line (authoring none) gets it back at
exit 0; the installed `AGENT.md` tells the agent never to use `git commit --amend` by hand, so jigc
offers no route that removes it. Not graded separately — the design declares it cannot tell a human
from an agent inside one session — but it is the consequence the false sentence hides.

**Proposed tier: 3** — the step states the landed message's content and the binary lands a different
one; no loss, the trailer is added or kept, never dropped.

### (R9, F3) — with the opt-in invocation log on, `jigc uninstall` leaves `.jigc/` behind, un-ignored, and its second run is not a no-op

**Door:** `uninstall` · **cell:** C1 · **exit:** 0 · **code:** none.

**What is contradicted.** `jigc uninstall --help` (and the top-level one-liner, which is the same
text): *"removes `.jigc/`"* and *"Idempotent: a second run is a clean no-op"*. The run's own ack:
*"removed .jigc/"*.

**Driven (2.4).** With `invocation-log` on, the uninstall removes the tree and then appends its own
invocation record, re-creating `.jigc/logs/invocations.jsonl`. After an exit-0 run that printed
*"removed .jigc/"*, `.jigc/` exists and — because `.jigc/.gitignore` went with the install — git
reports `?? .jigc/logs/`, so the guides' *"gitignored `.jigc/logs/invocations.jsonl`"* is no longer
true of it and a later `git add -A` would stage it. The second `jigc uninstall` prints *"removed
.jigc/"* again and acts. With the knob off (the control) both sentences hold.

**Proposed tier: 3** — the ack and the help say the tree is gone and it is not; the residue is one
record jigc wrote, nothing of the user's is touched.

### (R9, F4) — a migration task's `--dry-run` forecasts two `modified` paths that are not modified and that the commit does not carry

**Door:** `task finalize` (migration arm) · **cell:** C3 · **exit:** 0 · **code:** none.

**What is contradicted.** MIGRATING gate 1: *"It prints the manifest the finalize would commit — each
path tagged by how it enters"*; QUICKSTART §3: *"To see that set before committing, run `jigc task
finalize <id> --dry-run`"*; the flag's help: *"the file-set the commit would carry"*.

**Driven (2.6), on two rigs.** Forecast: `promoted docs/decisions/use-postgresql.md`, `modified
.jigc/config`, `modified .jigc/.gitignore`. Neither `.jigc` path differs from `HEAD` (before-control:
`git status` and `git diff HEAD` both empty over them). Landed: one file, `M
docs/decisions/use-postgresql.md`; the landed envelope's `committed.manifest` has one row. An ordinary
docs-only task in the same rig forecasts exactly what it lands — the over-report is the migration
arm's.

**Not an accident, and outside the range.** `predict_manifest`'s doc-comment states the migration
forecast is *"exactly its set — promotions, tracked retirements, and the jigc-tracked config layer
`.jigc/config` / `.jigc/.gitignore` (`modified`)"*, and `crates/cli/tests/finalize_manifest.rs` asserts
the `.jigc/config` row. The forecast prints the **pathspec**, not the change-set; the guide and the
help call it the file-set the commit would carry. This predates M54 — it is reported because cell 1
drives MIGRATING gate 1, a sentence the range edited, and the drive does not hold.

**Proposed tier: 3** — the forecast names two paths, tagged `modified`, that the binary neither
modifies nor commits; nothing is lost.

### (R9, F5) — `jigc uninstall`, without `--force`, destroys files inside `.jigc/logs/`, `.jigc/state/` and `.jigc/index/` at exit 0, named by nothing — where its help says every such file blocks

**Door:** `uninstall` (a `DESTROYING_DOORS` member, `Refuse` disposition) · **cell:** C1 · **exit:**
0 · **code:** none.

**What is contradicted.** `jigc uninstall --help`: *"Four states it refuses instead of destroying,
because `.jigc/` is their only copy — [worktrees], [an open task's staged doc], and **any other file
under `.jigc/` that no index has a copy of** — a recorded config delta among them — blocks with
`uninstall.untracked-workbench-file` …, and a file jigc did not write inside a working area under
`.jigc/tasks/` or `.jigc/milestones/`, or anything parked under `.jigc/displaced/`, blocks with
`uninstall.foreign-bytes`. Any of them removes nothing until you re-run — or pass `--force`"*.

**Driven (2.5).** Three files planted inside `.jigc/logs/`, `.jigc/state/` and `.jigc/index/`, plus a
control at `.jigc/config/notes.txt`. Before-control: `command grep -rl` finds all four. `jigc
uninstall` refuses over the **control only** and lists only it. With the control moved out, `jigc
uninstall` — no `--force` — exits **0**, prints *"removed .jigc/"*, warns about the five tracked
install files and nothing else, and afterwards the marker is found nowhere in the repository: the
three files are gone, in no index, no stash, no narration.

**Why.** The third guard's subject is the complement of `crate::gitignore::ENTRIES`
(`workbench_paths`); `tasks/`, `milestones/`, `worktrees/` and `displaced/` are excluded from it
because other guards own them. `logs/`, `state/` and `index/` are excluded by the same computation
and **no** guard owns them. (Read after the drive, to explain it; the finding is the drive.)

**The realistic member of the class** is the invocation log itself: an adopter who opted into
`invocation-log` loses every record at `jigc uninstall`, exit 0, unnamed (2.4, knob-on: two records
before; after, the file exists again holding only what the uninstall itself appended — counted on the 2.2 rig, one line). It is jigc-written and
gitignored, it is not rebuildable the way `index/` and `state/` are, and nothing has another copy.

**Precedent, and what is new.** M52's baseline entry **L-2** (`completions/artifacts/M52/baseline-destroying.md`)
is this door destroying a plain file planted **at** an `ENTRIES` name at exit 0, *"named by nothing,
refused by nothing"*; M52 Increment 4 closed it. This is the same door and the same outcome one level
down — a file **inside** three of those directories. I found no record that declares these three
areas outside the guard as a bound; the help states the opposite universal.

**Outside the range.** Neither M54, M55 nor the trailer changed this door's guards. It is reported
because cell 5 drives `uninstall --help`'s refusal sentences at the verb, and this one fails.

**Proposed tier: 1** — on the predicate's letter: an exit-0 loss of sole-copy bytes through a
destroying door, without the door's consent flag, shown with a before-control and their absence.
**The alternative reading is tier 3**, if the reconciler holds that `logs/`, `state/` and `index/`
are jigc-only populations in which no user byte is expected — then what remains is a help universal
the binary does not honour, plus an un-narrated deletion of the opt-in log. That ruling is the
human's; I did not find it made anywhere, so I am not making it by proposing the lower tier.

---

## 4 · What I did NOT drive, and why

| # | (door, cell) pair or claim | why not |
|---|---|---|
| ND-1 | the install line **executed** (`cargo install jigc --version '^1.0.0-rc.1' --locked`), and the guide's sentences about where cargo puts the binary, the unpinned line resolving nothing, and an upgrade being the same line | the scope gives it to row 1 and forbids it on this host; row 1's table carries it (its rows 6.1, 6.2, 6.4, 8.4, 8.5). Here it is a byte comparison only |
| ND-2 | the **rc.24 crates.io page** as rendered, and whether each README URL answers 200 | not re-read by anyone in this run; rc.23's was (`completions/artifacts/M55/crates-page-reread.md`). The README is byte-unchanged rc.23 → rc.24 (row 3.2), which is a fact about the file, not about the page |
| ND-3 | the **packaged** README bytes | row 1 drove it (its row 7.3) — cited, not duplicated |
| ND-4 | `migrate-corpus` landing a **real** migration commit, and MIGRATING's upgrade gates (`schema-version-current`, `schema-version-ahead`, `store-version.binary-mismatch`) | needs a corpus stamped at another schema-version; none can be built with this binary alone and no older binary may be built. Row 4's subject. The verb ran on its no-op arm only (row 1.34) |
| ND-5 | `milestone finalize` under `finalize.fan-out.squash: false` | one fan-out driven (the default arm); the chain arm is row 5's |
| ND-6 | the other `--dry-run` refusal codes the help lists: `finalize.base-mismatch`, `finalize.milestone-sub-task`, `finalize.promote-clobber`, `finalize.migration-no-replacement`, `finalize.amend-index-dirty`, `finalize.amend-staged-doc` | three of the nine driven (`empty-commit`, `nothing-staged`, `carried-staged`); the rest predate the range and are row 5's |
| ND-7 | QUICKSTART's *"A byte jigc cannot move"* paragraph (`finalize.foreign-bytes`), and `milestone finalize`'s displacement of the milestone's own area | not reached; pre-range sentences |
| ND-8 | `setup`'s install commit passing `--no-verify`, and the hook-location cases beyond the default and an in-tree `core.hooksPath` (a submodule's hooks dir, a sparse-checkout exclusion, a linked worktree) | row 1's subject |
| ND-9 | `doc author`'s `already existed — copied in for update` ack | the one re-author I ran hit `write.already-present` first; not re-staged |
| ND-10 | the manifest tag `deleted` | no retirement in any driven finalize (the one migration was same-path) |
| ND-11 | MIGRATING gate 8 (`schema-conformance.home-vacated`, its three routes) | row 3's subject |
| ND-12 | MIGRATING's large-document timings | a measurement, not a contract cell |
| ND-13 | the `validate` families one by one | the sweep ran in five store states; findings were observed from `doc-code`, `file-state` and `schema-conformance` only. The help's family list is graded against the registry (row 6.1), not family by family |
| ND-14 | `relocate`, `doc remove-item`, `doc retitle-item`, `doc rename`, `task bind`, `config insert-step`, `config replace-step`, `config remove-step`, `config fill`, `config fork`, `config list` | their `--help` ran (exit 0); the verbs did not. None is in the range's sentence batch |
| ND-15 | baseline **D-1** (both default-text paths render carried findings through one renderer) | no cell of this row reached it; the scope says re-drive only if one does |
| ND-16 | the trailer's remaining matrix (every committing door × set / empty / cleared; a human typing inside an agent session) | row 10's; this row drove the cells where a **sentence** speaks (7.3–7.5) |
| ND-17 | the ten generated help texts the instrument did not name (`doc create`/`rename`/`set-slot`/`set-field`/`add-item`, `milestone create`/`add-task`/`add-from-spec`, `task amend`/`discard`) against their generating constants, byte for byte | their claims were driven at the verb for six of them; only the four the scope lists were compared to a registry |

---

## 5 · Baseline rows: CLOSED / STILL-OPEN

| key | rc.16 | rc.24 | datum |
|---|---|---|---|
| `(8, N-1)` | tier 3, open | **STILL-OPEN (expected, 1.x)** | `jigc uninstall --help`: about *"Four states … deletes all four"*, `--force` *"a fan-out worktree with content, an open task's staged docs, or a workbench file no index has a copy of … Inert when all three guards are already clean"*. Driven (2.4): the three enumerated guards clean, `--force` deletes `.jigc/displaced/add-a-rate-limiter/notes.txt` and narrates it |
| `(8, N-2)` | tier 3, open | **STILL-OPEN (expected, 1.x)** | text ack `renamed vision:vision -> vision:vision (VISION.md -> VISION.md)`, subject `rename VISION.md -> VISION.md`, while `VISION.md:5` became `# New Vision`. New datum: `--format json` carries `"title"`, so the envelope is informative where the text and the git record are not |
| M51 `CX-1` | CLOSED | **CLOSED** (reached by row 1.17) | the invocation-log exception: knob default `false`; on, three reads appended three records; `migrate-corpus --dry-run` prints the exception |
| M51 `CX-2` | CLOSED | **CLOSED** (rows 1.3, 1.4) | edited guide left byte-identical, `adapter-guide.user-modified` at `setup` and `upgrade`, dropped from the install commit |
| M51 `CX-3` | CLOSED by scoping the command to a clone | **CLOSED on the rewritten sentence** (rows 1.37, 2.1) | the installed guide's one install command is `cargo install jigc --version '^1.0.0-rc.1' --locked`: no path, no `# cwd:`, nothing a working directory can break. **Its execution is not mine** — row 1's 6.1 (ND-1) |
| M51 `D-1` | CLOSED | **NOT RE-DRIVEN** (no cell reached it) | ND-15 |
| M51 `D-2` | CLOSED (9 verbs, 10 rows) | **CLOSED** (rows 7.1, 7.2) | all **11** `COMMITTING_DOORS` clauses are in their door's help; nine of the eleven rows driven to their commit (not `migrate-corpus`, not `milestone finalize (squash: false)`), plus the doc-only arm, which has no row |

---

## 6 · Observations (driven, not graded)

- **O-1 · `setup`'s help enumerates less than `setup` writes.** The about and the top-level one-liner
  list the bootstrap, the `CLAUDE.md` reference, the project layer, the `Bash(jigc:*)` permit and the
  two hooks. The run also writes the guide artifact, a second permit (`Bash(git add:*)`) and a
  22-entry `deny` list (row 1.1). The ack names the guide; nothing adopter-facing names the deny
  list at install — `uninstall`'s ack (*"removed deny safety floor"*) is where a reader first meets
  it. An omission, not a false statement; pre-range.
- **O-2 · No adopter-facing sentence says a commit is signed.** Grep over the installed guide and all
  49 help texts for `co-author`, `trailer`, `authorship`: zero hits about the agent's trailer. The
  only adopter-visible sentences about trailers are the two composed steps — `author-commit` (still
  invites a hand-recorded co-author, absorbed only when the address matches) and `amend-message`
  (F2). QUICKSTART's *"renders the commit-doc into the git commit message"* is therefore incomplete
  under an agent: the message is the render plus one line the doc does not hold.
- **O-3 · `describe` homes the `migrate-*` workflows at pre-`docs-root` paths** — *"into a managed
  `adr` at `decisions/`"* — where the doctype homes at `docs/decisions/` by default. Pre-range pack
  prose; `unmanage --help`'s example path has the same shape.
- **O-4 · `jigc doc list` reports `managed` for a doc `jigc unmanage` has just dropped**, and
  `unmanage`'s re-run says *"is not managed"*. `design/doc-read-surface.md` defines the listing's
  `managed` by the stamp, so this is declared; it reads as two verbs disagreeing. Rows 3 and 7.
- **O-5 · `uninstall` leaves `.claude/settings.json` as a four-key skeleton** (`hooks.SessionStart:
  []`, `permissions.allow: []`, `permissions.deny: []`) in a repository that had no such file before
  `setup`. Row 1's subject.
- **O-6 · A lead for row 8, not driven here:** the composed `plan` workflow prints `jigc doc set-field
  spec:<slug>#derived-from …` while `jigc doc schema spec` prints the address as
  `spec:<slug>#meta/derived-from`. I did not run either form.

---

# PART II — Reconciliation ledger

Status words: **CONFIRMED** = driven to a repro block by the reconciler · **REFUTED** = a driven
output contradicts it (the datum is quoted) · **OPEN LEAD** = not drivable here, with the reason —
never promoted on a source read, never dropped.

## R.1 The door-set count, against the registries

Read by symbol at the working tree (`git diff jigc-v1.0.0-rc.24 HEAD` over `crates/` and the two
READMEs is empty, so the tree read is the tag's).

| registry | driver's count | reconciler's read | verdict |
|---|---|---|---|
| `VERB_KINDS` (`crates/cli/src/cli.rs`) | 48 = 13 + 11 + 7 + 8 + 9 | 48: 13 top-level · 11 `doc` · 7 `task` · 8 `config` · 9 `milestone` | **agrees** |
| `COMMITTING_DOORS` (`crates/cli/src/invocation_log.rs`) | 11 rows over 9 verbs | 11 rows; verbs `task finalize` (2 rows), `milestone finalize` (2 rows), `rename`, `migrate-corpus`, `milestone create`, `milestone add-task`, `milestone add-from-spec`, `milestone discard`, `task discard` = 9 | **agrees** |
| `*_long_about()` builders | 14 (the instrument named 4) | 14: `cli.rs` 2 · `doc.rs` 6 · `milestone.rs` 3 · `task.rs` 3 | **agrees with the driver**; the instrument's 4 is the subset generated from a named registry |
| `STORE_FAMILIES` | 7 | 7 | agrees |
| `SchemaChangeKind::ALL` | 18 | 18 | agrees |
| `WHOLE_DOC_KEYS` | 7 (+ `staged`) | 7 | agrees |
| `CommitModel` | 3 | 3 (`Index`, `Amend`, `DocOnly`) | agrees |
| shipped workflows | 39 = 18 + 21 | 18 + 21 | agrees |
| router-visible set | 13 | 13 rows printed by bare `jigc start` (RD-F1); `jigc describe --workflows` carries the hidden clause on 26 of 39 (RD-G1) → 13 visible | agrees |
| crate README links | 7 occurrences / 6 targets | 7 / 6 | agrees |
| homes of the install line | 3 (+ the installed copy) | 3 repository homes + the installed `SKILL.md`, four equal SHA-256 (RD-B, RD-R) | agrees |

**No count the driver states differs from its registry.** The driver file has no *Doors covered*
list of its own; R.8 supplies it.

## R.2 Rows marked driven that carry no repro block — demoted

The row schema: *a row is driven iff its argv ran on that binary*; the evidence of that is a repro
block. §2 of the driver file carries eight blocks. Read row by row against them:

**21 rows carry no repro block at all** — their argv and exit appear in no block (three of them
are named in a block's *heading* but not in its body: 1.26 and 1.29 under 2.8, 4.5 and 4.6 under
2.8):

`1.7` · `1.14` · `1.18` · `1.19` · `1.24` · `1.26` · `1.29` · `1.34` · `1.35` · `1.36` · `4.5` ·
`4.6` · `5.1` · `5.8` · `5.9` · `5.15` · `6.1` · `6.2` · `6.3` · `6.4` · `7.1`

**8 rows carry a block for some arms and none for others:** `1.12` (the `doc-code.symbol-exists`
arm) · `1.17` (`config get`, the three-reads count, `git check-ignore`) · `1.32` (`jigc ingest`) ·
`5.4` (the drifted and un-baselined arms) · `5.7` (`write.already-present`) · `5.11` (the clean
arm) · `5.13` (`milestone list-tasks`, the unknown-id arm) · `5.14` (the unknown-id arm).

**Consequence on the driver's own evidence:** 55 of its 76 rows are driven, and seven leaves lose
their only block — `doc schema`, `task diff`, `config get`, `ingest`, `migrate-corpus`,
`describe`, `milestone list-tasks`. Cell 6 (*generated
help == its registry*) has **no** driven row left: all four of its rows are block-less.

**Every demoted row was then re-driven by the reconciler** (blocks in R.5). Outcome:

| row(s) | re-drive | outcome |
|---|---|---|
| 1.7 | RD-B | holds — `setup --force` on a clean footprint: exit 0, `HEAD` unmoved, tree clean |
| 1.14 | RD-G1 | holds |
| 1.18, 1.19, 1.24, 1.36, 5.9 | RD-G2 | hold |
| 5.8 | RD-G2 + RD-D | holds, all five arms (the two slice arms on `roadmap:roadmap`, not on an inconsistency) |
| 1.26, 1.29 | RD-G3 | hold (1.26's two refusals were driven on two tasks: `task-discard.staged-prose` on one, `task-discard.foreign-bytes` on the other) |
| 1.34, 1.35, 4.5, 4.6 | RD-G1 | hold |
| 5.1, 6.1, 6.2, 6.3, 6.4, 7.1 | RD-H (+ RD-D for 6.3's emitted keys) | hold |
| 5.15 | RD-H | holds for the five top-level one-liners (compared by script; `validate`'s differs only by clap's stripped full stop); **the eight sub-verb one-liners were not compared and stay undriven** |
| partial arms: 1.12, 1.17, 1.32, 5.7, 5.13, 5.14, 5.4 (drifted) | RD-G2, RD-S, RD-F3, RD-F4, RD-G3, RD-M | hold |
| partial arms: 5.4 (un-baselined store), 5.11 (a finding-free `task validate`) | — | **not re-driven; stay undriven** |

One datum bounds row 5.7: the same **item-less** payload authored twice exits 0 both times
(RD-G2); `write.already-present` at exit 1 needs a payload that carries an item (RD-G3). The
driver's row says *"the same payload twice in one task"* without that condition.

## R.3 The Codex source pass — every claim

### Claim 1 — `lead(codex, task amend --help says the message is authored from scratch and HEAD's message is not read back, but an amend with CLAUDECODE unset reads HEAD and carries its co-author trailer over)` → **CONFIRMED · `(R9, C-1)` · tier 3 · door `task amend`**

Driven (RD-F2, arm A): `HEAD` carries `Co-Authored-By: Claude Opus <noreply@anthropic.com>`;
`env -u CLAUDECODE jigc task amend …`, `type` + `summary` authored, **zero** trailer items (the
staged commit doc's `## Trailers` is empty); `jigc task finalize` → exit 0, and the amended
message carries `Co-Authored-By: Claude <noreply@anthropic.com>`.

What the help says (read from the binary): *"HEAD's message is not read back into the doc: you
author the new message from scratch."* **The narrower literal holds** — nothing of `HEAD`'s
message enters the *doc*. **The sentence's second half does not**: the landed message is the
rendered doc plus one line taken from `HEAD`'s message, which the binary read to find it.

**Wider than the claim, found while driving it:** the installed guide carries the same sentence —
*"You re-author the message from scratch rather than editing the old one — jigc does not read the
landed message back into the doc"* (`SKILL.md`, the *Landed it with a wrong message?* paragraph;
RD-B). The driver graded that paragraph *matches* at row 1.25. So the seam has three adopter
surfaces: the composed step (driver `(R9, F2)`, the flatly false one), the help (this claim) and
the guide paragraph. `(R9, C-1)` is the help and the guide; `(R9, F2)` is the step.

**Tier 3, and why not 1:** exit 0 through a committing door (the amend), but neither loss nor
repository harm — no byte is lost (the superseded commit stays in the reflog, the tree hash is
unchanged) and the landed message is the one `design/assistant-adapter.md` → *The co-author
trailer* declares. The harm half is missing.

### Claim 2 — `lead(codex, baseline (8, N-2) remains open)` → **CONFIRMED · baseline `(8, N-2)` STILL-OPEN · tier 3 · door `rename`**

RD-N2: `jigc rename vision:vision --to 'New Vision' --slug vision` → exit 0, ack `renamed
vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)`, subject `rename
VISION.md -> VISION.md`, while `VISION.md:5` became `# New Vision`. The no-op re-run prints the
title; `--format json` carries `"title"`. Both passes agree.

### Baseline disposition — `lead(codex, (8, N-1) is CLOSED: the flag help's "all three guards" refers to the three categories in that flag sentence)` → **REFUTED**

**Falsifying datum (RD-N1).** With the three guards the flag sentence enumerates all clean (no
worktree with content, no open task, no untracked workbench file) and only
`.jigc/displaced/add-a-rate-limiter/notes.txt` present:

```
$ jigc uninstall          → blocking · uninstall.foreign-bytes … .jigc/displaced/add-a-rate-limiter/notes.txt   EXIT=1
$ jigc uninstall --force  → warning: removing the relocation workbench .jigc/displaced discards work that is not in git: …   EXIT=0
```

The flag help reads *"Inert when all three guards are already clean"*. Under the source pass's own
reading of *"all three"*, those three **are** clean here and `--force` is not inert — it deletes
the parked file. The reading does not rescue the sentence; the row is **STILL-OPEN** (R.6).

### *Consistent source read* — eight statements

| # | `lead(codex, …)` | status | datum |
|---|---|---|---|
| S-1 | no adopter guide, CLI help or composed step says jigc adds the trailer automatically | **CONFIRMED** | RD-H: 0 of 49 help texts match `co-author\|trailer\|authorship`; RD-B: the installed guide's one hit is the `Refs:` clause of the amend paragraph. The only adopter-visible sentences are the two composed steps (= driver O-2) |
| S-1b | `step:author-commit` still invites a hand-recorded `Co-Authored-By`; the result is redundant, not double-written | **CONFIRMED** | RD-F1: the composed `single-task` prints the two trailer commands; an authored item `Claude Opus <noreply@anthropic.com>` with the variable set lands **one** trailer, in the authored spelling |
| S-2 | every registered committing door funnels through the signing seam; `setup` signs separately; no production `git commit` bypasses it | **CONFIRMED for 10 commit constructions · OPEN LEAD for 3** | variable set → exactly one trailer at: `setup` (RD-B) · `task finalize` index arm (RD-F1, by de-duplication), doc-only arm (RD-F1), amend arm (RD-F2 arms C, D) · `rename` (RD-N2) · `milestone create`, `milestone add-task`, `task discard` (sub-task), `milestone discard` (RD-M) · `milestone finalize` squash arm (RD-M). Cleared → none at `milestone add-task` (RD-M), `task amend` over a trailer-less `HEAD` (RD-F2 arm B). **Not driven:** `milestone add-from-spec`, `migrate-corpus` on a real migration, `milestone finalize (squash: false)` → R.7 L-1. The universal *"no bypassing site"* is a source statement; ten doors is what a drive supports |
| S-3 | the M55 surfaces are registry-backed (`doc show` keys from `WHOLE_DOC_KEYS` incl. `title`; `task finalize` coverage and amend wording; the committing-door clauses) | **CONFIRMED** | RD-H: the seven keys in registry order in `doc show --help`; RD-D: the emitted object's key set is those seven (+ `staged` on a `--task` read); the coverage sentence is byte-identical in `task finalize --help` and the `what's-left:` line of a composed `single-task`; all 11 `commits` clauses are in their door's help |
| S-4 | `report-inconsistency` is router-visible with a `when:` hint, and its description agrees with its create-only gate and doc-only finalize | **CONFIRMED** | RD-F1: in bare `jigc start` and in the router text with its hint; the three sibling workflows absent; driven to its commit — one file, a pre-task staged path still staged. The `create.already-exists` half of the create-only gate is carried by the driver's own block (2.8, row 4.7); the reconciler did not re-drive it |
| S-5 | the guide seam embeds the two crate-local guides and no root copies; strips repository-relative links; stamps; refuses to overwrite a user-modified body | **CONFIRMED** | RD-B: 409 lines, `jigc-version: 1.0.0-rc.24` + `jigc-body-blake3:`, 0 markdown links, 0 URLs; an appended line → `adapter-guide.user-modified`, file byte-identical after, `HEAD` unmoved, `guide_file: null`. RD-R: no `QUICKSTART.md` / `MIGRATING.md` at the tag's root |
| S-6 | `crates/cli/README.md` is byte-identical to `dev/crate-readme --stdout`; Cargo names it | **CONFIRMED** (file) | RD-R: `cmp` exit 0, tree unchanged; `readme = "README.md"`. The **packaged** bytes are row 1's (driver ND-3) and the rc.24 crates.io page is unread by anyone in this run (ND-2) |
| S-7 | the install line occurs only in QUICKSTART, the root README and the crate README | **CONFIRMED** | RD-R: one `^cargo install` line in each, equal hashes; RD-H: no help text prints it; RD-B: the installed guide's copy hashes the same |
| S-8a | MIGRATING correctly identifies the orphaned `doc-code`; no second-binary requirement | **CONFIRMED** | RD-S: a sentinel `doc-code` beside a `cmp`-identical copy of the binary; `setup`, a code-anchor `task finalize`, `validate` (clean, then `doc-code.symbol-exists`), `uninstall` all run from the copy — the sentinel never ran and its bytes are unchanged |
| S-8b | no surviving clone-based install instruction | **CONFIRMED** | RD-B: 0 hits for `git clone`, `cargo build --release`, `install -m755`, `cargo install --path` in the installed guide (= `CX-3`) |
| S-8c | no false `--no-verify` claim | **CONFIRMED** | RD-S: with a rejecting `commit-msg` hook in place, `jigc setup` lands its install commit (exit 0) and `jigc task finalize` is refused (exit 1, hook stderr verbatim, `HEAD` unmoved, the *is intact* frame) — both halves of gate 5's sentence |
| S-8d | no false **dry-run** claim in the scoped guides | **REFUTED** | two guide sentences about `--dry-run` do not hold when driven: MIGRATING gate 4 *"`jigc task validate <id>` and `jigc task finalize <id> --dry-run` run the same probe and refuse the same states"* — on a doc-only task with a pre-task staged path both exit **0** (RD-F1, = `(R9, F1)`); MIGRATING gate 1 *"It prints the manifest the finalize would commit"* — a migration forecast names two `modified` paths the commit does not carry (RD-F4, = `(R9, F4)`) |
| S-8e | no false migration-key claim | **CONFIRMED** | RD-G1: `migrate-corpus --format json` emits exactly the eight keys the guide lists |
| S-8f | no false manifest-vocabulary claim | **CONFIRMED for six tags · OPEN LEAD for one** | observed: `promoted`, `added`, `carried-over`, `untracked`, `left-staged` (RD-F1), `modified` (RD-M, on the milestone record — and RD-F4, where it is the false one). `deleted` was reached by no drive → R.7 L-2 |

### *Schema boundary* — `lead(codex, M55 adds exactly two methodology manifest rows at schema-version 1; no other hash or version moves)` → **CONFIRMED**

RD-R: the dev manifest is byte-identical from the commit before M54's pack move to the rc.24 tag;
the methodology manifest differs by exactly the `inconsistency` and `jigc-feedback` rows, both
`schema-version: 1`; rc.23 → rc.24 touches one pack file (`planning-record.yaml`, one line) and
no manifest. RD-D: `jigc doc schema` reports `schema-version: 1` for all three, and pack-load ran
clean at every one of the reconciler's drives. Row 4 owns this subject; recorded here because the
source pass stated it on this row.

### The source pass's own bound

It drove no binary. Nothing it stated was promoted on the read: each row above names the drive.

## R.3b The driver's observation leads

| lead | status | datum |
|---|---|---|
| O-6 — the composed `plan` step prints `spec:<slug>#derived-from` where `jigc doc schema spec` prints `spec:<slug>#meta/derived-from` (*"I did not run either form"*) | **REFUTED as a defect** | RD-O6: both spellings exit 0 and write the same front-matter field (`derived-from: prd:one`, then `prd:two`). Two accepted spellings of one address, not a dead end |
| O-1 … O-5 | driven by the driver, graded by it as observations | not re-driven as findings; O-2 is confirmed by S-1 above |

## R.4 The driver's defects — re-driven once each, and the tiers

| key | re-drive | reproduces | tier | door | the source pass on it |
|---|---|---|---|---|---|
| `(R9, F1)` | RD-F1 | **yes** | **3** | `task finalize` | silent on the help; its *"no false dry-run claim in the guides"* is refuted by this repro (S-8d) |
| `(R9, F2)` | RD-F2 | **yes** — and the fourth arm the driver left undriven (set · `HEAD` carried it) lands one trailer | **3** | `task amend` | concurs (its Claim 1 is the same seam on another surface) |
| `(R9, F3)` | RD-F3 | **yes** | **3** | `uninstall` | silent |
| `(R9, F4)` | RD-F4 | **yes** | **3** | `task finalize` | silent on the help; refuted in part as for F1 (S-8d) |
| `(R9, F5)` | RD-F5, RD-F3 | **yes** | **1 (proposed)** | `uninstall` | silent — the pass read `uninstall`'s help for `(8, N-1)` and did not reach the guard's subject |

No driver defect failed to reproduce; none is demoted to a lead.

### The tier-1 adjudication, both directions

The predicate: *tier 1 = exit-0 loss or repository harm through a committing, destroying or moving
door.* A tier-1 claim needs **both** halves — exit 0, and the loss or harm shown.

**`(R9, F5)` — tier 1 on the predicate's letter. Both halves are shown.**

* *Exit 0, through a destroying door, without its consent flag:* `jigc uninstall` (a
  `DESTROYING_DOORS` member), no `--force`, exit 0, ack *"removed .jigc/"* (RD-F5).
* *Loss, with a before-control:* `command grep -rl MARKER-RC9 .jigc` finds the three planted
  files before; after, the marker is nowhere in the repository, `git stash list` is empty, and the
  only narration names the five tracked install files. The door's own control proves the guard is
  alive: the same run refuses over `.jigc/config/notes.txt` and names **only** it.
* *The member that is not a plant:* with the opt-in `invocation-log` on, three records were in
  `.jigc/logs/invocations.jsonl` before; after an exit-0 `jigc uninstall` the file holds one
  line, the uninstall's own (RD-F3). The log is jigc-written, gitignored, sole-copy and not
  rebuildable.

**What argues for tier 3, stated so the ruling is not made by omission.** The guard matches its
*design*: `design/project-setup.md` and the M50 decision define the third guard's subject as the
files **outside** the transient `ENTRIES` prefixes, so `logs/`, `state/` and `index/` are outside
it by construction, and two of the three are rebuildable caches. On that reading the defect is
the help's universal (*"any other file under `.jigc/` that no index has a copy of … blocks"*),
which is tier 3, plus an un-narrated deletion.

**Why the reconciler still proposes tier 1.** (i) The predicate asks whether bytes died at exit 0
through a destroying door, not whether a design sentence anticipated it; both halves are driven.
(ii) The project's own precedent tiers this shape as loss: the same door destroying a plain file
planted **at** an `ENTRIES` name was counted among the exit-0 loss defects and fixed (M52 baseline
L-2), as was a plain file under `.jigc/worktrees/` (M49) and the parking home (M52) — each a
synthetic plant one prefix over. (iii) No record declares these three prefixes a bound; the guard's
own doc-comment says `displaced/` was *"the one excluded prefix no door owned"*, and the drive
shows three more. (iv) The invocation log is a real, adopter-opted, non-rebuildable population.
**The ruling between tier 1 and tier 3 is the human's**; nothing found by either pass makes it.
It sits **outside the range** (M54, M55 and the trailer did not touch this guard) — which under
the exit rule is the *post-review fix* case, not a reason to lower the tier.

**The other rows, checked for a hidden tier 1:**

| key | exit 0 | loss / harm | verdict |
|---|---|---|---|
| `(R9, F1)` | yes | **none** — the pre-task staged path is still staged before and after; `--carry-staged` lands nothing and takes nothing | tier 3: the loss half is missing |
| `(R9, F2)` / `(R9, C-1)` | yes, committing door | **none** — tree hash unchanged, superseded commit in the reflog; the trailer is added or kept, never dropped. A reader following the step to *drop* a co-author line gets it back, and the authored spelling `Claude Opus` is respelled `Claude` — both are the declared design, neither is a damaged repository | tier 3: the harm half is missing |
| `(R9, F3)` | yes, destroying door | residue, not loss: `.jigc/logs/invocations.jsonl` is re-created and reads `?? .jigc/logs/` because `.jigc/.gitignore` went with the install; nothing of the user's is touched. (The records lost in the same run are F5's loss, counted there once) | tier 3 |
| `(R9, F4)` | yes | **none** — the forecast over-reports; the landed commit is correct | tier 3 |
| `(8, N-1)` | yes, destroying door | bytes die (`my analysis`, before-control in RD-N1) — **but under `--force`**, the door's consent flag, after a refusal that named the file, with a narration that names it again. Consented and narrated destruction is not the predicate's loss | tier 3: the loss half is missing |
| `(8, N-2)` | yes, committing door | **none** — the commit is the one asked for; its subject and ack omit the title | tier 3 |

Nothing the driver tiered lower shows both halves.

**Tier 2 check.** No confirmed row is a posture or route dead end: every route printed in a
reconciler drive ran or was read as runnable (`write.identity-change`'s route, verbatim, in RD-N2;
`store.not-found`'s `--task` route in RD-G2; the `finalize.carried-staged` routes in RD-F1 and
RD-M).

## R.5 The reconciler's repro blocks

`$REPO` is the rig's repository, `$RIG` its root, `<tmp>` a `mktemp -d` directory. Output is
trimmed to the asserted lines; `EXIT=` is the driven process's own status, captured unpiped.

### RD-F5 — `(R9, F5)`

```
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$ jigc validate >/dev/null ; mkdir -p .jigc/logs .jigc/state .jigc/index
$ printf 'MARKER-RC9 logs my own notes\n'  > .jigc/logs/notes.txt
$ printf 'MARKER-RC9 state my own notes\n' > .jigc/state/notes.txt
$ printf 'MARKER-RC9 index my own notes\n' > .jigc/index/notes.txt
$ printf 'MARKER-RC9 control\n'            > .jigc/config/notes.txt
$ command grep -rl "MARKER-RC9" .jigc | sort          # before-control
.jigc/config/notes.txt
.jigc/index/notes.txt
.jigc/logs/notes.txt
.jigc/state/notes.txt
$ git status --short --ignored
?? .jigc/config/notes.txt
!! .jigc/index/
!! .jigc/logs/
!! .jigc/state/
$ jigc uninstall
blocking · uninstall.untracked-workbench-file — `.jigc/` holds 1 file(s) that no index has a copy of — removing `.jigc/` would destroy them:
  .jigc/config/notes.txt
EXIT=1
$ mv .jigc/config/notes.txt "$RIG/control-notes.txt"
$ command grep -rl "MARKER-RC9" .jigc | sort          # before-control, second
.jigc/index/notes.txt
.jigc/logs/notes.txt
.jigc/state/notes.txt
$ jigc uninstall                                       # no --force
warning: removing `.jigc/` also removes 5 tracked file(s) under it:
    .jigc/.gitignore  .jigc/AGENT.md  .jigc/config/.gitkeep  .jigc/config/packs.yaml  .jigc/version
jigc uninstall — repo-local install removed
  - removed .jigc/   … (six further lines)
EXIT=0
$ command grep -rl "MARKER-RC9" . | sort              → (nothing)
$ ls -a                                               → . .. .claude .git README.md
$ git stash list | wc -l                              → 0
$ git rev-parse HEAD                                  → unchanged
```

### RD-F3 — `(R9, F3)`, row 1.17, and F5's invocation-log member

```
# control, knob off: the RD-F5 rig, already uninstalled once
$ jigc uninstall  → "(nothing to remove — no repo-local jigc install was present)"   EXIT=0

rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$ jigc config get invocation-log        → invocation-log = false  (pack-default)      EXIT=0
$ jigc config set invocation-log true   → config: set `invocation-log` = `true` — written to `.jigc/config/`, uncommitted …   EXIT=0
$ git add .jigc/config ; git commit -q -m "chore: turn the invocation log on"
$ jigc validate   → no findings …   EXIT=0
$ jigc doc list   → jigc doc list — no committed docs   EXIT=0
$ wc -l < .jigc/logs/invocations.jsonl  → 3        # before-control: argv ["validate","--format","json"] (the pre-commit hook), ["validate"], ["doc","list"]
                                                    # each record: argv, binary_version, duration_ms, error_code, exit_code, finding_codes, output_bytes, timestamp
$ git check-ignore -v .jigc/logs/invocations.jsonl  → .jigc/.gitignore:6:logs/
$ jigc uninstall                                    # no --force
warning: removing `.jigc/` also removes 6 tracked file(s) under it: …      # the log is not named
  - removed .jigc/   … (six further lines)
EXIT=0
$ find .jigc -type f                    → .jigc/logs/invocations.jsonl
$ wc -l < .jigc/logs/invocations.jsonl  → 1        # argv ["uninstall"] — the three earlier records are gone
$ git status --short --ignored          → … ?? .jigc/logs/
$ jigc uninstall
  - removed .jigc/
EXIT=0                                              # the second run acted
$ find .jigc -type f                    → find: .jigc: No such file or directory
$ jigc uninstall  → "(nothing to remove …)"   EXIT=0        # the third run is the no-op
```

### RD-F1 — `(R9, F1)`, cell 4, the ordinary-model control, row 7.4

```
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$ jigc start                            → 13 workflows, among them
  - report-inconsistency — code and a doc, or two docs, disagree and the disagreement is worth a record until it is reconciled
                                          (report-jigc-feedback, triage-inconsistency, triage-jigc-feedback absent)   EXIT=0
$ jigc start "the README says the port default is 8080 but the code says 9090"
  … the same 13 rows … / jigc start --workflow <chosen> "<intent>"     EXIT=0 ; .jigc/tasks empty
$ printf 'port = 9090\n' > server.conf ; git add server.conf ; printf 'scratch\n' > scratch.txt     # BEFORE the mint
$ jigc start --workflow report-inconsistency "the README says the port default is 8080 but the code says 9090"
task minted: readme-says-the-port
… so the carryover gate does not fire here and `--carry-staged` changes nothing. …
create-gates: inconsistency
EXIT=0
$ jigc doc create inconsistency --title "Port default disagrees" --task readme-says-the-port   → inconsistency:port-default-disagrees   EXIT=0
$ jigc doc set-field inconsistency:port-default-disagrees#meta/kind --value code-doc --task readme-says-the-port          EXIT=0
$ jigc doc add-item inconsistency:port-default-disagrees#sides --title README.md --slug readme --task readme-says-the-port   EXIT=0
$ jigc doc add-item inconsistency:port-default-disagrees#sides --title server.conf --slug conf --task readme-says-the-port   EXIT=0
$ … jigc doc set-slot …#sides/readme/says · #sides/conf/says · #description · #evidence   (--from-file -)                 EXIT=0 each
$ jigc doc set-field commit:readme-says-the-port#type --value docs · #scope --value findings ; set-slot #summary · #body  EXIT=0 each
$ jigc task validate readme-says-the-port               → advisory · file-state.staged-copy only   EXIT=0
$ git status --short                                    → A  server.conf / ?? scratch.txt
$ jigc task finalize readme-says-the-port --dry-run     # an undeclared pre-task staged path is present
would commit — docs(findings): record the port default disagreement
  promoted docs/inconsistencies/port-default-disagrees.md
  left-out (this commit takes only this task's docs and the artifacts they record — a staged path stays staged for the task it belongs to):
    server.conf
    scratch.txt
EXIT=0
$ jigc task finalize readme-says-the-port --dry-run --format json
  "left_out": [{"kind":"left-staged","path":"server.conf"},{"kind":"untracked","path":"scratch.txt"}], "manifest": [{"kind":"promoted", …}]   EXIT=0
$ jigc task finalize readme-says-the-port --carry-staged
finalized 546eef1 — docs(findings): record the port default disagreement
  promoted docs/inconsistencies/port-default-disagrees.md
  1 file committed
EXIT=0
$ git status --short                                    → A  server.conf / ?? scratch.txt        # --carry-staged landed nothing
$ git show --stat --format='%s [%(trailers:only)]' HEAD → 1 file ; [Co-Authored-By: Claude <noreply@anthropic.com>]

# the control: the ordinary commit model, same rig, same staged path
$ jigc start --workflow single-task "add a rate limiter"   → task minted: add-a-rate-limiter ; the step prints
    jigc doc add-item commit:add-a-rate-limiter#trailers --title Co-Authored-By --task add-a-rate-limiter
    jigc doc set-field commit:add-a-rate-limiter#trailers/<id>/value --value "Name <email>" --task add-a-rate-limiter
$ mkdir -p src ; printf 'pub fn limit() {}\n' > src/limiter.rs ; git add src/limiter.rs
$ jigc doc set-field commit:add-a-rate-limiter#type --value feat … ; set-slot #summary ; add-item #trailers --title Co-Authored-By
$ jigc doc set-field commit:add-a-rate-limiter#trailers/co-authored-by/value --value "Claude Opus <noreply@anthropic.com>" --task add-a-rate-limiter   EXIT=0
$ jigc task validate add-a-rate-limiter
blocking · finalize.carried-staged — `server.conf` was already staged before this task existed …
  route: unstage it (`git -C $REPO restore --staged -- server.conf`) … or pass `--carry-staged` …
EXIT=3
$ jigc task finalize add-a-rate-limiter --dry-run                   → the same finding   EXIT=3
$ jigc task finalize add-a-rate-limiter --dry-run --carry-staged    → carried-over server.conf / added src/limiter.rs   EXIT=0
$ printf 'my analysis\n' > .jigc/tasks/add-a-rate-limiter/notes.txt
$ jigc task finalize add-a-rate-limiter --carry-staged --format json
note: the working area held 1 entry jigc did not write … moved aside, not taken:
    .jigc/tasks/add-a-rate-limiter/notes.txt → .jigc/displaced/add-a-rate-limiter/notes.txt
  "committed": {"displaced": [{from, to}], "files": 2, "left_out": [{"kind":"untracked","path":"scratch.txt"}],
                "manifest": [{"kind":"carried-over","path":"server.conf"},{"kind":"added","path":"src/limiter.rs"}], …}
EXIT=0
$ git log -1 --format='%(trailers:only)'   → Co-Authored-By: Claude Opus <noreply@anthropic.com>      # exactly one, the authored spelling
```

The contradicted sentences, read from the installed binary and the installed guide:
`task finalize --help` — *"it takes its **second commit model**"*; `--carry-staged` *"land index
entries staged before this task existed instead of refusing … Inert when nothing is carried, and
inert on an **amend** task in every state"*; `--dry-run` *"the carryover gate, where an undeclared
carry-over is reported (exit 3) instead of the manifest"*. Guide — *"A change staged before the
task existed refuses to ride the commit — one blocking `finalize.carried-staged` per carried path
— unless you declare it with `--carry-staged`"*; MIGRATING gate 4 — *"at finalize anything staged
before the task existed refuses"* and *"`jigc task validate <id>` and `jigc task finalize <id>
--dry-run` run the same probe and refuse the same states"*.

### RD-F2 — `(R9, F2)` and `(R9, C-1)`: four arms, zero trailer items authored in each

```
# the RD-F1 rig; each arm: `jigc task amend "<intent>"`, set #type + #summary only,
# `jigc doc show commit:<id> --task <id> | tail -3` → "## Trailers" and nothing under it, `jigc task finalize <id>`
# every mint prints: "Trailers are re-authored too — the amended message carries only the trailer items / you add here, …"

ARM A · env -u CLAUDECODE · HEAD: feat: add a token bucket rate limiter | [Co-Authored-By: Claude Opus <noreply@anthropic.com>]
  amended 11f9c7d → e11e29a   EXIT=0   → trailers [Co-Authored-By: Claude <noreply@anthropic.com>]     # the sentence is false
ARM D · CLAUDECODE set     · HEAD carried [Co-Authored-By: Claude <noreply@anthropic.com>]
  amended e11e29a → 14cb1f8   EXIT=0   → trailers [Co-Authored-By: Claude <noreply@anthropic.com>]     # false; exactly one (the arm the driver did not drive)
$ git add scratch.txt ; env -u CLAUDECODE git commit -q -m "chore: add the scratch file"               # a trailer-less HEAD
ARM B · env -u CLAUDECODE · HEAD: trailers []
  amended e1f402e → 804ded7   EXIT=0   → trailers []                                                   # true
ARM C · CLAUDECODE set     · HEAD: trailers []
  amended 804ded7 → a8f0a1e   EXIT=0   → trailers [Co-Authored-By: Claude <noreply@anthropic.com>]     # false
# every arm: tree hash unchanged; ack "the tree and the author are unchanged; the message is re-authored …"

$ jigc task amend --help
  … HEAD's message is not read back into the doc: you author the new message from scratch. …          # (R9, C-1)
```

### RD-N1 — baseline `(8, N-1)`, row 1.38

```
# the RD-F1 rig: no open task, one worktree, .jigc/displaced/add-a-rate-limiter/notes.txt parked by the finalize above
$ git status --short --ignored   → !! .jigc/displaced/  !! .jigc/index/  !! .jigc/state/
$ jigc task list                 → no active tasks    EXIT=0 ;  git worktree list | wc -l → 1
$ jigc uninstall
blocking · uninstall.foreign-bytes — `.jigc/` holds 1 path(s) jigc did not write …
  .jigc/displaced/add-a-rate-limiter/notes.txt
EXIT=1
$ cat .jigc/displaced/add-a-rate-limiter/notes.txt   → my analysis          # before-control
$ jigc uninstall --force
warning: removing the relocation workbench .jigc/displaced discards work that is not in git:
    .jigc/displaced/add-a-rate-limiter/notes.txt
  note: the relocation workbench is the only copy of these bytes — they are not recoverable.
EXIT=0                                               # not inert with the three enumerated guards clean
$ jigc uninstall --help | grep -c 'Four states it refuses'                         → 1
$ jigc uninstall --help | grep -c 'deletes all four'                               → 1
$ jigc uninstall --help | grep -c 'Inert when all three guards are already clean'  → 1
```

### RD-F4 — `(R9, F4)`, rows 1.32, 1.33

```
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$ mkdir -p docs/decisions ; printf '# Use PostgreSQL\n\nWe picked postgres because of reasons.\n' > docs/decisions/use-postgresql.md
$ git add docs ; git commit -q -m "docs: old adr"
$ jigc ingest
needs-reconcile docs/decisions/use-postgresql.md → adr
  blocking · conformance.section-missing — required section heading `## context` is missing
  route: `jigc migrate $REPO/docs/decisions/use-postgresql.md --as adr` …
EXIT=0
$ jigc migrate docs/decisions/use-postgresql.md --as adr   → task minted: migrate-adr-docs-decisions-use-postgresql-038c497bd8bf   EXIT=0
$ jigc doc author adr --from-file - --task <id> <<'PAY' … title + context / decision / consequences … PAY   → adr:use-postgresql   EXIT=0
$ git status --short -- .jigc/config .jigc/.gitignore | wc -l     → 0        # before-control
$ git diff --stat HEAD -- .jigc/config .jigc/.gitignore | wc -l   → 0
$ jigc task finalize <id> --dry-run
would commit — docs(adr): adopt docs/decisions/use-postgresql.md as a managed adr
  promoted docs/decisions/use-postgresql.md
  modified .jigc/config
  modified .jigc/.gitignore
EXIT=0
$ jigc task finalize <id>            → migration review required — nothing committed. …   EXIT=4 ; HEAD unmoved
$ jigc task finalize <id> --approve --format json
  committed.manifest: [{"kind":"promoted","path":"docs/decisions/use-postgresql.md"}]   files: 1   EXIT=0
$ git show --name-status --format='%s' HEAD
docs(adr): adopt docs/decisions/use-postgresql.md as a managed adr
M	docs/decisions/use-postgresql.md
```

### RD-N2 — baseline `(8, N-2)`

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$ jigc rename vision:vision --to "New Vision"
blocking · write.identity-change — cannot reslug `vision:vision` … only a retitle is supported
  route: `jigc rename vision:vision --to 'New Vision' --slug vision` …
EXIT=1
$ jigc rename vision:vision --to 'New Vision' --slug vision
renamed vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)
EXIT=0
$ grep -n '^# ' VISION.md      → 5:# New Vision          (was 5:# Vision)
$ git log -1 --format='%s | [%(trailers:only)]'   → rename VISION.md -> VISION.md | [Co-Authored-By: Claude <noreply@anthropic.com>]
$ jigc rename vision:vision --to 'New Vision' --slug vision
no-op: vision:vision already holds the title "New Vision" at VISION.md — nothing renamed, nothing committed
EXIT=0
$ jigc rename vision:vision --to 'Newer Vision' --slug vision --format json   → { …, "title": "Newer Vision", "commit": "27389d4", … }   EXIT=0
```

### RD-B — the installed guide (rows 1.1–1.4, 1.7, 1.37, 2.1; `CX-2`, `CX-3`; S-5, S-7, S-8b)

```
rig=$(dev/jigc-rig bare --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$ jigc setup
  - bootstrap reference → CLAUDE.md … - jigc allowlist … - SessionStart hook … - pre-commit hook → .git/hooks/pre-commit …
  - jigc guides → .claude/skills/jigc/SKILL.md …
  - install commit → 66eb53c …
EXIT=0
$ git log -1 --format='%s | [%(trailers:only)]'   → chore(jigc): install jigc workspace config | [Co-Authored-By: Claude <noreply@anthropic.com>]
$ git show --stat --format= HEAD | tail -1        → 8 files changed
$ head -5 .claude/skills/jigc/SKILL.md            → jigc-version: 1.0.0-rc.24 / jigc-body-blake3: 94afec4c…
$ wc -l < SKILL.md → 409 ; grep -c '\](' → 0 ; grep -c 'https\?://' → 0 ; grep -c 'crates/cli\|# cwd:' → 0
$ grep -c 'git clone\|cargo build --release\|install -m755\|cargo install --path' SKILL.md   → 0
$ grep '^cargo install' SKILL.md | shasum -a 256  → equal to the same line of README.md, crates/cli/README.md, crates/cli/guides/QUICKSTART.md
$ jigc setup                 → EXIT=0 ; HEAD unmoved
$ jigc setup --force         → EXIT=0 ; HEAD unmoved ; git status --short empty                     # row 1.7
$ printf '\nmy own note\n' >> .claude/skills/jigc/SKILL.md ; jigc setup
advisory · adapter-guide.user-modified — `.claude/skills/jigc/SKILL.md` no longer carries the bytes jigc wrote, so jigc left it untouched …
EXIT=0                       → file hash unchanged by the run ; HEAD unmoved
$ jigc setup --format json   → guide_file: null, install_commit: null, findings: [adapter-guide.user-modified]
$ jigc upgrade               → the same advisory ; EXIT=0
$ grep -ic 'co-author\|trailer\|authorship' SKILL.md   → 1   (the amend paragraph's `Refs: <value>` clause)
```

### RD-S — the orphaned `doc-code`, and both halves of the `--no-verify` sentence (rows 1.12, 1.13, 1.28; S-8a, S-8c)

```
rig=$(dev/jigc-rig bare --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$ B=$(mktemp -d "$RIG/bin.XXXXXX") ; cp ~/.local/bin/jigc "$B/jigc" ; cmp → identical
$ printf '#!/bin/sh\necho ran >> "<tmp>/doc-code.ran"\nexit 7\n' > "$B/doc-code" ; chmod +x "$B/doc-code"
$ printf '#!/bin/sh\necho "policy: subject must carry a ticket" >&2\nexit 1\n' > .git/hooks/commit-msg ; chmod +x …
$ "$B/jigc" setup                         → install commit → 3371612   EXIT=0 ; HEAD moved          # the install commit passes the rejecting hook
$ "$B/jigc" start --workflow record-decision "use a router"   → task minted: use-a-router
$ … src/router.rs committed with hooks off … ; "$B/jigc" doc author adr … cites-code: "src/router.rs#route" …   → adr:use-a-router   EXIT=0
$ "$B/jigc" task finalize use-a-router
`git commit` was rejected (no commit was made):
policy: subject must carry a ticket
task use-a-router is intact — nothing was committed, your task's staged docs are still in `.jigc/tasks/use-a-router/docs/`, … re-run `jigc task finalize use-a-router`.
EXIT=1 ; HEAD unmoved
$ mv .git/hooks/commit-msg "$RIG/commit-msg.off" ; "$B/jigc" task finalize use-a-router   → finalized db5b256 … promoted docs/decisions/use-a-router.md   EXIT=0
$ "$B/jigc" validate                      → no findings   EXIT=0
$ printf 'pub fn other() {}\n' > src/router.rs ; "$B/jigc" validate
blocking (gates at finalize) · doc-code.symbol-exists — anchor `src/router.rs#route` resolves to no symbol …
EXIT=0
$ "$B/jigc" uninstall                     → EXIT=0
$ ls "$B"                                 → doc-code  jigc        # no doc-code.ran: never executed ; sentinel hash unchanged
```

### RD-G1 — demoted rows 1.14, 1.34, 1.35, 4.5, 4.6 (the RD-F4 rig)

```
$ jigc upgrade                 → no findings — no recorded config deltas …, and the adapter's guide artifact … is still jigc's own   EXIT=0 ; git status empty
$ jigc migrate-corpus --dry-run
corpus migration (dry run — nothing written; the opt-in `invocation-log` knob's gitignored `.jigc/logs/invocations.jsonl` record is the one exception): 0 would migrate, 1 already current, 0 blocked
EXIT=0
$ jigc migrate-corpus --format json   → keys: already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled   EXIT=0 ; tree unchanged
$ jigc describe --workflows    → EXIT=0 ; 39 workflows, 26 carry "It is hidden from the router catalog"
    report-inconsistency: no hidden clause
    report-jigc-feedback: "It is hidden from the router catalog: invoked by name (`jigc start --workflow report-jigc-feedback "<what>"`) — …"
    triage-inconsistency, triage-jigc-feedback: "… invoked by name with the record|finding named in plain words … — triage is a maintainer's act, not a catalog entry."
$ jigc describe                → EXIT=0
$ jigc workflow report-jigc-feedback --preview   → preview: workflow `report-jigc-feedback` — no task minted. …   EXIT=0 ; .jigc/tasks empty
$ jigc start "add a rate limiter"                → the catalog + `jigc start --workflow <chosen> "<intent>"`    EXIT=0 ; .jigc/tasks empty
```

### RD-G2 — demoted rows 1.18, 1.19, 1.24, 1.36, 5.8, 5.9; partial rows 1.12, 5.4 (the RD-F4 rig)

```
$ jigc start --workflow single-task "add a rate limiter"   → task minted: add-a-rate-limiter
$ printf 'pub fn limit() {}\n' > src/limiter.rs ; git add src/limiter.rs
$ jigc doc author adr --from-file <payload: title + context/decision/consequences> --task add-a-rate-limiter   → adr:token-bucket-limiter   EXIT=0
$ jigc doc author adr --from-file <the same payload> --task add-a-rate-limiter                                 → adr:token-bucket-limiter   EXIT=0   # item-less: no refusal
$ jigc doc set-field adr:token-bucket-limiter#status/cites-code --value 'src/limiter.rs#limit' --task add-a-rate-limiter   EXIT=0
$ jigc doc schema adr | grep code-anchor
  - cites-code: code-anchor <repo-relative-path>[#<symbol>] (section: status) (set-field: adr:<slug>#status/cites-code)        EXIT=0
$ jigc doc list --task add-a-rate-limiter
adr:token-bucket-limiter  docs/decisions/token-bucket-limiter.md  managed
commit:add-a-rate-limiter  commit:add-a-rate-limiter  managed
EXIT=0
$ jigc doc list                → stdout: the committed row only ; stderr: note: docs are also staged in open task add-a-rate-limiter — … `jigc doc list --task add-a-rate-limiter`   EXIT=0
$ jigc doc list --format json  → rows keyed fields, id, item-count, path, state, title
$ jigc doc show adr:token-bucket-limiter --task add-a-rate-limiter   → the staged body (cites-code: src/limiter.rs#limit)   EXIT=0
$ jigc doc show adr:token-bucket-limiter
blocking · store.not-found — … route: … read it with `jigc doc show adr:token-bucket-limiter --task <task-id>` …
EXIT=1
$ jigc doc show adr:token-bucket-limiter --task add-a-rate-limiter --format json   → the seven keys + "staged": "add-a-rate-limiter"
$ jigc doc show adr:use-postgresql --format json                                   → fields, item-count, schema-version, sections, slug, title, type
$ jigc task diff add-a-rate-limiter   → diff --git a/src/limiter.rs … +pub fn limit() {} / "# staged docs" / adr:token-bucket-limiter.md   EXIT=0
$ … commit doc type + summary … ; jigc task finalize add-a-rate-limiter   → finalized a362fb4 — 2 files committed   EXIT=0
$ ls ~/.local/bin | grep -c doc-code   → 0
$ jigc validate                → no findings   EXIT=0
$ printf 'pub fn other() {}\n' > src/limiter.rs ; git add src/limiter.rs ; git commit -m "refactor: rename limit"
jigc: doc<->code drift detected in committed docs — run `jigc validate` for details (commit not blocked).
EXIT=0
$ jigc validate
blocking (gates at finalize) · doc-code.symbol-exists — anchor `src/limiter.rs#limit` resolves to no symbol …
1 finding(s) — report-only at store scope (exit 0); these gate at `jigc task validate` / `jigc task finalize` / `jigc milestone finalize`.
EXIT=0
```

### RD-G3 — demoted rows 1.26, 1.29; partial row 5.7 (the RD-F4 rig)

```
$ jigc start --workflow report-inconsistency "limiter doc and code disagree"   → task minted: limiter-doc-and-code-disagree
$ jigc doc author inconsistency --from-file <payload with one `sides` item> --task …   → inconsistency:limiter-mismatch   EXIT=0
$ jigc doc author inconsistency --from-file <the same payload> --task …
blocking · write.already-present — write rejected: item "src-limiter-rs" in section "sides" is already present
EXIT=1
$ jigc task discard limiter-doc-and-code-disagree
blocking · task-discard.staged-prose — task … stages 2 doc(s) that no commit has a copy of …
EXIT=1
$ jigc start --workflow quick-fix "throwaway" ; printf 'foreign bytes\n' > .jigc/tasks/throwaway/notes.txt
$ jigc task discard throwaway
blocking · task-discard.foreign-bytes — task `throwaway`'s working area holds 1 path(s) jigc did not write …
  .jigc/tasks/throwaway/notes.txt
EXIT=1
$ jigc task discard throwaway --force
warning: removing the working area .jigc/tasks/throwaway discards work that is not in git:
    .jigc/tasks/throwaway/notes.txt
discarded task throwaway — dropped staged edits to: commit:throwaway (transient)
EXIT=0
$ jigc task discard limiter-doc-and-code-disagree --force   → discarded task …   EXIT=0 ; HEAD unmoved through all four
$ jigc config set docs-root documentation
relocating 2 committed doc(s) stranded by the `docs-root` re-point to `documentation` … each move is a staged `git mv` …
EXIT=0
$ git status --short   → R  docs/decisions/… -> documentation/decisions/… (×2) / ?? .jigc/config/manifest.yaml
$ jigc doc list        → both rows at documentation/decisions/…   EXIT=0
$ jigc config set docs-root ""
config: set `docs-root` = `.` (you typed an empty value, which names the repository root `.` — a root knob has no unset spelling) …
EXIT=0
$ jigc config get docs-root   → docs-root = .  (project)   EXIT=0
```

### RD-M — the milestone doors, `unmanage`, and the trailer on each commit (rows 5.5, 5.13, 5.14, 1.27, 7.2, 7.3; S-2)

```
# the RD-N2 rig
$ jigc unmanage docs/decisions-log.md   → unmanaged docs/decisions-log.md (decisions-log:decisions-log) — dropped its file-state baseline + forward edges; the file is left on disk. …   EXIT=0
$ jigc unmanage docs/decisions-log.md   → no-op: docs/decisions-log.md is not managed (nothing to drop)   EXIT=0
$ printf 'staged by me\n' > mine.txt ; git add mine.txt
$ jigc milestone create "Ship limiter"                         → record commit: 6309f95   EXIT=0 ; trailers [Co-Authored-By: Claude <noreply@anthropic.com>]
$ jigc milestone add-task ship-limiter "first sub task"        → record commit: a51c861   EXIT=0 ; trailers [Co-Authored-By: Claude <noreply@anthropic.com>]
$ env -u CLAUDECODE jigc milestone add-task ship-limiter "second sub task"   → record commit: e3dd6d4   EXIT=0 ; trailers []
$ jigc milestone list-tasks ship-limiter                       → milestone:ship-limiter tasks (2): first-sub-task, second-sub-task   EXIT=0
$ jigc milestone execute ship-limiter                          → two `Spawn:` command lines   EXIT=0 ; HEAD unmoved
$ jigc milestone execute no-such-milestone                     → blocking · milestone.unknown …   EXIT=1
$ jigc task discard first-sub-task                             → record commit: e302d71 — … settled to `discarded` and committed on its own   EXIT=0 ; trailer present
$ jigc milestone discard ship-limiter                          → discarded milestone:ship-limiter (2 sub-task(s); workbench removed)   EXIT=0 ; trailer present
$ git status --short                                           → A  mine.txt        # through all five commits

$ jigc milestone create "Second wave" ; jigc milestone add-task second-wave "write the widget"
$ jigc milestone provision second-wave                         → provisioned 1 worktree(s) …   EXIT=0
$ (cd .jigc/worktrees/write-the-widget && jigc workflow sub-task --task write-the-widget ; widget.rs staged ; commit doc type + summary)   EXIT=0 each
$ jigc milestone join second-wave                              → joined milestone:second-wave — 1 doc(s) merged   EXIT=0
$ jigc milestone finalize second-wave
blocking · finalize.carried-staged — `mine.txt` was already staged before this milestone existed …
EXIT=3 ; HEAD unmoved
$ jigc milestone finalize second-wave --carry-staged
finalized f9d07b1 — Finalize milestone second-wave (1 sub-task)
  modified docs/milestone-records/second-wave.md / added widget.rs / 2 files committed
  staged in the shared checkout (not committed by this boundary …; these stay staged):  mine.txt
EXIT=0 ; trailers [Co-Authored-By: Claude <noreply@anthropic.com>]
$ jigc milestone finalize no-such-milestone                    → blocking · milestone.unknown …   EXIT=1
```

### RD-H — the help reads (rows 5.1, 5.15, 6.1–6.4, 7.1, 2.2; S-1, S-3). Help reads confer no door coverage

```
$ for each of the 48 VERB_KINDS leaves: jigc <leaf> --help   → 48 of 48 exit 0 ; jigc --help → exit 0
$ grep -l 'cargo install' <the 49 texts>                     → none
$ grep -il 'co-author\|trailer\|authorship' <the 49 texts>   → none
6.1  jigc validate --help        : the 7 STORE_FAMILIES names (doc↔code, workflow↔refs, file↔CLI-state, forward-ref integrity,
                                   schema-completeness, mention integrity, schema-conformance) each present, in registry order
6.2  jigc migrate-corpus --help  : the 18 SchemaChangeKind::ALL wire names, in ALL's order, as one comma-joined run
6.3  jigc doc show --help        : `type`, `slug`, `title`, `item-count`, `schema-version`, `fields`, `sections`  (+ `staged`)
6.4  jigc task finalize --help   : "previews part of the finalize gate: … the carryover gate, … at finalize" — byte-identical to the
                                   what's-left line of a composed single-task; the amend clause equals TASK_FINALIZE_AMEND_COMMITS
7.1  all 11 COMMITTING_DOORS `commits` clauses found (whitespace-normalized) in their door's --help
5.15 top-level one-liner == first paragraph of the verb's own help: setup, uninstall, unmanage, workflow — equal ;
     validate — equal up to clap's stripped final full stop
```

### RD-R — the repository reads (rows 3.1, 3.2, 2.2; S-5, S-6, S-7; the schema boundary). No door coverage

```
$ dev/crate-readme --stdout | cmp - crates/cli/README.md       → exit 0 ; git status --short empty after
$ links in crates/cli/README.md                                → 7 occurrences, 6 distinct, 0 relative ; every target
                                                                 exists at jigc-v1.0.0-rc.24 (git cat-file -e, exit 0 ×6)
$ git diff --stat jigc-v1.0.0-rc.23 jigc-v1.0.0-rc.24 -- README.md crates/cli/README.md crates/cli/guides/   → empty
$ grep '^cargo install' in README.md, crates/cli/README.md, crates/cli/guides/QUICKSTART.md   → one line each, equal SHA-256
$ git cat-file -e jigc-v1.0.0-rc.24:QUICKSTART.md ; … :MIGRATING.md   → both absent at the root
$ dev manifest, the commit before M54's pack move vs the tag    → 0 differing lines
$ methodology manifest, the same pair                           → + inconsistency (schema-version 1), + jigc-feedback (schema-version 1), nothing else
$ git diff --stat jigc-v1.0.0-rc.23 jigc-v1.0.0-rc.24 -- crates/cli/packs   → planning-record.yaml, 1 line ; no manifest
```

### RD-D — `doc show` slices and `doc schema` versions (rows 5.8, 6.3; the schema boundary)

```
# the RD-N2 rig
$ jigc doc show vision:vision --format json                               → key set: fields, item-count, schema-version, sections, slug, title, type   EXIT=0
$ jigc doc show 'roadmap:roadmap#milestones' --format json                → an item array (1 item: decomposition, id, proves, title)   EXIT=0
$ jigc doc show 'roadmap:roadmap#milestones/m-alpha/proves' --format json → a leaf string   EXIT=0
$ jigc doc schema inconsistency --format json    → contract-version 7, schema-version 1   EXIT=0
$ jigc doc schema jigc-feedback --format json    → contract-version 7, schema-version 1   EXIT=0
$ jigc doc schema planning-record --format json  → contract-version 7, schema-version 1   EXIT=0
```

### RD-O6 — the driver's lead O-6

```
# the RD-F4 rig
$ jigc start --workflow plan "plan the widget"   → the step prints: jigc doc set-field spec:<slug>#derived-from --value prd:<slug> --task plan-the-widget
$ jigc doc schema spec | grep derived-from        → - derived-from: ref -> prd (section: meta) (set-field: spec:<slug>#meta/derived-from)
$ jigc doc create spec --title "Widget spec" --task plan-the-widget                                  → spec:widget-spec   EXIT=0
$ jigc doc set-field 'spec:widget-spec#derived-from' --value prd:one --task plan-the-widget          EXIT=0 → front-matter derived-from: prd:one
$ jigc doc set-field 'spec:widget-spec#meta/derived-from' --value prd:two --task plan-the-widget     EXIT=0 → front-matter derived-from: prd:two
```

## R.6 Baseline rows — CLOSED / STILL-OPEN, reconciled

| key | rc.16 | driver | source pass | **reconciled on rc.24** | datum |
|---|---|---|---|---|---|
| `(8, N-1)` | tier 3, open | STILL-OPEN | CLOSED | **STILL-OPEN (expected, 1.x) · tier 3** — the source pass's CLOSED is refuted | RD-N1: the three enumerated guards clean, `--force` deletes the parked file at exit 0; the help still carries both numerals (*"deletes all four"* · *"all three guards"*) |
| `(8, N-2)` | tier 3, open | STILL-OPEN | STILL-OPEN | **STILL-OPEN (expected, 1.x) · tier 3** | RD-N2 |
| M51 `CX-1` | CLOSED | CLOSED | — | **CLOSED** | RD-F3: knob default `false (pack-default)`; on, one record per run; `migrate-corpus --dry-run` prints the exception (RD-G1) |
| M51 `CX-2` | CLOSED | CLOSED | consistent (S-5) | **CLOSED** | RD-B: edited guide left byte-identical, `adapter-guide.user-modified` at `setup` and `upgrade` |
| M51 `CX-3` | CLOSED by scoping to a clone | CLOSED on the rewritten sentence | consistent (S-8b) | **CLOSED on the rewritten sentence** | RD-B: one install command, the crates.io line, no path and no clone step. **Its execution is row 1's** — not run here |
| M51 `D-1` | CLOSED | NOT RE-DRIVEN | — | **NOT RE-DRIVEN** | no cell of this row reached it; nothing here says it holds on rc.24 |
| M51 `D-2` | CLOSED (10 rows) | CLOSED | consistent (S-3) | **CLOSED** | RD-H: all 11 clauses in their door's help; commits driven at 9 of the 11 rows (driver 2.7 + RD-M, RD-F1, RD-F2, RD-N2); not `migrate-corpus`, not `squash: false` |

### New on this row

| key | origin | tier | door | one line |
|---|---|---|---|---|
| `(R9, F5)` | driver | **1 (proposed; the tier-3 reading is stated in R.4)** | `uninstall` | without `--force`, destroys sole-copy files inside `.jigc/logs/`, `.jigc/state/`, `.jigc/index/` at exit 0, unnamed — the opt-in invocation log's records among them — where its help says every such file blocks |
| `(R9, F1)` | driver | 3 | `task finalize` | the help and two guides state the carryover gate and `--carry-staged` as universal over tasks; on a doc-only task neither fires |
| `(R9, F2)` | driver | 3 | `task amend` | the composed `amend` step says the amended message carries only the trailers you author; it carries the co-author trailer in three of four arms |
| `(R9, C-1)` | codex | 3 | `task amend` | `task amend --help` and the installed guide say the new message is authored from scratch; the landed message carries a line read from `HEAD` |
| `(R9, F3)` | driver | 3 | `uninstall` | with the invocation log on, `uninstall` leaves `.jigc/logs/` behind, un-ignored, and its second run acts |
| `(R9, F4)` | driver | 3 | `task finalize` | a migration task's `--dry-run` forecasts two `modified` paths the commit does not carry |

## R.7 Open leads carried out of this row

| # | lead | origin | why open |
|---|---|---|---|
| L-1 | the signing seam at `milestone add-from-spec`, at `migrate-corpus` on a real migration, and at `milestone finalize (squash: false)` | codex (S-2's universal) | not driven here: the first needs a landed spec (the driver's 2.7 drove the door, not its trailer); the second needs a corpus stamped at another schema-version, which this binary alone cannot build; the third is row 5's arm. Row 10 owns the matrix |
| L-2 | the manifest tag `deleted` | codex (S-8f) / driver ND-10 | no retirement occurred in any driven finalize |
| L-3 | the rc.24 crates.io page as rendered, and each README URL over HTTP | driver ND-2; the source pass bounds it the same way | a network observation nobody made in this run; rc.23's page is on record, rc.24's is not |
| L-4 | the eight sub-verb one-liners of row 5.15; a finding-free `task validate` (5.11); `validate` on an un-baselined store (5.4) | driver rows, demoted | no repro block from the driver, not re-driven by the reconciler |
| L-5 | M51 `D-1` on rc.24 | baseline | reached by no cell |
| L-6 | the ruling on `(R9, F5)`'s tier | — | the human's (R.4) |

The driver's ND-1 … ND-17 stand as written; none was promoted or dropped. ND-9 (`doc author`'s
*already existed — copied in for update* ack) is still undriven: RD-G2's second item-less author
printed only the address.

## R.8 Doors covered (reconciled)

A leaf is covered iff it is the door of ≥ 1 driven row carrying a repro block — the driver's (§2)
or the reconciler's (R.5). `--help` reads confer none. `VERB_KINDS` spelling.

**Covered — 37 of 48:**

- top level (12): `start` · `workflow` · `setup` · `uninstall` · `upgrade` · `ingest` · `migrate` ·
  `migrate-corpus` (the no-op arm only) · `unmanage` · `rename` · `describe` · `validate`
- `doc` (8): `doc create` · `doc add-item` · `doc set-field` · `doc set-slot` · `doc author` ·
  `doc show` · `doc schema` · `doc list`
- `task` (6): `task list` · `task diff` · `task validate` · `task amend` · `task discard` ·
  `task finalize`
- `config` (2): `config set` · `config get`
- `milestone` (9): `milestone create` · `milestone add-task` · `milestone add-from-spec` ·
  `milestone list-tasks` · `milestone provision` · `milestone execute` · `milestone join` ·
  `milestone finalize` · `milestone discard`

**Covered only by a reconciler block** (the driver's rows for them carried none): `ingest` ·
`migrate-corpus` · `describe` · `doc schema` · `task diff` · `config get` · `milestone list-tasks`.
**Covered only by a driver block:** `milestone add-from-spec` (2.7).

**Not covered — 11:** `relocate` · `doc remove-item` · `doc retitle-item` · `doc rename` ·
`task bind` · `config insert-step` · `config replace-step` · `config remove-step` · `config fill` ·
`config fork` · `config list` — the driver's ND-14, unchanged: their `--help` ran, the verbs did not.
