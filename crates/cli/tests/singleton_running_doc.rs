//! M16 Increment 2 / T3 — the **running-singleton maintenance substrate** proven
//! end-to-end through the real `jigc` binary (`design/methodology-docs.md` →
//! Acceptance flows, the running-singleton maintained over time / Codex finding 4;
//! `design/reconciliation.md` → the state machine).
//!
//! The substrate this increment ships — a `singleton: true` doctype that mints at a
//! **fixed slug = the type id** and an **idempotent `create`** that copies-in an
//! already-committed singleton instead of clobbering it — is exercised over a
//! throwaway git repo against a **fixture** singleton doctype (`runlog`). The fixture
//! is a test pack, never methodology-pack content (the real running doctypes land in
//! increments 4–5); it ships its OWN `location: runlog/` subdir (review finding B-1)
//! with a repeatable `entries` section as the `add-item` append target.
//!
//! Every assertion runs on the EMITTED bytes of the real binary
//! (`CARGO_BIN_EXE_jigc`); the committed promotion is read back via `git show
//! HEAD:<path>`, so the byte-stability claims are over the bytes git actually holds.
//! The fixture pack is **listed** in the in-repo `.jigc/config/packs.yaml` (the
//! flow-18 mechanism) so it UNIONS with the embedded base pack — the base supplies
//! `commit`/knobs/defaults, the fixture supplies its singleton doctype + a
//! `creates-task: true` workflow whose create-gate admits it.
//!
//! The four done-criteria:
//!   (a) **cold-create round-trips** — `jigc start --workflow <fixture>` → `doc create
//!       runlog` → author → `task finalize` promotes `runlog/runlog.md` byte-stable;
//!   (b) **warm re-create copies-in + `add-item` appends** — a second task over the
//!       committed singleton copies the committed body in (prior content preserved),
//!       `doc add-item` appends an entry, finalize re-promotes byte-stable;
//!   (c) **baseline-recorded-then-OOB-drifted warm edit conflict-blocks** at
//!       finalize-preflight with `reconciliation.conflict-block` (the baseline is
//!       recorded by the cold finalize FIRST, then the committed file is drifted
//!       out-of-band — an unrecorded-baseline drift would vacuously baseline-adopt);
//!   (d) **copy-in keys on the canonical path only** — a `create` of the embedded
//!       **non-singleton** `adr` doctype over a committed body at an OFF-canonical
//!       path mints fresh + serial-rejects on re-create (since M43 inc-7 T1 the
//!       copy-in is doctype-blind for the CANONICAL committed path —
//!       `create_or_update.rs` pins that half; this arm pins the boundary).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-singleton-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output. NO
/// `JIGC_PACK_DIR`: the embedded base pack stays in the union, and the listed fixture
/// pack (named in `packs.yaml`) composes alongside it (the flow-18 listed-pack seam).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Lift a **mechanical route's argv** out of the bytes the binary actually printed —
/// the backticked span opening with `prefix`, split on whitespace. The emitted bytes are
/// the contract (M47 inc-2 / T4): the route is run VERBATIM from this lift, never
/// reconstructed in test code, so a route that composes a placeholder instead of the real
/// id fails here rather than passing against a hand-built equivalent.
fn lifted_route_argv(rendered: &str, prefix: &str) -> Vec<String> {
    let open = rendered
        .find(&format!("`{prefix}"))
        .unwrap_or_else(|| panic!("no backticked `{prefix}…` route span in:\n{rendered}"));
    let rest = &rendered[open + 1..];
    let close = rest
        .find('`')
        .unwrap_or_else(|| panic!("the backticked route span never closes in:\n{rendered}"));
    rest[..close]
        .split_whitespace()
        .map(str::to_owned)
        .collect()
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

/// Seed the fixture pack: a `singleton: true` `runlog` doctype under its OWN
/// `location: runlog/` subdir (review finding B-1) with a repeatable `entries`
/// section (the `add-item` append target) and a prose `overview` slot; plus a
/// `creates-task: true` workflow `keep-runlog` whose create-gate admits `runlog`. The
/// fixture references no `{{cli.X}}` command, so an empty catalog satisfies the read.
fn seed_fixture_pack(pack: &Path) {
    let schemas = pack.join("schemas");
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    for d in [&schemas, &workflows, &steps, &config] {
        fs::create_dir_all(d).expect("mk fixture pack subdir");
    }

    // The fixture singleton doctype: a fixed slug (= the type id `runlog`) under its
    // own location, an `overview` prose slot, and a repeatable `entries` section
    // carrying one prose-slot item — the running-doc shape `add-item` appends into.
    fs::write(
        schemas.join("runlog.yaml"),
        "type: runlog\n\
         singleton: true\n\
         location: runlog/\n\
         description: A fixture running log maintained as a singleton over time.\n\
         usage: the test substrate for idempotent create-or-update over a singleton.\n\
         sections:\n\
         \x20 - id: overview\n\
         \x20   slot: {}\n\
         \x20 - id: entries\n\
         \x20   repeatable:\n\
         \x20     id-from: title\n\
         \x20     block:\n\
         \x20       - { id: title, type: string }\n\
         \x20       - { id: note, slot: {} }\n",
    )
    .expect("seed runlog schema");

    // A minimal `creates-task: true` workflow whose create-gate admits `runlog`. Its
    // single step references the engine-native `{{task.intent}}` only (no command-ref),
    // so the listed fixture composes against an empty catalog.
    fs::write(
        workflows.join("keep-runlog.yaml"),
        "---\n\
         when: maintain the fixture running log\n\
         description: A fixture workflow that creates-or-updates the runlog singleton.\n\
         usage: proving the singleton running-doc substrate through the binary.\n\
         creates-task: true\n\
         allows-create: [{type: runlog, as: log}]\n\
         ---\n\
         {{ include: step:keep }}\n",
    )
    .expect("seed keep-runlog workflow");
    fs::write(
        steps.join("keep.yaml"),
        "Maintain the running log for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed keep step");

    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    // A fixture pack that ships steps owes the four ambush-class statements since M51
    // Increment 8 T2: the stated-at fence's structural tier keys on the constituents
    // that SHIP STEPS and checks each in isolation.
    crate::support::seed_ambush_class_declarer(pack);
}

/// Record the fixture pack in the in-repo project layer's `packs.yaml`, UNIONing it
/// with the embedded base pack (the flow-18 listed-pack mechanism).
fn list_fixture_pack(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the fixture pack");
}

/// Fill the commit doc's author-required header + prose so a finalize over it
/// validates clean (the `commit` doctype comes from the base pack in the union).
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "chore");
    set_field(&format!("commit:{task}#scope"), "runlog");
    set_slot(&format!("commit:{task}#summary"), b"maintain the runlog\n");
    set_slot(
        &format!("commit:{task}#body"),
        b"A runlog maintenance pass.\n",
    );
}

