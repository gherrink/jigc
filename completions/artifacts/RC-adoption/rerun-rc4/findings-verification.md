# Findings verification — migration-half rerun on rc.4 (2026-07-11)

Every finding from the rerun's feedback ([trial-record.md](trial-record.md) → Feedback) plus the log-discovered findings, verified adversarially against the rc.4 code (`main` @ 976f3fc) by three independent read-only verifiers, with live repros in throwaway repos (the trial repo untouched). Verdicts: **CONFIRMED-BUG** (defect) / **CONFIRMED-GAP** (real, by omission or design hole) / **PARTLY-WRONG** (claim inaccurate in part; residual finding noted) / **MISUSE** (tool behaved as designed).

This file is analysis, not disposition — routing (fix wave vs park vs decline) is the triage session's output.

## Summary table

| # | Finding | Verdict | Severity |
|---|---------|---------|----------|
| V1 | Author templates demonstrate double-quoted YAML → silent newline folding | CONFIRMED-BUG | **High** |
| V2 | `start --format json` = `{"text": prose}`, no structured `task_id` | CONFIRMED-GAP | **High** (driver contract) |
| V3 | `doc schema` omits enum members | CONFIRMED-GAP | Medium |
| V4 | `.vue` anchors symbol-blind (log-discovered, 209 advisory emissions) | CONFIRMED-GAP | Medium |
| V5 | No `--unset`; some optional fields have no clearing path | PARTLY-WRONG / CONFIRMED-GAP | Medium |
| V6 | Author-payload field placement "no inferable rule" | PARTLY-WRONG / CONFIRMED-GAP | Medium |
| V7 | Slug truncation cuts mid-word (`contrac`) | CONFIRMED-BUG | Medium |
| V8 | `validate` routes to a command that needs an active task, without saying so | CONFIRMED-BUG | Low-medium |
| V9 | `ingest` report: 219 identical lines, no grouping | CONFIRMED-GAP | Low-medium |
| V10 | arch-doc `cites` → ADR hard ordering undocumented in the authoring workflow | CONFIRMED-GAP | Low |
| V11 | Repeatable payload key `items:` "undocumented" | PARTLY-WRONG | Low |
| V12 | `rename` bare-slug error gives no hint; clap help contradicts itself | CONFIRMED-GAP | Low (papercut) |
| V13 | `store.unknown-type` routes to nonexistent `jigc doc types` (verifier-discovered) | CONFIRMED-BUG | Low (dangling route) |
| V14 | Mid-run `doc show --format json` exit 1 (log-discovered) | MISUSE | None |
| V15 | Advisory noise floor: no per-instance acknowledge; permanent unactionable advisories re-emit every sweep | CONFIRMED-GAP | Medium |

## V1 — double-quoted YAML scalars in author templates silently fold newlines — CONFIRMED-BUG, HIGH

Reproduced end-to-end on rc.4: a `research` doc authored per the template with multi-line prose + bullets inside `"<<…>>"` commits clean with bullets merged into one line — standard YAML flow-scalar folding at the payload parse (`crates/cli/src/author.rs:245`, `serde_yaml_ng::from_str`). No guard exists or can exist at parse time (a legal string arrives). The block-scalar form (`findings: |`) is verified fold-safe end-to-end — `classify()`/`is_slot_wrapped()` (`author.rs:104-120`) trim whitespace, so the fix is purely template-side.

**Every affected demonstration surface** (all show slot prose as a double-quoted flow scalar; none use a block scalar): `packs/methodology/steps/author-migration-research.yaml:20,23,26` · `author-migration-vision.yaml:18,21,24` · `author-migration-idea.yaml:20` · `author-migration-decisions-log.yaml:22` · `author-migration-deferral-ledger.yaml:24` · `author-migration-roadmap.yaml:20,21` · `crates/cli/pack/steps/author-migration.yaml:26,66` · `author-migration-spec.yaml:16,19,24,27` · `author-migration-prd.yaml:16,21,24,27` · `author-migration-adr.yaml:19,22,25` · `author-migration-arch-doc.yaml:22,27,31` · **and `crates/cli/src/doc.rs:136` — the `doc author --help` text itself**. (`author-migration-completion-record.yaml` quotes only fields, no slots — stylistic exposure only. The M40 fillable skeleton is the on-disk Markdown mint, not a payload emitter — unaffected. `doc set-slot --from-file` is inherently fold-safe, raw bytes.)

