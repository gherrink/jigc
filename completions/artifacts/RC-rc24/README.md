# The blind agent trial on the published `jigc 1.0.0-rc.24` — record and verdict

**Run:** 2026-10-03 (sessions 22:19Z–22:38Z). **Scored:** 2026-10-04, twice, independently.
**Protocol:** [protocol.md](protocol.md), pre-registered before any session ran.
**Ledger:** [findings-verification.md](findings-verification.md) — every lead driven, 39 rows.

---

## 1 · What this is, and the question it answers

This is the blind agent trial that [DECISIONS.md](../../../DECISIONS.md) → *the road to the 1.0.0
call* (D3) owes **before the call**, on the candidate that carries M54 and M55. No blind agent
had run since `1.0.0-rc.14`, and five milestones have since landed on the surfaces an agent
touches. The per-axis reviews measure defects; this trial asks the other question — *is anything
left that makes jigc unusable by an LLM agent?* — over the three surfaces
[decisions-pending.md](../../../implementation/decisions-pending.md) → *A blind agent trial on the
release binary* names: **the fan-out, `jigc task amend`, and the findings channel**. rc.24 is that
candidate plus the co-author trailer, so the trailer is checked here too.

**The binary is the PUBLISHED one.** Not a tree of ours: `cargo install jigc --version 1.0.0-rc.24
--locked` from crates.io (with `jigc-engine 0.1.0-rc.2`), inside the isolated container.

| | |
|---|---|
| image tag | `jigc-gate:registry-1.0.0-rc.24` |
| image id | `sha256:cead273d9c68f8e706dbccc71da04ad076e0c310747d8f95253ca4b8aa5f50bd` |
| `jigc --version` | `jigc 1.0.0-rc.24` |
| `jigc-sha` | `unknown` — by construction: a registry image carries no tree of ours, so **the image id is the binary's identity** |
| Claude CLI | `2.1.233 (Claude Code)`, pinned by the image |
| worker model | `claude-sonnet-5` (the harness default; no override) |
| permissions | `bypassPermissions` on every arm |

Every turn's `PROVENANCE` file ([evidence/](evidence/)) carries that image id.

**The isolation record** is [gate-rc24.json](gate-rc24.json): `verify-image.sh` 7 passed / 0 failed
(isolation · trusted workspace · version stamp · the `doc-code` probe on the single-binary arm ·
history round-trip · transcript recoverable · no host instructions), the host-YES / container-NO
differential among them. `run.py gate` accepted the record before each arm, and
`run.py observe --archive` reproduced the 1.0.0-gate channel table exactly before any session was
read.

**`verify-pair.sh` is not applicable in registry mode and was not run.** It needs a sha-built older
image and a probe set for the pair; neither exists for a registry build, and crates.io never serves
two builds under one version. The consequence is declared, not hidden: **this trial carries no
behavioural proof against rc.14** (§10, §11).

---

## 2 · The headline

**Tier-1 findings: 1 — by the predicate, out of verification, and not from anything a session
did — and, after this record was assembled, upheld as tier 1 at low likelihood by an independent
adversarial re-drive** ([tier1-verification-L-22.md](tier1-verification-L-22.md), 2026-10-04, the
same published binary).

- **No tier-1 event occurred in any session.** Neither scorer saw an exit-0 loss or repository
  harm through a committing, destroying or moving door in any arm.
- **One confirmed finding meets the tier-1 predicate: L-22's variant.** Arm (a)'s worker made a
  linked worktree with raw git beside a provisioned milestone, which raised the question *what do
  the milestone doors do over a worktree jigc did not cut?* Driven on fresh rigs: a **live**
  foreign worktree is left byte-identical by every door (the lead as filed is refuted). But a
  foreign worktree **whose directory is absent at the door's instant** — git's `prunable` state,
  unlocked — has its git admin record deleted at **exit 0, silently**, by `jigc milestone
  provision`, `jigc milestone finalize` and `jigc milestone discard`: the worktree's index, `HEAD`
  and reflog are gone, a commit only that `HEAD` reached dangles until gc, and the returned
  checkout is no longer a git repository that `git worktree repair` can mend.
- **Its verifier fixed the tier at 1 by the predicate, and set out for the reader what argues it
  down to 3**; that argument is quoted in full in §7. This record does not take the step down:
  protocol §1 fixes a tier from the evidence before its consequence is looked up, and *"judged not
  to matter"* is not a disposition.
  The pre-registered consequence of a tier-1 row is **a fix pass before the call, never a wave**.
  Whether this row is one is the human's.
- **The adversarial re-drive could not refute it: L-22 stands at tier 1 by the predicate, low
  likelihood** ([tier1-verification-L-22.md](tier1-verification-L-22.md)). It drove what the first
  verifier had left as inference or as argue-down, and added cells this record did not have:
  - **the door set is four, and a refusing run prunes too.** A bare, repository-wide
    `git worktree prune` runs in `milestone provision`, `milestone finalize`, `milestone discard`
    and `uninstall`. `jigc milestone finalize` on a never-provisioned milestone **refuses** (exit 3,
    `milestone.zero-contribution`) and the foreign record is gone all the same; a refused `discard`
    does not prune; the single-task loop never does.
  - **`uninstall` narrates a prune it attributes to `.jigc/`.** With no fan-out worktree at all and
    one prunable foreign registration, its ack says it pruned the registrations *for the fan-out
    worktrees `.jigc/` held* — the one door that says anything misattributes what it dropped.
  - **permanence is shown, with a control.** After the prune, a detached commit older than git's
    default prune age is deleted by the next plain `git gc`; the same gc on a rig no jigc door
    touched keeps it.
  - **the everyday trigger (O-1) was driven:** a host worktree that was never moved or deleted is
    orphaned when jigc runs in a container that sees the repository and not the worktree's path;
    so is a worktree moved with plain `mv` that still works at its new path.

  What the re-drive sets against it, for the human to weigh: working files are never touched, branch
  work survives whole, permanence needs a second event (gc) and age, git itself calls the state
  `prunable`, and it did not occur in the trial. **§7's L-22 entry and §11's bounds were written
  before this re-drive and are left as written** — where they say O-1 was not driven, this bullet
  and the file are the later word. **The ruling on the row, and the scope of any fix pass, are the
  human's and have not been taken.**

**The arms** — both scorers, independently, the same class on every arm:

| arm | class | reading |
|---|---|---|
| **(a)** the fan-out | **A-1** — provisioned, worked in isolation, landed through jigc | **reached** |
| **(b)** the correction | **B-1** — corrected through `jigc task amend` | **reached** |
| **(c)** the disagreement | **C-5** — missed (sub-case *propagated*: **not** met) | **not reached** |

**The trailer: the requirement holds.** Every commit jigc made inside an agent session carries
`Co-Authored-By: Claude <noreply@anthropic.com>`, byte-exact, exactly once: **9 of 9**; 0 variant,
0 NONE, 0 DUPLICATE, 0 not-jigc. The control — the install commit made outside the agent — carries
none, 3 of 3. Two paths were not exercised and the verdict says nothing about them: a committing
door run by a sub-agent, and de-duplication by address (§5).

**The rest of the ledger:** one tier-2 row and six tier-3 rows, recorded and never blocking;
fourteen product leads reproduced and contradicting no contract; two refuted. Fifteen tooling
rows about the trial's own apparatus, none of which moved a class or a trailer row (§9).

**What the headline does not say:** n = 1 per arm, headless, under `bypassPermissions`, one
worker model. A *reached* shows the path can be walked blind once. C-5 shows where one worker
stopped once — and the instrument could not tell *missed* from *saw no disagreement to report*
(T-12). §10 is the full list of bounds; read it before believing a row.

---

## 3 · What ran

### 3.1 · The arms

Run order **(c), (b), (a)** — the arm that does not depend on `seed` first, the longest last.

| arm | corpus | turns | transport | permissions | exit | wall-clock | invocation records | denials |
|---|---|---|---|---|---|---|---|---|
| **(c)** the disagreement | `calderby` — **the standing wart**, adopted | 1 | headless, `run-session.sh --headless` | `bypassPermissions` | 0 | 22:19:39Z → 22:21:29Z · **1 min 50 s** | **15** in the log — 2 adoption's, **13** in-session (11 worker-typed + 2 the adapter hooks') | 0 |
| **(b)** the correction | `halloway` — clean prose, adopted | 2 | headless, `run.py seed` | `bypassPermissions` | 0 · 0 | 22:21:46Z → 22:23:44Z · **1 min 58 s** | **26** cumulative — 2 adoption's, **15** turn 1, **9** turn 2 | 0 |
| **(a)** the fan-out | `fenwick` — clean prose, adopted | 2 | headless, `run.py seed` | `bypassPermissions` | 0 · 0 | 22:24:00Z → 22:34:18Z · **10 min 18 s** | **48** cumulative — 2 adoption's, **37** turn 1, **9** turn 2 | 0 |

Wall-clock is the operator's own start/end stamps around each driver command, both turns of a
seeded arm together. Non-zero jigc exits inside a session: none in (c), none in (b), three in (a)
(ordinals 4, 20, 41) — each a refusal that named its route, and each was recovered within two
calls.

**No arm was void and none was re-run.** Every turn's `PROVENANCE` says `exit-code 0`; every
stream's final `result` is `subtype: success` with 0 permission denials; every arm has an
invocation log; both seeds froze with `turns 2`, and each turn 2 opened on state turn 1 left (it
addressed a commit or a sub-task id minted in turn 1) — not cold. The prompts are
[paste/c-prompt.txt](paste/c-prompt.txt), [paste/b-turns.txt](paste/b-turns.txt) and
[paste/a-turns.txt](paste/a-turns.txt), delivered exactly as committed.

Two **unscored** debrief turns followed, after the arms were read: `run.py fork` over the frozen
conversations of (b) (22:35:53Z → 22:36:29Z) and (a) (22:36:35Z → 22:38:11Z), each at exit 0, 0
denials, neither beginning cold (§8). Arm (c) has no frozen conversation and so no debrief (§11).

### 3.2 · The preconditions — each would have stopped the trial

| # | precondition | result |
|---|---|---|
| 1 | **walk arm 00**, the positive control, run first | exit 0 · `ARM 0 PASS — the channel fires and is countable` · 10 invocation records, `doc show … --task` **1**, adjacent 2 ([evidence/walk/walk-record-rc24.md](evidence/walk/walk-record-rc24.md)). Arms 01–23 are listed `NOT RUN`, which is the truth |
| 2 | **the environment probe**, one headless turn, not blind, not scored | **`MAIN-SET`** / **`SUB-SET`** — the variable the co-author trailer keys on is set, non-empty, in the headless agent's shell and in a sub-agent's. 0 denials. By protocol §6.3: the precondition holds everywhere, so any NONE row would be a defect in the feature. It prints `SET`/`UNSET`, never a value |
| 3 | **the seed smoke**, two trivial turns through `run.py seed` | froze with `turns 2`; turn 2's reply was `ALPHA BETA` — the second container resumed the first one's conversation |

Before those: the driver's seven suites green; the gate accepted; the four corpora instantiated
from `completions/trial-corpus-template/`, gated (`fenwick`, `halloway`, `walk-rc24` 12 passed;
`calderby` 11 passed with the prose bar **skipped and saying so**), adopted through the
container's own binary and carried ([corpora.md](corpora.md)). No corpus was planted.

### 3.3 · The read-back — reported, not the headline (protocol §7)

`run.py observe`, gated, one arm per command:

| row | recs | wrote | VERB | adj | fs | outcome |
|---|---|---|---|---|---|---|
| (c) | 13 | 4 | 1 | 1 | 0 | `read back through the fence's verb` |
| (b) turn 1 | 15 | 4 | 1 | 1 | 0 | `read back through the fence's verb` |
| (b) turn 2 | 9 | 4 | 1 | 0 | 0 | `read back through the fence's verb` — plus the `HISTORY REWRITTEN` line, expected under B-1 (T-1) |
| (a) turn 1 | 37 | 12 | 2 | 2 | 0 | `read back through the fence's verb` |
| (a) turn 2 | 9 | 0 | 0 | 0 | 0 | `VOID unmeasured — no authoring occasion existed` — the read-back reader's row for a turn with no doc write, **not** the arm's (T-2) |

Every turn that authored a doc read it back through `jigc doc show … --task`. Zero filesystem
reads of a managed doc. The series stands where it has stood; no arm was built for it.

---

## 4 · The arms, one by one

Classes are the protocol's own, **fixed before the run** and used as written. Order of authority
(protocol §8): the invocation log, then `.git`, then the transcripts. Ordinals are 1-based lines of
the arm's cumulative log ([evidence/](evidence/)); times are UTC.

### 4.1 · Arm (c) — the disagreement

**Classes as fixed before the run** (protocol §5.3):

| class | the evidence that puts a session here | reading |
|---|---|---|
| **C-1** filed through the channel | `start --workflow report-inconsistency`, `doc create inconsistency`, that task's `task finalize`, all at exit 0; a doc under `docs/inconsistencies/` in a commit of its own, naming at least two of the four sites as sides | reached |
| **C-2** recorded through jigc, by another door | the disagreement is written into a managed doc through jigc, and no `inconsistency` doc exists | not reached |
| **C-3** fixed silently | a site is edited to remove the claim (or code added to make it true), and nothing records that they disagreed | not reached |
| **C-4** mentioned only | the worker names the disagreement in a final message, or ends asking, and nothing in the repository records it | not reached |
| **C-5** missed | nothing in the transcript, the final message or any doc shows the worker noticed — sub-case **propagated**: the architecture document asserts the hand-off the code does not make | not reached |
| **C-V** void | apparatus; or no occasion — the worker read none of the three sites | VOID |

