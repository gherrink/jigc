//! The `milestone` work-unit — a higher-level fan-out container in the work-unit
//! family (`milestone > task` for M7; see `design/structural-grammar.md` →
//! Work-units and runtime identity). A milestone shares the work-unit minting
//! discipline (frozen content-slug from its title, empty → the type-name
//! fallback) but has no internal section/leaf structure; its address is
//! `milestone:<slug>` with **no fragment**.
//!
//! This module lands the **mint site** (`design/write-commands.md` → Minting a
//! milestone): `jigc milestone create "<title>"` slugs the title into a frozen
//! id, opens the milestone's gitignored area at `.jigc/milestones/<id>/`, pins
//! **one shared base** (the commit every sub-task will inherit) and an **empty
//! task list**. The single shared base is what makes the by-task-id join's
//! "present at the milestone base" a deterministic lookup against a frozen commit
//! rather than a live-filesystem race (`design/storage.md` → The by-task-id
//! join). A serial collision — an active milestone dir of that id already exists
//! — **rejects** with a routed blocking [`Finding`]; nothing new is created (the
//! deterministic `-2`/`-3` collision suffix runs only at the join, never at the
//! mint site).
//!
//! The base SHA is supplied by the caller (the CLI reads HEAD via
//! `git rev-parse` — "CLI orchestrates, git executes"); the engine performs no
//! git I/O and never shells out, so minting is a pure function of
//! (jigc-root, title, base) → on-disk effect, golden-testable. No sub-task is
//! minted here — `add_task` (the next task) is the incremental populator.

use crate::finding::{Finding, Location, Severity};
use crate::state::BasePin;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The base-pin filename inside a milestone's area — the **single shared base**
/// every sub-task inherits, in the same frozen byte form as a task's `base.json`.
const BASE_PIN_FILE: &str = "base.json";

/// The task-list filename inside a milestone's area — the sub-task ids the
/// by-task-id join will enumerate, persisted as deterministic engine state.
const TASKS_FILE: &str = "tasks.json";

/// A milestone's persisted task list — the sub-task ids appended by `add_task`,
/// the collection the by-task-id join enumerates (`design/storage.md` → The
/// by-task-id join). Serialized as a JSON list in a stable, golden-locked byte
/// form; created **empty** at mint (`design/write-commands.md` → Minting a
/// milestone: "opens its area with an empty task list … No sub-task minted yet").
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskList {
    /// The sub-task ids belonging to this milestone, in the order written.
    /// Deterministic *enumeration* by id is a later task's concern; this field is
    /// the recorded backing state.
    pub tasks: Vec<String>,
}

impl TaskList {
    /// Serialize to the frozen on-disk byte form: pretty JSON, one trailing
    /// newline (golden-locked, matching the `base.json` convention in
    /// [`crate::state`]).
    fn to_bytes(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("TaskList serializes");
        s.push('\n');
        s
    }
}

/// A freshly minted milestone: its frozen slug `id`, its area `dir`, and the
/// single shared `base` every sub-task inherits.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintedMilestone {
    /// The frozen content-slug id (slugged from the title, type-name fallback).
    pub id: String,
    /// The milestone's area, `<jigc_root>/milestones/<id>/`.
    pub dir: PathBuf,
    /// The single shared base every sub-task is pinned to.
    pub base: BasePin,
}

/// The on-disk directory a milestone's state lives in: `<jigc_root>/milestones/<id>/`.
pub fn milestone_dir(jigc_root: &Path, id: &str) -> PathBuf {
    jigc_root.join("milestones").join(id)
}

/// Mint a milestone: slug the `title` (empty → the `milestone` type-name
/// fallback, the same [`crate::slug::slugify`] + fallback discipline as
/// `state::mint_task`), reject on a serial collision with an existing milestone
/// dir, else open `<jigc_root>/milestones/<id>/`, write the **single shared
/// base-pin** capturing `base`, and write an **empty task list**.
///
/// `jigc_root` is the project's `.jigc/` home (a temp root under test). On
/// success the milestone area, its `base.json`, and an empty `tasks.json` exist
/// on disk. On a serial collision the returned [`Finding`] is
/// [`Severity::Blocking`], names the existing milestone, and carries a route —
/// nothing new is created.
pub fn mint_milestone(
    jigc_root: &Path,
    title: &str,
    base: BasePin,
) -> Result<MintedMilestone, Finding> {
    let id = mint_id(title);
    let dir = milestone_dir(jigc_root, &id);

    // Serial collision: an active milestone dir of that id already exists →
    // reject, never silently suffixed or reused (the `-2` suffix is the join's).
    if dir.exists() {
        return Err(collision_finding(&id));
    }

    std::fs::create_dir_all(&dir)
        .map_err(|err| io_finding(&id, "open the milestone area", &err))?;

    let pin_path = dir.join(BASE_PIN_FILE);
    let pin_body = render_base_pin(&base);
    std::fs::write(&pin_path, pin_body)
        .map_err(|err| io_finding(&id, "write the base pin", &err))?;

    // The empty task list — no sub-task minted yet.
    std::fs::write(dir.join(TASKS_FILE), TaskList::default().to_bytes())
        .map_err(|err| io_finding(&id, "write the task list", &err))?;

    Ok(MintedMilestone { id, dir, base })
}

