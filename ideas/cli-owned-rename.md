# CLI-owned rename — make the rename a structural operation, not a forbidden one

> **GRADUATED at M35 planning (2026-06-28) — this is the shaping doc, not the settled design.**
> All eight mechanism opens are settled and the identity-model invariant is amended. The
> **canonical record is [DECISIONS.md](../DECISIONS.md) → 2026-06-28 M35 planning**; the settled
> surface is `jigc rename <old> --to "<New Title>"` (top-level, retitle+reslug+repoint unified —
> *not* the `jigc doc rename … --to <new-slug>` sketched below), specced in
> [write-commands.md](../design/write-commands.md) → `jigc rename`, with the OOB A+B backstop in
> [reconciliation.md](../design/reconciliation.md)/[validation.md](../design/validation.md) and the
> acceptance flow in [worked-examples.md](../design/worked-examples.md) → flow 37. Where this doc
> and those disagree, those win. Kept for the *why* (the reframe, the cost-win thesis).

**Status:** shaped; **direction decided** (user, 2026-06-24); **mechanism settled at M35 planning
(2026-06-28)** — see the banner above. Scheduled as **M35**. Surfaced by the cross-doc forward-ref integrity study
([completions/artifacts/cross-doc-refint-study/VERDICT.md](../completions/artifacts/cross-doc-refint-study/VERDICT.md)),
whose refutation exposed that jigc was enforcing the wrong *verb*. **Cross-reviewed by Codex
(gpt-5.5, 2026-06-24)** before commit; its corrections are worked in inline (claim softened from
"structurally cannot replicate" to "canonical transaction-bound interface"; the identity-model
amendment stated bluntly; collision / git-history / OOB-precedence / rollback / cost-confound
holes added).

## Where this came from (the study's deeper lesson)

The cross-doc study tested whether jigc's blocking `ref-resolves` hook beats a static
`CLAUDE.md` at keeping `supersedes`/`cites` references valid as decisions are renamed/deleted.
It **refuted** the claim: a static rule ("grep `docs/` for the old slug and fix every
reference") tied jigc, because a dangling slug in a plain-file store is *greppable* — the
broken ref literally contains the old slug. The study tested **detection** (catch the dangle
after the fact), and detection is exactly what a `grep` rule also does.

Two things that refutation exposed:

1. **The rename-dangle is artificial for jigc by its own design.** jigc *freezes* slugs (the
   "stable opaque IDs, never positions" invariant), so in a correctly-operated jigc project a
   rename shouldn't change the slug at all — you change the title, the slug stays, refs never
   break. The study's jigc arm only dangled because the agent *re-slugged the files*, which is
   un-jigc-like. So "jigc catches the dangling ref" proves little.
2. **Frozen slugs fight the agent's natural model.** In a non-jigc project a cross-reference is
   an ordinary path/markdown link the agent renames-and-updates as a matter of course. A frozen
   slug `adr:csrf-double-submit-cookie` sitting under a decision now titled *"CSRF strict
   origin"* reads stale — like a variable named `oldName` holding new data. It is *valid* but
   *illegible*, and it asks the agent to suppress an instinct (rename to match) the rest of the
   world rewards.

## The reframe: own the rename, don't forbid it

The differentiator is not *detecting* a dangle — it is **owning the rename as a deterministic
operation, shipped as the canonical managed-doc interface**:

> `jigc doc rename adr:csrf-double-submit-cookie --to csrf-origin-validation`
> → the CLI rewrites **every structured referrer's ref field across the managed store**,
> atomically, including referrers the agent never opened — then `git mv`s the file, one
> transaction.

This is the project's actual thesis — *take the structural operation from the LLM, give it to
the CLI* — applied to renaming.

**Honest scope of the claim (a Codex cross-review correction, 2026-06-24).** A static project
is *not* structurally incapable of the *effect* — an agent there can approximate it with
`git mv` + scripted `grep`/`sd`, an ADR linter, or IDE rename. The differentiator is weaker and
truer than "static can't do this": **jigc ships the rename as the *canonical, transaction-bound
managed-doc interface*** — one command, every structured ref, atomic, with the edge index as
the source of truth — versus a static project's ad-hoc, per-project, error-prone scripting. The
study's cost study (below) must therefore allow the static arm a **shell one-liner**, not only
manual N-turn editing, or it tests a straw baseline.

**The honest justification (precise, from the data).** The plain arm drifted to **4 dangling
refs** — the agent naturally **renames the thing in front of it** (the file + its own H1) but,
**absent an explicit rule or operation**, does not chase **referrers elsewhere**. The
*static-rule* arm *did* chase them (the verdict's "same-intent" finding — refs are greppable, so
an instructed agent fixes them). So the tool isn't redundant with the *instructed* agent on
greppable refs; its value is (a) **on the non-greppable referrers** an instruction can't direct
the agent to, and (b) **as one cheap command** instead of N turns of grep-and-edit — it
**completes and cheapens the half the agent reaches for but does manually.**

## Why this is the win the prior studies couldn't show

- **It inverts the cost story.** Today jigc's enforcement makes renames *more* expensive (the
  block → re-orient → recover loop — the ceremony tax measured in the long-horizon study). A
  `jigc doc rename` makes jigc **cheaper** than manual ref-chasing: one command vs. N turns of
  grep-and-edit. This is the first surface where jigc could plausibly **beat plain on cost**,
  not just tie static on correctness — and it directly relieves the
  [cost-of-enforcement](cost-of-enforcement.md) concern.
