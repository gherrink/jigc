//! The genuine `v1 → v2` upgrade-reconciliation acceptance + the re-pin→clean loop
//! (M5 increment 4, T4 — `design/worked-examples.md` → flow 7; `design/overrides.md`
//! → The `jigc upgrade` command).
//!
//! This is the milestone's headline proven **genuinely through the binary** across a
//! real pack change: *no upstream change silently lost; no override silently broken*.
//! It drives the built `jigc` binary end-to-end over the `JIGC_PACK_DIR` two-pack
//! seam — the same `make_pack` factory the *recording* verbs and the *upgrade* path
//! share, so a basis recorded under one pack compares against the same env-selected
//! pack (`overrides.md` → the `FilesystemPack` seam).
//!
//! The walk, flow-7 verbatim onto the embedded pack's real step ids.
//!
//! Step 1 — under `JIGC_PACK_DIR=<v1>` (a faithful copy of the embedded pack tree)
//! record the flow-7 delta spread through the **real `jigc config` verbs**: a
//! `scalar-set` (`config set default-workflow single-task`); a `slot-fill` on the
//! pack's only declared extension point (`config fill step:implement#extra-guidance`);
//! two `tracked-fork`s with pinned basis (`config fork` on `locate` and `finalize`); a
//! `replace` with pinned basis (`config replace-step` on `implement`); plus one
//! **hand-authored** M4-era `replace` on `superseded-context` with **no** `base-hash`
//! (the sanctioned delta-form manifest edit).
//!
//! Step 2 — build `<v2>`: `implement` bytes changed **and** its `{{fill:
//! extra-guidance}}` point dropped; `finalize` bytes changed; `locate` untouched; the
//! body otherwise identical.
//!
//! Step 3 — `JIGC_PACK_DIR=<v2> jigc upgrade` classifies, on the **emitted bytes**:
//! `clean` (the scalar-set + the unchanged `locate` fork — emit nothing); `conflict`
//! (`replace #implement` AND the `finalize` fork — the fork shadow-bypass case, where
//! the probe re-reads the *pack's* `finalize`, not the fork's own shadow copy, so a
//! genuinely-changed upstream unit fires); `orphaned` (the `slot-fill` whose `{{fill:}}`
//! point v2 dropped — a context that **omits** the target, increment-workflow.md
//! hardening #5); `needs-rebasing` (the basis-less hand-authored `replace`); exit
//! non-zero.
//!
//! Step 4 — resolve via the `jigc config` verbs (re-record the conflicted /
//! needs-rebasing deltas — re-pinning their basis to v2 — and drop the orphaned
//! slot-fill by the sanctioned manifest delta-form edit), then `JIGC_PACK_DIR=<v2> jigc
//! upgrade` again asserts **all-clean, exit zero** — a clean re-run is the verification.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-upgrade-v1v2-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
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

