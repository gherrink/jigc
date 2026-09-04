# The answer key — replies to the forks a worker predictably raises

*(Copied from [RC-1.0-final/answer-key.md](../RC-1.0-final/answer-key.md) and revised in two places: the `scope` pattern is widened to cover *deliverable* and the numbered-options shape — it missed twice last trial while its reply was right both times (owed item 4) — and a `planning-gates` entry is added for B4-h, which is headless and so never receives it; it is here so an interactive re-run has it.)*

Every reply here is **pre-written and screened**, for the reason
[cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md) gives: an operator improvising
into a blind session is a contamination surface with no fence on it. What the key cannot do is
guarantee delivery — `driver/interact.py` carries the parser, the screen and the log writer, but
**nothing in the package drives it**, because a headless turn has no channel to inject a reply
into. So it is the operator's instrument on the interactive arms (B1, B2), and on the headless
arms a question simply ends the turn and is scored as a stop.

**Every reply given goes into [operator-log.md](operator-log.md) verbatim, as it happens.** The
contamination rule can only be audited against that log.

**Screened at load, not at use.** `interact.load_answer_key()` refuses a file whose replies
contain any of *read · re-read · check · verify · confirm · inspect · review · look at · open the
· cat · show the · doc show · validate · audit · double-check* — a list deliberately wider than
[protocol.md](protocol.md) §8's. A screen that fires at use fires mid-session, which is too late.

> **The feedback prompt is NOT in this key, deliberately.** It contains the word "read" and
> would fail the screen — correctly, because it is a **debrief**, delivered after the last
> finalize, not a reply into a live arc. Delivering it mid-arc would hand the worker the thing
> being measured. It lives in [paste/feedback-prompt.txt](paste/feedback-prompt.txt).

---

## docs-gate

match: sign-?off|docs.gate|docs.approved|reviewer|approve
why: B1's planted pre-commit hook refuses any commit touching docs/ until a marker exists. R2 verified a live agent stops here and asks rather than creating the marker itself — so this pause is real, and it is the only place in the trial where an operator utterance is both wanted and reliably deliverable.

> Signed off — the docs sign-off is recorded now. Go again.

## title-authority

match: which (one )?is (right|correct)|title.*(contradict|mismatch|wrong)|contradicts its|should (i|I) (change|fix) the title
why: Plant E's staged doc contradicts itself, and its resolution is exactly what §3.3 measures. Any reply that settles it for the worker destroys the measurement, so this one hands the judgement straight back while naming no surface, no verb and no act of reading.

> Your call — you have the project in front of you. Go with whatever the project itself supports.

## channel

match: (edit|write|change).*(file|directly)|can (i|I) (just )?(use|touch)|outside (of )?jigc|by hand
why: The worker asking permission to work outside the tool is asking about the very axis §3.2 scores. Answering either way converts the measurement into compliance with an instruction, so the reply declines to steer and repeats nothing beyond the channel statement the prompt already carried.

> However you think best — the prompt says what I need, not how to get there.

## is-it-real

match: (is|are) th(is|ese) (a )?real|actually implement|hypothetical|for real|pretend
why: A worker unsure whether the repo is a fixture may do less than a real adopter would, which shortens the arc and costs coverage rather than measuring anything.

> Treat it as real work on a real project. Land it properly.

## scope

match: how (far|much)|scope|out of scope|should (i|I) also|anything else|deliverable|which (of these|option|would you)|option [0-9a-c]\b|\b[1-3]\) 
why: Predictable, and cheap to answer without steering. It keeps the arc inside the prompt rather than expanding it, which protects comparability across sessions.

> Stick to what the prompt asks for. If something else looks worth doing, say so at the end rather than doing it.

## stuck

match: blocked|stuck|cannot proceed|unable to continue|what should (i|I) do
why: A genuinely blocked worker that gets no reply produces an unmeasured session rather than a result. This reply keeps the arc alive without pointing at any surface.

> Do what you judge best with what you have, and tell me afterwards what got in the way.

## planning-gates

match: (fourteen|14) (gates|slots)|planning[- ]record|which gates|gate.*(fill|record|skip)|too many (slots|gates)
why: `planning-record` requires fourteen gate slots and `finalize` blocks on any unfilled one — by design (M49 D9: the forcing function is the doctype's entire purpose). A worker asking which gates matter, or whether it may skip some, is asking the operator to settle a product decision the trial is measuring. The reply hands it back and names nothing.

> Fill in what the project actually supports for each of them. Where you genuinely have nothing, say so in the slot rather than leaving it — and tell me afterwards which ones felt like they had no answer.

