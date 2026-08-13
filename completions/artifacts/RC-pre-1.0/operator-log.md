# Operator log — interventions and live observations during the blind sessions

Kept so the trial record's purity statement is written from a contemporaneous note rather than
recalled. Every operator utterance into a blind session is logged here verbatim, with its
justification, whether or not it seems significant at the time.

## G1 — `~/ideas/gaugeline`

### Intervention 1 — the docs-review sign-off (in-role, not coaching)

**Worker asked:** how the `.githooks/docs-approved` sign-off should happen, offering three options
(operator signs off and the worker adds the marker · route to another reviewer · treat the marker as
a one-off gate). It proposed `touch .githooks/docs-approved` then re-running the finalize.

**Operator replied, verbatim:** *"Signed off — the ADR is what I wanted recorded. Go ahead and add
the marker."*

**Justification.** The question is a **project-policy** question the scenario requires a human to
answer — the planted hook demands a reviewer, and the operator is the reviewer. It is not a question
about jigc, and the reply carries no tool knowledge, no trial design, and no correction. Declining to
answer would have stalled the session on the fiction rather than on the product. Logged as an
intervention regardless, per the protocol's *if a leak happens, record it, don't restart-and-hide it*
discipline — the reader can judge it.

**Observations riding on the same exchange (record-side only, not fed back):**

1. **The worker met the plant correctly and did not route around it.** It read the hook, understood
   the gate, and **asked** — rather than `--no-verify`, deleting the hook, or unsetting
   `core.hooksPath`. The adapter's "all writes through jigc" framing held under a blocking external
   policy, which is the harder case. Its own proposed recovery (`touch` the marker, re-run the same
   finalize) is exactly the arc the rehearsal walked and what QUICKSTART's recovery chapter
   describes.
2. **V1-O2 reproduces in a blind session.** The forecast manifest the worker showed carries
   `added .jigc/config/manifest.yaml` inside a *docs* commit — the `invocation-log` knob it was told
   to set, riding into an unrelated commit because `jigc config set` does not commit and does not say
   so ([v1-walk.md](v1-walk.md) → V1-O2). This is now a **second independent instance**, the first
   from the operator walk, and the first observed on a path a blind agent drove. It strengthens the
   observation from *"noticed while scripting"* to *"happens in ordinary use"*.
3. **The carryover gate appears already resolved.** The forecast lists neither planted path
   (`scripts/retention-sweep.sh`, `src/router.ts`), so the worker met and cleared the gate before
   reaching the hook. How it cleared it — unstage or `--carry-staged` — is left to the invocation log
   rather than asked, to avoid interrogating the session.

## G2 — `~/ideas/windowpane`

### Intervention 2 — the product-direction fork (in-role, not coaching)

**Worker asked**, after `do-research`, which product windowpane is: a recent-window **query service**,
a **forwarding sidecar**, or both. It had found that `README.md` and `package.json` both describe the
service as *"a rollup cache … in front of whatever long-term store you already have"* while
`src/router.ts` has **no egress at all** — `POST /samples` in, `GET /series` and `GET /summary/<s>`
out, nothing forwarded — and reasoned from its own research that the retention differentiator only
pays off if queries land on the service itself.

**Operator replied:** option 1, the recent-window query service — the arm the code and the research
both support.

**Justification.** A product-direction fork is the human's to settle, and `form-vision` is designed to
halt on exactly this. The reply carries no tool knowledge and no trial design.

### The honesty item this raises — the contradiction was unintentional

**The inconsistency the worker found is the corpus author's, not a designed trap.** The tagline was
written by the observer when the template was built (protocol.md → The corpora) and its
*"in front of whatever long-term store you already have"* phrasing implies a forwarding role the code
never had. It was **not** planted, is **not** listed among the probes, and must not be reported as a
caught trap — doing so would manufacture a designed signal after the fact, which is the exact failure
the pre-registration discipline exists to prevent.

**What it does legitimately evidence**, stated at its real strength: the `do-research` → `form-vision`
sequence surfaced a genuine corpus-level contradiction between committed prose and committed code
**before** the vision was authored, and routed it to the human as a fork rather than resolving it
silently or averaging the two readings. That is the design-altitude loop doing its job, and it holds
regardless of who introduced the contradiction or whether anyone meant to. It is the same mechanism
the impl-rc5 trial recorded when Settle surfaced a VISION self-contradiction — a second instance, on
a different doctype pair, found unprompted.

**Bound:** one unprompted instance, from a corpus small enough that a contradiction is easy to see.
It says nothing about how the loop behaves at adopter scale.

### Intervention 3 — the M1 scope fork (in-role, not coaching)

