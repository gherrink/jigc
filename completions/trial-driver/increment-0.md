# Increment 0 — the validity probe

**Question, pre-registered before the run:** does a session's read-back channel
behaviour survive being driven headless? Everything else in `trial-driver/` is
worthless if it does not, so this was run first, alone, with the result to be
written down whichever way it came out.

**Answer: yes under `bypassPermissions`, and the arm that was predicted to be
unusable turned out to be usable.** Both halves are below, including the
prediction that was wrong.

Run 2026-08-25 on `jigc-gate:rc11` (`JIGC_SHA=9a37f0152744f0cba5f9140483e1ca1b1c453c46`,
`jigc 1.0.0-rc.11`, `claude-sonnet-5`). Evidence in
[increment-0-evidence/](increment-0-evidence/).

## The design

Three arms, of which **only two had to be bought**: the archive already *is* the
interactive rc.11 baseline, and `jigc-gate:rc11` was still on disk, so the
comparison is same-image and same-prompt rather than same-version. The operator
session the plan budgeted for dropped out entirely.

Every arm ran the identical B3 blind prompt
([paste/b3-prompt.txt](../artifacts/RC-1.0-gate/paste/b3-prompt.txt)) against a
corpus of the same shape: `instantiate.sh --clean-prose`, gated 11/11, then adopted
by the **container's own** jigc via `arms/adopt.sh` through `run-session.sh --exec`
— 9 commits, adapter present, invocation log on, exactly `corpora.md`'s specified
end state, performed by a script instead of by prose.

## The result

| arm | driven | records | authoring writes | **VERB** | §3.3 adj | fs | outcome |
|---|---|---|---|---|---|---|---|
| B3a `stonefly` | interactive (archive) | 94 | 53 | **6** | 5 | 0 | VERB |
| B3b `rosewater` | interactive (archive) | 99 | 41 | **7** | 6 | 1† | VERB |
| **`b3-bypass`** | **headless, `bypassPermissions`** | **81** | **36** | **6** | **7** | **0** | **VERB** |
| **`b3-strict`** | **headless, `--strict-permissions`** | **13** | **0** | **0** | **0** | **0** | **VOID — unmeasured** |

† the planted foreign ADR, which the record dispositions as permitted.

**The channel does not invert.** Headless `bypassPermissions` scored **6** read-backs
against the archive's 6 and 7, on the same image and the same prompt, and completed
the whole arc: four pieces, six finalize commits, 39/39 tests green, **0** permission
denials, 120 turns, `subtype: success`. The gate is passed and the driver's scored use
is not narrowed.

## The prediction that was wrong

The plan carried a bound, B2, taken from `knowledge/17`: *project allow rules are
ignored under `-p`*. If true, `jigc setup`'s `Bash(jigc:*)` allowlist would be inert
headless, and `--strict-permissions` would **misrepresent** the adopter's asymmetry
rather than merely flatten it — so the adopter-condition arm could never be driven
unattended.

Measured, it does not hold on this build. In `b3-strict`:

- **13 of 13 jigc invocations executed** under `--permission-mode default`;
- **all 7 permission denials were file writes** — the `Write` tool, and Bash `>`
  redirects (`cat > /work/tmp-changelog-payload.yaml`);
- zero jigc calls were denied.

The allowlist was honoured. **`--strict-permissions` headless therefore reproduces
the adopter's real condition** — jigc frictionless, raw file writes gated — which is
the condition the trial most wants and has never once been able to run unattended.
Recorded rather than quietly deleted, because a bound that evaporates is how a false
premise survives.

## What the strict arm actually shows

Its `0 VERB` is **not** a channel inversion and must not be read as one. The arc
truncated at 13 records having authored nothing, so no read-back occasion ever
existed — which §3.3 scores `unmeasured`, not `NEITHER`, and which the cascade now
voids.

The truncation is itself a finding about the adopter condition, and a clean one: the
worker tried four times to stage an authoring payload as a temp file
(`.tmp-changelog.yaml`, `tmp-changelog-payload.yaml`, a heredoc, a write probe), was
denied each time, **named the correct escape itself** — *"e.g. piping directly via
`jigc doc author --from-file -`"* — and still stopped to ask rather than take it.
Whether that generalises is untested at n=1.

## What it cost the apparatus, and what that is worth

Three defects, none of which reading would have found. Each is now pinned by a test
naming this run.

1. **A missing cascade row.** The strict arm would have scored `proceeded without
   reading` — a claim about the worker its own evidence does not support. §3.3 is
   explicit that no-authoring-occasion is `unmeasured`, and the cascade did not
   implement it.
2. **`jigc doc author --help` counted as an authoring write**, manufacturing a
   read-back occasion for a session that had none and converting an honest
   `unmeasured` into a false `NEITHER`.
3. **A halt is indistinguishable from success.** The strict turn ended asking the
   operator to approve a write, at exit **0**, `subtype: success`,
   `is_error: false`, and an empty stderr. Only `permission_denials` and the text of
   `result` reveal it stopped mid-task — so the process exit code cannot be the
   completion signal.

## Bounds

- **n = 1 per arm, one corpus shape, one model alias, one day, one CLI build.**
  This licenses *"the channel survives the transport"* and nothing quantitative.
- **The interactive arms are archived, not re-run.** Same image and prompt, but they
  were driven by a human on 2026-08-17 and these were not; nothing here separates a
  transport effect from an eight-day-apart effect.
- **`bypassPermissions` remains protocol §9's declared directional confound.** It is
  unchanged by this probe, which compares transports *within* it.
- **Nothing here says a headless session is a valid substitute for a blind session in
  a scored trial.** It says the read-back channel behaves the same way. Whether the
  next trial's scored sessions run headless is a separate decision, on this evidence
  plus whatever else the human wants.
