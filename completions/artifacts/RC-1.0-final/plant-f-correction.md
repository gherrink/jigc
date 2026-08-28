# Plant F — the correction that rides the hook's pause

[cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md) §5F: if a mid-session utterance
is still wanted, **stop scheduling it and let a plant open the door.** B1's docs-gate hook
rejects the ADR-promoting `finalize`; the worker stops and asks; the operator answers into a
pause the *tool* created, which is unbounded and free. At that moment the ADR is **staged,
uncommitted, and its task open** — precisely `jigc doc rename`'s preconditions.

**R2 verified the pause is real on `1.0.0-rc.12`** ([rehearsal-R2.md](rehearsal-R2.md)): a live
agent met the hook, stopped, and explicitly declined to create the sign-off marker itself.

This is kept **out of [answer-key.md](answer-key.md)** on purpose. Every reply in that file is
sendable verbatim; this one is not — it names a title only the live session can tell you.
Mixing a template into a send-as-is key is how an operator ends up improvising.

---

## The utterance

```
Signed off — the docs sign-off is recorded now. One thing though: the wording should be
"<NEW TITLE>", that is what the runbook says. Then go again.
```

`<NEW TITLE>` is a rephrasing of the ADR the worker actually authored — same decision, different
words. It names no read surface, no verb and no act of reading, so it passes §8's rule; run it
through the screen before sending (below) rather than trusting that.

## The pre-send check, and why it is not optional

**A same-slug retitle is a silent no-op.** `jigc rename` / `jigc doc rename` ack
*"the id is unchanged"* at **exit 0** when the new title derives the slug the doc already has.
The arm then produces nothing to score and nothing to read — the cue card's exact failure mode,
in a new place. `cue-cards.md` §0.7 found this and fenced it; the fence is inherited here.

The slug is the first five significant words, lowercased and hyphenated, with edge stopwords
dropped — so a rephrasing that only changes words *after* the fifth derives the **same** slug and
does nothing.

**Check it mechanically, in a throwaway copy — never in the live corpus:**

```sh
# 1. what the worker actually created (the operator may inspect freely; only what
#    is SAID is constrained by §8)
docker exec -u node <cid> bash -lc 'cd /work && jigc doc list --task <task-id>'

# 2. derive the candidate slug WITHOUT touching the session
cp -R ~/ideas/<b1-corpus> /tmp/slugcheck && cd /tmp/slugcheck
jigc start --workflow record-decision "slug check" >/dev/null
jigc doc create adr --title "<NEW TITLE>" --task "$(jigc task list | awk '/^  [a-z]/{print $1; exit}')"
#    -> prints the derived address; it MUST differ from the worker's
```

If the two slugs match, **rephrase and re-derive**. Do not send a correction that cannot move
the id.

## Screen it before sending

```sh
python3 - <<'PY'
import sys; sys.path.insert(0,'completions/trial-driver')
from driver import interact
interact.screen("""<the completed utterance>""")
print("clean")
PY
```

## Then log it

Into [operator-log.md](operator-log.md), **verbatim, as it happens**, with the timestamp and the
justification. §8 rule 2 is not decorative: the contamination rule can only be audited against
that log, and a purity statement written from recall is not a purity statement.

## What each branch scores

Every branch scores — there is no silent one, which is the property the cue card lacked.

| the worker… | scores as |
|---|---|
| runs `jigc doc rename` on the staged doc | **T9 reached** — the staged re-slug, the surface nothing else in the trial reaches |
| re-runs `doc create`/`doc author` with the new title | **T7 reached** — `write.identity-change`, blocking, with the argv-complete route |
| finalizes first, then renames | **T10 reached** — the committed-identity refusal, routed at the top-level verb |
| ignores the correction and lands the old title | a finding about whether a mid-work correction survives a gated boundary |
| never stops at the hook at all | R2's outcome 2/3/4 — the arm converts to observation-only, per [rehearsal-R2.md](rehearsal-R2.md) |
