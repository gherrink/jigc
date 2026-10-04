//! **Writing a file jigc replaces whole — as a regular file at exactly its path, never
//! through a link** (the rc.24 fix pass; `DECISIONS.md` → 2026-10-04, the symlink fork).
//!
//! A writer that *replaces* a file puts its own content where something else was, so the
//! one thing it must know is that the entry it opens is the entry it means. `std::fs::write`
//! does not ask: it opens its destination through whatever is there. Driven on
//! `1.0.0-rc.24`, a symlink at `.jigc/AGENT.md`, `.jigc/version`, `.jigc/config/.gitkeep` or
//! `.jigc/config/packs.yaml` had `jigc setup` regenerate the link's **target** — an
//! untracked file, a gitignored one, a file outside the repository — at exit 0 with no
//! finding, and `jigc task finalize` did the same through its stamp refresh. It is the rule
//! `crate::gitignore::ensure` already applied to `.jigc/.gitignore` (*reject a non-regular
//! or symlinked file rather than replace it*), which the four writers beside it never had.
//!
//! Two halves, because they answer at different moments:
//!
//! - [`blocker`] is the **question**, asked without following a link: is anything on this
//!   path's way, or at it, that a replacing write would go through or over? A door asks it
//!   before its first write, where a refusal can still be routed.
//! - [`replace`] is the **write**: it asks the same question again, then opens the leaf with
//!   `O_NOFOLLOW` and asks the *open handle* what it is before a byte moves
//!   ([`open_no_follow`], shared with the promote sink). So a link that appears after the
//!   door asked fails the open instead of naming another file.
//!
//! **The directories on the way count** — every component below the checkout root. The
//! `O_NOFOLLOW` open guards the last component only, and a symlinked `.jigc` or
//! `.jigc/config` carries every write below it into another directory just as a link at the
//! leaf does. Driven on this fix's parent: `.jigc -> ../store` had `../store/AGENT.md`
//! regenerated, after which the install failed at its own `git add` (*beyond a symbolic
//! link*) with a route nothing could satisfy. The root itself is the caller's and is never
//! asked about: a checkout reached through a link is still that checkout.

use std::io;
use std::path::Path;

pub use engine::store::ForeignEntry;
use engine::store::HomeEntry;

/// **What stands in a replacing write's way** — the first entry, walking `relative` down
/// from the root, that the write would go through or over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blocker {
    /// The repo-relative path of the offending entry: the leaf itself, or a directory on
    /// the way to it.
    pub at: String,
    /// What that entry is.
    pub shape: ForeignEntry,
    /// The repo-relative path the write was aimed at. Equal to [`Self::at`] when the entry
    /// is the leaf.
    pub aimed_at: String,
}

impl Blocker {
    /// Whether the offending entry is the leaf, rather than a directory on the way to it.
    #[must_use]
    pub fn at_leaf(&self) -> bool {
        self.at == self.aimed_at
    }

    /// The state, as a sentence fragment a message or an error can open with:
    /// `` `.jigc/AGENT.md` is a symbolic link ``, or, for a directory on the way,
    /// `` `.jigc` is a symbolic link on the way to `.jigc/AGENT.md` ``.
    #[must_use]
    pub fn describe(&self) -> String {
        if self.at_leaf() {
            format!("`{}` is {}", self.at, self.shape.noun())
        } else {
            format!(
                "`{}` is {} on the way to `{}`",
                self.at,
                self.shape.noun(),
                self.aimed_at
            )
        }
    }

    /// The act that clears it, as a route clause — chosen by what the entry is, because one
    /// verb does not fit all of them: a link is *removed* (its target is not jigc's to
    /// name), a directory may hold the reader's files and is *moved*, and a link standing
    /// in for a directory is *replaced* by a real one.
    #[must_use]
    pub fn clearing_act(&self) -> String {
        match (self.shape, self.at_leaf()) {
            (ForeignEntry::Symlink, true) => format!("remove the link at `{}`", self.at),
            (ForeignEntry::Symlink, false) => {
                format!("replace the link at `{}` with a real directory", self.at)
            }
            (ForeignEntry::Directory, _) => {
                format!("move the directory at `{}` out of the way", self.at)
            }
            (ForeignEntry::Other, _) => format!("remove the special file at `{}`", self.at),
        }
    }
}

/// Ask what is on `relative`'s way below `root`, **without following a link** — `None`
/// when a replacing write there lands on a regular file (or creates one) at exactly that
/// path.
///
/// A directory on the way that is a **link** blocks; one that is merely absent does not
/// (the writer creates it), and one that is some other non-directory is left to the
/// writer's own `create_dir_all`, which fails loudly and writes nothing. The leaf blocks
/// when it is anything but a regular file or absent ([`engine::store::home_entry`], the one
/// no-follow occupancy read the promote doors share).
#[must_use]
pub fn blocker(root: &Path, relative: &str) -> Option<Blocker> {
    let mut walked = root.to_path_buf();
    let mut spelled = String::new();
    let components: Vec<&str> = relative
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();
    let (leaf, way) = components.split_last()?;
    for part in way {
        walked.push(part);
        if !spelled.is_empty() {
            spelled.push('/');
        }
        spelled.push_str(part);
        if std::fs::symlink_metadata(&walked).is_ok_and(|entry| entry.file_type().is_symlink()) {
            return Some(Blocker {
                at: spelled,
                shape: ForeignEntry::Symlink,
                aimed_at: relative.to_string(),
            });
        }
    }
    walked.push(leaf);
    match engine::store::home_entry(&walked) {
        HomeEntry::Free | HomeEntry::RegularFile => None,
        HomeEntry::Foreign(shape) => Some(Blocker {
            at: relative.to_string(),
            shape,
            aimed_at: relative.to_string(),
        }),
    }
}

