//! Repo-root vs **jigc_home** resolution (M31 worktree fan-out, Inc 2 / WF3).
//!
//! `discover_repo_root` (the walk-up that every command module already owns) finds the
//! **worktree** root — the dir where code, the git index, and `HEAD` live. In a linked
//! git worktree that is *not* where `.jigc/` and the committed doc-store live: those
//! bind to **jigc_home**, the *main* checkout, so all worktrees of one project share a
//! single `.jigc/`. [`jigc_home`] is that separate resolver.
//!
//! It is **layered** so the ~15 fake-`.git` unit fixtures stay green: it shells to git
//! (`dirname(git rev-parse --git-common-dir)`) **only** when a *real linked-worktree*
//! `.git` is present — which is exactly the case where `.git` is a *file* (a `gitdir:`
//! pointer). A main checkout and a fake `create_dir_all(".git")` fixture both keep
//! `.git` as a *directory*, and for both the answer is the `.git`-bearing dir itself —
//! resolved by walk-up, never by shelling out (so a fake `.git` can never walk up to,
//! and bind against, a real ancestor repo).
//!
//! **The scope of that "never by shelling out": path resolution, and nothing else.** It
//! binds [`jigc_home`] and [`worktree_git_dir`] — the two functions that answer *which
//! directory is this?* — because a shell-out there is what lets a fake `.git` bind
//! against a real ancestor repository. It has never bound the probes that then ask
//! questions *about* a resolved repository, and three ship: [`head_ref`]
//! (`git symbolic-ref -q HEAD`), [`crate::task::head_is_unborn`], and
//! [`index_has_unmerged_paths`] (`git ls-files -u`, M52 Increment 3 — the one member of
//! the operation family that no file on disk records). The standing rule for those is
//! `DECISIONS.md` 2026-05-31 → Git invocation. The two rules meet at [`posture`], which
//! resolves the git dir by walk-up **first** and asks nothing at all when that
//! resolution finds no `HEAD` — so a fake `.git` still reaches no shell-out.
//!
//! # The repository-posture family (M51 Increment 2)
//!
//! [`posture`] answers one question — *is this repository in a state a door may commit
//! or move in?* — over **three** members: **HEAD detached** · **HEAD unborn** · **an
//! operation in progress** — the third being **any operation git can leave
//! un-concluded**, [`InProgress::ALL`] (M52 Increment 3), not a list of markers.
//! [`finalize.md`](../../../design/finalize.md) → 1. Preflight has promised the third
//! verbatim since it was written (*"No in-progress merge/rebase/bisect"*) while **zero**
//! probes existed in either crate, and the M51 baseline drove the first two as live damage
//! at exit 0 (a branchless commit no branch contains; git's **empty tree** pinned into a
//! committed milestone record). The probe is the family's one home so the doors that read
//! it cannot each hand-enumerate a different three.
//!
//! **The probe's subject is a path, and a door owes it for every checkout it commits
//! from — not only for the one the command was typed in** (M53 post-review fix,
//! 2026-09-22; the M53 per-axis review, axis 2 `DEFECT 1`). Through `1.0.0-rc.17` the
//! sentence above was true of the *probe* and false of its *consumers*: `Cli::dispatch`'s
//! guard asked [`posture`] for the process cwd's repository and nothing else, while `jigc
//! milestone finalize` reads `git diff --cached` out of every provisioned sub-task
//! worktree's index and applies it. Driven on the installed `1.0.0-rc.17` across **every
//! member of [`InProgress::ALL`] buildable in a linked worktree** — which is all ten, since
//! the surjectivity fence in `repo_posture.rs` requires each of them to be produced by some
//! `GitState` and the one state a worktree cannot hold, `Unborn`, names no `InProgress`
//! member at all — the
//! boundary landed at exit **0**, committed the user's un-concluded operation under jigc's
//! own subject, and destroyed that operation's authored message (`MERGE_MSG` /
//! `SQUASH_MSG`) with the worktree teardown, while the *same* worktree refused its own
//! `jigc task finalize` one command earlier. The subject set is therefore the boundary's
//! own: [`adjudicated_breach`] is the one composition both the door guard and the
//! boundary's fan-out preflight ask, and [`BreachSite`] is how the answer says *which*
//! checkout it is about.
//!
//! **A probe that cannot answer reads as NO breach** — a `git` that cannot be spawned, a
//! path that is not a work tree, any exit code outside the two a discriminator names. The
//! shipped precedents are [`crate::task::nothing_staged`] (a git that cannot be spawned
//! reads as `false`) and `setup`'s `commit_install` (skip only when there is no git work
//! tree). Fail-*closed* would turn every fake-`.git` unit fixture in this crate into a
//! refusal, which is a test-harness fact deciding a user-facing verdict.
//!
//! **Declared bound — the `GIT_DIR` redirect is OUT, and this is the row that says so.**
//! A `GIT_DIR` exported by the user redirects git's answer to a *different repository*,
//! and the M51 baseline drove a committing door writing its record into repo A while
//! landing the commit in repo B at exit 0. This probe does not adjudicate that: its
//! subject is the path it is handed, and it passes the ambient environment through
//! untouched. (The walk-up rule this module opens with is what keeps *that* honest —
//! see its scope above: it binds where a repository is resolved, never what a probe may
//! then ask about the one it resolved.)
//!
//! **The scope is reopenable**, recorded as such: `dirname(git-common-dir) != toplevel` is
//! **not** a discriminator for the redirect — a legitimate linked worktree has the
//! identical asymmetry — so closing the cell needs a repository-**identity** check the
//! fixtures above can survive, and it reopens the moment one exists.

use engine::finding::{Finding, Location, Route, Severity};
use std::path::{Path, PathBuf};

/// Resolve **jigc_home** — the dir `.jigc/` state and the committed doc-store bind to.
///
/// Returns the same path [`discover_repo_root`](crate::repo::discover_repo_root) would
/// **outside** a worktree (byte-identical, by construction — both return the
/// `.git`-bearing ancestor directly); inside a linked worktree it returns the *main*
/// checkout. `None` when no `.git` is found walking up from `start`.
pub fn jigc_home(start: &Path) -> Option<PathBuf> {
    let repo_root = discover_repo_root(start)?;
    // Only a *linked worktree* keeps `.git` as a FILE (a `gitdir:` pointer); its
    // jigc_home is the MAIN checkout, reachable only through git. A main checkout — and
    // a fake `.git` fixture (an empty dir with no git internals) — keeps `.git` as a
    // directory, whose jigc_home is the `.git`-bearing dir itself. Shell to git only
    // for the file case, so a fake `.git` never walks up to a real ancestor repo.
    if !repo_root.join(".git").is_file() {
        return Some(repo_root);
    }
    git_common_dir_parent(&repo_root).or(Some(repo_root))
}

/// Walk up from `start` until a directory containing a `.git` entry is found,
/// returning that directory (the worktree root) — co-located here so [`jigc_home`] can
/// layer over it, and reached by [`crate::cli`]'s posture guard, which runs before any
/// door has resolved a repo.
///
/// **This is the only copy** (M53 — the cwd census, the verb class). The doc-comment said
/// *"the same walk-up every command module owns"*, and it was literally true: `start.rs`,
/// `locate.rs`, `ingest.rs`, `task.rs` and `milestone.rs` each carried a byte-identical
/// private `discover_repo_root`, six in all. Six copies of one function is its own defect —
/// a fix to the walk-up reaches one of them, and the census found the *callers* of those
/// copies split on a question the copies cannot express: whether the door's subject is the
/// **standing checkout** (what this returns) or the **milestone/task store**, which binds
/// to [`jigc_home`] and inside a linked worktree is a different directory. Collapsing them
/// puts that choice at every call site, where it is visible.
pub fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}

/// `dirname(git -C <repo_root> rev-parse --path-format=absolute --git-common-dir)` —
/// the *main* checkout behind a linked worktree. `None` if git fails or returns empty.
fn git_common_dir_parent(repo_root: &Path) -> Option<PathBuf> {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let common = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if common.is_empty() {
        return None;
    }
    Path::new(&common).parent().map(PathBuf::from)
}

/// One member of the **repository-posture family** — the three states a door that commits
/// or moves on the user's behalf must adjudicate before it acts
/// ([finalize.md](../../../design/finalize.md) → 1. Preflight; `DECISIONS.md` 2026-09-14
/// → M51 Increment 2).
///
/// The variants are a **set, not a list**: [`PostureMember::ALL`] is the family's
/// defining case-set, so a fourth member cannot be added without every consumer's
/// exhaustive match answering it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PostureMember {
    /// HEAD points at a commit rather than a branch: a commit made here belongs to no
    /// branch, and the next checkout loses it.
    HeadDetached,
    /// A real repository with no commits yet. Driven at the M51 baseline: the milestone
    /// doors pin git's **empty tree** as a base that `git worktree add` cannot take.
    HeadUnborn,
    /// A merge, rebase or bisect the user started and has not concluded — the member
    /// `design/finalize.md` → 1. Preflight has promised verbatim with no probe behind it.
    OperationInProgress,
}

