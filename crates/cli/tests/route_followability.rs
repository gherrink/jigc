//! M47 Increment 6, T3 — **P6 route-followability through the real binary**
//! (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 8 + the pre-decompose review's P6
//! rider; `design/surface-contract.md` → The route fence).
//!
//! The M43 route fence proves a mechanical route **parses**. It cannot prove the route an
//! agent actually reads is **followable**, because the fence sits on `Route::mechanical`
//! and never sees the finding: a route whose argv still carries `<doctype>` or `<address>`
//! passes the fence — those tokens are declared members of the fence's own dummy table —
//! and then reaches a driver as a command it cannot run. P6 closes that: *a placeholder
//! whose value is derivable from the finding's own `key.target` must be substituted*,
//! asserted at the finding-**serialization** seam (the fourth assert there).
//!
//! This suite is the end-to-end half. For each shipped route family it provokes the block
//! through the built binary, reads the emitted `route` out of the `--format json` findings
//! envelope, and then **runs the emitted backticked argv verbatim** — the emitted bytes are
//! the contract, never a reconstruction — asserting exit 0:
//!
//!   - the `<doctype>`-bearing write reject (`write.wrong-shape`) names the real doctype,
//!     and `jigc doc schema <that doctype>` runs;
//!   - the `write.not-present` item-id miss names the real containing section **and** the
//!     real task id, and that `jigc doc show … --task …` runs;
//!   - the `schema-conformance.*` gate block names its real write address, and that
//!     `jigc doc set-slot … --from-file -` runs and clears the finding.
//!
//! **The axis (M49 Increment 8 / T3).** The third arm below is no longer one pinned repro
//! but the whole **route-followability axis**: every gate repair code
//! ([`engine::validate::conformance_repair_codes`] — the mechanical arms of the engine's own
//! `conformance_route` map) crossed with every boundary door
//! ([`cli::render::BOUNDARY_DOORS`] — the code-side door registry). Both sides are read from
//! code, and a door the registry gains with no arm here is a hard panic, not a silent skip.
//! Every cell runs **in a repo with two open tasks**, which is the state that made the
//! emitted argv unrunnable: a task-selector-less `jigc doc` write exits 1 on `more than one
//! active task`, so a blocked gate's only stated exit could not be taken.
//!
//! **Declared bound, recorded not glossed:** the *un-enriched* `write.not-present` fallback
//! (`engine::write::write_route`) has no binary-reachable producer today — M47 Inc 6 T2
//! wired the enrichment onto all six write verbs, and the batch `jigc doc author` path
//! lowers every item hop with the same `slugify` the engine mints ids with, so it cannot
//! address an item it did not just add. Its substitution is therefore pinned where it is
//! reachable, in `crates/engine/src/write.rs` →
//! `mod not_present_route_followability`; the arm below drives the **enriched** producer,
//! which is what an agent meets.

use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use cli::pack::EmbeddedPack;
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{Leaf, Repeatable, Schema, SectionBody};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-route-followability-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — byte-identical to the binary-embedded pack.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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

/// A live fixture: a `git init` repo with the project layer, `jigc setup` run, and one
/// open `single-task` task (so its transient `commit` doc is provisioned).
struct Fixture {
    repo: TempDir,
    home: TempDir,
    task: String,
}

impl Fixture {
    fn new(tag: &str) -> Self {
        let fixture = Fixture::bare(tag);
        fixture.ok(
            &["start", "--workflow", "single-task", "add a widget"],
            None,
            "jigc start --workflow single-task",
        );
        fixture
    }

    /// The installed repo with **no** task yet — the shared half of [`Fixture::new`] and
    /// [`Fixture::milestone`], whose sub-task areas are minted by `milestone add-task`.
    fn bare(tag: &str) -> Self {
        let repo = TempDir::new(tag);
        let home = TempDir::new("home");
        git(repo.path(), &["init", "-q"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-q", "-m", "initial"]);
        fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

        let fixture = Fixture {
            repo,
            home,
            task: "add-a-widget".to_owned(),
        };
        fixture.ok(&["setup"], None, "jigc setup");
        fixture
    }

    /// Run a `jigc` subcommand against this fixture, optionally piping `stdin`.
    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", dev_pack())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if stdin.is_some() {
            command.stdin(Stdio::piped());
        }
        let mut child = command.spawn().expect("spawn the jigc binary");
        if let Some(bytes) = stdin {
            child
                .stdin
                .take()
                .expect("stdin piped")
                .write_all(bytes)
                .expect("write stdin");
        }
        child.wait_with_output().expect("wait for jigc")
    }

