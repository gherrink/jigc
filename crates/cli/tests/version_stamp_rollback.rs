//! Milestone-audit fix (M45) — the **config-layer rollback axis**, the sibling the M45
//! Inc 8 owner-artifact axis left un-swept. Since M30/M40, every non-migration/migration
//! `jigc task finalize` **stages jigc's own config layer** into the index — the committed
//! `.jigc/config/` + `.jigc/.gitignore`, and the `.jigc/version` binary-provenance stamp
//! (which `refresh_version_stamp` rewrites to the running build before staging). But the
//! shared `Err` rollback arm restored only promotions/retirements (`rollback_promotions`)
//! and owner-artifact paths (`rollback_owner_artifact_index`) — **never the version-stamp /
//! config index entries**. So a hook-rejected finalize left `.jigc/version` (and a
//! first-finalize `.jigc/.gitignore`) staged, violating the "any failure before `git commit`
//! rolls back to as-if-never-called" atomicity claim (`design/finalize.md` → the transaction
//! is atomic up through `git commit`).
//!
//! The proof, through the real `jigc` binary: a repo whose committed `.jigc/version` carries
//! **stale bytes** (so `refresh_version_stamp` stages a genuinely different entry) runs a
//! finalize a rejecting `.git/hooks/pre-commit` aborts. After rollback the **index is
//! byte-identical to its pre-finalize state** — `.jigc/version`'s staged blob is back to the
//! stale bytes (not jigc's refreshed overwrite), and the freshly-`ensure`d `.jigc/.gitignore`
//! is not left staged. Red before the axis; green after. Mirrors the `owner_artifact_rollback.rs`
//! mold (real binary via `CARGO_BIN_EXE_jigc`, self-cleaning temp repo).

#![cfg(unix)]

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
            "jigc-version-rollback-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success, returning raw stdout as a String.
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

/// The whole index as `<mode> <sha> <stage>\t<path>` lines — the pre/post comparison key.
fn index_snapshot(repo: &Path) -> String {
    git(repo, &["ls-files", "--stage"])
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer AND a
/// committed `.jigc/version` carrying **stale** bytes — so the finalize's `refresh_version_stamp`
/// stages a genuinely different blob (the leak the rollback must undo).
fn init_repo(repo: &Path, stale_stamp: &str) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(repo.join(".jigc").join("version"), stale_stamp).expect("write stale stamp");
    git(repo, &["add", "README.md", ".jigc/version"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
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

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Fill the provisioned commit doc so a finalize over it validates clean — leaving the
/// config-layer staging + rollback as the only lever the test turns.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
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
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");
}

/// Install a `.git/hooks/pre-commit` that always rejects (exit 1) so `git commit` aborts.
fn install_rejecting_pre_commit(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        "#!/bin/sh\necho 'rejected by test hook' 1>&2\nexit 1\n",
    )
    .expect("write pre-commit hook");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&hook, perms).expect("chmod hook");
}

/// Where the promoted ADR lands on disk: the `adr` schema's `decisions/` nested under the
/// default `docs-root` knob (`docs/`).
fn adr_destination(slug: &str) -> String {
    format!("docs/decisions/{slug}.md")
}

