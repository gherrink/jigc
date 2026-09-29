//! M49 Increment 10 / T6 — `finalize.render-io` stops reporting a **never-composed**
//! sub-task as a disk fault.
//!
//! Driven at `f29c6cd` under `finalize.fan-out.squash = false`, with one sub-task composed
//! and the other merely `git add`-ed in its worktree, the boundary refused with
//!
//! ```text
//! blocking · finalize.render-io — could not read the staged commit doc
//!   `…/.jigc/tasks/beta/docs/commit:beta.md`: No such file or directory (os error 2)
//!   route: resolve the read fault on `…` (a disk or permissions problem), then re-run the
//!   finalize
//! ```
//!
//! Both halves are law-1 lies (`design/surface-contract.md` → law 1, the route floor):
//! nothing is faulty — the sub-task was **never entered**, so nothing ever provisioned its
//! commit doc — and the named recovery (fix a disk or permissions problem) is not an act the
//! operator can perform, on a path that is jigc's own gitignored workbench rather than
//! anything they own.
//!
//! **The axis is the two call sites × the two read outcomes**, because one shared
//! constructor ([`engine::finalize`]'s `render_io_finding`) serves both renders and a fix at
//! one of them would leave the identical lie standing at the other:
//!
//! | call site | absent (`NotFound`) | unreadable (any other I/O fault) |
//! |---|---|---|
//! | `plan_finalize` phase 3 (a serial task) | the task stages no commit doc | the shipped read-fault text |
//! | `render_subtask_messages` (a fan-out sub-task) | the sub-task was never entered | the shipped read-fault text |
//!
//! Each arm drives the **real binary** and reads the **emitted** bytes: the absent arms
//! assert their route's leading `` `…` `` span parses against the real CLI (the copy-runnable
//! floor), that no route names a `.jigc/tasks/…` path as the operator's subject, and that the
//! disk-fault vocabulary is gone; the serial unreadable arm asserts the shipped text is
//! **kept** verbatim — a genuine fault must not be re-described as a missing doc.
//!
//! **One cell of the axis is not reachable through the binary, and this suite proves that
//! rather than skipping it.** A milestone `join` reads every staged doc body *before* the
//! per-sub-task render is called, so an unreadable `commit:<sub>` is answered by
//! `milestone.area-io` — which describes the same fault honestly — and the render is never
//! reached. The sub-task unreadable arm asserts exactly that hand-off; the render's own
//! unreadable branch for a sub-task has its standing home in the engine
//! (`the_render_finding_tells_an_absent_commit_doc_from_a_read_fault` in
//! `crates/engine/src/finalize.rs`), driven over a real unreadable path.
//!
//! **Reachability of the sub-task arms** (why the fixture is shaped exactly this way): the
//! per-sub-task render runs **only** under `squash: false`, and **only** for a sub-task that
//! staged code — `subtask_patches_and_messages` skips an empty patch, and
//! `milestone.zero-contribution` fires first when no sub-task contributes at all. So both
//! worktrees stage code, and only the *second* sub-task in id order is left without a
//! readable commit doc.

use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-render-io-{tag}-{}-{:?}",
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

