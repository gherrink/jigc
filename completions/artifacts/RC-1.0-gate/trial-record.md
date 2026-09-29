# RC trial — the 1.0.0-gate trial on rc.11 (2026-08-16/18)

**Status: RC input — the last instrument before the 1.0.0 call, and the only one with nowhere to
send its findings.** M46 is not scheduled before the call, so this trial's decision rule was fixed in
advance rather than adjudicated after the result: six finding classes, each with a pre-registered
consequence, and *"judged not to matter"* barred as a disposition
([protocol.md](protocol.md) §1). Design, measurement and declared bounds in
[protocol.md](protocol.md); the contemporaneous findings record in
[session-findings.md](session-findings.md); every operator utterance into a blind session in
[operator-log.md](operator-log.md); the operator walk in [v1-walk.md](v1-walk.md); what the
preparation itself turned up in [pre-trial-findings.md](pre-trial-findings.md); verbatim worker
feedback in `feedback-B1.md` … `feedback-B3b.md`; corpora, cue cards, prompts and answer key in
[corpora.md](corpora.md) / [cue-cards.md](cue-cards.md) / [blind-prompts.md](blind-prompts.md) /
[answer-key.md](answer-key.md).

## Provenance

- **Binary:** `jigc 1.0.0-rc.11` throughout — `binary_version` on every one of the 331 logged
  records, `jigc-sha d1ebbc2` in every corpus's `PROVENANCE.txt`. Walk arm 2 additionally drove
  **rc.10 built from the pinned `1d4f9bc`**, the genuine pre-M48 tree, confirmed by
  `verify-pair.sh` before the arm ran.
- **Isolation:** every session ran in the container rig, not on the operator's machine — a change to
  a pre-registered instrument, made **before any session ran**, because the host contaminates every
  worker with the operator's global `PRINCIPLES.md`/`LACON.md` (§2, with the measurement that proved
  it). `model claude-sonnet-5`, `permissions bypassPermissions` in all four blind sessions.
- **Corpora:** five, all instantiated fresh from the committed
  [trial-corpus-template](../../trial-corpus-template/) with `--clean-prose`, each gated **11/11** by
  `check-corpus.sh` in its naive state ([corpora.md](corpora.md)). Blind: `harborlight` (B1) ·
  `pinegrove` (B2) · `stonefly` (B3a) · `rosewater` (B3b, the re-run). Walk: `rc11-control` (arm 0) ·
  `rc11-walk` (arms 1–4). B2/B3 were adopted **by the container's own binary** before their sessions,
  per §4's preload note; B1 installs jigc in-session by design.
- **Invocation log:** ON in all of them. **B1 26 · B2 102 · B3a 94 · B3b 99 · control 10 = 331**, of
  which 3 are the operator's pre-session adoption `validate` on the adopted corpora — **318 worker
  invocations**.
- **Method:** the operator pasted verbatim prompts from `paste/` into fresh sessions and answered
  only from [answer-key.md](answer-key.md); the observer built corpora and plants, ran the walk, and
  never edited a blind corpus once its session started. No mid-trial fixes to the binary.

## What was run

| Instrument | Corpus | Outcome |
|---|---|---|
| **Arm 0** positive control | `rc11-control` | **PASS in the rig** — the VERB channel fires, greps, and survives copy-out ([pre-trial-findings.md](pre-trial-findings.md)) |
| **B1** cold start, setup in-session, two plants | `harborlight` | ADR landed through a rejecting `core.hooksPath` hook; 10 commits, 1 managed doc. **Carryover plant defused by the worker, not tested** |
| **B2** design altitude from zero | `pinegrove` | research → vision → roadmap → milestone → 2 provisioned sub-tasks → join → milestone finalize; 16 commits, 8 managed docs incl. `VISION.md` and a milestone record |
| **B3a** the corpus accretes | `stonefly` | changelog → spec → implement-from-spec → single-task → arch-doc; 14 commits, 4 managed docs. **Both of its plant-dependent arms VOID** — the plant never fired |
| **B3b** the B3 re-run | `rosewater` | same arc, plant working: 17 commits, 7 managed docs. **Both plant arms caught** |
| **Walk** arms 1–4 | `rc11-walk` | 4/4 pass — the declared breaking change, the rc.10→rc.11 upgrade, the guide clobber refusal, the unreachable verbs |

