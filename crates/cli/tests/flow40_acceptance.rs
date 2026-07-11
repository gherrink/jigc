//! M39 Increment 8 / T1 — the **M39 done-picture acceptance suite**: the whole
//! pre-1.0 RC-findings wave, driven end-to-end through the **real `jigc` binary** over
//! the **`[dev ▸ methodology]`** composition. Inc 1–7 proved each feature per-feature
//! (`doc_show.rs`, `flow_form_vision.rs`, `milestone_record_fresh_clone.rs`,
//! `slug_override.rs`, `flow_form_vision_advisory.rs`); this suite ties them into the
//! four integrated done-picture arms (`design/team-ready-state.md` → Acceptance;
//! `design/worked-examples.md` → flow 40, authored in T3).
//!
//! The four arms, each a `#[test]` over the real binary:
//!
//!   (1) **F1 — the fresh-session read/revise story.** A committed `vision` grounded in
//!       TWO committed `research` docs: the Shape-2 re-compose edge-walk reads BOTH
//!       groundings' findings (the multi-valued anchor), finalize promotes `VISION.md`,
//!       and the M39 read surface (`jigc doc show`) serves the committed vision carrying
//!       BOTH grounded-in targets — the read + revise loop that made the record legible.
//!
//!   (2) **team-ready arc.** `milestone create → add-task ×2 → finalize` (the join-commit
//!       boundary) yields the committed `milestone-record` with the base pin + both
//!       sub-tasks + the header flipped `joined`; a **fresh clone** (delete `.jigc/`)
//!       reads it via `jigc doc show milestone-record:<id> --format json` (the pinned 1.0
//!       shape) and **continues** — a subsequent milestone op re-derives the demoted cache
//!       from the committed record.
//!
//!   (3) **freeze-exempt relocation.** A pre-existing committed instance of a freeze-exempt
//!       doctype stranded at a prior home is **detected + moved** to its current schema home
//!       by `jigc relocate`, never silently stranded. Since M40 A1 the methodology doctypes
//!       are manifest-frozen, so the moving arm rides a genuinely manifest-less fixture
//!       doctype and `research` asserts the refusal + `migrate-corpus` route.
//!
//!   (4) **slug + advisory.** A long intent mints a `≤5`-word capped slug (and `--slug`
//!       sets identity verbatim); `form-vision` composed against an EMPTY research store
//!       prints the `do-research` advisory (non-blocking).
//!
//! Everything is asserted on the EMITTED bytes / exit codes / committed files of the real
//! binary (`CARGO_BIN_EXE_jigc`). No external test crates.

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
            "jigc-flow40-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer.
fn git_init(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// `[dev ▸ methodology]` via the **listed-pack** mechanism: `packs.yaml` names the on-disk
/// methodology pack OVER the embedded dev base (the `doc_show` / `flow_design_altitude`
/// harness — methodology multi-instance doctypes land flat at their `location:`, so
/// `research` → `research/<slug>.md`, `vision` → the root `VISION.md` placement literal).
fn init_listed_pack(repo: &Path) {
    git_init(repo);
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// `[dev ▸ methodology]` via the **compose-embedded marker** — the exact key `make_pack`
/// reads to assemble the composition dev-highest (so the dev `docs-root` knob applies →
/// `docs/`, and `milestone-record` homes at `docs/milestone-records/<id>.md`). The
/// `milestone_record_*` harness shape.
fn init_compose_marker(repo: &Path) {
    git_init(repo);
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`. Never
/// inherits a harness `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT — else
/// the env pack supersedes the marker — and the listed-pack base is the EmbeddedPack either
/// way, so removing it is safe + uniform across both mechanisms).
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
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

/// Trimmed stdout of an invocation.
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone())
        .expect("utf-8 stdout")
        .trim()
        .to_string()
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

/// Fill the auto-provisioned commit doc's levers so a code-less finalize validates clean.
fn fill_commit(repo: &Path, home: &Path, task: &str, scope: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), scope);
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
        b"A managed doc.\n",
    );
}

/// The `HEAD` commit count.
fn head_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap()
}

