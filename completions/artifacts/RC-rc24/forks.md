# Open forks — what the records did not settle

> **Every fork below was taken before the trial ran, and each carries a `Taken:` line saying
> how.** This is `open-forks.md` as drafted — the name [protocol.md](protocol.md),
> [runbook.md](runbook.md) and `paste/README.md` still use — renamed when the trial's directory was
> committed, with those lines added and nothing else changed. F1 and F8 were the human's explicit
> decision to run the prompts as drafted; the rest were taken by the orchestrator.

Each is the human's. Each says what was found, why it forces a decision, and what is recommended.
The draft is written on the recommendation in every case, so taking a different option means
editing the file named.

Ordered so the ones that change a frozen instrument come first. F1, F2, F7 and F8 change a turn
file; F3–F6 change what runs; F9–F12 are housekeeping and block nothing.

---

## F1 · Arm (a): set the milestone up in turn 1 and start it in turn 2, or ask for everything in turn 1?

**What was found.** The fan-out has been put to a blind agent three times, always with one prompt:
plan the milestone, cut it, run it, land it. It reached the fan-out once, in RC-m50's seeded
two-turn run — and that run's own out-dirs show **turn 1 did all of it**. `provision`, the three
sub-task workflows, the join and the boundary commit are all in turn 1's log, the last at
18:44:02Z; turn 2 began at 18:44:43Z and added two records. The second turn was a reply written
for a question the worker had asked in the *earlier, single-prompt* run, and in the seeded run the
worker never asked it. So the one success and the two failures are three draws of one prompt. The
failures were a stop at the pack's human-owned Settle gate, and a worker that cut the milestone
into steps depending on each other and then correctly judged the fan-out unfit for them.

**Why it forces a decision.** "A seeded two-turn conversation" is decided; what the second turn is
*for* is not, and the record that suggested it mattered does not show that it did.

- **Option A — the split (drafted).** Turn 1 hands over the cut — three independent pieces in
  three modules — and the sign-off, and asks for the milestone to be set up *and no more*. Turn 2
  says start all three together, each kept apart, land them together. Turn 2 then arrives into a
  state the product re-raises: driven on rc.24, orientation lists the unstarted sub-tasks, and
  resuming one routes to provisioning. Turn 2 also restates the sign-off, so it still answers a
  turn 1 that stopped at a gate.
- **Option B — everything in turn 1.** The earlier shape, with the two fixes: the cut given as
  independent, the sign-off given. Turn 2 is the same go-ahead and is idle unless turn 1 stopped.
  Turn 1 would end: *"Record this as the project's next milestone with those three pieces under
  it, then have all three worked at the same time, each kept apart from the other two, and land
  them together as the one milestone."*

**Recommendation: A.** It gives the second turn work to do in every case but one, it puts the
request to run the pieces side by side at the start of a turn instead of at the end of a long
planning arc, and its trigger is a state. Its cost is one unnatural sentence — *stop there for the
moment* — and a worker that ignores it turns A into B, which is still scored.

**Taken: as recommended** — option A, by the human's explicit decision to run the prompts as drafted (`paste/a-turns.txt`).

## F2 · Do *milestone*, *finalize* and *architecture document* stay in the prompts?

**What was found.** The blindness rule says no prompt names a jigc verb, workflow, doctype or
flag. Three words in the drafted prompts are ordinary product words that are also jigc vocabulary:
*milestone* (arm a — also a verb), *finalize* (arms b and c, in "finish with a clean finalize" —
also a verb), and *architecture document* (arm c — the doctype is `arch-doc`, the workflow
`architecture-documentation`). RC-m50 and RC-rc14 used all three, and also *roadmap*, *the
project's planning workflow* and *boundary*, which the draft drops.

**Why it forces a decision.** "As the RC-rc14 paste files do" and "names no jigc verb" pull in
different directions on exactly these words.

**Recommendation: keep all three.** Without *milestone* arm (a) measures whether a worker invents
the concept, not whether it can run the fan-out; no other product word leads to that work-unit.
*Finalize* sits beside the one channel statement the earlier trials carried, in the same closing sentence they used.
*Architecture document* is what a person would call it. The draft carries none of *fan out*,
*amend*, *report*, *inconsistency*, *worktree*, *provision*, *sub-task* or *parallel*; that was
checked mechanically.

**Taken: as recommended** — by the orchestrator; all three words stayed in the turn files.

## F3 · Should one arm run `jigc setup` itself, so the install commit is checked for the trailer?

