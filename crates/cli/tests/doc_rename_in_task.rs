//! M48 Increment 2 / T1 — **the in-task doc-level title change exists, and it splits
//! on committed-store identity** (`design/write-commands.md` → `jigc doc rename`;
//! `design/storage.md` → Identity; `DECISIONS.md` → 2026-08-13 the Settle, F2 and its
//! committed-store-identity re-settle).
//!
//! Until now a title correction had **nowhere to go**: `jigc rename` is task-less and
//! self-committing (it refuses outright while any task is in flight), and no `jigc doc`
//! verb moved a doc-level title at all — so an agent that mistyped a title re-ran
//! `doc create` / `doc author` with the right one and got a *second* document at exit 0.
//! `jigc doc rename <addr> --to "<New Title>"` is the destination that reject needs.
//!
//! **The cells are enumerated from the loaded pack schema registry, never hand-listed.**
//! Every shipped doctype falls into exactly one of three partitions, keyed on the same
//! code-side predicates the verb branches on:
//!
//!   * **fixed identity** — `Schema::fixed_title().is_some()`, i.e. `singleton: true`:
//!     the slug IS the type id and the `# H1` is the schema's, so there is neither a
//!     title nor a slug to change → **refuse**. Keyed on the rule, not on the
//!     `placement:` / `display-title:` knobs every *shipped* singleton happens to carry
//!     as well — the cell no shipped doctype occupies gets a fixture arm of its own;
//!   * **slug identity** — `location.is_some()`: the title mints the slug, so the verb
//!     applies → re-slug freely while the identity is uncommitted, retitle-only once it
//!     is committed (`milestone-record` is refused above all of it, by the shipped
//!     `machine_maintained_guard`);
//!   * **transient sink** — neither: no committed home and no doc-level `id-from`; the
//!     slug IS the task id (`commit:<task-id>`), machine-derived → **refuse**.
//!
//! A doctype added to either pack therefore **joins an arm** rather than escaping one.
//!
//! Every arm drives the **real binary** against a throwaway corpus and asserts the
//! emitted bytes — the ack, the blocking finding's code, and the route **run verbatim**.

use crate::support;

use cli::pack::{CompositePack, EmbeddedPack};
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::Schema;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use support::trial_corpus::{State, TrialCorpus};

// ───────────────────────────── the doctype census ─────────────────────────────

/// Which rename cell a doctype falls in — the three code-side predicates, in the
/// order the verb evaluates them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    /// `Schema::fixed_title().is_some()` — the `singleton:` set, keyed on the code-side
    /// rule itself rather than on the knobs a shipped singleton happens to declare.
    FixedIdentity,
    /// `location.is_some()` — the slug-identity set (the verb's subject).
    SlugIdentity,
    /// Neither — a transient sink whose slug is the task id.
    Transient,
}

/// The production pack composition, built the **CWD-free** way (`registry_seam.rs`'s
/// idiom): `[dev ▸ methodology]`, dev highest-precedence.
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// Every shipped doctype, keyed by id, carrying its loaded schema and its cell —
/// the enumeration every arm below iterates.
fn census() -> BTreeMap<String, (Schema, Cell)> {
    let pack = composite();
    let mut out = BTreeMap::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .expect("a listed schema reads back");
        let schema = cli::pack::load_pack_schema(&pack, &bytes).expect("a shipped schema parses");
        let cell = if schema.fixed_title().is_some() {
            Cell::FixedIdentity
        } else if schema.location.is_some() {
            Cell::SlugIdentity
        } else {
            Cell::Transient
        };
        out.insert(schema.ty.clone(), (schema, cell));
    }
    out
}

/// The doctype ids in `cell`, sorted.
fn ids_in(cell: Cell) -> Vec<String> {
    census()
        .into_iter()
        .filter(|(_, (_, c))| *c == cell)
        .map(|(ty, _)| ty)
        .collect()
}

// ───────────────────────────── driving helpers ─────────────────────────────

/// Run `jigc --format json <args…>` and return `(exit-ok, stdout, stderr)`.
fn json(corpus: &TrialCorpus, args: &[&str]) -> (bool, String, String) {
    let mut argv = vec!["--format", "json"];
    argv.extend_from_slice(args);
    let out = corpus.jigc(&argv);
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

/// The leading backticked command of a route string (`` `<cmd>`<tail> ``).
fn backticked<'a>(route: &'a str, what: &str) -> &'a str {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("`{what}`: the route carries a backticked command; got: {route}"))
}

/// Split an emitted command into argv **through a real `sh`**, so the bytes an agent
/// would paste are parsed by the thing that will actually parse them — a hand-rolled
/// splitter understands one quoting form and expands nothing, which is exactly how a
/// route that rewrites its own title in a terminal passes a suite
/// (`support::shell_words`).
fn shell_split(corpus: &TrialCorpus, cmd: &str) -> Vec<String> {
    support::shell_words(cmd, &corpus.repo(), &corpus.home())
}

/// Every file under `dir`, as `(display path, contents)`.
fn files_under(dir: &Path) -> Vec<(String, String)> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else {
                out.push(path);
            }
        }
    }
    let mut paths = Vec::new();
    walk(dir, &mut paths);
    paths.sort();
    paths
        .into_iter()
        .map(|p| {
            let body = fs::read(&p).map(|b| String::from_utf8_lossy(&b).into_owned());
            (p.display().to_string(), body.unwrap_or_default())
        })
        .collect()
}

/// **The derived-set fence.** After an in-task re-slug, nothing anywhere under the
/// task working area may still name the old identity — not a filename, not the
/// provenance manifest, not a bound role, not a staged referrer's ref field, not a
/// migration's recorded `slug-override`. Deriving it as a whole-area scan (rather
/// than a hand list of the keyed files) is what makes the fence catch a keyed file
/// this suite never enumerated.
fn no_trace_of(task_dir: &Path, old_slug: &str, old_uri: &str, what: &str) {
    for (path, body) in files_under(task_dir) {
        assert!(
            !path.contains(old_slug),
            "`{what}`: the task area still holds a path naming the old slug `{old_slug}`: {path}"
        );
        assert!(
            !body.contains(old_uri) && !body.contains(old_slug),
            "`{what}`: {path} still names the old identity (`{old_uri}` / `{old_slug}`):\n{body}"
        );
    }
}

// ───────────────────────────── arm 0 — the census ─────────────────────────────

/// The partition is **total over the live registry** and all three cells are
/// populated — so the arms below iterate the real doctype set, and a doctype added
/// to either pack joins an arm rather than escaping the sweep.
#[test]
fn the_rename_census_partitions_the_whole_doctype_registry() {
    let census = census();
    let pack = composite();
    let mut registry: Vec<String> = pack
        .list(PackResourceKind::Schemas)
        .iter()
        .map(|id| id.as_str().to_string())
        .collect();
    registry.sort();

    let mut covered: Vec<String> = census.keys().cloned().collect();
    covered.sort();
    assert_eq!(
        covered, registry,
        "every listed doctype must land in exactly one rename cell"
    );

    for cell in [Cell::FixedIdentity, Cell::SlugIdentity, Cell::Transient] {
        assert!(
            !ids_in(cell).is_empty(),
            "the {cell:?} cell must be populated — an empty cell means its arm proves nothing"
        );
    }

    // The cell IS the `singleton:` set — the rule `Schema::fixed_title` owns and all
    // three seams read — and **every shipped singleton also carries both knobs**, which
    // is exactly why a refusal re-derived from `placement || display-title` stays green
    // over this whole registry while missing the rule. The shipped set cannot catch that
    // divergence, so `a_location_bearing_singleton_refuses_the_rename` drives the cell
    // no shipped doctype occupies.
    for (ty, (schema, cell)) in &census {
        assert_eq!(
            *cell == Cell::FixedIdentity,
            schema.singleton,
            "`{ty}`'s cell must be its `singleton:` flag — the fixed-identity rule"
        );
        if *cell == Cell::FixedIdentity {
            assert!(
                schema.placement.is_some() && schema.display_title.is_some(),
                "`{ty}` is a shipped singleton missing a knob — the coincidence this \
                 census records (and the fixture arm covers) has changed shape"
            );
        }
    }
}

// ──────────────── arm A — a never-committed doc re-slugs, wholly ────────────────

