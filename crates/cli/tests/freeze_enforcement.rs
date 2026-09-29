//! Real-binary acceptance for the M33 **pack-load freeze gate** (T3): a
//! `JIGC_PACK_DIR` on-disk dev-pack copy whose schema *shape* drifted from the
//! shipped `config/schema-manifest.yaml` without a version bump makes a `jigc`
//! command exit **non-zero**, naming the schema-hash mismatch; an unmutated copy
//! composes clean; a manifest-less copy is unaffected (the same drift passes once
//! the freeze manifest is dropped).
//!
//! This drives the **real `jigc` binary** end-to-end — the emitted exit code +
//! stderr are the contract, not a reconstructed equivalent. It is the runtime
//! sibling of the build-time freeze gate
//! (`pack::shipped_schema_manifest_matches_the_frozen_doctype_set`); the design is
//! `design/corpus-migration.md` → The enforcement gate fires at pack-load
//! (review Finding 3) + `design/worked-examples.md` → the freeze-enforcement flow.

use clap::Parser;
use cli::cli::{VERB_KINDS, VerbKind};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-freeze-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    /// A throwaway directory **whose path carries shell metacharacters** — one space
    /// and one apostrophe, the two bytes that break an unquoted and a naively
    /// single-quoted command line respectively.
    ///
    /// Every route this file follows is `sh -c`'d verbatim, so a route that
    /// interpolates a repo-relative-to-absolute path raw runs *correctly* under
    /// [`TempDir::new`] (whose name is drawn from `std::env::temp_dir()` plus an
    /// inert `jigc-freeze-<tag>-<pid>-<nanos>` segment — never a space) and
    /// *silently wrong* here. The fixture axis is the point: an assertion that "the
    /// emitted route must run" certifies only the path shapes it is handed.
    fn new_metachar(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc freeze's {tag}-{}-{:?}",
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

/// The embedded dev pack tree — the faithful source the on-disk copy mirrors.
fn embedded_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the faithful
/// source the methodology-arm copies mirror.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Copy the dev pack into a fresh temp dir and return the copy's root.
fn dev_pack_copy(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());
    dir
}

/// Mutate the copied `adr` schema's **shape** (its `location:`) — a hash-affecting
/// change that bumps no `schema-version`, the un-migrated schema change the freeze
/// forbids. The manifest is deliberately left unbumped.
fn drift_adr_schema(pack: &Path) {
    let schema = pack.join("schemas").join("adr.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied adr.yaml");
    let drifted = body.replacen("location: decisions/", "location: adr-records/", 1);
    assert_ne!(
        body, drifted,
        "adr.yaml must declare `location: decisions/`"
    );
    fs::write(&schema, drifted).expect("write the drifted adr.yaml");
}

/// Initialize a real git repo with one commit (composition reads HEAD) plus the
/// `.jigc/config/` project layer the cascade expects.
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    // The `--from-file` payload the `doc set-slot` / `doc author` / `config` door rows
    // name. Committed, so no door of the sweep meets it as untracked noise.
    fs::write(root.join("payload.txt"), "probe prose\n").expect("write payload");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc start --workflow single-task "<intent>"` with `cwd = repo`,
/// `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_start(repo: &Path, home: &Path, pack: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "single-task", "freeze-gate probe"])
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("spawn the jigc binary")
}

/// A schema-shape change with no manifest bump is **blocked at pack-load**: the
/// composing `jigc start` exits non-zero, and stderr names the schema-hash
/// mismatch on the drifted doctype.
#[test]
fn schema_shape_drift_without_manifest_bump_is_blocked() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("drift");
    init_repo(repo.path());
    drift_adr_schema(pack.path());

    let out = run_start(repo.path(), home.path(), pack.path());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a drifted schema must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("schema-hash mismatch"),
        "stderr must name the schema-hash mismatch; got:\n{stderr}",
    );
    assert!(
        stderr.contains("adr"),
        "stderr must name the drifted `adr` doctype; got:\n{stderr}",
    );
}

/// The control: an **unmutated** dev-pack copy matches its shipped manifest, so the
/// freeze gate is inert and `jigc start` composes clean.
#[test]
fn unmutated_pack_copy_composes_clean() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("clean");
    init_repo(repo.path());

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "an unmutated dev-pack copy must compose clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Copy the methodology pack into a fresh temp dir and return the copy's root.
fn methodology_pack_copy(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&methodology_pack_tree(), dir.path());
    dir
}

/// Mutate the copied `research` schema's **shape** (its `location:`) — a hash-affecting
/// change that bumps no `schema-version`. The methodology manifest is deliberately
/// left unbumped. Mirrors the dev-pack sibling [`drift_adr_schema`] exactly.
///
/// **This used to reword a slot `hint:`** — the conflation M47's presentation projection
/// ends: it called a prose reword "a schema-shape change" while the dev arm it was
/// modelled on drifted `location:`, and once prose left the hash it stopped drifting
/// anything at all. The prose side now has its own arm
/// ([`a_reworded_methodology_hint_composes_clean_with_no_bump`]).
fn drift_research_schema(pack: &Path) {
    let schema = pack.join("schemas").join("research.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied research.yaml");
    // Anchored on the line-start key: `research.yaml` *mentions* `location: research/`
    // inside its header comment first, and an unanchored `replacen` rewrote the comment
    // and left the schema untouched — a drift function that drifts nothing (caught by
    // the gate, which is what the gate is for).
    let drifted = body.replacen(
        "\nlocation: research/\n",
        "\nlocation: investigations/\n",
        1,
    );
    assert_ne!(
        body, drifted,
        "research.yaml must declare `location: research/` as a top-level key"
    );
    fs::write(&schema, drifted).expect("write the drifted research.yaml");
}

/// Reword the copied `research` schema's `question` slot **hint** — authored prose, and
/// since M47 **outside** the frozen `schema-hash` ([`engine::manifest::schema_hash`] hashes
/// the presentation projection). No bump, no re-pin, and the gate must stay silent.
fn reword_research_hint(pack: &Path) {
    let schema = pack.join("schemas").join("research.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied research.yaml");
    let reworded = body.replacen(
        "The question this research set out to answer.",
        "The question this research set out to resolve.",
        1,
    );
    assert_ne!(body, reworded, "research.yaml must carry the question hint");
    fs::write(&schema, reworded).expect("write the reworded research.yaml");
}