**Class assigned: C-5 — missed. Not reached. The *propagated* sub-case is not met.**

The evidence it rests on:

- **The log.** 15 records, no non-zero exit. Ordinal 3 is the SessionStart hook's `start`, whose
  output carries the thirteen-entry router catalog including *"report-inconsistency — code and a
  doc, or two docs, disagree and the disagreement is worth a record until it is reconciled"*.
  Ordinal 4 is `start --workflow architecture-documentation "<intent>"`; 8–9 `doc create
  arch-doc` and `doc author arch-doc`; 10 `doc show arch-doc:sample-rollup-pipeline --task …`; 15
  `task finalize`. There is **no** `start --workflow report-inconsistency`, **no** `doc create
  inconsistency`, no second task.
- **Git.** One commit since the session began: `d107e87`, one file,
  `docs/architecture/sample-rollup-pipeline.md` (+142). No `docs/inconsistencies/`. `README.md`,
  `package.json` and `src/` are byte-unchanged; the three claim sites still read as instantiated.
- **The occasion existed — not C-V.** The worker read all three claim sites and the code side
  before its first own jigc call: `src/router.ts` ≈ 8.7 s after session-start, the `src/store.ts`
  header ≈ 9.3 s, `README.md` ≈ 11.0 s, `package.json` ≈ 15.1 s.
- **What the committed document says about the hand-off** — two things, side by side, with no
  tension flagged. It **answers the clause from the code**: *"The only outbound surfaces are the
  two GET routes' plain-text replies … there are no outbound calls to any other system"*. And it
  **restates the prose's framing as fact**: *"calderby is a rollup cache for time-series samples
  that sits in front of whatever long-term store the caller already owns"*. One sentence holds
  both and reconciles them by interpretation: *"It hands nothing on to persistent storage of its
  own — the long-term store this is a cache in front of is entirely the caller's responsibility,
  outside this service's edges."*
- **The worker's own wording**, final message: *"a stated edges section (inbound: router's
  transport-agnostic `dispatch`; outbound: plain-text GET replies only; retention/cap bounds; no
  durable storage or outbound calls to other systems)"*. It does not end asking and names no
  disagreement. The transcript's thinking blocks are empty in the saved format; no worker text
  mentions a disagreement. No `git` command at all; no refusal met.

**Why not the neighbours.** C-1: none of its records exists. C-3: no site edited. C-4: nothing
named, nothing asked. **C-2 is the nearest**, and both scorers rejected it on the same condition:
it needs *the disagreement* written into a managed doc, and the document states both facts as one
consistent picture — it names no site and says nowhere that two sources differ. *Propagated* is
not met because the document does not assert the hand-off; it denies one.

**Where the two scorers differed.** Not on the class. The second scorer named C-2 against C-5 as
*the* judgment call and declined a generous reading (*"C-2 presupposes a worker that saw a
disagreement and chose a different door for it"*); the first reached C-5 by the same route. They
count the restated framing differently — *three times* (first scorer, counting the reconciling
sentence) against *twice* (second) — over the same four passages; nothing turns on it. **Both
flagged, unprompted, that §5.3 has no cell for this behaviour**, which is T-12: the standing wart
states nothing the code contradicts, only a role the code does not implement, so the evidence
cannot separate *missed* from *saw no disagreement to report*. The reading — not reached — does
not move either way.

**Surfaces to look at** (protocol §5.3 names the router line first): L-1 — on the path this worker
was handed, the catalog line is the one surface naming the channel, and the composed
`architecture-documentation` text says nothing about what to do when code and prose disagree. No
contract is contradicted; an arm's outcome is not a finding (protocol §1).

### 4.2 · Arm (b) — the correction

**Classes as fixed before the run** (protocol §4.3):

| class | the evidence that puts a session here | reading |
|---|---|---|
| **B-1** corrected through `jigc task amend` | a `task amend` record at exit 0 in turn 2, then that task's `task finalize` at exit 0; exactly one `commit (amend)` in the reflog; `HEAD`'s message cites `TKT-221` and not `TKT-212`; `HEAD`'s tree equals the superseded commit's; no raw git history act in the transcript | reached |
| **B-2** a new commit; the landed message unchanged | a second commit in turn 2, and the turn-1 message still cites `TKT-212` | not reached |
| **B-3** rewritten outside jigc | the reflog shows an amend, reset or rebase with no `task amend` record behind it | not reached |
| **B-4** not corrected | `HEAD` still cites `TKT-212` and there is no second commit | not reached |
| **B-5** through the door, wrong result | B-1's records, and the message is wrong or lost what the commit does | reached the door; the repair failed |
| **B-V** void | apparatus; or turn 1 left no occasion | VOID |

**Class assigned: B-1 — corrected through `jigc task amend`. Reached.** Every element holds:

- **The occasion turn 1 left — not B-V.** One commit after adoption, `7757a4c` *fix(summary):
  reject GET /summary/ with no series name*, at `HEAD`, landed by `task finalize` (ordinal 17);
  its message ends `Refs: TKT-212`; `git grep TKT-` over the tree finds nothing.
- **The door.** Ordinal **19**, `task amend --format json`, exit 0, seven seconds into turn 2 →
  task `amend-7757a4c`. Ordinals 20–23 re-author type, scope, summary and body; 24 reads the
  commit doc back; **26**, `task finalize amend-7757a4c`, exit 0, ack `"amended": "7757a4c"`,
  `"hash": "4b80510"`.
- **Git.** Exactly one `commit (amend)` reflog entry, `4b80510`, over `7757a4c`. `HEAD`'s message
  is the same subject and body with `Refs: TKT-221`; `TKT-212` appears nowhere in it. The trees
  are equal (`766eb5cc…` both), the diff between the two commits is empty, the parent is the
  same, the author date is kept and the committer date reset — as the door's own text says.
- **No raw history act.** Every Bash command of both turns read: `git add` of the two changed
  files (the workflow's own instruction), `git status`, `git log -1 …`. No `commit`, `reset`,
  `rebase`, `--amend`. No sub-agents.
- **The worker's words** on arriving in turn 2: *"Index is clean and `HEAD` is the commit in
  question. Let's amend it via jigc."* Final message: *"Amended cleanly — the commit is still a
  single `fix(summary): ...` commit (now `4b80510`, replacing `7757a4c`), with the exact same tree
  and `Refs: TKT-221`"*.

Recorded beside the class: **attempts** — none failed; no `--help`, no discarded mint, no refusal.
**Where `TKT-212` went** — the last line of the commit doc's *body* slot, not a `#trailers` item
and nowhere in the tree, so the amend reached the only copy. **The trailer on the rewritten
commit** — exact, once (§5). `observe`'s `HISTORY REWRITTEN` line is the bound protocol §8.3
declared, backed here by the `task amend` record (T-1).

**Where the two scorers differed.** Nowhere on the evidence. The second scorer added one datum:
the arm's own log contradicts the worker's later debrief claim that the composed step's
`#type` / `#scope` addresses *"would have failed"* — the same worker wrote them at exit 0 in turn
2 (ordinals 20–21). That became L-11.

### 4.3 · Arm (a) — the fan-out

**Classes as fixed before the run** (protocol §3.3), scored over the whole conversation:

| class | the evidence that puts a session here | reading |
|---|---|---|
| **A-1** provisioned, worked in isolation, landed through jigc | `milestone provision` at exit 0; work for ≥ 2 sub-tasks done inside `.jigc/worktrees/<id>/`; `milestone finalize <id>` at exit 0, its commit carrying all three pieces' code | reached |
| **A-2** provisioned, not landed | `milestone provision` at exit 0, and no `milestone finalize` at exit 0 by the end of turn 2 | reached provision; the boundary not reached |
| **A-3** the work done without the fan-out | the three pieces exist, and there is no `milestone provision` at exit 0 | not reached |
| **A-4** halted | the conversation ends asking, or on a denial, with no provision and the pieces not done | not reached |
| **A-V** void | a turn's CLI exited non-zero; no log; `seed` aborted; turn 2 began cold | VOID |

**Class assigned: A-1 — provisioned, worked in isolation, landed through jigc. Reached.**

- **Turn 1 took the designed path: it set the milestone up and stopped.** Ordinal 5 composes the
  `planning` workflow; 13–24 author four planning docs; **31** `task finalize` lands them
  (`a9f1851`); **33** `milestone create` (`07a451f`); **35, 37, 39** three `milestone add-task`
  (`bc3ab92`, `dafdc52`, `29ea968`). The turn ends *"Say when, and the three can be started
  together."* — a statement, not a question. The pack's human-owned Settle gate did not halt it.
- **The route to `provision` was the resume refusal.** Turn 2's orientation (ordinal 40) lists
  each sub-task as an active task. Ordinal **41**, `start --task cap-the-store-at-1000` from the
  main checkout, exit **1**: *"… a sub-task's work happens in its own worktree, cut from that base
  rather than in this checkout: run `jigc milestone provision bound-what-the-service-will` … then
  `cd /work/.jigc/worktrees/cap-the-store-at-1000` and re-run this command there"*. Ordinal
  **42**, `milestone provision`, exit 0, three seconds later; 43–45 `start --task <id>` from
  inside each worktree, exit 0. Not `milestone execute`, not `--help`.
- **Who worked the pieces: three sub-agents, concurrently, one per worktree.** Their spans
  overlap (22:31:03–22:31:41, 22:31:17–22:32:04, 22:31:34–22:32:07). Every `Edit`/`Write` each
  made is under its own `.jigc/worktrees/<id>/`: `src/store.ts` + a new `test/store.test.ts`;
  `src/validate.ts` + `test/validate.test.ts`; `src/ingest.ts` + `test/ingest.test.ts`. Each ran
  the suite and `git add`ed its two files; none committed, none ran a committing jigc door. The
  main agent then reviewed each staged diff and made two edits itself inside one worktree.
- **The boundary.** Ordinal **48**, `milestone finalize bound-what-the-service-will`, exit 0 →
  `cb4a90f` *Finalize milestone bound-what-the-service-will (3 sub-tasks)*, 7 files, +123 −6,
  carrying all three pieces: `MAX_SERIES = 1000`, `MAX_SERIES_LENGTH = 128`,
  `MAX_LINE_LENGTH = 1024`, each a module-local constant, `src/config.ts` untouched. No join
  collision; `milestone join` was not run on its own (`finalize` joined). Afterwards:
  `.jigc/worktrees/` empty, one branch, clean tree, 15 commit objects = 9 pre-session + 6 door
  commits, and the landed suite passes 33 of 33.
- **Final message**, turn 2: *"All three pieces are implemented, independently verified, and
  landed as one commit (`cb4a90f`) on `main` … Each was built test-first in its own worktree by an
  independent agent, with no shared files"*. It ends on a *worth doing, not done* list, not a
  question.

Recorded beside the class, never changing it:

- **A-1 with sub-agents — the full path by the protocol's note, but not the product's spawn.**
  `milestone execute` was never run, so no `Spawn:` line was ever seen and no sub-agent ran
  `jigc workflow sub-task`. The sub-agents were briefed by the main agent's own prompts and never
  spoke to jigc about their work (one ran a bare `jigc start`, ordinal 46). The composed sub-task
  text was read only by the main agent.
- **Outside jigc: a scratch integration check by raw git.** `git worktree add <tmp>/milestone-verify
  a9f1851` (detached), the three staged diffs saved as patches and `git apply`'d there, `npm test`
  (33 pass), `git worktree remove … --force`. It made no commit, moved no ref, touched no managed
  doc and left nothing behind. The worker's stated reason (debrief): it *"didn't find a jigc verb
  that previews a milestone's merged state ahead of `finalize`"*. This is the origin of L-17 and
  L-22.
- **Refusals met, against protocol §0.6** — ordinal 4 (*requires an intent*): names the objection
  and the route, and the route worked (ordinal 5); but the command refused is the one orientation
  printed (L-2) and the envelope carries no code (L-3). Ordinal 20 (`write.wrong-shape`): a keyed
  finding naming the offending field and the accepted ones; the retry worked after the worker read
  `doc author --help` (L-18). Ordinal 41: names objection, consequence, route and path, and both
  halves ran verbatim — **reads as designed** (declared, §0.2), with L-3 and L-4 beside it.

**Where the two scorers differed** — on detail, never on the class; the stricter reading stands:

| point | first scorer | second scorer | what stands |
|---|---|---|---|
| where the sub-agents started | **all three** started in one worktree (the wire-line piece's — the main agent's shell directory at launch); two `cd`'d to their own, the third's inherited directory happened to be its own | **one** sub-agent found its shell had started in a sibling's worktree and corrected | **all three** — the stricter reading, and T-15 confirmed it from each transcript's recorded start directory. *Worked in isolation* rests on the main agent's prompts relaying a `cd`, not on anything jigc printed |
| what caught the cross-piece break | the main agent's **review of the staged diffs** found it (a test built from a 1017-character series name that the sibling piece's 128 cap would reject); the lead it filed (L-17) nonetheless says the scratch worktree *"is how"* it was caught | *"caught by the worker's raw-git scratch integration before the boundary"* | **the review** — L-17's verification read the transcript's order: the break is named at 22:33:04 after reading the diffs, fixed and re-staged, and the scratch worktree is created at 22:33:43 and reports 33 pass. It confirmed a fix already made; it caught nothing. The worker's debrief, written from memory, has the order reversed |
| how wide *reached* is | recorded under *who worked the pieces* | stated outright: *"arm (a)'s reached is narrower than its label suggests"* — the product's spawn surface was never touched | the narrower statement stands beside the class, as above |

---

## 5 · The trailer check (protocol §6)

**The requirement, the human's:** every commit jigc makes in the trial carries
`Co-Authored-By: Claude <noreply@anthropic.com>`.

**Scope — identified, not guessed:** the committing-door records at exit 0 in each arm's log, at
or after the session's start; every commit object created since (reachable from a ref, from the
reflog, or — the scorers' own widening — from nowhere); joined on the hash each door's own ack
printed and on committer time; each match then checked against the raw git acts read from every
transcript. Trailers counted with git's own parser, and the line matched byte-exact.

### 5.1 · Every commit jigc made inside a session

| arm | sha | door (log ordinal) | ran by | trailer as git parses it | row |
|---|---|---|---|---|---|
| (c) | `d107e87` | `task finalize document-the-service-architecture-how` (15) | main | 1 × `Co-Authored-By: Claude <noreply@anthropic.com>` | **exact** |
| (b) | `7757a4c` — reflog only, superseded | `task finalize fix-get-summary-with-empty` (17) | main | 1 × the same line | **exact** |
| (b) | `4b80510` | `task finalize amend-7757a4c` (26) — **the amend** | main | 1 × the same line | **exact** |
| (a) | `a9f1851` | `task finalize bound-what-the-service-will` (31) | main | 1 × the same line | **exact** |
| (a) | `07a451f` | `milestone create …` (33) | main | 1 × the same line | **exact** |
| (a) | `bc3ab92` | `milestone add-task … "Cap the store at 1000 …"` (35) | main | 1 × the same line | **exact** |
| (a) | `dafdc52` | `milestone add-task … "Reject a sample whose series name …"` (37) | main | 1 × the same line | **exact** |
| (a) | `29ea968` | `milestone add-task … "Reject a wire line longer …"` (39) | main | 1 × the same line | **exact** |
| (a) | `cb4a90f` | `milestone finalize bound-what-the-service-will` (48) — **the boundary** | main | 1 × the same line | **exact** |

**Tally: exact 9 · variant 0 · NONE 0 · DUPLICATE 0 · not-jigc 0.** Nine doors, nine commits, one
to one: every door at exit 0 has its commit and every in-session commit has its door. Five kinds
of commit are covered — an ordinary task commit, a doc-only task commit, the record-only
`milestone create` / `add-task` commits, the fan-out boundary, and an amend, including the commit
the amend superseded. The three `add-task` commits share one second; they were paired by the task
id in each subject and the hash each ack printed, not by time (T-9).

**The amend re-derives it:** `4b80510` carries the line once although the worker added no trailer
item on the amend task. What the amend text says about that is L-10.

### 5.2 · The control rows — expected to carry none

The install commit `jigc setup` made through `adopt.sh`, **outside** the agent, one per corpus:

| arm | sha | subject | `Co-Authored-By` |
|---|---|---|---|
| (c) | `c61afab` | `chore(jigc): install jigc workspace config` | none |
| (b) | `518de16` | `chore(jigc): install jigc workspace config` | none |
| (a) | `839647c` | `chore(jigc): install jigc workspace config` | none |

**3 of 3 carry none.** A trailer there would be a false co-author on a commit no agent made.

### 5.3 · The commits jigc did not make

- **Inside a session: none**, in any arm. No transcript — main or sub-agent — contains a raw
  `git commit` or any other history verb, so no row moves to *not-jigc*.
- **Before the sessions, out of scope:** the template's seven commits per corpus, and `adopt.sh`'s
  raw-git adoption commit — `fbbf746` (c), `df2fc63` (b), `3c490f8` (a). None carries the trailer,
  and none should.
- Arm (b)'s `Refs: TKT-…` line is body-slot prose that git's parser reads as a trailer beside
  jigc's line; it is a different key (L-23).

### 5.4 · The verdict

**The requirement is met on every commit jigc made inside an agent session in this trial: 9 of 9
carry `Co-Authored-By: Claude <noreply@anthropic.com>`, byte-exact, exactly once; the control
carries none, 3 of 3.**

**Where the two scorers differed:** the first says *holds*; the second says *met, with two
unexercised paths*. The stricter statement stands, and both paths are bounds on the verdict:

- **A committing door run by a sub-agent.** The probe read `SUB-SET`, but all nine doors were run
  by a main session. Arm (a)'s three sub-agents ran no committing door.
- **De-duplication by address.** No worker added its own `Co-Authored-By` item — there is no
  `doc add-item commit:<task>#trailers` record in any log — so no *variant* could arise and the
  de-dup seam was not met.

The operator's helper output ([evidence/](evidence/) → each arm's `trailer-rows.txt`) agrees with
both hand derivations row for row.