/// A hook-rejected `jigc task finalize` must leave the index **byte-identical to its
/// pre-finalize state** — including the jigc-staged config layer. Two members of that axis,
/// one red assertion each without the fix: the **stale `.jigc/version`** whose staged blob
/// jigc's `refresh_version_stamp` overwrote (present-but-differing → the rollback must restore
/// the stale blob, not leave the refreshed one staged), and the freshly-`ensure`d
/// **`.jigc/.gitignore`** the stage newly added to the index (absent pre-finalize → the
/// rollback must drop it, not leave it staged).
#[test]
fn a_hook_rejected_finalize_restores_the_staged_config_layer() {
    let repo = TempDir::new("cfg");
    let home = TempDir::new("home");
    // Bytes no real `jigc` build stamps — so `refresh_version_stamp` is guaranteed to stage a
    // different `.jigc/version` blob, making the leak observable.
    let stale_stamp = "0.0.0-fixture-stale-stamp\n";
    init_repo(repo.path(), stale_stamp);

    // Precondition: the committed stamp is the stale bytes, and `.jigc/.gitignore` is untracked
    // (the finalize's `gitignore::ensure` writes it and the stage adds it).
    let staged_stamp_before = git(repo.path(), &["show", ":.jigc/version"]);
    assert_eq!(
        staged_stamp_before, stale_stamp,
        "precondition: the staged `.jigc/version` is the stale bytes going in"
    );
    let index_at_init = index_snapshot(repo.path());
    assert!(
        !index_at_init.contains(".jigc/.gitignore"),
        "precondition: `.jigc/.gitignore` is not tracked before the finalize; got:\n{index_at_init}"
    );

    let intent = "cache sessions in a single in-memory node";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", intent],
        ),
        "`jigc start`",
    );
    let task = "cache-sessions-in-a-single";
    fill_commit(repo.path(), home.path(), task);

    // Stage a real code change so the finalize proceeds to the stage/commit phase (an empty
    // index blocks earlier on `finalize.nothing-staged`, never reaching the config-layer stage).
    fs::write(repo.path().join("README.md"), "hello world\n").expect("edit README");
    git(repo.path(), &["add", "README.md"]);

    // Re-snapshot the pre-finalize index now that the agent has staged its code — this is the
    // exact state the rollback must return the index to.
    let index_before = index_snapshot(repo.path());

    install_rejecting_pre_commit(repo.path());

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "a rejecting pre-commit must make finalize exit non-zero; got:\n{rendered}",
    );

    // No commit landed.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a hook-rejected finalize creates no commit");

    // The staged `.jigc/version` blob is byte-identical to the pre-finalize stale bytes — the
    // rollback restored it, NOT jigc's refreshed overwrite.
    let staged_stamp_after = git(repo.path(), &["show", ":.jigc/version"]);
    assert_eq!(
        staged_stamp_after, stale_stamp,
        "after rollback `git show :.jigc/version` must equal the pre-finalize stale bytes, got:\n{staged_stamp_after}",
    );

    // The freshly-`ensure`d `.jigc/.gitignore` must NOT be left staged (it was absent from the
    // index pre-finalize).
    let index_after = index_snapshot(repo.path());
    assert!(
        !index_after.contains(".jigc/.gitignore"),
        "the newly-staged `.jigc/.gitignore` must be dropped from the index on rollback; got:\n{index_after}"
    );

    // The strong form: the whole index is byte-identical to its pre-finalize state — every
    // path jigc's own staging contributed is rolled back, nothing else touched.
    assert_eq!(
        index_after, index_before,
        "the index must be byte-identical to its pre-finalize state after a rejected finalize",
    );

    // **The declaration is reversed at M51 Increment 4, and these two assertions are its
    // reversal** (`design/finalize.md` → Rollback discipline; `settle-record.md` → §6 / D4).
    // The confidence audit's minor item 3 declared the worktree residue acceptable on the
    // ground that both files are jigc-owned maintenance the next store-writing op rewrites
    // identically — which is false of `.jigc/.gitignore`, a file the **user co-owns**: the
    // replace-era writer destroyed an uncommitted private line there at exit 0 with the bytes
    // in no git object, and even after the amend the transaction left an unasked-for
    // modification standing while its own frame said *"nothing was committed"*. So the family
    // gained a worktree axis and the two files are restored — compare-and-swap, so a
    // concurrent edit is preserved rather than overwritten.
    //
    // The stamp is restored to its pre-finalize bytes…
    let worktree_stamp =
        fs::read_to_string(repo.path().join(".jigc").join("version")).expect("read the stamp");
    assert_eq!(
        worktree_stamp, stale_stamp,
        "the worktree `.jigc/version` must be restored to its pre-finalize bytes — a refused \
         transaction leaves no rewrite of its own standing",
    );
    // …and the `.gitignore` the amend CREATED is deleted, because its pre-image is *absent*
    // and an absence restores under the identical rule.
    assert!(
        !repo.path().join(".jigc").join(".gitignore").exists(),
        "`.jigc/.gitignore` did not exist pre-finalize, so the rollback must delete the one \
         this transaction created",
    );
}