/// Commit a project-layer file this fixture hand-wrote into `.jigc/`.
///
/// Since M50 Increment 4 `jigc uninstall` refuses over a file under `.jigc/` that lies
/// outside the transient `ENTRIES` prefixes and that **no index has a copy of**
/// (`uninstall.untracked-workbench-file`) — the third sole-copy subject, beside the
/// dirty fan-out worktree and the open task's staged prose. A real project's cascade
/// delta and pack list are committed files; only this fixture left them dangling, so
/// without this the `uninstall` row of [`FREEZE_DOORS`] would be answered by a fixture
/// artifact instead of by the freeze axis it is declared on.
fn commit_workbench_file(repo: &Path, rel: &str) {
    for args in [
        vec!["add", "--force", rel],
        vec!["commit", "-q", "-m", "project layer"],
    ] {
        let out = Command::new("git")
            .args(&args)
            .current_dir(repo)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} in {repo:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// Compose `[dev ▸ methodology-copy]` via the listed-pack mechanism: `packs.yaml`
/// names the on-disk copy over the embedded dev base. `JIGC_PACK_DIR` must stay
/// unset so the embedded base (not an env pack) anchors the composition.
fn list_pack(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the methodology copy");
    commit_workbench_file(repo, ".jigc/config/packs.yaml");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and NO `JIGC_PACK_DIR`
/// (the listed-pack composition path — the pack rides in `packs.yaml`).
fn run_listed(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

/// The M40 A1 headline: a **methodology** schema-shape change with no manifest bump
/// is **blocked at pack-load** — the methodology pack now ships its own
/// `config/schema-manifest.yaml`, so it is freeze-enforced exactly like the dev
/// pack (`design/corpus-migration.md` → M40 revises the dichotomy). The composing
/// `jigc start` exits non-zero and stderr names the schema-hash mismatch on the
/// drifted `research` doctype.
#[test]
fn methodology_schema_shape_drift_without_manifest_bump_is_blocked() {
    let repo = TempDir::new("m-repo");
    let home = TempDir::new("m-home");
    let pack = methodology_pack_copy("m-drift");
    init_repo(repo.path());
    list_pack(repo.path(), pack.path());
    drift_research_schema(pack.path());

    let out = run_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "freeze-gate probe"],
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a drifted methodology schema must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("schema-hash mismatch"),
        "stderr must name the schema-hash mismatch; got:\n{stderr}",
    );
    assert!(
        stderr.contains("research"),
        "stderr must name the drifted `research` doctype; got:\n{stderr}",
    );
}

/// The **invariance arm beside it** (M47 Decision 3), through the real binary: the very
/// mutation [`drift_research_schema`] used to make — a slot-`hint` reword — now composes
/// **clean** against an unbumped, un-re-pinned methodology manifest. While prose sat
/// inside the `schema-hash` the freeze was **unfenceable**: a hint reword and a real
/// structural change were the same keystroke (re-pin, green), guarded only by a comment.
/// The two arms together are the discrimination the projection buys.
#[test]
fn a_reworded_methodology_hint_composes_clean_with_no_bump() {
    let repo = TempDir::new("m-prose-repo");
    let home = TempDir::new("m-prose-home");
    let pack = methodology_pack_copy("m-prose");
    init_repo(repo.path());
    list_pack(repo.path(), pack.path());
    reword_research_hint(pack.path());

    let out = run_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "freeze-gate probe"],
    );
    assert!(
        out.status.success(),
        "a reworded authoring hint must need no bump and no re-pin; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The stamp goes live with the manifest (one atomic unit): a **fresh methodology
/// mint** through the real `do-research` workflow carries `schema-version: 1` in
/// its committed front matter — the value the validate side's version-aware
/// routing demands (`design/corpus-migration.md` → The schema-version stamp).
#[test]
fn fresh_methodology_mint_carries_schema_version_1() {
    let repo = TempDir::new("mint-repo");
    let home = TempDir::new("mint-home");
    init_repo(repo.path());
    list_pack(repo.path(), &methodology_pack_tree());

    // Mint a research doc through the real workflow: start → create → author →
    // fill the commit doc → finalize.
    let start = run_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "do-research", "study the cache"],
    );
    assert!(
        start.status.success(),
        "`jigc start --workflow do-research` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
    );
    let run = |args: &[&str]| {
        let out = run_listed(repo.path(), home.path(), args);
        assert!(
            out.status.success(),
            "jigc {args:?} must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        use std::io::Write;
        let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(["doc", "set-slot", addr, "--from-file", "-"])
            .current_dir(repo.path())
            .env("HOME", home.path())
            .env_remove("JIGC_PACK_DIR")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("spawn jigc set-slot");
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(prose)
            .expect("write stdin");
        let out = child.wait_with_output().expect("wait for jigc");
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    };
    run(&["doc", "create", "research", "--title", "Cache Study"]);
    for slot in ["question", "findings", "sources"] {
        set_slot(&format!("research:cache-study#{slot}"), b"Some prose.\n");
    }
    let task = "study-the-cache";
    run(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "docs",
    ]);
    set_slot(&format!("commit:{task}#summary"), b"record the study\n");
    run(&["task", "finalize", task]);

    let committed = fs::read_to_string(repo.path().join("research").join("cache-study.md"))
        .expect("the finalized research doc is committed at research/cache-study.md");
    assert!(
        committed.contains("schema-version: 1"),
        "a fresh methodology mint must carry `schema-version: 1` in its front matter; got:\n{committed}",
    );
}

/// Compose `[drifted-dev-copy ▸ methodology ▸ embedded-dev]` via the listed-pack
/// mechanism — the production shape a real project uses (`packs.yaml` names pack
/// **directories**, the trigger that discharged the M33 deferral). The methodology
/// pack rides along so the `milestone-record` doctype resolves and `jigc milestone
/// create` reaches its committing write path.
fn list_packs(repo: &Path, packs: &[&Path]) {
    let mut body = String::from("packs:\n");
    for pack in packs {
        body.push_str(&format!("  - {}\n", pack.display()));
    }
    fs::write(repo.join(".jigc").join("config").join("packs.yaml"), body)
        .expect("write packs.yaml naming the listed packs");
    commit_workbench_file(repo, ".jigc/config/packs.yaml");
}