/// **Cell A.** A doc this task minted has no committed identity **by construction**,
/// so a title correction re-slugs it outright: the staged file moves, the `# H1` is
/// rewritten, and every in-task reference to the old identity moves with it — the
/// bound role, the provenance manifest, and a **staged referrer's ref field** (the
/// task's own commit doc, whose `implements` edge points at the renamed spec).
///
/// The fence is a whole-area scan, not a checklist: no file under
/// `.jigc/tasks/<id>/` may still name the old slug or the old address. Then the
/// re-slugged doc still reads through `jigc doc show <new-addr> --task <id>` and
/// still promotes on `finalize`.
#[test]
fn a_never_committed_doc_reslugs_and_moves_every_in_task_reference() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("plan", "pin down the throttle");

    corpus.jigc_ok(&[
        "doc",
        "create",
        "spec",
        "--title",
        "Rate limiting",
        "--task",
        &task,
    ]);
    for (section, prose) in [
        ("goal", "Cap requests per client."),
        ("context", "The gateway is unprotected."),
    ] {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("spec:rate-limiting#{section}"),
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            prose,
        );
    }
    // A **staged referrer**: the task's own commit doc points its `implements` edge at
    // the spec, so the rename has a real in-task reference to move.
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#implements"),
        "--value",
        "spec:rate-limiting",
        "--task",
        &task,
    ]);

    let (ok, stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "rename",
            "spec:rate-limiting",
            "--to",
            "Throttling",
            "--task",
            &task,
        ],
    );
    assert!(
        ok,
        "renaming a never-committed staged doc must land; stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let ack: Value = serde_json::from_str(stdout.trim()).expect("the rename ack is one JSON doc");
    assert_eq!(ack["op"], "rename", "the ack names its op:\n{stdout}");
    assert_eq!(ack["reslugged"], Value::Bool(true), "the identity moved");
    assert_eq!(
        ack["from"], "spec:rate-limiting",
        "the ack names the origin"
    );
    assert_eq!(ack["target"]["doctype"], "spec");
    assert_eq!(ack["target"]["slug"], "throttling");
    assert_eq!(ack["title"], "Throttling");

    let task_dir = corpus.repo().join(".jigc/tasks").join(&task);
    let moved = task_dir.join("docs/spec:throttling.md");
    assert!(moved.is_file(), "the staged file moved to the new identity");
    let body = fs::read_to_string(&moved).expect("read the moved staged doc");
    assert!(
        body.contains("# Throttling") && !body.contains("# Rate limiting"),
        "the `# H1` is rewritten to the new title:\n{body}"
    );

    // The derived-set fence — the whole task area, not a hand list of keyed files.
    no_trace_of(
        &task_dir,
        "rate-limiting",
        "spec:rate-limiting",
        "in-task re-slug",
    );

    // The staged referrer moved *to something*, not merely away from the old id.
    let commit_doc = fs::read_to_string(task_dir.join(format!("docs/commit:{task}.md")))
        .expect("read the staged commit doc");
    assert!(
        commit_doc.contains("spec:throttling"),
        "the staged commit doc's `implements` edge repoints to the new identity:\n{commit_doc}"
    );

    // The read-back path serves the new address, and the old one is gone.
    let shown = corpus.jigc_ok(&["doc", "show", "spec:throttling", "--task", &task]);
    assert!(
        shown.contains("# Throttling"),
        "`doc show <new-addr> --task` serves the renamed doc:\n{shown}"
    );
    assert!(
        !corpus
            .jigc(&["doc", "show", "spec:rate-limiting", "--task", &task])
            .status
            .success(),
        "the old address no longer resolves in the task area"
    );

    // And it still promotes.
    corpus.finalize(&task, "spec", "pin down the throttle", false);
    assert!(
        corpus.repo().join("docs/specs/throttling.md").is_file(),
        "the renamed doc promotes at its new identity"
    );
    assert!(
        !corpus.repo().join("docs/specs/rate-limiting.md").exists(),
        "nothing promotes at the old identity"
    );
}

/// **Cell A, the migration sub-case.** A migration task records its `--slug` override
/// in the working area as a bare slug; the whole-area fence catches it if the rename
/// leaves it stale. The migrated doc is `created` in that task — a migration's target
/// has no committed *managed* identity — so the re-slug arm applies.
#[test]
fn a_migration_slug_override_moves_with_the_rename() {
    let corpus = TrialCorpus::build(State::Fresh);
    fs::create_dir_all(corpus.repo().join("docs/adr")).expect("create the foreign adr dir");
    fs::write(
        corpus.repo().join("docs/adr/0005-choose-a-store.md"),
        "# Use PostgreSQL\n\nWe will use PostgreSQL for the primary store.\n",
    )
    .expect("write the foreign adr");
    corpus.git(&["add", "docs/adr/0005-choose-a-store.md"]);
    corpus.git(&["commit", "-q", "-m", "the foreign decision"]);

    let composed = corpus.jigc_ok(&[
        "migrate",
        "docs/adr/0005-choose-a-store.md",
        "--as",
        "adr",
        "--slug",
        "pinned-id",
    ]);
    let task = composed
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .expect("the migrate mint prints its task id")
        .trim()
        .to_string();

    corpus.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        ADR_PAYLOAD,
    );

    let (ok, stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "rename",
            "adr:pinned-id",
            "--to",
            "Adopt PostgreSQL",
            "--task",
            &task,
        ],
    );
    assert!(
        ok,
        "renaming a migration task's staged doc must land; stdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let task_dir = corpus.repo().join(".jigc/tasks").join(&task);
    no_trace_of(&task_dir, "pinned-id", "adr:pinned-id", "migration re-slug");
    assert_eq!(
        fs::read_to_string(task_dir.join("slug-override")).expect("read the slug override"),
        "adopt-postgresql",
        "the recorded migration slug override moves with the rename"
    );
}

/// A whole-instance `adr` payload — the migration arm's authored body.
const ADR_PAYLOAD: &str = r#"title: "Use PostgreSQL"
sections:
  - id: status
    set:
      status: accepted
  - id: context
    set:
      context: "<<We need a relational store with strong consistency.>>"
  - id: decision
    set:
      decision: "<<We will use PostgreSQL as the primary datastore.>>"
  - id: consequences
    set:
      consequences: "<<Operators must run and back up an instance.>>"
"#;

// ─────────── arm B — a committed identity is retitle-only, and routes ───────────

/// **Cell B.** A doc copied in from the **committed** store keeps its slug: its
/// identity is the path, its referrers live outside the task area, and moving it is
/// `jigc rename`'s job — task-less, transactional, referrer-repointing. So a
/// same-slug retitle lands (the `# H1` corrected in place, the slug frozen), and a
/// `--to` that would re-slug **blocks** with a route to `jigc rename` that runs
/// verbatim.
#[test]
fn a_committed_doc_is_retitle_only_and_a_reslug_routes_at_jigc_rename() {
    let corpus = TrialCorpus::build(State::Fresh);
    let first = corpus.start_workflow("single-task", "pick the cache");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Adopt Redis",
        "--task",
        &first,
    ]);
    for (section, prose) in [
        ("context", "Reads are hot."),
        ("decision", "Use Redis."),
        ("consequences", "One more service."),
    ] {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("adr:adopt-redis#{section}"),
                "--from-file",
                "-",
                "--task",
                &first,
            ],
            prose,
        );
    }
    corpus.finalize(&first, "cache", "pick the cache", false);
    assert!(
        corpus
            .repo()
            .join("docs/decisions/adopt-redis.md")
            .is_file(),
        "the ADR is committed — the cell's precondition"
    );

    let second = corpus.start_workflow("single-task", "revisit the cache");

    // (i) the same-slug retitle lands, slug frozen.
    let (ok, stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "rename",
            "adr:adopt-redis",
            "--to",
            "Adopt Redis!",
            "--task",
            &second,
        ],
    );
    assert!(
        ok,
        "a same-slug retitle of a committed doc must land; stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let ack: Value = serde_json::from_str(stdout.trim()).expect("the rename ack is one JSON doc");
    assert_eq!(ack["reslugged"], Value::Bool(false), "the slug is frozen");
    assert_eq!(ack["target"]["slug"], "adopt-redis");
    let staged = corpus
        .repo()
        .join(".jigc/tasks")
        .join(&second)
        .join("docs/adr:adopt-redis.md");
    let body = fs::read_to_string(&staged).expect("read the staged copy");
    assert!(
        body.contains("# Adopt Redis!"),
        "the retitle rewrote the `# H1` in place:\n{body}"
    );

    // (ii) a divergent `--to` blocks, and its route runs verbatim.
    let (ok, stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "rename",
            "adr:adopt-redis",
            "--to",
            "Adopt Valkey",
            "--task",
            &second,
        ],
    );
    assert!(
        !ok,
        "a re-slug of a committed identity must block; stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.trim().is_empty(),
        "a blocked write leaves stdout empty; got:\n{stdout}"
    );
    let finding = blocking_finding(&stderr, "committed re-slug");
    assert_eq!(
        finding["code"], "write.identity-change",
        "the committed re-slug converges on the shipped identity-change code:\n{stderr}"
    );
    assert!(
        staged.is_file(),
        "the refused re-slug leaves the staged copy where it was"
    );
    assert!(
        !corpus
            .repo()
            .join(".jigc/tasks")
            .join(&second)
            .join("docs/adr:adopt-valkey.md")
            .exists(),
        "the refused re-slug stages nothing at the new identity"
    );

    let route = finding["route"]
        .as_str()
        .expect("the blocking finding carries a route")
        .to_string();
    let cmd = backticked(&route, "committed re-slug").to_string();
    assert!(
        cmd.starts_with("jigc rename "),
        "the route names the task-less identity op; got: {route}"
    );

    // Run it **verbatim** — after the precondition the route's own tail states.
    corpus.jigc_ok(&["task", "discard", &second, "--force"]);
    let argv = shell_split(&corpus, &cmd);
    let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    corpus.jigc_ok(&args);
    assert!(
        corpus
            .repo()
            .join("docs/decisions/adopt-valkey.md")
            .is_file(),
        "the emitted route, run verbatim, performs the identity move"
    );
}

// ─────────── the metachar axis — a route is bytes a shell will parse ───────────

