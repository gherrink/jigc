# M39 planning gate-record

The plan-time forcing function ([methodology-docs.md](../../../design/methodology-docs.md) → The planning gate-record), filled **by hand** before decompose. One row per thing M39 builds; every cell carries evidence or `N/A` + why. An empty/hand-wavy cell is a halt. Filled 2026-07-06 against the verified ledgers (4 capability-auditors + 4 gap-detectors, exercised at HEAD `f372b2b`), the independent design-review (8 findings baked), and 2 robust-advocate briefs.

## Per-scope-item gates

Gates: **PA** prior-art-reconciled · **DC** design-complete (behaviour + scope + acceptance concrete) · **RE** reuse-exercised (claims spiked, not analogised) · **CS** check-scope-pinned · **IS** integration-seam named.

### 1 · doc-read-surface — `jigc doc show <ref>` + `read_slice` extension + stable `--format json`
- **PA:** `introspection.md:24` already forward-refs `doc show` (made real); `write-commands.md` is write-side by charter, `introspection.md` excludes filled-prose reads → **home = a read-surface part-doc** (decided). VISION:27/CLAUDE:34 "reads *and* writes" is honest only for compose-slicing today → doc show closes the doc-vs-reality gap. Determinism boundary does **not** foreclose (a deterministic read projection is CLI-owned). Reconciled.
- **DC:** whole-doc + `#section` + item/leaf slice via the canonical render path; the general `--format json` shape pinned in the read-surface doc (milestone-record is its witness, [team-ready-state.md](../../../design/team-ready-state.md)). Acceptance: a committed `vision` round-trips through `doc show`; an item slice returns the item; json stable. Concrete.
- **RE:** SPIKED — `store::read_slice` (store.rs:90) is section-scoped only; whole-doc (store.rs:110) + item/leaf (store.rs:198) **block** today; extension isolated from the write path (2 non-test callers; writer path separate) → cannot regress the writer.
- **CS:** N/A — a read verb introduces no validation finding.
- **IS:** new `DocCommand::Show` (doc.rs) → extended `read_slice` (store.rs) → canonical render (committed docs are byte-copies of canonical staged, task.rs:1417).

### 2 · compose renders ALL grounded sources (multi-target `walk_edge`) [G2]
- **PA:** `design-altitude-doctypes.md:175-180` documented first-only as *intended* → **revised** (basis "in session memory" falsified by the trial for a fresh session); `doc-read-surface.md:14` called it a bug. Reconciled by the revise + flipping the two green tests.
- **DC:** `walk_edge` returns all terminal-hop targets; `emit_content` renders each source as its own labelled blockquote (separator pinned). Acceptance: a vision grounded in 2 research docs re-composes both `#findings`; flip `flow_form_vision.rs:353` + `flow_design_altitude.rs:389`.
- **RE:** SPIKED — `walk_edge` (index.rs:343) `.find()` = first-only reproduced; blast radius 1 trait/1 impl/1 caller; validation `ref_resolves` (index.rs:368) iterates all edges (untouched).
- **CS:** N/A — validation already all-edge; render-only change.
- **IS:** `ContentStore::walk_edge` (compose.rs:687) → `deref_content` (compose.rs:778) → `emit_content` (compose.rs:740).

### 3 · the `milestone-record` doctype (write + read-back + commit model)
- **PA:** [team-ready-state.md](../../../design/team-ready-state.md) (new design of record); `methodology-docs.md` composes-alone bound honored (no cross-pack ref); `doctype-map.md:51` "work-units are not doctypes" reconciled (governs *identity*, not *emitted state* — the `completion-record` precedent); `storage.md` source-of-truth invariant revised in the *strengthening* direction. Review confirmed the reconciliation holds.
- **DC:** two engine capabilities (write `set: on-transition`; read-back = the committed `.md` is engine authority, JSON→cache) + commit model (path-scoped, join fold-in) + name/address/home + pinned json shape + no-silent-overwrite (reconcile-preflight conflict-block). All concrete post-review; acceptance section written. **DC met only after the review** (F1 read-back was the missing half).
- **RE:** SPIKED — current shape (base.json/tasks.json, milestone_dir, per-task intent/status) confirmed; `set: on-transition`, the `join` in-place-mutate arm, and read-back all flagged **net-new** (not blind reuse of `add-item`/`on-create`).
- **CS:** OOB drift → reconcile-preflight **conflict-block** at each op; scope = the record file, at create/add-task/join. Store-scope `validate` also covers its conformance. Pinned.
- **IS:** `engine::milestone.rs` (read_base_pin/read_task_list/join/add_task → cache re-derived from record); the M22 `on_create` seam → `on_transition`; `milestone_dir` record/WIP split; the 3→1 gitignore; the item-leaf splice → direct-committed write (net-new plumbing, flagged F5).

