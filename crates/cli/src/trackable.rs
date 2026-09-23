//! **Can git track a path here?** — the one predicate every door that computes a
//! *destination* for a managed doc asks before it moves or writes one there.
//!
//! The rule is M48 Increment 5's, generalized off the install commit's `pre-commit`
//! hook and onto any repo-relative path: a path is trackable **iff** it is under the
//! canonicalized repo root, outside git's own dirs, and owned by *this* repository
//! (not a submodule or an embedded repo). [`crate::setup::committable_hook_path`] is
//! the shape it was first written in, and now calls this — one rule, one home, so the
//! next door to ask cannot get a different answer than the install commit does.
//!
//! **Why the movers need it (M49).** `git mv <src> .git/<dest>` prints
//! `error: invalid path '.git/<dest>'` on stderr and **exits 0**: the file moves on
//! disk, the source leaves the index, and nothing is added — a staged deletion with no
//! matching add. Every doc-relocating door read that exit 0 as a successful move and
//! said so, so `jigc config set placement-root .git` reported the relocation, exited 0,
//! and left the doc surviving only in history — gone from the next clone. An exit code
//! is therefore not a trackability test; this is.
//!
//! **Two families live here, and they are not the same question** (M51 Increment 1). The
//! original is the *destination* rule above. The second is the **source** rule
//! ([`resolve_source_token`], [`source_read_reason`]) — asked by the doors that read a
//! caller-named file *in*, where the question is not *can git record this path* but *is this a
//! file jigc is willing to open*. They answer differently on purpose: a destination outside
//! the repository is refused, a source outside it is admitted and copied in.
//!
//! **What is *not* asked: gitignore.** A gitignored destination is a perfectly
//! trackable path git has merely been told to skip — `git mv docs/x.md .jigc/x.md`
//! stages a real `R` rename — and the workbench relocation
//! ([`crate::relocate`]'s squatter displacement) depends on exactly that. Ignoring is a
//! policy about a path git *can* record; this predicate is about paths it cannot.

use std::path::{Path, PathBuf};

/// Why `relative` (a repo-root-relative path, existing or not) is a destination git
/// **cannot record** in the repository at `repo_root` — `None` when it can.
///
/// The five tests, in the order they can be answered most cheaply, and none of them
/// optional:
///
///   - **under the repo root** — a destination reached through `..` is outside the tree
///     git commits from. (`git mv` already rejects this one loudly, at exit 128; it is
///     tested here so the *door* can refuse before it moves anything.)
///   - **no `.git` path component** — git refuses to record any path with a `.git`
///     component (`error: invalid path`), whatever the git dir actually is, so this
///     holds in a linked worktree where `.git` is a *file* and the dirs below name a
///     different tree.
///   - **outside git's own dirs** — `--git-dir` *and* `--git-common-dir`, both printed
///     by one `rev-parse`. The literal-component test above does not subsume this: a
///     `--separate-git-dir` / `GIT_DIR` repo keeps its object store under a directory
///     that is not called `.git` at all.
///   - **owned by *this* repository** — location is not trackability. A directory under
///     the root can belong to another repo (a submodule, or a plain embedded repo), and
///     a path inside one is not this index's to take. Git is asked, from the nearest
///     ancestor that exists, which repo owns the destination.
///   - **not under a gitlink in the *index*** — the same ownership question asked of the
///     index, which is the only side that can see a submodule that is registered but not
///     checked out (a `git clone` without `--recursive`, the default clone).
///
/// **Conservative toward the move**: when git cannot be asked at all, the git-side tests
/// abstain and the answer rests on the two structural ones. A door that then moves is no
/// worse off than before — the `git mv` it is about to run needs the same git.
pub(crate) fn untrackable_reason(repo_root: &Path, relative: &str) -> Option<String> {
    let relative = relative.trim_matches('/');
    if relative.is_empty() || relative == "." {
        return None; // the repo root itself is always trackable.
    }

    // 1 — under the repo root. Resolved lexically (the destination need not exist yet);
    // an existing path is canonicalized so a symlinked ancestor cannot smuggle the
    // target out from under the tests below.
    let root = std::fs::canonicalize(repo_root).ok()?;
    let target = resolve(&root, relative)?;
    if !target.starts_with(&root) {
        // No path at all: the subject IS the repository, whose repo-relative spelling is
        // `.`, and `resolves outside the repository at .` is noise. Naming the host root
        // here was the M50 completion audit's finding 3 — law 1 (`a surface prints no host
        // filesystem`) enforced at the four destroying doors and nowhere else, while this
        // one predicate hands its text to four more.
        return Some(format!("`{relative}` resolves outside the repository root"));
    }

    // 2 — git's own `invalid path` rule: no `.git` component, at any depth. Compared
    // case-insensitively because git's is (`core.protectNTFS`/`protectHFS` exist so the
    // rule cannot be dodged by case on the filesystems where case does not bind).
    if Path::new(relative)
        .components()
        .any(|c| c.as_os_str().eq_ignore_ascii_case(".git"))
    {
        return Some(format!(
            "`{relative}` is inside git's own directory — git refuses to track any path \
             with a `.git` component (`error: invalid path`), so the bytes would survive \
             only in history"
        ));
    }

    // 3 — git's own dirs, wherever they actually live.
    if let Some(out) = git_output(
        repo_root,
        [
            "rev-parse",
            "--path-format=absolute",
            "--git-dir",
            "--git-common-dir",
        ],
    ) && out.status.success()
    {
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            let git_dir = line.trim();
            if git_dir.is_empty() {
                continue;
            }
            // A git dir that does not resolve cannot contain the destination.
            if std::fs::canonicalize(git_dir).is_ok_and(|dir| target.starts_with(&dir)) {
                // Rendered against the root, not printed as git handed it over: a git dir
                // reachable from a destination *under* the root is itself under the root
                // (`.git`, or a `--separate-git-dir` inside the tree), so this has a
                // repo-relative spelling. The helper's absolute fallback covers the
                // pathological case where it does not.
                let git_dir = engine::path::repo_relative(&root, Path::new(git_dir));
                return Some(format!(
                    "`{relative}` is inside this repository's git directory ({git_dir}) — \
                     git records nothing there"
                ));
            }
        }
    }

    // 4 — ownership, asked from the nearest ancestor that exists on disk (the
    // destination itself may not).
    let mut probe = target.as_path();
    let owner = loop {
        if probe.is_dir() {
            break git_output(probe, ["rev-parse", "--show-toplevel"]);
        }
        match probe.parent() {
            Some(parent) if parent.starts_with(&root) || parent == root => probe = parent,
            _ => break None,
        }
    };
    if let Some(out) = owner
        && out.status.success()
    {
        let toplevel = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if std::fs::canonicalize(&toplevel).is_ok_and(|owner| owner != root) {
            // The owning repo was found by walking UP from a destination under the root and
            // stopping at the root, so its toplevel is under the root and has a
            // repo-relative spelling by construction.
            let toplevel = engine::path::repo_relative(&root, Path::new(&toplevel));
            return Some(format!(
                "`{relative}` is inside another repository ({toplevel}) — a submodule or an \
                 embedded repo, whose paths this index cannot record"
            ));
        }
    }

    // 5 — …and the same question of the index, the only side that can see a registered
    // but un-checked-out submodule.
    if index_gitlink_covers(repo_root, relative) {
        return Some(format!(
            "`{relative}` is inside a submodule registered in this repository's index — a \
             pathspec inside a submodule matches nothing"
        ));
    }

    None
}

