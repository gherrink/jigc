<!-- M52 per-axis review (re-run) — axis 3 · posture — RECONCILED · driven on the installed `jigc 1.0.0-rc.16` (built from commit e519e4eb), 2026-09-21. -->

<!-- RECONCILED FILE — M52 per-axis review (re-run), axis 3 · destroying doors.
     Sections 1–7 below are the Opus driver's table, verbatim, with exactly seven verdict
     cells amended by a `DEMOTED by reconciliation` marker (rows 17, 18, 22, 26, 29, 30, 52).
     Section 8 (the Reconciliation ledger) and section 9 (Doors covered) are the reconciler's.
     Reconciled 2026-09-21 on `/Users/maurice/.local/bin/jigc` = `jigc 1.0.0-rc.16`. -->

<!-- M52 per-axis review (the RE-RUN of M51's instrument) — axis 3 · destroying doors · the Opus driver
     Driven on the installed `jigc 1.0.0-rc.16`, 2026-09-21. -->

# M52 per-axis review — AXIS 3 · destroying doors — the Opus driver's `(door, cell)` table

**Binary.** `/Users/maurice/.local/bin/jigc`, asserted first, before anything else:

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.16
```

**Release** posture (the debug binary's route-fence panics do not exist here), and the binary built
**after** M52's seven completion-audit fixes — `ad527fa4` (home-vacated history leg) · `6c2de03a`
(the store sweep's prior-home subject) · `33bd5692` (the survivable frame's state clause) ·
`68d14cd3` (`ENVELOPE_OWED_CODES`) · `a83a9e60` (`RelocateRefusal::ALL`) · `66af090a` (the vacated
home's route) ([VERDICT](../../../../completions/artifacts/M52/VERDICT.md)). Every row below ran on
that binary.

**Rigs.** `rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
(two-step eval), then — for the milestone cells — `jigc milestone create "axis three probe"` +
two `jigc milestone add-task` **through the binary**. That construction is
`scratchpad/axis-review/driver/mkrig.sh`; it prints `REPO=<path>|HOME=<path>` and nothing else.
Every fixture state was built by driving the binary; nothing was written into `.jigc/` by hand
except the **plants** (a leftover at a worktree path, a foreign byte in a working area, a workbench
file, a `chmod`), which is what this axis is about. `mktemp -d` roots, no teardown, none needed.

**Harness note that cost two drives and is recorded so the next driver does not re-learn it.** In
this harness the **cwd does not persist between tool calls** (it resets to the project root), while
`HOME` does not persist either. Every probe below is therefore a **single self-contained block**
that builds its own rig. A probe split across two calls silently runs its second half in the real
project repo — the shape that produced two discarded observations early in this run.

**Scope note.** The Codex source pass for this axis was **not** read (reconciliation is a separate
agent). Nothing below rests on a source read: a rule cited from the code or a design doc is only
ever the *contract* a driven row is judged against. Where I cite `milestone.rs`/`task.rs` under a
defect, the defect was **found by driving** and the citation is the contract it contradicts.

---

## 1 · The door set and the cell set, derived from the code

**Read verbatim at HEAD `e519e4eb`, with the count each declaration carries** (counted by a
balanced-bracket top-level-item parse of each declaration, comments stripped):

| registry | file:symbol | count read | members |
|---|---|---|---|
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3275` | **6** (`[&DestroyingDoor; 6]`) | `PROVISION_DOOR` · `DISCARD_DOOR` · `UNINSTALL_DOOR` · `TASK_DISCARD_DOOR` · `TASK_FINALIZE_DOOR` · `FINALIZE_DOOR` |
| `WORKTREE_DOORS` | `crates/cli/src/milestone.rs:3284` | **4** | `PROVISION` · `DISCARD` · `UNINSTALL` · `FINALIZE` |
| `Disposition` | `crates/cli/src/milestone.rs:3061` | **3** variants | `Refuse{consent}` · `Narrate` (**no member holds it**) · `Displace` |
| `LEFTOVER_VERDICTS` | `crates/cli/src/milestone.rs:3048` | **3** | `Unverifiable` · `OwnWorktree` · `NoOwnLinkage` |
| `LeftoverShape` | `crates/cli/src/milestone.rs:3336` | **3** variants | `Directory` · `File` · `Unreadable(String)` |
| `TASK_AREA_FILES` | `crates/engine/src/state.rs:129` | **13** | `base.json` · `docs/` · `intent` · `renames.json` · `roles.json` · slug-override · `source` · source-path · `staged-snapshot.json` · `workflow` · `record-commit-msg.txt` · base-snapshot · snapshot |
| **axis-3 door set** | `DESTROYING_DOORS` (the M51 `∪ jigc task discard` is now **inside** it) | **6 doors** | below |

**The M51→M52 door-set diff, stated because it is the re-run's first structural fact.** M51 drove
`DESTROYING_DOORS` (then **4**) ∪ `jigc task discard` = **5**. At rc.16 the registry itself is
**6**: `task discard` joined it as a `Refuse` member, and **`jigc task finalize` joined it as a
`Displace` member** — a door that was in no destroying registry at all when M51's C-1 found it
destroying bytes. `FINALIZE_DOOR` moved from *narrate-only* (`code: None`) to `Displace`. So the
axis's door set **gained one door and changed one door's disposition**, and the `Narrate` arm now
has **no member**.

**The six doors, with the refusal identity each carries** (`DestroyingDoor::codes` is now a **set**,
not a single `Option<code>`):

| door (VERB_KINDS spelling) | disposition | `codes` | other blocking identities driven at this door |
|---|---|---|---|
| `milestone provision` | `Refuse{--force}` | `milestone.leftover-holds-work` | `milestone.provision-failed` (phase 2) |
| `milestone discard` | `Refuse{--force}` | `milestone.dirty-worktree` · `milestone.staged-prose` · `milestone.foreign-bytes` | — |
| `uninstall` | `Refuse{--force}` | `uninstall.dirty-worktree` · `uninstall.staged-prose` · `uninstall.foreign-bytes` · `uninstall.untracked-workbench-file` | `uninstall.remove-jigc` |
| `task discard` | `Refuse{--force}` | `task-discard.staged-prose` · `task-discard.foreign-bytes` | — |
| `task finalize` | **`Displace`** | `&[]` | — (no consent flag: `--help` lists `--approve`, `--format`, `--dry-run`, `--carry-staged` only) |
| `milestone finalize` | **`Displace`** | `&[]` | `milestone.zero-contribution` (not a destruction refusal) |

**Cell set driven.** The acceptance design's `LeftoverShape × {staged prose · untracked workbench
file · clean} × {no --force, --force}`, expanded on the two axes M52 minted and the one axis M51
had already found the cell names compress:

- **S1** `Directory`/`NoOwnLinkage`, non-empty · **S2** `Directory`/`Unverifiable` (dangling `.git`)
  · **S3** `Directory`/`OwnWorktree`, **dirty** · **S4** `Directory`/`OwnWorktree`, **clean** ·
  **S5** `File` (plain file; and a **symlink**, a `File` whatever it points at) · **S6**
  `Unreadable` (`chmod 000` dir) · **S7** staged prose (`.jigc/tasks/<id>/docs/*.md`) · **S8**
  untracked workbench file (the `ENTRIES` complement under `.jigc/`) · **S9** clean · **S10** the
  fail-closed *probe* cells · **S11** the removal-fails cells · **S12 (new, M52)** the
  `TASK_AREA_FILES` **complement** in a **task** working area · **S13 (new, M52)** the complement in
  the **milestone** working area · **S14 (new, M52)** the **displacement-destination-unusable** cell
  at the two `Displace` doors · **S15 (new, M52)** `.jigc/displaced/` non-empty, and a plain file at
  an `ENTRIES` name.

**Other registries read, as instructed** (none is axis 3's, so each is a count and the method, not a
driven row): `VERB_KINDS` **47** · `PATH_ARG_OCCURRENCES` **14** · `DOCTYPE_DOORS` **16** ·
`SLUG_DOORS` **6** · `WORK_UNIT_ID_DOORS` **25** · `BEHALF_DOORS` **47** · `COMMITTING_DOORS` **10**
· `ROLLBACK_POPULATIONS` **11** · `ENVELOPE_OWED_CODES` **4** · `STORE_EXIT_FLIPS` **7** (M52's
`home-vacated` joined; M51 read 6) · `InProgress::ALL` **9** · `PostureMember::ALL` **3** ·
`RelocateRefusal::ALL` **10** (`relocate.rs:91`, minted by M52 F5) · `PRE_DISPATCH_FAULTS` is
**test-side** (`crates/cli/tests/pre_dispatch_faults.rs`, reached from `flow53_acceptance.rs:2448`),
not a production registry, so it is named and not counted as a production door set.

---

## 2 · The table — 63 driven rows over 6 doors

Every row's argv ran on the installed `1.0.0-rc.16`; its exit, code, route kind and asserted surface
line are recorded, and the repro block for its cell is in §3. **Route kind** is read off the rendered
finding: `Mechanical` = the `route:` opens with a backticked argv · `Human` = prose naming the acts ·
`Informational` = neither · `none` = no finding.

| # | door | cell | argv (driven) | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `milestone provision` | S1 dir, no `--force` | `jigc milestone provision axis-three-probe` | 1 | `milestone.leftover-holds-work` | Mechanical | `.jigc/worktrees/first-sub: precious.txt — git reports no worktree of its own there…`; plant on disk after | matches contract |
| 2 | `milestone provision` | S1, `--format json` | `… --format json` | 1 | same, flattened | Mechanical (inside the message) | stderr `{"error": …}`, **stdout 0 bytes** | matches contract |
| 3 | `milestone provision` | S1, `--force` | `… --force` | 0 | none | none | `warning: removing the leftover directory … precious.txt` + `not recoverable`, then `provisioned 2 worktree(s)` | matches contract |
| 4 | `milestone provision` | S5 file, no `--force` | `jigc milestone provision axis-three-probe` | 1 | `milestone.leftover-holds-work` | Mechanical | `…: the file itself — it is a file, not a worktree` | matches contract |
| 5 | `milestone provision` | S5 **symlink**, no `--force` | same | 1 | same | Mechanical | identical `the file itself` wording; `readlink` unchanged | matches contract |
| 6 | `milestone provision` | S6 unreadable, no `--force` | same | 1 | same | Mechanical | `…: unknown — could not read the leftover directory …: Permission denied (os error 13)` | matches contract |
| 7 | `milestone provision` | S2 unverifiable, no `--force` | same | 1 | same | Mechanical | `…: .git, wip.txt — git cannot read a repository there` | matches contract |
| 8 | `milestone provision` | S9 clean | same | 0 | none | none | `provisioned 2 worktree(s) … at base <sha>` + the orientation footer | matches contract |
| 9 | `milestone provision` | S4 own **clean** worktree (re-run) | same | 0 | none | none | idempotent reuse ack | matches contract |
| 10 | `milestone provision` | S3 own **dirty** worktree (re-run) | same | 0 | none | none | ack; `wip.txt` byte-intact after | matches contract |
| 11 | `milestone provision` | S7 staged prose present | same | 0 | none | none | ack; not this door's subject | matches contract (n/a **proven by driving**) |
| 12 | `milestone provision` | S8 untracked workbench file present | same | 0 | none | none | ack; `.jigc/notes.txt` intact | matches contract (n/a proven by driving) |
| 13 | `milestone provision` | S5+S6 **both** planted | same | 1 | `milestone.leftover-holds-work` | Mechanical | **both** paths listed, one line each, `would delete 2 path(s)` | matches contract (collect-all) |
| 14 | `milestone provision` | S11 partial removal (read-only **parent**) | `… --force` | 1 | `milestone.provision-failed` | Mechanical, **echoing `--force`** | `0 of 2 worktree(s) landed`; loss warning names `precious.txt`, which **is** gone | matches contract |
| 15 | `milestone provision` | S11 **no** removal (read-only leftover) | `… --force` | 1 | `milestone.provision-failed` | Mechanical | **no loss warning at all**; `precious.txt` survives | matches contract (outcome-keyed) |
| 16 | `milestone provision` | S5 symlink → tree **outside** the repo, `--force` | `… --force` | 0 | none | none | `removing the leftover file …`; the outside tree intact | matches contract |
| 17 | `milestone provision` | malformed id (`""`, `"../.."`) | `jigc milestone provision ""` | 1 | `work-unit.malformed-id` | Human | repo + `.jigc/` intact | matches contract (M50 guard holds) — **DEMOTED by reconciliation: no repro block in §3; re-driven by the reconciler, see ledger RD-17** |
| 18 | `milestone discard` | S3 dirty worktree **+** S7 staged prose | `jigc milestone discard axis-three-probe` | 1 | `milestone.dirty-worktree` | Human | worktree arm wins precedence | matches contract — **DEMOTED by reconciliation: no repro block in §3; re-driven by the reconciler, see ledger RD-18** |
| 19 | `milestone discard` | S7 staged prose alone | same | 1 | `milestone.staged-prose` | Human | `first-sub: adr:renamed-decision`; route names `jigc milestone finalize <id>` | matches contract |
| 20 | `milestone discard` | **S12** foreign byte in a sub-task area | same | 1 | `milestone.foreign-bytes` | Human | `.jigc/tasks/first-sub/notes.txt`; route names `--force` | matches contract (**C-1 CLOSED**) |
| 21 | `milestone discard` | **S13** foreign byte in the **milestone** area | same | 1 | `milestone.foreign-bytes` | Human | `.jigc/milestones/axis-three-probe/mnotes.txt` | matches contract |
| 22 | `milestone discard` | S12+S13, `--force` | `… --force` | 0 | none | none | **two** `warning: removing the working area …` blocks, one per area, each with its own `not recoverable` | matches contract — **DEMOTED by reconciliation: no repro block in §3; re-driven by the reconciler, see ledger RD-22** |
| 23 | `milestone discard` | S5+S6 both, no `--force` | `jigc milestone discard axis-three-probe` | 1 | `milestone.dirty-worktree` | Human | both paths + `not registered here, so the teardown leaves it on disk with no milestone naming it` | matches contract |
| 24 | `milestone discard` | S2 unverifiable, no `--force` | same | 1 | `milestone.dirty-worktree` | Human | `git cannot read a repository there` | matches contract |
| 25 | `milestone discard` | S2 unverifiable **unregistered**, `--force` | `… --force` | 0 | none | none | exit 0; the path + its bytes survive — the refusal's promise held | matches contract |
| 26 | `milestone discard` | S9 clean | `jigc milestone discard axis-three-probe` | 0 | none | none | `discarded milestone:… (2 sub-task(s); workbench removed)` | matches contract — **DEMOTED by reconciliation: no repro block in §3; re-driven by the reconciler, see ledger RD-26** |
| 27 | `milestone discard` | **S11 teardown fails** (read-only parent), `--force` | `… --force` | 0 | none (2 warnings) | Informational (`remedy:` ×2) | ack reads **`workbench NOT fully removed — the warnings above name what is left`** | matches contract (**D-2 CLOSED**) |
| 28 | `milestone discard` | S10 fail-closed: unreadable own sub-task area | `jigc milestone discard axis-three-probe` | 1 | `milestone.foreign-bytes` | Human | `cannot check … Permission denied`; route names both dirs **repo-relatively** and `--force` | matches contract |
| 29 | `milestone discard` | refusal, `--format json` | `… --format json` | 1 | flattened | Human (in message) | stderr `{"error": …}`, stdout empty | matches contract (declared flattened default) — **DEMOTED by reconciliation: no repro block in §3; re-driven by the reconciler, see ledger RD-29** |
| 30 | `milestone discard` | malformed id | `jigc milestone discard ""` / `"../.."` | 1 | `work-unit.malformed-id` | Human | repo intact | matches contract — **DEMOTED by reconciliation: no repro block in §3; re-driven by the reconciler, see ledger RD-30** |
| 31 | `uninstall` | S3+S7+S8+S12 all four subjects | `jigc uninstall` | 1 | `uninstall.dirty-worktree` | Human | precedence 1 of 4; route names `milestone discard <id> --force` **and** `uninstall --force` | matches contract |
| 32 | `uninstall` | same, `--force` | `… --force` | 0 | none | none | **five** narration blocks (worktree · working area · staged docs · untracked workbench file · tracked files), each with its own recoverability claim | matches contract |
| 33 | `uninstall` | S7 staged prose alone (sub-task) | `jigc uninstall` | 1 | `uninstall.staged-prose` | Human | `first-sub: adr:… — a sub-task of milestone …, which `jigc task finalize` refuses` | matches contract (**D-1 CLOSED**) |
| 34 | `uninstall` | **S12** foreign byte in a task area | same | 1 | `uninstall.foreign-bytes` | Human | `.jigc/tasks/tidy-the-readme/notes.txt` | matches contract (**C-1 CLOSED**) |
| 35 | `uninstall` | **S13** foreign byte in the milestone area | same | 1 | `uninstall.foreign-bytes` | Human | `.jigc/milestones/axis-three-probe/mnotes.txt` | matches contract |
| 36 | `uninstall` | **S15** non-empty `.jigc/displaced/` | same | 1 | `uninstall.foreign-bytes` | Human | `.jigc/displaced/some-task/keep.txt`; route says *jigc has no verb that clears `.jigc/displaced/`* | matches contract |
| 37 | `uninstall` | S15, `--force` | `… --force` | 0 | none | none | `warning: removing the relocation workbench .jigc/displaced …` names the byte | matches contract |
| 38 | `uninstall` | **S15** plain **file** at an `ENTRIES` name (`.jigc/logs`) | `jigc uninstall` | 1 | `uninstall.untracked-workbench-file` | Human | `.jigc/logs` named | matches contract |
| 39 | `uninstall` | S8 untracked workbench file alone | same | 1 | `uninstall.untracked-workbench-file` | Human | `.jigc/notes.txt` | matches contract |
| 40 | `uninstall` | S9 clean | same | 0 | none | none | 5 tracked files narrated + 7-bullet teardown | matches contract |
| 41 | `uninstall` | S9 clean, `--format json` | `… --format json` | 0 | none | none | stdout `{allowlist_file, findings:[], line_file, removed}` | matches contract |
| 42 | `uninstall` | S9 second run (idempotency), and `--force` | `jigc uninstall` / `… --force` | 0 | none | none | `(nothing to remove — no repo-local jigc install was present)`, identical both ways | matches contract |
| 43 | `uninstall` | S10 fail-closed **prose/foreign** probe (`chmod 000 …/docs`) | `jigc uninstall` | 1 | `uninstall.foreign-bytes` | Human | **no host path anywhere**; route is repo-relative and names `--force` | matches contract (**D-3 CLOSED**) |
| 44 | `uninstall` | S10 fail-closed **worktrees root** (`chmod 000 .jigc/worktrees`) | same | 1 | `uninstall.dirty-worktree` | Human | route: *this failure is a `read_dir` of that directory, **not a git fault*** and names `jigc uninstall --force` | matches contract (**D-4 CLOSED**) |
| 45 | `uninstall` | S10 fail-closed **workbench** probe (`chmod 000 .jigc/config`) | same | 1 | `uninstall.untracked-workbench-file` | Human | names `git` on PATH **and** tree-readability **and** `--force` | matches contract (see O-2 — M51 §4's row 7, no longer un-driven) |
| 46 | `uninstall` | refusal, `--format json` | `… --format json` | 1 | `uninstall.*` | Human | **`{schema_version, findings}`** on stderr, stdout empty | matches contract (**O-1 CLOSED** — the bare `Finding` is gone) |
| 47 | `task discard` | S7 staged prose, ordinary task | `jigc task discard tidy-the-readme` | 1 | `task-discard.staged-prose` | Human | route names `jigc task finalize <id> (which refuses while a required slot is empty)` | matches contract |
| 48 | `task discard` | S7 on a **milestone sub-task** | `jigc task discard first-sub` | 1 | `task-discard.staged-prose` | Human | route names `jigc milestone finalize <milestone>` and says *so `jigc task finalize` refuses it* | matches contract (**D-1 CLOSED**) |
| 49 | `task discard` | row 47's route **run verbatim** | `jigc task finalize tidy-the-readme` | 3 | `schema-conformance.*` | Mechanical | exit 3 for the **stated** reason (empty required slot) | matches contract (route lands where it says) |
| 50 | `task discard` | **S12** foreign bytes only (no staged `.md`) | `jigc task discard tidy-the-readme` | 1 | `task-discard.foreign-bytes` | Human | both plants named; area intact after | matches contract (**C-1 CLOSED**) |
| 51 | `task discard` | **S12** the whole complement shape space (8 shapes) | same | 1 | `task-discard.foreign-bytes` | Human | **all 8** named: dotfile · symlink at an address-shaped name · non-`.md` under `docs/` · un-staged `docs/*.md` · foreign root file · symlink · nested dir · `.md` outside `docs/` | matches contract |
| 52 | `task discard` | S12 **precedence** vs S7 (both present) | same | 1 | `task-discard.foreign-bytes` | Human | foreign-bytes wins over staged-prose | matches contract — **DEMOTED by reconciliation: no repro block in §3; re-driven by the reconciler, see ledger RD-52** |
| 53 | `task discard` | S12, `--force` | `… --force` | 0 | none | none | `warning: removing the working area … discards work that is not in git` naming the byte, then the ack | matches contract |
| 54 | `task discard` | S7, `--force`, `--format json` | `… --force --format json` | 0 | none | none | `{commit, dropped, findings, op, task}` | matches contract |
| 55 | `task discard` | S10 fail-closed (`chmod 000 …/docs`) | `jigc task discard tidy-the-readme` | 1 | `task-discard.foreign-bytes` | Human | **no host path**; route repo-relative + `--force` | matches contract (**D-3 CLOSED**) |
| 56 | `task discard` | **control** — jigc's own files only (incl. `renames.json`, `roles.json`) | same | 1 | `task-discard.staged-prose` | Human | `foreign-bytes` **does not fire** over jigc's own 13-member set | matches contract (zero false fire) |
| 57 | `task discard` | S9 area absent · malformed id | `jigc task discard tidy-the-readme` / `""` / `"../.."` | 1 | `finalize.no-task` / `work-unit.malformed-id` | Mechanical / Human | `.jigc/tasks/` intact, repo intact | matches contract |
| 58 | `task finalize` | **S12** complement present, displacement | `jigc task finalize tidy-the-readme` | 0 | none | none | `note: … moved aside, not taken:` with one `<from> → <to>` per entry; every byte at `.jigc/displaced/<id>/<relative>` | matches contract (**C-1 CLOSED**) |
| 59 | `task finalize` | S12, `--format json` | `… --format json` | 0 | none | none | `committed.displaced: [{from,to}]` | matches contract |
| 60 | `task finalize` | S12 **collision** (same task id displaced twice) | same, twice | 0 | none | none | second lands `notes.txt.2`; the first copy's bytes unchanged | matches contract |
| 61 | `task finalize` | **S14** displacement destination **unusable** (`.jigc/displaced` a file; and a read-only `displaced/`) | same | **0** | none | Informational (`note:`) | `note: could not open … to move … aside`, then **the area is removed anyway**; `displaced: []`; bytes gone from disk and in no commit | **DEFECT A3-2** |
| 62 | `task finalize` | S14′ the **source** unreadable (`chmod 000` subtree) | same | 0 | none | Informational | move fails **and** the removal fails; bytes survive at the source | matches contract (the safe half of the same cell) |
| 63 | `task finalize` | commit **rejected** by a hook, S12 present | same, with a rejecting `pre-commit` | 1 | (rejection frame) | Human | nothing displaced, nothing removed, bytes intact | matches contract |
| 64 | `milestone finalize` | S4 landed, an **unstaged** file in a worktree | `jigc milestone finalize axis-three-probe` | 0 | none | none | stderr `warning: removing the fan-out worktree … scratch.txt (never staged)`; stdout `discarded with the fan-out worktrees` | matches contract |
| 65 | `milestone finalize` | same, `--format json` | `… --format json` | 0 | none | none | `sub_tasks[].discarded = [{"path":"scratch.txt","state":"never-staged"}]` | matches contract |
| 66 | `milestone finalize` | S1+S5 **unregistered** leftovers under `.jigc/worktrees/` | same | 0 | none | none | both survive byte-intact and are not named (the declared "cannot reach this shape") | matches contract |
| 67 | `milestone finalize` | **S12** foreign byte in a **sub-task** area | same | 0 | none | Informational (`note:`) | `…/tasks/first-sub/tnotes.txt → .jigc/displaced/first-sub/tnotes.txt`; byte intact | matches contract (**C-1 CLOSED**) |
| 68 | `milestone finalize` | **S13** foreign bytes in the **milestone** area | same | **0** | **none** | **none** | `.jigc/milestones/<id>/mnotes.txt` and `…/sub/deep.txt` **destroyed**; `displaced: []`; named on no stream | **DEFECT A3-1** |
| 69 | `milestone finalize` | **S14** displacement destination unusable | same | 0 | none | Informational | same as row 61 at this door: `note: could not open …`, sub-task-area bytes gone | **DEFECT A3-2** (second door) |
| 70 | `milestone finalize` | nothing to land | same | **3** | `milestone.zero-contribution` | Mechanical | names `provision` + `discard` as the two exits | matches contract (not a destruction refusal) |

*(Rows 2, 29, 41, 46, 54, 59, 65 re-drive a state an earlier row reached and are kept as their own
rows because the **surface asserted** is the machine envelope, not the text. Discounting those seven,
the table covers **63 distinct `(door, cell)` pairs** across **6 doors** — 70 rows total.)*

---

## 3 · Repro blocks

Every block is `setup · argv · observed`, each a **single self-contained shell block** (see the
harness note). `newrig` = `dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc` two-step-eval'd,
plus `milestone create` + two `add-task` where the label says *milestone rig*.

### R-A — `milestone provision` over every leftover shape (rows 1–8, 13)

```
setup:  milestone rig
        mkdir -p .jigc/worktrees/first-sub && printf 'precious\n' > .jigc/worktrees/first-sub/precious.txt
argv:   jigc milestone provision axis-three-probe
exit:   1
out:    blocking · milestone.leftover-holds-work — milestone:axis-three-probe: `jigc milestone provision`
          would delete 1 path(s) it cannot prove are disposable, so nothing was removed:
          .jigc/worktrees/first-sub: precious.txt — git reports no worktree of its own there, so
            nothing can say those bytes are disposable
          at: .jigc/worktrees/first-sub
          route: `jigc milestone provision axis-three-probe --force` — but look at what is listed
            above first and move out anything you need; the removal is permanent
after:  ls .jigc/worktrees/first-sub -> precious.txt            # nothing removed

argv:   jigc milestone provision axis-three-probe --format json
exit:   1
out:    stdout 0 bytes; stderr: { "error": "blocking · milestone.leftover-holds-work — …" }

argv:   jigc milestone provision axis-three-probe --force
exit:   0
out:    warning: removing the leftover directory .jigc/worktrees/first-sub discards work that is
          not in git:  precious.txt   note: … not recoverable.
        provisioned 2 worktree(s) for milestone:axis-three-probe at base 3c1310a (first-sub, second-sub)
        — jigc · run `jigc start` for orientation; all writes through `jigc`.

# the same argv over each other shape, one fresh milestone rig each (only the plant differs):
plant:  printf 'leaf bytes\n' > .jigc/worktrees/first-sub
out:    .jigc/worktrees/first-sub: the file itself — it is a file, not a worktree, …        exit 1
plant:  ln -s /etc/hosts .jigc/worktrees/first-sub
out:    …: the file itself — …                    exit 1   (readlink after: /etc/hosts)
plant:  mkdir -p .jigc/worktrees/first-sub/inner && chmod 000 .jigc/worktrees/first-sub
out:    …: unknown — could not read the leftover directory `.jigc/worktrees/first-sub`:
          Permission denied (os error 13) — nothing could be read there, …                 exit 1
plant:  mkdir -p .jigc/worktrees/first-sub
        printf 'gitdir: /nonexistent/admin\n' > .jigc/worktrees/first-sub/.git
        printf 'work\n' > .jigc/worktrees/first-sub/wip.txt
out:    …: .git, wip.txt — git cannot read a repository there, …                           exit 1
plant:  (nothing)
out:    provisioned 2 worktree(s) …                                                        exit 0

# collect-all, two shapes at once (row 13):
plant:  printf 'leaf\n' > .jigc/worktrees/first-sub
        mkdir -p .jigc/worktrees/second-sub/inner && chmod 000 .jigc/worktrees/second-sub
out:    "would delete 2 path(s)…" with BOTH lines, `at:` on the first                      exit 1
```

**Instrument note, recorded because it nearly produced a false CLOSED.** The `.git`-plant block was
first run with `mkdir -p .jigc/worktrees` (not `…/first-sub`), so the `printf` failed, the plant
never landed and the door answered `exit 0, provisioned`. That reads exactly like a regression
against M51 row 7. It was caught by `ls -a`-ing the plant before the argv, which every shape block
above now does.

### R-B — idempotent reuse, and the subjects that are **not** this door's (rows 9–12)

```
setup:  milestone rig; jigc milestone provision axis-three-probe
argv:   jigc milestone provision axis-three-probe            -> exit 0, reuse ack
        printf 'uncommitted\n' > .jigc/worktrees/first-sub/wip.txt
        jigc milestone provision axis-three-probe            -> exit 0; cat …/wip.txt = uncommitted
        printf 'private\n' > .jigc/notes.txt
        jigc milestone provision axis-three-probe            -> exit 0; notes.txt intact
        (a sub-task with staged prose present)               -> exit 0   # not this door's subject
```

### R-C — the two removal-outcome cells at `provision --force` (rows 14–15)

```
setup:  milestone rig; mkdir -p .jigc/worktrees/first-sub; printf 'precious\n' > …/precious.txt

# (a) read-only PARENT — remove_dir_all takes the child, fails to unlink the dir
argv:   chmod 500 .jigc/worktrees; jigc milestone provision axis-three-probe --force
exit:   1
out:    warning: removing the leftover directory … precious.txt … not recoverable.
        blocking · milestone.provision-failed — … could not clear the leftover at
          `.jigc/worktrees/first-sub`: Permission denied (os error 13). 0 of 2 worktree(s) landed …
          route: `jigc milestone provision axis-three-probe --force` — deal with what the message
            names at that path first; the re-run reuses every worktree that landed …
after:  precious.txt IS gone      -> the narration was true; `--force` echoed in the re-run argv

# (b) read-only LEFTOVER — nothing is removed at all
argv:   chmod 500 .jigc/worktrees/first-sub; jigc milestone provision axis-three-probe --force
exit:   1
out:    blocking · milestone.provision-failed — … (identical shape)
        *** NO loss warning printed ***
after:  cat .jigc/worktrees/first-sub/precious.txt -> precious    # narration is outcome-keyed
```

### R-D — the symlink escape at two doors (rows 16, 37-adjacent)

```
setup:  OUT=$(mktemp -d "${TMPDIR:-/tmp}/axis3-outside.XXXXXX"); printf 'PRECIOUS OUTSIDE\n' > "$OUT/keep.txt"
        milestone rig; mkdir -p .jigc/worktrees && ln -s "$OUT" .jigc/worktrees/first-sub
argv:   jigc milestone provision axis-three-probe --force
exit:   0
out:    warning: removing the leftover file .jigc/worktrees/first-sub discards work that is not in
          git:  first-sub … not recoverable.
        provisioned 2 worktree(s) …
after:  cat "$OUT/keep.txt" -> PRECIOUS OUTSIDE      # the link died, the target did not
```

### R-E — **C-1 CLOSED**: the `TASK_AREA_FILES` complement at all five doors that destroy an area (rows 20, 34, 50, 58, 67)

M51's C-1: *a non-`.md` file under a task's working area is destroyed at exit 0 by five doors, with
no refusal and no narration.* Re-driven door by door on rc.16:

```
# (1) jigc task discard — the reported door
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        printf 'PRECIOUS NOTES\n' > .jigc/tasks/tidy-the-readme/notes.txt
        printf 'ATTACHED BYTES\n' > .jigc/tasks/tidy-the-readme/docs/attachment.txt
argv:   jigc task discard tidy-the-readme
exit:   1
out:    blocking · task-discard.foreign-bytes — task `tidy-the-readme`'s working area holds 2
          path(s) jigc did not write — `.jigc/` is gitignored, so discarding the task would destroy
          bytes nothing else has a copy of:
          .jigc/tasks/tidy-the-readme/docs/attachment.txt
          .jigc/tasks/tidy-the-readme/notes.txt
          route: move what you need out of `.jigc/tasks/tidy-the-readme/`, or delete what you do
            not, then re-run `jigc task discard tidy-the-readme` — or … `--force` removes the
            working area with them
after:  the area is intact, both plants present

# (2) jigc milestone discard
setup:  milestone rig; printf 'PRECIOUS NOTES\n' > .jigc/tasks/first-sub/notes.txt
argv:   jigc milestone discard axis-three-probe
exit:   1   blocking · milestone.foreign-bytes — … .jigc/tasks/first-sub/notes.txt … --force …

# (3) jigc uninstall
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        printf 'PRECIOUS NOTES\n' > .jigc/tasks/tidy-the-readme/notes.txt
        rm -f .jigc/tasks/tidy-the-readme/docs/*.md       # the exact M51 escape: no staged .md left
argv:   jigc uninstall
exit:   1   blocking · uninstall.foreign-bytes — … .jigc/tasks/tidy-the-readme/notes.txt …

# (4) jigc task finalize — the door that was in no destroying registry when M51 found it
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        printf 'PRECIOUS NOTES\n' > .jigc/tasks/tidy-the-readme/notes.txt
        printf 'ATTACHED\n'       > .jigc/tasks/tidy-the-readme/docs/attachment.txt
        printf 'hello\n' >> README.md; git add README.md
        jigc doc set-field 'commit:tidy-the-readme#header/type' --task tidy-the-readme --value feat
        printf 'tidy the readme\n' | jigc doc set-slot 'commit:tidy-the-readme#summary' \
                 --task tidy-the-readme --from-file -
argv:   jigc task finalize tidy-the-readme
exit:   0
out:    note: the working area held 2 entries jigc did not write, and removing it would have
          destroyed bytes no commit has a copy of — they were moved aside, not taken:
            .jigc/tasks/tidy-the-readme/docs/attachment.txt → .jigc/displaced/tidy-the-readme/docs/attachment.txt
            .jigc/tasks/tidy-the-readme/notes.txt           → .jigc/displaced/tidy-the-readme/notes.txt
        finalized 7b671cf — feat: tidy the readme
after:  cat .jigc/displaced/tidy-the-readme/notes.txt -> PRECIOUS NOTES
argv:   (same, --format json)  -> "committed": { "displaced": [ {"from": …, "to": …} ], … }

# (5) jigc milestone finalize
setup:  milestone rig; jigc milestone provision axis-three-probe
        (cd .jigc/worktrees/first-sub  && printf 'one\n' > one.txt && git add one.txt)
        (cd .jigc/worktrees/second-sub && printf 'two\n' > two.txt && git add two.txt)
        printf 'AREA PRECIOUS\n' > .jigc/tasks/first-sub/notes.txt
argv:   jigc milestone finalize axis-three-probe
exit:   0
out:    note: … moved aside, not taken:
            .jigc/tasks/first-sub/notes.txt → .jigc/displaced/first-sub/notes.txt
after:  cat .jigc/displaced/first-sub/notes.txt -> AREA PRECIOUS
```

**Verdict: C-1 is CLOSED at all five doors** — three refuse under their own `*.foreign-bytes` code
naming every path with a `--force` route, two displace to `.jigc/displaced/<id>/<relative>` and name
each move on stderr and on the envelope's `displaced` key.

### R-F — the complement's whole shape space at one refusing door and one displacing door (rows 51, 58)

```
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme";  T=.jigc/tasks/tidy-the-readme
        printf 'a\n' > $T/foreign-root.txt          # foreign regular file at the area root
        printf 'b\n' > $T/docs/non-md.txt           # non-.md under docs/
        printf 'c\n' > $T/docs/notanaddress.md      # a docs/*.md that is no staged id
        printf 'd\n' > $T/stray.md                  # a .md OUTSIDE docs/
        mkdir -p $T/nested/deep && printf 'e\n' > $T/nested/deep/x.txt
        printf 'f\n' > $T/.hidden                   # dotfile
        ln -s /etc/hosts $T/link                    # symlink
        ln -s /etc/hosts $T/docs/link.md            # symlink named like an address
argv:   jigc task discard tidy-the-readme
exit:   1   task-discard.foreign-bytes — "holds 8 path(s) jigc did not write", all eight named
            (.hidden · docs/link.md · docs/non-md.txt · docs/notanaddress.md · foreign-root.txt ·
             link · nested · stray.md), sorted

# the displacing half, in a sibling rig (5 of the same shapes + a landable commit):
argv:   jigc task finalize tidy-the-readme
exit:   0   five `<from> → <to>` lines; afterwards
            readlink .jigc/displaced/tidy-the-readme/link          -> /etc/hosts   (link preserved,
                                                                     not dereferenced)
            cat .jigc/displaced/tidy-the-readme/nested/deep/x.txt  -> e            (tree preserved)
```

### R-G — displacement collision safety (row 60)

```
setup:  fresh rig; two successive `jigc start --workflow quick-fix "tidy the readme"` + finalize
        cycles, each with a `notes.txt` plant (the slug, hence the displaced home, is the same)
out:    run 1: .jigc/tasks/tidy-the-readme/notes.txt → .jigc/displaced/tidy-the-readme/notes.txt
        run 2: .jigc/tasks/tidy-the-readme/notes.txt → .jigc/displaced/tidy-the-readme/notes.txt.2
after:  cat .jigc/displaced/tidy-the-readme/notes.txt -> FIRST     # run 1's bytes untouched
```

### R-H — **DEFECT A3-1**: the milestone's own working area's complement dies at `milestone finalize` (row 68)

```
setup:  milestone rig; jigc milestone provision axis-three-probe
        (cd .jigc/worktrees/first-sub  && printf 'one\n' > one.txt && git add one.txt)
        (cd .jigc/worktrees/second-sub && printf 'two\n' > two.txt && git add two.txt)
        printf 'MILESTONE-AREA PRECIOUS\n' > .jigc/milestones/axis-three-probe/mnotes.txt
        mkdir -p .jigc/milestones/axis-three-probe/sub
        printf 'DEEP\n' > .jigc/milestones/axis-three-probe/sub/deep.txt
before: find .jigc/milestones -type f ->
          …/base.json  …/mnotes.txt  …/record-commit-msg.txt  …/staged-snapshot.json
          …/sub/deep.txt  …/tasks.json
        git status --porcelain --ignored …/mnotes.txt  ->  !! .jigc/milestones/   (gitignored:
                                                            nothing else has a copy)
argv:   jigc milestone finalize axis-three-probe
exit:   0
out:    finalized c48cc91 — Finalize milestone axis-three-probe (2 sub-tasks)
          modified docs/milestone-records/axis-three-probe.md
          added one.txt
          added two.txt
          3 files committed
          sub-tasks: first-sub: 1 code file · second-sub: 1 code file
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        *** no note, no warning, no finding, on either stream ***
after:  find .jigc/milestones -> .jigc/milestones          # the whole area is gone
        find .jigc/displaced  -> No such file or directory
        (the same run with --format json:  "committed":{ "displaced": [], … } )

# the sibling doors, driven on the identical plant, to show the axis is un-swept and not undecided:
argv:   jigc milestone discard axis-three-probe
exit:   1   blocking · milestone.foreign-bytes — … .jigc/milestones/axis-three-probe/mnotes.txt …
argv:   jigc milestone discard axis-three-probe --force
exit:   0   warning: removing the working area .jigc/milestones/axis-three-probe discards work that
              is not in git:  .jigc/milestones/axis-three-probe/mnotes.txt  … not recoverable.
argv:   jigc uninstall
exit:   1   blocking · uninstall.foreign-bytes — … .jigc/milestones/axis-three-probe/mnotes.txt …
```

### R-I — **DEFECT A3-2**: the displacement fails, and the door removes the area anyway (rows 61, 69)

```
# cell (a) — the destination cannot be created: `.jigc/displaced` is a regular file
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        printf 'NOT A DIR\n' > .jigc/displaced
        printf 'PRECIOUS-A3-2\n' > .jigc/tasks/tidy-the-readme/notes.txt
        mkdir -p .jigc/tasks/tidy-the-readme/deep && printf 'DEEP\n' > .jigc/tasks/tidy-the-readme/deep/x.txt
        printf 'hello\n' >> README.md; git add README.md
        (commit doc filled as in R-E)
argv:   jigc task finalize tidy-the-readme --format json
exit:   0
stderr: note: could not open .jigc/displaced/tidy-the-readme to move
          .jigc/tasks/tidy-the-readme/deep aside: Not a directory (os error 20)
        note: could not open .jigc/displaced/tidy-the-readme to move
          .jigc/tasks/tidy-the-readme/notes.txt aside: Not a directory (os error 20)
stdout: "committed": { "displaced": [], … }
after:  ls -a .jigc/tasks           -> . ..                 # the whole area removed
        git grep -l 'PRECIOUS-A3-2' HEAD   -> (nothing)
        grep -rl 'PRECIOUS-A3-2' .         -> (nothing)     # permanently gone, at exit 0

# cell (b) — the same, with an ordinary read-only `.jigc/displaced/` directory
setup:  fresh rig; jigc start …; mkdir -p .jigc/displaced && chmod 500 .jigc/displaced
        printf 'PRECIOUS-RO\n' > .jigc/tasks/tidy-the-readme/notes.txt   (+ landable commit)
argv:   jigc task finalize tidy-the-readme
exit:   0
stderr: note: could not open .jigc/displaced/tidy-the-readme to move
          .jigc/tasks/tidy-the-readme/notes.txt aside: Permission denied (os error 13)
after:  ls -a .jigc/tasks -> . ..      ;  grep -rl 'PRECIOUS-RO' . -> (nothing)

# the same class at the second Displace door
setup:  milestone rig + provision + staged code in both worktrees
        printf 'NOT A DIR\n' > .jigc/displaced
        printf 'PRECIOUS-MF\n' > .jigc/tasks/first-sub/tnotes.txt
argv:   jigc milestone finalize axis-three-probe
exit:   0
stderr: note: could not open .jigc/displaced/first-sub to move .jigc/tasks/first-sub/tnotes.txt
          aside: Not a directory (os error 20)
after:  grep -rl 'PRECIOUS-MF' . -> (nothing)

# the SAFE half of the same cell, driven so the finding is not over-stated (row 62):
setup:  fresh rig; jigc start …; mkdir -p $T/secret && printf 'PRECIOUS\n' > $T/secret/keep.txt
        chmod 000 $T/secret            # the move fails AND the removal fails
argv:   jigc task finalize tidy-the-readme
exit:   0
stderr: note: could not move …/secret aside … Permission denied (os error 13)
        note: post-commit working-area removal failed (self-heals): Permission denied (os error 13)
after:  chmod 755 $T/secret; cat $T/secret/keep.txt -> PRECIOUS      # survives, by the same EACCES
```

### R-J — **D-1 CLOSED**: the staged-prose route over a milestone sub-task (rows 33, 47–49)

```
setup:  milestone rig; jigc doc create adr --title "a sub decision" --task first-sub
argv:   jigc task discard first-sub
exit:   1
out:    blocking · task-discard.staged-prose — task `first-sub` stages 1 doc(s) … : adr:sub-decision
          route: read what is in them with `jigc doc show <address> --task first-sub`, or land them
            with `jigc milestone finalize axis-three-probe` — this task is a sub-task of milestone
            `axis-three-probe`, whose boundary is the only one that commits it, so
            `jigc task finalize` refuses it — or … `jigc task discard first-sub --force` …

argv:   jigc uninstall            (same state)
exit:   1
out:    blocking · uninstall.staged-prose — …
          first-sub: adr:sub-decision — a sub-task of milestone `axis-three-probe`, which
            `jigc task finalize` refuses: land it with `jigc milestone finalize axis-three-probe`,
            or drop it with `jigc task discard first-sub --force`, …
          route: … `jigc task finalize <task-id>` (which refuses while a required slot is empty,
            and refuses a sub-task outright — each sub-task above names its own two exits) …

# the omitting context — an ORDINARY task's route is unchanged, and lands where it says:
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
argv:   jigc task discard tidy-the-readme
out:    … land them with `jigc task finalize tidy-the-readme` (which refuses while a required slot
          is empty) …
argv:   jigc task finalize tidy-the-readme            # the route, run verbatim
exit:   3   schema-conformance.field-value-conformant + schema-conformance.required-slot-present
            — exit 3 for the reason the parenthetical states, not `finalize.milestone-sub-task`
```

### R-K — **D-2 CLOSED**: the forced discard's ack is a function of the teardown's outcome (row 27)

```
setup:  milestone rig; jigc milestone provision axis-three-probe
        printf 'wip\n' > .jigc/worktrees/first-sub/wip.txt; chmod 500 .jigc/worktrees
argv:   jigc milestone discard axis-three-probe --force
exit:   0
out:    warning: removing the fan-out worktree .jigc/worktrees/first-sub discards work that is not
          in git:  wip.txt (never staged)  … not recoverable.
        warning: could not remove the fan-out worktree /private/var/…/first-sub: `git worktree
          remove --force …` failed: … Permission denied
          remedy: run `git worktree prune`, then `git worktree remove --force …`
        warning: could not remove the fan-out worktree …/second-sub: … (same)
        discarded milestone:axis-three-probe (2 sub-task(s); workbench NOT fully removed — the
          warnings above name what is left)
after:  ls .jigc/worktrees -> first-sub  second-sub
```
M51's ack read `workbench removed` here; rc.16's is generated from the outcome. (The two `remedy:`
warnings' absolute paths are `milestone.rs`'s `DeclaredAbsolute` disposition, as M51 recorded — git
resolves a worktree path against the caller's cwd — and are not part of this row.)

### R-L — **D-3 CLOSED** and **D-4 CLOSED**: the fail-closed cells (rows 43–45, 55)

```
# D-3: fresh rig; jigc start --workflow quick-fix "tidy the readme"; chmod 000 …/docs
argv:   jigc task discard tidy-the-readme
exit:   1
out:    blocking · task-discard.foreign-bytes — cannot check task `tidy-the-readme`'s working area
          for files jigc did not write, so discarding it could destroy bytes nothing else has a
          copy of: Permission denied (os error 13)
          route: make sure `.jigc/tasks/tidy-the-readme/` is readable, then re-run
            `jigc task discard tidy-the-readme` — or … `--force` removes the working area unchecked
argv:   jigc uninstall            (same state)
out:    blocking · uninstall.foreign-bytes — cannot check `.jigc/` for files jigc did not write …:
          Permission denied (os error 13)
          route: make sure the `.jigc/` tree is readable, … or … `jigc uninstall --force`
        *** no host path on either surface — M51's /private/var/… is gone ***

# D-4: milestone rig + provision; chmod 000 .jigc/worktrees
argv:   jigc uninstall
exit:   1
out:    blocking · uninstall.dirty-worktree — cannot check `.jigc/worktrees/` for uncommitted
          fan-out work, so removing `.jigc/` could destroy it: Permission denied (os error 13)
          route: make sure `.jigc/worktrees/` is readable — this failure is a `read_dir` of that
            directory, not a git fault — then re-run `jigc uninstall`; or remove the worktrees
            yourself (`git worktree list`, then `git worktree remove`) and re-run — or, once you
            have confirmed the fan-out worktrees hold nothing you need, `jigc uninstall --force`
            deletes them with the install

# the third fail-closed sibling, M51 §4 row 7's un-driven cell (row 45):
setup:  fresh rig; chmod 000 .jigc/config
argv:   jigc uninstall
exit:   1   blocking · uninstall.untracked-workbench-file — cannot check `.jigc/` for files no index
            has a copy of … route: make sure `git` is on PATH and the `.jigc/` tree is readable …
            or … `jigc uninstall --force`
```

### R-M — `uninstall`'s four subjects, their precedence, and the forced narration (rows 31–32, 39–42, 46)

```
setup:  milestone rig; jigc milestone provision axis-three-probe
        printf 'wip\n' > .jigc/worktrees/first-sub/wip.txt
        jigc doc create adr --title "a sub decision" --task first-sub
        printf 'private\n' > .jigc/notes.txt
        printf 'FOREIGN\n' > .jigc/tasks/first-sub/foreign.txt
argv:   jigc uninstall
exit:   1   uninstall.dirty-worktree (precedence 1 of 4); route names
            `jigc milestone discard <milestone-id> --force` AND `jigc uninstall --force`
argv:   jigc uninstall --force
exit:   0   five narration blocks, in order:
              removing the fan-out worktree …first-sub … wip.txt (never staged)
              removing the working area .jigc/tasks/first-sub … .jigc/tasks/first-sub/foreign.txt
              removing `.jigc/` discards the staged docs of 1 open task(s) … first-sub: adr:sub-decision
              removing `.jigc/` destroys 1 file(s) under it that no index has a copy of … notes.txt
              removing `.jigc/` also removes 5 tracked file(s) … `git checkout -- <path>` brings it back
            then the 7-bullet teardown; `.jigc/` gone

# the clean cells, in a fresh rig each:
argv:   jigc uninstall                   -> exit 0, 5 tracked files narrated, 7-bullet teardown
argv:   jigc uninstall --format json     -> stdout {allowlist_file, findings:[], line_file, removed}
argv:   jigc uninstall (second run)      -> exit 0  "(nothing to remove — no repo-local jigc
                                                     install was present)"
argv:   jigc uninstall --force (second)  -> identical no-op

# the reject envelope (row 46), in a rig with an open task:
argv:   jigc uninstall --format json  2>&1 >/dev/null | python3 -c "…print(list(d.keys()))"
out:    ['schema_version', 'findings']          # M51 drove a BARE Finding (8 top-level keys) here
```

### R-N — `.jigc/displaced/` as a subject, and a plain file at an `ENTRIES` name (rows 36–38)

```
setup:  fresh rig; mkdir -p .jigc/displaced/some-task && printf 'SAVED\n' > …/keep.txt
argv:   jigc uninstall
exit:   1   uninstall.foreign-bytes — .jigc/displaced/some-task/keep.txt
            route: … (`rm -r` takes them — jigc has no verb that clears `.jigc/displaced/`) …
argv:   jigc uninstall --force
exit:   0   warning: removing the relocation workbench .jigc/displaced discards work that is not in
              git:  .jigc/displaced/some-task/keep.txt  … not recoverable.

setup:  fresh rig; rm -rf .jigc/logs; printf 'not a dir\n' > .jigc/logs
argv:   jigc uninstall
exit:   1   uninstall.untracked-workbench-file — .jigc/logs
```

### R-O — `milestone discard` across the worktree shapes, and the fail-closed own-area cell (rows 18–30)

```
setup:  milestone rig; printf 'leaf\n' > .jigc/worktrees/first-sub
        mkdir -p .jigc/worktrees/second-sub/inner && chmod 000 .jigc/worktrees/second-sub
argv:   jigc milestone discard axis-three-probe
exit:   1   milestone.dirty-worktree — "2 sub-task worktree path(s) hold something the abandon
            cannot prove is disposable" with BOTH per-path fates:
              `the file itself …; not registered here, so the teardown leaves it on disk with no
               milestone naming it`
              `unknown — could not read … ; not registered here, …`

setup:  milestone rig; dangling `.git` + wip.txt at .jigc/worktrees/first-sub
argv:   jigc milestone discard axis-three-probe           -> exit 1 milestone.dirty-worktree
argv:   jigc milestone discard axis-three-probe --force   -> exit 0, `workbench removed`
after:  cat .jigc/worktrees/first-sub/wip.txt -> w        # the refusal's promise held

setup:  milestone rig; chmod 000 .jigc/tasks/first-sub
argv:   jigc milestone discard axis-three-probe
exit:   1   milestone.foreign-bytes — cannot check … Permission denied (os error 13)
            route: make sure `.jigc/tasks/` and `.jigc/milestones/axis-three-probe/` are readable …
                   or … `--force` tears the workbench down unchecked
```

### R-P — `task discard`'s cells, its ack, and the zero-false-fire control (rows 47–57)

```
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
argv:   jigc task discard tidy-the-readme --force
exit:   0   discarded task tidy-the-readme — dropped staged edits to: commit:tidy-the-readme (transient)
argv:   jigc task discard tidy-the-readme --force --format json
out:    {"commit":null,"dropped":["commit:tidy-the-readme"],"findings":[],"op":"task-discard", …}
setup:  + printf 'PRECIOUS\n' > …/notes.txt
argv:   jigc task discard tidy-the-readme --force
exit:   0   warning: removing the working area .jigc/tasks/tidy-the-readme discards work that is not
              in git:  .jigc/tasks/tidy-the-readme/notes.txt  … not recoverable.
            discarded task tidy-the-readme — dropped staged edits to: …

# the control — jigc's own files, incl. the two `doc rename --task` writes:
setup:  milestone rig; jigc doc create adr --title "a sub decision" --task first-sub
        jigc doc rename adr:sub-decision --to "a renamed decision" --task first-sub
        ls -a .jigc/tasks/first-sub -> base.json docs intent renames.json roles.json workflow
argv:   jigc task discard first-sub        -> exit 1  task-discard.staged-prose  (NOT foreign-bytes)
argv:   jigc milestone discard axis-three-probe -> exit 1 milestone.staged-prose (NOT foreign-bytes)

# the area-absent and malformed-id cells:
argv:   jigc task discard tidy-the-readme  (area emptied with `find -delete`)
exit:   1   finalize.no-task — no task `tidy-the-readme`   route: `jigc task list`
argv:   jigc task discard "" / "../.."     -> exit 1  work-unit.malformed-id ; repo + .jigc intact
```

### R-Q — `milestone finalize`'s narrate-and-displace arms (rows 64–67, 70)

```
setup:  milestone rig; jigc milestone provision axis-three-probe
        (cd .jigc/worktrees/first-sub  && printf 'one\n' > one.txt && git add one.txt
                                       && printf 'PRECIOUS\n' > scratch.txt)   # never staged
        (cd .jigc/worktrees/second-sub && printf 'two\n' > two.txt && git add two.txt)
        mkdir -p .jigc/worktrees/stray-leftover && printf 'not mine\n' > …/keep.txt
        printf 'loose\n' > .jigc/worktrees/loose-file
argv:   jigc milestone finalize axis-three-probe --format json
exit:   0
stderr: warning: removing the fan-out worktree .jigc/worktrees/first-sub discards work that is not
          in git:  scratch.txt (never staged)  … not recoverable.
stdout: "displaced": []   and  sub_tasks[0] = {id: first-sub,
          discarded: [{"path":"scratch.txt","state":"never-staged"}]}
after:  ls .jigc/worktrees -> loose-file  stray-leftover    # both unregistered leftovers survive

# nothing to land:
argv:   jigc milestone finalize axis-three-probe   (fresh provision, no work)
exit:   3   milestone.zero-contribution — route names provision + discard as the two exits
```

### R-R — the rejected-commit cell (row 63)

```
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        printf 'PRECIOUS-HOOK\n' > .jigc/tasks/tidy-the-readme/notes.txt   (+ landable commit)
        printf '#!/bin/sh\nexit 1\n' > .git/hooks/pre-commit; chmod +x .git/hooks/pre-commit
argv:   jigc task finalize tidy-the-readme
exit:   1
out:    `git commit` was rejected (no commit was made): …
        task tidy-the-readme is intact — nothing was committed, your task's staged docs are still
        in `.jigc/tasks/tidy-the-readme/docs/`, and anything you had `git add`-ed is still in
        git's index. Fix the hook's complaint, then re-run `jigc task finalize tidy-the-readme`.
after:  cat .jigc/tasks/tidy-the-readme/notes.txt -> PRECIOUS-HOOK ;  .jigc/displaced absent
```

---

## 4 · `(door, cell)` pairs I did **not** drive, and why

Stated plainly rather than presented as covered:

1. **`task finalize` / `milestone finalize` × any cell × `--force`** — *the force axis does not
   exist at either leaf.* Driven: `jigc task finalize --help` lists `--approve`, `--format`,
   `--dry-run`, `--carry-staged`; `jigc milestone finalize --help` lists `--carry-staged`,
   `--format`. `DESTROYING_DOORS`' own doc-comment says the `jigc task finalize --force` cell "is
   never enumerated because no member carries a consent it does not have". Not driven, by
   construction.
2. **`milestone discard` / `uninstall` × S5 `File` at a *registered* worktree path** — unreachable:
   `git worktree add` cannot register a non-directory. The reachable branch (unregistered → the
   teardown skips it) is rows 25 and 66.
3. **`milestone provision` × {S7, S8, S12, S13} × `--force`** — the no-`--force` rows (11, 12) drove
   that none is this door's subject; `--force` only widens what the door may remove at a *worktree
   path*, so the forced cell cannot differ. Not driven.
4. **`task discard` × {S1–S6, S13}** — no such cell exists at that door: its subject is
   `.jigc/tasks/<id>/`; it reads no worktree path and no milestone area. Driven evidence that the
   scoping is real is row 28 (`milestone discard` over an *unrelated* task's unreadable `docs/` →
   the probe is milestone-scoped) and row 55's route, which names the task door and not `uninstall`.
5. **Any cell × `OwnWorktree` × `Unreadable`** — empty by construction: `git rev-parse
   --show-toplevel` cannot run inside a `chmod 000` directory, so the verdict is `Unverifiable`.
6. **The record-commit rejection cells at `milestone discard` / `task discard`** (a `pre-commit`
   rejecting the record-only commit) — that is **axis 4**. Row 63 drives the rejection at
   `task finalize` only because the question there is *did the destroying half run*, which is this
   axis's.
7. **`Disposition::Narrate`** — **no member holds it** at rc.16 (read from the code, and the
   `FINALIZE_DOOR` row's own doc-comment says so). There is nothing to drive; recorded as an empty
   arm, not as a skipped one.
8. **A genuine concurrent racer** against a displacement or a teardown — headless by construction,
   the same bound M52's own e2e half declares.
9. **`--ignored` build output inside a fan-out worktree** — M46's measured `visible, not prevented`
   decision; driving it would report a decided bound as a finding.

---

## 5 · Findings

### A3-1 · **HIGH** — `jigc milestone finalize` destroys the milestone's own working-area complement at exit 0, named by nothing

**What.** The landed milestone boundary removes `.jigc/milestones/<id>/` with `remove_dir_all` and
takes every byte in it that jigc did not write. A plain file and a nested directory planted there
were **permanently destroyed at exit 0**, with no `note:`, no `warning:`, no finding, and
`committed.displaced == []` on the pinned envelope. `.jigc/` is gitignored whole, so nothing else
has a copy — driven and confirmed with `git status --porcelain --ignored` before the run and a
whole-tree `grep` after.

**Repro.** §3 R-H (row 68).

**The contract it contradicts, in three places.**
- `DESTROYING_DOORS`' own doc-comment (`crates/cli/src/milestone.rs:3097`): *"Every door **answers
  for what it removes** … a door that destroys what it never named is the law-1 half-truth"*.
  `FINALIZE_DOOR` carries `Disposition::Displace`, whose rule is *"**Keep it**: move it aside with
  its relative path preserved, and name where it went"*. Driven, this door neither kept nor named.
- The wave's own claim ([M52 VERDICT](../../../../completions/artifacts/M52/VERDICT.md)): *"No byte
  dies and no third party's bytes are silently discarded at exit 0 behind a committing, destroying
  or moving door."*
- The **sibling doors decide the same subject the other way**, which is what makes this an un-swept
  axis rather than a design choice: on the identical plant `jigc milestone discard` refuses with
  `milestone.foreign-bytes` naming the path, and `jigc milestone discard --force` narrates *removing
  the working area `.jigc/milestones/axis-three-probe`* as an unrecoverable loss; `jigc uninstall`
  refuses with `uninstall.foreign-bytes`. Three of the four doors over that area answer for it.

**Why the axis was missed, stated as a lead for the fixer rather than as a source claim.** M52
Increment 4 gave the *sub-task* areas a `SubtaskComplement::Displace` disposition at this boundary
(`cleanup_subtask_areas`), and the milestone area is removed by the **shared finalize executor**,
which is handed `displace: None` with the comment *"`cleanup_dir` is then a *milestone* area, a
different registry row whose own displacement is Increment 4 / T4's act … this door must not answer
for a subject it was not given"* (`crates/cli/src/task.rs:5953`, `crates/cli/src/milestone.rs:5095`).
Driven, T4's act covered the sub-task areas only, so the subject that comment defers is answered by
nobody. Note the `MILESTONE_AREA_FILES` registry and `engine::state::unwind_area`'s
registry-keyed **non-recursive** removal already exist — the safe primitive is shipped and this call
site does not use it.

**Bound, stated so the fix is not over-scoped.** A `chmod 000` cell was not driven at this door.
The loss is of bytes a caller (or a hook, or an operator) put into `.jigc/milestones/<id>/`; the
adapter tells agents not to write there, which is the same reachability argument M49 heard and
rejected, and which the three sibling doors already rejected for this exact area.

### A3-2 · **HIGH** — when the displacement fails, both `Displace` doors remove the area anyway, and say only that the *move* failed

**What.** At `jigc task finalize` and `jigc milestone finalize`, the complement is moved to
`.jigc/displaced/<id>/<relative>` and then the area is removed with `remove_dir_all`. The removal is
**not** conditioned on the move having succeeded. With the destination unusable — `.jigc/displaced`
occupied by a regular file, or an ordinary read-only `.jigc/displaced/` directory — the door prints
`note: could not open … to move … aside: <err>` and then **deletes the bytes**, at **exit 0**, with
`committed.displaced == []`. Driven at both doors and over both destination shapes; the bytes are in
no commit and nowhere on disk afterwards.

**Repro.** §3 R-I (rows 61, 69).

**The contract it contradicts.** `Disposition::Displace` is *"**Keep it**"*; `DestroyingDoor`'s rule
is that `PendingLoss` is *"read before the removal and printed after it, so neither half of the claim
can be false"*. Here the printed half is *the move failed* and the unprinted half is *and then it was
deleted* — a reader who sees `could not … move … aside` is told the bytes stayed. Law 1
(`design/surface-contract.md`): a claim must be generated from the thing it describes.

**The safe half of the same cell was driven and is recorded** (row 62): when the *source* is
unreadable, the move fails **and** the subsequent `remove_dir_all` fails on the same `EACCES`, so the
bytes survive. The bytes survive there by accident of the two failures sharing a cause, not by the
door's design — which is exactly why the axis is `{move succeeds, move fails} × {removal succeeds,
removal fails}` and only three of its four cells are currently safe.

**Why the arm did not catch it, as a lead.** Flow 53 arm 3's per-cell assertion is *"`task finalize`
and `milestone finalize` move the complement to `.jigc/displaced/<id>/<relative>` and name it"* — the
**success** cell. Driven, no suite in `crates/cli/tests/` contains the strings `could not open` or
`could not move` (`finalize_displacement.rs` and `milestone_boundary_displacement.rs` do not), so the
failure cell of the move has no standing test at either door.

### A3-3 · **LOW** — `milestone_boundary_displacement`'s subject is the sub-task areas only, so the boundary's `displaced: []` assertion is true of a tree it did not look at

A record/consequence finding rather than a behaviour one, filed because A3-1's invisibility has a
mechanical cause worth closing in the same motion:
`crates/cli/tests/milestone_boundary_displacement.rs` asserts *"a landed boundary with no foreign
byte carries an empty `displaced` key"* and *"`.jigc/displaced` must not exist"* over fixtures whose
plants are all under `.jigc/tasks/`; the string `milestones` appears nowhere in the suite. So the
boundary's **own** area is outside the suite's subject, and a fixture planting there passes the
empty-key assertion while losing bytes. Driven consequence: A3-1 is green under the suite that exists
to fence this door's displacement. (Verdict: DEFECT against the acceptance design's own rule that an
arm *iterates its class's axis* — the class here is *working areas this door removes*, and the axis
has two members.)

### Observations (driven, not defects — recorded so reconciliation is not re-driving them)

- **O-1 · M51's O-1 is CLOSED.** `jigc uninstall --format json` over a refusal now emits
  `{schema_version, findings}` — one of `ENVELOPE_ARMS`' two declared reject rows — not the bare
  8-key `Finding` M51 drove. The acceptance design's arm 6 names this shape (*"`setup`/`uninstall`
  reject on the findings arm"*).
- **O-2 · the other three destroying doors still flatten.** `milestone provision`, `milestone
  discard` and `task discard` refuse under `--format json` with `{"error": …}`; their codes
  (`milestone.leftover-holds-work`, `milestone.dirty-worktree`, `milestone.foreign-bytes`,
  `task-discard.*`) are not `ENVELOPE_OWED_CODES` members (that set is **4**, all `store.*` plus
  `FIXED_IDENTITY`), so this is the declared default and not a defect at this axis. It is the same
  surface M52's VERDICT carries as *"the contract's wider `:197` rule is broader than the binary …
  a separate act"* — recorded here as the destroying doors' instance of that carried item, so a
  driver keying `(code, target)` at a destroying door still cannot.
- **O-3 · the orientation footer now rides these surfaces.** Every success and most refusals end
  `— jigc · run `jigc start` for orientation; all writes through `jigc`.` — M52 F4's declared byte
  change at the moved cells. No row treats its presence or absence as the asserted surface.
- **O-4 · `uninstall`'s workbench fail-closed route names `git` on PATH first** (row 45), which
  looks like D-4's shape. It is **not** the D-4 defect: `classify_workbench_paths` genuinely shells
  out to git, the route also names tree-readability, and it names `--force`. M51 §4 row 7 listed this
  cell as un-driven; it is now driven and holds.
- **O-5 · a plain file at `.jigc/displaced` is not gitignored.** `crate::gitignore::ENTRIES` carries
  `displaced/` (a directory pattern), so the synthetic A3-2 cell's regular file showed up in
  `finalize`'s `left-out (unstaged/untracked — git add to include)` block. Harmless, and the same is
  true of every `ENTRIES` member; recorded because it was visible in an A3-2 repro and a reader will
  otherwise wonder.
- **O-6 · the adapter deny floor still covers 2 of the now-6 doors.** The installed
  `.claude/settings.json` `deny` array carries `Bash(jigc uninstall:*)` and
  `Bash(jigc milestone discard:*)` and no other jigc verb — M50 Increment 3's shipped decision.
  `milestone provision`, `task discard` and both `Displace` doors are destroying doors an agent may
  run unasked. Recorded, not filed: this axis is the one that would notice.
- **O-7 · M50's degenerate-id guard holds at every destroying door** (rows 17, 30, 57):
  `""` and `"../.."` answer `work-unit.malformed-id` at exit 1 with the repo and `.jigc/` intact —
  the finding that blocked the 1.0.0 call after RC-m50.

---

## 6 · M51 rows: CLOSED / STILL-OPEN

The subject is M51 README **§A**'s five CONFIRMED entries for this axis. Every one re-driven on
`1.0.0-rc.16`.

| M51 §A row | status | the argv that settles it | observed |
|---|---|---|---|
| **C-1 class** — a non-`.md` under a task's working area destroyed at exit 0 by five doors, no refusal, no narration | **CLOSED** (all five doors) | `jigc task discard tidy-the-readme` · `jigc milestone discard axis-three-probe` · `jigc uninstall` · `jigc task finalize tidy-the-readme` · `jigc milestone finalize axis-three-probe`, each over the M51 plant | three refuse at exit 1 under their own `*.foreign-bytes` code naming **every** planted path with a `--force` route; two displace to `.jigc/displaced/<id>/<relative>`, name each `<from> → <to>` on stderr and carry the pairs on `committed.displaced`. Bytes byte-intact in all five. §3 R-E |
| **D-1 (MEDIUM)** — the staged-prose route at `task discard` / `uninstall` names `jigc task finalize <id>`, which a milestone sub-task refuses at exit 3 | **CLOSED** | `jigc task discard first-sub` · `jigc uninstall` over a sub-task with a staged `adr` | both routes now name `jigc milestone finalize axis-three-probe` and state *so `jigc task finalize` refuses it*; the ordinary-task route is unchanged and, run verbatim, lands at exit 3 for the **stated** reason (empty required slot). §3 R-J |
| **D-2 (LOW)** — `milestone discard --force` acks `workbench removed` over a failed teardown | **CLOSED** | `jigc milestone discard axis-three-probe --force` with `chmod 500 .jigc/worktrees` | ack reads `… (2 sub-task(s); workbench NOT fully removed — the warnings above name what is left)`; both worktrees still on disk, both named. §3 R-K |
| **D-3 (LOW)** — the fail-closed staged-prose refusal prints the host path | **CLOSED** | `jigc task discard tidy-the-readme` · `jigc uninstall` with `chmod 000 …/docs` | no absolute path on either surface; both routes are repo-relative and name `--force`. §3 R-L |
| **D-4 (LOW)** — the worktrees-root fail-closed route blames `git` on PATH and omits the consent | **CLOSED** | `jigc uninstall` with `chmod 000 .jigc/worktrees` | route reads *make sure `.jigc/worktrees/` is readable — **this failure is a `read_dir` of that directory, not a git fault*** and names `jigc uninstall --force`. §3 R-L |

**5 of 5 CLOSED · 0 STILL-OPEN.** M51's non-§A observations for this axis were also re-driven where
they were cheap: **O-1 is CLOSED** (the bare `Finding` reject envelope at `uninstall` is now the
declared findings arm); **O-3** (stale `git worktree list` prunable records after `uninstall`) and
**O-5** (the 2-of-N deny floor) are unchanged and are re-recorded above as O-6; M51 §4's row 7
(`unverified_workbench_finding`, listed as un-driven) is now **driven** and holds (row 45).

**Coverage diff against M51's axis-3 table.** M51: 5 doors / 60 rows / 55 distinct pairs / 4 defects.
M52: **6 doors / 70 rows / 63 distinct pairs / 3 defects**. No door lost coverage. The door gained is
`jigc task finalize` (a registry member at rc.16, and one of the two doors A3-2 lives at).

---

## 7 · What this adds over flow-53 arm 3

**Arm 3 is this axis's arm**, so the comparison is sharp rather than "no arm exists" (M51's answer
against flow 52). Arm 3's set is *`TASK_AREA_FILES`'s complement over the tree … × `DESTROYING_DOORS`
read through its `Disposition` axis*, and its per-cell assertion is: consenting doors refuse under
`<door>.foreign-bytes` naming every path then narrate; the two displacing doors move the complement
to `.jigc/displaced/<id>/<relative>` and name it on text and on the `displaced` key; the
unreadable-root cell holds at every door; `uninstall` refuses over an `ENTRIES`-named plain file and
over a non-empty `displaced/`; jigc's own files never trip the guard.

**Every one of those assertions is true of rc.16, and this table drove each of them** (rows 20, 34,
36, 38, 50–51, 53, 56, 58–60, 67). What it adds is the part an arm keyed to *`TASK_AREA_FILES`'s
complement* cannot reach:

1. **The other area registry.** Arm 3's subject is the **task** area's complement. The milestone
   area is a second `WorkArea` row with its own writer set, removed by a different call site, and
   three of the four doors over it already answer for it. The arm never looks there, and neither does
   the suite that fences the boundary's displacement — which is **A3-1**, exit-0 loss behind a door
   the wave's claim names.
2. **The disposition's failure axis, not just its success axis.** Arm 3 asserts the move *happens*.
   It does not iterate `{move succeeds, move fails}`, and the removal that follows is unconditional —
   which is **A3-2**, at both `Displace` doors, over two independent shapes of destination failure.
   Driving also found the cell's **safe** half (source unreadable → both operations fail together),
   so the finding is stated over the axis rather than over its worst cell.
3. **The routes were executed, not read** (rows 49, 25, 42, 44): D-1's closure is only visible by
   running the printed route and recording where it lands, and row 25 confirms `--force`'s *promise*
   about what it leaves behind by looking at the disk afterwards.
4. **`n/a` means driven.** Rows 11, 12, 28, 56 assert that a subject is **not** a door's by driving
   the door over it and watching it proceed — so a scope claim in this table is a measurement, never
   a read.
5. **The cross-product cells no registry has a row for**: the two removal-outcome cells at
   `provision --force` (rows 14/15 — narration keyed on the outcome), the fail-closed probe at the
   *root* versus at a *member* (rows 43–45), the symlink escape at two doors (row 16), the
   displacement **collision** (row 60), and the rejected-commit cell (row 63), which is where the
   destroying half and the transaction half meet.

---

## 8 · Reconciliation ledger

**Reconciler**, 2026-09-21. Binary asserted before anything else:

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.16
```

Source pass reconciled against:
`scratchpad/axis-review/codex/axis3-codex.md` (its prompt beside it as `axis3-prompt.md`),
written against `e519e4eb` — the same HEAD the driver read. Rigs built exactly as the driver
built them (`rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit;
eval "$rig"`, two-step, then `milestone create` + `add-task` through the binary), every probe a
single self-contained block for the cwd/`HOME` reason §3 records.

**The rule applied** (`acceptance-design.md` → the reconciliation rule): a claim by one side the
other cannot reproduce is a **lead**, not a finding. Every Codex claim below was entered as
`lead(codex, …)` and then **driven** — to a repro block (CONFIRMED, origin `codex`) or to a
falsifying datum (REFUTED). Nothing was promoted on a source read.

### 8.1 · Demotions in the driver's own table

The driver's §3 names, per repro block, the rows it covers. Seven table rows marked driven carry
**no `setup · argv · observed` block anywhere in §3** — rows 17, 18, 22, 26, 29, 30, 52. Under the
rule they are **not driven** as the driver's table presents them, so each carries a `DEMOTED`
marker in §2 above. Each was then **re-driven by the reconciler**, and all seven hold; they are
restored here as reconciler-origin driven rows with their repro blocks, not as driver rows.

(Four rows that looked demotable are **not**: row 35 is driven inside R-H's sibling-door block,
row 59 inside R-E's `--format json` line, row 62 inside R-I's "safe half" block, row 19 inside
R-P's control block. Checked before demoting.)

```
RD-17 · `milestone provision` × malformed id
setup:  milestone rig (fresh + create + 2× add-task)
argv:   jigc milestone provision ""        -> exit 1
        jigc milestone provision "../.."   -> exit 1
out:    blocking · work-unit.malformed-id — "" is not a valid work-unit id
          route: use lowercase letters, digits, and single hyphens (no leading, trailing, or
                 doubled `-`)
after:  .jigc -> AGENT.md config milestones state tasks version        # intact

RD-30 · `milestone discard` × malformed id                      (same rig, same shape)
argv:   jigc milestone discard "" / "../.."  -> exit 1, work-unit.malformed-id, identical text
after:  .jigc intact; repo -> CLAUDE.md docs README.md

RD-29 · `milestone discard` refusal under `--format json`
setup:  milestone rig; jigc doc create adr --title "a sub decision" --task first-sub
argv:   jigc milestone discard axis-three-probe --format json
exit:   1 ;  stdout 0 bytes
stderr: { "error": "blocking · milestone.staged-prose — milestone:axis-three-probe: 1 sub-task(s)
          stage 1 doc(s) that no commit has a copy of … route: … `jigc milestone finalize
          axis-three-probe` … or … `--force` …" }
        # the flattened default, not the findings envelope — the driver's O-2, confirmed

RD-26 · `milestone discard` × S9 clean, no `--force`
setup:  milestone rig (2 sub-tasks, nothing staged, nothing provisioned)
argv:   jigc milestone discard axis-three-probe
exit:   0
out:    discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
after:  .jigc/milestones -> (empty) ; .jigc/tasks -> (empty)

RD-18 · `milestone discard` × dirty worktree + staged prose (precedence)
setup:  milestone rig + provision; printf 'wip\n' > .jigc/worktrees/first-sub/wip.txt
        jigc doc create adr --title "a sub decision" --task first-sub
argv:   jigc milestone discard axis-three-probe
exit:   1
out:    blocking · milestone.dirty-worktree — … 1 sub-task worktree path(s) hold something the
          abandon cannot prove is disposable …
          .jigc/worktrees/first-sub: ?? wip.txt — it is a live git worktree holding uncommitted
            work, registered here or not; registered here, so the teardown removes it and this
            content is destroyed
        # the worktree arm wins over the staged-prose arm, as the row claimed

RD-22 · `milestone discard` × S12+S13 × `--force` — one narration block per area
setup:  milestone rig (1 sub-task)
        printf 'TASKAREA\n' > .jigc/tasks/first-sub/notes.txt
        printf 'MSAREA\n'   > .jigc/milestones/axis-three-probe/mnotes.txt
argv:   jigc milestone discard axis-three-probe --force
exit:   0
stderr: warning: removing the working area .jigc/tasks/first-sub discards work that is not in git:
            .jigc/tasks/first-sub/notes.txt
          note: the working area is the only copy of these bytes — they are not recoverable.
        warning: removing the working area .jigc/milestones/axis-three-probe discards work that is
          not in git:
            .jigc/milestones/axis-three-probe/mnotes.txt
          note: … not recoverable.
stdout: discarded milestone:axis-three-probe (1 sub-task(s); workbench removed)
        # two blocks, one per area — and the direct sibling-door contrast for A3-1

RD-52 · `task discard` × S12 vs S7 precedence
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        jigc doc create adr --title "a decision" --task tidy-the-readme
        printf 'FOREIGN\n' > .jigc/tasks/tidy-the-readme/notes.txt
argv:   jigc task discard tidy-the-readme
exit:   1   blocking · task-discard.foreign-bytes — … 1 path(s) jigc did not write …
            .jigc/tasks/tidy-the-readme/notes.txt
        # foreign-bytes wins over staged-prose, as the row claimed
```

### 8.2 · Codex claims, each driven

The source pass carries **one numbered claim**, **five M51-row dispositions**, **four
destroying-axis consistency statements**, **one aggregate census claim** and **two adjacent-seam
statements**. Entered as CX-1 … CX-13 in the order they appear in the pass.

| id | Codex claim (one line) | verdict |
|---|---|---|
| **CX-1** | "No new source-grounded completeness defect found … refusing doors guard foreign bytes before removal, **while both finalizing doors displace them**"; expected behaviour, per its own proposed repro: "plant … **in every area it removes** … task/milestone finalize **move every entry byte-intact** beneath `.jigc/displaced/<id>/` and report each `{from,to}` pair" | **REFUTED** (two independent falsifying drives) |
| **CX-2** | M51 **C-1 CLOSED** — the five-door silent loss of a task working area's complement is structurally covered | **CONFIRMED** (driven at all five doors) |
| **CX-3** | M51 **D-1 CLOSED** — `task discard` / `uninstall` route a milestone sub-task's staged prose to `jigc milestone finalize`, saying task finalize refuses | **CONFIRMED** |
| **CX-4** | M51 **D-2 CLOSED** — `workbench removed` is selected from the aggregate teardown outcome | **CONFIRMED** (drove its own proposed repro) |
| **CX-5** | M51 **D-3 CLOSED** — the fail-closed staged-prose error converts through `repo_relative` | **CONFIRMED** (drove its own proposed repro) |
| **CX-6** | M51 **D-4 CLOSED** — the unreadable-worktrees-root route identifies `read_dir`, denies a git fault, names `--force` | **CONFIRMED** |
| **CX-7** | `LeftoverAt` dispatch is shape-complete; no surviving M49-style `is_dir()` bypass | **CONFIRMED** |
| **CX-8** | `--force` is attached only to the four refusing rows' common consent; the two `Displace` rows expose no consent, so `--force` cannot silently widen finalize | **CONFIRMED** |
| **CX-9** | `TASK_AREA_FILES` covers the fixed writer names; staged bodies require `<type>:<slug>.md`; foreign directory, file, symlink and wrong-shape entries stay in the destroying-door subject | **CONFIRMED** |
| **CX-10** | Writer rollback uses the same registry and **non-recursive** deletion, leaving a non-empty area intact — closing the hook-created-file bypass adjacent to this axis | **CONFIRMED** (driven; the pass proposed no repro for it) |
| **CX-11** | Census aggregate: "I found **no unguarded production removal of adopter bytes** outside an ownership, transaction, cache, temporary-file, refusal, narration, or displacement seam" — with `cli/task.rs:5981` guarded as "foreign complement displaced first" and `cli/milestone.rs:5538` as "foreign complement already refused or displaced" | **REFUTED** (same two drives as CX-1, plus the source coordinate the census mis-reads) |
| **CX-12** | `schema-conformance.home-vacated` is the **seventh** `STORE_EXIT_FLIPS` member; `ENVELOPE_OWED_CODES` is enforced centrally at `render.rs` | **CONFIRMED as consistent** — not an axis-3 behaviour claim; the axis-3 half of it (the destroying doors' codes are **not** `ENVELOPE_OWED_CODES` members, hence flatten) is driven at RD-29 |
| **CX-13** | The diff `577a0099..HEAD` shows **no schema-manifest or schema-file changes** — M52's zero-schema-hash-movement boundary is not violated | **CONFIRMED** (datum, not a drive) |

#### CX-1 · REFUTED — the falsifying data

Two drives, each independently sufficient. The claim's own words are *"both finalizing doors
displace them"* and *"in every area it removes … move every entry byte-intact"*.

```
FALSIFIER 1 — the milestone boundary's OWN area is never displaced (= driver defect A3-1)
setup:  fresh rig; jigc milestone create "axis three probe"
        jigc milestone add-task axis-three-probe "first sub" / "second sub"
        jigc milestone provision axis-three-probe
        (cd .jigc/worktrees/first-sub  && printf 'one\n' > one.txt && git add one.txt)
        (cd .jigc/worktrees/second-sub && printf 'two\n' > two.txt && git add two.txt)
        printf 'MILESTONE-AREA-PRECIOUS-RECON\n' > .jigc/milestones/axis-three-probe/mnotes.txt
        mkdir -p .jigc/milestones/axis-three-probe/sub
        printf 'DEEP-RECON\n' > .jigc/milestones/axis-three-probe/sub/deep.txt
before: find .jigc/milestones -type f ->
          …/base.json  …/mnotes.txt  …/record-commit-msg.txt  …/staged-snapshot.json
          …/sub/deep.txt  …/tasks.json
        git status --porcelain --ignored -- …/mnotes.txt  ->  !! .jigc/milestones/
argv:   jigc milestone finalize axis-three-probe
exit:   0
stdout: finalized 7720bc2 — Finalize milestone axis-three-probe (2 sub-tasks)
          modified docs/milestone-records/axis-three-probe.md / added one.txt / added two.txt
          3 files committed …
stderr: (0 bytes)
after:  find .jigc/milestones  -> .jigc/milestones            # the whole area gone
        ls .jigc/displaced     -> No such file or directory   # never created
        grep -rl 'MILESTONE-AREA-PRECIOUS-RECON\|DEEP-RECON' .  -> (nothing)

FALSIFIER 2 — when the displacement FAILS the area is removed anyway (= driver defect A3-2)
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        printf 'NOT A DIR\n' > .jigc/displaced
        printf 'PRECIOUS-A32-RECON\n' > .jigc/tasks/tidy-the-readme/notes.txt
        mkdir -p .jigc/tasks/tidy-the-readme/deep
        printf 'DEEP-A32-RECON\n' > .jigc/tasks/tidy-the-readme/deep/x.txt
        printf 'hello\n' >> README.md; git add README.md   (+ the commit doc filled)
argv:   jigc task finalize tidy-the-readme --format json
exit:   0
stderr: note: could not open .jigc/displaced/tidy-the-readme to move
          .jigc/tasks/tidy-the-readme/deep aside: Not a directory (os error 20)
        note: could not open .jigc/displaced/tidy-the-readme to move
          .jigc/tasks/tidy-the-readme/notes.txt aside: Not a directory (os error 20)
stdout: committed.displaced == []
after:  ls -a .jigc/tasks -> . ..
        grep -rl 'PRECIOUS-A32-RECON\|DEEP-A32-RECON' . -> (nothing)   # gone, at exit 0
        git log --oneline -1 -> d4cea6a feat: tidy the readme

FALSIFIER 2, the second Displace door
setup:  milestone rig + provision + staged code in both worktrees
        printf 'NOT A DIR\n' > .jigc/displaced
        printf 'PRECIOUS-MF-RECON\n' > .jigc/tasks/first-sub/tnotes.txt
argv:   jigc milestone finalize axis-three-probe
exit:   0
stderr: note: could not open .jigc/displaced/first-sub to move
          .jigc/tasks/first-sub/tnotes.txt aside: Not a directory (os error 20)
after:  ls -a .jigc/tasks -> . ..  ;  grep -rl 'PRECIOUS-MF-RECON' . -> (nothing)
```

#### CX-11 · REFUTED — and where the census mis-reads

Same drives. The coordinate the census gives a clean bill to is one function, and the census
reads it as if it had one caller:

`crates/cli/src/task.rs:5957-5983` — `post_commit(…, displace: Option<(&str, &mut Vec<Displaced>)>)`.
Its doc-comment (`task.rs:5950-5955`) states the two facts the census's guard description misses:
*"The milestone boundaries pass `None`: `cleanup_dir` is then a milestone area … and this door must
not answer for a subject it was not given."* And the body runs

```rust
if let Some((task_id, sink)) = displace { … displace_foreign_area(…) … }
if let Err(err) = std::fs::remove_dir_all(cleanup_dir) { eprintln!("note: … (self-heals): {err:#}"); }
```

— so the `remove_dir_all` is (a) reached with `displace == None` from both milestone boundaries
(falsifier 1) and (b) **not conditioned on the displacement having succeeded** in either caller
(falsifier 2). `displace_foreign_area` returns only the moves that *landed*, so a failed move
leaves the sink empty and the removal proceeds. The census's `cli/task.rs:5981` line ("foreign
complement displaced first") is true of the **task** caller's success path and of nothing else.

#### CX-2 … CX-10 · CONFIRMED — the drives

```
CX-2 (C-1) — the task-area complement at the doors that destroy an area
  milestone discard   : plant .jigc/tasks/first-sub/notes.txt
                        -> exit 1  blocking · milestone.foreign-bytes  naming the path,
                           route `… --force settles the record and tears the workbench down`
  uninstall           : same state -> exit 1  blocking · uninstall.foreign-bytes  naming the path
                        after both: cat .jigc/tasks/first-sub/notes.txt -> PRECIOUS-C1
  task discard        : 8 planted shapes -> exit 1 task-discard.foreign-bytes, all 8 named (CX-9)
  task finalize       : plant notes.txt + docs/attachment.txt, landable commit
                        -> exit 0, "note: the working area held 2 entries jigc did not write …
                           they were moved aside, not taken:" with both `<from> → <to>` lines;
                           cat .jigc/displaced/tidy-the-readme/notes.txt -> PRECIOUS-C1-TF
  milestone finalize  : the sub-task arm fires (seen firing in falsifier 2's `note:`), and the
                        driver's R-E(5) drove its success half
  VERDICT: CLOSED for the subject the claim names — the TASK area's complement. It is NOT closed
  for the milestone area (CX-1 falsifier 1); C-1's own wording was task-area-scoped, so this is a
  confirmation of the claim as written and not of the wider reading.

CX-3 (D-1) — driven inside CX-9's control:
  jigc task discard first-sub   (sub-task with a staged adr)
  -> exit 1 task-discard.staged-prose; route: "… land them with `jigc milestone finalize
     axis-three-probe` — this task is a sub-task of milestone `axis-three-probe`, whose boundary
     is the only one that commits it, so `jigc task finalize` refuses it — or … `--force` …"

CX-4 (D-2) — Codex's own proposed repro, run:
  milestone rig + provision; printf 'wip\n' > .jigc/worktrees/first-sub/wip.txt
  chmod 500 .jigc/worktrees ; jigc milestone discard axis-three-probe --force
  -> exit 0; stdout "discarded milestone:axis-three-probe (2 sub-task(s); workbench NOT fully
     removed — the warnings above name what is left)"; two `warning: could not remove the fan-out
     worktree …` + `remedy: run `git worktree prune`, then `git worktree remove --force …``
  after: ls .jigc/worktrees -> first-sub second-sub

CX-5 (D-3) — Codex's own proposed repro, run:
  fresh rig + task; chmod 000 .jigc/tasks/tidy-the-readme/docs
  jigc task discard tidy-the-readme -> exit 1 task-discard.foreign-bytes, "cannot check … 
    Permission denied (os error 13)", route names `.jigc/tasks/tidy-the-readme/` and `--force`
  jigc uninstall                    -> exit 1 uninstall.foreign-bytes, "cannot check `.jigc/` …"
  grep -c '/private/var\|/var/folders' on both stderrs -> 0 and 0   # no host path

CX-6 (D-4): milestone rig + provision; chmod 000 .jigc/worktrees; jigc uninstall
  -> exit 1 uninstall.dirty-worktree; route: "make sure `.jigc/worktrees/` is readable — this
     failure is a `read_dir` of that directory, not a git fault — then re-run `jigc uninstall`;
     or remove the worktrees yourself (`git worktree list`, then `git worktree remove`) … or …
     `jigc uninstall --force` deletes them with the install"

CX-7: milestone rig; .jigc/worktrees/first-sub = a plain file, second-sub = a symlink to /etc/hosts
  jigc milestone provision axis-three-probe
  -> exit 1 milestone.leftover-holds-work, BOTH named "the file itself — it is a file, not a
     worktree"   (the File arm sees a symlink as a File — no shape is invisible)
  jigc milestone provision axis-three-probe --force
  -> exit 0, one `warning: removing the leftover file …` per path, both removed and replaced by
     real worktrees; /etc/hosts intact  (the M49 `is_dir()` bypass would have left them standing)
  separate rig, chmod 000 .jigc/worktrees/first-sub -> exit 1, "unknown — could not read the
     leftover directory …: Permission denied (os error 13)"   (the Unreadable arm)

CX-8: jigc task finalize --help      -> --approve · --format · --dry-run · --carry-staged
      jigc milestone finalize --help -> --carry-staged · --format
      jigc task finalize <id> --force -> error: unexpected argument '--force' found
      (no consent exists at either Displace door — so `--force` cannot widen finalize, confirmed,
       and note this is also why A3-1/A3-2 have no operator escape hatch)

CX-9: eight planted shapes in one task area — dotfile · symlink · symlink at an address-shaped
  name under docs/ · non-.md under docs/ · a docs/*.md that is no staged id · a .md outside
  docs/ · a foreign root file · a nested directory
  jigc task discard tidy-the-readme -> exit 1 task-discard.foreign-bytes, "holds 8 path(s) jigc
    did not write", all eight named and sorted
  control: an area holding ONLY jigc's own files (base.json · docs · intent · renames.json ·
    roles.json · workflow, i.e. after a `doc create` + `doc rename --task`)
    -> exit 1 task-discard.STAGED-PROSE, never foreign-bytes   # zero false fire

CX-10 (the pass proposed no repro; this is the reconciler's):
setup:  fresh rig; jigc milestone create "axis three probe"
        .git/hooks/pre-commit:  writes .jigc/tasks/third-sub/hookfile.txt (+ hookdir/d.txt)
                                if that directory exists, then `exit 1`
argv:   jigc milestone add-task axis-three-probe "third sub"
exit:   1
out:    `git commit` was rejected (no commit was made): … apart from the 1 path named below,
          which survived the rollback: nothing was committed — the record append and the sub-task
          mint were both rolled back …
        blocking · milestone.foreign-bytes — `.jigc/tasks/third-sub` holds bytes jigc did not
          write, so the working area this call minted was left standing rather than removed with
          them …
          route: nothing was committed. Keep what you need from `.jigc/tasks/third-sub` and
            delete the rest — until that path is gone, the identical re-run blocks on the id this
            call already minted
after:  ls -a .jigc/tasks/third-sub -> . .. hookfile.txt        # jigc's own mint files gone,
        cat .jigc/tasks/third-sub/hookfile.txt -> HOOK-BYTES-RECON   # the hook's bytes survive
        (the unwind is registry-bounded and non-recursive, exactly as claimed, AND the door
         names the survivor with a code and a route)

CX-13 (a datum, not a drive):
  git diff --stat 577a0099..HEAD -- '*schema-manifest.yaml' 'crates/cli/pack/schemas/**' \
      'packs/methodology/schemas/**'        -> (empty)
  git diff 577a0099..HEAD -- '*schema-manifest.yaml' | grep -c '^[+-]'  -> 0
```

### 8.3 · Driver defects — status after reconciliation

| defect | source pass's position | reconciler's verdict |
|---|---|---|
| **A3-1 · HIGH** — `jigc milestone finalize` destroys the milestone's own working-area complement at exit 0, named by nothing | **contradicted** (CX-1: "both finalizing doors displace them … in every area it removes"; CX-11: no unguarded production removal of adopter bytes) | **STANDS — CONFIRMED, re-driven by the reconciler** (§8.2 falsifier 1: exit 0, both streams silent, `committed.displaced == []`, `.jigc/displaced` never created, bytes in no commit and nowhere on disk). The source claim is **REFUTED with that repro.** |
| **A3-2 · HIGH** — both `Displace` doors remove the area even when the displacement failed, printing only that the *move* failed | **contradicted** (same two Codex claims) | **STANDS — CONFIRMED, re-driven at both doors** (§8.2 falsifier 2 and its milestone-boundary twin). The source claim is **REFUTED with that repro.** Root coordinate: `task.rs:5957-5983`, where `remove_dir_all` is not conditioned on the displacement's outcome. |
| **A3-3 · LOW** — `milestone_boundary_displacement.rs`'s subject is the sub-task areas only, so the boundary's `displaced: []` assertion is true of a tree it never looked at | **silent** | **STANDS**, verified by datum: `grep -c 'milestones' crates/cli/tests/milestone_boundary_displacement.rs` → **0**. **One precision correction to the driver's wording:** the driver's A3-2 parenthetical says *"no suite in `crates/cli/tests/` contains the strings `could not open` or `could not move`"* — a tree-wide grep returns **3** hits (`root_knob_rules.rs:546`, `relocate.rs:293`, `config_relocation_rollback.rs:427`), all about `relocate`, none about the finalize displacement. The substantive claim — *the move-failure cell has no standing test at either `Displace` door* — holds: neither `finalize_displacement.rs` nor `milestone_boundary_displacement.rs` contains either string. The over-broad sentence is corrected, the finding is not weakened. |

### 8.4 · Open leads

**None.** Every Codex claim was driven to a repro or to a falsifying datum on this binary; no claim
required a state the rig cannot build or an environment I lack. Recorded explicitly so a reader can
tell an empty list from an unwritten one.

The two claims that are **not behaviour claims** are marked as such rather than parked: CX-12 is a
registry-count statement (its axis-3 half is driven at RD-29) and CX-13 is a `git diff`.

### 8.5 · Observations the reconciliation drive added

- **RO-1 · `uninstall.foreign-bytes`' route names `.jigc/displaced/` for a subject that is not
  there.** Driven with a single plant at `.jigc/tasks/first-sub/notes.txt` (no `.jigc/displaced/`
  in the repo at all), the route reads: *"move what you need out of the paths above, or delete the
  ones you do not (`rm -r` takes them — **jigc has no verb that clears `.jigc/displaced/`**), then
  re-run `jigc uninstall`"*. The parenthetical is the `.jigc/displaced/` subject's sentence
  (correct at the driver's row 36) rendered unconditionally. A surface wobble, not a destruction
  defect: the code, the named paths and the consent are all correct. Recorded at tier 3, origin
  reconciler.
- **RO-2 · the driver's O-2 is confirmed at `milestone discard`** (RD-29): the refusal flattens to
  `{"error": …}` on stderr with stdout empty, so a driver keying `(code, target)` at that door
  still cannot. Consistent with CX-12's `ENVELOPE_OWED_CODES` = 4, all `store.*` plus
  `FIXED_IDENTITY`.
- **RO-3 · `milestone discard --force` is A3-1's sharpest contrast, driven** (RD-22): over the
  *identical* `.jigc/milestones/<id>/mnotes.txt` plant it prints `warning: removing the working
  area .jigc/milestones/axis-three-probe … not recoverable.` — so three of the four doors over that
  area (discard refusing, discard forced, uninstall refusing) answer for it and the landed boundary
  does not.

---

## 9 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling
(`crates/cli/src/cli.rs:1834`):

| leaf | rows | note |
|---|---|---|
| `milestone provision` | driver 1–17, RD-17, CX-7 | `DESTROYING_DOORS` · `Refuse{--force}` |
| `milestone discard` | driver 18–30, RD-18/22/26/29/30, CX-2, CX-4 | `DESTROYING_DOORS` · `Refuse{--force}` |
| `uninstall` | driver 31–46, CX-2, CX-5, CX-6 | `DESTROYING_DOORS` · `Refuse{--force}` |
| `task discard` | driver 47–57, RD-52, CX-3, CX-5, CX-9 | `DESTROYING_DOORS` · `Refuse{--force}` |
| `task finalize` | driver 58–63, CX-1 falsifier 2, CX-2, CX-8 | `DESTROYING_DOORS` · `Displace` |
| `milestone finalize` | driver 64–70, CX-1 falsifier 1 + its twin, CX-8 | `DESTROYING_DOORS` · `Displace` |
| `milestone add-task` | CX-10 | **not** a `DESTROYING_DOORS` member — reached only as the door of CX-10's rollback lead (the registry-bounded, non-recursive unwind), and listed because the rule is "door of ≥1 driven row", not "door in the registry" |

`DESTROYING_DOORS` at `1.0.0-rc.16` has **6** members
(`crates/cli/src/milestone.rs:3275`: `PROVISION_DOOR` · `DISCARD_DOOR` · `UNINSTALL_DOOR` ·
`TASK_DISCARD_DOOR` · `TASK_FINALIZE_DOOR` · `FINALIZE_DOOR`) and **all six are covered**; the
seventh row above is the one non-registry leaf a Codex lead pulled in.
