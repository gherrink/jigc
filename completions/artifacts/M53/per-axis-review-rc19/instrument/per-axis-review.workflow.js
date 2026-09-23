export const meta = {
  name: 'm53-per-axis-review-rc19',
  description: 'M53 acceptance (the PARTIAL per-axis review — axes 2, 3, 5, 6 — the THIRD partial run after the cwd-dependence arc, compared row by row against the rc.19/rc.17/M52 baselines): four per-axis reviews on the installed 1.0.0-rc.19 — Opus driver, then reconciliation against the Codex source pass, then one assembler',
  phases: [
    { title: 'Drive' },
    { title: 'Reconcile' },
    { title: 'Assemble' },
  ],
}

const SCRATCH = '/private/tmp/claude-501/-Users-maurice-projects-gherrink-jigc/fc2f5e3a-a01b-43ba-8a1b-f93e48070233/scratchpad/axis-review-rc19'
const BIN = '/Users/maurice/.local/bin/jigc'
// The PARTIAL re-run: M53 is a fix pass and its acceptance is the three axes its four
// tier-1 rows sit on (decisions-pending.md → The rc.19 fix pass (M53) → the boundary).
const AXES = [
  { n: 2, name: 'posture' },
  { n: 3, name: 'destroying doors' },
  { n: 5, name: 'pinned contracts' },
  { n: 6, name: 'composed surfaces' },
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
    'M53 THIRD PARTIAL per-axis review on rc.19 after the cwd-dependence arc (axes 2 · 3 · 5 · 6 — 5 for task diff\'s pinned envelope + PATH_ARG_OCCURRENCES bases, 6 for the spawn line; the exit rule: the 1.0.0 call is taken iff this re-review finds NO TIER-1 ROW, tier 1 = exit-0 loss or repository harm through a committing, destroying or moving door) — AXIS ' + a.n + ' · ' + a.name + ' — the OPUS DRIVER. You own the (door, cell) table for this axis and you may not mark a row driven from a source read.',
    '',
    'READ FIRST — THE BASELINES: for axes 2/3 the rc.18 run (completions/artifacts/M53/per-axis-review-rc18/README.md §A + axis-N.md); for axis 5 the rc.17 run (completions/artifacts/M53/per-axis-review/README.md §A + axis-5.md); for axis 6 M52\'s §A (completions/artifacts/M52/per-axis-review/README.md + axis-6.md) — RE-DRIVE every row for your axis on rc.19 and record CLOSED (argv+observed) or STILL-OPEN (datum); tier-2/3 rows triaged to 1.x are expected STILL-OPEN, say so. THE CWD ARC IS WHAT CHANGED (completions/artifacts/M53/cwd-census.md; VERDICT.md → Addendum 2; the four 2026-09-23 DECISIONS.md entries; two reviews at completions/artifacts/M53/audit/cwd-fix-code-review*.md — every finding fixed, verify each is closed): DRIVE FROM FOUR CWDS — the repo root, a subdirectory (mkdir docs/deep), inside a provisioned fan-out worktree, and an ordinary linked worktree — and under a repo path containing a SPACE (mktemp -d "$SCRATCH/jigc space.XXXXXX"); the new mechanism to probe hardest: repo::jigc_home at every store door (milestone finalize/create/discard, task diff, pack loading, setup, uninstall), engine::finding::git_at/migrate_at and the four span fences, the quoted absolute Spawn: cd, PATH_ARG_OCCURRENCES\' typed base, the hook\'s shell-word tokenizer, uninstall\'s prune + site line. THEN completions/artifacts/M51/acceptance-design.md Part 2 (your axis row: the door set, the cell set, the Row schema) and completions/artifacts/M53/acceptance-design.md. INSTRUMENT NOTE: this harness\'s `grep` honours .gitignore — under `.jigc/` use `command grep` with a before-control.',
    'BINARY: the installed `' + BIN + '` — assert `jigc --version` prints `jigc 1.0.0-rc.19` before anything else and STOP if it does not. This is the RELEASE posture: route-fence panics do not exist here.',
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
    'INPUTS: the driver table ' + f + ' (driven rows + repro blocks + defects) and the Codex source pass ' + SCRATCH + '/codex/axis' + a.n + '-codex.md (claims as leads; its prompt is beside it as axis' + a.n + '-prompt.md). Binary: `' + BIN + '` (assert `jigc 1.0.0-rc.19`). Rigs as the driver used them (`rig=$(dev/jigc-rig <state> --binary ' + BIN + ') || exit; eval "$rig"`).',
    'BEFORE ANYTHING: the Codex source pass file may still be being written — if ' + SCRATCH + '/codex/axis' + a.n + '-codex.md does not exist yet or is empty, WAIT for it with a shell loop (`until [ -s <file> ]; do sleep 60; done`, up to 60 minutes) and only then proceed; never reconcile against a missing pass.',
    'THE RULE (acceptance-design.md → The reconciliation rule): a claim by one that the other cannot reproduce is a LEAD, not a finding. For EVERY Codex claim: enter it as `lead(codex, <claim>)`, then DRIVE it — to a repro block (then it is a CONFIRMED finding, origin codex) or to a recorded refutation with the falsifying datum (the driven output that contradicts it). For every driver DEFECT the source pass says cannot happen (or is silent on): it stays a finding (it was driven) — re-drive it once yourself to confirm the repro block, and record the source claim as refuted with that repro if the pass contradicted it. A Codex claim you genuinely cannot drive (needs a state no rig builds, or an environment you lack) stays an OPEN lead with the reason — never promote it on the source read, never drop it.',
    'Also check the driver table for rows marked driven that carry no repro block — such a row is NOT driven; demote it and say so.',
    'WRITE the reconciled axis file to ' + SCRATCH + '/reconciled/axis' + a.n + '.md: the driver table (unchanged except demotions), then a "Reconciliation ledger" section: each Codex claim → CONFIRMED (repro) | REFUTED (datum) | OPEN LEAD (reason); each driver defect → its status; then a "Doors covered" list (every clap leaf that is the door of ≥1 driven row, VERB_KINDS spelling). Return the file path, the confirmed/refuted/open lists, doors_covered, and notes. No fixes, no commits, no repo edits.',
  ].join('\n')
}

