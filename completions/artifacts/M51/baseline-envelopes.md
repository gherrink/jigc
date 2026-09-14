<!-- M51 baseline · companion 3 of 4 — the pinned-envelope and fence machinery. Driven 2026-09-10 by one Opus capability-auditor against the release binary `1.0.0-rc.14` at HEAD `bd348a83`; no cargo run, no repo file edited. Verbatim as returned; consolidated in [baseline-ledger.md](baseline-ledger.md). -->

# M51 Scope baseline — the pinned-envelope and fence machinery

**Verified at HEAD `bd348a83`, release binary `1.0.0-rc.14`. A map, not gospel.** Every row
marked *driven* was produced by running that binary in a throwaway rig
(`dev/jigc-rig fresh --binary target/release/jigc`, bases copied per case). No repo file
edited; no cargo run. Nothing here is settled — that is the human's gate in *Settle*.

Driving artifacts: `/private/tmp/.../baseline3/sweep.sh` (the 48-row envelope sweep),
`keysets.txt` (its output).

---

## 1 · Every `--format json` envelope at HEAD — driven

**The axis.** `cli::cli::VERB_KINDS` (`crates/cli/src/cli.rs:1412-1465`) enumerates **47**
leaf verbs (13 top-level · 11 `doc` · 6 `task` · 8 `config` · 9 `milestone`), fenced total
against the clap tree by `cli_parse::every_leaf_verb_is_classified`. **All 47 speak
`--format json`** — `--format` is a global clap arg, and
`format_json_success_axis::every_leaf_verb_honours_format_json_on_its_success_path` drives
each to a genuine success. I replicated its 47 recipes against the release binary and
recorded the **actual top-level key set** of each, plus one extra arm
(`task finalize --dry-run`), for 48 rows. **Every row exited 0.**