/// Run `git <args>` in `cwd`, asserting success.
fn git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Initialize a real git repo with one commit and a project cascade layer. `squash` writes
/// the `finalize.fan-out.squash: false` delta the per-sub-task render lives behind.
fn init_repo(root: &Path, squash_false: bool) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    let config = root.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("mk config layer");
    let manifest = if squash_false {
        "scalar:\n  finalize.fan-out.squash: false\n"
    } else {
        "scalar: {}\n"
    };
    fs::write(config.join("manifest.yaml"), manifest).expect("write manifest");
    git(root, &["add", "README.md"]);
    git(root, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The whole emitted surface of a refused run — the bytes an agent reads, both streams.
fn emitted(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// The staged commit doc of task `id` in the main checkout's workbench — built on the
/// **canonicalized** repo root, because that is the form the binary discovers and therefore
/// the form its messages carry (on macOS `/var/folders/…` resolves to `/private/var/…`).
fn commit_doc(repo: &Path, id: &str) -> PathBuf {
    fs::canonicalize(repo)
        .expect("the repo root resolves")
        .join(".jigc")
        .join("tasks")
        .join(id)
        .join("docs")
        .join(format!("commit:{id}.md"))
}

/// The two read outcomes the shared constructor must tell apart, applied to a commit doc
/// that currently exists.
#[derive(Clone, Copy)]
enum ReadOutcome {
    /// The doc is simply not there — the never-entered / never-authored state.
    Absent,
    /// A genuine I/O fault: the path is a **directory**, so the read fails with something
    /// other than `NotFound` on every platform. Stands in for the permissions/disk faults
    /// the shipped text was written for.
    Unreadable,
}

impl ReadOutcome {
    fn label(self) -> &'static str {
        match self {
            ReadOutcome::Absent => "absent",
            ReadOutcome::Unreadable => "unreadable",
        }
    }

    /// Put an existing commit doc at `path` into this state. A top-level task's commit doc
    /// is provisioned at mint and no door removes it, so **deletion** is the only way the
    /// serial call site reaches `Absent` — unlike the sub-task one, which reaches it by the
    /// state the repro was found in (never entered, so never provisioned).
    fn apply(self, path: &Path) {
        fs::remove_file(path).expect("the commit doc exists before the arm shapes it");
        if let ReadOutcome::Unreadable = self {
            fs::create_dir(path).expect("replace the commit doc with a directory");
        }
    }
}

/// The first `` `…` `` span of `text`, split into an argv — the *emitted* route bytes an
/// agent would copy, never a reconstruction.
fn first_command_span(text: &str) -> Vec<String> {
    let (_, rest) = text.split_once('`').expect("the route carries a `…` span");
    let (span, _) = rest.split_once('`').expect("the `…` span closes");
    span.split_whitespace().map(str::to_owned).collect()
}

/// The `route:` line of the emitted refusal (the `finding_to_err` / findings-surface shape).
fn route_line(text: &str) -> String {
    text.lines()
        .find_map(|l| l.trim().strip_prefix("route: "))
        .unwrap_or_else(|| panic!("the refusal carries a route; got:\n{text}"))
        .to_string()
}

/// The shipped read-fault text, kept verbatim for a genuine fault (the second column of the
/// axis). Asserted as *substrings of the emitted bytes* so a re-word is caught.
fn assert_shipped_read_fault(emitted: &str, path: &Path, label: &str) {
    let display = path.display();
    assert!(
        emitted.contains(&format!("could not read the staged commit doc `{display}`")),
        "[{label}] a genuine I/O fault keeps the shipped message; got:\n{emitted}",
    );
    assert!(
        emitted.contains(&format!(
            "resolve the read fault on `{display}` (a disk or permissions problem), then \
             re-run the finalize"
        )),
        "[{label}] a genuine I/O fault keeps the shipped route; got:\n{emitted}",
    );
}

/// The floor every **absent** arm meets: the disk-fault framing is gone, the route leads with
/// a span that parses against the real CLI, and no route hands the operator a `.jigc/tasks/…`
/// path as their subject.
fn assert_absent_floor(emitted: &str, label: &str) -> String {
    assert!(
        emitted.contains("finalize.render-io"),
        "[{label}] the refusal still names its code; got:\n{emitted}",
    );
    assert!(
        !emitted.contains("could not read the staged commit doc"),
        "[{label}] an absent doc is not a failed read; got:\n{emitted}",
    );
    let route = route_line(emitted);
    assert!(
        !route.contains("a disk or permissions problem"),
        "[{label}] an absent doc is not a disk or permissions problem; got: {route}",
    );
    assert!(
        !route.contains(".jigc/tasks"),
        "[{label}] no route names jigc's own workbench path as the operator's subject; \
         got: {route}",
    );
    let argv = first_command_span(&route);
    cli::cli::Cli::try_parse_from(&argv).unwrap_or_else(|err| {
        panic!(
            "[{label}] the route span `{}` must parse: {err}",
            argv.join(" ")
        )
    });
    route
}

/// Run the route's leading span **as emitted** — the argv split off the printed bytes, never a
/// hand-built equivalent — and require exit 0. A route the operator is told to run must run
/// from the state that printed it (`design/surface-contract.md` → the route floor).
fn run_route_verbatim(repo: &Path, home: &Path, argv: &[String], label: &str) -> String {
    let tail: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    assert_eq!(argv.first().map(String::as_str), Some("jigc"));
    let out = run_jigc(repo, home, &tail);
    assert_ok(
        &out,
        &format!("[{label}] the emitted route `{}`", argv.join(" ")),
    );
    emitted(&out)
}

// ---------------------------------------------------------------------------------------
// Call site 1 — `plan_finalize` phase 3: a serial task's own commit doc.
// ---------------------------------------------------------------------------------------

#[test]
fn a_serial_task_render_tells_an_absent_commit_doc_from_a_read_fault() {
    for outcome in [ReadOutcome::Unreadable, ReadOutcome::Absent] {
        let label = outcome.label();
        let repo = TempDir::new(&format!("task-{label}"));
        let home = TempDir::new(&format!("home-task-{label}"));
        init_repo(repo.path(), false);

        assert_ok(
            &run_jigc(
                repo.path(),
                home.path(),
                &["start", "--workflow", "quick-fix", "Warm the cache"],
            ),
            "`jigc start`",
        );
        let id = "warm-the-cache";
        let doc = commit_doc(repo.path(), id);
        outcome.apply(&doc);

        // Staged code, so the empty-commit guard does not answer first.
        fs::write(repo.path().join("code.rs"), "fn x() {}\n").expect("write code");
        git(repo.path(), &["add", "code.rs"]);

        let out = run_jigc(repo.path(), home.path(), &["task", "finalize", id]);
        assert!(
            !out.status.success(),
            "[{label}] the render fault must block; got {:?}",
            out.status,
        );
        let text = emitted(&out);

        match outcome {
            ReadOutcome::Unreadable => assert_shipped_read_fault(&text, &doc, label),
            ReadOutcome::Absent => {
                let route = assert_absent_floor(&text, label);
                assert!(
                    text.contains(&format!("task `{id}` stages no commit doc")),
                    "[{label}] the absent doc is named as the task's missing stage; got:\n{text}",
                );
                assert_eq!(
                    first_command_span(&route),
                    vec!["jigc", "doc", "list", "--task", id],
                    "[{label}] the route leads with the read of what the task stages; got: {route}",
                );
                assert!(
                    route.contains(&format!("jigc task discard {id}")),
                    "[{label}] the route names the other honest exit; got: {route}",
                );
                run_route_verbatim(repo.path(), home.path(), &first_command_span(&route), label);
            }
        }
    }
}

// ---------------------------------------------------------------------------------------
// Call site 2 — `render_subtask_messages`: a fan-out sub-task's commit doc, `squash: false`.
// ---------------------------------------------------------------------------------------

/// The `squash: false` fan-out fixture: a milestone with two sub-tasks, both provisioned and
/// both staging code in their worktrees (the reachability requirement — an empty patch is
/// skipped, and a milestone contributing nothing is refused earlier). `enter_beta` is the
/// difference between the two arms: `beta-area` is **re-entered** (which provisions its
/// commit doc) only when the arm needs a doc to break, and is left in the repro's own
/// never-entered state otherwise.
/// Fill a provisioned sub-task's `commit:<sub>` skeleton — its `type` field and its `summary`
/// slot, the doctype's whole author-required leaf set. **The fixture must do this**: since the
/// milestone-boundary gate's subject grew to the transient commit docs a `squash: false`
/// boundary renders (`design/validation.md` → The milestone-boundary gate), an unfilled
/// skeleton on the *entered* sub-task blocks the boundary at the gate and this suite's
/// subject — the render's two read outcomes on the *other* sub-task — is never reached.
fn author_commit_doc(worktree: &Path, home: &Path, sub: &str) {
    let addr = format!("commit:{sub}");
    assert_ok(
        &run_jigc(
            worktree,
            home,
            &[
                "doc",
                "set-field",
                &format!("{addr}#header/type"),
                "--task",
                sub,
                "--value",
                "feat",
            ],
        ),
        "`jigc doc set-field commit:<sub>#header/type`",
    );
    let prose = worktree.join(format!(".{sub}-summary"));
    fs::write(&prose, "rework the cache path\n").expect("write the summary prose");
    assert_ok(
        &run_jigc(
            worktree,
            home,
            &[
                "doc",
                "set-slot",
                &format!("{addr}#summary"),
                "--task",
                sub,
                "--from-file",
                prose.to_str().expect("utf-8 prose path"),
            ],
        ),
        "`jigc doc set-slot commit:<sub>#summary`",
    );
    fs::remove_file(&prose).expect("the prose source is scratch, not worktree content");
}

fn setup_fanout(repo: &Path, home: &Path, enter_beta: bool) {
    assert_ok(
        &run_jigc(repo, home, &["milestone", "create", "Cache rework"]),
        "`jigc milestone create`",
    );
    for intent in ["Alpha area", "Beta area"] {
        assert_ok(
            &run_jigc(
                repo,
                home,
                &["milestone", "add-task", "cache-rework", intent],
            ),
            "`jigc milestone add-task`",
        );
    }
    assert_ok(
        &run_jigc(repo, home, &["milestone", "provision", "cache-rework"]),
        "`jigc milestone provision`",
    );
    for (sub, enter) in [("alpha-area", true), ("beta-area", enter_beta)] {
        let worktree = repo.join(".jigc").join("worktrees").join(sub);
        if enter {
            assert_ok(
                &run_jigc(&worktree, home, &["workflow", "sub-task", "--task", sub]),
                "`jigc workflow sub-task --task <sub>` (the re-entry that provisions the doc)",
            );
            author_commit_doc(&worktree, home, sub);
        }
        let code = format!("{sub}.rs");
        fs::write(worktree.join(&code), "pub fn f() {}\n").expect("write worktree code");
        git(&worktree, &["add", &code]);
    }
}

#[test]
fn a_sub_task_render_tells_a_never_entered_sub_task_from_a_read_fault() {
    for outcome in [ReadOutcome::Unreadable, ReadOutcome::Absent] {
        let label = outcome.label();
        let repo = TempDir::new(&format!("fanout-{label}"));
        let home = TempDir::new(&format!("home-fanout-{label}"));
        init_repo(repo.path(), true);
        // `beta-area` sorts after `alpha-area`, so the first render succeeds and the second
        // is the one under test — the driven repro's shape.
        let sub = "beta-area";
        let unreadable = matches!(outcome, ReadOutcome::Unreadable);
        setup_fanout(repo.path(), home.path(), unreadable);
        let doc = commit_doc(repo.path(), sub);
        if unreadable {
            outcome.apply(&doc);
        }

        let out = run_jigc(
            repo.path(),
            home.path(),
            &["milestone", "finalize", "cache-rework"],
        );
        assert!(
            !out.status.success(),
            "[{label}] the render fault must block the boundary; got {:?}\n{}",
            out.status,
            emitted(&out),
        );
        let text = emitted(&out);

        match outcome {
            // The join's own read answers first, and answers honestly — the render's
            // unreadable branch is unreachable from here (see the module header).
            ReadOutcome::Unreadable => {
                assert!(
                    text.contains("milestone.area-io")
                        && text.contains("could not read a staged doc body"),
                    "[{label}] the join's read of the sub-task's staged bodies answers first; \
                     got:\n{text}",
                );
                assert!(
                    !text.contains("finalize.render-io"),
                    "[{label}] the render is never reached; got:\n{text}",
                );
            }
            ReadOutcome::Absent => {
                let route = assert_absent_floor(&text, label);
                assert!(
                    text.contains(&format!("no commit doc for sub-task `{sub}`")),
                    "[{label}] the absent doc is named as the sub-task's, not a fault; \
                     got:\n{text}",
                );
                assert_eq!(
                    first_command_span(&route),
                    vec!["jigc", "milestone", "execute", "cache-rework"],
                    "[{label}] the route leads with the door that prints the launch line; \
                     got: {route}",
                );
                assert!(
                    route.contains(&format!("jigc task discard {sub}")),
                    "[{label}] the route names the settle exit; got: {route}",
                );
                // The route's own claim, checked rather than trusted: running it prints the
                // sub-task's launch line.
                let printed = run_route_verbatim(
                    repo.path(),
                    home.path(),
                    &first_command_span(&route),
                    label,
                );
                // The `cd` operand is the ABSOLUTE worktree path since M53 (the cwd
                // census, C1-14 / C3-01), so the suffix is what identifies the line.
                assert!(
                    printed.contains(&format!(
                        "/.jigc/worktrees/{sub} && jigc workflow sub-task --task {sub}`"
                    )),
                    "[{label}] the route claims it prints the launch line; got:\n{printed}",
                );
            }
        }
    }
}