impl PostureMember {
    /// The family's defining case-set, in probe order.
    pub const ALL: [PostureMember; 3] = [
        PostureMember::HeadDetached,
        PostureMember::HeadUnborn,
        PostureMember::OperationInProgress,
    ];

    /// This member's blocking finding code — the identity a log reader keys on
    /// (`design/validation.md` → the `repo.*` rows). None of the three joins
    /// `ERROR_CODE_REGISTRY`: that registry mirrors **door** identities derived from
    /// `COMMITTING_DOORS`, and a blocking `Finding` is not an `Outcome` identity.
    pub fn code(self) -> &'static str {
        match self {
            PostureMember::HeadDetached => "repo.head-detached",
            PostureMember::HeadUnborn => "repo.head-unborn",
            PostureMember::OperationInProgress => "repo.operation-in-progress",
        }
    }
}

/// The git operation an [`PostureMember::OperationInProgress`] breach names — carried on
/// the breach so the route can name **the command that concludes this operation**, never a
/// menu of every operation git can leave un-concluded.
///
/// **The member set is the operations git can leave un-concluded, not the markers it
/// writes** (M52 Increment 3; `settle-record.md` → D2.1). The M51 shape carried three
/// markers over three operations, and the M52 baseline drove what that costs: a clean
/// `git merge --squash` writes **`SQUASH_MSG` and nothing else**, a conflicted `git stash
/// pop` writes **no marker at all**, and `rebase-apply/` is written by **two** operations
/// — so no widening of a marker list could have reached the first two, and a list that
/// reached the third named `git am` *a rebase* and routed it at `git rebase --abort`,
/// which git refuses at exit 128
/// ([baseline-posture.md](../../../completions/artifacts/M52/baseline-posture.md) §1.1,
/// §2.2, §2.6). Each variant therefore owns its own [`detect`](InProgress::detect), its
/// own noun and its own concluding and abandoning commands.
///
/// **M53 Increment 4 adds the member that could only be named this way.** A `git
/// cherry-pick --no-commit` writes **no marker of its own** — `MERGE_MSG` and nothing
/// else — so it is not reachable by *adding* a marker to any list: it is named by what
/// git left **minus** what every marker-keyed member owns
/// ([`InProgress::UncommittedCherryPick`]), which is a predicate a marker list has no
/// shape for. Driven at this wave's base, `jigc task finalize` concluded that pick at
/// exit 0, committing the picked payload under jigc's own subject and destroying the
/// picked commit's authored message with `MERGE_MSG`
/// ([baseline-a2-cherry-pick-posture.md](../../../completions/artifacts/M53/baseline-a2-cherry-pick-posture.md)
/// — three damage shapes across the acting doors, of which that is the *swallow*).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InProgress {
    /// A `git merge` the user has not concluded — `MERGE_HEAD`.
    Merge,
    /// A `git merge --squash` whose result is staged and uncommitted — `SQUASH_MSG`
    /// with **no** `MERGE_HEAD`, the only evidence git leaves. Driven at the M52
    /// baseline: the whole squashed payload landed inside jigc's own commit at exit 0
    /// and the merge's authored message was destroyed with `SQUASH_MSG` (§2.6 / L3).
    SquashMerge,
    /// A `git rebase` stopped mid-replay — `rebase-merge/` (the merge backend) or
    /// `rebase-apply/` **without** `applying` (the apply backend).
    Rebase,
    /// A `git am` stopped on a patch that does not apply — `rebase-apply/applying`.
    ///
    /// git's **own** discriminator: `rebase-apply/applying` is written by `am` and
    /// `rebase-apply/onto` by `rebase --apply`, which is how `git rebase --abort` can
    /// answer *"It looks like 'git am' is in progress"*. Both were driven in both
    /// directions (§1.1), and it is why this cell cannot be fixed by renaming a noun.
    Am,
    /// A `git cherry-pick` stopped on a conflict — `CHERRY_PICK_HEAD`.
    CherryPick,
    /// A `git revert` stopped on a conflict — `REVERT_HEAD`.
    ///
    /// git's own partial-commit guard reads `MERGE_HEAD` and `CHERRY_PICK_HEAD` and
    /// **not** `REVERT_HEAD`, so this is the cell where *every* commit door concluded
    /// the user's revert at exit 0 and left `git revert --continue` answering *"no
    /// cherry-pick or revert in progress"* (§2.5 / L2).
    Revert,
    /// A queue of commits `sequencer/` still holds with no pick or revert live —
    /// the residue of a multi-commit pick or revert whose current commit was
    /// concluded some other way.
    ///
    /// It is probed **after** [`InProgress::CherryPick`] and [`InProgress::Revert`] on
    /// purpose: while either marker is present the pick or the revert *is* the
    /// operation, and `git cherry-pick --abort` clears a multi-commit revert's queue
    /// too (driven) — so the opposite order would route a user mid-`revert` at a
    /// cherry-pick. Driven at the baseline as the state jigc's own exit-0 finalize
    /// produced, whose advertised recovery then destroyed that commit (§2.7 / L5).
    Sequencer,
    /// A `git bisect` the user has not reset — `BISECT_LOG`.
    Bisect,
    /// A `git cherry-pick --no-commit` the user has applied and not committed —
    /// `MERGE_MSG` with **none** of the markers every member above keys on.
    ///
    /// git writes **no `CHERRY_PICK_HEAD`** under `--no-commit`, which is why every
    /// marker-keyed member misses this state: the picked payload is in the index, the
    /// picked commit's own message is in `MERGE_MSG`, and on a clean pick that file is
    /// the whole of git's record. (`AUTO_MERGE` is written here too and discriminates
    /// nothing — a clean `git stash apply` and a concluded rebase leave it as well —
    /// which is why the predicate does not read it.)
    ///
    /// It is probed **after every marker-keyed member and before**
    /// [`InProgress::UnmergedIndex`], and both belts matter: the predicate negates the
    /// six markers, so a paused `rebase-merge` — which leaves `MERGE_MSG` too — is still
    /// a rebase, and the position keeps the conflicted cell from falling through to *a
    /// conflict*, whose route is `git reset --merge` and throws the picked bytes away
    /// (M53 Increment 4; `settle-record.md` → D4).
    ///
    /// **The noun names the pick and only the pick**, which is a driven fact rather than
    /// a hope: `git revert --no-commit` writes `REVERT_HEAD` **clean or conflicting**
    /// (driven on git 2.54.0), so an uncommitted revert is answered by
    /// [`InProgress::Revert`] and never lands here.
    UncommittedCherryPick,
    /// **Unmerged paths in the index with no operation marker at all** — a conflicted
    /// `git stash pop`, or a conflicted `git merge --squash`.
    ///
    /// The family's last question, and the only one that is not a filesystem read:
    /// `git ls-files -u`. It is asked **last** because every conflicting operation
    /// above also leaves unmerged entries, and each of those is better named by its own
    /// operation; this member is what is left when none of them answered — the cell the
    /// baseline drove reaching the commit seam and being dressed as a hook's complaint
    /// (§2.6 / L6).
    UnmergedIndex,
}

impl InProgress {
    /// Every operation, **in probe order** — the order [`posture`] asks them in and the
    /// order a consumer taking the first answer inherits.
    ///
    /// Four of the orderings are load-bearing and each is driven:
    /// [`InProgress::Am`] and [`InProgress::Rebase`] are disjoint by predicate rather
    /// than by position; [`InProgress::CherryPick`] and [`InProgress::Revert`] come
    /// **before** [`InProgress::Sequencer`];
    /// [`InProgress::UncommittedCherryPick`] comes after **every marker-keyed member**,
    /// because each of them can leave `MERGE_MSG` beside its own marker, and **before**
    /// [`InProgress::UnmergedIndex`], because its conflicted cell would otherwise fall
    /// through to *a conflict* and be routed at a command that discards the picked
    /// bytes; and [`InProgress::UnmergedIndex`] is **last**, so the shell-out it needs
    /// is reached only where no marker answered.
    pub const ALL: [InProgress; 10] = [
        InProgress::Merge,
        InProgress::SquashMerge,
        InProgress::Rebase,
        InProgress::Am,
        InProgress::CherryPick,
        InProgress::Revert,
        InProgress::Sequencer,
        InProgress::Bisect,
        InProgress::UncommittedCherryPick,
        InProgress::UnmergedIndex,
    ];

