# robust-advocate — `r1-setup-installs-outside-work-tree` (run `canary-one`, round 1, stage `test`, attempt 1)

**The proposal below was driven.** It was spiked in a scratch clone of the candidate's commit
(eeffe347), taken through the full gate there (`GATE: PASS`, 4968 passed, 0 failed) and through
every next step listed under *Driven*; what was not driven is listed under *Undriven*, with why.
Nothing in the repository was edited: the one file I wrote is this report.

One fork: the finding is confirmed and contested, and the cheap cut is to leave `jigc setup` as
it is in the layouts whose home is no work tree, at its recorded trigger (M57, after the 1.0.0
call). My charter is the other side. I am one-sided on purpose and held to what I drove.

## Verdict in one paragraph

**`robust-now`.** The fork was raised on one door and one looping route. Driven past that door,
on the candidate and on the previous release alike, these layouts are not *partial*: `jigc`
lands **one** doc there and then stops. The second doc's finalize is blocked on the first doc as
*missing* (exit 3) with a route whose arms do not lead out; `jigc doc list` says *no committed
docs* over a doc it just committed; `jigc validate` exits 1 with six blocking findings, five of
them about files that were never there; `jigc milestone create` and `jigc uninstall` refuse; and
**an untracked file of the user's at a doc's home is destroyed at exit 0** by `jigc task
finalize` — in no file and in no git object afterwards — where the same walk in a plain checkout
refuses and keeps it. All of it has one cause, in one function: `cli::repo::jigc_home` resolves
the home to `dirname(git-common-dir)` whether or not a checkout stands there. Holding that
candidate to the question the fix pass already built for these same layouts (*is it a checkout —
asked of git*) and falling back to the standing checkout makes all four layouts the ordinary
single-checkout case: 76 of 78 step outputs of a 39-step walk are then byte-identical to what
the candidate prints in a plain checkout. The product half is one file, about 85 added lines,
no new stored file, registry, flag or finding code. It takes the decision the record holds
open — it treats the layouts as supported — so it is the human's to accept, and a standing
parity test flips with it. I recommend he accepts it before the call, and I show below why the
other two answers cost more later than now.

**A caution on what this run is.** `completions/artifacts/canary-one/opening.md` says the run
is a synthetic canary that closes nothing. I argued the fork as I would in a run that does: the
binaries, the record and the behaviour are real.

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

Every *before* in this report is one of those two, its directory first on `PATH` (the walk
script prints `command -v jigc` when it switches binary). **Every *after* is a spike**: a debug
build of my scratch clone with the proposal applied, sha256 `2ff0072d…260e63`, copied to
`<W>/bin2/jigc`. It is called *the spike* wherever its output is quoted, and it is nobody's
candidate.

## How it was driven

- `<W>` is my own directory under the scratch root, minted with `mktemp -d`:
  `<scratch>/advocate-setup-outside.Lde4Zs`. Every root below is a fresh `mktemp -d` under it.
  No rig: its states do not include these layouts.
- Environment of every root: `HOME` a fresh directory inside the root, `GIT_CONFIG_NOSYSTEM=1`,
  `GIT_CONFIG_GLOBAL=/dev/null`, a synthetic author and committer. git 2.54.0, macOS.
- Every exit was read bare (`cmd > out 2> err; rc=$?`), never through a pipe.
- Two small scripts of mine do the walking, so that one sequence is typed once and run on three
  binaries: `<W>/tools/walk.sh <layout> <root> <phase…>` (layouts `plain`, `linked`, `bare`,
  `behind`, `sep`, `sub`) and `<W>/tools/loss.sh <bindir> <root>`. Each step's stdout and stderr
  are kept under `<root>/ev/`.
- The spike: `git clone` of the working clone into `<W>/spike`, detached at eeffe347, the
  proposal applied, `cargo build -p jigc`, then `dev/gate` there. A second clone, `<W>/spike2`,
  built the binary the walks used; its `crates/cli/src/repo.rs` is byte-identical to the gated
  one (`cmp`, exit 0). The spike had two earlier forms, each named where it is quoted (rows 2
  and 7 of *Driven*); every other *after* is the final one.

The layouts, as built:

| name here | construction | the checkout | what `dirname(git-common-dir)` is |
|---|---|---|---|
| `bare` | `git init --bare store.git`; `git -C store.git worktree add ../wt -b main`; an 80-byte `CLAUDE.md` beside them | `proj/wt` | `proj/` — a folder in no repository |
| `behind` | `git init --bare .git`; `git -C .git worktree add ../wt -b main` | `proj/wt` | `proj/` — holds a bare `.git`, no work tree |
| `sep` | `git init --separate-git-dir <abs>/proj/store/sep.git work` | `proj/work` | `proj/store/` — in no repository |
| `sub` | `git submodule add <abs>/lib vendor/lib` in `super`, which has a 41-byte `pre-commit` hook of its own | `super/vendor/lib` | `super/.git/modules/vendor/` — inside the superproject's git dir |

## What the fork did not have: these layouts are one doc deep

The walk: `setup`, `setup` again (the printed route), a `record-decision` task that creates and
finalizes an ADR, the reads, a second such task, `milestone create`, `uninstall`. **Candidate
c1; the previous release answers the same in every cell of this table** — each was driven on
it, the `doc list` cell of `behind` excepted.

