<!-- M51 per-axis review — axis 8 · reconciled · driven on the installed `jigc 1.0.0-rc.15` (commit 35195f56), 2026-09-16 -->

# M51 per-axis review — AXIS 8 · adopter docs & help — RECONCILED

> **This file is the Opus driver's table, unchanged except for one demotion (row 31), followed
> by the Reconciliation ledger against the Codex source pass.** Every Codex claim was entered as
> a lead and driven on `/Users/maurice/.local/bin/jigc` (`jigc 1.0.0-rc.15`, release posture);
> every driver defect was re-driven once to confirm its repro block. Rigs as the driver used
> them, two-step eval, every root a `mktemp -d`, nothing hand-written into `.jigc/`.

---

## The driver table (as filed, one demotion marked inline)

**Binary asserted first, before anything else:**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.15
```

Release posture. Route-fence panics (`#[cfg(debug_assertions)]`) do not exist here — every row
below is what an adopter's installed binary does.

All fixtures built with `dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc`, two-step
eval, states `bare` · `fresh` · `committed-singletons` · `vendored`. No `rm -rf` on a variable
path anywhere; every root is a `mktemp -d`. Nothing was written into `.jigc/` by hand — every
fixture state was built by driving the binary.

---

## 0 · The door set, derived from the code (counts read at `HEAD`, not from the design doc)

