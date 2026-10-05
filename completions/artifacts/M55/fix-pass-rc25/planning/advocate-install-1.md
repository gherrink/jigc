# Robust-case advocacy for fork install-1

## robust_case

**Position: refuse.** It is the only arm that restores the contract as published; the narrow arm amends the contract to fit a reduced behaviour. I spiked the robust delta in a scratch clone and it lands in about 20 lines with the affected suites green.

**The contract of record already covers this file.**
- `jigc uninstall --help` on rc.24: "any other file under `.jigc/` that no index has a copy of ... blocks with `uninstall.untracked-workbench-file`".
- `crates/cli/src/setup.rs`, `uninstall`'s doc-comment (line 2998): "unless `force`, it removes nothing at all while `.jigc/` holds the sole copy of anything".
- `design/project-setup.md` → Teardown / cleanup (G5), (c): "Authorship is not a discriminator here either: jigc's own writes are not exempt from the rule jigc's own guards run on".
- `design/worked-examples.md`, flow 52 arm 2: "No destroying door destroys bytes it does not name, and none takes a path it never looked at."
- `VISION.md` line 51: "discard is explicit, never silent. No silent data loss."

The log meets every term of the guard's ground: jigc-written, sole-copy, in no index, and not rebuildable. `design/storage.md` line 9 declares only "the staging working area, the edge index, the file↔state hashes" disposable; it does not name `logs/`. `design/measurement.md` → The in-repo invocation log calls it "a product instrument" for adopters, with no rotation.

**The project has made this exact trade three times, each time for refusal over jigc-authored bytes at the dominant cell.**
- `manifest.yaml`: G5 (c) says an `uninstall` after any `config set` refuses.
- The pristine skeleton `jigc start` mints: G5 (b) says "the skeleton is in fact the dominant cell, and it refuses over both on the one ground it can prove".
- `DECISIONS.md` line 7341 (i): "an ordinary `jigc task discard <id>` now needs `--force` from the moment the task is minted".

Taking the log would be the first jigc-written, non-rebuildable sole-copy file the door takes without consent.

**The "visible, not prevented" bound does not transfer.** Its recorded reason (`DECISIONS.md` line 8792; flow 52 arm 2) is that a refusal would fire "on the ordinary fan-out success path" over rebuildable build output and so train `--force` into reflex. Here the refusal fires only behind a default-off knob, at a human-only door run about once per repository, over bytes that cannot be rebuilt, and the exit that keeps the data needs no `--force` (one `mv`).

**The newly refusing population is smaller than the planner's cost line says.** Driven on the installed rc.24 (fresh rig): `config set invocation-log true`, two reads, `jigc uninstall` already exits 1 with `uninstall.untracked-workbench-file` over `.jigc/config/manifest.yaml`. So an adopter who turned the knob on at the project layer and did not stage the delta is refused today. Robust adds a second line to a refusal they already get. Only adopters who staged or committed the delta, or set the knob at the team layer, see a command that used to succeed now refuse.

**Robust is the reversible direction across the 1.0.0 pin.** Relaxing refuse to take in 1.x breaks no one. Tightening take to refuse after 1.0.0 makes a published, previously succeeding command refuse. The declared-gap precedent at this same door went badly: `workbench_paths`' doc-comment (`setup.rs` lines 3383-3391) records that the `displaced/` gap was "declared ... rather than closing it" at M50, and "the declaration stopped holding" by M52, when it was fixed as a loss.

**Spike evidence.** Scratch clone at `<scratch>/adv-install1.pvDRr8/src`; the repository itself is untouched. Three changes: `append_record` returns early when `.jigc/` is absent, `workbench_paths` admits `logs/invocations.jsonl`, and the route branches for a gitignored path. Driven on the debug build over `dev/jigc-rig fresh`:

| Cell | Result |
|---|---|
| Log on, config committed, `jigc uninstall` | exit 1, `uninstall.untracked-workbench-file` names `.jigc/logs/invocations.jsonl`, log intact (6 → 7 records, the refused run logs itself) |
| Route followed (`mv` the log out), re-run | exit 0 in one run, `.jigc/` absent, no residue, all 7 records kept |
| Third run | "(nothing to remove ...)" — `(R9, F3)` closed |
| `uninstall --force` | exit 0, the existing "destroys 1 file(s) ... not recoverable" warning names the log, no residue |
| `--format json` refusal | findings envelope with `key.code` `uninstall.untracked-workbench-file` |
| Knob off (control) | unchanged |