**What was found.** The trailer is added only while `CLAUDECODE` is set in jigc's own process.
All three arms start from corpora adopted by `arms/adopt.sh`, which runs outside Claude Code — so
the install commit lands without a trailer, by design, before any session. An arm that ran
`jigc setup` itself would put that commit inside the session and under the check.

Three costs, each read from the tooling or an earlier record:

1. **The prompt would have to name jigc commands, or lose the log.** The invocation log is off
   until `jigc config set invocation-log true` runs. RC-rc14's B1 prompt named both `jigc setup`
   and that command. A prompt that names neither gets no log, and without the log the arm is void
   and its rubric has nothing to read.
2. **The adapter is not in the worker's context in the turn that runs setup.** RC-rc14 reported
   B1 separately and discounted it for this reason. For arm (b), that is the turn that lands the
   commit; for (a) and (c) it is the measured turn.
3. **It answers nothing the arms do not.** What only a live session can show is whether the
   headless agent's processes carry the variable. Every commit jigc makes in-session shows that.
   The install commit is one more door over the same signing function, and
   `crates/cli/tests/agent_co_author.rs` sets the variable per cell for it.

**Recommendation: no — all three arms start adopted.** The install commit stays in the check as
its **control**: made outside the agent, expected to carry no trailer, and a trailer there would
be a false co-author. If the human wants that door under a live agent anyway, the cheap form is a
fourth, **unscored, non-blind** headless turn on a naive corpus with RC-rc14's B1 opening
sentence, read for the one commit and nothing else.

**Taken: as recommended** — by the orchestrator; all three arms started adopted and the install commit is the trailer check's control (README.md → The trailer check).

## F4 · Is an arm rehearsed against a live agent before it is scored — and is a void arm re-run?

**What was found.** The postmortem's rule 4: the assumption a design flags as its largest is paid
for before the trial. This design's largest is that turn 1 of each seeded arm leaves the state
turn 2 needs. For (b) that is narrow and decides everything: if turn 1 lands no commit, or two,
or puts the ticket somewhere other than the commit at `HEAD`, the amend door cannot reach it and
the arm is void. For (a) a rehearsal *is* the arm — same cost, and a second draw at n = 1.

**Why it forces a decision.** A rehearsal costs a session and a corpus, and at n = 1 it is tempting
to read as data. RC-rc14 ran its rehearsals as preconditions and reported them separately.

- **Rehearse (b)'s turn 1 only** — one headless turn of line 1 of `paste/b-turns.txt` on a spare
  clean corpus, read for three things: one commit, `TKT-212` in its message, nothing of it in the
  tree. Unscored.
- **Rehearse nothing**, and rely on the re-run rule below.

**Recommendation: rehearse (b)'s turn 1; do not rehearse (a) or (c).** And, drafted into
[protocol.md](protocol.md) §2.4 for the human to confirm: **no arm is re-run because of its
outcome; a void arm is re-run once on a fresh corpus, and both runs are reported.**

**Taken: as recommended** — by the orchestrator. **As run, read from the out-dirs and the operator's logs by the record's assembler:** no rehearsal of (b)'s turn 1 exists — no out-dir, no spare corpus, no log step. The re-run rule stood and was not needed (no arm was void). Arm (b)'s scored turn 1 left the occasion the rehearsal would have paid for: one commit, `TKT-212` on it at `HEAD`, nothing of it in the tree (README.md → Honest bounds).

## F5 · Is each session debriefed, and how does arm (c) get one?

**What was found.** Every earlier trial took a per-session feedback report, and twice it found
what the reader had missed — a read in RC-m50, a rewritten commit in RC-rc14. Headless has no
channel for one inside a run. `run.py fork` resumes a frozen conversation for one more turn in a
new out-dir, which is a debrief that cannot touch the scored evidence. It exists for (a) and (b)
because `seed` freezes them. Arm (c) is a single prompt through `run-session.sh` and has no frozen
conversation.

**Why it forces a decision.** The trial's question is about usability, and *what confused you* is
the worker's half of that answer. But "two-turn" and "one prompt" are decided shapes, and a debrief
is a further turn.

- **Debrief (a) and (b) by `fork`, after scoring**; `paste/debrief-prompt.txt` is drafted and
  [runbook.md](runbook.md) §9 has the commands. It contains the word *read* and is never delivered
  into a measured turn.
- **To debrief (c) too, run it through `seed` as a one-line turns file** — the same `claude -p`
  underneath with a fixed session id. `paste/c-prompt.txt` is one line and passes the seed's
  constraints as written. The cost is that (c) then also depends on the repaired `seed`.
- **No debrief.** The final message of each turn is the only account.