/// The title the two metachar arms below carry. A title is **author-owned prose**, so a
/// `$` and a command substitution are input these doors are designed to receive — and
/// both are live inside the double quotes the pre-M47 rendering form emits, which is how
/// a route can rename a document to something nobody authored *at exit 0*. (The backtick
/// form of substitution is deliberately absent: a route renders its argv inside a
/// backticked code span, so a backticked title cannot be lifted back out of the emitted
/// text at all — a presentation question of its own, not one shell-quoting answers.)
const METACHAR_TITLE: &str = "Cache $HOME $(touch PWNED) rework";

/// The file the embedded command creates if a real shell parses the emitted route.
const PWNED: &str = "PWNED";

/// **The argument-shape reject's route is shell bytes** (`jigc doc rename <addr>#<hop>`).
/// The reject hands back the same call with the fragment dropped, `--to <title>` and all
/// — so the title it re-emits has to survive `sh`. Run through a real shell, the route
/// must land the title that was asked for, byte for byte, and run nothing else.
#[test]
fn the_fragment_reject_route_survives_a_real_shell() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("single-task", "pick the cache");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Adopt Redis",
        "--task",
        &task,
    ]);

    let out = corpus.jigc(&[
        "doc",
        "rename",
        "adr:adopt-redis#context",
        "--to",
        METACHAR_TITLE,
        "--task",
        &task,
    ]);
    assert!(
        !out.status.success(),
        "a fragment address at a whole-doc verb is refused"
    );
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let route = stderr
        .split("route: run ")
        .nth(1)
        .unwrap_or_else(|| panic!("the reject carries a route; got:\n{stderr}"));
    let cmd = backticked(route.trim(), "fragment reject").to_string();

    let argv = shell_split(&corpus, &cmd);
    let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    corpus.jigc_ok(&args);

    // Read the staged doc back off the working area rather than at a reconstructed
    // address: the rename re-slugs an uncommitted identity, and the slug is derived from
    // the very title under test.
    let staged = files_under(&corpus.repo().join(".jigc/tasks").join(&task).join("docs"));
    let adrs: Vec<&(String, String)> = staged
        .iter()
        .filter(|(path, _)| path.contains("/adr:"))
        .collect();
    assert_eq!(
        adrs.len(),
        1,
        "the task stages exactly one adr; got: {adrs:?}"
    );
    assert!(
        adrs[0].1.contains(&format!("# {METACHAR_TITLE}\n")),
        "the emitted route, parsed by a real shell, must land the title that was asked \
         for — not one the shell rewrote; got:\n{}",
        adrs[0].1
    );
    assert!(
        !corpus.repo().join(PWNED).exists(),
        "the emitted route must not execute a command embedded in the title"
    );
}

/// **The committed-re-slug refusal's route is shell bytes too** — and this one is a
/// `jigc rename`, i.e. a **self-committing store op**: a shell-rewritten title there
/// lands a wrong `# H1` and a wrong path in the repo's history, repointing every
/// referrer at it.
#[test]
fn the_committed_reslug_route_survives_a_real_shell() {
    let corpus = TrialCorpus::build(State::Fresh);
    let first = corpus.start_workflow("single-task", "pick the cache");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Adopt Redis",
        "--task",
        &first,
    ]);
    for (section, prose) in [
        ("context", "Reads are hot."),
        ("decision", "Use Redis."),
        ("consequences", "One more service."),
    ] {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("adr:adopt-redis#{section}"),
                "--from-file",
                "-",
                "--task",
                &first,
            ],
            prose,
        );
    }
    corpus.finalize(&first, "cache", "pick the cache", false);

    let second = corpus.start_workflow("single-task", "revisit the cache");
    let (ok, _stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "rename",
            "adr:adopt-redis",
            "--to",
            METACHAR_TITLE,
            "--task",
            &second,
        ],
    );
    assert!(!ok, "a re-slug of a committed identity blocks");
    let finding = blocking_finding(&stderr, "committed re-slug, metachar title");
    let route = finding["route"]
        .as_str()
        .expect("the blocking finding carries a route")
        .to_string();
    let cmd = backticked(&route, "committed re-slug, metachar title").to_string();

    // The route's own stated precondition, then the emitted bytes through a real shell.
    corpus.jigc_ok(&["task", "discard", &second, "--force"]);
    let argv = shell_split(&corpus, &cmd);
    let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    corpus.jigc_ok(&args);

    let decisions = files_under(&corpus.repo().join("docs/decisions"));
    assert_eq!(
        decisions.len(),
        1,
        "the store holds exactly one decision — the moved one; got: {:?}",
        decisions.iter().map(|(p, _)| p).collect::<Vec<_>>()
    );
    assert!(
        decisions[0].1.contains(&format!("# {METACHAR_TITLE}\n")),
        "the emitted route, parsed by a real shell, must commit the title that was asked \
         for; got:\n{}",
        decisions[0].1
    );
    assert!(
        !corpus.repo().join(PWNED).exists(),
        "the emitted route must not execute a command embedded in the title"
    );
}

// ─────────── arms C/D — the refusing cells, enumerated from the census ───────────

/// **Cell C + the transient sink.** Every doctype whose identity the CLI supplies —
/// the `placement`/`display-title` singletons and the transient `commit` sink —
/// refuses the verb outright, non-zero, with a route. The set is read from the
/// census, so a singleton added later is refused by this same arm.
#[test]
fn every_fixed_identity_doctype_refuses_the_rename() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("single-task", "poke the singletons");

    let cells: Vec<String> = ids_in(Cell::FixedIdentity)
        .into_iter()
        .chain(ids_in(Cell::Transient))
        .collect();
    assert!(!cells.is_empty(), "the refusing cells are populated");

    for ty in cells {
        // A transient sink's only instance is the task's own commit doc.
        let addr = if ty == "commit" {
            format!("commit:{task}")
        } else {
            ty.clone()
        };
        let (ok, stdout, stderr) = json(
            &corpus,
            &[
                "doc",
                "rename",
                &addr,
                "--to",
                "Something Else",
                "--task",
                &task,
            ],
        );
        assert!(
            !ok,
            "`doc rename {addr}` must refuse — `{ty}`'s identity is CLI-supplied; stdout:\n{stdout}"
        );
        let finding = blocking_finding(&stderr, &format!("{ty} refusal"));
        assert_eq!(
            finding["code"], "write.identity-change",
            "`{ty}`'s refusal converges on the identity-change code:\n{stderr}"
        );
        assert!(
            finding["route"].as_str().is_some_and(|r| !r.is_empty()),
            "`{ty}`'s refusal names a route (the route floor):\n{stderr}"
        );
    }
}

/// **Cell D.** `milestone-record` is refused by the **shipped**
/// `machine_maintained_guard` — called, not re-implemented — while every *other*
/// slug-identity doctype passes the doctype gate and fails only on the absent
/// instance. That discrimination is what proves the shipped guard does the work.
#[test]
fn the_milestone_record_refuses_through_the_shipped_machine_maintained_guard() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("single-task", "poke the slug-identity set");

    for ty in ids_in(Cell::SlugIdentity) {
        let addr = format!("{ty}:no-such-instance");
        let (ok, _stdout, stderr) = json(
            &corpus,
            &[
                "doc",
                "rename",
                &addr,
                "--to",
                "Something Else",
                "--task",
                &task,
            ],
        );
        assert!(
            !ok,
            "`doc rename {addr}` cannot succeed — nothing is staged"
        );
        if ty == "milestone-record" {
            let finding = blocking_finding(&stderr, "milestone-record refusal");
            assert_eq!(
                finding["code"], "write.machine-maintained",
                "the record is refused by the shipped doctype guard:\n{stderr}"
            );
            assert!(
                finding["message"]
                    .as_str()
                    .is_some_and(|m| m.contains("milestone-record")),
                "the shipped guard's own sentence is what fires:\n{stderr}"
            );
        } else {
            assert!(
                !stderr.contains("write.machine-maintained"),
                "`{ty}` must pass the doctype gate and fail on the absent instance:\n{stderr}"
            );
        }
    }
}

// ───── arm F — the fixed-identity axis, past the shipped-pack knob coincidence ─────

