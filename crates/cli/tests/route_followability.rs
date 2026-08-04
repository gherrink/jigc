//! M47 Increment 6, T3 — **P6 route-followability through the real binary**
//! (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 8 + the pre-decompose review's P6
//! rider; `design/surface-contract.md` → The route fence).
//!
//! The M43 route fence proves a mechanical route **parses**. It cannot prove the route an
//! agent actually reads is **followable**, because the fence sits on `Route::mechanical`
//! and never sees the finding: a route whose argv still carries `<doctype>` or `<address>`
//! passes the fence — those tokens are declared members of the fence's own dummy table —
//! and then reaches a driver as a command it cannot run. P6 closes that: *a placeholder
//! whose value is derivable from the finding's own `key.target` must be substituted*,
//! asserted at the finding-**serialization** seam (the fourth assert there).
//!
//! This suite is the end-to-end half. For each shipped route family it provokes the block
//! through the built binary, reads the emitted `route` out of the `--format json` findings
//! envelope, and then **runs the emitted backticked argv verbatim** — the emitted bytes are
//! the contract, never a reconstruction — asserting exit 0:
//!
//!   - the `<doctype>`-bearing write reject (`write.wrong-shape`) names the real doctype,
//!     and `jigc doc schema <that doctype>` runs;
//!   - the `write.not-present` item-id miss names the real containing section **and** the
//!     real task id, and that `jigc doc show … --task …` runs;
//!   - the `schema-conformance.*` gate block names its real write address, and that
//!     `jigc doc set-slot … --from-file -` runs and clears the finding.
//!
//! **Declared bound, recorded not glossed:** the *un-enriched* `write.not-present` fallback
//! (`engine::write::write_route`) has no binary-reachable producer today — M47 Inc 6 T2
//! wired the enrichment onto all six write verbs, and the batch `jigc doc author` path
//! lowers every item hop with the same `slugify` the engine mints ids with, so it cannot
//! address an item it did not just add. Its substitution is therefore pinned where it is
//! reachable, in `crates/engine/src/write.rs` →
//! `mod not_present_route_followability`; the arm below drives the **enriched** producer,
//! which is what an agent meets.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-route-followability-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — byte-identical to the binary-embedded pack.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A live fixture: a `git init` repo with the project layer, `jigc setup` run, and one
/// open `single-task` task (so its transient `commit` doc is provisioned).
struct Fixture {
    repo: TempDir,
    home: TempDir,
    task: String,
}

impl Fixture {
    fn new(tag: &str) -> Self {
        let repo = TempDir::new(tag);
        let home = TempDir::new("home");
        git(repo.path(), &["init", "-q"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-q", "-m", "initial"]);
        fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

        let fixture = Fixture {
            repo,
            home,
            task: "add-a-widget".to_owned(),
        };
        fixture.ok(&["setup"], None, "jigc setup");
        fixture.ok(
            &["start", "--workflow", "single-task", "add a widget"],
            None,
            "jigc start --workflow single-task",
        );
        fixture
    }

    /// Run a `jigc` subcommand against this fixture, optionally piping `stdin`.
    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", dev_pack())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if stdin.is_some() {
            command.stdin(Stdio::piped());
        }
        let mut child = command.spawn().expect("spawn the jigc binary");
        if let Some(bytes) = stdin {
            child
                .stdin
                .take()
                .expect("stdin piped")
                .write_all(bytes)
                .expect("write stdin");
        }
        child.wait_with_output().expect("wait for jigc")
    }

    /// Run a `jigc` subcommand and assert exit 0, returning trimmed stdout.
    fn ok(&self, args: &[&str], stdin: Option<&[u8]>, what: &str) -> String {
        let out = self.run(args, stdin);
        assert!(
            out.status.success(),
            "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout)
            .expect("utf-8 stdout")
            .trim_end_matches('\n')
            .to_owned()
    }

    /// **Follow the emitted route**: split the backticked argv verbatim, drop the leading
    /// `jigc`, and run exactly those bytes — the followability proof. Asserts exit 0.
    fn follow(&self, route: &str, stdin: Option<&[u8]>, what: &str) -> String {
        let cmd = backticked(route);
        let mut parts = cmd.split_whitespace();
        assert_eq!(parts.next(), Some("jigc"), "a route leads with `jigc`");
        let args: Vec<&str> = parts.collect();
        self.ok(&args, stdin, what)
    }
}

/// The leading backticked command of a route string (`` `<cmd>`<tail> ``).
fn backticked(route: &str) -> &str {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("a mechanical route carries a backticked command; got: {route}"))
}

