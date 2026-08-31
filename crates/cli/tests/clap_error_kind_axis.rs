//! **The clap error-kind axis** — jigc's own disposition of every `clap::error::ErrorKind`
//! (M49 Inc 11 T5; charter Tier 1 **S-4**; `design/surface-contract.md` → law 2, nothing
//! hides).
//!
//! Two of jigc's surfaces are rendered by clap, not by jigc: a usage error, and the
//! `--help`/`--version` arms. `main.rs` used to state the seam's coverage in prose —
//! *"Every other clap error kind still prints clap's own render"* — and that sentence was
//! the charter's own evidence that a fence built at M46/M48 had been applied at the sites
//! those waves named and nowhere else. This suite replaces the sentence with a table.
//!
//! **Why the table is jigc's own, and what "complete" can mean here.** `ErrorKind` is
//! `#[non_exhaustive]` and exposes no `all()`, so the axis **cannot be a `match` and
//! cannot be derived from clap**: no code in this repo can enumerate the variants, and a
//! clap upgrade that adds one cannot redden a `match` that does not exist. So the
//! completeness proof is **reachability by driving**, not exhaustiveness:
//!
//! * every kind a `jigc` argv can produce carries that argv, and the argv is **driven
//!   through the real binary** — the emitted bytes are asserted against the row's
//!   disposition, never against a reconstruction;
//! * every kind no `jigc` argv can produce carries the reason it cannot, and — where that
//!   reason is a property of jigc's **own** argument definitions rather than of clap —
//!   the reason is fenced against the clap tree, so the row cannot rot silently;
//! * the two rows whose reason is neither (`Io`, `Format`) say out loud that they carry no
//!   fence, and why;
//! * and because the variant *count* is unverifiable from inside, the row set is pinned to
//!   the `clap_builder` version it was derived against — an upgrade reddens
//!   [`the_table_is_pinned_to_the_clap_it_was_derived_against`] and the table is re-derived.
//!
//! **S-4, the finding this suite ships with.** `jigc rename <type>:<slug> --to X --task
//! <id>` drew clap's *"tip: to pass `--task` as a value, use `-- --task`"* and named `jigc
//! doc rename` nowhere — a misdirection on precisely the confusable pair M48's identity
//! split is built around, at a node whose only positional is already filled and where no
//! value form of `--task` exists at all.

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use cli::cli::{
    Cli, FORECLOSED_ARGUMENT_TIPS, ForeclosedArgument, unexpected_argument_block,
    unknown_subcommand_block,
};
use cli::task::{EXIT_SUCCESS, EXIT_USAGE};
use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The `clap_builder` release [`KIND_DISPOSITIONS`] was derived against. The row set is
/// hand-written because the enum cannot be enumerated; this pin is what stops it going
/// stale in silence.
const DERIVED_AGAINST_CLAP_BUILDER: &str = "4.6.0";

/// The number of `ErrorKind` variants that release declares — counted by reading
/// `clap_builder-4.6.0/src/error/kind.rs`, since neither `match` nor `all()` can count
/// them from here.
///
/// **This count was 16 in the increment's planning basis and is 17.** `InvalidUtf8` sits
/// between `MissingSubcommand` and `DisplayHelp` and was missed; deriving the axis instead
/// of inheriting the number is what found it — and the kind turned out to be the one jigc
/// handled worst (see its row).
const ERROR_KIND_VARIANTS: usize = 17;

/// The one probe token that is **not** valid UTF-8, written as a marker because the
/// table's tokens are `&str` and this one by construction is not a string:
/// [`os_argv`] substitutes the raw byte `0xFF`, which is what reaches the process.
const NON_UTF8_TOKEN: &str = "<non-utf-8-byte>";

/// What jigc does with a clap error of a given kind.
#[derive(Debug, PartialEq, Eq)]
enum Disposition {
    /// jigc composes the block itself from the error's own context — the takeover.
    JigcRenders,
    /// clap's own render stands, byte for byte. *Covered* is not *taken over*: a kind
    /// whose clap render already says what the reader needs is disposed by saying so.
    ClapStands,
    /// No `jigc` argv produces this kind.
    NotProducible,
}

/// One row of the axis: a kind, the argv that reaches it (empty when nothing does), what
/// jigc does with it, and why.
struct KindRow {
    kind: ErrorKind,
    /// The argv **after** the binary name, driven through the real binary. Empty for a
    /// [`Disposition::NotProducible`] row, which by definition has none.
    probe: &'static [&'static str],
    disposition: Disposition,
    reason: &'static str,
}

