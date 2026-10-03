//! M51 Increment 4 / T3 — the **config-layer worktree pre-image family**: capture,
//! compare-and-swap restore, and `finalize.rollback-conflict`
//! (`design/finalize.md` → Rollback discipline; `completions/artifacts/M51/settle-record.md`
//! → §6 / D4; the roadmap's Increment 4 → *Proves*).
//!
//! `finalize` **rewrites two files in the worktree** inside its commit closure —
//! `.jigc/.gitignore` (the shared `gitignore::ensure` amend) and `.jigc/version` (the
//! stamp `refresh_version_stamp` refreshes) — and until this increment the transaction
//! rolled back neither. Four index axes restored the *index* and the M45 audit's own row
//! said *"worktree untouched"* about the mechanism, so a hook-rejected finalize left
//! ` M .jigc/.gitignore` and ` M .jigc/version` on disk while telling the operator
//! *"nothing was committed"*. Driven at `92755635`, both residues present.
//!
//! **The restore is compare-and-swap, not a rewrite** (§6, Codex 4). The interval between
//! jigc's write and the rollback spans promotion, retirement, staging and the user's hooks —
//! every one of which can run arbitrary code — so an unconditional restore destroys a
//! concurrent edit, *the same loss D4 exists to prevent, in the other direction*. Each entry
//! therefore carries the **pre-image** and **the exact bytes jigc wrote**, and restores only
//! while the file still holds jigc's bytes. When it does not, nothing is overwritten: the
//! pre-image is parked in the gitignored `.jigc/displaced/` workbench and one blocking
//! `finalize.rollback-conflict` names **both** copies.
//!
//! **The shape space is manufactured, and this suite says so.** `{stage failure, hook
//! rejection} × {unchanged since jigc's post-write image, concurrently edited}` is not read
//! off any registry: the failure points are *decided* (which two of the closure's many `?`s
//! are worth driving), and the second axis is a **race**, which no code-side set carries.
//! [`CELLS`] is that manufactured space written down, each row carrying the injector that
//! manufactures it — because the alternative to a manufactured space that says so is a
//! single happy cell that looks like a swept axis (M49's `spec` lesson, one wave over).
//!
//! The two injectors are real git, not a shim: a rejecting `.git/hooks/pre-commit` for the
//! commit phase, and a **required clean filter** on a file under `.jigc/config/` for the
//! stage phase — `git add` runs it, it exits non-zero, and `git add` fails at 128. Both run
//! *inside* the transaction, after both writes, so the "concurrently edited" cell is a
//! genuine interleave rather than a simulated one: the injector edits the two files and only
//! then refuses.
//!
//! The arms beyond the space: the **absent pre-image** (the restore is a *delete* — the
//! `RecordPreImage` discipline's third axis, where a capture modelling only "present" leaves
//! jigc's write behind), the **never-written** path (a file jigc did not touch this run is
//! neither restored nor reported, which is what keeps the fan-out arms — where
//! `refresh_version_stamp` never runs — from being rolled back against bytes jigc never
//! produced), and the **totality** arm: the config-layer pathspecs the binary actually
//! stages, lifted from its own stage-failure message, must equal the dispositioned rows of
//! `cli::task::CONFIG_LAYER_SPECS`.

#![cfg(unix)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use cli::rollback::{FINALIZE_DOOR, PreImage, PreImageFamily};
use cli::task::{CONFIG_LAYER_SPECS, ConfigLayerWrite};

/// The finding one conflicted path mints.
const CONFLICT: &str = "finalize.rollback-conflict";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-config-preimage-{tag}-{}-{:?}",
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

// ---------------------------------------------------------------------------
// The manufactured shape space
// ---------------------------------------------------------------------------

/// Where inside the commit closure the transaction is made to fail. **Decided, not
/// enumerated**: the closure has many failure points and no code-side set names them, so
/// these two are chosen for what they discriminate — the stage arm fails *before* any hook
/// exists (jigc's own `git add`), the commit arm fails *after* the whole stage succeeded, in
/// the user's own hook.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Failure {
    /// `git add` refuses — manufactured with a **required clean filter** on a file under
    /// `.jigc/config/`, which is one of the pathspecs jigc's own stage adds.
    Stage,
    /// The user's `pre-commit` hook refuses.
    Hook,
}

/// Whether the two files still hold jigc's own bytes when the rollback runs. **A race**,
/// which is precisely why no registry carries this axis: the fixture manufactures it by
/// having the failure injector itself do the editing, from inside the transaction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Race {
    /// Nothing touched the files after jigc wrote them — the compare-and-swap holds and both
    /// pre-images are restored byte-for-byte.
    Untouched,
    /// The injector rewrote both files after jigc wrote them — the compare-and-swap fails and
    /// neither is overwritten.
    ConcurrentEdit,
}

/// One cell of the manufactured space.
struct Cell {
    failure: Failure,
    race: Race,
    /// What this cell is *for* — so a reddening cell says which property it was holding.
    note: &'static str,
}