Suites run against the spike, all exit 0: `g_milestone` (`staged_prose_consent_axis`, `uninstall_workbench_subject`, `cwd_verb_subject`) 36 passed; `g_flow` (`flow48`, `flow50`, `flow52`, `flow53`, `format_json_success_axis`, `help_truth`) 84 passed; `g_migrate` (`adapter_artifact`) 16 passed; `g_methodology` (`invocation_log`) 12 passed; lib filters 7 passed. The five suites the planner says must be re-read do not move.

**Verdict: robust-now.** It is the contract as written, proven buildable, and the cheaper direction to undo.

## what_the_narrow_option_leaves

**The reachable loss.**
- State: `invocation-log` true at project or team layer, with N records in `.jigc/logs/invocations.jsonl`. Each record carries `argv`, which includes the operator's intent text.
- Door: `jigc uninstall` with no `--force`.
- Result: exit 0 and all N records gone from every place. Rig B in `completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R9-F5.md` §1.2 shows 5 records before and the earlier argv found nowhere after.

**The naming comes after the bytes are gone, by construction.** `pending.narrate_taken` runs after `remove_dir_all` (`setup.rs` lines 3067-3069). It is stderr prose only. Under `--format json` the stdout envelope still reads `"findings": []`, so a driver has no machine-legible sign of the loss.

**Four further things the narrow arm leaves or creates.**
1. **The "not recoverable" warning becomes reachable without consent for the first time.** `pending_teardown`'s own comment (`setup.rs` lines 3745-3747) says the untracked half "reaches here only under `--force` (the guard refuses on it otherwise)". Narrow makes the door print, on an unforced exit 0, the same phrase ("no index has a copy of") that its help gives as the ground for refusing.
2. **The refusal's listing stays incomplete.** Driven on rc.24: with the config delta uncommitted, the refusal says "`.jigc/` holds 1 file(s) that no index has a copy of" and lists only `manifest.yaml` while the log sits beside it. Following the printed route (`git add`) and re-running takes the log. Under narrow the refusal and the narration would use the same phrase over two different sets — a law-1 disagreement (`design/surface-contract.md` → Law 1) inside the fix's own new code.
3. **`(R9, F3)` stays open.** Driven on rc.24: the run that would announce the log destroyed and unrecoverable leaves `.jigc/logs/invocations.jsonl` behind with one record. The ack "removed .jigc/" is false. The residue is un-ignored (`?? .jigc/logs/`), so a later `git add -A` stages it. The second run acts again and, under narrow, would warn again that it is destroying a log whose only content is the first uninstall's record.
4. **A new declared loss at 1.0.0.** The help's universal is replaced by a carve-out. "Authorship is not a discriminator" gains its first exception for non-rebuildable bytes.

**Would it survive the re-review?** Not on the predicate's letter. A re-reviewer driving knob-on then `uninstall` still finds exit 0, no consent flag, and sole-copy bytes present before and absent after, through a `DESTROYING_DOORS` member. The verifier already wrote (R9-F5.md §5) that the log member "standing alone I would still tier 1 on the letter". Narrow clears the exit rule only if the human rules in advance that a named, declared loss of jigc's own telemetry is not tier 1. That is a decision the human can honestly take, but it is a ruling, not a fix. Items 1-3 would land as tier-3 rows; they do not block the call.

## what_the_robust_option_really_costs

**Code.** About 20 lines in two files in the spike: `crates/cli/src/invocation_log.rs` `append_record` and `crates/cli/src/setup.rs` `workbench_paths` plus `untracked_workbench_finding`.

**Tests.** The log cell (refuse, route followed verbatim, re-run lands, third run is a no-op, `--force` names it), a mixed cell (log plus an addable path), and a unit for the append condition.