    /// Run a `jigc` subcommand and assert exit 0, returning trimmed stdout.
    fn ok(&self, args: &[&str], stdin: Option<&[u8]>, what: &str) -> String {
        let out = self.run(args, stdin);
        assert!(
            out.status.success(),
            "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout)
            .expect("utf-8 stdout")
            .trim_end_matches('\n')
            .to_owned()
    }

    /// **Follow the emitted route**: split the backticked argv verbatim, drop the leading
    /// `jigc`, and run exactly those bytes — the followability proof. Asserts exit 0.
    ///
    /// `subs` is the **declared non-derivable** placeholder set the agent fills in
    /// (`engine::finding::ROUTE_PLACEHOLDERS` — `<value>` is the agent's to author); every
    /// other `<…>` token surviving into the emitted argv is a P6 fence violation and fails
    /// here rather than being quietly substituted. Whether the route reads stdin is read off
    /// the emitted bytes too (`--from-file -`), never decided by the caller.
    fn follow(&self, route: &str, subs: &[(&str, &str)], what: &str) -> String {
        let cmd = backticked(route);
        let mut parts = cmd.split_whitespace();
        assert_eq!(parts.next(), Some("jigc"), "a route leads with `jigc`");
        let args: Vec<String> = parts
            .map(|token| {
                subs.iter()
                    .find(|(placeholder, _)| *placeholder == token)
                    .map(|(_, value)| (*value).to_owned())
                    .unwrap_or_else(|| token.to_owned())
            })
            .collect();
        for arg in &args {
            assert!(
                !(arg.starts_with('<') && arg.ends_with('>')),
                "`{what}` emitted the undeclared placeholder `{arg}` — a followed route may \
                 only carry the declared agent-owned tokens {subs:?}; route: {route}"
            );
        }
        let stdin: Option<&[u8]> = args
            .iter()
            .any(|arg| arg == "--from-file")
            .then_some(b"Prose the agent authored.\n");
        let borrowed: Vec<&str> = args.iter().map(String::as_str).collect();
        self.ok(&borrowed, stdin, what)
    }

    /// Open a **second** active task, so every axis cell runs in the state that made the
    /// emitted argv unrunnable: with one task open a selector-less `jigc doc` write resolves
    /// it implicitly and the defect is invisible.
    fn open_second_task(&self) {
        self.ok(
            &["start", "--workflow", "single-task", "a second thing"],
            None,
            "jigc start --workflow single-task (the second task)",
        );
        let listed = self.ok(&["task", "list"], None, "jigc task list");
        assert!(
            listed.contains(&self.task) && listed.contains("2 active task(s)"),
            "two tasks are open, which is what makes a selector-less write ambiguous; got:\n{listed}"
        );
    }

    /// A **fan-out** fixture: the same repo, with a milestone holding two sub-tasks and no
    /// standalone task. The sub-task areas are themselves active tasks, so this door always
    /// stands in the ≥2-task state by construction.
    fn milestone(tag: &str) -> Self {
        let fixture = Fixture::bare(tag);
        fixture.ok(
            &["milestone", "create", "Cache rework"],
            None,
            "jigc milestone create",
        );
        for intent in ["Doc area", "Code area"] {
            fixture.ok(
                &["milestone", "add-task", MILESTONE, intent],
                None,
                "jigc milestone add-task",
            );
        }
        fixture
    }

    /// Stage `body` into a milestone sub-task's `docs/` area with `created` provenance,
    /// exactly as the staging primitives would (the `milestone_boundary_gate.rs` idiom — no
    /// front-door verb stages into a milestone sub-area).
    fn stage_sub_doc(&self, sub: &str, address: &str, body: &str) {
        let docs = self
            .repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join(sub)
            .join("docs");
        fs::create_dir_all(&docs).expect("mk the sub-area docs/");
        fs::write(docs.join(format!("{address}.md")), body).expect("write the staged body");
        let manifest = docs.join("provenance.json");
        let mut record: serde_json::Value = fs::read_to_string(&manifest)
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok())
            .unwrap_or_else(|| serde_json::json!({ "docs": {} }));
        record["docs"][address] = serde_json::Value::String("created".to_owned());
        fs::write(
            &manifest,
            serde_json::to_string_pretty(&record).expect("serialize the provenance manifest"),
        )
        .expect("write the provenance manifest");
    }
}

