## Ranked findings

### Blocking

1. **The lifecycle cannot record why an item closed.** The settled convention permits changes to “status (and resolution fields),” but neither schema has resolution fields ([doctype-proposal.md:10](completions/artifacts/M55/doctype-proposal.md:10), [doctype-proposal.md:20](completions/artifacts/M55/doctype-proposal.md:20), [doctype-proposal.md:33](completions/artifacts/M55/doctype-proposal.md:33)). M57 explicitly needs “declined with reason,” while `duplicate`, `refuted`, `intended`, and `resolved` also require rationale ([gap-list.md:24](completions/artifacts/M55/gap-list.md:24)).  
   **Fix:** add optional `resolution` prose and optional `resolved-date` to both doctypes; add optional self-ref `duplicate-of` to `jigc-feedback`. State that closing status requires resolution by workflow convention. Adding these later is structurally cheap, but omitting them blocks the immediately scheduled consumer and invites status-only information loss.

2. **The proposal leaves the required read surface unresolved.** Per-instance JSON omits the title and `doc list` omits fields, so “all open feedback” requires N+1 calls and markdown parsing ([planning-findings.md:28](completions/artifacts/M55/planning-findings.md:28)); the proposal merely repeats the problem ([doctype-proposal.md:54](completions/artifacts/M55/doctype-proposal.md:54)). That is not a usable triage channel for M57.  
   **Fix:** ship a bulk machine-readable read: `doc list <type> --format json` returning identity, title, and header fields for every instance. Filtering can remain client-side initially. Also add `title` to per-instance `doc show` JSON. Do this before freezing consumers onto N+1 scraping.

### Important

3. **`tier` freezes an already-colliding name without defining its semantics.** The repository has three meanings for “tier” ([gap-list.md:14](completions/artifacts/M55/gap-list.md:14)); the proposal supplies only “our review predicate” ([doctype-proposal.md:26](completions/artifacts/M55/doctype-proposal.md:26)). Renaming a field later is effectively remove+add, and removal is refused ([corpus-migration.md:211](design/corpus-migration.md:211)).  
   **Fix:** rename it now to `consequence` or `review-tier`, use quoted enum values `tier-1`, `tier-2`, `tier-3`, and define each predicate in the schema hint/design.

4. **`fixed` is not a coherent terminal state for all three feedback kinds.** “Feedback fixed” and “inconvenience fixed” blur outcome and implementation; `resolved` is already the generic inconsistency vocabulary ([doctype-proposal.md:22](completions/artifacts/M55/doctype-proposal.md:22), [doctype-proposal.md:36](completions/artifacts/M55/doctype-proposal.md:36)). Enum widening is cheap, but replacing a value needs an authored semantic map.  
   **Fix:** use `open · resolved · declined · duplicate · refuted`; put “fixed/improved/documented/no-change” in `resolution`.

5. **Defaulted `status` is unsafe for the exact query this design needs.** A deleted defaulted field disappears from JSON and silently evades `status == open` ([planning-findings.md:20](completions/artifacts/M55/planning-findings.md:20)).  
   **Fix:** reporting workflows must explicitly write `status: open`; do not depend on default materialization. Repair F10 or have the bulk read project the effective default and flag missing stored required fields.

6. **Required `version` overconstrains non-runtime feedback.** `feedback` can concern documentation, workflow wording, or a design limitation and have no meaningful observed binary version ([doctype-proposal.md:22](completions/artifacts/M55/doctype-proposal.md:22)).  
   **Fix:** make `version` optional now. Required→optional is migratable, but forcing invented values degrades the initial corpus.

7. **`left`/`right` have no declared orientation.** For `code-doc`, consumers cannot know whether `left` must be code; for `doc-doc`, ordering should be explicitly immaterial ([doctype-proposal.md:35](completions/artifacts/M55/doctype-proposal.md:35)).  
   **Fix:** retain the generic strings, but define: `code-doc` means `left=code`, `right=document`; `doc-doc` ordering is arbitrary. Renaming these typed fields later would be a refused removal.

### Minor

8. **The design must explicitly distinguish this channel from backlog.** The map excludes TODO/backlog because work belongs in milestones/roadmap ([doctype-map.md:99](implementation/doctype-map.md:99)).  
   **Fix:** state that an open finding records observed evidence, not scheduled work; accepted work must be promoted to the roadmap/task system, with the finding retaining only its resolution.

## OPEN reference recommendation

Use **subject**, not discovery context.

- Field: `subject`
- Type: required `string`
- On: `jigc-feedback` only
- Meaning: the stable product surface that owns triage, with a documented address grammar such as `command:task-finalize`, `workflow:report-jigc-feedback`, `doctype:vision`, `component:reconciliation`, or `path:…`.

A managed `ref` is the wrong abstraction: it targets one doctype ([document-type-schema.md:73](design/document-type-schema.md:73)), while the real subjects include commands, workflows, components, and paths. Even a new polymorphic document ref would not cover those. Discovery context is provenance, not ownership; add optional `found-in` later if a consumer emerges.

`inconsistency` does **not** need `subject`: required `left` and `right` already identify where it belongs. A third locator would duplicate them.

## Field changes and one-way-door judgment

- **Add now:** `subject`, `resolution`, `resolved-date`, `duplicate-of`; rename `tier` before freeze.
- **Remove:** `door`; it is a lossy duplicate of required `subject`. Keeping both creates two homes for the same routing fact.
- **Do not add:** `trigger`; it belongs to deferral scheduling—the established ledger models it as such ([deferral-ledger.yaml:38](crates/cli/packs/methodology/schemas/deferral-ledger.yaml:38))—and would turn findings into the forbidden backlog.
- **Keep:** `pinned-by` as the settled string grammar; the pinning design explicitly treats it as an audit-enforced test name or `UNPINNED` reason, not a symbol parser ([pinning.md:51](implementation/pinning.md:51)).