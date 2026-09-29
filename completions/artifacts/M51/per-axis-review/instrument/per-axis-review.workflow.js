export const meta = {
  name: 'm51-per-axis-review',
  description: 'M51 acceptance: eight per-axis reviews on the installed 1.0.0-rc.15 — Opus driver, then reconciliation against the Codex source pass, then one assembler',
  phases: [
    { title: 'Drive' },
    { title: 'Reconcile' },
    { title: 'Assemble' },
  ],
}

const SCRATCH = '/tmp/claude-501/-Users-maurice-projects-gherrink-jigc/277a6565-6113-4f40-9f0a-b420630ff1b6/scratchpad/axis-review'
const BIN = '/Users/maurice/.local/bin/jigc'
const AXES = [
  { n: 1, name: 'caller tokens' },
  { n: 2, name: 'posture' },
  { n: 3, name: 'destroying doors' },
  { n: 4, name: 'transaction/rollback' },
  { n: 5, name: 'pinned contracts' },
  { n: 6, name: 'composed surfaces' },
  { n: 7, name: 'freeze & migration' },
  { n: 8, name: 'adopter docs & help' },
]

const DRIVE_SCHEMA = {
  type: 'object',
  required: ['file', 'rows_driven', 'rows_not_applicable', 'findings', 'notes'],
  properties: {
    file: { type: 'string' },
    rows_driven: { type: 'number' },
    rows_not_applicable: { type: 'number' },
    findings: { type: 'array', items: { type: 'object', required: ['title', 'door', 'cell', 'exit', 'code', 'summary'], properties: { title: { type: 'string' }, door: { type: 'string' }, cell: { type: 'string' }, exit: { type: 'string' }, code: { type: 'string' }, summary: { type: 'string' } } } },
    doors_covered: { type: 'array', items: { type: 'string' }, description: 'every clap leaf (VERB_KINDS spelling, e.g. "doc show", "task finalize", "config set") that is the door of at least one DRIVEN row' },
    notes: { type: 'string' },
  },
}
const RECON_SCHEMA = {
  type: 'object',
  required: ['file', 'confirmed', 'refuted', 'leads_open', 'notes'],
  properties: {
    file: { type: 'string' },
    confirmed: { type: 'array', items: { type: 'object', required: ['title', 'origin', 'summary'], properties: { title: { type: 'string' }, origin: { type: 'string', description: 'driver | codex' }, summary: { type: 'string' } } } },
    refuted: { type: 'array', items: { type: 'object', required: ['title', 'origin', 'datum'], properties: { title: { type: 'string' }, origin: { type: 'string' }, datum: { type: 'string' } } } },
    leads_open: { type: 'array', items: { type: 'string' } },
    doors_covered: { type: 'array', items: { type: 'string' } },
    notes: { type: 'string' },
  },
}
const ASSEMBLE_SCHEMA = {
  type: 'object',
  required: ['files', 'coverage_ok', 'summary'],
  properties: { files: { type: 'array', items: { type: 'string' } }, coverage_ok: { type: 'boolean' }, summary: { type: 'string' } },
}

