//! M43 Inc 1 T7 — the anyhow-embedded route rewrite, bounded to shipped verbs' error
//! strings that carry `` `jigc …` `` command spans (`design/surface-contract.md` → The
//! route fence, closing paragraph: an errorish surface is still a surface; prose-only
//! error text untouched).
//!
//! Every rewritten site builds its span through the checked
//! `engine::finding::Route::mechanical` constructor, so the CLI-seam parse fence (T2)
//! asserts the span parses against the real CLI at construction. The fence is
//! debug-posture and `main` installs it, so the binary these arms drive carries it
//! **live**: a rewritten span that stopped parsing panics the construction (exit 101,
//! never the asserted clean exit 1) — each arm therefore proves both the emitted
//! message bytes *and* that its spans passed the fence.
//!
//! One arm per rewritten family, asserting the **emitted stderr bytes** (the emitted
//! bytes are the contract). Four messages changed under the fence; the rest are pinned
//! goldens:
//!
//! - the absent-staged-instance provision hint names the **full, parseable**
//!   `doc create` form (the bare `jigc doc create` never parsed — a required arg short);
//! - the milestone-discard wrong-id route carries the declared `<milestone-id>`
//!   placeholder (was the undeclared, ambiguous `<id>`);
//! - the sub-task re-seed route names the **full** `milestone add-task` form (bare
//!   `jigc milestone add-task` never parsed);
//! - the doc-verb no-recorded-workflow reject names the **concrete** task id in its
//!   discard span (was the undeclared `<id>` — the style guide's "the exact next
//!   command for the state at hand").
//!
//! Out of scope, per the planner bound: T5's converged wrong-task-id sites
//! (`no_such_task_route.rs`), the hook-rejection verbatim-stderr channel
//! (`finalize.md:78`), prose-only error text, and `Finding` routes (the T3 floor).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-anyhow-route-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Initialize a real git repo with one commit — **without** the `.jigc/config/`
/// project layer (the not-set-up arm needs its absence).
fn init_repo_bare(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    init_repo_bare(repo);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Assert `out` is a **clean operational error** (exit 1 — never a fence panic's 101)
/// whose stderr is exactly `expected`.
fn assert_error_bytes(out: &std::process::Output, expected: &str) {
    assert_eq!(
        out.status.code(),
        Some(1),
        "expected a clean operational error (a fence panic would exit 101); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    assert_eq!(stderr, expected, "the emitted error bytes are the contract");
}

/// Like [`assert_error_bytes`] but for messages carrying run-dependent values
/// (temp paths, shas): asserts exit 1 + every `needles` substring present.
fn assert_error_contains(out: &std::process::Output, needles: &[&str]) {
    assert_eq!(
        out.status.code(),
        Some(1),
        "expected a clean operational error (a fence panic would exit 101); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    for needle in needles {
        assert!(
            stderr.contains(needle),
            "stderr must carry {needle:?}; got:\n{stderr}",
        );
    }
}

/// Mint a top-level `single-task` task with a controlled slug.
fn mint_task(repo: &Path, home: &Path, slug: &str) {
    mint_task_on(repo, home, "single-task", slug);
}

/// Mint a top-level task on a **named** workflow with a controlled slug — the
/// create-gate arm needs both a granting workflow (`single-task`, `allows-create:
/// [adr, changelog]`) and a forbidding one (`quick-fix`, `allows-create: []`).
fn mint_task_on(repo: &Path, home: &Path, workflow: &str, slug: &str) {
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            workflow,
            "--slug",
            slug,
            "Do the thing",
        ],
    );
    assert!(
        out.status.success(),
        "minting `{slug}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Mint a milestone + one `single-task` sub-task (`do-the-thing`) under it.
fn mint_subtask(repo: &Path, home: &Path) -> &'static str {
    let created = jigc(repo, home, &["milestone", "create", "Rework"]);
    assert!(
        created.status.success(),
        "`milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );
    let added = jigc(
        repo,
        home,
        &[
            "milestone",
            "add-task",
            "rework",
            "Do the thing",
            "--workflow",
            "single-task",
        ],
    );
    assert!(
        added.status.success(),
        "`add-task` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&added.stderr),
    );
    "do-the-thing"
}

// ── the not-set-up family (six sites, one shared helper) ────────────────────────────

/// Every project-layer-requiring verb emits the **one** not-set-up rejection with the
/// `jigc setup` route — the six formerly-duplicated `bail!` sites converge on the
/// shared `locate::not_set_up` constructor, whose span rides the parse fence.
#[test]
fn not_set_up_family_emits_the_one_setup_route() {
    let repo = TempDir::new("not-set-up");
    let home = TempDir::new("home");
    init_repo_bare(repo.path());

    const EXPECTED: &str =
        "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)\n";
    let arms: &[&[&str]] = &[
        &["describe"],
        &["ingest"],
        &["migrate", "README.md", "--as", "changelog"],
        &["migrate-corpus"],
        &["upgrade"],
        &["start", "Do something"],
    ];
    for args in arms {
        let out = jigc(repo.path(), home.path(), args);
        assert_error_bytes(&out, EXPECTED);
    }
}

// ── the no-active-task reject (`doc.rs` → `ActiveTask::resolve`, zero tasks) ────────

/// A doc write verb with no active task routes to `jigc start` — the span rides the
/// checked constructor (distinct from the T5 wrong-id state, which routes `task list`).
#[test]
fn no_active_task_routes_to_start() {
    let repo = TempDir::new("no-active");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "set-field", "adr:x#status", "--value", "accepted"],
    );
    assert_error_bytes(&out, "no active task — start one with `jigc start`\n");
}

// ── the malformed-address reject (`doc.rs` → `parse_verb_addr`) ─────────────────────

/// A malformed doc address keeps its example + the `jigc describe` route, the span
/// riding the checked constructor.
#[test]
fn malformed_doc_address_routes_to_describe() {
    let repo = TempDir::new("bad-addr");
    let home = TempDir::new("home");
    init_repo(repo.path());
    mint_task(repo.path(), home.path(), "my-task");

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "bad-addr",
            "--value",
            "x",
            "--task",
            "my-task",
        ],
    );
    assert_error_bytes(
        &out,
        "malformed address `bad-addr`: missing ':' between type and slug — a doc is \
         addressed as `<type>:<slug>`, e.g. `adr:single-node-cache` (a singleton doctype \
         like `changelog` or `vision` may be named bare)\n  route: run `jigc describe` \
         for the doctype surface\n",
    );
}

