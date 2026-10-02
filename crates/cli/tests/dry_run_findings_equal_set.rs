//! M51 Increment 5 / T5 — **the equal-set fence**: `jigc task validate`,
//! `jigc task finalize --dry-run` and the committing `jigc task finalize` emit **one**
//! finding-code set (EC-20; `implementation/roadmap.md` → Milestone 51, Increment 5 →
//! Riders; `completions/artifacts/M51/settle-record.md` → D5, the S5 clause) — **joined at
//! M52 Increment 10 / T3 by the fence's other half**: what the **text** arm says about the
//! findings the envelope carries, at the forecast door and at every `DocAck` write ack
//! (per-axis review row D-1; `completions/artifacts/M52/settle-record.md` → D11). The two
//! halves share a home because they are one claim read twice — *the surfaces of one door do
//! not disagree about what it found* — and the bound struck below is the M51 half's own
//! sentence that let the second half ship broken.
//!
//! **What was broken.** Driven at `2f9f7993`, one task, one moment: `task validate`
//! emitted `file-state.staged-copy` and `changelog-recording.gate-granted-unused`, the
//! landed `task finalize` emitted the *identical* pair — and `--dry-run` emitted
//! `{dry_run, left_out, manifest, subject}`, **no `findings` key at all**, dropping both.
//! QUICKSTART presents `task validate` and `finalize --dry-run` as one preview surface,
//! so the forecast whose whole job is *"tell me what this finalize will do"* was silent
//! about every finding the door it forecasts would report — a value computed three lines
//! above the branch and discarded.
//!
//! **Why a registry and not the two reported codes.** `cli::gate_coverage` already owns
//! the membership of what `task validate` previews, and it says in its own words that it
//! fences what surfaces *say*, never what a door *emits* — so this suite iterates
//! [`Tier::Previewed`] **behaviourally**. Every member gets a corpus built to make it
//! fire, and a member joining the tier without a row here reddens
//! [`every_previewed_member_carries_a_row`]. That is the axis
//! ([pinning.md](../../../implementation/pinning.md) §1): the class is *the previewed
//! set*, not the pair EC-20 happened to report.
//!
//! **The fence as restated at the Settle (S5):** `validate` == `--dry-run` == **the
//! committing door's emission, landed *or* blocked** — never a three-way equality that
//! presumes a landing, because `Tier::Previewed` holds **blocking** members
//! (`finalize.carried-staged`, `owner-artifact.present`) and in the state that makes one
//! fire `task finalize` does not land. Each row therefore builds a corpus in which **its
//! own member is the only thing that fires**: a door that refuses emits the refusal's own
//! findings (`TaskArea::blocked`, the shipped planner-block surface), so a state mixing a
//! blocking member with an advisory would compare a short-circuited set against a
//! complete one and prove nothing about either.
//!
//! **The axis is the sweep-invoked members.** Since M52 a previewed member also states
//! *where* in the preview it runs ([`gate_coverage::Invocation`]), and this suite scopes
//! itself to [`gate_coverage::Invocation::PreviewGates`] — the members the shared
//! task-scope sweep computes and all three doors report in a `findings` envelope on
//! **stdout**. The `SeparatelyAtDoor` member (the repository-posture family) is a
//! *refusal*: all three doors decline before the sweep runs, and the refusal document
//! rides **stderr**, so `emitted_codes` would read it from the wrong stream. Its own
//! three-door equality is pinned — byte-identically, over the whole git-state axis —
//! by `validate_previews_posture.rs`, so the property this suite asserts holds for that
//! member too; what differs is the stream it is read from, and the scoping is stated on
//! the row rather than special-cased by member id.
//!
//! **Declared bounds.**
//!
//!   * **`--carry-staged` is outside the fence.** It is a *consent* flag: declared, the
//!     carryover gate is omitted at both doors by construction, so the equality it would
//!     assert is the one `validate_previews_the_gate.rs` already pins.
//!   * **Exit codes are not compared, and deliberately so.** With the forecast now
//!     carrying a previewed *blocking* finding, `--dry-run` still exits 0 while the other
//!     two exit 3. Making the forecast refuse is the M47 Inc 4 / T3 move — *the forecast
//!     obeys the gate it forecasts* — applied to a second member, and that is a change to
//!     `--dry-run`'s exit contract this task does not carry. The finding is on the wire,
//!     which is what EC-20 asked for.
//!   * ~~**The text surface is unchanged.**~~ **Struck at M52 Increment 10 / T3** (per-axis
//!     review row D-1). The M51 rider reasoned that *the standing parity fence runs text →
//!     envelope, so a JSON-only addition is inside it* — true about the fence and false
//!     about the surface: driven at `528f1eda` on a granted-and-unused changelog gate, the
//!     forecast's default agent arm named no finding while its envelope carried the
//!     advisory, so the agent reading the surface jigc actually hands it was told a gated
//!     forecast was clean. The two text-arm fences below
//!     ([`the_dry_run_text_arm_names_every_code_its_envelope_carries`],
//!     [`every_doc_ack_arm_renders_a_carried_finding`]) close it over the class rather than
//!     over the one code the review reported.
//!
//! Every assertion runs on the **emitted bytes** of the real binary
//! (`CARGO_BIN_EXE_jigc`) over throwaway `git init` repos — with the one stated exception
//! of the `DocAck` sweep, which drives [`render::doc_ack`] directly because eight of its
//! nine arms have no known non-empty producer to drive (see its own bound).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use cli::cli::Format;
use cli::gate_coverage::{self, Tier};
use cli::render::{self, AckTarget, DocAck};
use engine::finding::{Finding, Findings, Location, Route, Severity};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-equal-set-{tag}-{}-{:?}",
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

/// One corpus: an isolated `$HOME`, a `git init` repo, and a directory a row may seed a
/// fixture pack into. All three live under one self-cleaning root.
struct Corpus {
    root: TempDir,
}

impl Corpus {
    fn new(tag: &str) -> Self {
        let root = TempDir::new(tag);
        for sub in ["home", "repo", "pack"] {
            fs::create_dir_all(root.path().join(sub)).expect("create corpus subdir");
        }
        let corpus = Corpus { root };
        init_repo(corpus.repo());
        ok(&corpus, &["setup"], "jigc setup");
        corpus
    }

    fn repo(&self) -> PathBuf {
        self.root.path().join("repo")
    }

    fn home(&self) -> PathBuf {
        self.root.path().join("home")
    }

