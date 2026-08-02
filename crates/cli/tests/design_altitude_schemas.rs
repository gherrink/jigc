//! M37 Increment 3 / T1 — the two ref-free design-altitude leaves (`research` +
//! `idea`) load and compose under the **`[dev ▸ methodology]`** composition, proven
//! through the real `jigc describe` binary (`design/design-altitude-doctypes.md`
//! §1/§2; `implementation/roadmap.md` → M37 Increment 3).
//!
//! The composition is the RC-trial's actual on-ramp: the on-disk methodology pack
//! listed in `packs.yaml` OVER the embedded dev base (the `multi_pack_acceptance.rs`
//! scaffolding — `init_repo`, `write_packs_yaml`, `methodology_pack_tree`, `run`).
//! Under it, `jigc describe` enumerates the *unfiltered* union doctype set and weaves
//! each doctype's authored `description:` / `usage:` into its facts-not-advice prose
//! ("`<type>` is `<description>`. Reach for it when `<usage>`."). So the observable
//! proof that both new schemas are **real and composable** is that describe:
//!   - exits 0 (a schema that failed to parse would `bail` out of `load_schemas`), AND
//!   - narrates BOTH `research` and `idea` with their authored prose verbatim.
//!
//! Both are ref-free one-per-doc leaves (no `type: ref` field), so T1 is self-
//! contained gate-green — no dangling type-ref, no `vision` (the `grounded-in → research`
//! ref lands in T2). Asserted on the EMITTED bytes of the real binary (hardening #4),
//! so a pack file that drops or garbles either doctype's prose fails here.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! methodology pack from `CARGO_MANIFEST_DIR/../../packs/methodology`, the temp repo
//! is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's real repo / files.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use engine::schema::{FieldType, SectionBody, load_schema};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-design-altitude-{tag}-{}-{:?}",
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
/// directory a `.jigc/config/packs.yaml` entry names.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer
/// (the setup gate the cascade + `describe` require).
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

