<!-- persisted verbatim 2026-09-10 from the evidence-check-1.0 session; agent: Claude Opus 5 (1M context); see VERDICT.md -->
# Adversarial probe of `1.0.0-rc.14` — data loss / repo destruction / silent corruption

Binary: `/Users/maurice/projects/gherrink-jigc/target/release/jigc` (1.0.0-rc.14), HEAD `bd348a83`.
All corpora built with `dev/jigc-rig <state> --binary <release>`; every probe snapshotted
`git status --porcelain`, `git ls-files -s`, `git rev-parse HEAD` and a sha256 of every
non-`.git` file before and after the door, and diffed them.

**Verdict: one blocking-class finding, on an axis no registry covers — HEAD posture.
The M50 sweeps themselves held everywhere I could reach them.**

---

## 1. BLOCKING — no door asks whether HEAD is on a branch; on a detached HEAD every committing door lands its commit on no branch at exit 0, and `task finalize` destroys the only other copy in the same act

**The axis.** `grep -rn "symbolic-ref\|symbolic_ref\|rev-parse --abbrev-ref" crates/cli/src
crates/engine/src` returns **zero** hits. The word `detached` appears only in comments about
jigc's *own* fan-out worktrees (which are detached by design). `COMMITTING_DOORS` has ten
members; **HEAD posture is not a cell in any registry** — not `WORK_UNIT_ID_DOORS`, not
`SLUG_DOORS`, not `ROOT_KNOBS`, not `DESTROYING_DOORS`. This is the sibling nobody swept.

### 1a. `jigc task finalize` — the authored prose leaves the tree at exit 0

```sh
rig=$(dev/jigc-rig refs-post-hoc --binary /Users/maurice/projects/gherrink-jigc/target/release/jigc) || exit; eval "$rig"
cd "$REPO"; T="$RIG_TASK"
$JIGC doc set-field "commit:$T#header/type"  --task "$T" --value docs
$JIGC doc set-field "commit:$T#header/scope" --task "$T" --value vision
echo "ground the vision" | $JIGC doc set-slot "commit:$T#summary" --task "$T" --from-file -
echo "body"             | $JIGC doc set-slot "commit:$T#body"    --task "$T" --from-file -
printf 'UNIQUEPROSE42\n' | $JIGC doc set-slot "vision:vision#thesis" --task "$T" --from-file -
git checkout -q --detach HEAD          # any bisect, any `git checkout <sha>`, any CI checkout
$JIGC task finalize "$T"; echo "exit=$?"
git checkout main                       # the ordinary next act
grep -c UNIQUEPROSE42 VISION.md
```

Measured:

| point | state |
|---|---|
| staged copy before | `.jigc/tasks/<id>/docs/vision:vision.md` holds `UNIQUEPROSE42` |
| `task finalize` | **exit 0** — `finalized ed1ca33 — docs(vision): ground the vision / promoted VISION.md / 1 file committed` |
| mention of branch posture in that output | **0** (`grep -ci 'detach\|branch'` → 0) |
| `.jigc/tasks/` after | **empty** — the working area is gone |
| `git branch --contains HEAD` | `(HEAD detached from b8a00cf)` — **on no branch** |
| after `git checkout main` | `grep -c UNIQUEPROSE42 VISION.md` → **0** |
| `jigc validate` after | **exit 0 body, blocking row**: `file-state.hash-matches — on-disk content of VISION.md differs from the recorded state`, routed at *"review the out-of-band edit"* — there was no out-of-band edit |
| `jigc doc show vision:vision#thesis` after | serves the **old** committed text, silently |

So: exit 0, the task's working area (the only non-git copy) destroyed, the result on a commit
connected to no branch, jigc silent about all three, and the store left asserting a
non-existent out-of-band edit. The bytes survive only as a dangling commit until `git gc`
expires it, and the **only** warning in the whole sequence comes from *git*, at checkout time,
about a commit the user never knowingly made.

### 1b. `jigc milestone create` — the same posture bricks a milestone

```sh
rig=$(dev/jigc-rig committed-singletons --binary <release>) || exit; eval "$rig"; cd "$REPO"
git checkout -q --detach HEAD
$JIGC milestone create "Wave One"; echo "exit=$?"      # exit 0
git checkout main
ls docs/milestone-records            # -> No such file or directory
ls .jigc/milestones                  # -> wave-one          (workbench survives)
$JIGC milestone list-tasks wave-one  # -> exit 0, "milestone:wave-one tasks (0):"
$JIGC validate                       # -> exit 1
```