function assemblePrompt(results) {
  const summary = results.map((r, i) => 'axis ' + (i + 1) + ': ' + (r ? (r.file + ' — confirmed ' + (r.confirmed || []).length + ', refuted ' + (r.refuted || []).length + ', open ' + (r.leads_open || []).length) : 'NO RESULT (agent died)')).join('\n')
  return [
    'M53 THIRD PARTIAL per-axis review — ASSEMBLE the persisted record. The four reconciled axis files are in ' + SCRATCH + '/reconciled/axis{2,3,5,6}.md (drivers in ' + SCRATCH + '/driver/, Codex passes in ' + SCRATCH + '/codex/). Per-axis results:',
    summary,
    '',
    'WRITE, under completions/artifacts/M53/per-axis-review-rc19/ (create it; the rc.18, rc.17 and M52 records are the BASELINES and are NOT edited): README.md (what this is — the THIRD partial re-run after the cwd-dependence arc, AXES 2 · 3 · 5 · 6, on the installed `jigc 1.0.0-rc.19` from commit <RC19 SHA> on 2026-09-23; FIRST a ROW-BY-ROW COMPARISON table against the baselines\' §A (rc.19 for axes 2/3; rc.17 for axis 5; M52 for axis 6, all tiers) — every row CLOSED (argv) or STILL-OPEN (datum), tier-2/3 rows M53 deliberately did not fix marked STILL-OPEN(1.x, expected); then EVERY NEW FINDING tiered on the charter\'s predicate (tier 1 = exit-0 loss/harm through a committing/destroying/moving door · tier 2 = posture/route dead end · tier 3 = a surface says what the binary does not) — and the headline the human reads first: THE TIER-1 COUNT, and for each tier-1 row whether it sits INSIDE the cwd arc\'s new code (the ledger in VERDICT.md → Addendum 2 and the four 2026-09-23 DECISIONS entries) or outside it; the staffing; the reconciliation rule; a FINDINGS section listing every CONFIRMED finding with its repro block, every REFUTED claim with its datum, every OPEN lead with its reason; the honest bounds — anything an axis did not drive, and that axes 1, 4, 7, 8 were NOT re-driven by design), axis-2.md, axis-3.md, axis-5.md, axis-6.md (the reconciled files, copied verbatim with a one-line header naming the binary and date), and codex/axis-N-source-pass.md (the raw Codex outputs, verbatim).',
    'THE COVERAGE TABLE: read the VERB_KINDS leaves from crates/cli/src/cli.rs (state the count you read), then for each leaf list which of the FOUR axes reached it as the door of ≥1 DRIVEN row. Compare against the rc.18 run\'s coverage (axes 2/3), the rc.17 run\'s (axis 5) and M52\'s (axis 6) — for every leaf those baselines\' axes reached, this run must reach it too (list every leaf that lost coverage, with the reason); report the union and a stricter substantive column, and do not pass a control-only union off as coverage.',
    'Return: the list of files written, coverage_ok (true iff uncovered is empty), and a 5-sentence summary (findings confirmed / refuted / open; coverage; bounds). Do NOT commit; do NOT edit anything outside completions/artifacts/M53/per-axis-review-rc19/.',
  ].join('\n')
}

phase('Drive')
log('THIRD PARTIAL RE-RUN: driving axes 2, 3, 5 and 6 on the installed 1.0.0-rc.19 after the cwd-dependence arc, comparing row by row against the baselines')
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