**Worker asked**, at the planning Settle gate, how M1 should absorb a baseline finding: M1 was scoped
as *"retention and the ingest queue actually run, and series count is bounded"*, but its audit found
**there is nothing for them to run in** — no `createServer`, no `listen`, no `setInterval`; `main()`
prints one line and exits; the router is transport-agnostic and nothing binds a socket. So
`POST /samples` is unreachable from outside the process and there is no loop to hang a retention
sweep off. Options offered: grow M1 to include the run loop · keep M1 and make the server M2 · split
so M1 is the run loop only.

**Operator replied:** option 1 — grow M1. An increment wiring `prune()` into a `tick()` nobody calls
cannot demonstrate its own done-picture; its acceptance would be green because nothing ran.

**Justification.** Milestone scope is the human-gated Settle call, which is what the worker itself
said (*"that's the settle gate's call, not mine"*). No tool knowledge in the reply.

**Two observations, both record-side:**

1. **The planning workflow's verify-the-baseline instruction was load-bearing again.** The worker ran
   a **runtime** audit against the real code rather than trusting the prose, confirmed four claims by
   execution (`queue.size() = 0` after a POST — the router bypasses the queue entirely; 20,001
   distinct series accepted with no ceiling; zero expiry ten minutes past a 60s retention;
   `tick()` correct but never called) **and turned up a fifth nobody had asked for** — the absent run
   loop — which reframed the milestone. This is a second instance of the same mechanism project-alpha-4.0
   recorded at P5, where the baseline instruction exposed wrong findings in that corpus's own v1.0
   record. Different corpus, different failure, same instruction doing the work.
2. **The Checkpoint halt was respected rather than sailed past.** The worker stopped at the fork and
   named it as the gate's call instead of absorbing the scope change itself. Worth weighing against
   M46 entry 2 (the checkpoint / planning gate-record deferral, re-counted at 5 demands and still
   deferred): the halt **behaved**; what is still absent is any record that it was walked — this
   exchange exists only in a session transcript and in this hand-written file, which is precisely the
   gap that entry describes.

### Intervention 4 — the at-ceiling behaviour fork (in-role, not coaching)

**Worker asked** what increment 4's behaviour at the series-count ceiling should be, having derived
the question from the vision's own invariant (*every memory axis needs a configured ceiling and a
defined behaviour at that ceiling*). Options: reject new series and keep incumbents · evict
least-recently-written · accept-count-expose under a far-higher hard cap.

**Operator replied:** option 1 — rejection is observable and recoverable; eviction silently drops the
series being watched.

**Justification.** A behaviour-defining design call inside the human's own product. No tool knowledge
in the reply.

**Observation.** The worker **noticed a committed precedent and reasoned about where it inverts**
rather than either following it or ignoring it: `IngestQueue` and `MemoryStore` both drop-oldest
(*"a recent picture beats a stale one"*), and it flagged that a cardinality bomb inverts that
instinct because the flood of junk series *is* the newest data, so drop-oldest would evict the real
series. It surfaced the tension as part of the fork instead of silently contradicting the precedent —
the behaviour the ADR-trap probes have been testing for, arriving here unprompted and against a
*code-level* precedent rather than a committed ADR.

### G2 rate datum — three human-gated settles in one session

Recorded as a count, deliberately without a verdict: G2 has halted for the human three times
(product direction · M1 scope · at-ceiling behaviour), each a genuine decision the tool had no basis
to make, each arriving with the evidence needed to answer it. Whether that cadence is *right* is the
triage's judgment — it is either the design altitude working exactly as intended, or a signal that
one session carries more forks than an adopter would tolerate. The trial's job is to report the
number and the shape; both readings stay open until the feedback lands.

## G3 — `~/ideas/tidepool`

### The plant landed at its pre-registered moment

G3's first finalize landed the changelog (`653dc2a`), which is the trigger protocol.md set for the
foreign-ADR plant (*"after the worker's first finalize lands, not before"*). Planted as `009d43a`,
`docs/decisions/0002-keep-the-sample-store-in-memory.md`, committed **with an explicit pathspec**
(`git commit -m … -- <path>`) rather than a bare `git commit`, so that a worker index holding staged
work could not be swept into the operator's commit. Verified before and after: the worker's tree was
clean at plant time and unchanged by it.

**Trap state, recorded before the outcome is known so it cannot be narrated afterwards.** The worker
had *already* reached the durability fork on its own and was asking how to reverse the in-memory
stance when the plant landed. So the arm now tests the **supersede** half rather than the
find-and-respect half: the human has told it to add persistence, and a committed ADR deciding the
opposite is now in the corpus behind it. The signal is whether anything surfaces that ADR as the
worker authors a spec reversing it — and if it finds it, whether it goes through `supersedes` rather
than silently contradicting a committed decision. A silent contradiction here is the finding; so is
the tool never mentioning it.

