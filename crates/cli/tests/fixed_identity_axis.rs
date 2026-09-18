//! M52 Increment 6 / T1 — **the fixed-identity predicate has one engine home**, and
//! both seams that ask *"does this doctype's slug belong to the CLI?"* ask it there
//! (`completions/artifacts/M52/settle-record.md` → D5.1 · §15; `design/storage.md` →
//! Placement, the `cli::doc::fixed_identity_refusal` row's **converse**).
//!
//! The predicate is `placement.is_some() || singleton`. Before this suite, two seams
//! asked only the first disjunct — `engine::store::resolve_read_schema`'s read guard
//! and `cli::doc`'s bare-head expansion — while **the mint asks only the second**
//! (`engine::state::mint_instance`: `let slug = if schema.singleton { … }`). Every
//! shipped doctype declaring either declares both (five, enumerated at HEAD:
//! `changelog` · `vision` · `roadmap` · `decisions-log` · `deferral-ledger`), so the
//! divergence is invisible on the shipped packs and **every cell below is therefore
//! manufactured**: a `location:` + `singleton: true` doctype, the one shape that
//! separates the disjuncts.
//!
//! Driven at HEAD before the fix, that doctype created and promoted at its fixed slug
//! (`singleton_running_doc.rs` pins that half on this very fixture shape) while
//! `jigc doc show <ty>:bogus` fell through to a bare `store.not-found` naming **no
//! rule** and routing at *"create the referenced doc"* — a route that cannot be run,
//! since the only doc this type can have is the one at its fixed slug — and the bare
//! head `<ty>` was refused as a **malformed address**, the spelling the expansion
//! exists to accept.
//!
//! Scope of this suite, stated: it drives the two seams the predicate is shared by —
//! and, since **T2**, the doors that act on its answer. The second half below is the
//! nine-door axis: `store.fixed-identity` at `cli::doc::parse_verb_addr`, the one funnel
//! every caller-typed `doc` address passes through. The **three sibling doors** that
//! resolve their schema separately (`jigc rename`, `jigc milestone add-from-spec`,
//! `jigc task bind`) are **T3's** and are deliberately not asserted here.

use crate::support::trial_corpus::{State, TrialCorpus};
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
            "jigc-fixed-identity-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

fn jigc_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &[u8]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait for jigc")
}

fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// The manufactured cell: a doctype that is `singleton: true` **without** `placement:`,
/// homed under an ordinary `location:` directory. The shipped packs cannot produce this
/// shape (§15), so the predicate's second disjunct is reachable only from here.
///
/// The fixture ships alongside a `creates-task: true` workflow whose create-gate admits
/// it, so the staged instance this suite reads back is minted by the real `doc create`
/// door at the slug the mint chooses — never hand-written at the slug the test hopes for.
fn seed_fixture_pack(pack: &Path) {
    let schemas = pack.join("schemas");
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    for d in [&schemas, &workflows, &steps, &config] {
        fs::create_dir_all(d).expect("mk fixture pack subdir");
    }

    fs::write(
        schemas.join("runbook.yaml"),
        "type: runbook\n\
         singleton: true\n\
         location: runbooks/\n\
         description: A fixture running book homed in a location directory, not at a literal file.\n\
         usage: the manufactured cell separating `singleton:` from `placement:`.\n\
         sections:\n\
         \x20 - id: overview\n\
         \x20   slot: {}\n",
    )
    .expect("seed runbook schema");

    fs::write(
        workflows.join("keep-runbook.yaml"),
        "---\n\
         when: maintain the fixture running book\n\
         description: A fixture workflow that creates-or-updates the runbook singleton.\n\
         usage: proving the fixed-identity predicate over a location-homed singleton.\n\
         creates-task: true\n\
         allows-create: [{type: runbook, as: book}]\n\
         ---\n\
         {{ include: step:keep }}\n",
    )
    .expect("seed keep-runbook workflow");
    fs::write(
        steps.join("keep.yaml"),
        "Maintain the running book for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed keep step");

    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    crate::support::seed_ambush_class_declarer(pack);
}

