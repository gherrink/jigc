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
// answer**. Driven at `cbf7d833` on the shipped packs, `jigc doc set-field
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
// Driven at `9961013c` (T2's HEAD), the state each cell below refuses:
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

// ---------------------------------------------------------------------------
// M52 Increment 6 / T4 — the reslug refusal keys on the predicate, and its route
// is the schema's
// ---------------------------------------------------------------------------
//
// T3 closed the **head**: a fixed-identity doctype's non-canonical address no longer
// reaches this door's body. What is left is the cell one argument in — the **canonical**
// head asked to move to a different slug — and there the door still asked the old,
// narrower question. Arm (d) of `cli::rename`'s validation gate keyed on
// `schema.placement.is_some()`, so the second disjunct of `Schema::has_fixed_identity`
// fell straight through it.
//
// Driven at `79677138` over the manufactured `location:` + `singleton: true` doctype,
// with the instance committed by a real `jigc task finalize`:
//
//     $ jigc rename runbook:runbook --to Phantom --slug other
//     renamed runbook:runbook -> runbook:other (docs/runbooks/runbook.md ->
//       docs/runbooks/other.md), repointed 0 referrer(s)   # exit 0, commit 3943896
//
// …after which the doc was addressable by **nothing**: `jigc doc show runbook:other`
// refused `store.fixed-identity` (T2's guard, naming `runbook:runbook` as the one
// identity this doctype has) and `jigc doc show runbook` reported `store.not-found` at
// `docs/runbooks/runbook.md`. A committing door had moved a managed doc to an identity
// its own read doors refuse — the loss shape the predicate exists to prevent, reached
// through the *mint's* own disjunct.
//
// The second half is the route. A fixed-identity reslug refusal carries a
// [`Repair::Command`] route — the retitle this door *does* support — and it composed
// that argv from the **caller's** parsed address. That is correct today only because
// T3's guard sits upstream and has already forced the head canonical: a non-local
// invariant standing in for a local fact. Both halves are now read from the resolved
// schema (`<ty>:<ty>` and `--slug <ty>`), so the route is right at the site rather than
// by the grace of a guard three gates earlier — and the cells below **run the emitted
// argv** rather than asserting its shape, because a route that does not land is a dead
// end whatever it says (`design/surface-contract.md` → nothing dead-ends).

/// The code `jigc rename` raises for a reslug of a fixed identity
/// ([`cli::rename::RefusalKind::FixedIdentity`]).
const IDENTITY_CHANGE: &str = "write.identity-change";

/// `git <args>` in `repo`, returning trimmed stdout.
fn git_capture(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// The **emitted** route's argv: the first backticked span of the rendered `route:` line,
/// split on whitespace. Read off the surface the binary printed, never rebuilt here — the
/// bytes an agent would paste are the contract.
fn emitted_route_argv(rendered: &str) -> Vec<String> {
    let route = rendered
        .lines()
        .map(str::trim_start)
        .find(|line| line.starts_with("route:"))
        .unwrap_or_else(|| panic!("the refusal must print a route; got:\n{rendered}"));
    let (_, rest) = route
        .split_once('`')
        .unwrap_or_else(|| panic!("a `Command` route names its argv in backticks: {route}"));
    let (command, _) = rest
        .split_once('`')
        .unwrap_or_else(|| panic!("a route's command span must close: {route}"));
    command.split_whitespace().map(str::to_owned).collect()
}

/// Fill the fixture task's transient `commit` doc and land it, so the manufactured
/// singleton exists as a **committed** doc — the only state `jigc rename` acts on.
fn commit_the_runbook(repo: &Path, home: &Path, task: &str) {
    let commit = format!("commit:{task}");
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "set-field",
                &format!("{commit}#header/type"),
                "--task",
                task,
                "--value",
                "chore",
            ],
        ),
        "`jigc doc set-field commit:<task>#header/type`",
    );
    assert_ok(
        &jigc_stdin(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("{commit}#summary"),
                "--task",
                task,
                "--from-file",
                "-",
            ],
            b"seed the runbook\n",
        ),
        "`jigc doc set-slot commit:<task>#summary`",
    );
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task]),
        "`jigc task finalize` (landing the manufactured singleton)",
    );
}

/// **The red cell** — a `location:`-homed singleton's reslug, which arm (d)'s
/// `placement`-only key let through: the file moved, the commit landed, and the doc
/// became unaddressable. Both spellings of the ask refuse — the explicit `--slug other`
/// and the title-derived one a bare `--to` mints — and neither moves a byte.
#[test]
fn rename_refuses_a_reslug_of_a_location_homed_singleton() {
    let (repo, home, task) = corpus("rename-reslug-location-singleton");
    commit_the_runbook(repo.path(), home.path(), &task);

    let doc = repo.path().join("docs").join("runbooks").join("runbook.md");
    let before = fs::read(&doc).expect("the committed runbook");
    let head_before = git_capture(repo.path(), &["rev-parse", "HEAD"]);

    for argv in [
        vec![
            "rename",
            "runbook:runbook",
            "--to",
            "Phantom",
            "--slug",
            "other",
        ],
        vec!["rename", "runbook:runbook", "--to", "Phantom"],
    ] {
        let out = jigc(repo.path(), home.path(), &argv);
        let rendered = streams(&out);
        assert_eq!(
            out.status.code(),
            Some(1),
            "`jigc {}` must refuse at exit 1; got:\n{rendered}",
            argv.join(" "),
        );
        assert!(
            rendered.contains(IDENTITY_CHANGE),
            "`jigc {}` must refuse with `{IDENTITY_CHANGE}`; got:\n{rendered}",
            argv.join(" "),
        );
        assert_eq!(
            fs::read(&doc).expect("the runbook is still at its home"),
            before,
            "`jigc {}`: a refused reslug may not have touched the doc",
            argv.join(" "),
        );
        for stray in ["other.md", "phantom.md"] {
            assert!(
                !repo
                    .path()
                    .join("docs")
                    .join("runbooks")
                    .join(stray)
                    .exists(),
                "`jigc {}`: a refused reslug may not have minted `{stray}`",
                argv.join(" "),
            );
        }
        assert_eq!(
            git_capture(repo.path(), &["rev-parse", "HEAD"]),
            head_before,
            "`jigc {}`: a refused reslug may not have committed",
            argv.join(" "),
        );
    }
}

