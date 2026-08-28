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

*(No sessions have run. This file is created before the first one so that "logged as it happens"
is possible rather than aspirational.)*