/// Seed a **listed fixture pack** carrying a `singleton: true` doctype that declares
/// `location:` and *neither* `placement:` nor `display-title:` — the shape
/// `singleton_running_doc.rs` has driven through the real binary since M16, and the one
/// cell no shipped doctype occupies. The pack lives outside the repo and is named in the
/// in-repo `.jigc/config/packs.yaml`, so it UNIONs with the embedded packs (the flow-18
/// listed-pack seam): the base supplies `commit` + knobs, the fixture the singleton and a
/// `creates-task: true` workflow whose create-gate admits it.
fn seed_location_singleton_pack(corpus: &TrialCorpus) {
    let pack = corpus
        .repo()
        .parent()
        .expect("the corpus root")
        .join("pack");
    for sub in ["schemas", "workflows", "steps", "config"] {
        fs::create_dir_all(pack.join(sub)).expect("mk fixture pack subdir");
    }
    fs::write(
        pack.join("schemas/runlog.yaml"),
        "type: runlog\n\
         singleton: true\n\
         location: runlog/\n\
         description: A fixture running log maintained as a singleton over time.\n\
         usage: the fixture cell no shipped doctype occupies — a singleton with a location.\n\
         sections:\n\
         \x20 - id: overview\n\
         \x20   slot: {}\n",
    )
    .expect("seed the runlog schema");
    fs::write(
        pack.join("workflows/keep-runlog.yaml"),
        "---\n\
         when: maintain the fixture running log\n\
         description: A fixture workflow that creates-or-updates the runlog singleton.\n\
         usage: proving the fixed-identity refusal over a location-bearing singleton.\n\
         creates-task: true\n\
         allows-create: [{type: runlog, as: log}]\n\
         ---\n\
         {{ include: step:keep }}\n",
    )
    .expect("seed the keep-runlog workflow");
    fs::write(
        pack.join("steps/keep.yaml"),
        "Maintain the running log for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed the keep step");
    fs::write(pack.join("config/commands.yaml"), "commands: []\n").expect("seed empty catalog");
    // A listed pack that ships steps owes the four ambush-class statements since M51
    // Increment 8 T2 — the stated-at fence checks each step-shipping constituent in
    // isolation, so the dev pack's declarers do not answer for this one.
    crate::support::seed_ambush_class_declarer(&pack);

    fs::write(
        corpus.repo().join(".jigc/config/packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the fixture pack");
}

/// **The axis, not the shipped instance.** Every singleton in either shipped pack
/// happens to carry `placement:` *and* `display-title:`, so a refusal keyed on those two
/// knobs stays green over the whole registry while missing the rule it claims to enforce.
/// The rule is [`Schema::fixed_title`] — **`singleton: true`**: the slug IS the type id,
/// whatever knobs the schema declares. Unrefused on the location-bearing cell, `doc
/// rename` re-slugs the singleton, rebinds the task's role to the new identity, and
/// finalize promotes it OFF its canonical path at exit 0 — after which the next `doc
/// create <ty>` mints a *second* document at the canonical one and `jigc validate`
/// reports the fork clean. The same doctype is refused correctly at the create door
/// (`write.title-ignored`), which is the divergence this arm closes.
#[test]
fn a_location_bearing_singleton_refuses_the_rename() {
    let corpus = TrialCorpus::build(State::Fresh);
    seed_location_singleton_pack(&corpus);
    let task = corpus.start_workflow("keep-runlog", "keep the log");
    corpus.jigc_ok(&[
        "doc", "create", "runlog", "--title", "runlog", "--task", &task,
    ]);

    let (ok, stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "rename",
            "runlog:runlog",
            "--to",
            "Something Else",
            "--task",
            &task,
        ],
    );
    assert!(
        !ok,
        "`doc rename runlog:runlog` must refuse — a singleton's slug IS its type id, \
         whether it homes at a `placement` literal or under a `location`; stdout:\n{stdout}"
    );
    let finding = blocking_finding(&stderr, "location-bearing singleton refusal");
    assert_eq!(
        finding["code"], "write.identity-change",
        "the location-bearing singleton joins the shipped refusal, same code:\n{stderr}"
    );
    assert!(
        finding["route"].as_str().is_some_and(|r| !r.is_empty()),
        "the refusal names a route (the route floor):\n{stderr}"
    );
    // Law 1 over the cell that has no `display-title:` — the sentence may not attribute
    // the `# H1` to a knob this schema never declares.
    let message = finding["message"].as_str().unwrap_or_default();
    assert!(
        !message.contains("display-title"),
        "a singleton without `display-title:` must not be told its H1 comes from one:\n{stderr}"
    );

    // The identity did not move: the task still binds the canonical address and the
    // staged file still sits at the canonical slug.
    let task_dir = corpus.repo().join(".jigc/tasks").join(&task);
    let roles = fs::read_to_string(task_dir.join("roles.json")).expect("the task's bound roles");
    assert!(
        roles.contains("runlog:runlog"),
        "the refused rename left the role bound to the canonical identity; got:\n{roles}"
    );
    for (path, _) in files_under(&task_dir) {
        assert!(
            !path.contains("something-else"),
            "the refused rename staged nothing under the new slug: {path}"
        );
    }
}

// ─── arm G — the retitle-only ack names the cell it is in, over both provenances ───

/// **The retitle-only sentence, swept over the provenance axis.** `reslugged: false`
/// is reached from **both** cells of the split the verb makes, not just the committed
/// one:
///
///   * `EditedFromBase` — a committed doc copied in for edit, whose slug is frozen
///     because it is the committed path (arm B);
///   * `Created` (or no provenance record at all) — a doc this task minted, where the
///     `--to` simply lands on the id the doc already has. This is the **primary route
///     path** of the increment's own `write.title-ignored` guard: a punctuation-only
///     correction routes here, and the doc it corrects was never committed.
///
/// One ack sentence served both, claiming "the committed identity keeps its slug"
/// over a doc with no committed identity — `design/surface-contract.md` law 1, on the
/// route the guard emits. So the assertion is per cell, and the never-committed cell
/// is driven **through the emitted route, run verbatim**, not a reconstruction of it.
#[test]
fn the_retitle_only_ack_names_the_cell_it_is_in() {
    let corpus = TrialCorpus::build(State::Fresh);

    // ── cell 1: never committed — reached through the title guard's own route ──
    let minting = corpus.start_workflow("single-task", "pick the queue");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Adopt Kafka",
        "--task",
        &minting,
    ]);
    let (ok, _stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Adopt Kafka!",
            "--task",
            &minting,
        ],
    );
    assert!(!ok, "the punctuation-only re-create is refused");
    let finding = blocking_finding(&stderr, "title ignored");
    assert_eq!(
        finding["code"], "write.title-ignored",
        "the guard whose route this arm runs:\n{stderr}"
    );
    let route = finding["route"]
        .as_str()
        .expect("the blocking finding carries a route")
        .to_string();
    let cmd = backticked(&route, "title ignored").to_string();

    // The cell's precondition, read off the task's own provenance manifest and the
    // committed store — the doc the route is about was never committed.
    let task_dir = corpus.repo().join(".jigc/tasks").join(&minting);
    let provenance = fs::read_to_string(task_dir.join("docs/provenance.json"))
        .expect("read the staged-doc provenance");
    let recorded: Value = serde_json::from_str(&provenance).expect("the manifest is JSON");
    assert_eq!(
        recorded["docs"]["adr:adopt-kafka"], "created",
        "the cell's precondition: this task minted the doc:\n{provenance}"
    );
    assert!(
        !corpus.repo().join("docs/decisions/adopt-kafka.md").exists(),
        "the cell's precondition: nothing is committed at the doc's home"
    );

    // Run the emitted bytes verbatim and read the ack the agent actually sees.
    let argv = shell_split(&corpus, &cmd);
    let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    let line = corpus.jigc_ok(&args).trim().to_string();
    assert!(
        line.starts_with("adr:adopt-kafka (retitled \"Adopt Kafka!\""),
        "the retitle-only ack leads with the address and the new title; got:\n{line}"
    );
    assert!(
        !line.contains("committed"),
        "law 1: the ack may not claim a committed identity over a doc this task minted \
         and never committed; got:\n{line}"
    );
    assert!(
        line.contains("the id is unchanged"),
        "the ack states the reason this cell actually has — the id did not move; got:\n{line}"
    );

    // ── cell 2: a committed identity — the frozen-slug sentence still fires ──
    for (section, prose) in [
        ("context", "Fan-out is growing."),
        ("decision", "Use Kafka."),
        ("consequences", "One more broker to run."),
    ] {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("adr:adopt-kafka#{section}"),
                "--from-file",
                "-",
                "--task",
                &minting,
            ],
            prose,
        );
    }
    corpus.finalize(&minting, "queue", "pick the queue", false);
    assert!(
        corpus
            .repo()
            .join("docs/decisions/adopt-kafka.md")
            .is_file(),
        "the second cell's precondition: the ADR is committed"
    );

    let editing = corpus.start_workflow("single-task", "revisit the queue");
    let line = corpus
        .jigc_ok(&[
            "doc",
            "rename",
            "adr:adopt-kafka",
            "--to",
            "Adopt Kafka?",
            "--task",
            &editing,
        ])
        .trim()
        .to_string();
    let provenance = fs::read_to_string(
        corpus
            .repo()
            .join(".jigc/tasks")
            .join(&editing)
            .join("docs/provenance.json"),
    )
    .expect("read the editing task's provenance");
    let recorded: Value = serde_json::from_str(&provenance).expect("the manifest is JSON");
    assert_eq!(
        recorded["docs"]["adr:adopt-kafka"], "edited-from-base",
        "the second cell's precondition: the doc was copied in from the committed store:\n{provenance}"
    );
    assert!(
        line.starts_with("adr:adopt-kafka (retitled \"Adopt Kafka?\"")
            && line.contains("the committed identity keeps its slug"),
        "a committed identity still states why its slug is frozen; got:\n{line}"
    );
}

/// **Cell E, unchanged.** The item-level identity refusal the doc-level verb owes
/// nothing but a regression arm: `retitle-item` on an **enum** `id-from` (a
/// changelog change-group's `category`) still refuses with `write.identity-change`.
#[test]
fn retitle_item_still_refuses_an_enum_id_from() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-change", "cut a release");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "changelog",
        "--title",
        "Changelog",
        "--task",
        &task,
    ]);
    let group = corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            "changelog:changelog#unreleased-changes",
            "--title",
            "Added",
            "--task",
            &task,
        ])
        .trim()
        .to_string();

    let (ok, _stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "retitle-item",
            &group,
            "--title",
            "Changed",
            "--task",
            &task,
        ],
    );
    assert!(!ok, "an enum id-from retitle still refuses");
    let finding = blocking_finding(&stderr, "enum id-from retitle");
    assert_eq!(
        finding["code"], "write.identity-change",
        "the shipped item-level refusal is unchanged:\n{stderr}"
    );
}

