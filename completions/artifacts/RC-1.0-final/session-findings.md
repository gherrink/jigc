# Session findings

What the sessions produced, session by session. **Nothing here is adjudicated against
[protocol.md](protocol.md) §1** — that happens at the trial record, with a repro block per §7.

Sessions run so far: **B3** (scored), **B3-strict** (unscored by design), and the operator walk
(11 arms, 118 bars, zero failures). **B1 and B2 have not run** — they are interactive by design
and need an operator ([runbook.md](runbook.md) §2).

---

## B3 — headless, plant E + the mid-session foreign ADR — CLEAN

```
recs 113 · authoring 57 · VERB 7 (effective 6) · adjacent 8 · fs 3
outcome: read back through the fence's verb
fs split: 0 managed-document reads · 3 workbench-bookkeeping reads
```

**Both plants fired and were fully consumed.**

*Plant E.* The worker read the staged doc back through `jigc doc show … --task` **before acting**,
found the title↔prose contradiction, ran `jigc doc rename`, and applied **exactly the sanctioned
repair** for the planted `status: superseded` (`set-field … --value accepted`). §3.3's **acted**
outcome, and §3.2's channel-under-duress question answered: **zero managed-document filesystem
reads.** The invariant held where reading was not optional.

*The foreign ADR.* Fired on its state after **91 s**, polled from outside the session. Consumed on
its harder branch: the worker noticed a commit it had not made, **investigated** it, judged the
content legitimate, migrated it into managed shape, and **reported the anomaly** to the operator
in its own summary. Its `validate` went exit 1 (M46's declared change, live on a blind path), and
the migrate finalize held at **exit 4** (`migrate.review-pending`) before landing — the designed
two-step, unprompted.

*The three workbench reads are that investigation* — `cat`/`tail` of the invocation log while
tracing the mystery commit. **Without the document/bookkeeping split added after R1 they would
have scored as three channel bypasses** and pointed the conclusion the wrong way.

*The one failed read is a recovered typo, not a defect.* `doc show decisions/reject-…` — a
path-shaped address — refused with the grammar, an example, the singleton exception and a route;
the worker corrected to `adr:reject-…` in one step. **Q1's settlement vindicated on real data:**
7 attempts, 6 effective, and folding them into one number would have hidden both facts.

---

## B3-strict — the adopter's real permission condition — HALTED, and it found something

Unscored for the headline by design (increment 0 measured this arm truncating the arc). **Its
findings still count.**

```
recs 48 · authoring 12 · VERB 3 · adjacent 6 · fs 4 (1 managed-document, 3 workbench)
HALTED awaiting the operator — 11 denials on Bash, Edit, Write
```

### S-1 · The pack's own `--from-file -` idiom is unreachable under the default permission set

**Zero bare `jigc` commands were denied.** The allowlist works for direct invocations. But
**eight of the eleven denials are one act, attempted five different ways** — getting a payload to
`jigc doc author --from-file -`:

```
Bash    cat <<'YAML' | jigc doc author changelog --from-file - --task …   DENIED
Write   /tmp/changelog-payload.yaml                                        DENIED
Write   /work/.scratch-changelog-payload.yaml                              DENIED
Bash    cat > /work/.scratch-changelog-payload.yaml <<'YAML'               DENIED
Write   /work/changelog-payload.yaml                                       DENIED
Bash    printf '%s\n' 'title: Changelog' …            (three attempts)     DENIED
```

Every route to stdin is either a **file write** or a **Bash command that does not begin with
`jigc`** — and `jigc setup` allowlists `Bash(jigc:*)` and `Bash(git add:*)`. The worker exhausted
its idioms and stopped.

**The idiom is the pack's own, not the worker's invention.** `--from-file -` is what the shipped
steps print: `author-roadmap.yaml` (×3), `author-completion-record.yaml`, `author-research.yaml`,
and more.

**Measured, and its boundary stated.** What is measured is the denial set from this session: 11
denials, 0 of them a bare `jigc` call, 8 of them attempts to feed `--from-file -`. What is *not*
independently tested here is Claude Code's permission-matcher semantics — the conclusion that
`Bash(jigc:*)` cannot match a piped form is **inferred from the denials**, not from a matcher
test. That distinction is left in rather than smoothed over.

**Why it may matter more than an unscored arm suggests.** `jigc setup` writes that allowlist. So
the tool's own install produces a permission set under which the tool's own documented authoring
path cannot be executed — an adopter who does not blanket-approve meets this on their first
payload-authored doc. Whether that is a §1 **blocking dead end** (a printed route that cannot run,
reaching any payload-authored doc including root managed ones) or a **surface finding** is an
adjudication for the trial record, on evidence, per §1's own rule that the class is fixed before
the consequence is looked up.

**It confirms and sharpens increment 0**, which recorded *"all 7 denials were file writes"* and
that the worker *"named the correct escape itself and still stopped to ask."* This run shows
**why** it stopped: the escape needs stdin, and every route to stdin is closed.

### S-2 · The one managed-document read is a permitted one

`cat /work/CHANGELOG.md` — and the invocation log shows **no `ingest` or `migrate` ran before
it**, so the file was still **foreign and never adopted**. `.jigc/AGENT.md` explicitly permits
reading an unregistered doc before adopting it, and the 1.0.0-gate record dispositions exactly
one such read the same way.

**This is the FILESYSTEM heuristic's declared bound firing in the wild**: registration state is
not in the transcript, so the reader flags it and a human makes the call. Scored as **permitted**,
not as a bypass.

---

## What has not run

**B1 and B2.** Both interactive by design — B1 because plant F's mechanism is an utterance into a
pause `claude -p` has no channel to receive, B2 because it carries the headline and the transport
should not move under it on n=1 equivalence evidence. Until they run, §3.5's reading is at **N=1**
on the duress measurement, and the trial record must say so rather than reading B3 as the result.
