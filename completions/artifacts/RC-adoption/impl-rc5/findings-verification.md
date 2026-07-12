# Findings verification — implementation-half trial vs the rc.5 code (2026-07-12)

Every trial claim adversarially verified against HEAD (= installed 1.0.0-rc.5) by four independent verification passes — code inspection (file:line) plus live repros in throwaway repos with the installed binary. Verdicts: **CONFIRMED** / **PARTIAL** (real behavior, claim needs correction) / **REFUTED**. Source claims: [trial-record.md](trial-record.md). Grouped as the trial hit them: **U** (the upgrade path, probe 1) · **W** (workflow/spec surface, probe 3) · **R** (read surface/addressing, probe 2) · **M** (finalize/milestone, probe 4) · **L** (log-analysis discoveries).

## The headline chain — U1→U2→U3, verified end-to-end in one repro

rc.4 stamp → the `store-version.binary-mismatch` advisory routes to `jigc setup` (never `migrate-corpus`) → setup re-stamps + commits silently → **`jigc validate` reports "no findings — the committed store validates clean" over a v1-stamped, v2-non-conformant `docs/deferral-ledger.md`**. The load-bearing defect is U2's placement blindness; U1's route and U4's missing dry-run are amplifiers; U5 is an independent contract gap.

### U1 — the binary-mismatch route omits `migrate-corpus` — **CONFIRMED** (Medium)

`crates/cli/src/setup.rs:92-104` (`binary_mismatch_finding`): route = "align the running jigc to {recorded}, or re-run `jigc setup` to re-stamp the store at {running}" — verbatim in the live repro, no mention of `migrate-corpus`. Root cause: the advisory treats a version delta as pure provenance, written before schema-version bumps rode binary bumps. Fix: version-aware route — when any manifest doctype version exceeds any committed stamp, name `migrate-corpus`.

### U2 — validate blind to stale schema-version stamps — **PARTIAL: CONFIRMED for placement doctypes, refuted as a blanket claim** (High)

The help promise is real (`cli.rs:181-188`: "Detect-with `jigc validate`; this migrates"). For a *located* doctype, validate detects and routes correctly (repro: a v1-stamped ADR draws blocking findings routed to the corpus migration). But family 5 (`schema_conformance_store`, `crates/engine/src/validate.rs:554-640`) **skips every placement doctype** at `validate.rs:561-563` (`schema.location = None` → `continue`) — and the only doctypes at schema-version 2 are placement doctypes (`deferral-ledger`, `changelog`). The M40 `hollow_surplus_store` walk covers placement instances but runs no version check. **The gap is test-asserted as a "recorded placement bound"** (`crates/cli/tests/migrate_corpus_value_remap.rs:294-316`) — frozen in at M41 rather than closed. **False green reproduced live:** committed v1-stamped `kind: D` ledger + rc.5 stamp → validate clean, exit 0; `migrate-corpus` then migrates the doc the "detect" verb couldn't see. Every future placement-doctype version bump (changelog, roadmap, vision, decisions-log, deferral-ledger…) inherits this. Fix: extend `schema_conformance_store` over the `committed_instances` walk; flip the bound-assertion test to expect the routed finding.

### U3 — setup re-stamps over an unmigrated corpus — **CONFIRMED** (Medium alone; High in the chain)

`install` (`setup.rs:686-782`) writes `.jigc/version` unconditionally (`write_version_stamp`, `:765-771`); no step inspects committed schema-version stamps. Repro: normal success banner, re-stamp committed, zero warning, over the non-conformant ledger. The stamp is *defined* as binary provenance — but with U2, re-stamping erases the last visible trace of the delta. Fix direction: fix U1/U2 (the detector + route); optionally a one-line setup warning off the existing `frozen_doctype_versions` map.

### U4 — no `--dry-run`/`--check` on `migrate-corpus` — **CONFIRMED** (Medium)

