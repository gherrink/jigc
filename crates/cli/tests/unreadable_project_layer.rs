//! M52 Increment 8 / T2 — **an unreadable `.jigc/` names the read fault instead of
//! calling the project not set up.**
//!
//! ## The class this closes
//!
//! `completions/artifacts/M52/baseline-contracts.md` §4 LD-4, driven at HEAD over
//! `dev/jigc-rig fresh` with `chmod 000 .jigc`: `jigc doc list` and `jigc validate` both
//! exited 1 saying *"this project isn't set up — run `jigc setup` (no `.jigc/config/`
//! cascade layer found)"*. The project **is** set up; the layer is unreadable. The route
//! it printed is followable and would fail for the same reason, which makes it a law-1
//! lie of the purest kind — the surface states a fact the binary can disprove with the
//! same `stat` it just made (`design/surface-contract.md` → law 1).
//!
//! The cause is a **reduction**, not a door: `.is_dir()` answers `false` for *absent* and
//! for *unreadable* alike, and every door that requires the project layer reads that one
//! boolean. So the fix lands at the **producer of the answer** —
//! [`cli::locate::not_set_up`], which now takes the path it is about and asks `metadata`
//! which of the two states it is in — and every leaf sharing the constructor shares the
//! correction (`completions/artifacts/M52/settle-record.md` §13: *"LD-4's 'not set up'
//! lie over an unreadable `.jigc/` is fixed at the same producer"*).
//!
//! **No code is minted.** The increment's ledger declares exactly one
//! (`file-state.absorbed`, T3's); this stays the `{error}` arm at exit 1, which is the
//! answer 21 of the 47 leaves already give in the neighbouring state.
//!
//! **The route is a `Human` route.** No `jigc` argv repairs an unreadable directory —
//! the move is the filesystem's, so the constructor is not contorted into claiming a
//! mechanical conversion (`design/surface-contract.md` → The route fence, the same
//! judgment [`cli::locate::not_in_repo_route`] already ships for its own axis).
//!
//! ## Which arms the corrected producer reaches, and the one it does not
//!
//! Reached: every door that answers through [`cli::locate::not_set_up`] — the five
//! production call sites (`cli::require_project_layer` ▸ `validate`,
//! `ingest::require_project_layer` ▸ `doc list`/`ingest`,
//! `migrate_corpus::require_project_layer` ▸ the nine `milestone` doors and
//! `migrate-corpus`, `start::require_project_config` ▸ the compose paths,
//! plus `describe` and `migrate`). Three of them are driven below, one per call site
//! that a `--format json` `{error}` arm can be read off.
//!
//! **Written bound — the arm the producer does not reach:** bare `jigc start`'s
//! orientation does not call the constructor at all. `crate::orient` reads
//! `ctx.project_config` and returns `OrientationView::UnsetProject` at **exit 0** for
//! `None`, so over an unreadable `.jigc/` it still reports *unset-project* (driven at
//! HEAD: `{"state":"unset-project","schema_version":3}`, exit 0). Correcting it means a
//! second decision site or a fourth `OrientationView` variant — a pinned-envelope move
//! and new capability, which this wave's boundary (*fixes + understandability only, no
//! new functionality*) does not carry. It is stated here rather than left to be
//! rediscovered.
//!
//! ## The arms
//!
//! Per door, through the **real binary**:
//!
//! 1. **unreadable** — exit 1, the `{error}` arm names the read fault, byte-compared
//!    against the shared constructor rather than a copy typed here, and contains the
//!    not-set-up sentence's stem **nowhere**;
//! 2. **absent** — the control: the shipped sentence survives **byte-for-byte**, which is
//!    what makes arm 1 a discrimination rather than a replacement.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::support::trial_corpus::{State, TrialCorpus, unique_root};

/// The stem of the sentence an unreadable layer must **not** print — the fact the
/// baseline caught being asserted over a project that is set up.
const NOT_SET_UP_STEM: &str = "isn't set up";

/// The doors driven, one per production call site of the constructor that answers on a
/// readable `--format json` `{error}` arm: `validate` (`cli::require_project_layer`),
/// `doc list` (`ingest::require_project_layer`) and one `milestone` leaf
/// (`migrate_corpus::require_project_layer`, M52 Inc 8 / T1's door class).
const DOORS: &[&[&str]] = &[
    &["validate"],
    &["doc", "list"],
    &["milestone", "create", "read-fault-probe"],
];

/// Set `dir`'s permission bits, best-effort — the `PermissionsExt` idiom several shipped
/// suites use (`leftover_probe_fail_closed.rs`, `setup_install_pathspec_guard.rs`).
///
/// Best-effort rather than `expect`-ing, because the restore half runs from a `Drop` that
/// may execute during unwind, where a second panic aborts the process and buries the
/// assertion that actually failed. The *arming* half is checked by the caller instead —
/// against the state it wanted, not against the syscall's return.
fn set_mode(dir: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    let Ok(meta) = fs::metadata(dir) else {
        return;
    };
    let mut perms = meta.permissions();
    perms.set_mode(mode);
    let _ = fs::set_permissions(dir, perms);
}