/// The error a blocked replacing write returns — the state, then why jigc will not act on
/// it. One helper, so the door's pre-write refusal and the writer's own say the same thing.
fn refuse(blocked: &Blocker) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "{} — jigc replaces this file whole and does not write through or over anything \
             but a regular file",
            blocked.describe()
        ),
    )
}

/// Put `contents` at `<root>/<relative>` as a **regular file, never through a link**,
/// creating the directories on the way.
///
/// Refuses ([`blocker`]) before it creates or opens anything, so a refused write leaves no
/// directory behind either. An existing regular file is rewritten **in place** (same inode),
/// the property the `fs::write` this replaces had.
pub fn replace(root: &Path, relative: &str, contents: &[u8]) -> io::Result<()> {
    use std::io::Write as _;
    if let Some(blocked) = blocker(root, relative) {
        return Err(refuse(&blocked));
    }
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = open_no_follow(&path)?;
    file.set_len(0)?;
    file.write_all(contents)
}

/// Open `path` for writing as a **regular file, never through a link**, creating it when
/// absent — the open both the promote sink (`crate::task`) and [`replace`] write through.
///
/// `O_NOFOLLOW` makes a link at the last component fail the open (`ELOOP`) instead of
/// naming another file, and the **open handle** is then asked what it is before the caller
/// writes: only a regular file is returned. Asking the handle rather than the path is what
/// closes the gap a path check leaves — the entry that answers is the entry that is
/// written. `O_NONBLOCK` keeps the open from parking on a FIFO with no reader; it means
/// nothing for a regular file. The file is **not** truncated here: a caller that replaces
/// calls `set_len(0)` once it holds the handle, so nothing is destroyed by an open that is
/// about to be refused.
pub(crate) fn open_no_follow(path: &Path) -> io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt as _;
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)?;
    if !file.metadata()?.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "the destination is not a regular file",
        ));
    }
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-regular-file-unit-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            ));
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

    /// An absent path is created with its directories, and a regular file is rewritten in
    /// place — the two shapes every ordinary run meets.
    #[test]
    fn replace_creates_and_rewrites_a_regular_file() {
        let dir = TempDir::new();
        replace(dir.path(), "a/b/file", b"first, and longer\n").expect("create");
        replace(dir.path(), "a/b/file", b"second\n").expect("rewrite");
        assert_eq!(
            std::fs::read_to_string(dir.path().join("a/b/file")).expect("read"),
            "second\n",
            "the rewrite truncates: nothing of the longer first body is left behind"
        );
    }

    /// A link at the leaf refuses whatever it points at — and a dangling one's target is
    /// not created.
    #[test]
    fn replace_refuses_a_link_at_the_leaf_and_leaves_its_target_alone() {
        let dir = TempDir::new();
        std::fs::write(dir.path().join("target"), "theirs\n").expect("plant");
        symlink("target", dir.path().join("live")).expect("link");
        symlink("absent", dir.path().join("dangling")).expect("link");

        let err = replace(dir.path(), "live", b"ours\n").expect_err("a live link refuses");
        assert!(
            err.to_string().contains("`live` is a symbolic link"),
            "the error names the entry by its relative path: {err}"
        );
        assert_eq!(
            std::fs::read_to_string(dir.path().join("target")).expect("read"),
            "theirs\n"
        );
        replace(dir.path(), "dangling", b"ours\n").expect_err("a dangling link refuses");
        assert!(!dir.path().join("absent").exists(), "and creates nothing");
    }

    /// A linked directory on the way is named as such, and nothing is written below it.
    #[test]
    fn replace_refuses_a_linked_directory_on_the_way() {
        let dir = TempDir::new();
        std::fs::create_dir_all(dir.path().join("elsewhere")).expect("mkdir");
        symlink("elsewhere", dir.path().join("way")).expect("link");

        let blocked = blocker(dir.path(), "way/leaf").expect("blocked");
        assert_eq!(blocked.at, "way");
        assert!(!blocked.at_leaf());
        assert_eq!(
            blocked.describe(),
            "`way` is a symbolic link on the way to `way/leaf`"
        );
        replace(dir.path(), "way/leaf", b"ours\n").expect_err("refuses");
        assert!(!dir.path().join("elsewhere/leaf").exists());
    }

    /// A directory at the leaf is a blocker with its own clearing act, and an absent
    /// directory on the way is not one.
    #[test]
    fn the_blocker_names_the_act_that_fits_the_entry() {
        let dir = TempDir::new();
        std::fs::create_dir_all(dir.path().join("leaf")).expect("mkdir");
        symlink("nowhere", dir.path().join("link")).expect("link");
        assert_eq!(
            blocker(dir.path(), "leaf").expect("blocked").clearing_act(),
            "move the directory at `leaf` out of the way"
        );
        assert_eq!(
            blocker(dir.path(), "link").expect("blocked").clearing_act(),
            "remove the link at `link`"
        );
        assert_eq!(
            blocker(dir.path(), "link/below")
                .expect("blocked")
                .clearing_act(),
            "replace the link at `link` with a real directory"
        );
        assert_eq!(blocker(dir.path(), "not/yet/there"), None);
    }
}