`Command::MigrateCorpus` is a fieldless unit variant (`cli.rs:188`); `migrate_committed_corpus` writes to disk immediately (`migrate_corpus.rs:325-330`). The designed detect-half was `jigc validate` — U2 removes that for placement doctypes, so there is genuinely no non-mutating way to learn migration is needed. Precedent in-house: `task finalize --dry-run` exists (`task.rs:99-101`). Fix: fix U2 (validate becomes the real dry-run) and/or add `--dry-run` printing the existing `migrated/already_current/blocked` report without `persist`.

### U5 — `migrate-corpus` output can only be landed by raw git — **CONFIRMED** (Medium-High)

No git invocation, no task mint anywhere in `migrate_corpus.rs`; repro leaves ` M docs/deferral-ledger.md` dirty. Contrast: `setup` makes its own pathspec-limited commit (`commit_install`, `setup.rs:949-1017`). The adapter contract (`adapter.rs:1024`, `BOOTSTRAP_SENTENCE`: "write every change back through `jigc`") is unqualified — an agent following it has no sanctioned way to land the rewrite. Fix: give migrate-corpus a commit boundary in the setup mold (pathspec-limited, synthesized message, `--no-commit` opt-out).

## W — the workflow/spec surface (probe 3)

### W1 — the bind gate's caption contradicts the populated criteria — **CONFIRMED** (Low-Medium)

`crates/cli/pack/steps/locate-from-spec.yaml:1-15` is static prose; only the `{{ @task.spec#criteria }}` slice is bind-sensitive, so post-bind the "Pick the spec…" block and the caption "empty until you bind a spec and re-compose:" print verbatim above the populated criteria. **No conditional mechanism exists — by declared dialect invariant** (`design/workflow-dialect.md:104`: an empty resolution yields empty text, never conditional inclusion — that would be control flow). The fix is prose, not conditionals: state-neutral caption wording in the template. (The general done/pending render is [ideas/state-aware-compose.md](../../../ideas/state-aware-compose.md).)

### W2 — `task bind` silent on success — **PARTIAL** (Medium-Low)

Behavior confirmed: exit 0, stdout+stderr 0 bytes (`task.rs:281-284`, `:1051-1097` — every success path `Ok(())`, no print, no `--format`). The "violates the M41 contract" framing is overstated: the pinned ack surface enumerates **doc** write verbs only; `task bind` writes task-state and appears nowhere in `command-output-contract.md` (its sibling `task discard` is silent the same way). Contradicts the contract's *spirit*, not its letter. Fix: a one-line ack + JSON envelope (`op: "bind"`) as an additive contract extension.

### W3 — the slug fusing + trailing `-and` — **CONFIRMED, both halves** (Medium)

Repro mints exactly `implement-docsspecscorrectness-monitoring-and`. (a) `slug.rs:112-123`: only space/underscore map to `-`; `/` and `.` fall to the default arm and are **dropped entirely** — pre-M41 behavior, untouched by V7/F5. (b) The M41 retreat-then-re-drop pipeline *worked as designed* — the char backstop retreated to a word boundary and re-dropped edge stopwords — but `EDGE_STOPWORDS` (`slug.rs:64`) is articles + short prepositions only; **"and" is not in the set** (Fork 5 settled "articles + short prepositions"; conjunctions were never included). Fix: map `/`/`.` (and plausibly `:`) to `-` at the separator arm; add `and`/`or` to the edge set. Mint-time-only (`is_slug` recognition unchanged); a set change is a mint-behavior change worth a DECISIONS line.

### W4 — "bare `jigc start` lists the gates it grants" is false — **CONFIRMED, aggravated** (Medium)

The instruction lives in the shared `crates/cli/pack/steps/implement.yaml:23-25`; bare `jigc start` lists pack + workflows + Run hints, **no gate information on any surface** (describe included). Aggravation: `implement-from-spec` grants only the adr gate — the step's literal `jigc doc create changelog` command would be **refused** by the create-gate in that workflow. Fix: split the changelog paragraph into a step included only by gate-granting workflows (structurally honest, no conditionals needed), or reword to a true pointer.