/// The space, written down. Four cells, `{Stage, Hook} × {Untouched, ConcurrentEdit}`, each
/// driven over **both** files in one run.
const CELLS: &[Cell] = &[
    Cell {
        failure: Failure::Stage,
        race: Race::Untouched,
        note: "jigc's own `git add` refused; nothing else ran, so the pre-images restore",
    },
    Cell {
        failure: Failure::Stage,
        race: Race::ConcurrentEdit,
        note: "the clean filter edited both files and then refused — the earliest point at \
               which a rollback can meet bytes that are not jigc's",
    },
    Cell {
        failure: Failure::Hook,
        race: Race::Untouched,
        note: "the headline cell: a user's uncommitted private line in `.jigc/.gitignore` \
               plus a rejecting `pre-commit`",
    },
    Cell {
        failure: Failure::Hook,
        race: Race::ConcurrentEdit,
        note: "the hook rewrote both files before refusing — the widest interleave, since a \
               hook is arbitrary user code running inside the transaction",
    },
];

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

/// Bytes no real `jigc` build stamps, so the refresh is guaranteed to write a different
/// `.jigc/version` and the entry is live rather than a byte no-op.
const STALE_STAMP: &str = "0.0.0-fixture-stale-stamp\n";

/// The user's own two lines in `.jigc/.gitignore` — **uncommitted**, which is what makes the
/// residue observable in `git status` and the loss unrecoverable from any git object.
const PRIVATE_LINES: &str = "# my own\nscratch-notes/\n";

/// The committed ignore set, one canonical entry short (`displaced/`) — the ordinary
/// `ENTRIES`-upgrade shape, and the only shape in which the amend writes anything at all.
const COMMITTED_ENTRIES: &str = "tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\n";

/// What the ignore file holds pre-finalize in the ordinary fixture.
fn ignore_pre_image() -> String {
    format!("{COMMITTED_ENTRIES}{PRIVATE_LINES}")
}

/// Run a `git` command in `repo`, asserting success, returning raw stdout.
fn git(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR");
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The intent + task id every arm mints.
const INTENT: &str = "cache sessions in a single in-memory node";
const TASK: &str = "cache-sessions-in-a-single";

/// A repo with one commit, a `.jigc/config/` layer holding one file, a **committed** stale
/// `.jigc/version`, and `ignore` as the pre-finalize `.jigc/.gitignore` (`None` writes no
/// ignore file at all — the absent-pre-image arm). The private lines, when present, are
/// appended **after** the commit, so they exist in no git object.
fn init_repo(repo: &Path, ignore: Option<&str>) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(repo.join(".jigc").join("version"), STALE_STAMP).expect("write the stale stamp");
    // The stage injector's target: a file under `.jigc/config/`, which jigc's own stage
    // `git add`s. Untracked, so no `git status` on the way in ever hashes it — the filter
    // fires at jigc's stage and nowhere earlier.
    fs::write(
        repo.join(".jigc").join("config").join("filtered.txt"),
        "x\n",
    )
    .expect("write the filter target");
    let mut add = vec!["add", "README.md", ".jigc/version"];
    if let Some(body) = ignore {
        fs::write(repo.join(".jigc").join(".gitignore"), body).expect("write the ignore file");
        add.push(".jigc/.gitignore");
    }
    git(repo, &add);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Fill the provisioned commit doc so the finalize validates clean — leaving the transaction's
/// failure injector as the only lever the arm turns.
fn fill_commit(repo: &Path, home: &Path) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{TASK}#type"), "feat");
    set_field(&format!("commit:{TASK}#scope"), "cache");
    set_slot(&format!("commit:{TASK}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{TASK}#body"), b"A cache change.\n");
}

/// The shell body both injectors share: optionally rewrite the two files jigc just wrote,
/// then refuse. Written as one function so the two cells of the race axis differ by exactly
/// one thing — who refuses — and not by what the edit is.
fn injector_body(race: Race, refusal: &str) -> String {
    let edit = match race {
        Race::Untouched => String::new(),
        Race::ConcurrentEdit => format!(
            "printf '{CONCURRENT_IGNORE_LINE}\\n' >> .jigc/.gitignore\n\
             printf '{CONCURRENT_STAMP}' > .jigc/version\n"
        ),
    };
    format!("#!/bin/sh\n{edit}{refusal}\n")
}

/// The line the concurrent editor appends to `.jigc/.gitignore`.
const CONCURRENT_IGNORE_LINE: &str = "# written while finalize was running";
/// The bytes the concurrent editor puts in `.jigc/version`.
const CONCURRENT_STAMP: &str = "concurrent-stamp\\n";

