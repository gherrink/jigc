# M55 — the PARTIAL per-axis re-review (ten rows), on the published `jigc 1.0.0-rc.24`

**What this is.** The partial per-axis re-review that M54's **S18** and M55's **S16** owe before the
1.0.0 call — `design/findings-channel.md` §12 (*Around M55* → *The partial re-review axes (S16)*) and
`DECISIONS.md` → *2026-09-27 — the road to the 1.0.0 call*. It is written against the human's own gate,
`implementation/decisions-pending.md` → *The rc.17 fix pass (M53)* → *The exit rule*:

> *The 1.0.0 call is taken when a partial re-review of the fix pass's affected axes finds no tier-1 row.
> Tier-2 and tier-3 findings never block the call and are triaged into the ledger for 1.x. A finding
> inside a fix pass's own new code triggers another fix pass and a partial re-review, never a full wave.*
> *[Completed 2026-09-22, on the first time it fired: a tier-1 row the partial re-review finds
> **outside** the fix pass's own new code is fixed as a **post-review fix under the same milestone** —
> fix, an independent review of the diff standing in for the audit, the record, a new stamp, the
> affected axes re-driven — never a new milestone and never a wave.]*

**Ten rows, approved by the human; none added, dropped or merged.** The rows are *subjects of what
M54, M55 and the co-author trailer changed*, not the eight historical numbered axes. The instrument
([instrument/README.md](instrument/README.md)) carries the table; in short:

| row | subject | basis | baseline |
|---|---|---|---|
| 1 | setup · the hook · install · release | NEW | none — first drive |
| 2 | probe integrity · measurement / the invocation log | NEW | none — first drive |
| 3 | store exit codes / reconciliation | NEW | three rows + one lead of numbered axis 7 (rc.16), and **F21** |
| 4 | pack-load / manifest freeze · migration | numbered axis 7, scoped | rc.16 |
| 5 | finalize / transaction | numbered axis 4, scoped | rc.16 |
| 6 | write surface | NEW (nearest: axis 1) | none — first drive |
| 7 | pinned read contracts | numbered axis 5, scoped | rc.20 |
| 8 | composed surfaces | numbered axis 6, scoped | rc.19 |
| 9 | adopter docs & help | numbered axis 8, scoped | rc.16 |
| 10 | co-author trailer | NEW | none — first drive |

**A keying note that every reader needs.** The row files are named `row-N.md` here; inside them (and in
the instrument) the label `axisN` means **row N of this run, not numbered axis N**. A baseline row keeps
its key verbatim — `(7, A7-F3)`, `(5, DEFECT 1)` — where the first number *is* the historical numbered
axis. A new finding of this run is keyed **`(R<row>, <id>)`** — `(R4, F2)`, never `(4, F2)` — so that
nothing collides. **Numbered axes 1, 2 and 3 are not among the ten rows** (→ HONEST BOUNDS).

**The binary.** Every driven row in every file here ran on the installed **registry build**
`~/.local/bin/jigc` → **`jigc 1.0.0-rc.24`**, on **2026-10-03** (the reconciliations ran past midnight
into 2026-10-04). Each agent asserted `jigc --version` first. Row 1 read the binary's embedded source
paths: 18 strings under `registry/src/<index>/jigc-1.0.0-rc.24/` and 18 under
`…/jigc-engine-0.1.0-rc.2/`, none under a workspace `crates/` path — so it is the crate as crates.io
serves it, built by `cargo install`. It is the **release** posture: the debug-only fences do not exist in
it, so a fence violation shows as a bad emitted command, never a panic. The repository was read by symbol
at `bffa6667` (`work/rc24-gate`); `git diff --stat 91834b5e HEAD` touches only
`completions/trial-driver/`, so `crates/` and `dev/` are the tagged release's (`jigc-v1.0.0-rc.24` peels
to `91834b5e`). The Codex source passes ran on **Codex CLI `codex-cli 0.146.0`**, read-only, at the tag.
Fixtures came from `dev/jigc-rig <state> --binary ~/.local/bin/jigc` (two-step eval, stdout only, every
root from `mktemp -d`, no teardown); the fixtures a rig cannot express are named at their cells. Rig and
scratch paths are written `$REPO`, `$RIG`, `<tmp>` or `~`-relative throughout; an environment variable is
recorded as *set*, *unset* or *empty*, never by value.

**The registry deltas the range produced since the rc.20 record, read by symbol** (the instrument's
table, re-read by every reconciler; no reconciler found a count that differs from its registry):

