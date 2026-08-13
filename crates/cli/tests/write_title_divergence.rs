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
/// bytes**, split by a **real `sh`** (`support::shell_words`) so the route's own quoting
/// is adjudicated by the thing that will actually parse it, expansions included.
fn route_argv(corpus: &TrialCorpus, finding: &Value, what: &str) -> Vec<String> {
    let route = finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("`{what}`: the reject carries a route; got:\n{finding:#}"));
    let argv = support::shell_words(backticked(route, what), &corpus.repo(), &corpus.home());
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
        Self::titled("Adopt Redis")
    }

    /// The same base under an arbitrary incumbent `title` — the metachar axis needs an
    /// incumbent whose *own* title carries the shell-special bytes, so a same-slug
    /// divergence over it is reachable at all.
    fn titled(title: &str) -> Self {
        let corpus = TrialCorpus::build(State::Fresh);
        let task = corpus.start_workflow("record-decision", "adopt redis");
        corpus.jigc_ok(&["doc", "create", "adr", "--title", title, "--task", &task]);
        corpus.jigc_stdin_ok(
            &["doc", "author", "adr", "--from-file", "-", "--task", &task],
            &adr_payload(title),
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
    let argv = route_argv(&base.corpus, &finding, what);
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

// ─────────────── the metachar axis — a route is bytes a shell will parse ───────────────

/// The **metachar axis** these two arms iterate. A title is author-owned prose — LLM-
/// written by the determinism boundary — so `$`, a backtick, `$( … )` and a quote are
/// input this door is *designed* to receive, and each one is live inside the double
/// quotes the pre-M47 rendering form emits:
///
///   * `$HOME` — parameter expansion: the route renames to a *different* title at exit 0;
///   * `$(touch PWNED)` — command substitution: the emitted route **executes** the
///     embedded command, and the title loses the substituted span;
///   * `'quoted'` — the counter-member: nothing expands, but the POSIX single-quoting the
///     fix emits has to survive its own `'` (`'\''`), so a fix that quotes naively
///     reddens here rather than in production.
///
/// The backtick form of substitution is **deliberately absent**: a route renders its
/// argv inside a backticked code span, so a title carrying a backtick cannot be lifted
/// back out of the emitted text by any reader — a presentation question of its own, and
/// not one a shell-quoting fix answers. `$( … )` is the same hazard, liftable.
const METACHAR_TITLES: &[&str] = &[
    "Cache $HOME rework",
    "Cache $(touch PWNED) rework",
    "Cache 'quoted' rework",
];

/// The file a command-substituted title creates if the emitted route is parsed by a real
/// shell — the observable half of "the route ran a second command".
const PWNED: &str = "PWNED";

/// The single staged `adr:<slug>` identity of a task, as its **address** — read off the
/// working area rather than re-derived from the title, so no test-side copy of the slug
/// rule can paper over a divergence.
fn sole_staged_adr(repo: &Path, task: &str) -> String {
    let mut adrs = staged_adrs(repo, task);
    assert_eq!(
        adrs.len(),
        1,
        "the task stages exactly one adr; got: {adrs:?}"
    );
    let name = adrs.remove(0);
    name.strip_suffix(".md")
        .expect("a staged doc is a `.md`")
        .to_string()
}

/// **The identity-divergence route is shell bytes.** The reject's route is a `jigc doc
/// rename … --to <title>` an agent pastes into a shell, and [`reject`] runs it through a
/// real `sh`. So the claim under test is the increment's own Proves — *no title
/// correction mints a document nobody authored*: after the route, the task holds exactly
/// one ADR whose `# H1` is the title that was asked for, **byte for byte**, and no
/// embedded command has run.
#[test]
fn the_identity_divergence_route_survives_a_real_shell_over_the_metachar_axis() {
    for title in METACHAR_TITLES {
        let arm = Base::titled("Adopt Redis");
        let payload = adr_payload(title);
        let what = format!("identity divergence to {title:?}");
        reject(
            &arm,
            &what,
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
        let repo = arm.corpus.repo();
        let address = sole_staged_adr(&repo, &arm.task);
        let shown = arm
            .corpus
            .jigc_ok(&["doc", "show", &address, "--task", &arm.task]);
        assert!(
            shown.contains(&format!("# {title}\n")),
            "`{what}`: the emitted route, parsed by a real shell, must land the title \
             that was asked for — not one the shell rewrote; got:\n{shown}"
        );
        assert!(
            !repo.join(PWNED).exists(),
            "`{what}`: the emitted route must not execute a command embedded in the title"
        );
    }
}

/// **The title-ignored route is shell bytes too.** Same claim at the sibling code: the
/// incumbent itself carries the metachars, and the divergence is a trailing `!` — which
/// the slug rule strips, so the call lands on the *same* identity and earns
/// `write.title-ignored`. Its route is the same in-task rename, and it must land the
/// requested title rather than a shell-rewritten one.
#[test]
fn the_title_ignored_route_survives_a_real_shell_over_the_metachar_axis() {
    for title in METACHAR_TITLES {
        let arm = Base::titled(title);
        let divergent = format!("{title}!");
        let what = format!("dropped title {divergent:?}");
        reject(
            &arm,
            &what,
            "write.title-ignored",
            &[
                "doc", "create", "adr", "--title", &divergent, "--task", &arm.task,
            ],
            None,
        );
        let repo = arm.corpus.repo();
        let address = sole_staged_adr(&repo, &arm.task);
        let shown = arm
            .corpus
            .jigc_ok(&["doc", "show", &address, "--task", &arm.task]);
        assert!(
            shown.contains(&format!("# {divergent}\n")),
            "`{what}`: the emitted route, parsed by a real shell, must land the title \
             that was asked for; got:\n{shown}"
        );
        assert!(
            !repo.join(PWNED).exists(),
            "`{what}`: the emitted route must not execute a command embedded in the title"
        );
    }
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
        let argv = route_argv(&corpus, &finding, &what);
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

// ─────────────────── the input axis: incumbent bytes × supplied title ───────────────────

/// The guard compares a **supplied title** against the **incumbent's on-disk H1**, and it
/// owns neither input: the agent writes the `--title` / payload `title:`, and *humans* own
/// the committed file (`design/storage.md` — reviewed and edited through git). So the class
/// is swept over both messy axes rather than over the one canonical cell the cells above
/// drive:
///
///   * **the incumbent's bytes** — every checkout shape the engine already supports
///     (`implementation/parsing.md` → "preserve the file's existing EOL … never globally
///     normalize"; `engine::write::first_touch_canonicalize` strips a leading BOM): LF ·
///     CRLF · BOM + a human YAML comment in the front matter;
///   * **the supplied title** — padded, because the mint renders `# {title.trim()}`
///     (`engine::write::render`), so a title the guard reads as divergent can be the very
///     one the doc already carries.
///
/// Each row asserts **both faces**: the legitimate write still lands (a false block is the
/// defect this row exists for), *and* a genuinely divergent title on the same shape still
/// blocks naming the doc's real H1 (a guard neutered to fix the false block is the other
/// defect).
const COMMITTED_TITLE: &str = "Legacy Choice Revisited";

/// The human YAML comment the BOM row plants in the front matter — metadata, never the
/// document's title, and named here so the assertion that no refusal quotes it is the same
/// string the fixture writes.
const FRONT_MATTER_COMMENT: &str = "a human-added yaml comment";

/// A corpus holding one **committed** `adr` titled [`COMMITTED_TITLE`], with the slug the
/// binary emitted and the doc's repo-relative path.
fn committed_adr() -> (TrialCorpus, String, String) {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "legacy choice revisited");
    let address = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "adr",
            "--title",
            COMMITTED_TITLE,
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    let slug = address
        .strip_prefix("adr:")
        .expect("the create acks `adr:<slug>`")
        .to_string();
    corpus.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        &adr_payload(COMMITTED_TITLE),
    );
    corpus.finalize(&task, "decisions", "record the legacy choice", false);
    let path = format!("docs/decisions/{slug}.md");
    assert!(
        corpus.repo().join(&path).is_file(),
        "the ADR landed at its canonical home; got no `{path}`"
    );
    (corpus, slug, path)
}

/// Rewrite the committed doc's bytes out of band and commit them — the checkout a Windows
/// clone or a BOM-writing editor hands the tool, absorbed into the baseline exactly as a
/// real one is.
fn reshape_committed(corpus: &TrialCorpus, path: &str, reshape: impl Fn(&str) -> String) {
    let file = corpus.repo().join(path);
    let source = fs::read_to_string(&file).expect("read the committed doc");
    fs::write(&file, reshape(&source)).expect("rewrite the committed doc");
    corpus.git(&["add", path]);
    corpus.git(&["commit", "-m", "an out-of-band checkout shape"]);
}

/// Both faces over one committed-byte shape: the identical title still lands, and a
/// divergent title still blocks naming the doc's real H1. Returns the task, so a caller
/// whose shape is otherwise conformant can drive the sibling write verb over it too.
fn assert_committed_shape_holds(corpus: &TrialCorpus, slug: &str, shape: &str) -> String {
    let task = corpus.start_workflow("record-decision", &format!("revisit under {shape}"));

    let ack = corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        COMMITTED_TITLE,
        "--slug",
        slug,
        "--task",
        &task,
    ]);
    assert!(
        ack.contains("already existed — copied in for update"),
        "{shape}: a create under the doc's own title must still copy it in, not block on \
         bytes the engine supports; got:\n{ack}"
    );

    let (ok, stdout, stderr) = json(
        corpus,
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Adopt Valkey",
            "--slug",
            slug,
            "--task",
            &task,
        ],
        None,
    );
    assert!(
        !ok,
        "{shape}: a genuinely divergent title must still block; stdout:\n{stdout}\n\
         stderr:\n{stderr}"
    );
    let finding = blocking_finding(&stderr, shape);
    assert_eq!(
        finding["code"], "write.title-ignored",
        "{shape}: the divergent title carries the shipped code; got:\n{stderr}"
    );
    let message = finding["message"].as_str().unwrap_or_default();
    assert!(
        message.contains(COMMITTED_TITLE),
        "{shape}: the refusal names the H1 the doc actually carries; got:\n{message}"
    );
    assert!(
        !message.contains(FRONT_MATTER_COMMENT),
        "{shape}: the refusal never quotes a front-matter line as the doc's title; \
         got:\n{message}"
    );
    task
}