/// Install the cell's failure injector. Both are ordinary git features and both run **inside**
/// the transaction, after `gitignore::ensure` and after `refresh_version_stamp`.
fn install_injector(repo: &Path, cell: &Cell) {
    match cell.failure {
        Failure::Hook => {
            let hook = repo.join(".git").join("hooks").join("pre-commit");
            fs::write(
                &hook,
                injector_body(cell.race, "echo 'rejected by test hook' 1>&2\nexit 1"),
            )
            .expect("write the pre-commit hook");
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook, perms).expect("chmod the hook");
        }
        Failure::Stage => {
            let script = repo.join("stage-filter.sh");
            fs::write(
                &script,
                injector_body(cell.race, "echo 'clean filter refused' 1>&2\nexit 3"),
            )
            .expect("write the filter script");
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script)
                .expect("script metadata")
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script, perms).expect("chmod the script");
            fs::write(
                repo.join(".gitattributes"),
                ".jigc/config/filtered.txt filter=boom\n",
            )
            .expect("write .gitattributes");
            git(repo, &["config", "filter.boom.clean", "./stage-filter.sh"]);
            git(repo, &["config", "filter.boom.required", "true"]);
        }
    }
}

/// Take the injector back out, so the assertions afterwards run against ordinary git (a live
/// required filter makes every later `git status` fail, which would measure the injector).
fn remove_injector(repo: &Path, cell: &Cell) {
    match cell.failure {
        Failure::Hook => {
            let _ = fs::remove_file(repo.join(".git").join("hooks").join("pre-commit"));
        }
        Failure::Stage => {
            let _ = Command::new("git")
                .args(["config", "--unset", "filter.boom.clean"])
                .current_dir(repo)
                .output();
            let _ = Command::new("git")
                .args(["config", "--unset", "filter.boom.required"])
                .current_dir(repo)
                .output();
            let _ = fs::remove_file(repo.join(".gitattributes"));
            let _ = fs::remove_file(repo.join("stage-filter.sh"));
        }
    }
}

/// One driven run: the fixture, the task, the injector, the refused `jigc task finalize`.
struct Driven {
    repo: TempDir,
    #[allow(dead_code)]
    home: TempDir,
    /// stdout and stderr of the finalize, concatenated — the surface an operator reads.
    rendered: String,
}

impl Driven {
    fn repo(&self) -> &Path {
        self.repo.path()
    }

    fn read(&self, rel: &str) -> Option<String> {
        fs::read_to_string(self.repo().join(rel)).ok()
    }

    fn status(&self) -> String {
        git(self.repo(), &["status", "--porcelain"])
    }

    /// Every file parked in the gitignored relocation workbench, as
    /// `(path relative to `.jigc/displaced/`, bytes)`.
    ///
    /// **A recursive walk, because the park is keyed by the DOOR and then by the entry's own
    /// identity** — `.jigc/displaced/<door>/<identity>.pre-image.<nanos>` — so a flat
    /// `read_dir` sees only the door directory. The identity is what makes two populations
    /// sharing a basename distinguishable, which is exactly what a flat listing threw away.
    fn parked(&self) -> Vec<(String, String)> {
        let root = self.repo().join(".jigc").join("displaced");
        let mut out = Vec::new();
        let mut stack = vec![root.clone()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                out.push((
                    path.strip_prefix(&root)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .into_owned(),
                    fs::read_to_string(&path).unwrap_or_default(),
                ));
            }
        }
        out.sort();
        out
    }

    /// How many `finalize.rollback-conflict` findings the run rendered.
    fn conflicts(&self) -> usize {
        self.rendered.matches(CONFLICT).count()
    }
}

/// Drive one refused finalize: `ignore` is the pre-finalize `.jigc/.gitignore` (`None` = the
/// file does not exist), `cell` decides how and when the transaction fails.
fn drive(tag: &str, ignore: Option<&str>, cell: &Cell) -> Driven {
    let repo = TempDir::new(tag);
    let home = TempDir::new("home");
    init_repo(repo.path(), ignore);
    if ignore.is_some() {
        // The user's own lines land AFTER the commit — uncommitted, in no git object.
        let path = repo.path().join(".jigc").join(".gitignore");
        let mut body = fs::read_to_string(&path).expect("read the ignore file");
        body.push_str(PRIVATE_LINES);
        fs::write(&path, body).expect("append the private lines");
    }

    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", INTENT],
        ),
        "`jigc start`",
    );
    fill_commit(repo.path(), home.path());

    // A real code change in the index, so the finalize reaches the stage/commit phase rather
    // than blocking earlier on `finalize.nothing-staged`.
    fs::write(repo.path().join("README.md"), "hello world\n").expect("edit README");
    git(repo.path(), &["add", "README.md"]);

    install_injector(repo.path(), cell);
    let out = jigc(repo.path(), home.path(), &["task", "finalize", TASK]);
    remove_injector(repo.path(), cell);

    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "[{}/{:?}/{:?}] the injector must make finalize exit non-zero; got:\n{rendered}",
        cell.note,
        cell.failure,
        cell.race,
    );
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]).trim(),
        "1",
        "[{}] a refused finalize creates no commit",
        cell.note,
    );
    Driven {
        repo,
        home,
        rendered,
    }
}

// ---------------------------------------------------------------------------
// The four cells
// ---------------------------------------------------------------------------

