//! End-to-end integration test for `jigc task bind <role> <addr> <id>` — the
//! cross-task binding verb (`design/write-commands.md` → Binding a context role:
//! the five-step enforcement; `workflow-dialect.md` → `reads`).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo. A task is
//! minted over the `implement-from-spec` workflow (which declares
//! `reads: [{role: spec, type: spec}]`), a committed `spec` fixture lives at
//! `docs/specs/<slug>.md`, and `jigc task bind` is exercised across every rejection of
//! the five-step enforcement plus the success + re-bind (last-write-wins) path.
//! The observable proof is the persisted `.jigc/tasks/<id>/roles.json` mapping
//! `spec -> spec:<slug>`.
//!
//! The second test drives the **ack envelope** the two task-state verbs join at M42
//! (`design/command-output-contract.md` §2 → The task-state verbs join the envelope):
//! `task bind` and `task discard` mutate durable state, so each emits a JSON ack
//! (`op` + `task` + `findings`, plus `role`/`target` on bind) and a one-line plain ack —
//! and `task discard <absent-id>` still **exits 1** with the error envelope (the verb is
//! not idempotent: `TaskArea::resolve` bails first).
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init`, and a self-cleaning
//! `TempDir` keeps the test off the developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-task-bind-{tag}-{}-{:?}",
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

fn git(root: &Path, args: &[&str]) {
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
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project
/// layer so the cascade resolves (composition mints, which reads HEAD).
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Commit a `spec` fixture at its canonical path `docs/specs/<slug>.md` so the bind's
/// committed-store resolve (step 3) finds it. The body is a minimal conforming
/// spec — bind's resolve only needs the canonical file to exist.
fn commit_spec(root: &Path, slug: &str) {
    let specs = root.join("docs").join("specs");
    fs::create_dir_all(&specs).expect("create docs/specs/");
    fs::write(
        specs.join(format!("{slug}.md")),
        "# Cache the session store\n\n## Goal\n\nMove sessions to redis.\n",
    )
    .expect("write spec fixture");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "add spec"]);
}

/// Commit an `adr` fixture at its canonical path `docs/decisions/<slug>.md`. Used to
/// give the doctype-mismatch step (4) a target that resolves in the committed
/// store (step 3) yet has the wrong doctype for the `spec` role.
fn commit_adr(root: &Path, slug: &str) {
    let decisions = root.join("docs").join("decisions");
    fs::create_dir_all(&decisions).expect("create docs/decisions/");
    fs::write(
        decisions.join(format!("{slug}.md")),
        "# Some decision\n\n## Decision\n\nWe decided.\n",
    )
    .expect("write adr fixture");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "add adr"]);
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn task_bind_enforces_the_five_steps_and_records_the_binding() {
    let repo = TempDir::new("repo");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let slug = "cache-the-session-store";

    // Mint a task over the `implement-from-spec` workflow (declares
    // `reads: [{role: spec, type: spec}]`).
    let mint = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "implement the spec",
        ],
    );
    assert!(
        mint.status.success(),
        "the implement-from-spec mint must succeed; stderr:\n{}",
        stderr_of(&mint),
    );
    let task_id = "implement-the-spec";

    // Rejection 1 — no active task: bind against an id with no working area.
    let no_task = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            "ghost",
        ],
    );
    assert!(
        !no_task.status.success(),
        "binding against a nonexistent task must fail",
    );
    assert!(
        stderr_of(&no_task).contains("ghost"),
        "the no-active-task rejection must name the missing task; got:\n{}",
        stderr_of(&no_task),
    );

    // Rejection 2 — role not declared in the workflow's `reads`: the rejection
    // lists the declared roles so the agent sees what is bindable.
    let bad_role = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "design",
            "spec:cache-the-session-store",
            task_id,
        ],
    );
    assert!(
        !bad_role.status.success(),
        "binding an undeclared role must fail",
    );
    let bad_role_err = stderr_of(&bad_role);
    assert!(
        bad_role_err.contains("design") && bad_role_err.contains("spec"),
        "the undeclared-role rejection must name the bad role and list the declared ones (spec); \
         got:\n{bad_role_err}",
    );
    // …and it names itself and routes (M52 completion audit, fix 5). This refusal fires
    // **before** the two `decisions-pending.md`'s axis-6 lead 6a named — the ones M52
    // Increment 10 / T7 discharged — and was in no ledger: driven at `68d14cd3` it answered
    // a bare sentence with no code and no route, on text and inside `{"error": …}`.
    assert!(
        bad_role_err.contains("blocking · task-bind.undeclared-role")
            && bad_role_err.contains("route: "),
        "the undeclared-role rejection carries its code and a route (the route floor); \
         got:\n{bad_role_err}",
    );

    // Rejection 3 — `<addr>` does not resolve in the committed store (no spec
    // committed yet): the blocking `store.not-found` this door raises since M52
    // Increment 10 / T7 (it was a code-less, route-less `no such doc <addr>` bail).
    let no_doc = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            task_id,
        ],
    );
    assert!(
        !no_doc.status.success(),
        "binding an addr absent from the committed store must fail",
    );
    let no_doc_err = stderr_of(&no_doc);
    assert!(
        no_doc_err.contains("store.not-found")
            && no_doc_err.contains("no committed doc `spec:cache-the-session-store`"),
        "the unresolved-addr rejection must name the code and the address; got:\n{no_doc_err}",
    );

    // Now commit the spec fixture so the store resolve succeeds for the rest, plus
    // an ADR fixture so the doctype-mismatch step (4) is reached — its addr must
    // resolve in the committed store (step 3 passes) yet carry the wrong doctype.
    commit_spec(repo.path(), slug);
    commit_adr(repo.path(), "some-decision");
    // The task is pinned to its base; bind reads the committed store at HEAD, but
    // committing the spec advanced HEAD. Re-mint the task on the new HEAD so the
    // store resolve sees the committed spec. (bind itself does not touch the base
    // pin — the doctype/store checks read the committed store, not the task base.)
    run(
        repo.path(),
        home.path(),
        &["task", "discard", task_id, "--force"],
    );
    let remint = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "implement the spec",
        ],
    );
    assert!(
        remint.status.success(),
        "the re-mint on the spec-bearing HEAD must succeed; stderr:\n{}",
        stderr_of(&remint),
    );

    // Rejection 4 — doctype mismatch: the role `spec` declares type `spec`, but
    // the addr names an `adr`. The committed-store resolve and the declared-type
    // check disagree → reject with the mismatch.
    let mismatch = run(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", "adr:some-decision", task_id],
    );
    assert!(
        !mismatch.status.success(),
        "binding an addr of the wrong doctype must fail",
    );
    let mismatch_err = stderr_of(&mismatch);
    assert!(
        mismatch_err.contains("adr") && mismatch_err.contains("spec"),
        "the doctype-mismatch rejection must name both the target type (adr) and the declared \
         one (spec); got:\n{mismatch_err}",
    );
    // The step-2 sibling's other half (M52 completion audit, fix 5): the code, and a
    // **mechanical** route, because the declared type is in hand and `jigc doc list <ty>`
    // is the surface that answers *which docs could I have bound?*.
    assert!(
        mismatch_err.contains("blocking · task-bind.role-type-mismatch")
            && mismatch_err.contains("route: `jigc doc list spec`"),
        "the doctype-mismatch rejection carries its code and the runnable route; \
         got:\n{mismatch_err}",
    );

    // Success — a committed spec at the declared role + matching doctype binds.
    let ok = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            task_id,
        ],
    );
    assert!(
        ok.status.success(),
        "binding a committed spec to the declared `spec` role must succeed; stderr:\n{}",
        stderr_of(&ok),
    );

    // The binding landed on disk: roles.json maps `spec -> spec:<slug>`.
    let roles_path = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task_id)
        .join("roles.json");
    let roles = fs::read_to_string(&roles_path).expect("roles.json written by bind");
    assert!(
        roles.contains("\"spec\": \"spec:cache-the-session-store\""),
        "roles.json must bind spec -> the committed spec; got:\n{roles}",
    );

    // Re-bind overwrites (last-write-wins). Commit a second spec, re-mint on that
    // HEAD, and re-bind the same role to the new addr.
    commit_spec(repo.path(), "rate-limit-the-api");
    run(
        repo.path(),
        home.path(),
        &["task", "discard", task_id, "--force"],
    );
    run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "implement the spec",
        ],
    );
    // First bind to spec A, then re-bind to spec B.
    let first = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            task_id,
        ],
    );
    assert!(first.status.success(), "first bind must succeed");
    let rebind = run(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", "spec:rate-limit-the-api", task_id],
    );
    assert!(
        rebind.status.success(),
        "the re-bind must succeed; stderr:\n{}",
        stderr_of(&rebind),
    );
    let roles = fs::read_to_string(&roles_path).expect("roles.json after re-bind");
    assert!(
        roles.contains("\"spec\": \"spec:rate-limit-the-api\""),
        "the re-bind must overwrite (last-write-wins); got:\n{roles}",
    );
    assert!(
        !roles.contains("cache-the-session-store"),
        "the prior binding must be gone after the overwrite; got:\n{roles}",
    );
}

/// The two task-state verbs join the ack envelope (`design/command-output-contract.md`
/// §2 → The task-state verbs join the envelope (M42)). Both mutate durable state, so
/// neither may confirm success with silence:
///
/// - `task bind --format json` → `{op: task-bind, task, role, target: {doctype, slug},
///   findings: []}` — the two values the bind established, plus the work unit it
///   established them on; the plain format prints a one-line ack.
/// - `task discard --format json` → `{op: task-discard, task, findings: []}` — **no**
///   effect key (the ⚠ correction: a `discarded` key could only ever be constant-`true`,
///   because an absent id never reaches the removal).
/// - `task discard <absent-id>` still **exits 1** with the error envelope — the proof
///   that the removed `if task.dir.exists()` guard was dead code (`TaskArea::resolve`
///   bails first), so dropping it changes no behaviour.
#[test]
fn the_task_state_verbs_ack_their_mutation() {
    let repo = TempDir::new("ack");
    init_repo(repo.path());
    let home = TempDir::new("ack-home");
    let slug = "cache-the-session-store";
    commit_spec(repo.path(), slug);

    let mint = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "implement the spec",
        ],
    );
    assert!(
        mint.status.success(),
        "the implement-from-spec mint must succeed; stderr:\n{}",
        stderr_of(&mint),
    );
    let task_id = "implement-the-spec";

    // `task bind --format json` — the ack names the op, the work unit, the role, and
    // the bound doc decomposed into `doctype` + `slug` (a bind targets a whole doc).
    let bound = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            task_id,
            "--format",
            "json",
        ],
    );
    assert!(
        bound.status.success(),
        "the bind must succeed; stderr:\n{}",
        stderr_of(&bound),
    );
    let ack: serde_json::Value =
        serde_json::from_str(stdout_of(&bound).trim()).expect("the bind ack parses as JSON");
    assert_eq!(ack["op"], "task-bind");
    assert_eq!(ack["task"], task_id);
    assert_eq!(ack["role"], "spec");
    assert_eq!(ack["target"]["doctype"], "spec");
    assert_eq!(ack["target"]["slug"], slug);
    assert_eq!(ack["findings"], serde_json::json!([]));

    // The plain (agent) format is a one-line ack — not silence.
    let plain = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            task_id,
        ],
    );
    assert!(plain.status.success(), "the re-bind must succeed");
    let plain_ack = stdout_of(&plain);
    assert!(
        plain_ack.lines().filter(|l| !l.trim().is_empty()).count() == 1
            && plain_ack.contains("spec:cache-the-session-store")
            && plain_ack.contains(task_id),
        "the plain bind ack must be one line naming the bound doc and the task; got:\n{plain_ack}",
    );

    // `task discard --format json` — the ack names the op + the work unit, carries
    // `findings: []`, no `discarded` effect key and no `target` — and since the
    // 2026-07-17 surface review (B3) it enumerates the staged docs the removal
    // dropped (`dropped`, the `<type>:<slug>` identities): here the task's
    // provisioned commit doc.
    let discarded = run(
        repo.path(),
        home.path(),
        &["task", "discard", task_id, "--force", "--format", "json"],
    );
    assert!(
        discarded.status.success(),
        "the discard must succeed; stderr:\n{}",
        stderr_of(&discarded),
    );
    let ack: serde_json::Value =
        serde_json::from_str(stdout_of(&discarded).trim()).expect("the discard ack parses as JSON");
    assert_eq!(ack["op"], "task-discard");
    assert_eq!(ack["task"], task_id);
    assert_eq!(ack["findings"], serde_json::json!([]));
    assert_eq!(
        ack["dropped"],
        serde_json::json!([format!("commit:{task_id}")]),
        "the discard ack enumerates the dropped staged docs; got:\n{ack}",
    );
    assert!(
        ack.get("discarded").is_none() && ack.get("target").is_none(),
        "the discard ack carries no effect key and no target; got:\n{ack}",
    );
    assert!(
        !repo.path().join(".jigc/tasks").join(task_id).exists(),
        "the discard must have removed the working area",
    );

    // Discarding an absent id is **not** idempotent: it exits 1 with the **findings**
    // envelope (`TaskArea::resolve` refuses before any removal could run). Since M51
    // Increment 6 / T1 that refusal is the contract-keyed `finalize.no-task` finding —
    // the key a driver reads is `(finalize.no-task, task:<id>)` — rather than the
    // flattened `{"error": …}` (`work_unit_unknown_envelope.rs` sweeps the whole door set).
    let gone = run(
        repo.path(),
        home.path(),
        &["task", "discard", task_id, "--format", "json"],
    );
    assert_eq!(
        gone.status.code(),
        Some(1),
        "discarding an absent task must exit 1; stdout:\n{}",
        stdout_of(&gone),
    );
    let err: serde_json::Value = serde_json::from_str(stderr_of(&gone).trim())
        .expect("the absent-task findings envelope parses as JSON");
    assert_eq!(
        err["findings"][0]["key"],
        serde_json::json!({
            "code": "finalize.no-task",
            "target": format!("task:{task_id}"),
        }),
        "the absent-task rejection is the findings envelope, keyed at the work unit; \
         got:\n{err}",
    );

    // The plain-format discard also acks (one line, naming the task).
    run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "implement the spec",
        ],
    );
    let plain_discard = run(
        repo.path(),
        home.path(),
        &["task", "discard", task_id, "--force"],
    );
    assert!(
        plain_discard.status.success(),
        "the re-discard must succeed"
    );
    let plain_ack = stdout_of(&plain_discard);
    assert!(
        plain_ack.lines().filter(|l| !l.trim().is_empty()).count() == 1
            && plain_ack.contains(task_id),
        "the plain discard ack must be one line naming the task; got:\n{plain_ack}",
    );
}

