# The trial that follows M50 — handover

**Written 2026-09-09 by the session that planned, built and audited M50** — which is exactly the
position that produces confident wrong premises. Every row below was **driven or computed at the
time of writing**, not recalled; the ones the orchestrator verified personally are marked. Verify
the rows you lean on before you trust them: [RC-m50's handover carried two rows that did not
hold](../RC-m50/handover.md), one of them a debug-only panic carried as a product fact.

**The directory name is provisional.** `RC-rc14` names the binary under test. Rename it if the
trial's shape earns a better one (`RC-1.0-gate` and `RC-alpha4` took purpose and corpus names
respectively); if you do, move this file rather than copying it.

---

## State of the world

| | |
|---|---|
| Branch | `main`, **clean**, pushed through `95c79be` |
| HEAD | `95c79be68ab6873c788947f58190a0fd21f5d990` |
| Binary under test | **`1.0.0-rc.14`** |
| `target/release/jigc` | sha256 `99f226bb2e7dc67d42a1e70153afe3ed1714176eb0d1cc42a0b914d26feefdc5` |
| `~/.local/bin/jigc` | **byte-identical** to the above (same sha256, verified) |
| Is the binary HEAD's tree? | **Yes** — `find crates packs -type f \( -name '*.rs' -o -name '*.yaml' -o -name '*.toml' \) -newer target/release/jigc` returns **empty** |
| Gate at HEAD | **PASS · 3341 passed / 0 failed**, all five steps (probe · fmt · clippy · build · test), re-run **after** the version bump and the golden regen |
| The 1.0.0 call | **not taken** — deliberately, and it is the human's |

**Why the sha256 row exists.** `jigc --version` cannot distinguish two trees that stamp the same
version — that trap cost the last trial a corrected handover row. The hash comparison settles it
without trusting the stamp. **Build any container image from a pinned sha, not from a stamp.**

---

## What M50 changed — do not re-derive it, and do not skip it

**The declared behaviour-change list has one home:**
[decisions-pending.md](../../../implementation/decisions-pending.md) → *The trial that follows M50 —
protocol inputs*. It was derived at the wave's close **from the twelve increment records rather than
from memory**, each item naming the increment that landed it. It is not restated here.

**Read it before briefing the first session.** Several items are met within minutes of any arm and
an unbriefed observer scores them as regressions — the near-miss that has already happened twice.
The four most dangerous to misread:

1. **Two intended exit-0 → non-zero flips.** `jigc task validate ""` answered *the task validates
   clean* and `jigc doc list --task ""` answered *no docs staged* at exit 0 through rc.13. Both now
   block. That is the wave's headline fix, not a regression.
2. **`engine::result::SCHEMA_VERSION` 2 → 3** — the wave's only result-contract bump. A driver
   pinned to 2 is the one adopter-shaped consumer a trial can contain.
3. **A fan-out that landed clean on rc.13 may now block at exit 3** (`1799a2d`, landed *before* the
   wave). It is the intended tightening and it is met **late in a milestone arm**, where a misread
   costs the most.
4. **The deny floor gained `Bash(jigc uninstall:*)` and `Bash(jigc milestone discard:*)`.** A
   worker's call to either is blocked **by the harness** — the transcript shows a permission denial
   and *no jigc output at all*. An observer counting friction must not score these two, and a walk
   arm needing either **must run it outside the agent**.

---

## The measurement this trial exists for

**The duress cell is the headline, and the reading it must beat is 1/3.** RC-m50 landed 1 VERB /
2 FILESYSTEM on plant E across two transports; the human read it as a pull-tier finding on *state*,
routed it into M50 (F-5 and the orientation fork), and owed a re-measure here. Keep the **same
plant**, the **same N=3-across-two-transports shape**, and compare against **1/3** — not against
the read-back series, which has stood at 100 % effective for five trials and buys nothing more.

**What M50 shipped against it** — so you can tell a fix that worked from a fix that did not fire:
`jigc start` no longer calls a repo with a live task clean, both minting `start` forms append an
`also open:` block naming every open task with its resume/validate/finalize/discard directives, and
the active-task view carries `findings`. That is the **push** half of F-5. Whether it moves a worker
who has not been told to look is the whole question, and it is unprovable from inside the wave —
[VERDICT.md](../M50/VERDICT.md) says so in those words.

**One instrument rule, learned the expensive way** ([cue-card-postmortem](../RC-1.0-gate/cue-card-postmortem.md)):
*an instrument fires reliably iff its trigger is a **state**, its consequence is re-raised by the
product, and every worker behaviour maps to a scored outcome.* That is why plants work and cue cards
do not. Plant a state; never schedule a sentence.

---

## Apparatus owed before the first session — three items, all triggered

Each is recorded in the protocol-inputs entry with its trigger already fired. They are small, and
each one has a named failure it prevents:

1. **`observe` walks subagent transcripts.** It reads only the largest `.jsonl` under
   `.session-transcript/` — the main session. A worker that delegates its orientation moves the
   FILESYSTEM channel into files the reader never opens, and the cell scores clean for the wrong
   reason. Fix in `driver/observe.py` with a test over a synthetic `subagents/` tree.
