<!-- M53 THIRD PARTIAL per-axis review — axis 3 — the reconciled file, copied verbatim. Driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.19` (repo HEAD `a8904637`), 2026-09-23. -->

<!-- M53 THIRD partial per-axis review · AXIS 3 · destroying doors · the OPUS DRIVER.
     Every row driven on the installed /Users/maurice/.local/bin/jigc -> `jigc 1.0.0-rc.19`, 2026-09-23.
     No fix applied, no commit made, nothing written into the working repository. -->

# M53 third partial per-axis review — AXIS 3 · destroying doors — RECONCILED

> **Reconciler's banner.** §§1–6 below are the **Opus driver's table, unchanged except four marked
> demotions** (rows 12, 30, 47, 48 — each carried a verdict its cited repro block did not evidence; all four
> were re-driven by the reconciler and their new blocks are R-23 … R-25). **§7 is the reconciliation ledger**
> against the Codex source pass (`codex/axis3-codex.md`), **§8 the doors-covered list**. Every Codex claim was
> entered as a lead and **driven**; every driver defect the source pass is silent on was **re-driven once** by
> the reconciler before being carried. Reconciler runs: `jigc 1.0.0-rc.19`, 2026-09-23, each in its own
> `dev/jigc-rig` throwaway repo (root from `mktemp -d`; no teardown, no `rm` on a variable path); exit codes
> read **bare**, never through a pipe. No fix applied, no commit made, nothing written into the working repo.

**The binary, asserted first, before anything else ran.**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.19
```

**Baseline.** `completions/artifacts/M53/per-axis-review-rc18/axis-3.md` (the rc.18 reconciled table, 75 rows
over 15 doors) + its README §A. **What changed under it:** the cwd-dependence arc
([cwd-census.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/cwd-census.md),
VERDICT.md → Addendum 2, the four 2026-09-23 `DECISIONS.md` entries, the two reviews at
`completions/artifacts/M53/audit/cwd-fix-code-review*.md`).

**Door set, derived from the code, not from the design doc's numbers.** `crates/cli/src/milestone.rs:3430`
`DESTROYING_DOORS: [&DestroyingDoor; 6]` — **six** members (`PROVISION_DOOR`, `DISCARD_DOOR`,
`UNINSTALL_DOOR`, `TASK_DISCARD_DOOR`, `TASK_FINALIZE_DOOR`, `FINALIZE_DOOR`), with
`WORKTREE_DOORS: [&DestroyingDoor; 4]` its worktree-shaped subset (`milestone.rs:3444`). The cell axis is
`Disposition` (`milestone.rs:3196`, three variants — `Refuse{consent}` · `Narrate` (empty by construction) ·
`Displace`) × `LeftoverShape` (`milestone.rs:3491`, three — `Directory` · `File` · `Unreadable(String)`) ×
`LeftoverVerdict` (`LEFTOVER_VERDICTS`, three). **This review adds the cwd axis**: repo root · an ordinary
subdirectory (`docs/deep`) · inside a provisioned fan-out worktree · an ordinary branch-attached linked
worktree — and, for the shell-survival question the arc opened, a repository root containing a **space**.

**Method.** Every row ran on the installed release binary in a throwaway `dev/jigc-rig` repo (two-step eval;
every root from `mktemp -d`, so nothing needed teardown and the `rm -rf $V/$D` shape appears nowhere). Exit
codes read **bare**. Every loss/survival claim carries a `command grep` before-control beside its after-count,
because this harness's `grep` honours `.gitignore` and the whole axis lives under a gitignored `.jigc/`.

**Headline.**

- **Tier-1 rows on this axis: 0.** Every destroying cell reached a safe outcome with a before-control on
  every cwd. No plant died at exit 0 anywhere but where `--force` consented and the door said so first.
- **The rc.18 tier-2 row F-3 is CLOSED** — `jigc uninstall` from inside a fan-out worktree now acts on the
  main checkout, all four WIP guards fire from there, the install really is removed, git's worktree admin is
  **pruned**, and the site line names the removal of the worktree the caller is standing in.
- **Census C2-06 is CLOSED** — `jigc milestone finalize` now lands from inside a fan-out worktree, from a
  linked worktree and from a subdirectory, exit 0, no host path on any surface.
- **One new DEFECT (tier 2/3, not tier 1): F-A** — `jigc task finalize`'s **Displace** surfaces render every
  path host-**absolute** when the caller stands in a branch-attached linked worktree: the stderr displacement
  note, the **1.0-pinned** `committed.displaced[].from`/`.to` keys, and `finalize.foreign-bytes`'s `message`
  and `route`. Its sibling `milestone finalize` is clean in the identical cell. This is the same law-1 class
  the rc.18 post-review MEDIUM 1 closed at `milestone finalize`, un-swept at the task door.
- **Four rc.18 tier-3 rows are STILL-OPEN, as expected** (triaged to 1.x): F-1, F-2, F-4, F-5.
- **One new tier-3 LOW: F-B** — the `LeftoverShape::File` consent arm lists the leftover's own basename in
  the child-entry position, which is exactly the read the refusal arm's *"the file itself"* wording exists to
  prevent.

---

## 1 · The `(door, cell)` table

Route kinds: **M** = Mechanical (`jigc`-leading argv) · **H** = Human · **I** = Informational · **—** = none.
Every row's repro block is named in the last column; `§3` numbers them R-1 … R-20.

### 1.1 · `milestone provision` — `Disposition::Refuse{--force}` (17 rows)

| # | cell | cwd | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `Directory` holding a plant, no consent | docs/deep | `jigc milestone provision delta-probe` | 1 | `milestone.leftover-holds-work` | M | `… precious.txt — git reports no worktree of its own there…`; `at: .jigc/worktrees/area-one`; plant 1/1 after | matches contract | R-9 |
| 2 | the same, `--force` | docs/deep | `… --force` | 0 | none | I | `warning: removing the leftover directory … precious.txt` + *not recoverable*; then `provisioned 1 worktree(s)` | matches contract (consent) | R-9 |
| 3 | `Directory` with a **dangling `.git`** | docs/deep | `jigc milestone provision delta-probe` | 1 | same | M | `… .git, wip.txt — git cannot read a repository there` | matches contract | R-9 |
| 4 | `File` (a plain regular file at the worktree path) | docs/deep · root | same | 1 | same | M | `… the file itself — it is a file, not a worktree` | matches contract | R-9 · R-19 |
| 5 | the same, `--force` | root | `… --force` | 0 | none | I | `warning: removing the leftover **file** …` — **and the entry list names `area-one`, the path's own basename** | **DEFECT F-B** | R-19 |
| 6 | `File` = a **symlink** to an outside tree | docs/deep | `jigc milestone provision delta-probe` | 1 | same | M | identical *the file itself* wording | matches contract | R-9 |
| 7 | the same, `--force` | docs/deep | `… --force` | 0 | none | I | link removed; **outside tree 1/1 intact** | matches contract | R-9 |
| 8 | `Unreadable` (`chmod 000`) | docs/deep | `jigc milestone provision delta-probe` | 1 | same | M | `… unknown — could not read the leftover directory …: Permission denied (os error 13)` — fail-closed | matches contract | R-9 · R-16 |
| 9 | **S19** the leftover path is a **second repository's** linked worktree, mid-bisect, tree spotless | docs/deep | same | 1 | same | M | `a bisect git has left un-concluded (abandon it with \`git -C <ABS> bisect reset\`)`; no *registered here* clause; other repo's worktree + `keep.txt` intact | matches contract | R-12 |
| 10 | **S19** the same, `--force` | docs/deep | `… --force` | 0 | none | I | `warning: removing the fan-out worktree …` / *a bisect git had left un-concluded* / *not recoverable*; `$OTHER`'s objects + its own checkout intact | matches contract | R-12 |
| 11 | a **registered** worktree carrying a live bisect (idempotent re-provision) | root | `jigc milestone provision cc-probe-one` | 0 | none | — | `provisioned …`; `BISECT_LOG` and the worktree **untouched** | matches contract | R-4 |
| 12 | malformed / empty id | docs/deep | `jigc milestone provision ""` · `"../.."` | 1 | `work-unit.malformed-id` | H | `.jigc/` intact | **DEMOTED as filed** (no repro at this door) → **re-driven by the reconciler, matches contract** | ~~R-3 · R-14~~ → **R-23** |
| 13 | **cwd: root** — the `Directory` refusal | root | `jigc milestone provision cc-probe-two` | 1 | `milestone.leftover-holds-work` | M | byte-identical to row 1 | matches contract | R-3 |
| 14 | **cwd: inside a fan-out worktree** — an operation-bearing sibling | that worktree | `jigc milestone provision cc-probe-one` | 0 | none | — | idempotent, nothing probed or cleared | matches contract | R-4 |
| 15 | **cwd: an ordinary linked worktree** | linked | `jigc milestone provision juliet-probe` | 0 | none | — | `provisioned 1 worktree(s) … at base <sha>` against the **home**, not the standing checkout | matches contract | R-15 |
| 16 | **spaced repo root** — the operation route | docs/deep (spaced) | `jigc milestone discard`/`provision` refusals | 1 | — | M/H | the emitted `git -C '<abs with space>' bisect reset` is **single-quoted** and runs verbatim (exit 0) | matches contract — HIGH-1 class closed | R-6 |
| 17 | **control** — no leftover at all | root | `jigc milestone provision <id>` | 0 | none | — | `provisioned N worktree(s)`, no warning | matches contract | R-1 |

### 1.2 · `milestone discard` — `Disposition::Refuse{--force}` (13 rows)

