# `(3, A3-2)` — verified baseline ledger fragment

**Binary:** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.16`. Repo HEAD `155054cc`. Every rig: `rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit 1; eval "$rig"` (two-step), `mktemp -d` roots, no teardown. No edits, no commits, no cargo.

**Harness correction that invalidates part of the review's own evidence.** In this shell `grep` is a function that execs `ugrep -G --ignore-files --hidden …` — `--ignore-files` honours `.gitignore`, and `.jigc/` is gitignored, so **`grep -rl 'MARKER' .` returns rc=1 whether the byte is there or not**. The review's R-I repro asserts loss with `grep -rl`. I re-drove every negative claim with `command grep` plus `find` plus `git log -S` (control in §1.5). The verdict is unchanged — the bytes really are gone — but the *evidence shape* in `axis-3.md` §3 R-I is unsound and the fixer's acceptance must not copy it.

---

## 1 · The cells, driven

| # | door | cell | argv | exit | stderr identity | `displaced` | bytes after | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `task finalize` | (a) `.jigc/displaced` is a regular file | `jigc task finalize tidy-the-readme --format json` | **0** | `note:` only, no code, no route | `[]` | **gone** (2 entries) | **CONFIRMED** |
| 2 | `task finalize` | (b) `.jigc/displaced/` chmod 555 | `jigc task finalize tidy-the-readme` | **0** | `note:` only | n/a (text) | **gone** | **CONFIRMED** |
| 3 | `milestone finalize` | (a) file at `.jigc/displaced` | `jigc milestone finalize axis-three-probe --format json` | **0** | `note:` only | `[]` | **gone** | **CONFIRMED** |
| 4 | `milestone finalize` | (b) chmod 555 | `jigc milestone finalize axis-three-probe` | **0** | 2 × `note:` | n/a | **gone** (2 sub-task areas) | **CONFIRMED** |
| 5 | `task finalize` | **safe-by-accident** — source `chmod 000` | same | 0 | `note: could not move …` + `note: post-commit working-area removal failed (self-heals)` | n/a | **survives** | CONFIRMED safe, *and partial* — see §1.4 |
| 6 | `task finalize` | **PARTIAL move** (not in the review, not in the charter's 2×2) | same | **0** | one `note:` fail + a success sentence saying *"held 1 entry"* | 1 pair | **1 of 2 gone** | **NEW — law-1 lie** |
| 7 | `task finalize` | collision (same id twice) | same ×2 | 0 | `→ …/notes.txt.2` | 2 pairs | both intact | SAFE (review row 60 re-confirmed) |

### 1.1 Cell (a) at `jigc task finalize` — DRIVEN

```
REPO=/var/folders/…/jigc-rig-fresh-xk00eV/repo   jigc 1.0.0-rc.16
setup: jigc start --workflow quick-fix "tidy the readme"
       printf 'NOT A DIR\n' > .jigc/displaced
       printf 'PRECIOUS-A3-2\n' > .jigc/tasks/tidy-the-readme/notes.txt
       mkdir -p …/deep && printf 'DEEP-A3-2\n' > …/deep/x.txt
       printf 'hello\n' >> README.md; git add README.md
       jigc doc set-field 'commit:tidy-the-readme#header/type' --task … --value feat
       printf 'tidy the readme\n' | jigc doc set-slot 'commit:tidy-the-readme#summary' --task … --from-file -

$ jigc task finalize tidy-the-readme --format json
EXIT=0
--- STDERR ---
note: could not open .jigc/displaced/tidy-the-readme to move .jigc/tasks/tidy-the-readme/deep aside: Not a directory (os error 20)
note: could not open .jigc/displaced/tidy-the-readme to move .jigc/tasks/tidy-the-readme/notes.txt aside: Not a directory (os error 20)
finalize — about to commit the index; leaving out:
  left-out (unstaged/untracked — git add to include):
    .jigc/displaced
--- STDOUT ---  "displaced": []
--- AFTER ---   find .jigc/tasks .jigc/displaced  ->  .jigc/displaced
                                                     .jigc/tasks          # area gone, empty
$ git log --oneline -1   ->  0dfae5b feat: tidy the readme      # the commit LANDED
```

### 1.2 The sound negative evidence (re-drive of cell (a), marker `PRECIOUS-SOUND`) — DRIVEN

```
BEFORE: command grep -rl 'PRECIOUS-SOUND' .        ->  ./.jigc/tasks/tidy-the-readme/notes.txt   rc=0
$ jigc task finalize tidy-the-readme               ->  EXIT=0
  stderr: note: could not open .jigc/displaced/tidy-the-readme to move …/notes.txt aside: Not a directory (os error 20)
