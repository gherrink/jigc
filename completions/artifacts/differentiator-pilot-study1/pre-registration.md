# Differentiator pilot — Study 1 pre-registration

**Status:** PRE-REGISTERED, awaiting sign-off. Written 2026-06-21, before any
measured arm has run. Once signed off, the protocol below is fixed; deviations
are recorded as amendments with reasons.

**Reversibility:** everything runs on throwaway container twins of a baseline
commit. Nothing touches the real `gherrink-ui-doc` or the jigc repo. Records
export to `completions/artifacts/<pilot>/` as plain files. Zero corpus lock-in.

---

## 1. Question (the M17-relocated hypothesis)

Does jigc make a coding agent **measurably more correct** than a static
`CLAUDE.md` *when a differentiator is actually engaged*? M17 tied because its
task engaged **none** of jigc's differentiators. This study engages exactly one:
**doc↔code consistency** — jigc mechanically blocks a finalize when a managed
document references a code symbol that no longer exists.

The narrow, honest claim under test: **jigc guarantees the managed architecture
doc's anchored symbols stay valid across a refactor; freeform methods may ship a
document that names a renamed symbol.**

---

## 2. Design decisions settled this session (deviations from the handover, for transparency)

These were settled by verification + user decision before pre-registration:

1. **Task = public-API *symbol* rename, not the `@hideCode` tag rename.**
   Verification falsified the handover's premise: jigc's `symbol-exists`
   predicate matches *declaration symbols*, not string-literal values, and every
   tag transformer exports the same generic symbol `tag` — so a `name: 'hideCode'`
   string rename **cannot** fire jigc's validation. Renaming a documented
   declaration symbol (a `class`) engages the *built* predicate honestly. *(User-approved.)*
2. **The managed doctype = jigc's shipped `arch-doc`, not a bespoke pack.** The
   binary loads doctypes only from its embedded pack (the project layer *shadows*
   existing ids, never *adds* a new one). `arch-doc` is purpose-built for
   "components and the code that implements them," each component carrying an
   `implemented-by` code-anchor. Using it tests jigc's *real shipped capability*
   with zero custom-pack risk — the pinned binary is unmodified. *(Within the
   delegated "domain-pack scope — keep minimal" decision.)*
3. **Four arms, incl. the static-methodology control.** *(User-approved.)*
4. **Both model tiers** — Sonnet 4.6 + Opus 4.8. *(User-approved.)*
5. **GSD installed clean** via `npx @opengsd/gsd-core@latest --claude --global
   --portable-hooks` (faithful by construction; portable hooks fix container
   bind-mount paths). *(User-suggested.)*

---

## 3. The four arms

