<!-- persisted verbatim 2026-09-10 from the evidence-check-1.0 session; agent: Claude Opus 5 (1M context); see VERDICT.md -->
# What jigc's 1.0.0 would promise, and whether HEAD can keep it

Independent review · repo `/Users/maurice/projects/gherrink-jigc` @ `bd348a83` · binary `target/release/jigc` = `1.0.0-rc.14` · 2026-09-10.
Everything below was driven against that binary in throwaway corpora built with `dev/jigc-rig`, or read at HEAD. No cargo was run; no repo file was touched.

## Verdict on contract readiness

**The contract surface is real, unusually well-fenced, and not quite closed — and the repo's own gate on the 1.0.0 call is explicitly open.** The frozen doctype schemas (6 dev + 11 methodology), the three pinned read contracts (`doc show`, `doc list`, `doc schema` at contract-version 6), the orientation `SCHEMA_VERSION` 3, the findings envelope and its `(code, target)` key, the exit-code taxonomy and the error-code registry are each fenced by standing tests that assert *shape and value*, not key presence — several byte-verbatim, several derived from a code-side registry so a new member reddens rather than slips. That is genuinely better than most 1.0s. Against that: **four envelope keys ship on pinned surfaces that no design doc declares** (`guide` on `upgrade`; `guide_file`, `install_commit`, `hook_committed` on `setup`), which the command-output contract's own rule calls "a defect, not an addition"; **`jigc config set docs-root` `git mv`s the committed corpus and tells the JSON envelope nothing about it**, a parity gap the M48 fence is structurally blind to because the moves are not fields of `ConfigAck`; **two user-facing `--help` texts are stale or false** (`validate` claims to sweep "code anchors" — driven, it emits schema-conformance findings; `migrate-corpus` claims a "v0→v1 transform" over a surface that now ships five transform kinds across arbitrary bumps); **the adopter-facing back-out ladder tells adopters that `jigc task discard` makes no commit when it is the tenth committing door**, and **the 1.0 read contract's own declared conformance witness (`team-ready-state.md:195`) states a milestone-record shape missing four keys the binary emits**; **the invocation-log record shape and 20 of the 22 adapter deny-floor patterns are only partially fenced** — the deny-floor e2e test derives its expected list from the very file it is checking, so deleting `Read(./**/*.key)` from the profile keeps it green. None of these is a data-loss or corruption defect, and none was found by driving anything exotic. But the additive-key window closes *at the 1.0 pin* by written declaration, so every undeclared key on a pinned envelope becomes a post-1.0 versioned-extension problem the day the pin lands, and the parity gap on `config set` is a capability hole that the pin makes expensive to fill. **Blocking, on the repo's own terms: the RC-rc14 conversion ledger is OPEN** (`completions/artifacts/RC-rc14/trial-record.md:200` — *"The 1.0.0 call is not taken until it closes, and that is the human's gate"*), and `RC-rc14/findings-verification.md` carries 14 `UNPINNED:` against 15 `pinned-by:` across 13 rows. A pattern worth naming: **every one of these is a statement about a set — a key set, a door count, a doctype count, a probe scope — that the code moved and the prose did not.** That is the same incomplete-sweep class the repo has been closing wave after wave, now landing on the contract *documents* rather than the code. My recommendation: close the four undeclared keys, the two help lies and the four stale counts (each is one commit, each is contract-shaped, and after the pin the key ones cost a versioned extension), decide the `config set` parity gap explicitly rather than by omission, then take the call.

---

## 1 · The pinned surface, and what fences it

Seventeen frozen doctype schemas across two manifests, four machine contracts, three code-side registries, one adapter artifact, one log format. Grades below are mine, from reading the cited test bodies.

### 1.1 The dev-pack schema freeze — `crates/cli/pack/config/schema-manifest.yaml`

Frozen set: `commit` v1, `adr` **v2**, `spec` v1, `prd` v1, `arch-doc` v1, `changelog` **v2**; plus `slug-rule {version: 3, hash: 1291873d…}`.

- `crates/cli/src/pack.rs:3267` `shipped_schema_manifest_matches_the_frozen_doctype_set` — loads every shipped schema through the production loader, then `assert_eq!(declared, ["adr","arch-doc","changelog","commit","prd","spec"])`, per-entry `schema_version` equality, then `engine::manifest::check(&manifest, &schemas)` — the real recompute under strict set-equality.
- `crates/cli/tests/freeze_enforcement.rs:163` `schema_shape_drift_without_manifest_bump_is_blocked` — mutates a copied `adr.yaml`'s `location:`, drives the real binary, asserts non-zero + `"schema-hash mismatch"` + `"adr"`. Control arm at `:190`.
- Blast radius: `:1267` `every_door_blocks_on_a_drifted_frozen_schema` iterates `FREEZE_DOORS`, whose membership is fenced at `:1143` by `assert_eq!(FREEZE_DOORS.len(), VERB_KINDS.len())` with each argv parsed against the real clap tree. Slug-rule arms `:1315`/`:1373`; project-layer shadow arms `:1743`/`:1810`.
- Successor rule (a hash moving without its version) is a CI-only comparator, `crates/cli/tests/manifest_freeze_fence.rs` (verdict table `:405`–`:478`, fail-closed on unparseable, `Manifest-Repin:` escape).

**Grade: FENCED.** One declared bound, in the manifest header itself: the successor fence **protects this repo, not adopters** — it lives in this repo's CI and is a build fence, minting no finding, route or verb.

### 1.2 The methodology-pack freeze — `packs/methodology/config/schema-manifest.yaml`

Eleven entries: `commit` 1, `completion-record` **2**, `decisions-log` 1, `deferral-ledger` **2**, `dogfood-record` 1, `idea` 1, `milestone-record` **3**, `planning-record` 1, `research` 1, `roadmap` 1, `vision` 1; same slug-rule pin (both manifests must move together).

- `crates/cli/src/pack.rs:3346` `methodology_schema_manifest_matches_the_frozen_doctype_set` — identical shape, exact 11-member set equality, per-entry version, then `engine::manifest::check`.
- `freeze_enforcement.rs:312` drift arm through the real binary; **`:348` `a_reworded_methodology_hint_composes_clean_with_no_bump`** is the discrimination arm — it proves the M47 presentation projection, i.e. that the hash is not merely "any byte".

**Grade: FENCED.**

### 1.3 `jigc doc show --format json` — the pinned 1.0 content read

Driven shape (committed): `{fields, item-count, schema-version, sections, slug, type}`; staged adds `"staged": "<task-id>"`. Matches `design/doc-read-surface.md`:59–69 exactly.

- `crates/cli/tests/doc_show.rs:383` `doc_show_serves_the_committed_read_surface` — **four whole-output equalities against `r#"…"#` constants** (`VISION_JSON:314`, `PRD_JSON:329`, `REQUIREMENTS_JSON:355`, `ONE_REQUIREMENT_JSON:372`). Full key sets, byte-verbatim, not presence checks.
- `:1001` `the_stamp_key_is_whole_doc_only_and_fields_stays_stringy` pins the *type* discrimination the M49 additive key exists for: `whole["schema-version"] == json!(1)` (number) vs `whole["fields"]["schema-version"] == json!("1")` (string).
- Nested/item-leaf/staged shapes byte-pinned in `doc_show_nested.rs`, `doc_show_item_leaf.rs`, `doc_show_staged.rs`.

**Grade: FENCED.**

### 1.4 `jigc doc list --format json`