Severity rationale: silent content corruption that commits clean defeats the tool's own anti-silent-falsehood thesis. Fix sketch: switch all 12 templates + the help example to `key: |` block scalars; optionally one post-parse advisory (slot prose >N chars with sentence/bullet markers but no newline).

## V2 — `start --format json` is prose-in-a-box; no structured task id — CONFIRMED-GAP, HIGH

`ComposedWorkflow` has exactly one field, `text` (`crates/engine/src/compose.rs:1988-1991`); `render::composed` (`crates/cli/src/render.rs:140-153`) serdes it, so `jigc start … --format json` emits literally `{"text": "<composed prose>"}`. The minted id is in hand in `start.rs` at compose time but never surfaced structurally — it exists only inside pack prose, immediately followed by a closing backtick, so the reported `--task (\S+)` scrape capturing the backtick reproduced exactly. Survey: only `doc show` and `doc schema` are pinned JSON contracts; `validate`/`task list`/`describe`/ingest/write-acks are structured-but-unpinned serde projections; composed output and `milestone` summaries are explicit `{"text": …}` wrappers; the migration review gate is the *only* JSON output emitting `"task": <id>`. The only post-hoc structured route to a task id is `task list` (ambiguous under concurrent tasks).

Severity rationale: the task id is the handle every subsequent call requires; the feedback's broader "make `--format json` a real contract (task id, doc slug, item id, findings as data)" lands on a surface that is already declared (two pinned contracts exist) with a known hole beside them. Fix sketch: return the minted id alongside the view and render `{"task": "<id>"|null, "text": …}` (composed JSON is unpinned today — pin it while adding).

## V3 — enum members not discoverable via `doc schema` — CONFIRMED-GAP, MEDIUM

The members exist in the loaded model (`Field.of`, `crates/engine/src/schema.rs:257-259`); the M40 projection drops them — `ContractField` (`crates/cli/src/doc.rs:1534-1546`) has no `of`, `contract_field` (`doc.rs:1630-1641`) never reads it; the text listing inherits the omission. Reproduced: `kind: enum *` with no members, in both formats; `describe` doesn't expose them either. The write-path rejection *does* disclose (`crates/engine/src/write.rs:4673-4686`: "allowed: D, I") — so discovery costs one failed write per enum field, exactly as the feedback described. Nine enum fields across both packs are affected. Members live at `packs/methodology/schemas/deferral-ledger.yaml:36` (`D`/`I`; their semantics only in a YAML comment invisible to every read surface), `crates/cli/pack/schemas/adr.yaml:19` (`proposed/accepted/superseded`), `packs/methodology/schemas/completion-record.yaml:30,37-38`.

Fix sketch: add optional `of` to `ContractField` + `enum [D|I]` in the text listing — **note the pinned-contract implication**: additive key ⇒ explicit call (contract-version bump vs declare additive-tolerant) + golden update. Renaming `D`/`I` to self-documenting members is a schema-shape change to a manifest-frozen methodology doctype (schema-version 2 + corpus migration) — a design decision, not part of this fix.

## V4 — `.vue` anchors are symbol-blind in the code-anchor gate — CONFIRMED-GAP, MEDIUM (log-discovered)