    /// **Is this operation in progress in `repo_root`?** — the per-variant predicate
    /// that replaced a shared marker list.
    ///
    /// Every member but one reads the **worktree's own** git dir, so a linked worktree
    /// answers about itself. The remaining member, [`InProgress::UnmergedIndex`], asks
    /// git, because the state it names leaves nothing on disk to read.
    fn detect(self, git_dir: &Path, repo_root: &Path) -> bool {
        let present = |entry: &str| git_dir.join(entry).exists();
        match self {
            InProgress::Merge => present("MERGE_HEAD"),
            // A `--squash` merge never updates HEAD, so `MERGE_HEAD` is absent and
            // `SQUASH_MSG` is the whole of git's record. The conjunct keeps an ordinary
            // conflicting merge — which leaves both — answered as a merge.
            InProgress::SquashMerge => present("SQUASH_MSG") && !present("MERGE_HEAD"),
            // `rebase-apply/` without `applying` is a rebase on the apply backend; with
            // it, the operation is an `am`. The negated conjunct rather than a positive
            // `onto` read keeps the two total over `rebase-apply/`.
            InProgress::Rebase => {
                present("rebase-merge")
                    || (present("rebase-apply") && !present("rebase-apply/applying"))
            }
            InProgress::Am => present("rebase-apply/applying"),
            InProgress::CherryPick => present("CHERRY_PICK_HEAD"),
            InProgress::Revert => present("REVERT_HEAD"),
            InProgress::Sequencer => present("sequencer"),
            InProgress::Bisect => present("BISECT_LOG"),
            // `--no-commit` writes no marker of its own, so the state is named by what
            // git left MINUS what every marker-keyed member owns. The negated conjuncts
            // are the predicate's own belt: `rebase-merge` and a conflicting merge,
            // pick or revert all leave `MERGE_MSG` beside their own marker.
            InProgress::UncommittedCherryPick => {
                present("MERGE_MSG")
                    && !(present("MERGE_HEAD")
                        || present("CHERRY_PICK_HEAD")
                        || present("REVERT_HEAD")
                        || present("SQUASH_MSG")
                        || present("rebase-merge")
                        || present("rebase-apply"))
            }
            InProgress::UnmergedIndex => index_has_unmerged_paths(repo_root),
        }
    }

    /// How the message names it — **this** operation, so a user in the middle of one of
    /// the family's operations learns which one.
    pub fn noun(self) -> &'static str {
        match self {
            InProgress::Merge => "a merge",
            InProgress::SquashMerge => "a squash merge",
            InProgress::Rebase => "a rebase",
            InProgress::Am => "a `git am`",
            InProgress::CherryPick => "a cherry-pick",
            InProgress::Revert => "a revert",
            InProgress::Sequencer => "a cherry-pick or revert",
            InProgress::Bisect => "a bisect",
            InProgress::UncommittedCherryPick => "an uncommitted cherry-pick",
            InProgress::UnmergedIndex => "a conflict",
        }
    }

    /// What the message says is true of it — the half [`noun`](InProgress::noun) cannot
    /// carry, because two members are not *"in progress"* in git's own vocabulary: a
    /// squash merge is staged, and an unmerged index is what a conflict left behind.
    fn predicate(self) -> &'static str {
        match self {
            InProgress::SquashMerge => "is staged and not committed",
            InProgress::Sequencer => "left a queue of commits in `sequencer/`",
            InProgress::UnmergedIndex => "left unmerged paths in the index",
            _ => "is in progress",
        }
    }

    /// The git command that **concludes** it, where git has one, **with the clause that
    /// makes naming it true** — `(command, qualifier)`.
    ///
    /// **The qualifier is the member's own, not the mold's** (M53 Increment 4 / T3;
    /// `settle-record.md` → §10). *"once its conflicts are resolved"* was one hard-coded
    /// phrase in [`PostureBreach::finding`]'s `format!`, and it is true only of a member
    /// git detects in a state the user is **stopped** in — which every member carrying a
    /// concluding command today is. A member git can also leave behind with nothing
    /// conflicted cannot say it, so the clause travels with the command instead of with
    /// the sentence that prints it, and a member that has no command carries no clause by
    /// construction.
    ///
    /// Both qualifiers in this family are **literal suffixes, their own separator
    /// included** (the sibling is [`abandon_qualifier`](InProgress::abandon_qualifier)):
    /// the renderer appends them after the closing backtick and adds nothing, so a member
    /// whose clause is a sentence of its own can punctuate it rather than being forced
    /// into one joining word.
    ///
    /// `None` for the four members with no such command — three of them for a reason
    /// that distinguishes them, and one whose reason this family's tenth member
    /// falsified. A dangling `sequencer/` has no current commit to continue, a bisect
    /// ends rather than concludes, and an unmerged index is resolved rather than
    /// continued. The squash merge's stated reason was *"it is concluded by the user's
    /// own `git commit`"*, and that is **struck**: so is
    /// [`InProgress::UncommittedCherryPick`], which names the command — so the clause
    /// says nothing about why one member spells `git commit` and the other withholds it
    /// (M53 Increment 4). The row is kept rather than changed here, because naming it
    /// at the squash merge would reword a shipped route this task did not set out to
    /// touch; the question is carried as a deferral with a trigger
    /// (`implementation/decisions-pending.md`).
    pub fn conclude(self) -> Option<(&'static str, &'static str)> {
        // The five that stop the user where they stand all share the one qualifier — it
        // is shared here, where a member can decline it, rather than in the renderer,
        // where it could not.
        let stopped = " once its conflicts are resolved";
        match self {
            InProgress::Merge => Some(("git merge --continue", stopped)),
            InProgress::Rebase => Some(("git rebase --continue", stopped)),
            InProgress::Am => Some(("git am --continue", stopped)),
            InProgress::CherryPick => Some(("git cherry-pick --continue", stopped)),
            InProgress::Revert => Some(("git revert --continue", stopped)),
            // The one member whose concluding command is not a `--continue`, and the
            // reason the qualifier had to leave the mold: git parked the picked
            // commit's message in `MERGE_MSG`, so a bare `git commit` finishes the
            // pick the user started — and *"once its conflicts are resolved"* would be
            // false on the clean cells, which have none.
            InProgress::UncommittedCherryPick => Some((
                "git commit",
                " (which uses the pick's own message, once any conflicts are resolved)",
            )),
            InProgress::SquashMerge
            | InProgress::Sequencer
            | InProgress::Bisect
            | InProgress::UnmergedIndex => None,
        }
    }

    /// The git command that **abandons** it — the route's load-bearing bytes, and the
    /// one command that is runnable from the state as jigc found it.
    ///
    /// Every cell was driven on git 2.54.0: each exits **0** in the state its own
    /// `detect` answers and leaves no operation behind
    /// (`crates/cli/tests/repo_posture.rs` runs them out of the emitted route).
    pub fn abandon(self) -> &'static str {
        match self {
            InProgress::Merge => "git merge --abort",
            // `git merge --abort` answers *"There is no merge to abort (MERGE_HEAD
            // missing)"* here — the squash never updated HEAD. `git reset --merge`
            // drops the staged squash and `SQUASH_MSG` with it.
            InProgress::SquashMerge => "git reset --merge",
            InProgress::Rebase => "git rebase --abort",
            InProgress::Am => "git am --abort",
            InProgress::CherryPick => "git cherry-pick --abort",
            InProgress::Revert => "git revert --abort",
            // `--quit` rather than `--abort`: it forgets the queue **without rewinding
            // HEAD**, and the commits already made here may be the user's own (or
            // jigc's) — the baseline's L5 is exactly the loss `--abort` causes.
            InProgress::Sequencer => "git cherry-pick --quit",
            InProgress::Bisect => "git bisect reset",
            // Plain `git reset`, driven in all four cells plus a linked worktree
            // (`tests/repo_posture.rs`): it clears `MERGE_MSG` and empties the unmerged
            // index while leaving the picked bytes in the working tree. `git
            // cherry-pick --abort` exits 128 here — `CHERRY_PICK_HEAD` was never
            // written — and `git reset --merge`, the route this state fell through to
            // before the member existed, throws the picked bytes away.
            InProgress::UncommittedCherryPick => "git reset",
            InProgress::UnmergedIndex => "git reset --merge",
        }
    }

    /// What abandoning it **leaves behind**, where the command's name does not say —
    /// [`abandon`](InProgress::abandon)'s qualifier, on the same rule as
    /// [`conclude`](InProgress::conclude)'s: a literal suffix, its own separator
    /// included, appended after the closing backtick.
    ///
    /// **[Corrected 2026-09-22 (M53 completion audit, finding 4).** This doc read *"Empty
    /// for every member but one, and that is the honest answer rather than a placeholder:
    /// each of their commands is named for what it does to the operation and the operation
    /// is all it touches, so a clause would be restating the verb."* **Falsifying datum,
    /// driven on git 2.54.0 through `support::git_state`'s builder**, one unrelated file
    /// staged in the caller's index before the emitted command ran: `git merge --abort`,
    /// `git rebase --abort`, `git am --abort`, `git cherry-pick --abort`, `git revert
    /// --abort` and `git reset --merge` each deleted it from the index **and from the
    /// working tree**. *The operation is all it touches* is false at six of the members it
    /// was written about. The two it holds for are `git cherry-pick --quit` and `git
    /// bisect reset`, which left the staged entry exactly as they found it — their empty
    /// clause is honest, and it is now checked by
    /// `repo_posture.rs::every_abandon_commands_effect_on_unrelated_staged_work_is_declared`
    /// rather than asserted here.**]
    ///
    /// [`InProgress::UncommittedCherryPick`] is the member the qualifier was built for,
    /// and it is why the qualifier is not the mold's: `git reset` abandons the pick while
    /// **keeping** the bytes it applied, which is a claim no bare command name carries,
    /// and a user who cannot tell *abandoned* from *discarded* re-does the work or loses
    /// it (`settle-record.md` → §10). Until this member landed the clause was declared
    /// and printed nowhere.
    ///
    /// **The clause is one sentence per *fate*, not one per member** (M53 completion audit,
    /// finding 7 — the human's call, 2026-09-22). The datum above sorts the family into
    /// three fates and nothing finer, so the arms below are those three: every member whose
    /// command **discards** unrelated staged work shares one clause verbatim, reached by two
    /// different commands (`--abort` and `git reset --merge`); the one that **unstages** it
    /// carries its own; the ones that leave it alone carry none. A reader
    /// who meets the sentence at a merge has already read the one they will meet at a
    /// rebase.
    ///
    /// Finding 4 left the five conclude-bearing members — `Merge` · `Rebase` · `Am` ·
    /// `CherryPick` · `Revert` — silent, because their whole rendered route line ships as a
    /// byte literal in `repo_posture.rs::SHIPPED_ROUTE_LINES` (DECISIONS.md → 2026-09-05,
    /// §10) and rewording five shipped user-facing lines is not a side effect any fix gets
    /// to have. That was a **scope** hold with a named holder, and the holder — the human —
    /// took it: the pin is re-blessed to the new bytes deliberately, and the fence's
    /// pin-derived exemption is **gone**, so every member whose command touches unrelated
    /// staged work now owes its clause with no exemption left to hide behind.
    pub fn abandon_qualifier(self) -> &'static str {
        match self {
            // `--abort` reconstructs the pre-operation state, which resets the index AND
            // the working tree; `git reset --merge` resets the index to HEAD and takes the
            // working tree with it. Driven, one unrelated file staged beforehand: every
            // member below deleted it from both. An empty clause here reads as *this
            // touches the operation only*, which is exactly the claim the audit falsified.
            InProgress::Merge
            | InProgress::Rebase
            | InProgress::Am
            | InProgress::CherryPick
            | InProgress::Revert
            | InProgress::SquashMerge
            | InProgress::UnmergedIndex => {
                " (which also discards anything else you had staged, from the index and \
                 from your working tree)"
            }
            // True in both cells, which is what it is worded for: on a clean pick the
            // applied bytes go from staged to unstaged, and on a conflicted one the
            // conflict markers stay in the file (driven, `tests/repo_posture.rs`). The
            // second clause is the audit's: a MIXED reset unstages the **whole** index,
            // not the pick's share of it, so a caller who had unrelated work staged when
            // jigc refused finds that unstaged too — bytes kept, which is why this member
            // reads `Unstaged` and not `Discarded`.
            InProgress::UncommittedCherryPick => {
                " (which unstages everything — the picked changes and anything else you \
                 had staged — keeping all of it in your working tree)"
            }
            // The two the struck sentence was actually true of: driven, each left the
            // unrelated staged entry exactly as it found it.
            InProgress::Sequencer | InProgress::Bisect => "",
        }
    }
}

