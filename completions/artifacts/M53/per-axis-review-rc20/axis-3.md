<!-- Reconciled AXIS 3 file, copied verbatim. Driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.20` (repo HEAD `51e0b8e4`), 2026-09-27. -->

<!-- M53 FOURTH PARTIAL per-axis review · AXIS 3 · destroying doors · the OPUS DRIVER.
     Every row driven on the installed /Users/maurice/.local/bin/jigc -> `jigc 1.0.0-rc.20`, 2026-09-27.
     No fix applied, no commit made, nothing written into the working repository.
     The Codex source pass for this axis was NOT read (reconciliation is a separate agent). -->

# M53 fourth partial per-axis review — AXIS 3 · destroying doors — the driver's table

**The binary, asserted first, before anything else ran.**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.20
```

**Baseline.** `completions/artifacts/M53/per-axis-review-rc19/axis-3.md` (the rc.19 reconciled table, 95
driven rows over 18 doors) + `per-axis-review-rc19/README.md` §A / Layer B, **all tiers**.

**What changed under it.** [VERDICT.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/VERDICT.md)
→ Addendum 3 (the pre-v1 usability batch's six surface rows + **F-10 `jigc task amend`**, the one new
capability) and the 2026-09-26/27 `DECISIONS.md` entries. F-10's design of record is
`completions/artifacts/M53/f10-amend-settle.md`; its driven baseline `f10-amend-baseline.md`; its
independent review `completions/artifacts/M53/audit/f10-code-review.md` (1 HIGH · 3 MEDIUM · 4 LOW, all
declared fixed — **each verified closed below, §3**).

---

## 0 · The door set and the cell axis, read from the code at HEAD `51e0b8e4`

Counted **by symbol**, not from the design docs' numbers:

| registry | file:symbol | count read |
|---|---|---|
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3430` | **6** (`PROVISION_DOOR`, `DISCARD_DOOR`, `UNINSTALL_DOOR`, `TASK_DISCARD_DOOR`, `TASK_FINALIZE_DOOR`, `FINALIZE_DOOR`) |
| `WORKTREE_DOORS` | `milestone.rs:3447` | **4** (the worktree-shaped subset) |
| `Disposition` | `milestone.rs:3196` | **3** — `Refuse{consent}` · `Narrate` (**empty by construction**) · `Displace` |
| `LeftoverShape` | `milestone.rs` | **3** — `Directory` · `File` · `Unreadable(String)` |
| `LeftoverVerdict` | `milestone.rs` | **3** — `Unverifiable` · `OwnWorktree` · `NoOwnLinkage` |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:188` | **11** — the 11th is `"jigc task finalize (amend)"` / `ERROR_AMEND_REJECTED` / `TASK_FINALIZE_AMEND_COMMITS` |
| `ERROR_CODE_REGISTRY` | `invocation_log.rs:267` | **12** — `ERROR_AMEND_REJECTED` is the new member |
| `TASK_AREA_FILES` | `crates/engine/src/state.rs:186` | **15** — `AMEND_PIN_FILE` (`amend`) is the new member |
| `MINT_DOORS` | `crates/engine/src/state.rs:1560` | **6** — `jigc task amend ["<intent>"]` → `start.rs::mint_amend_in_repo`, `Snapshot::Written` |
| `AMBUSH_CONTRACTS` | `crates/cli/src/pack.rs:990` | **6 rows / 3 dispositions** — 3 `Owed` · **2 `DeclaredWhereReachable`** (the amend pair, the new third disposition) · 1 `Exempt` |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs:6572` | **66** (64 → 66 for `Landed` / `LandedAmend`) |
| `STORE_EXIT_FLIPS` | `render.rs:1049` | **7** (unmoved) |
| `VERB_KINDS` / `BEHALF_DOORS` | `crates/cli/src/cli.rs:1877` / `:2047` | **48 / 48** (47 → 48 for `task amend`) |

**The cwd axis, carried from rc.19 and re-driven**: (a) repo root · (b) an ordinary subdirectory
(`docs/deep`) · (c) a provisioned **fan-out** worktree · (d) an ordinary **branch-attached linked**
worktree — plus a repository root containing a **space** wherever bytes reach a shell.

**Method.** Every row ran on the installed **release** binary in its own `dev/jigc-rig` throwaway repo
(two-step eval; every root from `mktemp -d`, so nothing needed teardown and the `rm -rf $V/$D` shape
appears nowhere). Exit codes read **bare** except where a row is marked *(piped)* and re-driven bare.
Every loss/survival claim carries a `command grep` **before-control** beside its after-count, because this
harness's `grep` honours `.gitignore` and the whole axis lives under a gitignored `.jigc/`.

---

## HEADLINE

> ### TIER-1 ROWS ON THIS AXIS: **0**
>
> Tier 1, quoted: *exit-0 loss or repository harm through a committing, destroying or moving door.* No
> driven cell reached it. No plant died at exit 0 anywhere but where `--force` consented and the door said
> so first, with a before-control on every claim.
>
> **The new committing arm is safe in every destroying cell it reaches.** `jigc task finalize`'s **amend**
> arm displaces a foreign plant repo-relative on the 1.0-pinned envelope (R-3), leaves the area standing
> when the park fails (R-4), leaves the plant *in the area* and HEAD byte-identical on a hook-rejected
> amend (R-13), and lands the plant properly on the re-run. Its tree is byte-identical to the superseded
> commit's in every landing cell.
>
> **`(3, F-A)` — rc.19's one tier-2/3 row — is CLOSED** (R-6): `task finalize`'s Displace surfaces from a
> branch-attached linked worktree are **0 host-absolute hits** on stdout+stderr, on both the all-move and
> the none-move cell, including on `committed.displaced[].from`/`.to` and on `finalize.foreign-bytes`'s
> `message` and `route`.
>
> **All eight F-10 review findings verified closed** (§3), MEDIUM-4 by **mutation on the release binary**.
>
> **Two new findings, both tier 3.** `(3, F-C)` — the `write.unslugable-title` route at the new
> `jigc task amend` mint door never names that door's own sha fallback, the one exit it uniquely has among
> `MINT_DOORS`' prose rows. `(3, F-D)` — the F-10 settle's Envelopes row states `jigc task amend
> --format json` *"carries the pinned sha under `text`"*; driven, **no sha appears anywhere in that
> envelope**, and the authoritative declaration says so deliberately — the settle row was never struck
> though two sibling rows in the same file carry dated correction brackets.
>
> **Five rc.19 rows STILL-OPEN, all tier 3, all expected** (triaged to 1.x): F-1, F-2, F-4, F-5, F-B.

---

## 1 · The `(door, cell)` table

Route kinds: **M** = Mechanical (`jigc`-leading argv) · **H** = Human · **I** = Informational · **—** = none.

### 1.1 · `jigc task amend` — the new **mint** door (`BEHALF_DOORS` = `Neither`, `DESTROYING_DOORS` ∉) — 16 rows

| # | cell | cwd | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 1 | happy mint, intent supplied | root | `jigc task amend "repair the summary"` | 0 | none | M | `task minted: repair-the-summary`; `amending: 74bed2b "docs(changelog): cut the first release"`; area holds `amend` `base.json` `docs` `intent` `staged-snapshot.json` `workflow`(=`amend`); `amend` = the 40-char pinned sha; **HEAD unmoved, `git status` clean** | matches contract | R-1 |
| 2 | happy mint, **no** intent — the sha fallback | root | `jigc task amend` | 0 | none | M | `task minted: amend-6638746` (= `git rev-parse --short=7 HEAD`) | matches contract | R-16 |
| 3 | `amend.head-shape` — **root** HEAD | root | `jigc task amend "root probe"` | 1 | `amend.head-shape` | H | *"rewrites a single-parent commit, and HEAD is a root commit — it has no parent — nothing was minted"*; **`at: work-unit:root-probe`** (LOW-5's locus, fixed); `.jigc/tasks/` **does not exist** | matches contract · **LOW-5 CLOSED** | R-2 |
| 4 | the same, no intent | root | `jigc task amend` | 1 | same | H | `at: work-unit:amend-a602fef` — the id the mint *would* have taken | matches contract | R-2 |
| 5 | the same, `--format json` | root | `jigc --format json task amend "root probe"` | 1 | same | H | flattened `{"error": …}`, same three lines | matches contract | R-2 |
| 6 | `amend.head-shape` — **merge** HEAD | root | `jigc task amend "merge probe"` | 1 | same | H | *"HEAD is a merge commit — it has 2 parents"*; route names *"redo the merge with the message you want"*; nothing minted | matches contract | R-3b |
| 7 | **`write.unslugable-title`** × `{"" · whitespace · "###" · 的的的 · stopword-only}` | root | `jigc task amend "<t>"` | 1·1·1·1·**0** | `write.unslugable-title` (4 cells) | H | *"its id is slugged from the title, and this title slugs to nothing"*; `at: task`; **no area, `git status` 0 lines** in all four; `"the of and"` mints `and` (M47 generation-3 edge-stopword rule) | matches contract — **but the route omits this door's own sha exit → DEFECT F-C** | R-16 |
| 8 | mint over a **leftover** area at its own id, with a foreign plant | root | `jigc task amend` | 1 | `task.serial-collision` | H | the residual sentence (*"is a directory carrying no base pin"*); **plant 1 before / 1 after**; `precious.txt` still the only entry | matches contract | R-14 |
| 9 | mint twice on the same intent, plant present | root | `jigc task amend "dup probe"` ×2 | 1 | `task.serial-collision` | M | *"task `dup-probe` is already active"*; route `jigc start --task` / `jigc task discard … --force`; **plant 1/1** | matches contract | R-14 |
| 10 | **repository posture** — every `InProgress` member | root | `jigc task amend "…"` | **0** | none | M | mints; refuses nothing | matches contract (`BEHALF_DOORS` = `Neither`; settle row corrected 2026-09-26) — **LOW-7 half 1 CLOSED** | R-12 |
| 11 | **cwd (b) subdirectory** | docs/deep | `jigc task amend "subdir probe"` | 0 | none | M | mint ack identical to row 1; no checkout line (the two roots are one) | matches contract | R-5 |
| 12 | **cwd (d) linked worktree** — the checkout clause | linked | `jigc task amend "linked probe"` | 0 | none | M | ``that is the `HEAD` of the linked worktree at `<ABS linked>` on branch `feat` — not of the main checkout jigc's workbench binds to``; pin == the **standing** checkout's HEAD | **LOW-7 half 2 CLOSED** | R-7 |
| 13 | **cwd (c) fan-out worktree** | fan-out | `jigc task amend "fanout probe"` | 0 | none | M | the checkout clause renders **repo-relative** (`.jigc/worktrees/area-one`) — law 1 correct; pin == the worktree's HEAD, ≠ main's | matches contract; **OBS-A / OBS-B** | R-8 |
| 14 | `--format json` mint | root | `jigc --format json task amend "json probe"` | 0 | none | M | `{task, text}` exactly; **no `amending:` block, and no sha anywhere in `text` or in any key** | declared out by `text_json_parity_axis`'s census row — **but the settle says otherwise → DEFECT F-D** | R-15 |
| 15 | resume — `jigc start --task <amend-id>` | root | `jigc start --task repair-the-summary` | 0 | none | M | the `amending:` block **leads** the composed body (1 occurrence), *"already been pushed"* present (1); the step's pointer now names ``git log -1 --format=%s``, not *"the ack above"* | **MEDIUM-2 CLOSED** | R-1b |
| 16 | re-entry — `jigc workflow amend --task <id>` | root | `jigc workflow amend --task repair-the-summary` | 0 | none | M | byte-identical block (1 occurrence) | **MEDIUM-2 CLOSED** | R-1b |

### 1.2 · `jigc task finalize` — the **amend arm** (`COMMITTING_DOORS`' 11th row; `Disposition::Displace`) — 22 rows

| # | cell | cwd | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 17 | **happy path** | root | `jigc task finalize index-probe` | 0 | none | I | `amended 5259725 → 3f6ae33 — chore: …`; *"the tree and the author are unchanged; the message is re-authored and the committer becomes you, now"*; *"the superseded commit stays reachable in the reflog"*; **TREE byte-identical**, area torn down | matches contract · **LOW-8 CLOSED** | R-9 |
| 18 | landed `--format json` | root | `jigc --format json task finalize clean-probe` | 0 | none | I | top `{committed, findings, schema_version}`; `committed` = `{amended, displaced, files, hash, hook_output, left_out, manifest, promoted, subject}`; `amended`=`dd6dee4`, `hash`=`d002b7d` | matches contract (the declared additive key) | R-11 |
| 19 | **Displace — all move** | root | `jigc --format json task finalize repair-one` | 0 | none | I | `displaced: [{from ".jigc/tasks/repair-one/foreign.txt", to ".jigc/displaced/repair-one/foreign.txt"}]` · `files: 0` · `manifest: []`; stderr `… moved aside, not taken:` + the pair; **plant 1/1**; TREE unmoved | matches contract | R-3 |
| 20 | **Displace — none move** (`.jigc/displaced` a regular **file**) | root | same | 0 | `finalize.foreign-bytes` (advisory) | H | area **left standing**; `Not a directory (os error 20)`; key `{code, target:"task:repair-one"}`; `left_out` names `.jigc/displaced`; **0 host paths**; plant 1/1 | matches contract | R-4 |
| 21 | the pre-commit left-out header on the amend arm | root | same | 0 | — | — | ``finalize — about to rewrite HEAD's message; the committed tree does not move, so it leaves out:`` / ``left-out (unstaged/untracked — an amend commits no tree change, so none of it can join):`` — **no "git add to include"** | **MEDIUM-3 CLOSED** | R-4 |
| 22 | `finalize.amend-index-dirty` — **add** | root | `jigc task finalize index-probe` | **3** | `finalize.amend-index-dirty` | H | one finding per staged path; `at: added.txt`; route ``git -C <ABS> restore --staged -- added.txt`` + *"this arm takes no `--carry-staged`"*; **HEAD and TREE byte-unmoved** | matches contract | R-9 |
| 23 | the same — **modify** | root | same | 3 | same | H | `at: a.txt` | matches contract | R-9 |
| 24 | the same — **delete** (`git rm`) | root | same | 3 | same | H | `at: b.txt` | matches contract | R-9 |
| 25 | the same — **rename** (`git mv`) | root | same | 3 | same | H | `at: renamed.txt` — **destination only**, git's `--name-only` shape; the gate **re-fires** for `a.txt` after the first route runs, so the hole is closed by iteration, not by one round | matches contract (the review's stated behaviour) | R-9 · R-10 |
| 26 | the route **run verbatim**, both rounds | root | the emitted `git -C <ABS> restore --staged -- <p>` | 0 · 0 | — | — | round 1 clears `renamed.txt`, the gate re-fires on `a.txt`; round 2 clears it; the third finalize **lands** | matches contract | R-10 |
| 27 | `--dry-run` forecast, clean index | root | `jigc task finalize clean-probe --dry-run` | 0 | none | I | ``would rewrite dd6dee4 "<old>" → "chore: repair cleanly"`` + the tree/committer clause; **no `would commit`** | matches contract | R-11 |
| 28 | `--dry-run --format json` | root | `jigc --format json task finalize clean-probe --dry-run` | 0 | none | I | `{dry_run, findings, left_out, manifest, subject}` — **no superseded sha**, declared out at the parity census (the forecast's sha is `git log -1 HEAD` away) | matches contract | R-11 |
| 29 | `--dry-run` preview of the index gate | root | `jigc task finalize preview-probe --dry-run` | **3** | `finalize.amend-index-dirty` | H | the identical finding, before the manifest | matches contract | R-10 |
| 30 | `jigc task validate` preview of the index gate | root | `jigc task validate preview-probe` | **3** | same | H | byte-identical to row 29's finding | matches contract | R-10 |
| 31 | `jigc task validate` on a clean amend task | root | `jigc task validate clean-probe` | 0 | none | — | `no findings — the task validates clean` | matches contract | R-11 |
| 32 | `--carry-staged` **inert**, clean index | root | `… --carry-staged --dry-run` | 0 | none | I | byte-identical forecast to row 27 — accepted and inert, with both arg helps carrying the carve-out | matches contract · **MEDIUM-3 CLOSED** | R-11 |
| 33 | `finalize.base-mismatch` — HEAD moved by `milestone create` | root | `jigc task finalize moved-probe` | **3** | `finalize.base-mismatch` | H | *"`HEAD` is no longer the commit this amend was minted against — it pinned `291c6e5…` and `HEAD` is now `b9c3234…`"*; route `jigc task discard … --force`, then `jigc task amend`, **or** return HEAD to the pin; HEAD unmoved | matches contract | R-6b |
| 34 | the same, `--format json` | root | `jigc --format json task finalize moved-probe` | 3 | same | H | the findings envelope, `key {code, target:"task:moved-probe"}`, `location.address` the same | matches contract | R-6b |
| 35 | **hook-rejected** (`commit-msg` exit 1) | root | `jigc task finalize hook-probe` | 1 | `finalize.amend-rejected` *(log)* | M | the survivable frame: `` `git commit` was rejected (no commit was made):`` + the hook's stderr verbatim + the **state-truth clause** *"`HEAD` is unchanged — … and task hook-probe's authored commit doc is still in `.jigc/tasks/hook-probe/docs/`"* + the copy-runnable re-run; **HEAD sha, message body and committer date all byte-identical** | matches contract | R-13 |
| 36 | the log record for row 35 | root | `.jigc/logs/invocations.jsonl` | — | — | — | `{"argv":["task","finalize","hook-probe"],"exit_code":1,…,"binary_version":"1.0.0-rc.20","error_code":"finalize.amend-rejected"}` | matches contract (`ERROR_CODE_REGISTRY`'s 12th member) | R-13 |
| 37 | hook-rejected **with a foreign plant in the area** | root | same | 1 | same | M | plant stays **in the area** (`.jigc/tasks` 1 / `.jigc/displaced` 0) — the Displace has not run; area standing; commit doc still staged; then the re-run lands and displaces properly (area 0 / displaced 1) | matches contract | R-13 |
| 38 | **posture** × `{merge · bisect · revert · unmerged-index · detached}` | root | `jigc task finalize posture-probe` | 1 each | `repo.operation-in-progress` ×4 · `repo.head-detached` | H | each names its own noun and its own concluding command; **HEAD unmoved in all five** | matches contract | R-12 |
| 39 | **cwd (b) subdirectory** — the landing | docs/deep | `jigc --format json task finalize subdir-probe` | 0 | none | I | `amended 8728d48`, `hash 52043b4`, `files 0`; **0 host paths**; worktree clean after | matches contract | R-5 |
| 40 | **cwd (d) linked worktree** — the landing + `CommitSite` | linked | `jigc task finalize linked-probe` (+ `--dry-run`) | 0 · 0 | none | I | ``would commit / committed in the linked worktree at `<ABS>` on branch `feat` — not in the main checkout jigc's workbench binds to``; `feat` advances, **main HEAD unmoved** | matches contract (the declared absolute) | R-7 |
| 41 | **cwd (c) fan-out worktree** — the landing | fan-out | `jigc --format json task finalize fanout-probe` | 1 | `repo.head-detached` | H | refused **before** any commit; W HEAD unmoved | matches contract; **OBS-A** | R-8 |
| 42 | **spaced repo root** — the index route, quoted and run | docs/deep (spaced) | `jigc task finalize space-probe` | 3 | `finalize.amend-index-dirty` | H | ``git -C '/…/jigc space.Szc14F/…/repo' restore --staged -- 'my notes.txt'`` — **both** operands single-quoted; run verbatim → rc=0; the re-run lands | matches contract (M51's `shell_token`) | R-17 |

### 1.3 · `finalize.amend-staged-doc` — the HIGH-1 class over **all 8 `doc` Write leaves** — 9 rows

Derived, not assumed: `command grep -n '(&\["doc", "[a-z-]*"\], VerbKind::' crates/cli/src/cli.rs` → **8**
`Write` leaves (`create · add-item · remove-item · retitle-item · rename · set-field · set-slot · author`).

| # | leaf | argv (in an amend task, over a **committed** managed doc) | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|
| 43 | `doc set-slot` | `jigc doc set-slot vision:vision#thesis --from-file - --task doc-edit-probe` | 1 | `finalize.amend-staged-doc` | ``is a managed doc, and it promotes to `VISION.md` — but this task's commit model is an amend, which changes no tree``; `at: VISION.md`; route `jigc start "<intent>"`; **`git status` 0 lines** | matches contract | R-18 |
| 44 | `doc set-field` | `… changelog:changelog#releases/1-0-0/link --value …` | 1 | same | `at: CHANGELOG.md` | matches contract | R-18 |
| 45 | `doc add-item` | `… roadmap:roadmap#milestones --title "M Beta"` | 1 | same | `at: docs/roadmap.md` | matches contract | R-18 |
| 46 | `doc retitle-item` | `… roadmap:roadmap#milestones/m-alpha --title "M Alpha renamed"` | 1 | same | `at: docs/roadmap.md` | matches contract | R-18 |
| 47 | `doc remove-item` | `… roadmap:roadmap#milestones/m-alpha` | 1 | same | `at: docs/roadmap.md` | matches contract | R-18 |
| 48 | `doc rename` (non-singleton) | `jigc doc rename research:context-loss --to renamed-probe --task doc-edit-probe` | 1 | same | `at: docs/research/context-loss.md`; **nothing staged** — the area's `docs/` holds only `commit:doc-edit-probe.md` + `provenance.json` (the review's *already-refused re-slug staged the doc before refusing* cell) | matches contract | R-18b |
| 49 | `doc create` | `jigc doc create adr --title "A probe decision" --task doc-edit-probe` | 1 | `create.gate-blocked` | the amend workflow's `allows-create: []` refuses **earlier**; nothing staged | matches contract (a second, earlier fence) | R-18b |
| 50 | `doc author` | `jigc doc author vision --task doc-edit-probe --from-file -` | 1 | `create.gate-blocked` | the create-gate the batch verb implies refuses first; nothing staged | matches contract | R-18c |
| 51 | **control** — `git status` after all eight | — | — | — | worktree **clean**; no managed doc ever written into the tree; the area's `docs/` unchanged | **HIGH-1 CLOSED over its whole class** | R-18 · R-18b · R-18c |