/// The `{error}` string a `--format json` refusal carries, or a panic naming what came
/// back instead.
fn error_arm(out: &Output, label: &str) -> String {
    let stderr = String::from_utf8_lossy(&out.stderr);
    let value: serde_json::Value = serde_json::from_str(stderr.trim()).unwrap_or_else(|err| {
        panic!(
            "`jigc {label}` must answer on the `--format json` envelope; parsing stderr \
             failed ({err}).\nstderr: {stderr}\nstdout: {}",
            String::from_utf8_lossy(&out.stdout),
        )
    });
    value
        .get("error")
        .and_then(|e| e.as_str())
        .unwrap_or_else(|| panic!("`jigc {label}` must answer on the `{{error}}` arm; got {value}"))
        .to_string()
}

/// A set-up corpus whose `.jigc/` is unreadable for the duration of the closure, with the
/// mode restored afterwards **whatever happens** — [`TrialCorpus`]'s `Drop` removes the
/// root, and a `000` directory cannot be removed.
fn with_unreadable_jigc(corpus: &TrialCorpus, body: impl FnOnce()) {
    struct Restore(PathBuf);
    impl Drop for Restore {
        fn drop(&mut self) {
            set_mode(&self.0, 0o755);
        }
    }
    let jigc_dir = corpus.repo().join(".jigc");
    set_mode(&jigc_dir, 0o000);
    let _restore = Restore(jigc_dir.clone());
    // The fixture is checked against the **state** it wanted, never against the chmod's
    // return: as root the mode change succeeds and the traversal still resolves, and a
    // suite that only asked the syscall would then pass vacuously over a readable layer.
    assert!(
        fs::metadata(jigc_dir.join("config")).is_err(),
        "the fixture did not make `.jigc/config` unreadable — every arm below would be \
         driven against a readable layer (running as root?)",
    );
    body();
}

#[test]
fn an_unreadable_project_layer_names_the_read_fault_at_every_door() {
    let corpus = TrialCorpus::build(State::Fresh);
    let project_config = corpus.repo().join(".jigc").join("config");

    with_unreadable_jigc(&corpus, || {
        // The expected text is composed by the production constructor, in this process,
        // against the same unreadable path the binary read — so the assertion is about
        // the producer, never about a sentence retyped here.
        let expected = format!("{:#}", cli::locate::not_set_up(&project_config));
        assert!(
            !expected.contains(NOT_SET_UP_STEM),
            "the constructor must not call an unreadable layer absent; it reads: {expected}",
        );

        for door in DOORS {
            let label = door.join(" ");
            let mut argv = vec!["--format", "json"];
            argv.extend_from_slice(door);
            let out = corpus.jigc(&argv);

            assert_eq!(
                out.status.code(),
                Some(1),
                "`jigc {label}` over an unreadable `.jigc/` exits 1.\nstderr: {}\nstdout: {}",
                String::from_utf8_lossy(&out.stderr),
                String::from_utf8_lossy(&out.stdout),
            );
            let arm = error_arm(&out, &label);
            assert_eq!(
                arm, expected,
                "`jigc {label}` gives the shared read-fault answer byte-for-byte",
            );
            assert!(
                !arm.contains(NOT_SET_UP_STEM),
                "`jigc {label}` must not tell a set-up project it is not set up; it read: {arm}",
            );
        }
    });
}

#[test]
fn an_absent_project_layer_keeps_the_shipped_not_set_up_sentence() {
    let bare = BareRepo::build();
    let project_config = bare.repo().join(".jigc").join("config");
    let expected = format!("{:#}", cli::locate::not_set_up(&project_config));
    assert!(
        expected.contains(NOT_SET_UP_STEM),
        "the absent-layer control must keep the shipped sentence; it reads: {expected}",
    );

    for door in DOORS {
        let label = door.join(" ");
        let mut argv = vec!["--format", "json"];
        argv.extend_from_slice(door);
        let out = bare.drive(&argv);
        assert_eq!(
            out.status.code(),
            Some(1),
            "`jigc {label}` over a repository with no `jigc setup` exits 1.\nstderr: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        assert_eq!(
            error_arm(&out, &label),
            expected,
            "`jigc {label}` keeps the shipped not-set-up sentence byte-for-byte",
        );
    }
}

/// A git repository with one commit and **no `jigc setup`** — the absent-layer control.
/// Built here for the same reason `milestone_not_set_up_axis.rs` builds one:
/// `support::trial_corpus::State::ALL` deliberately carries no un-set-up state.
struct BareRepo {
    root: PathBuf,
}

impl BareRepo {
    fn build() -> Self {
        let root = unique_root("unreadable-layer-absent-control");
        fs::create_dir_all(root.join("repo")).expect("create the bare repo dir");
        fs::create_dir_all(root.join("home")).expect("create the bare home dir");
        let bare = BareRepo { root };
        bare.git(&["init", "-q"]);
        bare.git(&["config", "user.email", "bare@example.com"]);
        bare.git(&["config", "user.name", "Bare Repo"]);
        bare.git(&["config", "commit.gpgsign", "false"]);
        fs::write(bare.repo().join("README.md"), "bare repo\n").expect("write README.md");
        bare.git(&["add", "."]);
        bare.git(&["commit", "-q", "-m", "initial"]);
        assert!(
            !bare.repo().join(".jigc").exists(),
            "the absent-layer control must have no `.jigc/`",
        );
        bare
    }

    fn repo(&self) -> PathBuf {
        self.root.join("repo")
    }

    fn git(&self, args: &[&str]) {
        let out = Command::new("git")
            .arg("-C")
            .arg(self.repo())
            .args(args)
            .env("HOME", self.root.join("home"))
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    }

    fn drive(&self, argv: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(argv)
            .current_dir(self.repo())
            .env("HOME", self.root.join("home"))
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("spawn jigc")
    }
}

impl Drop for BareRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
