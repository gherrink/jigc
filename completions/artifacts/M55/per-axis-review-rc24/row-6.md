<!-- Reconciled ROW 6 file (write surface), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis6` in the body means ROW 6 of this run, not numbered axis 6. -->

# Row 6 · write surface — the driver's record (rc.24)

> **Reconciled 2026-10-04 against the source pass** (present, exit 0). The driver's record follows unchanged except eight row annotations marked **RECONCILER**; the reconciliation — ledger, the reconciler's repro blocks, open leads, doors covered — is the last part of this file. Outcome: all ten driver defects reproduce (D-1 tier 1 confirmed; D-7 re-tiered by the reconciler, with its weight stated); the source pass's one defect claim, the create-gate's check/use race, reproduces and is finding `(R6, K-1)`; no source claim was refuted; three leads stay open.


Partial per-axis re-review of the published `jigc 1.0.0-rc.24`. **Row 6 of this run** (the `new: true`
create-gate entry at both create doors, and the `write.title-ignored` route) — not the historical
numbered axis 6. First drive; no baseline. Driven 2026-10-03/04 on the installed registry build.

- **Binary:** `~/.local/bin/jigc`; `jigc --version` printed `jigc 1.0.0-rc.24` before the first drive and
  again inside the tier-1 repro's rig. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Source read at** `bffa6667` (`work/rc24-gate`; one `completions/trial-driver` commit past `aa6666cb`,
  nothing under `crates/`). Registries were read to derive the door set; **no row below is marked
  driven from a source read**.
- **Rigs:** seven, each `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig";
  [ -n "$REPO" ] || exit` — stdout only, two steps. `fresh` ×6 (A, B, D, E, F, G), `committed-singletons`
  ×1 (C). No teardown, no `rm`. Nothing was written into the gitignored workbench; the only
  hand-written files under `.jigc/` are the project-layer workflow shadows of cell 5
  (`.jigc/config/workflows/<id>.yaml`, committed with plain git), which the scope grants.
- **`CLAUDECODE` was set** throughout, so every commit jigc made carries the product trailer
  `Co-Authored-By: Claude <noreply@anthropic.com>`. Held constant; not a subject of this row.
- **The working repository was not touched** — `git status` clean and `HEAD` unmoved after the last drive.
- **Filesystem:** case-insensitive, case-preserving (APFS default). Every case cell below is a fact about
  this filesystem.
- **Docker:** `docker version` did not answer (exit 1), and no Linux build of the registry binary was at
  hand — the case-sensitive half is NOT DRIVEN.

## Result in one paragraph

**One tier-1 proposal, and it is on the fan-out join, not on the create doors.** Both create doors hold
the `new: true` contract in every cell driven: an id already on disk — committed, untracked, a symlink to
a file, copied in by another verb, whichever title or `--slug` reaches it — is refused
`create.already-exists` at exit 1 with nothing staged and the existing bytes unchanged, and the route
(split by where the id comes from) runs. **But the join's collision suffix is minted without looking at
the store:** when two sub-tasks mint one slug and `<slug>-2` (or `-3`) is already a doc on disk, `jigc
milestone join` suffixes onto that occupied id and `jigc milestone finalize` **overwrites it at exit 0** —
a committed finding (recoverable only from history) and an untracked file (recoverable from nowhere), with
`findings: []` and `displaced: []` in the envelope. It reproduces under `park-idea` as well, so it predates
M55; `new: true` does not close it, and M55's *one doc per finding* + *several reporters into one store* is
the configuration that reaches it. Besides that: two tier-2 route dead ends, seven tier-3 rows (two of
them design-doc sentences the binary does not do), and the declared edit-gate bound measured where it sits.

## The door set — derived from the code, beside the instrument's counts

| registry | file | the instrument's author read | I read | difference |
|---|---|---|---|---|
| `VERB_KINDS`, `doc` × `Write` | `crates/cli/src/cli.rs` | 8 leaves; 2 create doors | **8** — `create` · `add-item` · `remove-item` · `retitle-item` · `rename` · `set-field` · `set-slot` · `author`; create doors `create`, `author` (`VERB_KINDS` whole: 48 rows) | none |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs` | 16 | **16** (3 top-level — `migrate`, `relocate`, `rename` · 11 `doc` · `task bind` · `milestone add-from-spec`) | none |
| `SLUG_DOORS` | `crates/cli/src/cli.rs` | 6 | **6** — `start` · `migrate` · `rename` · `doc create` · `doc add-item` · `doc rename` | none |
| `MINT_DOORS` | `crates/engine/src/state.rs` | 6 | **6** | none |
| `allows-create` entries, both packs | `crates/cli/packs/{dev,methodology}/workflows/*.yaml` | 34 entries over 29 of 39 workflows; 2 carry `new: true` | **34 over 29 of 39; 2 with `new: true`** (`report-inconsistency`, `report-jigc-feedback`); 2 workflows carry an explicit empty list (`amend`, `quick-fix`) | none |
| `AllowsCreate` keys | `crates/engine/src/compose.rs` | 3, closed | **3** — `type`, `as`, `new`. **Datum:** `as` is a *required* field of the struct and the entry has **no bare-string form** — see D-5 | none in the count |
| the `create.*` family (grep) | `crates/cli/src`, `crates/engine/src` | 6 literals | **6 quoted literals — but only 5 are finding codes.** `create.singleton-copy-in` is a `const` identifier the stated-at fence checks for (`crates/cli/src/pack.rs`), by its own doc comment *not a minted `Finding` code*. A seventh hyphenated hit, `milestone-create.commit-rejected`, is another family. | **a datum:** 5 codes + 1 fence identifier |
| `PayloadReject::ALL` | `crates/cli/src/author.rs` | (count asked) | **4** — `Ungrammatical`, `MissingTitle` → `write.wrong-shape`; `BareSlotValue`, `WrappedFieldValue` → `write.malformed-value` | — |

**Doors of this row, as driven:** the two create doors (`doc create`, `doc author`) across the whole
matrix; the six other `doc` × `Write` leaves where cell 6 and the routes send them; and the doors the
routes, the fan-out and the entry shadows land on (`task finalize`, `task validate`, `task discard`,
`start`, `workflow`, `describe`, `validate`, `config get`, `config list`, the six `milestone` leaves
driven, `doc show`, `doc list`).

## How to read the tables

Row schema, per the instrument: `(door, cell) → {argv driven, exit, code|none, route kind, surface
asserted, verdict}`. A row is here only if its argv ran on the installed binary. `<t>` is the cell's own
freshly minted task id (each matrix cell runs in its own task, discarded with `--force` afterwards).
**Route kind:** *Mechanical* — the route carries a runnable `jigc …` line; *Human* — prose only; *none
(ack)* — a success ack. **Nothing staged** was read with `ls` of `$REPO/.jigc/tasks/<t>/docs/` before and
after each argv (the before-control lists the task's `commit:<t>.md` and `provenance.json`, so the listing
demonstrably sees that directory); the committed doc's checksum (`shasum -a 256`, first 16 hex) was taken
before and after each block. Cell labels: `N-` an entry carrying `new: true`, `G-` the general case (an
entry without it), `R-` a role already bound, `S` a project shadow; `c` = `doc create`, `a` = `doc
author`; identity letters `A` absent · `C` committed · `U` untracked file at the home · `S` staged by this
same task · `V` copied in by this task through another verb · `O` staged in another open task · `F` a
foreign (non-conformant) untracked file · `D`/`L` a directory / a symlink at the home · `K` case.

## Table 1 · the matrix — entry × door × identity × title (rigs B and D; one fresh task per cell)

