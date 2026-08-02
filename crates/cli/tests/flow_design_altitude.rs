//! M37 Increment 5 / T1 — the **consolidated design-altitude acceptance suite**: the
//! six §7 arms of `design/design-altitude-doctypes.md`, driven end-to-end through the
//! **real `jigc` binary** over the **`[dev ▸ methodology]`** composition (the RC-trial's
//! actual on-ramp — the on-disk methodology pack listed in `packs.yaml` OVER the embedded
//! dev base, the `flow_form_vision.rs` harness shape, NOT a methodology-alone
//! `JIGC_PACK_DIR`). This is the M37 end-to-end acceptance marquee (the flow19/flow20
//! pattern: the arms are assertions across one file), proving the whole done-picture —
//! research → vision (the compare-against anchor) → ideas parked — at the composed surface.
//!
//! The six arms (`design/design-altitude-doctypes.md` §7):
//!   1. `do-research` (×2) → two committed `research/<slug>.md` with an on-create `date`.
//!   2. `form-vision` → create `vision` → ground it in BOTH research (the multi-valued
//!      anchor) → **re-compose** → author → finalize. The LOAD-BEARING red step: the
//!      edge-walk slice `{{@task.vision.grounded-in#findings}}` is EMPTY before re-compose
//!      and CONTAINS the FIRST research's findings prose after `jigc start --task <id>`
//!      (closing the vacuous-green gap); both `grounded-in` targets resolve at finalize;
//!      the vision is managed directly at the repo-root literal `VISION.md` (placement knob)
//!      with H1 `# Vision` (display-title knob); `describe` lists all three doctypes + all
//!      three workflows.
//!   3. `park-idea` → one committed `ideas/<slug>.md`; `park-idea` on the router surface.
//!   4. A dangling `grounded-in` element **blocks** finalize PER-ELEMENT (`ref-resolves`),
//!      no commit.
//!   5. The pack-load **freeze** gate stays green — the additive `display-title:` /
//!      `placement:` keys (serialize-skipped when absent) perturb no frozen dev-pack
//!      doctype hash.
//!   6. A pre-existing FOREIGN root `VISION.md` (no managed vision yet) **blocks** the first
//!      `form-vision` finalize via the inherited `finalize.promote-clobber` (no silent data
//!      loss), the file untouched, no commit; once removed, finalize proceeds and the managed
//!      vision owns it.
//!
//! Everything is asserted on the EMITTED bytes of the real binary (`CARGO_BIN_EXE_jigc`)
//! over the `[dev ▸ methodology]` composition. The methodology pack ships no `docs-root`
//! knob, so its multi-instance doctypes land flat at their `location:` — research at
//! `research/`, ideas at `ideas/` — while the `vision` singleton declares
//! `placement: { file: VISION.md }` and is managed directly at the repo-root literal
//! `VISION.md`. No external test crates.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-design-altitude-acceptance-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the literal directory a
/// `.jigc/config/packs.yaml` entry names.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer, then
/// record the methodology pack in `packs.yaml` (listed over the embedded dev base: the
/// `[dev ▸ methodology]` composition — the flow_form_vision harness shape).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
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

/// Both streams of an invocation, rendered for assertion messages.
fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Fill the provisioned commit doc's four levers so a finalize validates clean — robust to
/// whichever `commit` doctype wins under the composition (the dev `commit`'s `scope`/`body`
/// are optional; both accept the value).
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

/// The committed bytes of `path` at HEAD.
fn committed(repo: &Path, path: &str) -> String {
    let out = Command::new("git")
        .args(["show", &format!("HEAD:{path}")])
        .current_dir(repo)
        .output()
        .expect("git show");
    assert!(
        out.status.success(),
        "{path} must be committed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 committed bytes")
}

/// The `HEAD` commit count.
fn head_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap()
}

