//! The route catalog's cases for `jigc setup` — the family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here.** The hook's link refusal, from the frame's first proof; and the two
//! refusals beside it that the install raises before its first write — over an entry in the
//! way of a file it **replaces** (`replaced_path_refusal`, six codes) and over a link at a
//! file it **merges into** that leads where it will not follow (`merged_through_links`, four
//! codes). Each of those ten sites has a row of [`READINGS`]: the functions that were read
//! to count its causes, the count, and every cause that has no case with why.
//!
//! **A cause is one way through the callee to the constructor**: which entry is in the way
//! — the file's own path, or a directory on the way to it — what that entry is, and, for a
//! link the install would follow, which of the ends it refuses. Whether git tracks the entry
//! is no branch of either callee; the route's prose turns on it (*committing the removal
//! where git tracks the entry*), so one case a function follows that clause, under the cause
//! of the case beside it. Two cases of one cause carry the same [`Case::cause`] to the
//! letter — that is how [`READINGS`] is held to the cases.

use super::{Reading, Reason, Why};
use crate::support::route_case::{Act, Case, Check, Operand, Place, Step};

/// The hook `jigc setup` installs, from the repository root.
pub const HOOK: &str = ".git/hooks/pre-commit";

/// The refusal a `pre-commit` hook draws when it is a link that leads out of the
/// repository and its hooks directory, or to nothing.
pub const HOOK_LINK_SITE: &str = "crates/cli/src/setup.rs::hook_link_refusal::setup.install-hook#1";

// The six codes `replaced_path_refusal` raises — one a member of the install whose writer
// replaces what it finds, in the order the install writes them.
const BOOTSTRAP: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.write-bootstrap#dynamic-1";
const STAMP: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.version-stamp#dynamic-1";
const LAYER: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.init-project-layer#dynamic-1";
const MARKER: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.compose-marker#dynamic-1";
const RECORD: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.inject-allowlist#dynamic-1";
const GUIDE: &str = "crates/cli/src/setup.rs::replaced_path_refusal::setup.write-guide#dynamic-1";

// The four codes `merged_through_links` raises — one a member whose writer keeps what it
// finds.
const HOST: &str =
    "crates/cli/src/setup.rs::merged_through_links::setup.inject-reference#dynamic-1";
const SETTINGS: &str =
    "crates/cli/src/setup.rs::merged_through_links::setup.inject-allowlist#dynamic-1";
const SEED: &str =
    "crates/cli/src/setup.rs::merged_through_links::setup.secrets-gitignore#dynamic-1";
const IGNORE: &str =
    "crates/cli/src/setup.rs::merged_through_links::setup.init-project-layer#dynamic-1";

// The install's members, from the repository root.
const AGENT_MD: &str = ".jigc/AGENT.md";
const VERSION: &str = ".jigc/version";
const GITKEEP: &str = ".jigc/config/.gitkeep";
const PACKS_YAML: &str = ".jigc/config/packs.yaml";
const SETTINGS_RECORD: &str = ".jigc/settings-entries.json";
const CLAUDE_MD: &str = "CLAUDE.md";
const SETTINGS_JSON: &str = ".claude/settings.json";
const ROOT_GITIGNORE: &str = ".gitignore";
const JIGC_GITIGNORE: &str = ".jigc/.gitignore";

/// What every case of `jigc setup` shares. A case states its acts itself: one that leaves
/// them empty under a printed route is refused by the runner.
const SETUP: Case = Case {
    name: "",
    site: "",
    cause: "",
    state: &[],
    refusing: &["setup"],
    acts: &[],
    holds: &[],
    witnesses: &[],
    doubt: None,
};

// **The acts.** Every route here prints the entry in the way as its first span and the
// re-run as the one after the entries — `--force`, the span after that, is named to say it
// changes nothing, and no case runs it.
const REMOVE: &[Act] = &[Act::Remove(Operand::Span(0)), Act::Run(1)];
const REMOVE_AND_COMMIT: &[Act] = &[
    Act::Remove(Operand::Span(0)),
    Act::Commit(Operand::Span(0)),
    Act::Run(1),
];
const MOVE_AWAY: &[Act] = &[Act::MoveAway(Operand::Span(0)), Act::Run(1)];
const MAKE_DIRECTORY: &[Act] = &[Act::ReplaceWithDirectory(Operand::Span(0)), Act::Run(1)];
const REMOVE_BOTH: &[Act] = &[
    Act::Remove(Operand::Span(0)),
    Act::Remove(Operand::Span(1)),
    Act::Run(2),
];
const POINT_AT_COMMITTED: &[Act] = &[Act::PointAtCommitted(Operand::Span(0)), Act::Run(1)];
const POINT_AT_NEW: &[Act] = &[Act::PointAtNew(Operand::Span(0)), Act::Run(1)];
const POINT_AT_COMMITTED_AND_COMMIT: &[Act] = &[
    Act::PointAtCommitted(Operand::Span(0)),
    Act::Commit(Operand::Span(0)),
    Act::Run(1),
];
const POINT_AT_NEW_AND_COMMIT: &[Act] = &[
    Act::PointAtNew(Operand::Span(0)),
    Act::Commit(Operand::Span(0)),
    Act::Run(1),
];

