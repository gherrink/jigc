//! M21 Increment 1 / T4 — the end-to-end real-binary acceptance for the embedded
//! `[dev ▸ methodology]` composition wired at `jigc setup` (the increment-1
//! Deliverable + Proves; `implementation/roadmap.md` → M21 Increment 1;
//! `design/multi-pack.md` → Embedded second pack + setup auto-wiring).
//!
//! Unlike `methodology_pack_compose.rs` (`JIGC_PACK_DIR=<methodology>`) and
//! `multi_pack_acceptance.rs` (a listed `packs.yaml` path), this drives the **marker**
//! path: a *normal* `jigc setup` writes the `compose-embedded-methodology` marker, and
//! the composite assembles `[dev ▸ methodology]` **dev-highest** off the binary-embedded
//! methodology tree — NO `JIGC_PACK_DIR`, NO listed packs. Dev-highest is the inverse of
//! the M14 listed-pack-highest tests: here **dev** wins every top-level collision, so the
//! cascade default stays `router` and `commit` keeps `implements→spec` (the Flow-A
//! milestone wall — methodology rides along for milestone/roadmap authoring without
//! shadowing dev's MVP surface).
//!
//! The binary changed across T1–T3 (a second `include_dir!` + factory + `setup`), so
//! `CARGO_BIN_EXE_jigc` rebuilds before this test runs. This test **asserts** the
//! composed surface those tasks built — it does not rebuild it.
//!
//! WITH the marker (after a normal `jigc setup`):
//!   (i)   `jigc start --explain` shows `config:knobs → won by dev` AND
//!         `doctype:commit → won by dev` (dev-highest collision resolution).
//!   (ii)  `jigc doc create roadmap` succeeds and `jigc start --workflow planning`
//!         composes/exits 0 — milestone authoring works out of the box.
//!   (iii) `jigc start --workflow dev-task` composes methodology's OWN test-first
//!         `implement` (pack-local `origin_pack`), not dev's direct-edit one.
//! WITHOUT the marker:
//!   (iv)  the surface + composition bytes are byte-identical to the pre-M21 dev-only
//!         path: no `collision:` line, `default-workflow` resolves to `router` from dev
//!         alone, methodology's `roadmap`/`planning` are absent.
//!   (v)   a `JIGC_PACK_DIR=<methodology>` run is unchanged (methodology-alone still wins
//!         `default-workflow → dev-task`) — `JIGC_PACK_DIR` supersedes the marker (T2a).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! methodology pack from `CARGO_MANIFEST_DIR/../../packs/methodology`, the temp repo is a
//! real `git init`, and a self-cleaning `TempDir` keeps the test off the developer's repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-embedded-methodology-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the literal
/// directory a `JIGC_PACK_DIR` env value names (the (v) supersession probe).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Initialize a real git repo with one commit (composition mints, which reads HEAD
/// via `git rev-parse`).
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
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting
/// `JIGC_PACK_DIR` from the harness environment (the marker path requires it ABSENT).
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

/// Like [`run`] but with `JIGC_PACK_DIR` set to the given pack dir (the (v) probe).
fn run_with_pack_dir(
    repo: &Path,
    home: &Path,
    pack_dir: &Path,
    args: &[&str],
) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

/// Stand up a repo whose project layer carries the `compose-embedded-methodology`
/// marker via a *normal* `jigc setup` (NO `JIGC_PACK_DIR`, NO listed packs) — the
/// marker path the increment wires. Returns `(repo, home)` self-cleaning temp dirs.
fn marker_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    init_repo(repo.path());
    let home = TempDir::new("home");
    let setup = run(repo.path(), home.path(), &["setup"]);
    assert!(
        setup.status.success(),
        "a normal `jigc setup` (no JIGC_PACK_DIR) must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );
    // The marker is what `make_pack` reads to compose `[dev ▸ methodology]` — assert it
    // landed in the project layer's `packs.yaml` (the T3 deliverable this path rides).
    let packs_yaml = repo.path().join(".jigc").join("config").join("packs.yaml");
    let marker = fs::read_to_string(&packs_yaml).expect("read packs.yaml after setup");
    assert!(
        marker.contains("compose-embedded-methodology: true"),
        "`jigc setup` must write the compose marker into {packs_yaml:?}; got:\n{marker}",
    );
    (repo, home)
}

