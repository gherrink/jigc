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

/// A sub-task freshly added under a milestone: the [`crate::state::MintedTask`]
/// returned by the reused mint, plus the milestone-relative bookkeeping the join
/// will read — the milestone `id` it was appended to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddedTask {
    /// The milestone the sub-task was appended to.
    pub milestone_id: String,
    /// The minted sub-task (its frozen id, isolated `tasks/<sub>/` area, and the
    /// **milestone's** base it inherited).
    pub task: crate::state::MintedTask,
}

/// Mint a sub-task under an existing milestone and append it to the milestone's
/// task list (`design/write-commands.md` → `jigc milestone add-task`;
/// `design/storage.md` → The by-task-id join). The sub-task is a task work-unit:
/// its id is the frozen slug of `intent`, it lives in the **shared**
/// `<jigc_root>/tasks/<sub>/` namespace in its own isolated area, and it inherits
/// the milestone's **single shared base** (read back from the milestone area, not
/// a fresh HEAD) so the join's "present at the milestone base" stays a lookup
/// against a frozen commit. `workflow_id` is the workflow the sub-task is minted
/// from (supplied by the caller), persisted by [`crate::state::mint_task`].
///
/// **Unknown milestone** — no area at `<jigc_root>/milestones/<milestone_id>/` →
/// reject with a blocking [`Finding`]; nothing is minted. **Within-milestone
/// serial collision** — the minted sub-task id is already in *this milestone's*
/// task list → reject with a routed blocking [`Finding`], appending nothing (the
/// `-2`/`-3` suffix is the join's, never incremental add). A slug colliding with a
/// *non-milestone* task is the existing `task.serial-collision` path inside
/// [`crate::state::mint_task`], surfaced unchanged.
pub fn add_task(
    jigc_root: &Path,
    milestone_id: &str,
    intent: &str,
    workflow_id: &str,
) -> Result<AddedTask, Finding> {
    let dir = milestone_dir(jigc_root, milestone_id);

    // Unknown milestone → reject before anything is minted.
    if !dir.is_dir() {
        return Err(unknown_milestone_finding(milestone_id));
    }

    // The single shared base every sub-task inherits — read back from the
    // milestone area, never a fresh HEAD.
    let base = read_base_pin(&dir)
        .map_err(|err| io_finding(milestone_id, "read the shared base pin", &err))?;

    // The id the sub-task will mint to — checked against *this milestone's* list
    // before the mint, so a within-milestone collision rejects without side effect.
    let sub_id = mint_sub_id(intent);
    let mut list =
        read_task_list(&dir).map_err(|err| io_finding(milestone_id, "read the task list", &err))?;
    if list.tasks.iter().any(|t| t == &sub_id) {
        return Err(sub_task_collision_finding(milestone_id, &sub_id));
    }

    // Reuse the task mint so the sub-task inherits the milestone's base and the
    // standard working-area files (base/intent/workflow), in the shared namespace.
    let task = crate::state::mint_task(jigc_root, intent, SUB_TASK_TYPE, workflow_id, base)?;

    // Append to the milestone's task list and persist it.
    list.tasks.push(task.id.clone());
    std::fs::write(dir.join(TASKS_FILE), list.to_bytes())
        .map_err(|err| io_finding(milestone_id, "append to the task list", &err))?;

    Ok(AddedTask {
        milestone_id: milestone_id.to_string(),
        task,
    })
}

/// The type-name a milestone sub-task mints under — the empty-intent fallback for
/// [`crate::state::mint_task`]. A sub-task is a `task` work-unit; the workflow it
/// runs is supplied separately and the fallback only fires on an empty slug.
const SUB_TASK_TYPE: &str = "task";

/// Slug `intent` into the sub-task id with the empty → `task` type-name fallback —
/// the *same* result [`crate::state::mint_task`] computes, recomputed here only to
/// key the within-milestone collision check before the mint (no separate
/// minting discipline).
fn mint_sub_id(intent: &str) -> String {
    let slug = crate::slug::slugify(intent);
    if slug.is_empty() {
        crate::slug::slugify(SUB_TASK_TYPE)
    } else {
        slug
    }
}

/// The unknown-milestone block: a blocking finding naming the missing milestone,
/// routing the agent to create it first (`write-commands.md` → add-task mints
/// *under* an existing milestone).
fn unknown_milestone_finding(milestone_id: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.unknown",
        format!("milestone `{milestone_id}` does not exist"),
        Some(Location::addressed(
            format!("milestone:{milestone_id}"),
            1,
            1,
        )),
        Some("create it first with `jigc milestone create \"<title>\"`".to_string()),
    )
}

