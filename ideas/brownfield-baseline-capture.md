# brownfield baseline capture — compile the terrain, not just the intent

**Status: parked 2026-07-12.** From the RC implementation-half trial, probe 3 ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md) → Probe 3). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

jigc compiles context about the *intent* (spec criteria, doc slices) and nothing about the *terrain*. On the trial's brownfield repo, three ground-truth facts changed the plan materially — 33 tests already red at HEAD, the suite running on SQLite so a MySQL-only view was untestable, ESLint broken at HEAD (which later blocked finalize through the pre-commit hook) — and the agent hand-rolled all three: snapshotting the failing set to a scratch file and `comm`-ing against it at the end.

> jigc could institutionalize that: a baseline capture step at jigc start on a brownfield task — snapshot test/lint/build state before any edit, so at finalize it can tell me "you fixed 51, broke 0, and these 33 were already red." […] It's the highest-value thing you could add. […] The context-compiler thesis is right […] it compiles context about the intent and not about the terrain — which is fine greenfield and is exactly the wrong half on brownfield, where the terrain is the thing lying to you.

## The shape

The determinism tension: jigc runs no project commands today (the gate-command knob was deferred at M15 as prose-is-better). The capture itself is deterministic *given* a configured command (run it, store the output/exit as the task's baseline artifact; at finalize, run again and report the delta — never judge it). Cascade-configured per project (`baseline: [<commands>]`), skip-when-absent. Kin: the M17 seeded-failure/build-health protocol counted this by hand; the invocation log's fd-tee shows the output-capture plumbing exists. The *comparison semantics* (which failures are "the same") is the hard part — start with verbatim-set diff, the same thing the agent hand-rolled.

## Trigger

An adopter on a brownfield repo (i.e., any adopter) — concretely, the next trial/adopter session that hand-rolls a baseline again, or the dev-task workflow's next revision.

**2026-07-15 — second demand, from the enforcement side** ([lacon trial](../completions/artifacts/RC-lacon/trial-record.md), task 3; verified [findings-verification](../completions/artifacts/RC-lacon/findings-verification.md) → B13): the tester asked for `jigc task check-red <test>` — run the test, record the observed failure, gate the implement step — calling test-first "the one promise the workflow makes and can't keep" (the pack's own text says "yours to police," candidly). That's this idea's capture mechanism pointed at a single test instead of the whole terrain. A de-park must engage the recorded test-running-gate deferral rationale (the A==A-oracle lint entry: a test-running gate breaches the CLI-runs-no-project-commands posture; the baseline shape here — cascade-configured command, capture + delta-report, never judge — is the boundary-respecting form).