// ───────── arm F — the destination-occupancy axis, over both of its homes ─────────

/// Where a doc answering to the **destination** identity can already live. A re-slug
/// has to be refused over **both** homes, because both are a collision: the task's
/// own working area (moving onto it discards that doc's authored content) and the
/// committed store (promoting onto it overwrites the committed file at finalize).
/// The axis is this cell set — the reported repro was one cell of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Occupant {
    /// Nothing answers to the destination identity — the re-slug lands.
    Absent,
    /// Another doc **staged in this task's working area** holds it.
    Staged,
    /// A **committed** doc in the store holds it.
    Committed,
}

impl Occupant {
    /// The whole axis. The arm below matches it **exhaustively**, so a fourth home
    /// cannot be added without a cell here.
    const ALL: [Occupant; 3] = [Occupant::Absent, Occupant::Staged, Occupant::Committed];

    /// The `--to` title the re-slug aims at, per cell.
    fn destination_title(self) -> &'static str {
        match self {
            Occupant::Absent => "Adopt NATS",
            Occupant::Staged => "Adopt Kafka",
            Occupant::Committed => "Adopt Redis",
        }
    }

    /// The `<type>:<slug>` identity that title mints — the destination.
    fn destination_uri(self) -> &'static str {
        match self {
            Occupant::Absent => "adr:adopt-nats",
            Occupant::Staged => "adr:adopt-kafka",
            Occupant::Committed => "adr:adopt-redis",
        }
    }

    /// The slug of the doc this task stages and then tries to move — one per cell, so
    /// a cell that finalizes cannot seed the next one's committed store.
    fn source_slug(self) -> &'static str {
        match self {
            Occupant::Absent => "broker-choice-a",
            Occupant::Staged => "broker-choice-b",
            Occupant::Committed => "broker-choice-c",
        }
    }
}

/// The free id the occupancy route's `<other-slug>` span is filled with when the arm
/// below runs that route verbatim.
const FREE_SLUG: &str = "broker-rethink";

/// **Every** backticked `jigc` command in a route, in emission order — the occupancy
/// refusal names two exits, and a route floor that only ever checks the first one is a
/// floor under half the surface. A backticked span that is not a command (a route also
/// backticks the addresses it names) is not one of them.
fn backticked_all(route: &str) -> Vec<String> {
    route
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.starts_with("jigc "))
        .map(str::to_string)
        .collect()
}

/// Author a complete `adr` into `task` at an explicit `slug`, through the real verbs.
fn author_adr_at(corpus: &TrialCorpus, task: &str, title: &str, slug: &str) {
    corpus.jigc_ok(&[
        "doc", "create", "adr", "--title", title, "--slug", slug, "--task", task,
    ]);
    for (section, prose) in [
        ("context", "Queue depth is unbounded."),
        ("decision", "Adopt a broker."),
        ("consequences", "One more service to run."),
    ] {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("adr:{slug}#{section}"),
                "--from-file",
                "-",
                "--task",
                task,
            ],
            prose,
        );
    }
}

/// **The re-slug's destination guard answers over both homes.** A never-committed
/// staged doc may move its identity inside the task — but only onto an identity
/// nothing already answers to. The **staged** home was guarded; the **committed** one
/// was not, so a re-slug onto a committed identity acked success at exit 0, `task
/// validate` stayed clean, and only `finalize` refused with
/// `finalize.promote-clobber` — a write ack over a state the task cannot complete.
///
/// The arm iterates [`Occupant::ALL`]: the free cell still lands, and **both**
/// occupied cells block at the write with `write.already-present` keyed at the
/// destination identity, carrying a route, staging nothing there and leaving the
/// source doc byte-untouched — a refused write is not a partial one.
#[test]
fn the_reslug_destination_guard_answers_over_both_homes() {
    let corpus = TrialCorpus::build(State::Fresh);

    // The committed home's occupant.
    let first = corpus.start_workflow("single-task", "pick the cache");
    author_adr_at(&corpus, &first, "Adopt Redis", "adopt-redis");
    corpus.finalize(&first, "cache", "pick the cache", false);
    assert!(
        corpus
            .repo()
            .join("docs/decisions/adopt-redis.md")
            .is_file(),
        "the committed occupant is in place — the Committed cell's precondition"
    );

    for cell in Occupant::ALL {
        let task = corpus.start_workflow("single-task", "pick the broker");
        let source_slug = cell.source_slug();
        author_adr_at(&corpus, &task, "Pick a broker", source_slug);

        let task_dir = corpus.repo().join(".jigc/tasks").join(&task);
        let source = task_dir.join(format!("docs/adr:{source_slug}.md"));
        let before = fs::read_to_string(&source).expect("read the staged source doc");
        let dest_staged = task_dir.join(format!("docs/{}.md", cell.destination_uri()));

        // The staged home's occupant is placed **into the working area directly** —
        // the state-derived producer the guard has to answer for (the area is a plain
        // directory, and the pre-1.0 trial found every blind session reaching into it).
        // Every gated create door binds one `as:` role, so no shipped door mints a
        // second `adr` into one task; the guard is about the state, not the door.
        if cell == Occupant::Staged {
            fs::write(
                &dest_staged,
                before.replace("# Pick a broker", "# Adopt Kafka"),
            )
            .expect("place the staged occupant");
        }
        let occupant_before = (cell == Occupant::Staged)
            .then(|| fs::read_to_string(&dest_staged).expect("read the staged occupant"));

        let (ok, stdout, stderr) = json(
            &corpus,
            &[
                "doc",
                "rename",
                &format!("adr:{source_slug}"),
                "--to",
                cell.destination_title(),
                "--task",
                &task,
            ],
        );

        if cell == Occupant::Absent {
            assert!(
                ok,
                "{cell:?}: a free destination must still land; stdout:\n{stdout}\nstderr:\n{stderr}"
            );
            assert!(
                dest_staged.is_file() && !source.exists(),
                "{cell:?}: the identity moved to the free destination"
            );
            corpus.jigc_ok(&["task", "discard", &task, "--force"]);
            continue;
        }

        assert!(
            !ok,
            "{cell:?}: an occupied destination must block at the write, not at finalize; \
             stdout:\n{stdout}\nstderr:\n{stderr}"
        );
        assert!(
            stdout.trim().is_empty(),
            "{cell:?}: a blocked write leaves stdout empty; got:\n{stdout}"
        );
        let finding = blocking_finding(&stderr, "occupied destination");
        assert_eq!(
            finding["code"], "write.already-present",
            "{cell:?}: the occupancy refusal keys on the shipped code:\n{stderr}"
        );
        assert_eq!(
            finding["key"]["target"],
            cell.destination_uri(),
            "{cell:?}: the refusal keys at the destination identity:\n{stderr}"
        );
        // A refused write is not a partial one: the source keeps its identity AND its
        // bytes (its `# H1` is not pre-rewritten), and the occupant is untouched.
        assert_eq!(
            fs::read_to_string(&source).expect("the source doc survives the refusal"),
            before,
            "{cell:?}: the refused rename leaves the source doc byte-untouched"
        );
        match occupant_before {
            Some(occupant) => assert_eq!(
                fs::read_to_string(&dest_staged).expect("the staged occupant survives"),
                occupant,
                "{cell:?}: the staged occupant is not overwritten"
            ),
            None => assert!(
                !dest_staged.exists(),
                "{cell:?}: nothing is staged at the refused destination"
            ),
        }

        // The route floor for a blocking finding: every command the route emits is
        // bytes a real shell parses and this binary runs — asserted on the **emitted**
        // route, run verbatim, with the free slug (the one span that is the agent's to
        // fill) substituted.
        let route = finding["route"]
            .as_str()
            .unwrap_or_else(|| panic!("{cell:?}: a blocking finding names its recovery:\n{stderr}"))
            .to_string();
        for emitted in backticked_all(&route) {
            let cmd = emitted.replace("<other-slug>", FREE_SLUG);
            let argv = shell_split(&corpus, &cmd);
            let mut args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
            // The rename recovery is task-scoped; the `doc show` read is not.
            if args.first() == Some(&"doc") && args.get(1) == Some(&"rename") {
                args.extend_from_slice(&["--task", &task]);
            }
            let out = corpus.jigc(&args);
            assert!(
                out.status.success(),
                "{cell:?}: the emitted route `{cmd}` must run; stdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            );
        }
        // The recovery did what it said it would.
        assert!(
            task_dir.join(format!("docs/adr:{FREE_SLUG}.md")).is_file() && !source.exists(),
            "{cell:?}: the emitted recovery moved the doc to the free id"
        );

        if cell == Occupant::Committed {
            // The refusal leaves a state the committing door agrees with — the
            // contract the exit-0 ack broke: with the task otherwise complete,
            // `task validate` previews clean and `finalize` lands the doc at the
            // identity it kept. (Before the fix, the two disagreed: the rename acked,
            // `validate` stayed clean, and `finalize` refused with
            // `finalize.promote-clobber`.)
            for (field, value) in [("type", "docs"), ("scope", "broker")] {
                corpus.jigc_ok(&[
                    "doc",
                    "set-field",
                    &format!("commit:{task}#{field}"),
                    "--value",
                    value,
                    "--task",
                    &task,
                ]);
            }
            for (slot, prose) in [("summary", "pick the broker"), ("body", "The broker call.")] {
                corpus.jigc_stdin_ok(
                    &[
                        "doc",
                        "set-slot",
                        &format!("commit:{task}#{slot}"),
                        "--from-file",
                        "-",
                        "--task",
                        &task,
                    ],
                    prose,
                );
            }
            let validate = corpus.jigc(&["task", "validate", &task]);
            assert!(
                validate.status.success(),
                "{cell:?}: `task validate` and `finalize` must agree; stdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&validate.stdout),
                String::from_utf8_lossy(&validate.stderr),
            );
            corpus.jigc_ok(&["task", "finalize", &task]);
            assert!(
                corpus
                    .repo()
                    .join(format!("docs/decisions/{FREE_SLUG}.md"))
                    .is_file(),
                "{cell:?}: finalize lands the doc at the id the route named"
            );
            let occupant = fs::read_to_string(corpus.repo().join("docs/decisions/adopt-redis.md"))
                .expect("the committed occupant survives");
            assert!(
                occupant.contains("# Adopt Redis"),
                "{cell:?}: the committed occupant is still there, unclobbered:\n{occupant}"
            );
        } else {
            corpus.jigc_ok(&["task", "discard", &task, "--force"]);
        }
    }
}

/// **The committed cell's carve-out, and the parity that earns it.** A migration whose
/// recorded `source-path` IS the destination's own canonical path is the **in-place
/// rewrite**: the committed file there is the very foreign original being replaced,
/// which is why `finalize`'s clobber guard carves it out
/// (`engine::finalize::plan_clobber_guard`, the M43 same-path carve-out). The write
/// guard reads the committed home through the shipped [`state::create_incumbent`]
/// predicate, whose committed arm reproduces that same carve-out — so the write
/// refuses exactly what `finalize` refuses, and a rename onto the migration's own
/// destination still lands.
#[test]
fn a_rename_onto_a_migrations_own_destination_is_not_a_collision() {
    let corpus = TrialCorpus::build(State::Fresh);
    fs::create_dir_all(corpus.repo().join("docs/decisions")).expect("create the decisions dir");
    fs::write(
        corpus.repo().join("docs/decisions/legacy-store.md"),
        "# Legacy store\n\nWe kept the legacy store, undocumented.\n",
    )
    .expect("write the foreign decision");
    corpus.git(&["add", "docs/decisions/legacy-store.md"]);
    corpus.git(&["commit", "-q", "-m", "the foreign decision"]);

    let composed = corpus.jigc_ok(&[
        "migrate",
        "docs/decisions/legacy-store.md",
        "--as",
        "adr",
        "--slug",
        "holding-id",
    ]);
    let task = composed
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .expect("the migrate mint prints its task id")
        .trim()
        .to_string();
    corpus.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        ADR_PAYLOAD,
    );

    let (ok, stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "rename",
            "adr:holding-id",
            "--to",
            "Legacy store",
            "--task",
            &task,
        ],
    );
    assert!(
        ok,
        "a re-slug onto the migration's own in-place destination must land — the committed \
         file there is the original being rewritten; stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        corpus
            .repo()
            .join(".jigc/tasks")
            .join(&task)
            .join("docs/adr:legacy-store.md")
            .is_file(),
        "the staged doc moved to the in-place identity"
    );
}

