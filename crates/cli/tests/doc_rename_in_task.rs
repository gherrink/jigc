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

/// Split an emitted command into argv the way a shell would — honouring the double
/// quotes a route puts around a title, so the **emitted bytes** are the ones run
/// rather than a test-side reconstruction of them.
fn shell_split(cmd: &str) -> Vec<String> {
    let mut argv = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut started = false;
    for ch in cmd.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                started = true;
            }
            c if c.is_whitespace() && !in_quotes => {
                if started {
                    argv.push(std::mem::take(&mut current));
                    started = false;
                }
            }
            c => {
                current.push(c);
                started = true;
            }
        }
    }
    if started {
        argv.push(current);
    }
    argv
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
    corpus.jigc_ok(&["task", "discard", &second]);
    let argv = shell_split(&cmd);
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
    let argv = shell_split(&cmd);
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
