# The reshaped `test` stage, run for real in parts — plan

**Approved by the human on 2026-10-12, with option A for the review before part 1** ([DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-12 — The reshaped `test` stage is run for real in parts*, rulings 1 and 2). It plans for the human's ruling of that day — the tool is run for real in parts, now, instead of one whole canary at the end — and from that day it is a plan the build is held to, beside the build plan it cuts across: [plan.md](plan.md), in this directory (below: **the reshape plan**; its section and task names — §9, `T7`, `K1b` — are used as they stand there). Option A answers the one deviation §10 puts to the human, at `R0`: before part 1 a `milestone-code-reviewer` reads only the code the parts run, for the three dangers, and the review of the build's whole diff stays at the join. The words of §10 and §14 that call it *the human's to overrule* stand as they were put; this paragraph is where the answer is.

*Drafted 2026-10-12 against the committed tree at `e619cee8` on `fix/rc24-tier1`, and revised the same day against an independent review and its spike (§14 is the disposition, finding by finding). The branch's tip was `72ca156a` when the plan was approved; that commit touches none of the five tools, so every line number below names a line of a file as those two commits hold it — and what the text says of the build's own state, `T7` being built among it, is that day's. Still a plan: nothing of it is built, no gate, test or stage was run for it, and it restates no decision. What was read, what the review's spike drove and what was only inferred is said at the claim, and gathered in §13.*

**[Noted 2026-10-12, after `T7` landed (`b717fb96`): the second of this plan's three weakest points is settled, and for the better. An order ruled while `next` is `stop` is taken and recorded, starts nothing, puts the listed rows into the queue, and **no `go` is owed** — the invocation after it continues the stage (the simulation's `an_order_ruled_while_the_run_is_stopped_after_a_round_reopens_its_queue_and_no_go_is_owed`; [DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-12 — The harness: what a stage returns, and the rulings about the run*). So part 3 opens as §5 has it, with no step added. `T7` also built what §10 asked of it otherwise: `budget` and `order` rulings, `hand-on` refused before any agent, and an order and a raise in one invocation as one record of two calls — which no part uses. What a stage returns is named there; where §5 and §8 say what a return holds, that entry is what holds.]**

**[Noted 2026-10-13, after the review before part 1 (`R0`) — three reviewers, each with files of its own, for the three dangers only; what they found and what was repaired is in [DECISIONS.md](../../../../../DECISIONS.md), the three entries of the repairs and the entry of `SC3`. Six things in this plan read otherwise since. (1) **Part 4, act 5**: the gate itself keeps running, as a held command does: the human asks `dev/stabilize-canary check --root <root>` until it says, under `held`, that the job `record-…` has `ended` — its supervisor has written `exit.json`. No `verdict.json` appears while nobody asks after the job, and none is waited for: the invocation that follows gates the batch again under a name of its own and reads nothing of the first gate. (2) **A `check` made while an act is running may print `next: rule` and `attempts-spent`**: while a second attempt in a row without a record is under way the state cannot tell a live attempt from a dead one. That is so during part 2 and during part 4's act 4, and it is no finding. **No ruling is typed on a `check` made while an act runs**, and no `check` is made while a stage is committing. (3) **The root is made under the machine's temporary directory or under the home directory**: a path of the machine outside the repository, the scratch roots, the temporary trees and a home directory is written into a record as it stands. (4) **The plants' keys are not the ones §4 names**: a planted row's key says the door and the subject of its claim and never how the claim falls; the mapping is `SC3`'s, printed by `check` under `sheet` to the terminal only. §4's, §5's and §9's names are kept here as the names of the plants, not of their rows. (5) **Nobody opens the run's `verdicts` or `reports` directory with a file browser**: until the repairs a stray file there refused every record, and a listing in a terminal leaves none. (6) **How many agents an act launches is what the setup's line prints for it under `expect`**, held to the clone's own harness by a drive under the stand-in runtime; where §5 or §10 names another number — part 4's act 1 as *one executor, one gate* among them — the line's is the one.]**

**[Noted 2026-10-13, after the second review before part 1 (`R1`) — one reviewer over the tool `SC3` built and the three repairs after it, for the three dangers only; what it found in the setup tool and what was repaired is in [DECISIONS.md](../../../../../DECISIONS.md), the entry *The canary's setup refuses a commit whose tool is not the one running, and `check` says what is so*. Six things in this plan read otherwise since. (1) **§5's *the commit that holds `SC3`*** is read as **the commit that holds the repairs after it — the tip of the branch when the setup is run**, by its full sha: the setup now refuses (`unfit`), before anything is made, a commit whose `dev/stabilize-canary` is not the file that runs, and its refusal names the commit to name. (2) **Of a batch that is applied and in no commit** — the state between act 4 ended at its record's gate and act 5 — `check` names the batch under `applied`, with every field it read across it (`across`) and the one call that takes it back, prints each filed verdict the batch takes as `recorded: false`, `applied: true`, and counts the batch as the record it will be where it says whether the run shows an act. **Of a held command** it says what the step tool would answer — `running`, `done`, or `dead` with its reason, and the kept verdict as `kept`: the wait before act 5 is until `done` or `dead` — the word of note (1) above, `ended`, is `done` now — and where it says `dead` the wait is over and nothing is ended. (3) **Part 1's second regression item reads `baseline.used: false` where the first run ended void**: since the repair of the regression tool a void run keeps no baseline, so the second builds its own; that is no finding where §5's table expects `true`. (4) **Act 3's instruction to list processes by the root's path and end one by its number is for act 3 only**: after act 4 the same list holds the supervisor of the record's gate, and nothing is ended there. (5) **The root is made with `mktemp -d` under the machine's temporary directory**, and not under an agent session's scratch directory: a root whose path embeds the home directory's makes a report that names a path under it refused as a host path. (6) **`check`'s output and every return file hold absolute paths of the machine**: none is pasted into a committed record — and `check`'s output is not redirected into a file under the root: it ends with the answer sheet, and the scratch root every agent works under lies there. And off the main line: the ruling that grants one more attempt, and the one more raise after a missed window, each have a prompt file under `<root>/prompts/` (`ruling-…`), which `check` names where it is the step — never typed while a session of an act is open.]**

**[Noted 2026-10-15, after part 2: it halted at its first record, and the root it ran in is ended.** The record was refused `hygiene`: the regression tool named the sha256 of its kept baseline's key `key`, the record carries the tool's line, and gitleaks reads a `key` beside a 64-hex value as a credential — so no record of a run that holds a regression item could be composed, and the state would have gone to `attempts-spent` after a second such halt. The tool now names it `sha256`, a halt of that kind names each file and rule, and a suite composes a record from the tool's own line and scans it with the machine's gitleaks ([DECISIONS.md](../../../../../DECISIONS.md) → *The regression tool names its kept baseline's hash `sha256`, never `key`*). (1) **The canary starts again from a fresh root of the repaired commit** (§9): the held verdicts in the part-2 root were written by the old tool and still carry `key`, so a record composed from them is refused again, rightly, and nothing in that root is carried over. (2) **§5's part-1 table reads `baseline: { used: false, sha256: <a sha256> }`** for the first regression since. (3) A record the public-hygiene scan stops now halts saying so, with each file by its path and the rule, where it said *a word of `dev/stabilize-record` this script has no row for*.]

**Words this plan uses.** *The harness* is `.claude/workflows/stabilize.js`; *the record script* `dev/stabilize-record`; *the step tool* `dev/stabilize-step`; *the setup tool* `dev/stabilize-canary`. *A root* is the one directory a setup makes outside the repository: a bare remote, a clone, a scratch root and a results directory. *A part* is one or more invocations of the harness, made by the human in a session of the human's own, that end at a named mechanism and are read before the next is made; *an act* is one such invocation, or one thing the human does between two. *A plant* is a ledger row the setup seeds whose right outcome is known beforehand. *A reserve plant* is a small false claim planted to fill the queue cheaply — a term of this plan. *A run with a queue* is the reshape plan's: a run whose opening holds an order of classes. *The spike* is the review's drive of the committed record script through the planned opening, in a throwaway repository.

---

## 1. In one page

| Part | Starts from | What the human invokes | How it ends | What it shows |
|---|---|---|---|---|
| **1 — the prelude** | a fresh root | the stage, with the tuning stop `preflight` | a stop that exists; no record is made | no permission prompt at a tool step; the two builds, the gate and the regression set held by commit; a baseline kept — and used, by a second regression item |
| **2 — the hunt and the first record** | the root part 1 left | the stage, with no stop | **the stage's own return:** the opening's order names no class a row can have yet, so the queue is empty and no verifier is launched | one triage that types its entries once; a record composed by the tool, no executor; the stage's passage from its first record to its return |
| **3 — verification** | the root part 2 left | a ruling of the order; then the stage, which continues | the budget of 9, spent — eight verdicts, then one — and the second record | an order ruled at a stop; verifiers file their own verdicts; at most two follow-ups enter the queue; a handed-on item verified ahead of an older row |
| **4 — the interruption** | the root part 3 left | a raise of the budget; the stage, ended by the human mid-wave; a look; the stage again | the human's kill, then the continuation's record | no instrument run again; the filed verdicts kept; only what has no verdict launched |
| later | the whole canary of §9 | — | — | the route hunter, the scripted arm, the trial image — and one stage that is not cut into parts |

**Built first, in this order:** `T7` (being built, to the corrections of §10) → `P0`, the plants' words driven once → `SC2`, the setup's opening with a queue and its plants → `SC3`, the setup's parts, its prompt files and its `check` → `R0`, a review of the code the parts run, for three dangers only. **No new stop is built.**

**Expected cost of the four parts together:** about 106 agents, 4.9 to 6.8 hours of run time, 0.85 to 1.35 M output tokens — about 44 to 70 USD at the first canary's rate — and about two and a half hours of the human's own attention. **That is an addition:** the whole canary stays owed after them (§7, §11).

**The three weakest points of this plan** (§11 has the rest): (1) parts 3 and 4 end where the plan says only if every verifier that is launched files a verdict, and the queue they run on is ten reserve plants that nobody has driven; (2) part 2 most likely ends at the stop after every round, and that an order ruled **there** re-opens the stage is read from the two scripts — the spike drove the record script's half, at another step — while the arm sent to `T7`'s builder names `next: rule`; (3) a repair between two parts changes the tool's commit, so the next part starts from a fresh root and pays 25 to 50 minutes again.

---

## 2. The facts the cut rests on

The review found rows A1 to A14 true as written, and they may be cited as checked.

| # | Fact | Where |
|---|---|---|
| A1 | The `test` stage has four stops, `state`, `preflight`, `instruments`, `triage`. **No record step runs at any of them.** Stopped after `state`, an invocation begins no attempt; stopped after a later one, its attempt counts as one that reached no record, and after two such attempts in a row the next is the human's to grant | `HARN` 661; 620–631; `REC` 6079 |
| A2 | The scope step runs **beside** the first preflight and the builds; the stop `preflight` returns after the builds, the candidate's gate, every held check, the scope step and — where an item needs the trial image — the second preflight | `HARN` 4356–4372, 4403–4433 |
| A3 | On the path of a run with a queue the stop `triage` returns **before** the first record; after the first record's read-back the stage goes straight on to the verify phase. No stop stands between a stage's first record and its first wave | `HARN` 4524, 4531–4543 |
| A4 | A wave is the first keys of the queue, **at most eight and never more than the budget has left**; a key is launched once an invocation; a wave with no key to launch ends the phase; after a first record a second record is composed only where a verifier was launched | `HARN` 3826–3836, 3918 |
| A5 | A budget and a hand-on number are **positive whole numbers**; the hand-on number is **written once**. A budget is **raised only at its own stop** — while it is spent and the queue holds a row nobody reached — and never lowered. **Another order is taken at any step.** An order and a raise in one call are judged against the state before the call, and refused whole | `REC` 788–825; the spike |
| A6 | The harness knows six rulings about the run — `go`, `rerun`, `rounds`, `cycles`, `reverify`, `again` — and none for a budget or an order. An invocation that carries rulings is taken **before the stage's position is asked**, records them, and starts nothing. A rulings record keeps its executor and its full gate | `HARN` 1138–1160, 4288–4290, 4090–4131; the reshape plan, `K3` |
| A7 | An invocation that finds the stage unfinished **continues** it: no preflight, no scope step, no gate, no held check, no reporter of an item; the two binaries asked for by commit and answered by the builds that stand. It takes no stop but `state`, which only looks | `HARN` 3965–4022, 1613–1616 |
| A8 | The two builds, the candidate's gate and a held check are **named by commit** under the scratch root. A job that stands is the answer **only while it runs or where it is green**; one that ended red, void or dead is passed over and started again under `<name>-2`. What a stage keeps under the scratch root beside them is named by **round and attempt, and by no run** | `HARN` 3322–3334, 3357–3359, 3382–3383, 4367, 4409; 1527, 1574, 4531 |
| A9 | The kept baseline lies at `<scratch>/baseline/previous-<12 digits>`; **a kept baseline is used where it was built**, and a held regression that stands green is not started again | `HARN` 724–731; `dev/regression-set` 186–192 |
| A10 | The setup tool opens a run with **no order, no budget and no hand-on number**, on the door `jigc doc list`, with **one seeded row, ungraded and without a class**, and with the stop mode *after every round*; its scratch root is `<root>/scratch`, and a root is used once | `CAN` 297, 306–364, 574–585, 626–636, 669, 691–693 |
| A11 | A round that names no candidate is not tested, and its next step is `test` whatever the queue holds | `REC` 5898–5904, 6089–6094 |
| A12 | **Only the verifier's prompt names the lookup kit** as its first read; of the definitions, the triage's and the verifier's name it | `HARN` 2461; `.claude/agents/finding-triage.md`, `.claude/agents/finding-verifier.md` |
| A13 | The first canary's session printed that it ignored the three committed allow rules because the clone's workspace had not been trusted, and said how a clone is trusted: *run the session interactively there once and accept the trust dialog*. It ran with permissions bypassed | [canary-one/evidence/session.out](../canary-one/evidence/session.out), line 2; [canary-one/README.md](../canary-one/README.md) |
| A14 | Two arms of the tooling suites went red in a full gate while two lanes' gates ran side by side, and passed alone. A day earlier thirty tests of the product's suites had run into the runner's bound in one full gate on a loaded machine, all passing alone | `DECISIONS.md`: the entry of 2026-10-12 on a held command's files; the dated note of 2026-10-11 above the entry on a verdict filed when it returns |
| A15 | **With an empty queue the phase launches nobody, no second record is made, and the stage returns `hunted`.** The next step is then `rule` where a row is the human's, and `stop` where the round is over and the run stops after every round | `HARN` 3836, 3918, 4559; `REC` 5860–5866, 5922–5926, 6096–6113 |
| A16 | **The first N items of a filing that carry their kind's evidence are handed on**, N the opening's number, in the verifier's own order — whatever their kind; a lead takes no place. A row whose class the order does not name is listed and never launched | `REC` 524–528, 2522–2531 |
| A17 | The queue is ranked by where a row's class stands on the order, ties by the ledger's order, and by nothing else | `REC` 5579–5592 |
| A18 | A plant found again by triage **takes triage's grade and keeps its class**; triage is told to use an existing row's key for *the same door and the same broken behaviour* | `REC` 394–407, 598–603; `.claude/agents/finding-triage.md`, *Found again* |
| A19 | An invocation that only looks returns at its first read where anything but a filed verdict is owed — a killed write among it — and does not read the state across it | `HARN` 4282 |
| A20 | **Driven by the spike on the committed record script:** a budget of 0 refused; the opening takes a budget, a hand-on number and an order; two held regression items taken in one test set; a row without a kind refused; eight plants graded `unclear` by the human taken, with their classes; the next step `test`, nothing untriaged; a lead and a class off the order `listed`; an order alone taken, which puts the listed class into the queue; a raise refused while the budget is not spent; a second hand-on number refused `exists` | the spike's output |

---

## 3. Where the ruled table cannot be built as worded

Nine places. Each names the lines that show it and the nearest thing that can be built. The review found the first eight true as first written; the fifth and the eighth are answered differently now, and the ninth is new.

1. **Part 1, "the two builds, the gate, the regression set" — and nothing else — is no stop of the harness.** The scope step runs beside them, and the preflight's assert before them (A2). *Nearest:* the tuning stop `preflight`, which exists.
2. **Part 1 ends with nothing on record.** That stop runs no record step (A1): it leaves four files uncommitted in the clone — the attempt's marker, the preflight's report, the scope step's report and the round's scope (`HARN` 4463–4464) — and one of the two attempts a stage may spend without a record is spent. *Nearest:* that is the documented state the next attempt starts from (`implementation/stabilization-workflow.md` → *Halt and resume*).
3. **Part 1, "the kept baseline": one stage shows a baseline kept, never one used** (A8, A9, A10). *Nearest:* **a second held regression item**, which the stage starts after the first under a name of its own and with the same baseline. About eleven minutes and two tool steps. The spike took two such items in one test set.
4. **Part 1, "no permission prompt": a clone is an untrusted workspace** (A13), and the preflight's and the scope step's own commands are no calls of the three tools. *Nearest:* part 1 shows that no **tool step** prompts, in a session that asks and that was trusted first (§6).
5. **Part 2, "the first record — no verifier": no stop stands there** (A3), and a budget of nought is refused (A5, A20). *Nearest, and built of nothing new:* **the opening's order names no class a row can have before a verdict.** The queue is then empty, the stage launches nobody and returns as it hunted (A15). Part 3 opens with a ruling of the order.
6. **No part runs a stage's passage from its first record into a first wave inside one invocation** (`HARN` 4543). Part 2 runs the passage from the first record to the stage's own return; part 3's waves are a continuation's.
7. **Part 3, "verification in waves over the planted findings": today's opening plants one row, ungraded and without a class** (A10). The plants are graded at the opening with their classes, as §9 rules, and the opening holds an order.
8. **Part 4 needs rows nobody verified and budget to spend on them.** A budget is raised only where it is spent with rows unreached (A5), and the harness cannot record a budget or an order yet (A6). *Nearest:* `T7` first; and a queue that is **surely longer than part 3's budget**, so that part 3 always ends with the budget spent and rows unreached, and part 4 always opens with a raise alone (§4).
9. **"In waves" cannot be had under a budget of eight or less.** A wave takes everything the budget has left, up to eight (A4): an invocation that starts with eight or less to spend has one wave, unless a verifier leaves no verdict. *Nearest:* part 3's budget is **nine** — a wave of eight, the queue read again, a wave of one. That is why the queue holds thirteen rows and ten of them are reserve plants.

---

## 4. The shape

**One root, continued from part to part; a fresh root only after a repair.** The four parts are four stretches of **one** `test` stage of one synthetic run. Nothing is built or gated twice while the root stands and its held jobs ended green (A8); the scope stands; and after part 2 the round has its candidate on record, which every continuation builds from (A7).

**A repair between two parts ends the root.** A clone holds the commit it was made from, and the harness is invoked by its path inside the clone. §9 says what a fresh root pays.

### The opening the setup writes (`SC2`)

- **The run:** `canary-parts`. Neither a directory nor a loop branch of that name is in the repository at `72ca156a`.
- **One door:** `jigc setup`. No second door, no route hunt, no trial arm: those are the *later* row.
- **Items:** `row-setup` (a review row over that door: a source pass, a driver, a reconciler), `gate` (held), `regression-set` (held), `regression-again` (held, the second regression of §3.3).
- **Facts:** budget **9**, hand-on **2**, the condition's heading (the entry of 2026-10-04 that states the exit rule; one heading in the log), and **the order `route under-confirmed`, `behaviour under-confirmed`** — two classes only a verifier's filing can give a row, so that nothing is in the queue before part 3 rules another order.

### The three orders, and what each admits

| When | The order | What is in the queue |
|---|---|---|
| the opening; parts 1 and 2 | `route under-confirmed` · `behaviour under-confirmed` | nothing: every plant and every row of the review is listed as off the order (A16) |
| part 3, ruled at its start | `route under-confirmed` · `route seed` · `behaviour under-confirmed` · `behaviour seed` | the three route plants, then the ten reserve plants: thirteen rows |
| part 4 | the same — **no order is ruled; only the budget is raised** | what part 3 did not reach |

**The hunt's classes stand on no order of the parts.** The parts have no hunter, and a row of the review is listed in all four. A handed-on behaviour item is on part 3's order, so it is launched like a handed-on route item and never left listed.

### The plants

Every plant is a row of round 0, graded `unclear` by the human at the opening, with its class and its kind's evidence — so that no triage decides whether it is launched. `<rig>` is a fresh repository with one commit that `jigc setup` never ran in, as `dev/jigc-rig bare` makes one; `BINARY` is the binary under test.

| Key | The claim | Its evidence — the route line, or the commands | Class | Source | What must happen, and where |
|---|---|---|---|---|---|
| `plant-install-hook-regression` | where `.git/hooks/pre-commit` is a link to an existing script outside the repository, `jigc setup` refuses at `setup.install-hook` and its route names the script as the link; done as printed, the script is deleted and the re-run exits 1 again; the previous release exits 0 | route: ``remove the link at `<rig>/outside/shared-hook`, or point it at a script inside this repository, then re-run `jigc setup` `` · code `setup.install-hook` · repro: the block *Repro VR-1* of the first canary's report [verify-p2-r1-setup-install-hook-route-other-causes-unexamined.a1.md](../canary-one/run/r1/reports/test/verify-p2-r1-setup-install-hook-route-other-causes-unexamined.a1.md) | `route seed` | on record, word for word: [canary-one/findings.md](../canary-one/findings.md), the row `r1-setup-install-hook-route-other-causes-unexamined`, and that report. `git diff --stat 7e1b34a1 e619cee8 -- crates/cli/src crates/engine/src` is empty | **part 3, wave 1: confirmed, a regression.** What its verifier hands on is in the queue; the first of it is wave 2 |
| `plant-false-route-claim` | where the default hooks directory is of mode 555, `jigc setup` refuses at `setup.install-hook`; done as its route prints — the directory made writable — the re-run exits 1 again | route: ``ensure the repo's git hooks directory is writable, then re-run `jigc setup` `` · code `setup.install-hook` | `route seed` | the claim is this plan's; **that it is false is on record**: the same report's table has that cause's re-run at exit 0 on both binaries. The route line is the report's quoted stderr | **part 3, wave 1: refuted as *does not reproduce*.** What its verifier hands on is never launched |
| `plant-linux-root-claim` | on a Linux host, for a caller that is root, a hooks directory carrying the immutable attribute draws the same refusal; the route as printed changes nothing for root, and the re-run exits 1 again | the same route line and code | `route seed` | this plan's, after the three findings of [canary-one/findings.md](../canary-one/findings.md) §5 whose verifiers halted for want of a Linux host or a root caller. Driven by nobody, and not to be | **part 3, wave 1: `undrivable`.** On the human's list. **No verifier for it in any later act** |
| `plant-prose-lead` | `jigc setup` may behave otherwise in a repository with several worktrees; nobody looked | none | `lead seed` | this plan's | no verifier, ever; on the leads list in every part |
| `plant-reserve-1` | in `<rig>`, `jigc setup` exits 1 | `BINARY setup` in `<rig>`; the exit status | `behaviour seed` | false on record: the same report's control — *exits 0 on both binaries, one install commit, porcelain empty* | **part 3, wave 1: refuted** |
| `plant-reserve-2` | in `<rig>`, `jigc setup` exits 0 and makes no commit | `git rev-parse HEAD` before and after `BINARY setup` | `behaviour seed` | false on record: the same control | part 3, wave 1: refuted |
| `plant-reserve-3` | in `<rig>`, `jigc setup` leaves `git status --porcelain` not empty | `BINARY setup`; `git status --porcelain` | `behaviour seed` | false on record: the same control | part 3, wave 1: refuted |
| `plant-reserve-4` | in `<rig>`, `jigc setup` makes two commits | `git rev-list --count HEAD` before and after | `behaviour seed` | false on record: the same control | part 3, wave 1: refuted |
| `plant-reserve-5` | an untracked file `notes.md` at the root of `<rig>` is gone after `jigc setup` | write it; `BINARY setup`; read it | `behaviour seed` | this plan's — **`P0` drives it** | part 3, wave 1: refuted |
| `plant-reserve-6` | a second `jigc setup`, right after the first, exits 1 | `BINARY setup` twice; the second exit status | `behaviour seed` | this plan's — `P0` | part 3, wave 2 where nothing was handed on; else part 4 |
| `plant-reserve-7` | a second `jigc setup` makes a second commit | `git rev-list --count HEAD` after each | `behaviour seed` | this plan's — `P0` | part 4 |
| `plant-reserve-8` | `jigc setup` leaves another branch checked out | `git branch --show-current` before and after | `behaviour seed` | this plan's — `P0` | part 4 |
| `plant-reserve-9` | `jigc setup` adds a remote | `git remote` before and after | `behaviour seed` | this plan's — `P0` | part 4 |
| `plant-reserve-10` | `jigc setup` leaves a stash entry | `git stash list` after | `behaviour seed` | this plan's — `P0` | part 4, or left unreached |

**Two spares,** for a reserve plant that `P0` finds true or unclear: *`jigc setup` creates a tag* (`git tag` before and after); *`jigc setup` writes `user.name` into the repository's own configuration* (`git config --local --list` before and after).

**The reserve plants are ballast, and the plan says so:** ten dull claims, each refuted by one drive, whose only work is to make the queue surely longer than the budget. The first canary's one seeded false claim cost its verifier 12 k output tokens and 2.5 minutes — one observation, on another door.

### What each wave holds — worked through the queue's rank and the wave's size

**Part 3**, after its order is ruled. The queue is thirteen rows; the budget has nine left.

- **Wave 1 — always the same eight:** `plant-install-hook-regression`, `plant-false-route-claim`, `plant-linux-root-claim`, `plant-reserve-1` to `-5`. Eight is the cap of a wave (A4). One verdict of the budget is left.
- **What can enter the queue between the waves:** only what the regression plant's verifier hands on — its first two items that carry their kind's evidence, of either kind (A16). The refuted plants' items are under a parent that did not reproduce, and are never queued; the undrivable plant's verifier hands on nothing.
- **Wave 2 — one key, the first of the queue as it then stands** (A17):

| What the regression plant's verifier handed on | Wave 2 | Left unreached when part 3 ends |
|---|---|---|
| nothing that carries evidence | `plant-reserve-6` | `plant-reserve-7` to `-10` |
| one route item | that item | `plant-reserve-6` to `-10` |
| two route items | the first of them, in the verifier's order | the second; `plant-reserve-6` to `-10` |
| a route item and a behaviour item | the route item | the behaviour item; `plant-reserve-6` to `-10` |
| behaviour items only | the first of them | the second, where there is one; `plant-reserve-6` to `-10` |

**In every case part 3 ends with nine of nine spent and at least four rows nobody reached** — provided each of the nine verifiers files a verdict. The next step is `rule`: the undrivable plant is the human's.

**A handed-on item verified ahead of an older row of a lower class is shown in part 3's wave 2**, in every case but the first: the handed-on item gets the budget's last verdict and `plant-reserve-6`, a row of the opening, gets none. Where nothing is handed on there is nothing to overtake, in any part.

**Part 4**, after the budget is raised to **13**: four are left, so the wave is the first four of the queue — the route items handed on under a confirmed finding, then the behaviour items so handed on, then the reserve in the ledger's order.

| What part 3 left | The wave of part 4's second act |
|---|---|
| `plant-reserve-7` to `-10` | those four: the whole queue |
| `plant-reserve-6` to `-10`, the handed-on item of wave 2 refuted | `plant-reserve-6` to `-9` |
| the same, the handed-on item of wave 2 confirmed and handing on again | its one or two items, then `plant-reserve-6` onward, to four |
| a second handed-on item, and `plant-reserve-6` to `-10` | that item first — ahead of the reserve again — then what the first handed on, then the reserve, to four |

---

## 5. What the human does, and the parts

### Once, before part 1 — from the repository's root, in a plain terminal, no agent

```text
mktemp -d <a scratch directory outside the repository>/canary.XXXXXX          → prints <root>
dev/stabilize-canary setup --root <root> --commit <the full sha of the commit that holds SC3> --run canary-parts --previous <version> --previous-commit <sha> --parts
dev/stabilize-canary check --root <root>
```

`<version>` and `<sha>` are the previous release and its commit, as the first canary's setup line holds them under `previous`. `--parts` is `SC2`'s switch; its name is the builder's. The setup's line names, for each act, **a prompt file under `<root>/prompts/`** and a return file under `<root>/results/`.

### Once, before part 1 — the clone is trusted

A terminal in the clone, `<root>/clone`. One interactive session is started there and its trust dialog accepted, as the first canary's own output says a clone is trusted (A13). **Right:** the session's first lines do not say that it ignores allow entries. Then that session is left.

### For every act that invokes the stage

1. `dev/stabilize-canary check --root <root>` from the repository — it asserts once more that the clone reaches nothing but its bare remote, and prints the attempt the next invocation will begin.
2. A session in the clone; the act's prompt file is opened and **its text pasted as the session's one message**. The text says one thing: invoke the Workflow tool once with the script's path and the act's arguments, write what it returned as JSON to the act's return file, and do nothing else.
3. When the session says the return is written, the session is left.
4. `check` again, and the reading of §8.

**No act is started while a full gate of the build runs on the same machine** (A14).

### Part 1 — the prelude

- **Starts from:** the fresh root.
- **Invocation:** `{ stage: 'test', run: 'canary-parts', scratch: <scratch>, scope: { doors: ['jigc setup'] }, stopAfter: 'preflight' }`.
- **Ends by:** the tuning stop `preflight`, which exists. Return: `status: 'stopped'`, `after: 'preflight'`.
- **Leaves:** under `<scratch>` the two binaries, the gate's job, the two regression jobs and the kept baseline; in the clone four uncommitted files, all under the run's `r1/`. No commit, no push. One attempt without a record is spent.
- **Reuses:** nothing. **Pays again:** nothing.

| Checked | Read from | Right |
|---|---|---|
| no prompt at a tool step | the human's note at the session; the transcripts | none |
| every command of a tool-step agent | the transcripts | each one `dev/stabilize-step …` line, or the structured return |
| the two builds, by commit | the return; `<scratch>/hold/build-candidate-<12>/verdict.json`, `build-previous-<12>` | both green; the hashes the return names |
| the gate, by commit | the return; `<scratch>/hold/gate-<12>/verdict.json` | green — or red with its steps and tests named, which is a fact about the candidate and ends nothing (see part 2) |
| the first regression | the return; its job's `verdict.json`, the tool's own line in it | a verdict; `baseline: { used: false, sha256: <a sha256> }` |
| the second regression | the same, for `regression-again` | the same verdict; `baseline: { used: true }`; its `seconds` about half the first's |
| the round's scope names the door | `check`: the items | `row-setup` selected |
| nothing on record, nothing pushed | `check` | 0 records; pushed; 4 pending lines, every one under the run's `r1/` |

**Watch, and when to interrupt.**
- *A prompt at the first tool step (`git:state`), within the first minute:* stop. It is the finding; the part has cost one agent.
- *A prompt at a command of the preflight or of the scope step, at a file the scope step writes under the scratch root, or at the session's write of the return file:* note the agent and what it asked for, answer it, go on (§6).
- *The gate past forty minutes, or a regression set past forty* (23.3 and 22.3 on the first canary): note it; a held command is ended by nothing but a signal by hand (the row `wf-hold-no-stop`).
- *A red gate whose red tests pass alone:* the machine was loaded. The root is given up and the part run again under a fresh one.

### Part 2 — the hunt and the first record

- **Starts from:** the root part 1 left. *(After a repair: a fresh root — §9.)*
- **Invocation:** `{ stage: 'test', run: 'canary-parts', scratch: <scratch>, scope: { doors: ['jigc setup'] } }` — the stage as it is invoked today; nothing new.
- **Ends by:** the stage's own return. The first record is composed, gated, committed, pushed and read back; the queue is empty; no verifier is launched and no second record made (A15). Return: `status: 'hunted'`, with `next: 'stop'` (the round is over, and the run stops after every round) or `next: 'rule'` (triage graded a row of the review as needing a bound).
- **Reuses:** every held job of part 1 that ended green — one start step each, answered at once — and the scope.
- **Pays again:** the three opening steps; the preflight's assert; **the scope step's agent**, which is launched again and finds the scope standing (`HARN` 4371; its cost there is nobody's measurement); and **every held job that part 1 left red**, started again under `<name>-2`.
- **What it still does not run:** the passage from a first record into a first wave inside one invocation (§3.6).

| Checked | Read from | Right |
|---|---|---|
| nothing built, gated or checked twice | the agents' labels | a `hold-start` for each of the five jobs and **no `hold-wait:build-…`, `hold-wait:gate-<12>` or `hold-wait:regression-…`** — except for a job part 1 left red, which shows once as `…-2`. **One wait is expected:** `hold-wait:record-…`, the record's own gate, about 22 minutes |
| three reporters of the review row, and no other instrument | the labels | `row-setup:source`, `:driver`, `:reconciler` |
| **one** triage | the labels | one |
| triage typed its entries once | the triage's transcript | one write of its entries file; one `dev/stabilize-record triage …` call; a return of a path, a hash and the counts — no entry in it |
| triage was handed no plant | the triage's prompt | its sources are the three reporters, and a red check if one is red; no source of rows nobody graded (the spike: nothing untriaged at this opening's neighbour) |
| the record composed by the tool | the agents | no agent of the executor's type; one composing step — or two, where its words came back altered: **count it** |
| the first record is on the branch and pushed | `check` | 1 record; pushed; nothing pending |
| no verifier, no second record | the labels; the run's directory; the return | no `verify:` label; no file under `r1/verdicts/`; `verify` in the return says no wave and no record |
| the state after it | `check` | `next` is `stop` or `rule`; the budget 9, spent 0; the queue empty; every plant listed, the lead as a lead and the others as off the order; the stop's view written |
| the plants stand as planted | `check`: the ledger | each plant's grade as the opening wrote it — **or the grade triage gave it, where the review found the planted defect and triage keyed it to the plant's row** (A18): say so, and see §11, risk 6 |
| the same defect under a second key | `check`: the ledger | none. **Possible, and what it looks like:** a row triage minted for the hook-link defect, class `route hunt`, listed as off the order beside the plant. No part verifies it — the hunt's classes are on no order of the parts — so nothing is verified twice here; in a real run it would be (`wf-same-evidence`, a row the reshape plan's §7 names and its `T10` adds to the pending list) |

**Watch, and when to interrupt.**
- *A prompt at a tool step:* stop. *At a reviewer's or a driver's own command, or at triage's write of its entries file under the scratch root:* note it; in a session that asks it is expected for a role's own command (§6).
- *A wait on a build, the gate or a regression set that part 1 left green:* interrupt — something runs twice.
- *A second triage, an advocate, an agent of the executor's type, a verifier:* interrupt.
- *The review row past seventy-five minutes* (44 on the first canary, on another door): note it; it is the measurement.
- *A halt before the record:* read it. **It is the second attempt without a record; the third is the human's to grant** (`rulings: [{ again: 'test' }]`, which exists).

### Part 3 — verification

- **Starts from:** the root part 2 left.
- **Act 1 — the order is ruled.** `{ stage: 'test', run: 'canary-parts', scratch: <scratch>, rulings: [ { order: [ 'route under-confirmed', 'route seed', 'behaviour under-confirmed', 'behaviour seed' ] } ] }`, as `T7` builds an order's ruling. Recorded by an executor, gated, committed, pushed; nothing started. Return: `status: 'ruled'`, `next: 'verify'`. About 25 minutes, nearly all of it the record's gate.
- **Act 2 — the stage continues.** `{ stage: 'test', run: 'canary-parts', scratch: <scratch> }`. Wave 1, the eight of §4; the queue read again; wave 2, one key; the second record.
- **Ends by:** the budget — nine of nine spent — and the record. `next: 'rule'`.
- **Reuses:** everything on record; the two binaries, by two start steps. **Pays again:** the opening steps of each act.

| Checked | Read from | Right |
|---|---|---|
| a ruling starts nothing | act 1's return; its agents | `status: 'ruled'`; one executor, one gate; no verifier, no reporter |
| the order re-opened the stage **without a go** | `check` after act 1 | `next: verify`; thirteen rows in the queue, the three route plants first; the lead still listed; no go on record |
| no instrument, no triage | act 2's labels | no preflight, no scope step, no reporter of an item, no triage |
| wave 1 | the labels | the eight keys of §4 and no other |
| each verifier's first call | its transcript | `dev/stabilize-record kit --run canary-parts --role verify`; then the read of its row |
| each verifier files its own verdict | its transcript; `r1/verdicts/` | its report handed over, then one `verdict … --from …` call; one file a key. **Count every filing the script refused and the verifier corrected** |
| the plants' verdicts | `check`: what is filed, the ledger | the regression confirmed, with the regression fact; the false route claim refuted as *does not reproduce*; the Linux-and-root claim `undrivable`; the reserve refuted |
| at most two follow-ups enter the queue per verdict | `check`: each filing's handed-on keys; the stop's view | never more than two keys a filing; what a verifier noticed beyond them listed as beyond the number; what a refuted plant's verifier handed on listed as under a parent that did not reproduce |
| wave 2 | the labels | one key: the one §4's table names for what was handed on |
| a handed-on item ahead of an older row | the labels; the view | where anything was handed on: it has a verifier, `plant-reserve-6` has none |
| a key is launched once in the invocation | the labels | no key twice. *This shows little by itself: the harness keeps a list of what it launched (`HARN` 3830). That a finding with a filed verdict is not launched by a later invocation is part 4's* |
| the budget | `check` | nine of nine spent; at least four rows unreached |
| the second record | `check`; the last commit | pushed; every filed file recorded; the view written, its rates adding up to nine |
| the return | act 2's return file | what `T7` gives a stage that verified: the budget, the queue by class, the hit rate's counts, the view's path and hash — and no list |

**Measured here, against the first canary** (the reshape plan's §9 table): a verifier's turns before its first state or drive (6.5 then), its turns of nothing but look-ups (13.5% then), and any search for the exit rule's text, the report's rules or the repro format (none is right). **And measured as a number nothing here changes:** verifiers that wrote their whole report more than once (31 of 80 then) — `KR` is not built.

**Watch, and when to interrupt.**
- *Act 1 launches anything but its executor and its steps, or is refused:* stop, and read the refusal's word — see §11, risk 2.
- *A reporter of an item, a preflight, a triage in act 2:* interrupt — the continuation runs an instrument.
- *A verifier for the lead, or for a row of the review:* interrupt — the queue is not the order's.
- *A tenth verdict filed, or a key launched twice:* interrupt.
- *A prompt at a tool step:* stop. *At a verifier's own command, or at its write of its filing under the scratch root:* note it (§6).
- *A verifier that returns and files nothing:* note the key. The wave after it is then two keys, not one.
- *A line of the runtime saying it changed a return:* note the agent and the pattern (the row `wf-neutralized-return`).

**If it ends otherwise.** `next: verify` with budget left — a verifier left no verdict and the queue still prints its finding: act 2 again, once. That is the one case in which part 4 does not open with a raise at once.

### Part 4 — the interruption

- **Starts from:** the root part 3 left: the budget spent, rows unreached, `next: 'rule'`.
- **Act 1 — the raise, alone.** `{ stage: 'test', run: 'canary-parts', scratch: <scratch>, rulings: [ { budget: 13 } ] }`, as `T7` builds a budget's ruling. Recorded, gated, committed, pushed; nothing started; `next: 'verify'`.
- **Act 2 — the wave, and the kill.** `check` first: it prints the attempt this invocation will begin, **N**. Then `{ stage: 'test', run: 'canary-parts', scratch: <scratch> }`: four verifiers. In a second terminal the human lists `<root>/clone/completions/artifacts/canary-parts/r1/verdicts/`, which holds part 3's nine files already. **The window opens when two files ending `.a<N>.json` are there, and closes when the fourth verifier returns.** Verifiers took between 2.5 and 34 minutes on the first canary, so the window is minutes wide where a slow verifier is in the wave, and may be under a minute where all four are reserve plants. In it, **the session is ended**: interrupted at its terminal, and left.
- **Act 3 — nothing of it still runs; a snapshot; a look.** *What ending a session does to a workflow's running agents is not established.* So: the verdicts directory is listed twice, a minute apart — **right: no file appears between the two**; and `pgrep -fl <root>` — **right: it prints nothing**, and a process it prints is ended by `kill` with its number. Then `check`, which prints every filed file with its hash. Then, in a new session, `{ stage: 'test', run: 'canary-parts', scratch: <scratch>, stopAfter: 'state' }`: it launches nobody.
- **Act 4 — the continuation.** `{ stage: 'test', run: 'canary-parts', scratch: <scratch> }`, to its record.
- **Act 5 — recommended, and the human's to leave out: a record killed at its gate.** Act 4 is ended by the human while its record's gate is held — the label `hold-wait:record-…`, a window of about 22 minutes. The gate itself keeps running, as a held command does: the human waits until `<scratch>/hold/record-…/verdict.json` is there. Then the same arguments once more: that invocation finds a batch applied and not committed, gates it again under the next name, commits it, pushes it and starts nothing. About 35 to 45 minutes more, some six tool steps, no verifier (§11 says why it is worth them).
- **Reuses:** everything. **Pays again:** the opening steps of each invocation; a verifier, under the next attempt, for each finding the kill left without a verdict — as far as the budget reaches.

| Checked | Read from | Right |
|---|---|---|
| a raise starts nothing | act 1's return; its agents | `status: 'ruled'`; one executor, one gate, no verifier; the budget 13, four left |
| the wave | act 2's labels | four `verify:` labels: the first four of the queue, as §4's second table has them |
| the look | act 3's return | **where the first read found nothing owed but filed verdicts:** `status: 'stopped'`, and what the stage is continued for names both a row to verify and a filed verdict; two tool steps. **Where the kill fell inside a write of the record script:** the look returns at its first read, names that write as owed and says nothing of a continuation (A19) — that is right too, and act 4 finishes the write first |
| **no instrument after the kill** | act 4's labels | no preflight, no scope step, no reporter of an item; no triage |
| **the filed verdicts kept** | act 3's snapshot against `check` at the end | every file of the snapshot still there, with its hash, and recorded |
| **only what has no verdict is launched** | act 4's labels against the snapshot | no `verify:` for a key the snapshot holds a counting file for; never more verifiers than the budget had left at the snapshot. *Not always one for each finding that was in flight: an item a filed verdict handed on ranks ahead of the reserve and may take its place* |
| no verifier for the Linux-and-root plant, in any act | the labels | none: its verdict was filed in part 3 |
| the budget across the acts | `check` | spent never above 13; the launches above it are the ones the kill cut off |
| the record takes both attempts | the last commit's files | the reports and the filings of attempt N and of the attempt after it |
| act 5 | its return; `check`; the commit | one commit for the batch; pushed; nothing pending; no agent but tool steps; the first gate's job passed over for the next name |

**Watch, and when to interrupt.**
- *Act 1 is refused:* read the word. A raise is refused while a verdict of the budget is left (A5) — then part 3 did not end as planned, and its last paragraph says what to do.
- *Act 2: the window is missed* — all four return: nothing is lost. The stage records, the budget is spent with rows unreached where any are left, and the interruption is made after one more raise. Where the queue is empty by then, part 4 is run from a fresh root (§9).
- *Act 4: a reporter of an item, or a verifier for a key whose verdict is filed:* interrupt — something runs twice.
- *Act 5: a second gate started while the first still runs:* the human did not wait; note it — two gates on one machine can redden either (A14).

---

## 6. Permission, part by part

A fresh clone is an untrusted workspace, and the first canary's output says how one is trusted (A13); §5 makes that a step before part 1. **Whether trusting a clone applies its committed allow rules to the agents a workflow launches is not established** — the first canary ran untrusted and bypassed, and what the probes showed of the rules they showed in the repository, a trusted workspace. Part 1's first minute establishes it.

| Part | Whose commands run | What follows |
|---|---|---|
| 1 | tool steps; one preflight; one scope step | Run in a session **that asks**. A prompt at a tool step is the finding the ruled table names. The preflight and the scope step type commands of their own; a prompt there is noted, with the command |
| 2 | the same, and three reporters that read source and drive the binary | a reporter's commands are free by design: in a session that asks, each may prompt. The human chooses the session's mode knowing that |
| 3, 4 | tool steps and verifiers, up to eight at once | the same, eight times over: in a session that asks, a wave parks on its prompts. The human chooses the mode |

**Two writes that are no command, and may prompt all the same** — each has a row in §5's watch-lists:

- *An agent's file-tool write under the scratch root:* the scope step's scope file, triage's entries file, a verifier's filing. In the repository a write under a scratch root outside it was observed not to prompt ([re-review-test-half/probes/README.md](../re-review-test-half/probes/README.md), the second run of the `payload` probe). In a clone, never observed.
- *The session's own write of the return file,* under `<root>/results/`, outside the directory the session stands in. Never observed.

**Whatever the mode,** the reading of every part lists each command an agent ran that is no call of the three tools, by role and by first word. That list is what the real run's session has to cover.

**Nothing is built for this.** The setup tool writes no rule and no settings (its header: *this tool writes none*).

---

## 7. What the parts are expected to cost

**The rates, and whose they are.** All from the first canary's record and the three counts beside the reshape plan; the review recomputed them.

| Rate | Value | Source |
|---|---|---|
| a tool step | 1.3 k output tokens | 22 steps, 29 k (the third count, table 1a) |
| the first preflight; the scope step | 7.5 k, 1.2 min; 54 k, 10.2 min | the canary's agents, attempt 1 |
| the two builds | 28 s and 26 s | [canary-one/data/held.json](../canary-one/data/held.json) |
| the candidate's gate; a record's gate | 23.3 min; 22.6 min | the canary's timeline |
| the regression set | 22.3 min | the timeline |
| a review row (source, driver, reconciler) | 241 k, 44 min | the canary's row on `jigc doc list` — **another door** |
| a triage pass | 118–169 k, 18.5–33 min, whatever it was handed | the reshape plan, §6 `Q2` |
| a verifier | 50 k, 10.5 agent-minutes (median 8.4; 2.5 to 34.1) | the canary's record |
| a verifier of a small false claim | 12 k, 2.5 min | the canary's seeded row — one observation |
| USD per M output tokens, cache traffic included | about 52 | 311 USD over 5.93 M, both invocations |

| Part | Agents | Run time, minutes | Output tokens | About, USD | The human's own time |
|---|---:|---:|---:|---:|---|
| 1 | 16 (14 tool steps, 2 roles) | 55–65 | 0.08 M | 4 | the first minutes, then the reading |
| 2 | 23 (17 tool steps, 6 roles) | 100–115 | 0.42–0.47 M | 22–25 | the first minutes; the reading |
| 3 | 32 (22 tool steps, an executor, 9 verifiers) | 70–125 | 0.25–0.5 M | 13–26 | two starts; the reading |
| 4, acts 1 to 4 | about 35 | 70–105 | 0.1–0.3 M | 5–16 | **act 2 attended to its window**, and four starts |
| together | about 106 | 295–410 | 0.85–1.35 M | 44–70 | about two and a half hours |
| act 5, if it is run | 6 more | 35–45 more | under 0.01 M | under 1 | one wait, one start |

Part 3's range is the nine verifiers at the two rates that exist: three plants at a verifier's average and six reserve plants at the seeded claim's, or all nine at the average.

**Nobody's measurement** — each is this plan's estimate, or left out of the table and said so here: a review row at `jigc setup`; a triage handed three reporters only; **the 30 k a triage is expected to save by typing its entries once** (the table prices a pass as the first canary's); **a composing step** (priced as any tool step); **what the kept baseline saves as a stage sees it** (the regression tool's own measurement is about eleven minutes; no stage has held one); a rulings record's executor (the 118 k of the canary's executor was a 232 KB batch typed again, which a ruling is not); the scope step where the scope stands; a verifier of the planted regression; a verifier of a reserve plant at `jigc setup`; the human's time.

**The cost is an addition, not a trade.** §11 recommends that the whole canary of §9 stays owed. So the parts' 4.9 to 6.8 hours and 44 to 70 USD are paid **on top of** that canary's 190 to 330 minutes and about 60 to 105 USD: together about 8 to 12 hours of run time and 105 to 175 USD — and 25 to 50 minutes more for every repair between two parts that ends a root (§9). What the parts buy for it is that a defect is met in the cheapest stretch that reaches it, and that the whole canary is then run on a path whose pieces have each been read by real agents once.

---

## 8. Reading a part afterwards

So that no reading rests on the human's memory of the session:

1. **The return** — the file the act's prompt had the session write. An act that was killed has none, and that is right.
2. **`dev/stabilize-canary check --root <root>`**, from the repository: a tool act, read-only. Today it prints the two heads, the commits on the opening, what is pending, the state's next step, round and attempt, the items with whether the round selected them, each ledger row's grade and route, and which returns are there. `SC3` adds what the parts read: the budget, the queue, every filed file with its hash and whether it counts and whether a record took it, the view's standing, and — for each act — whether its return is there **and what the run itself shows of it** (attempts begun, records made), so that *never ran* and *ran and was not written down* are two answers (the third fault of the row `wf-canary-setup`).
3. **The transcripts, by a read-only subagent** the orchestrating session launches with the part's table from §5 as its brief — as the third count was made: the agents' labels and types, each agent's commands by first word, the writes of each report and entries file, a verifier's first call, tokens and minutes per role. It returns the table filled, with the file and line behind each cell.
4. **The record of a part** is that table, the `check` line and the return, committed beside this plan as aggregated counts — no transcript, no path of a machine, nothing of the clone's tree.

**Rejected:** a committed script that counts transcripts. It is new tooling for four readings; if the four subagents' counts turn out not to compare, that is a row.

---

## 9. Between two parts

**The human's standing rule has three classes:** what a part shows is repaired before the next part where it can publish unscanned, record falsely or strand state. **This plan adds a fourth, and names it as its own:** *or where the next part cannot run without it.* Everything else becomes a row with the part's number beside it, and is tuned across real rounds.

**A repair of the harness, a tool or a definition ends the root** (§4). What the next part then pays, from a fresh root of the repaired commit:

| The next part | The fresh setup | What is invoked | Pays again | Does not run again |
|---|---|---|---|---|
| 1 | as written | part 1 | everything of part 1 | — |
| 2 | the same opening **without the two regression items** | part 2 | the two builds, the gate (about 25 min), the preflight, the scope step — inside part 2's own invocation | the regression sets |
| 3 | an opening whose **one item is the gate** | part 2's invocation, then part 3 as written | the builds, the gate, the first record's gate (about 50 min, some 20 agents, under 0.1 M tokens) | the review row; and the triage — **where the scope step leaves nothing open**: with no instrument and every seed graded nothing else is handed to a triage (`HARN` 3769–3771, 4503) |
| 4 | the gate as the one item, **the opening's order already part 3's, and a budget of four** | part 2's invocation, which then goes from its first record **into a first wave** — the three route plants and `plant-reserve-1` — and is ended there by the human; then part 4's acts 3 and 4 | the same as for part 3 | part 3's nine verdicts; **and no ruling is run in that root** (§11) |

So no part has to be preceded by a full run of the parts before it. A fresh root for part 4 runs, in passing, the one passage no part of the main line runs (§3.6).

**Rejected:** carrying the repair's commit into the open clone. The workflow's own path rule would take it — a tooling path commits directly on a loop branch — and it is what the real run will do when its tooling is repaired mid-run. But it is hand work on a clone's git state, outside any tool act.

---

## 10. The build before part 1, and what stands where

**How every builder task runs** is the reshape plan's §4: red first, one commit, one full gate on that commit's tree, a task carries the assertions it breaks, the old path untouched. Write-set abbreviations are that plan's.

**`T7` — the harness: what a stage returns, and the rulings about the run.** *Its place: first, before part 1; it is being built.* **Its row is no longer the committed plan's text.** The run rulings are as the record script has them (A5, A20), and the builder has been sent this:

- a `budget` ruling and an `order` ruling, each taking the one road the other run rulings take;
- **a `hand-on` ruling refused**: the number is written once, whatever the committed plan's `T7` says of it;
- an arm for an order ruled at `next: rule` that puts a listed class into the queue and starts nothing;
- an arm for an order and a raise in one invocation — refused as two invocations, or made as two calls: the builder's decision. **No part of this plan passes the two together.**

*What this plan needs beside that, and asks to be added to the same arm:* **an order ruled where `next` is `stop` after a round.** That is the state part 2 most likely leaves: the setup writes the stop mode *after every round* (A10), and the round is over where every row is listed or recorded (A15). By the record script an order is taken at any step, and by its next-step function the round is then no longer over and the step is `verify`, with no go; the harness's side of that is `T7`'s, and nothing drove it.

**`P0` — the plants' words, driven once.** *Not a builder's task: it commits nothing.* *Who:* one driving agent of the kind that drives a binary and commits nothing, launched by the orchestrating session. *With:* the candidate's binary and the previous release's, each built from its commit by the step tool's own held build under a scratch root of the agent's; a fresh `dev/jigc-rig bare` for every drive, with the binary named to it. *What:* every row of §4's plant table on both binaries, as written — the regression's block as the first canary's report has it, the false route claim, the ten reserve claims and the two spares. *Written:* one table under the orchestrating session's scratch root — per plant the commands, each exit status, what was printed, and one word: *as planted* or *not*. A reserve claim that is not false on both binaries is replaced by a spare. **If the regression does not reproduce on the candidate, or is not green on the previous release, `P0` halts and nothing after it is built on it.** *Depends on:* nothing; it may run while `T7` is built — but its binaries are not built while a full gate runs (A14).

**`SC2` — the setup's opening for the reshaped path, cut down** (the first part of `T9`). *For:* the ruling's *a canary setup for the new path*; the reshape plan's §9; the first third of `wf-canary-setup`.
- *Writes:* `CAN`, `LOG`.
- *What:* under one switch the setup writes the opening of §4 — the door, the four items, the budget, the hand-on number, the order and the condition's heading through the clone's own record script — and **the plants of §4's table as `P0` confirmed them, taken as data**: each a row with its grade, its class and its kind's evidence. The budget, the order and the items are flags with those defaults, so that a fresh root can be opened as §9 has it. **Without the switch the setup is what it was, to the byte.**
- *Done when* — `dev_stabilize_canary::`, red first: `a_fresh_setup_is_ready_with_the_budget_the_order_and_the_hand_on_it_was_given`; `its_seeded_rows_carry_the_grade_and_the_class_the_canary_plants` — and at the opening the queue is empty, nothing is untriaged and `next` is `test`; `the_order_part_three_rules_puts_the_route_plants_and_the_reserve_into_the_queue_in_that_rank`; `its_door_is_the_review_rows_and_the_plants`; `a_setup_without_the_switch_is_what_it_was`. **And its acceptance: each plant driven once more by the builder on both binaries, with what it showed in the commit — and the task halts if the regression does not reproduce.**
- *Must not:* write a route hunt, a second door, a trial arm or a scripted arm as a held kind (the later `T9`); reword a plant; write a rule or a settings file; make the switch the default.
- *Depends on:* `P0`. **Not on `T7`:** it prints no arguments and writes its facts through calls the record script has today (the spike). *Who:* a builder.
- *What it leaves to the later `T9`:* the hunt's door and its item; the scripted arm as a held item, and the image; `its_expectation_names_what_it_prices` for the whole canary; the opening with a queue as the only opening.

**`SC3` — the setup writes the parts' prompts, and `check` reads the parts.** *For:* the standing rule that a run is driven by the human with exactly what to type; §8; the third fault of `wf-canary-setup`.
- *Writes:* `CAN`, `PEND` (the row `wf-canary-setup`, annotated), `LOG`.
- *What:* under the switch the setup writes **one prompt file for each act** under `<root>/prompts/` and names it in its line beside the act's argument object, its return file and how many agents it launches where every agent ends well; `check` prints what §8 lists.
- *Done when* — `dev_stabilize_canary::`: `the_acts_run_in_order_through_the_clones_own_harness` — the stand-in runtime drives part 1 to its stop with no record, part 2 to its return with one record and no verifier, the order's ruling, part 3 to its second record, the raise, a wave that dies after two filings, the look and the continuation, **each launching as many agents as its line says**; `a_prompt_file_holds_the_arguments_its_line_names_and_nothing_a_shell_reads`; `check_says_of_each_act_what_was_returned_and_what_the_run_shows`; `a_filed_verdict_is_printed_with_its_hash_before_and_after_the_record_that_takes_it`; the arm that holds every named command to one plain invocation, carried.
- *Its suite, and the runtime's stand-in.* The fixture ends an agent by its label where a scenario says so, and the simulation's own scenario of an invocation that died after its wave filed uses exactly that, on the wave's report check; the canary's suite scripts endings the same way today. So **no change of the fixture is expected** for a wave that dies or for a ruling's record, which the fixture's executor stand-in already makes. *Not established:* the helpers that script a verifier's filing live in the simulation's own file and are not shared — `SC3` writes its own, or the task's write-set gains that file and the commit names it.
- *Must not:* change the opening (`SC2`); write into the repository; print a time nobody measured as an expectation.
- *Depends on:* `SC2`, `T7`. *Who:* a builder.

**`R0` — a review before part 1, scoped.** The committed sentence is: *a `milestone-code-reviewer` over the build's whole diff before any stage is run with it* (the reshape plan, `J2`). The parts are stages, and they run before the build is whole. So: **before part 1, a `milestone-code-reviewer` that built none of it reads the diff of the code the parts run** — the harness's path of a run with a queue, the record script, the step tool, the regression tool's kept baseline, and the setup tool — **for three dangers only**: can it publish unscanned, can it record falsely, can it strand state. What it finds in those three classes is repaired before part 1; everything else it finds is a row. The review of the whole diff stays at the join. **This is a deviation from the committed sentence, and the human's to overrule.** *Who:* the orchestrating session launches it, and checks each finding before it is worked in.

**Then, with no agent:** the harness's self-test by the script's path, as the reshape plan's §8 has it; and CI read for the pushed tip.

### `K1b`'s place

**After the parts.** A part measures the kit without it where it matters: the verifier's prompt names the kit as its first read already (A12), and the first canary's numbers for the kit are verifiers' numbers. What `K1b` adds is the kit's call in the reporters' and the triage's prompts, and the harness's half of `KR` — the line that tells a role when to write its report. The definitions that line binds on change only at `KR`, which the ruling holds back. **So part 3 gives `K1b` its before-and-after for verifiers, and part 2 its before for a review row.** *Not established:* how the reshape plan means the fence's arm for that line to hold between `K1b` and `KR`, which it orders apart.

### Every remaining task of the reshape plan

| Task | Where | Why | Do the parts' numbers change it? |
|---|---|---|---|
| `T7` | before part 1 | above | its return, if a part shows a field missing |
| `K1b` | after the parts, with `O4` and `KR` | above | **yes**: part 3 says whether verifiers use the kit and what they still look up |
| `O4` (the two reporters' definitions) | after the parts, before `T8` | held back by the ruling | **yes**: part 2 shows how many of a review row's findings triage could give a kind with its evidence, and how many became leads |
| `T8` (the route hunter) | after the parts | held back | its budget rule: part 3 gives a verifier's cost for one route at `jigc setup`; the hunter itself is measured by nothing here |
| `K7` (the image's base layer) | after the parts; it may be built beside them — **but not gated while a part runs** (A14) | third lane | no |
| `K7b`, `K6` | after the parts | held back | no — but part 1 says whether a session that asks stops at a role's own wait, which is `K7b`'s reason |
| `T9`, the rest | after `K6` | above | **yes**: by everything the parts showed of the setup, its prompts and its `check` |
| `T11`, `T12` | after `T9` | held back; no part runs the old path | no; a defect a part shows on the new path is repaired before the old one is deleted |
| `O5`, `KR` | after `T12` | held back | `KR`: part 3's count of reports written twice, under a prompt that already says *after your report* — if it is near nought, `KR` shrinks |
| `T10` (the doc pass) | last | held back | it takes in what the parts showed — and the two sentences below |
| `J1`, `J2`, `O6`; the review of the whole diff | the join | the reshape plan's | no |
| the rows due at the first stop — `wf-triage-per-wave` and the four `wf-pre-…` rows in the pending list, and `wf-rule-a-class` and `wf-same-evidence`, which the reshape plan's §7 names and its `T10` adds there | unchanged: the real run's first stop | plants are not a sample | **no, and nobody should let them** |

**Two sentences of `implementation/stabilization-workflow.md` that the parts' opening and rulings contradict as worded, named for `T10`:**

- *Bounds and exits:* **"nothing but the go lifts the stop"**. An order ruled at the stop after a round puts rows into the queue; the round is then not over, and the step is `verify` with no go on record (part 3, act 1).
- *Opening a run*, the table's last row: the ledger's opening rows — **"the first triage that runs is handed them"**. A row graded at the opening is handed to no triage (part 2; the spike: nothing untriaged).

Dropping the stop this plan first proposed leaves the document's sentences on `stopAfter` as true as they were.

---

## 11. What may go wrong with the cut itself

1. **A stage of parts is not a stage.** No part of the main line runs the passage from a first record into a first wave inside one invocation; an attempt whose reporters are instruments **and** verifiers under one second record; a wave at the cap of eight over the hunt's rows. *Shows:* nothing in the parts. It is the first reason a whole stage is still owed.
2. **Part 3 opens on a ruling at a state nobody drove.** That an order ruled at the stop after a round is taken by the harness and re-opens the stage is read from the two scripts (§10, `T7`). *Shows:* part 3's first act, in 25 minutes and before any verifier. *If it is refused:* a go first (`rulings: [{ go: true }]`, which exists) and then the order — one more record's gate — and a finding for `T7`.
3. **The ends of parts 3 and 4 rest on every launched verifier filing a verdict.** One that files nothing leaves a verdict of the budget unspent, and the raise is refused until the stage has been invoked once more. *Shows:* the count of filed files after part 3's first wave.
4. **Ten reserve plants nobody has driven are most of the queue.** One that turns out true is a real finding and a slow verifier; one whose verifier cannot settle it is a row the human did not plan for. *Shows:* `P0`, before anything is built on them.
5. **Every repair between parts costs the builds and a gate, and for parts 3 and 4 a record's gate too** (§9) — and a part whose finding was repaired is not run again in its own shape. *Shows:* the count of fresh roots, kept in each part's record.
6. **Triage may grade a plant after all.** Where the review row finds the planted regression itself, triage is told to key that finding to the plant's row, and the plant then takes triage's grade and keeps its class (A18). A grade that does not send a finding on keeps the plant out of the queue when part 3 rules its order. Part 3 still ends as planned — the reserve fills the wave — but nothing is confirmed, nothing is handed on, and wave 2 is `plant-reserve-7`. *Shows:* part 2's `check`, the plants' grades. *If so:* part 3 from a fresh root whose one item is the gate (§9).
7. **The numbers of a part are numbers over plants.** A hit rate, a count of follow-ups, a cost per verifier: none is a sample of a real round. *Shows:* every part's record says so at its top.
8. **The window of part 4 may be under a minute** where all four verifiers of the wave are reserve plants. *Shows:* at once; the watch-list says what a missed window costs.
9. **A gate in the clone and a gate of the build at the same time** make one of them red (A14). The rule in §5 stands for every act.
10. **A session that asks parks a wave.** *Shows:* part 2, before the eight verifiers of part 3 — the human then chooses the mode with one observation in hand.

### What no part runs

- the route hunter, the scripted arm as a held check, the trial image as a held command;
- a stage's passage from its first record into a first wave in one invocation — but for a fresh root for part 4 (§9);
- **a continuation that only records** — a filed verdict that no record took and nothing else left. It is the newest code of the harness; its window is the seconds between a wave's last filing and the record's batch, which a person cannot time;
- **a record killed between its batch and its commit** — *unless act 5 of part 4 is run, which this plan recommends.* The window is the record's gate, about 22 minutes, so it is easy to hit; it is of the class that strands state; and the simulation drives it for a stage's first record only, never for a continuation's;
- **the two rulings, in a root that was opened afresh for part 4** (§9): that root is opened with part 3's order and runs no ruling;
- a row the human puts into the queue by key (`queued`): no part needs it, and `T7` builds no ruling for it;
- a look over a killed write, unless the kill of part 4 happens to fall in one (A19);
- a triage inside a continuation (a row nobody graded, a seed added since); a re-run inside a round; a cross-model pass; a contested verdict, unless a verifier contests of its own accord;
- the last wave after two silent calls, and the halt that names the limit — a limit cannot be planted;
- a red candidate, unless the candidate is red; the `fix` stage; a second round;
- a state, a prompt or a queue at a real round's size; a tooling repair carried into an open run.

### Is the reshape plan's one whole canary at the end still owed?

**Yes — the review supports it, and this plan recommends it.** `T8`, `K6`, `K7b` and `T12` are unbuilt, and `T12` edits `runTest` exactly where an invocation is handed on to a continuation (`HARN` 4293–4315) — so the hope that the whole canary could drop its interruption because part 4 was clean will rarely hold. **What it would still have to show that no part showed** — so that the human can decide after part 4, with the parts' data, how much of it is run:

1. the route hunter: its coverage count beside the site list's for its command, and its cost for each site it provoked;
2. the scripted arm as a held check, run by no agent, and the trial image held as a command with no agent's wait;
3. a stage that is not cut: instruments, the one triage, the first record, the first wave and the second record in **one** invocation, under one attempt;
4. a wave at the cap of eight over rows a hunt found, and a handed-on item overtaking a hunt's row;
5. the harness after `T12`: one path, the old one deleted, the continuation's entry edited;
6. an interruption and a continuation on that harness;
7. a budget spent with **route items of a hunt** unreached, and the stop's view over rows that are no plants;
8. a prompt and a state at the size ten verdicts and a hunt give them;
9. whatever of the list above — *what no part runs* — the human wants shown before a real run and the whole canary's shape can reach: a continuation that only records cannot be timed there either.

**Rejected:** letting round 1 of the real run stand in for it — what a first run strands, it strands on the real loop branch.

---

## 12. Chosen between alternatives — what was rejected, one line each

- *A stop after the first record, built:* new code on the path every later part starts from; the opening's order ends part 2 with nothing built, and runs the stage's own return besides.
- *Ending part 2 by a budget of nought:* refused by the record script (A20).
- *Ending part 2 by the human's kill after the first record's push:* a race with the first wave, and it is part 4's subject.
- *A part 1 that runs to a record, with held items only:* nothing pending afterwards — but the round is then tested, and part 2 cannot run its instrument in that root.
- *A budget of five for part 3, with four reserve plants:* its end is as sure and it costs five verdicts less — but an invocation with five to spend has one wave (§3.9), and the queue read again after a wave, which is the phase's loop, would be run by no part.
- *A budget of ten for part 3, as §9 has it:* a second wave of two — but one more reserve plant and one more verdict for nothing the wave of one does not show.
- *An order for part 3 without the reserve's class, and the queue left to what a verifier hands on:* part 3 then ends with the budget spent in about one case in eight, and part 4 opens on a contingency.
- *The hunt's classes on an order of the parts:* the parts have no hunter; a row of the review in the queue would make the waves depend on what a reviewer happened to find, and a second key for the planted defect would be verified a second time.
- *Fresh roots for every part:* each pays the builds and the gate again, and parts 3 and 4 a record's gate.
- *One scratch root shared by several roots:* the stage's own files under it are named by round and attempt and by no run (A8).
- *Parts 3 and 4 as one, as §9 has them:* cheaper by two gates — but the ruled table separates them, and a wave that goes wrong under a kill cannot be told from one that goes wrong alone.
- *The kill at a record's gate in part 3 instead of at part 4's end:* it would stand between the first real second record and its reading.
- *Running part 1 today, on the setup as it stands:* the human would type an argument no tool prints, the root could not be continued into part 2, and the gate would be paid twice.
- *`K1b` before part 2:* its report line binds on definitions the ruling holds back.
- *A prompt lifted from the setup's line of JSON:* a text with escapes, pasted by hand.

---

## 13. What this plan did not establish

**Nobody has checked — neither this plan, nor the review, nor the spike:**

- **What ending a session does to a workflow's running agents.** Part 4's third act looks for what still runs because of it.
- **Whether trusting a clone applies its committed allow rules to the agents a workflow launches.**
- **Whether a first record composes with no triage at all** — a root whose one item is the gate (§9).
- **What the scope step does and costs where the round's scope stands** (part 2).
- **The stand-in runtime's scripting for `SC3`** beyond what §10 says of it: that the fixture ends an agent by its label was read from its header and from the simulation's own scenario; the helpers for a filing were not read.
- **The planted regression, driven by nobody since the first canary.** The product's source has not moved since; `P0` drives it.

**Read from the code, driven by nothing:**

- That an order ruled at `next: stop` is taken by the harness and re-opens the stage (§10; §11, risk 2). The spike drove the record script's half at `next: test`.
- That the opening of §4 — an order of the two classes only a filing gives — leaves every plant listed and nothing untriaged. The spike drove an opening whose order named the route plants' class; this one differs by that class.
- That the stage starts the second regression item with the same baseline and is answered `used`.
- That `check` sees a scope that is written and not yet committed (part 1's row on the selected item).
- That the invocation after a record killed at its gate passes the first gate's job over and gates again (act 5): read from the hold's header and from the step tool's header on a batch applied and not committed.
- What `T7` will call its rulings and its return: the argument objects of the rulings are this plan's reading of what its builder was sent.

**Not known until `P0`:** whether each of the six reserve claims that are this plan's is false on both binaries, and the exact bytes of the two route lines on the commit the parts will test.

**Whether the tree is green, and whether any of this runs:** no gate, no self-test, no stage and no tool act was run for this plan.

**Rates:** every one §7 marks as nobody's; and how the first canary's per-agent counts were derived beyond what the third count's own method section says.

**Not read whole:** the harness beyond its header on arguments and stops, `runTest`, the triage, the verify phase, the continuation, `hold`, the two builds, the rulings' validation and the two prompts of a run with a queue; the record script beyond its header on the ledger's writers, `run-set`, the triage's entries, a filing's hand-on, the state's fields, the queue and its second table, and the functions that compute the queue, the next step, the position and whether a round is over; the step tool beyond its header on the first read and on a held command; the regression tool beyond its header on a kept baseline; the setup tool's suite beyond its arms' names and its one function that invokes the stand-in; the simulation beyond the one scenario helper named; the fence; every agent definition but the triage's paragraph on a finding found again; the workflow document beyond *Invoking the harness*, *Opening a run*, *Bounds and exits* where it speaks of a stop, *Branches* to *Halt and resume*, and *The small canary*; the reshape plan's stream P, its §12 and its §1 beyond the table; `DECISIONS.md` beyond the entries of 2026-10-11 and 2026-10-12 on the first record, the verify phase, the continuation, a filed verdict no record took, the kept baseline, the contested verdict, the new path's triage and the three rulings — and of those, the sections on what was built, what the next tasks owe and what is not established.

---

## 14. What the review changed

The review found the plan's fourteen facts and its eight *cannot be built as worded* points true, and no step at which a part cannot start from what the part before leaves. What it changed:

| Finding | What the plan said | What it says now |
|---|---|---|
| a new stop on every later part's path (the plan's own weakest point) | part 2 ends at a stop to be built, `SC1` | part 2 ends by the stage's own return: the opening's order names no class a row can have yet; `SC1` is gone; part 3 opens with a ruling of the order (§3.5, §4, §5) |
| part 3's end rested on what a verifier hands on | a budget of 5; *expected: 5 of 5 spent* — true only where two route items are handed on; part 4 opened on a contingency in most cases | a budget of 9 over a queue of thirteen: the budget is spent and at least four rows are unreached in every case, worked case by case; part 4 opens with a raise alone (§4) |
| a handed-on behaviour item was listed and never launched | part 3's order named the two route classes | `behaviour under-confirmed` is on part 3's order |
| the hunt's classes ranked ahead of the reserve | part 4's order named them | on no order of the parts |
| the run rulings as the script has them | `T7` *as the reshape plan cuts it*; an order and a raise passed together | `T7`'s row restated: `budget` and `order`, a `hand-on` ruling refused; no part passes the two together; this plan asks for the arm at `next: stop` as well (§10) |
| the review before a stage is run | paraphrased as *before the canary*, and left at the join | the sentence quoted; a scoped review before part 1, `R0`, for three dangers; said to be a deviation, the human's to overrule (§10) |
| the plants left to a builder | four of §9's and *four small false claims* | the table: key, claim, evidence, class, source, outcome; a named step `P0` for the words that need a drive; `SC2` takes the table as data and halts if the regression does not reproduce; `SC2` no longer waits for `T7` (§4, §10) |
| what the human does | a prompt printed in a line of JSON; `<run>` left open; the window said in words | a prompt file for each act; the run named; the clone trusted as the first canary's output says; the window said in files and in minutes; how the session is ended and how to see that nothing still runs (§5) |
| the cost | compared with the whole canary as an alternative | stated as an addition, with the sum and the cost of a repair; three more rates marked as nobody's; every part recomputed (§7) |
| the whole canary | may drop its interruption where part 4 was clean | stays owed; a list of what it still has to show (§11) |
| part 1's pending files | three | four, every one under the run's `r1/` |
| a red held job in part 1 | *right* in part 1, and a wait in part 2 said to be wrong | part 2 passes a red job over and starts it again, once, under `…-2`; the labels that are wrong to see and the one that is expected are named |
| *nothing verified twice* | a row of part 3 | worded as what it shows there; the half across invocations is part 4's; a second key for the planted defect said to be possible, with what it looks like |
| what no part runs | a list without five of them | a continuation that only records; a record killed at its gate — and an act that runs it; the rulings in a fresh root for part 4; `queued`; a look over a killed write |
| `SC3`'s write-set | the setup tool and its suite | what the fixture gives already, and what is not established of the helpers |
| the workflow document | not looked at | two sentences named for `T10` |
| a root whose one item is the gate | *no triage is launched* | — where the scope step leaves nothing open |
| the rule between parts | *or where the next part cannot run without it*, unmarked | named as this plan's own fourth class |
| prompts at a write | no row | two rows in §6, and in the watch-lists |
| how a clone is trusted | not established | the first canary's own line; what stays not established is whether it reaches a workflow's agents |
| the pointer of the fact about a loaded machine | *the two entries of 2026-10-11 and 2026-10-12* | the tooling suites' two red arms are the entry of 2026-10-12 alone; the note of 2026-10-11 is of the product's suites, and is cited as that |

**Where the reviewer's reading was not taken whole:** the last row. The review has the fact at the 2026-10-12 entry only. That is right for the tooling suites; the dated note of 2026-10-11 above the entry on a filed verdict records thirty product tests at the runner's bound in one gate on a loaded machine, and the rule in §5 rests on both.
