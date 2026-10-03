# M55 baseline capability ledger — verified at 62c76009 (2026-10-02)

Four capability-auditors, driven against target/debug/jigc on dev/jigc-rig repos. A map, not gospel.

## Done-picture (confirmed by the human)
- Methodology-pack findings doctype (working fields tier · door · repro · status · pinned-by) + code-less `report-finding` workflow landing a record-only commit; code↔doc drift coverage (kind vs second doctype); seeded with the recorded 1.x rows.
- Proves: report + read back through the real binary — acceptance suite end-to-end + rig probe with two reporters against one store.
- Hard constraint: NO new engine mechanism (a fork needing one is a halt).
- Release: rc.23 (rc.22 already published at M54); release PR #2 merges at M55's release (human). README crates.io link fork owed before that merge.
- Out of M55: project-history.md fate (stays keyed to M56). Gate-speed work runs BEFORE the build as a separate work/ PR, not an increment.
- Fork order the human set: (1) singleton vs doc-per-finding, (2) drift as kind vs second doctype, (3) router visibility, then record-only meaning, append-only, pinned-by type. Also on the agenda: planning-record gate-row ("every fork lists the option that removes the drifting artifact" — a planning-record schema change → version bump + corpus migration).

## Built and proven
- New methodology doctype = schemas/<ty>.yaml + manifest entry at v1 (no snapshot); existing corpus validates clean after adding it. Pack-load refuses a schema absent from the manifest and prints the hash to pin.
- Engine-native field types: enum, string, date, bool, int, ref, owned-location.
- code-anchor in a methodology schema is refused at pack-load today (no methodology field-types.yaml). Copying dev's field-types.yaml into the methodology pack makes it work (doc-code.symbol-exists fires, pack-agnostic) — a pack-surface addition, not engine mechanism. BUT a pack-declared type can never be author-required (schema.rs:595-597) — pinned-by could be left empty.
- Code-less workflow (park-idea) lands a doc-only commit end-to-end; drove a prototype finding doctype + report-finding workflow end-to-end (methodology pack alone via JIGC_PACK_DIR). Workflow bodies may contain only `{{ include: step:… }}` lines.
- Code-less task appends an item to an existing committed singleton (decisions-log) and finalizes; `doc author` batch payload appends N items in one task (needs the type in allows-create even over a committed doc); duplicate item rejects the whole payload.
- Read-back: doc show whole doc / one item / one field (JSON); fenced code blocks with `#` lines inside item prose survive byte-for-byte; doc schema JSON lists item fields; doc list filters by doctype only.
- Validation: enum checked at write; finalize blocks on incomplete items (required field/slot); store-scope validate is report-only (exit 0) and catches hand edits.
- Fan-out join: two sub-tasks creating colliding new instances → deterministic -2 suffix ordered by task id (suffixed doc keeps identical H1 title). Two sub-tasks editing one committed singleton → blocking join.same-doc-clash; recovery = discard one sub-task, its entry is lost and must be redone.
- Router: catalog = creates-task && selectable workflows; methodology workflows listed for every adopter; SessionStart hook prints the catalog every session. Pack-level hide = `selectable: false` + required `suppressed: {reason, expires[, door]}` — hidden for ALL adopters, still callable by name (unless `door`). Methodology pack composed for every adopter by `jigc setup`; hand opt-out is silently reverted by the next setup.
- Bulk adoption: `jigc ingest` adopts hand-written conformant files (registers only; files stay untracked, bytes never pass jigc's writer).

## Shape-limited
- One created doc instance per doctype per task (`write.identity-change`) → doc-per-finding seeding = N tasks/N finalizes; singleton seeding = one task.
- id-from: title slugs to ~4 significant words; a create whose slug matches a committed instance silently becomes "copied in for update" (S1: driven in fan-out, a second reporter's "new" finding overwrote the first's description, no finding raised). Suffixing exists only between two created instances at a fan-out join. --slug exists but nothing forces it.
- No server-side filter: "all open findings" = singleton: 1 doc show + client filter; doc-per-finding: doc list + N doc show (doc list carries no field values).
- Item ids slug from titles with stop-words dropped.
- A fan-out sub-task running a workflow that ends in a finalize step is told to run `jigc task finalize`, which is refused (`finalize.milestone-sub-task`).
- Default `sub-task` workflow allows only adr; methodology docs in a fan-out need add-task --workflow <methodology wf>.
- Project-layer workflow shadow (`.jigc/config/workflows/<id>.yaml`) can hide a workflow per project (tested for a dev-pack workflow only); no CLI writes it, whole-file copy with no recorded base → silently stale on upgrade.

## Not built / convention only
- "Record-only commit": finalize commits anything staged during the task (code staged after start rides along; only pre-start staged is refused `finalize.carried-staged`). A doc-less finalize is `finalize.empty-commit`.
- Append-only: no knob; any task can add-item / remove-item / set-field on any committed doc regardless of its workflow's allows-create (only `doc create` and `doc author` are gated).
- No edit-gate on existing committed docs.
- No field-value filtering in doc list / doc show.

## Latent defects found by the baseline (NEW, driven)
- L1. After a pull/merge that changes a committed doc, the next task that edits that doc is falsely blocked `reconciliation.conflict-block` — start's absorb is in-memory only; persisted only at a landed finalize. Workarounds: `jigc ingest`, or any unrelated landed finalize. The route names neither. Every singleton reporter hits this after every pull.
- L2. Branch switch in one checkout: file-state.json has no branch/HEAD stamp (recorded bound) → singleton false conflict-block; for new instances, store-scope `jigc validate` exits 1 with `reconciliation.rename … missing` — store scope lacks the M45 no-history downgrade that task scope applies. Hits the branch-per-milestone model directly.
- L3. User-created git worktree: after one report lands in the worktree, doc show there gives store.not-found and every later finalize exits 3 (`reconciliation.rename … missing`); main checkout's validate exits 1.
- S3. Parallel branches, singleton: git content conflict; a naive union resolve silently dropped an item's date field and `jigc validate` exited 0 without flagging it.

## Sequential / concurrent outside fan-out
- Two open tasks appending to one singleton: first lands, second `finalize.base-mismatch` (exit 3), only route discard+redo. Same for two new instances with the same slug (no suffix outside join). Distinct-slug new instances: second re-pins and lands.
- Two concurrent finalizes from two worktrees: both cache entries survive (one clean pair; L3 prevented re-runs).

## Seed sources
- rc.16 wave tier-2/3: 23 rows at completions/artifacts/M52/per-axis-review/README.md 555–709 (8 tier-2) and 710–931 (15 tier-3); summary at decisions-pending.md:102 names only the 8 tier-2. Tier only by heading; severity missing on 10; repro block missing on 8; door prose only; no status, no pinned-by. At least 3 already fixed (6,D-1), (7,A7-F3), (4,DEFECT 1) in the 2026-09-23 usability batch (DECISIONS.md:998–1010) but still counted as seed.
- M53 Settle's six: decisions-pending.md:446–460, rows (a)–(f); tiered together tier-2/3 at 448; per-row trigger; (d),(e) READ-not-driven; prose evidence, no repro block.
- "Rows added on 2026-09-27": no such phrase. Candidates: decisions-pending.md:403–407 (blind trial; two dev/ tooling rows), CI block rows 80, 83, 84 (2026-09-27), drift-list row ~65 (M56).
- Wider candidates not in the charter: M53's four re-reviews (completions/artifacts/M53/per-axis-review{,-rc18,-rc19,-rc20}/README.md) each send tier-2/3 rows "into the ledger for 1.x"; some recorded closed; not counted or reconciled.
- Plus the latent defects above (L1–L3, S1, S3, S2, C3) are themselves candidate first findings.

## Registration fences that go red on a new methodology doctype (hand-listed)
pack.rs ~4010 methodology_schema_manifest_matches_the_frozen_doctype_set; roundtrip_registry_fence.rs:79; doc_read_surface.rs:63/155/825; doctype_map_versions.rs (row in doctype-map.md); count_fences.rs:725 ("eleven" in corpus-migration.md); unfenced "eleven" prose in doctype-map.md:42, doctype-authoring.md:22, manifest header. Missing from doctype-authoring.md's checklist.