Rows follow the drive order: `new: true` × absent / committed / untracked / own-staged; copied-in; the
`report-jigc-feedback` entry; the gate and empty-title cells; staged in another open task; the general
case (`park-idea`) over the same identities; foreign files, a directory and symlinks at the home; a role
already bound; the two principal shadows (`S1-*` the key dropped, `S2-*` the key added); the general
case staged in another open task; case. Repro: RB-1.

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| C001 | `doc create` | N-A-c1 · report-inconsistency · absent · title | `jigc doc create inconsistency --title "Brand new alpha" --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:brand-new-alpha · `existed` false · `copied_in` false | matches contract |
| C002 | `doc create` | N-A-c3 · report-inconsistency · absent · --slug distinct | `jigc doc create inconsistency --title "Brand new alpha" --slug alpha-by-slug --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:alpha-by-slug · `existed` false · `copied_in` false | matches contract |
| C003 | `doc author` | N-A-a1 · report-inconsistency · absent · title | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Brand new beta" + a description slot)` | 0 | none | none (ack) | ack `target` inconsistency:brand-new-beta · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C004 | `doc create` | N-C-c1 · report-inconsistency · committed · same title | `jigc doc create inconsistency --title "Port mismatch" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C005 | `doc create` | N-C-c2 · report-inconsistency · committed · different title, same id | `jigc doc create inconsistency --title "the PORT mismatch!" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C006 | `doc create` | N-C-c3 · report-inconsistency · committed · --slug distinct | `jigc doc create inconsistency --title "Port mismatch" --slug port-mismatch-two --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:port-mismatch-two · `existed` false · `copied_in` false | matches contract |
| C007 | `doc create` | N-C-c4 · report-inconsistency · committed · --slug occupied | `jigc doc create inconsistency --title "Unrelated title" --slug port-mismatch --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C008 | `doc author` | N-C-a1 · report-inconsistency · committed · same title | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Port mismatch" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C009 | `doc author` | N-C-a2 · report-inconsistency · committed · different title, same id | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Port, Mismatch?" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C010 | `doc create` | N-U-c1 · report-inconsistency · untracked · same title | `jigc doc create inconsistency --title "Untracked thing" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:untracked-thing)` · nothing staged | matches contract |
| C011 | `doc create` | N-U-c2 · report-inconsistency · untracked · different title, same id | `jigc doc create inconsistency --title "Untracked Thing!" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:untracked-thing)` · nothing staged | matches contract |
| C012 | `doc create` | N-U-c3 · report-inconsistency · untracked · --slug distinct | `jigc doc create inconsistency --title "Untracked thing" --slug untracked-thing-beside --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:untracked-thing-beside · `existed` false · `copied_in` false | matches contract |
| C013 | `doc create` | N-U-c4 · report-inconsistency · untracked · --slug occupied | `jigc doc create inconsistency --title "Other words" --slug untracked-thing --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:untracked-thing)` · nothing staged | matches contract |
| C014 | `doc author` | N-U-a1 · report-inconsistency · untracked · same title | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Untracked thing" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:untracked-thing)` · nothing staged | matches contract |
| C015 | `doc author` | N-U-a2 · report-inconsistency · untracked · different title, same id | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "untracked, thing" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:untracked-thing)` · nothing staged | matches contract |
| C016 | `doc create` | N-S-c1 · report-inconsistency · own staged · same title (re-run) | `jigc doc create inconsistency --title "Own thing" --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:own-thing · `existed` true · `copied_in` false | matches contract |
| C017 | `doc create` | N-S-c2 · report-inconsistency · own staged · different title, same id | `jigc doc create inconsistency --title "Own Thing!" --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, inconsistency:own-thing)` · nothing staged | matches write-commands.md (staged-doc arm) · contradicts findings-channel §10 → D-4 |
| C018 | `doc create` | N-S-c3 · report-inconsistency · own staged · --slug distinct | `jigc doc create inconsistency --title "Own thing" --slug own-thing-other --task <t> --format json` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, inconsistency:own-thing)` · nothing staged | matches contract |
| C019 | `doc create` | N-S-c4 · report-inconsistency · own staged · --slug own id, other title | `jigc doc create inconsistency --title "Different words" --slug own-thing --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, inconsistency:own-thing)` · nothing staged | matches write-commands.md · same as N-S-c2 → D-4 |
| C020 | `doc create` | N-S-c4b · report-inconsistency · own staged · --slug own id, same title | `jigc doc create inconsistency --title "Own thing" --slug own-thing --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:own-thing · `existed` true · `copied_in` false | matches contract |
| C021 | `doc author` | N-S-a1 · report-inconsistency · own staged · same title | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Own thing" + a description slot)` | 0 | none | none (ack) | ack `target` inconsistency:own-thing · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C022 | `doc author` | N-S-a2 · report-inconsistency · own staged · different title, same id | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "OWN thing?" + a description slot)` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, inconsistency:own-thing)` · nothing staged | matches write-commands.md · same as N-S-c2 → D-4 |
| C023 | `doc author` | N-S-a3 · report-inconsistency · own staged · different title, different id | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Wholly other" + a description slot)` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, inconsistency:own-thing)` · nothing staged | matches contract |
| C024 | `doc create` | N-V-c1 · report-inconsistency · copied-in · same title | `jigc doc create inconsistency --title "Port mismatch" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C025 | `doc create` | N-V-c2 · report-inconsistency · copied-in · different title, same id | `jigc doc create inconsistency --title "Port Mismatch!!" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C026 | `doc create` | N-V-c3 · report-inconsistency · copied-in · --slug distinct | `jigc doc create inconsistency --title "Port mismatch" --slug port-mismatch-v --task <t> --format json` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, inconsistency:port-mismatch)` · nothing staged | DEFECT → D-2 (the route refuses when followed) |
| C027 | `doc create` | N-V-c4 · report-inconsistency · copied-in · --slug occupied | `jigc doc create inconsistency --title "Else" --slug port-mismatch --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C028 | `doc author` | N-V-a1 · report-inconsistency · copied-in · same title | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Port mismatch" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C029 | `doc author` | N-V-a2 · report-inconsistency · copied-in · different title, same id | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "port: mismatch" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C030 | `doc create` | N-C-c1/fb · report-jigc-feedback · committed · same title | `jigc doc create jigc-feedback --title "Finalize sweeps a staged path" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, jigc-feedback:finalize-sweeps-a-staged-path)` · nothing staged | matches contract |
| C031 | `doc create` | N-C-c2/fb · report-jigc-feedback · committed · different title | `jigc doc create jigc-feedback --title "Finalize sweeps a staged path!" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, jigc-feedback:finalize-sweeps-a-staged-path)` · nothing staged | matches contract |
| C032 | `doc create` | N-C-c4/fb · report-jigc-feedback · committed · --slug occupied | `jigc doc create jigc-feedback --title "Else" --slug finalize-sweeps-a-staged-path --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, jigc-feedback:finalize-sweeps-a-staged-path)` · nothing staged | matches contract |
| C033 | `doc create` | N-C-c3/fb · report-jigc-feedback · committed · --slug distinct | `jigc doc create jigc-feedback --title "Finalize sweeps a staged path" --slug finalize-sweeps-again --task <t> --format json` | 0 | none | none (ack) | ack `target` jigc-feedback:finalize-sweeps-again · `existed` false · `copied_in` false | matches contract |
| C034 | `doc author` | N-C-a1/fb · report-jigc-feedback · committed · same title | `jigc doc author jigc-feedback --from-file - --task <t> --format json   (payload: title: "Finalize sweeps a staged path" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, jigc-feedback:finalize-sweeps-a-staged-path)` · nothing staged | matches contract |
| C035 | `doc author` | N-C-a2/fb · report-jigc-feedback · committed · different title | `jigc doc author jigc-feedback --from-file - --task <t> --format json   (payload: title: "finalize sweeps: a staged path" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, jigc-feedback:finalize-sweeps-a-staged-path)` · nothing staged | matches contract |
| C036 | `doc author` | N-A-a1/fb · report-jigc-feedback · absent | `jigc doc author jigc-feedback --from-file - --task <t> --format json   (payload: title: "Something new entirely" + a description slot)` | 0 | none | none (ack) | ack `target` jigc-feedback:something-new-entirely · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C037 | `doc create` | N-G · report-inconsistency · create idea under report-inconsistency | `jigc doc create idea --title "Not allowed here" --task <t> --format json` | 1 | `create.gate-blocked` | Mechanical | key `(create.gate-blocked, idea)` · nothing staged | matches contract |
| C038 | `doc author` | N-G · report-inconsistency · author idea under report-inconsistency (existing id) | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "A Parked Thought" + a description slot)` | 1 | `create.gate-blocked` | Mechanical | key `(create.gate-blocked, idea)` · nothing staged | matches contract |
| C039 | `doc create` | N-G · report-inconsistency · create jigc-feedback under report-inconsistency (existing id) | `jigc doc create jigc-feedback --title "Finalize sweeps a staged path" --task <t> --format json` | 1 | `create.gate-blocked` | Mechanical | key `(create.gate-blocked, jigc-feedback)` · nothing staged | matches contract |
| C040 | `doc create` | N-G · report-inconsistency · create nosuch | `jigc doc create nosuch --title "Port mismatch" --task <t> --format json` | 1 | `create.unknown-doctype` | Mechanical | key `(create.unknown-doctype, nosuch)` · nothing staged | matches contract |
| C041 | `doc create` | N-E · report-inconsistency · create empty-slug title on committed? (title '!!!') | `jigc doc create inconsistency --title "!!!" --task <t> --format json` | 1 | `create.empty-title` | Human | key `(create.empty-title, inconsistency)` · nothing staged | matches contract |
| C042 | `doc create` | N-E · report-inconsistency · create empty title + --slug occupied | `jigc doc create inconsistency --title "!!!" --slug port-mismatch --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C043 | `doc create` | N-O-c2 · report-inconsistency · other task stages it · different title | `jigc doc create inconsistency --title "Shared Thing!" --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:shared-thing · `existed` false · `copied_in` false | matches contract |
| C044 | `doc create` | N-O-c4 · report-inconsistency · other task stages it · --slug that id | `jigc doc create inconsistency --title "Else" --slug shared-thing --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:shared-thing · `existed` false · `copied_in` false | matches contract |
| C045 | `doc author` | N-O-a1 · report-inconsistency · other task stages it · same title | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Shared thing" + a description slot)` | 0 | none | none (ack) | ack `target` inconsistency:shared-thing · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C046 | `doc create` | N-O-c1 · report-inconsistency · other task stages it · same title (KEPT OPEN) | `jigc doc create inconsistency --title "Shared thing" --task <t> --format json` | 0 | `finalize.base-mismatch` | Human | key `(finalize.base-mismatch, task:cell-64-probe)` · staged: inconsistency:shared-thing | matches contract · **RECONCILER — `code` cell corrected:** this argv is exit 0 with **no finding** (`existed: false`); `finalize.base-mismatch` is the code of the kept-open task's *later* `task finalize` (RB-4, RR-6) |
| C047 | `doc create` | G-A-c1 · park-idea · absent · title | `jigc doc create idea --title "Fresh idea one" --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:fresh-idea-one · `existed` false · `copied_in` false | matches contract |
| C048 | `doc create` | G-A-c3 · park-idea · absent · --slug | `jigc doc create idea --title "Fresh idea one" --slug fresh-by-slug --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:fresh-by-slug · `existed` false · `copied_in` false | matches contract |
| C049 | `doc author` | G-A-a1 · park-idea · absent | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "Fresh idea two" + a description slot)` | 0 | none | none (ack) | ack `target` idea:fresh-idea-two · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C050 | `doc create` | G-C-c1 · park-idea · committed · same title | `jigc doc create idea --title "A Parked Thought" --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:parked-thought · `existed` true · `copied_in` false | matches contract |
| C051 | `doc create` | G-C-c2 · park-idea · committed · different title, same id | `jigc doc create idea --title "A Parked Thought!" --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:parked-thought)` · nothing staged | matches contract |
| C052 | `doc create` | G-C-c2b · park-idea · committed · different title (stop-word edge), same id | `jigc doc create idea --title "The parked thought" --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:parked-thought)` · nothing staged | matches contract |
| C053 | `doc create` | G-C-c3 · park-idea · committed · --slug distinct | `jigc doc create idea --title "A Parked Thought" --slug parked-thought-two --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:parked-thought-two · `existed` false · `copied_in` false | matches contract |
| C054 | `doc create` | G-C-c4 · park-idea · committed · --slug occupied, other title | `jigc doc create idea --title "Something else" --slug parked-thought --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:parked-thought)` · nothing staged | matches contract |
| C055 | `doc create` | G-C-c4b · park-idea · committed · --slug occupied, same title | `jigc doc create idea --title "A Parked Thought" --slug parked-thought --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:parked-thought · `existed` true · `copied_in` false | matches contract |
| C056 | `doc author` | G-C-a1 · park-idea · committed · same title | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "A Parked Thought" + a description slot)` | 0 | none | none (ack) | ack `target` idea:parked-thought · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C057 | `doc author` | G-C-a2 · park-idea · committed · different title, same id | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "Parked: thought" + a description slot)` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:parked-thought)` · nothing staged | matches contract |
| C058 | `doc create` | G-V-c1 · park-idea · copied-in · same title | `jigc doc create idea --title "A Parked Thought" --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:parked-thought · `existed` true · `copied_in` false | matches contract |
| C059 | `doc create` | G-V-c2 · park-idea · copied-in · different title, same id | `jigc doc create idea --title "Parked Thought!" --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:parked-thought)` · nothing staged | matches contract |
| C060 | `doc create` | G-V-c3 · park-idea · copied-in · --slug distinct | `jigc doc create idea --title "A Parked Thought" --slug parked-v --task <t> --format json` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, idea:parked-thought)` · nothing staged | DEFECT → D-2 (general-case twin) |
| C061 | `doc create` | G-V-c4 · park-idea · copied-in · --slug occupied, other title | `jigc doc create idea --title "Else" --slug parked-thought --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:parked-thought)` · nothing staged | matches contract |
| C062 | `doc author` | G-V-a1 · park-idea · copied-in · same title | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "A Parked Thought" + a description slot)` | 0 | none | none (ack) | ack `target` idea:parked-thought · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C063 | `doc author` | G-V-a2 · park-idea · copied-in · different title, same id | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "parked thought?" + a description slot)` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:parked-thought)` · nothing staged | matches contract |
| C064 | `doc author` | G-V-a3 · park-idea · copied-in · different id | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "Wholly other idea" + a description slot)` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, idea:parked-thought)` · nothing staged | DEFECT → D-2 (general-case twin) |
| C065 | `doc create` | G-S-c1 · park-idea · own staged · same title | `jigc doc create idea --title "My idea" --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:my-idea · `existed` true · `copied_in` false | matches contract |
| C066 | `doc create` | G-S-c2 · park-idea · own staged · different title, same id | `jigc doc create idea --title "My Idea!" --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:my-idea)` · nothing staged | matches contract |
| C067 | `doc create` | G-S-c3 · park-idea · own staged · --slug distinct | `jigc doc create idea --title "My idea" --slug my-idea-b --task <t> --format json` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, idea:my-idea)` · nothing staged | matches contract |
| C068 | `doc create` | G-S-c4 · park-idea · own staged · --slug own id, other title | `jigc doc create idea --title "Else" --slug my-idea --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:my-idea)` · nothing staged | matches contract |
| C069 | `doc author` | G-S-a1 · park-idea · own staged · same title | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "My idea" + a description slot)` | 0 | none | none (ack) | ack `target` idea:my-idea · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C070 | `doc author` | G-S-a2 · park-idea · own staged · different title, same id | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "my IDEA" + a description slot)` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:my-idea)` · nothing staged | matches contract |
| C071 | `doc author` | G-S-a3 · park-idea · own staged · different id | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "Not my idea" + a description slot)` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, idea:my-idea)` · nothing staged | matches contract |
| C072 | `doc create` | G-U-c1 · park-idea · untracked · same title | `jigc doc create idea --title "Squatter idea" --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:squatter-idea · `existed` true · `copied_in` false | matches the declared create-or-update (an untracked conformant file is copied in) — observation O-2 |
| C073 | `doc create` | G-U-c1k · park-idea · untracked · same title (KEPT) | `jigc doc create idea --title "Squatter idea" --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:squatter-idea · `existed` true · `copied_in` false | matches contract |
| C074 | `doc create` | G-U-c2 · park-idea · untracked · different title, same id | `jigc doc create idea --title "Squatter Idea!" --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:squatter-idea)` · nothing staged | matches contract |
| C075 | `doc create` | G-U-c3 · park-idea · untracked · --slug distinct | `jigc doc create idea --title "Squatter idea" --slug squatter-beside --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:squatter-beside · `existed` false · `copied_in` false | matches contract |
| C076 | `doc create` | G-U-c4 · park-idea · untracked · --slug occupied, other title | `jigc doc create idea --title "Else" --slug squatter-idea --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:squatter-idea)` · nothing staged | matches contract |
| C077 | `doc author` | G-U-a1 · park-idea · untracked · same title | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "Squatter idea" + a description slot)` | 0 | none | none (ack) | ack `target` idea:squatter-idea · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C078 | `doc author` | G-U-a2 · park-idea · untracked · different title, same id | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "squatter: idea" + a description slot)` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, idea:squatter-idea)` · nothing staged | matches contract |
| C079 | `doc create` | G-F-c1 · park-idea · untracked foreign · same slug | `jigc doc create idea --title "Foreign notes" --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:foreign-notes · `existed` true · `copied_in` false | matches (foreign bytes copied in verbatim; every later write and the finalize refuse; bytes intact) |
| C080 | `doc create` | N-F-c1 · report-inconsistency · untracked foreign · same slug | `jigc doc create inconsistency --title "Foreign inc" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:foreign-inc)` · nothing staged | matches contract |
| C081 | `doc author` | N-F-a1 · report-inconsistency · untracked foreign · same slug | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Foreign inc" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:foreign-inc)` · nothing staged | matches contract |
| C082 | `doc create` | N-D-c1 · report-inconsistency · a directory at the home path | `jigc doc create inconsistency --title "Dir home" --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:dir-home · `existed` false · `copied_in` false | matches the stated probe (*a file on disk*); the finalize it leads to is lead L-3 |
| C083 | `doc create` | N-L-c1 · report-inconsistency · a symlink (to a committed doc) at the home path | `jigc doc create inconsistency --title "Link home" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:link-home)` · nothing staged | matches contract |
| C084 | `doc create` | N-L-c2 · report-inconsistency · a DANGLING symlink at the home path | `jigc doc create inconsistency --title "Dangling home" --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:dangling-home · `existed` false · `copied_in` false | matches the stated probe; the finalize it leads to is D-7 |
| C085 | `doc create` | R-N-1 · report-inconsistency · role bound · committed id (title) | `jigc doc create inconsistency --title "Port mismatch" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C086 | `doc create` | R-N-2 · report-inconsistency · role bound · empty title | `jigc doc create inconsistency --title "???" --task <t> --format json` | 1 | `create.empty-title` | Human | key `(create.empty-title, inconsistency)` · nothing staged | matches contract |
| C087 | `doc create` | R-N-3 · report-inconsistency · role bound · empty title + --slug occupied | `jigc doc create inconsistency --title "???" --slug port-mismatch --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C088 | `doc create` | R-N-4 · report-inconsistency · role bound · bad slug grammar + title of committed id | `jigc doc create inconsistency --title "Port mismatch" --slug Port_Mismatch --task <t> --format json` | 1 | none | Human | `{"error": …}` — not a valid slug | n/a to the gate — the slug grammar answers first, code-less `error` envelope (axis 1's subject) |
| C089 | `doc create` | R-N-5 · report-inconsistency · unbound · bad slug grammar (uppercase of the occupied id) | `jigc doc create inconsistency --title "Else" --slug PORT-MISMATCH --task <t> --format json` | 1 | none | Human | `{"error": …}` — not a valid slug | n/a to the gate — the slug grammar answers first, code-less `error` envelope (axis 1's subject) |
| C090 | `doc author` | R-N-6 · report-inconsistency · role bound · author committed id | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Port mismatch" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C091 | `doc author` | R-N-7 · report-inconsistency · role bound · author empty title | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "!!!" + a description slot)` | 1 | `create.empty-title` | Human | key `(create.empty-title, inconsistency)` · nothing staged | matches contract |
| C092 | `doc create` | R-G-1 · park-idea · role bound · committed id, same title | `jigc doc create idea --title "A Parked Thought" --task <t> --format json` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, idea:mine)` · nothing staged | matches contract (identity-change outranks the copy-in); its route refuses when followed → D-9 |
| C093 | `doc create` | R-G-2 · park-idea · role bound · committed id, different title | `jigc doc create idea --title "Parked Thought!" --task <t> --format json` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, idea:mine)` · nothing staged | matches contract; its route refuses when followed → D-9 |
| C094 | `doc author` | R-G-3 · park-idea · role bound · author committed id | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "A Parked Thought" + a description slot)` | 1 | `write.identity-change` | Mechanical | key `(write.identity-change, idea:mine)` · nothing staged | matches contract; its route refuses when followed → D-9 |
| C095 | `doc create` | R-G-4 · park-idea · role bound · empty title | `jigc doc create idea --title "???" --task <t> --format json` | 1 | `create.empty-title` | Human | key `(create.empty-title, idea)` · nothing staged | matches contract |
| C096 | `doc create` | S1-c1 · report-inconsistency · committed · same title (shadow: key dropped) | `jigc doc create inconsistency --title "Port mismatch" --task <t> --format json` | 0 | none | none (ack) | ack `target` inconsistency:port-mismatch · `existed` true · `copied_in` false | matches contract |
| C097 | `doc create` | S1-c2 · report-inconsistency · committed · different title | `jigc doc create inconsistency --title "Port mismatch!" --task <t> --format json` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C098 | `doc author` | S1-a1 · report-inconsistency · committed · same title | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "Port mismatch" + a description slot)` | 0 | none | none (ack) | ack `target` inconsistency:port-mismatch · `existed` — (the `author` ack carries none) · `copied_in` false | matches contract |
| C099 | `doc create` | S1-ctl · report-inconsistency · committed · same title (shadow removed — control) | `jigc doc create inconsistency --title "Port mismatch" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C100 | `doc create` | S2-c1 · park-idea · committed · same title (shadow: key added) | `jigc doc create idea --title "A Parked Thought" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, idea:parked-thought)` · nothing staged | matches contract |
| C101 | `doc create` | S2-c2 · park-idea · committed · different title | `jigc doc create idea --title "Parked thought!" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, idea:parked-thought)` · nothing staged | matches contract |
| C102 | `doc create` | S2-c3 · park-idea · committed · --slug distinct | `jigc doc create idea --title "A Parked Thought" --slug parked-thought-2 --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:parked-thought-2 · `existed` false · `copied_in` false | matches contract |
| C103 | `doc author` | S2-a1 · park-idea · committed · same title | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "A Parked Thought" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, idea:parked-thought)` · nothing staged | matches contract · **RECONCILER — DEMOTED, NOT DRIVEN:** no repro block and not re-driven (the neighbouring C104 was) |
| C104 | `doc author` | S2-a2 · park-idea · committed · different title | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "the parked thought" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, idea:parked-thought)` · nothing staged | matches contract |
| C105 | `doc create` | S2-c0 · park-idea · absent | `jigc doc create idea --title "Entirely fresh" --task <t> --format json` | 0 | `create.already-exists` | Mechanical | key `(create.already-exists, idea:parked-thought)` · staged: idea:entirely-fresh | matches contract · **RECONCILER — `code` cell corrected:** re-driven, this argv is exit 0, `existed: false`, **no finding**; the `create.already-exists` key at `idea:parked-thought` is the previous cell's (RR-8) |
| C106 | `doc create` | G-O-c1 · park-idea · other task stages it · same title (KEPT) | `jigc doc create idea --title "Common idea" --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:common-idea · `existed` false · `copied_in` false | matches contract |
| C107 | `doc create` | G-O-c2 · park-idea · other task stages it · different title | `jigc doc create idea --title "Common Idea!" --task <t> --format json` | 0 | none | none (ack) | ack `target` idea:common-idea · `existed` false · `copied_in` false | matches contract |
| C108 | `doc author` | G-O-a1 · park-idea · other task stages it · same title | `jigc doc author idea --from-file - --task <t> --format json   (payload: title: "Common idea" + a description slot)` | 0 | `finalize.base-mismatch` | Mechanical | key `(finalize.base-mismatch, task:cell-401-probe)` · staged: idea:common-idea | matches contract · **RECONCILER — `code` cell corrected:** re-driven, this argv is exit 0 with **no finding**; `finalize.base-mismatch` at `task:cell-401-probe` is another task's later finalize (RR-12) |
| C109 | `doc create` | N-K-c1 · report-inconsistency · untracked file whose NAME differs in case (Case-Thing.md) · title 'Case thing' | `jigc doc create inconsistency --title "Case thing" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:case-thing)` · nothing staged | matches contract |
| C110 | `doc create` | N-K-c2 · report-inconsistency · committed port-mismatch · title differing only in case | `jigc doc create inconsistency --title "PORT MISMATCH" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |
| C111 | `doc author` | N-K-a2 · report-inconsistency · committed port-mismatch · title differing only in case | `jigc doc author inconsistency --from-file - --task <t> --format json   (payload: title: "port mismatch" + a description slot)` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:port-mismatch)` · nothing staged | matches contract |

## Table 2 · the first drive by hand (rig A, `fresh`) — the flow, the routes, the bound role

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| A01 | `doc create` | new · absent · first report | `jigc doc create inconsistency --title "Port mismatch" --task <t1>` | 0 | none | none (ack) | `inconsistency:port-mismatch` | matches contract |
| A02 | `doc create` | new · staged by this same task · re-run, same title | the same argv again, text and `--format json` | 0 | none | none (ack) | text `inconsistency:port-mismatch (already existed — copied in for update)` · JSON `existed: true`, `copied_in: false` | matches contract (idempotent; the ack text is the one `doc author --help` declares for this case — observation O-1) |
| A03 | `task finalize` | new · the first report lands | `jigc task finalize <t1>` | 0 | none | none (ack) | `promoted docs/inconsistencies/port-mismatch.md` · `1 file committed` | matches contract |
| A04 | `doc create` | new · committed · six different titles onto the id | `--title` each of `"Port mismatch!"` · `"port MISMATCH"` · `"The Port Mismatch"` · `"Port  mismatch."` · `"A port mismatch"` · `"Port-mismatch"` | 1 ×6 | `create.already-exists` | Mechanical | `at: inconsistency:port-mismatch` · route names a distinct `--title` or `--slug` · nothing staged · checksum unchanged | matches contract (punctuation, case, an edge stop-word, a hyphen: all refused, never `write.title-ignored`) |
| A05 | `doc create` | new · committed · the route run verbatim (title kept, `--slug` distinct) | `jigc doc create inconsistency --title "Port mismatch" --slug port-mismatch-again --task <t2>` | 0 | none | none (ack) | `inconsistency:port-mismatch-again` · role `inconsistency` bound to it | matches contract — the route lands beside |
| A06 | `doc create` | new · role bound · committed id by title | `jigc doc create inconsistency --title "Port mismatch" --task <t2>` | 1 | `create.already-exists` | Mechanical | route: *this task already holds `inconsistency:port-mismatch-again` … `jigc task finalize <t2>` / `jigc task discard <t2> --force` / `jigc start --workflow report-inconsistency "<intent>"`* | matches contract (O23's route; ahead of `write.identity-change`) |
| A07 | `doc author` | new · role bound · committed id by payload title | `jigc doc author inconsistency --from-file - --task <t2>` (title `Port mismatch`) | 1 | `create.already-exists` | Mechanical | the same route, spelled *a distinct payload `title:`* | matches contract |
| A08 | `doc create` | new · role bound · a distinct, free id | `jigc doc create inconsistency --title "Third thing" --task <t2>` | 1 | `write.identity-change` | Mechanical | `at: inconsistency:port-mismatch-again` · route `jigc doc rename inconsistency:port-mismatch-again --to 'Third thing' --task <t2>` | matches contract |
| A09 | `doc create` | new · role bound · `--slug <its own id>`, same title | `… --title "Port mismatch" --slug port-mismatch-again …` | 0 | none | none (ack) | `(already existed — copied in for update)` | matches contract (idempotent) |
| A10 | `doc create` | new · role bound · `--slug <another free id>` | `… --title "Port mismatch" --slug yet-another …` | 1 | `write.identity-change` | Mechanical | route `jigc doc rename … --to 'Port mismatch' --slug yet-another --task <t2>` | matches contract |
| A11 | `doc rename` | the `write.identity-change` route run verbatim (the task's own never-committed doc) | `jigc doc rename inconsistency:port-mismatch-again --to 'Third thing' --task <t2>` | 0 | none | none (ack) | `inconsistency:third-thing (renamed to "Third thing" from inconsistency:port-mismatch-again)` · role re-keyed | matches contract — the route runs |
| A12 | `doc rename` | a staged doc re-slugged onto the **occupied** committed id, by title | `jigc doc rename inconsistency:port-mismatch-again --to "Port mismatch" --task <t2>` | 1 | `write.already-present` | Mechanical | key `(write.already-present, inconsistency:port-mismatch)` · staged doc and committed checksum unchanged | matches contract (`doc rename` cannot do what the gate refused) |
| A13 | `doc rename` | the same, by `--slug port-mismatch` | `… --to "Whatever else" --slug port-mismatch …` | 1 | `write.already-present` | Mechanical | as A12 | matches contract |

## Table 3 · after the refusal — the other `doc` write leaves on the existing doc (scope item 6; rig A, the same task `<t2>`)

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| X01 | `doc set-field` | report task · the committed finding's address | `jigc doc set-field inconsistency:port-mismatch#meta/kind --value doc-doc --task <t2>` | 0 | none | none (ack) | `set … = doc-doc (copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)` | the **declared bound** (§1.5) — HOLDS-AS-DECLARED |
| X02 | `doc set-slot` | the same | `jigc doc set-slot inconsistency:port-mismatch#description --from-file - --task <t2>` | 0 | none | none (ack) | `set slot … (39 chars)` | declared bound |
| X03 | `doc add-item` | the same | `jigc doc add-item inconsistency:port-mismatch#sides --title "docs/extra.md" --slug extra --task <t2>` | 0 | none | none (ack) | `inconsistency:port-mismatch#sides/extra` | declared bound (§1.5 names `set-field`/`set-slot`/`remove-item`; `add-item` is the same bound, unnamed there) |
| X04 | `doc remove-item` | the same | `jigc doc remove-item inconsistency:port-mismatch#sides/readme --task <t2>` | 0 | none | none (ack) | `removed item …#sides/readme` | declared bound |
| X05 | `doc retitle-item` | the same | `jigc doc retitle-item inconsistency:port-mismatch#sides/server --title "src/main.rs" --task <t2>` | 0 | none | none (ack) | `retitled item … (anchor frozen)` | declared bound (unnamed in §1.5) |
| X06 | `doc rename` | the same · a re-slug (by title, and by `--slug moved-away`) | `jigc doc rename inconsistency:port-mismatch --to "Port mismatch retitled" [--slug moved-away] --task <t2>` | 1 ×2 | `write.identity-change` | Mechanical | route `jigc rename inconsistency:port-mismatch --to …` once the task is out of flight | matches `write-commands.md` → `jigc doc rename` (a committed doc copied in is retitle-only) |
| X07 | `doc rename` | the same · a same-slug retitle | `jigc doc rename inconsistency:port-mismatch --to "PORT mismatch!" --task <t2>` | 0 | none | none (ack) | `(retitled "PORT mismatch!" — the committed identity keeps its slug)` | declared bound (the existing finding's `# H1` is editable from a report task) |
| X08 | `doc create` | the same task, now holding the existing doc as a copied-in copy | `jigc doc create inconsistency --title "Port mismatch" --task <t2>` | 1 | `create.already-exists` | Mechanical | probed independently of what the task stages | matches contract (§4 Scope, P3) |
| X09 | `task finalize` | the report task lands with both docs | `jigc task finalize <t2>` | 0 | none | none (ack) | `promoted docs/inconsistencies/port-mismatch.md` · `promoted docs/inconsistencies/third-thing.md` · `2 files committed`; checksum `73f3f60a…` → `f745a70d…`, the description now the report task's text | **the declared bound, measured**: a `new: true` report task rewrote an earlier finding through six edit leaves and landed it at exit 0. §4's scope is the two create doors; §1.5 states the engine does not enforce append-only and parks the edit gate. Not filed as a finding |

## Table 4 · routes run verbatim, and the states behind D-2, D-3, D-6 and D-9

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| Rt01 | `doc rename` | N-S-c2's route (own staged doc, different title, same id) | `jigc doc rename inconsistency:own-thing --to 'Own Thing!' --task <t>` then the create re-run unchanged | 0, 0 | none | none (ack) | `(retitled "Own Thing!" — the id is unchanged)` then `(already existed — copied in for update)` | matches contract — route runs, re-run lands |
| Rt02 | `doc rename` | N-S-c4's route (`--slug <own id>`, other title) | `jigc doc rename inconsistency:own-thing --to 'Different words' --slug own-thing --task <t>` then the create re-run | 0, 0 | none | none (ack) | `# Different words` in the staged copy | matches contract |
| Rt03 | `doc set-field` | new · an edit leaf's first touch of the committed finding, role unbound | `jigc doc set-field inconsistency:port-mismatch#meta/kind --value doc-doc --task <t>` | 0 | none | none (ack) | `roles.json` before: absent · after: `{"inconsistency": "inconsistency:port-mismatch"}`; provenance `edited-from-base` | matches M45's *a copy-on-write binds the role* — and is the precondition of D-2 |
| Rt04 | `doc create` | new · role bound by Rt03 · the gate route's own advice (`--slug` distinct) | `jigc doc create inconsistency --title "Port mismatch" --slug port-mismatch-v --task <t>` | 1 | `write.identity-change` | Mechanical | route `jigc doc rename inconsistency:port-mismatch --to 'Port mismatch' --slug port-mismatch-v --task <t>` — *"moves the doc this task already holds onto the title (and id) you asked for"* | **DEFECT → D-2**: the route names a move the next row refuses |
| Rt05 | `doc rename` | Rt04's route run verbatim | `jigc doc rename inconsistency:port-mismatch --to 'Port mismatch' --slug port-mismatch-v --task <t>` | 1 | `write.identity-change` | Mechanical | route `jigc rename inconsistency:port-mismatch --to 'Port mismatch' --slug port-mismatch-v` — the whole-repo rename of the **existing committed finding** | **DEFECT → D-2** |
| Rt06 | `doc create` → `doc rename` | the same with a distinct title | `jigc doc create inconsistency --title "A wholly new finding" --task <t>` then its route `jigc doc rename inconsistency:port-mismatch --to 'A wholly new finding' --task <t>` | 1, 1 | `write.identity-change` ×2 | Mechanical | second hop: `jigc rename inconsistency:port-mismatch --to 'A wholly new finding'` | **DEFECT → D-2** |
| Rt07 | `doc rename` | general · G-V-c2's route (a committed idea copied in, same-slug retitle) | `jigc doc rename idea:parked-thought --to 'Parked Thought!' --task <t>` then the create re-run | 0, 0 | none | none (ack) | `(retitled … — the committed identity keeps its slug)` | matches contract |
| Rt08 | `doc rename` | general · G-V-c3 / G-V-a3's routes (the `write.identity-change` route over a committed copy) | `jigc doc rename idea:parked-thought --to 'A Parked Thought' --slug parked-v --task <t>` · `… --to 'Wholly other idea' --task <t>` | 1, 1 | `write.identity-change` | Mechanical | route `jigc rename idea:parked-thought --to …` | **DEFECT → D-2** (the general-case twin) |
| Rt09 | `doc rename` | general · R-G-1's route (own fresh doc onto an occupied committed id) | `jigc doc rename idea:mine --to 'A Parked Thought' --task <t>` | 1 | `write.already-present` | Mechanical | route: `--slug <other-slug>`, or `jigc doc show idea:parked-thought` and edit that one | **DEFECT → D-9** (the first route refuses when followed; the second is sound) |
| Rt10 | `doc set-field` → `doc create` | role bound to this task's own fresh doc, then an edit leaf copies a committed doc in | general: `set-field idea:parked-thought#meta/trigger …` then `doc create idea --title "A Parked Thought"`; new: `set-field inconsistency:port-mismatch#meta/kind …` then `doc create inconsistency --title "Port mismatch"` | 0 then 1 | general `write.identity-change` · new `create.already-exists` | Mechanical | the role stays on the task's own doc (`idea:mine` / `inconsistency:mine`) — a second copy-in does not re-bind | matches contract |
| Rt11 | `task finalize` | new · staged in another open task · the FIRST holder lands | `jigc task finalize <holder>` | 0 | none | none (ack) | `promoted docs/inconsistencies/shared-thing.md` | matches contract |
| Rt12 | `task validate` | the SECOND task, its own staged doc now colliding with the landed one | `jigc task validate <t>` | 0 | none (one advisory `file-state.staged-copy`) | Informational | no blocking row | a datum: the preview is silent about the block Rt13 raises (row 5's subject) |
| Rt13 | `task finalize` | the same | `jigc task finalize <t>` (text and `--format json`) | 3 | `finalize.base-mismatch` | Human | key `(finalize.base-mismatch, task:<t>)` · route *resolve the overlap on `docs/inconsistencies/shared-thing.md` against the new history, or discard …`--force`* · the landed doc's checksum unchanged, its marker still present | no overwrite — the safe direction; the route is half of **D-3** |
| Rt14 | `doc create` | the same task re-runs its create (its own staged doc; the id is now on disk) | `jigc doc create inconsistency --title "Shared thing" --task <t>` | 1 | `create.already-exists` | Mechanical | route: *this task already holds `inconsistency:shared-thing` … land it with `jigc task finalize <t>` … or `jigc task discard <t> --force`* | **DEFECT → D-3**: the route's first exit is the refusal of Rt13, its second destroys the second finding's prose |
| Rt15 | `doc rename` → `task finalize` | the exit no surface names | `jigc doc rename inconsistency:shared-thing --to "Shared thing" --slug shared-thing-2 --task <t>` then `jigc task finalize <t>` | 0, 0 | none | none (ack) | `promoted docs/inconsistencies/shared-thing-2.md`; both findings read back, each with its own marker | the working exit exists (D-3) |
| Rt16 | `task finalize` | general · staged in another open task (`park-idea`): first lands, second finalizes | `jigc task finalize <holder>` · `jigc doc create idea --title "Common idea" --task <t>` (re-run) · `jigc task finalize <t> --format json` | 0 · 0 (`existed: true`) · 3 | `finalize.base-mismatch` | Human | the landed idea's checksum unchanged | no overwrite; same generic route as Rt13 |
| Rt17 | `task discard` | a singleton under a shadow's `new: true` — the fixed-identity route run verbatim | `jigc task discard <t>` | 1 | `task-discard.staged-prose` | Mechanical | *stages 1 doc(s) that no commit has a copy of … `jigc task discard <t> --force`* | **DEFECT → D-6** (the route omits `--force`) |

## Table 5 · singleton / placement doctypes (scope item 3; rig C, `committed-singletons`)

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| Sg01 | `doc create` | shipped `form-vision` (no `new`) · committed `VISION.md` · the schema's title | `jigc doc create vision --title Vision --task <t> --format json` | 0 | none | none (ack) | `target` vision:vision · `existed: true` | matches contract — create-or-update unchanged |
| Sg02 | `doc create` | the same · another title | `… --title "Our Grand Vision" …` | 1 | `write.title-ignored` | Mechanical | key `(write.title-ignored, vision:vision)` · route `jigc doc create vision --title Vision --task <t>` — *the title the schema fixes* | matches contract (the fixed-title arm routes at itself, never at `doc rename`) |
| Sg03 | `doc create` | the same · `--slug other-vision` | `… --title Vision --slug other-vision …` | 0 | none | none (ack) | `target` vision:vision — `--slug` inert for a singleton, as `--help` says | matches contract |
| Sg04 | `doc author` | the same · payload title `Vision` / `Our Grand Vision` | `jigc doc author vision --from-file - --task <t> --format json` | 0 / 1 | none / `write.title-ignored` | none (ack) / Mechanical | route: *set the payload's `title:` to `Vision` (or drop the line …)* | matches contract |
| Sg05 | `doc create` | shipped `planning` · committed `docs/roadmap.md`, `docs/decisions-log.md`; `deferral-ledger` absent | `jigc doc create roadmap --title Roadmap` · `… decisions-log --title 'Decisions Log'` · `… deferral-ledger --title 'Deferral Ledger'` | 0 ×3 | none | none (ack) | `existed: true` · `true` · `false`; roles `roadmap`, `log`, `ledger` bound | matches contract — planning's idempotent singleton create is unchanged |
| Sg06 | `doc create` | shipped `planning` · roadmap, another title | `jigc doc create roadmap --title "Road Map 2" …` | 1 | `write.title-ignored` | Mechanical | route at `--title Roadmap` | matches contract |
| Sg07 | `doc author` | shipped `planning` · roadmap payload `Roadmap` / `Another roadmap` | `jigc doc author roadmap --from-file - …` | 0 / 1 | none / `write.title-ignored` | none (ack) / Mechanical | as Sg04 | matches contract |
| Sg08 | `doc create` | shipped `record-change` · committed root `CHANGELOG.md` (placement) | `jigc doc create changelog --title Changelog` / `--title "Release notes"` | 0 / 1 | none / `write.title-ignored` | none (ack) / Mechanical | `existed: true` / fixed-title route | matches contract |
| Sg09 | `doc author` | shipped `record-change` · payload `Changelog` / `Release notes` | `jigc doc author changelog --from-file - …` | 0 / 1 | none / `write.title-ignored` | none (ack) / Mechanical | as Sg04 | matches contract |
| Sh01 | `doc create` | **shadow adds `new: true`** to `form-vision` · committed vision · three identities asked (`--title Vision` · `--title "Other title"` · `--slug vision-two`) | `jigc doc create vision … --task <t> --format json` | 1 ×3 | `create.already-exists` | Mechanical | key `(create.already-exists, vision:vision)` · route: *`vision` has a fixed identity … no distinct identity exists for this create to mint … If this task holds nothing else, `jigc task discard <t>` abandons it* · nothing staged, no role | the refusal and its statement match `write-commands.md` (the fixed-identity arm); the route's command is **D-6** |
| Sh02 | `doc author` | the same · payload `Vision` / `Other title` | `jigc doc author vision --from-file - …` | 1 ×2 | `create.already-exists` | Mechanical | the same route | matches (ahead of the fixed-title arm) · **RECONCILER:** the `Other title` payload is a demoted sub-argv — only the `Vision` payload was re-driven (RR-12) |
| Sh03 | `doc create` | the same, after `doc set-slot vision:vision#thesis` copied the vision in and bound the role | `jigc doc create vision --title Vision …` | 1 | `create.already-exists` | Mechanical | the bound-role route (`task finalize` / `task discard --force` / `jigc start --workflow form-vision`) | matches contract |
| Sh04 | `doc create` · `doc author` | shadow adds `new: true` to `record-change` · committed root `CHANGELOG.md` | `jigc doc create changelog --title Changelog` · `jigc doc author changelog …` | 1, 1 | `create.already-exists` | Mechanical | key `(create.already-exists, changelog:changelog)` — the placement home is probed, not `docs-root` | matches contract |
| Sh05 | `doc create` · `doc author` | shadow adds `new: true` to `planning`'s `roadmap` and `deferral-ledger` entries | roadmap: create, author · deferral-ledger: create, create again · decisions-log (no `new` on its entry): create | 1, 1 · 0, 0 · 0 | `create.already-exists` (roadmap) / none | Mechanical / none (ack) | roadmap refused; the absent ledger is created (`existed: false`), its re-run idempotent (`existed: true`); the sibling entry without the key copies in | matches contract — the key is per entry · **RECONCILER:** the roadmap `doc author` argv is a demoted sub-argv — not re-driven (RR-12) |

All four singleton files' checksums were identical before and after tables 5's blocks, and `git status` clean.

## Table 6 · project shadows of the entry (scope item 5; rig D, `fresh`, each shadow committed with plain git and removed again)

The matrix cells under the two principal shadows (S1 the key **dropped** from `report-inconsistency`, S2
the key **added** to `park-idea`) are rows of Table 1 (`S1-*`, `S2-*`). The rest:

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| S01 | `doc create` | key dropped · a task minted **before** the shadow landed | `jigc doc create inconsistency --title "Port mismatch" --task <pre> --format json` | 0 | none | none (ack) | `existed: true` — copied in | a datum: the entry is read from the cascade **at the write**, not pinned at mint (consistent with *resolve its effective `allows-create` through the cascade*) |
| S02 | `doc create` | key added · a task minted before the shadow landed | `jigc doc create idea --title "A Parked Thought" --task <pre> --format json` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, idea:parked-thought)` | the same datum, the other direction |
| S03 | `start` | key dropped · what the composed text says | `jigc start --workflow report-inconsistency "<intent>"` | 0 | none | Informational | *"This task files exactly one NEW record. A title whose slug a doc on disk already holds is refused `create.already-exists`, with nothing staged"* — while the same shadow's create copies in at exit 0 (`S1-c1`) | **DEFECT → D-10** |
| S04 | `start` | key added to `park-idea` · what the composed text says | `jigc start --workflow park-idea "<intent>"` | 0 | none | Informational | the step says nothing of `create.already-exists`; it still describes the batch verb's `copied in for update` over a doc *this task* staged, which stays true | matches (silent, not false) |
| S05 | `start` | key misspelt `nwe: true` · mint under it | `jigc start --workflow report-inconsistency "<intent>"` (text; `--format json`) | 1 | `workflow-refs.malformed-front-matter` | Human | *allows-create[0]: unknown field `nwe`, expected one of `type`, `as`, `new` at line 7 column 47* · nothing minted · JSON is `{"error": "<the text finding>"}` | the refusal matches §4/§10 (names the key); what it does not name is **D-8** |
| S06 | `start` | misspelt · bare orientation, and minting an **unrelated** workflow | `jigc start` · `jigc start --workflow park-idea "<intent>"` | 1, 1 | `workflow-refs.malformed-front-matter` | Human | the identical message — naming neither the workflow nor the file | loud, as intended; **D-8** |
| S07 | `doc create` · `doc author` | misspelt · a `report-inconsistency` task minted before the typo | `jigc doc create inconsistency --title "Port mismatch" --task <pre>` · `jigc doc author inconsistency --from-file - --task <pre>` | 1, 1 | `workflow-refs.malformed-front-matter` | Human | nothing staged | matches §4 (*a misspelt `new` never loads as an absent one*) |
| S08 | `doc set-field` · `doc add-item` · `doc rename` | misspelt · the same task's edit leaves on its own staged doc | `jigc doc set-field inconsistency:typo-holder#meta/kind …` · `jigc doc add-item …#sides …` · `jigc doc rename inconsistency:typo-holder --to "Typo holder two" …` | 0 ×3 | none | none (ack) | the writes land; the rename re-slugs | a datum: these leaves do not load the workflow, so the load error does not reach them |
| S09 | `task validate` · `task finalize` · `start` (resume) | misspelt · the same task | `jigc task validate <pre>` · `jigc task finalize <pre> --dry-run` · `jigc start --task <pre>` | 1 ×3 | `workflow-refs.malformed-front-matter` | Human | `{"error": …}` under `--format json` | loud; the task cannot land until the shadow is fixed |
| S10 | `workflow` · `describe` | misspelt | `jigc workflow report-inconsistency --preview` · `jigc workflow park-idea --preview` · `jigc describe` | 1 ×3 | `workflow-refs.malformed-front-matter` | Human | as S05 | loud · **RECONCILER:** `jigc workflow report-inconsistency --preview` is a demoted sub-argv — the other two were re-driven (RR-8) |
| S11 | `validate` | misspelt · store scope | `jigc validate --format json` | 0 | `workflow-refs.malformed-front-matter` (blocking) | Human | key `(workflow-refs.malformed-front-matter, workflow:report-inconsistency)` — the only surface that names the workflow | matches `workflow-dialect.md` (*reports it at `workflow:<id>` and, being store scope, keeps exit 0*) |
| S12 | `doc create` | misspelt · an open task of an **unrelated** workflow (`park-idea`) | `jigc doc create idea --title "Unrelated idea" --task <other>` | 0 | none | none (ack) | minted | a datum: the write door loads only its own task's workflow; `start` loads all |
| S13 | `milestone create` · `milestone add-task` · `milestone list-tasks` · `config list` | misspelt · doors that do not load a workflow definition | `jigc milestone create "typo milestone"` · `jigc milestone add-task typo-milestone "<intent>" --workflow report-inconsistency` (×2) · `jigc milestone list-tasks typo-milestone` · `jigc config list` | 0 ×5 | none | none (ack) | `added task:… ` + a record commit, for a sub-task whose workflow no door can compose | a datum, not a contradiction of *every door that loads a workflow* — lead L-5 |
| S14 | `start` | a stray extra key — `{…, new: true, extra: 1}`; and `{type: idea, as: idea, note: hello}` on `park-idea` | `jigc start --workflow <wf> "<intent>"` | 1, 1 | `workflow-refs.malformed-front-matter` | Human | *unknown field `extra`* / *unknown field `note`* | matches §4 (R5) · **RECONCILER:** the `note: hello` shadow on `park-idea` is a demoted sub-argv — the `extra: 1` shadow was re-driven (RR-8) |
| S15 | `start` → `doc create` | value variants that **load**: `new: false` · `new: True` · a quoted key `"new": true` | mint, then `jigc doc create inconsistency --title "Port mismatch" …` | 0 → 0 / 1 / 1 | none / `create.already-exists` ×2 | — | `new: false` copies in (`existed: true`); `True` and `"new"` are the guard | matches contract |
| S16 | `start` | value variants that **do not load**: `new: yes` · `new: "true"` · `new: 1` · `new: ~` · `New: true` | `jigc start --workflow report-inconsistency "<intent>"` | 1 ×5 | `workflow-refs.malformed-front-matter` | Human | *allows-create[0].new: invalid type: …, expected a boolean* ×4 · *unknown field `New`* | matches contract — no value silently switches the guard off |
| S17 | `start` | entry forms: a **bare-string** entry `- inconsistency`; an object without `as` | `jigc start --workflow report-inconsistency "<intent>"` | 1, 1 | `workflow-refs.malformed-front-matter` | Human | *invalid type: string "inconsistency", expected struct AllowsCreate* · *missing field `as`* | **DEFECT → D-5** (`write-commands.md` documents the bare form) |
| S18 | `start` | one doctype twice (`new: true` on one, none on the other) | the same | 1 | `workflow-refs.allows-create-duplicate-type` | Human | *declares doctype `inconsistency` more than once* | matches `workflow-dialect.md` |
| S19 | `start` → `doc create` | two types, `new: true` on the **other** type's entry | mint, then `jigc doc create inconsistency --title "Port mismatch" …` | 0 → 0 | none | none (ack) | `existed: true` — the key is per entry | matches contract |

## Table 7 · `doc author`'s payload refusals against the gate (order of refusals; rig D, a `report-inconsistency` task)

| # | door | cell | argv driven (`jigc doc author <ty> --from-file - --task <t> --format json`) | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| P01 | `doc author` | `Ungrammatical` (an unknown top-level key) + the committed title | `inconsistency` | 1 | `write.wrong-shape` | Mechanical | key `(write.wrong-shape, inconsistency)` | matches the stated ranking (shape outranks admission outranks the title) |
| P02 | `doc author` | `MissingTitle` | `inconsistency` | 1 | `write.wrong-shape` | Mechanical | as P01 | matches |
| P03 | `doc author` | `BareSlotValue` + the committed title; and + a fresh title | `inconsistency` | 1, 1 | `write.malformed-value` | Mechanical | key `(write.malformed-value, inconsistency)` — ahead of `create.already-exists` | matches |
| P04 | `doc author` | `WrappedFieldValue` + the committed title | `inconsistency` | 1 | `write.malformed-value` | Mechanical | as P03 | matches |
| P05 | `doc author` | an unknown doctype · ungrammatical / well-formed | `nosuch` | 1, 1 | `write.wrong-shape` / `create.unknown-doctype` | Mechanical | keys at the bare id `nosuch` | matches |
| P06 | `doc author` | a gate-blocked doctype · ungrammatical / well-formed with an existing id | `idea` | 1, 1 | `write.wrong-shape` / `create.gate-blocked` | Mechanical | key `(create.gate-blocked, idea)` — ahead of any title or existence check | matches |

## Table 8 · the fan-out join (scope item 4; rigs E, F, G — `fresh`, `finalize.fan-out.squash = true`, the pack default)

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| F01 | `config get` | the knob the join commits under | `jigc config get finalize.fan-out.squash` | 0 | none | Informational | `finalize.fan-out.squash = true  (pack-default)` | datum |
| F02 | `milestone create` · `milestone add-task` · `milestone provision` · `milestone execute` | a milestone of `--workflow report-inconsistency` sub-tasks | `jigc milestone create "<title>"` · `jigc milestone add-task <m> "<intent>" --workflow report-inconsistency` ×2–3 · `jigc milestone provision <m>` · `jigc milestone execute <m>` | 0 each | none | none (ack) | one record commit per op · N worktrees · N `Spawn:` lines | matches contract |
| F03 | `workflow` | the sub-task compose (a `Spawn:` line run in its worktree) | `cd $REPO/.jigc/worktrees/<sub> && jigc workflow report-inconsistency --task <sub>` | 0 | none | Informational | names `jigc task finalize` 0 times, `jigc milestone finalize` once; carries the `create.already-exists` paragraph | matches (S2; row 8 owns the text) |
| F04 | `doc create` | two sub-tasks mint one slug in isolation · both filing orders | `jigc doc create inconsistency --title "<same>" --task <sub>` in each worktree — lower task id first (rig F, `batch-one`), higher first (rig E; rig F, `batch-three`) | 0, 0 | none | none (ack) | each acks the bare slug; neither home is on disk | matches §4 |
| F05 | `doc create` | a sub-task re-runs its create | the same argv again in the worktree | 0 | none | none (ack) | `(already existed — copied in for update)` | matches (idempotent) |
| F06 | `doc create` · `doc author` | a sub-task mints an id **committed at the base** | `jigc doc create inconsistency --title "Port mismatch" --task <sub>` · author with `Port mismatch!` | 1, 1 | `create.already-exists` | Mechanical | nothing staged in the sub-area | matches contract |
| F07 | `doc create` | a sub-task mints an id a plain task landed on the branch **after** provision (absent from its pinned worktree) | `jigc doc create inconsistency --title "Late clash" --task <sub>` | 1 | `create.already-exists` | Mechanical | key `(create.already-exists, inconsistency:late-clash)` — the home is probed in the main checkout, not the worktree | matches contract (the safe direction) |
| F08 | `milestone join` | the suffixing · both filing orders | `jigc milestone join <m>` | 0 | none | none (ack) | lower task id keeps `<slug>`, higher takes `<slug>-2`, whichever filed first — `← suffixed -2 on collision` | matches §4 / `storage.md` rule 4 |
| F09 | `milestone finalize` | the join lands · both filing orders | `jigc milestone finalize <m>` (`batch-one`, `batch-three`) | 0 | none | none (ack) | `promoted …/<slug>.md` + `…/<slug>-2.md` + the record · `3 files committed`; each file carries its own reporter's marker | matches contract |
| F10 | `doc create` | after the join landed, a plain report task files the same title | `jigc doc create inconsistency --title "Third twin" --task <t> --format json` | 1 | `create.already-exists` | Mechanical | key at `inconsistency:third-twin` | matches contract |
| F11 | `milestone join` | **the suffixed id is already on disk** — committed `inconsistency:stage-2`, an untracked `stage-3.md`; three sub-tasks each mint `stage` | `jigc milestone join stage-reports` (text; `--format json`) | 0 | none — `findings: []` | none (ack) | `inconsistency:stage-2 (created · from bravo-…) ← suffixed -2 on collision` · `inconsistency:stage-3 (created · from charlie-…) ← suffixed -3 on collision` | **DEFECT → D-1** — the suffix is minted onto two occupied ids |
| F12 | `milestone finalize` | the same | `jigc milestone finalize stage-reports --format json` | **0** | none — `displaced: []` | none (ack) | manifest `promoted docs/inconsistencies/stage-2.md`, `…/stage-3.md`, `…/stage.md` · the committed finding's marker 1 → 0 · the untracked note's marker 1 → 0, found nowhere under `$REPO` afterwards | **DEFECT → D-1, proposed TIER 1** |
| F13 | `milestone join` · `milestone finalize` | the same shape, first drive (rig F, `batch-four`): committed `inconsistency:pair-report-2`, two sub-tasks mint `pair-report` | `jigc milestone join batch-four` · `jigc milestone finalize batch-four` | 0, 0 | none | none (ack) | `promoted docs/inconsistencies/pair-report-2.md`; checksum `0ded9e6c…` → `8bd24211…`; the earlier finding's marker 1 → 0 | **DEFECT → D-1** |
| F14 | `milestone join` · `milestone finalize` | the same shape under a workflow **without** `new: true` (`park-idea` sub-tasks; committed `idea:pair-idea-2`) | `jigc milestone join batch-five` · `jigc milestone finalize batch-five` | 0, 0 | none | none (ack) | `promoted docs/ideas/pair-idea-2.md`; checksum `20a2c1cf…` → `181a2009…`; marker 1 → 0 | **DEFECT → D-1** — not specific to the `new: true` entry |
| F15 | `validate` | after F13 | `jigc validate --format json` | 0 | advisory `schema-conformance.repeatable-populated` only | Informational | nothing names the overwrite | part of D-1's evidence: no later surface reports it |
| F16 | `doc set-field` · `doc set-slot` → `milestone finalize` | a sub-task whose create was refused (F06's shape, `batch-two`) runs the step's follow-up lines at the slug the title minted | `jigc doc set-field inconsistency:twin-report#meta/kind …` · `jigc doc set-slot inconsistency:twin-report#description …` · `jigc milestone join batch-two` · `jigc milestone finalize batch-two` | 0 ×4 | none | none (ack) | join: `inconsistency:twin-report (edited-from-base · from bravo-…)`; the earlier finding's description replaced by the sub-task's | **the declared bound again** (Table 3), reached through the fan-out; and the precondition of D-2 inside a sub-task (F17) |
| F17 | `doc create` | the same sub-task then tries the gate route's distinct title | `jigc doc create inconsistency --title "Second twin" --task <sub>` | 1 | `write.identity-change` | Mechanical | route `jigc doc rename inconsistency:twin-report --to 'Second twin' --task <sub>` | **DEFECT → D-2** |
| F18 | `milestone finalize` | a plain report task's doc-only commit landed on the branch between provision and finalize (rig E) | `jigc milestone finalize seed-batch-one` | 3 | `finalize.base-mismatch` | Human | *the commits landed since move more than milestone-record bookkeeping* · route: out-of-band git · the committed `race-clash.md` unchanged | no overwrite; the wedge itself is lead L-1 (row 5's subject) |

## Table 9 · edge identities at the home, their finalizes, and the reads after (rig B)

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| E01 | `task finalize` | general · an untracked **conformant** file at the home, copied in by `G-U-c1`, then `set-slot` on its description | `jigc task finalize <t>` | 0 | advisory `file-state.baseline-adopt` | Informational | `promoted docs/ideas/squatter-idea.md`; the hand-written description's marker 1 → 0 | the declared create-or-update, on a file git never held: the ack said `existed: true`, the staged copy showed the human's prose, and the slot was overwritten by an explicit `set-slot`. Not filed; stated because the bytes are in no commit |
| E02 | `task validate` · `task finalize` | general · an untracked **foreign** file at the home, copied in verbatim by `G-F-c1` | `jigc task validate <t>` · `jigc task finalize <t>` | 3, 3 | `conformance.section-missing` | none | every later write refused *nothing was persisted*; the foreign file's checksum unchanged | matches contract (no loss) |
| E03 | `task finalize` | new · a **directory** named `<slug>.md` at the home (`N-D-c1` passed the gate) | `jigc task finalize <t>` | 1 | none | none | *could not read "$REPO/docs/inconsistencies/dir-home.md" before promoting over it: Is a directory (os error 21)* · *task … is intact* | lead L-3 (a code-less refusal at row 5's door) |
| E04 | `task finalize` | new · a **dangling symlink** at the home (`N-L-c2` passed the gate) | `jigc task finalize <t>` | 0 | none | none (ack) | `promoted docs/inconsistencies/dangling-home.md` · `1 file committed` — the commit holds mode `120000` → `nowhere.md`; the finding's bytes are in an untracked `nowhere.md` | **DEFECT → D-7** |
| E05 | `doc show` | after E04 | `jigc doc show inconsistency:dangling-home` | 0 | none | Informational | serves the finding through the symlink | part of D-7 (the local read hides it) |
| E06 | `doc list` | with E03's directory still at the home | `jigc doc list inconsistency` | 1 | none | none | *reading the committed doc at "$REPO/docs/inconsistencies/dir-home.md": Is a directory (os error 21)* — the whole listing fails | lead L-3 (row 7's subject) |

## Table 10 · the dev pack's `adr` entry (entries without `new`: `record-decision`, `single-task`; rig D)

Driven identically under both workflows; every exit and code was the same in both.

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| Ad1 | `doc create` | committed `adr:use-a-single-node-cache` · same title | `jigc doc create adr --title "Use a single node cache" --task <t> --format json` | 0 | none | none (ack) | `existed: true` | matches — create-or-update |
| Ad2 | `doc create` | committed · different title, same id | `… --title "Use a Single-Node cache!" …` | 1 | `write.title-ignored` | Mechanical | route: a distinct `--title`, or `--slug <slug>` — `jigc doc create adr --title <title> --slug <slug> --task <t>` · never `doc rename` | matches the F3 route fix |
| Ad3 | `doc author` | committed · different payload title, same id | `jigc doc author adr --from-file - …` (title `use a single node: cache`) | 1 | `write.title-ignored` | Mechanical | route: a distinct payload `title:` | matches |
| Ad4 | `doc create` | committed · `--slug <the occupied id>`, other title | `… --title "Something else" --slug use-a-single-node-cache …` | 1 | `write.title-ignored` | Mechanical | route: *the id comes from `--slug …`, not the title, so pass a distinct `--slug`* | matches |
| Ad5 | `doc create` | committed · `--slug` distinct | `… --title "Use a single node cache" --slug use-a-single-node-cache-2 …` | 0 | none | none (ack) | minted beside; role `decision` bound | matches — the route lands |
| Ad6 | `doc author` | committed · same payload title | `jigc doc author adr --from-file - …` | 0 | none | none (ack) | `op: author` on the copied-in ADR | matches — the four-way write |

## Table 11 · order-of-refusal pairs not carried by a matrix cell (rig D, a `report-inconsistency` task)

| # | door | cell | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| Or1 | `doc create` | gate-blocked doctype + a title that slugs to nothing | `jigc doc create idea --title "!!!" --task <t> --format json` | 1 | `create.gate-blocked` | Mechanical | key `(create.gate-blocked, idea)` | matches the stated ranking (admission outranks the title) |
| Or2 | `doc create` | unknown doctype + a title that slugs to nothing | `jigc doc create nosuch --title "!!!" --task <t> --format json` | 1 | `create.unknown-doctype` | Mechanical | key `(create.unknown-doctype, nosuch)` | matches |
| Or3 | `doc create` | malformed `--slug` + unknown doctype | `jigc doc create nosuch --title "X" --slug BAD_SLUG --task <t> --format json` | 1 | none | Human | `{"error": "`--slug \"BAD_SLUG\"` is not a valid slug — …"}` | a datum: the slug grammar answers first, code-less (axis 1's subject) |
| Or4 | `doc create` | malformed `--slug` + gate-blocked doctype | `jigc doc create idea --title "X" --slug BAD_SLUG --task <t> --format json` | 1 | none | Human | the same `error` | datum |
| Or5 | `doc create` | malformed `--slug` + an empty-slug title | `jigc doc create inconsistency --title "!!!" --slug BAD_SLUG --task <t> --format json` | 1 | none | Human | the same `error` | datum |

## Order of refusals (scope item 1) — as driven

At `doc create`: **the `--slug` grammar** (a code-less `{"error": …}`, `R-N-4`, `R-N-5`, Or3–Or5) → `create.unknown-doctype` /
`create.gate-blocked` (`N-G-*`, Or1, Or2: admission answers even when the title's id exists or the title slugs to nothing) → `create.empty-title` (only
when the id comes from the title: `"!!!"` alone is `empty-title`, `"!!!" --slug <occupied>` is
`create.already-exists`, `N-E-*`, `R-N-2`, `R-N-3`) → **`create.already-exists`** (under `new: true`; ahead of
everything below, whether the role is unbound or bound, `N-C-*`, `N-V-*`, `R-N-1`) → the fixed-title arm of
`write.title-ignored` (singletons, `Sg02`; under `new: true` the gate answers first, `Sh01`) →
`write.title-ignored` → `write.identity-change`. At `doc author` the payload's own refusals sit in front of
all of it: `write.wrong-shape` / `write.malformed-value` outrank `create.unknown-doctype` and
`create.gate-blocked`, which outrank the gate's existence check and the title (Table 7).

Between the last two: with the role bound to doc X and a call that would mint a **different** id, the
answer is `write.identity-change` whether or not that other id is occupied in the general case (`R-G-1`),
and `create.already-exists` under `new: true` when it is occupied (`R-N-1`) — the ranking
`write-commands.md` states. With the role bound to X and a call that lands **on X** under a different
title, the answer is `write.title-ignored` under both entry kinds (`N-S-c2`, `G-S-c2`) — see D-4.

`create.serial-collision` was **not reached** at either create door (un-driven list).

## Repro blocks

Every block starts from `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig";
[ -n "$REPO" ] || exit`. `fill(<t>)` below is four writes on the task's commit doc (`set-field
commit:<t>#type --value docs`, `#scope --value findings`, `set-slot #summary`, `#body`), each exit 0.

