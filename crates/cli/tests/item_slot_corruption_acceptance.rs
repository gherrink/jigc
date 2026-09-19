//! **The item-slot corruption class, blocked through both write doors** — M45
//! Increment 2's acceptance ([`implementation/roadmap.md`] → Milestone 45,
//! Increment 2 *Proves*).
//!
//! The class is three commit-clean variants the RC-alpha3 verification trial
//! reproduced against `1.0.0-rc.8`, each reached by writing a heading at a
//! schema-reserved depth into an *item's* slot prose. Each was accepted at exit 0,
//! left no finding at `task validate`, and committed clean
//! ([`DECISIONS.md`] → 2026-07-23 M45 Increment 2 planning, *Verified bases*):
//!
//! 1. **the minted ghost item** — `### Ghost  {#ghost}` inside a `spec` criterion's
//!    `statement` minted a *second, real* repeatable item through jigc's own write
//!    verb; `doc show` reported `item-count: 2`;
//! 2. **the sibling-leaf reattribution** — `#### Proves` inside a `roadmap`
//!    milestone's `decomposition` landed bytes that fell outside every recorded
//!    slot span, invisible to `doc show`, and the **next correct write** to a
//!    sibling leaf re-rendered the item from the parse and destroyed them;
//! 3. **the hijacked downstream section** — `## Context` inside a `spec` criterion's
//!    `statement` produced a duplicate `## Context`, truncated the `criteria`
//!    section at it, and swallowed the items below.
//!
//! Every arm here drives the **built binary**, and drives **both** write doors —
//! `jigc doc set-slot` and the `jigc doc author` batch. That the two lower through
//! one function today (`cli::doc::apply_slot_target`) is a fact about today's code,
//! not a contract, so the acceptance holds both doors to the claim independently.
//!
//! Each arm asserts four things, because a refusal that corrupts, or that dead-ends
//! the agent, is not the fix: **non-zero exit**, the staged bytes **byte-identical**
//! across the reject (nothing partially landed), the structural symptom **absent**,
//! and the printed route **followed** — the test re-runs the write with the heading
//! demoted to the depth the message itself named, and that write lands.
//!
//! The per-doctype × per-depth axis lives next door in
//! `crates/cli/tests/item_slot_ceiling_axis.rs`, which iterates the registry; this
//! suite is the three named variants and the declared-open read-side bound.
//!
//! [`implementation/roadmap.md`]: ../../../implementation/roadmap.md
//! [`DECISIONS.md`]: ../../../DECISIONS.md

use crate::support;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use support::trial_corpus::{State, TrialCorpus};

/// A migrate task with its create gate open, and (optionally) the managed doc
/// already created — the state both doors are driven from.
struct Fixture {
    corpus: TrialCorpus,
    task: String,
}

impl Fixture {
    /// A `fresh` corpus carrying one committed foreign source, migrated into a task
    /// whose workflow grants `doctype`'s create gate. The doc is **not** created —
    /// the `doc author` batch door creates it itself.
    fn migrated(doctype: &str) -> Fixture {
        let corpus = TrialCorpus::build(State::Fresh);
        let source = format!("docs/legacy-{doctype}.md");
        fs::create_dir_all(corpus.repo().join("docs")).expect("create the foreign source dir");
        fs::write(
            corpus.repo().join(&source),
            format!("# Legacy {doctype}\n\nfree-form prose the migration rewrites.\n"),
        )
        .expect("write the foreign source");
        corpus.git(&["add", "docs"]);
        corpus.git(&["commit", "-q", "-m", "the foreign source"]);

        let stdout = corpus.jigc_ok(&["migrate", &source, "--as", doctype]);
        let task = stdout
            .lines()
            .find_map(|line| line.strip_prefix("task minted: "))
            .unwrap_or_else(|| panic!("a mint must print `task minted: <id>`; got:\n{stdout}"))
            .trim()
            .to_string();
        Fixture { corpus, task }
    }