function drivePrompt(a) {
  return [
    'M51 per-axis review — AXIS ' + a.n + ' · ' + a.name + ' — the OPUS DRIVER. You own the (door, cell) table for this axis and you may not mark a row driven from a source read.',
    '',
    'READ FIRST: completions/artifacts/M51/acceptance-design.md (Part 2 — your axis row gives the door set and the cell set, and the "Row schema"; Part 1 names the flow-52 arm that already pins part of this axis, so you know what the review ADDS over the suite) and completions/artifacts/M51/VERDICT.md (the audit fixes that landed after the build: routes 6c2391c0, orphan territory da5173a1, setup guard ff2bde99, LOWs 507c332d — your table must reflect the FIXED binary).',
    'BINARY: the installed `' + BIN + '` — assert `jigc --version` prints `jigc 1.0.0-rc.15` before anything else and STOP if it does not. This is the RELEASE posture: route-fence panics do not exist here.',
    'RIGS: `rig=$(dev/jigc-rig <state> --binary ' + BIN + ') || exit; eval "$rig"` — two-step eval, never `eval "$(…)"`, never `rm -rf` a variable path; states: fresh · committed-singletons · migrated · chatty-hooks · vendored · bare (`dev/jigc-rig --help`). `--pack-from-dev`/`--schema`/`--workflow` manufacture a throwaway pack (axis 7 needs one). Build each fixture you need with the binary, never by writing into `.jigc/`.',
    'DERIVE THE DOOR SET FROM THE CODE, not from the design doc\'s numbers: read the registries the design names (crates/cli/src/cli.rs PATH_ARG_OCCURRENCES / DOCTYPE_DOORS / SLUG_DOORS / WORK_UNIT_ID_DOORS / BEHALF_DOORS / VERB_KINDS; crates/cli/src/invocation_log.rs COMMITTING_DOORS; crates/cli/src/milestone.rs DESTROYING_DOORS; crates/cli/src/render.rs ENVELOPE_ARMS / STORE_EXIT_FLIPS / ManifestKind; crates/engine/src/schema_diff.rs SchemaChangeKind) and state the count you read.',
    'DRIVE every applicable (door, cell) row and record per row: argv driven · exit · code|none · route kind (Mechanical/Human/Informational/none) · surface asserted (the line or JSON key) · verdict (matches contract / DEFECT / n/a with reason). A row is driven iff its argv ran on the installed binary and its verdict is recorded with a repro block (setup · argv · observed exit/output). Group repeated drives into one repro block per cell where the argv only differs by door. Where the cell set × door set is large, drive every door for at least the cells that discriminate (the escape shapes, the refusals) and state plainly which (door, cell) pairs you did NOT drive and why — never present an un-driven pair as driven. Do NOT read the Codex source pass for this axis (it exists; reconciliation is a separate agent).',
    'DEFECT = the binary contradicts a stated contract (a design doc rule, the finding\'s own route, a help text, the acceptance design\'s cell expectation). Record every defect with its repro block. Use `--format json` wherever the axis is about the envelope.',
    'WRITE the full table + repro blocks + the un-driven list + a "what this adds over flow-52 arm N" paragraph to ' + SCRATCH + '/driver/axis' + a.n + '.md (create the directory). Return: the file path, the counts, every defect as a finding, `doors_covered` = every clap leaf that is the door of at least one DRIVEN row (VERB_KINDS spelling), and notes. No fixes, no commits, no repo edits.',
  ].join('\n')
}

function reconPrompt(a, drive) {
  const f = drive && drive.file ? drive.file : (SCRATCH + '/driver/axis' + a.n + '.md')
  return [
    'M51 per-axis review — AXIS ' + a.n + ' · ' + a.name + ' — RECONCILIATION between the Opus driver\'s table and the Codex source pass.',
    '',
    'INPUTS: the driver table ' + f + ' (driven rows + repro blocks + defects) and the Codex source pass ' + SCRATCH + '/codex/axis' + a.n + '-codex.md (claims as leads; its prompt is beside it as axis' + a.n + '-prompt.md). Binary: `' + BIN + '` (assert `jigc 1.0.0-rc.15`). Rigs as the driver used them (`rig=$(dev/jigc-rig <state> --binary ' + BIN + ') || exit; eval "$rig"`).',
    'THE RULE (acceptance-design.md → The reconciliation rule): a claim by one that the other cannot reproduce is a LEAD, not a finding. For EVERY Codex claim: enter it as `lead(codex, <claim>)`, then DRIVE it — to a repro block (then it is a CONFIRMED finding, origin codex) or to a recorded refutation with the falsifying datum (the driven output that contradicts it). For every driver DEFECT the source pass says cannot happen (or is silent on): it stays a finding (it was driven) — re-drive it once yourself to confirm the repro block, and record the source claim as refuted with that repro if the pass contradicted it. A Codex claim you genuinely cannot drive (needs a state no rig builds, or an environment you lack) stays an OPEN lead with the reason — never promote it on the source read, never drop it.',
    'Also check the driver table for rows marked driven that carry no repro block — such a row is NOT driven; demote it and say so.',
    'WRITE the reconciled axis file to ' + SCRATCH + '/reconciled/axis' + a.n + '.md: the driver table (unchanged except demotions), then a "Reconciliation ledger" section: each Codex claim → CONFIRMED (repro) | REFUTED (datum) | OPEN LEAD (reason); each driver defect → its status; then a "Doors covered" list (every clap leaf that is the door of ≥1 driven row, VERB_KINDS spelling). Return the file path, the confirmed/refuted/open lists, doors_covered, and notes. No fixes, no commits, no repo edits.',
  ].join('\n')
}