`milestone create` exits 0 and commits the record (`e9a147d chore(milestone): open record for
milestone:wave-one`) **off-branch**. The `.jigc/milestones/wave-one` workbench is *not* on any
commit, so it survives the checkout while its committed record does not. Result:
`list-tasks` answers happily, `doc show milestone-record:wave-one` blocks `store.not-found`,
and `jigc validate` blocks `reconciliation.rename` routed at
`jigc unmanage docs/milestone-records/wave-one.md` — **the route drops the record rather than
restoring it**. This is the exact "record-only door bricks the milestone" class M47 Increment 4
shipped a fix for, reachable again through the un-swept axis.

### 1c. `jigc setup` — same shape, lower stakes

`jigc setup` on a detached HEAD makes its install commit off-branch at exit 0 with no mention
(`grep -ci 'detach' → 0`). `git checkout <branch>` then removes `.jigc/AGENT.md`,
`CLAUDE.md`, `.claude/settings.json` etc. — the install silently un-installs. Recoverable by
re-running `setup`, so this cell is annoyance rather than loss; it is listed because it shows
the axis is the door registry, not one verb.

**Why I call 1 blocking.** It satisfies the brief's rule literally (exit 0 + bytes gone from
the tree + the store left lying), and it is *specifically* the failure mode this project has
shipped four fixes for: a door that destroys the only workbench copy while landing the result
somewhere ordinary use will drop. The honest mitigation: the bytes are in a dangling commit
for the reflog window, and git — not jigc — warns once. The fix is one `git symbolic-ref -q
HEAD` at the ten `COMMITTING_DOORS`, with a `--force`-shaped consent if the fan-out worktree
path (which is detached on purpose) needs an exemption.

---

## 2. NON-BLOCKING — two destroying doors narrate a destruction they do not perform, and name tracked committed files as "not recoverable"

M50 Increment 12's stated rule: *"a `LeftoverShape` every destroying door reads, so no door
names one of the two things it would destroy and none narrates a removal it then fails to
perform."* Both halves are false over a **symlinked leftover**.

```sh
rig=$(dev/jigc-rig committed-singletons --binary <release>) || exit; eval "$rig"; cd "$REPO"
$JIGC milestone create "Wave One"; $JIGC milestone add-task wave-one "first bit"
$JIGC milestone add-task wave-one "second bit"; $JIGC milestone provision wave-one
git worktree remove --force .jigc/worktrees/first-bit
ln -s "$REPO/docs" .jigc/worktrees/first-bit          # symlink INTO the repo's own docs/
$JIGC milestone provision wave-one           # refusal
$JIGC milestone provision wave-one --force   # exit 0
```

Refusal (`milestone.leftover-holds-work`) says:
`.jigc/worktrees/first-bit: the file itself — it is a file, not a worktree` — for a symlink to
a **directory**.

`--force` then says:

```
warning: removing the leftover directory .jigc/worktrees/first-bit discards work that is not in git:
    decisions-log.md
    milestone-records
    roadmap.md
  note: the leftover directory is the only copy of these bytes — they are not recoverable.
```

Measured after: `docs/roadmap.md` and `docs/decisions-log.md` sha256 **unchanged**; both are
tracked and committed. The door enumerated *through* the link, named three of the repo's own
committed docs, asserted they were unrecoverable, and then deleted only the symlink.
The same text appears at `jigc uninstall --force`. `jigc milestone discard --force` over the
identical planted state prints **nothing at all** and removes the link silently — so three
destroying doors give three different answers about one path.

No bytes lost — this is a law-1 lie, not a loss. It matters because it trains an operator to
disbelieve the one warning that stands between them and a real `--force` deletion.

---

## 3. NON-BLOCKING — `jigc setup` commits the user's uncommitted work at exit 0, with no carryover gate and no mention

