//! **jigc writes through no link on the way** — the rule's one home in code
//! ([storage.md](../../../design/storage.md) → *No write through a link on the way*;
//! `DECISIONS.md` → 2026-10-28, the human's ruling B).
//!
//! A directory of the repository that is a **symbolic link** carries every write below it
//! wherever it leads — out of the repository, or to another place inside it — and git
//! follows none of it: what jigc wrote there is in no commit, and no command of jigc's reads
//! it back as the repository's. Driven on `0bd8a5b5`, nine cells of the byte table
//! (`r1-a-write-follows-a-link-at-a-directory-on-the-way-out-of-the-repository`) wrote,
//! moved or rebuilt through such a link at five doors, six of them at exit 0, and eight more
//! wrote through it and rolled back when `git add` refused — and git 2.56 refuses a `git mv`
//! past a link where git 2.55 and older follow it, so two of the nine came out one way on
//! macOS and another on CI. The rule is now jigc's, asked before git is asked: **every
//! command asks the directories on the way to what it writes, moves, or reads in order to
//! write whether one is a link, and refuses before its first write**, naming the link and
//! routing at a way out that leaves what the link led to in the repository, as a real
//! directory — what `jigc setup` already did for its install, now for every door and under
//! one code.
//!
//! **Where it is asked — two places every door inherits, and the doors that cannot.**
//!
//! - **At dispatch, before any door runs** ([`crate::cli`]'s `refuse_on_link_on_the_way`),
//!   over the store's **write surface** ([`store_links`]): the workbench — `.jigc`, every
//!   directory jigc keeps in it, and every link in it that leads to a directory
//!   ([`in_the_workbench`]) — and every directory on the way to every doctype's resolved
//!   home ([`on_the_way_to_homes`]). Every command that writes the repository writes the
//!   workbench, and reads or writes a home. A **read form** — a `VerbKind::Read` leaf, and
//!   the read forms of `start` and `workflow` — reports the links at exit 0 instead
//!   ([`advisory`]); `jigc uninstall` asks its own members' way; every other leaf is
//!   refused, a leaf added tomorrow included.
//! - **At the move primitive** ([`crate::relocate::move_doc`]) — every move of a doc, source
//!   and destination, for the homes the store does not have *yet*: a `config set` re-point's
//!   new homes and a `--from` prior home. The movers ask the same question of their whole
//!   batch before their first move ([`first_of`]), so nothing is moved before the refusal.
//! - **At the doors whose subject is not the store's homes**: `jigc config set`'s root knobs
//!   over the homes the new value resolves, `jigc migrate-corpus` over the prior homes its
//!   versioned snapshots declare, `jigc setup` and `jigc uninstall` over their own install
//!   members.
//!
//! **A link counts whatever it points at** — a directory outside the repository, another
//! inside it, or nothing: the check follows no link (the rows' record's §6, entry 25, ruled
//! 2026-10-26). In the workbench, which holds files' places and directories' alike, a link
//! counts where jigc keeps a directory ([`WORKBENCH_DIRECTORIES`]) and wherever it leads to
//! one; a link in a file's place is that file's own question. A link at a doc's **own**
//! home — the leaf — is not this rule's: it is `store.home-not-regular-file`
//! ([`engine::store::home_shape_refusal`]).
//!
//! **Why the reason holds inside the repository too.** Git records a link as the entry it
//! is, never a path past it — so what jigc wrote past a link is in no commit of the
//! repository whether the link leads out of it or to another directory inside it.

use std::path::{Path, PathBuf};

use engine::finding::{Finding, Location, Route, Severity};
use engine::schema::Schema;

/// The code of the refusal: a directory on the way to where jigc writes is a symbolic link.
/// One code for the state at every door that meets it — one fault, one code (the M49
/// Increment 11 / T1 rule), as `store.home-not-regular-file` is for the leaf.
pub const LINK_ON_THE_WAY: &str = "store.link-on-the-way";

