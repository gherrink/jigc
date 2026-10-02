//! M37 Increment 4 / T3 — the **`form-vision`** driving workflow (Shape-2 re-entry),
//! proven end-to-end over the real `jigc` binary under the **`[dev ▸ methodology]`**
//! composition (`design/design-altitude-doctypes.md` §3 + §4 + §7 arm 2;
//! `implementation/roadmap.md` → M37 Increment 4).
//!
//! `form-vision` is the trio's re-entry flow: it creates/updates the `vision`
//! singleton, sets `grounded-in` to the *committed* research it rests on, and authors
//! thesis/invariants/open-questions. Its author step reads the FIRST grounding
//! research's `findings` via the proven edge-walk slice
//! `{{ @task.vision.grounded-in#findings }}` — the superseding-context shape, but
//! walking `vision —grounded-in→ research` instead of `adr —supersedes→ adr`.
//!
//! The LOAD-BEARING fact (§7 arm 2, closing the vacuous-green gap): the slice resolves
//! at COMPOSE time, so it is EMPTY at the first `jigc start` (the vision does not exist,
//! `grounded-in` is unset) and NON-EMPTY only after `grounded-in` is set and the task is
//! RE-COMPOSED (`jigc start --task <id>`). This test asserts BOTH — the empty-before /
//! non-empty-after contrast is the proof the edge-walk slice actually READ the committed
//! research, not merely resolved. `grounded-in` is a `0..*` anchor, so the vision is
//! grounded in TWO research docs (the multi-valued anchor), and both resolve at finalize.
//!
//! Everything is asserted on the EMITTED bytes of the real binary (`CARGO_BIN_EXE_jigc`)
//! over the `[dev ▸ methodology]` composition (methodology pack listed in `packs.yaml`
//! over the embedded dev base — the RC-trial on-ramp). The `vision` doctype declares
//! `placement: { file: VISION.md }` (no `location`, docs-root never applies), so the
//! managed vision is homed DIRECTLY at the repo-root literal `VISION.md` — one file,
//! OOB-reconciled, not a `docs/vision/vision.md` source mirrored to root. No external
//! test crates.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-form-vision-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer,
/// then record the methodology pack in `packs.yaml` (listed over the embedded dev base:
/// the `[dev ▸ methodology]` composition).
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

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
///
/// Every write names its `--task` explicitly. That is redundant while one task is open
/// and **load-bearing** once two are (the mandated-ordering arm below keeps the vision
/// task open across a whole `do-research` task): several open tasks are legal, each
/// addressed by its own `--task` — the compose footer's own rule.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8], task: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
            Some(prose),
        ),
        &format!("set-slot {addr} --task {task}"),
    );
}

/// Set one header field through the binary, asserting success. `--task` explicit, for
/// the reason [`set_slot`] states.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str, task: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value, "--task", task],
            None,
        ),
        &format!("set-field {addr} --task {task}"),
    );
}

/// Fill the provisioned commit doc's four levers so a finalize validates clean — robust
/// to whichever `commit` doctype wins under the composition (the dev `commit`'s
/// `scope`/`body` are optional; both accept the value).
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs", task);
    set_field(repo, home, &format!("commit:{task}#scope"), "vision", task);
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"record it\n",
        task,
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"A design-altitude record.\n",
        task,
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