```sh
rig=$(dev/jigc-rig committed-singletons --binary <release>) || exit; eval "$rig"; cd "$REPO"
printf '\nUSER NOTE THE USER HAS NOT COMMITTED YET\n' >> CLAUDE.md
git status --porcelain          # " M CLAUDE.md"
$JIGC setup; echo "exit=$?"     # exit 0
git status --porcelain          # ""   <- the user's edit is now committed
git show HEAD -- CLAUDE.md | grep -c "USER NOTE"   # 1
```

The user's uncommitted note lands inside `chore(jigc): install jigc workspace config`, authored
by jigc, with zero mention (`grep -ci 'uncommitted|your edit|carried' → 0`). The same happens to
an edited `.jigc/.gitignore`. M43's carryover gate covers the task-minting doors and `finalize`;
`setup` is a committing door outside it. **Not** a pathspec leak: an unrelated staged
`feature.txt` was correctly left alone (verified on the `bare` state) — the swallow is confined
to files setup's own pathspec names, which is exactly where a user's edits also live.

---

## 4. NON-BLOCKING — the root-knob value rule refuses control characters but not spaces; a whitespace-only `docs-root` lands tracked bytes at a path nothing can name

```sh
$JIGC config set docs-root "   "        # exit 0: "set `docs-root` = `   `"
$JIGC config set docs-root $'\t'        # exit 1: config.value-rejected, "must not contain control characters"
```

Follow-through (create an ADR and finalize):

```
finalized 8e3f38d — docs(adr): s
  added /decisions/some-decision.md
git ls-files ->    /decisions/some-decision.md      # a directory literally named three spaces
$JIGC doc list ->  adr:some-decision     /decisions/some-decision.md  managed
$JIGC validate ->  exit 0, clean
$JIGC config get docs-root ->  "docs-root =      (project)"    # visually identical to unset
```

`-` is accepted the same way. Recoverable (tracked), so not loss — but it is the M50 Tier-1
value rule (*"neither root knob accepts a home or a value that leaves the store lying"*) with
the space token un-swept, and `config get` reading back a value indistinguishable from unset is
precisely "the store lying."

---

## 5. NON-BLOCKING — smaller