/// **The way out of one link**, read from the link's own text (`read_link`, which follows
/// nothing) — chosen so following it leaves one copy of what the link led to, in the
/// repository, where git records it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WayOut {
    /// The link leads to a real directory **inside** the repository, at this repo-relative
    /// path: move that directory into the link's place. A copy would leave two.
    MoveIn(String),
    /// The link leads to a directory outside the repository (or to one inside it only
    /// through another link): put a copy of what it leads to in the link's place, as a
    /// real directory. What stands outside is the user's and stays.
    CopyIn,
    /// The link leads to no directory — to nothing, or to a file: remove it; jigc makes the
    /// directory itself where it needs one.
    Remove,
}

/// **One link on the way** — the repo-relative path of the link, where it stands, and the
/// way out of it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    /// The link's path, relative to the checkout it stands in.
    pub at: String,
    /// Where it stands, as the clause that follows *is a symbolic link* —
    /// `` inside the workbench `.jigc` ``, `` on the way to the home of `adr` ``, or
    /// `` on the way to `<path>` ``.
    pub stands: String,
    /// What the route asks of it.
    pub way_out: WayOut,
}

impl Link {
    /// The link at `at` below `root`, standing as `stands` — its way out read from its text.
    #[must_use]
    pub fn at(root: &Path, at: &str, stands: &str) -> Self {
        Link {
            at: at.to_string(),
            stands: stands.to_string(),
            way_out: way_out(root, at),
        }
    }

    /// The route's clause for this link.
    fn clause(&self) -> String {
        match &self.way_out {
            WayOut::MoveIn(target) => format!(
                "remove the link at `{}` and move the directory `{target}` into its place",
                self.at
            ),
            WayOut::CopyIn => format!(
                "replace the link at `{}` with a real directory holding a copy of what it \
                 leads to",
                self.at
            ),
            WayOut::Remove => format!(
                "remove the link at `{}`, which leads to no directory — jigc makes one where \
                 it needs it",
                self.at
            ),
        }
    }
}

/// Read the link at `<root>/<at>` without following it and say what its way out is.
fn way_out(root: &Path, at: &str) -> WayOut {
    let link = root.join(at);
    let Ok(text) = std::fs::read_link(&link) else {
        return WayOut::Remove;
    };
    let target = if text.is_absolute() {
        text
    } else {
        link.parent().unwrap_or(root).join(text)
    };
    let target = engine::store::lexical_normalize(&target);
    let roots = [
        engine::store::lexical_normalize(root),
        root.canonicalize().unwrap_or_else(|_| root.to_path_buf()),
    ];
    let inside = roots
        .iter()
        .find_map(|root| target.strip_prefix(root).ok().map(Path::to_path_buf));
    let real_directory =
        std::fs::symlink_metadata(&target).is_ok_and(|entry| entry.file_type().is_dir());
    // A directory inside git's own directory is no directory of the repository's: git
    // records nothing there, so what it holds is copied in, never moved out of git's.
    let inside = inside.filter(|rel| {
        rel.components()
            .next()
            .is_some_and(|first| first.as_os_str() != ".git")
    });
    match inside {
        Some(rel) if real_directory && !rel.as_os_str().is_empty() => WayOut::MoveIn(
            rel.components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect::<Vec<_>>()
                .join("/"),
        ),
        _ if target.is_dir() => WayOut::CopyIn,
        _ => WayOut::Remove,
    }
}

/// The links, named once each by path, in order.
fn distinct(links: &[Link]) -> Vec<&Link> {
    let mut links: Vec<&Link> = links.iter().collect();
    links.sort();
    links.dedup_by(|a, b| a.at == b.at);
    links
}

/// The states, one sentence fragment a link.
fn states(links: &[&Link]) -> String {
    links
        .iter()
        .map(|link| format!("`{}` is a symbolic link {}", link.at, link.stands))
        .collect::<Vec<_>>()
        .join("; ")
}