**Recommendation: debrief all three**, with (c) through `seed` **if the seed smoke passes** and
through `run-session.sh` without a debrief if it does not. The draft has (c) on `run-session.sh`;
taking this changes [runbook.md](runbook.md) §5.1 to a `seed` call with `~/out/RC24-C-frozen`.

**Taken: as recommended** — by the orchestrator. **As run, read from the out-dirs by the record's assembler:** (a) and (b) were debriefed by `fork` after scoring; arm (c) ran through `run-session.sh` (`~/out/RC24-C`, no frozen conversation) although the seed smoke passed, so it has **no debrief** — the recommendation's fallback branch, not its first one. T-12 is where that absence costs (README.md → Honest bounds).

## F6 · If the probe says the headless agent does not carry `CLAUDECODE`, does the trial stop?

**What was found.** Nobody knows whether `claude -p` in this image sets `CLAUDECODE` for the
commands its agent runs, or for a sub-agent's. If it does not, jigc adds no trailer to any commit
in any arm, and the requirement — every commit jigc makes in the trial carries it — cannot be met
on this transport whatever jigc does. The draft adds a one-turn probe before the arms; it prints
`SET` or `UNSET` and never a value.

**Why it forces a decision.** The probe is an addition to what was decided, and its worst reading
needs a rule fixed before it is seen.

**Recommendation: run it, and stop on `MAIN-UNSET`.** Three arms would each land the same defect
row, and the human would want to choose first between fixing the gate (a fix pass on the
`when-env` condition, then a new candidate) and accepting the trailer as unmeasurable headless.
On `MAIN-SET` with `SUB-UNSET`, go on: only a commit a sub-agent's jigc call makes is affected,
and it is recorded as its own row.

**Taken: as recommended** — by the orchestrator; the probe ran before any arm and read `MAIN-SET` / `SUB-SET`, so the stop did not fire.

## F7 · Arm (c): does the prompt ask what the service *hands on*?

**What was found.** The standing disagreement is that the prose says the service sits in front of
a long-term store and the code hands nothing on. The drafted prompt asks for an architecture
document covering, among other things, *"what it hands on to anything outside itself"*. That
clause guarantees the worker has to answer the question the disagreement is about. It does not say
anything disagrees.

**Why it forces a decision.** It is the nearest thing in the trial to a hint. RC-rc14's plainer
wording — *"how ingest, store and rollup fit together"* — would be blinder, and a worker could
then write the document without ever asking where samples go, landing in *missed* or void and
measuring nothing about the channel.

**Recommendation: keep the clause.** The arm's question is what a worker does *on meeting* a
disagreement, so the meeting has to happen; what an architecture document hands on is a thing a
person would ask for. The alternative is to drop from *"and where the service's edges are"* to the
end of that sentence, and accept a higher chance of no occasion.

**Taken: as recommended** — by the orchestrator; the clause stayed in `paste/c-prompt.txt`.

## F8 · Arm (b): does turn 2 rule out a second commit?

**What was found.** `jigc task amend` repairs the message of the commit at `HEAD` and nothing
else. The drafted turn 2 says the wrong ticket should be corrected *"on that same commit, so the
history holds one commit for this fix"*. That makes a second commit a wrong answer, and makes the
amend door the designed route.

**Why it forces a decision.** A looser turn 2 — *"it should be TKT-221"* — leaves a follow-up
commit as a reasonable reading, and then B-2 is not a miss but a different, defensible answer. The
tighter one states the outcome a git user would hear as `git commit --amend`, which is exactly the
reflex the adapter's sentence has to redirect.

**Recommendation: keep the tight wording.** It names no means and no jigc word, *nothing has been
pushed* removes the one good reason to refuse, and it is the only wording under which the arm
measures the door it was chartered for.

**Taken: as recommended** — the tight wording, by the human's explicit decision to run the prompts as drafted (`paste/b-turns.txt`).

## F9 · Where does the declared-changes list live?

**What was found.** Every earlier trial's protocol pointed at a list in
`implementation/decisions-pending.md` and did not restate it. That file has no list for M51–M55 —
the blind-trial entry there names the surfaces and no changes. The draft's §0 therefore carries the
list itself, derived from the M51–M55 spans of `implementation/project-history.md` and from
driving rc.24, each item with its source.

**Why it forces a decision.** *Cross-reference, never restate* wants one home, and the protocol
could not write to the repository.

**Recommendation: the protocol is the home**, and `decisions-pending.md`'s blind-trial entry gets
a one-line pointer to it when the trial's directory is committed. The list was derived for this
instrument, on one day, against one binary; a dated protocol is the right place for it, and adding
it to a shared log would open a second branch on that log.

