//! M52 completion audit, fix 4 — **a `store.*` code the contract lists under a declared
//! target form answers the findings envelope at *every* producer.**
//!
//! ## The sentence this closes
//!
//! `design/command-output-contract.md` → *the `store.*` exception* states, of the one code
//! that is deliberately flattened:
//!
//! > `store.malformed-slug` … flattens at every one of its producers, while its `store.*`
//! > siblings — `store.not-found`, `store.no-such-leaf`, `store.unknown-type`, and since
//! > M52 `store.fixed-identity` — are keyed.
//!
//! Driven at `c80b3f8f` that was false at **five** doors and **seven** `(door, cell)`
//! coordinates, not the three doors and four coordinates the audit finding reported:
//!
//! | door | cell | code | arm at `c80b3f8f` |
//! |---|---|---|---|
//! | `jigc migrate --as` | unknown doctype | `store.unknown-type` | flattened |
//! | `jigc relocate` | unknown doctype | `store.unknown-type` | flattened |
//! | `jigc rename` | unknown doctype | `store.unknown-type` | flattened |
//! | `jigc rename` | no such doc | `store.not-found` | flattened |
//! | `jigc task bind` | unknown doctype | `store.unknown-type` | flattened |
//! | `jigc milestone add-from-spec` | unknown doctype | `store.unknown-type` | flattened |
//! | `jigc milestone add-from-spec` | no such doc | `store.not-found` | flattened |
//!
//! The two doors nobody had counted are the ones that make the defect legible, because
//! both diverged **inside themselves**: `jigc task bind spec bogus:x <task>` flattened
//! while `jigc task bind spec spec:nosuch <task>` and `jigc task bind spec
//! changelog:alias <task>` — the next two guards in the same function — enveloped. One
//! door, three consecutive refusals, two wire shapes.
//!
//! ## What is asserted, and by what kind of set
//!
//! Two arms, and they carry different weight on purpose:
//!
//!   1. **The class fence** iterates a **code-side registry**
//!      ([`ENVELOPE_OWED_CODES`]) through the crate's own carrier. It is the completeness
//!      argument: the arm a refusal takes is now a property of its **code**, asked once at
//!      the one carrier, so a producer added tomorrow cannot flatten a class member by
//!      forgetting to pick a constructor. Its negative half drives the declared exception
//!      (`store.malformed-slug`, flattened by a decision the contract states with its two
//!      measured grounds) so the fence cannot pass by enveloping everything.
//!   2. **The driven cells** are a **manufactured** coordinate list and say so. The
//!      `store.unknown-type` half already has a derived-registry sweep over
//!      [`DOCTYPE_DOORS`](cli::cli::DOCTYPE_DOORS) in `unknown_doctype_axis.rs` and is not
//!      restated here; what this arm owns is the `store.not-found` half, whose producing
//!      doors no registry computes — *which doors read a committed doc by address* is not
//!      a shape the clap tree carries. The enveloped controls ride along so the arm would
//!      notice a regression in the other direction.
//!
//! The exit code and the stream are read off [`ENVELOPE_ARMS`]' cross-cutting
//! `Reject::Findings` row rather than written down, exactly as `work_unit_unknown_envelope`
//! does for the work-unit family this sweep's rule was first taken for (M51 Increment 6).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cli::render::{
    ArmOutcome, ArmShape, ENVELOPE_ARMS, ENVELOPE_OWED_CODES, envelope_projecting_findings,
    finding_error,
};
use engine::finding::{Finding, Location, Severity};

/// The fixture's one open task. `implement-from-spec` is the workflow because `jigc task
/// bind` adjudicates the declared `reads` role *before* the address, so a role-less task
/// would never reach the fault this suite is about.
const TASK: &str = "axis";

/// The fixture's one milestone — `milestone add-from-spec` resolves the milestone before
/// it reads the spec, so the cell needs a real one or the door answers about the milestone.
const MILESTONE: &str = "axis-m";

/// The registry arm whose declaration every cell must satisfy.
const REJECT_ARM: &str = "Reject::Findings";

/// The one `store.*` code the contract keeps flattened, with its reason recorded there —
/// the negative half of the class fence, so "everything is enveloped" cannot pass.
const DECLARED_EXCEPTION: &str = "store.malformed-slug";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-store-key-envelope-{tag}-{}-{:?}",
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

