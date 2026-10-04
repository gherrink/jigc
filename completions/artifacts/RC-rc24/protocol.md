# The blind agent trial on `1.0.0-rc.24` — pre-registered protocol

**Binary under test: the published `jigc 1.0.0-rc.24`**, installed from crates.io into
`jigc-gate:registry-1.0.0-rc.24` and gated by `gate-rc24.json`. Written 2026-10-03, **before any
session runs**. Status: **draft** — [open-forks.md](open-forks.md) lists what the human has not
yet decided, and nothing below is frozen until those are taken.

**What this trial is for.** It is the last usability instrument before the 1.0.0 call, which is
the human's. The per-axis reviews measure *defects*; the question here is the other one, asked at
rc.19 and never answered from a blind session — *is anything left that makes jigc unusable by an
LLM agent?* No blind agent has run since `1.0.0-rc.14`, and five milestones have since landed on
the surfaces an agent touches. `implementation/decisions-pending.md` → *A blind agent trial on the
release binary* settles what the session covers — **the fan-out, `jigc task amend` and the
findings channel** — and `DECISIONS.md` → *the road to the 1.0.0 call* (D3) settles when: before
the call, on the candidate that carries M54 and M55. rc.24 is that candidate plus the co-author
trailer (`DECISIONS.md` → 2026-10-03), which is why the trailer is checked here too.

**Paths in this file are repo-root-relative and backticked, not links**: the file is drafted
outside the repository and its final directory (`completions/artifacts/RC-rc24/`, provisional) is
not settled.

**The binary's identity is the image id.** A registry image carries no tree of ours, so
`JIGC_SHA` is `unknown` by construction and every `jigc-sha` this trial records says so. crates.io
never serves two builds under one version, and `run.py gate` refuses a rebuilt tag by image id.
`verify-pair.sh` needs a sha-built older image and a probe set for the pair; neither exists for a
registry build, so it is **not applicable and not run** — which also means this trial carries no
behavioural proof against rc.14 (§12).

The rule every instrument obeys, from `completions/artifacts/RC-1.0-gate/cue-card-postmortem.md`:

> An instrument fires reliably iff its trigger is a **state**, its consequence is **re-raised by
> the product**, and **every worker behaviour maps to a scored outcome**.

A correction that was *scheduled* fired 0 times of 4 (`RC-1.0-gate/paste/b*-correction.txt`), and
headless `-p` has no mid-turn channel at all. So this trial schedules nothing: arm (c) uses a
standing state, and arms (a) and (b) deliver their second sentence at a **turn boundary**, the one
moment a headless conversation can receive one.

---

## 0 · Declared behaviour changes since rc.14, briefed before the first session

An unbriefed observer scores any of these as a regression and the trial then spends its budget
re-deriving a decision this repo already took. That near-miss has happened twice
(`RC-rc14/trial-record.md` → §0).

**Where this list lives is an open fork.** Every earlier trial cited a list in
`implementation/decisions-pending.md`; no entry there covers M51–M55. This section therefore
carries the list itself, derived 2026-10-03 from the M51–M55 spans of
`implementation/project-history.md` and from driving the installed registry binary. **The rc.14
list still stands** (`implementation/decisions-pending.md` → *The trial that follows M50 —
protocol inputs*) and is not restated; two of its items are the likeliest to be met again and are
repeated under 0.5.

Marked **driven** where the installed `jigc 1.0.0-rc.24` was run to see it (host registry build,
throwaway corpus from the template); everything else is **from the record**.

### 0.1 · Met on every arm

- **Every commit jigc makes under the agent carries a `Co-Authored-By` trailer.** New in rc.24;
  §6 is its check. *Source:* `DECISIONS.md` → 2026-10-03; `design/assistant-adapter.md` → *The
  co-author trailer*. **Driven.**
- **The router catalog has thirteen entries, and `report-inconsistency` is one of them** — *"code
  and a doc, or two docs, disagree and the disagreement is worth a record until it is
  reconciled"*. Two more doctypes (`jigc-feedback`, `inconsistency`) appear in `jigc describe` and
  `jigc doc schema`. *Source:* M55 span; `design/findings-channel.md` §2. **Driven.**
- **`.jigc/AGENT.md` and the installed guide changed.** The bootstrap body now says where printed
  paths are rooted and names `jigc task amend` as the repair for a landed commit message; the
  guide carries a paragraph on it. A transcript that quotes either is quoting the product.
  *Source:* M53 span (the rc.19 cwd arc; F-10). **Driven.**
- **Every `git` command jigc prints leads with `git -C <the repository's absolute path>`**, and a
  fan-out `Spawn:` line's `cd` is absolute and quoted. A worker that types `git -C …` is copying
  the product's own shape. *Source:* M53 span (rc.19). **Driven** (the `Spawn:` line).
- **A refusal under `--format json` is a findings document keyed `(code, target)`**, and an
  unknown work-unit id is a typed finding at all 25 doors that take one. *Source:* M51 span
  (Increment 6); M52 span (Increment 1).
- **Committing doors refuse under an un-concluded git operation or a detached HEAD**
  (`repo.operation-in-progress`, `repo.head-detached`), with no override and a route naming the
  git command; `jigc task validate` previews that posture. *Source:* M51 span (Increment 2); M52
  span (Increment 3); M53 span (Increment 4 and the rc.21 batch).
- **A file jigc did not write inside a task's working area is no longer silently removed.**
  `task finalize` and `milestone finalize` move it to `.jigc/displaced/<id>/…` and name it on a
  `displaced` key; the consenting doors refuse with `<door>.foreign-bytes` and take `--force`. A
  worker that parks a scratch file under `.jigc/tasks/<id>/` meets this. *Source:* M52 span
  (Increment 4); M53 span (Increments 1–2).
- **The `doc-code` probe is inside the one binary.** There is no sibling `doc-code` executable;
  a worker that goes looking for one finds none, and nothing is missing. *Source:* M54 span.
- **`doc show` and every `doc list` row carry `title` and the effective `fields`.** *Source:* M55
  span (the read surface).

### 0.2 · Met on arm (a), the fan-out

- **`milestone create` and `milestone add-task` each land a record-only commit and say so**
  (`record commit: <sha> — … committed on its own`). A milestone therefore moves `HEAD` before
  any work is done. *Source:* M52 span (Increment 10: every committing door says that it commits).
  **Driven.**