Driven: `{"docs":[{id, path, state, item-count}]}`; `--task` lists staged rows. `crates/cli/tests/doc_list.rs:277` asserts the whole stdout equals a `STORE_JSON` constant (`:196`) including an `unregistered` row, plus the empty case pinned as `"{\n  \"docs\": []\n}"`; `:461` pins non-zero `item-count`; `:564` pins the staged shape. **Grade: FENCED.**

### 1.5 `jigc doc schema --format json` — `contract-version` 6

Driven on `vision`: `{contract-version: 6, type, schema-version, fields[], sections[]}` with `set-field`/`set-slot` write addresses and a `ref`'s `to`.

- `crates/cli/tests/doc_schema.rs:523` `doc_schema_json_is_the_pinned_contract` — three byte-verbatim goldens each opening `"contract-version": 6`, plus behavioural assertions (`fields.len() == 15`, per-field `author-required` polarity, the M45 three-state settability: `field("schema-version").get("set-field").is_none()`). Version re-asserted `:985`, `:1155`; the M50 `to` key `:1206`.
- `compose_goldens.rs:380` runs `doc schema <ty> --format json` for **every doctype × 6 fixture states** against byte goldens (96 of 634 golden files).

**Grade: FENCED** — and this is the one contract with an explicit no-additive-carve-out bump rule (`doc-read-surface.md`:108).

### 1.6 `design/command-output-contract.md` — composed output, write-acks, findings envelope, `(code, target)`, `hook_output`, exit codes

| Element | Fence | Grade |
|---|---|---|
| Composed `{task, text}` | `compose_task_minted.rs:297` `the_json_contract_stays_exactly_task_and_text_on_both_arms` — `assert_eq!(keys, ["task","text"])`, a **closed** key set. Same at `start_compose.rs:2425`, `compose_statefulness.rs:325`, `compose_create_gates.rs:237` | FENCED |
| Write-ack `op`/`target`/`findings`/`value` | `text_json_parity_axis.rs:921` `doc_ack_parity` destructures all nine `DocAck` variants **exhaustively, no `..`** (a new field fails to compile) and `carries()` (`:503`) asserts `doc[key] == to_value(field)` — value identity. Concrete `op` values pinned per verb (`doc_write.rs:264`, `create_or_update.rs:216`, `doc_author.rs:419`, `doc_remove_item.rs:534`, `doc_rename_in_task.rs:307`) | **PARTIALLY FENCED** — no assertion that the envelope carries *no extra* key |
| Findings envelope + `(code, target)` | `crates/engine/src/finding.rs:1326` `finding_json_projection_is_the_pinned_envelope` — `insta` snapshot of the full 8-key document. Enforced **structurally**: `:1366` `serializing_a_targetless_finding_fires_the_seam` is `#[should_panic]` and rides `Finding`'s own `Serialize`, so no funnel list can go stale. `:1489` pins the URI target; `result.rs:1336` fires on a colliding key | FENCED |
| `hook_output` | `hook_output_axis.rs` enumerates all 7 hook-capable commit sites from the code (`git_commit_capture` callers) with the one stated exclusion (`setup` is `--no-verify`); `assert_hook_surfaced_json:202` asserts stdout is *exactly one* JSON doc, the key carries the marker, **and** stderr carries the verbatim relay | FENCED |
| Exit-code taxonomy | Constant `crates/cli/src/task.rs:129` `EXIT_CODES`; `exit_codes.rs:130` derives every expectation from `exit_code_for(class)` rather than a literal, with an arm per outcome class; `:599` pins the one-way door as a literal (`EXIT_ERROR == 1`, `(statement == constant)`) over a clap-derived case list; `registry_seam.rs:151` pins 3 and 4 | FENCED |
| Stream discipline | `machine_output.rs:134` (reject) and `format_json_success_axis.rs:933` (success), the latter bijected against the clap tree at `:885` | FENCED |

**Driven confirmations.** `task diff` → `{op:"task-diff", task, base:{sha,short}, code_diff:"", staged_docs:[{id}], findings:[]}` — present-always as declared. `task finalize --dry-run` clean → `{dry_run, left_out, manifest, subject}`; landed → `{committed:{files,hash,hook_output,left_out,manifest,promoted,subject}, findings, schema_version}` with `hook_output: ""` — as declared. `validate` → `{blocking_probes, findings, report_only, schema_version, scope}` — as declared. `task validate` → `{schema_version, findings}`, correctly *not* carrying `blocking_probes` (the contract scopes that key to the store sweep).

### 1.7 Orientation `SCHEMA_VERSION` 3 on `jigc start --format json`

- `crates/engine/src/result.rs:858` `orientation_view_state_tagged_projection_is_the_stable_contract` — **whole-object equality** for all three states (`unset-project`, `clean`, `active-task`), so no stray key can survive; includes the `findings: null` + reason arm.
- The literal 3 is pinned at `result.rs:692` (`assert_eq!(SCHEMA_VERSION, 3, "…describe's per-definition origin_pack bumps the schema version")`), plus five `start-orient-json--*.txt` compose goldens.
- End-to-end: `orientation_active_task.rs:67` / `:329`. Note that integration test re-asserts neither the version nor a closed key set — closure lives in the engine test.

**Grade: FENCED.**

### 1.8 The error-code registry — `crates/cli/src/invocation_log.rs:191`

Eleven members, all derived: `:504` `registry_mirrors_the_declared_members` asserts `ERROR_CODE_REGISTRY == COMMITTING_DOORS.map(error_code) + ERROR_REVIEW_PENDING`, `len() == 11`, and pairwise distinctness ("each door names ITSELF — a shared code would put a lying verb in the log"). `:480` asserts no member collides with `engine::result::check_inventory_codes()`, with a non-vacuity guard.

**Grade: FENCED**, with one honest caveat: membership at *construction* (`Outcome::error`) is a `debug_assert!`, compiled out of the release binary; the const-level test is the real fence.

### 1.9 The invocation-log record shape — `invocation_log.rs:382`

Eight fields: `timestamp`, `argv`, `exit_code`, `duration_ms`, `finding_codes`, `output_bytes`, `binary_version`, `error_code`.

`crates/cli/tests/invocation_log.rs:128` `knob_on_appends_one_well_formed_record` asserts **presence/type only** — `rec["timestamp"].as_str().is_some()`, `rec["duration_ms"].as_u64().is_some()`, `rec["finding_codes"].as_array().is_some()`. I read it: there is **no key-set equality anywhere**, so an added key is unfenced. The other four are individually value-fenced: `:236` `binary_version == CARGO_PKG_VERSION`; `:259` `output_bytes == stdout.len() + stderr.len()`; `:327` `error_code == Some("finalize.commit-rejected")` on a hook-rejected finalize vs. `null` on an absent task.

**Grade: PARTIALLY FENCED.** Every shipped field is reached; three only by type; the key set is open. This matters because `design/measurement.md` calls it "an independently-versioned surface" and it carries no version integer.

### 1.10 The adapter profile deny floor — `crates/cli/adapters/claude-code.yaml` (22 patterns)

- The two human-owned destroyers are **hard-pinned by literal**: `crates/cli/src/adapter.rs:1569` asserts `Bash(jigc uninstall:*)` and `Bash(jigc milestone discard:*)` are in the *loaded* profile, with a negative (`milestone provision` must stay off), and `flow51_acceptance.rs:1362` drives real `setup` → both present, a pre-seeded user entry survives → real `uninstall` → both gone, user entry stands.
- **The other 20** (`Read(./.env)`, `Bash(cat ./**/*.pem:*)`, `rm -rf`, `curl`, `wget`, `git push --force`, keys, credentials, `.npmrc`) are fenced only by `e2e_audit.rs:290`, whose expected list is **parsed out of the shipped YAML itself** — I read `floor_patterns()` at `:261`: `include_str!("../adapters/claude-code.yaml")`, scrape the `deny:` block. Deleting `Read(./**/*.key)` from the profile keeps that test green. The `insta` golden at `adapter.rs:1495` would redden but is re-acceptable by regeneration.

