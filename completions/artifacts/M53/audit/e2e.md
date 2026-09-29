# M53 completion audit — e2e (the built debug binary at `f664863a`, throwaway dev/jigc-rig repos)

**overall_pass:** false

## Summary

All five M53 acceptance arms hold end-to-end through the real debug binary (jigc 1.0.0-rc.16 @ f664863a) in throwaway `dev/jigc-rig` repos: the unslugable-title guard refuses every mint door over all five degenerate titles before any write; the tenth `InProgress` member refuses all 12 acting `BEHALF_DOORS` rows in all four cherry-pick states with MERGE_MSG, index and HEAD byte-unchanged and the abandon route runnable verbatim; a pin-less directory is a work unit at none of the 25 `WORK_UNIT_ID_DOORS` across 3 shapes × 3 placements with no host path leaked; all six foreign shapes under a milestone area are displaced, refused or narrated at every `DESTROYING_DOORS` member and survive a blocked boundary; and a failed displacement keeps every un-moved byte with the area's count, on `findings` at the task door and stderr-only at the boundary whose envelope stays exactly `Object(["committed"])`. The join is order-invariant: a 3-process fan-out driven in three divergent orders committed byte-identical output (shasum cad3c78c… ×3). FAILURE HEADLINE: jigc's own `finalize-message.tmp` (crates/cli/src/task.rs:3961, a string literal, in neither `TASK_AREA_FILES` nor `MILESTONE_AREA_FILES`) is classified as a byte jigc did not write whenever its best-effort removal fails under this pass's own in-transaction hook racer — producing a false `finalize.foreign-bytes` advisory at both finalize doors and blocking `task discard` and `uninstall` at exit 1 over jigc's own file, with no third-party byte anywhere in the repro; the planning saw the name (DECISIONS.md:408) and disposed it on "it is removed before phase 7", which the pass's own fault model falsifies, and the shipped flow54 suite is green (12 passed) over it. I explicitly flag that the genuine concurrent Task-tool spawn is the orchestrator's main-session artifact I did NOT run — my spawn evidence is an N-process binary sim plus the rendered template executed verbatim, which is not that proof.

## Scenarios

### PASS — Arm 5 — a title that yields no id mints nothing (MINT_DOORS ∪ milestone add-from-spec × five degenerate titles)

Repro (all from repo root, debug binary at /Users/maurice/projects/gherrink-jigc/target/debug/jigc):
  out=$(dev/jigc-rig fresh) || exit; eval "$out"; cd "$REPO"
  for t in "" "   " "!!!" "日本語" "the of a"; do "$JIGC" milestone create "$t" --format json; echo exit=$?; done
Observed, all five identical: exit=1, {"error":"blocking · write.unslugable-title — cannot mint a milestone: its id is slugged from the title, and this title slugs to nothing — ids are built from ASCII letters and digits, so a title in another script, or of stopwords only, yields none\n  at: milestone\n  route: re-run with a title carrying ASCII letters or digits …"}. `git log` unchanged (2 commits), `.jigc/milestones` never created, `git status --porcelain` empty.
  `milestone add-task cache-rework "<same five>"` → same code at `at: task`, exit=1, git log still 3 commits (only the legit `chore(milestone): open record for milestone:cache-rework`), `.jigc/tasks` empty.
  `start --workflow single-task "<same four>"` → same code, exit=1, no area, HEAD unmoved.
  add-from-spec (derived door): wrote docs/specs/rate-limit.md with `### 日本語  {#degenerate-criterion}`, committed, `milestone create "Rate limit"`, then `"$JIGC" milestone add-from-spec rate-limit spec:rate-limit` → exit=1, same `write.unslugable-title` at `at: task`; HEAD before==after (ee08f38…), `.jigc/tasks` empty, `git status` clean.
  Exempt rows still mint: re-seed — `milestone create "Rate limit"`; `milestone add-task rate-limit "Warm the cache"`; `find .jigc/tasks -mindepth 1 -delete`; `milestone add-task rate-limit "Shard the index"` → both `warm-the-cache` and `shard-the-index` areas rebuilt, `.jigc/tasks/warm-the-cache/base.json` present.
  §12 boundary cell: `NR=$(mktemp -d); cd "$NR" && "$JIGC" milestone create ""` → exit=1, "not inside a git repository (no `.git` found from …) — run jigc from inside the target git repository…" — the one not-in-repo answer, not this guard's.

