export const meta = {
  name: 'm53-per-axis-review-rc18',
  description: 'M53 acceptance (the PARTIAL per-axis review — axes 2, 3 — the SECOND partial run after the post-review fix, compared row by row against the rc.18 run): two per-axis reviews on the installed 1.0.0-rc.18 — Opus driver, then reconciliation against the Codex source pass, then one assembler',
  phases: [
    { title: 'Drive' },
    { title: 'Reconcile' },
    { title: 'Assemble' },
  ],
}

const SCRATCH = '/private/tmp/claude-501/-Users-maurice-projects-gherrink-jigc/fc2f5e3a-a01b-43ba-8a1b-f93e48070233/scratchpad/axis-review-rc18'
const BIN = '/Users/maurice/.local/bin/jigc'
// The PARTIAL re-run: M53 is a fix pass and its acceptance is the three axes its four
// tier-1 rows sit on (decisions-pending.md → The rc.18 fix pass (M53) → the boundary).
const AXES = [
  { n: 2, name: 'posture' },
  { n: 3, name: 'destroying doors' },
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
    'M53 SECOND PARTIAL per-axis review on rc.18 after the post-review fix (axes 2 · 3 only — axis 3 because the fix changed a destroying door — M53 is a FIX PASS; the exit rule: the 1.0.0 call is taken iff this re-review finds NO TIER-1 ROW, tier 1 = exit-0 loss or repository harm through a committing, destroying or moving door) on the installed rc.18 — AXIS ' + a.n + ' · ' + a.name + ' — the OPUS DRIVER. You own the (door, cell) table for this axis and you may not mark a row driven from a source read.',
    '',
    'READ FIRST — THE BASELINE IS NOW THE rc.17 RUN: completions/artifacts/M53/per-axis-review/README.md §A (all tiers) + axis-' + a.n + '.md — RE-DRIVE every row for your axis on rc.18 and record CLOSED (argv+observed) or STILL-OPEN (datum); the tier-1 row (2, DEFECT 1) and its post-review fix are the headline (completions/artifacts/M53/VERDICT.md → Addendum; completions/artifacts/M53/audit/post-review-fix-code-review.md — its seven findings were ALL fixed, verify each is closed; the new mechanism to probe hardest: repo::adjudicated_breach + BreachSite at every acting door and the boundary, LeftoverHold.operation at provision/discard/uninstall, the render root jigc_home, the declared target forms). THEN completions/artifacts/M51/acceptance-design.md Part 2 (your axis row gives the door set and the cell set and the "Row schema"), completions/artifacts/M53/acceptance-design.md (the flow-54 arms that pin part of this axis), completions/artifacts/M53/settle-record.md (D1–D5 + §1–§14; §14 is THE LEDGER of what M53 minted — one code `finalize.foreign-bytes`, one InProgress variant, four producers under shipped codes, the `merged/` walk, the RESIDUAL RULE (`carries_base_pin`), FINALIZE_MESSAGE_FILE in both registries — a finding inside THAT code is what the exit rule asks you to look hardest for), completions/artifacts/M53/VERDICT.md (M53\'s SEVEN audit fixes landed after its build — 4a8cec2d 620c1644 1e68662f 33ab0f76 2cb2c29f c00fdd97 5702bf4f — your table must reflect the FIXED binary; its Declared bounds are what you grade AGAINST, not re-find: the stderr-only advisory at milestone finalize, the 21-render DEBUG_REMAINDER, the residual cleared by hand, reseed_sub_task_areas\' .exists() skip), and completions/artifacts/M52/per-axis-review/README.md §A (ALL tiers) + axis-' + a.n + '.md (M52\'s CONFIRMED rows for THIS axis: you must RE-DRIVE every one of them on rc.18 and record it CLOSED (argv + observed) or STILL-OPEN (datum) — that row-by-row comparison is the re-run\'s first deliverable; tier-2/3 rows were triaged to 1.x and a STILL-OPEN there is expected and is NOT a new finding, but say so; new findings come second, each TIERED on the charter\'s predicate). INSTRUMENT NOTE from M53 planning: this harness\'s `grep` is a shell function honouring .gitignore — under `.jigc/` use `command grep` with a BEFORE-control that finds your plant, else every loss claim is unsound (M52\'s R-I evidence had that shape). M53\'s registries you must ALSO iterate where your axis touches them: InProgress::ALL (TEN states now; dev/jigc-rig --git-state builds them incl. the four uncommitted-cherry-pick shapes), DESTROYING_DOORS with its Disposition axis, TASK_AREA_FILES / MILESTONE_AREA_FILES (both now carry FINALIZE_MESSAGE_FILE; merged/ is walked, staged_doc_id alone decides inside it), WORK_UNIT_ID_DOORS (25 rows, now a residual cell), MINT_DOORS (five, three prose + two Exempt), BEHALF_DOORS acting rows.',
    'BINARY: the installed `' + BIN + '` — assert `jigc --version` prints `jigc 1.0.0-rc.18` before anything else and STOP if it does not. This is the RELEASE posture: route-fence panics do not exist here.',
    'RIGS: `rig=$(dev/jigc-rig <state> --binary ' + BIN + ') || exit; eval "$rig"` — two-step eval, never `eval "$(…)"`, never `rm -rf` a variable path; states: fresh · committed-singletons · migrated · chatty-hooks · vendored · bare (`dev/jigc-rig --help`). `--pack-from-dev`/`--schema`/`--workflow` manufacture a throwaway pack (axis 7 needs one). Build each fixture you need with the binary, never by writing into `.jigc/`.',
    'DERIVE THE DOOR SET FROM THE CODE, not from the design doc\'s numbers: read the registries the design names (crates/cli/src/cli.rs PATH_ARG_OCCURRENCES / DOCTYPE_DOORS / SLUG_DOORS / WORK_UNIT_ID_DOORS / BEHALF_DOORS / VERB_KINDS; crates/cli/src/invocation_log.rs COMMITTING_DOORS; crates/cli/src/milestone.rs DESTROYING_DOORS; crates/cli/src/render.rs ENVELOPE_ARMS / STORE_EXIT_FLIPS / ManifestKind; crates/engine/src/schema_diff.rs SchemaChangeKind) and state the count you read.',
    'DRIVE every applicable (door, cell) row and record per row: argv driven · exit · code|none · route kind (Mechanical/Human/Informational/none) · surface asserted (the line or JSON key) · verdict (matches contract / DEFECT / n/a with reason). A row is driven iff its argv ran on the installed binary and its verdict is recorded with a repro block (setup · argv · observed exit/output). Group repeated drives into one repro block per cell where the argv only differs by door. Where the cell set × door set is large, drive every door for at least the cells that discriminate (the escape shapes, the refusals) and state plainly which (door, cell) pairs you did NOT drive and why — never present an un-driven pair as driven. Do NOT read the Codex source pass for this axis (it exists; reconciliation is a separate agent).',
    'DEFECT = the binary contradicts a stated contract (a design doc rule, the finding\'s own route, a help text, the acceptance design\'s cell expectation). Record every defect with its repro block. Use `--format json` wherever the axis is about the envelope.',
    'WRITE the full table + repro blocks + the un-driven list + a "M52 §A rows: CLOSED / STILL-OPEN" section first, then a "what this adds over flow-54 arm N" paragraph to ' + SCRATCH + '/driver/axis' + a.n + '.md (create the directory). Return: the file path, the counts, every defect as a finding, `doors_covered` = every clap leaf that is the door of at least one DRIVEN row (VERB_KINDS spelling), and notes. No fixes, no commits, no repo edits.',
  ].join('\n')
}

