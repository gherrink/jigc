# jigc CLI output-surface inventory — rule-conformance audit checklist

Enumerated from code at HEAD `12bf692` (the corpora predate this by 46 commits — routes/spans changed; codes re-derived from source, not captures). Corpora matched: r1 = `cli-surface-corpus.md`, r2a = `cli-surface-corpus-r2a.md`, r2b = `cli-surface-corpus-r2b.md`. Match is loose (command + distinctive text); `none` = no capture found.

---

# PART 1 — VERB OUTCOME SURFACES

Notation: `formats: agent+json (human=agent)` means the site renders `json` distinctly and `agent`/`human` identically. **Load-bearing format finding:** `Format::Human` has NO custom rendering anywhere in `render.rs` — every match arm pairs `Format::Agent | Format::Human`, so `human` is byte-identical to `agent` across the entire surface; only `json` diverges. `doc show` / `doc schema` are the two verbs where `json` is a separately-pinned contract (not a mere projection). `task diff` and the `doc show` stale-read hint ignore `--format` entirely (plain text). `task finalize` / `task validate` route their `json` block-report to **stdout** while `agent`/`human` go to **stderr** (deliberate stream asymmetry, `task.rs:769-776`). Locate-preamble errors ("cannot determine the current directory") are one shared stderr row folded at the end.

## Part 1A — compose / task / doc core verbs

### jigc start (bare / orient)
- `start+orient-clean · render::orientation → orientation_clean crates/cli/src/render.rs:95 (print cli.rs:1188) · formats: agent+json (human=agent) · corpus: r1`
- `start+orient-unset-project · render::orientation → orientation_unset render.rs:2113 (print cli.rs:1188) · formats: agent+json (human=agent) · corpus: none`
- `start+orient-operational-error · orient::orient err → render::operational_error cli.rs:1192 · formats: agent+json (human=agent) · corpus: none`

### jigc start "&lt;intent&gt;" (run_compose — cascade default / router)
- `start+compose-success · render::composed render.rs:149 (print cli.rs:1026) · formats: agent+json (human=agent) · corpus: r1` (router route-and-mint-nothing r1:598; work-minting via run_compose_named r1:765)
- `start+compose-blocked-or-error · start::compose_in_repo err → render::operational_error cli.rs:1030 · formats: agent+json (human=agent) · corpus: none` (mint collision, empty-intent bail start.rs:85, --slug malformed bail start.rs:94)

### jigc start --workflow &lt;X&gt; "&lt;intent&gt;" (run_compose_named)
- `start-named+compose-success · render::composed render.rs:149 (print cli.rs:1054) · formats: agent+json (human=agent) · corpus: r1,r2b`
- `start-named+unknown-workflow-or-blocked · compose_named_in_repo err → render::operational_error cli.rs:1058 · formats: agent+json (human=agent) · corpus: none` (unknown X → workflow-refs.unknown-workflow block, start.rs:2932)

### jigc start --workflow &lt;X&gt; (no intent — run_compose_named_no_intent)
- `start-named-nointent+compose-success · render::composed render.rs:149 (print cli.rs:1082) · formats: agent+json (human=agent) · corpus: r2b` (ingest-existing/planning/completion/project-setup)
- `start-named-nointent+creates-task-true-rejected · bespoke bail! "workflow '&lt;X&gt;' requires an intent" start.rs:696 → operational_error cli.rs:1086 · formats: agent+json (human=agent) · corpus: none`
- `start-named-nointent+unknown-or-blocked · compose_named_no_intent_in_repo err → operational_error cli.rs:1086 · formats: agent+json (human=agent) · corpus: none`

### jigc start --explain (run_explain)
- `start-explain+tree · render::explain render.rs:253 (print cli.rs:1110) · formats: agent+json (human=agent) · corpus: r2b` (r2b:808)
- `start-explain+blocked-or-error · compose_explain_in_repo err → operational_error cli.rs:1114 · formats: agent+json (human=agent) · corpus: none`

### jigc start --task &lt;id&gt; (run_resume)
- `start-resume+compose-success · render::composed render.rs:149 (print cli.rs:1136) · formats: agent+json (human=agent) · corpus: r2b` (r2b:977, minted_header suppressed on resume)
- `start-resume+no-such-task · resume_in_repo → task::no_such_task task.rs:477 → operational_error cli.rs:1140 · formats: agent+json (human=agent) · corpus: r1` (loose)
- `start-resume+base-pin-mismatch · bespoke bail! "task pinned to base …" start.rs:1593 → operational_error · formats: agent+json (human=agent) · corpus: none`
- `start-resume+no-recorded-workflow · bespoke with_context start.rs:1609 → operational_error · formats: agent+json (human=agent) · corpus: none`

### jigc workflow &lt;W&gt; --task &lt;id&gt; (run_reenter)
- `workflow+compose-success · render::composed render.rs:149 (print cli.rs:1164) · formats: agent+json (human=agent) · corpus: r2a` (r2a:77 sub-task re-entry)
- `workflow+no-such-subtask · bespoke bail! start.rs:1656 → operational_error cli.rs:1168 · formats: agent+json (human=agent) · corpus: none`
- `workflow+base-pin-mismatch · bespoke bail! start.rs:1670 → operational_error · formats: agent+json (human=agent) · corpus: none`
- `workflow+workflow-mismatch · bespoke Finding::block "workflow-refs.workflow-mismatch" start.rs:1706 → operational_error · formats: agent+json (human=agent) · corpus: none`
- `workflow+no-recorded-workflow · bespoke with_context start.rs:1687 → operational_error · formats: agent+json (human=agent) · corpus: none`

### jigc doc create
- `doc-create+success-minted · render::doc_ack DocAck::Created render.rs:1143 (print doc.rs:1654) · formats: agent+json (human=agent) · corpus: r1` (r1:1216)
- `doc-create+success-copied-in · same site, existed:true "(already existed — copied in for update)" render.rs:1147 · formats: agent+json (human=agent) · corpus: none`
- `doc-create+machine-maintained-refusal · machine_maintained_guard Finding "write.machine-maintained" doc.rs:395 (call doc.rs:1615) → DocFailure::Block doc.rs:357 · formats: agent+json (human=agent) · corpus: none`
- `doc-create+invalid-slug · bespoke Orchestration anyhow! "--slug … is not a valid slug" doc.rs:1619 · formats: agent+json (human=agent) · corpus: none`
- `doc-create+blocked-findings-envelope · state::create_gated → block(…,"create") doc.rs:1645 · formats: agent+json (human=agent) · corpus: none`
- `doc-create+active-task-resolution-error · ActiveTask::resolve doc.rs:3100/3116 → Orchestration · formats: agent+json (human=agent) · corpus: none`

### jigc doc add-item
- `doc-add-item+success · render::doc_ack DocAck::AddedItem render.rs:1148 (print doc.rs:854) · formats: agent+json (human=agent) · corpus: none`
- `doc-add-item+machine-maintained-refusal · machine_maintained_guard doc.rs:824 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-add-item+blocked-findings-envelope · apply_add_item_target → block(…) doc.rs:836 (non-repeatable/unknown section, enum-membership) · formats: agent+json (human=agent) · corpus: none`
- `doc-add-item+no-section-addressed · with_context doc.rs:827 → Orchestration · formats: agent+json (human=agent) · corpus: none`
- `doc-add-item+no-staged-instance · read_or_copy_in→read_staged doc.rs:3390 → Orchestration · formats: agent+json (human=agent) · corpus: none`

### jigc doc remove-item
- `doc-remove-item+success · render::doc_ack DocAck::RemovedItem render.rs:1134 (print doc.rs:1139) · formats: agent+json (human=agent) · corpus: none`
- `doc-remove-item+machine-maintained-refusal · machine_maintained_guard doc.rs:1096 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-remove-item+blocked-findings-envelope · block(…,"remove-item") doc.rs:1109/1119 · formats: agent+json (human=agent) · corpus: none`
- `doc-remove-item+no-item-addressed · with_context doc.rs:1099 → Orchestration · formats: agent+json (human=agent) · corpus: none`

### jigc doc retitle-item
- `doc-retitle-item+success · render::doc_ack DocAck::RetitledItem "retitled … (anchor frozen)" render.rs:1135 (print doc.rs:1284) · formats: agent+json (human=agent) · corpus: none`
- `doc-retitle-item+milestone-record-refusal · bespoke Finding "write.machine-maintained" (item-level) doc.rs:1232 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-retitle-item+enum-id-identity-change · retitle_enum_refusal Finding "write.identity-change" doc.rs:1356 (call doc.rs:1251) → block envelope · formats: agent+json (human=agent) · corpus: r2a` (r2a:346)
- `doc-retitle-item+blocked-findings-envelope · block(…,"retitle-item") doc.rs:1267 · formats: agent+json (human=agent) · corpus: none`
- `doc-retitle-item+no-item-addressed · with_context doc.rs:1216 → Orchestration · formats: agent+json (human=agent) · corpus: none`

### jigc doc set-field
- `doc-set-field+success · render::doc_ack DocAck::Field "set &lt;addr&gt; = &lt;value&gt;" render.rs:1129 (print doc.rs:455) · formats: agent+json (human=agent) · corpus: r1,r2a` (r1:1222; r2a:154)
- `doc-set-field+machine-maintained-refusal · machine_maintained_guard doc.rs:429 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-set-field+id-from-guard-refusal · id_from_field_guard Finding doc.rs:626 (call doc.rs:486) → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-set-field+blocked-findings-envelope · apply_field_target → block(…,"set-field") doc.rs:497/503 · formats: agent+json (human=agent) · corpus: r1,r2a` (r1:1228 write.malformed-value; r2a:236 write.list-overwrite)
- `doc-set-field+empty-value-repoint · repoint_empty_value bespoke route "--value \"\" → --unset" doc.rs:600 (call doc.rs:445) → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-set-field+no-field-addressed · with_context doc.rs:432 → Orchestration · formats: agent+json (human=agent) · corpus: none`

### jigc doc set-field --unset (run_unset_field)
- `doc-unset-field+success · render::doc_ack DocAck::UnsetField "unset &lt;addr&gt;" render.rs:1132 (print doc.rs:547) · formats: agent+json (human=agent) · corpus: none`
- `doc-unset-field+machine-maintained-refusal · machine_maintained_guard doc.rs:529 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-unset-field+blocked-findings-envelope · apply_unset_target → block doc.rs:572 · formats: agent+json (human=agent) · corpus: none`

### jigc doc set-slot
- `doc-set-slot+success · render::doc_ack DocAck::Slot "set slot &lt;addr&gt; (&lt;n&gt; chars)" render.rs:1133 (print doc.rs:759) · formats: agent+json (human=agent) · corpus: r1,r2a` (r1:1238; r2a:160)
- `doc-set-slot+machine-maintained-refusal · machine_maintained_guard doc.rs:736 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-set-slot+blocked-findings-envelope · apply_slot_target → block(…,"set-slot") doc.rs:786/792 · formats: agent+json (human=agent) · corpus: none`
- `doc-set-slot+no-slot-addressed · with_context doc.rs:739 → Orchestration · formats: agent+json (human=agent) · corpus: none`
- `doc-set-slot+handoff-read-error · read_handoff doc.rs:3419 → Orchestration · formats: agent+json (human=agent) · corpus: none`

### jigc doc author
- `doc-author+success · render::doc_ack DocAck::Authored render.rs:1150 (print doc.rs:1760) · formats: agent+json (human=agent) · corpus: r1,r2b` (r1:1112; r2b:828)
- `doc-author+machine-maintained-refusal · machine_maintained_guard doc.rs:1689 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-author+payload-parse-reject · cli::author::parse_author_payload doc.rs:1700 → DocFailure · formats: agent+json (human=agent) · corpus: none`
- `doc-author+create-gate-blocked · block(…,"author") doc.rs:1732 · formats: agent+json (human=agent) · corpus: none`
- `doc-author+mid-chain-leaf-block-rollback · apply_leaf err (staged file removed) doc.rs:1749 → block/Orchestration · formats: agent+json (human=agent) · corpus: none`

### jigc doc show (run_show / run_show_staged)
- `doc-show+committed-success-json · show_json doc.rs:2537 (print doc.rs:1850) · formats: json custom (pinned contract) · corpus: none`
- `doc-show+committed-success-text · engine::store::read_slice doc.rs:1845 (print doc.rs:1850) · formats: agent/human · corpus: none`
- `doc-show+stale-read-hint (stderr side-channel) · stale_read_hint eprintln doc.rs:1899 · formats: format-agnostic plain (always stderr) · corpus: none`
- `doc-show+not-found · read_slice/show_json store.not-found → reroute_unadopted doc.rs:1854 → block envelope · formats: agent+json (human=agent) · corpus: r1` (r1:1274)
- `doc-show+unknown-type · parse_verb_addr / store.unknown-type doc.rs:1837 → block envelope · formats: agent+json (human=agent) · corpus: r1` (r1:1284)
- `doc-show+unadopted-foreign-reroute · reroute_unadopted doc.rs:1970 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-show+staged-success (--task) · show_json (staged marker) doc.rs:1935 / read_slice_staged doc.rs:1943 (print doc.rs:1947) · formats: json custom + agent/human text · corpus: none`
- `doc-show+staged-unparseable-block (--task) · read_slice_staged/show_json err → DocFailure::block doc.rs:1944 · formats: agent+json (human=agent) · corpus: none`

### jigc doc schema
- `doc-schema+success-json · schema_contract (contract-version 3) doc.rs:2060 · formats: json custom (separately versioned) · corpus: r1,r2b` (r1:726; r2b full doctype set)
- `doc-schema+success-text · schema_listing doc.rs:2061 · formats: agent/human · corpus: r1,r2b`
- `doc-schema+unknown-type · Finding "store.unknown-type" doc.rs:2045 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-schema+not-set-up · require_project_layer doc.rs:2041 → Orchestration · formats: agent+json (human=agent) · corpus: none`

