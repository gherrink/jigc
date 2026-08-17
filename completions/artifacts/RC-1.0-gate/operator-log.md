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