/// A set-up repo whose project layer lists the fixture pack (the flow-18 listed-pack
/// seam), with one open `keep-runbook` task holding a staged, authored `runbook`.
/// Returns `(repo, home, task_id)`.
fn corpus(tag: &str) -> (TempDir, TempDir, String) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    let pack = repo.path().join("fixture-pack");

    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("project layer");

    seed_fixture_pack(&pack);
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("list the fixture pack");

    let started = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "keep-runbook",
            "open the running book",
        ],
    );
    assert_ok(&started, "`jigc start --workflow keep-runbook`");
    let task = "open-the-running-book".to_string();

    let created = jigc(
        repo.path(),
        home.path(),
        &["doc", "create", "runbook", "--title", "runbook"],
    );
    assert_ok(&created, "`jigc doc create runbook`");
    assert!(
        streams(&created).contains("runbook:runbook"),
        "the mint fixes a `singleton`'s slug to the type id — that is the half this \
         suite's seams must agree with; got:\n{}",
        streams(&created),
    );

    let authored = jigc_stdin(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-slot",
            "runbook:runbook#overview",
            "--from-file",
            "-",
        ],
        b"The running book's overview prose.\n",
    );
    assert_ok(&authored, "`jigc doc set-slot runbook:runbook#overview`");

    (repo, home, task)
}

/// A non-canonical slug on a `location:`-homed **singleton** is refused with the
/// singleton rule and the canonical address, on both read copies.
///
/// Before the shared predicate this address read through the guard entirely: the
/// committed arm answered `store.not-found` at `docs/runbooks/bogus.md` naming no rule
/// and routing at *"create the referenced doc"*, which for a fixed-identity doctype
/// names an act that cannot happen.
#[test]
fn a_location_homed_singleton_refuses_a_non_canonical_read_address() {
    let (repo, home, task) = corpus("read-guard");

    let committed = jigc(repo.path(), home.path(), &["doc", "show", "runbook:bogus"]);
    assert!(
        !committed.status.success(),
        "a non-canonical address on a fixed-identity doctype must refuse; got:\n{}",
        streams(&committed),
    );
    let rendered = streams(&committed);
    assert!(
        rendered.contains("identity is fixed"),
        "the refusal must name the rule that makes the address impossible, not merely \
         report an absent file; got:\n{rendered}",
    );
    assert!(
        rendered.contains("runbook:runbook"),
        "the refusal must name the canonical address; got:\n{rendered}",
    );
    assert!(
        !rendered.contains("create the referenced doc"),
        "the generic missing-doc route names an act a fixed-identity doctype cannot \
         take; got:\n{rendered}",
    );

    let staged = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "runbook:bogus", "--task", &task],
    );
    assert!(
        !staged.status.success(),
        "the staged arm must refuse too; got:\n{}",
        streams(&staged),
    );
    let staged_rendered = streams(&staged);
    assert!(
        staged_rendered.contains("identity is fixed")
            && staged_rendered.contains("runbook:runbook"),
        "the staged arm's block names the same rule and the same canonical address; \
         got:\n{staged_rendered}",
    );
    // **The staged arm no longer names a copy, and that is the T2 refusal, not a
    // regression of M51's N15** (`staged_read_miss_arm.rs` carries the full reading):
    // the door's guard fires at the parse boundary, before either read arm selects a
    // source, so the block claims no copy at all — the copy-blind shape
    // `store.unknown-type` has had since M43. N15's defect was a block that *claimed*
    // the staged copy and routed at the committed one; a refusal about the address
    // itself claims neither.
    assert!(
        !staged_rendered.contains("committed"),
        "a copy-blind refusal may not name one copy; got:\n{staged_rendered}",
    );
}

