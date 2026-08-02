//! End-to-end integration test for `jigc upgrade` (M5 increment 4, T3).
//!
//! Drives the built `jigc` binary in a seeded repo and asserts the agent-facing
//! contract **on the emitted bytes** (increment-workflow.md hardening #4): the
//! command runs the `override-default` classifier over the recorded deltas against
//! the current (`JIGC_PACK_DIR`-selected) pack, renders findings + routes through
//! the standard renderer, and gates the exit code on `report.has_blocking()` —
//! **report-and-route only**, mutating nothing (`design/overrides.md` → The `jigc
//! upgrade` command, step 3).
//!
//! Two halves of the done-criterion:
//!   (a) a manifest with a blocking-classifying delta (a `remove-step` whose target
//!       the pack omits → `orphaned`) exits non-zero and the emitted output carries
//!       the finding's `severity · code — message` + an indented `route:` line;
//!   (b) an all-clean / no-delta manifest exits zero with the positive no-findings
//!       line.
//! The genuine v1→v2 two-pack walk + re-pin→clean loop is T4 (`flow 7`); this test
//! pins the dispatch + render + gate contract through the binary.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-upgrade-e2e-{tag}-{}-{:?}",
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

/// Seed a repo: a `.git` marker (repo-root discovery walks to it) and a
/// `.jigc/config/` project layer carrying `manifest_text`.
fn seed_repo(root: &Path, manifest_text: &str) {
    fs::create_dir_all(root.join(".git")).expect("mk .git");
    let config = root.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("mk .jigc/config");
    fs::write(config.join("manifest.yaml"), manifest_text).expect("write manifest");
}

/// Seed a `JIGC_PACK_DIR` filesystem pack carrying the given `(stem, body)` step
/// files plus a minimal `config/knobs.yaml` declaring `default-workflow` — the
/// current pack the classifier compares the recorded deltas against. The knob
/// surface is seeded so a recorded `scalar-set default-workflow` classifies clean
/// (its key is still declared); the scalar-set existence question reads this surface.
fn seed_pack(dir: &TempDir, steps: &[(&str, &str)]) -> PathBuf {
    let pack_root = dir.path().join("pack");
    let steps_dir = pack_root.join("steps");
    fs::create_dir_all(&steps_dir).expect("mk pack/steps");
    for (stem, body) in steps {
        fs::write(steps_dir.join(format!("{stem}.yaml")), body).expect("seed step");
    }
    let config_dir = pack_root.join("config");
    fs::create_dir_all(&config_dir).expect("mk pack/config");
    // The 11 intrinsic per-check knobs (each floored at `blocking`), generated from
    // the engine's intrinsic set so this minimal pack satisfies the load-time
    // intrinsic-floored assertion without hand-listing the surface.
    let intrinsic: String = engine::knobs::INTRINSIC_CHECK_KEYS
        .iter()
        .map(|k| {
            format!(
                "{k}:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n  floor: blocking\n"
            )
        })
        .collect();
    fs::write(
        config_dir.join("knobs.yaml"),
        // `default-workflow` (the resolve surface the no-delta tests lean on) plus the
        // `override-default.target-unchanged.severity` tunable — the closed enum the M6
        // warning-tier exit-code test demotes a conflict through — plus the intrinsic surface.
        format!(
            "default-workflow:\n  type: string\n  default: single-task\n\
             validation.override-default.target-unchanged.severity:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n{intrinsic}",
        ),
    )
    .expect("seed knobs.yaml");
    // `jigc upgrade` now resolves the project cascade to feed the M6 severity
    // post-pass (T3), reading `config/defaults`'s `pack-id` for the pack-default
    // layer provenance. A real pack always ships it; seed it here too.
    fs::write(config_dir.join("defaults.yaml"), "pack-id: test-pack\n")
        .expect("seed defaults.yaml");
    pack_root
}