- **`jigc migrate .git/config --as changelog` exits 0** and mints a migration task seeded from a
  file inside git's own directory (`.jigc/tasks/migrate-changelog-git-config-…/source`). Every
  other door treats `.git/` as off-limits by name (`config.untrackable-root`: *"git refuses to
  track any path with a `.git` component"*). `migrate`'s source path is checked for readability
  only.
- **`jigc doc rename … --slug <300 chars>` fails with a bare `File name too long (os error 63)`**
  — no finding code, no route, no `at:`. State intact (the staged doc survives), so it is a
  route-floor gap, not a loss.
- **N23 re-driven, and the ledger's classification is right.** With a project pack that defines
  no doctypes (`.jigc/config/packs.yaml` → a minimal pack), `jigc validate` prints *"no findings
  — the committed store validates clean"* at **exit 0** while `jigc doc list` drops 3 of 4
  managed rows entirely. `git ls-files` still carries every file and every sha is unchanged:
  **registration loss, not byte loss.** Not misfiled as surface-tier.
- **`doc retitle-item` mis-quotes a traversal address** — `#milestones/../../../etc` is refused
  as `no section "milestones/../.."` (one component dropped from the echo). Cosmetic.

---

## 6. Cell matrix actually run

`Δ` = any byte changed outside `.jigc/index/edges.json` (a benign derived cache that rebuilds on
several read paths).

### Axis 1 — degenerate tokens at path-taking / id-taking doors

| door | token | exit | verdict |
|---|---|---|---|
| `task discard` | `""` (empty) | 1 | refused, no Δ (M50 fix holds) |
| `task discard` | `.jigc` | 1 | `work-unit.malformed-id`, no Δ |
| `task discard` | `docs` | 1 | `no task docs`, no Δ |
| `unmanage` | `../../etc/hosts`, `.git/config`, `docs/../../outside/x.md`, `""`, `.`, `..`, `docs`, `docs/` | 0 (all) | "no-op: not managed", **no Δ** |
| `relocate --from` | `../..`, `.git`, `""` | 1 | refused (frozen doctype / missing arg), no Δ |
| `migrate <src>` | `../../outside/x.md` | 1 | unreadable, no Δ |
| `migrate <src>` | `.git/config` | **0** | **task minted from inside `.git/`** — §5 |
| `migrate <src>` | `""` | 1 | unreadable, no Δ |
| `config set docs-root` | `../../outside`, `..` | 1 | `config.untrackable-root` |
| `config set docs-root` | `/etc` | 1 | `config.unusable-root` |
| `config set placement-root` | `../..`, `.git/hooks` | 1 | `config.untrackable-root` |
| `config set docs-root` | `$'a\nb'`, `$'\t'`, `$'a\rb'` | 1 | `config.value-rejected` (control chars) |
| `config set docs-root` | `"   "`, `-` | **0** | **accepted** — §4 |
| `config set docs-root` | `.`, `Docs`, `docs/`, `docs/../docs` | 0 | accepted; no move, corpus intact, validate green |
| `doc create --slug` | 300×`a` | 1 | refused (create-gate), no Δ |
| `doc create --slug` | `a​b` | 1 | not-a-slug refusal, no Δ |
| `doc rename --slug` | 300×`a` | 1 | ENAMETOOLONG, bare error — §5; staged doc intact |
| `doc show` | `vision:../../../../../../etc/passwd` | 1 | `store.malformed-slug`, no Δ |
| `doc show` | `vision:vision#thesis/../../..` | 1 | `store.no-such-item`, no Δ |
| `doc retitle-item` | `#milestones/../../../etc` | 1 | `write.unknown-section`, no Δ |
| `doc retitle-item` | `--title ""` | 1 | `write.wrong-shape`, no Δ |
| `doc remove-item` | `#milestones/..` | 1 | `write.not-present`, no Δ |
| `doc set-field --unset` | `#milestones/m-alpha/../../x` | 1 | `write.unknown-section`, no Δ |
| `rename --slug` | an existing doc's slug (2 committed ADRs) | 1 | `write.already-present`, both files byte-identical after |
| `rename --to` | case-variant of its own slug (case-insensitive APFS) | 0 | correct no-op, no Δ |

### Axis 2 — symlinks planted where a door reads, writes, or deletes

| plant | door | exit | verdict |
|---|---|---|---|
| `.jigc/tasks/<id>/docs` → `$REPO/docs` | `task discard --force` | 0 | link removed, `docs/` **untouched**, index identical |
| `.jigc/tasks/<id>/docs/vision:vision.md` → outside file (non-conformant) | `doc set-slot` | 1 | `write.non-reparseable`, target untouched |
| same, **conformant** outside target | `doc set-slot` | 0 | **symlink replaced by a regular file** (atomic temp+rename); outside target byte-identical |
| `.jigc/linked-file`, `.jigc/linked-dir` → outside | `uninstall` | 1 | `uninstall.untracked-workbench-file`, nothing removed |
| same | `uninstall --force` | 0 | links removed, outside targets intact |
| `.jigc/worktrees/<sub>` → outside dir | `milestone provision` | 1 | refusal (mis-shaped, §2) |
| same | `milestone provision --force` | 0 | link removed, target intact — **narration lies**, §2 |
| `.jigc/worktrees/<sub>` → `$REPO/docs` | `provision --force`, `uninstall --force`, `discard --force` | 0 | `docs/` byte-identical at all three; narration lies at two, silent at the third |
| `docs/decisions` → outside dir | `doc create` + `task finalize` | 3 | promote wrote through the link then **rolled back cleanly**; `git add` refused (*beyond a symbolic link*); outside dir back to its pre-state |
| `docs/decisions` → outside dir **holding the colliding filename** | `doc create` + `task finalize` | 3 | conformance gate read the pre-existing file and blocked; **outside file sha unchanged** |
| committed managed doc replaced by a symlink outside | `validate` / `doc show` | 1 / 1 | drift caught (`file-state.hash-matches` + `unadopted-instance`); outside target untouched |

**No door followed a symlink to destroy or overwrite anything.** Writes go through an atomic
temp+rename that breaks the link; deletions do not follow.

### Axis 3 — rollback pre-image under a rejecting `pre-commit`

| staged state | door | exit | verdict |
|---|---|---|---|
| `git rm` of a tracked managed doc + `git mv` + untracked file at a managed home + staged `src.txt` | `task finalize` | 3 | blocked at the conformance/reconciliation gate before commit; index, status, HEAD and every sha identical |
| same + `--carry-staged` | `task finalize` | 3 | identical |
| `git mv` of a tracked file + untracked file at a managed home, commit doc complete | `task finalize --carry-staged` | 3 | `finalize.base-mismatch`; **HEAD, `git ls-files -s`, `git status --porcelain` and every file sha byte-identical** |
| `docs/decisions` symlinked out, complete commit doc | `task finalize` | 3 | `finalize.stage-failed` — *"no commit was made and the promotions were rolled back"*, and the promoted file really was removed from the link target |
| clean corpus, rejecting hook | `migrate-corpus` | 0 | *0 migrated, 1 already current* — **the hook was never reached; this cell did not test rollback** (see §7) |

### Axis 4 — concurrency, 50 rounds

Two live tasks, `doc set-slot` into each in parallel (`&` + `wait`), 50 rounds, against the
shared `.jigc/state/file-state.json` and `.jigc/index/edges.json`:
**fails=0, corrupt=0.** Every round: both staged docs present, both writes readable back,
both JSON files parse. `jigc task validate` clean for both afterwards. No torn or lost write.

### Axis 5 — re-install over user-edited state

| edited | `jigc setup` result |
|---|---|
| `.claude/skills/jigc/SKILL.md` | **preserved**; advisory `adapter-guide.user-modified` with a route |
| `.claude/settings.json` (custom key) | key preserved, jigc keys re-added |
| `.git/hooks/pre-commit` (user's own) | **wrapped** — user's body preserved verbatim below jigc's block |
| `.jigc/config/cascade.yaml` (hand-written) | untouched |
| `CLAUDE.md` / `.jigc/.gitignore` uncommitted edits | bytes preserved but **silently committed** — §3 |
| unrelated staged `feature.txt` | correctly **not** committed (pathspec-limited) |

### Axis 6 — carried defects

| entry | driven | verdict |
|---|---|---|
| N23 (retire a doctype over a committed corpus) | yes | **registration loss only, bytes intact** — classification correct, not a misfiled loss cell |
| N20, N26, N27, N28, N31 | no | see §7 |

### Axis 7 — git posture

| posture | door | exit | verdict |
|---|---|---|---|
| **detached HEAD** | `task finalize`, `milestone create`, `setup` | 0 | **§1 — blocking** |
| second branch checked out in a linked worktree | `task finalize` | 0 | correct: `main` advanced, `side` untouched |
| shallow clone (`--depth 1`) | `validate` / `start` / write / `task finalize` | 0 | clean throughout; advisory `file-state.un-baselined` as expected |
| `core.worktree` set | `validate` | 0 | clean |
| git submodule at `docs/subm` | `ingest` / `validate` | 0 | submodule not crossed, not adopted, intact |
| case-insensitive FS, `Docs` vs `docs` as `docs-root` | `config set` | 0 | accepted; existing corpus untouched (all four singletons are placement doctypes, so nothing was strandable) |

---

## 7. What I did **not** run

- **`jigc milestone finalize` under a rejecting hook, on both `squash: true` and `squash: false`.**
  The whole fan-out → execute → join → finalize arc was reached only as far as
  `provision`/`discard`; the join and the boundary commit were never driven. N20 (the boundary's
  non-hook refusal discarding its survivable frame) is therefore unverified here.
- **`migrate-corpus --approve` rollback with real pending migrations.** The `migrated` rig had
  nothing to migrate, so the rejecting-hook cell above proves nothing about that door. A corpus
  with a genuinely stale `schema-version` was not constructed.
- **`jigc relocate` on a freeze-exempt doctype** with real stranded instances (I only reached the
  frozen-doctype refusal).
- **N20 / N26 / N27 / N28** — not driven.
- **A real case-collision** (`Docs/x.md` vs a tracked `docs/x.md` for the same filename) on APFS.
- **NUL-adjacent tokens** — the shell cannot pass a NUL in an argv, so the cell is unreachable
  from bash; it needs a Rust or Python `execve` harness.
- **`doc author --from-file` with a payload whose declared addresses carry traversal** — I drove
  the address doors individually but not the batch verb's payload grammar.
- **A `>255`-char *work-unit id*** at the resolve seams (I tested it at `--slug` doors only).
- **Two `jigc task finalize` runs racing each other** (axis 4 covered concurrent *writes*, not
  concurrent *commits* against one index).
