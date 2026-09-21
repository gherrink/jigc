export const meta = {
  name: 'm52-per-axis-review',
  description: 'M52 acceptance (the M51 per-axis review RE-RUN, compared row by row): eight per-axis reviews on the installed 1.0.0-rc.16 — Opus driver, then reconciliation against the Codex source pass, then one assembler',
  phases: [
    { title: 'Drive' },
    { title: 'Reconcile' },
    { title: 'Assemble' },
  ],
}

const SCRATCH = '/private/tmp/claude-501/-Users-maurice-projects-gherrink-jigc/c9b07f4a-e007-4ea2-988f-f4c83653cd8f/scratchpad/axis-review'
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
    'M52 per-axis review (the RE-RUN of M51\'s instrument on the installed rc.16) — AXIS ' + a.n + ' · ' + a.name + ' — the OPUS DRIVER. You own the (door, cell) table for this axis and you may not mark a row driven from a source read.',
    '',
    'READ FIRST: completions/artifacts/M51/acceptance-design.md Part 2 (your axis row gives the door set and the cell set and the "Row schema" — the SAME eight axes are re-run), completions/artifacts/M52/acceptance-design.md (the flow-53 arms that pin part of this axis, and "The per-axis review re-run" rules), completions/artifacts/M52/VERDICT.md (M52\'s seven audit fixes landed after its build — 79e54c75 6d95756c fe8f29c4 c96137e4 b9ab6a70 1b036264 — your table must reflect the FIXED binary), and completions/artifacts/M51/per-axis-review/README.md §A + axis-' + a.n + '.md (M51\'s CONFIRMED rows for THIS axis: you must RE-DRIVE every one of them on rc.16 and record it CLOSED (argv + observed) or STILL-OPEN (datum) — that row-by-row comparison is the re-run\'s first deliverable; new findings come second). M52 minted registries you must ALSO iterate where your axis touches them: crates/cli/src ROLLBACK_POPULATIONS, TASK_AREA_FILES, PRE_DISPATCH_FAULTS, InProgress::ALL (nine git states; dev/jigc-rig has a git-state builder — see its --help), RelocateRefusal::ALL, ENVELOPE_OWED_CODES, DESTROYING_DOORS with its Disposition axis, the suppressed.door set, the fixed-identity doctype set (placement || singleton).',
    'BINARY: the installed `' + BIN + '` — assert `jigc --version` prints `jigc 1.0.0-rc.16` before anything else and STOP if it does not. This is the RELEASE posture: route-fence panics do not exist here.',
    'RIGS: `rig=$(dev/jigc-rig <state> --binary ' + BIN + ') || exit; eval "$rig"` — two-step eval, never `eval "$(…)"`, never `rm -rf` a variable path; states: fresh · committed-singletons · migrated · chatty-hooks · vendored · bare (`dev/jigc-rig --help`). `--pack-from-dev`/`--schema`/`--workflow` manufacture a throwaway pack (axis 7 needs one). Build each fixture you need with the binary, never by writing into `.jigc/`.',
    'DERIVE THE DOOR SET FROM THE CODE, not from the design doc\'s numbers: read the registries the design names (crates/cli/src/cli.rs PATH_ARG_OCCURRENCES / DOCTYPE_DOORS / SLUG_DOORS / WORK_UNIT_ID_DOORS / BEHALF_DOORS / VERB_KINDS; crates/cli/src/invocation_log.rs COMMITTING_DOORS; crates/cli/src/milestone.rs DESTROYING_DOORS; crates/cli/src/render.rs ENVELOPE_ARMS / STORE_EXIT_FLIPS / ManifestKind; crates/engine/src/schema_diff.rs SchemaChangeKind) and state the count you read.',
    'DRIVE every applicable (door, cell) row and record per row: argv driven · exit · code|none · route kind (Mechanical/Human/Informational/none) · surface asserted (the line or JSON key) · verdict (matches contract / DEFECT / n/a with reason). A row is driven iff its argv ran on the installed binary and its verdict is recorded with a repro block (setup · argv · observed exit/output). Group repeated drives into one repro block per cell where the argv only differs by door. Where the cell set × door set is large, drive every door for at least the cells that discriminate (the escape shapes, the refusals) and state plainly which (door, cell) pairs you did NOT drive and why — never present an un-driven pair as driven. Do NOT read the Codex source pass for this axis (it exists; reconciliation is a separate agent).',
    'DEFECT = the binary contradicts a stated contract (a design doc rule, the finding\'s own route, a help text, the acceptance design\'s cell expectation). Record every defect with its repro block. Use `--format json` wherever the axis is about the envelope.',
    'WRITE the full table + repro blocks + the un-driven list + a "M51 rows: CLOSED / STILL-OPEN" section first, then a "what this adds over flow-53 arm N" paragraph to ' + SCRATCH + '/driver/axis' + a.n + '.md (create the directory). Return: the file path, the counts, every defect as a finding, `doors_covered` = every clap leaf that is the door of at least one DRIVEN row (VERB_KINDS spelling), and notes. No fixes, no commits, no repo edits.',
  ].join('\n')
}