/// The **no-delta** `jigc upgrade` rendered output over a blocking-classifying
/// `remove-step` delta whose target the current pack omits, byte for byte —
/// captured from the binary as the pre-M6 baseline. T3 threaded a real `Resolved`
/// into `upgrade_in_repo` to feed the M6 severity post-pass; this manifest carries
/// **no** `validation.*.severity` scalar-set, so the post-pass overrides nothing
/// and the rendered bytes must equal the pre-M6 baseline (`design/validation.md` →
/// Severity assignment — the M6 post-pass: the byte-identical golden must cover the
/// validate and upgrade paths; review B2).
const NO_DELTA_BLOCKING_UPGRADE_GOLDEN: &str = "\
blocking · override-default.target-exists — override target `workflow:single-task#gone` no longer exists in the current pack
  route: remove or re-target the delta on `workflow:single-task#gone`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// The **no-delta** `jigc upgrade` rendered output over a clean / no-delta manifest,
/// byte for byte — the positive no-findings path. The companion baseline to
/// `NO_DELTA_BLOCKING_UPGRADE_GOLDEN`: the post-pass must perturb neither the
/// blocking nor the clean render of the upgrade path.
/// (Round-2 D4: the clean line names WHAT was checked — the seeded manifest records
/// one `scalar-set` (`default-workflow`), so the count is 1; the old task-scoped
/// wording was the wrong noun on a config-delta sweep.)
const NO_DELTA_CLEAN_UPGRADE_GOLDEN: &str = "\
no findings — 1 recorded config delta(s) re-apply clean against the current pack
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// Run `jigc upgrade <args>` with `cwd = repo`, `$HOME = home`, and the given
/// `JIGC_PACK_DIR` selecting the current pack.
fn run_upgrade(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("upgrade");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn blocking_delta_exits_non_zero_with_the_finding_line_and_route() {
    // (a) A `remove-step` over `workflow:single-task#gone`; the current pack omits
    // `gone` entirely → the existence question orphans it, blocking. The command
    // exits non-zero and the EMITTED bytes carry the finding's
    // `severity · code — message` line + the indented `route:` line.
    let repo = TempDir::new("block");
    let home = TempDir::new("home");
    seed_repo(
        repo.path(),
        "\
deltas:
  - kind: remove-step
    target: workflow:single-task#gone
",
    );
    // The pack carries some *other* step but not `gone`.
    let pack_dir = seed_pack(&repo, &[("implement", "implement body\n")]);

    let out = run_upgrade(repo.path(), home.path(), &pack_dir, &[]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        !out.status.success(),
        "a blocking-classifying delta must exit non-zero; got {:?}\nstdout:\n{stdout}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    // The finding line: `<severity> · <code> — <message>`. Assert each constituent
    // on the emitted bytes (the agent-facing contract).
    assert!(
        stdout.contains("blocking · override-default.target-exists — "),
        "the emitted output must carry the `severity · code — message` finding line; got:\n{stdout}",
    );
    assert!(
        stdout.contains("workflow:single-task#gone"),
        "the finding message must name the orphaned delta's target; got:\n{stdout}",
    );
    // The block-payload envelope: a blocking finding carries an indented `route:`.
    assert!(
        stdout.contains("\n  route: "),
        "the emitted output must carry the indented `route:` line; got:\n{stdout}",
    );

    // Report-and-route only: the manifest is byte-unchanged on disk.
    let manifest = repo
        .path()
        .join(".jigc")
        .join("config")
        .join("manifest.yaml");
    assert!(
        fs::read_to_string(&manifest)
            .expect("manifest still readable")
            .contains("workflow:single-task#gone"),
        "`jigc upgrade` must mutate nothing — the manifest's delta survives",
    );
}

#[test]
fn no_delta_manifest_exits_zero_with_the_positive_no_findings_line() {
    // (b) A no-delta manifest (only a `scalar:` block) classifies clean → exit zero,
    // and the emitted output carries the positive no-findings line so the agent sees
    // a positive signal.
    let repo = TempDir::new("clean");
    let home = TempDir::new("home");
    seed_repo(repo.path(), "scalar:\n  default-workflow: single-task\n");
    let pack_dir = seed_pack(&repo, &[("implement", "implement body\n")]);

    let out = run_upgrade(repo.path(), home.path(), &pack_dir, &[]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "a clean / no-delta manifest must exit zero; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        stdout.contains("no findings"),
        "the emitted output must carry the positive no-findings line; got:\n{stdout}",
    );
}

#[test]
fn warning_demoted_conflict_exits_zero_through_the_upgrade_gate() {
    // T2 — the `warning` live tier, produce → render → exit through the REAL binary.
    // A `tracked-fork` conflict (the recorded `base-hash` ≠ the current pack's
    // `validate` body, emitted blocking under `override-default.target-unchanged`) is
    // demoted to `warning` by a recorded `validation.override-default.target-unchanged.
    // severity: warning` scalar-set. The upgrade gate maps `!has_blocking()` →
    // `ExitCode::SUCCESS`, so the warning-only report exits **0** (non-blocking), and
    // the emitted bytes carry the `warning · …` line — DISTINCT from `blocking`/
    // `advisory` (`design/validation.md` → `warning` is a live third tier;
    // `design/worked-examples.md` → flow 8). This is the exit-code half of the
    // done-criterion the unit `has_blocking()` proof anchors; the full flow-8 bundle
    // (tunable/intrinsic/byte-identical) is T3.
    let repo = TempDir::new("warning-exit");
    let home = TempDir::new("home");

    // The pack ships a v2 `validate` body; the recorded fork basis is the v1 hash → a
    // content conflict the classifier emits blocking under `target-unchanged`.
    let validate_v1 = "validate body v1\n";
    let recorded = engine::file_state::hash_bytes(validate_v1.as_bytes());
    let pack_dir = seed_pack(&repo, &[("validate", "validate body v2 — changed\n")]);

    seed_repo(
        repo.path(),
        &format!(
            "\
scalar:
  validation.override-default.target-unchanged.severity: warning
deltas:
  - kind: tracked-fork
    target: workflow:single-task#validate
    base-version: v1
    base-hash: {recorded}
"
        ),
    );

    let out = run_upgrade(repo.path(), home.path(), &pack_dir, &[]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "a warning-demoted conflict must exit 0 (the gate maps !has_blocking → success); got {:?}\nstdout:\n{stdout}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    // The emitted bytes carry the `warning · …` finding line — the live third tier.
    // The finding's `code` is `override-default.content-changed` (its post-pass tuning
    // key is the `target-unchanged` check); the render line leads with the code.
    assert!(
        stdout.contains("warning · override-default.content-changed — "),
        "the demoted conflict must render a `warning · code — message` line; got:\n{stdout}",
    );
    // Distinct from the other tiers: neither `blocking ·` nor `advisory ·` appears.
    assert!(
        !stdout.contains("blocking · ") && !stdout.contains("advisory · "),
        "the warning tier must render distinct from blocking/advisory; got:\n{stdout}",
    );
}

#[test]
fn no_delta_upgrade_render_is_byte_identical_to_the_baseline() {
    // T4 (M6 Increment 1) determinism guard: T3 threaded a real `Resolved` into
    // `upgrade_in_repo` so the M6 severity post-pass has a cascade to read. Building
    // that `Resolved` must not perturb the no-override render — `design/validation.md`
    // flags the upgrade path (not only `start_compose`) as the real M6 risk surface
    // (review B2). Both manifests below carry no `validation.*.severity` scalar-set, so
    // the post-pass overrides nothing and the emitted bytes must equal the captured
    // pre-M6 baselines — for the blocking AND the clean render.
    let repo = TempDir::new("byte-identical");
    let home = TempDir::new("home");
    let pack_dir = seed_pack(&repo, &[("implement", "implement body\n")]);

    // The blocking render: a `remove-step` orphaned by the current pack.
    seed_repo(
        repo.path(),
        "\
deltas:
  - kind: remove-step
    target: workflow:single-task#gone
",
    );
    let out = run_upgrade(repo.path(), home.path(), &pack_dir, &[]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        !out.status.success(),
        "the blocking delta must still exit non-zero; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        stdout, NO_DELTA_BLOCKING_UPGRADE_GOLDEN,
        "the no-delta blocking upgrade render must stay byte-identical to the pre-M6 baseline",
    );

    // The clean render: a no-delta manifest, same repo, re-seeded.
    seed_repo(repo.path(), "scalar:\n  default-workflow: single-task\n");
    let out = run_upgrade(repo.path(), home.path(), &pack_dir, &[]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "the clean manifest must still exit zero; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        stdout, NO_DELTA_CLEAN_UPGRADE_GOLDEN,
        "the no-delta clean upgrade render must stay byte-identical to the pre-M6 baseline",
    );
}
