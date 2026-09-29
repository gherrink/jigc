<!-- The unseeded Codex source pass for AXIS 5, verbatim. Source read at `1.0.0-rc.20` (repo HEAD `4d3175c3`), 2026-09-27. It drove nothing. -->

# Axis 5 source pass — pinned contracts

Read-only source review only; I did not drive the binary or modify files.

## Claims

1. **The pinned orientation rows still declare `next_steps`, although serialization omits that key for a reachable empty set.**

   - Evidence: `Clean` and `ActiveTask` declare `next_steps` as required envelope keys at `crates/cli/src/render.rs:6584-6617`; both fields use `skip_serializing_if = "Vec::is_empty"` at `crates/engine/src/result.rs:138-145,172-175`; the producer can construct the empty set when the composed pack omits every `OFF_CATALOG_VERBS` member at `crates/cli/src/orient.rs:116-141`.
   - Proposed reproduction: compose a pack containing no off-catalog workflows, then run `jigc --format json start`, once clean and once with an active task. Expected wrong behavior: exit 0 and a document lacking the declared `next_steps` key.
   - Confidence: **High**.

2. **`start --explain` remains a production JSON-success envelope with no `ENVELOPE_ARMS` row.**

   - Evidence: dispatch prints `render::explain` at `crates/cli/src/cli.rs:1732-1749`; its JSON arm serializes `ResolutionTree` directly at `crates/cli/src/render.rs:711-728`; the complete registry at `crates/cli/src/render.rs:6572-7375` contains four `start` rows—three orientation variants and `Composed`—but no explain/`ResolutionTree` row.
   - Proposed reproduction: `jigc --format json start --explain --workflow single-task`. Expected wrong behavior: exit 0 with the resolution-tree object on stdout, but no production registry row declares its shape or keys.
   - Confidence: **High**.

No new F-10-specific completeness defect was found.

## rc.20 baseline rows, source disposition

The seven rc.20 rows remain:

1. `(5, DEFECT 1)` fabricated milestone identity — **CLOSED**. This source pass found no bypass of the existing unsluggable-title guard.
2. `(5, DEFECT 2)` resolved included step rejected by config mutation — **STILL-OPEN**, expected tier 3 / 1.x.
3. `(5, DEFECT 3)` malformed address has no finding code — **STILL-OPEN**, expected tier 3 / 1.x. Both paths still construct plain `anyhow` errors: `crates/cli/src/rename.rs:1193-1207` and `crates/cli/src/doc.rs:6511-6518`.
4. `(5, DEFECT 4)` absent optional leaf becomes `store.no-such-leaf` — **STILL-OPEN**, expected tier 3 / 1.x; that code remains envelope-owed at `crates/cli/src/render.rs:5957-5962`.
5. `(5, C1)` optional `next_steps` contradicts the pinned key set — **STILL-OPEN**, Claim 1.
6. `(5, D1)` undeclared `start --explain` JSON arm — **STILL-OPEN**, Claim 2.
7. rc.17 `DEFECT A`, `doc show` uses the full address as the `store.unknown-type` target — **STILL-OPEN**, expected tier 3 / 1.x.

Thus: **1 CLOSED, 6 STILL-OPEN**, matching the expected tier-3 deferrals.

## M51 confirmed-row source status

- Pre-dispatch `current_dir()` bypass — **CLOSED**. `cwd_or_refusal(format)` is used before posture and dispatch at `crates/cli/src/cli.rs:584-599,1660-1767`; `PRE_DISPATCH_FAULTS` asserts one `current_dir` reader, 24 seam callers, and no unanswered cell at `crates/cli/tests/pre_dispatch_faults.rs:139-180`.
- Setup/uninstall’s third bare-`Finding` reject shape — **CLOSED**. The registry declares only `Reject::Error` and `Reject::Findings` at `crates/cli/src/render.rs:7320-7374`, and the shared carrier selects the structured arm at `crates/cli/src/render.rs:6003-6016`.
- Missing `doc show` array/data-keyed projections — **CLOSED**. Eight projection rows are present at `crates/cli/src/render.rs:6521-6528,6572-7375`.
- Success rows falsely promising invariant exit 0 — **CLOSED as a prose claim**: the registry’s status/outcome model no longer states that every success-shaped document always exits 0.
- Four pre-pin deletes — **CLOSED**: setup lacks `installed`, uninstall lacks `uninstalled`, migration review lacks `review`, and milestone list-tasks lacks `hook_output`.
- Unpinned-shape classification — **CLOSED**: only `describe` and milestone text acknowledgements are explicitly unpinned, each with a reason.

## F-10 and completeness census

The registry now contains **66 arms over 48 clap leaves**, not the standing brief’s pre-F-10 “60 over 47” figure. Relative to rc.20’s actual 64/47 census, the two additions are correctly represented:

- `task amend / Composed`, keys `{task,text}`: `crates/cli/src/render.rs:7018-7030`.
- `task finalize / LandedAmend`, top-level `{committed,findings,schema_version}`: `crates/cli/src/render.rs:7063-7080`.
- The additive nested `committed.amended` value is populated from the pinned SHA at `crates/cli/src/task.rs:3498-3513`; `finalize_landed` inserts the complete landed object at `crates/cli/src/render.rs:2868-2878`.
- The two new blocking codes and rejected-commit code are declared at `crates/cli/src/render.rs:1534-1554`; the amend index/doc guards are reached before execution at `crates/cli/src/task.rs:2967-2975`.
- All four `store.*`/fixed-identity owed codes are centrally forced through the findings envelope at `crates/cli/src/render.rs:5957-6016`.

The four proof fences remain: leaf coverage, production-arm equality, registry/recipe equality, and driven keys/stream/exit equality at `crates/cli/tests/format_json_success_axis.rs:1272-1306,1308-1437,1440-1463,1465-1575`.

## Serialization census and bounds

All command-stdout JSON paths were reviewed. Registered families cover orientation/composition; setup/uninstall; upgrade, ingest, migrate, migrate-corpus, unmanage, rename and relocate; validate; all `DocAck`, `TaskAck`, and `ConfigAck` variants; doc show/schema/list; task list/diff/validate/finalize; milestone acknowledgements/list/join/finalize; migration review; describe; and both reject funnels. The sole uncovered stdout serialization is Claim 2.

Non-command serialization hits are not envelope surfaces: generated setup front matter (`setup.rs:204`), adapter settings files (`adapter.rs:928,1078`), invocation JSONL (`invocation_log.rs:549`), and nested doc-show value construction (`doc.rs:5525-5532`).

The schema manifest SHA-256 is identical at `609da011` and the reviewed tree (`e15d332b…f0a`): **zero schema-hash movement was not violated**.

Bounds: source structure only; no binary behavior, tier-2/3 repro, release-only behavior, hostile cwd, or hook execution was independently driven. The worktree was already dirty; nothing was changed.