function reconPrompt(a, drive) {
  const f = drive && drive.file ? drive.file : (SCRATCH + '/driver/axis' + a.n + '.md')
  return [
    'M53 partial per-axis review — AXIS ' + a.n + ' · ' + a.name + ' — RECONCILIATION between the Opus driver\'s table and the Codex source pass.',
    '',
    'INPUTS: the driver table ' + f + ' (driven rows + repro blocks + defects) and the Codex source pass ' + SCRATCH + '/codex/axis' + a.n + '-codex.md (claims as leads; its prompt is beside it as axis' + a.n + '-prompt.md). Binary: `' + BIN + '` (assert `jigc 1.0.0-rc.18`). Rigs as the driver used them (`rig=$(dev/jigc-rig <state> --binary ' + BIN + ') || exit; eval "$rig"`).',
    'BEFORE ANYTHING: the Codex source pass file may still be being written — if ' + SCRATCH + '/codex/axis' + a.n + '-codex.md does not exist yet or is empty, WAIT for it with a shell loop (`until [ -s <file> ]; do sleep 60; done`, up to 60 minutes) and only then proceed; never reconcile against a missing pass.',
    'THE RULE (acceptance-design.md → The reconciliation rule): a claim by one that the other cannot reproduce is a LEAD, not a finding. For EVERY Codex claim: enter it as `lead(codex, <claim>)`, then DRIVE it — to a repro block (then it is a CONFIRMED finding, origin codex) or to a recorded refutation with the falsifying datum (the driven output that contradicts it). For every driver DEFECT the source pass says cannot happen (or is silent on): it stays a finding (it was driven) — re-drive it once yourself to confirm the repro block, and record the source claim as refuted with that repro if the pass contradicted it. A Codex claim you genuinely cannot drive (needs a state no rig builds, or an environment you lack) stays an OPEN lead with the reason — never promote it on the source read, never drop it.',
    'Also check the driver table for rows marked driven that carry no repro block — such a row is NOT driven; demote it and say so.',
    'WRITE the reconciled axis file to ' + SCRATCH + '/reconciled/axis' + a.n + '.md: the driver table (unchanged except demotions), then a "Reconciliation ledger" section: each Codex claim → CONFIRMED (repro) | REFUTED (datum) | OPEN LEAD (reason); each driver defect → its status; then a "Doors covered" list (every clap leaf that is the door of ≥1 driven row, VERB_KINDS spelling). Return the file path, the confirmed/refuted/open lists, doors_covered, and notes. No fixes, no commits, no repo edits.',
  ].join('\n')
}