| step | `plain` | `bare` | `behind` | `sep` | `sub` |
|---|---|---|---|---|---|
| `jigc setup`, first | 0 | **1** | 0 | **1** | 0 |
| `jigc setup`, the route as printed | 0 | **1**, same refusal | 0 | **1**, same refusal | 0 |
| where the install is written | the checkout, committed | `proj/`, in no repository; `proj/CLAUDE.md` 80 → 119 bytes | `proj/`, beside a bare `.git` | `proj/store/`, in no repository | `super/.git/modules/vendor/`, and the **superproject's** `pre-commit` rewritten |
| first doc: `jigc task finalize` | 0 | 0 | 0 | 0 | 0 |
| `jigc doc list` right after | lists `adr:cache` | *no committed docs* | *no committed docs* | *no committed docs* | *no committed docs* |
| `jigc validate` | 0 | **1** — six blocking findings | **1** — six | **1** — six | 0, one advisory calling the doc missing |
| second doc: `jigc task finalize` | 0 | **3** | **3** | **3** | **3** |
| `jigc milestone create` | 0 | **1**, a bare git error | **1** | **1**, a bare git error | **1** |
| `jigc uninstall`, no task open | 0 | **1** | **1** | **1** | **1** |
| an untracked file at the doc's home, then the first doc | refused, exit 3, file kept | **exit 0, file destroyed** | **exit 0, destroyed** | **exit 0, destroyed** | **exit 0, destroyed** |

What the refusals say, candidate, paths shortened:

- **The second finalize** (all four layouts), with `docs/decisions/cache.md` on disk and in
  `HEAD` of the checkout:

  ```text
  blocking · reconciliation.rename — tracked managed doc adr:cache (docs/decisions/cache.md) is missing
    at: docs/decisions/cache.md
    route: restore docs/decisions/cache.md, or confirm the deletion by dropping it from the index: `jigc unmanage docs/decisions/cache.md`
  ```

  The first arm restores a file that is there. The second, typed as printed: in `bare` and `sep`
  it exits 1 with one line and no code or route — `` `git rev-parse --verify -q HEAD` failed:
  fatal: not a git repository (or any of the parent directories): .git `` — and the finalize
  re-run exits 3 again, as does a third task's. In `sub` it exits 0, saying *there was no file
  at docs/decisions/cache.md to leave on disk* about a file that is on disk, and the finalize
  then lands — so every doc after the first is paid for by un-managing a healthy one, and
  `jigc doc list` still says *no committed docs*. (In `behind` I typed that arm on the previous
  release only: exit 0, and the finalize then lands.)
- **`jigc validate`** in `bare`, `sep`, `behind`, after one clean finalize: the same
  `reconciliation.rename`, and five `schema-conformance.home-vacated` findings — *a declared
  home jigc committed into has been vacated* — for `CHANGELOG.md`, `VISION.md`,
  `docs/decisions-log.md`, `docs/deferral-ledger.md` and `docs/roadmap.md`, none of which ever
  existed, each routed at *restore the document … and commit it*. (Read in `bare` and `sep`;
  counted, six, in `behind`.)
- **`jigc milestone create`**: `bare`, `sep` — the same bare `git rev-parse` line, exit 1;
  `sub`, `behind` — `` `git add -- docs/milestone-records/ship-it.md` failed: fatal: this
  operation must be run in a work tree ``, then a rollback notice.
- **`jigc uninstall`**, with no task open: `uninstall.untracked-workbench-file` — *cannot check
  `.jigc/` for files no index has a copy of* — routed at *make sure `git` is on PATH and the
  `.jigc/` tree is readable, then re-run*, both of which already hold. Its other arm,
  `jigc uninstall --force`: exit 0 in `sub`; **exit 1 in `bare` and `sep`**, at
  `uninstall.remove-precommit`, after `.jigc/` is removed, with the route *ensure the repo's git
  hooks directory is writable, then re-run `jigc uninstall`* — re-run in `bare`: exit 1, the
  same line, and `proj/.claude/settings.json` and `proj/.claude/skills/` are left in a folder
  that is no repository. (`behind` was not typed with `--force`.)
- **The finalize's own ack** in all four: *committed in the linked worktree at `wt` … — not in
  the main checkout jigc's workbench binds to*. There is no main checkout.

**The loss.** `<W>/tools/loss.sh`, the `bare` layout: after `jigc setup`, a file
`docs/decisions/cache.md` is written in the checkout and not added — one line carrying a marker.
A before-control finds the marker in one file under the root. Then `jigc start --workflow
record-decision`, `jigc doc create adr --title Cache`, the slots, `jigc task finalize`: every
step exit 0, the ack *promoted docs/decisions/cache.md*. Afterwards the marker is in **no file
under the root** (`command grep -rl`, the same command as the control) and in **no git object**
(every blob of `git cat-file --batch-all-objects`, read and searched). Candidate and previous
release, the same. The same walk on `sep`, `behind` and `sub`, both binaries: finalize exit 0,
zero files and zero objects holding the marker afterwards. In a plain checkout the candidate
refuses that finalize (exit 3, `conformance.section-missing`) and the file is intact.

Why: `cli::task::git_home_claims` asks nothing where the home is no checkout — its doc comment
says so, citing these three layouts — so the guard behind *a home git holds is occupied* is off
there by construction, and the promote planner stats the destination under the resolved home,
where nothing is.