/// Adjudicate a caller-supplied path token that names a **source file inside this
/// repository**, answering with its clean repo-relative spelling — or with the reason it is
/// not one (M51 Increment 1 / T1; `completions/artifacts/M51/settle-record.md` → D1 parts 1+3,
/// as amended by §2).
///
/// **Three predicates, not two, and a resolve step before any of them.** D1 decided the door
/// would ask [`untrackable_reason`] + [`crate::config::is_workbench_root`] and called that
/// *"shipped predicates, no new capability"*. ***That is struck.*** The claim does not survive
/// being driven on the cell it was decided for: this module's own parameter is typed
/// *repo-root-relative* and [`untrackable_reason`] opens `relative.trim_matches('/')`
/// (`design/storage.md` says so in its own words), so `/private/tmp/victim/keepme.md` is
/// re-read as `<repo>/private/tmp/victim/keepme.md` and answers **trackable**. The
/// resolve-or-refuse step below is therefore a **new rule**, small and stated, and what the
/// `jigc config set` door actually refuses an absolute value with is a **third** predicate D1
/// never named — `config::unusable_root_reason`, whose symlink leg
/// ([`crate::config::symlinked_component`]) is the one this door reuses.
///
/// The four steps, in order:
///
///   1. **Resolve, or refuse** — the token is placed inside the repository
///      ([`place_inside`]); one that lands outside it is refused *as itself*, never folded
///      back in. A `..` that climbs above the root was previously folded **lexically**, so a
///      source one directory up was recorded as a repo-relative spelling naming a file that
///      is not the operator's.
///   2. **[`untrackable_reason`]** of the *resolved* value — git's own `.git`-component and
///      git-dir rules, ownership, and the index's gitlinks.
///   3. **[`crate::config::is_workbench_root`]** — jigc's own `.jigc/` tree, which git tracks
///      perfectly well and `jigc uninstall` removes whole.
///   4. **[`crate::config::symlinked_component`]** — the symlink leg of the root knobs'
///      usability rule, and only that leg: its file-shaped sibling refuses an existing
///      non-directory, which is exactly what a *source* is. The leg is shared; its sentence is
///      not, because the root knobs' names a consequence (`RD`-staged docs) that a source does
///      not have.
///
/// **Stated bound: the adapter-install leg is deliberately not asked here** (M52 Increment 8 /
/// T7). [`crate::config::installed_artifact_root`] joined that same usability rule as its
/// seventh shape, and it is a **destination** rule in the sense step 3 already draws: it
/// refuses a directory managed docs would be *resolved against*, or swept *out of*. A source
/// under the adapter's install root is neither — `jigc migrate .claude/skills/jigc/SKILL.md`
/// reads a committed file and retires it on `--approve`, which is recoverable from the index,
/// so the ground step 3 stands on (*bytes no index has a copy of*) does not transfer. Refusing
/// it would narrow a shipped affordance to close nothing, which is the same call
/// [`source_read_reason`]'s out-of-repo admission takes.
///
/// The reason is a sentence, not a code: one door, one code, the reason in the message — the
/// `config.untrackable-root` precedent, where the operator's fix is the same whichever leg
/// answered.
///
/// **The refusal quotes the token as typed**, including an absolute one. Law 1
/// (`design/surface-contract.md`) asks that every printed path be repo-real or a typed
/// identity; a source that resolves outside the repository *has no repo-real spelling* — the
/// `locate::not_in_repo_message` case, absolute by its subject — and the string is the
/// operator's own argument, which is the one thing they can edit.
/// One git **pathspec magic** form carried by `token`, as the clause that names it — `None`
/// when git reads the token as the literal file name it looks like (M51 Increment 1, the
/// axis fix).
///
/// **The rule is a class, and it shipped as one spelling of it.** T3 refused a *recorded*
/// retirement path `starts_with(':')` and T6 refused a *root-knob value* the same way, both
/// on the same true sentence: `git add -- <path>` prevents **option** parsing and nothing
/// else, so a token git reads as a pathspec stages a set nobody named. But `:` is only git's
/// *prefix* magic; its **wildmatch** magic needs no prefix at all, and that half was left
/// open at every site. Driven at `8bc6f4e` on a corpus holding six tracked `.md` files, with
/// an untracked file whose name is literally `*.md`:
///
/// ```text
/// $ git ls-files -- '*.md'
/// .jigc/AGENT.md CHANGELOG.md CLAUDE.md README.md notes.md …   <- non-empty
/// $ git cat-file -e 'HEAD:*.md'   ->  fatal: path '*.md' does not exist in 'HEAD'
/// ```
///
/// so `jigc migrate '*.md' --as changelog` asked the trackedness leg about **other people's
/// files**, was told `tracked`, minted at exit 0 — and `jigc task finalize --approve` then
/// unlinked the literal `*.md` (recoverable from no git object at all: the exact loss
/// `migrate.source-untracked` was built to prevent) while `git add -- '*.md'` swept an
/// unrelated unstaged `notes.md` into a commit whose message named the changelog.
///
/// **The four metacharacters are git's, not a guess**: `dir.c` matches a pathspec with
/// `wildmatch()` unless `:(literal)` is given, and wildmatch reads `*` and `?` as wildcards,
/// `[` as the opening of a character class, and `\` as escaping the byte after it. Each one
/// makes the token match paths it does not name, so each one is a name this door cannot hand
/// git.
///
/// **It names the magic and stops there.** The consequence is the caller's sentence — a
/// source is *read and then deleted*, a root knob *prefixes every managed doc's path* — which
/// is the same split every other rule in this module takes, and the reason the clause is
/// returned rather than a `bool`.
pub(crate) fn pathspec_magic_reason(token: &str) -> Option<String> {
    if token.starts_with(':') {
        return Some(format!(
            "`{token}` begins with `:`, which git reads as pathspec magic (`:(top)`, `:!`) \
             and not as a name"
        ));
    }
    wildmatch_magic_reason(token)
}