### RB-1 · the matrix cells (Table 1) — one block per cell shape

```text
setup    rig B = fresh. Three findings landed first, each through its own workflow and finalize:
           report-inconsistency  → inconsistency:port-mismatch          ("Port mismatch")
           report-jigc-feedback  → jigc-feedback:finalize-sweeps-a-staged-path
           park-idea             → idea:parked-thought                  ("A Parked Thought")
         checksums taken: 5b90656b5d4fce69 · 21ec9172ae2ab6db · f4cbf545a4540d0b
per cell jigc start --workflow <wf> "cell <n> probe"            → task cell-<n>-probe, exit 0
         <the cell's precondition, if any>
           own:       jigc doc create <ty> --title "<T>" --task <t>                  (exit 0)
           copied-in: jigc doc set-field <addr>#meta/<field> --value <v> --task <t>  (exit 0, "copied in for update")
           untracked: a plain file written to $REPO/docs/<home>/<slug>.md, never `git add`ed
         ls $REPO/.jigc/tasks/<t>/docs/      → the before-control: commit:<t>.md provenance.json [+ the precondition's doc]
         <the argv of the row> --format json → exit, findings[0].code, findings[0].key, findings[0].route  | the ack
         ls $REPO/.jigc/tasks/<t>/docs/      → after
         cat $REPO/.jigc/tasks/<t>/roles.json
         jigc task discard <t> --force
author   payload on stdin:  title: "<T>"
                            sections:
                              - id: description
                                set:
                                  description: |
                                    <<Authored by the batch door in cell <n>.>>
observed every exit, code, key and staged delta is the row's; after each block the three checksums above
         were unchanged and `git status --short` showed only the untracked plants.
```