**Grade: FENCED for the two destroying verbs / UNFENCED for which secrets are denied.** For a 1.0 that ships a security floor, that is the wrong side of the line.

### 1.11 `.claude/skills/jigc/SKILL.md` — the version stamp

Driven: `jigc setup` writes front-matter `jigc-version: 1.0.0-rc.14` + `jigc-body-blake3: 01f1ab33…`, body = QUICKSTART then MIGRATING, and the file is listed in the install summary.

`adapter_artifact.rs:185` asserts `jigc-version == env!("CARGO_PKG_VERSION")` and `jigc-body-blake3 == hash_bytes(body)`; `:232` replaces a stale-but-self-consistent `0.0.1-old` stamp; `:476`/`:568`/`:617` are the clobber-refusal and `upgrade`-reports-only arms; `:808`/`:861`/`:921` the teardown. **Grade: FENCED.**

### 1.12 `MIGRATING.md`'s cross-version promise

The promise as written: `jigc validate` **blocks** (non-zero) on a doc below its doctype's current schema-version via `schema-conformance.schema-version-current` (the exit flip is `cli::render::STORE_EXIT_FLIPS` member `unmigrated-corpus`; I read the table at `render.rs:909` — five members: `probe-unreliable`, `oob-rename`, `unmigrated-corpus`, `ahead-corpus`, `unadopted-instance`); `migrate-corpus` is deterministic, byte-stable, idempotent, lands its own pathspec-limited commit, and `blocked` docs are first-class output. Driven on a current corpus: `migrate-corpus --dry-run --format json` → `{migrated, already_current, blocked, unadopted, unfilled, commit, hook_output, dry_run}`, exit 0, nothing written. **Grade: FENCED for the mechanism** (the exit-flip table is a code-side registry with a `report_only` predicate reading the same source as the exit code). See §5 for two doc-lag defects in the prose.

### 1.13 The M48 text/JSON parity fence — what it actually buys

`crates/cli/tests/text_json_parity_axis.rs`, three tests: `:1800` the registry **bijects** the clap leaf-verb tree (`REGISTRY.len() == from_clap.len()`, floor ≥ 47, every verb classified `Fenced(renderer)` or `Judgment(reason, Disposition)`); `:1836` renderer names ↔ the `FENCES` table; `:1863` runs each check — build a witness, render through the **real renderer** in both formats, destructure the Rust value **exhaustively (no `..`)**, per field assert `carries()` (key present *and* value-identical) or a declared exclusion, with `text_prints()` proving the gap is a gap.

**Its declared blind spots, and both bite below:** it does not check keys the envelope carries that the text never prints, it does not check for *extra* envelope keys, and it can only see values that are **fields of the acked Rust value**.

---

## 2 · Law 1 spot-check — the docs vs the binary

Twenty-four `--format json` outputs driven across the read and acting verbs. Twenty conform to their governing doc exactly. Six divergences, graded.

### D1 — `jigc upgrade --format json` carries an undeclared key `guide` · **binary side**

```
$ jigc upgrade --format json
{ "checked": 0, "findings": [], "guide": ".claude/skills/jigc/SKILL.md", "schema_version": 3 }
```
`guide` is added at `crates/cli/src/render.rs:730` and documented **only** in that function's doc-comment (`:712-718`, "The clean line widens with the sweep (M48 Increment 10 / T2)"). `grep -c guide design/command-output-contract.md` = 1, and that one hit is an unrelated word inside the `task-bind` bullet. Every other envelope key ships with its own declaration paragraph — `hook_output` (`:437`), `hook_file` (`:439`), `committed` (`:441`), `unadopted` (`:448`), `unfilled` (`:454`), `already_absent` (`:460`), `checked` (inside the `:446` discharge). By that doc's own words at `:446`: *"an undeclared key on a pinned envelope is a defect, not an addition, whichever wave mints it."*

### D2 — `jigc setup --format json` carries three more undeclared keys · **binary side**

```
$ jigc setup --format json
{ "allowlist_file": …, "findings": [], "guide_file": ".claude/skills/jigc/SKILL.md",
  "hook_committed": false, "hook_file": ".git/hooks/pre-commit",
  "install_commit": "24a45bb", "installed": true, "line_file": "CLAUDE.md" }
```
The contract's M48 paragraph (`:439`) describes this envelope as carrying `hook_file` *"the **fourth** fact beside `installed` / `line_file` / `allowlist_file`"* — four facts. The binary emits eight. `guide_file` and `install_commit` appear in **no** design doc (grep across `command-output-contract.md`, `assistant-adapter.md`, `project-setup.md` = 0); `hook_committed` appears once, in `assistant-adapter.md`, not as a declared envelope key.

D1 + D2 together are the same shape: the additive window is declared closed at the pin, and four keys will be inside it undeclared on the day it closes.

### D3 — `jigc config set docs-root <path>` moves the committed corpus and the envelope never says so · **binary side, capability-tier**

```
$ jigc config set docs-root documents --format json
STDOUT: { "committed": false, "key": "docs-root", "op": "config-set", "value": "documents" }
STDERR: relocating 1 committed doc(s) stranded by the `docs-root` re-point to `documents`
        (… each move is a staged `git mv` — commit it with your next commit):
          - docs/milestone-records/probe.md → documents/milestone-records/probe.md
```
Stream discipline holds (stdout is pure JSON). But a driver reading the pinned ack sees a knob write and **cannot tell that N tracked files just moved and were staged**. This is precisely the rule M48 shipped a fence for, and the fence cannot see it: `ConfigAck::Set` is `{key: String, value: String}` (`render.rs:2541`) and `config_ack_parity` (`text_json_parity_axis.rs:1314`) destructures exactly those two fields. The relocation is not a field of the acked value, so an exhaustive destructure over `ConfigAck` proves nothing about it. (`committed` is likewise renderer-added, outside the enum.) The fence is sound over its subject and blind here — the incomplete-sweep shape this repo names in its own record, one layer out from where it was last looked for.

### D4 — `jigc validate --help` describes a sweep narrower than the one it runs · **binary side, law 1**

Help text: *"Re-check every committed doc's **code anchors** against the codebase and report drift."* Driven on `committed-singletons`, the sweep's only finding was `schema-conformance.repeatable-populated` — not a code anchor. `design/validation.md`:356 states the store-scope envelope has **three** targets (doc↔code, workflow↔refs, file↔CLI-state), joined since M42 by the `schema-conformance` intrinsic whose whole job is to flip this verb's exit code. This is the verb `MIGRATING.md` step 1 tells adopters to gate on and the one an adopter's CI binds; its own help understates its scope to one of four families.

### D5 — `jigc migrate-corpus --help` names a transform space that no longer exists · **binary side, law 1**

Help text: *"applies the deterministic **v0→v1** transform (the schema-version stamp + any structural splice)."* At HEAD the shipped kind space is `SchemaChangeKind::ALL` — `EnumWidened`, `ValueRemapped` (M41), `AddedOptionalSection`, `AddedItemSlot`, `AddedItemField` (M49/M50) — over arbitrary version bumps (`adr` 1→2, `changelog` 1→2, `milestone-record` 1→2→3, `completion-record` 1→2, `deferral-ledger` 1→2). "v0→v1" is a sentence from M34 that four waves have falsified.

### D6 — `jigc doc show --help` understates its own pinned shape · **binary side, mild**