/// Commit one grounding `research` doc through the REAL `do-research` workflow, so it is a
/// reachable `grounded-in` target with a genuinely-committed home. `intent` derives the task
/// id; `title` mints `research:<slug>`. Returns the committed `research:<slug>`.
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
    let addr = stdout_of(&create);
    set_slot(repo, home, &format!("{addr}#question"), b"A question.\n");
    set_slot(repo, home, &format!("{addr}#findings"), findings);
    set_slot(repo, home, &format!("{addr}#sources"), b"Some sources.\n");
    let task = intent.replace(' ', "-");
    fill_commit(repo, home, &task, "research");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task], None),
        "`jigc task finalize` (do-research) — the committed grounding target",
    );
    addr
}

// ───────────────────────────── Arm 1 — F1 ─────────────────────────────

/// **Arm 1 (F1).** A committed `vision` grounded in TWO committed `research` docs: the
/// Shape-2 re-compose edge-walk reads BOTH groundings' findings (the multi-valued anchor),
/// finalize promotes the managed `VISION.md`, and the M39 read surface (`jigc doc show`)
/// serves the committed vision carrying BOTH grounded-in targets — a fresh session's read +
/// revise loop over a committed, legible record.
#[test]
fn f1_committed_vision_reads_and_recomposes_both_groundings() {
    let repo = TempDir::new("f1");
    let home = TempDir::new("home");
    init_listed_pack(repo.path());

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

    // form-vision → create the singleton → ground it in BOTH research → RE-COMPOSE.
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
    let addr = stdout_of(&create);
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

    // RE-COMPOSE: the edge-walk slice now reads ALL grounding research's findings — the
    // load-bearing multi-valued read (both distinct findings echoed into the guidance).
    let resume = jigc(repo.path(), home.path(), &["start", "--task", task], None);
    assert_ok(&resume, "`jigc start --task <id>` re-compose (form-vision)");
    let recomposed = String::from_utf8(resume.stdout).expect("utf-8 recomposed stdout");
    assert!(
        recomposed.contains(FIRST_FINDINGS) && recomposed.contains(SECOND_FINDINGS),
        "the re-compose edge-walk must read BOTH groundings' findings into the guidance; \
         got:\n{recomposed}",
    );

    // Author + finalize → the managed VISION.md.
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
    fill_commit(repo.path(), home.path(), task, "vision");
    let before = head_count(repo.path());
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`jigc task finalize` (form-vision) — BOTH grounded-in targets resolve, no block",
    );
    assert_eq!(
        head_count(repo.path()),
        before + 1,
        "form-vision finalize lands exactly ONE commit",
    );

    // The M39 read surface serves the committed vision — plain `doc show` is the byte-exact
    // committed view, carrying the thesis AND BOTH grounded-in targets (F1 read/revise).
    let show = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision"],
        None,
    );
    assert_ok(
        &show,
        "`jigc doc show vision:vision` over the committed record",
    );
    let shown = stdout_of(&show);
    assert!(
        shown.contains("A context compiler for coding agents.")
            && shown.contains(&research_a)
            && shown.contains(&research_b),
        "the read surface serves the committed vision carrying the thesis + BOTH grounded-in \
         targets; got:\n{shown}",
    );
}

// ───────────────────────── Arm 2 — team-ready arc ─────────────────────────

