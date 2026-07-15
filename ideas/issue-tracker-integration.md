# issue-tracker integration — external tracker items as task origination sources

**Status: parked 2026-07-15.** From the human's trial scratch notes (the lacon-trial batch, [trial-record](../completions/artifacts/RC-lacon/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Real teams originate work in JIRA / GitHub Issues / similar, not in a shell prompt. Today the only origination path is `jigc start "<intent>"` — the intent string is hand-carried from wherever the work actually lives, and nothing in the managed record points back at the tracker item, so the tracker and the jigc record drift apart (the commit body may mention a ticket; nothing structured does).

## The shape

Determinism boundary first: the CLI can **carry** ids and links; judgment stays out. Candidates, cheapest first: (a) a structured `origin:` field on the task (and threaded into the commit doc as a trailer) — `jigc start --origin JIRA-123 "<intent>"`, pure data, no network; (b) a ref field-type for tracker URLs on relevant doctypes (spec/prd "tracked-at") — detect-and-route only, no API calls; (c) actual API integration (pull an issue's text as intent, push status back) — a whole adapter class, almost certainly post-pack-platform, and the LLM/agent side may already do this better through its own tools. Start at (a)/(b); (c) needs a real multi-tool team adopter to be worth designing.

## Trigger

A team adopter whose work originates in a tracker — or the first time a trial/adopter hand-carries ticket ids into intents and loses them.