/// What a manifest-governed schema drift does to one door of [`FREEZE_DOORS`].
#[derive(Debug, Clone, Copy)]
enum FreezeCheck {
    /// The door's own path reaches `crate::pack::make_pack`, so the drift blocks it
    /// and the block **names the freeze**.
    Named,
    /// The door refuses *before* pack-load, on a precondition of its own that the
    /// sweep fixture does not satisfy: it never acts, but the diagnosis it prints is
    /// that precondition's, not the freeze's. `says` is the substring it prints —
    /// asserted, so the disposition is a measurement rather than a claim — and `why`
    /// states the precondition.
    ///
    /// This is an **ordering** fact, not a hole, and it is proven rather than argued:
    /// [`the_task_scoped_doors_block_once_their_task_exists`] gives the fixture the task
    /// these rows name and drives every one of them into `make_pack`, where it blocks
    /// naming the drift. What the sweep itself proves over them is the half that matters
    /// for safety — over a drifted corpus the door refuses and writes nothing.
    RefusedEarlier {
        says: &'static str,
        why: &'static str,
    },
    /// The door completes at **exit 0** over the drift, because it loads no pack at
    /// all. The string states why that is the right answer rather than a hole — the
    /// difference between a disposition and an oversight.
    Exempt(&'static str),
    /// The door **loads the pack and completes at exit 0 anyway**, reporting the drift as
    /// an advisory forecast rather than a refusal (M50 Increment 12 / T3). Distinct from
    /// [`FreezeCheck::Exempt`], and the distinction is the point: an exempt door is
    /// asserted to say *nothing* about the freeze, and this one is asserted to **name
    /// it** — so a forecast that silently stopped forecasting reddens here instead of
    /// passing as an exemption. The string states why exit 0 is the right answer.
    Forecasts(&'static str),
}

/// Whether a door's argv exits 0 against a fixture carrying **no drift** and no task,
/// doc or milestone state — the subset an *inert-gate* arm can assert success over.
#[derive(Debug, Clone, Copy)]
enum CleanExit {
    /// It exits 0.
    Zero,
    /// It does not, for the stated reason: the door is fine, the fixture lacks what
    /// this argv names.
    Needs(&'static str),
}

/// One door — the argv after `jigc`, plus its two dispositions.
struct FreezeDoor {
    /// The argv a sweep runs, after `jigc`. Its leading segments must be a
    /// [`VERB_KINDS`] leaf; the fence
    /// [`every_leaf_verb_has_a_dispositioned_freeze_door`] checks that, and that the
    /// whole argv parses against the real clap tree.
    argv: &'static [&'static str],
    /// What a manifest-governed schema drift does to this door.
    check: FreezeCheck,
    /// What this argv does with no drift in the way.
    clean: CleanExit,
}

/// **The door axis of the pack-load freeze** — every leaf verb of the CLI, each with
/// what a manifest-governed schema drift does to it (M49 completion audit).
///
/// It exists because the set was a **hand-written literal of five** — `validate`,
/// `describe`, `doc schema adr`, `migrate-corpus`, `start` — feeding a test named
/// `a_shape_changing_project_schema_shadow_blocks_every_door`. The constant solved
/// *arm-to-arm* drift (a later arm cannot sweep a narrower set than the one before
/// it) and was never a claim about door coverage, so "every door" was a claim about
/// **47** doors proven over a sample of five. It was also wrong: `jigc setup`,
/// `jigc uninstall` and `jigc task list` all complete at **exit 0** over a shadow that
/// blocks every door the five happened to name.
///
/// Membership is therefore **derived, not remembered**: the fence
/// [`every_leaf_verb_has_a_dispositioned_freeze_door`] asserts a bijection with
/// [`VERB_KINDS`] — the table that bijects the clap leaf tree — so a verb added
/// anywhere in the CLI owes this table a row before it can ship, and no row may name
/// a verb the tree does not carry. A grep is not a fence: the sweep that found these
/// three exit-0 doors cannot stop the fourth.
///
/// **Why the table lives in the test and not beside [`VERB_KINDS`] in `cli.rs`.**
/// Its rows are fixture-shaped — `adr:probe`, `freeze-probe`, `payload.txt`, an
/// absent milestone id — and a disposition like [`CleanExit`] is a fact about *this*
/// fixture, not about the shipped binary. Shipping argv fixtures inside the product
/// would be worse than the derivation is good; the derivation itself does not depend
/// on where the array sits, because it is checked against the production registry
/// either way.
const FREEZE_DOORS: &[FreezeDoor] = &[
    // ── Top level ────────────────────────────────────────────────────────────────
    FreezeDoor {
        argv: &["start"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["workflow", "single-task", "--preview"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["setup"],
        check: FreezeCheck::Forecasts(
            "the BOOTSTRAP door, and it reads no doctype by design: it wires the \
             adapter into the repo (the CLAUDE.md reference, the allowlist, the \
             SessionStart hook, the pre-commit hook, the guides) and none of that \
             resolves a schema. Refusing to install over a drifted corpus would be \
             circular — the install is how the operator's agent learns `jigc` exists \
             at all — and that bound is unchanged. What changed at M50 Increment 12 / \
             T3 is the SILENCE beside it: this row used to say the freeze's diagnosis \
             is emitted by the first door that does load the pack, which left the \
             operator meeting the block for the first time on their next command. The \
             install now probes `make_pack` after its writes and reports one \
             `setup.pack-load` advisory naming what that next door will refuse — still \
             at exit 0, still having installed. Its ref-target twin is dispositioned \
             identically.",
        ),
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["uninstall"],
        check: FreezeCheck::Exempt(
            "the teardown twin of `setup`, exempt for the mirror reason: removing an \
             install must not be gated on the corpus being loadable, or a drifted \
             schema would trap the operator inside an install they cannot remove. It \
             resolves no doctype either. Its own destructive guard is a different \
             axis (`uninstall.dirty-worktree`, `tests/destroying_doors.rs`).",
        ),
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["upgrade"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["ingest"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["migrate", "README.md", "--as", "adr"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["migrate-corpus"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["unmanage", "README.md"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["rename", "adr:probe", "--to", "A New Title"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs("the fixture commits no `adr:probe` to rename"),
    },
    FreezeDoor {
        argv: &["relocate", "vision", "--from", "docs/vision/"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(
            "`relocate` takes a freeze-EXEMPT doctype, and the only ones shipped are \
             the methodology pack's; the project-shadow fixture composes the dev pack \
             alone, which declares no `vision`",
        ),
    },
    FreezeDoor {
        argv: &["describe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["validate"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    // ── `jigc doc` — the managed-doc surface ─────────────────────────────────────
    FreezeDoor {
        argv: &["doc", "create", "adr", "--title", "Freeze Probe"],
        check: NO_ACTIVE_TASK,
        clean: CleanExit::Needs(NO_ACTIVE_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &[
            "doc",
            "add-item",
            "adr:probe#options",
            "--title",
            "Freeze Probe",
        ],
        check: NO_ACTIVE_TASK,
        clean: CleanExit::Needs(NO_ACTIVE_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &["doc", "remove-item", "adr:probe#options/freeze-probe"],
        check: NO_ACTIVE_TASK,
        clean: CleanExit::Needs(NO_ACTIVE_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &[
            "doc",
            "retitle-item",
            "adr:probe#options/freeze-probe",
            "--title",
            "Freeze Probe",
        ],
        check: NO_ACTIVE_TASK,
        clean: CleanExit::Needs(NO_ACTIVE_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &["doc", "rename", "adr:probe", "--to", "A New Title"],
        check: NO_ACTIVE_TASK,
        clean: CleanExit::Needs(NO_ACTIVE_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &[
            "doc",
            "set-field",
            "adr:probe#status",
            "--value",
            "accepted",
        ],
        check: NO_ACTIVE_TASK,
        clean: CleanExit::Needs(NO_ACTIVE_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &[
            "doc",
            "set-slot",
            "adr:probe#context",
            "--from-file",
            "payload.txt",
        ],
        check: NO_ACTIVE_TASK,
        clean: CleanExit::Needs(NO_ACTIVE_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &["doc", "author", "adr", "--from-file", "payload.txt"],
        check: NO_ACTIVE_TASK,
        clean: CleanExit::Needs(NO_ACTIVE_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &["doc", "show", "adr:probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs("the fixture commits no `adr:probe` to read"),
    },
    FreezeDoor {
        argv: &["doc", "schema", "adr"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["doc", "list"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    // ── `jigc task` — the task lifecycle ─────────────────────────────────────────
    FreezeDoor {
        argv: &["task", "list"],
        check: FreezeCheck::Exempt(
            "a WORKBENCH listing: it reads `.jigc/tasks/` and reports ids, intents \
             and workflows, resolving no doctype schema — so the freeze has nothing \
             to say about it, and blocking it would deny the operator the view of \
             in-flight work at exactly the moment every other door is refusing. Its \
             `VerbKind::Read` is a read of the workbench, not of the store.",
        ),
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["task", "diff", "freeze-probe"],
        check: NO_SUCH_TASK,
        clean: CleanExit::Needs(NO_SUCH_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &["task", "validate", "freeze-probe"],
        check: NO_SUCH_TASK,
        clean: CleanExit::Needs(NO_SUCH_TASK_CLEAN),
    },
    // `jigc task amend` takes no task id — it MINTS one — so it has no precondition to
    // refuse on ahead of the pack, and its own provisioning resolves the `commit` schema.
    // It therefore blocks naming the drift, which is the answer the freeze owes a door that
    // would otherwise open an authoring area against a corpus jigc cannot adjudicate.
    FreezeDoor {
        argv: &["task", "amend", "repair the freeze probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["task", "discard", "freeze-probe"],
        check: NO_SUCH_TASK,
        clean: CleanExit::Needs(NO_SUCH_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &["task", "finalize", "freeze-probe"],
        check: NO_SUCH_TASK,
        clean: CleanExit::Needs(NO_SUCH_TASK_CLEAN),
    },
    FreezeDoor {
        argv: &["task", "bind", "decision", "adr:probe", "freeze-probe"],
        check: NO_SUCH_TASK,
        clean: CleanExit::Needs(NO_SUCH_TASK_CLEAN),
    },
    // ── `jigc config` — the cascade surface ──────────────────────────────────────
    FreezeDoor {
        argv: &["config", "set", "docs-root", "docs"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &[
            "config",
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "orient",
            "payload.txt",
        ],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(ANCHOR_ABSENT),
    },
    FreezeDoor {
        argv: &[
            "config",
            "replace-step",
            "workflow:single-task#orient",
            "payload.txt",
        ],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(ANCHOR_ABSENT),
    },
    FreezeDoor {
        argv: &["config", "remove-step", "workflow:single-task#orient"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(ANCHOR_ABSENT),
    },
    FreezeDoor {
        argv: &[
            "config",
            "fill",
            "step:orient#freeze-probe",
            "--from-file",
            "payload.txt",
        ],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(
            "`step:orient#freeze-probe` is no `{{fill:}}` point of the resolved \
             `orient` step",
        ),
    },
    FreezeDoor {
        argv: &["config", "fork", "workflow:single-task#orient"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(ANCHOR_ABSENT),
    },
    FreezeDoor {
        argv: &["config", "get", "docs-root"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["config", "list"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    // ── `jigc milestone` — the work-unit surface ─────────────────────────────────
    FreezeDoor {
        argv: &["milestone", "create", "Freeze Probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Zero,
    },
    FreezeDoor {
        argv: &["milestone", "add-task", "m-freeze-probe", "probe intent"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(NO_SUCH_MILESTONE),
    },
    FreezeDoor {
        argv: &["milestone", "add-from-spec", "m-freeze-probe", "spec:probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(NO_SUCH_MILESTONE),
    },
    FreezeDoor {
        argv: &["milestone", "list-tasks", "m-freeze-probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(NO_SUCH_MILESTONE),
    },
    FreezeDoor {
        argv: &["milestone", "provision", "m-freeze-probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(NO_SUCH_MILESTONE),
    },
    FreezeDoor {
        argv: &["milestone", "execute", "m-freeze-probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(NO_SUCH_MILESTONE),
    },
    FreezeDoor {
        argv: &["milestone", "join", "m-freeze-probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(NO_SUCH_MILESTONE),
    },
    FreezeDoor {
        argv: &["milestone", "finalize", "m-freeze-probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(NO_SUCH_MILESTONE),
    },
    FreezeDoor {
        argv: &["milestone", "discard", "m-freeze-probe"],
        check: FreezeCheck::Named,
        clean: CleanExit::Needs(NO_SUCH_MILESTONE),
    },
];

/// The eight `jigc doc` write verbs' shared disposition: the active-task pre-check
/// runs ahead of pack-load, so over a drifted corpus they answer with *that*, and
/// they act on nothing.
const NO_ACTIVE_TASK: FreezeCheck = FreezeCheck::RefusedEarlier {
    says: "no active task",
    why: "the active-task pre-check runs before pack-load and the sweep fixture holds \
          no task. The ordering is not a hole: a task minted before the drift landed \
          carries the same argv into `make_pack`, where it blocks naming the drift. \
          What is proven here is that the door refuses and writes nothing.",
};

/// What the same eight verbs lack against an undrifted fixture — the same absence,
/// stated for the other axis.
const NO_ACTIVE_TASK_CLEAN: &str = "the fixture holds no active task to write into";

/// The task id the [`NO_SUCH_TASK`] rows name. The sweep fixture deliberately holds
/// no such task; [`the_task_scoped_doors_block_once_their_task_exists`] mints exactly
/// it, which is what makes those rows' *ordering* claim checkable rather than argued.
const TASK_PROBE_ID: &str = "freeze-probe";

/// The five task-lifecycle verbs that take an id: the lookup precedes pack-load.
const NO_SUCH_TASK: FreezeCheck = FreezeCheck::RefusedEarlier {
    says: "no task `freeze-probe`",
    why: "the task lookup runs before pack-load and the sweep fixture holds no task \
          `freeze-probe`; the door refuses and acts on nothing. With a real id the \
          same argv reaches `make_pack` and blocks naming the drift.",
};

/// Their undrifted-axis twin.
const NO_SUCH_TASK_CLEAN: &str = "the fixture holds no task `freeze-probe`";

/// The four `config` delta verbs that name a step position the shipped `single-task`
/// does not carry — enough to reach pack-load, not enough to edit anything.
const ANCHOR_ABSENT: &str = "`orient` is no step of the shipped `single-task`";

/// The eight `milestone` verbs that take an id the fixture never minted.
const NO_SUCH_MILESTONE: &str = "the fixture holds no milestone `m-freeze-probe`";

/// A repo + `$HOME` pair a sweep drives, holding the temp dirs that back it alive for
/// as long as the site is.
struct Site {
    repo: PathBuf,
    home: PathBuf,
    _keep: Vec<TempDir>,
}

impl Site {
    fn run(&self, argv: &[&str]) -> std::process::Output {
        run_listed(&self.repo, &self.home, argv)
    }
}

/// The [`VERB_KINDS`] leaf a door's argv reaches — the longest classified prefix, and
/// its read/write kind.
fn verb_of(argv: &[&str]) -> (&'static [&'static str], VerbKind) {
    VERB_KINDS
        .iter()
        .filter(|(path, _)| argv.len() >= path.len() && argv[..path.len()] == **path)
        .max_by_key(|(path, _)| path.len())
        .map(|(path, kind)| (*path, *kind))
        .unwrap_or_else(|| panic!("`jigc {}` reaches no VERB_KINDS leaf", argv.join(" ")))
}

/// The `pack-load freeze check failed` banner — the one string that says *the freeze
/// fired*, whichever door printed it.
const FREEZE_BANNER: &str = "pack-load freeze check failed";

/// Assert one door's declared [`FreezeCheck`] against a drifted `site`, returning the
/// `route:` command it emitted (when it emitted one).
fn assert_drifted_door(
    label: &str,
    site: &Site,
    door: &FreezeDoor,
    needles: &[&str],
) -> Option<String> {
    let argv = door.argv;
    let out = site.run(argv);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let said = format!("stdout:\n{stdout}\nstderr:\n{stderr}");
    match door.check {
        FreezeCheck::Named => {
            assert!(
                !out.status.success(),
                "[{label}] `jigc {}` is declared `Named` — it must exit non-zero over the \
                 drift; {said}",
                argv.join(" "),
            );
            for needle in needles {
                assert!(
                    stderr.contains(needle),
                    "[{label}] `jigc {}` stderr must name {needle:?}; got:\n{stderr}",
                    argv.join(" "),
                );
            }
            stderr
                .lines()
                .find(|line| line.trim_start().starts_with("route:"))
                .map(|_| route_command(&stderr))
        }
        FreezeCheck::RefusedEarlier { says, why } => {
            assert!(
                !out.status.success(),
                "[{label}] `jigc {}` is declared `RefusedEarlier` ({why}) — it must still \
                 exit non-zero over the drift, acting on nothing; {said}",
                argv.join(" "),
            );
            assert!(
                stderr.contains(says),
                "[{label}] `jigc {}` must refuse with {says:?} ({why}); got:\n{stderr}",
                argv.join(" "),
            );
            assert!(
                !stderr.contains(FREEZE_BANNER) && !stdout.contains(FREEZE_BANNER),
                "[{label}] `jigc {}` is declared `RefusedEarlier` but named the freeze — \
                 it now reaches pack-load, so its row owes `FreezeCheck::Named`; {said}",
                argv.join(" "),
            );
            None
        }
        FreezeCheck::Exempt(why) => {
            assert!(
                out.status.success(),
                "[{label}] `jigc {}` is declared exempt from the pack-load freeze \
                 ({why}) — it must complete at exit 0 over the drift; {said}",
                argv.join(" "),
            );
            assert!(
                !stderr.contains(FREEZE_BANNER) && !stdout.contains(FREEZE_BANNER),
                "[{label}] the exempt door `jigc {}` must not name the freeze; {said}",
                argv.join(" "),
            );
            None
        }
        FreezeCheck::Forecasts(why) => {
            assert!(
                out.status.success(),
                "[{label}] `jigc {}` is declared a forecasting door ({why}) — it must \
                 complete at exit 0 over the drift; {said}",
                argv.join(" "),
            );
            assert!(
                stdout.contains(FREEZE_BANNER),
                "[{label}] the forecasting door `jigc {}` must NAME the freeze — silence \
                 there is the defect M50 Increment 12 / T3 closed; {said}",
                argv.join(" "),
            );
            assert!(
                stdout.contains(cli::setup::PACK_LOAD_CODE),
                "[{label}] the forecasting door `jigc {}` must name the freeze through \
                 its advisory code, not in prose of its own; {said}",
                argv.join(" "),
            );
            // `needles` is deliberately not asserted here: like every acting door this one
            // runs on its **own** fixture (`solo`), so the shared fixture's shadow path is
            // not the path this run met. That the forecast names its own shadow is pinned
            // where the fixture is the subject —
            // `crates/cli/tests/setup_pack_load_advisory.rs`.
            None
        }
    }
}

/// Drive **every** door of [`FREEZE_DOORS`] against a drifted state, assert each
/// door's declared disposition, and return the `route:` command the last blocked door
/// emitted (`None` when the drift's diagnosis carries no route).
///
/// `shared` serves the doors that never act — which is sound because *a blocked door
/// acting* is precisely what the sweep denies. The doors that **do** act
/// ([`FreezeCheck::Exempt`] and [`FreezeCheck::Forecasts`] — the two that complete at
/// exit 0) each get their own fixture from
/// `solo`: `jigc uninstall` removes the `.jigc/` tree the project-layer drift lives
/// in, and `jigc setup` rewrites `packs.yaml`, so sharing would erase the very state
/// the remaining doors are meant to meet.
fn sweep_drifted(
    label: &str,
    shared: &Site,
    needles: &[&str],
    solo: &dyn Fn(&str) -> Site,
) -> Option<String> {
    let mut route = None;
    for door in FREEZE_DOORS {
        let acting;
        let site = match door.check {
            FreezeCheck::Exempt(_) | FreezeCheck::Forecasts(_) => {
                acting = solo(label);
                &acting
            }
            _ => shared,
        };
        if let Some(found) = assert_drifted_door(label, site, door, needles) {
            route = Some(found);
        }
    }
    route
}

/// The doors a second pass may re-run **on the same fixture**: they report and mutate
/// nothing (`VerbKind::Read`) and they exit 0 with nothing in the way
/// ([`CleanExit::Zero`]). A write door's second run would act on the repo, which
/// proves nothing about the thing the second pass is testing.
fn reruns_clean(door: &FreezeDoor) -> bool {
    matches!(door.clean, CleanExit::Zero) && verb_of(door.argv).1 == VerbKind::Read
}

/// The other half of [`FreezeCheck::RefusedEarlier`]: those thirteen doors are not
/// *outside* the freeze, they are **ordered behind a precondition of their own**. Give
/// the fixture the task their rows name — minted before the drift lands, which is the
/// real sequence: work in flight when someone reshapes a schema — and every one of
/// them reaches `make_pack` and blocks naming the drift, at the shadow's own path.
///
/// Without this arm the sweep's answer for a third of the door set would stop at *"it
/// refuses for some other reason"* — the safety half, but not the claim. With it,
/// *blocks every door* holds over both states a door can be met in.
#[test]
fn the_task_scoped_doors_block_once_their_task_exists() {
    let repo = TempDir::new("ordering-repo");
    let home = TempDir::new("ordering-home");
    init_repo(repo.path());

    // The task the `RefusedEarlier` rows name, minted BEFORE the drift lands. Its id is
    // asserted rather than read back, because those rows spell it literally.
    let minted = run_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "freeze probe"],
    );
    let stdout = String::from_utf8_lossy(&minted.stdout);
    assert!(
        minted.status.success(),
        "the fixture task must mint against an undrifted pack; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&minted.stderr),
    );
    assert!(
        stdout.contains(&format!("task minted: {TASK_PROBE_ID}")),
        "the minted id must be the one FREEZE_DOORS' task-scoped rows name \
         (`{TASK_PROBE_ID}`); got:\n{stdout}",
    );

    let shadow = install_schema_shadow(repo.path(), "adr", &adr_shadow_with_owner());
    let shadow_path = shadow.display().to_string();
    let ordered: Vec<&FreezeDoor> = FREEZE_DOORS
        .iter()
        .filter(|door| matches!(door.check, FreezeCheck::RefusedEarlier { .. }))
        .collect();
    assert!(
        !ordered.is_empty(),
        "the `RefusedEarlier` disposition must have members for this arm to mean anything",
    );
    for door in ordered {
        let out = run_listed(repo.path(), home.path(), door.argv);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "`jigc {}` must block once its task exists; stdout:\n{}\nstderr:\n{stderr}",
            door.argv.join(" "),
            String::from_utf8_lossy(&out.stdout),
        );
        for needle in ["schema-hash mismatch", "adr", shadow_path.as_str()] {
            assert!(
                stderr.contains(needle),
                "`jigc {}` must now name the freeze ({needle:?}) rather than its own \
                 precondition; got:\n{stderr}",
                door.argv.join(" "),
            );
        }
    }
}

/// **The membership fence.** [`FREEZE_DOORS`] bijects [`VERB_KINDS`] — the registry
/// that bijects the clap leaf tree — so the door axis is *derived* rather than
/// remembered, and each argv parses against the real CLI rather than against a
/// plausible spelling of it. A verb added anywhere in the tree reddens here until its
/// freeze disposition is stated.
#[test]
fn every_leaf_verb_has_a_dispositioned_freeze_door() {
    let mut covered: BTreeSet<Vec<&str>> = BTreeSet::new();
    for door in FREEZE_DOORS {
        let mut argv = vec!["jigc"];
        argv.extend_from_slice(door.argv);
        cli::cli::Cli::try_parse_from(&argv).unwrap_or_else(|err| {
            panic!(
                "a FREEZE_DOORS argv must parse against the real clap tree — \
                 `{}` did not:\n{err}",
                argv.join(" "),
            )
        });
        let (leaf, _) = verb_of(door.argv);
        assert!(
            covered.insert(leaf.to_vec()),
            "`jigc {}` is covered twice — one door per leaf verb",
            leaf.join(" "),
        );
    }

    let declared: BTreeSet<Vec<&str>> = VERB_KINDS.iter().map(|(path, _)| path.to_vec()).collect();
    let missing: Vec<String> = declared
        .difference(&covered)
        .map(|path| path.join(" "))
        .collect();
    assert!(
        missing.is_empty(),
        "every leaf verb owes FREEZE_DOORS a row stating what a manifest-governed \
         schema drift does to it — undisposed: {missing:?}",
    );
    let stray: Vec<String> = covered
        .difference(&declared)
        .map(|path| path.join(" "))
        .collect();
    assert!(
        stray.is_empty(),
        "FREEZE_DOORS names verbs the clap tree does not carry: {stray:?}",
    );
    assert_eq!(
        FREEZE_DOORS.len(),
        VERB_KINDS.len(),
        "one door per leaf verb, both ways",
    );
}

/// A repo + home + listed **drifted** dev-pack copy (with the methodology pack
/// composed for the `milestone-record` doctype). Every door run against it must
/// block, naming [`needles`](DriftedProject::needles).
struct DriftedProject {
    repo: TempDir,
    home: TempDir,
    _pack: TempDir,
    /// The substrings every blocked door's stderr must carry — what the operator
    /// reading the failure is owed: *what* drifted and *how to fix it*.
    needles: &'static [&'static str],
}

impl DriftedProject {
    /// The schema-shape drift (a frozen `adr` whose `location:` moved with no
    /// version bump).
    fn new(tag: &str) -> Self {
        Self::with(tag, drift_adr_schema, &["schema-hash mismatch", "adr"])
    }

    /// A listed dev-pack copy mutated by `drift`; every door must then block naming
    /// `needles`.
    fn with(tag: &str, drift: fn(&Path), needles: &'static [&'static str]) -> Self {
        let repo = TempDir::new(&format!("{tag}-repo"));
        let home = TempDir::new(&format!("{tag}-home"));
        let pack = dev_pack_copy(&format!("{tag}-pack"));
        init_repo(repo.path());
        list_packs(repo.path(), &[pack.path(), &methodology_pack_tree()]);
        drift(pack.path());
        DriftedProject {
            repo,
            home,
            _pack: pack,
            needles,
        }
    }

    /// This project as a [`Site`] a sweep can drive, the temp dirs moving with it.
    fn into_site(self) -> Site {
        let repo = self.repo.path().to_path_buf();
        let home = self.home.path().to_path_buf();
        Site {
            repo,
            home,
            _keep: vec![self.repo, self.home, self._pack],
        }
    }

    /// Run one door and assert it exits non-zero naming the drift — the emitted exit
    /// code + stderr are the contract.
    fn door_blocks(&self, args: &[&str]) -> std::process::Output {
        let out = run_listed(self.repo.path(), self.home.path(), args);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "`jigc {}` must exit non-zero on a drifted frozen pack; stdout:\n{}\nstderr:\n{stderr}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
        );
        for needle in self.needles {
            assert!(
                stderr.contains(needle),
                "`jigc {}` stderr must name {needle:?}; got:\n{stderr}",
                args.join(" "),
            );
        }
        out
    }
}

/// The M42 Inc 6 headline: the freeze assert lives in the **pack-source factory**, so
/// a drifted frozen schema blocks **every** door — not just the compose front door it
/// used to guard. The read/report verbs sailed past it before this task
/// (`implementation/decisions-pending.md` → the discharged M33 deferral: the trigger
/// "packs are ever loaded from the filesystem in production" fired at M14).
///
/// **The set is [`FREEZE_DOORS`], and it is the whole clap leaf tree** (M49 completion
/// audit): this arm used to sweep five hand-listed read verbs under a name that
/// claimed all of them.
#[test]
fn every_door_blocks_on_a_drifted_frozen_schema() {
    let project = DriftedProject::new("doors");
    let needles = project.needles;
    let shared = project.into_site();

    let _ = sweep_drifted("doors", &shared, needles, &|seed| {
        DriftedProject::new(&format!("{seed}-solo")).into_site()
    });
}

/// Rewrite the copied manifest's declared `slug-rule.hash` to a well-formed but
/// **wrong** digest — the identity-mint sibling of [`drift_adr_schema`]: it stands in
/// for a change to `engine::slug::slugify` (the function that mints every doc slug,
/// task id, and `{#id}` anchor) shipped without the declared `slug-rule-version` bump.
/// Mutating the *manifest* rather than the engine is what makes the drift reachable
/// from a test at all — the pack is data, the rule is compiled in — and it drives the
/// same `check` arm from the same side the gate reads.
fn drift_slug_rule_hash(pack: &Path) {
    let path = pack.join("config").join("schema-manifest.yaml");
    let body = fs::read_to_string(&path).expect("read the copied schema-manifest.yaml");
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if !hit && line.starts_with("  hash: ") {
            out.push_str(&format!("  hash: {}", "0".repeat(64)));
            hit = true;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped manifest must declare a `slug-rule.hash` to drift"
    );
    fs::write(&path, out).expect("write the drifted schema-manifest.yaml");
}

/// The M42 Inc 10 T1 headline — **the fence**: `slugify` is jigc's identity-derivation
/// function (every doc slug, task/milestone id, and `{#id}` anchor) and it sits in no
/// `schema-hash` and no manifest, so the gate that blocks renaming a *field* waved
/// through a change to the function that *names every id in every corpus* — a change
/// **no migration can repair** (no transform kind re-mints an id). With the rule
/// declared (`slug-rule: { version, hash }`), a manifest whose declared fingerprint no
/// longer matches the shipped rule blocks **every** door, naming the recomputed hash
/// and routing to the declared bump (`design/storage.md` → Identity → *The slug rule is
/// itself a versioned rule (M42)*).
#[test]
fn a_drifted_slug_rule_blocks_every_door() {
    const NEEDLES: &[&str] = &["the slug rule changed", "bump slug-rule-version + re-pin"];
    let shared = DriftedProject::with("slugrule", drift_slug_rule_hash, NEEDLES).into_site();

    let _ = sweep_drifted("slugrule", &shared, NEEDLES, &|seed| {
        DriftedProject::with(&format!("{seed}-solo"), drift_slug_rule_hash, NEEDLES).into_site()
    });
}

/// **Delete** the copied manifest's whole `slug-rule:` block — the third silencer,
/// and the cheapest one: [`drift_slug_rule_hash`]'s route says *re-pin the hash*, but
/// an author (or an agent) who instead removes three lines got the gate to go quiet
/// entirely. A pack opts out of the freeze by shipping **no manifest** (a visible,
/// wholesale act — [`manifest_less_pack_is_unaffected`]); a manifest that freezes
/// doctype *shapes* may not quietly decline to declare the rule the ids inside them
/// are *minted* by.
fn delete_slug_rule_block(pack: &Path) {
    let path = pack.join("config").join("schema-manifest.yaml");
    let body = fs::read_to_string(&path).expect("read the copied schema-manifest.yaml");
    let mut out = String::new();
    let mut dropping = false;
    let mut hit = false;
    for line in body.lines() {
        if line.starts_with("slug-rule:") {
            dropping = true;
            hit = true;
            continue;
        }
        // The block's own body is indented; the next unindented key ends it.
        if dropping {
            if line.starts_with(' ') {
                continue;
            }
            dropping = false;
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped manifest must declare a `slug-rule:` block to delete"
    );
    assert!(
        !out.lines().any(|line| line.starts_with("slug-rule:")),
        "the declared block must be gone from the rewritten manifest; got:\n{out}"
    );
    fs::write(&path, out).expect("write the slug-rule-less schema-manifest.yaml");
}

/// The M42 completion-audit LOW: a manifest that **omits** `slug-rule:` asserted
/// nothing, so the fence [`a_drifted_slug_rule_blocks_every_door`] builds could be
/// silenced by deleting it — and a third-party / project pack authored without the
/// key inherited **no fence at all** over the one change no migration can repair.
/// "Absent means unchecked" is the exact shape M42 has been bitten by repeatedly: a
/// fence that only fires on the members it happens to know about is not a fence
/// (`DECISIONS.md` → 2026-07-14, *a census cannot enforce a predicate*). Every door
/// must block, and the route must hand the author the block to paste.
#[test]
fn a_manifest_omitting_the_slug_rule_blocks_every_door() {
    const NEEDLES: &[&str] = &["declares no `slug-rule:` block", "slug-rule:"];
    let shared = DriftedProject::with("slugless", delete_slug_rule_block, NEEDLES).into_site();

    let _ = sweep_drifted("slugless", &shared, NEEDLES, &|seed| {
        DriftedProject::with(&format!("{seed}-solo"), delete_slug_rule_block, NEEDLES).into_site()
    });
}

/// The control (the omitting context's twin): an **unmutated** listed pack — whose
/// manifests declare the slug rule the engine really ships — runs every one of those
/// doors clean. Without this arm, a fence that blocked *unconditionally* would pass the
/// test above; it is also the standing proof that both shipped manifests' `slug-rule`
/// blocks stay in sync with `engine::slug::rule_fingerprint()`.
#[test]
fn an_unmutated_listed_pack_passes_the_slug_rule_gate() {
    let repo = TempDir::new("slugclean-repo");
    let home = TempDir::new("slugclean-home");
    let pack = dev_pack_copy("slugclean-pack");
    init_repo(repo.path());
    list_packs(repo.path(), &[pack.path(), &methodology_pack_tree()]);

    for args in [
        vec!["validate"],
        vec!["describe"],
        vec!["doc", "schema", "adr"],
        vec!["start"],
    ] {
        let out = run_listed(repo.path(), home.path(), &args);
        assert!(
            out.status.success(),
            "`jigc {}` must run clean against an unmutated pack; stdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// The **committing** door: `jigc milestone create` wrote and committed a brand-new
/// milestone record at exit 0 over a drifted frozen schema. It must block — and land
/// **no** record file and **no** record commit (the write is what makes this door the
/// dangerous one; Inc 7 builds a second write verb on it).
#[test]
fn the_committing_milestone_door_blocks_and_writes_nothing() {
    let project = DriftedProject::new("mcreate");

    project.door_blocks(&["milestone", "create", "drift probe"]);

    let records = project.repo.path().join("docs").join("milestone-records");
    let landed: Vec<PathBuf> = fs::read_dir(&records)
        .map(|dir| {
            dir.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
                .collect()
        })
        .unwrap_or_default();
    assert!(
        landed.is_empty(),
        "a blocked `milestone create` must land no milestone-record file; found: {landed:?}",
    );

    let log = Command::new("git")
        .args(["log", "--oneline"])
        .current_dir(project.repo.path())
        .output()
        .expect("run git log");
    let log = String::from_utf8_lossy(&log.stdout);
    assert!(
        !log.contains("chore(milestone): open record"),
        "a blocked `milestone create` must land no record commit; git log:\n{log}",
    );
}

/// Run the **real** `jigc setup` — the only production install path (QUICKSTART) — in
/// `repo`. It writes `compose-embedded-methodology: true` into
/// `.jigc/config/packs.yaml` (`crates/cli/src/setup.rs` → step 2b).
fn run_setup(repo: &Path, home: &Path) -> std::process::Output {
    run_listed(repo, home, &["setup"])
}

/// The topology **`jigc setup` actually creates**: the setup-written compose marker
/// *plus* an operator-listed filesystem pack (the `packs:` list
/// `design/multi-pack.md` → The pack-set documents). The listed pack used to be
/// **silently dropped**, which left the freeze gate nothing to check: over a drifted
/// frozen `adr` every door, including the **committing** `jigc milestone create`, ran
/// at exit 0 and landed a record commit. M42 stopped the drop with a blanket refusal of
/// the whole combination; **M49 Increment 6 composes it instead**
/// (`[listed… ▸ dev ▸ methodology]` — `crates/cli/tests/project_pack_composition.rs`),
/// so the listed pack is now genuinely **loaded and checked**: its drifted `adr` is
/// named by the pack-load freeze gate at every door.
///
/// The claim this test pins is unchanged and is the one its name states — *a listed
/// pack is never silently dropped* — and it is now pinned to the **truthful**
/// diagnosis: the doors block naming the doctype that actually drifted, rather than
/// naming a pack-set combination the loader no longer refuses. The harm arm is
/// unchanged: the committing door lands no record file and no record commit.
#[test]
fn the_setup_written_marker_never_silently_drops_a_listed_pack() {
    let repo = TempDir::new("marker-repo");
    let home = TempDir::new("marker-home");
    let pack = dev_pack_copy("marker-pack");
    init_repo(repo.path());

    // The production install writes the marker — assert it verbatim, so this arm is
    // pinned to what setup really emits, not to a reconstruction of it.
    let setup = run_setup(repo.path(), home.path());
    assert!(
        setup.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    let packs_yaml = repo.path().join(".jigc").join("config").join("packs.yaml");
    let marker = fs::read_to_string(&packs_yaml).expect("read the setup-written packs.yaml");
    assert!(
        marker.contains("compose-embedded-methodology: true"),
        "`jigc setup` must write the compose marker; got:\n{marker}",
    );

    // The operator then lists their own pack alongside setup's marker bytes (verbatim),
    // and that pack's frozen `adr` schema has drifted with no manifest bump.
    drift_adr_schema(pack.path());
    fs::write(
        &packs_yaml,
        format!("packs:\n  - {}\n{marker}", pack.path().display()),
    )
    .expect("list the operator's pack alongside the setup-written marker");

    for args in [
        vec!["validate"],
        vec!["describe"],
        vec!["doc", "schema", "adr"],
        vec!["migrate-corpus"],
        vec!["start"],
        vec!["start", "--workflow", "single-task", "freeze-gate probe"],
        vec!["milestone", "create", "drift probe"],
    ] {
        let out = run_listed(repo.path(), home.path(), &args);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "`jigc {}` must exit non-zero over a listed pack whose frozen `adr` has drifted; stdout:\n{}\nstderr:\n{stderr}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
        );
        assert!(
            stderr.contains("pack-load freeze check failed") && stderr.contains("`adr`"),
            "`jigc {}` stderr must name the drifted frozen doctype in the LISTED pack — \
             which is only possible because that pack was actually loaded, not dropped; \
             got:\n{stderr}",
            args.join(" "),
        );
    }

    // The committing door landed nothing — no record file, no record commit.
    let records = repo.path().join("docs").join("milestone-records");
    let landed: Vec<PathBuf> = fs::read_dir(&records)
        .map(|dir| {
            dir.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
                .collect()
        })
        .unwrap_or_default();
    assert!(
        landed.is_empty(),
        "a blocked `milestone create` must land no milestone-record file; found: {landed:?}",
    );
    let log = Command::new("git")
        .args(["log", "--oneline"])
        .current_dir(repo.path())
        .output()
        .expect("run git log");
    let log = String::from_utf8_lossy(&log.stdout);
    assert!(
        !log.contains("chore(milestone): open record"),
        "a blocked `milestone create` must land no record commit; git log:\n{log}",
    );
}

/// The other order — `jigc setup` run **over** a project already on the M14 listed-pack
/// path. Setup wires the compose marker there (M49 Inc 6 T2) *and* leaves the
/// operator's declared list intact, because the marker no longer makes listed packs
/// inert: since T1 the loader composes `[listed… ▸ dev ▸ methodology]`. The claim this
/// arm carries is unchanged — the listed pack stays genuinely **loaded**, so the freeze
/// gate still sees it and the same `adr` drift blocks the door naming the schema-hash
/// mismatch — but the proof no longer rests on setup declining to write the marker.
#[test]
fn setup_over_a_listed_pack_leaves_that_pack_loaded_and_freeze_checked() {
    let repo = TempDir::new("m14-repo");
    let home = TempDir::new("m14-home");
    let pack = dev_pack_copy("m14-pack");
    init_repo(repo.path());
    list_packs(repo.path(), &[pack.path(), &methodology_pack_tree()]);

    let setup = run_setup(repo.path(), home.path());
    assert!(
        setup.status.success(),
        "`jigc setup` must succeed over a project that lists packs; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    let packs_yaml =
        fs::read_to_string(repo.path().join(".jigc").join("config").join("packs.yaml"))
            .expect("read packs.yaml after setup");
    assert!(
        packs_yaml.contains("compose-embedded-methodology: true"),
        "`jigc setup` must wire the compose marker over a listed `packs:` list — the two \
         compose since M49 Inc 6; got:\n{packs_yaml}",
    );
    assert!(
        packs_yaml.contains(&pack.path().display().to_string()),
        "the operator's listed pack must survive setup; got:\n{packs_yaml}",
    );

    // The listed pack is really loaded: drift its frozen `adr` and the gate fires.
    drift_adr_schema(pack.path());
    let out = run_listed(repo.path(), home.path(), &["validate"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "the listed pack's drifted frozen schema must block `jigc validate`; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("schema-hash mismatch") && stderr.contains("adr"),
        "stderr must name the drifted `adr` doctype's schema-hash mismatch; got:\n{stderr}",
    );
}

/// The omitting context: a **manifest-less** pack is unchecked. The *same* schema
/// drift that the manifest-bearing copy blocks composes clean once
/// `config/schema-manifest.yaml` is dropped — proving the manifest is the gate, and
/// that a seeded / composed pack with no manifest stays inert (never errors).
#[test]
fn manifest_less_pack_is_unaffected() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("nomanifest");
    init_repo(repo.path());
    drift_adr_schema(pack.path());
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "a manifest-less pack must be unaffected by the freeze gate; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

// ---------------------------------------------------------------------------
// The project layer (M49 Increment 3, T1)
//
// The freeze gate read the **pack** and nothing else, so `.jigc/config/schemas/
// <ty>.yaml` — the whole-file definition shadow `design/overrides.md` documents —
// changed a frozen doctype's shape at every surface while the gate stayed silent:
// the corpus validated clean at exit 0, `jigc doc schema` reported the frozen
// `schema-version` for an unfrozen shape, and `doc create` wrote a third. The fence
// now hashes the **resolved** schema for a project-owned id, so the freeze binds at
// every layer that can change a schema — and the *documented* capability survives
// intact, because the presentation keys are outside the hash by M47's projection.
// ---------------------------------------------------------------------------

/// The shipped `adr` schema's bytes — read from the pack tree, never re-typed, so a
/// shadow built from it differs from the frozen shape in **exactly** the mutation
/// under test (and a schema edit elsewhere cannot leave these arms asserting over a
/// stale copy).
fn shipped_adr_schema() -> String {
    fs::read_to_string(embedded_pack_tree().join("schemas").join("adr.yaml"))
        .expect("read the shipped adr.yaml")
}

/// The shipped `adr` shape with a required `owner` header field added — a schema
/// **shape** change made from the project layer.
fn adr_shadow_with_owner() -> String {
    let body = shipped_adr_schema();
    let shadowed = body.replacen(
        "      - { id: cites-code, type: code-anchor }\n",
        "      - { id: cites-code, type: code-anchor }\n      - { id: owner, type: string }\n",
        1,
    );
    assert_ne!(body, shadowed, "adr.yaml must declare the cites-code field");
    shadowed
}

/// The shipped `adr` shape relocated `decisions/` → `adrs/` — a **home** change,
/// version-gated exactly like a shape change since M38 (`location:` is inside the
/// `schema-hash`).
fn adr_shadow_relocated() -> String {
    let body = shipped_adr_schema();
    let shadowed = body.replacen("\nlocation: decisions/\n", "\nlocation: adrs/\n", 1);
    assert_ne!(
        body, shadowed,
        "adr.yaml must declare `location: decisions/` as a top-level key"
    );
    shadowed
}

/// The reworded `description:` the presentation-only arm asserts is visible through
/// `jigc describe` — authored prose, outside the frozen hash since M47.
const REWORDED_ADR_DESCRIPTION: &str =
    "A dated architectural decision record, in this project's own words.";

/// The shipped `adr` shape with only its authored `description:` reworded — the
/// capability `design/overrides.md` → *Authored metadata on a definition resolves by
/// whole-file shadow* documents, which must keep working.
fn adr_shadow_reworded() -> String {
    let body = shipped_adr_schema();
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if !hit && line.starts_with("description: ") {
            out.push_str(&format!("description: {REWORDED_ADR_DESCRIPTION}"));
            hit = true;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    assert!(hit, "adr.yaml must declare a top-level `description:`");
    out
}

/// Write a project-layer whole-file schema shadow at
/// `<repo>/.jigc/config/schemas/<ty>.yaml` and return its path.
fn install_schema_shadow(repo: &Path, ty: &str, body: &str) -> PathBuf {
    let dir = repo.join(".jigc").join("config").join("schemas");
    fs::create_dir_all(&dir).expect("mk .jigc/config/schemas/");
    let path = dir.join(format!("{ty}.yaml"));
    fs::write(&path, body).expect("write the project schema shadow");
    commit_workbench_file(repo, &format!(".jigc/config/schemas/{ty}.yaml"));
    path
}

/// The first backtick-delimited span of a rendered `route:` line — the command an
/// operator would paste.
fn route_command(stderr: &str) -> String {
    let line = stderr
        .lines()
        .find(|l| l.trim_start().starts_with("route:"))
        .unwrap_or_else(|| panic!("the block must render a `route:` line; got:\n{stderr}"));
    let mut parts = line.split('`');
    parts.next();
    parts
        .next()
        .unwrap_or_else(|| panic!("the route must carry a backticked command; got: {line}"))
        .to_owned()
}

/// **The T1 headline** — a project-layer shadow that changes a manifest-governed
/// doctype's *shape* blocks every door, over both shape axes the freeze covers
/// (`design/corpus-migration.md` → The freeze — declared *and* enforced): a **field**
/// added, and the **home** moved (`location:` is inside the `schema-hash` since M38).
///
/// Before this fence the same shadow made `jigc validate` exit **0** over a corpus it
/// had just made non-conformant, while `jigc doc schema adr` reported the frozen
/// `schema-version` for a shape nothing froze. Each door must name *what* drifted
/// (`adr`, the hash mismatch), *where* (the shadow's own path), and *how to fix it* —
/// with a route that runs.
///
/// **Two axes, both iterated.** The shape axis is the freeze's own (`field` × `home`).
/// The second is the *fixture's*: the emitted route interpolates the shadow's absolute
/// path, so its runnability is a property of the **path shape**, not of the drift — and
/// a fixture drawn only from `std::env::temp_dir()` plus an inert segment can never see
/// it. Each shape therefore runs under both an inert path and a
/// [metacharacter-carrying one](TempDir::new_metachar), and the `sh -c <route>` arm
/// below is what distinguishes them.
#[test]
fn a_shape_changing_project_schema_shadow_blocks_every_door() {
    for (shape_tag, body) in [
        ("shadow-owner", adr_shadow_with_owner()),
        ("shadow-home", adr_shadow_relocated()),
    ] {
        for (path_tag, metachar) in [("inert-path", false), ("metachar-path", true)] {
            let tag = format!("{shape_tag}/{path_tag}");
            let build = |seed: &str| -> (Site, PathBuf) {
                let repo = if metachar {
                    TempDir::new_metachar(&format!("{seed}-repo"))
                } else {
                    TempDir::new(&format!("{seed}-repo"))
                };
                let home = TempDir::new(&format!("{seed}-home"));
                init_repo(repo.path());
                let shadow = install_schema_shadow(repo.path(), "adr", &body);
                let site = Site {
                    repo: repo.path().to_path_buf(),
                    home: home.path().to_path_buf(),
                    _keep: vec![repo, home],
                };
                (site, shadow)
            };

            let (shared, shadow) = build(&format!("{shape_tag}-{path_tag}"));
            let shadow_path = shadow.display().to_string();
            let needles = ["schema-hash mismatch", "adr", shadow_path.as_str()];

            let route = sweep_drifted(&tag, &shared, &needles, &|seed| build(seed).0)
                .unwrap_or_else(|| panic!("[{tag}] a blocked door must emit a `route:` line"));

            // The route is followed **verbatim**, through a real shell, and it must clear
            // the block — a route that names the wrong file (or no file) reddens here.
            let ran = Command::new("sh")
                .arg("-c")
                .arg(&route)
                .current_dir(&shared.repo)
                .env("HOME", &shared.home)
                .output()
                .expect("spawn sh to follow the route");
            assert!(
                ran.status.success(),
                "[{tag}] the emitted route `{route}` must run; stderr:\n{}",
                String::from_utf8_lossy(&ran.stderr),
            );
            for door in FREEZE_DOORS.iter().filter(|door| reruns_clean(door)) {
                let out = shared.run(door.argv);
                assert!(
                    out.status.success(),
                    "[{tag}] `jigc {}` must run clean once the emitted route has been followed; stdout:\n{}\nstderr:\n{}",
                    door.argv.join(" "),
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr),
                );
            }
        }
    }
}

/// The omitting context's twin, and the reason the fence hashes the **resolved**
/// schema rather than refusing a shadow by name: a shadow that reworks only the
/// *authored prose* changes no frozen byte (M47's presentation projection erases
/// `description:` / `usage:` / slot `hint:`), so it composes clean at exit 0 — and the
/// reworded prose is genuinely live, visible through `jigc describe`. Refusing the
/// shadow by name would have deleted the capability `design/overrides.md` documents
/// for all sixteen shipped doctypes.
#[test]
fn a_presentation_only_project_schema_shadow_composes_clean() {
    let build = |seed: &str| -> Site {
        let repo = TempDir::new(&format!("{seed}-repo"));
        let home = TempDir::new(&format!("{seed}-home"));
        init_repo(repo.path());
        install_schema_shadow(repo.path(), "adr", &adr_shadow_reworded());
        Site {
            repo: repo.path().to_path_buf(),
            home: home.path().to_path_buf(),
            _keep: vec![repo, home],
        }
    };

    // The read verbs share one fixture — they mutate nothing. Every write verb gets its
    // own, because here the doors really *run*: with the freeze inert `jigc uninstall`
    // would remove the shadow the next door is meant to meet.
    let shared = build("shadow-prose");
    for door in FREEZE_DOORS {
        let acting;
        let site = if verb_of(door.argv).1 == VerbKind::Read {
            &shared
        } else {
            acting = build("shadow-prose-solo");
            &acting
        };
        let out = site.run(door.argv);
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        let said = format!("stdout:\n{stdout}\nstderr:\n{stderr}");
        assert!(
            !stdout.contains(FREEZE_BANNER) && !stderr.contains(FREEZE_BANNER),
            "`jigc {}` must not fire the freeze over a presentation-only schema shadow; {said}",
            door.argv.join(" "),
        );
        match door.clean {
            CleanExit::Zero => assert!(
                out.status.success(),
                "`jigc {}` is declared `CleanExit::Zero` — it must exit 0 with the gate \
                 inert; {said}",
                door.argv.join(" "),
            ),
            CleanExit::Needs(why) => assert!(
                !out.status.success(),
                "`jigc {}` is declared to need what this fixture lacks ({why}), so it must \
                 exit non-zero — if it now succeeds its row is stale; {said}",
                door.argv.join(" "),
            ),
        }
    }

    let describe = shared.run(&["describe"]);
    let stdout = String::from_utf8_lossy(&describe.stdout);
    assert!(
        stdout.contains(REWORDED_ADR_DESCRIPTION),
        "the shadow's reworded `description:` must be live through `jigc describe`; got:\n{stdout}",
    );
}

/// The wholesale opt-out reaches the project layer too: a shadow of a doctype whose
/// **owning pack ships no manifest** is unchecked, exactly as that pack's own schemas
/// are ([`manifest_less_pack_is_unaffected`]). The freeze records what a pack
/// *declares* frozen; a pack that declares nothing freezes nothing, at either layer.
#[test]
fn a_project_shadow_of_a_manifest_less_pack_doctype_loads_clean() {
    let repo = TempDir::new("shadow-nomanifest-repo");
    let home = TempDir::new("shadow-nomanifest-home");
    let pack = dev_pack_copy("shadow-nomanifest-pack");
    init_repo(repo.path());
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");
    install_schema_shadow(repo.path(), "adr", &adr_shadow_with_owner());

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "a project shadow of a manifest-less pack's doctype must load clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}
