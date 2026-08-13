//! M48 Increment 2 / T2 — **`doc create` and `doc author` stop acking success over a
//! dropped or diverted title** (`design/write-commands.md` → The four-way write over a
//! committed doc, its fourth member; `DECISIONS.md` → 2026-08-13 the Settle, F2).
//!
//! Three cells reproduced at HEAD, all exit 0, and the last of them commits two ADRs:
//!
//!   * **cell A — the silent no-op.** A re-author titled `Adopt Redis!` over the staged
//!     `adr:adopt-redis` slugs to the *same* id, so the create is handed the staged copy
//!     back **as found** and the supplied title is never written: the doc still reads
//!     `# Adopt Redis` and the write acks success over a title change that did not
//!     happen.
//!   * **cell B — the diverted title.** A re-author titled `Adopt Valkey` mints a
//!     *different* identity, so the correction becomes a **second document** bound over
//!     the first — and `jigc task finalize` commits both.
//!   * **cell C — the third-doc mint.** `doc create --slug <other>` does the same one hop
//!     further out, with the title held constant.
//!
//! **Two shapes, two codes, and the split is what the cells are for.** A call that would
//! mint or re-point a *different* identity is `write.identity-change` — the shipped code
//! whose sentence is *"you are changing identity through a verb that cannot"*. A call
//! that lands on the **same** identity and merely drops the title is **not** an identity
//! change — `design/storage.md` → Identity calls an ordinary retitle identity-**stable**
//! — so it earns its own member, `write.title-ignored`, whose sentence (*the title you
//! supplied will not become this doc's `# H1`*) is true of exactly the cell it fires on.
//!
//! Every arm drives the **real binary**, and every reject is adjudicated four ways: it
//! **blocks non-zero**, its `(code, key.target)` discriminates, the task's staged doc set
//! is **byte-identical and unchanged in membership** across it (no second doc minted, no
//! empty doc rolled forward), and the emitted `route` is lifted out of the `--format
//! json` finding and **run verbatim** to exit 0 — the emitted bytes, never a test-side
//! reconstruction of them.
//!
//! The counter-arms are half the suite, because a guard that blocks the legitimate write
//! is its own defect: a **first** create/author still lands, an **identical**-title
//! re-author still acks `already existed — copied in for update`, a singleton created
//! with the title it will actually carry still lands, and a `jigc migrate --slug`-driven
//! author still lands.

use crate::support;

use cli::pack::{CompositePack, EmbeddedPack};
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::Schema;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use support::trial_corpus::{State, TrialCorpus};

// ───────────────────────────── driving helpers ─────────────────────────────

/// Run `jigc --format json <args…>` (optionally with `stdin`) and return
/// `(exit-ok, stdout, stderr)`.
fn json(corpus: &TrialCorpus, args: &[&str], stdin: Option<&str>) -> (bool, String, String) {
    let mut argv = vec!["--format", "json"];
    argv.extend_from_slice(args);
    let out = match stdin {
        Some(payload) => corpus.jigc_stdin(&argv, payload),
        None => corpus.jigc(&argv),
    };
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// The first blocking finding of a `--format json` reject, parsed off stderr.
fn blocking_finding(stderr: &str, what: &str) -> Value {
    let report: Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("`{what}`: stderr must be one JSON report ({e}):\n{stderr}"));
    report["findings"][0].clone()
}

/// The finding's stable `key.target` — the value a driver dedupes on.
fn key_target(finding: &Value) -> String {
    finding["key"]["target"]
        .as_str()
        .unwrap_or_else(|| panic!("key.target is a string, not null; got:\n{finding:#}"))
        .to_string()
}

/// The leading backticked command of a route string (`` `<cmd>`<tail> ``).
fn backticked<'a>(route: &'a str, what: &str) -> &'a str {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("`{what}`: the route carries a backticked command; got: {route}"))
}

/// The route of a blocking finding, as the argv a shell would run — the **emitted
/// bytes**, split on the route's own quoting so a multi-word title survives.
fn route_argv(finding: &Value, what: &str) -> Vec<String> {
    let route = finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("`{what}`: the reject carries a route; got:\n{finding:#}"));
    let argv = support::shell_split(backticked(route, what));
    assert_eq!(
        argv.first().map(String::as_str),
        Some("jigc"),
        "`{what}`: the route leads `jigc`; got: {route}"
    );
    argv
}