AFTER:  command grep -rl 'PRECIOUS-SOUND' .        ->  rc=1
        command grep -rl 'PRECIOUS-SOUND' "$RIG"   ->  rc=1        # the whole rig root, not just the repo
        git grep -l 'PRECIOUS-SOUND' HEAD          ->  rc=1
        git log -S'PRECIOUS-SOUND' --oneline --all ->  (empty)     # in no git object
        git fsck --lost-found                      ->  (empty)     # no dangling blob
        find .jigc  ->  .jigc/.gitignore .jigc/AGENT.md .jigc/config/… .jigc/displaced
                        .jigc/index/… .jigc/state/… .jigc/tasks .jigc/version
```

**Incidental (DRIVEN, one line):** `.jigc/.gitignore`'s `ENTRIES` is `…displaced/\n` with a trailing slash (`crates/cli/src/gitignore.rs:38`), so the cell-(a) plant — a *file* named `displaced` — is **not** ignored and surfaces in finalize's own `left-out` manifest. git sees the thing that caused the loss; nothing sees the loss.

### 1.3 Cell (b) at both doors — DRIVEN

```
# task finalize, .jigc/displaced/ = dr-xr-xr-x
$ jigc task finalize tidy-the-readme     EXIT=0
stderr: note: could not open .jigc/displaced/tidy-the-readme to move .jigc/tasks/tidy-the-readme/notes.txt aside: Permission denied (os error 13)
stdout: no findings — the task validates clean
        finalized 9872c5b — feat: tidy the readme
          modified README.md
          1 file committed
after:  find .jigc/tasks -> .jigc/tasks     (empty);  command grep/git grep/git log -S: rc=1

# milestone finalize, .jigc/displaced/ = dr-xr-xr-x, two sub-task areas planted
$ jigc milestone finalize axis-three-probe   EXIT=0
stderr: note: could not open .jigc/displaced/first-sub  to move .jigc/tasks/first-sub/tnotes.txt aside: Permission denied (os error 13)
        note: could not open .jigc/displaced/second-sub to move .jigc/tasks/second-sub/sub aside: Permission denied (os error 13)
stdout: finalized baf7bd4 — Finalize milestone axis-three-probe (2 sub-tasks)
          modified docs/milestone-records/axis-three-probe.md / added one.txt / added two.txt
          3 files committed
after:  find .jigc/tasks -> .jigc/tasks     (both areas gone, both plants gone)
```

Cell (a) at `milestone finalize` is identical (`Not a directory (os error 20)`, `"displaced": []`, exit 0, `9a6e898 Finalize milestone axis-three-probe (2 sub-tasks)` landed).

### 1.4 The "safe by accident" cell is only **partly** safe — DRIVEN, correction to the review

The review records this cell (row 62) as *"bytes survive"*. They do — but `remove_dir_all` is **not atomic**: it removes everything it can before hitting the EACCES, so the area is left as a **skeleton**.

```
setup: mkdir -p $T/secret && printf 'PRECIOUS-SAFE\n' > $T/secret/keep.txt && chmod 000 $T/secret
$ jigc task finalize tidy-the-readme    EXIT=0
stderr: note: could not move .jigc/tasks/tidy-the-readme/secret aside to .jigc/displaced/tidy-the-readme/secret: Permission denied (os error 13)
        note: post-commit working-area removal failed (self-heals): Permission denied (os error 13)
after (chmod 755 back):  ls -a $T  ->  .  ..  secret          # base.json, docs/, intent, workflow,
                         cat $T/secret/keep.txt -> PRECIOUS-SAFE   # staged-snapshot.json ALL removed