/// Start a `keep-runlog` task for `intent`, returning its task id (the slugged intent).
fn start_keep_runlog(repo: &Path, home: &Path, intent: &str, task_id: &str) {
    let out = jigc(repo, home, &["start", "--workflow", "keep-runlog", intent]);
    assert_ok(
        &out,
        &format!("`jigc start --workflow keep-runlog` ({task_id})"),
    );
}

/// The committed `runlog/runlog.md` bytes at HEAD (the promoted singleton).
fn committed_runlog(repo: &Path) -> String {
    let out = Command::new("git")
        .args(["show", "HEAD:docs/runlog/runlog.md"])
        .current_dir(repo)
        .output()
        .expect("git show runlog");
    assert!(
        out.status.success(),
        "the singleton must be committed at docs/runlog/runlog.md; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 committed runlog")
}

#[test]
fn cold_create_promotes_the_singleton_byte_stable() {
    // (a) Cold-create: no committed runlog/runlog.md exists, so `doc create runlog`
    // mints the empty template at the FIXED slug (= the type id), author fills the
    // overview slot, and finalize promotes `runlog/runlog.md`. The promoted bytes are
    // byte-stable: the working-area staged bytes equal what git holds (the canonical
    // render survives the promote copy unchanged).
    let repo = TempDir::new("cold");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("pack");
    seed_fixture_pack(pack.path());
    list_fixture_pack(repo.path(), pack.path());

    let task = "open-the-running-log";
    start_keep_runlog(repo.path(), home.path(), "open the running log", task);

    // The fixed-slug mint: the created address is `runlog:runlog` (slug == type id),
    // never a title-slugged id. The `--title` is the one the schema fixes — a singleton
    // with no `display-title:` fixes it to the type id — because since M48 a divergent
    // one is refused rather than silently dropped (`write.title-ignored`;
    // `design/write-commands.md` → The four-way write).
    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "runlog", "--title", "runlog"],
        None,
    );
    assert_ok(&create, "`jigc doc create runlog` (cold)");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(
        addr, "runlog:runlog",
        "a singleton mints at the fixed slug = the type id, NOT a `--title` slug",
    );

    // Author the overview slot.
    let set = jigc_doc(
        repo.path(),
        home.path(),
        &["set-slot", "runlog:runlog#overview", "--from-file", "-"],
        Some(b"The cold-created running log.\n"),
    );
    assert_ok(&set, "`set-slot runlog:runlog#overview` (cold)");

    // The staged working-area bytes — the promote source.
    let staged_path = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("runlog:runlog.md");
    let staged = fs::read_to_string(&staged_path).expect("read staged runlog");

    fill_commit(repo.path(), home.path(), task);
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize` (cold)");

    // Promoted at its canonical singleton path, byte-stable vs the staged source.
    let committed = committed_runlog(repo.path());
    assert_eq!(
        committed, staged,
        "the cold-promoted singleton is byte-stable (committed == staged source)",
    );
    assert!(
        committed.contains("The cold-created running log."),
        "the authored overview prose is committed; got:\n{committed}",
    );
}

#[test]
fn warm_recreate_copies_in_appends_and_repromotes_byte_stable() {
    // (b) Warm re-create: a SECOND task over the already-committed singleton. `doc
    // create runlog` copies the committed body in (prior content preserved — no
    // clobber-on-blank), `doc add-item` appends an entry into the repeatable section,
    // and finalize re-promotes byte-stable, the cold overview prose intact.
    let repo = TempDir::new("warm");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("pack");
    seed_fixture_pack(pack.path());
    list_fixture_pack(repo.path(), pack.path());

    // ── Run 1: cold-create + finalize (records the committed baseline) ───────────
    let cold = "open-the-running-log";
    start_keep_runlog(repo.path(), home.path(), "open the running log", cold);
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["create", "runlog", "--title", "runlog"],
            None,
        ),
        "`doc create runlog` (warm/run1 cold)",
    );
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["set-slot", "runlog:runlog#overview", "--from-file", "-"],
            Some(b"The original running log.\n"),
        ),
        "`set-slot overview` (warm/run1)",
    );
    fill_commit(repo.path(), home.path(), cold);
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", cold]),
        "`task finalize` (warm/run1 cold)",
    );

    // ── Run 2: a second task warm-re-creates the same committed singleton ─────────
    let warm = "append-to-the-running-log";
    start_keep_runlog(repo.path(), home.path(), "append to the running log", warm);

    // The warm `create` copies the committed body in rather than minting blank — the
    // prior overview prose survives in the staged copy (the B-5 clobber fix).
    let recreate = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "runlog", "--title", "runlog"],
        None,
    );
    assert_ok(&recreate, "`doc create runlog` (warm/run2)");
    let staged_path = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(warm)
        .join("docs")
        .join("runlog:runlog.md");
    let after_copy_in = fs::read_to_string(&staged_path).expect("read warm-staged runlog");
    assert!(
        after_copy_in.contains("The original running log."),
        "the warm create copies the committed body in (prior content preserved); got:\n{after_copy_in}",
    );

    // The copy-in records `edited-from-base`, NOT `created` — the warm append edits an
    // existing committed singleton.
    let provenance = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(warm)
            .join("docs")
            .join("provenance.json"),
    )
    .expect("read warm provenance");
    assert!(
        provenance.contains("\"runlog:runlog\": \"edited-from-base\""),
        "a warm singleton re-create records `edited-from-base`; got:\n{provenance}",
    );

    // `add-item` appends an entry into the repeatable section (the shipped verb, which
    // already rides `read_or_copy_in`).
    let item = jigc_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            "runlog:runlog#entries",
            "--title",
            "Second pass",
        ],
        None,
    );
    assert_ok(&item, "`doc add-item runlog:runlog#entries` (warm)");
    let item_addr = String::from_utf8(item.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(
        item_addr, "runlog:runlog#entries/second-pass",
        "add-item prints the minted item address",
    );
    // Author the appended item's `note` slot (the item-leaf slot address).
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &[
                "set-slot",
                "runlog:runlog#entries/second-pass/note",
                "--from-file",
                "-",
            ],
            Some(b"The appended entry prose.\n"),
        ),
        "`set-slot` on the appended item (warm)",
    );

    let staged = fs::read_to_string(&staged_path).expect("read warm-staged runlog after append");

    fill_commit(repo.path(), home.path(), warm);
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", warm]),
        "`task finalize` (warm/run2)",
    );

    // Re-promoted byte-stable, with the prior overview prose AND the appended entry.
    let committed = committed_runlog(repo.path());
    assert_eq!(
        committed, staged,
        "the warm-re-promoted singleton is byte-stable (committed == staged source)",
    );
    assert!(
        committed.contains("The original running log."),
        "the warm re-promote preserves the prior overview prose; got:\n{committed}",
    );
    assert!(
        committed.contains("The appended entry prose."),
        "the warm re-promote carries the appended entry; got:\n{committed}",
    );
}

