//! **The byte table's owed list** — the pilot's write leaves that have no row yet
//! (`crates/cli/tests/byte_table.rs` → the leaf fence). A file of its own under the suite's
//! directory, wired in by one `#[path]` line: the list is emptied by the plan's later tasks,
//! and the last of them removes this file with it.

/// The tasks of the plan that owe a leaf its row.
pub const OWING: &[&str] = &["B4", "B5", "B6", "B7", "B8", "B9"];

/// **The owed list** — the pilot's write leaves that have no row yet, each under the task
/// of the plan that owes it. `B4` is the first cut's; `B5` to `B9` are what would follow if
/// the human rules for it at the stop, and until then their leaves stand here rather than
/// on the scheduled list, which is for a leaf with a reason to wait. The four leaves of the
/// acceptance left this list with the task `B2c`, which brought their rows.
pub const OWED: &[(&[&str], &str)] = &[
    (&["task", "discard"], "B4"),
    (&["doc", "create"], "B6"),
    (&["doc", "add-item"], "B6"),
    (&["doc", "remove-item"], "B6"),
    (&["doc", "retitle-item"], "B6"),
    (&["doc", "rename"], "B6"),
    (&["doc", "set-field"], "B6"),
    (&["doc", "set-slot"], "B6"),
    (&["doc", "author"], "B6"),
    (&["milestone", "create"], "B7"),
    (&["milestone", "add-task"], "B7"),
    (&["milestone", "add-from-spec"], "B7"),
    (&["milestone", "provision"], "B7"),
    (&["milestone", "execute"], "B7"),
    (&["milestone", "join"], "B7"),
    (&["milestone", "discard"], "B7"),
    (&["rename"], "B8"),
    (&["relocate"], "B8"),
    (&["migrate-corpus"], "B8"),
    (&["config", "set"], "B8"),
    (&["start"], "B9"),
    (&["workflow"], "B9"),
    (&["migrate"], "B9"),
    (&["task", "amend"], "B9"),
];