---

## 6 · The declared changes each session met (protocol §0)

None is scored a regression. A declared refusal is judged against §0.6: it names what it objects
to · says the consequence · names the route in the same output · the route, run verbatim, works.

| § | declared change | met by | evidence |
|---|---|---|---|
| 0.1 | the co-author trailer on every commit jigc makes under the agent | a, b, c | §5 |
| 0.1 | thirteen-entry router catalog with `report-inconsistency` | a, b, c — shown | orientation at c#3, b#3/#18, a#3/#40; the router at b#4. No arm chose it |
| 0.1 | `.jigc/AGENT.md` and the installed guide changed | a, b, c | (b)'s worker took `jigc task amend`, which `AGENT.md` names |
| 0.1 | a fan-out path printed absolute | a | a#41's route prints the worktree's absolute `cd`. No `Spawn:` line was seen. A printed `git` span without `-C` is L-9 — no contradiction |
| 0.1 | a refusal under `--format json` is a findings document keyed `(code, target)` | a — once | a#20 `write.wrong-shape` is one; a#4 and a#41 are `{"error": …}` with no code (L-3). **This item over-states its source** — see below |
| 0.1 | the `doc-code` probe is inside the one binary | c — implicitly | c#15 finalized an arch-doc with eight `implemented-by` anchors; no sibling executable was looked for |
| 0.1 | `doc show` / `doc list` rows carry `title` and `fields` | a, c | a#7, a#15, a#18 |
| 0.2 | `milestone create` / `add-task` each land a record-only commit and say so | a | a#33, #35, #37, #39: *"record commit: 07a451f — …"* (L-16) |
| 0.2 | a sub-task's composed text has no commit-boundary steps; the boundary lands one CLI-synthesized commit | a | a#43–45 end *"never `git commit` and never `jigc task finalize` here"*; `cb4a90f` (L-20) |
| 0.2 | resuming a sub-task from the main checkout refuses and routes to `milestone provision` | a | a#40, a#41 → #42 → #43. §0.6: all four hold — **reads as designed** |
| 0.3 | `jigc task amend` exists; ack `amended <old> → <new>` | b | b#19, b#26 |
| 0.3 | a correct amend leaves `commit (amend)`, and `observe` says `HISTORY REWRITTEN` | b | the reflog; the read-back row in §3.3 |

**Not met by any session**, so nothing to judge: the two `repo.*` posture refusals; the
`foreign-bytes` displacement (`displaced: []` at every finalize); the three
`workflow.verb-routed` refusals; `milestone finalize` over a worktree mid git operation;
`repo.head-detached` inside a worktree; all of §0.4 (no report was filed); all of §0.5 (no
`setup`, no non-zero `validate`, no unslugable title, no singleton-address refusal, no deny-floor
call, no `task discard`).

**One correction to the protocol's own list, from L-3's verification.** §0.1 says *"A refusal
under `--format json` is a findings document keyed `(code, target)`"*. Its source says *"a reject
**carrying a finding**"*: the product's contract declares two reject arms, and a bare refusal
takes `{"error": …}`. [protocol.md](protocol.md) is committed as it was registered; the sentence
should be read as *"a refusal that carries a finding …"*.

---

## 7 · Findings

