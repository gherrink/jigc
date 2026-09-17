Verdict: **not ready to decompose**.

## BLOCKING

1. **D1’s central reuse claim is false: the CAS restore body cannot remain unchanged.**

The decision says the M51 CAS will be generalized while “the ~150-line `restore` body … [is] unchanged” and simultaneously makes the finding code door-specific ([settle-record.md:96](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:96), [settle-record.md:102](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:102)).

The implementation is not generic:

- Its type is `ConfigLayerWorktree` over `ConfigLayerPreImage` ([task.rs:4092](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4092), [task.rs:4123](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4123)).
- `restore` calls `rollback_conflict_finding(repo_root, entry, ...)` with that concrete type ([task.rs:4192](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4192), [task.rs:4231](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4231)).
- The helper hard-codes `finalize.rollback-conflict` ([task.rs:4301](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4301)).
- Its route repeatedly says “this finalize” ([task.rs:4286](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4286), [task.rs:4297](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4297)).
- Parking uses only the last path component, not the proposed runtime family identity ([task.rs:4253](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4253)).

Changing only `spec` and `path` cannot produce per-door codes, per-door wording, collision-safe identities, or the different population ownership semantics. The generic mechanism must be designed explicitly—probably a generic pre-image entry plus injected door/finding metadata—and the record must stop promising an unchanged body.

2. **D6 collapses the two declared reject arms into an ambiguous wire shape and contradicts `ArmRoot`.**

D6 says `Reject::Error` gains optional `findings` and `schema_version` ([settle-record.md:321](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:321)). Today the registry deliberately distinguishes:

- `Reject::Error`: `{"error"}`, ad-hoc root ([render.rs:6001](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6001)).
- `Reject::Findings`: `{"findings","schema_version"}`, result-contract root ([render.rs:6014](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6014)).

The registry also states that an ad-hoc root “carries no `schema_version`” because that contract does not version it ([render.rs:5157](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5157)). D6 therefore is not merely additive: it makes the error arm partly impersonate the findings arm and puts a result-contract version on an explicitly non-result-contract root.

The pre-pin rule permits a declared addition or a door moving between already-declared arms; it does not cure contradictory arm semantics ([command-output-contract.md:476](/Users/maurice/projects/gherrink-jigc/design/command-output-contract.md:476), [command-output-contract.md:488](/Users/maurice/projects/gherrink-jigc/design/command-output-contract.md:488)). The clean design is to keep `Reject::Error` unchanged and route errors carrying findings onto `Reject::Findings`, or declare a genuinely distinct third arm with an unambiguous discriminator and version owner.

3. **Acceptance arm 3 contains argv cells the settled design expressly refuses to create.**

The arm declares:

> `TASK_AREA_FILES` complement × `DESTROYING_DOORS` × `{no --force, --force}`

([acceptance-design.md:21](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/acceptance-design.md:21)).

But `DESTROYING_DOORS` is expanded to include `task finalize` ([settle-record.md:217](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:217)), while the record explicitly refuses “A `--force` on `task finalize`” ([settle-record.md:464](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:464)). Thus the asserted Cartesian product includes an invalid `task finalize --force` cell.

Split the axis into:

- consenting doors × `{without --force, with --force}`;
- non-consenting displacement doors × their actual modes.

Until then, flow 53 is not implementable as written.

4. **Acceptance arm 6 does not define a realizable product or expected-arm mapping.**

It proposes every pre-dispatch failure point × all 47 verbs and says every cell emits “the declared arm” ([acceptance-design.md:24](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/acceptance-design.md:24)). But the failure-point registry does not yet exist, is illustrated with an ellipsis, and different failures occur at incompatible phases:

- deleted CWD occurs before repository discovery;
- malformed `packs.yaml` requires a door that loads packs;
- unreadable `.jigc/` requires setup state and filesystem access;
- some leaves do not load a pack or project layer at all.

No applicability predicate, fixture constructor, or `(failure point, verb) → Reject arm/exit` mapping is specified. “Every leaf × every failure point” is therefore either impossible or forces artificial code paths unrelated to production. This arm needs a code-side applicability relation and an exhaustive expected-shape table before decomposition.

## SIGNIFICANT

1. **The task-validate posture change exceeds the locked preview boundary without updating all owning text.**

The contract says `task validate` previews checks “up to the commit,” while preflight and other transaction phases are deliberately excluded ([command-output-contract.md:470](/Users/maurice/projects/gherrink-jigc/design/command-output-contract.md:470)). `finalize.md` likewise says it previews the shared validation phase, not the phases around it ([finalize.md:55](/Users/maurice/projects/gherrink-jigc/design/finalize.md:55)).

D2.6 adds repository posture to `preview_gates` and calls the exit flip deliberate ([settle-record.md:170](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:170)), but D2.7 only names updates at `finalize.md:31` and `validation.md:663`; it does not disposition the stronger preview-boundary declarations. Either classify posture as part of the shared phase and revise those locked statements, or keep it outside `preview_gates` and invoke it separately at the validate door.

