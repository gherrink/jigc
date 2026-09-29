<!-- M51 per-axis review — axis 3 · reconciled · driven on the installed `jigc 1.0.0-rc.15` (commit 577a0099), 2026-09-16 -->

# M51 per-axis review — AXIS 3 · destroying doors — the Opus driver's `(door, cell)` table

**Binary.** `/Users/maurice/.local/bin/jigc` — asserted first, before anything else:

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.15
```

This is the **release** posture (the route-fence panics that exist in the debug binary do not
exist here), and it is the binary built **after** the M51 audit fixes — routes `6c2391c0`,
orphan territory `da5173a1`, setup guard `ff2bde99`, LOWs `507c332d`
([VERDICT](../../../../completions/artifacts/M51/VERDICT.md)). Every row below was driven on it.

**Rigs.** `rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
(two-step eval), then a `milestone create` + two `milestone add-task` through the binary. That
construction is the script `scratchpad/axis-review/driver/mkrig.sh`; it prints `REPO=<path>|HOME=<path>`
and nothing else. Every fixture state below was built **by driving the binary** — nothing was
written into `.jigc/` by hand except the *plants* (a leftover at a worktree path, a workbench
file, a `chmod`), which is what the axis is about. Rigs are labelled **A–Q**; there is no
teardown and none is needed (`mktemp -d` roots).

**Scope note.** The Codex source pass for this axis was **not** read (reconciliation is a
separate agent). Nothing below rests on a source read: a rule cited from the code or a design
doc is only ever the *contract* a driven row is judged against.

---

## 1 · The door set and the cell set, derived from the code

**Read verbatim, with the count each declaration carries:**

