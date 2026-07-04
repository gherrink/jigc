//! M37 Increment 4 / T2 — the **`park-idea`** driving workflow, proven end-to-end
//! over the real `jigc` binary under the methodology pack (`design/design-altitude-doctypes.md`
//! §3, fork 5; `implementation/roadmap.md` → M37 Increment 4).
//!
//! `park-idea` is the design-altitude trio's parking flow: it authors + commits a
//! standalone `idea` doc (the shaped direction + the trigger that would bring it back)
//! as one commit, so a mid-work thought has a durable home cheaper than losing it. It is
//! `creates-task: true, selectable: true` with a `when:` hint — parking must be
//! discoverable (the fork-5 requirement), so it legitimately joins the router catalog.
//! Its body composes `step:author-idea` (which surfaces `{{cli.create-idea}}`, sets the
//! `trigger` field + `description` slot) over the shipped generic `step:finalize`.
//!
//! The done-criteria, all on the EMITTED bytes of the real binary
//! (`CARGO_BIN_EXE_jigc`) over the on-disk methodology pack (`JIGC_PACK_DIR=<methodology>`):
//!   (1) **compose** — `jigc start --workflow park-idea "<intent>"` exits 0 and its
//!       composed instructions carry the RESOLVED `{{cli.create-idea}}` command-ref
//!       (a literal `jigc doc create idea …` line), never the unresolved placeholder
//!       (the emitted-artifact contract, hardening #4);
//!   (2) **create + author + finalize** — under that task, `doc create idea` (the
//!       create-gate GRANTS `idea`), setting the `trigger` field + authoring the
//!       `description` slot, then `finalize` lands exactly ONE commit promoting
//!       `ideas/<slug>.md` with the trigger, the description, and an on-create `date`;
//!   (3) **router catalog** — bare `jigc start` lists `park-idea` (selectable: true);
//!   (4) **describe** — `jigc describe` narrates the `park-idea` workflow's authored
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
            "jigc-park-idea-{tag}-{}-{:?}",
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
    set_field(&format!("commit:{task}#scope"), "ideas");
    set_slot(&format!("commit:{task}#summary"), b"park the idea\n");
    set_slot(&format!("commit:{task}#body"), b"A shaped direction.\n");
}

/// (1) + (2): the compose + create + author + finalize spine. `jigc start --workflow
/// park-idea "<intent>"` composes the RESOLVED `create-idea` ref (proving
/// `{{cli.create-idea}}` resolves against the catalog), the create-gate GRANTS `idea`,
/// and finalize lands exactly one commit promoting `ideas/<slug>.md` with the trigger,
/// the description, and an on-create `date`.
#[test]
fn park_idea_composes_creates_authors_and_finalizes() {
    let repo = TempDir::new("spine");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    // (1) Compose: the workflow mints a task and emits its composed instructions. The
    // create-idea command-ref must be RESOLVED into a literal command line — never left
    // as the unresolved `{{cli.create-idea}}` placeholder (hardening #4).
    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "park-idea", "a cache warming pass"],
        None,
    );
    assert_ok(
        &start,
        "`jigc start --workflow park-idea` must compose (create-gate granted, \
         create-idea resolves)",
    );
    let composed = String::from_utf8(start.stdout).expect("utf-8 composed stdout");
    assert!(
        composed.contains("jigc doc create idea"),
        "the composed workflow must carry the RESOLVED create-idea command-ref \
         (a `jigc doc create idea …` line); got:\n{composed}",
    );
    assert!(
        !composed.contains("cli.create-idea"),
        "the create-idea placeholder must be RESOLVED, not emitted literally; got:\n{composed}",
    );

    // (2) Create + author + finalize. The create-gate admits `idea` (the workflow's
    // `allows-create: [{type: idea, as: idea}]`).
    let task = "a-cache-warming-pass";
    let create = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "idea",
            "--title",
            "Warm The Cache On Boot",
            "--task",
            task,
        ],
        None,
    );
    assert_ok(&create, "`doc create idea` — the create-gate grants idea");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(
        addr, "idea:warm-the-cache-on-boot",
        "id-from: title slugs it"
    );

    // The `trigger` field (agent-set string) + the `description` slot.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{addr}#trigger"),
                "--value",
                "cold-start latency becomes a complaint",
                "--task",
                task,
            ],
            None,
        ),
        &format!("set-field {addr}#trigger"),
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                &format!("{addr}#description"),
                "--from-file",
                "-",
                "--task",
                task,
            ],
            Some(b"Prefill the hot keys during boot so the first request is warm.\n"),
        ),
        &format!("set-slot {addr}#description"),
    );

    fill_commit(repo.path(), home.path(), task);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`task finalize` (park-idea) — promotes the idea record",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "park-idea finalize lands exactly ONE commit"
    );

    // The idea record is promoted to its canonical path, carrying the trigger field, the
    // description slot, and an on-create `date` (the `meta` header's CLI-set field).
    let doc = committed(repo.path(), "ideas/warm-the-cache-on-boot.md");
    assert!(
        doc.contains("cold-start latency becomes a complaint")
            && doc.contains("Prefill the hot keys during boot so the first request is warm."),
        "the promoted idea carries the authored trigger + description; got:\n{doc}",
    );
    let date_line = doc
        .lines()
        .find_map(|l| l.trim().strip_prefix("date:"))
        .unwrap_or_else(|| panic!("the promoted idea carries an on-create `date`; got:\n{doc}"));
    let date = date_line.trim();
    assert!(
        date.len() == 10 && date.as_bytes()[4] == b'-' && date.as_bytes()[7] == b'-',
        "the on-create date is an ISO `YYYY-MM-DD`; got {date:?}",
    );
}

/// (3) + (4): `park-idea` is on the router catalog (selectable: true — the fork-5
/// discoverability requirement) AND narrated by `describe` with its authored prose.
#[test]
fn park_idea_is_on_the_router_catalog_and_described() {
    let repo = TempDir::new("catalog");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    // (3) Router catalog: bare `jigc start --format json` lists `park-idea` in the
    // selectable `workflows` array (the emitted-bytes contract).
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
        ids.contains(&"park-idea"),
        "the selectable catalog must name `park-idea` (selectable: true, the fork-5 \
         parking-discoverability requirement); got: {ids:?}",
    );

    // (4) describe narrates the workflow's authored description + usage (woven prose).
    let describe = jigc(repo.path(), home.path(), &["describe"], None);
    assert_ok(&describe, "`jigc describe` over the methodology pack");
    let out = String::from_utf8(describe.stdout).expect("utf-8 stdout");
    assert!(
        out.contains("park-idea is"),
        "describe must narrate the `park-idea` workflow's authored description; got:\n{out}",
    );
    assert!(
        out.contains(
            "Reach for it when a direction is worth keeping but not worth scheduling now, and you want it carried through the binary to one committed idea record with the trigger that would revisit it."
        ),
        "describe must narrate the `park-idea` workflow's authored usage; got:\n{out}",
    );
}