### jigc doc list
- `doc-list+populated-json · render::json DocListing doc.rs:2131 · formats: json custom · corpus: none`
- `doc-list+populated-text · row loop "id  path  state" doc.rs:2145 · formats: agent/human · corpus: none`
- `doc-list+empty-set · bespoke "jigc doc list — no committed […] docs" doc.rs:2140-2141 · formats: agent+json (human=agent) · corpus: r1` (r1:762)
- `doc-list+unknown-type-filter · Finding "store.unknown-type" doc.rs:2087 → block envelope · formats: agent+json (human=agent) · corpus: none`
- `doc-list+not-set-up · require_project_layer doc.rs:2081 → Orchestration · formats: agent+json (human=agent) · corpus: none`

### jigc task list
- `task-list+roster · render::task_list render.rs:1538 (print task.rs:192) · formats: agent+json (human=agent) · corpus: none` (empty roster same site)
- `task-list+read-error · run_list with_context → operational_error task.rs:147 · formats: agent+json (human=agent) · corpus: none`

### jigc task diff
- `task-diff+changeset · run_diff plain (code diff header + staged docs) task.rs:208-220 · formats: format-agnostic plain (takes no Format) · corpus: none`
- `task-diff+no-such-task · TaskArea::resolve → no_such_task task.rs:507 → operational_error task.rs:147 · formats: agent+json (human=agent) · corpus: r1` (loose)

### jigc task validate
- `task-validate+report · render::validation render.rs:359 (print task.rs:247; exit 0 or 3) · formats: agent+json (human=agent) · corpus: r1,r2a` (r1:1244 exit 3; r2a:273/298)
- `task-validate+no-such-task-or-op-error · TaskArea::resolve / validate err → operational_error task.rs:238/259 · formats: agent+json (human=agent) · corpus: none`
- `task-validate+probe-preflight-missing · require_doc_code_probe bespoke bail! task.rs:594 → operational_error · formats: agent+json (human=agent) · corpus: none`

### jigc task discard
- `task-discard+success · render::task_ack TaskAck::Discarded render.rs:1229 (print task.rs:281) · formats: agent+json (human=agent) · corpus: none`
- `task-discard+no-such-task · TaskArea::resolve → no_such_task task.rs:507 → operational_error task.rs:147 · formats: agent+json (human=agent) · corpus: r1` (loose)
- `task-discard+remove-error · run_discard with_context task.rs:280 → operational_error · formats: agent+json (human=agent) · corpus: none`

### jigc task bind
- `task-bind+success · render::task_ack TaskAck::Bound render.rs:1223 (print task.rs:349) · formats: agent+json (human=agent) · corpus: r2b` (r2b:971)
- `task-bind+role-not-declared · bespoke bail! "role … is not a declared read-role" task.rs:1359 → operational_error · formats: agent+json (human=agent) · corpus: none`
- `task-bind+malformed-address · bespoke anyhow! "malformed address …" task.rs:1364 → operational_error · formats: agent+json (human=agent) · corpus: r2b` (r2b:964)
- `task-bind+no-such-doc · bespoke bail! "no such doc …" task.rs:1375 → operational_error · formats: agent+json (human=agent) · corpus: none`
- `task-bind+doctype-mismatch · bespoke bail! "doctype mismatch …" task.rs:1380 → operational_error · formats: agent+json (human=agent) · corpus: none`
- `task-bind+no-such-task · TaskArea::resolve → no_such_task → operational_error · formats: agent+json (human=agent) · corpus: r1` (loose)

### jigc task finalize
- `task-finalize+landed-success · render::finalize_landed render.rs:913 (print task.rs:1143; relay_hook_output task.rs:1149) · formats: agent+json (human=agent) · corpus: r1,r2a,r2b` (r1:1315 manifest; r1:1339 carry-staged; r2a:312; r2b:845)
- `task-finalize+dry-run-manifest · render::finalize_manifest render.rs:880 (print task.rs:1009) · formats: agent+json (human=agent) · corpus: none`
- `task-finalize+validation-blocked · TaskArea::blocked → render::validation task.rs:769-776 (exit 3) · formats: agent+json (json→stdout, agent/human→stderr) · corpus: r1,r2a,r2b` (r1:1260; r2a:246; r2b:834)
- `task-finalize+sub-task-membership-refusal · engine sub_task_finalize_finding via blocked() task.rs:807 · formats: agent+json (human=agent) · corpus: none`
- `task-finalize+base-repin-conflict-block · decide_base_repin Err → blocked() task.rs:845 · formats: agent+json (human=agent) · corpus: none`
- `task-finalize+carried-staged-refusal · decide_carryover findings → blocked() task.rs:1028 (finalize.carried-staged) · formats: agent+json (human=agent) · corpus: r1` (r1:1329)
- `task-finalize+nothing-staged-recolor · nothing_staged_finding task.rs:949 → blocked() · formats: agent+json (human=agent) · corpus: none`
- `task-finalize+left-out-advisory (pre-commit) · emit_left_out_advisory task.rs:2014 (call task.rs:1069) · formats: agent+json (json→stderr, agent→stdout) · corpus: none`
- `task-finalize+carried-over-advisory (pre-commit) · emit_carried_advisory task.rs:2030 (call task.rs:1077) · formats: agent+json (human=agent) · corpus: r1` (r1:1339)
- `task-finalize+migration-review-hold · render::migration_review render.rs:1975 (print task.rs:1045; exit 4) · formats: agent+json (human=agent) · corpus: none` (described in migrate template r1:1183, no exit-4 capture)
- `task-finalize+stage-git-failure · stage_failed_finding task.rs:1163 → blocked() · formats: agent+json (human=agent) · corpus: none`
- `task-finalize+commit-rejected · render::commit_rejected (verbatim hook stderr) render.rs:862 (print task.rs:1179) · formats: agent+json (human=agent) · corpus: none`
- `task-finalize+operational-error · run_finalize err → operational_error task.rs:424/1182 · formats: agent+json (human=agent) · corpus: none`
- `task-finalize+no-such-task · TaskArea::resolve → no_such_task task.rs:417 → operational_error · formats: agent+json (human=agent) · corpus: r1` (r1:1294)

### Top-level clap surfaces (main.rs)
- `top+version · clap-generated ("jigc 1.0.0-rc.7") main.rs:72 (exit 0) · formats: clap-generated (format flag inert) · corpus: r1` (r1:3)
- `top+help · clap-generated main.rs:72 (exit 0) · formats: clap-generated · corpus: r1` (r1:9; per-verb --help throughout r1)
- `top+usage-error · clap-generated (exit 2) main.rs:71-72 · formats: clap-generated · corpus: r2a,r2b` (r2a:332; r2b:964)
- `top+unknown-subcommand-tip · unknown_subcommand_tip appended eprintln main.rs:79 (map cli.rs:1219; curated task discard-write, task status) · formats: plain stderr appended after clap error (exit 2) · corpus: r1` (r1:1302)

### Shared funnel (folded)
- `shared+cwd-unresolvable · eprintln! "cannot determine the current directory: &lt;err&gt;" e.g. cli.rs:385 (identical in every run_* wrapper) · formats: format-agnostic plain (bypasses render) · corpus: none`

> Cross-verb note: `write.area-barrier` (`barrier_block`, doc.rs:3372) is reachable from every address-bearing doc write verb via `staged_path`; folded here as one cross-verb bespoke refusal state rather than repeated per verb.

_Part 1A subtotal: 97 rows (95 verb/arm rows + 4 top-level clap block − shared row counted once; per the enumerating pass). 68 rows have zero corpus evidence._

## Part 1B — operator / store verbs

All `render::*` funnels are `agent+json (human=agent)`. The shared operational-error funnel `render::operational_error` (`render.rs:2103`) emits `{"error": "<chain>"}` on json, the raw `{err:#}` chain on agent/human. Distinct refusal *text* (each bail!/anyhow!/finding_to_err) = its own row; one generating funnel.

### shared surfaces (referenced by many verbs)
- `shared+cwd-unreadable · run_* wrappers eprintln crates/cli/src/cli.rs:385,413,441,464,483,499,518,550,578,601,619,637,664,710 · agent only (plain eprintln, not format-routed) · corpus: none` (every run_* wrapper)
- `shared+not-set-up · locate::not_set_up locate.rs:52 · via operational_error → agent+json (human=agent) · corpus: none` (describe/upgrade/ingest/migrate/migrate-corpus/unmanage/rename/relocate/config-verbs/validate; NOT setup/uninstall)
- `shared+no-git-repo · locate::locate_from bail locate.rs:71 · via operational_error → agent+json (human=agent) · corpus: none`
- `shared+home-unset · locate::locate context locate.rs:63 · via operational_error → agent+json (human=agent) · corpus: none`

### jigc milestone create
- `milestone-create+success · run_create → render::milestone milestone.rs:296 / render.rs:1759 · agent+json (human=agent) · corpus: r2a`
- `milestone-create+refusal-record-owns-slug · guard_record_free → record_exists_finding milestone.rs:882 · via operational_error · agent+json (human=agent) · corpus: none` (methodology-pack only)
- `milestone-create+refusal-serial-collision · mint_milestone(finding_to_err) milestone.rs:277 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-create+err-io/git · run_create context milestone.rs:246-299 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc milestone add-task
- `milestone-add-task+success · run_add_task → render::milestone milestone.rs:683 / render.rs:1759 · agent+json (human=agent) · corpus: r2a`
- `milestone-add-task+refusal-drift-conflict · reconcile_record_preflight milestone.rs:494 · via operational_error · agent+json (human=agent) · corpus: none` (reconciliation.* conflict-block)
- `milestone-add-task+refusal-terminal · reseed_cache → milestone.terminal milestone.rs:906 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-add-task+refusal-add-collision · add_task(finding_to_err) milestone.rs:664 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc milestone add-from-spec
- `milestone-add-from-spec+success · run_add_from_spec → render::milestone milestone.rs:802 / render.rs:1759 · agent+json (human=agent) · corpus: none`
- `milestone-add-from-spec+refusal-no-criteria · add_from_spec milestone.no-criteria milestone.rs:773 · via operational_error · agent+json (human=agent) · corpus: r2a` (capture note)
- `milestone-add-from-spec+refusal-unknown-spec/milestone · add_from_spec(finding_to_err) milestone.rs:773 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-add-from-spec+refusal-drift/terminal · reconcile_record_preflight / reseed_cache milestone.rs:756,763 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc milestone list-tasks
- `milestone-list-tasks+success · run_list_tasks → render::milestone milestone.rs:933 / render.rs:1759 · agent+json (human=agent) · corpus: r2a`
- `milestone-list-tasks+refusal-no-such · no_such_milestone milestone.rs:926→225 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-list-tasks+refusal-terminal · reseed_cache milestone.terminal milestone.rs:923 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc milestone provision
- `milestone-provision+success · run_provision → render::milestone milestone.rs:988 / render.rs:1759 · agent+json (human=agent) · corpus: r2a` (idempotent reuse silent within success line)
- `milestone-provision+refusal-no-such · no_such_milestone milestone.rs:971 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-provision+refusal-stale-base · guard_base_live → stale_base_finding milestone.rs:981→626 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-provision+err-git-worktree · git_worktree bail milestone.rs:1126 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc milestone execute
- `milestone-execute+success · dispatch_execute → render::composed milestone.rs:1325 / render.rs:149 · agent+json (human=agent) · corpus: none` (composes milestone-execution over id-sorted list)
- `milestone-execute+refusal-no-such · run_execute → no_such_milestone milestone.rs:1357 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-execute+refusal-terminal · reseed_cache milestone.terminal milestone.rs:1354 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-execute+refusal-compose-block · execute_milestone_in_repo Err milestone.rs:1363 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc milestone join
- `milestone-join+success-clean · dispatch_join → render::milestone_join milestone.rs:1385 / render.rs:1792 · agent+json (human=agent) · corpus: r2a`
- `milestone-join+success-with-blocking-clash · render::milestone_join THEN per-finding route to stderr milestone.rs:1385,1402-1406 · summary agent+json; join.same-doc-clash msg+route AGENT-ONLY stderr (bespoke) · corpus: none` (exit 1)
- `milestone-join+refusal-unknown/stale-base/io · run_join Err milestone.rs:1378 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc milestone finalize
- `milestone-finalize+landed-squash · milestone_landed_summary → render::milestone_finalized milestone.rs:1800 / render.rs:1899 · agent+json (human=agent) · corpus: r2a` (capture note; see honesty #3)
- `milestone-finalize+landed-per-subtask-chain · milestone_finalized ChainPerSubtask milestone.rs:1740 / render.rs:1899 · agent+json (human=agent) · corpus: none` (squash:false)
- `milestone-finalize+blocked-plan · blocked() → render::validation milestone.rs:1647→2000 · json→stdout envelope, agent/human→stderr msg+route, exit 3 · corpus: none` (finalize.base-mismatch / finalize.empty-commit / validation)
- `milestone-finalize+blocked-carryover · blocked() finalize.carried-staged milestone.rs:1677→2000 · same split, exit 3 · corpus: none`
- `milestone-finalize+refusal-code-collision · detect_code_collision (squash:false) milestone.rs:1702 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-finalize+commit-rejected-squash · try_execute_finalize_plan Err → operational_error milestone.rs:1818 · agent+json (human=agent); git stderr verbatim; finalize.commit-rejected in log · corpus: none`
- `milestone-finalize+commit-rejected-chain · ChainPerSubtask abort → operational_error milestone.rs:1765 · agent+json (human=agent) · corpus: none`
- `milestone-finalize+refusal-no-such · no_such_milestone milestone.rs:1540 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-finalize+refusal-stale-base/terminal/drift · guard_base_live / reseed_cache / reconcile_record_preflight milestone.rs:1527,1536,1553 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-finalize+warn-leaked-worktree · remove_worktrees milestone.rs:1972 · agent only (non-blocking eprintln, does not gate exit) · corpus: none`
- `milestone-finalize+note-subtask-cleanup-failed · cleanup_subtask_areas milestone.rs:1933 · agent only (self-heal eprintln) · corpus: none`

### jigc milestone discard
- `milestone-discard+success · run_discard → render::milestone milestone.rs:1234 / render.rs:1759 · agent+json (human=agent) · corpus: r2a`
- `milestone-discard+refusal-dirty-worktree · dirty_worktree_finding milestone.rs:1204→1279 · via operational_error · agent+json (human=agent); milestone.dirty-worktree · corpus: r2a`
- `milestone-discard+refusal-no-such · run_discard bespoke bail milestone.rs:1185 · via operational_error · agent+json (human=agent); OWN variant (routes to list-tasks, never create) · corpus: none`
- `milestone-discard+refusal-drift/terminal · reconcile_record_preflight / reseed_cache milestone.rs:1167,1175 · via operational_error · agent+json (human=agent) · corpus: none`
- `milestone-discard+note-workbench-removal-failed · remove_milestone_area milestone.rs:1313 · agent only (self-heal eprintln) · corpus: none`

### jigc config set
- `config-set+success · ConfigCommand::dispatch Ok config.rs:151 · PRINTS NOTHING (silent success, exit 0) · corpus: none` (see honesty #1)
- `config-set+refusal-undeclared-key · run_set config.undeclared-key config.rs:191 · via operational_error · agent+json (human=agent) · corpus: r2a`
- `config-set+refusal-value-rejected · run_set config.value-rejected config.rs:200 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-set+advisory-docs-root-relocation · route_docs_root_repoint_orphans config.rs:266,287,289 · agent only (best-effort eprintln block, never fails write) · corpus: r2a`

### jigc config insert-step
- `config-insert-step+success · run_insert_step Ok config.rs:388 · PRINTS NOTHING (silent, exit 0) · corpus: none`
- `config-insert-step+refusal-basename-collision · check_basename_collision config.step-id-collision config.rs:371→926 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-insert-step+refusal-anchor-absent · check_anchor_present config.anchor-absent config.rs:372→971 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-insert-step+err-source-file · run_insert_step context config.rs:352,355 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc config replace-step
- `config-replace-step+success · run_replace_step Ok config.rs:450 · PRINTS NOTHING (silent, exit 0) · corpus: none`
- `config-replace-step+refusal-malformed-target · StructuralTarget::parse config.rs:408 · via operational_error · agent+json (human=agent) · corpus: r2a` (shared parser w/ remove-step)
- `config-replace-step+refusal-basename-collision · check_basename_collision config.rs:424 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-replace-step+refusal-anchor-absent · check_anchor_present config.rs:425 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-replace-step+refusal-fork-bytes-absent · resolve_fork_bytes config.anchor-absent config.rs:432→611 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc config remove-step
- `config-remove-step+success · run_remove_step Ok config.rs:483 · PRINTS NOTHING (silent, exit 0) · corpus: none`
- `config-remove-step+refusal-malformed-target · StructuralTarget::parse config.rs:462 · via operational_error · agent+json (human=agent) · corpus: r2a`
- `config-remove-step+refusal-anchor-absent · check_anchor_present config.rs:469 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-remove-step+refusal-fork-bytes-absent · resolve_fork_bytes config.rs:476 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc config fill
- `config-fill+success · run_fill Ok config.rs:519 · PRINTS NOTHING (silent, exit 0) · corpus: none`
- `config-fill+refusal-malformed-target · SlotFillTarget::parse config.rs:503 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-fill+refusal-nested-fill · check_content_no_nested config.nested-fill config.rs:509→669 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-fill+refusal-fill-point-absent · check_fill_point_present config.fill-point-absent config.rs:510→703 · via operational_error · agent+json (human=agent) · corpus: r2a`
- `config-fill+err-handoff-read · doc::read_handoff config.rs:504 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc config fork
- `config-fork+success · run_fork Ok config.rs:576 · PRINTS NOTHING (silent, exit 0) · corpus: none`
- `config-fork+refusal-malformed-target · StructuralTarget::parse config.rs:542 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-fork+refusal-already-forked · check_not_already_forked config.step-id-collision config.rs:554→595 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-fork+refusal-anchor-absent · check_anchor_present config.rs:555 · via operational_error · agent+json (human=agent) · corpus: none`
- `config-fork+refusal-fork-bytes-absent · resolve_fork_bytes config.rs:561 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc setup
- `setup+success · run_setup → render::setup_success cli.rs:419 / render.rs:1302 · agent+json (human=agent) · corpus: r1` (install-commit bullet conditional)
- `setup+advisory-marker-unwired · install (write_compose_marker false) setup.rs:791 · agent only (eprintln warning + route, install still succeeds) · corpus: none`
- `setup+block-* · run_setup Err → render::setup_block cli.rs:423 / render.rs:1349 · agent+json (human=agent) · corpus: none` (one funnel, distinct setup.* codes: setup.repo-root :695, profile-load :702, spawn-template :723, profile-incomplete :734, write-bootstrap :749, inject-reference :756, init-project-layer :765, compose-marker :782, version-stamp :806, secrets-gitignore :819, inject-allowlist :829, inject-hook :839, inject-deny :853, install-hook :865/:874, extract-probe :889/:897, install-commit :916)