- **It wins on the genuinely non-greppable referrers.** A referrer the agent *cannot* find by
  grepping the old slug — a rename where the agent never learned the old slug (a semantic
  ticket), or a ref the agent has no reason to search for — is what the CLI's edge-index walk
  catches and a manual grep misses. (A transitive `supersedes` chain is **not** an example: the
  old slug still appears textually in the chain, so it's greppable — the non-greppable case must
  be defined precisely, and the cost study must construct it deliberately.)
- **Detection becomes the backstop, not the headline.** `jigc doc rename` is the happy path
  (prevention). The store-wide `ref-resolves` sweep we shipped this session
  ([validation.md](../design/validation.md) → the fourth family) catches an *out-of-band*
  rename (agent renames a file directly, not via the verb). They **compose**: the verb prevents,
  the hook backstops. This session's work is the safety net under the new operation.

## Engaging the invariant (the record is rebuttable — engage the rationale)

This touches `VISION.md`'s **"stable opaque IDs, never positions"** and, more sharply,
[storage.md](../design/storage.md)'s **"identity *is* the path; a path rename *is* an identity
change."** Be blunt about what changes (a Codex cross-review correction — the first draft was too
soft here):

- **This is a genuine identity-model revision, not just a mechanism swap.** Today a doc's
  identity is *immutable* (the frozen slug/path). After this, **identity becomes
  *transactionally refactorable by the CLI*** — a `jigc doc rename` *does* change a doc's
  identity, and that is the point. Stating it as "ref-stability survives via a different
  mechanism" understates it: **identity stability does not survive a rename — only
  *reference-integrity* does** (refs are repointed atomically as identity changes).
- **What the invariant keeps:** stability **under reorder and ordinary retitle-without-reslug**
  — those still never change a slug. The amended invariant reads roughly: *IDs are stable under
  reorder and ordinary edit; an identity change is allowed only through an explicit, atomic CLI
  refactoring operation* (never an untracked file move — the `reconciliation.rename` / store
  backstop catches that). That is the surgical line: not "IDs never change," but "IDs change
  only through one owned, atomic operation."
- **Diff-legibility *improves*.** Content-slugs were chosen for diff-legibility; a slug that
  tracks its title is *more* legible than one frozen out of sync. We explicitly **reject the
  opaque-ID-indirection alternative** (refs like `adr:7f3a`), which would trade away the very
  legibility the content-slug design exists for.