#[test]
fn warm_edit_over_an_oob_drifted_singleton_conflict_blocks_at_finalize() {
    // (c) The warm-drift spike. The cold finalize records the committed baseline FIRST;
    // a second task copies-in + edits (TOUCHED), then the committed runlog/runlog.md is
    // drifted OUT-OF-BAND (DRIFTED vs the recorded baseline). finalize-preflight's
    // `reconcile_committed_store` sees DRIFTED+TOUCHED and conflict-blocks — both sides
    // moved, no silent merge. (An unrecorded baseline would baseline-adopt vacuously;
    // recording-first is what makes this a genuine red.)
    let repo = TempDir::new("drift");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("pack");
    seed_fixture_pack(pack.path());
    list_fixture_pack(repo.path(), pack.path());

    // ── Run 1: cold-create + finalize → records the committed baseline ───────────
    let cold = "open-the-running-log";
    start_keep_runlog(repo.path(), home.path(), "open the running log", cold);
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["create", "runlog", "--title", "runlog"],
            None,
        ),
        "`doc create runlog` (drift/run1 cold)",
    );
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["set-slot", "runlog:runlog#overview", "--from-file", "-"],
            Some(b"The baseline running log.\n"),
        ),
        "`set-slot overview` (drift/run1)",
    );
    fill_commit(repo.path(), home.path(), cold);
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", cold]),
        "`task finalize` (drift/run1 cold) — records the baseline FIRST",
    );

    // OOB drift: a human hand-edits the committed runlog/runlog.md and commits it.
    // The cold finalize recorded the baseline hash in the file-state record; this raw
    // `git commit` advances HEAD + the on-disk bytes but does NOT touch that record, so
    // the recorded baseline now diverges from disk (DRIFTED). Drift BEFORE the warm task
    // starts so the warm task's base == this drifted HEAD (no `finalize.base-mismatch`,
    // which would pre-empt the reconcile gate). This rides the existing reconcile
    // machinery — no reconcile code is added by this task.
    let committed_path = repo.path().join("docs").join("runlog").join("runlog.md");
    let on_disk = fs::read_to_string(&committed_path).expect("read committed runlog on disk");
    fs::write(
        &committed_path,
        on_disk.replace(
            "The baseline running log.",
            "An out-of-band human edit to the running log.",
        ),
    )
    .expect("apply OOB drift");
    git(repo.path(), &["add", "docs/runlog/runlog.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "oob: hand-edit the runlog"],
    );

    // ── Run 2: a warm task touches the singleton (copy-in via create + edit) ──────
    let warm = "drift-the-running-log";
    start_keep_runlog(repo.path(), home.path(), "drift the running log", warm);
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["create", "runlog", "--title", "runlog"],
            None,
        ),
        "`doc create runlog` (drift/run2 warm copy-in)",
    );
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["set-slot", "runlog:runlog#overview", "--from-file", "-"],
            Some(b"The task-edited overview.\n"),
        ),
        "`set-slot overview` (drift/run2 — the task TOUCHED the singleton)",
    );

    // finalize-preflight: DRIFTED (committed != recorded baseline) + TOUCHED (staged in
    // the warm area) → conflict-block, non-zero exit, NO new commit.
    fill_commit(repo.path(), home.path(), warm);
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", warm]);
    assert!(
        !out.status.success(),
        "a DRIFTED+TOUCHED singleton must conflict-block finalize non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        rendered.contains("reconciliation.conflict-block"),
        "the block is the `reconciliation.conflict-block` finding; got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a conflict-block creates no commit");

    // ── The route belongs to the CALLER (M47 inc-2 / T4) — the task-scope emission seam.
    // The classifier has no task; the task-scope caller does, so the emitted route names
    // the REAL task id. No `<task-id>` placeholder survives the print, and the emitted
    // argv — lifted from the printed bytes and run VERBATIM — resolves that task.
    assert!(
        !rendered.contains("<task-id>"),
        "a blocking finding's mechanical route carries no unsubstituted placeholder \
         (the M43 route floor); got:\n{rendered}",
    );
    let argv = lifted_route_argv(&rendered, "jigc task discard");
    assert_eq!(
        argv,
        vec!["jigc", "task", "discard", warm, "--force"],
        "the emitted route names the real task id",
    );
    let discarded = jigc(
        repo.path(),
        home.path(),
        &argv[1..].iter().map(String::as_str).collect::<Vec<_>>(),
    );
    assert_ok(&discarded, "the emitted conflict route, run verbatim");
    assert!(
        !repo.path().join(".jigc").join("tasks").join(warm).is_dir(),
        "the emitted route resolved the real task — its working area is gone",
    );
}

