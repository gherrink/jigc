# M51 — the eight per-axis review matrices

**What this is.** The second of the two things M51's `acceptance-spiked` gate cell halts on. §17 rules
that M51 ships flow 52's arm set *and* **eight per-axis review matrices**; D15 rules that the matrices are
driven on the **built and installed** binary, after the completion audit's fixes land; §19
([settle-record.md](../settle-record.md) → *§19 — D15: the acceptance design is adopted*) adopts the
[acceptance design](../acceptance-design.md) as decided rather than as a draft — the eight door-set
derivations, the driver/source-pass split, the reconciliation rule, and the strengthened coverage fence.
This directory is that instrument's **persisted record**: what was driven, what it found, what it refused
to claim, and which of the 47 `VERB_KINDS` leaves each axis reached.

**The binary.** Every row in every file here was driven on the installed release
**`jigc 1.0.0-rc.15`**, built from commit **`35195f56`** (*"docs(m51): completion audit closed — five
findings, five fixed, rc.15 stamped after"*), on **2026-09-16**. Fixtures were built with
[`dev/jigc-rig`](../../../dev/jigc-rig) (every root from `mktemp -d`, so nothing needed teardown and the
`rm -rf $V/$D` shape never appears) and by driving the binary; nothing was written into the working
repository.

**An axis matrix row** is `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted,
verdict}`. A row is **driven** iff its argv ran on that binary and its verdict carries a **repro block**.
**A verb is covered iff it is the door of ≥1 driven row.** A classification-only row — a leaf proven by a
⇔ fence rather than by driving — confers **no** coverage, or the fence would measure a table instead of
the binary.

## The staffing

Per axis, three agents, adopted from §19:

- **One Opus driver** owns the `(door, cell)` table. It drives every row on the installed binary and
  records exit, code, route, surface and a repro block per row. **It may not mark a row driven from a
  source read.**
- **One Codex source pass** (`codex/axis-N-source-pass.md`, verbatim, so every lead is auditable) owns
  **completeness of the row set** — the question driving cannot answer: *is there a door the registry does
  not carry, or a bypass of the seam this axis is about?* It reads; it drives nothing.
- **One reconciler** (a separate Opus agent, which did **not** author the driver file it reconciles) merges
  the two: it re-drives every driver defect once, enters every Codex claim as a lead and drives it, and
  audits the driver's table for rows marked driven that carry no repro block (**demotion pass**).

## The reconciliation rule

> *A claim by one that the other cannot reproduce is a **lead, not a finding**.*

A Codex claim with no driven repro enters the table as `lead(codex, <claim>)` and is either **driven to a
repro block** — at which point it is a finding — or **recorded REFUTED with its falsifying datum**. An Opus
row the source pass says cannot happen **stays a finding** (it was driven), and the source claim is
recorded refuted with the datum that refutes it. Silence is not refutation. **No fix ships on a source read
alone, and no completeness claim ships on driving alone** — M46's planning correction (*an agent's report
is a lead, not a measurement*) and M50's two planning halts (*a claim about how the composed product
behaves is not established by reading the files it is composed from*).

## Roll-up

| axis | subject | CONFIRMED | REFUTED | OPEN | doors covered |
|---|---|---|---|---|---|
| 1 | caller tokens | 14 | 1 | 2 | 39 |
| 2 | posture | 5 | 0 | 1 | 22 |
| 3 | destroying doors | 5 | 0 | 0 | 6 |
| 4 | transaction / rollback | 6 | 1 | 0 | 13 |
| 5 | pinned contracts | 6 | 2 | 0 | 47 |
| 6 | composed surfaces | 9 | 1 | 4 | 23 |
| 7 | freeze & migration | 6 | 0 | 0 | 10 |
| 8 | adopter docs & help | 5 | 0 | 0 | 23 |
| **total** | | **56** | **5** | **7** | **47 / 47 union** |

CONFIRMED counts two kinds of ledger row, both carrying repro blocks, and they are separated below.
**§A** lists every CONFIRMED finding that names a **defect or a contradiction** — 39 entries across the
eight axes, of which four originate as Codex claims driven by the reconciler (axis 2's displacement claim,
axis 3's C-1, axis 4's C1–C4) and one (axis 7's C-2) is confirmed behaviour **already on the record as a
declared residual**, recorded as an amplifier rather than a new defect. **§B** lists the source passes'
**positive completeness claims** that were driven and held, cross-referenced where the claim is itself one
of §A's defects. The per-axis counts in the table above are each ledger's own roll-up.

---

# FINDINGS

Each row below carries the repro block in its **condensed** form — the setup, the argv, and the observed
lines that carry the verdict. The **full** block, with every cell of the surrounding table, is in the axis
file cited on the row.

## A · CONFIRMED defects and contradictions (39 entries)

### Axis 1 · caller tokens — [axis-1.md](axis-1.md) §3, §11

**A1-D1 (HIGH) — a bogus `<slug>` head on a singleton is accepted by five write doors, and `finalize`
silently drops all but one authored payload.** Doors: `doc set-slot` · `doc add-item` · `doc retitle-item`
· `doc remove-item` · `doc set-field`; consequence door `task finalize` (∈ `COMMITTING_DOORS`). The cell is
the *well-formed control* — the pass cell.

```
setup: rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
       jigc start --workflow single-task "repro"
$ jigc doc set-slot "adr:ghost#context" --from-file - </dev/null
  exit 1 — no staged instance … provision it first          ← non-singleton REFUSED
$ printf 'PAYLOAD-ALPHA\n' | jigc doc set-slot "vision:alpha#thesis" --from-file -   → exit 0
$ printf 'PAYLOAD-BETA\n'  | jigc doc set-slot "vision:beta#thesis"  --from-file -   → exit 0
$ jigc doc show vision:alpha --task repro
  exit 1  blocking · store.not-found — `vision` is a singleton, so its only address is `vision:vision`
$ ls .jigc/tasks/repro/docs/   → vision:alpha.md  vision:beta.md   ← two path components from the token
$ jigc task finalize repro     → exit 0 · "promoted VISION.md · 1 file committed"
$ sed -n '/## Thesis/,/## Invariants/p' VISION.md   → PAYLOAD-BETA   ← ALPHA gone, no finding
```

Contradicts `reject_malformed_slug_head`'s own doc-comment (*one code for the whole family, the read doors
and the write doors alike*) and M51's claim clause at a committing door. Honest qualification carried in
the axis file: `finalize` does print one `file-state.staged-copy` **advisory** per staged copy, whose route
is *"no action needed"* — so it is not literally silent, but no surface says a payload was dropped.