**The regression fact, for everything above: `regression: false`.** Every cell I drove on the
previous release answers as the candidate does.

## The robust position

**The hole.** The second clause is *a working product others can use and rely on*. A reader in
a submodule types `jigc setup`, gets exit 0 and *adapter installed*, lands a doc, and cannot
land a second; in a worktree of a bare repository the very first command exits 1 after writing
seven files into a folder that is no repository, and no jigc command takes them back. The
printed refusals of four doors there — `setup`, `task finalize`, `uninstall`, `uninstall
--force` — route back into themselves or at a command that fails. And one path is not a
refusal at all but a loss at exit 0, which is the first clause's subject, not the second's.

**The one-way-door tell: each answer to (D) is cheapest before the call, and dearer after it.**
`implementation/decisions-pending.md`, the M57 list's row on these layouts, owes *whether these
three layouts are supported — and if they are not, a refusal that says so at `setup`, rather
than doors that half-work*. So the record names two answers, and **leaving it as it is is
neither of them**: it is the half-working doors the row itself says are not to stand.

- *Unsupported, said at `setup`.* Today that refusal removes little, because nothing past one
  doc works. After 1.0.0 the regression rule ratchets it: `jigc setup` exits 0 in a submodule
  and behind a bare `.git` on the release, and *what succeeds on the previous release's binary
  in a layout must succeed on the candidate* (`DECISIONS.md`, 2026-10-06, the regression set's
  part 2). A refusal introduced at 1.x breaks a success path of 1.0.0 in a layout that ruling
  names.
- *Supported.* Today nobody has a working install at the old home to carry over: the driven
  population is installs that landed one doc and wedged. After 1.0.0 every adopter who stayed
  in such a layout has an install under `<super>/.git/modules/…` or beside a bare repository,
  and moving the home is a store-location migration on a released major.

**It is on the committed trajectory, by the human's own word and the design's.**

- The ruling on the regression set (`DECISIONS.md`, 2026-10-06, part 2) lists *a worktree of a
  bare repository · a separate git directory · a submodule* among *the nine ordinary
  configurations and layouts* in which the 61 success paths are run on both binaries. That
  instrument is owed before the first stabilization run can close, and its fixtures for three of
  its nine states cannot honestly be built on a `setup` that exits 1 and a store that holds one
  doc: of the doors driven here, `milestone create`, `uninstall`, a second finalize and a true
  `doc list` all fail there on the previous release too, so over those the instrument would be
  green on nothing.
- `design/storage.md` defines the term: *jigc_home — the main checkout
  (`dirname(git rev-parse --git-common-dir)`)*. The noun is the design; the expression is an
  implementation of it that is false in exactly these layouts. `design/assistant-adapter.md`
  holds the hook install to *resolve the real hooks dir*; asking git at a directory that is no
  repository is the opposite.
- `VISION.md`'s storage invariant — plain files humans review and edit through git — is not met
  by an install git cannot see: `proj/.jigc/`, and a `CLAUDE.md` appended to in a folder that
  is no repository, have no diff, no commit and no revert.

**Where the record leans the other way, said plainly.** The first clause's scope sentence names
four ordinary configurations and these are not among them; the declared-bounds row of
`decisions-pending.md` lists *the layouts whose home is no work tree* among bounds still to be
written down; and (D)'s trigger is after the call. The record does not settle it. What I add is
what was driven after that record was written.

## The two stock rationalizations

- ***Pre-existing, no regression* is not *not needed*.** The clause's second measure — *every
  refusal's route works as printed* — carries no regression term, and the loss is the first
  clause's. And the record that deferred this was built on a reading, then corrected, and is
  still short: the row's first text said jigc is unusable there, its correction of 2026-10-05
  said the previous release *creates and finalizes a doc* in all three. That is true, and one
  doc deep. The fix pass's own plan marks the bare-repository row *read*, with a drive owed
  (`completions/artifacts/M55/fix-pass-rc25/planning/plan-linked-worktree.md`).
- ***Any fix takes the decision* is why this is a halt, not why it waits.** And it is not
  premature generality: no new mechanism is needed. The predicate *is this directory a checkout
  of this repository — asked of git* was built by the fix pass for these same layouts
  (`cli::repo::home_is_a_checkout`, `DECISIONS.md`, the round-4 entry, Area B); the proposal
  asks it one call earlier, where the home is resolved, instead of at two guards afterwards.
- ***Not provable yet.*** It is proved on a synthetic case, below, on every door I named.

## The cost of the cheap cut, priced

- **A 1.0.0 that loses an untracked file at exit 0** in layouts the regression ruling calls
  ordinary — or a declared bound whose written reach has to read: *anyone working in a
  submodule, a worktree of a bare repository or a separate git directory; `setup` reports
  success in two of the four constructions; the second doc cannot be landed; a file at a doc's
  home is replaced*. A bound has to be written with its reach; this is its reach.
- **Every round re-finds it.** Three reporters of this round's one door returned it (the
  driver's DL-3, the source's L1, the reconciler's RC-6), then a triage row, a verifier and this
  case. The review rows run every printed route, so they will keep arriving at these four doors.
- **M57 inherits both arms at their dear price** (the tell above), and whichever it takes, the
  guards the fix pass wrote *around* the wrong home stay in the tree as special cases:
  `InstallSubject::NoWorkTree`, the skip in `git_home_claims`, the silence in
  `CodeOnlyCheckout::differing`, each with a paragraph explaining three layouts.