### 4 · team-ready `.jigc` layout split (record/WIP home split + gitignore consolidation)
- **PA:** `storage.md` (.jigc layout + invariant, revised); `reconciliation.md` (committed-record OOB reconcile). Reconciled.
- **DC:** field-granular split table + the flat `milestone-records/` home ([team-ready-state.md](../../../design/team-ready-state.md)); gitignore 3→1. Acceptance folded into item 3's fresh-clone arm.
- **RE:** SPIKED — 3 divergent hardcoded gitignore constants (adapter.rs:998, task.rs:1654, milestone.rs:1022, drifted on `worktrees/`); milestone_dir roots record+WIP together.
- **CS:** N/A — layout, no new check.
- **IS:** the 3 gitignore sites + every `<jigc_root>/milestones/<id>/` consumer.

### 5 · freeze-exempt relocation floor
- **PA:** `validation.md:279` ("the only stable discriminator is the location directory") **scoped/qualified** (blind to placement/root — must add a second discriminator); `reconciliation.md` owns the route/resolution arm; `corpus-migration.md` (parallel freeze-exempt path vs. the frozen-gated M38 transform); `storage.md:148` (the owed obligation). The move-INTO vs. adopt-in-place collision reconciled (foreign-only for M39). Contradictions flagged → revised in the build.
- **DC:** orphan re-key on recorded-prior-home / found-stranded-doc; freeze-exempt has no snapshot → from-path human-supplied/confirmed (auto-move unsafe); rename-move extraction; parallel freeze-exempt path; `config set docs-root` → detect+route+offer-move. **F8 scope stated honestly:** M39 closes *detection* (generalized orphan at validate + relocation/upgrade + config-set) + *routed resolution* (human-confirmed target); the bare-schema-edit strand is detected at the next validate/pack-load, resolution human-confirmed. Acceptance: relocate a freeze-exempt doctype → detected + routed (not silently stranded); `config set docs-root` → offers the move.
- **RE:** SPIKED — orphan detector basename-keyed/blind (orphan.rs:74); rename move entangled (rename.rs:271 → extraction net-new); M38 transform frozen-gated (migrate_corpus.rs:140 → parallel path); no prior-home snapshot for freeze-exempt.
- **CS:** the generalized orphan finding — scope = committed store; fires at `validate` (report-only, existing severity) + relocation/upgrade + `config set` (detect+route). Pinned.
- **IS:** `orphan.rs` (re-key), `config.rs` `warn_if_docs_root_repoint_orphans` (→ route), `rename.rs:271` (extract standalone move primitive), `migrate_corpus.rs` (parallel freeze-exempt path).