/// The leading backticked command of a route string (`` `<cmd>`<tail> ``).
fn backticked(route: &str) -> &str {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("a mechanical route carries a backticked command; got: {route}"))
}

/// The `route` of the first blocking-envelope finding carrying `code` — [`blocking_finding`]
/// projected to the one field the two placeholder arms assert on.
fn route_of(out: &Output, code: &str, what: &str) -> String {
    let finding = blocking_finding(out, code, what);
    finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("`{what}`'s `{code}` carries a route; got:\n{finding}"))
        .to_owned()
}

/// Parse a blocking `--format json` findings envelope out of `out` (either stream — the
/// write verbs block on stderr, the task gate reports on stdout) and return the first
/// finding carrying `code`.
fn blocking_finding(out: &Output, code: &str, what: &str) -> serde_json::Value {
    assert!(
        !out.status.success(),
        "`{what}` must block (non-zero exit); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let report: serde_json::Value = serde_json::from_str(stderr.trim())
        .or_else(|_| serde_json::from_str(stdout.trim()))
        .unwrap_or_else(|e| {
            panic!("`{what}` emits a JSON envelope: {e}; stdout:\n{stdout}\nstderr:\n{stderr}")
        });
    let findings = report["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` carries a findings array; got:\n{report}"));
    findings
        .iter()
        .find(|f| f["code"] == code)
        .unwrap_or_else(|| panic!("`{what}` carries a `{code}` finding; got:\n{report}"))
        .clone()
}

/// **The `<doctype>` family.** A genuine shape question (`add-item` into a section the
/// schema declares non-repeatable) routes `jigc doc schema <doctype>` — the placeholder is
/// derivable from the finding's own `key.target` (`commit:<task>#summary` → `commit`), so
/// the emitted route names the real doctype and runs.
#[test]
fn a_shape_question_route_names_the_real_doctype_and_runs() {
    let fx = Fixture::new("doctype");
    let address = format!("commit:{}#summary", fx.task);

    let out = fx.run(
        &[
            "doc", "add-item", &address, "--title", "Nope", "--format", "json",
        ],
        None,
    );
    let route = route_of(
        &out,
        "write.wrong-shape",
        "add-item into a non-repeatable section",
    );
    assert_eq!(
        backticked(&route),
        "jigc doc schema commit",
        "the shape question names the doctype its own target carries; route:\n{route}",
    );

    let shown = fx.follow(&route, &[], "the emitted `jigc doc schema`");
    assert!(
        shown.contains("summary"),
        "the followed route projects the doctype's declared shape; got:\n{shown}",
    );
}

/// **The `write.not-present` family.** An item-id miss names the followable containing
/// section *and* the real task id — `<address>` derived from `key.target`, `<task-id>` from
/// the dispatch context that resolved the task (the declared non-derivable placeholder's
/// "different source"). Both are concrete, so the emitted read runs.
#[test]
fn an_item_id_miss_route_names_the_real_address_and_task_and_runs() {
    let fx = Fixture::new("notpresent");
    let trailers = format!("commit:{}#trailers", fx.task);
    fx.ok(
        &["doc", "add-item", &trailers, "--title", "Refs"],
        None,
        "add-item a real trailer",
    );

    let out = fx.run(
        &[
            "doc",
            "retitle-item",
            &format!("{trailers}/nonesuch"),
            "--title",
            "Reviewed-by",
            "--format",
            "json",
        ],
        None,
    );
    let route = route_of(&out, "write.not-present", "retitle-item at an absent item");
    assert_eq!(
        backticked(&route),
        format!("jigc doc show {trailers} --task {}", fx.task),
        "the item-id miss names the real containing section and the real task; route:\n{route}",
    );

    let shown = fx.follow(&route, &[], "the emitted `jigc doc show`");
    assert!(
        shown.contains("Refs"),
        "the followed route reveals the section's live item ids; got:\n{shown}",
    );
}

