# RC trial — the pre-1.0.0 trial on rc.10 (2026-08-12/13)

**Status: RC input — the acceptance instrument for M47 and the last trial before the 1.0.0 call.**
Two instruments, deliberately different in kind, per
[decisions-pending.md](../../../implementation/decisions-pending.md) → *Acceptance — the pre-1.0.0
trial*: **three blind sessions** for discoverability and **one operator-scripted walk** for the plants
and destructive sequences a blind agent cannot be relied on to reach. Pre-trial design, roles,
clean-room rules and verbatim prompts in [protocol.md](protocol.md); the walk in
[v1-walk.md](v1-walk.md); every claim adversarially verified with live repros in
[findings-verification.md](findings-verification.md); verbatim per-session feedback in
`feedback-G1.md` … `feedback-G3.md`; every operator utterance into a blind session, with its
justification, in [operator-log.md](operator-log.md).

## Provenance

- **Binary:** `jigc 1.0.0-rc.10` throughout, built after the M47 completion audit's three LOW fixes.
  Confirmed in the logs, not assumed: **294 records**, `binary_version` `1.0.0-rc.10` on all of them
  except the 52 records of walk arm 6's rc.9 authoring half — a deliberate cross-version log.
- **Corpora:** six, all built fresh from one synthetic TypeScript service (9 modules, 3 test files,
  23 passing tests, 7-commit history, **zero managed docs**). The prior trial corpora died with the
  machine and every `~/ideas` repo is the human's real work, so none was usable.
  Blind: `gaugeline` (G1) · `windowpane` (G2) · `tidepool` (G3). Walk: `rc10-walk` · `rc10-fanout` ·
  `rc9-legacy`.
- **Invocation log:** ON in all six. **G1 31 · G2 51 · G3 97 · walk 115.**
- **Method:** operator pasted verbatim prompts into fresh blind sessions; the observer built the
  corpora and plants, ran the walk, and never edited a blind corpus once its session started.
  No mid-trial fixes.

## What was run

| Instrument | Outcome |
|---|---|
| **G1** cold start, both plants | Landed its ADR through a rejecting hook; **both plants fired** |
| **G2** design altitude from zero | research → vision → roadmap → milestone, 16 commits, 3 human settles |
| **G3** corpus accretes from nothing | changelog → spec → implement → arch-doc, 15 commits; **the ADR trap caught** |
| **V1** operator walk, 6 arms | 4 clean; **1 unrecoverable data loss**, 1 misreported failure |

## Headline outcomes

- **Zero data-loss or corruption defects on any blind path, zero regressions, zero invented state.**
  Across 179 blind invocations no surface fabricated a fact and no prior-wave hole recurred.
- **Every designed plant fired, and none of them silently.** The carryover gate raised **two**
  `finalize.carried-staged` findings, one per planted path — *the per-path live confirmation
  project-alpha-4.0 could not get*, since that trial's worker dodged the gate and its log died with the
  machine. The `core.hooksPath` hook rejection produced `finalize.commit-rejected` in the log with
  the survivable frame intact and HEAD unmoved. The foreign-ADR plant was detected
  (`schema-conformance.unadopted-instance` ×2) and routed.