Help says the pinned shape is *"a whole-doc object `{ type, slug, fields, sections }`"*. The binary emits six keys — `item-count` (M44) and `schema-version` (M49) additionally — both declared in `design/doc-read-surface.md`:68–69 and both byte-pinned in `doc_show.rs`. Its sibling `jigc doc list --help` **does** name `item-count` in its shape sentence, so the two read surfaces describe themselves to different standards.

### Non-divergences worth recording (checked, and clean)

- `jigc doc show --task` adds exactly `staged` and nothing else; a committed serve carries no `staged` key. Matches `doc-read-surface.md`:19.
- `jigc describe --format json` carries `schema_version: 3`, `router_hidden`, `origin_pack` — matches `introspection.md`:56/58/60, including the doc's careful "unpinned rather than unparseable" posture, which `describe --help` restates correctly.
- `jigc task bind` failure → `{"error": "…"}`, a flattened string with no `code`/`key`. This is **not** an undeclared divergence: `implementation/decisions-pending.md`:~600 already carries it as a recorded, triggered deferral ("An operational-funnel refusal reaches `--format json` as a flattened `{"error": …}` string"), and `command-output-contract.md`:209 states the `finding_to_err` posture. Correctly declared, not fixed.
- The advisory-route floor held on every finding I saw, including the informational one (`file-state.staged-copy` → *"no action needed…"*), and a route I checked for existence (`jigc start --workflow record-change`) resolves to a real hidden workflow.
- The `{#id}` anchor sentence, the stdout-purity guarantee and the `doc rename` vs `jigc rename` split are all present in long help, as `doc-read-surface.md` says they are.

### `--help` vs `design/command-catalog.md`

Worth stating plainly: `command-catalog.md` governs `{{cli.<id>}}` command-refs, **not** verb help text. No verb-help claim can be checked against it. The governing docs for help prose are `design/surface-contract.md` (laws 1–3 + the style guide) and each verb's own part-doc; D4/D5/D6 are law-1 findings against those. SKILL.md is byte-identical to QUICKSTART + MIGRATING concatenated, so every §5 finding below is also a SKILL.md finding — it inherits them by construction.

---

## 3 · Evolution posture

**There is a written statement, it is unusually explicit, and it is consistent across its two homes.** The two homes are named as such:

- `design/doc-read-surface.md`:88–92 — the read side. *"Additive keys are permitted **pre-1.0 only**; from the 1.0 pin, the shape evolves only by an **explicitly versioned extension** — never a silent additive key."* Then `:90` **The window closes here — the discharge (M48)**: keyed to the pin, not to a wave name, with the earlier *"M48 is the last pre-1.0 wave"* premise explicitly **withdrawn**. `:92` records M49's last spend (`schema-version`) and states *"from the pin, an absent key is a decision."*
- `design/command-output-contract.md`:399 (Evolution posture) and `:446` (the discharge) — the write/compose side, same rule, same keying to the pin, plus the sentence that does the real work: *"an undeclared key on a pinned envelope is a defect, not an addition, whichever wave mints it."*

**Contract-version semantics are stated per surface, and the five regimes are named in one place** — `doc-read-surface.md`:175–194, "The version/posture map":

| Surface | Version integer | Rule |
|---|---|---|
| `doc show --format json` | **none** | pinned shape; additive pre-1.0 only, then a versioned extension |
| `doc list --format json` | **none**, by declaration | the `doc show` posture |
| `doc schema --format json` | `contract-version`, now **6** | bumps on **any** structural change — **no additive carve-out** |
| the result envelope | `schema_version`, now **3** | versions the envelope, orthogonal to the two above |
| a doctype | `schema-version` | versions the document shape; two readings — expected (from `doc schema`) vs actual (from `doc show`) |

`doc schema`'s bump history is recorded inline (1→2 rc.5 `of`/`section`; 2→3 rc.7 write addresses; 3→4 M45 settability; 4→5 M48 `write-key`; 5→6 M50 ref `to`), which is what makes the no-carve-out rule checkable rather than aspirational.

**The freeze promise** — `implementation/doctype-map.md`:38–42 — is the strongest statement in the set: the six dev doctypes + the schema-definition format are frozen v1, the freeze is *"machine-checkable, not prose discipline"*, and *"the only way to change a frozen schema's shape is to bump its `schema-version` and ship the M34 corpus migration (or revert)."* `:42` restates the methodology set at eleven and is itself fenced (`crates/cli/tests/doctype_map_versions.rs` checks the table against **both** shipped manifests).

**The corpus-migration promise across versions** is forward-only and now symmetric: below-version blocks and routes at `migrate-corpus`; **above-version also blocks** (the confidence-audit's `schema-conformance.schema-version-ahead`, routed *"upgrade jigc, or restore the stamp from git history"* — `render.rs:909` member `ahead-corpus`). A corpus stamped ahead of the binary is never silently skipped. That is the right answer and it is on the record.

### What is NOT stated

1. **No statement anywhere of what a 1.x may change vs. what triggers 2.0.** Searched `design/`, `implementation/`, `VISION.md`, `CLAUDE.md`, `QUICKSTART.md`, `MIGRATING.md`, `WHY-JIGC.md`, `READINESS-ASSESSMENT.md` for `semver` / `semantic version` / `major version` / `2.0.0` / `1.x` / `breaking change` / `deprecat` / `versioning policy`. **No document states a release-versioning policy for the jigc binary** — nothing on what a 1.x may change, what triggers a 2.0, whether a verb, flag, finding code or exit code may be removed inside 1.x, or what an adopter can rely on across a `contract-version` bump (`doc schema` bumps on "any structural change" with "no additive carve-out", and no compatibility promise accompanies it). The only "v2" language in the repo is **per-contract, not per-binary**. Every posture statement is keyed to "the 1.0 pin" and stops there — *after* the pin the rule is "a versioned extension", but nothing says whether minting `command-output contract v2` is a 1.x-compatible act or a 2.0 act, nor whether a doctype `schema-version` bump (which is explicitly permitted, with a migration) is a minor or a major event for the CLI. For a product whose whole value proposition to an adopter is *your corpus is safe across upgrades*, that is the missing sentence.
2. **No adopter-facing home for any of it.** All five statements above live in `design/` part-docs — which the QUICKSTART and MIGRATING guides that jigc actually installs into an adopter's repo (as SKILL.md) do not carry and do not link (SKILL.md's preamble says so explicitly: *"every other jigc document named below lives in the jigc project's own repository, not in this one"*). An adopter reading the installed guides learns nothing about contract versioning. There is no README.md at repo root either.
3. **The freeze fence's adopter bound is declared but not surfaced.** Both manifests' headers say the successor fence *"protects this repo only, not adopters"*. True and honestly recorded — but it means an adopter shipping a project pack (which PB-1 now permits) gets no such guarantee, and `decisions-pending.md`'s **N26** records the consequence driven: a manifest-less project pack stamps nothing, so `schema-conformance.schema-version-current` can never fire for its doctypes and `migrate-corpus` reports `0 blocked` after a shape change.

### Inconsistencies found

Six, three of them load-bearing. All quoted at HEAD; the first three I re-verified by driving or by counting the code-side registry myself.