**The decision rule** (protocol §1): **tier-1** — exit-0 loss, or repository harm, through a door
that commits, destroys or moves → a fix pass before the call, never a wave. **tier-2** — a posture
or route dead end; a code-less or undeclared refusal → recorded, never blocks. **tier-3** — a
surface that says something the binary does not do → recorded, never blocks. The scale's one home
is [decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.16 wave (M52)*.

**Each row carries the fields the findings channel files** (`design/findings-channel.md` §1.1), so
it files at the port without being reshaped: `tier:` · `door:` · `found-in:` · the fenced `repro:`
block · `contract:` — the contract contradicted · `pinned-by:` or `UNPINNED:`. Every repro was
driven on a fresh rig against the installed registry `jigc 1.0.0-rc.24` before it was written
down; the blocks here are condensed from the full ones in
[findings-verification.md](findings-verification.md), which also carry the controls. `$REPO` is
the rig's repository, `<tmp>` its temporary root. Nothing is filed through jigc until the port:
this repository has no store.

**An arm's outcome is not a finding**, and a worker's judgment call is never a row.

### Tier 1

#### L-22 · A foreign linked worktree whose directory is absent has its git admin record silently pruned at exit 0 — **PARTIAL**

`tier:` tier-1 **by the predicate, for the confirmed variant** — its verifier also states what
argues it down to 3; the lead as filed (a *live* foreign worktree) is `none`.
`door:` `jigc milestone provision` · `jigc milestone finalize` · `jigc milestone discard`
**[Corrected 2026-10-04 by the adversarial re-drive:** the doors are **four**, not three —
`jigc milestone provision`, `jigc milestone finalize` (landing *and* refusing: on a never-provisioned
milestone it exits 3, `milestone.zero-contribution`, and the foreign records are gone all the same),
`jigc milestone discard` (landing only — also at exit 0 on a never-provisioned milestone; a refused
discard does not prune) and `jigc uninstall` (driven: exit 0, the foreign record gone). So the
precondition on jigc's side is not a provisioned fan-out; it is any run of one of these doors
([tier1-verification-L-22.md](tier1-verification-L-22.md) → 1, 7).**]**
`found-in:` `trial:RC-rc24/a` — raised by what the session did around jigc (it made and removed a
raw linked worktree); **no jigc command ran while that worktree existed, so the session itself is
no evidence either way.**

`repro:`

```sh
jigc --version                                   # jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary "$(command -v jigc)") || exit; eval "$rig"; [ -n "$REPO" ] || exit
T=$(mktemp -d "${TMPDIR:-/tmp}/l22.XXXXXX") || exit; M=foreign-worktree-probe
jigc milestone create "Foreign worktree probe" >/dev/null
jigc milestone add-task $M "Add alpha file" >/dev/null; jigc milestone add-task $M "Add beta file" >/dev/null
jigc milestone provision $M
for s in add-alpha-file add-beta-file; do
  echo code > "$REPO/.jigc/worktrees/$s/$s.txt"; git -C "$REPO/.jigc/worktrees/$s" add "$s.txt"; done

# a foreign linked worktree holding a commit only its own HEAD reaches — then its directory absent
Y="$T/y"; git -C "$REPO" worktree add -q --detach "$Y" HEAD
echo c > "$Y/foreign-committed.txt"; git -C "$Y" add .
git -C "$Y" -c user.name=probe -c user.email=probe@example.invalid commit -qm "foreign detached commit"
FC=$(git -C "$Y" rev-parse HEAD); echo s > "$Y/foreign-staged.txt"; git -C "$Y" add foreign-staged.txt
mv "$Y" "$T/y.away"            # moved by hand · volume unmounted · path not mounted where jigc runs

ls "$REPO/.git/worktrees/y"                                   # before: commondir gitdir HEAD index logs …
git -C "$REPO" fsck --unreachable | command grep -c "$FC"     # before: 0 — reachable, from that HEAD

jigc milestone finalize $M     # exit 0 — "finalized <sha> — Finalize milestone … (2 sub-tasks)";
                               # stdout and stderr carry no word about any worktree registration

ls "$REPO/.git/worktrees/y"                                   # after: No such file or directory
git -C "$REPO" fsck --unreachable | command grep -c "$FC"     # after: 1 — dangles until gc
mv "$T/y.away" "$Y"; git -C "$Y" status --porcelain           # fatal: not a git repository   [exit 128]
git -C "$REPO" worktree repair "$Y"                           # error: unable to locate repository   [exit 1]

# controls, each on its own rig: no jigc command between the two `mv` → the record survives and
# `git worktree repair` exits 0; `git worktree lock "$Y"` before the `mv` → `milestone provision`
# exits 0 and the record survives; `jigc start` over the absent directory prunes nothing;
# `jigc milestone provision $M` alone and `jigc milestone discard $M` alone each drop it the same way.
# The refuted half: a LIVE foreign worktree (untracked, staged and unstaged changes; at <tmp>/x, or
# even at $REPO/.jigc/worktrees/mine) is byte-identical after provision / finalize / discard.
```

`contract:` two statements. **`DECISIONS.md` → 2026-09-23, the confirmation pass**, the `uninstall`
paragraph — *"It prunes now, best-effort as they are, and **says so**: a destroying door narrating
what it changed in the repository is law 1."* `jigc uninstall` prints `pruned git's worktree
registrations`; its three siblings run the same repository-wide `git worktree prune` and print
nothing when what it dropped was not jigc's. **`design/storage.md` → *Repository layout*** —
*"No door removes such a path silently: each probes it first and then either refuses what it
cannot prove is disposable … or names every byte it is about to destroy"* — describes a door whose
reach is `.jigc/worktrees/<sub-task-id>`; the prune reaches every registration, probes nothing and
names nothing. Mechanism, read in source: a bare `git worktree prune` in `provision_worktrees` and
at the end of `remove_worktrees` (`crates/cli/src/milestone.rs`), with no `--expire`. The harm
class is one the project already ruled HIGH for its *own* worktrees
(`crates/cli/tests/leftover_operation_in_progress.rs`, module doc).

`UNPINNED:` both halves. No suite plants a registered worktree of this repository outside
`.jigc/worktrees/<sub-task-id>` and asserts it survives a milestone door, and none builds a
`prunable` foreign registration; the only prune assertion is `uninstall`'s narration, over jigc's
own fan-out records.

**What argues it down — the verifier's own words, quoted so the human can weigh them:** *"the
precondition is not jigc's doing and did **not** occur in the trial (the session removed its
worktree with `git worktree remove` before the boundary); git itself calls this state `prunable`,
a bare `git worktree prune` typed by anyone does the same, and `git worktree lock` is git's
documented protection (cell 12 honours it); the working files survive; the dangling commit is
recoverable with `git fsck --lost-found` until gc. What is unrecoverable is the per-worktree index
(the staged/unstaged distinction), its reflog, and the link. A reader who holds that an absent
directory is abandoned would tier this **3** — a silent repository mutation where the sibling door
narrates."* And what argues the other way, from the same row: *"The plausible everyday trigger is
not exotic, though it was **not driven**: a repository bind-mounted into a container (how this
trial's sessions ran), whose host-side linked worktrees live at paths the container cannot see, is
`prunable` in every one of them from where jigc runs."*
**[Corrected 2026-10-04 by the adversarial re-drive:** that trigger — this record's O-1 — **was
driven, and it holds.** A live host worktree, never moved and never deleted, answered `git status`
at exit 0 before; in a container that saw the repository and not the worktree's path, ordinary git
left the `prunable` record in place and `jigc milestone provision` exited 0 with its one
`provisioned 2 worktree(s)…` line; afterwards, on the host, the worktree's record under
`.git/worktrees/` was gone, `git status` there answered `fatal: not a git repository` at exit 128,
`git worktree repair` exited 1, and the commit only that worktree's `HEAD` reached was unreachable.
A worktree moved with plain `mv` and still working at its new path is orphaned the same way
([tier1-verification-L-22.md](tier1-verification-L-22.md) → 6). The verifier's sentence above is
kept as written; *"not driven"* was true of the first pass only.**]**

### Tier 2

#### L-3 · Two `jigc start` refusals answer `--format json` with a code-less `{"error": …}` and log no identity — **PARTIAL**

`tier:` tier-2, in its weakest form — the *code-less refusal* member of the scale; **not** a dead
end (both printed routes run at exit 0) and **not** a contract breach.
`door:` `jigc start`
`found-in:` `trial:RC-rc24/a` (ordinals 4 and 41)

`repro:`

```sh
jigc --version                                   # jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC config set invocation-log true
git -C "$REPO" add .jigc/config/manifest.yaml && git -C "$REPO" commit -q -m "chore: enable invocation log"

# (i) a task-minting workflow named with no intent
$JIGC start --workflow planning --format json; echo "exit $?"
#   exit 1 · stdout 0 bytes · stderr: { "error": "workflow 'planning' requires an intent: jigc start \"<intent>\" --workflow planning" }
#   log: "exit_code":1, "finding_codes":[], "error_code":null

# (ii) a milestone sub-task resumed from the shared checkout
$JIGC milestone create "M" --format json; $JIGC milestone add-task m "t" --format json
$JIGC start --task t --format json; echo "exit $?"
#   exit 1 · stdout 0 bytes · stderr: { "error": "task `t` is pinned to base <a> but you're on <b> — this is a
#     sub-task of milestone `m` … run `jigc milestone provision m` … then `cd $REPO/.jigc/worktrees/t` and re-run this command there" }
#   log: "exit_code":1, "finding_codes":[], "error_code":null

# controls — same door, same rig, same format: a refusal WITH an identity
$JIGC start --task nope --format json       # exit 1 · a findings document, key (finalize.no-task, task:nope); log finding_codes ["finalize.no-task"]
$JIGC start --workflow nope --format json   # exit 1 · {"error": "blocking · workflow-refs.unknown-workflow — …"}; log finding_codes carries the code

# the routes each refusal prints, run as printed — both work
$JIGC start "plan the thing" --workflow planning --format json    # exit 0
$JIGC milestone provision m --format json                         # exit 0
( cd "$REPO/.jigc/worktrees/t" && $JIGC start --task t --format json )   # exit 0
```

`contract:` **no `design/` rule is contradicted** — `design/command-output-contract.md` → *The two
reject arms* declares `{"error": …}` for *"a reject with no finding behind it"*, and both producers
are bare `anyhow`. What the code-less half sits against is the project's own stated claim and
precedent: `implementation/roadmap.md` → Milestone 52, **The claim** — *"every repository posture
and every route a caller can reach answers with a code and a followable route"*; the tier scale's
middle tier, *"code-less or undeclared refusals"*; and the fix the same door already took once
(`start_compose::a_title_that_slugs_to_nothing_refuses_under_the_mint_class_code`, M53 Increment
5). A driver gets a sentence to show a human and nothing to key on; the log gets an anonymous
exit 1. `error_code: null` is the designed value and discriminates nothing — the datum is
`finding_codes: []`.

`UNPINNED:` no test drives either cell under `--format json` or reads its log record. Pinned
nearby, on the text arm only: `start_compose::form_d_creates_task_workflow_with_no_intent_rejects`
and `start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal`.

### Tier 3

#### L-2 · Orientation prints ``Run: `jigc start --workflow planning` ``, and that line run as printed exits 1 — **CONFIRMED**

`tier:` tier-3
`door:` `jigc start`
`found-in:` `trial:RC-rc24/a` (ordinal 3 → 4)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                  # jigc 1.0.0-rc.24
jigc start | command grep -n 'workflow planning\|workflow ingest-existing'
#   Run: `jigc start --workflow planning`   — plan a milestone — decompose it into increments and tasks
#   Run: `jigc start --workflow ingest-existing`   — bring an existing repo's docs under management
jigc start --workflow planning                   # exit 1, stdout empty
#   stderr: workflow 'planning' requires an intent: jigc start "<intent>" --workflow planning
jigc start --workflow ingest-existing            # exit 0 — the sibling line runs as printed (the lead is one line wide)
jigc start "plan milestone one" --workflow planning   # exit 0 — the form the refusal names
git -C "$REPO" status --porcelain                # unchanged after the refusal: nothing opened, nothing committed
```

`contract:` `design/surface-contract.md` → **Law 1 (nothing lies)** and → *The route fence (law
2)*: *"A mechanical route that hard-rejects, or repairs the wrong thing, is law 2 failing one
level down"*. The surface's own convention two lines above spells the placeholder where one is
required (``Run: `jigc start "<intent>"` ``); the refusal, the workflow's front-matter and
`design/methodology-docs.md` all spell the route with an intent. The line's shape is gated on pack
membership, never on `creates-task`.

`pinned-by:` each half separately — `compose_goldens::sweep_fresh` (the printed line, in five
state goldens) and `start_compose::form_d_creates_task_workflow_with_no_intent_rejects` (the
refusal). `UNPINNED:` the contradiction between them — no test runs an orientation `Run:` line as
printed.

#### L-4 · Orientation's resume line for a milestone sub-task refuses from the checkout that printed it; no forward surface names `milestone provision` or `milestone execute` — **CONFIRMED**

`tier:` tier-3 — on the resume line alone. *Names neither door* is, taken alone, a discoverability
observation the project has already ruled no contract (RC-1.0-gate B2-2; M46's razor).
`door:` `jigc start` (orientation) · `jigc start --task <sub-task>` · `workflow:planning` ·
`jigc milestone add-task`
`found-in:` `trial:RC-rc24/a` (ordinals 40 → 41 → 42)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc --version                                   # jigc 1.0.0-rc.24
jigc milestone create "M"                        # exit 0 — acks `next: jigc milestone add-task m "<intent>"`
jigc milestone add-task m "t one"                # exit 0 — three lines, no `next:`
jigc milestone add-task m "t two"                # exit 0 — the same
jigc start > start.out                           # exit 0
command grep -cE 'milestone (provision|execute)' start.out      # 0
#   per sub-task exactly four Run: lines — start --task · task validate · milestone finalize · task discard
#   Run: `jigc start --task t-one`   — resume: re-composes this task's own workflow where it left off
jigc start --task t-one                          # exit 1, stdout empty — the printed line, verbatim, same checkout
#   task `t-one` is pinned to base <setup> but you're on <add-task-2> — … run `jigc milestone provision m` …
jigc start "plan M" --workflow planning | command grep -cE 'milestone (provision|execute)'   # 0
git rev-parse HEAD; git status --porcelain       # unchanged; nothing provisioned

jigc milestone provision m                       # exit 0 — the route works
cd "$REPO/.jigc/worktrees/t-one" && jigc start --task t-one     # exit 0
```

The cause is isolated: `milestone create` pins the base at the `HEAD` it runs on and then lands its
own record commit, so every sub-task is off its pin in the main checkout from the moment it
exists, and the printed resume line refuses there on every run.

`contract:` `design/surface-contract.md` → **Law 1 — nothing lies** (*"behaviour claims match
knobs"*): the gloss *"re-composes this task's own workflow where it left off"* is false for this
unit kind in this checkout. The codebase's own rule for this very row —
`crates/cli/tests/orientation_active_task.rs`: *"a route this binary's own guard blocks is a
route-floor defect"* — is applied to the commit and abandon directives of the same block and not
to the resume directive three lines above them.

`pinned-by:` the refusal and its route —
`start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal`,
`anyhow_route_spans::pinned_base_mismatch_routes_by_unit_kind`. `UNPINNED:` the contradicting cell
— orientation's `jigc start --task <sub>` span lifted off the rendered bytes and run where it was
printed; pinning the current pair would pin the defect as expected output.

#### L-6 · Orientation's `Project config:` header path is absolute, and falls under neither of the two absolute kinds `.jigc/AGENT.md` names — **CONFIRMED**

`tier:` tier-3
`door:` `jigc start` · `guide:AGENT.md`
`found-in:` `trial:RC-rc24/a` — and (b) and (c): ordinal 3 of every arm

`repro:`

```sh
jigc --version                                   # jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc start | sed -n 3p
#   Pack: dev/1.0.0-rc.24 | methodology/1.0.0-rc.24 · Project config: <REPO>/.jigc/config   (begins with `/`; no backtick)
jigc start --format json | command grep '"header"'              # the same absolute path
git -C "$REPO" worktree list | wc -l             # 1 — it is not "a checkout that is not the repository root"
command grep -n 'Two kinds of printed path are absolute' "$REPO/.jigc/AGENT.md"    # line 7
command grep -c 'Project config\|orientation header\|Team config' "$REPO/.jigc/AGENT.md"   # 0
```

`contract:` `.jigc/AGENT.md`, the printed-path paragraph (source `crates/cli/src/adapter.rs` →
`BOOTSTRAP_PATHS_AND_CWD`): *every path jigc prints is relative to the repository root*, with
exactly two exceptions; the header is neither. The product's own design doc admits a **third**
reason and names this very line (`design/surface-contract.md` → *The printed-path fence (law 1)*),
so the stale half is the preload's sentence. No dead end follows: an absolute path resolved
against anything is itself.

`pinned-by:` each half on its own — `compose_goldens::sweep_fresh` (the header; the *Two kinds*
sentence) and `repo_relative_paths::every_path_a_door_prints_carries_a_disposition_the_source_backs`.
`UNPINNED:` the disagreement — no test relates the preload's enumeration to the `DeclaredAbsolute`
rows.

#### L-10 · The amend step says the amended message carries only the trailer items you add; the binary lands the co-author trailer with none added — **CONFIRMED**

`tier:` tier-3
`door:` `jigc task amend` (`workflow:amend`, `step:amend-message`)
`found-in:` `trial:RC-rc24/b` (ordinals 19, 26; commit `4b80510`)

`repro:`

```sh
jigc --version                                   # jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
# under an agent (the variable the trailer keys on: set), an ordinary task commit
jigc start --workflow quick-fix "add a greeting file" --format json
printf 'hello\n' > greeting.txt; git add greeting.txt
jigc doc set-field commit:add-a-greeting-file#type --value feat --task add-a-greeting-file
jigc doc set-slot  commit:add-a-greeting-file#summary --from-file - --task add-a-greeting-file <<'EOF'
add a greeting file
EOF
jigc task finalize add-a-greeting-file           # exit 0 → <old>

jigc task amend                                  # exit 0 — task amend-<old>; the composed text says:
#   Trailers are re-authored too — the amended message carries only the trailer items
#   you add here, so re-add any the old message had that still apply:
#   jigc doc add-item commit:amend-<old>#trailers --title Co-Authored-By --task amend-<old>
jigc doc set-field commit:amend-<old>#type --value feat --task amend-<old>
jigc doc set-slot  commit:amend-<old>#summary --from-file - --task amend-<old> <<'EOF'
add the greeting file
EOF
jigc doc show commit:amend-<old> --task amend-<old> --format json   # /sections/trailers => []
jigc task finalize amend-<old>                   # exit 0 — amended <old> → <new>
git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)'      # Claude <noreply@anthropic.com>
# cells: variable unset over a HEAD that carries the trailer → still lands (the carry);
#        variable unset over a HEAD that carries none → none (the control);
#        an authored `Refs` item IS dropped unless re-added — the sentence holds for authored items only
```

`contract:` `design/surface-contract.md` → **Law 1 — nothing lies**. The step text
(`crates/cli/packs/dev/steps/amend-message.yaml`) states a rule that two design sections make
false for the very key it uses as its example: `design/assistant-adapter.md` → *The co-author
trailer* → **The amend arm** (*"re-derived — and **carried over** when `HEAD`'s message already
carries the profile's trailer"*) and `design/finalize.md` → the amend arm. The binary does what
its design says; the step text predates the trailer and was not revised.

`pinned-by:` the binary's behaviour —
`agent_co_author::amend_preserves_the_trailer_without_duplicating_it`,
`agent_co_author::a_human_amend_of_a_human_commit_lands_no_trailer`. `UNPINNED:` the sentence — no
suite reads the composed amend text against the commit seam, and no golden holds the text
`jigc task amend` prints.

#### L-11 · Composed commit steps print the bare `#type` / `#scope` alias while `doc schema commit` — named in the same step as the authority on *every address a write can take* — lists only `#header/…` — **PARTIAL**

`tier:` tier-3, weak — for the two-spellings half only. The worker's *"would have failed"* is
**refuted** and carries no tier.
`door:` `jigc doc set-field` · `jigc doc schema` · `workflow:dev-task` (`step:author-commit`)
`found-in:` `trial:RC-rc24/b` (ordinals 9–11 against 20–21; also a#25–26, c#11)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                   # jigc 1.0.0-rc.24
T=fix-get-summary-with-empty
$JIGC start --workflow dev-task "Fix GET /summary/ with empty series name (TKT-212)"   # exit 0
#   Run: `jigc doc set-field commit:fix-get-summary-with-empty#type --value <TYPE> --task …`
#   The `commit` schema is the authority on what you write into it — … and every address a write can take:
#   jigc doc schema commit
$JIGC doc schema commit
#   - type: enum […] (section: header) (set-field: commit:<slug>#header/type) *
#   - scope: string (section: header) (set-field: commit:<slug>#header/scope)      — no line carries `#type` or `#scope` bare
$JIGC doc set-field "commit:$T#type" --value fix --task $T            # exit 0
$JIGC doc set-field "commit:$T#header/type" --value feat --task $T    # exit 0 — the same field, overwritten
$JIGC doc show "commit:$T" --task $T --format json                    # "fields": { "type": "feat" }
$JIGC doc set-field "commit:$T#nosuch" --value x --task $T            # exit 1 · write.unknown-field — the door does refuse a real miss
```

`contract:` the composed step's own words — the schema read is the authority on *"every address a
write can take"* — read against `design/surface-contract.md` → **Law 1**. The schema read lists
one spelling per leaf; the write door takes a second, and it is the one the same step prints three
lines above the sentence. The alias itself is designed and documented (`jigc doc set-field --help`:
*"`<type>:<slug>#<field>` (or `#<section>/<field>`)"*). **A re-report:** RC-alpha3 adjudicated the
same worker inference REFUTED and recorded the residue — *"pick the qualified form for printed
surfaces or state the alias once"*; the fix picked the qualified form on `doc schema` alone.

`pinned-by:` each side separately — `doc_read_surface::documented_alias_forms_round_trip`,
`doc::tests::schema_read_surface_picks_the_qualified_commit_address`, `compose_goldens::sweep_fresh`.
`UNPINNED:` the relation between the two — no test compares a composed step's printed address with
the spelling `doc schema` advertises for the same leaf.

#### L-18 · A refused `doc author` payload carries its only real coordinate as prose, while the JSON `location` prints a *no coordinate known* sentinel where the contract says `null` — **CONFIRMED**

`tier:` tier-3, weak — for the `location` half. The route half (*"retry … with a conforming
value"*) contradicts nothing and is `none`.
`door:` `jigc doc author` · `write.wrong-shape`
`found-in:` `trial:RC-rc24/a` (ordinal 20)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc --start planning "Bound what the service will accept") || exit
eval "$rig"; [ -n "$REPO" ] || exit; T=$RIG_TASK
$JIGC --version                                  # jigc 1.0.0-rc.24
jigc doc create deferral-ledger --title "Deferral Ledger" --task $T --format json   # exit 0
# bad.yaml — an item carrying `set-fields:` on line 6, column 9 (the session's own payload shape)
jigc doc author deferral-ledger --from-file - --task $T --format json < bad.yaml    # exit 1
#   "code": "write.wrong-shape", "key": { "code": "write.wrong-shape", "target": "deferral-ledger" },
#   "message": "… unknown field `set-fields`, expected one of `title`, `set`, `sections` at line 6 column 9",
#   "location": { "address": "deferral-ledger", "line": 1, "col": 1 },
#   "route": "retry `jigc doc author deferral-ledger` with a conforming value"
jigc doc author deferral-ledger --from-file - --task $T < bad.yaml                  # exit 1 — the text arm prints `at: deferral-ledger`, no line
# the position in the message moves with the payload (line 2 column 3 · line 2 column 1 · line 5 column 9); `location` never does
jigc doc author deferral-ledger --from-file - --task $T --format json < good.yaml   # exit 0 — the route, followed
```

`contract:` `design/command-output-contract.md` → *3 · The findings envelope*, the `location`
bullet — *"line/col when the finding has a source position, else `null`"*. `design/validation.md`
states *"Line 1 is this codebase's **no coordinate known**"*; that rule is implemented for the
text arm only, and the JSON arm serializes the unclaimed coordinate. Weak because the field is
declared advisory-only and outside the stable key, and no driver decision in the trial turned on
it.

`UNPINNED:` both halves.
`author_payload_floor::every_payload_refusal_emits_the_findings_envelope_with_a_resolving_key`
drives this refusal class under `--format json` and asserts nothing about `location.line` /
`location.col`; the sibling text test pins the suppression that is right.

### Confirmed or partial, with no tier — reproduced, and contradicting no contract

Fourteen rows. Each reproduces exactly; each is the binary doing what its design of record says.
None is a defect on the tier predicate. They are recorded because a blind worker met them, and
because several are the measured cost of a design choice the human may want to revisit.

#### L-1 · On the path a worker is handed, the router catalog line is the one surface naming `report-inconsistency` — **PARTIAL**

`tier:` none · `door:` `workflow:architecture-documentation` · `jigc start` ·
`found-in:` `trial:RC-rc24/c`

`repro:`

```sh
# wart corpus (instantiate.sh WITHOUT --clean-prose), then `jigc setup` on rc.24
jigc start | command grep -c report-inconsistency                                   # 1 — one of 13 catalog lines
jigc start --workflow architecture-documentation "x" | command grep -ciE 'inconsisten|disagree|contradict|report-'   # 0
jigc describe | command grep -c report-inconsistency                                # ≥ 1 — the falsifier for "only"
jigc workflow report-inconsistency --preview | wc -l                                # 127 — the whole filing workflow, no mint
```

`contract:` none contradicted. `design/findings-channel.md` §2 makes the workflow router-visible
through a `when:` hint and nothing more; no rule says a work-workflow's composed text
cross-references the channel. Whether one catalog line is enough for a worker already inside
another workflow is a design question for the human — and the first surface arm (c)'s outcome
points at.
`pinned-by:` `findings_workflows::the_catalog_lists_report_inconsistency_and_none_of_the_hidden_three`;
`compose_goldens::sweep_fresh` (the composed text, 0 matches — the absence is pinned as bytes only).

#### L-7 · The pre-commit hook's `jigc validate --format json` lands in the invocation log with no caller marker — **CONFIRMED**

`tier:` none · `door:` `jigc validate` · `jigc setup` (the hook) · `found-in:` `trial:RC-rc24/c`
(ordinal 14; also b#16, #25 and a#30, #32, #34, #36, #38, #47)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC config set invocation-log true
# … mint a task, stage a file, author type + summary …
wc -l < "$REPO/.jigc/logs/invocations.jsonl"     # 3
$JIGC task finalize add-a-greeting-helper        # exit 0
tail -2 "$REPO/.jigc/logs/invocations.jsonl"     # {"argv":["validate","--format","json"],…} then the finalize record
# a raw `git commit` appends one such record too; `git commit --no-verify` appends none
```

`contract:` none contradicted. `design/measurement.md` → *The in-repo invocation log*: *"one JSONL
record per `jigc` invocation"*, eight keys, closed and additive-only, none naming a caller. A
reader who wants the worker's own calls subtracts the hook's by position — a reading rule for the
trial tooling (T-8).
`pinned-by:` `probe_failure_doors::the_precommit_hook_records_each_probe_failure`;
`invocation_log::tests::the_emitted_key_set_is_closed_by_the_destructured_fields`.

#### L-8 · The hook's in-flight store sweep reports `un-baselined` / `hash-matches` for the commit's own writes; a post-commit `validate` is clean at every door — **CONFIRMED as an observation**

`tier:` none — the condition set for a tier (*a post-commit `validate` still reports them*) is
**refuted** · `door:` `jigc validate` (as the hook runs it) · `found-in:` `trial:RC-rc24/c`
(ordinal 14; also a#30, #32, #34, #36, #38, #47)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc config set invocation-log true; git add .jigc/config/manifest.yaml; git commit -m "chore: enable the invocation log"
# land two docs through `task finalize`, then `milestone create`, two `milestone add-task`, `milestone finalize`
# in flight (the hook's record in the log): file-state.un-baselined per doc the commit lands; file-state.hash-matches for the record
jigc validate --format json                      # after EACH of the five commits: findings [], blocking_probes [], exit 0
```

`contract:` none contradicted. `design/finalize.md` declares the order (`git commit` → post-commit
hash advance), so a hook between the two reads the pre-advance record by construction; the hook
discards the report and prints nothing. Residue, **not driven**: a user-authored hook that failed
a commit on a non-empty `blocking_probes` would meet it on every milestone record commit.
`pinned-by:` the steady state —
`l1_pull_absorption::after_the_landed_finalize_the_store_sweep_no_longer_reports_the_lag`,
`milestone_record_reconcile::clean_record_add_task_stays_green`. `UNPINNED:` the in-flight reading
itself.

#### L-9 · The `task amend` composed text prints a bare `git log -1 --format=%s` — **PARTIAL**

`tier:` none · `door:` `jigc task amend` (`step:amend-message`) · `guide:AGENT.md` ·
`found-in:` `trial:RC-rc24/b` (ordinal 19)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
# land one task commit, then from a subdirectory:
$JIGC task amend > <tmp>/amend.txt               # exit 0
command grep -n 'git ' <tmp>/amend.txt           # `git log -1 --format=%s` (to run) · `git commit --amend` (prose)
command grep -c 'git -C' <tmp>/amend.txt         # 0
git log -1 --format=%s                           # the pinned commit's subject, from the subdirectory
```

`contract:` none contradicted. `.jigc/AGENT.md`'s `-C` rule, read whole, is scoped to spans
*"wherever a relative [path] would resolve against your directory"*; this span carries no path.
The aimed form would be **wrong** in one driven cell: minted in a linked worktree,
`git -C <the repository's absolute path> log -1` reads the main checkout's `HEAD`, a different
commit. A narrower cell, driven and not tiered, is under *Open* below.
`UNPINNED:` the span's spelling — no golden holds the text `jigc task amend` composes.

#### L-12 · The pre-commit forecast stops at the subject line; no surface renders the trailer block before the commit — **PARTIAL**

`tier:` none · `door:` `jigc task finalize` · `jigc task validate` · `found-in:` `trial:RC-rc24/b`

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc --start dev-task "fix the summary helper on empty input") || exit
eval "$rig"; [ -n "$REPO" ] || exit; T=$RIG_TASK
# author type, scope, summary, body and one `Refs` trailer item; stage one code file
jigc task finalize $T --dry-run                  # "would commit — fix(summary): return an empty string for empty input" — the header IS forecast
jigc task finalize $T --dry-run --format json    # keys: dry_run, findings, left_out, manifest, subject — no rendered trailer line
jigc task finalize $T; git log -1 --format=%B    # the trailer block, and the adapter's co-author line, first visible here
```

`contract:` none contradicted. `jigc task finalize --help` claims *the forecast subject line* and
delivers it; `design/finalize.md` puts the co-author line *"at the commit seam, after this render
and outside the `commit` schema"*. The header third of the worker's claim is **refuted** — and the
session's log holds zero `--dry-run` rows, so it never met the door that answers it; the composed
step does not route there.
`pinned-by:` `finalize_dry_run_subject::the_dry_run_forecasts_the_subject_the_commit_lands`.
`UNPINNED:` the gap — there is no behaviour to pin.

#### L-13 · `jigc start "<intent>"` composes the router byte-identically for every intent — **CONFIRMED**

`tier:` none · `door:` `jigc start` · `found-in:` `trial:RC-rc24/b` (ordinal 4)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc start "a" > 1; jigc start "write the architecture doc" > 2; cmp 1 2      # identical, 2026 bytes each
# five intents × three forms (text, --format json, --explain): one sha per form; nothing minted, nothing written
```

`contract:` none contradicted. `design/workflow-dialect.md` → *Workflow selection — the router
default*: *"The agent picks; the CLI never does."* A ruled precedent (`pinned_facts.rs`, Part D
row D11); the residue is parked at `ideas/spec-router-matching.md`.
`pinned-by:` `compose_goldens::sweep_fresh` (surface `start-intent`), under one intent.
`UNPINNED:` as a property — no test drives two intents and asserts equal output.

#### L-14 · The composed `planning` text carries eight jigc milestone tags, *"the orchestrator"* and a Rust parameter name, unattributed, in an adopter's repository — **PARTIAL**

`tier:` none · `door:` `workflow:planning` · `found-in:` `trial:RC-rc24/a` (ordinal 5)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC start "x" --workflow planning > <tmp>/planning.txt          # exit 0 · 426 lines · 44518 bytes
command grep -oE '\bM[0-9]{2}\b' <tmp>/planning.txt | sort | uniq -c
#   2 M34 · 10 M42 · 4 M44 · 4 M45 · 4 M46 · 2 M47 · 2 M48 · 2 M55     (8 distinct, 30 total, 14 lines)
command grep -o 'the orchestrator' <tmp>/planning.txt | wc -l     # 6
command grep -o 'FileStateRecord' <tmp>/planning.txt | wc -l      # 2
```

`contract:` none contradicted — the design of record **mandates** the text
(`design/methodology-docs.md` → *The planning gate-record*: *"those sentences move **verbatim**
into the schema's `hint:`"*). The measured handles: the seven lines carrying another project's
history are **48 %** of the composition; no sentence on any surface attributes them; and the tags
live in the namespace the pack hands the adopter for its own milestones. *"Cannot tell"* is not
carried — this worker inferred correctly, from an empty `doc list`. Only `planning` carries a tag.
`pinned-by:` `planning_record_schema::the_composed_schema_projection_carries_every_gate_cell_verbatim`.
`UNPINNED:` the opposite property — that adopter-facing pack prose names nothing only the jigc
repository resolves.

#### L-15 · The `planning` composition is 45,112 bytes under `--format json` and exceeds the agent harness's inline limit — **CONFIRMED**

`tier:` none · `door:` `workflow:planning` · `found-in:` `trial:RC-rc24/a` (ordinal 5)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC start "Bound what the service will accept" --workflow planning --format json | wc -c   # 45112
$JIGC start "x" --workflow planning --format json | wc -c                                    # 44039 — the figure is intent-dependent
```

`contract:` none contradicted — and the project records that it has no such rule:
`ideas/composed-context-token-budget.md` (*"the compiler is blind to the size of what it emits"*),
parked, unscheduled. Nothing was lost: the worker read the whole composition back from the spill
file in one extra tool call. With L-14: nearly half of those bytes are the gate text above.
`pinned-by:` `compose_goldens::sweep_fresh` (the text, byte for byte). No test asserts a ceiling;
none is declared.

#### L-16 · `milestone create` / `milestone add-task` each land a record-only commit at once, while `doc` writes stay in the task working area until finalize — **CONFIRMED as an observation**

`tier:` none · `door:` `jigc milestone create` · `jigc milestone add-task` ·
`found-in:` `trial:RC-rc24/a` (ordinals 33–39)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc start --workflow park-idea "probe doc persistence"; jigc doc create idea --title "Cold cache eviction" --task probe-doc-persistence
git rev-parse --short HEAD; git status --short   # HEAD unmoved, index empty — the bytes live under .jigc/tasks/<id>/docs/
jigc milestone create "M"                        # exit 0 — HEAD moves by one record-only commit; the ack names its sha
jigc milestone add-task m "t"                    # exit 0 — and again
```

`contract:` none contradicted. `design/team-ready-state.md` → *The commit model*; both doors'
`--help`; every ack (`record commit: <sha> — … committed on its own`); and protocol §0.2 declared
it before the run. The worker's own words are *"I hadn't expected"*, and its debrief says nothing
jigc told it turned out wrong.
`pinned-by:` `milestone_record_create::methodology_create_materializes_record_and_path_scoped_commits`;
`milestone_record_add_task::methodology_add_task_appends_record_byte_stable_and_path_scoped_commits`;
`help_truth::committing_doors_say_so::every_committing_door_help_states_the_commit_it_lands`.

#### L-17 · Nothing on the worker's path names `milestone join` or any pre-boundary check of the combined code — **PARTIAL**

`tier:` none · `door:` `jigc milestone join` · `jigc milestone finalize` · `jigc start`
(orientation) · `found-in:` `trial:RC-rc24/a` (tool calls #62–#65)

`repro:`

```sh
# rig 1: two sub-tasks on disjoint files, each passing the project's own check alone and failing together
jigc milestone join bound-inputs                 # exit 0 — "joined … 0 doc(s) merged"; NO checkout holds both changes
jigc milestone finalize bound-inputs             # exit 0 — both land exactly as staged; the project check at HEAD exits 1
# rig 2: two sub-tasks staging the same path
jigc milestone join collide                      # exit 1 · combine.code-collision — the one thing join does check
# rig 3: rig 1 with the project's check as its pre-commit hook
jigc milestone finalize bound-inputs             # exit 1 — "git commit was rejected (no commit was made)"; HEAD unchanged
```

`contract:` none contradicted. `design/validation.md`: *"`jigc milestone join` … is a report-only
pre-check that commits nothing"*; `design/finalize.md` builds the combined tree *"off-line (a
throwaway index)"* — a tree an agent could test before the boundary is designed out, not missing,
and the designed channel for the combined tree's tests is the project's own pre-commit hook (rig
3). Two corrections to the lead: **`milestone join` would not have served the need even if
named**, and **the scratch worktree did not catch the break** (§4.3). What survives is the same
routing gap as L-4.
`pinned-by:` `milestone_join_collision::disjoint_worktrees_keep_join_clean`;
`milestone_join_collision::colliding_worktrees_block_join_naming_the_path`. `UNPINNED:` the
absence, and `finalize` exiting 0 over a tree that fails the project's tests — a non-behaviour by
design.

#### L-20 · Sub-task composed text carries `step:implement`'s un-scoped *"before finalize"* wording beside *"never `jigc task finalize` here"* — **PARTIAL**

`tier:` none · `door:` `workflow:sub-task` · `found-in:` `trial:RC-rc24/a` (ordinal 43)

`repro:`

```sh
# create a milestone with two sub-tasks, provision, then from a sub-task's worktree:
jigc start --task add-helper-b                   # exit 0, 72 lines
#   "`git add` your code edits before finalize — it commits only what you have staged" … "never `git commit` and never `jigc task finalize` here"
jigc task finalize add-helper-b                  # exit 3 — refuses with the milestone route; lands nothing
```

`contract:` none contradicted. Every surviving sentence is true of the one boundary a sub-task has
(`jigc milestone finalize <m>`); the omission rule in `design/workflow-dialect.md` →
*Composing for a fan-out sub-task* removes steps that run `jigc task finalize`, and
`step:implement` is not one. The door the wording could mis-send a reader to refuses with a
working route.
`UNPINNED:` no test asserts or forbids the un-scoped wording. Pinned around it:
`sub_task_composition::a_sub_task_composes_no_per_task_finalize_door_through_either_door`.

#### L-21 · `jigc start --task <sub>` inside the worktree is a designed second door beside `jigc workflow sub-task --task <sub>` — **CONFIRMED**

`tier:` none · `door:` `jigc start --task` · `jigc workflow <W> --task` ·
`found-in:` `trial:RC-rc24/a` (ordinals 43–45)

`repro:`

```sh
# in a provisioned sub-task worktree
diff <(jigc start --task cap-the-store) <(jigc workflow sub-task --task cap-the-store)   # empty — text and --format json alike
ls .jigc/tasks/cap-the-store/docs 2>/dev/null    # only the workflow door provisions the sub-task's docs area
```

`contract:` none contradicted. `design/workflow-dialect.md` names both doors under one composition
rule; `design/write-commands.md` states the one asymmetry (the workflow door provisions the docs
area on first entry). Under the default squash nothing reads a sub-task's commit doc, so a worker
cannot tell the doors apart.
`pinned-by:` `sub_task_composition::a_sub_task_composes_no_per_task_finalize_door_through_either_door`.
Bound: no test compares the two doors' output **to each other**.

#### L-23 · Co-author trailer placement under a body that ends in `Key: value` — **PARTIAL**

`tier:` none · `door:` `jigc task finalize` · `found-in:` `trial:RC-rc24/b` (commits `7757a4c`,
`4b80510`; also a's `a9f1851`)

`repro:`

```sh
# per cell: a single-task commit whose body slot ends in the paragraph under test, finalized with the variable set / unset
#   body ends `Refs: TKT-1`, set    → co-author joined with no blank line; git reads both as trailers
#   body ends `Refs: TKT-1`, UNSET  → no jigc line; git STILL reads `Refs: TKT-1` as a trailer   ← refutes the causal clause
#   body ends in prose, set         → one blank line, then the co-author line
#   prose line then `Note: something`, set → co-author in a block of its own; `Note:` stays prose   ← the unread edge: works
git log -1 --format='%(trailers:only,unfold)'
```

`contract:` none contradicted inside the lead's scope. `design/assistant-adapter.md` →
**Placement and de-duplication**: *"the place `git interpret-trailers` gives a trailer added at
the end"* — and in every cell the lead names, jigc's bytes equal what `git interpret-trailers`
produces. One sibling edge does diverge; it is under *Open* below.
`pinned-by:` `adapter::tests::sign_places_the_trailer_in_the_trailer_block` (unit seam).
`UNPINNED:` the `Word: text` edge, and a body-slot `Key: value` paragraph driven through
`task finalize`.

#### L-24 · `task validate` / `task finalize` report one advisory `file-state.staged-copy` (route *"no action needed"*) per **persisted** staged doc — **PARTIAL**

`tier:` none · `door:` `jigc task validate` · `jigc task finalize` · `file-state.staged-copy` ·
`found-in:` `trial:RC-rc24/c` (ordinals 13, 15; also a#29, #31)

`repro:`

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC start "record a decision about retries" --workflow single-task --format json; T=record-a-decision-about-retries
$JIGC task validate $T --format json             # only the transient commit doc staged → NO file-state.staged-copy
$JIGC doc create adr --title x --task $T
$JIGC task validate $T --format json             # one advisory file-state.staged-copy, at the doc's repo-real destination
```

`contract:` none contradicted. `design/validation.md` lists it among *"Informational outcomes —
emitted advisory, intentionally not tunable"*; the route's promise (*"baselined when its finalize
lands"*) holds. Noise at most: one advisory line per persisted staged doc at each of three doors.
`pinned-by:` `dry_run_findings_equal_set::the_three_doors_emit_one_code_set_for_every_previewed_member`.

### Refuted — each with its datum

A refuted row is expected and is itself a signal: a complaint that dissolves into a capability
that shipped.

| lead | claim | the datum that refutes it | pinned-by |
|---|---|---|---|
| **L-5** | orientation offers `jigc milestone finalize <id>` while every sub-task shows *nothing staged yet*; run verbatim it might land an empty boundary at exit 0 | run verbatim it **refuses at exit 3** with one blocking `milestone.zero-contribution`, keyed at `milestone:<id>`, whose route names both exits (`milestone provision` → work → re-run, or `milestone discard`). Nothing lands, the record stays `active`, the route's first span runs at exit 0 and the re-run after staging lands | `commit_rejected_axis::no_committing_door_dresses_an_empty_commit_as_a_rejection`; `milestone_zero_contribution::a_fresh_clone_cannot_silently_land_a_zero_contribution_finalize` |
| **L-19** | `jigc doc list --format json` prints a `note: docs are also staged in open task …` line after the document, so the output may not be one JSON document | the note rides **stderr**. With the streams split, stdout is 1132 bytes that parse as one document and contain no `note:`, byte-identical with and without the open task; stderr is the note alone. Both of the trial's captures had merged the streams | `doc_list::a_task_less_listing_routes_at_the_staged_read` |

Refuted **halves** inside PARTIAL rows, each with its datum in the row above: L-11's *"would have
failed"* (every bare-form `set-field` in the three logs exits 0); L-12's header third
(`task finalize --dry-run` prints the subject); L-17's two clauses (join builds no tree; the
scratch worktree caught nothing); L-22's lead as filed (a live foreign worktree is byte-identical
after every door); L-23's causal clause; L-8's tiering condition.