### PASS — Arm 4 — an uncommitted cherry-pick is refused at every acting door, in all four states

Repro (script /private/tmp/claude-501/-Users-maurice-projects-gherrink-jigc/fc2f5e3a-a01b-43ba-8a1b-f93e48070233/scratchpad/arm4.sh, one run per state):
  out=$(dev/jigc-rig fresh --git-state uncommitted-pick) || exit; eval "$out"; cd "$REPO"
  then all 12 ActsOnBehalf::{CommitsOnBehalf,MovesOnBehalf} rows of cli::cli::BEHALF_DOORS, with their registry argv: setup · migrate-corpus · rename adr:keeper --to Axis · relocate vision --from docs/vision/ · task discard <id> · task finalize <id> · config set docs-root docs · milestone create "Axis milestone" · milestone add-task <id> "axis intent" · milestone add-from-spec <id> spec:axis · milestone finalize <id> · milestone discard <id>
Observed for each of the four states (uncommitted-pick · uncommitted-pick-range · uncommitted-pick-conflicted · uncommitted-pick-resolved): RESULT[<state>] fail=0 — every door carried `repo.operation-in-progress` with the noun "an uncommitted cherry-pick is in progress", none carried the stale qualifier "once its conflicts are resolved"; `shasum <git-dir>/MERGE_MSG` byte-unchanged before/after all 12, `shasum <git-dir>/index` byte-unchanged, `git rev-parse HEAD` unmoved.
  Route identity — piping all 12 doors' output through `grep route: | sort -u` yields exactly ONE line: "  route: conclude it with `git commit` (which uses the pick's own message, once any conflicts are resolved), or abandon it with `git reset` (which keeps the picked changes in your working tree, unstaged), then re-run this command".
  `jigc task validate xx` previews it at exit=1.
  Abandon route run verbatim: `git reset` → rc=0; re-probe gives `finalize.no-task` (posture clean); `git status --porcelain` still shows the picked changes (`?? posture-other.txt` / `?? posture-other.txt`+`?? posture-second.txt` / ` M posture-shared.txt`) — unstaged, not discarded.

### PASS — Arm 3 — a directory carrying no base pin is a work unit at no door (3 shapes × 3 placements × all 25 WORK_UNIT_ID_DOORS)

Repro (script …/scratchpad/arm3.sh <shape> <placement>, 9 runs):
  out=$(dev/jigc-rig fresh) || exit; eval "$out"; cd "$REPO"
  shapes: `mkdir .jigc/tasks/stray-alpha` (empty) | + notes.txt (foreign file) | + docs/adr:cache-policy.md (foreign doc-shaped); same three at .jigc/milestones/stray-mile-a
  placements: plain · a repo with an open milestone · a repo with a joined milestone
  then all 17 task rows and all 8 milestone rows of cli::cli::WORK_UNIT_ID_DOORS with their registry argv, each `--format json`.
Observed: RESULT[<shape>/<placement>] fail=0 for all nine cells. Every task row answered `(finalize.no-task, task:stray-alpha)`, every milestone row `(milestone.unknown, milestone:stray-mile-a)`; no `/var/folders` or `/private/var/folders` host-absolute path on any surface.
  Residual sentence + route, verbatim: "blocking · finalize.no-task — no task `stray-alpha`: `.jigc/tasks/stray-alpha` is a directory carrying no base pin, so it is a leftover and not a work unit — either jigc never minted a task there, or a teardown stopped partway and left the directory behind / at: task:stray-alpha / route: nothing was changed. Keep anything you need from `.jigc/tasks/stray-alpha` and delete the rest by hand — jigc mints no verb that clears a leftover working area…".
  Enumerators: `jigc task list` → "jigc task list — no active tasks"; `jigc start` names no residual; a work-starting `jigc start --workflow dev-task "Probe intent"` appends no `also open:` line naming it.
  Named cell (no false record flip): milestone create/add-task/provision/join, then `find .jigc/tasks/real-work -mindepth 1 -delete`, then `jigc task discard real-work` → exit=1 with the residual sentence, `git rev-parse HEAD` before==after (8010dda…), `jigc milestone list-tasks joined-mile` still "tasks (1): real-work", directory untouched.
  Riders: same-slug mint → `jigc start --workflow dev-task "Stray alpha"` over `.jigc/tasks/stray-alpha` refuses `task.serial-collision` carrying the residual sentence, exit=1. `jigc rename` proceeds: with `.jigc/tasks/stray-alpha` AND `.jigc/milestones/stray-mile-a` planted, `dev/jigc-rig vendored` + `jigc rename spec:padding --to "Padding v2"` → "renamed spec:padding -> spec:padding-v2 (docs/specs/padding.md -> docs/specs/padding-v2.md), repointed 0 referrer(s)".