/// **The route, on the second disjunct** — composed from the resolved schema and *run*.
///
/// The refusal withholds the reslug and offers the retitle; the argv it prints is
/// executed verbatim here, and what it lands is asserted: the H1 rewritten, the doc still
/// at the home its identity fixes it to, and one commit.
#[test]
fn a_location_homed_singletons_reslug_route_is_the_schemas_and_it_lands() {
    let (repo, home, task) = corpus("rename-reslug-route-location");
    commit_the_runbook(repo.path(), home.path(), &task);
    let doc = repo.path().join("docs").join("runbooks").join("runbook.md");
    let head_before = git_capture(repo.path(), &["rev-parse", "HEAD"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "rename",
            "runbook:runbook",
            "--to",
            "Phantom",
            "--slug",
            "other",
        ],
    );
    let rendered = streams(&out);
    let route = emitted_route_argv(&rendered);
    assert_eq!(
        route,
        vec![
            "jigc",
            "rename",
            "runbook:runbook",
            "--to",
            "Phantom",
            "--slug",
            "runbook",
        ],
        "the route's identity and its `--slug` are the schema's — `<ty>:<ty>` and the \
         type id — never the caller's `other` echoed back; got:\n{rendered}",
    );

    let followed: Vec<&str> = route[1..].iter().map(String::as_str).collect();
    let landed = jigc(repo.path(), home.path(), &followed);
    assert_ok(&landed, "the emitted route, run verbatim");
    assert!(
        fs::read_to_string(&doc)
            .expect("the runbook is still at its home")
            .starts_with("# Phantom\n"),
        "the retitle the refusal offered must land: the H1 is the new title and the doc \
         never moved; got:\n{}",
        fs::read_to_string(&doc).unwrap_or_default(),
    );
    assert_ne!(
        git_capture(repo.path(), &["rev-parse", "HEAD"]),
        head_before,
        "the retitle commits — the refusal withheld the reslug, not the whole verb",
    );
}

/// **The route, on the first disjunct** — the shipped `placement:` doctype, under the
/// `--slug` override cell (`flow37_rename`'s own scene drives the bare `--to`). The same
/// two halves come off the same schema, and the same argv runs.
#[test]
fn a_placement_singletons_reslug_route_is_the_schemas_and_it_lands() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let vision = corpus.repo().join("VISION.md");

    let out = corpus.jigc(&[
        "rename",
        "vision:vision",
        "--to",
        "Phantom",
        "--slug",
        "other",
    ]);
    let rendered = streams(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a placement singleton's reslug refuses at exit 1; got:\n{rendered}",
    );
    assert!(
        rendered.contains(IDENTITY_CHANGE),
        "the refusal carries `{IDENTITY_CHANGE}`; got:\n{rendered}",
    );
    let route = emitted_route_argv(&rendered);
    assert_eq!(
        route,
        vec![
            "jigc",
            "rename",
            "vision:vision",
            "--to",
            "Phantom",
            "--slug",
            "vision",
        ],
        "the route's identity and its `--slug` are the schema's; got:\n{rendered}",
    );

    let followed: Vec<&str> = route[1..].iter().map(String::as_str).collect();
    let landed = corpus.jigc(&followed);
    assert!(
        landed.status.success(),
        "the emitted route must run verbatim at exit 0; got:\n{}",
        streams(&landed),
    );
    assert!(
        vision.is_file(),
        "the retitle keeps the doc at the literal home its placement fixes it to",
    );
}