All arms are container twins of `gherrink-ui-doc @ 542b3206` (2024-10-03, the
pre-AI-era HEAD — its AI-era `docs/` tree does not exist, avoiding contamination).
The **only** documentation artifact under measurement is byte-identical across
all four twins: `docs/architecture/core-public-api.md` (sha
`5e6496f7…`), documenting three components (`UIDoc`, `CommentBlockParser`,
`MarkdownDescriptionParser`), each recording `implemented-by: <path>#<Symbol>`.
The baseline `packages/core/README.md` is identical baseline across all arms (a
secondary, equal drift surface that **no** arm's tooling polices).

| Arm | Method | Build | The only variable |
|---|---|---|---|
| **A · jigc** | `jigc setup` + the arch-doc authored as a **managed** doc (anchors validated at finalize) | `pilot-jigc`: jigc + doc-code binaries on PATH; `.jigc/` + committed managed arch-doc | the arch-doc is **validated** |
| **B · GSD** | clean `@opengsd/gsd-core` install (69 skills + hooks) | `pilot-gsd` | GSD's methodology + skills |
| **C · static-methodology** | a flat `CLAUDE.md` stating the *same* doc↔code discipline jigc enforces (no jigc, no validation) | `pilot-static` | the rule as **static instruction** |
| **D · plain** | bare Claude, clean container root, no `CLAUDE.md`, no skills, no memory | `pilot-deps` | nothing |

**Isolation (proven):** one container per arm = full filesystem isolation (defeats
the M17 global-`CLAUDE.md` leak by construction — a fresh container root has none).
Auth via a read-only mounted OAuth credential (verified working in-container). No
`/etc/claude-code/` managed policy present. Each run is a fresh `--rm` container
from the arm's image; runs cannot contaminate each other.

---

## 4. The two tasks (a pattern, not a single point)

Both engage the same differentiator (doc↔code consistency under a symbol rename),
with different documentation footprints so a *pattern* across them is the signal.

- **Task 1:** rename the public class **`CommentBlockParser` → `CommentTagParser`**.
  Footprint: the class declaration; the `createCommentBlockParser` factory; the
  `index.ts` re-export; internal usages (`UIDoc.ts`); tests
  (`comment-block-parser.test.ts`); README `## CommentBlockParser` section,
  prose, and code examples; **the arch-doc component anchor**.
- **Task 2:** rename the public class **`MarkdownDescriptionParser` → `MarkdownDocParser`**.
  Footprint: declaration; `createMarkdownDescriptionParser` factory; re-export;
  internal usage; tests; the README **cross-reference** inside the
  `CommentBlockParser` section ("…will use the `MarkdownDescriptionParser`…") and
  its code example; **the arch-doc component anchor**.

**Byte-identical prompt per task, issued to every arm** (no doc/test hints — a
plain refactor ticket, so the *method* determines completeness):

> Task 1: `Rename the public class ``CommentBlockParser`` to ``CommentTagParser`` throughout this project.`
>
> Task 2: `Rename the public class ``MarkdownDescriptionParser`` to ``MarkdownDocParser`` throughout this project.`

---

## 5. Matrix, models, order, settings

- **Matrix:** 4 arms × 2 models × 2 tasks = **16 runs**.
- **Models (pinned):** `claude-sonnet-4-6`, `claude-opus-4-8`.
- **Settings (identical):** `claude -p "<prompt>" --model <pinned> --output-format
  json --permission-mode bypassPermissions`. One fresh container per run.
- **Order:** runs are independent headless invocations (no cross-run memory), so
  ordering cannot induce carryover. Recorded fixed order: task1 then task2; within
  each, arms D, C, B, A; within each, sonnet then opus.

---

## 6. Acceptance — what "correct completion" requires

A run is **correct** iff, in its final committed/working state:
1. the symbol is renamed at its declaration, factory, re-export, and all code usages;
2. the tests reference the new name (and would compile);
3. **no documentation names a symbol absent from the code** — neither the
   architecture doc nor the README references the old name, and the arch-doc's
   `implemented-by` anchor resolves to a symbol that exists.

Criterion (3) is the differentiator-sensitive one.

---

## 7. The measured win (non-seeded, correctness-based)

The headline outcome per run: **does the final state contain a doc↔code drift?**
— i.e. does any committed document name the renamed (now-absent) symbol, or does
the arch-doc anchor dangle? Measured by (a) grepping the final docs for the old
symbol name, and (b) on the jigc arm, `jigc validate` (captured per run).

A **pilot-passing win** = ≥1 **non-seeded** run where an arm with the
differentiator (A) ends consistent while an arm without it (B/C/D) **ships a
doc↔code drift** on the same task+model — a real correctness difference, not
"jigc has a feature." A tie (all arms consistent, or all drift) is an honest null
and is reported as such.

---

## 8. Seeded instrument check (validates the apparatus, never the thesis)

Already demonstrated on the host and re-run as the recorded instrument check:
plant a dangling anchor (rename a documented symbol without updating the managed
arch-doc) → `jigc validate` / `task finalize` must **block** with
`doc-code.symbol-exists`. Proven: clean → "validates clean"; post-rename →
"blocking · doc-code.symbol-exists — anchor …#CommentBlockParser resolves to no
symbol". The seed validates the *instrument* and is excluded from the organic
outcome counts.

---

## 9. Blind judging

A cross-model judge (Codex, as M17 did) receives the **diffs + final docs for all
runs of one task, arm-labels stripped/coded**, and applies this rubric per run:

- **Correctness** of the rename (code + tests) — complete / incomplete.
- **Doc↔code consistency** — does any doc reference the old symbol? (the headline)
- **Ceremony cost** — turns/tool-friction spent (jigc/GSD overhead is *data*).
- **Collateral** — anything broken or out-of-scope.

The judge returns a per-run verdict; the analyst then unblinds and assembles the
comparison. The judge does not know which coded arm is jigc.

---

## 10. Honesty bounds (carried from M17 / measurement.md)

- **n=1 per (arm×model×task)** — a structured pilot, not statistics. The pattern
  across 2 tasks × 2 models is the signal.
- **The win must be a non-seeded correctness observation**, never feature-presence.
- **jigc's claim is narrow:** it polices the *managed* arch-doc's anchored symbols
  only — not README prose, not cross-references in another component's description.
  A README-only drift counts against whichever arm ships it, jigc included.
- **doc↔code is the only differentiator engaged** — not validation-in-general,
  not drift/reconciliation, not supersession.
- **The judge is cross-model but the diffs are self-identifying in places** (e.g.
  a `.jigc/` dir, GSD artifacts); arm-coding mitigates, the bound is named.
- **GSD activation:** the prompt is bare; GSD's skills/hooks are available but the
  agent may not invoke a GSD workflow on a bare ticket — that is GSD's ergonomics,
  legitimately measured, not a rigged handicap.

## 11. What this study does NOT test

Greenfield; existing-doc adoption/migration; the other differentiators; any
productive-corpus commitment (this is reversible twins only); statistical
generalization.
