# M40 completion verdict — the rc.4 wave (pre-1.0 adoption-findings hardening)

**Verdict: ✅ SHIPPED** (2026-07-11). Built (8 increments), independently validated per increment, milestone-audited (code-review + e2e through the real binary), all confirmed findings fixed, re-verified green. The adoption-trial findings wave gating the rc.4 → implementation-half-trial → 1.0.0 sequence.

## What shipped (8 increments, base `a8da62a` → HEAD `8562715`; 44 commits + 4 post-audit fixes)

1. **F7 — finalize transaction hardening**: retirement pathspec discriminated on the **index** (`path_in_index` via `git ls-files`), not HEAD; `stage_migration` returns its staged set; **two-axis scoped rollback** (worktree keyed on the retire byte-capture set, index on the staged set) — a user's pre-staged `git rm` lands clean and survives hook-rejection rollback; stage-phase git failures are a **routed finding with verbatim stderr**, commit-phase hook rejections stay raw (the recorded hook decision honored).
2. **F2+F5 — the item-identity surface**: `jigc doc retitle-item` (anchor frozen — the declared retitle-without-reslug invariant's verb; **enum id-from refused** as an identity change, remove+add route); the **set-field id-from guard** (type-aware routes) kills the or-insert corruption class; `doc-code.title-names-symbol` demoted to **advisory** via its newly-minted cascade knob, route names the real verb.
3. **F8+F4+F3 — ingest & store visibility**: candidate set = `git ls-files --cached --others --exclude-standard -- '*.md'` (prune kept, re-sorted, no dead fallback); `schema-conformance.repeatable-populated` + `surplus-sections-absent` advisories (adopt-time triage annotation row-carried in every format + store sweep; the string exempt-knob with 3 pack-default exemptions); the **two-tier orphan advisory** (`file-state.unregistered-doc` routes migrate-or-ignore, conditioned on the workflow existing — the unmanage dead-end killed).
4. **F1 — the schema-visibility pair**: `jigc doc schema <doctype>` as a **separately-pinned contract-version-1 projection**; the generalized fillable skeleton via the shared `pub is_author_required` predicate (idea mints `trigger:`; dogfood-record's 14 named; adr mint-prefix untouched — the tripwire held).
5. **A1 — methodology schema versioning (the full cut)**: the ~9-arm byte-stability census (which surfaced + fixed a real CRLF front-matter parse defect) → per-origin manifest-resolution unification (the split-brain closed) → the ten-schema methodology manifest (frozen v1; snapshots absent at v1 by design) → stamp injection + the v0→stamped `migrate-corpus` arm. Dev-pack frozen hashes untouched; a methodology shape edit now blocks loudly at compose.
6. **A2 — nested content joins the pinned read contract**: recursive item objects keyed by block id; the write-grammar address resolves on show; the false `no-such-item` and the exit-0 wrong-node degrade both dead; the plain slice carries fields + nested content (one grammar shared verbatim with the writer — `pub(crate) physical_item_chain`).
7. **F6+F10 — the migration surface**: the fidelity kept-set scans the whole rewrite (false "dropped 2.0" dead; trailing-dot under-report fixed); **seven methodology migrate workflows** (vision/roadmap/decisions-log/deferral-ledger/idea/research/completion-record) with decision-tree templates (TRANSCRIBE/OMIT dates, `[D,I]` mapping, the completion-record **artifact ladder** with the refusal rung — M17's floored gate untouched; Framing A intact); migratable set = 12; F9/F12 help cross-pointers + de-enumeration.
8. **A3 + A4 + acceptance + fold-back**: invocation-log `binary_version`; the adr implement step enumerates all four slots; the placement-reslug real-rule message; the milestone-record reslug refuse-always guard; `flow41_acceptance` (the six-arm done-picture suite); the doctype-authoring ⚠ re-verify (zero markers remain); reading-order fold-back + banner.

## Audit (independent, adversarial)

- **Increment validation** — all 8 validated clean at their increments (bounded fix rounds never exhausted).
- **Milestone code review — deliverable holds, no invariant violations** (engine pack-empty + LLM-free confirmed; every golden change traced to a principled cause). 3 LOW latent findings, none reachable on the shipped pack surface.
- **Milestone e2e — all done-picture arms passed** through the real binary (pre-staged `git rm` × 3 shapes · gitignored ingest + hollow/surplus annotations, order-invariant byte-identical · two-tier orphan · retitle round-trip with inbound addresses surviving · both id-from guards · `doc schema` determinism · the v0 corpus stamp migration byte-identical across seed orders · the compose-time freeze block · nested reads at every depth with honest misses · fidelity positive + negative controls · root `VISION.md` migration · help fold-ins · `binary_version`). **One genuine finding** (pre-existing, reproduced on the rc.3 binary too): nested undeclared-field writes committed silently.

## Findings → fixes (4 post-audit commits, all test-first, serial, gate-green)

| # | Finding | Fix | Commit |
|---|---------|-----|--------|
| 1 | (e2e) An undeclared field on a **nested** item committed silently — root cause: the parser never scanned stray field groups on **field-less** item templates at any depth | `read_field_group` runs unconditionally in `parse_items`; blocks at validate/finalize; retroactively arms the store sweep for corpora already carrying stray bytes | `56178cf` |
| 2 | (review) `retitle-item` had no machine-maintained refusal — a milestone-record task heading could be retitled away from its work-unit id, and the set-field guard's route pointed there | `write.machine-maintained` refusal on milestone-record + the guard's route names the milestone verbs instead | `8725fbf` |
| 3 | (review, latent) control-char titles could splice a two-line heading (anchor stranded) on a schema with an undeclared id-from; the mint side never checked titles at all | unconditional control-char reject in the shared title guard (`reject_malformed_title`) across `add_item`/`add_nested_item`/`retitle_item`; the id-from loader assertion deliberately not shipped (unsettled model choice) | `f460a9c` |
| 4 | (review, latent) two nested repeatables per block would union both groups under every block id in the read surfaces — physically unrepresentable in the parse model | loader-level breadth guard (`SchemaError::MultipleNestedRepeatables`); depth stays general; the breadth bound documented in structural-grammar.md | `e244f34` |

## Re-verify (the acceptance gate)

Full unscoped gate at HEAD `e244f34`: `cargo fmt --check` clean · `cargo clippy --all-targets -- -D warnings` clean · `cargo build` clean · `cargo test` → **1676 passed, 0 failed** (178 target summary lines, incl. the `--bin jigc` goldens and doc-tests). Tree clean.

## Honest bounds (into the implementation-half trial)

- The methodology freeze crystallizes at v1 exactly as M33/M34 did for the dev pack — the first methodology v1→v2 bump will be the first live exercise of its migrate-corpus arm beyond the v0 stamp recovery.
- The completion-record artifact ladder's refusal rung is proven by test + template text; a live foreign corpus exercising every rung arrives with real adoption (the dashbard migration exercised rung 1 by hand pre-M40).
- `describe`-side discoverability of `doc schema` rides the adapter/help text; its usage profile is unmeasured until the implementation-half trial's log analysis.