/// The **wildmatch half** of [`pathspec_magic_reason`], asked on its own — `None` when the
/// token carries none of git's pattern bytes.
///
/// It is split out for the one subject whose *prefix* magic is not this rule's to refuse: a
/// root knob's second and later components sit in the middle of every pathspec built from
/// them, and git reads `:` only at the **start** of one, so `docs/:x` is an ordinary home
/// ([`crate::config`]'s root leg takes that narrowing deliberately). Wildmatch has no such
/// position rule — a `*` in any component patterns the whole pathspec — so that is the half
/// every component is asked.
pub(crate) fn wildmatch_magic_reason(token: &str) -> Option<String> {
    token
        .chars()
        .find(|ch| WILDMATCH_METACHARS.contains(ch))
        .map(|found| {
            format!(
                "`{token}` contains `{found}`, which git reads as pathspec wildmatch \
                 (`*`, `?`, `[…]`, `\\`) and not as part of a name"
            )
        })
}

/// The bytes git's `wildmatch()` reads as something other than themselves — the wildcard half
/// of [`pathspec_magic_reason`], and the whole of it: a pathspec is wildmatched unless the
/// caller asks for `:(literal)`, so any of these in a token makes it a pattern.
const WILDMATCH_METACHARS: &[char] = &['*', '?', '[', '\\'];