2. **`observe` over a pre-existing out-dir says whose evidence it is reading, or refuses (I-2).**
   `run-session.sh` refuses correctly; the reader then scores the *previous* trial's evidence at
   that path **without a word**. A `PROVENANCE.txt` image/sha check against the round's gate record
   closes it. **Prefix every out-dir with the trial name regardless** — `~/out/<name>` is shared
   across trials.
3. **The corpus template's `IngestQueue` (PT-D, third trial running).** `push()` is called on no
   live path; four workers found it last time and two filed it as a deferral. Wire `tick()` into a
   live path and add the `check-corpus.sh` bar that fails when a symbol a plant depends on is
   unreachable. *A plant whose subject is dead code has "the worker fixes the code instead" as its
   falsifier.*

---

## Trial tooling — all four homes present, checked

Do not rebuild any of these. Verified present at the time of writing:

| home | what it does |
|---|---|
| [completions/trial-corpus-template/](../../trial-corpus-template/) | instantiates a **foreign, pre-jigc** corpus (zero managed docs) + `check-corpus.sh` |
| [completions/trial-harness/](../../trial-harness/) | builds and verifies the **isolated container**; `build-image.sh <sha>` pins the binary by commit |
| [completions/trial-driver/](../../trial-driver/) | drives and scores a session without an operator; `observe`, `seed`/`fork`, `plant`, `gate`, `walk.py` |
| [completions/workflow-eval/](../../workflow-eval/) | the older per-workflow agent-run eval |

**The boundary between the three corpus builders**, since they are one grep apart: the template
builds **foreign** corpora (what an adoption trial adopts *from*); `dev/jigc-rig` builds **adopted**
ones for a shell probe; `crates/cli/tests/support/trial_corpus.rs` is that same adopted builder for
the Rust suites. A trial reaching for the rig is asking the wrong question.

---

## Traps

- **An agent's report is a lead, not a measurement — and so is the orchestrator's.** RC-m50's
  blocking finding sat on the orchestrator's own screen hours before an arm found it, read as an ack.
  Drive every premise; read every capture.
- **The rig drives the DEBUG binary by default.** `#[cfg(debug_assertions)]` fences panic there and
  do nothing in release; the handover before the last trial carried a debug artefact as a product
  fact. Pass `--binary target/release/jigc` **always**, and say which posture in the record.
- **`~/out/<name>` is shared across trials** — see apparatus item 2.
- **`.jigc/AGENT.md` is not in a session that runs `setup` itself.** An arm that wants the adapter
  loaded starts adopted; a read-back measured in a self-setup session is discounted.
- **Never run two tree-mutating agents on one working tree.** Broken at M38, M42 and M49. M50 shipped
  the fan-out Fix phase as the fix — **and M50's own audit round did not use it** (its four fixes had
  overlapping write-sets, so the settled serial fallback applied). **The primitive's first real use
  is still owed**, which means the prose rule is what is protecting you until then.
- **Orientation now does real work.** The active-task view's `findings` key shells out to git, spawns
  the `doc-code` probe and materializes `.jigc/index/edges.json` — at the door the adapter binds to
  `SessionStart`. **A session's very first `jigc start` therefore writes.** Any arm that asserts a
  pristine tree after orientation must account for this.

---

## Already verified — you need not re-derive these

- The binary is HEAD's tree, and the installed copy is byte-identical to it (both above, computed).
- The gate is green at HEAD: **3341 / 0**, re-run after the bump and the golden regen; the golden
  diff was verified to carry **nothing but the version string**.
- M50's four audit findings were each **confirmed live before any fix and fixed axis-complete**;
  three of the four repros were re-driven by the orchestrator personally after the fix
  ([VERDICT.md](../M50/VERDICT.md) carries the commands and outcomes).
- The degenerate-id sweep over the archived RC invocation logs **came back empty** — no prior trial
  result was contaminated by an unnoticed destruction (Inc 13 / T3).

## Declared bounds M50 carries into this trial

State these to the observer, because each one is a place a finding is **expected** rather than new:

1. **`ArgToken::Plain` is unchecked from the outside** — the argument classification catches
   *forgetting* a new argument, never *mis-answering* one.
2. **60 host-path producers remain**, across 11 files, each with a measured count in
   `UNSWEPT_PRODUCERS` so closing one reddens the row. Two are genuine law-1 siblings left
   deliberately (`engine::finalize`, `cli::start`).
3. **`rename`'s flattening door predates the wave** and stays outside the findings envelope.
4. **The wave's second claim-half is unproven inside the wave.** That is what this trial measures.

## What this handover does not claim

It does not claim the orientation change works — only that it shipped and that the lie it replaced
is gone. It does not claim the four fixes are complete over classes nobody has driven yet; four
consecutive waves have had audit findings that were **larger classes than reported**, and that
pattern is the strongest single reason this trial exists. And it makes no claim at all about any
corpus: the RC-m50 corpora are **not** carried forward, and fresh ones are the human's standing call.
