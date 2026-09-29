# M52 handover — start here, then plan, then build

Written 2026-09-16 while the M51 build, audit and per-axis review were live in one session. **Nothing
below needs reconstructing from a transcript**: every decision, correction and drive is committed, and
this file only points.

## State you're inheriting

| | |
|---|---|
| HEAD | `b2d78da8` (the tooling chore) plus this handover commit on top, pushed, tree clean |
| Gate | **3550 passed / 0 failed**, fmt + clippy clean, measured bare via `dev/gate` (the chore commit's own run; its per-step times are in that commit's report) |
| Binary | `1.0.0-rc.15` installed at `~/.local/bin/jigc` (sha256 `126f1584…`), built from `577a0099`; `target/` was `cargo clean`ed by the chore, so the first build is cold |
| The 1.0.0 call | **Not taken on rc.15.** The per-axis review found exit-0 data-loss rows behind committing doors — the class that blocked the call after RC-m50 ([DECISIONS.md](../../../DECISIONS.md) → 2026-09-16 M52 chartered) |
| M51 | **Complete**: built, audited (5 findings, 5 fixed axis-complete), rc.15 stamped after the fixes — [VERDICT](../M51/VERDICT.md) |
| M52 | **Chartered, not planned.** [decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.16 wave (M52)*: all 39 confirmed review rows, tiered (12 · 17 · 10), the 7 open leads dispositioned with triggers, fixes + understandability only |

## What you're running

**First `/milestone-plan M52`, then `milestone-build` for M52.** Both from the main session; planning is
human-led and its Settle forks go to the human one at a time, each beside an independent
`robust-advocate` case.

**The evidence base is already adversarially verified.** The per-axis review
([per-axis-review/README.md](../M51/per-axis-review/README.md)) drove every row on the installed rc.15
with a repro block, reconciled every Codex lead to a repro or a refutation, and its coverage table
reaches all 47 leaves. So the Scope phase's baseline check does **not** re-verify the 39 rows; it
drives the **classes** behind them (the axis each fix must iterate), because M51's audit widened every
class it was handed (5 → 22 route producers · 11 → 10 install paths · 2 → 3 roadmap sentences · four
stamp readers) and the review's own reconciliation widened two more (axis-3 C-1: three doors claimed,
five driven; axis-5 D: 47/47). **A row in the review is a verified instance; the class is what the wave
ships.**

**The Settle forks already visible** are written into the charter (*Owed at planning*) — the rollback
family (promote / record / `RecordFlipGuard` unconditional restores vs the config-layer CAS), the
posture family's members (cherry-pick, revert, `git am`; `--carry-staged` under any operation), the
destroying-door subject (what makes a non-`.md` task-area file a subject), the location-only bump
(`Relocated` and the `schema-hash` — **a home change sits inside the hash**, so the Settle must say *bump*
in those words with the adopter migration priced, or find the no-bump route), the singleton `<slug>`
head at write doors, and `ingest` absorbing a blocking edit. Planning re-poses them against a driven
baseline; it does not inherit them.

**Build with the harness defaults:** `Workflow({ name: 'milestone-build', args: { milestone: 'M52',
base: '<HEAD after planning>', model: 'opus' } })`. The `model` arg is what pins the subagents to Opus
— the `.claude/agents/*.md` frontmatter key is **not** honored by the Workflow runtime (measured
2026-09-14; `2e24b324`). Re-pass **all three** args on every resume.

## The rules that bit M51 — do not re-learn them

1. **An agent's report is a lead, not a measurement.** Two settle sentences were falsified by one
   `cat` on the rig (§20: *the stamp is a bare integer*) and one `grep` (§21: a scope row lost between
   the gate-record and the roadmap). **Before decompose, diff the gate-record's rows against the
   roadmap's Grouped scope**; nothing fences the two.
2. **Drive with `dev/jigc-rig <state> --binary <path>`**, two-step eval, never `rm -rf` a variable
   path, and **`cd "$REPO"`** — one review driver ran in the working repo instead of its rig and left a
   stray `.jigc/` there. The rig drives the debug binary by default; pass
   `--binary ~/.local/bin/jigc` for the release posture (route-fence panics do not exist in release).
3. **The gate runs bare and in the background.** `dev/gate` measured ~9.5 min after the litter was
   cleaned (it was ~20 min before: 1.8 M split-debuginfo sidecars in `target/debug/deps`, walked by
   Gatekeeper and Spotlight). The chore commit packed the debuginfo, added a deps-count advisory to
   `dev/gate`, and an after-milestone cleanup step; if the advisory warns, clean before building.