/// **CRLF** — a Windows checkout. The H1 line ends `\r\n`, so a raw byte compare reads the
/// title as `Legacy Choice Revisited\r` and refuses a write whose supplied title is
/// byte-identical to the one on disk (and prints both sides as the same string — a
/// `design/surface-contract.md` law-1 self-contradiction).
#[test]
fn a_crlf_committed_incumbent_takes_its_own_title_and_still_gates_a_divergent_one() {
    let (corpus, slug, path) = committed_adr();
    reshape_committed(&corpus, &path, |source| source.replace('\n', "\r\n"));
    let task = assert_committed_shape_holds(&corpus, &slug, "CRLF");
    // The verb axis: the pre-check is one seam, and `doc author` sends its title through
    // the same one. A CRLF checkout is conformant end to end, so the whole write lands.
    corpus.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        &adr_payload(COMMITTED_TITLE),
    );
}

/// **BOM + a human YAML comment in the front matter.** The BOM defeats a `starts_with("---")`
/// front-matter detection, so the metadata block is scanned as body and a human's `# ` comment
/// is read as the document's H1 — the refusal then names a YAML comment as the title the doc
/// "would keep". (This row drives `create` only: a colon-less YAML comment is not a
/// conformant field block, so `doc author` over it rightly rejects `write.non-reparseable`
/// — a different gate, and not this guard's to pre-empt with a lie about the H1.)
#[test]
fn a_bom_committed_incumbent_never_reads_its_front_matter_as_the_h1() {
    let (corpus, slug, path) = committed_adr();
    reshape_committed(&corpus, &path, |source| {
        format!(
            "\u{feff}{}",
            source.replacen("---\n", &format!("---\n# {FRONT_MATTER_COMMENT}\n"), 1)
        )
    });
    assert_committed_shape_holds(&corpus, &slug, "BOM + front-matter comment");
}