function assemblePrompt(results) {
  const summary = results.map((r, i) => 'axis ' + (i + 1) + ': ' + (r ? (r.file + ' — confirmed ' + (r.confirmed || []).length + ', refuted ' + (r.refuted || []).length + ', open ' + (r.leads_open || []).length) : 'NO RESULT (agent died)')).join('\n')
  return [
    'M51 per-axis review — ASSEMBLE the persisted record. The eight reconciled axis files are in ' + SCRATCH + '/reconciled/axis{1..8}.md (drivers in ' + SCRATCH + '/driver/, Codex passes in ' + SCRATCH + '/codex/). Per-axis results:',
    summary,
    '',
    'WRITE, under completions/artifacts/M51/per-axis-review/ (create it): README.md (what this is — the acceptance instrument of D15/§17/§19, run on the installed `jigc 1.0.0-rc.15` from commit 577a0099 on 2026-09-16; the staffing — one Opus driver + one Codex source pass per axis + one reconciler; the reconciliation rule; a FINDINGS section listing every CONFIRMED finding across all axes with its repro block, every REFUTED claim with its datum, every OPEN lead with its reason; the honest bounds — anything an axis states it did not drive; and the COVERAGE table), axis-1.md … axis-8.md (the reconciled files, copied verbatim with a one-line header naming the binary and date), and codex/axis-N-source-pass.md (the raw Codex outputs, verbatim, so the leads are auditable).',
    'THE COVERAGE TABLE (acceptance-design.md → Coverage — all 47 VERB_KINDS leaves; §17\'s fence strengthened): read the 47 leaves from crates/cli/src/cli.rs VERB_KINDS (state the count you read — if it is not 47, say so and use the real set), then for each leaf list the axes in which it is the door of at least one DRIVEN row (union of the eight "Doors covered" lists). A leaf reached by no axis is `uncovered(<reason>)` — that set must be EMPTY or every member must carry a reason; a leaf reached only by axis 5 is `only-5(<reason>)` with the reason from acceptance-design.md (task list · config get · config list are the three expected ones — if the drive changed that set, say so). Compare the resulting table against acceptance-design.md\'s planned table and list every difference.',
    'Return: the list of files written, coverage_ok (true iff uncovered is empty), and a 5-sentence summary (findings confirmed / refuted / open; coverage; bounds). Do NOT commit; do NOT edit anything outside completions/artifacts/M51/per-axis-review/.',
  ].join('\n')
}

phase('Drive')
log('Driving eight axes on the installed 1.0.0-rc.15, then reconciling each against its Codex source pass')
const recon = await pipeline(
  AXES,
  (a) => agent(drivePrompt(a), { label: 'drive:axis' + a.n, phase: 'Drive', agentType: 'general-purpose', model: 'opus', schema: DRIVE_SCHEMA }),
  (drive, a) => agent(reconPrompt(a, drive), { label: 'reconcile:axis' + a.n, phase: 'Reconcile', agentType: 'general-purpose', model: 'opus', schema: RECON_SCHEMA }),
)
const dead = recon.map((r, i) => (r ? null : i + 1)).filter(Boolean)
if (dead.length) log('axes with no reconciliation result: ' + dead.join(', ') + ' — the assembler must mark them un-reviewed, not covered')

phase('Assemble')
const assembled = await agent(assemblePrompt(recon), { label: 'assemble', phase: 'Assemble', agentType: 'general-purpose', model: 'opus', schema: ASSEMBLE_SCHEMA })
return { axes: recon, dead, assembled }