/// Run a lifted route **verbatim** and assert it exits 0.
fn run_route(corpus: &TrialCorpus, argv: &[String], what: &str) {
    let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    let out = corpus.jigc(&args);
    assert!(
        out.status.success(),
        "`{what}`: the emitted route must run verbatim to exit 0; argv {argv:?}\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The task's **staged doc set** — every `docs/<type>:<slug>.md` under the working
/// area, keyed by file name. Membership *and* bytes, because the two failure shapes
/// this guard closes are different: a diverted title mints a second file, and a
/// mid-write rollback can roll an empty one forward.
fn staged_docs(repo: &Path, task: &str) -> BTreeMap<String, String> {
    let dir = repo.join(".jigc/tasks").join(task).join("docs");
    let Ok(entries) = fs::read_dir(&dir) else {
        return BTreeMap::new();
    };
    entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        .map(|p| {
            let name = p
                .file_name()
                .expect("a staged doc has a file name")
                .to_string_lossy()
                .into_owned();
            (name, fs::read_to_string(&p).expect("read a staged doc"))
        })
        .collect()
}

/// The staged `adr:*` file names of a task, sorted — the membership half of the
/// no-second-doc claim, stated where it is asserted.
fn staged_adrs(repo: &Path, task: &str) -> Vec<String> {
    staged_docs(repo, task)
        .into_keys()
        .filter(|name| name.starts_with("adr:"))
        .collect()
}

// ───────────────────────────── the shared base state ─────────────────────────────

/// The ADR payload the base authors, and the shape every divergence arm re-sends with
/// one line changed.
fn adr_payload(title: &str) -> String {
    format!(
        "\
title: {title}
sections:
  - id: context
    set:
      context: |-
        <<The cache is cold on every deploy.>>
  - id: decision
    set:
      decision: |-
        <<Adopt Redis.>>
  - id: consequences
    set:
      consequences: |-
        <<One more service to run.>>
"
    )
}

/// The base every divergence cell drives: a `record-decision` task holding one
/// **staged, never-committed** `adr:adopt-redis` titled `Adopt Redis`.
struct Base {
    corpus: TrialCorpus,
    task: String,
}

impl Base {
    fn build() -> Self {
        let corpus = TrialCorpus::build(State::Fresh);
        let task = corpus.start_workflow("record-decision", "adopt redis");
        corpus.jigc_ok(&[
            "doc",
            "create",
            "adr",
            "--title",
            "Adopt Redis",
            "--task",
            &task,
        ]);
        corpus.jigc_stdin_ok(
            &["doc", "author", "adr", "--from-file", "-", "--task", &task],
            &adr_payload("Adopt Redis"),
        );
        Base { corpus, task }
    }
}

/// Adjudicate one reject: it blocks non-zero with `code`, the staged doc set is
/// unchanged in membership **and** bytes, and the emitted route runs verbatim to exit 0
/// (run last — the route is itself a write, and this is where the recovery is proven
/// followable rather than merely well-worded).
fn reject(base: &Base, what: &str, code: &str, args: &[&str], stdin: Option<&str>) -> Value {
    let before = staged_docs(&base.corpus.repo(), &base.task);
    let (ok, stdout, stderr) = json(&base.corpus, args, stdin);
    assert!(
        !ok,
        "`{what}` must block (non-zero exit); stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let finding = blocking_finding(&stderr, what);
    assert_eq!(
        finding["code"], code,
        "`{what}` must carry `{code}`; got:\n{stderr}"
    );
    let after = staged_docs(&base.corpus.repo(), &base.task);
    assert_eq!(
        after.keys().collect::<Vec<_>>(),
        before.keys().collect::<Vec<_>>(),
        "`{what}`: the staged doc SET is unchanged — no second doc minted, no empty doc \
         rolled forward"
    );
    assert_eq!(
        after, before,
        "`{what}`: the staged bytes are unchanged — a rejected write persists nothing"
    );
    let argv = route_argv(&finding, what);
    run_route(&base.corpus, &argv, what);
    finding
}

// ───────────────────────────── cell A — the silent no-op ─────────────────────────────

/// **Cell A.** `Adopt Redis!` slugs to the same `adopt-redis`, so the create is handed
/// the staged copy back as found and the supplied title is dropped on the floor. It is
/// **not** an identity change (the id never moves — `design/storage.md` → Identity), so
/// it earns `write.title-ignored`, and it routes at the verb that *does* move a staged
/// doc's title.
#[test]
fn cell_a_a_same_slug_reauthor_with_a_dropped_title_blocks_and_routes() {
    let arm = Base::build();
    let payload = adr_payload("Adopt Redis!");
    let finding = reject(
        &arm,
        "author with a same-slug divergent title",
        "write.title-ignored",
        &[
            "doc",
            "author",
            "adr",
            "--from-file",
            "-",
            "--task",
            &arm.task,
        ],
        Some(&payload),
    );
    assert_eq!(
        key_target(&finding),
        "adr:adopt-redis",
        "the reject keys on the doc whose title would have been dropped"
    );
    assert!(
        finding["message"]
            .as_str()
            .is_some_and(|m| m.contains("Adopt Redis!") && m.contains("Adopt Redis")),
        "the message names both the supplied title and the one the doc would keep:\n{finding:#}"
    );
    // The route did what it promised: the staged doc now carries the requested title, so
    // the very write that was refused now lands unchanged.
    let shown = arm
        .corpus
        .jigc_ok(&["doc", "show", "adr:adopt-redis", "--task", &arm.task]);
    assert!(
        shown.contains("# Adopt Redis!"),
        "the route's rename must land the requested title; got:\n{shown}"
    );
    arm.corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "adr",
            "--from-file",
            "-",
            "--task",
            &arm.task,
        ],
        &payload,
    );
}

// ─────────────────────── cell B — the diverted title, a second doc ───────────────────

/// **Cell B.** The headline: a divergent title on the re-author minted a *second* ADR
/// and `finalize` committed both. The gate entry's `as:` role is already bound to
/// `adr:adopt-redis`, so a call that would mint a different identity is refused —
/// `write.identity-change`, routed at the correction the agent actually meant.
#[test]
fn cell_b_a_divergent_title_second_author_blocks_instead_of_minting_a_second_doc() {
    let arm = Base::build();
    let payload = adr_payload("Adopt Valkey");
    let finding = reject(
        &arm,
        "a divergent-title second author",
        "write.identity-change",
        &[
            "doc",
            "author",
            "adr",
            "--from-file",
            "-",
            "--task",
            &arm.task,
        ],
        Some(&payload),
    );
    assert_eq!(
        key_target(&finding),
        "adr:adopt-redis",
        "the reject keys on the identity this task holds — the doc the correction is about"
    );
    assert!(
        finding["message"]
            .as_str()
            .is_some_and(|m| m.contains("adr:adopt-valkey") && m.contains("adr:adopt-redis")),
        "the message names both the identity this task holds and the one the call would \
         have minted:\n{finding:#}"
    );
    // The route moved the identity rather than minting beside it: one doc, renamed.
    assert_eq!(
        staged_adrs(&arm.corpus.repo(), &arm.task),
        vec!["adr:adopt-valkey.md".to_string()],
        "after the route, the task stages exactly ONE adr — the renamed one"
    );
}

// ──────────────────── cell C — `--slug <other>`, the third-doc mint ────────────────────

/// **Cell C.** The same divergence one hop further out: the title is held constant and
/// `--slug` alone moves the identity, which at HEAD minted a *third* document at exit 0.
/// The reject's route carries the `--slug` through, so the correction the agent asked for
/// is the one that runs.
#[test]
fn cell_c_a_slug_override_over_a_staged_doc_blocks_instead_of_minting_a_third() {
    let arm = Base::build();
    let finding = reject(
        &arm,
        "create --slug over a staged doc",
        "write.identity-change",
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Adopt Redis",
            "--slug",
            "adopt-memcached",
            "--task",
            &arm.task,
        ],
        None,
    );
    let route = finding["route"].as_str().expect("the reject routes");
    assert!(
        route.contains("--slug adopt-memcached"),
        "the route carries the requested slug through, so the correction is the one the \
         agent asked for; got: {route}"
    );
    assert_eq!(
        staged_adrs(&arm.corpus.repo(), &arm.task),
        vec!["adr:adopt-memcached.md".to_string()],
        "after the route, the task stages exactly ONE adr — the re-slugged one"
    );
}