**B3 ran twice, and the reason is an apparatus failure, not a product result.** The B3a watcher's
*timing* was right — it caught the worker's first `finalize` and fired 2 min 11 s later — but the
invocation died on `chmod: changing permissions of '/tmp/plant.sh': Operation not permitted`
(`docker cp` lands root-owned; `docker exec -u node` cannot chmod it). The corpus was checked
afterwards and carried **no** planted ADR, so nothing partial was left behind. Two arms therefore
never ran in B3a — detect-and-route and the contradiction trap — and neither may be reported as run.
The fix (`bash /tmp/plant.sh`, no chmod) was proven end-to-end on a throwaway corpus before B3b, and
B3b's plant landed 101 seconds after that session's first finalize, back-dated, explicit pathspec,
sweeping nothing else.

## The headline measurement — instructed read-back compliance

**4/4 VERB, none of them prompted.** Every blind session read its own staged work back through
`jigc doc show <addr> --task <id>`, the verb M48's pack-load fence makes an authoring step name.

| session | records | `doc show … --task` | `doc show` any | VERB-ADJACENT (`task diff` / `doc list --task`) | `task validate` | outcome |
|---|---|---|---|---|---|---|
| **B1** `harborlight` | 26 | **3** | 3 | 0 | 2 | **VERB** |
| **B2** `pinegrove` | 102 | **4** | 5 | 2 | 7 | **VERB** |
| **B3a** `stonefly` | 94 | **6** | 6 | 0 | 5 | **VERB** |
| **B3b** `rosewater` | 99 | **7** | 11 | 0 | 6 | **VERB** |

> **Correction, from the logs rather than from the running note.** [session-findings.md](session-findings.md)
> records B1 as `5 / 5 / 0 / 1`. Recounted from `~/ideas/harborlight-out/.jigc/logs/invocations.jsonl`
> — 26 records, whole file — B1 ran `doc show … --task` at records **8, 14 and 18** and `task validate`
> at **19 and 26**: **3 / 3 / 0 / 2**. The operator-log snapshot taken live at intervention 1 (21
> records, 3 reads) agrees with the log, so the contemporaneous table is the outlier. **B1's verdict
> is unchanged — VERB** — and B2/B3a/B3b's figures reproduce exactly. §3.3 counts `task validate` as
> VERB-ADJACENT too; it is broken out here because the running record broke it out.