### jigc uninstall
- `uninstall+success · run_uninstall → render::uninstall_success cli.rs:447 / render.rs:1372 · agent+json (human=agent); per-artifact bullets or "(nothing to remove …)" no-op · corpus: none`
- `uninstall+block-* · run_uninstall Err → render::setup_block cli.rs:451 / render.rs:1349 · agent+json (human=agent) · corpus: none` (codes: uninstall.repo-root :1175, profile-load :1183, remove-jigc :1206, unwire-reference :1222, remove-allowlist :1233, remove-hook :1244, remove-deny :1254, remove-precommit :1264)

### jigc upgrade
- `upgrade+report-clean-or-findings · run_upgrade → render::validation_upgrade cli.rs:528 / render.rs:374 · agent+json (human=agent); exit gated on report.has_blocking() · corpus: r2a` (see honesty #2 — wording mismatch)
- `upgrade+err-locate · run_upgrade Err → operational_error cli.rs:533 · agent+json (human=agent) · corpus: none` (incl. shared+not-set-up)

### jigc ingest
- `ingest+report · run_ingest → render::ingest cli.rs:556 / render.rs:1441 · agent+json (human=agent); per-candidate rows + verdict legend + footer; always exit 0 · corpus: r1,r2a,r2b`
- `ingest+err · run_ingest Err → operational_error cli.rs:560 · agent+json (human=agent) · corpus: none`

### jigc migrate
- `migrate+success · migrate::run → render::composed migrate.rs:56 / render.rs:149 · agent+json (human=agent) · corpus: r2b`
- `migrate+refusal-not-migratable · ensure_migratable bail migrate.rs:114 · via operational_error · agent+json (human=agent) · corpus: none` (unknown vs known-not-migratable, names set + route)
- `migrate+refusal-bad-slug · migrate_in_repo bail migrate.rs:175 · via operational_error · agent+json (human=agent) · corpus: none`
- `migrate+refusal-unreadable-source · migrate_in_repo anyhow+route migrate.rs:192 · via operational_error · agent+json (human=agent) · corpus: none`
- `migrate+refusal-not-set-up · migrate_in_repo → not_set_up migrate.rs:155 · via operational_error · agent+json (human=agent) · corpus: none`
- `migrate+err-compose/mint · mint_migration_in_repo / compose_migrate_in_repo migrate.rs:212,239 · via operational_error · agent+json (human=agent) · corpus: none`

### jigc migrate-corpus
- `migrate-corpus+report-clean · migrate_corpus::run → render::corpus_migration migrate_corpus.rs:183 / render.rs:1667 · agent+json (human=agent); exit 0 when blocked empty · corpus: r1,r2a` (+ --dry-run r2a)
- `migrate-corpus+report-with-blocked · render::corpus_migration migrate_corpus.rs:189 / render.rs:1690 · agent+json (human=agent); migrate-corpus.* code+msg+route; exit 1 · corpus: none`
- `migrate-corpus+err-commit-rejected · git_run bail migrate_corpus.rs:371 · via operational_error · agent+json (human=agent); git stdout+stderr verbatim, "written but NOT committed" · corpus: none`
- `migrate-corpus+err-locate/build · migrate_in_repo Err → operational_error migrate_corpus.rs:193 · agent+json (human=agent) · corpus: none`

### jigc unmanage
- `unmanage+success-dropped · run_unmanage → render::unmanage cli.rs:643 / render.rs:1593 · agent+json (human=agent); two dropped variants · corpus: r2a`
- `unmanage+noop-idempotent · render::unmanage (dropped=false) render.rs:1615 · agent+json (human=agent); "no-op: … not managed" · corpus: r2a`
- `unmanage+err · run_unmanage Err → operational_error cli.rs:647 · agent+json (human=agent) · corpus: none`

### jigc rename
- `rename+success · run_rename → render::rename cli.rs:670 / render.rs:1627 · agent+json (human=agent); move summary + repointed referrers + optional advisory prose-mentions · corpus: r1,r2a` (r2a:256 one inbound ref)
- `rename+refusal-no-target · run bail rename.rs:98 · via operational_error · agent+json (human=agent) · corpus: none`
- `rename+refusal-slug-empty · run bail rename.rs:111 · via operational_error · agent+json (human=agent) · corpus: none`
- `rename+refusal-dirty-tree · run bail rename.rs:134 · via operational_error · agent+json (human=agent) · corpus: none`
- `rename+refusal-mid-fanout · run bail rename.rs:144 · via operational_error · agent+json (human=agent); names in-flight task/milestone · corpus: none`
- `rename+refusal-placement-singleton · run bail rename.rs:161 · via operational_error · agent+json (human=agent) · corpus: none`
- `rename+refusal-milestone-record-reslug · run bail rename.rs:174 · via operational_error · agent+json (human=agent) · corpus: none`
- `rename+refusal-collision · run bail rename.rs:184 · via operational_error · agent+json (human=agent) · corpus: none`
- `rename+refusal-introduced-dangle · apply_and_commit bail rename.rs:360 · via operational_error · agent+json (human=agent); rolled back · corpus: none`
- `rename+err-commit-rejected/io · apply_and_commit rename.rs:327,343,389 · via operational_error · agent+json (human=agent); transaction rolled back · corpus: none`

### jigc relocate
- `relocate+report · relocate::run → render::freeze_exempt_relocation relocate.rs:82 / render.rs:1723 · agent+json (human=agent); moved/displaced/blocked lines; exit 0 · corpus: none`
- `relocate+refusal-frozen-doctype · relocate_freeze_exempt bail relocate.rs:144 · via operational_error · agent+json (human=agent) · corpus: r1,r2a`
- `relocate+refusal-unknown-doctype · relocate_in_repo anyhow relocate.rs:104 · via operational_error · agent+json (human=agent) · corpus: none`
- `relocate+refusal-transient-doctype · relocate_freeze_exempt anyhow relocate.rs:152 · via operational_error · agent+json (human=agent) · corpus: none`
- `relocate+refusal-bad-from · parse_prior_home bail relocate.rs:115,120 · via operational_error · agent+json (human=agent) · corpus: none`
- `relocate+err-managed-squatter · displace_foreign_squatter bail relocate.rs:263 · surfaces per-doc in blocked list (render.rs:1744), not fatal · corpus: none`

### jigc describe
- `describe+success · run_describe → render::describe cli.rs:391 / render.rs:2151 · agent+json (human=agent); free-prose menu + footer · corpus: r1`
- `describe+err · run_describe Err → operational_error cli.rs:395 · agent+json (human=agent) · corpus: none` (incl. shared+not-set-up)

### jigc validate (store sweep)
- `validate+report · run_validate_store → render::validation_store cli.rs:719 / render.rs:398 · agent+json (human=agent); findings at cascade severity + "(gates at finalize)" label + store trailer + footer; JSON adds scope/report_only; exit 0 EXCEPT three exit-flip exceptions → exit 1 (validation_store_exit_flips cli.rs:736) · corpus: r1,r2a`
- `validate+report-clean · render::validation_store (empty) render.rs:451 · agent+json (human=agent); "no findings — the committed store validates clean" · corpus: r2a` (loose)
- `validate+err-probe-missing · require_doc_code_probe bail cli.rs:809→975 · via operational_error · agent+json (human=agent) · corpus: none`
- `validate+err-locate/build · validate_store_in_repo Err → operational_error cli.rs:744 · agent+json (human=agent) · corpus: none`

_Part 1B subtotal: 108 rows (incl. 4 shared-surface rows + 8 config-*+success silent-success rows listed for completeness). 78 rows have zero corpus evidence._

**Part 1B honesty notes:** (1) `config-*+success` prints NOTHING to stdout (`config.rs:151`, `Ok(()) => Outcome::success()`) — real outcome states, no user-visible text, listed flagged (silent-success is itself audit-relevant). (2) `jigc upgrade` corpus discrepancy: r2a (rc.7) shows `"no findings — the task validates clean"` but current code (`render::validation_upgrade` render.rs:374) uses `"no findings — no recorded config deltas …"` — rc.7 wording predates or is a different path; pinned to current code. (3) Milestone-finalize landed stdout: r2a note records empty stdout on rc.7 but current code prints `render::milestone_finalized`; pinned to current. (4) `milestone join` blocking-clash + `milestone finalize` `blocked()` split streams: json→stdout envelope, agent/human→stderr message+route. (5) `orphan.rs` / `route_fence.rs` generate NO user-facing text of their own (pure detection / parse-assert helper) — excluded per brief. (6) `setup.*`/`uninstall.*`/`config.*` finding codes folded into their single render-funnel rows (codes listed inline) — individual codes are Part 2's scope.

---

# PART 2 — FINDING SURFACES

Re-derived from code at HEAD `12bf692` (the r2b Part-3 static inventory was the completeness cross-check only). `tests`: `yes` = asserted in `crates/cli/tests`; `unit` = asserted only in a src `#[cfg(test)]` module; `no` = not asserted. Route = `Route::*` kind + verbatim text. Codes marked **graded** are constructed at a fixed severity but their `(probe,check)` inventory row can be re-graded via a cascade knob (constructed severity reported). Grouped by file.

## crates/engine/src/parse.rs — `conformance.*` (all Blocking, route NONE, exempt via is_route_exempt "conformance.*")
- `conformance.header-not-first` · blocking · "the header section must be the document's first section" · NONE(exempt) · parse.rs:216 · tests:no · corpus:none
- `conformance.section-missing` · blocking · "required section heading `## {section.id}` is missing" · NONE(exempt) · parse.rs:287 · tests:yes · corpus:r2a
- `conformance.section-renamed` · blocking · "section heading {text:?} does not match required section `{section.id}`" · NONE(exempt) · parse.rs:301 · tests:unit · corpus:r2a
- `conformance.orphaned-sentinel` · blocking · "`<!-- fields -->` sentinel without a following bullet list" · NONE(exempt) · parse.rs:769 · tests:unit · corpus:none
- `conformance.unknown-field` · blocking · `unknown_field_message(...)` (dynamic) · NONE(exempt) · parse.rs:819 · tests:yes · corpus:none
- `conformance.malformed-field-block` · blocking · `e.message` (dynamic) · NONE(exempt) · parse.rs:835 · tests:no · corpus:none
- `conformance.item-anchor-missing` · blocking · "repeatable item `{heading}` has no `{#id}` anchor" · NONE(exempt) · parse.rs:920 · tests:unit · corpus:none (declared non-unique)
- `conformance.item-anchor-malformed` · blocking · "malformed `{#id}` anchor `{#{found}}` (must be a slug `[a-z0-9-]`)" · NONE(exempt) · parse.rs:931 · tests:unit · corpus:none (declared non-unique)
- `conformance.item-anchor-duplicate` · blocking · "duplicate `{#id}` anchor `{#{id}}` in repeatable section" · NONE(exempt) · parse.rs:944 · tests:unit · corpus:none
- `conformance.item-slot-label-missing` · blocking · "multi-slot item is missing its `#### {title_case(leaf_id)}` sub-heading" · NONE(exempt) · parse.rs:1098 · tests:no · corpus:none
- `conformance.item-slot-delimiter-shadowed` · blocking · "`#### {label}` in multi-slot item prose shadows the item-slot delimiter; slot prose must not start a line with `#### ` (use `#####`+ or rephrase)" · NONE(exempt) · parse.rs:1135 · tests:unit · corpus:none (declared non-unique)
- `conformance.slot-setext-heading` · blocking · "Setext heading in slot prose at line {line}; use `####` ATX depth or rephrase" · NONE(exempt) · parse.rs:1330 · tests:no · corpus:none (declared non-unique)
- `conformance.slot-heading-depth` · blocking · "heading at schema-reserved depth `{depth}` in slot prose at line {line}; use a deeper level or rephrase" · NONE(exempt) · parse.rs:1340 · tests:unit · corpus:none (declared non-unique)
- `conformance.heading-missing` · blocking · (found ONLY in finding.rs test fixtures at HEAD — no production constructor; flagged as possible retired code) · NONE(exempt) · — · tests:no · corpus:none

## crates/engine/src/index.rs
- `schema-conformance.mention-resolves` · advisory · "schema-conformance — in-prose mention `#{mention}` in `{from}` resolves to no committed doc …; correct or drop the mention" · [Human] "correct or drop the `#{mention}` mention in `{from}`" · index.rs:679 · tests:yes · corpus:r2b
- `schema-completeness.inverse-cardinality` · advisory · "schema-completeness — `{target}` has {count} inbound `{relation}` edge(s){inverse_phrase}, below the inverse-card minimum of {min}" · [Human] "author a `{relation}` referrer of `{target}`" · index.rs:776 · tests:yes · corpus:r2b
- `schema-conformance.ref-resolves` · blocking · "forward-ref integrity — `{from}#{relation}` target `{to}` resolves in neither the committed store nor this task's working area; resolution: fix the reference …, create the target …, or drop the `{relation}` field" · [Human] "fix the reference, create the target in this task, or drop the field" · index.rs:834 · tests:yes · corpus:r2a,r2b

## crates/engine/src/validate.rs (+ blocking_conformance helper validate.rs:2369; conformance_route map)
- `schema-conformance.unadopted-instance` · advisory · dynamic msg · [Human] `adoption_route(...)` dynamic · validate.rs:885 · tests:yes · corpus:r2b
- `schema-conformance.schema-version-current` (const SCHEMA_VERSION_CURRENT_CODE) · blocking · dynamic msg · dynamic human route · validate.rs:1192 · tests:yes · corpus:r2b
- `schema-conformance.unknown-type` · blocking · "staged doc `{identity}` has type `{ty}`, which the resolved cascade does not define" · conformance_route (human) · validate.rs:1594 · tests:unit · corpus:r2b
- `owner-artifact.present` · blocking · "owner-artifact `{declared.id}` in section `{section.id}` of `{display}`: {why}" · conformance_route · validate.rs:1714 · tests:yes · corpus:r2b
- `schema-conformance.repeatable-populated` · advisory · "repeatable section `{section.id}` parses zero items — structurally empty" · [Human] "populate the section, or exempt `{token}` via the `validation.schema-conformance.repeatable-populated.exempt` knob" · validate.rs:1867 · tests:unit · corpus:r2b
- `schema-conformance.surplus-sections-absent` · advisory · "{n} trailing surplus section heading(s) beyond the schema's {m} body section(s), starting at `## {first_text}` — the positional parse never visits them …" · [Human] "fold the surplus content into a schema section or remove it — jigc never reads or splices it" · validate.rs:1947 · tests:yes · corpus:r2b
- `schema-conformance.required-slot-present` · blocking · "required slot `{leaf_id}` in item `{item_path}` is empty" · conformance_route · validate.rs:2080 · tests:yes · corpus:r1,r2a,r2b
- `schema-conformance.field-value-conformant` · blocking · "field `{declared.id}` in item `{item_path}`: {why}" · conformance_route · validate.rs:2105 · tests:yes · corpus:r2a,r2b
- `schema-conformance.required-field-present` · blocking · "required field `{declared.id}` is missing from item `{item_path}`" · conformance_route · validate.rs:2114 · tests:yes · corpus:r2b
- `schema-conformance.field-value-conformant` (const ID_FROM_ENUM_CODE) · blocking · "id-from field `{repeatable.id_from}` in item `{item_path}`: `{slug}` is not an enum member" · conformance_route · validate.rs:2184 · tests:yes · corpus:r2a,r2b
- `schema-conformance.required-slot-present` · blocking · "required slot in section `{section.id}` is empty" · conformance_route · validate.rs:2225 · tests:yes · corpus:r1,r2a,r2b
- `schema-conformance.required-field-present` · blocking · "required field `{declared.id}` is missing from section `{section.id}`" · conformance_route · validate.rs:2258 · tests:yes · corpus:r2b
- `schema-conformance.field-value-conformant` · blocking · "field `{declared.id}` in section `{section.id}`: {why}" · conformance_route · validate.rs:2296 · tests:yes · corpus:r2a,r2b

## crates/engine/src/file_state.rs
- `file-state.staged-copy` · advisory · "staged copy of `{dest}` — this task's in-flight version of the doc" · [Informational] "no action needed — the staged copy is validated in-task and baselined when its finalize lands" · file_state.rs:139 · tests:unit · corpus:r1,r2b
- `file-state.hash-matches` · blocking · "on-disk content of `{path}` differs from the recorded state" · [Human] "review the out-of-band edit to `{path}` and re-author it through the owning workflow" · file_state.rs:645 · tests:yes · corpus:r2a,r2b
- `file-state.un-baselined` · advisory · "committed doc `{path}` is not yet baselined in the file-state record" · [Human] "no action needed — the doc is baselined on its next author or finalize" · file_state.rs:668 · tests:yes · corpus:r2b
- `reconciliation.rename` · blocking · "tracked managed doc {from} ({path}) is missing; {suspect} has the same content hash — likely renamed via `git mv`" · [Human] "adopt it as a CLI-owned rename …: `jigc rename {from} --to \"<New Title>\"`; or revert the move: `git mv {suspect} {path}`" · file_state.rs:847 · tests:yes · corpus:r2b
- `reconciliation.rename` · blocking · "tracked managed doc {from} ({path}) is missing" · [Human] "restore {path}, or confirm the deletion by dropping it from the index: `jigc unmanage {path}`" · file_state.rs:864 · tests:yes · corpus:r2b
- `reconciliation.absorb` · advisory · "external edit absorbed: `{path}`" · [Human] "no action needed — the external edit was absorbed into the baseline" · file_state.rs:881 · tests:yes · corpus:r2a,r2b
- `reconciliation.conformance-block` · blocking · "nonconformant edit on `{path}`: {detail}" · [Human] "fix the file to restore conformance, or revert the edit …" · file_state.rs:902 · tests:yes · corpus:r2a,r2b
- `reconciliation.conformance-block` · advisory · "unvetted file `{path}` in a managed location is not schema-conformant: {detail}" · [Human] "ingest, migrate, or move `{path}` out of the managed location to resolve it" · file_state.rs:935 · tests:yes · corpus:r2a,r2b
- `reconciliation.conflict-block` · blocking · "conflict on `{path}`: an external edit and this task's staged writes both changed it" · [Mechanical] `jigc task discard <task-id>` + " to drop this task's staged writes …, or revert the external edit on disk to keep them" · file_state.rs:961 · tests:yes · corpus:r2b
- `file-state.baseline-adopt` · advisory · "baseline adopted: `{path}`" · [Human] "no action needed — the baseline was adopted on first encounter" · file_state.rs:982 · tests:yes · corpus:r2b

## crates/engine/src/finalize.rs
- `finalize.promote-clobber` · blocking · dynamic msg · dynamic human route · finalize.rs:433 · tests:yes · corpus:r1,r2a,r2b
- `finalize.provenance-io` · blocking · "could not read the task provenance manifest under `{task_dir}`: {err}" · NONE (None; NOT exempt — survives via Err abort path, not the Serialize seam) · finalize.rs:447 · tests:no · corpus:r2b
- `finalize.carried-staged` · blocking · dynamic msg · [Human] dynamic route · finalize.rs:654 · tests:yes · corpus:r1,r2b
- `finalize.promote-io` · blocking · "could not read the staged managed doc `{path}` to promote it: {err}" · NONE (None) · finalize.rs:872 · tests:no · corpus:r2b
- `finalize.migration-no-replacement` · blocking · "this migration recorded the foreign source `{source_path}` but staged no managed doc to replace it …" · [Human] "author the canonical doc (e.g. `jigc doc create <doctype> --task <id>`), then re-run `jigc task finalize <id> --approve`" · finalize.rs:891 · tests:unit · corpus:r2b
- `finalize.source-path-io` · blocking · "could not read the recorded migration source path under `{task_dir}`: {err}" · NONE (None) · finalize.rs:912 · tests:no · corpus:r2b
- `finalize.no-task` · blocking · "no task working area at `{task_dir}` — nothing to finalize" · [Human] "start a task with `jigc start \"<intent>\"`" · finalize.rs:929 · tests:unit · corpus:r2b
- `finalize.base-mismatch` · blocking · dynamic msg · dynamic route · finalize.rs:1036 · tests:yes · corpus:r2b
- `finalize.base-mismatch` · blocking · "the task was started at base `{base.short}` but HEAD is now `{head_sha}`, and the moved history overlaps the task's work on `{paths}`" · [Human] "resolve the overlap on `{paths}` against the new history, or discard the task with `jigc task discard`" · finalize.rs:1061 · tests:yes · corpus:r2b
- `finalize.empty-commit` · blocking · "task validated but produced no diff — nothing to finalize" · [Human] "make a change, then re-run `jigc task finalize`" · finalize.rs:1085 · tests:unit · corpus:r2a,r2b
- `finalize.render-io` · blocking · "could not read the staged commit doc `{path}`: {err}" · NONE (None) · finalize.rs:1097 · tests:unit · corpus:r2b

## crates/engine/src/milestone.rs (store_block helper milestone.rs:433)
- `store.unparseable` · blocking · "could not parse spec address `{spec_addr}`: {err}" · [Human] "supply a valid `<type>:<slug>` spec address" · milestone.rs:360 · tests:yes · corpus:r2b
- `store.unknown-type` · blocking · "unknown doctype `{type_name}` for `{spec_addr}`" · [Human] "list the available doctypes with `jigc describe`" · milestone.rs:371 · tests:yes · corpus:r1,r2b
- `store.transient-type` · blocking · "doctype `{type_name}` is transient (no `location:`); `{spec_addr}` is not committed" · [Human] "the referenced doctype has no committed location" · milestone.rs:380 · tests:yes · corpus:r2b
- `store.not-found` · blocking · "could not read `{spec_addr}` at `{path}`: {err}" · [Human] "create the referenced spec, or fix the address to an existing one" · milestone.rs:391 · tests:yes · corpus:r1,r2b
- `store.unparseable` · blocking · "`{spec_addr}` at `{path}` does not parse: {why}" · [Human] "fix the committed spec so it conforms to its schema" · milestone.rs:408 · tests:yes · corpus:r2b
- `store.no-such-section` · blocking · "`{spec_addr}` names no `criteria` section to seed from" · [Human] "the spec must declare a `criteria` section" · milestone.rs:420 · tests:yes · corpus:r2b
- `milestone.no-criteria` · blocking · "spec `{spec_addr}` has no criteria to seed from" · [Human] "add criteria to the spec in a task — `jigc doc add-item {spec_addr}#criteria --title \"<criterion>\" --task <task-id>` …; or add sub-tasks directly with `jigc milestone add-task`" · milestone.rs:453 · tests:yes · corpus:r1,r2a
- `milestone.unknown` · blocking · "milestone `{milestone_id}` does not exist" · [Human] "create it first with `jigc milestone create \"<title>\"`" · milestone.rs:492 · tests:unit · corpus:none
- `milestone.stale-base` · blocking · "milestone `{milestone_id}` is pinned to base `{base_short}`, which no longer exists …" · [Human] "restore the base commit, or re-pin the milestone's base, then re-run the op" · milestone.rs:513 · tests:no · corpus:none
- `milestone.sub-task-collision` · blocking · "sub-task `{sub_id}` is already in milestone `{milestone_id}`" · [Human] "add the sub-task with a distinct intent" · milestone.rs:532 · tests:unit · corpus:none
- `finalize.milestone-sub-task` · blocking · "task `{task_id}` is a sub-task of milestone `{milestone_id}` — the parent milestone's finalize is the only commit boundary …" · [Mechanical] `jigc milestone finalize <milestone_id>` + " — the milestone finalize folds every sub-task's staged work into the one aggregate commit" · milestone.rs:592 · tests:yes · corpus:none
- `milestone.terminal` · blocking · "milestone `{milestone_id}` is `{status}` — a settled milestone is over and has no workbench" · [Human] "read the settled record with `jigc doc show milestone-record:{milestone_id}`; new work starts a new milestone …" · milestone.rs:788 · tests:unit · corpus:none
- `milestone.record-exists` · blocking · "milestone `{milestone_id}` already has a committed record{reads}" · dynamic route · milestone.rs:828 · tests:no · corpus:none
- `milestone.record-read-back` · blocking · "could not re-derive milestone state from its record: {why}" · [Human] "reconcile the milestone record, then re-run the milestone op" · milestone.rs:849 · tests:no · corpus:none
- `milestone.record-flip` · blocking · "could not flip milestone record `{milestone_id}` to {target}: {err:?}" · [Human] "reconcile the milestone record, then re-run the {op}" · milestone.rs:1168 · tests:no · corpus:none
- `join.same-doc-clash` · blocking · "same-doc clash — sub-tasks [{tasks}] each write `{address}` at the milestone base; the join never blind-merges a shared managed doc" · [Human] "have the contending sub-tasks edit distinct docs, or merge their intent by hand" · milestone.rs:1765 · tests:yes · corpus:none
- `join.same-doc-clash` · blocking · "same-doc clash — {listing} resolve to the same final address `{final_address}`; the join never silently overwrites a managed doc" · [Human] "have the contending sub-tasks write distinct docs, or rename one …" · milestone.rs:1793 · tests:yes · corpus:none
- `join.unknown-type` · blocking · "colliding staged doc `{address}` in sub-task `{sub_id}` of milestone `{milestone_id}` has an unknown doctype" · [Human] "re-stage the doc under a known doctype" · milestone.rs:1814 · tests:no · corpus:none
- `join.self-ref-rewrite` · blocking · "could not rewrite the self-reference `{relation}` of colliding doc `{address}` in milestone `{milestone_id}`: {err:?}" · [Human] "re-stage the colliding doc so its self-reference is well-formed" · milestone.rs:1834 · tests:no · corpus:none
- `join.missing-provenance` · blocking · "staged doc `{address}` in sub-task `{sub_id}` of milestone `{milestone_id}` has no recorded provenance" · [Human] "re-stage the doc so its provenance is recorded" · milestone.rs:1850 · tests:no · corpus:none
- `join.area-isolation` · blocking · "isolation — sub-task `{sub_id}` of milestone `{milestone_id}` attributes `{address}` to itself but stages no such doc …" · [Human] "re-stage the doc inside its own sub-task area, or drop the stray attribution" · milestone.rs:1870 · tests:unit · corpus:none
- `milestone.serial-collision` · blocking · "milestone `{id}` already exists" · [Human] "add tasks with `jigc milestone add-task {id} \"<intent>\"` or pick a different title" · milestone.rs:1910 · tests:unit · corpus:none
- `milestone.area-io` · blocking · "could not {doing} for milestone `{id}`: {err}" · NONE (None) · milestone.rs:1923 · tests:no · corpus:none

## crates/engine/src/state.rs
- `task.serial-collision` · blocking · "task `{id}` is already active" · [Human] "resume with `jigc start --task {id}` or abandon with `jigc task discard {id}`" · state.rs:688 · tests:yes · corpus:r2b
- `create.unknown-doctype` · blocking · "unknown doctype `{type_name}`; known doctypes: [{set}]" · [Human] "run `jigc describe` to see the doctypes you can author" · state.rs:1047 · tests:unit · corpus:r2b
- `create.serial-collision` · blocking · "instance `{address}` already exists in the working area" · [Human] "edit the existing `{address}` instead of re-creating it" · state.rs:1075 · tests:yes · corpus:r2b
- `create.empty-title` · blocking · "`jigc doc create {type_name}` needs a title that yields an id, but the given title is empty or slugs to nothing" · [Human] "re-run with a non-empty `--title` (its slug becomes the doc id)" · state.rs:1090 · tests:unit · corpus:r2b
- `create.gate-blocked` · blocking · "the workflow does not allow `jigc doc create {type_name}` in-task; allowed doctypes: [{allowed}]" · [Human] "create `{type_name}` in a task minted from a workflow that grants it …" · state.rs:1111 · tests:yes · corpus:r2b
- `task.working-area-io` · blocking · "could not {doing} for task `{id}`: {err}" · NONE (None) · state.rs:1132 · tests:no · corpus:r2b

## crates/engine/src/store.rs (block helper store.rs:331)
- `store.not-staged` · blocking · "`{address_str}` is not staged in this task — only its committed copy exists" · [Mechanical] `jigc doc show {address_str}` + " — the task-less read serves the committed copy" · store.rs:302→332 · tests:yes · corpus:r2b
- `store.not-staged` · blocking · "`{address_str}` is not staged in this task and has no committed copy — nothing to read yet" · [Human] "create or author the doc in this task first — a staged copy exists only after a write" · store.rs:302→332 · tests:yes · corpus:r2b

## crates/engine/src/override_default.rs (delta_block helper override_default.rs:229; .with_check overrides check id)
- `override-default.scalar-set-orphaned` (check target-exists) · blocking · "scalar-set target `scalar:{key}` is no longer a declared knob in the current pack" · [Human] "drop this delta, or re-target `scalar:{key}` to a current knob key" · override_default.rs:207 · tests:yes · corpus:none
- `override-default.slot-fill-orphaned` (check target-exists) · blocking · "slot-fill target `step:{step}#{fill}` is no longer a declared `{{fill:}}` point in the current pack" · [Human] "drop this delta, or re-target `step:{step}#{fill}` to a current `{{fill:}}` point" · override_default.rs:279 · tests:yes · corpus:none
- `override-default.content-changed` (check target-unchanged) · blocking · "override target `{target_str}` changed in the current pack since it was recorded" · [Human] "review the change on `{target_str}`: keep your override, re-target it, or drop it" · override_default.rs:333 · tests:yes · corpus:none
- `override-default.needs-rebasing` (check basis-recorded) · blocking · "override target `{target_str}` has no recorded base-hash (an older manifest); its basis cannot be compared" · [Human] "re-record the delta on `{target_str}` (e.g. `jigc config replace-step …`) to pin a basis against the current pack" · override_default.rs:352 · tests:yes · corpus:none
- `override-default.target-exists` (check target-exists) · blocking · "override target `{target_str}` no longer exists in the current pack" · [Human] "remove or re-target the delta on `{target_str}`" · override_default.rs:384 · tests:yes · corpus:none

## crates/engine/src/probe.rs
- `pack-probe-integrity.probe-failure` (check re-set timeout/malformed-output/crash) · blocking · 3 templates: "probe `{probe_id}` exceeded its time budget and was killed" / "probe `{probe_id}` exited 0 but emitted unparseable output: {err}" / "probe `{probe_id}` exited non-zero (exit-code {exit}) with no usable output" · [Human] "the probe subprocess misbehaved … repair or re-install the probe binary (`jigc setup` reinstalls the shipped probes), then re-run the sweep" · probe.rs:328 (meta_finding), callers 281/294/301 · tests:no · corpus:r2b

## crates/engine/src/target_surface.rs
- `doc-code.title-names-symbol` · advisory · "component title `{item.title}` names symbol(s) {title_symbols:?} but its anchor implements `{symbol}` (`{value}`) …" · [Human] "start a task (`jigc start \"<intent>\"`), then run `jigc doc retitle-item {ty}:{slug}#{section}/{item} --title \"<new title>\"` within it …" · target_surface.rs:455 · tests:unit · corpus:none
- `doc-code.multi-valued-anchor` · blocking · "code-anchor `{address}` is list-valued; multi-valued anchors are not enumerated … resolve to a single anchor or add list-element support" · NONE (None; abort path) · target_surface.rs:560 · tests:unit · corpus:none

## crates/engine/src/cascade.rs — the parse-target families (all Blocking, target=null; declared singleton)
`structural-target.*` (cascade.rs:158), shared route [Human] "correct the target to the delta-target grammar — `workflow:<id>#<step-id>` … or `workflow:<id>` with an `after:`/`before:` anchor — in the `jigc config <verb>` argument … or the recorded `deltas:` entry that carries it"; all tests:no · corpus:none; exempt via is_declared_singleton "structural-target.":
- `structural-target.wrong-scheme` · "structural-op target scheme must be `workflow`"
- `structural-target.missing-colon` · "structural-op target needs a `workflow:<id>` scheme"
- `structural-target.empty-workflow-id` · "structural-op target has an empty workflow id"
- `structural-target.empty-step-id` · "structural-op target `#<step-id>` is empty"
- `structural-target.empty-anchor-step-id` · "structural-op `after:` / `before:` anchor step id is empty"
- `structural-target.missing-anchor` · "structural-op target needs a `#<step-id>` or an `after:` / `before:` anchor"
- `structural-target.conflicting-anchors` · "structural-op target cannot carry both `#<step-id>` and an `after:` / `before:` anchor"
- `structural-target.non-ascii` · "structural-op target ids must be ASCII"

`slot-fill-target.*` (cascade.rs:365), shared route [Human] "correct the target to the `step:<step-id>#<fill-id>` grammar in the `jigc config fill <target>` argument or the recorded `deltas:` entry that carries it"; all tests:no · corpus:none; exempt via is_declared_singleton "slot-fill-target.":
- `slot-fill-target.wrong-scheme` · "slot-fill target scheme must be `step`"
- `slot-fill-target.missing-colon` · "slot-fill target needs a `step:<id>` scheme"
- `slot-fill-target.missing-hash` · "slot-fill target needs a `#<fill-id>` extension point"
- `slot-fill-target.empty-step-id` · "slot-fill target has an empty step id"
- `slot-fill-target.empty-fill-id` · "slot-fill target `#<fill-id>` is empty"
- `slot-fill-target.non-ascii` · "slot-fill target ids must be ASCII"

## crates/engine/src/compose.rs + data_value.rs — the `workflow-refs.*` family (31 codes; is_declared_non_unique "workflow-refs.*")
blocking_workflow_refs (compose.rs:23) shares route **[Human] "fix the workflow/step/catalog definition the message names (a definition defect, repaired once at its source), then re-run"**. All Blocking; corpus:none for all.
- `workflow-refs.fan-out-join-paired` · "a `fan-out` step has no matching `join` step …" / "a `join` step has no matching `fan-out` step …" · Finding::block own routes ("add a `join` step after the `fan-out` …" / "add a `fan-out` step before the `join`, or remove the unpaired `join`") · compose.rs:3337/3342 · tests:yes
- `workflow-refs.structural-anchor-resolves` · "structural-op anchor `{anchor_id}` resolves to no entry in the include list (orphaned)" · [Human] "remove or re-target the structural-op anchored at `{anchor_id}` with `jigc config` …" · compose.rs:2229 · tests:unit
- `workflow-refs.slot-fill-orphan` · "slot-fill targets `step:{step}#{fill}`, a `{{fill:}}` point no resolved step body declares (orphaned)" · [Human] "remove or re-target the slot-fill on `step:{step}#{fill}` with `jigc config fill` …" · compose.rs:3170 · tests:unit
- `workflow-refs.suppressed-malformed` · "workflow front-matter `suppressed:` block is malformed: {msg}" · compose.rs:181 · tests:unit
- `workflow-refs.not-utf8` · "command catalog is not valid UTF-8" / "step definition is not valid UTF-8" / "workflow definition is not valid UTF-8" · compose.rs:286/1866/1962 · tests:no
- `workflow-refs.malformed-command-catalog` · "command catalog is malformed: {source}" · compose.rs:293 · tests:no
- `workflow-refs.duplicate-command-id` · "command catalog has two entries under id `{id}`" · compose.rs:314 · tests:no
- `workflow-refs.malformed-command-arg` · dynamic · compose.rs:388 · tests:unit
- `workflow-refs.malformed-data-value` · e.g. "command-ref `from:` path `{from}` is malformed: {source}" · compose.rs:465/987/1577/1587/1705 · tests:no
- `workflow-refs.command-ref-resolves` · "command-ref `{{cli.{id}}}` resolves to no catalog entry" · compose.rs:608/3055 · tests:yes
- `workflow-refs.schema-ref-resolves` · dynamic · compose.rs:652/3087 · tests:yes
- `workflow-refs.collection-not-lone` · dynamic · compose.rs:1720 · tests:no
- `workflow-refs.step-kind-malformed` · "step front-matter declares a malformed step kind: {msg}" · compose.rs:1905 · tests:unit
- `workflow-refs.missing-front-matter` · "workflow definition has no `---`-fenced front-matter block" · compose.rs:1969 · tests:unit
- `workflow-refs.malformed-front-matter` · "workflow front-matter is malformed config-family YAML: {source}" · compose.rs:1976 · tests:unit
- `workflow-refs.body-include-only` · "workflow body line is not an `{{ include: step:<id> }}`, blank, or comment: `{line}`" · compose.rs:2041 · tests:unit
- `workflow-refs.include-cycle-absent` · "include cycle: {cycle}" · compose.rs:2570 · tests:unit
- `workflow-refs.include-resolves` · "include `step:{id}` resolves to no step file in the cascade" · compose.rs:2578 · tests:yes
- `workflow-refs.run-marker-not-shadowed` · "step prose shadows the composer-reserved `Run: ` marker: `{}`" · compose.rs:3250 · tests:no
- `workflow-refs.spawn-marker-not-shadowed` · "step prose shadows the composer-reserved `Spawn: ` marker: `{}`" · compose.rs:3276 · tests:no
- `workflow-refs.checkpoint-marker-not-shadowed` · "step prose shadows the composer-reserved `Checkpoint: ` marker: `{}`" · compose.rs:3302 · tests:yes
- `workflow-refs.fill-survivor` · "a `{{fill: {fill_id}}}` survives composition unresolved (phase 5 does not re-run …)" · compose.rs:3367 · tests:no
- `workflow-refs.catalog-not-navigable` · data_value.rs:397 · tests:no
- `workflow-refs.store-not-navigable` · data_value.rs:422 · tests:no
- `workflow-refs.milestone-not-navigable` · data_value.rs:452 · tests:no
- `workflow-refs.undeclared-root` · data_value.rs:473 · tests:unit
- `workflow-refs.task-ref-in-no-task-workflow` · data_value.rs:486 · tests:yes
- `workflow-refs.undeclared-role` · data_value.rs:499/529 · tests:unit
- `workflow-refs.at-marker-on-non-scalar` · data_value.rs:515 · tests:unit
- `workflow-refs.unresolved` · data_value.rs:560 · tests:no

## crates/engine/src/write.rs (blocking_write helper write.rs:5048, route from write_route map; Finding::block for unset-ineligible)
- `write.list-overwrite` · blocking · "write rejected: field {field_key:?} already has {count} value(s); `set-field` replaces the whole list and would silently drop them. To set multiple values, pass them all in one call: --value {suggested:?}" · [Human] "re-run `jigc doc set-field` with the inline-list form shown in the message, carrying every value to keep" · write.rs:5338 · tests:yes · corpus:r2a,r2b
- `write.non-reparseable` · blocking · "write rejected: result does not re-parse ({detail})" · [Human] "nothing was persisted — revise the payload so the result still conforms …" · write.rs:5445/5558/5679/5747 · tests:unit · corpus:r2b
- `write.target-escape` · blocking · "write rejected: the change touched bytes outside the intended target" · [Human] "nothing was persisted — re-run the write; a recurring escape is a write-path defect to report" · write.rs:5459 · tests:unit · corpus:r2b
- `write.unknown-field` · blocking · "no field {field_key:?} declared in section {section_id:?}" / "no field {field_key:?} declared on item {item_ids:?} in section {section_id:?}" · [Mechanical] `jigc doc schema <doctype>` + " to see the declared shape, then re-run the write at a declared address" · write.rs:5488/5583/5609 · tests:no · corpus:r2b
- `write.malformed-value` · blocking · "write rejected: {why}" · [Mechanical] `jigc doc schema <doctype>` + " to see the field's declared type and members, then re-run the write with a conformant value" · write.rs:5495 · tests:yes · corpus:r1,r2b
- `write.unset-ineligible` · blocking · "write rejected: field {id:?} is {why} and cannot be unset" / "write rejected: field {id:?} is required (or defaulted) and cannot be unset" · [Human] "leave it in place — the CLI owns its value" / "to change it, set a new value with `jigc doc set-field <addr> --value <value>`" · write.rs:5645/5655 · tests:yes · corpus:none
- `write.not-present` · blocking · "slot in section {section_id:?} is not present" / "write rejected: {what} is not present" · [Mechanical] `jigc doc schema <doctype>` + " to see the declared shape …" · write.rs:5726 / 5917(splice_error_finding) · tests:yes · corpus:r2b
- `write.slot-setext-heading` · blocking · "Setext heading in slot prose at line {line}; use `####` ATX depth or rephrase" · [Human] "rewrite the Setext heading as `####` ATX depth or plain prose, then re-run the same write" · write.rs:5793 · tests:unit · corpus:r2b
- `write.slot-heading-depth` · blocking · "heading at schema-reserved depth `{depth}` in slot prose at line {line}; use `####` or rephrase" · [Human] "demote the heading to `####` depth or rephrase it as plain prose, then re-run the same write" · write.rs:5809 · tests:unit · corpus:r2b
- `write.already-present` · blocking · "write rejected: {what} is already present" · [Human] "the target already exists — edit it in place (`set-field`/`set-slot`) instead of re-creating it" · write.rs:5865 (generate_error_finding) · tests:no · corpus:r2b
- `write.unknown-section` · blocking · "write rejected: no section {id:?} declared in the schema" · [Mechanical] `jigc doc schema <doctype>` · write.rs:5865 · tests:yes · corpus:none
- `write.wrong-shape` · blocking · "write rejected: {what}" · [Mechanical] `jigc doc schema <doctype>` · write.rs:5865 · tests:no · corpus:none
- `write.unslugable-title` · blocking · "write rejected: title {title:?} has no slug-able content for an item id" · [Human] "re-run the write with a title carrying at least one word character …" · write.rs:5865 · tests:no · corpus:r2b
- `schema-conformance.field-value-conformant` (write-time via generate_error_finding) · blocking · "write rejected: {why}" · conformance_route · write.rs:5865 · tests:yes · corpus:r2a,r2b
- `write.non-reparseable` (splice_error_finding NotConformant) · blocking · "write rejected: the source does not conform to the schema" · write.rs:5917 · tests:unit · corpus:r2b

## crates/cli/src/cli.rs (store-sweep post-append advisories)
- `file-state.orphaned-doc` · advisory · "committed doc `{rel}` sits outside the resolved doctype roots — a `docs-root` change likely stranded it …" · [Human] "move it under the current resolved root (re-point `docs-root` to cover it) or drop it with `jigc unmanage`" · cli.rs:918 · tests:yes · corpus:none
- `file-state.unregistered-doc` · advisory · "committed doc `{rel}` looks managed (it sits under a `{doctype}`-style directory) but was never adopted …" · dynamic route · cli.rs:940 · tests:yes · corpus:none

## crates/cli/src/combine.rs
- `combine.code-collision` · blocking · "code collision — {listing} staged by more than one worktree; the combine disjoint-applies code and never text-merges a shared file" · [Human] "have the contending sub-tasks touch distinct files, or combine their overlapping changes by hand" · combine.rs:190 · tests:unit · corpus:none

## crates/cli/src/ingest.rs
- (dynamic re-wrap) `{finding.code}` · blocking · `{finding.message}` (re-emits an engine conformance/schema finding as a routed block) · dynamic route · ingest.rs:334 · tests:— · corpus:— (dynamic family)
- `ingest.needs-reconcile` · blocking · "`{rel_path}` does not conform to the `{home.ty}` schema" · dynamic route · ingest.rs:365 · tests:no · corpus:r2b
- `ingest.wrong-location` · blocking · "conformant `{ty}` at `{rel_path}` sits outside `{location}` — relocate to adopt (jigc never auto-moves)" · [Human] "move {rel_path} into {location}, then re-run `jigc ingest`" · ingest.rs:413 · tests:no · corpus:r2b

## crates/cli/src/migrate_corpus.rs (blocked_finding helper migrate_corpus.rs:1016)
- `migrate-corpus.missing-snapshot` · blocking · "`{rel_key}` is stamped schema-version {stamp}, below current, but no prior-schema snapshot `schema-snapshots/{ty}.v{stamp}.yaml` is shipped …" · [Human] "ship the prior-schema snapshot …, then re-run `jigc migrate-corpus`" · migrate_corpus.rs:1030 · tests:unit · corpus:none
- `migrate-corpus.unclassified-change` · blocking · "`{ty}` changed its conformance-relevant structure between schema-version {from} and {to}, but the schema-diff classifies no transform kind …" · [Human] "build the transform kind for the change in `crates/engine/src/schema_diff.rs` + `crates/engine/src/transform.rs`, then re-run `jigc migrate-corpus`" · migrate_corpus.rs:1056 · tests:yes · corpus:none
- `migrate-corpus.narrowed-cardinality` · blocking · "`{ty}` narrows the cardinality of `{section}.{field}` between schema-version {from} and {to} …" · [Human] "restore the wider bound …, or build the narrowing arm …, then re-run `jigc migrate-corpus`" · migrate_corpus.rs:1092 · tests:unit · corpus:none
- `migrate-corpus.removed-field` · blocking · "`{ty}` drops the declared field `{section}.{field}` between schema-version {from} and {to} …" · [Human] "restore `{section}.{field}` to the schema, or build the field-removal (strip) arm …, then re-run `jigc migrate-corpus`" · migrate_corpus.rs:1130 · tests:unit · corpus:none
- `migrate-corpus.prose-needed` · blocking · "`{rel_key}`'s migration mints a new **required** prose slot, which no transform can fill …" · [Human] "author the new required prose in `{rel_key}` through the write verbs, then re-run `jigc migrate-corpus` …" · migrate_corpus.rs:1248 · tests:unit · corpus:none
- `migrate-corpus.destination-collision` · blocking · "`{rel_key}` relocates to `{target}`, which already holds a *different* document …" · [Human] "fold the content of `{rel_key}` into `{target}` through the write verbs, delete `{rel_key}`, then re-run `jigc migrate-corpus`" · migrate_corpus.rs:1266 · tests:no · corpus:none
- `migrate-corpus.deferred` · blocking · "`{rel_key}` was not reached — the fold halts at the run's first blocked doc …" · [Human] "resolve the blocker reported above, then re-run `jigc migrate-corpus` …" · migrate_corpus.rs:1285 · tests:no · corpus:none

## crates/cli/src/milestone.rs
- `milestone.dirty-worktree` · blocking · "milestone:{milestone_id} has uncommitted work in {n} sub-task worktree(s) — discarding it would destroy that work:\n{listing}" · [Human] "get the work out of those worktrees first (commit, stash, or copy it), then re-run `jigc milestone discard {milestone_id}` — or re-run with `--force` …" · milestone.rs:1285 · tests:no · corpus:r2b

## crates/cli/src/doc.rs
- `write.machine-maintained` · blocking · "{verb} rejected: `{target}` is a milestone-record — the record is machine-maintained …" · [Human] "leave the record to the milestone verbs — `jigc milestone create` opens it, …; read it with `jigc doc show`" · doc.rs:395 · tests:yes · corpus:r2b
- `write.id-from-field` · blocking · "set-field rejected: `{field}` is the heading-derived id-from field of item `{item_path}` — its value lives in the item heading, not a field bullet" · dynamic route · doc.rs:706 · tests:yes · corpus:r2b
- `schema-conformance.field-value-conformant` (const ID_FROM_ENUM_CODE) · blocking · "add-item rejected: `{slug}` is not an enum member of id-from field `{repeatable.id_from}`" · [Mechanical] `jigc doc schema <doctype>` + " to see the declared members, then re-run `add-item` with a member title" · doc.rs:1007 · tests:yes · corpus:r2a,r2b
- `write.machine-maintained` · blocking · "retitle-item rejected: item `{item_path}` lives in a milestone-record …" · [Human] "leave the record to the milestone verbs — `jigc milestone add-task` … no manual retitle applies" · doc.rs:1232 · tests:yes · corpus:r2b
- `write.identity-change` · blocking · "retitle-item rejected: item `{item_path}` derives its id from enum field `{repeatable.id_from}` — a member change is an identity change, not a retitle" · [Human] "run `jigc doc remove-item {addr}` then `jigc doc add-item {dest} --title \"{title}\"` under the target category, moving the prose in the same motion" · doc.rs:1356 · tests:no · corpus:r2a,r2b
- `store.unknown-type` · blocking · "unknown doctype `{doctype}`" / "unknown doctype `{ty}`" · [Human] "list the available doctypes with `jigc describe`" · doc.rs:2045/2087 · tests:yes · corpus:r1,r2b
- `write.area-barrier` · blocking · "barrier — the staged destination for `{address}` lands outside task `{task_id}`'s area (`tasks/{task_id}/docs/`); a staging write must stay within its own sub-area" · [Human] "address the doc with a slug inside task `{task_id}`'s own area" · doc.rs:3374 · tests:no · corpus:r2b

## crates/cli/src/start.rs
- `workflow-refs.workflow-mismatch` · blocking · "`jigc workflow {workflow_id} --task {id}` names workflow `{workflow_id}`, but sub-task `{id}` was minted with `{recorded}` — re-entry must compose the recorded workflow" · [Human, Finding::block] "re-run as `jigc workflow {recorded} --task {id}`, or re-seed the sub-task with the intended `--workflow`" · start.rs:1706 · tests:no · corpus:none
- `overrides.project-step-missing` · blocking · "project layer owns step `{id}` but its file {path} is unreadable: {e}" · [Human] "restore the project step file {path}, or drop the project layer's claim on `{id}` …" · start.rs:2199 · tests:unit · corpus:r2b (declared singleton)
- `config.shadow-id-collision` · blocking · "project layer shadows id `{id}` in both `{prior}/` and `{dir}/` — definition ids share one namespace, so one id maps to one file" · [Human] "rename one of the two `{id}.yaml` shadow files …" · start.rs:2501 · tests:no · corpus:none
- `workflow-refs.unknown-workflow` · blocking · "no workflow `{id}` — list the selectable work-workflows with `jigc start`" · [Human] "run `jigc start` to see the selectable work-workflows, then re-run `jigc start --workflow <id> \"<intent>\"`" · start.rs:2932 · tests:yes · corpus:none

## crates/cli/src/task.rs
- `changelog-recording.gate-granted-unused` · advisory · "workflow `{workflow_id}` grants the `changelog` create-gate and this task recorded no changelog entry" · [Human] "if the change is user-facing, record it — `jigc start --workflow record-change \"<what changed>\"` (or, before finalize, in-task: `jigc doc create changelog …`); if it is not user-facing, no action is needed" · task.rs:1501 · tests:yes · corpus:r1,r2b
- `finalize.nothing-staged` · blocking · "you staged nothing — the working tree has changes but the index is empty" · [Human] "`git add` your changes, then re-run `jigc task finalize`" · task.rs:2071 · tests:yes · corpus:r2a,r2b
- `finalize.stage-failed` · blocking · "jigc could not stage its own changes — no commit was made and the promotions were rolled back: {git_error}" · [Human] "resolve the embedded git failure (e.g. remove a stale `.git/index.lock`), then re-run `jigc task finalize`" · task.rs:2097 · tests:yes · corpus:r2b

## crates/cli/src/config.rs (Finding::block → Blocking, Human route)
- `config.undeclared-key` · blocking · "`{key}` is not a settable knob — the cascade surface is closed" · "run `jigc start` to orient; settable knobs are declared by the pack" · config.rs:191 · tests:no · corpus:none
- `config.value-rejected` · blocking · "`{value}` is not a valid value for `{key}`: {reason}" · "set a value matching the knob's declared type" · config.rs:200 · tests:no · corpus:none
- `config.step-id-collision` · blocking · "`{step_id}` is already forked …" / "`{basename}` is already a step id …" · "edit the existing native file (or remove it first) instead of re-forking, then re-run" / "rename the source file so its basename is a fresh step id, then re-run" · config.rs:595/926 · tests:unit · corpus:none
- `config.anchor-absent` · blocking · "no step `{step_id}` body to fork" / "no workflow `{workflow_id}` to anchor against: {err}" / "`{anchor}` is not a step in `{workflow_id}` as of this edit — its include list is: {snapshot}" · "name a step id present in the workflow's resolved include list, then re-run" / "name an existing workflow id, then re-run" · config.rs:614/961/971 · tests:unit · corpus:none
- `config.nested-fill` · blocking · "fill content declares a nested `{{fill: {fill_id}}}` point — phase 5 does not re-run …" · "remove the nested `{{fill:}}` from the content …, then re-run" · config.rs:669 · tests:no · corpus:none
- `config.fill-point-absent` · blocking · "`step:{step_id}#{fill_id}` is not a `{{fill:}}` point in the resolved `{step_id}` step body" / "could not resolve the `{step_id}` step body …: {err}" · "name a `{{fill:<fill-id>}}` point the step body declares …, then re-run" / "name an existing step whose body declares the `{{fill:<fill-id>}}` point, then re-run" · config.rs:703/714 · tests:no · corpus:none

## crates/cli/src/setup.rs (Finding::block → Blocking, Human route; all is_declared_singleton "setup."/"uninstall."; store-version.binary-mismatch also singleton)
- `store-version.binary-mismatch` · advisory · dynamic msg · dynamic route · setup.rs:128 · tests:yes · corpus:r2b (declared singleton)
- `setup.repo-root` · "cannot locate the repository root: {err}" · "run `jigc setup` from inside the target git repository" · setup.rs:695 · tests:unit · corpus:none
- `setup.profile-load` · "cannot load the `{SETUP_ASSISTANT}` adapter profile: {err}" · "reinstall jigc — the embedded adapter profile is missing or malformed" · setup.rs:703 · tests:no · corpus:none
- `setup.spawn-template` · "the `{assistant}` adapter profile's spawn launch template is invalid: {reason}" · `{reason}` · setup.rs:723 · tests:yes · corpus:none
- `setup.profile-incomplete` · "the `{assistant}` adapter profile declares no inject reference floor" · "reinstall jigc — the embedded adapter profile is missing its bootstrap reference" · setup.rs:734 · tests:no · corpus:none
- `setup.write-bootstrap` · "cannot write the managed bootstrap file `{bootstrap_file}`: {err}" · "ensure `{bootstrap_file}` is writable, then re-run `jigc setup`" · setup.rs:749 · tests:no · corpus:none
- `setup.inject-reference` · "cannot inject the bootstrap reference into `{line_file}`: {err}" · "ensure `{line_file}` is writable, then re-run `jigc setup`" · setup.rs:756 · tests:no · corpus:none
- `setup.init-project-layer` · "cannot initialize the project layer under `.jigc/`: {err}" · "ensure `.jigc/` is writable, then re-run `jigc setup`" · setup.rs:765 · tests:no · corpus:none
- `setup.compose-marker` · "cannot write the `compose-embedded-methodology` marker into `.jigc/config/packs.yaml`: {err}" · "ensure `.jigc/config/` is writable, then re-run `jigc setup`" · setup.rs:782 · tests:no · corpus:none
- `setup.version-stamp` · "cannot write the binary-provenance stamp `{VERSION_STAMP_PATH}`: {err}" · "ensure `.jigc/` is writable, then re-run `jigc setup`" · setup.rs:806 · tests:no · corpus:none
- `setup.secrets-gitignore` · "cannot seed the secrets-floor root `.gitignore`: {err}" · "ensure the repository root is writable, then re-run `jigc setup`" · setup.rs:819 · tests:no · corpus:none
- `setup.inject-allowlist` · "cannot merge the allowlist into `{allowlist_file}`: {err}" · "ensure `{allowlist_file}` is writable, then re-run `jigc setup`" · setup.rs:829 · tests:no · corpus:none
- `setup.inject-hook` · "cannot install the session hook into `{allowlist_file}`: {err}" · same · setup.rs:839 · tests:no · corpus:none
- `setup.inject-deny` · "cannot merge the deny safety floor into `{allowlist_file}`: {err}" · same · setup.rs:853 · tests:no · corpus:none
- `setup.install-hook` · "cannot resolve the running `jigc` path to embed in the pre-commit hook: {err}" / "cannot install the `pre-commit` hook into the repo's hooks dir: {err}" · "re-run `jigc setup` (the install resolves its own absolute path)" / "ensure the repo's git hooks directory is writable, then re-run `jigc setup`" · setup.rs:865/874 · tests:yes · corpus:none
- `setup.extract-probe` · "cannot resolve the directory of the running `jigc` to place the `doc-code` probe" / "cannot write the `doc-code` probe beside `jigc` at `{bin_dir}`: {err}" · "re-run `jigc setup` from an installed `jigc` …" / "ensure the directory holding the `jigc` binary (`{bin_dir}`) is writable, then re-run `jigc setup`" · setup.rs:889/897 · tests:yes · corpus:none
- `setup.install-commit` · "the jigc install files were written and staged, but `git commit` was rejected …:\n{git_err}" · "tell git who you are — set `git config user.email …` and `git config user.name …` — then re-run `jigc setup` …" · setup.rs:916 · tests:no · corpus:none
- `uninstall.repo-root` · "cannot locate the repository root: {err}" · "run `jigc uninstall` from inside the target git repository" · setup.rs:1175 · tests:no · corpus:none
- `uninstall.profile-load` · "cannot load the `{SETUP_ASSISTANT}` adapter profile: {err}" · "reinstall jigc — the embedded adapter profile is missing or malformed" · setup.rs:1183 · tests:no · corpus:none
- `uninstall.remove-jigc` · "cannot remove the repo-local `.jigc/` tree: {err}" · "ensure `.jigc/` is writable, then re-run `jigc uninstall`" · setup.rs:1206 · tests:no · corpus:none
- `uninstall.unwire-reference` · "cannot unwire the bootstrap reference from `{line_file}`: {err}" · "ensure `{line_file}` is writable, then re-run `jigc uninstall`" · setup.rs:1222 · tests:no · corpus:none
- `uninstall.remove-allowlist` · "cannot remove the allowlist permit from `{allowlist_file}`: {err}" · "ensure `{allowlist_file}` is writable, then re-run `jigc uninstall`" · setup.rs:1233 · tests:no · corpus:none
- `uninstall.remove-hook` · "cannot remove the session hook from `{allowlist_file}`: {err}" · same · setup.rs:1244 · tests:no · corpus:none
- `uninstall.remove-deny` · "cannot remove the deny safety floor from `{allowlist_file}`: {err}" · same · setup.rs:1254 · tests:no · corpus:none
- `uninstall.remove-precommit` · "cannot remove the `pre-commit` hook from the repo's hooks dir: {err}" · "ensure the repo's git hooks directory is writable, then re-run `jigc uninstall`" · setup.rs:1264 · tests:no · corpus:none

## crates/cli/probes/doc-code/src/main.rs (probe binary; own Finding struct; route Option<String> on the wire)
- `doc-code.<check_id>` (dynamic; ships `symbol-exists`, `criterion-maps-to-test`) · **graded (default blocking** via `validation.doc-code.<check>.severity` knob) · (dangling-file) "anchor `{v}` resolves to no file (`{file}` is absent from {tree})" / (dangling-symbol) "anchor `{v}` resolves to no symbol …" · root-aware Human route: StagedIndex → "if the cited code is on disk but unstaged, `git add` it — finalize adjudicates the staged index, not the working tree; otherwise {restore}"; WorkingTree → "update the citation to match the renamed/moved code, or restore the cited symbol (e.g. revert the change)" · main.rs:167/185/240 · tests:yes · corpus:r2b (symbol-exists only)
- `doc-code.criterion-maps-to-test` (not_a_test) · graded (default blocking) · "anchor `{v}` maps to no test (`{symbol}` in `{file}` is not a `#[test]` fn)" · [Human] "point the criterion at a real test, or correct the cited symbol" · main.rs:199 · tests:yes · corpus:none
- `doc-code.symlink-anchor` · advisory · "anchor `{v}` not validated — `{file}` is a symlink (path-locality not guaranteed)" · "no action needed — the anchor is uncheckable by design …" · main.rs:171 · tests:unit · corpus:none
- `doc-code.unsupported-language` · advisory · "anchor `{v}` not validated — no grammar for `{file}` (uncheckable citation)" · "no action needed — the citation is uncheckable by design (no shipped grammar for this file)" · main.rs:199 · tests:yes · corpus:none

**Part 2 totals:** ~146 distinct finding codes; ~205 rows (multi-site codes carry 2–5 rows each). ~118 rows have zero corpus evidence (the entire `workflow-refs.*`/`structural-target.*`/`slot-fill-target.*`/`setup.*`/`uninstall.*`/`override-default.*`/`migrate-corpus.*` families). ~68 rows have `tests=no`; ~35 are `tests=unit` only.

**Part 2 honesty notes:** (1) `conformance.heading-missing` — no production constructor found at HEAD, only finding.rs test fixtures — flagged as possibly retired. (2) Dynamic code families enumerated by template not instance: `doc-code.<check_id>` (ships symbol-exists + criterion-maps-to-test), `ingest.rs:334` re-wrap of arbitrary engine codes, and the `blocking_conformance`/`store_block`/`block`/`delta_block`/`blocked_finding` helpers that take code as a parameter (all callers traced). (3) Several Blocking findings carry `route: None` and are NOT route-exempt (`finalize.provenance-io`/`promote-io`/`source-path-io`/`render-io`, `milestone.area-io`, `task.working-area-io`, `doc-code.multi-valued-anchor`) — they survive the route-floor because they return on `Result<_, Finding>` abort paths (anyhow/finding_to_err), not through the findings-envelope Serialize seam; confirmed by reading constructors, not by exercising the seam — **audit target**. (4) `render.rs`/`result.rs` construct findings ONLY in `#[cfg(test)]` — no production codes there (their high reference density is Finding *uses*, not constructors). (5) `tests=unit` heuristic keys on assert-near-code-string, so a code asserted only via an insta snapshot body may under-read as `no`.

---

# PART 3 — ERROR / OUTCOME IDENTITIES + anyhow command-span strings

## 3a — the error-code registry (`crates/cli/src/invocation_log.rs`)

The dotted error identities carried on `Outcome` into the invocation log — **not** `Finding`s; route-exempt by construction; a const registry with an anti-collision test. `ERROR_CODE_REGISTRY` at `invocation_log.rs:71`.

- `finalize.commit-rejected` · const `ERROR_COMMIT_REJECTED` `invocation_log.rs:55` · emitted at `task.rs:1180` (`Outcome::error`) + `task.rs:2002` (hook-rejected finalize) · exit code carried via `Outcome::error` · corpus: none
- `migrate.review-pending` · const `ERROR_REVIEW_PENDING` `invocation_log.rs:60` · emitted at `task.rs:1057` (`Outcome::coded_error(4, …)` — the exit-4 migration review-hold) · corpus: none
- Registry membership is asserted at `Outcome::error`/`coded_error` (`invocation_log.rs:98,125`) — a non-member error identity panics; a collision test vs the finding-code inventory is the doc's own medicine (`surface-contract.md` → The error-code namespace).

## 3b — anyhow/bail! strings carrying `jigc` command spans in shipped verbs

Every span rides `engine::finding::Route::mechanical(...)` and is proven parseable by the CLI-seam route fence (`crates/cli/src/route_fence.rs`, installed from `main`). Bounded per `surface-contract.md` → law-2 (errorish surfaces carrying command spans). Test modules excluded from the list; each is exercised by the route-fence parse test at construction.

- **not-set-up** (shared: `describe`/`ingest`/`migrate`/`migrate-corpus`/`upgrade`/compose paths) · `locate.rs:53` (`not_set_up`) · `"this project isn't set up — run \`jigc setup\` (no \`.jigc/config/\` cascade layer found)"` · corpus: none
- **no-such-task** (shared wrong-id convergence) · `task.rs:478` (`no_such_task`) · `"no task \`{id}\` — list live tasks with \`jigc task list\`"` · corpus: r1 (older `jigc start` route text — changed at HEAD)
- **resume: no recorded workflow** · `start.rs:1612` (`resume_in_repo`) · `"task \`{id}\` has no recorded workflow — discard it with \`jigc task discard {id}\` and re-start with \`jigc start\`"` · corpus: none
- **resume: base-pin divergence** · `start.rs:1593` · `"task \`{id}\` is pinned to base {short} but you're on {short} — switch back with \`git checkout {short}\` or \`jigc task discard {id}\`"` · corpus: r2b (`pinned to base`)
- **re-enter: no such sub-task** · `start.rs:1656` (`reenter_in_repo`) · `"no task \`{id}\` — list a milestone's sub-tasks with \`jigc milestone list-tasks <milestone-id>\`"` · corpus: none
- **re-enter: base-pin divergence** · `start.rs:1670` · same template as resume divergence, task-id span · corpus: none
- **re-enter: no recorded workflow** · `start.rs:1692` · `"task \`{id}\` has no recorded workflow — discard it with \`jigc task discard {id}\` and re-seed it with \`jigc milestone add-task <milestone-id> "<intent>"\`"` · corpus: none
- **start: workflow requires an intent** · `start.rs:696` · `"workflow '{workflow_id}' requires an intent: jigc start \"<intent>\" --workflow {workflow_id}"` · corpus: none (note: inline span, NOT via Route::mechanical — flag for law-2 review)
- **task: recorded-workflow missing (workflow_def)** · `task.rs:1408` · `"task at {dir:?} has no recorded workflow — discard it and re-start with \`jigc start\`"` · corpus: none
- **doc: no active task** · `doc.rs:3100` (`resolve`) · `"no active task — start one with \`jigc start\`"` · corpus: none
- **doc: workflow_gate missing workflow** · `doc.rs:3229` · `"the active task has no recorded workflow — discard it with \`jigc task discard {id}\` and re-start with \`jigc start\`"` · corpus: none
- **doc: malformed address** · `doc.rs:3265` (`parse_verb_addr`) · `"malformed address \`{addr}\`: {err} — a doc is addressed as \`<type>:<slug>\`, e.g. \`adr:single-node-cache\` (a singleton doctype … may be named bare)\n  route: run \`jigc describe\` for the doctype surface"` · corpus: r2b (`malformed address`)
- **doc: no staged instance (read_staged)** · `doc.rs:3395,3398` · `"no staged instance for \`{addr}\` — provision it first (\`jigc start\` / \`jigc doc create <type> --title "X"\`). Note: … derives the id from the title …"` · corpus: none
- **rename: no managed doc** · `rename.rs:98` · `"no managed doc \`{ty}:{old_slug}\` to rename (expected at {old_rel})\n  route: check the id (or run \`jigc describe\` for the doctype surface)"` · corpus: r2b (`no managed doc`)
- **rename: cannot reslug a milestone-record** · `rename.rs:174` · `"cannot reslug \`{old_id}\` — a milestone-record's slug IS its milestone work-unit id … only a retitle is supported: pass a \`--to\` title that keeps the slug \`{old_slug}\`"` · corpus: none (no Route span — bespoke prose reject)
- **rename: not a `<type>:<slug>` address** · `rename.rs:540` (`parse_addr`) · `"\`{addr}\` is not a \`<type>:<slug>\` address — e.g. \`adr:single-node-cache\`\n  route: run \`jigc describe\` for the doctype surface"` · corpus: none
- **relocate: frozen doctype refused** · `relocate.rs:144` · `"\`{ty}\` is a frozen doctype — relocate it through the version-gated \`jigc migrate-corpus\`, not the freeze-exempt path"` · corpus: r1, r2a (`frozen doctype`)
- **migrate: unknown / non-migratable doctype** · `migrate.rs:113` (`ensure_migratable`) · `"unknown doctype \`{doctype}\`"` OR `"doctype \`{doctype}\` exists but is not migratable (no \`migrate-{doctype}\` workflow)"` + `"; migratable doctypes: {set}\n  route: re-run \`jigc migrate <path> --as <doctype>\` with one of: {set}"` · corpus: none
- **migrate: unreadable foreign source** · `migrate.rs:191` (`migrate_in_repo`) · `"could not read the foreign \`{doctype}\` source at {path}\n  route: check the path, then re-run \`jigc migrate <path> --as {doctype}\` with a readable file"` · corpus: none
- **milestone: does-not-exist (list-tasks/provision/execute/finalize share)** · `milestone.rs:222` (`no_such_milestone`) · `"milestone \`{milestone_id}\` does not exist\n  route: create it first with \`jigc milestone create "<title>"\`"` · corpus: r2a (milestone create context)
- **milestone: discard does-not-exist (own variant)** · `milestone.rs:1181` (`run_discard`) · `"milestone \`{milestone_id}\` does not exist\n  route: check the milestone id (\`jigc milestone list-tasks <milestone-id>\` names a live milestone's sub-tasks); nothing was discarded"` · corpus: none
- **doc-code probe not found** · `cli.rs:975` + `task.rs:594` · `"\`doc-code\` probe not found at {program:?} — place the \`doc-code\` binary beside \`jigc\` or set \`JIGC_DOC_CODE_PROBE\` to its path"` · corpus: none (env-dependent; unlikely captured)
- **pack: unsupported pack-set** · `pack.rs:805` · `"unsupported pack-set in \`.jigc/config/packs.yaml\`: \`compose-embedded-methodology: true\` cannot be combined with a \`packs:\` list …"` · corpus: none (bespoke prose, no command span)
- **unknown-subcommand tips** (append to clap exit-2 error, `cli.rs:1219` `unknown_subcommand_tip`): two curated entries —
  - `("task","discard-write")` `cli.rs:1237` · `"tip: no per-write discard exists — \`jigc task discard <task-id>\` abandons the WHOLE task …"` · corpus: none
  - `("task","status")` `cli.rs:1247,1252` · `"tip: \`jigc task list\` enumerates the active tasks …; \`jigc task validate <task-id>\` previews the finalize gate …"` · corpus: r1 (`tip: jigc task list`)
- **stale-read hint** (stderr advisory on task-less `doc show` of a staged doc) · `doc.rs:1904` (`stale_read_hint`) · `"note: \`{doc}\` is also staged in open task(s) {ids} — the committed copy served here may be stale; staged read: \`jigc doc show {addr} --task {id|<task-id>}\`"` · corpus: none

> Flag for the law-2 audit: `start.rs:696` builds its command span inline (`jigc start "<intent>" --workflow …`) rather than through `Route::mechanical`, so the route fence does not prove it parses. Every other span above rides the checked constructor.

---

# PART 4 — COMPOSED / GENERATED SURFACES

All rows here were reviewed in r1/r2 passes; listed for completeness with evidence pointers. Compose is deterministic (engine `compose.rs`), rendered per `--format` by `render.rs`.

## 4a — pack workflow composes (33 workflows across dev + methodology packs)

Dev pack (`crates/cli/pack/workflows/`, 16): `architecture-documentation`, `implement-from-spec`, `ingest-existing`, `migrate-adr`, `migrate-arch-doc`, `migrate-changelog`, `migrate-prd`, `migrate-spec`, `milestone-execution`, `plan`, `project-setup`, `quick-fix`, `record-change`, `router`, `single-task`, `sub-task`.

Methodology pack (`packs/methodology/workflows/`, 17): `completion`, `decided-task`, `dev-task`, `do-research`, `form-vision`, `increment`, `migrate-completion-record`, `migrate-decisions-log`, `migrate-deferral-ledger`, `migrate-idea`, `migrate-research`, `migrate-roadmap`, `migrate-vision`, `park-idea`, `planning`, `record-dogfood`.

Generating path: `run_compose*`/`run_resume`/`run_reenter` (cli.rs) → `start.rs::compose_*` → `engine::compose` → `render.rs`. Corpus evidence per workflow (loose substring match on compose command):

| workflow | corpus | | workflow | corpus |
|---|---|---|---|---|
| architecture-documentation | r1, r2b | | migrate-completion-record | r1 |
| implement-from-spec | r1, r2b | | migrate-decisions-log | r1 |
| ingest-existing | r1, r2b | | migrate-deferral-ledger | r1 |
| migrate-adr | r1 | | migrate-idea | r1 |
| migrate-arch-doc | r1 | | migrate-research | r1 |
| migrate-changelog | r1 | | migrate-roadmap | r1 |
| migrate-prd | r1 | | migrate-vision | r1, r2b |
| migrate-spec | r1 | | park-idea | r1, r2b |
| milestone-execution | r1 (r2a: milestone execute) | | planning | r1, r2b |
| plan | r1, r2a, r2b | | record-dogfood | r1 |
| project-setup | r1, r2b | | completion | r1, r2b |
| quick-fix | r1 | | decided-task | r1, r2a |
| record-change | r1, r2b | | dev-task | r1 |
| router | r1 | | do-research | r1, r2b |
| single-task | r1, r2a, r2b | | form-vision | r1, r2b |
| sub-task | r1, r2a, r2b | | increment | r1, r2b |

All 33 have ≥1 corpus capture. `router` also appears as the `jigc start "<intent>"` default compose (r1). The `route-to-workflow.yaml` step (`pack/steps/`) is the router's selection body, not a standalone compose.

## 4b — `{{schema:<doctype>}}` generated projections

Lone `{{schema:<doctype>}}` placeholders resolved by `engine::compose::render_schema_projection` (`compose.rs:1056`, seam at `compose.rs:640`); dangling ref → blocking finding (`compose.rs:655`). Used in 12 migration author-step bodies:

- dev pack: `author-migration.yaml` → `{{ schema:changelog }}`; `-adr`→adr; `-prd`→prd; `-spec`→spec; `-arch-doc`→arch-doc
- methodology pack: `-vision`→vision; `-research`→research; `-idea`→idea; `-roadmap`→roadmap; `-decisions-log`→decisions-log; `-deferral-ledger`→deferral-ledger; `-completion-record`→completion-record

Corpus: rendered projections appear inside the migrate-* composes captured in r1/r2b (4a). Direct `jigc doc schema <doctype>` projection (a separate read surface, `doc.rs::run_schema` → `schema_listing` `doc.rs:2418`) captured in r1 (`doc schema adr`, `doc schema commit`) — that is a Part 1 verb surface, cross-referenced here as the same generation code.

## 4c — the adapter bootstrap (`.jigc/AGENT.md`)

Generated by `adapter::write_bootstrap_file` (`adapter.rs:477`); canonical routing sentence at `adapter.rs:1022`; profile bytes from embedded `crates/cli/adapters/claude-code.yaml`. Corpus: r1 (`cat .jigc/AGENT.md`, verbatim capture).

## 4d — `jigc setup` install output

`run_setup` (`cli.rs:409`) → `setup.rs` (install-commit + reference/hook/gitignore wiring). Corpus: r1 (`jigc setup` in a fresh repo). Uninstall (`run_uninstall` `cli.rs:437`) — enumerated as a Part 1 verb outcome.

## 4e — `jigc describe` menu projection

`run_describe` (`cli.rs:381`) → `describe.rs` whole-menu projection (pack-only resolved workflow + doctype + command-catalog sets), rendered per `format`. Explicitly a non-stable human menu ("don't parse it"). Corpus: r1 (`jigc describe`).

---

## Coverage summary

**Enumerated at HEAD `12bf692`.** The three capture corpora (r1/r2a/r2b) predate this HEAD by **46 commits** (not the ~11 the brief estimated), so routes and command spans were re-derived from source, not trusted from captures.

**Row counts per part:**
- PART 1 — verb outcome surfaces: **205 rows** (1A core verbs 97 + 1B operator/store verbs 108). Zero corpus evidence: **146** (1A 68 + 1B 78).
- PART 2 — finding surfaces: **~205 rows across ~146 distinct finding codes** (multi-site codes carry 2–5 rows). Zero corpus evidence: **~118**. `tests=no`: **~68**; `tests=unit`-only: **~35**.
- PART 3 — error/outcome identities: **2 error-code registry members** + **24 anyhow/bail! command-span rows**. Registry: 0 corpus. Command-spans: 6 with corpus, 18 zero.
- PART 4 — composed/generated surfaces: **33 workflow composes** (all ≥1 corpus) + **12 `{{schema:}}` projection uses** (rendered inside migrate composes) + **AGENT.md** (r1) + **setup** (r1) + **describe** (r1) = **48 rows**, all reviewed-r1/r2.

**Total across all parts: ~460 distinct output-surface rows.** Rows with zero corpus evidence: **~266** (PART 1 146 + PART 2 118 + PART 3 18 − overlap where a Part-2 finding also underlies a Part-1 `blocked-findings-envelope` row; the two axes deliberately overlap — Part 1 counts the verb *state*, Part 2 the finding *code* rendered inside it).

**Load-bearing cross-cutting findings for the audit:**
1. `Format::Human` has NO custom rendering anywhere — every `render.rs` arm pairs `Agent | Human`; only `json` diverges. `doc show` / `doc schema` are the only two verbs whose `json` is a separately-pinned *contract* (not a mere projection).
2. `render::operational_error` is the single funnel for the vast majority of refusals — distinct text per site, one format contract.
3. Stream asymmetry: `task finalize` / `task validate` / `milestone finalize` route their `json` block-report to **stdout** but `agent`/`human` message+route to **stderr**. `milestone join`'s blocking-clash and several advisory/warn/note lines are **agent-only stderr** (bypass render).
4. `config-*+success` (all 6 config verbs) **print nothing** — silent success, audit-relevant as a surface-law concern.
5. **Route-floor audit target:** ~7 Blocking findings carry `route: None` and are NOT route-exempt (`finalize.provenance-io`/`promote-io`/`source-path-io`/`render-io`, `milestone.area-io`, `task.working-area-io`, `doc-code.multi-valued-anchor`) — they survive only because they abort via `Result<_, Finding>` (anyhow), never reaching the findings-envelope Serialize seam. Confirmed by reading constructors, not by exercising the seam.
6. **One law-2 gap flagged:** `start.rs:696` builds its `jigc start "<intent>" --workflow …` command span **inline**, not through `Route::mechanical` — so the route-fence parse check does not cover it. Every other command span rides the checked constructor.

**Could not confidently enumerate / low-confidence areas (stated honestly, not padded):**
- `conformance.heading-missing` — no production constructor found at HEAD, only in finding.rs test fixtures; possibly a retired code.
- **Dynamic finding-code families** are enumerated by template, not by every instance: `doc-code.<check_id>` (ships two: symbol-exists, criterion-maps-to-test; a pack could declare more), the `ingest.rs:334` re-wrap of arbitrary engine codes, and the parameterized helper constructors (`blocking_conformance`/`store_block`/`block`/`delta_block`/`blocked_finding`) — every literal caller was traced, but a code minted only on an untaken branch could be missed.
- `tests=unit`/`tests=no` split uses an assert-near-code-string heuristic; a code asserted only via an `insta` snapshot body (not naming the code literally) may under-read as `no` — a few `setup.*`/`workflow-refs.*` rows may be undercounted on the test axis.
- PART 1 verb rows collapse each verb's generic engine-finding block path into one `blocked-findings-envelope` row (the individual codes are PART 2's scope); bespoke CLI-minted refusals are listed individually. So PARTs 1 and 2 are two intentionally-overlapping axes over the same rendered surface, not disjoint sets.
- `orphan.rs` and `route_fence.rs` were checked and generate no user-facing text of their own (pure detection / parse-assert helper) — correctly excluded.