The log's 209 `doc-code.unsupported-language` emissions are **9 distinct `.vue` anchors** (arch-doc `implemented-by` into Vue SFCs, e.g. `apps/dashboard/resources/js/Layouts/AppLayout.vue#AppLayout`) re-emitted per validate/finalize sweep. The grammar table (`crates/cli/probes/doc-code/src/resolve.rs:188-200`) covers rs/ts/tsx/js/py/php/sh/css/yaml — **no `.vue`**. Degradation semantics (`crates/cli/probes/doc-code/src/main.rs:276-334`): the *file* half still blocks (`dangling_file` fires before grammar dispatch), but on a present file with no grammar the `#symbol` is entirely unchecked — one advisory, `route: None`, non-blocking. **A doc CAN commit `AppLayout.vue#DoesNotExist` clean.** This is the deliberate M27 F2 design (surface as uncheckable rather than hide or wrongly block), but it means the flagship "a doc cannot claim code that doesn't exist" property holds only at file granularity for Vue-first codebases — this trial's stack exactly. Related: `criterion-maps-to-test`'s is-a-test half is Rust-only (`main.rs:322-330`).

Fix sketch: a Vue grammar arm — cheapest: extract `<script>`/`<script setup>` and parse under the existing TS/JS grammar (+ the SFC's implicit component name from the filename); independently, dedupe repeated unsupported-language advisories per file per sweep, and give the advisory a route.

## V5 — no `--unset`; empty-value claim wrong; some fields have no clearing path — PARTLY-WRONG / CONFIRMED-GAP, MEDIUM

**The "empty write was accepted" claim is wrong:** `check_opaque_scalar` (`crates/engine/src/write.rs:4748-4759`) rejects empty for every scalar family — reproduced: `blocking · write.malformed-value`, exit 1, old value untouched; the trial log's one empty-value call exited 1. The write never landed, which is *why* finalize still saw the old value. **The gap stands:** no `--unset` exists anywhere; the reject's route ("retry with a conforming value") is a dead end when the intent is *no value*. Clearing paths by field type: list-cardinality refs (`supersedes`, `cites`) clear via `--value "[]"` (verified, `write.rs:4782-4786`); item-level scalars clear only via the destructive remove-item→add-item→re-author sequence (exactly what the trial log shows); **section-level optional scalars (e.g. adr `cites-code`) have no clearing path at all**. Single-valued code-anchors are replace-or-nothing — "remove one from the list" doesn't apply (`check_code_anchor`, `write.rs:4881-4898`).

Fix sketch: `--unset` on `set-field` (reject on author-required/`set:`-stamped fields; splice-remove otherwise) + point the empty-value reject's route at it.

## V6 — author-payload field placement — PARTLY-WRONG on "no rule"; CONFIRMED-GAP on discoverability, MEDIUM

**A uniform rule exists:** the payload mirrors the schema's declared section structure — a field goes under `set:` of the section that declares it (`author.rs:243-291`). adr's `status`/`date`/`supersedes`/`cites-code` all live in its header section whose id happens to be `status`; research's `date` in `meta`; arch-doc's `cites` in `meta`. Sub-claims wrong in detail: adr's `date` *is* documented in the migration author template; `cites-code` is deliberately excluded there ("a separate cross-code pass"). **The discoverability gap is real:** `jigc doc schema` — both formats — flattens all simple-section fields and drops the field→section mapping (`doc.rs:1520-1526`), so the section id needed by the payload is doctype-specific trivia available only in the raw pack YAML (adapter-forbidden) or migration step prompts. (`set-field` is unaffected — the single-hop `#<field>` form searches all sections, `doc.rs:2452-2463`.)

Fix sketch: carry `section` per field (or group `fields` by section) in the `doc schema` projection — same pinned-contract call as V3.

## V7 — slug truncation cuts mid-word — CONFIRMED-BUG, MEDIUM

The M39 word cap is innocent: `MAX_WORDS = 5` always cuts on a `-` boundary (`crates/engine/src/slug.rs:38,169`). The mid-word cut is the `MAX_CHARS = 50` backstop (`slug.rs:48,182-187`), which truncates at byte 50 regardless of boundaries and only trims a trailing `-`. Its own doc comment claims normal 5-word slugs sit far below it — false: the trial's original slug (one long compound word) exceeded the length cap, so the backstop truncated it mid-word (`cms-integration-contrac`). The `-and-the` / `the-…-cannot` endings and the `ls`-unpredictability complaint (TODO's separate bullet) are the word cap working as designed — no stopword logic exists anywhere in `slugify` — a design/usability gap, not a defect. Severity rationale: slugs are frozen identity (filenames, ref targets); ugly cuts are permanent absent an atomic `jigc rename`.