| registry | file:symbol | count read | members |
|---|---|---|---|
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:2625` | **4** (the type *is* the count: `[&DestroyingDoor; 4]`) | `PROVISION_DOOR` · `DISCARD_DOOR` · `UNINSTALL_DOOR` · `FINALIZE_DOOR` |
| `LEFTOVER_VERDICTS` | `crates/cli/src/milestone.rs:2510` | **3** (`[LeftoverVerdict; 3]`) | `Unverifiable` · `OwnWorktree` · `NoOwnLinkage` |
| `LeftoverShape` | `crates/cli/src/milestone.rs` (enum) | **3** variants | `Directory` · `File` · `Unreadable(String)` |
| axis-3 door set | `DESTROYING_DOORS` ∪ `jigc task discard` | **5 doors** | see below |

**The five doors, with the refusal identity each carries** (`DestroyingDoor::code` is the
axis's refuse-vs-narrate discriminator; the second and third codes per door are the *other
subjects* the same door guards, read from the code and each driven):

| door (VERB_KINDS spelling) | `DestroyingDoor::code` | other blocking identities at this door |
|---|---|---|
| `milestone provision` | `milestone.leftover-holds-work` | `milestone.provision-failed` (phase 2) |
| `milestone discard` | `milestone.dirty-worktree` | `milestone.staged-prose` |
| `uninstall` | `uninstall.dirty-worktree` | `uninstall.staged-prose` · `uninstall.untracked-workbench-file` · `uninstall.remove-jigc` |
| `milestone finalize` | **`None`** — the declared narrate-only member | `milestone.zero-contribution` (not a destruction refusal) |
| `task discard` | *(not a `DESTROYING_DOORS` row)* | `task-discard.staged-prose` |

**Cell set driven** (the acceptance design's `LeftoverShape × {staged prose · untracked
workbench file · clean} × {no --force, --force}`, expanded on the one axis the design's cell
names compress — a leftover's **verdict**, which is what decides the refusal's sentence):

- **S1** `Directory` / `NoOwnLinkage`, non-empty · **S2** `Directory` / `Unverifiable` (a
  dangling `.git` file), non-empty · **S3** `Directory` / `OwnWorktree`, **dirty** · **S4**
  `Directory` / `OwnWorktree`, **clean** · **S5** `File` (plain file; and a **symlink**, which
  the code declares is a `File` whatever it points at) · **S6** `Unreadable` (dir `chmod 000`)
  · **S7** staged prose (`.jigc/tasks/<id>/docs/*.md`) · **S8** untracked workbench file (the
  `ENTRIES` complement under `.jigc/`) · **S9** clean · **S10** the fail-closed *probe* cells
  (an unreadable `docs/` dir; an unreadable `.jigc/worktrees/` **root**) · **S11** the
  removal-fails cells (read-only parent → partial removal; read-only leftover → no removal).

**Other registries read, as instructed** (none is axis 3's, so each is reported as a count and
the method, not as a driven row): `VERB_KINDS` **47** leaves · `PATH_ARG_OCCURRENCES` **14** ·
`DOCTYPE_DOORS` **16** rows · `SLUG_DOORS` **6** · `WORK_UNIT_ID_DOORS` **25** ·
`BEHALF_DOORS` **47** · `COMMITTING_DOORS` **10** · `ENVELOPE_ARMS` **60** ·
`STORE_EXIT_FLIPS` **6** · `ManifestKind::ALL` **6** · `SchemaChangeKind` **18** variants.
(Counted by balanced-bracket top-level-item parse of each declaration, comments stripped;
`STORE_EXIT_FLIPS` and `ManifestKind` by literal count against the declared `ALL` length.)

---

## 2 · The table — 60 driven rows over 5 doors

Every row's argv ran on the installed `1.0.0-rc.15`; its exit, code, route kind and the asserted
surface line are recorded, and the repro block for its cell is in §3. **Route kind** is read off
the rendered finding: `Mechanical` = the `route:` line opens with a backticked argv,
`Human` = prose naming the acts, `Informational` = neither, `none` = no finding.

| # | door | cell | argv (driven) | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `milestone provision` | S1 dir, no `--force` | `jigc milestone provision axis-three-probe` | 1 | `milestone.leftover-holds-work` | Mechanical | `.jigc/worktrees/first-sub: precious.txt — git reports no worktree of its own there…`; plant on disk after | matches contract |
| 2 | `milestone provision` | S1, `--format json` | `… --format json` | 1 | same, flattened | Mechanical (inside the message) | top-level `{"error": …}`, stdout empty | matches contract (the declared `finding_to_err` complement) |
| 3 | `milestone provision` | S1, `--force` | `… --force` | 0 | none | none | `warning: removing the leftover directory … precious.txt` + `not recoverable`, then `provisioned 2 worktree(s)` | matches contract |
| 4 | `milestone provision` | S5 file, no `--force` | `jigc milestone provision axis-three-probe` | 1 | `milestone.leftover-holds-work` | Mechanical | `…: the file itself — it is a file, not a worktree` | matches contract |
| 5 | `milestone provision` | S5 **symlink**, no `--force` | same | 1 | same | Mechanical | identical `the file itself` wording; `readlink` unchanged | matches contract |
| 6 | `milestone provision` | S6 unreadable, no `--force` | same | 1 | same | Mechanical | `…: unknown — could not read the leftover directory …: Permission denied (os error 13)` | matches contract |
| 7 | `milestone provision` | S2 unverifiable, no `--force` | same | 1 | same | Mechanical | `…: .git, wip.txt — git cannot read a repository there` | matches contract |
| 8 | `milestone provision` | S9 clean, no `--force` | same | 0 | none | none | `provisioned 2 worktree(s) … at base <sha>` | matches contract |
| 9 | `milestone provision` | S9 clean, `--force` (inert) | `… --force` | 0 | none | none | same ack, no warning | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-1 |
| 10 | `milestone provision` | S4 own clean worktree (re-run) | same | 0 | none | none | idempotent reuse ack | matches contract |
| 11 | `milestone provision` | S3 own **dirty** worktree (re-run) | same | 0 | none | none | ack; `wip.txt` byte-intact after | matches contract |
| 12 | `milestone provision` | S7 staged prose present | same | 0 | none | none | ack; not this door's subject | matches contract (n/a proven by driving) |
| 13 | `milestone provision` | S8 untracked workbench file present | same | 0 | none | none | ack; `.jigc/notes.txt` intact | matches contract (n/a proven by driving) |
| 14 | `milestone provision` | S5+S6 **both** planted | same | 1 | `milestone.leftover-holds-work` | Mechanical | **both** paths listed, one line each (RC-m50 N9's collect-all) | matches contract |
| 15 | `milestone provision` | S11 partial removal (read-only **parent**) | `… --force` | 1 | `milestone.provision-failed` | Mechanical, **echoing `--force`** | `0 of 2 worktree(s) landed`; loss warning names `precious.txt`, which **is** gone | matches contract |
| 16 | `milestone provision` | S11 **no** removal (read-only leftover) | `… --force` | 1 | `milestone.provision-failed` | Mechanical | **no loss warning at all**; `precious.txt` survives | matches contract (outcome-keyed narration) |
| 17 | `milestone provision` | S5 symlink → tree outside the repo, `--force` | `… --force` | 0 | none | none | `removing the leftover file …`; the **outside** tree intact | matches contract |
| 18 | `milestone discard` | S3 dirty worktree **+** S7 staged prose | `jigc milestone discard axis-three-probe` | 1 | `milestone.dirty-worktree` | Human | worktree arm wins precedence; per-path fate `registered here, so the teardown removes it` | matches contract |
| 19 | `milestone discard` | same, `--format json` | `… --format json` | 1 | flattened | Human (in message) | `{"error": …}`, stdout empty | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-2 |
| 20 | `milestone discard` | S7 staged prose alone | `jigc milestone discard axis-three-probe` | 1 | `milestone.staged-prose` | Human | `first-sub: adr:sub-decision`; route names `jigc milestone finalize <id>` | matches contract |
| 21 | `milestone discard` | S7, `--force` | `… --force` | 0 | none | none | `warning: … discards the staged docs of 1 open task(s)` + record commit + teardown; unrelated task and `.jigc/notes.txt` survive | matches contract |
| 22 | `milestone discard` | S5+S6 both, no `--force` | `jigc milestone discard axis-three-probe` | 1 | `milestone.dirty-worktree` | Human | both paths + `not registered here, so the teardown leaves it on disk` | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-3 |
| 23 | `milestone discard` | S2 unverifiable, no `--force` | same | 1 | `milestone.dirty-worktree` | Human | `git cannot read a repository there` | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-2 |
| 24 | `milestone discard` | S2 unverifiable **unregistered**, `--force` | `… --force` | 0 | none | none | exit 0, **no** loss narrated, path + bytes survive — the refusal's claim held | matches contract |
| 25 | `milestone discard` | S3 **registered** dirty worktree, `--force` | `… --force` | 0 | none | none | `warning: removing the fan-out worktree … wip.txt (never staged) … not recoverable` | matches contract — **DEMOTED (partial)**: R-G carries this surface text verbatim, but in the failed-teardown state, not this row's. **Re-driven by reconciliation**, block RR-7 |
| 26 | `milestone discard` | S4 clean live worktrees, no `--force` | `jigc milestone discard axis-three-probe` | 0 | none | none | exit 0, stderr empty, worktrees cleared | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-4 |
| 27 | `milestone discard` | S9 clean, no `--force` | same | 0 | none | none | `discarded milestone:… (2 sub-task(s); workbench removed)` | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-4 |
| 28 | `milestone discard` | S10 unreadable `docs/` of an **unrelated** task | same | 0 | none | none | exit 0 — the probe is scoped to *this* milestone's sub-tasks | matches contract |
| 29 | `milestone discard` | S11 teardown fails (read-only parent), `--force` | `… --force` | **0** | none (2 warnings) | Informational (`remedy:` line, ×2) | ack says **`workbench removed`** while both worktrees are still on disk | **DEFECT D-2** |
| 30 | `uninstall` | S5+S6 both, no `--force` | `jigc uninstall` | 1 | `uninstall.dirty-worktree` | Human | both paths + `registered as a worktree nowhere in this repository` | matches contract — **DEMOTED (partial)**: R-P records this state's exit + stream bytes; the asserted surface text appears nowhere. **Re-driven by reconciliation**, block RR-5 |
| 31 | `uninstall` | S2 unverifiable, no `--force` | same | 1 | `uninstall.dirty-worktree` | Human | `git cannot read a repository there` | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-5 |
| 32 | `uninstall` | S3 registered dirty worktree + S7 + S8 all present | same | 1 | `uninstall.dirty-worktree` | Human | precedence 1 of 3; route names `milestone discard --force` **and** `uninstall --force` | matches contract — **DEMOTED (partial)**: R-I records the exit + code; the asserted route text appears nowhere. **Re-driven by reconciliation**, block RR-7 |
| 33 | `uninstall` | S7 staged prose alone | same | 1 | `uninstall.staged-prose` | Human | `tidy-the-readme: commit:tidy-the-readme` | matches contract (route: see **D-1**) |
| 34 | `uninstall` | S8 untracked workbench file alone | same | 1 | `uninstall.untracked-workbench-file` | Human | `.jigc/notes.txt` | matches contract |
| 35 | `uninstall` | S8 **tracked-but-modified** `.jigc/config/packs.yaml` | same | 1 | `uninstall.untracked-workbench-file` | Human | both `packs.yaml` and `notes.txt` named (the *bytes*, not membership, leg) | matches contract |
| 36 | `uninstall` | S8 route **followed verbatim** (`git add` both) | `git add … && jigc uninstall` | 0 | none | none | exit 0; narration names **6** tracked files + `git checkout -- <path>` | matches contract (route runs and clears) |
| 37 | `uninstall` | S9 clean, no `--force` | `jigc uninstall` | 0 | none | none | `- removed .jigc/` … 7-bullet teardown | matches contract |
| 38 | `uninstall` | S9 clean, `--format json` | `… --format json` | 0 | none | none | success envelope (`removed`, `findings: []`, `line_file`, …) on stdout, warning on stderr | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-6 |
| 39 | `uninstall` | S9 second run (idempotency) | `jigc uninstall` | 0 | none | none | `(nothing to remove — no repo-local jigc install was present)` | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-6 |
| 40 | `uninstall` | S9 second run, `--force` | `… --force` | 0 | none | none | identical no-op | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-6 |
| 41 | `uninstall` | S3+S7+S8 all three, `--force` | `… --force` | 0 | none | none | **four** narration lines, each with its own recoverability claim; `.jigc/` gone | matches contract |
| 42 | `uninstall` | S4 clean live worktree, no `--force` | `jigc uninstall` | 0 | none | none | exit 0 — a clean worktree clears | matches contract — **DEMOTED**: no repro block in §3. **Re-driven by reconciliation**, block RR-6 |
| 43 | `uninstall` | S5 symlink → outside tree, `--force` | `… --force` | 0 | none | none | link removed and named; the outside tree intact | matches contract |
| 44 | `uninstall` | S6, `--force` → `remove_dir_all` fails | `… --force` | 1 | `uninstall.remove-jigc` | Human | partial removal; the 5 tracked files narrated as taken (**true**) | matches contract |
| 45 | `uninstall` | row 44's route **followed verbatim** | `chmod 755 …; jigc uninstall` | 1 | `uninstall.dirty-worktree` | Human | a second, self-describing refusal that itself names `--force` | matches contract (two-hop; see note O-4) |
| 46 | `uninstall` | S10 unreadable **`docs/`** (fail-closed) | `jigc uninstall` | 1 | `uninstall.staged-prose` | Human | `cannot check .jigc/tasks/ … Permission denied`; **absolute host path** in the message | **DEFECT D-3** |
| 47 | `uninstall` | S10 unreadable `.jigc/worktrees/` **root** | `jigc uninstall` | 1 | `uninstall.dirty-worktree` | Human | route says *make sure `git` is on PATH*, never names `--force` | **DEFECT D-4** |
| 48 | `uninstall` | refusal, `--format json` | `jigc uninstall --format json` | 1 | `uninstall.dirty-worktree` | Human | **bare `Finding`** at top level (`severity, probe, check, code, key, message, location, route`), stderr, stdout empty | matches contract, with tension (**O-1**) |
| 49 | `milestone finalize` | S4 landed, an **unstaged** file in a worktree | `jigc milestone finalize axis-three-probe` | 0 | none (never refuses) | none | stderr `warning: removing the fan-out worktree … scratch.txt (never staged)`; stdout `discarded with the fan-out worktrees (not committed, not recoverable)` | matches contract (`code: None`) |
| 50 | `milestone finalize` | same, `--format json` | `… --format json` | 0 | none | none | `committed.sub_tasks[].discarded = [{"path":"scratch.txt","state":"never-staged"}]` | matches contract |
| 51 | `milestone finalize` | S1+S5 **unregistered** leftovers under `.jigc/worktrees/` | same | 0 | none | none | both survive byte-intact and are **not named** — the declared "cannot reach this shape" | matches contract |
| 52 | `milestone finalize` | nothing to land | same | **3** | `milestone.zero-contribution` | Mechanical | names `provision` + `discard` as the two exits | matches contract (not a destruction refusal) |
| 53 | `task discard` | S7 staged prose, no `--force` | `jigc task discard tidy-the-readme` | 1 | `task-discard.staged-prose` | Human | names the doc identity; route names `doc show`, `task finalize`, `--force` | **DEFECT D-1** (the `task finalize` leg) |
| 54 | `task discard` | S7, `--format json` | `… --format json` | 1 | flattened | Human (in message) | `{"error": …}` | matches contract |
| 55 | `task discard` | S7 on a **milestone sub-task**, no `--force` | `jigc task discard first-sub` | 1 | `task-discard.staged-prose` | Human | same route, and the state refuses its landing verb | **DEFECT D-1** |
| 56 | `task discard` | S7, `--force`, `--format json` | `… --force --format json` | 0 | none | none | `{"commit":null,"dropped":["commit:tidy-the-readme"],"findings":[],"op":"task-discard","task":…}` | matches contract |
| 57 | `task discard` | S7 sub-task, `--force` | `jigc task discard first-sub --force` | 0 | none | none | `discarded task first-sub — dropped staged edits to: adr:sub-decision` + `record commit: <sha>` | matches contract |
| 58 | `task discard` | S9 sub-task, no staged docs | `jigc task discard first-sub` | 0 | none | none | `discarded task first-sub` + record commit | matches contract |
| 59 | `task discard` | S9, `--force` (inert) | `… --force` | 0 | none | none | identical ack | matches contract |
| 60 | `task discard` | S10 unreadable `docs/` (fail-closed) | `jigc task discard tidy-the-readme` | 1 | `task-discard.staged-prose` | Human | `cannot check task … working area`; **absolute host path**; route names the *task* door, not `uninstall` | **DEFECT D-3** |

*(All 60 rows are **driven** — each argv ran on the installed `1.0.0-rc.15` and its verdict is
recorded with a repro block in §3. Rows 2, 19, 45, 48 and 54 re-drive a state an earlier row
already reached, and are kept as their own rows because the **surface asserted** is different
(the `--format json` envelope, or a route followed to its next state); discounting those five,
the table covers **55 distinct `(door, cell)` pairs**.)*

---

## 3 · Repro blocks

Every block is `setup · argv · observed`. All were run with `HOME` repointed at the rig's home
and cwd at the rig's `$REPO`, per `dev/jigc-rig`'s eval.

### R-A — `milestone provision` over each leftover shape (rows 1–14)

```
setup:  rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
        jigc milestone create "axis three probe"; jigc milestone add-task axis-three-probe "first sub"
        jigc milestone add-task axis-three-probe "second sub"
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
        (stdout 0 bytes, stderr 720 bytes)
        `ls .jigc/worktrees/first-sub` -> precious.txt        # nothing removed

argv:   jigc milestone provision axis-three-probe --format json
exit:   1
out:    { "error": "blocking · milestone.leftover-holds-work — …" }      # one key, on stderr

argv:   jigc milestone provision axis-three-probe --force
exit:   0
out:    warning: removing the leftover directory .jigc/worktrees/first-sub discards work that is
          not in git:
              precious.txt
          note: the leftover directory is the only copy of these bytes — they are not recoverable.
        provisioned 2 worktree(s) for milestone:axis-three-probe at base 3c0fb4e (first-sub, second-sub)

# the same argv, over each other shape, in a fresh rig (only the plant differs):
plant:  printf 'leaf bytes\n' > .jigc/worktrees/first-sub
out:    .jigc/worktrees/first-sub: the file itself — it is a file, not a worktree, and nothing can
          say those bytes are disposable                                             exit 1
plant:  ln -s /etc/hosts .jigc/worktrees/first-sub
out:    …: the file itself — …                       exit 1   (readlink after: /etc/hosts)
plant:  mkdir -p .jigc/worktrees/first-sub/inner && chmod 000 .jigc/worktrees/first-sub
out:    …: unknown — could not read the leftover directory `.jigc/worktrees/first-sub`:
          Permission denied (os error 13) — nothing could be read there, …               exit 1
plant:  printf 'gitdir: /nonexistent/admin\n' > .jigc/worktrees/first-sub/.git; printf 'work\n' > …/wip.txt
out:    …: .git, wip.txt — git cannot read a repository there, …                          exit 1
plant:  (nothing)
out:    provisioned 2 worktree(s) …                                                       exit 0

# collect-all (RC-m50 N9), two shapes at once:
plant:  printf 'leaf\n' > .jigc/worktrees/first-sub
        mkdir -p .jigc/worktrees/second-sub/inner && chmod 000 .jigc/worktrees/second-sub
out:    "would delete 2 path(s)…" with BOTH lines, `at:` on the first                     exit 1
```

### R-B — `milestone provision`'s idempotent reuse, and the subjects that are not its own (rows 10–13)

```
setup:  (rig A, after `--force` provisioned both worktrees)
argv:   jigc milestone provision axis-three-probe                 -> exit 0, reuse ack
        printf 'uncommitted\n' > .jigc/worktrees/first-sub/wip.txt
        jigc milestone provision axis-three-probe                 -> exit 0; `cat …/wip.txt` = uncommitted
        (cd .jigc/worktrees/first-sub && jigc doc create adr --title "a sub decision" --task first-sub)
        jigc milestone provision axis-three-probe                 -> exit 0   # staged prose is not its subject
        printf 'private\n' > .jigc/notes.txt
        jigc milestone provision axis-three-probe                 -> exit 0; notes.txt intact
```

### R-C — the two removal-outcome cells at `provision --force` (rows 15–16)

```
setup:  fresh rig; mkdir -p .jigc/worktrees/first-sub; printf 'precious\n' > …/precious.txt

# (a) read-only PARENT — remove_dir_all takes the child, fails to unlink the dir
argv:   chmod 500 .jigc/worktrees; jigc milestone provision axis-three-probe --force
exit:   1
out:    warning: removing the leftover directory .jigc/worktrees/first-sub discards work that is
          not in git:  precious.txt   … not recoverable.
        blocking · milestone.provision-failed — … could not clear the leftover at
          `.jigc/worktrees/first-sub`: Permission denied (os error 13). 0 of 2 worktree(s) landed …
          at: .jigc/worktrees/first-sub
          route: `jigc milestone provision axis-three-probe --force` — deal with what the message
            names at that path first; the re-run reuses every worktree that landed …
after:  precious.txt IS gone -> the narration was true, and `--force` is echoed in the re-run argv

# (b) read-only LEFTOVER — nothing is removed at all
argv:   chmod 500 .jigc/worktrees/first-sub; jigc milestone provision axis-three-probe --force
exit:   1
out:    blocking · milestone.provision-failed — … (identical shape)
        *** NO loss warning printed ***
after:  cat .jigc/worktrees/first-sub/precious.txt -> precious       # narration is outcome-keyed
```

### R-D — the symlink-escape cell at two doors (rows 17, 43)

```
setup:  OUT=$(mktemp -d "${TMPDIR:-/tmp}/axis3-outside.XXXXXX"); printf 'PRECIOUS OUTSIDE\n' > "$OUT/keep.txt"
        mkdir -p .jigc/worktrees && ln -s "$OUT" .jigc/worktrees/first-sub
argv:   jigc milestone provision axis-three-probe --force
exit:   0
out:    warning: removing the leftover file .jigc/worktrees/first-sub discards work that is not in git:
              first-sub … not recoverable.
        provisioned 2 worktree(s) …
after:  cat "$OUT/keep.txt" -> PRECIOUS OUTSIDE            # the link died, the target did not

setup:  ln -s "$OUT" .jigc/worktrees/second-sub-link
argv:   jigc uninstall --force
exit:   0
out:    warning: removing the leftover file .jigc/worktrees/second-sub-link discards work … 
after:  cat "$OUT/keep.txt" -> PRECIOUS OUTSIDE
```

### R-E — `milestone discard`: precedence, both arms, both consents (rows 18–27)

```
setup:  rig with provisioned worktrees, `wip.txt` in first-sub, an adr staged in .jigc/tasks/first-sub/docs/,
        and an unrelated `.jigc/notes.txt`
argv:   jigc milestone discard axis-three-probe
exit:   1
out:    blocking · milestone.dirty-worktree — … 1 sub-task worktree path(s) hold something the
          abandon cannot prove is disposable, and `jigc milestone discard` would settle the record
          and tear the workbench down over them:
          .jigc/worktrees/first-sub: ?? wip.txt — it is a live git worktree holding uncommitted
            work, registered here or not; registered here, so the teardown removes it and this
            content is destroyed
          route: look at those paths and get out what you need … or run
            `jigc milestone discard axis-three-probe --force` to abandon the milestone anyway: …
                                                  # the worktree arm takes precedence over staged prose

argv:   rm -f .jigc/worktrees/first-sub/wip.txt; jigc milestone discard axis-three-probe
exit:   1
out:    blocking · milestone.staged-prose — … 1 sub-task(s) stage 1 doc(s) that no commit has a
          copy of … first-sub: adr:sub-decision
          route: read what is in them with `jigc doc show <address> --task <sub-task-id>` …, or land
            the milestone with `jigc milestone finalize axis-three-probe` … — or … `--force` …

argv:   jigc milestone discard axis-three-probe --force
exit:   0
out:    warning: discarding milestone:axis-three-probe discards the staged docs of 1 open task(s),
          which no commit has a copy of:  first-sub: adr:sub-decision  … not recoverable.
        discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
after:  .jigc/worktrees empty · .jigc/milestones empty · the UNRELATED task `tidy-the-readme` survives
        · .jigc/notes.txt survives · git log HEAD = "chore(milestone): discard record for milestone:…"
```

### R-F — the unregistered path `--force` leaves behind, as its refusal promised (row 24)

```
setup:  fresh rig; printf 'gitdir: /nonexistent/admin\n' > .jigc/worktrees/first-sub/.git
        printf 'work\n' > .jigc/worktrees/first-sub/wip.txt          # never provisioned => unregistered
argv:   jigc milestone discard axis-three-probe --force
exit:   0
out:    (stderr EMPTY)  discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
after:  .jigc/worktrees/first-sub/{.git,wip.txt} both present, `cat wip.txt` -> work
        # the refusal had said "not registered here, so the teardown leaves it on disk" — driven true,
        # and the door narrates no loss it did not take
```

### R-G — **DEFECT D-2**: the exit-0 ack claims a removal that did not happen (row 29)

```
setup:  fresh rig; jigc milestone provision axis-three-probe; printf 'wip\n' > .jigc/worktrees/first-sub/wip.txt
        chmod 500 .jigc/worktrees
argv:   jigc milestone discard axis-three-probe --force
exit:   0
stderr: warning: removing the fan-out worktree .jigc/worktrees/first-sub discards work that is not
          in git:  wip.txt (never staged)  … not recoverable.
        warning: could not remove the fan-out worktree /private/var/.../.jigc/worktrees/first-sub:
          `git worktree remove --force …` failed: error: failed to delete '…': Permission denied
          remedy: run `git worktree prune`, then `git worktree remove --force …`
        warning: could not remove the fan-out worktree …/second-sub: … (same)
stdout: discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
after:  ls .jigc/worktrees -> first-sub  second-sub          # both still there
```

The **loss** narration two lines above is outcome-keyed and correct (`wip.txt` really is gone).
The **ack** is not: `workbench removed` is a fixed string, not generated from the teardown's
outcome, and in this state it is false for two of the three things the teardown names
(`design/surface-contract.md` → **Law 1**: *"Every claim a surface makes is generated from the
thing it describes, or asserted against it"*; `design/team-ready-state.md` → *"each refusal
states what its own door does and never claims a removal it will not perform"* — driven, the
**ack** does exactly that). The absolute host paths in the two `remedy:` warnings are **not**
part of this finding: `milestone.rs:4573` disposes them `DeclaredAbsolute` with a stated reason
(git resolves a worktree path against the caller's cwd), and `repo_relative_paths.rs` carries
that disposition.

### R-H — `uninstall`'s three subjects, in their declared precedence (rows 30–35)

```
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"      # stages commit:tidy-the-readme
argv:   jigc uninstall
exit:   1
out:    blocking · uninstall.staged-prose — `.jigc/` holds 1 staged doc(s) for 1 open task(s) that
          no commit has a copy of … tidy-the-readme: commit:tidy-the-readme
          route: read what is in them with `jigc doc show <address> --task <task-id>`, then throw
            the task away with `jigc task discard <task-id> --force` — or, once its doc is complete,
            land it with `jigc task finalize <task-id>` (which refuses while a required slot is
            empty) — then re-run `jigc uninstall`; `jigc uninstall --force` deletes them …

setup:  jigc task discard tidy-the-readme --force; printf 'private\n' > .jigc/notes.txt
argv:   jigc uninstall
exit:   1
out:    blocking · uninstall.untracked-workbench-file — `.jigc/` holds 1 file(s) that no index has
          a copy of … .jigc/notes.txt
          route: put them where they can be recovered (`git add <path>` is enough …) …

setup:  printf '# a hand edit\n' >> .jigc/config/packs.yaml          # TRACKED, but the index has the old bytes
argv:   jigc uninstall
exit:   1
out:    … holds 2 file(s) that no index has a copy of:  .jigc/config/packs.yaml  .jigc/notes.txt

# the route, followed verbatim:
argv:   git add .jigc/notes.txt .jigc/config/packs.yaml; jigc uninstall
exit:   0
out:    warning: removing `.jigc/` also removes 6 tracked file(s) under it: … 
          note: each is in the index, so `git checkout -- <path>` brings it back.
        jigc uninstall — repo-local install removed …
```

### R-I — all three `uninstall` subjects at once, under `--force` (rows 32, 41)

```
setup:  fresh rig; jigc milestone provision axis-three-probe; printf 'wip\n' > .jigc/worktrees/first-sub/wip.txt
        jigc start --workflow quick-fix "tidy the readme"; printf 'private\n' > .jigc/notes.txt
argv:   jigc uninstall
exit:   1     ->  uninstall.dirty-worktree  (subject 1 of 3 wins)
argv:   jigc uninstall --force
exit:   0
stderr: warning: removing the fan-out worktree .jigc/worktrees/first-sub … wip.txt (never staged) … not recoverable.
        warning: removing `.jigc/` discards the staged docs of 1 open task(s) … tidy-the-readme: commit:… not recoverable.
        warning: removing `.jigc/` destroys 1 file(s) under it that no index has a copy of: .jigc/notes.txt … not recoverable.
        warning: removing `.jigc/` also removes 5 tracked file(s) under it: … `git checkout -- <path>` brings it back.
stdout: jigc uninstall — repo-local install removed   (7 bullets)
```

### R-J — `uninstall --force` when `remove_dir_all` fails part-way (rows 44–45)

```
setup:  rig B (a leftover file at first-sub, a chmod-000 dir at second-sub)
argv:   jigc uninstall --force
exit:   1
out:    warning: removing `.jigc/` also removes 5 tracked file(s) under it: … (all 5 really gone)
        blocking · uninstall.remove-jigc — cannot remove the repo-local `.jigc/` tree: Permission
          denied (os error 13)
          route: ensure `.jigc/` is writable, then re-run `jigc uninstall`
after:  git status -> ` D .jigc/.gitignore … ?? .jigc/worktrees/`   (partial teardown, loudly)
argv:   chmod 755 .jigc/worktrees/second-sub; jigc uninstall        # the route, verbatim
exit:   1  -> uninstall.dirty-worktree, which itself names `jigc uninstall --force`
```

### R-K — **DEFECT D-4**: the fail-closed root cell (row 47)

```
setup:  a rig with a provisioned milestone; chmod 000 .jigc/worktrees
argv:   jigc uninstall
exit:   1
out:    blocking · uninstall.dirty-worktree — cannot check `.jigc/worktrees/` for uncommitted
          fan-out work, so removing `.jigc/` could destroy it: Permission denied (os error 13)
          route: make sure `git` is on PATH and the repository is readable, then re-run
            `jigc uninstall` — or, once you have confirmed the fan-out worktrees hold nothing you
            need, remove them yourself (`git worktree list`, then `git worktree remove`) and re-run
```

Two halves, both driven: (a) the failure is a `read_dir` EACCES on `.jigc/worktrees/` — the
producer's own doc-comment says so (*"the one failure with no path to name — `read_dir` on
`.jigc/worktrees/` itself"*) — and the route's first act is *make sure `git` is on PATH*, which
fits nothing that happened; (b) the route never names **`--force`**, while the two sibling
fail-closed refusals at this same door (`unverified_prose_finding`, `unverified_workbench_finding`
— rows 46 and 34's fail-closed twin) both do, and `--force` does clear this state because it
skips the guards. That pair — *a route about `git` being on PATH, with the `--force` consent
unnamed* — is verbatim the defect `design/project-setup.md`:160 records M50 as having fixed at
this door; it survives at the root-enumeration cell.

### R-L — **DEFECT D-3**: the fail-closed prose probe prints the host filesystem (rows 46, 60)

```
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        chmod 000 .jigc/tasks/tidy-the-readme/docs
argv:   jigc task discard tidy-the-readme
exit:   1
out:    blocking · task-discard.staged-prose — cannot check task `tidy-the-readme`'s working area
          for staged docs, so discarding it could destroy work no commit has a copy of:
          /private/var/folders/nj/…/repo/.jigc/tasks/tidy-the-readme/docs: Permission denied (os error 13)
          route: make sure `.jigc/tasks/tidy-the-readme/` is readable, then re-run …
argv:   jigc uninstall
exit:   1
out:    blocking · uninstall.staged-prose — cannot check `.jigc/tasks/` for open tasks' staged
          docs … /private/var/folders/nj/…/repo/.jigc/tasks/tidy-the-readme/docs: Permission denied …
argv:   jigc milestone discard axis-three-probe          # the unreadable task is not ITS sub-task
exit:   0                                                 # correctly scoped — matches contract
```

The printed path is the **host** path at two destroying doors, against law 1's *every printed
path is repo-real or a typed identity*. The producer is `crates/cli/src/task.rs:697`. That file
sits in `UNSWEPT_PRODUCERS` (`crates/cli/tests/repo_relative_paths.rs:746`) with **count 3** —
which is right, `grep -n '\.display()' crates/cli/src/task.rs` returns exactly lines 697, 1861,
3498 — but the row's **reason** reads *"two `with_context` promote/probe faults and one
`git archive --prefix=`, which is an argument handed to a subprocess"*. Line 697 is neither: it
composes the message of a **blocking finding** that two destroying doors print. The bound is
therefore counted but mis-described, and the description is exactly the thing that decides
whether a site is an error-channel remainder (licensed) or a finding surface (not).

### R-M — **DEFECT D-1**: the staged-prose route names a verb the state refuses (rows 53, 55, 33)

```
setup:  fresh rig; jigc milestone provision axis-three-probe
        (cd .jigc/worktrees/second-sub && jigc doc create adr --title "second decision" --task second-sub)
argv:   jigc task discard second-sub
exit:   1
out:    blocking · task-discard.staged-prose — task `second-sub` stages 1 doc(s) that no commit has
          a copy of — discarding it would destroy them: adr:second-decision
          route: read what is in them with `jigc doc show <address> --task second-sub`, or land them
            with `jigc task finalize second-sub` (which refuses while a required slot is empty) —
            or, once you have confirmed the task holds nothing you need, `jigc task discard
            second-sub --force` removes the working area with them

# the route's landing exit, followed verbatim:
argv:   jigc task finalize second-sub
exit:   3
out:    blocking · finalize.milestone-sub-task — task `second-sub` is a sub-task of milestone
          `axis-three-probe` — the parent milestone's finalize is the only commit boundary; a
          per-sub-task finalize would land a commit outside it and strand this sub-task's work
          at: task:second-sub
          route: `jigc milestone finalize axis-three-probe` …

# the sibling at the install door, over the same state:
argv:   jigc uninstall
out:    blocking · uninstall.staged-prose — … second-sub: adr:second-decision
          route: … land it with `jigc task finalize <task-id>` (which refuses while a required
            slot is empty) …
```

The route's parenthetical states the **only** condition under which the verb it names refuses,
and in this state the verb refuses for a different reason entirely, at a different door.
`design/surface-contract.md` → **Law 2**: *"Routes and `Run:` lines parse against the real CLI"*
and the shipped sibling rule M46 Increment 5 states — *the diagnosis stops naming a verb the
state refuses*. It is not a dead end (the follow-on refusal carries the right route,
`jigc milestone finalize <id>`), but it is a two-hop misdirection at a **destroying** door, and
the third member of the same family gets it right: `milestone.staged-prose` (row 20) routes at
`jigc milestone finalize <id>` directly. Both offending producers are reachable from one state.

### R-N — `milestone finalize`: the narrate-only member (rows 49–52)

```
setup:  fresh rig; jigc milestone provision axis-three-probe
        (cd .jigc/worktrees/first-sub  && printf 'one\n' > one.txt && git add one.txt
                                       && printf 'PRECIOUS\n' > scratch.txt)          # never staged
        (cd .jigc/worktrees/second-sub && printf 'two\n' > two.txt && git add two.txt)
        mkdir -p .jigc/worktrees/stray-leftover && printf 'not mine\n' > .jigc/worktrees/stray-leftover/keep.txt
        printf 'loose\n' > .jigc/worktrees/loose-file
argv:   jigc milestone finalize axis-three-probe --format json
exit:   0
stderr: warning: removing the fan-out worktree .jigc/worktrees/first-sub discards work that is not
          in git:  scratch.txt (never staged)  … not recoverable.
stdout: { "committed": { … "sub_tasks": [ { "id": "first-sub", "code_files": 1,
            "discarded": [ { "path": "scratch.txt", "state": "never-staged" } ], … } … ] } }
after:  ls .jigc/worktrees -> loose-file  stray-leftover        # both unregistered leftovers survive,
                                                               # and neither is named anywhere
# and the text arm of the same cell, in a sibling rig:
stdout: finalized e12e056 — Finalize milestone axis-three-probe (2 sub-tasks) …
          discarded with the fan-out worktrees (not committed, not recoverable):
            first-sub: scratch.txt (never staged)
# with nothing to land:
argv:   jigc milestone finalize axis-three-probe        (fresh provision, no work)
exit:   3   blocking · milestone.zero-contribution — … route names provision + discard
```

### R-O — `task discard`'s two arms and its ack (rows 53–59)

```
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
argv:   jigc task discard tidy-the-readme                       -> exit 1, task-discard.staged-prose
argv:   jigc task discard tidy-the-readme --format json         -> exit 1, {"error": …}
argv:   jigc task discard tidy-the-readme --force --format json -> exit 0
out:    { "commit": null, "dropped": ["commit:tidy-the-readme"], "findings": [], "op": "task-discard",
          "task": "tidy-the-readme" }          (stdout; stderr empty)
argv:   jigc task discard first-sub            # a sub-task whose area stages nothing
exit:   0   discarded task first-sub / record commit: 6b627c6  — settled to `discarded`
argv:   jigc task discard second-sub --force   # inert force
exit:   0   (identical ack)
argv:   jigc task discard first-sub --force    # a sub-task WITH a staged adr
exit:   0   discarded task first-sub — dropped staged edits to: adr:sub-decision
            record commit: 95cb1d7 …
```

### R-P — stream discipline and the reject envelope at the three refusing doors (rows 2, 19, 48)

```
setup:  rig B (file + unreadable dir under .jigc/worktrees/)
argv:   jigc milestone provision axis-three-probe   -> stdout 0 bytes, stderr 720, exit 1
argv:   jigc milestone discard  axis-three-probe    -> stdout 0 bytes, stderr 1290, exit 1
argv:   jigc uninstall                               -> stdout 0 bytes, stderr 1186, exit 1
argv:   jigc uninstall --format json  2>&1 >/dev/null | python3 -c "…print(list(d.keys()))"
out:    ['severity','probe','check','code','key','message','location','route']      # a BARE Finding
argv:   jigc milestone provision … --format json     -> {"error": …}                # the flattened arm
```

---

## 4 · `(door, cell)` pairs I did **not** drive, and why

Stated plainly rather than presented as covered:

1. **`milestone finalize` × {S1, S2, S3, S5, S6} × `--force`** — *the force axis does not exist
   at this leaf.* Driven: `jigc milestone finalize --help` lists `--carry-staged` and
   `--format` only. The shape axis itself is unreachable at this door by construction (its
   teardown's subject is the **registered** set), and the *survival* half of that claim was
   driven at row 51 for two shapes (`Directory` and `File`).
2. **`milestone discard` / `uninstall` × S5 `File` at a *registered* worktree path** —
   unreachable: `git worktree add` cannot register a non-directory, so no state exists in which
   a `File` sits at a registered path. The reachable branch (unregistered → the teardown skips
   it) was driven at rows 24 and 51.
3. **`milestone provision` × {S7 staged prose, S8 untracked workbench} × `--force`** — the
   no-`--force` rows (12, 13) drove that neither is this door's subject; `--force` only widens
   what the door may remove at a *worktree path*, so the forced cell cannot differ. Not driven.
4. **`task discard` × {S1–S6, S8}** — no such cell exists at that door: its subject is
   `.jigc/tasks/<id>/docs/`, it reads no worktree path and no workbench file. Driven evidence
   that the scoping is real is row 28 (`milestone discard` over an unrelated task's unreadable
   `docs/` → exit 0) and row 60's route, which names the task door and not `uninstall`.
5. **Any cell × `LeftoverVerdict::OwnWorktree` × `LeftoverShape::Unreadable`** — the two cannot
   co-exist: `git rev-parse --show-toplevel` cannot run inside a `chmod 000` directory, so the
   verdict is `Unverifiable` by construction. The combination is empty, not skipped.
6. **The record-commit rejection cells** (a `pre-commit` hook rejecting `milestone discard`'s or
   `task discard`'s record-only commit; the `chatty-hooks` rig) — that is **axis 4**
   (transaction/rollback), and driving it here would report another axis's rows as mine.
7. **`uninstall` × S8 where the workbench probe itself fails** (`unverified_workbench_finding`)
   — not driven; the sibling fail-closed cells at the same door were (rows 46, 47), and this one
   needs an unreadable non-transient `.jigc/` subtree that the earlier guards do not reach
   first. Stated as un-driven rather than inferred.

---

## 5 · Findings

### D-1 · MEDIUM — the staged-prose route at two destroying doors names a landing verb the state refuses

`task-discard.staged-prose` (`crates/cli/src/task.rs`, the `discard_staged_prose_finding`
route) and its sibling `uninstall.staged-prose` (`crates/cli/src/setup.rs`,
`staged_prose_finding`) both offer *"land them with `jigc task finalize <id>` (which refuses
while a required slot is empty)"*. Over a **milestone sub-task** — the exact state both doors
list, since `milestone add-task` + a first `doc create --task <sub>` reaches it — that verb
refuses with `finalize.milestone-sub-task` at **exit 3** for a reason the parenthetical excludes.
Repro: §3 R-M. Contract: `design/surface-contract.md` → law 2 (*routes parse against the real
CLI*) and M46 Increment 5's shipped rule (*the diagnosis stops naming a verb the state refuses*).
The third member of the family, `milestone.staged-prose`, is correct (row 20), which is what
makes this an un-swept axis rather than a design choice.

### D-2 · LOW — `jigc milestone discard --force` acks `workbench removed` over a teardown that failed

Rows 29 / §3 R-G. At exit 0 the ack asserts the removal unconditionally while two worktrees are
still on disk and the same run printed two `could not remove the fan-out worktree …` warnings.
The loss narration beside it *is* outcome-keyed (M50's fix, driven correct at rows 16 and 25);
the ack is a fixed string. Law 1: a claim must be generated from the thing it describes.

### D-3 · LOW — the fail-closed staged-prose refusal prints the host path, and the bound that counts it describes a different site

Rows 46 / 60, §3 R-L. `crates/cli/src/task.rs:697` composes an **absolute** path into a blocking
finding printed by `jigc task discard` and `jigc uninstall`. The file is in `UNSWEPT_PRODUCERS`
with the right count (3) and a **false reason**: it describes all three sites as `with_context`
faults and a `git archive --prefix=`, i.e. *error channels*, when one of them is a finding
surface at two destroying doors. The remainder is counted; what licenses it is not true.

### D-4 · LOW — the worktrees-root fail-closed route blames `git` on PATH and omits the consent

Row 47, §3 R-K. `unverified_worktrees_finding`'s route opens with *make sure `git` is on PATH*
for a failure its own doc-comment identifies as `read_dir` on `.jigc/worktrees/`, and never
names `--force`, while both sibling fail-closed refusals at the same door do. That pairing is
verbatim the defect `design/project-setup.md`:160 records as fixed at this door by M50; it
survives at the one cell M50's repair did not cover.

### Observations (driven, but not defects — recorded so reconciliation is not re-driving them)

- **O-1 · the third reject envelope.** `jigc uninstall --format json` over a refusal emits a
  **bare `Finding`** (8 top-level keys), which is neither of `ENVELOPE_ARMS`' two declared
  reject rows (`Reject::Error` = `{error}`, `Reject::Findings` = `{findings, schema_version}`).
  It is **declared** — `design/command-output-contract.md`:304 and :412 name `setup_block` as
  the *third funnel* and print this exact shape — so the binary is not lying; what is in tension
  is the contract's own *"the two reject arms"* framing and `ENVELOPE_ARMS`' proof (2) that
  *every production arm has exactly one row*. Axis 5's call, not a defect at axis 3.
- **O-2 · what a clean `uninstall` takes in silence.** With a milestone and two sub-task areas
  present but no staged `*.md`, `jigc uninstall` exits 0, removes `.jigc/` and narrates only the
  tracked files: the sub-task areas' `intent`/`workflow`/`base.json` and the milestone workbench
  go unnamed. That is inside the declared three-subject scope (`setup.rs::workbench_paths`
  excludes the `ENTRIES` prefixes; `pending_teardown`'s doc-comment states the
  *visible-not-prevented* bound over the rest), so it is recorded, not filed.
- **O-3 · stale worktree admin records.** After `uninstall`, `git worktree list` still shows the
  removed fan-out paths as `prunable`. Driven consequence: a later `setup` + `milestone create` +
  `provision` at the **same** sub-task path still succeeds (exit 0), so nothing is bricked.
- **O-4 · `uninstall` → `setup` does not round-trip without consent.** After a clean teardown,
  `jigc setup` exits 1 with `setup.dirty-install-path` over `.claude/settings.json` — uninstall's
  own uncommitted edit. That is M51 F2's **declared** cost (*"forgetting a new install path now
  costs a loud false alarm `--force` clears, never a loss"*), driven at the seam between axis 3's
  door and axis 4's.
- **O-5 · the adapter deny floor covers 2 of the 5 doors.** The installed `.claude/settings.json`
  `deny` array carries `Bash(jigc uninstall:*)` and `Bash(jigc milestone discard:*)` and no other
  jigc verb — as M50 Increment 3 declared. `milestone provision`, `milestone finalize` and
  `task discard` are destroying doors an agent may run unasked; that is the shipped decision, and
  it is recorded here because this axis is the one that would notice.

---

## 6 · What this review adds over the flow-52 arm set

**Flow 52 mints no destroying-door arm, and the acceptance design says so by omission**: its
nine arms are the path-argument registry, the posture family, `setup`'s install pathspec, the
config-layer CAS pre-image, the `EnvelopeArm` registry, the unknown-id doors, `ManifestKind`,
the ambush owe-set and the orphan/`STORE_EXIT_FLIPS` arm. The nearest neighbour is **arm 3**
(`setup`'s own install pathspec, D3): it drives the *same guard shape* this axis is about —
refuse **before** the first write, `--force` as the single consent, the user's bytes still on
disk afterwards — at a door that is **not** in `DESTROYING_DOORS`. Arm 3 therefore proves the
shape at a new door and proves nothing about the four old ones. The axis proper is pinned by
**flow 48 arm 1** (`DESTROYING_DOORS × LEFTOVER_VERDICTS`), **flow 49 arm 2** (*every destroying
door answers for what it removes*, iterating the refuse-vs-narrate discriminator), and the
standing suites `leftover_probe_fail_closed.rs`, `uninstall_worktree_guard.rs`,
`uninstall_workbench_subject.rs` and `repo_relative_paths.rs`.

**What this table adds over all of that** is the part a suite that iterates one registry cannot
reach:

1. **The cross-product, not the two axes separately.** The suites iterate `door × verdict` and
   `door × shape`; this table drives `door × {shape, verdict, other-subject} × force` — and the
   four cells that only exist in the cross-product are where three of the four findings live:
   the *removal-outcome* pair (rows 15/16/29 — narration vs. ack over a failed removal), the
   *fail-closed probe at the root vs. at a member* (rows 46/47), and the *route followed to its
   next state* (rows 36, 45, 55 — a route is not proven by existing, only by running).
2. **The routes were executed, not read.** Rows 36, 45 and the D-1 drive follow the printed
   route verbatim and record where it lands. That is the only way D-1 is visible: the route
   parses against the real CLI (so law 2's mechanical fence is green) and still names a verb the
   state refuses.
3. **The subjects that are *not* a door's** (rows 12, 13, 28) are driven rather than asserted,
   so `n/a` in this table means *"driven and the door proceeded"*, not *"I read the code and it
   looked scoped"*.
4. **The data-safety cells a registry has no row for**: the symlink escape at two doors (rows
   17, 43 — the link dies, the tree it points at does not), the partial-teardown states (rows
   15, 29, 44), and the machine surface of the loss (rows 50, 56 — `discarded[]` and `dropped[]`
   carry the loss as data, so a driver sees what a human reads on stderr).

---

# RECONCILIATION — the Opus driver's table × the Codex source pass

**Reconciled by** a third agent on the **same** binary the driver used —
`/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15` (asserted before anything else), release
posture, the M51 post-audit build. Rigs built exactly as the driver built them
(`dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc`, two-step eval, then
`milestone create` + two `milestone add-task` through the binary — the driver's own
`scratchpad/axis-review/driver/mkrig.sh`). Twenty-one fresh rigs; `mktemp -d` roots, no teardown.

**The rule applied** (`acceptance-design.md` → The reconciliation rule): every Codex claim
entered as `lead(codex, …)` and then **driven** — to a repro block (CONFIRMED, origin codex) or
to a recorded refutation with the falsifying datum. Every driver DEFECT re-driven once here. A
claim reachable by no rig stays an **OPEN LEAD** with its reason; nothing was promoted on a
source read.

---

## 1 · Table demotions — rows marked driven that carry no repro block

The driver's §2 says all 60 rows carry a §3 repro block. Mapping each row to a block, **14 do
not**. Each is marked in the table above and was **re-driven here**, so none is lost — but the
driver's claim for them was not supported by its own evidence section.

| rows | why demoted | re-driven as |
|---|---|---|
| 9, 19, 22, 23, 26, 27, 31, 38, 39, 40, 42 | no observed output for the cell appears anywhere in §3 | RR-1 … RR-6 |
| 25 | §3 R-G carries the asserted surface text verbatim — but in the **failed-teardown** rig (D-2's), not in this row's clean-teardown state | RR-7 |
| 30 | §3 R-P records this state's exit + stream byte counts; the asserted surface text (`registered as a worktree nowhere in this repository`, both paths) appears nowhere | RR-5 |
| 32 | §3 R-I records the exit + code (`subject 1 of 3 wins`); the asserted route text (`milestone discard --force` **and** `uninstall --force`) appears nowhere | RR-7 |

**All 14 re-drove to the driver's stated verdict** — no row's verdict changed. The demotion is
about the evidence, not the claim. Re-drive blocks are §4 below.

---

## 2 · Reconciliation ledger — every Codex claim

### C-1 — CONFIRMED (origin codex), and **wider than claimed**

> *"`jigc task discard` guards only staged `*.md` files, so an arbitrary untracked file elsewhere
> in the task area is silently destroyed without `--force`."*

**Driven, exit 0, no refusal, no loss narration, bytes gone** — and the same drive over the
other doors shows the class is **five** doors, not the three Codex named: driving found
`jigc uninstall` and `jigc task finalize` as well.

```
### RC-1 — the task-area non-`.md` bypass at `jigc task discard`

setup:  rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
        jigc milestone create "axis three probe"
        jigc milestone add-task axis-three-probe "first sub"
        jigc start --workflow quick-fix "tidy the readme"
        printf 'PRECIOUS NOTES\n'  > .jigc/tasks/tidy-the-readme/notes.txt
        printf 'ATTACHED BYTES\n'  > .jigc/tasks/tidy-the-readme/docs/attachment.txt

argv:   jigc task discard tidy-the-readme            # the staged `commit:` md is still there
exit:   1
out:    blocking · task-discard.staged-prose — task `tidy-the-readme` stages 1 doc(s) …
                                                     # the guard fires — on the `.md` only

argv:   rm -f .jigc/tasks/tidy-the-readme/docs/*.md   # leave ONLY the two non-`.md` plants
        jigc task discard tidy-the-readme             # no --force
exit:   0
out:    discarded task tidy-the-readme
        (stderr empty — no refusal, no warning, no path named)
after:  .jigc/tasks/tidy-the-readme            -> No such file or directory
        notes.txt GONE · docs/attachment.txt GONE
```

```
### RC-2 — the same bypass at `jigc milestone discard` (Codex C-2)

setup:  fresh rig + milestone with two sub-tasks
        printf 'PRECIOUS SUBTASK NOTES\n' > .jigc/tasks/first-sub/notes.txt
        mkdir -p .jigc/tasks/second-sub/docs
        printf 'ATTACHED\n'              > .jigc/tasks/second-sub/docs/attachment.txt

argv:   jigc milestone discard axis-three-probe        # no --force
exit:   0
out:    discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
        (stderr empty)
after:  notes.txt GONE · second-sub/docs/attachment.txt GONE
```

```
### RC-3 — the same bypass at `jigc milestone finalize` (Codex C-3)

setup:  fresh rig; jigc milestone provision axis-three-probe
        (cd .jigc/worktrees/first-sub  && printf 'one\n' > one.txt && git add one.txt)
        (cd .jigc/worktrees/second-sub && printf 'two\n' > two.txt && git add two.txt)
        printf 'PRECIOUS SUBTASK NOTES\n' > .jigc/tasks/first-sub/notes.txt
        mkdir -p .jigc/tasks/second-sub/docs && printf 'ATTACHED\n' > .jigc/tasks/second-sub/docs/attachment.txt

argv:   jigc milestone finalize axis-three-probe
exit:   0
out:    finalized 1303c9f — Finalize milestone axis-three-probe (2 sub-tasks)
          modified docs/milestone-records/axis-three-probe.md · added one.txt · added two.txt
        (no warning names either plant)
after:  notes.txt GONE · second-sub/docs/attachment.txt GONE
```

```
### RC-4 — the two doors Codex did NOT name, found by driving

# (a) `jigc uninstall` — the third `DESTROYING_DOORS` refusing member
setup:  fresh rig; printf 'PRECIOUS SUBTASK NOTES\n' > .jigc/tasks/first-sub/notes.txt
argv:   jigc uninstall                                  # no --force
exit:   0
out:    warning: removing `.jigc/` also removes 5 tracked file(s) under it: …
        jigc uninstall — repo-local install removed  (7 bullets)
                # `uninstall.untracked-workbench-file` does NOT fire — `tasks/` is an
                # `ENTRIES` prefix, excluded from `workbench_paths`' subject by construction
after:  notes.txt GONE, named by nothing

# (b) `jigc task finalize` — NOT a `DESTROYING_DOORS` member at all
setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        (append a line to README.md; git add -A)
        jigc doc set-field 'commit:tidy-the-readme#header/type' --task tidy-the-readme --value fix
        printf 'tidy the readme\n' | jigc doc set-slot 'commit:tidy-the-readme#summary' \
              --task tidy-the-readme --from-file -
        printf 'PRECIOUS NOTES\n' > .jigc/tasks/tidy-the-readme/notes.txt
argv:   jigc task finalize tidy-the-readme
exit:   0
out:    finalized bc34fc2 — fix: tidy the readme / modified README.md / 1 file committed
after:  .jigc/tasks/tidy-the-readme gone; notes.txt GONE and in no commit — the landed
        boundary carried README.md and the commit message, never this file
```

**The contract it is judged against, driven not read.** `setup.rs::workbench_paths`' doc-comment
states the exclusion and names its warrant: *"`tasks/` and `worktrees/` are inside `ENTRIES` …
excluded by construction and **answered by the doors that own them**."* Driven, the doors that
own them answer for `docs/*.md` and nothing else (`task.rs::staged_doc_ids` — *"the `*.md` set,
never directory-non-emptiness"*), so a non-`.md` byte under `.jigc/tasks/<id>/` is answered by
**no door at all**. That is the shape M49 closed one layer out at `fanout_worktree_paths`, in
that function's own words: *"`path.is_dir()` … is a claim about **shape** where the door's
question is about **bytes**"* — here the shape-claim is the `.md` suffix. And
`design/team-ready-state.md`'s derived rule is stated over **paths under `.jigc/worktrees/`**,
so it does not reach this subject; the *visible, not prevented* bound is declared for the
worktree's unstaged/untracked/ignored bytes and for `displaced/`, and for no other path.

**Scoping the claim honestly.** jigc's own bookkeeping in a task area is a known, derivable set
(`intent`, `workflow`, `base.json`, `staged-snapshot.json`, `docs/provenance.json`, `docs/*.md`)
— the driver's **O-2** records those going unnamed and disposes it inside the declared scope.
The finding here is the **complement**: a file that is none of those, which only a human or an
agent can have placed, destroyed at exit 0 by five doors with no refusal and no narration. Two
of those five are doors whose whole premise (`discard`, `uninstall`) is a refusal over
sole-copy bytes.

**Severity, stated rather than assumed:** the loss is real and silent, but the state needs
someone to write into `.jigc/tasks/<id>/` — which the adapter tells agents not to do. That is
the same reachability argument M49 heard and rejected at `fanout_worktree_paths`, where a plain
file under `.jigc/worktrees/` was equally off the sanctioned path.

### C-2 — CONFIRMED (origin codex) · see **RC-2**

Driven exactly as proposed. One correction to the claim's wording: the ack does **not** stay
silent about the workbench — it says `workbench removed`, which is *true here* (the teardown
succeeded); what it never does is name the file it took. The lost-file half is confirmed as
stated.

### C-3 — CONFIRMED (origin codex) · see **RC-3**

Driven exactly as proposed. Note it lands on the **committed** side of the boundary, so the
`*.md` really are in git — which is precisely why the non-`.md` file is the one byte the
landed-finalize warrant (*"the commit lands first, so those bytes are already in git"*) does not
cover.

### Codex's "consistent read" — checked, and one sentence corrected

Codex reports `DESTROYING_DOORS` as four rows, `probe_leftover` as `symlink_metadata`-based and
fail-closed, provision guarding before removal, and uninstall's three subjects — all of which
the driver's rows 1–48 independently drove true. **Its one factual slip is harmless**: it notes
*"there is no current `src/uninstall.rs`"* against the prompt, which is right — `uninstall` lives
in `crates/cli/src/setup.rs`, exactly where the driver's D-1 and D-4 place it.

**No Codex claim was refuted, and none is an open lead** — all three were reachable with the
`fresh` rig and all three drove to loss.

---

## 3 · Reconciliation ledger — every driver DEFECT

The source pass is **silent on all four** (it hunted removal sites and bypasses, not surface
truth), so none is contradicted. Each was re-driven once here; all four reproduce.

| defect | status | re-drive |
|---|---|---|
| **D-1** · the staged-prose route names `jigc task finalize <id>`, which a milestone sub-task refuses with `finalize.milestone-sub-task` at exit 3 | **CONFIRMED** — reproduced verbatim at both doors | RD-1 |
| **D-2** · `milestone discard --force` acks `workbench removed` over a teardown that failed | **CONFIRMED** — exit 0, both worktrees still on disk, two `could not remove` warnings in the same run | RD-2 |
| **D-3** · the fail-closed staged-prose refusal prints an absolute host path, and the `UNSWEPT_PRODUCERS` reason describes a different site | **CONFIRMED** — both halves, including the reason's falsity | RD-3 |
| **D-4** · the worktrees-root fail-closed route blames `git` on PATH and omits `--force` | **CONFIRMED**, with **one supporting sentence corrected** | RD-4 |

```
### RD-1 — D-1, re-driven

setup:  fresh rig; jigc milestone provision axis-three-probe
        (cd .jigc/worktrees/second-sub && jigc doc create adr --title "second decision" --task second-sub)

argv:   jigc task discard second-sub
exit:   1
out:    blocking · task-discard.staged-prose — task `second-sub` stages 1 doc(s) that no commit
          has a copy of — discarding it would destroy them: adr:second-decision
          route: read what is in them with `jigc doc show <address> --task second-sub`, or land
            them with `jigc task finalize second-sub` (which refuses while a required slot is
            empty) — or … `jigc task discard second-sub --force` …

argv:   jigc task finalize second-sub                    # the route, verbatim
exit:   3
out:    blocking · finalize.milestone-sub-task — task `second-sub` is a sub-task of milestone
          `axis-three-probe` — the parent milestone's finalize is the only commit boundary …
          route: `jigc milestone finalize axis-three-probe` …

argv:   jigc uninstall                                   # the sibling producer, same state
exit:   1
out:    blocking · uninstall.staged-prose — … second-sub: adr:second-decision
          route: … land it with `jigc task finalize <task-id>` (which refuses while a required
            slot is empty) …                              # the identical misdirection
```

```
### RD-2 — D-2, re-driven

setup:  fresh rig; jigc milestone provision axis-three-probe
        printf 'wip\n' > .jigc/worktrees/first-sub/wip.txt ; chmod 500 .jigc/worktrees
argv:   jigc milestone discard axis-three-probe --force
exit:   0
stderr: warning: removing the fan-out worktree .jigc/worktrees/first-sub discards work that is
          not in git:  wip.txt (never staged)  … not recoverable.
        warning: could not remove the fan-out worktree …/first-sub: `git worktree remove --force
          …` failed: … Permission denied     remedy: run `git worktree prune`, then …
        warning: could not remove the fan-out worktree …/second-sub: … (same)
stdout: discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
after:  chmod 700 .jigc/worktrees; ls .jigc/worktrees -> first-sub  second-sub   # BOTH still there
        ls .jigc/worktrees/first-sub -> (empty)   # so the LOSS narration is true: wip.txt is gone
```

The split the driver states is exactly what re-drives: the **loss** warning is outcome-keyed and
true; the **ack** is a fixed string and false for two of the three things the teardown names.

```
### RD-3 — D-3, re-driven

setup:  fresh rig; jigc start --workflow quick-fix "tidy the readme"
        chmod 000 .jigc/tasks/tidy-the-readme/docs

argv:   jigc task discard tidy-the-readme
exit:   1
out:    blocking · task-discard.staged-prose — cannot check task `tidy-the-readme`'s working area
          for staged docs …: /private/var/folders/nj/…/repo/.jigc/tasks/tidy-the-readme/docs:
          Permission denied (os error 13)

argv:   jigc uninstall
exit:   1
out:    blocking · uninstall.staged-prose — cannot check `.jigc/tasks/` … :
          /private/var/folders/nj/…/repo/.jigc/tasks/tidy-the-readme/docs: Permission denied …

argv:   jigc milestone discard axis-three-probe        # the unreadable task is not ITS sub-task
exit:   0                                              # correctly scoped

# the bound that counts the site, read verbatim:
crates/cli/src/task.rs        -> `.display()` at lines 697, 1861, 3498            (count 3, correct)
crates/cli/tests/repo_relative_paths.rs:784 reason:
  "two `with_context` promote/probe faults and one `git archive --prefix=`, which is an argument
   handed to a subprocess (the `dirty_worktrees` precedent)"
task.rs:697 is `path.join("docs").display()` inside `staged_doc_ids`' error map — it composes the
  MESSAGE of a blocking Finding that two destroying doors print. Neither a `with_context` fault
  nor a subprocess argument.
```

```
### RD-4 — D-4, re-driven

setup:  fresh rig; jigc milestone provision axis-three-probe; chmod 000 .jigc/worktrees
argv:   jigc uninstall
exit:   1
out:    blocking · uninstall.dirty-worktree — cannot check `.jigc/worktrees/` for uncommitted
          fan-out work, so removing `.jigc/` could destroy it: Permission denied (os error 13)
          route: make sure `git` is on PATH and the repository is readable, then re-run
            `jigc uninstall` — or, once you have confirmed the fan-out worktrees hold nothing you
            need, remove them yourself (`git worktree list`, then `git worktree remove`) and re-run

# the route's literal text, read from the producer:
crates/cli/src/setup.rs:3569 `unverified_worktrees_finding` — the route string above, verbatim.
The failure it fires on is `read_dir` on `.jigc/worktrees/` (setup.rs:2954, the `map_err` on
`fanout_worktree_paths`), which `git` being on PATH has nothing to do with; and `--force` is
unnamed while both sibling fail-closed refusals at this door name it (RD-3's two, driven).
```

**The one correction.** D-4's supporting sentence says *"`--force` does clear this state because
it skips the guards."* Driven, `--force` **skips the guard** but does **not** clear the state:

```
argv:   jigc uninstall --force            # same chmod-000 `.jigc/worktrees/`
exit:   1
out:    blocking · uninstall.remove-jigc — cannot remove the repo-local `.jigc/` tree:
          Permission denied (os error 13)
          route: ensure `.jigc/` is writable, then re-run `jigc uninstall`
after:  ls .jigc -> milestones  worktrees          # partial teardown; the rest is gone
```

The finding stands unchanged — the route names neither the actual fault nor the consent — but
the **consent it omits would not have finished the job in this cell either**, and the sentence
claiming it would is the driver's, not the binary's.

---

## 4 · Re-drive blocks for the demoted rows

```
### RR-1 — row 9 · `milestone provision`, S9 clean, `--force` (inert)
setup:  fresh rig, no plant
argv:   jigc milestone provision axis-three-probe --force
exit:   0
out:    provisioned 2 worktree(s) for milestone:axis-three-probe at base 69326ff (first-sub, second-sub)
        (no warning — `--force` is inert over a clean path)            -> driver verdict holds
```

```
### RR-2 — rows 23, 19 · `milestone discard`, S2 unverifiable, text then `--format json`
setup:  fresh rig; mkdir -p .jigc/worktrees/first-sub
        printf 'gitdir: /nonexistent/admin\n' > .jigc/worktrees/first-sub/.git
        printf 'work\n'                       > .jigc/worktrees/first-sub/wip.txt
argv:   jigc milestone discard axis-three-probe
exit:   1
out:    blocking · milestone.dirty-worktree — … 1 sub-task worktree path(s) …
          .jigc/worktrees/first-sub: .git, wip.txt — git cannot read a repository there, so
            nothing can say those bytes are disposable; not registered here, so the teardown
            leaves it on disk with no milestone naming it
          at: .jigc/worktrees/first-sub
          route: … or run `jigc milestone discard axis-three-probe --force` …
argv:   jigc milestone discard axis-three-probe --format json
exit:   1     stdout 0 bytes; stderr: { "error": "blocking · milestone.dirty-worktree — …" }
                                                                       -> both driver verdicts hold
```

```
### RR-3 — row 22 · `milestone discard`, S5 file + S6 unreadable, no `--force`
setup:  fresh rig; printf 'leaf\n' > .jigc/worktrees/first-sub
        mkdir -p .jigc/worktrees/second-sub/inner && chmod 000 .jigc/worktrees/second-sub
argv:   jigc milestone discard axis-three-probe
exit:   1
out:    blocking · milestone.dirty-worktree — … 2 sub-task worktree path(s) …
          .jigc/worktrees/first-sub: the file itself — it is a file, not a worktree … not
            registered here, so the teardown leaves it on disk with no milestone naming it
          .jigc/worktrees/second-sub: unknown — could not read the leftover directory …:
            Permission denied (os error 13) … not registered here, so the teardown leaves it …
          at: .jigc/worktrees/first-sub                                -> driver verdict holds
```

```
### RR-4 — rows 26, 27 · `milestone discard` over clean live worktrees / a clean workbench
setup:  fresh rig; jigc milestone provision axis-three-probe     (worktrees: first-sub second-sub)
argv:   jigc milestone discard axis-three-probe
exit:   0   stderr 0 bytes
out:    discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
after:  ls .jigc/worktrees -> (empty)
setup:  a second fresh rig, nothing provisioned
argv:   jigc milestone discard axis-three-probe
exit:   0   discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
                                                                       -> both driver verdicts hold
```

```
### RR-5 — rows 30, 31 · `uninstall`, S5+S6 then S2
setup:  fresh rig; printf 'leaf\n' > .jigc/worktrees/first-sub
        mkdir -p .jigc/worktrees/second-sub/inner && chmod 000 .jigc/worktrees/second-sub
argv:   jigc uninstall
exit:   1
out:    blocking · uninstall.dirty-worktree — `.jigc/` holds 2 fan-out sub-task worktree path(s) …
          .jigc/worktrees/first-sub:  the file itself … ; registered as a worktree nowhere in this repository
          .jigc/worktrees/second-sub: unknown — could not read … ; registered as a worktree nowhere in this repository
          route: … `jigc uninstall --force` deletes them with the install
setup:  (same rig) replace both with a dangling `.git` + wip.txt at first-sub
argv:   jigc uninstall
exit:   1   … .jigc/worktrees/first-sub: .git, wip.txt — git cannot read a repository there …
                                                                       -> both driver verdicts hold
```

```
### RR-6 — rows 42, 39, 40, 38 · `uninstall` clean, idempotency, and the success envelope
setup:  fresh rig; jigc milestone provision axis-three-probe     (two CLEAN live worktrees)
argv:   jigc uninstall
exit:   0   warning: removing `.jigc/` also removes 5 tracked file(s) … + the 7-bullet teardown
after:  .jigc present? no
argv:   jigc uninstall            -> exit 0, "(nothing to remove — no repo-local jigc install was present)"
argv:   jigc uninstall --force    -> exit 0, identical no-op
setup:  a second fresh rig
argv:   jigc uninstall --format json
exit:   0
stdout: { "allowlist_file": ".claude/settings.json", "findings": [], "line_file": "CLAUDE.md",
          "removed": { "allowlist": true, "deny": true, "guide": true, "hook": true,
                       "jigc_dir": true, "precommit": true, "reference": true } }
stderr: warning: removing `.jigc/` also removes 5 tracked file(s) under it: …
                                                                       -> all four driver verdicts hold
```

```
### RR-7 — rows 32, 25 · `uninstall` precedence 1-of-3 with its route, then the clean forced abandon
setup:  fresh rig; jigc milestone provision axis-three-probe
        printf 'wip\n' > .jigc/worktrees/first-sub/wip.txt
        jigc start --workflow quick-fix "tidy the readme"; printf 'private\n' > .jigc/notes.txt
argv:   jigc uninstall
exit:   1
out:    blocking · uninstall.dirty-worktree — `.jigc/` holds 1 fan-out sub-task worktree path(s) …
          .jigc/worktrees/first-sub: ?? wip.txt — it is a live git worktree holding uncommitted
            work, registered here or not; registered as a worktree of this repository
          route: get the work out of those paths first …, then re-run `jigc uninstall` — or
            abandon the milestone with `jigc milestone discard <milestone-id> --force`, which
            destroys the uncommitted work in the path(s) registered here, and re-run
            `jigc uninstall`; `jigc uninstall --force` deletes them with the install
                # row 32's asserted route: BOTH consents named — holds
argv:   jigc task discard tidy-the-readme --force ; jigc milestone discard axis-three-probe --force
exit:   0
out:    warning: removing the fan-out worktree .jigc/worktrees/first-sub discards work that is
          not in git:  wip.txt (never staged)  … not recoverable.
        discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
after:  ls .jigc/worktrees -> (empty) ; .jigc/notes.txt present     -> both driver verdicts hold
```

---

## 5 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling
(`crates/cli/src/cli.rs:1669`):

| leaf | rows driven | source |
|---|---|---|
| `milestone provision` | driver 1–17 · RR-1 | driver + reconciliation |
| `milestone discard` | driver 18–29 · RC-2 · RD-2 · RR-2/3/4/7 | driver + reconciliation |
| `uninstall` | driver 30–48 · RC-4a · RD-3 · RD-4 · RR-5/6/7 | driver + reconciliation |
| `milestone finalize` | driver 49–52 · RC-3 | driver + reconciliation |
| `task discard` | driver 53–60 · RC-1 · RD-1 · RD-3 | driver + reconciliation |
| `task finalize` | RC-4b | **reconciliation only** — not in `DESTROYING_DOORS`, reached by driving C-1's class |

Supporting leaves driven only to **build or read** a state, not as a door under test (listed so
the set above is not read as wider than it is): `milestone create`, `milestone add-task`,
`start`, `doc create`, `doc set-field`, `doc set-slot`.

---

## 6 · Notes

- **The reconciliation's one substantive addition is a class, not a row.** Codex's three claims
  are one finding, and driving it found two more doors than the source read did — including
  `task finalize`, which no registry in this axis carries. A source pass that greps removal
  sites finds the sites; only driving tells you which ones actually take a user's bytes.
- **The driver's four defects all stand**, re-driven here on the same binary. One supporting
  sentence inside D-4 is corrected (see RD-4) — the finding is unaffected.
- **Fourteen rows were demoted on evidence and re-promoted on re-drive.** No verdict changed,
  which is the best case for the driver's judgment and the worst case for its §3: a table that
  asserts 60 blocks and ships 46 cannot be audited without re-running it.
- **What neither pass reached**, carried forward rather than implied: the driver's own §4 list
  stands unchanged (the `finalize × --force` axis that does not exist, the registered-`File`
  cell that cannot exist, the hook-rejection cells that belong to axis 4, and
  `unverified_workbench_finding`'s own fail-closed cell). The reconciliation added no rig for
  any of them; each remains **not driven**, stated rather than inferred.
- **Driver observation O-1** (the third reject envelope, a bare `Finding` at
  `jigc uninstall --format json`) is left where the driver put it — axis 5's call. The
  reconciliation neither promoted nor refuted it; it was not re-driven.