/// The bare head of a `location:`-homed **singleton** expands to `<ty>:<ty>` and reads
/// the instance — the spelling the expansion exists to accept, refused as a *malformed
/// address* before the predicate was shared.
#[test]
fn a_location_homed_singletons_bare_head_expands_to_its_canonical_address() {
    let (repo, home, task) = corpus("bare-head");

    let bare = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "runbook", "--task", &task],
    );
    assert_ok(&bare, "`jigc doc show runbook --task <id>` (the bare head)");
    let rendered = streams(&bare);
    assert!(
        rendered.contains("The running book's overview prose."),
        "the bare head must read the instance at the canonical slug; got:\n{rendered}",
    );

    // …and it reads the same bytes the canonical spelling does — the expansion is an
    // alias, not a second read path.
    let canonical = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "runbook:runbook", "--task", &task],
    );
    assert_ok(&canonical, "`jigc doc show runbook:runbook --task <id>`");
    assert_eq!(
        String::from_utf8_lossy(&bare.stdout),
        String::from_utf8_lossy(&canonical.stdout),
        "the bare head is an alias for the canonical address, not a second read path",
    );
}

// ---------------------------------------------------------------------------
// M52 Increment 6 / T2 — `store.fixed-identity` at the parse boundary
// ---------------------------------------------------------------------------
//
// The predicate above answers the question; this half is the **doors acting on the
// answer**. Driven at `89ff8232` on the shipped packs, `jigc doc set-field
// vision:alpha#meta/grounded-in --value "[research:x]" --task <id>` exited **0** and
// minted `.jigc/tasks/<id>/docs/vision:alpha.md` — a staged instance at an identity the
// store cannot hold, while every read surface said `vision:alpha` does not exist. Taken
// end to end on the released `1.0.0-rc.15` over the sibling `roadmap:bogus`
// (`completions/artifacts/M52/advocates/F5.md` §1), the same cell **finalized and
// committed**: the real `docs/roadmap.md` rewritten under a never-existing identity,
// `jigc validate` clean at exit 0.
//
// `parse_verb_addr` is the **one funnel** for all nine `doc.rs` doors, and it already
// resolves the pack, so the guard is one home rather than nine. It sits after the shipped
// `reject_malformed_slug_head`: a token that is not a slug is not an identity question.
// Precedence is unchanged — an unknown doctype resolves to no schema, the predicate is
// false, and the address falls through to the door's own `store.unknown-type`.
//
// The axis is **nine doors × two doctype shapes**: a `placement:` doctype with a
// committed instance (`vision`, the shipped cell) and the manufactured `location:` +
// `singleton: true` cell above (§15 — the shipped packs cannot produce it).
//
// **What the control asserts, and why it is the guard's silence rather than a success.**
// The guard fires at the *parse* boundary, before any shape adjudication, so whether a
// door's `#fragment` exists is not a variable this axis ranges over: the canonical and
// bare spellings are asserted to carry **no `store.fixed-identity`** at all nine doors,
// and the doors that genuinely apply to each shape are additionally driven green.

use std::process::Output;

/// The code every door raises for an address whose `<slug>` head is not the identity a
/// fixed-identity doctype has.
const FIXED_IDENTITY: &str = "store.fixed-identity";

/// The address hops one doctype shape offers each door. The item hops need not name a
/// real item: the guard refuses before any shape is adjudicated, and the control's claim
/// is the guard's silence, not the door's success.
struct Hops {
    slot: &'static str,
    field: &'static str,
    item_section: &'static str,
    item: &'static str,
}