// ── the absent-staged-instance reject (`doc.rs` → `read_staged`) — bytes CHANGED ────

/// The provision hint names the **full, parseable** `doc create` form: the old bare
/// `` `jigc doc create` `` span never parsed (required args short) — the parse fence
/// forces the honest, copy-adaptable form.
#[test]
fn absent_staged_instance_names_the_full_create_form() {
    let repo = TempDir::new("no-instance");
    let home = TempDir::new("home");
    init_repo(repo.path());
    mint_task(repo.path(), home.path(), "my-task");

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "add-item",
            "adr:ghost#options",
            "--title",
            "An option",
            "--task",
            "my-task",
        ],
    );
    assert_error_bytes(
        &out,
        "no staged instance for `adr:ghost#options` — provision it first (`jigc start` / \
         `jigc doc create <type> --title 'X'`). Note: `jigc doc create <type> --title 'X'` \
         derives the id from the title (`X` → slug), not the task id — address writes at \
         that title-derived id\n",
    );
}

// ── the absent-instance refusal's create offer is gate-aware (M46 Inc 8, B2-1) ──────

/// Lift every backticked `` `jigc …` `` span from a printed refusal, in order and
/// deduplicated — the surface's *printed routes*, which is what a reader runs.
fn jigc_spans(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for span in text.split('`').skip(1).step_by(2) {
        if span.starts_with("jigc ") && !out.iter().any(|s| s == span) {
            out.push(span.to_owned());
        }
    }
    out
}

