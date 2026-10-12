# The byte table — the four acceptance rows, written from the design docs

*Written 2026-10-12 on the branch `fix/rc24-route-catalog`, as task `B0` of [the pilot's plan](byte-table-plan.md) (§2, *The row and its allowance*; §8). A record: it runs nothing and decides nothing. No binary was run to learn what a command writes, and no list of files was taken from the product's source; every line below was read off a design doc, at the heading named.*

*Changed 2026-10-13, with the task `B2c` that lifts the rows into data
([install.rs](../../../../../crates/cli/tests/byte_table/install.rs), [task.rs](../../../../../crates/cli/tests/byte_table/task.rs), [milestone.rs](../../../../../crates/cli/tests/byte_table/milestone.rs)): the first entry of §6 is rewritten as the human ruled it on 2026-10-12, three lines stand in every row for it (§4a), and `M5` gains the sentence that names its place. No other line moved.*

**What a row is.** The byte table compares a repository's bytes before and after one writing command. A leaf's *row* says what that command may change — its *allowance* — line by line, and each line rests on one sentence of the design doc that owns the command. This file holds the rows of the four commands the plan's acceptance needs: `jigc setup`, `jigc uninstall`, `jigc task finalize` and `jigc milestone finalize`.

**How a line is written.** Its number; the place it speaks of; what may happen there; when, where the doc gives a condition; and whether it still holds when the command refuses. Under it, set off as a quotation, **the sentence the line rests on — a run of the doc's own bytes, markup included, never a paraphrase** — and under that the doc and the heading the sentence stands under. A line that needs two sentences quotes both, each with its own source. The quotation is what `B1`'s fence holds against the doc: a doc that moves reddens the row.

**How a silence is written.** Where no sentence was found for something a command plainly has to do, the row says *the doc is silent*, no line is made up for it, and it is a numbered entry of §6 — the list for the human, in the form of [the route catalog's list](route-catalog-doubts.md): the words, the readings, what each makes of which cells.

**What was read.** `design/project-setup.md` (whole), `design/assistant-adapter.md` (*Generated, minimal, regenerated* to *2. Make `jigc` frictionless*), `design/finalize.md` (*What `finalize` is*; phases 2 to 7; *Rollback discipline*, its table's rows 4 to 6; *`fan-out` finalize*; *The doc-only arm*; *What `finalize` does NOT do*), `design/storage.md` (*Source of truth*; *Repository layout & state location*; *The per-task working area (staging)*; *Derived caches*; *Concurrent writers*, its last part), `design/team-ready-state.md` (*The commit model*; *The working area's two populations*; *`jigc milestone discard <id>`*), `design/surface-contract.md` (the carryover gate's paragraph on `setup`), `design/measurement.md` (*The in-repo invocation log*, its first paragraph). **Not read:** `design/finalize.md` → *1. Preflight*, *The amend arm* and *Commit-doc rendering*; `design/reconciliation.md`; `design/validation.md`; `design/write-commands.md` — §6 says where a row may be short for it.

---

## 1. `jigc setup`

Owner docs: `design/assistant-adapter.md`, `design/project-setup.md`.

**S1 — `.jigc/AGENT.md`: written whole.** When: always. On a refusal: no (S13).

> a stable one-line pointer (Claude Code: an `@.jigc/AGENT.md` import) to a **managed bootstrap file** the adapter writes under `.jigc/` and **regenerates on every `setup`**

— `design/assistant-adapter.md` → *1. Inject the bootstrap — hook (primary) + line (floor)*

**S2 — `CLAUDE.md`: merged into — the one import line is added, and every line that stood there stays.** When: always. On a refusal: no (S13).

> The always-loaded file is touched only to ensure the pointer is present (idempotent on the exact reference line); the content lives in the managed file.

— `design/assistant-adapter.md` → *1. Inject the bootstrap — hook (primary) + line (floor)*

**S3 — `.claude/settings.json`: merged into — the permit, the `deny` floor and the `SessionStart` hook are added where absent, and every entry that stood there stays.** When: always. On a refusal: no (S13).

> a pre-existing non-trivial `CLAUDE.md` / `.claude/settings.json` is **merged, never clobbered** (structure-aware)

— `design/project-setup.md` → *Idempotency & irreversibility — the acceptance obligation*

**S4 — `.jigc/config/`: the project layer is made where absent; its files `.jigc/config/.gitkeep` and `.jigc/config/packs.yaml` are written whole.** When: always. On a refusal: no (S13).

> `jigc setup` also **initializes the in-repo project layer** (`.jigc/config/`) when absent

— `design/assistant-adapter.md` → *Generated, minimal, regenerated*

> a file at a path it **replaces** (`.jigc/AGENT.md`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml`, the settings record `.jigc/settings-entries.json`, a `.jigc/version` that is not jigc's own one-line stamp) is refused before the first write while no commit holds its bytes, on an unborn `HEAD` as on a born one

— `design/project-setup.md` → *Idempotency & irreversibility — the acceptance obligation*

**S5 — `.jigc/version`: written whole.** When: always. On a refusal: no (S13). The sentence is S11's second, which names it a member of the install; that it is replaced and not merged is S4's second sentence.

**S6 — `.jigc/settings-entries.json`: written whole, or removed.** When: the doc's condition is *where its install commit can carry the settings file* — written there, removed elsewhere. On a refusal: no (S13).

> `setup` writes it only where its install commit can carry the settings file, and removes a record that stands where it cannot

— `design/assistant-adapter.md` → *2. Make `jigc` frictionless*

**S7 — `.jigc/.gitignore`: merged into.** When: always. On a refusal: no (S13).

> `.jigc/.gitignore` is the one merged-into member a link is never followed at — its writer amends in place

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S8 — the guide, `.claude/skills/jigc/SKILL.md`: written whole.** When: nothing stands there, or what stands there is jigc's own copy; a copy somebody edited is left as it is. On a refusal: no (S13).

> **rewritten whole on every `setup`**, the `.jigc/AGENT.md` mold

— `design/assistant-adapter.md` → *The adapter's owned artifacts (M48)*

> `.claude/skills/jigc/SKILL.md` is Claude Code's skill convention, not jigc's

— `design/assistant-adapter.md` → *The adapter's owned artifacts (M48)*

> and is left **byte-identical**, by `setup` and by `uninstall` alike

— `design/assistant-adapter.md` → *The adapter's owned artifacts (M48)*

**S9 — the root `.gitignore`: made, or merged into.** When: **the repository has no commit** — read off the state before the command. On a refusal: no (S13).

> Setup seeds the floor **iff the repo has zero commits**

— `design/project-setup.md` → *The secrets-floor `.gitignore` — fresh-repo install only (M36)*

> Absent → create; present → append-if-absent; re-run → byte-stable no-op.

— `design/project-setup.md` → *The secrets-floor `.gitignore` — fresh-repo install only (M36)*

**S10 — the `pre-commit` hook, in the hooks directory git resolves: made, or merged into — jigc's block is added and every other line stays.** When: always. On a refusal: no (S13).

> `jigc setup` installs a `pre-commit` hook (into the hooks dir git resolves — usually `.git/hooks/`, see the install discipline below)

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

> **non-destructive** (preserves/wraps any existing `pre-commit` hook)

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S11 — one commit on the checked-out branch, holding the paths of S1 to S10 that are there and that git does not ignore, and nothing else: not the index as it stood.** When: always. On a refusal: no.

> `jigc setup` lands its footprint as a dedicated `chore(jigc): install jigc workspace config` commit so the scaffolding never rides the user's first `finalize`

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

> The pathspec is **enumerated, never a blanket `git add -A`**: the always-loaded host file (`CLAUDE.md`), the settings file (`.claude/settings.json`), the managed `.jigc/AGENT.md`, `.jigc/.gitignore`, the `.jigc/version` provenance stamp, the project layer (`.jigc/config/`)

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S12 — the index, at the paths of S1 to S10: staged.** When: on a refusal that came **after** the install's first write, and then the written files of S1 to S10 stand too. On a refusal: yes, that one — see §6, entry 2, for why no cell can be held to this line yet.

> records the install footprint (M51's `record_install_footprint`) over the paths it wrote in that run, clean against `HEAD` before it and dirty at the failure, and **stages them**

— `design/project-setup.md` → *Idempotency & irreversibility — the acceptance obligation*

**S13 — a refusal before the first write changes nothing.** This is the sentence S1 to S11 take their *on a refusal: no* from.

> **And because the ask is pre-write, the refusal installs nothing**

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S14 — the consent is `--force`, and what it destroys is named.** A cell under the consent spelling is exempt from the plant's rule only where the command's output names the planted path.

> **`--force` is consent, not preservation:** it lets the install run, so at a whole-rewrite path the adopter's bytes are replaced on the way to the commit — which the install then **says**, as an advisory `setup.forced-install-path` naming every path the consent was spent on

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S15 — a condition with two sides, which the plan's acceptance runs on both: `HEAD` born or unborn.** No line of its own: it says which of S2, S3, S7, S9 and S10 a file somebody left at the path rides the first commit under, and that S1, S4, S5 and S6 refuse over one on either side.

> on an unborn `HEAD` an *untracked* file at a path the install **merges into** rides the repository's first commit instead of refusing — the on-ramp — while a path it **replaces** refuses there too

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**What the docs do not say of `setup`:** where the record of S12 is kept (§6, entry 3); where *the spawn launch-template* of the boundary table is written (§6, entry 4); whether a `setup` that exits 0 writes under `.jigc/index/` or `.jigc/state/` (§6, entry 1 — which the ruling of 2026-10-12 leaves open: it speaks of a refusal). **Declared, and not a line here:** a `CLAUDE.md` or a settings file that is a link is followed to a regular file inside the checkout, which then becomes a member of the install (*`jigc setup` has assistant-*neutral* install responsibilities too*, the paragraph on a file the install replaces) — the row gains that line with its sentence when a cell plants a link there, which no cell of the first cut does.

---

## 2. `jigc uninstall`

Owner doc: `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*; every sentence below stands under that heading unless another is named.

**U1 — `.jigc/` and everything under it: removed.** When: none of the four refusing states stands (U7). On a refusal: no (U7).

> **`uninstall` removes exactly the enumerated repo-local setup-created set — now *complete*: `.jigc/`, the import line, all three `.claude/settings.json` writes (permit + `SessionStart` hook + `deny` floor), the `pre-commit` hook, and the adapter's owned guide artifact**

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U2 — `CLAUDE.md`: taken out of — the import line leaves, and every other line stays.** When: always. On a refusal: no (U7).

> unwire the `CLAUDE.md` import line

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U3 — `.claude/settings.json`: taken out of — the entries `setup` recorded leave, and every other entry stays.** When: git tracks the settings file and the record; elsewhere none of the three kinds leaves without the consent. On a refusal: no (U7).

> `setup` now records the entries it adds in one committed file, `.jigc/settings-entries.json`, and the teardown removes exactly those, reading the record **only where git tracks the settings file**

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U4 — the `pre-commit` hook: taken out of — the lines between jigc's two markers leave; the file is removed only where nothing else is left.** When: always. On a refusal: no (U7).

> **`uninstall` removes exactly the lines between jigc's two markers — whatever version wrote them, whatever stands around them**: what is left is restored byte for byte, and the file is removed only where nothing is left but the frame jigc itself puts around its block

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U5 — the guide, `.claude/skills/jigc/SKILL.md`: removed, with the directories jigc made for it once they are empty.** When: what stands there is jigc's own copy. On a refusal: no (U7).

> jigc's own copy is removed, and a copy the user has edited is **left in place and reported**

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

> The **directories jigc created to hold it** go with it once they empty, stopping at any directory that houses another host file

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U6 — git's registrations of worktrees: removed, at a path under `.jigc/worktrees/` and at no other.** When: always. On a refusal: no — the sentence says so itself.

> A door removes a registration from `.git/worktrees/` **only at a path it created** under `<jigc_home>/.jigc/worktrees/` — a sub-task's `<sub-task-id>`, or a boundary's own `.combine-*` — **by path**, one record at a time (`git worktree remove --force <path>`), and **a run that refuses removes none**

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

**U7 — a refusal removes nothing.** This is the sentence U1 to U5 take their *on a refusal: no* from. The four states it speaks of are a worktree-shaped path under `.jigc/worktrees/` that holds content, an open task's staged doc, a file under `.jigc/` that no index has a copy of, and a file jigc did not write in a working area, under `.jigc/displaced/`, or under a directory of `.jigc/` nobody has stated an owner for.

> All four remove nothing, name what they found, and route at the honest exits

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U8 — the consent is `--force`, and what it takes is named.**

> the one way past it is **`--force`**, the operator's explicit consent to delete, so the destruction is asked for rather than assumed

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

> so a staged file is **narrated, not refused**, and what the guards let through this door still names before it removes it

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**What the docs do not say of `uninstall`:** what it does to git's index, and whether it commits (§6, entry 5). So the row has **no line for the index and none for a commit**.

---

## 3. `jigc task finalize`

Owner doc: `design/finalize.md`.

**T1 — the canonical path of each doc staged in the task's working area: a regular file that is a copy of the staged doc — made, or written over the doc's own committed home.** When: always; a doc the task minted never lands on a file that stands at its destination. On a refusal: no (T7).

> - **Copy** (not move) the file from the working area to its canonical path.

— `design/finalize.md` → *4. Promote managed docs*

> **A promote lands a regular file at exactly its canonical path, or the transaction refuses before anything is written

— `design/finalize.md` → *4. Promote managed docs*

> A `created` doc over an occupied destination blocks with **`finalize.promote-clobber`**, keyed at the destination path, and nothing is committed; an `edited-from-base` doc re-promotes over its own home, which is the update path.

— `design/finalize.md` → *4. Promote managed docs*

**T2 — `.jigc/version`: written whole.** When: what stands there is jigc's own stamp. On a refusal: no (T7).

> the `.jigc/version` stamp, refreshed to the running build first — **where it is jigc's to write**

— `design/finalize.md` → *5. Stage*

**T3 — the index, at the paths of T1 and T2, at `.jigc/config` and `.jigc/.gitignore`, and at every recorded owner-artifact path: staged.** When: always. On a refusal: no (T7).

> This phase stages **jigc's own** managed artifacts into that existing index

— `design/finalize.md` → *5. Stage*

> on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`),

— `design/finalize.md` → *5. Stage*

**T4 — one commit on the checked-out branch, holding the index as it stood and what T3 staged — nothing that was neither staged nor a line's.** When: always. On a refusal: no (T7).

> The commit set is therefore **the git index** — the agent's staged code plus jigc's just-staged docs/config — committed whole

— `design/finalize.md` → *5. Stage*

> finalize **never** runs `git add --all`: untracked and unstaged working-tree changes stay out of the commit and are surfaced

— `design/finalize.md` → *5. Stage*

**T5 — `.jigc/index/` and `.jigc/state/`: written — the edge index's stamp, the file-state hashes.** When: the commit landed. On a refusal: no — what a refusal may do there is `C1` and `C2` (§4a; §6, entry 1, as ruled).

> - **Invalidate the edge-index stamp** ([storage.md](storage.md) → Derived caches).

— `design/finalize.md` → *7. Post-commit (best-effort)*

> - **Update `file-state` hashes** for every committed managed doc.

— `design/finalize.md` → *7. Post-commit (best-effort)*

**T6 — the task's working area, `.jigc/tasks/<task-id>/`: removed — jigc's own files; what jigc did not write there is moved aside to `.jigc/displaced/<task-id>/`, under the path it had.** When: the commit landed. On a refusal: no (T7).

> - **Keep what jigc did not write, then remove `.jigc/tasks/<task-id>/`.**

— `design/finalize.md` → *7. Post-commit (best-effort)*

> The removal's subject is jigc's own files only: the area's **complement** ([team-ready-state.md](team-ready-state.md) → The working area's two populations) is moved aside first, to `.jigc/displaced/<task-id>/` with its relative paths intact

— `design/finalize.md` → *7. Post-commit (best-effort)*

**T7 — a refusal leaves the working area and the canonical paths as they were.** This is the sentence T1 to T4 and T6 take their *on a refusal: no* from. It names two places; of the caches of T5 it says nothing.

> The transaction is **atomic up through `git commit`**: any failure before that point rolls back to "as if `finalize` was never called" — the working area is intact, canonical paths unchanged.

— `design/finalize.md` → *What `finalize` is*

**T8 — the rule for prose handed to jigc.** Not a line: it is what the plan's §2 holds a staged doc to once its working area is gone — T1's copy, in T4's commit, as a regular file. The commit doc is the exception the doc names.

> - The commit doc (`commit:<task-id>`) is **not** promoted — its sink is the git message rendered in phase 3, not a repo file

— `design/finalize.md` → *4. Promote managed docs*

**A second spelling, with its own commit line — the doc-only arm.** A task whose workflow includes `step:finalize-doc-only` commits its own paths and never the index; which task that is cannot be read off a repository's bytes, so it is a spelling of the leaf that a cell names, and its row replaces T3 and T4 by this sentence. No cell of the acceptance is one.

> commits those paths alone (`git add -- <paths>` then `git commit -F <message> -- <paths>`), never the index

— `design/finalize.md` → *The doc-only arm*

**What the docs do not say of `task finalize`:** whether a refused finalize may leave `.jigc/index/` or `.jigc/state/` changed (§6, entry 1 — ruled on 2026-10-12, and carried by `C1` and `C2`); where the rendered message's temporary file stands (*6. Commit* says *a temp file, deleted after* and names no directory — §6, entry 6). **Not read, so the row may be short:** the amend arm, and `--dry-run`.

---

## 4. `jigc milestone finalize`

Owner doc: `design/finalize.md` → *`fan-out` finalize*, with `design/team-ready-state.md` and `design/storage.md`.

**M1 — `.jigc/milestones/<id>/merged/`: cleared and written again.** When: always. On a refusal: **yes** — the sentence says so itself.

> `materialize` writes only into the gitignored `.jigc/milestones/<id>/merged/`, which it **clears and rebuilds every call**, so blocking after it still writes nothing durable

— `design/finalize.md` → *2. Validate*

**M2 — the canonical path of each doc of the merged set: a regular file that is a copy of it — as T1.** When: always. On a refusal: no (M8).

> The promote plan is one planner for both modes, and it carries the task door's clobber guard: a sub-task's `created` doc never promotes over a file already at its destination, and the refusal is taken before either mode lays down a commit

— `design/finalize.md` → *`fan-out` finalize*

**M3 — one commit on the checked-out branch, holding what each sub-task staged in its own worktree, the docs of M2, the first commit's config and the record of M5 — and nothing of the main checkout's index or files.** When: always (`finalize.fan-out.squash: true`, the default). On a refusal: no (M8).

> The CLI folds the N worktrees' staged code-sets into one combined tree **off-line** (a throwaway index), overlays the promoted docs + first-commit config, and commits that tree from a **clean dedicated worktree**

— `design/finalize.md` → *`fan-out` finalize*

> The main checkout is never mutated until that clean fast-forward, so a blocked combine or a hook rejection lands nothing and leaves unrelated main-checkout WIP intact (WIP-safe).

— `design/finalize.md` → *`fan-out` finalize*

**M4 — a second spelling: under `finalize.fan-out.squash: false`, one commit a sub-task that staged code, then the aggregate.** A cascade knob, so a spelling a cell names; no cell of the acceptance sets it.

> **`squash: false` — one commit per code-carrying sub-task in task-id order, then the parent's aggregate commit.**

— `design/finalize.md` → *`fan-out` finalize*

**M5 — the milestone's committed record, `docs/milestone-records/<slug>.md`: written — its status flipped, inside M3's commit.** When: always. On a refusal: no (M8). The second sentence is where the place comes from — added on 2026-10-13, when the line became data and had to name one; the home is under the docs root as the prepared states have it.

> the record's `status`-flip write **folds into the existing join commit** (path-added alongside the promoted docs), not a second commit

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)

— `design/storage.md` → *Source of truth*

**M6 — `.jigc/worktrees/<sub-task-id>/`, each sub-task's working area under `.jigc/tasks/`, and `.jigc/milestones/<id>/`: removed — and what jigc did not write in an area is moved aside to `.jigc/displaced/<unit-id>/`.** When: the boundary landed. On a refusal: no (M8).

> ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)

— `design/storage.md` → *Repository layout & state location*

> Both keep their complement, to `.jigc/displaced/<unit-id>/<relative>`

— `design/storage.md` → *The per-task working area (staging)*

**M7 — git's registrations of worktrees: removed, at a path under `.jigc/worktrees/` and at no other; every other registration is as it was.** When: the boundary landed. On a refusal: no. The first sentence is U6's.

> So every other registration — live, `prunable` or locked; a human's, or jigc's own under another path (a moved repository's sub-task records name the old one, and are exactly what `git worktree repair` needs) — is left byte-identical

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

**M8 — a refusal tears nothing down.** This is the sentence M2, M3, M5 and M6 take their *on a refusal: no* from, with M3's second.

> So **no aborted finalize arm tears down**: the whole working area — sub-task areas and worktrees alike — survives for the retry.

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

**M9 — git's object store: objects are added, also on a refusal.** Not a line of the allowance: the table holds the store to one rule, that no object there before is absent after, and an object more is no difference. The sentence is here because a refused boundary leaves the store larger and a reader will ask.

> it creates a temp index and runs `read-tree` / `apply --cached` / `write-tree`, writing loose objects into the ODB

— `design/finalize.md` → *2. Validate*

**M10 — `.jigc/index/` and `.jigc/state/`: written, as T5.** When: the boundary landed. On a refusal: no — what a refusal may do there is `C1` and `C2` (§4a; §6, entry 1, as ruled).

> 4. **Post-commit** runs once for the whole batch.

— `design/finalize.md` → *`fan-out` finalize*

**What the docs do not say of `milestone finalize`:** the caches on a refusal (§6, entry 1 — ruled on 2026-10-12, and carried by `C1` to `C3`). **Declared, and not a line here:** the two worktrees the boundary makes for itself and removes on every exit — the dedicated one it commits from and the detached one its `doc-code` check reads (*2. Validate*: *torn down on both exits*) — leave no difference between the two manifests where they are removed as declared; one left behind is a registration under `.jigc/worktrees/` that M7 does not cover on a refusal, and is red. The plan's §7 lists this command's commit as *not read for this plan*: it is read now, and M3 to M5 are what the docs declare it holds.

---

## 4a. The caches, on a refusal — three lines that stand in every row

Written on 2026-10-13 from the human's ruling of 2026-10-12 on §6, entry 1, which that entry now quotes. **What a cache is comes from the doc and not from the ruling's two examples**: `design/storage.md` declares three things rebuildable derived state — the edge index, the file↔state hashes, and a milestone's `{base,tasks}.json` — and names where each stands in its layout. Each line below quotes the sentence that declares the cache, and what the layout says of its directory: the comment that stands on the line of `index/`, of `state/` and of `milestones/` in that block, from its first word (the line's own columns of spaces are left out of the quotation). Nothing else under `.jigc/` is declared so: a task's working area is *a transient working copy* and one line of it is *authority* (→ *Source of truth*, the paragraph on `source-path`); `displaced/` is *the one gitignored subdir whose contents no operation can reconstruct*; a milestone's `merged/` has its own line, `M1`.

**What *rebuilt* lets a refusal do, and what it does not.** A file of a cache may be made, or written whole; a directory of one may be made. **No file of a cache may be taken away and left gone** — that is no rebuild — and nothing outside the three places is touched by these lines. They hold **when the command refused, and only then**: what a command that exits 0 may write there is its own row's to say (`T5`, `M10`; `U1` removes the workbench whole), and where a row says nothing — `setup` — the ruling does not speak (§6, entry 1, *what the ruling leaves open*).

**C1 — `.jigc/index/` and everything under it: rebuilt.** When: the command refused. On a refusal: yes.

> The edge index and the file↔state hashes are **rebuildable from the committed docs**, so they are gitignored, never committed

— `design/storage.md` → *Derived caches*

> gitignored — edge index (rebuildable cache)

— `design/storage.md` → *Repository layout & state location*

**C2 — `.jigc/state/` and everything under it: rebuilt.** When: the command refused. On a refusal: yes. The first sentence is C1's first.

> gitignored — file↔CLI-state hashes (rebuildable)

— `design/storage.md` → *Repository layout & state location*

**C3 — `.jigc/milestones/<id>/base.json` and `.jigc/milestones/<id>/tasks.json`: rebuilt.** When: the command refused. On a refusal: yes.

> **demotes** that JSON to exactly a *rebuildable cache* (re-derived from the record when absent or stale)

— `design/storage.md` → *Source of truth*

> gitignored — milestone WIP (merge staging, worktrees) + the {base,tasks}.json cache re-derived from the committed milestone-record (M39; team-ready-state.md)

— `design/storage.md` → *Repository layout & state location*

---

## 5. The acceptance's lines, read against the rows

Each line of the plan's §5, with the rule or the line that makes it red on `1.0.0-rc.24` as the plan's own runs describe that binary — read by hand, nothing run. *The plant's rule* is the plan's §2: a planted entry is the user's, and where its bytes stand nowhere after exit 0 the cell is *lost* whatever the row declares.

| the cell | what the plan says `rc.24` did | red by |
|---|---|---|
| `setup` · `bare`, `HEAD` unborn · an untracked file at `.jigc/AGENT.md`, at `.jigc/config/packs.yaml` | exit 0; both replaced, the bytes in no object | **the plant's rule** — *lost*. S1 and S4 let the command write those paths, and S4's second sentence says it refuses over a file no commit holds *on an unborn `HEAD` as on a born one*; no line is needed to make it red, and none can make it green |
| — the same, `HEAD` born | exit 1, nothing changed | not red: S13 |
| `task finalize` · `committed-singletons`, no file-state baseline · a line added by hand after the task's first write | exit 0; the line in no object | **the plant's rule** — *lost*. T1 lets the command write the doc's home; the line that stood there is the user's |
| — the same, the baseline present | a refusal | not red, if the refusal changed nothing but what `C1` and `C2` let a refusal rebuild (ruled on 2026-10-12; this cell read *what §6, entry 1 asks about* until then) |
| `milestone finalize` · `fresh` · an untracked file at the id the join numbered onto | exit 0; the bytes in no object | **the plant's rule** — *lost*; and M2's sentence, which says the refusal is taken there |
| `milestone finalize` · `fresh` · a committed doc at that id, committed before `milestone create` | exit 0; the doc's content replaced, history holds it | **M2 and M3** — *git's state moved outside its allowance*: the commit changes a path a commit already held to a doc the unit minted, which M2's sentence says never happens. The planted bytes stand in history, so it is not *lost* |
| `task finalize` · `fresh` · a link to nothing at the created doc's home | exit 0; the commit holds a link; the prose at the link's target | **T1 and T8** — *git's state moved outside its allowance*: T1's second sentence declares *a regular file at exactly its canonical path*, and the commit holds a link there; the staged doc stands in no commit. The prose stands at another path, so it is not *lost*; that path is also a write no line covers |
| `uninstall` · `fresh` · a file somebody put under `.jigc/logs/`, `.jigc/state/`, `.jigc/index/` | exit 0; gone, in no object | **the plant's rule** — *lost*, three times. U1 lets the command remove `.jigc/` and what is under it, and U7 names the state as one that refuses |
| — the same at `.jigc/`'s root | exit 1, nothing changed | not red: U7 |
| `milestone finalize` · `fresh` · the user's own linked worktree, its directory moved away | exit 0; the registration gone, a commit no longer reachable | **M7** — *git's state moved outside its allowance*: a registration at a path that is not under `.jigc/worktrees/` |

Six of the seven red lines are red by the plant's rule or by a sentence that names the state; **none needed a line widened**, and the one row a wrong allowance could have turned green — `uninstall`, whose doc declares that `.jigc/` is removed — is held by the plant's rule and not by U1.

---

## 6. Where a doc is silent, or says two things — the list for the human

One list to rule from, one entry at a time. It decides nothing. Until an entry is ruled, a cell that hangs on it follows the stricter reading — the one under which more is red — and is marked. **Entry 1 is ruled** (2026-10-12) and says how it is carried; entries 2 to 6 are open.

**1. *Either a rebuildable cache or a transient working copy* — may a command that refuses leave the caches rebuilt? Ruled on 2026-10-12: it may.**

- *The words.* `design/storage.md` → *Source of truth*: *"Every other artifact — the staging working area, the edge index, the file↔state hashes — is either a rebuildable cache or a transient working copy."* And `design/finalize.md` → *What `finalize` is*, T7's sentence: a failure before the commit *"rolls back to "as if `finalize` was never called" — the working area is intact, canonical paths unchanged"*; → *2. Validate*: *"**Blocking findings present** → abort with the finding list. No side effects."*
- *The two readings it was put with.* (a) What can be rebuilt may be rebuilt by a command that refuses: a refusal that leaves a cache changed is no finding. (b) The sentence of `storage.md` says what those files **are**, and not which command may write them when; two sentences of `finalize.md` say a refusal leaves no side effect; under it the change is *refused, and changed what a refusal may not*.
- *The ruling* — the human's, of 2026-10-12, option A of two, as the orchestrating session handed it on:

  > **A refusal may rebuild a cache: nothing is lost, and such a cell is not red for it.** The exit rule's first clause is about lost files, and a rebuilt cache loses none. The sentence *No side effects* in `design/finalize.md` is an inconsistency between two design docs, to be recorded as one and corrected — not a rule the table holds a command to.
  > **Not taken:** *a refusal leaves every byte as it was, caches included* — under which most refusing cells are red for a behaviour the storage design intends.

- *How it is carried.* By three lines that stand in every row, `C1` to `C3` (§4a): the places are the ones `design/storage.md` declares rebuildable derived state — **three, not the two the question named**: the edge index under `.jigc/index/`, the file↔state hashes under `.jigc/state/`, and a milestone's `base.json` and `tasks.json` under `.jigc/milestones/<id>/`. `M1` is as it was. `T5` and `M10` are as they were, for a command that exits 0.
- *Bounded by what rebuilt means.* A refusal may write a cache whole from what the tree holds. Under these lines it may not take a cache file away and leave it gone — **held**: the frame's word for the lines covers a file made or written and no file gone. Nor may it write a cache that the next acting command would read as other than the tree says — **not held by the frame**: a manifest shows that a cache was written, never whether what was written is true. The check that would hold it is the same command run again once the refusal's cause is gone, coming out as it does on a tree whose cache nobody touched; removing the cause is the route's act, and the byte table follows no route. One narrower form the frame could run and no cell does: the refused command run a second time over what the first left — it must refuse again, and the plant must still stand.
- *What the ruling leaves open.* **(i) Exit 0.** The ruling speaks of a refusal. Whether a command that exits 0 may write a cache its own doc names no write to — `setup` is the one of the four — is not said; the lines hold on a refusal only, so such a write is *written outside its allowance* until somebody rules. No cell of the first cut turns on it. **(ii) The file↔state hashes are called rebuildable and have no rebuild.** The sentence C1 and C2 quote says both caches are *rebuildable from the committed docs*; the next paragraph of the same section says the hashes *"carry **no stamp and no rebuild path**"*. So a refusal that writes under `.jigc/state/` is not rebuilding from the tree: it is recording a baseline, and a baseline recorded over a file somebody edited is exactly a cache that says other than the commit does. `C2` lets the write, by the doc's first sentence and the ruling's word; whether that is what was meant is the human's, and it is where the bound that is not held would bite. **(iii) A minted work unit is no cache.** A task area or a milestone area that a refusal leaves behind, and that a later command reads as authority, is covered by none of the three lines and stays red, cell by cell, until ruled (the plan's §7, question 2).
- *The inconsistency* between the two docs is recorded, and fixed by nobody here: [byte-table-findings.md](byte-table-findings.md) → `r1-finalize-design-says-no-side-effects-where-storage-design-says-caches-are-rebuilt`.

**2. `setup` refuses in two ways, and the docs tell them apart by the moment, which no manifest shows.**

- *The words.* S13: *"because the ask is pre-write, the refusal installs nothing"*. S12: a run that *"fails after its first write"* *"records the install footprint … over the paths it wrote in that run … and **stages them**"*. And of the ask repeated before the commit, in the same paragraph as S13: *"where that ask fires the install is written and staged"*. One finding code, `setup.dirty-install-path`, is the first refusal's and the third's.
- *Reading (a)* — a refusal of `setup` may leave S1 to S10 written and staged: S12 holds on every refusal. A refusal before the first write that had nevertheless written would pass.
- *Reading (b)* — a refusal of `setup` leaves nothing: S13 holds on every refusal. A failure after the first write — a hook that rejects the install commit, a hooks directory that cannot be written — is then red although S12 declares it.
- *A third way, which is a change to the plan and so the human's:* the cell says which of the two refusals it constructs, as it says whether it expects a refusal at all, and the row carries a line for each.
- *What each makes of the cells.* No cell of the first cut turns on it: the acceptance's two `setup` cells refuse before the first write, and their plant is held by the plant's rule under either reading. The cells it decides are `B5`'s — a rejecting hook at `setup`'s commit.
- *Until ruled:* (b). S12 is recorded and is no line of the row as data.

**3. Where `setup` keeps the record of a failed run.** S12's sentence names *the install footprint* and M51's function, and no path. `design/project-setup.md` → *Flow 2 hardening* lists *the install-footprint record a failed first `setup` writes* among jigc's own files under `.jigc/index/`, `.jigc/state/` and `.jigc/logs/` without saying which. No line is written for it; it is decided with entry 2, whose cells are the only ones that would see it.

**4. *The spawn launch-template + its install-time validation*.** `design/project-setup.md` → *The boundary that frames everything: command vs workflow* lists it among what `jigc setup` generates. No sentence read says where it is written — whether it is a file of its own or part of one S1 to S8 already name. No line is written for it; a file `setup` writes that no line covers is red, and would be this entry.

**5. `uninstall`, git's index, and a commit.** The doc says what leaves the working tree. It does not say whether the entries of the removed files leave the index, or whether the teardown commits anything. *Reading (a)*: the working tree only — the removals are the user's to commit, as `relocate`'s move is. *Reading (b)*: it may stage what it removed. Under (a) a staged removal is *git's state moved outside its allowance*; under (b) it is covered. Every acting `uninstall` cell turns on it. *Until ruled:* (a) — the row has no line for the index.

**6. The rendered message's temporary file.** `design/finalize.md` → *6. Commit*: *"`git commit -F <rendered-message-file>` (a temp file, deleted after)"*. A file that is made and deleted leaves no difference; one left behind is a write no line covers, wherever it stands — under the system's temporary directory, which the table reads, or under `.jigc/`. No reading is needed unless a cell finds one.

**Decided here, as a builder, and the human may overturn each:**

- **The invocation log is no line of the four rows.** `design/measurement.md` → *The in-repo invocation log*: a knob, default off, *"appends one JSONL record per `jigc` invocation to `.jigc/logs/invocations.jsonl`"*. Every command writes it while the knob is on, and no prepared state switches it on; the third of the bounds of 2026-10-06 is about exactly this file. A row gains the line, with a condition read off the project's configuration, when a cell switches the log on.
- **A directory on the way to a covered write is covered with it, and so is one that empties under a covered removal.** No doc says of `.jigc/` or `.claude/skills/jigc/` that the directory is made; U5's second sentence says it of the removal. The table reads a directory as an entry, so the rule is needed and is stated once, here.
- **The plan's first question of reading is not this record's** — *bytes no git object holds*, and how far *holds* reaches. Nothing read for these rows says which reach is meant; the plan's §7 has the three readings, and a cell follows the narrowest.

---

## 7. What a line had to be able to say

The plan leaves the allowance's vocabulary to this record: `B1` builds what these rows needed and no more. Read off §1 to §4:

| a line says | which lines needed it |
|---|---|
| a place that is **one path** of the repository | S1 to S9; U2, U3, U5; T2; M5 and C3, with one component left open for a milestone's id or its record's name |
| a place that is **everything under a directory** | S4; U1; T5, T6; M1, M6, M10; C1, C2 |
| a place that is **the `pre-commit` hook in the hooks directory git resolves** — not a fixed path | S10; U4 |
| a place that is **wherever a staged doc of the unit lands** — known by its bytes, since the path is the product's to compute | T1; M2 |
| **written whole** — made, or replaced by any bytes | S1, S4, S5, S6, S8; T2, T5; M1, M5, M10 |
| **merged into** — every line that stood there stays, in order | S2, S3, S7, S9, S10 |
| **taken out of** — every line that stands there afterwards stood there before, in order | U2, U3, U4 |
| **removed** | S6; U1, U4, U5; T6; M6 |
| **rebuilt** — a file made or written whole, a directory made; nothing taken away (added 2026-10-13, for the ruling) | C1, C2, C3 |
| **a copy of a staged doc**, as a regular file | T1; M2 |
| **moved aside** — gone from its place, and standing with the same bytes under a named directory | T6; M6 |
| **a commit** on the checked-out branch, holding **only what the row's other lines wrote** | S11 |
| **a commit** holding **the index as it stood, and what the row's other lines wrote** | T4 |
| **a commit** holding **what the sub-tasks' worktrees had staged, and what the row's other lines wrote** | M3 |
| **the index staged** at the places the row's other lines wrote | T3 (and S11, M3, through their commits) |
| **a worktree's registration removed**, at a path under a named directory | U6; M7 |
| a condition read off the state before the command: **the repository has no commit** | S9 |
| a condition read off how the command ended: **it refused** (added 2026-10-13, for the ruling) | C1, C2, C3 |
| **holds on a refusal** — yes or no, from a sentence | M1 (yes); C1, C2, C3 (yes, by the ruling); every other line (no) |
| **the consent's flag**, under which a planted path named in the output may go | S14; U8 |

Three conditions a doc gives could not be put as something a manifest shows, and are words in their lines only: *where its install commit can carry the settings file* (S6), *what stands there is jigc's own copy* (S8, U5; T2), and *git tracks the settings file and the record* (U3). Each lets the command do **less** — leave a file alone — so a line without the condition allows no write the doc forbids to anything but a file somebody edited, and such a file is a plant, which no line excuses.
