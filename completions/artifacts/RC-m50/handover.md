# The pre-v1 trial — handover to the session that runs it

**State of the world at handover, then what is decided, then what is owed.** Written 2026-09-04.

## State

| | |
|---|---|
| Branch | `main`, clean, **pushed** through `d854e25` |
| Binary | **`1.0.0-rc.13`**, built and installed |
| Gate | green — `dev/gate`: **3169 passed / 0 failed over 17 test binaries** |
| M49 | complete + audited; 7 findings, all fixed axis-complete ([VERDICT](../M49/VERDICT.md)) |
| Since M49 | the milestone-boundary gate fix (`d854e25`) and the `dev/` tooling — both **on the binary this trial runs** |
| The v1 call | the human's, **after M50** |

## The sequence the human set

**gate fix → this trial → M50 → v1.** The gate fix is done. **M50 is the last milestone before v1**, and
**this trial's findings are its input.**

**M50's admission criterion is broader than M49's razor**, in the human's words: *everything that
improves usability, routing, bug-fixes — everything that makes the product better and more acceptable
at v1*, and above all **everything that becomes impossible or expensive to change once people start
using the product. Now it's cheap.** Note this drops the second, unwritten test M49 actually applied
(*"the port doesn't need it"*), which was never part of its leg 0 — so items refused on that ground are
**re-openable**, and at least one should be re-adjudicated (below).

## Decided about this trial

- **Fresh corpora only** (the human's call). A corpus that never met `rc.12` has nothing to migrate, so
  the blind sessions measure **the fixes**, not the migration.
- **The migration gets its own arm**, kept separate, because the record's own warning is now live:
  *"a schema bump makes the following trial about migration rather than about the fixes, which is the
  opposite of what that trial is for"* — and **M49 shipped three schema bumps plus a new doctype**.
- **Plants, not cue cards.** The 1.0.0-gate trial's cue card **never fired in four sessions** — the
  window between its trigger and the finalize that closed it was **3–5 seconds**. The rule it yielded:
  *an instrument fires reliably iff its trigger is a **state**, its consequence is re-raised by the
  product, and every worker behaviour maps to a scored outcome.* See
  [cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md).
- **Blind sessions + one operator-scripted walk** — the walk carries the plants and the destructive
  sequences a blind agent cannot be relied on to reach.
- **Container-isolated, invocation log ON.** Reuse [trial-harness/](../../trial-harness/) — do not
  rebuild it. `CLAUDE.md` → *Trial tooling* records why that warning exists.

## What is new on this binary, and therefore what the trial should reach

M49's twelve increments, and in particular the surfaces a worker meets: the item-region fix (both
seams), the closed `set:` vocabulary, `doc add-item --slug`, `placement-root` and its move floor,
`planning-record`'s 14 required gate slots, `completion-record` at v2, `milestone-record` at v3, PB-1's
composed packs, N1's converged codes and N2's additive integer. Plus the two post-M49 changes above —
**the boundary gate is a tightening, so a fan-out that landed before may now block.**

## Owed to M50, carried so it is not rediscovered

1. **`dev/gate` has no standing end-to-end fence** (its report layer does). The proposed CI arm must
   assert the **named step and exit code**, not merely non-zero — asserting non-zero passes on the
   exit-127 shape and makes the fence the vacuous red it exists to catch.
2. **`AddedNestedRepeatable` deserves re-adjudication** under the human's criterion. It was refused at
   M49 partly on *"the port doesn't need it"*. But `changelog.releases/changes` is a **shipped** nested
   block that no migration can reshape, and the route `migrate-corpus` prints points into
   `crates/engine/src/transform.rs` — a file no adopter can edit.
3. **The fan-out Fix phase** and its two increments are cut and waiting
   ([decisions-pending.md](../../../implementation/decisions-pending.md) → *Chartered and cut*). Its
   prerequisite shipped at `d854e25`.
4. **Measure whether the dev tooling took.** The counts to beat, from a scan of 139 subagent
   transcripts of the M49 build: **390** `rm`-on-a-variable-path calls by 77 of 137 agents, and **215**
   piped-gate blocks across 117 of 137. **This is measured in M50's *build* transcripts, not this
   trial's** — trial workers drive `jigc`, not `cargo`.
6. **One product defect is routed here from the harness sweep** — `jigc task validate ""` **panics at
   exit 101** on `1.0.0-rc.13` (`crates/engine/src/finding.rs:741`: the route fence asserts on a route
   built from an id the door never validated). It is the un-swept sibling of M49's own *"one of them
   panicked at exit 101"*. The class was sized by driving: of seven id-taking doors, one panics and
   `jigc task discard ""` acks at **exit 0**. Everything else the sweep found is harness, not product,
   and is chartered away from M50 on purpose
   ([decisions-pending.md](../../../implementation/decisions-pending.md) → *The harness-surface wave*).