pub(crate) fn resolve_source_token(
    repo_root: &Path,
    base: &Path,
    token: &str,
) -> Result<String, String> {
    // 0 — pathspec magic, before anything touches the filesystem, because a token git reads
    // as a pattern is not a path to resolve at all: both callers hand this value to git as a
    // pathspec (the door's trackedness leg `git ls-files`, the sink's `git add`), where a
    // pattern answers about files it does not name. Asked here rather than at each caller for
    // the reason step 1 is: one home, so the door and the sink cannot get different answers.
    if let Some(clause) = pathspec_magic_reason(token) {
        return Err(format!(
            "{clause} — `git add -- <path>` prevents option parsing, never magic, so git \
             would answer about a set of files nobody named instead of about this source, \
             and the retirement would delete bytes no index holds a copy of"
        ));
    }
    let relative = place_inside(repo_root, base, token).ok_or_else(|| {
        format!(
            "`{token}` resolves outside the repository — `jigc migrate` reads its source and, \
             on `--approve`, retires it, so a source outside the tree would be deleted with no \
             copy in any commit of this repository"
        )
    })?;
    if let Some(reason) = untrackable_reason(repo_root, &relative) {
        return Err(reason);
    }
    if crate::config::is_workbench_root(&relative) {
        return Err(format!(
            "`{relative}` is inside jigc's own workbench (`.jigc/`) — the tree `jigc uninstall` \
             removes whole, so a source retired from there leaves bytes no index has a copy of"
        ));
    }
    if let Some(shown) = crate::config::symlinked_component(repo_root, &relative) {
        // The *fact* is the root knobs' — git records the link, never a path through it — but
        // the sentence is this door's: theirs names moved docs staging as `RD`, which is not
        // what happens to a source, and law 1 (`nothing lies`) is a rule about the sentence.
        return Err(format!(
            "`{shown}` is a symlink — git records the link, not a path through it, so \
             retiring the source would stage the removal of a path the worktree no longer \
             has, while the file it points at is untouched"
        ));
    }
    Ok(relative)
}

