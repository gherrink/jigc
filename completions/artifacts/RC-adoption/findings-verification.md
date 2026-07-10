# RC adoption trial — findings verification (2026-07-10)

Independent read-only verification of the trial's 12 feedback findings against the rc.3 workspace (two agents; F7 and F4 reproduced live). Verdicts feed the triage table in [trial-record.md](trial-record.md); this file preserves the evidence + minimal fix shapes for rc.4 planning.

## F1 — create path hides the schema — PARTLY

- `run_create` (`crates/cli/src/doc.rs:941-980`) materializes only `set: on-create` / `default:` header fields (`on_create_doc_fields`, doc.rs:856-882); the render (`crates/engine/src/write.rs:133-145`) emits only fields handed to it. Finalize independently blocks on every author-required field (`is_author_required`, `crates/engine/src/validate.rs:1599-1620`). **Invisible-required = required minus seeded.**
- Confirmed for `idea` (`trigger` — `packs/methodology/schemas/idea.yaml:24`, no default/set, not optional); same class hits `completion-record` (`verdict`+`owner-artifact`) and `dogfood-record` (all 14 meta fields; skeleton mints an empty `---\n---`).
- **"Same for adr" REFUTED:** adr's only author-required header field is `status` with `default: proposed` (`crates/cli/pack/schemas/adr.yaml:19`), which is seeded (unit test doc.rs:2449-2466); required prose slots render as visible headings.
- No shape surface exists: `describe` projects only description/usage prose (`crates/engine/src/introspect.rs:95-152`); no `doc schema` verb in `DocCommand` (doc.rs:39-150). Migrate payload templates are hand-authored pack step prose (`crates/cli/pack/steps/author-migration-*.yaml`), not generated.
- **Fix shape:** generalize the commit doc's `fillable_form` precedent (`start.rs:225-243` — every header field pre-stamped as an empty `key:` line; `insert_front_matter_field` write.rs:2538 proves render/parse tolerate it) + an additive `jigc doc schema <doctype>` rendered deterministically from the resolved `Schema`. **Do not** seed empty required *values* into the skeleton — `check_field` (validate.rs:1538-1560) routes a present-but-empty required field into value checks where an empty string may pass, silently weakening the gate; and it changes golden-locked mint bytes.

## F2 — no repeatable-item retitle/reorder — CONFIRMED

- Verb surface = Create/AddItem/RemoveItem/SetField/SetSlot/Author/Show (doc.rs:39-150); `jigc rename` is doc-slug-only (`cli.rs:194-210`).
- `add_item` appends, no position parameter; its own comment says "`reorder` is a separate verb" (`crates/engine/src/write.rs:2250-2253`) — no reorder verb exists anywhere.
- Engine has `set_title` for the doc H1 only (write.rs:978); no item-heading retitle primitive; `set-field …/title` returns NotPresent (write.rs:1203-1205). The data model supports retitle-without-reslug (the `{#id}` anchor is separate from heading text — the declared CLAUDE.md invariant) but no verb exposes it.
- Aggravator: F5's blocking route commands "rename the component heading" (`crates/engine/src/target_surface.rs:415-419`) — an operation the verb surface cannot perform.
- **Fix shape:** `jigc doc retitle-item <addr> --title "…"` — splice heading text, keep the frozen `{#id}`; engine op mirroring `set_title` over `locate_item_path`. **Lens: known hole** (declared invariant with no verb + a shipping blocking route that commands it).

## F3 — three definitions of "managed" — CONFIRMED

1. **ingest classify** (`crates/engine/src/ingest.rs:127-157`): conformant parse + exact resolved location/placement match → else `Unmanaged`.
2. **validate orphan advisory** (`crates/cli/src/cli.rs:772-788` → `orphan.rs:146-172`): parent-dir basename == some doctype's `location:` basename (`.planning/research/` matches the `research` doctype) — no conformance check, no was-it-ever-registered check.
3. **unmanage** (`crates/cli/src/unmanage.rs:47-89` → `engine/src/ingest.rs:256-268`): registered = forward edges or a `FileStateRecord` baseline; never-adopted file → `dropped=false` → "not managed (nothing to drop)" (`render.rs:911`).

The advisory's routed remedy is a **guaranteed no-op for the entire false-positive class** (basename-coincidence files jigc never managed); it only works for the M36 scenario it was designed for (previously-adopted docs stranded by a docs-root re-point). **Fix shape:** gate the found-stranded arm on `FileStateRecord`/index membership — registered strand keeps the `unmanage` route; unregistered coincidence goes silent or routes to migrate.

