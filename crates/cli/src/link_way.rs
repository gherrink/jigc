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
//! routing at replacing it with a real directory — what `jigc setup` already did for its
//! install ([`crate::setup`], `setup.write-bootstrap` / `setup.write-guide`), now for every
//! door.
//!
//! **Where it is asked — two places every door inherits, and one per door that cannot.**
//!
//! - **At dispatch, before any door runs** ([`crate::cli`]'s `refuse_on_link_on_the_way`),
//!   over the store's **write surface**: the workbench — `.jigc` and every directory below
//!   it, but the inside of a fan-out worktree's checkout ([`in_the_workbench`]) — and every
//!   directory on the way to every doctype's resolved home ([`on_the_way_to_homes`]). Every
//!   command that writes the repository writes the workbench, and reads or writes a home: a
//!   finalize rebuilds the edge index from every home, a `doc` write copies a committed doc
//!   in from its home, a milestone record door writes the record's. So a door added
//!   tomorrow is guarded the moment it is a leaf of the clap tree, unless it is named an
//!   exception with its reason (`LINK_GUARD_EXCEPTIONS`).
//! - **At the move primitive** ([`crate::relocate::move_doc`]) — every move of a doc, source
//!   and destination, for the homes the store does not have *yet*: a `config set` re-point's
//!   new homes and a `--from` prior home. The movers ask the same question of their whole
//!   batch before their first move ([`first_of`]), so nothing is moved before the refusal.
//! - **At `migrate-corpus`**, over the prior homes its versioned snapshots declare — homes
//!   only that command knows of.
//!
//! **On the way to a path, a link counts whatever it points at** — a directory outside the
//! repository, another inside it, or nothing: the check follows no link (the rows' record's
//! §6, entry 25, ruled 2026-10-26), so what a write past it would reach is never jigc's to
//! guess. The workbench walk, which meets files' places and directories' alike, counts a
//! link where it leads to a directory ([`in_the_workbench`]). A link at a doc's **own**
//! home — the leaf — is not this rule's: it is `store.home-not-regular-file`
//! ([`engine::store::home_shape_refusal`]), a door's question about one path.

use std::path::{Path, PathBuf};

use engine::finding::{Finding, Route};
use engine::schema::Schema;

/// The code of the refusal: a directory on the way to where jigc writes is a symbolic link.
/// One code for the state at every door that meets it — one fault, one code (the M49
/// Increment 11 / T1 rule), as `store.home-not-regular-file` is for the leaf.
pub const LINK_ON_THE_WAY: &str = "store.link-on-the-way";

/// **One link on the way** — the repo-relative path of the link, and where it stands, as
/// the clause the message names it by.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    /// The link's repo-relative path.
    pub at: String,
    /// Where it stands, as the clause that follows *is a symbolic link* —
    /// `` inside the workbench `.jigc` ``, `` on the way to the home of `adr` ``, or
    /// `` on the way to `<path>` ``.
    pub stands: String,
}