/// Why the door will **not read** `token` as a source file — `None` when it will (M51
/// Increment 1 / T4; `completions/artifacts/M51/settle-record.md` → §2, *"`file` and
/// `from_file` get a SOURCE rule (S11)"*).
///
/// **A source rule, and the distinction is the whole of §2's correction.** D1 part 1 had
/// pre-committed `jigc config insert-step` / `replace-step` to [`untrackable_reason`] +
/// [`crate::config::is_workbench_root`] — *destination* predicates, which ask the wrong
/// question of a source twice over: the first refuses *"resolves outside the repository
/// root"*, which kills a shared team steps library at `~/steps/foo.yaml`, and the second
/// refuses every path under `.jigc/`, which is precisely where those two verbs **write**
/// (`.jigc/config/steps/<basename>.yaml`). A destination must be a path git can record; a
/// source only has to be one jigc is willing to open.
///
/// The two legs, and what each is about:
///
///   1. **git's own directory** — by the literal `.git` component *and* by the git dirs git
///      itself reports, so a `--separate-git-dir` / `GIT_DIR` repository whose object store is
///      not called `.git` is covered too. Git's private files are not authored content, and
///      the harm was exactly this: driven at `dddc11a5`,
///      `jigc config insert-step … .git/config` exits **0** and copies this repository's git
///      config into `.jigc/config/steps/config.yaml`; that the copy then **composes** —
///      `repositoryformatversion = 0` rendered into the step text `jigc start` hands the
///      agent — is the baseline's own drive
///      (`completions/artifacts/M51/baseline-tokens.md` §2d).
///   2. **jigc's own *transient* workbench** — `.jigc/tasks/`, `.jigc/state/` and their
///      siblings: the subtrees jigc rewrites per task and `jigc uninstall` removes whole. A
///      shadow sourced from there copies a transient into the cascade, where it outlives the
///      thing it was copied from.
///
///      **Transient, not the whole `.jigc/` tree, and the narrowing is forced rather than
///      cautious.** §2's own argument against the *destination* predicate is that
///      [`crate::config::is_workbench_root`] *"refuses any path under `.jigc` — precisely
///      where `insert-step` **writes**"*, so a rule that refused all of `.jigc/` would
///      re-commit the error §2 struck: `.jigc/config/` is the **committed** cascade layer,
///      and sourcing a step from a shadow already there is a shipped, tested affordance
///      (`e2e_audit::scenario_7_structural_op_shifts_composed_bytes_vs_no_override` writes
///      its source to `.jigc/config/extra.yaml`; the first cut of this leg reddened it, which
///      is how the contradiction surfaced). The transient set is
///      [`crate::gitignore::ENTRIES`] — jigc's own declaration of which parts of its
///      workbench are rewritten rather than authored — so a subtree joins this rule by
///      joining that set, and no second list can drift from it.
///
/// **Out-of-repo is deliberately admitted**, and that is a decision rather than an omission:
/// the caller names the file, the bytes are *copied in*, and what lands in the repository is a
/// file their next diff shows. Refusing it would narrow a shipped affordance to close nothing.
///
/// **Readability is not asked here.** §2 states the shared rule as *readable · no `.git`
/// component · not reached through the workbench*; the readable leg is answered by the read
/// itself, one statement later at each door, and its route-floor gap (a bare `anyhow` + errno
/// on the missing/directory shapes) is a pre-existing one this rule neither closes nor widens
/// (`completions/artifacts/M51/baseline-tokens.md` §2d).
///
/// **The leaf IS canonicalized here**, the opposite of [`place_inside`]'s rule, and for the
/// same reason: there, the leaf is the *subject* of a later retire, so resolving it would
/// retire the wrong path; here, the door is about to **read bytes**, and the read follows the
/// link — so the bytes' real home is what the rule has to be asked of. A symlink pointing into
/// `.git/` or `.jigc/` is refused because of where it lands, which is also how *"not reached
/// through the workbench"* is satisfied rather than merely tested for.
pub(crate) fn source_read_reason(cwd: &Path, repo_root: &Path, token: &Path) -> Option<String> {
    let resolved = resolve_source(cwd, token);
    let root = std::fs::canonicalize(repo_root).unwrap_or_else(|_| repo_root.to_path_buf());
    let inside = resolved.strip_prefix(&root).ok();
    // Law 1 (`design/surface-contract.md`: every printed path is repo-real or a typed
    // identity): the repo-relative spelling when the source is inside the repository, and the
    // caller's own argument — the one string they can edit — when it is not, which is the
    // `locate::not_in_repo_message` carve-out [`resolve_source_token`] already takes.
    let shown = match inside {
        Some(tail) => tail.to_string_lossy().into_owned(),
        None => token.to_string_lossy().into_owned(),
    };

    // 1 — git's own directory. The component test is asked of the repo-relative tail when the
    // source is inside the repository, so a repository that itself lives under a directory
    // named `.git` does not refuse its own ordinary files.
    let named_git = inside
        .unwrap_or(resolved.as_path())
        .components()
        .any(|part| part.as_os_str().eq_ignore_ascii_case(".git"));
    if named_git || git_dirs(cwd).iter().any(|dir| resolved.starts_with(dir)) {
        return Some(format!(
            "`{shown}` is inside git's own directory — git's private files are not \
             authored content"
        ));
    }

    // 2 — jigc's own TRANSIENT workbench. A source outside the repository reaches no
    // workbench of this repository's, so the question is only asked of an inside tail.
    if let Some(tail) = inside
        && let Some(subdir) = transient_workbench_subdir(&tail.to_string_lossy())
    {
        return Some(format!(
            "`{shown}` is inside jigc's own transient workbench (`.jigc/{subdir}`) — the \
             tree jigc rewrites per task and `jigc uninstall` removes whole"
        ));
    }

    None
}

/// The **transient** workbench subdirectory `relative` lies inside, when it lies inside one —
/// `None` for every other path, `.jigc/config/` and `.jigc/AGENT.md` included.
///
/// The set is [`crate::gitignore::ENTRIES`], jigc's own declaration of which parts of its
/// workbench are rewritten rather than authored — the lines it writes into `.jigc/.gitignore`.
/// Reading it rather than restating it is the point: a subtree becomes transient by joining
/// that constant, and a second list here would be one `ENTRIES` edit away from lying.
///
/// The first component is asked through [`crate::config::is_workbench_root`] so the
/// case-insensitivity rule for `.jigc` keeps one home; the subject is a repo-relative spelling
/// with `.`/`..` already folded out, which is what the resolved tail is.
fn transient_workbench_subdir(relative: &str) -> Option<&'static str> {
    if !crate::config::is_workbench_root(relative) {
        return None;
    }
    let mut components = Path::new(relative)
        .components()
        .filter_map(|part| match part {
            std::path::Component::Normal(name) => name.to_str(),
            _ => None,
        });
    let _workbench = components.next()?;
    let subdir = components.next()?;
    crate::gitignore::ENTRIES
        .lines()
        .map(|entry| entry.trim_end_matches('/'))
        .find(|entry| *entry == subdir)
}