/// The embedded pack source tree (`crates/cli/pack/`) — `CARGO_MANIFEST_DIR` is
/// `<root>/crates/cli`, the very tree `include_dir!` embeds into the binary. The v1
/// pack is a faithful copy of it, so the recording verbs read byte-identical pack
/// bytes (mirroring `pack_source_determinism.rs`).
fn embedded_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read source tree").flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Copy the embedded pack tree into `dir` and stamp `config/defaults.yaml`'s
/// `version:` (the `FilesystemPack` reads it through `pack_version`; narrative-only
/// for classification but flow 7 sets distinct `0.3.0`/`0.4.0`). Returns the pack
/// root the `JIGC_PACK_DIR` selection points at.
fn dir_pack_with_version(dir: &Path, version: &str) -> PathBuf {
    copy_tree(&embedded_pack_tree(), dir);
    let defaults = dir.join("config").join("defaults.yaml");
    let existing = fs::read_to_string(&defaults).expect("read copied defaults.yaml");
    fs::write(&defaults, format!("{existing}version: {version}\n"))
        .expect("write versioned defaults.yaml");
    dir.to_path_buf()
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer
/// so the cascade resolves (mirrors `pack_source_determinism.rs` / `config_fork.rs`).
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
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc config <args>` with `cwd = repo`, `$HOME = home`, the recording pack
/// selected by `JIGC_PACK_DIR=<v1>`, and optional `stdin` (for `fill --from-file -`).
fn run_config(
    repo: &Path,
    home: &Path,
    pack_dir: &Path,
    args: &[&str],
    stdin: Option<&str>,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("config").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir);
    if let Some(input) = stdin {
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = command.spawn().expect("spawn jigc config");
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(input.as_bytes())
            .expect("write stdin");
        child.wait_with_output().expect("wait jigc config")
    } else {
        command.output().expect("run the jigc binary")
    }
}

/// Run `jigc upgrade` with `cwd = repo`, `$HOME = home`, and the **current** pack
/// selected by `JIGC_PACK_DIR=<v2>` — the same env-selected obligation the recording
/// verbs honored under v1.
fn run_upgrade(repo: &Path, home: &Path, pack_dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("upgrade")
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

fn config_dir(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("config")
}

/// Assert a `jigc config` invocation succeeded, surfacing stderr on failure.
fn expect_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Append a hand-authored `replace-step` delta with **no** `base-hash` to the
/// project manifest — the sanctioned M4-era delta-form edit (`overrides.md` →
/// Hand-editing is allowed only in delta form). The `with:` references a native step
/// file we drop alongside so the replaced id resolves; the missing basis is exactly
/// what makes the upgrade classify it `needs-rebasing`.
fn hand_author_basisless_replace(repo: &Path) {
    let config = config_dir(repo);
    // The native replacement step (id = basename) — re-includes the pack step so the
    // composition stays valid; its presence is irrelevant to classification (which
    // re-reads the *pack* `superseded-context`, not this shadow).
    let steps = config.join("steps");
    fs::create_dir_all(&steps).expect("mk steps");
    fs::write(
        steps.join("project-super.yaml"),
        "{{ include: step:superseded-context }}\nProject house rule for superseding.\n",
    )
    .expect("write native step");

    let manifest_path = config.join("manifest.yaml");
    let text = fs::read_to_string(&manifest_path).expect("read manifest");
    let mut doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).expect("manifest is valid YAML");
    let map = doc.as_mapping_mut().expect("manifest is a mapping");
    let deltas = map
        .entry(serde_yaml_ng::Value::String("deltas".to_owned()))
        .or_insert_with(|| serde_yaml_ng::Value::Sequence(Vec::new()));
    let list = deltas.as_sequence_mut().expect("deltas is a sequence");
    // A `replace-step` entry carrying NO `base-hash`/`base-version` — the M4-era shape.
    let entry = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
        "kind: replace-step\ntarget: workflow:single-task#superseded-context\nwith: step:project-super\n",
    )
    .expect("entry yaml");
    list.push(entry);
    fs::write(
        &manifest_path,
        serde_yaml_ng::to_string(&doc).expect("serialize"),
    )
    .expect("write manifest");
}

/// Drop the orphaned `slot-fill` delta by the sanctioned manifest delta-form edit
/// (`worked-examples.md` flow 7 → resolve: "drop the orphaned slot-fill by
/// hand-editing the manifest"). Removes the `slot-fill` entry from `deltas:`.
fn drop_orphaned_slot_fill(repo: &Path) {
    let manifest_path = config_dir(repo).join("manifest.yaml");
    let text = fs::read_to_string(&manifest_path).expect("read manifest");
    let mut doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).expect("manifest is valid YAML");
    let list = doc
        .as_mapping_mut()
        .and_then(|m| m.get_mut("deltas"))
        .and_then(serde_yaml_ng::Value::as_sequence_mut)
        .expect("deltas sequence");
    list.retain(|e| e.get("kind").and_then(serde_yaml_ng::Value::as_str) != Some("slot-fill"));
    fs::write(
        &manifest_path,
        serde_yaml_ng::to_string(&doc).expect("serialize"),
    )
    .expect("write manifest");
}

/// Drop a stale `replace-step` delta (by `target`) and its native step file
/// `steps/<basename>.yaml` by the sanctioned manifest delta-form edit, so a
/// re-record of the same target/basename does not collide on the native-step id
/// (`config_fork.rs`'s already-forked rejection has the symmetric shape for forks).
/// Re-recording afterward re-pins the basis pack-direct against the current pack.
fn drop_replace_step(repo: &Path, target: &str, basename: &str) {
    let config = config_dir(repo);
    let manifest_path = config.join("manifest.yaml");
    let text = fs::read_to_string(&manifest_path).expect("read manifest");
    let mut doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).expect("manifest is valid YAML");
    let list = doc
        .as_mapping_mut()
        .and_then(|m| m.get_mut("deltas"))
        .and_then(serde_yaml_ng::Value::as_sequence_mut)
        .expect("deltas sequence");
    list.retain(|e| {
        !(e.get("kind").and_then(serde_yaml_ng::Value::as_str) == Some("replace-step")
            && e.get("target").and_then(serde_yaml_ng::Value::as_str) == Some(target))
    });
    fs::write(
        &manifest_path,
        serde_yaml_ng::to_string(&doc).expect("serialize"),
    )
    .expect("write manifest");
    let native = config.join("steps").join(format!("{basename}.yaml"));
    if native.exists() {
        fs::remove_file(&native).expect("rm stale native step file");
    }
}

/// Build the `<v2>` pack from a fresh copy of the embedded tree, then mutate it:
/// `implement` bytes change **and** drop the `{{fill: extra-guidance}}` point;
/// `finalize` bytes change; `locate` stays byte-identical to v1. Returns the pack root.
fn build_v2(dir: &Path) -> PathBuf {
    let root = dir_pack_with_version(dir, "0.4.0");
    let steps = root.join("steps");
    // implement: changed body, AND the `{{fill: extra-guidance}}` point removed.
    // It still solicits a `doc set-slot` write through the catalog, so it owes the
    // M48 read-back statement + declaration (the write-solicit tier of the same
    // stated-at fence the `finalize` rewrite below stays conformant with).
    fs::write(
        steps.join("implement.yaml"),
        "---\nstates-constraints: [read.staged-read-back]\n---\n\
         Implement the change directly in the working tree (v2 rewrite).\n\n\
         {{ cli.set-commit-summary }}\n<<author: {{ task.commit#summary }}>>\n\n\
         Read your write back: jigc doc show commit:{{task.id}} --task {{task.id}}\n",
    )
    .expect("write v2 implement");
    // finalize: changed body (the fork's upstream moved on). The
    // `states-constraints:` declaration is kept AND the three staging contracts it
    // declares are re-stated in the rewritten prose — a manifest-shipping pack must
    // stay stated-at-fence-conformant down to the M47 named-fact tier (a kept
    // declaration over prose that states nothing is exactly what that fence
    // refuses), and this test's subject is the changed BODY, not a withdrawn or
    // hollowed declarer.
    fs::write(
        steps.join("finalize.yaml"),
        "---\n\
         states-constraints: [finalize.left-out, finalize.nothing-staged, finalize.carried-staged]\n\
         ---\n\
         Validate and commit the task as one logical commit (v2 rewrite). Unstaged edits\n\
         and untracked files are left out; with nothing staged it refuses. Anything staged\n\
         from BEFORE this task was minted makes it refuse too — unstage it, or pass\n\
         `--carry-staged`:\n\n\
         {{ cli.finalize-task }}\n",
    )
    .expect("write v2 finalize");
    // locate: deliberately untouched (the copy from v1 stands) → the unchanged fork.
    root
}

#[test]
fn genuine_v1_v2_upgrade_classifies_every_outcome_then_repins_to_all_clean() {
    let repo = TempDir::new("repo");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // The two genuine packs across the `v1 → v2` boundary.
    let v1_dir = TempDir::new("pack-v1");
    let v1 = dir_pack_with_version(v1_dir.path(), "0.3.0");
    let v2_dir = TempDir::new("pack-v2");
    let v2 = build_v2(v2_dir.path());

    // ---- Step 1: record the flow-7 delta spread under JIGC_PACK_DIR=<v1> ----
    // scalar-set → clean.
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v1,
            &["set", "default-workflow", "single-task"],
            None,
        ),
        "config set default-workflow",
    );
    // slot-fill on the pack's `{{fill: extra-guidance}}` point → orphaned in v2 (the
    // point is dropped). Content via stdin (`--from-file -`).
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v1,
            &["fill", "step:implement#extra-guidance", "--from-file", "-"],
            Some("Project lint: run the house formatter before staging.\n"),
        ),
        "config fill step:implement#extra-guidance",
    );
    // tracked-fork on `locate` → unchanged in v2 → clean (basis pinned to v1 locate).
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v1,
            &["fork", "workflow:single-task#locate"],
            None,
        ),
        "config fork locate",
    );
    // tracked-fork on `finalize` → changed in v2 → conflict (the fork shadow-bypass:
    // the probe re-reads the *pack's* finalize, not the fork's own shadow copy).
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v1,
            &["fork", "workflow:single-task#finalize"],
            None,
        ),
        "config fork finalize",
    );
    // replace-step on `implement` → changed in v2 → conflict (basis pinned to v1 implement).
    let project_impl = repo.path().join("project-impl.yaml");
    fs::write(
        &project_impl,
        "{{ include: step:implement }}\nProject house rule for implementing.\n",
    )
    .expect("write replacement source");
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v1,
            &[
                "replace-step",
                "workflow:single-task#implement",
                project_impl.to_str().expect("utf-8 path"),
            ],
            None,
        ),
        "config replace-step implement",
    );
    // plus the M4-era basis-less replace on `superseded-context` → needs-rebasing.
    hand_author_basisless_replace(repo.path());

    // The manifest carries every recorded delta before the upgrade (recording
    // happened; nothing mutated yet).
    let manifest_before = fs::read_to_string(config_dir(repo.path()).join("manifest.yaml"))
        .expect("manifest readable");

    // ---- Step 2: JIGC_PACK_DIR=<v2> jigc upgrade — classify every outcome ----
    let out = run_upgrade(repo.path(), home.path(), &v2);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        !out.status.success(),
        "the v2 upgrade with blocking findings must exit non-zero; got {:?}\nstdout:\n{stdout}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // conflict — `replace #implement` (content changed upstream). Emitted-bytes assert.
    assert!(
        stdout.contains("blocking · override-default.content-changed — ")
            && stdout.contains("workflow:single-task#implement"),
        "the changed `replace #implement` must emit a blocking content-changed conflict; got:\n{stdout}",
    );
    // conflict — the `finalize` fork (the shadow-bypass case). Same code, the fork's target.
    assert!(
        stdout.contains("workflow:single-task#finalize"),
        "the changed `finalize` fork must emit a blocking conflict (the shadow-bypass case); got:\n{stdout}",
    );
    // orphaned — the `slot-fill` whose `{{fill:}}` point v2 dropped (a context that
    // OMITS the target — increment-workflow.md hardening #5).
    assert!(
        stdout.contains("blocking · override-default.slot-fill-orphaned — ")
            && stdout.contains("step:implement#extra-guidance"),
        "the dropped `{{fill: extra-guidance}}` point must orphan the slot-fill; got:\n{stdout}",
    );
    // needs-rebasing — the basis-less hand-authored replace.
    assert!(
        stdout.contains("blocking · override-default.needs-rebasing — ")
            && stdout.contains("workflow:single-task#superseded-context"),
        "the basis-less `replace #superseded-context` must classify needs-rebasing; got:\n{stdout}",
    );
    // clean (scalar-set + the unchanged `locate` fork) emit NOTHING — neither target
    // appears in a finding line. The fork conflict above proves the probe is
    // pack-direct (else the unchanged-fork would have shadowed the changed one).
    assert!(
        !stdout.contains("workflow:single-task#locate"),
        "the unchanged `locate` fork is clean — it must emit no finding; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("default-workflow"),
        "the scalar-set is clean — it must emit no finding; got:\n{stdout}",
    );
    // Every blocking finding carries the indented `route:` envelope.
    assert!(
        stdout.contains("\n  route: "),
        "each blocking finding must carry an indented `route:` line; got:\n{stdout}",
    );

    // Report-and-route only: the manifest is byte-unchanged on disk after upgrade.
    assert_eq!(
        fs::read_to_string(config_dir(repo.path()).join("manifest.yaml"))
            .expect("manifest still readable"),
        manifest_before,
        "`jigc upgrade` mutates nothing — the manifest must be byte-unchanged",
    );

    // ---- Step 3: resolve via the `jigc config` verbs, then re-run all-clean ----
    // conflict `replace #implement` → re-record re-pins the basis to v2's implement.
    // The v1 record already wrote `steps/project-impl.yaml` + the replace delta, so a
    // re-record with the same basename collides; drop the stale delta + native file
    // (the sanctioned delta-form edit) first, then re-record pack-direct against v2.
    drop_replace_step(
        repo.path(),
        "workflow:single-task#implement",
        "project-impl",
    );
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v2,
            &[
                "replace-step",
                "workflow:single-task#implement",
                project_impl.to_str().expect("utf-8 path"),
            ],
            None,
        ),
        "re-record replace-step implement against v2",
    );
    // conflict `finalize` fork → re-fork against v2's finalize re-pins the basis. The
    // first fork shadowed `finalize`; a re-fork of an already-forked unit is rejected,
    // so drop the stale fork delta + native file first (the sanctioned delta-form edit),
    // then re-fork pack-direct against v2.
    refork_finalize_against_v2(repo.path(), home.path(), &v2);
    // needs-rebasing `replace #superseded-context` → re-record pins a basis against v2.
    // The hand-authored M4-era replace already registered `steps/project-super.yaml` +
    // its delta; drop both (the sanctioned delta-form edit), then re-record pack-direct.
    drop_replace_step(
        repo.path(),
        "workflow:single-task#superseded-context",
        "project-super",
    );
    let project_super = repo.path().join("project-super.yaml");
    fs::write(
        &project_super,
        "{{ include: step:superseded-context }}\nProject house rule for superseding.\n",
    )
    .expect("write super replacement source");
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v2,
            &[
                "replace-step",
                "workflow:single-task#superseded-context",
                project_super.to_str().expect("utf-8 path"),
            ],
            None,
        ),
        "re-record replace-step superseded-context against v2",
    );
    // orphaned slot-fill → drop by the sanctioned manifest delta-form edit.
    drop_orphaned_slot_fill(repo.path());

    // Re-run the upgrade against v2 — a clean re-run is the verification.
    let out = run_upgrade(repo.path(), home.path(), &v2);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "after resolving every route the v2 re-run must exit zero; got {:?}\nstdout:\n{stdout}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        stdout.contains("no findings"),
        "the all-clean re-run must emit the positive no-findings line; got:\n{stdout}",
    );
}