/// **Arm 2.** `milestone create → add-task ×2 → finalize` (the join-commit boundary) yields
/// the committed `milestone-record` with the base pin + both sub-tasks + the header flipped
/// `joined`; a **fresh clone** (delete `.jigc/`) reads it via `jigc doc show
/// milestone-record:<id> --format json` (the pinned 1.0 shape) and continues — a subsequent
/// milestone op re-derives the demoted cache from the committed record.
#[test]
fn team_ready_arc_joins_then_fresh_clone_reads_and_continues() {
    let repo = TempDir::new("team");
    let home = TempDir::new("home");
    init_compose_marker(repo.path());

    let milestone = |args: &[&str]| -> std::process::Output {
        let mut v = vec!["milestone"];
        v.extend_from_slice(args);
        jigc(repo.path(), home.path(), &v, None)
    };

    // Commit the tracked compose marker BEFORE `create` — it is what survives a clone, and
    // pinning the milestone base AFTER this commit keeps `base..HEAD` record-only (else this
    // non-record commit would block the finalize base-guard).
    git(repo.path(), &["add", ".jigc/config/packs.yaml"]);
    git(repo.path(), &["commit", "-q", "-m", "jigc config"]);
    // `create` pins the base to HEAD at create time (this `jigc config` commit).
    let base_sha = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    let base_short = git(repo.path(), &["rev-parse", "--short", "HEAD"])
        .trim()
        .to_string();

    assert_ok(&milestone(&["create", "Cache rework"]), "milestone create");
    assert_ok(
        &milestone(&["add-task", "cache-rework", "Warm the read cache"]),
        "add-task #1",
    );
    assert_ok(
        &milestone(&["add-task", "cache-rework", "Evict cold entries"]),
        "add-task #2",
    );

    let record_rel = "docs/milestone-records/cache-rework.md";
    let record_abs = repo.path().join(record_rel);
    let before = fs::read_to_string(&record_abs).expect("record after add-task ×2");
    assert_eq!(
        before.matches("status: active").count(),
        3,
        "before finalize: the header + both items are `status: active`; got:\n{before}",
    );

    // finalize is the join-commit boundary — flip every status active → joined, folded into
    // the single finalize commit.
    assert_ok(
        &milestone(&["finalize", "cache-rework"]),
        "milestone finalize",
    );
    let joined = fs::read_to_string(&record_abs).expect("record after finalize");
    assert_eq!(
        joined,
        before.replace("status: active", "status: joined"),
        "finalize flips only the status values, byte-stable; every other byte survives",
    );
    assert_eq!(
        joined.matches("status: joined").count(),
        3,
        "the header + both items flipped to joined; got:\n{joined}",
    );

    // ── Fresh clone: drop ALL of `.jigc/`, restore only the *tracked* bits a clone carries. ──
    fs::remove_dir_all(repo.path().join(".jigc")).expect("rm -rf .jigc");
    git(repo.path(), &["checkout", "--", ".jigc"]);
    let cache = repo
        .path()
        .join(".jigc")
        .join("milestones")
        .join("cache-rework");
    assert!(
        !cache.exists(),
        "the WIP cache must be absent after the fresh-clone simulation",
    );

    // The read surface serves the committed JOINED record under the pinned 1.0 json shape,
    // WITHOUT any `.jigc/` cache (Inc 1's committed-record read).
    let show = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "milestone-record:cache-rework",
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(
        &show,
        "fresh-clone `doc show milestone-record --format json`",
    );
    let json = stdout_of(&show);
    for needle in [
        "\"type\": \"milestone-record\"",
        "\"slug\": \"cache-rework\"",
        "\"status\": \"joined\"",
        // `schema-version` joined the pinned witness fields when the M40 A1
        // methodology manifest froze milestone-record.
        "\"schema-version\": \"1\"",
        "\"task-id\": \"warm-the-read-cache\"",
        "\"intent\": \"Warm the read cache\"",
        "\"task-id\": \"evict-cold-entries\"",
    ] {
        assert!(
            json.contains(needle),
            "the pinned `--format json` shape must carry `{needle}`; got:\n{json}",
        );
    }
    // No un-flipped status survived the join.
    assert!(
        !json.contains("\"status\": \"active\""),
        "the joined record must carry no `active` status; got:\n{json}",
    );
    // The compound `base` pin projects as a structured `{ sha, short }` object — the pinned
    // 1.0 machine shape, NOT the space-joined scalar the `.md` stores (`DECISIONS.md`
    // 2026-07-07). Parse + assert the object (a substring check would pass on the scalar too).
    let doc: serde_json::Value = serde_json::from_str(&json).expect("pinned json parses");
    let base = &doc["fields"]["base"];
    assert_eq!(
        base["sha"].as_str(),
        Some(base_sha.as_str()),
        "the pinned json `base` object carries the full SHA; got:\n{json}",
    );
    assert_eq!(
        base["short"].as_str(),
        Some(base_short.as_str()),
        "the pinned json `base` object carries the short SHA; got:\n{json}",
    );
    assert!(
        !base.is_string(),
        "the pinned json `base` must be a structured object, not the space-joined scalar; got:\n{json}",
    );

    // Continue: a subsequent milestone op re-derives the demoted cache from the committed
    // record and lists the sub-tasks (resume-from-scratch).
    let list = jigc(
        repo.path(),
        home.path(),
        &["milestone", "list-tasks", "cache-rework"],
        None,
    );
    assert_ok(&list, "fresh-clone `list-tasks` (resume)");
    let out = stdout_of(&list);
    assert!(
        out.contains("evict-cold-entries") && out.contains("warm-the-read-cache"),
        "fresh-clone `list-tasks` emits the re-derived sub-task ids; got:\n{out}",
    );
    assert!(
        cache.join("tasks.json").is_file() && cache.join("base.json").is_file(),
        "the milestone op re-seeded `.jigc/milestones/cache-rework/{{base,tasks}}.json` from \
         the committed record",
    );
}