/// **A padded supplied title**, over a *staged* incumbent — no out-of-band edit needed at
/// all, because the mint's own render trims (`# {title.trim()}`) while a raw compare does
/// not. The first create writes `# Leading Space`; the second, sent the identical argv, is
/// refused for a divergence the tool itself created.
#[test]
fn a_padded_title_matches_the_trimmed_h1_the_mint_wrote() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "leading space");
    let padded = "  Leading Space ";
    let address = corpus
        .jigc_ok(&["doc", "create", "adr", "--title", padded, "--task", &task])
        .trim()
        .to_string();

    let ack = corpus.jigc_ok(&["doc", "create", "adr", "--title", padded, "--task", &task]);
    assert!(
        ack.contains("already existed — copied in for update"),
        "the identical argv must be idempotent — the mint trimmed the title it wrote, so \
         the guard must compare it trimmed; got:\n{ack}"
    );

    let shown = corpus.jigc_ok(&["doc", "show", &address, "--task", &task]);
    assert!(
        shown.contains("# Leading Space\n"),
        "the H1 stays the trimmed title the mint rendered; got:\n{shown}"
    );

    // The other face: a title that differs by more than padding still blocks.
    let (ok, _, stderr) = json(
        &corpus,
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "  Leading Space!  ",
            "--slug",
            address.strip_prefix("adr:").expect("`adr:<slug>`"),
            "--task",
            &task,
        ],
        None,
    );
    assert!(!ok, "a genuinely divergent padded title still blocks");
    assert_eq!(
        blocking_finding(&stderr, "padded divergent title")["code"],
        "write.title-ignored",
        "…and carries the shipped code; got:\n{stderr}"
    );
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