So: the invariant's *intent* (cross-refs never dangle under structural change) is preserved; its
*mechanism* moves from immutability to atomic CLI refactoring; and its *literal claim* ("IDs are
stable / immutable") is genuinely amended. Principled revision, named honestly — recorded as
such in `DECISIONS.md`, and promoted into VISION + storage at M35 planning.

## The determinism boundary holds — and draws one honest line

The CLI owns the **structured refs** (`supersedes`/`cites` *fields*) **inside the managed
store** and rewrites exactly those. Two things it does **not** own, so the verb must not claim
"every referrer / completely" (a Codex cross-review correction):

- **Prose mentions** of the slug inside managed docs ("see the csrf-double-submit-cookie
  decision") — prose the CLI doesn't author; stays the agent's job.
- **Unmanaged-store references** — code comments, `README`, tests, migration notes, issue
  templates, external docs, URLs. The CLI cannot rewrite these and must not pretend to.

For both, the verb should **report textual occurrences it finds** (a boundary-safe *report*,
never a rewrite) so the agent can fix them, and the cost study must **score completeness only
over structured managed refs** — never count unmanaged/prose mentions as CLI-owned completeness.
This is exactly the determinism boundary (CLI owns structure, LLM owns prose), and naming it
precisely keeps the operation from over-promising.

## Buildability — cheap, and on infrastructure we already have

- **The inverse edge walk exists.** The committed edge index already stores forward edges; "who
  points at this doc?" is their inverse (derived, not stored — the invariant) and computable
  from the same `rebuild_committed` path the `ref-resolves` sweep walks.
- **Writes are already transactional** — but rename needs its **own rollback inventory.** Unlike
  `finalize` (which promotes a task working-area then commits), rename mutates **committed paths
  directly** + file-state + the edge index + git staging. A Codex cross-review flagged this: the
  operation must define rollback for the partial-failure cases (`git mv` succeeds but a referrer
  rewrite, a hook, or the commit fails). Plan the transaction boundary explicitly at M35; don't
  assume the existing `finalize` boundary covers it.
- So `jigc doc rename` ≈ validate (target exists, new slug free, tree clean, not mid-fan-out) →
  compute the target's inverse edges → rewrite each referrer's ref field (old slug → new) →
  `git mv` the file → re-baseline file-state + rebuild the index → commit — as one transaction
  with a defined rollback.

## Open questions for milestone planning to settle (mechanism)

- **Rename vs retitle.** Is the surface a bare `rename <old> --to <new-slug>`, or a `retitle`
  that changes the H1 *and* re-slugs to match *and* rewrites refs in one motion (closer to what
  the agent naturally wants — "this decision is now about X")? The latter is more ergonomic but
  couples slug to title; decide deliberately.
- **Collision on `--to`.** When the new slug already names an existing doc, rename must
  **block** (this is an identity refactor of an existing doc, not a fresh mint) — *not* silently
  suffix the way the by-task-id join suffixes colliding *new* docs ([storage.md](../design/storage.md)
  → the join's deterministic suffixing). Confirm and specify the block.
- **Out-of-band rename — precedence between two detectors.** A direct `git mv` (not via the
  verb) is caught two ways: the M20 store-wide `ref-resolves` sweep (a dangling ref appears),
  **and** reconciliation's *stronger* rename classifier — *missing tracked path + content-hash
  match* ([reconciliation.md](../design/reconciliation.md) → rename-detect). Decide the
  **precedence**: `reconciliation.rename` should classify the move *first* (and offer "adopt this
  as a `jigc doc rename` — re-point refs" vs. "revert"), with `ref-resolves` as the fallback —
  else the two emit competing diagnoses for the same event.
- **Git-history continuity.** Git infers renames heuristically; bundling the `git mv` with N
  referrer rewrites (and possibly H1/prose) in one commit degrades `git log --follow` and review
  — which cuts against the diff-legibility selling point. Constrain the rename commit (e.g. *move
  + structured-ref rewrites only*; keep title/prose changes separate unless an explicit
  `retitle`).
- **Concurrency / fan-out.** **Hard constraint unless full join semantics is written:** the
  by-task-id join classifies docs `created` vs `edited-from-base` and detects same-doc clashes
  *by key* ([storage.md](../design/storage.md) → the join) — a rename *changes that key*, so one
  task editing `adr:old` while another renames it to `adr:new` would split refs + provenance
  across identities. Default: **forbid rename inside a fan-out**; keep it a top-level op.
- **Prose / unmanaged mentions.** Out of scope for the CLI rewrite (determinism-boundary
  section) — but the verb should **report** textual occurrences of the old slug it finds (in
  managed prose *and* unmanaged files: code comments, README, tests) so the agent can fix them.
  A report, not a rewrite — boundary-safe.
- **Migration of existing corpora.** Existing frozen slugs need no migration — they simply
  become renameable. Confirm no `decisions-pending` entry forbids mutable slugs before shipping.

## Acceptance — the post-build cost study (where jigc beats plain)

The milestone's proof is a **study that measures cost + completeness, not just correctness**,
reusing the cross-doc harness:

- **Arms:** jigc-with-`doc rename` vs plain vs static-rule, on a rename-heavy sequence over the
  same multi-doc store (referrers at varying hop distance, **including a deliberately-constructed
  non-greppable case** — a rename where the old slug is not textually discoverable, since a
  transitive chain *is* greppable).
- **Metrics:** (1) **turns + cost per rename** — the new headline; jigc should be *cheaper* (one
  command) where plain/static burn turns hand-editing; (2) **dangling refs over structured
  managed refs** — jigc 0 by construction; plain drifts; static ties only on greppable refs;
  (3) **completeness on the non-greppable referrer** — the case static cannot reach. Score (2)/(3)
  over **structured managed refs only**, never unmanaged/prose mentions.
- **The pre-registered win condition:** jigc strictly beats plain on cost **and** ≥ static on
  completeness — the first study where jigc wins on *effort*, turning the enforcement tax into a
  labor saving.
- **Confounds to pre-register (a Codex cross-review correction — control these or the win is not
  valid):**
  - **Verb-availability ≠ induced-usage.** Separate "the verb exists" from "the adapter actually
    got the agent to use it." If the agent ignores `doc rename` and hand-edits, jigc pays the
    *same* block→recovery cost the cross-doc study identified as the cost driver — and the cost
    win evaporates. **Measure verb-engagement as a primary quantity, not a footnote.**
  - **The command erroring → its own recovery loop.** A `rename` that errors on collision, a
    dirty tree, OOB drift, or fan-out context can trigger exactly the re-orient loop we're trying
    to avoid. Count rename-error→recovery cycles.
  - **Static arm must be allowed a shell one-liner** (`git mv` + `sd`/`grep`), not only manual
    N-turn editing — else it's a straw baseline and any jigc cost-win is against a weakling.
  - **Small-N effort noise.** Short rename tasks make per-rename cost noisy; size the sequence so
    the signal dominates.
- **Honest bound:** the win requires the agent to *use* the verb — adapter-advertised, not
  sandboxed; if it hand-edits anyway, the `ref-resolves` hook + `reconciliation.rename` still
  backstop correctness but the cost win is forfeited.

## Next step when scheduled

Run M35 planning: settle the rename-vs-retitle surface + the four other mechanism opens above,
promote the build design into [write-commands.md](../design/write-commands.md) (the verb) +
[storage.md](../design/storage.md) (the mutable-slug identity revision) + [VISION.md](../VISION.md)
(the invariant amendment), then build verb → backstop-compose → the cost study as acceptance.