function assemblePrompt(results) {
  const summary = results.map((r, i) => 'axis ' + (i + 1) + ': ' + (r ? (r.file + ' — confirmed ' + (r.confirmed || []).length + ', refuted ' + (r.refuted || []).length + ', open ' + (r.leads_open || []).length) : 'NO RESULT (agent died)')).join('\n')
  return [
    'M53 PARTIAL per-axis review — ASSEMBLE the persisted record. The two reconciled axis files are in ' + SCRATCH + '/reconciled/axis{2,3}.md (drivers in ' + SCRATCH + '/driver/, Codex passes in ' + SCRATCH + '/codex/). Per-axis results:',
    summary,
    '',
    'WRITE, under completions/artifacts/M53/per-axis-review-rc18/ (create it; the rc.18 record at completions/artifacts/M53/per-axis-review/ is the BASELINE and is NOT edited): README.md (what this is — M53\'s acceptance per decisions-pending.md → The rc.18 fix pass (M53) and the EXIT RULE: the SECOND partial re-run after the post-review fix, AXES 2 · 3 ONLY, on the installed `jigc 1.0.0-rc.18` from commit fbd8b190 on 2026-09-23; FIRST a ROW-BY-ROW COMPARISON table against the rc.18 run\'s §A (all tiers) restricted to those two axes — the tier-1 row (2, DEFECT 1) CLOSED or not, with argv, is the headline — every row CLOSED (argv) or STILL-OPEN (datum), with tier-2/3 rows M53 deliberately did not fix marked STILL-OPEN(1.x, expected); then EVERY NEW FINDING tiered on the charter\'s predicate (tier 1 = exit-0 loss/harm through a committing/destroying/moving door · tier 2 = posture/route dead end · tier 3 = a surface says what the binary does not) — and the headline the human reads first: THE TIER-1 COUNT, and for each tier-1 row whether it sits INSIDE M53\'s own new code (settle-record §14\'s ledger) or outside it; the staffing — one Opus driver + one Codex source pass per axis + one reconciler; the reconciliation rule; a FINDINGS section listing every CONFIRMED finding with its repro block, every REFUTED claim with its datum, every OPEN lead with its reason; the honest bounds — anything an axis states it did not drive, and that axes 1, 4, 6, 7, 8 were NOT re-driven by design), axis-2.md, axis-3.md (the reconciled files, copied verbatim with a one-line header naming the binary and date), and codex/axis-N-source-pass.md (the raw Codex outputs, verbatim).',
    'THE COVERAGE TABLE: read the VERB_KINDS leaves from crates/cli/src/cli.rs (state the count you read), then for each leaf list which of the THREE axes reached it as the door of ≥1 DRIVEN row (union of the three "Doors covered" lists). This is a PARTIAL run, so `uncovered` will be non-empty: compare against the rc.17 run\'s coverage table (completions/artifacts/M53/per-axis-review/README.md → COVERAGE) RESTRICTED to axes 2/3 — for every leaf the rc.17 axes 2/3 reached, this run must reach it too (list every leaf that lost coverage, with the reason), and any leaf newly reached is listed. Do not claim 47/47.',
    'Return: the list of files written, coverage_ok (true iff uncovered is empty), and a 5-sentence summary (findings confirmed / refuted / open; coverage; bounds). Do NOT commit; do NOT edit anything outside completions/artifacts/M53/per-axis-review-rc18/.',
  ].join('\n')
}

phase('Drive')
log('SECOND PARTIAL RE-RUN: driving axes 2 and 3 on the installed 1.0.0-rc.18 after the post-review fix, comparing row by row against the rc.18 run')
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