/// Arm 1: commit one grounding `research` doc through the REAL `do-research` workflow, so it
/// is a reachable `grounded-in` target for the vision task. `intent` derives the task id;
/// `title` mints the `research:<slug>`. Returns the committed `<type>:<slug>`.
fn commit_research(repo: &Path, home: &Path, intent: &str, title: &str, findings: &[u8]) -> String {
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "do-research", intent],
            None,
        ),
        "`jigc start --workflow do-research`",
    );
    let create = jigc(
        repo,
        home,
        &["doc", "create", "research", "--title", title],
        None,
    );
    assert_ok(&create, "`jigc doc create research`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    set_slot(repo, home, &format!("{addr}#question"), b"A question.\n");
    set_slot(repo, home, &format!("{addr}#findings"), findings);
    set_slot(repo, home, &format!("{addr}#sources"), b"Some sources.\n");
    let task = intent.replace(' ', "-");
    fill_commit(repo, home, &task);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task], None),
        "`jigc task finalize` (do-research) — the committed grounding target",
    );
    // Arm 1 done-criterion: the promoted research carries an on-create ISO `date`.
    let doc = committed(
        repo,
        &format!("research/{}.md", addr.trim_start_matches("research:")),
    );
    let date = doc
        .lines()
        .find_map(|l| l.trim().strip_prefix("date:"))
        .map(str::trim)
        .unwrap_or_else(|| {
            panic!("the promoted research carries an on-create `date`; got:\n{doc}")
        });
    assert!(
        date.len() == 10 && date.as_bytes()[4] == b'-' && date.as_bytes()[7] == b'-',
        "the on-create date is an ISO `YYYY-MM-DD`; got {date:?}",
    );
    addr
}