### 6 · invocation-log output-size (fd-level tee)
- **PA:** `measurement.md:62,70` (record schema + "stderr-text capture — lean NO" → clarify byte-*counts* ≠ text-capture). Reconciled.
- **DC:** an fd-level counting tee at `main()` wrapping fd 1/2 at the parse+dispatch boundary; `Record` + `output_bytes`. Acceptance: recorded size == emitted bytes for a known-output verb; `--help`/`--version` counted (the tally can't reach them).
- **RE:** SPIKED — no output choke point, 122 print sites, `main()` gets only `Outcome{code, finding_codes}` (auditor + advocate); the tee is one-file, smaller than the cheap tally.
- **CS:** N/A — instrumentation, no check.
- **IS:** `main.rs` (the boundary the timing `Instant` already wraps) + `invocation_log.rs` `Record`. New dep (`libc`/`os_pipe`), Unix-only — conscious, jigc's target.

### 7 · slug word-cap + `--slug` override
- **PA:** `structural-grammar.md:127` (minting-mechanics open — currently framed case/charset, update to *length*); `write-commands.md:41` (`--slug` **already** on `jigc rename` → reuse the flag, not a new `--id`). Reconciled.
- **DC:** word-count cap (~5); `--slug` on `start`/`doc create`, validated via `is_slug`/`slugify`, routed through the reject/resume collision. Acceptance: long intent → ≤5-word slug; `--slug` accepted; `--slug` collision rejects.
- **RE:** SPIKED — `slugify` (slug.rs:72) caps 50 *chars* at a word boundary; the RC slugs reproduced (premise **corrected** — not mid-word); `--slug` on rename exists (cli.rs:189); golden churn contained to `slug.rs`.
- **CS:** `--slug` collision routing (reject/resume, state.rs:430) pinned.
- **IS:** `slug.rs` (cap), `cli.rs` StartCommand + `doc.rs` DocCommand::Create (`--slug`), `state.rs` mint collision.

### 8 · form-vision empty-research advisory (rider)
- **PA:** `form-vision-research-routing.md` (direction pre-settled route-not-merge/advisory); `design-altitude-doctypes.md:80` (`grounded-in` `0..*` — zero research is legal). Reconciled, no conflict.
- **DC:** CLI-side advisory — query committed `research` count → print the route line at the `author-vision` opening step; advisory, not blocking. Acceptance: empty research store → the line prints on `form-vision` compose; non-empty → silent.
- **RE:** SPIKED — no state-aware-advisory-at-compose precedent; `committed_slugs` (index.rs:733) counts but is unwired to compose → CLI-side print chosen (state-aware-compose primitive stays stretch).
- **CS:** N/A — an advisory print, not a validation finding.
- **IS:** the `form-vision` compose path (CLI-side) + `committed_slugs`.

## Milestone-level gates

- **strategic-claim-fresh:** the pre-1.0 admission test (one-way doors + known holes in declared surfaces; general value → post-1.0) is fresh — agreed with the human 2026-07-06, trial-driven; every core item maps to a door/hole (doc-read = known hole in the "reads" contract; team-state = the committed-layout one-way door; floor = an owed known-hole; log = the trial's measurement prerequisite; slug = permanent-bad-ID one-way door).
- **cheap-vs-robust:** 2 forks ran **robust-advocates** — G1 record-shape → (c) managed doctype (preserves the source-of-truth invariant); G5 log → fd-tee (smaller *and* correct). Both settled robust with evidence, not self-framed.
- **foreclosed-by-doc:** 3 foreclosing docs engaged, not obeyed on faith — `design-altitude-doctypes.md §3` (revised, basis changed), `doctype-map.md:51` (reconciled identity-vs-emitted-state), `validation.md:279` (scoped).
- **census:** enumerated by the auditors — the M38 9-site placement census, the `milestone_dir` consumers, the 3 gitignore constants, the 122 print sites, the `walk_edge` 1-trait/1-impl/1-caller blast radius. No silent cap.
- **acceptance-spiked:** current-behavior claims **exercised** (not inferred) by the auditors/gap-detectors — the card-1 render bug reproduced, the silent-strand confirmed, the `read_slice` blocks, `--slug`-on-rename, the milestone record shape, the actual minted slugs, `grounded-in` `0..*`. The *new* behaviors are what M39 builds (nothing to spike pre-build).
- **value-flow-exercised:** the RC greenfield trial is the live value witness — F1's `doc show` hole (task [50], the vision-revision), A6's lost-on-clone set measured (tasks/milestones), A7 the log-size driver.

## Verdict

**No halts.** Every cell carries evidence. The one gate that was *not* met at first-write — item 3's **design-complete** (the read-back half, F1) — was surfaced by the independent review and baked before this record was filled, which is the gate-record working as intended. Ready to decompose.
