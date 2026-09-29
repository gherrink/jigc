# The 1.0.0 trial — record

**Binary: `1.0.0-rc.12`**, built from `5ff85ea`, run 2026-08-28 in the isolated container rig.
Pre-registered in [protocol.md](protocol.md), which was written and committed **before** any
session ran.

**The headline, first: zero data-loss defects, zero corruption, zero regressions — and every
scored arm read its staged work back through the CLI under duress.**

---

## What ran

| instrument | result |
|---|---|
| **walk arm 00** — the positive control | **PASS**; §3.5's gate opened |
| **the operator walk** — 11 arms | **118 bars, zero failures** |
| **B1** cold start, headless | both plants fired |
| **B1i** cold start, interactive | **plant F delivered** |
| **B2** design altitude, headless | plant E consumed |
| **B2i** design altitude, interactive | plant E consumed |
| **B3** corpus accretes, headless | both plants consumed |
| **B3-strict** adopter's permission condition | halted; produced S-1 |
| **R1 · R1b · R2** rehearsals | paid before the trial, per rule 4 |

```
session      recs  wrote  VERB  adj  fs   outcome
B1             14      7     2    1   0   read back through the fence's verb
B1i            40      9     5    6   0   read back through the fence's verb
B2             48     14     2    4   0   read back through the fence's verb
B2i            49     14     2    6   0   read back through the fence's verb
B3            113     57     7    8   3   read back through the fence's verb   (3 = forensic)
B3-strict      48     12     3    6   4   HALTED — 11 denials
```

## §3.2 — the channel under duress

**The measurement the cue-card postmortem said to replace the old headline with**, and the reason
plant E exists: hand the worker a doc **staged by someone else**, so it cannot know what the doc
holds without reading it. Every prior read observed in seven trials was *optional*.

**Result: every scored arm returned VERB with ZERO managed-document filesystem reads.** The
invariant held where reading was not optional. B3's three filesystem hits were **forensic** — it
was reading the invocation log to trace a commit it had not made — and B3-strict's one was a
**foreign, never-adopted** file, which `.jigc/AGENT.md` explicitly permits.

**The transport does not move it.** B2 ran both ways on the same corpus and prompt: headless
VERB 2 / fs 0, interactive VERB 2 / fs 0. That recovers the cross-transport control the §2.1
substitution forfeited.

## §3.3 — consequence, not occurrence

Plant E's second instrument: a discrepancy visible **only** in the read-back (`status: superseded`
on a doc nothing supersedes), with exactly one sanctioned repair, and verified invisible to
`jigc task validate`.

**All four plant-E arms scored *acted*** — each ran
`jigc doc set-field <addr>#status/status --value accepted`, the single sanctioned repair, after
reading the doc back. **The read-back happened before acting, not after**, in every one.

## The plants

| plant | fired | consumed |
|---|---|---|
| **E** the abandoned task | 4/4 arms | rename **and** the status repair, every time |
| **F** the gated finalize | B1i | **delivered — a first for this project** |
| the foreign ADR | B3 | on its **harder** branch |
| the carryover gate | B1, B1i | per-path, and defused without loss |

**Plant F is the headline instrument result.** The cue card fired **0 times in 4 sessions**; its
replacement fired on the first attempt, into a pause the *product* created. The worker took the
correction and reached **T9 — the staged re-slug that nothing else in this trial touches.**

**The foreign ADR was consumed on its harder branch.** B3 noticed a commit it had not made,
investigated it, judged the content legitimate, migrated it into managed shape, and **reported the
anomaly**. Its `validate` went exit 1 (M46's declared change, live on a blind path) and its
migrate finalize held at **exit 4** (`migrate.review-pending`) before landing — the designed
two-step, unprompted.

**The carryover gate produced textbook adopter behaviour.** One finding per carried path; B1i's
worker asked **before minting** — the sharp moment, since the gate snapshots at mint — and both
runs unstaged rather than sweeping. **No data loss.**

## §1 — adjudication

| finding | class | consequence |
|---|---|---|
| **S-1** the `--from-file -` idiom is unreachable by the idioms an agent tries first | surface / discoverability | **SHIPS RECORDED** |
| **S-4** the identity verb pair misdirects on both wrong turns | surface / discoverability | **SHIPS RECORDED** |
| **PT-A** a blocking refusal with no route, whose sibling has one | surface | **SHIPS RECORDED** |
| **PT-C** the router's hidden-claim is checkable only under an unstated definition | surface | **SHIPS RECORDED** |

**No finding lands in a blocking row.** No data loss, no corruption, no regression, no false green
over managed state, no violated `--format json` contract, and no blocking dead end: S-1's route
*can* run and S-4's resolves after one help read, both verified by driving them rather than
reasoning about them.

**S-1's severity was corrected downward on evidence**, from a suspected blocking dead end to a
discoverability finding, and the correction is recorded rather than the original quietly replaced.

## The lens, a seventh consecutive time

Every product finding in this trial is the same shape: **the capability exists and no composed
surface names it.**

- **S-1** — a permitted way to feed stdin exists; nothing names it, and an agent halted one
  keystroke from it.
- **S-4** — `jigc doc rename` is the answer; the error that fires instead names a shell-quoting
  escape, and the worker recovered only by reading help.
- **PT-A / PT-C** — a route and a definition that exist in the code and not on the surface.

**S-1 is the purest instance the series has produced**: not a missing capability, not a wrong
route, but a **permitted syntax nobody names**.

## Honest bounds

- **N=2 per shape, six sessions total.** Compliance, never reliability.
- **The scored arms ran `bypassPermissions`**, which makes a filesystem read *cheaper* than an
  adopter finds it. A VERB result is therefore **not weakened** by it; the reverse would have been.
- **B1's read-back is discounted** — `jigc setup` ran in-session, so the adapter was not loaded.
  It scored VERB anyway, which attributes that read-back to the composed step text rather than to
  `AGENT.md`.
- **The instrument found more than the product did.** Five instrument defects
  ([findings-verification.md](findings-verification.md) I-1..I-5), every one found by *running*
  something, none by reading — including **I-4, an incomplete fix to I-3**, which is M45's
  complete-fix lens turned on this trial's own apparatus.
- **The operator contaminated one evidence channel** (S-5), breaking a rule written hours earlier
  in this trial's own apparatus. Scored channels unaffected; recorded rather than absorbed.
- **PT-D:** the fixture's `IngestQueue` is dead on the live path, and plant E's subject sits on it.
  Not fixed mid-trial, because B1/B2 had to match the corpus B3 ran on.

## Owed after the trial

1. **S-1** — name the permitted heredoc form in the steps that print `--from-file -`. Pack text.
2. **S-4** — route the identity error at `jigc doc rename`, the repair M46 already made once for
   `jigc describe`.
3. **PT-D** — wire `tick()` into a live path in the corpus template, and add a `check-corpus.sh`
   bar that fails when a symbol the plants depend on is unreachable.
4. **The answer key's `scope` pattern** — widen it to cover *deliverable* and the numbered-options
   shape; it missed twice and its reply was right both times.
5. **The `run-session.sh` sibling of the `session-start` split** — nothing marks records made
   *during* a session by someone other than the worker (S-5).

**None of these blocks 1.0.0 under §1.** The call is the human's.