/// **Arms 1 + 2 + 3 — the integrated done-picture** (research → vision → park-idea) at the
/// `[dev ▸ methodology]` composition (the flow-38-aligned marquee). Two committed research
/// docs ground the vision (the multi-valued anchor); `form-vision` is the Shape-2 re-entry
/// flow whose edge-walk slice is EMPTY before re-compose and READS the FIRST research's
/// findings after; finalize resolves BOTH grounded-in targets, promotes `# Vision`, and
/// renders `VISION.md` byte-faithful; a parked `idea` lands its own commit; and `describe`
/// + the router surface enumerate all three doctypes and all three workflows.
#[test]
fn done_picture_research_to_vision_to_park_idea() {
    let repo = TempDir::new("marquee");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── Arm 1: two committed grounding research docs, with DISTINCT findings prose. ──
    const FIRST_FINDINGS: &str = "A single node caps throughput under contention.";
    const SECOND_FINDINGS: &str = "Sharding removes the write-contention ceiling.";
    let research_a = commit_research(
        repo.path(),
        home.path(),
        "benchmark the cache",
        "Cache Benchmarks",
        format!("{FIRST_FINDINGS}\n").as_bytes(),
    );
    let research_b = commit_research(
        repo.path(),
        home.path(),
        "measure sharded writes",
        "Sharded Writes",
        format!("{SECOND_FINDINGS}\n").as_bytes(),
    );
    assert_eq!(research_a, "research:cache-benchmarks");
    assert_eq!(research_b, "research:sharded-writes");

    // ── Arm 2: form-vision. First compose — the edge-walk slice resolves EMPTY (the vision
    // does not exist yet, `grounded-in` unset). The create-vision command-ref must resolve
    // to a literal `jigc doc create vision` line (never the unresolved placeholder). ──
    let start = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "form-vision",
            "form the project vision",
        ],
        None,
    );
    assert_ok(
        &start,
        "`jigc start --workflow form-vision` must compose (create-gate granted, \
         create-vision resolves, the empty slice resolves cleanly)",
    );
    let first_compose = String::from_utf8(start.stdout).expect("utf-8 composed stdout");
    assert!(
        first_compose.contains("jigc doc create vision"),
        "the composed workflow must carry the RESOLVED create-vision command-ref; got:\n{first_compose}",
    );
    assert!(
        !first_compose.contains("cli.create-vision"),
        "the create-vision placeholder must be RESOLVED, not emitted literally; got:\n{first_compose}",
    );
    assert!(
        !first_compose.contains(FIRST_FINDINGS),
        "LOAD-BEARING (red before re-compose): before `grounded-in` is set + re-composed, the \
         edge-walk slice must resolve EMPTY — the grounding findings must NOT appear; got:\n{first_compose}",
    );

    // Create the vision singleton, ground it in BOTH research (the multi-valued anchor,
    // inline-list form), then RE-COMPOSE.
    let task = "form-the-project-vision";
    let create = jigc(
        repo.path(),
        home.path(),
        &[
            "doc", "create", "vision", "--title", "Vision", "--task", task,
        ],
        None,
    );
    assert_ok(
        &create,
        "`jigc doc create vision` — the create-gate grants vision",
    );
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(
        addr, "vision:vision",
        "the singleton mints at the fixed slug"
    );

    set_field(
        repo.path(),
        home.path(),
        &format!("{addr}#meta/grounded-in"),
        &format!("[{research_a}, {research_b}]"),
    );

    // RE-COMPOSE: the edge-walk slice now READS ALL grounding research's findings
    // (the green side of the red step — the slice actually read the committed research).
    let resume = jigc(repo.path(), home.path(), &["start", "--task", task], None);
    assert_ok(&resume, "`jigc start --task <id>` re-compose (form-vision)");
    let recomposed = String::from_utf8(resume.stdout).expect("utf-8 recomposed stdout");
    assert!(
        recomposed.contains(FIRST_FINDINGS),
        "LOAD-BEARING (closing the vacuous-green gap): after `grounded-in` is set + re-composed, \
         the edge-walk slice must READ the FIRST research's findings into the guidance; got:\n{recomposed}",
    );
    assert!(
        recomposed.contains(SECOND_FINDINGS),
        "walk_edge fans out to ALL grounded targets — the SECOND research's findings must ALSO appear \
         (the all-source content-echo, §3 constraint 2); got:\n{recomposed}",
    );

    // Author the three prose slots, fill the commit, and finalize.
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#thesis"),
        b"A context compiler for coding agents.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#invariants"),
        b"The CLI owns structure; the LLM owns prose.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#open-questions"),
        b"When does a public pack platform earn its keep?\n",
    );
    fill_commit(repo.path(), home.path(), task);

    let before = head_count(repo.path());
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`jigc task finalize` (form-vision) — BOTH grounded-in targets resolve, no block",
    );
    assert_eq!(
        head_count(repo.path()),
        before + 1,
        "form-vision finalize lands exactly ONE commit"
    );

    // The vision is MANAGED DIRECTLY at the repo-root literal `VISION.md` (placement knob):
    // H1 reads `# Vision` (display-title knob), carries the thesis, and records BOTH
    // grounded-in targets (the multi-valued anchor).
    let managed = committed(repo.path(), "VISION.md");
    assert!(
        managed.lines().any(|l| l.trim() == "# Vision"),
        "the managed vision doc's H1 reads `# Vision` (display-title knob); got:\n{managed}",
    );
    assert!(
        managed.contains("A context compiler for coding agents."),
        "the promoted vision carries the authored thesis; got:\n{managed}",
    );
    assert!(
        managed.contains(&research_a) && managed.contains(&research_b),
        "the committed vision records BOTH grounded-in targets; got:\n{managed}",
    );
    // No one-file-folder mirror: the vision lives ONLY at the literal `VISION.md`.
    let show = Command::new("git")
        .args(["show", "HEAD:vision/vision.md"])
        .current_dir(repo.path())
        .output()
        .expect("git show");
    assert!(
        !show.status.success(),
        "the vision must NOT be promoted to `vision/vision.md` — placement homes it at \
         the literal `VISION.md` alone",
    );

    // ── Arm 3: park-idea → one committed `ideas/<slug>.md`. ──
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "park-idea",
                "park a shaped direction",
            ],
            None,
        ),
        "`jigc start --workflow park-idea`",
    );
    let idea_task = "park-a-shaped-direction";
    let create_idea = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "idea",
            "--title",
            "Warm The Cache On Boot",
            "--task",
            idea_task,
        ],
        None,
    );
    assert_ok(
        &create_idea,
        "`jigc doc create idea` — the create-gate grants idea",
    );
    let idea_addr = String::from_utf8(create_idea.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(idea_addr, "idea:warm-the-cache-on-boot");
    set_field(
        repo.path(),
        home.path(),
        &format!("{idea_addr}#trigger"),
        "cold-start latency becomes a complaint",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{idea_addr}#description"),
        b"Prefill the hot keys during boot so the first request is warm.\n",
    );
    fill_commit(repo.path(), home.path(), idea_task);
    let before_idea = head_count(repo.path());
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", idea_task],
            None,
        ),
        "`jigc task finalize` (park-idea) — promotes the idea record",
    );
    assert_eq!(
        head_count(repo.path()),
        before_idea + 1,
        "park-idea finalize lands exactly ONE commit"
    );
    let idea_doc = committed(repo.path(), "ideas/warm-the-cache-on-boot.md");
    assert!(
        idea_doc.contains("cold-start latency becomes a complaint")
            && idea_doc.contains("Prefill the hot keys during boot so the first request is warm."),
        "the promoted idea carries the authored trigger + description; got:\n{idea_doc}",
    );

    // ── Arm 2/3 surface: `describe` narrates all THREE doctypes + all THREE workflows. ──
    let describe = jigc(repo.path(), home.path(), &["describe"], None);
    assert_ok(&describe, "`jigc describe` over `[dev ▸ methodology]`");
    let desc = String::from_utf8(describe.stdout).expect("utf-8 stdout");
    for phrase in [
        "research is",
        "idea is",
        "vision is",
        "do-research is",
        "form-vision is",
        "park-idea is",
    ] {
        assert!(
            desc.contains(phrase),
            "describe must narrate `{phrase} …` (all three doctypes + three workflows); got:\n{desc}",
        );
    }

    // Router surface: bare `jigc start --format json` lists all three selectable workflows.
    let json = jigc(
        repo.path(),
        home.path(),
        &["start", "--format", "json"],
        None,
    );
    assert_ok(&json, "bare `jigc start --format json` orientation");
    let json_out = String::from_utf8(json.stdout).expect("utf-8 stdout");
    let value: serde_json::Value = serde_json::from_str(&json_out)
        .unwrap_or_else(|e| panic!("orientation must be valid JSON ({e}); got:\n{json_out}"));
    let ids: Vec<&str> = value["workflows"]
        .as_array()
        .unwrap_or_else(|| panic!("`workflows` must be an array; got:\n{json_out}"))
        .iter()
        .map(|w| w["id"].as_str().unwrap_or_default())
        .collect();
    for id in ["do-research", "form-vision", "park-idea"] {
        assert!(
            ids.contains(&id),
            "the selectable router catalog must name `{id}` (selectable: true); got: {ids:?}",
        );
    }
}