**Read against §3.4, over the any-point window — the one that carries the fence's claim, because the
fence makes the *authoring* step name the verb.** The pre-registered rows for *0 VERB* (the seventh
landing, escalate VISION principle #3) and *0 VERB with a dead control* (apparatus failure) are both
unreached. The control fired first, in the rig, so a null would have been readable had one occurred.

**What that does and does not license, stated in the same breath.** §3.4 names the construct
honestly: the composed `record-decision` step prints, verbatim and copy-pasteable, *"Read your write
back before you move on — with `--task` the read serves THIS task's staged copy"*. So this is
**compliance with a named instruction at N=4**, not discovery of an unnamed capability, and not
reliability. A failure here would also have been a **worse** result than the six prior landings: ignoring a
printed, copy-pasteable command is a stronger negative than failing to find something nobody
mentioned. The historical discoverability series stays a separate, explicitly non-comparable line.

**Two observations that sharpen attribution, both from the archived transcripts:**

- **No session read a staged doc from the filesystem.** Across all four transcripts, **zero** tool
  calls of any kind touch `.jigc/tasks/**/docs` — the act by which the discoverability lens was
  measured in the prior trials, and the one RC-pre-1.0 recorded in all three of its sessions. B1
  listed `.jigc/tasks/` (`find`, `ls`) without reading a doc; B2 read **code** under
  `.jigc/worktrees/<id>/src` (an ordinary working tree, not a managed doc) and probed
  `.gitignore`/`git check-ignore`; B3b read the **unregistered** foreign ADR before adopting it,
  which `.jigc/AGENT.md` explicitly permits. The FILESYSTEM channel did not fire on a managed doc in
  any session.
- **The session that could not have had the adapter still used the verb.** B1 ran `jigc setup`
  in-session, so `CLAUDE.md`/`.jigc/AGENT.md` did not exist when its system prompt was assembled —
  §4 discounts its measurement for exactly that reason. It read back three times anyway. Since the
  shipped guide (`.claude/skills/jigc/SKILL.md`) names **no** read surface at all across its 250
  lines, the only channel left is the pack step text, which is M48's fence. The discount cuts toward
  the fence, not away from it.

**Guide-artifact use, per §4 — recorded as use, not as discovery** (the harness advertises
`.claude/skills/`, so the worker is *told* it exists): the `jigc` skill was invoked **exactly once in
each of B2, B3a and B3b** (`Skill{"skill":"jigc"}`) and **never in B1**, which is consistent —
`setup` had not yet written it when that session started.

## Honesty statement — written from the operator log and the transcripts

**What was operator-touched.**

- **B1 · two interventions.** (1) The docs sign-off, logged verbatim with timestamp
  `2026-08-17T05:46:11Z`: *"Signed off — the ADR is what I wanted recorded. I've cleared the gate my
  end; go again."* — an in-fiction product decision the planted hook requires a human to make, with
  the marker released **before** the reply so the worker would not meet the same refusal twice.
  (2) **A second answer the operator log does not carry**, recovered verbatim from the transcript:
  *"Yes, split it. I want that commit to be the ADR and nothing else — leave my two files staged the
  way they were."* — consenting to the worker's own proposal to `git reset --soft` and re-split the
  commit that had swept the two planted paths. It is clean against §3.2's pre-send checklist (no
  *verify / check / read / look / confirm*, no jigc verb), so the measurement is not contaminated —
  but §8 rule 2 requires **every** utterance logged, and this one was not. Recorded here rather than
  quietly absorbed, and it changes how B1-3 reads: the raw-git surgery was **operator-approved**, not
  a unilateral bypass.
- **B2, B3a, B3b · zero interventions.** Verified from the transcripts, not from recall: each
  session carries exactly two operator utterances — the verbatim task prompt and the verbatim
  feedback prompt — and nothing else.

**What was contaminated.** Nothing, under §3.2's rule. No operator utterance in any session named
reading, checking, verifying, or any jigc read verb. The declared, pre-registered confounds stand
unchanged: `bypassPermissions` in all four sessions (§9, directional — it makes the FILESYSTEM
outcome *cheaper*, so it does not weaken a VERB result), and the corpus being the observer's own.

**What was void.**

- **The cue card never fired — in any of the four sessions.** The designed-need correction (§3.2),
  the trial's own instrument for manufacturing a read-back occasion, was never delivered: every
  worker ran its arc to completion with no pause an operator could inject into. The mechanism was
  pre-registered as the design's largest untested assumption ([cue-cards.md](cue-cards.md) §0.3,
  timing *"reasoned from how Claude Code delivers queued messages, not measured"*), and it is now
  **falsified four times**. Consequence, stated plainly: **the designed-need correction path is
  UNTESTED, not passed.** The F2 reject-and-retitle path it was built to drive was never exercised by
  a blind session, and the 4/4 result rests entirely on the any-point window — read-backs the workers
  took at authoring time, before any correction existed. The record must not present the cue card as
  exercised.
- **B3a's two plant-dependent arms.** The plant never landed (above), so detect-and-route and the
  contradiction trap were **void, not failed**, and B3a's zero `ingest`/`migrate`/`migrate-corpus`
  calls mean only that it met nothing foreign. They were re-run as B3b.