// ───────── arm H — the rename moves the identity and nothing else ─────────

/// **The conversion-ledger row plant E left open** (`RC-m50/findings-verification.md`
/// → *The conversion ledger — closed*, the last row, which cites this arm as its
/// `pinned-by:`; M50 Increment 13 / T1). Two blind
/// workers (R3, B3) repaired plant E's contradicting title through `jigc doc rename`
/// on a staged doc that already carried authored content — the header fields and the
/// slot prose. Every arm above asserts what the rename **moves** (the file, the `# H1`,
/// the bound role, the provenance manifest, a staged referrer's ref field) and what it
/// **refuses**; none asserted what it must leave **untouched**, so a rename that
/// re-stamped the `on-create` date, dropped an unset-then-set header field, or
/// re-rendered a slot would have passed the whole suite.
///
/// The set iterated is **the doctype's own declared leaf surface, read off the loaded
/// schema** — every header field id and every slot-bearing section id of `adr` — so
/// the fixture is *maximal by fence*: a leaf added to the schema reddens the coverage
/// assertion until the fixture authors it, rather than silently leaving a leaf
/// unpinned. `options` is deliberately left empty: an optional slot that was empty
/// must survive as empty, not vanish and not gain filler.
///
/// The claim is asserted twice, at two altitudes:
///
///   * through the **pinned read contract** — `jigc doc show <addr> --task <id>
///     --format json` before and after are equal **modulo `slug`**, which is the
///     whole claim stated as an equality rather than as a checklist of leaves;
///   * on the **staged bytes** — the file before and after differ in **exactly one
///     line**, and that line is the `# H1`. The JSON projection erases blank-line
///     structure and trailing space; the byte fence does not.
#[test]
fn a_rename_preserves_every_header_field_and_slot_body_of_the_staged_doc() {
    let corpus = TrialCorpus::build(State::Fresh);

    // A committed `adr` for the `supersedes` ref to point at — the create-gate binds
    // one `decision` role per task, so the ref target cannot be a second in-task doc.
    let earlier = corpus.start_workflow("single-task", "pick the cache");
    author_adr_at(&corpus, &earlier, "Adopt Redis", "adopt-redis");
    corpus.finalize(&earlier, "cache", "pick the cache", false);

    // A real symbol for the `code-anchor` field, so the fixture's fourth header leaf
    // carries a value that resolves rather than a plausible-looking literal.
    fs::create_dir_all(corpus.repo().join("src")).expect("create the src dir");
    fs::write(
        corpus.repo().join("src/pad.ts"),
        "export function pad(n: number): string {\n  return String(n);\n}\n",
    )
    .expect("write the anchored source file");

    let task = corpus.start_workflow("single-task", "reject the newest sample");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Reject The Newest Sample",
        "--task",
        &task,
    ]);
    let old_uri = "adr:reject-the-newest-sample";

    // Every author-settable header leaf `adr` declares. `date` is `set: on-create`
    // and is therefore *not* written here — it is the leaf a re-stamping rename
    // would silently move, and the equality below is what catches that.
    for (leaf, value) in [
        ("status", "accepted"),
        ("supersedes", "adr:adopt-redis"),
        ("cites-code", "src/pad.ts#pad"),
    ] {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("{old_uri}#status/{leaf}"),
            "--value",
            value,
            "--task",
            &task,
        ]);
    }
    // Three of the four slots are authored; `options` stays empty on purpose.
    let authored: BTreeMap<&str, &str> = BTreeMap::from([
        ("context", "The newest sample is the noisiest."),
        ("decision", "Prefer the oldest sample in the window."),
        ("consequences", "The window has to be bounded."),
    ]);
    for (section, prose) in &authored {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("{old_uri}#{section}"),
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            prose,
        );
    }

    let show = |addr: &str| -> Value {
        let (ok, stdout, stderr) = json(&corpus, &["doc", "show", addr, "--task", &task]);
        assert!(
            ok,
            "`doc show {addr} --task` must serve the staged doc; stdout:\n{stdout}\nstderr:\n{stderr}"
        );
        serde_json::from_str(stdout.trim()).expect("the read contract emits one JSON doc")
    };
    let before = show(old_uri);

    // ── the coverage fence: the fixture covers the doctype's whole declared surface ──
    let (schema, _) = census().remove("adr").expect("`adr` is a loaded doctype");
    let mut declared_fields = Vec::new();
    let mut declared_slots = Vec::new();
    for section in &schema.sections {
        if let engine::schema::SectionBody::Simple { slot, fields } = &section.body {
            if slot.is_some() {
                declared_slots.push(section.id.clone());
            }
            declared_fields.extend(fields.iter().map(|f| f.id.clone()));
        }
    }
    assert!(
        !declared_fields.is_empty() && !declared_slots.is_empty(),
        "`adr` must declare both header fields and slots — the fence has no subject otherwise"
    );
    for field in &declared_fields {
        assert!(
            before["fields"].get(field).is_some(),
            "the fixture must carry a value for every declared header leaf; `{field}` is \
             absent from the read-back, so the rename's effect on it would go unpinned:\n{before:#}"
        );
    }
    for section in &declared_slots {
        assert!(
            before["sections"].get(section).is_some(),
            "every declared slot must read back; `{section}` is absent:\n{before:#}"
        );
    }
    // …and non-vacuously: the read-back carries what was authored, not empty strings.
    assert_eq!(before["fields"]["status"], "accepted");
    assert_eq!(before["fields"]["supersedes"], "adr:adopt-redis");
    assert_eq!(before["fields"]["cites-code"], "src/pad.ts#pad");
    assert!(
        before["fields"]["date"]
            .as_str()
            .is_some_and(|d| !d.is_empty()),
        "the `on-create` date is stamped — the leaf a re-stamping rename would move:\n{before:#}"
    );
    for (section, prose) in &authored {
        assert_eq!(
            before["sections"][section], *prose,
            "the fixture's authored prose reads back before the rename:\n{before:#}"
        );
    }
    assert_eq!(
        before["sections"]["options"], "",
        "the optional slot is empty going in — the case the rename must leave empty"
    );

    let task_dir = corpus.repo().join(".jigc/tasks").join(&task);
    let staged_before = fs::read_to_string(task_dir.join(format!("docs/{old_uri}.md")))
        .expect("read the staged doc");

    let (ok, stdout, stderr) = json(
        &corpus,
        &[
            "doc",
            "rename",
            old_uri,
            "--to",
            "Prefer The Oldest Sample",
            "--task",
            &task,
        ],
    );
    assert!(
        ok,
        "the in-task rename must land; stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let ack: Value = serde_json::from_str(stdout.trim()).expect("the rename ack is one JSON doc");
    assert_eq!(ack["reslugged"], Value::Bool(true), "the identity moved");
    let new_uri = "adr:prefer-the-oldest-sample";
    assert_eq!(ack["target"]["slug"], "prefer-the-oldest-sample");

    // ── the claim, through the pinned read contract ──
    // The identity a rename moves is the slug and the `# H1` it was minted from, and the
    // whole-doc serve carries both (`title`, M55) — so both are stripped here and each
    // is asserted to have moved, to exactly what the rename named, below.
    let after = show(new_uri);
    let strip_identity = |mut doc: Value| -> Value {
        let object = doc
            .as_object_mut()
            .expect("the read contract emits an object");
        object.remove("slug");
        object.remove("title");
        doc
    };
    assert_eq!(
        strip_identity(before.clone()),
        strip_identity(after.clone()),
        "a rename changes the identity and nothing else: every header field and every \
         slot body must read back byte-equal.\nbefore:\n{before:#}\nafter:\n{after:#}"
    );
    assert_ne!(
        before["slug"], after["slug"],
        "…and the identity really did move, or the equality above is trivially true"
    );
    assert_eq!(
        (&before["title"], &after["title"]),
        (
            &Value::from("Reject The Newest Sample"),
            &Value::from("Prefer The Oldest Sample")
        ),
        "…and the served `title` moved from the old H1 to exactly the new one"
    );

    // ── the claim, on the staged bytes ──
    let staged_after = fs::read_to_string(task_dir.join(format!("docs/{new_uri}.md")))
        .expect("read the renamed staged doc");
    let before_lines: Vec<&str> = staged_before.lines().collect();
    let after_lines: Vec<&str> = staged_after.lines().collect();
    assert_eq!(
        before_lines.len(),
        after_lines.len(),
        "the rename must not add or drop a line.\nbefore:\n{staged_before}\nafter:\n{staged_after}"
    );
    let differing: Vec<(usize, &str, &str)> = before_lines
        .iter()
        .zip(after_lines.iter())
        .enumerate()
        .filter(|(_, (b, a))| b != a)
        .map(|(i, (b, a))| (i + 1, *b, *a))
        .collect();
    assert_eq!(
        differing.len(),
        1,
        "exactly one line may differ across the rename; got {differing:?}\nbefore:\n\
         {staged_before}\nafter:\n{staged_after}"
    );
    let (_, was, now) = differing[0];
    assert_eq!(
        was, "# Reject The Newest Sample",
        "the one differing line is the `# H1`"
    );
    assert_eq!(
        now, "# Prefer The Oldest Sample",
        "…rewritten to the new title"
    );

    // The old address is gone from the task area, as every re-slug arm requires.
    assert!(
        !corpus
            .jigc(&["doc", "show", old_uri, "--task", &task])
            .status
            .success(),
        "the old address no longer resolves in the task area"
    );
}

