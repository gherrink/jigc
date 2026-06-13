# Owner-artifact — M17 self-hosting dogfood (flow 21): the `decided-task` run

The genuine measured run that develops jigc with jigc's own methodology, at the spine's
native grain. Half-B (the live run) recorded here; the raw capture (hook log + tally) is
exported **unchanged** alongside this file and content-hashed below, so the
`dogfood-record`'s transcribed fact fields are auditable against the mechanical capture.

## Run identity
- **Case:** pilot (stand-in — see Honesty bounds; this is the *self-hosting* run, a case the
  `dogfood-record` enum predates).
- **Binary (pinned):** `jigc` sha256 `8d7ab2626411da7c169e0002dc7fd85e7474b1f1f2a3288e781debd13fbb7779`
  (`cargo build --release` at gherrink-jigc `fdcab9d`; reproduces the handover's pinned sha).
- **Twin:** local `git clone` of gherrink-jigc at `fdcab9d`, on real disk (not the working tree).
- **Capture:** inline-orchestrated — jrun shadows `jigc` (real exit captured), `JIGC_DOGFOOD_LOG`
  set. The Write|Edit OOB hook is inactive in inline mode (a compliant run routes managed-doc
  writes through jigc, so that channel is ~0 organically); the absorb channel + the sed-seed are
  unaffected. Both bounds named below.
- **doc↔code:** gated off — the methodology pack ships no `doc-code` probe for this run's docs.

## Manifest (content hashes — the auditable core)
| file | sha256 | lines |
|---|---|---|
| `hook-log.jsonl` (raw, unchanged) | `ee0d5e5e7282cdc065343b9cd06ec35d7d50713580638836abb2c42ab0427683` | 38 |
| `tally.json` (tally.py output, unchanged) | `eda932035fc90bbbc47f42e4f25689bba374dcd269ed5cbdae72d3aee3792549` | 83 |

## What the run did (the spine, native grain)
The target was the **deferred increment-level workflow itself** (decisions-pending.md:77 — the
grain gap the M17 pilot surfaced: no lightweight sub-milestone decision-recording path). jigc
dogfooded its methodology *by building the workflow that would have helped*:

1. **`dev-task`** → built `decided-task` (a `creates-task:true` dev-task variant: scope → implement
   → gate → **author-decision** → finalize, with `allows-create:[{decisions-log}]`) + the
   `author-decision` step, **test-first**: a new compose test was observed red (`no workflow
   decided-task`) then green; the off-router catalog guard gained the `decided-task` negatives.
   Full gate green (fmt · clippy -D · 77 suites · build). One commit (`56c9158`).
2. **`decided-task` (the just-built workflow)** → recorded its own two design decisions (its shape;
   its `selectable:false` off-routerness) to the managed, schema-validated `decisions-log`
   singleton — **the differentiator engaging via the very workflow the run built**. First
   *promoting* finalize (`5c280c9`). A preview `validate` first blocked on the unfilled commit doc
   (the designed preview loop — organic validate-block).
3. **Seed 1 (OOB):** a conformant `sed` edit to the committed `decisions-log` (outside the observed
   tools → absorb channel), absorbed at the next landed finalize.
4. **`completion`** → audited the increment (verdict green; one advisory finding deferred), authored
   the managed `completion-record` + this owner-artifact. The finalize first **blocked organically**
   (`owner-artifact.present` — the #5 gate caught the artifact untracked; staged → re-finalized:
   the landed finalize absorbed Seed 1). Commit `94bb3ac`.
5. **Seed 2 (bad-finalize):** a throwaway task finalized with an unfilled commit doc → blocked at
   exit 3 (`schema-conformance`); discarded.

## Facts (transcribed from the raw capture; seeded never enters organic)
Organic: adapter-writes **5**, oob-edits **0**, drift-caught **1** (`owner-artifact.present`),
validate-blocks **1**, halts-expected **2** (completion's two checkpoints), halts-unplanned **1**
(the `selectable` vs off-router fork a failing catalog test forced), fix-rounds **2**
(validate-block fix + owner-artifact finalize-block fix), audit-findings **1**.
Seeded (instrument checks, never thesis): seeded-oob **1**, seeded-blocks **1**. Verdict **green**.

Telemetry (never headline): jigc-invocations 38, write-verb-invocations 24.

## Honesty bounds (what this run did NOT test / where it is weak)
- **n=1, unblinded judge = run orchestrator.** The same agent built, ran, and judged. Mitigations:
  the committed raw hook log + tally + these hashes; facts are mechanical, the verdict is judgment.
- **Inline orchestration, not a fresh session.** The Write|Edit OOB hook was inactive; organic OOB
  was thus only observable via the absorb channel (which fired once — the seed). A compliant jigc
  agent routes managed writes through jigc, so the organic OOB denominator is structurally ~0 here.
  For self-hosting a knowledgeable operator is the *realistic* user, not a contamination — but the
  "naive fresh-agent discovery" angle the foreign pilot had is absent by construction.
- **Grain stretch on the spine.** Milestone-grained `completion` was run over a 2-commit increment
  (the audit/checkpoints are small); `dev-task` + `decided-task` are the natural grain.
- **`case` enum predates self-hosting.** Recorded as `pilot` (the methodology-validation phase);
  an owed one-line delta adds a `self-hosting` member. Named, not hidden.
- **Seeds validate the instrument, not the thesis.** The two seeded events are excluded from the
  organic facts; thesis evidence is the organic facts + the managed-doc quality only.