| registry | file | count read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1669` | **47** clap leaves |
| `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs` | **24** occurrence rows |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs` | **16** rows |
| `SLUG_DOORS` | `crates/cli/src/cli.rs` | **6** rows |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs` | **25** rows |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs` | **47** rows (the total leaf classification) |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:130` | **10** rows / 9 verbs |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:2625` | **4** (`[&DestroyingDoor; 4]`) |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:~949` | **6** members (`probe-unreliable` · `oob-rename` · `unmigrated-corpus` · `ahead-corpus` · `orphaned-instance` · `foreign-squatter`) |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs:5316` | **60** arms |
| `ManifestKind::ALL` | `crates/cli/src/render.rs:1834` | **6** (5 `in_commit`, 1 left-out-only) |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs:757` | **18** |
| `engine::validate::STORE_FAMILIES` | `crates/engine/src/validate.rs:473` | **7** (the M51-minted registry) |
| `doc::WHOLE_DOC_KEYS` | `crates/cli/src/doc.rs:~5205` | **6** |

**Axis 8's own door set is derived, not enumerated by a registry.** Its subject is *the
doc-sentence batch* — M51's Tier-2/Tier-4 edit set (roadmap → Milestone 51, Increments 9–11),
the EC-11 generated help texts, and the two `include_str!`'d guides — so its doors are *the
verbs those sentences name*. Derived from the code and the wave's own records:

- **The `include_str!`'d guides — two, read at `crates/cli/src/setup.rs:70-71`**:
  `QUICKSTART.md` and `MIGRATING.md`, composed into the installed
  `.claude/skills/jigc/SKILL.md` behind `guide_preamble()` + `unlink_in_repo_links(…)`,
  stamped `jigc-version:` / `jigc-body-blake3:`.
- **The generated help texts — FOUR, not the three the acceptance design names**
  (`crates/cli/src/cli.rs:60` `migrate_corpus_long_about`, `:102` `validate_long_about`;
  `crates/cli/src/doc.rs:5158` `show_long_about`; `crates/cli/src/task.rs:206`
  `finalize_long_about`). The acceptance design's Part 2 row says *"the three EC-11 generated
  help texts"*; the roadmap's Increment 9 Grouped scope says **"EC-11's four help texts"** and
  is the one that matches the binary. Recorded here as a count discrepancy in the planning
  record, not a defect in the product — I drove all four.
- **The guide batch's edit set**, read from `git show 4a00f2d8 -- QUICKSTART.md MIGRATING.md`
  and `git show 0fc80bab -- QUICKSTART.md MIGRATING.md` (the F2 audit fix).

**Doors reached by at least one axis-8 sentence, and driven below: 23** — `start` · `workflow` ·
`setup` · `uninstall` · `upgrade` · `migrate` · `migrate-corpus` · `rename` · `validate` ·
`doc rename` · `doc show` · `doc schema` · `doc list` · `task validate` · `task discard` ·
`task finalize` · `config set` · `milestone create` · `milestone provision` · `milestone execute` ·
`milestone join` · `milestone finalize` · `milestone discard`. Verbs used only to build a fixture
state (`doc create`, `doc set-slot`, `doc set-field`, `milestone add-task`, `task list`,
`config get`, `describe`, `ingest`) are **not** counted as covered — a fixture builder is not the
door of a row.

---

## 1 · The `(door, cell)` table

Cells, from the acceptance design: **C1** the sentence's claim driven at the verb · **C2** the
generated help text equals the registry that generates it · **C3** the guide sentence is true
after install.

| # | door | cell | argv driven | exit | code \| none | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `migrate-corpus` | C2 | `jigc migrate-corpus --help` | 0 | none | none | the 18-kind list in the long help | **matches** — byte-for-byte `SchemaChangeKind::ALL` (18), same order; the EC-11 D5 *"v0→v1"* lie is gone |
| 2 | `validate` | C2 | `jigc validate --help` | 0 | none | none | *"It sweeps every content family, in sweep order: …"* | **matches** — all 7 `STORE_FAMILIES` names + clauses, in declaration order |
| 3 | `doc show` | C2 | `jigc doc show --help` | 0 | none | none | *"a whole-doc object keyed by `type`, `slug`, `item-count`, `schema-version`, `fields`, `sections`"* | **matches** `WHOLE_DOC_KEYS` (6) |
| 4 | `doc show` | C2 (driven against the wire) | `jigc doc show vision:vision --format json` | 0 | none | none | top-level key set | **matches** — `['fields','item-count','schema-version','sections','slug','type']` == the help's list |
| 5 | `task finalize` | C2 | `jigc task finalize --help` | 0 | none | none | *"`jigc task validate <id>` previews part of the finalize gate: … the staged set, promotion and the commit surface at finalize"* | **matches** — the sentence is `whats_left_coverage()`, i.e. `GATE_COVERAGE ▸ Tier::Previewed` ∪ `LaterSummary`, not a typed second home |
| 6 | `validate` | C1 | `jigc validate` on `committed-singletons` | 0 | `schema-conformance.repeatable-populated` | Human (knob) | the report + report-only trailer | **matches** — a family the old help never named is what the sweep actually reports |
| 7 | `validate` | C1 | `jigc validate` on `vendored` with a broken anchor | 0 | `doc-code.symbol-exists` | Human | *"1 finding(s) — report-only at store scope (exit 0); these gate at `jigc task validate` / `jigc task finalize` / `jigc milestone finalize`"* | **matches** — the doc↔code family (help member 1) drives |
| 8 | `validate` | C1 | `jigc validate` over one never-adopted `.md` at the `adr` home | **1** | `schema-conformance.unadopted-instance` (advisory) | Mechanical (`jigc ingest` / `jigc migrate … --as adr`) | the exit-flip trailer *"a never-adopted file sits at a managed home … the sweep exits non-zero"*; `--format json` `report_only=false`, `blocking_probes=[]` | **matches** — `STORE_EXIT_FLIPS` member 6, and MIGRATING's *"`jigc validate` **does** exit non-zero over them"* is true |
| 9 | `validate` | C3 | same as #8, plus a stamp edited to 99 | 1 | `schema-conformance.schema-version-ahead` | Human (upgrade jigc / restore the stamp) | the ahead trailer | **matches** MIGRATING item 6 |
| 10 | `migrate-corpus` | C3 | `jigc migrate-corpus --format json` (clean corpus, and with a squatter) | 0 | none | none | the eight top-level keys | **matches** — `migrated`, `already_current`, `blocked`, `unadopted`, `unfilled`, `commit`, `hook_output`, `dry_run`: exactly MIGRATING's *"**five** sets, not three … and alongside them `commit` …, `hook_output` and `dry_run`"* |
| 11 | `migrate-corpus` | C3 | same, with the squatter committed | **0** | none | none | `unadopted: 1`, `blocked: 0` | **matches** — *"excluded from the fold before it runs, `migrate-corpus` still exits 0 over them"*, and *"only `blocked`'s emptiness is the exit rule"* |
| 12 | `migrate-corpus` | C3 | `jigc migrate-corpus` over an ahead stamp | **1** | `migrate-corpus.schema-version-ahead` | Human | *"this jigc build has no schema to migrate it to"* | **matches** MIGRATING item 6's *"and it blocks too"* |
| 13 | `setup` | C3 | `jigc setup` over a dirty **regenerated** path (`.jigc/AGENT.md`) | **1** | `setup.dirty-install-path` | Human + `--force` | *"nothing was installed and no install commit was made — `HEAD` is untouched"*; `git status` still ` M .jigc/AGENT.md`; the user's line still on disk | **matches** — the F2 audit fix holds at the exact path that lost bytes pre-fix |
| 14 | `setup` | C3 | `jigc setup` on a **first** install with an untracked `CLAUDE.md` | **1** | `setup.dirty-install-path` | Human + `--force` | `.jigc` absent, `SKILL.md` absent after the refusal | **matches** — *"installs **nothing**"* is literal: the pre-write gate fires before the first write |
| 15 | `setup` | C3 | `jigc setup --force` on the same state | 0 | `setup.forced-install-path` (advisory) | Human (`git show HEAD -- <path>`) | the install commit `b27888e` carries `CLAUDE.md`; the user's 7 lines survive in it | **matches** — *"lets the install run and commit those paths as it leaves them … and *says* which paths it was spent on"* |
| 16 | `setup` | C3 | `jigc setup` with a **rejecting** `pre-commit` hook planted | 0 | none | none | install commit `b9c5161` lands anyway | **matches** — QUICKSTART's stated single exception: *"`jigc setup`'s own install commit passes `--no-verify`"* |
| 17 | `setup` | C1 | `jigc setup` over a foreign `.git/hooks/pre-commit` | 0 | none | none | resulting hook = jigc's block + the foreign body verbatim; `grep -c 'ACME CORP POLICY HOOK'` → 1 | **matches the stated bound** (`install_candidate_paths` doc-comment: the hook is deliberately outside the pre-write gate because a foreign hook is preserved verbatim). **Observation O-1** below on the guide's *"every path in its own install footprint"* universal |
| 18 | `setup` | C3 | `jigc setup`, then read `.claude/skills/jigc/SKILL.md` | 0 | none | none | 34 217 bytes; `jigc-version: 1.0.0-rc.15`; `jigc-body-blake3: c25024ac…`; every M51 sentence present (`setup.dirty-install-path` ×2, `cargo install --path crates/cli`, *"What an upgrade may change"*, *"the tenth committing door"*, `record commit:`, *"no jigc commit of *your*"*, `unadopted`, `schema-version-ahead`) | **matches** — the batch shipped into the installed artifact |
| 19 | `setup` | C3 | `grep -oE '\]\([^)]*\)' SKILL.md` | — | none | none | **zero** surviving markdown links; `MIGRATING.md → Reconciling and backing out` resolves to `## Reconciling and backing out` at line 326 of the same file | **matches** — the preamble's *"A cross-reference to `QUICKSTART.md` or `MIGRATING.md` means the matching part of this file"* is true after flattening |
| 20 | (step zero) | C3 | `cargo install --path crates/cli --root <tmp>` | 0 | none | none | `Installed package 'cli v1.0.0-rc.15' (executable 'jigc')`; `jigc --version` → `jigc 1.0.0-rc.15` | **matches** — QUICKSTART's new *"there is **one** install command"* |
| 21 | `validate` | C3 | the freshly `cargo install`-ed binary drives `jigc setup` + `jigc validate` in an isolated `$HOME` rig | 0 | none | none | `doc-code` extracted beside the new binary; `no findings — the committed store validates clean` over a corpus with real anchors | **matches** — *"a `cargo install` from a fresh machine is a supported install channel: no manual probe copy"* |
| 22 | `upgrade` | C2/C1 | `jigc upgrade --help`, then `jigc upgrade` | 0 | none | none | *"Report-and-route only — it changes nothing"*; before/after digest of `git status` + `HEAD` + every `.jigc` file identical | **matches** — and the versioning paragraph's *"`jigc upgrade` … writes nothing itself"* |
| 23 | `doc schema` | C3 | `jigc doc schema adr --format json` | 0 | none | none | `contract-version: 6` present | **matches** the versioning paragraph's *"`contract-version` on `jigc doc schema`'s projection"* |
| 24 | `doc show` | C3 | `jigc doc show changelog:changelog --format json` | 0 | none | none | no `contract-version`; `schema-version: 2` == the doc's own on-disk stamp | **matches** *"`jigc doc show` … carry **no** version of their own … the `schema-version` you see on a `doc show` is the *document's*"* |
| 25 | `doc list` | C3 | `jigc doc list --format json` | 0 | none | none | top-level keys `['docs']` — no version key at all | **matches** the same sentence's `doc list` half |
| 26 | `validate` | C3 | `jigc validate --format json` | 0/1 | — | — | `schema_version: 3` on the envelope | **matches** *"the `schema_version` riding jigc's structured result envelopes"*. Observation O-2: `task list --format json` is a bare `[]` and carries none — the census's declared top-level-array anomaly, not a new gap |
| 27 | `task finalize` | C3 | `jigc task finalize <fresh task> --dry-run` | **3** | `schema-conformance.field-value-conformant`, `…required-slot-present` | Mechanical | two blocking findings, **no manifest** | **matches** — EC-19 fixed: QUICKSTART's old *"prints the manifest and stops, changing nothing"* is now *"On a task that would otherwise commit cleanly…"*, and the driven state is the one its own narrative puts the reader in |
| 28 | `task finalize` | C3 | `jigc task finalize <clean task> --dry-run` | 0 | none | none | `finalize --dry-run — pre-commit manifest (nothing committed)` / `would commit — …` / `  added newfile.txt` | **matches** |
| 29 | `task finalize` | C3 | same task with a pre-task staged path | **3** | `finalize.carried-staged` ×2 (one per path) | Mechanical (`git restore --staged`) + `--carry-staged` | one finding **per carried path** | **matches** MIGRATING item 4 |
| 30 | `task finalize` | C3 | `… --dry-run --carry-staged` | 0 | none | none | `  carried-over carried.txt` / `  carried-over newfile.txt` | **matches** — *"the manifest then labels it `carried-over`"* |
| ~~31~~ | `task finalize` | C2-shaped (guide ↔ registry) | **DEMOTED — not driven** (no argv, no exit; see ledger) | — | — | — | MIGRATING: *"`promoted` / `modified` / `deleted` / `added` / `carried-over`, the five a committed path can carry; the manifest's left-out list adds a sixth, `untracked`, which no committed path ever carries"* | **matches** `ManifestKind::ALL` (6) partitioned by `in_commit()` 5/1; `added` and `carried-over` driven above, `promoted` driven at every rig finalize |
| 32 | `task discard` | C3 | `jigc task discard <ordinary task>` | **1** | `task-discard.staged-prose` | Mechanical (read-back → finalize → `--force`) | area still on disk | **matches** — *"expect the refusal on any task you have started"* |
| 33 | `task discard` | C3 | `jigc task discard <ordinary task> --force` | 0 | none | none | `discarded task ordinary-job — dropped staged edits to: commit:ordinary-job (transient)`; area gone | **matches** |
| 34 | `task discard` | C3 | `jigc task discard <milestone sub-task>` | 0 | none | none | `record commit: 57d4285   — this sub-task's milestone record, settled to \`discarded\` and committed on its own`; `HEAD` moved | **matches** — the guide's *"tenth committing door"* + *"the ack names the sha on a `record commit:` line"* |
| 35 | `task discard` | C3 | same, under a rejecting `pre-commit` hook | **1** | (survivable frame) | Human (fix the hook, re-run the printed line) | *"nothing was committed — the milestone record still names task:sub-job as it did, and the task's working area is intact"*; `HEAD` unmoved; the area and the record both intact; `jigc task discard sub-job` still resolves | **matches** — *"the commit runs before the area is removed, so a refused commit leaves both intact and the line it prints to re-run still resolves"* (`team-ready-state.md` → *It settles before it destroys*) |
| 36 | `task discard` | **C1** | `jigc task discard --help` | 0 | none | none | `about` = *"Abandon the task — remove its working area `.jigc/tasks/<id>/`"* — and **nothing else** | **DEFECT D-2** (below): the one adopter-facing surface of the EC-12 class left unswept |
| 37 | `uninstall` | C2 | `jigc uninstall --help` | 0 | none | none | *"**Three** states it refuses"*, `--force` *"Inert when **all three** guards are already clean"* | **matches** — driven to 3/3 in rows 38–40 |
| 38 | `uninstall` | C1 | `jigc uninstall` with an open task staging a doc | 1 | `uninstall.staged-prose` | Mechanical | names `open-job: commit:open-job` | **matches** |
| 39 | `uninstall` | C1 | `jigc uninstall` with `.jigc/config/notes.txt` untracked | 1 | `uninstall.untracked-workbench-file` | Mechanical (`git add <path>`) | names the path | **matches** |
| 40 | `uninstall` | C1 | `jigc uninstall` with a provisioned worktree holding `wip.txt` | 1 | `uninstall.dirty-worktree` | Human + Mechanical (`milestone discard … --force`) | `?? wip.txt — … registered as a worktree of this repository` | **matches** |
| 41 | `uninstall` | C3 | `jigc uninstall --force` | 0 | none | none | two `warning:` blocks — *"destroys 1 file(s) … not recoverable"* and *"also removes 5 tracked file(s) … `git checkout -- <path>` brings it back"*; `.jigc/` gone | **matches** — *"it never buys silence — the loss is narrated either way"* |
| 42 | `milestone discard` | C2 | `jigc milestone discard --help` | 0 | none | none | *"`--force`, the single consent for **both** guards"* | **matches** — the two guards are `milestone.dirty-worktree` + `milestone.staged-prose` |
| 43 | `milestone discard` | C1 | `jigc milestone discard m3-x` over a dirty worktree | 1 | `milestone.dirty-worktree` | Human + Mechanical (`--force`) | the per-path shape clause *"registered here, so the teardown removes it and this content is destroyed"* | **matches** — EC-17's *"no door narrates a destruction it does not perform"* |
| 44 | `milestone provision` | C1 | `jigc milestone provision m4-y` over an unregistered leftover **directory** | 1 | `milestone.leftover-holds-work` | Mechanical (`--force`) | `…/left-job: stuff.txt — git reports no worktree of its own there` | **matches** EC-17 (`LeftoverShape::Directory`) |
| 45 | `milestone provision` | C1 | same, over a leftover **file** at the worktree path | 1 | `milestone.leftover-holds-work` | Mechanical (`--force`) | `…/left-job: the file itself — it is a file, not a worktree` | **matches** EC-17 (`LeftoverShape::File`) — the M49-audit `is_dir()` class stays closed |
| 46 | `milestone create` | C3 | `jigc milestone create "M5 z"` with a foreign file staged | 0 | none | none | `record commit: 4d9d10a   — the record on its own; anything else you had staged stayed staged`; `foreign.txt` still staged; `HEAD` moved by the record alone | **matches** MIGRATING item 4's *"the milestone record-only doors commit without asking this question at all"* |
| 47 | `rename` | C3 | `jigc rename adr:use-redis --to "Use Valkey"` with a foreign file staged | **1** | `rename.dirty-tree` | Human | *"a rename commits in place with no pathspec, so anything already in the index would ride its commit"* | **matches the literal claim** (the *carryover* question is not asked) — **Observation O-3**: the guide lists `rename` among doors that "commit without asking this question at all", and a reader infers the foreign path would be swept in; driven, `rename` refuses harder |
| 48 | `config set` | C2 | `jigc config set --help` | 0 | none | none | *"A **placement** doctype's file is not among them … a `docs-root` re-point never moves it"* | **matches** EC-16 |
| 49 | `config set` | C1 | `jigc config set docs-root documentation/` on a corpus of **placement** doctypes only | 0 | none | none | ack names no move; `docs/decisions-log.md` + `docs/roadmap.md` unmoved | **matches** — the old universal would have claimed them |
| 50 | `config set` | C1 | same, with a committed `adr` under the old root | 0 | none | none | `relocating 1 committed doc(s) … (… a placement doctype's file is not carried …)` / `docs/decisions/use-redis-caching.md → documentation/decisions/use-redis-caching.md`; `git status` shows `R` | **matches** — both halves of the corrected subject |
| 51 | `start` | C1 | `jigc start --task <sub-task>` **before** provision | 1 | (base-mismatch refusal) | Mechanical (`jigc milestone provision …`) | *"run `jigc milestone provision m9-the-arc` — it adds a worktree that is missing and leaves one that exists untouched — then re-run this from that worktree"* | **matches** — the route is stated idempotent, so it is not a route that changes nothing (M46 PT-1 class) |
| 52 | `start` | C1 | `jigc start --task <sub-task>` **after** provision, from the main checkout | 1 | same | Mechanical | identical line | **matches** — by the idempotency clause; the line is true in both states rather than three different lines |
| 53 | `workflow` | C1 | `jigc workflow sub-task --task first-sub-job` from inside `.jigc/worktrees/first-sub-job` | 0 | none | none | `resume: \`jigc workflow sub-task --task first-sub-job\`   — re-composes this workflow and provisions this sub-task's write-ready docs area on first entry; run it from this sub-task's own worktree at \`.jigc/worktrees/first-sub-job\`` | **matches** — the M51 `resume:` line names the door that provisions **and where it runs** |
| 54 | `milestone execute` | C1 | `jigc milestone execute m9-the-arc` | 0 | none | none | the composed step text: `Spawn: cd .jigc/worktrees/<id> && jigc workflow sub-task --task <id>` per sub-task, and the worktree-deps paragraph | **matches** — every command the step text names is a verb that answers |
| 55 | `milestone join` | C1 | `jigc milestone join m9-the-arc` + `--format json` | 0 | none | none | text `no docs staged from: second-sub-job`; JSON `no_docs_from: ["second-sub-job"]`, `overlay: {commit:first-sub-job: {provenance: "created", source_task: "first-sub-job"}}` | **matches** EC-15 — the overlay is per-sub-task provenance, `no_docs_from`'s values are true, the shape did not move |
| 56 | `milestone finalize` | C2 | `jigc milestone finalize --help` | 0 | none | none | the two-half fold sentence + `--carry-staged`'s *"the carried entries never ride it — they stay staged across the boundary either way"* | **matches** (the two-half fold is the arm `help_truth.rs` already fences) |
| 57 | `migrate` | C1 | `jigc migrate adir --as adr` | **1** | (read-fault, route-bearing) | Mechanical (`jigc migrate <path> --as adr`) | ``could not read the foreign `adr` source at `adir` `` — **the token as typed**, no host path | **matches** — the M51 audit's third LOW (`dc508994`) holds in the release binary |
| 58 | `doc show` | C1 | `jigc doc show adr:nope` | 1 | `store.not-found` | Mechanical (`jigc doc show adr:nope --task <task-id>` + `jigc task list`) | the route's `--task` arm | **matches** N15 |
| 59 | `doc show` | C1 | `jigc doc show adr:nope --task ghost-task` | 1 | `finalize.no-task` | Mechanical (`jigc task list`) | *"no task `ghost-task`"* | **matches** |
| 60 | `doc rename` | C1 | `jigc doc rename … --to "Use Memcached caching" --task <id>` after the summary named the old title | 0 | **none printed (agent text)** | none | ack only: `adr:use-memcached-caching (renamed to "Use Memcached caching" from adr:use-redis-caching)` | **DEFECT D-1 (producer half, text arm)** |
| 60b | `doc rename` | C1 | the same argv with `--format json` | 0 | `commit-recording.stale-title` present in `findings` | Mechanical | the write-ack envelope carries `op`/`from`/`target`/`reslugged`/`title` **and** the finding | **matches** — the producer works; only the text arm is silent |
| 61 | `task validate` | C1 | `jigc task validate <id>` after that rename | 0 | `commit-recording.stale-title` (advisory) | Mechanical (`jigc doc set-slot commit:<id>#summary --task <id> --from-file -`) | the advisory renders in **text** | **matches** — the re-raise works here |
| 62 | `task finalize` | C1 | `jigc task finalize <id> --dry-run` after that rename (clean task) | 0 | **none printed** | none | `would commit — docs: adopt Use Redis caching for the API` / `  promoted docs/decisions/use-memcached-caching.md` — the stale subject and the new path on adjacent lines, unconnected | **DEFECT D-1** |
| 63 | `task finalize` | C1 | same, `--format json` | 0 | `commit-recording.stale-title` present in `findings` | Mechanical | the JSON envelope carries it | **matches** (JSON arm only) — which is what makes D-1 a text/JSON divergence rather than a missing computation |
| 64 | `start` | C1 | `jigc start` (orientation) after that rename | 0 | `commit-recording.stale-title` | Mechanical | `findings: 2 advisory` + the rendered advisory | **matches** |
| 65 | `task finalize` | C1 | the real `jigc task finalize <id>` after that rename | 0 | `commit-recording.stale-title` | Mechanical | printed in text before `finalized 3be4bfb — docs: adopt Use Redis caching for the API` | **matches** — the committing preflight re-raises in text |