- **B1's carryover plant was defused, not tested.** The worker unstaged both planted paths in its
  first minute — *before* minting — so the gate's snapshot was legitimately empty and it had nothing
  to refuse. The gate itself is sound: three probes on rc.11 (staged→finalize; staged→unstaged→
  re-staged→finalize; and the same through a hook rejection) all raise `finalize.carried-staged` per
  path at exit 3, and identically on rc.10, so this is neither a gate failure nor a regression. The
  first reading of this result was **wrong and is corrected in the operator log itself** — `0
  carried-staged` was read as the gate failing on the case it was built for; the transcript shows the
  case never reached it.

## The results that hold

- **4/4 VERB, unprompted**, with the control fired first and the filesystem channel silent on managed
  docs (above). At N=4 this is compliance, not reliability.
- **Zero data-loss and zero corruption defects on any path driven** — blind or scripted. Nothing was
  destroyed unrecoverably, and no managed doc was silently written wrong. B1's swept commit was
  recovered cleanly with `git reset --soft`; B2's `git reset --hard` on a sub-task worktree was the
  worker's deliberate consolidation, taken after it verified the blast radius.
- **Zero regressions.** The only arm with a baseline is walk arm 2, and it found none: a corpus
  authored end-to-end on rc.10 (adr · root `CHANGELOG.md` · milestone-record, 12 commits) continues
  on rc.11 — `validate` advises the version mismatch, `setup` installs the guide artifact onto a
  corpus that predates it, `doc show` reads what rc.10 wrote, `migrate-corpus` correctly reports *0
  migrated, 3 already current, 0 blocked* rather than inventing work, and a task finalizes clean.
- **Both B3b plant arms caught.** *Detect-and-route*: the worker's own `validate` surfaced
  `schema-conformance.unadopted-instance`, and it followed the route to `ingest` → `migrate … --as
  adr` → an adoption commit, with **`migrate-corpus` invocations: 0**. M42's managed-vs-foreign
  discriminator did exactly the job it shipped for. *The contradiction trap*: the worker found the
  planted decision forbidding what it had been asked to build and reached
  `doc set-field adr:persist-admitted-samples#status/supersedes` unhinted — **the harder branch of
  the pass condition**, on a decision it disagreed with.
- **The migration review hold fired on a blind path and its route was followed.** B3b's adoption
  finalize exited **4** (`migrate.review-pending`), the worker re-validated, then ran
  `task finalize … --approve` at exit 0. Review-before-destroy, live, unassisted.
- **The declared breaking change reads as protection, judged against the standard §5 arm 1 fixed in
  advance.** All four criteria hold at all three doors (`milestone provision` ·
  `milestone discard` · `uninstall`): each names the exact path, enumerates what it holds, explains
  *why* it cannot decide the bytes are disposable, and names `--force` in the same output — which,
  run verbatim, works. The idempotent case still works (an empty directory at a real sub-task path is
  reused, exit 0). This is the one deliberate regression an adopter meets, and it does not read as
  obstruction.
- **Every refusal encountered on the walk named its path, its reason and a route, and every route
  followed verbatim worked** — including `rename`'s three-attempt chain (in-flight task → in-flight
  milestone → success), which is a route-followability pass, not a papercut.
- **The guide artifact behaves on both branches**: an edited guide earns `adapter-guide.user-modified`
  and is left untouched (not a silent clobber, not a blocked setup); `uninstall` removes it when the
  bytes are jigc's and **keeps** it when they are the user's.
- **The read-shaped near-miss answers with a read verb.** `jigc doc read` is met by M48's read-intent
  rule naming `doc list` / `doc show <address>` (with `--task`), and clap's contradicting suggestion
  is gone.
- **Exit-code census: 17 non-zero across 331 records, every one a designed refusal or gate** — the
  planted hook rejection, `create.gate-blocked`, `store.not-staged`, a blocking `task validate`, the
  exit-4 review hold, a mis-guessed task id. **No panic anywhere.**

## The findings

Grouped and ranked; the evidence and the worker's own words are in
[session-findings.md](session-findings.md), which is not restated here. Classification against §1 is
recorded there where it was decided from evidence at the time; the rest is owed to
`findings-verification.md` with live repro blocks (§7).