```

That skeleton is today's live instance of the state the charter's fix would create — see §3(iii).

### 1.5 Control proving the `grep` caveat

```
$ cd <fresh dir with NO .gitignore>; grep -rl 'PRECIOUS-CTL' .  ->  .jigc/x/f.txt  rc=0
$ type grep  ->  grep () { … ARGV0=ugrep "$_cc_bin" -G --ignore-files --hidden … }
```

---

## 2 · The code — file:line, and the decisive before/after-commit fact

| thing | file:line | signature / fact |
|---|---|---|
| **the displacement primitive** | `crates/cli/src/task.rs:5849` | `pub(crate) fn displace_foreign_area(repo_root, jigc_root, area: &Path, kind: state::WorkArea, unit_id: &str) -> Vec<render::Displaced>` — **returns no `Result` at all**. There is no error for a caller to drop; there is no error. |
| granularity | `task.rs:5871-5894` | **per entry** of `state::foreign_area_paths` (a foreign directory is one entry, moved whole). `create_dir_all(parent)` failure → `eprintln!` + **`continue`**; `fs::rename` failure → `eprintln!`, entry simply not pushed. |
| **partial move** | same | structurally normal and unsignalled: `moved` carries only successes, and the caller cannot tell "moved 1 of 1" from "moved 1 of 2". |
| no-clobber | `task.rs:5902` `free_displacement_path` | `<name>.2`, `.3`, … unbounded `loop` (theoretical hang on a pathological tree; READ, not driven) |
| narration | `task.rs:5924` `narrate_displacement(moved: &[render::Displaced])` | prints only over `moved` — *"the working area held N entries … moved aside, not taken"*, where **N is what moved, not what was there** |
| **the removal** | `crates/cli/src/task.rs:5981` | `if let Err(err) = std::fs::remove_dir_all(cleanup_dir) { eprintln!("note: post-commit working-area removal failed (self-heals): {err:#}"); }` — **unconditional**; nothing reads `moved` |
| its function | `task.rs:5956` `fn post_commit(...)` | doc-comment `task.rs:5942`: *"**Phase 7** (`design/finalize.md` → 7. **Post-commit**, best-effort) … Each step self-heals on failure, so a failure is logged, never raised (**the commit is already truth**)"* |
| **call site 1** — `task finalize` | `crates/cli/src/task.rs:2859` | `Some((&self.id, &mut displaced_foreign))` into `try_execute_finalize_plan` → `post_commit` |
| **call site 2/3** — `milestone finalize` (`squash:true` and the N+1 chain) | `crates/cli/src/milestone.rs:5098` and `:5228` | pass `None`; the executor's `cleanup_dir` is then the **milestone** area, removed by the same unconditional `remove_dir_all` at `task.rs:5981` with **no displacement at all** — that is `(3, A3-1)`, the sibling row |
| **call site 4** — sub-task areas at the boundary | `crates/cli/src/milestone.rs:5528` (displace) / `:5538` (removal) | `fn cleanup_subtask_areas(jigc_root, list, complement: SubtaskComplement) -> bool` (`milestone.rs:5510`); the removal is `if let Err(err) = std::fs::remove_dir_all(&area) { eprintln!(…); all_gone = false; }` — **also unconditional**, and `all_gone` is *"returns whether every area it reached is gone"*, **ignored by both landed arms** (its doc-comment says so: *"The landed finalize arms ignore it"*) |

**BEFORE or AFTER the commit — the decisive fact (READ, corroborated DRIVEN).** **AFTER, at every one of the four call sites.** Phase 7 runs only on the executor's `Ok(hook_output)` arm; the commit sha is already `HEAD`. Driven confirmation: cell (a) exits 0 with `git log -1 -> 0dfae5b feat: tidy the readme`, and the review's row 63 (hook-rejected commit) shows nothing is displaced and nothing removed on the refusal arm.

**Transactional / rollback scope: none.** `ROLLBACK_POPULATIONS` (`crates/cli/src/rollback.rs:169`) has **11** rows — `config-layer-worktree`, `promote-destination`, `retired-original`, `milestone-record`, `fan-out-record-flip`, `rename-worktree`, `created-doc-staged-write`, `milestone-mint-area`, `unrecorded-seed-areas`, `config-root-relocation`, `setup-install-path`. **Not one covers the phase-7 working-area removal**, and by construction none can: every population is a *pre-commit* capture restored when the commit does not land, and this removal happens only when it did. `milestone-mint-area` is the *mint* unwind, a different interval.

**The shipped safe primitive this call site does not use** — `crates/engine/src/state.rs:385`:
```rust
pub fn unwind_area(area: &Path, kind: WorkArea) -> Result<AreaUnwind, AreaUnwindError>
```
Its doc-comment (`state.rs:~364`): *"**The safety is structural, not a check.** Nothing is enumerated and then deleted: each removal names a registry member, and the directory itself goes through `remove_dir`, which refuses a non-empty directory. So a third party's file — including one written after this walk read the directory — survives by construction, and the caller is told (`AreaUnwind::Foreign`)."* It is `pub`, keyed on the same `WorkArea::jigc_written` registry (`TASK_AREA_FILES` / `MILESTONE_AREA_FILES`, `state.rs:129/168`) that `foreign_area_paths` complements, and already serves the mint doors from `crates/cli/src/milestone.rs:1063`.

---

## 3 · The halt question, answered

### (i) Do the two `Displace` doors have an existing `*.foreign-bytes` code? **No. The charter's premise is false.**

| registered code | producer(s) | file:line |
|---|---|---|
| `milestone.foreign-bytes` | `jigc milestone discard`'s refusal **and** the mint-unwind at `milestone create` / `add-task` / `add-from-spec` | `crates/cli/src/milestone.rs:4049` (`DISCARD_FOREIGN_BYTES_CODE`), member of `DISCARD_DOOR.codes` `:3194` |
| `uninstall.foreign-bytes` | `jigc uninstall` | `crates/cli/src/setup.rs:3328`, `:3347`; `milestone.rs:3213` |
| `task-discard.foreign-bytes` | `jigc task discard` | `crates/cli/src/task.rs:847` |

Those are the **only three**. And:

```rust
pub const TASK_FINALIZE_DOOR: DestroyingDoor = { verb: "jigc task finalize", disposition: Displace, codes: &[] };  // milestone.rs:3245
pub const FINALIZE_DOOR:      DestroyingDoor = { verb: "jigc milestone finalize", disposition: Displace, codes: &[] };  // milestone.rs:3263
```
with the field's own doc-comment (`milestone.rs:3116`): *"Empty ⇔ the door refuses over nothing it destroys."*

`task.rs:826-847` forecloses reuse explicitly: `task-discard.foreign-bytes` is *"The **task** door's foreign-byte code — **its own, never a sibling door's** … the two other doors that refuse over this subject destroy different things and their routes lead different ways."* The `finalize.*` family has 19 registered members (`base-mismatch`, `carried-staged`, `commit-rejected`, `empty-commit`, `fan-out`, `forward-ref-dangling`, `left-out`, `migration-no-replacement`, `milestone-sub-task`, `no-task`, `nothing-staged`, `promote-clobber`, `promote-io`, `provenance-io`, `render-io`, `retire-untrackable`, `rollback-conflict`, `source-path-io`, `stage-failed`) — **none is a foreign-bytes code.**

**So: `jigc milestone finalize` could arguably reuse `milestone.foreign-bytes` (same namespace, and it already means exactly *"the working area was left standing rather than removed with them"* — driven in §3(ii) below). `jigc task finalize` carries nothing, and minting `task-finalize.foreign-bytes` is a new finding code. The charter's own boundary permits that — *"no new finding code **unless the row's door already carries none**"* — and this door carries none. But the two doors would then answer the same class with codes minted under different warrants, so this is a fork the human should see rather than a fixer's pick.**

### (ii) The removal runs AFTER the commit. What can "refuse" mean?

It cannot mean refuse-the-commit: the commit is `HEAD` and the run is at exit 0 with a landed-envelope document already composed. The honest options the existing code admits:

**Option A — make the removal safe-by-construction instead of refusing (no new mechanism at all, and my recommendation to put to the human).** Replace `remove_dir_all` at `task.rs:5981` and `milestone.rs:5538` with the shipped `engine::state::unwind_area(area, kind)`. A failed displacement then leaves *exactly* the un-moved entries standing, `remove_dir` refuses the non-empty directory, and the caller receives `AreaUnwind::Foreign` — the "left in place" signal, already typed. **Needs today:** the door's answer for `Foreign` on a *landed* run (see (iii)) and a decision on exit code. **Needs nothing new:** primitive, registry, disposition, all shipped.

**Option B — post-commit leave-in-place + a carried finding.** `post_commit`/`cleanup_subtask_areas` return the outcome (the boolean already exists at `milestone.rs:5510` and is thrown away); the landed arm carries a blocking or advisory finding beside `render::Landed`. **Needs today:** a code (see (i)); an exit code — the landed arm returns `Ok(Outcome::with_findings(0, &report.findings))` (`task.rs:2929`), and turning that non-zero on a landed commit is a contract change against `design/command-output-contract.md`'s *"the landed arm (exit 0)"*; and a JSON home. **The envelope's existing keys can carry it without a new key**: the landed document already inserts `findings` beside `committed`, and `committed.displaced` is already `[{from,to}]` — but there is **no key for a *failed* move**, so the list of un-displaced paths would have to ride the finding's `target`/message or a new key (new key = new mechanism, and the additive-key window closed at M48).

**Option C — pre-flight the destination *before* the commit.** Probe `.jigc/displaced/<unit-id>` usability in the validate/preflight phase and block at exit 3 with a route, so the commit never lands over an unusable parking home. **Needs today:** a preflight seam that runs per-unit *and knows the complement* — the complement is computed in phase 7 only; `jigc task validate`'s previewed set (`GATE_COVERAGE ▸ Tier::Previewed`) has no displacement member. This is the largest of the three and reads as new mechanism.

### (iii) What a left-in-place area does to later doors — **DRIVEN, and it is a real lying-state sibling**

`jigc task list` / `jigc start` derive *active* from the **area's existence on disk**, not from recorded state. Two simulations, both through the real binary.

**(iii-a) whole task area restored after a landed `task finalize`** (`cp -a` snapshot before, restored after — exactly what Option A/B would leave):

```
finalize exit=0 ; HEAD=92e5e4f feat: tidy the readme        # the commit landed
$ jigc task list            EXIT=0   jigc task list — 1 active task(s)
                                       tidy-the-readme  [quick-fix]  tidy the readme
