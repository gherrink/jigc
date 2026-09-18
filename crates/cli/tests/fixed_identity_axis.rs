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
//! and, since **T2**, the doors that act on its answer. The second half is the nine-door
//! axis: `store.fixed-identity` at `cli::doc::parse_verb_addr`, the one funnel every
//! caller-typed `doc` address passes through. The third half (**T3**) is the **three
//! sibling doors** that hold no such funnel and resolve their schema each for itself —
//! `jigc rename`, `jigc milestone add-from-spec`, `jigc task bind` — each guarded at its
//! own resolution point, ahead of every mutation and behind `store.unknown-type`
//! (settle-record → D5.2 as amended by §9).

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
///
/// Its `reads:` role is the **manufactured** half of the `jigc task bind` cell (T3's
/// declared bound): across both shipped packs there is exactly one `reads` role —
/// `implement-from-spec.yaml:6`, type `spec`, non-singleton — so `bind`'s fixed-identity
/// cell is unreachable on a stock corpus and is never claimed to be reachable there
/// (`completions/artifacts/M52/baseline-tokens.md` §2.3).
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
         reads: [{role: manual, type: runbook}]\n\
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

// ---------------------------------------------------------------------------
// M52 Increment 6 / T3 — the three sibling doors
// ---------------------------------------------------------------------------
//
// `parse_verb_addr` is the nine `doc` doors' shared funnel; these three doors are not its
// callers and hold no funnel of their own — each parses the caller's address and resolves
// its schema at its own site (settle-record → §9). So each takes the predicate **at that
// resolution point**: after the door's `store.unknown-type` answer, so precedence is
// unchanged, and ahead of every mutation, because two of the three mutate.
//
// Driven at `14b9ebb5` (T2's HEAD), the state each cell below refuses:
//
//   * `jigc rename vision:alpha --to Phantom --slug alpha` → **exit 0**, `# Vision`
//     rewritten to `# Phantom` in the real `VISION.md`, commit `7107ec5` landed. A
//     committing, `MovesOnBehalf` door taking an identity `vision` cannot have — and the
//     no-`--slug` cell one argument away refused `write.identity-change` while *composing
//     the caller's bogus slug into the escape hatch it printed*
//     (`completions/artifacts/M52/baseline-tokens.md` §4.1).
//   * `jigc milestone add-from-spec probe-milestone vision:alpha` → exit 1, but the head
//     was **accepted**: `store.no-such-section` read off the real committed `VISION.md`
//     (it enumerates that file's sections), routed at `jigc doc show vision:alpha` — an
//     address its sibling read door refuses (§4.2). A dead end printed by jigc itself.
//   * `jigc task bind vision vision:alpha <task>` over a manufactured `reads:` role →
//     **exit 0**, `roles.json` carrying `"vision": "vision:alpha"` — a read role bound to
//     an identity no read door resolves.

/// The `store.unknown-type` control every sibling owes: the predicate needs a resolved
/// schema, so an unknown doctype must still answer *unknown doctype* rather than being
/// re-diagnosed as an identity fault.
fn assert_unknown_type_answers_first(out: &Output, what: &str) {
    let rendered = streams(out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "{what}: an unknown doctype refuses at exit 1; got:\n{rendered}",
    );
    assert!(
        rendered.contains("store.unknown-type"),
        "{what}: an unknown doctype resolves to no schema, so the identity predicate is \
         not applicable and the door's own `store.unknown-type` still answers first; \
         got:\n{rendered}",
    );
    assert!(
        !rendered.contains(FIXED_IDENTITY),
        "{what}: the identity guard must not pre-empt the unknown-doctype answer; \
         got:\n{rendered}",
    );
}

/// The refusal shape every sibling door owes: exit 1, the code, and the canonical address
/// in the message or the route — never the caller's own token as the way forward.
fn assert_fixed_identity_refusal(out: &Output, doctype: &str, bogus: &str, what: &str) {
    let rendered = streams(out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "{what}: a fixed-identity alias refuses at exit 1; got:\n{rendered}",
    );
    assert!(
        rendered.contains(FIXED_IDENTITY),
        "{what}: the refusal must carry `{FIXED_IDENTITY}`; got:\n{rendered}",
    );
    let canonical = format!("{doctype}:{doctype}");
    assert!(
        rendered.contains(&canonical),
        "{what}: the refusal must name the identity this doctype does have \
         (`{canonical}`); got:\n{rendered}",
    );
    for route in rendered
        .lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with("route:"))
    {
        assert!(
            !route.contains(bogus),
            "{what}: the route must be built from the schema, never by composing the \
             caller's own `{bogus}` back into the escape hatch; got:\n{route}",
        );
    }
}

