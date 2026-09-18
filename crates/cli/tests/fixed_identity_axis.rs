//! M52 Increment 6 / T1 — **the fixed-identity predicate has one engine home**, and
//! both seams that ask *"does this doctype's slug belong to the CLI?"* ask it there
//! (`completions/artifacts/M52/settle-record.md` → D5.1 · §15; `design/storage.md` →
//! Placement, the `cli::doc::fixed_identity_refusal` row's **converse**).
//!
//! The predicate is `placement.is_some() || singleton`. Before this suite, two seams
//! asked only the first disjunct — `engine::store::resolve_read_schema`'s read guard
//! and `cli::doc`'s bare-head expansion — while **the mint asks only the second**
//! (`engine::state::mint_instance`: `let slug = if schema.singleton { … }`). Every
//! shipped doctype declaring either declares both (five, enumerated at HEAD:
//! `changelog` · `vision` · `roadmap` · `decisions-log` · `deferral-ledger`), so the
//! divergence is invisible on the shipped packs and **every cell below is therefore
//! manufactured**: a `location:` + `singleton: true` doctype, the one shape that
//! separates the disjuncts.
//!
//! Driven at HEAD before the fix, that doctype created and promoted at its fixed slug
//! (`singleton_running_doc.rs` pins that half on this very fixture shape) while
//! `jigc doc show <ty>:bogus` fell through to a bare `store.not-found` naming **no
//! rule** and routing at *"create the referenced doc"* — a route that cannot be run,
//! since the only doc this type can have is the one at its fixed slug — and the bare
//! head `<ty>` was refused as a **malformed address**, the spelling the expansion
//! exists to accept.
//!
//! Scope of this suite, stated: it drives the two seams the predicate is shared by.
//! The write doors' own refusal (`store.fixed-identity` at `parse_verb_addr` and the
//! three sibling doors) is T2/T3's, and is deliberately not asserted here.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-fixed-identity-{tag}-{}-{:?}",
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

fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

fn jigc_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &[u8]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait for jigc")
}

fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// The manufactured cell: a doctype that is `singleton: true` **without** `placement:`,
/// homed under an ordinary `location:` directory. The shipped packs cannot produce this
/// shape (§15), so the predicate's second disjunct is reachable only from here.
///
/// The fixture ships alongside a `creates-task: true` workflow whose create-gate admits
/// it, so the staged instance this suite reads back is minted by the real `doc create`
/// door at the slug the mint chooses — never hand-written at the slug the test hopes for.
fn seed_fixture_pack(pack: &Path) {
    let schemas = pack.join("schemas");
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    for d in [&schemas, &workflows, &steps, &config] {
        fs::create_dir_all(d).expect("mk fixture pack subdir");
    }

    fs::write(
        schemas.join("runbook.yaml"),
        "type: runbook\n\
         singleton: true\n\
         location: runbooks/\n\
         description: A fixture running book homed in a location directory, not at a literal file.\n\
         usage: the manufactured cell separating `singleton:` from `placement:`.\n\
         sections:\n\
         \x20 - id: overview\n\
         \x20   slot: {}\n",
    )
    .expect("seed runbook schema");

    fs::write(
        workflows.join("keep-runbook.yaml"),
        "---\n\
         when: maintain the fixture running book\n\
         description: A fixture workflow that creates-or-updates the runbook singleton.\n\
         usage: proving the fixed-identity predicate over a location-homed singleton.\n\
         creates-task: true\n\
         allows-create: [{type: runbook, as: book}]\n\
         ---\n\
         {{ include: step:keep }}\n",
    )
    .expect("seed keep-runbook workflow");
    fs::write(
        steps.join("keep.yaml"),
        "Maintain the running book for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed keep step");

    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    crate::support::seed_ambush_class_declarer(pack);
}

