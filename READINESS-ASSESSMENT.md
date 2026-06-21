# jigc — format-stability readiness assessment

**Question:** are jigc's contract surfaces (CLI vocabulary, directory/storage,
doctype schemas, workflow dialect, config/migration machinery) stable enough to
point at a **productive project** — where, post-commitment, every format change
becomes a migration the adopter must absorb?

**Method:** assessed 2026-06-21 at HEAD `5f753dd` by five independent read-only
recon agents (git-history churn + current shape + open instabilities + live binary
exercise), then the consolidated verdict was **cross-model-validated by Codex
(gpt-5.5)**, which re-checked every load-bearing claim against the real files/git
history. Codex confirmed the verdict sound — *"if anything the repo evidence makes
the recommendation stronger"* — and sharpened three overstatements, folded in below.

## Bottom line

**Not yet.** Two independent facts converge, and they compound: the doctype
**schemas are still changing**, and there is **no automated migration path for a
managed-document corpus** across a schema/dialect/layout change. The surface most
likely to change is the one with no migration tooling — so locking a productive
corpus now means hand-migrating committed instances, unaided, on the next breaking
change. Both model families (the five-agent synthesis and Codex) reached this
independently.

The damage mode is **migration labor, not silent breakage** — drift is detected
and *blocked*, never silently corrupted. And the value question that would justify
paying any lock-in cost (does jigc beat a static `CLAUDE.md` when the
differentiators are engaged?) is **not cleared** — the reversible value-gate pilot
ran (2026-06-21, doc↔code engaged) and jigc **did not beat static**: it tied the
unguided baselines (plain, GSD) and lost to a static-methodology `CLAUDE.md`
(4/4 clean) — even when that rule was diluted into a realistic 157-line file. Not a
clean refutation (one small task; jigc's case narrows to the *long-horizon
many-edit* regime), but the gate is **unproven, leaning negative**. Full result:
[completions/artifacts/differentiator-pilot-study1/VERDICT.md](completions/artifacts/differentiator-pilot-study1/VERDICT.md).

## Per-surface verdict

| Surface | Verdict | The load-bearing evidence (Codex-corrected) |
|---|---|---|
| **CLI vocabulary** | STILL-MOVING | Single-task core (`start`/`doc`/`task`/`config`) stable + lock-ready. But the binary name **`jigc` is an explicit placeholder** (VISION.md:3, CLAUDE.md:7) — a rename breaks 100% of the typed surface, the `jigc *` allowlist, the adapter bootstrap, all 16 catalog refs at once. The `milestone` group is the youngest tier — gained `provision` in M31. A `jigc adapter` verb group is planned/unbuilt. |
| **Storage byte-format** | STRONG REGRESSION PROTECTION (not uniformly fuzzed) | `commit`/`adr` are genuinely **proptest-fuzzed** over arbitrary conformant docs (the "#1-technical-risk" suite, `write.rs:5542`); EOL/BOM/trailing-newline covered. **But** `prd`/`spec`/`changelog`/`arch-doc` are **fixture/golden-locked, not fuzzed** over arbitrary instances, and a byte-instability was fixed as recently as **M26** (empty-slot + field-group render). So "diffs won't churn from formatting" holds with high confidence for the fuzzed types, lower for the newer ones — *regression-protected*, not *uniformly lock-ready*. |
| **Directory layout** | STILL-MOVING | The default **`docs-root → docs/` landed 2026-06-19 (2 days ago)** — mitigable by pinning `docs-root` explicitly so the committed layout is adopter-owned, not default-tracked. |
| **Doctype schemas** | STILL-MOVING (the primary blocker) | Three core schemas changed in the last 4 days (`348022f` commit, `9cbe924` adr, `e612025` prd) — **but only one is corpus-migration-forcing**: `prd.requirements` fixed-slot→**repeatable** (`e612025`, Jun 17) breaks every existing managed PRD. `commit` is **transient** (no `location:` — never a persisted corpus doc); `adr.supersedes` 0..1→0..\* is a **backward-compatible widening** that was not enforced, so existing ADRs need no migration. The honest framing: *schema churn is active, at least one recent persisted core shape is clearly breaking, and more format-engine opens remain queued* (in-prose mentions, repeatable-item edges, prd→spec). |
| **Workflow dialect** | STILL-MOVING (additive *in practice*) | Grammar shape stable; governed as "add kinds as earned" — open extension triggers + queued address/front-matter additions. Changes have been additive **so far**, but additive-only is **not a hard invariant**: a future validation-tightening (e.g. the queued required-read-role finalize gate) could make a previously-valid authored workflow invalid or stricter. The spec was incomplete until today (M32 wrote the `selectable` contract). No external party authors against it yet (not a public platform). |
| **Config cascade + migration machinery** | HAS-GAPS (the critical one) | **Config/pack churn is well-bounded**: `jigc upgrade` reconciles clean/conflict/orphaned/needs-rebasing, binary-proven (v1→v2 e2e 3/3); drift is detected + routed, never silently corrupted. **But there is NO automated managed-corpus migration path** — `schema_version` stamps exist only on result/probe/finalize JSON (`result.rs:17`), **not on managed Markdown docs**; `jigc upgrade` = pack-definition deltas only; `jigc migrate` = foreign-doc *adoption* only. A breaking doctype/dialect/layout change is *detected and blocked* but the repair is **manual** (hand-edit each file) or a lossy per-file re-migrate. The project's own "v1 in-place corpus migration" is **explicitly deferred + unbuilt** (DECISIONS.md:4714, decisions-pending.md:65) — jigc dodges it by keeping its own docs as plain markdown. |

