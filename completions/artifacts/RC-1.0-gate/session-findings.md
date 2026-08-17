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
| B1 `harborlight` | **5** | 5 | 0 | 1 | 26 |
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