// ──────────────── arm I — the pre-rename identity is durable state ────────────────

/// **The pre-rename title survives the invocation that moved it** (M51 Increment 11 /
/// T1; `RC-rc14/findings-verification.md` → F-9, `settle-record.md` → §21).
///
/// `jigc doc rename --task` rewrites a staged doc's `# H1` and, until now, wrote **no
/// task state at all** — the old title existed only in the renaming process's memory
/// and in the ack it printed once. F-9 is what that costs: the task's staged `commit`
/// doc can already carry the old title in its summary, and the producer that has to
/// notice (the task-scope sweep at `jigc task validate <id>` / `task finalize
/// --dry-run`) runs in a **later invocation** — and, for the pre-commit hook's nested
/// `jigc validate`, in a **different process**. So the pre-rename identity has to be
/// *durable task state* before anything can re-raise it.
///
/// The arm drives the real binary and reads the task-area record back through its own
/// loader, over the axis the rename verb actually has:
///
///   * a **re-slug** (`Rate limiting` → `Throttling`) records the title it moved away
///     from;
///   * a **retitle-only** (`Throttling` → `THROTTLING`, same slug) records it too —
///     the stale summary names the *title*, so keying the record on `reslugged` would
///     ship the fix for half its own axis;
///   * a `--to` equal to the title the doc already carries moved nothing and records
///     nothing — recording it would make the doc's **current** title a stale
///     candidate;
///   * and a task that never renamed has **no key at all**, not an empty one.
///
/// The order is asserted, because the record is a history: the oldest title is the one
/// a summary written at `doc create` time would name.
#[test]
fn the_pre_rename_titles_become_durable_task_state() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("plan", "pin down the throttle");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "spec",
        "--title",
        "Rate limiting",
        "--task",
        &task,
    ]);

    let task_dir = corpus.repo().join(".jigc/tasks").join(&task);
    let record_path = engine::state::RenameRecord::path_in(&task_dir);
    assert!(
        !record_path.exists(),
        "a task that has created but not renamed carries no rename record: {}",
        record_path.display()
    );

    // 1. the re-slug.
    corpus.jigc_ok(&[
        "doc",
        "rename",
        "spec:rate-limiting",
        "--to",
        "Throttling",
        "--task",
        &task,
    ]);
    // 2. the retitle-only — `THROTTLING` slugs to the identity the doc already holds.
    corpus.jigc_ok(&[
        "doc",
        "rename",
        "spec:throttling",
        "--to",
        "THROTTLING",
        "--task",
        &task,
    ]);

    let record = engine::state::RenameRecord::load(&task_dir).expect("the rename record loads");
    assert_eq!(
        record.pre_rename_titles,
        vec!["Rate limiting".to_string(), "Throttling".to_string()],
        "both pre-rename titles survive, oldest first — the re-slug's and the \
         retitle-only's"
    );

    // The on-disk byte form, read as bytes rather than through the loader: the key is
    // what a later process reads back.
    let raw = fs::read_to_string(&record_path).expect("read the rename record");
    let parsed: Value = serde_json::from_str(&raw).expect("the rename record is one JSON doc");
    assert_eq!(
        parsed["pre-rename-titles"],
        serde_json::json!(["Rate limiting", "Throttling"]),
        "the persisted key carries the ordered history:\n{raw}"
    );

    // 3. the no-op retitle — the title the doc already carries is not a stale one.
    corpus.jigc_ok(&[
        "doc",
        "rename",
        "spec:throttling",
        "--to",
        "THROTTLING",
        "--task",
        &task,
    ]);
    assert_eq!(
        engine::state::RenameRecord::load(&task_dir)
            .expect("the rename record loads")
            .pre_rename_titles,
        record.pre_rename_titles,
        "a `--to` equal to the doc's own title moved no `# H1`, so it records nothing"
    );

    // 4. a task that never renamed: no key at all, and the absent read is empty.
    let quiet = corpus.start_workflow("plan", "leave every title exactly as it is");
    let quiet_dir = corpus.repo().join(".jigc/tasks").join(&quiet);
    assert!(
        !engine::state::RenameRecord::path_in(&quiet_dir).exists(),
        "a task that never renamed writes no rename record"
    );
    assert!(
        engine::state::RenameRecord::load(&quiet_dir)
            .expect("an absent rename record loads as empty")
            .pre_rename_titles
            .is_empty(),
        "…and reads back empty, never as an error"
    );
}

// ────────── arm J — the stale commit summary is noticed, and re-raised ──────────

/// The advisory's code — one spelling, read by every assertion below
/// (`design/validation.md` → The M51 registrations — Increment 11).
const STALE_TITLE_CODE: &str = "commit-recording.stale-title";

/// A task whose only outstanding finding can be the stale-title advisory: the spec's two
/// required slots and the commit doc's `type` field are filled, so `task finalize
/// --dry-run` reaches its **forecast** branch rather than its blocked one (a blocked
/// finalize renders the planner's blocking set, which an advisory is by construction not
/// in). Returns the task id; the doc is minted as `Rate limiting` and the commit summary
/// names it.
fn task_with_a_summary_naming(corpus: &TrialCorpus, intent: &str, title: &str) -> String {
    let task = corpus.start_workflow("plan", intent);
    corpus.jigc_ok(&["doc", "create", "spec", "--title", title, "--task", &task]);
    let slug = title.to_lowercase().replace(' ', "-");
    for section in ["goal", "context"] {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("spec:{slug}#{section}"),
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            "bound the request rate",
        );
    }
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#header/type"),
        "--value",
        "feat",
        "--task",
        &task,
    ]);
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        &format!("the {title} spec lands"),
    );
    task
}