**I-1 (high) — `MIGRATING.md` contradicts itself about `jigc task discard`, two lines apart.** `:39`: *"**All nine committing doors** carry the frame, each with its own error code in the invocation log."* `:40`: *"Back out a task: `jigc task discard <id>`. Workbench-only — it removes the task's working area … and touches nothing else: **no commit**, the committed store exactly as it was."* I counted `cli::invocation_log::COMMITTING_DOORS` (`invocation_log.rs:130-171`): **ten** members, and the tenth is `jigc task discard`. `design/team-ready-state.md:102` and `:105` say why — a milestone sub-task discard is a **record-only door** that *"settles before it destroys"*, landing a path-scoped record commit **before** removing the working area, and `design/surface-contract.md:142` registers `task-discard.commit-rejected`. So MIGRATING.md's adopter-facing back-out ladder is wrong about whether the safest-sounding verb in it commits. The same stale count appears in `CLAUDE.md:7` (*"the whole nine-door committing axis"*) and `implementation/decisions-pending.md:286`; `design/surface-contract.md:129` correctly says **eleven** registry members (ten doors + `migrate.review-pending`), and `design/worked-examples.md:3066` correctly avoids a count by iterating the table.

**I-2 (high) — the 1.0 read contract's declared *conformance witness* states a shape the binary no longer emits.** `design/doc-read-surface.md:5` names the milestone record as the pin's proof. `design/team-ready-state.md:195` enumerates it as `{ type, slug, fields: { base, status, schema-version }, sections: { tasks: [ { task-id, intent, status }, … ] } }`. Driven at HEAD:

```
$ jigc doc show milestone-record:witness-probe --format json
{ "fields": {"base": {"sha":…,"short":…}, "schema-version": "3", "status": "active"},
  "item-count": 1,
  "schema-version": 3,
  "sections": {"tasks": [{"id":"first-thing","intent":"first thing","status":"active",
                          "task-id":"first-thing","workflow":"dev-task"}]},
  "slug": "witness-probe", "type": "milestone-record" }
```
Four keys the witness's enumeration omits: top-level `item-count` (M44), top-level `schema-version` as a number (M49), the item `id` (M42 — *"the key that closes the json contract under its own address grammar"*), and the item `workflow` leaf (M49, the fresh-clone durability fix that took `milestone-record` to schema-version 3). Every one of the four is correctly declared in `doc-read-surface.md`; the witness doc was never updated.

**I-3 (medium) — `design/corpus-migration.md:281` counts the methodology manifest at ten.** *"…would have deleted a documented capability for all sixteen shipped doctypes (**6/6 dev + 10/10 methodology are manifest-listed**)."* The methodology manifest lists **eleven** (ten persisted + the transient `commit` shadow, which is exactly what its own header and `doctype-map.md:42` say — *"Size a methodology schema change against eleven, not ten"*). Seventeen doctypes total, not sixteen. Both manifest headers also still say *"re-pinned all **16** doctype hashes"*, though that reads as M47 history rather than present tense.

**I-4 (medium) — the four undeclared envelope keys of §2 D1/D2 are inconsistencies with the posture itself.** The doc says every spend is declared *"here, in its own paragraph, as it ships"*; `guide`, `guide_file`, `install_commit` and `hook_committed` are not, and `command-output-contract.md:439` states the setup envelope's fact count as **four** where the binary emits **eight**.

**I-5 (medium) — the result envelope's `schema_version` rides an unstated subset.** `doc-read-surface.md:185/188` says it is *"a top-level key on **every** result envelope"* and *"has ridden every block/error envelope and every orientation since the engine's first result type."* Driven, it rides `start`, `describe`, `validate`, `task validate`, `task finalize` (both arms) and `upgrade` — and does **not** ride `task diff`, the doc write-acks, `config get`/`list`/`set`, `setup`, `ingest`, `migrate-corpus`, or the `milestone` record-only acks. `task diff` is the sharp case: declared in the *same* contract doc, same envelope family as `task validate`, no version integer while its sibling has one. The split is defensible (`ValidationReport`/`OrientationView` carry it; ad-hoc `serde_json::json!` envelopes do not) but no doc states it, so a driver cannot tell which envelopes are versioned.

**I-6 (low) — presentation drift in both posture homes.** `doc-read-surface.md:90` is headed *"The window closes here — the discharge (M48, the rc.11 wave)"* and is immediately followed at `:92` by *"The last spend before the pin (M49…) — the window is still open."* Same shape at `command-output-contract.md:446`, followed by three further declared spends (`:448`, `:454`, `:460`). Both are semantically reconciled — the close is re-keyed to the pin, not the wave — but a reader hitting "closes here" is told the window shut two waves before the last key landed, and both headings still name M48. Also: `doc-read-surface.md:175/177` says *"five regimes"* over a table with **six** rows (`describe` is the sixth, marked `n/a`). And `DECISIONS.md:3629`/`:3753`/`:5268` cite `doc-read-surface.md:87` and `command-output-contract.md:367` for the posture paragraphs — both now blank lines.

### Two adopter-facing gaps in the migration promise

- **`MIGRATING.md` never mentions the ahead-stamp case.** Its "Upgrading a jigc corpus across versions" section describes only the below-version direction. An adopter who downgrades a binary, or clones a repo written by a newer jigc, meets `schema-conformance.schema-version-ahead` — blocking, Human-routed, **no verb-side repair** (`migrate_corpus.rs:1472-1488`: *"no verb fixes a future stamp… upgrade jigc, or restore the stamp from git history"*) — with nothing in the guide it was handed. The behaviour is right; the documentation of it exists only in `design/validation.md:453`.
- **The probe wire shares the result contract's integer, and its versioning policy is deferred.** `doc-read-surface.md:185` records that `SCHEMA_VERSION` also stamps the probe request/response (`probe.rs:40`), so a bump moves what the engine writes to a probe's stdin and what the shipped `doc-code` probe writes back — while `design/validation.md:167`/`:187` leave the *policy* explicitly deferred and note *"no consumer compares the integer."* At 1.0, a pack author shipping a third-party probe binds to a wire whose evolution rule is undecided.


### Counts stated in prose that a future edit will stale

Not defects today (except I-1 and I-3 above, which already are), but each is a hand-written number over a set the code can move, on a surface that has no fence: `doc-read-surface.md:99`/`:183` (`contract-version` **6**, plus the running `3→4→5→6` history); `:185` (result contract **3**); `:173` ("four members"); `:175`/`:177` ("five regimes", six rows); `surface-contract.md:129` ("member-for-member — **eleven**"); `write-commands.md:94` ("nine-cell matrix"); `CLAUDE.md` ("thirteen `CELLS` rows", "nine-door committing axis"); both manifest headers ("all 16 doctype hashes", now 17 entries); and `MIGRATING.md:47`'s wall-clock perf figures ("800 items under 7 s, 1500 under 30 s") in an adopter-facing doc. `doctype-map.md:42`'s eleven/ten is the one that *is* fenced (`crates/cli/tests/doctype_map_versions.rs`, against both shipped manifests) — which is the shape the others want.

---

## 4 · Release mechanics — rc.14 → 1.0.0

### The single source

`Cargo.toml` `[workspace.package] version = "1.0.0-rc.14"` (line 10). Both crates inherit (`version.workspace = true`, `crates/cli/Cargo.toml:3`, `crates/engine/Cargo.toml:3`). Every runtime reader is `env!("CARGO_PKG_VERSION")` — `pack.rs:1444` (pack version), `invocation_log.rs:423` (`binary_version`), `setup.rs:54` (the SKILL.md stamp), `setup.rs:260`/`:370` (the `.jigc/version` store stamp). **There is exactly one string to edit.**

### Everything that carries a literal `rc.14`

Outside `target/` and `.git/`:

| Path | Count | Naïve bump leaves it stale? |
|---|---|---|
| `Cargo.toml` | 1 | — this is the edit |
| `Cargo.lock` (`cli`, `engine`) | 2 | **Yes** unless any cargo command runs; the CI `cargo build` regenerates it, but a hand-bump-and-push without a local build pushes a stale lock |
| `crates/cli/tests/goldens/compose/composite/start-orient--*.txt` (5) | 5 | **Yes** — these are the "ten version-bearing goldens", half of them |
| `crates/cli/tests/goldens/compose/composite/start-orient-json--*.txt` (5) | 5 | **Yes** — the other half |
| `CLAUDE.md` | 1 | Record prose; needs the M-close/fold-back edit anyway |
| `DECISIONS.md` | 3 | Record prose |
| `implementation/decisions-pending.md` | 4 | Record prose (§22 header says "SHIPPED 2026-09-09 as `1.0.0-rc.14`") |
| `completions/trial-harness/verify-pair.sh:22` | 1 | A default probe-set comment (`rc.13 -> rc.14`); stale but harmless |
| `completions/artifacts/RC-rc14/**` | many | Archived trial evidence — must **not** be touched |

The ten goldens all carry one line: `"header": "Pack: dev/1.0.0-rc.14 | methodology/1.0.0-rc.14 · Project config: <REPO>/.jigc/config"`. They redden on the bump, which is the only mechanical forcing function in the whole procedure.

### SKILL.md, CHANGELOG, self-hosting

- **SKILL.md has no repo file.** It is generated at `jigc setup` from `crates/cli/src/setup.rs:54` (`jigc-version: {CARGO_PKG_VERSION}`) plus the blake3 of its body. Nothing to bump; every adopter re-running `setup` after upgrading picks up the new stamp, and `adapter_artifact.rs:232` proves a stale-stamped pristine copy is replaced.
- **There is no `CHANGELOG.md` at the repo root** (`ls CHANGELOG.md` → No such file). **This repo does not self-host** — no `.jigc/` under version control, no managed docs, the doctype-completeness/self-migration milestone is explicitly post-1.0 (`decisions-pending.md`:482). So the `changelog` doctype's `record-change` / cut-a-release flow **does not apply to jigc's own release**. That is a real gap for a 1.0: jigc ships a changelog doctype and a release-cutting workflow and will ship 1.0.0 without a changelog of its own.
- **Install path.** `CLAUDE.md` prescribes `cargo build --release && install -m755 target/release/jigc ~/.local/bin/jigc`. `QUICKSTART.md` prescribes `cargo install --path crates/cli` **or** `cp target/release/jigc /usr/local/bin/`. Two documents, three paths, no single answer. Minor, but it is the first command an adopter runs.

### The "version-stamp confirmation"

I looked for it and it is **prose, not a fence.** `implementation/milestone-completion-workflow.md:24`: *"When the wave carries an rc name, its close bumps the workspace version to rc.N, regenerates any version-bearing goldens, re-runs the gate, and installs the release binary… This was convention, not rule, and it broke exactly once proving the need."* There is **no step in `packs/methodology/workflows/completion.yaml`** and no test asserting the bump. `crates/cli/tests/foldback_truth.rs:223` says so in as many words: *"The owed **version bump** is deliberately outside this fence… asserting a version string would be this fence choosing it."* What has actually caught the miss five times is a human running `jigc --version` at the completion step, backed by the ten goldens going red if you bump without regenerating. The forcing function is on the *regeneration*, never on the *bump*.

`crates/cli/tests/release_smoke.rs:109` asserts `jigc --version == "jigc {CARGO_PKG_VERSION}\n"` — derived, so it cannot catch a stale bump either.

### CI

`.github/workflows/ci.yml` — five steps (`fmt --check`, `clippy -D warnings`, `build`, `test`, then the `#[ignore]`d manifest-freeze fence over the pushed range with `fetch-depth: 0`). **No step is keyed to the version**, no publish step, no tagging, no release artifact. `Cargo.toml` carries `publish = false`.

### The ordered checklist