**Carried in from the preparation, and still the trial's only blocking-class candidate**

- **PT-1 · `migrate-corpus` claims a never-adopted foreign file and prints a route that cannot run**
  ([pre-trial-findings.md](pre-trial-findings.md)) — provisionally **blocking dead end
  (project-carrying reach)**, because the defect fires at any managed doctype's *home* and two of
  those homes are repo-root files (`CHANGELOG.md`, `VISION.md`) plus decision history. **Its owed
  recheck has landed, with evidence rather than inference:** a blind worker met exactly the file that
  triggers it, ran **zero** `migrate-corpus` invocations, and was steered correctly by `validate`'s
  `unadopted-instance` advisory. So reach is unchanged (demonstrated) and likelihood is now measured
  — **0 of 4 blind sessions reached it**. The disposition remains the human's under §1.

**Wrong result on a non-destructive path**

1. **F-1 · `changelog-recording.gate-granted-unused` fires on a task that DID record a changelog
   entry** (VERIFIED, reproduced on rc.11). The gate tracker counts only `add-item`, so a `set-slot`
   edit of an existing category does not register — and the advisory asserts nothing was recorded
   **in the same output that prints `promoted CHANGELOG.md`**. Its route tells the worker to do what
   it has already done, so a worker that trusts it duplicates user-facing history. M43 law 1 is
   *nothing lies*; this is a statement contradicted by the adjacent line. Classed in the running
   record as **SHIPS RECORDED**, at the top of that list.

**Surface findings — a lie, an ambush, or a missing route on a non-blocking path**

2. **B2-1 · The pack instructs an action its own create gate refuses.** `sub-task-commit.yaml`
   includes `step:author-commit` while `sub-task.yaml`'s `allows-create` lists only `adr`, so the
   printed step earns `create.gate-blocked`. VERIFIED at the source; the pure case of a law-1 lie,
   and reversible.
3. **B1-1 · "staged" means two different things**, and M47's survivable hook-rejection frame borrows
   git's word (*"your staged changes are still staged"*) at the exact moment a worker is deciding
   whether its work survived. It cost B1 a `git diff --cached` detour and a dig into `.jigc/tasks`.
4. **The carryover gate protects staging that exists at the mint, not staging that existed before the
   session.** A worker that tidies first and restores later gets those paths swept silently — `909a24c`
   carried both planted paths into a docs-only ADR commit, and **the worker caught it; jigc did not**.
   Not a gate failure and not data loss; `finalize` names what it is carrying nowhere the worker saw.
5. **B3a/B3b-1 · The router never surfaces off-router workflows** — two of four sessions, independently.
   A changelog-only intent got the 12-item catalog; `record-change`, deliberately hidden from the
   router, was found only by reading `jigc describe`.
6. **B2-3 · `milestone join --help` undersells the verb** — it describes a disjoint-union of staged
   *docs* and says nothing about code, while finalize reported `"code_files": 7`.
7. **B3b-3 · A foreign file's advisory repeats on every unrelated task** — correct, non-blocking, and
   noisy enough that the worker re-checked each time whether it was new.
8. **B3b-2 · `doc show --task` refuses right after `task bind`** with `store.not-staged`, because
   binding stages no copy. *"Sensible in hindsight, not obvious in the moment"* — and it is the verb
   this trial measures, refusing where a worker reaches for it.
9. **B3b-4 · Minted task ids are not predictable**, and were guessed wrong once (`no task`) until the
   worker read `.task` from the JSON instead of predicting the slug.

**Possible dead end — classification owed**

10. **B2-2 · Seeding sub-tasks makes them un-resumable from the main tree.** `jigc start --task
    <sub-task-id>` refused with *"task is pinned to base a99b6d9 but you're on 1d46b86"* — the
    milestone's **own bookkeeping commits** had moved HEAD past the pin. The error text is what
    taught the worker that sub-tasks need `milestone provision` and `.jigc/worktrees/<id>`; nothing
    before it did. Needs a repro to classify: if the only route out is the error text, it is close to
    a dead end.