/// Split a printed span into argv, honouring the single quoting the route fence
/// mandates (`--title 'X'` is one argument).
fn span_argv(span: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    for ch in span.chars() {
        match ch {
            '\'' => quoted = !quoted,
            c if c.is_whitespace() && !quoted => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Run one lifted span from the state that raised it and assert it does **not** answer
/// `create.gate-blocked` — the B2-1 defect: a refusal that routes to a create the task's
/// own `allows-create:` gate refuses.
fn assert_span_is_not_gate_blocked(repo: &Path, home: &Path, argv: &[String], cell: &str) {
    let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    let out = jigc(repo, home, &args);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("create.gate-blocked"),
        "{cell}: the refusal's route `{}` answers create.gate-blocked:\n{stderr}",
        argv.join(" "),
    );
}

/// The absent-instance refusal never routes to a create this task cannot run.
///
/// Drives the 2×2 cell set {the gate admits the addressed doctype, it forbids it} ×
/// {a `commit:<task>` address, a non-commit address}. The commit column collapses —
/// `commit` sits in **no** shipped workflow's `allows-create:` (it is
/// workflow-provisioned and *bypasses* the gate, `design/write-commands.md` → The
/// create-gate) — so both workflows must answer the same thing there, and the test
/// asserts that collapse rather than assuming it.
///
/// Each cell's emitted bytes are asserted, then **every** backticked `jigc …` span is
/// lifted and run from the raising state: verbatim, and — when the span carries the
/// `<type>` placeholder, which stands for the doctype the reader is addressing — again
/// with that placeholder substituted, because a placeholder run literally answers
/// `unknown doctype` and would mask the defect. No run may answer `create.gate-blocked`.
struct Cell {
    /// The cell's label in a failure message.
    tag: &'static str,
    /// The workflow the task is minted on — its `allows-create:` is the gate axis.
    workflow: &'static str,
    /// The task id (and its controlled slug).
    task: &'static str,
    /// The absent address the write verb addresses — its `<type>` head is the doctype axis.
    address: &'static str,
    /// The write verb that raises the refusal at `address`.
    write: &'static [&'static str],
    /// The refusal's emitted stderr bytes — the contract.
    expected: &'static str,
}

#[test]
fn absent_instance_refusal_never_routes_to_a_forbidden_create() {
    let cells = [
        Cell {
            tag: "admits × non-commit",
            workflow: "single-task",
            task: "st",
            address: "adr:ghost#options",
            write: &[
                "doc",
                "add-item",
                "adr:ghost#options",
                "--title",
                "An option",
            ],
            expected: "no staged instance for `adr:ghost#options` — provision it first (`jigc start` / \
             `jigc doc create <type> --title 'X'`). Note: `jigc doc create <type> --title 'X'` \
             derives the id from the title (`X` → slug), not the task id — address writes at \
             that title-derived id\n",
        },
        Cell {
            tag: "forbids × non-commit",
            workflow: "quick-fix",
            task: "qf",
            address: "adr:ghost#options",
            write: &[
                "doc",
                "add-item",
                "adr:ghost#options",
                "--title",
                "An option",
            ],
            expected: "no staged instance for `adr:ghost#options` — task `qf`'s workflow grants no \
             in-task create for `adr` (its `allows-create:` gate lists []), so nothing in this \
             task provisions it; create `adr` from a task minted on a workflow that grants it \
             (`jigc start` lists the catalog)\n",
        },
        Cell {
            tag: "admits-other × commit",
            workflow: "single-task",
            task: "st",
            address: "commit:ghost#type",
            write: &["doc", "set-field", "commit:ghost#type", "--value", "feat"],
            expected: "no staged instance for `commit:ghost#type` — task `st`'s workflow provisions its \
             `commit` doc at compose and grants no in-task create for it; list what task `st` \
             stages with `jigc doc list --task st`\n",
        },
        Cell {
            tag: "forbids × commit",
            workflow: "quick-fix",
            task: "qf",
            address: "commit:ghost#type",
            write: &["doc", "set-field", "commit:ghost#type", "--value", "feat"],
            expected: "no staged instance for `commit:ghost#type` — task `qf`'s workflow provisions its \
             `commit` doc at compose and grants no in-task create for it; list what task `qf` \
             stages with `jigc doc list --task qf`\n",
        },
    ];

    for Cell {
        tag,
        workflow,
        task,
        address,
        write,
        expected,
    } in cells
    {
        let repo = TempDir::new("gate-aware");
        let home = TempDir::new("home");
        init_repo(repo.path());
        mint_task_on(repo.path(), home.path(), workflow, task);

        let doctype = address.split(':').next().expect("a typed address");
        let mut argv = write.to_vec();
        argv.extend_from_slice(&["--task", task]);
        let out = jigc(repo.path(), home.path(), &argv);
        assert_eq!(
            out.status.code(),
            Some(1),
            "{tag}: expected a clean operational error; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
        assert_eq!(
            stderr, expected,
            "{tag}: the emitted bytes are the contract"
        );

        let spans = jigc_spans(&stderr);
        assert!(!spans.is_empty(), "{tag}: a refusal must print a route");
        for span in &spans {
            let argv = span_argv(span);
            assert_span_is_not_gate_blocked(repo.path(), home.path(), &argv, tag);
            if argv.iter().any(|a| a == "<type>") {
                let substituted: Vec<String> = argv
                    .iter()
                    .map(|a| {
                        if a == "<type>" {
                            doctype.to_owned()
                        } else {
                            a.clone()
                        }
                    })
                    .collect();
                assert_span_is_not_gate_blocked(repo.path(), home.path(), &substituted, tag);
            }
        }
    }
}

/// The two claims the gate-aware refusal makes about *this* state are true here:
/// composing really does provision the commit doc the `commit` cell names, and the
/// catalog the forbidding cell points at really does list a workflow that grants `adr`.
#[test]
fn the_gate_aware_refusal_routes_land_where_they_claim() {
    let repo = TempDir::new("gate-aware-truth");
    let home = TempDir::new("home");
    init_repo(repo.path());
    mint_task_on(repo.path(), home.path(), "quick-fix", "qf");

    let listed = jigc(repo.path(), home.path(), &["doc", "list", "--task", "qf"]);
    assert!(listed.status.success(), "`doc list --task qf` must exit 0");
    let stdout = String::from_utf8_lossy(&listed.stdout);
    assert!(
        stdout.contains("commit:qf"),
        "composing must provision the commit doc the refusal names; got:\n{stdout}",
    );

    let catalog = jigc(repo.path(), home.path(), &["start"]);
    assert!(catalog.status.success(), "`jigc start` must exit 0");
    let catalog = String::from_utf8_lossy(&catalog.stdout);
    assert!(
        catalog.contains("record-decision"),
        "the catalog must list a workflow granting `adr`; got:\n{catalog}",
    );
}

// ── the migrate rejects (`migrate.rs`) ──────────────────────────────────────────────

/// A non-migratable doctype routes back to the verb with the live set; the
/// placeholder-carrying span rides the checked constructor (the `<path>` placeholder
/// joins the declared dummy table).
#[test]
fn not_migratable_doctype_routes_back_to_migrate() {
    let repo = TempDir::new("bad-doctype");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["migrate", "README.md", "--as", "bogus"],
    );
    assert_error_contains(
        &out,
        &[
            "unknown doctype `bogus`; migratable doctypes: ",
            "\n  route: re-run `jigc migrate <path> --as <doctype>` with one of: ",
        ],
    );
}

/// An unreadable foreign source routes back to the verb, the runtime doctype riding
/// the span's argv (a real value parses like any value).
#[test]
fn unreadable_foreign_source_routes_back_to_migrate() {
    let repo = TempDir::new("unreadable");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["migrate", "missing.md", "--as", "changelog"],
    );
    assert_error_contains(
        &out,
        &[
            "could not read the foreign `changelog` source at ",
            "\n  route: check the path, then re-run `jigc migrate <path> --as changelog` \
             with a readable file",
        ],
    );
}