// ---------------------------------------------------------------------------
// M52 Increment 6 / T5 — the OS name ceiling at the address head
// ---------------------------------------------------------------------------
//
// T2/T3 closed *which* identity a fixed-identity doctype may be addressed by. This half
// closes a question one tier below the doctype, and therefore below the identity guard:
// whether the caller's `<slug>` head can be a **name** at all. A doc's slug becomes one
// filesystem path component — `<docs-root>/<location>/<slug>.md` committed,
// `.jigc/tasks/<id>/docs/<type>:<slug>.md` staged — which is the exact path shape
// `cli::cli::SLUG_NAME_CEILING` is derived for, so the head takes that constant and the
// code M51 Increment 9 / T3 minted for it (`cli::task::SLUG_NAME_CEILING_CODE`), on that
// code's own *one code for the whole family* reading.
//
// **The state this refuses, driven at the wave's base** (`completions/artifacts/M52/
// baseline-tokens.md` §2.5 row 1): `jigc doc set-slot "vision:<300 bytes>#thesis"
// --from-file - --task <id>` answered
//
//     could not copy in `vision:aaaa…#thesis` for editing: File name too long (os error 63)
//
// — no code, no `at:`, no route, and under `--format json` the same sentence inside
// `{"error": …}`. A door had accepted a token it could never name, discovered that at the
// write, and reported the discovery as an I/O fault.
//
// **Why the guard is one edit and not five.** The baseline measured the symptom at five
// `doc` write doors, because only there did the head reach a `create_dir_all`/`write`. But
// an over-long head is unusable wherever it is typed — at the read doors it resolves
// nothing, at `rename` it names no file to move, at `add-from-spec` no spec to seed from —
// so the guard rides **beside the grammar reject**, inside
// `cli::task::reject_malformed_slug_head`, which all **four** user-address parse
// boundaries already call (`doc::parse_verb_addr` · `rename::parse_addr` ·
// `milestone::run_add_from_spec` · `TaskArea::bind`). That is `reject_slug_over_name_
// ceiling`'s own rule one family over: *an override that is inert at one door today is an
// identity at that door tomorrow*.
//
// **Precedence, and why the identity guard now runs second.** The order inside the
// boundary is the fault order — is this token a slug (`store.malformed-slug`), can it be a
// name (`write.slug-name-ceiling`), is it the identity this doctype has
// (`store.fixed-identity`). The first two are facts about the **token**; the third needs a
// resolved doctype. So a 300-byte head on `vision` answers the ceiling, not the identity —
// T2's cells are all short heads and are untouched.

/// The code an over-long `<slug>` head carries — `cli::task::SLUG_NAME_CEILING_CODE`,
/// spelled out rather than imported: a test comparing emitted bytes against the constant
/// that produced them proves only that the constant equals itself.
const NAME_CEILING: &str = "write.slug-name-ceiling";

/// The bare OS fault no boundary may leak any longer — the baseline's whole symptom.
const OS_FAULT: &str = "File name too long";

/// A well-formed `<slug>` head of `bytes` bytes. 300 is the token the baseline drove: it
/// clears every filesystem's own `NAME_MAX` by enough that a boundary which let it through
/// fails loudly rather than subtly.
fn head_of(bytes: usize) -> String {
    "a".repeat(bytes)
}