1. Close the RC-rc14 conversion ledger (§6 — the human's own stated gate).
2. Edit `Cargo.toml` `[workspace.package] version` → `1.0.0`. (One line; both crates inherit.)
3. `cargo build` (or any cargo command) to regenerate `Cargo.lock`'s two entries.
4. `UPDATE_GOLDENS=1 cargo test -p cli compose_goldens::` — regenerates the ten `start-orient*` goldens. **Verify the diff carries nothing but the version string** (this is the check M50's close performed by hand and recorded).
5. `dev/gate` — the full five-command gate, bare, exit codes captured.
6. `cargo build --release && install -m755 target/release/jigc ~/.local/bin/jigc`.
7. `jigc --version` → `jigc 1.0.0`. (The only confirmation that exists.)
8. Fold back the record: `CLAUDE.md`, `DECISIONS.md`, `decisions-pending.md`. `foldback_truth.rs` will redden if `CLAUDE.md`'s milestone paragraph overstates.
9. Optional but owed: `completions/trial-harness/verify-pair.sh:22`'s stale default-probe comment.
10. Not covered by anything: **git tag, release notes, CHANGELOG, root README, crates.io metadata, and the `publish = false` decision.** None exists, none is scripted, and `implementation/decisions-pending.md:619` is the deferral whose trigger this release fires (§6.4).

**Where a stale `rc.14` survives a naïve bump:** `Cargo.lock` (if no cargo runs before push), all ten goldens (caught by the gate), `verify-pair.sh`'s comment (caught by nothing), and every record document (caught by nothing except `foldback_truth.rs`'s narrow M50-paragraph arm). Nothing catches a bump that is *not performed* — that remains a human step with a prose rule behind it.

---

## 5 · User-facing docs against the release binary

Every command in QUICKSTART.md and MIGRATING.md was run as printed, in `bare` / `committed-singletons` rigs.

### QUICKSTART — what ran correctly

`jigc --version` → `jigc 1.0.0-rc.14` ✓. `jigc setup` → installed all six items the doc lists, in one `chore(jigc): install jigc workspace config` commit, with the hook correctly reported *not* committed (`.git/hooks/pre-commit` — the doc's "git cannot track a file there" branch) ✓. `CLAUDE.md` carries the bare `@.jigc/AGENT.md` import ✓. `.claude/settings.json` carries `Bash(jigc:*)`, `Bash(git add:*)`, the `SessionStart` hook and all 22 deny patterns ✓. `jigc start "<intent>"` → the router menu with the exact re-run line ✓. `jigc start --workflow single-task "<intent>"` → `task minted: add-per-client-rate-limit` ✓. The three reads (`doc list --task`, `doc show … --task`, `doc schema adr`) all ran as printed ✓. `jigc task discard <id>` → the `task-discard.staged-prose` refusal, exit 1, naming the read-back and the `--force` consent, exactly as documented ✓. `jigc task finalize <id>` → landed one commit with the rendered subject ✓. `jigc config set docs-root .` accepted ✓; `""` accepted and normalized to `.` ✓. `docs/milestone-records/` is the real home (`milestone-record.yaml:53` `location: milestone-records/`) ✓.

### QUICKSTART divergences

**Q1 — `--dry-run` "prints the manifest and stops" is true only on a clean task.** The doc: *"To see that set before committing, run `jigc task finalize <id> --dry-run`: it prints the manifest and stops, changing nothing."* Driven on a freshly minted task (the state a reader is in when they read that sentence — the commit doc's `type` and `summary` are still empty), it prints **two blocking findings and no manifest**, exit 3. MIGRATING.md:step 1 states the rule correctly (*"It forecasts no green it would refuse"*); QUICKSTART does not, and QUICKSTART is the file a first-time reader is in.

**Q2 — `jigc task validate` and `jigc task finalize --dry-run` do not preview the same set.** Same task, same moment: `task validate` emitted three findings (two blocking + the `changelog-recording.gate-granted-unused` advisory); `finalize --dry-run` emitted the two blocking ones and **dropped the advisory**. QUICKSTART presents them as the same preview surface (*"You can preview part of what finalize will gate on… this task's content findings, the carryover gate, the `owner-artifact` causes… and the granted-but-unused changelog gate"*), and `command-output-contract.md`'s third clause says *"every check `task validate` runs… is the same check `finalize` runs, at the same severity."* Advisories only, so no gate moves — but the sentence as written is falsified by the two doors' own output.

**Q3 — the install stanza contradicts CLAUDE.md.** QUICKSTART: `cargo install --path crates/cli` / `cp target/release/jigc /usr/local/bin/`. CLAUDE.md: `install -m755 target/release/jigc ~/.local/bin/jigc`. Not wrong, but three paths across two docs for step zero.

### MIGRATING divergences

**M1 — the dry-run manifest tag vocabulary is incomplete.** The doc: *"each path tagged by how it enters (`promoted` / `modified` / `deleted` / `carried-over`)."* Driven, an ordinary staged new file tags `added` — a fifth member (`ManifestKind::Added`, `render.rs:5872`, pinned at `:7381`). A reader looking for their file under four names will not find it.

**M2 — the `migrate-corpus` triage key list is two keys short.** The doc: *"the triage (`migrated` / `already_current` / `blocked` in `--format json`)"*. Driven: six keys — `unadopted` (M46) and `unfilled` (M46) are both first-class output the doc does not name, and `unadopted`'s emptiness **is the exit rule**. Both are declared in `command-output-contract.md`:448/454; MIGRATING has not caught up.

**M4 — `jigc task discard` is described as making no commit; it is the tenth committing door.** `MIGRATING.md:40`: *"Workbench-only — it removes the task's working area … and touches nothing else: **no commit**, the committed store exactly as it was."* On a **milestone sub-task** it lands a path-scoped record-only settle commit *before* removing the area (`design/team-ready-state.md:102`/`:105`; `cli::invocation_log::COMMITTING_DOORS` member 10; `task-discard.commit-rejected` in `surface-contract.md:142`). QUICKSTART.md carries the same sentence (*"it removes only the working area under `.jigc/tasks/`; no commit is made"*). The line above it in MIGRATING (*"All nine committing doors"*) is stale by the same edit. Two adopter-facing back-out sentences, both wrong in the direction that matters — they say nothing is committed when something is.

**M3 — `migrate-corpus --help` says "v0→v1"** (§2 D5). MIGRATING's own step 5 correctly says "every changed line either a `schema-version` stamp or a declared transform" — the doc is right and the help it sends the reader to is wrong.

### Not walked

MIGRATING steps 1–2 (validate blocks over a stale corpus, then `migrate-corpus` before `setup`), step 3's same-path `M`-not-`D`+`A`, and step 5's hook-rejection frame across all nine committing doors were **not driven** — each needs a deliberately stale or hook-rejecting corpus the rig does not build. They are covered by standing tests (`STORE_EXIT_FLIPS` + `commit_rejected_axis.rs` over `COMMITTING_DOORS`), which is why I graded §1.12 FENCED rather than verified-by-driving. Stated as a bound, not glossed.

---

## 6 · Open obligations

### 6.1 The one hard blocker, on the repo's own terms

**The RC-rc14 conversion ledger is OPEN.** `completions/artifacts/RC-rc14/trial-record.md:200`: *"Every row carries a repro block; most carry `UNPINNED:` with a reason, several because pinning them would pin a gap or a carried defect as expected output. **The 1.0.0 call is not taken until it closes, and that is the human's gate, not this record's.**"* Counted at HEAD: `findings-verification.md` carries 13 rows, 15 `pinned-by:` citations and **14 `UNPINNED:` markers**. `implementation/pinning.md` §3 explicitly refuses a mechanical checker for this, so closing it means reading each cited test's content by hand. This is the only obligation in the repo that is *stated* as gating the call.

Four more items are listed as "Owed after the trial" (`trial-record.md:204`): the human's reading of 3/3 against 1/3; **two declared changes reached by nothing** — the fan-out boundary (`1799a2d`) and the SKILL.md re-clobber — *"either walked or stated as uncovered in the 1.0.0 record"*; N27's re-argument as a cost question; and the F-2→F-9→F-10 chain.

### 6.2 The rc.14 trial's seven findings — all triggered, none 1.0-keyed

`decisions-pending.md`:179–195. Every row carries a written trigger; **none** is keyed to "before 1.0.0". Summary and my grading:

| Row | Substance | Tier | Contract-breaking to fix after 1.0? |
|---|---|---|---|
| (D) F-2→F-9→F-10 chain | `doc rename` leaves the staged commit summary naming the old title; no verb amends a landed commit; the two composed produced the trial's only adapter bypass. Bytes were byte-identical, nothing corrupted | capability | **No** — F-9 (rename re-slugs, tells the commit doc nothing) is a behaviour fix inside a task; adding a *new verb* to amend a landed commit would be additive |
| (D) 22 unknown-id doors answer `exit 1 NOCODE route` | A driver keying on `(code, target)` gets nothing when it names a task that does not exist | surface | **No** — minting a code + key at those doors is an addition to the findings envelope, which the envelope already permits per-instance. It does change an exit-1 payload from a flattened `{"error"}` to a findings document at 22 doors, so it is a **wire change** on a surface the flattened form has de-facto pinned — price it as such |
| (D) `status: superseded` with no `supersedes` validates clean | A conditional ref requirement no doctype states | capability | **Yes, potentially** — expressing it means a schema-definition-format addition, and the format itself is frozen v1 (`doctype-map.md`:40). A new schema key is a format change |
| (D) `jigc doc rename` vs `jigc rename` costs 2–3 help reads | Most-corroborated finding of the trial; B3-h2 ran the wrong verb first (exit 2) | surface | **No** |
| (D) No structural `adr → research` ref | Only prose links; `jigc rename` cannot repoint prose | capability | **Yes** — adding a ref field to `adr` moves its `schema-hash`, so it is a frozen-doctype event: bump `adr` to schema-version 3 + ship a corpus migration. Cheap mechanically, adopter-facing by construction |
| (D) F-11 `doc author` payload parse error has no severity/code/`at:`/route/footer; F-13 a milestone offers one execution shape | F-11 is un-swept M50 Increment 9; F-13 is a capability gap that made a blind worker abandon a milestone | F-11 surface / F-13 capability | F-11 **no**; F-13 **no** (a second execution shape is additive) |
| (D) The apparatus had no write channel | Instrument fix, already landed 2026-09-10 | n/a | n/a |

### 6.3 The six carried M50-close defects

The parent brief's "+1" is **N15**; N31 was struck 2026-09-09 (its trigger fired at the M50 audit and F3 discharged it over a larger class). All six are recorded at `decisions-pending.md`:~562–568, all driven at HEAD, all with written triggers.

| # | Defect | Tier | Contract-breaking after 1.0? |
|---|---|---|---|
| **N15** | `jigc doc show <bad-addr> --task <id>` is **byte-identical** to the task-less form, says the address names no **committed** doc, and routes to a read that silently **drops `--task`** — so following it serves committed prose to a reader holding a staged copy. Driven end to end. One arm of one resolver, on the **1.0-pinned read surface** | surface, but on a pinned surface | **No** — the message and route are not the pinned JSON. But it is the sharpest of the six: law 2's *nothing hides* at the last inch, on the read contract 1.0 pins, and the sibling arm (`store.not-staged`) is already correct |
| **N20** | The milestone boundary's **non-hook** refusal discards a fully-built `RejectionFrame`: no code, no route, no survived-state clause, and `Outcome::failure()` instead of `Outcome::error(code)`, so the door has **no identity in the invocation log**. State-safe (HEAD unmoved), and it is the *normal* state a fan-out fix round runs in | surface | **No** — but minting the door's identity adds a member to `ERROR_CODE_REGISTRY`, which is a closed vocabulary derived from `COMMITTING_DOORS`. Additive there, and the registry has no version integer |
| **N23** | **Retiring a doctype silently unmanages its committed, tracked corpus.** `jigc validate` says *"validates clean"* at exit 0, `doc list` drops the row entirely (not even `unregistered`), `migrate-corpus` reports `0 blocked`, `git ls-files` still carries the file. Only `doc show` tells the truth. Reachable by an ordinary adopter now that PB-1 lets a project own doctypes | capability | **No** to fix — but note it is a **false green on `jigc validate`**, the verb MIGRATING tells adopters to CI-gate on. Not data loss (bytes are tracked), but the worst of the six by consequence |
| **N26** | A **manifest-less project pack stamps nothing**, so `schema-conformance.schema-version-current` can never fire for its doctypes; after a shape change `migrate-corpus` reports `0 blocked` with no route and `doc show` routes to hand-repair. And adding a correct manifest makes **every door exit 1** at the pack-load stated-at fence | capability | **Possibly** — the "absent manifest = unchecked" rule is *stated design* in the manifest header, so changing it is a posture change adopters would feel. The stated-at-fence half is a plain bug |
| **N27** | `jigc task diff`'s cold-start form answers *what is here* with almost nothing — no `# code changes` header, no task id/workflow/intent/read-roles/provenance; JSON carries `task` + `base` and nothing else | surface | **Yes, mildly** — `task diff`'s envelope is pinned in `command-output-contract.md`:70–84 and the additive window closes at the pin, so adding `workflow`/`intent` keys after 1.0 costs a versioned extension. **Its trigger is the next trial's plant-E re-measure** — i.e. it is scheduled to be re-argued *after* the call, on a contract that will be closed by then |
| **N28** | Two forward edges may declare the same `inverse:` name at exit 0. Inert today by a recorded argument (the inverse name reaches exactly one prose phrase; the finding key is built from the forward relation) | capability | **No** — a pack-load uniqueness assert is additive |

### 6.4 The one deferral whose trigger a 1.0.0 release literally fires

`implementation/decisions-pending.md:619`:

> **Repo publishing floor — LICENSE, root README, `publish=false`, repo metadata (v1-audit-recorded, 2026-07-02).** Fine for internal go-live; owed the moment the repo is shown outward. *Trigger:* **first external adopter, public release, or opening the repo.**

Shipping 1.0.0 is that trigger by definition. State at HEAD: `LICENSE-MIT` + `LICENSE-APACHE` present ✓; **no root `README.md`** ✗; `Cargo.toml` carries `repository` but **no `description`, no `readme`, no `keywords`** ✗; `publish = false` still set — which is correct if 1.0.0 is not going to crates.io and wrong if it is, and nothing in the repo says which. This is the item I would expect a 1.0.0 checklist to open with and it is the one nobody has re-read.

### 6.5 Other pre-1.0-adjacent items I found in `decisions-pending.md`

None carries a literal "before 1.0.0" trigger. The nearest are: **the harness-surface wave** (chartered 2026-09-04, awaiting a milestone), **the doctype-completeness milestone** (`:482`, explicitly **post-1.0**, the jigc-self-migration prerequisite), **`jigc ingest` still adopts an unaddressable identity** (M50 Inc 2 T1, triggered on the ingest funnel next opening), **`engine::validate::adoption_route` emits an unquoted path token** so a spacey foreign path panics at exit 101 on a debug build and prints a route that runs against the wrong path on release (triggered on "the first adopter corpus carrying a foreign file whose path has a space" — that trigger is live the moment jigc has adopters), and **three facts `design/bootstrap.md` draws in its orientation examples are still unbuilt**.

### 6.6 What I would put in front of the human, ranked

1. **Close the conversion ledger.** It is the only stated gate, and it is open.
2. **Declare or delete the four undeclared envelope keys** (`guide`, `guide_file`, `install_commit`, `hook_committed`) and correct `command-output-contract.md`:439's "fourth fact" count. One commit; after the pin it is a versioned extension.
3. **Decide `config set`'s relocation parity explicitly** (§2 D3) — either the ack carries the moves or the doc records the refusal with a reason. Silence here becomes a permanent absent key at the pin.
4. **Fix the two `--help` lies** (`validate`'s scope, `migrate-corpus`'s "v0→v1"). Law 1, on the two verbs adopters bind CI and migration to.
5. **Fence the 20 deny-floor secrets patterns against something other than themselves.** A security floor whose test reads the file it checks is not a fence.
6. **Write the missing semver sentence** — what a 1.x may change, what triggers 2.0 — and put a version-policy paragraph somewhere an adopter reads (the installed SKILL.md, or a README that does not yet exist).
7. **Correct the four stale counts and the stale witness shape** — `MIGRATING.md`'s "nine committing doors" + "no commit" (I-1, and it is an adopter-facing back-out instruction), `team-ready-state.md:195`'s milestone-record enumeration (I-2 — the 1.0 read contract's own declared conformance witness), `corpus-migration.md:281`'s "10/10 methodology" (I-3), and `doc-read-surface.md:175`'s "five regimes" over six rows.
8. **Document the ahead-stamp direction in `MIGRATING.md`.** The behaviour is right and has no verb-side repair; the guide an adopter is handed does not mention it exists.
9. **N23** is the carried defect I would not carry: a false-green `jigc validate` over a committed, tracked, orphaned corpus, on the verb the migration guide tells adopters to gate on.

---

## Not verified

- **I ran no cargo.** Every test grade in §1 is from **reading the test body** at HEAD, not from executing it. I did not confirm the suite is green at `bd348a83`; `completions/artifacts/RC-rc14/trial-record.md:197` records that the gate was **red** at the sha the RC-rc14 handover certified green, and states it was fixed there — I did not re-verify that.
- **MIGRATING steps 1–2, 3 and 5 were not driven** (§5 "Not walked"): the stale-corpus block+route, the `migrate-corpus`-before-`setup` ordering, the same-path `M`-not-`D`+`A` landing, and the hook-rejection frame across the nine committing doors. Graded from their standing tests only.
- **The fan-out / milestone execute / join / `milestone finalize` path was not driven.** `milestone create` only. So N20 is relayed from the record, not reproduced.
- **`jigc relocate`, `jigc unmanage`, `jigc uninstall`, `jigc migrate <path> --as`, `jigc rename`, `jigc doc rename` were not driven** — help text read only.
- **The six carried N-defects were not independently reproduced.** They are quoted from `decisions-pending.md`, which states each was driven at HEAD when recorded. Per this repo's own rule (*an agent's report is a lead, not a measurement*), treat my tier gradings as mine and the underlying repros as the record's.
- **§1.13's mutation-non-vacuity claim** for the parity fence (deleting `hook_file` reddens it) is quoted from the suite's module doc; I did not apply the mutant.
- **The `Judgment`-tier disposition census** in `text_json_parity_axis.rs` (`:1678`, `:1750`) I read only in summary; I did not check each member's stated disposition against its verb.
- **Two grep-based absence claims are relayed, not exhaustively re-run by me**: that no document states a semver/release policy, and that the six stale line-number citations into the posture homes point at blank lines. I spot-checked both and they held; the exhaustive sweep was a delegated search. Per this repo's rule, treat them as strong leads.
- **I did not audit `crates/engine`'s internal invariants** — parse/write byte-stability, the splice path, the edge index. Out of scope for a contract review, and they are the most heavily fenced part of the repo.