// ── the unknown-milestone rejects (`milestone.rs`) ──────────────────────────────────

/// An unknown milestone id routes to minting it — the four formerly-duplicated sites
/// converge on one shared constructor, its quoted-title span riding the fence.
#[test]
fn unknown_milestone_routes_to_create() {
    let repo = TempDir::new("no-milestone");
    let home = TempDir::new("home");
    init_repo(repo.path());

    const EXPECTED: &str = "milestone `nope` does not exist\n  route: create it first \
                            with `jigc milestone create \"<title>\"`\n";
    for args in [
        ["milestone", "list-tasks", "nope"],
        ["milestone", "execute", "nope"],
    ] {
        let out = jigc(repo.path(), home.path(), &args);
        assert_error_bytes(&out, EXPECTED);
    }
}

/// The discard wrong-id variant carries the **declared** `<milestone-id>` placeholder
/// (was the undeclared, ambiguous `<id>` — the dummy table is the declared set).
#[test]
fn milestone_discard_wrong_id_names_list_tasks_with_the_declared_placeholder() {
    let repo = TempDir::new("discard-wrong");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["milestone", "discard", "nope"]);
    assert_error_bytes(
        &out,
        "milestone `nope` does not exist\n  route: check the milestone id \
         (`jigc milestone list-tasks <milestone-id>` names a live milestone's sub-tasks); \
         nothing was discarded\n",
    );
}