**Capability gaps — §1 routes these to M46, and they are not defects**

11. **B2-4 · No `--dry-run` for `milestone finalize`** (`task finalize` has one) — and it is the
    direct cause of **B2-5**: with no way to preview a one-shot commit boundary over two worktrees
    holding *dependent* code, the worker consolidated by hand and `git reset --hard` the other
    worktree. Two things worth separating: the workaround was destructive, and the worker's caution
    was exemplary — it confirmed through `git check-ignore` that the ADR lived under the gitignored
    `.jigc/tasks/` before acting. The gap is that it had to ask git, because no jigc verb states that
    guarantee.
12. **B1-3 · No way to redo or narrow a finalize.** After the swept commit, the worker fixed it with
    `git reset --soft HEAD~1` and a hand-written `git commit`, bypassing jigc — *with the operator's
    explicit consent* (above) — then confirmed jigc reconciled cleanly. The clearest adapter-bypass
    instance of the trial, and the reasoning behind it is sound.
13. **B3a/B3b-5 · No append-to-slot and no bulk edit of existing items.** `set-slot` is
    overwrite-only, so extending a changelog category across two tasks meant re-authoring the whole
    text by hand; wiring `maps-to-test` across five committed criteria meant five `set-field` calls,
    because `doc author` refuses with `write.already-present`.
14. **B1-2 · No jigc surface for the docs gate** — the finalize refusal said only *"ask the docs
    reviewer to sign off"* and the worker read `.githooks/pre-commit` to learn the contract. The gate
    is a **foreign hook the plant installed**, not jigc's, so "jigc doesn't expose it" is arguably
    correct by design; recorded because the worker reached for a gate-status surface and found none.
15. **B2 · `jigc describe <item>` does not exist** — `describe milestone` exits 2; `describe` does
    the full tour or category flags only.

## The seventh discoverability landing — and what it is evidence of