/// **Sibling 1 — `jigc rename`, the committing door.** Both head shapes refuse, with and
/// without the `--slug` override that made the bogus head land at exit 0, and the real
/// file and the git history are untouched.
#[test]
fn rename_refuses_a_non_canonical_fixed_identity_head() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let vision = corpus.repo().join("VISION.md");
    let before = fs::read(&vision).expect("read the committed VISION.md");
    let head_before = corpus.git(&["rev-parse", "HEAD"]);

    // The cell that landed: the override pins the bogus identity, so the reslug guard
    // one task over sees no identity change and every later guard is about the title.
    let pinned = corpus.jigc(&[
        "rename",
        "vision:alpha",
        "--to",
        "Phantom",
        "--slug",
        "alpha",
    ]);
    assert_fixed_identity_refusal(&pinned, "vision", "alpha", "`rename … --slug <bogus>`");

    // …and the cell one argument away, whose refusal used to be about the *reslug* and
    // composed `alpha` into the route it printed.
    let bare = corpus.jigc(&["rename", "vision:alpha", "--to", "Phantom"]);
    assert_fixed_identity_refusal(&bare, "vision", "alpha", "`rename` (no `--slug`)");

    assert_eq!(
        fs::read(&vision).expect("re-read VISION.md"),
        before,
        "a refused rename may not have rewritten the real file's `# H1`",
    );
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        head_before,
        "a refused rename may not have committed",
    );

    assert_unknown_type_answers_first(
        &corpus.jigc(&["rename", "nosuch:thing", "--to", "Phantom"]),
        "`rename nosuch:thing`",
    );

    // The control: the identity this doctype does have is not touched by the guard.
    let canonical = corpus.jigc(&[
        "rename",
        "vision:vision",
        "--to",
        "Phantom",
        "--slug",
        "vision",
    ]);
    assert!(
        !streams(&canonical).contains(FIXED_IDENTITY),
        "the canonical spelling names the identity this doctype has — the guard must stay \
         silent; got:\n{}",
        streams(&canonical),
    );
}

/// **Sibling 1, the second disjunct** — the manufactured `location:` + `singleton: true`
/// doctype at the same door. The fixture's open task is settled first, so the cell is
/// about the identity and nothing else.
///
/// This cell also carries the **log identity** every `jigc rename` refusal owes
/// (`cli::rename::RefusalKind`'s doc: the code that reaches `finding_codes`, without
/// which a refused rename is one more anonymous exit 1 there). This refusal is
/// deliberately *not* a `RefusalKind` member — the code, the message and the route belong
/// to the class, not to this door — so the obligation the enum states is discharged here,
/// in the class's own axis, rather than left unasserted on the strength of that sentence.
#[test]
fn rename_refuses_a_non_canonical_location_singleton_head() {
    let (repo, home, task) = corpus("rename-location-singleton");
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "discard", &task, "--force"],
        ),
        "`jigc task discard` (settling the fixture's open task)",
    );
    fs::write(
        repo.path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml"),
        "scalar:\n  invocation-log: true\n",
    )
    .expect("enable the invocation log");

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "rename",
            "runbook:bogus",
            "--to",
            "Phantom",
            "--slug",
            "bogus",
        ],
    );
    assert_fixed_identity_refusal(&out, "runbook", "bogus", "`rename runbook:bogus`");

    let log = repo
        .path()
        .join(".jigc")
        .join("logs")
        .join("invocations.jsonl");
    let body = fs::read_to_string(&log).expect("the invocation log");
    let record: serde_json::Value = body
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("each log line is valid JSON"))
        .rfind(|record: &serde_json::Value| record["argv"].to_string().contains("runbook:bogus"))
        .expect("a logged record for the refused rename");
    assert!(
        record["finding_codes"]
            .as_array()
            .expect("finding_codes is an array")
            .iter()
            .any(|code| code == FIXED_IDENTITY),
        "the refused rename must be legible in the invocation log by its code, not as one \
         more anonymous exit 1; record:\n{record:#}",
    );
    assert_eq!(
        record["exit_code"], 1,
        "the logged record must carry exit 1; record:\n{record:#}",
    );
}