- **A sub-task's composed text has no commit-boundary steps.** It ends *"never `git commit` and
  never `jigc task finalize` here"*, and under the default `finalize.fan-out.squash = true` it
  asks for no commit doc: the boundary lands one commit with a CLI-synthesized message
  (`Finalize milestone <id> (N sub-tasks)`). A boundary that lands over un-authored sub-task
  commit docs is as designed, **not** a regression of the rc.14 list's `d854e25` item, which binds
  under `squash: false`. *Source:* M55 span (sub-task composition); `design/workflow-dialect.md` →
  *Composing for a fan-out sub-task*. **Driven** (default squash only).
- **`jigc start --workflow milestone-execution` and `--workflow sub-task` refuse**
  (`workflow.verb-routed`, exit 1) and route to `jigc milestone execute <id>` /
  `jigc workflow sub-task --task <id>`. *Source:* M52 span (Increment 9). **Driven.**
- **Resuming a sub-task from the main checkout refuses and routes to `jigc milestone provision`.**
  Orientation lists each sub-task of an open milestone as an active task. **Driven**; the
  increment that landed the route was not traced.
- **`jigc milestone finalize` refuses over a sub-task worktree that is mid git operation**, and
  store doors run from inside a worktree resolve the repository, not the worktree. *Source:* M53
  span (the rc.18 fix; the rc.19 cwd arc).
- **Inside a fan-out worktree an ordinary task's `finalize` refuses `repo.head-detached`** and
  routes to `git switch -c <new-branch>`. A worker that starts a non-sub-task there meets it.
  *Source:* M53 span (the rc.21 batch).

### 0.3 · Met on arm (b), the correction

- **`jigc task amend` exists.** It mints a task pinned to `HEAD` with an *empty* commit doc; the
  worker re-authors the message from scratch and `jigc task finalize <id>` rewrites the commit
  with `git commit --amend`, tree untouched. It refuses a non-empty index
  (`finalize.amend-index-dirty`), a staged managed doc (`finalize.amend-staged-doc`) and a moved
  `HEAD`. The ack reads `amended <old> → <new>`. *Source:* M53 span (F-10, the one capability
  built before 1.0.0). **Driven.**
- **`jigc start --workflow amend` refuses** (`workflow.verb-routed`) and routes to
  `jigc task amend`. **Driven.**
- **A correct amend leaves `commit (amend)` in the reflog**, and `run.py observe` reports that
  line as `HISTORY REWRITTEN` — §8.3's declared bound. **Driven.**

### 0.4 · Met on arm (c), the disagreement

- **A report task files exactly one new doc and lands it alone.** The finalize is path-scoped: it
  commits the task's doc, leaves every other staged path staged and names it `left-staged`, and
  the carryover gate does not fire there. A second create onto an existing slug refuses
  `create.already-exists`. The doc lands at `docs/inconsistencies/<slug>.md`. *Source:* M55 span
  (the doc-only finalize; the create-only gate); `design/findings-channel.md` §§3–4. **Driven**
  (file and land; not beside an open task).
- **`report-jigc-feedback` and both triage workflows are hidden from the router by design.** They
  compose by name and a blind worker is **not expected** to find them; not finding them is not a
  finding. *Source:* `design/findings-channel.md` §2. **Driven** (it composes by name).

### 0.5 · Met only if a session strays, and the two carried from rc.14