/// Resolve a **source** token to the real path its bytes live at: absolute as typed, else
/// joined onto `cwd` (which is what the doors' own `fs::read` does), then canonicalized
/// whole — leaf included, per [`source_read_reason`]'s stated asymmetry. A path that does not
/// resolve falls back to the lexical fold: the read one statement later answers it, and the
/// rule still gets a placed subject to ask its questions of.
fn resolve_source(cwd: &Path, token: &Path) -> PathBuf {
    let joined = if token.is_absolute() {
        token.to_path_buf()
    } else {
        cwd.join(token)
    };
    std::fs::canonicalize(&joined)
        .unwrap_or_else(|_| fold_lexically(&joined).unwrap_or_else(|| joined.clone()))
}

/// The git directories git itself reports for the repository at `cwd` — `--git-dir` and
/// `--git-common-dir`, canonicalized — so a `--separate-git-dir` / `GIT_DIR` object store that
/// is not called `.git` is covered by the same leg as the literal component. Empty when git
/// cannot be asked: the predicate form, a git that cannot answer abstains.
fn git_dirs(cwd: &Path) -> Vec<PathBuf> {
    let Some(out) = git_output(
        cwd,
        [
            "rev-parse",
            "--path-format=absolute",
            "--git-dir",
            "--git-common-dir",
        ],
    ) else {
        return Vec::new();
    };
    if !out.status.success() {
        return Vec::new();
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| std::fs::canonicalize(line).ok())
        .collect()
}

/// Place `token` inside `repo_root`, answering with its clean repo-relative spelling — `None`
/// when it lands anywhere else.
///
/// **A relative token joins `base`, and `base` is the caller's cwd at the door that reads a
/// file the operator typed** (M53 — the cwd census, rows C2-03 / C2-04). It joined
/// `repo_root` unconditionally, and that made `jigc migrate` the one verb on the binary whose
/// path argument was **not** what every shell tool means by a path: from `docs/deep`,
/// `jigc migrate note.md` could not find the `note.md` sitting next to the caller, and
/// `jigc migrate ../../rootnote.md` was refused as *"resolves outside the repository"* about
/// a file that is plainly inside it — the `..` was folded against the root instead of against
/// where the caller stood, so the message was **false**. Containment is still judged against
/// `repo_root`; only the join moved. A door whose token is a **lookup key** or a **declared
/// home** (`jigc unmanage`, the two root knobs, a `code-anchor` value) passes `repo_root` as
/// the base, because there the token is not a filesystem path the caller typed but an
/// identity the store is keyed by.
///
/// **Canonicalization-safe on both sides, and asymmetric on purpose.** The repository root and
/// the token's *directory* chain are both canonicalized, because a repo legitimately sits under
/// a symlinked ancestor (every macOS temp corpus lives under `/var` → `/private/var`) and a
/// caller types whichever spelling their shell handed them — comparing the two raw would refuse
/// an ordinary in-repo absolute path. The **final component is never canonicalized**: resolving
/// it would silently rewrite a symlinked source into its target, and whether the token names a
/// link is step 4's question, not this step's.
///
/// That asymmetry between the leaf and its ancestors is the rule, not an accident of the
/// implementation. An ancestor link is only a *route* to the bytes — resolving it records the
/// same file under a spelling git can record — while the leaf **is** the subject: recording a
/// link there would retire the link and leave the bytes, which is why step 4 refuses it.
fn place_inside(repo_root: &Path, base: &Path, token: &str) -> Option<String> {
    let root = std::fs::canonicalize(repo_root).unwrap_or_else(|_| repo_root.to_path_buf());
    let base = std::fs::canonicalize(base).unwrap_or_else(|_| base.to_path_buf());
    let joined = if Path::new(token).is_absolute() {
        PathBuf::from(token)
    } else {
        base.join(token)
    };
    let folded = fold_lexically(&joined)?;
    let placed = match (folded.parent(), folded.file_name()) {
        (Some(parent), Some(leaf)) => real_dir(parent).join(leaf),
        // No file name at all (the token is the root itself) — nothing to keep literal.
        _ => real_dir(&folded),
    };
    let relative = placed
        .strip_prefix(&root)
        .or_else(|_| placed.strip_prefix(repo_root))
        .ok()?;
    Some(relative.to_string_lossy().into_owned())
}

/// Fold `.` and `..` out of an absolute path **lexically** — `None` when `..` walks off the
/// front of the filesystem. No filesystem access: this is the placement step, and the
/// components that exist are canonicalized by [`real_dir`] afterwards.
fn fold_lexically(path: &Path) -> Option<PathBuf> {
    use std::path::Component;

    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    return None;
                }
            }
            Component::Normal(part) => out.push(part),
            other => out.push(other.as_os_str()),
        }
    }
    Some(out)
}