    /// Migrate a **second** committed foreign source of `doctype`, returning the
    /// composed bytes — the `migrate-<doctype>` workflow's own door, and since M52
    /// Increment 9 / T2 the only door that composes it.
    fn migrate_again(&self, doctype: &str) -> String {
        let source = format!("docs/legacy-{doctype}-again.md");
        fs::write(
            self.corpus.repo().join(&source),
            format!("# Legacy {doctype} again\n\nmore free-form prose.\n"),
        )
        .expect("write the second foreign source");
        self.corpus.git(&["add", &source]);
        self.corpus
            .git(&["commit", "-q", "-m", "the second foreign source"]);
        self.corpus.jigc_ok(&["migrate", &source, "--as", doctype])
    }

    /// `jigc doc create <doctype>`, returning the doc id the binary emitted.
    fn create(&self, doctype: &str, title: &str) -> String {
        self.corpus
            .jigc_ok(&[
                "doc", "create", doctype, "--title", title, "--task", &self.task,
            ])
            .trim()
            .to_string()
    }

    /// `jigc doc add-item`, returning the item address the binary emitted.
    fn add_item(&self, section: &str, title: &str) -> String {
        self.corpus
            .jigc_ok(&[
                "doc", "add-item", section, "--title", title, "--task", &self.task,
            ])
            .trim()
            .to_string()
    }

    /// `jigc doc set-slot <address>` with `prose` on stdin — the **first** write
    /// door, run raw so a refusal is the observation rather than a panic.
    fn set_slot(&self, address: &str, prose: &str) -> std::process::Output {
        self.corpus.jigc_stdin(
            &[
                "doc",
                "set-slot",
                address,
                "--from-file",
                "-",
                "--task",
                &self.task,
            ],
            prose,
        )
    }

    /// `jigc doc author <doctype>` with `payload` on stdin — the **second** write
    /// door (the whole-document batch), run raw for the same reason.
    fn author(&self, doctype: &str, payload: &str) -> std::process::Output {
        self.corpus.jigc_stdin(
            &[
                "doc",
                "author",
                doctype,
                "--from-file",
                "-",
                "--task",
                &self.task,
            ],
            payload,
        )
    }

    /// Every file under the task's staged `docs/` tree, path → bytes. The
    /// byte-identity fence on the `set-slot` door: a refused write must leave the
    /// working area **exactly** as it found it, ledger included.
    fn staged_tree(&self) -> BTreeMap<String, Vec<u8>> {
        let root = self
            .corpus
            .repo()
            .join(format!(".jigc/tasks/{}/docs", self.task));
        let mut out = BTreeMap::new();
        collect(&root, &root, &mut out);
        out
    }

    /// Every staged **document** in the task's working area (`*.md`), path → bytes.
    ///
    /// The `author` door's fence is stated over documents rather than the whole
    /// tree because a refused batch *does* leave one non-document trace: the
    /// create step's `provenance.json` entry, written before the slot writes it is
    /// refused at. That residue is asserted **by name** in
    /// [`variant_1_the_ghost_item_is_blocked_on_the_author_door`], together with
    /// the checks that make it benign — it is a ledger note, never bytes an agent
    /// or a human would read as content.
    fn staged_documents(&self) -> BTreeMap<String, Vec<u8>> {
        self.staged_tree()
            .into_iter()
            .filter(|(path, _)| path.ends_with(".md"))
            .collect()
    }

    /// The task's staged-doc provenance ledger, verbatim.
    fn provenance(&self) -> String {
        fs::read_to_string(
            self.corpus
                .repo()
                .join(format!(".jigc/tasks/{}/docs/provenance.json", self.task)),
        )
        .expect("read the staged provenance ledger")
    }

    /// Whether a doc's staged file exists at all.
    fn staged_doc_exists(&self, doc_id: &str) -> bool {
        self.corpus
            .repo()
            .join(format!(".jigc/tasks/{}/docs/{doc_id}.md", self.task))
            .exists()
    }