**B1 could not find `jigc doc schema`.** It reported looking for *"a schema/introspection command for
a doctype's exact address tree (which leaves are fields vs. slots, required vs. optional) before
writing to it"*, judged `describe` and the workflow preview to give *"prose, not a structured
schema"*, and **reverse-engineered the shape empirically** — `doc create`, then `doc show --format
json`. `jigc doc schema adr` answers that question exactly, and has since M40.

**Report it as what it is.** §1's blocking row for a seventh landing is keyed to §3's measurement
returning *filesystem* or *neither*; that measurement returned **VERB 4/4**, so that row did not
fire. What landed instead is the pattern under §9's declared bound — *"whether the read-back fence
generalises beyond `doc show` … it says nothing about the next capability nobody finds"*. **The fence
worked for the verb it names, and the lens moved to the next unnamed capability.** That is a surface
finding routed to M46 and a piece of evidence for a bound the protocol declared in advance.

**Its true strength, which the running record does not yet state: this is a one-session landing.**
Counted from the logs, `doc schema` was invoked **11 times across three of the four sessions** — B2
4 (`research`, `deferral-ledger`, `decisions-log`, `roadmap`), B3a 3 (`changelog`, `spec`,
`arch-doc`), B3b 4 (`changelog`, `spec`, `adr`, `arch-doc`) — and **0 times in B1**. B1 is also the
one session without the adapter preloaded. That is n=1 on a correlation and must not be read as a
mechanism, but it does mean the honest headline is *one of four workers did not find `doc schema`*,
not *the fleet cannot find it*. It remains the signature the protocol pre-registered — a headline
complaint that dissolves into shipped capability — for the fifth consecutive trial.

## What held — prior fixes this trial confirms

M43's carryover gate (three probes, per-path, identical on rc.10) · M47's survivable hook-rejection
frame, with HEAD unmoved and the ADR intact through a `core.hooksPath` rejection · M42's
managed-vs-foreign discriminator, live on a blind path, routing a foreign ADR to `ingest`/`migrate`
and never to `migrate-corpus` · M43's exit-4 migration review hold and `--approve` as the sole
destructive gate, followed unassisted · M48's three destroying-door guards, all four criteria at each
door, with `--force` honoured · M48's guide artifact on both ownership branches · M48's read-intent
rule at a read-shaped near-miss · the rc.10 → rc.11 upgrade path across three doctypes
(adr · root changelog · milestone-record) · the route floor: every block encountered anywhere in this trial carried a route,
and every route the walk followed verbatim worked.

## Honest bounds

- **The RC-pre-1.0 comparison is indicative, not direct.** Isolation (§2) removed a variable present
  in all six prior trials — deliberately, with the cost stated before the run — so a difference in
  the numbers cannot be attributed to M48's fence alone. The absolute measurement, read against the
  control, is what carries weight.
- **N=4 proves compliance, not reliability**, and it measures **instructed** read-back: the composed
  step prints the command verbatim. This is a different construct from the six prior discoverability
  landings and is not comparable to them.
- **The cue-card mechanism is unproven and its arm untested.** It never fired in four attempts, so
  the F2 reject-and-retitle path went unexercised by any blind session, and the trial has no
  measurement of what a worker does when a doc's identity moves under prose it already wrote. The
  design lesson is recorded: a correction aimed at a turn boundary cannot be delivered into an arc
  that never pauses.
- **Two sessions' plants were defused or void.** B1's carryover plant was cleared by the worker
  before the mint (a plant is only live if the staged set survives to the mint); B3a's foreign-ADR
  plant never landed. B3b re-ran both of the latter's arms; B1's carryover arm has no re-run and is
  **untested by this trial** — its gate is fenced by probes, not by a blind path.
- **`bypassPermissions` is a declared, directional confound** (§9). It makes the FILESYSTEM outcome
  cheaper than an adopter would find it, so the 4/4 VERB result is *not* weakened by it — the verb
  was chosen against a filesystem read made artificially easy — but any future null under this
  setting must carry the bound.
- **Regressions have a detection path on exactly one arm.** Blind sessions carry no baseline; only
  walk arm 2 touches rc.10. *"Worked on rc.10, does not on rc.11"* is not systematically searched.
- **The corpus is the observer's**, one template, `--clean-prose`, no `node_modules`, 23 tests. It
  buys an exact "from nothing" premise and pays in unfamiliarity; nothing here exercises a heavy
  real-world tree.
- **The isolated container is not an adopter's machine.** No user-global instructions, no plugins, no
  other skills. That is the right instrument for *attributing* a result and the wrong one for
  predicting an adopter's day.
- **B2's research prose came partly from a delegated sub-agent** (one `Agent` call), so its
  authoring arc is not a single-context one. Recorded as a structural fact, without a verdict.
- **The walk cannot see interaction defects by construction**, and the manifest-hash fence is **not
  trial-testable** at all (§6) — it fires in CI over a pushed range and carries no verb, finding or
  route.
- **Adopter-side freeze protection, multi-process concurrency and long-horizon drift are out of
  scope** (§9), unchanged.

## What this trial owes onward

1. **`findings-verification.md`, with a live repro block per verdict** — CONFIRMED *and* REFUTED
   (§7) — and each row carrying `pinned-by: <suite>::<test>` or a stated `UNPINNED: <why>`, verified
   by reading what the cited test asserts. **The human's gate on the 1.0.0 call is that ledger**, and
   this trial opens a new one.
2. **The §6 coverage table**, in three columns — trial-reached · test-fenced (naming the suite) ·
   neither — over M48's diff, with the *neither* column empty or each member explained.
3. **PT-1's disposition**, the human's under §1's blast-radius qualifier. The recheck it was waiting
   on has landed and is recorded above; what remains is the call.
4. **Two corrections owed to the contemporaneous records**: B1's channel counts in
   [session-findings.md](session-findings.md) (`5/5/0/1` → `3/3/0/2`, verdict unchanged), and B1's
   second operator utterance, missing from [operator-log.md](operator-log.md).
5. **The cue-card instrument**, if a future trial wants the designed-need occasion: it must be fired
   from something other than a queued message at a turn boundary, or aimed at an arc that pauses.