/// A real git repo with one commit — the ground `jigc setup` installs into.
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
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
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
}

/// A set-up repo with one open task and one milestone.
struct Fixture {
    repo: TempDir,
    home: TempDir,
}

impl Fixture {
    fn new() -> Self {
        let repo = TempDir::new("repo");
        let home = TempDir::new("home");
        init_repo(repo.path());
        let fixture = Fixture { repo, home };
        fixture.ok(&["setup"]);
        fixture.ok(&[
            "start",
            "--workflow",
            "implement-from-spec",
            "axis intent",
            "--slug",
            TASK,
        ]);
        let minted = fixture.ok(&["milestone", "create", "Axis M"]);
        assert!(
            minted.contains(MILESTONE),
            "the fixture milestone must be `{MILESTONE}`; got: {minted}",
        );
        fixture
    }

    fn run(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("spawn the jigc binary")
    }

    fn ok(&self, args: &[&str]) -> String {
        let out = self.run(args);
        assert!(
            out.status.success(),
            "`jigc {}` must succeed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }
}

/// The `Reject::Findings` row of the production registry — the declaration every driven
/// cell is asserted against, read rather than restated.
fn reject_findings_keys() -> Vec<&'static str> {
    let arm = ENVELOPE_ARMS
        .iter()
        .find(|arm| arm.path.is_empty() && arm.arm == REJECT_ARM)
        .expect("the cross-cutting `Reject::Findings` row is declared");
    assert!(
        matches!(arm.outcome, ArmOutcome::Reject),
        "`{REJECT_ARM}` is a reject arm — stdout empty, the document on stderr",
    );
    match arm.shape {
        ArmShape::Object(keys) => keys.to_vec(),
        _ => panic!("`{REJECT_ARM}` declares an object key set"),
    }
}

/// A blocking finding with `code`, located at `target` — the minimal shape the carrier
/// selects on.
fn blocking(code: &str, target: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        code,
        format!("a manufactured `{code}` for the carrier fence"),
        Some(Location::addressed(target, 1, 1)),
        Some(engine::finding::Route::from(
            "no action needed — this finding exists only inside a test".to_string(),
        )),
    )
}

/// One manufactured coordinate: the argv, and the class code that argv must answer with.
struct Cell {
    /// What the reader sees in a failure message.
    shown: &'static str,
    argv: &'static [&'static str],
    code: &'static str,
}

/// **The cells** — manufactured, because *which doors read a committed doc by address* is
/// a property of what a verb means and not of any table the binary carries.
///
/// **Two** of the three `store.not-found` rows are the ones the fix moved — `rename` and
/// `milestone add-from-spec`. The third (`task bind`) and all three `store.fixed-identity`
/// rows were already enveloped at `c80b3f8f` and ride along as **controls**: they are the
/// halves of the two self-diverging doors that were already right, so a regression in the
/// other direction is caught by the same sweep rather than by nothing, and the row set
/// records which cell of each door was the broken one rather than implying all were.
const CELLS: &[Cell] = &[
    Cell {
        shown: "rename · no such doc",
        argv: &["rename", "adr:nosuch", "--to", "New Title"],
        code: "store.not-found",
    },
    Cell {
        shown: "milestone add-from-spec · no such doc",
        argv: &["milestone", "add-from-spec", MILESTONE, "spec:nosuch"],
        code: "store.not-found",
    },
    Cell {
        shown: "task bind · no such doc",
        argv: &["task", "bind", "spec", "spec:nosuch", TASK],
        code: "store.not-found",
    },
    Cell {
        shown: "rename · fixed-identity alias",
        argv: &["rename", "changelog:alias", "--to", "New Title"],
        code: "store.fixed-identity",
    },
    Cell {
        shown: "milestone add-from-spec · fixed-identity alias",
        argv: &["milestone", "add-from-spec", MILESTONE, "changelog:alias"],
        code: "store.fixed-identity",
    },
    Cell {
        shown: "task bind · fixed-identity alias",
        argv: &["task", "bind", "spec", "changelog:alias", TASK],
        code: "store.fixed-identity",
    },
];

