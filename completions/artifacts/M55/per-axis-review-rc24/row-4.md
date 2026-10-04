<!-- Reconciled ROW 4 file (pack-load / manifest freeze · migration), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis4` in the body means ROW 4 of this run, not numbered axis 4. -->

**ROW 4 · pack-load / manifest freeze · migration — RECONCILED.** This row HAS a source pass: the Opus driver's table (below, unchanged except the demotions marked `[RECONCILER]`) reconciled against the Codex source pass (exit code 0: one claim, eight baseline dispositions, four consistent-findings paragraphs). The reconciliation ledger follows the driver's §8.

---

# rc.24 partial per-axis re-review — ROW 4 · pack-load / manifest freeze · migration — the Opus driver

Numbered axis **7** (freeze & migration), scoped to what M54, M55 and the co-author trailer changed.
Baseline: rc.16 (`completions/artifacts/M52/per-axis-review/`). A baseline row keeps its `(7, …)` key
verbatim; a new finding of this run is keyed `(R4, <id>)`.

- **Binary:** `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` (asserted first, and again by
  the probe helper before every rig load). Release posture. No `target/debug/jigc`, no `cargo run`.
- **Source read at** `bffa6667` (`work/rc24-gate`); `git diff --stat jigc-v1.0.0-rc.24 HEAD -- crates/cli/packs
  crates/engine crates/cli/src` is empty, so the registries below are the tagged tree's.
- **Rigs:** every one built with `dev/jigc-rig <state> --binary ~/.local/bin/jigc`, stdout-only capture,
  two-step eval, `[ -n "$REPO" ]` guard, every root from `mktemp -d` under a session scratch root
  (`<tmp>`). No teardown, no recursive removal. Nothing was committed in the working repository.
- **`CLAUDECODE` is set** in this session and was held constant: every commit jigc made in a rig
  carries `Co-Authored-By: Claude <noreply@anthropic.com>`, the migration commits included. Expected,
  row 10's.
- **Verdict up front:** **no tier-1 row.** Five findings — two proposed tier 2, three proposed tier 3 —
  and `(7, A7-F3)` is **CLOSED** on both home kinds.

---

## 1 · The door set and the registries, read from the code

| registry | file | the instrument's author read | **I read** | datum |
|---|---|---|---|---|
| the embed seam | `crates/cli/src/pack_builtin.rs` | 2 packs, 1 module | **2** `include_dir!` statics (`DEV`, `METHODOLOGY`), one module, one `PackSource` impl (`EmbeddedPack`) | same |
| dev manifest entries | `crates/cli/packs/dev/config/schema-manifest.yaml` | 6 | **6** (`commit` `adr` `spec` `prd` `arch-doc` `changelog`) | same |
| methodology manifest entries | `crates/cli/packs/methodology/config/schema-manifest.yaml` | 13 | **13** (12 persisted + the transient `commit` shadow); `jigc-feedback` and `inconsistency` at schema-version 1 | same |
| shipped schemas | `crates/cli/packs/{dev,methodology}/schemas/` | 6 + 13 | **6 + 13** | same |
| prior-shape snapshots | `…/schema-snapshots/` | 2 + 4; none for the new two | **2 + 4** (`adr.v1`, `changelog.v1` · `deferral-ledger.v1`, `completion-record.v1`, `milestone-record.v1`, `milestone-record.v2`); none for either new doctype | same |
| `SchemaChangeKind::ALL` × `LOCI` | `crates/engine/src/schema_diff.rs` | 18 × 3 = 54 | **18** variants × `LOCI = MAX_NESTING_DEPTH + 1` = **3** → 54 | same |
| `RelocateRefusal::ALL` | `crates/cli/src/relocate.rs` | "read the count" | **10** | the instrument left it open; 10 is also the baseline's count |
| `AllowsCreate`'s keys | `crates/engine/src/compose.rs` | 3, closed | **3** (`type`, `as`, `new`) | same — and the binary's own refusal prints the same three |
| workflows shipped | `…/workflows/` | 18 + 21 = 39; 34 entries over 29 | **18 + 21 = 39**; front-matter parse: **34** `allows-create` entries over **29** workflows, **2** carrying `new: true` | same |
| `VERB_KINDS` | `crates/cli/src/cli.rs` | 48 | **48** | same |

No count differs. The one thing the count table could not tell me, driven instead: `jigc describe
--format json` on a `fresh` rig carries 39 workflow + 18 doctype definitions (18 = 6 + 13 − the one
`commit` id both packs ship), split `origin_pack` dev 18 + 6, methodology 21 + 12.

**Doors driven:** the numbered axis's eight (`describe` · `doc schema` · `validate` · `doc list` ·
`migrate-corpus` · `migrate` · `ingest` · `unmanage`) + `relocate` + the "other leaves" for the
*freeze blocks at every door that loads a pack* cell — **`start`, `workflow`, `doc show`, `config get`,
`task list`** (five, three asked) — + the fixture doors (`setup`, `doc create`, `doc set-field`,
`doc set-slot`, `doc add-item`, `task finalize`).

---

## 2 · The table

Row schema: `(door, cell) → {argv, exit, code|none, route kind, surface asserted, verdict}`. Route kind:
**M** mechanical (an argv or a shell command to run) · **H** human · **I** informational · **—** none.
Rows are grouped by cell; within a group the argv differs only by door. Repro blocks are §3.

### Cell 1 · the pack move — 37 rows

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1.1 | `describe` | `fresh`, no `JIGC_PACK_DIR` | `jigc describe` | 0 | none | I | the tour renders; 16 command refs tagged `(dev pack)`, 17 `(methodology pack)` | matches — both packs named; **no pack version on this surface** (O-3) |
| 1.2 | `describe` | same, json | `jigc describe --format json` | 0 | none | — | `schema_version: 3`; `definitions` 57 with `origin_pack` dev 24 / methodology 33 | matches |
| 1.3 | `doc schema` | same | `jigc doc schema jigc-feedback` | 0 | none | — | `doctype: jigc-feedback (schema-version 1)` | matches |
| 1.4 | `validate` | same | `jigc validate` | 0 | none | — | `no findings — the committed store validates clean` | matches |
| 1.5 | `doc list` | same | `jigc doc list` | 0 | none | — | `no committed docs` | matches |
| 1.6 | `migrate-corpus` | same | `jigc migrate-corpus --dry-run` | 0 | none | — | `0 would migrate, 0 already current, 0 blocked` | matches |
| 1.7 | `migrate-corpus` | same | `jigc migrate-corpus` | 0 | none | — | `0 migrated…`; no commit | matches |
| 1.8 | `ingest` | same | `jigc ingest` | 0 | none | I | `3 candidate(s) classified`, all `unmanaged` | matches |
| 1.9 | `start` | same | `jigc start --explain` | 0 | none | — | `Pack input: dev/1.0.0-rc.24 = <embedded>` and `Pack input: methodology/1.0.0-rc.24 = <embedded>`, each with its blake3 | matches — **this** is the surface that names both packs **and** their version |
| 1.10 | `setup` | `bare`, filesystem diff of `$RIG` before/after | `jigc setup` | 0 | none | I | 14 new paths, **all under `$REPO`**; 0 under `$RIG_HOME` (`HOME` repointed there) | matches, within the bound in §5.12 |
| — | `migrate` · `unmanage` · `relocate` | `fresh` + both docs, no shadow | rows 3.0b (the full 19-argv control) | 0 · 0 · 1 | — | — | each answers | matches (counted in cell 3) |
| 1.11–1.18 | `start` · `doc schema` ×2 · `validate` · `doc list` · `migrate-corpus` · `ingest` · `describe` | **`JIGC_PACK_DIR` pack copy** (`fresh --repin`) | `jigc start --explain` · `doc schema adr` · `doc schema jigc-feedback` · `validate` · `doc list` · `migrate-corpus --dry-run` · `ingest` · `describe --format json` | 0 ×7, **1** on `doc schema jigc-feedback` | `store.unknown-type` on that one | M (`jigc describe`) | `Pack input: dev/fs-local = $RIG/pack (blake3 1128e1be…)` — the **same digest** 1.9 prints for the embedded dev pack; the composition is dev alone (24 definitions, 0 methodology) | matches — the copy resolves; see O-8 for the dropped methodology pack **[RECONCILER]** members with no driver repro block — 1.18, `describe --format json` — **demoted**; re-driven by the reconciler (RC-M): match. |
| 1.19–1.28 | `describe` · `validate` · `doc list` · `ingest` · `migrate-corpus` · `doc schema` · `start` · `unmanage` · `relocate` · `migrate` | **a pack copy with `config/knobs.yaml` removed** | the ten argvs (§3 R-1c) | **1** each | `pack.resource-missing` | H | `no composed pack ships \`config/knobs\` — searched, highest-precedence first: dev/fs-local ($RIG/pack)` + route `restore \`config/knobs.yaml\` in a pack the composition already searches, or add the pack that ships it to \`.jigc/config/packs.yaml\`` | matches at 9 doors; **DEFECT `(R4, F1)` at `migrate`** (below) |
| 1.29 | `validate` | same, json | `jigc validate --format json` | 1 | (in the string) | H | stdout 0 bytes; stderr one `{"error": "blocking · pack.resource-missing — …"}` document | matches the flatten path's declared shape (outside the envelope) |
| 1.30 | `validate` | route followed verbatim (the file restored) | `jigc validate` | 0 | none | — | clean | matches — the route repairs **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 1.31–1.33 | `start` ×2 · `task list` | resource missing, **does a compose door mint?** | `jigc start --workflow quick-fix "<i>"` · `jigc start "<i>"` · `jigc task list` | 1 · 1 · 0 | `pack.resource-missing` | H | `start` mints **nothing** | matches |
| 1.34–1.37 | `task list` · `migrate` · `migrate` · `start` | resource missing → restored | `task list` (before: *no active tasks*) · `migrate foreign-adr.md --as adr` → **1** · (restore) · `migrate foreign-adr.md --as adr` → **1** `task.serial-collision` · `start --task <id>` → 0 | — | — | M on the collision | a task directory with four files exists after the **exit-1** `migrate` | **DEFECT `(R4, F1)`** |

### Cell 2 · the two new doctypes at schema-version 1 — 37 rows

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 2.1–2.6 | `doc schema` | each doctype × `agent` / `human` / `json` | `jigc doc schema {jigc-feedback,inconsistency} [--format human\|json]` | 0 ×6 | none | — | text: `doctype: <id> (schema-version 1)`, the injected `schema-version: int … (set: schema-version)` field, every write address. json: `contract-version: 7`, `schema-version: 1`, `identity: {kind: slugged, address: <id>:<slug>}`, `home: {kind: location, path: docs/jigc-feedback/<slug>.md}` / `docs/inconsistencies/<slug>.md` | matches `design/findings-channel.md` §1.1/§1.2. The text arms carry no home and no identity line (O-2) **[RECONCILER]** members with no driver repro block — the four `agent` / `human` text arms — **demoted**; re-driven by the reconciler (RC-M): match. |
| 2.7 | `start` · `doc create` · `doc set-field` · `doc set-slot` · `task finalize` | a `jigc-feedback` filed by this binary | `jigc start --workflow report-jigc-feedback "<what>"` → `doc create jigc-feedback --title …` → 3 `set-field` → `set-slot …#description` → commit `type` + `summary` → `task finalize` | 0 | none | — | `finalized <sha>` · `promoted docs/jigc-feedback/<slug>.md` · `1 file committed`; the file carries `schema-version: 1`, `status: open`, `date:` CLI-set | matches |
| 2.8 | + `doc add-item` | an `inconsistency` filed by this binary | `jigc start --workflow report-inconsistency "<what>"` → create → `set-field …#meta/kind` → 2 `add-item …#sides --slug` → `set-slot …#description` → commit → finalize | 0 | none | — | committed at `docs/inconsistencies/<slug>.md`, `schema-version: 1`, two `### … {#readme}` / `{#claude}` items | matches |
| 2.9 | `validate` | corpus holding both | `jigc validate` | 0 | none | — | clean | matches |
| 2.10–2.11 | `doc list` | same | `jigc doc list [--format json]` | 0 | none | — | two rows `managed`; json rows carry `title`, `fields` (incl. `schema-version: "1"`), `item-count` 2 / 0 | matches **[RECONCILER]** members with no driver repro block — 2.11, the json arm — **demoted**; re-driven by the reconciler (RC-M): match. |
| 2.12–2.15 | `migrate-corpus` | same — nothing to migrate | `jigc migrate-corpus [--dry-run] [--format json]` | 0 ×4 | none | — | `0 migrated, 2 already current, 0 blocked`; json keys `migrated` `already_current` `blocked` `unadopted` `unfilled` `commit: null` `hook_output` `dry_run`; **no commit made** | matches **[RECONCILER]** members with no driver repro block — the bare arm, the `--dry-run` text arm and the bare json arm — **demoted**; re-driven by the reconciler (RC-M): match. |
| 2.16–2.19 | `relocate` | each new doctype, a foreign prior home · its own home · json | `jigc relocate jigc-feedback --from docs/feedback` · `relocate inconsistency --from docs/inconsistency` · `relocate jigc-feedback --from docs/jigc-feedback` · `relocate inconsistency --from docs/feedback --format json` | **1** ×4 | `relocate.frozen-doctype` | M (`jigc migrate-corpus`) | `\`<id>\` is a frozen doctype — relocate it through the version-gated \`jigc migrate-corpus\`` / route: *"`jigc migrate-corpus` walks every prior home the doctype's versioned snapshots declare and lands the move under the freeze"* | refused as the contract says; **the route is DEFECT `(R4, F3)`** — and the help's example is `(R4, F4)` **[RECONCILER]** members with no driver repro block — 2.17–2.19 — **demoted**; re-driven by the reconciler (RC-M): match. |
| 2.20–2.22 | `migrate` | `--as` each new doctype (+ json) | `jigc migrate notes/fb.md --as jigc-feedback` · `… notes/inc.md --as inconsistency [--format json]` | **1** ×3 | **none** | M | `doctype \`<id>\` exists but is not migratable (no \`migrate-<id>\` workflow); migratable doctypes: adr, arch-doc, changelog, completion-record, decisions-log, deferral-ledger, idea, prd, research, roadmap, spec, vision` + route *re-run … with one of: …*; **no task minted** | matches (`design/findings-channel.md` §1.7: no `migrate-*` workflow ships for either); code-less (O-5) **[RECONCILER]** members with no driver repro block — 2.21–2.22 — **demoted**; re-driven by the reconciler (RC-M): match. |
| 2.23 | `migrate` | an option the door does not have | `jigc migrate notes/inc.md --as idea --dry-run` | 2 | none (clap) | — | `unexpected argument '--dry-run'` | matches (usage error) **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 2.24–2.27 | `ingest` | a **conformant** and a **non-conformant** hand-placed file at **each** home | `jigc ingest` | 0 | row-carried `conformance.section-renamed` on the two loose files | H | conformant → `adoptable … (adopted — indexed + baselined, no file moved)`; non-conformant → `needs-reconcile`, route *"reconcile … against the `<id>` schema by hand, then re-run `jigc ingest` — no `migrate-<id>` workflow ships to rewrite it for you"* | matches — the route is honest about the missing workflow |
| 2.28–2.29 | `validate` · `doc list` | the two unadopted files now at the homes | `jigc validate` · `jigc doc list` | **1** · 0 | `schema-conformance.unadopted-instance` (advisory, exit-flipped) | M (`jigc ingest`) | trailer *"a never-adopted file sits at a managed home — … the sweep exits non-zero"*; `doc list` state `unregistered` | matches (baseline e3/e4 on the new homes) |
| 2.30–2.34 | `unmanage` ×3 · `doc list` · `validate` | one of each | `jigc unmanage docs/jigc-feedback/hand-placed-conformant.md` · `unmanage docs/inconsistencies/loose-note.md` · `unmanage docs/inconsistencies/hand-placed-conformant.md --format json` | 0 ×3 | none | H / — | drop ack; `no-op: … is not managed (nothing to drop)`; json `{path, identity, dropped: true}`; afterwards `doc list` still `managed` (stamped ∧ resolves — registration, not baseline), `validate` adds advisory `file-state.un-baselined` | matches **[RECONCILER]** members with no driver repro block — 2.31–2.34 — **demoted**; re-driven by the reconciler (RC-M): match. |
| 2.35–2.37 | `migrate-corpus` ×2 · `doc show` | the triage keys with unadopted files present | `jigc migrate-corpus --dry-run` · `--format json` · `jigc doc show jigc-feedback:loose-note` | 0 · 0 · **1** | `schema-conformance.unadopted-instance` in `unadopted[]` · `store.unparseable` | M (`jigc ingest`) | `0 would migrate, 4 already current, 0 blocked, 2 not adopted`; json `unadopted` length 2, each with `{code, key: {code, target}, route}`; `doc show` routes *adopt — run `jigc ingest`* | matches **[RECONCILER]** members with no driver repro block — 2.36–2.37 — **demoted**; re-driven by the reconciler (RC-M): match. |

### Cell 3 · the freeze over the new doctypes — 215 rows

