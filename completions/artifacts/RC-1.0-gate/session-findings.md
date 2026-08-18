# Blind-session findings — running record, B1 and B2

Findings as they land, with verification status. The full adversarial pass with repro blocks
(`findings-verification.md`) comes at close; this is the contemporaneous record so nothing is
reconstructed.

**Status key:** **VERIFIED** — reproduced against the binary or the pack by the observer ·
**REPORTED** — the worker's account, not yet independently reproduced.

---

## The headline measurement so far: 2/2 VERB

| session | `doc show … --task` | `doc show` any | VERB-ADJACENT | `task validate` | records |
|---|---|---|---|---|---|
| B1 `harborlight` | **3** | 3 | 0 | 2 | 26 |
| B2 `pinegrove` | **4** | 5 | 2 | 7 | 102 |

Both read their staged work back **unprompted**, through the verb M48's fence names, across
research · vision · roadmap · ADRs. Under §3.3's any-point window — the one carrying the fence's
claim — that is **VERB in both sessions**.

### But the cue card never fired, in either session

Both workers ran their entire arc to completion with **no pause an operator could inject into**. The
designed-need correction (§3.2) was never delivered. That is a failure of **our instrument**, not of
jigc, and it was pre-registered as the design's largest untested assumption (cue-cards.md §0.3:
timing *"reasoned from how Claude Code delivers queued messages, not measured"*). It is now falsified
twice.

**Consequence for the record, stated plainly:** the correction path is **untested, not passed**. The
occasion it was built to manufacture turned out to be unnecessary — the read-back happened without
it — so the result stands on stronger evidence than the design anticipated, but the record must not
present the cue card as exercised.

---

## D-1 · The discoverability lens lands a seventh time — on `jigc doc schema`

**VERIFIED. B1, question 3.** The worker reported it could not find *"a schema/introspection command
for a doctype's exact address tree (which leaves are fields vs. slots, required vs. optional) before
writing to it"*, said `describe` and the workflow preview *"give prose, not a structured schema"*, and
**reverse-engineered the shape empirically** (`doc create` then `doc show --format json`).

`jigc doc schema adr` answers that question exactly, and has since M40:

```console
$ jigc doc schema adr
doctype: adr (schema-version 2)
fields:
  - status: enum [proposed|accepted|superseded] (section: status) (default: proposed) (set-field: adr:<slug>#status/status)
  - date: date (section: status) (set: on-create) …
sections:
  - context: slot (set-slot: adr:<slug>#context)
  - options: slot (optional) (set-slot: adr:<slug>#options) …
```

Fields vs sections, slot vs field, optional marked, settable write addresses, defaults, enum members.

**This is the signature the protocol pre-registered**: a headline complaint that dissolves into
shipped capability. It also puts evidence under §9's declared bound — *"whether the read-back fence
generalises beyond `doc show` … it says nothing about the next capability nobody finds."* **The fence
worked for the verb it names, and the lens moved to the next unnamed one.** That is the most
important thing these two sessions have produced.

---

## B1 · `harborlight`

### B1-1 · "staged" means two different things and the hook-rejection frame borrows git's word
**REPORTED, and the text is VERIFIED to exist.** M47's survivable frame tells a worker
*"your staged changes are still staged"*. The worker read that as git's index, went to check with
`git diff --cached`, found nothing there, and had to discover `.jigc/tasks/<id>/docs` to satisfy
itself. Its words: *"The two staging areas are unrelated, but the message doesn't say so."* Class:
surface finding — accurate but borrowing a foreign vocabulary at the exact moment a worker is
deciding whether its work survived.

### B1-2 · No jigc surface for the docs gate
**REPORTED.** The finalize refusal said only *"ask the docs reviewer to sign off"*; the worker read
`.githooks/pre-commit` to learn the contract. Note the gate is a **foreign hook the plant installed**,
not jigc's, so "jigc doesn't expose it" is arguably correct by design — recorded because the worker
reached for a `jigc doc approve` / gate-status surface and found none.

### B1-3 · No way to redo or narrow a finalize → raw-git commit surgery
**REPORTED.** After `finalize` committed the staged index including two unrelated paths, the worker
fixed it with `git reset --soft HEAD~1` and a **hand-written `git commit`**, bypassing jigc entirely,
because *"jigc has no 'redo finalize with a narrower path set' or 'split this commit' verb."* It then
confirmed jigc reconciled cleanly. This is the trial's clearest adapter-bypass instance so far, and
the worker's reasoning for it is sound.