/// **Does `repo_root`'s index carry unmerged paths?** — `git ls-files -u`, the one
/// question in this family that no file on disk answers.
///
/// A probe that cannot answer reads as **no breach** (module header): a git that cannot
/// be spawned, a non-zero exit, or empty output all answer `false`.
fn index_has_unmerged_paths(repo_root: &Path) -> bool {
    std::process::Command::new("git")
        .args(["ls-files", "-u"])
        .current_dir(repo_root)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .is_some_and(|out| !String::from_utf8_lossy(&out.stdout).trim().is_empty())
}

/// One breach of the posture family: the member, plus the concrete operation when the
/// member is [`PostureMember::OperationInProgress`].
///
/// Constructed only by [`posture`], so a breach in hand is a breach that was **probed**,
/// never one a caller asserted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostureBreach {
    member: PostureMember,
    operation: Option<InProgress>,
    /// The sub-task this checkout is jigc's own fan-out worktree for, when it is one —
    /// [`posture_subject`]'s verdict, read back at the probe and carried to the render.
    ///
    /// It exists for one route. `repo.head-detached`'s advice is *"re-attach HEAD with
    /// `git switch <branch>`"*, and in a provisioned fan-out worktree there is no branch the
    /// placeholder can take: jigc detached it, the main checkout holds `main`, and git
    /// refuses a branch another worktree has checked out at exit **128** (driven, the M53
    /// rc.20 per-axis review `(2, A2-2)`). A route that is textually perfect and returns 128
    /// is a dead end, so the one cell where the family's own subject classifier says
    /// *dedicated* renders an exit that runs there instead.
    provisioned: Option<String>,
}

/// **Whose checkout a breach is being reported about, as the reader must read it** — the
/// subject of the message and the checkout every command in the route has to run in
/// (M53 post-review fix, 2026-09-22).
///
/// The family's probe has always answered about *a path*, and until this existed every
/// producer rendered the answer as though that path were the one the user typed the
/// command in. The milestone boundary broke that: it commits from each provisioned
/// sub-task worktree's **index**, so a breach it must refuse on lives in a checkout the
/// caller is not standing in, and a route reading *"conclude it with `git merge
/// --continue`, then re-run this command"* would be run in the wrong repository. The site
/// travels with the render rather than being patched into the text afterwards, so the two
/// arms cannot drift.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreachSite<'a> {
    /// The checkout the command was run in — the door guard's subject, and the bytes
    /// every shipped posture refusal has printed since M51.
    Here,
    /// A fan-out worktree **jigc** provisioned for one milestone sub-task, named by its
    /// repo-relative path (`crate::render::repo_relative`, law 1) and carrying the absolute
    /// path of the same directory. The milestone boundary commits that worktree's index, so
    /// the user has to conclude or abandon the operation *there*.
    ///
    /// **Two spellings of one path, and each is load-bearing** (M53 — the cwd census, the
    /// route class). The repo-relative half is what the message says and what the `at:` locus
    /// and the `(code, target)` key carry — portable across the two checkouts a fan-out is
    /// made of, which an absolute is not. The absolute half is the `-C` operand of the route,
    /// which git resolves against the *caller's* cwd: driven, `git -C
    /// .jigc/worktrees/<id> bisect reset` exits **128** from every directory but the
    /// repository root, and `-C` takes a directory, so no pathspec spelling reaches it.
    FanOutWorktree {
        /// The repo-relative spelling — the message, the locus and the key.
        at: &'a str,
        /// The same directory, absolute — the route's `-C` operand.
        abs: &'a std::path::Path,
    },
}