### 1.4 · `AMBUSH_CONTRACTS`' third disposition, driven **by mutation** — 4 rows

| # | cell | argv | exit | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|
| 52 | control — the shipped `amend-message.yaml` | `jigc validate` on `fresh --repin` | 0 | `no findings — the committed store validates clean` | matches contract | R-19 |
| 53 | **delete only the dirty-index paragraph**, keep the declaration | same | **1** | `pack-load named-fact fence failed: step 'amend-message' declares 'finalize.amend-index-dirty' but its prose never says "fold the whole index"; … "non-empty index"; … "no flag"` + the route naming both escapes | **MEDIUM-4 CLOSED** | R-19 |
| 54 | **delete only the staged-doc paragraph** | same | **1** | the same fence for `finalize.amend-staged-doc` over *"exactly one doc"* and *"promotes"* | **MEDIUM-4 CLOSED** | R-19 |
| 55 | drop the two codes from `states-constraints:` | same | 0 | pack loads clean | matches contract — `DeclaredWhereReachable` is **outside** `ambush_class_codes`, so no pack *owes* a declarer; the `declarer:` claim is fenced by `crates/cli/tests/stated_at_fence.rs`, a repo test, not a runtime fence. Stated rather than filed | R-19 |

### 1.5 · `milestone provision` — `Refuse{--force}` × `LeftoverShape` × `LeftoverVerdict` — 12 rows

| # | cell | cwd | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|
| 56 | `Directory` + plant | docs/deep | 1 | `milestone.leftover-holds-work` | ``precious.txt — git reports no worktree of its own there`` (`NoOwnLinkage`); `at: .jigc/worktrees/area-one` | matches contract | R-20 |
| 57 | the same, `--force` | docs/deep | 0 | none | warning names `precious.txt` + *not recoverable*; then `provisioned 1 worktree(s) … at base 7df6484` | matches contract (consent) | R-20 |
| 58 | `Directory` with a **dangling `.git`** | docs/deep | 1 | same | ``.git, wip.txt — git cannot read a repository there`` (`Unverifiable`) | matches contract | R-20 |
| 59 | the same, `--force` | docs/deep | 0 | none | both entries listed | matches contract | R-20 |
| 60 | `File` (a plain regular file) | docs/deep | 1 | same | ``the file itself — it is a file, not a worktree, and nothing can say those bytes are disposable`` | matches contract | R-20 |
| 61 | the same, `--force` | docs/deep | 0 | none | ``warning: removing the leftover file .jigc/worktrees/area-one …:`` then **`area-one` — the path's own basename — in the child-entry position** | **F-B STILL-OPEN** (tier 3) | R-20 |
| 62 | `File` = a **symlink** to an outside tree | docs/deep | 1 | same | identical *the file itself* wording | matches contract | R-20 |
| 63 | the same, `--force` | docs/deep | 0 | none | link removed; **outside tree 1/1 intact** | matches contract | R-20 |
| 64 | `Unreadable` (`chmod 000`) | docs/deep | 1 | same | ``unknown — could not read the leftover directory `.jigc/worktrees/area-one`: Permission denied (os error 13)`` — fail-closed | matches contract | R-20 |
| 65 | the same, `--force` | docs/deep | 1 | `milestone.provision-failed` | ``could not clear the leftover … Permission denied``; *"0 of 1 worktree(s) landed before it, so the milestone is now partially provisioned"* — the honest partial | matches contract | R-20 |
| 66 | **S19** — the path is a **second repository's** linked worktree, mid-bisect, tree spotless | docs/deep | 1 | `milestone.leftover-holds-work` | ``a bisect git has left un-concluded (abandon it with `git -C <ABS> bisect reset`)``; **no *"registered here"* clause**; `$OTHER`'s worktree rows 2, `keep.txt` present | matches contract | R-21 |
| 67 | **S19**, `--force` | docs/deep | 0 | none | ``warning: removing the fan-out worktree … / a bisect git had left un-concluded … / not recoverable``; **`$OTHER/a.txt` intact and its committed `area-one` branch still in its object store** | matches contract | R-21 |
| 68 | a **registered** worktree carrying a live bisect (idempotent re-provision) | root | 0 | none | `provisioned 1 worktree(s) … at base 445c890`; **`BISECT_LOG` and the worktree untouched** | matches contract | R-22 |

### 1.6 · `milestone discard` — `Refuse{--force}` — 8 rows