/// Both streams of one run — a refusal's bytes reach stderr, an ack's stdout, and the
/// arms below assert over whichever the door chose.
fn streams(out: &Output) -> String {
    format!("{}{}", stdout_of(out), stderr_of(out))
}

/// The **one** `route:` line a rendered refusal carries, trimmed of its indent. Panics on
/// a surface carrying none or several, so a byte-equality claim below is about *the* route
/// and never about the first of two.
fn sole_route_line(rendered: &str, what: &str) -> String {
    let routes: Vec<&str> = rendered
        .lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with("route:"))
        .collect();
    assert_eq!(
        routes.len(),
        1,
        "{what}: expected exactly one `route:` line; got:\n{rendered}",
    );
    routes[0].to_string()
}

/// **M52 Increment 10 / T7 — `jigc task bind`'s two refusals join the route floor.**
///
/// Both shipped as bare `anyhow` bails — ``malformed address `padding` `` and ``no such
/// doc `spec:nope` ``, flattened to `{"error": …}` under `--format json` with no code, no
/// locus and no route (per-axis-review axis-6 §4 lead 2: *"the last inch of DEFECT A6-3's
/// dead end"*; settle-record D13 lead 6a fires).
///
///   * the **grammar** fault now composes through `cli::doc`'s single guidance home, so
///     its explanation and its route are the ones `jigc doc show` emits for the same
///     fault — the route asserted **byte-equal** against the sibling's *emitted* line,
///     never against a string rebuilt here;
///   * the **store miss** raises `store.not-found` on the findings envelope, so the
///     `(code, target)` key `design/command-output-contract.md` promises a driver for that
///     code actually resolves at this door.
///
/// **The one byte the two doors do not share, and why.** The head sentence's parenthetical
/// — *a singleton doctype … may be named bare* — is true at the nine `doc` doors, which
/// expand a bare fixed-identity head, and **false here**: this door parses the address
/// verbatim. The arm drives that divergence rather than asserting it from the source
/// (`jigc doc show changelog` reaches `changelog:changelog`; `jigc task bind spec
/// changelog <id>` is refused as malformed), so offering the bare spelling in this door's
/// explanation would be a law-1 lie routing the caller into the same refusal.
#[test]
fn task_bind_refusals_carry_the_shared_guidance_and_the_store_not_found_key() {
    let repo = TempDir::new("route-floor-repo");
    init_repo(repo.path());
    let home = TempDir::new("route-floor-home");

    let mint = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "implement the spec",
        ],
    );
    assert!(
        mint.status.success(),
        "the implement-from-spec mint must succeed; stderr:\n{}",
        stderr_of(&mint),
    );
    let task_id = "implement-the-spec";

    // ── the grammar fault ────────────────────────────────────────────────────────────
    let bind_bad = run(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", "padding", task_id],
    );
    assert_eq!(
        bind_bad.status.code(),
        Some(1),
        "a malformed bind address refuses at exit 1",
    );
    let bind_rendered = streams(&bind_bad);
    let show_bad = run(repo.path(), home.path(), &["doc", "show", "padding"]);
    let show_rendered = streams(&show_bad);

    assert_eq!(
        sole_route_line(&bind_rendered, "`task bind spec padding`"),
        sole_route_line(&show_rendered, "`doc show padding`"),
        "the two doors answer one grammar fault with one route; got:\n{bind_rendered}\n---\n{show_rendered}",
    );
    let shared = "a doc is addressed as `<type>:<slug>`, e.g. `adr:single-node-cache`";
    assert!(
        bind_rendered.contains(shared) && show_rendered.contains(shared),
        "both doors carry the shared head explanation; got:\n{bind_rendered}\n---\n{show_rendered}",
    );

    // The divergence, driven: the bare fixed-identity head IS an address at the `doc`
    // door and is NOT one here, so this door's sentence may not offer it.
    let show_bare = run(repo.path(), home.path(), &["doc", "show", "changelog"]);
    assert!(
        streams(&show_bare).contains("changelog:changelog"),
        "`doc show changelog` must expand the bare head; got:\n{}",
        streams(&show_bare),
    );
    let bind_bare = run(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", "changelog", task_id],
    );
    assert!(
        streams(&bind_bare).contains("malformed address `changelog`"),
        "`task bind` parses the address verbatim, so the bare head is malformed here; got:\n{}",
        streams(&bind_bare),
    );
    assert!(
        show_rendered.contains("may be named bare") && !bind_rendered.contains("may be named bare"),
        "only the expanding door may offer the bare spelling; got:\n{bind_rendered}\n---\n{show_rendered}",
    );

    // ── the store miss ───────────────────────────────────────────────────────────────
    let miss = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:nope",
            task_id,
            "--format",
            "json",
        ],
    );
    assert_eq!(
        miss.status.code(),
        Some(1),
        "a bind address that names no committed doc refuses at exit 1",
    );
    let envelope: serde_json::Value =
        serde_json::from_str(streams(&miss).trim()).unwrap_or_else(|err| {
            panic!(
                "the miss must be the findings envelope, not a flattened \
             `{{error}}`: {err}; got:\n{}",
                streams(&miss)
            )
        });
    let finding = &envelope["findings"][0];
    assert_eq!(finding["code"], "store.not-found");
    assert_eq!(finding["key"]["code"], "store.not-found");
    assert_eq!(
        finding["key"]["target"], "spec:nope",
        "the key's target is the address the caller named; got:\n{envelope}",
    );
    // …and the route is followable, driven from the **emitted** bytes rather than rebuilt
    // here: the backticked span of the finding's own route, run verbatim.
    let route = finding["route"].as_str().expect("the refusal routes");
    let span = route
        .split('`')
        .nth(1)
        .unwrap_or_else(|| panic!("the route carries a command span; got: {route}"));
    let argv: Vec<&str> = span.split_whitespace().skip(1).collect();
    let followed = run(repo.path(), home.path(), &argv);
    assert!(
        followed.status.success(),
        "the emitted route must run as printed (`{span}`); got:\n{}",
        streams(&followed),
    );
}