/// The **promotions rollback axis** joins the same pre-finalize index capture/restore
/// discipline (confidence-audit sibling-hunt item 2 — the axis-3 failure shape, one axis
/// over): a blob the user stages at a promoted destination path **mid-task** (post-mint, so
/// outside the carryover snapshot; worktree copy removed, so the pre-promotion clobber guard
/// — which covers the worktree-file shape — does not fire) must survive a hook-rejected
/// finalize. The old primitive (`git restore --staged --worktree <dest>`) reset the
/// destination to **HEAD** — for a path new at HEAD that *drops the index entry entirely*,
/// silently destroying the user's staged blob. After rollback the staged blob must be
/// byte-identical to the pre-finalize state, the promoted worktree copy gone (the
/// pre-finalize worktree had no file there), and the whole index byte-identical.
#[test]
fn a_hook_rejected_finalize_restores_a_staged_blob_at_a_promotion_destination() {
    let repo = TempDir::new("promo");
    let home = TempDir::new("home");
    let stale_stamp = "0.0.0-fixture-stale-stamp\n";
    init_repo(repo.path(), stale_stamp);

    let intent = "record the cache decision";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", intent],
        ),
        "`jigc start`",
    );
    let task = "record-the-cache-decision";

    // Create + author an ADR in-task (the create-gate: single-task admits `adr` as
    // `decision`) — the doc finalize will PROMOTE to `docs/decisions/<slug>.md`.
    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Cache in one node"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr, "adr:cache-in-one-node", "the created adr address");
    let dest = adr_destination("cache-in-one-node");

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo.path(),
            home.path(),
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_slot("adr:cache-in-one-node#context", b"Sessions need a cache.\n");
    set_slot("adr:cache-in-one-node#decision", b"One in-memory node.\n");
    set_slot(
        "adr:cache-in-one-node#consequences",
        b"No cross-node state.\n",
    );
    fill_commit(repo.path(), home.path(), task);

    // MID-TASK (after mint — outside the carryover snapshot), the user stages blob X at the
    // very destination the finalize will promote to, then removes the worktree copy. The
    // index now holds X at `docs/decisions/cache-in-one-node.md`; the worktree has no file
    // there, so the pre-promotion clobber guard (which keys on a worktree file) passes and
    // the finalize reaches the stage/commit phase — where jigc's `git add <dest>` overwrites
    // the entry with the promotion blob.
    let blob_x = "user-staged draft at the destination — pre-finalize blob X\n";
    fs::create_dir_all(repo.path().join("docs/decisions")).expect("mk destination dir");
    fs::write(repo.path().join(&dest), blob_x).expect("write blob X");
    git(repo.path(), &["add", &dest]);
    fs::remove_file(repo.path().join(&dest)).expect("remove the worktree copy");
    let staged_before = git(repo.path(), &["show", &format!(":{dest}")]);
    assert_eq!(
        staged_before, blob_x,
        "precondition: the staged blob at the destination is X going in"
    );

    // This is the exact state the rollback must return the index to.
    let index_before = index_snapshot(repo.path());

    install_rejecting_pre_commit(repo.path());

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "a rejecting pre-commit must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("rejected by test hook"),
        "the finalize must have reached the commit phase (the hook is the rejector), not \
         blocked earlier; got:\n{rendered}",
    );

    // No commit landed.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a hook-rejected finalize creates no commit");

    // The staged blob at the promotion destination is byte-identical to the pre-finalize X —
    // NOT reset to HEAD (which, the path being new at HEAD, drops the entry and destroys X),
    // NOT left as jigc's promotion overwrite.
    let show = Command::new("git")
        .args(["show", &format!(":{dest}")])
        .current_dir(repo.path())
        .output()
        .expect("run git show");
    assert!(
        show.status.success(),
        "after rollback the destination's index entry must still exist — the old primitive \
         dropped it to HEAD (i.e. to nothing); stderr:\n{}",
        String::from_utf8_lossy(&show.stderr),
    );
    let staged_after = String::from_utf8(show.stdout).expect("utf-8");
    assert_eq!(
        staged_after, blob_x,
        "after rollback `git show :{dest}` must equal the pre-finalize blob X",
    );

    // The promoted worktree copy is rolled back (the pre-finalize worktree had no file there).
    assert!(
        !repo.path().join(&dest).exists(),
        "the promoted copy at {dest} must be removed on rollback",
    );

    // The strong form: the whole index is byte-identical to its pre-finalize state.
    let index_after = index_snapshot(repo.path());
    assert_eq!(
        index_after, index_before,
        "the index must be byte-identical to its pre-finalize state after a rejected finalize",
    );
}
