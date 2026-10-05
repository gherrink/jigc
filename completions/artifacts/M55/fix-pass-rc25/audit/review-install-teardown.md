# Code review — install-teardown (`jigc setup`, `jigc uninstall`, the stamp at `task finalize`)

Tree: `fix/rc24-tier1` at `5f5b273a`. Commits read: `12398ddc` · `dc0d7586` · `104a7d4b` · `ef9456b5` · `4fa1c0f5`.
Driven on a release binary built from that tree in a private target dir; controls on the published `jigc 1.0.0-rc.24`. git 2.54.0, macOS. Every cell in its own `dev/jigc-rig` root. Read-only: no edit, no commit.
The area's own suites pass at HEAD: `g_migrate` filter (`setup_install_pathspec_guard::`, `replacing_writers_never_follow::`, `setup_failed_first_run::`) 59 passed; `g_milestone` filter (`uninstall_workbench_subject::`) 23 passed.

**Verdict: red.** The pass's new status-blind ask (`ef9456b5`) introduces a regression: a clean, committed, unflagged install path refuses `jigc setup` with a false cause and a route that cannot clear it. Three exit-0 losses that predate the pass stay reachable at the two doors, one of them inside the class `(R1, F1)` says is closed.

## Findings

### F1 — HIGH — regression + dead-end route: a clean tracked install path whose committed blob holds CRLF refuses `jigc setup` as "flagged"
- **Where:** `crates/cli/src/setup.rs:3005-3024` (`unseen_by_status`: `git hash-object -- <paths>` compared with the index blob), claim at `:2933-2938` ("through the path's own clean filters, so a line-ending conversion is not a difference"); wording `crates/engine/src/finalize.rs:1635-1639` and route `:1698-1716`; `design/validation.md:676`. Commit `ef9456b5`.
- **Mechanism (observed):** `git status` reads the path clean; `git hash-object -- CLAUDE.md` returns a different blob id than the index entry when the blob holds CRLF and conversion is in `auto` mode. The code reads any mismatch as "assume-unchanged or skip-worktree" — it never reads the flag.
- **Repro (driven):**
  - setup: `bare` rig; `git config core.autocrlf false`; `printf '# Team notes\r\n\r\nUse tabs.\r\n' > CLAUDE.md`; `git add CLAUDE.md && git commit -m notes`; `git config core.autocrlf input`. `git status --porcelain` is empty. Index blob `887784cb…`, `git hash-object -- CLAUDE.md` `fa1fe3ef…`.
  - `jigc setup` → **exit 1**, `blocking · setup.dirty-install-path`, "`CLAUDE.md`: tracked, with the change hidden by its index entry (assume-unchanged or skip-worktree)".
  - route as printed: `git -C <repo> update-index --no-assume-unchanged -- CLAUDE.md` → exit 0; `… --no-skip-worktree -- CLAUDE.md` → exit 0; `git status --porcelain` → empty; `git commit -am keep` → exit 1 "nothing to commit, working tree clean"; `git stash` → "No local changes to save"; `jigc setup` → **exit 1**, the identical refusal.
  - control, rc.24, same repository: `jigc setup` → exit 0, install commit made.
  - control, this tree, no conversion configured: exit 0.
  - second trigger: no `core.autocrlf`, a committed `.gitattributes` holding `* text=auto` → exit 1, both `CLAUDE.md` and `.claude/settings.json` listed.
  - upgrade: rc.24 installs into the CRLF repository (exit 0, re-run exit 0), then this tree's `jigc setup` → exit 1 over `CLAUDE.md` (the install appended to it and kept its CRLF lines).
  - `jigc setup --force` → exit 0, and its advisory says `CLAUDE.md` "carried changes in no commit" (false); the next plain `jigc setup` → exit 1 again; after a second `--force`, exit 1 again. The refusal is permanent.
- **Class:** one comparison site (`setup.rs:3005-3024`), one gating caller (`InstallSubject::probe`, `setup.rs:2566`), asked of every present tracked install member (the table of 10 at `setup.rs:1949-2012`; 2 driven). Triggers: 2 driven (`core.autocrlf=input`, `* text=auto`); 1 read-only sibling at `setup.rs:3023` (a `hash-object` that fails flags the whole tracked set, same route). The trigger set is **instance, unbounded** — I did not enumerate git's conversion modes or filters.
- **Test masking:** cells 33 and 34 (`crates/cli/tests/setup_install_pathspec_guard.rs:2284`, `:2408`) plant LF-only content; no test or doc in the area names a line-ending case (grep for `autocrlf|crlf|text=auto|gitattributes` over the three suites, `setup.rs` and the two design docs: 0 hits).