Two refusals in full, one per door (the `(code, target)` key is the same at both):

```text
$ jigc doc create inconsistency --title "Port mismatch" --task second-report-same-port-thing --format json
{ "schema_version": 3, "findings": [ {
    "severity": "blocking", "probe": "create", "check": "already-exists", "code": "create.already-exists",
    "key": { "code": "create.already-exists", "target": "inconsistency:port-mismatch" },
    "message": "`inconsistency:port-mismatch` already exists on disk at its home, and this workflow's `allows-create` entry for `inconsistency` carries `new: true` — it creates a new doc only, so the existing one is never copied in for update",
    "location": { "address": "inconsistency:port-mismatch", "line": 1, "col": 1 },
    "route": "choose a distinct `--title`, or keep this one and pass `--slug <slug>` to mint beside the existing doc: `jigc doc create inconsistency --title <title> --slug <slug> --task second-report-same-port-thing`" } ] }
[exit 1]
$ jigc doc author inconsistency --from-file - --task second-report-same-port-thing      # title: "Port mismatch"
blocking · create.already-exists — `inconsistency:port-mismatch` already exists on disk at its home, …
  at: inconsistency:port-mismatch
  route: set the payload's `title:` to a distinct title (its slug becomes the doc id) and re-run the same `jigc doc author inconsistency --from-file <payload> --task second-report-same-port-thing`
[exit 1]
$ ls $REPO/.jigc/tasks/second-report-same-port-thing/docs
commit:second-report-same-port-thing.md  provenance.json           # before and after: identical
$ shasum -a 256 $REPO/docs/inconsistencies/port-mismatch.md         # 73f3f60ad0c03f5f before and after (rig A)
```

### RB-2 · D-1 — the join's suffix lands on an occupied id and `milestone finalize` overwrites it (rig G, `fresh`)

```text
$ jigc --version
jigc 1.0.0-rc.24

# 1 · an earlier finding is filed and landed
$ jigc start --workflow report-inconsistency "file the stage two finding"   → task file-the-stage-two-finding
$ jigc doc create inconsistency --title "Stage 2" --task file-the-stage-two-finding
inconsistency:stage-2                                                                       [exit 0]
$ jigc doc set-field inconsistency:stage-2#meta/kind --value code-doc --task …              [exit 0]
$ jigc doc set-slot  inconsistency:stage-2#description --from-file - --task …               [exit 0]
     (stdin: "EARLIER-FINDING-MARKER the stage-2 deploy doc and the script disagree.")
  fill(file-the-stage-two-finding)
$ jigc task finalize file-the-stage-two-finding
finalized 9e53c81 — docs(findings): file a finding
  promoted docs/inconsistencies/stage-2.md
  1 file committed                                                                          [exit 0]

# 2 · an untracked, hand-written, conformant note at docs/inconsistencies/stage-3.md
#     (front-matter kind/status/date/schema-version, "# Stage 3", and a Description holding
#      "UNTRACKED-NOTE-MARKER written by hand, never committed.") — written with a plain redirect, not added

# 3 · a fan-out of three reporters, each filing the title "Stage"
$ jigc milestone create "stage reports"                                                     [exit 0]
$ jigc milestone add-task stage-reports "alpha reports the stage"   --workflow report-inconsistency   [exit 0]
$ jigc milestone add-task stage-reports "bravo reports the stage"   --workflow report-inconsistency   [exit 0]
$ jigc milestone add-task stage-reports "charlie reports the stage" --workflow report-inconsistency   [exit 0]
$ jigc milestone provision stage-reports      → provisioned 3 worktree(s) … at base 9e53c81          [exit 0]
$ jigc milestone execute stage-reports        → 3 Spawn: lines                                        [exit 0]
  in each $REPO/.jigc/worktrees/<sub>:
$ jigc workflow report-inconsistency --task <sub>                                           [exit 0]
$ jigc doc create inconsistency --title "Stage" --task <sub>
inconsistency:stage                                                                         [exit 0]   ×3
$ jigc doc set-field inconsistency:stage#meta/kind --value doc-doc --task <sub>             [exit 0]
$ jigc doc set-slot  inconsistency:stage#description --from-file - --task <sub>             [exit 0]
     (stdin: "<alpha|bravo|charlie> reporter: the stage finding.")

# BEFORE-CONTROL (in $REPO)
stage-2.md (committed): sha 836a31f4e9d8e5da · `command grep -c EARLIER-FINDING-MARKER` → 1
stage-3.md (untracked): sha 967f8536d5ffb23f · `command grep -c UNTRACKED-NOTE-MARKER`  → 1
$ git status --short
?? docs/inconsistencies/stage-3.md

# 4 · the join
$ jigc milestone join stage-reports
joined milestone:stage-reports — 6 doc(s) merged
  - commit:alpha-reports-the-stage  (created · from alpha-reports-the-stage)
  - commit:bravo-reports-the-stage  (created · from bravo-reports-the-stage)
  - commit:charlie-reports-the-stage  (created · from charlie-reports-the-stage)
  - inconsistency:stage  (created · from alpha-reports-the-stage)
  - inconsistency:stage-2  (created · from bravo-reports-the-stage)  ← suffixed -2 on collision
  - inconsistency:stage-3  (created · from charlie-reports-the-stage)  ← suffixed -3 on collision
[exit 0]
$ jigc milestone join stage-reports --format json      → exit 0, "findings": []

# 5 · the finalize
$ jigc milestone finalize stage-reports --format json
{ "committed": { "commits": [ { "hash": "97be26e",
      "paths": [ "docs/inconsistencies/stage-2.md", "docs/inconsistencies/stage-3.md",
                 "docs/inconsistencies/stage.md", "docs/milestone-records/stage-reports.md" ],
      "subject": "Finalize milestone stage-reports (3 sub-tasks)" } ],
    "displaced": [], "files": 4,
    "manifest": [ { "kind": "promoted", "path": "docs/inconsistencies/stage-2.md" },
                  { "kind": "promoted", "path": "docs/inconsistencies/stage-3.md" },
                  { "kind": "promoted", "path": "docs/inconsistencies/stage.md" },
                  { "kind": "modified", "path": "docs/milestone-records/stage-reports.md" } ],
    "still_staged": [], … } }
[exit 0]

# AFTER
stage-2.md: sha 8a73bd7e9acbd673 · EARLIER-FINDING-MARKER → 0 · "bravo reporter"   → 1
stage-3.md: sha 933ad31774a62db7 · UNTRACKED-NOTE-MARKER  → 0 · "charlie reporter" → 1
$ git status --short                                   → (empty)
$ git show --stat --format='%h %s' HEAD
97be26e Finalize milestone stage-reports (3 sub-tasks)
 docs/inconsistencies/stage-2.md         |  6 +++---
 docs/inconsistencies/stage-3.md         | 21 +++++++++++++++++++++
 docs/inconsistencies/stage.md           | 21 +++++++++++++++++++++
 docs/milestone-records/stage-reports.md |  8 ++++----
$ git log --all --oneline -S UNTRACKED-NOTE-MARKER | wc -l          → 0
$ command grep -rl UNTRACKED-NOTE-MARKER $REPO                      → (nothing; `.jigc/` included — `command grep`,
                                                                       and the before-control above found the bytes)
```

The first drive of the same shape (rig F, `batch-four`; committed `inconsistency:pair-report-2`, two
sub-tasks minting `pair-report`), with the diff the commit made to the earlier finding:

```text
$ jigc milestone finalize batch-four
finalized 0cd679a — Finalize milestone batch-four (2 sub-tasks)
  promoted docs/inconsistencies/pair-report-2.md
  promoted docs/inconsistencies/pair-report.md
  modified docs/milestone-records/batch-four.md
  3 files committed                                                                         [exit 0]
$ git show --format='%h %s' HEAD -- docs/inconsistencies/pair-report-2.md
-kind: code-doc
+kind: doc-doc
-# Pair report 2
+# Pair report
-EARLIER-FINDING-MARKER-h88 an unrelated finding that happens to own the id pair-report-2.
+BRAVO-FOUR pair report
$ jigc validate --format json        → exit 0; seven advisory `schema-conformance.repeatable-populated` rows, nothing else
```

And under a workflow whose entry carries **no** `new: true` (rig F, `batch-five`, two `--workflow
park-idea` sub-tasks minting `pair-idea`, committed `idea:pair-idea-2`): the same join line
(`idea:pair-idea-2 (created · from bravo-batch-five-parks) ← suffixed -2 on collision`), the same
`promoted docs/ideas/pair-idea-2.md`, exit 0, checksum `20a2c1cfc991dfd9` → `181a2009d2e9b454`, the earlier
idea's marker 1 → 0.

### RB-3 · D-2 — the `write.identity-change` route over a committed doc copied in (rig B; the same in a sub-task, rig F)

```text
setup    fresh + the landed inconsistency:port-mismatch.
$ jigc start --workflow report-inconsistency "role binding probe"          → task role-binding-probe
$ cat $REPO/.jigc/tasks/role-binding-probe/roles.json                      → (no such file)
$ jigc doc set-field inconsistency:port-mismatch#meta/kind --value doc-doc --task role-binding-probe
set inconsistency:port-mismatch#meta/kind = doc-doc (copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)
[exit 0]
$ cat $REPO/.jigc/tasks/role-binding-probe/roles.json
{ "roles": { "inconsistency": "inconsistency:port-mismatch" } }
$ jigc doc create inconsistency --title "A wholly new finding" --task role-binding-probe
blocking · write.identity-change — create rejected: this task's `inconsistency` is already `inconsistency:port-mismatch`, and this call would mint `inconsistency:wholly-new-finding` instead — a second document beside the first, not a correction of it
  at: inconsistency:port-mismatch
  route: `jigc doc rename inconsistency:port-mismatch --to 'A wholly new finding' --task role-binding-probe` moves the doc this task already holds onto the title (and id) you asked for; a genuinely separate second document is its own task — finalize or discard this one first
[exit 1]
$ jigc doc rename inconsistency:port-mismatch --to 'A wholly new finding' --task role-binding-probe     # the route, verbatim
blocking · write.identity-change — rename rejected: `inconsistency:port-mismatch` is committed, so `--to "A wholly new finding"` would move its identity to `wholly-new-finding` — a committed doc's path IS its identity, and referrers outside this task point at the old one. A same-slug retitle of the staged copy is supported; a re-slug is not
  at: inconsistency:port-mismatch
  route: `jigc rename inconsistency:port-mismatch --to 'A wholly new finding'` moves it for real — repointing every committed referrer in one transaction — once this task is finalized or discarded (it is a task-less, self-committing store op)
[exit 1]
```

`--slug` (the `create.already-exists` route's own advice) takes the same two hops (`Rt04`, `Rt05`). The
general case is the same chain (`G-V-c3`, `G-V-a3`, `Rt08`): `doc create idea --title "A Parked Thought"
--slug parked-v` after a copy-in → `write.identity-change` → `jigc doc rename idea:parked-thought --to …
--slug parked-v` → refused → `jigc rename idea:parked-thought …`. How a report sub-task gets there by
running its own step's lines (rig F, `batch-two`):

```text
$ jigc doc create inconsistency --title "Twin report" --task bravo-batch-two-files
blocking · create.already-exists — … route: choose a distinct `--title`, or … `--slug <slug>` …        [exit 1]
$ jigc doc set-field inconsistency:twin-report#meta/kind --value doc-doc --task bravo-batch-two-files  [exit 0]   # the step's next emitted line, at the slug the title minted
$ jigc doc create inconsistency --title "Second twin" --task bravo-batch-two-files                     # the route's distinct title
blocking · write.identity-change — create rejected: this task's `inconsistency` is already `inconsistency:twin-report`, …
  route: `jigc doc rename inconsistency:twin-report --to 'Second twin' --task bravo-batch-two-files` moves the doc this task already holds …
[exit 1]
```

### RB-4 · D-3 — two open report tasks mint one id; the second's routes (rig B)

```text
$ jigc start --workflow report-inconsistency "other open holder"            → task other-open-holder
$ jigc doc create inconsistency --title "Shared thing" --task other-open-holder       → inconsistency:shared-thing [exit 0]
  (kind set; description "HOLDER-MARKER-a11 the first reporter.")
$ jigc start --workflow report-inconsistency "cell 64 probe"                → task cell-64-probe
$ jigc doc create inconsistency --title "Shared thing" --task cell-64-probe --format json
  → exit 0 · op create · target inconsistency:shared-thing · existed false          # neither home is on disk yet
  (kind set; description "SECOND-MARKER-b22 the second reporter.")
  fill(other-open-holder)
$ jigc task finalize other-open-holder            → promoted docs/inconsistencies/shared-thing.md · exit 0
$ shasum …/shared-thing.md → 6b8c09cae84d188a ; command grep -c HOLDER-MARKER-a11 → 1
  fill(cell-64-probe)
$ jigc task validate cell-64-probe                → one advisory (file-state.staged-copy) · exit 0
$ jigc task finalize cell-64-probe
blocking · finalize.base-mismatch — the task was started at base `da01bb3…` but HEAD is now `36b570e…`, and the moved history overlaps the task's work on `docs/inconsistencies/shared-thing.md`
  at: task:cell-64-probe
  route: resolve the overlap on `docs/inconsistencies/shared-thing.md` against the new history, or discard the task with `jigc task discard cell-64-probe --force`
[exit 3]                                           # shared-thing.md: 6b8c09cae84d188a, HOLDER-MARKER-a11 → 1 (unchanged)
$ jigc doc create inconsistency --title "Shared thing" --task cell-64-probe          # the create re-run
blocking · create.already-exists — `inconsistency:shared-thing` already exists on disk at its home, …
  at: inconsistency:shared-thing
  route: this task already holds `inconsistency:shared-thing` (its `inconsistency`), and a task carries one doc per role — a distinct `--title` or a `--slug` would mint a second one beside it, and a second doc in one task is refused too. Finish this task first: land it with `jigc task finalize cell-64-probe`, or abandon it with `jigc task discard cell-64-probe --force`; then file the next one in its own task: `jigc start --workflow report-inconsistency "<intent>"`
[exit 1]
# the exit neither route names:
$ jigc doc rename inconsistency:shared-thing --to "Shared thing" --slug shared-thing-2 --task cell-64-probe
inconsistency:shared-thing-2 (renamed to "Shared thing" from inconsistency:shared-thing)               [exit 0]
$ jigc task finalize cell-64-probe                → promoted docs/inconsistencies/shared-thing-2.md · exit 0
$ command grep -c HOLDER-MARKER-a11 …/shared-thing.md → 1 ; command grep -c SECOND-MARKER-b22 …/shared-thing-2.md → 1
```

### RB-5 · D-4 — `write.title-ignored` under a `new: true` entry (rig B)

```text
$ jigc start --workflow report-inconsistency "cell 28 probe"
$ jigc doc create inconsistency --title "Own thing" --task cell-28-probe            → inconsistency:own-thing [exit 0]
$ jigc doc create inconsistency --title "Own Thing!" --task cell-28-probe --format json
  → exit 1 · code write.title-ignored · key (write.title-ignored, inconsistency:own-thing)
    route: `jigc doc rename inconsistency:own-thing --to 'Own Thing!' --task cell-28-probe` retitles the doc in place; then re-run this write unchanged
$ jigc doc author inconsistency --from-file - --task cell-33-probe --format json    # same precondition, payload title "OWN thing?"
  → exit 1 · code write.title-ignored · key (write.title-ignored, inconsistency:own-thing)
```

The route runs (`Rt01`). Against: `design/findings-channel.md` §10's `write.title-ignored` row — *"unchanged
— reachable only under an entry without `new: true`"*, Workflows *"all but the report workflows"* — §4
*"Inside a report task this arm cannot arise"*, and flow 57's *"never `write.title-ignored`, which cannot
arise under a `new: true` entry"*.

### RB-6 · D-5 — the bare entry form (rig D)

```text
shadow   $REPO/.jigc/config/workflows/report-inconsistency.yaml = the shipped file with its entry replaced by
           allows-create:
             - inconsistency
         git add + git commit (plain git)                                                    [exit 0]
$ jigc start --workflow report-inconsistency "probe 350"
blocking · workflow-refs.malformed-front-matter — workflow front-matter is malformed config-family YAML: allows-create[0]: invalid type: string "inconsistency", expected struct AllowsCreate at line 7 column 5
  route: fix the workflow/step/catalog definition the message names (a definition defect, repaired once at its source), then re-run
[exit 1]
shadow   the entry as  - { type: inconsistency, new: true }
$ jigc start --workflow report-inconsistency "probe 351"
blocking · workflow-refs.malformed-front-matter — … allows-create[0]: missing field `as` at line 7 column 5     [exit 1]
```

Against `design/write-commands.md` → *The create-gate*: *"`allows-create: [<doctype-id>, ...]`"* and *"Two
entry forms. Each `allows-create` entry is either a bare doctype id (`adr`) or an object … The bare form
grants create permission without declaring a role."* (`design/workflow-dialect.md` states the form the
binary accepts: *"The entry is `{type, as, new}` and nothing else."*)