/// The route: a clause a link, then the commit and the re-run.
fn route(links: &[&Link]) -> Route {
    let noun = if links.len() == 1 { "link" } else { "links" };
    Route::human(format!(
        "{} — committing the change where git tracks the {noun} — then re-run this command",
        links
            .iter()
            .map(|link| link.clause())
            .collect::<Vec<_>>()
            .join("; "),
    ))
}

/// **The reason**, true of a link inside the repository as of one outside it.
const REASON: &str = "git records a link as the entry it is and never a path past it, so \
                      whatever jigc wrote, moved or rebuilt past one would be in no commit of \
                      this repository";

/// **The refusal** — every link found, named once each, sorted, in one blocking finding.
///
/// No locus: the state is the repository's, not one doc's, as the posture family's
/// refusals carry none. The route is [`Route::human`], because no `jigc` argv replaces a
/// link, and it names a link's in-repository target by its repo-relative path and an
/// outside one never: that is a host path, and a printed path is repo-real (law 1).
#[must_use]
pub fn finding(links: &[Link]) -> Finding {
    let links = distinct(links);
    Finding::block(
        LINK_ON_THE_WAY,
        format!(
            "{} — {REASON}: jigc writes nothing past a link. Nothing was written",
            states(&links)
        ),
        route(&links),
    )
}

/// **The report a read form makes** — the same state, code and route as [`finding`], at
/// advisory severity: the command only reads, wrote nothing, and runs; the commands that
/// write refuse here until the links are real directories.
#[must_use]
pub fn advisory(links: &[Link]) -> Finding {
    let links = distinct(links);
    Finding::graded(
        Severity::Advisory,
        LINK_ON_THE_WAY,
        format!(
            "{} — {REASON}: every command that writes refuses here until it is a real \
             directory. This one only reads, and wrote nothing",
            states(&links)
        ),
        None::<Location>,
        Some(route(&links)),
    )
}

/// [`finding`], as the error a door returns on — the flattening carrier every refusal of
/// the doors that meet it already rides ([`crate::render::finding_error`]).
#[must_use]
pub fn refusal(links: &[Link]) -> anyhow::Error {
    crate::render::finding_error(&finding(links))
}

/// The link on the way to the file or directory `relative` below `root`, the leaf not
/// asked ([`crate::regular_file::link_on_the_way`]), as a [`Link`] past that path.
#[must_use]
pub fn on_the_way_to(root: &Path, relative: &str) -> Option<Link> {
    let blocker = crate::regular_file::link_on_the_way(root, relative)?;
    let stands = format!("on the way to `{}`", blocker.aimed_at);
    Some(Link::at(root, &blocker.at, &stands))
}

/// The link at the directory `dir` below `root` **or on the way to it** — a directory jigc
/// writes in, itself included — named by the clause `stands`.
pub(crate) fn at_or_on_the_way_to_dir(root: &Path, dir: &str, stands: &str) -> Option<Link> {
    let dir = dir.trim_matches('/');
    if dir.is_empty() || dir == "." {
        return None;
    }
    // The directory is asked as the way to a name inside it, so the walk asks it too.
    let blocker = crate::regular_file::link_on_the_way(root, &format!("{dir}/-"))?;
    Some(Link::at(root, &blocker.at, stands))
}

/// The first link on the way to any of `paths` below `root` — the batch a mover asks
/// before its first move, so a link stops the whole batch with nothing moved.
#[must_use]
pub fn first_of<'a>(root: &Path, paths: impl IntoIterator<Item = &'a str>) -> Option<Link> {
    paths
        .into_iter()
        .filter_map(|path| on_the_way_to(root, path))
        .min()
}