/// Record the methodology pack in the project layer's `packs.yaml` — listed highest,
/// over the implicit embedded dev base: the `[dev ▸ methodology]` composition.
fn write_packs_yaml(repo: &Path, listed: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", listed.display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
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
fn describe_narrates_research_and_idea_under_dev_methodology() {
    let repo = TempDir::new("describe");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["describe"]);
    assert!(
        out.status.success(),
        "`jigc describe` over the `[dev ▸ methodology]` composition must exit 0 — a schema \
         that failed to parse would bail; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // `research` — the one-per-doc append-only record. Its authored `description:` and
    // `usage:` are woven facts-not-advice ("`<type>` is `<description>`. Reach for it
    // when `<usage>`."). Asserting both clauses proves the schema loaded, composed
    // under the union, and carries the authored prose verbatim.
    assert!(
        stdout.contains(
            "research is One investigation and what it found — the evidence a vision or design is formed from."
        ),
        "describe must narrate the `research` doctype's authored description; got:\n{stdout}",
    );
    assert!(
        stdout.contains(
            "Reach for it when a question needs evidence gathered and recorded before a vision or decision rests on it, and that record should stay addressable by what it grounds."
        ),
        "describe must narrate the `research` doctype's authored usage; got:\n{stdout}",
    );

    // `idea` — the one-per-doc parked shaped direction (coexists with
    // `deferral-ledger.kind=Idea`). Same woven-prose proof.
    assert!(
        stdout.contains(
            "idea is One shaped-but-unscheduled direction, with the trigger that would bring it back."
        ),
        "describe must narrate the `idea` doctype's authored description; got:\n{stdout}",
    );
    assert!(
        stdout.contains(
            "Reach for it when a direction is worth keeping but not worth scheduling now, so it needs a durable home cheaper than losing the thought and a note of when to revisit it."
        ),
        "describe must narrate the `idea` doctype's authored usage; got:\n{stdout}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// T2 — the `vision` singleton + the `grounded-in → research` ref (the pack's
// first internal managed ref). Two proofs (`design/design-altitude-doctypes.md`
// §1/§2/§4):
//   (a) `vision` COMPOSES under `[dev ▸ methodology]` (narrated by the real
//       `describe`) AND its schema declares the singleton + `display-title:
//       Vision` + `placement: { file: VISION.md }` knobs, the thesis/invariants/
//       open-questions slots, and the `grounded-in` ref (to `research`, `0..*`,
//       inverse `grounds`).
//   (b) the `grounded-in` `ref-resolves` gate PASSES when the target is a
//       committed `research`, and BLOCKS PER-ELEMENT on a dangling target —
//       driven through the REAL binary over the Shape-2 committed-target flow
//       (`do-research`/`form-vision` land in Inc-4, so a workflow-only fixture
//       pack supplies the create-gates; the `dogfood_record_schema.rs` /
//       `placement_acceptance.rs` precedent).
// ─────────────────────────────────────────────────────────────────────────────

/// The shipped `vision` schema's exact bytes from the methodology pack tree.
fn vision_schema_bytes() -> Vec<u8> {
    fs::read(methodology_pack_tree().join("schemas").join("vision.yaml"))
        .expect("read the shipped vision.yaml")
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn run_io(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
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

/// Assert a `jigc` invocation exited 0, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Both streams of an invocation, rendered for assertion messages.
fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Seed a workflow-only fixture pack: two `creates-task: true` host workflows
/// whose create-gates admit the (REAL, methodology-supplied) `research` and
/// `vision` doctypes — the `do-research`/`form-vision` stand-ins (Inc-4). NO
/// schema here: the schemas under test are the real ones. They reference no
/// `{{cli.X}}` command, so an empty catalog satisfies the read.
fn seed_fixture_pack(pack: &Path) {
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    for d in [&workflows, &steps, &config] {
        fs::create_dir_all(d).expect("mk fixture pack subdir");
    }
    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    fs::write(
        workflows.join("host-research.yaml"),
        "---\n\
         when: gather evidence for a vision\n\
         description: A host workflow that creates a research record.\n\
         usage: proving the real research doctype through the binary.\n\
         creates-task: true\n\
         allows-create: [{type: research, as: record}]\n\
         ---\n\
         {{ include: step:author }}\n",
    )
    .expect("seed host-research workflow");
    fs::write(
        workflows.join("host-vision.yaml"),
        "---\n\
         when: form the project vision from research\n\
         description: A host workflow that creates the vision singleton.\n\
         usage: proving the real vision doctype + grounded-in ref through the binary.\n\
         creates-task: true\n\
         allows-create: [{type: vision, as: vision}]\n\
         ---\n\
         {{ include: step:author }}\n",
    )
    .expect("seed host-vision workflow");
    fs::write(
        steps.join("author.yaml"),
        "Author the doc for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed author step");
}

/// Record BOTH the methodology pack AND the fixture workflow pack in the project
/// layer's `packs.yaml` — over the implicit embedded dev base: the
/// `[dev ▸ methodology ▸ fixture]` composition (the RC on-ramp plus the
/// workflow-only create-gate host).
fn write_packs_yaml_composed(repo: &Path, methodology: &Path, fixture: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!(
            "packs:\n  - {}\n  - {}\n",
            methodology.display(),
            fixture.display()
        ),
    )
    .expect("write packs.yaml naming both packs");
}

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    assert_ok(
        &run_io(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    assert_ok(
        &run_io(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Fill the provisioned commit doc's four levers (`type`/`scope`/`summary`/`body`)
/// so a finalize over the task validates clean — leaving the `grounded-in`
/// `ref-resolves` gate as the only lever under test. Filling all four is robust to
/// whichever `commit` doctype wins under the composition (the dev `commit`'s
/// `scope`/`body` are optional; the methodology `commit`'s are required — both
/// accept the value).
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), "vision");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"record it\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"A design-altitude record.\n",
    );
}

/// Start a host-workflow task, create + fully author a `research` record, and
/// finalize it — committing `research:cache-benchmarks` into the store so it is a
/// reachable `grounded-in` target for the vision task that follows. Returns the
/// committed research's `<type>:<slug>` identity.
fn commit_research(repo: &Path, home: &Path) -> String {
    assert_ok(
        &run_io(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "host-research",
                "benchmark the cache",
            ],
            None,
        ),
        "`jigc start --workflow host-research`",
    );
    let task = "benchmark-the-cache";
    let create = run_io(
        repo,
        home,
        &["doc", "create", "research", "--title", "Cache Benchmarks"],
        None,
    );
    assert_ok(&create, "`jigc doc create research`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(addr, "research:cache-benchmarks", "id-from: title slugs it");

    set_slot(
        repo,
        home,
        &format!("{addr}#question"),
        b"Is one node enough?\n",
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#findings"),
        b"A single node caps throughput under contention.\n",
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#sources"),
        b"The load-test transcript and the p99 latency graph.\n",
    );
    fill_commit(repo, home, task);

    assert_ok(
        &run_io(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (research task) — the committed grounding target",
    );
    addr
}

/// Start the vision task, create the `vision` singleton, author its three slots,
/// and set `grounded-in` to `targets` (an inline `[...]` ref list). Returns the
/// vision task id (the caller drives finalize + asserts the gate outcome).
fn author_vision_grounded_in(repo: &Path, home: &Path, targets: &str) -> &'static str {
    assert_ok(
        &run_io(
            repo,
            home,
            &["start", "--workflow", "host-vision", "form the vision"],
            None,
        ),
        "`jigc start --workflow host-vision`",
    );
    let task = "form-the-vision";
    let create = run_io(
        repo,
        home,
        &["doc", "create", "vision", "--title", "Vision"],
        None,
    );
    assert_ok(&create, "`jigc doc create vision`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(
        addr, "vision:vision",
        "the singleton mints at the fixed slug"
    );

    // The load-bearing anchor: the `grounded-in` `0..*` ref, set via the inline-list
    // form (the blessed multi-value path).
    set_field(repo, home, &format!("{addr}#meta/grounded-in"), targets);
    set_slot(
        repo,
        home,
        &format!("{addr}#thesis"),
        b"A context compiler for coding agents.\n",
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#invariants"),
        b"The CLI owns structure; the LLM owns prose.\n",
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#open-questions"),
        b"When does a public pack platform earn its keep?\n",
    );
    fill_commit(repo, home, task);
    task
}

/// (a) `vision` composes under `[dev ▸ methodology]` (narrated by the real
/// `describe`), and its shipped schema declares the singleton + `display-title` +
/// `placement` knobs, the three prose slots, and the `grounded-in → research`
/// ref. The knobs/slots/ref are read from the loaded model (the `describe`
/// projection carries only the woven prose); the `describe` run proves the schema
/// LOADED and COMPOSED under the union (a parse failure would `bail`).
#[test]
fn vision_composes_and_declares_its_knobs_slots_and_grounded_in_ref() {
    let repo = TempDir::new("vision-describe");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    // Composition proof: the real binary narrates `vision` under `[dev ▸ methodology]`.
    let out = run(repo.path(), home.path(), &["describe"]);
    assert!(
        out.status.success(),
        "`jigc describe` over `[dev ▸ methodology]` must exit 0 (a schema that failed to \
         parse would bail); stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains(
            "vision is The project's charter — its thesis, the invariants it holds, and its open questions — grounded in the research it was formed from."
        ),
        "describe must narrate the `vision` doctype's authored description; got:\n{stdout}",
    );
    assert!(
        stdout.contains(
            "Reach for it when the project's direction is worth a durable, managed anchor that later work compares against, formed from and traceable to the research behind it."
        ),
        "describe must narrate the `vision` doctype's authored usage; got:\n{stdout}",
    );

    // Structural proof: the shipped schema declares the knobs, slots, and ref.
    let schema = load_schema(&vision_schema_bytes()).expect("vision.yaml loads engine-native");
    assert_eq!(schema.ty, "vision");
    assert!(schema.singleton, "vision is a singleton");
    assert_eq!(
        schema.location, None,
        "a placement doctype sets no `location` — the home is the literal `placement.file`",
    );
    assert_eq!(
        schema.display_title.as_deref(),
        Some("Vision"),
        "the display-title knob makes the H1 read `# Vision`",
    );
    assert_eq!(
        schema.placement.as_ref().map(|p| p.file.as_str()),
        Some("VISION.md"),
        "the placement knob homes the managed vision directly at the repo-root VISION.md",
    );

    // The `meta` header carries the `grounded-in → research` ref (0..*, inverse grounds)
    // — the pack's first internal managed ref, modeled exactly like adr.supersedes.
    let meta = schema
        .sections
        .iter()
        .find(|s| s.id == "meta")
        .expect("vision carries a meta header section");
    assert!(meta.header, "meta is the front-matter header");
    let SectionBody::Simple { fields, .. } = &meta.body else {
        panic!("meta is a simple header section");
    };
    let grounded_in = fields
        .iter()
        .find(|f| f.id == "grounded-in")
        .expect("meta carries the grounded-in field");
    assert_eq!(grounded_in.ty, FieldType::Ref);
    assert_eq!(grounded_in.to.as_deref(), Some("research"));
    assert_eq!(grounded_in.card.as_deref(), Some("0..*"));
    assert_eq!(grounded_in.inverse.as_deref(), Some("grounds"));

    // The three prose slots, in document order after the header.
    for id in ["thesis", "invariants", "open-questions"] {
        let section = schema
            .sections
            .iter()
            .find(|s| s.id == id)
            .unwrap_or_else(|| panic!("vision carries a `{id}` section"));
        let SectionBody::Simple { slot, fields } = &section.body else {
            panic!("`{id}` is a simple slot section");
        };
        assert!(slot.is_some(), "`{id}` is a prose slot");
        assert!(fields.is_empty(), "`{id}` carries no fields");
    }
}

/// (b, pass) The `grounded-in` `ref-resolves` gate PASSES when the ref points at a
/// committed `research`: task 1 commits `research:cache-benchmarks`; task 2 grounds
/// the vision in it and finalizes clean, landing exactly one commit.
#[test]
fn grounded_in_ref_resolves_passes_against_a_committed_research() {
    let repo = TempDir::new("vision-pass");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let fixture = TempDir::new("fixture");
    seed_fixture_pack(fixture.path());
    write_packs_yaml_composed(repo.path(), &methodology_pack_tree(), fixture.path());

    // Task 1: commit the grounding research.
    let research = commit_research(repo.path(), home.path());

    // Task 2: ground the vision in the committed research, then finalize.
    let task = author_vision_grounded_in(repo.path(), home.path(), &format!("[{research}]"));
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = run_io(repo.path(), home.path(), &["task", "finalize", task], None);
    assert_ok(
        &out,
        "`jigc task finalize` (vision task) — grounded-in resolves against the committed research",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "a passing vision finalize lands exactly ONE commit"
    );
}

/// (b, block) The `grounded-in` gate BLOCKS PER-ELEMENT: with `grounded-in` set to
/// a list of one committed research AND one non-existent target, finalize blocks —
/// naming ONLY the dangling element (the committed one resolves), surfacing the
/// three routing options, and creating no commit. The `ref_resolves` walk emits one
/// finding per unreachable edge (`index.rs`), so the resolvable element passing while
/// the dangling one blocks is the per-element proof.
#[test]
fn grounded_in_ref_resolves_blocks_per_element_on_a_dangling_target() {
    let repo = TempDir::new("vision-block");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let fixture = TempDir::new("fixture");
    seed_fixture_pack(fixture.path());
    write_packs_yaml_composed(repo.path(), &methodology_pack_tree(), fixture.path());

    // Task 1: commit one real grounding research.
    let research = commit_research(repo.path(), home.path());

    // Task 2: ground the vision in [committed, dangling] — one resolves, one does not.
    let task = author_vision_grounded_in(
        repo.path(),
        home.path(),
        &format!("[{research}, research:nonexistent-typo]"),
    );
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = run_io(repo.path(), home.path(), &["task", "finalize", task], None);
    assert!(
        !out.status.success(),
        "a dangling grounded-in element must make finalize exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = streams(&out);
    // Per-element: the block names ONLY the dangling element; the committed one resolved.
    assert!(
        rendered.contains("research:nonexistent-typo"),
        "the block names the dangling grounded-in element; got:\n{rendered}",
    );
    assert!(
        !rendered.contains(&format!("{research} ")) && !rendered.contains(&format!("{research}\n")),
        "the committed research element resolves and is NOT flagged (per-element); got:\n{rendered}",
    );
    // The three routing options (fix the ref / create the target in this task / drop it).
    assert!(
        rendered.contains("fix")
            && rendered.contains("create the target in this task")
            && rendered.contains("drop"),
        "the block surfaces the three routing options; got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize creates no commit");
}

// ─────────────────────────────────────────────────────────────────────────────
// T3 — the methodology `commit` fix (GF5). Under methodology-ALONE
// (`JIGC_PACK_DIR=<methodology>`, NO `packs.yaml`, so the dev `commit` never
// shadows the methodology one), a `dev-task` finalize whose commit leaves the
// `scope` FIELD and the `body` SLOT EMPTY must land clean — the observable proof
// that BOTH carry `optional: true` (`design/design-altitude-doctypes.md` §5).
//
// This is a genuine red→green driven through the REAL shipped methodology
// `commit` schema: before the two flags, `scope`/`body` were author-required and
// finalize HARD-BLOCKED an empty scope/body (the M26 finalize-wall the dev pack
// already removed). Asserting the finalize LANDS (not a schema-flag read) makes
// the emitted finalize-gate outcome the contract — a regression re-adding the
// wall goes red here. The `dogfood_record_schema.rs` methodology-alone precedent.
// ─────────────────────────────────────────────────────────────────────────────

/// Run `jigc <args>` methodology-ALONE (`JIGC_PACK_DIR=<methodology>`, no
/// `packs.yaml` so the dev `commit` never shadows the methodology one),
/// optionally piping `stdin` (the `set-slot --from-file -` path).
fn run_alone(
    repo: &Path,
    home: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", methodology_pack_tree())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
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

/// GF5: the methodology `commit` finalizes with an EMPTY `scope` field + `body`
/// slot — both are `optional: true`. Mint a `dev-task`, stage one code change,
/// fill ONLY the required `type`/`summary` levers, and finalize: it lands exactly
/// one commit. Before the fix the missing `scope`/`body` block finalize.
#[test]
fn methodology_commit_finalizes_with_empty_scope_and_body() {
    let repo = TempDir::new("commit-optional");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Mint a task via the methodology default-workflow (`dev-task`, creates-task:
    // true — provisions the methodology `commit` as the finalize sink, no other doc).
    assert_ok(
        &run_alone(
            repo.path(),
            home.path(),
            &["start", "unblock the empty commit"],
            None,
        ),
        "`JIGC_PACK_DIR=<methodology> jigc start` (dev-task)",
    );
    let task = "unblock-the-empty-commit";

    // A real staged code change — finalize stages only the task's change-set and
    // blocks on an empty one, so the empty scope/body must be the ONLY lever left.
    fs::write(repo.path().join("change.txt"), "the task's one change\n")
        .expect("write code change");
    git(repo.path(), &["add", "change.txt"]);

    // Fill ONLY the required levers: `type` (enum field) + `summary` (slot).
    // Deliberately leave the `scope` field AND the `body` slot EMPTY — the two
    // levers GF5 makes optional.
    assert_ok(
        &run_alone(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#type"),
                "--value",
                "chore",
            ],
            None,
        ),
        "set commit type",
    );
    assert_ok(
        &run_alone(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#summary"),
                "--from-file",
                "-",
            ],
            Some(b"unblock the empty commit\n"),
        ),
        "set commit summary",
    );

    // With `scope` + `body` empty, finalize MUST land clean now (both optional).
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let fin = run_alone(repo.path(), home.path(), &["task", "finalize", task], None);
    assert_ok(
        &fin,
        "finalize with an EMPTY scope + body must land clean — GF5 makes both `optional: true`; \
         before the fix the methodology commit hard-blocks the empty scope/body (the M26 \
         finalize-wall)",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "the empty-scope/body finalize lands exactly one commit",
    );
}