/// The within-milestone serial-collision block: a blocking finding naming the
/// already-listed sub-task id, routing the agent to a distinct intent (the `-2`
/// suffix is the join's, never incremental add; `write-commands.md` → add-task).
fn sub_task_collision_finding(milestone_id: &str, sub_id: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.sub-task-collision",
        format!("sub-task `{sub_id}` is already in milestone `{milestone_id}`"),
        Some(Location::addressed(format!("task:{sub_id}"), 1, 1)),
        Some("add the sub-task with a distinct intent".to_string()),
    )
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

    /// The done-criterion for T2 (`design/write-commands.md` → `jigc milestone
    /// add-task`; `design/storage.md` → The by-task-id join): two distinct
    /// sub-tasks each open an **isolated** `tasks/<sub>/` area whose `base.json`
    /// equals the **milestone's** shared base (not a fresh HEAD), and both ids
    /// land in the milestone's task list. A third add whose intent slugs to an
    /// already-listed id returns a routed blocking collision and appends nothing;
    /// an add against an unknown milestone rejects.
    #[test]
    fn add_task_mints_isolated_sub_tasks_on_the_shared_base_and_rejects_collision() {
        let root = TempRoot::new("add-task");
        let milestone_base = BasePin::new("1111111111111111111111111111111111111111", "1111111");

        let milestone = mint_milestone(root.path(), "Cache rework", milestone_base.clone())
            .expect("milestone mints");

        // First sub-task.
        let a = add_task(
            root.path(),
            &milestone.id,
            "Add rate limiter",
            "single-task",
        )
        .expect("first sub-task adds");
        assert_eq!(a.milestone_id, "cache-rework");
        assert_eq!(a.task.id, "add-rate-limiter");
        // Isolated area in the shared tasks/ namespace.
        let a_dir = root.path().join("tasks").join("add-rate-limiter");
        assert_eq!(a.task.dir, a_dir);
        assert!(a_dir.is_dir(), "sub-task opens its own isolated area");
        // Its base IS the milestone's shared base, not a fresh HEAD.
        assert_eq!(
            a.task.base, milestone_base,
            "sub-task inherits the shared base"
        );
        assert_eq!(
            crate::state::read_base_pin(&a_dir).expect("read sub base"),
            milestone_base,
            "the sub-task's base.json equals the milestone's base on disk"
        );

        // Second, distinct sub-task — its own isolated area, same shared base.
        let b = add_task(
            root.path(),
            &milestone.id,
            "Evict stale keys",
            "single-task",
        )
        .expect("second sub-task adds");
        assert_eq!(b.task.id, "evict-stale-keys");
        let b_dir = root.path().join("tasks").join("evict-stale-keys");
        assert!(
            b_dir.is_dir() && b_dir != a_dir,
            "the two areas are distinct"
        );
        assert_eq!(
            crate::state::read_base_pin(&b_dir).expect("read sub base"),
            milestone_base,
            "the second sub-task also inherits the milestone's shared base"
        );

        // Both ids are in the milestone's task list.
        let list = read_task_list(&milestone.dir).expect("read list");
        assert_eq!(
            list.tasks,
            vec![
                "add-rate-limiter".to_string(),
                "evict-stale-keys".to_string()
            ],
            "both sub-task ids land in the milestone's task list"
        );

        // A third add whose intent slugs to an already-listed id → routed block,
        // appends nothing, mints nothing new.
        let tasks_before = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .count();
        let collide = add_task(
            root.path(),
            &milestone.id,
            "Add rate limiter",
            "single-task",
        )
        .expect_err("a within-milestone slug collision rejects");
        assert_eq!(collide.severity, Severity::Blocking);
        assert_eq!(collide.code, "milestone.sub-task-collision");
        assert!(
            collide.message.contains("add-rate-limiter")
                && collide.message.contains("cache-rework"),
            "the block names the colliding sub-task and the milestone: {collide:?}"
        );
        assert!(collide.route.is_some(), "the collision carries a route");
        // Nothing appended: the list is unchanged.
        assert_eq!(
            read_task_list(&milestone.dir).expect("read list").tasks,
            list.tasks,
            "a collision appends nothing to the task list"
        );
        let tasks_after = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .count();
        assert_eq!(tasks_before, tasks_after, "a collision mints no new area");

        // Add against an unknown milestone → reject.
        let unknown = add_task(root.path(), "no-such-milestone", "Whatever", "single-task")
            .expect_err("an unknown milestone rejects");
        assert_eq!(unknown.severity, Severity::Blocking);
        assert_eq!(unknown.code, "milestone.unknown");
        assert!(
            unknown.route.is_some(),
            "the unknown-milestone block carries a route"
        );
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
