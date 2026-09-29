# Handover — M46, the combined pre-1.0 wave

Written 2026-08-18 by the session that ran and closed the 1.0.0-gate trial, while the context is
live. Nothing below needs reconstructing from a transcript.

## State you're inheriting

| | |
|---|---|
| HEAD | `e1ac87b`, pushed, tree clean |
| Binary | `1.0.0-rc.11` at `~/.local/bin/jigc` |
| Gate | 2735 passed / 0 failed as of M48 — **not re-measured today**; this session wrote docs and trial artifacts only, no `crates/` code |
| 1.0.0-gate trial | **complete** — `completions/artifacts/RC-1.0-gate/` |
| Conversion ledger | **closed**, 17/17 — the human's gate on the 1.0.0 call |
| Docker images | `jigc-gate:rc11` and `jigc-gate:rc10` (1d4f9bc) built and verified |

## What you're building

**M46 — one combined pre-1.0 wave.** The human's call, 2026-08-18: the chartered capability ledger
**plus** two defects the trial found, shipped together because all three are pre-1.0 debt and a
second wave boundary buys nothing. Then a trial, then the 1.0.0 call.

**Read the scope brief first**: [next-wave-scope.md](../RC-1.0-gate/next-wave-scope.md). Its claim —
*the disposition wave: a count is not a disposition* — is that no ledger entry leaves M46 deferred on
a count; each ships, is retired as dead, or is re-keyed to a condition that can fire on its own.

**Keep the number M46.** 25 references in [decisions-pending.md](../../../implementation/decisions-pending.md)
key deferrals to *"the M46 Settle"* as their literal trigger. I chartered it as M49 for one commit and
was corrected; renumbering orphans all of them for nothing.

## The two defects, with their root causes already located

Both were verified against the code, not inferred.

**PT-1** — `migrate-corpus` claims a never-adopted foreign file and prints a route that cannot run,
while `validate` routes the same file correctly and says the other surface's premise is wrong.
**Root cause: `engine::validate::is_unadopted_foreign` — M42's managed-vs-foreign discriminator — is
called from `doc.rs`, `validate.rs` and `file_state.rs`, and zero times from
`crates/cli/src/migrate_corpus.rs`.** The fix is reuse of a shipped predicate. The axis is *every
store-walking surface × every managed home, including the placement branch* — that branch is where
the root-`CHANGELOG.md` blast radius lives, and the blast radius is why the human's rule classes it
blocking.

**F-1** — `changelog-recording.gate-granted-unused` fires on a task that **did** record a changelog
entry. Keyed on staged-vs-committed repeatable-**item count**, not on the verb, so a `set-slot` edit
of an existing category is invisible to the tracker. Three aggravations: the advisory prints beside
`promoted CHANGELOG.md` in the same output; its two routes point at each other (the in-task option is
refused with `write.already-present`, whose route sends you back to `set-slot`); and it is a
**post-commit ambush** — `task validate` does not surface it, while `crates/cli/src/task.rs`'s own
comment asserts *"it is live on the `task validate` preview"*. **That makes it a sibling of M47's own
claim** (*"`task validate` previews what finalize gates on"*) through an un-swept axis, which is the
M45 complete-fix lens pointed at M47.

## Four things that will bite you

1. **A standing test pins the route M46 wants to change.**
   `start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal` asserts
   `err.contains("is pinned to base") && err.contains("jigc task discard do-the-thing")`. If you
   improve that route (B2-2), you meet a **red test on a correct change**. Revise the assertion; do
   not revert the fix, and do not add a second assertion beside a wrong one.
2. **One fork expires at the 1.0.0 call and cannot be deferred again** — a frozen methodology-doctype
   schema bump. On the Settle agenda with **both arms open** (decisions-pending → the capability
   wave). "Defer" and "take neither" are the same outcome here; the record must say which was chosen
   deliberately. Weigh it against the brief's top sequencing risk: **a schema bump makes the
   following trial about migration rather than about the fixes**, which is the opposite of what that
   trial is for.