/// **The gate repair route axis (M49 Increment 8 / T3): every code, at every boundary door.**
///
/// Rows are `engine::validate::conformance_repair_codes()` — the **mechanical** arms of the
/// engine's own `conformance_route` map, derived by asking it rather than by re-listing them.
/// Columns are [`cli::render::BOUNDARY_DOORS`] — the code-side registry of doors the shared
/// `engine::validate::validate_task` entry stands behind. Neither side is hand-listed here,
/// and a door added to the registry with no arm below panics rather than silently skipping.
///
/// Every cell is driven **in a repo with two open tasks**. That state is the whole point: a
/// `jigc doc` write with no `--task` selector exits 1 on `more than one active task`, so
/// before this axis a blocked gate at any of the three doors printed, as its only stated
/// exit, a command the agent could not run — while the door had been *given* the id (the two
/// task doors take it as their own argument; the milestone door reads it off the merged
/// doc's contributing sub-task). Each cell asserts the emitted argv names the task, runs
/// that argv, and re-runs the door to prove the finding it emitted is gone.
#[test]
fn every_gate_repair_route_runs_at_every_boundary_door() {
    let codes = engine::validate::conformance_repair_codes();
    assert!(
        !codes.is_empty(),
        "`conformance_repair_codes` derives the mechanical arms of `conformance_route`; \
         an empty set means the derivation broke, not that the axis is done"
    );
    let mut driven: BTreeSet<(String, String)> = BTreeSet::new();
    for door in cli::render::BOUNDARY_DOORS {
        for code in &codes {
            match *door {
                ["task", verb] => task_door_cell(verb, code),
                ["milestone", "finalize"] => milestone_door_cell(code),
                other => panic!(
                    "`cli::render::BOUNDARY_DOORS` gained `jigc {}`, which this axis has no \
                     arm for. A door that emits gate blocks must have its repair routes \
                     proven runnable at it — add the arm, do not narrow the registry.",
                    other.join(" ")
                ),
            }
            driven.insert((door.join(" "), (*code).to_owned()));
        }
    }
    assert_eq!(
        driven.len(),
        cli::render::BOUNDARY_DOORS.len() * codes.len(),
        "every (code, door) cell of the axis is driven exactly once"
    );
}

/// One **task-door** cell (`jigc task validate` / `jigc task finalize`, whose shared
/// `TaskContext::validate` entry both doors reach). Provokes `code` on the task's staged
/// `commit` doc, reads the emitted route out of the door's own `--format json` envelope,
/// runs it, and re-runs the door to prove the finding cleared.
fn task_door_cell(door_verb: &str, code: &str) {
    let fx = Fixture::new(&format!("axis-task-{door_verb}-{}", tag(code)));
    fx.open_second_task();
    provoke_at_task_door(&fx, code);

    let door = ["task", door_verb, fx.task.as_str(), "--format", "json"];
    let what = format!("`jigc task {door_verb}` over the provoked staged commit doc");
    let finding = blocking_finding(&fx.run(&door, None), code, &what);
    let (route, target) = route_and_target(&finding, &what);

    assert_names_the_task(&route, &fx.task, &what);
    // `<value>` is the one placeholder the emitted route legitimately keeps — the agent's to
    // author (`engine::finding::ROUTE_PLACEHOLDERS`). `feat` is a member of the `commit`
    // header `type` enum, which is the only field these cells write.
    fx.follow(
        &route,
        &[("<value>", "feat")],
        &format!("the emitted route of {what}"),
    );

    assert_cleared(&fx, &door, code, &target, &what);
}

/// The state that makes `code` fire at a task door, built on the pristine `single-task`
/// commit skeleton (`type:` present-but-empty, every slot empty):
///
///   * `required-slot-present` — the empty `## Summary` slot, pristine;
///   * `field-value-conformant` — the empty `type:` value, which is no member of the header
///     enum; pristine too;
///   * `required-field-present` — the `type:` line removed, so the author-required field is
///     absent rather than empty. No front-door verb produces this state (`--unset` refuses a
///     non-optional field, which is the correct refusal), so the fixture writes the staged
///     body; the *route* under test is still the binary's own emitted bytes.
///
/// A code with no declared provocation panics: an axis that silently skips a row proves
/// nothing about it.
fn provoke_at_task_door(fx: &Fixture, code: &str) {
    match code {
        "schema-conformance.required-slot-present"
        | "schema-conformance.field-value-conformant" => {}
        "schema-conformance.required-field-present" => {
            let path = fx
                .repo
                .path()
                .join(".jigc")
                .join("tasks")
                .join(&fx.task)
                .join("docs")
                .join(format!("commit:{}.md", fx.task));
            let body = fs::read_to_string(&path).expect("the staged commit doc reads back");
            let stripped = body.replacen("type: \n", "", 1);
            assert_ne!(
                stripped, body,
                "the pristine commit skeleton carries the `type:` line this cell removes"
            );
            fs::write(&path, stripped).expect("write the staged commit doc");
        }
        other => panic!(
            "no task-door provocation is declared for `{other}` — the gate repair code set \
             grew and this axis must drive the new row, not skip it"
        ),
    }
}