// ───────────────────── the singleton arm, enumerated from the census ─────────────────

/// The production pack composition, built the **CWD-free** way: `[dev ▸ methodology]`.
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// Every shipped doctype, keyed by id.
fn shipped_schemas() -> BTreeMap<String, Schema> {
    let pack = composite();
    let mut out = BTreeMap::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .expect("a listed schema reads back");
        let schema = cli::pack::load_pack_schema(&pack, &bytes).expect("a shipped schema parses");
        out.insert(schema.ty.clone(), schema);
    }
    out
}

/// A workflow whose create-gate admits `doctype`, read from the loaded workflow
/// registry. A `migrate-*` workflow is minted by `jigc migrate <path>` rather than
/// `jigc start`, so a directly-startable one is preferred when both admit the doctype.
fn admitting_workflow(doctype: &str) -> String {
    let pack = composite();
    let mut admitting: Vec<String> = pack
        .list(PackResourceKind::Workflows)
        .iter()
        .filter_map(|id| {
            let bytes = pack.read(PackResourceKind::Workflows, id).ok()?;
            let def = engine::compose::load_workflow_def(&bytes).ok()?;
            def.allows_create
                .iter()
                .any(|entry| entry.doc_type == doctype)
                .then(|| id.as_str().to_string())
        })
        .collect();
    admitting.sort_by_key(|id| (id.starts_with("migrate-"), id.clone()));
    admitting
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("no shipped workflow admits `{doctype}`"))
}