### W5 — `start --help` claims an intent mints — **CONFIRMED** (Low)

`cli.rs:56-58` ("an `<intent>` mints a task and composes the default workflow") is stale from before the M2 router flip: the shipped default is `router` (`creates-task: false`, `knobs.yaml:20-23`); repro confirms no task minted. Trivial help-string fix.

### W6 — nothing prompts `maps-to-test` — **CONFIRMED** (Medium)

The field exists (`spec.yaml:38`) but is optional; `author-spec.yaml` prompts title+statement only; `implement`/`implement-from-spec`/`locate-from-spec` never mention it; the **only** pack text naming it says to leave it off (`author-migration-spec.yaml:50-52`). The `criterion-maps-to-test` check gates only anchors that *exist* — there is no "criterion lacks a test anchor" finding; repro: a criterion with no anchor validates clean. **Zero fills across all trials is the expected outcome of this wiring, not agent negligence.** Fix: the implement step demands the wiring per satisfied criterion (pack YAML); a `criterion-without-test` advisory is a deliberate later scope (touches the frozen check surface).

### W7 — spec has no lifecycle/closure — **CONFIRMED** (Medium-High by the known-hole lens)

`spec.yaml` sections: meta/goal/context/criteria — no status; the schema's own header comment concedes it ("No `status`/`date`: nothing in the M3 arc reads them … one-line additions when a flow earns them"). The only inbound edge is `commit.implements` — unprompted by any step **and on a transient doctype** (never persisted, so the edge index never sees it). The adr lifecycle pattern exists in-pack; spec never got it. The trial's implement-from-spec flow is exactly the flow that now earns it — but it's a frozen-v1 schema change (version bump + migration): the post-1.0 doctype-completeness bucket, [ideas/spec-criterion-status.md](../../../ideas/spec-criterion-status.md).

### W8 — spec open-decisions unconnected — **CONFIRMED, sharpened** (Medium)

The schema declares **no open-decisions structure at all** — the trial spec's section is prose (or an undeclared surplus section drawing only the M40 advisory). Zero pack references to any such structure; the ADR create-gate is a prose invitation connected to nothing; finalize never checks. Fix two-tier: template instruction now; the version-gated `open-decisions` repeatable + `resolved-by → adr` ref is [ideas/spec-open-decisions.md](../../../ideas/spec-open-decisions.md).

## R — the read surface (probe 2)

**Cross-cutting:** R1+R2 are one theme — the M41 finding-key/write grammar and the M39/M40 pinned read contract **diverged at exactly two points** (simple-section field leaves; item ids), both on the read side, and both were *pinned that way* rather than slipping in silently. The fixes are contract revisions (pre-1.0 additive), not just code patches.

### R1 — validate's field-level target unreadable by `doc show` — **PARTIAL: the read hole CONFIRMED; "accepted by no verb" REFUTED** (Medium)

Repro: validate emits `adr:<slug>#status/cites-code`; `doc show` on it → `store.no-such-item`; on `#cites-code` → `store.no-such-section`; on `#status` → exit 0 with **empty output** (a third wart the trial missed — a header section slices to nothing). **Refuted half:** `set-field` accepts **both** forms (`#cites-code` *and* `#status/cites-code`) — validate's address round-trips into the *write* grammar; only the read surface rejects it. Item-level round-trips are fine. Root cause: `slice_fragment` (`engine/src/store.rs:246-331`) has **no field-leaf branch for simple/header sections at all** — the pinned slice table (`design/doc-read-surface.md:18-22`) simply has no `#section/<field>` form for non-repeatable sections. A design-level hole in the M39/M40 pin colliding with the M41 finding-key normal form. Mitigation: whole-doc `--format json` exposes `fields`. Fix: a field-leaf branch in `slice_fragment` (+ header `#section` renders its fields instead of empty) — additive contract revision.

### R2 — item `{#id}` missing from the JSON read surface — **CONFIRMED** (Medium)