// ── the frozen-doctype relocate reject (`relocate.rs`) ──────────────────────────────

/// A frozen doctype's relocate refusal routes to the version-gated verb, the span
/// riding the checked constructor.
#[test]
fn frozen_doctype_relocate_routes_to_migrate_corpus() {
    let repo = TempDir::new("relocate-frozen");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["relocate", "changelog", "--from", "docs/"],
    );
    assert_error_bytes(
        &out,
        "`changelog` is a frozen doctype — relocate it through the version-gated \
         `jigc migrate-corpus`, not the freeze-exempt path\n",
    );
}

// ── the rename rejects (`rename.rs`) ────────────────────────────────────────────────

/// Both rename rejects — the malformed address and the missing doc — keep their
/// `jigc describe` route, the spans riding the checked constructor.
#[test]
fn rename_rejects_route_to_describe() {
    let repo = TempDir::new("rename");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let malformed = jigc(repo.path(), home.path(), &["rename", "bad", "--to", "X"]);
    assert_error_bytes(
        &malformed,
        "`bad` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`\n  \
         route: run `jigc describe` for the doctype surface\n",
    );

    let missing = jigc(
        repo.path(),
        home.path(),
        &["rename", "adr:nope", "--to", "X"],
    );
    assert_error_bytes(
        &missing,
        "no managed doc `adr:nope` to rename (expected at docs/decisions/nope.md)\n  \
         route: check the id (or run `jigc describe` for the doctype surface)\n",
    );
}

// ── the pinned-base mismatch (`start.rs`, the sub-agent re-entry form) ──────────────