### F2 — HIGH — exit-0 loss, unchanged from rc.24 and undeclared: when `git status` fails, `jigc setup` installs blind
- **Where:** `crates/cli/src/setup.rs:2573-2581` (`probe` → `InstallSubject::Unknown`), `:1524` (`InstallSubject::Unknown => {}` at the gate), `:3327-3329` (commit skipped). `DECISIONS.md:23` states the fix "*Restores:* `jigc setup` never replaces bytes no commit holds without refusing first". No row in `implementation/decisions-pending.md`; `design/validation.md:676` does not state the arm.
- **Repro (driven):**
  - `fresh` rig; add a submodule, commit, then move its gitdir out of `.git/modules/`; `git status` → 128, `git rev-parse --is-inside-work-tree` → 0. Append a marked line to the tracked `.jigc/AGENT.md`, uncommitted.
  - `jigc setup` → **exit 0**, "jigc setup — adapter installed"; the mark is gone from disk and `git log --all -S <mark>` finds it in no object. Same on rc.24.
  - same with an unreadable `.git/index` instead of the submodule: exit 0, mark gone, both binaries.
  - `jigc setup --force` in the submodule cell → exit 0, `findings: []` — the consent names nothing.
  - no git on PATH: `jigc setup` → exit 1 `setup.install-hook` ("cannot install the `pre-commit` hook into the repo's hooks dir: No such file or directory", route "ensure the repo's git hooks directory is writable"); the mark is gone. Both binaries.
- **Class:** 3 `InstallSubject::Unknown` sites by grep (`setup.rs:1524`, `:2580`, `:3327`). Triggers: 3 driven; **instance, unbounded**. No test in the three setup suites names the arm (grep `Unknown`: 0 hits). The fixer of `ef9456b5` reported only the exit-1 shape, as a human decision; that deferral is in no tracked file (F7).

### F3 — HIGH — exit-0 loss, pre-existing: the guide's ownership oracle proves the body only, so a front-matter edit is replaced by `setup` and removed by `uninstall`
- **Where:** `crates/cli/src/setup.rs:285-295` (`recorded_body_digest`), `:273-277`, member declared `ExemptWhenJigcOwned` + `OwnContent::GuideDigest` at `:2001-2006`, oracle at `:2155-2159`, teardown at `:3998`. The installed file itself says "Edit it and it becomes yours: every later `setup` leaves it byte-identical" (`:114-120`).
- **Repro (driven):**
  - `fresh` rig; in `.claude/skills/jigc/SKILL.md` replace the `description:` line and add an `allowed-tools:` line, body untouched; `git status` shows ` M`.
  - `jigc setup --format json` → **exit 0**, `findings: []`; both marks gone from disk, in no object, `git status` empty. Same on rc.24.
  - same edit, `jigc uninstall` → **exit 0**, "removed jigc guide artifact", file gone, mark in no object.
  - control, body edit: `setup` exit 0 with `adapter-guide.user-modified`, bytes kept; `uninstall` leaves the file with the same advisory.
- **Class:** 4 consumers of the oracle by grep (`guide_ownership(` at `setup.rs:1435`, `setup.rs:3998`, `upgrade.rs:136`; `GuideDigest` at `setup.rs:2155`); the 2 that destroy are both driven. The blind region is the whole front matter.

### F4 — HIGH (exit-0 loss; pre-existing, outside the class `(R9, F5)` names) — `jigc uninstall` deletes a standalone jigc `pre-commit` hook the adopter extended
- **Where:** `crates/cli/src/setup.rs:997-1002` (`classify_precommit_for_teardown`: a prefix match decides *wholly ours*), `:1032`.
- **Repro (driven):** `fresh` rig; insert `npm run lint || exit 1   # <mark>` before the final `exit 0` of `.git/hooks/pre-commit`. `jigc uninstall` → **exit 0**, "- removed pre-commit hook", file gone. Same on rc.24. Control: the same edit followed by `jigc setup` keeps the line (the install side treats it as foreign and wraps it).
- **Class:** instance, unbounded. Of the teardown's host-file classifiers I read `unwire_reference` (exact match, held) and did not examine the three settings-file removals.

### F5 — MEDIUM — the repository `ef9456b5` makes first-class is certified at `setup` only
- **Where:** `crates/cli/src/task.rs:7063-7069` (`existing_pathspecs` filters on existence, not on ignore), `:6494-6495`; `crates/cli/src/setup.rs:4577-4611`; cells 31/32 (`setup_install_pathspec_guard.rs:2021`, `:2111`); `design/validation.md:676` ("keeps re-running `setup` at exit 0").
- **Repro (driven):** `bare` rig, `.gitignore` holding one of four patterns, `jigc setup` twice (exit 0, 0), open a task, stage code, `jigc task finalize`:
  - `.jigc/AGENT.md` → exit 0. `.jigc/version` → **exit 3** `finalize.stage-failed`. `.jigc/config/` → exit 3. `.jigc/` → exit 3 (route: "once the embedded git failure is resolved (e.g. remove a stale `.git/index.lock`)"). Same on rc.24.
  - `.jigc/` ignored, `jigc uninstall` → exit 1 `uninstall.untracked-workbench-file` over jigc's own five generated files. Same on rc.24.