/// Read the persisted [`TaskList`] of a milestone from its area
/// (`<jigc_root>/milestones/<id>/tasks.json`) — the companion of the mint write,
/// read back to enumerate the sub-tasks. A missing or malformed file is an error
/// (the list is written at mint, so its absence is a real fault).
pub fn read_task_list(milestone_dir: &Path) -> std::io::Result<TaskList> {
    let bytes = std::fs::read(milestone_dir.join(TASKS_FILE))?;
    serde_json::from_slice(&bytes)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
}

/// Read the persisted single shared [`BasePin`] of a milestone from its area
/// (`<jigc_root>/milestones/<id>/base.json`) — the base every sub-task inherits.
/// A missing or malformed pin is an error (it is written at mint).
pub fn read_base_pin(milestone_dir: &Path) -> std::io::Result<BasePin> {
    let bytes = std::fs::read(milestone_dir.join(BASE_PIN_FILE))?;
    serde_json::from_slice(&bytes)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
}

/// Slug the title into the milestone id, applying the empty → type-name
/// (`milestone`) fallback — the same discipline as `state::mint_id`.
fn mint_id(title: &str) -> String {
    let slug = crate::slug::slugify(title);
    if slug.is_empty() {
        crate::slug::slugify("milestone")
    } else {
        slug
    }
}

/// Render the base-pin file body — the frozen on-disk form (golden-locked,
/// identical to `state.rs`'s `base.json`).
fn render_base_pin(base: &BasePin) -> String {
    let mut s = serde_json::to_string_pretty(base).expect("BasePin serializes");
    s.push('\n');
    s
}

/// The serial-collision block: a blocking finding naming the existing milestone
/// and routing the agent to add tasks to it or discard it (`write-commands.md` →
/// Minting a milestone; the mint rejects, never suffixes at this site).
fn collision_finding(id: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.serial-collision",
        format!("milestone `{id}` already exists"),
        Some(Location::addressed(format!("milestone:{id}"), 1, 1)),
        Some(format!(
            "add tasks with `jigc milestone add-task {id} \"<intent>\"` or pick a different title"
        )),
    )
}

/// A blocking finding for an area I/O failure during minting.
fn io_finding(id: &str, doing: &str, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.area-io",
        format!("could not {doing} for milestone `{id}`: {err}"),
        Some(Location::addressed(format!("milestone:{id}"), 1, 1)),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop — keeps mint tests off
    /// any real `.jigc/` tree (mirrors `state.rs`'s `TempRoot`).
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-milestone-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp root");
            TempRoot(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The done-criterion: minting "Cache rework" opens
    /// `.jigc/milestones/cache-rework/` with a single shared base-pin recording
    /// the supplied HEAD and an **empty** task list that reads back empty; a
    /// second mint of the same slug rejects with a route-bearing blocking
    /// finding naming the existing milestone and creates nothing new (golden
    /// over the frozen on-disk bytes).
    #[test]
    fn mint_creates_milestone_area_shared_base_and_empty_task_list() {
        let root = TempRoot::new("create");
        let base = BasePin::new("0123456789abcdef0123456789abcdef01234567", "0123456");

        let minted =
            mint_milestone(root.path(), "Cache rework", base.clone()).expect("first mint succeeds");

        assert_eq!(
            minted.id, "cache-rework",
            "address is `milestone:cache-rework`"
        );
        let dir = root.path().join("milestones").join("cache-rework");
        assert_eq!(minted.dir, dir);
        assert!(dir.is_dir(), "milestone area must exist");
        assert_eq!(minted.base, base);

        // Golden over the frozen single-shared-base byte form for a fixed SHA.
        let pin = std::fs::read_to_string(dir.join(BASE_PIN_FILE)).expect("base pin written");
        insta::assert_snapshot!("milestone_base_pin", pin);
        // The pin round-trips back to the supplied base.
        let back: BasePin = serde_json::from_str(&pin).expect("pin parses");
        assert_eq!(back, base);
        assert_eq!(
            read_base_pin(&dir).expect("read base pin"),
            base,
            "the shared base reads back through the reader"
        );

        // Golden over the frozen empty-task-list byte form.
        let tasks = std::fs::read_to_string(dir.join(TASKS_FILE)).expect("task list written");
        insta::assert_snapshot!("milestone_empty_task_list", tasks);
        // The task list reads back empty.
        assert_eq!(
            read_task_list(&dir).expect("read task list").tasks,
            Vec::<String>::new(),
            "a freshly minted milestone has an empty task list"
        );

        // Second mint of the same slug → serial reject, nothing new created.
        let before = std::fs::read_dir(root.path().join("milestones"))
            .expect("milestones dir")
            .count();
        let err = mint_milestone(root.path(), "Cache rework", base.clone())
            .expect_err("re-mint of the same slug rejects");
        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "milestone.serial-collision");
        assert!(
            err.message.contains("cache-rework"),
            "block must name the existing milestone: {err:?}"
        );
        assert!(err.route.is_some(), "serial collision carries a route");
        let after = std::fs::read_dir(root.path().join("milestones"))
            .expect("milestones dir")
            .count();
        assert_eq!(before, after, "no second dir created on collision");
    }

    /// A title that normalizes to nothing falls back to the `milestone` type
    /// name (the same fallback discipline as `state::mint_id`).
    #[test]
    fn empty_title_falls_back_to_type_name() {
        let root = TempRoot::new("fallback");
        let base = BasePin::new("a".repeat(40), "aaaaaaa");

        let minted =
            mint_milestone(root.path(), "!!!___---", base).expect("fallback mint succeeds");
        assert_eq!(
            minted.id, "milestone",
            "stripped-to-empty title uses the type name"
        );
        assert!(root.path().join("milestones").join("milestone").is_dir());
    }
}
