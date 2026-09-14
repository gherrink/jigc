<!-- M51 · the independent pre-decompose design review (Opus `design-reviewer`, read-only, driving the release binary `1.0.0-rc.14` at HEAD `bd348a83`). Written 2026-09-11; persisted here verbatim because it was authored in the planning session's scratchpad. Every finding accepted by the human 2026-09-11 — the amendments are in [settle-record.md](settle-record.md) → Review amendments. -->

# M51 — independent pre-decompose design review

Written 2026-09-11 against `HEAD bd348a83` + the uncommitted M51 planning files. Every
"driven" row below was executed against the **release** binary `target/release/jigc`
(`1.0.0-rc.14`) on `dev/jigc-rig` corpora and on `mktemp -d` scratch repos. No cargo was
run; no repo file was edited.

Reviewed: `completions/artifacts/M51/{settle-record,charter,baseline-ledger,gap-findings}.md`,
the six advocate cases, the new `DECISIONS.md` head entry, `implementation/decisions-pending.md:22-63`,
and the design docs each decision moves.

**Overall:** the Settle is unusually strong on *argument* — the advocate discipline worked and
three forks were genuinely re-posed. The weaknesses are concentrated in one place: **the
decisions that say "reuse the shipped predicate / the shipped mechanism" have not been checked
against the exact new shape**, and four of those claims are false when driven. Two of them
(B1, B4) mean the decided fix does not cover the cell it was decided on.

---

## BLOCKING

### B1 — D3's mechanism cannot see the cell D3 was decided on. `StagedSnapshot` is an *index* probe; `setup`'s commit is a *worktree* pathspec commit. **Driven.**

`completions/artifacts/M51/settle-record.md:185-189` (D3, bullet 3):

> **An in-memory pathspec-scoped `StagedSnapshot` pair** — `decide_carryover` already takes in-memory values … so nothing persists and **`MINT_DOORS` is untouched**.

Three facts, each checked:

1. `crates/cli/src/task.rs:3925` `git_staged_snapshot` runs `git diff --cached --raw -z`. Its
   subject is **the index vs HEAD**. An unstaged worktree edit is invisible to it.
2. `crates/cli/src/setup.rs:1709` commits with `git commit --no-verify -m … -- <paths>`. A
   **pathspec commit takes the worktree contents** of those paths, not the index.
3. `crates/engine/src/finalize.rs:~735` `decide_carryover`'s rule is *"a path is carried iff its
   `(path, blob)` pair matches a snapshot entry — the same path restaged to a **different** blob
   is the task's own work."*

So for the driven cell, the pair produces **zero findings**: before setup, nothing is staged
(empty snapshot); after setup writes and stages, the blob differs from the snapshot, which
`decide_carryover` classifies as *the door's own work*.

Driven, on `dev/jigc-rig bare`, with an edit that was **never staged at all** (sharper than F4's
cell A, and a cell no artifact carries):

```
printf 'user line one\n' > CLAUDE.md; git add CLAUDE.md; git commit -m "add CLAUDE.md"
printf 'user line one\nUNSTAGED WIP SECRET\n' > CLAUDE.md     # index CLEAN, worktree dirty
jigc setup                                                     # EXIT=0
git show HEAD:CLAUDE.md
  user line one
  UNSTAGED WIP SECRET          <-- rode chore(jigc): install jigc workspace config
git status --short              # (empty — the tree ends clean, nothing prompts recovery)
```

**Correction.** D3's predicate is not the carryover predicate. The honest subject for this door
is the one D3's own prose names — *"any of its own paths carries bytes it did not write"* — i.e.
per path in setup's pathspec, compare the **worktree** bytes about to be committed against what
setup itself wrote (or, equivalently, refuse when a path in the pathspec is dirty relative to
HEAD before setup touches it). Either state that predicate, or keep `decide_carryover` **and**
add the worktree axis to it; a `StagedSnapshot` pair alone leaves the driven defect at exit 0.
Also re-word the `MINT_DOORS`-is-untouched claim accordingly — it is true either way, but it is
not what makes the mechanism correct.

---

### B2 — D2's **unborn-HEAD** refusal, at the door D2 puts in its new registry, re-opens a recorded M30 audit decision. **Driven.**

`settle-record.md:125-126` (D2): *"**HEAD detached** · **HEAD unborn** · **an operation in
progress** … `finalize.md:31`'s promise is built"*, and `:134-137`: the new registry *"holds
`jigc setup`"*, with the posture probe as its first consumer and `--force` the single consent.

`crates/cli/src/setup.rs:1617-1625` states the opposite, as a recorded audit fix:

> Require a git work tree — but **DO mint on an unborn HEAD** (a brand-new repo with no
> commits). Setup owns committing its own install footprint regardless of HEAD state (**M30
> audit finding 1**): … if setup skipped the install here, `CLAUDE.md`/`.claude/settings.json`/
> `.jigc/AGENT.md` would be left untracked after the first managed commit.

Driven (`git init` + one uncommitted file, no commits):

```
git init -q . && jigc setup    # EXIT=0
git log --oneline              # 8e345d5 chore(jigc): install jigc workspace config
```

So `git init && jigc setup` — the QUICKSTART on-ramp — becomes a refusal routed at
`jigc setup --force`, training the exact reflex D3 itself prices as the honest weakness, on the
**first command an adopter runs**.