/// **Sibling 2 — `jigc milestone add-from-spec`.** The door stops reading the real
/// committed file under an identity its sibling read door refuses, seeds no sub-task and
/// lands no record commit.
#[test]
fn milestone_add_from_spec_refuses_a_non_canonical_fixed_identity_head() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    assert_ok(
        &corpus.jigc(&["milestone", "create", "Probe milestone"]),
        "`jigc milestone create`",
    );
    let head_before = corpus.git(&["rev-parse", "HEAD"]);
    let tasks_json = corpus
        .repo()
        .join(".jigc/milestones/probe-milestone/tasks.json");
    let seeded_before = fs::read_to_string(&tasks_json).unwrap_or_default();

    let out = corpus.jigc(&[
        "milestone",
        "add-from-spec",
        "probe-milestone",
        "vision:alpha",
    ]);
    assert_fixed_identity_refusal(
        &out,
        "vision",
        "alpha",
        "`milestone add-from-spec … vision:alpha`",
    );
    // The door read the file's own section list before refusing; naming those sections is
    // the tell that the bogus identity resolved to the real `VISION.md`.
    let rendered = streams(&out);
    assert!(
        !rendered.contains("thesis"),
        "the refusal must land before the committed file is read, so it cannot enumerate \
         that file's sections; got:\n{rendered}",
    );
    assert_eq!(
        fs::read_to_string(&tasks_json).unwrap_or_default(),
        seeded_before,
        "a refused seeding may not have minted a sub-task",
    );
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        head_before,
        "a refused seeding may not have landed a record commit",
    );

    assert_unknown_type_answers_first(
        &corpus.jigc(&[
            "milestone",
            "add-from-spec",
            "probe-milestone",
            "nosuch:thing",
        ]),
        "`milestone add-from-spec … nosuch:thing`",
    );

    let canonical = corpus.jigc(&[
        "milestone",
        "add-from-spec",
        "probe-milestone",
        "vision:vision",
    ]);
    assert!(
        !streams(&canonical).contains(FIXED_IDENTITY),
        "the canonical spelling must pass the guard and fail on its own merits (a vision \
         carries no `criteria` section); got:\n{}",
        streams(&canonical),
    );
}

/// List a project-layer pack declaring a `reads:` role of the **shipped** `vision`
/// placement doctype, and return its workflow id.
///
/// The manufactured half is the *role*, never the doctype: `vision` is a shipped
/// placement singleton with a committed instance in this state, so the cell drives the
/// harm — a role bound to an identity no read door resolves — over real managed bytes.
fn seed_vision_reader_pack(repo: &Path) -> &'static str {
    let pack = repo.join("fixture-reader-pack");
    for dir in ["schemas", "workflows", "steps", "config"] {
        fs::create_dir_all(pack.join(dir)).expect("mk fixture reader pack subdir");
    }
    fs::write(
        pack.join("workflows").join("read-the-vision.yaml"),
        "---\n\
         when: a committed vision is the ground for this work\n\
         description: A fixture workflow that reads the committed vision as a bound role.\n\
         usage: proving the fixed-identity predicate at the bind door.\n\
         creates-task: true\n\
         reads: [{role: vision, type: vision}]\n\
         ---\n\
         {{ include: step:consult }}\n",
    )
    .expect("seed the read-the-vision workflow");
    fs::write(
        pack.join("steps").join("consult.yaml"),
        "Consult the bound vision for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed the consult step");
    fs::write(pack.join("config").join("commands.yaml"), "commands: []\n")
        .expect("seed empty catalog");
    crate::support::seed_ambush_class_declarer(&pack);

    let packs_yaml = repo.join(".jigc").join("config").join("packs.yaml");
    let existing = fs::read_to_string(&packs_yaml).unwrap_or_default();
    fs::write(
        &packs_yaml,
        format!("{existing}packs:\n  - {}\n", pack.display()),
    )
    .expect("list the fixture reader pack");
    "read-the-vision"
}