### PASS — Arm 1 — no byte jigc did not write dies under .jigc/milestones/<id>/, at any door over it

Repro (script …/scratchpad/mile2.sh builds a joined, validating milestone and plants six shapes: <area>/plant-root.txt · merged/plant-merged.txt · merged/docs/deep.txt · merged/docs/adr.md · merged/docs/provenance.json · merged/scratch/note.txt):
  Displace member, squash:true — `"$JIGC" milestone finalize cache-rework --format json` → exit=0; committed.displaced carries all six, `merged/scratch` as ONE entry (directory whole); all six parked byte-intact under .jigc/displaced/cache-rework/<relative> (verified by cat: "P1 area root"…"P6 in a directory"); the same six named on stderr under "note: the working area held 6 entries jigc did not write … they were moved aside, not taken:"; milestone area removed.
  Displace member, squash:false — `jigc config set finalize.fan-out.squash false` then the same finalize → identical six-entry `displaced`, same stderr, contents intact.
  Refuse member, no consent — `"$JIGC" milestone discard cache-rework` → exit=1, `blocking · milestone.foreign-bytes` naming all six paths, route at `jigc milestone discard cache-rework --force`; all six still on disk afterwards.
  Refuse member, with consent — `"$JIGC" milestone discard cache-rework --force` → exit=0, two `warning:` blocks naming every entry, ack "discarded milestone:cache-rework (1 sub-task(s); workbench removed)", area gone.
  Second refuse member — `"$JIGC" uninstall` → exit=1, `blocking · uninstall.foreign-bytes` naming all six, plants still on disk; `"$JIGC" uninstall --force` → narrates the working-area loss + the staged docs + the 5 tracked files, then removes `.jigc/`.
  Blocked boundary (mile.sh, unauthored adr) — `"$JIGC" milestone finalize cache-rework` → exit=3 on three `schema-conformance.required-slot-present`; all six plants verified still on disk by cat, and `grep -c 'plant|deep.txt|scratch'` over the blocked output = 0, so no plant is the blocker.
  No-accretion preserved — planting a jigc-shaped stale body `merged/docs/adr:old-thing.md` beside foreign `merged/docs/keep-me.txt` and finalizing: the stale jigc body is taken (file gone), only `keep-me.txt` is displaced.
  Control — the same fixture with no plants: `committed.displaced == []`, stderr empty, `.jigc/displaced` never created.

### PASS — Arm 2 — a displacement that failed leaves every un-moved byte on disk, with a code, a route, and the AREA's count