$ jigc start                EXIT=0   Active task: tidy-the-readme
                                       workflow: quick-fix   intent: tidy the readme
                                       staged:   commit:tidy-the-readme      findings: none
                                     Run: `jigc task finalize tidy-the-readme`   — validate + commit
$ jigc task validate …      EXIT=0   no findings — the task validates clean
$ jigc task finalize …      EXIT=3   blocking · finalize.empty-commit — task validated but produced no diff
                                       route: … or abandon it with `jigc task discard … --force`
$ git log --oneline         ->  92e5e4f (unchanged — NO double commit)
```
So: no second commit (good), but `task list`, `start` and `task validate` all lie at exit 0 about a task that finalized, and the only door that tells the truth does so under a *wrong* code (`finalize.empty-commit`).

**(iii-b) a sub-task area restored after a landed `milestone finalize` — worse:**

```
boundary EXIT=0
$ jigc task list                EXIT=0   1 active task(s):  first-sub  [sub-task]  first sub
$ jigc start                    EXIT=0   Active task: first-sub … Run: `jigc task finalize first-sub`
$ jigc task discard first-sub   EXIT=0   discarded task first-sub
                                         record commit: 23ada59 — this sub-task's milestone record,
                                           settled to `discarded` and committed on its own
$ jigc milestone finalize …     EXIT=1   blocking · milestone.terminal — milestone `axis-three-probe` is `joined`
```
A **joined** sub-task was settled to `discarded` and that mutation was **committed** — a lying committed record, the class M42's `milestone discard` exists to prevent, reachable from a left-in-place area at exit 0.

**(iii-c) the partial skeleton (today's live state, §1.4):** `task list` → *1 active task(s) tidy-the-readme*; `start` → *Active task … workflow: none recorded … findings: none*; `task validate` → *validates clean*, exit 0; but `task finalize` and `start --task` dead-end at **exit 1** with an **unregistered, code-less, route-less** message carrying a **host absolute path** — a standing law-1 violation in its own right:
```
$ jigc task finalize tidy-the-readme            EXIT=1  (bare, not piped)
stderr: could not read the base pin at "/private/var/folders/…/repo/.jigc/tasks/tidy-the-readme/base.json": No such file or directory (os error 2)
$ jigc task finalize tidy-the-readme --format json   EXIT=1
stderr: { "error": "could not read the base pin at \"/private/var/folders/…\": No such file or directory (os error 2)" }
```

**The consequence for the charter:** *"leaves the area in place"* contradicts a rule stated in two locked design docs, and the rule's stated *premise* is driven true. `design/storage.md:291`: *"The **teardown itself is not skippable** — a left-over area makes `jigc task list` report an active task that finalized — so a move that cannot be made is said out loud rather than turned into a silent retry."* `design/finalize.md:157`: *"The removal itself is **never skipped** — a left-over area makes `jigc task list` report an active task that finalized."* Same sentence in `task.rs:5839-5843` and in `crates/cli/tests/finalize_displacement.rs`'s module doc. **The fix must revise all four homes and answer the lying-state it creates — that is the halt, not the code change.**

---

## 4 · Sibling cells of the class — *fallible preserve followed by unconditional destroy*

Production sites (test-guard `Drop` impls excluded — ~60 of the 100 `remove_dir_all` hits are `let _ = …(&self.0)` in `#[cfg(test)]` guards):