    /// The staged bytes of one doc in this task's working area.
    fn staged_doc(&self, doc_id: &str) -> String {
        fs::read_to_string(
            self.corpus
                .repo()
                .join(format!(".jigc/tasks/{}/docs/{doc_id}.md", self.task)),
        )
        .expect("read the staged doc")
    }

    /// `jigc doc show <address> --task <id> --format json` over the staged copy.
    fn show(&self, address: &str) -> String {
        self.corpus.jigc_ok(&[
            "doc", "show", address, "--task", &self.task, "--format", "json",
        ])
    }
}

fn collect(root: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let entry = entry.expect("read a staged entry");
        let path: PathBuf = entry.path();
        if path.is_dir() {
            collect(root, &path, out);
        } else {
            let rel = path
                .strip_prefix(root)
                .expect("a staged path sits under the staged root")
                .to_string_lossy()
                .into_owned();
            out.insert(rel, fs::read(&path).expect("read a staged file"));
        }
    }
}

/// Count lines that are **exactly** `heading`. A substring count would read
/// `##### Proves` as a `#### Proves` hit and `#### Context` as a `## Context` one —
/// precisely the distinction every variant here turns on.
fn heading_lines(source: &str, heading: &str) -> usize {
    source
        .lines()
        .filter(|line| line.trim_end() == heading)
        .count()
}

/// Slot prose carrying one ATX heading at `depth`, plus prose beneath it — the
/// smuggled bytes whose fate is what makes each variant a corruption rather than a
/// formatting quibble.
fn poisoned(depth: usize, heading: &str) -> String {
    format!(
        "Axis prose.\n\n{} {heading}\n\n{SMUGGLED}\n",
        "#".repeat(depth)
    )
}

/// The prose beneath the smuggled heading — one distinctive string, so its
/// survival (or destruction) is checkable in the staged bytes.
const SMUGGLED: &str = "the smuggled paragraph";

/// The depth the reject's own message names as free **at this address**, read out
/// of the emitted bytes rather than assumed: the ceiling is per-address, so a
/// test-side constant would be right for `spec` and wrong for `roadmap`.
fn depth_named_free(stderr: &str) -> usize {
    let head = stderr
        .split_once("` is the shallowest depth free at this address")
        .unwrap_or_else(|| panic!("the reject must name the depth free here; got:\n{stderr}"))
        .0;
    let marker = head
        .rsplit_once('`')
        .unwrap_or_else(|| panic!("the named depth is backticked; got:\n{stderr}"))
        .1;
    assert!(
        marker.chars().all(|c| c == '#') && !marker.is_empty(),
        "the named depth is an ATX marker; got `{marker}` in:\n{stderr}"
    );
    marker.len()
}

/// Assert the shape every refusal in this suite shares, and return the depth its
/// route sends the agent to. Kept in one place so a variant arm reads as the
/// *scenario*, not as a re-assertion of the contract.
fn assert_refused(out: &std::process::Output, what: &str) -> usize {
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "{what} must be refused, not accepted:\n{stderr}"
    );
    assert!(
        stderr.contains("write.slot-heading-depth"),
        "{what} is refused as the ceiling reject:\n{stderr}"
    );
    assert!(
        stderr.contains("route: demote the heading to the depth the message names"),
        "{what} carries a route, not a dead end:\n{stderr}"
    );
    depth_named_free(&stderr)
}

// ---------------------------------------------------------------------------
// Variant 1 — the minted ghost item
// ---------------------------------------------------------------------------

