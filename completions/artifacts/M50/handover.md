# M50 — handover to the planning session

**Read [charter.md](charter.md) first.** This file is the state of the world, the order, and the
traps. Written 2026-09-04, the same day as the trial it rests on.

## State of the world

| | |
|---|---|
| Branch | `main`, clean, **pushed** through the trial's last commit |
| Binary under test | `1.0.0-rc.13` from **`979baca`**, image `jigc-gate:rc13`, gated ([../RC-m50/gate-rc13.json](../RC-m50/gate-rc13.json)) |
| Host binaries | `target/release/jigc` **is** HEAD's tree (built 20:03). **`~/.local/bin/jigc` is stale** — Sep 1, predates `1799a2d`, same stamp. Reinstall before any host probe: `cargo build --release && install -m755 target/release/jigc ~/.local/bin/jigc` |
| Gate | green at HEAD before the trial (3171 / 0 / 17); the trial changed no Rust |
| The 1.0.0 call | **blocked by W-13** until M50 lands; then the human's |
| This repo | still **not self-hosted** — the port follows 1.0.0 (M49's order stands) |

**The trial:** [../RC-m50/](../RC-m50/) — protocol, corpora, plants by reference, prompts, answer
key, operator log, rehearsals R3/R4, the walk records, [session-findings.md](../RC-m50/session-findings.md)
(leads, in arrival order), [findings-verification.md](../RC-m50/findings-verification.md) (13
CONFIRMED · 0 REFUTED · 5 instrument, every row with a repro block and a `pinned-by:` or a stated
`UNPINNED`), [coverage.md](../RC-m50/coverage.md), [next-wave-scope.md](../RC-m50/next-wave-scope.md),
and [evidence/](../RC-m50/evidence/) — every session's two scored channels, archived.

## What the human decided, 2026-09-04

- **The 1/3 on the duress cell is read as a pull-tier finding on state, not as the invariant
  failing.** Every worker repaired the doc correctly through jigc; the bypass is on the first read
  of someone else's working area; the cause is named twice and is answerable. The decision the
  trial could not make is **routed into M50** (F-5, and fork 2 on the read verb) and **re-measured
  by the next trial's duress cell**.
- **M50 takes the empty-id class at Tier 0**, over its seam, and **decides the read-verb question
  on its merits** — neither by reflex nor by the port's convenience.

## Do these in this order

1. **Reinstall the host binary from HEAD**, then **drive W-13** on a rig
   (`dev/jigc-rig refs-post-hoc --binary target/release/jigc`; `jigc task discard ""`). The whole
   first half of the claim rests on `TaskArea::resolve` being the one seam — the baseline must say
   whether it is, over walk 17's 25 doors, before anything is cut.
2. **Baseline against the binary, not the charter** — capability-auditors over the destroying
   doors, the id-taking doors, the orientation/router surfaces, `read_pack`'s call sites, the
   placement-root guard. Each charter row is a lead.
3. **Settle the eight forks** ([decisions-pending.md](../../../implementation/decisions-pending.md)
   → *M50*), a robust-advocate first for every cheap cut. Fork 2 (the read verb) and fork 4
   (`AddedNestedRepeatable`) are the two where the human's criterion and the razor's *necessary*
   leg pull differently — argue both, decide, record.
4. **Fill the planning gate-record** the way M49's is built, then **decompose**, risk-first: the
   Tier 0 seam before anything that touches `task discard`; W-14's guard before any placement
   work; the Fix-phase increment (if fork 8 takes it) before the completion audit that would use it.
5. **The next trial's protocol** inherits RC-m50's instrument as fixed (I-1..I-5) and keeps the
   duress cell as its headline; the acceptance of the claim's second half lives there.

## Traps

- **An agent's report is a lead, not a measurement — and so is the orchestrator's.** This trial's
  blocking finding was on the orchestrator's own screen (*"task list after: no active tasks"*)
  hours before the arm found it, read as an ack. Two of the reader's five self-defects sat on the
  duress cell. Drive every premise; read every capture.
- **The rig drives the debug binary by default.** `#[cfg(debug_assertions)]` fences panic there and
  do nothing in release; the handover before this trial carried a debug artefact as a product
  fact. `--binary target/release/jigc`, always, and say which posture in the record.
- **Two trees stamp rc.13.** `jigc --version` cannot tell the host-installed binary from HEAD. Check
  `ls -l` against the last `crates/*/src` commit; build images from a sha.
- **Never run two tree-mutating agents on one working tree.** Broken at M38, M42, M49. Fork 8 is
  the fix; until it lands, the rule is prose.
- **`~/out/<name>` is shared across trials.** `run-session.sh` refuses correctly; `observe` on the
  refused dir scores the *old* evidence silently (I-2, open). Prefix every out-dir with the trial.
- **`.jigc/AGENT.md` is not in a session that runs `setup` itself.** B1's read-back is discounted
  for that reason and reported separately; a future arm that wants the adapter loaded starts adopted.

## What is already verified, so you need not re-derive it

Every row of [findings-verification.md](../RC-m50/findings-verification.md) carries the command
and output; the ones the charter leans on:

- `task.rs:763` is the join; `DESTROYING_DOORS` (`milestone.rs:2397`) has four members and
  `task discard` is not one; `no_such_task_route.rs:141` is the nearest test and drives a
  non-existent directory, so `""` (an existing one) is outside it.
- `milestone.rs:4106` (`fn blocked`) prints `finding.message` raw; `render.rs:2642` is the house
  form; `milestone_boundary_gate.rs` drives JSON only.
- `write_miss_shape_axis.rs:854-885` — both `set-field` rows carry a leaf; the completeness fence at
  `:1330` is satisfied by them.
- `untrackable_home_axis.rs:272` pins `.jigc` as *accepted*, on purpose (`config.rs:365-378`).
- `start.rs:3565` `read_pack`'s closure holds the resource id only.
- `describe.rs:499` pins the `pack` key on `--commands`; `describe`'s definitions arm carries none.
- `jigc start` and `jigc start "<intent>"` over one live task: zero mentions (driven on
  `refs-post-hoc`).

## Apparatus owed — trial tooling, not this wave

1. **PT-D** — the corpus template's `IngestQueue` is dead on the live path; four workers found it
   this trial. Wire `tick()`, add the `check-corpus.sh` bar.
2. **I-2** — `observe` over a refused (pre-existing) out-dir should refuse too, or say whose
   evidence it is reading.
3. The FILESYSTEM heuristic reads the **main** transcript only; B2's subagents ran after the
   repair and read nothing managed, but a future arm's subagent could. `observe` should walk
   `subagents/*.jsonl` and label the hits.