/// The staged docs a task holds, by filename — `[]` before the first write creates the
/// directory.
fn staged_docs(repo: &Path, task: &str) -> Vec<String> {
    let dir = repo.join(".jigc").join("tasks").join(task).join("docs");
    let Ok(entries) = fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .map(|e| {
            e.expect("a staged entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}

/// **The axis: all four user-address parse boundaries.** Each refuses the over-long head
/// with the ceiling code, an `at:` and a route; none leaks the OS fault; and the task's
/// staged set is byte-for-byte what it was before the sweep.
#[test]
fn every_user_address_parse_boundary_refuses_a_head_over_the_os_name_ceiling() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    assert_ok(
        &corpus.jigc(&["milestone", "create", "Probe milestone"]),
        "`jigc milestone create`",
    );
    let workflow = seed_vision_reader_pack(&corpus.repo());
    let task = corpus.start_workflow(workflow, "consult the vision");
    let before = staged_docs(&corpus.repo(), &task);

    let head = head_of(300);
    let cells: Vec<(&str, Vec<String>)> = vec![
        (
            "doc::parse_verb_addr (`doc set-field`)",
            vec![
                "doc".into(),
                "set-field".into(),
                format!("vision:{head}#meta/grounded-in"),
                "--value".into(),
                "[research:x]".into(),
                "--task".into(),
                task.clone(),
            ],
        ),
        (
            "rename::parse_addr",
            vec![
                "rename".into(),
                format!("adr:{head}"),
                "--to".into(),
                "Phantom".into(),
            ],
        ),
        (
            "milestone::run_add_from_spec",
            vec![
                "milestone".into(),
                "add-from-spec".into(),
                "probe-milestone".into(),
                format!("spec:{head}"),
            ],
        ),
        (
            "TaskArea::bind",
            vec![
                "task".into(),
                "bind".into(),
                "vision".into(),
                format!("vision:{head}"),
                task.clone(),
            ],
        ),
    ];

    for (boundary, argv) in cells {
        let args: Vec<&str> = argv.iter().map(String::as_str).collect();
        let out = corpus.jigc(&args);
        let rendered = streams(&out);
        assert_eq!(
            out.status.code(),
            Some(1),
            "[{boundary}] an over-long `<slug>` head refuses at exit 1; got:\n{rendered}",
        );
        assert!(
            rendered.contains(NAME_CEILING),
            "[{boundary}] the refusal must carry `{NAME_CEILING}` — the head is a slug the \
             grammar accepts and a name the filesystem cannot hold; got:\n{rendered}",
        );
        assert!(
            !rendered.contains(OS_FAULT),
            "[{boundary}] the boundary adjudicates the token, so the OS fault must never \
             be reached, let alone reported; got:\n{rendered}",
        );
        assert!(
            rendered
                .lines()
                .any(|line| line.trim_start().starts_with("at:")),
            "[{boundary}] the refusal must say where — the address the caller typed; \
             got:\n{rendered}",
        );
        assert!(
            rendered
                .lines()
                .any(|line| line.trim_start().starts_with("route:")),
            "[{boundary}] the refusal must carry a route; got:\n{rendered}",
        );
    }

    assert_eq!(
        staged_docs(&corpus.repo(), &task),
        before,
        "no boundary may mint a staged instance at a head it refused",
    );
}

/// **The boundary is the derived constant, not a number written here.** `SLUG_NAME_CEILING`
/// bytes is accepted by the guard; one byte more is not.
///
/// Read from `cli::cli::SLUG_NAME_CEILING` on purpose, and this is where this suite parts
/// from its `slug_override_axis` sibling — which spells the same number out. That suite
/// asserts the *rendered message* names the ceiling, and comparing emitted bytes against
/// the constant that produced them proves only that the constant equals itself. This cell
/// asserts **behaviour at N and N+1**, which is a claim about what the guard reads: a
/// re-derivation that moved the ceiling and left the guard on a stale literal would pass a
/// hard-coded `165` and fail here.
#[test]
fn the_heads_ceiling_is_the_derived_constant_at_both_sides_of_the_boundary() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let ceiling = cli::cli::SLUG_NAME_CEILING;

    // At the ceiling: the guard is silent and the door answers on its own merits — there
    // is no such doc, which is a fact about the corpus and not about the token.
    let accepted = corpus.jigc(&["doc", "show", &format!("adr:{}", head_of(ceiling))]);
    let rendered = streams(&accepted);
    assert!(
        !rendered.contains(NAME_CEILING),
        "a head of exactly {ceiling} bytes is nameable — the guard must stay silent and \
         let the door answer; got:\n{rendered}",
    );
    assert!(
        rendered.contains("store.not-found"),
        "…and the door's own answer is that no such doc exists; got:\n{rendered}",
    );

    // One byte over: refused.
    let refused = corpus.jigc(&["doc", "show", &format!("adr:{}", head_of(ceiling + 1))]);
    let rendered = streams(&refused);
    assert_eq!(
        refused.status.code(),
        Some(1),
        "a head of {} bytes refuses at exit 1; got:\n{rendered}",
        ceiling + 1,
    );
    assert!(
        rendered.contains(NAME_CEILING),
        "one byte over the ceiling is over the ceiling; got:\n{rendered}",
    );
}

// ---------------------------------------------------------------------------
// M52 Increment 6 / T8 — the author steps stop calling `doc schema <ty>` the
// authority on a fixed-identity doctype's addresses
// ---------------------------------------------------------------------------
//
// T2–T5 closed what a caller *may type*. This half closes what the pack *tells* the
// caller to type, and it is last because the sentence it corrects only became false
// when the doors started refusing (settle-record → D5.5).
//
// The sentence, at the wave's base, on five methodology author steps and one dev step:
//
//     The `vision` schema is the authority on what you write into it — its required
//     slots and fields, each field's enum members, and every address a write can take:
//
//     jigc doc schema vision
//
// `doc schema` is a **type-level** projection: every address it advertises places the
// instance under [`engine::schema::SLUG_PLACEHOLDER`] — `vision:<slug>#thesis` on both
// arms, plain and JSON. For a **slugged** doctype that is the whole truth. For a
// fixed-identity one it is the one thing the schema read cannot answer: `<slug>` is not
// the caller's to fill, and since T2 a caller who fills it with anything but the type id
// is refused `store.fixed-identity` at every door. So a step that names that projection
// *the authority on every address a write can take* sends an agent to a surface which,
// followed literally, produces the refusal — a law-1 lie at the exact seam this
// increment made refusable (`design/surface-contract.md` → nothing lies).
//
// **The fence is derived twice over, which is the point.** The doctype set is the
// loaded packs filtered by T1's predicate ([`engine::schema::Schema::has_fixed_identity`])
// — never a list of the five shipped today — and the step set is *the steps that name
// that doctype's schema read*, read out of the same pack trees. So the sixth author step
// added tomorrow, for a doctype that is fixed-identity tomorrow, joins the axis with no
// edit here.
//
// **What the fence buys, stated.** Two absences and one presence: no `<ty>:<slug>`
// pattern (the projection's spelling, reproduced in prose), no *"every address a write
// can take"* clause, and the doctype's real identity spelled at least once. It does not
// grade the replacement's prose — that is the A-3 presence-never-content bound the
// `states-constraints` tier already draws (`design/methodology-docs.md`) — so a step
// could still say something false in some third wording. What it does make impossible is
// the shipped sentence coming back, and a new fixed-identity doctype's author step
// inheriting it by copy.

use cli::pack::{FilesystemPack, load_pack_schema};
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{SLUG_PLACEHOLDER, Schema};

/// The clause the shipped steps carried, and the one no step over a fixed-identity
/// doctype may carry again: it promises the schema read enumerates the write addresses,
/// and for this doctype class the read's `<slug>` is exactly what it cannot say.
///
/// Authored in [`normalized`]'s view — lowercase, single-spaced — because the shipped
/// prose wraps it across a line break in four of the six steps (`…every address a write\n
/// can take:`). A first cut of this fence compared against the raw body and was **green
/// over the mutant**: restoring one wrapped step's sentence changed nothing, and the axis
/// bought only the one step whose clause happened to fit on a line.
const EVERY_ADDRESS_CLAUSE: &str = "every address a write can take";

/// The comparison view every leg below reads: whitespace runs collapsed to one space,
/// ASCII case folded — so a phrase that wraps across a newline or opens a sentence
/// capitalized still matches.
///
/// This is `cli::pack::normalized_body`'s rule, and re-implemented rather than shared
/// because that function is `pub(crate)` and this is an integration test. Stated rather
/// than silently duplicated: if the two ever diverge, the pack's own named-fact map is
/// the authority and this is the copy that is wrong.
fn normalized(body: &str) -> String {
    let mut out = String::new();
    let mut pending_space = false;
    for ch in body.chars() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        pending_space = false;
        out.push(ch.to_ascii_lowercase());
    }
    out
}

/// The two shipped pack trees, by the name their steps are reported under.
fn pack_trees() -> Vec<(&'static str, PathBuf)> {
    vec![
        ("dev", Path::new(cli::pack_path!(dev)).to_path_buf()),
        (
            "methodology",
            Path::new(cli::pack_path!(methodology)).to_path_buf(),
        ),
    ]
}

/// The **fixed-identity doctype set of the loaded packs**, by type id — the walk lives in
/// [`fixed_identity_schemas`] (M52 Increment 8 / T5 gave it a second consumer, so the two
/// cannot read the class differently).
///
/// Read from the packs rather than written down: the five members at HEAD (`changelog` ·
/// `vision` · `roadmap` · `decisions-log` · `deferral-ledger`) are what the trees happen
/// to hold today, and a sixth joins this set by being declared, not by being listed.
fn fixed_identity_doctypes() -> Vec<String> {
    fixed_identity_schemas()
        .into_iter()
        .map(|schema| schema.ty)
        .collect()
}

/// Every `(pack, step id, body)` in the shipped trees.
fn all_steps() -> Vec<(&'static str, String, String)> {
    let mut steps = Vec::new();
    for (label, root) in pack_trees() {
        let pack = FilesystemPack::new(root);
        for id in pack.list(PackResourceKind::Steps) {
            let bytes = pack
                .read(PackResourceKind::Steps, &id)
                .expect("a listed step resource is readable");
            steps.push((
                label,
                id.as_str().to_owned(),
                String::from_utf8(bytes).expect("a shipped step is UTF-8"),
            ));
        }
    }
    steps.sort();
    steps
}

/// The axis: `(doctype, pack, step id, body)` for every step that sends the agent to a
/// **fixed-identity** doctype's schema read. That command line is the structural signal
/// — a step naming `jigc doc schema <ty>` is a step telling an agent where the authority
/// on writing `<ty>` lives — so the pairing is derived from the two sets rather than
/// hand-paired.
fn schema_authority_steps() -> Vec<(String, &'static str, String, String)> {
    let doctypes = fixed_identity_doctypes();
    let mut pairs = Vec::new();
    for (pack, step, body) in all_steps() {
        for ty in &doctypes {
            if body.contains(&format!("jigc doc schema {ty}")) {
                pairs.push((ty.clone(), pack, step.clone(), body.clone()));
            }
        }
    }
    pairs
}

/// **The axis.** No step that points at a fixed-identity doctype's schema read may
/// reproduce that read's `<ty>:<slug>` pattern or promise it enumerates the addresses —
/// and each must spell the one identity the doctype actually has.
#[test]
fn no_author_step_calls_doc_schema_the_authority_on_a_fixed_identitys_addresses() {
    let doctypes = fixed_identity_doctypes();
    assert!(
        doctypes.len() >= 2,
        "the predicate must find the shipped fixed-identity doctypes in the pack trees, \
         or this axis sweeps nothing; got {doctypes:?}",
    );

    let pairs = schema_authority_steps();
    assert!(
        !pairs.is_empty(),
        "at least one shipped step must point at a fixed-identity doctype's schema read, \
         or the fence is vacuous; fixed-identity doctypes: {doctypes:?}",
    );
    // Both trees, because the plan's own list reached only one of them: the sentence shipped
    // on five methodology steps AND on the dev pack's `author-change` over `changelog`, which
    // is the first of the five doctypes T1 enumerates. A derivation that silently read one
    // tree would satisfy every leg below over half the class.
    for tree in ["dev", "methodology"] {
        assert!(
            pairs.iter().any(|(_, pack, _, _)| *pack == tree),
            "the axis must reach the {tree} pack — its fixed-identity author steps are in the \
             class too; pairs: {:?}",
            pairs
                .iter()
                .map(|(ty, pack, step, _)| (ty, pack, step))
                .collect::<Vec<_>>(),
        );
    }

    for (ty, pack, step, body) in &pairs {
        let body = normalized(body);
        let pattern = format!("{ty}:{SLUG_PLACEHOLDER}");
        assert!(
            !body.contains(&pattern),
            "the {pack} pack's `{step}` reproduces the type-level pattern `{pattern}` for \
             the fixed-identity doctype `{ty}` — a slug the doors refuse \
             (`store.fixed-identity`); its one address is `{ty}:{ty}`",
        );
        assert!(
            !body.contains(EVERY_ADDRESS_CLAUSE),
            "the {pack} pack's `{step}` calls `jigc doc schema {ty}` the authority on \
             `{EVERY_ADDRESS_CLAUSE}` — the one thing a type-level projection cannot say \
             about a fixed-identity doctype, whose `<slug>` is the CLI's",
        );
        assert!(
            body.contains(&format!("{ty}:{ty}")),
            "the {pack} pack's `{step}` must spell `{ty}`'s one identity (`{ty}:{ty}`) \
             somewhere in the step it authors it from",
        );
    }
}

// ---------------------------------------------------------------------------
// M52 Increment 8 / T5 — a fixed-identity doctype has ONE identity, minted once:
// the adopt seam stops re-deriving it
// ---------------------------------------------------------------------------
//
// T1 gave the predicate one home and T2/T3 made the doors refuse a non-canonical
// address. This half closes the other direction: what jigc itself **writes down** as a
// fixed-identity doc's identity. Driven at the wave's base
// (`completions/artifacts/M52/baseline-freeze.md` §1.5, §4 L-2/L-3):
//
//     dev/jigc-rig refs-post-hoc → land the staged edge → jigc ingest → exit 0
//     .jigc/index/edges.json:
//       { "from": "vision:VISION", "relation": "grounded-in", … }
//       { "from": "vision:vision", "relation": "grounded-in", … }
//
// Two identities, one document, one relation, one target. `engine::ingest::adopt` minted
// `<ty>:<filename stem>` while, two frames earlier, `cli::ingest::classify_row` had
// already asked `cli::ingest::home_identity` — which answers `<ty>:<ty>` for a
// fixed-identity doctype, the same answer `engine::index::committed_instances`,
// `cli::unmanage::identity_of`, `cli::doc`'s bare-head expansion and the mint all give.
// The stem coincides for `roadmap` / `decisions-log` / `deferral-ledger`, so the two
// derivations disagreed for exactly the two members whose home file is not spelled like
// their type (`vision` → `VISION.md`, `changelog` → `CHANGELOG.md`).
//
// **Bound, read rather than assumed:** of those agreeing surfaces, two still ask
// `placement.is_some()` rather than the class predicate — `engine::index::instance_slug`
// (and through it `committed_instances`) and `cli::unmanage::identity_of`. Both are
// nonetheless correct *at a fixed-identity doctype's home*, because a `location:` +
// `singleton: true` doctype's one file is `<location>/<ty>.md` and its stem **is** the
// type id, which is why the arms below find them agreeing. What they would answer about a
// hand-placed sibling at such a home is undecided here and untouched: `ingest` now refuses
// to adopt one (`ingest.unaddressable-identity`), so no jigc operation produces the state,
// and re-keying an *enumeration* on the predicate would hide that file from the store
// sweep rather than report it — the opposite of what the sweep is for.
//
// Its consequence was L-3, the same defect read from the other end: `jigc unmanage
// VISION.md` drops the edges of the identity it computes (`vision:vision`), so the
// stem-keyed ones survive **and the door says `nothing to drop`** while `edges.json`
// still carries them — three runs in a row, exit 0 every time.
//
// **The fix is the seam, not a branch** (`implementation/dev-workflow.md` → the fix-shape
// hierarchy: unify the seam > derive from the registry > …). `adopt` no longer derives an
// identity at all; it takes the caller's, and the caller has exactly one mint. So the
// class is swept by construction — there is no per-doctype branch left to get wrong — and
// the arms below assert the *agreement* that fact buys, over the class read from the
// resolved schemas rather than from a list of the five members shipped today.

/// Every **fixed-identity** schema either shipped pack tree declares, parsed through the
/// production `load_pack_schema` and filtered by T1's predicate — the one walk
/// [`fixed_identity_doctypes`] and [`fixed_identity_homes`] both read.
///
/// Deduped by type id, in type order, so a doctype a project pack would shadow is counted
/// once and the sweep order is stable.
fn fixed_identity_schemas() -> Vec<Schema> {
    let mut found: Vec<Schema> = Vec::new();
    for (_, root) in pack_trees() {
        let pack = FilesystemPack::new(root);
        for id in pack.list(PackResourceKind::Schemas) {
            let bytes = pack
                .read(PackResourceKind::Schemas, &id)
                .expect("a listed schema resource is readable");
            let schema = load_pack_schema(&pack, &bytes)
                .unwrap_or_else(|e| panic!("shipped schema `{}` loads: {e:?}", id.as_str()));
            if schema.has_fixed_identity() && !found.iter().any(|s| s.ty == schema.ty) {
                found.push(schema);
            }
        }
    }
    found.sort_by(|a, b| a.ty.cmp(&b.ty));
    found
}

/// The fixed-identity class as `(type id, canonical identity, declared home path)` —
/// **all three read through [`engine::schema::Schema::projection`]**, the one structured
/// answer to *where this doctype's instances live and under what identity* (M52 Increment
/// 6 / T7). Never a written list: a sixth doctype joins by being declared.
///
/// The home is the **declared** one, which at the default cascade is also the resolved one:
/// `placement-root`'s default is `""` = unset, under which every declared home stands
/// byte-identical to no knob at all (`crates/cli/packs/dev/config/knobs.yaml`). The arms below
/// intersect it with what the corpus actually committed rather than assuming it is on
/// disk, so a corpus that *did* re-point the knob drops out of the sweep instead of
/// failing it on a path this helper guessed.
fn fixed_identity_homes() -> Vec<(String, String, String)> {
    fixed_identity_schemas()
        .into_iter()
        .filter_map(|schema| {
            let projection = schema.projection();
            let home = projection.home.path?;
            // The projection's `identity.address` is the bare type id for a fixed-identity
            // doctype; the `<ty>:<ty>` spelling is that address under its own type.
            let identity = format!("{}:{}", schema.ty, projection.identity.address);
            Some((schema.ty.clone(), identity, home))
        })
        .collect()
}

/// The `from` identities `.jigc/index/edges.json` holds, sorted — `[]` when the index has
/// not been written yet (it is a rebuildable cache, absent until a door persists one).
fn edge_froms(repo: &Path) -> Vec<String> {
    let path = repo.join(".jigc").join("index").join("edges.json");
    let Ok(body) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    let json: serde_json::Value =
        serde_json::from_str(&body).expect("the edge index is valid JSON");
    let mut froms: Vec<String> = json["edges"]
        .as_array()
        .expect("the edge index carries an `edges` array")
        .iter()
        .map(|edge| {
            edge["from"]
                .as_str()
                .expect("every edge carries a `from` identity")
                .to_owned()
        })
        .collect();
    froms.sort();
    froms
}

/// The fixed-identity doctypes this corpus has actually **committed** — the class
/// intersected with the homes on disk, so an arm below drives real documents and says so
/// when the intersection is empty rather than passing vacuously over nothing.
fn committed_fixed_identity_docs(repo: &Path) -> Vec<(String, String, String)> {
    let present: Vec<(String, String, String)> = fixed_identity_homes()
        .into_iter()
        .filter(|(_, _, home)| repo.join(home).is_file())
        .collect();
    assert!(
        present.len() >= 2,
        "the corpus must commit at least two of the shipped fixed-identity doctypes, or \
         every arm below sweeps nothing; class: {:?}",
        fixed_identity_homes(),
    );
    present
}

/// Build the `refs-post-hoc` state and **land its staged edge**: the fixture leaves the
/// `form-vision` task live with `vision —grounded-in→ research` set on the committed
/// vision and its `commit` doc unfilled, so the edge only reaches the committed index
/// through a finalize the fixture deliberately does not perform.
fn corpus_with_a_landed_fixed_identity_edge() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let task = corpus
        .live_task()
        .expect("`refs-post-hoc` leaves the form-vision task live")
        .to_owned();
    corpus.finalize(&task, "vision", "ground the vision in research", false);
    corpus
}

