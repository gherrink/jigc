# Operator log

**Every operator utterance into a blind session, logged verbatim as it happens**, with its
timestamp and its justification — [protocol.md](protocol.md) §8 rule 2. Includes the opening
prompt paste, the `bypassPermissions` confirmation, every answer-key reply, and plant F's
correction. The feedback prompt is logged only if delivered before the last finalize — in which
case the session's measurement is **void**.

Format, one entry per utterance:

```
## <UTC ISO timestamp> · <session> · <kind>

> the utterance, verbatim

*Why:* <the answer-key entry it came from, or the justification if improvised —
and an improvised utterance is itself a finding about the key>
```

---

*(created before the first session, 2026-09-04 — empty until one runs)*