/// A set-up repo whose project layer lists the fixture pack (the flow-18 listed-pack
/// seam), with one open `keep-runbook` task holding a staged, authored `runbook`.
/// Returns `(repo, home, task_id)`.
fn corpus(tag: &str) -> (TempDir, TempDir, String) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    let pack = repo.path().join("fixture-pack");

    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("project layer");

    seed_fixture_pack(&pack);
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("list the fixture pack");

    let started = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "keep-runbook",
            "open the running book",
        ],
    );
    assert_ok(&started, "`jigc start --workflow keep-runbook`");
    let task = "open-the-running-book".to_string();

    let created = jigc(
        repo.path(),
        home.path(),
        &["doc", "create", "runbook", "--title", "runbook"],
    );
    assert_ok(&created, "`jigc doc create runbook`");
    assert!(
        streams(&created).contains("runbook:runbook"),
        "the mint fixes a `singleton`'s slug to the type id — that is the half this \
         suite's seams must agree with; got:\n{}",
        streams(&created),
    );

    let authored = jigc_stdin(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-slot",
            "runbook:runbook#overview",
            "--from-file",
            "-",
        ],
        b"The running book's overview prose.\n",
    );
    assert_ok(&authored, "`jigc doc set-slot runbook:runbook#overview`");

    (repo, home, task)
}

/// A non-canonical slug on a `location:`-homed **singleton** is refused with the
/// singleton rule and the canonical address, on both read copies.
///
/// Before the shared predicate this address read through the guard entirely: the
/// committed arm answered `store.not-found` at `docs/runbooks/bogus.md` naming no rule
/// and routing at *"create the referenced doc"*, which for a fixed-identity doctype
/// names an act that cannot happen.
#[test]
fn a_location_homed_singleton_refuses_a_non_canonical_read_address() {
    let (repo, home, task) = corpus("read-guard");

    let committed = jigc(repo.path(), home.path(), &["doc", "show", "runbook:bogus"]);
    assert!(
        !committed.status.success(),
        "a non-canonical address on a fixed-identity doctype must refuse; got:\n{}",
        streams(&committed),
    );
    let rendered = streams(&committed);
    assert!(
        rendered.contains("is a singleton"),
        "the refusal must name the rule that makes the address impossible, not merely \
         report an absent file; got:\n{rendered}",
    );
    assert!(
        rendered.contains("runbook:runbook"),
        "the refusal must name the canonical address; got:\n{rendered}",
    );
    assert!(
        !rendered.contains("create the referenced doc"),
        "the generic missing-doc route names an act a fixed-identity doctype cannot \
         take; got:\n{rendered}",
    );

    let staged = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "runbook:bogus", "--task", &task],
    );
    assert!(
        !staged.status.success(),
        "the staged arm must refuse too; got:\n{}",
        streams(&staged),
    );
    let staged_rendered = streams(&staged);
    assert!(
        staged_rendered.contains("is a singleton") && staged_rendered.contains("runbook:runbook"),
        "the staged arm's block names the same rule and the same canonical address; \
         got:\n{staged_rendered}",
    );
    assert!(
        staged_rendered.contains(&task),
        "the staged arm's block must still name the copy it was asked about (M51 N15); \
         got:\n{staged_rendered}",
    );
}

/// The bare head of a `location:`-homed **singleton** expands to `<ty>:<ty>` and reads
/// the instance — the spelling the expansion exists to accept, refused as a *malformed
/// address* before the predicate was shared.
#[test]
fn a_location_homed_singletons_bare_head_expands_to_its_canonical_address() {
    let (repo, home, task) = corpus("bare-head");

    let bare = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "runbook", "--task", &task],
    );
    assert_ok(&bare, "`jigc doc show runbook --task <id>` (the bare head)");
    let rendered = streams(&bare);
    assert!(
        rendered.contains("The running book's overview prose."),
        "the bare head must read the instance at the canonical slug; got:\n{rendered}",
    );

    // …and it reads the same bytes the canonical spelling does — the expansion is an
    // alias, not a second read path.
    let canonical = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "runbook:runbook", "--task", &task],
    );
    assert_ok(&canonical, "`jigc doc show runbook:runbook --task <id>`");
    assert_eq!(
        String::from_utf8_lossy(&bare.stdout),
        String::from_utf8_lossy(&canonical.stdout),
        "the bare head is an alias for the canonical address, not a second read path",
    );
}