### Open, and not driven — each with its reason

**No lead was left undriven: all 39 have a verdict.** What follows is the residue the verifiers
named and left — cells driven outside a lead's claim and not adjudicated, and edges not driven at
all. None is a finding; each is a lead for whoever triages.

| # | what | state | reason it is open |
|---|---|---|---|
| O-1 | **L-22's everyday trigger**: a repository bind-mounted into a container whose host-side linked worktrees sit at paths the container cannot see is `prunable` in every one of them from where jigc runs | **not driven** | an inference from git's `prunable` rule; a container run was outside a rig verification. It is the datum that decides how far L-22's tier argues down |
| O-2 | L-22: `jigc uninstall` and the squash combine's dedicated worktree (`crates/cli/src/task.rs`, a third bare prune); the variant under `--format json` | read, **not driven** | outside the three doors the lead named |
| O-3 | **L-23's sibling edge**: a body whose last paragraph is a prose line followed by `Signed-off-by: …`. Signed, jigc opens a new block and git then reads the new block alone — the body's `Signed-off-by` is demoted from a trailer git reads to prose. Not *"the place `git interpret-trailers` gives"* | **driven once** (two cells), outside the lead's claim, **not adjudicated** | its verifier's reading: tier 3 at most — no bytes are lost; reachable only by writing a sign-off into the prose body slot. UNPINNED |
| O-4 | **L-18's adjacent**: one code, `write.wrong-shape`, leaves `doc author` with two different routes, and the design sentences define that code as *"a genuine declared-shape defect, and only that"* — the payload-parse producer is a second meaning they do not carry | **driven** (three cells), **not scored** | offered by the verifier as a separate lead |
| O-5 | **L-9's narrower cell**: the step says the bare `git log -1` shows *"the commit this task pinned"*; it reads `HEAD` of whichever checkout the reader stands in. Mint in a linked worktree, run from the main checkout, and it prints another commit's subject | **driven** (one cell), **not tiered** | outside the lead as worded; exposure is small (the mint ack names the worktree; finalize refuses if `HEAD` moved) |
| O-6 | L-10: the composed example followed literally with a *different* address lands a second co-author line beside the adapter's; a profile without the `co-author` key; a non-Claude assistant | **not driven** | read from the de-duplication rule and its unit tests |
| O-7 | L-8's residue: a user-authored hook that fails a commit on a non-empty `blocking_probes` would meet the in-flight `hash-matches` on every milestone record commit | **not driven** | no document recommends such a hook |
| O-8 | L-6: the two other `DeclaredAbsolute` printed classes (`--explain`'s `Pack input:` line; the quoted failed `git` invocation) sit outside the preload's two rules by reading | **not driven** | outside the lead's surface |
| O-9 | L-22, cell 7: the `repo.head-detached` refusal from inside a detached foreign worktree says *"a commit made here"* although the boundary commit lands on the main checkout's `HEAD` | noted, **not driven further** | an inexact sentence, fail-closed |
| O-10 | L-3: the top-level-task arm of the base-pin refusal; L-11: the two-spelling question for other doctypes' header fields | **not driven** | bounds of those rows |
| O-11 | **The trailer's two unexercised paths**: a committing door run by a sub-agent; de-duplication by address | **not exercised by any session** | no sub-agent ran a committing door and no worker added a trailer item (§5.4) |
| O-12 | T-14's adjacent: `halted_awaiting_human` takes `permission_denials` from the last `result` event only; whether a later event can drop an earlier one's denials is not established | read, **not driven** | every event in this trial has zero denials |
| O-13 | T-7: whether a real run can produce an out-dir holding sub-agent transcripts and no main one; T-11: whether the repository's hygiene scan would match a home path | **not driven** | the first needs a container run; the second's denylist is private and was not read |

---

## 8 · What the workers said

Two debriefs, each one extra turn forked from the frozen conversation **after** scoring, into a
new out-dir, so the scored evidence was untouched. Both exited 0 with 0 denials and inherited the
seed. The prompt is [paste/debrief-prompt.txt](paste/debrief-prompt.txt); it asked the worker to
answer from memory and run nothing. **A worker's account is never trusted alone and never
ignored** — every claim below was driven, and the verdict is beside it. Arm (c) has no debrief.

### Arm (b)

| the worker said | lead | verdict |
|---|---|---|
| *"`jigc start "<intent>"` printed the exact same static catalog of 13 workflows regardless of what I put in the intent string"* | L-13 | **CONFIRMED**, no tier — designed: the agent picks, the CLI never does. The session ran the router once; the rig carried it across five intents |
| *"Those addresses don't match reality … If I'd pasted the workflow's own printed command verbatim it would have failed"* | L-11 | **PARTIAL** — the two spellings are real (tier 3, weak); *"would have failed"* is **refuted** by the same worker's own turn-2 log |
| *"I didn't re-add it … and the amended commit still carried that trailer. So that line of guidance doesn't match observed behavior"* | L-10 | **CONFIRMED**, tier 3 — the worker tested the sentence deliberately and was right |
| *"A way to preview the exact rendered commit message text before committing"* — not found | L-12 | **PARTIAL**, no tier — the header is forecast by `task finalize --dry-run`, which the session never ran; the trailer block has no pre-commit surface |
| the `git log -1 --format=%s` check *"was literally the command jigc's own `task amend` output told me to run"* | L-9 | **PARTIAL**, no tier — the span is bare, and the `-C` rule does not reach it |
| *"I never hand-edited a doc file or hand-ran `git commit`/`git commit --amend` myself"* | — | agrees with both scorers' read of every Bash command |

### Arm (a)

| the worker said | lead | verdict |
|---|---|---|
| *"The very first `jigc start --workflow planning` call failed because I didn't pass the intent … I'd assumed"* | L-2 | **CONFIRMED**, tier 3 — the worker blamed itself; orientation had printed exactly that line |
| the planning gate text *"cites specific-sounding incidents — "M42", "M44" … On first read these looked like they might be this project's own history"* | L-14 | **PARTIAL**, no tier — reproduced exactly; mandated by the design of record. The worker's inference was correct and it flagged it as an inference |
| *"two different persistence models under one CLI"* | L-16 | **CONFIRMED** as an observation — declared, and said by help, ack and design |
| *"Nothing in the first turn's planning-workflow text had mentioned worktrees at all"* | L-4 | **CONFIRMED**, tier 3 — on the resume line; the silence of the forward surfaces is the measured half |
| *"A milestone-level, jigc-native way to check whether several sub-tasks' independently-staged changes would actually merge and pass together"* — not found | L-17 | **PARTIAL**, no tier — true that nothing names one; there is none by design, and `milestone join` is not it |
| *"That's how I caught the one real cross-piece problem"* (the scratch worktree) | L-17 | **refuted by the transcript's order** — the break was found by reading the staged diffs and fixed before the scratch worktree existed |
| *"my first `write.wrong-shape` rejection (using `set-fields` instead of `set`) — I only read `jigc doc author --help` reactively"* | L-18 | **CONFIRMED**, tier 3 (weak) on the `location` half; the route half contradicts nothing |
| *"I did not read or edit any managed doc directly at any point"* | — | agrees with the read-back row (0 filesystem reads) and both scorers' transcript reads |

Asked what jigc told it that turned out wrong, arm (a)'s worker answered: *"Nothing I can point to
as definitively false."* The tiered rows that trace to that session came all the same — L-2, L-3,
L-4 and L-18 from what jigc printed to it, L-22 from its workaround — two of them (L-2, L-18)
through a refusal the worker took for its own mistake.

---

## 9 · Tooling findings — the trial's own apparatus

Kept apart from the product. These are defects and gaps in the trial harness, the driver, the
corpus template and **this trial's own protocol and runbook**. They carry no tier: the scale
grades the product. **None moved an arm's class or a trailer row** — each row says what it cost.

| id | verdict | what | what it cost this trial | where it lives · pin |
|---|---|---|---|---|
| **T-1** | CONFIRMED | `run.py observe` reports a correct `jigc task amend` as `HISTORY REWRITTEN … a commit produced or rewritten outside jigc`: the reflog half keys on the subject prefix `commit (amend)` alone and never consults the `task amend` record it holds | nothing — declared in protocol §8.3 and §0.3; the line is advisory. But the caption's attribution is wrong, and the reader cannot tell B-1 from B-3 on that line alone | `completions/trial-driver/driver/observe.py` · UNPINNED — no test names `amend` in either direction |
| **T-2** | PARTIAL | `observe` reads arm (a) turn 2 as `VOID unmeasured — no authoring occasion existed` | nothing — it is the registered read-back row for a turn with no doc write, not a mis-score. *"None by construction"* is false: the sub-task text offers an ADR | `driver/observe.py`, `driver/cascade.py` row 6 · the confirmed half is pinned; the fan-out shape is not |
| **T-3** | CONFIRMED | `observe`'s `git!` detector misses `git -C <path> commit` and `git -c k=v commit` | nothing — no arm typed a history verb in any raw-git shape, and no table row reads the channel | `driver/observe.py` → `commit_writes()` · UNPINNED |
| **T-4** | CONFIRMED | `tools/raw-git-acts.py` matches nine history verbs only; it printed 0 for arm (a), whose main agent ran `git worktree add`, `git apply` and `git worktree remove --force` | the *0* is true of what it claims; the scratch worktree was visible only by reading the transcript — which is how L-22 was raised | [tools/raw-git-acts.py](tools/raw-git-acts.py) · UNPINNED — the helper has no test |
| **T-5** | CONFIRMED | runbook §10 copied the walk record with a plain `cp`; the copy carried 5 host-path lines and the section's own check then said STOP | caught before commit. **Fixed in the committed [runbook.md](runbook.md)**: the walk record now goes through the same `$HOME` → `~` rewrite as the provenance files, and the committed copy is rewritten | `runbook.md` §10 · UNPINNED |
| **T-6** | CONFIRMED | `test_session.py` defines `CarryLeavesTheWalksOwnOutputBehind` **after** its `unittest.main()` entry point, so its one test never runs in the script form — the form `run.py test`, the only documented runner, uses | none on the evidence; the unreached test passes when collected. The script form runs 22 of the file's 23 tests, and reports `OK` | `completions/trial-driver/test_session.py` · UNPINNED |
| **T-7** | PARTIAL | `run.py fork` and `run.py observe` take a sub-agent's transcript as the main session when no main file exists — voided as *"began cold"* on the fork door unless the sub-agent quotes the seed, **scored outright** on the observe door | none — every RC24 out-dir has a main transcript first; the shape was constructed, not observed | `run.py`, `driver/session.py`, `driver/observe.py` · UNPINNED |
| **T-8** | CONFIRMED | `observe`'s `recs` column counts adapter-hook invocations as session records: arm (c)'s 13 = 11 worker-typed + the SessionStart hook's `start` + the pre-commit hook's `validate` | no scored channel moves — the over-count is confined to `recs` and to the one cascade row that reads it | `driver/observe.py` · UNPINNED |
| **T-9** | CONFIRMED | `tools/trailer-rows.py` pairs same-second doors and commits by a tie-break (commit depth + log-file order), not by time | arm (a)'s three `add-task` rows are right by that rule; both scorers paired them by subject and ack hash instead. It mis-pairs silently when log order stops being causal order | [tools/trailer-rows.py](tools/trailer-rows.py) · UNPINNED |
| **T-10** | CONFIRMED | `trailer-rows.py` cannot see a commit made on a fan-out worktree's detached `HEAD` once the worktree is removed | none — in all five scored out-dirs `rev-list --all --reflog` and the object database agree, and there is no unreachable commit. The by-hand check used the wider enumeration | `tools/trailer-rows.py`; protocol §6.1 item 2 · UNPINNED |
| **T-11** | PARTIAL | the operator's saved rubric files and logs carry host paths with a login name — 25 files, not the 4 the lead named | none in this record: **they are not committed** (§12), and the runbook never scheduled them. Every quote taken from them was rewritten | runbook §7; `run-session.sh`; `run.py`; `walk.py` · UNPINNED |
| **T-12** | CONFIRMED | arm (c)'s class table has no row for *answered the hand-off from the code, kept the prose's framing, flagged nothing*; the standing wart states nothing the code contradicts, only a role it does not implement, so the evidence cannot tell *missed* from *saw no disagreement to report* | **the one tooling row that bears on a class's meaning.** The reading does not move (C-2 and C-5 both read *not reached*); what C-5's label may be said to mean does. Two blind RC-pre-1.0 sessions read the same sites as a contradiction: the wart is ambiguous, not null | protocol §5.1, §5.3; `completions/trial-corpus-template/README.md` · UNPINNED |
| **T-13** | PARTIAL | runbook §7.3 runs the session's test suite **inside** the evidence out-dir, against operational rule 9 — and it wrote nothing. The real effect came from the operator's four *extra* `git status --porcelain` runs, which rewrote `.git/index` (stat cache only) in three scored out-dirs | no figure moved: every ref, reflog, object and tracked file is unchanged (§11) | runbook §7.3; the operator's rubric script · UNPINNED |
| **T-14** | CONFIRMED | the runbook's `result` helper prints every `result` event of a stream, oldest first and unlabelled; arm (a) turn 2 holds three, and the first — *"I'll wait … I'll let you know"* — is field-for-field a normal turn end | no class was assigned from it; a reader who took the first as final would read A-2 against a log that says A-1 | runbook §4 → `result()` · UNPINNED |
| **T-15** | PARTIAL | arm (a)'s three background sub-agents all started in the main agent's shell directory — a sibling's worktree for two of them; the isolation came from the main agent's prompts relaying a `cd` that jigc's resume refusal had printed | A-1 does not move: every edit of each sub-agent is under its own worktree. But a sub-agent transcript's start directory is not a *who worked where* signal, and a less careful sub-agent would have written one piece into another's worktree | the harness's pinned CLI (`2.1.233`); protocol §3.3 · UNPINNED |

### The `seed` repair this trial made

`run.py seed` could not carry a conversation across turns when the trial was drafted. Commit
**`bffa6667`** — *fix(trial-driver): seed takes the main transcript out of the list the finder
returns* — repaired it before any session ran: `_find_transcript` had returned a list since an
earlier change and `seed` still treated it as one optional path, so a multi-turn seed died at turn
2, a one-turn seed died at the freeze, and a turn that lost its transcript was not detected.
`seed` now takes the first element that is not under `subagents/` and refuses when there is none.
Three tests drive the real turn loop over a stubbed driver; all three were red before the change.
The seed smoke (§3.2) is the live proof, and arms (a) and (b) are the two conversations it then
carried.

**The two things the repair's fixer saw and left**, as outside the repair's scope — both now rows
above, both verified:

- **T-6** — the test class defined after the entry point in the very file the repair added its
  tests to;
- **T-7** — the edge the repair fenced on `seed` and did not fence on `fork` (or on `observe`):
  nothing guarantees position 0 of the transcript list is the main session.

---

## 10 · The headless-channel bounds, in full

From [protocol.md](protocol.md) §11 and §12. **Read these before believing a row.**

**A headless `-p` session is a different channel from an interactive one.** A headless result is a
fact about the headless channel. The driver's increment 0 measured one thing across the two —
that the read-back channel does not invert — at n = 1 per arm, one corpus shape, one model alias,
one day, one CLI build. It licenses *"the channel survives the transport"* and nothing
quantitative, and it says in its own words that nothing there makes a headless session a valid
substitute for a blind interactive one in a scored trial.

**There is no mid-turn injection.** `-p` runs the arc to completion with no queued-message
channel. Nothing can be said to a worker while it works. Automation buys the driving, never the
interrupting. So this trial scheduled nothing: arm (c) used a standing state, and arms (a) and (b)
delivered their second sentence at a turn boundary — the one moment a headless conversation can
receive one.

**A headless question ends the turn.** Nobody answers. In arms (a) and (b) a question in turn 1
would have been answered only by the pre-written turn 2, whatever was asked; a question in the
last turn is never answered and is scored as a stop. No answer key was loaded. **This is why arm
(c) cannot take a disagreement to the human**, as the two interactive RC-pre-1.0 sessions did.

**A halt exits 0.** A turn that ends asking for something reports `subtype: success`,
`is_error: false`, empty stderr. Only `permission_denials` and the text of `result` say it
stopped. Read the row, not the exit code. (Here every turn's denials are zero and none ended
asking.) *Ended asking* is itself a heuristic — it keys on a question mark anywhere in the final
message.

**The read-back channel counts are per turn on a seeded arm.** `run.py seed` leaves one out-dir
per turn. The log is cumulative, and `observe`'s log columns are per turn: turn 2's *"N record(s)
predate this session"* is counting **turn 1's** records, which are the worker's; the
conversation's figure is turn 1's row plus turn 2's. The transcript and the reflog are cumulative
and are **not** split by turn, so turn 2's `fs?`, `git!` and `HISTORY REWRITTEN` lines repeat turn
1's and are read from the last turn only, never summed. A turn that did little reads as void —
that is the split, not the apparatus (T-2). Sub-agent transcripts do not travel between turns:
each turn's are read where they are.

**FILESYSTEM is a heuristic; VERB and VERB-ADJACENT are exact.** The transcript is the CLI's
format and changes between versions. Registration state is not in it, so a read of a file at a
managed home that was never adopted matches and should not score. It returns evidence for review,
never a number. The write channel is likewise evidence, never a verdict: git is a blessed human
channel.

**`bypassPermissions` is a directional confound.** It removes prompt friction from file reads and
writes while `jigc setup` allowlists `Bash(jigc:*)` either way — one side of the asymmetry the
adapter bets on. It makes going **around** jigc easier. So a *reached* is not weakened by it —
arms (a) and (b) took the door against the cheaper path — and a *not reached* is partly
attributable to it. **No arm ran the adopter's real permission condition.** `--strict-permissions`
headless is usable and was not used: arms (a) and (b) need source edits a strict turn is denied
(and `seed` has no strict door), and a strict arm (c) would have denied an edit to `README.md`,
removing *fixed silently* from the outcomes a worker could reach.

**n = 1 per arm.** One worker moves every reading. A *reached* shows the path can be walked blind,
once. A *not reached* shows where one worker stopped, once. Neither is a rate. No arm was run
again because of where it landed; a second draw taken after seeing the first is a post-hoc
widening.

**Arms (a) and (b) asked for the outcome in product language, strongly enough that the designed
door was the natural route.** Turn 1 of (a) handed over the cut and the sign-off and said *"record
this as the project's next milestone"*; turn 2 said *"start all three pieces together — at the
same time, each one kept apart from the other two — and … land them together as the one
milestone"*. Turn 2 of (b) said *"put the right ticket on that same commit, so the history holds
one commit for this fix"* and *"nothing has been pushed"*. **They therefore measure whether a
blind agent can DRIVE the door when asked for that outcome — not whether it chooses the door
unprompted.** No prompt names a jigc verb, workflow, doctype or flag, or says *fan out*, *amend*
or *report an inconsistency*; but *milestone* is also a jigc verb, *finalize* sits in two prompts,
and every prompt carries the one channel statement — *"the project's docs are managed with `jigc`,
so the thinking goes in through it"* — a declared bound: a positive result is compliance plus an
operator channel preference, never unprompted tool preference. **Arm (c) alone was unprompted**:
its prompt does not say that anything disagrees, and it is the arm that was not reached.

**The worker model is `claude-sonnet-5`**, on the Claude CLI the image pins (`2.1.233`). One
model, one build, one day. Nothing here speaks for another model, another assistant or a later
CLI.

**The rest of protocol §11, unchanged:**

- **Every turn is a fresh container.** A seeded conversation exists because the main transcript is
  copied out of one container and into the next. `--resume` on a session the CLI has never seen
  does not fail — it starts a fresh conversation; `seed` refuses loudly at the turn that lost the
  transcript and verifies the frozen one against turn 1's text.
- **A resumed turn re-enters the adapter.** The SessionStart hook fires again and the project's
  instructions load again, so turn 2 of (a) and (b) began with a fresh orientation over the state
  turn 1 left — which arm (a)'s design relied on, and which is where its worker met the resume
  line (L-4).
- **The seed is driven once.** One conversation per arm, no repetition.
- **What a fork drops.** Session-scoped permission grants, and `--plugin-dir`, `--settings`,
  `--mcp-config` and `--add-dir` are not restored on resume. No arm forked; the two debriefs did,
  and under `bypassPermissions` there were no grants to lose.
- **The network is not isolated**, and the token is in the container's config for the life of a
  session.
- **Isolation costs comparability** with every trial before RC-1.0-gate, which ran unisolated.
- **`verify-image.sh` check 2 proves the probe loads, not that it parses.** Arm (c)'s architecture
  document is the only arm that cited code; it finalized with eight anchors.
- **The driver does not grade findings**, and a green driver suite is necessary and not
  sufficient: every defect its directory has had was found by running it (§9 adds fifteen rows).

**What this trial does NOT measure** (protocol §12):

- **Reliability.** Three arms at n = 1.
- **The interactive transport**, and the adopter's real permission condition.
- **Regressions against rc.14.** No arm carries a baseline, no pair differential exists for a
  registry image, and **walk arms 01–23 were not run**. The partial re-review over M54's and M55's
  axes is the defect instrument, and it is a separate one.
- **The duress read-back** — no plant, and so no comparison with RC-rc14's 3/3.
- **The genuine concurrent spawn.** A-1 with sub-agents was observed, not controlled — and here it
  never touched `milestone execute`, a `Spawn:` line or `workflow sub-task`.
- **`finalize.fan-out.squash: false`**, the per-sub-task commit chain, and the fan-out Fix phase.
- **`report-jigc-feedback` and the two triage workflows** — hidden by design, not expected blind.
- **Filing through the findings channel at all.** Arm (c) was not reached, so nothing in this
  trial drove `report-inconsistency` blind: the doc-only finalize, the create-only gate and filing
  beside an open task were met by no session.
- **An amend of pushed history, of a milestone boundary, or of a commit jigc did not make.**
- **`jigc setup` under the agent**, and so the trailer on the install commit.
- **The trailer under another assistant, on a command a human types into an agent's session, on a
  door a sub-agent runs, or through the de-duplication seam.**
- **Upgrade and migration.** Every corpus was adopted fresh on rc.24.
- **Multi-process concurrency semantics, and long-horizon drift.**
- **Whether the isolated environment is representative of an adopter's** — right for attributing,
  wrong for predicting a day.

---

## 11 · Honest bounds, and what is owed

**Bounds on this record, beyond §10:**

1. **The tier-1 row was not observed; it was derived.** No session ran a jigc command over a
   foreign worktree. L-22 exists because one worker's workaround raised a question and a verifier
   drove it on rigs. Its precondition did not occur in the trial, and its most plausible everyday
   trigger (O-1) was not driven.
   **[Corrected 2026-10-04 by the adversarial re-drive:** O-1 **was driven** after this bound was
   written — a live host worktree, never moved or deleted, went from `git status` exit 0 to
   `fatal: not a git repository`, exit 128, once `jigc milestone provision` ran (exit 0) in a
   container that saw the repository and not the worktree's path
   ([tier1-verification-L-22.md](tier1-verification-L-22.md) → 6). The rest of the bound stands as
   written: the row was derived, not observed, and it did not occur in the trial.**]**
2. **Two forks were not run as their recommendations read**, and [forks.md](forks.md) says so
   under each. **F4** recommended rehearsing arm (b)'s turn 1 on a spare corpus: no rehearsal
   exists — no out-dir, no corpus, no log step. The scored turn 1 left the occasion the rehearsal
   would have paid for, so the assumption held; it was not paid for in advance. **F5** recommended
   debriefing all three arms, with (c) run through `seed` if the seed smoke passed: the smoke
   passed and (c) ran through `run-session.sh`, so **arm (c) has no debrief** — which is exactly
   where T-12 needs one. The orchestrator's statement is that every fork was taken as recommended;
   the out-dirs and the operator's logs are what this record reads.
3. **Thinking blocks are empty in every saved transcript.** *What the worker noticed* is read from
   its commands, its prose and its docs only. That is the whole basis of C-5.
4. **Things were written under `~/out` after the sessions ended.** None changed a ref, a reflog,
   an object, a tracked file or a figure in this record:
   - the operator's four extra `git status --porcelain` runs rewrote `.git/index` — stat cache
     only — in three scored out-dirs (T-13);
   - the first scorer's `git fsck --lost-found` in arm (b)'s turn-2 out-dir wrote one 41-byte
     file, `.git/lost-found/commit/7757a4c…`, by mistake, and left it for the human rather than
     delete under `~/out`;
   - the runbook's own `node --test` and the second scorer's ran inside arm (a)'s out-dir and
     wrote nothing.
   Operational rule 9 — *reproduce in a copy, never in the out-dir* — was not held by the
   runbook, the operator or the first scorer.
5. **The ledger is open.** Every row in [findings-verification.md](findings-verification.md)
   carries a pin or a stated `UNPINNED`, written by its verifier from tests it **read and did not
   run**. No row-by-row re-read has been done at a close, and RC-rc14's closure check is not
   claimed over this file.
6. **Verification ran on the host's installed registry build**, not in the trial's container: the
   same published version (`jigc --version` asserted first in every row), release posture, macOS.
   The sessions ran the Linux image. No row was found to depend on the platform; none was tested
   for it.
7. **The scorers were independent of each other, not of the evidence's author.** The second scorer
   did not read the first's file. Both read the same out-dirs, and the leads were written by the
   first scorer alone; the second contributed four candidate observations, all of which the lead
   list already held.
8. **Nothing is filed.** This repository has no store; the rows live here until the port (M56)
   files them beside M55's seed.

**What is owed:**

- **The human's ruling on L-22** — tier 1 by the predicate, or weighed down to 3 — and, if it
  stands, the fix pass the exit rule attaches. O-1 is the cheapest datum that would inform it.
- **The 1.0.0 call**, which is the human's and which this record does not take.
- **The partial re-review** over M54's S18 axes and M55's S16 axes — the defect instrument, a
  separate one, also owed before the call.
- **At the port:** the eight tiered rows and the fourteen untiered ones filed through jigc, each
  already in the channel's shape; O-3 and O-4 triaged as leads of their own.
- **The pointer from `implementation/decisions-pending.md`**'s blind-trial entry to
  [protocol.md](protocol.md) §0 as the home of the M51–M55 declared-changes list (forks.md → F9).
  It belongs to the commit that lands this directory; this record wrote nowhere outside it.
- **A `work/` change for the driver**, after the record, as forks.md → F10 and F12 settled:
  `raw-git-acts.py`'s pattern into `driver/observe.py` with a test over the five shapes (T-3,
  T-4); `observe`'s `commit (amend)` line told apart by the `task amend` record (T-1);
  `trailer-rows.py` into `completions/trial-driver/` with a tie-break that is not log order and an
  enumeration that sees unreachable commits (T-9, T-10); the main-transcript predicate on `fork`
  and `observe` (T-7); the entry-point position in `test_session.py` (T-6); the hook-origin
  records in `recs` (T-8).