### RB-7 · D-6 — the singleton arm's route (rig C, `committed-singletons`)

```text
shadow   $REPO/.jigc/config/workflows/form-vision.yaml = the shipped file, its entry { type: vision, as: vision, new: true }; committed with plain git
$ jigc start --workflow form-vision "vision route check"                      → task vision-route-check
$ jigc doc create vision --title Vision --task vision-route-check
blocking · create.already-exists — `vision:vision` already exists on disk at its home, and this workflow's `allows-create` entry for `vision` carries `new: true` — …
  at: vision:vision
  route: `vision` has a fixed identity — one instance at one home, whatever the title or `--slug` — so no distinct identity exists for this create to mint, and this workflow creates a new `vision` only; changing the existing one is the work of a workflow whose `allows-create` entry for `vision` does not carry `new: true`. If this task holds nothing else, `jigc task discard vision-route-check` abandons it
[exit 1]
$ jigc task discard vision-route-check                                         # the route, verbatim
blocking · task-discard.staged-prose — task `vision-route-check` stages 1 doc(s) that no commit has a copy of — discarding it would destroy them: commit:vision-route-check
  route: read what is in them with `jigc doc show <address> --task vision-route-check`, or land them with `jigc task finalize vision-route-check` (which refuses while a required slot is empty) — or, once you have confirmed the task holds nothing you need, `jigc task discard vision-route-check --force` removes the working area with them
[exit 1]
```

`jigc task discard --help` says why it can never run as routed: *"`jigc start` stages the task's
`commit:<id>` doc at mint, so an ordinary discard needs this [`--force`] from the moment the task exists."*
The sibling bound-role route spells `jigc task discard <t> --force`.

### RB-8 · D-7 — a dangling symlink at the minted home (rig B)

```text
setup    ln -s nowhere.md $REPO/docs/inconsistencies/dangling-home.md          (nowhere.md does not exist)
$ jigc start --workflow report-inconsistency "edge finalize dangling-home"
$ jigc doc create inconsistency --title "Dangling home" --task edge-finalize-dangling-home
inconsistency:dangling-home                                                                  [exit 0]
  (kind, description, commit doc filled)
$ jigc task finalize edge-finalize-dangling-home
  promoted docs/inconsistencies/dangling-home.md
  1 file committed
  left-out (…): … docs/inconsistencies/nowhere.md …                                          [exit 0]
$ git ls-tree HEAD docs/inconsistencies/
120000 blob c5f140c8…	docs/inconsistencies/dangling-home.md
$ git cat-file -p HEAD:docs/inconsistencies/dangling-home.md
nowhere.md
$ git status --short
?? docs/inconsistencies/nowhere.md                    # holds the finding: "# Dangling home" …
$ jigc doc show inconsistency:dangling-home           → serves the finding, exit 0
```

A symlink to an existing file at the home is refused by the gate (`N-L-c1`, `create.already-exists`).

### RB-9 · D-8 — what the load error does not name (rig D)

```text
shadow   report-inconsistency's entry as { type: inconsistency, as: inconsistency, nwe: true }; committed
$ jigc start --workflow park-idea "another unrelated one"                     # a different, healthy workflow
blocking · workflow-refs.malformed-front-matter — workflow front-matter is malformed config-family YAML: allows-create[0]: unknown field `nwe`, expected one of `type`, `as`, `new` at line 7 column 47
  route: fix the workflow/step/catalog definition the message names (a definition defect, repaired once at its source), then re-run
[exit 1]
$ jigc start --workflow park-idea another-unrelated-one --format json
{ "error": "blocking · workflow-refs.malformed-front-matter — workflow front-matter is malformed config-family YAML: allows-create[0]: unknown field `nwe`, … at line 7 column 47\n  route: fix the workflow/step/catalog definition the message names …" }
[exit 1]
$ jigc validate --format json    → exit 0 · key (workflow-refs.malformed-front-matter, workflow:report-inconsistency)
```

The same text, with no `at:` line and no workflow id or file path, at `jigc start` (bare, `--workflow <any>`,
`--task`), `jigc workflow <any> --preview`, `jigc describe`, `jigc doc create` / `doc author` in a task of
that workflow, `jigc task validate` and `jigc task finalize --dry-run` (Table 6, S05–S10).

### RB-10 · D-9 — the general-case `write.identity-change` route onto an occupied id (rig B)

```text
$ jigc start --workflow park-idea "route check rg1"
$ jigc doc create idea --title "Mine" --task route-check-rg1                  → idea:mine [exit 0]
$ jigc doc create idea --title "A Parked Thought" --task route-check-rg1      # a committed idea's title
blocking · write.identity-change — … route: `jigc doc rename idea:mine --to 'A Parked Thought' --task route-check-rg1` moves the doc this task already holds onto the title (and id) you asked for; …
[exit 1]
$ jigc doc rename idea:mine --to 'A Parked Thought' --task route-check-rg1    # the route, verbatim
blocking · write.already-present — rename rejected: the committed store already holds `idea:parked-thought` at `docs/ideas/parked-thought.md` — landing this task's doc under that identity would overwrite it at finalize
  at: idea:parked-thought
  route: give this doc an id nothing else answers to — re-run `jigc doc rename idea:mine --to 'A Parked Thought' --slug <other-slug>`; or if `idea:parked-thought` is the doc you meant to work on, read it with `jigc doc show idea:parked-thought` and edit that one instead of minting a second under its identity
[exit 1]
```

### RB-11 · D-10 — the composed text under a shadow that drops the key (rig D)

```text
shadow   report-inconsistency's entry as { type: inconsistency, as: inconsistency }; committed
$ jigc start --workflow report-inconsistency "compose under dropped key"        → exit 0; lines 8–9 of the composed text:
  This task files exactly one NEW record. A title whose slug a doc on disk already
  holds is refused `create.already-exists`, with nothing staged — choose a distinct
$ jigc doc create inconsistency --title "Port mismatch" --task <t> --format json
  → exit 0 · op create · target inconsistency:port-mismatch · existed true                  # copied in for update
```

### RB-12 · the declared bound, measured (Table 3; rig A) and reached through a fan-out (rig F)

```text
$ jigc doc create inconsistency --title "Port mismatch" --task <t2>           → create.already-exists · exit 1
$ jigc doc set-field    inconsistency:port-mismatch#meta/kind --value doc-doc --task <t2>          [exit 0]  (copied in for update — …)
$ jigc doc set-slot     inconsistency:port-mismatch#description --from-file - --task <t2>          [exit 0]  (stdin: "OVERWRITTEN by the second report task.")
$ jigc doc add-item     inconsistency:port-mismatch#sides --title "docs/extra.md" --slug extra --task <t2>   [exit 0]
$ jigc doc remove-item  inconsistency:port-mismatch#sides/readme --task <t2>                       [exit 0]
$ jigc doc retitle-item inconsistency:port-mismatch#sides/server --title "src/main.rs" --task <t2> [exit 0]
$ jigc doc rename       inconsistency:port-mismatch --to "PORT mismatch!" --task <t2>              [exit 0]
$ jigc task finalize <t2>
finalized 5c53be7 — docs(findings): file a finding
  promoted docs/inconsistencies/port-mismatch.md
  promoted docs/inconsistencies/third-thing.md
  2 files committed                                                                               [exit 0]
$ shasum … port-mismatch.md   73f3f60ad0c03f5f → f745a70d84f0244b
  # H1 "PORT mismatch!" · kind doc-doc · sides src/main.rs {#server}, docs/extra.md {#extra} · description "OVERWRITTEN by the second report task."
```

### RB-13 · the fan-out's two filing orders (Table 8, F04–F09; rig F)

```text
batch-one    alpha files "Twin report" FIRST, bravo second
$ jigc milestone join batch-one
  - inconsistency:twin-report  (created · from alpha-batch-one-files)
  - inconsistency:twin-report-2  (created · from bravo-batch-one-files)  ← suffixed -2 on collision      [exit 0]
$ jigc milestone finalize batch-one    → promoted …/twin-report-2.md, …/twin-report.md · 3 files committed · exit 0
  twin-report.md: "ALPHA-ONE filed first" · twin-report-2.md: "BRAVO-ONE filed second"
batch-three  bravo files "Third twin" FIRST, alpha second
$ jigc milestone join batch-three
  - inconsistency:third-twin  (created · from alpha-batch-three-files)
  - inconsistency:third-twin-2  (created · from bravo-batch-three-files)  ← suffixed -2 on collision     [exit 0]
$ jigc milestone finalize batch-three  → exit 0 · third-twin.md: "ALPHA-THREE filed second" · third-twin-2.md: "BRAVO-THREE filed first"
```

## Findings

Tier predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying or
moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the binary
does not do.

### (R6, D-1) · the join's collision suffix is minted onto an id already on disk, and `milestone finalize` overwrites the doc there at exit 0 — **proposed tier 1**

- **Doors:** `jigc milestone join` (mints the occupied id, `findings: []`) → `jigc milestone finalize` (the
  committing door that overwrites).
- **Cell:** N sub-tasks mint one slug in isolation × `<slug>-2` … `<slug>-N` already a doc at the
  doctype's home — committed, or an untracked file.
- **Exit:** 0 at both. **Code:** none. Rows F11–F15; repro RB-2.
- **Loss shown:** a committed finding's content replaced (`EARLIER-FINDING-MARKER` 1 → 0; recoverable only
  from history), and an untracked file's content replaced (`UNTRACKED-NOTE-MARKER` 1 → 0; afterwards in no
  commit, not displaced, and nowhere under `$REPO`). The ack says `promoted`; `displaced: []`; `jigc
  validate` afterwards is clean of it.
- **Contract contradicted:** `design/findings-channel.md` §4 (*the defect: … overwrites the earlier
  finding* — the loss the create-only entry exists to stop; *"`new: true` does not change fan-out join
  suffixing: two sub-tasks that mint one slug in isolation still land `-2` at the join"*, which here is a
  landing **on** a filed finding) and §1 (*append-only is free at the doc level*); `design/storage.md` →
  the by-task-id join, rule 4 (*colliding new instances … are distinct docs, not a clash*); and the
  promote clobber guard the single-task door carries (`write-commands.md` → `jigc doc rename` bounds:
  *"the write refuses exactly what `finalize.promote-clobber` refuses"* — at the milestone door nothing
  refused).
- **Why tier 1:** exit 0, a committing door, bytes lost — one of the two victims unrecoverably.
- **Reach:** not specific to `new: true` (F14 reproduces it under `park-idea`), so it predates M55; M55's
  findings channel is the workload that makes it likely — many reporters, one doc per finding,
  title-minted ids, and titles that end in a number (`Stage 2`, `Phase 2`, `HTTP 2`) are ordinary. The
  create doors, the `doc rename` door (A12, A13) and the single-task finalize (Rt13) all refuse the same
  collision; the join is the one door that does not look.

### (R6, D-2) · `write.identity-change`'s route, when the role is bound to a **committed doc copied in**, names a `doc rename` that refuses, whose own route is the whole-repo rename of the existing doc — **proposed tier 2**