/// **Sibling 3 — `jigc task bind`.** A `reads:` role may not be bound to an identity its
/// doctype cannot have — the cell that bound one at exit 0.
#[test]
fn task_bind_refuses_a_non_canonical_fixed_identity_address() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let workflow = seed_vision_reader_pack(&corpus.repo());
    let task = corpus.start_workflow(workflow, "consult the vision");
    let roles = corpus
        .repo()
        .join(".jigc/tasks")
        .join(&task)
        .join("roles.json");

    let out = corpus.jigc(&["task", "bind", "vision", "vision:alpha", &task]);
    assert_fixed_identity_refusal(&out, "vision", "alpha", "`task bind vision vision:alpha`");
    assert!(
        !fs::read_to_string(&roles)
            .unwrap_or_default()
            .contains("alpha"),
        "a refused bind may not have recorded the role: {}",
        fs::read_to_string(&roles).unwrap_or_default(),
    );

    assert_unknown_type_answers_first(
        &corpus.jigc(&["task", "bind", "vision", "nosuch:thing", &task]),
        "`task bind vision nosuch:thing`",
    );

    // The control: the identity this doctype does have binds, as it always did.
    let canonical = corpus.jigc(&["task", "bind", "vision", "vision:vision", &task]);
    assert!(
        canonical.status.success(),
        "the canonical address must still bind; got:\n{}",
        streams(&canonical),
    );
}

/// **Sibling 3, the second disjunct** — the manufactured `location:` + `singleton: true`
/// doctype at the bind door. This shape did refuse before the guard, but with the door's
/// **code-less** `no such doc <addr>` bail: a fact about the corpus, carrying no identity
/// and inviting the caller to create a doc this doctype cannot have.
#[test]
fn task_bind_refuses_a_non_canonical_location_singleton_address() {
    let (repo, home, task) = corpus("bind-location-singleton");
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "bind", "manual", "runbook:bogus", &task],
    );
    assert_fixed_identity_refusal(&out, "runbook", "bogus", "`task bind manual runbook:bogus`");
}

/// **One code, one wire shape.** T2 put `store.fixed-identity` on the findings arm with a
/// non-null `(code, target)` key; a sibling answering the flattened `{"error": …}` default
/// would ship two shapes for one registered code, which is exactly what
/// `cli::milestone::ENVELOPE_ARM_CODES` exists to prevent.
#[test]
fn every_sibling_door_answers_the_refusal_on_the_findings_arm() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    assert_ok(
        &corpus.jigc(&["milestone", "create", "Probe milestone"]),
        "`jigc milestone create`",
    );
    let workflow = seed_vision_reader_pack(&corpus.repo());
    let task = corpus.start_workflow(workflow, "consult the vision");

    let cells: Vec<(&str, Vec<String>)> = vec![
        (
            "rename",
            vec![
                "rename".into(),
                "vision:alpha".into(),
                "--to".into(),
                "Phantom".into(),
                "--format".into(),
                "json".into(),
            ],
        ),
        (
            "milestone add-from-spec",
            vec![
                "milestone".into(),
                "add-from-spec".into(),
                "probe-milestone".into(),
                "vision:alpha".into(),
                "--format".into(),
                "json".into(),
            ],
        ),
        (
            "task bind",
            vec![
                "task".into(),
                "bind".into(),
                "vision".into(),
                "vision:alpha".into(),
                task.clone(),
                "--format".into(),
                "json".into(),
            ],
        ),
    ];

    for (door, argv) in cells {
        let args: Vec<&str> = argv.iter().map(String::as_str).collect();
        let out = corpus.jigc(&args);
        let body = String::from_utf8_lossy(&out.stderr);
        let json: serde_json::Value = serde_json::from_str(body.trim())
            .unwrap_or_else(|e| panic!("[{door}] the envelope parses: {e}\n{body}"));
        let finding = &json["findings"][0];
        assert_eq!(
            finding["code"], FIXED_IDENTITY,
            "[{door}] the refusal rides the findings arm; envelope:\n{json:#}",
        );
        assert_eq!(
            finding["key"]["code"], FIXED_IDENTITY,
            "[{door}] envelope:\n{json:#}",
        );
        assert_eq!(
            finding["key"]["target"], "vision:alpha",
            "[{door}] the key's target is the address the caller named, never null; \
             envelope:\n{json:#}",
        );
    }
}