/// Re-fork `finalize` against v2: the v1 fork already shadows `finalize` (a re-fork of
/// an already-forked unit is rejected — `config_fork.rs`), so drop the stale
/// `tracked-fork` delta + its native `steps/finalize.yaml` (the sanctioned delta-form
/// edit) before re-running `config fork`, which re-pins the basis pack-direct against
/// v2's `finalize`.
fn refork_finalize_against_v2(repo: &Path, home: &Path, v2: &Path) {
    let config = config_dir(repo);
    // Drop the stale fork delta from the manifest.
    let manifest_path = config.join("manifest.yaml");
    let text = fs::read_to_string(&manifest_path).expect("read manifest");
    let mut doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).expect("manifest is valid YAML");
    let list = doc
        .as_mapping_mut()
        .and_then(|m| m.get_mut("deltas"))
        .and_then(serde_yaml_ng::Value::as_sequence_mut)
        .expect("deltas sequence");
    list.retain(|e| {
        !(e.get("kind").and_then(serde_yaml_ng::Value::as_str) == Some("tracked-fork")
            && e.get("target").and_then(serde_yaml_ng::Value::as_str)
                == Some("workflow:single-task#finalize"))
    });
    fs::write(
        &manifest_path,
        serde_yaml_ng::to_string(&doc).expect("serialize"),
    )
    .expect("write manifest");
    // Drop the stale native fork file so the re-fork's no-collision check passes.
    fs::remove_file(config.join("steps").join("finalize.yaml")).expect("rm stale fork file");
    // Re-fork pack-direct against v2's finalize.
    expect_ok(
        &run_config(
            repo,
            home,
            v2,
            &["fork", "workflow:single-task#finalize"],
            None,
        ),
        "re-fork finalize against v2",
    );
}

