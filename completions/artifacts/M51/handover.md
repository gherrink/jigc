# M51 handover — start here, then run `milestone-build`

Written 2026-09-14 while the planning context was live. **Nothing below needs reconstructing from a
transcript**: every decision, correction and drive is committed, and this file only points.

## State you're inheriting

| | |
|---|---|
| HEAD | `cd4ba5fa`, pushed, tree clean |
| Gate | **3341 passed / 0 failed**, fmt + clippy clean, measured unpiped via `dev/gate` ([log](../evidence-check-1.0/gate-rc14-at-74627547.log), at `74627547` — the planning commit on top is docs only) |
| Binary | `1.0.0-rc.14` installed at `~/.local/bin/jigc`; `target/release/jigc` built from HEAD is the same product code |
| The 1.0.0 call | **Not taken.** The evidence check found two data-loss defects at exit 0 that no trial reached ([VERDICT](../evidence-check-1.0/VERDICT.md)); the call waits on M51 and its acceptance |
| M51 | **Planned, settled, reviewed, gate-record filled, decomposed — not built** |

## What you're running

**`milestone-build` for M51, base `cd4ba5fa`.** The decomposition has one home and is not restated
here: [implementation/roadmap.md](../../../implementation/roadmap.md) → *Milestone 51 … decomposition*
— eleven increments, risk-first and linear, each with Deliverable / Grouped scope / Proves / the design
docs it moves / the codes it registers / its flow-52 arm / declared bounds.

The decisions behind it have one home too: [settle-record.md](settle-record.md) — D1–D15, then
**Review amendments §1–§19**, which correct four "reuse the shipped mechanism" claims that were false
when driven. **Read a decision together with its amendment bracket**; the amendment wins where they
differ (D3's predicate is worktree-vs-HEAD per path, not a staged snapshot; D1's door is three
predicates plus a resolve-or-refuse step, not "no new capability"; D4's restore is compare-and-swap;
D5's table is the [census](envelope-key-census.md), not "17 keys").

Three shapes are deliberately left to the plan (settle-record → *Owed at decompose*): D2's registry
identifier and home, D3's `CarryoverBoundary::Setup` code spelling and route text, the eleven
hand-enumerated `EnvelopeArm` rows' reasons, and D6's policy text. Everything else is decided.

## The rules that bit this wave — do not re-learn them

1. **An agent's report is a lead, not a measurement.** Two Settle decisions and two reuse claims were
   falsified only when someone drove the binary. A build-planner that reads a claim in the settle
   record and does not spike it inherits the same risk; the increments name their red tests for that
   reason.
2. **Drive with `dev/jigc-rig <state> --binary <path>`, two-step eval, never `rm -rf` a variable
   path.** The rig drives the *debug* binary by default; pass `--binary target/release/jigc` when the
   release posture matters (route-fence panics do not exist in release).
3. **The gate is slow and must run bare.** `dev/gate` takes ~20 min; background it, never foreground-
   poll, never read an exit code through a pipe. Concurrent cargo runs contend on `target/`; use
   `--private-target` or a private `CARGO_TARGET_DIR` for a second run.
4. **The guide bytes move exactly once** (Increment 9). Increments 3, 5 and 7 record their guide
   sentences into that batch; they do not edit `QUICKSTART.md`/`MIGRATING.md` themselves.
5. **A doc move rides the increment that makes it true.** Increment 10 carries only record
   corrections; every `design/` move is named in the increment whose behaviour it describes.
6. **Zero schema-hash movement, zero `schema-version` bumps, zero corpus migrations.** `optional:`,
   `set:`, `default:`, `of:`, `check:` and `title-names-symbol` are inside the hash; a "reword" that
   flips one is version-gated.
7. **The bump to `1.0.0-rc.15` happens after the completion audit's fixes, not before** — the
   version-stamp confirmation has caught the owed bump unshipped in five consecutive waves.
8. **`foldback_truth.rs:242` still pins the `**M50 —` span of CLAUDE.md.** Increment 11 re-aims it to
   `**M51 —` and inverts it to the pre-audit direction; the fold-back and that fence move together.

## After the build

1. The milestone-completion audit, its persisted verdict, the rc.15 build + install.
2. **The per-axis review on the installed rc.15** — M51's acceptance instrument (D15, §17, §19).
   Its definition is done: [acceptance-design.md](acceptance-design.md) holds the eight axes'
   `(door, cell)` matrices, the door-set derivations, the 47-leaf coverage table with `only-5(reason)`
   for the three leaves only the envelope axis reaches, and the Opus-driver / Codex-source-pass split
   with the reconciliation rule (*a claim by one that the other cannot reproduce is a lead*). What is
   owed is the driving.
3. A blind duress re-measure **only if** the wave changed a surface a worker reads (orientation, the
   read-back fence). Otherwise none.
4. **The 1.0.0 call — the human's.**

## Where everything is

| | |
|---|---|
| The evidence check (what found the blockers) | [evidence-check-1.0/VERDICT.md](../evidence-check-1.0/VERDICT.md) + eight reports |
| Charter, boundary, razor (with the D13 correction) | [charter.md](charter.md) |
| Baseline (four driven ledgers) | [baseline-ledger.md](baseline-ledger.md) + four companions |
| Gaps (69 ranked, the forks, 14 doc contradictions) | [gap-findings.md](gap-findings.md) + four reports |
| Settle + amendments | [settle-record.md](settle-record.md); advocate cases in [advocates/](advocates/) |
| Reviews | [design-review.md](design-review.md) (Opus, drove the binary) · [codex-design-review.md](codex-design-review.md) |
| Key census (60 arms, 57 dispositions) | [envelope-key-census.md](envelope-key-census.md) |
| Gate-record + halt discharges | [planning-gate-record.md](planning-gate-record.md) · [gate-halt-discharges.md](gate-halt-discharges.md) |
| Acceptance design | [acceptance-design.md](acceptance-design.md) |
| The why, dated | `DECISIONS.md` → the four 2026-09-10/11 entries |