/// The nine `doc` doors that take a caller-typed address through `parse_verb_addr`, each
/// row carrying its own runnable argv — so the axis is driven from the door set rather
/// than from a prose list of it. `true` marks a door that reads its payload from stdin.
fn doors(head: &str, hops: &Hops, task: &str) -> Vec<(&'static str, Vec<String>, bool)> {
    let own = |parts: &[&str]| parts.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    vec![
        (
            "doc set-field",
            own(&[
                "doc",
                "set-field",
                &format!("{head}#{}", hops.field),
                "--value",
                "x",
                "--task",
                task,
            ]),
            false,
        ),
        (
            "doc set-field --unset",
            own(&[
                "doc",
                "set-field",
                &format!("{head}#{}", hops.field),
                "--unset",
                "--task",
                task,
            ]),
            false,
        ),
        (
            "doc set-slot",
            own(&[
                "doc",
                "set-slot",
                &format!("{head}#{}", hops.slot),
                "--from-file",
                "-",
                "--task",
                task,
            ]),
            true,
        ),
        (
            "doc add-item",
            own(&[
                "doc",
                "add-item",
                &format!("{head}#{}", hops.item_section),
                "--title",
                "An item",
                "--task",
                task,
            ]),
            false,
        ),
        (
            "doc remove-item",
            own(&[
                "doc",
                "remove-item",
                &format!("{head}#{}", hops.item),
                "--task",
                task,
            ]),
            false,
        ),
        (
            "doc retitle-item",
            own(&[
                "doc",
                "retitle-item",
                &format!("{head}#{}", hops.item),
                "--title",
                "Another",
                "--task",
                task,
            ]),
            false,
        ),
        (
            "doc rename",
            own(&["doc", "rename", head, "--to", "Phantom", "--task", task]),
            false,
        ),
        ("doc show", own(&["doc", "show", head]), false),
        (
            "doc show --task",
            own(&["doc", "show", head, "--task", task]),
            false,
        ),
    ]
}

/// How one cell invokes the binary: argv, plus the stdin payload the `--from-file -`
/// door needs. The two cells build their repos differently — one from the shared trial
/// corpus, one from the manufactured fixture pack — so the axis takes the invocation as a
/// parameter and stays one body over both.
type RunDoor<'a> = dyn Fn(&[&str], Option<&[u8]>) -> Output + 'a;