`item_json` (`crates/cli/src/doc.rs:2262-2308`) never inserts `item.id` — mirroring the pinned contract ("An item object", `doc-read-surface.md:52`), **a pinned omission, not a serializer bug**. Worse: the rendered plain-text section slice also strips the `{#id}` anchor from headings. Why it's real, not a wart: the id is not derivable from the title (slug word-cap/stopwords, and decisively `retitle-item` **freezes the anchor**), so after any retitle the id is undiscoverable programmatically except by parsing raw markdown — the stable-opaque-IDs thesis contradicted by its own machine read surface. Fix: `"id"` in `item_json` (additive) + keep the anchor on rendered headings.

### R3 — "absent from the working tree" for an untracked file — **CONFIRMED: message wrong, behavior by design** (Low-Medium)

The task-scope probe adjudicates the **staged index** deliberately (`ActiveTask::materialize_index`, `task.rs:578-593` — "the about-to-be-committed code is exactly the staged set", M30 G4); `git add` alone turns it clean. But the message (`probes/doc-code/src/main.rs:150`) says "absent from the working tree" — false in git terms — and the route ("update the citation … or restore the cited symbol") is actively wrong advice for the untracked-new-file case. The probe can't currently tell index-scratch from live tree (same string serves store scope, where it's accurate). Fix: pass a root-kind flag in the snapshot; task-scope message says "absent from the staged index — `git add` it if new".

### R4 — `describe` run-on paragraphs — **CONFIRMED: deliberate design, over-applied** (Low)

`render::describe` joins all definitions with `" "` (`render.rs:1484-1528`), declared "deliberately hostile to parsing" per `design/introspection.md` → Non-contractual by design. But the pinned properties forbid key-shaped lines and bullet rows — **nothing forbids a paragraph break per definition**; the renderer over-applies the rule, producing output hostile to *reading*, not just parsing. Fix: blank-line-per-definition — still no extractable key, still passes the pinned format properties.

### R5 — singletons require `type:type` — **CONFIRMED** (Low)

`Address::parse` hard-requires `:` (`address.rs:174-180`); no caller special-cases singletons. The machinery exists — `store::read_slice` already knows a placement singleton's only slug is its type (`store.rs:133-146` rejects others with "its only address is `<type>:<type>`", proving the redundancy). Fix: on `MissingColon` at the CLI parse boundary, resolve a bare token that names a singleton/placement doctype to `type:type`; parser stays strict otherwise. *(The log shows 7 live failures on exactly this — the M41 V12 repair covered `rename`'s parse error, not the doc verbs.)*

### R6 — no `jigc doc list` — **CONFIRMED** (known/owned)

No enumeration verb exists (`describe` is the doctype *menu*, `ingest` is adoption triage, `doc show` needs a known address). Matches [ideas/doc-search.md](../../../ideas/doc-search.md) layer 1 — fix-shaped.

### R7 — `doc show` committed-only, no `--task` — **CONFIRMED as a declared design bound** (no defect)

`design/doc-read-surface.md:9` declares it; the live `store.not-found` route says "read it with `jigc task diff <id>`" verbatim (M40). Legitimate follow-on question (is a *diff* an adequate read-back surface?) but the bound is deliberate and documented — out of any fix wave.

### R8 — store validate "blocking" + exit 0 — **PARTIAL: facts confirmed, "undocumented contradiction" overstated** (Low)

The disambiguating trailer **exists** ("report-only at store scope (exit 0); these gate at `jigc task validate` / `jigc task finalize`", `render.rs:372`) and the exit contract is documented (`validation.md:281-283`). The real basis of the confusion: store scope reads **committed docs** but resolves code against the **live working tree** (verified live) — a repoint staged in an open task is invisible to it, and no per-finding line says so. Fix: render store-scope severity as `blocking (gates at finalize)` and/or one trailer clause on the committed-docs bound.

## M — finalize + milestone (probe 4)