/// A `spec` criterion whose `statement` carries `### Ghost  {#ghost}` minted a
/// second real item through jigc's own write verb. Door one.
#[test]
fn variant_1_the_ghost_item_is_blocked_on_the_set_slot_door() {
    let fixture = Fixture::migrated("spec");
    let doc = fixture.create("spec", "Widget");
    let item = fixture.add_item(&format!("{doc}#criteria"), "Pads input");
    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/statement"),
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        "Padding leaves a wide-enough string unchanged.\n",
    );

    let before = fixture.staged_tree();
    let out = fixture.set_slot(
        &format!("{item}/statement"),
        &poisoned(3, "Ghost  {#ghost}"),
    );
    let free = assert_refused(&out, "a `###` heading in a criterion statement");

    assert_eq!(
        fixture.staged_tree(),
        before,
        "the refused write leaves the staged bytes byte-identical"
    );
    let items = fixture.show(&format!("{doc}#criteria"));
    assert!(
        !items.contains("ghost"),
        "no ghost item is minted:\n{items}"
    );
    assert_eq!(
        items.matches("\"id\":").count(),
        1,
        "the criteria section still holds exactly one item:\n{items}"
    );

    // The route is followed, not merely printed: the same write at the depth the
    // message named lands.
    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/statement"),
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        &poisoned(free, "Ghost"),
    );
    assert!(
        fixture.staged_doc(&doc).contains(SMUGGLED),
        "following the route lands the prose the agent wanted to write"
    );
}

/// The same ghost, authored through the whole-document batch door. The doc does not
/// exist yet, so the byte fence is that **no document bytes land**: the refused
/// batch leaves no half-written instance for anything downstream to read.
///
/// The one trace it *does* leave is stated here rather than filtered out of the
/// fence: the batch's create step records `spec:widget` in `provenance.json` before
/// the slot write it is refused at, so the ledger keeps an entry for a document
/// that is not on disk. The two assertions below are what make that benign rather
/// than a second corruption — the finalize gate refuses the task with a followable
/// route instead of committing a phantom, and re-authoring lands normally.
#[test]
fn variant_1_the_ghost_item_is_blocked_on_the_author_door() {
    let fixture = Fixture::migrated("spec");
    let before = fixture.staged_documents();
    let before_provenance = fixture.provenance();

    let out = fixture.author("spec", &spec_payload(&poisoned(3, "Ghost  {#ghost}")));
    let free = assert_refused(&out, "a `###` heading in an authored criterion statement");

    assert_eq!(
        fixture.staged_documents(),
        before,
        "the refused batch lands no document bytes"
    );
    assert!(
        !fixture.staged_doc_exists("spec:widget"),
        "the instance the batch would have created is not on disk"
    );

    // The residue, named — and shown harmless at the gate it could have lied to.
    assert_ne!(
        fixture.provenance(),
        before_provenance,
        "the refused batch does record its create step in the ledger — asserted so \
         this stays a KNOWN trace rather than a silently tolerated one"
    );
    let finalize = fixture.corpus.jigc(&["task", "finalize", &fixture.task]);
    let stderr = String::from_utf8_lossy(&finalize.stderr);
    assert!(
        !finalize.status.success() && stderr.contains("finalize.migration-no-replacement"),
        "the ledger entry does not buy a phantom document a clean finalize:\n{stderr}"
    );

    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "spec",
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        &spec_payload(&poisoned(free, "Ghost")),
    );
    let items = fixture.show("spec:widget#criteria");
    assert_eq!(
        items.matches("\"id\":").count(),
        1,
        "the followed route authors exactly the one intended item:\n{items}"
    );
}

// ---------------------------------------------------------------------------
// Variant 2 — the sibling-leaf reattribution, and its second, destroying write
// ---------------------------------------------------------------------------