/// **The disposition table** — one row per `clap::error::ErrorKind`, in the enum's own
/// declaration order.
const KIND_DISPOSITIONS: &[KindRow] = &[
    KindRow {
        kind: ErrorKind::InvalidValue,
        probe: &["--format", "bogus", "describe"],
        disposition: Disposition::ClapStands,
        reason: "clap names the argument, the offending value AND the full possible-value \
                 set — the three things a rejected enum value needs, and the set is the \
                 route. jigc knows nothing clap does not.",
    },
    KindRow {
        kind: ErrorKind::UnknownArgument,
        probe: &["rename", "adr:x", "--to", "X", "--task", "t1"],
        disposition: Disposition::JigcRenders,
        reason: "Taken over where a curated (node, argument) row records the answer — the \
                 foreclosed positional at `jigc describe` (M46 Inc 8 T5) and the \
                 foreclosed `--task` at `jigc rename` (S-4). Every other rejected \
                 argument keeps clap's render: the takeover replaces a misdirection, it \
                 does not run globally.",
    },
    KindRow {
        kind: ErrorKind::InvalidSubcommand,
        probe: &["task", "discard-write", "docs/adr.md"],
        disposition: Disposition::JigcRenders,
        reason: "Taken over whole (M48 Inc 6 T2): clap emits its block before jigc can \
                 speak, so a curated tip could only print BELOW the suggestion it \
                 contradicts. The tip slot carries the curated tip, the parent's read \
                 answer, or clap's own did-you-mean — never two of them.",
    },
    KindRow {
        kind: ErrorKind::NoEquals,
        probe: &[],
        disposition: Disposition::NotProducible,
        reason: "Fires only for an argument declared `require_equals`. jigc declares none \
                 — fenced over the clap tree by \
                 `the_not_producible_kinds_are_fenced_over_the_clap_tree`.",
    },
    KindRow {
        kind: ErrorKind::ValueValidation,
        probe: &[],
        disposition: Disposition::NotProducible,
        reason: "Fires only when a `TypedValueParser` rejects a value — a \
                 `value_parser!(u32)` or a hand-written one. Every jigc value-taking \
                 argument is one of clap's four built-in parsers, whose only failures are \
                 `InvalidValue` (an empty `PathBuf`) and `InvalidUtf8` (a `String`), or a \
                 `ValueEnum` whose miss is `InvalidValue`. Fenced over the clap tree.",
    },
    KindRow {
        kind: ErrorKind::TooManyValues,
        probe: &[],
        disposition: Disposition::NotProducible,
        reason: "Fires only for an argument whose `num_args` upper bound exceeds one. No \
                 jigc argument takes more than one value — fenced over the clap tree.",
    },
    KindRow {
        kind: ErrorKind::TooFewValues,
        probe: &[],
        disposition: Disposition::NotProducible,
        reason: "Fires only for an argument whose `num_args` lower bound exceeds one. No \
                 jigc argument requires more than one value — fenced over the clap tree.",
    },
    KindRow {
        kind: ErrorKind::WrongNumberOfValues,
        probe: &[],
        disposition: Disposition::NotProducible,
        reason: "Fires only for an argument with an exact multi-value `num_args`. Same \
                 fence as its two siblings: every jigc argument's value range is 0..=0 \
                 (a flag) or 1..=1.",
    },
    KindRow {
        kind: ErrorKind::ArgumentConflict,
        probe: &["start", "an intent", "--task", "t1"],
        disposition: Disposition::ClapStands,
        reason: "clap names BOTH conflicting arguments and prints the usage. The conflict \
                 is declared in the argument definitions themselves (`conflicts_with`), \
                 and the recovery is to drop one of the two it just named.",
    },
    KindRow {
        kind: ErrorKind::MissingRequiredArgument,
        probe: &["doc", "set-field", "--value", "v"],
        disposition: Disposition::ClapStands,
        reason: "clap lists exactly the arguments that were not provided and the usage \
                 line showing where each goes — a complete route, in clap's own words.",
    },
    KindRow {
        kind: ErrorKind::MissingSubcommand,
        probe: &["--format", "json"],
        disposition: Disposition::ClapStands,
        reason: "clap names the node and enumerates its subcommands inline. The menu IS \
                 the route, and `jigc describe` is a tour of workflows and doc-types, not \
                 of the verb tree — it would not answer this.",
    },
    KindRow {
        kind: ErrorKind::InvalidUtf8,
        probe: &["doc", "show", NON_UTF8_TOKEN],
        disposition: Disposition::ClapStands,
        reason: "clap names the fault and prints the usage; jigc cannot render the \
                 offending bytes any better than clap declines to. It is disposed CLAP \
                 STANDS only since M49 T5, and only completely since T5's own fix was \
                 swept: TWO production sites read the process argv — `main`'s takeover \
                 render and the invocation-log record — and `std::env::args()` PANICS on \
                 a non-UTF-8 argument at either. Fixing the first left the kind at exit \
                 101 whenever the (documented, trial-default) `invocation-log` knob was \
                 ON, which is why this row is driven in BOTH fixture topologies and the \
                 class is fenced at the source.",
    },
    KindRow {
        kind: ErrorKind::DisplayHelp,
        probe: &["--help"],
        disposition: Disposition::ClapStands,
        reason: "Not an error: the requested help, on stdout at exit 0. jigc's own text \
                 IS this render's content (every `about`/`long_about` in the tree), so \
                 there is nothing to take over.",
    },
    KindRow {
        kind: ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
        probe: &["doc"],
        disposition: Disposition::ClapStands,
        reason: "A bare parent node prints its own full help on stderr at exit 2 — the \
                 same content as `--help`, which already lists every child with its \
                 one-liner. The catalog-truth pass (M43) is what makes those one-liners \
                 the route.",
    },
    KindRow {
        kind: ErrorKind::DisplayVersion,
        probe: &["--version"],
        disposition: Disposition::ClapStands,
        reason: "Not an error: the version string on stdout at exit 0, the same stamp the \
                 invocation log records as `binary_version`.",
    },
    KindRow {
        kind: ErrorKind::Io,
        probe: &[],
        disposition: Disposition::NotProducible,
        reason: "clap's parser never constructs it — in clap_builder the variant appears \
                 only in `use_stderr`'s match, never in an `Error::new`. It exists for \
                 library users building their own errors, and jigc constructs no \
                 `clap::Error`. NO FENCE: this is a property of clap's source, not of \
                 jigc's argument tree, so nothing here can hold it.",
    },
    KindRow {
        kind: ErrorKind::Format,
        probe: &[],
        disposition: Disposition::NotProducible,
        reason: "Same as `Io`, for the same reason and with the same absent fence: a \
                 formatting failure a library user raises, never a parse verdict.",
    },
];

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-clapkind-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(out.status.success(), "git {args:?} failed");
}

