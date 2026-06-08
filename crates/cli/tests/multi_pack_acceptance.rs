//! M14 Increment 1 / T5 — the real-binary acceptance for multi-pack composition:
//! the **single-pack byte-identity floor**, a **two-pack load** (union read), and
//! the **top-level precedence-winner** for a colliding doctype + knob — all driven
//! through the built `jigc` binary against throwaway git repos.
//!
//! This task asserts **loading + the floor + the top-level winner ONLY**
//! (`design/worked-examples.md` → flow 17 setup (a)+(b) and the walk's top-level
//! collision lines; `design/multi-pack.md` → Collision resolution). It MUST NOT
//! assert correct cross-pack workflow *body* composition — a loser-pack workflow's
//! colliding `{{include: step:X}}` / `{{cli.X}}` is knowingly mis-resolved until
//! increment 2, so asserting it here would be a tautology against an unbuilt
//! feature. The M12 `methodology_pack_*` compose-alone tests stay green untouched.
//!
//! The four observables, each on the EMITTED bytes the agent would actually see
//! (increment-workflow hardening #4):
//!   - (floor) with NO `packs.yaml`, bare `jigc start "<intent>"` composes the
//!     cascade-default router **byte-identical** to the single-pack golden (the same
//!     `NO_OVERRIDE_ROUTER_GOLDEN` `start_compose.rs` pins) — proving `Composite([base])`
//!     does not perturb the one-pack path (the headline regression);
//!   - (two-pack load) with `packs.yaml` listing the on-disk methodology pack OVER
//!     the embedded dev base, both packs' non-colliding workflows resolve through the
//!     union: `jigc describe` narrates methodology's `dev-task` AND dev's `single-task`;
//!   - (knob winner) `default-workflow` resolves to methodology's `dev-task` (it wins
//!     the whole-`knobs.yaml` shadow), so bare `jigc start "<intent>"` mints under
//!     `.jigc/tasks/<slug>/` — dev's router (the base's default) is `creates-task: false`
//!     and would mint NOTHING, so the mint is the proof the knob resolved to the winner;
//!   - (doctype winner) the `commit` schema resolves to methodology's, which dropped
//!     dev's `implements→spec` field: `jigc doc set-field commit:<id>#implements` blocks
//!     non-zero ("no field addressed"), where the single-pack dev base resolves it fine.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! methodology pack from `CARGO_MANIFEST_DIR/../../packs/methodology`, the temp repo
//! is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's real repo / files.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-multipack-accept-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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
/// directory a `.jigc/config/packs:` entry names.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// The single-pack floor golden: the no-override bare-`jigc start "<intent>"`
/// compose of the cascade-default router. This is byte-identical to the
/// `NO_OVERRIDE_ROUTER_GOLDEN` `start_compose.rs` pins — kept here as the floor
/// oracle the composite-of-one path must reproduce (the headline regression: the
/// multi-pack machinery adds zero observable change to a one-pack project).
const NO_OVERRIDE_ROUTER_GOLDEN: &str = "\
These are the selectable work-workflows, each with the situation it fits:

- architecture-documentation — document the architecture of a part of the system, tying its components to the code that implements them
- implement-from-spec — build from a committed spec whose acceptance criteria already exist
- plan — draft the specification for upcoming work before writing any code
- project-setup — bootstrap a brand-new project by developing the idea into its first product requirements
- quick-fix — apply a small commit-only fix with no decision to record
- single-task — implement one scoped change end-to-end

Pick the workflow whose situation best fits the intent, then re-run with that
choice and the original intent:

jigc start --workflow <chosen> \"<intent>\"
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// Initialize a real git repo with one commit (composition mints, which reads HEAD
/// via `git rev-parse`) and create the `.jigc/config/` project layer (the setup gate
/// the cascade + `describe` require).
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

/// Record the listed pack-set in the in-repo project layer's `packs.yaml` — the
/// pre-cascade selector `make_pack()` CWD-discovers. The named directory sits at
/// highest precedence; the embedded dev pack is the implicit base below it.
fn write_packs_yaml(repo: &Path, listed: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", listed.display()),
    )
    .expect("write packs.yaml naming the listed pack");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, optionally piping stdin.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

#[test]
fn floor_single_pack_compose_is_byte_identical_to_the_golden() {
    // (floor) NO `packs.yaml` ⇒ the pack-set is `[base]` (the embedded dev pack) ⇒
    // composition flows through `Composite([base])`, which must be byte-identical to
    // the single-pack path. Bare `jigc start "<intent>"` composes the cascade-default
    // router; the emitted bytes must equal the golden exactly. A composite that
    // perturbs the one-pack path is the headline regression this rejects.
    let repo = TempDir::new("floor");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert!(
        out.status.success(),
        "the no-packs.yaml bare compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout, NO_OVERRIDE_ROUTER_GOLDEN,
        "the composite-of-one compose must stay byte-identical to the single-pack golden",
    );
    // The router is `creates-task: false`: the floor mints nothing.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "the cascade-default router mints nothing — no .jigc/tasks/ dir may appear on the floor",
    );
}