2. **The displacement reuse claim is only an idiom, not a transferable implementation.**

`displace_foreign_squatter` does exist and uses `fs::rename` ([relocate.rs:283](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:283), [relocate.rs:322](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:322)). But it:

- classifies one destination using the file-state baseline;
- frees an index slot with `git rm --cached`;
- parks by basename into a flat directory ([relocate.rs:300](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:300), [relocate.rs:304](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:304));
- can collide when two source paths share a filename.

D3 requires a recursive task-area complement, relative-path preservation, no index manipulation, and multiple entries under `.jigc/displaced/<task-id>/` ([settle-record.md:209](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:209)). Only the same-filesystem rename technique transfers. The plan should say a new tree-preserving displacement primitive is required.

3. **The MintedSet premise holds, but the error contract is incomplete.**

The mint really writes exactly three constant files after creating the directory ([state.rs:633](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:633), [state.rs:649](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:649)). Current unwind really is destructive `remove_dir_all` ([milestone.rs:852](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:852)), and non-recursive `remove_dir` is a valid empty-directory guard.

However, the existing idiom merely stops on any `remove_dir` error; it does not classify `ENOTEMPTY` ([task.rs:4897](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4897)). D1 specifies a finding only for `ENOTEMPTY`, leaving permission errors, file-vs-directory replacement, and failures deleting one of the three owned files without a defined finding/route. The promised “no byte dies” family needs those branches dispositioned.

4. **The fixed-identity funnel claim mostly holds, but schema resolution is missing at three cited doors.**

`parse_verb_addr` exists, holds the pack/config inputs, and has exactly nine `doc.rs` call sites ([doc.rs:6120](/Users/maurice/projects/gherrink-jigc/crates/cli/src/doc.rs:6120)). It currently expands singleton syntax, parses, and applies the malformed-slug guard ([doc.rs:6121](/Users/maurice/projects/gherrink-jigc/crates/cli/src/doc.rs:6121)). It is a sound insertion seam for those nine doors.

The three siblings are not equivalent funnels. For example, `task bind` parses and checks malformed slug before loading schemas ([task.rs:2682](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2682), [task.rs:2694](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2694)); milestone add-from-spec only performs the syntax guard at the cited line and resolves schemas later ([milestone.rs:1397](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:1397), [milestone.rs:1431](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:1431)). Ordering must require schema resolution before the new predicate without moving mutation or changing existing unknown-doctype precedence.

This does not repeat M50’s rejected `Address::parse` design: D5 explicitly leaves the grammar untouched, so that alleged contradiction does not exist.

5. **`projection_home_line` covers the home cases but cannot directly supply a structured JSON fact.**

The helper exists and handles placement, located singleton, located collection, and transient shapes ([compose.rs:1191](/Users/maurice/projects/gherrink-jigc/crates/engine/src/compose.rs:1191)). Its output is prose, however, and it is private. D5 requires a driver-facing identity/home fact in `doc schema`, not another prose string. The reusable part is its branching logic; the implementation needs a structured projection primitive from which both prose and JSON derive. Parsing this sentence would violate the repository’s general structured-surface posture.

6. **D9 leaves the new `suppressed` key materially unspecified.**

The existing fence is real: `Suppressed` has exactly required `reason` and `expires` fields ([compose.rs:77](/Users/maurice/projects/gherrink-jigc/crates/engine/src/compose.rs:77)), and malformed/missing values produce a blocking pack-load finding ([compose.rs:173](/Users/maurice/projects/gherrink-jigc/crates/engine/src/compose.rs:173)). D9 only says the new key’s “name [is] owed to the plan” and its value is argv ([settle-record.md:390](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:390)).

Before decomposition it must settle:

- key name and YAML type;
- optional versus required;
- whether it is legal only with `reason: verb-routed`;
- argv token array versus shell string;
- malformed-key code, severity, route and exit;
- behavior for project-layer workflows and unknown-key tolerance.

7. **Acceptance arm 4 pins the reported home-vacated repro instead of iterating that class.**

The arm exhaustively iterates `{location, placement}²` for corpus migration, but its `home-vacated` assertion is only “a `git mv`’d home on the fresh-clone shape” ([acceptance-design.md:22](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/acceptance-design.md:22)). D7’s rule is quantified over “each resolved doctype home” ([settle-record.md:349](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:349)). It needs its own resolved-home axis: location directory, placement root file, location singleton, non-singleton directory, never-had-history control, partially populated home, and index-vacated/worktree-present variants as applicable.

8. **Several new finding codes are not fully registered.**

Fully specified:

- `<door>.rollback-conflict`: door reject, blocking, per-path, reject envelope; Human route ([settle-record.md:102](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:102)).
- `<door>.foreign-bytes`: destroying-door subject, blocking, `--force` route/consent ([settle-record.md:205](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:205)).
- `store.fixed-identity`: read/write address boundary, blocking, canonical-address route ([settle-record.md:284](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:284)).
- `schema-conformance.home-vacated`: store scope, blocking, exit 1 via `STORE_EXIT_FLIPS`, state-dependent route ([settle-record.md:349](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:349)).
- `workflow.verb-routed`: compose doors, blocking, Mechanical route, exit 1 ([settle-record.md:390](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:390)).

Under-specified:

- `migrate-corpus.missing-snapshot`: blocking is stated, but target/key, route kind, exact route, envelope arm and exit are not ([settle-record.md:248](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:248)).
- `file-state.absorbed`: advisory and two doors are stated, but target, stable key, route kind/content, output arm and exit behavior are not ([settle-record.md:411](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:411)).
- MintedSet’s conflict finding has severity and Human route but no stable code, target form, envelope arm, or precise exit ownership ([settle-record.md:107](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:107)).
- `displaced` is an envelope key, not a finding. Its value shape, ordering, relative-path representation and empty-case presence are not declared despite becoming one-way wire surface ([settle-record.md:209](/Users/maurice/projects/gherrink-jigc/completions/artifacts/M52/settle-record.md:209)).

## ADVISORY

1. **The `current_dir()` reuse claim is plausible, but “24 one-line substitutions” is too strong.**

`operational_failure(format, &err)` is already the common rendering seam at numerous dispatches, and the cited functions generally hold `format`. But `refuse_on_posture` currently returns `Option<Outcome>` and turns `current_dir()` failure into `None` ([cli.rs:526](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:526), [cli.rs:540](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:540)). Closing that skip changes control flow, not merely one expression. Preserve D6-before-D2.4 as an explicit dependency.

2. **The history predicate genuinely transfers, subject to a conservative-failure decision.**

`git_path_has_history` is exactly `git log HEAD -1 -- <path>` and returns whether output is non-empty ([task.rs:5268](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:5268)). It works for a vacated declared path. The existing consumers convert git failure to “history present,” conservatively blocking ([task.rs:1835](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:1835)); D7 should state whether `home-vacated` inherits that behavior.

Joining `STORE_EXIT_FLIPS` does not itself reopen M51’s orphan territory: one check concerns a still-resolved declared home; the other concerns a stamped file no resolved doctype claims. The distinction should remain explicit.

3. **`locate::not_set_up()` exists and is reusable, but it is a bare operational error.**

The helper returns an `anyhow::Error` with a mechanically rendered `jigc setup` route ([locate.rs:45](/Users/maurice/projects/gherrink-jigc/crates/cli/src/locate.rs:45)). Reusing it at milestone doors gives consistent prose, but not a finding code or findings envelope. D8 should state whether LD-3 deliberately remains on `Reject::Error`.

4. **The zero-schema-hash claim holds on the settled scope.**

`schema_hash` serializes an erased-presentation projection of the doctype `Schema`, whose exhaustive destructure includes all semantic fields ([manifest.rs:52](/Users/maurice/projects/gherrink-jigc/crates/engine/src/manifest.rs:52), [manifest.rs:71](/Users/maurice/projects/gherrink-jigc/crates/engine/src/manifest.rs:71)). The proposed workflow `suppressed` metadata changes `WorkflowDef`, not doctype `Schema`; `doc schema` contract-version 6→7 is a read-contract bump; and the `milestone-record.base` action is described as correcting prose to the already-shipped schema. I found no settled instruction to change a doctype schema, manifest version, schema snapshot, or corpus bytes.

The decomposition should nevertheless fence this negatively: no changes under pack schema YAML, schema manifests, or schema-snapshot fixtures except the expressly named documentation correction.

5. **The bounds are mostly deferrals and should be labelled as such.**

- Git 2.54 marker knowledge is an unfenced compatibility deferral, not closure.
- `GIT_DIR` remains an explicit posture deferral.
- Departed-doctype/root-placement orphan detection remains an explicit deferral until namespaced stamps.
- “Displacement behavior reversible, key one-way” is accurate versioning posture, but spending the key before its value shape is fully specified is unsafe.

6. **Required decomposition order is stronger than the record currently states.**

At minimum:

1. Settle reject-arm design and `ENVELOPE_ARMS`/`ArmShape`.
2. Build the pre-dispatch funnel.
3. Add D2.4 and all reject-path finding folding.
4. Define generic CAS metadata and `ROLLBACK_POPULATIONS`.
5. Add per-door rollback implementations.
6. Define the fixed-identity engine predicate.
7. Wire `parse_verb_addr`, then the three schema-separate doors, then `rename --slug`.
8. Define `TASK_AREA_FILES` and displacement value shape before teardown behavior or its landed key.
9. Extend the `suppressed` format/fence before enforcing `workflow.verb-routed`.
10. Repair the acceptance axes only after those registries and applicability relations exist.

The two blocking wire/acceptance contradictions and the false CAS reuse promise need to be resolved before an increment plan can be trusted.