## F4 — ingest conformance shallower than it sounds — CONFIRMED (reproduced; worse)

- Reproduced with the real engine + real roadmap schema: a GSD-ish `docs/roadmap.md` (headings match, zero `### {#id}` items) → `parse OK, items=Some(0), conformance findings=0, classify=Adoptable`. Adopted, baselined, `doc show` → `"milestones": []`.
- `parse_items` (`crates/engine/src/parse.rs:784-813`) scans only H3 headings — prose/bullets in a repeatable region are invisible. `check_repeatable` (`validate.rs:1288-1301`) iterates parsed items; zero items → zero findings. No minimum-cardinality concept exists (the MVP record's "advisory at store scope" was never built).
- **Worse:** surplus `##` headings silently tolerated — body sections map positionally to the first N H2s (parse.rs:256-283), extras unchecked → whole foreign sections adopted-but-invisible (the dashboard roadmap's `## Phases`/`## Progress`).
- Triage row (`crates/cli/src/ingest.rs:144-157`) carries type+verdict only — no item count.
- **Fix shape:** additive advisories — (1) adopt-time triage annotation "adopted — structurally empty: 0 milestones" (the parsed doc is in hand at `classify_row`); (2) the same zero-items advisory in the store-scope validate sweep; (3) a surplus-H2 advisory in the same motion. No new gate — preserves the MVP-era rationale (never a per-task gate).

## F5 — title-names-symbol taxes prose — CONFIRMED

- `crates/engine/src/target_surface.rs:378-421`, opted in by `title-names-symbol: true` (arch-doc.yaml:41). Trigger = item **has** `implemented-by` with a `#symbol` fragment ∧ a title token passes `is_compound_identifier` (lines 474-483: lower→upper hump `w[0]..w[1]`, or upper-upper-lower) ∧ no flagged token exactly equals the anchor's symbol (line 399).
- So **WordPress** (d→P hump) blocks; the audit-service, git-host and Redis names pass; PostgreSQL (e→S) would block. Anchorless items skip entirely (`implemented-by` is `FieldType::Pack`, never author-required — validate.rs:1609-1611) → the same title is legal or blocking depending on whether an anchor was supplied. The false-positive class is documented in-code as "acceptable and rare" (lines 388-389, 472-473) — miscalibrated for domains where brand names are the component vocabulary.
- **Fix shape (options):** (a) relax exact equality to case-folded containment either way (`wordpress` ⊂ `CmsGlobalClient` passes) — smallest; (b) demote to advisory; (c) a schema-level exempt-tokens knob. Whichever lands, the route must stop naming a nonexistent operation (F2).

## F6 — fidelity gate cried wolf — CONFIRMED

- The release-delta line renders for **every** migration task (`crates/cli/src/task.rs:814-836`, gated only on `is_migration`). `dropped_release_versions` (`crates/cli/src/render.rs:1200-1219` + `scan_version_tokens` :1225-1248): foreign side scans version-like tokens from the *entire* text (incl. the H1 `# Project-alpha 2.0`); kept side collects only from `### ` heading lines (the changelog release-item shape). A PRD rewrite has no `### <version>` headings → `2.0` reported dropped even though the rewrite's title still says it.
- **Fix shape:** kept-set scans the whole rewrite text (~1 line in `dropped_release_versions`); or gate the line to migrate-changelog tasks. (a) is smaller.

## F7 — pre-staged `git rm` kills finalize raw — CONFIRMED (reproduced)

- Retire itself is idempotent on an absent file (`task.rs:1472`). The failure is `stage_migration` (`task.rs:1501-1531`): the retirement pathspec is included whenever the path exists **at HEAD** (`path_at_head`, task.rs:2476-2485 via `git cat-file -e`); after a user `git rm` the path is gone from index+worktree but present at HEAD → `git add -- <path>` is fatal (`fatal: pathspec … did not match any files`, exit 128; reproduced).
- `git_run` bails with raw stderr (`task.rs:1998-2003`) → `render::operational_error` (task.rs:910-913), exit 1, no routed finding. **Bonus damage:** the failure triggers `rollback_promotions`, whose `git restore --staged --worktree` on each retirement (task.rs:1648) **resurrects the file the user deliberately removed**.
- **Fix shape:** discriminate on the **index**, not HEAD — include the retirement pathspec only when `git ls-files -- <spec>` is non-empty (pre-staged deletion rides the whole-index commit anyway); optionally wrap residual git failures in a routed finding. **Lens: known hole** in the detect-and-route invariant, hit by the most sympathetic OOB edit imaginable (the user's intent matched jigc's plan exactly).

## F8 — ingest ignores .gitignore — CONFIRMED

- `engine::ingest::discover_candidates` (`crates/engine/src/ingest.rs:54-89`): hand-rolled recursive `read_dir` walk over every `*.md`, pruning only `.jigc`/`.git` (`INTERNALS_DIRS`, ingest.rs:36). No ignore/walkdir crate, no `git ls-files`, no .gitignore consultation. Every candidate classified against every schema (`crates/cli/src/ingest.rs:100-124`), every row in the report → 4,627 candidates / 630 KB on a node project (91% of the trial's total logged output).
- **Fix shape:** CLI computes candidates via `git ls-files --cached --others --exclude-standard -- '*.md'` and feeds the list to classification (git shell-outs belong in the CLI, not the engine); raw walk stays as the no-git fallback.

## F9 — ingest vs migrate routing — PARTLY

- `ingest --help` (`cli.rs:156-159`) and `migrate --help` (`cli.rs:161-174`) each state their own semantics, neither cross-points (ironically `migrate-corpus`'s help does, cli.rs:182). AGENT.md bootstrap mentions neither verb (`adapter.rs:1032, 1042, 1377-1381`). The one cross-pointer is post-hoc: the ingest report's verdict legend routes `unmanaged` rows to migrate (`render.rs:884-886`).
- **Fix shape:** one cross-pointer sentence in each clap doc-comment; optionally migrate in the `needs-reconcile` legend gloss (render.rs:880-882).

## F10 — migration coverage gaps — CONFIRMED (extensible)

- Shipped: `migrate-{adr,spec,prd,arch-doc,changelog}` (`crates/cli/pack/workflows/`); `commit` deliberately excluded (transient — `migrate.rs:73-74`). Methodology pack ships **zero** migrate workflows → no foreign-migration path for roadmap/deferral-ledger/decisions-log/completion-record/dogfood-record/vision/research/idea/milestone-record — exactly the GSD-relevant set.
- **Not hardcoded:** the migratable set is derived at runtime from the presence of a `migrate-<doctype>` workflow in the composed pack (`migrate.rs:64-91`, hard error otherwise :94-107). Closing a gap = one workflow YAML + author step; zero engine/CLI code.
- **Fix shape:** author the adoption-relevant migrate workflows in the methodology pack, mirroring migrate-prd.

## F11 — mixed-content sources — CONFIRMED (deliberate; re-open trigger fired)

- `jigc migrate` is strictly one foreign file → one `--as` doctype → one task → one commit, by recorded decision (`design/auto-migration.md:99-103`; the M25 open at :200 — "NOT built … re-opened only if a real corpus makes the N-invocation cost hurt"). Mechanics: single source seam (`migrate.rs:39, 186-199`), per-workflow single-type create-gate, all-or-nothing retire (`task.rs:1461-1481`) — so a mixed source can't be migrated twice nor fan into two doctypes in one task.
- **The transaction machinery already supports multi-promotion** (`plan_promotions` walks all staged docs; the review gate renders a rewrites vector, render.rs:1127-1144) — the constraint lives only in the seam/create-gate/retire.
- **The trigger has fired twice:** the dashboard trial (manual splits: ARCHITECTURE.md → arch-doc + 4 ADRs; TODO+CONCERNS → 22 ideas) and jigc's own VISION.md (needs vision + several reference docs — see [self-migration-coverage.md](self-migration-coverage.md)). Fork re-opened in decisions-pending, keyed to the doctype-completeness milestone. Smallest build shape when picked up: widen a migration workflow's `allows-create` to multiple types (or a `migrate-mixed` workflow), keep single seam + single retire.

## F12 — doc show help — CONFIRMED

- Show help (`crates/cli/src/doc.rs:138-149`) ends "takes no `--task`" with no pointer; the not-found route (`engine/src/store.rs:159-166`) says "create the referenced doc…", actively misleading when the doc is staged in a task. `jigc task diff` (task.rs:73-77) is the sanctioned read.
- **Fix shape:** one sentence in the Show doc-comment (+ optionally the `store.not-found` route).