#[test]
fn two_pack_load_unions_both_packs_non_colliding_workflows() {
    // (two-pack load) methodology listed OVER the embedded dev base. Both packs'
    // non-colliding workflows must resolve through the union read: `jigc describe`
    // narrates methodology's `dev-task` AND dev's `single-task` (and `router`). The
    // describe projection enumerates the *unfiltered* pack workflow set
    // (`introspection.md` → enumeration is over the unfiltered set), so the union is
    // directly observable on its emitted prose.
    let repo = TempDir::new("union");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["describe"]);
    assert!(
        out.status.success(),
        "`jigc describe` over the two-pack set must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // methodology's workflow (the listed, highest-precedence pack) ...
    assert!(
        stdout.contains("dev-task"),
        "describe must narrate methodology's `dev-task` (the listed pack's workflow); got:\n{stdout}",
    );
    // ... and the base (embedded dev) pack's workflows — the union, base last.
    assert!(
        stdout.contains("single-task"),
        "describe must still narrate dev's `single-task` (the base pack's workflow); got:\n{stdout}",
    );
    assert!(
        stdout.contains("router"),
        "describe must still narrate dev's `router` (the base pack's workflow); got:\n{stdout}",
    );
}

#[test]
fn default_workflow_knob_resolves_to_the_precedence_winner() {
    // (knob winner) `default-workflow` collides: methodology declares `[dev-task]`,
    // dev declares `[router, …]` (disjoint enums). Methodology wins the whole-
    // `knobs.yaml` shadow, so the resolved default is `dev-task` (`creates-task: true`).
    // The proof is behavioral on the emitted bytes: bare `jigc start "<intent>"` mints
    // under `.jigc/tasks/<slug>/` — dev's router (the base's default) is
    // `creates-task: false` and would mint NOTHING, so the minted working area can only
    // mean the knob resolved to methodology's value.
    let repo = TempDir::new("knob-winner");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert!(
        out.status.success(),
        "bare `jigc start` over the two-pack set must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // methodology's `dev-task` is `creates-task: true`, so the bare compose minted a
    // working area + base pin under the intent slug — the knob winner is `dev-task`.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("add-rate-limiter");
    assert!(
        task_dir.join("base.json").is_file(),
        "the resolved `default-workflow` must be methodology's `dev-task` (creates-task: true), \
         which mints .jigc/tasks/add-rate-limiter/ — dev's router default would mint nothing",
    );
}

#[test]
fn commit_doctype_resolves_to_the_precedence_winner() {
    // (doctype winner) the `commit` schema collides: dev's carries an
    // `implements→spec` ref field in its header; methodology's dropped it (the M12
    // subsume). Methodology wins the whole-schema shadow, so the resolved `commit` is
    // methodology's — which has NO `implements` field. The proof is behavioral on the
    // emitted bytes: `jigc doc set-field commit:<id>#implements` addresses a field the
    // resolved schema lacks and blocks non-zero ("no field addressed"). A control
    // `#type` set-field (present in BOTH schemas) succeeds, so the block is the
    // dropped field, not a broken commit doc.
    let repo = TempDir::new("doctype-winner");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    // Mint the task (methodology's `dev-task` is the resolved default) — this
    // provisions the `commit:<id>` doc the set-field verbs address.
    let started = run(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert!(
        started.status.success(),
        "the bare compose must mint the task; got {:?}\nstderr:\n{}",
        started.status,
        String::from_utf8_lossy(&started.stderr),
    );

    // Control: `#type` is in BOTH packs' commit schemas, so it sets cleanly — the
    // commit doc + the resolved schema are sound.
    let ok = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "commit:add-rate-limiter#type",
            "--value",
            "feat",
        ],
    );
    assert!(
        ok.status.success(),
        "`set-field commit#type --value feat` must succeed against the resolved commit schema; got {:?}\nstderr:\n{}",
        ok.status,
        String::from_utf8_lossy(&ok.stderr),
    );

    // The discriminator: `#implements` exists only on DEV's commit schema. Methodology
    // (the precedence winner) dropped it, so addressing it blocks non-zero.
    let blocked = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "commit:add-rate-limiter#implements",
            "--value",
            "spec:foo",
        ],
    );
    assert!(
        !blocked.status.success(),
        "the resolved (methodology) commit schema dropped `implements`, so set-field on it must block non-zero; got {:?}",
        blocked.status,
    );
    let stderr = String::from_utf8_lossy(&blocked.stderr);
    assert!(
        stderr.contains("implements"),
        "the block must name the unaddressable field (proving methodology's commit won the collision); got:\n{stderr}",
    );
}

#[test]
fn multi_pack_pack_header_names_both_packs_highest_first() {
    // (provenance) the multi-pack `Pack:` orientation header (T4) names the composed
    // pack-set highest-precedence first, joined by ` | `: methodology (listed) then dev
    // (base). Bare `jigc start` (no intent) is the read-only orientation path that emits
    // it. On the floor (one pack) the header degrades to today's single segment — proven
    // by the `floor` test composing byte-identically — so the bar here is only the
    // two-pack ordering.
    let repo = TempDir::new("provenance");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["start"]);
    assert!(
        out.status.success(),
        "bare `jigc start` (orientation) over the two-pack set must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // The `Pack:` header lists methodology (highest) then dev (base), ` | `-joined —
    // the composed-set provenance, highest-first. The version tails come from each
    // pack's own identity (methodology's defaults.yaml `version: 0.1.0`; dev's binary
    // version). Asserted on the literal ordered segment, not reconstructed.
    assert!(
        stdout.contains("Pack: methodology/0.1.0 | dev/"),
        "the orientation header must name the composed pack-set highest-first (methodology | dev); got:\n{stdout}",
    );
}