/// The two **re-raising** doors, each read as the bytes that door actually emits.
///
/// `jigc task validate` is a text surface and is read as text. `jigc task finalize
/// --dry-run` carries its findings on the **JSON envelope only** — a decision this same
/// wave landed (M51 Inc 5 / T5: the forecast's text shape is unchanged and `task
/// validate` stays the text surface for the findings themselves) — so it is read as
/// JSON. Asserting the dry-run's text arm instead would pin a surface the design
/// deliberately left alone.
fn stale_title_doors(corpus: &TrialCorpus, task: &str) -> Vec<(&'static str, String)> {
    let dry_run = corpus.jigc(&["--format", "json", "task", "finalize", task, "--dry-run"]);
    vec![
        (
            "jigc task validate",
            String::from_utf8_lossy(&corpus.jigc(&["task", "validate", task]).stdout).into_owned(),
        ),
        (
            "jigc task finalize --dry-run --format json",
            String::from_utf8_lossy(&dry_run.stdout).into_owned(),
        ),
    ]
}

/// **A rename leaves the staged commit summary naming the old title, and the product
/// says so — once where it happens, and again at every door until it is repaired**
/// (M51 Increment 11 / T2; `RC-rc14/findings-verification.md` → F-9,
/// `settle-record.md` → §21).
///
/// Driven at `46eef998`, every surface was silent: `doc rename --task` acked `findings:
/// []`, and `task validate` / `task finalize --dry-run` printed the stale subject and
/// the new path on adjacent, unconnected lines. That is the shape the rc.14 trial
/// measured as its **only adapter bypass** — B1 renamed at invocation 18 and finalized
/// at 23 with nothing re-raising it, then rewrote jigc's commit with raw git — so the
/// fix is not a notice printed once but a `Finding` **re-raised from durable state**
/// until the summary is repaired (`cue-card-postmortem.md`: *an instrument fires
/// reliably iff its trigger is a state and its consequence is re-raised by the
/// product*).
///
/// The arm drives the real binary and asserts the **emitted bytes**: the rename ack's
/// `--format json` `findings[]` entry with its key, located address and route, and **no
/// other envelope key moved**; the read surface still returning the stale prose (this
/// reports, it never rewrites); both re-raising doors carrying it; the **emitted route
/// run verbatim** silencing all three; and a task that renamed nothing producing nothing
/// anywhere — the omitting context, where the check must be *inert*, never an error.
#[test]
fn the_stale_commit_summary_is_noticed_where_it_happens_and_re_raised_until_repaired() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = task_with_a_summary_naming(&corpus, "pin down the throttle", "Rate limiting");
    let summary_addr = format!("commit:{task}#summary");

    // Before the rename nothing has moved, so nothing is stale.
    for (door, out) in stale_title_doors(&corpus, &task) {
        assert!(
            !out.contains(STALE_TITLE_CODE),
            "{door} is quiet while no title has moved; got:\n{out}"
        );
    }

    // ── the producer: the rename's own ack ──
    let ack: Value = serde_json::from_str(&corpus.jigc_ok(&[
        "doc",
        "rename",
        "spec:rate-limiting",
        "--to",
        "Throttling",
        "--task",
        &task,
        "--format",
        "json",
    ]))
    .expect("the rename ack is one JSON document");

    let keys: Vec<&str> = ack
        .as_object()
        .expect("the ack is an object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        vec![
            "committed_identity",
            "copied_in",
            "findings",
            "from",
            "op",
            "reslugged",
            "target",
            "title",
        ],
        "the finding rides the `findings` array already on the wire — no envelope key \
         moves for it:\n{ack:#}"
    );
    assert_eq!(
        ack["target"],
        serde_json::json!({"doctype": "spec", "slug": "throttling"}),
        "the ack's own target is still the renamed doc:\n{ack:#}"
    );
    assert_eq!(ack["title"], "Throttling");

    let findings = ack["findings"].as_array().expect("`findings` is an array");
    assert_eq!(
        findings.len(),
        1,
        "one stale summary is one finding, not one per rename:\n{ack:#}"
    );
    let finding = &findings[0];
    assert_eq!(finding["code"], STALE_TITLE_CODE);
    assert_eq!(
        finding["severity"], "advisory",
        "a stale subject is a cost, never a reason to refuse the commit:\n{finding:#}"
    );
    assert_eq!(
        finding["key"],
        serde_json::json!({"code": STALE_TITLE_CODE, "target": summary_addr}),
        "the stable `(code, target)` key is the summary the repair writes"
    );
    assert_eq!(
        finding["location"]["address"], summary_addr,
        "the location is the same address, so every text surface renders an `at:`"
    );
    assert!(
        finding["message"]
            .as_str()
            .expect("the message is a string")
            .contains("Rate limiting"),
        "the message names the title that is still in the summary:\n{finding:#}"
    );
    let route = finding["route"]
        .as_str()
        .expect("the finding carries a route")
        .to_string();
    let cmd = backticked(&route, "stale commit summary").to_string();

    // ── the read surface reports; it never rewrites ──
    assert_eq!(
        corpus.jigc_ok(&["doc", "show", &summary_addr, "--task", &task]),
        "the Rate limiting spec lands\n",
        "the summary's prose is the author's — the finding reports it, nothing rewrites it"
    );

    // ── re-raised at both doors, located and routed ──
    for (door, out) in stale_title_doors(&corpus, &task) {
        assert!(
            out.contains(STALE_TITLE_CODE),
            "{door} re-raises the advisory; got:\n{out}"
        );
        assert!(
            out.contains(&summary_addr),
            "{door} carries the located address; got:\n{out}"
        );
        assert!(
            out.contains(&cmd),
            "{door} carries the same route the ack did (`{cmd}`); got:\n{out}"
        );
    }
    let validate =
        String::from_utf8_lossy(&corpus.jigc(&["task", "validate", &task]).stdout).into_owned();
    assert!(
        validate.contains(&format!("advisory · {STALE_TITLE_CODE} — "))
            && validate.contains(&format!("at: {summary_addr}")),
        "the text surface renders the severity, the code and the `at:`; got:\n{validate}"
    );

    // ── following the emitted route verbatim repairs it ──
    let argv = shell_split(&corpus, &cmd);
    let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    corpus.jigc_stdin_ok(&args, "the Throttling spec lands");
    assert_eq!(
        corpus.jigc_ok(&["doc", "show", &summary_addr, "--task", &task]),
        "the Throttling spec lands\n"
    );
    for (door, out) in stale_title_doors(&corpus, &task) {
        assert!(
            !out.contains(STALE_TITLE_CODE),
            "{door} goes quiet once the summary no longer names the old title; got:\n{out}"
        );
    }
    let repaired: Value = serde_json::from_str(&corpus.jigc_ok(&[
        "doc",
        "rename",
        "spec:throttling",
        "--to",
        "Throttling limits",
        "--task",
        &task,
        "--format",
        "json",
    ]))
    .expect("the second rename ack is one JSON document");
    assert_eq!(
        repaired["findings"],
        serde_json::json!([]),
        "a rename whose old title the summary does not name acks nothing:\n{repaired:#}"
    );

    // ── the omitting context: a task that renamed nothing ──
    let quiet = task_with_a_summary_naming(&corpus, "leave every title alone", "Rate limiting");
    for (door, out) in stale_title_doors(&corpus, &quiet) {
        assert!(
            !out.contains(STALE_TITLE_CODE),
            "{door} is inert for a task that renamed nothing — never an error; got:\n{out}"
        );
    }
}

/// **A recorded title a staged doc still carries is not a stale one** (M51 Increment 11
/// / T2 — the repairability leg).
///
/// The predicate's whole job is to be silenceable by the route it prints. A recorded
/// title that is a **substring of the title a staged doc now carries** cannot be: every
/// correct summary naming `Rate limiting` also names `Rate`, so firing on it would print
/// a route that, followed exactly, changes nothing — M46's PT-1 defect, minted fresh.
/// The same reading covers the rename-back cell (`A → B → A`), where the recorded `A` is
/// the title the doc carries again.
///
/// So a recorded title is stale only when **no other staged doc's current `# H1`
/// contains it**, and this arm is what makes that clause load-bearing rather than
/// merely stated.
#[test]
fn a_recorded_title_a_staged_doc_still_carries_is_not_stale() {
    let corpus = TrialCorpus::build(State::Fresh);

    // (i) the superstring cell — the new title contains the old one.
    let grow = task_with_a_summary_naming(&corpus, "grow the title", "Rate");
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("commit:{grow}#summary"),
            "--from-file",
            "-",
            "--task",
            &grow,
        ],
        "the Rate limiting spec lands",
    );
    corpus.jigc_ok(&[
        "doc",
        "rename",
        "spec:rate",
        "--to",
        "Rate limiting",
        "--task",
        &grow,
    ]);
    for (door, out) in stale_title_doors(&corpus, &grow) {
        assert!(
            !out.contains(STALE_TITLE_CODE),
            "{door}: `Rate` is inside the title the doc now carries, so a summary naming \
             `Rate limiting` is correct and the finding would be unsilenceable — it is \
             not stale; got:\n{out}"
        );
    }

    // (ii) the rename-back cell — the doc carries the recorded title again.
    let back = task_with_a_summary_naming(&corpus, "change the title back", "Throttling");
    corpus.jigc_ok(&[
        "doc",
        "rename",
        "spec:throttling",
        "--to",
        "Rate limiting",
        "--task",
        &back,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "rename",
        "spec:rate-limiting",
        "--to",
        "Throttling",
        "--task",
        &back,
    ]);
    for (door, out) in stale_title_doors(&corpus, &back) {
        assert!(
            !out.contains(STALE_TITLE_CODE),
            "{door}: the doc carries `Throttling` again, and `Rate limiting` is nowhere \
             in the summary — nothing is stale; got:\n{out}"
        );
    }
}
