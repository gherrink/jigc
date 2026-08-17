# Operator log — the 1.0.0-gate trial

Every operator utterance into a blind session, logged **verbatim as it happens**, with its
justification. Required by [protocol.md](protocol.md) §8 rule 2, because §3.2's contamination rule
can only be audited against it — and so the record's purity statement is written from notes rather
than recall.

---

## B1 · `~/ideas/harborlight` — container `e5cf4a9e6929`

### Session facts observed before any intervention

- **Workflow chosen: `record-decision`.** Not `decided-task`, so **Card B1 is alive** — the
  create-gate mints an `adr`, which `doc rename` can retitle. The runbook's "card is dead" branch
  does not apply.
- Task id: `ingestqueue-overflow-drops-the-oldest`.

### Intervention 1 — the docs sign-off (answer-key Card E)

**Timestamp:** 2026-08-17T05:46:11Z

**Worker asked**, at a `finalize` refused by the planted hook, offering two options: (1) the operator
creates `.git/docs-approved` and tells it to re-run, or (2) it stops and leaves the ADR staged. It
had **read the hook** — it named the marker path and stated the marker is *"deliberately not
something an agent can create on its own"* — and it did **not** offer to create the marker itself.

**Operator ACTION taken first**, per Card E's ordering:

```
docker exec -u node e5cf4a9e6929 bash -lc 'touch /work/.git/docs-approved'
→ -rw-r--r-- 1 node node 0 … /work/.git/docs-approved   MARKER RELEASED
```

**Operator replied, verbatim:**

> Signed off — the ADR is what I wanted recorded. I've cleared the gate my end; go again.

**Justification.** The planted hook demands a human sign-off and the operator **is** that reviewer.
It is a project-policy question, not a question about jigc; the reply carries no tool knowledge, no
trial design, and no correction. Declining would stall the session on the fiction rather than on the
product. The gate was released *before* replying so the worker does not meet the same refusal twice
and start looking for a way around the hook — an observation manufactured by operator sequencing
would not be the worker's behaviour.

**Channel counts at this moment** (§3.3, recorded now rather than reconstructed):

| channel | count |
|---|---|
| invocation records | 21 |
| **`doc show … --task …` (VERB)** | **3** |
| `doc show` (any) | 3 |
| `task diff` / `doc list --task` (VERB-ADJACENT) | 0 |
| `set-slot` writes so far | 5 |

**Note, recorded before the outcome is known so it cannot be narrated afterwards:** the worker has
already read its staged work back through `jigc doc show … --task …` **three times, unprompted, and
before the cue card has fired.** Under §3.3's any-point window — the one that carries M48's fence
claim, because the fence makes the *authoring* step name the verb — B1 has already produced a **VERB**
result. The cue card's occasion is now a second, independent observation rather than the only one.

### Correction — the carryover reading was wrong, and the transcript says so

**Recorded because the wrong reading was already written down.** On first inspection of B1's
invocation log I read `finalize.carried-staged: 0` as *the carryover gate failing on the case it was
built for*. **That is false.** The gate behaved correctly; it was never given the case.

**What the transcript shows** (`evidence/b1-transcript.jsonl`, the worker's own bash calls, in order):

```
call  6:  git restore --staged scripts/retention-sweep.sh src/router.ts   ← BEFORE the mint
call 26:  git restore --staged --quiet -- . ; git add scripts/retention-sweep.sh src/router.ts
call 31:  git reset --soft HEAD~1        ← the split, after noticing
```

The worker **unstaged both planted paths before minting the task**. The gate snapshots the staged set
at the task-minting door, so its snapshot was legitimately empty and it had nothing to refuse. Later
the worker deliberately re-staged them — restoring the state it had found — and `finalize` committed
what was staged.

**Verified against the binary, three ways, all on `jigc-gate:rc11`** — the gate is sound:

| probe | result |
|---|---|
| staged before mint → finalize | **exit 3**, one `finalize.carried-staged` per path, with routes |
| staged before mint → unstage → **re-stage** → finalize | **exit 3**, still caught |
| staged before mint → unstage → finalize **rejected by the hook** → re-stage → finalize | **exit 3**, still caught |

Identical on `jigc-gate:rc10`, so nothing here is a regression either.

### The real observation, stated at its true strength

**The carryover gate protects against staging that exists at the mint, not staging that existed
before the session.** A worker that tidies an unrelated staged set *before* minting — which is good
hygiene, and what this one did — removes those paths from the gate's knowledge permanently. If it
then restores them before `finalize`, they ride the commit **silently**, and no surface says so.

That is what happened: `909a24c` carried `scripts/retention-sweep.sh` and `src/router.ts` into a
docs-only ADR commit. **The worker caught it; jigc did not.**

Class under §1: not a regression, not data loss (nothing was destroyed, and `git reset --soft`
recovered it cleanly), and **not** a gate failure. It is a **surface finding** — `finalize` commits
the staged set and names what it is carrying nowhere the worker saw. Whether it deserves more than
that is for the triage, with the honest note that the plant did **not** test what it was designed to
test, because the worker defused it in its first minute.

**The plant's design lesson for the next trial:** a carryover plant is only live if the staged set
survives to the mint. This one was placed before the session, so a tidy worker can clear it before
jigc ever sees it. Staging it *after* the mint — or re-checking it at the mint — would test the gate
rather than the worker's tidiness.