/// A file of somebody's outside the repository — the witness of most cases.
const THEIRS: Step = Step::File(Place::Outside("theirs"), "theirs\n");

/// Commit everything the steps before it planted.
const COMMIT_IT: [Step; 2] = [
    Step::Git(&["add", "--all"]),
    Step::Git(&["commit", "-q", "-m", "what was planted"]),
];

/// Take the sandbox's one commit away again: `HEAD` is unborn, which is where `jigc setup`
/// seeds the root `.gitignore`.
const UNBORN: Step = Step::Git(&["update-ref", "-d", "HEAD"]);

// The causes, to the letter — two cases of one cause carry the same words.
const AGENT_A_LINK: &str = "`.jigc/AGENT.md` is a link";
const HOST_LEADS_OUT: &str = "`CLAUDE.md` is a link to a file outside the repository";
const SETTINGS_LEADS_OUT: &str =
    "`.claude/settings.json` is a link to a file outside the repository";
const SEED_LEADS_OUT: &str =
    "on an unborn `HEAD`, the root `.gitignore` is a link to a file outside the repository";

/// The two readings of *a regular file inside this repository*.
const WHICH_FILE: &str = "*point it at a regular file inside this repository*: (a) any regular \
                          file in the checkout, one written a moment ago and in no commit \
                          included — followed here; (b) a file the repository already holds \
                          committed — the case `setup-host-committed-link-pointed-at-a-committed-file`";