/// **Every place below `.jigc` where jigc keeps a directory** — the places a link counts at
/// whatever it leads to (`*` one name). A link elsewhere in the workbench stands in a
/// file's place, or in a place jigc does not write, and counts only where it leads to a
/// directory.
pub const WORKBENCH_DIRECTORIES: &[&str] = &[
    "config",
    "config/steps",
    "config/fills",
    "config/schemas",
    "config/workflows",
    "index",
    "state",
    "tasks",
    "tasks/*",
    "tasks/*/docs",
    "milestones",
    "milestones/*",
    "milestones/*/merged",
    "milestones/*/merged/docs",
    "displaced",
    "worktrees",
    "worktrees/*",
    "logs",
];

/// The parking home: jigc writes into it only at a name that is free, so what it holds —
/// what a door parked there — is on the way to nothing jigc writes.
const DISPLACED: &str = "displaced";

/// The fan-out worktrees' directory below the workbench — each entry there is a git
/// checkout jigc provisioned, whose own tree is the code's and not the workbench's.
const WORKTREES: &str = "worktrees";

/// Whether `rel` (below `.jigc`) is one of [`WORKBENCH_DIRECTORIES`].
fn keeps_a_directory(rel: &str) -> bool {
    let parts: Vec<&str> = rel.split('/').collect();
    WORKBENCH_DIRECTORIES.iter().any(|place| {
        let pattern: Vec<&str> = place.split('/').collect();
        pattern.len() == parts.len()
            && pattern
                .iter()
                .zip(&parts)
                .all(|(want, got)| *want == "*" || want == got)
    })
}

/// **Every link in the workbench** — `.jigc` itself; a link where jigc keeps a directory,
/// whatever it leads to; and a link anywhere else that leads to a directory — read without
/// following a link.
///
/// It does not descend into a fan-out worktree (`.jigc/worktrees/<id>/`): that is a
/// checkout of the code, where a link is the code's own business. Nor into the parking
/// home (`.jigc/displaced/`): what is parked there is on the way to nothing jigc writes —
/// the entry `displaced` itself is asked.
#[must_use]
pub fn in_the_workbench(jigc_home: &Path) -> Vec<Link> {
    let workbench = jigc_home.join(".jigc");
    let Ok(entry) = std::fs::symlink_metadata(&workbench) else {
        return Vec::new();
    };
    if entry.file_type().is_symlink() {
        return vec![Link::at(
            jigc_home,
            ".jigc",
            "where jigc keeps its workbench",
        )];
    }
    let stands = "inside the workbench `.jigc`";
    let mut found = Vec::new();
    let mut pending: Vec<(PathBuf, String)> = vec![(workbench, String::new())];
    while let Some((dir, below)) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            let name = entry.file_name().to_string_lossy().into_owned();
            let rel = if below.is_empty() {
                name
            } else {
                format!("{below}/{name}")
            };
            if kind.is_symlink() {
                if keeps_a_directory(&rel) || entry.path().is_dir() {
                    found.push(Link::at(jigc_home, &format!(".jigc/{rel}"), stands));
                }
            } else if kind.is_dir()
                && rel != DISPLACED
                && !rel.starts_with(&format!("{WORKTREES}/"))
            {
                pending.push((entry.path(), rel));
            }
        }
    }
    found.sort();
    found
}

/// **Every link at a doctype's home directory or on the way to one** — the resolved
/// `location:` directory, or the directories on the way to a `placement:` file, below
/// `root`. A transient doctype has no home and names nothing.
#[must_use]
pub fn on_the_way_to_homes<'a>(
    root: &Path,
    schemas: impl IntoIterator<Item = &'a Schema>,
) -> Vec<Link> {
    let mut found: Vec<Link> = Vec::new();
    for schema in schemas {
        let stands = format!("on the way to the home of `{}`", schema.ty);
        let link = match crate::orphan::home_of(schema) {
            Some(crate::orphan::Home::Location(dir)) => {
                at_or_on_the_way_to_dir(root, &dir, &stands)
            }
            Some(crate::orphan::Home::Placement(file)) => {
                crate::regular_file::link_on_the_way(root, &file)
                    .map(|blocker| Link::at(root, &blocker.at, &stands))
            }
            None => None,
        };
        if let Some(link) = link
            && !found.iter().any(|seen| seen.at == link.at)
        {
            found.push(link);
        }
    }
    found.sort();
    found
}