function reconPrompt(a, drive) {
  const f = drive && drive.file ? drive.file : (SCRATCH + '/driver/axis' + a.n + '.md')
  return [
    'M52 per-axis review (re-run) — AXIS ' + a.n + ' · ' + a.name + ' — RECONCILIATION between the Opus driver\'s table and the Codex source pass.',
    '',
    'INPUTS: the driver table ' + f + ' (driven rows + repro blocks + defects) and the Codex source pass ' + SCRATCH + '/codex/axis' + a.n + '-codex.md (claims as leads; its prompt is beside it as axis' + a.n + '-prompt.md). Binary: `' + BIN + '` (assert `jigc 1.0.0-rc.16`). Rigs as the driver used them (`rig=$(dev/jigc-rig <state> --binary ' + BIN + ') || exit; eval "$rig"`).',
    'BEFORE ANYTHING: the Codex source pass file may still be being written — if ' + SCRATCH + '/codex/axis' + a.n + '-codex.md does not exist yet or is empty, WAIT for it with a shell loop (`until [ -s <file> ]; do sleep 60; done`, up to 60 minutes) and only then proceed; never reconcile against a missing pass.',
    'THE RULE (acceptance-design.md → The reconciliation rule): a claim by one that the other cannot reproduce is a LEAD, not a finding. For EVERY Codex claim: enter it as `lead(codex, <claim>)`, then DRIVE it — to a repro block (then it is a CONFIRMED finding, origin codex) or to a recorded refutation with the falsifying datum (the driven output that contradicts it). For every driver DEFECT the source pass says cannot happen (or is silent on): it stays a finding (it was driven) — re-drive it once yourself to confirm the repro block, and record the source claim as refuted with that repro if the pass contradicted it. A Codex claim you genuinely cannot drive (needs a state no rig builds, or an environment you lack) stays an OPEN lead with the reason — never promote it on the source read, never drop it.',
    'Also check the driver table for rows marked driven that carry no repro block — such a row is NOT driven; demote it and say so.',
    'WRITE the reconciled axis file to ' + SCRATCH + '/reconciled/axis' + a.n + '.md: the driver table (unchanged except demotions), then a "Reconciliation ledger" section: each Codex claim → CONFIRMED (repro) | REFUTED (datum) | OPEN LEAD (reason); each driver defect → its status; then a "Doors covered" list (every clap leaf that is the door of ≥1 driven row, VERB_KINDS spelling). Return the file path, the confirmed/refuted/open lists, doors_covered, and notes. No fixes, no commits, no repo edits.',
  ].join('\n')
}

function assemblePrompt(results) {
  const summary = results.map((r, i) => 'axis ' + (i + 1) + ': ' + (r ? (r.file + ' — confirmed ' + (r.confirmed || []).length + ', refuted ' + (r.refuted || []).length + ', open ' + (r.leads_open || []).length) : 'NO RESULT (agent died)')).join('\n')
  return [
    'M52 per-axis review (re-run) — ASSEMBLE the persisted record. The eight reconciled axis files are in ' + SCRATCH + '/reconciled/axis{1..8}.md (drivers in ' + SCRATCH + '/driver/, Codex passes in ' + SCRATCH + '/codex/). Per-axis results:',
    summary,
    '',
    'WRITE, under completions/artifacts/M52/per-axis-review/ (create it; M51\'s record at completions/artifacts/M51/per-axis-review/ is the baseline and is NOT edited): README.md (what this is — M52\'s acceptance instrument per settle-record D12 and acceptance-design.md → The per-axis review re-run: M51\'s eight-axis instrument re-run on the installed `jigc 1.0.0-rc.16` from commit a3eb026b on 2026-09-21; a ROW-BY-ROW COMPARISON table against M51\'s §A — every one of the 39 rows CLOSED (argv) or STILL-OPEN (datum), plus M51\'s §D open leads re-dispositioned; every NEW finding tiered on the charter\'s predicate (exit-0 loss/harm through a committing/destroying/moving door · posture/route dead end · surface says what the binary does not); the staffing — one Opus driver + one Codex source pass per axis + one reconciler; the reconciliation rule; a FINDINGS section listing every CONFIRMED finding across all axes with its repro block, every REFUTED claim with its datum, every OPEN lead with its reason; the honest bounds — anything an axis states it did not drive; and the COVERAGE table), axis-1.md … axis-8.md (the reconciled files, copied verbatim with a one-line header naming the binary and date), and codex/axis-N-source-pass.md (the raw Codex outputs, verbatim, so the leads are auditable).',
    'THE COVERAGE TABLE (acceptance-design.md → Coverage — all 47 VERB_KINDS leaves; §17\'s fence strengthened): read the 47 leaves from crates/cli/src/cli.rs VERB_KINDS (state the count you read — if it is not 47, say so and use the real set), then for each leaf list the axes in which it is the door of at least one DRIVEN row (union of the eight "Doors covered" lists). A leaf reached by no axis is `uncovered(<reason>)` — that set must be EMPTY or every member must carry a reason; a leaf reached only by axis 5 is `only-5(<reason>)` with the reason from acceptance-design.md (task list · config get · config list are the three expected ones — if the drive changed that set, say so). Compare the resulting table against M51\'s driven coverage table (completions/artifacts/M51/per-axis-review/README.md → COVERAGE) and list every difference — NO leaf may lose an axis; the only-5 reasons come from M51\'s table.',
    'Return: the list of files written, coverage_ok (true iff uncovered is empty), and a 5-sentence summary (findings confirmed / refuted / open; coverage; bounds). Do NOT commit; do NOT edit anything outside completions/artifacts/M52/per-axis-review/.',
  ].join('\n')
}

phase('Drive')
log('RE-RUN: driving the eight M51 axes on the installed 1.0.0-rc.16, then reconciling each against its Codex source pass, comparing row by row')
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