/// Drive one cell of the axis: every door under the bogus head refuses, none of them
/// writes, and neither the canonical nor the bare spelling is touched by the guard.
fn drive_cell(
    label: &str,
    doctype: &str,
    hops: &Hops,
    task: &str,
    docs_dir: &Path,
    run: &RunDoor<'_>,
) {
    let bogus_head = format!("{doctype}:bogus");
    let canonical_head = format!("{doctype}:{doctype}");

    for (verb, argv, stdin) in doors(&bogus_head, hops, task) {
        let args: Vec<&str> = argv.iter().map(String::as_str).collect();
        let out = run(&args, stdin.then_some(b"prose\n".as_slice()));
        let rendered = streams(&out);
        assert_eq!(
            out.status.code(),
            Some(1),
            "[{label}] `jigc {}` must refuse at exit 1; got:\n{rendered}",
            args.join(" "),
        );
        assert!(
            rendered.contains(FIXED_IDENTITY),
            "[{label}] `{verb}` must refuse a non-canonical fixed-identity address with \
             `{FIXED_IDENTITY}`; got:\n{rendered}",
        );
        assert!(
            rendered.contains(&canonical_head),
            "[{label}] `{verb}`'s refusal must name the canonical address \
             `{canonical_head}`; got:\n{rendered}",
        );
        // The write the door did not make: no staged instance at the bogus identity.
        let staged: Vec<String> = std::fs::read_dir(docs_dir)
            .expect("the task's staged docs directory")
            .map(|e| {
                e.expect("a staged entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .filter(|name| name.starts_with(&format!("{doctype}:")) && name.contains("bogus"))
            .collect();
        assert!(
            staged.is_empty(),
            "[{label}] `{verb}` refused and still staged {staged:?} — the refusal has to \
             land before the copy-in, not after it",
        );
    }

    // The control: the two spellings a fixed-identity doctype really has.
    for head in [canonical_head.as_str(), doctype] {
        for (verb, argv, stdin) in doors(head, hops, task) {
            let args: Vec<&str> = argv.iter().map(String::as_str).collect();
            let out = run(&args, stdin.then_some(b"prose\n".as_slice()));
            let rendered = streams(&out);
            assert!(
                !rendered.contains(FIXED_IDENTITY),
                "[{label}] `{verb}` under the `{head}` spelling names the identity this \
                 doctype has — the guard must stay silent; got:\n{rendered}",
            );
        }
    }

    // …and silence is not the whole control: the two doors that genuinely apply to both
    // shapes — the slot write each schema declares, and the staged read back of it — run
    // green under the canonical spelling. A guard that refused everything would satisfy
    // the loop above and fail here.
    let slot = format!("{canonical_head}#{}", hops.slot);
    let wrote = run(
        &[
            "doc",
            "set-slot",
            slot.as_str(),
            "--from-file",
            "-",
            "--task",
            task,
        ],
        Some(b"The control prose.\n".as_slice()),
    );
    assert!(
        wrote.status.success(),
        "[{label}] the canonical slot write must still land; got:\n{}",
        streams(&wrote),
    );
    let read_back = run(
        &["doc", "show", canonical_head.as_str(), "--task", task],
        None,
    );
    assert!(
        read_back.status.success(),
        "[{label}] the canonical staged read-back must still serve; got:\n{}",
        streams(&read_back),
    );
}

/// **The axis, cell 1** — a `placement:` doctype (`vision`) with a committed instance.
#[test]
fn every_doc_door_refuses_a_non_canonical_placement_identity() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let task = corpus.start_workflow("form-vision", "revise the vision");
    let docs = corpus
        .repo()
        .join(".jigc")
        .join("tasks")
        .join(&task)
        .join("docs");
    let hops = Hops {
        slot: "thesis",
        field: "meta/grounded-in",
        item_section: "open-questions",
        item: "open-questions/one",
    };
    drive_cell(
        "vision",
        "vision",
        &hops,
        &task,
        &docs,
        &|args, stdin| match stdin {
            Some(bytes) => corpus.jigc_stdin(args, &String::from_utf8_lossy(bytes)),
            None => corpus.jigc(args),
        },
    );
}

/// **The axis, cell 2** — the manufactured `location:` + `singleton: true` doctype, the
/// shape no shipped pack takes and the one the `placement`-only predicate missed.
#[test]
fn every_doc_door_refuses_a_non_canonical_location_singleton_identity() {
    let (repo, home, task) = corpus("door-axis");
    let docs = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(&task)
        .join("docs");
    let hops = Hops {
        slot: "overview",
        field: "meta/owner",
        item_section: "steps",
        item: "steps/one",
    };
    drive_cell(
        "runbook",
        "runbook",
        &hops,
        &task,
        &docs,
        &|args, stdin| match stdin {
            Some(bytes) => jigc_stdin(repo.path(), home.path(), args, bytes),
            None => jigc(repo.path(), home.path(), args),
        },
    );
}

/// The refusal reaches a driver as **data**: the findings arm of the pinned envelope,
/// with a non-null `(code, target)` key — not the flattened `{"error": …}` its
/// `store.malformed-slug` neighbour at this same boundary takes
/// (`design/command-output-contract.md` → the `store.*` exception).
#[test]
fn the_refusal_rides_the_findings_arm_with_a_key() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let task = corpus.start_workflow("form-vision", "revise the vision");

    let out = corpus.jigc(&[
        "doc",
        "set-field",
        "vision:alpha#meta/grounded-in",
        "--value",
        "[research:x]",
        "--task",
        &task,
        "--format",
        "json",
    ]);
    let body = String::from_utf8_lossy(&out.stderr);
    let json: serde_json::Value = serde_json::from_str(body.trim())
        .unwrap_or_else(|e| panic!("the envelope parses: {e}\n{body}"));
    let finding = &json["findings"][0];
    assert_eq!(finding["code"], FIXED_IDENTITY, "envelope:\n{json:#}");
    assert_eq!(
        finding["key"]["code"], FIXED_IDENTITY,
        "envelope:\n{json:#}"
    );
    assert_eq!(
        finding["key"]["target"], "vision:alpha#meta/grounded-in",
        "the key's target is the address the caller named, never null; envelope:\n{json:#}",
    );
    assert!(
        finding["route"]
            .as_str()
            .is_some_and(|route| route.contains("vision:vision")),
        "the route names the identity this doctype has; envelope:\n{json:#}",
    );
}
