//! **The byte table's owed list** — the pilot's write leaves that have no row yet
//! (`crates/cli/tests/byte_table.rs` → the leaf fence). A file of its own under the suite's
//! directory, wired in by one `#[path]` line: the list is emptied by the plan's later tasks,
//! and the last of them removes this file with it.

/// The tasks of the plan that owe a leaf its row.
pub const OWING: &[&str] = &["B4", "B5", "B6", "B7", "B8", "B9"];

/// **The owed list** — the pilot's write leaves that have no row yet, each under the task
/// of the plan that owes it. `B4` was the first cut's; `B5` to `B9` are built now, one
/// family per builder — the human's ruling of 2026-10-13 at the pilot's stop, option A —
/// and their leaves stand here until each family brings its rows; the scheduled list is for
/// the six leaves the same ruling leaves for before the port. The four leaves of the
/// acceptance left this list with the task `B2c`, which brought their rows, and `task
/// discard` with the first commit of `B4`, which brought its own. `ingest` and
/// `unmanage` joined it with the task `B3`, whose count holds them touched under the plan's
/// rule where the plan's script had not: they stand under `B5`, the install family's task,
/// whose docs they share with `setup` and `uninstall`, and left it with that task's
/// second commit, which brought their rows. The eight `doc` leaves left it with the doc
/// family's first commit of `B6`, which brought theirs, the seven `milestone` leaves with
/// the first commit of `B7`'s second part, and the four of the movers family with `B8`'s
/// second commit.
pub const OWED: &[(&[&str], &str)] = &[
    (&["start"], "B9"),
    (&["workflow"], "B9"),
    (&["migrate"], "B9"),
    (&["task", "amend"], "B9"),
];

/// **The leaves whose cells are owed** — a leaf that has its row, and whose cells beyond
/// the pilot's acceptance are a later task's to bring: the cell fence
/// (`byte_table::every_need_of_a_row_has_a_cell_a_scheduled_row_or_is_owed`) asks nothing
/// of such a leaf need by need, and names it the day it also has a scheduled row. `B4`
/// was the first cut's, and brought `jigc task finalize`'s and `jigc task discard`'s; `B5`
/// brought the install family's four; `B6` brought the doc family's eight — the six edit
/// leaves with its second commit, the two minting doors with its third — and `B7` the
/// `milestone` family's eight, by the human's ruling of 2026-10-13: its second part brought
/// the rows of the family's seven other leaves, and their cells and `milestone finalize`'s
/// in three parts after. The list was empty from then until `B8`'s rows came, a commit
/// before their cells — the movers family's four, and with them their spellings' needs, owed
/// until a cell of a spelling comes — and is empty again since `B8`'s fourth commit brought
/// the last of them. A leaf leaves this list in the commit that brings its cells — after
/// which every need of its row is a cell or a line of `scheduled-cells.tsv` with its reason.
pub const CELLS_OWED: &[(&[&str], &str)] = &[];