- **`jigc setup` refuses its install commit over bytes it did not write**, with `--force` the loud
  consent. *Source:* M51 span (Increment 3 and the audit's MEDIUM).
- **`jigc validate` exits non-zero in two more cases**: a stamped doc at a home no doctype claims
  (`schema-conformance.orphaned-instance`) and a declared home jigc committed into that is now
  empty (`schema-conformance.home-vacated`). *Source:* M51 span (Increment 8); M52 span.
- **A title that yields no id is refused where the id is derived** (`write.unslugable-title`).
  *Source:* M53 span (Increment 5).
- **A singleton's address is refused at the address doors** (`store.fixed-identity`), and
  `jigc doc schema --format json` is at `contract-version` 7. *Source:* M52 span (Increment 6).
- **Carried from the rc.14 list: the deny floor.** `Bash(jigc uninstall:*)` and
  `Bash(jigc milestone discard:*)` are denied by the adapter profile, so a worker's call to either
  shows a **harness** denial and no jigc output. rc.14's fan-out arm halted exactly there. It is
  not friction to count. **Driven** (the installed `.claude/settings.json`).
- **Carried from the rc.14 list: `jigc task discard` refuses over staged prose**, and minting a
  task stages its commit doc, so an ordinary discard meets the refusal from the first moment.

### 0.6 · The standard a declared refusal is judged against

Carried from `RC-rc14/protocol.md` §5. A declared change **reads as designed** only if (a) the
output **names what it objects to**; (b) it **says the consequence**; (c) it **names the route or
consent in the same output**; and (d) that route, run verbatim, **works**. Any of the four
missing, it reads as obstruction and is a finding.

---

## 1 · The pre-registered decision rule

The exit rule (`DECISIONS.md` → 2026-09-21; its consequence at *the road to the 1.0.0 call*)
governs, and this trial adds nothing to it.

| A finding is | Consequence |
|---|---|
| **tier-1** — exit-0 loss, or repository harm, through a door that commits, destroys or moves | **A fix pass** before the call. Never a wave. |
| **tier-2** — a posture or route dead end; a code-less or undeclared refusal | **Recorded**, never blocks. Filed through jigc at the port. |
| **tier-3** — a surface that says something the binary does not do | **Recorded**, never blocks. Filed through jigc at the port. |

The tier scale's one home is `implementation/decisions-pending.md` → *The rc.16 wave (M52)*; it
is not restated here.

**An arm's outcome is not a finding.** §§3–5 say which outcome each arm landed in; a *not
reached* is evidence about usability at n=1 and goes to the human as that. It becomes a finding
only when a **surface** is shown to have caused it — a route that could not run, a sentence that
was false, a door that was not named where the worker stood — and then the finding is tiered on
its own evidence. A worker's judgment call is recorded as worker behaviour, never as a row
(`RC-m50/findings-verification.md` → *Worker behaviour, pre-registered* is the precedent).

**Two rules bind the adjudicator**, carried from `RC-rc14/protocol.md` §1: a finding's tier is
fixed from its evidence **before** its consequence is looked up; and *"judged not to matter"* is
not a disposition.

---

## 2 · Instruments

**Three blind arms, headless, n = 1 each.** Every session runs filesystem-isolated
(`completions/trial-harness/README.md`), on a fresh corpus that is never reused, with the
invocation log on.

### 2.1 · The arms

| arm | corpus | prompt | transport | permissions | scored |
|---|---|---|---|---|---|
| **walk 00** | `walk-rc24`, naive | — | scripted, `walk.py --only 00` | n/a | **the positive control — runs FIRST** |
| **(a)** the fan-out | `fenwick`, clean prose, adopted | `paste/a-turns.txt`, two turns | headless, `run.py seed` | `bypassPermissions` | by hand, §3 |
| **(b)** the correction | `halloway`, clean prose, adopted | `paste/b-turns.txt`, two turns | headless, `run.py seed` | `bypassPermissions` | by hand, §4 |
| **(c)** the disagreement | `calderby`, **standing wart**, adopted | `paste/c-prompt.txt`, one prompt | headless, `run-session.sh --headless` | `bypassPermissions` | by hand, §5 |
| the trailer | all three out-dirs | — | read from `.git` and the log | — | by hand, §6 |
| the read-back | all three out-dirs | — | `run.py observe` | — | reported, not headline, §7 |

**Model and CLI:** the harness defaults — `claude-sonnet-5`, and the Claude CLI the image pins
(`2.1.233`, in the gate record). `JIGC_GATE_MODEL` must be unset.

**Permissions, per arm, and why** (the increment-0 finding is that `--strict-permissions`
headless reproduces the adopter's condition faithfully: jigc runs, raw file writes are denied):

- **(a) and (b): `bypassPermissions`, with no alternative.** `run.py seed` has no strict door.
  And both arms need source edits, which a strict headless turn is denied — increment 0's strict
  arm stopped at 13 records and `RC-rc14`'s B3-strict halted after four denials, three of them
  `Edit`s. A strict run of either would measure the denial.
- **(c): `bypassPermissions`, by choice.** A strict run would deny an edit to `README.md`, which
  removes *fixed silently* from the outcomes a worker can reach and pushes it toward the product's
  channel. Under bypass every outcome in §5 stays reachable, and a worker that files through jigc
  did so against the cheaper path.

`bypassPermissions` is a **directional confound**, declared (§11): it makes going around jigc
easier, so a *reached* is not weakened by it and a *not reached* is partly attributable to it.

### 2.2 · The corpora

All from `completions/trial-corpus-template/`; [corpora.md](corpora.md) has the commands. Order is
fixed: **instantiate → gate → adopt → carry**. Adoption runs through the container's own binary
(`run-session.sh --exec arms/adopt.sh`), so the state under test is produced by the build the
sessions run. No corpus is planted.

### 2.3 · Three preconditions, in order — each stops the trial if it fails

1. **Walk arm 00, the positive control.** It proves the chain a blind session rides: copy-in, the
   binary, the invocation log, copy-out, a countable `doc show … --task`. If it does not fire, no
   blind result may be read. Its commands were driven against the host registry binary and pass;
   the container run is the control itself. It proves the channel records. It proves nothing about
   discoverability — the operator already knows the verbs. **Arms 01–23 are not run** (§12).
2. **The environment probe.** One headless turn, `paste/env-probe.txt`, **not blind and not
   scored**: is `CLAUDECODE` set, non-empty, in the headless agent's shell, and in a sub-agent's?
   It prints `SET` or `UNSET` and never a value. §6 says what each reading means.
3. **The seed smoke.** Two trivial turns through `run.py seed` (`paste/seed-smoke-turns.txt`).
   `seed`'s turn loop was repaired on the day this was written and has not carried a live
   conversation since `_find_transcript` began returning a list. The postmortem's rule 4 is that
   the axis you are uncertain about is paid for **before** the trial. Pass condition: turn 2's
   reply contains `ALPHA BETA`, and the frozen manifest says `turns: 2`.

**No arm is rehearsed against a live agent.** That is the assumption this design flags as its
largest, and whether to pay for it is the human's ([open-forks.md](open-forks.md) → F4).

### 2.4 · No widening, and one re-run for a void

**No arm is run again because of where it landed.** Three arms at n = 1 is the registered shape;
a second draw taken after seeing the first is a post-hoc widening, and `RC-rc14/protocol.md` §3.5
refuses it in advance for the same reason.

**A void arm is re-run once**, on a fresh corpus under a new name, and both runs are reported. A
void says nothing about jigc, so re-running it selects on nothing. A second void is reported as
the arm's result: the instrument could not be made to fire.

Order: **(c), (b), (a)** — the arm that does not depend on `seed` first, the longest last.

---

## 3 · Arm (a) — the fan-out

### 3.1 · What the record says, corrected by the evidence

The fan-out has been put in front of a blind agent three times with **one** prompt
(`RC-m50/paste/b4-prompt.txt` = `RC-rc14/paste/b4-prompt.txt` = line 1 of
`RC-m50/paste/b4-turns.txt`), and it reached provision and join once:

| run | what happened |
|---|---|
| RC-m50 B4-h, single prompt | Stopped at the planning workflow's **human-owned Settle gate** and asked. 5 records, nothing written. |
| RC-m50 B4-s, seeded two turns | Reached `provision`, three `workflow sub-task`s, `join` (one collision, resolved) and `milestone finalize`. |
| RC-rc14 B4-h, single prompt | Planned a three-increment **dependent** spine, judged the fan-out unfit for it, moved to discard the milestone and halted at the deny floor (F-13). |

**The seeded run's second turn did not cause its success.** Read from that run's own out-dirs on
the machine that ran it: turn 1 began 18:23:07Z and its log holds `milestone provision`
(18:35:51Z), the three sub-task workflows, both joins and `milestone finalize` (18:44:02Z); turn 2
began 18:44:43Z and added two records. `RC-m50/trial-record.md`'s own row says *"B4-s (turn 1)"*.
`RC-m50/operator-log.md` describes the reply as delivered *"after the worker's turn-1 question"*;
in the seeded run there was no such question. So the one success and the two failures are **three
draws of the same prompt**, and what separated them was whether the worker stopped at a gate and
how it cut the work. In the run that reached the fan-out the worker implemented the three
sub-tasks itself, one after another, in three worktrees that all touched `src/store.ts`.

What the two failures establish:

- **A human-owned gate ends a headless turn.** The pack's Settle gate says *"The human owns this
  gate"* (driven on rc.24: the sentence is still there).
- **A dependent cut has nowhere to go.** The planning text says *"The spine stays linear — each
  increment builds on the last"* (driven on rc.24: still there). A worker that maps its increments
  onto sub-tasks then correctly judges the fan-out unfit.

### 3.2 · The design

Both causes are removed in the prompt, in product language, and the second turn is given real
work instead of being a reply that may arrive after everything is done:

- **Turn 1 hands over the cut and the sign-off, and asks for the milestone to be set up and
  nothing more.** Three pieces, stated as independent, each in its own module with its own tests
  and its own constant, so no piece touches a file another touches. Read against the template's
  source: the three live in `src/store.ts`, `src/validate.ts` and `src/ingest.ts`, and with the
  limits as constants no piece needs `src/config.ts`.
- **Turn 2 says go**: start all three together, each kept apart from the other two, land them
  together as the one milestone. It also restates the sign-off, so it is a valid reply to a turn 1
  that stopped at a gate and asked.

**The state turn 2 arrives into is re-raised by the product** (driven on rc.24): with a milestone
open and its sub-tasks unstarted, the SessionStart hook's `jigc start` lists each sub-task as an
active task, and resuming one from the main checkout refuses and routes to
`jigc milestone provision <id>` and the worktree's absolute path.

**One bound of that state, also driven:** the orientation view names `resume`, `validate`,
`milestone finalize` and `discard` for each sub-task. It names neither `jigc milestone provision`
nor `jigc milestone execute`. A worker reaches `provision` through the resume refusal's route or
through `jigc milestone --help`, and reaches the `Spawn:` lines only through `execute`. If arm (a)
lands in A-3 or A-4, this is the first surface to look at.

Every turn-1 end state maps to an outcome: it set the milestone up and stopped (the designed
path); it stopped at a gate and asked (turn 2 answers); it ignored *stop there* and ran everything
(turn 2 is then idle, and the conversation is still scored as one); it did the work without a
milestone (A-3).

The words *milestone* and *sign-off* are in the prompt. *Milestone* is also a jigc verb, and
RC-m50 and RC-rc14 used it the same way; whether that stays is [open-forks.md](open-forks.md) →
F2. The alternative shape — the whole job in turn 1, as in the three earlier runs — is F1.

### 3.3 · Outcome classes, fixed before the run

Scored over the **whole conversation** — turn 1's and turn 2's records together.

| class | the evidence that puts a session here | reading |
|---|---|---|
| **A-1** provisioned, worked in isolation, landed through jigc | `milestone provision` at exit 0; work for ≥ 2 sub-tasks done inside `.jigc/worktrees/<id>/`; `milestone finalize <id>` at exit 0, its commit carrying all three pieces' code | **reached** |
| **A-2** provisioned, not landed | `milestone provision` at exit 0, and no `milestone finalize` at exit 0 by the end of turn 2 | **reached provision; the boundary not reached** — the refusal or halt that stopped it is quoted and judged against §0.6 |
| **A-3** the work done without the fan-out | the three pieces exist in the tree or in commits, and there is no `milestone provision` at exit 0 — through ordinary tasks, its own sub-agents in the main checkout, or raw commits | **not reached** — includes *judged unfit*; the worker's stated reason is quoted verbatim |
| **A-4** halted | the conversation ends asking, or on a denial, with no `milestone provision` at exit 0 and the pieces not done | **not reached** — the gate or door that stopped it is named |
| **A-V** void | a turn's CLI exited non-zero; no invocation log; `seed` aborted between turns; turn 2 began cold | **VOID** — apparatus, says nothing about jigc |

Recorded beside the class, never changing it:

- **who worked the pieces** — sub-agents (sub-agent transcripts in a turn's out-dir whose commands
  run inside the worktrees) or the main agent, one worktree after another. A-1 with sub-agents is
  the full path; A-1 without is the RC-m50 path. Neither is the genuine concurrent spawn M55's
  audit ran from a main session, and a headless arm cannot be made to be;
- **the route to `provision`** — `milestone execute` and its `Spawn:` lines, the resume refusal,
  or `--help`;
- **whether `milestone join` was run on its own** — `milestone finalize` runs the join itself, so
  a session with no `join` record still joined;
- **a join collision** (`combine.code-collision`) and how it was resolved;
- **a halt at the pack's human gate after two sign-offs** is A-4, not void: the prompt answered the
  gate twice, so a third ask is a result about the gate under this transport.

**Why A-V is narrow.** RC-m50 scored its Settle halt VOID because nothing had been authored and
headless had no reply. Here the reply is pre-written into turn 2, so a halt is a branch with an
outcome.

---

## 4 · Arm (b) — the correction

### 4.1 · What the door is, and so what the correction has to be

Driven on rc.24: `jigc task amend` *"repairs the message, never the change"*. It rewrites the
commit at `HEAD` and nothing else, and its own text says a wrong change is a new task. So the only
correction this arm can measure the door with is one to a **landed commit's message**, on the
commit that is still `HEAD`. A correction to code or to a managed doc has a different right
answer and would measure nothing about the door.

It re-measures RC-rc14's F-10 — the one place in five blind sessions where an agent left the
adapter, because no jigc verb could repair a landed commit and raw git was the only path. The verb
now exists and `.jigc/AGENT.md` names it. The question is whether a blind worker asked for exactly
that repair takes it.

### 4.2 · The design

- **Turn 1** asks for one small fix landed as a single commit, and gives it a ticket to cite:
  `TKT-212`. The fix is real and small (driven against the template: `GET /summary/` with an empty
  series name answers 200 with `: no samples`). *One commit* is in the prompt so that the commit
  citing the ticket is `HEAD` when turn 2 arrives.
- **Turn 2** says the ticket was wrong — it is `TKT-221` — that nothing has been pushed, and that
  the same commit should cite the right one with the change itself untouched.

Turn 2 states an outcome and names no means. *Nothing has been pushed* removes the one good reason
to refuse a rewrite. Any git-literate worker hears `git commit --amend`; the adapter's sentence is
what has to carry it to the door, and that is the measurement.

`TKT-<n>` is this repository's own anonymization vocabulary
(`implementation/public-hygiene.md`), so the tokens are safe to commit.

### 4.3 · Outcome classes, fixed before the run

| class | the evidence that puts a session here | reading |
|---|---|---|
| **B-1** corrected through `jigc task amend` | a `task amend` record at exit 0 in turn 2, then that task's `task finalize` at exit 0; exactly one `commit (amend)` in the reflog; `HEAD`'s message cites `TKT-221` and not `TKT-212`; `HEAD`'s tree equals the superseded commit's; no raw git history act in the transcript | **reached** |
| **B-2** a new commit; the landed message unchanged | a second commit in turn 2 by any means, and the turn-1 commit's message still cites `TKT-212` | **not reached** — and the correction did not happen in substance |
| **B-3** rewritten outside jigc | the reflog shows an amend, reset or rebase with **no** `task amend` record behind it | **not reached** — an adapter bypass on the commit boundary; what the worker tried first, and its stated reason, are quoted |
| **B-4** not corrected | turn 2 ends with `HEAD`'s message still citing `TKT-212` and no second commit: it refused, asked, or only described the fix | **not reached** — the reason is quoted |
| **B-5** through the door, wrong result | B-1's records, and `HEAD`'s message does not cite `TKT-221`, still cites `TKT-212`, or lost what the commit does | **reached the door; the repair failed** — worker or surface, decided from the transcript |
| **B-V** void | apparatus, as A-V; **or turn 1 left no occasion** — no commit, the ticket on no commit message, or the commit citing it is not `HEAD` | **VOID** — the door rewrites `HEAD` only, so a correction it cannot reach measures nothing about it |

Recorded beside the class:

- **attempts** — a `task amend --help`, a `task amend` that minted and was then discarded, an
  amend refused at `finalize.amend-index-dirty`: each quoted with its output and judged against
  §0.6. A B-3 that went through a refused amend first is a different result from one that never
  tried;
- **where `TKT-212` went in turn 1** — a trailer, the summary, the body. If it also landed in the
  **tree** (a code comment, a changelog entry), the amend cannot reach that copy by design and a
  full correction is an amend plus a new task; the class is scored on the message;
- **the trailer on the rewritten commit** — §6.

---

## 5 · Arm (c) — the disagreement

### 5.1 · The instrument is the template's standing disagreement, used on purpose

Nothing is planted. Instantiated **without** `--clean-prose`, the corpus template says in three
places — `README.md`, `package.json`, and the header of `src/store.ts` — that the service is *a
rollup cache in front of whatever long-term store* the caller has, while `src/router.ts` has no
egress and no upstream reader. Driven against a fresh instantiation: three sites, and nothing in
`src/` forwards or fetches.

`completions/trial-corpus-template/README.md` records this honestly and so does this protocol:
**it is an accident, not a designed trap.** It was introduced unintentionally when the template
was first written, and it was kept because two blind sessions in RC-pre-1.0 found it independently
and routed it to the human rather than resolving it silently. What is deliberate here is only the
choice to instantiate *with* it and to give the worker a task that passes through it.

Those two sessions were interactive, and both took the disagreement to the human. A headless
worker cannot: a question ends the turn. The channel M55 built is the product's answer to *found
mid-work, worth a record* — and the router shows it to every worker. This arm asks whether a blind
worker that meets a real disagreement reaches for it.

### 5.2 · The design

One prompt: write the architecture document — how a sample travels from the wire to a summary,
which component owns what, and **where the service's edges are: what comes in, what it keeps and
for how long, and what it hands on to anything outside itself**.

The last clause is what makes the occasion exist. An architecture document has to say what the
service hands on; the prose says a long-term store; the code hands on nothing. A worker that
answers that clause from the code meets the disagreement while doing the work it was asked for.
The prompt does not say that anything disagrees, and it does not make `README.md` the deliverable
— so correcting the README is the worker's choice, not its task.

### 5.3 · Outcome classes, fixed before the run

| class | the evidence that puts a session here | reading |
|---|---|---|
| **C-1** filed through the channel | a `start --workflow report-inconsistency` record at exit 0, a `doc create inconsistency` at exit 0, that task's `task finalize` at exit 0; a doc under `docs/inconsistencies/` in a commit of its own, naming at least two of the four sites as sides | **reached** |
| **C-2** recorded through jigc, by another door | the disagreement is written into a managed doc through jigc — the architecture document's own prose, a decision record, a parked idea — and no `inconsistency` doc exists | **not reached** — recorded, and the router line's wording is the first surface to look at |
| **C-3** fixed silently | `README.md`, `package.json` or the `src/store.ts` header is edited to remove the claim (or code is added to make it true), and nothing records that they disagreed | **not reached** |
| **C-4** mentioned only | the worker names the disagreement in a turn's final message, or ends asking which side is right, and nothing in the repository records it | **not reached** — under this transport a question is a stop |
| **C-5** missed | nothing in the transcript, the final message or any doc shows the worker noticed | **not reached** — with the sub-case **propagated**: the architecture document asserts the hand-off the code does not make |
| **C-V** void | apparatus, as A-V; **or no occasion** — the transcript shows the worker read none of the three sites that make the claim | **VOID** |

Recorded beside the class:

- **filed and fixed** — a C-1 that also edits a site is C-1; the note says which sites, and
  whether the doc's `status` was moved;
- **filed beside the open task or after it** — M55's claim is that a report lands beside an open
  task without touching its staging; which of the two happened is read from the record order;
- **the ordinal and elapsed time of the first read of each site**, from the transcript — the
  sub-measurement `RC-rc14/protocol.md` §3.2 added, because it costs nothing;
- **the worker's own wording** for what it found, quoted, before any class is assigned.

---

## 6 · The trailer check

**The requirement, the human's:** every commit jigc makes in the trial carries
`Co-Authored-By: Claude <noreply@anthropic.com>`.

**What the design says, so the check is scoped to what can be true**
(`design/assistant-adapter.md` → *The co-author trailer*; each line driven on rc.24):

- jigc appends the trailer **only while `CLAUDECODE` is set and non-empty in its own process**.
  With the variable empty, a finalize lands no trailer.
- `arms/adopt.sh`, walk arm 00 and every `--exec` script run **outside** Claude Code. `jigc setup`'s
  install commit made there carries no trailer **by design**, and `adopt.sh`'s own adoption commit
  is raw git, not jigc's.
- jigc **de-duplicates by address**. The composed commit step invites the worker to add its own
  `Co-Authored-By` item, so a commit may legitimately carry
  `Co-Authored-By: Claude Opus <noreply@anthropic.com>` and nothing else.
- **The amend arm re-derives it**: the rewritten commit carries the trailer once.

### 6.1 · Scope

**Commits jigc made inside the agent session** — and they are identified, not guessed:

1. the committing-door records in `.jigc/logs/invocations.jsonl` at exit 0, at or after the
   session's start (`task finalize`, `milestone finalize`, `milestone create`,
   `milestone add-task`, `milestone add-from-spec`, `milestone discard`, a sub-task's
   `task discard`, `rename`, `migrate-corpus`, `setup`);
2. every commit object created since the session's start, **reachable from any ref or from the
   reflog** — a commit a later amend superseded is still a commit jigc made;
3. the join of the two by time, each match then checked against the raw git acts in the
   transcript, because a raw `git commit` inside a door's window would otherwise be credited to it.

`tools/trailer-rows.py` prints that join and `tools/raw-git-acts.py` prints the raw acts. Both
print rows and score nothing; each was run against commits made on rc.24 with the variable set,
empty, and with a same-address variant.

### 6.2 · Rows, fixed before the run

| row | what it is | reading |
|---|---|---|
| **exact** | one trailer on the address, byte-exact `Co-Authored-By: Claude <noreply@anthropic.com>` | **the expected result** |
| **variant** | one trailer on the address, any other spelling of the name or the key | **its own row, with its cause** — the `doc add-item commit:<task>#trailers` record that authored it is cited. Never silently passed, never a failure of the feature |
| **NONE** | a jigc-made, in-session commit with no trailer on the address | **a defect row** |
| **DUPLICATE** | more than one trailer on the address | **a defect row** — de-duplication failed |
| **not-jigc** | a commit made since the session's start that no door record accounts for | listed, **not checked** — it goes to the arm's rubric as a raw commit |
| **control** | the install commit `adopt.sh` made outside the agent | **expected to carry none**. A trailer there would be a false co-author on a commit no agent made |

A commit carrying the exact trailer **and** a second `Co-Authored-By` on another address is
*exact*, with the second line noted.

### 6.3 · The environment probe's readings, fixed before it runs

Whether the headless agent's processes, and a sub-agent's, carry `CLAUDECODE` in this image is
**not known**. The probe says, before an arm is spent:

| probe | what it means for the check |
|---|---|
| `MAIN-SET` · `SUB-SET` | the precondition holds everywhere; any **NONE** is a defect in the feature |
| `MAIN-SET` · `SUB-UNSET` | a commit made by a jigc call a **sub-agent** ran is expected to land **NONE**, and that is a defect row about the gate, not about the seam; main-session commits are expected *exact* |
| `MAIN-UNSET` | no headless commit can carry the trailer. **Stop and take it to the human before any arm runs** — the requirement cannot be met on this transport, and three arms would each land the same row |

---

## 7 · The read-back, reported and not the headline

`run.py observe` scores the read-back channels of `RC-rc14/protocol.md` §3.3 — **VERB**
(`jigc doc show … --task …` in the invocation log), **VERB-ADJACENT** (`task diff`,
`doc list --task`, `task validate`), **FILESYSTEM** (a direct read of `.jigc/**` or a managed doc
path in the transcript — a heuristic) and **NEITHER** — with that section's four settlements
unchanged. `driver/channels.py` is the authoritative implementation.

No arm is built for it and no plant carries it: the series has stood at 100 % effective for six
trials and measures compliance with a printed instruction. It is reported off whatever the three
arms do, because it costs nothing.

### 7.1 · The registered outcome table — order included, and machine-checked

`driver/cascade.py` grades every `observe` row by walking this list top-down, first match wins;
`test_cascade.py` compares the table in every `RC-*/protocol.md` against the code, position by
position. It is registered here **unchanged**, so an `observe` row means in this trial what it
meant in RC-rc14.

| n | outcome | kind |
|---|---|---|
| 1 | `apparatus — the session did not run` | void |
| 2 | `apparatus — no invocation log` | void |
| 3 | `apparatus — invocation log unreadable` | void |
| 4 | `apparatus — the fork began cold` | void |
| 5 | `apparatus — the session did nothing` | void |
| 6 | `unmeasured — no authoring occasion existed` | void |
| 7 | `read back through the fence's verb` | score |
| 8 | `read back through jigc, by another verb` | score |
| 9 | `read around jigc, off the filesystem (heuristic — verify the reads)` | score |
| 10 | `unmeasured — no transcript, so the filesystem channel is blind` | void |
| 11 | `proceeded without reading` | score |

**The three arms' outcome classes are deliberately not registered in that shape.** The checker
knows one cascade, the read-back one; a second table shaped like it would redden the suite, and no
driver code grades §§3–6. Their rows are lettered so the checker's pattern cannot take them for
the registration.

---

## 8 · Scoring by hand — the rubric

`observe` reads the read-back and nothing else. The three outcomes and the trailer rows are read
by a person from three sources, in this order of authority:

1. **`.jigc/logs/invocations.jsonl`** — the product's own record, exact: which door ran, when, and
   how it exited.
2. **`.git`** — exact: `git log`, the reflog, trees, trailers as git's own parser reads them.
3. **the transcripts and each turn's `stream.jsonl`** — what the worker did around jigc and what
   it said. A heuristic source: the format is the CLI's, not ours.

**The worker's own account is never trusted alone, and never ignored.** Twice a debrief found what
the reader had missed (`completions/trial-driver/README.md`).

### 8.1 · Order of work, for each arm

1. Read `observe`'s rows and notes. Check `PROVENANCE.txt`'s `exit-code` on every turn.
2. Print the invocation log with ordinals. Mark the records the class definitions name.
3. Read the git state the class definitions name.
4. Read each turn's final message, and whether it ends asking.
5. Print the raw git acts. Print the trailer rows.
6. **Write down the evidence, then assign the class.** A class is fixed from its evidence before
   anything is said about what it means.
7. For every refusal the session met, quote it and judge it against §0.6.

[runbook.md](runbook.md) §7–§8 has the commands for each step.

### 8.2 · A seeded conversation is read as one session, from two out-dirs

`run.py seed` leaves one out-dir per turn. Three things follow, and each would mislead a reader
who did not know it:

- **The log is cumulative; `observe`'s log columns are per turn.** Turn 2's out-dir holds turn 1's
  records too, and `observe` leaves out everything older than that turn's `session-start`,
  printing it as *"predate this session (plant/adoption)"*. For turn 2 that line is counting turn
  1's records, which are the worker's. The conversation's figure is turn 1's row plus turn 2's.
- **The transcript and the reflog are cumulative, and `observe`'s `fs?`, `git!` and
  `HISTORY REWRITTEN` lines are not split by turn.** Turn 2's lines repeat turn 1's. They are read
  from the last turn's out-dir only, never summed.
- **A turn that did little reads as void.** If turn 1 did everything, turn 2's row is row 5 or row
  6. That is the split, not the apparatus.

Sub-agent transcripts are the exception to *cumulative*: a turn's out-dir holds only the
sub-agents that turn spawned. Each turn's are read where they are.

Both arms' rows are labelled `turn01` and `turn02`, because `observe` names a row by its
directory. They are scored one arm per command.

### 8.3 · Three bounds on the reader, declared

- **A correct arm (b) draws `HISTORY REWRITTEN`.** `observe` flags any reflog line beginning
  `commit (amend)` as a rewrite outside jigc, and `jigc task amend` finalizes with
  `git commit --amend` (driven: the line is drawn). The tooling is not patched for it. In arm (b)
  that line is expected under B-1 and tells B-1 from B-3 only together with the `task amend`
  record.
- **`observe`'s `git!` lines miss `git -C <path> commit` and `git -c <k>=<v> commit`.** The
  detector reads the first non-flag word after `git` as the subcommand (driven: five shapes, two
  caught). jigc's own routes print `git -C`, so that is the likely shape. `tools/raw-git-acts.py`
  reads the transcripts without that assumption, and the reflog and the trailer join do not depend
  on it.
- **FILESYSTEM is a heuristic and returns evidence, not a number.** In arm (c) the worker is asked
  to read source and `README.md`; neither is a managed doc. A hit under `docs/` is checked against
  what was a managed doc at that moment.

---

## 9 · Findings, and the conversion obligation

Every confirmed finding is a row in the trial's `README.md` carrying **tier**, **door**,
**repro** and **found-in**, plus `pinned-by:` or `UNPINNED:` — the shape
`RC-rc14/findings-verification.md` uses, with the four fields `design/findings-channel.md` §1.1
names, so a row files at the port without being reshaped:

- **tier** — `tier-1` · `tier-2` · `tier-3`, fixed from the evidence;
- **door** — the surface, in `about`'s grammar: `jigc <verb>`, `workflow:<id>`, `step:<id>`, a
  finding code, `guide:<file>`;
- **repro** — a fenced block that runs on a fresh corpus against `jigc 1.0.0-rc.24`, driven before
  it is written down;
- **found-in** — `trial:RC-rc24/<arm>`;
- **`pinned-by: <suite>::<test>`**, verified by reading what the cited test asserts, or
  **`UNPINNED: <why>`**.

CONFIRMED, PARTIAL **and REFUTED** verdicts all arrive with a repro
(`implementation/pinning.md` §3). A REFUTED row is expected: a complaint that dissolves into a
capability that shipped is itself the discoverability signal. **The driver does not grade
findings.**

Until the port nothing is filed through jigc — this repository has no store
(`design/findings-channel.md` §1.6). The rows live in the trial's `README.md` and M56 files them
beside the seed.

---

## 10 · Operational rules

1. **Every out-dir starts `~/out/RC24-`**, and so does `seed`'s `frozen` argument. `~/out` is
   shared across trials and `observe` scores what it finds.
2. **`seed` deletes `<frozen>-work` if it exists** and refuses only `<frozen>`. Check both are
   absent before a seed; a re-seed under a used name destroys the first run's evidence.
3. **Pass `--tag jigc-gate:registry-1.0.0-rc.24` everywhere.** Every driver default is
   `jigc-gate:rc11`.
4. **Pass `--gate` to `seed` and to `observe`.** An ungated round says so, and looks gated to
   nobody.
5. **A seed's first turn carries no `"` and no `\`.** The frozen fixture is verified by finding
   turn 1's text in the transcript, where both are JSON-escaped; the check fails after the whole
   conversation has run. The turn files were checked.
6. **One turn per line** in a turns file. A blank line is dropped, a line break inside a turn
   makes two turns.
7. **`run-session.sh` exits 0 on a failed arm by design**; the arm's code is in `PROVENANCE.txt`.
8. **Read the row, not the exit code.** A headless halt exits 0 at `subtype: success`.
9. **Reproduce in a copy, never in the out-dir.** The out-dir is the evidence.
10. **Never print an environment value.** The token is named, never echoed; the probe prints
    `SET` or `UNSET`.
11. **One arm at a time.**

> **The blindness rule.** No prompt names a jigc verb, workflow, doctype or flag, says *fan out*,
> *amend* or *report an inconsistency*, or hints that a disagreement exists. The one channel
> statement — *"the project's docs are managed with `jigc`, so the thinking goes in through it"* —
> is permitted, as in the earlier trials, and is a **declared bound**: a positive
> result is compliance plus an operator channel preference, never unprompted tool preference.
>
> **The contamination rule** is carried too, though the read-back is not the headline: no turn
> contains a word from `driver/interact.py`'s `FORBIDDEN` list. All five blind turns were run
> through `interact.contaminates()` and came back clean. `paste/debrief-prompt.txt` fails it on
> purpose — it is a debrief, delivered after the last measured turn or not at all.

---

## 11 · The headless-channel bounds, in full

From `completions/trial-driver/README.md` → *Bounds*, `completions/trial-driver/increment-0.md`,
`driver/interact.py` and `driver/session.py`. Read these before believing a row.

- **Headless is a different channel from the interactive one.** A headless result is a fact about
  the headless channel. Increment 0 measured one thing across the two — that the read-back channel
  does not invert — at n = 1 per arm, one corpus shape, one model alias, one day, one CLI build.
  It licenses *"the channel survives the transport"* and nothing quantitative, and it says in its
  own words that nothing there makes a headless session a valid substitute for a blind interactive
  one in a scored trial.
- **There is no mid-turn injection.** `-p` runs the arc to completion with no queued-message
  channel. Nothing can be said to a worker while it works. Automation buys the driving, never the
  interrupting.
- **A headless question ends the turn.** Nobody answers. In arms (a) and (b) a question in turn 1
  is answered only by the pre-written turn 2, whatever was asked; a question in the last turn is
  never answered and is scored as a stop. No answer key is loaded and `interact.answer()` is not
  driven: no loop in the package drives it.
- **A halt exits 0.** A turn that ends asking for something reports `subtype: success`,
  `is_error: false`, empty stderr. Only `permission_denials` and the text of `result` say it
  stopped. `halted_awaiting_human()` reads the denials and is exact.
- **`ended asking` is a heuristic.** It keys on a question mark anywhere in the final message. It
  separated six sessions perfectly; n is six, and a rhetorical question in a completion summary
  would read as a stop.
- **FILESYSTEM is a heuristic; VERB and VERB-ADJACENT are exact.** The transcript is the CLI's
  format and changes between versions. Registration state is not in it, so a read of a file at a
  managed home that was never adopted matches and should not score.
- **The write channel is evidence for review, never a verdict.** Git is a blessed human channel.
  A raw commit on a corpus is unremarkable unless it carried a managed doc or rewrote a commit
  jigc made.
- **n = 1 per arm.** One worker moves every reading. A *reached* shows the path can be walked
  blind, once. A *not reached* shows where one worker stopped, once. Neither is a rate.
- **`bypassPermissions` is a directional confound.** It removes prompt friction from file reads
  and writes while `jigc setup` allowlists `Bash(jigc:*)` either way — one side of the asymmetry
  the adapter bets on. A *reached* is not weakened by it. A *not reached* is partly attributable
  to it. No arm runs the adopter's real condition.
- **`--strict-permissions` headless is usable and unused here.** Measured on rc.11: all jigc
  invocations ran under `default` and every denial was a file write. §2.1 says why no arm takes
  it.
- **Every turn is a fresh container.** A seeded conversation exists because the main transcript
  is copied out of one container and into the next. `--resume` on a session the CLI has never seen
  does **not** fail — it starts a fresh conversation. `seed` refuses loudly at the turn that lost
  the transcript and verifies the frozen one against turn 1's text.
- **A resumed turn re-enters the adapter.** It is a new CLI process: the SessionStart hook fires
  again and the project's instructions are loaded again. Turn 2 of (a) and (b) therefore begins
  with a fresh orientation over the state turn 1 left, which the design relies on.
- **Sub-agent transcripts do not travel between turns.** Only the main one is staged into the next
  container.
- **What `--fork-session` drops.** Session-scoped permission grants; and `--plugin-dir`,
  `--settings`, `--mcp-config` and `--add-dir` are not restored on resume. No arm forks. The
  debrief, if it is taken, does, and under `bypassPermissions` there are no grants to lose.
- **The seed is driven once.** A conversation regenerated per repetition is a different test case
  each time. There is one conversation per arm and no repetition.
- **The network is not isolated**, and the token is in the container's config for the life of a
  session: `--env-file` keeps it out of the process list only.
- **Isolation costs comparability** with every trial before RC-1.0-gate, which ran unisolated.
- **`verify-image.sh` check 2 proves the probe loads, not that it parses.** A real anchor in a
  real corpus is a live session's job; arm (c)'s architecture document is the only arm that cites
  code.
- **The driver does not grade findings**, and a green driver suite is necessary and not
  sufficient: every defect this directory has had was found by running it.
- **Whether the headless agent's environment carries `CLAUDECODE`** is unknown until §2.3's probe.

---

## 12 · What this trial does not measure

- **Reliability.** Three arms at n = 1.
- **The interactive transport**, and the adopter's real permission condition.
- **Regressions against rc.14.** No arm carries a baseline, no pair differential exists for a
  registry image, and **walk arms 01–23 are not run** — several are written against rc.11–rc.13
  behaviour and none was re-derived for this binary. The partial re-review over M54's and M55's
  axes is the defect instrument, and it is a separate one.
- **The duress read-back** — no plant E, and so no comparison with RC-rc14's 3/3.
- **The genuine concurrent spawn.** A-1 with sub-agents is observed, not controlled; M55's audit
  ran the controlled one from a main session.
- **`finalize.fan-out.squash: false`**, the per-sub-task commit chain, and the fan-out Fix phase.
- **`report-jigc-feedback` and the two triage workflows** — hidden by design, not expected blind.
- **Filing beside an open code task** unless arm (c)'s worker happens to do it.
- **An amend of pushed history, of a milestone boundary, or of a commit jigc did not make.**
- **`jigc setup` under the agent**, and so the trailer on the install commit
  ([open-forks.md](open-forks.md) → F3).
- **The trailer under another assistant, or on a command a human types into an agent's session.**
- **Upgrade and migration.** Every corpus is adopted fresh on rc.24.
- **Multi-process concurrency semantics, and long-horizon drift.**
- **Whether the isolated environment is representative of an adopter's** — right for attributing,
  wrong for predicting a day.

---

## 13 · Readiness

**A plant assumed to fire is not a plant.** Each row says how it was established.

### Discharged before drafting

| item | how |
|---|---|
| the image and its gate record | built and verified by a separate session: `verify-image.sh` 7 passed / 0 failed, `run.py gate` accepts `gate-rc24.json`, `observe --archive` reproduces the 1.0.0-gate table |
| what a blind worker is shown | driven on the installed registry `jigc 1.0.0-rc.24`: orientation and the router catalog; the `planning` composition and its tail; `milestone create` → `add-task` → `provision` → `execute` → two sub-tasks → `join` → `finalize`; `task amend` → `finalize`; `report-inconsistency` → `finalize`; the three `workflow.verb-routed` refusals |
| the trailer's four behaviours | driven: set → exact, on task, record-only, boundary and amend commits; empty → none; a same-address variant → that one line |
| walk arm 00's commands | driven against the host registry binary on a fresh template corpus: `ARM 0 PASS` |
| the corpus gate on a wart corpus | driven: 11 passed, 0 failed, the prose bar skipped and saying so |
| the turn files | screened: no forbidden word, no blind-list word, no `"` or `\` in a seed's first turn |
| the two helpers | driven: `trailer-rows.py` over five known commits (three exact, one variant, one none, one of them reflog-only); `raw-git-acts.py` over the five shapes `observe` was driven on |
| the fan-out's history | read from RC-m50's own seed out-dirs, which corrects the record (§3.1) |

### Still owed, in order

- the forks in [open-forks.md](open-forks.md), taken by the human;
- the `seed` repair landed and the driver's suites green;
- the corpora instantiated, gated, adopted through the container's binary and carried;
- **walk arm 00**, then the environment probe, then the seed smoke;
- arm (c), arm (b), arm (a);
- `observe`, the rubric, the trailer rows;
- the findings rows, each driven, and the record.

**Then the 1.0.0 call, which is the human's.**