/// Commit one grounding `research` doc through the REAL `do-research` workflow (§7 arm 1),
/// so it is a reachable `grounded-in` target for the vision task. `intent` derives the
/// task id; `title` mints the `research:<slug>`. Returns the committed `<type>:<slug>`.
fn commit_research(repo: &Path, home: &Path, intent: &str, title: &str, findings: &[u8]) -> String {
    let task = intent.replace(' ', "-");
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
        &[
            "doc", "create", "research", "--title", title, "--task", &task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create research`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    set_slot(
        repo,
        home,
        &format!("{addr}#question"),
        b"A question.\n",
        &task,
    );
    set_slot(repo, home, &format!("{addr}#findings"), findings, &task);
    set_slot(
        repo,
        home,
        &format!("{addr}#sources"),
        b"Some sources.\n",
        &task,
    );
    fill_commit(repo, home, &task);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task], None),
        "`jigc task finalize` (do-research) — the committed grounding target",
    );
    addr
}

/// (1)+(2): the compose + re-entry + author + finalize spine. Commits two research docs,
/// then `form-vision` grounds the vision in BOTH (the multi-valued anchor). The
/// LOAD-BEARING contrast: the first grounding research's `findings` are ABSENT from the
/// first compose (empty slice) and PRESENT only after `grounded-in` is set and the task is
/// re-composed (the edge-walk slice actually read the committed research). BOTH research
/// docs' findings appear (walk_edge fans out to ALL grounded targets — the all-source
/// content-echo, §3 constraint 2). Finalize resolves BOTH grounded-in targets (no block),
/// lands exactly one commit, the vision is managed directly at the repo-root literal
/// `VISION.md` (placement knob) with H1 `# Vision` (display-title knob), and an OOB edit to
/// it is detected + routed (managed, not a mirror).
#[test]
fn form_vision_reentry_reads_grounding_research_and_finalizes() {
    let repo = TempDir::new("spine");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Two committed grounding research docs, with DISTINCT findings prose.
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

    // (1) Compose form-vision. The create-vision command-ref must be RESOLVED into a
    // literal `jigc doc create vision` line (never the unresolved placeholder, hardening
    // #4) — AND, load-bearing, the first research's findings must be ABSENT (the slice
    // resolves empty: the vision does not exist yet, `grounded-in` is unset).
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
        "the composed workflow must carry the RESOLVED create-vision command-ref \
         (a `jigc doc create vision …` line); got:\n{first_compose}",
    );
    assert!(
        !first_compose.contains("cli.create-vision"),
        "the create-vision placeholder must be RESOLVED, not emitted literally; got:\n{first_compose}",
    );
    assert!(
        !first_compose.contains(FIRST_FINDINGS),
        "LOAD-BEARING: before `grounded-in` is set + re-composed, the edge-walk slice must \
         resolve EMPTY — the grounding research's findings must NOT appear; got:\n{first_compose}",
    );

    // (2) Create the vision singleton, ground it in BOTH research (the multi-valued anchor,
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
        task,
    );

    // RE-COMPOSE: the edge-walk slice now reads ALL grounding research's findings.
    let resume = jigc(repo.path(), home.path(), &["start", "--task", task], None);
    assert_ok(&resume, "`jigc start --task <id>` re-compose (form-vision)");
    let recomposed = String::from_utf8(resume.stdout).expect("utf-8 recomposed stdout");
    assert!(
        recomposed.contains(FIRST_FINDINGS),
        "LOAD-BEARING (closing the vacuous-green gap): after `grounded-in` is set + \
         re-composed, the edge-walk slice must READ the FIRST grounding research's findings \
         into the guidance; got:\n{recomposed}",
    );
    assert!(
        recomposed.contains(SECOND_FINDINGS),
        "walk_edge fans out to ALL grounded targets — the SECOND research's findings must \
         ALSO appear (the all-source content-echo, §3 constraint 2); got:\n{recomposed}",
    );

    // (3) Author the three prose slots, fill the commit, and finalize.
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#thesis"),
        b"A context compiler for coding agents.\n",
        task,
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#invariants"),
        b"The CLI owns structure; the LLM owns prose.\n",
        task,
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#open-questions"),
        b"When does a public pack platform earn its keep?\n",
        task,
    );
    fill_commit(repo.path(), home.path(), task);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`jigc task finalize` (form-vision) — BOTH grounded-in targets resolve, no block",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "form-vision finalize lands exactly ONE commit"
    );

    // The vision is MANAGED DIRECTLY at the repo-root literal `VISION.md` (the placement
    // knob: `placement: { file: VISION.md }`, no `location`, docs-root never applies) — one
    // file, not a `docs/vision/vision.md` source mirrored to root. Its H1 reads `# Vision`
    // (the display-title knob), NOT `# vision`.
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
        "the committed vision records BOTH grounded-in targets (the multi-valued anchor); \
         got:\n{managed}",
    );

    // No one-file-folder mirror: the vision lives ONLY at the literal `VISION.md`, never at
    // a `vision/vision.md` / `docs/vision/vision.md` `<location>/<slug>.md` home.
    for stale in ["vision/vision.md", "docs/vision/vision.md"] {
        let show = Command::new("git")
            .args(["show", &format!("HEAD:{stale}")])
            .current_dir(repo.path())
            .output()
            .expect("git show");
        assert!(
            !show.status.success(),
            "the vision must NOT be promoted to `{stale}` — placement homes it at the literal \
             `VISION.md` alone",
        );
    }

    // VISION.md IS the managed doc (not a byte-mirror of a docs/-buried source): an
    // out-of-band human edit to it is DETECTED + ROUTED by the reconcile/validate sweep —
    // the "managed, not a mirror" promise. Drift it nonconformantly (rename a required
    // section heading) through git, then a later task's store-scope validate blocks on it.
    let drifted = managed.replace("## Thesis", "## Thesisz");
    assert_ne!(
        drifted, managed,
        "the OOB edit must actually change the heading"
    );
    fs::write(repo.path().join("VISION.md"), &drifted).expect("write the OOB-edited VISION.md");
    git(repo.path(), &["add", "VISION.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "human edits VISION.md out of band"],
    );

    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "form-vision", "reconcile the store"],
            None,
        ),
        "`jigc start` (reconcile pass)",
    );
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "task",
            "validate",
            "reconcile-the-store",
            "--format",
            "json",
        ],
        None,
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 validate stdout");
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the report envelope must parse ({e}); got:\n{stdout}"));
    let findings = value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"));
    let block = findings
        .iter()
        .find(|f| {
            f["code"] == "reconciliation.conformance-block"
                && f["message"]
                    .as_str()
                    .is_some_and(|m| m.contains("VISION.md"))
        })
        .unwrap_or_else(|| {
            panic!(
                "the OOB edit to the managed VISION.md must be detected + routed \
                 (managed, not a mirror); got:\n{findings:#?}"
            )
        });
    assert_eq!(
        block["severity"], "blocking",
        "a recorded-then-drifted managed VISION.md conformance-blocks: {block:#?}",
    );
    assert!(
        block["route"].is_string(),
        "the conformance-block carries a route to a human: {block:#?}",
    );
}