/// The re-entry door's blanket base-pin refusal, over the **unit-kind axis** its route
/// splits on — every span riding the checked constructor with the runtime id in its argv.
///
/// **Retargeted at M47 Inc 8 T1 (N7):** this arm drove the `jigc start --task` *resume*
/// door, whose blanket `base != HEAD` refusal is gone — resume now makes the commit
/// door's overlap-aware `decide_base_repin` decision and blocks on a `Finding` route
/// (the T3 floor, out of this file's scope; pinned in `start_resume.rs`). The **re-entry**
/// door is the declared non-goal that keeps the blanket refusal — its commit boundary is
/// the consciously-strict `plan_milestone_finalize` — so it is where this anyhow-embedded
/// span still lives.
///
/// **Re-split at M46 Inc 8 (B2-2):** the door takes no membership decision, so both unit
/// kinds reach it, and the recovery differs by kind. A **sub-task** routes at the worktree
/// its work happens in (`jigc milestone provision <m>`) — the discard it used to offer
/// exits 0 while the milestone's committed record still calls the sub-task active. A
/// **top-level task** keeps `git checkout` (not a jigc span — untouched prose) and its own
/// `jigc task discard <id>`, which really is its teardown. The full sub-task behaviour —
/// both read doors, the emitted span run verbatim, the record left standing — is pinned in
/// `start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal`.
#[test]
fn pinned_base_mismatch_routes_by_unit_kind() {
    let repo = TempDir::new("pinned");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let sub = mint_subtask(repo.path(), home.path());
    mint_task(repo.path(), home.path(), "top-task");

    fs::write(repo.path().join("more.md"), "more\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "second"]);

    let sub_out = jigc(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", sub],
    );
    assert_error_contains(
        &sub_out,
        &[
            &format!("task `{sub}` is pinned to base "),
            &format!(
                " — this is a sub-task of milestone `rework`, and a sub-task's work happens in its own worktree at .jigc/worktrees/{sub}, "
            ),
            "run `jigc milestone provision rework` — it adds a worktree that is missing and leaves one that exists untouched — then re-run this from that worktree\n",
        ],
    );
    assert!(
        !String::from_utf8_lossy(&sub_out.stderr).contains(&format!("jigc task discard {sub}")),
        "a sub-task must not be routed at a discard the milestone's record contradicts",
    );

    let top_out = jigc(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", "top-task"],
    );
    assert_error_contains(
        &top_out,
        &[
            "task `top-task` is pinned to base ",
            " — switch back with `git checkout ",
            "` or `jigc task discard top-task`\n",
        ],
    );
}

// ── the missing-workflow-record family (four sites, three verbs) ────────────────────

/// A task working area whose recorded workflow is gone rejects with the discard +
/// re-start/re-seed routes — every span riding the checked constructor:
/// the resume form keeps its bytes; the sub-task re-entry names the **full**
/// `milestone add-task` form (the bare span never parsed); the doc-verb form names
/// the **concrete** task id (was the undeclared `<id>`); `task bind` keeps its
/// span-less discard prose + the `jigc start` span.
#[test]
fn missing_workflow_record_family_routes_to_discard_and_restart() {
    // Resume (`jigc start --task`) + the doc verb + `task bind` — a top-level task.
    let repo = TempDir::new("no-workflow");
    let home = TempDir::new("home");
    init_repo(repo.path());
    mint_task(repo.path(), home.path(), "my-task");
    fs::remove_file(repo.path().join(".jigc/tasks/my-task/workflow"))
        .expect("remove the workflow record");

    let resume = jigc(repo.path(), home.path(), &["start", "--task", "my-task"]);
    assert_error_bytes(
        &resume,
        "task `my-task` has no recorded workflow — discard it with \
         `jigc task discard my-task` and re-start with `jigc start`\n",
    );

    let doc_verb = jigc(
        repo.path(),
        home.path(),
        &["doc", "create", "adr", "--title", "T", "--task", "my-task"],
    );
    assert_error_bytes(
        &doc_verb,
        "the active task has no recorded workflow — discard it with \
         `jigc task discard my-task` and re-start with `jigc start`\n",
    );

    let bind = jigc(
        repo.path(),
        home.path(),
        &["task", "bind", "decision", "adr:x", "my-task"],
    );
    assert_error_contains(
        &bind,
        &["has no recorded workflow — discard it and re-start with `jigc start`"],
    );

    // Sub-task re-entry (`jigc workflow <W> --task`) — a milestone sub-task.
    let repo2 = TempDir::new("no-workflow-sub");
    let home2 = TempDir::new("home");
    init_repo(repo2.path());
    let sub = mint_subtask(repo2.path(), home2.path());
    fs::remove_file(repo2.path().join(format!(".jigc/tasks/{sub}/workflow")))
        .expect("remove the workflow record");

    let reentry = jigc(
        repo2.path(),
        home2.path(),
        &["workflow", "single-task", "--task", sub],
    );
    assert_error_bytes(
        &reentry,
        "task `do-the-thing` has no recorded workflow — discard it with \
         `jigc task discard do-the-thing` and re-seed it with \
         `jigc milestone add-task <milestone-id> \"<intent>\"`\n",
    );
}

// ── the sub-task workflow-mismatch reject (`start.rs` → workflow re-entry) ──────────

/// A `jigc workflow <W>` re-entry whose `<W>` ≠ the sub-task's recorded workflow rejects
/// with the `workflow-refs.workflow-mismatch` block. Both `jigc workflow … --task …`
/// spans (the one the agent ran + the recorded re-run form) now ride the checked
/// constructor (F4 — the route-fence seam-sweep), so this arm — driving the binary with
/// the fence live — proves they parse (a non-parsing span would panic construction, exit
/// 101, never the asserted clean exit 1) and pins the emitted span bytes against drift.
#[test]
fn workflow_reentry_mismatch_names_both_workflow_spans() {
    let repo = TempDir::new("reentry-mismatch");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let sub = mint_subtask(repo.path(), home.path());

    // Re-enter the `single-task` sub-task naming a *different* workflow.
    let out = jigc(
        repo.path(),
        home.path(),
        &["workflow", "quick-fix", "--task", sub],
    );
    assert_error_contains(
        &out,
        &[
            "`jigc workflow quick-fix --task do-the-thing` names workflow `quick-fix`",
            "was minted with `single-task` — re-entry must compose the recorded workflow",
            "re-run as `jigc workflow single-task --task do-the-thing`",
        ],
    );
}

// ── the sub-task wrong-id reject (`start.rs` → workflow re-entry) ───────────────────

/// A `jigc workflow` re-entry naming no live sub-task routes to `milestone
/// list-tasks` — the `<milestone-id>` placeholder joins the declared dummy table.
#[test]
fn workflow_reentry_unknown_task_names_list_tasks() {
    let repo = TempDir::new("reentry-wrong");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["workflow", "sub-task", "--task", "nope"],
    );
    assert_error_bytes(
        &out,
        "no task `nope` — list a milestone's sub-tasks with \
         `jigc milestone list-tasks <milestone-id>`\n",
    );
}