impl BreachSite<'_> {
    /// **What the message says *before* naming the breach** — empty for [`BreachSite::Here`],
    /// so that arm's bytes are the ones M51 shipped.
    ///
    /// It leads rather than trails, and that is the whole of the fix (the independent review
    /// of `3c71da87`, LOW 1). Appended, the clause landed after a predicate that may already
    /// end in a prepositional phrase of its own — [`InProgress::predicate`] gives
    /// [`InProgress::UnmergedIndex`] *"left unmerged paths in the index"* and
    /// [`InProgress::Sequencer`] *"left a queue of commits in `sequencer/`"* — so those two
    /// read *"… in the index **in the fan-out worktree `X`**"*, a place inside a place.
    ///
    /// **Led for every member, not for the two that read badly.** A per-member choice would
    /// put a rendering rule inside the member table, where the next member added has to
    /// remember it; leading unconditionally is one shape, and the site is the first thing a
    /// reader standing in a different checkout needs anyway.
    fn lead(self) -> String {
        match self {
            BreachSite::Here => String::new(),
            BreachSite::FanOutWorktree { at, .. } => {
                format!("in the fan-out worktree `{at}`, ")
            }
        }
    }

    /// The clause that says why the breach matters *here* — the half that cannot be shared,
    /// because at a fan-out worktree the repository the caller is standing in may be
    /// perfectly committable and the one being committed *from* is not.
    fn tail(self) -> &'static str {
        match self {
            BreachSite::Here => "the repository is not in a committable state",
            BreachSite::FanOutWorktree { .. } => {
                "the milestone boundary commits that worktree's index, and it is not in a \
                 committable state"
            }
        }
    }

    /// `command`, aimed at this site — unchanged here, `-C <worktree>` there.
    ///
    /// Every command this family routes at is a `git` invocation, which is what makes the
    /// redirection one insertion after the program name rather than a per-member rewrite;
    /// `repo_posture.rs::every_routed_command_is_a_git_invocation` is the fence, so a
    /// member added with a non-`git` command reddens instead of being printed un-aimed.
    fn aim(self, command: &str) -> String {
        match self {
            BreachSite::Here => command.to_string(),
            BreachSite::FanOutWorktree { abs, .. } => aim_at(abs, command),
        }
    }

    /// The finding's own located address — the worktree path, so two breaching worktrees
    /// carry two discriminating `(code, target)` keys rather than one.
    fn location(self) -> Option<Location> {
        match self {
            BreachSite::Here => None,
            BreachSite::FanOutWorktree { at, .. } => Some(Location::addressed(at, 1, 1)),
        }
    }
}

/// **`command`, aimed at the checkout at repo-relative `at`** — the `-C` redirection, in one
/// home (M53 post-review-fix review, the HIGH; 2026-09-22).
///
/// [`BreachSite::aim`] is one caller; the other is the leftover classifier's refusal listing
/// (`crate::milestone::hold_line`), which names the command that clears an operation held in a
/// path the caller is not standing in and must aim it the same way, at a path that is not
/// necessarily a *fan-out* worktree at all — `jigc milestone provision` reaches an
/// unregistered one that can belong to a different repository entirely. Sharing the render
/// rather than the site keeps the two from drifting into two spellings of one redirection.
///
/// **The directory is the absolute one** (M53 — the cwd census, C1-06). It shipped as the
/// repo-relative spelling, and `-C` takes a *directory*, not a pathspec: driven from
/// `docs/deep` and from a sibling fan-out worktree, `git -C .jigc/worktrees/<id> bisect reset`
/// exits **128** (`fatal: cannot change to …`), and `git -C ':/…'` exits 128 too — there is no
/// relative spelling that works from more than one cwd. This is the row that opened the
/// census, and it is the *pasteable shell bytes* disposition `design/surface-contract.md`
/// already writes down. The message and the `at:` locus beside it keep the repo-relative
/// spelling: they are read and keyed, not run.
///
/// The path is a [`engine::finding::shell_operand`], so a name with a space stays one operand
/// (M51's route-safety class); a command this family does not own — one that is not a `git`
/// invocation — is returned unaimed rather than mis-rewritten, which
/// `repo_posture.rs::every_routed_command_is_a_git_invocation` is the standing fence against.
pub fn aim_at(at: &Path, command: &str) -> String {
    match command.strip_prefix("git ") {
        Some(rest) => engine::finding::git_at(at, rest),
        None => command.to_string(),
    }
}

impl PostureBreach {
    /// Which member of the family this breach is — the handle the two door classes filter
    /// on (a mover refuses only [`PostureMember::OperationInProgress`]).
    pub fn member(&self) -> PostureMember {
        self.member
    }

    /// The git operation in progress, when that is the member; `None` otherwise.
    pub fn operation(&self) -> Option<InProgress> {
        self.operation
    }

    /// The blocking [`Finding`] this breach refuses with, **about the checkout the command
    /// was run in** — [`finding_at`](PostureBreach::finding_at) at [`BreachSite::Here`],
    /// which is every shipped producer's site and the bytes `repo_posture.rs`'s
    /// `SHIPPED_ROUTE_LINES` pins.
    pub fn finding(&self) -> Finding {
        self.finding_at(BreachSite::Here)
    }

    /// The blocking [`Finding`] this breach refuses with — §10's mold: a code, a
    /// [`Route::human`] naming **the git command that resolves the state**, and no
    /// override. A posture is a repository state the user can resolve, not bytes only
    /// they can value, so the family carries no consent flag.
    ///
    /// `site` says *whose* checkout, and nothing else: the member, the noun, the
    /// predicate and both qualifiers are the same values at both sites, so the two arms
    /// cannot say different things about the same state.
    pub fn finding_at(&self, site: BreachSite<'_>) -> Finding {
        let lead = site.lead();
        let (message, route) = match (self.member, self.operation) {
            (PostureMember::HeadDetached, _) => (
                format!(
                    "{lead}HEAD is detached — a commit made here would belong to no \
                     branch, and the next checkout would leave it unreachable"
                ),
                // **The one member whose generic advice is unrunnable in one checkout.**
                // `git switch <branch>` resolves a detached HEAD everywhere but jigc's own
                // provisioned fan-out worktree, where the only branch in the repository is
                // the one the main checkout holds and git refuses it at exit 128. The
                // substitution is keyed on [`PostureBreach::provisioned`] — the family's own
                // subject classifier, not a path-shape guess — and only at
                // [`BreachSite::Here`]: a breach reported *about* a fan-out worktree from the
                // main checkout is `HeadDetached`-exempt by construction, so that pair cannot
                // occur, and the `_` arm keeps the render total rather than asserting it.
                match (&self.provisioned, site) {
                    (Some(sub_task), BreachSite::Here) => format!(
                        "this checkout is the fan-out worktree jigc provisioned for sub-task \
                         `{sub_task}` and detached itself, and a branch another worktree holds \
                         cannot be switched to from here — give this work a branch of its own \
                         with `git switch -c <new-branch>`, then re-run this command"
                    ),
                    _ => format!(
                        "re-attach HEAD with `{}`, then re-run this command",
                        site.aim("git switch <branch>"),
                    ),
                },
            ),
            (PostureMember::HeadUnborn, _) => (
                format!(
                    "{lead}HEAD is unborn — this repository has no commits yet, so \
                     there is no base for jigc to commit against"
                ),
                format!(
                    "land the repository's first commit with `{}`, then re-run this \
                     command",
                    site.aim("git commit"),
                ),
            ),
            (PostureMember::OperationInProgress, operation) => {
                // `posture` never builds this member without an operation; the fallback
                // keeps the renderer total rather than panicking on a shape it owns.
                let operation = operation.unwrap_or(InProgress::Merge);
                // Both halves name a command and then say what the member itself says
                // about naming it — the mold carries the sentence, never the clause.
                let abandon = format!(
                    "abandon it with `{}`{}",
                    site.aim(operation.abandon()),
                    operation.abandon_qualifier(),
                );
                let route = match operation.conclude() {
                    Some((conclude, qualifier)) => {
                        let conclude = site.aim(conclude);
                        format!(
                            "conclude it with `{conclude}`{qualifier}, or {abandon}, then \
                             re-run this command"
                        )
                    }
                    None => format!("conclude it, or {abandon}, then re-run this command"),
                };
                (
                    format!(
                        "{lead}{} {} — {}",
                        operation.noun(),
                        operation.predicate(),
                        site.tail(),
                    ),
                    route,
                )
            }
        };
        Finding::graded(
            Severity::Blocking,
            self.member.code(),
            message,
            site.location(),
            Some(Route::human(route)),
        )
    }
}

/// **The breach `repo_root` owes an answer for** — probe the family and take the first
/// breach *both* `subject` and the asking door adjudicate.
///
/// The family's one composition home (M53 post-review fix, 2026-09-22). Three askers: the
/// door guard ([`crate::cli::posture_refusal_in`]), whose `owes` is its
/// [`crate::cli::BEHALF_DOORS`] row (a mover's one member, minus the row's stated
/// exemptions); the milestone boundary's fan-out preflight, whose `owes` is `true` — it
/// commits, and it carries no exemption row; and the `jigc task finalize` preview. Splitting
/// the composition would let them classify the same checkout differently, which is exactly
/// the defect the 2026-09-22 fix closes one level down.
///
/// **The subject is the caller's to state, because it is a fact about the *act*, not about
/// the path** (M53, the rc.20 per-axis review `(2, A2-2)`). [`posture_subject`]'s
/// `Dedicated` verdict exempts [`PostureMember::HeadDetached`] so jigc's own `--detach`
/// provisioning is not refused — and that exemption belongs to the acts jigc performs there
/// **holding the worktree's handle** ([`SeamSubject::dedicated`]), never to a door a user
/// invoked inside one. Derived from the path here, it was taken by a surface forecasting a
/// [`SeamSubject::live`] commit, and the preview read *clean* over a state the seam refused:
/// so the caller says which subject its act will have, and the classifier is one of the two
/// answers rather than the only one.
pub fn adjudicated_breach(
    repo_root: &Path,
    subject: &PostureSubject,
    owes: impl Fn(PostureMember) -> bool,
) -> Option<PostureBreach> {
    posture(repo_root)
        .into_iter()
        .find(|breach| subject.adjudicates(breach.member()) && owes(breach.member()))
}