/// One **milestone-boundary** cell (`jigc milestone finalize`). Two of the three codes are
/// producible over the merged effective state; the third's absence is *derived* below rather
/// than asserted in prose.
fn milestone_door_cell(code: &str) {
    match code {
        // The merged ADR whose required `## Decision` slot is empty.
        "schema-conformance.required-slot-present" => {
            drive_milestone_cell(code, adr_body("Broken policy", "accepted", ""), "accepted");
        }
        // The merged ADR whose `status:` value is no member of the declared enum.
        "schema-conformance.field-value-conformant" => {
            drive_milestone_cell(
                code,
                adr_body("Broken policy", "bogus", "Do the thing."),
                "accepted",
            );
        }
        // **No producer at this door, and the premise is fenced, not narrated.** The merged
        // gate validates the **persisted** subset only (a transient `commit:<sub-task>`
        // skeleton renders into a message and is filtered out before the sweep), and
        // `required-field-present` fires only for an *author-required* field. No dev-pack
        // doctype with a committed home declares one — every header field is defaulted,
        // `set:`-derived, an optional `ref`, or a pack-declared anchor, and every repeatable
        // block field is its item's `id-from` (exempt at the heading) or a slot. So the cell
        // has no reachable state. The assertion below is that premise: if a persisted
        // doctype ever gains an author-required field, this reddens and the next author
        // drives the cell instead of inheriting a comment.
        "schema-conformance.required-field-present" => {
            let carriers = persisted_author_required_fields();
            assert!(
                carriers.is_empty(),
                "`{code}` now has a producer at `jigc milestone finalize`: the persisted \
                 doctype(s) {carriers:?} declare an author-required field, so the merged gate \
                 can raise it. Drive this cell — stage such a doc into a sub-task area with \
                 the field absent, follow the emitted route, and re-run the door."
            );
        }
        other => panic!(
            "no milestone-door disposition is declared for `{other}` — the gate repair code \
             set grew and this axis must either drive the new row or fence why it cannot"
        ),
    }
}

/// Drive one producible milestone-boundary cell: stand up a two-sub-task fan-out, stage
/// `body` as `adr:broken-policy` into the first sub-area, block the join's boundary gate,
/// follow the emitted route, and re-run `finalize` to prove the finding cleared.
fn drive_milestone_cell(code: &str, body: String, value: &str) {
    let fx = Fixture::milestone(&format!("axis-ms-{}", tag(code)));
    fx.stage_sub_doc(SUB_DOC_AREA, "adr:broken-policy", &body);
    fx.stage_sub_doc(
        SUB_CODE_AREA,
        "adr:code-policy",
        &adr_body("Code policy", "accepted", "Do the other thing."),
    );
    fx.ok(
        &["milestone", "provision", MILESTONE],
        None,
        "jigc milestone provision",
    );

    let door = ["milestone", "finalize", MILESTONE, "--format", "json"];
    let what = "`jigc milestone finalize` over the merged effective state".to_owned();
    let finding = blocking_finding(&fx.run(&door, None), code, &what);
    let (route, target) = route_and_target(&finding, &what);

    // The doc came from `doc-area`, and that is the area the repair has to land in — the
    // merged copy is rebuilt from it on the next join.
    assert_names_the_task(&route, SUB_DOC_AREA, &what);
    fx.follow(
        &route,
        &[("<value>", value)],
        &format!("the emitted route of {what}"),
    );
    assert_cleared(&fx, &door, code, &target, &what);
}

/// The milestone the fan-out cells drive, and its two sub-task areas.
const MILESTONE: &str = "cache-rework";
const SUB_DOC_AREA: &str = "doc-area";
const SUB_CODE_AREA: &str = "code-area";

/// A conformant-shaped ADR body with two levers the cells turn: the header `status:` value
/// and the `## Decision` slot's prose (empty prose leaves the required slot unfilled).
fn adr_body(title: &str, status: &str, decision: &str) -> String {
    format!(
        "---\nstatus: {status}\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n\
         ## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\n{decision}\n\n\
         ## Consequences\n\nTradeoffs.\n"
    )
}

/// A filesystem-safe tag for a finding code (`schema-conformance.required-slot-present` →
/// `required-slot-present`) — the per-cell temp-dir discriminator.
fn tag(code: &str) -> &str {
    code.rsplit('.').next().unwrap_or(code)
}