/// A `roadmap` milestone is the only *multi-slot* repeatable either pack ships, so
/// `####` is a sub-label there, not free prose depth. `#### Proves` written into
/// `decomposition` used to land bytes that fell outside every recorded slot span —
/// invisible to `doc show`, and destroyed by the **next correct write**. Both writes
/// are driven here: the poisoned one must be refused, and the sibling write that
/// followed it must find the authored prose intact.
#[test]
fn variant_2_the_sibling_leaf_reattribution_is_blocked_on_the_set_slot_door() {
    let fixture = Fixture::migrated("roadmap");
    let doc = fixture.create("roadmap", "Roadmap");
    let item = fixture.add_item(&format!("{doc}#milestones"), "M-Alpha");
    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/decomposition"),
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        "Increment 1 lands the seam.\n",
    );

    let before = fixture.staged_tree();
    let out = fixture.set_slot(&format!("{item}/decomposition"), &poisoned(4, "Proves"));
    let free = assert_refused(&out, "a `####` sub-label depth heading in a milestone slot");
    assert_eq!(
        free, 5,
        "a multi-slot item reserves the sub-label depth too, so `#####` is the first free one"
    );
    assert_eq!(
        fixture.staged_tree(),
        before,
        "the refused write leaves the staged bytes byte-identical"
    );

    // The second, destroying write: the *next correct* write to a sibling leaf
    // re-renders the item from the parse. With the poisoned write refused, there is
    // nothing off-span for it to destroy.
    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/proves"),
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        "That the composed loop lands one task end to end.\n",
    );
    let staged = fixture.staged_doc(&doc);
    assert!(
        staged.contains("Increment 1 lands the seam."),
        "the sibling write preserves the authored decomposition:\n{staged}"
    );
    assert_eq!(
        heading_lines(&staged, "#### Proves"),
        1,
        "the milestone carries exactly one `Proves` sub-label:\n{staged}"
    );

    // The route, followed at the depth it named.
    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/decomposition"),
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        &poisoned(free, "Proves"),
    );
    let staged = fixture.staged_doc(&doc);
    assert_eq!(
        heading_lines(&staged, "#### Proves"),
        1,
        "the followed route writes a `#####` sub-heading, not a second sub-label:\n{staged}"
    );
    assert!(staged.contains(SMUGGLED), "the intended prose lands");
}

/// The same reattribution inside a **single** `doc author` batch, with the poisoned
/// leaf ordered first — the shape in which one batch both wrote and destroyed the
/// prose. The whole batch must be refused.
#[test]
fn variant_2_the_sibling_leaf_reattribution_is_blocked_on_the_author_door() {
    let fixture = Fixture::migrated("roadmap");
    let before = fixture.staged_documents();

    let out = fixture.author("roadmap", &roadmap_payload(&poisoned(4, "Proves")));
    let free = assert_refused(&out, "a `####` heading in an authored milestone slot");
    assert_eq!(
        fixture.staged_documents(),
        before,
        "the refused batch lands no document bytes"
    );
    assert!(
        !fixture.staged_doc_exists("roadmap:roadmap"),
        "the instance the batch would have created is not on disk"
    );

    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "roadmap",
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        &roadmap_payload(&poisoned(free, "Proves")),
    );
    let staged = fixture.staged_doc("roadmap:roadmap");
    assert_eq!(
        heading_lines(&staged, "#### Proves"),
        1,
        "the authored milestone carries exactly one `Proves` sub-label:\n{staged}"
    );
    assert!(staged.contains(SMUGGLED), "the intended prose lands");
}

// ---------------------------------------------------------------------------
// Variant 3 — the hijacked downstream section
// ---------------------------------------------------------------------------

/// `## Context` inside a `spec` criterion's `statement` produced a **duplicate**
/// `## Context`; the `criteria` section ended at it and the items below dropped out
/// of the parse, all at exit 0 with `task validate` clean (trailing-surplus
/// tolerance is what made it invisible).
#[test]
fn variant_3_the_hijacked_downstream_section_is_blocked_on_the_set_slot_door() {
    let fixture = Fixture::migrated("spec");
    let doc = fixture.create("spec", "Widget");
    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{doc}#context"),
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        "The vendored runtime shipped its own.\n",
    );
    let item = fixture.add_item(&format!("{doc}#criteria"), "Pads input");
    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/statement"),
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        "Padding leaves a wide-enough string unchanged.\n",
    );

    let before = fixture.staged_tree();
    let out = fixture.set_slot(&format!("{item}/statement"), &poisoned(2, "Context"));
    let free = assert_refused(
        &out,
        "a `##` section-depth heading in a criterion statement",
    );

    assert_eq!(
        fixture.staged_tree(),
        before,
        "the refused write leaves the staged bytes byte-identical"
    );
    let staged = fixture.staged_doc(&doc);
    assert_eq!(
        heading_lines(&staged, "## Context"),
        1,
        "the document still carries exactly one `## Context` section:\n{staged}"
    );
    let items = fixture.show(&format!("{doc}#criteria"));
    assert_eq!(
        items.matches("\"id\":").count(),
        1,
        "no item is swallowed by a hijacked section boundary:\n{items}"
    );

    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/statement"),
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        &poisoned(free, "Context"),
    );
    let staged = fixture.staged_doc(&doc);
    assert_eq!(
        heading_lines(&staged, "## Context"),
        1,
        "the followed route writes a deeper heading, not a second section:\n{staged}"
    );
}