| # | cell | cwd | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 18 | dirty sub-task worktrees | root · docs/deep · inside W1 | `jigc milestone discard cc-probe-one` | 1 | `milestone.dirty-worktree` | H | both worktrees named with `git status` codes + *registered here, so the teardown removes it*; `at: .jigc/worktrees/area-one`; **byte-identical from all three cwds** | matches contract | R-3 |
| 19 | the same, `--force`, run **from inside the worktree it removes** | inside W1 | `… --force` | 0 | none | I | `warning: removing the working area … plant.txt` + *not recoverable*; then `discarded … (workbench removed)`; `git worktree list` back to one row | matches contract (M52's declared behaviour) | R-3 |
| 20 | **spotless** worktree carrying a live **bisect** | root · docs/deep | `jigc milestone discard cc-probe-one` | 1 | `milestone.dirty-worktree` | H | `a bisect git has left un-concluded (abandon it with \`git -C <ABS> bisect reset\`)`; worktree + `BISECT_LOG` on disk after | matches contract | R-5 |
| 21 | the route **run verbatim** from three cwds | root · docs/deep · inside W1 | the emitted `git -C <ABS> bisect reset` | 0 · 0 · 0 | — | — | `HEAD is now at <sha>` each time — the census's C1-06 dead end is gone | **CLOSED** | R-5 |
| 22 | `milestone.foreign-bytes` — the whole complement (5 loci incl. `merged/`, `merged/docs/`, two task areas) | docs/deep | `jigc milestone discard foxtrot-probe` | 1 | `milestone.foreign-bytes` | H | **all 5** named; before 5 / after 5 | matches contract | R-13 |
| 23 | **terminal** milestone | docs/deep | `jigc milestone discard tango-probe` | 1 | `milestone.terminal` | H | ``is `joined` — a settled milestone is over and has no workbench``; route names `doc show milestone-record:…` | matches contract | R-13 |
| 24 | **cwd: linked worktree**, dirty fan-out worktree | linked | `jigc milestone discard juliet-probe` | 1 | `milestone.dirty-worktree` | H | the home's worktree named, repo-relative; 0 host paths | matches contract | R-15 |
| 25 | the same, `--force` | linked | `… --force` | 0 | none | I | `discarded … (1 sub-task(s); workbench removed)`; fan-out gone, the **linked** worktree untouched | matches contract | R-15 |
| 26 | `--force` loss narration, `--ignored` axis (ignore rule committed **after** the pin) | docs/deep (spaced root) | `… --force` | 0 | none | I | `build/out.o (never staged)` · `dirt.txt (never staged)` — matches what git reports in that worktree | matches contract | R-7 |
| 27 | `--force` over a plant in `.jigc/milestones/<id>/`, from a linked worktree | linked | `… --force` | 0 | none | I | `warning: removing the working area .jigc/milestones/sierra-two …` — **repo-relative**, 0 host paths | matches contract | R-18 |
| 28 | the same refusal arm, from a linked worktree | linked | `jigc milestone discard sierra-two` | 1 | `milestone.foreign-bytes` | H | `.jigc/milestones/sierra-two/plant.txt` — repo-relative, 0 host paths | matches contract | R-18 |
| 29 | **control** — clean and concluded | root | `jigc milestone discard <id>` | 0 | none | — | `discarded … workbench removed`, zero false fire | matches contract | R-1 |
| 30 | malformed / empty id (sibling of row 12) | docs/deep | `jigc milestone discard ""` · `"../.."` | 1 | `work-unit.malformed-id` | H | `.jigc/` intact | **DEMOTED as filed** (no repro at this door) → **re-driven by the reconciler, matches contract** | ~~R-14~~ → **R-23** |

### 1.3 · `uninstall` — `Disposition::Refuse{--force}` (18 rows)

| # | cell | cwd | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 31 | clean install, a clean fan-out worktree present | **docs/deep** | `jigc uninstall` | 0 | none | — | 8 removal lines incl. **`pruned git's worktree registrations …`**; **no** site line (the two roots are one) | matches contract | R-1 |
| 32 | the same | **inside the fan-out worktree** | `jigc uninstall` | **0** | none | — | the **main** install removed (`.jigc/`, `CLAUDE.md`, `SKILL.md`, hook all gone); site line: ``removed at `<ABS main>` — … and that workbench held the worktree you are standing in, which this removed`` | **rc.18 F-3 CLOSED** | R-2 |
| 33 | the same | **an ordinary linked worktree** | `jigc uninstall` | 0 | none | — | site line's **other** branch: *"…, not the worktree you are standing in"*; the linked worktree survives, the fan-out one does not | matches contract | R-2 |
| 34 | **dirty** fan-out worktree | inside that worktree · root | `jigc uninstall` | 1 | `uninstall.dirty-worktree` | H | `.jigc/worktrees/area-one: A  f.txt …`; **byte-identical from both cwds** — the guard is no longer inert from a worktree | matches contract (review LOW 10 CLOSED) | R-2 |
| 35 | **untracked workbench file** at the home | inside the fan-out worktree | `jigc uninstall` | 1 | `uninstall.untracked-workbench-file` | H | `.jigc/notes.txt`; route `git -C <ABS> add -- <path>` / `git -C <ABS> checkout -- <path>` | matches contract | R-2 |
| 36 | **foreign bytes** in the home's task area | inside the fan-out worktree | `jigc uninstall` | 1 | `uninstall.foreign-bytes` | H | `.jigc/tasks/hand-made/notes.txt`; plant 1/1 after | matches contract | R-2 |
| 37 | **staged prose** for an open sub-task | inside the fan-out worktree (spaced root) | `jigc uninstall` | 1 | `uninstall.staged-prose` | H | `area-one: commit:area-one — a sub-task of milestone \`space-probe\`…`; names the real milestone | matches contract | R-6 |
| 38 | an un-concluded **bisect** in the fan-out worktree | root · docs/deep · that worktree | `jigc uninstall` | 1 | `uninstall.dirty-worktree` | H | `git -C <ABS> bisect reset` on the hold line, identical from all three | matches contract | R-2 |
| 39 | the `git -C <ABS> add -- <path>` route **run verbatim** | docs/deep | the emitted span, `<path>` filled | 0 | — | — | `staged: .jigc/notes.txt`; the re-run then proceeds to exit 0 | **CLOSED** | R-5 |
| 40 | the same under a **spaced** repo root | docs/deep (spaced) | the emitted span | 0 | — | — | the span is `git -C '/…/jigc space.…/repo' add -- .jigc/notes.txt` — **single-quoted**, runs verbatim | matches contract | R-17 |
| 41 | `--force` over a dirty worktree **and** an untracked workbench file | inside the fan-out worktree | `jigc uninstall --force` | 0 | none | I | *two* warnings — the untracked file's *"nothing has a copy … not recoverable"* **and** the tracked-file note; worktree removed, admin **pruned**, site line present | matches contract (consent) | R-11 |
| 42 | the prune line's honesty — **no worktree ever provisioned** | root | `jigc uninstall` | 0 | none | — | the prune line is **absent** | matches contract (law 1) | R-10 |
| 43 | the prune line's honesty — worktrees already torn down | root | `jigc uninstall` | 0 | none | — | the prune line is **absent** — measured, not asserted | matches contract | R-10 |
| 44 | `.jigc/` holds the whole 5-locus foreign complement | docs/deep | `jigc uninstall` | 1 | `uninstall.foreign-bytes` | H | all 5 named, identical set to row 22 | matches contract | R-13 |
| 45 | **cwd: linked worktree**, foreign bytes | linked | `jigc uninstall` | 1 | `uninstall.foreign-bytes` | H | `.jigc/tasks/hand-made/n.txt` — repo-relative; **0 host paths** outside the declared route absolute | matches contract | R-18 |
| 46 | idempotency — second run on a clean repo | docs/deep | `jigc uninstall` | 0 | none | — | `(nothing to remove — no repo-local jigc install was present)` | matches contract | R-1 |
| 47 | the tracked-file warning's restore note | docs/deep · spaced | `jigc uninstall` | 0 | none | — | ``note: each is in the index, so `git -C <ABS> checkout -- <path>` brings it back`` — quoted under a spaced root | **EVIDENCE GAP as filed** (the spaced half is in no cited block) → **re-driven by the reconciler, matches contract** | R-1 · ~~R-17~~ → **R-24** |
| 48 | **route placeholder** on the dirty-worktree arm | root | `jigc uninstall` | 1 | `uninstall.dirty-worktree` | H | the route says ``jigc milestone discard <milestone-id> --force`` — an **unfilled** placeholder, while the `staged-prose` sibling one screen over names the real id | OBS-1 (observation, not a defect) — **EVIDENCE GAP as filed** (the route line is in no cited block) → **re-driven by the reconciler, OBS-1 stands** | R-2 · R-6 · **R-25** |

### 1.4 · `task discard` — `Disposition::Refuse{--force}` (11 rows)

| # | cell | cwd | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 49 | jigc's own files only (staged prose) | docs/deep | `jigc task discard tidy-the-readme` | 1 | `task-discard.staged-prose` | H | `foreign-bytes` does **not** fire; route names `doc show … --task` | matches contract | R-8 |
| 50 | one foreign byte | docs/deep · linked · fan-out worktree | same | 1 | `task-discard.foreign-bytes` | H | `.jigc/tasks/tidy-the-readme/foreign.txt`; **byte-identical from all three cwds**, 0 host paths | matches contract | R-8 · R-18 |
| 51 | the same, `--force` | docs/deep · linked · fan-out worktree | `… --force` | 0 | none | I | `warning: removing the working area …` + *not recoverable*; then the ack; plant gone | matches contract (consent) | R-8 · R-18 |
| 52 | a foreign byte whose **own name contains a space**, spaced repo root | docs/deep (spaced) | `jigc task discard tidy-the-readme` | 1 | `task-discard.foreign-bytes` | H | `.jigc/tasks/tidy-the-readme/my notes.txt` listed unquoted **in the message** (a display listing, not a command — the declared split); no runnable span names it | matches contract | R-6 |
| 53 | the same, `--force` | docs/deep (spaced) | `… --force` | 0 | none | I | warning names the spaced path; plant GONE | matches contract | R-6 |
| 54 | malformed / empty id | docs/deep | `jigc task discard ""` · `"../.."` | 1 | `work-unit.malformed-id` | H | `.jigc/` intact (`.gitignore AGENT.md config state version`) | matches contract | R-14 |
| 55 | **S18** residual (bare `mkdir`) | docs/deep | `jigc task discard ghost-residual` | 1 | `finalize.no-task` | H | residual sentence + repo-relative route; nothing changed | matches contract | R-14 |
| 56 | **F-2** the residual is a **symlink** to an outside tree | docs/deep | `jigc task discard ghost` | 1 | `finalize.no-task` | H | *"`.jigc/tasks/ghost` **is a directory** carrying no base pin"* — it is a symlink; outside target 1/1 intact | **F-2 STILL-OPEN** (tier 3) | R-20 |
| 57 | **control** for F-2 — a real directory | docs/deep | `jigc task discard ghost-dir` | 1 | `finalize.no-task` | H | the same sentence, true there | matches contract | R-20 |
| 58 | **F-1** a **symlink wearing a staged identity** | docs/deep | `jigc task discard tidy-the-readme` | 1 | `task-discard.foreign-bytes` | H | names `adr:via-symlink.md` as **foreign** while `doc list --task` calls the same path `managed` | **F-1 STILL-OPEN** (tier 3) | R-20 |
| 59 | **F-5** a settled sub-task's leftover, with and without `--force` | docs/deep | `jigc task discard area-two [--force]` | 1 · 1 | `milestone.terminal` | H | *already `joined` … a leftover, not live work*; `--force` refuses identically; HEAD unmoved | **F-5 STILL-OPEN** (tier 3) | R-14 |

### 1.5 · `task finalize` — `Disposition::Displace` (9 rows)

| # | cell | cwd | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 60 | **all move** | docs/deep | `jigc --format json task finalize tidy-the-readme` | 0 | none | I | `committed.displaced` carries the pair **repo-relative**; `<from> → <to>` on stderr; area gone; plant 1/1 | matches contract | R-8 |
| 61 | **none move** (`.jigc/displaced` a regular **file**) | docs/deep | same | 0 | `finalize.foreign-bytes` (advisory) | H | area **left standing**; `Not a directory (os error 20)`; key `{code, target:"task:tidy-the-readme"}`; `left_out` names `.jigc/displaced`; **0 host paths** | matches contract | R-8 |
| 62 | **control** — ordinary lifecycle, no plant | docs/deep | same | 0 | none | — | `displaced: []`, `findings: []`, `.jigc/displaced` **absent**, `.jigc/tasks` empty | matches contract | R-8 |
| 63 | **all move**, run from a **branch-attached linked worktree** | linked | `jigc task finalize tidy-the-readme` | 0 | none | I | commit lands on `feat`; `CommitSite` line prints — **but the displacement note renders both paths host-absolute** | **DEFECT F-A** | R-21 |
| 64 | the same, `--format json` | linked | `jigc --format json task finalize …` | 0 | none | I | **`committed.displaced[].from`/`.to` are host-absolute** on the 1.0-pinned envelope | **DEFECT F-A** | R-21 |
| 65 | **none move**, from a linked worktree | linked | same | 0 | `finalize.foreign-bytes` | H | `message` **and** `route` host-absolute; `at:`/`location.address`/`key.target` stay `task:tidy-the-readme` (identity, unaffected) | **DEFECT F-A** | R-21 |
| 66 | the same from a **subdirectory of** the linked worktree | linked/docs/deep | `jigc task finalize …` | 0 | none | I | identical absolutes — the fault is the checkout, not the depth | **DEFECT F-A** | R-21 |
| 67 | **CommitSite** — the door names where it committed | linked | `… --dry-run` then the real run | 0 · 0 | none | I | ``would commit / committed in the linked worktree at `<ABS>` on branch `feat` — not in the main checkout jigc's workbench binds to`` | matches contract (the declared absolute) | R-21 |
| 68 | **cwd: fan-out worktree** — is F-A reachable there? | fan-out worktree | `jigc --format json task finalize …` | 1 | `repo.head-detached` | H | refused **before** the Displace — a provisioned worktree is detached by construction, so F-A's reachable cwd is a *branch-attached* linked worktree only | matches contract | R-22 |

### 1.6 · `milestone finalize` — `Disposition::Displace` (12 rows)

| # | cell | cwd | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 69 | **A3-1** — 8 planted loci (area root · nested area dir · `merged/top.txt` · `merged/docs/{deep.txt, provenance.json, adr.md}` · `merged/sub/` · a sub-task area) | docs/deep | `jigc --format json milestone finalize alpha-probe` | 0 | none | I | **8 before / 8 after**; 8 pairs on `committed.displaced` sorted by `from`; each foreign directory moved **whole**; both area roots empty | **A3-1 holds** | R-13a |
| 70 | **A3-2 / D1×D2** — `.jigc/displaced` a regular file, so nothing can be parked | docs/deep | same | 0 | `finalize.foreign-bytes` ×2 (stderr) | H | **3 before / 3 after**; both areas **left standing**; one advisory per standing area keyed `milestone:beta-probe` / `task:area-one`; `displaced: []`; **0 host paths** | **A3-2 holds** | R-13b |
| 71 | **A3-3** — the same plant set, finalized **from inside a fan-out worktree** | fan-out worktree | same | 0 | none | I | 3 before / 3 after, all parked; HEAD moved on **main**; **0 host paths** | matches contract | R-13c |
| 72 | **census C2-06** — the boundary from inside a fan-out worktree | fan-out worktree | `jigc --format json milestone finalize cc-probe-one` | **0** | none | — | full landed envelope, `main` HEAD advanced, both areas cleared | **C2-06 CLOSED** | R-4 |
| 73 | the boundary from `docs/deep` | docs/deep | same | 0 | none | — | byte-equal envelope to the root control but for the sha | matches contract | R-4 |
| 74 | the boundary from an **ordinary linked worktree** | linked | same | 0 | none | — | commits onto **main**, not `feat`; no site line (`CommitSite` is `task finalize`'s, and the milestone boundary is always at the home) | matches contract — OBS-2 | R-4 |
| 75 | **Displace** from a linked worktree, with a plant | linked | `jigc --format json milestone finalize uniform-probe` | 0 | none | I | `committed.displaced` and the stderr note are **repo-relative**, **0 host paths** — the clean sibling of F-A | matches contract | R-21 |
| 76 | **F-4** — a **deleted tracked** file in the fan-out worktree | docs/deep | `jigc milestone finalize f-four` | 0 | none | I | `keeper.md (never staged)` under *"the only copy … not recoverable"*; after the run `git show HEAD:keeper.md` returns the bytes and the main checkout's copy is untouched | **F-4 STILL-OPEN** (tier 3) | R-7 |
| 77 | the `--ignored` axis (ignore rule committed **before** the pin) | docs/deep | `jigc milestone finalize papa-probe` | 0 | none | I | `build/ (ignored by git)` · `scratch.txt (never staged)` on **both** streams | matches contract | R-16 |
| 78 | `milestone.zero-contribution` | docs/deep | `jigc milestone finalize mike-probe` | 3 | `milestone.zero-contribution` | M | names `provision` + `discard`; explains the terminal flip | matches contract | R-16 |
| 79 | the boundary over a milestone **created in a linked worktree** | root | `jigc milestone finalize kilo-probe` | 0 | none | — | lands clean — the review's MEDIUM 4 (base pin left on the standing checkout) is **CLOSED** | matches contract | R-15 |
| 80 | the same, finalized **from** that linked worktree | linked | `jigc milestone finalize lima-probe` | 0 | none | — | lands on **main** | matches contract | R-15 |

### 1.7 · the residual rule's other doors, and the doors the axis touches (13 rows)

| # | door | cell | cwd | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 81 | `task list` | **S18** residual present | docs/deep | 0 | none | `no active tasks` — the residual is not listed | matches contract | R-14 |
| 82 | `task list` | **F-5** settled sub-task's leftover | docs/deep | 0 | none | `1 active task(s)` / `area-two [sub-task]` while `task discard` calls it a leftover | **F-5 STILL-OPEN** | R-14 |
| 83 | `task validate` | residual | docs/deep | 1 | `finalize.no-task` | the residual sentence + repo-relative route | matches contract | R-14 |
| 84 | `task diff` | residual | docs/deep | 1 | `finalize.no-task` | the same | matches contract | R-14 |
| 85 | `start` | residual present (orientation) | docs/deep | 0 | none | clean orientation, no `also open:` row; `Pack: dev/1.0.0-rc.19 \| methodology/1.0.0-rc.19` | matches contract | R-14 · R-16 |
| 86 | `start` | mint at the residual's slug | docs/deep | 1 | `task.serial-collision` | residual sentence; nothing minted | matches contract | R-14 |
| 87 | `milestone create` | a settled milestone's title | docs/deep | 1 | `milestone.record-exists` | ``(its record reads `joined`)``; route names a different title | matches contract | R-13 |
| 88 | `milestone create` | from an ordinary linked worktree | linked | 0 | none | `minted milestone:kilo-probe (shared base <sha>)` + record commit; finalizable afterwards from both cwds | matches contract (MEDIUM 4 closed) | R-15 |
| 89 | `milestone add-task` | from a linked worktree | linked | 0 | none | `added task:area-one …`; record commit on its own | matches contract | R-15 |
| 90 | `milestone join` | from docs/deep · fan-out worktree · linked | all three | 0 | none | `joined milestone:<id> — 0 doc(s) merged`, byte-identical | matches contract | R-4 |
| 91 | `milestone list-tasks` · `milestone execute` | a **pin-less** milestone area while the record is live | docs/deep | 0 · 0 | none | roster renders; the execute composition renders; nothing bricks | matches contract | R-15 |
| 92 | `milestone execute` | the **Spawn** line under a **spaced** repo root | root · docs/deep · fan-out worktree | 0 | none | ``Spawn: `cd '/…/jigc space.…/.jigc/worktrees/area-one' && jigc workflow sub-task --task area-one` `` — **single-quoted**, and `sh -c` runs it to exit 0 from all three cwds | **review HIGH-1 CLOSED** | R-6 |
| 93 | `doc list` · `validate` | from root · docs/deep · fan-out worktree | all three | 0 | none | byte-identical listings, same `Project config` home, `validate` exit 0 — the pack-set no longer vanishes inside a worktree | matches contract (the `pack.rs` walk-up copy is gone) | R-16 |
| 94 | `rename` | over a task residual **and** a milestone-area residual, live task open | docs/deep | 1 | `rename.in-flight` | names `task finalize` / `task discard` | matches contract | R-17 |
| 95 | `rename` | the same after the task is discarded | docs/deep | 0 | none | `renamed research:context-loss -> research:renamed-probe …, repointed 0 referrer(s)`; **both residuals still on disk** | matches contract | R-17 |

**Row count: 95 driven rows over 18 doors.**

---

## 2 · Findings

### F-A (new) — `jigc task finalize`'s Displace surfaces print host-absolute paths from a linked worktree, including on the 1.0-pinned envelope

**Tier: 2/3 — NOT tier 1.** No byte is lost: the plant is 1/1 before and after in every cell, the move
itself succeeds, and the area is left standing in the failed-move cell exactly as contracted. What breaks is
**law 1** and a **pinned key's value shape**.

**The stated contract it contradicts.**
`design/surface-contract.md` → *The printed-path fence (law 1)*: the rule has one home,
`crate::render::repo_relative(repo_root, path)` — repo-relative and `/`-separated, and *"the honest absolute
for a path genuinely outside the repository."* `.jigc/tasks/<id>/notes.txt` is genuinely **inside** the
repository; it is outside only the *standing checkout*. The rc.18 post-review MEDIUM 1 closed exactly this
class at `milestone finalize` (baseline row 60: *"0 host-path hits across message · `at:` · `location.address`
· `key.target` · route"*), and `milestone finalize` is still clean in the identical cell (row 75 above). The
task door was not swept with it.

**The affected surfaces, enumerated:**

1. the stderr displacement note, **both** arms (`… were moved aside, not taken:` and `… not one of them could
   be moved aside:`) — `<from> → <to>` and the park-failure line;
2. `committed.displaced[].from` and `.to` — a **1.0-pinned** key
   (`design/command-output-contract.md`; the additive-key window closed at M48);
3. `finalize.foreign-bytes`'s `message` (three path occurrences) and `route` (one).

Unaffected: `at:` / `location.address` / `key.target`, which carry the work-unit identity
`task:tidy-the-readme` and never a path — so the finding **key** does not leak the host path.

**Reachable cwd axis, driven in both directions:** a **branch-attached linked worktree** (and any
subdirectory of one) fires it; a provisioned **fan-out** worktree cannot, because it is detached and the door
refuses `repo.head-detached` first (row 68); the repo root and an ordinary subdirectory are clean (rows
60–62). `milestone finalize`, `milestone discard`, `task discard` and `uninstall` are clean from every cwd
(rows 27–28, 45, 50–51, 75).

### F-B (new) — the `LeftoverShape::File` consent arm lists the leftover's own basename as though it were a child entry

**Tier 3.** `milestone.rs`'s `LeftoverHold.entries` doc-comment states the rule the refusal arm obeys:
*"**Empty for the two shapes with no inside** — a `LeftoverShape::File` is named by its path … because listing
an empty set beside either would read as 'it holds nothing'."* The refusal arm honours it (*"the file itself
— it is a file, not a worktree"*). The `--force` arm does not: it prints the same child-entry listing a
`Directory` leftover gets, filled with the file's own basename, so the reader sees a path and a child that are
the same file named twice. Same law-1 *nothing lies* clause, one screen over from the wording built to prevent
that exact misread. Repro R-19 carries the directory control beside it.

### Still-open rc.18 rows (all tier 3, triaged to 1.x — expected)

| id | claim | re-driven verdict on rc.19 | repro |
|---|---|---|---|
| **F-1** | a symlink wearing a staged identity is jigc's own at the read/ack surfaces and a third party's at the destroying probe | **STILL-OPEN** — `task discard` names it under `task-discard.foreign-bytes`, `doc list --task` calls the same path `adr:via-symlink … managed`; outside target 1/1 intact | R-20 |
| **F-2** | the residual note asserts *"is a directory"* over a shape it did not check | **STILL-OPEN** — a symlink at `.jigc/tasks/ghost` produces *"`.jigc/tasks/ghost` is a directory carrying no base pin"*; the real-directory control makes the same sentence true | R-20 |
| **F-4** | the fan-out teardown narrates a **deleted tracked** file as *"never staged"* under *"the only copy … not recoverable"* | **STILL-OPEN** — `git show HEAD:keeper.md` returns the bytes after the run, and the main checkout's copy is untouched | R-7 |
| **F-5** | a settled sub-task's leftover is *"1 active task(s)"* at `task list` and *"a leftover, not live work"* at `task discard`, and `finalize`'s route names a command that refuses | **STILL-OPEN, all three halves** — `task discard --force` refuses identically; `finalize.empty-commit`'s route names that refusing command; HEAD unmoved | R-14 |

### Observations (not findings)

- **OBS-1** — `uninstall.dirty-worktree`'s route says ``jigc milestone discard <milestone-id> --force`` with
  an unfilled placeholder, while the `uninstall.staged-prose` sibling one screen over names the real milestone
  (`jigc milestone finalize space-probe`). Placeholders are the house style (`<path>`, `<task-id>`,
  `<address>` all ship), so this is an inconsistency rather than a contract break. Rows 37 · 48.
- **OBS-2** — `milestone finalize` from a branch-attached linked worktree commits onto **main** and says
  nothing about it. That is exactly what `render::CommitSite`'s doc-comment prescribes (*"emitted **only**
  when the commit site is not the checkout jigc's workbench binds to … naming the main checkout on every run
  would be a line that is always true and never news"*), so it matches the stated contract — recorded because
  a reader standing on `feat` watching `main` move is the cell a future review will want on the record. Row 74.
- **OBS-3** — `jigc start`'s orientation prints `Project config: <host-absolute>/.jigc/config` from every cwd.
  Cwd-invariant, and axis 6's subject rather than this one; noted only so it is not mistaken for an F-A
  sibling. Row 85.
- **OBS-4** — `Disposition::Narrate` is still empty by construction at rc.19 (no member holds it), so that
  arm of the disposition axis has no drivable cell. Stated rather than silently skipped.

---

## 3 · Repro blocks

All ran on `jigc 1.0.0-rc.19`, each in its own `dev/jigc-rig` throwaway repo (root from `mktemp -d`; no
teardown, no `rm` on a variable path). Exit codes read bare. `command grep` carries every loss/survival claim.

### R-1 — `uninstall` from the repo root and a subdirectory; the prune line; idempotency (rows 17, 29, 31, 46, 47)

```
setup: dev/jigc-rig fresh --binary ~/.local/bin/jigc ; mkdir -p $REPO/docs/deep
       jigc milestone create "CC probe one"; … add-task "area one"; … provision cc-probe-one
BEFORE: .jigc P · .git/hooks/pre-commit P · CLAUDE.md P · SKILL.md P · worktree P

$ (cd $REPO/docs/deep) jigc uninstall                                          -> exit 0
  jigc uninstall — repo-local install removed
    - removed .jigc/
    - pruned git's worktree registrations for the fan-out worktrees `.jigc/` held
    - unwired bootstrap reference ← CLAUDE.md
    - removed jigc allowlist permit / SessionStart hook / deny safety floor ← .claude/settings.json
    - removed pre-commit hook              - removed jigc guide artifact
  stderr: warning: removing `.jigc/` also removes 5 tracked file(s) under it: …
          note: each is in the index, so `git -C <ABS repo> checkout -- <path>` brings it back.
  NO site line (jigc_home == the standing checkout)
AFTER: .jigc A · hook A · CLAUDE.md A · SKILL.md A
$ jigc uninstall   (again)                                                     -> exit 0
  "(nothing to remove — no repo-local jigc install was present)"
```

### R-2 — `uninstall` × cwd: the fan-out worktree, the linked worktree, and all four guards (rows 32–36, 38, 48)

```
setup (one FRESH rig per cell): fresh; milestone create/add-task/provision -> W=.jigc/worktrees/area-one

CELL 32 — cwd = W, clean                                                       -> exit 0
  stdout  the 8 removal lines, then:
    removed at `/private/var/…/repo` — the main checkout this repository's jigc install and `.jigc/`
    workbench bind to, and that workbench held the worktree you are standing in, which this removed
  AFTER (measured in the MAIN checkout): .jigc A · hook A · CLAUDE.md A · SKILL.md A · worktree A
  git worktree list -> one row (the main checkout)              <- the prune ran

CELL 33 — cwd = an ordinary linked worktree ($REPO/../linked)                  -> exit 0
    removed at `/private/var/…/repo` — … , not the worktree you are standing in
  AFTER: linked worktree PRESENT · fan-out worktree ABSENT

CELL 34 — a DIRTY fan-out worktree (A  f.txt)
  cwd = W    -> exit 1  uninstall.dirty-worktree — `.jigc/` holds 1 fan-out sub-task worktree path(s) …
                 .jigc/worktrees/area-one: A  f.txt — … registered as a worktree of this repository
  cwd = $REPO-> exit 1  BYTE-IDENTICAL                                  <- the guard is not inert from W
CELL 35 — .jigc/notes.txt, cwd = W  -> exit 1 uninstall.untracked-workbench-file; plant 1 after
CELL 36 — .jigc/tasks/hand-made/notes.txt, cwd = W -> exit 1 uninstall.foreign-bytes; plant 1 after
CELL 38 — a live bisect in W, spotless tree (git status --porcelain -> 0 lines)
  cwd = $REPO / docs/deep / W -> exit 1 each, identical:
    .jigc/worktrees/area-one: a bisect git has left un-concluded (abandon it with
      `git -C /private/var/…/repo/.jigc/worktrees/area-one bisect reset`) …
  AFTER: worktree PRESENT · BISECT_LOG PRESENT
```

### R-3 — `milestone discard` / `provision` × cwd, and `--force` from inside the worktree it removes (rows 13, 18, 19)

```
setup: fresh; milestone create "CC probe one" + two sub-tasks; provision; both stage code
       KEEPMD > .jigc/milestones/cc-probe-one/plant.txt      BEFORE hits 1

$ jigc milestone discard cc-probe-one   from $REPO, from docs/deep, from inside W1
  -> exit 1 each, milestone.dirty-worktree, BYTE-IDENTICAL:
     .jigc/worktrees/area-one: A  one.txt — … registered here, so the teardown removes it …
     .jigc/worktrees/area-two: A  two.txt — …
     at: .jigc/worktrees/area-one
AFTER hits 1
$ (cd W1) jigc milestone discard cc-probe-one --force                          -> exit 0
  stdout  discarded milestone:cc-probe-one (2 sub-task(s); workbench removed)
  stderr  warning: removing the working area .jigc/milestones/cc-probe-one … plant.txt / not recoverable
AFTER: W1 ABSENT · plant hits 0 · git worktree list -> one row
$ (cd $REPO) jigc milestone provision cc-probe-two   over a Directory leftover  -> exit 1
  milestone.leftover-holds-work — … precious.txt — git reports no worktree of its own there …
```

### R-4 — the milestone boundary × four cwds (census C2-06) (rows 11, 14, 72–74, 90)

```
setup (one FRESH rig per cell): fresh; milestone create "CC probe one"; two sub-tasks; provision;
       both worktrees stage one file; jigc milestone join cc-probe-one

cwd = $REPO         jigc --format json milestone finalize cc-probe-one  -> exit 0  HEAD 7dc182f
cwd = docs/deep     same                                                -> exit 0  HEAD b571c6a
cwd = W1 (fan-out)  same                                                -> exit 0  HEAD a22e999
                    areas after: milestones=[] tasks=[] ; host-absolute hits 0
cwd = linked        same                                                -> exit 0  main HEAD 12792d3
                    feat HEAD unmoved                                   <- the boundary is home-bound
every envelope: committed.{commits,displaced:[],files:3,hash,hook_output,manifest,still_staged:[],
                sub_tasks:[2],subject}
`jigc milestone join` from all four cwds -> exit 0, byte-identical

row 11/14 — a REGISTERED worktree carrying a live bisect, idempotent re-provision:
  jigc milestone provision cc-probe-one -> exit 0 "provisioned 1 worktree(s) … at base <sha>"
  AFTER: BISECT_LOG PRESENT · worktree PRESENT           <- neither probed nor cleared
```

### R-5 — the operation-bearing routes, RUN VERBATIM from three cwds (rows 20, 21, 39)

```
setup: fresh; milestone create/add-task/provision; git -C W bisect start/bad/good HEAD~1
       git -C W status --porcelain -> 0 lines                            <- SPOTLESS

$ (cd $REPO) jigc milestone discard cc-probe-one                               -> exit 1
  milestone.dirty-worktree; hold line carries
    `git -C /private/var/…/repo/.jigc/worktrees/area-one bisect reset`
SPAN extracted and run VERBATIM:
  from $REPO        -> exit 0   HEAD is now at 9eb27b3 …
  from docs/deep    -> exit 0   HEAD is now at 9eb27b3 …                 <- census C1-06 dead end GONE
  from W            -> exit 0   HEAD is now at 9eb27b3 …
$ (cd docs/deep) jigc uninstall  (with .jigc/notes.txt planted)                -> exit 1
  uninstall.untracked-workbench-file; route carries `git -C <ABS repo> add -- <path>`
RUN VERBATIM (<path> -> .jigc/notes.txt) from docs/deep -> exit 0; staged: .jigc/notes.txt
re-run jigc uninstall from docs/deep                                           -> exit 0 (6 tracked files)
```

### R-6 — a repository root containing a SPACE: the Spawn line, the routes, the doors (rows 16, 37, 52, 53, 92)

```
setup: SCRATCH=$(mktemp -d "$TMPDIR/jigc space.XXXXXX"); dev/jigc-rig fresh
       REPO=/var/folders/…/jigc space.qBMpBr/jigc-rig-fresh-5rwdac/repo
       milestone create "Space probe"; two sub-tasks; provision

$ jigc milestone execute space-probe                                           -> exit 0
  Spawn: `cd '/private/var/…/jigc space.qBMpBr/…/repo/.jigc/worktrees/area-one' && jigc workflow
          sub-task --task area-one`                                     <- SINGLE-QUOTED
  run with sh -c from $REPO / docs/deep / the worktree -> exit 0, 0, 0  <- review HIGH 1 CLOSED
$ (cd docs/deep) jigc milestone discard space-probe   (bisect live in area-one) -> exit 1
  `git -C '/private/var/…/jigc space.qBMpBr/…/area-one' bisect reset`   <- quoted
  run VERBATIM from docs/deep -> exit 0  "HEAD is now at 3f40134 …"
$ (cd W) jigc uninstall                                                        -> exit 1
  uninstall.staged-prose — area-one: commit:area-one — a sub-task of milestone `space-probe`, which
    `jigc task finalize` refuses: land it with `jigc milestone finalize space-probe` …   <- names the id
$ a foreign byte whose OWN name has a space: .jigc/tasks/tidy-the-readme/my notes.txt
  jigc task discard tidy-the-readme          -> exit 1 task-discard.foreign-bytes, path listed in the
    MESSAGE (a display listing; no runnable span names it)
  jigc task discard tidy-the-readme --force  -> exit 0; warning names it; plant GONE
```

### R-7 — F-4, and `milestone discard --force`'s loss narration (rows 26, 76)

```
setup: fresh; TRACKED-BYTES-AT-HEAD > keeper.md; git add + commit
       milestone create "F four" + one sub-task; provision; SUB > W/sub.txt; git -C W add sub.txt
       rm W/keeper.md                       # delete a TRACKED file, do not stage the deletion
BEFORE: git -C W status --porcelain -> " D keeper.md" / "A  sub.txt"
        git -C $REPO show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD

$ jigc milestone join f-four && (cd docs/deep) jigc milestone finalize f-four  -> exit 0
  stdout  discarded with the fan-out worktrees (not committed, not recoverable):
            area-one: keeper.md (never staged)
  stderr  warning: removing the fan-out worktree … keeper.md (never staged)
          note: the fan-out worktree is the only copy of these bytes — they are not recoverable.
AFTER — is the claim true?
  git show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD       # the bytes ARE in git
  cat $REPO/keeper.md     -> TRACKED-BYTES-AT-HEAD       # and in the main checkout      -> F-4 STANDS

row 26 (spaced root, docs/deep): jigc milestone discard november-probe --force -> exit 0
  build/out.o (never staged) · dirt.txt (never staged)   (the ignore rule was committed AFTER the pin,
  so git in that worktree reports both as untracked — the narration matches what git reports)
```

### R-8 — `task discard` and `task finalize`'s Displace from `docs/deep` (rows 49–51, 60–62)

```
setup: fresh; jigc start --workflow quick-fix "tidy the readme"; echo code > src.txt; git add src.txt
       printf 'tidy the readme\n' | jigc doc set-slot commit:tidy-the-readme#summary --from-file -
       jigc doc set-field commit:tidy-the-readme#type --value chore
       jigc task validate tidy-the-readme -> "no findings — the task validates clean"

row 49  jigc task discard tidy-the-readme          -> 1  task-discard.staged-prose ONLY
row 50  + .jigc/tasks/tidy-the-readme/foreign.txt  -> 1  task-discard.foreign-bytes, path named
row 51  … --force                                  -> 0  warning + not recoverable; AFTER: GONE
row 60  KEEPTF > .jigc/tasks/tidy-the-readme/notes.txt   BEFORE 1
        jigc --format json task finalize tidy-the-readme -> 0
          displaced [{from .jigc/tasks/tidy-the-readme/notes.txt,
                      to   .jigc/displaced/tidy-the-readme/notes.txt}]   findings []
          stderr "… held 1 entry jigc did not write … moved aside, not taken:" + the → pair
        AFTER 1 · area GONE · host-absolute hits 0
row 61  printf 'NOT A DIR' > .jigc/displaced ; same finalize             -> 0
          displaced []   left_out [{untracked, .jigc/displaced}]
          findings[0] advisory finalize.foreign-bytes key {code, target:"task:tidy-the-readme"}
            "… could not open .jigc/displaced/tidy-the-readme to park it: Not a directory (os error 20)"
        AFTER 1 · area STANDING · host-absolute hits 0
row 62  control, no plant -> 0  displaced [] findings [] .jigc/displaced ABSENT .jigc/tasks empty
```

### R-9 — `LeftoverShape` × `--force` at `milestone provision`, from `docs/deep` (rows 1–8)

```
setup per cell: fresh; milestone create "Delta probe" + one sub-task (NOT provisioned)

Directory + plant       -> 1 milestone.leftover-holds-work "… precious.txt — git reports no worktree
                             of its own there …";  --force -> 0, warning, plant GONE
dangling .git inside    -> 1 "… .git, wip.txt — git cannot read a repository there …"
File (regular)          -> 1 "… the file itself — it is a file, not a worktree …"
File (symlink→outside)  -> 1 identical wording;  --force -> 0; outside tree 1/1 INTACT
Unreadable (chmod 000)  -> 1 "… unknown — could not read the leftover directory `.jigc/worktrees/
                             area-one`: Permission denied (os error 13) …"    at: .jigc/worktrees/area-one
```

### R-10 — the prune line is measured, not asserted (rows 42, 43)

```
cell A: fresh, NO milestone ever created
  jigc uninstall -> 0 ; `command grep -c 'pruned git' stdout` -> 0        <- line ABSENT
cell B: fresh; milestone create/add-task/provision; milestone discard --force  (worktrees already gone)
  git worktree list -> one row
  jigc uninstall -> 0 ; `command grep -c 'pruned git' stdout` -> 0        <- line ABSENT
cf. R-1/R-2, where a live fan-out worktree WAS present: the line prints.
```

### R-11 — `uninstall --force` from inside the fan-out worktree (row 41)

```
setup: fresh; milestone create/add-task/provision; SUBWORK > W/f.txt; git -C W add f.txt
       KEEPUF > .jigc/notes.txt
BEFORE: plant 1 · hook P · worktree P

$ (cd W) jigc uninstall --force                                                -> exit 0
  stdout  the 8 removal lines incl. the prune, then the site line with
          "… and that workbench held the worktree you are standing in, which this removed"
  stderr  warning: removing `.jigc/` destroys 1 file(s) under it that no index has a copy of:
              .jigc/notes.txt
            note: nothing has a copy of those bytes — they are not recoverable.
          warning: removing `.jigc/` also removes 5 tracked file(s) under it: … (`git -C <ABS> checkout`)
AFTER: .jigc A · hook A · CLAUDE.md A · SKILL.md A · worktree A · plant GONE
       git worktree list -> one row ; .git/worktrees/ EMPTY          <- pruned
```

### R-12 — S19: the leftover path is a SECOND repository's linked worktree, mid-bisect (rows 9, 10)

```
setup: fresh; milestone create "Sierra probe" + one sub-task
       OTHER=$(mktemp -d); git init; two commits; git -C $OTHER worktree add $REPO/.jigc/worktrees/area-one
       OTHERKEEP > area-one/keep.txt; committed there; git -C area-one bisect start/bad/good HEAD~1
BEFORE: git -C area-one status --porcelain -> 0 lines ; OTHER worktree list -> 2 rows

$ (cd docs/deep) jigc milestone provision sierra-probe                         -> exit 1
  milestone.leftover-holds-work — .jigc/worktrees/area-one: a bisect git has left un-concluded
    (abandon it with `git -C <ABS> bisect reset`) — … (NO "registered here" clause — correct)
AFTER: other repo's worktree PRESENT · keep.txt 1/1
$ (cd docs/deep) jigc milestone provision sierra-probe --force                 -> exit 0
  warning: removing the fan-out worktree … / a bisect git had left un-concluded … / not recoverable
AFTER: $OTHER/a.txt intact; $OTHER's own checkout intact; its committed `area-one` branch is in its
       object store (the destroyed bytes were a checkout, not a copy of record)
```

### R-13 — the foreign-bytes complement and the terminal arms; A3-1 / A3-2 / A3-3 (rows 22, 23, 44, 69–71, 87)

```
### R-13 (complement) — fresh; milestone "Foxtrot probe" + two sub-tasks; provision; stage; join
  5 plants: merged/x.txt · merged/docs/y.txt · mnote.txt · tasks/area-one/t1.txt · tasks/area-two/t2.txt
  (worktrees removed first so the dirty-worktree guard does not mask the foreign arm)
  jigc milestone discard foxtrot-probe   (docs/deep) -> 1 milestone.foreign-bytes, ALL 5 named
  jigc uninstall                          (docs/deep) -> 1 uninstall.foreign-bytes, the SAME 5
  BEFORE 5 / AFTER 5
### terminal — fresh; "Tango probe" finalized to `joined`
  jigc milestone discard tango-probe -> 1 milestone.terminal "is `joined` — a settled milestone is over"
  jigc milestone create "Tango probe" -> 1 milestone.record-exists "(its record reads `joined`)"

### R-13a (A3-1) — fresh; "Alpha probe"; two sub-tasks; provision; stage; join; 8 plants
  BEFORE (command grep -rl 'KEEP-m-' .jigc | wc -l) -> 8
  (cd docs/deep) jigc --format json milestone finalize alpha-probe             -> exit 0
    committed.displaced = 8 pairs, sorted by from:
      merged/docs/adr.md · merged/docs/deep.txt · merged/docs/provenance.json · merged/sub (WHOLE) ·
      merged/top.txt · mnotes.txt · sub (WHOLE) · .jigc/tasks/area-one/tnotes.txt
    two stderr notes, one per area
  AFTER 8 · area roots milestones=[] tasks=[] · host-absolute hits 0

### R-13b (A3-2 / D1×D2) — the same with `printf 'NOT A DIR' > .jigc/displaced`
  BEFORE 3 ; (cd docs/deep) jigc --format json milestone finalize beta-probe   -> exit 0
    committed.displaced []   two advisories on stderr, one per standing area:
      finalize.foreign-bytes at: milestone:beta-probe   (2 paths, "Not a directory (os error 20)")
      finalize.foreign-bytes at: task:area-one          (1 path)
  AFTER 3 · both areas LEFT STANDING · host-absolute hits 0

### R-13c (A3-3) — the same plant set finalized from INSIDE W1
  BEFORE 3 -> exit 0, all three parked under .jigc/displaced/…, AFTER 3, HEAD moved on main,
  host-absolute hits 0
```

### R-14 — the residual doors and F-5, from `docs/deep` (rows 12, 30, 54, 55, 59, 81–84, 86)

```
### residual (bare mkdir .jigc/tasks/ghost-residual)
  jigc task list                                     -> 0 "no active tasks"
  jigc start                                         -> 0 clean orientation, no `also open:`
  jigc start --workflow quick-fix "ghost residual"   -> 1 task.serial-collision, residual sentence
  jigc task validate ghost-residual                  -> 1 finalize.no-task, repo-relative route
  jigc task diff ghost-residual                      -> 1 finalize.no-task
  jigc task discard ghost-residual                   -> 1 finalize.no-task
  jigc task discard "" / "../.."                     -> 1 work-unit.malformed-id ("" is not a valid
     work-unit id / route: use lowercase letters, digits, and single hyphens …)
     .jigc after: .gitignore AGENT.md config state version

### F-5 — fresh; "Hotel probe" + two sub-tasks; provision; both stage; cp -R the area-two dir to $SAVE;
         join; finalize; cp -R $SAVE/area-two back      (built by restoring a backup — see §5)
  record: `status: joined`   HEAD 03931bc…
  jigc task list                          -> 0  "1 active task(s)" / "area-two [sub-task]"
  jigc task discard area-two              -> 1  milestone.terminal — "already `joined` … its working
                                                area is a leftover, not live work"
  jigc task discard area-two --force      -> 1  the SAME refusal (--force does not consent past it)
  jigc task finalize area-two             -> 3  finalize.empty-commit, route: "abandon it with
                                                `jigc task discard area-two --force`"   <- refuses, above
  HEAD unmoved: YES
```

### R-15 — the linked-worktree cwd: create · add-task · provision · discard · finalize (rows 15, 24, 25, 79, 80, 88, 89, 91)

```
setup: fresh; git -C $REPO worktree add -b feat $REPO/../linked

(cd linked) jigc milestone create "Kilo probe"     -> 0 "minted milestone:kilo-probe (shared base
                                                       77ab205)"; record commit 52f4206
(cd linked) jigc milestone add-task kilo-probe …   -> 0 record commit fdb660e
(cd linked) jigc milestone provision kilo-probe    -> 0 provisioned at base 77ab205
stage code in the fan-out worktree; (cd $REPO) join; (cd $REPO) finalize        -> 0  main HEAD 74a174f
   <- review MEDIUM 4 (the base pin left on the standing checkout) CLOSED
"Lima probe": the same, finalized FROM the linked worktree                      -> 0  main HEAD c6c9316,
   feat HEAD unmoved

dirty fan-out worktree, (cd linked) jigc milestone discard juliet-probe        -> 1 milestone.dirty-worktree
                        (cd linked) jigc milestone provision juliet-probe      -> 0
                        (cd linked) jigc milestone discard juliet-probe --force-> 0 fan-out gone,
                                                                                  linked untouched

pin-less milestone area (base.json removed) while the record is live, from docs/deep:
  jigc milestone list-tasks india-probe -> 0 "milestone:india-probe tasks (1): area-one"
  jigc milestone execute india-probe    -> 0 composes; nothing bricks
```

### R-16 — pack/config resolution × cwd; the `--ignored` narration; zero-contribution (rows 8, 77, 78, 85, 93)

```
setup: committed-singletons; milestone create/add-task/provision
for cwd in $REPO, $REPO/docs/deep, $REPO/.jigc/worktrees/area-one:
  jigc start | grep '^Pack:' -> "Pack: dev/1.0.0-rc.19 | methodology/1.0.0-rc.19 · Project config:
    /private/var/…/repo/.jigc/config"      IDENTICAL from all three   <- the pack.rs walk-up copy is gone
  jigc doc list  -> identical 4-row listing        jigc validate -> exit 0 from all three

--ignored axis (ignore rule committed BEFORE the pin): status "A first.txt / ?? scratch.txt",
  --ignored adds "!! build/"
  (cd docs/deep) jigc milestone finalize papa-probe -> 0
    stdout  area-one: build/ (ignored by git) · scratch.txt (never staged)
    stderr  the same two + "note: the fan-out worktree is the only copy …"

zero-contribution: (cd docs/deep) jigc milestone finalize mike-probe -> 3
  milestone.zero-contribution — "… the boundary would commit only jigc's own bookkeeping and flip the
  milestone record to the terminal `joined` …"; route names provision + discard
```

### R-17 — `rename` over residuals, and the spaced-root uninstall route (rows 40, 94, 95)

```
### spaced root — SCRATCH=$(mktemp -d "$TMPDIR/jigc space.XXXXXX"); fresh; KEEPSP2 > .jigc/notes.txt
  (cd docs/deep) jigc uninstall  -> 1 uninstall.untracked-workbench-file
    route: `git -C '/private/var/…/jigc space.Grz0kz/…/repo' add -- <path>` is enough — the index keeps
      a copy `git -C '/private/var/…/jigc space.Grz0kz/…/repo' checkout -- <path>` restores
  RUN VERBATIM from docs/deep -> exit 0; staged: .jigc/notes.txt

### rename — refs-post-hoc rig ($RIG_TASK live); mkdir .jigc/tasks/ghost-residual .jigc/milestones/ghost-ms
  (cd docs/deep) jigc rename research:context-loss --to renamed-probe        -> 1 rename.in-flight
     "… finalize or discard it first"; route names both verbs
  jigc task discard $RIG_TASK --force ; re-run                                -> 0
     "renamed research:context-loss -> research:renamed-probe (docs/research/context-loss.md ->
      docs/research/renamed-probe.md), repointed 0 referrer(s)"
  residuals after: tasks=[ghost-residual] milestones=[ghost-ms]               <- untouched
```

### R-18 — the host-absolute sweep across the clean doors (rows 27, 28, 45, 50, 51)

```
Every cell: one fresh rig, a plant, then `command grep -cE '/private/var|/var/folders'` over
stdout+stderr together.

task discard        from a LINKED worktree, refusal   -> 1  hits 0
task discard        from a LINKED worktree, --force   -> 0  hits 0
task discard        from a FAN-OUT worktree, refusal  -> 1  hits 0
task discard        from a FAN-OUT worktree, --force  -> 0  hits 0
milestone discard   from a LINKED worktree, refusal   -> 1  hits 0
milestone discard   from a LINKED worktree, --force   -> 0  hits 0
uninstall           from a LINKED worktree, foreign   -> 1  hits 0  (outside the declared `git -C` route)
task discard        from docs/deep (control), both    -> 1/0 hits 0
```

### R-19 — F-B: the `File` leftover's consent arm, with the `Directory` control beside it (row 5)

```
setup: fresh; milestone create "Quebec probe" + one sub-task
       printf 'LEFTOVER-FILE-BYTES\n' > .jigc/worktrees/area-one     (file -b -> "ASCII text")

$ jigc milestone provision quebec-probe                                        -> exit 1
  .jigc/worktrees/area-one: the file itself — it is a file, not a worktree, and nothing can say those
    bytes are disposable                                        <- the refusal arm, correct
$ jigc milestone provision quebec-probe --force                                -> exit 0
  warning: removing the leftover file .jigc/worktrees/area-one discards work that is not in git:
      area-one                                                  <- the path's OWN basename, in the
    note: the leftover file is the only copy of these bytes …       child-entry position
CONTROL, the same arm over a DIRECTORY leftover holding precious.txt:
  warning: removing the leftover directory .jigc/worktrees/area-one discards work that is not in git:
      precious.txt                                              <- a real child
```

### R-20 — F-1 and F-2 re-driven (rows 56–58)

```
### F-1 — fresh; jigc start --workflow quick-fix "tidy the readme"
  OUT=$(mktemp -d); TARGET-BYTES > $OUT/body.md
  ln -s $OUT/body.md .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
  (cd docs/deep) jigc task discard tidy-the-readme  -> 1 task-discard.foreign-bytes
      .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md                    <- FOREIGN here
  (cd docs/deep) jigc doc list --task tidy-the-readme -> 0
      adr:via-symlink  docs/decisions/via-symlink.md  managed                <- JIGC'S OWN here
  outside target 1/1 intact
### F-2 — fresh; OUT2=$(mktemp -d); OUTSIDE-KEEP > $OUT2/keep.txt; ln -s $OUT2 .jigc/tasks/ghost
  ls -l -> lrwxr-xr-x  ghost -> /var/folders/…
  (cd docs/deep) jigc task discard ghost -> 1 finalize.no-task
      "no task `ghost`: `.jigc/tasks/ghost` IS A DIRECTORY carrying no base pin …"
  outside target 1/1 intact
  CONTROL (a real directory .jigc/tasks/ghost-dir): the same sentence, true there
```

### R-21 — F-A: the defect, its surfaces, and its clean sibling (rows 63–67, 75)

```
setup: fresh; git -C $REPO worktree add -b feat $REPO/../linked
       (cd linked) jigc start --workflow quick-fix "tidy the readme"
       echo code > linked/src.txt; git -C linked add src.txt
       the commit doc filled from `linked`; KEEPHP > .jigc/tasks/tidy-the-readme/notes.txt
BEFORE plant: 1

$ (cd linked) jigc --format json task finalize tidy-the-readme                 -> exit 0
  "committed": { "displaced": [ {
      "from": "/private/var/folders/…/repo/.jigc/tasks/tidy-the-readme/notes.txt",
      "to":   "/private/var/folders/…/repo/.jigc/displaced/tidy-the-readme/notes.txt" } ], … }
  stderr  note: … moved aside, not taken:
      /private/var/…/repo/.jigc/tasks/tidy-the-readme/notes.txt → /private/var/…/repo/.jigc/displaced/…
  host-absolute hits: 3
AFTER plant: 1                                                  <- no loss; the fault is the rendering

$ the NONE-MOVE cell (printf 'NOT A DIR' > .jigc/displaced), same cwd                -> exit 0
  findings[0].message  "`/private/var/…/repo/.jigc/tasks/tidy-the-readme` holds 1 path(s) … :
      `/private/var/…/notes.txt`; 1 of them could not be moved aside: `/private/var/…/notes.txt` —
      could not open /private/var/…/.jigc/displaced/tidy-the-readme to park it: Not a directory …"
  findings[0].route    "… Keep what you need from `/private/var/…/repo/.jigc/tasks/tidy-the-readme` …"
  findings[0].key      {"code":"finalize.foreign-bytes","target":"task:tidy-the-readme"}   <- CLEAN
  findings[0].location {"address":"task:tidy-the-readme", …}                               <- CLEAN

$ TEXT format, same cell        -> the same two absolutes on stderr; the ack's own CommitSite line
  "committed in the linked worktree at `/private/var/…/linked` on branch `feat` — not in the main
   checkout jigc's workbench binds to"                          <- the DECLARED absolute, correct
$ from linked/docs/deep         -> identical absolutes          <- depth is not the variable

CLEAN SIBLING, identical cell shape: (cd linked) jigc --format json milestone finalize uniform-probe
  committed.displaced [{from ".jigc/milestones/uniform-probe/mnote.txt",
                        to   ".jigc/displaced/uniform-probe/mnote.txt"}]
  stderr the same pair, repo-relative        host-absolute hits: 0
CONTROLS: the same task-finalize cells from $REPO and $REPO/docs/deep -> hits 0 (R-8)
```

### R-22 — F-A's reachability bound: a fan-out worktree cannot reach it (row 68)

```
setup: fresh; milestone create "Whiskey probe" + one sub-task; provision
       (cd W) jigc start --workflow quick-fix "tidy the readme"; code staged in W;
       commit doc filled with --task tidy-the-readme (exit 0 both writes)
       KEEPP22 > .jigc/tasks/tidy-the-readme/notes.txt      BEFORE 1

$ (cd W) jigc --format json task finalize tidy-the-readme                      -> exit 1
  {"error":"blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no
   branch …\n  route: re-attach HEAD with `git switch <branch>`, then re-run this command"}
  host-absolute hits 0 ; AFTER 1
  -> a provisioned fan-out worktree is detached by construction, so the posture guard fires before the
     Displace. F-A's reachable cwd is a BRANCH-ATTACHED linked worktree (and its subdirectories).
```

---

## 4 · M52 / rc.18 §A rows: CLOSED / STILL-OPEN

The rc.18 run is this axis's baseline (M52's §A rows for axis 3 were carried into it). Each row re-driven on
rc.19:

| baseline row | rc.18 verdict | rc.19 verdict | evidence |
|---|---|---|---|
| **F-3** — `uninstall` inside a fan-out worktree exits 0, reports an install it did not remove, and takes the repository's shared `pre-commit` hook | tier 2, CONFIRMED | **CLOSED** — the door binds `jigc_home`, removes the real install, prunes git's admin, and the site line says the standing worktree was removed | R-2 · R-11 |
| **F-1** — a symlink wearing a staged identity | tier 3, CONFIRMED | **STILL-OPEN** (expected; triaged to 1.x) | R-20 |
| **F-2** — the residual note asserts a shape it did not check | tier 3, CONFIRMED | **STILL-OPEN** (expected) | R-20 |
| **F-4** — the teardown narrates a deleted **tracked** file as unrecoverable | tier 3, CONFIRMED | **STILL-OPEN** (expected) | R-7 |
| **F-5** — a settled sub-task's leftover, both halves + the refusing route | tier 3, CONFIRMED | **STILL-OPEN**, all three halves | R-14 |
| the whole `LeftoverShape` × `--force` matrix (baseline rows 1–9) | matches contract | **holds**, with **F-B** newly recorded on the `File` consent arm | R-9 · R-19 |
| **S19** (rows 10, 11) — a second repository's linked worktree | CLOSED | **holds** | R-12 |
| the post-review HIGH (rows 14–16, 26) — a **spotless** worktree carrying an un-concluded operation refuses at all three worktree doors | CLOSED | **holds**, and the emitted `git -C <ABS>` route now **runs from every cwd** and from a spaced root | R-2 · R-5 · R-6 |
| **A3-1** (rows 54–56) | CLOSED | **holds**, 8 before / 8 after | R-13a |
| **A3-2 / D1×D2** (rows 47, 48, 57) | CLOSED | **holds**, both areas left standing, one advisory each | R-8 · R-13b |
| post-review MEDIUM 1 (row 60) — 0 host paths at `milestone finalize` from a sibling worktree | CLOSED | **holds** at `milestone finalize` — **and F-A is the same class un-swept at `task finalize`** | R-13c · R-21 |
| the terminal / residual / record-state doors (rows 22, 64, 68–72, 74, 75) | matches contract | **hold**, every one re-driven from `docs/deep` | R-13 · R-14 · R-15 · R-17 |
| census **C2-06** (`milestone finalize` unreachable from any worktree) | open at rc.18 | **CLOSED** | R-4 |
| census **C2-07** (= F-3) | open at rc.18 | **CLOSED** | R-2 |
| census **C1-06 / C1-14** (the `git -C <rel>` dead end; the unquoted `cd`) | open at rc.18 | **CLOSED** — both run verbatim, including under a spaced root | R-5 · R-6 |

**Tier-1 rows on this axis after the re-drive: 0.** Stated for the exit rule, for this axis only.

---

## 5 · What I did NOT drive, and why

Stated as un-driven rather than presented as passing:

1. **`Disposition::Narrate`** — no `DESTROYING_DOORS` member holds it at rc.19 (the variant's own doc-comment
   says so), so the arm has no cell. Not a gap in the drive; a gap in the shipped set.
2. **`task finalize --force`** — the door has no `--force`; the Settle refuses one. No cell enumerated.
3. **Hook-rejection cells at the two `Displace` doors** (baseline rows 49, 50 — the hook writing into the area
   during the commit, and a rejected commit with a plant present). These are axis 4's transaction/rollback
   subject; I carried the baseline's verdict and did not re-drive them.
4. **`milestone finalize` with a rejecting hook under a spaced root** — the confirmation pass's MEDIUM 1
   (the pre-commit awk's shell-word tokenizer, and `docs-root 'my docs'`) is axis 2/4's subject and was not
   re-driven here.
5. **A genuine concurrent racer** at any destroying door. The doors are driven serially; nothing here
   discharges the concurrency bound `design/storage.md` → *Concurrent writers* carries.
6. **`GIT_DIR` redirect** — the declared posture residual; out of this axis by M51's stated bound.
7. **`LeftoverVerdict::NoOwnLinkage` at `uninstall` and `milestone discard`** specifically — driven at
   `milestone provision` (R-9's directory cells) and inferred at the other two from the shared classifier;
   the shared-classifier claim is a source fact I did not re-derive.
8. **F-5's reachability** — the state was built by restoring a `cp -R` backup, exactly as the rc.18 driver
   and reconciler did. No jigc sequence reaches it, so the finding's *reachability* is undischarged; only its
   behaviour is driven.

**Instrument faults of mine, recorded rather than dropped:**

- Two milestone ids were wrong on a first attempt (`jigc milestone create "A three"` slugs to `three`, not
  `a-three`, under the edge-stopword rule), so the first A3-1/A3-2 attempt drove `milestone.unknown` instead.
  Re-driven with `alpha-probe`/`beta-probe`; both attempts are in the log.
- One `task finalize` cell first refused because the rig had **two** active tasks and the implicit-task
  resolution rejected — reproduced from the repo root too, so it is my fixture, not the binary. Re-driven with
  `--task`.
- One `--ignored` cell first committed `.gitignore` **after** the milestone pin, so the worktree reported the
  tree as untracked rather than ignored. Re-driven with the rule committed before the pin (R-16).

---

## 6 · What this adds over flow-54 arm 1 / arm 2

Flow 54's arms 1 and 2 are the axis's in-repo acceptance: arm 1 iterates `DESTROYING_DOORS` **through its
`Disposition` axis** over a manufactured plant-shape set, and arm 2 iterates
`{all move · some move · none move} × {unwind ok · fault on the pin · fault on a later member} × {task area ·
sub-task areas · milestone area}`. Both run in-process fixtures built by `trial_corpus.rs`, from the
**repository root**, on the debug binary.

This review adds four things they structurally cannot:

1. **The cwd axis, which is not in either arm's set.** Every fixture in `trial_corpus.rs` builds and drives
   from the repo root, so `{root · subdirectory · fan-out worktree · linked worktree}` is a dimension the
   suites do not iterate. That dimension is the whole subject of the rc.19 arc, and it is where **F-A** lives:
   arm 2's `none move` and `all move` cells are green at rc.19 *and* the same cells print host-absolute paths
   one cwd over.
2. **The shell-survival axis.** Every suite root comes from `std::env::temp_dir()`, which has no space in it —
   the fact the cwd-fix review itself named as the mask on its HIGH 1 and MEDIUM 3. R-6 and R-17 drive a
   repository root containing a space through `milestone execute`'s Spawn line, three destroying-door routes
   and `task discard`'s warning, and **run the emitted spans verbatim** through `sh -c`.
3. **Routes executed, not asserted.** The suites assert a route's *text*; R-5, R-6 and R-17 paste the emitted
   `git -C <abs> …` span into a shell of a different cwd and read its exit status. That is the difference
   between *the route says the right thing* and *the route works* — and it is the only way census C1-06's
   dead end could be shown closed.
4. **The release posture.** The suites run the debug binary, where the route fences panic; this review runs
   the installed release binary, where a bad span is emitted silently. Every verdict here is about the bytes
   an adopter actually receives.

It also re-drives the four still-open tier-3 rows on the shipped binary, so their triage to 1.x rests on a
current measurement rather than on rc.18's.

---

**Scope.** This is **axis 3 only**, on `1.0.0-rc.19`. The Codex source pass for this axis was **not read**
(reconciliation is a separate agent). No fix was applied, no commit made, nothing written into the working
repository.

<!-- END of the Opus driver's file. Everything below is the reconciler's. -->

---

## 7 · Reconciliation ledger

**The rule applied** (`acceptance-design.md` → *The reconciliation rule*): a claim by one pass that the other
cannot reproduce is a **lead, not a finding**. Every Codex claim below was entered as `lead(codex, …)` and then
**driven on `jigc 1.0.0-rc.19`** — to a repro block (CONFIRMED, origin codex) or to a recorded refutation with
the falsifying datum. Every driver **defect** the source pass is silent on was **re-driven once** by the
reconciler before being carried.

### 7.1 · The Codex source pass's two named claims

| # | lead(codex, …) | disposition | evidence |
|---|---|---|---|
| **C-1** | a symlink named as a staged document is classified **simultaneously** as managed prose and as foreign bytes — the refusal calls the link foreign while the `--force` ack calls the same entry dropped staged prose; the external target survives | **CONFIRMED** (origin codex; = the driver's **F-1**, and Codex's predicted `--force` half is **new evidence the driver did not carry**) | **R-23a** |
| **C-2** | the residual-area message asserts *"is a directory"* without testing that the area has that shape — `carries_base_pin` tests only `<area>/base.json`, `residual_area_note` says *directory* unconditionally | **CONFIRMED** (origin codex; = the driver's **F-2**, and the reconciler adds a **second door**, `task validate`, on the identical sentence) | **R-23b** |

**C-1 — what the drive adds over the driver's F-1.** The driver drove two surfaces (the refusal calls it
foreign; `doc list --task` calls it `managed`). Codex predicted a **third**, and it fires: the consent ack
names the same entry as jigc's own staged prose.

```
$ jigc task discard tidy-the-readme                                          -> exit 1
  blocking · task-discard.foreign-bytes — … holds 1 path(s) jigc did not write …
    .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md              <- A THIRD PARTY'S
$ jigc doc list --task tidy-the-readme                                       -> exit 0
  adr:via-symlink  docs/decisions/via-symlink.md  managed                    <- JIGC'S OWN
$ jigc task discard tidy-the-readme --force                                  -> exit 0
  discarded task tidy-the-readme — dropped staged edits to: adr:via-symlink,
    commit:tidy-the-readme (transient)                                       <- JIGC'S OWN, AGAIN
```

Three surfaces, two answers, one path. The external target is **1/1 before and after** — the finding is a
classification split, not a loss, which is why it stays tier 3.

**C-2 — what the drive adds over the driver's F-2.** The sentence is produced from one home
(`engine::state::residual_area_note`), so it is the same lie at every door that names a residual: the
reconciler drove `task validate` beside `task discard` and got the byte-identical sentence over the symlink.

### 7.2 · The Codex pass's dispositions, census and bounds

| # | lead(codex, …) | disposition | evidence |
|---|---|---|---|
| **C-3** | the registry contains **six** destroying doors, not four; the brief's four-door set is the narrower `WORKTREE_DOORS` subset | **CONFIRMED** (source datum, agrees with the driver's own code-derived door set) | `milestone.rs:3430` `DESTROYING_DOORS: [&DestroyingDoor; 6]` · `milestone.rs:3444` `WORKTREE_DOORS: [&DestroyingDoor; 4]`, both read at HEAD |
| **C-4** | M52 **A3-1** CLOSED — the milestone complement walks `merged/` and recognizes only regular staged bodies; teardown removes members non-recursively and cannot take a non-empty foreign remainder | **CONFIRMED** (driven, driver row 69 / R-13a: 8 planted loci, **8 before / 8 after**, each foreign directory moved whole) | driver R-13a |
| **C-5** | M52 **A3-2** CLOSED — both finalize paths displace first, then use registry-bounded `unwind_area`; `Foreign` or an error leaves the area standing and produces `finalize.foreign-bytes` | **CONFIRMED** (driven twice: driver rows 70 / R-13b **and** the reconciler's own none-move cell, R-24b — area left standing, one advisory per area, plant 1/1) | driver R-13b · **R-24b** |
| **C-6** | M52 **A3-3** CLOSED — both `WorkArea::Task` and `WorkArea::Milestone` carry explicit writer rows; milestone finalize passes both subjects through the shared teardown | **CONFIRMED** (driven, driver row 71 / R-13c — the same plant set finalized from inside a fan-out worktree, 3 before / 3 after, all parked) | driver R-13c |
| **C-7** | the post-review **operation-bearing-worktree HIGH** CLOSED — `probe_leftover` accepts only absent, empty, or clean-and-concluded; provision, discard and uninstall consume that shared probe | **CONFIRMED** (driven at all three doors: driver rows 9–10 (provision, second repo mid-bisect), 20–21 (discard, spotless + live bisect), 38 (uninstall, live bisect), and the emitted `git -C <ABS> bisect reset` **runs verbatim** from three cwds) | driver R-5 · R-12 · R-2 |
| **C-8** | M51 claim 1 CLOSED — task-discard foreign-complement refusal precedes recursive deletion | **CONFIRMED** (driven by the reconciler's own C-1 run: the refusal fires at exit 1 and the area is intact; only `--force` removes) | **R-23a** · driver R-8 |
| **C-9** | M51 claim 2 CLOSED — milestone discard uses the same task-area complement; only the consented `Take` arm retains `remove_dir_all` | **CONFIRMED** (driven, driver row 22 / R-13: the whole 5-locus complement incl. **two sub-task areas** named, **5 before / 5 after**) | driver R-13 |
| **C-10** | M51 claim 3 CLOSED — milestone finalize uses `Displace`, then `unwind_settled_area`, not recursive deletion | **CONFIRMED** (driven, driver rows 69–71) | driver R-13a/b/c |
| **C-11** | M51 §A `C-1`, `D-1` … `D-4` remain CLOSED — six-door enumeration, base-pin-derived ownership, acks keying on actual removal success, paths rendered relative to `jigc_home`, `symlink_metadata` distinguishing absent / directory / leaf / unreadable | **CONFIRMED in part, with one carried exception**: the *worktree-leftover* classifier does distinguish shapes (driver rows 4, 6, 8 — `File`, symlink-`File`, `Unreadable`), and acks are outcome-filtered (driver rows 42–43: the prune line is **absent** when nothing was pruned). **But `paths rendered relative to jigc_home` is exactly what F-A falsifies at `jigc task finalize`** — see 7.3 | driver R-9 · R-10 · **R-24a** |
| **C-12** | `--force` is scoped to the four refusing rows' declared populations; **the two finalizers expose no invented force consent** | **CONFIRMED** (driven bare: `jigc task finalize <id> --force` → **exit 2**, `error: unexpected argument '--force' found`; `jigc milestone finalize <id> --force` → **exit 2**, same) | **R-25** |
| **C-13** | no schema-manifest or schema-hash file changed in `20c18b74..4a862b96` — M52's zero-schema-hash-movement boundary is not violated | **CONFIRMED** (driven: `git diff --name-only 20c18b74..4a862b96` = **84 files**, and the `schema\|manifest\|pack/` filter returns **nothing**) | **R-26** |
| **C-14** | production removal sites are fully accounted for — destructive doors guarded, owned-artifact removals enumerated, and unguarded removals confined to minted temporaries / empty-directory pruning | **OPEN LEAD** — a **completeness assertion over the whole production surface**, not reducible to an argv, so it cannot be driven to a repro block; adjudicating it is its own sweep. Codex bounds it itself (*"source completeness only; no argv was driven"*). The reconciler's spot census is **consistent with it and did not falsify it**: 111 `remove_dir_all` textual hits over both crates' `src/`, of which the overwhelming majority are the in-module `#[cfg(test)]` temp-root destructor `remove_dir_all(&self.0)`; the one site Codex's account does **not** name — `invocation_log.rs:738` — was checked and is under `#[cfg(test)]` (line 565), i.e. not a reachable CLI door | **R-26** |

### 7.3 · The driver's defects, re-driven by the reconciler

The source pass is **silent on both** — it states *"No new completeness or tier-1 lead was found inside the
cwd-dependence arc"* and names neither. **Silence is not a contradiction**, so neither is recorded as refuted;
per the rule both stay findings (they were driven), and each was **re-driven once** by the reconciler to
confirm its repro block.

| id | driver's claim | reconciler's re-drive | status |
|---|---|---|---|
| **F-A** | `jigc task finalize`'s **Displace** surfaces render every path host-**absolute** from a branch-attached linked worktree — the stderr note, the **1.0-pinned** `committed.displaced[].from`/`.to`, and `finalize.foreign-bytes`'s `message` + `route` | **CONFIRMED, both arms, with the cwd control** — the all-move cell from `linked` gives `"from": "/private/var/…/repo/.jigc/tasks/…"`, **host-absolute hits 3**; the *identical fixture from the repo root* gives `".jigc/tasks/tidy-the-readme/notes.txt"`, **hits 0**. The none-move cell's `message` and `route` are absolute while `key` and `location` stay `task:tidy-the-readme`. Plant **1/1** before and after in both | **CONFIRMED · tier 2/3, not tier 1** | 
| **F-B** | the `LeftoverShape::File` consent arm lists the leftover's **own basename** in the child-entry position | **CONFIRMED, with the `Directory` control beside it** — `File`: *"removing the leftover file `.jigc/worktrees/area-one` … : **area-one**"*; `Directory`: the same sentence listing **`precious.txt`**, a real child. The refusal arm is correct in both (*"the file itself — it is a file, not a worktree"*) | **CONFIRMED · tier 3** |

**F-A is the falsifying datum for the `jigc_home`-relative half of C-11**, and the reconciler records it as
such rather than letting the two passes' halves stand side by side: the source pass read the rule's **one
home** (`render::repo_relative`) and concluded the rule holds; the drive shows one door **not reaching that
home** on one cwd. This is the shape M50's planning rule names — *a claim about how the composed product
behaves is not established by reading the files it is composed from*.

### 7.4 · The four still-open rc.18 rows

Both passes agree, from opposite directions, on all four:

| id | driver (driven) | codex (source) | reconciled |
|---|---|---|---|
| **F-1** | STILL-OPEN, re-driven R-20 | **STILL-OPEN(1.x, expected)**, claim 1 | **STILL-OPEN** — and now carrying a **third surface** (the `--force` ack) that neither pass had alone |
| **F-2** | STILL-OPEN, re-driven R-20 | **STILL-OPEN(1.x, expected)**, claim 2 | **STILL-OPEN** — and now driven at a **second door** (`task validate`) |
| **F-4** | STILL-OPEN, re-driven R-7 | not addressed | **STILL-OPEN** — carried on the driver's drive (`git show HEAD:keeper.md` returns the bytes after the run) |
| **F-5** | STILL-OPEN, all three halves, R-14 | not addressed | **STILL-OPEN** — carried on the driver's drive; its **reachability** stays undischarged (the state is built by restoring a `cp -R` backup; no jigc sequence reaches it) |

### 7.5 · Demotions — rows marked driven whose cited repro block does not evidence them

Four. Each was demoted on the rule, then **re-driven by the reconciler**; all four land where the driver filed
them, so the demotion changes the *provenance*, not the verdict.

| row | door | what the citation did not cover | re-drive |
|---|---|---|---|
| **12** | `milestone provision` | cited *"R-3 (sibling door, identical text) · R-14"* — R-3's body drives a `Directory` leftover, R-14's drives `jigc task discard "" / "../.."`. **Neither runs a malformed id at `milestone provision`.** The cell text admits it | **R-23c** — driven at the door: both `""` and `"../.."` → exit 1, `work-unit.malformed-id`, `.jigc/` intact |
| **30** | `milestone discard` | cited *"R-14 (same producer, driven at `task discard`)"* — an explicit sibling-door inference | **R-23c** — driven at the door, both ids, same code, same exit |
| **47** | `uninstall` | the row's load-bearing half is *"quoted under a **spaced** root"*; R-1's body is unspaced and R-17's spaced block shows the `untracked-workbench-file` **route**, not the tracked-file **restore note** | **R-24c** — driven under a root containing a space; the note emits `git -C '/…/jigc space.…/repo' checkout -- <path>`, **single-quoted** |
| **48** | `uninstall` | the row's subject is the **route line's** unfilled `<milestone-id>`; R-2's body prints the finding's message and `at:` but not its route | **R-25** — driven; the route reads verbatim ``… abandon the milestone with `jigc milestone discard <milestone-id> --force` …`` while the same run's worktree is registered under the known milestone `romeo-probe`. **OBS-1 stands as an observation, not a defect** |

---

## 8 · The reconciler's repro blocks (R-23 … R-26)

All on `jigc 1.0.0-rc.19`, each in its own `dev/jigc-rig` throwaway repo (`rig=$(dev/jigc-rig <state>
--binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps, never `eval "$(…)"`). Roots from `mktemp -d`,
so there is no teardown and the `rm -rf $V/$D` shape appears nowhere. Exit codes read **bare**.

### R-23 — the two Codex claims driven, and the two demoted malformed-id cells (C-1, C-2, rows 12, 30)

```
### R-23a (C-1 / F-1) — fresh; jigc start --workflow quick-fix "tidy the readme"
  OUT=$(mktemp -d); printf 'OUTSIDE-TARGET-BYTES\n' > $OUT/body.md
  ln -s $OUT/body.md .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
  ls -l -> lrwxr-xr-x adr:via-symlink.md -> /var/folders/…/outside.nB3cRF/body.md
  BEFORE outside target: 1

  (cd docs/deep) jigc doc list --task tidy-the-readme                         -> exit 0
      adr:via-symlink  docs/decisions/via-symlink.md  managed
  (cd docs/deep) jigc task discard tidy-the-readme                            -> exit 1
      blocking · task-discard.foreign-bytes — task `tidy-the-readme`'s working area holds 1 path(s)
        jigc did not write …:  .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
  (cd docs/deep) jigc task discard tidy-the-readme --force                    -> exit 0
      warning: removing the working area .jigc/tasks/tidy-the-readme discards work that is not in git:
          .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
      discarded task tidy-the-readme — dropped staged edits to: adr:via-symlink,
        commit:tidy-the-readme (transient)          <- THE THIRD SURFACE Codex predicted
  AFTER outside target: 1 (intact) ; .jigc/tasks/ empty

### R-23b (C-2 / F-2) — fresh; mkdir -p .jigc/tasks
  OUT=$(mktemp -d); printf 'OUTSIDE-KEEP\n' > $OUT/keep.txt; ln -s $OUT .jigc/tasks/ghost
  mkdir -p .jigc/tasks/ghost-dir                                   # the control
  ls -l .jigc/tasks -> lrwxr-xr-x ghost -> /var/folders/…  ·  drwxr-xr-x ghost-dir
  BEFORE outside: 1

  (cd docs/deep) jigc task discard  ghost      -> 1  finalize.no-task
      "no task `ghost`: `.jigc/tasks/ghost` IS A DIRECTORY carrying no base pin, so it is a
       leftover and not a work unit …"                             <- it is a SYMLINK
  (cd docs/deep) jigc task validate ghost      -> 1  BYTE-IDENTICAL sentence   <- a SECOND door
  (cd docs/deep) jigc task discard  ghost-dir  -> 1  the same sentence, TRUE there   <- CONTROL
  AFTER outside: 1 (intact) ; both entries still on disk
  NOTE: a first attempt on a `fresh` rig failed because `.jigc/tasks/` does not exist until a task
        is minted — `ln -s` errored and the probe drove the plain "no task `ghost`" arm. Re-driven
        with `mkdir -p .jigc/tasks` first. Recorded rather than dropped.

### R-23c (rows 12, 30 — the demotions, driven at their own doors)
  fresh; milestone create "Romeo probe"; milestone add-task romeo-probe "area one"; cwd = docs/deep
  jigc milestone provision ""       -> 1  work-unit.malformed-id — "" is not a valid work-unit id
  jigc milestone provision "../.."  -> 1  work-unit.malformed-id — "../.." is not a valid work-unit id
  jigc milestone discard   ""       -> 1  work-unit.malformed-id — "" …
  jigc milestone discard   "../.."  -> 1  work-unit.malformed-id — "../.." …
  route each time: use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)
  .jigc after: AGENT.md config milestones state tasks version            <- intact
```

### R-24 — F-A re-driven with its cwd control, F-B re-driven with its shape control, and row 47 (F-A, F-B, C-5, C-11, row 47)

```
### R-24a (F-A) — fresh; git -C $REPO worktree add -b feat $REPO/../linked
  (cd linked) jigc start --workflow quick-fix "tidy the readme"; echo code > linked/src.txt; git add
  the commit doc filled from `linked` (set-slot #summary, set-field #type chore — exit 0 both)
  printf 'KEEPHP\n' > $REPO/.jigc/tasks/tidy-the-readme/notes.txt        BEFORE 1

  (cd linked) jigc --format json task finalize tidy-the-readme                -> exit 0
    committed.displaced = [ {
      "from": "/private/var/folders/…/jigc-rig-fresh-xeeFA3/repo/.jigc/tasks/tidy-the-readme/notes.txt",
      "to":   "/private/var/folders/…/jigc-rig-fresh-xeeFA3/repo/.jigc/displaced/tidy-the-readme/notes.txt"
    } ]                                                          <- the 1.0-PINNED key, HOST-ABSOLUTE
    stderr  "… they were moved aside, not taken:"  + the same two absolutes
    host-absolute hits (stdout+stderr): 3      AFTER 1 (parked under .jigc/displaced/)
  CONTROL, the identical fixture from the REPO ROOT                          -> exit 0
    committed.displaced = [{"from": ".jigc/tasks/tidy-the-readme/notes.txt",
                            "to":   ".jigc/displaced/tidy-the-readme/notes.txt"}]
    stderr  the same pair, repo-relative
    host-absolute hits: 0                       <- THE CWD IS THE VARIABLE

### R-24b (F-A none-move arm; also C-5's standing-area half)
  the same fixture from `linked`, plus  printf 'NOT A DIR' > $REPO/.jigc/displaced
  (cd linked) jigc --format json task finalize tidy-the-readme                -> exit 0
    findings[0].code   finalize.foreign-bytes
    findings[0].message "`/private/var/…/repo/.jigc/tasks/tidy-the-readme` holds 1 path(s) jigc did not
        write, so the working area was left standing … : `/private/var/…/notes.txt`; 1 of them could not
        be moved aside: `/private/var/…/notes.txt` — could not open /private/var/…/.jigc/displaced/
        tidy-the-readme to park it: Not a directory (os error 20)"
    findings[0].route  "… Keep what you need from `/private/var/…/repo/.jigc/tasks/tidy-the-readme` …"
    findings[0].key    {"code":"finalize.foreign-bytes","target":"task:tidy-the-readme"}      <- CLEAN
    findings[0].location {"address":"task:tidy-the-readme","col":1,"line":1}                  <- CLEAN
    committed.displaced []          host-absolute hits: 3        BEFORE 1 / AFTER 1 (area standing)

### R-24c (F-B, with the shape control; and row 47's spaced half)
  F-B   — fresh; milestone create "Quebec probe" + one sub-task (NOT provisioned)
          printf 'LEFTOVER-FILE-BYTES\n' > .jigc/worktrees/area-one   (file -b -> "ASCII text")
    jigc milestone provision quebec-probe            -> 1  milestone.leftover-holds-work
        .jigc/worktrees/area-one: the file itself — it is a file, not a worktree, and nothing can
          say those bytes are disposable                              <- the refusal arm, CORRECT
    jigc milestone provision quebec-probe --force    -> 0
        warning: removing the leftover file .jigc/worktrees/area-one discards work that is not in git:
            area-one                                                  <- THE PATH'S OWN BASENAME
          note: the leftover file is the only copy of these bytes — they are not recoverable.
        provisioned 1 worktree(s) for milestone:quebec-probe at base efc2761 (area-one)
  CONTROL — the same two arms over a DIRECTORY leftover holding precious.txt:
        refusal: "precious.txt — git reports no worktree of its own there …"
        --force: "removing the leftover directory … :  precious.txt"   <- A REAL CHILD

  row 47 — SPACED root: SCRATCH=$(mktemp -d "$TMPDIR/jigc space.XXXXXX"); TMPDIR=$SCRATCH/; fresh
    REPO=/var/folders/…/jigc space.koAkAG/jigc-rig-fresh-WfozZw/repo
    (cd docs/deep) jigc uninstall                                            -> exit 0
      warning: removing `.jigc/` also removes 5 tracked file(s) under it: …
        note: each is in the index, so `git -C '/private/var/…/jigc space.koAkAG/…/repo' checkout
          -- <path>` brings it back.                                  <- SINGLE-QUOTED
      `command grep -cE "git -C '.*jigc space"` -> 1
```

### R-25 — row 48's route line, and the finalizers' absent `--force` (row 48, C-12)

```
fresh; milestone create "Romeo probe" + "area one"; milestone provision romeo-probe -> 0
echo dirty > $W/f.txt ; git -C $W add f.txt

$ jigc uninstall                                                             -> exit 1
  blocking · uninstall.dirty-worktree — `.jigc/` holds 1 fan-out sub-task worktree path(s) that
    removing it would destroy and nothing can say are disposable:
    .jigc/worktrees/area-one: A  f.txt — it is a live git worktree holding uncommitted work,
      registered here or not; registered as a worktree of this repository
    route: get the work out of those paths first (commit, stash, or copy it), then re-run
      `jigc uninstall` — or abandon the milestone with `jigc milestone discard <milestone-id>
      --force`, which destroys the uncommitted work in the path(s) registered here, and re-run
      `jigc uninstall`; `jigc uninstall --force` deletes them with the install
                                        ^^^^^^^^^^^^^^  UNFILLED, while the run knows `romeo-probe`

$ jigc task finalize nonexistent --force          (exit read BARE)          -> exit 2
    error: unexpected argument '--force' found ; tip: a similar argument exists: '--format'
    Usage: jigc task finalize --format <FORMAT> <ID>
$ jigc milestone finalize romeo-probe --force     (exit read BARE)          -> exit 2
    the same rejection, Usage: jigc milestone finalize --format <FORMAT> <MILESTONE_ID>
  -> neither finalizer exposes a force consent (C-12 confirmed). NOTE: a first attempt read these
     through `| head -4` and got head's 0 — the exact trap this repo's shell rule names. Re-read bare.
```

### R-26 — the two source-side claims (C-3, C-13, C-14)

```
$ git diff --name-only 20c18b74..4a862b96 | wc -l                      -> 84
$ git diff --name-only 20c18b74..4a862b96 | grep -iE 'schema|manifest|pack/'  -> (nothing, rc=1)
  -> C-13 CONFIRMED: no schema-manifest, schema-hash, schema-snapshot or pack file in the range.

$ sed -n '3430,3455p' crates/cli/src/milestone.rs
    pub const DESTROYING_DOORS: [&DestroyingDoor; 6] = [ &PROVISION_DOOR, &DISCARD_DOOR,
      &UNINSTALL_DOOR, &TASK_DISCARD_DOOR, &TASK_FINALIZE_DOOR, &FINALIZE_DOOR ];
    pub const WORKTREE_DOORS:   [&DestroyingDoor; 4] = [ &PROVISION_DOOR, &DISCARD_DOOR,
      &UNINSTALL_DOOR, &FINALIZE_DOOR ];
  -> C-3 CONFIRMED at the source, and it agrees with the driver's own code-derived door set.

$ command grep -rc 'remove_dir_all' crates/*/src/*.rs                  -> 111 hits over 30 files
  the overwhelming majority are the in-module #[cfg(test)] temp-root destructor
  `remove_dir_all(&self.0)`; the one site Codex's account does not name:
$ command grep -n '#\[cfg(test)\]' crates/cli/src/invocation_log.rs    -> 565
$ command grep -n 'remove_dir_all'  crates/cli/src/invocation_log.rs   -> 738
  738 > 565  -> test-only, not a reachable CLI door.
  -> C-14 stays an OPEN LEAD: a completeness assertion over the production surface is not
     reducible to an argv. Not falsified; not promoted on a source read.
```

---

## 9 · Doors covered

Every clap leaf that is the door of **≥ 1 driven row** in this reconciled file, in `VERB_KINDS` spelling
(`crates/cli/src/cli.rs:1835`). Fixture-construction calls (`jigc setup` via the rig, `jigc doc set-slot` /
`doc set-field` filling a commit doc, `jigc start --workflow` minting a task) are **excluded** — they are how a
cell was built, not a cell.

| door (`VERB_KINDS` spelling) | rows |
|---|---|
| `milestone provision` | 1–17 · R-23c |
| `milestone discard` | 18–30 · R-23c |
| `uninstall` | 31–48 · R-24c · R-25 |
| `task discard` | 49–59 · R-23a · R-23b |
| `task finalize` | 60–68 · R-24a · R-24b · R-25 |
| `milestone finalize` | 69–80 · R-25 |
| `task list` | 81, 82 |
| `task validate` | 83 · R-23b |
| `task diff` | 84 |
| `start` | 85, 86 |
| `milestone create` | 87, 88 |
| `milestone add-task` | 89 |
| `milestone join` | 90 |
| `milestone list-tasks` | 91 |
| `milestone execute` | 91, 92 |
| `doc list` | 93 · R-23a |
| `validate` | 93 |
| `rename` | 94, 95 |

**18 doors.** The six `DESTROYING_DOORS` members are all covered, and `WORKTREE_DOORS`' four are covered at
both arms of their `Refuse{--force}` disposition.

**Not covered, stated rather than implied** (the driver's §5 carries the full list and its reasons):
`Disposition::Narrate` has no member at rc.19 and therefore no cell; `task finalize --force` does not exist
(driven, R-25); the hook-rejection cells at the two `Displace` doors are axis 4's subject; a genuine concurrent
racer at any destroying door is driven by nothing here; and `GIT_DIR` redirect is M51's declared posture
residual.