/// **The singleton arm, over the whole census.** A `placement` / `display-title`
/// singleton has no author-owned title: its `# H1` is the schema's own, so a `--title`
/// that diverges from it is silently dropped — the same defect cell A closes, one rank
/// higher (it is true of the *doctype*, before any instance is read). The complement is
/// asserted in the same loop: the title the doc will actually carry still lands.
#[test]
fn every_fixed_title_doctype_refuses_a_title_it_would_drop() {
    // The set is read from the same `Schema::fixed_title` predicate the guard branches
    // on, so a singleton added to either pack joins this sweep rather than escaping it.
    let cells: Vec<(String, String)> = shipped_schemas()
        .into_iter()
        .filter_map(|(ty, schema)| schema.fixed_title().map(|title| (ty, title)))
        .collect();
    assert!(
        cells.len() >= 5,
        "the fixed-title census is populated (5 shipped singletons at M48); got: {cells:?}"
    );

    for (ty, fixed) in cells {
        let workflow = admitting_workflow(&ty);
        let corpus = TrialCorpus::build(State::Fresh);
        let task = corpus.start_workflow(&workflow, "poke the singleton title");

        let divergent = format!("{fixed} of ours");
        let (ok, stdout, stderr) = json(
            &corpus,
            &["doc", "create", &ty, "--title", &divergent, "--task", &task],
            None,
        );
        assert!(
            !ok,
            "`doc create {ty} --title {divergent:?}` must block — the title is the \
             schema's, not the author's; stdout:\n{stdout}"
        );
        let what = format!("{ty} fixed title");
        let finding = blocking_finding(&stderr, &what);
        assert_eq!(
            finding["code"], "write.title-ignored",
            "`{ty}`'s refusal names the dropped title, not an identity change:\n{stderr}"
        );
        assert!(
            finding["message"]
                .as_str()
                .is_some_and(|m| m.contains(&fixed)),
            "`{ty}`'s refusal names the title the doc WILL carry:\n{stderr}"
        );
        let argv = route_argv(&finding, &what);
        run_route(&corpus, &argv, &what);

        // The complement, in the same loop: the guard refuses the dropped title and
        // nothing else — the title the doc will actually carry still lands.
        assert!(
            corpus
                .jigc(&["doc", "create", &ty, "--title", &fixed, "--task", &task])
                .status
                .success(),
            "`doc create {ty} --title {fixed:?}` must still land — the guard refuses the \
             DROPPED title, not the doctype"
        );
    }
}