/// `dir` with its **existing** prefix canonicalized and the rest re-appended — so a directory
/// chain that does not exist yet is still placed against the real filesystem rather than
/// compared raw.
fn real_dir(dir: &Path) -> PathBuf {
    if let Ok(real) = std::fs::canonicalize(dir) {
        return real;
    }
    let mut tail: Vec<&std::ffi::OsStr> = Vec::new();
    let mut probe = dir;
    while let Some(parent) = probe.parent() {
        tail.push(probe.file_name().unwrap_or(probe.as_os_str()));
        if let Ok(real) = std::fs::canonicalize(parent) {
            let mut out = real;
            for part in tail.iter().rev() {
                out.push(part);
            }
            return out;
        }
        probe = parent;
    }
    dir.to_path_buf()
}

/// Resolve `relative` against the canonicalized `root`: the real path when it exists
/// (symlinks followed), else the lexical join with `.`/`..` folded out — so a
/// destination that does not exist yet is still placed. `None` when `..` walks off the
/// front of the path entirely.
fn resolve(root: &Path, relative: &str) -> Option<PathBuf> {
    let joined = root.join(relative);
    if let Ok(real) = std::fs::canonicalize(&joined) {
        return Some(real);
    }
    let mut out = root.to_path_buf();
    for component in Path::new(relative).components() {
        use std::path::Component;
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    return None;
                }
            }
            Component::Normal(part) => out.push(part),
            // An absolute component restarts the path — `join` already honoured it.
            Component::RootDir | Component::Prefix(_) => out = joined.clone(),
        }
    }
    Some(out)
}

/// Whether the index holds a **gitlink** (mode `160000`) at any ancestor directory of
/// `relative` — i.e. whether the path lies inside a submodule as far as *this* index is
/// concerned, checked out or not.
///
/// Asks about the ancestors rather than the path itself, because a pathspec *inside* a
/// submodule matches nothing (that is the whole problem). The gitlink's own reported path
/// is then checked to be a proper ancestor: a sibling submodule under a shared parent
/// (`my-hooks/vendored` beside `my-hooks/pre-commit`) matches the ancestor pathspec but
/// does not contain the path, and refusing on it would drop a perfectly committable one.
/// `-z` so paths arrive unquoted whatever `core.quotePath` says. Conservative on failure:
/// if git cannot be asked, the path keeps whatever the other tests granted it.
pub(crate) fn index_gitlink_covers(repo_root: &Path, relative: &str) -> bool {
    let mut ancestors: Vec<String> = Vec::new();
    let mut prefix = String::new();
    // Every proper ancestor DIRECTORY of the path (its own component dropped).
    let mut components: Vec<&str> = relative.split('/').collect();
    components.pop();
    for component in components {
        if !prefix.is_empty() {
            prefix.push('/');
        }
        prefix.push_str(component);
        ancestors.push(prefix.clone());
    }
    if ancestors.is_empty() {
        return false;
    }

    let mut args: Vec<String> = vec![
        "ls-files".into(),
        "--stage".into(),
        "-z".into(),
        "--".into(),
    ];
    args.extend(ancestors);
    let Some(out) = git_output(repo_root, args) else {
        return false;
    };
    if !out.status.success() {
        return false;
    }
    String::from_utf8_lossy(&out.stdout)
        .split('\0')
        .filter_map(|entry| entry.strip_prefix("160000 "))
        .filter_map(|entry| entry.split_once('\t'))
        .any(|(_, path)| relative.starts_with(&format!("{path}/")))
}