3. **PT-1's fix invalidates walk arm 2** — the only rc.10→rc.11 continuation evidence the trial has.
   Plan its re-run into the wave rather than discovering it at acceptance.
4. **Fork 7 introduces a second declared behaviour change**, and §1's regression row exempts exactly
   one. **The next trial's protocol must be rewritten before it runs**, or the new refusal reads as a
   regression against its own rule.

## The next trial's instrument must change before it runs

The designed-need **cue card never fired in any of four sessions**. Measured: the window between its
trigger and the `finalize` that closed the doc's staged life was **3–5 seconds**. It was impossible,
not mistimed. Consequences: the correction path is *untested, not passed*, and **`doc rename` was
reached by nothing** — 0 calls across four sessions and the control. The instrument failure and that
coverage gap are the same event.

The replacement is designed and argued: [cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md)
recommends the **abandoned-task corpus** — ship the corpus with an open task already staged whose
doc's title is wrong in a way a committed doc settles. It reaches `doc rename` deterministically,
needs no operator utterance at all, and makes the read genuinely necessary.

**The rule it yields, worth applying before you build any instrument:** one fires reliably iff its
**trigger is a state**, its **consequence is re-raised by the product**, and **every worker behaviour
maps to a scored outcome**. Three plants satisfied it and fired; the cue card satisfied none and
did not.

## Lessons this session paid for

- **Reading a gate declaration is not running the command it gates.** I made three corrections to my
  own record, all the same shape — a static read inferring a dramatic conclusion the binary then
  refuted. The carryover gate did not fail (it was never given the case). The "seventh
  discoverability landing" was one worker of four, and the one without the adapter preloaded — 3 of 4
  used `doc schema`, 11 times. The pack does not lie about sub-task commit authorship (the step runs
  at exit 0 from the worktree the `Spawn:` line names). **Drive the binary before you write the
  finding.**
- **A citation is verified by what a test asserts, never by its name** — [pinning.md](../../../implementation/pinning.md)
  §3 addendum, with the three near-misses this ledger produced.
- **Read the logs, not the record, when they disagree.** The record carried B1 at 5 read-backs for a
  day; the log said 3 the whole time.
- **Measure exit codes unpiped.** `cmd | head` reports *head's* status. This caught me three times in
  one session, and zsh uses `pipestatus`, not `PIPESTATUS`.
- **`verb_suite_coverage` green means *named by a suite*, never *fenced*.** Misread five times in
  this project's history.

## What's already done, so you don't redo it

- Trial artifacts, all 47 files: `completions/artifacts/RC-1.0-gate/` — record, adversarial
  verification (17 claims), coverage (36/28/3), post-mortem, scope brief, plants, prompts, evidence.
- **Primary evidence archived into the repo** (`RC-1.0-gate/evidence/`) — invocation logs and
  transcripts for all four sessions plus the control. They previously existed on one machine only.
- **Reusable trial tooling, indexed in `CLAUDE.md`** so it isn't rebuilt a third time:
  `completions/trial-harness/` (isolated container, sha-pinned builds, 7 verification checks) and
  `completions/trial-corpus-template/` (`instantiate.sh --clean-prose`, `check-corpus.sh`).

## What I'd watch for

The scope brief was written by an agent and I verified its load-bearing claims, but **not all of
them** — I checked the numbering argument, both root causes, and entry 12's already-shipped status.
Its ledger dispositions (3 retired · 4 re-keyed · 1 split · 1 out as its own wave) are reasoned but
unverified by me. Re-derive any disposition you are about to act on.

And the brief's own honest note: the combination is feasible as one wave at ~10–12 increments, **but
only because the ledger closes by disposition rather than by build.** If that assumption breaks under
the Settle, the split is *not* defects-then-capability — the destroying-door guard must stay in the
pre-1.0 half, or it ships later as a breaking change to an adopted binary.
