# Operator log

**Every operator utterance into a blind session, logged verbatim as it happens**, with its
timestamp and its justification. Required by [protocol.md](protocol.md) §8 rule 2, because §8's
contamination rule can only be audited against this file — a purity statement written from
recall is not a purity statement.

Includes the opening prompt, every answer-key reply, and plant F's correction. Excludes the
feedback prompt only if it is delivered after the last finalize; if it is delivered earlier,
log it here and mark the session's measurement **void**.

Format, one entry per utterance:

```
## <UTC ISO timestamp> · <session> · <kind>

> the utterance, verbatim

*Why:* <the answer-key entry it came from, or the justification if improvised —
and an improvised utterance is itself a finding about the key>
```

---

## 2026-08-28 · B2 (interactive) · answer-key reply — `scope`

The worker asked, mid-arc, what the deliverable was for the series-cardinality investigation,
offering three options (research+ADR only / research+ADR+implement a cap / free text).

> Stick to what the prompt asks for. If something else looks worth doing, say so at the end rather than doing it.

*Why:* the pre-written `scope` entry from [answer-key.md](answer-key.md), sent verbatim.

**Recorded gap, and it is a finding about the key rather than about the session.** The `scope`
entry's **reply** fits this question exactly; its **`match:` pattern does not fire** on it —
checked mechanically, all six patterns returned False, and `interact.answer()` returned a HALT.
The pattern is
`how (far|much)|scope|out of scope|should (i|I) also|anything else`, and the worker asked about a
*"deliverable"* and offered numbered options instead.

**The call, and its reasoning, so it can be audited:** a pre-written, screened reply was sent, so
the no-improvisation rule is satisfied — the regex is an implementation detail of a loop nothing
drives, and a human operator matches on meaning. The `scope` reply is also the
**least-information** answer available: it declines to expand and points back at the prompt, which
asks for investigate-then-record and not implement. The generic fallback would have invited a free
choice between the options, adding arc variance and hurting comparability with B3 — precisely what
this entry exists to prevent.

**Owed after the trial:** widen the `scope` pattern to cover *deliverable* and the
numbered-options shape. **Not changed mid-run** — an instrument is not edited during the session
it is measuring.