| registry | rc.20 | rc.24 | what moved |
|---|---|---|---|
| `ManifestKind::ALL` | 6 | **7** | `left-staged` (M55, the doc-only arm's left-out kind) |
| `CommitModel` | 2 | **3** | `DocOnly` (M55) |
| the methodology manifest | 11 | **13** | `inconsistency`, `jigc-feedback`, both at schema-version 1 |
| shipped workflows | 35 | **39** | the two report and two triage workflows |
| the `suppressed.door` set | 14 (rc.19) | **15** | — |
| **unmoved** | | | `VERB_KINDS` / `BEHALF_DOORS` 48 / 48 · `ENVELOPE_ARMS` 66 · `COMMITTING_DOORS` 11 · `ERROR_CODE_REGISTRY` 12 · `TASK_AREA_FILES` 15 · `MINT_DOORS` 6 · `AMBUSH_CONTRACTS` 6 · `STORE_EXIT_FLIPS` 7 · `WORK_UNIT_ID_DOORS` 25 · `SLUG_DOORS` 6 · `DOCTYPE_DOORS` 16 · `PATH_ARG_OCCURRENCES` 14 · `ROLLBACK_POPULATIONS` 11 · `DESTROYING_DOORS` 6 · `doc schema` `contract-version` 7 |

**A row of a matrix** is `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted,
verdict}`. A row is **driven** iff its argv ran on that binary. **A verb is covered iff it is the door of
≥1 driven row.** A classification-only row — a leaf proven by a ⇔ fence or a source read rather than by
driving — confers **no** coverage.

---

## THE HEADLINE — the tier-1 count

**Read this headline with the section that follows it.** The count in the box is the reconcilers', as
assembled, and the assembled text below is left as written. **After assembly, each of the six rows was
independently and adversarially re-driven: five are upheld at tier 1 and `(R6, K-1)` is not**
(→ [After assembly: the adversarial re-drive of the tier-1 rows](#after-assembly-the-adversarial-re-drive-of-the-tier-1-rows)).

> # TIER-1 ROWS FOUND: **6**
>
> Tier 1, quoted from the charter: **exit-0 loss or repository harm through a committing, destroying or
> moving door.** (**Tier 2** = a posture or route dead end · **tier 3** = a surface says something the
> binary does not do.)
>
> **Six finding rows were graded tier 1 by the reconciler of their row.** The assembler re-read each
> one's repro block; the last two columns are that re-read, not a copy of the row's own claim.
>
> | # | key | door | does the block show **exit 0**? | does it show the **loss or harm**? | where it sits |
> |---|---|---|---|---|---|
> | 1 | **`(R1, F1)`** — on an unborn `HEAD`, `jigc setup` overwrites untracked user bytes at a whole-rewrite install path | `setup` (committing: `CommitsOnBehalf`) | **yes** — `jigc setup` → exit 0, no finding (repro A and repro B arm 2) | **yes, loss** — before-control finds the mark (`:1`, `:1`); after: worktree 0, `HEAD` 0, `git log --all -S` 0, `git fsck --unreachable` 0, every blob scanned: none | **outside the new code** — the unborn `??` exemption is the M51-era guard's (assembler's `git log -L` read: `15b919ca`, 2026-09-15). Repro B reaches it **through M54's S22 cell** (the failed-first-run re-arm does not hold on an unborn `HEAD` once the user unstages) |
> | 2 | **`(R3, F7)`** — with no file-state baseline, an uncommitted out-of-band edit made after a task's first write to that doc is overwritten by `task finalize` | `task finalize` (committing) | **yes** — `task validate` 0, `--dry-run` 0, `task finalize` 0 (three ways in) | **yes, loss** — the hand line: worktree 1 → 0; `HEAD` 0; every commit on every ref 0; files under `.jigc/` 0 (`command grep -rlF`); status clean | **outside the new code** — the `UNKNOWN → baseline-adopt` mechanism is dated M46 by the standing test pair's header (**read, not driven** on an older binary). Two of the three ways in pass through `jigc unmanage` (a 2026-06 verb; an operator act no emitted route asks for in this cell); the second-clone way needs no act at all |
> | 3 | **`(R6, D-1)`** — the fan-out join's collision suffix is minted onto an id already on disk and `milestone finalize` overwrites the doc there | `milestone join` → `milestone finalize` (committing) | **yes** — both exit 0, `findings` empty, `displaced: []` | **yes, loss** — a committed finding's content replaced (marker 1 → 0 in the worktree; history only) **and** an untracked conformant file's content replaced (marker 1 → 0; `git log --all -S` 0; nowhere under `$REPO`, `.jigc/` included) | **outside the new code** — reproduces under `park-idea` sub-tasks (no `new: true`), so it predates M55. M55's *one doc per finding, several reporters* is the workload that reaches it |
> | 4 | **`(R6, D-7)`** — a dangling symlink at the minted home: the create-gate passes it and `task finalize` commits the symlink while the finding lands untracked | `doc create` → `task finalize` (committing) | **yes** — `task finalize` → exit 0, *promoted … 1 file committed* | **no loss; harm only** — no pre-existing byte is gone (the prose sits in an untracked file). What is shown: the commit holds a `120000` entry in place of the finding, `git log --all -S` finds the prose in **no** commit, and in a clone `jigc doc list inconsistency` exits 1 for the whole doctype where the parent commit's exits 0 | **outside the new code** — the precondition is a planted link and symlinked paths are numbered axis 1's class; M55's gate is the door that passes it. Not driven under an entry without the key |
> | 5 | **`(R6, K-1)`** — the create-only gate is check-then-use: a home that becomes occupied inside the window is copied in at exit 0 under `new: true` | `doc create` / `doc author` → `task finalize` (committing) | **yes, in one of two variants** — a file landing in the window from outside jigc: `doc create` 0 (`existed: true`), `task finalize` 0 | **yes, loss, in that variant** — the victim's marker 1 → 0, in no commit, nowhere under `$REPO`. **In the source pass's own scenario (two jigc reporters) the loss half is missing**: `task finalize` → exit 3 `finalize.base-mismatch`, the landed finding's checksum unchanged — tier 2 there | **inside M55's code** — the `new: true` gate is M55's. The window was won against a synthetic file toggler (19 / 60 creates) and once in 160 creates against a real second reporter |
> | 6 | **`(R9, F5)`** — `jigc uninstall`, without `--force`, destroys files inside `.jigc/logs/`, `.jigc/state/` and `.jigc/index/`, named by nothing | `uninstall` (a `DESTROYING_DOORS` member) | **yes** — exit 0, *removed .jigc/*, no `--force` | **yes, loss** — three planted files found before (`command grep -rl`), nowhere after, stash empty; and, without a plant, the opt-in invocation log: 3 records before, 1 after (the uninstall's own) | **outside the new code** — the workbench guard is 2026-09-05's (assembler's `git log -S` read); M54, M55 and the trailer did not touch it |
>
> **So: 0 inside M54's code · 1 inside M55's code (in one of its two variants) · 0 inside the co-author
> trailer · 5 outside the new code.**

**What the six is made of — stated so the count cannot be read as six equal rows.**

- **Four show both halves on the loss disjunct with no qualification on the showing:** `(R1, F1)`,
  `(R3, F7)`, `(R6, D-1)`, `(R9, F5)`. Each carries a before-control that finds the bytes and an
  after-control that finds them nowhere, at exit 0, through a door the registries class as committing or
  destroying.
- **One shows both halves in one of two variants, and only under a synthetic racer:** `(R6, K-1)`. Its
  reconciler tiers it *split* — tier 1 *"by the predicate's letter"* for a non-jigc writer, tier 2 for
  two jigc reporters.
- **One is tier 1 on the *harm* disjunct alone, and the two Opus agents disagree about it:**
  `(R6, D-7)`. The driver proposed tier 3 (*"no pre-existing byte is lost or damaged"*); the reconciler
  re-tiered it (*"the driver's tier 3 does not survive the predicate — tier 1 on the harm disjunct"*)
  and stated its weight in the same sentence. The loss half is **not** shown.
- **One reconciler handed its ruling to the human in so many words:** `(R9, F5)` — *"the ruling between
  tier 1 and tier 3 is the human's; nothing found by either pass makes it"*. The tier-3 reading is that
  `logs/`, `state/` and `index/` are jigc-only populations outside the guard's designed subject, leaving
  a help universal the binary does not honour.
- **Read strictly — both halves, the loss disjunct, no variant — the count is 4. Read as the
  reconcilers graded it, it is 6.** This record reports 6 and says which is which; it does not choose.

**Every one of the six carries a stated caveat about its precondition**, and they travel with the
finding (→ FINDINGS → Tier 1): `(R1, F1)` needs a repository with zero commits holding human bytes at
`.jigc/AGENT.md` or `.jigc/config/packs.yaml` (or jigc's own failed first run, an unstage and an edit),
and `design/validation.md` states the unborn `??` exclusion as a door rule; `(R3, F7)`'s lost bytes are
an uncommitted edit made outside jigc into a doc a task holds a staged copy of, and
`design/storage.md` declares a *neighbouring* cost (a silent merge) for the same lost baseline;
`(R6, D-1)` needs `<slug>-2` … `<slug>-N` occupied when N sub-tasks mint one slug; `(R6, D-7)` and
`(R6, K-1)` need a planted link or a sub-millisecond race; `(R9, F5)`'s non-planted member is a log the
adopter opted into.

**Four rows that are *not* tier 1 and that a reader hunting for the line should still see**, because
their reconcilers say how close they are:

| key | graded | why it is named here |
|---|---|---|
| `(R5, F1)` | tier 2 | a doc-only finalize exits 0 and leaves the index holding a staged reversion nobody staged. Its reconciler: *"if an index entry nobody staged, left at exit 0 by a committing door, is itself repository harm, this finding is tier 1 on the evidence already here."* Inside M55's code |
| `(R4, F5)` | tier 3 | `migrate-corpus` commits a human's uncommitted edit on a migrated path under jigc's own message at exit 0. *"Closest to tier 1, and still not it"* — both plants are in `HEAD` and on disk afterwards |
| the M55 declared bound, pressed (`(R8, F-4)`) | tier 3 | bytes an agent authored on a route a **blocking** finding printed are in no commit and nowhere under `.jigc/` after an exit-0 `milestone finalize` — *"the loss half is the knob's declared meaning"* (`squash: true`) |
| `(R4, C1)` | tier 3 | a filesystem pack's freeze manifest that vanishes between two reads is skipped without a word and `migrate-corpus` commits; the commit's whole diff is the stamp |

**What held.** Rows **2, 4, 5, 7, 8 and 10 found no tier-1 row**, each with its own both-directions
test on the record. The co-author trailer (row 10) was driven at all **12 doors × {set · unset ·
empty}** — one trailer under *set*, none under the other two (the tree compared at `task finalize`:
identical in all three arms), the pinned `--format json` envelopes byte-equal between arms over twelve
pairs — and its seven findings are all tier 3. The
doc-only commit (row 5) commits no unrelated path in any driven cell and leaves the index byte-identical
on the un-hooked path; the Codex pass's one tier-1 claim on row 2 (a rejected agent commit permanently
signs its message file) was driven through 13 commit constructions and **refuted at all 13**. **The one
tier-1 row inside any of the ten rows' baselines, `(5, DEFECT 1)`, stays CLOSED** at both mint doors.

**The exit rule, read against this.** Its first clause (*no tier-1 row*) is **not satisfied**. Its
location clauses then say two different things about the six: the five outside the new code are the
*post-review fix under the same milestone* case; the one inside M55's code is the *another fix pass and
a partial re-review* case. **That is stated here, not resolved: the adjudication — including whether
the count is 4, 5 or 6 — is the human's, and this record does not pre-empt it.**

## After assembly: the adversarial re-drive of the tier-1 rows

Made on **2026-10-04**, after everything above was assembled, on the same installed registry build
(`jigc 1.0.0-rc.24`, asserted first in every cell). **One agent per row, six agents**, none of which
drove or reconciled the row it was given; each was told to refute it — re-drive the repro on a fresh
rig, look for the bytes in every place they could survive, find where the guard holds, read whether
the behaviour is declared, and argue the tier both ways. Each file is in
[tier1-verification/](tier1-verification/), verbatim.

| key | door | the reconciler's tier | the adversarial verifier's tier | reach, in the verifier's words | file |
|---|---|---|---|---|---|
| `(R3, F7)` | `task finalize` | 1 | **1 — upheld, both halves** | a standing precondition, not an exceptional one: any fresh clone, CI checkout or container session before its first landed finalize. *"Low-to-moderate in ephemeral-clone agent setups, low on a long-lived developer checkout"* | [R3-F7.md](tier1-verification/R3-F7.md) |
| `(R6, D-1)` | `milestone join` → `milestone finalize` | 1 | **1 — upheld, both halves**, and wider than recorded | *"low"* — two sub-tasks minting one slug while `<slug>-2` is already a file; or, with no collision at all, an untracked file appearing at a sub-task doc's home before the finalize | [R6-D-1.md](tier1-verification/R6-D-1.md) |
| `(R1, F1)` | `setup` | 1 | **1 — upheld**, *"a low-reach tier 1 with a small blast radius"* | *"low"* — no route an adopter following QUICKSTART reaches without a deliberate extra act | [R1-F1.md](tier1-verification/R1-F1.md) |
| `(R9, F5)` | `uninstall` | 1, the ruling handed to the human | **1 — upheld**, *"at the low end of tier-1 severity"* | the invocation log: certain once the knob is on and `uninstall` runs, *"low to moderate"* in absolute terms; a planted file: *"low"*; a human-only door | [R9-F5.md](tier1-verification/R9-F5.md) |
| `(R6, D-7)` | `doc create` → `task finalize` | 1 on the harm disjunct (the driver proposed 3) | **1 on the harm disjunct only** — *"the weakest tier-1 row"* | *"very low"* for an ordinary adopter — a planted or inherited symlink at exactly the minted home | [R6-D-7.md](tier1-verification/R6-D-7.md) |
| `(R6, K-1)` | `doc create` / `doc author` | split: 1 (a non-jigc writer) · 2 (two jigc reporters) | **not tier 1 as a row of its own — tier 3, with a tier-2 tail** | *"not reachable in ordinary use; reachable on purpose in one or two tries"* — a window measured at ≈1 ms | [R6-K-1.md](tier1-verification/R6-K-1.md) |

**Five of six are upheld.** `(R6, K-1)` is demoted because its loss half is not the gate's: the bytes
die through the pre-M55 rule that a file *on disk* at a doc home is read as *committed* (copied in,
which switches the clobber guard off), and the verifier reached the same exit-0 loss with no race at
all under an entry without `new: true`, and through the committing door's own check-then-use window.
What K-1 itself is — three surfaces stating a universal the binary honours only sequentially — is
tier 3, and where it parks a task, tier 2. It points at a loss class (*an untracked file at a doc home
is read as committed; no door is atomic against a non-jigc writer*) that its verifier says should be
ruled on **once, as a class**: if a genuine concurrent non-jigc writer is inside the predicate, the
tier-1 row is that class at `task finalize`, and K-1 is its narrowest member.

**Every upheld row is outside the new code** — none inside M54's code, M55's code or the co-author
trailer. The one row the assembled headline placed inside M55's code is K-1, whose gate is M55's, and
K-1 is the one demoted; `(R6, D-7)` reproduces on `adr` under an entry without `new: true`, so its
mechanism is not M55's gate either. Two of the five carry their verifier's own weight against them and
are the contested ones: `(R6, D-7)` (*"not the one that should cost a release cycle on its own — but it
is cheap to fold into a fix pass that other rows already trigger"*) and `(R9, F5)` (the invocation-log
member is *"the member a human ruling could most defensibly move"*). With the blind trial's one
verified row (`L-22`, in [the trial record](../../RC-rc24/README.md)), the gate's evidence carries
**six verified tier-1 rows**: five here, one there.

**The re-drive widened every upheld row past its entry below**, and each entry now carries one line
saying so: `(R1, F1)` is four install paths and its trigger is an unborn `HEAD`, not only a zero-commit
repository — the design declares the exemption, not the loss; `(R6, D-1)`'s milestone planner runs no
clobber guard at all, suffix or not; `(R9, F5)` has two more members; `(R3, F7)` reaches
`milestone finalize` and a location doctype too; `(R6, D-7)` reproduces without M55's gate and has two
worse members. The verifiers' *cheap vs robust* forks and the contract each fix must restore are in the
files; **no fix was written and nothing in the repository was edited**.

**The ruling is the human's and has not been taken.** Which of the contested rows count, whether the
class K-1 points at is inside the predicate, which clause of the exit rule the upheld rows fall under,
and the scope of the fix pass are all the human's to decide. This section reports what the re-drive
found; it decides none of them, and the 1.0.0 call is not taken here.

### Leads the re-drive raised and did not grade

Four things the verifiers drove or noted outside the row each was given, collected here because no
row of this record carries them. **They are leads — for the fix pass's class derivation and for
triage — not graded rows, and nothing here rules on them**: none is in the count box, and none has a
tier from this record.

- **[tier1-verification/R3-F7.md](tier1-verification/R3-F7.md)** (§3 last row; §7, class 5) — the sibling loss **with the baseline present**: a task driven from inside a user-made `git worktree add` of the rig finalizes at exit 0 with only `file-state.staged-copy` on the path (*"no `baseline-adopt`, no conflict row"*) and the hand edit is *"0 · 0 in the linked worktree, no blob anywhere"* — *"driven once, not bounded, not graded here"*, and, in the verifier's words, *"This should be triaged as its own lead."*
- **[tier1-verification/R6-K-1.md](tier1-verification/R6-K-1.md)** (§7, class 3) — `task finalize`'s **own** check-then-use window, between its clobber-guard probe at plan time and the promote copy: against an external writer planting a file at a random delay after the door was spawned, delay 0–160 ms, N=60 → 60 refused `promote-clobber`; delay 140–300 ms, N=80 → 26 refused · *"13 exit 0 with the planted file's bytes gone"* · 41 planted after — a window the verifier computes as 160 ms × 13/80 ≈ 26 ms, any workflow, pre-M55, and *"Not in the review record"*: it is in none of the findings below.
- **[tier1-verification/R6-D-1.md](tier1-verification/R6-D-1.md)** (§3, V9) — with a directory at the suffixed home, `milestone finalize` exits 1 on `blocking milestone-finalize.commit-rejected` (*"could not read … before promoting over it: Is a directory"*), nothing committed, and the verifier's side datum, *"not this row's"*, is that *"the message prints an absolute host path"*.
- **[tier1-verification-L-22.md](../../RC-rc24/tier1-verification-L-22.md)**, in [the trial record](../../RC-rc24/README.md) (§7, *`uninstall` driven*) — on a fresh rig with no milestone and one prunable foreign worktree, `jigc uninstall` exits 0 with the record gone and acks ``- pruned git's worktree registrations for the fan-out worktrees `.jigc/` held``, where *"`.jigc/` held no fan-out worktree; the only record dropped was foreign"*: the one door that narrates the prune attributes it to worktrees `.jigc/` did not hold (the verifier's passing word for the sentence is *tier-3*; no row here or there carries it).

---

## The staffing

Per row, three agents — M51 §19's shape, unchanged through M52 and the four M53 runs. **Thirty agents
over ten rows.**

- **One Opus driver** owns the `(door, cell)` table. It drives every row on the installed binary and
  records exit, code, route, surface and a repro per row. **It may not mark a row driven from a source
  read.** Each driver was given the row's `axisN-driver-scope.md` and told not to read the row's Codex
  pass.
- **One Codex source pass** ([codex/](codex/), verbatim) owns **completeness of the row set** — *is
  there a door the registry does not carry, or a bypass of the seam this row is about?* It reads; it
  drives nothing. All ten passes exited 0 and are non-empty. Each states its own bound (row 3's:
  *"no binary, tests, or reproductions were run"*).
- **One reconciler** (a separate Opus agent, which did **not** author the driver file it reconciles)
  merges the two: it re-drives every driver defect, enters every Codex claim as a lead and drives it,
  re-reads the driver's door-set counts against the registries, and audits the driver's table for rows
  marked driven that carry **no repro block** (the **demotion pass**).

**How the Codex source passes were run.** Codex CLI `codex-cli 0.146.0`, model `gpt-5.6-sol`, at the
CLI's default reasoning effort (`none`), each invoked as
`codex exec -s read-only -o <out> - < axisN-prompt.md`.

**A second, higher-effort Codex pass was launched for every row and none produced a report** — it was
attempted at **high** reasoning effort for all ten rows, and all ten ended on the account's usage limit
(exit code 1, no output file), so it contributed nothing. Nothing in this record rests on them, and
whatever they would have claimed is unreviewed (→ HONEST BOUNDS).

**After assembly, six further agents** — one independent adversarial verifier per tier-1 row, none of
which drove or reconciled its row — re-drove the six rows on 2026-10-04
(→ [After assembly](#after-assembly-the-adversarial-re-drive-of-the-tier-1-rows);
[tier1-verification/](tier1-verification/)). They are outside the thirty.

## The reconciliation rule

> *A claim by one that the other cannot reproduce is a **lead, not a finding**.*

A Codex claim with no driven repro enters the table as `lead(codex, <claim>)` and is either **driven to a
repro block** — at which point it is a finding — or **recorded REFUTED with its falsifying datum**. An
Opus row the source pass says cannot happen **stays a finding** (it was driven), and the source claim is
recorded refuted with the datum. **Silence is not refutation.** A row a reconciler cannot drive at all
stays an **OPEN LEAD** with the reason.

It earned its keep on this run in five places worth reading:

- **Row 3 — the reconciler found the row's tier-1 finding in the driver's own NOT DRIVEN list.** The
  driver listed `unmanage` × a managed doc a live task holds a staged copy of as *"the nearest thing to a
  tier-1 shape on this surface … a stated bound, not a finding of this run"* and wrote *NO TIER-1 ROW*.
  Driven, it is `(R3, F7)`; it does not even need `unmanage`; and the stated bound describes the
  neighbouring order. The same reconciler re-tiered `(R3, F4)` from 3 to 2 by following its route late.
- **Row 6 — a Codex claim became a finding, and its tier was decided by a drive.** The source pass
  proposed the create-gate's check/use race as *"potentially tier 1"*. In its own scenario the
  committing door **refuses**; in the variant with a non-jigc writer both halves show. `(R6, K-1)`.
- **Row 2 — the only Codex claim proposed at tier 1 with high confidence was refuted.** 13 of 13 human
  retries after a hook-rejected agent commit landed with an empty trailer block.
- **Rows 1 and 8 — a source read of *closed* and of *still open* each failed against a drive.** Row 1's
  pass reported zero defects; five findings stand, two of them reached *by driving its positive claims*
  (`(R1, F4)` from *"quoted for spaces"*, `(R1, F5)` from *"honors `core.hooksPath` and worktrees"*).
  Row 8's pass called `(6, D-1)` still open; orientation carries the posture on rc.24. Row 9's pass
  called `(8, N-1)` closed; `--force` deletes a parked file with *all three guards* clean.
- **The demotion pass moved rows on seven of ten rows and no finding on any.** Row 4: 54 of 419 rows
  carried no block, 53 re-driven and matching, one (5.7) not driven. Row 6: 92 of 224 rows carried no
  re-runnable block, 91 re-driven; one row and four sub-argv stay demoted, and **three `code` cells were
  wrong** (an exit-0 ack recorded beside another cell's blocking code). Row 9: 21 rows with no block and
  8 partial — seven leaves would have lost their only block — all but three arms re-driven. Row 2: 17,
  all re-driven. Row 5: 3 demoted and not re-driven, 27 given a reconciler block. Row 8: 7, all
  re-driven. Rows 3 and 7: 2 and 1. **Rows 1 and 10: none.**

---

## Roll-up

| row | subject | hand-off tally: confirmed / refuted / open | new finding rows (tier) | driver rows · demotions | leaves covered |
|---|---|---|---|---|---|
| 1 | setup · the hook · install · release | 5 / 6 / 8 | **5 — 1 tier-1 · 4 tier-3** | 132 · 0 | 2 (+2 as lead rows) |
| 2 | probe integrity · the invocation log | 4 / 1 / 10 | **4 — all tier-3** | 174 · 17, all re-driven | 15 (+1) |
| 3 | store exit codes / reconciliation | 14 / 5 / 9 | **7 — 1 tier-1 · 1 tier-2 · 5 tier-3** | 138 · 2 | 18 |
| 4 | pack-load / freeze · migration | 6 / 4 / 10 | **6 — 2 tier-2 · 4 tier-3** | 419 · 54 (53 re-driven) | 22 |
| 5 | finalize / transaction | 12 / 2 / 5 | **10 — 2 tier-2 · 8 tier-3** | 105 · 3 | 15 |
| 6 | write surface | 11 / 3 / 6 | **11 — 3 tier-1 · 2 tier-2 · 6 tier-3** | 224 · 92 without a block (91 re-driven) | 27 |
| 7 | pinned read contracts | 4 / 0 / 10 | **4 — all tier-3** | 212 · 1 | 5 |
| 8 | composed surfaces | 8 / 2 / 9 | **5 — all tier-3** | 54 · 7, all re-driven | 26 |
| 9 | adopter docs & help | 8 / 3 / 7 | **6 — 1 tier-1 · 5 tier-3** | 76 · 21 + 8 partial | 37 |
| 10 | co-author trailer | 7 / 4 / 9 | **7 — all tier-3** | 237 · 0 | 29 |
| **total** | | **79 / 30 / 83** | **65 rows — 6 tier-1 · 7 tier-2 · 52 tier-3** | | **47 of 48 union** |

**The counting basis, stated rather than smoothed** (the rc.18 – rc.20 records' own precedent). The
*hand-off tally* column is the orchestration's per-row summary, and it does **not** count one thing: on
some rows *confirmed* is the number of new findings (rows 1, 2, 4, 6, 7, 10), on others it adds confirmed
Codex claims or confirmed still-open baseline rows (row 3's 14 against 7 findings; row 5's 12 against 10;
row 8's 8 against 5; row 9's 8 against 6). *Refuted* on row 6 is 3 while that row's ledger says
*"Refuted source claims: none"* — the three are driver `code` cells the reconciler corrected. **The
auditable set is the enumeration in §FINDINGS below, not this table, and where a number here disagrees
with a number in a row file, the row file governs, because it carries the repro.**

**65 finding rows are 62 distinct defects by the assembler's reading** — three were found twice, by two
rows independently: `(R4, F2)` = `(R6, D-8)` (the malformed-front-matter refusal names the key and not
the workflow — **graded tier 2 by row 4's reconciler and tier 3 by row 6's**, stated at the entry rather
than harmonised) · `(R9, F2)` = `(R10, F5)` (the composed amend step's *only the trailers you add*) ·
`(R5, F10)` ⊂ `(R9, F1)` (the commit boundary's help names two commit models). A fourth pair shares a
class without being one defect: `(R3, F6)` and `(R4, F1)` (a door mints its task before a compose-time
refusal).

---

# THE ROW-BY-ROW COMPARISON — every baseline row the ten rows re-drove

One layer per row that has a baseline, keys **verbatim** from the instrument README's table (*Baseline
rows each row re-drives*). **CLOSED** carries the argv that settles it; **STILL-OPEN** carries the datum.
A row the rc.16 wave or the M53 fix pass triaged into the 1.x ledger
(`implementation/decisions-pending.md` → *The rc.16 wave (M52)*: *"Tier 2 and tier 3 — 23 rows, triaged
to the ledger for 1.x, none blocking"*) is marked **STILL-OPEN(1.x, expected)** — a *measurement*, not a
finding. **Rows 1, 2, 6 and 10 have no baseline: they are first drives**, and each carried one declared
item, listed at the end.

**The one tier-1 baseline row inside any of the ten rows' baselines is `(5, DEFECT 1)`. It is CLOSED.**
The other three tier-1 rows the M52 record carries — `(3, A3-1)`, `(3, A3-2)`, `(2, DEFECT A)` — and
rc.17's `(2, DEFECT 1)` sit on numbered axes 2 and 3, which no row here drove (→ HONEST BOUNDS).

## Row 3 — store exit codes / reconciliation, against `completions/artifacts/M52/per-axis-review/README.md` (numbered axis 7) and M55's VERDICT

| baseline row | tier there | verdict on rc.24 | the argv / the datum |
|---|---|---|---|
| **`(7, A7-F1)`** — the `oob-rename` trailer claims a commit-introduced `git mv` over a deletion | 3 | **STILL-OPEN(1.x, expected)** — and wider by one cell | `rm CHANGELOG.md` (unstaged) → `jigc validate` exit 1, trailer *"out-of-band rename detected — a structural-identity change this commit introduced; … (revert the `git mv` or adopt it via `jigc rename`)"*; `HEAD` unchanged, status ` D CHANGELOG.md`, no rename in `HEAD`. Same trailer after a committed `git rm`, and — new on this run — over a move that landed in a *prior* commit. M55 narrowed the member to the blocking arm and left the trailer text. [row-3.md](row-3.md) P7 · R-1 |
| **`(7, A7-F2)`** — `ingest` files a stamped orphan that `validate` blocks on under *no action needed* | 3 | **STILL-OPEN(1.x, expected)** | one commit: `jigc validate --format json` exit 1, two `(schema-conformance.orphaned-instance, docs/…)` keys whose route opens *ask `jigc ingest`*; `jigc ingest` exit 0, *"fine to stay plain … no action needed"*, JSON rows `finding: null`. Re-observed on both M55 doctypes by row 4, not re-filed. [row-3.md](row-3.md) P6 · R-2 |
| **`(7, C-2)`** (declared residual) — a departed doctype's stamped root-placement instance is outside orphan territory | — | **STILL-OPEN (expected)** — a declared residual on a written trigger; not re-filed. **Its exit-0 half is now shown** | `VISION.md` still stamped; named by 0 lines at `validate` · `doc list` · `ingest` · `migrate-corpus --dry-run`. With the two `docs/` orphans retired: `jigc validate` → *no findings — the committed store validates clean*, exit 0, `report_only: true`, the stamped file tracked and unclaimed. [row-3.md](row-3.md) R-2 |
| §D lead — the **`probe-unreliable`** member no axis-7 door had driven | lead | **CLOSED** (discharged by a drive; no defect) | three failure shapes (a mode-644 override, `exit 3`, garbage output) → `jigc validate` exit 1, `(pack-probe-integrity.probe-failure, doc-code)` blocking, the member's own trailer, `report_only: false`, both arms; and through the hook. [row-3.md](row-3.md) P4 · R-11a |
| **F21** (`completions/artifacts/M55/VERDICT.md`) — store-scope `validate` reads the raw workflow definition and misses a project structural-op delta | declared open gap | **STILL-OPEN (expected)** — a measurement, and **bounded wider than M55 declared** | `jigc config insert-step` of a broken include, committed → `jigc validate --format json` exit 0, `findings: []`; `jigc task amend` → exit 1 `workflow-refs.include-resolves`; the same include in a whole-file shadow **is** reported. Missed the same way, each driven: an include **cycle**, a **command-ref** in isolation, a **schema-ref**, a marker-shadowing native step, and a **`replace-step`** delta. Not driven: `remove-step`, fan-out/join pairing, the front-matter read. [row-3.md](row-3.md) P20 · R-3 |

## Row 4 — pack-load / manifest freeze · migration, against `completions/artifacts/M52/per-axis-review/README.md` (numbered axis 7)

| baseline row | tier there | verdict on rc.24 | the argv / the datum |
|---|---|---|---|
| **`(7, A7-F3)`** — `doc show` refuses a relocated managed doc with a route that names none of the repair paths | 2 | **CLOSED — both arms** | **placement:** `jigc doc show changelog` → 1 `store.not-found`, route *"`changelog:changelog` is committed at `CHANGELOG.md`, a prior home of `changelog` — … run `jigc migrate-corpus` to land it at the home this read resolves, then read it again"*. **location:** `jigc doc show adr:use-sqlite` → the same, naming `docs/decisions/use-sqlite.md`. The route run verbatim → `2 migrated`; both reads then exit 0; each file's sha1 with the stamp put back equals its sha1 before. Also on `inconsistency` and on a `docs-root` knob strand. **Bound:** scoped to snapshot-declared prior homes — a doc moved by hand still gets the generic route (→ `(R4, F3)`). [row-4.md](row-4.md) R-7 · RC-A7 · RC-A7k |
| §D lead — **the methodology pack's own manifest** | lead | **NOW DRIVEN** (the embedded arm excepted) | project layer: 14 shape edits over the two new doctypes block, 6 prose edits load. Pack layer: a listed filesystem copy, reshaped without a re-pin, blocks naming the **methodology** manifest's declared hash. A reshaped **embedded** pack: not reachable without a rebuild. [row-4.md](row-4.md) R-3 · RC-C3 · RC-C7 |
| §D lead — **the un-driven `SchemaChangeKind × LOCI` cells** (40 of 54) | lead | **STILL-OPEN** | six cells driven on this run (`Relocated` location and placement · `EnumWidened` @1 · `RemovedField` @1 · `ProseNeeding` @2 · `AddedItemSlot` @2); no complement claimed. [row-4.md](row-4.md) RC-K |
| §D lead — **`cwd-unreadable`** | lead | **NOW DRIVEN** | seven argvs → exit 1, code-less, route-less, *"cannot determine the current directory: …"*, as the fence declares. [row-4.md](row-4.md) R-9 · RC-M |
| §D lead — **`pack-resource-missing`** | lead | **NOW DRIVEN**; two findings sit on it | `config/knobs.yaml` removed → ten doors exit 1 `pack.resource-missing` with a route that repairs; `(R4, F1)` at `migrate`, `(R4, C1)` on the manifest (the one resource whose absence is silent). [row-4.md](row-4.md) R-1c · RC-F1 |
| §D lead — **CX-8** (a third origin pack) | lead | **NOW DRIVEN** | a third manifest-owning pack, reshaped without a re-pin, blocks 8 of 8 pack-loading argvs naming its own doctype. [row-4.md](row-4.md) RC-C5 |
| §D lead — **CX-9** as behaviour | lead | **STILL-OPEN** | as the 54-cell lead |
| §D lead — **CX-13** | lead | **this row's intersection CONFIRMED; the rest not this row's** | 18 of 19 argvs exit 1 with 0 bytes of stdout under a shape shadow (the nineteenth, `task list`, loads no pack); `RelocateRefusal::ALL` stays 1 of 10 driven. [row-4.md](row-4.md) RC-C7 |
| `(7, A7-F1)` · `(7, A7-F2)` · `(7, C-2)` | — | **NOT RE-DRIVEN here — row 3's** | above |

Passed through and still CLOSED: M51 **D-1** (`ingest` prints `advisory · file-state.absorbed`), M51
**D-2** (all three legs), M51 **D-4 half 1**. M51 **C-1** and **D-3**: not re-driven — no cell of the row
passes through them. **No baseline row regressed.**

## Row 5 — finalize / transaction, against `completions/artifacts/M52/per-axis-review/README.md` (numbered axis 4)

| baseline row | tier there | verdict on rc.24 | the argv / the datum |
|---|---|---|---|
| **`(4, DEFECT 1)`** — a stage-failure route ends in a bare `jigc task finalize` | 2 | **CLOSED** — and **the class is not** | `: > .git/index.lock; jigc task finalize axis-four-probe` → exit 3 `finalize.stage-failed`, route *"`jigc task finalize axis-four-probe` once the embedded git failure is resolved …"*; with flags the route preserves `--carry-staged`, `--approve`, `--approve --carry-staged`; the pasted argv lands the commit. Same on the doc-only arm. Two sibling producers are still bare → `(R5, F6)`. [row-5.md](row-5.md) X-5 |
| **`(4, DEFECT 2)`** — `config set`'s rollback message renders an absolute host path | 3 | **STILL-OPEN(1.x, expected)** | `jigc config set docs-root documentation --format json` with the config layer unwritable → exit 1 `config.repoint-failed`; `message` carries the absolute host path of `.jigc/config/manifest.yaml` while `location.address` is repo-relative. Reproduces on the source pass's own variant (file at 0444) and the driver's (directory at 0555). The rollback also leaves two empty directories → `(R5, F9)`. [row-5.md](row-5.md) X-8 |
| **`(4, DEFECT 3)`** (= the open half of M51 `(4, C4)`) — `milestone create`'s rejected commit says *nothing survives* while the ignore-file amend does | 3 | **STILL-OPEN(1.x, expected)** | under a rejecting hook → exit 1, *"nothing of milestone:amend-probe survives"*, ` M .jigc/.gitignore` (six lines), `gitignore` named 0 times on either stream, text and JSON. Byte-for-byte the rc.16 datum. [row-5.md](row-5.md) X-8 |
| §D lead — CL-4's **stale-base ordering** leg | lead | **now driven → CONFIRMED** (the guard precedes the `ensure`) | a depth-1 clone whose object store lacks the pinned base: `jigc milestone provision prov-probe` → exit 1 `milestone.stale-base`, `.jigc/.gitignore` byte-identical. [row-5.md](row-5.md) X-8 |
| §D lead — **cell D in full** (stage failure × worktree concurrently edited) | lead | **partly driven; OPEN for every population but `promote-destination`** | a *required* git clean filter as the in-transaction racer (the driver's own instrument, not in the fixture library): the compare-and-swap holds; the frame is `(R5, F5)`. [row-5.md](row-5.md) X-7 |
| §D lead — a **genuine concurrent process** racing a pre-image | lead | **OPEN** | no deterministic instrument; unchanged reason |
| §D lead — **`milestone provision`'s post-`ensure` failure point** | lead | **now driven → finding `(R5, F8)`** | `: > .git/worktrees` → exit 1 `milestone.provision-failed`, the amend on disk and unnamed; the clean re-run acks it neither. [row-5.md](row-5.md) X-8 |

The five M51 rows axis 4 closed on rc.16, re-driven **on the doc-only arm**: `C1` holds (the racer's
bytes stand, one `finalize.rollback-conflict`); `C2` / `C3` are not reachable on this arm (a sub-task is
never doc-only); `DEFECT 1` holds (stdout 0 bytes, one stderr document, 8 / 8); `DEFECT 2`'s promote half
holds, its retire half does not apply.

## Row 7 — pinned read contracts, against `completions/artifacts/M53/per-axis-review-rc20/README.md` (numbered axis 5)

| baseline row | tier there | verdict on rc.24 | the argv / the datum |
|---|---|---|---|
| **`(5, DEFECT 1)`** (rc.17) — a mint door commits a record at a fabricated identity | **1** | **CLOSED (stays closed)** — both mint doors | `jigc milestone create ""` · `jigc task amend ""` · `"!!!"` · `"   "` · a title in another script → exit **1** `write.unslugable-title` ×5; `HEAD` unmoved, `git status` empty, no `.jigc/tasks`, no `.jigc/milestones`, no `docs/`, `task list` → *no active tasks*. Driver, source pass and reconciler agree. [row-7.md](row-7.md) block K · R-K |
| **`(5, DEFECT 4)`** (rc.17) — `doc show` blocks a declared but unpopulated optional leaf | 3 | **STILL-OPEN(1.x, expected)** — with a new neighbour | `doc schema vision` advertises `vision:<slug>#meta/grounded-in` `required: false`; `doc show 'vision:vision#meta/grounded-in'` → exit 1 `store.no-such-leaf`. After M55 the same block meets an absent **defaulted** leaf whose whole-doc value is projected: `doc show <doc>` says `fields.status = "open"`, `doc show <doc>#meta/status` says *names no leaf `status`* — declared (`design/doc-read-surface.md`), so not a new finding. [row-7.md](row-7.md) §2 · R.4 |
| **rc.17 `DEFECT A`** — `store.unknown-type` emits a URI-shaped target at `doc show`, the bare id at `doc schema` | 3 | **STILL-OPEN(1.x, expected)** | `doc show 'nosuchtype:x'` → target `nosuchtype:x`; `doc schema nosuchtype` and `doc list nosuchtype` → target `nosuchtype`. [row-7.md](row-7.md) H1 – H3 |
| **`(5, DEFECT 3)`** (its `doc show` half) — the colon-less-address bail carries no code | 3 | **STILL-OPEN(1.x, expected)**; the `rename` half **not re-driven** | `doc show nosuchtype` → exit 1, `{"error":"malformed address …"}`; no `blocking · <code>`, no key; same on `--task`. [row-7.md](row-7.md) H6 · H28 |
| M52 §D lead — **`store.no-such-leaf`**, the 2nd `ENVELOPE_OWED_CODES` member | lead | **CLOSED as a lead** — the owed envelope is paid | driven at four producers on both arms: exit 1, `Reject::Findings`, stable key `{code, target}`, stdout 0 bytes. `(5, DEFECT 4)`, which rides the same code, stays open. [row-7.md](row-7.md) R-H |
| `(5, DEFECT 2)` · `(5, C1)` · `(5, D1)` · `(5, DEFECT 1 · rc.20)` = `(2, A2-2)` · `(5, DEFECT 2 · rc.20)` | 3 · 3 · 3 · 2 · 3 | **NOT RE-DRIVEN** — outside the row's subject | they stand where rc.20 left them (STILL-OPEN); **their absence here is not closure**. The Codex pass read *no disposition change*; a source read is not a drive |
| `(5, C-13)` · `(5, C-18)` | lead | **OPEN LEADS, unchanged reason** | a doc-comment no invocation emits; four `#[test]` targets on the debug build |

M51 **`DEFECT A · B · C · D`** on the three read verbs: still CLOSED (outside any repository 5 / 5,
cwd deleted 3 / 3, each `Reject::Error`, stdout 0 bytes).

## Row 8 — composed surfaces, against `completions/artifacts/M53/per-axis-review-rc19/README.md` (numbered axis 6)

| baseline row | tier there | verdict on rc.24 | the argv / the datum |
|---|---|---|---|
| **`(6, D-1)`** — orientation reports a live task's findings without the repository posture | 2 | **CLOSED** — the Codex pass's *STILL OPEN* is refuted | after `git bisect start`: `jigc start` → exit 0, `blocking · repo.operation-in-progress` on the text arm and as `tasks[0].findings[0].code` on the wire; `jigc task validate posture-parity` → exit 1, the same code. [row-8.md](row-8.md) RD-D1 |
| **`(6, D-2)`** — `fix-task` composable by name; its forbidden door is the only one that works | 2 | **STILL-OPEN(1.x, expected)** | line 25 forbids `jigc task finalize`; `jigc task finalize fix-the-thing` → exit 0, *finalized … fix: fix the thing*, 1 file. The other half of the same sentence is `(R8, F-2)`. [row-8.md](row-8.md) RD-D2 |
| **`(6, D-3)`** — the orientation `Preview:` footer states the pre-M52 rule | 3 | **STILL-OPEN(1.x, expected)** | footer byte-for-byte as at rc.19; `start --workflow milestone-execution` → 1 `workflow.verb-routed`. [row-8.md](row-8.md) RD-D3 |
| **`(6, D-4)`** — an empty `milestone execute` walk does not state its empty case | 3 | **STILL-OPEN(1.x, expected)** | exit 0, 0 `Spawn:` lines, stderr empty, no empty-case sentence; `milestone finalize` → 3 `milestone.zero-contribution`. [row-8.md](row-8.md) RD-D4 |
| **`(2, N-1)`** = **`(6, A6-R1)`** — the hook announces a rename on a commit containing none | 3 | **CLOSED** (rc.20's closure carried onto this axis) | after a committed deletion the next unrelated commit's stderr is empty; a staged placement-doc `git mv` is **blocked** (rc 1). [row-8.md](row-8.md) RD-N1 |
| `(6, L-1)` non-UTF-8 path | lead | **OPEN LEAD — unbuildable** on this filesystem (`OSError 92`) | — |
| `(6, L-2)` the `suppressed:` guarantee is manifest-scoped | lead | **OPEN LEAD, unchanged (declared)** | [row-8.md](row-8.md) RD-L2 |
| `(6, L-3)` the route-span carve-out enumerates two span kinds | lead | **OPEN LEAD** — the third kind is emitted (87 `Spawn:` lines driven); the carve-out is a test-support property (read) | — |
| `(6, L-4)` a `jigc_home`-bound door acks a repo-relative path | lead | **OPEN LEAD, unchanged (declared convention)** | [row-8.md](row-8.md) RD-L4 |
| `(6, L-5)` three declared bounds | lead | **OPEN LEAD, unchanged ×3 (declared)** | [row-8.md](row-8.md) RD-L5 |
| `(6, L-6)` `pack-resource-missing` × this axis's doors | lead | **reached by both drivers — matches contract; discharged** | four doors, exit 1 ×4, `pack.resource-missing`, stdout 0 bytes. [row-8.md](row-8.md) RD-L6 |
| `(6, L-7)` a project pack listed in `packs.yaml` | lead | **OPEN LEAD — driven by nobody** | needs a hand-written pack tree |
| **M55's declared bound** — `jigc task validate <sub>` exits 3 on the omitted commit doc under `squash=true` | bound | **HOLDS AS DECLARED** at the door it names; **wider at orientation** → `(R8, F-4)` | `jigc task validate code-sub` → 3, the two `schema-conformance.*` codes at `commit:<sub>`, the composed text asking for neither, the join landing without either. [row-8.md](row-8.md) RD-B · RD-F4 |
| M55's second declared bound — a docs-only sub-task under `squash=false` still authors an unread doc; the knob flip meets the join's routed block | bound | **HOLDS AS DECLARED** | [row-8.md](row-8.md) RD-K4 |
| `(2, A2-2)` | 2 | **NOT RE-DRIVEN** — named so it is not conflated; its datum was not seen | — |

Still CLOSED, carried by a drive: M51 **`A6-1`**, **`A6-2`** (the re-derived 15-member set × 3 argv
forms, 45 / 45 / 45 — a *fourth* form is `(R8, F-3)`), **`A6-3`**; M55 **E1**, **E2**, and *structural-op
deltas on a fresh compose only* (now also at the finalize-time arm and the verb-minted composes).

## Row 9 — adopter docs & help, against `completions/artifacts/M52/per-axis-review/README.md` (numbered axis 8)

| baseline row | tier there | verdict on rc.24 | the argv / the datum |
|---|---|---|---|
| **`(8, N-1)`** — `uninstall --help` carries two numerals (*"deletes all four"* · *"all three guards"*) | 3 | **STILL-OPEN(1.x, expected)** — the Codex pass's *CLOSED* is refuted | with the three enumerated guards clean and only `.jigc/displaced/<task>/notes.txt` present: `jigc uninstall` → exit 1 `uninstall.foreign-bytes`; `jigc uninstall --force` → exit 0, deletes it and narrates it. *"Inert when all three guards are already clean"* does not hold. [row-9.md](row-9.md) RD-N1 |
| **`(8, N-2)`** — a retitle-only `rename` acks and commits as `X -> X` | 3 | **STILL-OPEN(1.x, expected)** | `jigc rename vision:vision --to 'New Vision' --slug vision` → exit 0, ack *"renamed vision:vision -> vision:vision (VISION.md -> VISION.md)"*, subject `rename VISION.md -> VISION.md`, while `VISION.md:5` became `# New Vision`. New datum: `--format json` carries `"title"`. [row-9.md](row-9.md) RD-N2 |
| M51 **`CX-3`** — re-driven because M54 rewrote the sentence that closed it | — | **CLOSED on the rewritten sentence** | the installed guide's one install command is `cargo install jigc --version '^1.0.0-rc.1' --locked`: no path, no clone step; 0 hits for `git clone`, `cargo build --release`, `install -m755`, `cargo install --path`. **Its execution is row 1's** (rows 6.1, 8.4). [row-9.md](row-9.md) RD-B |

Also reached: M51 **`CX-1`** CLOSED · **`CX-2`** CLOSED · **`D-2`** CLOSED (all 11 `COMMITTING_DOORS`
clauses are in their door's help; commits driven at 9 of the 11 rows). M51 **`D-1`**: **NOT RE-DRIVEN** —
no cell reached it; nothing here says it holds on rc.24.

## The rows with no baseline — the one item each carried

| row | item | status on rc.24 |
|---|---|---|
| 1 | **U.3** — `jigc uninstall` after a failed first `setup` leaves `AD` / `AM` index entries (declared open, owed at M57) | **STILL-OPEN (expected, declared)** — re-driven: status `AM .claude/settings.json` + seven `AD` |
| 2 | M54's *Not run*: the **Linux replaced-binary-while-running** arm (`/proc/self/exe`) | **DRIVEN — holds**, on the published rc.24 as `cargo install` builds it for Linux, in a container: the image file removed and an impostor at its path → the probe ran, the impostor did not; 12 of 12 mid-run replacements likewise. Driven at `validate` and `task validate` only |
| 6 | the declared **edit-gate bound** (`design/findings-channel.md` §1.5; M55 baseline C7 / F14) | **STILL-OPEN (expected) — HOLDS AS DECLARED**, and measured: after a `create.already-exists` refusal the same report task reaches the existing finding through **six** leaves — three more than §1.5 names — and `task finalize` lands the rewrite at exit 0 |
| 10 | `(2, DEFECT C)` — the neighbour on the same seam | **NOT RE-DRIVEN** — numbered axis 2's; nothing in row 10 changes its status |

---

# FINDINGS

## A · CONFIRMED — every new finding, tiered on the charter's predicate

The predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying or
moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the binary
does not do.

**65 finding rows: 6 tier-1 · 7 tier-2 · 52 tier-3.** Every one was re-driven by its row's reconciler
(or, for a Codex-origin finding, driven by it for the first time) and carries a repro block in its row
file. **Each entry below carries the same labelled fields, so it can be filed through jigc at the
port:**

- `tier:` — as the row's reconciler graded it; a disagreement between the two Opus agents is said.
- `door:` — the clap leaf (`VERB_KINDS` spelling) the finding is filed against.
- `found-in:` — `review:rc24-per-axis/(R<row>, <id>)`, the grammar `design/findings-channel.md` §1.3 gives.
- `origin:` — `driver` or `codex` (a Codex lead driven to a repro by the reconciler).
- `new-code:` — **inside M54's code** · **inside M55's code** · **inside the co-author trailer** ·
  **outside the new code** · or **not established**, where neither pass dates the mechanism. A dating
  marked *(read)* is a source or history read, not a drive on an older binary — no older binary was
  installed and none might be built.
- the fenced `repro:` block — condensed from the row file to *setup / argv / observed / after*. The row
  file's block governs.
- `contract:` — the sentence contradicted, and its home.
- `pinned-by:` — in the grammar `implementation/pinning.md` §3 owns: `<module>::<test_name>`, or
  `UNPINNED: <why>`. By that section the field is filled when the fix's red test lands; **none has, so
  every entry here reads `UNPINNED:`**, and its *why* names the nearest standing test — the neighbouring
  cell, or the test that asserts today's behaviour — so the fix knows what already fences the seam.
  **Bound:** the assembler searched `crates/cli/tests/` by code and by sentence for each entry and did
  not read every test, so *no test found* is not *no test exists*.

### Tier 1 (6)

#### `(R1, F1)` · on an unborn `HEAD`, `jigc setup` overwrites untracked user bytes at a whole-rewrite install path at exit 0 with nothing said

> **After assembly — the adversarial re-drive** ([tier1-verification/R1-F1.md](tier1-verification/R1-F1.md)): **upheld at tier 1, low reach, and wider than this entry** — the class is four paths (`.jigc/AGENT.md`, `.jigc/config/.gitkeep`, `.jigc/version` whole; `.jigc/config/packs.yaml` comments and formatting only), plus a symlink variant that overwrites a file outside the install set; the trigger is an **unborn `HEAD`** (an orphan branch in a born repository reaches it), not only a zero-commit repository; a plain `git reset` is enough for repro B; and the design declares the *exemption*, not the *loss*.

- `tier:` **1** — upheld by the reconciler after arguing it downward
- `door:` `setup`
- `found-in:` `review:rc24-per-axis/(R1, F1)`
- `origin:` driver
- `new-code:` **outside the new code** — the unborn `??` exemption is the M51-era install guard's
  (`15b919ca`, 2026-09-15 *(read)*). **Repro B passes through M54's S22 cell**: the failed-first-run
  re-arm M54 added holds while the index keeps the staged entries and does not hold on an unborn `HEAD`
  once they are unstaged — which is also where the Codex pass's *"post-failure user edits re-arm both
  worktree and index checks"* is refuted.

```text
repro:
  setup     bare rig (for the isolated HOME); F=$(mktemp -d "$RIG/unborn.XXXXXX"); cd "$F"; git init -q .
            repo-local identity trial@example.com; mkdir -p .jigc/config
            printf '# Team notes for agents — USERMARK-A written by a human\n…' > .jigc/AGENT.md
            printf '# USERMARK-P why we pin dev only: see the team wiki\npacks:\n- dev\n' > .jigc/config/packs.yaml
  before    git rev-list --count --all → 0 ; status: ?? .jigc/AGENT.md / ?? .jigc/config/packs.yaml
            command grep -rc USERMARK .jigc → .jigc/AGENT.md:1  .jigc/config/packs.yaml:1
  argv      jigc setup
  observed  exit 0 · "jigc setup — adapter installed … install commit → <sha>" · no finding, stderr empty
  after     status empty ; command grep -rc USERMARK .jigc CLAUDE.md → every file :0
            git log --all -S USERMARK → 0 commits ; git fsck --unreachable → 0 ; every blob scanned → none
  control   the same two files in a repository with ONE commit: jigc setup → exit 1,
            setup.dirty-install-path names both paths, "nothing was installed", the mark still 1 and 1
  repro B   unborn repo, no identity (user.useConfigOnly true): jigc setup → exit 1 setup.install-commit, 9 paths "A "
            identity restored. arm 1 (index kept), a line appended to .jigc/AGENT.md → exit 1, re-arms, mark 1
            arm 2: git rm -r -q --cached . (9 paths ??), the same append → exit 0, findings [], mark 0,
            git log --all -S → 0, blob scan → none
```

- `contract:` `crates/cli/guides/QUICKSTART.md` → *1. `jigc setup`* — *"If any of them already carries
  changes that are in no commit — staged, unstaged or untracked — it stops with one blocking
  `setup.dirty-install-path` … your bytes are still exactly where you left them — including at the files
  jigc regenerates whole (`.jigc/AGENT.md`, `.jigc/config/packs.yaml`)"* (no unborn carve-out);
  `design/validation.md` → the `setup.dirty-install-path` row — *"never a silent sweep"*. **The caveat
  that travels with it:** the same row states the unborn `??` exclusion as a door rule, on the rationale
  that setup *merges into* the host files — true of `CLAUDE.md` (driven: the mark rides `HEAD`), false of
  the two paths the install rewrites whole.
- `UNPINNED:` the cell is the **intersection of two pinned neighbours** —
  `setup_install_pathspec_guard::an_untracked_install_path_on_an_unborn_head_is_the_stated_exemption`
  plants `CLAUDE.md` (a merged-into path) and asserts the bytes ride the commit;
  `setup_install_pathspec_guard::a_whole_rewrite_install_path_refuses_with_the_users_bytes_intact`
  plants `.jigc/AGENT.md` on a **born** repository. No test plants a whole-rewrite path on an unborn one.

#### `(R3, F7)` · with no file-state baseline, an uncommitted out-of-band edit made after a task's first write to that doc is overwritten by `task finalize` at exit 0

> **After assembly — the adversarial re-drive** ([tier1-verification/R3-F7.md](tier1-verification/R3-F7.md)): **upheld at tier 1, both halves, on a plain second clone with no other act** — the window is *the whole of the first task in every clone* (only a landed finalize or `jigc ingest` writes the baseline); it reaches **`milestone finalize`** and a **location** doctype (`adr`) as well, closing this row's open lead L7; and a sibling with the baseline *present* — a task driven from a user-made linked worktree — was driven once and is flagged as a lead of its own, not graded.

- `tier:` **1** — the reconciler's; the driver listed the cell NOT DRIVEN and wrote *no tier-1 row*
- `door:` `task finalize`
- `found-in:` `review:rc24-per-axis/(R3, F7)`
- `origin:` driver (its NOT-DRIVEN entry #17), **driven by the reconciler**
- `new-code:` **outside the new code** — the `UNKNOWN → baseline-adopt` mechanism is dated M46 by the
  standing pair's header *(read; not driven on an older binary)*

```text
repro:
  setup     rig committed-singletons; git clone -q $REPO $RIG/clone2 ; cd $RIG/clone2      (a second clone)
            .jigc/state/file-state.json present: no
            jigc validate → exit 0, advisory file-state.un-baselined, route "no action needed — the doc is
              baselined on its next author or finalize"
            jigc start --workflow single-task "sharpen the open questions"
            jigc doc set-slot vision:vision#open-questions --from-file - --task <id>   (+ the commit-doc writes)
            baseline for VISION.md after the first write: none
            (a hand line appended under VISION.md's thesis in the worktree, uncommitted)
  before    the hand line: worktree 1 · the task's staged copy 0 · HEAD 0
  argv      jigc task validate <id> ; jigc task finalize <id>
  observed  exit 0 (advisories file-state.staged-copy, file-state.baseline-adopt — both "no action needed")
            exit 0 "finalized <sha> — docs(vision): sharpen the open questions"
  after     HEAD moved ; status clean ; the hand line: worktree 0 · HEAD 0 · files under .jigc carrying it 0
  also      unmanage-first (jigc unmanage VISION.md, then the same sequence) → the same loss
            unmanage after the conflict-block fired → validate 0, --dry-run 0 "would commit … promoted VISION.md",
              finalize 0 ; every commit on every ref 0
  controls  baseline present → exit 3 reconciliation.conflict-block, HEAD unchanged, the line still on disk
            the second clone after one unrelated finalize → exit 3, the same block
            edit BEFORE the first write → carried into the copy-in, lands (the declared silent merge, no loss)
            edit COMMITTED during the task → exit 3 finalize.base-mismatch
```

- `contract:` the conflict guard itself — with a baseline the identical sequence blocks
  (`reconciliation.conflict-block`: *"an external edit and this task's staged writes both changed it"*,
  the control above); and `design/storage.md` → *What none of this buys*, which declares that a lost
  baseline *"switches the guarantee off — silently, at exit 0, with both sides' bytes merged into one
  commit"* — **a merge**. The order driven here costs a **loss**: one side's bytes are gone, and neither
  that paragraph nor the standing pair states it. The surfaces on the way say *no action needed* three
  times. **Whether the declaration covers this cell is the triage's call**; the reconciler's four
  weights (uncommitted out-of-band bytes · a pre-range mechanism · a window that closes at a clone's
  first finalize · two of three ways need an unrouted operator act) travel with it in
  [row-3.md](row-3.md) §8.5.
- `UNPINNED:` `reconciliation_baseline_contrast::a_lost_file_state_baseline_turns_a_conflict_block_into_a_silent_merge` pins the **other order** (the
  human edit before the in-task write, asserting the commit carries both). No test makes the edit after
  the copy-in.

#### `(R6, D-1)` · the fan-out join's collision suffix is minted onto an id already on disk, and `milestone finalize` overwrites the doc there at exit 0

> **After assembly — the adversarial re-drive** ([tier1-verification/R6-D-1.md](tier1-verification/R6-D-1.md)): **upheld at tier 1, both halves, and wider than this entry** — the suffix is one way in, not the mechanism: **the milestone planner runs no clobber guard at all**, so an untracked file at a sub-task doc's home is overwritten at exit 0 with no collision and no suffix; both squash modes, another pack and doctype (`adr`), a foreign non-conformant file; the single-task door refuses the identical state (`finalize.promote-clobber`). Reach: low.

- `tier:` **1** — driver and reconciler agree
- `door:` `milestone finalize` (the committing door that overwrites); `milestone join` mints the occupied id
- `found-in:` `review:rc24-per-axis/(R6, D-1)`
- `origin:` driver — the Codex pass states the fact (*the join stays outside the pre-check and keeps
  its suffixing*) and derives no defect from it
- `new-code:` **outside the new code** — driven under `park-idea` sub-tasks (no `new: true`), so it
  predates M55; `new: true` does not close it, and M55's findings channel (many reporters, one doc per
  finding, title-minted ids) is the workload that reaches it

```text
repro:
  setup     rig fresh. An earlier finding filed and landed through report-inconsistency:
              jigc doc create inconsistency --title "Stage 2" …  (description carries EARLIER-FINDING-MARKER)
              jigc task finalize …  → promoted docs/inconsistencies/stage-2.md
            an untracked, hand-written, conformant note at docs/inconsistencies/stage-3.md (UNTRACKED-NOTE-MARKER)
            jigc milestone create "stage reports" ; add-task ×3 --workflow report-inconsistency ; provision ; execute
            in each worktree: jigc doc create inconsistency --title "Stage" --task <sub> → inconsistency:stage [exit 0] ×3
  before    stage-2.md (committed): EARLIER-FINDING-MARKER → 1 ; stage-3.md (untracked): UNTRACKED-NOTE-MARKER → 1
  argv      jigc milestone join stage-reports ; jigc milestone finalize stage-reports --format json
  observed  join exit 0: "inconsistency:stage-2 (created · from bravo-…) ← suffixed -2 on collision",
              "inconsistency:stage-3 (created · from charlie-…) ← suffixed -3 on collision" ; no finding
            finalize exit 0: manifest promoted stage-2.md · stage-3.md · stage.md ; displaced: []
  after     stage-2.md: EARLIER-FINDING-MARKER → 0 ("bravo reporter" → 1) ; git log --all -S → 2 (history only)
            stage-3.md: UNTRACKED-NOTE-MARKER → 0 ; git log --all -S → 0 ; command grep -rl . → nothing (.jigc/ included)
            jigc validate → exit 0, clean of it ; jigc doc show inconsistency:stage-2 → the "bravo" finding
  also      two park-idea sub-tasks titled "Pair idea" over a committed idea:pair-idea-2 → the same overwrite
```

- `contract:` `design/findings-channel.md` §4 (the loss the create-only entry exists to stop; *"two
  sub-tasks that mint one slug in isolation still land `-2` at the join"* — here a landing **on** a filed
  finding) and §1 (*append-only is free at the doc level*); `design/storage.md` → the by-task-id join,
  rule 4 (*colliding new instances … are distinct docs, not a clash*); and the clobber guard every other
  door carries — the create doors, `doc rename` and the single-task finalize all refuse the same
  collision.
- `UNPINNED:` `create_only_gate::fan_out_sub_tasks_still_join_with_a_suffix` and
  `milestone::milestone_join_suffixes_a_created_collision_and_reports_the_decision`
  pin the suffix with the suffixed id **free**. No test occupies it.

#### `(R6, D-7)` · a dangling symlink at the minted home: the create-gate passes it, and `task finalize` exits 0 committing the symlink while the finding lands untracked

> **After assembly — the adversarial re-drive** ([tier1-verification/R6-D-7.md](tier1-verification/R6-D-7.md)): **upheld at tier 1 on the harm disjunct only — *"the weakest tier-1 row"*, near adversarial-only reach** — the mechanism is not M55's gate (it reproduces on `adr` under an entry without `new: true` and on the `vision` placement singleton); two members are worse than the filed one (a dangling target *outside* the repository, and a link to a non-regular target, where the authored prose is in no place afterwards). Its verifier: not the row that should cost a release cycle on its own, but cheap to fold into a fix pass other rows trigger.

- `tier:` **1 on the harm disjunct** (reconciler) — **the driver proposed tier 3**. The loss half is not
  shown; see the headline
- `door:` `task finalize`; `doc create` passes the link
- `found-in:` `review:rc24-per-axis/(R6, D-7)`
- `origin:` driver
- `new-code:` **outside the new code** — a planted link, and symlinked paths are numbered axis 1's class
  (not re-opened by this run). M55's gate is the door that passes it; the same promote under an entry
  **without** the key was not driven

```text
repro:
  setup     rig fresh; ln -s nowhere.md docs/inconsistencies/dangling-home.md      (nowhere.md does not exist)
            jigc start --workflow report-inconsistency "edge finalize dangling home"
            jigc doc create inconsistency --title "Dangling home" --task <t> → exit 0 ; prose carries DANGLING-MARKER
  argv      jigc task finalize <t>
  observed  exit 0 "finalized <sha> — docs(findings): file a finding / promoted docs/inconsistencies/dangling-home.md
              / 1 file committed"
  after     git ls-tree HEAD docs/inconsistencies/ → 120000 … dangling-home.md ; git cat-file -p → nowhere.md
            git status --short → ?? docs/inconsistencies/nowhere.md (holds the marker)
            git log --all -S DANGLING-MARKER → 0 ; jigc doc show inconsistency:dangling-home → serves it (locally)
            in a clone: at HEAD~1 jigc doc list inconsistency → exit 0 ; at HEAD → exit 1, code-less
              ("reading the committed doc … No such file or directory") ; doc show → exit 1 store.not-found
  control   an UNTRACKED dangling link at a home, no finalize: jigc doc list inconsistency → exit 1 already
            a symlink to an EXISTING doc at the home is refused by the gate (create.already-exists)
```

- `contract:` the finalize ack (*promoted … 1 file committed*) and `design/findings-channel.md` §4 (an
  id already on disk is refused with nothing staged). What the committed record holds is a link whose
  target is outside it.
- `UNPINNED:` `create_only_gate::an_untracked_file_at_the_home_is_refused` pins a
  regular file at the home. No test plants a dangling link.

#### `(R6, K-1)` · the create-only gate is check-then-use with nothing held between: a home that becomes occupied inside the window is copied in at exit 0 under a `new: true` entry

> **After assembly — the adversarial re-drive** ([tier1-verification/R6-K-1.md](tier1-verification/R6-K-1.md)): **reproduces, both halves — and is NOT tier 1 as a row of its own: tier 3, with a tier-2 tail.** The loss half is not the gate's; it belongs to a pre-M55 class — *an untracked file at a doc home is read as committed, and no door is atomic against a non-jigc writer* — which the verifier reached with no race under an entry without `new: true` and through `task finalize`'s own check-then-use window (13 losses in 80), and which it says should be ruled on once, as a class. The window here was measured at ≈1 ms. The entry below is left as the reconciler graded it.

- `tier:` **split by the reconciler** — **tier 1** *"by the predicate's letter"* when the writer in the
  window is outside jigc (both halves shown); **tier 2** in the source pass's own scenario, two jigc
  reporters (the committing door refuses)
- `door:` `doc create` (also `doc author`); the loss lands at `task finalize`
- `found-in:` `review:rc24-per-axis/(R6, K-1)`
- `origin:` codex — *"potentially tier 1"*; its weighting was decided by the drive
- `new-code:` **inside M55's code** — the `new: true` gate (S7, revised by R2)

```text
repro:
  setup     rig fresh, one finding landed. A conformant victim file (description "RACE-VICTIM-MARKER written by
            another hand.") parked outside the worktree; a toggler renames it into and out of
            docs/inconsistencies/race.md as fast as it runs
  controls  no toggler, file absent → doc create exit 0, existed false
            no toggler, file present → exit 1 (create.already-exists, inconsistency:race), nothing staged
  argv      jigc doc create inconsistency --title "Race" --task race-attempt-0 --format json      (toggler running)
  observed  exit 0 · existed true ; provenance "edited-from-base" ; the staged copy carries RACE-VICTIM-MARKER
            tally: 60 creates → 31 refused · 19 exit 0 existed true (the breach) · 10 exit 0 existed false
                   60 doc author → 31 refused · 8 copied in (payload already written over the victim's body) · 21 created
  then      toggler stopped, the victim at its home (untracked): RACE-VICTIM-MARKER → 1 ; git log --all -S → 0
            the step's next lines: set-field #meta/kind · set-slot #description (REPORTER-MARKER) · fill
            jigc task validate → exit 0 (staged-copy, baseline-adopt advisories)
            jigc task finalize --format json → exit 0, promoted docs/inconsistencies/race.md, displaced: []
  after     race.md: RACE-VICTIM-MARKER → 0 · REPORTER-MARKER → 1 ; nowhere under $REPO ; in no commit ; status empty
  variant   the source pass's own scenario — a second reporter's task finalize landing the id mid-create:
            1 hit in 160 raced creates (exit 0, existed true) ; that task's finalize → exit 3
            finalize.base-mismatch ; the landed finding's checksum unchanged                       (no loss)
```

- `contract:` `design/findings-channel.md` §4 and `design/write-commands.md` → *The create-gate* —
  *adjudicated before anything is copied in*; the refusal's own message (*"the existing one is never
  copied in for update"*). The JSON ack does say `existed: true`; the text ack is the sentence a re-run
  over the task's own doc also prints, so a reporter cannot tell the two apart.
- `UNPINNED:` the gate's tests are sequential
  (`create_only_gate::a_reused_title_is_refused_by_both_doors_with_nothing_staged`);
  a window between the pre-check and the engine create has no deterministic fence.

#### `(R9, F5)` · `jigc uninstall`, without `--force`, destroys files inside `.jigc/logs/`, `.jigc/state/` and `.jigc/index/` at exit 0, named by nothing — where its help says every such file blocks

> **After assembly — the adversarial re-drive** ([tier1-verification/R9-F5.md](tier1-verification/R9-F5.md)): **upheld at tier 1, at the low end of tier-1 severity, and wider than this entry** — two further members (a leaf file directly under `.jigc/tasks/` and under `.jigc/milestones/`), a tracked-then-edited leg, and `--force` narrating nothing for these paths; the boundary is *every byte inside an `ENTRIES` directory no other guard's subject reaches*. The **invocation-log member** is the one its verifier says a human ruling could most defensibly move; the tier-3 reading is argued in full in the file.

- `tier:` **1 (proposed by driver and reconciler; the ruling handed to the human)** — the tier-3 reading
  is stated by both: `logs/`, `state/` and `index/` are outside the third guard's designed subject and
  two of the three are rebuildable caches
- `door:` `uninstall`
- `found-in:` `review:rc24-per-axis/(R9, F5)`
- `origin:` driver
- `new-code:` **outside the new code** — neither M54, M55 nor the trailer changed this door's guards
  (the workbench guard: `1bd5efaf`, 2026-09-05 *(read)*). Found because the row drives
  `uninstall --help`'s refusal sentences at the verb

```text
repro:
  setup     rig fresh; jigc validate ; mkdir -p .jigc/logs .jigc/state .jigc/index
            one file planted in each, and a control at .jigc/config/notes.txt, all carrying MARKER-RC9
  before    command grep -rl MARKER-RC9 .jigc → the four files
  argv      jigc uninstall
  observed  exit 1 · uninstall.untracked-workbench-file — lists .jigc/config/notes.txt ONLY     (the guard is alive)
            (the control moved out of the repository; before-control again → the three files)
  argv      jigc uninstall                                                                      (no --force)
  observed  exit 0 · warns about the 5 tracked install files and nothing else · "removed .jigc/"
  after     command grep -rl MARKER-RC9 . → nothing ; git stash list → 0 ; HEAD unchanged
  no plant  jigc config set invocation-log true (committed); three invocations → .jigc/logs/invocations.jsonl: 3 records
            jigc uninstall → exit 0 ; the log is not named ; afterwards the file holds 1 line, the uninstall's own
```

- `contract:` `jigc uninstall --help` — *"Four states it refuses instead of destroying, because `.jigc/`
  is their only copy — … and **any other file under `.jigc/` that no index has a copy of** … blocks with
  `uninstall.untracked-workbench-file` … Any of them removes nothing until you re-run — or pass
  `--force`"*. Precedent the reconciler cites: the same door destroying a plain file planted **at** an
  ignore-entry name was counted an exit-0 loss and fixed (M52 baseline L-2); this is one level down.
- `UNPINNED:` `uninstall_workbench_subject::a_hand_dropped_workbench_file_blocks_and_the_install_survives` pins the control arm. No test plants
  under `logs/`, `state/` or `index/`.

### Tier 2 (7)

#### `(R3, F4)` · a pulled edit to the machine-owned milestone record is absorbed by the next unrelated finalize, and every milestone door then answers `milestone.terminal`

- `tier:` **2** — re-tiered by the reconciler (the driver proposed 3)
- `door:` `task finalize` (the unrelated task that absorbs) · `milestone add-task` (the record door)
- `found-in:` `review:rc24-per-axis/(R3, F4)`
- `origin:` driver
- `new-code:` **not established** — whether the unrelated-finalize absorb predates M55 was not driven;
  M55 added the store route that announces it (*absorbed at the next finalize*)

```text
repro:
  setup     rig committed-singletons + a teammate clone. jigc milestone create "Vision pass" ; add-task …
            the sub-task stages prose (set-slot vision:vision#open-questions)
            the teammate edits docs/milestone-records/vision-pass.md (status: active → joined), pushes ; git pull
  argv      jigc milestone add-task vision-pass "Evict cold entries"
  observed  exit 1 reconciliation.conflict-block — "restore … to what jigc last wrote … and re-run"   (as designed)
  argv      jigc task finalize unrelated-work-1 --format json
  observed  exit 0 · (reconciliation.absorb, docs/milestone-records/vision-pass.md) advisory "no action needed"
  after     milestone add-task · finalize · join · list-tasks · discard → exit 1 milestone.terminal ("a settled
            milestone is over and has no workbench") ; jigc task list → the live sub-task, its staged prose 1 → 1 ;
            jigc task finalize <sub> → exit 3 finalize.milestone-sub-task
            the earlier route run late (git revert the teammate's commit) → conflict-block again, the same route
            the exit that works — a second unrelated finalize — is on no route of that door
```

- `contract:` `design/team-ready-state.md` → *No-silent-overwrite discipline* (*"detected +
  conflict-blocked, **not** hand-editable and merged like an authored doc"*) and
  `design/reconciliation.md` (*every drift of the machine-owned record conflict-blocks there, pulled or
  not*). Not tier 1: the sub-task's staged prose and the record's bytes are intact.
- `UNPINNED:` no test found that lands an unrelated finalize between a pulled record edit and the
  record door.

#### `(R4, F2)` = `(R6, D-8)` · the malformed-front-matter refusal names the key and not the workflow, at every compose door, for any workflow asked; its line number is one short

- `tier:` **2** by row 4's reconciler · **3** by row 6's (as `(R6, D-8)`) — **two reconcilers, two
  tiers, one defect**; recorded here at the higher and not harmonised
- `door:` `start` (also `workflow`, `describe`, `migrate`; row 6 adds `doc create`, `doc author`,
  `task validate`, `task finalize`)
- `found-in:` `review:rc24-per-axis/(R4, F2)` · `review:rc24-per-axis/(R6, D-8)`
- `origin:` driver ×2 (two rows independently)
- `new-code:` **outside the new code** for the message (the generic flatten path is older than M55);
  **M55's closed-key rule is what makes it reachable through a one-letter typo**

```text
repro:
  setup     rig fresh. .jigc/config/workflows/report-jigc-feedback.yaml = the shipped file with ONE edit on file
            line 10, committed:  { type: jigc-feedback, as: feedback, new: true } → … nwe: true }
  argv      jigc start --workflow report-jigc-feedback probe-one | jigc workflow quick-fix --preview | jigc describe
            | jigc start --workflow quick-fix probe-three | jigc start probe-two
  observed  exit 1 each: "blocking · workflow-refs.malformed-front-matter — … allows-create[0]: unknown field `nwe`,
            expected one of `type`, `as`, `new` at line 9 column 42 / route: fix the workflow/step/catalog definition
            the message names …"            — no workflow id, no file, no `at:` line; file line 10 is "line 9"
  after     jigc validate → exit 0, the same lines + "at: workflow:report-jigc-feedback" (report-only);
            no refusal routes there. Under --format json the refusing doors emit {"error": …}: no (code, target) key
```

- `contract:` the finding's own route (*"the definition the message names"* — the message names none);
  the route floor. `design/findings-channel.md` §10's cell (*a load error naming the key*) is met.
- `UNPINNED:` `create_only_gate::start_refuses_a_misspelt_entry_key_naming_it` and
  `create_only_gate::validate_reports_a_misspelt_entry_key_and_exits_zero` pin the behaviour as it is
  (the key named; `validate` exits 0) — §10's cell, not a workflow id in the refusal.

#### `(R4, F3)` · `relocate.frozen-doctype`'s route is a no-op for a frozen doctype that has no versioned snapshot — both M55 doctypes

- `tier:` **2**
- `door:` `relocate`
- `found-in:` `review:rc24-per-axis/(R4, F3)`
- `origin:` driver
- `new-code:` **outside the new code** for the route (M40's); **M55 adds two members** to a class that is
  every manifest-listed doctype still at its first schema-version (read off the manifests, not each driven)

```text
repro:
  setup     rig fresh, both findings docs filed.
            git mv docs/jigc-feedback/<slug>.md docs/feedback/<slug>.md ; git commit
  argv      jigc relocate jigc-feedback --from docs/feedback
  observed  exit 1 · relocate.frozen-doctype — route: "`jigc migrate-corpus` walks every prior home the doctype's
            versioned snapshots declare and lands the move under the freeze"
  argv      jigc migrate-corpus                                                    (the route, verbatim)
  observed  exit 0 · "0 migrated, 1 already current, 0 blocked" — the stranded doc neither named nor moved
  after     the file's sha and HEAD identical before and after ; jigc doc show jigc-feedback:<slug> → exit 1
            store.not-found ; the working exits are at other doors (ingest: "move it into docs/jigc-feedback/";
            validate: "revert the `git mv`")
```

- `contract:` the route itself; `design/findings-channel.md` §1.7 (*no snapshot and no migration* for a
  schema-version-1 doctype), which makes the walk the route describes empty by construction.
- `UNPINNED:` `relocate.frozen-doctype` is reached by `crates/cli/tests/anyhow_route_spans.rs` only; no
  test found that runs the route on a doctype without a snapshot.

#### `(R5, F1)` · a formatter-style `pre-commit` hook makes the doc-only commit leave a staged reversion of its own doc, call the committed doc `left-staged`, and block the code task beside it

- `tier:` **2** — **the row's closest call**: the reconciler states that if an index entry nobody staged,
  left at exit 0 by a committing door, is itself *repository harm*, this is tier 1 on the evidence here
- `door:` `task finalize`
- `found-in:` `review:rc24-per-axis/(R5, F1)`
- `origin:` driver — the Codex pass's *"no new source-grounded completeness defect"* is refuted by it
- `new-code:` **inside M55's code** — the doc-only arm's path-scoped `git commit -- <paths>` (the same
  phantom exists at the record-only doors since M47; M55 put it on every report and triage, beside open
  code tasks by design)

```text
repro:
  setup     rig fresh; seed code; a single-task CT open; a report-inconsistency task authored
            a pre-commit hook behind core.hooksPath that appends '<!-- formatted -->' to each staged *.md and `git add`s it
            echo new > src/new.rs; git add src/new.rs          ; git diff --cached --name-status → A src/new.rs
  argv      jigc task finalize readme-disagrees-with-code --format json
  observed  exit 0 · manifest [promoted docs/inconsistencies/readme-mismatch.md]
            left_out [left-staged docs/inconsistencies/readme-mismatch.md, left-staged src/new.rs]
  after     git diff --cached --name-status → M docs/inconsistencies/readme-mismatch.md / A src/new.rs
            status: MM …/readme-mismatch.md ; the hook's line: HEAD 1 · index 0 · worktree 1
            jigc validate → exit 0 clean ; jigc task validate code-change-probe → exit 0
            jigc task finalize code-change-probe → exit 3 finalize.base-mismatch, route "resolve the overlap … against
              the new history, or discard the task" — no command that clears it
            git restore --staged -- docs/inconsistencies → the code task then lands
  control   the same hook on the ordinary model (park-idea): cached empty afterwards, index 1
  pressed   the phantom against seven ways to commit: five jigc doors refuse it (base-mismatch · carried-staged ·
            amend-index-dirty · rename.dirty-tree · carried-staged at the milestone boundary); --carry-staged commits
            it labelled carried-over ; plain `git commit` commits it. No byte is missing anywhere.
```

- `contract:` `design/command-output-contract.md` → *The M55 additive kind* (*"no committed path ever
  carries it"*); and the arm's stated property (`design/findings-channel.md` §3; the left-out
  narration's own *"a staged path stays staged for the task it belongs to"*) — the index is not what it
  was.
- `UNPINNED:` `crates/cli/tests/doc_only_finalize.rs` drives the arm under `jigc setup`'s own hook and
  under a rejecting one (`a_rejecting_hook_commits_nothing_and_leaves_the_index_alone`); no test runs a
  hook that `git add`s.

#### `(R5, F6)` · two more finalize routes end in a bare `jigc task finalize`

- `tier:` **2** — on the baseline's precedent (`(4, DEFECT 1)` was this class at tier 2); *"the weak end
  of tier 2 — the id is one token the reader holds"*
- `door:` `task finalize`
- `found-in:` `review:rc24-per-axis/(R5, F6)`
- `origin:` driver
- `new-code:` **outside the new code** — the two producers the `(4, DEFECT 1)` fix did not reach

```text
repro:
  setup     rig fresh; seed; a single-task CT; an UNSTAGED edit to src/tracked1.rs
  argv      jigc task finalize code-change-probe
  observed  exit 3 · finalize.nothing-staged — route: "`git add` your changes, then re-run `jigc task finalize`"
  argv      jigc task finalize                                                       (the route's argv, pasted)
  observed  exit 2 · "error: the following required arguments were not provided: <ID>"
  also      finalize.promote-clobber's route ends "…; then re-run `jigc task finalize`" (and carries an absolute
            host path — (R5, F7))
```

- `contract:` the route floor — a mechanical route runs as printed; `(4, DEFECT 1)`'s own closure.
- `UNPINNED:` the codes are reached by `crates/cli/tests/finalize_finding_keys.rs` and
  `dry_run_findings_equal_set.rs`; no test found that runs these two routes' argv.

#### `(R6, D-2)` · `write.identity-change`'s route, when the role is bound to a committed doc copied in, names a `doc rename` that refuses, whose own route is the whole-repo rename of the existing doc

- `tier:` **2**
- `door:` `doc create` (also `doc author`) → `doc rename` → `rename`
- `found-in:` `review:rc24-per-axis/(R6, D-2)`
- `origin:` driver
- `new-code:` **outside the new code** — the general twin reproduces under `park-idea`. It bounds an M55
  sentence: the VERDICT's *"the identity-change route under `new: true` was checked and runs"* holds for a
  doc the task minted and not for this arm

```text
repro:
  setup     rig fresh, inconsistency:port-mismatch landed. jigc start --workflow report-inconsistency "role binding probe"
            jigc doc set-field inconsistency:port-mismatch#meta/kind --value doc-doc --task role-binding-probe
              → exit 0 "(copied in for update …)" ; roles.json now binds the role to inconsistency:port-mismatch
  argv      jigc doc create inconsistency --title "A wholly new finding" --task role-binding-probe
  observed  exit 1 · write.identity-change — route: "`jigc doc rename inconsistency:port-mismatch --to 'A wholly new
            finding' --task …` moves the doc this task already holds onto the title (and id) you asked for"
  argv      (that route, verbatim)
  observed  exit 1 · write.identity-change — "rename rejected: … is committed" — route: "`jigc rename
            inconsistency:port-mismatch --to 'A wholly new finding'` moves it for real … once this task is finalized
            or discarded"
  after     hop 2 followed to its end: jigc rename … → exit 0 — the EARLIER reporter's finding now carries the new
            reporter's title (its prose intact). No create can succeed in the task; neither route says so.
```

- `contract:` `design/write-commands.md` → `jigc doc rename` (retitle-only for a committed copy) and its
  M55 sentence on the misroute F3 removed (*"that doc is someone else's work, so renaming it is the wrong
  correction"*).
- `UNPINNED:` `create_only_gate::under_new_the_identity_change_route_runs` pins
  the minted-doc arm and `a_committed_doc_copied_in_by_set_slot_is_refused` the refusal; neither runs
  this arm's route.

#### `(R6, D-3)` · two open report tasks mint one id: after the first lands, the second's `create.already-exists` route names an exit that refuses and an exit that destroys, and no surface names the one that works

- `tier:` **2**
- `door:` `doc create` (the route's producer) · `task finalize` (refuses)
- `found-in:` `review:rc24-per-axis/(R6, D-3)`
- `origin:` driver. It is also the state `(R6, K-1)`'s two-reporter variant leaves a task in
- `new-code:` **inside M55's code** — the bound-role arm of `create.already-exists`'s route

```text
repro:
  setup     rig fresh. Task A: report-inconsistency, doc create --title "Shared thing" → exit 0
            Task B: the same create → exit 0, existed false (the id is only staged elsewhere)
            A finalized → promoted docs/inconsistencies/shared-thing.md
  argv      jigc task validate cell-64-probe ; jigc task finalize cell-64-probe                     (task B)
  observed  exit 0 (one advisory) ; exit 3 finalize.base-mismatch — its route names no command
  argv      jigc doc create inconsistency --title "Shared thing" --task cell-64-probe
  observed  exit 1 · create.already-exists — route: "this task already holds `inconsistency:shared-thing` … land it
            with `jigc task finalize cell-64-probe`, or abandon it with `jigc task discard cell-64-probe --force`"
  after     the exit that works, named by no surface:
            jigc doc rename inconsistency:shared-thing --to "Shared thing" --slug shared-thing-2 --task cell-64-probe → 0
            jigc task finalize cell-64-probe → 0 ; both findings on disk, the landed one unchanged throughout
```

- `contract:` the route's premise — *"a distinct `--title` or a `--slug` would mint a second one beside
  it"* — is false in this state; `design/write-commands.md` → *The create-gate*.
- `UNPINNED:` `create_only_gate::a_bound_role_routes_an_occupied_id_at_the_next_task_not_a_distinct_identity` pins the route's
  wording; no test found with two open tasks minting one id.

### Tier 3 (52)

Grouped by row. Every entry carries the same fields as above; the repro blocks are shorter because the
defect is a sentence. *Why each is not tier 1 or tier 2* is argued in its row file's ledger and is not
repeated here except where a reconciler pressed it.

#### Row 1 — setup · the hook · install · release

##### `(R1, F2)` · `setup --force` over a dirty hook spends the consent without naming it

- `tier:` 3 · `door:` `setup` · `found-in:` `review:rc24-per-axis/(R1, F2)` · `origin:` driver
- `new-code:` **not established** — neither pass dates the hook member's place in the forced-path advisory

```text
repro:
  setup     bare rig; a foreign .githooks/pre-commit committed; git config core.hooksPath .githooks
            one uncommitted marked line appended to it                          ( M .githooks/pre-commit)
  argv      jigc setup --force --format json
  observed  exit 0 · {"findings": [], "hook_committed": true, "hook_file": ".githooks/pre-commit", …}
  after     git show HEAD:.githooks/pre-commit carries the marked line ; no setup.forced-install-path finding
  control   CLAUDE.md ALSO dirty → setup.forced-install-path "consented over 1 install path(s): `CLAUDE.md`" —
            two paths consumed, one named. Without --force: exit 1 setup.dirty-install-path names the hook
```

- `contract:` `design/validation.md` → the `setup.dirty-install-path` row — `--force` *"says so through the
  advisory `setup.forced-install-path` … naming every path the consent was spent on"*.
- `UNPINNED:` `crates/cli/tests/setup_install_pathspec_guard.rs` carries the advisory's tests; none found
  that forces over a dirty hook member.

##### `(R1, F3)` · a standalone jigc hook written with another `jigc` path, another build's template or one added line is wrapped as *foreign*, not regenerated; `uninstall` then leaves a jigc hook behind while saying it removed it

- `tier:` 3 · `door:` `setup` (also `uninstall`, the hook) · `found-in:` `review:rc24-per-axis/(R1, F3)` · `origin:` driver, widened by the reconciler
- `new-code:` **not established** — the hook template is byte-identical at the rc.22, rc.23 and rc.24 tags, so no published step triggers the template arm; the path arm is the live one

```text
repro:
  setup     fresh rig (hook: 65 lines, 1 start sentinel, line 9 jigc='~/.local/bin/jigc')
            BIN=$(mktemp -d "$RIG/cargo-bin.XXXXXX"); ln -s ~/.local/bin/jigc "$BIN/jigc"    (the same build, a second path)
  argv      "$BIN/jigc" setup
  observed  exit 0 · the summary lists the hook as installed
  after     129 lines · 2 start sentinels · line 9 the second path, line 73 the first (the old hook kept as "foreign")
            jigc uninstall --format json → exit 0, "precommit": true — a standalone jigc hook is STILL PRESENT
            jigc uninstall (again) → exit 0, "- removed pre-commit hook" ; the file is gone
  also      one comment line reworded, or one user line appended → the next setup wraps the whole file (2 sentinels)
            a committed hook met by a second path → the install commit adds 64 lines, rewrites nothing
            with 2 blocks a drift commit prints the warning twice
```

- `contract:` `design/assistant-adapter.md` — the hook install is *"idempotent (a sentinel-marked block,
  re-runnable, and **regenerated on every `setup`**)"*; *"that developer's own `jigc setup` **rewrites it
  to their path**"*; `jigc uninstall`'s own *"removed pre-commit hook"* / `"precommit": true`.
- `UNPINNED:` no test found that runs `setup` from a second path over an installed hook.

##### `(R1, F4)` · a `jigc` whose absolute path contains a single quote installs a hook whose quoting is broken: no drift warning, no rename block, and three `command not found` lines on every commit

- `tier:` 3 · `door:` `setup` (and the hook through `git commit`) · `found-in:` `review:rc24-per-axis/(R1, F4)` · `origin:` codex — reached by driving its *"quoted for spaces"*
- `new-code:` **not established**

```text
repro:
  setup     BQ=$(mktemp -d "$RIG/it's bin.XXXXXX"); ln -s ~/.local/bin/jigc "$BQ/jigc"
  argv      "$BQ/jigc" setup                                          (bare rig)
  observed  exit 0 · "- pre-commit hook → .git/hooks/pre-commit (warn-only doc↔code drift backstop)"
            line 9: jigc='$RIG/it's bin.…/jigc'
  after     any commit → exit 0 + ".git/hooks/pre-commit: line 9: blocking_probes: command not found" (×3 lines)
            drift an anchored symbol, commit → no "jigc: doc<->code drift" line (jigc validate: the drift is real)
            git mv docs/roadmap.md docs/plan.md; git commit → exit 0, HEAD MOVED, no block
  controls  a path with a SPACE survives (the warning prints) ; the same rename with a hook from
            ~/.local/bin/jigc → exit 1 "(commit blocked)" ; a wrapped foreign hook still rejects
```

- `contract:` `setup`'s summary (a backstop was installed); QUICKSTART (the hook *blocks* a commit that
  stages a bare `git mv` of a managed doc); the writer's own comment — *"quoted in the script so a path
  with spaces survives"*.
- `UNPINNED:` no test found installing from a path that carries a quote.

##### `(R1, F5)` · from a linked worktree the hook's sweep asks about the main checkout: drift committed in the worktree draws no warning and a bare `git mv` of a managed doc committed there is not blocked

- `tier:` 3 · `door:` `setup` (the hook it installs, through `git commit`) · `found-in:` `review:rc24-per-axis/(R1, F5)` · `origin:` codex — reached by driving *"honors `core.hooksPath` and worktrees"*
- `new-code:` **outside the new code** — *"the M53 jigc_home binding meeting the M35 hook, not something M54, M55 or the trailer changed"*

```text
repro:
  setup     vendored rig; git worktree add -q "$WT/tree" -b wt-branch; cd "$WT/tree"
  argv      (in the worktree) rename an anchored symbol; git add; git commit
  observed  exit 0 · 0 "jigc:" lines ; jigc validate there → exit 0, findings []
  argv      (committed-singletons rig, a linked worktree) git mv docs/roadmap.md docs/plan.md; git commit
  observed  exit 0 · HEAD MOVED · no "jigc:" line
  control   the same git mv + commit in a main checkout → exit 1 "(commit blocked)"
            inverse: drift committed in MAIN → the worktree's next commit prints the warning, about main's tree
```

- `contract:` QUICKSTART lists *"a linked worktree's shared hooks dir"* among the hook's homes and says it
  warns on drift and blocks a staged bare `git mv`; no doc read says what it does on a worktree commit.
- `UNPINNED:` `crates/cli/tests/precommit_hook_acceptance.rs` exists; no test found committing from a
  linked worktree.

#### Row 2 — probe integrity · the invocation log

##### `(R2, T-1)` · the probe's 30 s wall-clock budget is not enforced when the killed probe leaves a child holding its pipes

- `tier:` 3 · `door:` `validate` (the one door driven; the invoker is shared by all six) · `found-in:` `review:rc24-per-axis/(R2, T-1)` · `origin:` driver
- `new-code:` **not established** — reached on the **override** path only, by a probe that itself spawns a child; the bundled self-exec probe (M54) is one process and bounded

```text
repro:
  setup     vendored rig, an anchored symbol removed. Override stubs (mode 755):
            doc-code-spin  = while :; do :; done            doc-code-hang47 = sleep 47 (the shell + a child)
  argv      JIGC_DOC_CODE_PROBE=<tmp>/fx/doc-code-spin   jigc validate --format json
            JIGC_DOC_CODE_PROBE=<tmp>/fx/doc-code-hang47 jigc validate --format json
  observed  exit 1, wall 30.2 s ; exit 1, wall 47.2 s (the driver: 120 s for `sleep 120`)
            both: pack-probe-integrity.probe-failure · check timeout · "exceeded its time budget and was killed"
  after     no stub process left running ; nothing committed
```

- `contract:` `design/validation.md` → the six rules, 5 (*"A runaway probe is killed and surfaces as a
  `timeout` failure"*) and → Scope (*"a wall-clock budget the CLI invoker enforces"*); `invoke.rs`'s own
  *"killed and reaped (no leaked process, no hang)"*.
- `UNPINNED:` no test under `crates/cli/tests/` names the timeout message; a child that holds the pipes
  has no fixture.

##### `(R2, P-1)` · an override that names no file takes two different shapes at the task and milestone doors

- `tier:` 3 · `door:` `task validate` (also `task finalize`, `start`, `milestone finalize`) · `found-in:` `review:rc24-per-axis/(R2, P-1)` · `origin:` driver
- `new-code:` **not established** — the override path's pre-flight is what M54's probe move kept *(read)*; neither pass dates the two-shape split

```text
repro:
  setup     vendored rig; a task whose staged file is cited by a COMMITTED arch-doc, staging no anchored doc itself
  argv      JIGC_DOC_CODE_PROBE=<tmp>/fx/no-such-probe jigc --format json task validate rename-the-pad
  observed  exit 3 · findings: pack-probe-integrity.probe-failure · crash — "could not start … No such file or directory"
  setup     the task now also stages an anchored doc (one set-slot); nothing else changes
  argv      the same
  observed  exit 1 · stdout empty · stderr {"error": "`doc-code` probe not found at … — `JIGC_DOC_CODE_PROBE` names no file; …"}
  after     task finalize: exit 3 / exit 1 likewise, HEAD unmoved ; start: findings / findings_unavailable likewise
```

- `contract:` `design/validation.md` → Distribution bound (*"the one-operational-error shape is unchanged
  for that case"*); `design/assistant-adapter.md` (*case (iv) survives only on the override path*).
- `UNPINNED:` `crates/cli/tests/probe_failure_doors.rs` pins one finding per door; no test found holding
  the door fixed and varying the task's own anchor surface.

##### `(R2, L-1)` · orientation surfaces findings and its log record says `finding_codes: []`

- `tier:` 3 · `door:` `start` · `found-in:` `review:rc24-per-axis/(R2, L-1)` · `origin:` driver
- `new-code:` **outside the new code** *(read)* — the log's `finding_codes` scoping is `invocation_log.rs`'s own (*"the report-bearing verbs"*); the row's scope makes the field the observable for a probe that did not run

```text
repro:
  setup     vendored rig, jigc config set invocation-log true (committed); a live task whose staged file dangles a
            committed anchor
  argv      jigc --format json start
  observed  exit 0 · tasks[0].findings carries doc-code.symbol-exists (blocking)
  after     the record: {"argv":["--format","json","start"],"exit_code":0,"finding_codes":[], …}
            under a failing override: tasks[0].findings carries pack-probe-integrity.probe-failure ; the record: []
  control   jigc validate → record finding_codes ["doc-code.symbol-exists"] ; task validate likewise
```

- `contract:` `design/measurement.md` → *The in-repo invocation log* — *"`finding_codes` being `[]` on a
  run that raised none"*. Which text gives way is a triage decision.
- `UNPINNED:` `crates/cli/tests/invocation_log.rs` carries the log's tests; none found for orientation's
  record under a finding.

##### `(R2, L-2)` · `design/measurement.md` says `output_bytes` is the emitted **stdout** size; the binary records stdout **plus** stderr

- `tier:` 3 · `door:` `validate` (driven also at a clap usage error and `task finalize`) · `found-in:` `review:rc24-per-axis/(R2, L-2)` · `origin:` driver
- `new-code:` **outside the new code** — the field is M39's; a doc edit, not a code change

```text
repro:
  setup     vendored rig, the invocation log on
  argv      jigc bogus-verb ; jigc task finalize no-such-task ; jigc validate
  observed  exit 2, stdout 0, stderr 114 ; exit 1, stdout 0, stderr 200 ; exit 0, stdout 549, stderr 0
  after     the records' output_bytes: 114 · 200 · 549
```

- `contract:` `design/measurement.md` — *"the `output_bytes` field (M39) is the invocation's emitted
  stdout size"*. The binary agrees with its own module doc.
- `UNPINNED:` `crates/cli/tests/invocation_log.rs` carries the `output_bytes` assertions (the binary's
  behaviour); the design sentence is the unfenced half.

#### Row 3 — store exit codes / reconciliation

##### `(R3, F1)` · the strong-signal route's *adopt* exit cannot be run: `jigc rename <old-id>` answers `store.not-found`

- `tier:` 3 · `door:` `validate` (the route's producer; run at `rename`) · `found-in:` `review:rc24-per-axis/(R3, F1)` · `origin:` driver
- `new-code:` **not established** — the route text is rename detection's (M35); M55's L2 narrowed which arm blocks

```text
repro:
  setup     rig committed-singletons; git mv docs/roadmap.md docs/plan.md
  argv      jigc validate
  observed  exit 1 · reconciliation.rename — route: "adopt it as a CLI-owned rename …: `jigc rename roadmap:roadmap --to
            "<New Title>"`; or revert the move: `git -C $REPO mv docs/plan.md docs/roadmap.md`"
  argv      jigc rename roadmap:roadmap --to "Plan"
  observed  exit 1 · store.not-found — "no managed doc `roadmap:roadmap` to rename (expected at docs/roadmap.md)"
  after     the revert exit runs verbatim and clears the row. Same in two more states (a location doc staged; committed)
```

- `contract:` the finding's own route; `design/reconciliation.md` → *Rename detection* (*routing to the
  owned op first, revert second*); the trailer (*"or adopt it via `jigc rename`"*).
- `UNPINNED:` no test found that runs the adopt exit.

##### `(R3, F2)` · `jigc ingest` fails with a code-less, route-less OS error when any tracked `.md` is missing from the worktree

- `tier:` 3 · `door:` `ingest` · `found-in:` `review:rc24-per-axis/(R3, F2)` · `origin:` driver
- `new-code:` **not established** — not specific to managed docs (`rm README.md` reproduces it)

```text
repro:
  setup     rig committed-singletons; rm CHANGELOG.md                              (unstaged, uncommitted)
  argv      jigc ingest ; jigc ingest --format json
  observed  exit 1 · "could not read the candidate at "$REPO/CHANGELOG.md": No such file or directory (os error 2)"
            exit 1 · stdout empty · stderr {"error": "could not read the candidate at …"}
  after     .jigc/state/file-state.json byte-identical (cmp) — the whole triage is lost, not the one row
```

- `contract:` `jigc ingest --help` — *"Misplaced or non-conformant files are flagged for a human; exits
  0"*; the route floor (no code, no route).
- `UNPINNED:` no test found with a tracked markdown file missing from the worktree at `ingest`.

##### `(R3, F3)` · `write.non-reparseable` says *nothing was persisted* while the copy-in is persisted and changes the task's reconcile state

- `tier:` 3 · `door:` `doc set-slot` · `found-in:` `review:rc24-per-axis/(R3, F3)` · `origin:` driver
- `new-code:` **not established** — a write-surface seam reached from this row's L1 cell

```text
repro:
  setup     rig committed-singletons + a teammate who removes VISION.md's `## Thesis` heading and pushes; git pull
            jigc start --workflow single-task "sharpen the open questions"
  before    ls .jigc/tasks/<id>/docs → commit:<id>.md provenance.json ; task validate → reconciliation.conformance-block
  argv      jigc doc set-slot vision:vision#open-questions --from-file - --task <id>
  observed  exit 1 · write.non-reparseable — route: "nothing was persisted — revise the payload …"
  after     ls …/docs → + vision:vision.md (byte-equal to the on-disk doc) ; jigc doc list --task <id> lists it
            task validate → file-state.staged-copy + reconciliation.conflict-block ("this task's staged writes both
            changed it" — the task wrote nothing) ; the hand-repair route no longer clears the block
```

- `contract:` the finding's own route, which opens *"nothing was persisted"*.
- `UNPINNED:` no test found asserting the task area after a refused first write.

##### `(R3, F5)` · `jigc unmanage` matches its path literally and reports a managed doc as *not managed*

- `tier:` 3 · `door:` `unmanage` · `found-in:` `review:rc24-per-axis/(R3, F5)` · `origin:` driver
- `new-code:` **not established** — the verb is 2026-06's *(read)*; M55 added routes that name it. Not checked against numbered axis 1 (caller tokens)

```text
repro:
  setup     rig fresh, one ADR docs/decisions/alpha-cache.md landed ; the baseline: 1
  argv      jigc unmanage ./docs/decisions/alpha-cache.md      (also docs//…, …/x/../…, the absolute path, cwd-relative)
  observed  exit 0 · "no-op: ./docs/decisions/alpha-cache.md is not managed (nothing to drop)"
            --format json → {"path": "./docs/…", "identity": null, "dropped": false}
  after     the baseline: 1 (nothing dropped). The bare spelling → "unmanaged … (adr:alpha-cache)", baseline 0
```

- `contract:` the ack sentence itself — the doc at that path *is* managed; the miss is exit 0 by contract,
  so the sentence and `dropped` are the only discriminators a caller has.
- `UNPINNED:` no test found passing a non-canonical spelling of a managed path.

##### `(R3, F6)` · a compose-gate refusal leaves a minted live task, and the route's *then re-run* collides with it

- `tier:` 3 · `door:` `task amend` (also `start --workflow`, driver only) · `found-in:` `review:rc24-per-axis/(R3, F6)` · `origin:` driver
- `new-code:` **not established** — *"whether it predates the range was not established"*; the same under a whole-file shadow, so it is not F21's delta path. Same class as `(R4, F1)` at `migrate`

```text
repro:
  setup     rig fresh; jigc config insert-step --workflow amend --after amend-message <a step with a broken include>
            committed ; ls .jigc/tasks → (empty)
  argv      jigc task amend "repair the message"
  observed  exit 1 · workflow-refs.include-resolves — route: "fix the … definition the message names (…), then re-run"
  after     ls .jigc/tasks → repair-the-message ; the same argv again → exit 1 task.serial-collision
            jigc task discard repair-the-message --force → exit 0 clears it ; jigc workflow amend --preview mints nothing
```

- `contract:` the route (*then re-run*); `design/validation.md` on the sibling `workflow.verb-routed`
  (*raised before anything mints, so a refused name strands no task dir*).
- `UNPINNED:` no test found asserting `jigc task list` after a compose-time `workflow-refs` refusal.

#### Row 4 — pack-load / manifest freeze · migration

##### `(R4, F1)` · `jigc migrate` leaves a minted task behind a refusal that exits 1

- `tier:` 3 · `door:` `migrate` · `found-in:` `review:rc24-per-axis/(R4, F1)` · `origin:` driver
- `new-code:` **not established** — the retention is deliberate in the source (`migrate_in_repo`'s doc: *"leaves the minted task in place for inspection / re-entry"*); what is missing is the surface

```text
repro:
  setup     rig fresh --repin, one committed foreign file ; jigc task list → "no active tasks"
            mv "$JIGC_PACK_DIR/config/knobs.yaml" aside            (also: an unrelated malformed project workflow shadow)
  argv      jigc migrate foreign-adr.md --as adr
  observed  exit 1 · pack.resource-missing — names neither a task nor that anything was written
  after     ls .jigc/tasks → migrate-adr-foreign-adr-<hash> ; jigc task list → "1 active task(s)"
            (repair) the same argv → exit 1 task.serial-collision, whose route works
  control   jigc start --workflow quick-fix … under the identical fault → exit 1, mints nothing
```

- `contract:` `ensure_migratable`'s own discipline — *"a rejected migrate leaves `jigc task list`
  unchanged"*; `start`'s behaviour under the same fault.
- `UNPINNED:` no test found for `migrate` under a compose-time fault.

##### `(R4, F4)` · `jigc relocate --help`'s example doctype is one the door refuses

- `tier:` 3 · `door:` `relocate` · `found-in:` `review:rc24-per-axis/(R4, F4)` · `origin:` driver
- `new-code:` **outside the new code** — pre-existing since M40; M55 adds two more doctypes to the refused set

```text
repro:
  argv      jigc relocate --help
  observed  "<TYPE>  The freeze-exempt doctype id whose schema home moved (e.g. `vision`)" · "--from … (`docs/vision/`)"
  argv      jigc relocate vision --from docs/vision-old
  observed  exit 1 · relocate.frozen-doctype — "`vision` is a frozen doctype"
  after     every doctype `jigc describe --format json` lists (18): exit 1 relocate.frozen-doctype ×18
```

- `contract:` the help text against the binary; `design/corpus-migration.md` (*M40 … empties this path's
  domain*).
- `UNPINNED:` help examples are not driven by a test found.

##### `(R4, F5)` · `migrate-corpus` commits an uncommitted edit on a doc it migrates, un-narrated

- `tier:` 3 — *"closest to tier 1, and still not it"*: both plants are in `HEAD` and on disk afterwards
- `door:` `migrate-corpus` · `found-in:` `review:rc24-per-axis/(R4, F5)` · `origin:` driver
- `new-code:` **not established** — the range's one change on this seam (`28ca86b4`, the trailer) moved the commit message into the signing seam and left the pathspec alone

```text
repro:
  setup     rig fresh, both findings docs filed; the stamp stripped from one and committed with git
            on that doc: one edit staged (PLANT-A), a second unstaged (PLANT-B) ; an unrelated staged file and an
            unrelated dirty README beside it ; git show HEAD:<doc> | grep -c PLANT → 0
  argv      jigc migrate-corpus --dry-run ; jigc migrate-corpus
  observed  exit 0 "1 would migrate …" (says nothing of the edits)
            exit 0 "1 migrated … committed <sha> — only the migrated paths were staged"
  after     git show HEAD -- <doc> → +schema-version: 1 · +PLANT-A · +PLANT-B ; the unrelated two untouched
  pressed   a destination already occupied is REFUSED (migrate-corpus.destination-collision, the occupant unchanged)
```

- `contract:` `jigc migrate-corpus --help` — *"re-parses every **committed** doc … folds the
  deterministic diff … into its bytes"*; the ack (*"only the migrated paths were staged"*).
- `UNPINNED:` no test found migrating a path that carries an uncommitted edit.

##### `(R4, C1)` · a filesystem pack's freeze manifest that vanishes between the enumeration read and the second read is skipped without a word; the drifted schema then loads, and `migrate-corpus` commits

- `tier:` 3 — the Codex pass's *"potentially tier 1 … through `migrate-corpus`"* is refuted: the commit's whole diff is the stamp
- `door:` `migrate-corpus` (also every pack-loading door; driven at `validate`) · `found-in:` `review:rc24-per-axis/(R4, C1)` · `origin:` codex
- `new-code:` **not established** — the source comment at the skip calls it deliberate. The embedded packs are immune

```text
repro:
  setup     rig fresh --repin (a dev pack copy with its manifest) + a listed filesystem copy of the methodology pack
            an adr filed; the dev copy's adr.yaml drifted (one enum member), its manifest NOT re-pinned
  control   jigc validate → exit 1 "pack-load freeze check failed: doctype `adr`: schema-hash mismatch"
  argv      ( sleep 1.5 ; mv <pack>/config/schema-manifest.yaml aside ) &  jigc validate      (window widened; traced)
  observed  exit 0 — the second read finds no file and the loop continues ; validation runs
  argv      the same race with the manifest restored before the door's own read:  jigc migrate-corpus
  observed  exit 0 · "1 migrated … committed <sha>" ; the diff: -schema-version: 2 / +schema-version: 3, nothing else
  after     the file back: every door blocks again ; re-pinned: clean. Unpadded packs: the race won 4 of 90 runs
```

- `contract:` the pack factory's — a drifted frozen schema *blocks every door*; under this fault it does
  not and no surface says the freeze was skipped. (Deleting the manifest outright is the designed
  opt-out, and at `validate` prints `schema-conformance.unversioned-doctype`.)
- `UNPINNED:` a two-read race has no deterministic fence. **The reconciler's instrument is its own**: an
  observational `open()` tracer loaded into the unmodified binary for these drives, and a padded pack to
  widen the window (→ HONEST BOUNDS).

#### Row 5 — finalize / transaction

##### `(R5, F2)` · a path git C-quotes reaches the manifest as the quoted string, and the forecast then disagrees with the door

- `tier:` 3 · `door:` `task finalize` (`--dry-run`) · `found-in:` `review:rc24-per-axis/(R5, F2)` · `origin:` driver
- `new-code:` **outside the new code** in origin (the ordinary model has it); M55's `left-staged` list and the doc-only forecast inherit it

```text
repro:
  setup     git status --short → A  "src/with space.rs"
  argv      jigc task finalize code-change-probe --dry-run --format json ; then without --dry-run
  observed  exit 0 · manifest path "\"src/with space.rs\"" ; exit 0 · landed manifest path "src/with space.rs"
  also      DocOnly, a recorded artifact "my cap.md": the forecast lists it left_out: untracked (quotes in the JSON
            string); the door commits it — manifest [added …/my cap.md, promoted …]
```

- `contract:` `design/command-output-contract.md` — *"one classification serves the forecast and the
  landed set, so the two stay identical by construction"*.
- `UNPINNED:` no test found with a staged name git quotes.

##### `(R5, F3)` · on the ordinary model the forecast lists an untracked recorded owner-artifact as left out, and the door commits it

- `tier:` 3 · `door:` `task finalize` (`--dry-run`) · `found-in:` `review:rc24-per-axis/(R5, F3)` · `origin:` driver
- `new-code:` **outside the new code** — M45's staging arm; the doc-only model forecasts the same artifact correctly

```text
repro:
  setup     a project workflow granting dogfood-record, ending step:finalize ; a recorded artifact cap.md untracked,
            other.md staged, third.md untracked
  argv      jigc task finalize glob-run --dry-run --format json ; then without --dry-run
  observed  exit 0 · manifest [added …/other.md, promoted …] · left_out [untracked …/cap.md, untracked …/third.md]
            exit 0 · manifest [added …/cap.md, added …/other.md, promoted …] · left_out [untracked …/third.md]
```

- `contract:` as `(R5, F2)` — forecast ≡ door.
- `UNPINNED:` `crates/cli/tests/doc_only_finalize.rs` pins the owner-artifact cells on both arms for a
  glob byte; no test found comparing forecast to door for an untracked recorded artifact on the
  ordinary arm.

##### `(R5, F4)` · a `git rm --cached` path appears twice in the doc-only left-out list

- `tier:` 3 · `door:` `task finalize` · `found-in:` `review:rc24-per-axis/(R5, F4)` · `origin:` driver
- `new-code:` **inside M55's code** — the doc-only arm's left-out classification

```text
repro:
  setup     beside a report task: git rm -q --cached src/c.rs      (status: "D  src/c.rs" and "?? src/c.rs")
  argv      jigc task finalize readme-disagrees-with-code --dry-run --format json
  observed  exit 0 · left_out carries {left-staged, src/c.rs} AND {untracked, src/c.rs}; the text arm prints it twice
  after     the landed commit is the doc alone ; the index byte-identical
```

- `contract:` `design/command-output-contract.md` — *"one entry per path, whichever change it stages"*.
- `UNPINNED:` `doc_only_finalize::every_flow_b_state_forecasts_the_path_set_and_narrates_what_stays`
  pins the left-out kinds per staging state; none found with a path carrying two porcelain lines.

##### `(R5, F5)` · `finalize.stage-failed` says the promotions were rolled back and the same re-run lands, beside a `finalize.rollback-conflict` that says one was not

- `tier:` 3 · `door:` `task finalize` · `found-in:` `review:rc24-per-axis/(R5, F5)` · `origin:` driver
- `new-code:` **not established** — both models; the compare-and-swap itself is correct

```text
repro:
  setup     a triage task; a REQUIRED git clean filter bound to docs/** that appends a line to the file and exits 1
            (git's own mechanism, run inside `git add` — the driver's instrument, not in the fixture library)
  argv      jigc task finalize triage-crate-count
  observed  exit 3 · finalize.stage-failed — "… no commit was made and the promotions were rolled back …" route:
            "… the task survives intact, so the same re-run lands the commit"
            + finalize.rollback-conflict — "… changed while this finalize was running, so the rollback did not restore it"
  after     HEAD unmoved · index identical · the pre-image parked ; filter removed, "the same re-run" → exit 3
            reconciliation.conflict-block (report shape: finalize.promote-clobber)
```

- `contract:` the two findings contradict on one surface; the hook-rejection frame has an *"apart from the
  N path(s) named below"* opener for exactly this and the stage-failure frame does not.
- `UNPINNED:` no fixture fails `git add` with the worktree concurrently edited.

##### `(R5, F7)` · absolute host paths inside the repository, in a route and in a message

- `tier:` 3 · `door:` `task finalize` · `milestone provision` · `found-in:` `review:rc24-per-axis/(R5, F7)` · `origin:` driver
- `new-code:` **outside the new code** — the class of `(4, DEFECT 2)`, two more producers

```text
repro:
  argv      jigc task finalize readme-disagrees-with-code           (a file already at the promote destination)
  observed  exit 3 · finalize.promote-clobber — route: "… `jigc migrate <absolute host path>/repo/docs/inconsistencies/
            readme-mismatch.md --as <doctype>` …"
  argv      jigc milestone provision prov-probe                     (.git/worktrees a regular file)
  observed  exit 1 · milestone.provision-failed — "`git worktree add --detach <absolute host path>/repo/.jigc/worktrees/
            prov-sub-one <sha>` failed" ; at: .jigc/worktrees/prov-sub-one   (the repo-relative spelling is in hand)
```

- `contract:` the class of `(4, DEFECT 2)` — a path inside the repository rendered host-absolute while the
  repo-relative spelling is in hand in the same finding. The declared *pasteable shell bytes* rule covers
  `git -C <absolute>` only and is **not** part of this finding.
- `UNPINNED:` no test found for these two producers' path spelling.

##### `(R5, F8)` · `milestone provision` failing after its `ensure` leaves the `.jigc/.gitignore` amend on disk and no run ever names it

- `tier:` 3 · `door:` `milestone provision` · `found-in:` `review:rc24-per-axis/(R5, F8)` · `origin:` driver — the Codex pass independently named this drive
- `new-code:` **outside the new code** — the baseline's open lead, now driven; the sibling of `(4, DEFECT 3)`

```text
repro:
  setup     .jigc/.gitignore trimmed to 'tasks/' + one user line and committed ; : > .git/worktrees
  argv      jigc milestone provision prov-probe
  observed  exit 1 · milestone.provision-failed
  after     git status → " M .jigc/.gitignore" (six lines appended) ; "gitignore" on stdout 0, on stderr 0
            the regular file removed, re-run → exit 0 "provisioned 2 worktree(s) …" — the amend is acked by neither run
```

- `contract:` the ack every door that ensures the ignore file owes (the control prints *".jigc/.gitignore →
  appended index/, state/, …"*); here a union amend of a tracked file survives a failed door and no run
  names it.
- `UNPINNED:` `crates/cli/tests/gitignore_amend_union.rs` exists; no test found failing `provision`
  after the `ensure`.

##### `(R5, F9)` · a rolled-back `config set docs-root` leaves the new root's empty directories behind

- `tier:` 3 · `door:` `config set` · `found-in:` `review:rc24-per-axis/(R5, F9)` · `origin:` driver
- `new-code:` **outside the new code** — seen on `(4, DEFECT 2)`'s repro

```text
repro:
  setup     a landed inconsistency doc ; the config layer unwritable (not root)
  argv      jigc config set docs-root documentation --format json
  observed  exit 1 · config.repoint-failed — route: "`docs-root` is unchanged and every doc this re-point moved is back
            at its prior home …"
  after     git status --short → (empty) ; find documentation → documentation · documentation/inconsistencies
```

- `contract:` `design/finalize.md` → Rollback discipline — *"nothing jigc wrote inside a transaction
  survives … and every such survival is named"*.
- `UNPINNED:` `crates/cli/tests/config_layer_preimage.rs` exists; no test found asserting the absence
  of the new root after a rolled-back re-point.

##### `(R5, F10)` ⊂ `(R9, F1)` · `task finalize --help` and `task validate --help` describe two commit models; the binary has three

- `tier:` 3 · `door:` `task finalize` (also `task validate`) · `found-in:` `review:rc24-per-axis/(R5, F10)` · `origin:` driver
- `new-code:` **inside M55's code** — M55 added `DocOnly` and the help was not swept

```text
repro:
  argv      jigc task finalize --help | grep -c -i 'doc-only\|path-scoped\|path scope'
            jigc task validate --help | grep -c -i 'doc-only\|path-scoped\|path scope'
  observed  0 ; 0 — the help calls the amend arm "its second commit model" and stops
```

- `contract:` the help against `CommitModel` (3). The fuller statement, with the guides, is `(R9, F1)`.
- `UNPINNED:` `crates/cli/tests/task_amend.rs` and `rejection_frame_outcome.rs` carry the phrase
  *second commit model* — they fence the help as it stands.

#### Row 6 — write surface

##### `(R6, D-4)` · `write.title-ignored` arises under a `new: true` entry, which `findings-channel.md` says it cannot

- `tier:` 3 · `door:` `doc create` (also `doc author`) · `found-in:` `review:rc24-per-axis/(R6, D-4)` · `origin:` driver
- `new-code:` **inside M55's code** — the design-of-record sentence is M55's; the binary does what `write-commands.md` states

```text
repro:
  setup     a report-inconsistency task; jigc doc create inconsistency --title "Own thing" --task <t> → exit 0
  argv      jigc doc create inconsistency --title "Own Thing!" --task <t> --format json
  observed  exit 1 · (write.title-ignored, inconsistency:own-thing) — route: `jigc doc rename … --to 'Own Thing!' --task <t>`
  after     the route runs (exit 0, retitled) ; the same write then acks "(already existed — copied in for update)"
```

- `contract:` `design/findings-channel.md` §10 (*"reachable only under an entry without `new: true`"*) and
  §4 (*"Inside a report task this arm cannot arise"*); `design/worked-examples.md` flow 57.
- `UNPINNED:` `create_only_gate::a_different_title_onto_the_same_id_is_the_gate_refusal_not_title_ignored`
  pins the on-disk arm; the staged-only arm driven here is not asserted by a test found.

##### `(R6, D-5)` · the bare `allows-create` entry form `write-commands.md` documents is refused at load

- `tier:` 3 · `door:` `start` · `found-in:` `review:rc24-per-axis/(R6, D-5)` · `origin:` driver
- `new-code:` **not established** — `workflow-dialect.md` agrees with the binary, so two designs disagree with each other

```text
repro:
  setup     a project shadow of report-inconsistency whose entry is the bare form:  allows-create: [ - inconsistency ]
  argv      jigc start --workflow report-inconsistency "probe bare form"
  observed  exit 1 · workflow-refs.malformed-front-matter — "allows-create[0]: invalid type: string "inconsistency",
            expected struct AllowsCreate" ; { type: inconsistency, new: true } → "missing field `as`"
```

- `contract:` `design/write-commands.md` → *The create-gate* → *Two entry forms*.
- `UNPINNED:` no test found loading the bare form.

##### `(R6, D-6)` · the singleton arm of `create.already-exists` routes at `jigc task discard <id>`, which is refused as written

- `tier:` 3 · `door:` `doc create` (also `doc author`; run at `task discard`) · `found-in:` `review:rc24-per-axis/(R6, D-6)` · `origin:` driver
- `new-code:` **inside M55's code** — the route's producer; reachable only under a project shadow that puts `new: true` on a singleton's entry (no shipped workflow does)

```text
repro:
  setup     rig committed-singletons; a shadow giving form-vision's entry new: true, committed
  argv      jigc doc create vision --title Vision --task vision-route-check
  observed  exit 1 · create.already-exists — route: "… If this task holds nothing else, `jigc task discard
            vision-route-check` abandons it"
  argv      jigc task discard vision-route-check                                   (the route, verbatim)
  observed  exit 1 · task-discard.staged-prose (the task stages its commit doc) — the next route names --force, which works
```

- `contract:` the route (*"abandons it"*).
- `UNPINNED:` no shipped workflow reaches the arm; no test found running its route.

##### `(R6, D-8)` = `(R4, F2)` · the unknown-key load error names the key and not the workflow

- `tier:` 3 by row 6's reconciler · **2 by row 4's** — the full entry is under Tier 2, `(R4, F2)`
- `door:` `start` · `found-in:` `review:rc24-per-axis/(R6, D-8)` · `origin:` driver
- `new-code:` as `(R4, F2)`

```text
repro:
  as (R4, F2), on report-inconsistency; row 6 adds: a report task minted BEFORE the typo is refused at doc create,
  doc author, task validate, task finalize --dry-run and start --task (exit 1 each), while its edit leaves still
  exit 0 ; `jigc milestone add-task … --workflow report-inconsistency` exits 0 and commits a record row for a
  sub-task no door can compose
```

- `contract:` as `(R4, F2)`.
- `UNPINNED:` as `(R4, F2)`.

##### `(R6, D-9)` · general case: `write.identity-change`'s route names a `doc rename` onto an **occupied** committed id, which refuses

- `tier:` 3 · `door:` `doc create` (also `doc author`; run at `doc rename`) · `found-in:` `review:rc24-per-axis/(R6, D-9)` · `origin:` driver
- `new-code:` **outside the new code** — under `new: true` M55 closed it by ranking `create.already-exists` first; the entries without the key keep it

```text
repro:
  setup     a park-idea task holding idea:mine ; idea:parked-thought committed
  argv      jigc doc create idea --title "A Parked Thought" --task route-check-rg1
  observed  exit 1 · write.identity-change — route: `jigc doc rename idea:mine --to 'A Parked Thought' --task …`
  argv      (that route)
  observed  exit 1 · write.already-present — its route (`… --slug <other-slug>`) is sound and lands
```

- `contract:` the first route states a move the binary refuses.
- `UNPINNED:` `crates/cli/tests/doc_rename_in_task.rs` exists; no test found running this route onto an
  occupied id.

##### `(R6, D-10)` · under a project shadow that drops `new: true`, the composed step still says the create is refused

- `tier:` 3 · `door:` `start` · `found-in:` `review:rc24-per-axis/(R6, D-10)` · `origin:` driver
- `new-code:` **inside M55's code** — the pack step's sentence; needs a deliberate project-layer edit

```text
repro:
  setup     a committed shadow of report-inconsistency with the entry { type: inconsistency, as: inconsistency }
  argv      jigc start --workflow report-inconsistency "compose under dropped key"
  observed  exit 0 · lines 8–10: "This task files exactly one NEW record. A title whose slug a doc on disk already holds
            is refused `create.already-exists`, with nothing staged …"
  after     jigc doc create inconsistency --title "Port mismatch" --task … → exit 0, existed true (copied in)
```

- `contract:` the composed step's own sentence.
- `UNPINNED:` `create_only_gate::without_new_the_create_or_update_is_unchanged`
  pins the behaviour; the composed text under the shadow is not asserted by a test found.

#### Row 7 — pinned read contracts

##### `(R7, D-1)` · the two triage steps say `jigc doc list <type>` lists each record *"by its slug and its `status`"*; the command as printed lists no status

- `tier:` 3 · `door:` `doc list` (the sentence is emitted by `start` composing the two triage workflows) · `found-in:` `review:rc24-per-axis/(R7, D-1)` · `origin:` driver
- `new-code:` **inside M55's code** — the pack steps `author-inconsistency-triage`, `author-jigc-feedback-triage`. Filed once, here; row 8 did not count it again

```text
repro:
  setup     rig fresh; one inconsistency landed
  argv      jigc start --workflow triage-inconsistency "triage the retry inconsistency"
  observed  exit 0 · "… The committed records, each by its slug and its `status`:" / jigc doc list inconsistency
  argv      jigc doc list inconsistency
  observed  exit 0 · "id  path  state" — `state` is the registration state (`managed`), not the doc's `status`
  after     jigc doc list inconsistency --format json → .docs[0].fields.status = "open" (the only arm that carries it)
```

- `contract:` the step's own sentence.
- `UNPINNED:` the compose goldens fence the sentence as written —
  `crates/cli/tests/goldens/compose/methodology/start--triage-inconsistency--migrated.txt` and its
  siblings; no test asserts the listing prints a status.

##### `(R7, D-2)` · on `--task`, the fragment-miss routes tell the reader to look at the **committed** doc, including for a doc that has no committed copy

- `tier:` 3 · `door:` `doc show` · `found-in:` `review:rc24-per-axis/(R7, D-2)` · `origin:` driver
- `new-code:` **not established** — rc.20 did not drive these codes on `--task`; no older binary installed

```text
repro:
  setup     a report-inconsistency task; doc create "Two docs disagree"; one item added — nothing is committed
  argv      jigc --format json doc show 'inconsistency:two-docs-disagree#sides/nope' --task second-open-task
            jigc --format json doc show 'inconsistency:two-docs-disagree#meta/nope'  --task second-open-task
  observed  exit 1 · store.no-such-item — route "name an item that exists in the committed doc"
            exit 1 · store.no-such-leaf — route "name a field the committed section carries (…)"
  control   store.no-such-section on the same arm carries an arm-neutral route
```

- `contract:` `design/doc-read-surface.md` → *What it reads* — the rule it states for `store.unparseable`:
  *"the shared tail claiming a staged doc is committed would be a lie"*.
- `UNPINNED:` no test found driving the three fragment-miss codes on the staged arm.

##### `(R7, D-3)` · a `doc list` row prints an `id` that `doc show` refuses, and the refusal routes back to `doc list`

- `tier:` 3 · `door:` `doc list` · `found-in:` `review:rc24-per-axis/(R7, D-3)` · `origin:` driver
- `new-code:` **outside the new code** — the identity leg is M50's

```text
repro:
  setup     a stamped, conformant inconsistency written by hand at docs/inconsistencies/Not_A_Slug.md, committed with git
  argv      jigc doc list --format json
  observed  exit 0 · {"id":"inconsistency:Not_A_Slug", …, "state":"unregistered", …}
  argv      jigc --format json doc show 'inconsistency:Not_A_Slug'
  observed  exit 1 · store.malformed-slug — route: "`jigc doc list` lists the committed docs and the identity each one carries …"
  after     jigc ingest says it itself: ingest.unaddressable-identity — "its name is not a doc id", with a `git mv` route
```

- `contract:` `jigc doc list --help` — *"Every instance … carries its `<type>:<slug>` identity"*;
  `design/doc-read-surface.md` on the orphan row — *"never a synthesized `<type>:<slug>`, which would be
  an address `doc show` refuses"*.
- `UNPINNED:` `ingest.unaddressable-identity` is pinned at `ingest` (`crates/cli/tests/ingest.rs`); the
  `doc list` row's `id` for such a file is not asserted by a test found.

##### `(R7, C-1)` · a body-field `default:` is projected into `fields`; `findings-channel.md` and `doc list --help` say *header* fields

- `tier:` 3 — *"the weakest kind of 3"*: a doc ↔ doc disagreement plus one help phrase
- `door:` `doc show` · `doc list` · `found-in:` `review:rc24-per-axis/(R7, C-1)` · `origin:` codex
- `new-code:` **inside M55's code** — the default projection (S10). **Unreachable with the shipped packs**: none of the 18 doctypes declares a body field group

```text
repro:
  setup     rig fresh --pack-from-dev --schema adr <the shipped adr.yaml + a BODY field group: visibility (default
            public), audience> ; an adr landed ; its `- visibility:` line hand-deleted and committed
  argv      jigc doc show adr:body-default-probe --format json ; jigc doc list adr --format json
  observed  exit 0 · fields {"audience":"operators", …, "visibility":"public"} at both doors, committed and --task
  after     the #context slice omits it ; #context/visibility → exit 1 store.no-such-leaf ; the bytes untouched
```

- `contract:` `design/findings-channel.md` §5 / §10 (*a defaulted **header** field*) and
  `jigc doc list --help` (*"`fields` its **header** fields"*). **Not contradicted:**
  `design/doc-read-surface.md`, which states the driven behaviour. May equally be filed as an
  `inconsistency` between the two design docs.
- `UNPINNED:` no shipped doctype reaches it.

#### Row 8 — composed surfaces

##### `(R8, F-1)` · a `squash=true` sub-task composed from `implement-from-spec` is still told to write and read back its commit doc, and the reason it is given is not something the binary does

- `tier:` 3 · `door:` `workflow` · `found-in:` `review:rc24-per-axis/(R8, F-1)` · `origin:` driver
- `new-code:` **inside M55's code** — the S2 omission (§6) derives the commit-doc clause from catalog refs and does not see the step's literal; the step sentence itself predates the range

```text
repro:
  setup     a spec committed through the plan workflow ; finalize.fan-out.squash = true (pack-default)
            jigc milestone create "Spec run" ; add-task spec-run "impl sub" --workflow implement-from-spec ; provision ; execute
  argv      (the Spawn line as printed)
  observed  exit 0 · ":41 Wire this task's commit to the spec it implements, so the landed record links back to what it
            built." · ":45 jigc doc set-field commit:impl-sub#implements --value spec:<slug> --task impl-sub"
  after     followed (bind, set-field, doc show: "implements: spec:greeting-spec"); milestone join / finalize → 0 / 0
            the landed message: "Finalize milestone spec-run (1 sub-task)" ; greeting-spec in message 0, in diff 0
  control   top-level, the same workflow: the landed message carries no mention of the spec either
```

- `contract:` `design/findings-channel.md` §6 (S2) — *"the omission removes every false line, not only
  `Run:`"*.
- `UNPINNED:` the compose goldens fence the step sentence as written —
  `crates/cli/tests/goldens/compose/dev/start--implement-from-spec--fresh.txt` and siblings (top-level
  compositions); the sub-task text is fenced by `crates/cli/tests/sub_task_composition.rs`, which does
  not see a literal.

##### `(R8, F-2)` · two shipped steps say `jigc task finalize` in a sub-task *"lands a commit on this worktree's detached HEAD"*; the binary refuses it

- `tier:` 3 (low) · `door:` `workflow` (the text; falsified at `task finalize`) · `found-in:` `review:rc24-per-axis/(R8, F-2)` · `origin:` driver
- `new-code:` **outside the new code** — the step text predates the range; M55's E2 fence classes the *never* phrasing as correct. The other half of the same sentence is the STILL-OPEN `(6, D-2)`

```text
repro:
  argv      (a sub-task composed from `sub-task`, and one from `fix-task`)
  observed  "… never `git commit` and never `jigc task finalize` here — either lands a commit on this worktree's
            detached HEAD and strands the sub-task's work outside the milestone boundary."
  argv      jigc task finalize s-sub-task                           (from its worktree, a file staged)
  observed  exit 3 · finalize.milestone-sub-task ; HEAD unmoved ; the staged path still staged
  control   orientation, same binary: "`jigc task finalize <sub>` refuses here"
```

- `contract:` `design/workflow-dialect.md` → *Emitted format* — *"`jigc task finalize <sub>` refuses
  (`finalize.milestone-sub-task`)"*.
- `UNPINNED:` the sentence lives in `crates/cli/packs/dev/steps/sub-task-commit.yaml` and the
  methodology `fix-finding` step; no test found asserting the reason clause.

##### `(R8, F-3)` · `jigc milestone add-task --workflow` accepts every loaded workflow, so a verb-routed or mints-nothing workflow is composed off its one declared door through `jigc workflow <W> --task <sub>`

- `tier:` 3 · `door:` `milestone add-task` (accepts) · `workflow` (composes) · `found-in:` `review:rc24-per-axis/(R8, F-3)` · `origin:` driver
- `new-code:` **outside the new code** — `--workflow` on `add-task` is M8's. M51 `A6-2` is CLOSED over three argv forms; this is a fourth

```text
repro:
  argv      jigc milestone add-task sweep-all "s migrate-adr" --workflow migrate-adr     (and each of the 39 workflows)
  observed  exit 0 ×39 — the 15 `suppressed.door` workflows and the 4 `creates-task: false` ones included
  argv      (the s-migrate-adr Spawn line as printed)
  observed  exit 0 · "… Below is the foreign source the CLI staged for you …" followed by nothing
  argv      (the s-milestone-execution Spawn line)
  observed  exit 0 · "Run: `jigc milestone provision <MILESTONE_ID>`" — an unfilled operand, 0 Spawn lines
  control   jigc start --workflow <X> · start --workflow <X> "an intent" · workflow <X> --preview → exit 1
            workflow.verb-routed ×9, whose own message says "composes a degenerate walk … off-verb"
  pressed   a migrate-adr sub-task followed to the boundary: finalize exit 0, one doc promoted, the source untouched
```

- `contract:` `jigc describe --workflows` — *"that verb binds what its steps read, so it is the one door
  that composes it, and both of these refuse it by name"*; `design/workflow-dialect.md` → *No nested
  `fan-out`*.
- `UNPINNED:` the three refused forms are fenced; no test found adding a verb-routed workflow as a
  sub-task's mint workflow.

##### `(R8, F-4)` · the declared bound is wider than its sentence: orientation reports the omitted commit doc as blocking findings on every `squash=true` sub-task

- `tier:` 3 — pressed toward tier 1 by its reconciler (→ headline): the bytes a blocking route asks for are discarded by the boundary, as `squash: true` declares
- `door:` `start` · `found-in:` `review:rc24-per-axis/(R8, F-4)` · `origin:` driver
- `new-code:` **inside M55's code** — the difference against M55's declared bound, which HOLDS at the door it names

```text
repro:
  setup     squash true (pack-default); a milestone with a dev-task and a report-inconsistency sub-task, both entered
            (the composed texts ask for no commit doc)
  argv      jigc start ; jigc start --format json
  observed  exit 0 · "Active task: code-sub … findings: 2 blocking" — schema-conformance.field-value-conformant and
            .required-slot-present at commit:code-sub, each with a route to author it ; the same per sub-task ;
            on the wire: tasks[i].findings = the two codes
  after     both routes run, a marked subject authored: task validate → 0 ; milestone join / finalize → 0 / 0 ;
            the marker in commit messages 0 · tracked files 0 · under .jigc/ 0
```

- `contract:` `completions/artifacts/M55/VERDICT.md` → *Declared bounds* names one door
  (`jigc task validate <sub>`); `design/workflow-dialect.md` → *Composing for a fan-out sub-task* — the
  boundary *"reads no sub-task's commit doc"*.
- `UNPINNED:` no test found asserting orientation's findings for a `squash=true` sub-task.

##### `(R8, C-1)` · a project-layer step that names the per-task door as a literal survives sub-task composition

- `tier:` 3 · `door:` `workflow` · `found-in:` `review:rc24-per-axis/(R8, C-1)` · `origin:` codex (= the driver's observation O-2, promoted by a repro)
- `new-code:` **inside M55's code** — the S2 derivation classifies *by the catalog entry*, so a literal is outside it by its declared mechanism. A candidate for a written bound rather than a code change

```text
repro:
  setup     jigc milestone add-task lit-run "park sub" --workflow park-idea ; jigc config fork workflow:park-idea#author-idea
            one line appended to that tracked shadow: "… run `jigc task finalize {{task.id}}` to commit it."
  argv      (the Spawn line as printed)
  observed  exit 0 · ":47 … run `jigc task finalize park-sub` to commit it." three lines above a trailer naming
            `jigc milestone finalize lit-run` as the only commit boundary ; the pack's own finalize step is omitted
  after     jigc task finalize park-sub → exit 3 finalize.milestone-sub-task (routed) ; HEAD unmoved
```

- `contract:` `design/findings-channel.md` §6 — no line naming the refused door; it is **not** one of
  §6's two declared bounds.
- `UNPINNED:` `crates/cli/tests/sub_task_composition.rs` — the E2 fence's own doc comment lists *"a
  project-layer step shadow, which no shipped-pack scan can see"* among what it misses.

#### Row 9 — adopter docs & help

##### `(R9, F1)` ⊃ `(R5, F10)` · the commit boundary's help and both guides state the carryover gate as a universal; the doc-only commit model is on none of them

- `tier:` 3 · `door:` `task finalize` (also `task validate`) · `found-in:` `review:rc24-per-axis/(R9, F1)` · `origin:` driver
- `new-code:` **inside M55's code** — the unswept siblings of surfaces M55 did sweep (the composed step, the left-out narration, MIGRATING gate 1)

```text
repro:
  setup     rig fresh; printf 'port = 9090\n' > server.conf ; git add server.conf        (BEFORE the mint)
            jigc start --workflow report-inconsistency "…" ; the finding and its commit doc authored
  argv      jigc task finalize readme-says-the-port --dry-run
  observed  exit 0 · "would commit … promoted docs/inconsistencies/port-default-disagrees.md / left-out (…): server.conf"
  argv      jigc task finalize readme-says-the-port --carry-staged
  observed  exit 0 · 1 file committed ; git status → "A  server.conf" still — the flag landed nothing
  control   a single-task task, same rig, same staged path: task validate → exit 3 finalize.carried-staged
```

- `contract:` `jigc task finalize --help` (*"its **second commit model**"* · `--carry-staged` *"inert on
  an **amend** task in every state"* · `--dry-run` *"the carryover gate … (exit 3) instead of the
  manifest"*); QUICKSTART §3 (*"A change staged before the task existed refuses to ride the commit"*);
  MIGRATING gate 4 (*"run the same probe and refuse the same states"*).
- `UNPINNED:` as `(R5, F10)` — the help is fenced as it stands; the guide sentences are unfenced
  against the doc-only arm.

##### `(R9, F2)` = `(R10, F5)` · the composed `amend` step says the amended message carries only the trailers you author; it carries the co-author trailer you did not

- `tier:` 3 · `door:` `task amend` · `found-in:` `review:rc24-per-axis/(R9, F2)` · `review:rc24-per-axis/(R10, F5)` · `origin:` driver ×2 (two rows independently)
- `new-code:` **inside the co-author trailer** — the binary matches its design (*The amend arm*); the step text did not move (`28ca86b4` touched no pack file)

```text
repro:
  setup     HEAD carries Co-Authored-By: Claude <noreply@anthropic.com> (an agent-signed finalize)
  argv      env -u CLAUDECODE jigc task amend "repair the message"
  observed  exit 0 · line 58: "Trailers are re-authored too — the amended message carries only the trailer items you add
            here, so re-add any the old message had that still apply"
  argv      (type and summary authored, NO trailer item)  env -u CLAUDECODE jigc task finalize repair-the-message
  observed  exit 0 · neither task validate nor task finalize prints a line naming a trailer
  after     the amended message ends "Co-Authored-By: Claude <noreply@anthropic.com>" ; the tree unchanged
            four arms (set/unset × HEAD carried it or not): false in three, true in one
            adding only a different co-author lands both ; through jigc there is no way to land an amended message
            without the profile's trailer once HEAD has it
```

- `contract:` the step's own sentence (`crates/cli/packs/dev/steps/amend-message.yaml`).
  `design/assistant-adapter.md` → *The co-author trailer* → *The amend arm* declares the carry.
- `UNPINNED:` `crates/cli/tests/agent_co_author.rs` fences the carry; the step sentence against it is
  not asserted by a test found.

##### `(R9, C-1)` · `task amend --help` and the installed guide say the new message is authored from scratch and `HEAD`'s message is not read back; the landed message carries a line read from `HEAD`

- `tier:` 3 · `door:` `task amend` · `found-in:` `review:rc24-per-axis/(R9, C-1)` · `origin:` codex
- `new-code:` **inside the co-author trailer** — the same seam as `(R9, F2)` on two more surfaces (the help; the guide's *Landed it with a wrong message?* paragraph)

```text
repro:
  setup     HEAD carries Co-Authored-By: Claude Opus <noreply@anthropic.com>
  argv      env -u CLAUDECODE jigc task amend … ; type + summary authored, zero trailer items ; jigc task finalize <id>
  observed  exit 0 · the amended message carries Co-Authored-By: Claude <noreply@anthropic.com> (the profile's
            rendering, not HEAD's bytes)
  argv      jigc task amend --help
  observed  "… HEAD's message is not read back into the doc: you author the new message from scratch."
```

- `contract:` the help sentence's second half; the narrower literal (nothing of `HEAD`'s message enters
  the *doc*) holds.
- `UNPINNED:` as `(R9, F2)`.

##### `(R9, F3)` · with the opt-in invocation log on, `jigc uninstall` leaves `.jigc/` behind, un-ignored, and its second run is not a no-op

- `tier:` 3 · `door:` `uninstall` · `found-in:` `review:rc24-per-axis/(R9, F3)` · `origin:` driver
- `new-code:` **not established**

```text
repro:
  setup     jigc config set invocation-log true (committed)
  argv      jigc uninstall
  observed  exit 0 · "- removed .jigc/"
  after     find .jigc -type f → .jigc/logs/invocations.jsonl (the uninstall's own record) ; git status → ?? .jigc/logs/
            jigc uninstall (again) → "- removed .jigc/" (it acted) ; the THIRD run is the no-op
  control   knob off: the second run is "(nothing to remove …)"
```

- `contract:` `jigc uninstall --help` — *"removes `.jigc/`"*; *"Idempotent: a second run is a clean
  no-op"*.
- `UNPINNED:` no test found uninstalling with the log on.

##### `(R9, F4)` · a migration task's `--dry-run` forecasts two `modified` paths that are not modified and that the commit does not carry

- `tier:` 3 · `door:` `task finalize` · `found-in:` `review:rc24-per-axis/(R9, F4)` · `origin:` driver
- `new-code:` **outside the new code** — predates M54; reported because the row drives MIGRATING gate 1, a sentence the range edited

```text
repro:
  setup     a hand-written docs/decisions/use-postgresql.md committed ; jigc migrate … --as adr ; jigc doc author adr …
            git status and git diff HEAD over .jigc/config and .jigc/.gitignore: empty
  argv      jigc task finalize <id> --dry-run
  observed  exit 0 · "promoted docs/decisions/use-postgresql.md / modified .jigc/config / modified .jigc/.gitignore"
  argv      jigc task finalize <id> --approve --format json
  observed  exit 0 · committed.manifest: one row (promoted) ; files: 1 ; git show: M docs/decisions/use-postgresql.md
```

- `contract:` MIGRATING gate 1 — *"It prints the manifest the finalize would commit"*; the flag's help —
  *"the file-set the commit would carry"*. The forecast prints the pathspec, not the change-set.
- `UNPINNED:` `crates/cli/tests/finalize_manifest.rs` asserts the `.jigc/config` row of the migration
  forecast — the behaviour as it is, by `predict_manifest`'s own doc-comment.

#### Row 10 — co-author trailer

##### `(R10, F1)` · signing opens a new block under a git-recognized mixed last paragraph, so git stops reading the `Signed-off-by` — reachable from `#trailers` items alone

- `tier:` 3 · `door:` `task finalize` · `found-in:` `review:rc24-per-axis/(R10, F1)` · `origin:` driver, widened by the reconciler; it refutes the Codex pass's *Placement, sentence 1: carried*
- `new-code:` **inside the co-author trailer**

```text
repro:
  setup     a single-task commit doc; two #trailers items: Signed-off-by (Pat Example <pat@example.com>) and
            Reviewed_by (an underscore key the add-item accepts) ; a file staged
  argv      env -u CLAUDECODE jigc task finalize land-the-work            (one rig) ; CLAUDECODE=1 … (an identical rig)
  observed  exit 0 ; exit 0
  after     unset: git log -1 --format='%(trailers:only,unfold)' → Signed-off-by: Pat Example <pat@example.com>
            set:   the message gains a NEW block after one blank line ; git reads only Co-Authored-By: Claude
                   <noreply@anthropic.com> ; trailers:key=Signed-off-by → 0
            `git interpret-trailers` on the same message appends INTO the existing block and keeps both
```

- `contract:` `design/assistant-adapter.md` → *Placement and de-duplication* — *"the place `git
  interpret-trailers` gives a trailer added at the end"*. The line is still in the message; git's parser
  stops reading it.
- `UNPINNED:` `crates/cli/tests/agent_co_author.rs` and `commit_trailer_roundtrip.rs` carry no
  `Signed-off-by` cell.

##### `(R10, F2)` · the amend carry misses a `HEAD` trailer git reads inside a mixed block

- `tier:` 3 · `door:` `task finalize` (amend arm) · `found-in:` `review:rc24-per-axis/(R10, F2)` · `origin:` driver; it refutes the Codex pass's *Amend sentence: carried*
- `new-code:` **inside the co-author trailer** — only for a hand-built `HEAD` message (jigc's renderer never emits a mixed block)

```text
repro:
  setup     a plain-git commit whose last paragraph is Signed-off-by: + Co-Authored-By: Claude <noreply@anthropic.com> +
            one prose line ; git counts 1 co-author trailer (before-control)
  argv      env -u CLAUDECODE jigc task amend "repair the message" ; (fill) ; env -u CLAUDECODE jigc task finalize …
  observed  exit 0 ; exit 0
  after     trailers:key=Co-Authored-By → 0 ; tree and parent unchanged ; with CLAUDECODE=1 on both calls: 1 → 1
```

- `contract:` *The amend arm* — *"carried over when `HEAD`'s message already carries the profile's
  trailer"*.
- `UNPINNED:` no mixed-block fixture in the trailer suites.

##### `(R10, F3)` · `setup.profile-load`'s route blames the embedded profile for a directory-selected one

- `tier:` 3 · `door:` `setup` · `found-in:` `review:rc24-per-axis/(R10, F3)` · `origin:` driver
- `new-code:` **outside the new code** for the route text (it predates the trailer); **the trailer adds six new ways to reach it**. Reachable only through `JIGC_ADAPTERS_DIR`, a test seam no adopter guide names

```text
repro:
  setup     bare rig; P=$(mktemp -d); a copy of crates/cli/adapters/claude-code.yaml with the co-author name set to ""
  argv      JIGC_ADAPTERS_DIR="$P" CLAUDECODE=1 jigc setup --format json
  observed  exit 1 · setup.profile-load — message names the fault exactly ; route: "reinstall jigc — the embedded
            adapter profile is missing or malformed"
  after     commits 1 → 1, status clean ; with the directory variable empty the embedded profile installs (exit 0)
```

- `contract:` the route names a cause the binary did not have.
- `UNPINNED:` no test under `crates/cli/tests/` names `setup.profile-load`.

##### `(R10, F4)` · the declared bound's `#trailers` recording route is honoured only for a code-carrying sub-task under `squash: false`

- `tier:` 3 · `door:` `milestone finalize` · `found-in:` `review:rc24-per-axis/(R10, F4)` · `origin:` driver
- `new-code:` **inside the co-author trailer** — two design sentences contradict each other

```text
repro:
  setup     squash true (pack-default); a sub-task entered under CLAUDECODE=1, a file staged
            jigc doc add-item commit:area-low#trailers --title Co-Authored-By ; set-field …/value "Claude <noreply@anthropic.com>"
            jigc doc show commit:area-low --task area-low → prints the item                       (before-control)
  argv      env -u CLAUDECODE jigc milestone finalize cache-rework
  observed  exit 0 · "Finalize milestone cache-rework (2 sub-tasks)" ; 0 output lines name a trailer
  after     trailers:key=Co-Authored-By → 0. squash false, the item on a code-carrying sub-task → its commit carries it ;
            on a docs-only sub-task → nowhere
  route     a working repair exists: jigc task amend over the boundary commit with the item lands it (0 → 1)
```

- `contract:` `design/assistant-adapter.md` → *Declared bounds* — *"the commit doc's `#trailers` is the
  route to record it"*; against `design/finalize.md` (*"there is no authored commit doc behind the
  boundary message"*).
- `UNPINNED:` no test found authoring a trailer item on a sub-task's commit doc.

##### `(R10, F5)` = `(R9, F2)` · the composed amend step says the amended message carries *only* the trailers you add; it carries one more

- `tier:` 3 · `door:` `task amend` · `found-in:` `review:rc24-per-axis/(R10, F5)` · `origin:` driver
- `new-code:` **inside the co-author trailer** — the full entry is `(R9, F2)`

```text
repro:
  as (R9, F2); row 10's own block: an agent-signed HEAD (=1); `env -u CLAUDECODE jigc task amend "repair the message"`
  prints the sentence at line 58 ; no item added → task finalize exit 0, =1 ; adding only Pat Example
  <pat@example.com> → exit 0, =2
```

- `contract:` as `(R9, F2)`.
- `UNPINNED:` as `(R9, F2)`.

##### `(R10, F6)` · a bracket-less `Co-Authored-By: <address>` in the doc is not read as the same address *(marginal)*

- `tier:` 3, marginal — a strike candidate at triage, as its driver says
- `door:` `task finalize` · `found-in:` `review:rc24-per-axis/(R10, F6)` · `origin:` driver
- `new-code:` **inside the co-author trailer**

```text
repro:
  setup     a commit doc trailer item Co-Authored-By with the value "noreply@anthropic.com" (no name, no angle brackets)
  argv      CLAUDECODE=1 jigc task finalize land-the-work
  observed  exit 0
  after     git reads two: "noreply@anthropic.com" and "Claude <noreply@anthropic.com>"
```

- `contract:` *"A trailer block that already names the **same address** — whatever name it spells — is left
  byte-identical"*. The sentence does not say the address must be in angle brackets.
- `UNPINNED:` no bracket-less value in the trailer suites.

##### `(R10, F7)` · the co-author is the commit-time environment's profile, not the one `jigc setup` installed from; no per-repo record *(marginal)*

- `tier:` 3, marginal — the Codex pass's *"relevant to the tier-1 exit rule"* is refuted: no loss, no harm
- `door:` `task finalize` (also driven at `setup`, `milestone create`, `milestone add-task`, the amend arm) · `found-in:` `review:rc24-per-axis/(R10, F7)` · `origin:` codex
- `new-code:` **inside the co-author trailer** — needs `JIGC_ADAPTERS_DIR` in jigc's own environment

```text
repro:
  setup     two profile copies declaring Agent A <a@example.com> and Agent B <b@example.com>
  argv      JIGC_ADAPTERS_DIR="$PA" CLAUDECODE=1 jigc setup
  observed  exit 0 · the install commit: Co-Authored-By: Agent A <a@example.com> ; 0 files record that identity
  argv      JIGC_ADAPTERS_DIR="$PB" CLAUDECODE=1 jigc task finalize land-the-work
  observed  exit 0 · Co-Authored-By: Agent B <b@example.com>
  after     the next door with no directory set lands the embedded Claude <noreply@anthropic.com>
```

- `contract:` `design/assistant-adapter.md` → *Which profile* — *"The one `jigc setup` installs"* (the same
  bullet already says no per-repo record of the choice exists).
- `UNPINNED:` the Codex pass names the installed/current profile mismatch as a suite-coverage gap.

## B · REFUTED — each with its falsifying datum (25)

Claims of either pass that a drive contradicted. (The hand-off tallies sum to 30; the 25 below are what
the ten ledgers enumerate. Row 6's three are driver table cells, not source claims; row 7 has none.)

| # | row | the claim | origin | the falsifying datum |
|---|---|---|---|---|
| 1 | 1 | the headline *"no missing door, undispositioned install path, unrecorded post-write error return, unsafe teardown classification …"* | codex (K) | refuted in three limbs: the hook member is consumed under `--force` and named by no advisory `(R1, F2)`; a jigc hook is classified foreign and `uninstall` reports a removal that left a jigc hook on disk `(R1, F3)`; and `(R1, F1)`, `(R1, F4)`, `(R1, F5)`. Two limbs hold (package/allowlist; release-path bypass) |
| 2 | 1 | the four-CPU `dev/runner-faithful` is *"still a declared non-run variant"* | codex (N5) | `dev/runner-faithful --cpus 4 tarball` → exit 0, `cpus 4`, every step `ok` — driven by the driver and again by the reconciler |
| 3 | 1 | the dirty-path refusal occurs before writes — as a universal | codex (S2) | the hook member is refused at the *backstop*, after the install is written and staged (8 paths `A `); on an unborn `HEAD` an untracked install path draws no refusal at all. Holds for the enumerated members on a born `HEAD` |
| 4 | 1 | post-failure user edits re-arm both worktree and index checks | codex (S3) | `(R1, F1)` repro B arm 2: after `git rm -r --cached .` on an unborn `HEAD`, a post-failure edit → exit 0, the edit in no object. Holds on a born `HEAD` |
| 5 | 1 | the embedded path is *"quoted for spaces"* — as sufficient | codex (S6b) | a space survives; a single quote does not → `(R1, F4)` |
| 6 | 1 | teardown distinguishes standalone, wrapped and foreign bytes — at the edge | codex (S6e) | a jigc hook from another path, another template or with one appended line is classified foreign → `(R1, F3)` |
| 7 | 2 | a rejected agent-authored commit permanently signs its message file, so a later human retry lands a false `Co-Authored-By` at exit 0 *(proposed tier 1, confidence high)* | codex (C-1) | driven through 13 commit constructions — all 11 `COMMITTING_DOORS` rows, the doc-only finalize and the approved migration commit: agent run under a rejecting hook → exit 1, `HEAD` unmoved; the human retry → exit 0 and **no trailer on any landed commit**, beside two positive controls |
| 8 | 3 | `jigc validate --format json` *incorrectly exits 0* without the F21 finding | codex (C1c) | refuted as worded: the whole-file-shadow control **reports** the finding and exits 0 too (`report_only: true`); `workflow-refs` is no `STORE_EXIT_FLIPS` member. What is missing is the row, not the exit |
| 9 | 3 | *NO TIER-1 ROW* on the row, and the reading that `design/storage.md` covers the lost-baseline cell | driver | `(R3, F7)` — driven, the cell is an exit-0 loss, and the declared cost is the neighbouring order's |
| 10 | 4 | the manifest race is *"potentially tier 1 when reached through `migrate-corpus`"* | codex (S-1t) | under the race `migrate-corpus` exits 0 and commits; the commit's whole diff is `-schema-version: 2 / +schema-version: 3`. The exit-0 half is shown, the loss-or-harm half absent → `(R4, C1)` at tier 3 |
| 11 | 4 | `cwd-unreadable` — *NOT CLOSED, fixture-dependent* | codex (S-8) | the fixture is `mkdir` · `cd` · `rmdir`; seven argvs → exit 1, *"cannot determine the current directory"* |
| 12 | 4 | the eager pack-load sweep reaches every pack-loading door | codex (S-11b) | true of a pack-layer definition (10 of 10 exit 1). For the same defect in a **project-layer shadow**: `start --explain`, `doc list`, `doc schema`, `migrate-corpus --dry-run`, `ingest` → exit 0 |
| 13 | 4 | `pack-resource-missing` is loud for every resource | codex (S-9), split | the manifest is the one resource whose absence is silent → `(R4, C1)` |
| 14 | 5 | *"No new source-grounded completeness defect found in the M54/M55/trailer scope"* | codex (K2) | `(R5, F1)`: exit 0, `git diff --cached` gains an entry, a committed doc labelled `left-staged`; also `(R5, F4)` and `(R5, F10)` on M55's own surfaces |
| 15 | 5 | the left-out classification is consistent (one entry per porcelain path) | codex (K14) | the mechanism is confirmed and is exactly what produces `(R5, F4)` (one path, two entries) and `(R5, F1)`'s mislabel |
| 16 | 6 | row `C046`'s code cell (`finalize.base-mismatch` beside exit 0) | driver | re-driven: `doc create` exit 0, `existed: false`, no finding — the code is a **later** finalize's |
| 17 | 6 | row `C105`'s code cell (`create.already-exists` beside exit 0) | driver | re-driven: exit 0, `existed: false`, no finding — carried over from the previous cell |
| 18 | 6 | row `C108`'s code cell (`finalize.base-mismatch` beside exit 0) | driver | re-driven: `doc author` exit 0, no finding — another task's later finalize |
| 19 | 8 | `(6, D-1)` is STILL OPEN — orientation has no repository-posture sweep | codex | after `git bisect start`, `jigc start` prints `blocking · repo.operation-in-progress` and carries it as `tasks[0].findings[0].code` |
| 20 | 8 | the driver's row 5 count — `0 ×20 · 1 ×19` | driver | `milestone-execution` is counted twice there; driven: 21 previews exit 0, 18 exit 1, 20 carry one `Run: jigc task finalize`. The verdict is unaffected |
| 21 | 9 | `(8, N-1)` is CLOSED — *"all three guards"* refers to the three categories in that flag sentence | codex | with those three clean and only a parked file under `.jigc/displaced/` present, `--force` is not inert: it deletes the file |
| 22 | 9 | no false **dry-run** claim in the scoped guides | codex (S-8d) | MIGRATING gate 4 (*"refuse the same states"*) — on a doc-only task with a pre-task staged path both exit 0 `(R9, F1)`; MIGRATING gate 1 — a migration forecast names two paths the commit does not carry `(R9, F4)` |
| 23 | 9 | the composed `plan` step's `spec:<slug>#derived-from` against the schema's `#meta/derived-from` is a defect | driver (lead O-6) | both spellings exit 0 and write the same front-matter field |
| 24 | 10 | *Placement, sentence 1* is carried — and claim 1 is *"relevant to the tier-1 exit rule"* | codex | `(R10, F1)`'s repro: on the identical message `git interpret-trailers` appends into the existing block; jigc opens a new one. And for claim 1 `(R10, F7)`: exit 0, **no loss and no repository harm** |
| 25 | 10 | the *Amend sentence* is carried; and the reconciler's own lead that a `BREAKING CHANGE` item widens `(R10, F1)` | codex · reconciler | `(R10, F2)`'s repro: git counts 1 before, a human's amend lands 0. The `BREAKING CHANGE` key is refused at `add-item` (*"the trailer key contains whitespace"*) |

**Corrected, not refuted:** row 6's source pass weighted `(R6, K-1)` *"potentially tier 1 because the
successful path reaches a committing door"*; in the scenario it proposes, the committing door refuses. The
claim's gate half reproduces, so it stands as a finding with a split tier.

## C · OPEN leads — driven as far as the state allows, promoted by nothing

Never promoted on a source read, never dropped. The row file's own list governs; this is each list
condensed, with the reason. (The hand-off tallies sum to 83; they count each row's entries at different
grain.)

**Row 1 (8).** Recovery of a release-PR merge whose deployment did not publish (agents may not drive the
release workflow) · the genuine Task-tool spawn · the Linux replaced-binary arm (**driven by row 2** at
two doors — see row 2) · the cold CI runtime (a GitHub-runner fact) · the real release pipeline, the
environment approval, Trusted Publishing, the yank (read only: both crates published, `missing=false`) ·
`dev/install-release-plz` fetching the genuine asset (only the digest-mismatch refusal was driven) · a
`jigc` path carrying shell metacharacters after a single quote (it would mean planting a hostile path) ·
rows standing on the driver alone (8.3, 8.7, and the three non-commit failure sites 2.1, 2.10, 2.11).

**Row 2 (10).** The *response serialize* failure arm (no input reaches it) · *no fourth production probe
spawn* and *no error identity outside `ERROR_CODE_REGISTRY`* (absence claims over source) · the
schema-hash boundary (row 4's) · a probe grandchild that **never** exits (driven to 47 s and 120 s, never
unbounded — it would hold a `git commit` open through the hook) · `(R2, T-1)` at the five doors not
driven · `(R2, P-1)` at `milestone finalize` (driver only) · two leads handed to other rows, below · the
second Codex pass.

**Row 3 (9).** F21's raw read over fan-out/join pairing and front matter (not constructed) · two
completeness negatives · the trailer's pinned keys (row 10's) · F21 × a `remove-step` delta (a possible
false positive, the inverse of F21) · `(R3, F4)` with a pulled record edit that drops a sub-task row, then
the join · `task discard <sub> --force` in the absorbed-terminal posture (it bounds how dead the dead end
is) · **`(R3, F7)`'s bounds** — a location-doctype doc, the `milestone finalize` door, the sanctioned
cache deletion as the way in, an rc.23-or-older binary · thirteen of the driver's seventeen not-driven
rows · the surface of `jigc start --task <id>` once the moved history overlaps the task.

**Row 4 (8).** `(R4, C1)` at a committing door on unpadded packs · the embedded-pack arms (an absent
manifest entry; a reshaped schema inside the binary — both need a rebuild) · the rest of the 54
`SchemaChangeKind × LOCI` cells · the cross-version half (*the existing corpus is read unchanged across
the range* — no older binary) · `RelocateRefusal::ALL` 9 of 10 and `relocate`'s success arm (need a
freeze-exempt doctype; none ships) · row 5.7 · *"changed no pinned JSON key"* · three reconciler
observations adjudicated against no contract (a `docs-root` hybrid strand where `doc show`, `validate`
and `doc list` describe one state three ways; `migrate-corpus`'s `unadopted` row calling an untracked
file *committed* and printing a host path; `relocate --format json` flattening a coded refusal to
`{"error": …}`).

**Row 5 (4).** The other rollback populations at the stage-failure point · a genuine concurrent process
· `git_commit_paths`' empty-list refusal (unreachable through the door — `finalize.empty-commit` refuses
first) · amend-over-doc-only precedence (no task carrying both markers was built).

**Row 6 (3 + the driver's U1 – U14).** *No third production create path* at `jigc migrate` under a
`new: true` migrate shadow and at `milestone add-from-spec` · *rc.23 → rc.24 changes no pinned key* ·
`(R6, K-1)` at `doc author` against a second reporter's finalize, and a sub-task's create against a plain
task's finalize. Chiefly un-driven: a **case-sensitive filesystem**, `create.serial-collision` in the
order of refusals, `(R6, D-1)` with a foreign file, a directory or a symlink at the suffixed home, a
team-layer shadow.

**Row 7 (8).** The five not-re-driven rc.20 rows (X6, X7) · the BOM arm of the H1 reader · whether any
methodology `schema-hash` moved since rc.20 (no surface prints a hash) · the committing doors' pinned
subjects (row 10's) · `(5, C-13)`, `(5, C-18)` · the provenance of `(R7, D-2)` and `(R7, D-3)` · the
driver's not-driven list.

**Row 8.** `(6, L-1)`, `(6, L-3)` (the carve-out half), `(6, L-7)` · `(6, L-2)`, `(6, L-4)`, `(6, L-5)`
×3 (driven, unchanged, each a declared bound) · the ~40 unprefixed *finalize* mentions in sub-task text
(adjudicated by neither pass; the VERDICT's own *Not run*) · *no other hash or pinned key moved* ·
`config remove-step`, a delta recorded after a mint, the team layer, the knob flipped `false` → `true`,
the two `*-jigc-feedback` workflows followed to a commit, eleven of twelve `migrate --as <ty>` doors as
top-level controls.

**Row 9 (6).** The signing seam at `milestone add-from-spec`, at `migrate-corpus` on a real migration and
at `milestone finalize (squash: false)` (row 10 drove all three) · the manifest tag `deleted` · **the
rc.24 crates.io page as rendered** and each README URL over HTTP · three demoted arms not re-driven · M51
`D-1` · **the ruling on `(R9, F5)`'s tier**.

**Row 10.** `(R10, F7)` at the seven doors not driven against a swapped profile · *Which profile*
sentence 3 (prospective) · what the assistant exposes to its agent (not drivable without recording an
environment value) · **a human typing into an agent's session** · the run-through-cargo bound on the
installed binary · `relocate`'s success path as a non-committing leaf · the driver's own list (the
profile variants at ten doors, a rejecting hook on the doc-only finalize, the echoing hook on eight doors,
any other git or platform, a multi-line trailer value, ambient `commit.cleanup` / `core.commentChar`).

**Handed between rows and graded by none — these are the leads most likely to be lost:**

| lead | seen by | why it is open |
|---|---|---|
| the doc-only left-out narration lists an **untracked** path under *"a staged path stays staged for the task it belongs to"* | rows 2, 6, 9 (it is in `(R9, F1)`'s own repro output) | row 5, which owns the narration, recorded *the text arm's left-out list carries no kind* as data and filed nothing |
| `jigc start --task <sub>` inside a sub-task worktree exits 0 and does not provision the commit doc; `jigc workflow sub-task --task <sub>` does | row 2 | not re-driven by row 8 |
| a **directory** named `<slug>.md` at a doc home: `task finalize` exits 1 code-less, and `jigc doc list <type>` exits 1 code-less for the whole doctype (a dangling link does the same to `doc list`) | row 6 | owned by rows 5 and 7; neither drove it |
| three surfaces call an **untracked** file *committed* — `schema-conformance.unadopted-instance`, `file-state.un-baselined`, `migrate-corpus`'s `unadopted` row | rows 4, 5, 6 | row 3's surface; not graded |
| `milestone add-task <m> … --workflow <a workflow whose shadow fails to load>` exits 0 and commits a record row for a sub-task no door can compose | row 6 | row 8's; not graded |
| a plain task's commit landed between `milestone provision` and `milestone finalize` puts `finalize.base-mismatch` in front of the join — including a doc-only report that moves no code | rows 3, 6 | `design/team-ready-state.md` declares the guard; whether a doc-only commit should trip it is undecided |
| a move plus an edit of a managed doc passes the hook silently, while the sweep the hook runs exits 1 on that staged set | row 1 | the weak signal by design; handed to row 3, not graded there |

## D · Observations driven and carried — **not** defects

Each is in its row file with its drive. The ones a reader of the headline should not have to dig for:

- **A declared cost sits one cell from `(R3, F7)`.** With the baseline absent, a hand edit made *before*
  the task's first write is carried into the copy-in and lands with the task's prose at exit 0 — the
  *silent merge* `design/storage.md` declares; a hand edit *committed* during the task is caught
  (`finalize.base-mismatch`). The loss is exactly: baseline absent × edit after the copy-in × edit
  uncommitted ([row-3.md](row-3.md) R-8b).
- **An override that answers `{"findings":[]}` passes a drifted anchor at exit 0** at `validate` and
  `task validate`, and no surface says the adjudicator was an override. Declared — the override is the
  operator's own trusted probe; `task finalize` under it was deliberately not driven by either agent
  ([row-2.md](row-2.md) §6.3, R-2e).
- **Under an entry *without* `new: true`, an untracked conformant file at a doc home is copied in and the
  task's finalize replaces bytes git never held** (exit 0, a marker 1 → 0). Weighed against the tier-1
  predicate and left an observation: it is the declared create-or-update, on an address the caller named
  ([row-6.md](row-6.md) O-2).
- **`CLAUDECODE=0`, `=false` and a single space all sign as the agent** — exactly *set and non-empty*
  ([row-10.md](row-10.md) O-3).
- **`milestone finalize` and `task finalize` sweep an uncommitted `jigc config` write into the commit** —
  what the `config` ack says will happen ([row-8.md](row-8.md) O-6).
- **`jigc rename <id> --to <its current title>` commits** — `renamed X -> X … repointed 0 referrer(s)`,
  `HEAD` moved ([row-3.md](row-3.md) O-9).
- **`milestone finalize`'s landed envelope has no `findings` key**, so a non-blocking L1 / L2 advisory
  present at the join is not on its JSON arm ([row-3.md](row-3.md) O-7).
- **No adopter-facing sentence says a commit is signed.** Zero hits for `co-author`, `trailer`,
  `authorship` over the installed guide and all 49 help texts; the only adopter-visible sentences are two
  composed steps, one of which is `(R9, F2)` ([row-9.md](row-9.md) O-2).
- **`implementation/release.md` says the `jigc` package listing is 184 files; on rc.24 it is 195.** The
  equality with the allowlist, which is the contract, holds from both sides — and the published crate
  equals the listing file for file, checksum for checksum ([row-1.md](row-1.md) O5, 7.1 – 7.4).
- **The rc.20 record's `ENVELOPE_ARMS` partition is wrong in its preamble** (`60 Pinned / 6 Unpinned`
  against the registry's 59 / 7) and its *unreachable* `CompoundFieldSlice` arm is reachable through
  `milestone-record.base` ([row-7.md](row-7.md) §8.3).
- **The instrument's *"8 call sites of `render::composed`"* is one short** — nine print sites over five
  verbs ([row-8.md](row-8.md) R.1).

---

# COVERAGE — the ten rows against the 48 `VERB_KINDS` leaves

`VERB_KINDS` (`crates/cli/src/cli.rs`) carries **48** leaves at this tree — 36 `Write`, 12 `Read`. A leaf
is *reached* by a row iff it is the door of ≥1 **driven** row of that row file carrying a repro block —
the driver's or the reconciler's. A leaf used only to build a fixture is not listed.
**On this scoped run the union of all 48 is not a goal**; the table says what was reached, not what was
owed.

| leaf | reached by rows | note |
|---|---|---|
| `start` | 2 · 3 · 4 · 5 · 6 · 8 · 9 | |
| `workflow` | 3 · 4 · 5 · 6 · 8 · 9 | |
| `setup` | 1 · 2ʳ · 4 · 9 · 10 | |
| `uninstall` | 1 · 9 · 10ᶜ | its refusing arms: row 9 only |
| `upgrade` | 3 · 9 · 10ᶜ | |
| `ingest` | 3 · 4 · 9ʳ · 10ᶜ | |
| `migrate` | 4 · 8 · 9 · 10ᶜ | |
| `migrate-corpus` | 2 · 3 · 4 · 9ʳ · 10 | row 9: the no-op arm only |
| `unmanage` | 3 · 4 · 9 · 10ᶜ | |
| `rename` | 2 · 3 · 5ʳ · 9 · 10 | |
| `relocate` | 4 · 10ᶜ | **the refusal path only**, in both — no stock doctype is freeze-exempt |
| `describe` | 4 · 6 · 8 · 9ʳ · 10ᶜ | |
| `validate` | 1ʳ · 2 · 3 · 4 · 6 · 8 · 9 · 10ᶜ | |
| `doc create` | 4 · 6 · 8 · 9 | |
| `doc add-item` | 4 · 6 · 8 · 9 · 10 | |
| `doc remove-item` | 6 | one row |
| `doc retitle-item` | 6 | one row |
| `doc rename` | 6 | one row |
| `doc set-field` | 4 · 6 · 8 · 9 · 10 | |
| `doc set-slot` | 3 · 4 · 6 · 8 · 9 | |
| `doc author` | 6 · 8ʳ · 9 | |
| `doc show` | 2 · 4 · 6 · 7 · 8 · 9 | row 7: all 8 registry arms |
| `doc schema` | 1ʳ · 4 · 6ʳ · 7 · 8ʳ · 9ʳ | row 7: 18 of 18 doctypes |
| `doc list` | 3 · 4 · 6 · 7 · 8 · 9 · 10ᶜ | |
| `task list` | 4 · 9 · 10ᶜ | |
| `task diff` | 9ʳ | one row, reconciler block only |
| `task validate` | 2 · 3 · 5 · 6 · 8 · 9 | |
| `task amend` | 3 · 5 · 7 · 8ʳ · 9 · 10 | |
| `task discard` | 2 · 3 · 4ʳ · 6 · 9 · 10 | |
| `task finalize` | 2 · 3 · 4 · 5 · 6 · 8 · 9 · 10 | the most-reached leaf — eight rows |
| `task bind` | 8 | one row |
| `config set` | 2 · 4ʳ · 5 · 8 · 9 · 10ᶜ | |
| `config insert-step` | 3 · 5 · 8 | |
| `config replace-step` | 3ʳ · 5 · 8 | |
| `config remove-step` | 5 | one row |
| **`config fill`** | **— none** | **reached by no row** — row 9 ran its `--help` only (a help read confers nothing) |
| `config fork` | 8 | one row |
| `config get` | 2 · 4 · 6 · 9ʳ | |
| `config list` | 6 · 10ᶜ | |
| `milestone create` | 2 · 5 · 6 · 7 · 8 · 9 · 10 | row 7: the baseline tier-1 row's first mint door |
| `milestone add-task` | 2 · 3 · 5 · 6 · 8 · 9 · 10 | |
| `milestone add-from-spec` | 2 · 9 · 10 | |
| `milestone list-tasks` | 6 · 9ʳ · 10ᶜ | |
| `milestone provision` | 5 · 6 · 9 · 10ᶜ | |
| `milestone execute` | 6 · 8 · 9 · 10ᶜ | |
| `milestone join` | 5 · 6 · 8 · 9 · 10ᶜ | |
| `milestone finalize` | 2 · 3 · 5 · 6 · 8 · 9 · 10 | |
| `milestone discard` | 2 · 9 · 10 | |

*ʳ = reached on that row only by a reconciler block (the driver's rows for it carried none, or the
reconciler drove it to settle a lead). ᶜ = reached on row 10 as a negative control — a row asserting
`HEAD` does not move under the agent variable; a driven row with a verdict, and a thin one.*

**Union: 47 of 48. The one leaf no row reached is `config fill`.**

**Leaves per row:** row 1 — 2 (+ `validate`, `doc schema` as lead rows) · row 2 — 15 (+ `setup`) ·
row 3 — 18 · row 4 — 22 · row 5 — 15 · row 6 — 27 (all 8 `doc` × `Write` leaves among them) · row 7 — 5
· row 8 — 26 · row 9 — 37 · row 10 — 29 (10 leaves carrying its 12 doors, `task amend`, 16 negative
controls, 2 fixture-side rows).

**What the 47 does not say.** Coverage is per leaf, not per cell: **seven leaves were reached by exactly
one row** (`doc remove-item`, `doc retitle-item`, `doc rename`, `task diff`, `task bind`,
`config remove-step`, `config fork`), `relocate` only on its refusal path, and several only as a negative
control. **Doors that are not clap leaves** carried driven rows and confer no leaf coverage: the installed
pre-commit hook through a real `git commit` (rows 1, 2, 3, 8), the `__probe` intercept (row 2), the
install line, `cargo package --list`, `cargo publish --workspace --dry-run`, `dev/unpublished-versions`,
`dev/runner-faithful`, `dev/hygiene-scan`, `dev/install-release-plz` (row 1). **No comparison against
each baseline's own coverage, restricted to the row's subject, was computed** — the baseline READMEs
tabulate coverage per numbered axis, unrestricted, and the restriction would need their axis files
re-read (→ HONEST BOUNDS).

---

# HONEST BOUNDS — what this record does **not** say

**Numbered axes 1, 2 and 3 are not among the ten rows.** Caller tokens, posture and destroying doors
were not asked. Their rows stand where the last run that drove them left them — axis 1 on rc.16, axes 2
and 3 on rc.20 — and **three of the four historical tier-1 rows live there**: `(3, A3-1)`, `(3, A3-2)`
and `(2, DEFECT A)`, with rc.17's `(2, DEFECT 1)` beside them. They were last driven CLOSED on rc.20.
**This record says nothing about them on rc.24.** (`(R9, F5)` and `(R6, D-1)` are new findings whose
*subjects* — a destroying door, an overwriting promote — are the kind numbered axis 3 owns; they were
found from rows 9 and 6, not by re-driving that axis.)

**Every row has a Codex pass; no row has a second one.** All ten source passes exited 0. A second,
higher-effort pass was launched for each row and **all ten ended on the provider's usage limit** (exit
code 1, no output file). Nothing rests on them; whatever they would have claimed is unreviewed.

**No row is without a result.** All ten reconciled files exist and are copied here.

**The instrument's own *Not pinned down* list stands, by reference** —
[instrument/README.md](instrument/README.md), items 1 – 16. The ones this run touched:

- *Item 1 (Docker).* Rows 1 and 2 probed and found it answering (the container cells were driven).
  **Row 6 records *"Docker did not answer"*** and so drove no case-sensitive filesystem. Both statements
  are in the files; this record does not reconcile them.
- *Item 2 (the phrase "M54's store exit flips").* Row 3 drove the registry as it is — all seven members.
- *Item 8 (row 10's run-through-cargo cell).* A fence row, no coverage; the *set, empty* arm is the drive.
- *Item 9 (project-layer shadows).* Rows 4, 5, 6 and 8 each hand-wrote files under the tracked
  `.jigc/config/`, as granted. **Row 4's reconciler went one kind further** — a hand-written
  `.jigc/config/packs.yaml` — and says so (its L0); the rows that depend on it are named there.
- *Item 12 (cells no driver can reach).* All still unreached, except the Linux replaced-binary arm, which
  row 2 drove in a container.
- *Item 14.* The first paragraph of this section.

**What each row states it did not drive** (each row file carries the full list with reasons):

| row | chiefly not driven |
|---|---|
| 1 | merging, pushing, tagging, publishing, approving a deployment (agents may not) · the bare install line on the host · **the rc.24 crates.io page as rendered** · `uninstall`'s refusing arms and every posture cell but detached `HEAD` (numbered axes 3 and 2) · 14 of 20 `setup.*` codes · a hook template that really differs between two published versions (it is identical at all three tags) |
| 2 | 41 `(door, cell)` pairs — the timeout and the malformed-output arms at five of six doors · the Linux arm at four doors · a partially landed `milestone add-from-spec` · the log unwritable while a door commits · the knob at the team layer · `task finalize` under an override that answers clean (it would commit) |
| 3 | 13 of 17 listed rows — the `live_record_finding` arm · L1 at the task gate reached by merge or hand commit with the doc touched · L2 × a doc only the stash holds · every door × `--format human` · `probe-unreliable` by timeout (row 2's) |
| 4 | the embedded-pack arms · the cross-version half · 48 of 54 `SchemaChangeKind × LOCI` cells · `RelocateRefusal::ALL` 9 of 10 and `relocate`'s success arm · shape variants at 16 of 19 doors · anything `setup` might write outside `$RIG` |
| 5 | 19 groups — every cell on `--format human` · hook rejection on the `jigc-feedback` pair · a racing hook at an owner-artifact path · `squash=false` and a hook-rejected boundary for a doc-only sub-task · six `COMMITTING_DOORS` rows outside the partial scope · `setup` |
| 6 | U1 – U14 — a case-sensitive filesystem · `create.serial-collision` · the feedback entry beyond absent and committed · 23 of 29 workflows with entries · `squash=false` and `doc author` in the fan-out · a team-layer shadow |
| 7 | 45 of 48 leaves by scope · a staged copy that does not parse · a managed row at a prior home · a BOM-prefixed file · the whole-doc serve on 8 of 18 doctypes · four non-root cwds · a project-layer schema shadow |
| 8 | the genuine concurrent spawn (every sub-task ran sequentially in one process) · cell 2 followed to a landed boundary for 4 of 39 workflows · `config remove-step` · the team layer · `squash=false` under deltas · the knob flipped `false` → `true` |
| 9 | 17 entries — the install line executed · the crates.io page · `migrate-corpus` on a real migration · `squash: false` · six of nine `--dry-run` refusal codes · eleven leaves whose `--help` ran and whose verb did not |
| 10 | a human typing into an agent's session · run-through-cargo on the installed binary · the profile variants at ten of twelve doors · the *set, empty* arm of five cells · any other git or platform (git 2.54.0, Apple Git-157, darwin only) |

**The platform.** One host: Darwin arm64, APFS (case-insensitive), git 2.54.0 (Apple Git-157). Linux was
reached only by row 1's `dev/runner-faithful` cells and row 2's two replaced-binary rows — the latter on
the published rc.24 as `cargo install` builds it for Linux, in a container, **not** on the installed host
binary. Row 10's trailer-block findings rest on this git's recognized-prefix rule.

**No older binary was installed and none was built.** Every *"predates the range"* and every `new-code:`
dating in this record is a source or history read, never a drive on rc.23 or earlier. `(R3, F7)`,
`(R6, D-1)` (by its `park-idea` reproduction), `(R9, F5)`, `(R1, F1)` are dated that way; `(R3, F4)`,
`(R3, F6)`, `(R4, F5)`, `(R7, D-2)` say *not established*. And the cross-version half of the freeze —
*the existing corpus is read unchanged across the range* — was driven by nobody.

**Instruments and deviations the agents put on the record, restated so they are not buried:**

- **Row 4's reconciler** loaded an observational `open()` tracer into the unmodified binary for the three
  `(R4, C1)` drives only, and padded a schema with 198 MB of comment lines to widen the race window in two
  of them. One reconciler error minted two tasks by ordinary successful calls; both were discarded through
  `jigc task discard --force`.
- **Row 5's driver** used a *required git clean filter* as an in-transaction racer — its own instrument,
  not in the fixture library, driven although the scope said to keep the lead open. `(R5, F5)` rests on it.
- **Row 6's reconciler** raced the create-gate with a file toggler (a synthetic writer); its first
  finalize-race run read stdout only and is not counted.
- **Row 10's driver** lost a rig's `cd` inside a pipeline subshell, so nine `jigc` calls and one
  `git rev-parse` ran with **the working repository as cwd**. Every one of those calls exited 1 and landed
  nothing; `git status` was empty, `HEAD` unchanged and no `.jigc/` existed afterwards. A cwd guard was
  added and the cells re-driven. Every reconciler records the working repository clean and its `HEAD`
  unmoved after its last drive.
- **Row 1's `cargo install`, `cargo package` and `cargo publish --dry-run` cells** needed the network and
  a C compiler and ran against private target and install roots under the scratch root.

**The demotion pass left these not driven:** row 4's row 5.7 · row 5's R6.17, R6.34 and L4b · row 6's
`C103` and four sub-argv · row 7's G1 · row 9's arms 5.4 (an un-baselined store), 5.11 (a finding-free
`task validate`) and the eight sub-verb one-liners of 5.15 · row 3's row 102. Every other block-less row
was re-driven by its reconciler and stands **on the reconciler's block**, not on the driver's word.

**The assembler's own bounds.** This README was assembled from the ten reconciled files' ledgers,
findings and repro blocks; it **re-drove nothing**. The tier-1 re-read in the headline is a reading of
each repro block as written. The matrices were not read row by row (the ten files are about 1.3 MB). The
`pinned-by:` search and the `new-code:` datings are the assembler's own greps and `git log` reads, bounded
as stated at §FINDINGS. The roll-up's hand-off tallies were taken as handed and do not reproduce uniformly
from the ledgers (→ Roll-up). The restricted baseline-coverage comparison was not computed. Where this
README and a row file disagree, **the row file governs**.

**The adversarial re-drive's bounds** (→ *After assembly*; each file in
[tier1-verification/](tier1-verification/) carries its own list). It covered the **six tier-1 rows
only** — no tier-2 or tier-3 row was re-driven, and none of the four near-tier-1 rows the headline names.
It ran on the same host, the same binary and the same platform as the rows it checks, so it is
independent of their authors and not of their environment. Every dating in it is a `git log` / `git
blame` read; no older binary was built or driven. Reach and likelihood are each verifier's judgement,
not a measurement. Several class members are named from a code read and say so (*read, not driven*).
The scripts and raw outputs the files name are the verifiers' work files — they carry rig host paths
and are **not committed**, so the files' condensed blocks are what this record holds. The section's
table and its one line per finding are a reading of those six files; where they disagree with a file,
**the file governs**.

**What the run's preparation saw is recorded in the trial's README, not here** (added 2026-10-04).
The observations made while this gate's two instruments were being prepared — before any session or
row ran — are in [RC-rc24/README.md](../../RC-rc24/README.md): §9 → *Seen while preparing the trial —
recorded, not graded* (the trial's apparatus) and §11 → *Doc drift seen during the run — owed to the
closing fold-back or to the port, not fixed here*. One entry of the second list concerns this review's
own predecessors: the committed M51 – M53 per-axis records and their instruments carry absolute
home-directory paths, counted there and quoted nowhere. Neither list holds a finding of this review, and
neither changes a row.

---

# Files

| file | what |
|---|---|
| [row-1.md](row-1.md) | **setup · the hook · install · release** — the reconciled file: the driver's 132 rows over nine cells (install shapes, the failed first run, the settings pre-check, the dirty-install guard, the hook, install, package, release), three findings; the ledger adds two more reached by driving Codex leads. **Carries `(R1, F1)`.** |
| [row-2.md](row-2.md) | **probe integrity · the invocation log** — 174 rows (103 probe, 71 log), four findings; the ledger refutes the row's one tier-1 claim through 13 commit constructions and re-drives the Linux replaced-binary arm |
| [row-3.md](row-3.md) | **store exit codes / reconciliation** — 138 rows over the seven exit-flip members, L1 at both scopes, L2's seven arms, `unmanage`, F21; six driver findings; the ledger (§8) adds **`(R3, F7)`** and re-tiers `(R3, F4)` |
| [row-4.md](row-4.md) | **pack-load / manifest freeze · migration** — 419 rows; five driver findings, one Codex-origin; `(7, A7-F3)` CLOSED on both arms; the ledger's L0 names what the reconciler did beyond the instrument's grant |
| [row-5.md](row-5.md) | **finalize / transaction** — 105 rows over the doc-only arm's ten cells; ten findings; the tier-1 adjudication of `(R5, F1)` against seven ways to commit (§R.6) |
| [row-6.md](row-6.md) | **write surface** — 224 rows over entry × door × identity × title; ten driver findings, one Codex-origin. **Carries `(R6, D-1)`, `(R6, D-7)` and `(R6, K-1)`.** |
| [row-7.md](row-7.md) | **pinned read contracts** — 212 rows; all 8 `doc show` arms; `(5, DEFECT 1)` CLOSED; four tier-3 findings |
| [row-8.md](row-8.md) | **composed surfaces** — all 39 workflows composed as sub-tasks under both knob values; M55's declared bound measured; five tier-3 findings |
| [row-9.md](row-9.md) | **adopter docs & help** — the installed guide line by line, the install line, the crate README, the help of the range's verbs; six findings. **Carries `(R9, F5)`.** |
| [row-10.md](row-10.md) | **co-author trailer** — 237 rows: 12 doors × three arms, amend, dedupe, message shapes, the profile, fan-out, rejection and re-run, pinned keys, the hook stream; seven tier-3 findings |
| [codex/](codex/) | the ten **unseeded Codex source passes** (`row-N-source-pass.md`), so every lead is auditable against the verdict it was driven to |
| [instrument/](instrument/) | the instrument — the ten Codex prompts (`axisN-prompt.md`), the ten driver scopes (`axisN-driver-scope.md`) and its README. It contains no workflow script |
| [tier1-verification/](tier1-verification/) | **added after assembly (2026-10-04)** — the six independent adversarial re-drives of the tier-1 rows, one file per row: [R1-F1.md](tier1-verification/R1-F1.md) · [R3-F7.md](tier1-verification/R3-F7.md) · [R6-D-1.md](tier1-verification/R6-D-1.md) · [R6-D-7.md](tier1-verification/R6-D-7.md) · [R6-K-1.md](tier1-verification/R6-K-1.md) · [R9-F5.md](tier1-verification/R9-F5.md). Each carries its re-drive, the both-halves check, the boundary table, whether the behaviour is declared, the tier argued both ways, the code site, the fix's *cheap vs robust* fork and the pin. Five uphold tier 1; `R6-K-1.md` does not |

*The six verification files are copied **verbatim** from the verifiers' originals, with one added header
line each naming what the file is, the binary and the date; none of the verifiers' scripts or raw outputs
is copied.*

*The ten row files and the ten Codex passes are copied **verbatim** from the reconciled originals, with
one added header line naming the binary, the row and the date — **and one mechanical exception, made for
public hygiene**: in five Codex passes (rows 2, 3, 4, 7, 8) the absolute host prefix of this repository's
checkout was removed at 179 occurrences (markdown link targets), leaving the repository-relative path. Nothing else in any
copied file was edited; their internal references were written against the scratchpad's file names
(`axisN`) and are left as written.*

**What this record claims and nothing more:** ten rows, driven on the installed registry build of the
published `jigc 1.0.0-rc.24` on 2026-10-03; **six finding rows graded tier 1 by their reconcilers** —
four showing both halves on the loss disjunct without qualification, one in one of two variants, one on
the harm disjunct alone — **of which one sits inside M55's code and five outside the new code, none
inside M54's and none inside the co-author trailer**; **65 finding rows in all (6 · 7 · 52), 62 distinct
defects**; the one tier-1 baseline row inside the ten rows' baselines, `(5, DEFECT 1)`, **CLOSED**;
`(7, A7-F3)`, `(4, DEFECT 1)`, `(6, D-1)` and `(2, N-1)` = `(6, A6-R1)` **CLOSED**; **F21 STILL-OPEN as
expected and bounded wider than declared; M55's declared bound HOLDS AS DECLARED and is wider at
orientation**; **47 of 48 `VERB_KINDS` leaves reached, `config fill` by none**; and **nothing about
numbered axes 1, 2 and 3 on rc.24**. **The 1.0.0 call is the human's**, and so is the reading of the
count.

**And, after assembly:** an independent adversarial re-drive of those six rows on 2026-10-04 **upheld
five at tier 1** — `(R3, F7)`, `(R6, D-1)`, `(R1, F1)`, `(R9, F5)` and, on the harm disjunct only,
`(R6, D-7)` — **all five outside the new code**, and **did not uphold `(R6, K-1)` as a tier-1 row**
(tier 3 with a tier-2 tail; the loss class it points at is the human's to rule on once).
**The ruling on the contested rows and the scope of the fix pass are the human's and have not been
taken.**