#[test]
fn with_marker_explain_names_dev_as_the_collision_winner() {
    // (i) The marker composes `[dev ▸ methodology]` DEV-highest, so dev wins every
    // top-level collision. `jigc start --explain` names dev as the winner of BOTH the
    // `default-workflow` knob and the `commit` doctype — the inverse of the M14
    // listed-pack-highest tests (where methodology wins). Asserted on the EMITTED bytes.
    let (repo, home) = marker_repo("explain");

    let explain = run(
        repo.path(),
        home.path(),
        &["start", "--explain", "add rate limiter"],
    );
    assert!(
        explain.status.success(),
        "`jigc start --explain` over the marker path must exit 0; got {:?}\nstderr:\n{}",
        explain.status,
        String::from_utf8_lossy(&explain.stderr),
    );
    let out = String::from_utf8(explain.stdout).expect("utf-8 stdout");

    // The whole `knobs.yaml` resolves to DEV's (hence `default-workflow` → `router`) —
    // dev wins the collision, named by the resource actually adjudicated.
    assert!(
        out.contains("collision: config:knobs → won by dev/"),
        "(i) `--explain` must name dev as the `config/knobs` collision winner \
         (dev-highest); got:\n{out}",
    );
    // The `commit` doctype resolves to DEV's — dev wins (keeps `implements→spec`).
    assert!(
        out.contains("collision: doctype:commit → won by dev/"),
        "(i) `--explain` must name dev as the `commit` doctype collision winner; got:\n{out}",
    );
    // The resolved workflow is dev's `router` (dev's `default-workflow`), not
    // methodology's `dev-task` — the dev-highest resolution's behavioral tell.
    assert!(
        out.contains("workflow:router"),
        "(i) the resolved default-workflow must be dev's `router` (dev-highest); got:\n{out}",
    );
}

#[test]
fn with_marker_milestone_authoring_works_out_of_the_box() {
    // (ii) Methodology rides along under the marker: its `planning` workflow composes and
    // its `roadmap` singleton doctype is creatable — milestone authoring works on a fresh
    // `jigc setup`, with NO JIGC_PACK_DIR and NO listed packs.
    let (repo, home) = marker_repo("planning");

    // `planning` is `creates-task: true` — a bare `--workflow planning` mints `m99`
    // ("M99" slugified) and composes the planning spine.
    let planning = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "planning", "M99"],
    );
    assert!(
        planning.status.success(),
        "(ii) `jigc start --workflow planning` over the marker path must compose + exit 0; \
         got {:?}\nstderr:\n{}",
        planning.status,
        String::from_utf8_lossy(&planning.stderr),
    );
    let planning_out = String::from_utf8(planning.stdout).expect("utf-8 stdout");
    // The minted task dir is the behavioral proof methodology's `creates-task: true`
    // workflow resolved (dev ships no `planning`).
    assert!(
        repo.path().join(".jigc").join("tasks").join("m99").is_dir(),
        "(ii) `--workflow planning` must mint the task dir `.jigc/tasks/m99/`; got:\n{planning_out}",
    );
    // The planning spine emits methodology's running-doctype authoring (the singleton
    // create-refs) — its distinctive guidance, absent from dev alone.
    assert!(
        planning_out.contains("jigc doc create roadmap"),
        "(ii) the planning spine must emit methodology's roadmap authoring line; \
         got:\n{planning_out}",
    );

    // `jigc doc create roadmap` succeeds: methodology's `roadmap` singleton doctype
    // resolves under the marker (dev ships no `roadmap`). `roadmap` is `id-from: type`,
    // so its slug is the fixed `roadmap` and the address is `roadmap:roadmap`.
    let created = run(
        repo.path(),
        home.path(),
        &[
            "doc", "create", "roadmap", "--title", "Roadmap", "--task", "m99",
        ],
    );
    assert!(
        created.status.success(),
        "(ii) `jigc doc create roadmap` over the marker path must succeed — methodology's \
         `roadmap` doctype resolves out of the box; got {:?}\nstderr:\n{}",
        created.status,
        String::from_utf8_lossy(&created.stderr),
    );
    let addr = String::from_utf8(created.stdout).expect("utf-8 stdout");
    assert_eq!(
        addr.trim(),
        "roadmap:roadmap",
        "(ii) the created roadmap singleton's address is `roadmap:roadmap`",
    );
}

#[test]
fn with_marker_dev_task_composes_methodologys_own_test_first_implement() {
    // (iii) `--workflow dev-task` resolves METHODOLOGY's `dev-task` (dev ships none), and
    // its body-references resolve in METHODOLOGY's own pack (`origin_pack`): its test-first
    // `step:implement`, NOT dev's direct-edit one. Even though dev wins every top-level
    // collision, the loser pack's workflow keeps its own pack-local body — the M3-class
    // corruption the pack-local rule kills. Asserted on the distinguishing body text.
    let (repo, home) = marker_repo("dev-task");

    let out = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "dev-task", "add cache"],
    );
    assert!(
        out.status.success(),
        "(iii) `jigc start --workflow dev-task` over the marker path must compose + exit 0; \
         got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // No `{{ … }}` placeholder survives a clean compose.
    assert!(
        !stdout.contains("{{") && !stdout.contains("}}"),
        "(iii) no `{{{{ … }}}}` placeholder may survive the dev-task compose; got:\n{stdout}",
    );
    // Methodology's OWN test-first implement — its distinctive prose.
    assert!(
        stdout.contains("Implement the change test-first"),
        "(iii) `dev-task` must compose methodology's OWN test-first implement \
         (pack-local origin); got:\n{stdout}",
    );
    // ... and NOT dev's direct-edit implement (the `single-task` body's distinctive line),
    // proving body-references resolved pack-locally, not by precedence winner.
    assert!(
        !stdout.contains("Implement the change directly in the working tree"),
        "(iii) `dev-task` must NOT compose dev's direct-edit implement — body-references \
         resolve in methodology's own pack, not the precedence winner; got:\n{stdout}",
    );
}