/// (5) M47 Increment 8 / T2 (N7): the step's **own mandated ordering**, driven end to end.
///
/// `author-vision.yaml` opens by advising `do-research` *first* and later mandates a
/// literal `jigc start --task {{task.id}}` re-compose. An agent that follows both — the
/// only ordering the step describes — mints `form-vision` over an empty store, goes and
/// **commits** the grounding research in a separate task, then comes back through the
/// resume door. That research commit moves HEAD off the vision task's pin, so before the
/// resume door adopted the finalize guard's overlap-aware test (M47 Inc 8 / T1) this exact
/// sequence dead-ended at the re-compose: the workflow's own step mandated a door the
/// binary refused. The work is disjoint (`research/` vs the vision's `VISION.md`), so the
/// task now re-pins and is carried through — and the advisory that sent the agent away
/// **says so**, rather than leaving the return trip to be discovered.
///
/// The shipped spine arm above commits both research docs *before* minting `form-vision`,
/// which is exactly why the defect never showed there — this arm inverts that order.
#[test]
fn form_vision_carries_through_research_committed_in_a_separate_task() {
    let repo = TempDir::new("mandated-order");
    let home = TempDir::new("home");
    init_repo(repo.path());

    const FINDINGS: &str = "One writer node caps the ingest path.";
    let vision_task = "form-the-project-vision";

    // (1) `form-vision` FIRST, over an EMPTY research store: the advisory renders — and
    // it states the ordering it is sending the agent into, namely that the grounding
    // research may be committed in a SEPARATE task and this one re-composed afterwards.
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
        "`jigc start --workflow form-vision` over an empty research store",
    );
    let first_compose = String::from_utf8(start.stdout).expect("utf-8 composed stdout");
    let advisory = first_compose
        .split("Form the project vision on the")
        .next()
        .expect("the composed text opens with the advisory region")
        .to_string();
    assert!(
        advisory.contains("consider running `do-research`"),
        "the empty-research advisory must render over an empty store; got:\n{first_compose}",
    );
    assert!(
        advisory.contains("separate task"),
        "the advisory must state that the grounding research may be committed in a \
         SEPARATE task — the ordering it is sending the agent into; got:\n{advisory}",
    );
    assert!(
        advisory.contains(&format!("jigc start --task {vision_task}")),
        "the advisory must name the return trip it implies — the literal re-compose that \
         carries this task forward after the research lands; got:\n{advisory}",
    );

    // (2) Follow the advisory: the grounding research is authored and FINALIZED in its own
    // task while the vision task stays open. HEAD moves off the vision task's pin.
    let pin_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let research = commit_research(
        repo.path(),
        home.path(),
        "benchmark the ingest path",
        "Ingest Benchmarks",
        format!("{FINDINGS}\n").as_bytes(),
    );
    assert_eq!(research, "research:ingest-benchmarks");
    assert_ne!(
        pin_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "the research task's finalize must move HEAD off the vision task's pinned base — \
         without that, this arm proves nothing",
    );

    // (3) The step's LITERAL re-compose door, over moved-but-disjoint history.
    let resume = jigc(
        repo.path(),
        home.path(),
        &["start", "--task", vision_task],
        None,
    );
    assert_ok(
        &resume,
        "the step's own `jigc start --task <id>` must carry the agent through after the \
         grounding research was committed in a separate task",
    );
    let resumed = String::from_utf8(resume.stdout).expect("utf-8 resumed stdout");
    assert!(
        !resumed.contains("consider running `do-research`"),
        "with research committed the advisory falls silent; got:\n{resumed}",
    );

    // (4) Now follow the step body: create, ground, re-compose, author, finalize.
    let create = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "vision",
            "--title",
            "Vision",
            "--task",
            vision_task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create vision` after the resume");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(addr, "vision:vision");

    set_field(
        repo.path(),
        home.path(),
        &format!("{addr}#meta/grounded-in"),
        &format!("[{research}]"),
        vision_task,
    );

    let second = jigc(
        repo.path(),
        home.path(),
        &["start", "--task", vision_task],
        None,
    );
    assert_ok(
        &second,
        "the step's second `jigc start --task <id>` re-compose",
    );
    let recomposed = String::from_utf8(second.stdout).expect("utf-8 recomposed stdout");
    assert!(
        recomposed.contains(FINDINGS),
        "the edge-walk slice must read the separately-committed research's findings into \
         the guidance; got:\n{recomposed}",
    );

    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#thesis"),
        b"Ingest is the constraint.\n",
        vision_task,
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#invariants"),
        b"One writer, many readers.\n",
        vision_task,
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#open-questions"),
        b"When does sharding earn its keep?\n",
        vision_task,
    );
    fill_commit(repo.path(), home.path(), vision_task);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", vision_task],
            None,
        ),
        "the vision task finalizes over the moved-but-disjoint base",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(after, before + 1, "exactly ONE commit lands");

    let managed = committed(repo.path(), "VISION.md");
    assert!(
        managed.contains("Ingest is the constraint.") && managed.contains(&research),
        "the committed vision carries the authored thesis and the grounding it was formed \
         from; got:\n{managed}",
    );
}

/// (4): `form-vision` is on the router selection surface (selectable: true) AND narrated
/// by `describe` with its authored prose.
#[test]
fn form_vision_is_on_the_router_catalog_and_described() {
    let repo = TempDir::new("catalog");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Router catalog: bare `jigc start --format json` (no intent) ORIENTS and lists
    // `form-vision` in the selectable `workflows` array (the emitted-bytes contract).
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
    assert!(
        ids.contains(&"form-vision"),
        "the selectable catalog must name `form-vision` (selectable: true); got: {ids:?}",
    );

    // describe narrates the workflow's authored description + usage (woven prose).
    let describe = jigc(repo.path(), home.path(), &["describe"], None);
    assert_ok(
        &describe,
        "`jigc describe` over the `[dev ▸ methodology]` composition",
    );
    let out = String::from_utf8(describe.stdout).expect("utf-8 stdout");
    assert!(
        out.contains("form-vision is"),
        "describe must narrate the `form-vision` workflow's authored description; got:\n{out}",
    );
}