/// **Existence-only kinds through the real binary — `insert` orphaned (hardening
/// #5: a v2 pack that OMITS the anchor step).** Under v1 record an `insert-step`
/// anchored `--after locate` (a step v1 ships), then build a v2 that **drops**
/// `steps/locate.yaml`. `JIGC_PACK_DIR=<v2> jigc upgrade` must emit, **on the
/// emitted bytes**, a blocking `override-default.target-exists` orphaned finding
/// naming the anchor target `workflow:single-task#locate`, and exit non-zero —
/// the insert is existence-only (anchor gone → orphaned, never conflicts).
#[test]
fn insert_over_a_dropped_anchor_classifies_orphaned_through_the_binary() {
    let repo = TempDir::new("insert-repo");
    init_repo(repo.path());
    let home = TempDir::new("insert-home");

    let v1_dir = TempDir::new("insert-pack-v1");
    let v1 = dir_pack_with_version(v1_dir.path(), "0.3.0");

    // Record an `insert-step` anchored after `locate` (a v1 step). The native step
    // file's basename is its id; the delta references `step:project-extra`.
    let extra = repo.path().join("project-extra.yaml");
    fs::write(&extra, "Project-only extra step body.\n").expect("write insert source");
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v1,
            &[
                "insert-step",
                "--workflow",
                "single-task",
                "--after",
                "locate",
                extra.to_str().expect("utf-8 path"),
            ],
            None,
        ),
        "config insert-step --after locate",
    );

    // Build a v2 that DROPS the `locate` anchor step entirely.
    let v2_dir = TempDir::new("insert-pack-v2");
    let v2 = dir_pack_with_version(v2_dir.path(), "0.4.0");
    fs::remove_file(v2.join("steps").join("locate.yaml")).expect("drop v2 locate step");

    let out = run_upgrade(repo.path(), home.path(), &v2);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        !out.status.success(),
        "the orphaned insert blocks the upgrade — exit non-zero; got {:?}\nstdout:\n{stdout}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        stdout.contains("blocking · override-default.target-exists — ")
            && stdout.contains("workflow:single-task#locate"),
        "the insert over the dropped `locate` anchor must emit a blocking orphaned finding; got:\n{stdout}",
    );
    assert!(
        stdout.contains("\n  route: "),
        "the orphaned insert finding carries an indented `route:` line; got:\n{stdout}",
    );
}