| # | cell | cwd | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|
| 69 | 2 dirty sub-task worktrees | docs/deep | 1 | `milestone.dirty-worktree` | both named with `git status` codes + *"registered here, so the teardown removes it and this content is destroyed"*; `at: .jigc/worktrees/area-one` | matches contract | R-23 |
| 70 | **spotless** worktree carrying a live **bisect** | root | 1 | same | ``a bisect git has left un-concluded (abandon it with `git -C <ABS> bisect reset`)`` + the mid-operation clause of the route | matches contract (rc.18 post-review HIGH holds) | R-22 |
| 71 | the emitted bisect route **run verbatim** | root · docs/deep · the worktree | 0 · 0 · 0 | — | exits 0 from all three — census **C1-06 CLOSED** | matches contract | R-22 |
| 72 | `milestone.foreign-bytes` — the whole 5-locus complement | docs/deep | 1 | `milestone.foreign-bytes` | all 5 named (`tasks/area-one/t1.txt` · `tasks/area-two/t2.txt` · `merged/docs/y.txt` · `merged/x.txt` · `mnote.txt`); **before 5 / after 5** | matches contract | R-24 |
| 73 | **terminal** milestone | docs/deep | 1 | `milestone.terminal` | ``is `joined` — a settled milestone is over and has no workbench``; route names `doc show milestone-record:tango-probe` | matches contract | R-25 |
| 74 | `--force` **run from inside the worktree it removes** | the worktree | 0 | none | warning names `.jigc/milestones/bb-probe/plant.txt` + *not recoverable*; `discarded milestone:bb-probe (1 sub-task(s); workbench removed)`; worktree gone, `git worktree list` back to 1 row, plant 0 | matches contract (M52's declared behaviour) | R-22 |
| 75 | `milestone.zero-contribution` at `finalize` (the discard route's sibling) | docs/deep | 3 | `milestone.zero-contribution` | names `provision` + `discard`, explains the terminal flip | matches contract | R-26 |
| 76 | `milestone create` over a settled title | docs/deep | 1 | `milestone.record-exists` | ``(its record reads `joined`)``; route names a different title | matches contract | R-25 |

### 1.7 · `uninstall` — `Refuse{--force}` — 10 rows

| # | cell | cwd | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|
| 77 | clean install, a clean fan-out worktree present | **inside the fan-out worktree** | **0** | none | the 8 removal lines incl. ``pruned git's worktree registrations for the fan-out worktrees `.jigc/` held``; site line ``removed at `<ABS main>` — … and that workbench held the worktree you are standing in, which this removed``; measured in the main checkout: `.jigc` A · hook A · `CLAUDE.md` A · `SKILL.md` A; `git worktree list` 2 → **1**; `.git/worktrees/` **gone** | **rc.19 F-3 still CLOSED** | R-27 |
| 78 | **dirty** fan-out worktree | the worktree · root | 1 | `uninstall.dirty-worktree` | `.jigc/worktrees/area-one: A  f.txt`; **byte-identical from both cwds**; install still present | matches contract | R-28 |
| 79 | **untracked workbench file** | the worktree · root | 1 | `uninstall.untracked-workbench-file` | `.jigc/notes.txt`; route ``git -C <ABS> add -- <path>`` / ``checkout -- <path>``; **byte-identical** | matches contract | R-28 |
| 80 | **foreign bytes** in a hand-made task area | the worktree · root | 1 | `uninstall.foreign-bytes` | `.jigc/tasks/hand-made/notes.txt`; **byte-identical** | matches contract | R-28 |
| 81 | an un-concluded **bisect** in the fan-out worktree, tree spotless | the worktree · root | 1 | `uninstall.dirty-worktree` | ``a bisect git has left un-concluded (abandon it with `git -C <ABS> bisect reset`)`` + the mid-operation route clause; **byte-identical** | matches contract | R-28 |
| 82 | **staged prose** for an open sub-task | the fan-out worktree | 1 | `uninstall.staged-prose` | ``area-one: commit:area-one — a sub-task of milestone `pp-probe`, which `jigc task finalize` refuses: land it with `jigc milestone finalize pp-probe`, or drop it with `jigc task discard area-one --force``` — **names the real milestone** | matches contract | R-29 |
| 83 | the 5-locus foreign complement | docs/deep | 1 | `uninstall.foreign-bytes` | the **identical** 5 paths `milestone discard` named (row 72); before 5 / after 5 | matches contract | R-24 |
| 84 | the tracked-file warning + restore note | the fan-out worktree | 0 | none | ``warning: removing `.jigc/` also removes 5 tracked file(s) under it:`` (all five listed) + ``note: each is in the index, so `git -C <ABS> checkout -- <path>` brings it back.`` | matches contract | R-27 |
| 85 | idempotency | root | 0 | none | `(nothing to remove — no repo-local jigc install was present)` | matches contract | R-29 |
| 86 | **route placeholder** on the `dirty-worktree` arm | root · the worktree | 1 | `uninstall.dirty-worktree` | the route says ``jigc milestone discard <milestone-id> --force`` — an **unfilled** placeholder, while `staged-prose` one screen over names the real id | **OBS-E (rc.19 OBS-1) STILL-OPEN** | R-28 |

### 1.8 · `task discard` — `Refuse{--force}` — 11 rows

| # | cell | cwd | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|
| 87 | an **amend** task, jigc's own files only | root | 1 | `task-discard.staged-prose` | `commit:repair-one` named; **`foreign-bytes` does NOT fire** — the new `amend` marker, `base.json`, `intent`, `workflow`, `staged-snapshot.json` are all inside the writer set (`TASK_AREA_FILES` 15/15) | matches contract | R-30 |
| 88 | an amend task + one foreign byte | root | 1 | `task-discard.foreign-bytes` | `.jigc/tasks/repair-one/foreign.txt`; **plant 1/1** | matches contract | R-30 |
| 89 | the same, `--force` | root | 0 | none | warning + *not recoverable*; `discarded task repair-one — dropped staged edits to: commit:repair-one (transient)`; plant 0 | matches contract (consent) | R-30 |
| 90 | ordinary task, foreign byte — **four cwds** | root · docs/deep · fan-out · linked | 1 each | `task-discard.foreign-bytes` | **byte-identical from all four**, `0` host-path hits each | matches contract | R-31 |
| 91 | the same, `--force` — four cwds | all four | 0 each | none | byte-identical warning + ack, plant 0, `0` host paths | matches contract | R-31 |
| 92 | **F-1** — a symlink wearing a staged identity | docs/deep | 1 | `task-discard.foreign-bytes` | names `.jigc/tasks/…/docs/adr:via-symlink.md` as **a third party's**, while `doc list --task` calls the same path ``adr:via-symlink  docs/decisions/via-symlink.md  managed`` and `--force` acks ``dropped staged edits to: adr:via-symlink`` — **three surfaces, two answers, one path**; outside target 1/1 | **F-1 STILL-OPEN** (tier 3) | R-32 |
| 93 | **F-2** — the residual is a **symlink** | docs/deep | 1 | `finalize.no-task` | ``no task `ghost`: `.jigc/tasks/ghost` **is a directory** carrying no base pin`` — it is a symlink; outside target 1/1; the link still on disk | **F-2 STILL-OPEN** (tier 3) | R-33 |
| 94 | **F-2** at two more doors | docs/deep | 1 · 1 | same | `task validate ghost` and `task diff ghost` print the **byte-identical** sentence (one producer, `engine::state::residual_area_note`) | **F-2 STILL-OPEN at 3 doors** | R-33 |
| 95 | control for F-2 — a real directory | docs/deep | 1 | same | the same sentence, **true** there, with the by-hand route | matches contract | R-33 |
| 96 | **F-5** — a settled sub-task's leftover | docs/deep | 1 | `task-discard.staged-prose` | route names `jigc task discard area-two --force`… | **F-5 STILL-OPEN** | R-34 |
| 97 | …which then refuses | docs/deep | 1 | `milestone.terminal` | ``sub-task `area-two` … is already `joined` on the committed record — its working area is a leftover, not live work``; **the route one line up names a command that refuses**; HEAD unmoved | **F-5 STILL-OPEN, the route-dead-end half driven** | R-34 |

### 1.9 · `milestone finalize` — `Disposition::Displace` — 11 rows

| # | cell | cwd | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|
| 98 | **A3-1** — 8 planted loci (area root · `merged/top.txt` · `merged/docs/{deep.txt, provenance.json, adr.md}` · `merged/sub/` · a nested `sub/` · a sub-task area) | docs/deep | 0 | none | **8 before / 8 after**; 8 pairs on `committed.displaced` sorted by `from`, each foreign **directory** moved whole; both area roots empty; **0 host paths** | **A3-1 still CLOSED** | R-35 |
| 99 | **A3-2 / D1×D2** — `.jigc/displaced` a regular file | docs/deep | 0 | `finalize.foreign-bytes` ×2 (stderr) | **3 before / 3 after**; **both areas left standing**; one advisory per standing area keyed `milestone:beta-probe` / `task:area-one`; `displaced: []`; **0 host paths** | **A3-2 still CLOSED** | R-36 |
| 100 | the landed envelope's declared shape | docs/deep | 0 | — | top keys `['committed']` only; `committed` = `{commits, displaced, files, hash, hook_output, manifest, still_staged, sub_tasks, subject}` — **no `findings`**, so the two advisories reach stderr alone | matches contract (`ENVELOPE_ARMS` `Landed` = `Object(["committed"])`, `Pinned`) — **OBS-C** | R-35 · R-36 |
| 101 | **Displace from a linked worktree** (F-A's clean sibling) | linked | 0 | none | `displaced: [{from ".jigc/milestones/uniform-probe/mnote.txt", to ".jigc/displaced/uniform-probe/mnote.txt"}]`; stderr the same pair; **0 host paths**; plant 1/1 | matches contract | R-37 |
| 102 | **census C2-06** — the boundary from **four cwds** | root · docs/deep · fan-out · linked | 0 each | none | full landed envelope each (`files 2`, `sub_tasks 1`, `displaced 0`); **main HEAD advances in all four**; from `linked` it lands on **main**, `feat` unmoved; **0 host paths** in all four; areas cleared | **C2-06 still CLOSED** · **OBS-2** | R-38 |
| 103 | **F-4** — a **deleted tracked** file in the fan-out worktree | docs/deep | 0 | none | `area-one: keeper.md (never staged)` under ``the fan-out worktree is the only copy of these bytes — they are not recoverable``; after the run `git show HEAD:keeper.md` **returns the bytes** and the main checkout's copy is untouched | **F-4 STILL-OPEN** (tier 3) | R-39 |
| 104 | the `--ignored` axis (ignore rule committed **before** the pin) | docs/deep | 0 | none | `build/ (ignored by git) · scratch.txt (never staged)` on **both** streams — matching what `git status --porcelain --ignored` reports in that worktree | matches contract | R-40 |
| 105 | the commit-boundary ack's manifest | docs/deep | 0 | none | `modified docs/milestone-records/papa-probe.md` / `added first.txt` / `2 files committed` / `sub-tasks: area-one: 1 code file` | matches contract | R-40 |
| 106 | `milestone join` × three cwds | docs/deep · fan-out · linked | 0 each | none | the boundary is reachable and byte-consistent from each | matches contract | R-38 |
| 107 | `milestone execute` under the boundary | root | 0 | none | composes; the Spawn line renders | matches contract | R-34 |
| 108 | `milestone list-tasks` | root | 0 | none | the roster renders | matches contract | R-34 |

### 1.10 · the doors the axis touches — 6 rows

| # | door | cell | cwd | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 109 | `task list` | an open **amend** task | root | 0 | none | `1 active task(s)` / `open-amend  [amend]  open amend` — the workflow is named | matches contract | R-41 |
| 110 | `task list` | **F-5**'s settled sub-task leftover | docs/deep | 0 | none | `1 active task(s)` / `area-two [sub-task]` while `task discard` calls it a leftover | **F-5 STILL-OPEN** | R-34 |
| 111 | `start` (orientation) | an open amend task | root | 0 | none | `Active task: open-amend` / `workflow: amend` / `base: 793e3cf` / `staged: commit:open-amend` / the two conformance findings; four `Run:` affordances; the per-task coverage line says *"the carryover gate"* | matches contract — **OBS-D, the declared bound** | R-41 |
| 112 | `start "<intent>"` | a second work-start with an amend open | root | 0 | none | ``also open: 1 other task was already open before this call … - `open-amend` (workflow `amend`) — resume it with `jigc start --task open-amend``` | matches contract | R-41 |
| 113 | `rename` | over a task residual + a milestone-area residual, live task open | docs/deep | 1 | `rename.in-flight` | names `task finalize` / `task discard`; nothing changed | matches contract | R-42 |
| 114 | `rename` | the same after the task is discarded | docs/deep | 0 | none | `renamed research:context-loss -> research:renamed-probe (…), repointed 0 referrer(s)`; **both residuals still on disk** | matches contract | R-42 |
| 115 | `config set` | `invocation-log true` (the log fixture) | root | 0 | none | `config: set 'invocation-log' = 'true' — written to '.jigc/config/', uncommitted`; the `on` spelling refuses `config.value-rejected` | matches contract | R-13 |

**Row count: 115 driven rows over 28 doors** (the doors of §1.11's list; a verb that appears only in a
fixture — `jigc setup`, `jigc milestone add-task` — is **not** counted, because no row asserts its
verdict).

---

### 1.11 · Doors covered — every clap leaf that is the door of ≥1 driven row

`task amend` · `task finalize` · `task validate` · `task diff` · `task discard` · `task list` · `start` ·
`workflow` · `doc create` · `doc add-item` · `doc remove-item` · `doc retitle-item` · `doc rename` ·
`doc set-field` · `doc set-slot` · `doc author` · `doc list` · `milestone create` ·
`milestone provision` · `milestone execute` · `milestone join` · `milestone finalize` ·
`milestone discard` · `milestone list-tasks` · `uninstall` · `validate` · `rename` · `config set`
— **28 of the 48 `VERB_KINDS` leaves.** The other 20 are not doors of any cell on this axis; §6 item 10
names them rather than leaving them implicit.

---

## 2 · Findings

### F-C (new, tier 3) — `write.unslugable-title`'s route never names `jigc task amend`'s own sha fallback, the one exit that door uniquely has

**Tier 3** — *a surface says something the binary does not do*, in the *nothing hides* direction. No byte
is lost: the door refuses **before any write** (no area, `git status` clean).

**Driven.** `jigc task amend ""` (and `"   "`, `"###"`, `"的的的"`) → exit 1,
`blocking · write.unslugable-title`, route:

```
route: re-run with a title carrying ASCII letters or digits — the task id is slugged from it
```

**The contradiction.** At this door an intent is **optional**: `jigc task amend` with no argument mints
`amend-<sha7>` at exit 0 (row 2, driven: `task minted: amend-6638746`). The verb's own long help says so
— *"Omitted, the task is named after the commit it rewrites (`amend-<sha7>`)"* — and
`f10-amend-settle.md` states it as design. So the route names one of the door's two exits and hides the
cheaper one, which is the clause `design/surface-contract.md` → *nothing hides* governs.

**Why it is the incomplete-sweep shape, not a wording slip.** `write.unslugable-title` is a **shared
producer** across `MINT_DOORS`' prose rows, and the advice is *correct* at every pre-F-10 member —
`jigc start ""`, `jigc milestone create ""`, `jigc milestone add-task … ""` have no fallback, so
supplying a sluggable title is genuinely the only exit. `MINT_DOORS` grew a **sixth** member whose exit
set is wider, and the shared route was not re-derived over the widened set. That is M45's complete-fix
lens turned on M53's own new code: `MINT_DOORS` 5 → 6 and the route's axis is the registry.

**Class, bounded by drive.** One producer, one route string; the cell is `(jigc task amend, unslugable
intent)`. The other five `MINT_DOORS` rows are correct as written — driven at `jigc milestone create ""`
(exit 1, same route, and there *is* no fallback).

### F-D (new, tier 3) — the F-10 settle's Envelopes row states that `jigc task amend --format json` carries the pinned sha under `text`; it carries no sha anywhere

**Tier 3 — a record surface, not a binary behaviour.** Stated as a finding rather than an observation
because the settle is what Addendum 3 names *"the design of record"*, and the review method's DEFECT
predicate includes *a design doc rule*.

**Driven.**

```
$ jigc --format json task amend "json probe"                              -> exit 0
  keys: ['task', 'text']          task: json-probe
  'amending:' in text  -> False        'already been pushed' in text -> False
  regex \b[0-9a-f]{7,40}\b over the whole `text`  -> set()      <- NO sha, in any form
  stderr: empty
```

**The stated rule.** `f10-amend-settle.md` → *Envelopes*:

> `jigc task amend` → the composed `{task, text}` arm … **`--format json` carries the pinned sha under
> `text`, no new key needed — verify**; if a key is needed it is declared in the additive-key paragraph.

**The mitigating datum, stated because it decides the tier.** The row ends in *"— **verify**"*, and the
verification came back negative and was **acted on**: the MEDIUM-2 fix commit `e1cf5871` records the
decision in full (*"the block is presentation, DECLARED OUT of the pinned `{task, text}` arm … No key was
added and none was needed"*), and the authority is a code-side census row —
`crates/cli/tests/text_json_parity_axis.rs:314` `Disposition::DeclaredOut` with a three-clause reason
(the sha and subject are facts about the **repository**, readable by any driver from `git log -1 HEAD`).
So **the binary matches its authoritative declaration**; what is stale is the settle row. It was not
struck, while **two sibling rows in the same table carry dated correction brackets** added by the same
review pass (the posture row and the worktree row, both 2026-09-26, LOW-7) — so this file's own
convention was applied to two rows and not to the third.

A second, smaller half rides with it: that census row's *subject* sentence describes the surface as
*"prose, pinned as {task, text}, **led by the `amending:` block naming the commit at HEAD**"* while the
disposition beneath it declares that block out of the very arm the row is about. The two halves of one
row describe different surfaces.

### Still-open rc.19 rows (all tier 3, triaged to 1.x — expected)

| id | claim | re-driven verdict on rc.20 | repro |
|---|---|---|---|
| **F-1** | a symlink wearing a staged identity is jigc's own at the read/ack surfaces and a third party's at the destroying probe | **STILL-OPEN**, all **three** surfaces: `task-discard.foreign-bytes` names it · `doc list --task` calls it `managed` · `--force` acks *"dropped staged edits to: adr:via-symlink"*. Outside target 1/1 | R-32 |
| **F-2** | the residual note asserts *"is a directory"* over a shape it did not check | **STILL-OPEN** at **three** doors (`task discard` · `task validate` · `task diff`), byte-identical sentence; the real-directory control makes it true | R-33 |
| **F-4** | the fan-out teardown narrates a **deleted tracked** file as *"never staged"* under *"the only copy … not recoverable"* | **STILL-OPEN** — `git show HEAD:keeper.md` returns the bytes after the run and the main checkout's copy is untouched | R-39 |
| **F-5** | a settled sub-task's leftover is *"1 active task(s)"* at one door and *"a leftover, not live work"* at another, and a route names a command that refuses | **STILL-OPEN, all three halves** — and the route half driven in its cleanest form: `task-discard.staged-prose`'s route names `jigc task discard area-two --force`, which refuses `milestone.terminal` one invocation later. HEAD unmoved. **Reachability still undischarged** (built by restoring a `cp -R` backup) | R-34 |
| **F-B** | the `LeftoverShape::File` `--force` arm lists the leftover's own basename in the child-entry position | **STILL-OPEN** — `warning: removing the leftover file .jigc/worktrees/area-one …:` then `area-one`; the `Directory` control in the same block lists a real child (`precious.txt`) | R-20 |

### Observations (not findings)

- **OBS-A** — the amend mint door refuses HEAD's **shape** (`amend.head-shape` over root and merge,
  *"nothing was minted"*) and does **not** refuse HEAD's **attachment**. Driven in a provisioned fan-out
  worktree: `jigc task amend` mints at exit 0, and the finalize arm then refuses `repo.head-detached`
  forever, so the task can only be discarded (rows 13, 41). This matches the settle
  (`jigc task amend` is `BEHALF_DOORS`' `Neither` and adjudicates no posture member) and is recorded only
  because `head_shape_refusal`'s own doc-comment articulates the opposite principle — *"a mint that
  cannot land is a task the agent must then discard"* — and applies it on one axis of unlandability and
  not the other. The two axes were decided separately and on the record; nothing here is a contract
  break.
- **OBS-B** — the mint ack calls a provisioned **fan-out** worktree *"the linked worktree at
  `.jigc/worktrees/area-one`"*. Git's own vocabulary makes that true, and the path is **repo-relative**
  (law 1 correct). Recorded because a reader who knows jigc's fan-out vocabulary may read *linked* as
  *not jigc's*.
- **OBS-C** — `milestone finalize`'s landed envelope is `Object(["committed"])`, so the two
  `finalize.foreign-bytes` advisories naming standing areas reach **stderr text only**, while the
  identical cell at `task finalize` puts the advisory on `findings` (rows 20, 99, 100). This is the
  **declared, `Pinned`** shape — `ENVELOPE_ARMS`' row carries the reason (*"the two share no type, and
  only one of them carries `findings`"*) — so it is not a defect. Recorded because a driver reading only
  the machine surface sees the two `Displace` doors answer the same cell differently.
- **OBS-D** — orientation's per-task coverage line says *"the carryover gate"* on an amend task, where
  the answering check is `finalize.amend-index-dirty` (row 111). This is the declared bound
  [VERDICT.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/VERDICT.md)
  → Addendum 3 carries verbatim (*"a pinned envelope; one predicate, one authority"*). The composed
  `what's-left:` line **is** model-aware (*"the empty-index gate this arm refuses any staged path at"*),
  and `task finalize --help` states the substitution one paragraph below its own *"carryover gate"*
  sentence. Graded **against** the bound, per the acceptance design's instruction to its drivers.
- **OBS-E** (= rc.19's OBS-1) — `uninstall.dirty-worktree`'s route still says ``jigc milestone discard
  <milestone-id> --force`` with an **unfilled** placeholder while the `staged-prose` sibling one screen
  over names the real milestone (row 86 vs 82). Placeholders are the house style, so this stays an
  inconsistency rather than a contract break.
- **OBS-F** — `Disposition::Narrate` is **still empty by construction** at rc.20 (no
  `DESTROYING_DOORS` member holds it), so that arm of the disposition axis has no drivable cell. Stated
  rather than silently skipped.
- **OBS-G** — `AmbushDisposition::DeclaredWhereReachable` is, at **runtime**, weaker than its two
  siblings: dropping the two amend codes from `amend-message.yaml`'s `states-constraints:` leaves
  pack-load **green** (row 55), because the disposition is outside `ambush_class_codes` by design. What
  binds the `declarer:` claim is `crates/cli/tests/stated_at_fence.rs`, a **repo test**. That is the
  disposition's own written design and the price of admitting the cell at all; recorded because an
  adopter's project pack gets the named-fact fence (rows 53–54) and not the presence fence.

---

## 3 · The F-10 review's eight findings — each verified closed on the installed rc.20

| review finding | its own claim | verdict on rc.20 | the driven datum | repro |
|---|---|---|---|---|
| **HIGH-1** — an amend finalize promotes a staged managed doc into the worktree, commits nothing, baselines it anyway | exit-0 divergence behind a committing door + a false green on `jigc validate` | **CLOSED, over its whole class** | All **8** `doc` `Write` leaves driven against a committed managed doc in an amend task: six answer `finalize.amend-staged-doc` at exit 1 (`set-slot`/`set-field`/`add-item`/`retitle-item`/`remove-item`/`rename`), two are refused earlier by the workflow's `allows-create: []` (`create`/`author`). After all eight: `git status` **0 lines**, no managed doc in the tree, the area's `docs/` holds only `commit:<id>.md` + `provenance.json`. The `rename` cell — the one the fixer *added* beyond the review's three — refuses **without staging** | R-18 · R-18b · R-18c |
| **MEDIUM-2** — the resume and json arms drop the `amending:` block while the step points at it | the mitigation vanished on the path five trials show agents taking | **CLOSED at both re-compose doors** | `jigc start --task <id>` and `jigc workflow amend --task <id>` each render the block exactly once, with the pushed advisory; the step's pointer now names ``git log -1 --format=%s``. The json arm is **declared out** with a code-side census row — which is where **F-D** lands (the settle row was not struck) | R-1b · R-15 |
| **MEDIUM-3** — the left-out / dry-run / flag surfaces describe the ordinary model, and one instructs what the arm refuses | a route the same binary refuses | **CLOSED** | Both live render sites take the model: ``finalize — about to rewrite HEAD's message; the committed tree does not move, so it leaves out:`` and ``left-out (unstaged/untracked — an amend commits no tree change, so none of it can join):`` — **no `git add to include`**. Both arg helps carry the carve-out (`--dry-run`: *"the amend adds no commit, so the empty-commit guard does not apply"*; `--carry-staged`: *"inert on an **amend** task in every state"*), and `--carry-staged` is driven accepted-and-inert with a byte-identical forecast | R-4 · R-11 |
| **MEDIUM-4** — `finalize.amend-index-dirty` joined no `AMBUSH_CONTRACTS` row and bought no facts | the voluntary `states-constraints:` bought nothing | **CLOSED, driven by mutation on the release binary** | `AMBUSH_CONTRACTS` now carries **6** rows over **3** dispositions; deleting *only* the dirty-index paragraph reddens pack-load with three named facts missing (*"fold the whole index"*, *"non-empty index"*, *"no flag"*), and deleting *only* the staged-doc paragraph reddens with two (*"exactly one doc"*, *"promotes"*). Both routes name the two escapes | R-19 |
| **LOW-5** — `amend.head-shape`'s locus names an id the call would never have minted | the doc-comment's claim was false for the intent-supplied branch | **CLOSED** | `jigc task amend "root probe"` over a root HEAD → `at: work-unit:root-probe`; with no intent → `at: work-unit:amend-a602fef` | R-2 |
| **LOW-6** — the acceptance suite overstates its posture coverage | a test comment, not a user surface | **not a drivable surface** — the *behaviour* half re-verified: `{merge · bisect · revert · unmerged-index · detached}` all refuse at the amend arm with HEAD unmoved (5/5) | R-12 |
| **LOW-7** — two settle rows are not real at the mint door | *"transfers unchanged at both doors"*; *"the ack names the checkout"* | **CLOSED, both halves** | Half 1: the settle row now carries a dated bracket and the driven fact matches it (the mint door refuses no posture member; `BEHALF_DOORS` = `Neither`). Half 2 was **built**, not struck: from an attached linked worktree the **mint** ack renders ``that is the `HEAD` of the linked worktree at `<ABS>` on branch `feat` — not of the main checkout jigc's workbench binds to``, in its own tense, before any commit | R-7 · R-12 |
| **LOW-8** — *"only its message was rewritten"* omits the committer rewrite | an amend silently re-attributes | **CLOSED** | Both the forecast and the landed ack now read *"the tree and the author are unchanged; the message is re-authored and **the committer becomes you, now**"*, and the mint ack's own paragraph says *"it also resets the committer identity and date to yours"* | R-9 · R-11 |

---

## 4 · Repro blocks

All ran on `jigc 1.0.0-rc.20` (installed release), each in its own `dev/jigc-rig` throwaway repo (root
from `mktemp -d`; no teardown, no `rm` on a variable path). Exit codes read bare unless marked.
`command grep` carries every loss/survival claim.

### R-1 — the amend mint door, its marker and its area (rows 1, 15, 16)

```
setup: rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
HEAD before: d8bacd3 docs(changelog): cut the first release

$ jigc task amend "repair the summary"                                         -> exit 0
  task minted: repair-the-summary
  amending: 74bed2b "docs(changelog): cut the first release"
    `jigc task finalize repair-the-summary` replaces that message and leaves the commit's tree
    exactly as it is — so this repairs the message, never the change.
    If this commit has already been pushed, amending it rewrites shared history — jigc cannot
    tell whether it has.
  …then the composed `amend` step body (19 paragraphs)
AREA: amend  base.json  docs  intent  staged-snapshot.json  workflow
  cat amend    -> 74bed2ba0065c501bd1bfb150918e64a08461fdf        (the 40-char pin)
  cat base.json-> {"sha":"74bed2ba…","short":"74bed2b"}
  cat workflow -> amend
HEAD after mint: 74bed2b   git status --porcelain -> (empty)     <- the mint commits nothing
```

### R-1b — MEDIUM-2: every composing door names the commit (rows 15, 16)

```
$ jigc start --task repair-the-summary                                         -> exit 0
  amending: 7bf76cd "docs(changelog): cut the first release"          <- the block LEADS
  grep -c 'amending:'          -> 1
  grep -c 'already been pushed'-> 1
  the step's pointer now reads: "check the subject line of the commit this task pinned
    (`git log -1 --format=%s`) and stop if it is one of them"        <- names a COMMAND
$ jigc workflow amend --task repair-the-summary                                -> exit 0
  grep -c 'amending:'          -> 1                                  <- byte-identical block
```

### R-2 — `amend.head-shape`, the root cell, and LOW-5's locus (rows 3, 4, 5)

```
setup: dev/jigc-rig bare; jigc setup; git checkout --orphan rootline; git add -A;
       git commit -q -m "root commit" --no-verify      (rc=0, read bare)
  git rev-list --parents -1 HEAD | awk '{print NF-1}'  -> 0          <- a ROOT commit

$ jigc task amend "root probe"                                                 -> exit 1
  blocking · amend.head-shape — `jigc task amend` rewrites a single-parent commit, and HEAD is
    a root commit — it has no parent — nothing was minted
    at: work-unit:root-probe                                         <- LOW-5's locus, FIXED
    route: a repository's first commit is not rewritten on your behalf; amend it yourself with
      `git commit --amend` if that is what you want
$ jigc task amend            (no intent)                                       -> exit 1
    at: work-unit:amend-a602fef                                      <- the id it WOULD take
$ jigc --format json task amend "root probe"                                   -> exit 1
  {"error":"blocking · amend.head-shape — … \n  at: work-unit:root-probe\n  route: …"}
ls .jigc/tasks/ -> No such file or directory                         <- NOTHING minted
```

### R-3 — the amend arm's Displace, all-move (row 19)

```
setup: committed-singletons; jigc task amend "repair one"; type=docs; summary authored
       KEEP-A3-BYTES > .jigc/tasks/repair-one/foreign.txt
BEFORE: command grep -rl 'KEEP-A3-BYTES' .jigc | wc -l  -> 1

$ jigc --format json task finalize repair-one                                  -> exit 0
  "committed": { "amended":"22b9e1f", "hash":"cf85cb6", "files":0, "manifest":[],
    "promoted":[], "left_out":[], "hook_output":"",
    "subject":"docs: rename the changelog release heading",
    "displaced":[{"from":".jigc/tasks/repair-one/foreign.txt",
                  "to":  ".jigc/displaced/repair-one/foreign.txt"}] },
  "findings":[], "schema_version":3
  stderr: note: the working area held 1 entry jigc did not write … moved aside, not taken:
      .jigc/tasks/repair-one/foreign.txt → .jigc/displaced/repair-one/foreign.txt
AFTER: 1   (at .jigc/displaced/repair-one/foreign.txt)
HEAD 22b9e1f… -> cf85cb6…      TREE ee956c6… -> ee956c6…            <- TREE BYTE-IDENTICAL
.jigc/tasks/ -> (empty)                                             <- teardown ran
```

### R-3b — `amend.head-shape`, the merge cell (row 6)

```
setup: committed-singletons; git checkout -b side HEAD~1; commit "side";
       git checkout main; git merge --no-ff -m "merge side" side    (rc=0, read bare)
  git rev-list --parents -1 HEAD | awk '{print NF-1}'  -> 2

$ jigc task amend "merge probe"                                                -> exit 1
  blocking · amend.head-shape — … HEAD is a merge commit — it has 2 parents — nothing was minted
    at: work-unit:merge-probe
    route: a merge commit's message is git's own narration of the merge, not a
      Conventional-Commits subject; leave it, or redo the merge with the message you want
ls .jigc/tasks/ -> (empty)
```

### R-4 — the amend arm's Displace, none-move, and MEDIUM-3's header (rows 20, 21)

```
setup: as R-3, plus  printf 'NOT A DIR' > .jigc/displaced
BEFORE: 1

$ jigc --format json task finalize repair-one                                  -> exit 0
  committed.displaced []   committed.amended "8253ddf"   committed.hash "b68b7de"
  findings[0] advisory finalize.foreign-bytes
    key      {"code":"finalize.foreign-bytes","target":"task:repair-one"}
    location {"address":"task:repair-one","line":1,"col":1}
    message  "`.jigc/tasks/repair-one` holds 1 path(s) jigc did not write, so the working area
      was left standing rather than removed with them … `.jigc/tasks/repair-one/foreign.txt`;
      1 of them could not be moved aside: … could not open .jigc/displaced/repair-one to park
      it: Not a directory (os error 20)"
    route    "the commit landed and nothing in it is affected. Keep what you need from
      `.jigc/tasks/repair-one` and delete the rest — jigc mints no verb that clears it"
  stderr (the MEDIUM-3 surfaces):
    note: … not one of them could be moved aside:  <the path> — <the os error>
    finalize — about to rewrite HEAD's message; the committed tree does not move, so it leaves out:
      left-out (unstaged/untracked — an amend commits no tree change, so none of it can join):
        .jigc/displaced
AFTER: 1   area STANDING (.jigc/tasks/ -> repair-one)
host-absolute hits over stdout+stderr: 0
```

### R-5 — cwd (b): the amend arc from a subdirectory (rows 11, 39)

```
setup: committed-singletons; mkdir -p docs/deep; cd docs/deep
$ jigc task amend "subdir probe"       -> 0   amending: 8728d48 "docs(changelog): …"
$ jigc --format json task finalize subdir-probe                                -> exit 0
  hash 52043b4  amended 8728d48  files 0  displaced []
  stderr: (empty)      host-absolute hits: 0
HEAD 8728d48 -> 52043b4 ;  git status --porcelain -> 0 lines
```

### R-6 — F-A RE-DRIVEN: `task finalize`'s Displace from a linked worktree (rc.19's one tier-2/3 row)

```
setup: dev/jigc-rig fresh; git -C $REPO worktree add -b feat $RIG/linked
       (cd linked) jigc start --workflow quick-fix "tidy the readme"; echo code > linked/src.txt;
       git -C linked add src.txt; the commit doc filled from `linked`
       KEEPHP > $REPO/.jigc/tasks/tidy-the-readme/notes.txt            BEFORE 1

$ (cd linked) jigc --format json task finalize tidy-the-readme                 -> exit 0
  "displaced":[{"from":".jigc/tasks/tidy-the-readme/notes.txt",
                "to":  ".jigc/displaced/tidy-the-readme/notes.txt"}]   <- REPO-RELATIVE
  manifest [{"kind":"added","path":"src.txt"}]   files 1   hash 58f8b37
  stderr   .jigc/tasks/tidy-the-readme/notes.txt → .jigc/displaced/tidy-the-readme/notes.txt
  host-absolute hits (stdout+stderr): 0            (on rc.19: 3)       AFTER 1
THE NONE-MOVE CELL, same cwd (printf 'NOT A DIR' > .jigc/displaced)            -> exit 0
  findings[0].message  three path occurrences, ALL repo-relative
  findings[0].route    "… Keep what you need from `.jigc/tasks/tidy-the-readme` …"  repo-relative
  findings[0].key      {"code":"finalize.foreign-bytes","target":"task:tidy-the-readme"}
  committed.displaced  []      area STANDING      AFTER 1
  host-absolute hits: 0                                             <- F-A CLOSED, BOTH CELLS
```

### R-6b — `finalize.base-mismatch` on the amend arm (rows 33, 34)

```
setup: committed-singletons; jigc task amend "moved probe"; doc authored
  pin = 291c6e56aac66cca271dea025357bbf52ce1945c
$ jigc milestone create "Move probe"    (a record commit moves HEAD)            -> rc 0
  HEAD now = b9c3234a11e2775cecabb988f5cc607a74de9e8d
$ jigc task finalize moved-probe                                               -> exit 3
  blocking · finalize.base-mismatch — `HEAD` is no longer the commit this amend was minted
    against — it pinned `291c6e56…` and `HEAD` is now `b9c3234a…`, so the message this task
    authored would rewrite a different commit
    at: task:moved-probe
    route: start the repair again against the commit that is there now (`jigc task discard
      moved-probe --force`, then `jigc task amend`), or return `HEAD` to `291c6e56…` first
$ jigc --format json task finalize moved-probe                                 -> exit 3
  the findings envelope, key {code, target:"task:moved-probe"}, location.address the same
HEAD unmoved: b9c3234a…
```

### R-7 — cwd (d): the linked worktree, and LOW-7's mint-ack checkout clause (rows 12, 40)

```
setup: committed-singletons; git -C $REPO worktree add -b feat $RIG/linked; cd linked
$ jigc task amend "linked probe"                                               -> exit 0
  task minted: linked-probe
  amending: 617cc8f "docs(changelog): cut the first release"
    that is the `HEAD` of the linked worktree at `<ABS>/linked` on branch `feat` — not of the
    main checkout jigc's workbench binds to                       <- LOW-7 half 2, BUILT
  linked HEAD == main HEAD == pin == 617cc8f4…
$ jigc task finalize linked-probe --dry-run                                    -> exit 0
  would rewrite 617cc8f "docs(changelog): cut the first release" → "chore: repair from a
    linked worktree"
    the tree and the author are unchanged; the message is re-authored and the committer
    becomes you, now                                             <- LOW-8, CLOSED
    would commit in the linked worktree at `<ABS>/linked` on branch `feat` — not in the main
    checkout jigc's workbench binds to
$ jigc task finalize linked-probe                                              -> exit 0
  amended 617cc8f → 0094b9e — chore: repair from a linked worktree
    … / the superseded commit stays reachable in the reflog / committed in the linked worktree …
  linked HEAD 0094b9e on feat  ·  main HEAD 617cc8f on main       <- main UNMOVED
```

### R-8 — cwd (c): the fan-out worktree (rows 13, 41)

```
setup: committed-singletons; milestone create "Wt probe" + one sub-task; provision
  W=.jigc/worktrees/area-one ; git -C $W symbolic-ref -q HEAD -> (none)  DETACHED
$ (cd W) jigc task amend "fanout probe"                                        -> exit 0
  amending: f3cbb33 "docs(changelog): cut the first release"
    that is the `HEAD` of the linked worktree at `.jigc/worktrees/area-one` — not of the main
    checkout jigc's workbench binds to                           <- REPO-RELATIVE (law 1)
  pin = f3cbb336… == W HEAD ;  main HEAD = 7908562…              <- pins where you STAND
$ (cd W) jigc --format json task finalize fanout-probe                         -> exit 1
  {"error":"blocking · repo.head-detached — HEAD is detached — a commit made here would belong
   to no branch … route: re-attach HEAD with `git switch <branch>`, then re-run this command"}
  W HEAD unmoved                                                  <- OBS-A lives here
```

### R-9 — `finalize.amend-index-dirty` over add · modify · delete · rename (rows 17, 22–25)

```
setup: committed-singletons; a.txt + b.txt committed ("base files", rc read bare);
       jigc task amend "index probe"; type=chore; summary authored
  HEAD before = 50b85b70…   TREE before = f5dbb3f9…

CELL add     : printf new > added.txt; git add added.txt        -> exit 3
  blocking · finalize.amend-index-dirty — `added.txt` is staged, and an amend rewrites `HEAD`
    from the index — finalizing now would fold it into the commit whose message this task is
    repairing, a change that commit never carried
    at: added.txt
    route: unstage it (`git -C <ABS repo> restore --staged -- added.txt`) and re-run the
      finalize — this arm takes no `--carry-staged`, because an amend that carried anything
      would change a tree it promised not to touch
CELL modify  : a.txt appended, git add a.txt                    -> 3   at: a.txt
CELL delete  : git rm -q b.txt  (status "D  b.txt")             -> 3   at: b.txt
CELL rename  : git mv a.txt renamed.txt (status "R  a.txt -> renamed.txt")
                                                                -> 3   at: renamed.txt
HEAD after all four = 50b85b70…   TREE = f5dbb3f9…              <- BOTH BYTE-UNMOVED
```

### R-10 — the route run verbatim, both rounds, and the two previews (rows 26, 29, 30)

```
setup: as R-9's rename cell
$ git -C $REPO restore --staged -- renamed.txt          (the emitted span)  -> rc 0
  git status --porcelain -> "D  a.txt" / "?? renamed.txt"
$ jigc task finalize index-probe                                -> 3   at: a.txt   <- re-fires
$ git -C $REPO restore --staged -- a.txt                                    -> rc 0
  git status --porcelain -> " D a.txt" / "?? renamed.txt"
$ jigc task finalize index-probe                                            -> exit 0
  left-out (unstaged/untracked — an amend commits no tree change, so none of it can join):
    a.txt / renamed.txt
  amended 5259725 → 3f6ae33 — chore: repair the base a message

PREVIEWS (a fresh rig, a staged p.txt):
$ jigc task validate preview-probe               -> exit 3  finalize.amend-index-dirty
$ jigc task finalize preview-probe --dry-run     -> exit 3  BYTE-IDENTICAL finding
```

### R-11 — the clean-task previews, the landed envelope, `--carry-staged`'s inertness (rows 18, 27, 28, 31, 32)

```
setup: committed-singletons; jigc task amend "clean probe"; type=chore; summary authored
$ jigc task validate clean-probe                 -> 0  "no findings — the task validates clean"
$ jigc task finalize clean-probe --dry-run       -> 0
  finalize --dry-run — pre-commit manifest (nothing committed)
  would rewrite dd6dee4 "docs(changelog): cut the first release" → "chore: repair cleanly"
    the tree and the author are unchanged; the message is re-authored and the committer
    becomes you, now
$ jigc --format json task finalize clean-probe --dry-run -> 0
  {"dry_run":true,"findings":[],"left_out":[],"manifest":[],"subject":"chore: repair cleanly"}
                                                        <- NO superseded sha (declared out)
$ jigc task finalize clean-probe --carry-staged --dry-run -> 0   BYTE-IDENTICAL to the above
$ jigc --format json task finalize clean-probe    -> 0
  top    ['committed','findings','schema_version']
  committed ['amended','displaced','files','hash','hook_output','left_out','manifest',
             'promoted','subject']       amended dd6dee4   hash d002b7d
```

### R-12 — posture at the amend arm, and the mint door's silence (rows 10, 38)

```
setup per cell: committed-singletons; jigc task amend "posture probe"; doc authored; then the
  git state entered by running the command a user runs.

merge           -> exit 1  repo.operation-in-progress — a merge is in progress …
  route: conclude it with `git merge --continue` … or abandon it with `git merge --abort` …
bisect          -> exit 1  repo.operation-in-progress — a bisect is in progress …
  route: conclude it, or abandon it with `git bisect reset` …
revert          -> exit 1  repo.operation-in-progress — a revert is in progress …
unmerged-index  -> exit 1  repo.operation-in-progress — a merge is in progress …
detached        -> exit 1  repo.head-detached — HEAD is detached …
HEAD unmoved: YES in all five.
THE MINT DOOR under the same states: `jigc task amend "…"` -> exit 0, mints, refuses nothing.
```

### R-13 — the hook-rejected amend, the log identity, and the plant (rows 35, 36, 37, 115)

```
setup: committed-singletons; jigc config set invocation-log true   -> rc 0
         ("config: set `invocation-log` = `true` — written to `.jigc/config/`, uncommitted";
          the `on` spelling is refused `config.value-rejected`)
       .git/hooks/commit-msg = { echo "hook: refusing" >&2 ; exit 1 }
       jigc task amend "reject probe"; doc authored
       KEEP-RJ > .jigc/tasks/reject-probe/foreign.txt
BEFORE: area 1 / displaced 0

$ jigc task finalize reject-probe                                              -> exit 1
  `git commit` was rejected (no commit was made):
  hook: refusing
  `HEAD` is unchanged — the commit this task is repairing still carries the message it had,
  and task reject-probe's authored commit doc is still in `.jigc/tasks/reject-probe/docs/`.
  Fix the hook's complaint, then re-run `jigc task finalize reject-probe`.
HEAD sha unmoved YES · message body unmoved YES · committer date unmoved YES
AFTER: area 1 / displaced 0   area STANDING   docs/ -> commit:reject-probe.md provenance.json
.jigc/logs/invocations.jsonl:
  {"argv":["task","finalize","hook-probe"],"exit_code":1,"duration_ms":443,"finding_codes":[],
   "output_bytes":326,"binary_version":"1.0.0-rc.20","error_code":"finalize.amend-rejected"}
THEN remove the hook and re-run                                                -> exit 0
  amended 3fc501a → 197ee77 ; displaced [{.jigc/tasks/reject-probe/foreign.txt ->
    .jigc/displaced/reject-probe/foreign.txt}] ; area 0 / displaced 1
```

### R-14 — the mint door over a leftover and over an active id (rows 8, 9)

```
CELL leftover: committed-singletons; SHA7=$(git rev-parse --short=7 HEAD) -> 922b058
  mkdir .jigc/tasks/amend-922b058 ; KEEP-MINT-PLANT > …/precious.txt     BEFORE 1
$ jigc task amend                                                (read bare) -> exit 1
  blocking · task.serial-collision — cannot mint task `amend-922b058`:
    `.jigc/tasks/amend-922b058` is a directory carrying no base pin, so it is a leftover and
    not a work unit …
    route: nothing was changed. Keep anything you need from `.jigc/tasks/amend-922b058` and
      delete the rest by hand …
AFTER 1 ; ls the area -> precious.txt (only)

CELL active: jigc task amend "dup probe"; KEEP-DUP > .jigc/tasks/dup-probe/keeper.txt
$ jigc task amend "dup probe"                                    (read bare) -> exit 1
  blocking · task.serial-collision — task `dup-probe` is already active
    route: resume with `jigc start --task dup-probe` or abandon with
      `jigc task discard dup-probe --force`
plant AFTER 1
```

### R-15 — F-D: the json mint arm carries no sha (row 14)

```
$ jigc --format json task amend "json probe"                                   -> exit 0
  keys: ['task','text']                task: json-probe
  'amending:' in text                  -> False
  'already been pushed' in text         -> False
  re.findall(r'\b[0-9a-f]{7,40}\b', text) -> set()
  len(text) = 4188 ; stderr empty
  text begins "The commit at HEAD keeps every byte of its tree, and its author; …"
  text ends   "Run: `jigc task finalize json-probe`"
```

### R-16 — `write.unslugable-title` at the amend door, and the fallback its route hides (rows 2, 7 — F-C)

```
setup: committed-singletons
intent=[]           -> exit 1  write.unslugable-title   at: task   areas [] git status 0
intent=[   ]        -> exit 1  same
intent=[###]        -> exit 1  same
intent=[的的的]      -> exit 1  same
  route (all four): "re-run with a title carrying ASCII letters or digits — the task id is
    slugged from it"
intent=[the of and] -> exit 0  task minted: and          (edge-stopword rule, generation 3)

THE EXIT THE ROUTE DOES NOT NAME:
$ jigc task amend        (no intent at all)                                    -> exit 0
  task minted: amend-6638746      (HEAD sha7 = 6638746)
CONTROL, a MINT_DOORS row with no fallback:
$ jigc milestone create ""                                     -> exit 1, the same route, and
  there the advice is the only exit.
```

### R-17 — a repository root containing a SPACE: the index route, quoted and run (row 42)

```
setup: SCRATCH=$(mktemp -d "$S/rigs/jigc space.XXXXXX"); dev/jigc-rig committed-singletons
  REPO=/…/jigc space.Szc14F/jigc-rig-committed-singletons-rwWPIC/repo
  cd docs/deep; jigc task amend "space probe"; doc authored
  printf x > "$REPO/my notes.txt"; git add -- "my notes.txt"

$ jigc task finalize space-probe                                               -> exit 3
  blocking · finalize.amend-index-dirty — `my notes.txt` is staged …
    at: my notes.txt
    route: unstage it (`git -C '/…/jigc space.Szc14F/…/repo' restore --staged -- 'my notes.txt'`)
      and re-run the finalize — this arm takes no `--carry-staged` …
SPAN extracted and RUN VERBATIM from docs/deep                                 -> rc 0
  git status --porcelain -> ?? "my notes.txt"
$ jigc task finalize space-probe                                               -> exit 0
  amended a160a6e → 66118c4 — chore: repair under a spaced root
```

### R-18 — HIGH-1's class: five `doc` Write leaves (rows 43–47, 51)

```
setup: committed-singletons; jigc task amend "doc edit probe"    (NO doc authored — the guard
       fires at the write door, before the finalize)

set-slot      vision:vision#thesis              -> exit 1  finalize.amend-staged-doc
  `vision:vision` is a managed doc, and it promotes to `VISION.md` — but this task's commit
  model is an amend, which changes no tree: its finalize would write `VISION.md` into the
  worktree and commit none of it, leaving the file diverged from the commit it just rewrote
    at: VISION.md
    route: `jigc start "<intent>"` mints an ordinary task, whose finalize commits the promoted
      doc; this amend task keeps its own job, repairing `HEAD`'s message
set-field     changelog:changelog#releases/1-0-0/link -> 1  at: CHANGELOG.md
add-item      roadmap:roadmap#milestones             -> 1  at: docs/roadmap.md
retitle-item  roadmap:roadmap#milestones/m-alpha     -> 1  at: docs/roadmap.md
remove-item   roadmap:roadmap#milestones/m-alpha     -> 1  at: docs/roadmap.md
git status --porcelain after all five -> 0 lines
.jigc/tasks/doc-edit-probe/docs/ -> commit:doc-edit-probe.md  provenance.json
```

### R-18b — HIGH-1's class: `doc rename` over a non-singleton, and `doc create` (rows 48, 49)

```
setup: dev/jigc-rig refs-post-hoc; jigc task discard $RIG_TASK --force;
       jigc task amend "doc edit probe"
  jigc doc list -> changelog / decisions-log / research:context-loss / roadmap / vision

$ jigc doc rename research:context-loss --to renamed-probe --task doc-edit-probe -> exit 1
  blocking · finalize.amend-staged-doc — `research:context-loss` … promotes to
    `docs/research/context-loss.md` …    at: docs/research/context-loss.md
  git status -> 0 lines ; area docs/ -> commit:doc-edit-probe.md provenance.json
                                                      <- nothing staged before the refusal
$ jigc doc create adr --title "A probe decision" --task doc-edit-probe          -> exit 1
  blocking · create.gate-blocked — the workflow does not allow `jigc doc create adr` in-task;
    allowed doctypes: []      at: adr
```

### R-18c — HIGH-1's class: `doc author` (row 50)

```
$ jigc doc author vision --task doc-edit-probe --from-file - <<'EOF'            -> exit 1
sections:
  - id: thesis
    set: {thesis: <<A new thesis authored through the batch verb>>}
EOF
  blocking · create.gate-blocked — the workflow does not allow `jigc doc create vision`
    in-task; allowed doctypes: []      at: vision
  git status -> 0 lines ; area docs/ unchanged
(a malformed payload answers `write.wrong-shape` first, driven, with nothing staged either way)
```

### R-19 — MEDIUM-4, driven by MUTATION on the release binary (rows 52–55)

```
setup: rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc --repin) || exit; eval "$rig"
  (--repin KEEPS the freeze manifest; --pack-from-dev DROPS it, and the four sibling fences —
   assert_named_facts_stated among them — take the manifest subject, so the mutation is
   invisible under --pack-from-dev. Recorded because my first attempt used it.)
  JIGC_PACK_DIR=<rig>/pack

CONTROL, shipped amend-message.yaml                    jigc validate -> exit 0
MUTANT B2: front matter kept, ONLY the dirty-index body paragraph deleted    -> exit 1
  pack-load named-fact fence failed: step `amend-message` declares
  `finalize.amend-index-dirty` but its prose never says "fold the whole index"; … never says
  "non-empty index"; … never says "no flag" — the `states-constraints:` declaration would buy
  presence alone while the contract itself went unstated …
  route: restate each named fact in that step's body, above the solicit the constraint gates —
    or, if the step no longer states the contract, drop its code from the step's
    `states-constraints:` front-matter
MUTANT C: ONLY the staged-doc paragraph deleted                              -> exit 1
  … declares `finalize.amend-staged-doc` but its prose never says "exactly one doc"; … never
  says "promotes" …
MUTANT A: the two codes dropped from states-constraints:                     -> exit 0
  (DeclaredWhereReachable is outside `ambush_class_codes`; the declarer claim is fenced by
   crates/cli/tests/stated_at_fence.rs, a repo test — OBS-G)
RESTORE the shipped bytes                              jigc validate -> exit 0
```

### R-20 — `LeftoverShape` × `LeftoverVerdict` × `--force` at `milestone provision`, and F-B (rows 56–65)

```
setup per cell: dev/jigc-rig fresh; milestone create "Quebec probe" + one sub-task
  (NOT provisioned); the leftover planted at .jigc/worktrees/area-one; cwd = docs/deep

Directory + precious.txt   -> 1 milestone.leftover-holds-work "precious.txt — git reports no
                               worktree of its own there";  --force -> 0, warning lists
                               precious.txt, plant GONE, "provisioned 1 worktree(s) … base 7df6484"
dangling .git inside       -> 1 ".git, wip.txt — git cannot read a repository there"
                               --force -> 0, BOTH entries listed
File (regular)             -> 1 "the file itself — it is a file, not a worktree, and nothing
                               can say those bytes are disposable"
  --force -> 0   warning: removing the leftover file .jigc/worktrees/area-one discards work
                 that is not in git:
                     area-one              <- THE PATH'S OWN BASENAME  (F-B)
                 note: the leftover file is the only copy of these bytes …
  CONTROL in the same block: the Directory arm lists `precious.txt`, a real child.
File (symlink→outside)     -> 1 identical wording;  --force -> 0; outside tree 1/1 INTACT
Unreadable (chmod 000)     -> 1 "unknown — could not read the leftover directory
                               `.jigc/worktrees/area-one`: Permission denied (os error 13)"
  --force -> 1 milestone.provision-failed "could not clear the leftover … Permission denied.
    0 of 1 worktree(s) landed before it, so the milestone is now partially provisioned"
```

### R-21 — S19: the leftover is a SECOND repository's linked worktree, mid-bisect (rows 66, 67)

```
setup: fresh; milestone create "Sierra probe" + one sub-task
  OTHER=$(mktemp -d); git init; two commits; git -C $OTHER worktree add \
    $REPO/.jigc/worktrees/area-one ; OTHERKEEP > area-one/keep.txt, committed THERE;
  git -C area-one bisect start/bad
  git -C area-one status --porcelain -> 0 lines            <- SPOTLESS

$ (cd docs/deep) jigc milestone provision sierra-probe                         -> exit 1
  milestone.leftover-holds-work — .jigc/worktrees/area-one: a bisect git has left un-concluded
    (abandon it with `git -C <ABS> bisect reset`) — … registered here clause: ABSENT (correct)
AFTER: $OTHER worktree rows 2 · keep.txt PRESENT
$ (cd docs/deep) jigc milestone provision sierra-probe --force                 -> exit 0
  warning: removing the fan-out worktree .jigc/worktrees/area-one discards work that is not in
    git:  a bisect git had left un-concluded — it can only be concluded or abandoned from this
    checkout / note: … not recoverable.
AFTER: $OTHER/a.txt intact ("a") ; git -C $OTHER branch -a -> "+ area-one  * main"
       <- the committed bytes are in $OTHER's object store; what died was a checkout
```

### R-22 — the operation-in-progress leg, the route run verbatim, and `--force` from inside (rows 68, 70, 71, 74)

```
setup: fresh; milestone create "Bb probe" + one sub-task; provision;
       git -C W bisect start; git -C W bisect bad ; git -C W status --porcelain -> 0 lines

$ jigc milestone provision bb-probe        (idempotent re-provision)           -> exit 0
  "provisioned 1 worktree(s) for milestone:bb-probe at base 445c890 (area-one)"
  BISECT_LOG PRESENT · worktree PRESENT                    <- neither probed nor cleared
$ jigc milestone discard bb-probe                                              -> exit 1
  milestone.dirty-worktree — .jigc/worktrees/area-one: a bisect git has left un-concluded
    (abandon it with `git -C <ABS>/.jigc/worktrees/area-one bisect reset`) — … registered here,
    so the teardown removes it and this content is destroyed
    at: .jigc/worktrees/area-one
    route: … A path listed above as mid-operation holds no bytes to move: conclude or abandon
      the operation in that checkout — the command that abandons it is on its line
SPAN run VERBATIM:  from $REPO -> rc 0 · from docs/deep -> rc 0 · from W -> rc 0   (C1-06)
$ KEEPMD > .jigc/milestones/bb-probe/plant.txt ; w.txt staged in W
$ (cd W) jigc milestone discard bb-probe --force                               -> exit 0
  warning: removing the working area .jigc/milestones/bb-probe … plant.txt / not recoverable
  discarded milestone:bb-probe (1 sub-task(s); workbench removed)
  W gone YES · git worktree list -> 1 row · plant 0
```

### R-23 — `milestone discard` over dirty sub-task worktrees (row 69)

```
setup: fresh; "Foxtrot probe" + two sub-tasks; provision; both stage c.txt; join
$ (cd docs/deep) jigc milestone discard foxtrot-probe                          -> exit 1
  milestone.dirty-worktree — 2 sub-task worktree path(s) …
    .jigc/worktrees/area-one: A  c.txt — … registered here, so the teardown removes it and
      this content is destroyed
    .jigc/worktrees/area-two: A  c.txt — …
    at: .jigc/worktrees/area-one
```

### R-24 — the 5-locus foreign complement at TWO doors (rows 72, 83)

```
setup: as R-23, then `git worktree remove --force` both (so the dirty-worktree guard does not
  mask the foreign arm), then 5 plants:
  merged/x.txt · merged/docs/y.txt · mnote.txt · tasks/area-one/t1.txt · tasks/area-two/t2.txt
BEFORE 5
$ (cd docs/deep) jigc milestone discard foxtrot-probe                          -> exit 1
  milestone.foreign-bytes — its workbench holds 5 path(s) jigc did not write …
    .jigc/tasks/area-one/t1.txt · .jigc/tasks/area-two/t2.txt ·
    .jigc/milestones/foxtrot-probe/merged/docs/y.txt · …/merged/x.txt · …/mnote.txt
$ (cd docs/deep) jigc uninstall                                                -> exit 1
  uninstall.foreign-bytes — the SAME 5 paths, same order
AFTER 5
```

### R-25 — the terminal arms (rows 73, 76)

```
setup: fresh; "Tango probe" + one sub-task; provision; stage; join; finalize -> `joined`
$ (cd docs/deep) jigc milestone discard tango-probe                            -> exit 1
  milestone.terminal — milestone `tango-probe` is `joined` — a settled milestone is over and
    has no workbench     at: milestone:tango-probe
    route: read the settled record with `jigc doc show milestone-record:tango-probe`; new work
      starts a new milestone (`jigc milestone create "<title>"`)
$ jigc milestone create "Tango probe"                                          -> exit 1
  milestone.record-exists — … already has a record (its record reads `joined`) …
```

### R-26 — `milestone.zero-contribution` (row 75)

```
setup: fresh; "Mike probe" + one sub-task; provision; join (nothing staged anywhere)
$ (cd docs/deep) jigc milestone finalize mike-probe                            -> exit 3
  milestone.zero-contribution — … the boundary would commit only jigc's own bookkeeping and
    flip the milestone record to the terminal `joined`, after which the milestone could never
    be finalized again
    route: `jigc milestone provision mike-probe` … or settle the milestone as abandoned with
      `jigc milestone discard mike-probe`
```

### R-27 — F-3 still closed: `uninstall` from inside a fan-out worktree (rows 77, 84)

```
setup: fresh; milestone create "Uu probe" + one sub-task; provision
BEFORE: .jigc P · hook P · CLAUDE.md P · SKILL P · git worktree list 2 rows
$ (cd .jigc/worktrees/area-one) jigc uninstall                                 -> exit 0
  stderr  warning: removing `.jigc/` also removes 5 tracked file(s) under it:
            .jigc/.gitignore .jigc/AGENT.md .jigc/config/.gitkeep
            .jigc/config/packs.yaml .jigc/version
          note: each is in the index, so `git -C <ABS> checkout -- <path>` brings it back.
  stdout  jigc uninstall — repo-local install removed
            - removed .jigc/
            - pruned git's worktree registrations for the fan-out worktrees `.jigc/` held
            - unwired bootstrap reference ← CLAUDE.md
            - removed jigc allowlist permit / SessionStart hook / deny safety floor
            - removed pre-commit hook   - removed jigc guide artifact
          removed at `<ABS main>` — the main checkout this repository's jigc install and
          `.jigc/` workbench bind to, and that workbench held the worktree you are standing in,
          which this removed
AFTER (measured in the MAIN checkout): .jigc A · hook A · CLAUDE.md A · SKILL A
  git worktree list -> 1 row ; .git/worktrees/ does not exist        <- the prune RAN
```

### R-28 — `uninstall`'s four WIP guards, two cwds each (rows 78–81, 86)

```
setup per cell: fresh; milestone create "Gg probe" + one sub-task; provision; then the cell.
Each guard driven from the fan-out worktree AND from the repo root; the two outputs compared.

dirty      -> 1 uninstall.dirty-worktree ".jigc/worktrees/area-one: A  f.txt — it is a live git
              worktree holding uncommitted work, registered here or not; registered as a
              worktree of this repository"                            BYTE-IDENTICAL both cwds
untracked  -> 1 uninstall.untracked-workbench-file ".jigc/notes.txt"; route `git -C <ABS>
              add -- <path>` / `checkout -- <path>`                   BYTE-IDENTICAL
foreign    -> 1 uninstall.foreign-bytes ".jigc/tasks/hand-made/notes.txt"  BYTE-IDENTICAL
bisect     -> 1 uninstall.dirty-worktree "a bisect git has left un-concluded (abandon it with
              `git -C <ABS>/.jigc/worktrees/area-one bisect reset`) …" BYTE-IDENTICAL
install still present after each: YES
THE ROUTE PLACEHOLDER (OBS-E), on the dirty-worktree arm, both cwds:
  "… or abandon the milestone with `jigc milestone discard <milestone-id> --force` …"
```

### R-29 — `uninstall.staged-prose`, and idempotency (rows 82, 85)

```
setup: fresh; milestone create "Pp probe" + one sub-task; provision;
  (cd W) jigc workflow sub-task --task area-one   -> rc 0   (the door that PROVISIONS the doc;
         `jigc start --task area-one` alone does not, and my first attempt got the
         "no staged instance … enter it with `jigc workflow sub-task`" route — recorded)
  (cd W) jigc doc set-field commit:area-one#type --value chore --task area-one -> "set … = chore"
  area docs/ -> commit:area-one.md  provenance.json

$ (cd W) jigc uninstall                                                        -> exit 1
  uninstall.staged-prose — `.jigc/` holds 1 staged doc(s) for 1 open task(s) …
    area-one: commit:area-one — a sub-task of milestone `pp-probe`, which `jigc task finalize`
      refuses: land it with `jigc milestone finalize pp-probe`, or drop it with `jigc task
      discard area-one --force`, which also settles it as `discarded` in the committed record
  install still present YES
$ jigc uninstall   (on an already-clean repo)                                  -> exit 0
  "(nothing to remove — no repo-local jigc install was present)"
```

### R-30 — `task discard` over an AMEND task (rows 87–89)

```
setup: committed-singletons; jigc task amend "repair one"; type+summary authored
$ jigc task discard repair-one                                                 -> exit 1
  task-discard.staged-prose — task `repair-one` stages 1 doc(s) that no commit has a copy of
    — discarding it would destroy them: commit:repair-one
  <- `foreign-bytes` does NOT fire: `amend`, `base.json`, `intent`, `workflow`,
     `staged-snapshot.json` are all inside TASK_AREA_FILES (15 members)
$ KEEP-A2-BYTES > .jigc/tasks/repair-one/foreign.txt    BEFORE 1
$ jigc task discard repair-one                                                 -> exit 1
  task-discard.foreign-bytes — .jigc/tasks/repair-one/foreign.txt      AFTER 1
$ jigc task discard repair-one --force                                         -> exit 0
  warning: removing the working area … foreign.txt / not recoverable
  discarded task repair-one — dropped staged edits to: commit:repair-one (transient)
  AFTER 0 ; HEAD unmoved
```

### R-31 — `task discard` × four cwds (rows 90, 91)

```
setup per cwd: fresh; a linked worktree AND a provisioned fan-out worktree present;
  jigc start --workflow quick-fix "tidy the readme"; KEEPTD > the area/foreign.txt

cwd = root · docs/deep · .jigc/worktrees/area-one · the linked worktree:
  refusal  -> exit 1 each, task-discard.foreign-bytes, BYTE-IDENTICAL, host-abs hits 0
  --force  -> exit 0 each, BYTE-IDENTICAL warning + ack, host-abs hits 0, plant 0
```

### R-32 — F-1 re-driven, three surfaces (row 92)

```
setup: fresh; jigc start --workflow quick-fix "tidy the readme"
  O=$(mktemp -d); TARGET-BYTES > $O/body.md
  ln -s $O/body.md .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md

$ (cd docs/deep) jigc task discard tidy-the-readme                             -> exit 1
  task-discard.foreign-bytes — .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
                                                              <- A THIRD PARTY'S
$ (cd docs/deep) jigc doc list --task tidy-the-readme                          -> exit 0
  adr:via-symlink  docs/decisions/via-symlink.md  managed      <- JIGC'S OWN
$ (cd docs/deep) jigc task discard tidy-the-readme --force                     -> exit 0
  discarded task tidy-the-readme — dropped staged edits to: adr:via-symlink,
    commit:tidy-the-readme (transient)                         <- JIGC'S OWN, AGAIN
outside target 1/1 intact
```

### R-33 — F-2 re-driven at three doors (rows 93–95)

```
setup: fresh; mkdir -p .jigc/tasks ; O2=$(mktemp -d); OUTSIDE-KEEP > $O2/keep.txt
  ln -s $O2 .jigc/tasks/ghost     (ls -l -> "lrwxr-xr-x  ghost -> /private/tmp/…")

$ (cd docs/deep) jigc task discard ghost                                       -> exit 1
  finalize.no-task — no task `ghost`: `.jigc/tasks/ghost` IS A DIRECTORY carrying no base pin,
    so it is a leftover and not a work unit …
$ (cd docs/deep) jigc task validate ghost   -> exit 1  the BYTE-IDENTICAL sentence
$ (cd docs/deep) jigc task diff     ghost   -> exit 1  the BYTE-IDENTICAL sentence
outside target 1/1 intact ; the symlink still on disk
CONTROL, a real directory (.jigc/tasks/ghost-dir): the same sentence, TRUE there, with the
  by-hand route ("jigc mints no verb that clears a leftover working area").
```

### R-34 — F-5 re-driven, all three halves (rows 96, 97, 110, 107, 108)

```
setup: fresh; "Hotel probe" + two sub-tasks; provision; both stage; both enter their workflow;
  cp -R .jigc/tasks/area-two $SAVE ; join; finalize; cp -R $SAVE/area-two back
  (built by restoring a backup — no jigc sequence reaches it; the reachability half stays
   undischarged, exactly as rc.18 and rc.19 recorded)
  docs/milestone-records/hotel-probe.md -> "status: joined" (doc level AND the per-task item)

$ (cd docs/deep) jigc task list                                                -> exit 0
  jigc task list — 1 active task(s)      area-two  [sub-task]  area two
$ jigc task discard area-two                                                   -> exit 1
  task-discard.staged-prose … route: … `jigc task discard area-two --force` removes the
    working area with them
$ jigc task discard area-two --force                                           -> exit 1
  milestone.terminal — sub-task `area-two` of milestone `hotel-probe` is already `joined` on
    the committed record — its working area is a leftover, not live work, and discarding it
    would settle an item the record has already settled
  <- THE ROUTE ONE INVOCATION UP NAMES A COMMAND THAT REFUSES
HEAD unmoved.
(`milestone execute` and `milestone list-tasks` over a pin-less area both render, exit 0.)
```

### R-35 — A3-1: `milestone finalize`'s Displace over 8 loci (rows 98, 100)

```
setup: fresh; "Alpha probe" + one sub-task; provision; stage c.txt; join; 8 plants:
  mnotes.txt · merged/top.txt · merged/docs/{deep.txt,provenance.json,adr.md} ·
  merged/sub/inner.txt · sub/x.txt · .jigc/tasks/area-one/tnotes.txt
BEFORE: command grep -rl 'KEEP-m-' .jigc | wc -l  -> 8

$ (cd docs/deep) jigc --format json milestone finalize alpha-probe             -> exit 0
  committed.displaced = 8 pairs, sorted by `from`:
    .jigc/milestones/alpha-probe/merged/docs/adr.md          -> .jigc/displaced/…/adr.md
    …/merged/docs/deep.txt · …/merged/docs/provenance.json
    …/merged/sub            -> .jigc/displaced/alpha-probe/merged/sub      (WHOLE directory)
    …/merged/top.txt · …/mnotes.txt
    …/sub                   -> .jigc/displaced/alpha-probe/sub             (WHOLE directory)
    .jigc/tasks/area-one/tnotes.txt -> .jigc/displaced/area-one/tnotes.txt
  top-level keys: ['committed']            committed keys: ['commits','displaced','files',
    'hash','hook_output','manifest','still_staged','sub_tasks','subject']      <- no findings
AFTER 8 ; areas: milestones=[] tasks=[] ; host-absolute hits 0
```

### R-36 — A3-2 / D1×D2: the none-move cell at `milestone finalize` (rows 99, 100)

```
setup: as R-35 with 3 plants and  printf 'NOT A DIR' > .jigc/displaced
BEFORE 3
$ (cd docs/deep) jigc --format json milestone finalize beta-probe              -> exit 0
  committed.displaced []
  stderr, TWO notes (one per area) + TWO advisories:
    advisory · finalize.foreign-bytes  at: milestone:beta-probe   (2 paths, "Not a directory
      (os error 20)")
    advisory · finalize.foreign-bytes  at: task:area-one          (1 path)
    each route: "the commit landed and nothing in it is affected. Keep what you need from
      `<area>` and delete the rest — jigc mints no verb that clears it"
AFTER 3 ; areas STANDING: milestones=[beta-probe] tasks=[area-one] ; host-absolute hits 0
```

### R-37 — `milestone finalize`'s Displace from a linked worktree (row 101)

```
setup: fresh; a linked worktree on `feat`; "Uniform probe" + one sub-task; provision; stage;
  join; KEEPUN > .jigc/milestones/uniform-probe/mnote.txt       BEFORE 1
$ (cd linked) jigc --format json milestone finalize uniform-probe              -> exit 0
  displaced [{from ".jigc/milestones/uniform-probe/mnote.txt",
              to   ".jigc/displaced/uniform-probe/mnote.txt"}]
  stderr the same pair                       host-absolute hits 0      AFTER 1
```

### R-38 — census C2-06: the boundary from four cwds (rows 102, 106)

```
setup (one FRESH rig per cell): fresh + a linked worktree on `feat`; "Cc probe one" + one
  sub-task; provision; stage c.txt in the fan-out worktree; join

cwd = root          -> exit 0  hash 56aab4f  files 2  sub_tasks 1  displaced 0
cwd = docs/deep     -> exit 0  hash 695f4d3  files 2  sub_tasks 1  displaced 0
cwd = fan-out W     -> exit 0  hash abb4cf4  files 2  sub_tasks 1  displaced 0
cwd = linked        -> exit 0  hash acdd3a9  files 2  sub_tasks 1  displaced 0
  every cell: main HEAD ADVANCES on `main`; from `linked`, `feat` is UNMOVED (OBS-2);
  areas cleared (milestones=[] tasks=[]); host-absolute hits 0
`jigc milestone join` from docs/deep, the fan-out worktree and the linked worktree: exit 0 each.
```

### R-39 — F-4 re-driven (row 103)

```
setup: fresh; TRACKED-BYTES-AT-HEAD > keeper.md, committed (rc read bare);
  "F four" + one sub-task; provision; SUB > W/sub.txt staged; rm W/keeper.md (NOT staged)
BEFORE: git -C W status --porcelain -> " D keeper.md" / "A  sub.txt"
        git -C $REPO show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD
$ join ; (cd docs/deep) jigc milestone finalize f-four                         -> exit 0
  stdout  discarded with the fan-out worktrees (not committed, not recoverable):
            area-one: keeper.md (never staged)
  stderr  warning: removing the fan-out worktree … keeper.md (never staged)
          note: the fan-out worktree is the only copy of these bytes — they are not recoverable.
AFTER — is the claim true?
  git show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD      <- the bytes ARE in git
  cat $REPO/keeper.md     -> TRACKED-BYTES-AT-HEAD      <- and in the main checkout  F-4 STANDS
```

### R-40 — the `--ignored` narration, both streams (rows 104, 105)

```
setup: fresh; `build/` appended to .gitignore and COMMITTED BEFORE the pin (rc read bare);
  "Papa probe" + one sub-task; provision; W/first.txt staged, W/scratch.txt untracked,
  W/build/out.o ignored
  git -C W status --porcelain          -> "A  first.txt" / "?? scratch.txt"
  git -C W status --porcelain --ignored -> adds "!! build/"
$ join ; (cd docs/deep) jigc milestone finalize papa-probe                     -> exit 0
  stdout  finalized 3359267 — Finalize milestone papa-probe (1 sub-task)
            modified docs/milestone-records/papa-probe.md / added first.txt / 2 files committed
            sub-tasks: area-one: 1 code file
            discarded with the fan-out worktrees (not committed, not recoverable):
              area-one: build/ (ignored by git) · scratch.txt (never staged)
  stderr  the same two, one per line, + "note: the fan-out worktree is the only copy …"
```

### R-41 — orientation, `task list` and `also open:` with an amend task (rows 109, 111, 112)

```
setup: committed-singletons; jigc task amend "open amend"
$ jigc start                                                                   -> exit 0
  Pack: dev/1.0.0-rc.20 | methodology/1.0.0-rc.20 · Project config: <ABS>/.jigc/config
  Active task: open-amend
    workflow: amend      intent: open amend      base: 793e3cf
    staged:   commit:open-amend        findings: 2 blocking
  (the two conformance findings, each with its own route)
  Run: `jigc start --task open-amend`  /  `jigc task validate open-amend` — previews … the
    carryover gate …                                          <- OBS-D, the declared bound
  Run: `jigc task finalize open-amend`  /  `jigc task discard open-amend --force`
$ jigc start "something else"                                                  -> exit 0
  also open: 1 other task was already open before this call — nothing here touched it …
    - `open-amend` (workflow `amend`) — resume it with `jigc start --task open-amend`
$ jigc task list            -> 0   "1 active task(s)"  /  "open-amend  [amend]  open amend"
$ jigc --format json start   -> 0   keys ['header','next_steps','schema_version','state',
                                          'tasks','workflows']
```

### R-42 — `rename` over residuals (rows 113, 114)

```
setup: dev/jigc-rig refs-post-hoc ($RIG_TASK live);
  mkdir .jigc/tasks/ghost-residual .jigc/milestones/ghost-ms
$ (cd docs/deep) jigc rename research:context-loss --to renamed-probe          -> exit 1
  rename.in-flight — cannot rename while task `ground-the-vision-in-research` is in flight —
    finalize or discard it first (a rename changes the by-task-id join key)
    route: … `jigc task finalize …` if that work is done, `jigc task discard …` if it is not
$ jigc task discard $RIG_TASK --force ; re-run                                 -> exit 0
  renamed research:context-loss -> research:renamed-probe (docs/research/context-loss.md ->
    docs/research/renamed-probe.md), repointed 0 referrer(s)
residuals after: tasks=[ghost-residual] milestones=[ghost-ms]         <- untouched
```

---

## 5 · M52 / rc.19 §A rows: CLOSED / STILL-OPEN

The **rc.19** run is this axis's baseline; M52's §A rows for axis 3 were carried into it and are carried
through here. Every row re-driven on rc.20.

| baseline row | tier | rc.19 verdict | **rc.20 verdict** | evidence |
|---|---|---|---|---|
| **`(3, F-A)`** — `task finalize`'s Displace surfaces print host-absolute paths from a linked worktree, incl. on the 1.0-pinned envelope | 2/3 | CONFIRMED | **CLOSED** — both cells, both streams, `0` host-absolute hits; `committed.displaced[].from`/`.to` repo-relative; `finalize.foreign-bytes`'s `message` and `route` repo-relative; the reachability bound (a fan-out worktree refuses `repo.head-detached` first) unchanged | R-6 · R-8 |
| **`(3, F-B)`** — the `LeftoverShape::File` consent arm lists the leftover's own basename as a child entry | 3 | CONFIRMED | **STILL-OPEN (1.x, expected)** — `area-one` in the child-entry position, with the `Directory` control listing `precious.txt` in the same block | R-20 |
| **`(3, F-1)`** — a symlink wearing a staged identity | 3 | CONFIRMED | **STILL-OPEN (1.x, expected)** — all three surfaces re-driven | R-32 |
| **`(3, F-2)`** — the residual note asserts *"is a directory"* over a shape it did not check | 3 | CONFIRMED | **STILL-OPEN (1.x, expected)** — three doors, byte-identical sentence | R-33 |
| **`(3, F-4)`** — the teardown narrates a **deleted tracked** file as unrecoverable | 3 | CONFIRMED | **STILL-OPEN (1.x, expected)** | R-39 |
| **`(3, F-5)`** — a settled sub-task's leftover, both halves + the refusing route | 3 | CONFIRMED | **STILL-OPEN (1.x, expected), all three halves**; reachability still undischarged | R-34 |
| **`(3, F-3)`** — `uninstall` inside a fan-out worktree exits 0 and reports an install it did not remove | 2 | **CLOSED** | **still CLOSED** — the main install removed in full, git's admin pruned (2 → 1 rows, `.git/worktrees/` gone), the site line names the standing worktree's removal, all four WIP guards byte-identical from both cwds | R-27 · R-28 |
| **`(3, A3-1)`** (M52) — `milestone finalize` destroys every byte jigc did not write at exit 0 | **1** | CLOSED | **still CLOSED** — 8 loci, **8 before / 8 after**, 8 pairs sorted by `from`, foreign directories moved whole, both area roots empty | R-35 |
| **`(3, A3-2)`** (M52) — when the displacement fails, both `Displace` doors remove the area anyway | **1** | CLOSED | **still CLOSED at both doors**, and now at a **third** arm: the **amend** arm leaves the area standing with one keyed advisory (`task finalize` 1/1 · `milestone finalize` 3/3 · **amend** 1/1) | R-4 · R-36 |
| **`(3, A3-3)`** — the boundary's own area from inside a fan-out worktree | 3 | CLOSED behaviourally | **still CLOSED** — C2-06's fan-out cell lands, areas cleared, 0 host paths; the structural half remains a source read, unchanged disposition | R-38 |
| the rc.18 post-review **HIGH** — a *spotless* worktree carrying an un-concluded operation | — | holds | **holds at all three worktree doors** — `provision` → `leftover-holds-work`, `discard` → `dirty-worktree`, `uninstall` → `dirty-worktree`, each naming the operation and routing `git -C <ABS> bisect reset` | R-20 · R-22 · R-28 |
| the rc.18 post-review **MEDIUM 1** — 0 host paths at `milestone finalize` from a sibling worktree | — | holds (and F-A was its un-swept sibling) | **holds, and the sibling is now swept** — both doors 0 host paths in the identical cell | R-6 · R-37 |
| census **C2-06** · **C2-07** (= F-3) · **C1-06 / C1-14** | — | all CLOSED | **all still CLOSED** — the boundary lands from four cwds; the `git -C <ABS>` spans run verbatim from three cwds and under a spaced root | R-17 · R-22 · R-27 · R-38 |
| **OBS-1** — `uninstall.dirty-worktree`'s unfilled `<milestone-id>` placeholder | obs | open | **still open** (OBS-E) | R-28 |
| **OBS-2** — `milestone finalize` from a branch-attached linked worktree commits onto `main` silently | obs | matches contract | **unchanged** — `CommitSite::differing`'s own prescription | R-38 |
| **OBS-4** — `Disposition::Narrate` empty by construction | obs | open | **unchanged** (OBS-F) | — |

**Tier-1 rows on this axis after the re-drive: 0.** Stated for the exit rule, for **this axis only**.

---

## 6 · What I did NOT drive, and why

Stated as un-driven rather than presented as passing.

1. **`Disposition::Narrate`** — no `DESTROYING_DOORS` member holds it at rc.20 (OBS-F), so the arm has no
   cell. A gap in the shipped set, not in the drive.
2. **`jigc task finalize --force`** — the door has no `--force` on either commit model; the F-10 settle
   refuses one for the amend arm and Addendum 3 carries it as a declared bound. No cell enumerated.
3. **The remaining five `InProgress` members at the amend arm** — I drove `{merge, bisect, revert,
   unmerged-index, detached}` (5 of the shipped 9 + detached). The other members
   (`squash-merge`, `rebase-merge`, `rebase-apply`, `am`, `cherry-pick`, `sequencer`,
   `dangling-sequencer`, `uncommitted-pick*`, `unborn`) are **axis 2's** subject; the F-10 review reports
   8/8 refusing and I did not re-drive them here. Stated because the amend arm is *new* at that family.
4. **A genuine concurrent racer** at any destroying door, and at the amend arm's rollback. The doors were
   driven serially; nothing here discharges `design/storage.md` → *Concurrent writers*.
5. **`GIT_DIR` redirect** — M51's declared posture residual, out of this axis by its stated bound.
6. **`LeftoverVerdict::OwnWorktree` at `uninstall` and `task discard` specifically** — driven at
   `milestone provision` and `milestone discard` (R-20, R-22) and at `uninstall`'s dirty/bisect cells
   (R-28); the shared-classifier claim for the remaining door pair is a source fact I did not re-derive.
7. **F-5's reachability** — the state is built by restoring a `cp -R` backup, exactly as the rc.18 and
   rc.19 drivers did. Only its behaviour is driven.
8. **The F-10 acceptance suite's own arms** (`crates/cli/tests/task_amend.rs`) — this is a release-binary
   review; LOW-6 is a claim inside a test comment and is not a drivable user surface. Its behavioural
   half is re-verified in R-12.
9. **`jigc task amend` under a spaced root at the *mint* door's checkout clause** — the spaced root was
   driven at the index route (R-17), where bytes reach a shell. The mint ack's checkout clause carries a
   *declared* absolute and no runnable span, so the quoting question does not arise there; I did not
   build a spaced-root linked worktree to say so by measurement.
10. **`milestone add-from-spec` / `milestone list-tasks` / `task bind` / `relocate` / `unmanage` /
    `ingest` / `migrate` / `migrate-corpus` / `describe` / `upgrade` / `doc show` / `doc schema` /
    `config` (beyond `set`) / the `config insert-step` family** — not doors of any cell on this axis.
    Named so the coverage list is not read as a claim about them.

**Instrument faults of mine, recorded rather than dropped:**

- My first `AMBUSH_CONTRACTS` mutation ran under **`--pack-from-dev`**, which **drops the freeze
  manifest** — and `assert_named_facts_stated` is one of the four sibling fences that keep the
  *manifest-shipping* subject, so both mutants were silently green. Re-driven under **`--repin`**, where
  both redden (R-19). A green under the wrong pack posture would have filed MEDIUM-4 as still open.
- My first mutant split the file on blank lines and so removed the **front matter** with the paragraph
  (the `states-constraints:` line contains the code), which reddened the *staged-read-back* fence
  instead. Re-driven with the front matter preserved.
- Four early cells read `$?` **through a `| head`** pipeline, so the exit reported was `head`'s. Every
  one was re-driven **bare** before being tabled (rows 8, 9, 49, 48).
- One `uninstall.staged-prose` fixture used `jigc start --task area-one`, which does **not** provision a
  sub-task's `commit` doc (the binary said so: *"enter it with `jigc workflow sub-task --task area-one`"*).
  Re-driven through that door (R-29).
- `dev/jigc-rig committed-singletons --pack-from-dev` **cannot build**: the state's construction runs
  `jigc start --workflow planning`, a **methodology** workflow, and `--pack-from-dev` ships the dev pack
  alone (`workflow-refs.unknown-workflow`). The pack-mutation cells were built on `fresh` instead.
- Two `F-2`/`F-5` fixtures needed `mkdir -p .jigc/tasks` first on a `fresh` rig, which has no `tasks/`
  until a mint. The first attempt's `ln -s` failed and the door answered the *unknown-id* cell instead of
  the *residual* cell; re-driven.

---

## 7 · What this adds over flow-54 arms 1 · 2 · 3

Flow 54's arms 1–3 are this axis's in-repo acceptance
([acceptance-design.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/acceptance-design.md)):
arm 1 iterates `DESTROYING_DOORS` **through its `Disposition` axis** over a manufactured plant-shape set,
arm 2 iterates `{all move · some move · none move} × {unwind ok · fault on the pin · fault on a later
member} × {task area · sub-task areas · milestone area}`, and arm 3 iterates the residual rule over
`WORK_UNIT_ID_DOORS`. All three run `trial_corpus.rs` fixtures from the **repository root**, on the
**debug** binary.

This review adds five things they structurally cannot:

1. **The new commit model is a fourth `Displace` subject, and no arm carries it.** Arms 1 and 2 were
   designed and written before `jigc task amend` existed; `DESTROYING_DOORS` did not grow a member for
   it (the amend arm rides `TASK_FINALIZE_DOOR`), so nothing in the suite says *the amend arm displaces*.
   Rows 19, 20, 37 drive it: all-move, none-move, and **the hook-rejected cell with a plant present**,
   which is the cell where the Displace and the rollback interact and where the answer (*the plant stays
   in the area, un-displaced*) is not derivable from either arm.
2. **The cwd axis, which is in no arm's set.** Every `trial_corpus.rs` fixture builds and drives from the
   repo root, so `{root · subdirectory · fan-out worktree · linked worktree}` is a dimension the suites
   do not iterate — and it is the dimension rc.19's **F-A** lived in. Rows 39–42, 90–91, 101–102 close
   it at six doors, and the closure is only visible from the cwd the suites never stand in.
3. **The shell-survival axis.** Every suite root comes from `std::env::temp_dir()`, which has no space in
   it. R-17 drives a **spaced repository root** through the amend arm's brand-new
   `finalize.amend-index-dirty` route, with a **spaced filename** as the operand, and runs the emitted
   span verbatim. That is the first measurement that M51's `shell_token` rule reaches F-10's new code.
4. **Pack-load fences driven by mutation, in release posture.** `MEDIUM-4`'s closure is not assertable
   from inside the suite that ships the correct pack: the question is *what happens when the declared
   statement is deleted*. R-19 deletes each paragraph in a throwaway pack copy and reads the door's exit
   — and it also measured the fence's **subject** (manifest-shipping, not step-shipping), which is why
   the first attempt was green and the finding would have been mis-filed.
5. **The release posture itself.** The suites run the debug binary, where the route fences panic; every
   verdict here is about the bytes an adopter receives, where a bad span is emitted silently. Rows 26,
   42, 71 paste emitted spans into shells of a **different cwd** and read the exit status — the
   difference between *the route says the right thing* and *the route works*.

It also re-drives all five still-open tier-3 rows on the shipped binary, so their triage to 1.x rests on
a current measurement rather than on rc.19's, and it verifies each of F-10's eight review findings closed
**on the installed binary** rather than on the debug build the review used.

---

**Scope.** This is **axis 3 only**, on `1.0.0-rc.20` (repo HEAD `51e0b8e4`). The Codex source pass for
this axis was **not read** (reconciliation is a separate agent). No fix was applied, no commit made,
nothing written into the working repository.

---

# Reconciliation — AXIS 3 · destroying doors

<!-- The RECONCILER's section. Inputs: the driver table above (unchanged) and the Codex source pass
     `codex/axis3-codex.md`. Every Codex claim was entered as a lead and DRIVEN on the installed
     release binary before being graded; every driver DEFECT was re-driven once by this agent.
     No fix applied, no commit made, nothing written into the working repository. -->

**The binary, asserted before anything ran.**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.20
```

Source facts read at `HEAD = 51e0b8e4` (the same commit the driver counted from). Every drive in its own
`dev/jigc-rig` throwaway repo (two-step eval, every root from `mktemp -d`, no teardown, no `rm` on a
variable path). Exit codes read bare. Every loss/survival claim carries a `command grep` before-control
beside its after-count.

**Numbering.** The sections below restart at §0 under this heading; the driver's own §0–§7 above are
untouched, and a `§n` reference inside this half always means this half's `§n`. The driver's repro blocks
keep their `R-` ids; the reconciler's carry `RX-` ids, so no id means two things.

**The rule applied** (`acceptance-design.md` → The reconciliation rule): a claim by one that the other
cannot reproduce is a **lead**, not a finding. A Codex claim drives to a repro block (CONFIRMED) or to a
recorded refutation carrying the falsifying datum (REFUTED); one that cannot be driven at all stays an
**OPEN LEAD** with its reason, never promoted on the source read and never dropped.

---

## 0 · Demotions — none

Every one of the driver's **115** rows carries a repro reference, every referenced block **R-1 … R-42**
exists, and every block names the row that cites it (checked mechanically over the table, with ranges
expanded: `rows 22–25`, `rows 43–47, 51`, `rows 56–65`, …). Every block carries driven evidence — an exit
code and the asserted surface — including the eight that use a tabular `CELL … -> exit N` form rather
than a `$ ` prompt (`R-9`, `R-12`, `R-18`, `R-19`, `R-20`, `R-28`, `R-31`, `R-38`).

**Nothing is demoted.** One honest bound on the instrument, stated rather than left implicit: the driver's
repro blocks are *summarized* transcripts — a command, its bare exit code and the quoted surface — not raw
captured stdout/stderr. Every defect this agent re-drove reproduced, three of them byte-for-byte against
the quoted text (F-1, F-2, F-5), so the summarization is faithful where it was checked.

---

## 1 · Reconciliation ledger — the Codex source pass

The pass proposes **no new failing argv sequence**: it reports one still-open row, one closed rc.20 row,
five closed M51 rows, the F-10/M52 seam readings, and a removal-site census. Each is entered as
`lead(codex, …)` and graded on a drive.

| # | `lead(codex, …)` | verdict | the driven datum |
|---|---|---|---|
| **C-1** | *no destroying door is omitted from the registry; no removal of user/foreign bytes bypasses the guard/displacement seam; no new F-10 amend-specific bypass* | **CONFIRMED** | `DESTROYING_DOORS` read at `51e0b8e4` = **6**; the three refusing members driven over one foreign complement (**before 2 / after 2**), the two displacing members driven at RX-1/RX-6, `milestone provision` at RX-3. No drive reached a removal outside that seam — **RX-2** |
| **C-2** | rc.20 `(3, F-A)` **CLOSED** — `task finalize` displacement renders from `jigc_root`, zero host-absolute paths; *re-drive from a branch-attached linked worktree, both the all-move and a blocked destination, text and JSON* | **CONFIRMED CLOSED** — driven on Codex's own proposed argv | all-move: `displaced [{from ".jigc/tasks/fa-probe/notes.txt", to ".jigc/displaced/fa-probe/notes.txt"}]`, **0 host-absolute hits** over stdout+stderr; blocked destination: `displaced []`, area standing, plant 1/1, three path occurrences in `findings[0].message` **all repo-relative**, **0 host-absolute hits** — **RX-1** |
| **C-3** | rc.20 `(3, F-B)` **STILL-OPEN**, tier 3 — *place a plain file at `.jigc/worktrees/<sub-id>`, `milestone provision <ms> --force`, expect `<sub-id>` beneath the leftover-file heading* | **CONFIRMED** (agrees with driver row 61; independently predicted from source) | `warning: removing the leftover file .jigc/worktrees/area-one-work discards work that is not in git:` / `    area-one-work` — the path's own basename in the child-entry position; the `Directory` control in the same rig lists a real child (`precious.txt`). No loss — **RX-3** |
| **C-4** | M51 **C-1 CLOSED across all doors** — `task discard` · `milestone discard` · `uninstall` refuse over the complement unless `--force`; `task finalize` · `milestone finalize` displace; a failed displacement is never followed by recursive destruction | **CONFIRMED** | the three refusals driven in one rig, each naming the exact plant paths, **before 2 / after 2**; the failed-displacement cell (`.jigc/displaced` a regular file) leaves the area **standing** with the plant **1/1** and the commit landed — **RX-1 · RX-2** |
| **C-5** | M51 **D-1 CLOSED** — `uninstall` distinguishes milestone sub-tasks before constructing the staged-prose route | **CONFIRMED**, with a control | sub-task row: ``area-one: commit:area-one — a sub-task of milestone `delta-one`, which `jigc task finalize` refuses: land it with `jigc milestone finalize delta-one`, or drop it with `jigc task discard area-one --force` …``; the ordinary task in the same listing is bare: `plain-task: commit:plain-task` — **RX-4** |
| **C-6** | M51 **D-2 CLOSED** — the discard acknowledgement is outcome-keyed, not unconditional | **CONFIRMED** | with the area's parent made unwritable: `discarded milestone:echo-one (1 sub-task(s); workbench NOT fully removed — the warnings above name what is left)`, exit 0, and `.jigc/tasks/area-one` genuinely still on disk; stderr carries `note: post-commit sub-task working-area removal for 'area-one' failed (self-heals): Permission denied` — **RX-5** |
| **C-7** | M51 **D-3 CLOSED** — the staged-prose enumeration renders its unreadable `docs/` path through `repo_relative` (`task.rs:1259`) | **OPEN LEAD — the cited site is unreachable through any door I could build** | With `.jigc/tasks/<id>/docs` at mode `000`, all three doors fail closed **earlier**, at the foreign-bytes readability guard: `uninstall` → `uninstall.foreign-bytes — cannot check '.jigc/' …`; `task discard` → `task-discard.foreign-bytes — cannot check task 'plain-task's working area …`; `milestone discard` → `milestone.foreign-bytes — cannot check milestone:echo-one's workbench …`. The claim's **consequence** is corroborated (every one of those messages is repo-relative, **0 host-absolute hits**), but the enumeration itself was never entered, so the site is not confirmed. Reaching it needs the foreign scan to succeed while the docs read fails — a race, not a state a rig builds — **RX-6** |
| **C-8** | M51 **D-4 CLOSED** — the `uninstall` worktree-root failure is routed as a *readability* failure with the sibling `--force` consent, no longer diagnosed as missing `git` | **CONFIRMED**, verbatim | `.jigc/worktrees` at mode `000` → ``blocking · uninstall.dirty-worktree — cannot check `.jigc/worktrees/` for uncommitted fan-out work …`` with the route ``make sure `.jigc/worktrees/` is readable — this failure is a `read_dir` of that directory, not a git fault — … or … `jigc uninstall --force` deletes them with the install``; **0 host-absolute hits**; no mention of a missing git — **RX-6** |
| **C-9** | the F-10 seams — the `amend` marker is inside `TASK_AREA_FILES`; the amend arm refuses every staged index path (`finalize.amend-index-dirty`) and every staged promoting doc (`finalize.amend-staged-doc`); it stages nothing before `git commit --amend`; the review HIGH's worktree-divergence path is closed at both writer and backstop | **CONFIRMED**, four drives | area holds `amend base.json docs intent staged-snapshot.json workflow` and `task discard` answers `task-discard.staged-prose` — **not** `foreign-bytes` — so the marker is inside the writer set; `doc set-slot vision:vision#thesis --task` → exit 1 `finalize.amend-staged-doc`, `git status` **0 lines**; a staged `added.txt` → exit **3** `finalize.amend-index-dirty` with the `restore --staged` route and *"this arm takes no `--carry-staged`"*; the landing rewrites `61ce013 → 0b0cacd` with **TREE `76ceb188…` → `76ceb188…` byte-identical** and a clean worktree — **RX-7** |
| **C-10** | the M52 destroying-door seam is internally consistent — `Disposition::{Refuse,Narrate,Displace}` exhaustive (silence is not an arm), `WORKTREE_DOORS` explicitly four, `probe_leftover` `symlink_metadata`-based and fail-closed on an unreadable path | **CONFIRMED** — source read at `51e0b8e4` **plus** the fail-closed cells driven | `DESTROYING_DOORS: [&DestroyingDoor; 6]`, `WORKTREE_DOORS: [&DestroyingDoor; 4]` read by symbol; fail-closed driven at three doors: `milestone provision` → ``unknown — could not read the leftover directory …: Permission denied`` (driver row 64, re-driven shape at RX-6), `milestone discard` → ``unknown — Permission denied (os error 13) — nothing could be read there, so nothing can say those bytes are disposable``, `uninstall` → the readability refusal above — **RX-6** |
| **C-11** | the removal-site census — every production removal is a guarded/narrated/displaced door sink, a registry-keyed unwind, a transaction-owned temp artifact, a cache invalidation or a migration retirement with rollback capture; **no unguarded production removal of foreign/user bytes**; there is no `crates/cli/src/uninstall.rs` | **PARTLY CONFIRMED / OPEN LEAD on the exhaustive half** | `ls crates/cli/src/uninstall.rs` → *No such file or directory* (confirmed). The census itself is a **negative existential over all production paths**: no drive can establish it, and this agent found no counter-example in any cell it drove. Recorded as an open lead rather than promoted on the source read, per the rule |
| **C-12** | no schema or schema-manifest file changed in `834772b6..HEAD`; the M52 zero-schema-hash-movement boundary is not violated | **CONFIRMED** | `git diff --name-only 834772b6..HEAD -- '*schema*'` → **0** paths |

**Refuted Codex claims: none.** No claim in the source pass was contradicted by a drive.

---

## 2 · Reconciliation ledger — the driver's defects

Each re-driven once by this agent, in a fresh rig, on the same installed `1.0.0-rc.20`.

| id | tier | the source pass on it | verdict after the re-drive | the re-drive |
|---|---|---|---|---|
| **F-C** — `write.unslugable-title`'s route never names `jigc task amend`'s own sha fallback | 3 | **silent** | **CONFIRMED** — stays a finding | `jigc task amend ""` → exit 1, route ``re-run with a title carrying ASCII letters or digits — the task id is slugged from it``; same for `"   "`, `"###"`, `"的的的"`; `"the of and"` mints `and`. Then `jigc task amend` with **no** intent → exit **0**, `task minted: amend-1227894` (= `git rev-parse --short=7 HEAD`), and the door's own long help says so: *"Omitted, the task is named after the commit it rewrites (`amend-<sha7>`)"*. Sibling control: `jigc milestone create ""` → exit 1, the same route, and there **is** no fallback there — **RX-8** |
| **F-D** — the F-10 settle states `jigc task amend --format json` carries the pinned sha under `text`; it carries none | 3 | **silent** | **CONFIRMED**, both halves | `jigc --format json task amend "json probe"` → exit 0, keys `['task','text']`, `task: json-probe`, `'amending:' in text → False`, `'already been pushed' in text → False`, `\b[0-9a-f]{7,40}\b` over `text` → `set()`, over the **whole** envelope → `set()`, stderr empty. `f10-amend-settle.md:58` is **unstruck**, while lines **43** and **53** of the same table carry dated `2026-09-26` correction brackets from the same review pass. The second half also holds: the census row at `crates/cli/tests/text_json_parity_axis.rs` describes the subject as *"prose, pinned as {task, text}, **led by the `amending:` block** …"* and its `Disposition::DeclaredOut` beneath declares that block out of the arm the row is about — **RX-9** |
| **F-1** — a symlink wearing a staged identity is a third party's at the destroying probe and jigc's own at the read/ack surfaces | 3 | **silent** | **CONFIRMED**, all three surfaces, byte-for-byte | `task discard` → `task-discard.foreign-bytes … .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md`; `doc list --task` → `adr:via-symlink  docs/decisions/via-symlink.md  managed`; `--force` → `dropped staged edits to: adr:via-symlink, commit:tidy-the-readme (transient)`. Outside target **1/1** intact. A fourth half rides with it, driven: the `--force` warning says *"the working area is the only copy of these bytes — they are not recoverable"* over a symlink whose target survives — **RX-10** |
| **F-2** — the residual note asserts *"is a directory"* over a shape it never checked | 3 | **silent** | **CONFIRMED** at three doors, byte-identical | `.jigc/tasks/ghost` a **symlink** → `task discard ghost`, `task validate ghost`, `task diff ghost` each exit 1 with the identical ``finalize.no-task — no task `ghost`: `.jigc/tasks/ghost` is a directory carrying no base pin …``; the real-directory control (`ghost-dir`) prints the same sentence, true there. Outside target 1/1, the symlink still on disk — **RX-11** |
| **F-4** — the fan-out teardown narrates a **deleted tracked** file as *"never staged"* under *"the only copy … not recoverable"* | 3 | **silent** | **CONFIRMED** | `git -C W status --porcelain` → `" D keeper.md"`; after `milestone finalize f-four` (exit 0) stdout carries `area-one: keeper.md (never staged)` under ``discarded with the fan-out worktrees (not committed, not recoverable)`` and stderr the matching note — while `git show HEAD:keeper.md` → `TRACKED-BYTES-AT-HEAD` and the main checkout's `keeper.md` is untouched — **RX-12** |
| **F-5** — a settled sub-task's leftover is *"1 active task(s)"* at one door and *"a leftover, not live work"* at another, and a route names a command that refuses | 3 | **silent** | **CONFIRMED**, all three halves; reachability **still undischarged** | `task list` → `1 active task(s)` / `area-two  [sub-task]`; `task discard area-two` → `task-discard.staged-prose`, route ``… `jigc task discard area-two --force` removes the working area with them``; that route run verbatim → exit 1 `milestone.terminal — sub-task 'area-two' … is already 'joined' on the committed record — its working area is a leftover, not live work`. HEAD unmoved. Built the same way the driver built it (a `cp -R` of the area restored after the boundary): **no jigc sequence reaches this state**, so the reachability half stays open exactly as rc.18/rc.19/the driver recorded — **RX-13** |
| **F-B** — the `LeftoverShape::File` `--force` arm lists the leftover's own basename in the child-entry position | 3 | **CONFIRMS it** (independently, from source, with a proposed argv) | **CONFIRMED by both** — the one row on this axis where the driver and the source pass agree on an open defect | see **C-3** / **RX-3** |

**Driver defects refuted by the source pass: none.** The pass contradicts no driven row.

**Driver observations** OBS-A … OBS-G are carried unchanged; the source pass neither contradicts nor
addresses any of them, and C-10 above corroborates OBS-F's structural half (`Disposition::Narrate` holds
no `DESTROYING_DOORS` member at `51e0b8e4`, so that arm still has no drivable cell).

---

## 3 · Observations from the reconciliation drive itself (not findings, not promoted)

- **OBS-R1** — `jigc uninstall --force` over a `.jigc` subtree containing an unreadable directory exits
  **1** with `uninstall.remove-jigc` *after a partial teardown*: `.jigc/AGENT.md` and `.jigc/.gitignore`
  were already gone while `config/ index/ milestones/ state/ tasks/ version/ worktrees/` remained. The
  state is one this agent manufactured with `chmod 000`, the consent was given, and the failure carries a
  code and a route (``ensure `.jigc/` is writable, then re-run `jigc uninstall```) — so it is recorded,
  not filed. It is the failure-*point* axis, which the confidence-audit wave already treats as its own
  class.
- **OBS-R2** — at the C-6 cell the pre-teardown warning reads *"the workbench is the only copy of those
  bytes — they are not recoverable"*, the removal then **fails**, and the bytes are still on disk one
  line later, where the ack says so. Correct at print time, stale one line down; the same
  over-warning shape as **F-4** and the fourth half of **F-1**. Three instances of one shape across this
  axis — recorded here so the shape is countable, not as a fourth finding.
- **OBS-R3** — `jigc start ""` exits **0** (orientation) where `jigc task amend ""` and
  `jigc milestone create ""` refuse `write.unslugable-title`. Not a defect: the empty intent at `start`
  is the orientation arm, not a mint. Recorded because F-C's class argument reasons over `MINT_DOORS`'
  prose rows, and this is the one door of that family where an empty title is not an error at all.

---

## 4 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling. A verb that appears only
as fixture construction (`jigc setup`, `jigc milestone add-task`, `jigc doc schema`) is **not** counted,
because no row asserts its verdict — the driver's rule, kept.

`start` · `workflow` · `uninstall` · `rename` · `validate` · `config set` ·
`task amend` · `task finalize` · `task validate` · `task diff` · `task discard` · `task list` ·
`doc create` · `doc author` · `doc add-item` · `doc remove-item` · `doc retitle-item` · `doc rename` ·
`doc set-field` · `doc set-slot` · `doc list` ·
`milestone create` · `milestone provision` · `milestone execute` · `milestone join` ·
`milestone finalize` · `milestone discard` · `milestone list-tasks`

— **28 of the 48 `VERB_KINDS` leaves**, unchanged from the driver's §1.11: the reconciliation drive added
no new door (every leaf it touched was already the door of a driven row).

---

## 5 · The reconciler's repro blocks

All on `jigc 1.0.0-rc.20` (installed release), each in its own `dev/jigc-rig` throwaway repo.

### RX-1 — C-2 / C-4: `task finalize`'s Displace from a **branch-attached linked worktree**, both cells

```
setup: dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc
       git worktree add -b feat $LW      ->  git worktree list: repo [main] · $LW [feat]
       (cd $LW) jigc start --workflow quick-fix "fa probe" ; commit doc authored ; f.txt staged
       FOREIGN-KEEP > $REPO/.jigc/tasks/fa-probe/notes.txt        BEFORE 1

$ (cd $LW) jigc --format json task finalize fa-probe                           -> exit 0
  committed.displaced = [{"from": ".jigc/tasks/fa-probe/notes.txt",
                          "to":   ".jigc/displaced/fa-probe/notes.txt"}]
  stderr  note: … they were moved aside, not taken:
            .jigc/tasks/fa-probe/notes.txt → .jigc/displaced/fa-probe/notes.txt
  host-absolute hits over stdout+stderr (regex /Users/… | /var/folders/…)  -> 0
AFTER 1  (at .jigc/displaced/fa-probe/notes.txt)

THE BLOCKED-DESTINATION CELL, same cwd:   printf 'NOT A DIR' > $REPO/.jigc/displaced
  (cd $LW) jigc start --workflow quick-fix "fa blocked" ; authored ; g.txt staged
  FOREIGN-KEEP2 > $REPO/.jigc/tasks/fa-blocked/notes.txt
$ (cd $LW) jigc --format json task finalize fa-blocked                         -> exit 0
  top keys ['committed','findings','schema_version']   committed.displaced []  left_out []
  findings[0] advisory finalize.foreign-bytes
    key {"code":"finalize.foreign-bytes","target":"task:fa-blocked"}
    message  "`.jigc/tasks/fa-blocked` holds 1 path(s) jigc did not write … could not open
      .jigc/displaced/fa-blocked to park it: Not a directory"      <- all three paths RELATIVE
  stderr   … not one of them could be moved aside: .jigc/tasks/fa-blocked/notes.txt — …
  host-absolute hits -> 0 ; area STANDING (.jigc/tasks/ -> fa-blocked) ; plant 1/1
```

### RX-2 — C-1 / C-4: the three refusing destroying doors over one foreign complement

```
setup: fresh; milestone create "Charlie one" + one sub-task (unprovisioned);
       C1-KEEP > .jigc/tasks/area-one/t1.txt ; C1-KEEP > .jigc/milestones/charlie-one/mnote.txt
BEFORE  command grep -rl 'C1-KEEP' .jigc | wc -l  -> 2

$ (cd docs/deep) jigc milestone discard charlie-one                            -> exit 1
  milestone.foreign-bytes — 2 path(s) jigc did not write: both named; --force is the consent
$ (cd docs/deep) jigc uninstall                                                -> exit 1
  uninstall.foreign-bytes — the SAME 2 paths, same order
$ (cd docs/deep) jigc task discard area-one                                    -> exit 1
  task-discard.foreign-bytes — .jigc/tasks/area-one/t1.txt
AFTER 2                                             <- nothing removed at any of the three
```

### RX-3 — C-3 / F-B: `LeftoverShape::File` under `--force`, with the `Directory` control

```
setup: fresh; milestone create "Quebec probe"; add-task "area one work" (NOT provisioned)
       printf 'precious bytes' > .jigc/worktrees/area-one-work        <- a plain FILE

$ (cd docs/deep) jigc milestone provision quebec-probe                         -> exit 1
  milestone.leftover-holds-work — .jigc/worktrees/area-one-work: the file itself — it is a
    file, not a worktree, and nothing can say those bytes are disposable
$ (cd docs/deep) jigc milestone provision quebec-probe --force                 -> exit 0
  stdout  provisioned 1 worktree(s) for milestone:quebec-probe at base 051dd8c (area-one-work)
  stderr  warning: removing the leftover file .jigc/worktrees/area-one-work discards work that
            is not in git:
              area-one-work                       <- THE PATH'S OWN BASENAME   (F-B)
          note: the leftover file is the only copy of these bytes — they are not recoverable.

CONTROL, same rig, a second sub-task with a DIRECTORY leftover holding precious.txt:
  refuse -> "precious.txt — git reports no worktree of its own there"
  --force -> "warning: removing the leftover directory … :  precious.txt"   <- a REAL child
```

### RX-4 — C-5 / D-1: `uninstall`'s staged-prose route, sub-task vs ordinary task

```
setup: fresh; milestone create "Delta one" + one sub-task; provision;
       (cd .jigc/worktrees/area-one) jigc workflow sub-task --task area-one   (stages the doc)

$ jigc uninstall                                                               -> exit 1
  uninstall.staged-prose — …
    area-one: commit:area-one — a sub-task of milestone `delta-one`, which `jigc task finalize`
      refuses: land it with `jigc milestone finalize delta-one`, or drop it with
      `jigc task discard area-one --force`, which also settles it as `discarded` …
CONTROL: jigc start --workflow quick-fix "plain task" ; jigc uninstall         -> exit 1
    area-one: commit:area-one — a sub-task of milestone `delta-one`, …        <- distinguished
    plain-task: commit:plain-task                                             <- bare
```

### RX-5 — C-6 / D-2: the outcome-keyed discard ack

```
setup: fresh; milestone create "Echo one" + one sub-task; provision; sub-task workflow entered
       chmod 500 .jigc/tasks            <- the child area can no longer be removed

$ jigc milestone discard echo-one --force                                      -> exit 0
  stderr  note: post-commit sub-task working-area removal for `area-one` failed (self-heals):
            Permission denied (os error 13)
          warning: discarding milestone:echo-one discards the staged docs of 1 open task(s) …
  stdout  discarded milestone:echo-one (1 sub-task(s); workbench NOT fully removed — the
            warnings above name what is left)                     <- OUTCOME-KEYED
AFTER  ls .jigc/tasks -> area-one                                 <- and it really is left
```

### RX-6 — C-7 / C-8 / C-10: the unreadable-path cells at three doors

```
setup: fresh; milestone create "Echo one" + one sub-task; provision; workflow entered

chmod 000 .jigc/tasks/<id>/docs :
  $ jigc uninstall                 -> 1  uninstall.foreign-bytes  "cannot check `.jigc/` for
      files jigc did not write …: Permission denied (os error 13)"     host-abs hits 0
  $ jigc task discard plain-task   -> 1  task-discard.foreign-bytes "cannot check task
      `plain-task`'s working area …"   route names `.jigc/tasks/plain-task/`   host-abs 0
  $ jigc milestone discard echo-one-> 1  milestone.foreign-bytes  "cannot check
      milestone:echo-one's workbench …"  route names `.jigc/tasks/` + `.jigc/milestones/echo-one/`
  <- ALL THREE fail closed BEFORE the staged-prose enumeration D-3 cites; that site was never
     entered, which is why C-7 stays an OPEN LEAD rather than a confirmation.

chmod 000 .jigc/worktrees :
  $ jigc uninstall                 -> 1  uninstall.dirty-worktree  "cannot check
      `.jigc/worktrees/` for uncommitted fan-out work, so removing `.jigc/` could destroy it:
      Permission denied (os error 13)"
      route: "… this failure is a `read_dir` of that directory, NOT A GIT FAULT — then re-run
        `jigc uninstall`; or remove the worktrees yourself … — or … `jigc uninstall --force`"
  $ jigc milestone discard echo-one-> 1  milestone.dirty-worktree  ".jigc/worktrees/area-one:
      unknown — Permission denied (os error 13) — nothing could be read there, so nothing can
      say those bytes are disposable; registered here, so the teardown removes it …"
  host-absolute hits on both: 0 ; neither mentions a missing git
```

### RX-7 — C-9: the four F-10 seams

```
setup: committed-singletons; jigc task amend "repair one"
AREA: amend  base.json  docs  intent  staged-snapshot.json  workflow

$ jigc task discard repair-one                                                 -> exit 1
  task-discard.staged-prose — … commit:repair-one          <- NOT foreign-bytes: the `amend`
                                                              marker is inside the writer set
$ printf 'a new thesis' | jigc doc set-slot vision:vision#thesis --from-file - \
    --task repair-one                                                          -> exit 1
  finalize.amend-staged-doc — `vision:vision` … promotes to `VISION.md` — but this task's
    commit model is an amend, which changes no tree …      at: VISION.md
  git status --porcelain -> 0 lines
$ (type=docs, summary authored)  printf new > added.txt ; git add added.txt
  jigc task finalize repair-one                                                -> exit 3
  finalize.amend-index-dirty — `added.txt` is staged … route: unstage it (`git -C <ABS>
    restore --staged -- added.txt`) … this arm takes no `--carry-staged`
$ (unstaged)  jigc task finalize repair-one                                    -> exit 0
  amended 61ce013 → 0b0cacd — docs: repair the release note
  TREE before 76ceb18831f8650dc4c24aeb676cae5131f2f5e2
  TREE after  76ceb18831f8650dc4c24aeb676cae5131f2f5e2    <- BYTE-IDENTICAL
  git status --porcelain -> 0 lines ; .jigc/tasks/ empty  <- nothing staged, teardown ran
```

### RX-8 — F-C re-driven: the route names one exit and hides the other

```
setup: committed-singletons

$ jigc task amend ""                                                           -> exit 1
  blocking · write.unslugable-title — cannot mint a task: its id is slugged from the title, and
    this title slugs to nothing — ids are built from ASCII letters and digits …
    at: task
    route: re-run with a title carrying ASCII letters or digits — the task id is slugged from it
  same for "   " / "###" / "的的的"  (exit 1, byte-identical) ; "the of and" -> exit 0, mints `and`
  after each refusal: jigc task list -> "no active tasks"                 <- nothing minted

$ git rev-parse --short=7 HEAD  -> 1227894
$ jigc task amend                (no intent)                                   -> exit 0
  task minted: amend-1227894                                <- THE EXIT THE ROUTE NEVER NAMES
$ jigc task amend --help | head
  "[INTENT] … Omitted, the task is named after the commit it rewrites (`amend-<sha7>`)"

SIBLING CONTROL:
$ jigc milestone create ""                                                     -> exit 1
  the same code, route "… the milestone id is slugged from it" — CORRECT there: no fallback
```

### RX-9 — F-D re-driven: no sha anywhere in the json mint envelope

```
setup: committed-singletons
$ jigc --format json task amend "json probe"                                   -> exit 0
  keys: ['task', 'text']            task: json-probe
  'amending:' in text            -> False
  'already been pushed' in text  -> False
  \b[0-9a-f]{7,40}\b over text            -> set()
  \b[0-9a-f]{7,40}\b over the WHOLE envelope -> set()
  stderr: (empty)                                       text length 4188 chars

THE RECORD SIDE:
  completions/artifacts/M53/f10-amend-settle.md:58  — "`--format json` carries the pinned sha
    under `text`, no new key needed — verify"            <- UNSTRUCK, no bracket
  the same table, line 43 and line 53                    <- both carry dated 2026-09-26
                                                            correction brackets (LOW-7)
  crates/cli/tests/text_json_parity_axis.rs  (&["task","amend"]) Tier::Judgment(
    subject "… prose, pinned as {task, text}, led by the `amending:` block …",
    Disposition::DeclaredOut("… The `amending:` block is one more of those …"))
                                                         <- the row's two halves describe
                                                            different surfaces
```

### RX-10 — F-1 re-driven, three surfaces

```
setup: fresh; jigc start --workflow quick-fix "tidy the readme"
  O=$(mktemp -d); TARGET-BYTES > $O/body.md
  ln -s $O/body.md .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md

$ (cd docs/deep) jigc task discard tidy-the-readme                             -> exit 1
  task-discard.foreign-bytes — task `tidy-the-readme`'s working area holds 1 path(s) jigc did
    not write …  .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md     <- A THIRD PARTY'S
$ (cd docs/deep) jigc doc list --task tidy-the-readme                          -> exit 0
  adr:via-symlink  docs/decisions/via-symlink.md  managed                <- JIGC'S OWN
$ (cd docs/deep) jigc task discard tidy-the-readme --force                     -> exit 0
  discarded task tidy-the-readme — dropped staged edits to: adr:via-symlink,
    commit:tidy-the-readme (transient)                                   <- JIGC'S OWN, AGAIN
  stderr: "… the working area is the only copy of these bytes — they are not recoverable."
outside target: 1/1 intact, `cat $O/body.md` -> TARGET-BYTES        <- and the copy survives
```

### RX-11 — F-2 re-driven at three doors, with the control

```
setup: fresh; O2=$(mktemp -d); OUTSIDE-KEEP > $O2/keep.txt; ln -s $O2 .jigc/tasks/ghost
  ls -l -> lrwxr-xr-x  ghost -> /var/folders/…/outside2.JpAmc8        <- A SYMLINK

$ (cd docs/deep) jigc task discard  ghost   -> exit 1
$ (cd docs/deep) jigc task validate ghost   -> exit 1
$ (cd docs/deep) jigc task diff     ghost   -> exit 1
  all three, BYTE-IDENTICAL:
  blocking · finalize.no-task — no task `ghost`: `.jigc/tasks/ghost` IS A DIRECTORY carrying no
    base pin, so it is a leftover and not a work unit …   at: task:ghost
CONTROL .jigc/tasks/ghost-dir (a real directory): the same sentence, TRUE there
outside target 1/1 ; the symlink still on disk
```

### RX-12 — F-4 re-driven

```
setup: fresh; TRACKED-BYTES-AT-HEAD > keeper.md, committed (rc read bare);
  milestone create "F four" + one sub-task; provision; SUB > W/sub.txt staged; rm W/keeper.md
BEFORE  git -C W status --porcelain -> " D keeper.md" / "A  sub.txt"
        git -C $REPO show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD

$ jigc milestone join f-four -> 0 ; (cd docs/deep) jigc milestone finalize f-four -> exit 0
  stdout  discarded with the fan-out worktrees (not committed, not recoverable):
            area-one: keeper.md (never staged)
  stderr  warning: removing the fan-out worktree … keeper.md (never staged)
          note: the fan-out worktree is the only copy of these bytes — they are not recoverable.
AFTER   git show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD     <- the bytes ARE in git
        cat $REPO/keeper.md     -> TRACKED-BYTES-AT-HEAD     <- and in the main checkout
```

### RX-13 — F-5 re-driven, all three halves

```
setup: fresh; milestone create "Hotel probe" + two sub-tasks; provision; each worktree stages a
  DISTINCT file and enters `jigc workflow sub-task --task <id>`; each commit doc authored;
  cp -R .jigc/tasks/area-two $SAVE ; join (2 docs merged) ; finalize (5d63fe9) ;
  cp -R $SAVE/area-two back                    <- NO jigc sequence reaches this state
  docs/milestone-records/hotel-probe.md -> "status: joined" at doc level AND at the item

$ jigc task list                                                               -> exit 0
  jigc task list — 1 active task(s)        area-two  [sub-task]  area two
$ jigc task discard area-two                                                   -> exit 1
  task-discard.staged-prose — task `area-two` stages 1 doc(s) … commit:area-two
    route: … `jigc task discard area-two --force` removes the working area with them
$ jigc task discard area-two --force            (the route, run verbatim)      -> exit 1
  milestone.terminal — sub-task `area-two` of milestone `hotel-probe` is already `joined` on
    the committed record — its working area is a leftover, not live work …
HEAD before 5d63fe9 · HEAD after 5d63fe9                                       <- unmoved
```

---

## 6 · Reconciled totals

| | count |
|---|---|
| driver rows carried, unchanged | **115** over **28** doors |
| rows demoted | **0** |
| Codex claims entered as leads | **12** |
| → CONFIRMED on a drive | **10** (one of them, C-11, only in its checkable half) |
| → REFUTED | **0** |
| → OPEN LEAD | **2** (C-7 the unreachable enumeration site · C-11's negative-existential census half) |
| driver defects re-driven by the reconciler | **7** — F-C · F-D · F-1 · F-2 · F-4 · F-5 · F-B |
| → CONFIRMED (repro reproduced) | **7** |
| → refuted by the source pass | **0** |
| tier-1 rows on this axis | **0** |
| findings standing after reconciliation | **7**, all **tier 3** — two new (F-C, F-D), five still-open from rc.19 (F-1, F-2, F-4, F-5, F-B) |

**The one agreement worth naming:** F-B is the single row where a driven cell and an independent source
read arrived at the same open defect from opposite directions — the driver from the `--force` narration,
Codex from `milestone.rs:6986`'s `DoomedLine` construction over the leftover file's own basename. Every
other finding on this axis was reached by driving alone, and every other Codex reading was a closure the
drive confirmed.