/// Which homes [`store_links`] asks about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Homes {
    /// The homes the project layer resolves; none where it cannot be read — the door then
    /// answers that state itself, under its own code.
    Current,
    /// The same, and the pack's defaults where no project layer stands yet — `jigc setup`,
    /// which is about to lay one out over those homes.
    CurrentOrDefault,
    /// None: the door asks the homes its own act reaches (`jigc config set`).
    None,
}

/// The doctypes' resolved schemas at `jigc_home`, or `None` where they cannot be read.
fn schemas_at(jigc_home: &Path, homes: Homes) -> Option<Vec<Schema>> {
    let project_config = jigc_home.join(".jigc").join("config");
    let laid_out = project_config.is_dir();
    if homes == Homes::None || (!laid_out && homes == Homes::Current) {
        return None;
    }
    // The ungated assembly: the door that runs next asserts the pack-set itself.
    let pack = crate::pack::make_pack_ungated().ok()?;
    crate::start::resolved_schemas(pack.as_ref(), &project_config)
        .ok()
        .map(|schemas| schemas.into_values().collect())
}

/// **The store's write surface, asked from `cwd`** — every link in the workbench of the
/// repository `cwd` is in, and on the way to every doctype's resolved home ([`Homes`]) in
/// the checkout `cwd` stands in and in the main one, **all of them in one answer**. A link
/// is spelled relative to the checkout it stands in, and a main-checkout link is said to be
/// one where the reader stands in another. Empty outside a git repository.
#[must_use]
pub fn store_links(cwd: &Path, homes: Homes) -> Vec<Link> {
    let Some(jigc_home) = crate::repo::jigc_home(cwd) else {
        return Vec::new();
    };
    let standing = crate::repo::discover_repo_root(cwd).unwrap_or_else(|| jigc_home.clone());
    let elsewhere = standing != jigc_home;
    let in_main = |mut link: Link| {
        if elsewhere {
            link.stands.push_str(", in the main checkout");
        }
        link
    };
    let mut found: Vec<Link> = in_the_workbench(&jigc_home)
        .into_iter()
        .map(in_main)
        .collect();
    if let Some(schemas) = schemas_at(&jigc_home, homes) {
        found.extend(
            on_the_way_to_homes(&jigc_home, &schemas)
                .into_iter()
                .map(in_main),
        );
        if elsewhere {
            found.extend(on_the_way_to_homes(&standing, &schemas));
        }
    }
    found.sort();
    found.dedup_by(|a, b| a.at == b.at && a.stands == b.stands);
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-link-way-unit-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn named(links: Vec<Link>) -> Vec<String> {
        links.into_iter().map(|l| l.at).collect()
    }

    /// A linked workbench is named once, as itself; a link that leads to a directory is
    /// named anywhere below it; a link to nothing or to a file is named where jigc keeps a
    /// directory and nowhere else; a fan-out worktree's own entry is asked and its checkout
    /// is not; the parking home's entry is asked and what is parked in it is not.
    #[test]
    fn the_workbench_walk_names_every_link_on_the_way_and_nothing_parked() {
        let repo = TempDir::new();
        let outside = TempDir::new();
        let root = &repo.0;
        std::fs::create_dir_all(root.join(".jigc/tasks/t1")).expect("mkdir");
        std::fs::create_dir_all(root.join(".jigc/worktrees/w1/src")).expect("mkdir");
        std::fs::create_dir_all(root.join(".jigc/displaced")).expect("mkdir");
        assert_eq!(
            in_the_workbench(root),
            Vec::new(),
            "a plain workbench has none"
        );

        symlink(&outside.0, root.join(".jigc/tasks/t1/docs")).expect("link");
        symlink(&outside.0, root.join(".jigc/worktrees/w1/src/vendored")).expect("link");
        symlink(&outside.0, root.join(".jigc/worktrees/w2")).expect("link");
        symlink(&outside.0, root.join(".jigc/displaced/parked")).expect("link");
        symlink("nowhere", root.join(".jigc/index")).expect("link");
        symlink("nowhere", root.join(".jigc/tasks/t1/task.json")).expect("link");
        std::fs::write(outside.0.join("f"), "x").expect("write");
        symlink(outside.0.join("f"), root.join(".jigc/tasks/t1/a-file-link")).expect("link");
        assert_eq!(
            named(in_the_workbench(root)),
            [".jigc/index", ".jigc/tasks/t1/docs", ".jigc/worktrees/w2"],
        );

        let linked = TempDir::new();
        symlink("nowhere", linked.0.join(".jigc")).expect("link");
        assert_eq!(named(in_the_workbench(&linked.0)), [".jigc"]);
    }

    /// The way out is read from the link's text: a real directory inside the repository is
    /// moved into the link's place; a directory outside is copied in; a link to nothing is
    /// removed.
    #[test]
    fn the_way_out_follows_what_the_link_leads_to() {
        let repo = TempDir::new();
        let outside = TempDir::new();
        let root = &repo.0;
        std::fs::create_dir_all(root.join("kept")).expect("mkdir");
        std::fs::create_dir_all(root.join("docs")).expect("mkdir");
        symlink("../kept", root.join("docs/inside")).expect("link");
        symlink(&outside.0, root.join("docs/outside")).expect("link");
        symlink("../nowhere", root.join("docs/nothing")).expect("link");
        assert_eq!(
            way_out(root, "docs/inside"),
            WayOut::MoveIn("kept".to_string())
        );
        assert_eq!(way_out(root, "docs/outside"), WayOut::CopyIn);
        assert_eq!(way_out(root, "docs/nothing"), WayOut::Remove);
    }

    /// The finding names every link once under the rule's code, says what is true of a
    /// link inside the repository too, and routes each link its own way; the advisory is
    /// the same state at advisory severity.
    #[test]
    fn the_finding_names_each_link_and_routes_each_its_way() {
        let link = |at: &str, way_out: WayOut| Link {
            at: at.to_string(),
            stands: format!("on the way to `{at}/x`"),
            way_out,
        };
        let one = finding(&[link("docs/decisions", WayOut::MoveIn("kept".to_string()))]);
        assert_eq!(one.code, LINK_ON_THE_WAY);
        assert!(
            one.message
                .starts_with("`docs/decisions` is a symbolic link on the way")
        );
        assert!(
            one.message.contains("never a path past it"),
            "{}",
            one.message
        );
        assert!(!one.message.contains("where no commit"), "{}", one.message);
        let route = one.route.as_ref().expect("routed").to_string();
        assert!(
            route.contains(
                "remove the link at `docs/decisions` and move the directory `kept` into its place"
            ),
            "{route}"
        );
        let two = finding(&[
            link("b", WayOut::Remove),
            link("a", WayOut::CopyIn),
            link("a", WayOut::CopyIn),
        ]);
        assert!(
            two.message
                .starts_with("`a` is a symbolic link on the way to `a/x`; `b`")
        );
        let route = two.route.as_ref().expect("routed").to_string();
        assert!(
            route.contains("replace the link at `a` with a real directory holding a copy"),
            "{route}"
        );
        assert!(
            route.contains("; remove the link at `b`, which leads to no directory"),
            "{route}"
        );
        assert!(route.contains("where git tracks the links"), "{route}");
        let report = advisory(&[link("a", WayOut::CopyIn)]);
        assert_eq!(
            (report.code.as_str(), report.severity),
            (LINK_ON_THE_WAY, Severity::Advisory)
        );
    }
}