/// **Whose checkout a posture verdict is about** — the subject the posture family is
/// adjudicated against (`settle-record.md` → Review amendments §3).
///
/// The family's members are not universal facts about a repository: they are facts about
/// **the user's** repository. jigc's own fan-out provisions a `--detach` worktree per
/// milestone sub-task, and [`posture`] answers `repo.head-detached` there
/// **byte-identically** to a user's detached HEAD — measured in `tests/repo_posture.rs`,
/// which is why the exemption cannot be inferred from what the probe sees and is carried
/// as a type instead.
///
/// **The variants are not publicly constructible.** The discriminant is a private enum and
/// the only constructor is [`posture_subject`], so no caller inside this crate can hand a
/// seam a forged `DedicatedWorktree` — the class M50's completion audit condemned when
/// `fanout_worktree_paths` decided its subject with `path.is_dir()`, a claim about *shape*
/// where the question was about *bytes*.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostureSubject(Checkout);

/// The discriminant behind [`PostureSubject`] — private, so the type is the gate.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Checkout {
    /// The checkout the user works in: every member of the family is theirs to resolve.
    Live,
    /// A fan-out worktree **jigc** provisioned for one milestone sub-task. It carries no
    /// payload: the evidence is [`posture_subject`]'s three legs, and the *type* is what a
    /// seam holds — a value of this shape cannot be built any other way.
    Dedicated,
}

impl PostureSubject {
    /// **The user's own checkout** — the subject of every act but jigc's two fan-out commit
    /// sites, and the value [`SeamSubject::live`] records.
    ///
    /// Public because a surface that *forecasts* a live seam has to ask the same subject that
    /// seam will: `jigc task finalize` commits in the checkout it was run in, through
    /// [`SeamSubject::live`], whichever directory that is — so its preview asks this rather
    /// than [`posture_subject`], whose `Dedicated` exemption belongs to the boundary's own
    /// handle-holding commits. Constructing the **live** variant forges nothing: the variant
    /// that must not be forgeable is `Dedicated`, and it has no constructor at all outside
    /// [`posture_subject`]'s three legs.
    pub fn live() -> PostureSubject {
        PostureSubject(Checkout::Live)
    }

    /// Whether this subject owes an answer for `member`.
    ///
    /// A [`Checkout::Live`] subject adjudicates the whole family. A dedicated worktree is
    /// exempt from [`PostureMember::HeadDetached`] **and from that member only**: jigc
    /// detached it itself, so refusing there would refuse jigc's own provisioning — while
    /// a merge or rebase left un-concluded *inside* that worktree is as real there as
    /// anywhere, and an unborn HEAD cannot occur in a checkout git created at a commit.
    pub fn adjudicates(&self, member: PostureMember) -> bool {
        !matches!(
            (&self.0, member),
            (Checkout::Dedicated, PostureMember::HeadDetached)
        )
    }

    /// Whether this is a fan-out worktree jigc provisioned — [`posture_subject`]'s three
    /// legs, read back rather than re-derived.
    ///
    /// One caller: the sub-task posture preview, which needs to know whether the path it
    /// built from [`engine::milestone::worktree_path`] is a live provisioned worktree
    /// before it asks that worktree anything. Re-spelling the legs there is the shape
    /// M50's completion audit condemned, so the classifier answers instead.
    pub fn is_dedicated(&self) -> bool {
        matches!(self.0, Checkout::Dedicated)
    }
}

/// Classify `repo_root` as a [`PostureSubject`] — the one constructor.
///
/// A dedicated worktree must satisfy **three** legs, none of them a path-shape guess on
/// its own:
///
/// 1. `.git` is a **file** — only a real linked worktree keeps a `gitdir:` pointer there,
///    which no `create_dir_all(".git")` fixture produces;
/// 2. the path is the one [`engine::milestone::worktree_path`] mints, asked of the
///    production constant rather than by restating `.jigc/worktrees` here;
/// 3. the last component is a **registered sub-task** of a milestone in the shared
///    workbench ([`engine::milestone::owning_milestone`]) — the registry leg, which is
///    what makes this an answer about jigc's own provisioning rather than about a
///    directory named like one.
///
/// Anything else is the user's own checkout. The fallback direction is deliberate: a
/// misread live checkout refuses a commit the user can unblock with the routed git
/// command, while a misread worktree would exempt a posture nobody adjudicates.
pub fn posture_subject(repo_root: &Path) -> PostureSubject {
    let live = PostureSubject(Checkout::Live);
    // (1) A linked worktree — and only a linked worktree — keeps `.git` as a FILE.
    if !repo_root.join(".git").is_file() {
        return live;
    }
    let Some(sub_task) = repo_root.file_name().and_then(|name| name.to_str()) else {
        return live;
    };
    // (2) The home jigc's own fan-out provisions into.
    if !repo_root.ends_with(engine::milestone::worktree_path(sub_task)) {
        return live;
    }
    // (3) The registry leg: this id is a sub-task of a milestone in the shared workbench.
    let Some(home) = jigc_home(repo_root) else {
        return live;
    };
    if engine::milestone::owning_milestone(&home.join(".jigc"), sub_task).is_none() {
        return live;
    }
    PostureSubject(Checkout::Dedicated)
}

/// **What a seam is about to do** — the class whose posture family it adjudicates
/// ([`settle-record.md`](../../../completions/artifacts/M51/settle-record.md) → Review
/// amendments §3: *the two classes take different postures*).
///
/// The same split [`crate::cli::ActsOnBehalf`] makes at the door, re-stated at the act,
/// because the act is what the family is about: a `git commit` the user did not type
/// adjudicates all three members, a `git mv` of a committed file adjudicates only the
/// operation in progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamAct {
    /// A `git commit` the user did not type — or the fast-forward that lands one on their
    /// branch. The **full** family, minus the subject's stated exemptions.
    Commit,
    /// A `git mv` of a committed file, committing nothing:
    /// [`PostureMember::OperationInProgress`] only. Which commit the move joins stays the
    /// user's to decide, so a detached or unborn HEAD is none of the mover's business.
    Move,
}

/// **The typed subject of a commit / move / fast-forward seam** — *which checkout is this,
/// and what was it when this act was decided?*
///
/// `settle-record.md` → Review amendments §3 requires the subject to be **typed and
/// re-probed at the seam**, *"not only at the door, because an ordinary commit still
/// reaches `git_commit_capture` and a hook, a concurrent process or an earlier phase can
/// move HEAD in between"*. Two properties follow, and both are why this is a value rather
/// than a `&Path`:
///
///   * **It cannot be sniffed.** A fan-out worktree answers `git symbolic-ref -q HEAD`
///     with exit 1 — byte-identical to a user's detached HEAD (driven,
///     `tests/repo_posture.rs`) — and the throwaway worktree
///     [`crate::task::commit_combined_tree_with_hooks`] commits in is not even a
///     registered sub-task, so [`posture_subject`]'s three legs would classify it *live*
///     and refuse jigc's own provisioning. The seam is **told**, from a live
///     `DedicatedWorktree` handle, or it is live.
///   * **It cannot be forged.** [`SeamSubject::dedicated`] takes a
///     `&`[`crate::task::DedicatedWorktree`] — a handle whose own constructor is private
///     to `task.rs` and whose fields are private — so no caller can hand a seam a
///     dedicated subject it did not provision. A public variant or a caller-supplied
///     boolean is forgeable inside the codebase, the class M50's completion audit
///     condemned in `fanout_worktree_paths`' `is_dir()`.
///
/// **What [`verify`](SeamSubject::verify) re-probes is repository *identity* and *expected
/// ref*, not merely "HEAD is attached":** the canonical git dir this subject was built
/// against, and the symbolic ref HEAD pointed at then. A commit that lands on a branch
/// nobody asked for is the damage; *attached* does not exclude it.
pub struct SeamSubject {
    /// The checkout the seam runs `git` in — `current_dir` for every command below.
    path: PathBuf,
    /// The canonical per-worktree git dir at construction; `None` when unanswerable.
    identity: Option<PathBuf>,
    /// The symbolic ref HEAD pointed at when the subject was built: `Some(Some(ref))`
    /// attached, `Some(None)` detached, **`None` unanswerable**. A dedicated worktree
    /// records `Some(None)` — git created it `--detach` — so *"what it was"* is one field
    /// for both variants, and the drift check is one comparison.
    head: Option<Option<String>>,
    /// The exemption half, reused rather than re-derived: [`PostureSubject::adjudicates`]
    /// is the one home of *a dedicated worktree is exempt from `HeadDetached` and from
    /// that member only*.
    posture: PostureSubject,
    /// The members **this door** does not adjudicate, from its [`crate::cli::BEHALF_DOORS`]
    /// row. Empty everywhere but `jigc setup`'s install commit.
    exempt: &'static [PostureMember],
}