- **Saved now:** one fix cycle on one function, and one ruling.

## The proposal — the robust scope

**One rule, in `cli::repo::jigc_home`: a home is a checkout, or it is not the home.**

1. `.git` is a directory (a main checkout, a fake-`.git` fixture): the walk-up root. *Unchanged.*
2. `.git` is a file and `dirname(git-common-dir)` **is a checkout of this repository** — the
   top of a work tree, with the same common git dir, asked of git at that directory: that
   directory. *This is every ordinary linked worktree and every fan-out worktree of an ordinary
   repository: unchanged.*
3. Otherwise, the standing checkout is jigc's **own fan-out worktree** — its path is the one
   `engine::milestone::worktree_path` mints, `<home>/.jigc/worktrees/<id>`, and `<home>` is a
   checkout of this repository by the same question: `<home>`. *The sibling cell: without it
   the join breaks — driven, row 7.*
4. Otherwise: the standing checkout. *The four layouts.*

A probe git cannot answer reads as the standing checkout, the direction the function already
took when git could not name the common dir.

### As a change to a clone of eeffe347

The whole change is three files, `3 files changed, 389 insertions(+), 62 deletions(-)`. Its
bytes are at `<W>/proposal.diff` (sha256 `6d536acd6efbe63e78cde5f00bc3d1b9f2a1015d547ee213d9054b79a84ea74d`,
made by `git diff` on eeffe347) for as long as the scratch root stands; the gate in row 1 ran
on exactly that diff. **Should the file be gone, the blocks here are the proposal** — they are
whole functions, so they are applied by replacing text, not by matching context.

**(A) `crates/cli/src/repo.rs`, the product half.** In `jigc_home`, the last line
`git_common_dir_parent(&repo_root).or(Some(repo_root))` becomes:

```rust
    let Some(common) = git_common_dir(&repo_root) else {
        return Some(repo_root);
    };
    if let Some(main) = common.parent()
        && is_a_checkout_of(main, &common)
    {
        return Some(main.to_path_buf());
    }
    Some(provisioning_checkout(&repo_root, &common).unwrap_or(repo_root))
```

`git_common_dir_parent` is renamed `git_common_dir` and returns the directory itself — its last
line `Path::new(&common).parent().map(PathBuf::from)` becomes `Some(PathBuf::from(common))`.
Two functions are added beside it:

```rust
/// Is `dir` the top of a work tree of the repository whose common git dir is `common`?
/// One question to git, asked at `dir`. A probe that cannot answer reads as `false`.
fn is_a_checkout_of(dir: &Path, common: &Path) -> bool {
    let real = |path: &Path| path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let Some(out) = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args([
            "rev-parse",
            "--path-format=absolute",
            "--show-toplevel",
            "--git-common-dir",
        ])
        .output()
        .ok()
        .filter(|out| out.status.success())
    else {
        return false;
    };
    let said = String::from_utf8_lossy(&out.stdout);
    let mut lines = said.lines().map(|line| real(Path::new(line)));
    matches!(
        (lines.next(), lines.next(), lines.next()),
        (Some(toplevel), Some(at_dir), None) if toplevel == real(dir) && at_dir == real(common)
    )
}

/// The checkout a jigc fan-out worktree was provisioned from, read off the path
/// `engine::milestone::worktree_path` mints and confirmed by git. `None` otherwise.
fn provisioning_checkout(repo_root: &Path, common: &Path) -> Option<PathBuf> {
    let id = repo_root.file_name()?.to_str()?;
    let minted = engine::milestone::worktree_path(id);
    if !repo_root.ends_with(&minted) {
        return None;
    }
    let mut home = repo_root;
    for _ in minted.components() {
        home = home.parent()?;
    }
    is_a_checkout_of(home, common).then(|| home.to_path_buf())
}
```

And the body of the existing `home_is_a_checkout(home, standing)` becomes one line on the same
implementation, its meaning unchanged:

```rust
    git_common_dir(standing).is_some_and(|common| is_a_checkout_of(home, &common))
```

Plus the doc comment on `jigc_home` that states the rule above. Nothing else in `src/` moves:
36 call sites in 9 files reach the home through this one resolver.

**(B) The tests, red first.**

- `crates/cli/src/repo.rs`, two unit tests beside the three there, over one builder of the four
  layouts (`beside`, `behind`, `--separate-git-dir`, a submodule):
  `jigc_home_is_the_standing_checkout_where_no_main_checkout_exists` — `jigc_home(checkout)` is
  the checkout, and `dirname(git-common-dir)` is no checkout of it; and
  `jigc_home_of_a_fan_out_worktree_is_the_checkout_it_was_cut_from` — a worktree added at
  `<checkout>/.jigc/worktrees/a-sub-task` resolves to the checkout, one added anywhere else
  resolves to itself.