| verb | driven top-level keys | `schema_version`? | declared? | key-set fence |
|---|---|---|---|---|
| `start` (orient) | `header next_steps schema_version state workflows` | **yes** (3) | partly — `state` in doc-read-surface; `header`/`workflows`/`next_steps` named in no design doc | **closed** — `result.rs:858` whole-object equality ×3 states + 5 byte goldens `start-orient-json--*` |
| `start "<intent>"` / `workflow` | `task text` | no | yes — coc §1 | **closed** — `assert_eq!(keys,["task","text"])` at `compose_task_minted.rs:332`, `compose_statefulness.rs:325`, `compose_create_gates.rs:237`, `flow45_acceptance.rs:542` |
| `setup` | `allowlist_file findings guide_file hook_committed hook_file install_commit installed line_file` | no | **4 of 8** (`guide_file`, `install_commit`, `hook_committed` undeclared — **EC-3**; coc:439 states the count as *four*) | value-carry (`setup_success_parity`) |
| `uninstall` | `allowlist_file findings line_file removed uninstalled` | no | **no** — `uninstalled` in no design doc | value-carry (`uninstall_success_parity`) |
| `upgrade` | `checked findings guide schema_version` | **yes** (3) | `checked` yes; **`guide` undeclared — EC-3** | presence-only (`upgrade_checked_close` proof) |
| `ingest` | `rows summary` | no | **no** — neither key in any design doc | value-carry (`ingest_parity`) |
| `migrate` | `task text` | no | yes — coc §1 | **none** — the three closed-key assertions all drive `start`/`workflow` |
| `migrate-corpus` | `already_current blocked commit dry_run hook_output migrated unadopted unfilled` | no | yes (all eight declared in coc) | value-carry via whole-report `json(report)`; declared out of parity |
| `unmanage` | `dropped identity path` | no | **no** — `identity` in no design doc | value-carry (`unmanage_parity`) |
| `rename` | `commit from hook_output new_path old_path prose_mentions referrers title to` | no | `commit`+`hook_output` yes; **`new_path` `old_path` `prose_mentions` `referrers` undeclared** | value-carry (`rename_parity`) |
| `relocate` | `blocked displaced moved` | no | **no** — `moved`/`displaced` in no design doc | value-carry (`relocation_parity`) |
| `describe` | `commands definitions schema_version` | **yes** (3) | per-entry keys yes (`router_hidden`, `origin_pack`, `pack`); **top-level `commands`/`definitions` named nowhere** — but introspection.md:~56 declares the json arm *unpinned rather than unparseable*, so this is **deferred-by-design** | presence-only; **no `describe --format json` golden** (the 6 `describe--*` goldens are the text arm) |
| `validate` | `blocking_probes findings report_only schema_version scope` | **yes** (3) | yes | judgment tier; disposition derived from `STORE_EXIT_FLIPS` |
| `doc create` | `copied_in existed findings op target` | no | yes | value-carry (`doc_ack_parity`, exhaustive destructure of all 9 `DocAck` variants) |
| `doc add-item` | `copied_in findings op target` | no | yes | value-carry |
| `doc remove-item` | `copied_in findings op removed target` | no | yes | value-carry |
| `doc retitle-item` | `copied_in findings op target title` | no | yes | value-carry |
| `doc rename` | `committed_identity copied_in findings from op reslugged target title` | no | `committed_identity`/`copied_in` yes; **`reslugged` in no design doc** | value-carry |
| `doc set-field` | `copied_in findings op target value` | no | yes (+`already_absent` on `--unset`) | value-carry |
| `doc set-slot` | `chars copied_in findings op target` | no | yes | value-carry |
| `doc author` | `copied_in findings op target` | no | yes | value-carry |
| `doc show` | `fields item-count schema-version sections slug type` | no (own regime) | yes — doc-read-surface:59-69 | **closed** — 4 whole-output `r#""#` equalities `doc_show.rs:314-383` |
| `doc schema` | `contract-version fields schema-version sections type` | no (`contract-version` 6) | yes | **closed** — 3 byte goldens + 96 compose goldens |
| `doc list` | `docs` | no (own regime) | rows yes; top-level `docs` named in no design doc | **closed** — whole-stdout `STORE_JSON` equality `doc_list.rs:277` |
| `task list` | **a top-level JSON *array***, not an object | impossible | **no doc states this envelope is an array** | value-carry (`task_list_parity`) |
| `task diff` | `base code_diff findings op staged_docs task` | **no** — EC-5's sharp case | yes — coc §2 | value-carry (`task_diff_parity`); the `json!` literal is closed by construction, no test asserts it |
| `task validate` | `findings schema_version` | **yes** (3) | yes | judgment tier, declared out |
| `task finalize --dry-run` | `dry_run left_out manifest subject` | **no** | yes | presence-only (`finalize_dry_run_subject_close` proves `subject`) |
| `task finalize` (landed) | `committed findings schema_version` | **yes** (3) | yes | presence-only + whole-value `committed` carry |
| `task discard` | `dropped findings op task` | no | yes | value-carry (`task_ack_parity`) |
| `task bind` | `findings op role target task` | no | yes | value-carry |
| `config get` | `key layer op rejected value` | no | **no** — `layer`/`rejected` in no design doc | value-carry (`config_get_parity`) |
| `config list` | `knobs op` | no | **no** — `knobs` only in multi-pack.md, unrelated sense | value-carry (`config_list_parity`) |
| `config set` | `committed key op value` | no | `committed` yes (coc M48 ¶) | value-carry (`config_ack_parity`) |
| `config insert-step` | `anchor committed op side step workflow` | no | **no** — `anchor`/`side`/`step`/`workflow` in no design doc | value-carry |
| `config replace-step` | `committed op step target` | no | partial | value-carry |
| `config remove-step` | `committed op target` | no | partial | value-carry |
| `config fill` | `committed op target` | no | partial | value-carry |
| `config fork` | `base committed op path target` | no | partial | value-carry |
| `milestone create` | `hook_output text` | no | `hook_output` yes (M45 widening) | judgment, `MILESTONE_PROSE_SUMMARY` |
| `milestone add-task` | `hook_output text` | no | as above | judgment |
| `milestone add-from-spec` | `hook_output text` | no | as above | judgment |
| `milestone list-tasks` | `hook_output text` | no | **`hook_output` on a `VerbKind::Read` verb — see F-new-2** | judgment |
| `milestone provision` | `hook_output text` | no | as above | judgment |
| `milestone execute` | `task text` | no | **coc §1 names three composed producers; this is a fourth — F-new-1** | **none** |
| `milestone join` | `findings milestone no_docs_from overlay schema_version` | **yes** (3) | `milestone`/`no_docs_from` yes; **`overlay` in no design doc** | value-carry (`milestone_join_parity`) |
| `milestone finalize` (landed) | `committed` — **only** | **no** | partial | value-carry (`milestone_finalized_parity`) |
| `milestone discard` | `hook_output text` | no | as above | judgment |

### EC-3 — **CONFIRMED, and the class is ~4× larger than the row states**

*status: latent defect (contract) · driven*