impl SeamSubject {
    /// The user's checkout, on whatever ref it is on now — the subject of every seam but
    /// the two fan-out commit sites.
    pub fn live(repo_root: &Path) -> SeamSubject {
        SeamSubject::live_exempt(repo_root, &[])
    }

    /// [`live`](SeamSubject::live) with the door's stated exemptions carried through.
    ///
    /// One caller ships: `jigc setup`'s install commit, exempt from
    /// [`PostureMember::HeadUnborn`] with the M30 audit rationale quoted at
    /// [`crate::cli::SETUP_UNBORN_EXEMPTION`]. A hole in a family is a decision or it is a
    /// bug, and this parameter is how a seam states which.
    pub fn live_exempt(repo_root: &Path, exempt: &'static [PostureMember]) -> SeamSubject {
        SeamSubject {
            path: repo_root.to_path_buf(),
            identity: git_dir_identity(repo_root),
            head: head_ref(repo_root),
            posture: PostureSubject::live(),
            exempt,
        }
    }

    /// A worktree **jigc provisioned and still holds the handle to** — the one
    /// constructor of the dedicated variant, and it takes neither a path nor a boolean.
    ///
    /// The handle is the evidence: [`crate::task::DedicatedWorktree::add`] is private to
    /// `task.rs` and its fields are private, so a `&DedicatedWorktree` in hand is a
    /// worktree this process created with `git worktree add --detach` and has not yet
    /// dropped. That is what makes the [`PostureMember::HeadDetached`] exemption an answer
    /// about jigc's own provisioning rather than about a directory that looks like one.
    pub(crate) fn dedicated(worktree: &crate::task::DedicatedWorktree) -> SeamSubject {
        let path = worktree.path();
        SeamSubject {
            path: path.to_path_buf(),
            identity: git_dir_identity(path),
            head: head_ref(path),
            posture: PostureSubject(Checkout::Dedicated),
            exempt: &[],
        }
    }

    /// The checkout the seam runs `git` in.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// **Re-probe, immediately before the act.** `Ok(())` iff this is still the checkout
    /// the subject was built against, still on the ref it was built on, and in a posture
    /// `act` may run in.
    ///
    /// Three questions, in this order, because each is only meaningful once the one before
    /// it has held:
    ///
    ///   1. **Identity** — the canonical git dir is the one this subject was built
    ///      against. (The `GIT_DIR` **redirect** is a declared bound, module header: this
    ///      compares the path's *own* git dir before and after, and does not adjudicate an
    ///      ambient environment pointing git elsewhere.)
    ///   2. **The posture family**, filtered by `act` and by the subject's exemptions —
    ///      the same three members the door adjudicated, re-asked at the act, each
    ///      refusing with its registered `repo.*` code and a [`Route::human`] naming the
    ///      git command that resolves it.
    ///   3. **The expected ref** — the drift the family cannot express: HEAD is attached,
    ///      to a *different* branch than the one this act was decided for, or a dedicated
    ///      worktree has been re-attached under us.
    ///
    /// A probe that cannot answer reads as **no breach** (module header): an unanswerable
    /// identity or head on either side skips that comparison rather than refusing on a
    /// fact nobody has.
    pub fn verify(&self, act: SeamAct) -> anyhow::Result<()> {
        if let (Some(built), Some(now)) = (&self.identity, git_dir_identity(&self.path))
            && *built != now
        {
            anyhow::bail!(
                "refusing to act in {:?}: it is no longer the repository this command began \
                 in (its git dir moved from {built:?} to {now:?})\n\
                 route: re-run this command from the repository you meant",
                self.path,
            );
        }
        if let Some(breach) = posture(&self.path)
            .into_iter()
            .find(|breach| self.adjudicates(breach.member(), act))
        {
            return Err(crate::render::finding_error(&breach.finding()));
        }
        if let (Some(built), Some(now)) = (&self.head, head_ref(&self.path))
            && *built != now
        {
            anyhow::bail!("{}", self.drift_message(built.as_deref(), now.as_deref()));
        }
        Ok(())
    }

    /// Whether this subject owes an answer for `member` while doing `act` — the door's
    /// two rules, composed: a [`SeamAct::Move`] adjudicates the operation in progress and
    /// nothing else, and a commit adjudicates everything this subject does not exempt.
    fn adjudicates(&self, member: PostureMember, act: SeamAct) -> bool {
        match act {
            SeamAct::Move => member == PostureMember::OperationInProgress,
            SeamAct::Commit => self.posture.adjudicates(member) && !self.exempt.contains(&member),
        }
    }

    /// The refusal for a ref that moved under us — stated as *what this act was decided
    /// for* versus *what is there now*, with a route naming the git command that puts it
    /// back. It carries no code: the three `repo.*` identities name **postures**, and a
    /// checkout that moved between the door and the act is not one of them.
    fn drift_message(&self, built: Option<&str>, now: Option<&str>) -> String {
        match (built, now) {
            (Some(built), Some(now)) => format!(
                "refusing to act in {:?}: HEAD was on `{built}` when this command began and \
                 is now on `{now}`\n\
                 route: re-attach it with `git switch {}`, then re-run this command",
                self.path,
                crate::task::shell_token(built.strip_prefix("refs/heads/").unwrap_or(built)),
            ),
            (Some(built), None) => format!(
                "refusing to act in {:?}: HEAD was on `{built}` when this command began and \
                 is now detached\n\
                 route: re-attach it with `git switch {}`, then re-run this command",
                self.path,
                crate::task::shell_token(built.strip_prefix("refs/heads/").unwrap_or(built)),
            ),
            (None, Some(now)) => format!(
                "refusing to act in {:?}: this is a worktree jigc provisioned detached, and \
                 HEAD is now on `{now}`\n\
                 route: re-run the milestone boundary against an untouched fan-out",
                self.path,
            ),
            // Unreachable: the caller compared the two and found them different.
            (None, None) => format!("refusing to act in {:?}: HEAD moved", self.path),
        }
    }
}

/// The **canonical per-worktree git dir** of `repo_root` — a checkout's identity for
/// [`SeamSubject::verify`]'s first question. `None` when the `.git` entry does not
/// resolve or the path cannot be canonicalized, which reads as *unanswerable*, never as
/// *different*.
fn git_dir_identity(repo_root: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(worktree_git_dir(repo_root)?).ok()
}

/// Probe `repo_root` for the repository-posture family — the breaches, in
/// [`PostureMember::ALL`] order, one per [`InProgress`] operation actually found.
///
/// Empty means **committable posture or unanswerable**: see the module header for why a
/// probe that cannot answer reads as no breach, and for the `GIT_DIR` bound this probe
/// declares out.
///
/// **The probe answers about the repository it was handed, or not at all.** It resolves
/// `repo_root`'s own git dir first ([`worktree_git_dir`]) and asks nothing when that
/// resolution fails or lands on a directory with no `HEAD` — because git's own walk-up
/// would then answer about a **real ancestor repository**, which is the module header's
/// binding hazard reappearing as a *verdict* rather than as a path. That is also what
/// keeps the crate's fake-`create_dir_all(".git")` fixtures answering nothing instead of
/// inheriting the posture of whatever repository encloses them.
pub fn posture(repo_root: &Path) -> Vec<PostureBreach> {
    let mut breaches = Vec::new();
    let Some(git_dir) = worktree_git_dir(repo_root) else {
        return breaches;
    };
    if !git_dir.join("HEAD").exists() {
        return breaches;
    }
    // The OPERATION is answered FIRST, before the HEAD it detached (M52 Increment 3;
    // `settle-record.md` → D2.2). Both consumers take the first breach they adjudicate,
    // and a stopped rebase detaches HEAD itself: answered detached-first, three states
    // routed the user at a `git switch` git refuses at exit 128 while the operation that
    // caused the detachment went unnamed
    // ([baseline-posture.md](../../../completions/artifacts/M52/baseline-posture.md)
    // §2.2, §3.1). The cause before the symptom.
    // Asked once, of the same classifier the exemption is keyed on, and stamped on every
    // breach this probe raises — so the render can say *which* checkout this is without a
    // second opinion about it (the `repo.head-detached` route, M53's rc.20 `(2, A2-2)`).
    let provisioned = provisioned_sub_task(repo_root);
    if let Some(operation) = operation_in_progress(&git_dir, repo_root) {
        breaches.push(PostureBreach {
            member: PostureMember::OperationInProgress,
            operation: Some(operation),
            provisioned: provisioned.clone(),
        });
    }
    if head_is_detached(repo_root) == Some(true) {
        breaches.push(PostureBreach {
            member: PostureMember::HeadDetached,
            operation: None,
            provisioned: provisioned.clone(),
        });
    }
    // An unborn HEAD is a *symbolic* ref to a branch that does not exist yet, so it is
    // never also detached — the two members are disjoint by construction, not by ordering.
    if crate::task::head_is_unborn(repo_root).unwrap_or(false) {
        breaches.push(PostureBreach {
            member: PostureMember::HeadUnborn,
            operation: None,
            provisioned,
        });
    }
    breaches
}