- `crates/cli/tests/setup_install_pathspec_guard.rs`,
  `git_that_answers_is_never_refused_as_git_that_does_not`: **the parity pin flips, and this is
  the cell the ruling moves.** The table's `lands` column goes — all three rows (`--separate-git-dir`,
  beside a bare repository, behind a bare `.git`) are `(Some(0), false)` on the first run and
  the re-run, with `git status --porcelain` empty after each, `.jigc/config` a directory in the
  checkout, and the folder the checkout stands in listing the same names before and after.
  Inside the submodule: `.jigc/config` is in the submodule's checkout, `pre-commit` is a file
  in the directory `git rev-parse --git-path hooks` names from there, and
  `<super>/.git/modules/vendor/.jigc` does not exist. (Not *the superproject's hook is absent*:
  that fixture's superproject carries its own install. I wrote that assertion first and it was
  red for that reason.)
- `crates/cli/tests/linked_worktree_doc_home.rs`,
  `a_layout_whose_doc_home_is_no_checkout_lands_a_doc_as_it_did_before_the_guard`, in each of
  its four layouts and under each conversion it already walks: `setup` is now asserted exit 0
  with `.jigc/config` in the checkout; after the first doc, `doc list` names `adr:cache`; a
  second ADR is minted, authored and finalized at exit 0 with no `reconciliation.rename`;
  `validate` exits 0 and prints no `blocking`; then `milestone create`, `add-task`, `provision`,
  a file staged in `.jigc/worktrees/alpha-part`, `jigc start --task alpha-part` typed in that
  worktree, `milestone finalize` with the file in `HEAD`; then `uninstall` at exit 0.

**(C) The record, which follows the ruling and is not written by this proposal.**
`design/storage.md` (the definition of jigc_home, and *Where it binds*, whose last sentences
say the layouts do what rc.24 did), `design/assistant-adapter.md` (*Where it installs*),
`design/finalize.md` (the layered-resolver note), the two guides' sentence *the doc store has
one home: the main checkout* — which needs *or, where the repository has none, the checkout you
stand in* — the (D) row of `decisions-pending.md`, discharged, and the `DECISIONS.md` entry.
And one note for `crates/cli/guides/MIGRATING.md`: an install an earlier jigc wrote at the old
home is not read any more and is not removed (below).

### What the proposal does not do

- **It does not migrate or remove an install an earlier build wrote at the old home.** Driven
  (row 12): the first command says *this project isn't set up — run `jigc setup`*, that route
  works, the docs already in `HEAD` are listed as `managed`, and the old directory stays:
  `proj/.jigc/`, `proj/.claude/`, the import line in `proj/CLAUDE.md`; in a submodule
  `super/.git/modules/vendor/{.jigc,.claude,CLAUDE.md}` and jigc's block in the superproject's
  `pre-commit`. A task left open under the old home is no longer listed; its staged docs are
  untouched on disk there. I propose a note, not a mechanism: the population is installs that
  wedged after one doc.
- **A linked worktree the user adds to a `--separate-git-dir` or submodule repository is a home
  of its own**, as each worktree of a bare repository is — not a code-only checkout beside the
  main one. git names no main work tree there: `git worktree list --porcelain` prints the git
  dir itself as its first entry in both (driven), so that is no source either.
- **`jigc setup` typed in the folder that holds a bare `.git`** — the standing directory itself
  no work tree — still installs there at exit 0. Driven on the candidate and on the spike:
  unchanged. It is the finding's title in a rarer posture, and it is left open below.
- **A superproject managing docs across its submodules** stays parked
  (`ideas/monorepo-submodule-support.md`). The proposal makes a command typed in a submodule a
  command about that submodule's repository, which is jigc's one-repository model and nothing
  more.
- **It costs one more git command each time the home is resolved in a linked worktree.**
  Counted with `GIT_TRACE` to a file, candidate against spike: `jigc start` in a linked
  worktree, 14 → 19 git commands; `jigc validate`, 15 → 20; `jigc start --task <id>` in a
  fan-out worktree, 8 → 13. The home is resolved five times in one `jigc start` on the
  candidate already.

### The other answer, for the human's comparison

*Unsupported, said at `setup`* — a refusal before the first write wherever `.git` is a file and
no checkout stands behind it. **Not spiked.** From what I drove: it needs a new finding code; it
turns two exit-0 success paths of the previous release into refusals (`setup` in a submodule
and behind a bare `.git`, both pinned today as *what rc.24 does here*); and it closes one door
only — a layout already installed keeps its wedge and its loss until every other door is
taught the same refusal.

## Driven

Each row: the step, what was typed, what it printed. The *before* is the candidate
(`dded1fac…`); the *after* is the spike, in its final form unless the row says otherwise.