/// **Existence-only kinds through the real binary — `scalar-set` orphaned (hardening
/// #5: a v2 pack that OMITS the knob key).** Under v1 record a `scalar-set` on the
/// `default-workflow` knob, then build a v2 whose `config/knobs.yaml` **drops** the
/// `default-workflow` declaration (the remaining knobs keep their defaults, so the
/// surface still loads). `JIGC_PACK_DIR=<v2> jigc upgrade` must emit, **on the
/// emitted bytes**, a blocking `override-default.scalar-set-orphaned` finding naming
/// `scalar:default-workflow`, and exit non-zero — existence-only, never conflicts.
#[test]
fn scalar_set_over_a_dropped_knob_classifies_orphaned_through_the_binary() {
    let repo = TempDir::new("scalar-repo");
    init_repo(repo.path());
    let home = TempDir::new("scalar-home");

    let v1_dir = TempDir::new("scalar-pack-v1");
    let v1 = dir_pack_with_version(v1_dir.path(), "0.3.0");

    // Record a `scalar-set` on the `default-workflow` knob (declared in v1).
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v1,
            &["set", "default-workflow", "single-task"],
            None,
        ),
        "config set default-workflow",
    );

    // Build a v2 whose knob surface DROPS `default-workflow`, keeping only the two
    // still-declared validation-severity knobs (each retains a default → loads) —
    // plus the 11 intrinsic per-check knobs the engine's load-time assertion
    // requires (generated from the engine's intrinsic set so the pack still loads).
    let v2_dir = TempDir::new("scalar-pack-v2");
    let v2 = dir_pack_with_version(v2_dir.path(), "0.4.0");
    let intrinsic: String = engine::knobs::INTRINSIC_CHECK_KEYS
        .iter()
        .map(|k| {
            format!(
                "{k}:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n  floor: blocking\n"
            )
        })
        .collect();
    fs::write(
        v2.join("config").join("knobs.yaml"),
        format!(
            "validation.workflow-refs.severity:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n\
             validation.file-state.severity:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n{intrinsic}",
        ),
    )
    .expect("write v2 knobs without default-workflow");

    let out = run_upgrade(repo.path(), home.path(), &v2);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        !out.status.success(),
        "the orphaned scalar-set blocks the upgrade — exit non-zero; got {:?}\nstdout:\n{stdout}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        stdout.contains("blocking · override-default.scalar-set-orphaned — ")
            && stdout.contains("scalar:default-workflow"),
        "the scalar-set over the dropped `default-workflow` knob must emit a blocking orphaned finding; got:\n{stdout}",
    );
    assert!(
        stdout.contains("\n  route: "),
        "the orphaned scalar-set finding carries an indented `route:` line; got:\n{stdout}",
    );
}