#[test]
fn without_marker_surface_is_byte_identical_to_the_dev_only_floor() {
    // (iv) A project layer WITHOUT the marker (no `packs.yaml`) is the pre-M21 dev-only
    // floor: `--explain` carries NO `collision:` line (single-pack → no adjudicated
    // collision, hardening #5 — the omitting context), `default-workflow` resolves to
    // dev's `router` from dev alone, and methodology's `roadmap`/`planning` are absent.
    let repo = TempDir::new("no-marker");
    init_repo(repo.path());
    let home = TempDir::new("home");
    // Construct the project layer WITHOUT the marker — no `packs.yaml` at all (the
    // cleanest "no marker": absent file ⇒ `Ok(false)` ⇒ the single-pack `[base]` floor).
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("project layer");

    // `--explain`: no collision line, dev's `router` resolved.
    let explain = run(
        repo.path(),
        home.path(),
        &["start", "--explain", "add rate limiter"],
    );
    assert!(
        explain.status.success(),
        "(iv) the no-marker `--explain` must exit 0; got {:?}\nstderr:\n{}",
        explain.status,
        String::from_utf8_lossy(&explain.stderr),
    );
    let explain_out = String::from_utf8(explain.stdout).expect("utf-8 stdout");
    assert!(
        !explain_out.contains("collision:"),
        "(iv) the dev-only floor carries NO collision line (single-pack, hardening #5 — \
         the omitting context); got:\n{explain_out}",
    );
    // The label is byte-identical to the single-pack floor: dev names itself, at the
    // binary's own version — the composite-of-one is its own origin pack.
    assert!(
        explain_out.starts_with(&format!(
            "workflow:router    (pack-default · dev/v{})\n",
            env!("CARGO_PKG_VERSION"),
        )),
        "(iv) `default-workflow` must resolve to dev's `router` from dev alone, labelled \
         byte-identically to the single-pack floor; got:\n{explain_out}",
    );

    // methodology's `planning` is absent: `--workflow planning` blocks non-zero (no such
    // workflow), proving methodology did NOT compose without the marker.
    let planning = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "planning", "M99"],
    );
    assert!(
        !planning.status.success(),
        "(iv) without the marker, `--workflow planning` must block (methodology absent); \
         got exit {:?}",
        planning.status,
    );
    assert!(
        !repo.path().join(".jigc").join("tasks").join("m99").exists(),
        "(iv) the absent `planning` workflow mints nothing — no `.jigc/tasks/m99/` dir",
    );

    // methodology's `roadmap`/`dev-task` are absent from the dev-only describe projection.
    let describe = run(repo.path(), home.path(), &["describe"]);
    assert!(
        describe.status.success(),
        "(iv) the no-marker `jigc describe` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&describe.stderr),
    );
    let describe_out = String::from_utf8(describe.stdout).expect("utf-8 stdout");
    assert!(
        !describe_out.contains("dev-task"),
        "(iv) the dev-only floor must NOT narrate methodology's `dev-task`; got:\n{describe_out}",
    );
    assert!(
        describe_out.contains("single-task"),
        "(iv) the dev-only floor still narrates dev's `single-task`; got:\n{describe_out}",
    );
}