| # | site | door(s) | preserve → destroy | verdict |
|---|---|---|---|---|
| 1 | `crates/cli/src/task.rs:5972-5983` | `task finalize` | `displace_foreign_area` (no `Result`) → `remove_dir_all` | **UNSAFE — DRIVEN** (§1.1–1.2) |
| 2 | `crates/cli/src/milestone.rs:5528-5543` | `milestone finalize` (both landed arms) | same → `remove_dir_all(&area)` | **UNSAFE — DRIVEN** (§1.3) |
| 3 | `crates/cli/src/task.rs:5981` via `milestone.rs:5098/:5228` (`displace: None`) | `milestone finalize` | **no preserve at all** → `remove_dir_all` of `.jigc/milestones/<id>/` | **UNSAFE** — this is `(3, A3-1)`, the adjacent chartered row; same `remove_dir_all` call, so a fix at site 1 touches it |
| 4 | `crates/engine/src/state.rs:385` `unwind_area` | mint unwind at `milestone create` / `add-task` / `add-from-spec` | registry-keyed removal + non-recursive `remove_dir` | **SAFE by construction — DRIVEN**, see below |
| 5 | `crates/cli/src/task.rs:618` `remove_dir_all(&task.dir)` | `task discard` | guarded *before* by `task-discard.foreign-bytes` / `.staged-prose`, `--force` the consent | **SAFE (consent, not preserve)** — review rows 50–53 |
| 6 | `crates/cli/src/milestone.rs:4463` `remove_milestone_area` + `cleanup_subtask_areas(Take)` | `milestone discard` | guarded before by `milestone.foreign-bytes`, `--force` consents + narrates | **SAFE (consent)** — review rows 20–22 |
| 7 | `crates/cli/src/setup.rs:2785` `remove_dir_all(&jigc_dir)` | `uninstall` | four guards before (`setup.rs:2727` *"the WIP guards, BEFORE anything is removed"*), `--force` consents | **SAFE (consent)** — review rows 31–39 |
| 8 | `crates/cli/src/milestone.rs:2920/2925` `LeftoverAt::{Directory,File}` | `milestone provision --force` | guarded before by `milestone.leftover-holds-work` | **SAFE (consent)** — review rows 1–15 |
| 9 | `crates/cli/src/relocate.rs:756` `displace_foreign_squatter` | `config set docs-root` / `placement-root`, `relocate` | `git rm --cached` → `fs::rename(&dest_abs, &workbench_abs).with_context(…)?` — returns `Result`, **propagated** by `relocate.rs:703`; no destroy follows | **SAFE** — a failed park aborts; bytes stay at the source (index entry already dropped is a *separate*, non-destroying residue) |
| 10 | `crates/cli/src/rollback.rs:735-760` (`PreImageFamily::restore`) | 10 `FileCas` populations | the `remove_file` is **inside** the compare-and-swap (`holds_jigcs_bytes`), else the pre-image parks + `<door>.rollback-conflict` | **SAFE** — M52 Inc 5 |
| 11 | `crates/cli/src/migrate_corpus.rs:994-1000` | `migrate-corpus` relocation | `engine::state::persist(dest)?` **then** `remove_file(src)?` — comment *"WRITE-BEFORE-REMOVE … never zero copies"* | **SAFE** |
| 12 | `crates/engine/src/milestone.rs:1890-1893` `remove_dir_all(&docs_dir)` at `materialize` | `milestone finalize` / `execute` | clears `.jigc/milestones/<id>/merged/docs/` wholesale with **no complement probe**; `merged/` is a declared jigc-owned `MILESTONE_AREA_FILES` member whose registry row states *"`merged/` is jigc's wholesale and nothing inside it is walked"* (`state.rs`, `unwind_area`'s `WorkArea::Milestone` arm) | **SAFE by declaration, not by check** — a lead, not a finding: the declaration is what makes it safe, and it is the same declaration `unwind_area` honours |

**The (4) control, DRIVEN — this is the fix shape already shipped and working:**

```
setup: milestone create; a pre-commit hook that writes .jigc/tasks/first-sub/hook-wrote-this.txt and exits 1
$ jigc milestone add-task axis-three-probe "first sub"      EXIT=1
stderr: `git commit` was rejected (no commit was made):
        hook: refusing
        apart from the 1 path named below, which survived the rollback: nothing was committed …
        blocking · milestone.foreign-bytes — `.jigc/tasks/first-sub` holds bytes jigc did not write,
          so the working area this call minted was left standing rather than removed with them —
          `.jigc/` is gitignored, so nothing else has a copy of what is there
          at: .jigc/tasks/first-sub
          route: nothing was committed. Keep what you need from `.jigc/tasks/first-sub` and delete
            the rest — until that path is gone, the identical re-run blocks on the id this call already minted
after: .jigc/tasks/first-sub/hook-wrote-this.txt  ->  HOOK-PLANT
```

### 4.1 The partial-failure cell — NEW, not in the review, not in the charter's 2×2 — DRIVEN

The charter's axis is `{move succeeds, move fails} × {removal succeeds, removal fails}`. The move axis has a **third** value: *some entries moved, some did not*, reachable without touching source permissions.

```
setup: mkdir -p .jigc/displaced/tidy-the-readme
       printf 'OCCUPIED BY A FILE\n' > .jigc/displaced/tidy-the-readme/docs   # blocks only the docs/ entry
       printf 'PARTIAL-MOVES\n' > $T/root-foreign.txt
       printf 'PARTIAL-DIES\n'  > $T/docs/non-md.txt
$ jigc task finalize tidy-the-readme --format json     EXIT=0
stderr: note: could not open .jigc/displaced/tidy-the-readme/docs to move
          .jigc/tasks/tidy-the-readme/docs/non-md.txt aside: File exists (os error 17)
        note: the working area held 1 entry jigc did not write, and removing it would have destroyed
          bytes no commit has a copy of — they were moved aside, not taken:
            .jigc/tasks/tidy-the-readme/root-foreign.txt → .jigc/displaced/tidy-the-readme/root-foreign.txt
after:  command grep -rl 'PARTIAL-' .  ->  ./.jigc/displaced/tidy-the-readme/root-foreign.txt   (only)
        find .jigc/tasks  ->  .jigc/tasks        # PARTIAL-DIES permanently gone
```
The success sentence says the area **held 1 entry**. It held **2**, and the second was destroyed. The count comes from `moved.len()` (`task.rs:5936`), not from the complement — a law-1 lie that survives even if the move-failure cell is otherwise fixed, and the reason the fixer's axis needs `{all move, some move, none move}`, not `{succeeds, fails}`.

### 4.2 Collision cell — SAFE, re-confirmed DRIVEN

```
round 1: .jigc/tasks/tidy-the-readme/notes.txt → .jigc/displaced/tidy-the-readme/notes.txt
round 2: .jigc/tasks/tidy-the-readme/notes.txt → .jigc/displaced/tidy-the-readme/notes.txt.2
$ cat .jigc/displaced/tidy-the-readme/notes.txt    -> COLLIDE-1
$ cat .jigc/displaced/tidy-the-readme/notes.txt.2  -> COLLIDE-2
```
No overwrite (`free_displacement_path`, `task.rs:5902`, `symlink_metadata`-keyed so a dangling symlink counts as occupied).

---

## 5 · Standing tests and doc homes the fix must keep true (or revise)

| home | what it states | effect of the fix |
|---|---|---|
| `crates/cli/tests/finalize_displacement.rs` (module doc + arms) | axis = **{populated, omitting}** × {stderr, JSON}; states *"The teardown itself is never skipped — a left-over area makes `jigc task list` report an active task that no longer exists."* | **no move-failure cell exists**; the quoted sentence must be revised with the fix |
| `crates/cli/tests/milestone_boundary_displacement.rs` | axis = **{landed arms}** × {stderr, `committed.displaced`}, plus the omitting cell; subject is `.jigc/tasks/<sub-id>` only — the string `milestones` appears nowhere (this is review finding A3-3) | must gain the move-failure axis **and** the milestone-area subject |
| `crates/cli/tests/flow53_acceptance.rs` arm 3 | per-cell assertion = *"`task finalize` and `milestone finalize` move the complement to `.jigc/displaced/<id>/<relative>` and name it"* — the **success** cell | the acceptance arm to extend |
| `crates/cli/tests/task_area_writer_registry.rs` | count-fences `TASK_AREA_FILES` (13) / `MILESTONE_AREA_FILES` in both crates | untouched by an Option-A fix (same registry) |
| `crates/cli/tests/rollback_population_registry.rs` | fences `ROLLBACK_POPULATIONS` at 11 rows, and that a population's code stays **out** of `ERROR_CODE_REGISTRY` | a new population row would red this — Option A adds none |
| `design/storage.md:125` | *"`displaced/` … it holds bytes that were somewhere else until jigc moved them, so it is **the one gitignored subdir whose contents no operation can reconstruct**"* | the destination's unusability is therefore not recoverable — supports refusing/leaving rather than retrying |
| **`design/storage.md:291`** | *"The **teardown itself is not skippable** — a left-over area makes `jigc task list` report an active task that finalized — so a move that cannot be made is **said out loud rather than turned into a silent retry**."* | **directly contradicted by the charter's fix; premise driven TRUE (§3-iii)** — must be revised, not silently broken |
| **`design/finalize.md:157`** | *"The removal itself is **never skipped**…"* + `:159` *"a stale `.jigc/tasks/<id>/` gets cleaned up by `jigc task discard <id>` or the next `finalize` reusing the slot"* — the second clause is **driven false** for the partial skeleton (§iii-c: re-finalize exits 1 on the missing base pin) | both clauses need revision |
| `design/team-ready-state.md:84` | *The working area's two populations* — the membership home | untouched |
| `crates/cli/src/milestone.rs:3097` (`DESTROYING_DOORS` doc) | *"Every door **answers for what it removes** ([`PendingLoss`], read before the removal and printed after it, so neither half of the claim can be false) — a door that destroys what it never named is the law-1 half-truth"* | the rule the defect violates; `PendingLoss`' read-before/print-after discipline is the shipped pattern the `Displace` arm does **not** use |
| `crates/cli/src/milestone.rs:3081` (`Disposition::Displace`) | *"**Keep it**: move it aside with its relative path preserved, and name where it went"* | the contract; a fix should make the variant's statement true rather than reword it |
| `design/finalize.md:215` (the M52 rollback table) | already records the precedent: *"at `8f0fb833` the unwind was a bare `remove_dir_all`, so a `pre-commit` hook that wrote a file into the area it was about to remove had that file destroyed at exit 1 … the honest unwind is to remove that row and then the directory **non-recursively**"* | **the same class, already decided the Option-A way one door over** — the strongest argument that Option A is not new mechanism |

---

## 6 · Ledger summary

| capability | status | evidence | gap |
|---|---|---|---|
| `displace_foreign_area` moves a task/sub-task area's complement to `.jigc/displaced/<id>/<relative>`, preserving trees, symlinks and relative paths; names each move on stderr and on `committed.displaced` | **built + proven** | review R-E/R-F; §4.2 re-drive | — |
| no-clobber at the parking home (`.2`, `.3`, …) | **built + proven** | §4.2 DRIVEN | unbounded `loop` at `task.rs:5906` (READ, not driven) |
| `Disposition::Displace`'s *"Keep it"* contract at `jigc task finalize` | **latent defect** | §1.1, §1.2, §1.3 DRIVEN | destination unusable → area removed anyway, exit 0, `displaced: []`, bytes in no commit and no git object |
| `Disposition::Displace`'s *"Keep it"* contract at `jigc milestone finalize` (sub-task areas) | **latent defect** | §1.3 DRIVEN, both cells, both formats | same |
| the displacement narration's count (*"held N entries … moved aside, not taken"*) | **latent defect (new)** | §4.1 DRIVEN | N = `moved.len()`, not the complement's size → a partial move reports a smaller area than existed, and the missing entry is destroyed |
| the area removal is inside a rollback population | **stubbed / inert (claim does not exist)** | `rollback.rs:169`, 11 rows read | no population covers it and none can — it runs only after the commit landed |
| `<door>.foreign-bytes` identity at the two `Displace` doors | **deferred-by-design → charter premise false** | `milestone.rs:3245/3263` `codes: &[]`; the three registered producers at `milestone.rs:4049`, `setup.rs:3328/3347`, `task.rs:847` | the charter's *"the door's existing `foreign-bytes` identity"* does not exist at either door; `task finalize` has none in any namespace |
| *"the teardown is never skipped, because a left-over area makes `task list` report an active task that finalized"* | **built + proven (the rule's premise)** | §3-iii-a/b/c DRIVEN — `task list` *1 active task(s)*, `start` *Active task*, `task validate` exit 0 clean, and a **joined sub-task discarded + committed** at exit 0 | the charter's fix creates this state; it is stated in 4 homes and must be revised, and the milestone half produces a lying **committed** record |
| `engine::state::unwind_area` — registry-keyed, non-recursive, `AreaUnwind::Foreign` when a third party's byte remains | **built + proven, and unused by the two `Displace` doors** | §4 control DRIVEN: `milestone add-task` under a planting rejecting hook left the area standing, raised `milestone.foreign-bytes` located at it, with a route, and the hook's byte survived | the safe primitive is shipped; `task.rs:5981` and `milestone.rs:5538` call `remove_dir_all` instead |
| the move-failure axis has a standing test | **absent** | `finalize_displacement.rs` / `milestone_boundary_displacement.rs` / flow53 arm 3 read; no suite contains `could not open` or `could not move` | three suites to extend, on `{all move, some move, none move} × {removal succeeds, fails} × {task finalize, milestone finalize}` — **12 cells, not 4** |

**Verified at `155054cc` / `jigc 1.0.0-rc.16`. A map, not gospel. I settled nothing — §3(i) (no existing code at either door), §3(ii) (Option A vs B vs C) and §3(iii) (the four doc homes that state the opposite rule, and the joined-sub-task committed record) are the human's forks.**