**Rows driven: 65 (was filed as 66). Rows demoted to source-read: 1** — row 31 carries no argv and no exit and therefore did not run; the caption's *"every row in the table ran"* is corrected here. Its underlying source claim was checked and holds (`ManifestKind::ALL`, `render.rs:1834` = 6 members; `in_commit()` at `render.rs:1866-1875` = 5 true / `Untracked` false), and the tags it names as driven (`added`, `carried-over`, `promoted`) are driven by rows 28–30 and every rig finalize — so the fact survives as a **source read plus three driven tags**, not as a driven row. Un-driven `(door, cell)`
pairs are listed in §4 — they were never entered into the table as rows.

---

## 2 · Defects

### D-1 — the two surfaces F-9 named are the two that stay silent in the DEFAULT format, and three records say otherwise

**What is contradicted.** Three statements, all unqualified by format:

1. `crates/cli/src/task.rs:1632-1641` (the producer's own sited comment):
   *"being unconditional (no `GatePreview` arm) it reaches `task validate`, **the `--dry-run`
   forecast**, the committing preflight and `start`'s orientation from this one computation,
   which is what makes the advisory *re-raised* rather than printed once."*
2. `implementation/roadmap.md` → Milestone 51, Increment 11 amendment: *"produced at `doc rename
   --task` and **re-raised by the task-scope sweep** at `jigc task validate <id>` and `task
   finalize --dry-run`"*.
3. F-9's own reported repro, which this increment exists to close: *"`task finalize --dry-run`
   prints the stale subject and the new path on adjacent lines, unconnected."*

Driven on rc.15 in the **default `agent` format**, (3) still reproduces verbatim.

**The producer half is the same split.** `jigc doc rename … --task <id>` prints the ack alone in
the default `agent` format; the identical argv with `--format json` carries the finding in the
write-ack envelope beside `op`/`from`/`target`/`reslugged`/`title`. So the two surfaces F-9 named
by name — *"a `doc rename` leaves the staged commit summary naming the old title"* and *"`task
finalize --dry-run` prints the stale subject and the new path on adjacent lines, unconnected"* —
are **exactly** the two of five surfaces that are silent in the format an agent reads by default.
The three that do print it in text (`task validate`, `jigc start` orientation, the committing
`task finalize`) are the three F-9 did not name.

**Nothing fences the divergence.** `crates/cli/tests/dry_run_findings_equal_set.rs` iterates
`Tier::Previewed` behaviourally, and `commit-recording.stale-title` is deliberately outside that
tier (`task.rs:1634-1638`: *"it is a **content** finding, not a gate — joining the previewed set
would propagate a `GATE_COVERAGE` row to all eight enumerating surfaces"*), so the equal-set fence
does not reach it; and M48's standing text/JSON parity fence runs **text → envelope** (*"a value
the text prints but the envelope withholds is a gap"*), which is the direction that cannot see a
value the envelope prints and the text withholds.

**It is format-split, and the renderer declares the split deliberate** — `render.rs`
`finalize_manifest`'s doc-comment: *"**It is a JSON-only key, deliberately.** … the agent/human
forecast keeps the shape it has, and `jigc task validate` remains the text surface for the
findings themselves."* So the product has one behaviour and three records assert a wider one;
the divergence is between homes, and the home an agent reads by default is the one that is
silent. The class is wider than F-9: the text manifest arm drops **every** advisory (the
`file-state.staged-copy` row is dropped in the same run), so a dry-run forecast is a findings
surface in JSON and not in text.

**Repro (setup · argv · observed):**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$JIGC start "record the cache choice" --workflow record-decision
TID=record-the-cache-choice
$JIGC doc create adr --title "Use Redis caching" --task "$TID"
for s in context decision consequences; do printf 'Prose for %s.' "$s" \
  | $JIGC doc set-slot adr:use-redis-caching#$s --task $TID --from-file -; done
$JIGC doc set-field commit:$TID#header/type --task $TID --value docs
printf 'adopt Use Redis caching for the API' | $JIGC doc set-slot commit:$TID#summary --task $TID --from-file -
printf 'Body prose.'                         | $JIGC doc set-slot commit:$TID#body    --task $TID --from-file -

$ $JIGC doc rename adr:use-redis-caching --to "Use Memcached caching" --task "$TID"
adr:use-memcached-caching (renamed to "Use Memcached caching" from adr:use-redis-caching)
EXIT=0                                   # producer prints nothing about the stale summary (agent text)

$ $JIGC doc rename adr:use-redis-caching --to "Use Memcached caching" --task "$TID" --format json
{ … "findings": [ { "code": "commit-recording.stale-title", … } ], "op": "rename",
  "from": "adr:use-redis-caching", "reslugged": true, "title": "Use Memcached caching" }
EXIT=0                                   # the SAME call, JSON arm: the finding is there

$ $JIGC task validate "$TID"
advisory · commit-recording.stale-title — the staged commit summary still names `Use Redis caching` …
EXIT=0                                   # re-raise #1: PRESENT

$ $JIGC task finalize "$TID" --dry-run
finalize --dry-run — pre-commit manifest (nothing committed)
would commit — docs: adopt Use Redis caching for the API
  promoted docs/decisions/use-memcached-caching.md
EXIT=0                                   # re-raise #2: ABSENT — F-9's exact repro, unrepaired

$ $JIGC task finalize "$TID" --dry-run --format json | grep stale-title
      "code": "commit-recording.stale-title",     # present on the JSON arm only

$ $JIGC task finalize "$TID"
advisory · commit-recording.stale-title — … EXIT=0 ; finalized 3be4bfb — docs: adopt Use Redis caching for the API
                                         # the real door re-raises in text; the landed subject is stale anyway
```

**Severity: MEDIUM.** No data loss; the commit lands with a stale subject exactly as F-9
reported, and the surface the wave's record names as a repair site is silent in the format an
agent gets by default. The fix is a choice between making the three statements format-honest
and making the text arm carry the findings — not mine to make.

### D-2 — `jigc task discard --help` never says it commits

`task discard` is `COMMITTING_DOORS` row 10. Its entire `about` is:

```
$ jigc task discard --help
Abandon the task — remove its working area `.jigc/tasks/<id>/`
```

Driven on a milestone sub-task, the same verb settles a committed record item and lands a
commit (row 34: `record commit: 57d4285`, `HEAD` moved). Both shipped guides were corrected in
this wave to name that (EC-12's *qualifier*: *"One task shape carries a condition, and it is the
only one … That makes it the tenth committing door"*), and the sibling destroying door's help
does say it (`milestone discard --help`: *"in one record-only commit"*). The door that prints
the sentence is the one surface of the class left unswept — against Increment 9's own
deliverable, *"Every surface that says something the binary does not do is corrected **at the
door that prints it**"*, and against `surface-contract.md` law 3 (nothing ambushes): a door
whose help describes a directory removal makes a commit.

Honest bound on this finding: the sentence is an **omission**, not a false predicate, and
EC-12's declared axis was *"every adopter-facing sentence that says a verb does not commit"* —
this one does not say that. It is reported as a defect because the help is the adopter-facing
surface of the same fact, and because the two guides and the sibling door were all corrected.

```
$ jigc task discard --help          # → the one-line about above, no mention of a commit
# and, driven:
$ $JIGC milestone create "M1 the first"; $JIGC milestone add-task m1-the-first "do the thing"
$ git rev-parse --short HEAD
ef83d05
$ $JIGC task discard do-the-thing
discarded task do-the-thing
record commit: 57d4285   — this sub-task's milestone record, settled to `discarded` and committed on its own
EXIT=0
$ git rev-parse --short HEAD
57d4285
```

**Severity: LOW.**

---

## 3 · Observations (driven, not graded as defects)

- **O-1 · `setup`'s "every path in its own install footprint" is a universal with one real
  exception.** QUICKSTART: *"Before writing anything, `setup` compares **every path in its own
  install footprint** against `HEAD`. If any of them already carries changes that are in no
  commit — staged, unstaged or untracked — it stops."* `.git/hooks/pre-commit` is in the
  footprint by the binary's own ack (*"pre-commit hook → .git/hooks/pre-commit"* under *"setup
  installed:"*), and a foreign hook there does **not** stop setup. Driven, **no bytes are lost**
  — `install_precommit_hook` preserves the foreign body verbatim below jigc's block — and
  `install_candidate_paths`' doc-comment states the exclusion with that reason, so the
  consequence the guide promises (*"your bytes are still exactly where you left them"*) holds.
  Prose precision only.
  ```
  $ printf '#!/bin/sh\n# ACME CORP POLICY HOOK v7 - do not remove\nexec ./scripts/secret-scan.sh\n' > .git/hooks/pre-commit
  $ $JIGC setup            # EXIT=0, no setup.dirty-install-path
  $ grep -c 'ACME CORP POLICY HOOK' .git/hooks/pre-commit
  1                        # foreign body preserved, appended after jigc's block
  ```
- **O-2 · `jigc task list --format json` carries no `schema_version`.** The versioning paragraph
  says *"the `schema_version` riding jigc's structured result envelopes"*. Driven, `task list`
  emits a bare top-level array (`[]`), which the M51 envelope-key census already declares as its
  single top-level-array anomaly (acceptance design, leaf 25's `only-5` reason). Not a new gap;
  recorded so the reconciliation does not re-find it.
- **O-3 · the carryover paragraph understates `jigc rename`.** MIGRATING item 4: *"It is not a
  universal over every jigc commit: `jigc rename`, `jigc migrate-corpus` and the milestone
  record-only doors commit **without asking this question at all**."* Literally true — the
  carryover gate is not asked. Driven, `rename` asks a **stricter** question and refuses
  (`rename.dirty-tree`, exit 1, nothing committed), so the reader's inference (that a staged
  foreign path would ride a rename's commit) is the opposite of the shipped behaviour. The
  milestone record-only half of the sentence is exactly right (row 46).
- **O-4 · the acceptance design and the roadmap disagree on the EC-11 count.** Design Part 2:
  *"the three EC-11 generated help texts"*; roadmap Increment 9: *"EC-11's **four** help texts"*.
  The binary ships four (`migrate_corpus_long_about`, `validate_long_about`, `show_long_about`,
  `finalize_long_about`). A planning-record discrepancy, not a product defect; all four driven.
- **O-5 · the minted probe-family registry is `STORE_FAMILIES` with SEVEN members, not the five
  the Settle predicted.** `settle-record.md` and `baseline-prose.md` both name *"the five store
  probe families"* (`design/validation.md`'s taxonomy); the shipped registry
  (`crates/engine/src/validate.rs:473`) carries seven and its doc-comment states the widening and
  the one carve-out (the orphan-strand arm folded into the file↔CLI-state clause). The fixer
  widened correctly; the help renders the shipped set. Recorded because the numeral appears in
  two planning artifacts.

---

## 4 · What I did NOT drive, and why

Stated plainly, never presented as driven:

1. **`(task discard, C3)` — the guide's *"Nothing of *yours* is committed on either path — only
   the record item moves, and every sibling item and the milestone's own status are
   byte-untouched"***. I drove the commit and the sha; I did **not** diff the record's sibling
   items and header before/after to prove byte-untouchedness. That is `team-ready-state.md`'s
   per-item honesty rule and belongs to a milestone-record axis, not to an adopter-doc sentence
   about which door commits.
2. **`(migrate-corpus, C3)` — the `unfilled` array's own sentence** (*"names the leaves a
   migration *added* that carry a `set:` deriver and no default"*). Reaching a non-empty
   `unfilled` needs a manufactured pack with a shape change at a `set:`-without-default leaf —
   that is axis 7's `SchemaChangeKind × LOCI` fixture, and driving it here would prove axis 7
   twice. The key's **presence and emptiness** are driven (rows 10–11).
3. **`(migrate-corpus, C3)` — the `hook_output` key's non-empty arm.** Driven empty only; the
   rejecting-hook arm of `migrate-corpus` is axis 4's transaction cell.
4. **`(milestone finalize, C1)` — the two-half fold actually landing.** I drove only its help
   text (row 56). A real fan-out → join → finalize with code staged in two worktrees is axis 4's
   subject and carries no axis-8 sentence of its own beyond the *"ten committing doors"* count,
   which row 34 already moves off nine.
5. **`(setup, C3)` — `jigc setup --force` over a dirty `.git/hooks/pre-commit`.** The VERDICT
   declares this bound in `install_candidate_paths`' doc-comment (*"`--force` does not probe the
   hook path"*); I drove the plain path (row 17) and did not construct the `--force` variant,
   because the declared bound and the verbatim-preservation drive together settle the byte
   question.
6. **`(upgrade, C1)` — a recorded config delta actually drifting.** `jigc upgrade` was driven
   over a corpus with **no** recorded deltas (*"no findings — no recorded config deltas to
   check"*), so the *"Re-check **every** recorded config delta"* universal is driven only in its
   empty case. Manufacturing a delta + a pack move to make it drift is a multi-pack fixture
   (axis 7 territory).
7. **The Tier-4 record corrections (Increment 10, EC-31 … EC-43).** Every one is an edit to a
   dated record in `completions/artifacts/RC-rc14/` — the increment's own *Proves* line says
   *"Nothing through the binary"*. There is no verb to drive; they are a source-reading job and
   belong to the Codex pass, not to this table.
8. **`cargo install --path crates/cli` into the operator's real `~/.cargo/bin`.** Driven into a
   `mktemp -d` root instead (row 20), because installing into the real root would replace the
   `1.0.0-rc.15` this whole review is measured against.

I did **not** read the Codex source pass for this axis.

---

## 5 · What this adds over the flow-52 arm

**Axis 8 has no flow-52 arm, by an explicit decision.** The acceptance design's *"Deliberately
unrepresented"* list names exactly this axis's subject twice: *"the Tier-2 law-1 wording batch
and the Tier-4 record corrections: surfaces whose only change is what they **say**; each is keyed
to its findings-verification row, and none mints a verb, code or route"*, and *"D6 — the
release-versioning policy (two prose homes plus the guide body) and §13's batched adopter-doc
hash move: doc-only. The **behaviour** they describe is proven by arms 5 and 9."*

So the honest comparison is against **arms 5 and 9**, which are the arms the design says stand in
for this axis:

- **Arm 5 (the `EnvelopeArm` registry, 60 arms)** proves that each of the 47 leaves emits a
  declared `--format json` key set and that the four pre-pin deletes are off the wire. It reads
  the **wire**. It does not read a single English sentence, so it cannot see that
  `migrate-corpus --help` once named a transform space that did not exist, that `validate --help`
  named one of seven families, or that `doc show --help` listed four of six keys — every one of
  those was true while the envelope was perfectly conformant. Rows 1–5 and 23–26 are the reading
  arm 5 structurally cannot do: **help-and-guide prose compared to the registry that now
  generates it**, and to the wire it describes.
- **Arm 9 (the orphan arm over `STORE_EXIT_FLIPS`)** proves membership and the exit flip. Rows
  8–12 drive the **adopter's sentence about** that machinery — that `migrate-corpus` exits 0 over
  an `unadopted` file while `jigc validate` exits non-zero over the same file, which is a
  cross-verb claim no single arm's registry carries, and which a CI-gating adopter acts on.
- **Neither arm installs the guide.** Rows 18–21 are the only place the composed artifact is read
  **after a real `jigc setup`**: the stamp moves, the batch's sentences are actually in the
  shipped bytes, every relative link is flattened, the one cross-reference resolves inside the
  same file, and step zero's install command was run and produced a working `jigc`. A guide
  sentence that ships into every adopter repo is product surface; flow 52 never opens it.
- **And the two defects are both invisible to the arm set by construction.** D-1 lives in the
  *text* arm of a surface whose *JSON* arm arm 5 fences and finds conformant — the dry-run
  `findings` key is present, declared and correct, which is exactly why `dry_run_findings_equal_set.rs`
  is green over it. D-2 is a `--help` string; no arm reads one.

The general statement: flow 52's arms iterate **sets the code can enumerate**. Axis 8's subject
is the set of **sentences the code cannot enumerate** — which is why its door set had to be
derived from the wave's own edit set, and why the two defects it found are one text renderer and
one `about` string rather than a registry member.

---

# Reconciliation ledger

Binary re-asserted before every drive in this section:

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.15
```

The rule applied: **a claim by one pass that the other cannot reproduce is a LEAD, not a
finding.** Each Codex claim below was entered as `lead(codex, …)` and then *driven* — to a repro
block (→ CONFIRMED, origin codex) or to a falsifying datum (→ REFUTED). Each driver defect was
re-driven once here to confirm its repro block.

## A · Codex claims (source pass → driven)

### CX-1 — CONFIRMED (origin codex) · `migrate-corpus --help`'s *"`--dry-run` writes nothing at all"* is falsified by the invocation log

`lead(codex, "--dry-run does write to disk when the invocation log is enabled, contradicting
crates/cli/src/cli.rs:74-75 and MIGRATING.md:27")`.

**Driven.** The log did not exist before the dry run and exists after it, carrying the dry run's
own record:

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
cd "$REPO"

$ $JIGC config set invocation-log true        # EXIT=0
$ ls .jigc/logs/invocations.jsonl
(eval):5: no such file or directory: .jigc/logs/invocations.jsonl      # before_bytes=0

$ $JIGC migrate-corpus --dry-run
corpus migration (dry run — nothing written): 0 would migrate, 4 already current, 0 blocked
  current    CHANGELOG.md
  current    VISION.md
  current    docs/decisions-log.md
  current    docs/roadmap.md
EXIT=0

$ wc -c < .jigc/logs/invocations.jsonl
194                                            # after_bytes=194 — the file now exists
$ tail -1 .jigc/logs/invocations.jsonl
{"timestamp":"2026-09-16T14:33:31Z","argv":["migrate-corpus","--dry-run"],"exit_code":0,
 "duration_ms":305,"finding_codes":[],"output_bytes":280,"binary_version":"1.0.0-rc.15","error_code":null}
```

**Bounds, driven in the same run — these are what make it prose-precision rather than a
behaviour defect:**

```
$ git status --porcelain                      # byte-identical to the pre-dry-run capture
$ git rev-parse HEAD                          # d97f2 … unmoved
$ git check-ignore -v .jigc/logs/invocations.jsonl
.jigc/.gitignore:6:logs/    .jigc/logs/invocations.jsonl
```

So the only byte the dry run writes is a **gitignored workbench log entry behind an opt-in knob**
(the rig's earlier `setup`/`start`/three `finalize`s wrote no `.jigc/logs/` at all, the knob being
off). The verb's own ack (*"dry run — nothing written"*) and the help clause (*"`--dry-run` writes
nothing at all"*) are both unqualified universals; the sentence's own context is the migration's
writes (*"lands its own migration in a pathspec-limited commit; `--no-commit` leaves the writes
unstaged; `--dry-run` writes nothing at all"*), which is the reading under which it is true.
**MIGRATING.md item 3 is not reached** — it qualifies itself on the spot (*"changes nothing — no
migrated bytes, no relocation move, no commit"*), and all three of those held. Same shape as the
driver's O-1: a universal with one real, self-declared exception. **Severity: LOW** (prose
precision; the reached home is one help clause + one ack, and the write is opt-in and gitignored).

### CX-2 — CONFIRMED (origin codex) · the installed guide's *"every `jigc setup` rewrites it"* is false for a user-modified guide

`lead(codex, "QUICKSTART.md:57-62 says every jigc setup rewrites the guide, but setup.rs:172-222
classifies an edited/unstamped guide as user-modified and setup.rs:260-274 leaves it untouched")`.

**Driven** on a `bare` rig, with the user's file **committed** first so the pre-write dirty gate
(`setup.dirty-install-path`, driver rows 13–15) cannot be what fires:

```
rig=$(dev/jigc-rig bare --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"; cd "$REPO"
$ mkdir -p .claude/skills/jigc
$ printf 'MY OWN SKILL NOTES\nthis is the user file\n' > .claude/skills/jigc/SKILL.md
$ git add .claude/skills/jigc/SKILL.md && git commit -q -m "add my own skill file"
$ git status --porcelain          # clean

$ $JIGC setup
jigc setup — adapter installed
jigc is now wired into this project; setup installed:
  - bootstrap reference → CLAUDE.md   …
  - jigc allowlist → .claude/settings.json   …
  - SessionStart hook → .claude/settings.json   …
  - pre-commit hook → .git/hooks/pre-commit   …
  - install commit → 02168dc   …
                                   # ← NO `jigc guides → …` row: the ack does not claim the write
advisory · adapter-guide.user-modified — `.claude/skills/jigc/SKILL.md` no longer carries the bytes
  jigc wrote, so jigc left it untouched rather than clobber your edits — it is no longer
  version-matched to jigc 1.0.0-rc.15
  route: keep your copy and jigc will keep leaving it alone, or delete
  `.claude/skills/jigc/SKILL.md` and re-run `jigc setup` to reinstall jigc's own copy stamped at 1.0.0-rc.15
EXIT=0

$ cat .claude/skills/jigc/SKILL.md
MY OWN SKILL NOTES
this is the user file            # untouched — 41 bytes, not the 34 217-byte guide
```

The falsified sentence, verbatim from the **installed** copy of the guide
(`.claude/skills/jigc/SKILL.md:71`, i.e. it ships into every adopter repo):
*"…and every `jigc setup` rewrites it, so the guidance in your repo always matches the binary in
your `PATH`."* Driven, the second clause is exactly what the shipped advisory says is no longer
true (*"it is no longer version-matched to jigc 1.0.0-rc.15"*). **The product behaves correctly
and says so at the door** — the refuse-to-clobber is M48's deliberate capability, the ack omits
the guide row, and the advisory names the escape. **One home is wrong: the guide sentence.**
Note the sentence predates M51 (`0c7fb866`, the increment that *built* the refuse-to-clobber),
so this is an old law-1 overclaim the wave's guide walk did not catch, not an M51 regression.
**Severity: LOW** (law 1, a surface that ships into every adopter repo; no bytes at risk —
the honest behaviour is the one the sentence understates).

### CX-3 — CONFIRMED (origin codex) · the installed guide's one install/upgrade command cannot run from the repo the guide is installed into

`lead(codex, "QUICKSTART.md:15-26 declares `cargo install --path crates/cli` as the single install
and upgrade command, but setup installs those bytes into an adopter repo that has no crates/cli")`.

**Driven** — install the guide with a real `jigc setup`, then run the guide's own line from the
repo the guide now lives in:

```
rig=$(dev/jigc-rig bare --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"; cd "$REPO"
$ $JIGC setup                                            # EXIT=0, guide installed
$ head -6 .claude/skills/jigc/SKILL.md
---
description: "How to work in a jigc-managed repository — …"
name: "jigc"
jigc-version: 1.0.0-rc.15
jigc-body-blake3: c25024ac41e10b75896a6daf3ad91275f60c5f048c35cb35a856a8490e5b8c06
---
$ sed -n '26,30p' .claude/skills/jigc/SKILL.md
`jigc` ships as a single binary, and there is **one** install command. This is
the only place this guide states it, so an upgrade is the same line again:
```sh
cargo install --path crates/cli
```
$ ls crates
ls: crates: No such file or directory

$ cargo install --path crates/cli
error: `/private/var/folders/…/jigc-rig-bare-EIwt6y/repo/crates/cli` is not a directory.
       --path must point to a directory containing a Cargo.toml file.
EXIT=101
```

**This does not contradict the driver's row 20** — the two are complementary. Row 20 drove the
same line **from the jigc source tree** (`cargo install --path crates/cli --root <tmp>` → exit 0,
`Installed package 'cli v1.0.0-rc.15'`), which is the only place it can work; CX-3 drives it from
the *adopter* repo, which is where `setup` puts the sentence. The generated preamble narrows the
gap without closing it: it scopes its disclaimer to **documents** (*"every other jigc document
named below lives in the jigc project's own repository, not in this one"*), never to the source
path the install command takes, and the sentence markets itself as *"the only place this guide
states it, so an upgrade is the same line again"* — i.e. an adopter re-reading it after an upgrade
is told to run precisely this, from precisely here. **Severity: LOW** (prose/context; nothing is
destroyed and cargo's own error is clear). It is the one Codex claim the driver's own §4 item 8
brushes past — the driver deliberately installed into a `mktemp -d` root *from the source tree*,
so the adopter-context half was never in its frame.

## B · Driver defects (re-driven here)

### D-1 — STANDS · CONFIRMED (origin driver) — re-driven verbatim

The source pass is **silent** on D-1 (it read `task finalize --help` against `whats_left_coverage()`
and found it consistent — a different cell), so nothing to refute. Re-driven end to end on a fresh
`committed-singletons` rig:

```
$ $JIGC doc rename adr:use-redis-caching --to "Use Memcached caching" --task record-the-cache-choice
adr:use-memcached-caching (renamed to "Use Memcached caching" from adr:use-redis-caching)
EXIT=0                                   # producer, agent text: silent

$ $JIGC task validate record-the-cache-choice | grep -c stale-title
1                                        # re-raise #1 PRESENT (text)

$ $JIGC task finalize record-the-cache-choice --dry-run
finalize --dry-run — pre-commit manifest (nothing committed)
would commit — docs: adopt Use Redis caching for the API
  promoted docs/decisions/use-memcached-caching.md
EXIT=0                                   # re-raise #2 ABSENT (text) — F-9's exact repro

$ $JIGC task finalize record-the-cache-choice --dry-run --format json | grep -c stale-title
3                                        # present on the JSON arm of the SAME call

$ $JIGC task finalize record-the-cache-choice
advisory · commit-recording.stale-title — the staged commit summary still names `Use Redis caching` …
finalized c3a9a26 — docs: adopt Use Redis caching for the API
                                         # the real door re-raises in text; the subject lands stale
```

Identical to the driver's block (different shas, same shape). **Severity: MEDIUM**, as filed.

### D-2 — STANDS · CONFIRMED (origin driver) — re-driven verbatim

The source pass is **silent** on D-2 (it spot-checked `about`/`long_about` strings and reported no
finding at `task discard`), so again nothing to refute — and the silence is itself informative: a
source read that ranks *strings that state a count, a default, a gate subject, an exit code or a
"never"* cannot see an **omission**, which is exactly D-2's declared shape.

```
$ $JIGC task discard --help
Abandon the task — remove its working area `.jigc/tasks/<id>/`

Usage: jigc task discard [OPTIONS] <ID>
…
      --force  Remove the working area even when it holds staged docs no commit has a copy of …
EXIT=0                                   # the whole `about` — no mention of a commit anywhere

$ $JIGC milestone create "M1 the first" && $JIGC milestone add-task m1-the-first "do the thing"
$ git rev-parse --short HEAD
bd623d7
$ $JIGC task discard do-the-thing
discarded task do-the-thing
record commit: c90a774   — this sub-task's milestone record, settled to `discarded` and committed on its own
EXIT=0
$ git rev-parse --short HEAD
c90a774                                  # HEAD moved: the door is COMMITTING_DOORS row 10

$ $JIGC milestone discard --help | head -1
Abandon the milestone: … in one record-only commit, then tear the workbench down …
                                         # the sibling destroying door DOES say it
```

**Severity: LOW**, as filed, with the driver's honest bound kept: it is an omission, not a false
predicate, and EC-12's declared axis was *"every adopter-facing sentence that says a verb does not
commit"*.

## C · Driver observations — status after reconciliation

All five were driven by the driver and none is contradicted by the source pass; they are carried
unchanged. Two touch a Codex claim and are cross-referenced rather than merged:

- **O-1** (`setup`'s *"every path in its own install footprint"* universal, with
  `.git/hooks/pre-commit` the declared exception) is the **same shape as CX-1** — an unqualified
  universal in a guide/help sentence with one real, self-declared exception and no byte at risk.
- **O-3** (`rename` refuses harder than the carryover paragraph implies) and **O-2**, **O-4**,
  **O-5** are untouched by the source pass. The source pass explicitly read the manifest tag
  vocabulary and the triage keys and *"found no omitted typed list"*, which is consistent with
  rows 10–12 and 31's underlying source claim.

## D · Open leads

**None.** All three Codex claims were driven to a repro block on the installed rc.15 with the rigs
the driver used; no claim required a state no rig builds or an environment unavailable here.

---

# Doors covered

Every clap leaf that is the door of **≥1 driven row** (`VERB_KINDS` spelling,
`crates/cli/src/cli.rs:1669`). Fixture-building verbs (`doc create`, `doc set-slot`,
`doc set-field`, `milestone add-task`, `task list`, `config get`, `describe`, `ingest`) are **not**
counted — a fixture builder is not the door of a row. `cargo install` (driver row 20, CX-3) is not
a jigc leaf and is not counted.

`start` · `workflow` · `setup` · `uninstall` · `upgrade` · `migrate` · `migrate-corpus` ·
`rename` · `validate` · `doc rename` · `doc show` · `doc schema` · `doc list` · `task validate` ·
`task discard` · `task finalize` · `config set` · `milestone create` · `milestone provision` ·
`milestone execute` · `milestone join` · `milestone finalize` · `milestone discard`

**23 doors.** Unchanged by the reconciliation: CX-1 lands at `migrate-corpus` (+ `config set` as
its enabler, both already covered), CX-2 and CX-3 at `setup`, and the two re-driven defects at
`doc rename` / `task validate` / `task finalize` / `task discard` / `milestone create` — every one
already in the driver's set. The demotion of row 31 costs no door (`task finalize` is the door of
rows 5, 27–30, 62, 63, 65).
