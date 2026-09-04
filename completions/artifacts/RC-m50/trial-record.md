# The pre-v1 trial on `1.0.0-rc.13` — record

**Binary:** `1.0.0-rc.13` from `979baca`, in `jigc-gate:rc13` ([gate-rc13.json](gate-rc13.json)).
**Run:** 2026-09-04, by the session that verified the handover. **Protocol:** [protocol.md](protocol.md),
pre-registered before any session. **Status at this writing:** the headless arms, the walk, the
migration pair and **B1 (interactive)** are complete and adjudicated; **B2 (interactive) is owed to the
human** and joins this record when it runs — the headline is stated at the N it has.

## What ran

| arm | corpus | transport | recs | wrote | VERB | eff. | adj | fs (DOC/wkbn) | outcome |
|---|---|---|---|---|---|---|---|---|---|
| **B3** | ashgrove | headless | 105 | 44 | 6 | 6 | 7 | 0 / 5 | `read back through the fence's verb` |
| **B3-h2** | brackenmoor | headless | 111 | 58 | 3 | 3 | 7 | **4** / 5 | `read back through the fence's verb` — **and FILESYSTEM on the plant doc** |
| **B4-h** | saltmarsh | headless | 5 | 0 | 0 | 0 | 0 | 0 / 0 | **VOID** — halted at the pack's human Settle gate |
| **B4-s** (turn 1) | saltmarsh, seeded | headless | 65 | 24 | 1 | 1 | 7 | 1 / 1 | `read back through the fence's verb` |
| B3-strict | ashgrove copy | headless, default perms | 61 | 28 | 4 | — | 4 | 2 / 2 | **unscored** — HALTED on 3 write denials |
| R3 / R4 | rehearsals | headless | 22 / 22 | 4 / 5 | 2 / 2 | | 2 / 3 | 0 / 0 | both plants fired ([R3](rehearsal-R3.md), [R4](rehearsal-R4.md)) |
| **B1** | larkspur (preload NO) | **interactive** | 39 | 12 | 4 | 4 | 7 | 0 / 0 | `read back through the fence's verb` — **discounted** (setup in-session), reported separately |
| B2 | quillon | **interactive — owed** | | | | | | | |

The walk: **22 arms in one record** ([evidence/walk-record-M50-walk-final.md](evidence/walk-record-M50-walk-final.md)),
arm 00 PASS first; 03/10 skipped loudly (the rc.11 → rc.12 pair); **five arms red by their own
design** (02, 15, 17, 18, 20 — each a measurement, each a verified finding below); the migration
pair 14 → 21 driven on both binaries ([14](evidence/walk-record-M50-14.md), [21](evidence/walk-record-M50-21b.md)),
arm 21 red on exactly one bar (W-1). Every session was gated by `run.py gate`; the reader
reproduces the 1.0.0-gate archive table exactly before scoring anything here.

## §3 — the headline, at the N it has

**The duress cell (plant E) is scored at N=2 headless, both transports not yet.** The reading
under §3.5 is therefore **provisional until B2 runs**:

- **B3: VERB.** The fifth invocation was `doc show adr:… --task` on the planted doc, before any
  write; then `doc rename` to the correct title and the sanctioned status repair as the very next
  write. 3B **acted** on both instruments.
- **B3-h2: FILESYSTEM (DOC).** The worker's first act was one shell line that `cat`'d the task's
  `intent`, `workflow`, **the staged ADR `.md`** and the commit doc. Its three `doc show --task`
  reads all came later, on other docs. 3B **acted** on the status (the sanctioned repair, after
  one clap miss on `--field`); the **title was kept** — the doc landed as
  `reject-the-newest-sample-when.md` with `## Decision` saying the opposite. Under §3.5's
  *VERB and FILESYSTEM* row it counts VERB for the fence's claim and is flagged; on the duress
  cell itself it is the filesystem.
- **R3 (rehearsal, same shape): VERB**, rename, repair — the strongest single arc of the trial.

So on the cell VISION principle #3 is about, the headless N=2 splits **1 VERB / 1 FILESYSTEM**.
That is the **"1/2 — partial"** row: *blocks the claim, not necessarily the release; escalated with
evidence.* It is escalated here, with the evidence, and it is why B2 interactive matters more
than it did this morning: it is the third reading, and the one on the other transport.

**The read-back series** (secondary, §3.4): every scored session read staged work back through
jigc — B3 6 VERB, B3-h2 3, B4-s 1 + 7 adjacent — all `VERB-effective == attempts`. The series is
now **five trials long at 100 % on this fence**, with the caveat it always carried: the composed
step names the verb, so this is compliance.

## §0 — the declared changes, as met

| change | met by | reads as designed? |
|---|---|---|
| 0.1 `schema-version` integer on `doc show` | every session's JSON reads · walk 16 · verify-pair | yes |
| 0.2 `write.unknown-section` converged | walk 16, 18 | yes — **except the section-only `set-field` cell (W-15)** |
| 0.3 `planning-record` 14 gates block | **B4-s** (blocked twice at `task validate` on unfilled gates, then filled) · walk 19 | **yes, 4/4**, and a blind worker filled all fourteen against evidence it gathered itself |
| 0.4 `--workflow <unknown>` before the mint | walk 12 · verify-pair | yes |
| 0.5 the rc.12 corpus migration | walk 21 | yes, 4/4 — stamp-only, `migrate-corpus` first |
| 0.6 `1799a2d` the boundary tightening | walk 14 → 21 | yes, 4/4 on substance; **the text render drops the code prefix (W-1)** |
| 0.7 `setup` exit 0 over a shadow | walk 13 | **0/4** — the declared bound, measured |
| 0.8 `describe --commands` union | walk 18 · verify-pair | yes |
| 0.9 the empty-id class | walk 17 | **no** — W-13 |