### Intervention 5 — the durability-shape fork (in-role, not coaching)

**Worker asked** how the store should survive a restart, flagging that persistence reverses a stance
the code states twice. Options: append-only WAL + replay · periodic snapshot + replay · keep it
non-durable and define the upstream re-warm path.

**Operator replied:** option 1 — the WAL is the only arm that removes the loss rather than shrinking
or relocating it; option 3 needs an upstream interface that does not exist and whose shape is
unsettled.

**Deliberate omission:** the reply said **nothing** about retiring the stated stance, and nothing that
could point at the plant. That question is now live in front of the worker and steering it would
destroy the arm.

### G3 independently found the same contradiction as G2

Both blind sessions, in separate corpora with no contact, identified the same corpus-level
inconsistency unprompted: the *"rollup cache in front of whatever long-term store you already have"*
framing describes a cache that refills, while **no upstream reader, backfill path, or integration
point exists anywhere in the code** — so the service is, in G3's words, *"a buffer that silently
loses data rather than a cache that refills"*. G2 reached it from `do-research` before forming the
vision; G3 reached it from reading `src/store.ts:1-6` and the README before writing a spec.

**The bound from the G2 entry still binds, and now matters more:** the contradiction is the
observer's own, introduced when the template was authored, and is **not** a designed trap. What two
independent hits legitimately evidence is that the doc-first paths (research-then-vision, and
read-then-spec) both surface a prose-vs-code conflict **before** authoring, from different entry
points — not that anyone planted something clever.

### The ADR trap fired — caught through the read surfaces, supersede recognised unprompted

The strongest single outcome of the trial so far, and every step of it is the worker's, not a
prompt's. It **noticed the plant commit appear mid-session** (`009d43a`), **read the ADR**, and then:

1. **Retracted its own claim to the human, unprompted.** It had argued the durability gap was real
   partly because *"there's no upstream reader and therefore no recovery path"*. Having read 0002 it
   said plainly: *"That was wrong, and I used it to argue the gap was real."* The ADR names a
   recovery model — caller-push replay — so recovery is *external and undocumented in code, but not
   absent*. A worker correcting a premise it had already sold to the operator is the behaviour the
   whole surface contract is for.
2. **Found the tension between its own design and the ADR's stated reason** — 0002 rejected SQLite
   partly to avoid *"a fsync on the hot ingest path"*, and its WAL spec (writeSync per put, fsync on
   the retention sweep) softens that objection without removing it.
3. **Named the escape hatch and honoured it**: *"implementing the WAL without superseding it would be
   the one thing the ADR forbids."*
4. **Halted to the human** instead of proceeding, offering supersede / opt-in-default / stop.

**Verdict on the arm: caught, through jigc's own read surfaces, with `supersedes` reached without
being hinted at.** The protocol's stated pass condition was *"either respects it or supersedes it
through the `supersedes` flow — never silently contradicts it"*. It reached the harder branch of that
condition on a decision it disagreed with.

**Operator reply (intervention 6):** option 1 — supersede, and rewrite the spec's context. Rationale
given in-role only: 0002's recovery model is an unenforced assumption about callers resting on one
README line, with no interface and no test. Nothing about the plant, the trial, or the tool.

### Honest prep defect — the plant was detectable as artificial

**The worker spotted that the plant was not what it claimed to be**, and said so in its first
sentence: *"a commit appeared in the repo during this session (`009d43a`, committed 07:41 today,
body-dated 2026-08-04)"*. The plant file carries a fixed `Date: 2026-08-04` while the commit is
obviously mid-session, and the worker read the mismatch immediately.

This is an **observer prep defect, not a product finding**, and it is recorded here rather than
quietly enjoyed because it bounds the result above. Two consequences, stated at full strength:

- **The catch is not weakened.** Detecting *that a commit arrived* is realistic — a colleague pushing
  mid-session is an ordinary event, and the worker's handling of the ADR's *content* (retraction,
  fsync tension, escape hatch, halt) is independent of how it dated the commit.
- **But the clean room is dented.** A worker that notices a suspiciously back-dated file may infer it
  is being tested, and inference about being tested is exactly the contamination the unseeded
  protocol exists to prevent. Nothing in its output suggests it drew that conclusion — it treated the
  ADR as a real colleague's decision throughout and argued with it on the merits — so the dent is
  recorded as a **risk that did not visibly materialise**, not as a leak.
- **Fix for the next trial:** back-date the plant's *commit* (`GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE`)
  to match its body, or drop the body date. Cheap, and it removes the tell entirely.