#[test]
fn non_singleton_create_over_an_off_canonical_committed_body_mints_fresh() {
    // (d) The copy-in keys on the slug's CANONICAL committed path only (revised at
    // M43 inc-7 T1: the copy-in is doctype-blind — a committed instance at the
    // canonical home IS copied in for update, `create_or_update.rs`). This arm pins
    // the boundary: a committed body at an OFF-canonical path (here `decisions/`,
    // while the default `docs-root: docs/` homes the adr at `docs/decisions/`) is
    // not this slug's committed instance, so the create mints fresh and a re-create
    // serial-rejects — copy-in never reads a body from a path the schema does not
    // own. The fixture pack's `keep-runlog` create-gate does NOT admit `adr`, so the
    // test runs the embedded `single-task` workflow (no fixture pack listed).
    let repo = TempDir::new("regress");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // A committed ADR body at an OFF-canonical path (`decisions/`, not the
    // docs-root-nested `docs/decisions/` the adr schema owns) — the create must
    // not read it as this slug's committed instance.
    fs::create_dir_all(repo.path().join("decisions")).expect("mk decisions/");
    fs::write(
        repo.path().join("decisions").join("single-node-cache.md"),
        "---\nstatus: accepted\ndate: 2026-05-23\n---\n\n# Single-node cache\n\n\
         ## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nThe ORIGINAL committed decision.\n\n\
         ## Consequences\n\nNone.\n",
    )
    .expect("write committed adr");
    git(repo.path(), &["add", "decisions/single-node-cache.md"]);
    git(repo.path(), &["commit", "-q", "-m", "commit the prior adr"]);

    // Start a `single-task` (embedded pack) and `create adr --title "Single node
    // cache"` — the slug matches the off-canonical committed body's stem.
    let task = "supersede-the-cache";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "supersede the cache"],
        ),
        "`jigc start --workflow single-task` (regression)",
    );

    // The off-canonical committed body is NOT a copy-in source: the slug's canonical
    // home (`docs/decisions/single-node-cache.md`) is empty, so the create mints the
    // fresh skeleton. Prove the mint did NOT pull in the off-home prose.
    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Single node cache"],
        None,
    );
    assert_ok(&create, "`doc create adr` (off-canonical body, fresh mint)");
    let staged = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(task)
            .join("docs")
            .join("adr:single-node-cache.md"),
    )
    .expect("read staged adr");
    assert!(
        !staged.contains("The ORIGINAL committed decision."),
        "a create must NOT copy an off-canonical body in (copy-in keys on the \
         canonical committed path only); got:\n{staged}",
    );

    // A second `doc create` of the same slug is the agent-initiated create-gate over the
    // same-identity staged copy: it acks `existed` and binds the role (copied-in for
    // update), never rejects — the repairing action stays open. The gated serial-collision
    // reject routed the agent away from the only repairing action (M45 Inc 5 T2;
    // `DECISIONS.md` 2026-07-23 M45 planning → Fork 2). The *ungated* serial-mint reject
    // (fan-out) is untouched; only this agent-initiated gate acks-existed.
    let recreate = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Single node cache"],
        None,
    );
    assert_ok(
        &recreate,
        "a second gated `doc create adr` acks existed and binds, never rejects",
    );
    let stdout = String::from_utf8_lossy(&recreate.stdout);
    assert!(
        stdout.contains("already existed"),
        "the re-create acks `already existed` (copied in for update); got:\n{stdout}",
    );
}