/// The `(route, target)` of a finding — the emitted bytes under test and the stable
/// `key.target` the re-run checks for.
fn route_and_target(finding: &serde_json::Value, what: &str) -> (String, String) {
    let route = finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("{what}'s finding carries a route; got:\n{finding}"))
        .to_owned();
    let target = finding["key"]["target"]
        .as_str()
        .unwrap_or_else(|| panic!("{what}'s finding carries a key target; got:\n{finding}"))
        .to_owned();
    (route, target)
}

/// Assert the emitted argv names `task` through the `--task` selector — read off the
/// **emitted bytes**, never rebuilt: an argv that merely mentions the id elsewhere would
/// still not run.
fn assert_names_the_task(route: &str, task: &str, what: &str) {
    let argv: Vec<&str> = backticked(route).split_whitespace().collect();
    let at = argv.iter().position(|token| *token == "--task");
    assert_eq!(
        at.and_then(|at| argv.get(at + 1)).copied(),
        Some(task),
        "{what}'s repair route must name the task it has to run in — with a second task open \
         a selector-less `jigc doc` write exits 1. Got: {route}"
    );
}

/// Re-run `door` and assert no finding with this `(code, target)` key survives. An exit-0
/// re-run carries no envelope at all, which is the strongest form of cleared.
fn assert_cleared(fx: &Fixture, door: &[&str], code: &str, target: &str, what: &str) {
    let out = fx.run(door, None);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let Ok(report) = serde_json::from_str::<serde_json::Value>(stdout.trim())
        .or_else(|_| serde_json::from_str::<serde_json::Value>(stderr.trim()))
    else {
        assert!(
            out.status.success(),
            "the re-run of {what} either reports an envelope or exits 0; stdout:\n{stdout}\n\
             stderr:\n{stderr}"
        );
        return;
    };
    let survivors: Vec<&serde_json::Value> = report["findings"]
        .as_array()
        .map(|findings| {
            findings
                .iter()
                .filter(|f| f["key"]["code"] == code && f["key"]["target"] == target)
                .collect()
        })
        .unwrap_or_default();
    assert!(
        survivors.is_empty(),
        "the followed route must clear the finding that emitted it — `({code}, {target})` \
         survives the re-run of {what}; got:\n{report}"
    );
}

/// Every **persisted** dev-pack doctype's author-required fields, as
/// `"<doctype>.<section-or-item-path>/<field>"` — the derivation the milestone door's
/// `required-field-present` cell rests on. Persisted means the doctype declares a committed
/// home (`location:` or `placement:`), which is exactly the merged gate's own filter; the
/// per-field verdict is `engine::validate::is_author_required`, the shared predicate the
/// check itself consults. A repeatable block's `id-from` field is skipped because the check
/// skips it too (the id source is the item heading, never a bullet).
fn persisted_author_required_fields() -> Vec<String> {
    let pack = EmbeddedPack::new();
    let mut carriers = Vec::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .expect("a listed schema reads back");
        let schema: Schema =
            cli::pack::load_pack_schema(&pack, &bytes).expect("a shipped schema parses");
        if schema.location.is_none() && schema.placement.is_none() {
            continue;
        }
        for section in &schema.sections {
            match &section.body {
                SectionBody::Simple { fields, .. } => {
                    for field in fields {
                        if engine::validate::is_author_required(field) {
                            carriers.push(format!("{}.{}/{}", schema.ty, section.id, field.id));
                        }
                    }
                }
                SectionBody::Repeatable { repeatable } => {
                    collect_block(&schema.ty, &section.id, repeatable, &mut carriers);
                }
            }
        }
    }
    carriers
}

/// The repeatable half of [`persisted_author_required_fields`], recursing into nested
/// repeatables one path segment deeper.
fn collect_block(doctype: &str, path: &str, repeatable: &Repeatable, carriers: &mut Vec<String>) {
    for leaf in &repeatable.block {
        match leaf {
            Leaf::Field(field) if field.id != repeatable.id_from => {
                if engine::validate::is_author_required(field) {
                    carriers.push(format!("{doctype}.{path}/{}", field.id));
                }
            }
            Leaf::Field(_) | Leaf::Slot { .. } => {}
            Leaf::Repeatable {
                id,
                repeatable: nested,
            } => collect_block(doctype, &format!("{path}/{id}"), nested, carriers),
        }
    }
}