/// **The axis: every committed fixed-identity doctype × the five surfaces that name its
/// identity.** `jigc doc show`, `jigc rename`, `jigc ingest`, the edge index it writes,
/// and — in the sibling arm — `jigc unmanage` must all say `<ty>:<ty>`, and a second
/// `ingest` must add nothing.
///
/// The discriminating leg is the last one: at the wave's base the first `ingest` left
/// **two** identities for `vision`, and a surface that reads one of them (`doc show`,
/// `validate`'s ref resolution) cannot see the other's edges.
#[test]
fn every_surface_names_one_fixed_identity_and_ingest_mints_it_once() {
    let corpus = corpus_with_a_landed_fixed_identity_edge();
    let repo = corpus.repo();
    let docs = committed_fixed_identity_docs(&repo);

    // The two read doors, over each committed member.
    for (ty, identity, home) in &docs {
        let shown = corpus.jigc_ok(&["doc", "show", ty, "--format", "json"]);
        let json: serde_json::Value =
            serde_json::from_str(shown.trim()).expect("`doc show --format json` emits JSON");
        assert_eq!(
            format!(
                "{}:{}",
                json["type"].as_str().expect("the read names its type"),
                json["slug"].as_str().expect("the read names its slug"),
            ),
            *identity,
            "`jigc doc show {ty}` must name `{identity}` for the doc at `{home}`",
        );

        let renamed = corpus.jigc(&["rename", &format!("{ty}:bogus"), "--to", "Renamed"]);
        assert!(
            !renamed.status.success(),
            "`jigc rename {ty}:bogus` must refuse a non-canonical head",
        );
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&renamed.stdout),
            String::from_utf8_lossy(&renamed.stderr),
        );
        assert!(
            text.contains(identity),
            "`jigc rename {ty}:bogus`'s refusal must name the one identity `{identity}`; \
             got:\n{text}",
        );
    }

    // The write door, and the index it writes: one identity per fixed-identity doc, and a
    // re-run adds none.
    corpus.jigc_ok(&["ingest"]);
    let first = edge_froms(&repo);
    corpus.jigc_ok(&["ingest"]);
    let second = edge_froms(&repo);
    assert_eq!(
        first, second,
        "a second `jigc ingest` over an unchanged corpus must add no identity",
    );

    // Non-vacuity, asserted rather than hoped for: the fixture must carry at least one
    // edge *originating* at a fixed-identity doc, or the leg below iterates an empty set.
    let owned: Vec<&String> = first
        .iter()
        .filter(|from| {
            docs.iter()
                .any(|(ty, _, _)| from.split_once(':').is_some_and(|(head, _)| head == ty))
        })
        .collect();
    assert!(
        !owned.is_empty(),
        "the fixture must land at least one forward edge FROM a fixed-identity doc, or \
         this arm asserts nothing; the index holds {first:?}",
    );
    for from in owned {
        let (head, _) = from
            .split_once(':')
            .expect("an identity is `<type>:<slug>`");
        let (_, identity, home) = docs
            .iter()
            .find(|(ty, _, _)| ty == head)
            .expect("the head was filtered from this set");
        assert_eq!(
            from, identity,
            "`jigc ingest` indexed the doc at `{home}` under `{from}`, but its one \
             identity is `{identity}` — the whole index holds {first:?}",
        );
    }
}