**A1-D2 (MEDIUM-HIGH) — `jigc relocate <freeze-exempt type> --from .jigc` moves jigc's own install
artifacts into the doctype's home at exit 0.** Door: `relocate` (a `MovesOnBehalf` door). Cell: workbench
root.

```
setup: rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc --pack-from-dev) || exit; eval "$rig"
$ jigc relocate adr --from .jigc
  freeze-exempt relocation: 1 moved · moved .jigc/AGENT.md -> docs/decisions/AGENT.md     exit 0
$ jigc relocate adr --from .claude
  moved .claude/skills/jigc/SKILL.md -> docs/decisions/SKILL.md                            exit 0
```

The selector is pure prefix containment over `git ls-files` filtered to `.md` — no stamp, no conformance,
no identity. The two sibling doors refuse the same token (`config set docs-root .jigc` →
`config.workbench-root`; `jigc migrate .jigc/version` → `migrate.source-untrackable`). Recoverable (the
rename is staged, HEAD unmoved); adopter-reachable through a PB-1 project-pack doctype.

**A1-D3 (MEDIUM) — the `relocate`/`from` registry row's declared runnable argv cannot reach its own arm.**

```
argv: jigc relocate <ty> --from docs/old   for all 14 shipped doctypes
observed: 14/14 exit 1, CODE-LESS, route-less —
  "`<ty>` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`"
```

`PathArgArm::argv`'s doc-comment promises *a runnable argv … so the token is the only thing the door can
fault on*; driven, the doctype faults first, unconditionally, so the axis fence proves nothing about the
token.

**A1-D4 (MEDIUM) — the OS-name-ceiling cell at the address `<slug>` head leaks a bare OS error at five
write doors.**

```
argv: jigc doc set-slot "vision:$(python3 -c "print('a'*300)")#thesis" --from-file -
observed: exit 1 — could not copy in `vision:<A300>#thesis` for editing: File name too long (os error 63)
  (no `blocking ·`, no code, no route, no `at:`; --format json → {"error": …})
same at doc add-item · doc remove-item · doc retitle-item · doc set-field
boundary walk: 166→0 · 240→1
```

This is the surface `SLUG_NAME_CEILING_CODE`'s doc-comment names as the defect M51 EC-28 fixed — fixed over
the `--slug` family (6/6 verified) and not over the address family, whose `<slug>` head becomes the same
path component. The counter-argument (the fix's scope sentence is *one flag, one ceiling, six doors*) is
recorded in the axis file.

**A1-D5 (MEDIUM) — `jigc config set docs-root <component over NAME_MAX>` is accepted at exit 0 and leaves
`jigc validate` unable to run.**

```
$ jigc config set docs-root "$(python3 -c "print('a'*300)")"    → exit 0
$ jigc validate                       # measured BARE, not through a pipe
  validating the committed store at "…": File name too long (os error 63)      exit 1
```

`config set`'s declared predicate covers absolute · edge whitespace · pathspec magic · symlinked or
file-shaped component — **not nameability**. The driver's own first measurement (exit 0 through `| head`)
is corrected in place: there is **no false green**; the defect is the accepted home and the code-less
refusal.

### Axis 2 · posture — [axis-2.md](axis-2.md) §4, §R.1–R.2

**codex-1 — `jigc relocate` mutates the index with `git rm --cached` under a live merge, with no posture
re-probe immediately before that mutation** (origin: Codex, driven by the reconciler).

```
setup: fresh rig --pack-from-dev; a committed destination squatter; a `git` shim that fires ONE real
       merge after the first `ls-files` and then passes through
$ jigc relocate adr --from docs/old
  exit 0 — freeze-exempt relocation: 0 moved, 0 displaced, 4 blocked
    blocking · repo.operation-in-progress (per path)
$ git status --short →  D  docs/decisions/zsquat.md      ← the index WAS mutated under the live merge
  trace: … ls-files -z → SHIM fires (MERGE_HEAD) → rm --cached … ← no probe before it