| # | whose next step | command | result |
|---|---|---|---|
| 1 | the gate | `dev/gate` in `<W>/spike`, the proposal's diff applied (sha256 of `git diff` printed before and after the run: `6d536acd…`, both) | `fmt ok · clippy ok · build ok · tier1 ok · tier2 ok · doctest ok`; `tests passed=4968 failed=0 (over 18 test binaries)`; `GATE: PASS`; exit 0 |
| 2 | the standing suites, with the product half alone | `dev/gate` on an earlier form of the spike — the same four rules, the checkout question asked with two git calls, no test edits | `passed=4965 failed=1`; the one red is `setup_install_pathspec_guard::git_that_answers_is_never_refused_as_git_that_does_not`, at its parity assertion — *left: (Some(0), false) right: (Some(1), true)*. Nothing else in the suite holds the old home. (The fast tier was the whole suite on that run; being red, the doctests were not run.) |
| 3 | the installer, and its route | `walk.sh <layout> <root> setup setup`, four layouts | exit 0, 0 in all four. Created: ten paths, nine inside the checkout and `pre-commit` in the directory git names from it (`proj/store.git/hooks`, `proj/.git/hooks`, `proj/store/sep.git/hooks`, `super/.git/modules/vendor/lib/hooks`). Changed: the branch ref of the install commit. `proj/CLAUDE.md` 80 bytes; no `proj/.jigc`; the superproject's hook its own 41 bytes |
| 4 | the writer, twice | `… doc:Cache reads doc:Queue reads` | both finalizes exit 0; `doc list` names both docs; `validate` exit 0 |
| 5 | the reader | `jigc doc list`, `jigc validate`, `jigc start`, `jigc doc show adr:cache`, `jigc task list`, `jigc upgrade`, `jigc describe` in `bare` | all exit 0 |
| 6 | the join | `… milestone`: `create`, `add-task` ×2, `provision`, a file staged in each `.jigc/worktrees/<id>`, `jigc start --task <id>` and `jigc doc list` typed there, `milestone finalize` | all exit 0 in all four layouts; `HEAD` is `Finalize milestone ship-it (2 sub-tasks)` |
| 7 | the join, without rule 3 | the same, on the spike's first form — rules 2 and 4 only | `jigc start --task alpha-part` in the worktree: `finalize.no-task`; `milestone finalize`: exit 3, `repo.head-detached` for both worktrees, routed at `git switch <branch>` — **the sibling cell, found by driving, and why rule 3 is in the proposal** |
| 8 | the teardown | `… uninstall` | exit 0 in all four; the hook removed from git's hooks dir |
| 9 | equivalence | the 39-step walk's 78 output files, spike in each of the four layouts against the candidate in `plain`, the root, the checkout path and hashes normalized | 76 identical, 2 differ, in each layout: the two `setup` acks, on the one line naming where the hook went |
| 10 | every other repository | the same walk in `linked` (a worktree beside a real main checkout), candidate against spike | 84 output files compared, 0 differ. Control: two files expected to differ in `bare` do |
| 11 | the loss cell | `loss.sh` on the spike; and `walk.sh <bare\|sub> … setup plant:docs/decisions/cache.md doc:Cache` | finalize exit 3; the planted file byte for byte; all 20 step outputs identical to the candidate's in `plain` |
| 12 | the adopter already there | `walk.sh <bare\|sub> … bin=<candidate> setup doc:Cache doc:Queue bin=<spike> reads setup reads finalize:record-the-queue doc:Third reads` | on the spike: `doc list`, `validate` exit 1, *this project isn't set up — run `jigc setup`*; `jigc setup` exit 0; `doc list` lists `adr:cache` as `managed`; `validate` exit 0, one advisory, *no action needed*; the old task: `finalize.no-task`, its route `jigc task list` exit 0; a third doc finalizes at exit 0 |
| 13 | the other worktrees of one bare repository | in `bare`: `git worktree add ../wt2 -b other main` (after the install commit), `../wt3 -b early <first commit>` (before it); `jigc doc list`, `jigc setup` in each | `wt2`: set up by the commit it carries, lists the doc, `setup` exit 0. `wt3`: *isn't set up*, exit 1; `setup` exit 0 with its own install commit; `doc list` exit 0. `wt` undisturbed; `proj/` holds `CLAUDE.md`, `store.git` and the three worktrees |
| 14 | git's own hook run | `git commit` of a staged file in `bare`, the hook installed by the spike | exit 0, nothing on stderr |
| 15 | the user's own linked worktree in `sep` and `sub` | `git worktree add <root>/second -b second`; `jigc doc list`, `jigc start --workflow record-decision`, `jigc doc create adr` there | exit 0 on candidate and on spike; on the spike the task's area is under `second/.jigc/` — a home of its own (see *does not do*) |
| 16 | the standing directory that is no work tree | `jigc setup` typed in the folder holding a bare `.git`; and inside a bare repository | candidate and spike identical: exit 0, installed in that folder; and exit 1 `setup.repo-root` |

## Undriven

- **The record edits of (C)** — not applied in the spike, so no fence that reads those docs ran
  over them. They state a ruling that has not been made.
- **The other answer, a refusal at `setup`** — not spiked; priced from the driven facts only.
- **An older git.** `--path-format=absolute` needs git 2.31; below it both probes fail and the
  home is the standing checkout, as it already is on the candidate when the first probe fails.
  Read from the code, not driven: no older git on this machine.
- **The regression set's part 2 in these layouts** — its fixtures do not exist.
- **Any conversion beyond the four the standing suite walks**, and any adapter profile but the
  one `setup` installs.
- **The loss cell on the spike in `sep` and `behind`** — driven in `bare` and `sub` only; the
  resolver's answer is the same in all four (the unit test holds it).
- **`jigc uninstall --force` in `behind`**, on any binary.
- **The proposal applied by somebody else.** Mine is one spike by its own author.

## Repro AD-1 — one doc deep: the second finalize is blocked on a doc that is there