/// **Three consecutive `jigc unmanage` over each committed fixed-identity doc**: the
/// first drops the identity it names, the next two are honest no-ops, and no forward edge
/// of that doc survives any of them.
///
/// Driven at the base (L-3): run 1 reported `unmanaged VISION.md (vision:vision) —
/// dropped its file-state baseline + forward edges`, runs 2 and 3 reported `no-op:
/// VISION.md is not managed (nothing to drop)`, and `edges.json` carried
/// `{"from":"vision:VISION", …}` throughout. The door dropped the edges of the identity it
/// computes; the ones `adopt` had minted were not that identity, so they were left behind
/// and then denied.
///
/// Each member runs on its **own copy** of the built state, so the arms cannot mask one
/// another through a shared index.
#[test]
fn three_consecutive_unmanages_drop_the_edge_and_claim_no_drop_they_did_not_make() {
    let built = corpus_with_a_landed_fixed_identity_edge();
    built.jigc_ok(&["ingest"]);
    let docs = committed_fixed_identity_docs(&built.repo());

    for (ty, identity, home) in &docs {
        let corpus = built.copy_state();
        let repo = corpus.repo();
        let before = edge_froms(&repo);

        for run in 1..=3 {
            let out = corpus.jigc_ok(&["unmanage", home]);
            if run == 1 {
                assert!(
                    out.contains(identity),
                    "the first `jigc unmanage {home}` must name the identity it drops \
                     (`{identity}`); got:\n{out}",
                );
            } else {
                assert!(
                    out.contains("nothing to drop"),
                    "run {run} of `jigc unmanage {home}` must be an honest no-op; got:\n{out}",
                );
            }
            let survivors: Vec<String> = edge_froms(&repo)
                .into_iter()
                .filter(|from| from.split_once(':').is_some_and(|(head, _)| head == ty))
                .collect();
            assert!(
                survivors.is_empty(),
                "after run {run} of `jigc unmanage {home}`, `{ty}` still owns \
                 {survivors:?} in the edge index — the door dropped one identity and \
                 left another; the index held {before:?} before the sweep",
            );
        }
    }
}