/// Run `git <args>` in `dir`, handing back the captured output — `None` when git could
/// not be spawned at all. The predicate form: a git that cannot answer abstains.
fn git_output<I, S>(dir: &Path, args: I) -> Option<std::process::Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway git repo that removes itself on drop (the project's no-tempfile pattern).
    struct TempRepo(PathBuf);

    impl TempRepo {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-trackable-{tag}-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create temp repo");
            let repo = TempRepo(path);
            repo.git(&["init", "-q"]);
            repo.git(&["config", "user.email", "t@t"]);
            repo.git(&["config", "user.name", "t"]);
            repo
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn git(&self, args: &[&str]) {
            let out = std::process::Command::new("git")
                .arg("-C")
                .arg(&self.0)
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
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// **The refusing half, over the two shapes git cannot record**, each at more than one
    /// depth and whether or not it exists on disk — the destination of a move need not
    /// exist yet, which is exactly why the answer cannot be read off the filesystem.
    #[test]
    fn every_shape_git_cannot_record_is_named_as_such() {
        let repo = TempRepo::new("refuse");
        std::fs::create_dir_all(repo.path().join(".git/hooks")).expect("hooks dir");

        for relative in [
            ".git",
            ".git/roadmap.md",
            ".git/hooks/roadmap.md",
            ".git/jigc-docs/decisions/one.md",
            ".GIT/roadmap.md",
            "docs/.git/roadmap.md",
            "../escaped.md",
            "../../escaped.md",
            "docs/../../escaped.md",
        ] {
            assert!(
                untrackable_reason(repo.path(), relative).is_some(),
                "`{relative}` is a destination git cannot record — it must be refused",
            );
        }
    }

    /// **The admitting half**, and the bound the fix is scoped by: everything git *can*
    /// record is admitted, including a **gitignored** path. Ignoring is a policy about a
    /// path git can track and has been told to skip — `git mv docs/x.md .jigc/x.md` stages
    /// a real `R` rename — so refusing it would have broken the workbench relocation while
    /// closing nothing.
    #[test]
    fn an_ordinary_or_merely_gitignored_destination_is_admitted() {
        let repo = TempRepo::new("admit");
        std::fs::write(repo.path().join(".gitignore"), ".jigc/\n").expect("write gitignore");
        std::fs::create_dir_all(repo.path().join(".jigc")).expect("workbench dir");

        for relative in [
            "",
            ".",
            "docs/roadmap.md",
            "notes/roadmap.md",
            "ROADMAP.md",
            ".jigc/roadmap.md",
            ".jigc/displaced/roadmap.md",
            ".github/workflows/ci.yml",
            "docs/./decisions/one.md",
            "deep/nested/never/created/one.md",
        ] {
            assert_eq!(
                untrackable_reason(repo.path(), relative),
                None,
                "`{relative}` is a path git can record — refusing it would break a \
                 legitimate move",
            );
        }
    }

    /// **No refusal names the host filesystem** (the M50 completion audit, finding 3).
    ///
    /// Three of the five reasons composed an absolute path — the repo root, git's own dir,
    /// and the owning repository's toplevel — and this is the *shared* predicate four doors
    /// ask (`config set placement-root`, `jigc rename`, `jigc setup`, `jigc relocate`). Law 1
    /// is stated universally (`design/surface-contract.md`: *every printed path is repo-real
    /// or a typed identity*), so the rule belongs to the predicate, not to whichever caller
    /// last got a bug report. Each reason is reached through the shape that is the ONLY way
    /// to reach it — the `.git`-component reason answers first for every ordinary layout, so
    /// the git-dir reason needs a `--separate-git-dir` inside the tree.
    #[test]
    fn no_refusal_names_the_host_path_of_the_machine_it_ran_on() {
        let repo = TempRepo::new("host-path");
        // The owning-repository reason.
        let inner = repo.path().join("vendored");
        std::fs::create_dir_all(&inner).expect("inner dir");
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(&inner)
            .args(["init", "-q"])
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .output()
            .expect("run git init");
        assert!(out.status.success(), "the embedded repo initializes");
        // The git-dir reason: a git dir inside the tree that is not called `.git`.
        repo.git(&["init", "-q", "--separate-git-dir=gitstore"]);

        // Both spellings of the root, because a canonicalizing filesystem (macOS
        // `/var` → `/private/var`) can put either one on the surface.
        let mut prefixes = vec![repo.path().to_string_lossy().into_owned()];
        if let Ok(real) = repo.path().canonicalize() {
            let real = real.to_string_lossy().into_owned();
            if !prefixes.contains(&real) {
                prefixes.push(real);
            }
        }

        for relative in [
            "../escaped.md",
            "gitstore/roadmap.md",
            "vendored/roadmap.md",
        ] {
            let reason = untrackable_reason(repo.path(), relative).unwrap_or_else(|| {
                panic!("`{relative}` must be refused — the cell reached no reason to check")
            });
            for prefix in &prefixes {
                assert!(
                    !reason.contains(prefix.as_str()),
                    "the refusal for `{relative}` prints the host path `{prefix}`, which is \
                     neither repo-real nor a typed identity: {reason}",
                );
            }
        }
    }

    /// **Location is not trackability.** A directory under the root can belong to another
    /// repository, and a path inside one is not this index's to take — `git add` refuses it
    /// fatally (a submodule) or stages nothing at exit 0 (an embedded repo). This is the
    /// ownership half the M48 install-commit rule already carried; it comes along because
    /// the rule is asked here rather than restated.
    #[test]
    fn a_path_inside_an_embedded_repository_is_not_this_index_s_to_take() {
        let repo = TempRepo::new("embedded");
        let inner = repo.path().join("vendored");
        std::fs::create_dir_all(&inner).expect("inner dir");
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(&inner)
            .args(["init", "-q"])
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .output()
            .expect("run git init");
        assert!(out.status.success(), "the embedded repo initializes");

        assert!(
            untrackable_reason(repo.path(), "vendored/roadmap.md").is_some(),
            "a destination inside an embedded repository is refused",
        );
        assert_eq!(
            untrackable_reason(repo.path(), "roadmap.md"),
            None,
            "…and its sibling outside is unaffected",
        );
    }
}
