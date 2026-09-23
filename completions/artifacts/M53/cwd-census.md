## Census of cwd-dependence — `jigc 1.0.0-rc.18` (installed release), repo HEAD `86dbe224`

Every row marked **DRIVEN** ran on `~/.local/bin/jigc` (`jigc 1.0.0-rc.18`) on 2026-09-23 against `dev/jigc-rig` fixtures (`committed-singletons`, `bare`; roots from `mktemp -d`, nothing torn down). **READ** = source read, not driven. Nothing was edited or committed in the working repository.

**cwds used throughout:**
(a) `$REPO` — the repository root · (b) `$REPO/docs/deep` — an ordinary subdirectory · (c) `$REPO/.jigc/worktrees/<sub>` — a provisioned fan-out worktree · (d) `$RIG/feat` and `$RIG/linked` — ordinary linked worktrees outside `.jigc/`, one branch-attached, one detached.

---

## 0. The rule, and what it actually says (READ)

`design/surface-contract.md` → *The printed-path fence (law 1)*, lines 59–69:

> The rule now has one home — `crate::render::repo_relative(repo_root, path)`, **repo-relative** and `/`-separated, `.` for the root itself, and the honest absolute for a path genuinely outside the repository…

Relative to **the repository root**, never to the caller's cwd — and the doc never says so to the *reader of the output*, only to the implementer. The same section already admits the escape this census needs, as reason three an absolute **stays**:

> **pasteable shell bytes** — a `route:`/remedy span the operator pastes into **a shell of unknown cwd** (`rm <shadow>`, `git worktree remove --force <path>`).

So the contract already contains the disposition. It is **applied to exactly two remedies** (`rm <shadow>`, `git worktree remove --force <path>` — and driven, even the second one is *not* absolute today, see C1-08). `(2, F-1)` is not a new rule; it is that rule unapplied over its class.

**The mechanism, driven and read.** `crate::repo::discover_repo_root` (`crates/cli/src/repo.rs:108`) walks up to `.git`; `repo::jigc_home` layers `git rev-parse --path-format=absolute --git-common-dir` over it so a worktree finds the **main** checkout's `.jigc/`. But **six other private copies** of `discover_repo_root` exist — `ingest.rs:998`, `locate.rs:220`, `milestone.rs:7285`, `start.rs:4134`, `task.rs:7721` (+ `repo.rs`) — and all six are the bare walk-up, so from inside a worktree `repo_root` = **the worktree** while `jigc_home` = **the main checkout**. That split is the root cause of C2-02, C2-06 and C2-07 below.

**The load-bearing asymmetry (DRIVEN).** Every `Route::mechanical` argv must lead with `jigc` (`crates/cli/src/route_fence.rs:85`), and **jigc resolves a path token against the repo root** — so *all 107 production `Route::mechanical` sites are cwd-robust by construction*. The cwd-fragile class is entirely in **`Route::human` (77 sites) and bare remedy/warning prose**: a `git …` span whose operand is a `repo_relative` path, which git resolves against **cwd**. This inverts the brief's ranking premise and is the single most useful fact here — **the fix has one target, not two.**

---

## Census 1 — routes and acks embedding a path operand