**Cross-cutting:** M1+M3+M4+M5 share one root — the milestone/team-ready arc designed the **happy path** (create → add → join → finalize) and the trial exercised the **unhappy paths**: partial staging under an uninformed pack, dependent increments, abandonment.

### M1 — finalize commits only the index, edits silently left out — **PARTIAL: behavior by design; "silently/no warning" refuted; the real defect is a pack gap** (Medium-High in practice)

Index-honoring is the declared M30 contract (`design/finalize.md:66`: finalize **never** runs `git add --all`; `:61`: "the agent has already `git add`ed its own code edits"); only staged-*nothing* blocks (`finalize.nothing-staged`); a partially-staged index commits as-is **deliberately** ("surfaced, not prevented", `finalize.md:113`). And a warning *does* fire — the post-commit `left-out (unstaged/untracked — git add to include):` list (`render.rs:485-491`; `--dry-run` forecasts it). M40 F7 is unrelated (it hardened the *migration* stage). **The real root cause of the trial experience:** the dev pack's implement steps carry the staging contract verbatim ("`git add` your code edits before finalize — it commits only what you have staged", `pack/steps/implement.yaml:1-3`) — but the **methodology pack's `dev-task`**, which the trial ran, never mentions staging (`packs/methodology/steps/implement.yaml`/`finalize.yaml`). The agent was never told, and the left-out warning arrives only after the incomplete commit landed. Fix: the one-sentence staging contract in the methodology finalize/implement steps; anything stronger (pre-commit advisory, `--strict`) touches the recorded surfaced-not-prevented decision → Settle.

### M2 — hook-failure UX, no `--no-verify` — **PARTIAL** (Low)

No `--no-verify` is a **declared invariant** (`finalize.md:178-179`: "bypass would be silent-data-loss territory"; `task.rs:2094-2097` never passes it; pinned by `flow37_rename.rs:370`) — `HUSKY=0` works only because the environment is inherited. "No jigc framing" is refuted: the frame "`git commit` was rejected (no commit was made):" exists (`task.rs:2116-2123`), and raw hook relay is a recorded M40 decision (the hook output IS the correction signal). **The real gap:** the frame never says the *task* survives and finalize is re-runnable. Fix: one appended line — "task `<id>` intact — fix what the hook rejected and re-run `jigc task finalize <id>`". Do **not** add `--no-verify`.

### M3 — milestone sub-tasks pinned to a shared base — **CONFIRMED, BY DESIGN; the defect is discoverability** (Medium as UX)

The wall the agent hit is the re-entry base guard (`start.rs:1528`,`:1598`); the shared-base pin is the declared join model (`write-commands.md:119`, `storage.md:214`) under the one-bounded-primitive invariant — sequential-with-dependencies is outside it on purpose. The independence assumption is documented in exactly one thin place (`milestone-execution.yaml:4` — "independent sub-tasks"); nothing at create/add-task warns, and the guard's route ("switch back or discard") is what sent the agent into the discard-everything spiral. Fix: a sentence in `milestone create`'s success output + the guard message naming the independence constraint. The mode question is [ideas/sequential-milestone.md](../../../ideas/sequential-milestone.md).

### M4 — discard leaves a stale committed milestone-record — **CONFIRMED, a defect** (Medium)

`task discard` removes only `.jigc/tasks/<id>/` (`task.rs:259-266`); the record is flipped **only** at milestone finalize (`flip_record_for_finalize`, `milestone.rs:1481-1511`). Repro: discard both sub-tasks → the committed record still claims both `status: active`, `list-tasks` still lists them; manual `unmanage` + `git rm` is the only cleanup. `design/team-ready-state.md` models only active→joined — **the abandon path was never designed**, and a fresh clone would `reseed_cache` from the lying record and resume ghosts. Fix: a first-class abandon that settles the record (the write primitives — `set_item_field` splice, `commit_record_only` — already exist), or refuse discard on a sub-task outside it.

### M5 — milestone completion skippable — **CONFIRMED, one correction** (Medium via M4; the rest by-design at current altitude)

