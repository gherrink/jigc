# M52 — the rc.16 wave (the complete-class wave) · VERDICT

**Status: complete.** Built + audited — code review **1 HIGH · 1 LOW**, e2e **11 of 16 scenarios**
green through the real binary (its five reds: one HIGH, two MEDIUM, two LOW) — **seven audit findings,
all confirmed live before any fix and all fixed axis-complete**, in six commits (one fixer took a
second finding as the same class), re-verified **3798 passed / 0 failed**, clippy + fmt clean.
**`1.0.0-rc.16` is built and installed after the fixes, not before** — the act that follows this
record, with the ten version-bearing goldens regenerated in the same motion and their diff checked to
carry nothing but the version string.

Planning: [settle-record](settle-record.md) (D1–D13, amendments §1–§19) · [baseline-ledger](baseline-ledger.md)
(+ seven companions) · [gap-findings](gap-findings.md) (+ four) · [advocates/](advocates/) ·
[acceptance-design](acceptance-design.md) · [planning-gate-record](planning-gate-record.md) ·
reviews [design-review](design-review.md) / [codex-design-review](codex-design-review.md). Decomposition:
[roadmap](../../../implementation/roadmap.md) → Milestone 52. Acceptance:
[worked-examples](../../../design/worked-examples.md) → flow 53 / `crates/cli/tests/flow53_acceptance.rs`.
The audit's raw reports: [audit/code-review.json](audit/code-review.json) · [audit/e2e.json](audit/e2e.json).

---

## What the wave claimed, and whether it is true of what shipped

> **No byte dies and no third party's bytes are silently discarded at exit 0 behind a committing,
> destroying or moving door; every repository posture and every route a caller can reach answers
> with a code and a followable route; and every surface 1.0.0 pins says what the binary does — with
> each fix complete over its class's axis, machine-checkable.**

**The first two clauses hold, driven, and the third is what the audit corrected.** The e2e half drove
every tier-0 class through the real binary and found each working as claimed: every rollback
population kept a racing hook's bytes and emitted exactly one JSON document on stderr; all nine
un-concluded git states refused at every acting door with routes a real shell ran and git accepted;
every destroying door answered a foreign byte through its own disposition — refuse, narrate, or
displace to `.jigc/displaced/<id>/` with collision safety — and never tripped on jigc's own files; the
fixed-identity predicate refused at all eight address doors for all five derived doctypes at
`contract-version` 7; every pre-dispatch fault answered `--format json` with one document; all 14
verb-routed workflows refused at both compose doors; and the fan-out drove byte-identically across
three divergent orders. **No audit finding is data loss, corruption or a regression on a loss cell.**

**The corrections are inside the third clause and the fourth phrase.** Two of the seven were the
wave's own new store-scope check saying something the binary should not: `home-vacated` fired on a
brownfield repo's pre-jigc history (a false red on the CI-gated verb, with no exit that cleared it),
and `validate` stayed green over an unmigrated doc stranded at a prior home the walk had learned to
see but the sweep had not (the M42 false-green class reopened one axis over). Both were **incomplete
sweeps of the wave's own class** — the history leg lacked the discriminator D7 named in its own
words, and D4's *every prior home* was applied to the migrate side and not the store side. The
complete-fix lens landed on the wave's own work for the **sixth consecutive wave**, and every fixer
found a larger class than the finding reported.

## The audit — seven findings, seven fixed, nothing deferred that is not triggered

Every finding was **reproduced by its fixer on the release-posture binary before a line changed**,
fixed over the class's axis rather than the reported instance, gated bare, and committed with its own
`DECISIONS.md` entry. Fixers ran **one at a time** on the shared branch.

### F1 (HIGH, code review) — `home-vacated` asks what the history carries, not that it carries something

**Reported:** `schema-conformance.home-vacated` fires on *any* history at a declared path, so a stock
brownfield repo that once had and deleted a `CHANGELOG.md` is red at exit 1 the moment `jigc setup`
runs, with no exit that clears it — a measured regression against rc.15. **Confirmed** on a `bare`
rig; the installed rc.15 answered *validates clean* on the identical tree. **The class** is
`orphan::fixed_identity_homes` over the resolved schemas — **five** members across both packs, and
all five fired on one `validate`. **Fixed** (`79e54c75`): the history leg becomes *jigc committed
into* — the last committed blob at the exact declared path (or its first parent where that commit
removed it) must carry jigc's `schema-version:` stamp; `Territory` moves by zero bytes so M51's HIGH
narrowing is untouched; git failure stays conservative. **Bound stated, not hidden:** a pre-jigc doc
carrying a `schema-version:` key at precisely one of those exact paths still fires — accepted on the
exact-path ground, with the namespaced-stamp trigger as the honest fix, and pinned as expected
output. **Why no test caught it:** every `home_vacated.rs` arm built an adopted corpus and the
never-adopted state was rig-only; `TrialCorpus::build_never_adopted()` now exists, deliberately not a
`State::ALL` member.