/// Every cell of the manufactured space, over **both** files in one run.
///
/// `Untouched`: both pre-images come back byte-for-byte — the ignore file is the user's own
/// content with jigc's appended entry gone, the stamp is the stale committed bytes — and
/// `git status` is **not falsely clean**: the user's uncommitted line is still an uncommitted
/// modification, which is exactly what the pre-amend writer made disappear.
///
/// `ConcurrentEdit`: neither file is overwritten, **both** versions of each survive (the
/// editor's bytes in place, jigc's pre-image parked in `.jigc/displaced/`), and each
/// conflicted path mints one blocking `finalize.rollback-conflict` naming both copies.
#[test]
fn the_manufactured_shape_space_holds_over_both_files() {
    for cell in CELLS {
        let tag = format!("{:?}-{:?}", cell.failure, cell.race).to_lowercase();
        let run = drive(&tag, Some(COMMITTED_ENTRIES), cell);
        let label = format!("[{:?}/{:?}] {}", cell.failure, cell.race, cell.note);

        match cell.race {
            Race::Untouched => {
                assert_eq!(
                    run.read(".jigc/.gitignore").as_deref(),
                    Some(ignore_pre_image().as_str()),
                    "{label}: the ignore file must be restored to its pre-finalize bytes — \
                     jigc's appended entry gone, the user's own lines intact",
                );
                assert_eq!(
                    run.read(".jigc/version").as_deref(),
                    Some(STALE_STAMP),
                    "{label}: the stamp must be restored to its pre-finalize bytes",
                );
                let status = run.status();
                assert!(
                    status.contains(" M .jigc/.gitignore"),
                    "{label}: `git status` must NOT be falsely clean — the user's own \
                     uncommitted line is still an uncommitted modification; got:\n{status}",
                );
                assert!(
                    !status.contains(".jigc/version"),
                    "{label}: the restored stamp matches HEAD, so it must not be reported \
                     modified; got:\n{status}",
                );
                assert_eq!(
                    run.conflicts(),
                    0,
                    "{label}: nothing raced the rollback, so no conflict is reported; \
                     got:\n{}",
                    run.rendered,
                );
                assert!(
                    run.parked().is_empty(),
                    "{label}: nothing is parked when the restore succeeded; got {:?}",
                    run.parked(),
                );
            }
            Race::ConcurrentEdit => {
                let ignore = run
                    .read(".jigc/.gitignore")
                    .expect("the ignore file survives");
                assert!(
                    ignore.contains(CONCURRENT_IGNORE_LINE),
                    "{label}: the concurrent editor's bytes must NOT be overwritten; \
                     got:\n{ignore}",
                );
                assert!(
                    ignore.contains("displaced/"),
                    "{label}: the file still holds jigc's own append too — the rollback did \
                     not touch it at all; got:\n{ignore}",
                );
                assert_eq!(
                    run.read(".jigc/version").as_deref(),
                    Some("concurrent-stamp\n"),
                    "{label}: the concurrently written stamp must NOT be overwritten",
                );

                // Both versions survive: the pre-images are parked, not discarded.
                let parked = run.parked();
                assert_eq!(
                    parked.len(),
                    2,
                    "{label}: both pre-images must be parked in `.jigc/displaced/`; got {parked:?}",
                );
                // **The park is keyed by the door and then by the entry's own identity.**
                // A flat `<basename>.pre-image.<nanos>` says nothing about which live path
                // the bytes came from, so two populations sharing a basename land
                // indistinguishably in one directory — which is the shape this task
                // replaces (`settle-record.md` → §1).
                for identity in [".jigc/.gitignore", ".jigc/version"] {
                    let prefix = format!("finalize/{identity}.pre-image.");
                    assert_eq!(
                        parked
                            .iter()
                            .filter(|(path, _)| path.starts_with(&prefix))
                            .count(),
                        1,
                        "{label}: `{identity}`'s pre-image must be parked at \
                         `.jigc/displaced/{prefix}<nanos>` — the door names the \
                         sub-directory and the entry's own identity names the file, so a \
                         reader can tell which live path each parked copy came from; got \
                         {parked:?}",
                    );
                }
                let bodies: Vec<&str> = parked.iter().map(|(_, body)| body.as_str()).collect();
                assert!(
                    bodies.contains(&ignore_pre_image().as_str()),
                    "{label}: the ignore file's pre-image must survive verbatim; got {parked:?}",
                );
                assert!(
                    bodies.contains(&STALE_STAMP),
                    "{label}: the stamp's pre-image must survive verbatim; got {parked:?}",
                );

                // One finding per conflicted path, each naming BOTH copies.
                assert_eq!(
                    run.conflicts(),
                    2,
                    "{label}: one blocking `{CONFLICT}` per conflicted path; got:\n{}",
                    run.rendered,
                );
                assert!(
                    run.rendered
                        .contains("blocking · finalize.rollback-conflict"),
                    "{label}: the conflict must render through the house finding line; \
                     got:\n{}",
                    run.rendered,
                );
                for (name, _) in &parked {
                    assert!(
                        run.rendered.contains(name),
                        "{label}: the finding must name the parked copy `{name}` — both \
                         versions survive, so a reader must be told where each one is; \
                         got:\n{}",
                        run.rendered,
                    );
                }
                for live in [".jigc/.gitignore", ".jigc/version"] {
                    assert!(
                        run.rendered.contains(live),
                        "{label}: the finding must name the live path `{live}`; got:\n{}",
                        run.rendered,
                    );
                }
            }
        }

        // Whatever the cell, the door's own error survives the rollback intact — the
        // conflict prints beside the frame, never in place of it.
        match cell.failure {
            Failure::Hook => assert!(
                run.rendered.contains("rejected by test hook")
                    && run.rendered.contains("nothing was committed"),
                "{label}: the hook's stderr stays verbatim and the frame still prints; \
                 got:\n{}",
                run.rendered,
            ),
            Failure::Stage => assert!(
                run.rendered.contains("finalize.stage-failed"),
                "{label}: the stage failure keeps its own routed finding; got:\n{}",
                run.rendered,
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Absent, never-written, and the totality of the family
// ---------------------------------------------------------------------------

/// **Absent means absent.** When `.jigc/.gitignore` did not exist pre-finalize, the amend
/// *created* it — so the restore is a **delete**, not a rewrite. A capture modelling only
/// "present" leaves jigc's file behind and the transaction's "as if never called" claim is
/// false for exactly the state a fresh adopter is in (`milestone.rs`'s `RecordPreImage`
/// discipline, one family over).
#[test]
fn an_absent_pre_image_is_restored_by_deleting_what_jigc_created() {
    let cell = Cell {
        failure: Failure::Hook,
        race: Race::Untouched,
        note: "the absent-pre-image arm",
    };
    let run = drive("absent", None, &cell);
    assert!(
        run.read(".jigc/.gitignore").is_none(),
        "the ignore file did not exist pre-finalize, so the rollback must delete the one \
         jigc created; it holds:\n{:?}",
        run.read(".jigc/.gitignore"),
    );
    assert_eq!(
        run.read(".jigc/version").as_deref(),
        Some(STALE_STAMP),
        "the stamp's present pre-image is restored in the same pass",
    );
    assert_eq!(
        run.conflicts(),
        0,
        "an absence restored under the identical compare-and-swap rule reports nothing; \
         got:\n{}",
        run.rendered,
    );
}

/// **A path jigc never wrote this run is neither restored nor reported.** When the ignore
/// file already lists every canonical entry the amend writes *nothing* — and a rollback that
/// cannot name the bytes jigc produced has no post-image to compare against, so it must leave
/// the file alone even when it has changed. This is the distinction the fan-out arms live in:
/// `refresh_version_stamp` never runs there, and "jigc wrote identical bytes" is not the same
/// value as "jigc wrote nothing here".
#[test]
fn a_path_jigc_never_wrote_is_neither_restored_nor_reported() {
    let complete = format!("{COMMITTED_ENTRIES}displaced/\n");
    let cell = Cell {
        failure: Failure::Hook,
        race: Race::ConcurrentEdit,
        note: "the never-written arm",
    };
    let run = drive("never-written", Some(&complete), &cell);

    let ignore = run
        .read(".jigc/.gitignore")
        .expect("the ignore file survives");
    assert!(
        ignore.contains(CONCURRENT_IGNORE_LINE),
        "the hook's edit stands: jigc wrote nothing here, so there is nothing to roll back; \
         got:\n{ignore}",
    );
    assert_eq!(
        run.conflicts(),
        1,
        "exactly one conflict — the stamp's. A path jigc never wrote raises none, however \
         much it changed; got:\n{}",
        run.rendered,
    );
    let parked = run.parked();
    assert_eq!(
        parked.len(),
        1,
        "only the stamp's pre-image is parked; got {parked:?}",
    );
    assert_eq!(
        parked[0].1, STALE_STAMP,
        "and it is the stamp's, verbatim; got {parked:?}",
    );
    assert!(
        parked[0].0.starts_with("finalize/.jigc/version.pre-image."),
        "and it is parked under the door's own sub-directory, named by the entry's \
         identity; got {parked:?}",
    );
}

/// **The totality arm, driven rather than restated.** The family's subject is the two files
/// `finalize` *rewrites*, and the danger named in the Settle (§6 / A1) is a fourth
/// config-layer pathspec arriving with no disposition — or the directory being captured as if
/// it were a file, which is a different shape (absent-means-delete over N files).
///
/// So the check runs in both directions and neither end is hand-written: the pathspec set the
/// binary **actually stages** is lifted out of its own stage-failure message, and it must
/// equal the dispositioned rows of `cli::task::CONFIG_LAYER_SPECS`. A new pathspec reddens
/// here until someone says whether finalize rewrites it.
#[test]
fn every_config_layer_pathspec_carries_a_disposition() {
    for row in CONFIG_LAYER_SPECS {
        if let ConfigLayerWrite::Untouched(reason) = row.write {
            assert!(
                !reason.trim().is_empty(),
                "`{}` is declared out of the pre-image family with no reason",
                row.spec,
            );
        }
    }
    let rewritten: Vec<&str> = CONFIG_LAYER_SPECS
        .iter()
        .filter(|row| matches!(row.write, ConfigLayerWrite::Rewritten { .. }))
        .map(|row| row.spec)
        .collect();
    assert_eq!(
        rewritten,
        vec![".jigc/.gitignore", ".jigc/version"],
        "the family's subject is the two files `finalize` rewrites — and never the \
         `.jigc/config` DIRECTORY, whose pre-image shape is absent-means-delete over N files",
    );

    // The driven half: what the stage adds, read off the binary's own refusal.
    let cell = Cell {
        failure: Failure::Stage,
        race: Race::Untouched,
        note: "the totality arm's witness",
    };
    let run = drive("totality", Some(COMMITTED_ENTRIES), &cell);
    let marker = "`git add -- ";
    let at = run.rendered.find(marker).unwrap_or_else(|| {
        panic!(
            "the stage failure must quote its own argv:\n{}",
            run.rendered
        )
    });
    let rest = &run.rendered[at + marker.len()..];
    let argv = &rest[..rest.find('`').expect("the quoted argv is closed")];
    // Each pathspec reaches git as `:(literal)<path>` — the name, never a pattern (M55
    // Increment 1, `cli::task::literal_pathspec`) — so the set compared is the paths under
    // that magic, and a bare one is a pathspec that escaped it.
    let staged: Vec<&str> = argv
        .split_whitespace()
        .map(|spec| {
            spec.strip_prefix(":(literal)").unwrap_or_else(|| {
                panic!(
                    "the stage hands git every path as `:(literal)`; `{spec}` is bare:\n{}",
                    run.rendered
                )
            })
        })
        .collect();
    let declared: Vec<&str> = CONFIG_LAYER_SPECS.iter().map(|row| row.spec).collect();
    assert_eq!(
        staged, declared,
        "the config-layer pathspecs the stage adds must be exactly the dispositioned rows — \
         a pathspec the stage grew with no row is a file the rollback does not know it \
         wrote; the run printed:\n{}",
        run.rendered,
    );
}

// ---------------------------------------------------------------------------
// The park identity discriminates (M52 Increment 5 / T2)
// ---------------------------------------------------------------------------

/// **Two entries sharing a basename park distinctly, and the route names each one.**
///
/// The shipped M51 park was `<basename>.pre-image.<nanos>` in one flat `.jigc/displaced/`.
/// The two config-layer rows have different basenames, so the corpus was safe by accident of
/// shape — and the moment a second `FileCas` population joins (promote, retire, the milestone
/// record, `rename`), two raced paths called `notes.md` in different directories park as two
/// files a reader cannot tell apart, in a directory shared with every other door's parks.
///
/// **Manufactured, and this arm says so.** No registry carries "two populations that share a
/// basename": it is a property of a *corpus*, so there is nothing to enumerate. The cell is
/// built directly on the seam the driven cells above reach through the binary — the same
/// [`PreImageFamily`], the same shipped [`FINALIZE_DOOR`] metadata, so the bytes asserted here
/// are the bytes an operator reads.
///
/// **Red under the shipped park** (applied as a mutant: `park` reverted to
/// `<jigc_root>/displaced/<basename>.pre-image.<nanos>`): both copies land as
/// `notes.md.pre-image.<nanos>` beside each other, so the `alpha/` assertion below fails —
/// the files are two, and which live path either came from is unrecoverable.
#[test]
fn two_entries_sharing_a_basename_park_distinctly_and_name_both() {
    let dir = TempDir::new("basename");
    let repo = dir.path();
    let jigc = repo.join(".jigc");
    fs::create_dir_all(&jigc).expect("create the workbench root");

    let sides = ["alpha", "beta"];
    let mut family = PreImageFamily::empty(FINALIZE_DOOR);
    for side in sides {
        fs::create_dir_all(repo.join(side)).expect("create the side directory");
        let live = repo.join(side).join("notes.md");
        fs::write(&live, format!("{side}: the pre-image\n")).expect("write the pre-image");
        let identity = format!("{side}/notes.md");
        family.push(PreImage::capture(identity.clone(), live.clone()).expect("capture"));
        // jigc writes, and reports what it wrote one statement later.
        fs::write(&live, format!("{side}: jigc wrote this\n")).expect("write jigc's bytes");
        family.wrote(&identity);
        // …and a third party writes over it before the rollback runs.
        fs::write(&live, format!("{side}: a third party wrote this\n")).expect("the race");
    }

    let conflicts = family.restore(repo, &jigc);
    assert_eq!(
        conflicts.len(),
        2,
        "one conflict per raced path, never one `(code, null)` for the rollback",
    );

    for side in sides {
        let live = repo.join(side).join("notes.md");
        assert_eq!(
            fs::read_to_string(&live).expect("the live file survives"),
            format!("{side}: a third party wrote this\n"),
            "the racer's bytes are not overwritten — that is the same loss the rollback \
             exists to prevent, in the other direction",
        );
        let home = jigc.join("displaced").join("finalize").join(side);
        let parked: Vec<(String, String)> = fs::read_dir(&home)
            .unwrap_or_else(|err| {
                panic!(
                    "`{}` must hold `{side}`'s parked copy: {err}",
                    home.display()
                )
            })
            .flatten()
            .map(|entry| {
                (
                    entry.file_name().to_string_lossy().into_owned(),
                    fs::read_to_string(entry.path()).unwrap_or_default(),
                )
            })
            .collect();
        assert_eq!(
            parked.len(),
            1,
            "`{side}`'s pre-image parks alone under its own identity — the shipped flat \
             `<basename>.pre-image.<nanos>` put both here and named neither; got {parked:?}",
        );
        assert!(
            parked[0].0.starts_with("notes.md.pre-image."),
            "…named for the file it came from; got {parked:?}",
        );
        assert_eq!(
            parked[0].1,
            format!("{side}: the pre-image\n"),
            "…and holding that side's own bytes, verbatim",
        );

        // And the route names THIS side's park, so the two findings are followable apart.
        let expected = format!(".jigc/displaced/finalize/{side}/{}", parked[0].0);
        let matching: Vec<&engine::finding::Finding> = conflicts
            .iter()
            .filter(|finding| {
                finding
                    .route
                    .as_ref()
                    .is_some_and(|route| route.as_str().contains(&expected))
            })
            .collect();
        assert_eq!(
            matching.len(),
            1,
            "exactly one finding must name `{expected}` — a conflict that preserves both \
             copies is only useful to a reader told where the second one is, and two \
             findings naming one park is the flat shape's failure wearing a new path; \
             routes: {:?}",
            conflicts
                .iter()
                .map(|f| f.route.as_ref().map(|r| r.as_str().to_owned()))
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            matching[0].code, FINALIZE_DOOR.code,
            "…under the door's own injected code",
        );
    }
}

// ---------------------------------------------------------------------------
// The inventory registers exactly what this increment mints (T4)
// ---------------------------------------------------------------------------

/// The doc whose Severity inventory is the home the wave's Settle named for registering a
/// minted finding (`completions/artifacts/M51/settle-record.md` → §10).
const INVENTORY_DOC: &str = "design/validation.md";

/// The heading this increment registers under. **Keyed per increment on purpose**: each
/// increment's arm asserts *equality* over the codes it mints, and one shared M51 section
/// would redden every sibling arm the moment the next increment registered its own.
const INVENTORY_HEADING: &str = "### The M51 registrations — Increment 4: the rollback-conflict";

/// The codes **this increment** mints — one — spelled here so the three links below can be
/// compared against each other rather than each against the reader's memory:
///
/// 1. the registration table's **first column** in [`INVENTORY_DOC`], as a SET;
/// 2. **what the binary emits**, driven — the codes the `ConcurrentEdit` cell renders that
///    the identically-failing `Untouched` cell does not, which is this family's whole
///    contribution to that door's surface. A driven difference rather than a code-side
///    registry read, because this family's production-side set is *the rows of
///    `CONFIG_LAYER_SPECS` the transaction rewrites*, not a member enum of codes — so the
///    honest question is what the door prints when a path races, and the honest baseline is
///    the same door failing the same way with nothing racing;
/// 3. **none of them an [`cli::invocation_log::ERROR_CODE_REGISTRY`] member** (§10): that
///    registry mirrors **door identities** derived from `COMMITTING_DOORS`, and a blocking
///    `Finding` is not an `Outcome` identity — registering one there would file a finding in
///    a set whose own fence is the doc mirror of something else.
const INCREMENT_CODES: [&str; 1] = ["finalize.rollback-conflict"];

/// The repository root, two levels above `crates/cli`.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the repo root sits two levels above crates/cli")
        .to_path_buf()
}

/// The block of [`INVENTORY_DOC`] under [`INVENTORY_HEADING`], up to the next heading of the
/// same or a higher level — the section, never the rest of the file.
fn registration_section(doc: &str) -> &str {
    let start = doc.find(INVENTORY_HEADING).unwrap_or_else(|| {
        panic!(
            "{INVENTORY_DOC} must carry the section `{INVENTORY_HEADING}` — this increment \
             mints a blocking finding, and `validation.md`'s Severity inventory is the home \
             the wave's Settle named for registering one (settle-record.md → §10)",
        )
    });
    let body = &doc[start + INVENTORY_HEADING.len()..];
    let end = body
        .match_indices('\n')
        .map(|(at, _)| at + 1)
        .find(|at| body[*at..].starts_with("## ") || body[*at..].starts_with("### "))
        .unwrap_or(body.len());
    &body[..end]
}

/// Every code the registration table's **first column** names, as a set.
fn registered_codes(section: &str) -> std::collections::BTreeSet<String> {
    section
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|'))
        // The header row and the `|---|` separator carry no backticked code, so they drop out
        // of the flat_map below without being named here.
        .filter_map(|line| line.trim_start_matches('|').split('|').next())
        .flat_map(|first_cell| {
            first_cell
                .split('`')
                .skip(1)
                .step_by(2)
                .map(str::trim)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Every finding code a run **rendered through the house finding line** — `<severity> ·
/// <code> — <message>`, whatever indent the door framed it in.
fn rendered_codes(text: &str) -> std::collections::BTreeSet<String> {
    text.lines()
        .filter_map(|line| {
            let (severity, rest) = line.trim_start().split_once(" · ")?;
            // `blocking (gates at finalize)` is the store view's label and reaches no door
            // here; the three bare tokens are what a task-scope surface prints.
            matches!(severity, "blocking" | "warning" | "advisory").then_some(rest)
        })
        .filter_map(|rest| rest.split_whitespace().next())
        .filter(|code| code.contains('.'))
        .map(str::to_owned)
        .collect()
}

/// **The inventory names exactly the codes this increment's binary emits, and none of them
/// joins the door-identity registry** (`settle-record.md` → §10; the roadmap's Increment 4 →
/// *Codes it registers*).
///
/// Red at this task's start at the section-absent panic rather than at the equality: T3 had
/// already minted the code, wired it to every door that commits through the shared executor,
/// and registered it in `cli::render::FINALIZE_FAMILY` with its contract sub-table row — and
/// `validation.md`'s inventory, which calls itself the single source of truth for what the
/// engine emits, named it nowhere.
///
/// It is the one mechanical check on a task whose other deliverable is prose: `finalize.md`'s
/// reconciliation states *which sentence is the promise*, and a byte-assert over a promise's
/// wording pins editorial phrasing rather than a contract — so that locus is verified by
/// reading, the bound Increment 1 / T9 recorded and Increment 2 / T6 applied again.
#[test]
fn the_finding_inventory_registers_exactly_this_increments_codes() {
    let root = repo_root();
    let doc = std::fs::read_to_string(root.join(INVENTORY_DOC))
        .unwrap_or_else(|err| panic!("read {INVENTORY_DOC}: {err}"));
    let section = registration_section(&doc);

    // 1 — doc == these literals, as a SET.
    let registered = registered_codes(section);
    let expected: std::collections::BTreeSet<String> = INCREMENT_CODES
        .iter()
        .map(|code| (*code).to_owned())
        .collect();
    assert_eq!(
        registered, expected,
        "{INVENTORY_DOC} → `{INVENTORY_HEADING}` must register EXACTLY the codes this \
         increment mints — one table row per code, the code alone in the first column. A row \
         for a code the binary never emits is a lie on the inventory that calls itself *the \
         single source of truth* for what the engine emits; a missing row is the silence §10 \
         exists to end.\nsection read:\n{section}",
    );

    // 2 — these literals == what the binary emits, driven in BOTH directions: the codes the
    // raced rollback adds to a door's surface, over the baseline of the same door failing the
    // same way with nothing racing. A code this family mints and the doc omits reddens link 1;
    // a code it mints beyond this set reddens here.
    let hook = |race, tag: &str| {
        drive(
            tag,
            Some(COMMITTED_ENTRIES),
            &Cell {
                failure: Failure::Hook,
                race,
                note: "the inventory arm's witness",
            },
        )
    };
    let baseline = rendered_codes(&hook(Race::Untouched, "inventory-baseline").rendered);
    let raced = rendered_codes(&hook(Race::ConcurrentEdit, "inventory-raced").rendered);
    let minted: std::collections::BTreeSet<String> = raced.difference(&baseline).cloned().collect();
    assert_eq!(
        minted, expected,
        "the literals this arm spells must be exactly the codes the raced rollback adds to \
         the door's surface — one refused restore, one code, and nothing else new. A second \
         code minted here with no inventory row reddens as loudly as a doc row with nothing \
         behind it.\nbaseline: {baseline:?}\nraced: {raced:?}",
    );

    // …and the spelling is the shipped registry's, not a second copy of it: the family table
    // is what `finalize_family_registry.rs` scans production source against, so a literal that
    // drifts from it is a literal no fence is holding.
    for code in INCREMENT_CODES {
        assert!(
            cli::render::FINALIZE_FAMILY
                .iter()
                .any(|member| member.code == code),
            "`{code}` must be a `cli::render::FINALIZE_FAMILY` member — that table is the \
             enumeration the registry suite scans production source against, and the contract's \
             `finalize.*` sub-table mirrors it",
        );
    }

    // 3 — and none of them joins the door-identity registry (§10).
    for code in INCREMENT_CODES {
        assert!(
            !cli::invocation_log::ERROR_CODE_REGISTRY.contains(&code),
            "`{code}` must stay OUT of `ERROR_CODE_REGISTRY`: that registry mirrors door \
             identities derived from `COMMITTING_DOORS`, and a blocking `Finding` is not an \
             `Outcome` identity (settle-record.md → §10)",
        );
    }
    assert!(
        section.contains("ERROR_CODE_REGISTRY"),
        "…and the section must SAY so — the reason a blocking finding is not a door identity \
         is the half a reader cannot derive from the table, and §10 decided it once for the \
         whole family",
    );

    // …and that both versions survive, naming where the second one went. That is the half a
    // reader cannot derive from a code called *conflict*: the rollback did not restore, and it
    // did not discard either — the pre-image is parked in the gitignored workbench, and a
    // reader who is not told the path has no way to find the bytes.
    assert!(
        section.contains(".jigc/displaced/"),
        "…and the section must name `.jigc/displaced/`, where the pre-image the restore \
         refused to write is parked — a conflict that preserves both copies is only useful to \
         someone told where the second one is",
    );
}