The four keys EC-3 names are real: `guide` on `upgrade`; `guide_file`, `install_commit`,
`hook_committed` on `setup` (driven above; `command-output-contract.md:439` states that
envelope's fact count as **four**, the binary emits **eight**).

But a grep of every driven key against **all of `design/`** (not just the three contract
docs) returns **13 further envelope keys that no design document names**:
`uninstalled` · `rows` · `summary` · `identity` · `new_path` · `old_path` ·
`prose_mentions` · `referrers` · `moved` · `displaced` · `reslugged` · `layer` ·
`rejected` · `knobs` · `anchor` · `side` · `step` · `overlay` — across
`uninstall`, `ingest`, `unmanage`, `rename`, `relocate`, `doc rename`, `config get`,
`config list`, `config insert-step` and `milestone join`.

**The gap under EC-3 is not "four keys were forgotten" — it is that nothing enumerates
which envelopes are pinned at all.** `command-output-contract.md` → *"The three surfaces
this pins"* names composed output, write-acks, and the findings envelope. `setup`,
`uninstall`, `ingest`, `unmanage`, `rename`, `relocate` and the eight `config` envelopes
are in **none** of the three, yet `setup`/`upgrade` were dragged in one paragraph at a
time by M48/M50 declarations. So `guide_file` is "a defect" only because `hook_file` next
to it happens to have a paragraph, while `identity` on `unmanage` is unremarked. There is
no code-side or doc-side list of *pinned envelopes*, and
`text_json_parity_axis::REGISTRY` — the one table that bijects the verb tree — classifies
**fence-ability**, never **pinned-ness** (`Tier::Fenced` / `Tier::Judgment`).

### EC-5 — **CONFIRMED, and sharper than the row states**

*status: latent defect (contract) · driven*

`doc-read-surface.md:185/188`: `schema_version` is *"a top-level key on **every** result
envelope."* Driven, it rides **7 of 48** arms: `start` (orient), `upgrade`, `describe`,
`validate`, `task validate`, `task finalize` (landed), `milestone join`.

Two cells the audit did not name:

- **One verb, two envelopes, split on a flag.** `task finalize` **landed** carries
  `schema_version: 3`; `task finalize --dry-run` carries **none**. Same verb, same door.
- **Two sibling committing doors disagree.** `task finalize` landed →
  `{committed, findings, schema_version}`. `milestone finalize` landed → `{committed}`
  alone (`render.rs:4040`: `json(&json!({ "committed": landed }))`) — no findings channel,
  no version integer.

The underlying rule is real and unstated: typed engine result values
(`ValidationReport`, `OrientationView`) carry it; ad-hoc `serde_json::json!` envelopes do
not. Nothing states it, and nothing fences the partition.

### F-new-1 — the composed contract has a **fourth** producer the doc does not name

*status: latent defect (contract) · driven*

`command-output-contract.md` §1: *"**Three verbs** emit this composed shape … `jigc start`,
`jigc workflow`, and `jigc migrate`. All three render through the one `render::composed`
json arm."* Driven: `jigc --format json milestone execute cache-rework` →
`{"task": …, "text": …}` — a fourth. `text_json_parity_axis.rs`'s registry entry for
`milestone execute` **says so in its own words** (*"`execute` renders through
`render::composed`… stdout IS the pinned `{task, text}` contract"*), so the code-side
census knows and the contract doc does not. Neither `migrate` nor `milestone execute` has
a closed-key assertion — all four `assert_eq!(keys, ["task","text"])` sites drive
`start`/`workflow`.

### F-new-2 — a read verb ships a commit-hook key

*status: shape-limited · driven*

`milestone list-tasks` is `VerbKind::Read` (`cli.rs:1459`) and emits
`{"hook_output": "", "text": …}` — it shares `render::milestone` (`render.rs:3698`) with
five write siblings. `hook_output`'s declaration (coc, the M45 additive-key paragraph) is
scoped to *"the landed-commit envelopes… the `rename`/`migrate-corpus` reports and the
milestone record-only op acks"*. A read verb is none of those, and the value is
structurally always `""`.

### F-new-3 — `task list --format json` is a top-level **array**

*status: latent (contract) · driven*

Every other envelope is an object. `task list` returns `[{id, workflow, intent}, …]`
(`render.rs:3285` `json(&rows)`), so it can carry no `schema_version`, no `findings`, and
no discriminator, and a driver cannot deserialize one envelope shape. No design doc states
this. `format_json_success_axis` passes it because its predicate is *parses as exactly one
JSON document*.

---

## 2 · The fence machinery, and what a closed-key fence would need

| fence | what it iterates | what it **cannot** see |
|---|---|---|
| `text_json_parity_axis.rs` — `REGISTRY` (47 rows, bijects the clap tree at `:1808`, floor `>= 47`), `FENCES` (15 renderer checks, bijected at `:1852`), `carries()` (`:503`) | for the **28 `Tier::Fenced`** verbs: destructure the acked Rust value **exhaustively, no `..`** (compiler is the field enumerator), then per field assert the envelope key is present **and value-identical**. For the **19 `Tier::Judgment`** verbs: a stated `Disposition` per member, one derived from `STORE_EXIT_FLIPS` | **Extra envelope keys.** `grep -n "as_object\|keys()" text_json_parity_axis.rs` → **no key-set assertion of any kind** (only `.len()` on registries). Its own module doc declares the blind spots: *"it does not check keys the envelope carries that the text never prints, it does not check for extra envelope keys, and it can only see values that are fields of the acked Rust value."* Also blind to **renderer-added** keys (`committed` on all six `ConfigAck`s, `guide` on `upgrade`) because those are added at the `Format::Json` arm, outside the enum |
| `format_json_success_axis.rs` | all 47 leaf verbs driven to a **genuine success** via hand-written recipes bijected against the clap tree (`:885`) | asserts only: exit 0 · stdout is exactly one JSON doc · stderr carries none. **Nothing about keys.** It is the only place all 47 successes are reachable — the natural host for a key-set fence |
| `machine_output.rs` | all leaf verbs on the **reject** surface, fixture-free (no verb can succeed), plus the clap carve-out and one chatty-hook landed success | stream discipline only — which stream carries the document. No key assertions except the `{"error"}` single-key envelope pinned in `finalize_outcome_surface.rs:485/560/635` |
| `crates/engine/src/result.rs:858` `orientation_view_state_tagged_projection_is_the_stable_contract` | `OrientationView`'s three states | **This is the one true closed-key fence**: `assert_eq!(json, json!({…}))` — whole-object equality, so a stray key fails. Scope: the engine value only; the CLI serializes it unwrapped (`render.rs:333/385` `json(&view.view)`), so it holds end-to-end for `start` |
| compose goldens (`compose_goldens.rs`, 634 files) | byte-verbatim: `start --format json` ×6 states, `start "<intent>" --format json` ×6, `describe` (**text arm only**), `doc schema <ty> --format json` × every doctype × 6 states (96 files) | a byte golden is a closed key set **and** a closed value set — the strongest fence here. But it covers **3 of 47** json-speaking verbs, and its regeneration is routine at every wave close, so it is re-acceptable rather than blocking |

**Is there a single seam a "no undeclared key on any pinned envelope" fence could ride?**

Half-yes, and the half that is missing is the expensive half.

- **The bytes have one seam.** `cli::render::json<T: Serialize>` (`crates/cli/src/render.rs:207`)
  is the *only* stdout JSON serializer in the CLI — `grep to_string_pretty crates/cli/src/`
  returns exactly one other hit, `adapter.rs:1082`, which writes `.claude/settings.json`, not
  stdout. Every `Format::Json` arm (59 in `render.rs`, plus 5 in `doc.rs`) funnels through it.
- **The identity does not.** `render::json` receives an already-built `T` and knows nothing
  about which verb it is rendering, so it cannot look a key set up. `Outcome`
  (`invocation_log.rs:35`) is **not** a JSON envelope — it is `{code, finding_codes,
  error_code}`, the log's own value — so there is no `Outcome`-shaped chokepoint either.
- **The values are heterogeneous.** ~33 sites hand `render::json` a typed struct; the rest
  hand it an ad-hoc `serde_json::json!` literal. A `#[serde(deny_unknown_fields)]`-style
  compile-time closure is therefore unavailable for roughly half of them.

**Minimal shape that would work, in ascending cost:**

1. **Cheapest, and it composes with what exists.** Add a `declared_keys: &'static [&'static str]`
   column to `text_json_parity_axis::REGISTRY` (47 rows, already bijected against the clap
   tree) and one assertion inside each `FENCES` check plus each judgment proof:
   `assert_eq!(envelope.keys().collect(), declared_keys)`. Cost is 47 hand-written key
   lists — but the bijection already forces a new verb to appear, so the list cannot go
   stale silently. Blind spot it does **not** close: the 19 judgment-tier verbs build no
   witness, so their assertion would need a driven envelope rather than a rendered one.
2. **Behaviourally complete.** Put the assertion in
   `format_json_success_axis::every_leaf_verb_honours_format_json_on_its_success_path` —
   it already drives all 47 to a **real** success through the real binary, so it sees
   renderer-added keys, dry-run/landed arm splits and array envelopes that a rendered
   witness cannot. It needs one `Recipe` field: `keys: &'static [&'static str]`. This is
   the smallest change that makes *"no undeclared key on any pinned envelope"* checkable
   over the whole axis, and it catches EC-3, F-new-1, F-new-2 and F-new-3 in one pass.
   (`task finalize --dry-run` and `milestone finalize`'s two arms would need to be rows,
   so the axis becomes verb×arm, not verb.)
3. **Doc-side half, which neither buys.** Both above fence the *code* against a list. They
   do not fence the list against `design/`. Nothing today reads a design doc for an
   envelope key set; the nearest precedent is `doctype_map_versions.rs`, which checks
   `doctype-map.md:42` against both manifests. An equivalent would have to parse the key
   bullets out of `command-output-contract.md`, and there is no machine-readable form of
   them today.

---

## 3 · `ConfigAck` and the `config set` relocation (EC-4)

*status: **shape-limited** — the fence is sound over its subject and structurally cannot
see the effect · driven*

Driven, on a corpus with one committed `adr`:

```
$ jigc --format json config set docs-root documents
STDERR: relocating 1 committed doc(s) stranded by the `docs-root` re-point to `documents`
        (every committed doc under the prior resolved root, managed or not; each move is a
         staged `git mv` — commit it with your next commit):
          - docs/decisions/cache-strategy.md → documents/decisions/cache-strategy.md
STDOUT: { "committed": false, "key": "docs-root", "op": "config-set", "value": "documents" }
exit 0
$ git status --porcelain
R  docs/decisions/cache-strategy.md -> documents/decisions/cache-strategy.md
```

- **Where the moves are computed and narrated:** `crates/cli/src/config.rs:751`
  `route_docs_root_repoint_orphans` — the stranded set from `orphan::docs_root_would_orphan`,
  each move through the shared primitive `relocate::move_doc` (`git mv` + file-state re-key),
  narrated by two `eprintln!`s (`:781` the header, `:800-804` one line per move). Its
  `placement-root` sibling is `:846`, same shape.
- **Why they are not fields of the ack:** the function returns `()`. It is called from
  `run_set` (`config.rs:317`) purely for effect; `run_set` returns
  `ConfigAck::Set { key: String, value: String }` (`render.rs:2541`) and nothing else.
- **Why the M48 fence cannot see it:** `config_ack_parity`
  (`text_json_parity_axis.rs:1314`) destructures `ConfigAck` **exhaustively** — which is
  the strongest form of the parity rule — over exactly `{key, value}`. An exhaustive
  destructure of an enum that does not carry the fact proves nothing about the fact.
  `committed` is likewise renderer-added (`render.rs:2477`), i.e. already outside the enum,
  which shows the seam was crossed once before without the fence noticing.
- **What a value-carrying ack would need:** `route_docs_root_repoint_orphans` returns
  `Vec<Relocation{from, to}>` (plus the per-move failures it currently prints as *"move it
  by hand"*); `ConfigAck::Set` gains a `relocated` field; the exhaustive destructure then
  forces the key. **One design question the mechanism does not answer:** both root knobs
  relocate, and `config set` also stages an *index* change — so the honest key set is
  arguably `{relocated, staged}`, and the parity rule (*a value the text prints that the
  envelope withholds*) does not by itself decide whether the index change is a value or a
  side effect. EC-4's axis (*acks whose side effects are not fields of the acked value*)
  is wider than `ConfigAck`: `uninstall` and `setup` are the same shape (both narrate
  filesystem effects through renderer-added keys).

---

## 4 · The invocation-log record (EC-6)

*status: **shape-limited** (partially fenced) · driven + source-read — EC-6's source-read
grade reproduces exactly*

- **Writer:** `crates/cli/src/invocation_log.rs:398` `append_record` → `struct Record`
  (`:381`), serialized with `serde_json::to_string` and appended to
  `<jigc_home>/.jigc/logs/invocations.jsonl`.
- **Fields (8):** `timestamp`, `argv`, `exit_code`, `duration_ms`, `finding_codes`,
  `output_bytes`, `binary_version`, `error_code`.
- **Driven** (knob on, `jigc describe`):
  `{"timestamp":"2026-09-10T19:47:40Z","argv":["describe"],"exit_code":0,"duration_ms":17,
  "finding_codes":[],"output_bytes":27949,"binary_version":"1.0.0-rc.14","error_code":null}`
- **What `crates/cli/tests/invocation_log.rs` asserts.** 12 tests. `:128`
  `knob_on_appends_one_well_formed_record` asserts **presence/type only** —
  `rec["timestamp"].as_str().is_some()`, `rec["exit_code"].as_u64() == Some(0)`,
  `rec["duration_ms"].as_u64().is_some()`, `rec["finding_codes"].as_array().is_some()`.
  `argv` is used as a *selector* (`record_with_arg`) and never asserted. Four fields are
  value-fenced individually: `:236` `binary_version == CARGO_PKG_VERSION`; `:259`
  `output_bytes == stdout.len() + stderr.len()`; `:327` `error_code ==
  Some("finalize.commit-rejected")` vs `null`; `:710` the milestone sibling.
  `grep "as_object\|keys()"` over the file → **no key-set equality anywhere**. An added key
  is unfenced; a **removed** `argv` or `output_bytes` would also survive `:128`.
- **Version integer:** none. `grep -rn "log_version\|LOG_VERSION\|record_version"` over
  `crates/cli/src` and `design/measurement.md` → **zero hits**, while the `Record`
  doc-comment itself (`:378-380`) calls it *"an independently-versioned surface."*
  `binary_version` attributes a record to a binary; it does not version the *format*.
- **Related deferral:** `decisions-pending.md:455` — a first-class `task_id` field, DEFERRED
  to 1.1 at M44 planning, on the stated ground that the log *is* additively evolvable
  (*"purely additive with no one-way door"*). That ground and *"independently-versioned"*
  are the same claim, and the absence of an integer is what would make it false.

---

## 5 · The deny-floor fence (EC-7)

*status: **claim-vs-reality correction** — EC-7 is right about `e2e_audit.rs` and wrong as
a claim about the tree · source-read at HEAD*

- **The self-reference is real.** `crates/cli/tests/e2e_audit.rs:261` `floor_patterns()`
  does `include_str!("../adapters/claude-code.yaml")` and scrapes the `deny:` block; `:290`
  then asserts every scraped pattern reaches `.claude/settings.json`. Deleting a pattern
  from the profile deletes it from **both** sides. That test alone would stay green.
- **But two independent literal sets exist, in the crate's own source.**
  `crates/cli/src/adapter.rs` carries the whole floor **twice as inline `insta` literals**:
  `claude_code_profile_bytes_are_canonical` (the profile bytes, `:1495-1547`, the 22
  patterns at `:1509-1536`) and `deny_floor_added_then_idempotent` (`:2622`), whose inline
  snapshot at `:2660-2686` is the merged `settings.json` listing **all 22 patterns
  verbatim**. Deleting `Read(./**/*.key)` from the profile reddens both.
- **Live evidence that the coupling is genuine, not nominal:** the gitignored residue
  `crates/cli/src/.adapter.rs.pending-snap` records a real pending diff of
  `claude_code_profile_bytes_are_canonical` whose `old` snapshot lacks the `guide:` block
  the profile now carries — i.e. a past profile edit **did** redden that snapshot and
  required an explicit accept.
- **The residual gap is real but narrower than EC-7 states:** an inline `insta` snapshot is
  re-acceptable by `cargo insta accept`, which rewrites the *test source* to match the new
  profile. So the floor is fenced against an **accidental** drop and not against a
  **deliberate regeneration** — which is the same posture as the 634 compose goldens.
  Only the two human-owned destroyers are pinned by a *reasoned* literal:
  `adapter.rs:1569` `the_deny_floor_carries_the_two_human_owned_destroyers` asserts
  `Bash(jigc uninstall:*)` and `Bash(jigc milestone discard:*)` against the **loaded**
  profile, with a negative arm keeping `milestone provision` off, and
  `flow51_acceptance.rs:1362` drives it through real `setup`/`uninstall`.
- **What an independent literal set would look like, and where the profile loads.** The
  profile loads via `cli::adapter::load_profile("claude-code")` → `profile.allowlist.deny`
  (`Vec<String>`). An independent floor would be a `const SECRETS_FLOOR: &[&str]` in
  `adapter.rs` (or a test-side module), asserted `⊆ load_profile(...).allowlist.deny` with
  a `len()` floor — the `ConfigAck::ALL` / `STORE_EXIT_FLIPS` / `COMMITTING_DOORS` mold this
  repo already uses, and the shape `adapter.rs:1569` uses for the two destroyers. The wider
  axis EC-7 names (*every test whose expectation is derived from its own subject*) is
  worth a `grep -rn include_str! crates/cli/tests/` sweep at Settle; I did not run it.

---

## 6 · `GATE_COVERAGE` / `Tier::Previewed` (EC-20)

*status: **latent defect**, and the reported framing needs one correction · driven*

**Driven, one task, one moment** (`single-task`, adr filled, commit doc authored, changelog
untouched — so `changelog-recording.gate-granted-unused` is live):

| door | envelope keys | findings |
|---|---|---|
| `jigc --format json task validate <id>` | `{schema_version, findings}` | `file-state.staged-copy`, `changelog-recording.gate-granted-unused` — exit 0 |
| `jigc --format json task finalize <id> --dry-run` | `{dry_run, left_out, manifest, subject}` | **no `findings` key at all** — exit 0 |
| `jigc --format json task finalize <id>` (landed) | `{committed, findings, schema_version}` | `file-state.staged-copy`, `changelog-recording.gate-granted-unused` — exit 0 |

**Correction to EC-20 as written.** The contract sentence
(`command-output-contract.md`, the exit-code taxonomy's third clause: *"every check
`task validate` runs … is the same check `finalize` runs, at the same severity"*) is
**not** falsified by the committing door — driven, `task validate` and the landed
`task finalize` emit the *identical* two findings. What diverges is
**`--dry-run`, the forecast arm**, and it drops **both** advisories, not only the
changelog one: its envelope has no findings channel whatsoever. QUICKSTART Q2 presents
`task validate` and `finalize --dry-run` as one preview surface, and that is where the
law-1 problem lives.

- **Where the advisory tier is dropped:** `crates/cli/src/task.rs:1827` — the `if dry_run`
  branch. The full `report` (with the changelog advisory merged in at `:1677`) is computed
  **above** the branch, then the branch returns only
  `render::finalize_manifest(format, plan.subject(), &included, &left_out)`. The only
  findings that survive it are the **blocking** carryover set, via `self.blocked(...)`
  three lines earlier. A value computed and discarded — the parity rule's own definition of
  a gap, on the arm M50 already spent one close on (`subject`).
- **Why `GATE_COVERAGE` cannot see it.** `crates/cli/src/gate_coverage.rs:43-49` states it
  in its own words: *"The **membership** of `Tier::Previewed` is owned elsewhere — the
  checks `TaskArea::preview_gates` actually runs."* The table (11 rows, 4 at
  `Tier::Previewed`: `content-findings`, `carryover`, `owner-artifact-unstaged`,
  `changelog-gate`) is a **named-fact token guard over prose surfaces**:
  `gate_coverage_fence.rs:265` asserts each of eight enumerating sites contains the
  `token` of every member of the tiers it subscribes to; `:356` asserts the composed
  `what's-left:` line renders the generated fragment verbatim. **No test compares the
  finding sets two doors emit.** So the registry EC-20 points at is a fence over what
  surfaces *say*, and the un-swept cell is what a door *emits*.
- **Minimal fence:** one test that drives a state making each `Tier::Previewed` member
  fire and asserts the emitted `findings[].code` set is equal across
  `task validate` / `finalize --dry-run` / landed `finalize` — i.e. iterate the registry
  behaviourally rather than lexically. It needs `--dry-run` to gain a `findings` key,
  which is an **additive key on a pinned envelope** and therefore inside the closing window.

---

## 7 · The version-bump machinery (EC-10)

*status: **deferred-by-design → now prose-only**; EC-10 reproduces exactly · source-read*

**What exists (mechanical):**
- One string to edit: `Cargo.toml:10` `[workspace.package] version`; both crates inherit;
  every runtime reader is `env!("CARGO_PKG_VERSION")`.
- **Ten version-bearing goldens**, verified by `grep -rl "rc\.14" crates/cli/tests/goldens/`
  → exactly 10 files, all under `goldens/compose/composite/`: `start-orient--*.txt` ×5 and
  `start-orient-json--*.txt` ×5 (one header line each,
  `Pack: dev/1.0.0-rc.14 | methodology/1.0.0-rc.14 · …`). **These redden on a bump without
  regeneration — the only mechanical forcing function in the procedure, and it forces the
  regeneration, never the bump.**
- `crates/cli/tests/release_smoke.rs:100` `version_reports_the_workspace_version` asserts
  `jigc --version == format!("jigc {}\n", env!("CARGO_PKG_VERSION"))` — **derived from the
  same constant**, so it is green at any version and cannot catch a stale bump.

**What is prose:**
- `implementation/milestone-completion-workflow.md:24`: *"When the wave carries an rc name,
  its close bumps the workspace version to rc.N, regenerates any version-bearing goldens,
  re-runs the gate, and installs the release binary… This was convention, not rule, and it
  broke exactly once proving the need."*
- `packs/methodology/workflows/completion.yaml` — 8 includes
  (`audit`, `triage`, `fix-gate`, `re-verify`, `author-completion-record`,
  `author-decisions`, `author-commit`, `finalize`). `grep -rn "version\|bump\|golden"` over
  it → **zero hits**; over `packs/methodology/steps/*.yaml` for a bump step → zero.
  **No pack step mints the bump.**
- `crates/cli/tests/foldback_truth.rs:223` declines the assertion **by name**: *"The owed
  version bump is deliberately outside this fence… it carries no numeral: the roadmap's M50
  section names no target version and the 1.0.0 call is the human's, so asserting a version
  string would be this fence choosing it."* That rationale is sound for the *numeral* and
  does not cover the weaker checkable claim — *the version this fold-back names is the
  version `Cargo.toml` carries* — which is what has been missed five times.
- Same file `:229-238` records the fifth miss in its own words: the M50 audit rewrote
  CLAUDE.md at `95c79be6` without touching the fence, which went red and *stayed* red
  through two commits while the handover recorded a green gate.

**Additional stale-on-bump surface** (beyond the ten goldens): `Cargo.lock` ×2 entries
(stale unless a cargo command runs before push) and
`completions/trial-harness/verify-pair.sh:22` (a default probe-set comment,
`rc.13 -> rc.14`).

---

## 8 · Known deferred / latent in this area (`decisions-pending.md` at HEAD)

| entry | line | status |
|---|---|---|
| **M51's own boundary** — *"the wave takes every row of the evidence-check ledger (EC-1 … EC-43)"*; **Tier 1 = EC-3 · EC-4 · EC-5 · EC-6 · EC-7 · EC-8 · EC-9 · EC-10** as *cheap now, expensive after the pin* | `:28`, `:35` | chartered — this baseline is scoped to Tier 1's contract half |
| **The flattened `{"error": …}` envelope.** An operational-funnel refusal reaches `--format json` as one prose string — no `code`, no `key`, no `findings` array — through the shared `render::finding_error`. *Trigger:* a driver needs to key on `(code, target)` at one of those doors. Priced as *contract evolution*, i.e. with the `route` tagged-union promotion | `:605` | **deferred-by-design, with a trigger.** Fenced positively as a *single-key* envelope (`finalize_outcome_surface.rs:485/560/635`), so today's shape is pinned, not accidental. M51 tier 1 lists F-5/F-11 as *code and route inside the flattened string* — the string's contents, explicitly **not** the envelope shape, which is fork 5 |
| **(Q) `schema-version` a number on one pinned contract, a string on the other** — taken *additively* at M49 Inc 8; `doc-read-surface.md` records it *"recorded, not repaired"* (*"a type change is not an additive key, so the pre-1.0 window does not cover it"*) | `:74` | **latent, live at HEAD.** Driven: `doc show adr:… --format json` → top-level `"schema-version": 2` (**number**) and `fields["schema-version"]: "2"` (**string**) in one document. `doc_show.rs:1001` pins the discrimination deliberately. After the pin, reconciling costs a versioned extension |
| **`task_id` on the invocation-log `Record`** — DEFERRED to 1.1 at M44, on the ground that the log is *"an independently-versioned, rebuildable surface… purely additive with no one-way door"* | `:455` | **deferred**, and its premise is exactly what EC-6 puts in question: there is no version integer to be independent by |
| **Additive-window posture, both homes** — `doc-read-surface.md:88-92` and `command-output-contract.md:446`, *"The window closes here — the discharge (M48)"*, re-keyed to the pin, with M49/M46/M50 spends declared *below* the close heading | — | **latent (record)** — I-6 in the audit; the two headings still name M48 while four later spends sit under them |
| **The methodology-pack manifest / doctype counts** (`corpus-migration.md:281` ten vs eleven; both manifest headers *"all 16 doctype hashes"* against 17 entries) | — | **latent (record)**, fenced nowhere except `doctype_map_versions.rs`, which covers only `doctype-map.md:42` |

---

## What I did **not** reach (so it is not cleared)

- **No mutation was applied.** Every *fence-quality* grade above is from reading the test
  body plus driving the binary; I did not delete a key and watch a suite redden. EC-7's
  correction in particular deserves one applied mutation before it is acted on.
- **No cargo run.** I did not execute `text_json_parity_axis`, `format_json_success_axis`
  or `invocation_log` — their content is read at HEAD, and the gate result at this sha is
  the orchestrating session's (`gate-rc14-at-bd348a83.log`, 3341/0).
- **`describe --format json`'s per-entry keys** (`router_hidden`, `origin_pack`, `pack`)
  were not enumerated per definition — only the top-level three.
- **The reject-surface key sets** of the 47 verbs were not swept (I drove the success
  path); `machine_output.rs` covers stream discipline there but not keys, so the
  `{"error"}` single-key shape is pinned at 3 doors while `render::finding_error`
  (`render.rs:4438`) has **16** call sites across the CLI.
- **The `grep -rn include_str! crates/cli/tests/`** sweep for EC-7's wider axis
  (*self-referential fences*) was not run.