    fn pack(&self) -> PathBuf {
        self.root.path().join("pack")
    }
}

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit.
fn init_repo(repo: PathBuf) {
    git(&repo, &["init", "-q"]);
    git(&repo, &["config", "user.email", "test@example.com"]);
    git(&repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(&repo, &["add", "README.md"]);
    git(&repo, &["commit", "-q", "-m", "initial"]);
}

/// Drive the real binary against `corpus`, optionally feeding stdin.
fn jigc(corpus: &Corpus, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(corpus.repo())
        .env("HOME", corpus.home())
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Drive the real binary from `cwd` rather than the repository root — the one cell that needs
/// it is a milestone sub-task, whose re-entry door is pinned to the milestone's base and so
/// only composes from the provisioned worktree.
fn jigc_in(corpus: &Corpus, cwd: &Path, args: &[&str], what: &str) {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(cwd)
        .env("HOME", corpus.home())
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary");
    assert!(
        out.status.success(),
        "`{what}` must exit 0 from {cwd:?}; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Drive `jigc <args>`, asserting exit 0, returning trimmed stdout.
fn ok(corpus: &Corpus, args: &[&str], what: &str) -> String {
    let out = jigc(corpus, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_string()
}

/// Drive `jigc doc set-slot <addr>` with `prose` on stdin, asserting exit 0.
fn set_slot(corpus: &Corpus, addr: &str, prose: &[u8]) {
    let out = jigc(
        corpus,
        &["doc", "set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    assert!(
        out.status.success(),
        "`doc set-slot {addr}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The id of the one task the corpus has open, read back through `jigc task list`.
fn the_open_task(corpus: &Corpus) -> String {
    let listing = ok(corpus, &["task", "list", "--format", "json"], "task list");
    let tasks: serde_json::Value = serde_json::from_str(&listing).expect("task list is JSON");
    let array = tasks.as_array().expect("task list is an array");
    assert_eq!(array.len(), 1, "exactly one open task; got {tasks}");
    array[0]["id"].as_str().expect("the task id").to_string()
}

/// Author the commit doc's author-required leaves, so the finalize plan is reachable.
fn fill_commit(corpus: &Corpus, task: &str, ty: &str) {
    ok(
        corpus,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            ty,
        ],
        "set-field commit type",
    );
    ok(
        corpus,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            "gate",
        ],
        "set-field commit scope",
    );
    set_slot(
        corpus,
        &format!("commit:{task}#summary"),
        b"drive the gate\n",
    );
    set_slot(corpus, &format!("commit:{task}#body"), b"An M51 change.\n");
}

/// Stage the task's own code edit, so the commit is non-empty.
fn stage_code(corpus: &Corpus) {
    fs::write(corpus.repo().join("feature.rs"), "pub fn work() {}\n").expect("write task edit");
    git(&corpus.repo(), &["add", "feature.rs"]);
}

/// Seed the fixture pack the `owner-artifact-unstaged` row rides, and list it in the
/// in-repo project layer so it UNIONs with the embedded packs (the flow-18 mechanism).
///
/// The doctype is **transient** — no `location:`, no `placement:` — and that is the row's
/// whole construction, not a shortcut: the sweep emits a `file-state.staged-copy`
/// advisory for every *persisted* staged instance, so a persisted owner-artifact carrier
/// would always fire a second, non-blocking finding beside the gate — and the committing
/// door, which refuses with the gate's own findings, would then be compared against a
/// validate set of two. The gate itself reads every staged instance regardless of sink
/// (`engine::validate::owner_artifacts_gate`), so transience changes what *else* fires and
/// nothing about the member under test.
fn seed_owner_artifact_pack(corpus: &Corpus) {
    let pack = corpus.pack();
    for sub in ["schemas", "workflows", "steps", "config"] {
        fs::create_dir_all(pack.join(sub)).expect("mk fixture pack subdir");
    }
    fs::write(
        pack.join("schemas").join("audit-note.yaml"),
        "type: audit-note\n\
         id-from: title\n\
         description: A fixture audit note carrying an owner-artifact owned location.\n\
         usage: the equal-set fence's owner-artifact row.\n\
         sections:\n\
         \x20 - id: meta\n\
         \x20   header: true\n\
         \x20   fields:\n\
         \x20     - { id: owner-artifact, type: owned-location }\n\
         \x20 - id: body\n\
         \x20   slot: {}\n",
    )
    .expect("seed the audit-note schema");
    fs::write(
        pack.join("workflows").join("note.yaml"),
        "---\n\
         when: record an audit note\n\
         description: A fixture workflow that creates an audit-note.\n\
         usage: proving the equal-set fence's owner-artifact row.\n\
         creates-task: true\n\
         allows-create: [{type: audit-note, as: note}]\n\
         ---\n\
         {{ include: step:note-it }}\n",
    )
    .expect("seed the note workflow");
    fs::write(
        pack.join("steps").join("note-it.yaml"),
        "Record the audit note for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed the note step");
    fs::write(pack.join("config").join("commands.yaml"), "commands: []\n")
        .expect("seed the empty catalog");
    // A listed pack that ships steps owes the four ambush-class statements since M51
    // Increment 8 T2 — the stated-at fence's structural tier keys on step-shipping
    // constituents and checks each in isolation.
    crate::support::seed_ambush_class_declarer(&pack);
    fs::write(
        corpus.repo().join(".jigc/config/packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("list the fixture pack");
}

// --- the rows -----------------------------------------------------------------------

/// A corpus in which `single-task` has authored a persisted `adr` — the engine sweep's
/// `file-state.staged-copy` advisory over the task's staged copy of a committed-store
/// doctype. The changelog gate this workflow grants also goes unused, and that second
/// advisory is welcome: neither is blocking, so no door short-circuits and the three sets
/// stay comparable in full.
fn corpus_content_findings(corpus: &Corpus) -> String {
    ok(
        corpus,
        &["start", "--workflow", "single-task", "record the decision"],
        "jigc start --workflow single-task",
    );
    let task = the_open_task(corpus);
    let addr = ok(
        corpus,
        &["doc", "create", "adr", "--title", "Use the thing"],
        "doc create adr",
    );
    for section in ["context", "decision", "consequences"] {
        set_slot(corpus, &format!("{addr}#{section}"), b"Prose.\n");
    }
    stage_code(corpus);
    fill_commit(corpus, &task, "feat");
    task
}

/// A corpus with two foreign paths staged **before** the task existed — the carryover
/// gate's blocking `finalize.carried-staged`, one per carried path. `quick-fix` is the
/// workflow because it grants no create-gate at all, so the corpus fires this member and
/// nothing else.
fn corpus_carryover(corpus: &Corpus) -> String {
    fs::write(
        corpus.repo().join("foreign-a.txt"),
        "not this task's work\n",
    )
    .expect("write a");
    fs::write(corpus.repo().join("foreign-b.txt"), "also not\n").expect("write b");
    git(&corpus.repo(), &["add", "foreign-a.txt", "foreign-b.txt"]);
    ok(
        corpus,
        &["start", "--workflow", "quick-fix", "fix the thing"],
        "jigc start --workflow quick-fix",
    );
    let task = the_open_task(corpus);
    stage_code(corpus);
    fill_commit(corpus, &task, "fix");
    task
}

/// A corpus whose staged `audit-note` records an `owned-location` naming no file — the
/// #5 owner-artifact gate's `names-no-file` cause, one of the six the preview reaches.
fn corpus_owner_artifact(corpus: &Corpus) -> String {
    seed_owner_artifact_pack(corpus);
    ok(
        corpus,
        &["start", "--workflow", "note", "note the audit"],
        "jigc start --workflow note",
    );
    let task = the_open_task(corpus);
    let addr = ok(
        corpus,
        &["doc", "create", "audit-note", "--title", "M16 audit"],
        "doc create audit-note",
    );
    ok(
        corpus,
        &[
            "doc",
            "set-field",
            &format!("{addr}#meta/owner-artifact"),
            "--value",
            "completions/artifacts/M16/missing.md",
        ],
        "set-field owner-artifact",
    );
    set_slot(
        corpus,
        &format!("{addr}#body"),
        b"The audit landed green.\n",
    );
    stage_code(corpus);
    fill_commit(corpus, &task, "chore");
    task
}

/// A corpus whose `single-task` granted the `changelog` create-gate and never used it —
/// the finalize-scope `changelog-recording.gate-granted-unused` advisory, and nothing
/// else (no managed doc is authored, so no staged copy).
fn corpus_changelog_gate(corpus: &Corpus) -> String {
    ok(
        corpus,
        &["start", "--workflow", "single-task", "fix the thing"],
        "jigc start --workflow single-task",
    );
    let task = the_open_task(corpus);
    stage_code(corpus);
    fill_commit(corpus, &task, "feat");
    task
}

/// One row of the previewed axis: a [`gate_coverage`] member id, the code its corpus must
/// fire, and the corpus builder.
struct Row {
    /// The [`gate_coverage::GateCoverage::id`] this row covers.
    member: &'static str,
    /// The finding code the built corpus must make fire at every door — the proof the
    /// state reaches the member rather than merely being clean at three doors.
    code: &'static str,
    /// Builds the corpus and returns the open task's id.
    build: fn(&Corpus) -> String,
}

/// Every **sweep-invoked** member of [`Tier::Previewed`], one row each. The ⇔ against the
/// registry is [`every_previewed_member_carries_a_row`].
const ROWS: &[Row] = &[
    Row {
        member: "content-findings",
        code: "file-state.staged-copy",
        build: corpus_content_findings,
    },
    Row {
        member: "carryover",
        code: "finalize.carried-staged",
        build: corpus_carryover,
    },
    Row {
        member: "owner-artifact-unstaged",
        code: "owner-artifact.present",
        build: corpus_owner_artifact,
    },
    Row {
        member: "changelog-gate",
        code: "changelog-recording.gate-granted-unused",
        build: corpus_changelog_gate,
    },
];

// --- the fence ----------------------------------------------------------------------

/// The `findings[].code` multiset an envelope carries, sorted — a *multiset*, because the
/// carryover member emits one finding per carried path and a set would hide a door that
/// dropped one of two.
fn emitted_codes(out: &std::process::Output, door: &str) -> Vec<String> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "`{door}` must emit a JSON envelope on stdout ({e}); stdout:\n{stdout}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        )
    });
    let findings = envelope["findings"].as_array().unwrap_or_else(|| {
        panic!(
            "`{door}`'s envelope must carry a `findings` array — the key EC-20 adds; got:\n\
             {envelope:#}"
        )
    });
    let mut codes: Vec<String> = findings
        .iter()
        .map(|f| {
            f["code"]
                .as_str()
                .unwrap_or_else(|| panic!("`{door}`: every finding carries a `code`; got {f}"))
                .to_string()
        })
        .collect();
    codes.sort();
    codes
}

/// The three doors' emitted code multisets, in door order, over one built corpus.
///
/// Run in sequence against the **same** repo: `task validate` and `task finalize
/// --dry-run` are both pure readers (they commit nothing and touch nothing), so the
/// committing door last sees exactly the state the two forecasts described.
fn three_doors(corpus: &Corpus, task: &str) -> [Vec<String>; 3] {
    let validate = jigc(
        corpus,
        &["--format", "json", "task", "validate", task],
        None,
    );
    let dry_run = jigc(
        corpus,
        &["--format", "json", "task", "finalize", task, "--dry-run"],
        None,
    );
    let committing = jigc(
        corpus,
        &["--format", "json", "task", "finalize", task],
        None,
    );
    [
        emitted_codes(&validate, "jigc task validate"),
        emitted_codes(&dry_run, "jigc task finalize --dry-run"),
        emitted_codes(&committing, "jigc task finalize"),
    ]
}

/// **The axis is the registry.** Every sweep-invoked [`Tier::Previewed`] member has
/// exactly one row, and every row names one — so a member joining the tier reddens here
/// rather than slipping through a fence that iterates a hand-written list.
///
/// The filter is the row's own stated [`gate_coverage::Invocation`], never a member id:
/// a `SeparatelyAtDoor` member refuses before the sweep and its envelope rides stderr
/// (module header), and its equality is pinned by `validate_previews_posture.rs`.
#[test]
fn every_previewed_member_carries_a_row() {
    let mut members: Vec<&str> = gate_coverage::members(Tier::Previewed)
        .filter(|row| row.door.invocation() == Some(gate_coverage::Invocation::PreviewGates))
        .map(|row| row.id)
        .collect();
    let mut rows: Vec<&str> = ROWS.iter().map(|row| row.member).collect();
    members.sort_unstable();
    rows.sort_unstable();
    assert_eq!(
        rows, members,
        "every `Tier::Previewed` member needs a row that drives it, and every row needs a \
         member — the fence is over the registry, not over a list",
    );
}

/// **The fence.** For every previewed member, over a corpus built to make that member —
/// and only that member — fire: `task validate`, `task finalize --dry-run` and the
/// committing `task finalize` emit **one** code multiset, and it contains the member's
/// code.
#[test]
fn the_three_doors_emit_one_code_set_for_every_previewed_member() {
    for row in ROWS {
        let corpus = Corpus::new(row.member);
        let task = (row.build)(&corpus);
        let [validate, dry_run, committing] = three_doors(&corpus, &task);

        assert!(
            validate.iter().any(|code| code == row.code),
            "[{}] the corpus must make the member fire — `jigc task validate` emitted \
             {validate:?}, which does not contain `{}`",
            row.member,
            row.code,
        );
        assert_eq!(
            dry_run, validate,
            "[{}] `task finalize --dry-run` must emit what `task validate` emits — the \
             forecast and the preview are one surface",
            row.member,
        );
        assert_eq!(
            committing, validate,
            "[{}] the committing door must emit what `task validate` previewed — landed \
             or blocked",
            row.member,
        );
    }
}

/// **The clean arm.** A task with nothing to report emits the *empty* set at all three
/// doors — so the fence's equality is not satisfied by three doors that all say nothing
/// interesting, and the new `findings` key is present-and-empty rather than absent.
#[test]
fn a_clean_task_emits_the_empty_set_at_all_three_doors() {
    let corpus = Corpus::new("clean");
    ok(
        &corpus,
        &["start", "--workflow", "quick-fix", "fix the thing"],
        "jigc start --workflow quick-fix",
    );
    let task = the_open_task(&corpus);
    stage_code(&corpus);
    fill_commit(&corpus, &task, "fix");

    let [validate, dry_run, committing] = three_doors(&corpus, &task);
    let empty: Vec<String> = Vec::new();
    assert_eq!(validate, empty, "the clean corpus validates clean");
    assert_eq!(
        dry_run, empty,
        "`--dry-run` carries the key with an empty array — absent and empty are different \
         answers to a driver",
    );
    assert_eq!(committing, empty, "the landed commit reports nothing");
}

// --- the text arm (M52 Increment 10 / T3, D-1) ---------------------------------------

/// **The text arm names what the envelope carries.** A granted-and-unused changelog gate,
/// forecast twice: `--format json` reports `changelog-recording.gate-granted-unused`, and
/// the default agent-text arm must name it too.
///
/// Driven at HEAD before the fix, on this exact corpus: the text arm printed the manifest
/// title, the `would commit — …` line and `added feature.rs`, and **no findings line at
/// all**, while the envelope carried the advisory. The assertion is over the envelope's
/// own code set rather than the one code this corpus fires, so the fence is *the text arm
/// renders what its sibling carries*, not *it renders this string*.
#[test]
fn the_dry_run_text_arm_names_every_code_its_envelope_carries() {
    let corpus = Corpus::new("dry-run-text");
    let task = corpus_changelog_gate(&corpus);

    let envelope = jigc(
        &corpus,
        &["--format", "json", "task", "finalize", &task, "--dry-run"],
        None,
    );
    let codes = emitted_codes(&envelope, "jigc task finalize --dry-run");
    assert!(
        codes.contains(&"changelog-recording.gate-granted-unused".to_string()),
        "the corpus must make the advisory fire; the envelope carried {codes:?}",
    );

    let text = jigc(&corpus, &["task", "finalize", &task, "--dry-run"], None);
    assert!(
        text.status.success(),
        "`task finalize --dry-run` exits 0; stderr:\n{}",
        String::from_utf8_lossy(&text.stderr),
    );
    let rendered = String::from_utf8_lossy(&text.stdout).into_owned();
    for code in &codes {
        assert!(
            rendered.contains(code.as_str()),
            "the forecast's text arm must name `{code}` — the envelope carries it and the \
             two arms are one surface; text arm was:\n{rendered}",
        );
    }
}

/// **Every `DocAck` arm renders a carried finding.** The axis is the code-side registry
/// [`render::DOC_ACK_ARMS`]: one sample per variant, its arm name read back through
/// [`render::doc_ack_arm`]'s exhaustive match, and the two sets compared — so a tenth
/// variant cannot reach the text arm without a row here.
///
/// **Declared bound, and it is why the sweep is over the renderer.** Eight of the nine
/// arms have no known non-empty producer today (`baseline-surfaces.md` §2.7: only
/// `DocAck::Renamed` reaches one, via `commit-recording.stale-title`), so driving nine
/// cells through the binary is not available. The class D-1 names is the *renderer* —
/// `render::doc_ack`'s agent/human arm built one line per variant and never read
/// `findings` — and this asserts over exactly that.
#[test]
fn every_doc_ack_arm_renders_a_carried_finding() {
    let carried = || {
        Findings::from(vec![Finding::graded(
            Severity::Advisory,
            "commit-recording.stale-title",
            "the staged commit summary names the pre-rename title",
            Some(Location::addressed("adr:cache-policy", 1, 1)),
            Some(Route::human(
                "re-author the commit summary through `jigc doc set-slot`",
            )),
        )])
    };
    let target = || AckTarget {
        doctype: "adr".to_owned(),
        slug: "cache-policy".to_owned(),
        section: Some("decision".to_owned()),
        item: None,
        leaf: None,
    };
    let head = || AckTarget {
        doctype: "adr".to_owned(),
        slug: "cache-policy".to_owned(),
        section: None,
        item: None,
        leaf: None,
    };
    let address = "adr:cache-policy#decision".to_owned();

    let acks = vec![
        DocAck::Field {
            address: address.clone(),
            target: target(),
            value: serde_json::Value::String("accepted".to_owned()),
            findings: carried(),
            copied_in: false,
        },
        DocAck::UnsetField {
            address: address.clone(),
            target: target(),
            already_absent: false,
            findings: carried(),
            copied_in: false,
        },
        DocAck::Slot {
            address: address.clone(),
            target: target(),
            chars: 42,
            findings: carried(),
            copied_in: false,
        },
        DocAck::RemovedItem {
            address: address.clone(),
            target: target(),
            findings: carried(),
            copied_in: false,
        },
        DocAck::Renamed {
            address: "adr:cache-policy".to_owned(),
            target: head(),
            title: "Cache policy".to_owned(),
            from: "adr:cache-strategy".to_owned(),
            reslugged: true,
            committed_identity: false,
            findings: carried(),
            copied_in: false,
        },
        DocAck::RetitledItem {
            address: address.clone(),
            target: target(),
            title: "Limits by client".to_owned(),
            findings: carried(),
            copied_in: false,
        },
        DocAck::Created {
            address: "adr:cache-policy".to_owned(),
            target: head(),
            existed: false,
            findings: carried(),
        },
        DocAck::AddedItem {
            address: address.clone(),
            target: target(),
            findings: carried(),
            copied_in: false,
        },
        DocAck::Authored {
            address: "adr:cache-policy".to_owned(),
            target: head(),
            findings: carried(),
        },
    ];

    let mut covered: Vec<&str> = acks.iter().map(render::doc_ack_arm).collect();
    let mut declared: Vec<&str> = render::DOC_ACK_ARMS.to_vec();
    covered.sort_unstable();
    declared.sort_unstable();
    assert_eq!(
        covered, declared,
        "every `DocAck` arm needs a sample here, and every sample an arm — the axis is the \
         registry, not a hand list",
    );

    for ack in &acks {
        let arm = render::doc_ack_arm(ack);
        for format in [Format::Agent, Format::Human] {
            let rendered = render::doc_ack(format, ack);
            assert!(
                rendered.contains("commit-recording.stale-title"),
                "[{arm}/{format:?}] the text ack must name the finding it carries into JSON; \
                 got:\n{rendered}",
            );
            assert!(
                rendered.contains("the staged commit summary names the pre-rename title"),
                "[{arm}/{format:?}] the finding's message rides the text arm too; got:\n{rendered}",
            );
            assert!(
                rendered.contains("re-author the commit summary"),
                "[{arm}/{format:?}] a carried route reaches the text arm — the route floor \
                 binds every surface that renders a finding; got:\n{rendered}",
            );
        }
    }
}

/// **The clean ack is unchanged.** A `DocAck` carrying no finding renders exactly the one
/// line it always did — the addition is the findings block, never a header or a blank line
/// on the ordinary write.
#[test]
fn a_finding_free_doc_ack_is_still_one_line() {
    let ack = DocAck::Slot {
        address: "adr:cache-policy#decision".to_owned(),
        target: AckTarget {
            doctype: "adr".to_owned(),
            slug: "cache-policy".to_owned(),
            section: Some("decision".to_owned()),
            item: None,
            leaf: None,
        },
        chars: 42,
        findings: Findings::default(),
        copied_in: false,
    };
    assert_eq!(
        render::doc_ack(Format::Agent, &ack),
        "set slot adr:cache-policy#decision (42 chars)",
    );
}

// --- the out-of-set members the arg help has to name (M53, the rc.20 review `(2, A2-1)`) ---

/// **The forecast refuses on gates `jigc task validate` does not report — and the flag's
/// own help says which** (M53, the rc.20 per-axis review `(2, A2-1)`).
///
/// The `--dry-run` arg help shipped two false sentences at once: *"It refuses on **three
/// gates**: this task's validation findings, the empty-commit guard, and the carryover
/// gate"* and, three lines later, *"Its `findings` are the set `jigc task validate <id>`
/// reports"*. The second contradicts the first — the empty-commit guard is named as a gate
/// and is no member of `validate`'s set — and driven, **two** of the gates the forecast
/// refuses on sit outside that set.
///
/// **The axis is the whole out-of-set part of the refusal set, not either cell.** A fix
/// naming only the amend arm's `finalize.base-mismatch` (which is where the review first
/// found it) would ship the same incomplete sweep one member over: the ordinary arm's
/// `finalize.empty-commit` was already outside the set before `jigc task amend` existed.
/// Each cell drives the pair — `task validate` clean of the code, `--dry-run` refusing with
/// a code that preview will never print — and then asserts the **shipped help** names that
/// code, so the sentence is fenced against the binary rather than proof-read once.
///
/// **[Struck 2026-09-27 (the last pre-1.0.0 batch's review, MEDIUM 2).** This paragraph read
/// *"Why these two and no third: `cli::gate_coverage` is the membership authority … Every
/// other pre-transaction gate is either inside `Tier::Previewed` … or is a **door**-level
/// refusal both surfaces share (the repository posture, **the sub-task boundary**)."* Its
/// load-bearing clause is false, driven on `1.0.0-rc.20`: from the main checkout,
/// `jigc task validate <sub>` never prints `finalize.milestone-sub-task` and
/// `--dry-run` refuses with it at exit 3, so the sub-task boundary is not shared — it is a
/// **third** out-of-set member, which is the same *"incomplete sweep one member over"* the
/// fix set out to avoid, committed inside its own completeness leg. And a prose leg is what
/// let it: a paragraph reasoning about *"every other pre-transaction gate"* was never
/// measured against the producers the path actually runs through.**]
///
/// **So the subject is derived.** [`DRY_RUN_REFUSALS`] enumerates every refusal producer a
/// `--dry-run` passes through, in path order, each disposed exactly once — previewed, shared,
/// **out of set**, or not a gate with the reason — and three fences hold it to the source and
/// to the binary: the rows of each [`Span`] are **counted against that span's own text**
/// ([`the_refusal_table_counts_the_producers_the_source_carries`]), every `Previewed` payload
/// must name a real `Tier::Previewed` member, and every `OutOfSet` code must be **named in the
/// shipped help** and **driven** — here, or at a cited suite that drives it
/// ([`the_forecast_names_every_gate_it_refuses_on_that_the_preview_does_not_report`]).
///
/// **What it measured: six out-of-set codes, where the shipped sentence named two.** Beside
/// `finalize.base-mismatch` and `finalize.empty-commit` it found the review's
/// `finalize.milestone-sub-task`, plus `finalize.nothing-staged` (the empty-commit guard's CLI
/// recolor — a distinct code, pointing at `git add`, and the one an agent actually greps),
/// `finalize.promote-clobber` and `finalize.migration-no-replacement` — the planner's two
/// collision gates, both raised inside `plan_finalize`, which the `--dry-run` branch sits
/// *after*. All four new members were driven to a live repro before the help was touched.
///
/// The derivation also **shrank** the set in one place, which is the half that says it is a
/// derivation: the two amend gates the shipped sentence names read like out-of-set members and
/// are not — `preview_gates` computes both when the amend marker is present, so they are the
/// `carryover` member answered on the amend arm, and their rows say so.
#[test]
fn the_forecast_names_every_gate_it_refuses_on_that_the_preview_does_not_report() {
    let help = finalize_help();
    assert!(
        !help.contains("It *refuses* on three gates")
            && !help.contains("It refuses on three gates"),
        "the count the binary contradicts must be gone; help:\n{help}",
    );

    let mut driven = 0usize;
    for row in DRY_RUN_REFUSALS {
        let Disposition::OutOfSet { code, cell } = row.disposition else {
            continue;
        };
        assert!(
            help.contains(code),
            "the `--dry-run` help must name `{code}` — a gate the flag refuses on and the \
             preview it claims parity with never reports ({}); help:\n{help}",
            row.producer,
        );
        if let Cell::Driven(build) = cell {
            let corpus = Corpus::new(code.rsplit('.').next().expect("a code stem"));
            let task = build(&corpus);
            out_of_set_cell(&corpus, &task, code, &help);
            driven += 1;
        }
    }
    assert!(
        driven >= 4,
        "the cells this suite drives itself must not quietly empty out; drove {driven}",
    );
}

/// `jigc task finalize --help` as the binary ships it.
fn finalize_help() -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["task", "finalize", "--help"])
        .output()
        .expect("run the jigc binary");
    String::from_utf8_lossy(&out.stdout).to_string()
}

/// One out-of-set cell: `jigc task validate` does not report `code`, `--dry-run` refuses
/// with it, and the flag's shipped help names it.
///
/// **The preview assertion is `does not report the code`, not `reports nothing`** — widened at
/// the MEDIUM-2 fix, because two of the derived cells are legitimately noisy: the clobber
/// corpus's untracked squatter fires `schema-conformance.unadopted-instance` and
/// `file-state.staged-copy` at exit 0. A preview that is clean *of this code* is the property
/// the row claims; a preview that is empty was the two original cells' accident of shape, and
/// requiring it would have made the wider axis unreachable rather than false.
fn out_of_set_cell(corpus: &Corpus, task: &str, code: &str, help: &str) {
    let validate = jigc(
        corpus,
        &["--format", "json", "task", "validate", task],
        None,
    );
    assert_eq!(
        validate.status.code(),
        Some(0),
        "the preview must not block here, or this cell proves nothing about the difference; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&validate.stdout),
        String::from_utf8_lossy(&validate.stderr),
    );
    assert!(
        !emitted_codes(&validate, "jigc task validate").contains(&code.to_string()),
        "`{code}` is outside the previewed set — `task validate` must not report it here",
    );

    let forecast = jigc(
        corpus,
        &["--format", "json", "task", "finalize", task, "--dry-run"],
        None,
    );
    assert!(
        !forecast.status.success(),
        "`--dry-run` must refuse on `{code}`; stdout:\n{}",
        String::from_utf8_lossy(&forecast.stdout),
    );
    assert!(
        emitted_codes(&forecast, "jigc task finalize --dry-run").contains(&code.to_string()),
        "the forecast must refuse with `{code}`; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&forecast.stdout),
        String::from_utf8_lossy(&forecast.stderr),
    );

    assert!(
        help.contains(code),
        "the `--dry-run` help must name `{code}` — a gate the flag refuses on and the \
         preview it claims parity with never reports; help:\n{help}",
    );
}

// --- the derived subject: every refusal producer a `--dry-run` passes through ------------

/// The four source spans a `jigc task finalize --dry-run` can refuse from, in path order —
/// and the subject each [`Refusal`] row is counted against.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Span {
    /// `cli::task::run_finalize` — the door funnel, ahead of the transaction entirely.
    Funnel,
    /// `cli::task::TaskArea::finalize`, from its head to the `if dry_run {` branch.
    Finalize,
    /// `engine::finalize::plan_finalize`, whose every error reaches the caller through
    /// [`Span::Finalize`]'s planner arm.
    Planner,
    /// The `if dry_run {` branch itself.
    Forecast,
}

impl Span {
    /// `(workspace-relative file, from, to, needles)`. The occurrences of `needles` in the
    /// span's text must total the rows attributed to it — the completeness leg, measured
    /// rather than asserted in prose.
    fn source(
        self,
    ) -> (
        &'static str,
        &'static str,
        &'static str,
        &'static [&'static str],
    ) {
        match self {
            Span::Funnel => (
                "crates/cli/src/task.rs",
                "fn run_finalize(",
                "\n}\n",
                &["return "],
            ),
            Span::Finalize => (
                "crates/cli/src/task.rs",
                "    fn finalize(\n",
                "        if dry_run {",
                &["self.blocked("],
            ),
            Span::Planner => (
                "crates/engine/src/finalize.rs",
                "pub fn plan_finalize(",
                "    Ok(FinalizePlan::new(",
                &["return Err(", "?;"],
            ),
            Span::Forecast => (
                "crates/cli/src/task.rs",
                "        if dry_run {",
                "            return Ok(Outcome::with_findings(",
                &["return self.blocked("],
            ),
        }
    }
}

/// How one producer is disposed. Every row takes exactly one, which is what makes the
/// enumeration a *disposition* rather than a listing.
#[derive(Clone, Copy)]
enum Disposition {
    /// The code it raises belongs to a [`Tier::Previewed`] member — `jigc task validate`
    /// reports it, so the help owes it nothing beyond the parity sentence. The payload is the
    /// `gate_coverage` member id, asked of the registry by
    /// [`every_previewed_disposition_names_a_real_member`].
    Previewed(&'static str),
    /// Both surfaces raise it, with the same identity, so there is no difference to name.
    Shared(&'static str),
    /// Outside the previewed set: the shipped help must name `code`, and `cell` says where
    /// the pair is driven.
    OutOfSet { code: &'static str, cell: Cell },
    /// Not a gate — an I/O fault channel — carrying the reason it is not one. The help's
    /// sentence is about *gates*, so a fault channel is out of its scope by construction.
    NotAGate(&'static str),
}

/// Where an [`Disposition::OutOfSet`] row's `validate`-clean / `--dry-run`-refuses pair is
/// driven: in this suite, or at the suite cited (the `pinned-by:` discipline,
/// [pinning.md](../../../implementation/pinning.md) §3 — verified by reading what the cited
/// test asserts, never by a symbol parser).
#[derive(Clone, Copy)]
enum Cell {
    Driven(fn(&Corpus) -> String),
    PinnedBy(&'static str),
}

/// One refusal producer on the `--dry-run` path.
struct Refusal {
    /// The span it lives in — the count fence's subject.
    span: Span,
    /// The producer as the source spells it, in span order.
    producer: &'static str,
    disposition: Disposition,
}

/// **Every refusal producer a `jigc task finalize --dry-run` passes through**, in path order.
///
/// The rows of each [`Span`] are counted against that span's own text, so a producer added to
/// any of the four reddens this suite until someone disposes it — which is the leg the struck
/// prose paragraph did not have.
const DRY_RUN_REFUSALS: &[Refusal] = &[
    // ── the door funnel (`run_finalize`) ───────────────────────────────────────────────
    Refusal {
        span: Span::Funnel,
        producer: "`TaskArea::resolve`'s unknown / malformed work-unit id",
        disposition: Disposition::Shared(
            "`jigc task validate <unknown>` refuses with the same `finalize.no-task` at the \
             same `task:<id>` key — one of M51 Increment 6's 25 cells, so the two surfaces \
             agree and there is no difference for the help to name",
        ),
    },
    Refusal {
        span: Span::Funnel,
        producer: "`cli::cli::finalize_posture_refusal`",
        disposition: Disposition::Previewed("posture"),
    },
    // ── `TaskArea::finalize`, head → the `if dry_run` branch ───────────────────────────
    Refusal {
        span: Span::Finalize,
        producer: "`engine::milestone::sub_task_finalize_finding`",
        disposition: Disposition::OutOfSet {
            code: "finalize.milestone-sub-task",
            cell: Cell::Driven(corpus_sub_task),
        },
    },
    Refusal {
        span: Span::Finalize,
        producer: "`engine::finalize::decide_base_repin`'s overlap block",
        disposition: Disposition::OutOfSet {
            code: "finalize.base-mismatch",
            cell: Cell::PinnedBy(
                "the amend-pin row below drives this CODE, and the help's obligation is per \
                 code: all three producers of `finalize.base-mismatch` owe the same one \
                 sentence, so one driven cell discharges it and this row says which",
            ),
        },
    },
    // **The next two are PREVIEWED, and checking rather than assuming it is what kept them out
    // of the out-of-set list.** Both are the `carryover` member answered on the amend arm —
    // `TaskArea::preview_gates` extends its own set with `amend_index_findings` and
    // `amend_staged_doc_findings` when the amend marker is present (`task.rs`, the
    // `if amending` block), so `jigc task validate` reports them at the finalize exit and the
    // help's amend paragraph is a *spelling* note about a previewed member, not an out-of-set
    // claim. Read the other way round, the derivation would have grown two rows the binary
    // contradicts — the mirror image of the mistake it exists to correct.
    Refusal {
        span: Span::Finalize,
        producer: "`TaskArea::amend_index_findings`",
        disposition: Disposition::Previewed("carryover"),
    },
    Refusal {
        span: Span::Finalize,
        producer: "`TaskArea::amend_staged_doc_findings`",
        disposition: Disposition::Previewed("carryover"),
    },
    Refusal {
        span: Span::Finalize,
        producer: "`engine::finalize::amend_base_mismatch_finding` (the amend arm's own pin)",
        disposition: Disposition::OutOfSet {
            code: "finalize.base-mismatch",
            cell: Cell::Driven(corpus_amend_moved_head),
        },
    },
    Refusal {
        span: Span::Finalize,
        producer: "`cli::task::nothing_staged_finding` (the empty-commit block, recolored \
                   when the tree is dirty and the index empty)",
        disposition: Disposition::OutOfSet {
            code: "finalize.nothing-staged",
            cell: Cell::Driven(corpus_nothing_staged),
        },
    },
    Refusal {
        span: Span::Finalize,
        producer: "`engine::finalize::plan_finalize`'s findings, passed through",
        disposition: Disposition::Shared(
            "a carrier, not a producer of its own: every code it delivers is disposed at its \
             own `Span::Planner` row below",
        ),
    },
    // ── `plan_finalize` ───────────────────────────────────────────────────────────────
    Refusal {
        span: Span::Planner,
        producer: "`task_missing_finding`",
        disposition: Disposition::Shared(
            "the same `finalize.no-task` the funnel's first row disposes — and unreachable \
             from this door, which resolved the area before the planner ran",
        ),
    },
    Refusal {
        span: Span::Planner,
        producer: "`base_mismatch_finding` (the preflight's pin == HEAD leg)",
        disposition: Disposition::OutOfSet {
            code: "finalize.base-mismatch",
            cell: Cell::PinnedBy("the ordinary arm's cell above, on the same code"),
        },
    },
    Refusal {
        span: Span::Planner,
        producer: "the validation report's blocking findings",
        disposition: Disposition::Previewed("content-findings"),
    },
    Refusal {
        span: Span::Planner,
        producer: "`empty_commit_finding`",
        disposition: Disposition::OutOfSet {
            code: "finalize.empty-commit",
            cell: Cell::Driven(corpus_empty_commit),
        },
    },
    Refusal {
        span: Span::Planner,
        producer: "`render_io_finding` over the staged commit doc",
        disposition: Disposition::NotAGate(
            "`finalize.render-io` is an I/O fault channel — it reports that the staged commit \
             doc could not be read, never a verdict about the task's state; `gate_coverage`'s \
             own module header already declares it free prose no enumeration owes",
        ),
    },
    Refusal {
        span: Span::Planner,
        producer: "`write::instance_from_source` over the staged commit doc",
        disposition: Disposition::Previewed("content-findings"),
    },
    Refusal {
        span: Span::Planner,
        producer: "`plan_promotions`",
        disposition: Disposition::NotAGate(
            "`finalize.promote-io` — the same fault channel one phase on: a staged doc's bytes \
             could not be read",
        ),
    },
    Refusal {
        span: Span::Planner,
        producer: "`plan_clobber_guard`",
        disposition: Disposition::OutOfSet {
            code: "finalize.promote-clobber",
            cell: Cell::Driven(corpus_promote_clobber),
        },
    },
    Refusal {
        span: Span::Planner,
        producer: "`plan_retirements`",
        disposition: Disposition::OutOfSet {
            code: "finalize.migration-no-replacement",
            cell: Cell::Driven(corpus_migration_no_replacement),
        },
    },
    // ── the `if dry_run` branch ───────────────────────────────────────────────────────
    Refusal {
        span: Span::Forecast,
        producer: "the carryover decision, refused unless `--carry-staged` is declared",
        disposition: Disposition::Previewed("carryover"),
    },
];

/// **The completeness leg, measured against the source.** For each span, the occurrences of
/// its refusal needles in its own text equal the rows attributed to it — so a producer added
/// to any of the four reddens here until it is disposed above.
///
/// **Declared bound, and it is the reason this is a count rather than a proof.** A needle
/// counts *statements that can leave the span with a refusal*, not codes: `Span::Planner`'s
/// nine are four `return Err(` and five `?;` propagations, and two of those propagate a fault
/// channel rather than a gate — which the rows say. What the count buys is that the table
/// cannot silently fall behind the path; what it does not buy is that a *code* added inside an
/// existing producer is noticed, and that is what the help fence and the driven cells are for.
#[test]
fn the_refusal_table_counts_the_producers_the_source_carries() {
    for span in [Span::Funnel, Span::Finalize, Span::Planner, Span::Forecast] {
        let (file, from, to, needles) = span.source();
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join(file);
        let body = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {file}: {err}"));
        let start = body
            .find(from)
            .unwrap_or_else(|| panic!("{file} must carry the span opener {from:?}"));
        let end = body[start..]
            .find(to)
            .map(|at| start + at)
            .unwrap_or_else(|| panic!("{file} must carry the span closer {to:?} after {from:?}"));
        let text = &body[start..end];
        let counted: usize = needles
            .iter()
            .map(|needle| text.matches(needle).count())
            .sum();
        let rows = DRY_RUN_REFUSALS
            .iter()
            .filter(|row| row.span == span)
            .count();
        assert_eq!(
            counted, rows,
            "{span:?} carries {counted} refusal-producing statement(s) ({needles:?}) and \
             `DRY_RUN_REFUSALS` disposes {rows} — every producer on the `--dry-run` path owes \
             a row, which is the leg the struck prose paragraph did not have",
        );
    }
}

/// Every `Previewed` disposition names a member the registry actually carries at
/// [`Tier::Previewed`] — so "the preview reports it" is asked of `cli::gate_coverage`, the
/// membership authority, rather than asserted here.
#[test]
fn every_previewed_disposition_names_a_real_member() {
    let members: Vec<&str> = gate_coverage::members(Tier::Previewed)
        .map(|row| row.id)
        .collect();
    let mut cited = 0usize;
    for row in DRY_RUN_REFUSALS {
        if let Disposition::Previewed(id) = row.disposition {
            assert!(
                members.contains(&id),
                "`{id}` is cited as a previewed member by {} and `Tier::Previewed` has no \
                 such row; it carries {members:?}",
                row.producer,
            );
            cited += 1;
        }
    }
    assert!(
        cited >= 3,
        "the previewed dispositions must not empty out; found {cited}"
    );
}

/// **Every non-previewed disposition carries its stated reason, and the reason is read.** A
/// `Shared`, `NotAGate` or `PinnedBy` row is a *judgment* — nothing mechanical decides that a
/// fault channel is not a gate, or that another suite drives a code's pair — so the table's
/// discipline is that each such row states why, in the `UNSWEPT_PRODUCERS` shape: the payload is
/// an assertion subject here rather than a comment the compiler warns is unread.
#[test]
fn every_judged_disposition_states_its_reason() {
    for row in DRY_RUN_REFUSALS {
        let (kind, reason, floor) = match row.disposition {
            Disposition::Shared(reason) => ("Shared", reason, 60),
            Disposition::NotAGate(reason) => ("NotAGate", reason, 60),
            Disposition::OutOfSet {
                cell: Cell::PinnedBy(citation),
                ..
            } => ("PinnedBy", citation, 30),
            _ => continue,
        };
        assert!(
            reason.len() >= floor,
            "{} disposes {kind} on {} character(s) of reason — a judged row states why, at \
             least as fully as `UNSWEPT_PRODUCERS`' rows do; got:\n{reason}",
            row.producer,
            reason.len(),
        );
    }
}

// --- the corpora the out-of-set cells are driven over -----------------------------------

/// The ORDINARY arm's empty-commit guard. Pre-existing: it predates the amend arm entirely,
/// which is what makes this class older than the row that found it.
fn corpus_empty_commit(corpus: &Corpus) -> String {
    ok(
        corpus,
        &["start", "--workflow", "quick-fix", "empty commit probe"],
        "jigc start --workflow quick-fix",
    );
    let task = the_open_task(corpus);
    fill_commit(corpus, &task, "fix");
    task
}

/// The AMEND arm's base pin, over a HEAD that moved after the mint.
fn corpus_amend_moved_head(corpus: &Corpus) -> String {
    ok(corpus, &["task", "amend", "moved head"], "jigc task amend");
    let task = the_open_task(corpus);
    fill_commit(corpus, &task, "docs");
    ok(
        corpus,
        &["milestone", "create", "Move head wave"],
        "a record-only commit that moves HEAD under the amend pin",
    );
    task
}

/// **The empty-commit guard's recolor.** The working tree has changes and the index is empty,
/// so the CLI replaces the engine's clean-empty block with `finalize.nothing-staged` — a
/// distinct code, pointing at `git add` rather than at "produced no diff", and one the help's
/// `finalize.empty-commit` spelling does not answer for an agent grepping the refusal it got.
fn corpus_nothing_staged(corpus: &Corpus) -> String {
    ok(
        corpus,
        &["start", "--workflow", "quick-fix", "nothing staged probe"],
        "jigc start --workflow quick-fix",
    );
    let task = the_open_task(corpus);
    fill_commit(corpus, &task, "fix");
    // Dirty, unstaged: the tree has work and the index has none.
    fs::write(corpus.repo().join("unstaged.txt"), "never added\n").expect("write the dirty file");
    task
}

/// **The sub-task boundary** — the member the rc.20 fix's own completeness leg called shared.
/// Driven from the main checkout: the preview reports nothing, and the forecast refuses.
fn corpus_sub_task(corpus: &Corpus) -> String {
    ok(
        corpus,
        &["milestone", "create", "Boundary wave"],
        "jigc milestone create",
    );
    ok(
        corpus,
        &["milestone", "add-task", "boundary-wave", "Area one"],
        "jigc milestone add-task",
    );
    // The commit doc is authored so the PREVIEW is clean: an unauthored sub-task blocks on its
    // own conformance findings, and a cell whose preview blocks proves nothing about the
    // difference between the two surfaces. Reaching it takes the sub-task's **first re-entry**,
    // which is what provisions its `commit` doc — and that door is pinned to the milestone's
    // base, which the record commits have already left in the main checkout, so it is run from
    // the provisioned worktree. Authoring afterwards is done from the repository root, because
    // the `.jigc/` workbench a worktree binds to is the main checkout's.
    ok(
        corpus,
        &["milestone", "provision", "boundary-wave"],
        "jigc milestone provision",
    );
    jigc_in(
        corpus,
        &corpus
            .repo()
            .join(".jigc")
            .join("worktrees")
            .join("area-one"),
        &["workflow", "sub-task", "--task", "area-one"],
        "the sub-task's first re-entry",
    );
    fill_commit(corpus, "area-one", "feat");
    "area-one".to_owned()
}

/// **The planner's clobber guard.** A create-provenance staged doc whose canonical destination
/// already holds a file — here an untracked squatter, which is what keeps the cell reachable:
/// the guard asks the **worktree** (`repo_root.join(destination).is_file()`), so no commit is
/// needed and HEAD does not move. Planted through a commit instead, the base-pin overlap block
/// adjudicates the same state one phase earlier and this code never prints (driven).
fn corpus_promote_clobber(corpus: &Corpus) -> String {
    ok(
        corpus,
        &["start", "--workflow", "single-task", "record the decision"],
        "jigc start --workflow single-task",
    );
    let task = the_open_task(corpus);
    let addr = ok(
        corpus,
        &["doc", "create", "adr", "--title", "Fresh choice"],
        "doc create adr",
    );
    for section in ["context", "decision", "consequences"] {
        set_slot(corpus, &format!("{addr}#{section}"), b"Prose.\n");
    }
    stage_code(corpus);
    fill_commit(corpus, &task, "docs");
    let home = corpus.repo().join("docs").join("decisions");
    fs::create_dir_all(&home).expect("the adr home");
    fs::write(home.join("fresh-choice.md"), "a squatter\n").expect("plant the squatter");
    task
}

/// **The planner's retire-safety gate.** A migration task whose recorded foreign source has
/// nothing staged to replace it — the state a `jigc migrate` leaves before the agent authors
/// anything, which is exactly when an agent reaches for `--dry-run`.
fn corpus_migration_no_replacement(corpus: &Corpus) -> String {
    fs::write(
        corpus.repo().join("old-decision.md"),
        "# Old decision\n\nWe chose the thing, for the reasons below.\n",
    )
    .expect("write the foreign source");
    git(&corpus.repo(), &["add", "old-decision.md"]);
    git(&corpus.repo(), &["commit", "-q", "-m", "a foreign doc"]);
    ok(
        corpus,
        &["migrate", "old-decision.md", "--as", "adr"],
        "jigc migrate --as adr",
    );
    the_open_task(corpus)
}