/// The sub-task `repo_root` is jigc's own provisioned fan-out worktree for, or `None`.
///
/// It asks [`posture_subject`] rather than re-reading its three legs — the shape M50's
/// completion audit condemned — and reads the id back off the path only once that classifier
/// has answered *dedicated*, which is the verdict that already required the last component to
/// be a **registered** sub-task of a milestone in the shared workbench.
fn provisioned_sub_task(repo_root: &Path) -> Option<String> {
    if !posture_subject(repo_root).is_dedicated() {
        return None;
    }
    repo_root
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
}

/// `Some(true)` when HEAD points at a commit rather than a branch, `Some(false)` when it
/// is attached, **`None` when the probe cannot answer**.
///
/// `git symbolic-ref -q HEAD` exits **0** for an attached HEAD (an *unborn* one included —
/// its ref simply does not resolve yet) and **1** for a detached one; a missing work tree
/// or a broken git answers 128, and a git that cannot be spawned answers nothing at all.
/// The discriminator is [`crate::task::head_is_unborn`]'s, kept shape-for-shape: only the
/// two codes that *are* answers are read as answers.
fn head_is_detached(repo_root: &Path) -> Option<bool> {
    head_ref(repo_root).map(|head| head.is_none())
}

/// The **symbolic ref HEAD points at** in `repo_root` — `Some(Some("refs/heads/main"))`
/// attached (an *unborn* HEAD included: its ref simply does not resolve yet),
/// `Some(None)` detached, and **`None` when the probe cannot answer**.
///
/// `git symbolic-ref -q HEAD` exits **0** printing the ref for an attached HEAD and **1**
/// printing nothing for a detached one; a missing work tree or a broken git answers 128,
/// and a git that cannot be spawned answers nothing at all. Only the two codes that *are*
/// answers are read as answers — [`crate::task::head_is_unborn`]'s discriminator, kept
/// shape-for-shape.
///
/// [`head_is_detached`] is the attached/detached projection of this; [`SeamSubject`]
/// records the **value**, because a seam that only asked *is HEAD attached?* would let a
/// commit land on a branch nobody asked for.
pub(crate) fn head_ref(repo_root: &Path) -> Option<Option<String>> {
    let out = std::process::Command::new("git")
        .args(["symbolic-ref", "-q", "HEAD"])
        .current_dir(repo_root)
        .output()
        .ok()?;
    match out.status.code() {
        Some(0) => Some(Some(
            String::from_utf8_lossy(&out.stdout).trim().to_string(),
        )),
        Some(1) => Some(None),
        _ => None,
    }
}

/// **The** operation git has left un-concluded in this worktree — the **first** member of
/// [`InProgress::ALL`] whose own `detect` answers, `None` when none does.
///
/// One answer, not a list, because several members are true at once by construction and
/// only one of them is what the user is in the middle of: a conflicting merge, pick,
/// revert or rebase also leaves **unmerged paths**, and a multi-commit pick is also a
/// **queue**. Probe order is what picks the right one — the pick and the revert before
/// the queue they own, the queue before the bare unmerged index — so the route names the
/// command that resolves *this* state rather than a menu, and
/// [`InProgress::UnmergedIndex`]'s `git ls-files -u` is reached only where no marker on
/// disk answered.
fn operation_in_progress(git_dir: &Path, repo_root: &Path) -> Option<InProgress> {
    InProgress::ALL
        .into_iter()
        .find(|operation| operation.detect(git_dir, repo_root))
}

/// The **per-worktree** git dir of `repo_root` — where `MERGE_HEAD`, `rebase-merge` /
/// `rebase-apply` and `BISECT_LOG` live for *this* checkout, so a linked worktree answers
/// about itself and never about its main checkout (driven: a merge started in a linked
/// worktree leaves `MERGE_HEAD` in `.git/worktrees/<name>/`, and the main `.git` has none).
///
/// Resolved from the `.git` entry itself rather than by shelling out — the module header's
/// walk-up property, kept: an empty fake `.git` directory resolves to itself, and
/// [`posture`] then finds no `HEAD` there and asks nothing, where a shell-out would walk
/// up and answer about a real ancestor repository.
fn worktree_git_dir(repo_root: &Path) -> Option<PathBuf> {
    let dot_git = repo_root.join(".git");
    if dot_git.is_dir() {
        return Some(dot_git);
    }
    // A linked worktree keeps `.git` as a FILE holding `gitdir: <path>`.
    let pointer = std::fs::read_to_string(&dot_git).ok()?;
    let target = pointer
        .lines()
        .find_map(|line| line.strip_prefix("gitdir:"))?
        .trim();
    if target.is_empty() {
        return None;
    }
    let target = Path::new(target);
    Some(if target.is_absolute() {
        target.to_path_buf()
    } else {
        repo_root.join(target)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway directory that removes itself on drop (the project's no-tempfile
    /// pattern). Cleans up worktrees too — `remove_dir_all` flattens the tree.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-repo-unit-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn git(dir: &Path, args: &[&str]) {
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    }

    /// (a) Outside any worktree, `jigc_home` returns the byte-identical path the
    /// walk-up resolver does — the behavior-preserving guarantee.
    #[test]
    fn jigc_home_matches_walk_up_outside_worktree() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let start = dir.path();

        assert_eq!(
            jigc_home(start),
            discover_repo_root(start),
            "outside a worktree jigc_home must equal the walk-up repo root",
        );
        assert_eq!(jigc_home(start).as_deref(), Some(dir.path()));
    }

    /// (b) Inside a linked worktree (`.git` is a file), `jigc_home` returns the MAIN
    /// checkout, not the worktree root.
    #[test]
    fn jigc_home_resolves_to_main_checkout_from_worktree() {
        let main = TempDir::new();
        git(main.path(), &["init", "-q"]);
        git(main.path(), &["config", "user.email", "t@t"]);
        git(main.path(), &["config", "user.name", "t"]);
        git(
            main.path(),
            &["commit", "-q", "--allow-empty", "-m", "init"],
        );

        let linked = main.path().join("linked");
        git(
            main.path(),
            &["worktree", "add", "-q", linked.to_str().unwrap()],
        );
        assert!(
            linked.join(".git").is_file(),
            "a linked worktree's .git must be a file",
        );

        // The worktree's walk-up root is the worktree itself...
        assert_eq!(
            discover_repo_root(&linked).as_deref(),
            Some(linked.as_path())
        );
        // ...but jigc_home redirects to the main checkout.
        let home = jigc_home(&linked).expect("jigc_home resolves from a worktree");
        assert_eq!(
            std::fs::canonicalize(&home).unwrap(),
            std::fs::canonicalize(main.path()).unwrap(),
            "jigc_home from a worktree must be the main checkout, not the worktree",
        );
        assert_ne!(
            std::fs::canonicalize(&home).unwrap(),
            std::fs::canonicalize(&linked).unwrap(),
        );
    }

    /// (c) The load-bearing red: a fake `.git` fixture (empty `create_dir_all(".git")`,
    /// no `git init`) nested *inside a real git repo* resolves to the fake-bearing dir
    /// itself — the walk-up fallback fires and never shells out to the real ancestor.
    #[test]
    fn jigc_home_fake_git_does_not_shell_to_ancestor() {
        let ancestor = TempDir::new();
        // A real repo as the ancestor — what git WOULD bind to if we shelled out.
        git(ancestor.path(), &["init", "-q"]);

        let fake = ancestor.path().join("nested");
        std::fs::create_dir_all(fake.join(".git")).expect("seed a fake .git dir");

        assert_eq!(
            jigc_home(&fake).as_deref(),
            Some(fake.as_path()),
            "a fake .git must resolve to itself (walk-up fallback), not the ancestor",
        );
        assert_ne!(
            jigc_home(&fake).as_deref(),
            Some(ancestor.path()),
            "a fake .git must NOT shell out and bind to the real ancestor repo",
        );
    }
}