Correction: `chore(milestone): drop …` is **not a jigc op** — the agent hand-crafted that commit; there is no drop/abandon verb at all. So the accurate finding: **there is no close path other than finalize, and nothing notices if you never finalize** — no store-scope probe flags a committed record stuck `active` with no workbench. The methodology completion workflow is opt-in prose; finalize never runs tests by declared design (the gate is prose; enforcement was never promised — the ergonomics bet, invariant #2's honest boundary). Fix: the M4 abandon op closes the worst; a store-scope advisory "milestone-record `<id>` is active with no live workbench" makes the dangling state visible at `validate`.

### M6 — commit type/scope "four representations" — **PARTIAL: presentation inconsistent, functional claim REFUTED** (cosmetic)

The fields *are* nested in `header` (`commit.yaml:14-20`); the JSON `doc schema` (contract-v2) correctly carries `"section": "header"`; but the **agent-format** `doc schema` flattens with no section shown. Both address forms work on rc.5 — `#type` *and* `#header/type` succeed (`field_target` accepts single- and two-hop, `doc.rs:2841-2869`); "only `#header/type` worked" does not reproduce. Fix: print the owning section in the agent-format field lines.

### M7 — `milestone create` positional vs `doc create --title` — **CONFIRMED** (papercut)

Real, but patterned: positional is the work-unit-minting convention (`start "<intent>"`, `add-task <id> <intent>`), `--title` the doc-verb convention — a split that's nowhere documented. Fix: document the convention (or align); a help-text example on `milestone create`.

### M8 — jigc internals write bare `reset: moving to HEAD` reflog entries — **REFUTED**

jigc never runs `git reset` (full-source grep; rollback uses `git restore --staged --worktree`, landing uses `merge --ff-only`). Repro reflog after setup/finalize/hook-reject/milestone ops contains only identified `commit:` entries. The `reset: moving to HEAD` signature is **git's own `stash`** — written by the agent's baseline-measurement stash, not jigc. No fix; at most a docs note that jigc never rewrites HEAD destructively.

### M9 — minted task id not echoed — **CONFIRMED for agent/human formats; JSON has it** (papercut, real cost)

The id appears only embedded in `Run:` strings; `--format json` carries it first-class (M41 V2). Fix: a `task minted: <id>` header line in agent/human compose output (symmetric with `finalized <hash>`).

### M10 — create-gates discoverable only by failure — **CONFIRMED, plus a bonus defect** (Low-Medium)

Composed output demonstrates allowed creates but never states the gate; bare `start` and `describe` project no gates. **Bonus:** the dev pack's `implement-quick.yaml` asserts "bare `jigc start` lists the gates it grants" — false on rc.5 (the same false sentence W4 found in `implement.yaml`). Fix: a one-line `create-gates:` footer on composed task output (front-matter already loaded at compose) + fix/drop both false sentences.

## L — log-analysis discoveries (not in any agent's report)

### L1 — blocked finalizes log `finding_codes: []`

Two blocked `task finalize` records carry ~3.8 KB of output but an empty `finding_codes` array — the block *reasons* are opaque to log analysis (the succeeding retry logs codes correctly). Instrumentation gap in the invocation log's finding capture on the failure path.

### L2 — `doc show` bare-slug failures ×7: the V12 fix covered `rename` only

Bare singleton addresses (`roadmap`, `decisions-log`, `deferral-ledger`) failed 7 times on `doc show` — the M41 V12 bare-slug discoverability repair landed on `rename`'s parse error, not the doc-verb address parser.

### L3 — `doc list` invoked twice; the verb doesn't exist

Agents expect an enumeration verb next to `doc show`/`doc schema`. See [ideas/doc-search.md](../../../ideas/doc-search.md) (layer 1 is fix-shaped).

### L4 — one `task diff` emitted 187,851 bytes

4× the next-largest output of the whole trial — an output-size ceiling candidate (the M39 fd-tee measures it; nothing bounds it).
