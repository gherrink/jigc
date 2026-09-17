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

use engine::finding::{Finding, Route};
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
/// returning that directory (the worktree root). The same walk-up every command module
/// owns; co-located here so [`jigc_home`] can layer over it — and reached by
/// [`crate::cli`]'s posture guard, which runs before any door has resolved a repo.
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
/// menu of nine.
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
    /// Three of the orderings are load-bearing and each is driven:
    /// [`InProgress::Am`] and [`InProgress::Rebase`] are disjoint by predicate rather
    /// than by position; [`InProgress::CherryPick`] and [`InProgress::Revert`] come
    /// **before** [`InProgress::Sequencer`]; and [`InProgress::UnmergedIndex`] is
    /// **last**, so the shell-out it needs is reached only where no marker answered.
    pub const ALL: [InProgress; 9] = [
        InProgress::Merge,
        InProgress::SquashMerge,
        InProgress::Rebase,
        InProgress::Am,
        InProgress::CherryPick,
        InProgress::Revert,
        InProgress::Sequencer,
        InProgress::Bisect,
        InProgress::UnmergedIndex,
    ];

    /// **Is this operation in progress in `repo_root`?** — the per-variant predicate
    /// that replaced a shared marker list.
    ///
    /// Eight members read the **worktree's own** git dir, so a linked worktree answers
    /// about itself. The ninth asks git, because the state it names leaves nothing on
    /// disk to read.
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
            InProgress::UnmergedIndex => index_has_unmerged_paths(repo_root),
        }
    }

    /// How the message names it — **this** operation, so a user in the middle of one of
    /// nine things learns which one.
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

    /// The git command that **concludes** it, where git has one — named with the
    /// qualifier that makes it true, since a conflicted operation concludes only after
    /// the user has resolved it.
    ///
    /// `None` for the four members with no such command: a squash merge is concluded by
    /// the user's own `git commit`, a dangling `sequencer/` has no current commit to
    /// continue, a bisect ends rather than concludes, and an unmerged index is resolved
    /// rather than continued.
    fn conclude_command(self) -> Option<&'static str> {
        match self {
            InProgress::Merge => Some("git merge --continue"),
            InProgress::Rebase => Some("git rebase --continue"),
            InProgress::Am => Some("git am --continue"),
            InProgress::CherryPick => Some("git cherry-pick --continue"),
            InProgress::Revert => Some("git revert --continue"),
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
            InProgress::UnmergedIndex => "git reset --merge",
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

    /// The blocking [`Finding`] this breach refuses with — §10's mold: a code, a
    /// [`Route::human`] naming **the git command that resolves the state**, and no
    /// override. A posture is a repository state the user can resolve, not bytes only
    /// they can value, so the family carries no consent flag.
    pub fn finding(&self) -> Finding {
        let (message, route) = match (self.member, self.operation) {
            (PostureMember::HeadDetached, _) => (
                "HEAD is detached — a commit made here would belong to no branch, and the \
                 next checkout would leave it unreachable"
                    .to_string(),
                "re-attach HEAD with `git switch <branch>`, then re-run this command".to_string(),
            ),
            (PostureMember::HeadUnborn, _) => (
                "HEAD is unborn — this repository has no commits yet, so there is no base \
                 for jigc to commit against"
                    .to_string(),
                "land the repository's first commit with `git commit`, then re-run this \
                 command"
                    .to_string(),
            ),
            (PostureMember::OperationInProgress, operation) => {
                // `posture` never builds this member without an operation; the fallback
                // keeps the renderer total rather than panicking on a shape it owns.
                let operation = operation.unwrap_or(InProgress::Merge);
                let route = match operation.conclude_command() {
                    Some(conclude) => format!(
                        "conclude it with `{conclude}` once its conflicts are resolved, or \
                         abandon it with `{}`, then re-run this command",
                        operation.abandon(),
                    ),
                    None => format!(
                        "conclude it, or abandon it with `{}`, then re-run this command",
                        operation.abandon(),
                    ),
                };
                (
                    format!(
                        "{} {} — the repository is not in a committable state",
                        operation.noun(),
                        operation.predicate(),
                    ),
                    route,
                )
            }
        };
        Finding::block(self.member.code(), message, Route::human(route))
    }
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
            posture: PostureSubject(Checkout::Live),
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
    if let Some(operation) = operation_in_progress(&git_dir, repo_root) {
        breaches.push(PostureBreach {
            member: PostureMember::OperationInProgress,
            operation: Some(operation),
        });
    }
    if head_is_detached(repo_root) == Some(true) {
        breaches.push(PostureBreach {
            member: PostureMember::HeadDetached,
            operation: None,
        });
    }
    // An unborn HEAD is a *symbolic* ref to a branch that does not exist yet, so it is
    // never also detached — the two members are disjoint by construction, not by ordering.
    if crate::task::head_is_unborn(repo_root).unwrap_or(false) {
        breaches.push(PostureBreach {
            member: PostureMember::HeadUnborn,
            operation: None,
        });
    }
    breaches
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
fn head_ref(repo_root: &Path) -> Option<Option<String>> {
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