`DOORS` is the 19-argv list of §3 R-3. Under a freeze block every row's stdout is 0 bytes.

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 3.0a | 16 non-mutating members of `DOORS` | **control**, both docs filed, no shadow | §3 R-3 | 0 ×14, 1 ×2 (`relocate.frozen-doctype`) | — | — | every door answers | matches |
| 3.0b | all 19 | control, no shadow (the full list, mutating members included) | §3 R-3 | 0 ×17, 1 ×2 | — | — | `migrate notes/fb.md --as idea` → `task minted`; `unmanage` → drop ack; `start --workflow quick-fix` → `task minted` | matches |
| 3.a | all 19 | **project shadow, enum member added** to `jigc-feedback.kind` (committed) | §3 R-3a | **1** ×18 · 0 (`task list`) | none | **M** (`rm <shadow>`) + H | `pack-load freeze check failed: doctype \`jigc-feedback\`: schema-hash mismatch (manifest declares \`7e16fb98…\`, recomputed \`3ff572f1…\`) — the project schema shadow … changes the shape of a frozen doctype, which the freeze forbids at every layer` | matches (`corpus-migration.md` → The freeze; M49). `task list` loads no pack (O-13). No task minted, `git status` empty |
| 3.a-json | `validate` | same, json | `jigc validate --format json` | 1 | none | — | stdout 0 b; stderr one `{"error": …}` document | matches |
| 3.a-route | `validate` · `doc schema` | the route followed verbatim (`rm` the one named file) | — | 0 · 0 | none | — | loads clean | matches — the route repairs |
| 3.b | all 19 | project shadow, **field removed** (`about`) from `jigc-feedback` | §3 R-3a | 1 ×18 · 0 | none | M + H | the same block, naming `jigc-feedback` | matches |
| 3.c | all 19 (+1 full print) | project shadow, **`location` changed** on `inconsistency` (`inconsistencies/` → `disagreements/`) | §3 R-3a | 1 ×18 · 0 | none | M + H | `doctype \`inconsistency\`: schema-hash mismatch (manifest declares \`6b83fd12…\`, recomputed \`28e70084…\`)` | matches — the home is inside the hash (M38) |
| 3.d | `validate` · `doc schema <ty>` · `start --explain` | **11 further shape variants** (worktree shadow): default changed · inverse renamed · card widened · `set: on-create` dropped · optional slot made required · required field made optional · `id-from` changed · item slot made required · section renamed · enum member removed · `type:` renamed | §3 R-3d | **1** ×33 | none | M + H | freeze block at every one | matches — 14 distinct shape edits, 0 load |
| 3.e | same three doors | **6 prose-only variants**: `description:` reworded ×2 · `usage:` reworded · slot `hint:` reworded · item-slot `hint:` reworded · comments stripped | §3 R-3d | **0** ×18 | none | — | loads clean | matches (`overrides.md` → *Authored metadata … resolves by whole-file shadow*) |
| 3.f | all 19 + 7 | **prose-only shadows of both doctypes, committed** | §3 R-3f | 0 ×17, 1 ×2 (`relocate.frozen-doctype`) | — | — | `jigc describe` carries the reworded `description` ×2 and `usage` ×1; `doc schema` still `schema-version 1`, same home; `validate` clean over the filed docs | matches — the shadow capability survives and takes effect |
| 3.g | all 19 (+11) | **the methodology pack's own manifest at the pack layer**: a filesystem copy of the methodology pack **listed** in `.jigc/config/packs.yaml` over a `--repin` dev base, its `jigc-feedback` schema reshaped, **its own manifest not re-pinned** | §3 R-3g | **1** ×18 · 0 (`task list`) | none | **—** | `pack-load freeze check failed: doctype \`jigc-feedback\`: schema-hash mismatch (manifest declares \`7e16fb98…\`, recomputed \`3ff572f1…\`)` — the **methodology** manifest's declared value, and **no route line** | matches the per-origin resolution (`corpus-migration.md` → the M40 revision); route-less as baseline O-1 |
| 3.h | 9 doors (+2) | **absent manifest entry**, on that listed copy: `inconsistency`'s entry deleted, its schema still shipped | §3 R-3h | **1** ×9 | none | — | `pack-load freeze check failed: doctype \`inconsistency\` is shipped but absent from the freeze manifest` | matches. **The embedded-pack arm of this cell is still NOT DRIVEN** (§5.1) |

### Cell 4 · the closed entry keys — 34 rows

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 4.1 | `start` | project shadow of `report-jigc-feedback`, entry `{ type: jigc-feedback, as: feedback, nwe: true }` (committed) | `jigc start --workflow report-jigc-feedback "<i>"` | **1** | `workflow-refs.malformed-front-matter` | H | `workflow front-matter is malformed config-family YAML: allows-create[0]: unknown field \`nwe\`, expected one of \`type\`, \`as\`, \`new\` at line 9 column 42`; nothing minted | refused, key named — matches §10's row. **The route and the line number are DEFECT `(R4, F2)`** |
| 4.2 | `workflow` | same | `jigc workflow report-jigc-feedback --preview` | 1 | same | H | same text | same |
| 4.3–4.4 | `describe` | same (+ json) | `jigc describe [--format json]` | 1 | same | H | same text; json → stderr `{"error": …}` | same **[RECONCILER]** members with no driver repro block — 4.4, the json arm (re-driven on the stray-key shadow) — **demoted**; re-driven by the reconciler (RC-F2): match. |
| 4.5–4.6 | `validate` | same (+ json) | `jigc validate [--format json]` | **0** | same, **`blocking`** | H | `at: workflow:report-jigc-feedback` · trailer *"1 finding(s) — report-only at store scope (exit 0); these gate at compose (`jigc start`), never at the task or milestone boundary"*; json `blocking_probes: ["workflow-refs"]`, `report_only: true`, `key.target: workflow:report-jigc-feedback` | matches the **store-scope** contract (report-only). The scope's wording *"refused … at `validate`"* does not hold as an exit: the key is named, the door exits 0 (O-7) **[RECONCILER]** members with no driver repro block — 4.6, the json arm — **demoted**; re-driven by the reconciler (RC-F2): match. |
| 4.7–4.9 | `start` · `workflow` · `start` | same shadow, **a different workflow asked for** | `jigc start "<i>"` · `jigc workflow quick-fix --preview` · `jigc start --workflow quick-fix "<i>"` | 1 ×3 | same | H | the identical text — about a workflow the caller did not name | **DEFECT `(R4, F2)`** |
| 4.10–4.14 | `start` · `doc list` · `doc schema` · `migrate-corpus` · `ingest` | same shadow, doors that compose nothing | `start --explain` · `doc list` · `doc schema jigc-feedback` · `migrate-corpus --dry-run` · `ingest` | 0 ×5 | none | — | unaffected | matches (*every door that loads a workflow definition* — these load none) |
| 4.15–4.20 | `start` ×2 · `workflow` ×2 · `describe` · `validate` | **stray key**: project shadow of `single-task`, entry `{ type: adr, as: decision, note: stray }` | §3 R-4 | 1 ×5 · 0 (`validate`) | same | H | `allows-create[0]: unknown field \`note\`, expected one of \`type\`, \`as\`, \`new\` at line 5 column 44` | refused, key named — matches **[RECONCILER]** members with no driver repro block — 4.20, `validate` — **demoted**; re-driven by the reconciler (RC-F2): match. |
| 4.21–4.23 | `workflow` · `describe` · `validate` | **control**: a well-formed shadow carrying `new: true` on that entry | — | 0 ×3 | none | — | loads | matches — the third key is accepted |
| 4.24–4.25 | `workflow` · `validate` | `new: maybe` (non-boolean) | — | 1 · 0 | same | H | `allows-create[0].new: invalid type: string "maybe", expected a boolean` | matches |
| 4.26–4.27 | `workflow` · `validate` | a duplicated key (`as:` twice) | — | 1 · 0 | same | H | `allows-create[0]: duplicate field \`as\`` | matches |
| 4.28–4.31 | `task list` · `migrate` ×2 · `task list` | the stray-key shadow × `migrate` | `task list` (*no active tasks*) · `jigc migrate idea-note.md --as idea` → **1** · again → **1** `task.serial-collision` · `task list` (*1 active task*) | — | `workflow-refs.malformed-front-matter` then `task.serial-collision` | H / M | a `migrate-idea-…` task exists after the exit-1 refusal | **DEFECT `(R4, F1)`**, on the embedded packs |
| 4.32–4.34 | `start` · `validate` · `workflow` | full-text prints of 4.1 / 4.5 / 4.8 | — | — | — | — | (the quoted surfaces above) | — **[RECONCILER]** **not rows** — re-prints of 4.1 / 4.5 / 4.8, counted in the 34. |

### Cell 5 · the hint rewords — 7 rows

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 5.1–5.2 | `doc schema` | the two slots | `jigc doc schema planning-record [--format json]` | 0 | none | — | `cheap-vs-robust: slot (set-slot: …)` and `claim-driven: slot (…)`, schema-version **1**; **no hint text in any format** | **n/a against the scope's wording** — `doc schema` renders no slot `hint:` at all (O-2); the binary contradicts no stated contract |
| 5.3 | `workflow` | where the hints do render | `jigc workflow planning --preview` | 0 | none | — | the composed text carries *"…every fork lists the option that removes the artifact that can drift**, stated and priced like the others…"* (×2) and *"**An advocate's proposal and the orchestrator's halt recommendations are claims too** — each is driven end to end…"* (×2) | matches `design/findings-channel.md` §9 and the rc.23 → rc.24 `claim-driven` reword |
| 5.4 | `start` · `doc create` · `doc set-slot` ×15 · `doc set-field` · `task finalize` | a `planning-record` filed and committed by this binary (all 14 slots) | `jigc start --workflow planning "<m>"` → … → `task finalize` | 0 | none | — | `finalized <sha>` · `promoted docs/planning-records/<slug>.md` | matches |
| 5.5 | `validate` | that corpus | `jigc validate` | 0 | none | — | clean | matches |
| 5.6 | `migrate-corpus` | same | `jigc migrate-corpus --dry-run` | 0 | none | — | `0 would migrate, 1 already current, 0 blocked` — no version moved | matches (hint text is outside the hash) |
| 5.7 | `doc list` | same | `jigc doc list` | 0 | none | — | `planning-record:<slug> … managed` | matches **[RECONCILER]** **DEMOTED — NOT DRIVEN:** no driver repro block, and not re-driven by the reconciler. |

### Cell 6 · the stamp states, on the new doctypes only — 31 rows

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 6.1 | `validate` | **unstamped managed** `jigc-feedback` (the stamp line stripped, committed out of band) | `jigc validate` | **1** | `schema-conformance.schema-version-current` (blocking) + advisory `file-state.hash-matches` | M (`jigc migrate-corpus`) | *"field `schema-version` is absent; the committed doc predates the schema-version stamp (below the current schema-version 1)"* | matches (baseline c3′ on a new doctype) |
| 6.2 | `doc list` | same | `jigc doc list` | 0 | none | — | `managed` | matches baseline (state is registration, not currency) |
| 6.3 | `ingest` | same | `jigc ingest` | 0 | row none + advisory `file-state.absorbed` | I | `adoptable … (adopted)` + the absorb line naming the finding it retires | matches; currency-blind as baseline O-4 (**M51 D-1's closure passes through here: the absorb is surfaced**) |
| 6.4 | `migrate-corpus` | same | `jigc migrate-corpus --dry-run` | 0 | none | — | `1 would migrate, 1 already current` | matches **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 6.5 | `doc show` | same | `jigc doc show jigc-feedback:<slug>` | 0 | none | — | serves the unstamped bytes | matches **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 6.6 | `migrate-corpus` | same — the route, run | `jigc migrate-corpus` | 0 | none | — | `1 migrated … committed <sha> — only the migrated paths were staged`; the commit's whole diff is `+schema-version: 1` | matches — **no byte lost** |
| 6.7 | `validate` | after | `jigc validate` | 0 | none | — | clean | matches **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 6.8 | `validate` | **stamp ahead** (`inconsistency` stamped 2) | `jigc validate` | **1** | `schema-conformance.schema-version-ahead` | H | *"upgrade jigc, or restore the stamp from git history"*; the trailer states the exit flip | matches (baseline b3) |
| 6.9 | `doc list` | same | `jigc doc list` | 0 | none | — | `managed` | matches (b4) **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 6.10 | `ingest` | same | `jigc ingest` | 0 | none | I | `adoptable … (adopted)` | matches baseline b7 / O-4 **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 6.11–6.13 | `migrate-corpus` | same | `jigc migrate-corpus --dry-run` · bare · `--format json` | **1** ×3 | `migrate-corpus.schema-version-ahead` | H | `blocked` row + route; json `blocked[0].key = {code, target}`, `commit: null`; nothing written | matches (b5 — never a silent already-current) **[RECONCILER]** members with no driver repro block — 6.13, the json arm — **demoted**; re-driven by the reconciler (RC-M): match. |
| 6.14 | `doc show` | same | `jigc doc show inconsistency:<slug>` | 0 | none | — | serves it, stamp 2 visible | matches **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| — | `validate` · `doc list` · `ingest` · `migrate-corpus` · `doc show` | **unadopted foreign file at each home** | rows 2.26–2.29, 2.35–2.37 | — | — | — | — | (counted in cell 2) |
| 6.15–6.16 | `validate` · `doc list` | orphan cell's **control** (embedded composition) | — | 0 · 0 | none | — | clean; two `managed` | — **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 6.17 | `validate` | **orphaned instance**: the same corpus under a dev-only `JIGC_PACK_DIR` composition (the pack defining both doctypes departed) | `jigc validate` | **1** | `schema-conformance.orphaned-instance` ×2 | H | *"…carries a `schema-version:` stamp, but no resolved doctype claims this path…"*; route → `jigc ingest`, then re-add the pack, then `unmanage` + delete the stamp | matches (baseline d3; **M51 D-4 half 1 passes through: the false clause is gone**) |
| 6.18–6.19 | `doc list` | same (+ json) | `jigc doc list [--format json]` | 0 | none | — | `(none)  <path>  orphaned`; json `id: null`, `item-count: null`, `fields: null`, `title` the H1 | matches the declared third state |
| 6.20 | `ingest` | same | `jigc ingest` | 0 | none | I | both filed under `unmanaged … fine to stay plain` / *no action needed* | **the `(7, A7-F2)` shape, re-observed on the new doctypes — row 3's, not re-filed here** |
| 6.21 | `migrate-corpus` | same | `jigc migrate-corpus --dry-run` | 0 | none | — | `0 would migrate, 0 already current` — silent about both | matches baseline d5 **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 6.22–6.23 | `doc schema` · `doc show` | same | `jigc doc schema jigc-feedback` · `jigc doc show jigc-feedback:<slug>` | 1 · 1 | `store.unknown-type` | M (`jigc describe`) | `unknown doctype \`jigc-feedback\`` | matches (d2) |
| 6.24–6.25 | `unmanage` · `validate` | same | `jigc unmanage docs/jigc-feedback/<slug>.md` · `validate` | 0 · 1 | none · orphaned-instance | — | baseline dropped, file left; `validate` still exits 1, as the finding's own route predicts | matches (d8) **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 6.26 | `validate` | **a stamped `jigc-feedback` hand-moved out of its home** (`git mv` to `docs/feedback/`, committed) | `jigc validate` | **1** | `reconciliation.rename` + `schema-conformance.orphaned-instance` | H | routes: restore, or `jigc unmanage`; trailer *"revert the `git mv` or adopt it via `jigc rename`"* | matches (row 3's subject) **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-F3): matches. |
| 6.27 | `doc list` | same | `jigc doc list` | 0 | none | — | `(none)  docs/feedback/<slug>.md  orphaned` | matches **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-F3): matches. |
| 6.28 | `ingest` | same | `jigc ingest` | 0 | row-carried `ingest.wrong-location` | H | route *"move docs/feedback/<slug>.md into docs/jigc-feedback/, then re-run `jigc ingest`"* | matches — a route that works |
| 6.29 | `relocate` | same | `jigc relocate jigc-feedback --from docs/feedback` | 1 | `relocate.frozen-doctype` | M | route → `jigc migrate-corpus` | refusal matches; route is `(R4, F3)` |
| 6.30 | `migrate-corpus` | **that route, run verbatim** | `jigc migrate-corpus` | 0 | none | — | `0 migrated, 1 already current, 0 blocked` — the stranded doc is not mentioned and not moved | **DEFECT `(R4, F3)`** |
| 6.31 | `doc show` | same | `jigc doc show jigc-feedback:<slug>` | 1 | `store.not-found` | H | the generic route (*create the referenced doc, or fix the reference…, or `--task`*) | see O-9 — outside A7-F3's closed arm |