- **Doors:** `doc create`, `doc author` (the route's producer) → `doc rename` (refuses) → `jigc rename`
  (the second hop named). Rows `N-V-c3`, `G-V-c3`, `G-V-a3`, Rt03–Rt06, Rt08, F17; repro RB-3.
- **Exit:** 1, 1. **Code:** `write.identity-change` at both hops.
- **Why tier 2:** a route dead end. Hop 1 says the rename *"moves the doc this task already holds onto the
  title (and id) you asked for"*; it never does for a committed copy (`write-commands.md` → `jigc doc
  rename`: retitle-only). Hop 2 routes a reporter who wanted a **new** doc at renaming the **existing**
  one — someone else's work, the misroute M55's F3 removed from `write.title-ignored`
  (`write-commands.md`: *"that doc is someone else's work, so renaming it is the wrong correction"*). The
  M55 verdict's *"the identity-change route under `new: true` was checked and runs"* holds for a doc the
  task minted (A11) and not for this arm.
- **How a report task gets there:** an edit leaf's first touch of a committed doc binds the create role
  (Rt03 — M45's *a copy-on-write binds the role*). A reporter whose create was refused and who then runs
  the step's next emitted line at the slug the title minted is in this state (F16, F17). From it no create
  can succeed in the task, and neither route says so; the exits that work are `task discard --force` or
  finalizing the edit.

### (R6, D-3) · two open report tasks mint one id: after the first lands, the second's `create.already-exists` route names an exit that refuses and an exit that destroys, and no surface names the one that works — **proposed tier 2**

- **Doors:** `doc create` (route producer), `task finalize` (refuses). Rows Rt11–Rt15; repro RB-4.
- **Exit:** 1 (`create.already-exists`), then 3 (`finalize.base-mismatch`) on the route's first exit.
- **Why tier 2:** a route dead end. The bound-role route's premise — *"a distinct `--title` or a `--slug`
  would mint a second one beside it"* — is false in this state: the doc the task holds **is** the colliding
  identity, and re-slugging it (`jigc doc rename <addr> --to "<title>" --slug <new>`) is exactly what
  lands it (Rt15). The route instead offers `task finalize` (exit 3, whose own route is *"resolve the
  overlap … against the new history"*, no command) and `task discard --force` (the second finding's prose
  goes). `task validate` previews none of it (Rt12, exit 0).
- **No loss:** the landed finding is untouched at every step.

### (R6, D-4) · `write.title-ignored` arises under a `new: true` entry, which `findings-channel.md` says it cannot — **proposed tier 3**

- **Doors:** `doc create`, `doc author`. Rows `N-S-c2`, `N-S-c4`, `N-S-a2`, Rt01, Rt02; repro RB-5. Exit 1.
- **Why tier 3:** a design-of-record sentence the binary does not do (§10's row *"reachable only under an
  entry without `new: true`"*, its Workflows column *"all but the report workflows"*; §4 *"Inside a report
  task this arm cannot arise"*; `worked-examples.md` flow 57). The binary's behaviour is the one
  `write-commands.md` states (a dropped title over a doc the task **stages** routes at `doc rename`), the
  route runs, and nothing is lost — the defect is the sentence.

### (R6, D-5) · the bare `allows-create` entry form `write-commands.md` documents is refused at load — **proposed tier 3**

- **Door:** every door that loads the workflow (driven at `start`). Row S17; repro RB-6. Exit 1,
  `workflow-refs.malformed-front-matter`.
- **Why tier 3:** `write-commands.md` → *The create-gate* states two entry forms and what the bare one
  grants; the binary accepts only the object form with `as` present. `workflow-dialect.md` agrees with the
  binary, so the two designs disagree with each other.

### (R6, D-6) · the singleton arm of `create.already-exists` routes at `jigc task discard <id>`, which is refused as written — **proposed tier 3**

- **Doors:** `doc create`, `doc author` (route producer) → `task discard`. Rows Sh01, Sh02, Rt17; repro RB-7.
- **Why tier 3:** the route says the command *"abandons it"*; run verbatim it exits 1
  `task-discard.staged-prose` (every task stages its commit doc at mint), and the working spelling
  (`--force`) arrives one hop later. Reachable only under a project shadow that puts `new: true` on a
  singleton's entry — no shipped workflow does.

### (R6, D-7) · a dangling symlink at the minted home: the gate passes it, and `task finalize` exits 0 committing the symlink while the finding lands untracked — **proposed tier 3**

- **Doors:** `doc create` (passes), `task finalize`. Rows `N-L-c2`, E04, E05; repro RB-8.
- **Why tier 3, and not 1:** the ack says `promoted … 1 file committed`; the commit holds a `120000` entry
  whose target is outside it. **No pre-existing byte is lost or damaged** — the strict tier-1 showing (a
  before-control, then absence) does not exist here; the harm is that the committed record lacks the
  finding while every local read serves it. **Outside this row's subject** in two ways, stated so it is
  weighed as such: the precondition is planted (a dangling link at exactly the minted path), and symlinked
  paths are numbered axis 1's class, which this row does not re-open. It is recorded because the scope's
  identity axis (*a file placed at its home*) led there and it was driven.

### (R6, D-8) · the unknown-key load error names the key and not the workflow, and its route sends the reader to *"the definition the message names"* — **proposed tier 3**

- **Doors:** `start`, `workflow`, `describe`, `doc create`, `doc author`, `task validate`, `task finalize`.
  Rows S05–S10; repro RB-9. Exit 1.
- **Why tier 3:** the route's referent is absent from the message — no workflow id, no file path, no `at:`
  line — and the refusal reaches doors asked about a *different* workflow (S06). `jigc validate` is the one
  surface that names it, in its `target`. Under `--format json` the refusing doors emit the text finding
  inside `{"error": …}`, so the `(code, target)` key is not readable there either — recorded as a datum
  for row 7, whose funnel registry this is. §10's own cell (*a load error naming the key*) is met.

### (R6, D-9) · general case: `write.identity-change`'s route names a `doc rename` onto an **occupied** committed id, which refuses — **proposed tier 3**

- **Doors:** `doc create`, `doc author` → `doc rename`. Rows `R-G-1`…`R-G-3`, Rt09; repro RB-10. Exit 1, 1.
- **Why tier 3:** the route states a move the binary refuses (`write.already-present`); the second route is
  sound and lands, so it is a two-hop, not a dead end. Under `new: true` this was closed by ranking
  `create.already-exists` first (`write-commands.md`, P3–P6); the entries without the key keep it.

### (R6, D-10) · under a project shadow that drops `new: true`, the composed step still says the create is refused — **proposed tier 3**

- **Door:** `start` (composed text). Row S03 against `S1-c1`; repro RB-11.
- **Why tier 3:** the printed step states `create.already-exists … with nothing staged`; in that
  configuration the create copies the existing finding in at exit 0. The sentence is the pack step's and
  the entry is the shadow's, so a project that drops the key gets a true guard removed and a false line
  kept. Low weight: it needs a deliberate project-layer edit.

### Observations — not filed as defects

- **O-1 · the re-run ack's text.** A create re-run over the task's **own fresh** doc prints `(already
  existed — copied in for update)` while its JSON says `existed: true, copied_in: false` and no committed
  doc exists (A02, `N-S-c1`, F05). `command-output-contract.md` defines `existed: true` as *a committed doc
  already occupied the slug's canonical home*; `jigc doc author --help` declares this very text for this
  very case. Declared, so not filed; under a `new: true` step that says *files exactly one NEW record* it
  reads oddly.
- **O-2 · an untracked conformant file at the home, under an entry without the key,** is copied in for
  update like a committed one (`G-U-c1`, `existed: true`), and the task's finalize then replaces a file git
  never held (E01). The overwrite was an explicit `set-slot` on a copy whose ack said it existed, i.e. the
  declared create-or-update — but the replaced bytes are in no commit. Under `new: true` the same file is
  refused (`N-U-*`).

## NOT DRIVEN — every (door, cell) pair left out, and why

| # | door | cell | why not driven |
|---|---|---|---|
| U1 | `doc create`, `doc author` | a **case-sensitive** filesystem: a title, or a hand-placed file name, differing only in case | this host is case-insensitive; Docker did not answer and no Linux build of the registry binary was at hand. The `K` cells are facts about APFS only |
| U2 | `doc create`, `doc author` | `create.serial-collision` in the order of refusals | **not reached** in any cell: every shipped entry is object-form and binds its role, the same-identity staged copy acks `existed`, and the bare form that would leave a role unbound does not load (D-5). Its `jigc migrate` producer is outside this row |
| U3 | `doc author` | the **migration's recorded `--slug`** arm of the `create.already-exists` route (`write-commands.md`: *"discarding the task and re-running `jigc migrate … --slug <slug>`"*) | needs a `migrate-*` shadow carrying `new: true` plus a foreign source; not built |
| U4 | `doc author` | `--slug <distinct>` and `--slug <occupied>` title cells | n/a — the verb has no `--slug` |
| U5 | `doc create`, `doc author` | `report-jigc-feedback` × {untracked · own-staged · copied-in · other open task} | the feedback entry was driven at absent and committed only (7 rows); the other identities were driven on `report-inconsistency`, whose entry differs in doctype and role alone |
| U6 | `doc create`, `doc author` | `record-decision`, `single-task` × {untracked · own-staged · copied-in · other open task · role bound} | committed × title cells only (Table 10); `park-idea` carries the full general-case matrix |
| U7 | `doc create`, `doc author` | the other 23 shipped workflows whose entries carry no `new` | six driven (`park-idea`, `record-decision`, `single-task`, `form-vision`, `planning`, `record-change`) |
| U8 | `milestone join`, `milestone finalize` | `report-jigc-feedback` sub-tasks; `finalize.fan-out.squash = false`; a sub-task filing through `doc author` | the fan-out was driven with `report-inconsistency` and `park-idea` sub-tasks, `doc create`, squash `true` |
| U9 | `milestone join` | D-1 with the suffixed home occupied by a **foreign** (non-conformant) file, a directory or a symlink | committed and untracked-conformant victims only |
| U10 | `doc author` | the entry-form and value-variant shadows (S14–S19) at the author door | driven at `start` and `doc create`; `doc author` was driven under the dropped, added and misspelt shadows |
| U11 | all | a **team-layer** shadow of the entry | project layer only |
| U12 | `doc create`, `doc author` | a symlink to an existing doc at the home under an entry **without** the key (copy-in and promote through a link) | numbered axis 1's class; one `new: true` cell driven (`N-L-c1`) |
| U13 | `doc create`, `doc author` | `--format human` | `agent` text and `json` only |
| U14 | `task bind`, `milestone add-from-spec`, `migrate`, `relocate`, `rename`, `doc schema` (the `DOCTYPE_DOORS` rows outside the write surface's create subject) | — | not this row's subject; no row here confers coverage on them |

## Baseline rows: CLOSED / STILL-OPEN

**None — this row has no baseline; it is a first drive.**

The scope's one context note, dispositioned so it is not read as a new finding: *any task may
`set-field`/`set-slot`/`remove-item` on a committed doc whatever its workflow's `allows-create`* (the edit
gate is parked, `design/findings-channel.md` §1.5).

| key | status | datum |
|---|---|---|
| the declared edit-gate bound (§1.5; M55 baseline C7 / F14) | **STILL-OPEN (expected) — HOLDS AS DECLARED** | Table 3, RB-12. Where it sits on rc.24: after a `create.already-exists` refusal the same report task reaches the existing finding through **six** leaves — `set-field`, `set-slot`, `add-item`, `remove-item`, `retitle-item`, `doc rename` (same-slug retitle) — three more than §1.5 names; the first touch says `(copied in for update …)`, and the doc-only `task finalize` (X09) and the `milestone finalize` of a sub-task (F16) land the rewrite at exit 0. A re-slug of the existing doc is refused (X06). The two create doors do not reach it under `new: true` in any cell driven |

## Leads for other rows — driven here, owned elsewhere, not graded

- **L-1 → row 5.** A plain report task's doc-only commit landed on the branch between `milestone provision`
  and `milestone finalize` wedges the milestone: `finalize.base-mismatch`, exit 3, *"the commits landed
  since move more than milestone-record bookkeeping"*, routed at out-of-band git (F18). `team-ready-state.md`
  declares the guard; a doc-only commit moves no code, which is the argument that widened the guard for
  record commits.
- **L-2 → row 5.** The doc-only left-out narration lists **untracked** paths under *"a staged path stays
  staged for the task it belongs to"* (rig B, every finalize after the untracked plants).
- **L-3 → rows 5 and 7.** A directory named `<slug>.md` at a doc home: `task finalize` exits 1 code-less
  (E03), and `jigc doc list <type>` exits 1 code-less for the whole doctype (E06).
- **L-4 → row 3.** `schema-conformance.unadopted-instance` says *"committed file"* of an untracked one (seen
  at `task validate` in E02).
- **L-5 → row 8.** With a workflow shadow that fails to load, `milestone add-task <m> "<intent>" --workflow
  <that workflow>` exits 0 and commits a record row for a sub-task no door can compose (S13).

## Coverage

`doors_covered` — every clap leaf that is the door of at least one driven row above (`VERB_KINDS`
spelling): `start` · `workflow` · `describe` · `validate` · `doc create` · `doc add-item` · `doc
remove-item` · `doc retitle-item` · `doc rename` · `doc set-field` · `doc set-slot` · `doc author` · `doc
show` · `doc list` · `task validate` · `task discard` · `task finalize` · `config get` · `config list` ·
`milestone create` · `milestone add-task` · `milestone list-tasks` · `milestone provision` · `milestone
execute` · `milestone join` · `milestone finalize` — **26 of 48**. All 8 `doc` × `Write` leaves are among
them. `task discard --force` and `milestone discard --force` were also run as rig cleanup between cells;
those runs are not rows and confer nothing.

---

# Reconciliation — row 6 · write surface (rc.24)

Everything above this rule is the driver's record, unchanged except the eight row annotations marked
**RECONCILER** (three corrected `code` cells, one demoted row, four demoted sub-argv). Everything below is
the reconciler's: it did not author the driver's record and did not build the code.

- **Binary:** `~/.local/bin/jigc`; `jigc --version` printed `jigc 1.0.0-rc.24` before the first drive and
  inside every script (each one asserts it and stops otherwise). Release posture.
- **Source pass:** present (`axis6-codex.md`, its `.rc` holds `0`), read at tag `jigc-v1.0.0-rc.24`. It
  carries **one defect claim** and fifteen *carried / consistent* statements; each is entered below as a lead
  and driven.
- **Rigs:** six, each `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n
  "$REPO" ] || exit` — two steps, stdout only. `fresh` ×5 (r1 matrix and routes · r2 fan-out · r3 shadows ·
  r5 the toggler race · r6 the finalize race), `committed-singletons` ×1 (r4). No teardown, no recursive
  removal; the only removals are `rm -f` / `rmdir` of one literal planted file each. Nothing was written into
  the gitignored workbench; the hand-written files under `.jigc/` are the project-layer workflow shadows
  (`.jigc/config/workflows/<id>.yaml`, committed with plain git), which the scope grants. Under `.jigc/`
  every search is `command grep` with a before-control.
- **`CLAUDECODE` was set** throughout; held constant, not a subject of this row.
- **The working repository was not touched** — `git status --short` empty and `HEAD` unmoved after the last
  drive.
- **Reading a refusal under `--format json`:** a blocked write's findings envelope is on **stderr** with
  stdout empty — the contract's *reject* class (`command-output-contract.md` → Stream discipline). The
  reconciler's first finalize-race run read stdout only and classified every refusal as non-JSON; that run
  is **not counted** below and is named where it matters (RR-3).

## Door-set count — the driver's counts against the registries

Re-read at the checkout the driver read (`bffa6667`; nothing under `crates/` past the release tag).

| registry | driver | reconciler | verdict |
|---|---|---|---|
| `VERB_KINDS` rows · `doc` × `Write` | 48 · 8 | 48 (36 `Write`, 12 `Read`) · 8 — `create`, `add-item`, `remove-item`, `retitle-item`, `rename`, `set-field`, `set-slot`, `author` | agrees |
| `DOCTYPE_DOORS` | 16 | 16 — `migrate`, `relocate`, `rename` · eleven `doc` leaves · `task bind` · `milestone add-from-spec` | agrees |
| `SLUG_DOORS` | 6 | 6 — `start`, `migrate`, `rename`, `doc create`, `doc add-item`, `doc rename` | agrees |
| `MINT_DOORS` | 6 | 6 | agrees |
| `allows-create` entries | 34 over 29 of 39; 2 `new: true`; 2 empty lists | 34 over 29 of 39; `new: true` on `report-inconsistency` and `report-jigc-feedback` only; `amend`, `quick-fix` empty | agrees |
| `AllowsCreate` keys | 3 | 3 — `type`, `as` (required), `new` (defaulted) | agrees |
| `create.*` quoted literals | 6 = 5 codes + 1 fence identifier | 6 — `create.singleton-copy-in` occurs once, as `const SINGLETON_COPY_IN_CODE` in `crates/cli/src/pack.rs` | agrees |
| `PayloadReject::ALL` | 4 | 4 | agrees |
| `doors_covered` | 26 of 48 | 26 — each named leaf is the door of at least one row above | agrees; the reconciler's rows add `doc schema` (27) |

## Rows marked driven that carried no repro block

The rule: a row marked driven with no repro block is **not driven**. Of the driver's 224 table rows, **92
carried no block from which their precondition and argv could be re-run** — the driver's thirteen blocks
cover Table 1's absent / committed / untracked / own-staged / copied-in shapes (RB-1), Table 3 (RB-12) and
the defect rows, and nothing else:

- Table 1 — the cells whose precondition RB-1 does not template: `C082`, `C083` (a directory, a live
  symlink at the home) · `C085`–`C091`, `C093`–`C095` (a role already bound) · `C100`–`C105` (the *key
  added* shadow) · `C106`–`C108` (general case, staged in another open task) · `C109`–`C111` (case) — 24.
- Table 2 `A05`–`A13` (9) · Table 4 `Rt01`, `Rt02`, `Rt07`, `Rt10`, `Rt16` (5) · Table 5 `Sg01`–`Sg09`,
  `Sh02`–`Sh05` (13) · Table 6 `S01`, `S02`, `S04`, `S07`–`S10`, `S12`–`S16`, `S18`, `S19` (14) · Table 7
  `P01`–`P06` (6) · Table 8 `F01`, `F05`–`F07`, `F10`, `F18` (6) · Table 9 `E01`–`E03`, `E06` (4) · Table 10
  `Ad1`–`Ad6` (6) · Table 11 `Or1`–`Or5` (5).

**The reconciler re-drove 91 of the 92 rows** (register RR-12, with the blocks it stands on), four of them
short of one sub-argv; each came back with the exit and code the driver recorded, except the three `code`
cells corrected below. Those rows stand as driven **on the reconciler's blocks**, not on the driver's word.
What stays demoted:

| row | status | why |
|---|---|---|
| `C103` (`S2-a1` · key added · `doc author`, same title) | **DEMOTED — NOT DRIVEN** | no repro block, and the reconciler drove the neighbouring `C104` (`doc author`, different title — exit 1 `create.already-exists`), not this argv |
| `Sh02`, second payload (`Other title`) | **sub-argv DEMOTED** | the reconciler drove the `Vision` payload only (exit 1 `create.already-exists`) |
| `Sh05`, the `roadmap` author argv | **sub-argv DEMOTED** | create driven (exit 1); author not re-driven |
| `S10`, `jigc workflow report-inconsistency --preview` | **sub-argv DEMOTED** | `jigc workflow park-idea --preview` and `jigc describe` re-driven (exit 1 each) |
| `S14`, the `note: hello` shadow on `park-idea` | **sub-argv DEMOTED** | the `extra: 1` shadow re-driven (exit 1, *unknown field `extra`*) |

**Row corrections — three `code` cells that are not their argv's.** `C046`, `C105` and `C108` each record
**exit 0** beside a blocking code. Re-driven, each argv is a plain ack with no finding:

| row | the driver's `code` cell | re-driven | what the cell is |
|---|---|---|---|
| `C046` | `finalize.base-mismatch` at `task:cell-64-probe` | `doc create` exit 0, `existed: false`, no finding (RR-6) | the code of the **later** `task finalize` of that kept-open task (the driver's own RB-4 shows it) |
| `C105` | `create.already-exists` at `idea:parked-thought` | `doc create idea --title "Entirely fresh"` under the added key: exit 0, `existed: false`, staged `idea:entirely-fresh`, no finding | carried over from the previous cell (`C104`) |
| `C108` | `finalize.base-mismatch` at `task:cell-401-probe` | `doc author` exit 0, no finding | another cell's later finalize (a different task id) |

No finding of the driver's rests on any of the eight annotated rows.

## Reconciliation ledger

### A · the source pass's claims — `lead(codex, …)` → driven

| # | `lead(codex, <claim>)` | status | repro / datum |
|---|---|---|---|
| K-1 | *The create-only decision has a check/use race: `new: true` is checked only by the CLI pre-check, and the engine create can copy in a home that becomes occupied between the two — an existing document enters the task despite the gate; later write leaves plus `task finalize` can commit changes to it* | **CONFIRMED — finding `(R6, K-1)`, origin codex.** The gate half reproduces at both create doors and in the source pass's own scenario; the *commit* half reproduces only against an untracked victim, and is **refused** in the source pass's own scenario | RR-2 (toggler; both doors; the loss) · RR-3 (the source pass's proposed repro: 1 hit in 160 raced creates; `task finalize` exit 3) |
| S-1 | O23 carried — a role already holding a doc routes at the one-doc-per-task route (the held doc, the completion boundary, forced discard, a fresh `jigc start --workflow`) | **CONFIRMED as stated** — and D-3 is the state in which that route's two exits are a refusal and a destruction; the source pass did not read that state, which is not a refutation of it | RR-11 a |
| S-2 | Both create doors consult the gate | **CONFIRMED as stated** | RR-11 a |
| S-3 | Refusal before copy-in — carried in sequential execution, incomplete under the race | **CONFIRMED, both halves** — nothing staged after every sequential refusal; K-1 is the incomplete half | RR-11 a · RR-2 |
| S-4 | On-disk scope (`is_file` at the canonical home, independent of the staged area) and the same-task re-run | **CONFIRMED as stated** — an untracked file refuses, an id staged only in another open task does not, a re-run acks `existed: true` | RR-11 b · RR-12 (`C043`–`C045`) |
| S-5 | Strict entry keys — `deny_unknown_fields`, load failure is `workflow-refs.malformed-front-matter` | **CONFIRMED as stated** | RR-8 |
| S-6 | Precedence — the `new` check ranks ahead of `write.identity-change` and `write.title-ignored` | **CONFIRMED as stated**, for an id **on disk**. D-4 is the other arm (the id only staged by this task), where `write.title-ignored` answers under `new: true`; the source pass's sentence does not deny it, `findings-channel.md` does | RR-11 c · RR-7 |
| S-7 | Entry without `new` — the committed arm of `write.title-ignored` routes at a distinct identity; only the task-staged arm routes at `doc rename` | **CONFIRMED as stated** | RR-11 d |
| C-1 | `VERB_KINDS` / `DOCTYPE_DOORS` carry the write surface; *no third production create path found* | **CONFIRMED at the cells driven; OPEN at the rest.** The six non-create `doc` write leaves refuse an absent address and mint nothing. The top-level doctype doors that can place a doc (`migrate` under a `new: true` migrate shadow — the driver's U3 — and `milestone add-from-spec`) were not driven | RR-11 e · open lead OL-1 |
| C-2 | The non-create leaves address an existing document; they do not mint | **CONFIRMED as stated** | RR-11 e |
| C-3 | `create.already-exists` is blocking and instance-keyed | **CONFIRMED** — key `(create.already-exists, <type>:<slug>)`, severity `blocking`, exit 1 | RR-11 a |
| C-4 | Both shipped opt-ins present; every other shipped entry omits `new` | **CONFIRMED** — both report workflows refuse; six of the 27 other workflows with entries driven as create-or-update; the rest is the registry count above | RR-11 f · RR-12 |
| C-5 | The command registry supplies the two creates without a CLI flag | **CONFIRMED** — the composed steps emit `jigc doc create <type> --title <TITLE> --task <t>`; `jigc doc create --help` names no `--new`; passing one is clap exit 2 | RR-11 g |
| C-6 | The fan-out join stays outside the pre-check and keeps its suffixing; no `new` consumer there | **CONFIRMED as stated — and it is where D-1 lives.** The source pass states the fact and derives no defect from it; the driver's D-1 is not contradicted by the source pass and not found by it | RR-1 |
| C-7a | M55's manifest delta is exactly the two new schema-version-1 rows; no other hash moved | **CONFIRMED** — the binary projects `schema-version: 1` for both, and the pack-load freeze assertion passes at every door of every rig; the repository diff of the two manifests between `jigc-v1.0.0-rc.22` and `…rc.24` is the two rows (rc.23 → rc.24: empty) | RR-11 h |
| C-7b | rc.23 → rc.24 changes no pinned `--format json` key | **OPEN LEAD** — needs an rc.23 binary beside this one to compare envelopes; none was at hand, and a diff read is not a drive | OL-2 |
| C-8 | F21 stays report-only and opens no write-gate bypass | **CONFIRMED at the one cell driven** — with a committed project structural-op delta on `report-inconsistency`, both create doors still refuse an occupied id and stage nothing. F21's own subject (what `validate` misses) is not this row's and was not re-driven | RR-11 i |

**Refuted source claims: none.** No statement of the source pass was contradicted by a driven output. One
*weighting* in it is corrected by a drive: K-1's *"potentially tier 1 because the successful path reaches a
committing door and can modify the pre-existing document"* — in the scenario the source pass proposes, the
committing door **refuses** (RR-3). That datum is recorded under K-1's tier, not as a refutation of the
claim, whose gate half reproduces.

### B · the driver's defects — each re-driven once

| key | re-drive | status | tier — and which halves are shown |
|---|---|---|---|
| `(R6, D-1)` the join's suffix lands on an occupied id; `milestone finalize` overwrites it | RR-1 — reproduces exactly, and again under `park-idea` sub-tasks | **CONFIRMED** | **tier 1 — both halves shown.** Exit 0 at `milestone join` and at `milestone finalize` (the committing door), `findings` empty, `displaced: []`. Loss: before-control finds `EARLIER-FINDING-MARKER` (committed) and `UNTRACKED-NOTE-MARKER` (untracked) — after, 0 and 0; the untracked note is in no commit and nowhere under `$REPO` (`command grep -rl`, `.jigc/` included), the committed one survives in history only. No caller addressed either victim |
| `(R6, D-2)` the `write.identity-change` route over a committed doc copied in | RR-5 — reproduces; the reconciler also followed hop 2 to its end | **CONFIRMED** | **tier 2.** Not tier 1: followed to its end, the second route's `jigc rename` exits 0 and moves the **earlier reporter's** finding onto the new reporter's title — but no byte of its prose is lost (`B-MARKER` 1 → 1), and the door does what its argv and the route's sentence say. The harm half is a misroute, not a loss |
| `(R6, D-3)` two open report tasks mint one id; the second's routes | RR-6 — reproduces | **CONFIRMED** | **tier 2.** Not tier 1 — the loss half is missing: `task finalize` exits 3 and the landed finding is unchanged at every step. It is also the state K-1's natural variant leaves a reporter in |
| `(R6, D-4)` `write.title-ignored` under a `new: true` entry | RR-7 — reproduces at both doors; the route runs | **CONFIRMED** | **tier 3** — the sentences are in `design/findings-channel.md` (§4 *"Inside a report task this arm cannot arise"*, §10 *"reachable only under an entry without `new: true`"*); nothing is lost |
| `(R6, D-5)` the bare `allows-create` entry form | RR-8 — reproduces | **CONFIRMED** | **tier 3** — `design/write-commands.md` → *Two entry forms* against a load that accepts the object form with `as` only |
| `(R6, D-6)` the singleton arm routes at `jigc task discard <id>` | RR-9 — reproduces | **CONFIRMED** | **tier 3** — one extra hop, the next refusal names `--force`; shadow-only |
| `(R6, D-7)` a dangling symlink at the minted home: `task finalize` exits 0 committing the link | RR-4 — reproduces; the reconciler added the clone control | **CONFIRMED** | **the driver's tier 3 does not survive the predicate — tier 1 on the *harm* disjunct, and the reconciler says so with its weight.** Exit 0 ✓. Loss ✗ — no pre-existing byte is gone; the finding's prose sits in an untracked file. Repository harm ✓ — the commit the door made holds a `120000` entry in place of the finding, the finding is in no commit (`git log -S` → 0), and in a clone of that commit `jigc doc list inconsistency` exits 1 for the **whole doctype** where the parent commit's exits 0. Weight, stated so it is triaged as what it is: the precondition is a planted dangling link at exactly the minted path, an untracked dangling link already breaks the local listing before any finalize (the fragility is the read's — the driver's L-3 class) |
| `(R6, D-8)` the load error names the key and not the workflow | RR-8 — reproduces, incl. at a healthy workflow's `start` | **CONFIRMED** | **tier 3** |
| `(R6, D-9)` general case: the `write.identity-change` route onto an occupied id | RR-10 — reproduces; the second route lands | **CONFIRMED** | **tier 3** |
| `(R6, D-10)` composed text under a shadow that drops the key | RR-8 — reproduces | **CONFIRMED** | **tier 3** |

No driver defect was demoted to a lead, and none was contradicted by the source pass.

### C · the finding the source pass adds

**`(R6, K-1)` · the create-only gate is check-then-use with nothing held between: a home that becomes
occupied inside the window is copied in at exit 0 under a `new: true` entry** — origin codex.

- **Doors:** `doc create`, `doc author` (the breach) → `task finalize` (what the breach then meets).
- **Shown, the gate half:** with a file appearing at the minted home between the pre-check and the create,
  `doc create` exits 0 `existed: true`, provenance `edited-from-base`, the staged copy holding the other
  writer's prose — 19 of 60 creates against a file toggled in and out of the home, 8 of 60 at `doc author`
  (where the payload's slots land on the copied-in body in the same call), and **1 of 160** in the source
  pass's own scenario, a second reporter's `task finalize` landing the id mid-create.
- **Tier — split, because the two variants differ in exactly the half that decides it:**
  - *The source pass's scenario (two jigc reporters):* **tier 2, plus the tier-3 sentence.** The loss half is
    **missing** — the racing task's `task finalize` is refused `finalize.base-mismatch`, exit 3, and the
    landed finding's checksum is unchanged. What is left is a `create.already-exists` contract that says
    *"the existing one is never copied in for update"* over a create that copied it in, and a task parked in
    D-3's state.
  - *A non-jigc writer (an untracked file landing in the window):* **tier 1 by the predicate's letter —
    both halves shown.** `doc create` exit 0, the step's next lines, `task finalize` exit 0, `displaced:
    []`; `RACE-VICTIM-MARKER` 1 → 0, afterwards in no commit and nowhere under `$REPO`. Weight: the window
    is sub-millisecond and the writer has to be outside jigc — every jigc door that places a file at a home
    also moves `HEAD`, which is what the refusal above keys on. The JSON ack did say `existed: true`; the
    text ack is the sentence a re-run over the task's own doc also prints (the driver's O-1), so a reporter
    cannot tell the two apart.
- **Contract contradicted:** `design/findings-channel.md` §4 and `write-commands.md` → *The create-gate*
  (*adjudicated before anything is copied in*); the refusal's own message.

### D · baseline rows

**None — this row has no baseline; it is a first drive.** The one context row, re-driven once:

| key | status | datum |
|---|---|---|
| the declared edit-gate bound (`design/findings-channel.md` §1.5) | **STILL-OPEN (expected) — HOLDS AS DECLARED** | RR-12 (`X01`–`X09`): after a `create.already-exists` refusal the same report task reached the committed finding through `set-field`, `set-slot`, `add-item`, `retitle-item`, `remove-item` and a same-slug `doc rename`, and `task finalize` landed it at exit 0 — `PORT-MARKER` 1 → 0, by the caller's explicit `set-slot` on an address it named. A re-slug is refused. Not a finding |

The driver's observation **O-2** was weighed against the tier-1 predicate and stays an observation: under
an entry **without** the key, an untracked conformant file at the home is copied in (`existed: true`) and
the task's explicit `set-slot` plus `task finalize` replace bytes git never held (`SQUAT-MARKER` 1 → 0,
exit 0, RR-12 `E01`). Exit 0 and absent bytes are both shown; what is not shown is a defect — it is the
declared create-or-update, and the only bytes changed are the slot the caller addressed. K-1's second
variant is the same mechanism **under an entry whose contract is that it cannot start**.

## Reconciler's repro blocks

Every block starts from `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n
"$REPO" ] || exit`, run inside `$REPO`. `fill(<t>)` is the driver's: `set-field commit:<t>#type --value
docs`, `#scope --value findings`, `set-slot #summary`, `#body`, each exit 0. `sha` is `shasum -a 256`,
first 16 hex. A `--format json` refusal is read from stderr.

### RR-1 · `(R6, D-1)` re-driven — rig r2, `fresh`