### F2 (HIGH, e2e Finding 1 — and Finding 5, the same class) — the store sweep's subject is every home a doctype has declared

**Reported:** after a schema bump that also moved a doctype's home off a repo-root placement file,
`validate` printed *validates clean* at exit 0 and `doc list` said *no committed docs* over an
unmigrated below-version doc that `migrate-corpus --dry-run` saw. **Confirmed** on both the repo-root
cell and its location cousin (where `orphaned-instance` fired with the wrong diagnosis — Finding 5).
**The class:** 12 `committed_instances` call sites, four fixed (the conformance sweep, `doc list`, the
baseline trailer, the orphan walk's claimed set) and eight left with a stated reason each (address
resolution, file-state, shape advisories, compose-time store values, `vacated_homes`). **Fixed**
(`6d95756c`): the sweep's subject is every prior home of the doctype through the shared
`migrate_corpus::prior_homes` derivation, narrowed by `is_unmigrated` after the suite falsified the
first cut's premise (a re-pinned at-version reshape must stay the orphan family's). One stranded doc,
one finding, routed at `migrate-corpus`. `doc list` shows the row `managed` at the stale home — no new
declared key. **Bound:** `ingest`'s route for that doc says *move, then re-ingest* rather than naming
`migrate-corpus` (a different door's subject); `doc show` stays at the resolved home by decision.

### F3 (MEDIUM, e2e Finding 2) — the survivable frame's state clause is a function of the rollback's outcome

**Reported:** `RejectionFrame::survived` was a per-door constant composed before the rollback ran, so
beside a `rollback-conflict` or `foreign-bytes` finding it contradicted the finding two lines below,
and in one cell its re-run instruction was refused. **Confirmed** on both repros; the re-run in the
first was refused too, which the finding had not claimed. **The class:** the grep found 7 frame
constructions over 10 doors, but the axis is `COMMITTING_DOORS ∩ {rollback populations that can leave
a path standing}` — **9 doors, 12 driven cells** where 2 cells at 2 doors were reported. **Fixed**
(`fe8f29c4`): the clause opens with the exception and prescribes the re-run only behind the
survivors' routes; the count is threaded at the single render seam all three arms pass through, so
the non-hook cell cannot take the constant — a new arm reddens at the type level. **Bound:** the
non-hook cell × a surviving path cannot be driven (no racer instrument exists for it) and is covered by
the shared composition, stated.

### F4 (MEDIUM, e2e Finding 3) — a store code the contract keys owes the envelope at every producer

**Reported:** `command-output-contract.md:292` said the `store.*` siblings are keyed; driven,
`store.unknown-type` and `store.not-found` flattened into `{error}` at three doors. **Confirmed.**
**The class:** **5 doors / 7 coordinates, not 3 / 4** — `task bind` and `milestone add-from-spec`
were in no finding and each diverged inside one function. **Fixed** (`c96137e4`): `ENVELOPE_OWED_CODES`
makes the arm the **code's** property at every producer (13 constructor sites through named
constants); `store.malformed-slug` stays flattened on M50's measured grounds, fenced as the negative
half. The text arm gained the orientation footer at the seven moved cells (the declared byte change
M51's 25-door move took), and a `pre_dispatch_faults` cell that had been a false `Unreached`
declaration was made visible by the move and re-declared. **Flagged, not fixed:** the contract's wider
`:197` rule is broader than the binary and is a separate act.

### F5 (LOW, e2e Finding 4) — a door's refusal set is an axis, and six of `relocate`'s ten carried no code

**Reported:** six user-reachable `relocate` refusals with no code and no route. **Confirmed.** **The
class:** `RelocateRefusal::ALL` — **ten members / nine codes**, ⇔-fenced; the seventh grep hit was
unreachable by construction and is left with its reason. **Fixed** (`b9ab6a70`): `relocate.frozen-doctype`
and `relocate.malformed-prior-home` minted, `write.untrackable-destination` closed at the shared
`move_doc` primitive (a declared bound discharged), the rest reused. **Two of the brief's premises
were stale** and the record says so: `UNSWEPT_PRODUCERS` counts printed-path sites, not route gaps
(it stands at 74 across ten files, untouched), and `task bind`'s two bare refusals were already
discharged at Increment 10 — but the door had **four**, and the two uncounted ones now carry
`task-bind.undeclared-role` / `task-bind.role-type-mismatch`. **Declared:** the new codes flatten
with code and route rather than joining `ENVELOPE_OWED_CODES` — that set is scoped to codes the
contract lists under a declared target form, and joining it is a fifth pre-pin contract act.

### F6 (LOW, code review) — the vacated home's route is a function of the removal's state

**Reported:** on an *uncommitted* deletion the route's `git log --diff-filter=D` locator named
nothing and the repair it prescribed was not the one that state needed. **Confirmed**; the locator
run verbatim printed nothing. **The class:** the brief named five states; driven, they collapse onto
**three** git-observable ones (`orphan::Removal`: committed · worktree-only · staged removal),
because the repair turns on where the removal is, not why. **Fixed** (`1b036264`): each state its own
route, the locator run before it is printed and dropped when it answers nothing. **The one-finding
rule, decided with a datum:** both `reconciliation.rename` and `home-vacated` fire on the uncommitted
states — on a fresh clone the file-state gate does not exist, so `home-vacated` is the only voice
there; deferring would trade a wrong route for silence. **Found and not fixed (out of class):**
`reconciliation.rename`'s own route names `jigc rename` on the uncommitted-`mv` cell, which a
fixed-identity doctype refuses — carried below with a trigger.

## The e2e half — 16 scenarios, 11 green

Every flow-53 arm but arm 4c held, plus the fan-out sim, the milestone-boundary displacement (the
§18 door) and the misuse sweep. The five reds are F2–F6 above (Finding 5 folded into F2), all
surface- or route-tier, none a loss cell. The auditor's own bounds: the sub-task boundary under a
revert and the non-hook survivor cell were composed, not driven; `chmod 000` arms pass vacuously as
root; git 2.54.0 throughout.

## The instrument, honestly

- **Two mid-build halts were adjudicated by the human, not guessed** ([settle-record](settle-record.md)
  §18: the fifth destroying door; §19: the sixth rollback door was a transcription error against the
  advocate's own concession). Both were contradictions inside the Settle's own text that the planner
  found by driving, which is the instrument working.
- **The harness run was interrupted once** (the shell stopped mid-increment-9); the killed task had
  passed its gate and was committed as *recovered*, the increment finished outside the harness
  (validator FAIL → two fixers → PASS), and a fresh run skipped through it. The old run's loop never
  exited; that is recorded so the next session does not re-learn it.
- **Every fixer found a larger class than its finding** — 5 homes for 1, 12 call sites for 1, 9 doors
  for 2, 5 doors for 3, 10 refusals for 6, 3 states for 5 — the sixth consecutive wave with that
  signature, now the expected shape of an audit finding rather than a surprise.

## Declared bounds, carried in writing

- **Zero schema-hash movement, zero `schema-version` bumps, zero corpus migrations** — the negative
  fence over both packs' schemas, both manifests and every `schema-snapshots/` directory is empty
  across the whole wave, planning base to this record.
- The git-2.54.0 marker contract is a **deferral with a trigger**; `GIT_DIR` is the one declared
  posture residual; the departed-doctype orphan half stays declared and crosses the pin; the
  pre-jigc-stamped-file cell of `home-vacated` is stated on the finding itself.
- `jigc task finalize` gains no `--force`; the displacing doors have one mode; the displacement
  behaviour is reversible after 1.0 while the `displaced` key is not; clearing `.jigc/displaced/` is
  the human's act, named in the route.
- The guide hash moved **twice**, not once (Increment 3 landed the posture preview before Increment
  10's batch) — the one-batch rule is written with this wave's own violation as its first datum.
- **Carried, with triggers** (added to [decisions-pending.md](../../../implementation/decisions-pending.md)):
  `reconciliation.rename`'s route on a fixed-identity doctype's uncommitted move (F6's out-of-class
  find; trigger: the next wave that touches that producer or the next adopter finding on it);
  `command-output-contract.md:197`'s wider rule vs the three flattened exemptions (F4; trigger: the
  next contract act); `ingest`'s route for a below-version doc at a prior home (F2; trigger: the next
  `ingest` surface change).
- **No mechanical checker fences the conversion ledger** — [pinning.md](../../../implementation/pinning.md)
  §3, unchanged.

## What is next

`1.0.0-rc.16` built and installed after these fixes; the ten version-bearing goldens regenerated and
their diff checked to be the version string only; the per-axis review re-run on that installed binary
with M51's preserved instrument, persisted under `per-axis-review/` beside this record and compared
row by row against M51's. **Then the 1.0.0 call, which is the human's.**