#[test]
fn with_marker_bare_start_orientation_names_both_off_catalog_verbs() {
    // (G6) Off-catalog discoverability: over the marker-composed `[dev ▸ methodology]`
    // pack-set, a bare `jigc start` orientation NAMES BOTH off-catalog entry verbs —
    // `planning` (methodology, `creates-task: true, selectable: false` — off the router
    // by the M16 invariant, NOT flipped) and `ingest-existing` (dev, `creates-task:
    // false`) — as next-step route-prose, while the `Available workflows:` catalog still
    // lists ONLY selectable work-workflows (neither off-catalog verb leaks into it). The
    // contract holds in BOTH the agent and human formats. Asserted on the EMITTED bytes.
    let (repo, home) = marker_repo("g6-discover");

    for format in [&["start"][..], &["--format", "human", "start"][..]] {
        let out = run(repo.path(), home.path(), format);
        assert!(
            out.status.success(),
            "(G6) bare `jigc {format:?}` over the marker path must exit 0; got {:?}\nstderr:\n{}",
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
        let text = String::from_utf8(out.stdout).expect("utf-8 stdout");

        // Both off-catalog verbs are NAMED as next steps (the route-prose lines).
        assert!(
            text.contains("jigc start --workflow planning"),
            "(G6 {format:?}) orientation must name the off-catalog `planning` verb; got:\n{text}",
        );
        assert!(
            text.contains("jigc start --workflow ingest-existing"),
            "(G6 {format:?}) orientation must name the off-catalog `ingest-existing` verb; got:\n{text}",
        );

        // Neither off-catalog verb leaks into the selectable `Available workflows:`
        // catalog — only the selectable work-workflows are listed there (M16 /
        // creates-task:false filters hold). The catalog block runs from its header to
        // the first `Run:` directive; assert neither verb appears as a catalog row.
        let catalog_start = text
            .find("Available workflows:\n")
            .expect("the catalog header is present")
            + "Available workflows:\n".len();
        let catalog_end = text[catalog_start..]
            .find("\nRun:")
            .map(|rel| catalog_start + rel)
            .unwrap_or(text.len());
        let catalog = &text[catalog_start..catalog_end];
        assert!(
            !catalog.contains("planning") && !catalog.contains("ingest-existing"),
            "(G6 {format:?}) no off-catalog verb may appear in the selectable catalog block; \
             catalog was:\n{catalog}",
        );
    }
}

#[test]
fn without_marker_bare_start_names_ingest_existing_but_not_planning() {
    // (G6, hardening #5 — the omitting context) A dev-only (no-marker) project layer is
    // the pre-M21 floor: dev ships `ingest-existing` (always present) but NOT `planning`
    // (methodology-only). Bare `jigc start` orientation NAMES `ingest-existing` and does
    // NOT name `planning` — the feature is inert where the verb is absent, never routing
    // to a non-resolving workflow. Asserted on the EMITTED bytes over the real binary.
    let repo = TempDir::new("g6-no-marker");
    init_repo(repo.path());
    let home = TempDir::new("home");
    // The cleanest "no marker": a project layer with no `packs.yaml` ⇒ the single-pack
    // dev-only `[base]` floor (methodology absent).
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("project layer");

    let out = run(repo.path(), home.path(), &["start"]);
    assert!(
        out.status.success(),
        "(G6 no-marker) bare `jigc start` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let text = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // `ingest-existing` is dev-present → named.
    assert!(
        text.contains("jigc start --workflow ingest-existing"),
        "(G6 no-marker) orientation must name dev's `ingest-existing` verb; got:\n{text}",
    );
    // `planning` is methodology-only → NOT named (the omitting context stays inert).
    assert!(
        !text.contains("--workflow planning"),
        "(G6 no-marker) orientation must NOT name `planning` (methodology absent — naming it \
         would route to a non-resolving verb); got:\n{text}",
    );
}

#[test]
fn jigc_pack_dir_supersedes_the_marker_methodology_alone_unchanged() {
    // (v) `JIGC_PACK_DIR` is the explicit/dogfood channel and SUPERSEDES the marker even
    // when one is present (T2a). A normal `jigc setup` writes the marker; a subsequent
    // `JIGC_PACK_DIR=<methodology>` run composes methodology-ALONE (NOT the `[dev ▸
    // methodology]` composite), so methodology wins `default-workflow → dev-task`
    // (`creates-task: true`) — bare `jigc start "<intent>"` mints, where dev's marker-path
    // router would mint nothing. The mint is the proof the marker was superseded.
    let repo = TempDir::new("pack-dir");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    // Setup UNDER JIGC_PACK_DIR (methodology-alone) — T2a: this writes NO marker (the
    // explicit channel does not auto-wire the embed), but even were a marker present the
    // env var supersedes it. Assert no marker leaked so the (v) supersession is honest.
    let setup = run_with_pack_dir(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "(v) `JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    // methodology-alone wins `default-workflow → dev-task` (`creates-task: true`): a bare
    // `jigc start "<intent>"` under JIGC_PACK_DIR mints the task working area.
    let started = run_with_pack_dir(
        repo.path(),
        home.path(),
        &pack,
        &["start", "add rate limiter"],
    );
    assert!(
        started.status.success(),
        "(v) bare `jigc start` under JIGC_PACK_DIR must exit 0; got {:?}\nstderr:\n{}",
        started.status,
        String::from_utf8_lossy(&started.stderr),
    );
    assert!(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter")
            .is_dir(),
        "(v) methodology-alone (JIGC_PACK_DIR, superseding any marker) wins \
         `default-workflow → dev-task` (creates-task: true), so the bare compose mints \
         `.jigc/tasks/add-rate-limiter/` — dev's marker-path router would mint nothing",
    );
}