// ───────────────────── Arm 3 — freeze-exempt relocation ─────────────────────

/// A minimal **manifest-less** pack dir shipping one persisted `note` doctype
/// (`location: notes/`) — the genuinely freeze-exempt fixture arm 3 relocates,
/// now that the methodology pack ships its own `schema-manifest.yaml` (M40 A1)
/// and its doctypes left the freeze-exempt set.
fn write_noteless_pack(dir: &Path) {
    fs::create_dir_all(dir.join("schemas")).expect("mk fixture pack schemas/");
    fs::write(
        dir.join("schemas").join("note.yaml"),
        "type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The note.\" }\n",
    )
    .expect("write the note schema");
}

/// **Arm 3.** A pre-existing committed instance of a genuinely **freeze-exempt**
/// (manifest-less) doctype stranded at a prior home is **detected + moved** to its current
/// schema home by `jigc relocate`, never silently stranded — while a **manifest-frozen**
/// methodology doctype (`research`, in the M40 A1 methodology `schema-manifest.yaml`) is
/// **refused** by the same verb and routed to the version-gated `jigc migrate-corpus`
/// (`design/corpus-migration.md` → M40 revises the dichotomy: the exempt path covers only
/// genuinely manifest-less doctypes).
#[test]
fn freeze_exempt_relocation_moves_a_stranded_committed_instance() {
    let repo = TempDir::new("reloc");
    let home = TempDir::new("home");
    git_init(repo.path());
    // `[note-pack ▸ methodology]` over the embedded dev base: the manifest-less `note`
    // fixture pack plus the manifest-bearing methodology pack (for the refusal arm).
    let note_pack = repo.path().join(".jigc").join("note-pack");
    write_noteless_pack(&note_pack);
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!(
            "packs:\n  - {}\n  - {}\n",
            note_pack.display(),
            methodology_pack_tree().display()
        ),
    )
    .expect("write packs.yaml naming the note + methodology packs");

    // A committed `note` instance stranded at a legacy prior home (a pre-existing strand
    // from before the home convention — hand-authored, committed, never at `notes/`).
    let current_rel = "notes/cache-benchmarks.md";
    let prior_rel = "docs/legacy-notes/cache-benchmarks.md";
    let body = "# Cache Benchmarks\n\n## Body\n\nA single node caps throughput.\n";
    fs::create_dir_all(repo.path().join("docs").join("legacy-notes")).expect("mk prior home");
    fs::write(repo.path().join(prior_rel), body).expect("write the stranded note");
    git(repo.path(), &["add", prior_rel]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "strand the note at a legacy home"],
    );

    // Detect + move: `jigc relocate note --from docs/legacy-notes/`.
    let out = jigc(
        repo.path(),
        home.path(),
        &["relocate", "note", "--from", "docs/legacy-notes/"],
        None,
    );
    assert_ok(&out, "`jigc relocate note --from docs/legacy-notes/`");
    let report = stdout_of(&out);
    assert!(
        report.contains("1 moved") && report.contains(&format!("{prior_rel} -> {current_rel}")),
        "the relocation report names the detected move prior → current; got:\n{report}",
    );

    // The instance lands at its current schema home, byte-preserving, and the prior home
    // is vacated — never silently stranded.
    assert!(
        repo.path().join(current_rel).exists(),
        "the stranded instance must land at its current schema home",
    );
    assert!(
        !repo.path().join(prior_rel).exists(),
        "the prior home must be vacated after the move",
    );
    assert_eq!(
        fs::read(repo.path().join(current_rel)).expect("read the relocated instance"),
        body.as_bytes(),
        "the freeze-exempt relocation is byte-preserving (a pure `git mv`)",
    );

    // The reconcile: `research` is now manifest-FROZEN (M40 A1), so the freeze-exempt verb
    // refuses it and routes to the version-gated migrate-corpus path.
    let refused = jigc(
        repo.path(),
        home.path(),
        &["relocate", "research", "--from", "docs/legacy-research/"],
        None,
    );
    assert!(
        !refused.status.success(),
        "`jigc relocate research` must be refused now that research is manifest-frozen; \
         stdout:\n{}",
        String::from_utf8_lossy(&refused.stdout),
    );
    let stderr = String::from_utf8_lossy(&refused.stderr);
    assert!(
        stderr.contains("frozen") && stderr.contains("migrate-corpus"),
        "the refusal names the freeze and routes to `jigc migrate-corpus`; got:\n{stderr}",
    );
}