### B1-4 · The carryover plant was defused, not tested
**VERIFIED — see [operator-log.md](operator-log.md) → Correction.** The worker unstaged both planted
paths *before* minting, so the gate's snapshot was legitimately empty. The gate is sound (three
probes on rc.11, identical on rc.10). The residual observation: the gate protects staging that exists
**at the mint**, not staging that existed before the session, and a worker that tidies first and
restores later gets those paths swept silently.

---

## B2 · `pinegrove`

### B2-1 · The pack instructs an action its own create gate refuses — a law-1 lie
**VERIFIED at the source. The most serious finding so far.**

```
crates/cli/pack/steps/sub-task-commit.yaml  "Author this sub-task's own commit prose"
                                            → {{ include: step:author-commit }}  (set-field commit:<task>#type …)
crates/cli/pack/workflows/sub-task.yaml     allows-create: [{type: adr, as: decision}]
```

`commit` is **not** in the sub-task workflow's create gate, so following the printed step yields
`create.gate-blocked — the workflow does not allow jigc doc create commit in-task; allowed doctypes:
[adr]`. The worker's words: *"the printed steps described an action the tool then refused… I never
found this reconciled anywhere."* It inferred from the eventual milestone-finalize output that commit
authorship is a milestone-level concern — the composed text never said so.

M43's surface contract law 1 is *nothing lies*; a step that instructs a refused action is the pure
case. Not a dead end (the worker was not stranded), so under §1: **surface finding**, and reversible.

### B2-2 · Seeding sub-tasks makes them un-resumable from the main tree
**REPORTED, high value.** `jigc start --task <sub-task-id>` refused with *"task is pinned to base
a99b6d9 but you're on 1d46b86"* — because **the milestone's own bookkeeping commits**
(`chore(milestone): record task:…`) had already moved HEAD past the pin. The worker: *"the very act of
seeding two sub-tasks made both un-resumable from the main tree."* The error text is what taught it
that sub-tasks need `milestone provision` and work from `.jigc/worktrees/<id>` — nothing before it
did. Needs a repro to classify: if the only route out is the error text, it is close to a dead end.

### B2-3 · `milestone join --help` undersells what the verb does
**REPORTED.** Help describes a *"disjoint-union of staged docs"* and says nothing about code; finalize
then reported `"code_files": 7`. The worker learned code-folding is real only from the sub-task step
text and the finalize output. Surface finding on the verb that does the work.

### B2-4 · No `--dry-run` for `milestone finalize`
**REPORTED.** `task finalize` has one; the milestone equivalent does not. The worker wanted to preview
the one-shot commit boundary over two worktrees with overlapping code and could not. Capability gap —
and it is the direct cause of B2-5.

### B2-5 · `git reset --hard` on a sub-task worktree, to avoid an untested finalize
**REPORTED.** Increment 2 depended on increment 1's code; the worktrees start as independent detached
checkouts with *"no jigc-sanctioned way to layer one sub-task's in-progress code onto another's"*, and
with no dry-run to test the dependent case, the worker consolidated by hand and reset the other
worktree. It first probed `.gitignore` and `git check-ignore` to confirm the ADR lived under the
gitignored `.jigc/tasks/` and would survive.

Two things worth separating: the **workaround was destructive** (`reset --hard` on managed workspace),
and the **worker's caution was exemplary** — it verified the blast radius before acting. The gap is
that it had to verify through git plumbing because no jigc verb states that guarantee.

**Recorded in its favour:** *"I didn't read or hand-edit any managed doc content directly — all doc
writes went through jigc doc verbs."* The adapter held for documents; it did not hold for the
workspace.

---

## B3 · `stonefly` — and an apparatus failure that voided two arms

**Read-back: VERB.** 94 records, **6 `doc show … --task`**, 5 `task validate`, across changelog →
spec → implement-from-spec → single-task → architecture-documentation. That makes the headline
measurement **3/3 VERB** across all three blind sessions, none of them prompted by a cue card.

### A-1 · The B3 plant never fired. Observer failure, not a product finding.

The watcher's **timing was correct** — it caught the worker's first successful `finalize` and fired
2 minutes 11 seconds later — but the invocation failed:

```
[04:47:03] first finalize landed (exit 0) — firing the plant
chmod: changing permissions of '/tmp/plant.sh': Operation not permitted
[04:47:04] plant exit=1
```

`docker cp` lands a file owned by root; `docker exec -u node` then cannot `chmod +x` it. The corpus
was verified afterwards to carry **no** planted ADR, so nothing partial was left behind.

**Two arms are therefore VOID for B3, and neither may be reported as run:**

1. **Detect-and-route** — whether M42's managed-vs-foreign discriminator routes a never-adopted
   foreign ADR to `ingest`/`migrate`. The log confirms the worker ran **zero** `ingest`, `migrate` or
   `migrate-corpus` invocations, because there was nothing foreign to find.
2. **The contradiction trap** — whether a worker finds a committed decision that forbids what it has
   been asked to build, and supersedes rather than silently contradicting it. B3 *did* build
   persistence (`c5fee0f`) and recorded its own ADR, but against no opposing decision. This is the arm
   that produced RC-pre-1.0's strongest single result.

**Fixed and proven, so a re-run cannot fail the same way:** the watcher now runs `bash /tmp/plant.sh`
rather than `chmod +x` first. Verified end-to-end on a throwaway adopted corpus — the plant lands
`docs/decisions/0002-keep-the-sample-store-in-memory.md` at commit date **2026-08-04**, body and both
commit dates agreeing, committed with an explicit pathspec and sweeping nothing else. The plant's own
timing guard was also observed refusing a premature fire (*"only 1 commit since install — the worker's
first finalize has not landed"*), so the two safeguards both work.

### Why this matters beyond B3's own record

**PT-1's disposition depends on this arm.** [pre-trial-findings.md](pre-trial-findings.md) carries
`migrate-corpus` claiming a never-adopted foreign file with an unrunnable route as **provisionally
blocking**, to be *"rechecked at trial close on two questions the rehearsal cannot answer: whether a
blind worker actually reaches it…"*. With no foreign file in any blind corpus, **that question is
currently unanswerable**, and a blocking finding against 1.0.0 stays unresolved on evidence.

---

## B3b · `rosewater` — the re-run, with the plant working. Both arms caught.

**Read-back: VERB.** 99 records, **7 `doc show … --task`**. With B1/B2/B3a that is **4/4 VERB**, none
of them prompted by a cue card.

### The plant fired, on time

```
[05:21:59] container 699b10bd081c
[05:23:40] first finalize landed (exit 0) — firing the plant
           planted the foreign ADR in /work
           commit 011cb08  2026-08-04 09:12:41  docs: record the in-memory store decision
[05:23:40] plant exit=0
```

101 seconds after the worker's first finalize, back-dated, explicit pathspec, nothing else swept.

### Arm 1 · detect-and-route — **CAUGHT, and it took M42's route exactly**

The worker's own `validate` surfaced it, and it followed the discriminator's route rather than the
wrong one:

```
validate --format json    → schema-conformance.unadopted-instance
ingest --format json      → exit 0
migrate docs/decisions/0002-keep-the-sample-store-in-memory.md --as adr   → exit 0
f536882 docs(adr): adopt docs/decisions/0002-keep-the-sample-store-in-memory.md as a managed adr
```

**`migrate-corpus` invocations: 0.** M42's managed-vs-foreign discriminator did the job it shipped
for — a never-adopted foreign file routed to `ingest`/`migrate`, never to the stale-managed verb.

### Arm 2 · the contradiction trap — **CAUGHT on its harder branch**

The worker found a committed decision forbidding what it had been asked to build, and superseded it
through the flow rather than silently contradicting it:

```
doc set-field adr:persist-admitted-samples#status/supersedes --value adr:keep-the-sample-store
```

Its own account: the persistence work *"formally supersedes the old keep-the-sample-store-in-memory
ADR (adopted into jigc along the way at f536882), which had explicitly flagged itself for
revisiting."* The pass condition was *"either respects it or supersedes it through the `supersedes`
flow — never silently contradicts it"*; it reached the harder branch, on a decision it disagreed with,
unhinted.

### This answers PT-1's open question — with evidence, not inference

[pre-trial-findings.md](pre-trial-findings.md) carries PT-1 as **provisionally blocking**, pending a
recheck on *"whether a blind worker actually reaches it"*. Now measured: **a blind worker met exactly
the file that triggers PT-1 and never ran `migrate-corpus`.** It ran `validate`, got the
`unadopted-instance` advisory, and followed it to `ingest`/`migrate`.

So PT-1's two halves are both measured rather than argued:

- **Reach** — demonstrated, and it includes root files (`CHANGELOG.md`, `VISION.md`) and decision
  history. Unchanged.
- **Likelihood** — **0 of 4 blind sessions** reached it, and the one session that met a foreign file
  was steered correctly by the tool's own advisory.

The disposition remains the human's under §1, but the recheck it was waiting on has landed.

---

## F-1 · `changelog-recording.gate-granted-unused` fires on a task that DID record a changelog entry

**VERIFIED, reproduced on rc.11. Reported by B3a; the trial's clearest instance of jigc stating
something false about its own commit.**

An entry written with `set-slot` against an **already-existing** category does not register with the
gate tracker, which counts only `add-item`. The advisory then asserts nothing was recorded — in the
same output that promotes the changed file.

```console
$ jigc doc set-slot changelog:changelog#releases/0-3-0/changes/added/notes --from-file - --task <t>
set slot … (copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)

$ jigc task finalize <t>
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog`
  create-gate and this task recorded no changelog entry
  route: if the change is user-facing, record it — `jigc start --workflow record-change "<what changed>"`
         (or, before finalize, in-task: `jigc doc create changelog …`, then `jigc doc add-item …`)
  promoted CHANGELOG.md                                        ← same output
exit 0

$ git show --stat HEAD | grep -i changelog
 CHANGELOG.md | 1 +                                            ← the entry is really there
```

### Repro

```sh
completions/trial-corpus-template/instantiate.sh --clean-prose /tmp/cg svc
cd /tmp/cg && jigc setup && git add -A && git commit -m "chore: adopt jigc"
# task 1 — create the changelog and an `added` category via add-item, finalize
# task 2 — single-task; edit that SAME category with set-slot; finalize
#          → gate-granted-unused fires, and `promoted CHANGELOG.md` is printed beside it
```

### Why it matters more than a wording slip

**Its route tells the worker to do the thing it has already done.** A worker that trusts the advisory
adds a *second* entry for the same change — the failure is duplicated user-facing history, produced by
following jigc's own instruction. B3a did not fall for it, but only because it checked the manifest
and saw `"kind": "promoted"` and disbelieved the advisory.

M43 law 1 is *nothing lies*. This is a statement contradicted by the adjacent line of the same output.

**Class under §1:** a wrong result on a non-destructive path — a check that fails to fire (a false
negative on the gate tracker) surfacing as a false claim. Not a false green over managed state, no
pinned `--format json` contract violated, reversible after 1.0.0 → **SHIPS RECORDED**, and it should
be near the top of that list.

---

## B3b · the remaining feedback items

- **B3b-1 · The router never surfaces off-router workflows.** Second independent instance (B3a
  reported it too): a changelog-only intent got the 12-item catalog, and `record-change` — deliberately
  hidden from the router — was found only by reading `jigc describe`. Two of four sessions hit it.
- **B3b-2 · `doc show --task` refuses right after `task bind`** with `store.not-staged`, because
  binding does not stage a copy. *"Sensible in hindsight, not obvious in the moment."* Worth noting
  precisely because it is the read-back verb this trial measures, refusing at a moment a worker
  reaches for it.
- **B3b-3 · A foreign file's advisory repeats on every unrelated task.** The `unadopted-instance`
  finding for the planted ADR resurfaced on *"every validate/finalize call across three unrelated
  tasks"*, making the worker re-check each time whether it was new. Correct but noisy.
- **B3b-4 · Minted task ids are not predictable and were guessed wrong once** (`no task` error), until
  the worker read the `.task` field from the JSON instead of predicting the slug.
- **B3b-5 · No bulk edit for existing items** — wiring `maps-to-test` across five committed criteria
  meant five `set-field` calls; `doc author` refuses with `write.already-present`. Possibly deliberate
  (author is create-shaped, not patch-shaped), recorded as a capability gap.

**Adapter adherence, B3b: the cleanest of the four.** *"Nothing I can point to as factually
incorrect"* on question 2, and on question 4 the only direct read was of an **unregistered** file —
which AGENT.md explicitly permits until adoption — followed by `jigc migrate … --as adr` when it
needed to change. No bypass.

---

## Corrections to this file, 2026-08-18 — both from the archived logs, both mine

**1 · B1's counts were wrong.** This file recorded `5 / 5 / 0 / 1`; the archived log
(`~/ideas/harborlight-out/.jigc/logs/invocations.jsonl`, 26 records) gives **3 / 3 / 0 / 2**. The
live snapshot in [operator-log.md](operator-log.md) — taken at 21 records, showing 3 — agreed with
the log all along, so the running table was the outlier. Table corrected above. **B1's verdict is
unchanged: VERB.**

**2 · D-1's framing was wrong, and the correction matters more than the number.** I reported B1's
inability to find `jigc doc schema` as *"the seventh consecutive discoverability landing"*. The logs
say otherwise:

| session | `doc schema` invocations | adapter preloaded at session start? |
|---|---|---|
| B1 `harborlight` | **0** | **no** — the worker ran `jigc setup` itself, mid-session |
| B2 `pinegrove` | 4 | yes |
| B3a `stonefly` | 3 | yes |
| B3b `rosewater` | 4 | yes |

**Three of four workers found and used `doc schema`, eleven times between them.** The one that did
not is the one session where `CLAUDE.md` and `.jigc/AGENT.md` could not be in context, because setup
ran inside the measured session — the preload problem §4 already anticipates.

So the honest statement is **not** "the lens landed a seventh time". It is: **one of four workers
missed the capability, and it was the worker without the adapter loaded.** That is a materially
weaker claim, and it points at a different cause — preload, not discoverability. The correlation is
n=1 and cannot carry weight on its own; what it does do is remove the basis for the stronger claim I
made.

**What survives from D-1:** B1 genuinely reached for a schema-introspection surface, did not find one,
and reverse-engineered the shape with `doc create` + `doc show --format json`. That is a real
experience of a real worker and stays on the findings list. It is no longer evidence of a fleet-wide
discoverability failure.

---

## Correction 3 · B2-1 was wrong, and the way it was wrong is the lesson

I recorded B2-1 as **"VERIFIED at the source. The most serious finding so far"** — the pack
instructing an action its own create gate refuses, law 1 in its purest form. **It is not a lie. The
printed step is runnable in the context it is issued in.**

What the binary does, driven end to end:

```console
$ jigc milestone execute bound-the-store
Spawn: `cd .jigc/worktrees/cap-distinct-series && jigc workflow sub-task --task cap-distinct-series`

$ cd /work/.jigc/worktrees/cap-distinct-series
$ jigc workflow sub-task --task cap-distinct-series                       # exit 0
  58: Run: `jigc doc set-field commit:cap-distinct-series#type --value <COMMIT_TYPE> --task …`

$ jigc doc set-field commit:cap-distinct-series#type --value feat --task cap-distinct-series
exit 0
set commit:cap-distinct-series#type = feat
```

Composing the sub-task workflow provisions the transient commit doc, so the instruction the step
prints works. From the **main tree**, without that compose, the same command fails with
`no staged instance … provision it first` — and the second option that refusal offers
(`jigc doc create <type>`) is the one the sub-task gate forbids, which is the `create.gate-blocked`
B2 actually met.

**So B2-1 collapses into B2-2.** The real finding is the one already recorded: the sub-task context
is not reachable from the main tree, and nothing names `milestone provision` /
`.jigc/worktrees/<id>` until an error does. B2 hit a genuine wall; my account of *which* wall was
wrong.

### The lesson, which is worth more than the finding

**"VERIFIED at the source" was a static read.** I read `sub-task.yaml`'s `allows-create: [adr]`, read
`sub-task-commit.yaml`'s instruction, and inferred a refusal that the binary does not produce in the
context that matters. Reading a gate declaration is not running the command it gates.

That is the third correction this trial has needed from me, and the three have one shape: **B1's
carryover** (read a log count, inferred a gate failure — the gate was never given the case), **D-1's
seventh landing** (read one session's feedback, inferred a fleet-wide miss — three of four found the
verb), and this one (read a config, inferred a refusal). Each time the fix came from *driving the
binary*, and each time the first account was more dramatic than the truth.

**Downgraded from "the most serious finding" to a duplicate of B2-2**, with the runnable repro above
as the evidence.
