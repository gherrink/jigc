# Session findings — running notes, as they were driven

**Not the adjudication.** [findings-verification.md](findings-verification.md) is where each
claim gets its repro block and its `pinned-by:`; [trial-record.md](trial-record.md) is where the
adjudication lands. This file is what was observed, in the order it was observed, so a later
reader can see which corrections were made *before* a number was written down.

---

## The walk — 22 arms, and four of them red on the first pass

First pass, `~/out/RC14-walk`, `jigc-gate:rc14`: **arms 07, 08, 17, 18 red**.

RC-m50's walk had **02, 15, 17, 18, 20** red *by their own design*. So on the face of it three
arms went red→green (02, 15, 20 — M50's fixes landing) and **two went green→red (07, 08)**,
which is the shape of a regression and was treated as one until it was driven.

### 07 and 08 were the instrument meeting a declared change — driven, both sides

The cascade started at one line. Arm 08's `land_adr` returned an **empty address** for its
second document, and every downstream bar addressed `` — including a `jigc rename ` with no
argument, whose refusal the arm then scored.

Root cause: both arms clean up with `jigc task discard "$T"`. Driven on both binaries, on a
freshly minted task carrying nothing but its transient commit doc:

```
rc.13:  discarded task revise-the-overflow-policy — dropped staged edits to: … (transient)   [exit 0]
rc.14:  blocking · task-discard.staged-prose — task `revise-the-overflow-policy` stages 1 doc(s)
        that no commit has a copy of — discarding it would destroy them: commit:…
          route: read what is in them with `jigc doc show <address> --task …`, or land them with
          `jigc task finalize …` … or … `jigc task discard … --force` removes the working area
          with them                                                                          [exit 1]
```

That is **M50 Increment 3, declared**, and Increment 13's guide correction states the very fact
that makes it bite here: *minting a task stages its commit doc, so an ordinary discard hits the
refusal from the moment the task exists.* The arms were written before it, their cleanup was
refused, stale tasks stayed live, and `newtask()` — which takes the first task in the list —
started handing later steps the wrong task.

**Judged against the protocol's four-part standard for a declared change, it reads as designed:**
(a) it names what it objects to, by doc identity; (b) it says the consequence — *discarding it
would destroy them*; (c) it names three routes **in the same output**; (d) the consent route runs
verbatim — `jigc task discard <id> --force` → exit 0 — and so does the read route it offers
**first**, `jigc doc show <address> --task <id>` → exit 0.

Repaired at the seven **cleanup** sites across four arms, never at a subject under test (arm 17's
cell driver and arm 12's `step` drive `task discard` deliberately). Re-run:

```
07-changelog-gate      13 OK / 0 FAIL     (was 12 / 1)
08-identity-refusals   17 OK / 0 FAIL     (was 12 / 5)
```

**This is the §0 briefing paying for itself.** An unbriefed observer scores two green→red arms
as regressions and spends the trial's budget re-deriving a decision this repo already took —
the near-miss the ledger says has happened twice.

### 18 was the arm's expectation being stale — and the result is a closure

Arm 18 asked four write verbs for `write.unknown-section` on a section-shaped miss. `set-field`
answers **`write.unknown-field`**, and it is right to: on `set-field` a single-hop `#x` *is* a
field address, and the message says so rather than asserting an absence —

```
blocking · write.unknown-field — no field "nosection" declared on any section of `adr`
  (the single-hop `#<field>` form searches every declared section)
  at: adr:probe-decision#nosection
  route: `jigc doc schema adr` to see the declared shape, then re-run the write at a declared address
```

Code, `at:` locus, route — and the route runs verbatim at exit 0. **That closes RC-m50's W-15**
(*"the section-only `set-field` miss is bare"*), which shipped recorded into M50. The bar now
asks each verb for the code it owes; re-run: **66 OK / 0 FAIL**.

### 17 is the wave's own centrepiece, measured — and it splits cleanly

Arm 17 enumerates 25 id-taking doors × {`""`, `no-such-task`}. It was **expected RED on rc.13**
and found that trial's blocking finding. On rc.14: **86 OK / 22 FAIL**, and **every failure is
in the `[nonexistent]` column. The `[empty]` column is 53/53 green.**

```
EMPTY id      blocking · work-unit.malformed-id — "" is not a valid work-unit id
                route: use lowercase letters, digits, and single hyphens (…)          [exit 1]
UNKNOWN id    no task `no-such-task` — list live tasks with `jigc task list`          [exit 1]
```

So M50's fix landed exactly where it was aimed. What the arm's own diagnostic calls
`exit 1 NOCODE route` is the **sibling axis**: 22 doors refuse a well-formed but unknown id
correctly — no false green, no panic, and **a route** — but carry **no finding code**.

**Not a regression, driven both sides:** rc.13 and rc.14 answer identically (`NOCODE route`,
exit 1) at `task validate`, `task diff` and `milestone list-tasks`. And it is **declared** — the
ledger says in as many words that *a well-formed but unknown id keeps its unchanged roster
answer, so only the malformed cell moves.*

It is still a **surface-tier finding candidate**: a driver keying on the stable `(code, target)`
pair gets nothing when it names a task that does not exist, at 22 doors. Adjudication is
[trial-record.md](trial-record.md)'s, against §1.

### 22 — the new arm, and the two carried defects reproduce

`22-m50-orientation-and-carried.sh` iterates `engine::result::OrientationView` by driving the
states that produce each variant. **21 OK / 5 FAIL, and all five are the pre-registered
expected-RED measurements.**

Green: a repo with no task is `clean`; the envelope carries `schema_version` **3**; a repo
holding a live task is **not** reported clean and *is* the `active-task` view; the view names the
task, its `staged:` identities, and `findings:`; it routes at resume, validate and finalize, and
its abandon route carries **`--force`**, the consent that door now needs. The minting form
appends `also open:`, names the open task, and does not suppress the mint.

**Two observations the arm records rather than grades — and they are the headline's mechanism,
pre-registered in [protocol.md](protocol.md) §3.1 before any of this ran:**

```
NOTE  the active-task view names NO read verb — resume/validate/finalize/discard only
NOTE  the also-open block carries RESUME ONLY — the four directives are the orientation view's
```

Both confirm handover correction **C3** against the binary.

Red, as registered:

- **N27** — `jigc task diff <id>` cold-start names neither the workflow nor the intent, in text
  **or** `--format json`. Four bars. Its recorded trigger is this trial's plant-E arm.
- **N15** — the task-scoped miss still claims the address names no **committed** doc. (One N15
  bar passed: the two forms are no longer byte-identical.)

## Instrument corrections made before any figure was written

Every one was found by **running**, not by reading — the pattern this apparatus keeps producing.

| # | what | where |
|---|---|---|
| 1 | thirteen reader tests had never executed (a stray mid-file `unittest.main()`) | `test_observe.py`; PT-2 |
| 2 | the reader read one transcript, and said nothing about whose | `run.py`, `observe.py`, `session.py`; PT-3 |
| 3 | seven walk cleanup sites used a discard form rc.14 refuses | four walk arms; above |
| 4 | one walk arm asked the wrong verb for the wrong code | `18-surface-batch.sh`; above |
| 5 | one bar of the new arm called an untracked config file a dirty tree | `22-…`; narrowed to tracked files |
| 6 | the FILESYSTEM channel counts attempts, not reads | recorded, not fixed mid-trial; PT-7 |
| 7 | **there was no write channel at all** — a `git reset` + manual `git commit` over a managed doc scored clean | `observe.py`, `run.py`, `test_observe.py`; F-9/F-10 |

**#7 is the one worth reading twice.** The reader had a FILESYSTEM channel for *reads* and none
for *writes*. The trial's only adapter bypass was therefore invisible to it, and surfaced because
the worker volunteered it in a debrief and a human then read the record — **the second time this
apparatus has learned the same lesson on the same axis**, after `AFindPipedIntoCatIsARead` was
added because a duress *read* was found the same way. Reads were fenced, writes were not: the
incomplete-fix shape this trial keeps finding in the product, found in the instrument.

Fixed in the same motion: `commit_writes()` (transcript, heuristic) + `history_surgery()`
(**corpus reflog, exact — independent of the worker's account of itself**), both walked over the
subagent set, fenced against **B1's archived transcript** rather than a mock. It catches both acts
on the real case, is silent on `jigc task finalize`, silent on `git add`/`restore`/`status`/`log`,
and reports nothing on the four clean arms or the 1.0.0-gate archive.