- **Class:** 4 ignore shapes driven (3 fail), 3 doors driven; other doors **unbounded**.

### F6 — MEDIUM — a doc that is false: "whatever this door takes, it names"
- **Where:** `crates/cli/src/setup.rs:3777-3782` ("partition every byte under `.jigc/`"); `design/project-setup.md:154` (G5, (d)). Commit `12398ddc`.
- **Repro (driven):** `fresh` rig; `jigc milestone create "Ship the cache"`; `jigc uninstall` → exit 0. `.jigc/milestones/ship-the-cache/{base.json, record-commit-msg.txt, staged-snapshot.json, tasks.json}` are removed, refused over by nothing and named by nothing (the narration lists the tracked files and `state/`).
- **Class:** jigc's own registry rows in the work-unit areas — 6 milestone rows + 15 task rows, read off `crates/engine/src/state.rs:246-253` and `:186-202`; 4 driven. The completeness arm (n) reads `state/` and `index/` only.

### F7 — MEDIUM — records: the three round-3 commits are in no log, and what they left open has no trigger
- **Where:** `DECISIONS.md:5-15` ("None is built at this entry"); grep for `104a7d4b|ef9456b5|4fa1c0f5` over `DECISIONS.md`, `implementation/decisions-pending.md`, `implementation/project-history.md`: 0 hits.
- **Evidence (read only):** open items that exist only in the fixers' reports — the `Unknown` arm (F2), the index-flag class applied to merged-into members, leave-and-say-so at `task finalize`, the teardown-log bound (clear twice after a route through another verb), a linked `.claude/skills`, the tracked-path-deleted wedge. Also unrecorded: `jigc_engine::finalize::setup_dirty_install_finding` gained a fourth argument and `SetupUnseen` is a new pub struct in the published crate.
- **Class:** 6 items counted from the `left_open` sections of the three reports; not re-derived.

### F8 — LOW — the unborn refusal's commit arm silently forfeits the secrets floor
- **Where:** `crates/engine/src/finalize.rs:1772-1778`; `crates/cli/src/setup.rs:1140-1143`.
- **Repro (driven):** unborn repository, untracked `.jigc/AGENT.md` and `CLAUDE.md`; `jigc setup` → exit 1; follow the commit arm (`git add -- .jigc/AGENT.md CLAUDE.md && git commit`); `jigc setup` → exit 0 and no root `.gitignore`. Control (plain unborn install): root `.gitignore` seeded.
- **Class:** instance, unbounded.

## Held (tried to break, could not)
- On-ramp: unborn with untracked `CLAUDE.md` / `.gitignore` / `.claude/settings.json` → exit 0, mark in the first commit, re-run exit 0. Unborn with `.claude/` ignored → three runs exit 0. Born with an ignored hand-made settings file → merged, kept.
- No-identity first run on unborn (exit 1 `setup.install-commit`), identity set, re-run → exit 0, one commit, clean.
- Whole `.jigc/` ignored and never edited: three `setup` runs exit 0.
- Upgrade: all six installed rig states built by rc.24, then this tree's `setup` twice → exit 0, clean.
- Ignored hand-written `.jigc/AGENT.md` on a born HEAD: refusal, printed move, re-run exit 0, notes kept.
- Committed link at `packs.yaml`: `--force` still refuses, target intact; `git rm` + commit + re-run → exit 0, regular file.
- A linked `.claude/skills`: exit 1 on both binaries (no regression); this tree writes nothing into the shared directory.
- `git ls-files -s -z` over a path beyond a symlinked directory does not fail, so the pass adds no new route into `Unknown`.
- Finalize stamp: hand-edited prose, committed link (stamp-shaped target), absent — exit 0 each; advisory in the first two, bytes intact, stamp not staged; absent is re-created and committed.
- Uninstall: log refusal → move → exit 0 → second run no-op, `.jigc` not re-created; plants in all six prefixes at two depths named under `--force`; all six rig states (5 exit 0, 1 `staged-prose`); `CLAUDE.md` unwire is exact-match.
- Writers versus the `InstallMember` table: production writers enumerated by grep over `adapter.rs`, `setup.rs`, `gitignore.rs`; every one is a table row.

## Not examined
- The full `dev/gate` at HEAD (only the area's four suites).
- FIFO at `.jigc/version`; the `MigrationFixed` and fan-out finalize arms; `uninstall` from inside a fan-out worktree or a subdirectory.
- The flagged in-worktree hook; `(R1, F2)`.
- The three settings-file removals at `uninstall`; `jigc config set` as a replacing writer.
- Races between a door's question and its write.
- Linux.