/// **Arm 4 — the dangling `grounded-in` element blocks finalize PER-ELEMENT.** With
/// `grounded-in` set to one committed research AND one non-existent target, `form-vision`
/// finalize blocks — naming ONLY the dangling element (the committed one resolves), and
/// creating no commit. The `ref_resolves` walk emits one finding per unreachable edge
/// (`index.rs`), so the resolvable element passing while the dangling one blocks is the
/// per-element proof.
#[test]
fn dangling_grounded_in_blocks_finalize_per_element() {
    let repo = TempDir::new("dangle");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // One real committed grounding research.
    let research = commit_research(
        repo.path(),
        home.path(),
        "benchmark the cache",
        "Cache Benchmarks",
        b"A single node caps throughput under contention.\n",
    );

    // form-vision grounded in [committed, dangling] — one resolves, one does not.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "form-vision",
                "form the project vision",
            ],
            None,
        ),
        "`jigc start --workflow form-vision`",
    );
    let task = "form-the-project-vision";
    let create = jigc(
        repo.path(),
        home.path(),
        &[
            "doc", "create", "vision", "--title", "Vision", "--task", task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create vision`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    set_field(
        repo.path(),
        home.path(),
        &format!("{addr}#meta/grounded-in"),
        &format!("[{research}, research:nonexistent-typo]"),
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#thesis"),
        b"A context compiler for coding agents.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#invariants"),
        b"The CLI owns structure; the LLM owns prose.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#open-questions"),
        b"When does a public pack platform earn its keep?\n",
    );
    fill_commit(repo.path(), home.path(), task);

    let before = head_count(repo.path());
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task], None);
    assert!(
        !out.status.success(),
        "a dangling grounded-in element must make finalize exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = streams(&out);
    assert!(
        rendered.contains("research:nonexistent-typo"),
        "the block names the dangling grounded-in element; got:\n{rendered}",
    );
    assert!(
        !rendered.contains(&format!("{research} ")) && !rendered.contains(&format!("{research}\n")),
        "the committed research element resolves and is NOT flagged (per-element); got:\n{rendered}",
    );
    assert_eq!(
        head_count(repo.path()),
        before,
        "a blocked finalize creates no commit"
    );
}

/// **Arm 5 — the pack-load freeze gate stays green under the new schema knobs.** The
/// shipped `vision` schema declares the additive `display-title:` / `placement:` keys;
/// under `[dev ▸ methodology]` a `jigc start` compose runs `assert_schema_freeze` against
/// the dev pack's `schema-manifest.yaml`. If the additive `Option` fields serialized when
/// absent, every frozen dev-pack doctype's `schema-hash` would move and the gate would fire
/// (`schema-hash mismatch`). A clean compose is the proof the keys serialize-skip and
/// perturb no frozen hash.
#[test]
fn freeze_stays_green_under_the_new_schema_knobs() {
    let repo = TempDir::new("freeze");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // The additive knobs are genuinely present in the composed pack (so the freeze check
    // below is over a universe that actually uses them).
    let vision_schema =
        fs::read_to_string(methodology_pack_tree().join("schemas").join("vision.yaml"))
            .expect("read the shipped vision.yaml");
    assert!(
        vision_schema.contains("display-title:") && vision_schema.contains("placement:"),
        "the shipped vision schema declares the additive display-title/placement keys; got:\n{vision_schema}",
    );

    // A real compose over `[dev ▸ methodology]` fires the pack-load freeze gate against the
    // dev pack's frozen doctype set. It must stay green.
    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "do-research", "freeze probe"],
        None,
    );
    assert_ok(
        &out,
        "a compose under `[dev ▸ methodology]` must pass the pack-load freeze gate",
    );
    let rendered = streams(&out);
    assert!(
        !rendered.contains("schema-hash mismatch"),
        "the additive display-title/placement keys must perturb NO frozen dev-pack doctype hash \
         (no schema-hash mismatch); got:\n{rendered}",
    );
}

/// **Arm 6 — a pre-existing FOREIGN root `VISION.md` blocks the first form-vision finalize,
/// then proceeds once removed** (the no-silent-data-loss invariant over the REAL `vision`
/// doctype managed directly at root `VISION.md` — the existing-project RC on-ramp). The
/// vision is minted Created while `VISION.md` is absent; a human then hand-authors a foreign
/// `VISION.md` at the managed literal path before finalize. Promoting the Created doc would
/// overwrite it, so the **inherited `plan_clobber_guard`** blocks with `finalize.promote-clobber`
/// (zero bespoke root-render guard — `design/storage.md` → Placement: the no-silent-data-loss
/// guard is inherited, not rebuilt). The foreign file is left untouched and no commit lands;
/// after it is removed, finalize proceeds and the managed vision owns `VISION.md`.
#[test]
fn preexisting_foreign_root_vision_blocks_then_proceeds() {
    let repo = TempDir::new("foreign-root");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // form-vision, no research: `grounded-in` is `0..*`, so an unset anchor is valid — the
    // foreign-file clobber guard is the only lever under test.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "form-vision",
                "form the project vision",
            ],
            None,
        ),
        "`jigc start --workflow form-vision`",
    );
    let task = "form-the-project-vision";
    // Mint the vision while the literal `VISION.md` is EMPTY → `Provenance::Created`.
    let create = jigc(
        repo.path(),
        home.path(),
        &[
            "doc", "create", "vision", "--title", "Vision", "--task", task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create vision`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#thesis"),
        b"A context compiler for coding agents.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#invariants"),
        b"The CLI owns structure; the LLM owns prose.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#open-questions"),
        b"When does a public pack platform earn its keep?\n",
    );
    fill_commit(repo.path(), home.path(), task);

    // A hand-authored root VISION.md that jigc did NOT generate (the existing-project on-ramp),
    // dropped at the managed literal path AFTER the Created mint.
    const FOREIGN: &[u8] = b"# Vision\n\nhand-authored by a human, not jigc.\n";
    fs::write(repo.path().join("VISION.md"), FOREIGN).expect("write the foreign root file");

    let before = head_count(repo.path());
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--format", "json"],
        None,
    );
    assert!(
        !out.status.success(),
        "a foreign VISION.md at the managed literal path must block finalize; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the report envelope must parse ({e}); got:\n{stdout}"));
    let findings = value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"));
    let clobber = findings
        .iter()
        .find(|f| {
            f["code"] == "finalize.promote-clobber"
                && f["message"]
                    .as_str()
                    .is_some_and(|m| m.contains("VISION.md"))
        })
        .unwrap_or_else(|| {
            panic!("the foreign VISION.md must block at the inherited finalize.promote-clobber; got:\n{findings:#?}")
        });
    assert_eq!(clobber["severity"], "blocking");

    // No silent data loss: the foreign file is untouched, and no commit was created.
    let after_bytes =
        fs::read(repo.path().join("VISION.md")).expect("read VISION.md after the block");
    assert_eq!(
        after_bytes, FOREIGN,
        "a blocked finalize must not overwrite the foreign file"
    );
    assert_eq!(
        head_count(repo.path()),
        before,
        "a clobber block must create no commit"
    );

    // Recovery: remove the foreign file, re-run — the managed vision now owns `VISION.md`.
    fs::remove_file(repo.path().join("VISION.md")).expect("remove the foreign root file");
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`jigc task finalize` after removing the foreign root file",
    );
    let managed = committed(repo.path(), "VISION.md");
    assert!(
        managed.contains("A context compiler for coding agents."),
        "after recovery the managed vision owns `VISION.md`, carrying the authored thesis; \
         got:\n{managed}",
    );
}