```yaml
claim: "in a worktree of a bare repository, after `jigc setup`, a first ADR finalizes at exit 0 and a second task's finalize exits 3 reporting the first doc missing; the route's `jigc unmanage` arm exits 1"
verdict: CONFIRMED
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exits and the same refusal (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
regression: false
setup:
  - env: "HOME=<fresh dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null, author and committer set"
  - ["mkdir", "proj"]
  - cwd: proj
  - ["git", "init", "-q", "--bare", "store.git"]
  - ["git", "-C", "store.git", "worktree", "add", "-q", "../wt", "-b", "main"]
  - cwd: proj/wt
  - write: "README.md = '# readme\n'"
  - ["git", "add", "README.md"]
  - ["git", "commit", "-q", "-m", "base"]
  - ["jigc", "setup"]                                                  # exit 1 — the fork's own block
  - ["jigc", "start", "--workflow", "record-decision", "record the cache"]
  - ["jigc", "doc", "create", "adr", "--title", "Cache", "--task", "record-the-cache"]
  - "for each of context, decision, consequences: `jigc doc set-slot adr:cache#<slot> --from-file - --task record-the-cache`, stdin 'Prose.\n'"
  - ["jigc", "doc", "set-field", "commit:record-the-cache#type", "--value", "feat", "--task", "record-the-cache"]
  - ["jigc", "doc", "set-field", "commit:record-the-cache#scope", "--value", "core", "--task", "record-the-cache"]
  - "`jigc doc set-slot commit:record-the-cache#summary --from-file - --task record-the-cache`, stdin 'record the cache'; the same for #body, stdin 'Body prose.'"
  - ["jigc", "task", "finalize", "record-the-cache"]                  # exit 0
  - "the same nine authoring steps for a task `record the queue` and an ADR titled Queue"
repro:
  - ["jigc", "task", "finalize", "record-the-queue"]
  - ["jigc", "unmanage", "docs/decisions/cache.md"]                   # the route's second arm
  - ["jigc", "task", "finalize", "record-the-queue"]
expect:
  exit: [3, 1, 3]
  stderr_contains_first: "blocking · reconciliation.rename — tracked managed doc adr:cache (docs/decisions/cache.md) is missing"
  stderr_second: "`git rev-parse --verify -q HEAD` failed: fatal: not a git repository (or any of the parent directories): .git"
  proj_wt: "docs/decisions/cache.md exists and is in HEAD; `jigc doc list` prints `jigc doc list — no committed docs`"
variants:
  - "`--separate-git-dir`: the same three exits"
  - "a submodule: exits 3, 0, 0 — the unmanage arm un-manages the committed doc and the finalize then lands; `jigc doc list` still prints no committed docs"
  - "a worktree behind a bare `.git`: first exit 3 on both binaries; the unmanage arm typed on the previous release only: 0, then 0"
with-the-proposal: "spike: the first of the three exits 0; the other two have nothing to answer"
observed: "<W>/c3-bare.DWlduX, <W>/c5-sep.mILV38, <W>/c5-sub.zQXJWg, <W>/c6-behind.FcPijT; previous release <W>/p3-bare.nOGcAr, <W>/p3-sub.5TXaf2, <W>/p3-behind.2SgdAH, <W>/p2-sep.IbJ1e2"
pinned-by: "UNPINNED — linked_worktree_doc_home::a_layout_whose_doc_home_is_no_checkout_lands_a_doc_as_it_did_before_the_guard walks one doc and stops; no suite I opened walks a second"
```

## Repro AD-2 — an untracked file at a doc's home is destroyed at exit 0

```yaml
claim: "in a worktree of a bare repository, after `jigc setup`, an untracked file at docs/decisions/cache.md is replaced by `jigc task finalize` at exit 0; afterwards no file under the root and no git object holds its bytes"
verdict: CONFIRMED
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same: finalize exit 0, zero files and zero git objects hold the marker"
regression: false
setup:
  - "the first nine lines of Repro AD-1's setup, through `git commit -q -m base`"
  - ["jigc", "setup"]                                                  # exit 1
  - write: "docs/decisions/cache.md = 'MARK-7f3a my own notes, in no git object\n'   (not added)"
  - control: "`grep -rl MARK-7f3a <root>` names exactly that file; `git status --porcelain --untracked-files=all` prints `?? docs/decisions/cache.md`"
repro:
  - ["jigc", "start", "--workflow", "record-decision", "record the cache"]
  - ["jigc", "doc", "create", "adr", "--title", "Cache", "--task", "record-the-cache"]
  - "the slots, the two fields and the two commit slots of Repro AD-1"
  - ["jigc", "task", "finalize", "record-the-cache"]
expect:
  exit: 0                                                              # every step
  stdout_contains: "promoted docs/decisions/cache.md"
  after_files_holding_the_marker: 0                                    # the control's command
  after_git_objects_holding_the_marker: 0                              # every blob of `git cat-file --batch-all-objects`, searched
  proj_wt_docs_decisions_cache_md: "the ADR — it opens with `---` and `status: proposed`"
control-in-a-plain-checkout: "candidate, `git init` and the same steps: `jigc task finalize` exits 3 (`conformance.section-missing`), the file is byte for byte the planted one"
variants:
  - "`--separate-git-dir`, a worktree behind a bare `.git`, a submodule — candidate and previous release: finalize exit 0, zero files and zero objects hold the marker (a different marker text, the same two searches)"