**Taken: as recommended** — by the orchestrator; [protocol.md](protocol.md) §0 is the list's home. The one-line pointer from `implementation/decisions-pending.md` is owed to the commit that lands this directory: the record's assembler wrote nowhere outside it.

## F10 · The directory's name and layout, and the two helpers' home

**What was found.** The draft is in a scratch directory, with `gate-rc24.json` one level above
`protocol/`. `completions/trial-driver/test_cascade.py` reads every
`completions/artifacts/RC-*/protocol.md`; the draft registers the read-back table unchanged and
was checked against the code, so it passes wherever it lands. Two small helpers were written for
the by-hand checks — `tools/trailer-rows.py` and `tools/raw-git-acts.py` — because the brief was to
use the existing tooling unchanged.

**Recommendation.** `completions/artifacts/RC-rc24/`, on RC-rc14's layout: `protocol.md`,
`runbook.md`, `corpora.md`, `paste/`, `gate-rc24.json` and `evidence/` side by side, with
`open-forks.md` kept as the record of what was decided and how. The helpers ride in the trial's
directory for this trial. **Afterwards** `raw-git-acts.py`'s pattern belongs in
`driver/observe.py` as a fix with a test (F12), and `trailer-rows.py` belongs in
`completions/trial-driver/` — a tool nothing points at gets rebuilt.

**Taken: as recommended** — by the orchestrator, with this file renamed `forks.md` as the record of what was decided. The two helpers stay in this directory's `tools/` and were **not** moved into the driver; `raw-git-acts.py`'s pattern and `trailer-rows.py`'s home remain owed (README.md → Tooling findings, T-4, T-9, T-10).

## F11 · Does the rest of the walk run?

**What was found.** `walk.py` has 24 arms. Arm 00 is the positive control. Arms 01–23 were written
for rc.11 through rc.14 and none was re-derived for this binary; four of them are two upgrade
pairs against images this trial does not build, and in RC-rc14 two went red on a declared change
alone.

**Recommendation: arm 00 only.** Re-deriving 23 arms for rc.24 is a regression net, and the
partial re-review over M54's and M55's axes is the defect instrument on this binary. The walk
record lists the 23 as `NOT RUN`, which is what the record is for.

**Taken: as recommended** — by the orchestrator; walk arm 00 only, and `evidence/walk/walk-record-rc24.md` lists arms 01–23 as `NOT RUN`.

## F12 · Is `observe` patched before the trial, or after?

**What was found, by driving it.** `observe`'s `git!` lines read the first non-flag word after
`git` as the subcommand, so `git -C /work commit --amend` and `git -c user.name=x commit` are not
reported — two of five shapes were caught. jigc's own printed routes lead with `git -C`, so a
worker on rc.24 is primed to type exactly the shape the detector misses. The reflog check is exact
and unaffected. Separately, and already decided: `observe` reports a correct `jigc task amend` as
`HISTORY REWRITTEN`, and is not patched for it.

**Recommendation: after.** The trial does not depend on the `git!` lines — the reflog, the trailer
join and `tools/raw-git-acts.py` cover the same ground — and changing a reader between its
positive control and its sessions is how an instrument gets a defect nobody rehearsed. The fix,
with a test over the five shapes, is a `work/` change once the record is written.

**Taken: as recommended** — by the orchestrator; `observe` was not patched before or during the trial, and the fix with its test is still owed (README.md → Tooling findings, T-1, T-3).

---

## Not forks — three things driving turned up

Recorded here so they are not lost. Each was driven on the installed registry
`jigc 1.0.0-rc.24`; none is graded, and each is a candidate row for the trial's `README.md`.

- **Orientation prints the project's config directory as an absolute path.** `jigc start` opens
  with `Project config: <absolute path>/.jigc/config`. `.jigc/AGENT.md` says every printed path is
  repo-relative except two kinds — a backticked command to run, and a path naming a checkout that
  is not the repository root. This is neither. In the container it reads `/work/.jigc/config`, so
  no host path reaches a worker.
- **Orientation over an open, unstarted milestone names no door that starts it.** Each sub-task is
  listed as an active task with `resume`, `validate`, `milestone finalize` and `discard`. Neither
  `jigc milestone provision` nor `jigc milestone execute` is named there; `provision` is reached
  through the resume door's refusal, `execute` and its `Spawn:` lines only through
  `jigc milestone --help`. Arm (a) will show whether that matters.
- **The pre-commit hook writes to the invocation log.** With the log on, every commit — jigc's or
  raw — is preceded by a `validate --format json` record the hook made. An observer counting a
  worker's `validate` calls is counting the hook's too; the read-back channels do not key on it.