Fix sketch: in `cap_chars`, trim back to the last complete word (`rfind('-')`); separately (design call) skip trailing stopwords when the word cap lands on one.

## V8 — `validate` routes to a command that can't run task-less — CONFIRMED-BUG, LOW-MEDIUM

The `doc-code.title-names-symbol` advisory's route (`crates/engine/src/target_surface.rs:421-427`) is `jigc doc retitle-item <addr> --title "<new title>"` — no `--task`, no mention one is required; `run_retitle_item` (`crates/cli/src/doc.rs:882`) resolves the active task first and bails ("no active task — start one with `jigc start`", `doc.rs:2154`). `validate` is a store-scope, task-less surface, so the route is *guaranteed* to be issued in a state where it fails. Observed live in the trial log: the task-less attempt at 06:30:55, the minting + `--task` retry 12 s later. (The same route text on the write-time id-from guard at `doc.rs:447` fires inside a task — benign.)

Fix sketch: extend the route string to include the minting step.

## V9 — `ingest` report ungrouped — CONFIRMED-GAP, LOW-MEDIUM

`render::ingest` (`crates/cli/src/render.rs:789-850`) renders one line per row, no aggregation of any kind; `unmanaged` rows (`ingest.rs:224`) are identical per file. 220 candidates ⇒ 32 KB with the single actionable row buried — the report matches the code exactly. First-contact adoption surface, agent-context-hostile (the product's own thesis).

Fix sketch: text renderer only — collapse `unmanaged` rows into per-directory counts, keep adoptable/needs-reconcile/adopted itemized; JSON stays full-rows (tooling contract).

## V10 — arch-doc `cites` ordering dependency undocumented — CONFIRMED-GAP, LOW

The ordering is genuinely hard: `architecture-documentation`'s `allows-create` is arch-doc only — an ADR cannot be minted in the same task, so a `cites` to a nonexistent ADR can never resolve in-task and the finalize ref-gate blocks. `author-arch-doc.yaml` instructs authoring `cites` but never states the target must already be committed. **The M40 migration sibling has exactly this warning** (`author-migration-arch-doc.yaml:62-69`: "ONLY when that adr is already in the committed store… a dangling `cites` blocks finalize forever") — the ordinary authoring path was left without it.

Fix sketch: one line in `author-arch-doc.yaml` after line 14, mirroring the migration sibling's wording.

## V11 — repeatable payload key `items:` "undocumented" — PARTLY-WRONG, LOW

`items:` is demonstrated in every shipped migration author template with a repeatable (spec/prd/changelog/roadmap/decisions-log/deferral-ledger/completion-record), and the praised error comes free from serde `deny_unknown_fields` (`author.rs:73-80`). **Residual gap:** no *reference* surface documents the payload grammar outside per-workflow step prompts — `doc author --help` explains only the `<<…>>` marker, and `doc schema` says `repeatable` without naming the payload key, so an agent composing a payload from `doc schema` (this trial's likely path) has to guess.

Fix sketch: a payload-shape example (`sections:/set:/items:`) in the `doc author` long help.

## V12 — `rename` bare-slug error gives no hint — CONFIRMED-GAP (papercut), LOW

`parse_addr` (`crates/cli/src/rename.rs:534-543`) emits exactly `` `<addr>` is not a `<type>:<slug>` address `` — correct form, no example, no doctype list, no did-you-mean. Aggravating: the clap surface invites the mistake — the positional is `<OLD_SLUG>` and the command's own top help line says `jigc rename <old-slug> --to …` (`crates/cli/src/cli.rs:199-207`), contradicting the arg description two lines below.

Fix sketch: error → `` expected `<type>:<slug>` — e.g. `adr:<addr>`; run `jigc describe` for doctypes ``; rename the clap value_name/help to the address form.

## V13 — `store.unknown-type` routes to a nonexistent subcommand — CONFIRMED-BUG, LOW (verifier-discovered)

Found while refuting V12: the `store.unknown-type` route text (`crates/engine/src/store.rs:121`) says "list the available doctypes with `jigc doc types`" — **no such subcommand exists** (verified: unrecognized subcommand on rc.4). The correct pointer is `jigc describe` (which the rename not-found route already uses). A dangling route on a shipped error path — the same defect class as V8.

## V14 — mid-run `doc show --format json` exit 1 — MISUSE, no fix needed

Does not reproduce; the log is definitive: `store.not-found` at 06:30:23 on a doc `doc create`d in-task at 06:29:40 and first finalized at 06:30:40. `doc show` reads only the committed store by design (`doc.rs:163-165`); the rc.4 error even carries the exact route (`store.rs:170-172`, the M40 hardening): "a doc staged in an open task is not committed yet — read it with `jigc task diff <id>`". The post-finalize retry exited 0. Working as designed, correctly routed.

## V15 — advisory noise floor: no acknowledge path, permanent floors re-emit forever — CONFIRMED-GAP, MEDIUM

Named by the migrating agent in its fold-back priorities ("fix the advisory noise floor, or the gates die of neglect"); verified against the log and the code. **Log evidence:** 597 finding emissions across 105 sweep invocations during the run. Most were *transient* migration-state advisories that drained as the corpus was adopted/baselined (`file-state.*` 297) or were actually fixed by the agent (`title-names-symbol` 45 — cleared by retitles; `inverse-cardinality` 47). **But the steady-state floor measured today is 9 findings — all `doc-code.unsupported-language` (V4), route-less and unactionable by any agent** (no Vue grammar exists to satisfy them), re-emitted identically on every future `validate` and every finalize sweep, forever. **Code:** the mechanisms that exist don't cover this — the M40 severity knob is *per-check* via the cascade (`crates/cli/pack/config/knobs.yaml` — silencing `unsupported-language` for the 9 known anchors would silence the check for every future anchor too), and the exempt tokens (`crates/engine/src/validate.rs:1351,3084`) are per-section for specific checks. There is **no per-instance acknowledge** ("seen, accepted, stop re-emitting until it changes").

Severity rationale: a permanent unactionable advisory floor trains agents to skim past advisory output — which is how the *actionable* advisory drowns. The gate-credibility property ("the gates fired four times and were right every time") depends on near-zero noise. Fix sketch: a route for every advisory as the floor rule (V4's arm covers these 9), plus a design call on per-instance acknowledgement (e.g. an acknowledged-findings ledger keyed by finding code + stable target, invalidated when the target's hash changes) — the latter is a real design fork (silencing vs honesty), triage material, not a papercut.

## Triage raw material (not dispositions)

**The migrating agent's own fold-back priorities** (verbatim, post-run): *"fix the folding-YAML templates (silent data corruption), fix doc author's payload discoverability (it's the keystone verb and it's the one that breaks), and fix the advisory noise floor (or the gates die of neglect). Everything else on my list is a papercut by comparison."* Mapped to the findings: priority 1 = **V1**; priority 2 = **V3+V6+V11 as one problem** (the keystone-verb framing — `doc author` is the batch write path, and every one of its breaks traces to the payload shape being undiscoverable from the declared read surfaces); priority 3 = **V15** (with V9 and V4's dedupe arm as members).

Findings plausibly hitting the known-hole / one-way-door lens that has governed rc-wave chartering: **V1** (silent content corruption committing clean — contradicts the tool's own anti-silent-falsehood thesis), **V2** (the JSON write-path contract — a declared surface with a known hole beside the two pinned read contracts, and the explicit at-scale/driver ask), **V4** (the flagship anchor property symbol-blind on a mainstream stack), **V15** (gate credibility is a declared property; a permanent unactionable floor erodes it). V3+V6 share one mechanism (the `doc schema` projection drops model data it already has) and one pinned-contract versioning call. V8+V13 share one defect class (routes that don't survive being followed). Disposition belongs to the triage session with the human.