**Real costs, not softened.**
1. **A previously succeeding command refuses.** This hits adopters who turned the knob on and staged or committed the config delta, or who set it at the team layer. Once per repository, cleared by one `mv` or `rm`.
2. **A successful teardown no longer logs itself.** `crates/cli/guides/QUICKSTART.md` lines 146-148 say "every jigc run ... appends one record". That becomes false for exactly one run. The sentence needs a carve-out in the pass's single guide batch (which moves `jigc-body-blake3`, already planned), and `design/measurement.md` needs the same. Refused runs are still logged. A narrow arm that also closed `(R9, F3)` would owe this same carve-out; the planner's narrow arm does not, because it leaves the residue.
3. **Any jigc run between the `mv` and the re-run re-creates the log and refuses again.** That includes the pre-commit hook's `validate` on a `git commit`. The route should say to re-run directly.
4. **The route needs a decision for the mixed population.** My spike collapsed to "move out or delete" whenever a `.jigc/logs/` path is listed, which drops the `git add` exit for `manifest.yaml` beside it. That is true but not the cheapest exit. The fixer must choose and pin it. The help's parenthetical "(`git add <path>` is enough ...)" needs the same qualifier.
5. **The message must not quote a record count.** The refused run appends its own record (driven: 6 → 7).
6. **It composes with the planner's foreign-bytes fix.** The log must sit in jigc's own-file row so guard two does not call it foreign, and in guard four's subject so it refuses. That is one row with two readers, and it needs the planner's own-row completeness control.

**Risk of a new tier-1 inside this code.** Low by construction. Both additions are a refusal and a skipped write. A bug yields an over-refusal that `--force` clears, or a missing record. Neither yields an exit-0 loss.

**What I did not drive.** The full gate (only the named suites). The release-posture binary. The team-layer knob. The robust delta composed with the planner's `(R9, F5)` foreign-bytes change. The fixer's final route wording.

**Severity, stated plainly.** This is the low end of tier 1: telemetry, default off, a human-only door, no authored doc and no repository harm. If the human holds that "loss" means authored work only, the narrow arm with its declaration written is a legitimate choice.

## would_it_be_new_mechanism

**No.** It fits the fix-pass boundary as written in `implementation/decisions-pending.md` line 95: "a row in a registry that already exists, or a condition on a guard that already exists — no new disposition, no new family, no new registry, no new finding code".

- **Code and codes:** the existing `uninstall.untracked-workbench-file`; `UNINSTALL_DOOR.codes` stays four. No knob, store, doctype, JSON key, schema hash or contract version moves.
- **Guard:** the existing fourth guard and the existing `workbench_paths` derivation admit one more path. `classify_workbench_paths` already classifies a gitignored path as untracked; the spike confirmed it with no change there.
- **Narration:** the existing `--force` warning names the log with no change (driven).
- **Route variant under one code:** there is precedent at this door. `unverified_workbench_finding` (`setup.rs` lines 3694-3705) already carries a different route under the same code, and the plan's own `(R1, F1)` fix gives `setup.dirty-install-path` an unborn-HEAD route in this same pass.

**The one piece that is not literally a guard condition** is the early return in `append_record`. It is a condition on an existing writer, with no surface, and it closes the recorded row `(R9, F3)`. Its behavioural reach is one verb: `enabled_logs_dir` (`invocation_log.rs` lines 468-492) requires `.jigc/config/` at start, so `.jigc/` can only be absent at append time when the run removed it. R9-F5.md §7 cites `completions/artifacts/M52/baseline-destroying.md` §1.2 for `uninstall` being the only whole-workbench sink; I did not re-verify that independently.

**The narrow arm is the one that changes the stated contract.** It adds a newly declared loss and a carve-out to the published help, where the boundary asks a fix pass to restore a contract the product already states.

## verdict_for_the_human

You are choosing whether 1.0.0 ships the uninstall contract as written — nothing leaves `.jigc/` unconsented while it is the sole copy, jigc's own log included, at the price of one `mv` for opted-in adopters and one sentence saying a successful teardown no longer logs itself — or ships a newly declared exception under which the one non-rebuildable file jigc writes is taken at exit 0 and named afterwards. The robust arm is spiked, green on the affected suites, not new mechanism, and can be relaxed in 1.x without breaking anyone; the narrow arm passes the exit rule only if you rule in advance that this loss is not tier 1, and it leaves `(R9, F3)` and two surface disagreements open.