/// The cases of `jigc setup`. A case follows one way out of its route: where a route offers two —
/// *remove the link, or point it at a script inside this repository* — the other way is a
/// case of its own.
pub const CASES: &[Case] = &[
    Case {
        name: "setup-hook-linked-to-a-script-outside",
        site: HOOK_LINK_SITE,
        cause: "the `pre-commit` hook is a link to an existing script outside the repository",
        state: &[
            Step::Script(Place::Outside("pre-commit"), "#!/bin/sh\necho ours\n"),
            Step::Link(Place::Repo(HOOK), Place::Outside("pre-commit")),
        ],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(HOOK))],
        witnesses: &["pre-commit"],
        ..SETUP
    },
    Case {
        name: "setup-hook-linked-to-nothing",
        site: HOOK_LINK_SITE,
        cause: "the `pre-commit` hook is a link to a path outside the repository that holds \
                nothing",
        state: &[Step::Link(Place::Repo(HOOK), Place::Outside("pre-commit"))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(HOOK))],
        witnesses: &["pre-commit"],
        ..SETUP
    },
    // ---- `replaced_path_refusal` · `setup.write-bootstrap` · `.jigc/AGENT.md` ----
    Case {
        name: "setup-bootstrap-file-a-link",
        site: BOOTSTRAP,
        cause: AGENT_A_LINK,
        state: &[THEIRS, Step::Link(Place::Repo(AGENT_MD), Place::Outside("theirs"))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-bootstrap-file-a-committed-link",
        site: BOOTSTRAP,
        cause: AGENT_A_LINK,
        state: &[
            THEIRS,
            Step::Link(Place::Repo(AGENT_MD), Place::Outside("theirs")),
            COMMIT_IT[0],
            COMMIT_IT[1],
        ],
        acts: REMOVE_AND_COMMIT,
        holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-bootstrap-file-a-directory",
        site: BOOTSTRAP,
        cause: "`.jigc/AGENT.md` is a directory",
        state: &[Step::File(Place::Repo(".jigc/AGENT.md/kept"), "kept\n")],
        acts: MOVE_AWAY,
        holds: &[
            Check::RegularFile(Place::Repo(AGENT_MD)),
            Check::RegularFile(Place::Moved(".jigc/AGENT.md/kept")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-bootstrap-file-a-special-file",
        site: BOOTSTRAP,
        cause: "`.jigc/AGENT.md` is a FIFO",
        state: &[Step::Fifo(Place::Repo(AGENT_MD))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
        ..SETUP
    },
    Case {
        name: "setup-workbench-a-link",
        site: BOOTSTRAP,
        cause: "`.jigc`, on the way to `.jigc/AGENT.md`, is a link to a directory outside the \
                repository",
        state: &[
            Step::File(Place::Outside("store/theirs"), "theirs\n"),
            Step::Link(Place::Repo(".jigc"), Place::Outside("store")),
        ],
        acts: MAKE_DIRECTORY,
        holds: &[
            Check::Directory(Place::Repo(".jigc")),
            Check::RegularFile(Place::Repo(AGENT_MD)),
        ],
        witnesses: &["store/theirs", "store/AGENT.md", "store/version"],
        ..SETUP
    },
    Case {
        name: "setup-two-replaced-files-links",
        site: BOOTSTRAP,
        cause: "two replaced files are in the way at once — `.jigc/AGENT.md` and \
                `.jigc/version` are both links — and the route joins their acts",
        state: &[
            THEIRS,
            Step::File(Place::Outside("also-theirs"), "also theirs\n"),
            Step::Link(Place::Repo(AGENT_MD), Place::Outside("theirs")),
            Step::Link(Place::Repo(VERSION), Place::Outside("also-theirs")),
        ],
        acts: REMOVE_BOTH,
        holds: &[
            Check::RegularFile(Place::Repo(AGENT_MD)),
            Check::RegularFile(Place::Repo(VERSION)),
        ],
        witnesses: &["theirs", "also-theirs"],
        ..SETUP
    },
    // ---- `replaced_path_refusal` · `setup.version-stamp` · `.jigc/version` ----
    Case {
        name: "setup-version-stamp-a-link",
        site: STAMP,
        cause: "`.jigc/version` is a link",
        state: &[THEIRS, Step::Link(Place::Repo(VERSION), Place::Outside("theirs"))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(VERSION))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-version-stamp-a-directory",
        site: STAMP,
        cause: "`.jigc/version` is a directory",
        state: &[Step::File(Place::Repo(".jigc/version/kept"), "kept\n")],
        acts: MOVE_AWAY,
        holds: &[
            Check::RegularFile(Place::Repo(VERSION)),
            Check::RegularFile(Place::Moved(".jigc/version/kept")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-version-stamp-a-special-file",
        site: STAMP,
        cause: "`.jigc/version` is a FIFO",
        state: &[Step::Fifo(Place::Repo(VERSION))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(VERSION))],
        ..SETUP
    },
    // ---- `replaced_path_refusal` · `setup.init-project-layer` · `.jigc/config/.gitkeep` ----
    Case {
        name: "setup-project-layer-a-link",
        site: LAYER,
        cause: "`.jigc/config`, on the way to `.jigc/config/.gitkeep`, is a link to a \
                directory outside the repository",
        state: &[
            Step::File(Place::Outside("config/theirs"), "theirs\n"),
            Step::Link(Place::Repo(".jigc/config"), Place::Outside("config")),
        ],
        acts: MAKE_DIRECTORY,
        holds: &[
            Check::Directory(Place::Repo(".jigc/config")),
            Check::RegularFile(Place::Repo(GITKEEP)),
        ],
        witnesses: &["config/theirs", "config/.gitkeep", "config/packs.yaml"],
        ..SETUP
    },
    Case {
        name: "setup-layer-marker-a-link",
        site: LAYER,
        cause: "`.jigc/config/.gitkeep` is a link",
        state: &[THEIRS, Step::Link(Place::Repo(GITKEEP), Place::Outside("theirs"))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(GITKEEP))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-layer-marker-a-directory",
        site: LAYER,
        cause: "`.jigc/config/.gitkeep` is a directory",
        state: &[Step::File(Place::Repo(".jigc/config/.gitkeep/kept"), "kept\n")],
        acts: MOVE_AWAY,
        holds: &[
            Check::RegularFile(Place::Repo(GITKEEP)),
            Check::RegularFile(Place::Moved(".jigc/config/.gitkeep/kept")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-layer-marker-a-special-file",
        site: LAYER,
        cause: "`.jigc/config/.gitkeep` is a FIFO",
        state: &[Step::Fifo(Place::Repo(GITKEEP))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(GITKEEP))],
        ..SETUP
    },
    // ---- `replaced_path_refusal` · `setup.compose-marker` · `.jigc/config/packs.yaml` ----
    Case {
        name: "setup-compose-marker-a-link",
        site: MARKER,
        cause: "`.jigc/config/packs.yaml` is a link",
        state: &[
            Step::File(Place::Outside("theirs"), "packs: []\n"),
            Step::Link(Place::Repo(PACKS_YAML), Place::Outside("theirs")),
        ],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(PACKS_YAML))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-compose-marker-a-directory",
        site: MARKER,
        cause: "`.jigc/config/packs.yaml` is a directory",
        state: &[Step::File(Place::Repo(".jigc/config/packs.yaml/kept"), "kept\n")],
        acts: MOVE_AWAY,
        holds: &[
            Check::RegularFile(Place::Repo(PACKS_YAML)),
            Check::RegularFile(Place::Moved(".jigc/config/packs.yaml/kept")),
        ],
        ..SETUP
    },
    // ---- `replaced_path_refusal` · `setup.inject-allowlist` · `.jigc/settings-entries.json` ----
    Case {
        name: "setup-settings-record-a-link",
        site: RECORD,
        cause: "`.jigc/settings-entries.json` is a link",
        state: &[
            Step::File(Place::Outside("theirs"), "{}\n"),
            Step::Link(Place::Repo(SETTINGS_RECORD), Place::Outside("theirs")),
        ],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(SETTINGS_RECORD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-settings-record-a-directory",
        site: RECORD,
        cause: "`.jigc/settings-entries.json` is a directory",
        state: &[Step::File(
            Place::Repo(".jigc/settings-entries.json/kept"),
            "kept\n",
        )],
        acts: MOVE_AWAY,
        holds: &[
            Check::RegularFile(Place::Repo(SETTINGS_RECORD)),
            Check::RegularFile(Place::Moved(".jigc/settings-entries.json/kept")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-settings-record-a-special-file",
        site: RECORD,
        cause: "`.jigc/settings-entries.json` is a FIFO",
        state: &[Step::Fifo(Place::Repo(SETTINGS_RECORD))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(SETTINGS_RECORD))],
        ..SETUP
    },
    // ---- `replaced_path_refusal` · `setup.write-guide` · `.claude/skills/jigc/SKILL.md` ----
    Case {
        name: "setup-guide-under-a-linked-assistant-directory",
        site: GUIDE,
        cause: "`.claude`, on the way to the guide, is a link to a directory outside the \
                repository that holds no guide",
        state: &[
            Step::File(Place::Outside("dotclaude/theirs"), "theirs\n"),
            Step::Link(Place::Repo(".claude"), Place::Outside("dotclaude")),
        ],
        acts: MAKE_DIRECTORY,
        holds: &[
            Check::Directory(Place::Repo(".claude")),
            Check::RegularFile(Place::Repo(".claude/skills/jigc/SKILL.md")),
        ],
        witnesses: &[
            "dotclaude/theirs",
            "dotclaude/settings.json",
            "dotclaude/skills/jigc/SKILL.md",
        ],
        ..SETUP
    },
    Case {
        name: "setup-guide-under-a-linked-skills-directory",
        site: GUIDE,
        cause: "`.claude/skills`, on the way to the guide, is a link to a directory outside the \
                repository",
        state: &[
            Step::File(Place::Outside("skills/theirs"), "theirs\n"),
            Step::Link(Place::Repo(".claude/skills"), Place::Outside("skills")),
        ],
        acts: MAKE_DIRECTORY,
        holds: &[
            Check::Directory(Place::Repo(".claude/skills")),
            Check::RegularFile(Place::Repo(".claude/skills/jigc/SKILL.md")),
        ],
        witnesses: &["skills/theirs", "skills/jigc/SKILL.md"],
        ..SETUP
    },
    Case {
        name: "setup-guide-under-a-linked-skill-directory",
        site: GUIDE,
        cause: "`.claude/skills/jigc`, on the way to the guide, is a link to a directory \
                outside the repository",
        state: &[
            Step::File(Place::Outside("skill/theirs"), "theirs\n"),
            Step::Link(Place::Repo(".claude/skills/jigc"), Place::Outside("skill")),
        ],
        acts: MAKE_DIRECTORY,
        holds: &[
            Check::Directory(Place::Repo(".claude/skills/jigc")),
            Check::RegularFile(Place::Repo(".claude/skills/jigc/SKILL.md")),
        ],
        witnesses: &["skill/theirs", "skill/SKILL.md"],
        ..SETUP
    },
    // ---- `merged_through_links` · `setup.inject-reference` · `CLAUDE.md` ----
    Case {
        name: "setup-host-a-link-to-nothing",
        site: HOST,
        cause: "`CLAUDE.md` is a link to a path that holds nothing",
        state: &[Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("nothing"))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(CLAUDE_MD))],
        witnesses: &["nothing"],
        ..SETUP
    },
    Case {
        name: "setup-host-a-link-out-of-the-repository",
        site: HOST,
        cause: HOST_LEADS_OUT,
        state: &[THEIRS, Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("theirs"))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(CLAUDE_MD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-host-a-committed-link-out-of-the-repository",
        site: HOST,
        cause: HOST_LEADS_OUT,
        state: &[
            THEIRS,
            Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("theirs")),
            COMMIT_IT[0],
            COMMIT_IT[1],
        ],
        acts: REMOVE_AND_COMMIT,
        holds: &[Check::RegularFile(Place::Repo(CLAUDE_MD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-host-link-pointed-at-a-committed-file",
        site: HOST,
        cause: HOST_LEADS_OUT,
        state: &[THEIRS, Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("theirs"))],
        acts: POINT_AT_COMMITTED,
        holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-host-link-pointed-at-a-new-file",
        site: HOST,
        cause: HOST_LEADS_OUT,
        state: &[THEIRS, Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("theirs"))],
        acts: POINT_AT_NEW,
        holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-host-committed-link-pointed-at-a-committed-file",
        site: HOST,
        cause: HOST_LEADS_OUT,
        state: &[
            THEIRS,
            Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("theirs")),
            COMMIT_IT[0],
            COMMIT_IT[1],
        ],
        acts: POINT_AT_COMMITTED_AND_COMMIT,
        holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-host-committed-link-pointed-at-a-new-file",
        site: HOST,
        cause: HOST_LEADS_OUT,
        state: &[
            THEIRS,
            Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("theirs")),
            COMMIT_IT[0],
            COMMIT_IT[1],
        ],
        acts: POINT_AT_NEW_AND_COMMIT,
        holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
        witnesses: &["theirs"],
        doubt: Some(WHICH_FILE),
        ..SETUP
    },
    Case {
        name: "setup-host-a-link-to-a-directory",
        site: HOST,
        cause: "`CLAUDE.md` is a link to a directory inside the repository",
        state: &[
            Step::Dir(Place::Repo("docs")),
            Step::Link(Place::Repo(CLAUDE_MD), Place::Repo("docs")),
        ],
        acts: REMOVE,
        holds: &[
            Check::RegularFile(Place::Repo(CLAUDE_MD)),
            Check::Directory(Place::Repo("docs")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-host-a-link-into-gits-directory",
        site: HOST,
        cause: "`CLAUDE.md` is a link to a file inside git's own directory",
        state: &[
            Step::File(Place::Repo(".git/theirs"), "theirs\n"),
            Step::Link(Place::Repo(CLAUDE_MD), Place::Repo(".git/theirs")),
        ],
        acts: REMOVE,
        holds: &[
            Check::RegularFile(Place::Repo(CLAUDE_MD)),
            Check::RegularFile(Place::Repo(".git/theirs")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-two-merged-files-links",
        site: HOST,
        cause: "two merged-into files are links the install will not follow at once — \
                `CLAUDE.md` and `.claude/settings.json` — and the route joins their acts",
        state: &[
            Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("nothing")),
            Step::Link(Place::Repo(SETTINGS_JSON), Place::Outside("nothing-either")),
        ],
        acts: REMOVE_BOTH,
        holds: &[
            Check::RegularFile(Place::Repo(CLAUDE_MD)),
            Check::RegularFile(Place::Repo(SETTINGS_JSON)),
        ],
        witnesses: &["nothing", "nothing-either"],
        ..SETUP
    },
    // ---- `merged_through_links` · `setup.inject-allowlist` · `.claude/settings.json` ----
    Case {
        name: "setup-settings-a-link-to-nothing",
        site: SETTINGS,
        cause: "`.claude/settings.json` is a link to a path that holds nothing",
        state: &[Step::Link(Place::Repo(SETTINGS_JSON), Place::Outside("nothing"))],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(SETTINGS_JSON))],
        witnesses: &["nothing"],
        ..SETUP
    },
    Case {
        name: "setup-settings-a-link-out-of-the-repository",
        site: SETTINGS,
        cause: SETTINGS_LEADS_OUT,
        state: &[
            Step::File(Place::Outside("theirs"), "{}\n"),
            Step::Link(Place::Repo(SETTINGS_JSON), Place::Outside("theirs")),
        ],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(SETTINGS_JSON))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-settings-link-pointed-at-a-new-file",
        site: SETTINGS,
        cause: SETTINGS_LEADS_OUT,
        state: &[
            Step::File(Place::Outside("theirs"), "{}\n"),
            Step::Link(Place::Repo(SETTINGS_JSON), Place::Outside("theirs")),
        ],
        acts: POINT_AT_NEW,
        holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-settings-a-link-to-a-directory",
        site: SETTINGS,
        cause: "`.claude/settings.json` is a link to a directory inside the repository",
        state: &[
            Step::Dir(Place::Repo("conf")),
            Step::Link(Place::Repo(SETTINGS_JSON), Place::Repo("conf")),
        ],
        acts: REMOVE,
        holds: &[
            Check::RegularFile(Place::Repo(SETTINGS_JSON)),
            Check::Directory(Place::Repo("conf")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-settings-a-link-into-gits-directory",
        site: SETTINGS,
        cause: "`.claude/settings.json` is a link to a file inside git's own directory",
        state: &[
            Step::File(Place::Repo(".git/theirs"), "{}\n"),
            Step::Link(Place::Repo(SETTINGS_JSON), Place::Repo(".git/theirs")),
        ],
        acts: REMOVE,
        holds: &[
            Check::RegularFile(Place::Repo(SETTINGS_JSON)),
            Check::RegularFile(Place::Repo(".git/theirs")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-settings-under-a-link-that-holds-none",
        site: SETTINGS,
        cause: "`.claude`, on the way to `.claude/settings.json`, is a link to a directory \
                outside the repository that holds a guide of somebody's and no settings file",
        state: &[
            Step::File(Place::Outside("dotclaude/skills/jigc/SKILL.md"), "theirs\n"),
            Step::Link(Place::Repo(".claude"), Place::Outside("dotclaude")),
        ],
        acts: MAKE_DIRECTORY,
        holds: &[
            Check::Directory(Place::Repo(".claude")),
            Check::RegularFile(Place::Repo(SETTINGS_JSON)),
        ],
        witnesses: &["dotclaude/skills/jigc/SKILL.md", "dotclaude/settings.json"],
        ..SETUP
    },
    Case {
        name: "setup-settings-under-a-link-out-of-the-repository",
        site: SETTINGS,
        cause: "`.claude`, on the way to `.claude/settings.json`, is a link to a directory \
                outside the repository that holds a guide of somebody's and a settings file",
        state: &[
            Step::File(Place::Outside("dotclaude/skills/jigc/SKILL.md"), "theirs\n"),
            Step::File(Place::Outside("dotclaude/settings.json"), "{}\n"),
            Step::Link(Place::Repo(".claude"), Place::Outside("dotclaude")),
        ],
        acts: MAKE_DIRECTORY,
        holds: &[
            Check::Directory(Place::Repo(".claude")),
            Check::RegularFile(Place::Repo(SETTINGS_JSON)),
        ],
        witnesses: &["dotclaude/skills/jigc/SKILL.md", "dotclaude/settings.json"],
        ..SETUP
    },
    Case {
        name: "setup-settings-under-a-link-to-a-directory-there",
        site: SETTINGS,
        cause: "`.claude`, on the way to `.claude/settings.json`, is a link to a directory \
                inside the repository where `settings.json` is a directory",
        state: &[
            Step::File(Place::Repo("dotclaude/skills/jigc/SKILL.md"), "theirs\n"),
            Step::Dir(Place::Repo("dotclaude/settings.json")),
            Step::Link(Place::Repo(".claude"), Place::Repo("dotclaude")),
        ],
        acts: MAKE_DIRECTORY,
        holds: &[
            Check::Directory(Place::Repo(".claude")),
            Check::RegularFile(Place::Repo(SETTINGS_JSON)),
            Check::RegularFile(Place::Repo("dotclaude/skills/jigc/SKILL.md")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-settings-under-a-link-into-gits-directory",
        site: SETTINGS,
        cause: "`.claude`, on the way to `.claude/settings.json`, is a link to a directory \
                inside git's own directory",
        state: &[
            Step::File(Place::Repo(".git/dotclaude/skills/jigc/SKILL.md"), "theirs\n"),
            Step::File(Place::Repo(".git/dotclaude/settings.json"), "{}\n"),
            Step::Link(Place::Repo(".claude"), Place::Repo(".git/dotclaude")),
        ],
        acts: MAKE_DIRECTORY,
        holds: &[
            Check::Directory(Place::Repo(".claude")),
            Check::RegularFile(Place::Repo(SETTINGS_JSON)),
            Check::RegularFile(Place::Repo(".git/dotclaude/settings.json")),
        ],
        ..SETUP
    },
    // ---- `merged_through_links` · `setup.secrets-gitignore` · the root `.gitignore` ----
    Case {
        name: "setup-seeded-ignore-a-link-to-nothing",
        site: SEED,
        cause: "on an unborn `HEAD`, the root `.gitignore` is a link to a path that holds \
                nothing",
        state: &[
            UNBORN,
            Step::Link(Place::Repo(ROOT_GITIGNORE), Place::Outside("nothing")),
        ],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(ROOT_GITIGNORE))],
        witnesses: &["nothing"],
        ..SETUP
    },
    Case {
        name: "setup-seeded-ignore-a-link-out-of-the-repository",
        site: SEED,
        cause: SEED_LEADS_OUT,
        state: &[
            UNBORN,
            THEIRS,
            Step::Link(Place::Repo(ROOT_GITIGNORE), Place::Outside("theirs")),
        ],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(ROOT_GITIGNORE))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-seeded-ignore-link-pointed-at-a-new-file",
        site: SEED,
        cause: SEED_LEADS_OUT,
        state: &[
            UNBORN,
            THEIRS,
            Step::Link(Place::Repo(ROOT_GITIGNORE), Place::Outside("theirs")),
        ],
        acts: POINT_AT_NEW,
        holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
        witnesses: &["theirs"],
        ..SETUP
    },
    Case {
        name: "setup-seeded-ignore-a-link-to-a-directory",
        site: SEED,
        cause: "on an unborn `HEAD`, the root `.gitignore` is a link to a directory inside \
                the repository",
        state: &[
            UNBORN,
            Step::Dir(Place::Repo("docs")),
            Step::Link(Place::Repo(ROOT_GITIGNORE), Place::Repo("docs")),
        ],
        acts: REMOVE,
        holds: &[
            Check::RegularFile(Place::Repo(ROOT_GITIGNORE)),
            Check::Directory(Place::Repo("docs")),
        ],
        ..SETUP
    },
    Case {
        name: "setup-seeded-ignore-a-link-into-gits-directory",
        site: SEED,
        cause: "on an unborn `HEAD`, the root `.gitignore` is a link to a file inside git's \
                own directory",
        state: &[
            UNBORN,
            Step::File(Place::Repo(".git/theirs"), "theirs\n"),
            Step::Link(Place::Repo(ROOT_GITIGNORE), Place::Repo(".git/theirs")),
        ],
        acts: REMOVE,
        holds: &[
            Check::RegularFile(Place::Repo(ROOT_GITIGNORE)),
            Check::RegularFile(Place::Repo(".git/theirs")),
        ],
        ..SETUP
    },
    // ---- `merged_through_links` · `setup.init-project-layer` · `.jigc/.gitignore` ----
    Case {
        name: "setup-workbench-ignore-a-link",
        site: IGNORE,
        cause: "`.jigc/.gitignore` is a link",
        state: &[
            THEIRS,
            Step::Link(Place::Repo(JIGC_GITIGNORE), Place::Outside("theirs")),
        ],
        acts: REMOVE,
        holds: &[Check::RegularFile(Place::Repo(JIGC_GITIGNORE))],
        witnesses: &["theirs"],
        ..SETUP
    },
];

/// The sites of `jigc setup` that have no case, each with why.
pub const REASONS: &[Reason] = &[];

/// What both callees share: the question each asks of a member's path, and the table whose
/// order decides the code.
const SHARED: [&str; 4] = [
    "crates/cli/src/setup.rs::install — the order of the three refusals before the first write",
    "crates/cli/src/setup.rs::install_tracked_paths — the members, their writers and their order",
    "crates/cli/src/regular_file.rs::blocker and Blocker::clearing_act — what is in the way, and the act a route names for it",
    "crates/engine/src/store.rs::home_entry — the three shapes of an entry that is no regular file",
];

/// Why a link at `.jigc` is no case of a site but the first replacing member's.
const WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES: Why = Why::NotConstructible(
    "a link at `.jigc` is in the way of `.jigc/AGENT.md` first, and the refusal is raised \
     under the first blocked member's code — `setup.write-bootstrap`, the case \
     `setup-workbench-a-link`",
);

/// **What was read for each site, and how many causes it holds** — a hand count, with its
/// derivation; and the causes that have no case, each with why.
///
/// The count is of ways through the callee, as this file's head says. One thing it leaves
/// to one site a function: **several entries in the way at once**, which either callee
/// answers with one finding under the first member's code and one route that joins the
/// acts. The join is the same code whichever member is first, so it is counted — and
/// followed — at the first member of each function (`.jigc/AGENT.md`, `CLAUDE.md`) and at no
/// other site.
pub const READINGS: &[Reading] = &[
    Reading {
        site: HOOK_LINK_SITE,
        read: &["crates/cli/src/setup.rs::hook_link_refusal"],
        causes: 2,
        derivation: "the two arms of its `canonicalize`: the link leads to nothing, or leads \
                     out of the repository and its hooks directory. `display_hook_path`, \
                     which words the path the route names, and `resolve_hooks_dir` were not \
                     read for this count: how the link is spelled, what stands at its end \
                     and where `core.hooksPath` puts the hooks directory are states inside \
                     the second arm, and none has a case of its own",
        without_case: &[],
    },
    Reading {
        site: BOOTSTRAP,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 5,
        derivation: "`.jigc/AGENT.md` is the first replacing member: one directory on its way \
                     can be a link (`.jigc`), its own path can be a link, a directory or a \
                     special file, and — counted here for the function — several entries can \
                     be in the way at once. Where a link leads is not asked",
        without_case: &[],
    },
    Reading {
        site: STAMP,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 4,
        derivation: "`.jigc/version`: the directory on its way (`.jigc`), and its own path as \
                     a link, a directory or a special file",
        without_case: &[(
            "`.jigc`, on the way to `.jigc/version`, is a link",
            WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES,
        )],
    },
    Reading {
        site: LAYER,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 5,
        derivation: "`.jigc/config/.gitkeep`: two directories on its way (`.jigc`, \
                     `.jigc/config`), and its own path as a link, a directory or a special \
                     file",
        without_case: &[(
            "`.jigc`, on the way to `.jigc/config/.gitkeep`, is a link",
            WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES,
        )],
    },
    Reading {
        site: MARKER,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 5,
        derivation: "`.jigc/config/packs.yaml`: two directories on its way (`.jigc`, \
                     `.jigc/config`), and its own path as a link, a directory or a special \
                     file",
        without_case: &[
            (
                "`.jigc`, on the way to `.jigc/config/packs.yaml`, is a link",
                WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES,
            ),
            (
                "`.jigc/config`, on the way to `.jigc/config/packs.yaml`, is a link",
                Why::NotConstructible(
                    "a link at `.jigc/config` is in the way of `.jigc/config/.gitkeep` \
                     first, the member written before this one — the refusal is raised \
                     under `setup.init-project-layer`, the case `setup-project-layer-a-link`",
                ),
            ),
            (
                "`.jigc/config/packs.yaml` is a special file",
                Why::NotConstructible(
                    "the one special file the frame plants is a FIFO, and `jigc setup` \
                     reads this path before it reaches the refusal — a read that does not \
                     return. Written as a case it came out *not provoked*: the command had \
                     printed nothing when the runner ended it at its limit of two minutes, \
                     which is two minutes of every run of the suite, so it is no case",
                ),
            ),
        ],
    },
    Reading {
        site: RECORD,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 4,
        derivation: "`.jigc/settings-entries.json`: the directory on its way (`.jigc`), and \
                     its own path as a link, a directory or a special file",
        without_case: &[(
            "`.jigc`, on the way to `.jigc/settings-entries.json`, is a link",
            WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES,
        )],
    },
    Reading {
        site: GUIDE,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            "crates/cli/src/setup.rs::guide_ownership — asked of the guide's path before the \
             member table is built; a copy that is not jigc's takes the guide out of the table",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 6,
        derivation: "`.claude/skills/jigc/SKILL.md`: three directories on its way (`.claude`, \
                     `.claude/skills`, `.claude/skills/jigc`), and its own path as a link, a \
                     directory or a special file",
        without_case: &[
            (
                "the guide's own path is a link",
                Why::NotConstructible(
                    "`guide_ownership` asks the entry first and reads a link as a copy that \
                     is not jigc's: the guide leaves the member table, `jigc setup` exits 0 \
                     and says so under the advisory `adapter-guide.user-modified` — driven, \
                     the link's end untouched",
                ),
            ),
            (
                "the guide's own path is a directory",
                Why::NotConstructible(
                    "`guide_ownership` cannot read a directory as jigc's text, so the guide \
                     leaves the member table as for a link — driven, exit 0",
                ),
            ),
            (
                "the guide's own path is a special file",
                Why::NotConstructible(
                    "`guide_ownership` reads the entry before this refusal is reached, and \
                     the read of a FIFO does not return: driven by hand, `jigc setup` \
                     printed nothing for ten seconds and ended only when the FIFO was opened \
                     for writing. A case would cost every run of the suite the runner's \
                     limit, so there is none",
                ),
            ),
        ],
    },
    Reading {
        site: HOST,
        read: &[
            "crates/cli/src/setup.rs::merged_through_links",
            "crates/cli/src/setup.rs::link_end — the four ends a followed link is refused at",
            SHARED[0],
            SHARED[1],
            SHARED[2],
        ],
        causes: 5,
        derivation: "`CLAUDE.md` stands at the repository root, so only its own path can be \
                     a link; `link_end` refuses four ends — nothing, out of the checkout, no \
                     regular file, git's own directory — and, counted here for the function, \
                     several links can be refused at once. An entry there that is no link is \
                     not this function's",
        without_case: &[],
    },
    Reading {
        site: SETTINGS,
        read: &[
            "crates/cli/src/setup.rs::merged_through_links",
            "crates/cli/src/setup.rs::link_end — the four ends a followed link is refused at",
            "crates/cli/src/setup.rs::guide_ownership — what keeps `replaced_path_refusal` \
             from answering for a link at `.claude` first",
            SHARED[0],
            SHARED[1],
            SHARED[2],
        ],
        causes: 8,
        derivation: "`.claude/settings.json`: the link is at its own path or at `.claude`, \
                     the one directory on its way, and either is refused at each of \
                     `link_end`'s four ends",
        without_case: &[],
    },
    Reading {
        site: SEED,
        read: &[
            "crates/cli/src/setup.rs::merged_through_links",
            "crates/cli/src/setup.rs::link_end — the four ends a followed link is refused at",
            "crates/cli/src/setup.rs::is_fresh_repo — the root `.gitignore` is a member on an \
             unborn `HEAD` only",
            SHARED[0],
            SHARED[1],
            SHARED[2],
        ],
        causes: 4,
        derivation: "the root `.gitignore`, a member where `HEAD` is unborn: only its own \
                     path can be a link, refused at each of `link_end`'s four ends",
        without_case: &[],
    },
    Reading {
        site: IGNORE,
        read: &[
            "crates/cli/src/setup.rs::merged_through_links",
            SHARED[0],
            SHARED[1],
            SHARED[2],
        ],
        causes: 2,
        derivation: "`.jigc/.gitignore` is the one member a link is never followed at, so \
                     `link_end` is not asked: the link is at its own path, wherever it \
                     leads, or at `.jigc`, the directory on its way",
        without_case: &[(
            "`.jigc`, on the way to `.jigc/.gitignore`, is a link",
            Why::NotConstructible(
                "`replaced_path_refusal` is asked before this function and answers for a \
                 link at `.jigc` under `setup.write-bootstrap` — the case \
                 `setup-workbench-a-link`",
            ),
        )],
    },
];