/// **The class fence.** Every member of the owed set, raised through the crate's *flat*
/// carrier, still reaches the envelope — the arm is the code's property, not the door's.
///
/// This is the arm that makes the sweep below evidence rather than the argument: a sixth
/// producer of `store.not-found` written next year picks whichever constructor its author
/// reaches for, and the contract's promise holds anyway.
#[test]
fn every_owed_code_reaches_the_envelope_through_the_flat_carrier() {
    assert!(
        !ENVELOPE_OWED_CODES.is_empty(),
        "an empty owed set would make this fence vacuous",
    );
    for code in ENVELOPE_OWED_CODES {
        let err = finding_error(&blocking(code, "adr:subject"));
        let carried = envelope_projecting_findings(&err).unwrap_or_else(|| {
            panic!(
                "`{code}` is a code the contract lists under a declared target form, so it \
                 owes the findings envelope at every producer — raised through the flat \
                 carrier it still flattened, which is the defect this fence exists for"
            )
        });
        assert_eq!(
            carried.len(),
            1,
            "`{code}`: one refusal is one finding on the envelope arm",
        );
        assert_eq!(
            carried[0].code, *code,
            "`{code}`: the carried identity is kept"
        );
    }

    // The negative half: the one `store.*` code the contract keeps flattened, with its
    // reason recorded there — without this the fence would pass on a carrier that
    // enveloped everything, which is a different contract than the one being kept.
    assert!(
        !ENVELOPE_OWED_CODES.contains(&DECLARED_EXCEPTION),
        "`{DECLARED_EXCEPTION}` is flattened by a decision the contract states with its \
         two measured grounds — a null-target key is less informative than the message \
         that names the offending token — so it may not join the owed set by drift",
    );
    assert!(
        envelope_projecting_findings(&finding_error(&blocking(DECLARED_EXCEPTION, "adr:../../x")))
            .is_none(),
        "`{DECLARED_EXCEPTION}` stays on the flattened arm",
    );
}

/// **The driven cells** — each coordinate answers the `Reject::Findings` document with a
/// non-null `(code, target)` key.
#[test]
fn every_store_class_cell_answers_the_findings_envelope() {
    let fixture = Fixture::new();
    let declared = reject_findings_keys();

    for cell in CELLS {
        assert!(
            ENVELOPE_OWED_CODES.contains(&cell.code),
            "`{}` drives `{}`, which is not in the owed set this suite is about",
            cell.shown,
            cell.code,
        );
        let mut argv: Vec<&str> = cell.argv.to_vec();
        argv.extend(["--format", "json"]);
        let shown = cell.shown;

        let out = fixture.run(&argv);
        assert!(
            out.stdout.is_empty(),
            "{shown}: a reject leaves stdout empty; stdout:\n{}",
            String::from_utf8_lossy(&out.stdout),
        );
        assert_eq!(
            out.status.code(),
            Some(1),
            "{shown}: the class rejects at exit 1; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );

        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        let value: serde_json::Value = serde_json::from_str(&stderr).unwrap_or_else(|err| {
            panic!("{shown}: stderr must parse as the reject envelope ({err}); got:\n{stderr}")
        });
        let object = value
            .as_object()
            .unwrap_or_else(|| panic!("{shown}: the envelope is a JSON object; got:\n{stderr}"));
        let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
        keys.sort_unstable();
        let mut want = declared.clone();
        want.sort_unstable();
        assert_eq!(
            keys, want,
            "{shown}: a code the contract lists under a target form answers on the \
             `{REJECT_ARM}` arm — not the flattened `{{\"error\": …}}`, whose code lives \
             inside a message and projects no key; got:\n{stderr}",
        );

        let findings = value["findings"]
            .as_array()
            .unwrap_or_else(|| panic!("{shown}: `findings` is an array; got:\n{stderr}"));
        assert_eq!(
            findings.len(),
            1,
            "{shown}: one refusal is one finding; got:\n{stderr}",
        );
        let key = &findings[0]["key"];
        assert_eq!(
            key["code"], cell.code,
            "{shown}: the key carries this cell's code; got:\n{stderr}",
        );
        assert!(
            !key["target"].is_null(),
            "{shown}: the doc-URI target form is what makes `(code, target)` resolve; a \
             null target is the mis-filing the contract refuses a family over; got:\n{stderr}",
        );
    }
}