/// **The widened cell, on the manufactured doctype.** [`home_identity`]'s first branch
/// asks `has_fixed_identity()` rather than `placement.is_some()`, which brings the
/// predicate's second disjunct — `singleton: true` over a `location:` home — into the
/// fixed-identity branch. A `location:` home is a *directory*, so unlike a `placement:`
/// literal it can hold siblings, and this is the only shape where that matters.
///
/// A sibling's filename stem is not this doctype's slug and never can be: the slug is the
/// type id. Adopting it under `<ty>:<stem>` would record an edge-index entry
/// [`engine::store`]'s own read guard refuses to resolve — the identity-no-door-can-name
/// class `ingest.unaddressable-identity` exists for — so the sibling is refused, and the
/// refusal names **the doctype's one home** as the destination rather than a slugified
/// version of the name it happens to carry (the repair is *this file is a duplicate*, not
/// *this file is misnamed*).
///
/// The bytes are the ones the real writer staged for the canonical instance, copied to a
/// sibling name: a hand-authored fixture would be asserting against my guess at
/// conformance rather than against the doctype's actual shape.
#[test]
fn a_location_homed_singletons_sibling_is_refused_adoption_and_routed_at_the_one_home() {
    let (repo, home, task) = corpus("singleton-sibling");

    let staged = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(&task)
        .join("docs")
        .join("runbook:runbook.md");
    let conformant = fs::read(&staged).expect("the staged runbook was authored by the real writer");

    let sibling = repo.path().join("docs").join("runbooks").join("other.md");
    fs::create_dir_all(sibling.parent().expect("the runbook home")).expect("mk the runbook home");
    fs::write(&sibling, &conformant).expect("write the sibling");

    let out = jigc(repo.path(), home.path(), &["ingest"]);
    assert_ok(&out, "`jigc ingest`");
    let text = streams(&out);

    assert!(
        text.contains("ingest.unaddressable-identity"),
        "a `runbook` beside the one `runbook` home must be refused adoption, not adopted \
         under `runbook:other`; got:\n{text}",
    );
    assert!(
        text.contains("docs/runbooks/runbook.md"),
        "the refusal must route at the doctype's ONE home, not at a slugified copy of \
         the sibling's own name; got:\n{text}",
    );
    assert!(
        text.contains("needs-reconcile docs/runbooks/other.md")
            && !text.contains("adoptable docs/runbooks/other.md"),
        "the sibling's verdict must be `needs-reconcile`, never `adoptable`; got:\n{text}",
    );
    assert!(
        edge_froms(repo.path())
            .iter()
            .all(|from| from != "runbook:other"),
        "no edge may be indexed under an identity no door can resolve; the index holds \
         {:?}",
        edge_froms(repo.path()),
    );
}