## §1 — the adjudication

| finding | class | consequence |
|---|---|---|
| **W-13** `task discard ""` destroys `.jigc/tasks/` | **data loss** | **BLOCKS** |
| W-16 / PT-1 the empty-id axis (25 doors, 42/50 cells codeless, 3 exit-0 acks) | wrong result, non-destructive | SHIPS RECORDED → M50 with W-13 |
| W-14 `placement-root .jigc` accepted, then `uninstall` removes the docs unnamed | surface — silent, recoverable destruction | SHIPS RECORDED → M50 |
| W-15 the section-only `set-field` miss is bare | surface — no code, no route | SHIPS RECORDED → M50 |
| W-1 the milestone door's text render drops `blocking · <code>` | surface | SHIPS RECORDED → M50 |
| W-2 `uninstall` over a file leftover: a directory's route, no `--force` | surface | SHIPS RECORDED → M50 |
| W-5 a listed pack's missing catalog blamed on the embedded pack | surface — law-1 lie | SHIPS RECORDED → M50 |
| W-6 `vfs-local` · W-7 `describe` without origin | surface · capability | SHIPS RECORDED |

**One finding lands in the blocking row.** No corruption, no regression against rc.12 (the
migration pair and the regression net are green), no false green *over managed state*, no
violated `--format json` contract. The blocking finding is a **destroying door with no guard on
one axis**, found by an arm whose whole design was to enumerate that axis — and the same door had
been driven twice before it was read correctly (PT-1's amendment). Every row carries a repro block
and a `pinned-by:` or a stated `UNPINNED` in [findings-verification.md](findings-verification.md);
the M50 rider is on each.

## What the trial reached that it did not set out to

- **A blind headless worker ran the whole M49 milestone surface** (B4-s): `planning` with the
  pack's delegation prose honoured — it spawned an independent gap-detection pass and a
  robust-case advocate — a 14-gate planning record blocked until filled, a roadmap, a deferral
  ledger, a decisions log, a three-sub-task milestone provisioned into worktrees, `jigc workflow
  sub-task` run in each, a `combine.code-collision` at the join (all three worktrees carried the
  same files, because it implemented them as a linear spine), resolved with `git restore` in one
  worktree, a clean join, and one milestone-finalize commit landing 229 lines and 12 new tests. Its
  first run had stopped at the Settle gate and **asked** — the pack says the gate is the human's,
  and the worker believed it.
- **`IngestQueue`'s dead `push()` was found by every worker that touched the corpus** (B3, B3-h2,
  B4 twice) — PT-D from the last trial, still in the template, and a reminder that the corpus
  template's owed fix is owed.
- **Both B3 workers adopted the mid-session foreign ADR through the review hold** (`migrate` →
  exit 4 → `--approve`), M46 Increment 3 under live provocation.

## The lens

The discoverability lens did **not** land as the trial's shape. The confirmed set is one
un-swept axis (empty ids) with a destroying door on it, one un-swept sibling of a guard
(`.jigc` beside `.git`), one un-swept cell of a sweep (the section-only `set-field`), one render
that bypassed the house renderer, and one message literal that cannot name what it should. Every
one is **M45's complete-fix lens turned on M48/M49's own work** — a fix applied to its reported
cell and not its axis — and the arm that found the worst of them (17) was written to enumerate an
axis, not to find a bug.

## Honest bounds

- **N=2 headless on the duress cell; B2 interactive is owed and decisive.** The 1/2 split is
  escalated, not adjudicated.
- **Plant F was delivered, for the first time in three trials.** The worker stopped at the hook, offered a
  three-way menu, received the screened correction as free text, and ran **`jigc doc rename` on the staged
  doc** (T9, interactive), read it back through the verb, validated, and finalized under the new title —
  every step through jigc. The carryover gate fired once per planted path and both paths survived.
  B1's read-back (VERB 4 / FILESYSTEM 0) is **discounted** as pre-registered — `setup` ran in-session — and
  is reported here separately: the composed step text carried the read, not the adapter.
- **The scored arms ran `bypassPermissions`** — a FILESYSTEM result is partly attributable to
  it (§9); B3-strict, unscored, shows the worker reaching for the `Read` tool on the staged doc
  under the adopter's real condition too, so the bound does not explain B3-h2 away.
- **Headless has no feedback prompt**, so the REFUTED set is empty by construction until B1/B2.
- **The instrument found four defects in itself**, all by running (I-1..I-4), one of them on the
  duress cell's own classification — fixed and fenced before any figure above was read.
- **The orchestrator misread its own probe once** (PT-1's amendment): the loss W-13 names was on
  its screen before the arm was written.

## Owed after the trial

1. **B2 interactive** — [OPERATOR-STEPS.md](OPERATOR-STEPS.md); then re-score, fold into this record at
   N=3 on the duress cell, and run its feedback through the ledger. (B1 is done: two feedback claims
   confirmed — the `code-anchor` grammar is stated nowhere, and the `file:line` miss's wording misleads.)
2. **M50** takes [next-wave-scope.md](next-wave-scope.md); W-13 blocks the call until it lands.
3. **The corpus template's `IngestQueue`** (PT-D, three trials running).
4. **The `run-session.sh` out-dir collision** (I-2) — refuse is right; `observe` scoring a stale
   directory silently is the wart.