```

Reachability is race-only and stated as such; a positive control shows the door refusing when the marker
is present at the door's own probe.

**D1 — a commit-on-behalf door never reports `repo.operation-in-progress` under a rebase or a bisect, and
the code it reports instead routes at a command git refuses.**

```
state: .git/rebase-merge present, HEAD detached (git's own doing)
$ jigc task finalize <id>
  blocking · repo.head-detached … route: re-attach HEAD with `git switch <branch>`      exit 1
$ git switch main
  fatal: cannot switch branch while rebasing                                            exit 128
the MOVER in the same state: blocking · repo.operation-in-progress …
  route: conclude it, or abandon it with `git rebase --abort`                           exit 1
bisect: same mask; `git switch` warns and exits 0 while BISECT_LOG survives
```

`posture()` returns breaches in `PostureMember::ALL` order and both seams take the **first** match, so
`HeadDetached` masks `OperationInProgress` at all 10 commit-on-behalf doors. The right answer is in the
binary, one class over, in the same run.

**D2 — `rebase-apply` is `git am`'s marker too, and jigc names *a rebase* and routes at `git rebase
--abort`, which git refuses.** HEAD stays **attached** in this cell, so all 12 acting doors reach it.

```
state: git am stopped on a conflict → .git/rebase-apply present, HEAD = refs/heads/main
all 12 doors: blocking · repo.operation-in-progress — a rebase is in progress …
  route: … abandon it with `git rebase --abort`                                          exit 1
$ git rebase --abort
  fatal: It looks like 'git am' is in progress. Cannot rebase.                            exit 128
```

`InProgress::markers()`' own doc-comment already says *"rebase-apply/ (the am backend)"* — the source knows
what it is looking at and the message still says rebase (law 1).

**D3 — an un-concluded cherry-pick or revert is not a member of the family, and `jigc task finalize`
concludes it under jigc's own subject at exit 0.**

```
state: CHERRY_PICK_HEAD present, HEAD attached
$ jigc task finalize <id>              → exit 3 · finalize.carried-staged, route: --carry-staged
$ jigc task finalize <id> --carry-staged
  finalized 9ddeed3 — feat(axis): land under a cherry pick                                 exit 0
$ ls .git/CHERRY_PICK_HEAD             → GONE
(identically for REVERT_HEAD)
```

The family's markers are 4 of the 6 git writes. The route in is jigc's own: a **staging** consent
(`--carry-staged`) silently also concludes the user's cherry-pick — exactly the damage
`ActsOnBehalf::CommitsOnBehalf`'s doc-comment exists to prevent.

**D3b — git's own refusal during a cherry-pick is dressed as a pre-commit hook rejection, with no code and
no route.**

```
$ jigc milestone create cpm2
  `git commit` was rejected (no commit was made):
  fatal: cannot do a partial commit during a cherry-pick.
  … Fix the hook's complaint, then re-run `jigc milestone create cpm2`.                    exit 1
```

No hook exists in the rig. The rollback itself is correct and was verified (log unmoved, no record on
disk); the frame names a mechanism that did not act.

### Axis 3 · destroying doors — [axis-3.md](axis-3.md) §5, §2 of the ledger

**C-1 class (origin Codex, driven — and wider than claimed): a non-`.md` file under a task's working area
is destroyed at exit 0 by five doors, with no refusal and no narration.** Codex named three doors; driving
found **five**, including `jigc task finalize`, which is in no destroying registry at all.

```
setup: fresh rig; jigc start --workflow quick-fix "tidy the readme"
       printf 'PRECIOUS NOTES\n'  > .jigc/tasks/tidy-the-readme/notes.txt
       printf 'ATTACHED BYTES\n'  > .jigc/tasks/tidy-the-readme/docs/attachment.txt
$ jigc task discard tidy-the-readme      → exit 1 · task-discard.staged-prose  (fires on the .md only)
$ rm -f .jigc/tasks/tidy-the-readme/docs/*.md ; jigc task discard tidy-the-readme
  discarded task tidy-the-readme                                    exit 0, stderr empty
  after: .jigc/tasks/tidy-the-readme → No such file or directory    both plants gone, named by nothing
same class at: milestone discard · milestone finalize · uninstall · task finalize
  (task finalize: exit 0, "1 file committed"; notes.txt gone and in no commit)
```

`workbench_paths`' doc-comment excludes `tasks/`/`worktrees/` *"answered by the doors that own them"*;
driven, those doors answer for `docs/*.md` and nothing else, so a non-`.md` byte is answered by **no door**.
This is the shape M49 closed one layer out at `fanout_worktree_paths` (*a claim about shape where the
door's question is about bytes*) — here the shape-claim is the `.md` suffix. Severity is stated, not
assumed: the state needs someone to write into `.jigc/tasks/<id>/`, which the adapter tells agents not to
do — the same reachability argument M49 heard and rejected.

**D-1 (MEDIUM) — the staged-prose route at two destroying doors names a landing verb the state refuses.**
`task-discard.staged-prose` and `uninstall.staged-prose` both offer *"land them with `jigc task finalize
<id>` (which refuses while a required slot is empty)"*; over a **milestone sub-task** — the exact state both
doors list — that verb refuses with `finalize.milestone-sub-task` at **exit 3**, for a reason the
parenthetical excludes. The third family member, `milestone.staged-prose`, is correct — which makes this an
un-swept axis rather than a design choice.

**D-2 (LOW) — `jigc milestone discard --force` acks `workbench removed` over a teardown that failed.** At
exit 0 the ack asserts the removal unconditionally while two worktrees are still on disk and the same run
printed two `could not remove the fan-out worktree …` warnings. The loss narration beside it *is*
outcome-keyed (M50's fix, driven correct); the ack is a fixed string.

**D-3 (LOW) — the fail-closed staged-prose refusal prints the host path, and the bound that counts it
describes a different site.** `task.rs:697` composes an absolute path into a blocking finding printed by
`task discard` and `uninstall`. The site is in `UNSWEPT_PRODUCERS` with the right count and a **false
reason** (it describes all three as error-channel faults; one is a finding surface at two destroying doors).

**D-4 (LOW) — the worktrees-root fail-closed route blames `git` on PATH and omits the consent.**
`unverified_worktrees_finding`'s route opens with *make sure `git` is on PATH* for a failure its own
doc-comment identifies as `read_dir` on `.jigc/worktrees/`, and never names `--force`, while both sibling
fail-closed refusals at the same door do — verbatim the defect `design/project-setup.md`:160 records as
fixed at this door by M50, surviving at the one cell M50's repair did not cover.

### Axis 4 · transaction / rollback — [axis-4.md](axis-4.md) §5, §R1–R6

**C1 (origin Codex, driven) — the promote rollback is unconditional and emits no conflict finding.** Driven
at `task finalize` and, new, at `milestone finalize`. See DEFECT 2 below, which is the same class.

**C2 (origin Codex, driven) — `RecordPreImage` rollback restores the milestone record unconditionally.**

```
setup: a milestone with a committed record; a pre-commit hook that appends to the record and exits 1
$ jigc milestone add-task axis-four-milestone 'axis four sub task'
  `git commit` was rejected … nothing was committed — the record append and the sub-task mint were
  both rolled back                                                               exit 1
after: the racer's appended line is GONE from the record; no finding named it
```

**C3 (origin Codex, driven) — `RecordFlipGuard`'s `Drop` is a second unconditional restore**, driven at
`milestone finalize` with the racer's edit likewise gone and no finding.

**C4 (origin Codex, driven) — `milestone provision` amends `.jigc/.gitignore` and then fails without
acknowledging it.**

```
$ jigc milestone provision nonexistent-milestone
  blocking · milestone.unknown — milestone `nonexistent-milestone` does not exist          exit 1
$ cat .jigc/.gitignore   → the user's private line kept, and FOUR lines appended by the failed run
```

The registry says this ack is *"the only notice"* of the change; the run that failed gave none.

**DEFECT 1 — a rollback conflict breaks `--format json` stream discipline at the one class where the
document is on stderr.**

```
setup: committed-singletons rig; a hook that appends to .jigc/.gitignore and exits 1
$ jigc task finalize axis-four-probe --format json > out.json 2> err.txt ; echo $?   → 1
err.txt:  { "error": "`git commit` was rejected …" }
          blocking · finalize.rollback-conflict — `.jigc/.gitignore` changed while this finalize …
python: json.decoder.JSONDecodeError: Extra data: line 4 column 1
```

`carry_rollback_conflicts` prints the finding to stderr *"whatever the format"*, justified by a premise
(*the document on stdout still parses*) that is false for exactly the reject class, where stdout is empty
and the document is on stderr. Both `ConfigLayerWorktree::restore` doors reach it. The finding reaches the
machine surface **nowhere** — the `{"error": …}` envelope is single-key.

**DEFECT 2 — M51's compare-and-swap rollback discipline is applied to one of the three worktree axes, and
the other two lose a third party's bytes silently.**

```
promote axis (B4): migrate VISION.md --as vision, author, then a hook that appends to VISION.md and exits 1
$ jigc task finalize <t> --approve                                        → exit 1, ordinary frame
$ grep -c 'raced by a concurrent editor' VISION.md   → 0   the editor's bytes are GONE
$ grep -c 'rollback-conflict' <output>               → 0   nothing named it
retire axis (B5): the window's editor re-creates the retired path
$ …                                                  → the retired original is NOT put back, silently
```

`design/finalize.md`'s promise is written as a property of *the promise*: *nothing jigc wrote survives a
failure except where surviving it is the only way to avoid destroying someone else's bytes, and every such
survival is named*. `CONFIG_LAYER_SPECS` got the M51 treatment; promote and retire are a third and fourth
population of the same class and did not.

### Axis 5 · pinned contracts — [axis-5.md](axis-5.md) §8, ledger A–B

**DEFECT D (origin Codex, driven — and 47/47, larger than the claim) — the pre-dispatch `current_dir()`
failure bypasses both reject funnels, so `--format json` emits plain text on stderr.**

```
$ cd /tmp && D=$(mktemp -d …) ; cd "$D" && rmdir "$D"       # cwd no longer exists
$ jigc --format json describe                               → exit 1, stdout 0 bytes
  stderr: cannot determine the current directory: No such file or directory (os error 2)   NOT JSON
driven across every VERB_KINDS leaf:  47/47 not-JSON · 0/47 emitted either declared reject arm
```

**DEFECT A — `jigc setup` and `jigc uninstall` reject with a third, undeclared envelope shape.** Both doors
serialize a **bare `Finding`** (eight top-level keys) where the contract declares exactly two reject arms
(`{error}` and `{findings, schema_version}`), and both rows declare `ArmOrigin::Sole`. Driven in three
cells: outside a git repository, `setup.dirty-install-path` (the wave's own new refusal), and M50's
`uninstall.untracked-workbench-file`. 45 of 47 leaves answer the declared funnel in the same directory.

**DEFECT B — `jigc doc show <addr>#<repeatable-section>` is a second top-level array, declared nowhere.**

```
$ jigc --format json doc show roadmap#milestones        → exit 0 · root type ARRAY
$ jigc --format json doc show changelog#releases        → exit 0 · root type ARRAY
$ jigc --format json doc show spec:rate-limiting#criteria/limits-per-ip
                                                        → exit 0 · OBJECT, matching no declared row
```

`doc show` ships **six** projections where `ENVELOPE_ARMS` declares four, and the array falsifies
`ArmShape::ArrayOf`'s own doc-comment (*"`jigc task list` is the surface's one array"*) **by name**. The
binary is right and the registry is the false home.

**DEFECT C — `ArmOutcome::Success` claims exit 0 at three rows that ship non-zero by design.** `validate |
StoreSweep` → exit 1 on any `STORE_EXIT_FLIPS` member; `migrate-corpus | Report` → exit 1 on the same;
`task validate | Report` → exit 3 on a blocking preview. Each exit is itself declared by the exit-code
taxonomy in the same contract doc, so two pinned statements about one surface disagree; `format_json_
success_axis.rs` proof 4 cannot see it because each recipe drives one clean cell.

Two Codex claims were driven and **CONFIRMED**: *the four pre-pin deletes are gone from the wire* (driven at
its hardest cell, the exit-4 migration review hold: stdout keys `['retires','rewrites','source','task']`,
`review` absent) and *the only explicitly unpinned success shapes are `describe` and the milestone text
acknowledgements, each with a reason* (`describe` still carries `schema_version` on the wire).

### Axis 6 · composed surfaces — [axis-6.md](axis-6.md) §3, ledger B1–B2

**A6-1 — a sub-task's composed surface is byte-identical inside and outside its worktree, and the milestone
boundary then drops its staged code at exit 0.** Increment 9's *Proves* line says the sub-task `resume:`
line *"says which of its three states the repo is in"*; driven, the surface separates the pre-provision
refusal from the other two and stops — and states (b) and (c) are the two whose consequences differ. The
data-loss half was driven end to end by the reconciler: work done in the shared checkout, invited by the
composed body, is not what the boundary commits.

**A6-2 — `jigc start --workflow milestone-execution` composes three unrunnable `Run:` lines at exit 0, and
jigc's own preview refusal routes there.**

```
$ jigc workflow milestone-execution --preview
  workflow 'milestone-execution' mints no task … run it directly: jigc start --workflow
  milestone-execution                                                                exit 1
$ jigc start --workflow milestone-execution                                          exit 0
  → three Run: lines that cannot run; the real door is `jigc milestone execute <id>`,
    stated at `describe` and nowhere at the door that composes it              (law 3)
```

**A6-3 — `implement-from-spec` over a corpus with no committed spec claims a list that is not there, then
names an unanswerable step.** jigc's own `doc list spec` ships the sentence the composed surface needs
(*"no committed `spec` docs"* plus the `--task` note); the composed step asserts the list instead (law 1),
and its next step cannot be answered (law 2).

Six Codex claims were driven and **CONFIRMED** — the shared `render::composed` seam over its four producers
(8 render sites / 4 leaves); the three expressible `resume:` states; orientation's three states **including
the `findings: unknown — <reason>` arm the driver did not reach**; the registry rows for `describe`/`doc
show`/`doc list`/`task validate`; every executable `jigc …` form in both step trees resolving to a real
clap leaf and named flags (24 distinct leaf forms); and the catalog/narration front-matter fences, driven
to **both** causes firing on a manufactured pack.

### Axis 7 · freeze & migration — [axis-7.md](axis-7.md) §4, §8.1–8.2

**C-1 (origin Codex, driven, subject corrected) — `ingest` and `unmanage` disagree on a placement
document's identity, and an edge originating from the hidden identity survives the unmanage.** Codex's
literal repro (`CHANGELOG.md`) is **unobservable** — `changelog` declares no `ref` field, so the adopted doc
contributes zero edges (falsifying datum: exactly five `type: ref` fields across both packs). The only
placement doctype carrying a `ref` is `vision`, and there the class fires exactly as claimed (driven on the
`refs-post-hoc` rig).

**C-2 (origin Codex, driven) — the territory bound misses a root-placement orphan.**

```
$ printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml ; git commit
$ jigc validate   → exit 1 · 2× schema-conformance.orphaned-instance (docs/…)
  VISION.md — a stamped, formerly-managed doc at a ROOT placement home:
  validate 0 · doc list 0 · ingest 0 · migrate-corpus 0 mentions; the file is still there, still stamped
```

Already on the record as declared residual 2 (`orphan.rs:293-300`, `:346`) — not a new defect, but the
amplifier that makes D-2's placement arm a **total** false green.

**D-1 — `jigc ingest` durably absorbs an out-of-band edit and surfaces nothing.** A `blocking (gates at
finalize) · file-state.hash-matches` finding, whose route says *re-author it through the owning workflow*,
is cleared by an `ingest` that prints the ordinary first-adoption line and emits `"finding": null,
"adopted": true`; a later `validate` is green. `design/reconciliation.md`:186 — ***"No silent discard, ever.
Every block surfaces; every absorb surfaces."*** No `external edit absorbed:` line exists on either surface.
No data loss (the bytes are the user's); a blocking finding is retired by a door that never says it did.

**D-2 — a `Relocated`-only bump is invisible to `migrate-corpus`, and the refusal that routes there is a
dead end.** Driven on **both** home kinds.

```
placement arm: changelog v2→v3, the ONLY edit placement.file CHANGELOG.md → HISTORY.md, hash re-pinned
$ jigc migrate-corpus   → exit 0, all five arrays empty, commit: null
$ jigc validate         → exit 0, "no findings — the committed store validates clean"
$ jigc doc list         → "no committed docs"      ← while the below-version managed doc sits on disk
location arm: adr v2→v3, decisions/ → adrs/ ; migrate-corpus silent at exit 0; validate exit 1 with the
  wrong diagnosis (D-4); doc show exit 1 store.not-found
control: the identical bump WITH a content change migrates and commits at exit 0
```

The corpus walk is documented to key on the **`from`** home; driven, it keys on `to`. `jigc relocate`'s own
refusal routes at `migrate-corpus`, which changes nothing — the M46 PT-1 class reopened at the relocation
door.

**D-3 — `ValueRemapped` on an `id-from` enum falls through to `migrate-corpus.prose-needed`, whose route is
a dead end.** The route says *author the new required prose … then re-run*; there is no new prose in the
schema change, and the re-run reproduces the identical block at exit 1 while `validate` routes back to
`migrate-corpus`. A **plain** enum field takes the correct map-gap arm (driven) — the complete-fix lens, one
cell over. Bytes untouched.

**D-4 — the orphan finding asserts *"no schema in the composed set says what this file is"* about a doc
whose schema is in the composed set, and `ingest` contradicts it at the same commit.**

```
$ jigc doc schema adr    → exit 0 · doctype: adr (schema-version 3)
$ jigc ingest            → ingest.wrong-location — conformant `adr` at docs/decisions/use-sqlite.md
                           sits outside `docs/adrs/` — relocate to adopt      (a followable route)
$ jigc validate          → exit 1 · "no schema in the composed set says what this file is"
```

The producer computes *no resolved doctype claims this path* and the message upgrades that to *no schema
says what this file is* — the same class M46 Increment 3 closed for `migrate-corpus` vs the discriminator.

### Axis 8 · adopter docs & help — [axis-8.md](axis-8.md) §2, ledger A–B

**CX-1 (origin Codex, driven) — `migrate-corpus --help`'s *"`--dry-run` writes nothing at all"* is falsified
by the invocation log.**

```
$ jigc config set invocation-log true ; ls .jigc/logs/invocations.jsonl   → no such file
$ jigc migrate-corpus --dry-run                                          → exit 0
$ wc -c < .jigc/logs/invocations.jsonl → 194 ; tail -1 → {"argv":["migrate-corpus","--dry-run"], …}
bounds driven in the same run: git status byte-identical · HEAD unmoved · the path is gitignored
```

**CX-2 (origin Codex, driven) — the installed guide's *"every `jigc setup` rewrites it"* is false for a
user-modified guide.** With the user's own `SKILL.md` **committed** first (so the dirty-install gate cannot
be what fires), `jigc setup` exits 0, the ack carries **no** guide row, an `adapter-guide.user-modified`
advisory fires, and the file is untouched — while the sentence ships into every adopter repo.

**CX-3 (origin Codex, driven) — the installed guide's one install/upgrade command cannot run from the repo
the guide is installed into.** `cargo install --path crates/cli`, run from an adopter repo after a real
`jigc setup`, fails at exit 101 (`crates/` does not exist there). Complementary to the driver's row 20,
which drove the same line from the jigc source tree at exit 0 — the only place it works.

**D-1 — the two surfaces F-9 named are the two that stay silent in the DEFAULT format, and three records say
otherwise.** `commit-recording.stale-title` reaches five surfaces; the two F-9 named by name (`doc rename
--task`'s ack and `task finalize --dry-run`) are exactly the two that are silent in the default `agent`
format and carry it only under `--format json`. Neither fence sees it: the previewed-tier equal-set suite
excludes the code deliberately, and M48's text/JSON parity fence runs **text → envelope**, the direction
that cannot see a value the envelope prints and the text withholds. The class is wider than F-9 — the text
manifest arm drops **every** advisory.

**D-2 (LOW) — `jigc task discard --help` never says it commits.**

```
$ jigc task discard --help
Abandon the task — remove its working area `.jigc/tasks/<id>/`
$ jigc task discard do-the-thing        (a milestone sub-task)
discarded task do-the-thing / record commit: 57d4285          exit 0, HEAD moved ef83d05 → 57d4285
```

`task discard` is `COMMITTING_DOORS` row 10; both shipped guides were corrected in this wave to say so, and
the sibling door's help does. Honest bound: the sentence is an **omission**, not a false predicate.

## B · CONFIRMED source-pass claims driven and held

These are the Codex passes' positive completeness claims, entered as leads and **driven**. None is a defect;
each is a registry or seam property that now has a drive behind it rather than a read.

| axis | claim (abbreviated) | how it was driven |
|---|---|---|
| 1 | C2 — `ARG_TOKENS` classifies every path-bearing argument id; the one documented limitation is a deliberate `PlainValue::Other` | the classification driven at each member |
| 1 | C3 — the clap-tree fences are bidirectional | every occurrence's argv parsed to its exact leaf + argument (ceiling stated) |
| 1 | C4 — `DOCTYPE_DOORS` includes all bare-doctype and address-bearing leaves, incl. `milestone add-from-spec`'s `spec_addr` | driven at all 10 |
| 1 | C5 — work-unit ids and `--slug` overrides project from one total classification, not independent name lists | driven at both families |
| 1 | C6 — the registry distinguishes occurrences and conditional arms (stdin sentinel · root vs non-root config values · closed address vocabularies · deliberate unrestricted handoff reads) | all four driven |
| 1 | C7 — migration-source admission re-adjudicates pathspec magic, confinement, `.git`, `.jigc`, symlinked components | 8/8 driven |
| 1 | C8 — the persisted `source-path` has one raw reader; consumers get `recorded()`/`normalized()` only | structure + a driven consequence |
| 1 | C9 — the destructive sink re-adjudicates into `ValidatedRetirement` immediately before `remove_file` | driven on its safety claim; one rider recorded (O5) |
| 1 | C10 (operand half) — printed route operands pass through `shell_token`/`shell_operand` | driven with a shell-unsafe filename; `PWNED.md` never created |
| 2 | 12 consistency statements (`BEHALF_DOORS` totality · the 10 `COMMITTING_DOORS` · the hook-capable re-probe · `setup`'s `--no-verify` re-probe + unborn exemption · the sole `git mv` re-probe · no production `git switch` · no `neither` door commits or moves · `DedicatedWorktree` unforgeable · the unborn exemption does not leak · `GIT_DIR` declared out) | 5 re-driven, 7 consistent by read, **none contradicted**; the 13th is OPEN, below |
| 4 | C1–C4 | **cross-reference, not an extra row** — each was driven to a defect and is filed in §A above; C1–C4 all originate with Codex |
| 5 | the four pre-pin deletes are gone from the wire | driven at the exit-4 review hold |
| 5 | the unpinned set is `describe` + the milestone text acks, each with a reason | driven at `describe` |
| 6 | C1–C6 | driven — see the axis-6 entries above |
| 7 | 4 consistency statements (project-layer shadows hashed with the origin pack's manifest · the `SchemaChangeKind × LOCI` disposition table · `ManifestKind::ALL`'s exhaustive partitions · the store sweep's stale/ahead/unadopted partition) | consistent; two carry a **silence** noted against D-1/D-4 |
| 8 | the manifest tag vocabulary and triage keys carry no omitted typed list | consistent with the driver's rows 10–12 and 31 |

## C · REFUTED claims (5) — each with its falsifying datum

| axis | claim | datum |
|---|---|---|
| 1 | **C1, the Codex headline**: *"No grounded completeness defect found. I found neither an omitted door nor a caller-token path that bypasses the relevant adjudication seam."* | three independent repros: **A1-D1** (a token becomes two path components and a `COMMITTING_DOORS` member drops one of two authored payloads at exit 0), **A1-D2** (the workbench-root token taken at the third door), **A1-D3** (the row's declared argv cannot reach its arm). Membership is not adjudication. |
| 4 | the source pass's **clean read of the retire rollback path** | DEFECT 2's B5 drive: with the window's editor re-creating the path, the retired original is not put back and nothing says so |
| 5 | *"the registry contains all 47 leaves and 60 arms; the unusual closed shapes are explicitly represented … `doc show` has scalar and data-keyed slice arms, and all are driven"* | `doc show roadmap#milestones` and `changelog#releases` return **top-level arrays**; `…#criteria/limits-per-ip` returns an object matching no declared row — six projections where four are declared |
| 5 | *"stdout JSON renderers for `setup`/`uninstall` … each map to registry rows"* (refuted **on the reject path**; on stdout it holds) | DEFECT A: both doors serialize a bare `Finding` — eight keys, neither `error` nor `findings` — on stderr |
| 6 | **C0**: *"No grounded completeness defects found."* | A6-1, A6-2, A6-3, each re-driven by the reconciler. A6-2/A6-3 are true statements *missing* from a surface and A6-1 is an emptiness (a diff of one argv across two repo postures) — none is visible to a single source read |

## D · OPEN leads (7) — driven as far as the state allows, promoted by nothing

| axis | lead | why it is open, not a finding |
|---|---|---|
| 1 | C10's **fence half** — *all route constructors invoke the backticked-command-span fence in debug builds* | not drivable here: the reconciliation binary is the **release** rc.15, where those `debug_assert!`s do not exist; and even on a debug build the fence fires only on a violating route, which no CLI argv can construct from outside the process |
| 1 | **O5** — a tampered `.jigc/tasks/<id>/source-path` makes `--approve` silently retire nothing (`normalized()` resolves `../outside/victim.md` before the sink; `retire`'s already-absent arm swallows it) | premise-bound: the state needs a hand-edited gitignored workbench file, and **no caller token can produce it** (`jigc migrate` refuses every `../` spelling at the door, driven). Recorded for whichever axis owns workbench-state truth |
| 2 | the fan-out `git merge --ff-only` re-probe (`task.rs:5614-5619`) | **not driven at the seam by either pass**; the door above it is driven in every posture cell |
| 6 | C5 is a **manifest-scoped** guarantee — a manifest-less project-local pack can ship an off-catalog workflow that `describe` narrates with `router_hidden: null` and no reason (driven) | declared at `design/surface-contract.md`:127 and `introspection.md`:90 — a wave decision, not a hole |
| 6 | the **JSON arm of every composed door carries no task-state lines** (`resume:`, `what's-left:`, `task scope:`, `create-gates:`, `also open:`) | declared at the seam (`render::composed`'s `Format::Json` comment) — recorded because this axis's C2/C4/C5 cells are, in consequence, **text-arm-only** guarantees |
| 6 | the driver's own §4 leads, carried unchanged and re-read against the code: `migrate` mints silently (matches `write-commands.md`:185 while being a third work-starting door); `task bind`'s two refusals carry no code/route (bare `anyhow`, outside the route floor as written); `describe --commands --format json` carries no argv (declared non-contractual); the none-provisioned advisory silence is by design | none was driven further; each matches a stated record, so none is promoted |

*Count note, stated rather than smoothed:* axis 6's ledger lists **three** open entries, the third of which
carries the driver's four §4 leads. The handover reported *open 4* for that axis; the rows above are the
entries as the axis file writes them.

---

# HONEST BOUNDS — what each axis states it did **not** drive

Every axis carries its own *"what I did not drive, and why"* section; those sections are the bound, and they
are summarized — never softened — here.

**Axis 1** — **191 `(door, cell)` pairs**, in five groups: `relocate` × 9 non-control cells on a stock
corpus (unreachable — every shipped doctype is frozen, which *is* A1-D3; driven instead on a manifest-less
pack); `DOCTYPE_DOORS ▸ Address` and `SLUG_DOORS` × {untracked in-repo, stdin sentinel} (32 pairs, n/a by
shape — an address head and a `--slug` override are identities, never paths a caller points at); the 150
`WORK_UNIT_ID_DOORS` × six filesystem-shaped cells (one equivalence class under `engine::slug::is_slug`;
three members driven at all 25 doors); and `migrate-corpus`/`ingest`/`unmanage`'s relocation siblings and
`milestone provision`'s worktree paths, which key no caller token. Also un-driven: the `owned-location`
gate's presence and trackedness legs, and `config set docs-root -`'s downstream consequences.

**Axis 2** — 25 of the 35 `Neither` leaves × every posture cell (10 driven as controls; the class returns
`None` at one shared `refuse_on_posture` seam, ⇔-fenced); `SeamSubject::verify`'s identity-drift and
ref-drift legs (no CLI sequence opens that window); `milestone finalize`'s two commit models as distinct
rows (the guard is leaf-keyed); the `GIT_DIR` redirect at 11 of the 12 acting doors (declared out with a
reopening condition); and `.git/sequencer` — a **multi**-commit cherry-pick or revert sequence.

**Axis 3** — the `finalize × --force` axis (does not exist at that leaf); the registered-`File` cell (`git
worktree add` cannot register a non-directory, so the state is empty, not skipped); `provision` × staged
prose / untracked workbench × `--force`; `task discard` × the worktree/workbench shapes (no such cell at
that door); `OwnWorktree × Unreadable` (the two cannot co-exist); the record-commit **rejection** cells
(axis 4's); and `uninstall` × a failing workbench probe.

**Axis 4** — cell D in full (stage failure × concurrently edited) at all 13 doors — no deterministic
in-transaction racer exists before the stage; cell F at `milestone add-from-spec`, `milestone discard`,
`task discard` and `setup`; cell B at `migrate-corpus`'s relocation arm; **a genuine concurrent process**
rather than a hook; `.jigc/version` at any door but `task finalize`; and the `--format json` shape of every
cell where the axis is not about the envelope. Fixture honesty is stated: cells E1/E2 required rewriting
`.jigc/tasks/<id>/source-path` by hand — the only way to reach a guard that exists because that value can be
corrupted.

**Axis 5** — `ENVELOPE_ARMS` × `ManifestKind::ALL` and × `SchemaChangeKind::ALL` (axes 7 and 4's door sets);
`doc list`'s `orphaned` row reached through a **cheaper** fixture rather than a dropped pack; envelope rows
under a project-pack composition (key sets are pack-independent by construction; the one pack-sensitive row
is declared `Unpinned`); `hook_output`'s *value* across the producer axis (driven at one producer); the
`GIT_DIR` posture; **a concurrent / fan-out envelope** (`milestone join` driven single-process — M51's own
standing honest bound, unchanged by this review); and `task list`'s element keys on an empty list.

**Axis 6** — the full named-verb chain to a real outcome for **8 of 12** catalog workflows (the other four
were composed via `--preview` and every `jigc <token>` checked mechanically against `VERB_KINDS` — 18
distinct leaves, all real); 26 of 31 command hints and the 34 narration bodies at `describe`; state (a) at
`migrate` and `milestone execute` (n/a by construction); `doc list`'s `orphaned` state and `doc schema`'s
projection (axis 7); and every posture cell (axis 2).

**Axis 7** — **44 of the 54 `SchemaChangeKind × LOCI` cells** (10 driven, chosen to hit every *disposition*
at every reachable locus; each remaining cell needs its own manufactured pack + snapshot + re-pin, and the
table is compiler-fenced), with the un-driven kinds named one by one; all 6 `ManifestKind::ALL` cells,
because **no axis-7 door renders a manifest** (driven); `describe` × hash-move / project-shadow beyond the
pack-load block; `migrate-corpus --dry-run` on any cell; and a **second manifest-shipping constituent** (the
methodology pack's own manifest was not independently drifted).

**Axis 8** — the record's byte-untouchedness at `task discard` (driven the commit and the sha, not the
per-item diff); the `unfilled` array's non-empty arm and `hook_output`'s non-empty arm at `migrate-corpus`
(axes 7 and 4); the two-half fold actually landing at `milestone finalize` (help text only); `jigc setup
--force` over a dirty `.git/hooks/pre-commit` (a declared bound); `jigc upgrade` over a **drifting** recorded
config delta (driven only in its empty case); the Tier-4 record corrections (an increment whose own *Proves*
line is *"Nothing through the binary"*); and `cargo install` into the operator's real `~/.cargo/bin` — driven
into a `mktemp -d` root instead, because installing into the real one would replace the `1.0.0-rc.15` this
whole review is measured against.

**Two procedural bounds across the instrument.** (1) The driver files were written by agents that
deliberately did **not** read their axis's Codex pass; the reconciler read both. (2) Three axes ran a
**demotion pass** and re-drove the demoted rows: axis 3 demoted 14 rows for missing blocks and re-promoted
all 14 on re-drive with **no verdict change** (stated in its own §6 as the best case for the driver's
judgment and the worst for its table); axis 5 demoted two row *sets* as *repro under-specified* and sampled
them; axis 1 demoted none but re-drove samples of three grouped blocks (60/60, 12/12, 11 of 18 arms) because
those blocks carry argv without an `observed:` line.

---

# COVERAGE — all 47 `VERB_KINDS` leaves

**The count read from the code.** `crates/cli/src/cli.rs` → `VERB_KINDS` carries **47** leaf rows, matching
the acceptance design's number. (`VerbKind` itself is the classification enum beside it; the extraction
below counts the `(&[…], VerbKind::…)` rows only.)

**What the table reports.** §17's fence — *every leaf appears in ≥1 axis matrix* — is satisfied by **axis 5
alone**, whose `EnvelopeArm` sweep drives every leaf; §19 recorded that as *true but weak* and strengthened
it, so the column below reports **which axes reach each leaf**, and a leaf reached by axis 5 only is
`only-5(<reason>)` with the same reason obligation `uncovered(<reason>)` carries. **`uncovered`: none.**

| # | leaf | axes reaching it (driven rows) | note |
|---|---|---|---|
| 1 | `start` | 1, 2, 5, 6, 8 | |
| 2 | `workflow` | 1, 5, 6, 8 | |
| 3 | `setup` | 2, 4, 5, 6, 8 | |
| 4 | `uninstall` | 2, 3, 5, 8 | |
| 5 | `upgrade` | 5, 8 | |
| 6 | `ingest` | 2, 5, 6, 7 | |
| 7 | `migrate` | 1, 2, 4, 5, 6, 7, 8 | |
| 8 | `migrate-corpus` | 2, 4, 5, 7, 8 | |
| 9 | `unmanage` | 1, 2, 5, 7 | |
| 10 | `rename` | 1, 2, 4, 5, 8 | |
| 11 | `relocate` | 1, 2, 5, 7 | |
| 12 | `describe` | 5, 6, 7 | |
| 13 | `validate` | 1, 2, 5, 7, 8 | |
| 14 | `doc create` | 1, 5 | |
| 15 | `doc add-item` | 1, 5 | |
| 16 | `doc remove-item` | 1, 5 | |
| 17 | `doc retitle-item` | 1, 5 | |
| 18 | `doc rename` | 1, 2, 5, 8 | |
| 19 | `doc set-field` | 1, 5, 6 | |
| 20 | `doc set-slot` | 1, 5, 6 | |
| 21 | `doc author` | 1, 5, 6 | |
| 22 | `doc show` | 1, 5, 6, 7, 8 | |
| 23 | `doc schema` | 1, 5, 6, 7, 8 | |
| 24 | `doc list` | 1, 2, 5, 6, 7, 8 | |
| 25 | `task list` | 5, 6 | planned `only-5`; axis 6 drove it as the mint-count control |
| 26 | `task diff` | 1, 5 | |
| 27 | `task validate` | 1, 5, 6, 8 | |
| 28 | `task discard` | 1, 2, 3, 4, 5, 8 | |
| 29 | `task finalize` | 1, 2, 3, 4, 5, 6, 8 | |
| 30 | `task bind` | 1, 5, 6 | |
| 31 | `config set` | 1, 2, 5, 6, 8 | |
| 32 | `config insert-step` | 1, 5 | |
| 33 | `config replace-step` | 1, 5 | |
| 34 | `config remove-step` | 1, 5 | |
| 35 | `config fill` | 1, 5 | |
| 36 | `config fork` | 1, 5 | |
| 37 | `config get` | 1, 5 | planned `only-5`; axis 1 drove the cascade read-back for the invocation-log knob |
| 38 | `config list` | 5 | **`only-5`** — *takes no argument at all; no posture, no write, no destruction — its contract is its envelope* (the acceptance design's own reason, unchanged by the drive) |
| 39 | `milestone create` | 1, 2, 4, 5, 6, 8 | |
| 40 | `milestone add-task` | 1, 2, 4, 5, 6 | |
| 41 | `milestone add-from-spec` | 1, 2, 4, 5 | |
| 42 | `milestone list-tasks` | 1, 5 | |
| 43 | `milestone provision` | 1, 2, 3, 4, 5, 6, 8 | |
| 44 | `milestone execute` | 1, 5, 6, 8 | |
| 45 | `milestone join` | 1, 2, 4, 5, 6, 8 | |
| 46 | `milestone finalize` | 1, 2, 3, 4, 5, 6, 8 | |
| 47 | `milestone discard` | 1, 2, 3, 4, 5, 8 | |

**`uncovered(<reason>)`: EMPTY.** **`only-5`: one leaf** — `config list` — down from the three the design
planned.

Per-axis door counts, as each axis file states them: axis 1 **39** · axis 2 **22** (posture rows; two further
write doors were fixture construction only and are not counted) · axis 3 **6** · axis 4 **13** · axis 5
**47** · axis 6 **23** · axis 7 **10** (8 door-set members + 2 *evidence doors*, `doc show` and `relocate`,
which carry the falsifying data for D-2 and D-4) · axis 8 **23**.

## Differences against the acceptance design's planned table

**28 of 47 leaves differ, and every difference is an addition — no leaf lost a planned axis.** The
`only-5` set shrank from three (`task list`, `config get`, `config list`) to one (`config list`).

| leaf | planned | driven | difference |
|---|---|---|---|
| `start` | 1,5,6,8 | 1,2,5,6,8 | +2 |
| `workflow` | 1,5,6 | 1,5,6,8 | +8 |
| `setup` | 2,4,5,8 | 2,4,5,6,8 | +6 |
| `uninstall` | 3,5,8 | 2,3,5,8 | +2 |
| `ingest` | 5,7 | 2,5,6,7 | +2, +6 |
| `migrate` | 1,5,6,7,8 | 1,2,4,5,6,7,8 | +2, +4 |
| `unmanage` | 1,5,7 | 1,2,5,7 | +2 |
| `rename` | 1,2,4,5 | 1,2,4,5,8 | +8 |
| `relocate` | 1,2,5 | 1,2,5,7 | +7 |
| `validate` | 5,7,8 | 1,2,5,7,8 | +1, +2 |
| `doc rename` | 1,5,8 | 1,2,5,8 | +2 |
| `doc set-field` | 1,5 | 1,5,6 | +6 |
| `doc set-slot` | 1,5 | 1,5,6 | +6 |
| `doc author` | 1,5 | 1,5,6 | +6 |
| `doc show` | 1,5,6,8 | 1,5,6,7,8 | +7 |
| `doc schema` | 5,7 | 1,5,6,7,8 | +1, +6, +8 |
| `doc list` | 1,5,6,7 | 1,2,5,6,7,8 | +2, +8 |
| `task list` | 5 (`only-5`) | 5,6 | +6 — **leaves `only-5`** |
| `task validate` | 1,5,6 | 1,5,6,8 | +8 |
| `task finalize` | 1,2,4,5,8 | 1,2,3,4,5,6,8 | +3, +6 |
| `task bind` | 1,5 | 1,5,6 | +6 |
| `config set` | 1,2,5,8 | 1,2,5,6,8 | +6 |
| `config get` | 5 (`only-5`) | 1,5 | +1 — **leaves `only-5`** |
| `milestone create` | 2,4,5 | 1,2,4,5,6,8 | +1, +6, +8 |
| `milestone add-task` | 1,2,4,5 | 1,2,4,5,6 | +6 |
| `milestone provision` | 1,3,4,5,8 | 1,2,3,4,5,6,8 | +2, +6 |
| `milestone join` | 1,2,4,5,8 | 1,2,4,5,6,8 | +6 |
| `milestone finalize` | 1,2,3,4,5,8 | 1,2,3,4,5,6,8 | +6 |

The nineteen unchanged leaves are `upgrade`, `migrate-corpus`, `describe`, `doc create`, `doc add-item`,
`doc remove-item`, `doc retitle-item`, `task diff`, `task discard`, `config insert-step`, `config
replace-step`, `config remove-step`, `config fill`, `config fork`, `config list`, `milestone
add-from-spec`, `milestone list-tasks`, `milestone execute`, `milestone discard`.

**What the additions are, in one sentence each of the three kinds.** Axis 2 gained ten leaves it drove as
**negative controls** (the `Neither` class raising no `repo.*` code under a detached HEAD and under a live
merge) — controls the design did not plan but which are driven rows with recorded outcomes. Axis 6 reached
fifteen leaves *through* its composed surfaces (a composed step that names `jigc doc set-slot` was driven to
the write, not read off the text), which is the axis behaving as its brief intends. Axis 1 added four beyond
its door set (`validate`, `doc schema`, `config get`, `milestone create`) as setup-or-consequence drives with
recorded outcomes, and axis 7 added two **evidence doors**.

**Nothing in the differences weakens the fence**: no leaf that the design planned into an axis failed to be
reached there, and the only movement is leaves reached by *more* axes than planned.

---

# Files

| file | what it is |
|---|---|
| `README.md` | this record — the instrument, the findings, the bounds, the coverage |
| `axis-1.md` … `axis-8.md` | the eight **reconciled** axis files, verbatim (driver table + reconciliation ledger + doors covered), each headed with the binary and date |
| `codex/axis-1-source-pass.md` … `codex/axis-8-source-pass.md` | the eight **raw Codex source passes**, verbatim, so every lead the ledger drives or refutes is auditable against what was actually claimed |