// ──────────────────────── Arm 4 — slug + advisory ────────────────────────

/// **Arm 4.** A long intent mints a `≤5`-word capped slug and `--slug` sets identity
/// verbatim (G6); `form-vision` composed against an EMPTY research store prints the
/// non-blocking `do-research` advisory (G7). Two independent RC-findings features asserted
/// over the real binary.
#[test]
fn long_intent_caps_slug_and_form_vision_advises_on_empty_research() {
    // (a) Slug capping + `--slug` override — the `single-task` mint path (dev base workflow;
    //     a bare `.jigc/config` project layer resolves it).
    let slug_repo = TempDir::new("slug");
    let slug_home = TempDir::new("slug-home");
    git_init(slug_repo.path());

    // A 9-word intent caps to its first 5 words at a `-` boundary; the F5
    // edge-stopword rule then drops the trailing stopword (`to`) the cap exposes.
    let long = "move the session cache to a shared redis cluster";
    assert_ok(
        &jigc(
            slug_repo.path(),
            slug_home.path(),
            &["start", "--workflow", "single-task", long],
            None,
        ),
        "`jigc start --workflow single-task <long intent>`",
    );
    let capped = "move-the-session-cache";
    assert!(
        slug_repo.path().join(".jigc/tasks").join(capped).is_dir(),
        "the long intent must mint under the ≤5-word capped slug `{capped}`",
    );
    assert!(
        capped.split('-').count() <= 5,
        "the capped slug carries at most 5 dash-separated words",
    );

    // `--slug` sets identity verbatim — never the capped-intent slug.
    assert_ok(
        &jigc(
            slug_repo.path(),
            slug_home.path(),
            &[
                "start",
                "--workflow",
                "single-task",
                long,
                "--slug",
                "redis-cache",
            ],
            None,
        ),
        "`jigc start --workflow single-task <long> --slug redis-cache`",
    );
    assert!(
        slug_repo.path().join(".jigc/tasks/redis-cache").is_dir(),
        "`--slug` mints the task under the explicit id verbatim",
    );

    // (b) The empty-research advisory — `form-vision` over an EMPTY research store surfaces the
    //     non-blocking `do-research` nudge (the `[dev ▸ methodology]` composition).
    let adv_repo = TempDir::new("advisory");
    let adv_home = TempDir::new("advisory-home");
    init_listed_pack(adv_repo.path());

    let start = jigc(
        adv_repo.path(),
        adv_home.path(),
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
        "`jigc start --workflow form-vision` (empty research store)",
    );
    let composed = String::from_utf8(start.stdout).expect("utf-8 composed stdout");
    assert!(
        composed.contains("consider running `do-research`"),
        "an EMPTY research store must surface the `do-research` advisory; got:\n{composed}",
    );
    // Advisory, never blocking — form-vision still composes fully.
    assert!(
        composed.contains("jigc doc create vision"),
        "the advisory is non-blocking — form-vision still composes; got:\n{composed}",
    );
}