/// **The refusal** — every link found, named once each, sorted, in one blocking finding.
///
/// No locus: the state is the repository's, not one doc's, and the posture family's
/// refusals — the other state a door refuses on before its first write — carry none
/// either. The route is [`Route::human`], because no `jigc` argv replaces a link, and it
/// names the act and the command to re-run, never the link's target: that is a host path,
/// and a printed path is repo-real (law 1).
#[must_use]
pub fn finding(links: &[Link]) -> Finding {
    let mut links: Vec<&Link> = links.iter().collect();
    links.sort();
    links.dedup_by(|a, b| a.at == b.at);
    let states: Vec<String> = links
        .iter()
        .map(|link| format!("`{}` is a symbolic link {}", link.at, link.stands))
        .collect();
    let named: Vec<String> = links.iter().map(|link| format!("`{}`", link.at)).collect();
    let (noun, real, it) = if links.len() == 1 {
        ("link", "a real directory", "it")
    } else {
        ("links", "real directories", "they")
    };
    Finding::block(
        LINK_ON_THE_WAY,
        format!(
            "{} — jigc writes, moves and rebuilds only inside this repository and never past \
             a link, because past one a write lands wherever the link leads, where no commit \
             of this repository holds it. Nothing was written",
            states.join("; "),
        ),
        Route::human(format!(
            "replace the {noun} at {} with {real} holding what {it} led to — committing the \
             change where git tracks the {noun} — then re-run this command",
            named.join(", "),
        )),
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
    Some(Link {
        at: blocker.at,
        stands: format!("on the way to `{}`", blocker.aimed_at),
    })
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
    Some(Link {
        at: blocker.at,
        stands: stands.to_string(),
    })
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

/// The fan-out worktrees' directory below the workbench — each entry there is a git
/// checkout jigc provisioned, whose own tree is the code's and not the workbench's.
const WORKTREES: &str = "worktrees";

/// **Every link in the workbench** — `.jigc` itself, or any directory below it, read
/// without following a link. jigc makes no link anywhere in it, so every one is a third
/// party's, and every one is on the way to something jigc writes: the task and milestone
/// areas, the caches, the config layer, the parking home.
///
/// It does not descend into a fan-out worktree (`.jigc/worktrees/<id>/`): that is a
/// checkout of the code, where a link is the code's own business. The worktree's own entry
/// is asked — a link standing where jigc provisioned a checkout carries the sub-task's
/// writes elsewhere. A link that does not lead to a directory — to a **file**, or to
/// nothing — is not this walk's: it stands where jigc keeps a file, where what stands is
/// the door's own question about that path (the stamp, the manifest, a provisioned doc —
/// each refused or replaced under its own code), and a write that meets one in a
/// directory's place makes no directory through it but fails on it.
#[must_use]
pub fn in_the_workbench(jigc_home: &Path) -> Vec<Link> {
    let workbench = jigc_home.join(".jigc");
    let Ok(entry) = std::fs::symlink_metadata(&workbench) else {
        return Vec::new();
    };
    if entry.file_type().is_symlink() {
        return vec![Link {
            at: ".jigc".to_string(),
            stands: "where jigc keeps its workbench".to_string(),
        }];
    }
    let past = "inside the workbench `.jigc`";
    let mut found = Vec::new();
    let mut pending: Vec<(PathBuf, String)> = vec![(workbench, ".jigc".to_string())];
    while let Some((dir, spelled)) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        let in_worktrees = spelled == format!(".jigc/{WORKTREES}");
        for entry in entries.flatten() {
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            let name = entry.file_name().to_string_lossy().into_owned();
            let child = format!("{spelled}/{name}");
            if kind.is_symlink() {
                if entry.path().is_dir() {
                    found.push(Link {
                        at: child,
                        stands: past.to_string(),
                    });
                }
            } else if kind.is_dir() && !in_worktrees {
                pending.push((entry.path(), child));
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
        let past = format!("on the way to the home of `{}`", schema.ty);
        let link = match crate::orphan::home_of(schema) {
            Some(crate::orphan::Home::Location(dir)) => at_or_on_the_way_to_dir(root, &dir, &past),
            Some(crate::orphan::Home::Placement(file)) => {
                crate::regular_file::link_on_the_way(root, &file).map(|blocker| Link {
                    at: blocker.at,
                    stands: past.clone(),
                })
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

/// **The store's write surface, asked from `cwd`** — the links in the workbench of the
/// repository `cwd` is in and, where `homes`, on the way to every doctype's resolved home
/// in that checkout and in the main one. Empty outside a git repository, and — for the
/// homes — wherever the project layer or the cascade cannot be read: the door that is
/// about to run reports that state itself, under its own code, and this guard does not
/// pre-empt it with a different one.
#[must_use]
pub fn store_links(cwd: &Path, homes: bool) -> Vec<Link> {
    let Some(jigc_home) = crate::repo::jigc_home(cwd) else {
        return Vec::new();
    };
    let mut found = in_the_workbench(&jigc_home);
    if !found.is_empty() || !homes {
        return found;
    }
    let project_config = jigc_home.join(".jigc").join("config");
    if !project_config.is_dir() {
        return found;
    }
    let Ok(pack) = crate::pack::make_pack() else {
        return found;
    };
    let Ok(schemas) = crate::start::resolved_schemas(pack.as_ref(), &project_config) else {
        return found;
    };
    let mut roots = vec![jigc_home.clone()];
    if let Some(standing) = crate::repo::discover_repo_root(cwd)
        && standing != jigc_home
    {
        roots.push(standing);
    }
    for root in roots {
        for link in on_the_way_to_homes(&root, schemas.values()) {
            let link = Link {
                at: engine::path::repo_relative(&jigc_home, &root.join(&link.at)),
                ..link
            };
            if !found.iter().any(|seen| seen.at == link.at) {
                found.push(link);
            }
        }
    }
    found.sort();
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

    /// A linked workbench is named once, as itself; a link anywhere below it is named at
    /// its own path; a fan-out worktree's own entry is asked and its checkout is not.
    #[test]
    fn the_workbench_walk_names_every_link_and_skips_a_worktrees_checkout() {
        let repo = TempDir::new();
        let outside = TempDir::new();
        let root = &repo.0;
        std::fs::create_dir_all(root.join(".jigc/tasks/t1")).expect("mkdir");
        std::fs::create_dir_all(root.join(".jigc/worktrees/w1/src")).expect("mkdir");
        assert_eq!(
            in_the_workbench(root),
            Vec::new(),
            "a plain workbench has none"
        );

        symlink(&outside.0, root.join(".jigc/tasks/t1/docs")).expect("link");
        symlink(&outside.0, root.join(".jigc/worktrees/w1/src/vendored")).expect("link");
        symlink(&outside.0, root.join(".jigc/worktrees/w2")).expect("link");
        std::fs::write(outside.0.join("f"), "x").expect("write");
        symlink(outside.0.join("f"), root.join(".jigc/tasks/t1/a-file-link")).expect("link");
        let named: Vec<String> = in_the_workbench(root).into_iter().map(|l| l.at).collect();
        assert_eq!(
            named,
            [".jigc/tasks/t1/docs", ".jigc/worktrees/w2"],
            "a link to a directory is named; a worktree's checkout is the code's; a link to a \
             file is no directory on the way"
        );

        let linked = TempDir::new();
        symlink(&outside.0, linked.0.join(".jigc")).expect("link");
        assert_eq!(
            in_the_workbench(&linked.0),
            [Link {
                at: ".jigc".to_string(),
                stands: "where jigc keeps its workbench".to_string()
            }]
        );
    }

    /// The finding names every link once, its code is the rule's, and its route replaces
    /// the link with a real directory and re-runs.
    #[test]
    fn the_finding_names_each_link_and_routes_at_a_real_directory() {
        let one = finding(&[Link {
            at: "docs/decisions".to_string(),
            stands: "on the way to the home of `adr`".to_string(),
        }]);
        assert_eq!(one.code, LINK_ON_THE_WAY);
        assert!(
            one.message.starts_with(
                "`docs/decisions` is a symbolic link on the way to the home of `adr` —"
            ),
            "{}",
            one.message
        );
        let route = one.route.as_ref().expect("routed").to_string();
        assert!(
            route.contains("replace the link at `docs/decisions` with a real directory"),
            "{route}"
        );
        let two = finding(&[
            Link {
                at: "b".to_string(),
                stands: "on the way to `b/x`".to_string(),
            },
            Link {
                at: "a".to_string(),
                stands: "on the way to `a/x`".to_string(),
            },
            Link {
                at: "a".to_string(),
                stands: "on the way to `a/y`".to_string(),
            },
        ]);
        assert!(
            two.message
                .starts_with("`a` is a symbolic link on the way to `a/x`; `b`")
        );
        let route = two.route.as_ref().expect("routed").to_string();
        assert!(
            route.contains("the links at `a`, `b` with real directories"),
            "{route}"
        );
    }
}