5. **The shell-guard `rm` rule** is handed to the human for a session in `~/ideas/claude-work/shell-guard`.
   It is the actual fix; everything shipped here is mitigation. A guard that **blocks** is
   self-correcting (198 of 215 recovered in ≤3 calls); one that **prompts** parks an agent and leaves
   no transcript.

## Traps

- **An agent's report is a lead, not a measurement.** Every artifact checked this session found
  something: six audit fixers each found a **larger class** than the finding reported (6→9 · 5→47 · 1→8
  · 2→8 · one branch→nearly every break shape · 1→3), two of those widenings were **unreported data
  loss**, and an advocate spiking an unrelated fork found a **red tree two parties had called green**.
- **The orchestrator is not exempt.** This session reported *"no HIGH"* off one audit block while a
  data-loss HIGH sat in the block beside it; pushed a red tree by bumping the version after the gate;
  and piped `cargo test` into `tail` **an hour after shipping the tool built to prevent it**.
- **Never run two tree-mutating agents on one working tree.** Broken at M38, M42 and again in M49's
  triage. The fix is the fan-out Fix phase (item 3), not a fourth restatement.

## Verified 2026-09-04, by the session that runs the trial

Every row above was driven or read at HEAD `f266770` before anything was built. Corrections
are recorded here rather than edited into the rows they correct, so the handover still says what
it said when it was written.

| Claim | Verdict | Evidence |
|---|---|---|
| `main` clean, pushed through `d854e25` | **HOLDS** — pushed through `f266770`, four commits later | `git status -sb` → `## main...origin/main`, `origin/main..main` empty |
| gate green, 3169 / 0 over 17 | **HOLDS** — now **3171 / 0 over 17**, the log at 19:36 is later than HEAD at 19:27 | `dev/gate --report` over `jigc-gate-wVcFbB` |
| `1.0.0-rc.13` built and installed | **HOLDS as a stamp, FALSE as a tree.** `~/.local/bin/jigc` and `target/release/jigc` are dated **Sep 1 01:01**, the rc.13 bump (`6533f70`). `d854e25` (Sep 4 06:45) changed `crates/cli/src/milestone.rs` *after* that build. | `ls -l`; `git diff --stat 6533f70..HEAD -- crates/*/src` → one file, `milestone.rs` |
| *"both on the binary this trial runs"* | **FALSE for the installed binary; true only for an image built from HEAD.** Two trees stamp rc.13 — the exact trap `verify-pair.sh` exists for. The trial image is built from `f266770` and its sha recorded in the gate record. | as above |
| `jigc task validate ""` panics at exit 101 on rc.13 | **PARTIAL — debug posture only.** The assert is `#[cfg(debug_assertions)]` (`crates/engine/src/finding.rs:737`). On the **release** binary the same call prints *"no findings — the task validates clean"* at **exit 0** (`--format json`: `findings: []`), with and without a live task. `jigc task discard ""` acks *"discarded task "* at exit 0 on both postures. The shipped observable is a **false green**, a worse row than a panic under the trial's own §1 table. `dev/jigc-rig` defaults to `target/debug/jigc`, which is how the class was seen as a panic. The routing to M50 stands; its description does not. | driven on `dev/jigc-rig committed-singletons` and `refs-post-hoc`, once with `--binary ~/.local/bin/jigc` and once with the default debug build |
| reuse [trial-harness/](../../trial-harness/), do not rebuild | **HOLDS, with an omission.** `verify-pair.sh` defaults to the `m46` probe set (rc.11 → rc.12) and needs an **`m49`** set for this pair, exactly as the last trial needed `m46` ([RC-1.0-final/protocol.md](../RC-1.0-final/protocol.md) §5.1). Its `:72` version match and `:119` digit-leading-slug awk are the two items decisions-pending already time-boxes to *before the release*; they are taken here. | file read |
| M49's *"twelve increments"* vs the eleven the surface list implies | the roadmap and the VERDICT say **twelve** — eleven build increments plus the acceptance/goldens increment, which does ship one product change (`MINT_DOORS`) | `implementation/roadmap.md:2235`, [M49/VERDICT.md](../M49/VERDICT.md):37 |
| items numbered 1, 2, 3, 4, **6**, 5 | cosmetic | — |

**Consequences taken into the trial:** the image is built from HEAD, never from the bump commit;
T1-a goes to M50 with the corrected observation (release = false green at exit 0, debug = panic);
and `verify-pair.sh` gains the `m49` set with the two time-boxed fixes. The protocol is
[protocol.md](protocol.md); the pre-trial findings are [pre-trial-findings.md](pre-trial-findings.md).
