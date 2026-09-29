//! M37 Increment 4 / T1 — the **`do-research`** driving workflow, proven end-to-end
//! over the real `jigc` binary under the methodology pack (`design/design-altitude-doctypes.md`
//! §3; `implementation/roadmap.md` → M37 Increment 4).
//!
//! `do-research` is the first of the design-altitude trio's driving workflows: it
//! authors + commits a `research` doc standalone (so `form-vision` can later read it
//! committed — the Shape-2 committed-read path). It is `creates-task: true,
//! selectable: true` with a `when:` hint, and its body composes
//! `step:author-research` (which surfaces `{{cli.create-research}}`) over the shipped
//! generic `step:finalize`.
//!
//! The done-criteria, all on the EMITTED bytes of the real binary
//! (`CARGO_BIN_EXE_jigc`) over the on-disk methodology pack (`JIGC_PACK_DIR=<methodology>`,
//! the sibling methodology-test seam — `jigc setup` installs the pack, promotions read
//! back via `git show HEAD:<path>`):
//!   (1) **compose** — `jigc start --workflow do-research "<intent>"` exits 0 and its
//!       composed instructions carry the RESOLVED `{{cli.create-research}}` command-ref
//!       (a literal `jigc doc create research …` line), never the unresolved placeholder
//!       (the emitted-artifact contract, hardening #4);
//!   (2) **create + author + finalize** — under that task, `doc create research` (the
//!       create-gate GRANTS `research`), authoring `#question`/`#findings`/`#sources`,
//!       then `finalize` lands exactly ONE commit promoting `research/<slug>.md` with an
//!       on-create `date`;
//!   (3) **router catalog** — bare `jigc start` lists `do-research` (selectable: true);
//!   (4) **describe** — `jigc describe` narrates the `do-research` workflow's authored
//!       description + usage.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = env::temp_dir();
        let unique = format!(
            "jigc-do-research-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (composition mints, which reads HEAD).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` over the methodology pack (`JIGC_PACK_DIR=<methodology>`),
/// optionally piping `stdin`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
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

/// Run `jigc setup` (installs the methodology pack) over a fresh repo, asserting OK.
fn setup(repo: &Path, home: &Path) {
    assert_ok(
        &jigc(repo, home, &["setup"], None),
        "`JIGC_PACK_DIR=<methodology> jigc setup`",
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

/// Fill the commit doc's author-required levers so finalize validates clean.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        assert_ok(
            &jigc(
                repo,
                home,
                &["doc", "set-field", addr, "--value", value],
                None,
            ),
            &format!("set-field {addr}"),
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        assert_ok(
            &jigc(
                repo,
                home,
                &["doc", "set-slot", addr, "--from-file", "-"],
                Some(prose),
            ),
            &format!("set-slot {addr}"),
        );
    };
    set_field(&format!("commit:{task}#type"), "docs");
    set_field(&format!("commit:{task}#scope"), "research");
    set_slot(&format!("commit:{task}#summary"), b"record the research\n");
    set_slot(
        &format!("commit:{task}#body"),
        b"An investigation record.\n",
    );
}

/// (1) + (2): the compose + create + author + finalize spine. `jigc start --workflow
/// do-research "<intent>"` composes the RESOLVED `create-research` ref (proving
/// `{{cli.create-research}}` resolves against the catalog), the create-gate GRANTS
/// `research`, and finalize lands exactly one commit promoting `research/<slug>.md`
/// with an on-create `date`.
#[test]
fn do_research_composes_creates_authors_and_finalizes() {
    let repo = TempDir::new("spine");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    // (1) Compose: the workflow mints a task and emits its composed instructions. The
    // create-research command-ref must be RESOLVED into a literal command line — never
    // left as the unresolved `{{cli.create-research}}` placeholder (hardening #4).
    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "do-research", "benchmark the cache"],
        None,
    );
    assert_ok(
        &start,
        "`jigc start --workflow do-research` must compose (create-gate granted, \
         create-research resolves)",
    );
    let composed = String::from_utf8(start.stdout).expect("utf-8 composed stdout");
    assert!(
        composed.contains("jigc doc create research"),
        "the composed workflow must carry the RESOLVED create-research command-ref \
         (a `jigc doc create research …` line); got:\n{composed}",
    );
    assert!(
        !composed.contains("cli.create-research"),
        "the create-research placeholder must be RESOLVED, not emitted literally; got:\n{composed}",
    );

    // (2) Create + author + finalize. The create-gate admits `research` (the workflow's
    // `allows-create: [{type: research, as: record}]`).
    let task = "benchmark-the-cache";
    let create = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "research",
            "--title",
            "Cache Benchmarks",
            "--task",
            task,
        ],
        None,
    );
    assert_ok(
        &create,
        "`doc create research` — the create-gate grants research",
    );
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(addr, "research:cache-benchmarks", "id-from: title slugs it");

    let set_slot = |section: &str, prose: &[u8]| {
        assert_ok(
            &jigc(
                repo.path(),
                home.path(),
                &[
                    "doc",
                    "set-slot",
                    &format!("{addr}#{section}"),
                    "--from-file",
                    "-",
                    "--task",
                    task,
                ],
                Some(prose),
            ),
            &format!("set-slot {addr}#{section}"),
        );
    };
    set_slot("question", b"Is one node enough?\n");
    set_slot(
        "findings",
        b"A single node caps throughput under contention.\n",
    );
    set_slot(
        "sources",
        b"The load-test transcript and the p99 latency graph.\n",
    );

    fill_commit(repo.path(), home.path(), task);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`task finalize` (do-research) — promotes the research record",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "do-research finalize lands exactly ONE commit"
    );

    // The research record is promoted to its canonical path, carrying the three prose
    // slots and an on-create `date` (the `meta` header's CLI-set field).
    let doc = committed(repo.path(), "research/cache-benchmarks.md");
    assert!(
        doc.contains("Is one node enough?")
            && doc.contains("A single node caps throughput under contention.")
            && doc.contains("The load-test transcript and the p99 latency graph."),
        "the promoted research carries all three authored slots; got:\n{doc}",
    );
    let date_line = doc
        .lines()
        .find_map(|l| l.trim().strip_prefix("date:"))
        .unwrap_or_else(|| {
            panic!("the promoted research carries an on-create `date`; got:\n{doc}")
        });
    let date = date_line.trim();
    assert!(
        date.len() == 10 && date.as_bytes()[4] == b'-' && date.as_bytes()[7] == b'-',
        "the on-create date is an ISO `YYYY-MM-DD`; got {date:?}",
    );
}

/// (3) + (4): `do-research` is on the router catalog (selectable: true) AND narrated by
/// `describe` with its authored prose.
#[test]
fn do_research_is_on_the_router_catalog_and_described() {
    let repo = TempDir::new("catalog");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    // (3) Router catalog: bare `jigc start --format json` lists `do-research` in the
    // selectable `workflows` array (the emitted-bytes contract, like flow19's negative).
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
        ids.contains(&"do-research"),
        "the selectable catalog must name `do-research` (selectable: true); got: {ids:?}",
    );

    // (4) describe narrates the workflow's authored description + usage (woven prose).
    let describe = jigc(repo.path(), home.path(), &["describe"], None);
    assert_ok(&describe, "`jigc describe` over the methodology pack");
    let out = String::from_utf8(describe.stdout).expect("utf-8 stdout");
    assert!(
        out.contains("do-research is"),
        "describe must narrate the `do-research` workflow's authored description; got:\n{out}",
    );
    assert!(
        out.contains(
            "Reach for it when a question needs evidence gathered and recorded before a vision or decision rests on it, and you want that investigation carried through the binary to one committed research record."
        ),
        "describe must narrate the `do-research` workflow's authored usage; got:\n{out}",
    );
}