4. **Fixers run one at a time** on the shared branch — two agents committing on one index race.
   The build harness already serializes; do the same for audit fixers and the close.
5. **Report the audit's findings before fixing them**, confirm each by driving, and brief the fixer
   with the finding, never with the finding's boundary — every M51 fixer found a larger class.
6. **Zero schema-hash movement, zero `schema-version` bumps, zero corpus migrations — unless the
   Settle decides one explicitly.** `optional:`, `set:`, `default:`, `of:`, `check:`, `title-names-symbol`
   **and a doctype's home** are inside the hash.
7. **The bump to `1.0.0-rc.16` happens after the completion audit's fixes, not before.** The
   version-stamp confirmation caught the owed bump unshipped in five consecutive waves.
8. **`foldback_truth.rs` pins the newest `**M<nn> —` span of CLAUDE.md.** The close increment re-aims
   it to `**M52 —` in the *built, not audited* direction; the completion close inverts it back and
   requires the VERDICT citation (M51's close did exactly this — `577a0099`).
9. **A halt's resume note lives in the run's script snapshot, keyed on the increment, and stays
   byte-identical** across later resumes; every other planner call replays from cache.

## After the build

1. The milestone-completion audit → findings reported before fixing → fixes, one at a time → the
   persisted VERDICT under `completions/artifacts/M52/` → the rc.16 build + install, goldens
   version-only, checksum-matched.
2. **The per-axis review on the installed rc.16.** The instrument is preserved and indexed:
   [per-axis-review/instrument/](../M51/per-axis-review/instrument/) — the eight Codex prompts and
   the Workflow script; re-point the paths and the expected `--version`. Compare its coverage table
   and findings against M51's, row by row.
3. **The 1.0.0 call — the human's.**

## Where everything is

| | |
|---|---|
| The charter (tiers, open leads, rules, Settle forks) | [decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.16 wave (M52)* |
| The evidence base | [M51/per-axis-review/](../M51/per-axis-review/) — README (findings ledger, coverage, bounds), `axis-1.md` … `axis-8.md`, `codex/` (raw source passes), `instrument/` |
| M51's audit and its fixes | [M51/VERDICT.md](../M51/VERDICT.md); fix commits `6c2391c0` · `da5173a1` · `ff2bde99` · `507c332d` |
| M51's planning artifacts (the mold) | [M51/](../M51/) — charter, baseline-ledger, gap-findings, settle-record (D1–D15, §1–§21), planning-gate-record, acceptance-design, handover |
| The two mid-build halts | settle-record §20 (`548576a1`) and §21 (`5f9499ba`) |
| The build harness and its resume rules | `.claude/workflows/milestone-build.js` (header block) |
| The why, dated | `DECISIONS.md` → the 2026-09-14/16 M51 entries and the M52 charter entry |

## The prompt for the next session

> Read `completions/artifacts/M52/handover.md` first, then CLAUDE.md's "Project state" (the M51 paragraph
> and its tail), then `implementation/decisions-pending.md` → *The rc.16 wave (M52)* and
> `completions/artifacts/M51/per-axis-review/README.md` (the findings ledger §A–§D and the coverage table).
>
> Then plan M52 with `/milestone-plan M52`: the evidence base is the per-axis review, already verified
> with repro blocks, so the baseline check drives the classes behind the rows, not the rows; run the four
> gap-detectors and the design-reviewer on Opus; bring every Settle fork to me one at a time beside a
> robust-advocate's case; fill the gate-record; diff the gate-record's rows against the roadmap's Grouped
> scope before decompose. Zero schema-hash movement unless the Settle says bump in those words.
>
> Then build M52 with the milestone-build workflow: `args: { milestone: 'M52', base: '<HEAD after
> planning>', model: 'opus' }`, re-passed on every resume. Halt on any genuine fork the settle record does
> not answer. Report the audit's findings before fixing them; fixers one at a time; then the VERDICT under
> `completions/artifacts/M52/`, and only after the audit's fixes the bump to `1.0.0-rc.16`, the install
> (`cargo build --release && install -m755 target/release/jigc ~/.local/bin/jigc`), and the goldens
> checked to move by the version string only.
>
> Then re-run the per-axis review on the installed rc.16 with the preserved instrument
> (`completions/artifacts/M51/per-axis-review/instrument/`), persist it under
> `completions/artifacts/M52/per-axis-review/`, compare it row by row against M51's, and stop — the
> 1.0.0 call is mine.