```text
$ jigc --version                                                             → jigc 1.0.0-rc.24
# 1 · an earlier finding is filed and landed
$ jigc start --workflow report-inconsistency "file the stage two finding"
$ jigc doc create inconsistency --title "Stage 2" --task file-the-stage-two-finding      → inconsistency:stage-2 [exit 0]
$ jigc doc set-field inconsistency:stage-2#meta/kind --value code-doc --task …            [exit 0]
$ jigc doc set-slot  inconsistency:stage-2#description --from-file - --task …             [exit 0]
     (stdin: "EARLIER-FINDING-MARKER the stage-2 deploy doc and the script disagree.")
  fill(file-the-stage-two-finding)
$ jigc task finalize file-the-stage-two-finding   → finalized a8c61c2 · promoted docs/inconsistencies/stage-2.md [exit 0]
# 2 · an untracked conformant note: stage-2.md copied to docs/inconsistencies/stage-3.md with the H1
#     "# Stage 3" and the description "UNTRACKED-NOTE-MARKER written by hand, never committed." — never `git add`ed
# 3 · three reporters each file the title "Stage"
$ jigc config get finalize.fan-out.squash         → finalize.fan-out.squash = true  (pack-default) [exit 0]
$ jigc milestone create "stage reports"                                                   [exit 0]
$ jigc milestone add-task stage-reports "<alpha|bravo|charlie> reports the stage" --workflow report-inconsistency   [exit 0] ×3
$ jigc milestone provision stage-reports          → provisioned 3 worktree(s) … at base a8c61c2 [exit 0]
$ jigc milestone execute stage-reports            → 3 Spawn: lines [exit 0]
  in each $REPO/.jigc/worktrees/<sub>:
$ jigc workflow report-inconsistency --task <sub>                                         [exit 0]
$ jigc doc create inconsistency --title "Stage" --task <sub>      → inconsistency:stage   [exit 0] ×3
$ jigc doc set-field inconsistency:stage#meta/kind --value doc-doc --task <sub>           [exit 0]
$ jigc doc set-slot  inconsistency:stage#description --from-file - --task <sub>           [exit 0]   (stdin: "<who> reporter: the stage finding.")

# BEFORE-CONTROL (in $REPO)
stage-2.md (committed): sha 836a31f4e9d8e5da · command grep -c EARLIER-FINDING-MARKER → 1
stage-3.md (untracked): sha 042e92394eef78f1 · command grep -c UNTRACKED-NOTE-MARKER  → 1
$ command grep -rl UNTRACKED-NOTE-MARKER .        → ./docs/inconsistencies/stage-3.md
$ git status --short                              → ?? docs/inconsistencies/stage-3.md
$ git ls-files docs/inconsistencies               → docs/inconsistencies/stage-2.md

$ jigc milestone join stage-reports
joined milestone:stage-reports — 6 doc(s) merged
  - commit:alpha-reports-the-stage  (created · from alpha-reports-the-stage)
  - commit:bravo-reports-the-stage  (created · from bravo-reports-the-stage)
  - commit:charlie-reports-the-stage  (created · from charlie-reports-the-stage)
  - inconsistency:stage  (created · from alpha-reports-the-stage)
  - inconsistency:stage-2  (created · from bravo-reports-the-stage)  ← suffixed -2 on collision
  - inconsistency:stage-3  (created · from charlie-reports-the-stage)  ← suffixed -3 on collision
[exit 0]
$ jigc milestone join stage-reports --format json  → exit 0 · no findings · overlay provenance "created" for all three
$ jigc milestone finalize stage-reports --format json
  commits: ab4893c "Finalize milestone stage-reports (3 sub-tasks)"
           paths docs/inconsistencies/stage-2.md, stage-3.md, stage.md, docs/milestone-records/stage-reports.md
  manifest: promoted stage-2.md · promoted stage-3.md · promoted stage.md · modified the record
  displaced: [] · files: 4 · still_staged: []
[exit 0]

# AFTER
stage-2.md: sha 8a73bd7e9acbd673 · EARLIER-FINDING-MARKER → 0 · "bravo reporter"   → 1
stage-3.md: sha 933ad31774a62db7 · UNTRACKED-NOTE-MARKER  → 0 · "charlie reporter" → 1
$ git status --short                              → (empty)
$ git show --stat --format='%h %s' HEAD           → stage-2.md | 6 +++--- · stage-3.md | 21 + · stage.md | 21 + · the record | 8 ++++----
$ git log --all --oneline -S UNTRACKED-NOTE-MARKER | wc -l     → 0
$ git log --all --oneline -S EARLIER-FINDING-MARKER | wc -l    → 2      (history only)
$ command grep -rl UNTRACKED-NOTE-MARKER .                     → (nothing — `.jigc/` included)
$ command grep -rl EARLIER-FINDING-MARKER --exclude-dir=.git . → (nothing)
$ jigc validate --format json                     → exit 0 · advisory `schema-conformance.repeatable-populated` only
$ jigc doc show inconsistency:stage-2             → "# Stage" · kind doc-doc · "bravo reporter: the stage finding."
```