## What is already safe

- **Drift safety** — out-of-band edits are detected and routed; a non-conformant corpus is *blocked*, never silently corrupted.
- **Config/pack customization** — upgrade-reconciled across a pack version bump.
- **The `commit`/`adr` byte-format** — fuzz-proven; their diffs won't churn from formatting.

So the failure mode of going productive too early is *expensive manual migration on the next schema change*, not *data loss*.

## The de-risking path (productive signal now, without the lock-in) — DONE

The M17 protocol already does this by design: test on a **twin** of a real project
+ a fitting domain pack + differentiator-engaging tasks, with records **exported as
plain files** — the productive project and jigc's own repo stay unmanaged. This
yields the real signal (does jigc beat a static `CLAUDE.md` when the differentiators
are engaged — the M17-relocated hypothesis) **reversibly**.

**Executed 2026-06-21** ([study 1](completions/artifacts/differentiator-pilot-study1/)):
a 4-arm (jigc / GSD / static-methodology / plain) × 2-model × 2-task matrix on
isolated container twins of `gherrink-ui-doc @ 542b3206`, doc↔code engaged via the
shipped `arch-doc` doctype, blind-judged. **Result: jigc did not beat static** —
jigc 2/4 clean, tied plain (2/4) and GSD (dormant, 2/4), and **lost to the
static-methodology arm (4/4)**, which held even when the rule was diluted into a
realistic 157-line `CLAUDE.md` (arm E, 4/4). Three diagnosed jigc failure modes:
the weaker model **bypassed the adapter**; the anchor check has a **prose blind
spot**; on bypass jigc adds only **friction** (blocks a correct out-of-band edit).
Reversible as designed — zero corpus lock-in.

## Recommendation — the gates before the irreversible step

Do **not** commit a productive corpus (or self-host this repo) yet. Three gates:

- **G1 — schema/format freeze, proven not asserted** → milestone **M33** (schema/
  format stabilization). Triage the deferred schema-engine opens (build the
  productive-necessary ones, consciously defer the rest behind G2), declare the
  doctype set + schema format **v1-complete**, then hold a defined run of milestones
  with zero schema-shape commits. Convergence is evidence, not a promise.
- **G2 — the corpus-migration path exists** → milestone **M34** (managed-corpus
  migration). A per-doc schema-version stamp + an `upgrade`-class transform that
  takes a v1 managed corpus to a v2 schema (the explicitly-deferred "v1 in-place
  migration"). Until this is built, every schema change is hand-priced.
- **Value gate** (M17 named it; **study 1 ran 2026-06-21 — not cleared**) — a
  differentiator-engaging pilot showing jigc **>** static, not **≈** static. Study 1
  found jigc **≈** plain/GSD **<** static, so the gate is **unproven, leaning
  negative**. It is not refuted (one small task; dilution didn't hurt static even at
  realistic file size), but jigc's case has **narrowed to the long-horizon many-edit
  regime** — the one test that could still flip it. That is the **owed next study**
  before this gate can be called either way. Two fixable jigc weaknesses surfaced
  (make the warn-only pre-commit hook *blocking*; close the doc prose blind spot) —
  see [LESSONS.md](completions/artifacts/differentiator-pilot-study1/LESSONS.md).

**Net:** the twin pilot is **done and reversible** — it spent no lock-in and
returned a *negative-leaning* value signal. Point a *productive corpus* at jigc only
after **M33 + M34 + a value gate that actually clears** (the long-horizon study, plus
the two product fixes, are the path to clearing it — or to honestly concluding a
well-maintained static file suffices). The placeholder **name** is a separate,
deferrable cleanup (a 100%-surface rename) — settle it before any adopter builds
muscle memory, but it does not gate the pilots.