Repro (…/scratchpad/task.sh builds a finalizable task with three plants: foreign-root.txt · docs/foreign-docs.txt · scratch/note.txt; mile2.sh the milestone twin):
  all move — `"$JIGC" task finalize tidy-the-cache --format json` → exit=0, three `displaced` entries (scratch as one), `findings == []`, all three parked byte-intact.
  some move (partial) — `mkdir -p .jigc/displaced/tidy-the-cache; echo occupier > .jigc/displaced/tidy-the-cache/docs` then the same finalize → exit=0, `displaced` = [foreign-root.txt, scratch]; one `finalize.foreign-bytes` on the envelope's `findings`, key `{code: finalize.foreign-bytes, target: task:tidy-the-cache}`; `.jigc/tasks/tidy-the-cache/docs/foreign-docs.txt` still on disk (`cat` → "F2 docs"); stderr counts the AREA: "the working area held 3 entries jigc did not write … 2 of them moved aside … and 1 could not be moved: … File exists (os error 17)".
  none move at the boundary — `echo 'not a directory' > .jigc/displaced` then `"$JIGC" milestone finalize cache-rework --format json` → exit=0; stdout top-level keys are exactly `['committed']` (the landed envelope is NOT moved by the new code); the advisory is on stderr only, keyed `at: milestone:cache-rework`; all six plants (incl. the four under `merged/`) verified still on disk — this is cell D1×D2.
  fault on the pin — in-transaction racer `printf '#!/bin/sh\nchmod 0555 "$REPO/.jigc/tasks/tidy-the-cache"\nexit 0\n' > .git/hooks/pre-commit` → exit=0, commit landed, area standing WITH base.json, message names "the teardown stopped at `.jigc/tasks/tidy-the-cache/base.json` (Permission denied (os error 13))", un-moved bytes present, `jigc task list` truthfully lists the task as active.
  fault on a later member — the hook chmods `<area>/docs` instead → exit=0, base.json GONE (the pin's fate separates the coordinates), `docs/foreign-docs.txt` still present with its bytes, the two movable plants parked intact.
  hook-writes-during-the-commit — hook writes `<area>/hook-left-this.txt` and exits 0 → the displacement enumerates it, ZERO move failures, it is parked at .jigc/displaced/tidy-the-cache/hook-left-this.txt.
  Take carve-out — sub-task area plant + `"$JIGC" milestone discard cache-rework --force` → takes it, narrates "removing the working area .jigc/tasks/warm-the-cache discards work that is not in git: … sub-foreign.txt", acks "workbench removed", `.jigc/displaced` never created.
  Three zero-false-fire controls: (a) full lifecycle `start --workflow record-decision` → `doc create adr` → three `doc set-slot` → `doc rename adr:cache-policy --to "Cache policy v2" --task <id>` (area then holds renames.json + roles.json) → `task validate` (exit 0) → `task finalize` → `displaced == []`, findings only `file-state.staged-copy`, stderr empty, `.jigc/displaced` absent. (b) a migrate task (`jigc migrate CHANGELOG.md --as changelog`, area holds `source` + `source-path`): `task discard --force` → "discarded task … (transient)", nothing called foreign, `.jigc/displaced` absent. (c) ordinary post-join milestone area incl. `merged/`: `displaced == []`, stderr empty, `.jigc/displaced` absent.

### PASS — Order-invariance — a 3-process fan-out driven in three deliberately-divergent orders commits byte-identical output

Repro (…/scratchpad/fanout3.sh <fwd|rev|mid>): each run mints one fresh rig, `milestone create "Cache rework"`, three `milestone add-task` (Alpha/Beta/Gamma), `milestone provision`, then drives each sub-task as SEPARATE binary processes through the real re-entry verb `"$JIGC" workflow sub-task --task <sub>` + `doc create adr` + three `doc set-slot`, in order fwd=(alpha,beta,gamma) · rev=(gamma,beta,alpha) · mid=(beta,gamma,alpha); then `milestone join` + `milestone finalize`; then dumps every committed `docs/**` blob with the fixture-only `base: <sha> <short>` line normalized.
Observed: shasum of the three dumps identical —
  cad3c78c2a89034222d3ad5150244259bfa401ab  /tmp/committed-fwd.txt
  cad3c78c2a89034222d3ad5150244259bfa401ab  /tmp/committed-rev.txt
  cad3c78c2a89034222d3ad5150244259bfa401ab  /tmp/committed-mid.txt
  and `diff` between all three empty ("IDENTICAL").
Un-normalized, the only cross-order difference in any committed file was the milestone record's `base:` line (babb131… vs 5d04cc5…), which is the throwaway fixture's own base commit sha — each rig mints a fresh repo — not an order effect; the three joined ADR blobs hashed identically un-normalized (8a46253d…, cbbc0dd8…, 70c6da1b…).

### PASS — Adapter spawn template rendered through the binary and executed verbatim

Repro: milestone create/add-task ×2/provision, then
  "$JIGC" milestone execute cache-rework 2>/dev/null | grep '^Spawn: ' | sed 's/^Spawn: `//; s/`$//' > /tmp/spawns.txt
Rendered lines:
  cd .jigc/worktrees/alpha-work && jigc workflow sub-task --task alpha-work
  cd .jigc/worktrees/beta-work && jigc workflow sub-task --task beta-work
Each then run VERBATIM as its own process: ( cd "$REPO" && PATH="$(dirname "$JIGC"):$PATH" sh -c "$line" ) → exit=0 for both, first output line "Reason about the change. The intent is:" — the template names a verb that exists and composes.
HONEST BOUND, flagged per role: this is an N-process binary sim plus the verbatim spawn template. The genuine concurrent Task-tool spawn is the orchestrator's MAIN-SESSION half, and I did NOT run it — I run headless. A green here is not the real-spawn proof.

### FAIL — DEFECT — jigc's own `finalize-message.tmp` is classified as a byte jigc did not write, producing a false `finalize.foreign-bytes` advisory and blocking two destroying doors at exit 1 over jigc's own file

Self-contained repro on target/debug/jigc @ f664863a (jigc 1.0.0-rc.16) — NO foreign byte is ever planted:
  cd /Users/maurice/projects/gherrink-jigc
  out=$(dev/jigc-rig fresh) || exit; eval "$out"; cd "$REPO"; export HOME="$RIG_HOME"
  "$JIGC" start --workflow dev-task "Tidy the cache"
  "$JIGC" doc set-field 'commit:tidy-the-cache#header/type' --task tidy-the-cache --value chore
  printf 'tidy the cache\n' | "$JIGC" doc set-slot 'commit:tidy-the-cache#summary' --task tidy-the-cache --from-file -
  printf 'x\n' > code.txt; git add code.txt
  printf '#!/bin/sh\nchmod 0555 "%s/.jigc/tasks/tidy-the-cache"\nexit 0\n' "$REPO" > .git/hooks/pre-commit; chmod 755 .git/hooks/pre-commit
  "$JIGC" task finalize tidy-the-cache
  chmod 0755 .jigc/tasks/tidy-the-cache; rm -f .git/hooks/pre-commit
  "$JIGC" task discard tidy-the-cache; echo exit=$?
  "$JIGC" uninstall; echo exit=$?
Observed — stderr of the landed finalize (exit 0):
  note: the working area held 1 entry jigc did not write, and removing it would have destroyed bytes no commit has a copy of — not one of them could be moved aside:
      .jigc/tasks/tidy-the-cache/finalize-message.tmp — could not move it to .jigc/displaced/tidy-the-cache/finalize-message.tmp: Permission denied (os error 13)
  advisory · finalize.foreign-bytes — `.jigc/tasks/tidy-the-cache` holds 1 path(s) jigc did not write … `.jigc/tasks/tidy-the-cache/finalize-message.tmp` …
The area then holds only jigc's own bytes (base.json, docs, finalize-message.tmp, intent, staged-snapshot.json, workflow), and:
  jigc task discard tidy-the-cache → exit=1, "blocking · task-discard.foreign-bytes — task `tidy-the-cache`'s working area holds 1 path(s) jigc did not write … .jigc/tasks/tidy-the-cache/finalize-message.tmp", route demands --force
  jigc uninstall → exit=1, "blocking · uninstall.foreign-bytes — `.jigc/` holds 1 path(s) jigc did not write … finalize-message.tmp", route demands --force
Same at the milestone area kind (…/scratchpad/mile2.sh noplant + the same hook chmodding .jigc/milestones/cache-rework): exit 0, advisory names `.jigc/milestones/cache-rework/finalize-message.tmp` as a path jigc did not write, keyed `at: milestone:cache-rework`.
It also corrupts the genuine case's arithmetic: with three real foreign plants and the same racer, the narration reads "the working area held 4 entries jigc did not write" and lists finalize-message.tmp first.
ROOT CAUSE: crates/cli/src/task.rs:3961 writes `msg_tmp_dir.join("finalize-message.tmp")` as a STRING LITERAL, so the name is in neither `engine::state::TASK_AREA_FILES` nor `MILESTONE_AREA_FILES`, and `engine::state::foreign_area_paths` returns it as complement. This is the exact G-13 failure mode `TASK_AREA_FILES`' own doc-comment records for `record-commit-msg.txt`.
It was SEEN IN PLANNING AND DISPOSED ON A PREMISE THIS PASS'S OWN FAULT MODEL FALSIFIES. DECISIONS.md:408: "That name is in neither TASK_AREA_FILES nor MILESTONE_AREA_FILES, so an area still holding it would unwind to Foreign on every landed finalize … It is removed at task.rs:4059, one statement before post_commit, and best-effort (`let _ = std::fs::remove_file(…)`)." The removal's failure is swallowed, and Increment 2's own racer — an in-transaction pre-commit hook that changes the area's mode, exactly the fixture flow54's `install_faulting_hook` uses — makes it fail. crates/cli/tests/task_area_writer_registry.rs → NON_AREA_JOINS carries the matching disposition row, written at M53 Increment 2 / T1: "a name that is gone before that door ever reads the area is not one of them" — driven false above.
NOT CAUGHT BY THE SHIPPED SUITE: `cargo test -p cli --test g_flow flow54` → 12 passed; 0 failed (38.72s). Arm 2's FaultOnPin coordinates assert plant counts and one advisory per standing area; neither asserts the complement holds no jigc file, so the false member passes silently.
HOW THE COUNT WAS DERIVED: writer sites = 1 — `grep -rn "finalize-message" crates/cli/src crates/engine/src` → 1 hit (task.rs:3961); the fence's own NON_AREA_JOINS doc-comment states in prose that this is THE ONE row whose receiver is a working area ("The name is one row too narrow, and the row that falsifies it says so"). Area kinds reached = 2, both DRIVEN (msg_tmp_dir is cleanup_dir at all three call sites, per DECISIONS.md:388). Doors that read the complement = the 6 rows of `cli::milestone::DESTROYING_DOORS` (crates/cli/src/milestone.rs:3338, `[&DestroyingDoor; 6]`: milestone provision · milestone discard · uninstall · task discard · task finalize · milestone finalize); I DROVE the false classification at 4 of them (task finalize, milestone finalize, task discard, uninstall) — `milestone provision` and `milestone discard` read the same `engine::state::foreign_area_paths` producer but were NOT driven, so their cells are inferred, not witnessed. The registry fence's OWN declared bound (paths composed by `format!`, `PathBuf::push`, or a helper taking the filename as an argument are outside its reach) means an unenumerated remainder may exist beyond this one site.
SECONDARY, lower severity, pre-existing and NOT M53's: the same fault path prints a HOST-ABSOLUTE path on a user-facing surface — `"$JIGC" task finalize <id>` under `chmod a-w <area>` emits {"error": "could not write the commit message to \"/private/var/folders/.../repo/.jigc/tasks/tidy-the-cache/finalize-message.tmp\": Permission denied (os error 13)"} (crates/cli/src/task.rs:3961's `with_context(|| format!("… {msg_path:?}"))`), against law 1's repo-relative rule. Count derived by `grep -rncE '\{[a-z_]*(path|dir|file|tmp)[a-z_]*:\?\}' crates/cli/src crates/engine/src` → 39 hits across 9 files (cli: task.rs 9, milestone.rs 15, gitignore.rs 4, doc.rs 2, ingest.rs 1, relocate.rs 1; engine: parse.rs 4, data_value.rs 1, milestone.rs 2), of which the 2 engine/milestone.rs hits are `read_dir_order` and not paths → 37 path-bearing Debug-format sites. `crates/cli/tests/repo_relative_paths.rs` → UNSWEPT_PRODUCERS declares only 2 for task.rs and names them as other sites, so these Debug-formatted sites appear to sit outside the fence's reach entirely; I did not read the fence's scanner to confirm that, so treat the 37 as a grep-derived upper bound on the class, not an adjudicated one.