/// The same hijack authored through the batch door.
#[test]
fn variant_3_the_hijacked_downstream_section_is_blocked_on_the_author_door() {
    let fixture = Fixture::migrated("spec");
    let before = fixture.staged_documents();

    let out = fixture.author("spec", &spec_payload(&poisoned(2, "Context")));
    let free = assert_refused(&out, "a `##` heading in an authored criterion statement");
    assert_eq!(
        fixture.staged_documents(),
        before,
        "the refused batch lands no document bytes"
    );
    assert!(
        !fixture.staged_doc_exists("spec:widget"),
        "the instance the batch would have created is not on disk"
    );

    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "spec",
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        &spec_payload(&poisoned(free, "Context")),
    );
    let staged = fixture.staged_doc("spec:widget");
    assert_eq!(
        heading_lines(&staged, "## Context"),
        1,
        "the authored document carries exactly one `## Context` section:\n{staged}"
    );
}

// ---------------------------------------------------------------------------
// The declared-open read-side bound
// ---------------------------------------------------------------------------

/// **The read-side matching-label OOB arm is declared open, not claimed closed.**
///
/// M45's write-time reject closes the *jigc-authored* arm of the class. The
/// out-of-band arm is not closed: a `#### <Declared-Leaf-Title>` hand-written into
/// *another* leaf's prose parses as that leaf's sub-label, so the prose beneath it
/// is silently reattributed — no finding, exit 0. Both halves of the declaration
/// are fenced here so neither can quietly drift out of true:
///
/// * the **statement** stands in the two docs that own it
///   ([`implementation/parsing.md`] → Slot heading-depth ceiling, [`design/validation.md`]
///   → The M45 registrations);
/// * the **behaviour** still matches what those docs declare. If the arm is ever
///   closed, this witness reddens — and closing it therefore *requires* updating
///   the declaration in the same motion, which is the whole point of writing a
///   bound down rather than hoping it is remembered.
///
/// [`implementation/parsing.md`]: ../../../implementation/parsing.md
/// [`design/validation.md`]: ../../../design/validation.md
#[test]
fn the_read_side_matching_label_arm_is_declared_open_not_claimed_closed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the repo root sits two levels above crates/cli")
        .to_path_buf();
    let parsing = fs::read_to_string(root.join("implementation/parsing.md"))
        .expect("read implementation/parsing.md");
    assert!(
        parsing.contains("The read side stays open for the matching-label case"),
        "parsing.md still declares the matching-label arm open"
    );
    assert!(
        parsing.contains("the **OOB-authored** path is not closed"),
        "parsing.md still names the OOB-authored path as the open half"
    );
    let validation =
        fs::read_to_string(root.join("design/validation.md")).expect("read design/validation.md");
    assert!(
        validation.contains("the read-side **matching-label** OOB arm stays open"),
        "validation.md's M45 registration still declares the arm open"
    );

    // The witness: an out-of-band matching-label heading, written straight to the
    // staged bytes (the one path the write verbs do not own).
    let fixture = Fixture::migrated("roadmap");

    // No shipped surface claims otherwise. The two surfaces carrying the ceiling
    // statement — `set-slot --help` and the `{{schema:<doctype>}}` projection at an
    // authoring solicit — both speak about the *write* they reject; neither offers a
    // detection story for prose that arrives some other way, and neither may start
    // offering one while the arm is open.
    for (label, text) in [
        (
            "jigc doc set-slot --help",
            fixture.corpus.jigc_ok(&["doc", "set-slot", "--help"]),
        ),
        (
            "the {{schema:roadmap}} projection",
            // Composed through `migrate-roadmap`'s own declared door — the workflow is
            // verb-routed, so both compose-by-name doors refuse it (M52 Increment 9 /
            // T2) and the verb that stages the foreign source is what composes the
            // projection this arm reads. A second source, so the mint does not collide
            // with the one the fixture already opened.
            fixture.migrate_again("roadmap"),
        ),
    ] {
        let lowered = text.to_lowercase();
        for claim in ["out-of-band", "out of band", "hand-edit", "edited by hand"] {
            assert!(
                !lowered.contains(claim),
                "{label} must not reach for `{claim}` while the read-side arm is \
                 declared open — the write-time reject covers writes, nothing else"
            );
        }
    }

    let doc = fixture.create("roadmap", "Roadmap");
    let item = fixture.add_item(&format!("{doc}#milestones"), "M-Alpha");
    fixture.corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/decomposition"),
            "--from-file",
            "-",
            "--task",
            &fixture.task,
        ],
        "Increment 1 lands the seam.\n",
    );
    let path = fixture
        .corpus
        .repo()
        .join(format!(".jigc/tasks/{}/docs/{doc}.md", fixture.task));
    let staged = fs::read_to_string(&path).expect("read the staged roadmap");
    fs::write(
        &path,
        staged.replace(
            "Increment 1 lands the seam.\n",
            &format!("Increment 1 lands the seam.\n\n#### Proves\n\n{SMUGGLED}\n"),
        ),
    )
    .expect("write the out-of-band edit");

    let rendered = fixture.show(&format!("{doc}#milestones"));
    assert!(
        !rendered.contains(SMUGGLED),
        "the declared-open arm: the reattributed prose is silently dropped from the \
         read, with no diagnostic — if this now surfaces, the arm has been closed and \
         parsing.md / validation.md must stop declaring it open:\n{rendered}"
    );
}