// ───────────────────────────── the counter-arms ─────────────────────────────

/// A **first** create and a **first** author still land — the guard has nothing to
/// compare against until an identity exists, and must not invent one.
#[test]
fn a_first_create_and_a_first_author_still_land() {
    let corpus = TrialCorpus::build(State::Fresh);

    let create_task = corpus.start_workflow("record-decision", "pick a queue");
    let ack = corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Pick a queue",
        "--task",
        &create_task,
    ]);
    assert!(
        ack.contains("adr:pick-a-queue"),
        "a first create mints the identity it was asked for; got:\n{ack}"
    );

    let author_task = corpus.start_workflow("record-decision", "pick a cache");
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "adr",
            "--from-file",
            "-",
            "--task",
            &author_task,
        ],
        &adr_payload("Pick a cache"),
    );
    let shown = corpus.jigc_ok(&["doc", "show", "adr:pick-a-cache", "--task", &author_task]);
    assert!(
        shown.contains("# Pick a cache"),
        "a first author mints and titles the doc; got:\n{shown}"
    );
}

/// A re-author with the **identical** title still acks the copy-in and changes nothing —
/// the iteration loop `batch_author_rerun.rs` pins stays open. (That suite is the fence;
/// this arm is the statement of what keeps the guard from becoming its own defect.)
#[test]
fn an_identical_title_reauthor_still_acks_the_copy_in() {
    let base = Base::build();
    let ack = base.corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Adopt Redis",
        "--task",
        &base.task,
    ]);
    assert!(
        ack.contains("already existed — copied in for update"),
        "the identical-title re-create still acks the copy-in; got:\n{ack}"
    );
    let before = staged_docs(&base.corpus.repo(), &base.task);
    base.corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "adr",
            "--from-file",
            "-",
            "--task",
            &base.task,
        ],
        &adr_payload("Adopt Redis"),
    );
    let after = staged_docs(&base.corpus.repo(), &base.task);
    assert_eq!(
        after.keys().collect::<Vec<_>>(),
        before.keys().collect::<Vec<_>>(),
        "the identical-title re-author revises one doc, it does not mint another"
    );
}

/// A `jigc migrate … --slug <s>`-driven author still lands. The recorded slug override
/// drives the minted id away from the payload title, which is exactly the shape the
/// divergence guard adjudicates — so this arm proves the guard reads the **bound**
/// identity, never the title-to-slug relation.
#[test]
fn a_migrate_slug_driven_author_still_lands() {
    let corpus = TrialCorpus::build(State::Fresh);
    fs::write(
        corpus.repo().join("old-decision.md"),
        "# Use Postgres\n\nWe picked Postgres over MySQL for the JSON support.\n",
    )
    .expect("write the foreign doc");
    corpus.git(&["add", "old-decision.md"]);
    corpus.git(&["commit", "-m", "the foreign decision"]);

    corpus.jigc_ok(&[
        "migrate",
        "old-decision.md",
        "--as",
        "adr",
        "--slug",
        "database-choice",
    ]);
    let task = sole_task(&corpus.repo());
    corpus.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        &adr_payload("Use Postgres"),
    );
    let shown = corpus.jigc_ok(&["doc", "show", "adr:database-choice", "--task", &task]);
    assert!(
        shown.contains("# Use Postgres"),
        "the slug-overridden migration author lands its own title; got:\n{shown}"
    );
}

/// The one active task's id, read off the working area (the migrate door's mint).
fn sole_task(repo: &Path) -> String {
    let mut ids: Vec<String> = fs::read_dir(repo.join(".jigc/tasks"))
        .expect("the task area exists")
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    ids.sort();
    assert_eq!(ids.len(), 1, "exactly one task is in flight; got: {ids:?}");
    ids.remove(0)
}
