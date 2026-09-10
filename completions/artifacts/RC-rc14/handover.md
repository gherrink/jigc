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

---

## Verified 2026-09-09, by the session that runs the trial

Every row above was driven or read at HEAD `21ffc0d4` before anything was built.
Corrections are recorded **here** rather than edited into the rows they correct, so the
handover still says what it said when it was written — the convention
[RC-m50's handover](../RC-m50/handover.md) → *Verified* set.

| Claim | Verdict | Evidence |
|---|---|---|
| HEAD `95c79be6…`, pushed through `95c79be` | **STALE; the conclusion survives and is now proven.** HEAD is **`21ffc0d4`**, pushed. Two commits landed after this file was written — `56df440` (this handover) and `21ffc0d` (a CLAUDE.md + decisions-pending edit). | `git rev-parse HEAD`; `git rev-parse origin/main` agrees; `git diff --name-only 95c79be..HEAD` → exactly `CLAUDE.md`, `implementation/decisions-pending.md`, `completions/artifacts/RC-rc14/handover.md` — **zero** `crates/`, `packs/`, `.rs`, `.yaml`, `.toml` |
| `target/release/jigc` sha256 `99f226bb…`, installed copy byte-identical | **HOLDS** | `shasum -a 256` on both → identical |
| *"Is the binary HEAD's tree? Yes"* — via `find … -newer` | **HOLDS for what the check can say, and the check is weaker than the row implies.** `-newer` proves nothing under `crates/`/`packs/` was *touched after* the build; it cannot prove the build is *of* that tree. The docs-only diff above extends the claim to the new HEAD. The trial does not rest on it either way: the binary under test is the one baked into the image from a **pinned sha**, and no host binary is scored on. | `find crates packs -type f \( -name '*.rs' -o -name '*.yaml' -o -name '*.toml' \) -newer target/release/jigc` → empty |
| **`also open:` … "with its resume/validate/finalize/discard directives"** | **FALSE — two surfaces conflated, and it is load-bearing.** The `also open:` block renders **one** directive per row: `` `<id>` (workflow `<x>`) — resume it with `jigc start --task <id>` ``. The **four** `Run:` directives (`start --task`, `task validate`, `task finalize`/`milestone finalize`, `task discard [--force]`) belong to the *orientation* `active-task` view — a different form of the verb. **Neither surface names a read verb**, so RC-m50's twice-named cause (*no read verb shows a task's whole staged area*) is not closed. Consequence: the protocol pre-registers the indirect mechanism (orient → resume → M48's read-back fence in the re-composed step) and its three outcomes, rather than meeting a null as a surprise. | `crates/cli/src/render.rs:541` (`also_open_block`) vs `:87-160` (`orientation_active`) |
| declared bound 2 — *"**60** host-path producers remain, across 11 files"* | **FALSE. 79 across 11 files.** `UNSWEPT_PRODUCERS` sums 11+22+20+9+8+3+1+1+2+1+1 = **79**. The const did not exist before `b34a8c72` — the audit-fix commit that also states 60 — so 60 was never right for the table it cites. The *per-file* counts **hold**: `the_unswept_remainder_is_counted_not_described` checks each against the source and the gate is green. Only the unchecked summary is wrong. Same error in [M50/VERDICT.md](../M50/VERDICT.md) → declared bound 2 and `DECISIONS.md` → 2026-09-09. | `crates/cli/tests/repo_relative_paths.rs:721`; `git show 52840e6f:…` has no `UNSWEPT_PRODUCERS` |
| *"The declared behaviour-change list has one home … derived from the twelve increment records"* — read it before briefing | **HOLDS as a pointer; the list it points at is FIVE sources short.** It pins its own derivation to *"the twelve increment records"* while `CLAUDE.md` says *"Thirteen increments shipped"* — the two documents contradict each other — and the completion audit ran **2026-09-09**, a day *after* the derivation. Unrepresented, each with a user-visible change: **Increment 13** (the discard prose in `QUICKSTART.md`/`MIGRATING.md`, `include_str!`'d into `SKILL.md`, so shipped bytes and `jigc-body-blake3` moved → re-`setup` over a locally-edited copy now hits refuse-to-clobber); **F1** (`milestone add-from-spec` over an outside-repo path now refuses — a door that succeeded now blocks); **F2** (the finding identity swept over **31** production sites, so ~29 doors that dropped their code from the **invocation log** now record it — the trial's own measurement channel, and rc.13-vs-rc.14 logs are not like-for-like); **F3** (law 1 widened from the destroying/provisioning doors to the shared predicate plus **4 read-path sites**, so text and JSON bytes on `store.not-found`/`store.unparseable` moved); **F4** (`milestone discard` over staged prose now blocks — and it is deny-floored, so an agent arm sees a harness denial and no jigc output). | `implementation/decisions-pending.md:189-206`; `CLAUDE.md`; [M50/VERDICT.md](../M50/VERDICT.md) F1–F4; `implementation/roadmap.md` → Milestone 50 (13 increments) |
| *"Seven defects driven at this wave's close are carried"* | **SIX. N31 is discharged and still listed as open.** Its stated trigger was *"the M50 completion audit's triage"* — that trigger **fired**, and audit finding F3 closed it: the `store.not-found` / `store.unparseable` loci now render through `crate::path::repo_relative`. `implementation/decisions-pending.md:544` still reads as carried, which is that file's own preamble failure — *"it fired once already and nobody noticed"*. | `crates/engine/src/store.rs:155`, `:288`; `crates/engine/src/milestone.rs:604`, `:621`; `crates/engine/src/path.rs:48` |
| trial tooling — four homes present, do not rebuild | **HOLDS, with five omissions worth knowing before you reach for them.** (a) `verify-pair.sh` has **no `m50` case and exits 2 on one** — it needs the set, exactly as the last two trials needed `m49`/`m46`. (b) **Every** driver default is `jigc-gate:rc11` (`run.py` ×4 subcommands, `walk.py`, `run-session.sh`, `verify-image.sh`); only `verify-pair.sh` tracks the current trial — pass `--tag` everywhere. (c) `run.py`'s README row and module docstring under-report by six: `gate`, `record-gate`, `carry`, `seed`, `fork`, `plant` all exist. (d) There are **seven** `test_*.py`, not the six the README claims. (e) The archived evidence dirs are **flat** — no `.session-transcript/` tree — so `run.py observe <archived dir>` finds neither channel through `_find`, contradicting `evidence/README.md`. | file reads across `completions/trial-driver/`, `completions/trial-harness/` |
| plant E reused byte-identical | **HOLDS, and stronger than stated: there is only one copy.** `RC-m50/plants/` holds a by-reference README, not a script. md5 re-computed live → `e8bcbee6ad930eeb65a2f9869c738add`, matching the recorded value. Note `driver/plants.py:fire()` copies the plant's **whole directory**, so anything added to `RC-1.0-final/plants/` rides along into every firing. | `md5` on `../RC-1.0-final/plants/e-abandoned-task.sh`; `RC-m50/plants/README.md` |
| trap — the rig drives the DEBUG binary by default | **HOLDS** | `dev/jigc-rig:621`; `crates/engine/src/finding.rs:737` is `#[cfg(debug_assertions)]` |
| trap — `~/out/<name>` is shared across trials | **HOLDS, and it is live, not hypothetical** — `~/out/` currently holds RC-1.0-gate's `B1`/`B2`/`B3` beside RC-m50's `M50-B1`/`M50-B2`/`B3-h2`. Every out-dir this trial writes is prefixed `RC14-`. | `ls ~/out` |
| `SCHEMA_VERSION` 2 → 3; both deny-floor entries | **HOLD** (source-level; driven again by the `m50` pair set) | `crates/engine/src/result.rs:27`; `crates/cli/adapters/claude-code.yaml:35-36` |

**One thing the handover does not say, established here and taken into the protocol: no
migration pair is owed.** `git diff 979baca..HEAD` over both `config/schema-manifest.yaml`
files and every `schemas/*.yaml` is **empty** — no doctype schema moved between rc.13 and
rc.14 — so an rc.13-authored corpus is current on rc.14 and the RC-m50 arm 14↔21 analogue
is unnecessary. Recorded rather than silently omitted.

**And one carried defect names this trial as its own trigger.** **N27** — `jigc task diff
<id>`'s cold-start form answers almost nothing — is keyed to *"the duress cell re-measure —
the next trial's plant-E arm"*, with the instruction that if a worker again goes to the
filesystem to read its own abandoned working area, N27 *"is the named candidate and is
re-argued against its cost, not re-discovered."* It is briefed in §0 or the headline is
uninterpretable. **N15** — a `--task` read of an unresolvable address is byte-identical to
the task-less one, and its route serves the *committed* copy to a reader holding a staged
one — carries a trigger whose second clause is likewise *a trial observation*.

**Consequences taken into the trial:** the image is built from `21ffc0d4` and its sha
recorded in the gate record; the protocol's §0 completes the behaviour-change list with
Increment 13 and F1–F4 before the first session is briefed; the headline's mechanism and
its three outcomes are pre-registered from the `also open:` correction; `verify-pair.sh`
gains an `m50` set; and the three owed apparatus items land before any corpus exists.