Grep totals (READ): `repo_relative|Route::mechanical|Route::human` = **321** lines across 32 files. Production-only constructor counts: `Route::mechanical` **107** · `Route::human` **77** · `Route::informational` **6** · `repo_relative` **75** · `shell_token` **79**. Of these, **22** production strings interpolate a token into a backticked `git …` span; **~15 sites** are operator-facing routes/remedies (the rest are internal `bail!` diagnostics quoting a failed invocation — a declared-absolute class under law 1's *quoting an invocation* reason).

| # | producer (file:symbol) | finding / surface | route text as emitted | (a) | (b) `docs/deep` | (c) fan-out worktree | what makes it run everywhere |
|---|---|---|---|---|---|---|---|
| C1-01 | `engine/finalize.rs:847` · `Finding` — **`finalize.carried-staged`** | `jigc task finalize` | ``unstage it (`git restore --staged -- docs/deep/carried.txt`)`` | **0** | **1** ✗ `pathspec … did not match` — **DRIVEN** | **1** ✗ (worktree has its own index — **DRIVEN**) | (b): `:/`-prefix ✔ · (c): needs `git -C <abs main>` |
| C1-02 | `engine/finalize.rs:862`, `:884` — carried/stale siblings | `task finalize` | same `git restore --staged -- {token}` shape | 0 | ✗ | ✗ | READ, identical class |
| C1-03 | `cli/migrate.rs:283` — **`migrate.source-untracked`** | `jigc migrate <p> --as adr` | ``stage it with `git add -- docs/deep/untracked.md`, then re-run `jigc migrate docs/deep/untracked.md --as adr` `` | **0** | **128** ✗ `warning: could not open directory 'docs/deep/docs/deep/'` + `fatal: pathspec … did not match` — **DRIVEN** | ✗ (wrong index) | `:/` ✔ for the **git** span; the **`jigc`** span in the *same line* already runs from anywhere — **one route line, two resolution bases** |
| C1-04 | `cli/orphan.rs:1021` / `:1002` — **`schema-conformance.home-vacated`** (3 arms) | `jigc validate` | ``restore it: `git restore --source=HEAD --staged --worktree -- docs/decisions-log.md` `` · sibling arms ``git checkout -- {token}``, ``git show {sha} -- {token}`` | **0** | **1** ✗ — **DRIVEN** | ✗ | `:/` ✔ |
| C1-05 | `engine/validate.rs:2413` — **`owner-artifact.present`** (untracked cause) | task gate | ``git add <path>`` naming the recorded repo-relative path | 0 | ✗ | ✗ | READ, same class as C1-03 |
| C1-06 | `cli/repo.rs:671` `aim_at` ← `BreachSite::aim` (`repo.rs:641`) **and** `milestone::held_here` (`milestone.rs:3523`) — **`repo.operation-in-progress`**, `milestone.dirty-worktree`, `uninstall.dirty-worktree`, `milestone.leftover-holds-work` | `milestone finalize` · `milestone discard` · `task validate <sub>` · `uninstall` · `milestone provision` | ``abandon it with `git -C .jigc/worktrees/cc-area-one bisect reset` `` | **0** | **128** ✗ `fatal: cannot change to '.jigc/…': No such file or directory` — **DRIVEN** | **128** ✗ — **DRIVEN** | **`:/` does NOT help** — `git -C` takes a *directory*, not a pathspec; driven `git -C ':/…'` → **128**. Only an absolute works. **This is `(2, F-1)`, re-driven independently at both producers.** |
| C1-07 | `cli/milestone.rs:1388` — milestone-record conflict | any milestone door | ``restore `{key}` … (`git checkout -- {token}` …)`` | 0 | ✗ | ✗ | READ, `:/` ✔ |
| C1-08 | `cli/milestone.rs:6071` — worktree-removal warning | `milestone finalize`/`discard` teardown | ``run `git worktree prune`, then `git worktree remove --force {token}` `` | 0 | ✗ | ✗ | READ. **This is the exact remedy `surface-contract.md` cites as the declared-absolute example, and it ships repo-relative** (`crate::task::shell_token(path_str)`). `:/` → **128** driven. Absolute only. |
| C1-09 | `engine/file_state.rs:1345` — out-of-band rename | `validate`, task gate | ``adopt it: `jigc rename …`; or revert the move: `git mv docs/x.md docs/y.md` `` | 0 | ✗ | ✗ | **`git mv` accepts no pathspec magic** — driven: `git mv ':/a' ':/b'` → `fatal: bad source, source=docs/deep/:/a` **128**. Absolute or `-C` only. |
| C1-10 | `cli/ingest.rs:866` — `ingest.unaddressable-identity` | `jigc ingest` | ``{act} — `git mv <src> <dst>` `` | 0 | ✗ | ✗ | READ, same `git mv` constraint as C1-09 |
| C1-11 | `cli/setup.rs:3375/3376/3508` — `uninstall.foreign-bytes` / tracked-file warning | `jigc uninstall` | lists repo-relative paths, then ``(`git add <path>` is enough … `git checkout -- <path>` restores)`` | 0 | ✗ (reader substitutes the listed root-relative path) | ✗ — **DRIVEN**, see C2-07 | `:/` ✔ once the listed path is spliced in |
| C1-12 | `cli/task.rs:922` — **`task-discard.foreign-bytes`** | `jigc task discard` | ``move what you need out of `.jigc/tasks/<id>/` `` — **prose path, no command** | n/a | path does not exist rel. to (b) — **DRIVEN** | n/a | prose: needs the reader to know the base |
| C1-13 | `cli/setup.rs:3335` — `uninstall.foreign-bytes` · `cli/milestone.rs:4431` — `milestone.foreign-bytes` · `milestone.dirty-worktree` · `milestone.leftover-holds-work` | destroying doors | ``look at those paths and get out what you need`` + a root-relative listing | n/a | **DRIVEN**, text byte-identical from (a)/(b)/(c); `.jigc/worktrees/lo-one` unresolvable from (b) | same | prose |
| C1-14 | `cli/milestone.rs` — **`milestone execute` spawn template** | every fan-out | ``Spawn: `cd .jigc/worktrees/cc-area-one && jigc workflow sub-task --task cc-area-one` `` | **0** | **1** ✗ `cd: no such file or directory` — **DRIVEN** | **1** ✗ from a *sibling* worktree — **DRIVEN** | needs an absolute `cd`, or the door to take the id |
| C1-15 | `cli/config.rs:1357/1501` — `docs-root`/`placement-root` relocation ack | `jigc config set` | lists `docs/…/x.md → deepdocs/…/x.md`, *"each move is a staged `git mv` — commit it"* | 0 | **DRIVEN** — ack correct, no runnable command emitted | — | display-only; no fix owed |
| C1-16 | hook-rejection frame, `cli/task.rs:7043` | `task finalize` under a rejecting hook | ``your task's staged docs are still in `.jigc/tasks/<id>/docs/` … re-run `jigc task finalize <id>` `` | **0** | **DRIVEN rc=1 from (b)**; the **re-run argv is a jigc verb → runs from anywhere**; the `.jigc/…` path is prose | — | prose only |
| C1-17 | `engine/validate.rs` adoption routes (`unadopted-instance`, `unknown-type`, `orphaned-instance`) | `jigc validate`, `jigc ingest` | ``run `jigc ingest` … or `jigc migrate 'docs/decisions/Odd Name.md' --as adr` `` | **0** | **0 ✔ DRIVEN** — jigc resolves against the root | 0 ✔ | **already correct** — the model for the fix |

**Driven run-verbatim results, verbatim.**

```
cwd=$REPO/docs/deep
  git add -- docs/deep/untracked.md
    warning: could not open directory 'docs/deep/docs/deep/': No such file or directory
    fatal: pathspec 'docs/deep/untracked.md' did not match any files            rc=128
  git restore --staged -- docs/deep/carried.txt
    error: pathspec 'docs/deep/carried.txt' did not match any file(s) known to git  rc=1
  git restore --source=HEAD --staged --worktree -- docs/decisions-log.md        rc=1
  git -C .jigc/worktrees/cc-area-one bisect reset
    fatal: cannot change to '.jigc/worktrees/cc-area-one': No such file or directory  rc=128
  cd .jigc/worktrees/cc-area-one && jigc workflow sub-task --task cc-area-one
    cd: no such file or directory                                              rc=1
CONTROL, same five from $REPO                                                  rc=0,0,0,0,0
cwd=$REPO/.jigc/worktrees/cc-area-two
  git -C .jigc/worktrees/cc-area-one bisect reset                              rc=128
  git restore --staged -- docs/deep/carried.txt                                rc=1
  cd .jigc/worktrees/cc-area-one && …                                          rc=1
```

---

## Census 2 — verbs whose behaviour depends on cwd

All rows DRIVEN by running the leaf from (a), (b) and (c) and byte-comparing output.

| # | verb | (a) vs (b) | (a) vs (c) worktree | what differs |
|---|---|---|---|---|
| C2-01 | `jigc start` (orientation) · `doc list` · `doc list --format json` · `doc show` · `doc show --format json` · `doc schema` · `validate` · `describe` · `config list` · `config get` · `task validate` · `milestone execute` · `milestone join` · `migrate-corpus --dry-run` · `workflow --preview` | **byte-identical** | **byte-identical** | none — the read surfaces are cwd-robust, incl. from a worktree whose HEAD predates the file (`doc show milestone-record:…` served the main checkout's bytes although the file does **not exist** in the worktree) |
| C2-02 | **`jigc task diff <sub-id>`** (text **and** `--format json`) | identical | **DIFFERS** | **It diffs the checkout you are standing in.** From (a)/(b) it reported the *milestone-record commits* as the sub-task's work and **omitted** the sub-task's actually-staged `wt-file.txt`; from (c) it reported `wt-file.txt` and nothing else. An orchestrator at the root asking what a sub-task changed gets a **false picture**, at exit 0, on a contract-pinned verb |
| C2-03 | **`jigc migrate <PATH>`** | **PATH resolves against the REPO ROOT.** `jigc migrate note.md` from `docs/deep` (where `./note.md` exists) → **rc=1** `could not read the foreign adr source at 'note.md' / route: check the path` — a route that cannot succeed, since the path *is* right. `jigc migrate docs/deep/note.md` → rc=0 | — | Worse: `jigc migrate ../../rootnote.md` from `docs/deep` → **rc=1 `migrate.source-untrackable` — "`../../rootnote.md` resolves outside the repository"**. It does not; `$REPO/docs/deep/../../rootnote.md` is `$REPO/rootnote.md`. The message is **false**, produced by resolving a cwd-relative token against the root |
| C2-04 | **`doc set-slot --from-file <PATH>`**, `doc author --from-file`, `config fill --from-file`, `config insert-step/replace-step <file>` | **PATH resolves against the CWD** — the **opposite** of C2-03. `--from-file ./pay.txt` from `docs/deep` → rc=0; `--from-file docs/deep/pay.txt` from `docs/deep` → **rc=1 `No such file or directory`** | — | Two path-bearing arguments on one binary with **opposite** bases, and no surface states either. `PATH_ARG_OCCURRENCES` (`cli.rs:3106`) classifies all 8 occurrences for *safety* and says nothing about *resolution base* |
| C2-05 | `config set docs-root <path>` / `placement-root` | root-relative ✔ (`deepdocs/` not `docs/deep/deepdocs/`), relocation ack correct | — | correct + documented in `knobs.yaml` |
| C2-06 | **`jigc milestone finalize <id>`** | rc=0 ✔ (finalized from `docs/deep`, 2 files committed) | **rc=1, hard fail, every time** — **DRIVEN on two independent rigs, base-matching and base-mismatched, and from a branch-attached ordinary linked worktree** | Composes a **mixed** pathspec list and runs it with `current_dir` = the worktree: ``git add -- .jigc/config .jigc/.gitignore /private/var/…/repo/docs/milestone-records/loss-probe.md` failed: fatal: '…/loss-probe.md' is outside repository at '…/.jigc/worktrees/lp-one'``. **The milestone boundary is unreachable from the one cwd jigc's own spawn template puts agents in.** It is a **raw git error, not a finding** — no code, no route, no `at:`. Rollback is clean (**no loss**: authored commit-doc md5 `2b4deb2d…` identical before and after, prose intact, HEAD unmoved) |
| C2-07 | **`jigc uninstall`** from (c) | — | **rc=0**, reports *"jigc uninstall — repo-local install removed"* listing 6 removals | **DRIVEN clean on a fresh `bare` rig**: it removed the **shared repository's** `.git/hooks/pre-commit` (gone from the main checkout) and the worktree's own checked-out `.jigc/`+`.claude/`, while the **main checkout's install stands**: `.jigc/AGENT.md`, `.jigc/config`, `.jigc/milestones` and `.claude/skills/jigc/SKILL.md` all present, `CLAUDE.md` still carries `@.jigc/AGENT.md`. A half-uninstall that takes a repository-wide artifact and **claims a completion it did not perform**, at exit 0. (= the rc.18 `(3, F-3)` row, re-driven independently) |
| C2-08 | **`jigc start --task <sub-id>`** and **`jigc workflow sub-task --task <id>`** | **rc=1 from (a) AND (b)** — *"a sub-task's work happens in its own worktree at `.jigc/worktrees/lp-one` … run `jigc milestone provision loss-probe` … then re-run this from that worktree"* (the worktree **already existed**; the route's first clause is a no-op) | **rc=0 ✔ — only from (c)** | The product has one verb that **requires** a non-root cwd (C2-08) and one that **forbids** it (C2-06), at the same milestone, and no surface states either requirement |
| C2-09 | `jigc task finalize <plain id>` from an **ordinary branch-attached linked worktree** (d) | — | **rc=0, commits onto that worktree's branch** | `feat HEAD: ebf7231 chore: probe the branch target` / `main HEAD: unmoved`. The `.jigc/` workbench is **shared** (the task was minted and orientation read from the main checkout's store) but the commit lands where you stand. Undeclared cwd-determined commit targeting |
| C2-10 | `task validate` / `milestone finalize` from a **detached** linked worktree (d) | — | **rc=1 `repo.head-detached`** | posture is evaluated against the checkout you stand in, not the milestone's |
| C2-11 | **the base-mismatch gate** | from (a)/(b): **rc=3 `finalize.base-mismatch`** | from (c): **does not fire** — the run passed every gate and reached the staging step | The gate compares the milestone pin to **the standing checkout's HEAD**, and a fan-out worktree is pinned *to the base* by construction. Only C2-06's git fault stopped it — an accident, not a guard. (No loss observed; recorded as a gate whose subject is cwd-derived) |
| C2-12 | `jigc setup` from (b) | **rc=0 ✔**, installs at the root, all display paths root-relative | — | correct |
| C2-13 | `jigc ingest` | identical from (a)/(b)/(c) | identical | prints `unmanaged ./ — 4 file(s)`; `./` means **the repo root**, which from (b) reads as "the directory I am in". Mild law-1 ambiguity |
| C2-14 | `jigc unmanage <path>` | **rc=0 ✔ from (b)** with a root-relative token | ✔ | correct — `PATH_ARG_OCCURRENCES` calls it a lookup key, and it behaves as one |

---

## Census 3 — the composed surfaces an agent reads and follows

| # | surface | cwd-dependent content? | DRIVEN |
|---|---|---|---|
| C3-01 | **`jigc milestone execute`'s spawn template** | **YES — the only `cd` jigc emits.** ``Spawn: `cd .jigc/worktrees/<id> && jigc workflow sub-task --task <id>` ``, byte-identical from all three cwds. Fails from (b) and from a sibling worktree. This is the highest-leverage line in the product: the orchestrator is *told* to copy-run it, and it is the entry to the only composed authoring path that **requires** the worktree cwd (C2-08) | ✔ |
| C3-02 | `jigc start "<intent>"` / `jigc workflow --preview` composed step text (`single-task`, `sub-task`, `quick-fix`) | **NO** — addresses are identities (`adr:<slug>#context`), never paths; the only path-like tokens are display homes (*"the managed singleton at `CHANGELOG.md`"*). Byte-identical across (a)/(b)/(c) | ✔ |
| C3-03 | installed **`.jigc/AGENT.md`** (preloaded via `CLAUDE.md`'s `@.jigc/AGENT.md`) | **Silent.** It tells the agent *"the files are storage… Everything else — source, tests, any file not in that set — you read freely"* and never states that the paths `jigc doc list` prints, and every `at:` locus, are **repository-root-relative**, nor where jigc may be run from. An agent reading `docs/spec/x.md` from a subdirectory or a worktree resolves nothing | ✔ (read of the installed artifact) |
| C3-04 | installed **`.claude/skills/jigc/SKILL.md`** | one line only — *"a path from the repository root, optionally `#` …"* (SKILL.md:182), about **address grammar**, not about output paths or cwd | ✔ |
| C3-05 | `QUICKSTART.md` / `MIGRATING.md` | one matching line, the same address-grammar sentence (QUICKSTART.md:171). No `cd`, no cwd statement | ✔ |
| C3-06 | pack step bodies (`crates/cli/pack/**`, `packs/methodology/**`) | grep for `cd `/root assumptions: **2 hits**, both `knobs.yaml` comments about `placement-root` semantics. No step tells an agent to `cd` | ✔ (grep) |

---

## Ranked — by how often an agent hits it

Ranking is by **reach × copy-runnability**, corrected for the fact established in §0: *mechanical routes are already safe; the fragile set is `Route::human` + remedy prose*.

| rank | inconvenience | why it ranks here |
|---|---|---|
| **1** | **`finalize.carried-staged` → `git restore --staged -- <root-rel>`** (C1-01) | the trial-ranked **#1 v1 gate**; fires on the ordinary finalize path; the route is the *only* thing the agent is given, and it fails from both (b) and (c) — and from (c) it is **unfixable by any relative spelling**, because the worktree has its own index |
| **2** | **`jigc milestone finalize` is unreachable from any worktree** (C2-06) | the fan-out's one commit boundary, guaranteed to fail from the cwd jigc's own spawn line creates, with a **raw git message carrying no code, no route and no `at:`** — every fan-out that finalizes without an explicit `cd` back hits it |
| **3** | **`Spawn: cd .jigc/worktrees/<id> && …`** (C1-14/C3-01) | every fan-out, copy-run by the orchestrator, and the thing that *puts* the agent in the cwd that then breaks ranks 1, 2 and 6 |
| **4** | **`migrate.source-untracked` → `git add -- <root-rel>`** (C1-03) | every migration of an unstaged source — the adoption trials' bread and butter; and its own line mixes a **root-based `jigc` span** with a **cwd-based `git` span**, so half the route works and half does not |
| **5** | **`migrate <PATH>` root-based vs `--from-file <PATH>` cwd-based** (C2-03/C2-04) | two path arguments on one binary with **opposite** bases, neither stated; the `../../rootnote.md` case produces a **false** refusal (*"resolves outside the repository"* about a file inside it) |
| **6** | **`jigc task diff <sub-id>` diffs the standing checkout** (C2-02) | the natural orchestrator question, answered wrong at exit 0 on a `--format json`-pinned verb |
| **7** | **`(2, F-1)` — `git -C <root-rel>`** (C1-06) | five findings across five doors; **lower frequency** (needs an in-progress git operation or dirty worktree) but **zero** relative spelling can fix it, and it is the row that opened this census |
| **8** | **`jigc uninstall` from a worktree** (C2-07) | rare, but it takes a repository-wide artifact, leaves the install standing, and **says it removed it**, at exit 0 |
| **9** | file-state / home-vacated / record-conflict restore routes (C1-04, C1-07, C1-09) | fire on out-of-band edits; each fails from (b); `git mv` (C1-09/C1-10) is the one shape no pathspec fix reaches |
| **10** | prose-path routes: `task-discard.foreign-bytes`, `milestone.dirty-worktree`, `leftover-holds-work`, `uninstall.foreign-bytes`, hook-rejection frame (C1-12/13/16) | the reader must know the base to `ls` or `mv`; nothing tells them |
| **11** | `AGENT.md` never states the base (C3-03) | the cheapest single fix in the whole census, and it is upstream of ranks 5, 6 and 10 |
| **12** | `ingest`'s `unmanaged ./` (C2-13) | cosmetic ambiguity |

---

## What git itself supports — the fix-shape matrix (all DRIVEN)

| command | `:/`-prefixed pathspec from (b) | `:/` from (c) targeting the **main** checkout | `git -C "$(git rev-parse --show-toplevel)"` | `git -C <absolute>` |
|---|---|---|---|---|
| `git add -- <p>` | **rc=0 ✔** | **rc=1 ✗** (`:/` = the *worktree's* top) | ✔ from (b); ✗ from (c) (wrong toplevel) | ✔ |
| `git rm --cached -- <p>` | **rc=0 ✔** | ✗ | ✔/✗ | ✔ |
| `git restore --staged -- <p>` | **rc=0 ✔** | **rc=1 ✗** | ✔/✗ | **✔ rc=0 driven** |
| `git checkout -- <p>` | **rc=0 ✔** | ✗ | ✔/✗ | ✔ |
| `git diff -- <p>` | **rc=0 ✔** | ✗ | ✔/✗ | ✔ |
| `git mv <a> <b>` | **rc=128 ✗** — `fatal: bad source, source=docs/deep/:/…` (git mv takes **paths**, not pathspecs) | ✗ | ✔/✗ | ✔ |
| `git -C <dir>` | **rc=128 ✗** — `-C` is a directory, not a pathspec | ✗ | ✔/✗ | ✔ |
| `git worktree remove --force <p>` | **rc=128 ✗** | ✗ | ✔/✗ | ✔ |

And the one idiom that reaches **jigc's own home** from *every* cwd including a worktree (DRIVEN — identical `/…/repo/.git` from (b) and (c)):
`dirname "$(git rev-parse --path-format=absolute --git-common-dir)"` — which is exactly what `repo::git_common_dir_parent` already computes internally.

**The trade, stated.** Root-relative is law 1's **display** convention and it earns three things the census confirms: short lines, a locus a driver can key `(code, target)` on, and **portability across the two checkouts a fan-out is made of** (M50's stated reason — an absolute `at:` names a machine, not a repository). A bare absolute in the *route* re-opens the key leak the rc.18 review's MEDIUM 1 closed. So the fix must keep the **message, the `at:` locus and the finding key** repo-relative and change only the **command span** — which is precisely the disposition `surface-contract.md` already writes down and applies to two remedies.

---

## Candidate fix shapes

**Golden blast radius, measured (DRIVEN grep at HEAD):** **0 of 634 goldens** carry a path-bearing git route. The 156 golden files matching `git add` carry only the **bare** words in pack step prose (`git add`-staged, 180 occurrences, none with an operand); `git -C`, `git restore --staged`, `git checkout --`, `git mv`, `git worktree remove`, `move what you need` and `cd .jigc` appear in **0** goldens; `Spawn: \`cd ` in **0** (the ids are per-milestone, so spawn lines were never goldened). The real radius is **34 `crates/cli/tests/*.rs` suites + 3 `crates/engine/src/*.rs` unit modules** asserting these spans — headed by `repo_relative_paths.rs`, whose **disposition table** is the right home for the new verdict.

### Shape A — `:/`-prefix the pathspec, absolute the `-C` (two rules, ~13 producers)
Add one helper beside `engine::finding::shell_token` — call it the *runnable* spelling — that emits `:/<repo-relative>` for a **pathspec** operand and the **absolute** for a `-C`/`git mv`/`git worktree remove` operand, while every other render of the same path stays `repo_relative`.
- **Covers:** C1-01(b), C1-03, C1-04, C1-05, C1-07, C1-11 via `:/`; C1-06, C1-08, C1-09, C1-10 via absolute.
- **Does not cover:** C1-01(c) and C1-03(c) — a worktree's `:/` is its own top, so a route aimed at the *main* index still needs `-C <abs>`.
- **Collision to settle first:** M51 Increment 1 **refuses** git pathspec magic on *input* (`trackable::resolve_source_token` rejects a leading `:`), and C1-03's route line contains **both** a git span and a `jigc migrate <path>` span. The two spans must take **different** spellings in one sentence — which violates the contract's own *"one screen naming one path two ways is the break"* clause. **This is the shape's real cost, and it is a design call, not a swap.**
- Blast radius: ~13 producers, **0 goldens**, ~20 test files.

### Shape B — one absolute `git -C <jigc-home>` prefix on every emitted git span (one rule, ~15 producers)
Render every operator-facing git command as `` `git -C <absolute jigc_home> <subcommand> -- <repo-relative path>` ``, leaving message, `at:` and key untouched.
- **Covers every row in Census 1 including (c)** — the single shape that fixes the fan-out cells, because it names the index explicitly instead of inheriting it.
- Uniform: one rule, one home, one fence (extend `route_fence.rs`'s span fence to assert *every* backticked `git` span leads with `-C`), and it keys cleanly to the contract's existing **pasteable shell bytes** disposition.
- Cost: long lines; the absolute is the *machine's* path, so a route is no longer portable between two operators (the message and locus still are); `repo_relative_paths.rs`'s driven door table scans stdout for the fixture root prefix and would need a declared carve-out for route spans. It also reads as heavier than the two-token `:/`.
- Blast radius: ~15 producers, **0 goldens**, ~20 test files + the disposition table.

### Shape C — stop emitting git; give the act a `jigc` verb (fewest producers, largest surface change)
The rows that already work (C1-17, C1-12's re-run, C1-16's re-run, the `provision --force` route) all work for one reason: **a `jigc` span resolves its token against the root**. Route the highest-frequency cells at jigc verbs instead — a carryover unstage and a source-stage are the two the census shows agents hit constantly.
- **Covers:** ranks 1 and 4 completely, for every cwd, as `Route::mechanical` (which the argv fence already proves runnable).
- **Does not cover:** C1-06/08/09/10 — `git -C … bisect reset`, `git worktree remove`, `git mv` are genuinely git's, and jigc should not grow verbs for them; they still need Shape A's or B's absolute.
- Cost: **new capability**, which the standing exit rule forbids before 1.0.0.

### The shapes are not exclusive, and the census points at one combination
**C3-01/C2-06/C2-08 are one defect, not three**, and none of A/B/C touches it: *a milestone's spawn line creates the cwd from which the milestone's own boundary cannot be run.* The cheapest correct move there is to make the emitted `cd` absolute (same rule as Shape B) **and** make `milestone finalize` resolve its subject through `jigc_home` — which `repo::jigc_home` already computes — rather than through `milestone.rs:7285`'s bare walk-up. That also closes C2-02 and C2-11, which are the same six-copies-of-`discover_repo_root` root cause.

**Smallest honest first move, if only one thing ships:** `AGENT.md` gains one sentence — *every path jigc prints is relative to the repository root; jigc itself may be run from anywhere in the repository* — which is the only fix that reaches ranks 5, 6, 10 and 11 at once and costs one string. It does **not** make rank 1, 2, 3, 4 or 7 run.

---

### Claims I did **not** establish
- **No data loss was observed anywhere in this census.** C2-06's rollback was verified byte-exact (authored commit-doc md5 `2b4deb2dad8503a6435b6001103ebd2c` before and after the failed worktree finalize, prose intact, HEAD unmoved, worktree staging intact). C2-07 destroys only checked-out tracked copies plus the shared hook — recoverable from the index, but **the hook is not** and the report is false.
- C2-11 (the base-mismatch gate not firing from a worktree) was **observed**, not exploited — C2-06's git fault always intervened first on this binary. Whether the gate is *reachably* evadable is unproven.
- C1-02, C1-05, C1-07, C1-08, C1-09, C1-10 are **READ** (source-identical shape to driven siblings — all take `crate::task::shell_token(<repo_relative path>)`), not individually driven.
- Fix-shape producer counts are derived from the production-only grep (`Route::human` 77, 22 interpolated git spans, ~15 operator-facing) — a count, not an enumeration fence; no mechanical checker exists for this class today.

This ledger is verified at HEAD `86dbe224` on `1.0.0-rc.18` — **a map, not gospel**. Nothing was settled; that is the human's gate.