/// Parse a blocking `--format json` findings envelope out of `out` (either stream — the
/// write verbs block on stderr, the task gate reports on stdout) and return the `route` of
/// the first finding carrying `code`.
fn route_of(out: &Output, code: &str, what: &str) -> String {
    assert!(
        !out.status.success(),
        "`{what}` must block (non-zero exit); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let report: serde_json::Value = serde_json::from_str(stderr.trim())
        .or_else(|_| serde_json::from_str(stdout.trim()))
        .unwrap_or_else(|e| {
            panic!("`{what}` emits a JSON envelope: {e}; stdout:\n{stdout}\nstderr:\n{stderr}")
        });
    let findings = report["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` carries a findings array; got:\n{report}"));
    let finding = findings
        .iter()
        .find(|f| f["code"] == code)
        .unwrap_or_else(|| panic!("`{what}` carries a `{code}` finding; got:\n{report}"));
    finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("`{what}`'s `{code}` carries a route; got:\n{finding}"))
        .to_owned()
}

/// **The `<doctype>` family.** A genuine shape question (`add-item` into a section the
/// schema declares non-repeatable) routes `jigc doc schema <doctype>` — the placeholder is
/// derivable from the finding's own `key.target` (`commit:<task>#summary` → `commit`), so
/// the emitted route names the real doctype and runs.
#[test]
fn a_shape_question_route_names_the_real_doctype_and_runs() {
    let fx = Fixture::new("doctype");
    let address = format!("commit:{}#summary", fx.task);

    let out = fx.run(
        &[
            "doc", "add-item", &address, "--title", "Nope", "--format", "json",
        ],
        None,
    );
    let route = route_of(
        &out,
        "write.wrong-shape",
        "add-item into a non-repeatable section",
    );
    assert_eq!(
        backticked(&route),
        "jigc doc schema commit",
        "the shape question names the doctype its own target carries; route:\n{route}",
    );

    let shown = fx.follow(&route, None, "the emitted `jigc doc schema`");
    assert!(
        shown.contains("summary"),
        "the followed route projects the doctype's declared shape; got:\n{shown}",
    );
}

/// **The `write.not-present` family.** An item-id miss names the followable containing
/// section *and* the real task id — `<address>` derived from `key.target`, `<task-id>` from
/// the dispatch context that resolved the task (the declared non-derivable placeholder's
/// "different source"). Both are concrete, so the emitted read runs.
#[test]
fn an_item_id_miss_route_names_the_real_address_and_task_and_runs() {
    let fx = Fixture::new("notpresent");
    let trailers = format!("commit:{}#trailers", fx.task);
    fx.ok(
        &["doc", "add-item", &trailers, "--title", "Refs"],
        None,
        "add-item a real trailer",
    );

    let out = fx.run(
        &[
            "doc",
            "retitle-item",
            &format!("{trailers}/nonesuch"),
            "--title",
            "Reviewed-by",
            "--format",
            "json",
        ],
        None,
    );
    let route = route_of(&out, "write.not-present", "retitle-item at an absent item");
    assert_eq!(
        backticked(&route),
        format!("jigc doc show {trailers} --task {}", fx.task),
        "the item-id miss names the real containing section and the real task; route:\n{route}",
    );

    let shown = fx.follow(&route, None, "the emitted `jigc doc show`");
    assert!(
        shown.contains("Refs"),
        "the followed route reveals the section's live item ids; got:\n{shown}",
    );
}

/// **The `schema-conformance.*` gate family.** The task gate's required-slot block routes
/// `jigc doc set-slot <address> --from-file -`; `<address>` is the finding's own
/// `key.target`, so the emitted write runs verbatim — and clears the very finding that
/// emitted it (`<value>` stays a placeholder by declaration: the value is the agent's).
#[test]
fn a_gate_block_route_names_its_real_write_address_and_runs() {
    let fx = Fixture::new("gate");

    let out = fx.run(&["task", "validate", &fx.task, "--format", "json"], None);
    let route = route_of(
        &out,
        "schema-conformance.required-slot-present",
        "the task gate over an unfilled commit doc",
    );
    assert_eq!(
        backticked(&route),
        format!("jigc doc set-slot commit:{}#summary --from-file -", fx.task),
        "the gate block names the write address its own target carries; route:\n{route}",
    );

    fx.follow(
        &route,
        Some(b"Add a widget\n"),
        "the emitted `jigc doc set-slot`",
    );

    // The followed route actually repaired what the finding reported: the summary slot is
    // no longer among the gate's required-slot blocks.
    let after = fx.run(&["task", "validate", &fx.task, "--format", "json"], None);
    let stdout = String::from_utf8_lossy(&after.stdout);
    assert!(
        !stdout.contains(&format!("commit:{}#summary", fx.task)),
        "the followed route cleared the finding that emitted it; got:\n{stdout}",
    );
}