with-the-proposal: "spike, the same steps in the bare-worktree layout and in a submodule: `jigc task finalize` exits 3, the planted file is intact, and every step's output is identical to the candidate's in a plain checkout"
observed: "<W>/loss-cand-bare.* (two roots), <W>/loss-prev-bare.pA2tLl, <W>/loss2-cand-{sep,behind,sub}.*, <W>/loss2-prev-{sep,behind,sub}.*; plain <W>/clob-cand-plain.1C8jQQ; spike <W>/f-loss.*, <W>/f-clob-bare.Q2REJu, <W>/f-clob-sub.woOexo"
pinned-by: "UNPINNED — `cli::task::git_home_claims` documents that it asks nothing in these layouts; I found no test of the cell"
```

## Repro AD-3 — the proposal's own walk, for whoever drives it next

```yaml
claim: "with the proposal applied, the whole walk exits 0 in each of the four layouts, the install lands in the checkout, and nothing is written beside it"
verdict: CONFIRMED on the spike only
binary: "a build of eeffe347 with the proposal applied — a spike, sha256 2ff0072d97dfa7d04f1d001919497138da7ec5aba3d8e5c05735771e7e260e63; not the candidate"
setup:
  - "one of: the bare-worktree layout of Repro AD-1 with `proj/CLAUDE.md` written first (80 bytes); `git init --bare .git` and `git -C .git worktree add ../wt -b main`; `git init --separate-git-dir <abs>/proj/store/sep.git work`; a submodule `super/vendor/lib` whose superproject has a 41-byte pre-commit hook"
repro:
  - ["jigc", "setup"]
  - ["jigc", "setup"]
  - "the ADR task of Repro AD-1, titled Cache, to its finalize"
  - ["jigc", "doc", "list"]
  - ["jigc", "validate"]
  - "the same task, titled Queue, to its finalize"
  - ["jigc", "milestone", "create", "Ship it"]
  - ["jigc", "milestone", "add-task", "ship-it", "alpha part"]
  - ["jigc", "milestone", "add-task", "ship-it", "beta part"]
  - ["jigc", "milestone", "provision", "ship-it"]
  - "in each .jigc/worktrees/<id>: write <id>.txt, `git add` it, then `jigc start --task <id>`"
  - ["jigc", "milestone", "finalize", "ship-it"]
  - ["jigc", "uninstall"]
expect:
  exit: 0                                                              # every step
  after_setup: ".jigc/config/ is a directory in the checkout; `git status --porcelain` is empty; an install commit `chore(jigc): install jigc workspace config` is in the checkout's history"
  beside_the_checkout: "the folder it stands in lists the same names as before; proj/CLAUDE.md is its 80 bytes; the superproject's pre-commit is its 41 bytes"
  hook: "pre-commit is a file in the directory `git rev-parse --path-format=absolute --git-path hooks` names from the checkout"
  doc_list_after_the_first_doc: "names adr:cache"
  head_after_the_join: "Finalize milestone ship-it (2 sub-tasks)"
control: "the same steps on the candidate: Repro AD-1's exits, and VR-1's of the verifier's report"
observed: "<W>/f-bare.B6ZQsR, <W>/f-behind.IN7SrN, <W>/f-sep.maeujr, <W>/f-sub.iIwuo9; against <W>/c2-plain.R4WdKE"
pinned-by: "by the proposal's own tests, once applied: the two unit tests in repo.rs and the two suites named under (B)"
```

## Left open — not this fork, each a finding to triage

Every one of the first seven is closed by the proposal in the layouts driven, and stands on
the candidate whatever is decided here.

1. **`no-lost-files`:** `jigc task finalize` destroys an untracked file at a created doc's home
   at exit 0 in all four layouts (Repro AD-2). Known to the record in four words — *the ordinary
   file clobber … remain* — under the (D) row, and graded nowhere.
2. **`working-product`:** the second doc's finalize is blocked on the first as missing, and the
   route's arms do not lead out (Repro AD-1).
3. `jigc validate` exits 1 with six blocking findings, five of them `home-vacated` for homes
   that were never occupied, after one clean finalize (`bare`, `sep`, `behind`).
4. `jigc milestone create` exits 1 in all four; in `bare` and `sep` with a bare git error and no
   code or route.
5. `jigc uninstall` refuses in all four with a route whose precondition already holds;
   `jigc uninstall --force` exits 1 in `bare` and `sep` after removing `.jigc/`, its route loops,
   and `.claude/` is left in a folder that is no repository.
6. `jigc unmanage` exits 1 with a bare git error, no code and no route (`bare`, `sep`); in `sub`
   it exits 0 saying a file that is on disk was not there.
7. The finalize ack names a *main checkout jigc's workbench binds to* where none exists.
8. `jigc setup` typed in the folder holding a bare `.git` installs there at exit 0 — the
   standing directory is no work tree. Not closed by the proposal.
9. The (D) row's correction of 2026-10-05 — *creates and finalizes a doc* — is true one doc
   deep; the row owes that sentence.
10. A lead, measured in passing: the home is resolved five times in one `jigc start`, seven of
    its fourteen git commands in a linked worktree.

## Tree state

The working clone was read and not written: branch `fix/canary-one` at eeffe347, its status the
one untracked directory `completions/artifacts/canary-one/r1/`. No commit, no stage, no build
there. My two scratch clones, the layout roots and the scripts are under `<W>`. Two of my own
gate runs on earlier forms of the spike I stopped myself, by process id, once a targeted run
had shown an assertion of mine wrong; neither is quoted as a verdict. The gate quoted in row 1
ran whole, on the final diff.

<!-- end of report -->