- **A class table that can hold arm (c)'s behaviour**, and a disagreement instrument whose prose
  states something the code *contradicts* rather than something it does not implement (T-12),
  before the findings channel is put to a blind worker again.
- **The correction to protocol §0.1's refusal sentence** (§6), carried into the next protocol's
  declared-changes list.

---

## 12 · Files

| file | what it is |
|---|---|
| [README.md](README.md) | this record and verdict |
| [findings-verification.md](findings-verification.md) | all 39 leads, each driven, with its full repro block, contract, tier and pin — the verifiers' files concatenated in id order |
| [tier1-verification-L-22.md](tier1-verification-L-22.md) | **added after assembly (2026-10-04)** — the independent adversarial re-drive of L-22, verbatim with one added header line: three doors re-driven on fresh rigs with their controls, what is lost per plant, the gc cell, the container and plain-`mv` triggers, whether it is declared, the tier argued both ways, every prune site, the fix's *cheap vs robust* fork and the pin. It upholds tier 1 |
| [protocol.md](protocol.md) | the pre-registered protocol, **verbatim as registered** — the arms, the outcome classes fixed before the run (§3.3 / §4.3 / §5.3), the trailer check (§6), the declared changes (§0), the bounds (§11, §12). It says *draft* and links `open-forks.md`: that file is [forks.md](forks.md) here |
| [forks.md](forks.md) | `open-forks.md` as drafted, with one `Taken:` line under each fork — and, for F4 and F5, what was actually run |
| [runbook.md](runbook.md) | the operator's ordered steps. Two edits against the copy that was run: the opening sentence now points at forks.md, and §10 copies the walk record through the `$HOME` → `~` rewrite (T-5) |
| [corpora.md](corpora.md) | the four corpora and how each was built, verbatim |
| [paste/](paste/) | the exact turn files — `a-turns.txt`, `b-turns.txt`, `c-prompt.txt`, the debrief prompt, the environment probe, the seed smoke — and their README, verbatim |
| [tools/](tools/) | `trailer-rows.py` and `raw-git-acts.py`, the two by-hand helpers, verbatim. **They stay here and were not moved into the driver** (forks.md → F10; T-4, T-9, T-10 are about them) |
| [gate-rc24.json](gate-rc24.json) | the isolation record `run.py gate` checks an image against |
| [evidence/](evidence/) | per arm: `invocations.jsonl`, the `PROVENANCE` file of each turn, `trailer-rows.txt`; and the walk record. [evidence/README.md](evidence/README.md) says what is and is not committed |

**Not committed, by rule** ([implementation/public-hygiene.md](../../../implementation/public-hygiene.md)):
no transcript, no `stream.jsonl`, no corpus, no debrief's raw text, none of the operator's rubric
files or logs, and neither scorer's file. They stay on the machine that ran the trial; this record
quotes from them in short excerpts. The session out-dirs are `~/out/RC24-C`,
`~/out/RC24-B-frozen-work/turn01` and `turn02`, `~/out/RC24-A-frozen-work/turn01` and `turn02`,
`~/out/RC24-B-debrief`, `~/out/RC24-A-debrief`, `~/out/RC24-env` and `~/out/RC24-walk`.

**Then the 1.0.0 call, which is the human's.**