**Correction.** Either exempt `setup` from the *unborn* member with the M30 rationale quoted
(the repo's basis-unchanged form), or state the reversal as a basis-has-changed one with the
M30 finding engaged — but the family cannot be applied uniformly to a registry that holds
`setup`. The detached and operation-in-progress members are unaffected.

---

### B3 — D3 × D10 collide: a derived ambush owe-set + a new setup-side blocking contract with no soliciting step reddens **pack-load at every door**.

`settle-record.md:438` (D10): `AMBUSH_CLASS_CODES` is **derived rather than hand-listed**.
`settle-record.md:443-446` names the constraint and then draws the opposite conclusion:

> `finalize.carried-staged`'s declarer is `step:finalize`, and **no pack step of either pack
> solicits `jigc setup`**, so D3's setup-side carryover contract has no home the fence's own
> route can name today — **which is part of why the owe-set is derived** rather than extended by
> hand.

Deriving the owe-set does not solve that; it **causes** it. `crates/cli/src/pack.rs:803-830`
(`assert_stated_at`): every member of the owe-set must have at least one declarer among **each
manifest-shipping pack's own steps**, each pack checked in isolation. A derived set that picks
up D3's new blocking contract therefore makes both packs fail pack-load — *every* door exits 1
— with no legal declarer, because declaring a setup contract on `step:finalize` is a law-1 lie
and would also have to buy tokens under `CONSTRAINT_REQUIRED_TOKENS` (`pack.rs:1242`).

**Correction.** Decide one of: (a) the derivation's domain **excludes** contracts whose door no
step solicits, and that exclusion is itself stated and fenced (an `Exempt(<reason>)` row, the
repo's standing shape); (b) the fence's subject narrows so a door outside the pack's soliciting
surface is out of scope by construction; or (c) D3's setup contract is deliberately **not** an
ambush-class member, with the reason recorded. As written the two decisions cannot both land.

---

### B4 — D1's reuse claim is **false for the absolute cell** — the Tier-0 cell EC-1 drove. `untrackable_reason` trims the leading `/`. **Driven + doc-cited.**

`settle-record.md:68-70` (D1 part 1): *"The doors ask the shipped predicate.
`trackable::untrackable_reason` + `is_workbench_root` at `jigc migrate <path>` … — three call
sites, shipped predicates, no new capability."* F1's decisive table (`settle-record.md:61-64`)
rests on *"`config set docs-root` refuses all three escape shapes … the predicate is shipped and
`migrate.rs` never asks it"*.

`crates/cli/src/trackable.rs:56` — the first statement of the function:

```rust
pub(crate) fn untrackable_reason(repo_root: &Path, relative: &str) -> Option<String> {
    let relative = relative.trim_matches('/');
```

and its doc-comment types the parameter as *"a **repo-root-relative** path"*. `resolve()` then
joins the trimmed value onto the root, so `/private/tmp/victim/keepme.md` is re-read as
`<repo>/private/tmp/victim/keepme.md` — under the root, no `.git` component, owned by this
repo — and the predicate answers **`None` (trackable)**.

A locked design doc says so in its own words (`design/storage.md:177`, the
`config.unusable-root` bullet): *"the value is resolved against the repository root by
everything downstream (`crate::trackable` **trims the leading `/`**)"*.

And the `config set` door refuses the absolute shape through a **third** predicate D1 does not
name. Driven:

```
jigc config set docs-root /tmp/elsewhere
  blocking · config.unusable-root — … an absolute path …
jigc config set docs-root ../escape
  blocking · config.untrackable-root — … resolves outside the repository root
jigc config set docs-root .git/x
  blocking · config.untrackable-root — … inside git's own directory …
```

So the "shipped predicate" that refuses all three shapes is **three** predicates
(`untrackable_reason` + `is_workbench_root` + `unusable_root_reason`), and the one member of the
trio that answers the *absolute* cell is the one D1 omits. For `migrate <path>` the token is a
real absolute host path (not a knob value), so the door additionally needs a rule that
canonicalizes-and-compares before the predicate is asked — which is a small **new** rule, not a
reuse.

**Correction.** State D1 part 1 as: the door resolves the caller token to a repo-relative path
first (or refuses an absolute one outright), then asks `untrackable_reason` + `is_workbench_root`
+ `unusable_root_reason`'s absolute/symlink legs — and drop *"no new capability"* or scope it to
the three legs that genuinely transfer.

---

### B5 — The recorded warrant for keeping the deny floor's blanket permit is **falsified at HEAD**: after D1's guard, an in-repo **untracked** file is still permanently destroyed at exit 0. **Driven.**

`settle-record.md:86-88` (D1):

> The adapter deny floor **KEEPS the blanket permit** for `migrate` + `finalize --approve` …
> a guarded migrate destroys only a *reviewed, **in-repo, git-recoverable*** file, so the
> asymmetry against `uninstall`/`milestone discard` … is right once part 1 lands.

Both predicates D1 names are **location** predicates. Neither asks whether the source is
tracked. Driven end to end on `dev/jigc-rig fresh` (1.0.0-rc.14, release):

```
printf '# Old History\n\n## 1.0.0\n\n- did a thing\n' > HISTORY.md   # UNTRACKED, in-repo
jigc migrate HISTORY.md --as changelog          # exit 0, task minted
… doc create/add-item/set-slot … (author the canonical changelog)
jigc task finalize <id> --approve
  finalized 8028fae — chore(changelog): adopt HISTORY.md
    promoted CHANGELOG.md
    1 file committed                 <-- the deletion is named on NO surface
ls HISTORY.md                        -> No such file or directory
git log --all --oneline -- HISTORY.md -> (empty: the bytes exist in no git object)
```

The adapter profile's own stated membership rule for the floor
(`crates/cli/adapters/claude-code.yaml:32-34`) is *"each deletes bytes that exist in **no object
DB**, and the `--force` past their refusal is the human's consent to spend them, not the
agent's."* Applied honestly, this cell satisfies it — and there is no refusal to consent past.

**Correction** (choose, and record it): (a) give the guard a **trackedness leg** for the retire
— refuse, or route, an untracked source; (b) keep the permit but restate the warrant without the
*git-recoverable* clause and say what makes the asymmetry right in its absence; or (c) add the
third deny entry (it must sit **above** the profile's comment — see D1's own scraper repair).
Silence here is the by-omission blessing the Settle already refused once.

**Bonus — this also discharges the Settle's own owed re-drive (G-5).** `settle-record.md:101-104`
asks whether the fenced `migration-finalize` step sentence (*"the deletion staged … a `git rm`
in effect"*, `crates/cli/pack/steps/migration-finalize.yaml:10-11` and its methodology twin) is
*"still false for the **in-repo-untracked** cell"*. Driven above: **yes** — nothing is staged,
the removal does not ride the commit, and the ack says `1 file committed` (the promoted
`CHANGELOG.md`). Both packs ship a false statement under a fence that certifies its facts.

---

### B6 — D12 under-specifies the two things that decide whether it closes the hole it was admitted on.

`settle-record.md:487-494` (D12) decides the deregistration arm, a `Human` route, and *"`doc list`
printing the row instead of dropping it"*. Two consequences are unstated and both are load-bearing:

1. **Exit semantics.** D12's admission argument is *"a **false green** on the verb `MIGRATING.md`
   tells adopters to CI-gate on"*. But store-scope content findings are **report-only, exit 0**
   unless the code is a member of `cli::render::STORE_EXIT_FLIPS` (`crates/cli/src/render.rs:909`;
   CLAUDE.md → Code architecture). D12 argues the opposite way — *"a new code plus a route is
   **additive**, not one-way"* — which is the argument for *not* flipping. If it does not flip,
   `jigc validate` still exits 0 and **the adopter's CI is still green**: the decision's own
   justification is undischarged. State the `STORE_EXIT_FLIPS` membership either way, with the
   reason (M42's `schema-conformance.schema-version-current` and M46's
   `schema-conformance.unadopted-instance` are the two precedents, both members).
2. **A third `doc list` `state` value on a 1.0-pinned envelope.** `design/doc-read-surface.md:159`
   declares the enumeration exhaustively: *"carrying its state — **`managed` (stamped)** or
   **`unregistered` (no stamp…)**"*. Driven, `doc list --format json` emits
   `{"id","path","state","item-count"}`. An orphaned instance is stamped (so not `unregistered`)
   and managed by nothing (so `managed` is a law-1 lie) — it needs a **third value**, which is a
   change to a declared enumeration on a pinned contract and therefore owes a declared paragraph
   and a row in D5's registry. Its `id` also names a type no resolved schema defines and its
   best-effort `item-count` has no schema to parse against; both need a stated answer.

---

### B7 — D7 is ambiguous between the two arms of its own fork, and only one discharges F-5's stated harm. No acceptance test can be written from it.

`settle-record.md:346-350`: *"the declared contract is **made true** at the 22 unknown-id doors
… using the engine's shipped `finalize.no-task` and M50 Increment 9's typed `Result<_, Finding>`
… `command-output-contract.md:202`'s false sentence is **corrected** in the same edit."*

Those are two different landings and the docs pull in opposite directions:

- `design/command-output-contract.md:172` (the six declared target forms) lists **`finalize.no-task`
  under the work-unit form `task:<id>`** — i.e. as a finding that *projects a key*. Making *that*
  contract true means these doors emit the **findings envelope**.
- `design/command-output-contract.md:202` is the **flattened-string** paragraph (*"a finding
  consumed by `finding_to_err` projects no key at all"*). Correcting *that* sentence means the
  doors stay flattened and merely carry the code inside the string.

The charter's harm statement is *"a driver keying on the stable `(code, target)` pair gets
nothing at 22 doors"* — which the flattened arm does **not** repair (a code inside a message is
not a key). Driven today, and the divergence is visible one verb apart:

```
jigc --format json task finalize no-such-task
  {"error": "no task `no-such-task` — list live tasks with `jigc task list`"}     exit 1
jigc --format json doc show adr:nosuch
  {"schema_version":3,"findings":[…]}                                            exit 0-envelope
```

**Correction.** Say which shape the 22 doors emit after the fix, in one sentence, and record the
consequence: if it is the envelope, that is a **wire change on 22 verbs** and must be a declared
row of D5's registry (which is fine inside the open window, and is what "every fork resolved
robust" implies); if it is the flattened string, then `finalize.no-task` must leave the keyed
target-form table or gain a stated exception, and F-5's harm is knowingly only half-repaired.

---

### B8 — D5's "**17** undeclared keys" is a wrong count, and it is the size of the table that closes the 1.0 pin.

`settle-record.md:276-279` and the Owed-at-decompose item 4 both size the dispositions table at
**17 keys across 10 verbs**; `DECISIONS.md` repeats it (*"blesses **17** accidents into contract"*).

The source (`completions/artifacts/M51/baseline-envelopes.md:84-90`, relayed verbatim into
`baseline-ledger.md:114-121`) says *"**13** further envelope keys"* and then **enumerates 18**:
`uninstalled · rows · summary · identity · new_path · old_path · prose_mentions · referrers ·
moved · displaced · reslugged · layer · rejected · knobs · anchor · side · step · overlay`.
Counted at HEAD against `design/`:

```
for k in uninstalled rows identity new_path old_path prose_mentions referrers moved \
         displaced reslugged layer rejected anchor side step overlay guide guide_file install_commit; do
  grep -rc "`$k`" design/ ; done      # all 0
```

So the class is **≥ 21** (18 + `guide`/`guide_file`/`install_commit`), not 17 — the `13` in the
prose was already inconsistent with its own list, and the Settle keyed a decision to it.

This matters more than a numeral: D5's whole argument is that *"an undeclared key on a pinned
envelope is a defect, not an addition"* and that the disposition must happen **before** the pin.
A table sized at 17 ships with ~4 keys undisposed, i.e. **blessed by omission on 1.0.0 day** —
the precise failure D5 exists to prevent. It is also the wave whose D8 fences hand-counts.

**Correction.** Write the Owed-at-decompose item as *"one row per driven key that no `design/`
file names, the set **derived** at build time from the driven envelopes"* and strike the numeral
(or bracket it as a dated measurement, per D8's own `dated_correction_spans` discipline).

---

### B9 — Under-specified checks: every refusal this wave mints is missing its surface, severity, route kind and registry identity. Here is the list the build will have to mint.

`Scope confirmed` (`settle-record.md:592-594`) says only *"Every new finding code is registered in
`validation.md`'s inventory … and, where it is a door identity, in `surface-contract.md`'s
namespace. **At least seven codes are at risk of being minted homeless.**"* That names the homes
and no code. Per the review's own rule — an unscoped check is a fork the build resolves for you —
here is the census, `stated` / `unstated`:

| # | mint | code | severity | surface it fires against | route | `ERROR_CODE_REGISTRY`? |
|---|---|---|---|---|---|---|
| 1 | D1 — `jigc migrate <path>` refusal | unstated | unstated | unstated (operational bail vs `Finding`) | unstated | unstated |
| 2 | D1 — the **sink** refusal before `retire` | unstated | unstated | unstated (inside the commit closure — must it roll back the promote?) | unstated | unstated |
| 3 | D1 — `config insert-step` / `replace-step` `<file>` refusal | unstated | unstated | unstated | unstated | unstated |
| 4 | D2 — HEAD **detached** | unstated | unstated | unstated | `--force` (stated) | unstated |
| 5 | D2 — HEAD **unborn** | unstated | unstated | unstated | unstated | unstated |
| 6 | D2 — **operation in progress** | unstated | unstated | unstated | unstated | unstated |
| 7 | D3 — `CarryoverBoundary::Setup` | **owed at decompose (stated as owed)** | owed | owed | owed | owed |
| 8 | D10 — the unstamped-managed-corpus answer | **owed at decompose (stated as owed)** | owed | owed | owed | owed |
| 9 | D12 — the deregistration/orphan finding | unstated | unstated | unstated — and see **B6** (`STORE_EXIT_FLIPS`) | `Human` (stated) | n/a |
| 10 | Scope — `jigc ingest`'s identity leg | unstated | unstated | unstated (does `ingest` **refuse** the adoption, or adopt-and-advise?) | `Human` `git mv` (stated) | n/a |
| 11 | Tier 3 — EC-28's OS-name ceiling at 3 doors | unstated | unstated | unstated | unstated (today: a **false** route) | unstated |

Rows 7 and 8 are correctly carried as *owed shapes*. Rows 1–6 and 9–11 are not owed anywhere.

The sharpest of these is **D2's family**. A refusal at `task finalize` / `milestone create` /
`setup` is either (a) a blocking `Finding` (which puts it in `validation.md`'s inventory, gives it
a `target` form — a work unit? the repo? — and an exit-3 gate), or (b) an operational error with
an `Outcome` identity (which puts it in `surface-contract.md:131-142`'s namespace, whose 11 rows
are **derived from `COMMITTING_DOORS`** by the registry-mirror test — and D2's new registry is a
*superset* of `COMMITTING_DOORS`, so the derivation's subject moves). That is a fork, not a
detail: it changes the exit code, the `--force` route's rendering, the invocation-log field, and
whether the codes join D10's derived ambush owe-set (see **B3**). Decide it at Settle, not in a
plan.

---

### B10 — D10 states a property ("derived") with no derivation source. The acceptance test is unwritable.

`settle-record.md:438`: *"`AMBUSH_CLASS_CODES` is **derived rather than hand-listed**."* Derived
from **what**? The candidate sources are materially different and each fails differently:

- *every blocking finding code* — far too wide (`conformance.*`, `write.*` … none is an ambush
  contract), and pack-load reddens instantly;
- *codes minted at a committing door* — picks up D3's setup code, which has no declarer (**B3**);
- *codes whose first appearance is their own block message* — the honest definition, and it is
  not computable from the code side at all;
- *codes some step already declares* — circular, and toothless by construction.

Without the source, no red test can be written and the build-planner has nothing to decompose.
State the derivation (source set + the exclusion rule + its stated reason per excluded member),
or downgrade D10 to *"hand-listed with a fence that reddens when a door mints a blocking contract
outside the list"* and say so.

---

## SHOULD-FIX

### S1 — The claim is amended for the `Plain` family's honest bound and **not** for the posture family's, while D2 declares a posture member out.

`settle-record.md:32-43`: the claim is *"no … repository posture reaches a door that destroys,
commits or moves without that door having adjudicated it — the two exemptions … are closed **as
classes**"*, followed by an honest bound that covers **only** the `Plain` family.

`settle-record.md:127-132` declares the **`GIT_DIR` redirect out**, with its scope reopenable.
The baseline drove that cell as `milestone create` writing the record into repo A and landing the
commit in repo B at exit 0 (`baseline-ledger.md:101-104`). That is a repository posture reaching
a committing door unadjudicated — i.e. the posture half is **also** a named list, not a closed
class.

Add the symmetric bound to the claim (*"the posture family closes over three of four driven
members; the `GIT_DIR` redirect is declared out with its rationale and its reopenable scope"*).
The charter's own falsifier says the wave must *"ship a named list with a fence over the list, and
say so"* — it says so for one half only, and the asymmetry is exactly the false-completeness shape
this wave was chartered to correct.

### S2 — D2's new registry has a lower bound and one named element, but no membership rule — and three consumers read it.

`settle-record.md:134-138`: *"doors that commit **or move** on the user's behalf — a superset of
`COMMITTING_DOORS` (`COMMITTING_DOORS ⊆ …` asserted) that holds `jigc setup`"*, read by the
posture probe, D3's staged-set guard, and N20's cause vocabulary.

*"or move"* widens the subject past committing: `jigc relocate`, `config set docs-root`/
`placement-root` (which `git mv` the committed corpus — EC-4/EC-16), `migrate-corpus`'s
relocation arm and `rename` are all candidates. Whether the posture probe fires at a
**non-committing mover** is a real decision (N20's cause vocabulary is meaningless there), and
D3's guard is nonsense there. The Owed-at-decompose item asks only for *"the identifier, its home,
and the exact form of its `COMMITTING_DOORS ⊆` assertion and clap-tree ⇔ fence"* — the
**membership predicate** is not owed and not stated. Given M50's lesson, the right answer is
probably a **total** classification of every clap leaf (the `ARG_TOKENS`/`VERB_KINDS` mold), so a
new verb reddens until someone answers. Say which.

### S3 — `--force` at `jigc setup` would carry two different consents from two different decisions.

D2 (`settle-record.md:146`) and D3 (`:183`) each declare *"`--force` is the single consent"*, both
at `jigc setup`, for two unrelated acts (a posture refusal; committing bytes setup did not write).
`jigc setup --help` at rc.14 has **no `--force`**. A user pushed to `--force` by a legitimate
detached HEAD would silently also consent to swallowing their uncommitted work — the
consent-collapse the destroying-door mold exists to prevent. Decide: one flag with a stated
two-part meaning, or two flags, and record it once.

### S4 — D3 strikes `--no-verify`'s recorded basis and keeps `--no-verify`, supplying no replacement basis.

`settle-record.md:196-197`: *"`setup.rs:1603`'s false `--no-verify` rationale is **corrected**,
and the two shipped guides' *'jigc never passes `--no-verify`'* universal is **scoped**."*

The rationale at `setup.rs:1603-1607` is *"the only hook present is the warn-only `pre-commit`
setup just installed"* — driven false by F4 cell B (setup **preserves** a pre-existing hook, then
skips it). The second half of that rationale (*"the hook must not self-trigger on the very commit
that installs it"*) is a reason about **jigc's own** hook and says nothing about the user's.

So the behaviour's basis is gone and D3 records only a wording fix. Under this repo's own
discipline a decision whose stated basis is falsified needs a re-recorded basis or a reversal.
Name the fork — *keep `--no-verify` and scope the promise* vs *run a pre-existing user hook and
`--no-verify` only jigc's own* — and record which, with its ground. (Note the interaction: with
D3's refusal in place, the cell-B secret riding the commit is partly closed anyway, which may be
the right ground — but it has to be *written*.)

### S5 — D5's `--dry-run` equal-set fence is unsatisfiable as stated for blocking previewed members.

`settle-record.md:290-292`: *"the **behavioural equal-set fence** — drive each `Tier::Previewed`
member and assert the emitted `code` set is equal across `task validate` / `--dry-run` /
**landed**"*.

`Tier::Previewed` contains **blocking** members (`finalize.carried-staged` among them). In the
state that makes a blocking member fire, `task finalize` **does not land** — there is no landed
emission to compare against, so the third leg of the equality has no value for exactly the members
that matter most. Restate as: `validate == --dry-run` over every previewed member, and
`== the committing door's emission` (landed *or* blocked), which is the property the contract
sentence actually claims.

### S6 — D8's headline datum is itself a miscount, and it is the *argument* for D8.

`settle-record.md:366-368`: *"CLAUDE.md's *'thirteen `CELLS` rows'* was **63** when written and is
**64** at HEAD — it moved again between the evidence check and this Settle: a 100 % defect rate
with a **half-life shorter than the wave**."* `DECISIONS.md` repeats *"is 64 today"*.

At HEAD:

```
awk '/pub const CELLS/,/^\];/' crates/cli/tests/support/write_miss_cells.rs | grep -c "^    Cell {"
63
grep -n "struct Cell" crates/cli/tests/support/write_miss_cells.rs
108:pub struct Cell {          # the 64th "Cell {" match is the struct declaration
```

The count is **63**, unchanged since the evidence check. The decision (fence, don't rewrite)
stands without this datum, but the record would ship a false count inside the decision about false
counts, and `DECISIONS.md` is where the next wave will cite it. Strike it with the datum, per the
repo's own rule.

### S7 — D13 removes the recorded ground for excluding F-3/F-8 and replaces it with a leg the razor does not have.

`settle-record.md:502-513` (D13) establishes that *"additive after the pin"* **inverts** for a
shape change to an existing frozen doctype — a bump today runs over zero adopter corpora, the same
bump after 1.0.0 over every adopter's store. The human's criterion, quoted in the charter, is
*"everything that becomes impossible or **expensive** to change once people start using the
product."* The razor's leg 0 is that criterion codified: *necessary = a one-way door at the pin,
or a hole in a declared surface.*

D13 then keeps the exclusion on *"the **necessity** leg — **no adopter needs them**"*. That is a
different test from leg 0 as written, and it is the one leg the razor does not contain. It also
dissolves the M50 precedent D13 leans on: M50 refused the same edge partly because
*"`AddedOptionalField` makes the edge a one-byte-line 1.1 addition"* — the premise D13 has just
falsified.

Either add the leg explicitly (*"leg 0b — needed by a real adopter today; a cost that is expensive
but bounded and trigger-recorded is deferrable"*) and show it can refuse, or re-examine F-8 on the
corrected ground. As written a future wave inherits a razor whose refusal is unciteable.

### S8 — Two in-scope charter rows have no disposition anywhere: **EC-28** and **N15**.

The charter admits *every* EC row and the fix-shaped F/N rows. Searching the Settle record:
`EC-28: 0` mentions, `N15: 0` mentions. Both are more than wording:

- **EC-28** is a **3-door class with a false route** (`baseline-ledger.md:222-232`): `doc rename`,
  `doc create` and `start … --slug <300>` all fail on the OS name ceiling and answer
  `task.working-area-io` routed *"resolve the underlying I/O condition (a disk or permissions
  problem…)"* — a law-1 lie on a door the wave's claim covers. G-57 poses the decision (does
  `SLUG_DOORS` gain a filesystem-length predicate, with a code and a route?) and nothing answers it.
- **N15** is Tier 1, on the **1.0-pinned read surface**: a `--task` read of an address resolving
  nowhere is byte-identical to the task-less one, says *committed*, and its route **drops
  `--task`**. Its fix touches a pinned contract's route/value and therefore wants a row in D5's
  registry.

Record a disposition for each — even *"rides its tier as chartered, shape obvious from
`store.not-staged`"* — so neither is admitted or dropped by silence.

### S9 — D3's widened carryover sentence would overclaim, and the two shipped guides that state the gate's subject are not in the doc-consequence set.

`settle-record.md:192-195` widens the subject *"from **task-minting door** to **any door
committing paths it does not own**"* at `surface-contract.md:123` and `finalize.md:159`. As a
universal that is false at HEAD for every committing door that does **not** snapshot
(`jigc rename`, `migrate-corpus`, the milestone record-only doors). Scope the widened sentence to
the registry D2 mints, or the repair ships a fresh law-1 overclaim in the wave whose Tier 2 is
law-1 overclaims.

Also missing from the doc-consequence list and from G-61's consolidated adopter-doc batch:

- `MIGRATING.md:38` — *"**Minting a task** snapshots the git index, and at finalize anything
  staged *before the task existed* refuses…"*
- `QUICKSTART.md:176` — the same gate, same subject.

Both ship inside `SKILL.md` by `include_str!`, and D3 adds a refusal at **the first command an
adopter runs** with no guide sentence. Add them to the single batched hash move.

### S10 — D15: the coverage fence has a free escape, "cell matrix" is undefined, and whether a flow-52 acceptance chapter/suite ships is unanswered.

`settle-record.md:547-551`: *"**Eight axes**, each a cell matrix driven end to end … under the
review's own fence: every leaf in `VERB_KINDS` appears in at least one axis's cell matrix, **or is
named uncovered**."*

1. `VERB_KINDS` (`crates/cli/src/cli.rs:1412`) is a real total registry, so the subject is sound.
   But *"named uncovered"* carries **no reason requirement**, so the fence is satisfiable by
   listing all 47 leaves as uncovered. Every sibling exemption in this repo carries one
   (`Snapshot::Exempt(<reason>)`, `Unpinned(<reason>)`, `suppressed: {reason, expires}`,
   `is_route_exempt`'s enumeration). Require `uncovered(<reason>)`.
2. *"cell matrix"* is undefined, so *"a verb appears in an axis's matrix"* is unfalsifiable. Give
   it the minimum the gate cell needs: an axis's matrix is a written table of `(door, cell)` rows
   whose cells are driven, and a verb is covered iff it is the door of at least one driven row.
3. Every wave since M40 has landed its acceptance as a `worked-examples.md` flow chapter + a
   `flowNN_acceptance.rs` suite iterating a code-side set. D15 decides the *review* and says
   nothing about the flow — while `gap-findings.md` G-6 already presumes one (*"whether the
   **flow-52** chapter may restate the claim without the fifth family"*). Say explicitly whether
   M51 ships a flow-52 chapter + suite, because the decompose will otherwise have to invent the
   answer, and the review — unlike a suite — leaves nothing standing behind it.

### S11 — D1 pre-commits `insert-step`'s `<file>` to a **destination** predicate, which is the wrong question for a source and narrows a shipped affordance.

`settle-record.md:68-70` puts `config insert-step/replace-step <file>` under
`untrackable_reason` + `is_workbench_root`, while `settle-record.md:76-79` (part 3) says the
registry carries *"one stated rule **per member**"*. Part 1 answers `file` before part 3 gets to
state its rule, and the answer has two visible costs:

- `untrackable_reason` refuses *"resolves outside the repository root"*, so a source step file
  kept **outside** the repo (a shared team steps library, `~/steps/foo.yaml`) stops working. The
  driven harm was reading `.git/config` and rendering it into the composed step text
  (`baseline-ledger.md:91-95`), not out-of-repo-ness as such — the verb's help says only *"The
  source step file; its basename becomes the native step id"*, and the file is copied in either way.
- `is_workbench_root` refuses any path whose first normalized component is `.jigc` — which is
  precisely where `insert-step` **writes** (`.jigc/config/steps/<basename>.yaml`), so passing an
  existing step file from there is refused by the guard.

State `file`'s rule as a **source** rule in part 3 (readable · no `.git` component / not inside
git's dirs · not reached through the workbench) and say whether an out-of-repo source is refused,
with the reason. (Minor, same family: `is_workbench_root` is a private `fn` in `config.rs:622`
and `engine::validate::schema_version_from_front_matter` is `pub(crate)` — both need a visibility
change to be reused as D1/D12 describe; worth a line so "no new capability" stays accurate.)

---

## ADVISORY

### A1 — D4's capture subject and its "one capture point covers every arm" claim.

`settle-record.md:232-236` decides *"capture before `gitignore::ensure` and the version refresh,
restore at the existing failure site — **one capture point, one restore point**, both already
there, covering **every arm** including the fan-out `--ff-only`"*.

Checked: inside `task.rs`'s commit closure, `crate::gitignore::ensure(jigc_root)` sits at
`task.rs:2778`, above both `refresh_version_stamp` call sites (`:3086`, `:3429`), so one worktree
capture placed just above `:2778` does precede both — the claim holds **for the finalize arms**.
Two riders:

- `gitignore::ensure` has **three other production callers** outside that transaction:
  `adapter.rs:1108` (setup), `milestone.rs:472` (`milestone create`, a committing record-only door
  with its own rollback family) and `milestone.rs:2066` (`milestone provision`, which **never
  commits** — so on an `ENTRIES` upgrade a user's private lines die there with no transaction at
  all). EC-18's amend-to-union closes the loss for all four, which is why this is advisory rather
  than blocking — but *"covering every arm"* should read *"covering every finalize arm; the other
  three callers are covered by the amend, not by the capture."*
- The **subject** wants naming. `jigc_config_layer_pathspecs()` is three specs, one of which
  (`.jigc/config`) is a **directory**; a worktree pre-image family over a directory is a different
  shape (absent-means-delete over N files). The files finalize actually rewrites are
  `.jigc/.gitignore` and `.jigc/version`. Say *those two*, or the build will capture a directory.
- `RecordPreImage` (`milestone.rs:705`) is module-private and single-path; D4 transfers its
  *discipline*, not the type. Fine as written — worth one word so a plan does not go looking for a
  reusable primitive.

### A2 — D9 corrects two fields of `measurement.md:62` but leaves the sentence its own decision falsifies.

D9 declares the log **UNVERSIONED**. `design/measurement.md` (closing line of the section) reads:
*"where it and the dogfood hook's v1 JSONL schema overlap in fields, they are **independently
versioned surfaces** — no shared format is implied or required."* D9's reading of that phrase
(independent *of each other*) is correct and I verified it in context — but after the declaration
the phrase still says *versioned* of a surface now declared unversioned. Move it in the same edit
(e.g. *"independently governed surfaces; the invocation log is declared unversioned and
additive-only, with `binary_version` the per-record discriminator"*).

### A3 — D8 has no mechanism for the count claims with no code-side set, and its own instruction will halt a fixer.

D8's rule is *"fenced, not rewritten"*, and the fences it mints are concrete (`ManifestKind::ALL`,
the triage-report destructure, `COMMITTING_DOORS` + the `ERROR_CODE_REGISTRY` mirror,
`foldback_truth`'s re-key). But some EC-14 members have **no code-side set to fence against** —
`design/doc-read-surface.md:175` heads *"five regimes"* over a six-row table; the dev/methodology
manifest headers restate one historical fact in two files. A fixer told *"fence, don't rewrite"*
has nothing to reach for. Add the residue class to D8: *counts over sets the code cannot move are
corrected in place (and historical ones dated-bracketed)*, with the one-line reason.

### A4 — D11 (and D8's `foldback_truth` half) never answer razor leg 2.

The razor's leg 2 is *"Right for **any adopter** — if the only beneficiary is our layout, our
naming or our process, it is refused."* A fence asserting that CLAUDE.md's fold-back paragraph
names the same version as `Cargo.toml` benefits this repo's **process** only; the charter excludes
*the harness-surface wave* on exactly that ground (*"build infrastructure, reaching no adopter"*),
and M50 excluded it the same way. D11 argues only the foreclosed-by-doc narrowing
(`foldback_truth.rs:223`) and never leg 2 or leg 0. The answer is probably *"the human's boundary
admits every ledger row, EC-10 included"* — which is fine, but it should be **written**, because
D13 makes razor consistency this wave's own subject. (D8's `COMMITTING_DOORS` half is clean: it
sweeps `MIGRATING.md:39`, which ships.)

### A5 — D10 and D12 both promise a store-surface answer for adjacent populations; name the partition.

D12 detects *"a committed `.md` **carrying a jigc stamp** at a home no resolved doctype claims"*.
D10 owes *"a store-surface answer for an **unstamped managed corpus**"*. The two populations are
complementary and the user-visible condition is nearly the same (*jigc's store surface is silent
about a file it should speak for*). Say at decompose whether they are one finding with two causes
or two findings, and which surface each fires on — otherwise the plan mints two codes for one
condition, or leaves the seam between them open.

### A6 — D5 does not say whether the pinned-envelope registry is production or test-side, and the two differ in what can read it.

Owed-at-decompose item 4 asks for *"which file the registry lives in"*, which reads as a filing
question. It is not: a **test-side** registry (the `text_json_parity_axis::REGISTRY` /
`format_json_success_axis::recipes` mold) can be cited by a design doc but cannot be read by the
binary; a **production** one can feed `doc schema`-style introspection and the contract doc's own
fence. D5 also says *"the doc's *'three surfaces'* section **names the registry** as the list"* —
which makes a locked 1.0 contract doc point at the artifact. State the side and the consequence.

*(Checked and sound: reshaping `format_json_success_axis`'s recipe table to `(path, arm)` does not
weaken the duplicate-path guard — `crates/cli/tests/format_json_success_axis.rs:918` is
`assert_eq!(from_recipes.len(), from_clap.len())` after a set-containment bijection, so the
reshape needs "distinct paths bijective with the clap leaves **and** `(path, arm)` pairs unique".
That is mechanical, and the Settle correctly carries it as owed.)*

### A7 — D5 deletes a key from a shipped envelope; no posture paragraph authorizes a pre-pin **removal**.

*"`hook_output` on the read verb `milestone list-tasks` is **deleted**"* (`settle-record.md:278`).
Both posture homes authorize **additive** keys pre-1.0 (`design/doc-read-surface.md:88`;
`command-output-contract.md` → Evolution posture), and D6's new policy governs *inside 1.x*
(*"nothing pinned is removed or reshaped"*). Nothing states the rule for a removal **before** the
pin. Since D6 is writing the policy paragraph anyway, have it say the pre-pin rule explicitly —
it is the sentence that authorizes D5's own deletion, and the wave should not rely on an
unwritten one.

### A8 — D7's doc edit, stated precisely.

Minor but worth pinning in the plan: `command-output-contract.md:202`'s sentence is false in two
distinguishable ways — the **behaviour** (the 22 doors do not carry the code) and the
**provenance** (*"since M50 Increment 10"*, which is what makes it read as settled). D7 says the
sentence is *"corrected in the same edit"*; say which half the fix makes true and which half is
struck with its datum, so the edit is not a silent rewrite.

---

## What I checked and found sound (so the plan does not re-litigate it)

- **D9's reading of `measurement.md:75`** — *"independently versioned surfaces"* does mean
  independent **of each other**; the surrounding sentence says so explicitly. The two field
  corrections at `:62` (`duration` → `duration_ms`; `finding_codes` always present) are right —
  driven key set: `['argv','binary_version','duration_ms','error_code','exit_code','finding_codes','output_bytes','timestamp']`.
- **D7/G-17's core premise** — `design/command-output-contract.md:172` does declare the work-unit
  target form and does list `finalize.no-task` as a member, so the M50 `(code, null)` precedent
  genuinely does not transfer. (What is unsettled is only which shape the fix lands — B7.)
- **D5's duplicate-path guard concern** — real, mechanical, correctly carried as owed (A6).
- **D1's scraper repair** — `crates/cli/tests/e2e_audit.rs:265-278` does `break` on the
  profile's comment line, and the two M50 deny entries
  (`crates/cli/adapters/claude-code.yaml:35-36`) sit **below** it: 20 of 22 scraped, confirmed by
  reading both files.
- **EC-18's subject** — `crates/cli/src/gitignore.rs:44-53` does `fs::write(&path, ENTRIES)`
  (whole-file replace) against its own doc-comment's *"amended once to the union"*. Four
  production callers, as recorded.
- **D8's pack-step count** — 30 dev + 39 methodology = **69**, exactly the number
  `foldback_truth.rs` bans. Confirmed.
- **D4's capture ordering** — `gitignore::ensure` at `task.rs:2778` does precede both
  `refresh_version_stamp` sites, so "one capture point" is achievable on the finalize arms (A1
  narrows the claim, it does not refute it).
- **The freeze bound** — I found nothing in D1–D15 or the Scope-confirmed batch that moves a
  doctype schema shape, a `location`/`placement`, or a `schema-hash`. The *"zero schema-hash
  movement, zero `schema-version` bumps, zero corpus migrations"* bound holds as far as I can
  check it. (The pinned-*contract* movements are a different matter — see B6, B7, A7.)