/// A real git repo with one commit — every probe below is a parse-time verdict, but the
/// binary is driven for real, so it runs somewhere real.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// A real git repo with one commit, `jigc setup` run over it, and the **invocation-log
/// knob ON** — the second fixture topology the axis is driven in.
///
/// The knob is the axis member [`init_repo`] cannot reach: the log wrapper runs after the
/// clap arm has emitted, reads the process argv for its record, and is resolved only
/// inside a jigc project layer. A row driven in a bare repo therefore never executes it.
fn setup_project_with_the_log_on(repo: &Path, home: &Path) {
    init_repo(repo);
    let out = jigc(repo, home, &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let out = jigc(repo, home, &["config", "set", "invocation-log", "true"]);
    assert!(
        out.status.success(),
        "`jigc config set invocation-log true` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The JSONL invocation-log records at `.jigc/logs/invocations.jsonl` (empty when absent).
fn log_records(repo: &Path) -> Vec<serde_json::Value> {
    let path = repo.join(".jigc").join("logs").join("invocations.jsonl");
    match fs::read_to_string(&path) {
        Ok(body) => body
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).expect("each log line is valid JSON"))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// A probe's tokens as the OS sees them, with [`NON_UTF8_TOKEN`] substituted by the raw
/// byte it stands for. Every argv below — parsed in-process and driven through the binary
/// alike — is built here, so the two see the same bytes.
fn os_argv(probe: &[&str]) -> Vec<OsString> {
    probe
        .iter()
        .map(|token| {
            if *token == NON_UTF8_TOKEN {
                OsString::from_vec(vec![0xFF])
            } else {
                OsString::from(*token)
            }
        })
        .collect()
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(os_argv(args))
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// The clap error a probe produces, parsed in-process — the step that proves the probe
/// reaches the kind its row claims. The driven binary shows the *bytes*; only the parse
/// can show the *kind*.
fn parse_error(probe: &[&str]) -> clap::Error {
    let argv: Vec<OsString> = std::iter::once(OsString::from("jigc"))
        .chain(os_argv(probe))
        .collect();
    Cli::try_parse_from(&argv).expect_err("a probe argv must not parse")
}

/// The argv `main` hands the takeover renderers: the process argv read lossily, exactly as
/// `main.rs` reads it.
fn lossy_argv(probe: &[&str]) -> Vec<String> {
    std::iter::once(OsString::from("jigc"))
        .chain(os_argv(probe))
        .map(|token| token.to_string_lossy().into_owned())
        .collect()
}

/// **One row per kind, and the row set is internally consistent.**
#[test]
fn the_table_carries_one_row_per_error_kind() {
    assert_eq!(
        KIND_DISPOSITIONS.len(),
        ERROR_KIND_VARIANTS,
        "the table carries one row per `ErrorKind` variant of clap_builder \
         {DERIVED_AGAINST_CLAP_BUILDER}",
    );
    for (at, row) in KIND_DISPOSITIONS.iter().enumerate() {
        assert!(
            KIND_DISPOSITIONS[..at]
                .iter()
                .all(|prior| prior.kind != row.kind),
            "`{:?}` is disposed twice",
            row.kind,
        );
        assert!(!row.reason.is_empty(), "`{:?}` carries no reason", row.kind,);
        assert_eq!(
            row.probe.is_empty(),
            row.disposition == Disposition::NotProducible,
            "`{:?}`: a producible kind carries the argv that produces it, and only a \
             not-producible one may carry none",
            row.kind,
        );
    }
}

/// **The pin.** The variant count above was read out of one clap release and cannot be
/// checked from inside the language, so the release itself is asserted: a `clap_builder`
/// upgrade reddens here, and whoever takes it re-derives the table rather than inheriting
/// a count nobody re-counted.
#[test]
fn the_table_is_pinned_to_the_clap_it_was_derived_against() {
    let lock = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("Cargo.lock");
    let text = fs::read_to_string(&lock).expect("read the workspace Cargo.lock");
    let locked: Vec<&str> = text
        .split("[[package]]")
        .filter(|block| block.contains("name = \"clap_builder\""))
        .filter_map(|block| {
            block
                .lines()
                .find_map(|line| line.trim().strip_prefix("version = "))
        })
        .map(|version| version.trim_matches('"'))
        .collect();
    assert_eq!(
        locked,
        vec![DERIVED_AGAINST_CLAP_BUILDER],
        "the disposition table was derived against clap_builder \
         {DERIVED_AGAINST_CLAP_BUILDER}; `ErrorKind` is #[non_exhaustive] with no `all()`, \
         so nothing else in this repo notices a variant arriving",
    );
}

/// Drive every producible row's argv through the real binary in `repo`, asserting the
/// emitted bytes against the row's disposition — never against a reconstruction of them.
///
/// Three assertions per row: the argv produces exactly the kind the row claims (in-process
/// parse — the only way to see a *kind*); the driven binary exits on the taxonomy's own
/// codes; and the emitted stream carries clap's own render byte for byte
/// ([`Disposition::ClapStands`]) or jigc's own block byte for byte
/// ([`Disposition::JigcRenders`]).
///
/// `observed` is handed each row with the exit code it was asserted at, so a caller can
/// check what its own fixture topology adds — the log-on arm below checks that the
/// invocation the row just made was recorded.
fn drive_every_producible_kind(repo: &Path, home: &Path, mut observed: impl FnMut(&KindRow, u8)) {
    let mut driven = 0;
    for row in KIND_DISPOSITIONS {
        if row.disposition == Disposition::NotProducible {
            continue;
        }
        driven += 1;
        let shown = format!("jigc {}", row.probe.join(" "));
        let err = parse_error(row.probe);
        assert_eq!(
            err.kind(),
            row.kind,
            "`{shown}` must produce `{:?}`, the kind its row disposes; got `{:?}`",
            row.kind,
            err.kind(),
        );

        let out = jigc(repo, home, row.probe);
        let (expected_code, emitted) = if err.use_stderr() {
            (EXIT_USAGE, String::from_utf8(out.stderr.clone()))
        } else {
            (EXIT_SUCCESS, String::from_utf8(out.stdout.clone()))
        };
        let emitted = emitted.expect("utf-8 output");
        assert_eq!(
            out.status.code(),
            Some(i32::from(expected_code)),
            "`{shown}` exits on the taxonomy's code for its stream; got:\n{emitted}",
        );

        let argv = lossy_argv(row.probe);
        match row.disposition {
            Disposition::ClapStands => {
                assert_eq!(
                    emitted,
                    err.render().to_string(),
                    "`{shown}` is disposed CLAP STANDS, so the emitted bytes are clap's \
                     own render",
                );
            }
            Disposition::JigcRenders => {
                let block = unknown_subcommand_block(&err, &argv)
                    .or_else(|| unexpected_argument_block(&err, &argv))
                    .unwrap_or_else(|| {
                        panic!("`{shown}` is disposed JIGC RENDERS, but no jigc block claims it")
                    });
                assert_eq!(
                    emitted, block,
                    "`{shown}` is disposed JIGC RENDERS, so the emitted bytes are jigc's \
                     own block",
                );
                assert_ne!(
                    emitted,
                    err.render().to_string(),
                    "`{shown}` is disposed JIGC RENDERS, so clap's render must NOT be \
                     what reached the reader",
                );
            }
            Disposition::NotProducible => unreachable!("filtered above"),
        }
        observed(row, expected_code);
    }
    assert!(
        driven >= 10,
        "the driven set must not silently shrink; drove {driven}",
    );
}

/// **Every producible row is reached by driving its argv through the real binary.**
#[test]
fn every_producible_kind_is_reached_by_driving_its_probe() {
    let repo = TempDir::new("drive");
    let home = TempDir::new("home");
    init_repo(repo.path());
    drive_every_producible_kind(repo.path(), home.path(), |_, _| {});
}

/// **The same axis, driven in the topology the trials actually run in — the invocation
/// log ON.**
///
/// The fixture is an axis of its own, and the table above was driven along one point of
/// it. Every clap-rejected argv reaches `main`'s log wrapper *after* the render, and that
/// wrapper reads the process argv for its record — a second read of the same bytes, at a
/// second site, executed only inside a jigc project layer with the knob on. So the
/// `InvalidUtf8` row's stated disposition (CLAP STANDS, exit 2) was asserted in the one
/// state where the second read cannot run: with the knob on, `std::env::args()` there
/// panicked the process at exit **101** *after* clap's render had already been emitted —
/// and not only for that argv, but for **any** invocation carrying a non-UTF-8 argument,
/// including ones that are not errors at all. The knob is documented, shipped, and ON in
/// every RC trial this project runs, so this is the dominant configuration, not an exotic
/// one.
///
/// Each row is checked to have been *logged*, which is what keeps this arm from passing
/// vacuously: if the knob failed to apply, the log path would never run and the arm would
/// assert the same thing twice.
#[test]
fn every_producible_kind_holds_with_the_invocation_log_on() {
    let repo = TempDir::new("logon");
    let home = TempDir::new("home");
    setup_project_with_the_log_on(repo.path(), home.path());

    let mut recorded = log_records(repo.path()).len();
    drive_every_producible_kind(repo.path(), home.path(), |row, code| {
        let records = log_records(repo.path());
        assert_eq!(
            records.len(),
            recorded + 1,
            "`jigc {}` must append exactly one invocation-log record — the log path is \
             what this arm exists to execute",
            row.probe.join(" "),
        );
        assert_eq!(
            records[records.len() - 1]["exit_code"].as_u64(),
            Some(u64::from(code)),
            "`jigc {}` records the exit code it actually exited on",
            row.probe.join(" "),
        );
        recorded = records.len();
    });
}

/// **The not-producible rows, fenced where a fence exists.** Four of the six are absent
/// because of jigc's OWN argument definitions, so the clap tree can hold them: a
/// `require_equals` argument, a multi-value `num_args`, or a fallible value parser would
/// each make its kind reachable, and each reddens here the day it lands. `Io` and
/// `Format` carry no fence and their rows say so.
#[test]
fn the_not_producible_kinds_are_fenced_over_the_clap_tree() {
    fn walk(cmd: &clap::Command, path: &str) {
        for arg in cmd.get_arguments() {
            let at = format!("{path} {}", arg.get_id());
            assert!(
                !arg.is_require_equals_set(),
                "`{at}` sets require_equals, which makes `ErrorKind::NoEquals` reachable",
            );
            if let Some(range) = arg.get_num_args() {
                assert!(
                    range.max_values() <= 1,
                    "`{at}` takes up to {} values, which makes `TooManyValues` / \
                     `WrongNumberOfValues` reachable",
                    range.max_values(),
                );
                assert!(
                    range.min_values() <= 1,
                    "`{at}` requires {} values, which makes `TooFewValues` reachable",
                    range.min_values(),
                );
            }
            // clap's four built-in parsers fail only with `InvalidValue` (an empty
            // `PathBuf`) or `InvalidUtf8` (a `String`) — both disposed by their own rows.
            // `ValueParser::other(..)` is the door `ValueValidation` comes through, and a
            // `ValueEnum` is the one `other` that cannot: its miss is `InvalidValue`, and
            // it is recognisable by declaring possible values.
            const INFALLIBLE_BEYOND_INVALID_VALUE: [&str; 4] = [
                "ValueParser::string",
                "ValueParser::os_string",
                "ValueParser::path_buf",
                "ValueParser::bool",
            ];
            if arg.get_possible_values().is_empty() {
                let parser = format!("{:?}", arg.get_value_parser());
                assert!(
                    INFALLIBLE_BEYOND_INVALID_VALUE.contains(&parser.as_str()),
                    "`{at}` parses values with `{parser}`, which can fail with \
                     `ErrorKind::ValueValidation`",
                );
            }
        }
        for sub in cmd.get_subcommands() {
            walk(sub, &format!("{path} {}", sub.get_name()));
        }
    }
    walk(&Cli::command(), "jigc");
}

/// **S-4** — the identity pair stops misdirecting.
///
/// `jigc rename <type:slug> --to X --task <id>` drew clap's *"tip: to pass `--task` as a
/// value, use `-- --task`"*: advice that cannot help, at a node whose single positional is
/// already filled and which declares no `--task` in any form. The answer is on record —
/// `jigc doc rename` is defined as *"the in-task sibling of the top-level `jigc rename`
/// (which is task-less and self-committing…)"* — and this is the surface that produces the
/// state, so law 2 puts it here.
#[test]
fn the_rename_identity_pair_names_its_in_task_sibling() {
    let repo = TempDir::new("s4");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["rename", "adr:x", "--to", "X", "--task", "t1"],
    );
    assert_eq!(
        out.status.code(),
        Some(i32::from(EXIT_USAGE)),
        "the foreclosed `--task` stays a usage error",
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains("error: unexpected argument '--task' found"),
        "clap's own error line is preserved; got:\n{stderr}",
    );
    assert!(
        stderr.contains("`jigc doc rename"),
        "the tip names the in-task sibling; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("-- --task"),
        "clap's misdirecting value tip is REPLACED, not printed above the answer; \
         got:\n{stderr}",
    );
    assert!(
        stderr.contains("Usage: jigc rename"),
        "the node's own usage is preserved; got:\n{stderr}",
    );
}

/// **The curated table is the axis** — every [`FORECLOSED_ARGUMENT_TIPS`] row is driven
/// through the real binary at its own probe argv, and the **emitted** bytes carry the
/// row's own tip and none of clap's advice for that argument. Iterated from the table, so
/// a third row joins the fence the day it lands; the unit test beside the table checks its
/// structural legs, and this one checks what a reader actually receives.
#[test]
fn every_foreclosed_argument_row_is_driven_at_its_own_probe() {
    let repo = TempDir::new("foreclosed");
    let home = TempDir::new("home");
    init_repo(repo.path());

    assert!(
        !FORECLOSED_ARGUMENT_TIPS.is_empty(),
        "the curated table must carry rows, or this arm asserts nothing",
    );
    for row in FORECLOSED_ARGUMENT_TIPS {
        let shown = format!("jigc {}", row.probe.join(" "));
        let out = jigc(repo.path(), home.path(), row.probe);
        assert_eq!(
            out.status.code(),
            Some(i32::from(EXIT_USAGE)),
            "`{shown}` names the answer without building the form — it stays a usage error",
        );
        let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
        assert!(
            stderr.contains(&(row.tip)()),
            "`{shown}` must emit the row's own tip verbatim; got:\n{stderr}",
        );
        assert!(
            stderr.contains(&format!("Usage: jigc {}", row.node.join(" "))),
            "`{shown}` preserves the node's own usage; got:\n{stderr}",
        );
        if let ForeclosedArgument::Flag(flag) = row.arg {
            assert!(
                !stderr.contains(&format!("-- {flag}")),
                "`{shown}` must REPLACE clap's `-- {flag}` value advice, not print the \
                 answer beneath it; got:\n{stderr}",
            );
        }
    }
}

/// **The omitting context** — the takeover is keyed on (node, argument), so a `--task`
/// typed at a node with no curated row keeps clap's own render, misdirecting tip and all.
/// Widening the takeover to every rejected flag would put jigc in the business of
/// answering arguments it has recorded no answer for; the row set is the scope.
#[test]
fn a_foreclosed_flag_without_a_curated_row_keeps_claps_render() {
    let repo = TempDir::new("omitting");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let probe = ["task", "validate", "--task"];
    let out = jigc(repo.path(), home.path(), &probe);
    assert_eq!(
        out.status.code(),
        Some(i32::from(EXIT_USAGE)),
        "an uncurated rejected flag stays a usage error",
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert_eq!(
        stderr,
        parse_error(&probe).render().to_string(),
        "an uncurated node keeps clap's render byte for byte",
    );
    assert!(
        !stderr.contains("jigc doc rename"),
        "`rename`'s curated answer must not fire under another node; got:\n{stderr}",
    );
}
/// **The class fence: no production code reads the process argv as `String`s.**
///
/// `std::env::args()` panics on an argument that is not valid UTF-8. The two production
/// reads of the process argv — `main`'s takeover render and the invocation log's record —
/// were fixed one at a time, the second surviving the first by a whole increment, because
/// each fix was aimed at the *site* the repro named rather than the *class*
/// ([dev-workflow.md](../../../implementation/dev-workflow.md) → *a fix is complete over
/// its class's axis*). The class is: **every production read of the process argv**, and it
/// has exactly one safe form.
///
/// So the property is checked where membership is decided — the source itself — rather
/// than by a driven probe per reader, because a third reader added tomorrow would be
/// driven by nothing. Test-domain code is out of scope: a test's own argv is its own.
#[test]
fn no_production_code_reads_the_process_argv_as_strings() {
    /// The panicking form. `std::env::args_os()` does not match it — the `(` is required
    /// immediately after `args`.
    const BANNED: &str = "env::args(";
    /// The safe form, which the sweep must keep finding, or it is inspecting nothing.
    const SAFE: &str = "env::args_os(";

    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize the workspace root");
    let sources: Vec<PathBuf> = ["crates/engine", "crates/cli"]
        .iter()
        .flat_map(|krate| crate::support::rust_source::rust_files(&root.join(krate)))
        .filter(|path| !path.components().any(|c| c.as_os_str() == "probes"))
        .collect();
    assert!(
        sources.len() > 100,
        "the sweep must find the workspace sources; found only {}",
        sources.len(),
    );

    let mut offenders = Vec::new();
    let mut safe_reads = 0usize;
    for path in &sources {
        let body = fs::read_to_string(path).expect("read a workspace source");
        // Comments and string literals are blanked, so `main.rs`'s own prose about the
        // banned form — and this suite's constants — are not offenders.
        let code = crate::support::rust_source::code_only(&body);
        let regions = crate::support::rust_source::cfg_test_regions(&code);
        for (at, _) in code.match_indices(SAFE) {
            if !crate::support::rust_source::is_test_domain(path, &regions, at) {
                safe_reads += 1;
            }
        }
        for (at, _) in code.match_indices(BANNED) {
            if crate::support::rust_source::is_test_domain(path, &regions, at) {
                continue;
            }
            let rel = path.strip_prefix(&root).unwrap_or(path).display();
            offenders.push(format!("  {rel}:{}", code[..at].lines().count()));
        }
    }

    assert!(
        offenders.is_empty(),
        "production code must read the process argv with `{SAFE}` and convert lossily: \
         `{BANNED}` PANICS the process (exit 101) on an argument that is not valid \
         UTF-8, and every jigc invocation reaches both readers.\n{} offending site(s):\n{}",
        offenders.len(),
        offenders.join("\n"),
    );
    assert!(
        safe_reads >= 2,
        "the fence must keep seeing the production argv reads it governs; found only \
         {safe_reads} `{SAFE}` call(s) — the sweep has drifted off the sources",
    );
}