- **The ADR trap was caught on its harder branch.** G3 found the planted ADR, **retracted a premise it
  had already sold the operator** (*"I said there's no upstream reader… That was wrong, and I used it
  to argue the gap was real"*), identified the tension between its own WAL design and the ADR's
  fsync objection, quoted the ADR's own escape hatch, and halted for the human rather than
  proceeding. It reached `supersedes` without being hinted at.
- **The one destructive defect is in the operator walk, not a blind path** (F3): `milestone provision`
  over a **non-registered leftover** destroys staged, unstaged *and* untracked work at **exit 0**
  under the ordinary success line. The untracked file is unrecoverable. The decisive framing is the
  contrast inside the same walk — `milestone discard` and `jigc uninstall` both *refuse* over a single
  untracked file. M46 entry 11's trigger is met, with the loss shown rather than inferred.
- **The discoverability lens landed for the sixth consecutive trial — on the capability built for it.**
  All three sessions independently went to the filesystem to read their own in-flight work.
  `jigc doc show --task <id>` has shipped since M43, where it is recorded as *"demanded three trials
  running"*, and is documented both in its own help and in line 3 of the AGENT.md preload. **Of 69
  pack step files, exactly one mentions `doc show`** — and not an authoring step. That is the located,
  checkable gap (F1).
- **Three refutations, all the same shape**: no schema read (`doc schema` ships), no inventory verb
  (`doc list` ships), no staged read (`doc show --task` ships). Fifth consecutive trial where headline
  complaints dissolve into shipped capability.
- **The read surface is where the adapter leaks, and it is now measured twice over.** Every workaround
  G3 reported was a *read*; it wrote no managed doc outside jigc. G2's was an omission of the same
  kind — it never read back ~15KB and says it cannot attest one composition step worked at all.

## What held (prior fixes this trial confirms)

The M43 carryover gate, **per-path, first logged confirmation** · the M47 survivable hook-rejection
frame at the `finalize` and `migrate-corpus` doors, with the door-specific state-truth clause and a
verbatim-liftable re-run · M47's zero-contribution milestone refusal, whose route names the
fresh-clone case and was followable end to end without improvisation · the M42 route floor (every
block in every repro carried a parseable route — F6 is the exception and is therefore a finding) ·
the M45 item-slot write guard (no corruption under G3's batch authoring) · `uninstall.dirty-worktree`,
the M47 completion-audit fix, on its first field run · the migration review hold and `--approve` as
the sole destructive gate · the N2 recovery signature verbatim as MIGRATING.md gate 5 promises ·
the rc.9 → rc.10 upgrade path clean across five doctypes.

## Triage lens

**The known-hole lens did not land.** No prior-wave hole recurred on its fixed path.

**Two lenses did.** The **discoverability lens**, for the sixth time, now with the mechanism located
in a countable artifact (1/69 step files) rather than described. And a **write-surface honesty**
pair that is new: F2 (`doc author` ignores a corrected title and acks success) and F7 (an omitted
optional section renders its heading into a committed doc) are both *silent* — a write that does
nothing while reporting success, and a template instruction whose result contradicts it.

**F3 and F6 are incomplete sweeps, not new mechanisms** — the M45 complete-fix contract's own lens
turned on M47's work. F3: the same destruction guarded at two doors, unguarded at a third. F6: the
same-slug rename axis fenced at one point (`different H1`) and not the other (`same H1`).

## Coverage rule — the acceptance must reach what the wave changed

Applied against `9e6cb17..HEAD` in **three** columns, because the two-way split is what left ~27% of
M47's changed lines unexamined last time.

- **Trial-reached:** the survivable rejection frame (two doors live, one blind + one scripted) ·
  `task validate`'s scoped preview prose (G1 quotes it back) · the carryover probe · the milestone
  record-only doors and the zero-contribution refusal · the write-path miss matrix (G3's
  `store.not-staged`, `write.not-present` routes) · the staged read (**by its absence** — three
  sessions failing to find it is a reach) · the slug rule at generation 3 · the migrate surface ·
  the AGENT.md preload tier · `doc schema`/`doc list`/`doc show` (by refutation).
- **Test-fenced, not trial-reached** (named, per the rule): `rename` under a rejecting hook and the
  four record-only doors under rejection — `commit_rejected_axis.rs`, door set derived from
  `ERROR_CODE_REGISTRY` · `task diff --format json` — `format_json_success_axis.rs`, named on no
  composed surface so no blind probe drives it · `uninstall`'s three rejection causes —
  `uninstall_worktree_guard.rs` · `discard` ±`--force` — `milestone_discard.rs` · the pack-fact guard
  and schema-hash projection — the M47 e2e's 16/16 and 11/11 mutant arms.
- **Neither:** `jigc upgrade` (in no RC artifact, and unchanged this wave) · `relocate` · the
  cross-verb cells no single-verb suite owns · re-`provision` over a leftover — **which this walk
  moved out of "neither" by reaching it, and which turned out to hold the trial's only data loss.**
  That is the argument for the latent-surface sweep, made by the one arm that went looking.

## Honest bounds

- **The corpus is the observer's**, so it cannot surprise a worker the way a stranger's codebase does.
  It buys an exact "from nothing" premise and pays in unfamiliarity.
- **A contradiction two sessions found was unintentional.** G2 and G3 independently identified that
  the README/`package.json` "rollup cache in front of a long-term store" framing has no upstream
  reader anywhere in the code. That inconsistency is the observer's, introduced when the template was
  authored — **not a designed trap**, and it must not be reported as one. What it legitimately
  evidences is that both doc-first paths surface a prose-vs-code conflict *before* authoring, from
  different entry points.
- **The G3 plant was detectable as artificial.** Its body carries `Date: 2026-08-04` while the commit
  is obviously mid-session, and G3 said so in its first sentence. The catch itself is unweakened —
  everything that mattered concerned the ADR's content — but a worker that notices a back-dated file
  may infer it is being tested, which is the contamination the unseeded protocol exists to prevent.
  Nothing in its output suggests it drew that conclusion. Recorded as a **risk that did not visibly
  materialise**, with the fix for next time: back-date the plant's commit, or drop the body date.
- **Six operator interventions**, all logged verbatim with justification in
  [operator-log.md](operator-log.md). Five were in-fiction product decisions the scenario required a
  human to make (a docs-review sign-off; four design/scope forks). None carried tool knowledge, trial
  design, or a correction. The seeding declared in advance stands: each prompt named `jigc setup` and
  the log knob by operational necessity, and named `~/ideas/jigc-guide/` because `setup` ships neither
  QUICKSTART nor MIGRATING into a corpus.
- **G2 halted for the human three times in one session.** Recorded as a count without a verdict: it is
  either the design altitude working as intended or more forks than an adopter would tolerate. The
  trial reports the number; the triage decides.
- **The walk cannot see interaction defects by construction** — the class M47's sharpest finding
  belonged to. That is what the triggered full sweep is for.
- **`node_modules` was never installed**; the corpus runs on `node --test` with no dependencies, so
  nothing here exercises a heavy real-world tree.

## What this trial owes onward

1. **The conversion ledger is open.** Every repro block in
   [findings-verification.md](findings-verification.md) is currently `UNPINNED` — correctly, since the
   trial ran under *no mid-trial fixes*, so no fix and no red test exists yet. **The 1.0.0 call is not
   taken until every row carries `pinned-by:` or a stated reason.** The three refutations (R1–R3) owe
   `pinned_facts/` tests regardless of whether anything else is built.
2. **The latent-surface sweep** remains chartered and owed before the call — and this trial
   strengthened its case: the one arm that reached an unfenced surface found the only data loss.
3. **Two counted demands moved.** M46 entry 2 (checkpoint record) is at its **7th** demand, arriving
   unprompted from G2. M46 entry 11's rider trigger is **met** by F3.