The same shape under a workflow **without** `new: true` (the driver's F14), on the same rig: a committed
`idea:pair-idea-2` (`EARLIER-IDEA-MARKER`), two `--workflow park-idea` sub-tasks each `doc create idea
--title "Pair idea"`.

```text
BEFORE  pair-idea-2.md sha 42436ea556524b1f · EARLIER-IDEA-MARKER → 1
$ jigc milestone join batch-three
  - idea:pair-idea  (created · from alpha-parks-pair)
  - idea:pair-idea-2  (created · from bravo-parks-pair)  ← suffixed -2 on collision
  - inconsistency:stage  (edited-from-base · from charlie-edits-stage)
[exit 0]
$ jigc milestone finalize batch-three             → promoted docs/ideas/pair-idea-2.md, …/pair-idea.md, docs/inconsistencies/stage.md · 4 files committed [exit 0]
AFTER   pair-idea-2.md sha d7dd18ac4852fcd1 · EARLIER-IDEA-MARKER → 0 · "bravo idea" → 1
```

### RR-2 · `(R6, K-1)` — the window, driven with a file toggled at the home (rig r5, `fresh`)

```text
setup    one finding landed (inconsistency:seed) so the home directory exists. A conformant victim file
         (seed.md with the H1 "# Race" and the description "RACE-VICTIM-MARKER written by another hand.")
         is parked under $REPO/.git/race-park/, outside the worktree.
toggler  a python loop: os.rename(<park>/race.md, $REPO/docs/inconsistencies/race.md); os.rename(back) —
         as fast as it runs; on stop it leaves the file AT its home.
control  no toggler, file absent:   jigc doc create inconsistency --title "Race" --task race-ctl-absent --format json
                                    → exit 0 · existed false
control  no toggler, file present:  the same argv in task race-ctl-present
                                    → exit 1 · (create.already-exists, inconsistency:race) · docs/: commit doc + provenance.json only
race     toggler running:
$ jigc start --workflow report-inconsistency "race attempt 0"
$ jigc doc create inconsistency --title "Race" --task race-attempt-0 --format json
  → exit 0 · op create · existed true · copied_in false                       # first attempt
$ ls $REPO/.jigc/tasks/race-attempt-0/docs        → commit:race-attempt-0.md  inconsistency:race.md  provenance.json
$ cat …/roles.json                                → {"roles":{"inconsistency":"inconsistency:race"}}
$ cat …/docs/provenance.json                      → "inconsistency:race": "edited-from-base"
$ command grep -c RACE-VICTIM-MARKER "…/docs/inconsistency:race.md"            → 1   (the other writer's prose, staged)

tally    60 creates, each in its own fresh task, toggler running (81 951 round-trips), title "Race two":
           31 × exit 1 · create.already-exists
           19 × exit 0 · existed true          ← the gate breached
           10 × exit 0 · existed false
tally    60 `jigc doc author inconsistency --from-file - --task <t> --format json` (payload title "Race three",
         description <<AUTHOR-PAYLOAD prose>>), toggler running (82 218 round-trips):
           31 × exit 1 · create.already-exists
            8 × exit 0 · provenance edited-from-base   ← copied in, and the staged copy's description is
                                                          already "AUTHOR-PAYLOAD prose" in place of the victim's
           21 × exit 0 · provenance created
```

Downstream of the first hit — toggler stopped, the victim file at its home, the reporter runs the step's
next lines:

```text
BEFORE  docs/inconsistencies/race.md (untracked): sha ea2c80111393e1b8 · RACE-VICTIM-MARKER → 1
        $ command grep -rl RACE-VICTIM-MARKER --exclude-dir=.git .
          → ./.jigc/tasks/race-attempt-0/docs/inconsistency:race.md  ./docs/inconsistencies/race.md
        $ git log --all --oneline -S RACE-VICTIM-MARKER | wc -l    → 0
        $ git status --short                                       → ?? docs/inconsistencies/race.md
$ jigc doc create inconsistency --title "Race" --task race-attempt-0          # the re-run, sequential again
blocking · create.already-exists — … route: this task already holds `inconsistency:race` … [exit 1]
$ jigc doc set-field inconsistency:race#meta/kind --value doc-doc --task race-attempt-0          [exit 0]
$ jigc doc set-slot  inconsistency:race#description --from-file - --task race-attempt-0          [exit 0]
     (stdin: "REPORTER-MARKER the new finding the reporter meant to file.")
  fill(race-attempt-0)
$ jigc task validate race-attempt-0               → advisory file-state.staged-copy, file-state.baseline-adopt [exit 0]
$ jigc task finalize race-attempt-0 --format json → manifest: promoted docs/inconsistencies/race.md · displaced: [] [exit 0]
AFTER   race.md: sha a6113ad6a3c43f70 · RACE-VICTIM-MARKER → 0 · REPORTER-MARKER → 1
        $ command grep -rl RACE-VICTIM-MARKER --exclude-dir=race-park .    → (nothing)
        $ git log --all --oneline -S RACE-VICTIM-MARKER | wc -l            → 0
        $ git status --short                                               → (empty)
```

### RR-3 · `(R6, K-1)` — the source pass's own proposed repro: `doc create` (task A) against `task finalize` (task B) — rig r6, `fresh`

```text
per trial  B: jigc start --workflow report-inconsistency "<p> holder <n>"; doc create inconsistency --title "<P> <n>";
              kind; description "HOLDER-<n> the first reporter."; fill(B)
           A×40: jigc start --workflow report-inconsistency "<p> <n> racer <m>"          (forty fresh tasks)
           launch `jigc task finalize <B>`; then, at forty staggered delays centred on the instant the
           promote lands, one `jigc doc create inconsistency --title "<P> <n>" --task <A_m> --format json` each.
trial 0: finalize exit 0 · last fresh-mint delay 211.5 ms · first refused delay 196.5 ms · no hit
trial 1: finalize exit 0 · last fresh-mint delay 204.0 ms · first refused delay 204.8 ms · no hit
trial 2: finalize exit 0 · last fresh-mint delay 206.4 ms · first refused delay 205.7 ms · no hit
trial 3: finalize exit 0 · last fresh-mint delay 211.9 ms · first refused delay 213.4 ms · HIT
tally over the 160 creates:   86 × exit 0 · existed false
                              73 × exit 1 · create.already-exists
                               1 × exit 0 · existed true          ← task secondrun-3-racer-29
```

A first run of fourteen trials read stdout only, so its refusals were unclassified; it is not counted.
Downstream of the hit:

```text
$ ls $REPO/.jigc/tasks/secondrun-3-racer-29/docs  → commit:….md  inconsistency:secondrun-3.md  provenance.json
$ cat …/docs/provenance.json                      → "inconsistency:secondrun-3": "edited-from-base"
$ command grep -c 'HOLDER-' "…/docs/inconsistency:secondrun-3.md"              → 1    (task B's prose, staged in A)
landed secondrun-3.md: sha 0f55618ab4b88b58 · HOLDER- → 1
$ jigc doc set-field inconsistency:secondrun-3#meta/kind --value doc-doc --task secondrun-3-racer-29   [exit 0]
$ jigc doc set-slot  inconsistency:secondrun-3#description --from-file - --task …                       [exit 0]   (stdin: "RACER-MARKER …")
  fill(secondrun-3-racer-29)
$ jigc task validate secondrun-3-racer-29         → one advisory (file-state.staged-copy) [exit 0]
$ jigc task finalize secondrun-3-racer-29
blocking · finalize.base-mismatch — the task was started at base `5281e5a…` but HEAD is now `3006f10…`, and the moved history overlaps the task's work on `docs/inconsistencies/secondrun-3.md`
  at: task:secondrun-3-racer-29
  route: resolve the overlap on `docs/inconsistencies/secondrun-3.md` against the new history, or discard the task with `jigc task discard secondrun-3-racer-29 --force`
[exit 3]
landed secondrun-3.md: sha 0f55618ab4b88b58 · HOLDER- → 1 · RACER-MARKER → 0        # unchanged
```

The same refusal without the race, as a control (task A minted before B landed, the doc reached through an
edit leaf): `jigc doc create inconsistency --title "Late" --task task-a-early` → exit 1
`create.already-exists`; `doc set-field inconsistency:late#meta/kind …` → exit 0 *(copied in for update …)*;
`set-slot`; `fill`; `jigc task finalize task-a-early` → exit 3 `finalize.base-mismatch`; `late.md`:
`B-MARKER` → 1, `A-MARKER` → 0.

### RR-4 · `(R6, D-7)` re-driven, with the clone control — rig r1, `fresh`

```text
$ ln -s nowhere.md docs/inconsistencies/dangling-home.md                       (nowhere.md does not exist)
$ jigc start --workflow report-inconsistency "edge finalize dangling home"
$ jigc doc create inconsistency --title "Dangling home" --task edge-finalize-dangling-home   → inconsistency:dangling-home [exit 0]
  kind; description "DANGLING-MARKER finding prose."; fill
$ jigc task finalize edge-finalize-dangling-home
finalized 4eadb59 — docs(findings): file a finding
  promoted docs/inconsistencies/dangling-home.md
  1 file committed
  left-out (…): docs/inconsistencies/nowhere.md …
[exit 0]
$ git ls-tree HEAD docs/inconsistencies/          → 120000 blob c5f140c8…  docs/inconsistencies/dangling-home.md  (+ the regular docs)
$ git cat-file -p HEAD:docs/inconsistencies/dangling-home.md                   → nowhere.md
$ git status --short                              → ?? docs/inconsistencies/nowhere.md        (holds DANGLING-MARKER: 1)
$ git log --all --oneline -S DANGLING-MARKER | wc -l                           → 0
$ jigc doc show inconsistency:dangling-home       → serves the finding [exit 0]               (the local read hides it)

# the committed state, read where the untracked target does not exist
$ git clone -q "$REPO" <tmp>/c && cd <tmp>/c
$ git checkout -q HEAD~1 && jigc doc list inconsistency                        [exit 0]      # before-control: the parent commit
$ git checkout -q -      && jigc doc list inconsistency
reading the committed doc at "<tmp>/c/docs/inconsistencies/dangling-home.md": No such file or directory (os error 2)
[exit 1]
$ jigc doc show inconsistency:dangling-home
blocking · store.not-found — could not read `inconsistency:dangling-home` at `docs/inconsistencies/dangling-home.md`: No such file or directory (os error 2)
[exit 1]

# control, another rig: an UNTRACKED dangling link at a home, no finalize
$ ln -s nowhere.md docs/inconsistencies/dangle-ctl.md && jigc doc list inconsistency   [exit 1]
$ rm -f docs/inconsistencies/dangle-ctl.md        && jigc doc list inconsistency       [exit 0]
```

A symlink to an existing doc at the home is refused by the gate (`C083`): `jigc doc create inconsistency
--title "Link home" …` → exit 1 `(create.already-exists, inconsistency:link-home)`.

### RR-5 · `(R6, D-2)` re-driven, and the second route followed to its end — rigs r1, r6

```text
$ jigc start --workflow report-inconsistency "role binding probe"
$ cat $REPO/.jigc/tasks/role-binding-probe/roles.json                          → (no such file)
$ jigc doc set-field inconsistency:port-mismatch#meta/kind --value doc-doc --task role-binding-probe
set … = doc-doc (copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize) [exit 0]
$ cat …/roles.json                                → {"roles":{"inconsistency":"inconsistency:port-mismatch"}}
$ jigc doc create inconsistency --title "A wholly new finding" --task role-binding-probe
blocking · write.identity-change — … route: `jigc doc rename inconsistency:port-mismatch --to 'A wholly new finding' --task role-binding-probe` moves the doc this task already holds onto the title (and id) you asked for; … [exit 1]
$ jigc doc rename inconsistency:port-mismatch --to 'A wholly new finding' --task role-binding-probe
blocking · write.identity-change — rename rejected: `inconsistency:port-mismatch` is committed … route: `jigc rename inconsistency:port-mismatch --to 'A wholly new finding'` moves it for real … once this task is finalized or discarded [exit 1]
$ jigc doc create inconsistency --title "Port mismatch" --slug port-mismatch-v --task role-binding-probe      → write.identity-change [exit 1]
$ jigc doc rename inconsistency:port-mismatch --to 'Port mismatch' --slug port-mismatch-v --task …            → write.identity-change [exit 1]
general twin (park-idea): set-field idea:parked-thought#meta/trigger → copy-in; doc create idea --title "Wholly other idea" → write.identity-change;
                          its route `jigc doc rename idea:parked-thought --to 'Wholly other idea' --task …` → write.identity-change [exit 1, 1]
in a sub-task (r2):       create "Stage" refused → set-field inconsistency:stage#meta/kind (copy-in) → create "Second twin" → write.identity-change [exit 1]

# hop 2 followed to its end (r6; the same two refusals over inconsistency:late, then every open task discarded)
late.md: sha d529e005fba1c4c4 · B-MARKER → 1
$ jigc rename inconsistency:late --to 'A wholly new finding'
renamed inconsistency:late -> inconsistency:wholly-new-finding (docs/inconsistencies/late.md -> docs/inconsistencies/wholly-new-finding.md), repointed 0 referrer(s)
[exit 0]
late.md gone · wholly-new-finding.md: B-MARKER → 1 · H1 "# A wholly new finding"        # the EARLIER reporter's finding, under the new reporter's title
```

### RR-6 · `(R6, D-3)` re-driven — rig r1

```text
$ jigc start --workflow report-inconsistency "other open holder"
$ jigc doc create inconsistency --title "Shared thing" --task other-open-holder        → inconsistency:shared-thing [exit 0]
  kind; description "HOLDER-MARKER-a11 the first reporter."
$ jigc start --workflow report-inconsistency "cell 64 probe"
$ jigc doc create inconsistency --title "Shared thing" --task cell-64-probe --format json → exit 0 · existed false · no finding     (row C046, corrected)
  kind; description "SECOND-MARKER-b22 the second reporter."
  fill(other-open-holder); jigc task finalize other-open-holder   → promoted docs/inconsistencies/shared-thing.md [exit 0]
shared-thing.md: sha 73b82020febc2a6a · HOLDER-MARKER-a11 → 1
  fill(cell-64-probe)
$ jigc task validate cell-64-probe                → one advisory (file-state.staged-copy) [exit 0]
$ jigc task finalize cell-64-probe                → blocking · finalize.base-mismatch … at: task:cell-64-probe [exit 3]
shared-thing.md: sha 73b82020febc2a6a · HOLDER-MARKER-a11 → 1                            # unchanged
$ jigc doc create inconsistency --title "Shared thing" --task cell-64-probe
blocking · create.already-exists — … route: this task already holds `inconsistency:shared-thing` … land it with `jigc task finalize cell-64-probe`, or abandon it with `jigc task discard cell-64-probe --force`; then … `jigc start --workflow report-inconsistency "<intent>"` [exit 1]
$ jigc doc rename inconsistency:shared-thing --to "Shared thing" --slug shared-thing-2 --task cell-64-probe   → renamed [exit 0]
$ jigc task finalize cell-64-probe                → promoted docs/inconsistencies/shared-thing-2.md [exit 0]
HOLDER-MARKER-a11 in shared-thing.md → 1 · SECOND-MARKER-b22 in shared-thing-2.md → 1
```

### RR-7 · `(R6, D-4)` re-driven — rig r1 (task `second-report-same-port`, `report-inconsistency`)

```text
$ jigc doc create inconsistency --title "Own thing" --task <t> --format json    → exit 0 · existed false
$ jigc doc create inconsistency --title "Own thing" --task <t> --format json    → exit 0 · existed true            (the re-run)
$ jigc doc create inconsistency --title "Own Thing!" --task <t> --format json   → exit 1 · (write.title-ignored, inconsistency:own-thing)
    route: `jigc doc rename inconsistency:own-thing --to 'Own Thing!' --task <t>` retitles the doc in place; then re-run this write unchanged
$ jigc doc author inconsistency --from-file - --task <t> --format json          → exit 1 · (write.title-ignored, inconsistency:own-thing)   (payload title "OWN thing?")
$ jigc doc create inconsistency --title "Different words" --slug own-thing --task <t> --format json → exit 1 · write.title-ignored
$ jigc doc rename inconsistency:own-thing --to 'Own Thing!' --task <t>          → (retitled "Own Thing!" — the id is unchanged) [exit 0]
$ jigc doc create inconsistency --title "Own Thing!" --task <t>                 → inconsistency:own-thing (already existed — copied in for update) [exit 0]
```

### RR-8 · `(R6, D-5)`, `(R6, D-8)`, `(R6, D-10)` and the entry-key cells re-driven — rig r3, `fresh`

Each shadow is the shipped `report-inconsistency.yaml` (or `park-idea.yaml`) written to
`$REPO/.jigc/config/workflows/` with only its `allows-create` entry replaced, `git add -f` + `git commit`,
and removed again with `git rm` + commit. `inconsistency:port-mismatch` and `idea:parked-thought` are landed
first.

```text
entry := { type: inconsistency, as: inconsistency }                                    # the key dropped — D-10
$ jigc start --workflow report-inconsistency "compose under dropped key"               [exit 0]; lines 8–10 of the composed text:
  This task files exactly one NEW record. A title whose slug a doc on disk already
  holds is refused `create.already-exists`, with nothing staged — choose a distinct
  title, or keep it and pass `--slug <slug>` to the same create.
$ jigc doc create inconsistency --title "Port mismatch" --task compose-under-dropped-key --format json → exit 0 · existed true
$ ls …/docs                                       → commit doc · inconsistency:port-mismatch.md · provenance.json
  a task minted BEFORE the shadow, same argv      → exit 0 · existed true                 (S01)
  `--title "Port mismatch!"`                      → exit 1 · write.title-ignored          (C097)
  doc author, payload title "Port mismatch"       → exit 0 · op author                    (C098)
  shadow removed, same argv                       → exit 1 · create.already-exists        (C099)

entry := { type: inconsistency, as: inconsistency, nwe: true }                         # misspelt — D-8
$ jigc start --workflow report-inconsistency "probe under typo"
blocking · workflow-refs.malformed-front-matter — workflow front-matter is malformed config-family YAML: allows-create[0]: unknown field `nwe`, expected one of `type`, `as`, `new` at line 7 column 47
  route: fix the workflow/step/catalog definition the message names (a definition defect, repaired once at its source), then re-run
[exit 1]
$ jigc start --workflow report-inconsistency "probe under typo" --format json  → {"error": "<the text finding above>"} [exit 1]
$ jigc start --workflow park-idea "another unrelated one"                      → the identical message [exit 1]
$ jigc start · jigc workflow park-idea --preview · jigc describe               → the identical message [exit 1] ×3
  a report task minted before the typo:
$ jigc doc create inconsistency --title "Port mismatch" --task <pre> --format json · jigc doc author …   → {"error": …} [exit 1] ×2
$ jigc doc set-field inconsistency:typo-holder#meta/kind …  · jigc doc add-item …#sides --title "docs/b.md" · jigc doc rename … --to "Typo holder three"   [exit 0] ×3
$ jigc task validate <pre> · jigc task finalize <pre> --dry-run · jigc start --task <pre>                 → the identical message [exit 1] ×3
$ jigc doc create idea --title "Unrelated idea" --task <an open park-idea task>                           → idea:unrelated-idea [exit 0]
$ jigc validate --format json                     → exit 0 · blocking (workflow-refs.malformed-front-matter, workflow:report-inconsistency)
$ jigc milestone create "typo milestone two" · milestone add-task … --workflow report-inconsistency ×2 · milestone list-tasks · config list   [exit 0] ×5

entry := - inconsistency                                                               # the bare form — D-5
$ jigc start --workflow report-inconsistency "probe bare form"
blocking · workflow-refs.malformed-front-matter — … allows-create[0]: invalid type: string "inconsistency", expected struct AllowsCreate at line 7 column 5 [exit 1]
entry := { type: inconsistency, new: true }       → … allows-create[0]: missing field `as` at line 7 column 5 [exit 1]

value and key variants (each `jigc start --workflow report-inconsistency "<intent>"`; where it loads, then `doc create … --title "Port mismatch"`):
  { …, new: true, extra: 1 }   → exit 1 · unknown field `extra`
  new: yes · "true" · 1 · ~    → exit 1 ×4 · allows-create[0].new: invalid type: …, expected a boolean
  New: true                    → exit 1 · unknown field `New`
  two entries for one doctype  → exit 1 · workflow-refs.allows-create-duplicate-type
  new: false                   → loads · create exit 0 · existed true
  new: True · "new": true      → loads · create exit 1 · create.already-exists
  new: true on the OTHER type's entry → loads · create exit 0 · existed true

entry (park-idea) := { type: idea, as: idea, new: true }                               # the key added
  a task minted before the shadow: doc create idea --title "A Parked Thought"   → exit 1 · (create.already-exists, idea:parked-thought)   (S02)
  `--title "Parked thought!"` · doc author "the parked thought"                → exit 1 ×2 · create.already-exists                       (C101, C104)
  `--slug parked-thought-2`                                                    → exit 0 · existed false                                  (C102)
  `--title "Entirely fresh"`                                                   → exit 0 · existed false · no finding                     (C105, corrected)
  jigc start --workflow park-idea "<intent>" | command grep -c already-exists  → 0                                                       (S04)
```

### RR-9 · `(R6, D-6)` re-driven — rig r4, `committed-singletons`

```text
shadow   form-vision's entry as { type: vision, as: vision, new: true }; committed with plain git
$ jigc start --workflow form-vision "vision route check"
$ jigc doc create vision --title Vision --task vision-route-check
blocking · create.already-exists — `vision:vision` already exists on disk at its home, …
  route: `vision` has a fixed identity — … If this task holds nothing else, `jigc task discard vision-route-check` abandons it
[exit 1]
  `--title "Other title"` · `--title Vision --slug vision-two` · doc author (payload title Vision)   → exit 1 ×3 · (create.already-exists, vision:vision) · the same route
$ ls …/docs                                       → commit doc · provenance.json;  roles.json: absent
$ jigc task discard vision-route-check                                                 # the route, verbatim
blocking · task-discard.staged-prose — task `vision-route-check` stages 1 doc(s) that no commit has a copy of — discarding it would destroy them: commit:vision-route-check
  route: … `jigc task discard vision-route-check --force` removes the working area with them
[exit 1]
$ jigc task discard vision-route-check --force    → discarded [exit 0]
```

### RR-10 · `(R6, D-9)` re-driven — rig r1

```text
$ jigc start --workflow park-idea "route check rg1"
$ jigc doc create idea --title "Mine" --task route-check-rg1                    → idea:mine [exit 0]
$ jigc doc create idea --title "A Parked Thought" --task route-check-rg1
blocking · write.identity-change — … route: `jigc doc rename idea:mine --to 'A Parked Thought' --task route-check-rg1` moves the doc this task already holds onto the title (and id) you asked for; … [exit 1]
$ jigc doc rename idea:mine --to 'A Parked Thought' --task route-check-rg1
blocking · write.already-present — rename rejected: the committed store already holds `idea:parked-thought` … route: … re-run `jigc doc rename idea:mine --to 'A Parked Thought' --slug <other-slug>`; or … `jigc doc show idea:parked-thought` and edit that one … [exit 1]
$ jigc doc rename idea:mine --to 'A Parked Thought' --slug parked-thought-mine --task route-check-rg1   → renamed [exit 0]
```

### RR-11 · the source pass's *carried* statements, driven

```text
a · both doors, refusal before copy-in, the key, O23's route — rig r1, task <t> = second-report-same-port (report-inconsistency),
    after inconsistency:port-mismatch landed (sha 3e4b3de8fa15bace)
$ ls $REPO/.jigc/tasks/<t>/docs                   → commit:<t>.md  provenance.json           (before-control);  roles.json: absent
$ jigc doc create inconsistency --title "Port mismatch" --task <t> --format json
  → exit 1 · severity blocking · key (create.already-exists, inconsistency:port-mismatch)
    route: choose a distinct `--title`, or keep this one and pass `--slug <slug>` …
$ jigc doc author inconsistency --from-file - --task <t> --format json           (payload title "Port mismatch")
  → exit 1 · the same key · route: set the payload's `title:` to a distinct title …
  `--title "the PORT mismatch!"` · payload title "Port, Mismatch?" · `--title "Unrelated" --slug port-mismatch`  → exit 1 ×3 · the same key
$ ls …/docs                                       → commit:<t>.md  provenance.json           (after: identical);  port-mismatch.md sha unchanged
  with the role bound (the task holds inconsistency:own-thing):
$ jigc doc create inconsistency --title "Port mismatch" --task <t> --format json
  → exit 1 · create.already-exists · route: this task already holds `inconsistency:own-thing` (its `inconsistency`), and a task carries one doc per role — … land it with `jigc task finalize <t>`, or abandon it with `jigc task discard <t> --force`; then … `jigc start --workflow report-inconsistency "<intent>"`
  doc author, payload title "Port mismatch"       → exit 1 · the same route, spelled *a distinct payload `title:`*

b · on-disk scope — an untracked copy of a conformant doc at docs/inconsistencies/untracked-thing.md
$ jigc doc create inconsistency --title "Untracked thing" --task <t> --format json   → exit 1 · (create.already-exists, inconsistency:untracked-thing)
$ jigc doc author … (payload title "Untracked thing")                               → exit 1 · the same key;  docs/: unchanged

c · precedence — role bound to inconsistency:own-thing:
  `--title "Third thing"`                         → exit 1 · (write.identity-change, inconsistency:own-thing)
  `--title "Port mismatch"` (an occupied id)      → exit 1 · create.already-exists       (ahead of identity-change)

d · entry without `new` (park-idea, committed idea:parked-thought)
$ jigc doc create idea --title "A Parked Thought!" --task <t> --format json
  → exit 1 · (write.title-ignored, idea:parked-thought) · route: choose a distinct `--title`, or keep this one and pass `--slug <slug>` to mint beside the existing doc …
$ jigc doc author idea … (payload title "Parked: thought")   → exit 1 · the same key · route: set the payload's `title:` to a distinct title …
$ jigc doc create idea --title "My idea" …  then  --title "My Idea!"
  → exit 0, then exit 1 · (write.title-ignored, idea:my-idea) · route: `jigc doc rename idea:my-idea --to 'My Idea!' --task <t>` retitles the doc in place …

e · the other write leaves do not mint — a fresh report task
$ jigc doc set-field inconsistency:never-made#meta/kind --value doc-doc --task <t>   → no staged instance for `inconsistency:never-made#meta/kind` — provision it first … [exit 1]
$ jigc doc set-slot …#description · doc add-item …#sides --title "a.md" · doc rename inconsistency:never-made --to "Something"   → the same refusal [exit 1] ×3
$ ls …/docs                                       → commit doc · provenance.json;  docs/inconsistencies/: no never-made.md

f · the second shipped opt-in — report-jigc-feedback (rig r3), the id occupied by an untracked file at docs/jigc-feedback/
$ jigc doc create jigc-feedback --title "Finalize sweeps a staged path" --task feedback-second --format json
  → exit 1 · (create.already-exists, jigc-feedback:finalize-sweeps-a-staged-path)
  `--title "…path!"` · `--title "Else" --slug finalize-sweeps-a-staged-path` · doc author (payload title "finalize sweeps: a staged path")   → exit 1 ×3 · the same key
  `--slug finalize-sweeps-again`                  → exit 0 · existed false

g · no CLI flag
$ jigc start --workflow report-jigc-feedback "feedback holder"   → line 6: Run: `jigc doc create jigc-feedback --title <TITLE> --task feedback-holder`
$ jigc start --workflow report-inconsistency "<intent>"          → line 6: Run: `jigc doc create inconsistency --title <TITLE> --task <t>`
$ jigc doc create --help | command grep -c -- '--new'            → 0
$ jigc doc create inconsistency --title "X" --new --task <t>     → error: unexpected argument '--new' found [exit 2]

h · the two M55 doctypes as the binary projects them
$ jigc doc schema inconsistency --format json     → type inconsistency · schema-version 1 · contract-version 7 [exit 0]
$ jigc doc schema jigc-feedback --format json     → type jigc-feedback · schema-version 1 · contract-version 7 [exit 0]
$ git diff --stat jigc-v1.0.0-rc.23 jigc-v1.0.0-rc.24 -- 'crates/cli/packs/*/config/schema-manifest.yaml'   → (empty)      # a repository datum, not a drive
$ git diff jigc-v1.0.0-rc.22 jigc-v1.0.0-rc.24 -- … (the same two files)                                    → +2 rows: inconsistency, jigc-feedback, schema-version 1

i · a project structural-op delta (F21's precondition) does not open the gate — rig r3
$ jigc config remove-step "workflow:report-inconsistency#author-commit"   → config: removed … written to `.jigc/config/` [exit 0]; committed with plain git
$ jigc start --workflow report-inconsistency "f21 probe"
$ jigc doc create inconsistency --title "Port mismatch" --task f21-probe --format json       → exit 1 · create.already-exists
$ jigc doc author … (payload title "Port mismatch!")                                         → exit 1 · create.already-exists;  docs/: commit doc + provenance.json
$ jigc validate --format json                     → exit 0 · no blocking finding
```

### RR-12 · register — the rows that carried no block, re-driven

Each line is `row — the argv shape re-driven → exit · code`, in the rig named. Fixture titles are the
reconciler's where the driver's task no longer existed; the cell is the driver's.

```text
rig r1 (fresh; landed: inconsistency:port-mismatch, inconsistency:shared-thing, idea:parked-thought, idea:common-idea, adr:use-a-single-node-cache)
A05      C006's argv (`--slug` distinct over a committed id) — covered by RB-1; the feedback and added-key twins re-driven (RR-11 f, RR-8) → 0
A06 A07  role bound + the committed id by title / by payload title                         → 1 · create.already-exists · the one-doc-per-task route
A08 A10  role bound + a free id by title / by `--slug yet-another`                         → 1 · write.identity-change · route `jigc doc rename … --to … [--slug …]`
A09      role bound, `--slug <own id>`, the staged title                                   → 0 · (already existed — copied in for update)
A11      `jigc doc rename inconsistency:own-thing --to 'Third thing' --task <t>`           → 0 · renamed; roles.json re-keyed to inconsistency:third-thing
A12 A13  `doc rename inconsistency:third-thing --to "Port mismatch"` / `--to "Whatever else" --slug port-mismatch` → 1 ×2 · (write.already-present, inconsistency:port-mismatch); checksum unchanged
A04      six titles onto a committed id ("Shared thing!", "shared THING", "The Shared Thing", "Shared  thing.", "A shared thing", "Shared-thing") → 1 ×6 · (create.already-exists, inconsistency:shared-thing); docs/ and checksum unchanged
X01–X09  the declared bound: create refused → set-field (copied in for update) · set-slot · add-item · retitle-item · add-item + remove-item → 0 each;
         `doc rename … --to "Port mismatch retitled"` → 1 · write.identity-change; `--to "PORT mismatch!"` → 0; create again → 1 · create.already-exists;
         `task finalize` → 0 · promoted docs/inconsistencies/port-mismatch.md; sha 3e4b3de8fa15bace → 1b0d176c6ea4b0c4 · PORT-MARKER 1 → 0
Rt01     the `write.title-ignored` route run, then the create re-run                        → 0, 0
Rt02     `--title "Different words" --slug own-thing` → 1 · write.title-ignored; its route `doc rename … --to 'Different words' --slug own-thing` → 0; re-run → 0; staged H1 "# Different words"
Rt07     park-idea, idea:parked-thought copied in by set-field; `--title "Parked Thought!"` → 1 · write.title-ignored; `doc rename idea:parked-thought --to 'Parked Thought!'` → 0 (the committed identity keeps its slug); re-run → 0
Rt10     role bound to the task's own doc, then set-field copies a committed doc in, then the create of that committed title:
         new → 1 · (create.already-exists, inconsistency:shared-thing) · roles.json still inconsistency:own-thing;  general → 1 · (write.identity-change, idea:mine) · roles.json still idea:mine
Rt16 C106 C107 C108   park-idea, "Common idea" staged in another open task: create same title → 0 · existed false (C106); `--title "Common Idea!"` → 0 (C107);
         doc author same title → 0 · no finding (C108, corrected); holder lands; the kept task's create re-run → 0 · existed true; its `task finalize --format json` → 3 · (finalize.base-mismatch, task:common-cell-kept); common-idea.md sha unchanged
C043–C045 report-inconsistency, "Shared two" staged in another open task: `--title "Shared Two!"` → 0 · existed false; `--title "Else" --slug shared-two` → 0; doc author same title → 0
C085–C091 role bound (new): committed title → 1 · create.already-exists; `--title "???"` → 1 · (create.empty-title, inconsistency); `"???" --slug shared-thing` → 1 · create.already-exists;
         `--slug Shared_Thing` → 1 · {"error": … is not a valid slug …}; unbound `--slug SHARED-THING` → 1 · the same error; doc author committed title → 1 · create.already-exists; payload title "!!!" → 1 · create.empty-title
C092–C095 role bound (general, idea:mine): committed title → 1 · (write.identity-change, idea:mine); `--title "Parked Thought!"` → 1 · the same; doc author committed title → 1 · the same; `--title "???"` → 1 · (create.empty-title, idea)
C109–C111 an untracked `Case-Thing.md` at the home: `--title "Case thing"` → 1 · (create.already-exists, inconsistency:case-thing); `--title "SHARED THING"` → 1 · create.already-exists; doc author "shared thing" → 1 · the same   (APFS, case-insensitive)
C079–C081 a foreign untracked file at the home: general `doc create idea --title "Foreign notes"` → 0 · existed true; new `doc create` / `doc author` "Foreign inc" → 1 ×2 · create.already-exists
C082 E03 E06  a directory named dir-home.md at the home: `doc create … "Dir home"` → 0 · existed false; `task finalize` → 1, code-less: could not read "$REPO/docs/inconsistencies/dir-home.md" before promoting over it: Is a directory (os error 21) … task is intact;
         `jigc doc list inconsistency` → 1, code-less
E02      the foreign idea file, copied in: `doc set-field idea:foreign-notes#meta/trigger` → 1 · write.non-reparseable; `task validate` → 3; `task finalize --format json` → 3 · (conformance.section-missing, idea:foreign-notes#description); the file's sha unchanged
E01      an untracked CONFORMANT idea file (SQUAT-MARKER): `--title "Squatter Idea!"` → 1 · write.title-ignored; `--title "Squatter idea"` → 0 · existed true; set-slot description → 0; `task finalize` → 0 · promoted, advisory file-state.baseline-adopt;
         squatter-idea.md sha 79b094b1c7bd407a → e54d308ed451eb82 · SQUAT-MARKER 1 → 0 · in no commit, nowhere under $REPO
Ad1–Ad6  under record-decision AND single-task, identical: `--title "Use a Single-Node cache!"` → 1 · (write.title-ignored, adr:use-a-single-node-cache); doc author "use a single node: cache" → 1 · the same;
         `--title "Something else" --slug use-a-single-node-cache` → 1 · the same; same title → 0 · existed true; `--slug use-a-single-node-cache-2` → 0 · existed false · roles {"decision": …-2}; doc author same title → 0 · op author; adr sha unchanged
Or1–Or5  report task: `doc create idea --title "!!!"` → 1 · (create.gate-blocked, idea); `nosuch "!!!"` → 1 · (create.unknown-doctype, nosuch); `--slug BAD_SLUG` with nosuch / idea / inconsistency "!!!" → 1 ×3 · {"error": `--slug "BAD_SLUG"` is not a valid slug …}
         also `inconsistency "!!!"` → 1 · create.empty-title; `"!!!" --slug port-mismatch` → 1 · create.already-exists
P01–P06  doc author: unknown top-level key + committed title → 1 · (write.wrong-shape, inconsistency); no title → 1 · the same; a bare slot value + committed / fresh title → 1 ×2 · (write.malformed-value, inconsistency);
         a `<<…>>`-wrapped field + committed title → 1 · the same; `nosuch` ungrammatical / well-formed → 1 · write.wrong-shape / create.unknown-doctype; `idea` ungrammatical / well-formed existing id → 1 · write.wrong-shape / (create.gate-blocked, idea); docs/ unchanged, no role

rig r2 (fresh; the fan-out)
F01      `jigc config get finalize.fan-out.squash`                                         → 0 · true (pack-default)
F03      in a sub-task worktree, `jigc workflow report-inconsistency --task <sub>`         → 0 · `jigc task finalize` 0 times · `jigc milestone finalize` once · `create.already-exists` once
F05      a sub-task re-runs its create                                                     → 0 · (already existed — copied in for update)
F06      a sub-task mints an id committed at the base: `doc create … "Stage"` / doc author "Stage!"   → 1 ×2 · (create.already-exists, inconsistency:stage)
F16 F17  that sub-task's set-field + set-slot on inconsistency:stage → 0 ×2 (copied in for update); `doc create … "Second twin"` → 1 · write.identity-change;
         join: `inconsistency:stage (edited-from-base · from charlie-edits-stage)`; `milestone finalize` → 0; stage.md "alpha reporter" 1 → 0, CHARLIE-EDIT → 1   (the declared bound, through the fan-out)
F10      after the join landed, a plain report task files "Stage"                          → 1 · (create.already-exists, inconsistency:stage)
F07      a plain task lands inconsistency:late-clash AFTER provision; the sub-task worktree does not hold it; `doc create … "Late clash" --task <sub>` → 1 · (create.already-exists, inconsistency:late-clash)
F18      that milestone's `milestone finalize`                                             → 3 · finalize.base-mismatch at milestone:batch-two · route: out-of-band git; late-clash.md unchanged

rig r3 (fresh; shadows) — S01, S02, S04, S07–S09, S12–S16, S18, S19 are the lines of RR-8; S10 as `jigc workflow park-idea --preview` and `jigc describe`.

rig r4 (committed-singletons; checksums of VISION.md, CHANGELOG.md, docs/roadmap.md, docs/decisions-log.md identical before and after)
Sg01–Sg04 form-vision: `--title Vision` → 0 · existed true; `--title "Our Grand Vision"` → 1 · (write.title-ignored, vision:vision) · route `jigc doc create vision --title Vision --task <t>`; `--title Vision --slug other-vision` → 0 · target vision:vision;
         doc author payload "Vision" → 0 · op author; "Our Grand Vision" → 1 · write.title-ignored · route: set the payload's `title:` to `Vision` (or drop the line …)
Sg05 Sg06 planning: roadmap / decisions-log / deferral-ledger → 0 ×3 · existed true, true, false · roles roadmap, log, ledger; `roadmap --title "Road Map 2"` → 1 · write.title-ignored
Sg07     doc author roadmap, payload "Another roadmap" / "Roadmap"                         → 1 · write.title-ignored / 0
Sg08 Sg09 record-change: `--title "Release notes"` → 1 · (write.title-ignored, changelog:changelog); `--title Changelog` → 0 · existed true; doc author "Release notes" / "Changelog" → 1 / 0
Sh02     shadow new: true on form-vision, doc author payload "Vision"                      → 1 · (create.already-exists, vision:vision)
Sh03     the same shadow, after `doc set-slot vision:vision#thesis` copied the vision in (roles {"vision":"vision:vision"}); `doc create vision --title Vision` → 1 · create.already-exists · the one-doc-per-task route
Sh04     shadow new: true on record-change: `doc create changelog --title Changelog` / doc author → 1 ×2 · (create.already-exists, changelog:changelog); nothing staged
Sh05     shadow new: true on planning's roadmap and deferral-ledger entries: roadmap → 1 · create.already-exists; deferral-ledger → 0 · existed false, again → 0 · existed true; decisions-log → 0 · existed true
```

## Leads left open

- **OL-1 (codex C-1, the undriven half).** *No third production create path* was driven at the six
  non-create `doc` write leaves only. The top-level doctype doors that can place a doc — `jigc migrate`
  under a `migrate-*` shadow carrying `new: true` (the driver's U3) and `jigc milestone add-from-spec` —
  were not driven; no rig builds the foreign source the first needs.
- **OL-2 (codex C-7b).** *rc.23 → rc.24 changes no pinned `--format json` key* — needs an rc.23 binary
  beside this one; not at hand. Never promoted on the diff read.
- **OL-3 (K-1, the undriven cell).** The finalize race was driven at `doc create` only; `doc author` was
  raced against the file toggler (RR-2) and not against a second reporter's `task finalize`. A sub-task's
  create raced against a plain task's finalize (the driver's F07 door) was not driven either.
- **The driver's own not-driven list (U1–U14) stands unchanged** — chiefly the case-sensitive filesystem
  (U1), `create.serial-collision` in the order of refusals (U2), D-1 with a foreign file, a directory or a
  symlink at the suffixed home (U9) and a team-layer shadow (U11).
- **The driver's leads for other rows** — L-1 (the milestone wedge, re-seen as F18), L-2 (the left-out
  narration names untracked paths under *a staged path stays staged*, re-seen at every doc-only finalize on
  r1), L-3 (a directory at a home: code-less exit 1 at `task finalize` and `doc list`, re-seen; a dangling
  link does the same to `doc list`, RR-4), L-5 (`milestone add-task` under a workflow that does not load,
  re-seen) — are re-seen, not graded, and stay with their rows. L-4 was not re-driven.

## Doors covered

Every clap leaf that is the door of at least one driven row above, `VERB_KINDS` spelling — the driver's 26,
each re-seen by the reconciler, plus `doc schema` from the reconciler's own rows:

`start` · `workflow` · `describe` · `validate` · `doc create` · `doc add-item` · `doc remove-item` · `doc
retitle-item` · `doc rename` · `doc set-field` · `doc set-slot` · `doc author` · `doc show` · `doc schema` ·
`doc list` · `task validate` · `task discard` · `task finalize` · `config get` · `config list` · `milestone
create` · `milestone add-task` · `milestone list-tasks` · `milestone provision` · `milestone execute` ·
`milestone join` · `milestone finalize` — **27 of 48**; all 8 `doc` × `Write` leaves among them.

`rename` (run once, as the end of D-2's second route) and `config remove-step` (run once, to build C-8's
fixture) were driven as steps of another door's cell and are **not** claimed as covered; `task discard
--force` between cells is rig cleanup and confers nothing.