### Baseline cells — 58 rows

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 7.1–7.2 | `start` → `task finalize` | fixtures: an `adr` and a `changelog` filed by this binary under a `--repin` pack copy | `record-decision` / `record-change` | 0 | none | — | `docs/decisions/use-sqlite.md`, `CHANGELOG.md` committed | — |
| 7.3–7.4 | `describe` | **pack-layer hash moved, not re-pinned** (both relocation edits made, manifest version bumped) | `jigc describe` ×2 (the re-pin loop) | 1 ×2 | none | — | `doctype \`adr\`: schema-hash mismatch (… recomputed \`598eaf3d…\`)`, then `doctype \`changelog\`: … recomputed \`d9ee5fad7732b4b8…\`` — **the same digest the rc.16 record quotes for the same edit** | matches (baseline g1) |
| 7.5–7.6 | `doc schema` | after the re-pin | `jigc doc schema changelog` · `adr` | 0 | none | — | `(schema-version 3)` each | — |
| 7.7 | `validate` | **`(7, A7-F3)`: both homes moved by a version bump, before `migrate-corpus`** | `jigc validate` | 1 | `schema-conformance.schema-version-current` ×2 | M | route *run `jigc migrate-corpus`* | matches |
| 7.8 | `doc list` | same | `jigc doc list` | 0 | none | — | both at their **prior** homes, `managed` | as baseline |
| 7.9 | `ingest` | same | `jigc ingest` | 0 | row-carried `ingest.wrong-location` ×2 | H | *move … by hand, then re-run `jigc ingest`* | as baseline (row 3's neighbourhood) |
| 7.10–7.11 | **`doc show`** | **placement arm** (+ json) | `jigc doc show changelog` · `jigc doc show changelog:changelog --format json` | 1 | `store.not-found` | **M** | **route: *"`changelog:changelog` is committed at `CHANGELOG.md`, a prior home of `changelog` — the schema's home moved and this corpus has not been migrated; run `jigc migrate-corpus` to land it at the home this read resolves, then read it again"*; json: the findings envelope, `key {code, target}`, the same route | **`(7, A7-F3)` CLOSED — placement arm** |
| 7.12 | **`doc show`** | **location arm** | `jigc doc show adr:use-sqlite` | 1 | `store.not-found` | **M** | the same route shape, naming `docs/decisions/use-sqlite.md` as the prior home | **`(7, A7-F3)` CLOSED — location arm** |
| 7.13 | `migrate-corpus` | same | `jigc migrate-corpus --dry-run` | 0 | none | — | `2 would migrate` | matches **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 7.14 | `relocate` | same | `jigc relocate adr --from docs/decisions` | 1 | `relocate.frozen-doctype` | M | route → `migrate-corpus` — which here **acts** (7.15) | matches (**M51 D-2's route leg passes through, still CLOSED**) **[RECONCILER]** **DEMOTED — no driver repro block.** Re-driven by the reconciler (RC-M): matches. |
| 7.15 | `migrate-corpus` | the route, run verbatim | `jigc migrate-corpus` | 0 | none | — | `2 migrated HISTORY.md · docs/adrs/use-sqlite.md / committed <sha>`; git shows two renames, 2 insertions / 2 deletions; **each file's sha1 with the stamp put back equals its pre-move sha1** | matches — **no byte lost through the moving door** (D-2 placement + location legs CLOSED) |
| 7.16–7.19 | `doc show` ×2 · `validate` · `doc list` | after | — | 0 ×4 | none | — | both render; clean; both rows at the new homes | **the A7-F3 route runs** |
| 7.20–7.26 | `validate` · `doc list` · `doc show` · `relocate` · `migrate-corpus` · `doc show` · `validate` | **A7-F3 on a NEW doctype** — `inconsistency` relocated v1 → v2 in the listed methodology copy | §3 R-7b | 1 · 0 · **1** · 1 · 0 · 0 · 0 | `store.not-found` with the prior-home route | M | `doc show` names `inconsistencies/<slug>.md` as *a prior home of `inconsistency`* and routes at `migrate-corpus`, which lands it at `disagreements/`; bytes preserved modulo the stamp | CLOSED there too **[RECONCILER]** members with no driver repro block — 7.20, 7.21, 7.25, 7.26 — **demoted**; re-driven by the reconciler (RC-K): match. |
| 8.1–8.3 | `migrate-corpus` ×2 · `validate` | `EnumWidened` @ locus 1 — `adr.status` gains a member (v3 → v4) | `--dry-run` · bare · `validate` | 0 ×3 | none | — | `1 migrated`, the commit's diff is the stamp alone | matches (Applied) **[RECONCILER]** members with no driver repro block — 8.1, 8.3 — **demoted**; re-driven by the reconciler (RC-K): match. |
| 8.4–8.6 | `migrate-corpus` ×2 · `validate` | `RemovedField` @ locus 1 — `adr` drops `status.date`, which the committed doc carries (v4 → v5) | same | **1** · **1** · 1 | `migrate-corpus.removed-field` | H | *"the migration never strips a value (no data loss)"*; file sha1 unchanged; no commit | matches (Refused) — O-10 on the route's wording **[RECONCILER]** members with no driver repro block — 8.6 — **demoted**; re-driven by the reconciler (RC-K): match. |
| 8.7–8.12 | `validate` · `doc schema` · `validate` · `migrate-corpus` ×2 · `validate` | `EnumWidened` @ locus 1 **on `jigc-feedback`** (v1 → v2, snapshot shipped in the listed copy) | §3 R-8 | 0 · 0 · 1 · 0 · 0 · 0 | `schema-version-current` before | M | `1 migrated jigc-feedback/<slug>.md`, diff `-schema-version: 1 / +schema-version: 2` | matches — the first migration of a findings doctype, driven **[RECONCILER]** members with no driver repro block — 8.7, 8.8, 8.10, 8.12 — **demoted**; re-driven by the reconciler (RC-K): match. |
| 8.13–8.16 | `migrate-corpus` ×3 · `validate` | `ProseNeeding` @ **locus 2** on `inconsistency` — the `sides` item slot `says` made required (v2 → v3); committed items carry it empty | `--dry-run` · bare · `--format json` · `validate` | **1** ×4 | `migrate-corpus.prose-needed` | M | *"does not gate clean under its new schema, so its bytes are rolled back untouched"*; sha1 unchanged; no commit; json `blocked[0].key` | matches **[RECONCILER]** members with no driver repro block — 8.16 — **demoted**; re-driven by the reconciler (RC-K): match. |
| 8.17–8.24 | `start` · `doc set-slot` ×3 · `doc set-field` · `task finalize` · `migrate-corpus` · `validate` | that route followed: author through the write verbs (a `triage-inconsistency` task), re-run | §3 R-8 | 0 ×8 | none | — | `1 migrated disagreements/<slug>.md`; clean | matches — the route runs to green |
| 9.1–9.7 | `validate` · `doc list` · `describe` · `migrate-corpus` · `ingest` · `doc schema` · `validate --format json` | **`cwd-unreadable`** — the working directory removed under the process | the seven argvs | **1** ×7 | none | — | `cannot determine the current directory: No such file or directory (os error 2)`; json arm `{"error": …}` on stderr | matches the fence's own expectation (`pre_dispatch_faults.rs`: `Arm::Error`, exit 1) — code-less and route-less by declaration |
| 10.1 | `migrate-corpus` | **a dirty worktree**: an unstaged edit on the doc to be migrated, a staged unrelated file, an unstaged unrelated edit | `jigc migrate-corpus` | 0 | none | — | `committed <sha> — only the migrated paths were staged`; the staged file is still staged, the unrelated edit still unstaged — **and the uncommitted edit on the migrated doc is in the migration commit** | pathspec contract holds; **DEFECT `(R4, F5)`** on what the one migrated path carried |

**Rows driven: 419** — 37 + 37 + 215 + 34 + 7 + 31 + 58.

**[RECONCILER]** **419 as the driver counted; after the reconciliation: 362 on a driver repro block, 53 re-driven by the reconciler (all match), 1 not driven (5.7), 3 re-prints that are not rows (4.32–4.34).** See the ledger, L1.

---

## 3 · Repro blocks

Every block starts from

```
rig=$(dev/jigc-rig <state> [flags] --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
```

and `$JIGC` is that binary. "both docs filed" means the two `jigc start --workflow report-… → doc create
→ … → jigc task finalize` sequences of rows 2.7 / 2.8, run through the binary.

### R-1a · cell 1, the embedded packs (`fresh`)

```
$ jigc start --explain                         -> 0
  workflow:router    (pack-default · dev/1.0.0-rc.24)
    Pack input: dev/1.0.0-rc.24 = <embedded>  (blake3 1128e1be4cfe1609…)
    Pack input: methodology/1.0.0-rc.24 = <embedded>  (blake3 ecb065344c38ad06…)
$ jigc describe --format json                  -> 0   definitions: workflow/dev 18 · workflow/methodology 21
                                                      · doctype/dev 6 · doctype/methodology 12
$ jigc describe | grep -o '(dev pack)\|(methodology pack)' | sort | uniq -c   -> 16 · 17
$ jigc doc schema jigc-feedback | validate | doc list | migrate-corpus [--dry-run] | ingest   -> 0 each
# rig `bare`:  find "$RIG" (minus .git) before and after
$ jigc setup                                   -> 0   "install commit → <sha>"
  comm -13 before after | grep -v '^./repo/'   -> (empty)          # 14 new paths, all under repo/
```

### R-1b · cell 1, a `JIGC_PACK_DIR` copy (`fresh --repin`)

```
$ jigc start --explain   -> 0   "Pack input: dev/fs-local = $RIG/pack  (blake3 1128e1be4cfe1609…)"   # = the embedded digest
$ jigc doc schema adr | validate | doc list | migrate-corpus --dry-run | ingest   -> 0 each
$ jigc doc schema jigc-feedback   -> 1   blocking · store.unknown-type — unknown doctype `jigc-feedback`
                                         route: list the available doctypes with `jigc describe`
```

### R-1c · cell 1, `pack.resource-missing`, and `(R4, F1)`

```
mv "$JIGC_PACK_DIR/config/knobs.yaml" "$RIG/knobs.yaml.away"
$ jigc describe | validate | doc list | ingest | migrate-corpus --dry-run | doc schema adr | start --explain
       | unmanage README.md | relocate adr --from docs/x | migrate README.md --as adr      -> 1 each
  blocking · pack.resource-missing — no composed pack ships `config/knobs` — searched, highest-precedence
    first: dev/fs-local ($RIG/pack)
    route: restore `config/knobs.yaml` in a pack the composition already searches, or add the pack that
    ships it to `.jigc/config/packs.yaml`
$ jigc validate --format json   -> 1   stdout 0 b · stderr {"error": "blocking · pack.resource-missing — …"}

# (R4, F1) — a second rig, `fresh --repin`, one committed foreign file `foreign-adr.md`:
$ jigc task list                               -> 0   "no active tasks"          # BEFORE-CONTROL
  ls .jigc/tasks                               -> (no such directory)
mv "$JIGC_PACK_DIR/config/knobs.yaml" "$RIG/knobs.yaml.away"
$ jigc migrate foreign-adr.md --as adr         -> 1   blocking · pack.resource-missing — …   (as above)
$ jigc start --workflow quick-fix "a probe intent"   -> 1   (same)
$ jigc start "another probe intent"                  -> 1   (same)
  ls .jigc/tasks                               -> migrate-adr-foreign-adr-5304b572cd4f     # only migrate minted
       base.json  intent  staged-snapshot.json  workflow
mv "$RIG/knobs.yaml.away" "$JIGC_PACK_DIR/config/knobs.yaml"        # the route, followed
$ jigc migrate foreign-adr.md --as adr         -> 1
  blocking · task.serial-collision — task `migrate-adr-foreign-adr-5304b572cd4f` is already active
    route: resume with `jigc start --task migrate-adr-…` or abandon with `jigc task discard migrate-adr-… --force`
$ jigc start --task migrate-adr-foreign-adr-5304b572cd4f   -> 0   (composes)
```

The same on the **embedded** packs (rig `fresh`, row 4.28–4.31): with the stray-key `single-task`
shadow of R-4 in place, `jigc task list` → *no active tasks*; `jigc migrate idea-note.md --as idea` →
exit 1 `workflow-refs.malformed-front-matter`; `ls .jigc/tasks` → `migrate-idea-idea-note-1ec01db32a67`;
the re-run → exit 1 `task.serial-collision`. `jigc start` under the same shadow mints nothing.

### R-2 · cell 2 (`fresh`, both docs filed)

```
$ jigc doc schema jigc-feedback --format json   -> 0
  {"contract-version": 7, "type": "jigc-feedback", "schema-version": 1,
   "identity": {"kind": "slugged", "address": "jigc-feedback:<slug>"},
   "home": {"kind": "location", "path": "docs/jigc-feedback/<slug>.md"}, "fields": [10], "sections": [3]}
$ jigc doc schema inconsistency --format json   -> 0   … "home": {… "path": "docs/inconsistencies/<slug>.md"}, fields 4, sections 4
$ jigc migrate-corpus --dry-run --format json   -> 0
  {"migrated": [], "already_current": [2 paths], "blocked": [], "unadopted": [], "unfilled": [],
   "commit": null, "hook_output": "", "dry_run": true}
$ jigc relocate jigc-feedback --from docs/feedback   -> 1
  blocking · relocate.frozen-doctype — `jigc-feedback` is a frozen doctype — relocate it through the
    version-gated `jigc migrate-corpus`, not the freeze-exempt path
    route: `jigc migrate-corpus` walks every prior home the doctype's versioned snapshots declare and
    lands the move under the freeze
$ jigc migrate notes/fb.md --as jigc-feedback        -> 1
  doctype `jigc-feedback` exists but is not migratable (no `migrate-jigc-feedback` workflow); migratable
  doctypes: adr, arch-doc, changelog, completion-record, decisions-log, deferral-ledger, idea, prd,
  research, roadmap, spec, vision
    route: re-run `jigc migrate <path> --as <doctype>` with one of: …

# hand-placed, committed with plain git: a copy of each filed doc under a new H1 (conformant), and a
# front-matter-less note with a stray heading (non-conformant), at each home
$ jigc ingest   -> 0
  adoptable docs/jigc-feedback/hand-placed-conformant.md → jigc-feedback  (adopted — indexed + baselined, no file moved)
  needs-reconcile docs/jigc-feedback/loose-note.md → jigc-feedback
    blocking · conformance.section-renamed — section heading "Random heading" does not match required section `description`
    route: reconcile `docs/jigc-feedback/loose-note.md` against the `jigc-feedback` schema by hand, then
    re-run `jigc ingest` — no `migrate-jigc-feedback` workflow ships to rewrite it for you
  (the same pair for docs/inconsistencies/)
$ jigc validate   -> 1   advisory · schema-conformance.unadopted-instance ×2, exit-flip trailer
$ jigc doc list   -> 0   … loose-note … unregistered (×2), the rest managed
$ jigc unmanage docs/jigc-feedback/hand-placed-conformant.md   -> 0   "unmanaged … dropped its file-state baseline + forward edges; the file is left on disk …"
$ jigc migrate-corpus --dry-run   -> 0   "0 would migrate, 4 already current, 0 blocked, 2 not adopted"
```

### R-3 · cell 3, the door list and the control

```
DOORS = describe · doc schema jigc-feedback · doc schema inconsistency · doc schema adr · validate ·
        doc list · migrate-corpus --dry-run · migrate-corpus · migrate notes/fb.md --as idea · ingest ·
        unmanage docs/jigc-feedback/<slug>.md · relocate jigc-feedback --from docs/feedback ·
        relocate vision --from docs/vision-old · start --explain · start --workflow quick-fix probe-intent ·
        workflow quick-fix --preview · doc show jigc-feedback:<slug> · task list · config get docs-root
# control, no shadow: 0 everywhere but the two relocate rows (1, relocate.frozen-doctype)
```

### R-3a · project-layer shape shadows (the one stated exception to *never write into `.jigc/`*)

`.jigc/config/` is the tracked project layer (`git check-ignore` → not ignored). Each shadow is the
shipped `crates/cli/packs/methodology/schemas/<type>.yaml` with **one** edit, committed with plain git.

```
# a — .jigc/config/schemas/jigc-feedback.yaml:  of: [bug, inconvenience, feedback] -> […, question]
$ jigc <18 of DOORS>   -> 1 each, stdout 0 b
  pack-load freeze check failed: doctype `jigc-feedback`: schema-hash mismatch (manifest declares
  `7e16fb98352da837…`, recomputed `3ff572f1a5e27d08…`) — the project schema shadow
  $REPO/.jigc/config/schemas/jigc-feedback.yaml changes the shape of a frozen doctype, which the freeze
  forbids at every layer (`design/corpus-migration.md` → The freeze)
    route: `rm $REPO/.jigc/config/schemas/jigc-feedback.yaml` restores the frozen shape — a project schema
    shadow may only reword the authored presentation keys (`description:`, `usage:`, a slot `hint:`); …
$ jigc task list       -> 0   "no active tasks"            # loads no pack
$ jigc validate --format json   -> 1   stdout 0 b · stderr one {"error": …}
rm "$REPO/.jigc/config/schemas/jigc-feedback.yaml"         # the route, verbatim
$ jigc validate -> 0 ;  jigc doc schema jigc-feedback -> 0
# b — the line `{ id: about, type: string, optional: true }` deleted          -> the same 18 × exit 1
# c — .jigc/config/schemas/inconsistency.yaml:  location: inconsistencies/ -> disagreements/
  … doctype `inconsistency`: schema-hash mismatch (manifest declares `6b83fd12278eaa76…`, recomputed `28e70084b49be9d0…`) …
after each sweep:  git status --short -> (empty) ;  ls .jigc/tasks -> (none)
```

### R-3d · the variant sweep (worktree shadows, not committed; three doors each)

```
                                                  validate   doc schema <ty>   start --explain
e default open->resolved (jigc-feedback)          1 FREEZE   1 FREEZE          1 FREEZE
f inverse renamed                                 1          1                 1
g card "0..1" -> "0..n"                           1          1                 1
h set: on-create dropped                          1          1                 1
i optional slot made required (repro)             1          1                 1
j required field made optional (found-in)         1          1                 1
k id-from: title -> kind (inconsistency)          1          1                 1
l item slot `says` made required                  1          1                 1
m section renamed evidence -> proof               1          1                 1
n enum member removed (refuted)                   1          1                 1
o type: renamed inside the file                   1          1                 1
p description: reworded (jigc-feedback)           0 loads    0                 0
q usage: reworded                                 0          0                 0
r slot hint: reworded                             0          0                 0
s item-slot hint: reworded (inconsistency)        0          0                 0
t description: reworded (inconsistency)           0          0                 0
u comments stripped                               0          0                 0
```

### R-3f · committed prose-only shadows of both doctypes

```
$ jigc <DOORS>   -> 0 ×17, relocate ×2 -> 1 (relocate.frozen-doctype)
$ jigc describe | grep -o 'SHADOW[A-Z]*' | sort | uniq -c   -> 2 SHADOWDESC · 1 SHADOWUSAGE
$ jigc doc schema jigc-feedback --format json   -> schema-version 1, home docs/jigc-feedback/<slug>.md
$ jigc doc schema <either> --format agent|human|json | grep -c SHADOWHINT   -> 0     # hints render in no doc-schema arm
```

### R-3g · the methodology pack's own manifest, at the pack layer (a listed filesystem copy)

A second stated exception, of the same kind as the shadow one: `.jigc/config/packs.yaml` is the tracked
project-layer file the binary's own `pack.resource-missing` route tells an adopter to edit. Nothing in
the gitignored workbench was written.

```
rig: fresh --repin                                     # a dev pack copy with its manifest kept
cp -R crates/cli/packs/methodology "$RIG/methodology"
printf 'packs:\n  - %s\n' "$RIG/methodology" > .jigc/config/packs.yaml ; git commit -am …
$ jigc start --explain   -> 0
    Pack input: methodology/0.1.0 = $RIG/methodology  (blake3 ecb065344c38ad06…)     # = the embedded digest (R-1a)
    Pack input: dev/fs-local = $RIG/pack  (blake3 1128e1be4cfe1609…)
# both docs filed through the binary (they home at jigc-feedback/ and inconsistencies/ here)
sed -i '' 's/of: \[bug, inconvenience, feedback\]/of: [bug, inconvenience, feedback, question]/' \
    "$RIG/methodology/schemas/jigc-feedback.yaml"      # the methodology manifest NOT touched
$ jigc <18 of DOORS>   -> 1 each, stdout 0 b
  pack-load freeze check failed: doctype `jigc-feedback`: schema-hash mismatch (manifest declares
  `7e16fb98352da837…`, recomputed `3ff572f1a5e27d08…`)                 # no route line
```

### R-3h · absent manifest entry, on that listed copy

```
# delete the three lines of `- type: inconsistency` from "$RIG/methodology/config/schema-manifest.yaml"
$ jigc describe | doc schema inconsistency | validate | doc list | migrate-corpus --dry-run | ingest
       | relocate inconsistency --from inconsistencies | start --explain | doc show inconsistency:<slug>   -> 1 each
  pack-load freeze check failed: doctype `inconsistency` is shipped but absent from the freeze manifest
```

### R-4 · cell 4, and `(R4, F2)`

```
rig: fresh
# .jigc/config/workflows/report-jigc-feedback.yaml = the shipped file with ONE edit, committed:
#   line 10:   - { type: jigc-feedback, as: feedback, new: true }  ->  … nwe: true }
$ jigc start --workflow report-jigc-feedback probe-one   -> 1
  blocking · workflow-refs.malformed-front-matter — workflow front-matter is malformed config-family YAML:
  allows-create[0]: unknown field `nwe`, expected one of `type`, `as`, `new` at line 9 column 42
    route: fix the workflow/step/catalog definition the message names (a definition defect, repaired once
    at its source), then re-run
$ jigc workflow report-jigc-feedback --preview | describe | start probe-two   -> 1, the identical two lines
$ jigc workflow quick-fix --preview                       -> 1, the identical two lines      # asked for quick-fix
$ jigc start --workflow quick-fix probe-three             -> 1, the identical two lines
$ jigc validate                                           -> 0
  blocking · workflow-refs.malformed-front-matter — … unknown field `nwe` … at line 9 column 42
    at: workflow:report-jigc-feedback                     # the ONLY surface that names the definition
    route: fix the workflow/step/catalog definition the message names …
  1 finding(s) — report-only at store scope (exit 0); these gate at compose (`jigc start`), never at the
  task or milestone boundary.
$ jigc start --explain | doc list | doc schema jigc-feedback | migrate-corpus --dry-run | ingest   -> 0 each

# stray key — .jigc/config/workflows/single-task.yaml, file line 6:
#   allows-create: [{ type: adr, as: decision, note: stray }, {type: changelog, as: change}]
$ jigc start --workflow single-task … | workflow single-task --preview | describe | start … |
       workflow report-jigc-feedback --preview            -> 1
  … allows-create[0]: unknown field `note`, expected one of `type`, `as`, `new` at line 5 column 44
# control: `new: true` on that entry -> workflow --preview 0, describe 0, validate 0 (clean)
# `new: maybe` -> "allows-create[0].new: invalid type: string "maybe", expected a boolean"
# `as:` twice  -> "allows-create[0]: duplicate field `as`"
```

### R-5 · cell 5

```
rig: fresh
$ jigc doc schema planning-record [--format json] | grep -i 'cheap-vs-robust\|claim-driven'
    - cheap-vs-robust: slot (set-slot: planning-record:<slug>#cheap-vs-robust)      # the slot, no hint text
    - claim-driven: slot (set-slot: planning-record:<slug>#claim-driven)
$ jigc workflow planning --preview   -> 0 (44 404 b)
    …every fork lists the option that removes the artifact that can drift**, stated and priced like the others…   (×2)
    …**An advocate's proposal and the orchestrator's halt recommendations are claims too** — each is driven end to end…   (×2)
$ jigc start --workflow planning "m1 probe milestone" ; doc create planning-record --title … ; 14 × doc set-slot ;
  commit type + summary ; jigc task finalize m1-probe-milestone   -> 0
    finalized <sha> … promoted docs/planning-records/m1-probe-milestone.md
$ jigc validate                 -> 0   clean
$ jigc migrate-corpus --dry-run -> 0   "0 would migrate, 1 already current, 0 blocked"
```

### R-6 · cell 6

```
rig: fresh, both docs filed
# unstamped: delete the line `schema-version: 1` from the committed jigc-feedback doc, commit with git
$ jigc validate   -> 1   blocking · schema-conformance.schema-version-current — … field `schema-version` is absent;
                         the committed doc predates the schema-version stamp (below the current schema-version 1)
                         route: migrate — … run `jigc migrate-corpus` to upgrade it
$ jigc doc list   -> 0   … managed
$ jigc ingest     -> 0   adoptable … (adopted) + advisory · file-state.absorbed
$ jigc migrate-corpus   -> 0   "1 migrated … committed <sha> — only the migrated paths were staged"
  git show HEAD   -> "+schema-version: 1" and nothing else ;  trailer Co-Authored-By: Claude <noreply@anthropic.com>
# ahead: `schema-version: 1` -> `2` in the committed inconsistency doc, commit with git
$ jigc validate   -> 1   blocking · schema-conformance.schema-version-ahead — … above the current schema-version 1 …
$ jigc migrate-corpus [--dry-run]   -> 1   blocked … migrate-corpus.schema-version-ahead … route: upgrade jigc … or restore the stamp
# orphaned: a second rig (fresh, both docs filed), then
export JIGC_PACK_DIR=<the --repin dev pack copy of another rig>         # the composition is now dev alone
$ jigc validate   -> 1   blocking · schema-conformance.orphaned-instance ×2 — … no resolved doctype claims this path …
$ jigc doc list   -> 0   "(none)  docs/jigc-feedback/<slug>.md  orphaned" (×2) ; json id null, item-count null, fields null
$ jigc ingest     -> 0   "unmanaged docs/jigc-feedback/ — 1 file(s) parse against no schema (… fine to stay plain)"
$ jigc doc schema jigc-feedback | doc show jigc-feedback:<slug>   -> 1   store.unknown-type
```

### R-F3 · `(R4, F3)` — `relocate`'s route, run verbatim, on a doctype with no snapshot

```
rig: fresh, both docs filed
git mv docs/jigc-feedback/<slug>.md docs/feedback/<slug>.md ; git commit -m "move the feedback doc out of band"
$ jigc ingest   -> 0
  needs-reconcile docs/feedback/<slug>.md → jigc-feedback
    blocking · ingest.wrong-location — conformant `jigc-feedback` at `docs/feedback/<slug>.md` sits outside `docs/jigc-feedback/` …
    route: move docs/feedback/<slug>.md into docs/jigc-feedback/, then re-run `jigc ingest`
$ jigc relocate jigc-feedback --from docs/feedback   -> 1
  blocking · relocate.frozen-doctype — …
    route: `jigc migrate-corpus` walks every prior home the doctype's versioned snapshots declare and lands
    the move under the freeze
$ jigc migrate-corpus   -> 0
  corpus migration: 0 migrated, 1 already current, 0 blocked
    current    docs/inconsistencies/<slug>.md                    # the stranded doc: not named, not moved
ls docs/feedback   -> <slug>.md                                   # still there
$ jigc doc show jigc-feedback:<slug>   -> 1   store.not-found, the generic route
```

Contrast, same binary: on a doctype that **has** a versioned snapshot the identical route **acts**
(rows 7.14 → 7.15, and 7.23 → 7.24 on `inconsistency` once a `v1` snapshot exists).

### R-7 · `(7, A7-F3)` re-driven — both home kinds

```
rig: fresh --repin ; an adr (`record-decision`) and a changelog (`record-change`) filed by the binary
P=$JIGC_PACK_DIR
cp $P/schemas/changelog.yaml $P/schema-snapshots/changelog.v2.yaml ; cp $P/schemas/adr.yaml $P/schema-snapshots/adr.v2.yaml
changelog.yaml:  placement: { file: CHANGELOG.md } -> { file: HISTORY.md }       # the ONLY edits
adr.yaml:        location: decisions/ -> adrs/
manifest: adr 2 -> 3, changelog 2 -> 3 ; re-pin each hash from the binary's own "recomputed `…`" line
$ jigc doc schema changelog | adr   -> 0  "(schema-version 3)"

$ jigc validate   -> 1   schema-conformance.schema-version-current ×2, route: run `jigc migrate-corpus`
$ jigc doc list   -> 0   adr:use-sqlite  docs/decisions/use-sqlite.md  managed ; changelog:changelog  CHANGELOG.md  managed
$ jigc ingest     -> 0   ingest.wrong-location ×2, route: move … by hand, then re-run `jigc ingest`
$ jigc doc show changelog   -> 1
  blocking · store.not-found — could not read `changelog:changelog` at `HISTORY.md`: No such file or directory (os error 2)
    at: changelog:changelog
    route: `changelog:changelog` is committed at `CHANGELOG.md`, a prior home of `changelog` — the schema's
    home moved and this corpus has not been migrated; run `jigc migrate-corpus` to land it at the home this
    read resolves, then read it again
$ jigc doc show adr:use-sqlite   -> 1
  blocking · store.not-found — could not read `adr:use-sqlite` at `docs/adrs/use-sqlite.md`: …
    route: `adr:use-sqlite` is committed at `docs/decisions/use-sqlite.md`, a prior home of `adr` — … run `jigc migrate-corpus` …
$ jigc doc show changelog:changelog --format json   -> 1   {"schema_version": 3, "findings": [{… "code": "store.not-found",
    "key": {"code": "store.not-found", "target": "changelog:changelog"}, "route": "`changelog:changelog` is committed at `CHANGELOG.md`, a prior home …"}]}

shasum CHANGELOG.md docs/decisions/use-sqlite.md   -> 71a02bce… · b009b642…          # BEFORE
$ jigc migrate-corpus   -> 0   "2 migrated  HISTORY.md  docs/adrs/use-sqlite.md / committed <sha> — only the migrated paths were staged"
  git show --stat HEAD  -> CHANGELOG.md => HISTORY.md | 2 +- ; docs/{decisions => adrs}/use-sqlite.md | 2 +-
sed 's/^schema-version: 3$/schema-version: 2/' HISTORY.md | shasum               -> 71a02bce…          # AFTER, stamp put back
sed 's/^schema-version: 3$/schema-version: 2/' docs/adrs/use-sqlite.md | shasum  -> b009b642…
$ jigc doc show changelog | doc show adr:use-sqlite   -> 0 each, render
$ jigc validate   -> 0 clean ;  jigc doc list -> both rows at the new homes, managed
```

### R-7b / R-8 · the kind cells, incl. the two new doctypes (the listed methodology copy of R-3g)

```
# EnumWidened @1, adr (dev copy):   status … superseded] -> … superseded, rejected]   (v3 -> v4, adr.v3.yaml shipped)
$ jigc migrate-corpus   -> 0   "1 migrated docs/adrs/use-sqlite.md" ; diff: -schema-version: 3 / +schema-version: 4
# RemovedField @1, adr:             the `{ id: date, … }` line deleted                (v4 -> v5)
$ jigc migrate-corpus [--dry-run]   -> 1
  blocked docs/adrs/use-sqlite.md
    migrate-corpus.removed-field: `adr` drops the declared field `status.date` between schema-version 4 and 5,
    so `docs/adrs/use-sqlite.md` cannot be migrated: committed instances still carry the field, and the
    migration never strips a value (no data loss) — … This is a schema-authoring gap, not a doc problem
  shasum before == after ; git log unchanged
# EnumWidened @1, jigc-feedback (listed methodology copy, v1 -> v2, jigc-feedback.v1.yaml shipped)
$ jigc validate   -> 1 schema-version-current ;  jigc migrate-corpus -> 0 "1 migrated jigc-feedback/<slug>.md", stamp-only diff
# Relocated (location), inconsistency (v1 -> v2):  location: inconsistencies/ -> disagreements/
$ jigc doc show inconsistency:<slug>   -> 1  store.not-found, route names `inconsistencies/<slug>.md` as a prior home + `jigc migrate-corpus`
$ jigc relocate inconsistency --from inconsistencies   -> 1  relocate.frozen-doctype
$ jigc migrate-corpus   -> 0  "1 migrated disagreements/<slug>.md" ; sha1 with the stamp put back == sha1 before
# ProseNeeding @2, inconsistency (v2 -> v3):  `says, slot: { optional: true, …` -> `says, slot: { …`
$ jigc migrate-corpus [--dry-run] [--format json]   -> 1
  blocked disagreements/<slug>.md
    migrate-corpus.prose-needed: … does not gate clean under its new schema, so its bytes are rolled back
    untouched — the conformance gate reports: required slot `says` in item `sides/readme` is empty …
    route: author the new required prose … through the write verbs, then re-run `jigc migrate-corpus`
  shasum before == after
$ jigc start --workflow triage-inconsistency "fill says" ; 2 × doc set-slot …#sides/<id>/says ; commit doc ; task finalize   -> 0
$ jigc migrate-corpus   -> 0  "1 migrated disagreements/<slug>.md" ;  jigc validate -> 0 clean
```

### R-9 · `cwd-unreadable`

```
mkdir gone-dir ; cd gone-dir ; rmdir "$REPO/gone-dir"
$ jigc validate | doc list | describe | migrate-corpus --dry-run | ingest | doc schema jigc-feedback   -> 1 each
  cannot determine the current directory: No such file or directory (os error 2)
$ jigc validate --format json   -> 1   stderr {"error": …}
```

### R-F5 · `(R4, F5)` — the migration commit carries an uncommitted edit

```
rig: fresh, both docs filed ; the stamp line stripped from the committed jigc-feedback doc and committed (as R-6)
echo PLANT-STAGED > staged.txt ; git add staged.txt
echo PLANT-DIRTY-README >> README.md
sed -i '' 's/…no pack version\.$/…no pack version. PLANT-UNCOMMITTED-EDIT/' docs/jigc-feedback/<slug>.md      # NOT staged, NOT committed
git status --short                 ->  " M README.md" · " M docs/jigc-feedback/<slug>.md" · "A  staged.txt"   # BEFORE-CONTROL
$ jigc migrate-corpus   -> 0
  corpus migration: 1 migrated, 1 already current, 0 blocked
    migrated   docs/jigc-feedback/<slug>.md
  committed 3d9b287 — only the migrated paths were staged
git status --short                 ->  " M README.md" · "A  staged.txt"
git show --stat HEAD               ->  docs/jigc-feedback/<slug>.md | 3 ++-      "chore(jigc): migrate the managed corpus to the current schema versions"
git show HEAD:docs/jigc-feedback/<slug>.md | grep -c PLANT-UNCOMMITTED-EDIT   -> 1     # the human's uncommitted edit is in the commit
grep -c PLANT-UNCOMMITTED-EDIT docs/jigc-feedback/<slug>.md                    -> 1     # and still on disk — nothing lost
```

---

## 4 · Findings

Tier predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying or
moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the binary
does not do.

### `(R4, F1)` · `jigc migrate` leaves a minted task behind a refusal that exits 1 — proposed **tier 3**

- **Door / cell:** `migrate` × a compose-time refusal (`pack.resource-missing`; `workflow-refs.malformed-front-matter`).
- **Driven:** R-1c (a `JIGC_PACK_DIR` copy) and rows 4.28–4.31 (the **embedded** packs, an unrelated
  malformed project workflow shadow). Before-control `jigc task list` → *no active tasks*; the door exits
  **1**; afterwards a `migrate-<type>-<stem>-<hash>` task directory exists and `task list` shows it. The
  refusal names neither the task nor that anything was written. After the stated repair, the same argv
  answers `task.serial-collision`, whose route (`jigc start --task <id>` / `jigc task discard <id> --force`)
  **works** — so it is not a dead end.
- **Contract contradicted:** `jigc start` under the identical fault mints nothing (rows 1.31–1.32, 4.7,
  4.9); `migrate`'s own pre-mint discipline is *"a rejected migrate leaves `jigc task list` unchanged"*
  (`crates/cli/src/migrate.rs`, `ensure_migratable`'s doc). The same file's `migrate_in_repo` doc says a
  failed compose *"leaves the minted task in place for inspection / re-entry"* — so the retention is
  deliberate in the source, and what is missing is the surface: the refusal does not say it.
- **Why tier 3, in one clause:** a `blocking` refusal reads as *nothing happened* while the binary minted
  a task; no loss, exit 1, and the second-level route recovers.

### `(R4, F2)` · the malformed-front-matter refusal names no definition at the three compose doors, and its line number is off by one — proposed **tier 2**

- **Door / cell:** `start` · `workflow --preview` · `describe` (and `migrate`) × a project workflow shadow
  with an unknown `allows-create` key.
- **Driven:** R-4. The message names the **key** (as `design/findings-channel.md` §10 requires) and
  **nothing else**: no workflow id, no file, no `at:` line. The route reads *"fix the workflow/step/catalog
  definition **the message names**"* — the message names none. It is printed identically for a workflow
  the caller did not ask for (`jigc workflow quick-fix --preview` fails on a defect in
  `report-jigc-feedback`). The position is counted from the first line **after** the opening `---`
  (`line 9` for file line 10; `line 5` for file line 6), so it is one short against the file an editor
  opens. `jigc validate` — exit 0, report-only — is the one surface carrying `at: workflow:<id>`, and no
  refusal routes there.
- **Why tier 2, in one clause:** one bad shadow blocks every compose door behind a route that cannot be
  followed from the surface that printed it — the same shape the baseline graded tier 2 at `(7, A7-F3)`;
  a reader who grades it by *"the exit exists at another door"* would call it tier 3.
- **Not range-specific:** the generic flatten path is older than M55; M55's closed-key rule is what makes
  it reachable through a one-letter typo.

### `(R4, F3)` · `relocate.frozen-doctype`'s route is a no-op for a frozen doctype that has no versioned snapshot — both M55 doctypes — proposed **tier 2**

- **Door / cell:** `relocate` × `jigc-feedback` / `inconsistency` (rows 2.16–2.19, 6.29–6.30).
- **Driven:** R-F3. A stamped `jigc-feedback` doc sitting at a non-home; `jigc relocate jigc-feedback --from
  docs/feedback` → exit 1, route *"`jigc migrate-corpus` walks every prior home the doctype's versioned
  snapshots declare and lands the move under the freeze"*; `jigc migrate-corpus`, run verbatim → exit 0,
  `0 migrated`, the doc neither named nor moved. A schema-version-1 doctype has no versioned snapshot
  (`design/findings-channel.md` §1.7: *no snapshot and no migration*), so the walk the route describes is
  empty by construction. On a doctype that has one, the same route acts (7.14 → 7.15).
- **The working exits, at other doors, same commit:** `ingest` (*move it into `docs/jigc-feedback/`, then
  re-run `jigc ingest`*) and `validate` (*revert the `git mv`*).
- **Why tier 2, in one clause:** a route that, followed exactly, changes nothing, on the door whose whole
  subject is a stranded instance; reachable only after an out-of-band move, which is the argument for
  tier 3.
- **Scope of the class:** every manifest-listed doctype still at its first schema-version, not only the
  two new ones (`vision`, `roadmap`, `research`, `idea`, `spec`, `prd`, `arch-doc`, … — read off the
  manifests, **not** each driven).

### `(R4, F4)` · `jigc relocate --help`'s example doctype is one the door refuses — proposed **tier 3**

- **Door / cell:** `relocate` × its own help text.
- **Driven:** `jigc relocate --help` → `<TYPE>  The freeze-exempt doctype id whose schema home moved (e.g.
  \`vision\`)` and `--from … a directory (\`docs/vision/\`) or a literal file (\`docs/vision.md\`)`;
  `jigc relocate vision --from docs/vision-old` → exit 1 `relocate.frozen-doctype — \`vision\` is a frozen
  doctype` (R-3, in the control and in every sweep). On a stock composition no shipped persisted doctype
  is freeze-exempt (`design/corpus-migration.md` → *M40 … empties this path's domain*); M55 adds two more
  to the refused set.
- **Why tier 3:** the help exemplifies an argv the binary refuses. Pre-existing since M40; help text is
  row 9's subject, recorded here because it was driven here.

### `(R4, F5)` · `migrate-corpus` commits an uncommitted edit on a doc it migrates, un-narrated — proposed **tier 3**

- **Door / cell:** `migrate-corpus` (a committing door) × a dirty worktree.
- **Driven:** R-F5, with a before-control. The pathspec contract **holds**: the unrelated staged file
  stayed staged and the unrelated unstaged edit stayed unstaged (`design/corpus-migration.md` → *an ambient
  dirty tree is never swept in*). But the one migrated path is committed **as it stands in the worktree**,
  so a human's unstaged, uncommitted prose edit on that doc lands inside `chore(jigc): migrate the managed
  corpus to the current schema versions` (with the agent co-author trailer), and neither the ack
  (*"only the migrated paths were staged"*) nor `--dry-run` says so.
- **Contract contradicted:** `jigc migrate-corpus --help` — *"re-parses every **committed** doc … folds the
  deterministic diff … into its bytes"*: the bytes it folded into and committed were the worktree's, not
  the committed copy's.
- **Why not tier 1, stated so it can be overruled:** **no byte is lost** (the edit is in `HEAD` and on
  disk), nothing outside the migrated path is committed, and it is undone by `git reset --soft HEAD~1`.
  Whether *a human's uncommitted edit committed under jigc's message at exit 0* is *repository harm* is
  the predicate's call; I read harm as damage, and this is a mislabelled but intact commit.
- **Not established:** whether this predates the range — no older binary is installed. The range's one
  change on this seam (`28ca86b4`) moved the commit message into `git_commit_capture` and left the
  pathspec alone.

---

## 5 · What I did NOT drive, and why

1. **The absent-manifest-entry arm on an EMBEDDED pack** — needs an embedded-pack rebuild. Driven instead
   on a listed filesystem copy (row 3.h), which is the same assertion on a different `PackSource`.
2. **The cross-version half** (*the existing corpus is read unchanged across the range*) — no older binary
   is installed and none may be built. Only the forward half: corpora built by rc.24 validate clean.
3. **A reshaped EMBEDDED methodology pack** — out of reach. The project layer (14 shape edits) and a listed
   filesystem copy of the pack are what was driven.
4. **`SchemaChangeKind × LOCI` beyond five cells.** This run drove `Relocated` (location ×2, placement ×1;
   doctype-level), `EnumWidened` @1 (×2), `RemovedField` @1, `ProseNeeding` @2. I did not reconstruct which
   14 cells the baseline drove, so I claim no complement: the lead stays open.
5. **`RelocateRefusal::ALL`, 9 of 10** — only `FrozenDoctype` is reachable on any composition built here;
   the other nine need a freeze-exempt doctype (a manifest-less third pack).
6. **`relocate`'s success arm** — same reason.
7. **Shape variants e–o at the other 16 `DOORS` members** (176 pairs). Each was driven at three doors; the
   full 19-door sweep was driven for shadows a, b, c and for the pack-layer arm. Not extrapolated into rows.
8. **Prose variants p–u singly at the other 16 doors** — covered jointly by the committed combined shadow
   (3.f), not one by one.
9. **`pack.resource-missing` for any resource but `config/knobs`.**
10. **`migrate` · `unmanage` · `describe` · `doc schema` × the *unstamped* and *ahead* cells** (8 pairs) —
    the baseline shows them unaffected (b1/b2/b6/b8, c1/c2/c6/c8); not re-driven on the new doctypes.
11. **`--format human`** on any door but `doc schema`.
12. **`setup` writing outside `$RIG`.** The diff covers `$RIG` (the repo and the repointed `HOME`). A write
    to a system temp directory or anywhere else on the host would not have been seen.
13. **The setup-marker composition with a listed pack beside it** (`compose-embedded-methodology: true`
    **and** `packs:`) — the listed-pack rig replaced the marker with the list.
14. **M51 `C-1` (placement identity at `ingest`/`unmanage`) and `D-3` (`ValueRemapped` on an `id-from`
    enum)** — no cell of this row passes through them.
15. **`ManifestKind::ALL`** — the finalize left-out vocabulary; no door of this row renders it. Row 5's.

---

## 6 · Observations (driven, not defects)

- **O-1** — a **pack-layer** freeze block carries no route line (rows 3.g, 3.h, 7.3); the project-layer
  one carries `rm <shadow>`. As baseline O-1.
- **O-2** — `doc schema`'s text arms print no home and no identity (the json arm carries both), and **no
  arm prints a slot `hint:`**. The reworded hints surface in the composed `planning` workflow (row 5.3).
  The scope's cell-5 wording (*"`doc schema planning-record` shows the reworded … hints"*) describes a
  surface the binary does not have; no design doc claims it either.
- **O-3** — `describe` names both packs (`origin_pack`, `(dev pack)` / `(methodology pack)`) and carries no
  pack version; `start --explain` carries name, version, source and digest.
- **O-4** — `ingest` is currency-blind on the new doctypes exactly as on the old (unstamped and ahead both
  `adoptable … (adopted)`), as baseline O-4; and it files orphans under *no action needed*, which is
  `(7, A7-F2)`, row 3's.
- **O-5** — `migrate --as <new doctype>` is a **code-less** refusal whose route lists the migratable set;
  `ingest`'s legend still says *"To bring one under management: `jigc migrate <path> --as <doctype>`"*
  without that carve-out. `ingest`'s own per-row route is exact (*no `migrate-<id>` workflow ships*).
- **O-6** — the project-layer freeze message and its `rm` route print an absolute host path.
- **O-7** — `validate` reports a malformed workflow front-matter as `blocking` at **exit 0** with the
  trailer *"report-only at store scope … these gate at compose"*. That is the store contract; it is not
  the *refused at `validate`* the scope's cell 4 words.
- **O-8** — under a bare `JIGC_PACK_DIR` the methodology pack is not composed although `packs.yaml` carries
  the setup marker, so both findings doctypes are unknown there and committed findings read as orphans
  (rows 1.11–1.18, 6.17). This is how the orphan cell was built.
- **O-9** — `(7, A7-F3)`'s closure is scoped to **snapshot-declared** prior homes. A doc moved out of band
  still gets `store.not-found` with the generic route at `doc show` (row 6.31), while `validate` names the
  repair. Outside the baseline row's statement (a *relocated* doc); recorded so the closure is not read
  wider than it is.
- **O-10** — `migrate-corpus.removed-field`'s route names jigc's own source files
  (`crates/engine/src/schema_diff.rs` + `transform.rs`) — a pack author's route, not an adopter's.
- **O-11** — the filesystem copies of `crates/cli/packs/dev` and `crates/cli/packs/methodology` at this
  checkout hash to the digests the installed binary prints for its embedded packs (`1128e1be…`,
  `ecb06534…`): the tree read here is the tree rc.24 embeds.
- **O-12** — every `migrate-corpus` commit in these rigs carries `Co-Authored-By: Claude
  <noreply@anthropic.com>`. Expected; row 10's.
- **O-13** — `task list` is the one driven door that answers under a freeze block: it loads no pack.
- **O-14** — a driver error, kept because it is a driven row: `doc set-field commit:<id>#summary` →
  exit 1 `write.unknown-field` with a route to `jigc doc schema commit` (`summary` is a slot). Correct.

---

## 7 · Baseline rows: CLOSED / STILL-OPEN

| key | status | argv + observed |
|---|---|---|
| **`(7, A7-F3)`** — `doc show` refuses a relocated managed doc with a route that names none of the repair paths | **CLOSED — both arms** | **placement:** `jigc doc show changelog` → 1 `store.not-found`, route *"`changelog:changelog` is committed at `CHANGELOG.md`, a prior home of `changelog` — … run `jigc migrate-corpus` to land it at the home this read resolves, then read it again"*. **location:** `jigc doc show adr:use-sqlite` → the same, naming `docs/decisions/use-sqlite.md`. The route run verbatim → `2 migrated`, both reads then exit 0. Also driven on **`inconsistency`** (location arm, listed pack copy). Bound: O-9 |
| §D lead — **the methodology pack's own manifest** | **NOW DRIVEN** (the embedded arm excepted) | project layer: 14 shape edits over the two new methodology doctypes block, 6 prose edits load (rows 3.a–3.f). Pack layer: a **listed filesystem copy** of the methodology pack, reshaped without a re-pin, blocks 18 doors naming the **methodology** manifest's declared hash (3.g); with a bump + snapshot + re-pin it loads and `migrate-corpus` migrates (8.7–8.24). A reshaped **embedded** methodology pack: not reachable |
| §D lead — **40 of the 54 `SchemaChangeKind × LOCI` cells** | **STILL-OPEN** | five more cells driven here (§5.4), two of them on the new doctypes; no complement claimed. `schema_diff.rs` last changed 2026-09-06, before rc.16, so the open cells are the same code the baseline left open |
| §D lead — **`ManifestKind::ALL`** | **NOT THIS ROW'S** | the finalize left-out vocabulary; row 5 |
| §D lead — **`cwd-unreadable`** | **NOW DRIVEN** | seven argvs → exit 1, code-less, route-less, as the fence declares (R-9) |
| §D lead — **`pack-resource-missing`** | **NOW DRIVEN** | ten doors + json → exit 1 `pack.resource-missing` with a route that repairs (R-1c); one finding at `migrate`, `(R4, F1)` |
| §D lead — **CX-8's third origin pack** | **NOW DRIVEN at a listed filesystem pack** | per-origin manifest resolution driven on a pack that is neither the embedded dev base nor the project layer (3.g, 3.h). A manifest-owning pack **beyond** dev + methodology: not driven |
| §D lead — **CX-9 as behaviour** | **STILL-OPEN** | as the 54-cell lead |
| §D lead — **CX-13** | **NOT THIS ROW'S / STILL-OPEN** | the registries it names (`ROLLBACK_POPULATIONS`, `TASK_AREA_FILES`, `DESTROYING_DOORS`, …) have no member among this row's doors; `RelocateRefusal::ALL` stays 1 of 10 (§5.5) |
| M51 **D-1** (CLOSED on rc.16) | **passed through — still CLOSED** | row 6.3: `ingest` prints `advisory · file-state.absorbed` naming the finding it retires |
| M51 **D-2** (CLOSED on rc.16) | **passed through — still CLOSED, all three legs** | rows 7.14–7.15 (placement + location + the route leg), and 7.23–7.24 on `inconsistency` |
| M51 **D-4 half 1** (CLOSED on rc.16) | **passed through — still CLOSED** | row 6.17: *"no resolved doctype claims this path"* |
| M51 **C-1**, **D-3** | **NOT RE-DRIVEN** | no cell of this row passes through them (§5.14) |
| `(7, A7-F1)` · `(7, A7-F2)` · `(7, C-2)` · the `probe-unreliable` lead | **NOT RE-DRIVEN — row 3's** | `(7, A7-F2)`'s shape was re-observed on the new doctypes (row 6.20) and is **not** re-filed here |

**No baseline row regressed. No tier-1 row was found on this row.**

---

## 8 · Doors covered

Every clap leaf that is the door of at least one driven row (`VERB_KINDS` spelling):

`start` · `workflow` · `setup` · `ingest` · `migrate` · `migrate-corpus` · `unmanage` · `relocate` ·
`describe` · `validate` · `doc create` · `doc add-item` · `doc set-field` · `doc set-slot` · `doc show` ·
`doc schema` · `doc list` · `task list` · `task finalize` · `config get` — **20 of 48**.

---

# Reconciliation ledger — ROW 4

Reconciler: a third party to both inputs — it did not author the driver table above and did not build
the code. Rule applied (`completions/artifacts/M51/acceptance-design.md` → *The reconciliation rule*): a
claim by one that the other cannot reproduce is a **lead**, not a finding. Every Codex claim was entered
as a lead and then **driven**; every driver defect was re-driven once.

- **Binary:** `~/.local/bin/jigc` → `jigc 1.0.0-rc.24`, asserted before every rig load. Release posture.
- **Rigs:** twelve, each `dev/jigc-rig <state> --binary ~/.local/bin/jigc`, stdout-only capture, two-step
  eval, the `[ -n "$REPO" ]` guard plus a check that the working directory sits under the scratch root
  before any git command. Every root from `mktemp -d`. No teardown, no recursive removal. The working
  repository was read only: its `git status` is empty and its `HEAD` is unchanged after the run.
- **`CLAUDECODE`:** set, held constant. Every commit jigc made in a rig carries the trailer
  `Co-Authored-By: Claude <noreply@anthropic.com>` — row 10's.
- **Verdict up front:** **six confirmed findings — five of the driver's, one of Codex's — and no tier-1
  row.** Two tier 2, four tier 3. `(7, A7-F3)` is **CLOSED** on both home kinds.

## L0 · What the reconciler did that the instrument does not grant — read this before trusting a row

1. **`.jigc/config/packs.yaml` was hand-written** in one rig (a `packs:` list naming filesystem pack
   copies). The scope file grants hand-written schema and workflow shadows only; this file is a third
   kind. It was taken because the Codex claim under test is about a *filesystem* pack beside another
   manifest-owning pack, no verb writes a `packs:` list (`design/multi-pack.md`: *a hand-authored
   `packs.yaml`*), the file is tracked (`git check-ignore` → exit 1), and the driver took the same step
   at R-3g. **Rows that depend on it: RC-C1, RC-C3, RC-C5, RC-K.** Nothing was written into the
   gitignored workbench anywhere.
2. **An observational tracer** (RC-C1) — a 30-line library that logs each `open()` of a matching path
   with a timestamp and the result, and changes no result. It was loaded into the unmodified binary for
   the three RC-C1 drives only (`DYLD_INSERT_LIBRARIES` set for those invocations, unset everywhere
   else). Its job is to tell two exit-0 arms apart that no door's output distinguishes.
3. **A widened window** in two of the three RC-C1 drives: 198 MB of `#` comment lines appended to one
   schema of the *other* pack, which leaves that schema's hash unmoved (driver row 3.e) and makes its
   load take about one second. The third RC-C1 drive used unpadded packs.
4. **One reconciler error, recorded because it minted tasks:** the first attempt at RC-F1's embedded arm
   wrote the shadow into a directory `git rm` had just emptied, so no shadow existed and two tasks were
   minted by ordinary successful calls. Both were discarded through `jigc task discard <id> --force`
   (exit 0) before the cell was driven again.

## L1 · The driver's table: rows with no repro block

Standard applied: a **numbered** row is driven only if a §3 block (or, for the two filing sequences, the
§3 preamble that defines them) shows its argv with its exit and the surface the row asserts. Each row
that fails it is marked **`[RECONCILER]`** in the table above.

| class | rows | count |
|---|---|---|
| whole row, no block | 1.30 · 2.23 · 5.7 · 6.4 · 6.5 · 6.7 · 6.9 · 6.10 · 6.14 · 6.15 · 6.16 · 6.21 · 6.24 · 6.25 · 6.26 · 6.27 · 7.13 · 7.14 | 18 |
| member of a grouped row, no block | 1.18 · four text arms of 2.1–2.6 · 2.11 · three of 2.12–2.15 · 2.17–2.19 · 2.21–2.22 · 2.31–2.34 · 2.36–2.37 · 4.4 · 4.6 · 4.20 · 6.13 · 7.20 · 7.21 · 7.25 · 7.26 · 8.1 · 8.3 · 8.6 · 8.7 · 8.8 · 8.10 · 8.12 · 8.16 | 36 |
| not rows | 4.32–4.34 are re-prints of 4.1 / 4.5 / 4.8 and carry no verdict | 3 |

**54 of the driver's 419 rows are demoted**, and 3 more are re-prints. **53 of the 54 were re-driven by
the reconciler** (blocks RC-M, RC-K, RC-F1, RC-F2, RC-F3 below) and **every one matched** the surface
the driver's row asserts. **Row 5.7 stays NOT DRIVEN** (no `planning-record` was filed here). So:
362 rows stand on a driver block, 53 on a reconciler block, 1 is not driven, 3 are not rows. No door
loses its coverage. `(R4, F4)` had its evidence inline in §4 and no §3 block; it is re-driven at RC-F4.

## L2 · The door-set count, against the registries

Re-read at `bffa6667` (the driver's tree; `git diff --stat jigc-v1.0.0-rc.24 HEAD -- crates/cli/packs
crates/engine crates/cli/src` is empty).

| registry | driver | reconciler | |
|---|---|---|---|
| `VERB_KINDS` | 48 | **48** | same |
| embedded packs (`include_dir!` in `pack_builtin.rs`) | 2 | **2** | same |
| dev / methodology manifest entries | 6 / 13 | **6 / 13** | same |
| shipped schemas | 6 + 13 | **6 + 13** | same |
| prior-shape snapshots | 2 + 4 | **2 + 4**, none for `jigc-feedback` or `inconsistency` | same |
| `SchemaChangeKind::ALL` × `LOCI` | 18 × 3 | **18 × 3** | same |
| `RelocateRefusal::ALL` | 10 | **10** | same |
| `AllowsCreate` keys | 3 | **3**, `deny_unknown_fields` | same |
| workflows · `allows-create` entries | 39 · 34 over 29, 2 `new: true` | **39 · 34 over 29, 2 `new: true`** | same |

**No count differs.** The driver's "20 of 48" doors is 20 distinct `VERB_KINDS` spellings, each the door
of at least one row that carries a block; the reconciliation adds two (`task discard`, `config set`).

## L3 · Driver defects — re-driven once each

| key | reproduces | tier | door | the tier-1 test |
|---|---|---|---|---|
| `(R4, F1)` — `migrate` leaves a minted task behind an exit-1 refusal | **yes** (RC-F1, both pack sources) | **3** | `migrate` | not tier 1: **exit 1**, and nothing lost — the `task.serial-collision` route recovers, `task discard` clears it |
| `(R4, F2)` — the malformed-front-matter refusal names no definition; line number one short | **yes** (RC-F2) | **2** | `start` (also `workflow`, `describe`, `migrate`) | not tier 1: exit 1 at every compose door; `validate` exits 0 report-only and loses nothing |
| `(R4, F3)` — `relocate.frozen-doctype`'s route is a no-op on a doctype with no snapshot | **yes** (RC-F3) | **2** | `relocate` | not tier 1: the routed `migrate-corpus` **does** exit 0, but the loss half is absent — the stranded file's sha1 and `HEAD` are identical before and after, nothing moved |
| `(R4, F4)` — `relocate --help` exemplifies an argv the door refuses | **yes** (RC-F4) | **3** | `relocate` | not tier 1: a help surface |
| `(R4, F5)` — `migrate-corpus` commits an uncommitted edit on a doc it migrates | **yes** (RC-F5, staged **and** unstaged plants, and at the moving arm) | **3** | `migrate-corpus` | **closest to tier 1, and still not it:** the exit-0 half and the committing door are both there; the loss-or-harm half is not. Both plants are in `HEAD` **and** on disk afterwards, nothing outside the migrated path is committed, and it is what `git commit -- <path>` does. Driven further in the tier-1 direction: a destination already occupied is **refused** (exit 1, `migrate-corpus.destination-collision`, the occupant's sha1 unchanged) |

Why each tier, in a clause: **F1** a `blocking` refusal reads as *nothing happened* while a task exists —
a surface, no dead end. **F2** the route says *"the definition the message names"* and the message names
none, at every compose door, for a workflow the caller may not have asked for — a route that cannot be
followed from where it is printed. **F3** a route that, followed exactly, changes nothing. **F4** help
text against the binary. **F5** the ack and `--help` (*"re-parses every **committed** doc"*) against the
bytes committed.

The Codex pass contradicted **none** of the five; it did not address any of them.

## L4 · Codex claims — each entered as a lead, then driven

`S-<n>` numbers the source pass's statements in the order it makes them; `CX-8`, `CX-9` and `CX-13` inside a row are the baseline's own lead names.

| # | lead(codex, …) | status | datum |
|---|---|---|---|
| S-1 | *a filesystem pack can lose its freeze manifest between owner enumeration and the second read; `assert_schema_freeze` silently skips that pack, so a door proceeds without validating its frozen schemas* | **CONFIRMED** — `(R4, C1)`, repro RC-C1 | with a drifted, un-pinned `adr` in a filesystem pack: control → exit 1 at `validate`; manifest removed after the traced enumeration read → **exit 0**, the second read never opens the file. Won **4 of 90** on unpadded packs |
| S-1t | *"potentially tier 1 when reached through `migrate-corpus`"* | **REFUTED as tier 1**; the finding is **tier 3** | `migrate-corpus` under the race → exit 0 and a commit, whose whole diff is `-schema-version: 2 / +schema-version: 3`; after a re-pin `validate` is clean. Exit-0 half shown, **loss-or-harm half absent** |
| S-2 | *`(7, A7-F3)` CLOSED — recorded prior homes route to `migrate-corpus`; root-knob strands route through `validate` / move / re-point / `unmanage`* | **CONFIRMED** (RC-A7, RC-A7k) | both arms and the natural knob strand print the route Codex read; each route runs |
| S-3 | *the methodology manifest holds 13 entries, both new doctypes at v1; strict set equality and hashes are checked at load; neither new doctype has a snapshot* | **CONFIRMED** on a listed filesystem copy (RC-C3) | reshape without re-pin → 16 of 17 argvs exit 1; entry deleted → *"shipped but absent from the freeze manifest"*; schema file removed → *"declared in the freeze manifest but no schema ships it"*. The **embedded** arm: OPEN LEAD (needs a rebuild) |
| S-4 | *`SchemaChangeKind × LOCI` is 18 × 3 with no `Unbuilt` cell; `AddedItemSlot` covers the optional item-slot shape of `inconsistency.sides[]`* | **CONFIRMED** for the registry (read) and for `AddedItemSlot` @2 (RC-K) | a second optional item slot under `sides`, v1 → v2: `migrate-corpus` → exit 0, `1 migrated`, additions only |
| S-5 | *CX-8's third origin pack is closed: every manifest owner is enumerated, the freeze assumes no pack count* | **CONFIRMED** (RC-C5) | a third manifest-owning pack, reshaped without a re-pin, blocks 8 of 8 pack-loading argvs naming its own doctype |
| S-6 | *CX-9: source completeness closed; behavioural cells not closed by the source pass* | **agrees with the driver — OPEN LEAD** | six cells driven in this row in total (L5) |
| S-7 | *CX-13, this row's intersection: pack construction runs the freeze before the pack reaches any caller* | **CONFIRMED** (RC-C7) | 18 of 19 argvs exit 1 with 0 bytes of stdout under a shape shadow; the nineteenth, `task list`, loads no pack |
| S-8 | *`cwd-unreadable` — NOT CLOSED, fixture-dependent* | **REFUTED as a status — the cell is driven** (RC-M) | seven argvs → exit 1, `cannot determine the current directory: No such file or directory (os error 2)`; the fixture is `mkdir` · `cd` · `rmdir` |
| S-9 | *`pack-resource-missing` — promoted to claim 1* | **split:** the lead itself is driven and loud (driver R-1c, RC-F1, RC-M); the manifest is the one resource whose absence is silent — that half is S-1 | `config/knobs.yaml` removed → 10 doors exit 1 `pack.resource-missing` |
| S-10 | *M54's seam holds: only `pack_builtin.rs` embeds the two packs; the production embed sites are that module, the adapter directory and the two setup guides* | **CONFIRMED** behaviourally (RC-0); the site enumeration is a registry read that agrees | on a rig under `<tmp>` with `JIGC_PACK_DIR` unset and `HOME` repointed, `start --explain` prints `<embedded>` for both packs and every door answers |
| S-11a | *`AllowsCreate` closes its keys to `type`, `as`, `new`; a malformed entry fails the workflow load* | **CONFIRMED** (RC-F2) | misspelt, stray, non-boolean → refused naming the key; `new: true` loads |
| S-11b | *"…and the eager pack-load sweep reaches every pack-loading door"* | **CONFIRMED for a pack-layer definition, REFUTED for a project-layer shadow** (RC-F2) | pack layer: 10 of 10 pack-loading argvs exit 1, naming `single-task`. Project shadow, the same defect: `start --explain`, `doc list`, `doc schema`, `migrate-corpus --dry-run`, `ingest` → **exit 0**; `validate` → exit 0, report-only |
| S-12 | *the two `planning-record` hint rewords leave the hash unmoved; M54 moved no hash; M55 added only the two v1 rows* | **CONFIRMED** as far as a door shows it (RC-C7) | the embedded packs pass their own freeze at every door; `doc schema planning-record` → schema-version 1; a reworded slot `hint:` and item-slot `hint:` shadow load clean while a one-member enum edit blocks. *"…changed no pinned JSON key"*: not this row's doors — OPEN LEAD |

## L5 · Baseline rows — CLOSED / STILL-OPEN

| key | status | argv + observed (reconciler's own drive) |
|---|---|---|
| **`(7, A7-F3)`** | **CLOSED — both arms** | **placement:** `jigc doc show changelog` → 1 `store.not-found`, route *"`changelog:changelog` is committed at `CHANGELOG.md`, a prior home of `changelog` — … run `jigc migrate-corpus` to land it at the home this read resolves, then read it again"*. **location:** `jigc doc show adr:use-sqlite` → the same, naming `docs/decisions/use-sqlite.md`. The route run → `2 migrated`; both reads then exit 0; each file's sha1 with the stamp put back equals its sha1 before. Also on `inconsistency` (RC-K) and on a `docs-root` strand (RC-A7k). Bound as the driver's O-9: a doc moved by hand, with no snapshot and no knob change behind it, still gets the generic route (RC-F3) |
| §D — the methodology pack's own manifest | **NOW DRIVEN**, the embedded arm excepted | RC-C3 and RC-C7 |
| §D — 40 of the 54 `SchemaChangeKind × LOCI` cells | **STILL-OPEN** | driven in this row: `Relocated` (doctype-level, location and placement), `EnumWidened` @1, `RemovedField` @1, `ProseNeeding` @2, and — added by the reconciler — `AddedItemSlot` @2. No complement claimed |
| §D — `ManifestKind::ALL` | **NOT THIS ROW'S** | row 5 |
| §D — `cwd-unreadable` | **NOW DRIVEN** | RC-M |
| §D — `pack-resource-missing` | **NOW DRIVEN**; two findings sit on it | `(R4, F1)` at `migrate`, `(R4, C1)` on the manifest |
| §D — CX-8's third origin pack | **NOW DRIVEN — a third manifest-owning pack** | RC-C5; the driver had left the pack *beyond* dev + methodology open |
| §D — CX-9 as behaviour | **STILL-OPEN** | as the 54-cell lead |
| §D — CX-13 | **this row's intersection CONFIRMED; the rest NOT THIS ROW'S** | RC-C7; `RelocateRefusal::ALL` stays 1 of 10 driven |
| M51 D-1 · D-2 · D-4 half 1 | **passed through — still CLOSED** | `ingest` prints `advisory · file-state.absorbed` (RC-M); the move lands with every byte (RC-A7, RC-K); *"no resolved doctype claims this path"* (RC-F3, RC-M) |
| M51 C-1 · D-3 | **NOT RE-DRIVEN** | no cell of this row passes through them |
| `(7, A7-F1)` · `(7, A7-F2)` · `(7, C-2)` · `probe-unreliable` | **row 3's** | `(7, A7-F2)`'s shape re-observed on both new doctypes (RC-M, the orphan cell) and not re-filed |

**No baseline row regressed.**

## L6 · Confirmed findings

| key | origin | tier | door | one line |
|---|---|---|---|---|
| `(R4, F2)` | driver | 2 | `start` | the malformed-front-matter refusal names no workflow at the compose doors, for any workflow asked; its line number is one short; only `validate` (exit 0) names `workflow:<id>` |
| `(R4, F3)` | driver | 2 | `relocate` | `relocate.frozen-doctype` routes at `jigc migrate-corpus`, which for a doctype with no versioned snapshot — both M55 doctypes — exits 0 and moves nothing |
| `(R4, F1)` | driver | 3 | `migrate` | `migrate` mints its task before the compose-time fault and exits 1 without saying so; `start` under the same fault mints nothing |
| `(R4, F4)` | driver | 3 | `relocate` | `relocate --help` gives `vision` as its example; all 18 stock doctypes are refused `relocate.frozen-doctype` |
| `(R4, F5)` | driver | 3 | `migrate-corpus` | the migration commit carries a human's staged and unstaged edits on a migrated path; neither the ack nor `--dry-run` says so |
| `(R4, C1)` | codex | 3 | `migrate-corpus` (also every pack-loading door) | a filesystem pack's manifest that vanishes between the enumeration read and the second read is skipped without a word; the drifted schema then loads, and `migrate-corpus` commits |

**`(R4, C1)`, why tier 3 and what bounds it.** The factory's contract is that a drifted frozen schema
*blocks every door*; under this fault it does not, and no surface says the freeze was skipped. It is not
a route or posture dead end. It is not tier 1: the commit it lets through is the stamp alone. Three
bounds, each driven: the embedded packs are immune (no file to lose); in a one-filesystem-pack
composition the two reads are adjacent; in a two-filesystem-pack composition the window is the other
pack's own check, about 4 ms of a run, and it was won 4 times in 90. And the same exit-0 state is
reachable **by design** by deleting the manifest (*an absent manifest is the wholesale opt-out*) — at
`validate` that state at least prints `schema-conformance.unversioned-doctype`; when the file comes
back before the door's own read, as in the `migrate-corpus` drive, nothing does. The source comment at
the skip calls it deliberate (*"a racy filesystem pack that lost the file between the two reads simply
skips"*).

## L7 · Open leads

1. **`(R4, C1)` at a committing door on unpadded packs** — the `migrate-corpus` commit was driven with
   the window widened; unpadded, only `validate` was raced. The commit additionally needs the file back
   before the door's own manifest read.
2. **The embedded-pack arms** — an absent manifest entry and a reshaped schema *inside the binary*. Needs
   a rebuild; neither pass ran it.
3. **The rest of the 54 `SchemaChangeKind × LOCI` cells** (CX-9 as behaviour).
4. **The cross-version half** — no older binary is installed.
5. **`RelocateRefusal::ALL`, 9 of 10, and `relocate`'s success arm** — need a freeze-exempt doctype.
6. **Row 5.7** — not driven by either party.
7. **Codex: *"rc.24 … changed no pinned JSON key"*** — not this row's doors.
8. **Three reconciler observations, each driven once and adjudicated against no contract** — leads, not
   findings: (a) after `jigc config set docs-root` moved the docs and their baselines and plain git then
   moved the files back, `doc show`'s route says *"`jigc validate` names the repair for this store"*
   while `validate` calls the same files *"never adopted — a basename coincidence … not a tracked
   strand"* and `doc list` prints *no committed docs* (RC-A7k, the hybrid arm); (b) `migrate-corpus`'s
   `unadopted` row calls an **untracked** file *"committed file"* and its `jigc migrate <path>` route
   prints an absolute host path (RC-F5); (c) `relocate … --format json` answers a coded refusal as the
   flattened `{"error": …}` on stderr, where `doc show` answers one as the findings envelope (RC-M).

## L8 · Reconciler repro blocks

Every block starts from

```
rig=$(dev/jigc-rig <state> [flags] --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
```

### RC-0 · fixtures, all through the binary

```
# "both docs filed" (rig `fresh`):
$ jigc start --workflow report-jigc-feedback "describe names no pack version"     -> 0  task minted
$ jigc doc create jigc-feedback --title "Describe names no pack version" --task <t>
$ jigc doc set-field jigc-feedback:<slug>#meta/kind --value inconvenience --task <t>      (+ found-in, jigc-version)
$ jigc doc set-slot jigc-feedback:<slug>#description --from-file - --task <t>
$ jigc doc set-field commit:<t>#type --value docs ; jigc doc set-slot commit:<t>#summary --from-file -
$ jigc task finalize <t>                                                            -> 0
$ jigc start --workflow report-inconsistency "readme and claude disagree"           -> 0
$ jigc doc create inconsistency --title … ; doc set-field …#meta/kind --value doc-doc
$ jigc doc add-item inconsistency:<slug>#sides --title "README.md" --slug readme     (+ "CLAUDE.md" --slug claude)
$ jigc doc set-slot …#description ; commit type + summary ; jigc task finalize <t2>  -> 0
$ jigc validate   -> 0  "no findings — the committed store validates clean"
$ jigc doc list   -> 0  two rows, `managed` ; both files carry `schema-version: 1`
$ jigc start --explain | grep "Pack input"   -> dev/1.0.0-rc.24 = <embedded> · methodology/1.0.0-rc.24 = <embedded>
# an adr: start --workflow record-decision → doc create adr → 3 × set-slot → set-field …#status → finalize
# a changelog: start --workflow record-change → doc create changelog → add-item …#unreleased-changes
#              → set-slot …/added/notes → finalize
```

### RC-F1 · `(R4, F1)`

```
rig: fresh --repin ; one committed foreign file foreign-adr.md
$ jigc task list                             -> 0  "no active tasks"          # BEFORE-CONTROL
  ls .jigc/tasks                             -> No such file or directory
mv "$JIGC_PACK_DIR/config/knobs.yaml" "$RIG/knobs.yaml.away"
$ jigc migrate foreign-adr.md --as adr       -> 1  blocking · pack.resource-missing — no composed pack ships `config/knobs` …
$ jigc start --workflow quick-fix "a probe intent"   -> 1  (the same)
  ls .jigc/tasks                             -> migrate-adr-foreign-adr-5304b572cd4f   (base.json intent staged-snapshot.json workflow)
$ jigc task list                             -> 0  "1 active task(s)"
  git status --short                         -> (empty)
mv "$RIG/knobs.yaml.away" "$JIGC_PACK_DIR/config/knobs.yaml"
$ jigc migrate foreign-adr.md --as adr       -> 1  blocking · task.serial-collision — task `migrate-adr-…` is already active
$ jigc start --task migrate-adr-foreign-adr-5304b572cd4f      -> 0
$ jigc task discard migrate-adr-foreign-adr-5304b572cd4f --force   -> 0  "discarded task …"

rig: fresh, both docs filed ; the stray-key single-task shadow of RC-F2 committed ; idea-note.md committed
$ jigc task list                             -> 0  "no active tasks"          # BEFORE-CONTROL
$ jigc start --workflow quick-fix probe      -> 1  workflow-refs.malformed-front-matter ; ls .jigc/tasks -> (none)
$ jigc start "another probe"                 -> 1  (the same)                 ; ls .jigc/tasks -> (none)
$ jigc migrate idea-note.md --as idea        -> 1  workflow-refs.malformed-front-matter
  ls .jigc/tasks                             -> migrate-idea-idea-note-1ec01db32a67
$ jigc migrate idea-note.md --as idea        -> 1  task.serial-collision
```

### RC-F2 · `(R4, F2)`, and S-11

```
rig: fresh, both docs filed
# .jigc/config/workflows/report-jigc-feedback.yaml = the shipped file, ONE edit on file line 10, committed:
#   - { type: jigc-feedback, as: feedback, new: true }   ->   … nwe: true }
$ jigc start --workflow report-jigc-feedback probe-one | workflow report-jigc-feedback --preview | describe
       | workflow quick-fix --preview | start --workflow quick-fix probe-three | start probe-two     -> 1 each
  blocking · workflow-refs.malformed-front-matter — workflow front-matter is malformed config-family YAML:
  allows-create[0]: unknown field `nwe`, expected one of `type`, `as`, `new` at line 9 column 42
    route: fix the workflow/step/catalog definition the message names (a definition defect, repaired once
    at its source), then re-run                                    # no id, no file, no `at:` line
$ jigc validate   -> 0   the same two lines + "at: workflow:report-jigc-feedback" + the report-only trailer
$ jigc validate --format json   -> 0   blocking_probes ["workflow-refs"], report_only true,
                                       key {code: workflow-refs.malformed-front-matter, target: workflow:report-jigc-feedback}
$ jigc start --explain | doc list | doc schema jigc-feedback | migrate-corpus --dry-run | ingest   -> 0 each
$ jigc task list  -> "no active tasks"

# stray key — .jigc/config/workflows/single-task.yaml, file line 6: {type: adr, as: decision} -> { …, note: stray }
$ jigc start … | workflow report-jigc-feedback --preview   -> 1  … unknown field `note` … at line 5 column 44
$ jigc validate                -> 0  … at: workflow:single-task
$ jigc describe --format json  -> 1  stdout 0 b · stderr {"error": "blocking · workflow-refs.malformed-front-matter — …"}
# `new: true` on that entry    -> workflow single-task --preview 0 · describe 0 · validate 0
# `new: maybe`                 -> 1  allows-create[0].new: invalid type: string "maybe", expected a boolean

# the SAME stray key in a pack-layer file (rig fresh --repin, "$JIGC_PACK_DIR/workflows/single-task.yaml"):
$ jigc describe | doc list | doc schema adr | validate | migrate-corpus --dry-run | ingest | start --explain
       | workflow quick-fix --preview | config get docs-root | unmanage README.md       -> 1 each, stdout 0 b
  pack-load workflow-front-matter sweep failed on `single-task`: … unknown field `note` … at line 5 column 44
$ jigc task list   -> 0
```

### RC-F3 · `(R4, F3)`

```
rig: fresh, both docs filed
git mv docs/jigc-feedback/<slug>.md docs/feedback/<slug>.md ; git commit -m "move the feedback doc out of band"
$ jigc ingest     -> 0  needs-reconcile … blocking · ingest.wrong-location …
                        route: move docs/feedback/<slug>.md into docs/jigc-feedback/, then re-run `jigc ingest`
$ jigc relocate jigc-feedback --from docs/feedback   -> 1
  blocking · relocate.frozen-doctype — `jigc-feedback` is a frozen doctype — relocate it through the
  version-gated `jigc migrate-corpus`, not the freeze-exempt path
    route: `jigc migrate-corpus` walks every prior home the doctype's versioned snapshots declare and lands
    the move under the freeze
shasum docs/feedback/<slug>.md -> eb8bdca2… ; git rev-parse --short HEAD -> 19bbcde          # BEFORE
$ jigc migrate-corpus   -> 0
  corpus migration: 0 migrated, 1 already current, 0 blocked
    current    docs/inconsistencies/<slug>.md
shasum docs/feedback/<slug>.md -> eb8bdca2… ; HEAD -> 19bbcde ; git status --short -> (empty) # AFTER: nothing moved
$ jigc doc show jigc-feedback:<slug>   -> 1  store.not-found, the generic route
$ jigc validate   -> 1  reconciliation.rename + schema-conformance.orphaned-instance (… "no resolved doctype claims this path" …)
$ jigc doc list   -> 0  "(none)  docs/feedback/<slug>.md  orphaned"
```

### RC-F4 · `(R4, F4)`

```
rig: fresh
$ jigc relocate --help
  <TYPE>  The freeze-exempt doctype id whose schema home moved (e.g. `vision`)
  --from <FROM>  … a directory (`docs/vision/`) or a literal file (`docs/vision.md`) …
$ jigc relocate vision --from docs/vision-old   -> 1  blocking · relocate.frozen-doctype — `vision` is a frozen doctype …
# every doctype `jigc describe --format json` lists (18):  jigc relocate <id> --from docs/zzz   -> 1 relocate.frozen-doctype ×18
```

### RC-F5 · `(R4, F5)`, and the tier-1 direction

```
rig: fresh, both docs filed ; the stamp line stripped from the committed jigc-feedback doc, committed with git
echo PLANT-STAGED > staged.txt ; git add staged.txt ; echo PLANT-DIRTY-README >> README.md
# on the doc: one edit staged (PLANT-STAGED-EDIT-A), a second left unstaged (PLANT-UNCOMMITTED-EDIT-B)
git status --short   -> " M README.md" · "MM docs/jigc-feedback/<slug>.md" · "A  staged.txt"     # BEFORE-CONTROL
git show HEAD:<doc> | grep -c PLANT -> 0 ; grep -c PLANT <doc> -> 2
$ jigc migrate-corpus --dry-run   -> 0  "1 would migrate, 1 already current, 0 blocked"            # says nothing of the edits
$ jigc migrate-corpus             -> 0  "1 migrated … committed a505774 — only the migrated paths were staged"
git status --short   -> " M README.md" · "A  staged.txt"
git show HEAD -- <doc>   -> +schema-version: 1 · +…PLANT-STAGED-EDIT-A · +PLANT-UNCOMMITTED-EDIT-B
grep -c PLANT <doc>  -> 2                                                                         # AFTER: on disk too

# the moving arm (rig fresh --repin, adr + changelog filed, both homes moved by a version bump, re-pinned):
echo PLANT-UNTRACKED-AT-DESTINATION > NEWS.md ; echo PLANT-… > docs/records/use-sqlite.md ; echo PLANT-UNCOMMITTED-EDIT >> docs/adrs/use-sqlite.md
shasum NEWS.md docs/records/use-sqlite.md -> c6260d86… · f18f0c2c…                                # BEFORE
$ jigc migrate-corpus [--dry-run]   -> 1
  blocked HISTORY.md — migrate-corpus.destination-collision: `HISTORY.md` relocates to `NEWS.md`, which already
  holds a *different* document; the migration never overwrites it (no data loss) …        (and the adr likewise)
shasum NEWS.md docs/records/use-sqlite.md -> c6260d86… · f18f0c2c… ; HEAD unchanged               # AFTER
# occupants moved aside:
$ jigc migrate-corpus   -> 0  "2 migrated NEWS.md · docs/records/use-sqlite.md / committed 20e3bb8"
git show HEAD:docs/records/use-sqlite.md | grep -c PLANT -> 1 ; grep -c PLANT docs/records/use-sqlite.md -> 1
```

### RC-A7 · `(7, A7-F3)`, both arms — and RC-A7k, the knob strand

```
rig: fresh --repin ; adr + changelog filed ; P=$JIGC_PACK_DIR
cp $P/schemas/{changelog,adr}.yaml -> $P/schema-snapshots/{changelog,adr}.v2.yaml
changelog.yaml: placement: { file: CHANGELOG.md } -> { file: HISTORY.md } ; adr.yaml: location: decisions/ -> adrs/
manifest: adr 2 -> 3, changelog 2 -> 3 ; re-pinned from the binary's own "recomputed `598eaf3d…`" / "`d9ee5fad…`" lines
$ jigc validate   -> 1  schema-conformance.schema-version-current ×2
$ jigc doc list   -> 0  adr:use-sqlite docs/decisions/use-sqlite.md managed · changelog:changelog CHANGELOG.md managed
$ jigc doc show changelog   -> 1
  blocking · store.not-found — could not read `changelog:changelog` at `HISTORY.md`: No such file or directory (os error 2)
    route: `changelog:changelog` is committed at `CHANGELOG.md`, a prior home of `changelog` — the schema's home
    moved and this corpus has not been migrated; run `jigc migrate-corpus` to land it at the home this read
    resolves, then read it again
$ jigc doc show adr:use-sqlite   -> 1  the same route, naming `docs/decisions/use-sqlite.md`
$ jigc doc show changelog:changelog --format json   -> 1  schema_version 3, findings[0].key {code: store.not-found, target: changelog:changelog}
shasum CHANGELOG.md docs/decisions/use-sqlite.md   -> 3b859db9… · cf0b9925…                       # BEFORE
$ jigc migrate-corpus   -> 0  "2 migrated HISTORY.md · docs/adrs/use-sqlite.md / committed c56ba4a"
sed 's/^schema-version: 3$/schema-version: 2/' HISTORY.md | shasum               -> 3b859db9…     # AFTER
sed 's/^schema-version: 3$/schema-version: 2/' docs/adrs/use-sqlite.md | shasum  -> cf0b9925…
$ jigc doc show changelog | doc show adr:use-sqlite   -> 0 each ; jigc validate -> 0 clean

# RC-A7k — the natural knob strand (rig fresh):
$ jigc config set docs-root documentation/   -> 0 ; git commit ; both docs filed (they home under documentation/)
git revert --no-edit <the config commit>     ; jigc config get docs-root -> "docs/  (pack-default)"
$ jigc doc show jigc-feedback:<slug>   -> 1
  blocking · store.not-found — could not read … at `docs/jigc-feedback/<slug>.md` …
    route: `jigc-feedback:<slug>` is committed at `documentation/jigc-feedback/<slug>.md`, outside the home this
    read resolves — a `docs-root` / `placement-root` re-point stranded it; `jigc validate` names the repair for
    this store (move it to the resolved home, re-point the knob to cover where it sits, or drop it with
    `jigc unmanage documentation/jigc-feedback/<slug>.md`)
$ jigc validate   -> 0  advisory · file-state.orphaned-doc ×2 — "a `docs-root` change likely stranded it" + the move / re-point / unmanage route

# the hybrid arm (lead 8a) — rig fresh, both docs filed:
$ jigc config set docs-root documentation/   -> 0  "relocating 2 committed doc(s) … each move is a staged `git mv`"
git mv both files back under docs/ ; git commit (the knob change alone)
$ jigc doc show jigc-feedback:<slug>   -> 1  the same knob-strand route ("`jigc validate` names the repair for this store")
$ jigc validate   -> 0  advisory · reconciliation.rename ×2 ("the baseline outlived the doc … drop it with `jigc unmanage documentation/…`")
                        advisory · file-state.unregistered-doc ×2 ("never adopted — a basename coincidence … not a tracked strand")
$ jigc doc list   -> 0  "no committed docs"
```

### RC-C1 · `(R4, C1)` — the manifest read race

```
rig: fresh --repin                                    # P2 = $JIGC_PACK_DIR, a dev pack copy WITH its manifest
cp -R crates/cli/packs/methodology "$RIG/methodology"  # P1
printf 'packs:\n  - %s\n' "$RIG/methodology" > .jigc/config/packs.yaml ; git commit   # see L0.1
an adr filed through the binary
P2/schemas/adr.yaml:  of: [proposed, accepted, superseded] -> […, rejected]          # drift; P2's manifest NOT re-pinned
$ jigc validate   -> 1  pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares
                        `2d860240…`, recomputed `028db814…`)                         # CONTROL, manifest present
mv P2/config/schema-manifest.yaml aside ; jigc validate -> 0 ; mv it back            # CONTROL, absent throughout:
                                                                                     #   the designed opt-out
```

The tracer (L0.2), compiled with `cc -dynamiclib`:

```c
static int my_open(const char *path, int flags, ...) {
    mode_t mode = 0;
    if (flags & O_CREAT) { va_list ap; va_start(ap, flags); mode = (mode_t)va_arg(ap, int); va_end(ap); }
    int fd = open(path, flags, mode); int saved = errno;
    const char *m = getenv("RC_TRACE_MATCH"), *log = getenv("RC_TRACE_LOG");
    if (m && log && path && strstr(path, m)) { /* append "<time> pid open <path> -> ok|<error>" to log */ }
    errno = saved; return fd;
}
__attribute__((used)) static struct { const void *replacement, *replacee; } interpose_open
    __attribute__((section("__DATA,__interpose"))) = { (const void *)my_open, (const void *)open };
```

**Drive 1 — `validate`, window widened (L0.3).** Traced control: the door reads P2's manifest at
t=0.048 s (enumeration) and t=1.077 s (the check) in an instrumentation pre-load whose failure is
swallowed, then at t=1.080 s (enumeration) and t=2.126 s (the check) in the verb's own load, and exits 1.

```
( sleep 1.5 ; mv P2/config/schema-manifest.yaml aside ) &  jigc validate      # traced
  t=1.046  open $RIG/pack/config/schema-manifest.yaml -> ok        # the verb's enumeration read: P2 IS an owner
  t=1.542  manifest moved away
  (no further open of that path: the second read finds no file and the loop `continue`s)
$ jigc validate   -> 0                                              # reached validation
  blocking (gates at finalize) · conformance.unknown-field — `decisions/use-sqlite.md`: unknown field key `schema-version`
  advisory · schema-conformance.unversioned-doctype — … doctype `adr` is defined by a pack that declares no
  `schema-manifest.yaml` entry for it …
  2 finding(s) — report-only at store scope (exit 0) …
```

**Drive 2 — `migrate-corpus`, window widened.** P2's manifest additionally bumped `adr` 2 → 3 with the
stale hash kept, and `adr.v2.yaml` shipped as the snapshot.

```
$ jigc migrate-corpus | migrate-corpus --dry-run   -> 1 each  pack-load freeze check failed: doctype `adr` …   # CONTROL ; HEAD d5f5cea
( sleep 1.5 ; mv manifest aside ; sleep 1.1 ; mv it back ) &  jigc migrate-corpus        # traced
  t=1.024  open …/pack/config/schema-manifest.yaml -> ok          # the verb's enumeration read
  t=1.535  manifest moved away
  t=2.673  manifest restored
  t=3.061  open …/pack/config/schema-manifest.yaml -> ok          # the next read of it — after the freeze loop
$ jigc migrate-corpus   -> 0
  corpus migration: 1 migrated, 0 already current, 0 blocked
    migrated   decisions/use-sqlite.md
  committed 6d59108 — only the migrated paths were staged
git show HEAD   -> -schema-version: 2 / +schema-version: 3  — nothing else ; trailer Co-Authored-By: Claude <noreply@anthropic.com>
$ jigc validate   -> 1  pack-load freeze check failed …             # the file is back: every door blocks again
# re-pin the hash from the "recomputed" line:  jigc validate -> 0 clean ; migrate-corpus --dry-run -> "1 already current"
```

**Drive 3 — `validate`, unpadded packs, 90 runs.** The manifest moved aside once per run at a delay
stepped through 4–53 ms and restored after the process exits; each run traced. In the control the
verb's two reads of P2's manifest sit 4.0–4.8 ms apart (three measurements), the time P1's own check
takes.

```
80 runs  exit 1, 14 opens     the file outlived both of the verb's reads
 3 runs  exit 1, 9–12 opens   removed after the second read
 4 runs  exit 0,  8 opens     THE RACE: the 8th open is the verb's enumeration read; the second read found no file
 2 runs  exit 0,  7 opens     removed before the verb enumerated — the designed opt-out, not the race
 1 run   exit 0,  1 open      removed inside the pre-load's window
```

### RC-C3 · the methodology pack's own manifest (the listed copy of RC-C1, both docs filed)

```
P1/schemas/jigc-feedback.yaml:  of: [bug, inconvenience, feedback] -> […, question]     # P1's manifest NOT re-pinned
$ jigc describe | doc schema jigc-feedback | doc schema inconsistency | doc schema adr | validate | doc list
       | migrate-corpus --dry-run | migrate-corpus | ingest | unmanage jigc-feedback/<slug>.md
       | relocate jigc-feedback --from feedback | start --explain | start --workflow quick-fix probe-intent
       | workflow quick-fix --preview | doc show jigc-feedback:<slug> | config get docs-root        -> 1 each, stdout 0 b
  pack-load freeze check failed: doctype `jigc-feedback`: schema-hash mismatch (manifest declares
  `7e16fb98…`, recomputed `3ff572f1…`)                                              # no route line
$ jigc task list   -> 0 ; git status --short -> (empty) ; no task minted
# schema restored ; the three lines of `- type: inconsistency` deleted from P1's manifest:
$ jigc validate | doc list | doc schema inconsistency | migrate-corpus --dry-run | describe   -> 1 each
  pack-load freeze check failed: doctype `inconsistency` is shipped but absent from the freeze manifest
# manifest restored ; P1/schemas/dogfood-record.yaml moved aside, its entry kept:
$ jigc validate | doc list | describe   -> 1 each
  pack-load freeze check failed: doctype `dogfood-record` is declared in the freeze manifest but no schema ships it
```

### RC-C5 · a third manifest-owning pack

```
$RIG/third/  config/defaults.yaml (pack-id: third) · config/schema-manifest.yaml (the slug-rule block + one entry, `memo`,
             schema-version 1, a zero hash) · schemas/memo.yaml (one header section, one slot)
.jigc/config/packs.yaml:  packs: [ $RIG/third, $RIG/methodology ]                    # see L0.1
$ jigc start --explain | validate | doc list | doc schema memo | migrate-corpus --dry-run   -> 1 each
  pack-load freeze check failed: doctype `memo`: schema-hash mismatch (manifest declares `0000…`, recomputed `fcf91b6e…`)
# pinned from that line:
$ jigc start --explain   -> 0  Pack input: third/0.0.1 · methodology/0.1.0 · dev/fs-local
$ jigc validate | doc schema memo | doc schema jigc-feedback | doc schema adr   -> 0 each ("doctype: memo (schema-version 1)")
# memo.yaml gains one optional field, not re-pinned:
$ jigc describe | validate | doc list | doc schema adr | doc schema jigc-feedback | migrate-corpus | ingest
       | start --workflow quick-fix probe-intent        -> 1 each, stdout 0 b   … doctype `memo`: schema-hash mismatch …
$ jigc task list   -> 0 ; git status --short -> (empty)
```

### RC-C7 · the freeze at every pack-loading door; prose shadows; the hints

```
rig: fresh, both docs filed ; notes/fb.md committed
DOORS = the driver's 19 argvs (R-3)
# control, the 16 non-mutating members: 0 ×14, relocate ×2 -> 1 (relocate.frozen-doctype)
# .jigc/config/schemas/jigc-feedback.yaml = the shipped schema, ONE edit, committed:  of: […, feedback] -> […, feedback, question]
$ jigc <18 of DOORS>   -> 1 each, stdout 0 b
  pack-load freeze check failed: doctype `jigc-feedback`: schema-hash mismatch (manifest declares `7e16fb98…`,
  recomputed `3ff572f1…`) — the project schema shadow $REPO/.jigc/config/schemas/jigc-feedback.yaml changes the
  shape of a frozen doctype, which the freeze forbids at every layer (`design/corpus-migration.md` → The freeze)
    route: `rm $REPO/.jigc/config/schemas/jigc-feedback.yaml` restores the frozen shape — …
$ jigc task list                -> 0                       # loads no pack
$ jigc validate --format json   -> 1  stdout 0 b · stderr {"error": …}
git status --short -> (empty) ; ls .jigc/tasks -> (none)
rm "$REPO/.jigc/config/schemas/jigc-feedback.yaml"        # the route, verbatim
$ jigc validate -> 0 ; jigc doc schema jigc-feedback -> 0

# prose-only shadows of both, committed: description: ×2, usage: ×1, a slot hint: (jigc-feedback), the item-slot hint: (inconsistency)
$ jigc validate | doc schema jigc-feedback | doc schema inconsistency | start --explain | doc list | migrate-corpus --dry-run
       | ingest | describe | workflow quick-fix --preview | doc show jigc-feedback:<slug> | config get docs-root   -> 0 each
$ jigc describe | grep -o 'SHADOW[A-Z]*' | sort | uniq -c   -> 2 SHADOWDESC · 1 SHADOWUSAGE
$ jigc doc schema jigc-feedback --format json   -> schema-version 1, home docs/jigc-feedback/<slug>.md

$ jigc doc schema planning-record   -> 0  "doctype: planning-record (schema-version 1)" ; no hint text in it
$ jigc workflow planning --preview  -> 0  "…lists the option that removes the artifact that can drift…" ×2 ·
                                         "…An advocate's proposal and the orchestrator's halt recommendations are claims too…" ×2
```

### RC-K · the kind cells (the listed copies of RC-C1; each change = snapshot + bump + re-pin)

```
# AddedItemSlot @2, inconsistency v1 -> v2: a second optional item slot `source` under `sides`
$ jigc validate -> 1 (schema-version-current) ; shasum <doc> -> 1b78fa37…
$ jigc migrate-corpus --dry-run -> 0 "1 would migrate" ; jigc migrate-corpus -> 0 "1 migrated inconsistencies/<slug>.md / committed 14aed30"
git show HEAD   -> -schema-version: 1 / +schema-version: 2 and, per item, +#### Says / +#### Source — additions only
$ jigc validate -> 0 clean
# EnumWidened @1, adr v3 -> v4 (dev copy)
$ jigc validate -> 1 ; migrate-corpus --dry-run -> 0 "1 would migrate" ; migrate-corpus -> 0, diff = the stamp ; validate -> 0
# EnumWidened @1, jigc-feedback v1 -> v2
$ jigc validate -> 0 (before) ; doc schema jigc-feedback -> "(schema-version 2)" ; validate -> 1 ;
  migrate-corpus --dry-run -> 0 "1 would migrate" ; migrate-corpus -> 0 "1 migrated jigc-feedback/<slug>.md", diff = the stamp ; validate -> 0
# Relocated (location), inconsistency v2 -> v3:  location: inconsistencies/ -> disagreements/
$ jigc validate -> 1 ; doc list -> the row still at inconsistencies/<slug>.md, managed
$ jigc doc show inconsistency:<slug>   -> 1  route: … committed at `inconsistencies/<slug>.md`, a prior home of `inconsistency` … run `jigc migrate-corpus` …
$ jigc relocate inconsistency --from inconsistencies   -> 1  relocate.frozen-doctype
$ jigc migrate-corpus   -> 0  "1 migrated disagreements/<slug>.md" ; sha1 with the stamp put back == sha1 before
$ jigc doc show inconsistency:<slug> -> 0 ; jigc validate -> 0 clean
# ProseNeeding @2, inconsistency v3 -> v4:  the item slot `says` made required
$ jigc migrate-corpus [--dry-run] [--format json]   -> 1 each
  blocked disagreements/<slug>.md — migrate-corpus.prose-needed: … its bytes are rolled back untouched — … required
  slot `says` in item `sides/readme` is empty …        json: blocked[0].key {code, target}, commit null
$ jigc validate -> 1 ; sha1 unchanged ; HEAD unchanged
# RemovedField @1, adr v4 -> v5:  the `{ id: date, … }` line deleted
$ jigc migrate-corpus [--dry-run]   -> 1  blocked decisions/use-sqlite.md — migrate-corpus.removed-field: … the migration never strips a value (no data loss) …
$ jigc validate -> 1 ; sha1 unchanged ; HEAD unchanged
```

### RC-M · the driver's rows that carried no block (L1)

```
# 1.18 — rig fresh, both docs filed, JIGC_PACK_DIR set to a `fresh --repin` rig's pack copy:
$ jigc describe --format json   -> 0  schema_version 3 ; definitions: workflow/dev 18 · doctype/dev 6 ; 0 methodology
# 1.30 — rig fresh --repin, config/knobs.yaml moved aside, then back:
$ jigc validate -> 1 pack.resource-missing ; validate --format json -> 1, stdout 0 b, stderr {"error": …} ; (restored) validate -> 0 clean

# cell 2 members — rig fresh, both docs filed:
$ jigc doc schema {jigc-feedback,inconsistency} --format {agent,human}   -> 0 ×4  "doctype: <id> (schema-version 1)", the injected
                                                                          `schema-version: int` field, no home line
$ jigc doc list --format json   -> 0  docs[]: inconsistency… managed item-count 2 · jigc-feedback… managed item-count 0 ; fields.schema-version "1"
$ jigc migrate-corpus | migrate-corpus --dry-run | migrate-corpus --format json   -> 0 ×3  "0 migrated, 2 already current, 0 blocked" ;
                           json keys already_current blocked commit dry_run hook_output migrated unadopted unfilled ; commit null ; HEAD unchanged
$ jigc relocate inconsistency --from docs/inconsistency | relocate jigc-feedback --from docs/jigc-feedback   -> 1 ×2 relocate.frozen-doctype
$ jigc relocate inconsistency --from docs/feedback --format json   -> 1  stdout 0 b · stderr {"error": "blocking · relocate.frozen-doctype — …"}
$ jigc migrate notes/fb.md --as jigc-feedback | migrate notes/inc.md --as inconsistency [--format json]   -> 1 ×3
  doctype `<id>` exists but is not migratable (no `migrate-<id>` workflow); migratable doctypes: adr, arch-doc, …
$ jigc migrate notes/inc.md --as idea --dry-run   -> 2  error: unexpected argument '--dry-run' found ; jigc task list -> "no active tasks"
# a conformant copy and a front-matter-less note hand-placed at each home, committed with git:
$ jigc ingest     -> 0  adoptable …hand-placed-conformant.md ×2 (adopted) · needs-reconcile …loose-note.md ×2, route "… no `migrate-<id>` workflow ships …"
$ jigc validate   -> 1  schema-conformance.unadopted-instance ×2 ; jigc doc list -> loose-note ×2 `unregistered`, the rest `managed`
$ jigc unmanage docs/jigc-feedback/hand-placed-conformant.md   -> 0  "unmanaged … dropped its file-state baseline + forward edges; the file is left on disk …"
$ jigc unmanage docs/inconsistencies/loose-note.md             -> 0  "no-op: … is not managed (nothing to drop)"
$ jigc unmanage docs/inconsistencies/hand-placed-conformant.md --format json   -> 0  {path, identity, dropped: true}
$ jigc doc list   -> both hand-placed rows still `managed` ; jigc validate -> 1, + advisory · file-state.un-baselined ×2
$ jigc migrate-corpus --dry-run [--format json]   -> 0  "0 would migrate, 4 already current, 0 blocked, 2 not adopted" ;
                           json unadopted length 2, each {check, code, key {code, target}, location, message, probe, route, severity}
$ jigc doc show jigc-feedback:loose-note   -> 1  store.unparseable — … route: adopt — run `jigc ingest` …

# cell 6 members — rig fresh, both docs filed:
# unstamped jigc-feedback (stamp line deleted, committed with git)
$ jigc validate -> 1  blocking schema-conformance.schema-version-current + advisory file-state.hash-matches ; doc list -> managed
$ jigc migrate-corpus --dry-run -> 0 "1 would migrate, 1 already current" ; doc show jigc-feedback:<slug> -> 0, no stamp in the bytes served
$ jigc ingest -> 0  adoptable … (adopted) + advisory · file-state.absorbed
$ jigc migrate-corpus -> 0 "1 migrated", diff = +schema-version: 1 ; validate -> 0 clean
# stamp ahead on inconsistency (1 -> 2, committed with git)
$ jigc validate -> 1 schema-conformance.schema-version-ahead ; doc list -> managed ; ingest -> 0 adoptable … (adopted)
$ jigc migrate-corpus --dry-run | migrate-corpus | migrate-corpus --format json   -> 1 ×3  blocked … migrate-corpus.schema-version-ahead ;
                           json blocked[0].key {code, target}, commit null ; HEAD unchanged
$ jigc doc show inconsistency:<slug>   -> 0, `schema-version: 2` visible
# orphaned — a second rig (fresh, both docs filed); control: validate -> 0 clean, doc list -> two `managed`; then JIGC_PACK_DIR set to a dev-only pack copy
$ jigc validate -> 1 schema-conformance.orphaned-instance ×2 ; doc list -> "(none) <path> orphaned" ×2 ; json id null, item-count null, fields null
$ jigc ingest -> 0  "unmanaged docs/jigc-feedback/ — 1 file(s) parse against no schema (left untouched — fine to stay plain)" (×2)
$ jigc migrate-corpus --dry-run -> 0 "0 would migrate, 0 already current, 0 blocked"
$ jigc doc schema jigc-feedback | doc show jigc-feedback:<slug>   -> 1 ×2 store.unknown-type
$ jigc unmanage docs/jigc-feedback/<slug>.md -> 0 "… dropped its file-state baseline; the file is left on disk" ; validate -> 1, orphaned-instance ×2 still

# 7.9 / 7.13 / 7.14 — the RC-A7 construction on a second rig, before migrate-corpus:
$ jigc ingest -> 0 ingest.wrong-location ×2 ; migrate-corpus --dry-run -> 0 "2 would migrate"
$ jigc relocate adr --from docs/decisions -> 1 relocate.frozen-doctype, route -> `jigc migrate-corpus` ; migrate-corpus -> 0 "2 migrated"

# cwd-unreadable — mkdir gone-dir ; cd gone-dir ; rmdir "$REPO/gone-dir"
$ jigc validate | doc list | describe | migrate-corpus --dry-run | ingest | doc schema jigc-feedback   -> 1 each, stdout 0 b
  cannot determine the current directory: No such file or directory (os error 2)
$ jigc validate --format json   -> 1  stdout 0 b · stderr {"error": "cannot determine the current directory: …"}
```

## Doors covered

Every clap leaf that is the door of at least one driven row carrying a repro block — the driver's or
the reconciler's — in `VERB_KINDS` spelling:

`start` · `workflow` · `setup` · `ingest` · `migrate` · `migrate-corpus` · `unmanage` · `relocate` ·
`describe` · `validate` · `doc create` · `doc add-item` · `doc set-field` · `doc set-slot` · `doc show` ·
`doc schema` · `doc list` · `task list` · `task finalize` · `task discard` · `config get` · `config set`
— **22 of 48**. The driver's 20, plus `task discard` (RC-F1) and `config set` (RC-A7k).