// ---------------------------------------------------------------------------
// The two batch payloads
// ---------------------------------------------------------------------------

/// A whole-`spec` batch payload whose one criterion carries `statement` — the shape
/// the `migrate-spec` step's `{{schema:spec}}` skeleton solicits.
fn spec_payload(statement: &str) -> String {
    format!(
        "title: \"Widget\"\n\
         sections:\n\
         \x20 - id: goal\n\
         \x20   set:\n\
         \x20     goal: |-\n\
         \x20       <<A helper that pads a string to a fixed width.>>\n\
         \x20 - id: context\n\
         \x20   set:\n\
         \x20     context: |-\n\
         \x20       <<The vendored runtime shipped its own.>>\n\
         \x20 - id: criteria\n\
         \x20   items:\n\
         \x20     - title: \"Pads input\"\n\
         \x20       set:\n\
         \x20         statement: |-\n{}\n",
        indented(statement, 12)
    )
}

/// A whole-`roadmap` batch payload whose one milestone carries **both** slots, the
/// poisoned one **first** — the leaf order in which a single batch wrote and then
/// destroyed the smuggled prose.
fn roadmap_payload(decomposition: &str) -> String {
    format!(
        "title: \"Roadmap\"\n\
         sections:\n\
         \x20 - id: milestones\n\
         \x20   items:\n\
         \x20     - title: \"M-Alpha\"\n\
         \x20       set:\n\
         \x20         decomposition: |-\n{}\n\
         \x20         proves: |-\n\
         \x20           <<That the composed loop lands one task end to end.>>\n",
        indented(decomposition, 12)
    )
}

/// Wrap multi-line slot prose in the literal `<<…>>` markers the author grammar
/// requires and indent it into a YAML block scalar.
fn indented(prose: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    let body = format!("<<{}>>", prose.trim_end_matches('\n'));
    body.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{pad}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
