//! M47 Increment 6, T1 — **the write-verb × miss-shape axis, one row per cell.**
//!
//! The class: *which miss a write reject claims to be*. An **item-id miss** — the
//! addressed item was never minted — is not a shape question: the schema names the
//! *shape*, never the corpus's real item ids, so `jigc doc schema <doctype>` is a dead
//! end there. Before this suite the diagnosis was decided per call site, and four of the
//! eight cells got it wrong (`set-field --value`, `set-field --unset` at an undeclared
//! field, `retitle-item`, and nested `add-item` under an absent parent), while the two
//! already-correct cells (`set-slot`, `remove-item`) had no axis-level fence keeping the
//! rest with them (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 8 — sweep the axis,
//! not the instance; `baseline.md` §4e).
//!
//! The axis is enumerated as **data**, one row per cell, so a write verb added later is
//! covered by adding a row rather than by remembering this file exists:
//!
//!   * the **item-id misses** → `write.not-present`
//!     (`set-slot` · `remove-item` · `set-field --value` · `set-field --unset` at a
//!     declared field · `set-field --unset` at an *undeclared* field — the item miss
//!     outranks the field question — · `retitle-item` · nested `add-item` under an
//!     absent parent, the eighth cell the baseline census under-counted · and, added
//!     with the route column, `retitle-item` at an absent **nested** item under a real
//!     parent, the deepest strip);
//!   * the **undeclared-section** misses → `write.unknown-section`, over the same six
//!     item-addressing verbs plus the bare `add-item` (see the column note below);
//!   * a **genuine declared-shape defect** (`add-item` into a non-repeatable section) →
//!     `write.wrong-shape`, which the flip deliberately leaves standing.
//!
//! Every cell asserts the same three things through the real binary: the write **blocks
//! non-zero**, the emitted `--format json` finding carries the expected `code`, and the
//! **staged bytes are byte-identical** to before the call (a rejected write persists
//! nothing).
//!
//! **T2 adds the route column** — the same axis, one hop further: naming the miss
//! correctly is worthless if the route the agent is handed is still the schema dead end.
//! Every item-id-miss cell must emit
//! `jigc doc show <type>:<slug>#<top-section> --task <id>` carrying the **real** slug and
//! the **resolved** task id (never the `<address>` / `<task-id>` placeholders of the
//! engine's defensive fallback), and the emitted argv is **run verbatim** — it must exit 0
//! and print the section's live item ids, so the recovery is followable and not merely
//! well-worded (`design/surface-contract.md` law 2: nothing hides;
//! `design/validation.md` → the `write.*` route split). A **nested** cell strips to the
//! top *showable* section — the M44 N2 pin, re-asserted here at the four verbs the CLI
//! enrichment did not reach (`set-field --value` / `--unset` · `retitle-item` ·
//! `add-item`).
//!
//! **T3 closes the id-from-leaf strip.** Naming the miss and routing it are worth nothing
//! at a cell that never reaches either, and two *schema-only* CLI pre-checks —
//! `write.id-from-field` (`set-field` at an id-from leaf) and `write.identity-change`
//! (`retitle-item` under an **enum** id-from), plus the engine's own `retitle_item`
//! id-from re-validation — answered from the doctype alone and so out-ranked presence:
//! they asserted what a **nonexistent** item derives its id from, and routed to a
//! `retitle-item` / `remove-item` / `set-field` that blocks on the same absence. Four rows
//! were added over the two verbs × the two depths, and — the reason the one pre-existing
//! nested row greened over a live defect — **the fixture's id-from topology was corrected
//! to the shipped dev pack's**: its change-group `category` was declared a plain `string`,
//! the one shape that sidesteps the enum arm, while `crates/cli/pack/schemas/changelog.yaml`
//! declares it an **enum**. A fixture authored in the case the implementation handles
//! proves nothing about the case the corpus actually has.
//!
//! **The undeclared-SECTION column is swept like the item-id one.** The matrix is verb ×
//! miss-shape, and this column shipped with **one** cell (bare `add-item`) against the
//! item-id column's twelve — so the same miss came back four different ways underneath it:
//! `set-slot` / `remove-item` answered `write.not-present` (and, enriched, handed back a
//! `jigc doc show <doc>#<undeclared-section>` whose **verbatim run exits 1** — the
//! un-followable route this suite's T2 property exists to forbid), `retitle-item` / nested
//! `add-item` answered `write.wrong-shape`, and `set-field --unset` answered
//! `write.unknown-field` about a field on an item in a section that does not exist. Six
//! rows join the column, and the shape cells' route is now **run verbatim** too: it must be
//! `jigc doc schema <doctype>` — the read that genuinely answers a shape question — and it
//! must exit 0 naming the doctype's declared sections. Engine-side the fix is one ranked
//! predicate (`engine::write::section_undeclared`, rank 1 of shape → presence → leaf)
//! opening every item-addressing door.
//!
//! **M48 adds the title-miss column.** Every column above is an *address* miss; a
//! **doc-minting** verb (`create` / `author`) can miss one hop earlier — on the title it
//! was handed — and until M48 neither shape was a miss at all: both exited 0, one of them
//! committing a second document nobody authored. The two rows split on whether the
//! identity moves, because the recovery does not: a call that would mint a **different**
//! identity beside the one the gate's `as:` role already binds is `write.identity-change`
//! (a second document, not a correction), while a call landing on the **same** identity
//! that merely drops the title is identity-**stable** (`design/storage.md` → Identity)
//! and earns `write.title-ignored`. Both route at `jigc doc rename` — a route that is
//! itself a write, so it is run verbatim against a **private** fixture in the same state.
//!
//! **M49 adds the collision column.** Every column above is a *miss* — an address that
//! names nothing. This one is the opposite: the address is right, the shape is right, and
//! the **minted id is taken**. Two genuinely different titles can slug alike (`1-3-0` and
//! `1.3.0` mint the same id), and until M49 the reject's route was a `Route::human` —
//! *"the target already exists — edit it in place"* — which is wrong for a second, distinct
//! entry, and is not runnable at all, let alone from inside the `doc author` batch it
//! rejects whole. With `--slug` on `add-item` (T4 of this increment) the recovery **is** a
//! command, so the route is mechanical: the same mint under a free id. Two rows, because
//! the collision reaches an agent through two doors that must not disagree — the per-leaf
//! verb and the batch — and both inherit the one CLI-seam enrichment at
//! `apply_add_item_target`. The route is run **verbatim** and must *land the second item*,
//! not merely exit 0.
//!
//! **The batch door has a destination axis of its own, and the column shipped over one
//! cell of it** (the T5 triage and its validation fix —
//! [`a_batch_collision_route_describes_the_state_it_leaves_behind`]). The enrichment hands
//! back a mint *into a doc, beside an item*, and both halves are claims about a state the
//! batch has already unwound by the time the route prints. Two questions decide it:
//! **provisioning** (over a doc the task already staged the whole-or-nothing rollback
//! restores the pre-image; on the **create** arm — the shape every `migrate-*` workflow
//! drives — it removes the file the create provisioned, so the emitted command exited 1)
//! and **duplicate origin** (the id is taken by the corpus, or by an earlier item of the
//! same payload, which the rollback takes with it). The `CELLS` batch row below drives
//! staged × corpus-taken — the state the implementation was written against — so the
//! column greened over the deliverable's own headline case, *890 titles minting 884
//! slugs*, where the collision is **inside the payload**: the probe ran against the
//! discarded in-progress buffer, so the route asserted an item that is not there and
//! suffixed `-2` around a bare id that is free. The axis is driven in its own test rather
//! than as `CELLS` rows, because a row is defined against the one shared fixture.
//!
//! **M49 sweeps the undeclared-section column's nested arm.** Every row of that column
//! gets the section wrong at the address's **first** hop. An address can get it wrong at a
//! later one — `#releases/1-3-0/bogus/xyz…`, where `releases` is declared, `1-3-0` is live,
//! and the nested-section segment `bogus` is declared nowhere in the release block — and
//! that one miss came back **four** different ways across the six item-addressing doors:
//! `write.not-present` at `set-slot` / `remove-item`, `write.wrong-shape` at
//! `set-field --value` / `retitle-item` / both `add-item` shapes, and `write.unknown-field`
//! at `set-field --unset`. A driver keys on `(code, target)`, so one defect answering under
//! four keys is not a wording problem. Seven rows join the column — the six verbs plus the
//! nested dual of the bare `add-item` cell, whose destination *is* the undeclared nested
//! section — and all seven converge on the shipped `write.unknown-section` and its schema
//! read: no new contract member is minted, because a new code is itself a spend
//! (`design/command-output-contract.md` → Evolution posture, the M49 paragraph authorizing
//! the flip on the pinned key; `settle-record.md` → D8). Engine-side the fix is the ranked
//! predicate's nested sibling (`engine::write::nested_section_undeclared`), asked by the
//! same six doors that already ask `section_undeclared`.
//!
//! **The key's other half is asserted over the whole matrix.** Naming the miss is half of
//! `(code, target)`; wherever a cell's argv carries a write address, the finding's
//! `key.target` **and** its `location.address` must be that address, verbatim and in full.
//! Stated as a property of the matrix rather than a per-row column, so a row added later
//! inherits it.
//!
//! **M49 closes the column's section-level arm — the bound this file used to declare.**
//! The two *section-level* address forms — `set-slot` at `#<undeclared-section>` and
//! `set-field` at `#<undeclared-section>/<leaf>`, with no item hop — were **not** cells of
//! this matrix, because the CLI's own target resolvers refuse before any write door is
//! reached: searching for a slot or a field inside a section that does not exist, they
//! found neither and emitted a bare `{"error": "no slot addressed by …"}` — exit 1,
//! code-less, route-less, **outside the finding envelope**. `jigc doc author` reaches both
//! resolvers through its batch lowering (a payload section's `set:` key is a field hop; the
//! section's own id is its slot, key-less), so the same bare form came back at two more
//! doors — a fourth producer the M47 census did not count. Calling that "a different seam"
//! kept `design/validation.md`'s **universal** — *the address names a section the schema
//! does not declare* — false at four of them (M47 increment 6, advisory 2). Five rows join
//! the column, all on the shipped `write.unknown-section` and its schema read, and the
//! resolvers now ask the engine's own rank-1 predicate
//! (`engine::write::undeclared_section_splice`) rather than inferring the miss from the
//! absent leaf below.
//!
//! **The column is fenced against the write-verb registry, not hand-counted**
//! ([`every_doc_write_verb_answers_its_address_shape_and_declaredness_cells`]): every `doc`
//! write verb in `cli::cli::VERB_KINDS` either carries rows here or is dispositioned with
//! the reason it carries none. Shipping over one cell is how this column started, so a
//! ninth write verb is now a row to add rather than a rediscovery.
//!
//! **Declared bound, one hop shorter.** A *leaf*-only address with no section hop at all —
//! `set-field` at `#<name>`, the single-hop MVP form that searches every section for a
//! field of that id — is not a cell: it names no section, so *undeclared section* is not
//! what it got wrong, and it keeps the resolver's own sentence.
//!
//! **Declared bound, one level down (M49), narrowed at M50.** The nested form with **no
//! trailing item hop** splits by verb. At `set-slot` / `set-field`,
//! `#releases/1-3-0/bogus` names a **leaf** on a live item — `write.unknown-field` is the
//! right answer and is left standing. At `remove-item` / `retitle-item` the same string
//! names no item at all, and used to bottom out in the CLI address resolver as a bare
//! `{"error": …}` envelope before any write door was reached; that half of the bound is
//! **no longer a bound** — the address-shape column below is exactly that seam, and the
//! string now earns `write.wrong-shape` at both verbs (the *leading* hop `releases` is
//! declared, so rank 1 has nothing to say and what is left is the form the verb takes).
//! `add-item`, whose destination is a *section*, keeps its own row above.
//!
//! **M50 adds the address-shape column.** Every column above reaches a write door;
//! thirteen cells never did. `add-item` resolves its destination through
//! `add_item_target` and `remove-item` / `retitle-item` theirs through
//! `remove_item_target`, and an address whose **shape** those resolvers cannot map — a
//! bare `<type>:<slug>` with no fragment at all, an `add-item` at an item, a
//! `remove-item` / `retitle-item` at a section or at a leaf — fell out as `None` and was
//! dressed by a bare `with_context`: exit 1 carrying `{"error": "no section addressed by
//! …"}`, **code-less, route-less, outside the finding envelope**, at three of the five
//! `doc` write verbs. The column has two dimensions, because the answer is not a property
//! of the address shape alone: rank 1 asks whether the **leading** hop names a declared
//! section at all (`engine::write::undeclared_section_splice`, the predicate the six
//! item-addressing doors and M49's section-level resolvers already ask), so an undeclared
//! leading hop is `write.unknown-section` whatever shape the rest of the address takes,
//! and only over a declared one is what is left a genuine declared-shape defect,
//! `write.wrong-shape`. `write.not-present` earns **no** row here: nothing has been looked
//! for in the corpus yet, so naming an item absent would be a law-1 lie about a document
//! the resolver has not read. Engine-side there is no fix at all — the escape is closed at
//! the **type** level, by the two resolvers returning `Result<_, Finding>`, an error type
//! `DocFailure`'s blanket `From<anyhow::Error>` cannot reach.
//!
//! **M50 gives the fence that dimension, and states the shape half as manufactured (T3).**
//! Every fence over this matrix until now had a **verb** axis and nothing else — it asked
//! whether each `doc` write verb drove *one* undeclared-section row — so it was green over
//! a column that shipped with one cell of eighteen, twice. It now carries the cross
//! itself: [`MISS_SHAPES`], one row per `(verb, address shape, leading-hop declaredness)`
//! the verb rejects, each carrying the code it must earn, totalled against `CELLS` **in
//! both directions** — no row may claim a cell nothing drives, and no driven cell may sit
//! outside the table. The verb axis stays **derived** (`VERB_KINDS` ▸ `doc` ▸ `Write`) and
//! grows a third disposition beside the rows and [`NO_SECTION_HOP`]:
//! [`COVERED_BY_LOWERING`], which is `jigc doc author` — it carries no address argv at
//! all, its payload lowering reaching the five addressed verbs' own resolvers, and its
//! `CELLS` rows are the witness. The **shape** half is a manufactured space and says so
//! at [`Shape`], in `crates/cli/tests/support/shape_space.rs`'s idiom and for its reason:
//! `engine::address::Fragment` supplies hop counts and no verdicts, and declaredness is a
//! property of a fixture rather than of any registry, so a fence that looped a table here
//! would enumerate nothing that decides anything.
//!
//! The widening bought six cells on its first run — the four-hop-or-deeper address under
//! an **undeclared** leading hop at all five addressed verbs, and `set-slot`'s
//! `#<undeclared>/<hop>` — every one of them already answering `write.unknown-section`
//! correctly, and every one of them undriven until the cross demanded a witness.
//!
//! **The second property this file now asserts is the envelope itself**
//! ([`every_write_miss_answers_inside_the_finding_envelope`]). Naming the miss and routing
//! the recovery are properties *of a finding*; the defect this increment closed was one
//! seam earlier — a reject that never became one. So every `CELLS` row's `--format json`
//! reject must carry the pinned `schema_version` + non-empty `findings` envelope and **no
//! top-level `error` key**, which is the one assertion that reddens when a resolver
//! regresses to a bare `anyhow` whatever code the row expects.

use cli::cli::{VERB_KINDS, VerbKind};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-miss-shape-axis-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
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

/// Recursively copy `from` into `to` (both directories).
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create dest dir");
    for entry in fs::read_dir(from).expect("read src dir") {
        let entry = entry.expect("dir entry");
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if src.is_dir() {
            copy_tree(&src, &dst);
        } else {
            fs::copy(&src, &dst).expect("copy file");
        }
    }
}

/// The fixture `changelog` schema — a two-level repeatable (so a nested `add-item`
/// under an absent parent has a home), carrying a `summary` slot and an **optional**
/// `link` field (the only unset-eligible shape: a required or defaulted field is
/// refused by the eligibility guard before the item is ever adjudicated), plus a
/// **non-repeatable** `overview` section so the genuine shape question has a target.
///
/// **The id-from topology mirrors the shipped dev pack's changelog, deliberately** (M47
/// Increment 6, T3 — the fixture-topology mask): the two CLI-side, schema-only
/// pre-checks that outranked item presence (`write.id-from-field` /
/// `write.identity-change`) are reached only through an id-from leaf, and the
/// **`write.identity-change`** arm only through an **enum** id-from. A fixture that
/// declares its change-group `category` as a plain `string` — as this one first did —
/// sidesteps that arm entirely and greens a cell the shipped pack false-fails. So both
/// `category` blocks are enums (`crates/cli/pack/schemas/changelog.yaml`), and a
/// single-level `staged` repeatable mirrors the pack's `unreleased-changes` so the
/// **top-level** enum id-from is on the axis too, not only the nested one.
///
/// It declares a **`location:`** for the same class of reason (M48): the title-miss rows
/// route at `jigc doc rename`, which refuses a doctype with no committed home as a
/// *transient sink* — so a home-less fixture would emit a route it cannot follow, and the
/// new column would green on a dead end. The nine shipped slug-identity doctypes those
/// rows model all declare one.
///
/// **The slotless `meta` section is the M50 T2 ingredient**, and it is a **header**
/// section because that is the shape the corpus actually has: every fields-only section
/// in both shipped packs (`adr` `status`, `spec`/`commit`/`arch-doc` headers) declares
/// `header: true`. `set-slot` at a declared section that hosts no prose slot is a cell of
/// the address-shape column with no ingredient anywhere else in this fixture —
/// `overview` declares a slot, `staged` / `releases` are repeatable — so a fixture
/// without it would leave that cell driven nowhere, which is how a column ships over part
/// of itself.
const CHANGELOG_SCHEMA: &str = "\
type: changelog
location: changelogs/
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: owner, type: string, optional: true }
  - id: overview
    slot: { hint: \"What this changelog covers.\", optional: true }
  - id: staged
    repeatable:
      id-from: category
      block:
        - { id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }
        - { id: notes, slot: { hint: \"One bullet per staged change.\" } }
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: link, type: string, optional: true }
        - { id: summary, slot: { hint: \"One-line release summary.\" } }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
";

/// Build a throwaway pack: the embedded dev pack tree copied to a temp dir, plus the
/// fixture `changelog` schema and a `log-change` workflow whose create-gate admits it.
fn fixture_pack() -> TempDir {
    let pack = TempDir::new("pack");
    let dev_pack = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack");
    copy_tree(&dev_pack, pack.path());
    // The fixture ships a deliberately divergent `changelog` shape, so it is NOT the
    // frozen dev pack — drop the copied freeze manifest (the pack-load gate would
    // otherwise block the un-bumped shape change). A manifest-less pack is unchecked.
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the copied freeze manifest");

    fs::write(
        pack.path().join("schemas").join("changelog.yaml"),
        CHANGELOG_SCHEMA,
    )
    .expect("write changelog schema");

    fs::write(
        pack.path().join("workflows").join("log-change.yaml"),
        "\
---
when: record a release's changes in the changelog
description: Author the changelog for a release.
usage: a release's changes need recording in the changelog.
creates-task: true
allows-create: [{type: changelog, as: changelog}]
---
{{ include: step:finalize }}
",
    )
    .expect("write log-change workflow");

    pack
}

/// Initialize a real git repo with one commit (composition reads HEAD) plus the
/// `.jigc/config/` project layer the cascade expects.
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
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// The held provisioning: the temp guards must stay alive (dropping them removes the
/// repo), so they are returned to the caller.
struct Fixture {
    repo: TempDir,
    home: TempDir,
    pack: TempDir,
    slug: String,
}

impl Fixture {
    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
        use std::io::Write;
        use std::process::Stdio;
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", self.pack.path())
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

    /// The staged working copy of the fixture changelog — the bytes a rejected write
    /// must leave untouched.
    fn staged(&self) -> PathBuf {
        self.repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join(TASK_ID)
            .join("docs")
            .join(format!("changelog:{}.md", self.slug))
    }

    /// Run a route's emitted command **verbatim** (shell-split, the leading `jigc`
    /// dropped), returning its trimmed stdout — the followability proof.
    fn run_route(&self, cmd: &str, what: &str) -> String {
        let argv = shell_split(cmd, self.repo.path(), self.home.path());
        assert_eq!(
            argv.first().map(String::as_str),
            Some("jigc"),
            "`{what}`: the route leads `jigc`"
        );
        let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
        ok_stdout(self.run(&args, None), what)
    }
}

/// Split an emitted command into argv **through a real `sh`** — the bytes an agent pastes
/// are parsed by the thing that will actually parse them, expansions included. A
/// hand-rolled splitter understands one quoting form and expands nothing, which is exactly
/// how a route carrying live `$` or `$( … )` in a title passes a suite and rewrites the
/// document in a terminal (M48 inc-2 triage). `set --` is the shell's own word splitter.
/// (Kept local: this suite's group root carries no `support` module, and pulling one in
/// would compile the whole shared fixture substrate into this target.)
fn shell_split(cmd: &str, cwd: &Path, home: &Path) -> Vec<String> {
    let script = format!("set -- {cmd}\nfor w in \"$@\"; do printf '%s\\0' \"$w\"; done");
    let out = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .current_dir(cwd)
        .env("HOME", home)
        .output()
        .expect("spawn sh");
    assert!(
        out.status.success(),
        "the emitted command line must parse as shell words; got `{cmd}`\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let mut words: Vec<String> = out
        .stdout
        .split(|b| *b == 0)
        .map(|w| String::from_utf8_lossy(w).into_owned())
        .collect();
    words.pop(); // the trailing NUL of the last word yields one empty tail element
    words
}

/// The task id `jigc start "log the release"` mints — the id the enriched route must
/// carry (never the `<task-id>` placeholder).
const TASK_ID: &str = "log-the-release";

/// The leading backticked command of a route string (`` `<cmd>`<tail> ``).
fn backticked<'a>(route: &'a str, what: &str) -> &'a str {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("`{what}`: the route carries a backticked command; got: {route}"))
}

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying stderr.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// Provision a `changelog` task carrying release `1-3-0` with a nested `Added`
/// change-group, returning the live fixture.
fn provision() -> Fixture {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = fixture_pack();
    init_repo(repo.path());

    let mut fx = Fixture {
        repo,
        home,
        pack,
        slug: String::new(),
    };

    ok_stdout(fx.run(&["setup"], None), "jigc setup");
    ok_stdout(
        fx.run(
            &["start", "--workflow", "log-change", "log the release"],
            None,
        ),
        "jigc start --workflow log-change",
    );

    let created = ok_stdout(
        fx.run(
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog",
    );
    fx.slug = created
        .strip_prefix("changelog:")
        .expect("created address is changelog:<slug>")
        .to_owned();

    let release = ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("changelog:{}#releases", fx.slug),
                "--title",
                "1-3-0",
            ],
            None,
        ),
        "add-item release",
    );
    ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Added",
            ],
            None,
        ),
        "add-item nested change-group",
    );
    // One live item in the top-level **enum** id-from section, so the `staged` cells'
    // emitted route has something real to reveal when it is run verbatim.
    ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("changelog:{}#staged", fx.slug),
                "--title",
                "Changed",
            ],
            None,
        ),
        "add-item staged change-group",
    );

    fx
}

/// One cell of the write-verb × miss-shape matrix.
struct Cell {
    /// What the cell is, for the assertion messages.
    what: &'static str,
    /// The `jigc` argv after the binary, with `{addr}` standing for the doc address.
    args: &'static [&'static str],
    /// Optional stdin payload (the `--from-file -` cells).
    stdin: Option<&'static [u8]>,
    /// The finding `code` the reject must carry.
    code: &'static str,
    /// How the emitted route is adjudicated — see [`RouteCheck`].
    route: RouteCheck,
}

/// What a cell's emitted route must be, and how it is proven followable.
enum RouteCheck {
    /// An **item-id miss**: the top showable section the route must show
    /// (`jigc doc show <type>:<slug>#<section> --task <id>`), run verbatim on the shared
    /// fixture — a read moves nothing.
    Show(&'static str),
    /// A **shape / declaredness** question, which the schema answers: `jigc doc schema
    /// <doctype>`, run verbatim on the shared fixture.
    Schema,
    /// A route that is itself a **write** (`{addr}` substituted): proven followable on a
    /// **private** fixture in the same state, because running it on the shared one would
    /// move the very bytes the next row diffs against.
    MutatingWrite(&'static str),
    /// A **collision** (M49): the minted item id is already taken, so the recovery is the
    /// same mint under a distinct `--slug`. Proven on a **private** fixture in the same
    /// state, and proven to *land* — a route that exits 0 without minting would pass a
    /// bare followability check while leaving the agent exactly where it was.
    Collides {
        /// The minted id the reject's message must name — which of the payload's items
        /// collided, the question the batch door left unanswered.
        minted: &'static str,
        /// The expected route argv (`{addr}` substituted).
        argv: &'static str,
        /// The section the landing is read back from …
        section: &'static str,
        /// … and the `{#id}` anchor the second item must carry there afterwards.
        landed: &'static str,
    },
    /// The collision column's **omitting context**: a destination whose `id-from` is an
    /// **enum**, where the heading IS the member and `--slug` is refused as an identity
    /// change. The collision must stay on its shipped human route — "edit it in place",
    /// which is the true answer when the id *is* the category — so this asserts the whole
    /// route string verbatim. A mechanical `--slug` route here would name a command that
    /// blocks when run, which is the un-followable route this suite exists to forbid.
    StaysHuman(&'static str),
}

/// The item [`provision`] mints in each showable section — what the emitted route, run
/// verbatim, must reveal. A property of the fixture (not of the cell), so a new row only
/// declares *which* section its route must strip to.
fn live_item(section: &str) -> &'static str {
    match section {
        "releases" => "1-3-0",
        "staged" => "Changed",
        other => panic!("no live item provisioned in section `{other}`"),
    }
}

/// **The axis.** Six item-id misses, the undeclared-section miss, and the one genuine
/// declared-shape defect the flip leaves standing.
const CELLS: &[Cell] = &[
    Cell {
        what: "set-slot at a nonexistent item",
        args: &[
            "doc",
            "set-slot",
            "{addr}#releases/9-9-9/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "remove-item at a nonexistent item",
        args: &["doc", "remove-item", "{addr}#releases/9-9-9"],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --value at a nonexistent item",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/9-9-9/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --unset at a nonexistent item, declared field",
        args: &["doc", "set-field", "{addr}#releases/9-9-9/link", "--unset"],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --unset at a nonexistent item, undeclared field",
        args: &["doc", "set-field", "{addr}#releases/9-9-9/bogus", "--unset"],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "retitle-item at a nonexistent item",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/9-9-9",
            "--title",
            "9.9.9",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "nested add-item under an absent parent item",
        args: &[
            "doc",
            "add-item",
            "{addr}#releases/9-9-9/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        // The **deep** strip at one of the newly-enriched verbs: a real parent release
        // (`1-3-0`), an absent nested change-group — the route must still strip past two
        // hops to the top *showable* section (the N2 pin, re-asserted at `retitle-item`).
        what: "retitle-item at a nonexistent nested item under a real parent",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/1-3-0/changes/no-such-group",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    // ---- The **id-from leaf** strip (M47 Increment 6, T3). Two CLI-side, schema-only
    // pre-checks sit in front of the engine's presence adjudication at two of the six
    // enriched dispatch sites — `write.id-from-field` (`set-field` at an id-from leaf)
    // and `write.identity-change` (`retitle-item` under an **enum** id-from). Both
    // assert a property of an item and hand back a route whose first verb blocks at an
    // item that was never minted, so item presence must outrank them: the same
    // shape → presence → leaf order the engine's own item-field doors already keep.
    Cell {
        what: "set-field --value at a nonexistent item's id-from leaf",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/9-9-9/version",
            "--value",
            "9.9.9",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --value at a nonexistent item's enum id-from leaf",
        args: &[
            "doc",
            "set-field",
            "{addr}#staged/no-such-group/category",
            "--value",
            "fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("staged"),
    },
    Cell {
        what: "set-field --value at a nonexistent nested item's enum id-from leaf",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/1-3-0/changes/no-such-group/category",
            "--value",
            "fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "retitle-item at a nonexistent item under an enum id-from",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#staged/no-such-group",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("staged"),
    },
    // ---- The **undeclared-section** column. The row below (`add-item` at a bare
    // undeclared section) was for a long time the column's only cell, so the matrix had
    // 1 of N there while the item-id-miss column had every verb — and five of the six
    // item-addressing verbs disagreed underneath it: `set-slot` / `remove-item` claimed
    // `write.not-present` (and handed back a `jigc doc show <doc>#<undeclared>` that
    // **exits 1** — a route that does not answer), `retitle-item` / nested `add-item`
    // claimed `write.wrong-shape`, and `set-field --unset` claimed `write.unknown-field`
    // about a field on an item in a section that does not exist. Shape outranks presence
    // and presence outranks the leaf (`design/write-commands.md` → Adjudication order),
    // so an undeclared section is `write.unknown-section` at **every** door, whose route
    // is the schema read that genuinely answers it.
    Cell {
        what: "set-slot at an item in an undeclared section",
        args: &[
            "doc",
            "set-slot",
            "{addr}#no-such-section/9-9-9/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at an item in an undeclared section",
        args: &["doc", "remove-item", "{addr}#no-such-section/9-9-9"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --value at an item in an undeclared section",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/9-9-9/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --unset at an item in an undeclared section",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/9-9-9/link",
            "--unset",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at an item in an undeclared section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#no-such-section/9-9-9",
            "--title",
            "9.9.9",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "nested add-item under an item in an undeclared section",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section/9-9-9/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item into an undeclared section",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    // ---- The **section-level** arm of the same column (M49 Increment 11, T3). Every row
    // above carries an **item hop**, so the address reaches a write door and the engine's
    // rank-1 `section_undeclared` answers it. These do not: `set-slot` at `#<undeclared>`
    // and `set-field` at `#<undeclared>/<leaf>` bottom out in the CLI's own target
    // resolvers, which searched the schema for a *slot* / a *field* and — finding neither,
    // because the section they would live in does not exist — refused with a bare
    // `{"error": "no slot addressed by …"}`: exit 1, **code-less, route-less, outside the
    // finding envelope**, so a driver keying on `(code, target)` sees nothing at all and an
    // agent is handed no recovery. `jigc doc author` reaches the same two resolvers through
    // its batch lowering, one arm each (a payload section's `set:` key is a field hop; the
    // section's own id is its slot, key-less), so it produced the bare form twice more. The
    // miss is identical to the rows above — the schema declares no such section — so it
    // earns the same `write.unknown-section` and the same `jigc doc schema <doctype>` read,
    // asked at rank 1 in the resolver rather than inferred from a missing leaf
    // (`design/validation.md` → The `write.*` route split, whose universal this closes —
    // M47 increment 6, advisory 2).
    Cell {
        what: "set-slot at an undeclared section (no item hop)",
        args: &[
            "doc",
            "set-slot",
            "{addr}#no-such-section",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --value at a field in an undeclared section (no item hop)",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --unset at a field in an undeclared section (no item hop)",
        args: &["doc", "set-field", "{addr}#no-such-section/link", "--unset"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        // The batch door's **field** arm: a payload `set:` key under an undeclared section
        // lowers to `#<undeclared>/<key>`, the same address the per-leaf `set-field` above
        // carries — and `author.rs`'s own lowering comment already claimed the engine's
        // `write.unknown-section` handled it.
        what: "doc author batch naming an undeclared section carrying a field",
        args: &["doc", "author", "changelog", "--from-file", "-"],
        stdin: Some(
            b"title: Changelog\nsections:\n  - id: no-such-section\n    set:\n      link: \"https://x\"\n",
        ),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        // The batch door's **slot** arm: the section's own id is its slot key, which
        // lowers key-less to `#<undeclared>` — the per-leaf `set-slot` address, through the
        // other resolver. Both arms are driven, because one fix reaching only one of them
        // leaves the same payload answering two ways.
        what: "doc author batch naming an undeclared section carrying a slot",
        args: &["doc", "author", "changelog", "--from-file", "-"],
        stdin: Some(
            b"title: Changelog\nsections:\n  - id: no-such-section\n    set:\n      no-such-section: \"<<A summary.>>\"\n",
        ),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    // ---- The **nested** arm of the same column (M49 Increment 8, T1). Every row above
    // gets the section wrong at the address's FIRST hop; these get it wrong at a later one
    // — `#releases/1-3-0/bogus/xyz…`, where `releases` is declared, `1-3-0` is live, and
    // the nested-section segment `bogus` is declared nowhere in the release block. It is
    // the same miss one level down, and it came back four different ways: `write.not-
    // present` at `set-slot` / `remove-item` (whose route is a `jigc doc show …#releases`
    // that exits 0 and answers nothing, because the item ids it lists are not what the
    // address got wrong), `write.wrong-shape` at `set-field --value` / `retitle-item` /
    // both `add-item` shapes, and `write.unknown-field` at `set-field --unset` — a field
    // question about an item in a nested section that does not exist. Shape outranks
    // presence outranks the leaf at every depth, so all seven converge on the shipped
    // `write.unknown-section` and its schema read (`design/command-output-contract.md` →
    // Evolution posture, the M49 paragraph authorizing the flip on the pinned key).
    Cell {
        what: "set-slot at an item in an undeclared nested section",
        args: &[
            "doc",
            "set-slot",
            "{addr}#releases/1-3-0/bogus/xyz/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at an item in an undeclared nested section",
        args: &["doc", "remove-item", "{addr}#releases/1-3-0/bogus/xyz"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --value at an item in an undeclared nested section",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/1-3-0/bogus/xyz/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --unset at an item in an undeclared nested section",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/1-3-0/bogus/xyz/link",
            "--unset",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at an item in an undeclared nested section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/1-3-0/bogus/xyz",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "nested add-item under an item in an undeclared nested section",
        args: &[
            "doc",
            "add-item",
            "{addr}#releases/1-3-0/bogus/xyz/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        // The nested dual of `add-item into an undeclared section` directly above: the
        // destination IS the undeclared nested section, with no trailing item hop, so the
        // segment names a section rather than a leaf and the top-level row's answer is the
        // right one one level down.
        what: "add-item into an undeclared nested section",
        args: &[
            "doc",
            "add-item",
            "{addr}#releases/1-3-0/bogus",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item into a non-repeatable section (the genuine shape question)",
        args: &["doc", "add-item", "{addr}#overview", "--title", "Added"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    // ---- The **address-shape** column (M50 Increment 9, T1). Every column above reaches a
    // write door; these thirteen never did. `add-item` resolves its destination through
    // `add_item_target` and `remove-item` / `retitle-item` theirs through
    // `remove_item_target`, and an address whose *shape* the resolver cannot map — a bare
    // `<type>:<slug>` with no fragment at all, an `add-item` at an item, a `remove-item` /
    // `retitle-item` at a section or at a leaf — fell out of the resolver as `None` and was
    // dressed by a bare `with_context`: exit 1 carrying `{"error": "no section addressed by
    // …"}`, **code-less, route-less, outside the finding envelope**, exactly the shape M49's
    // section-level arm closed one seam over. A driver keying on `(code, target)` saw
    // nothing and the agent was handed no recovery, at three of the five `doc` write verbs.
    //
    // The column has **two dimensions**, because the right answer is not a property of the
    // address shape alone: rank 1 asks whether the *leading* hop names a declared section at
    // all (`engine::write::undeclared_section_splice`, the same rank-1 predicate the six
    // item-addressing doors and M49's section-level resolvers already ask), so an
    // undeclared leading hop is `write.unknown-section` whatever shape the rest of the
    // address takes, and only a **declared** one leaves a genuine declared-shape defect:
    // `write.wrong-shape`, whose sentence names the form the verb takes and never claims an
    // absence. `write.not-present` earns no row here at all — nothing has been looked for in
    // the corpus yet, so naming an item absent would be a law-1 lie about a document the
    // resolver has not read.
    //
    // Both codes route the same `jigc doc schema <doctype>` read, which is the point: a
    // shape question and a declaredness question are both answered by the schema, and it is
    // the *code* a driver keys on that had to stop being absent.
    Cell {
        what: "add-item at a bare doc address (no section hop at all)",
        args: &["doc", "add-item", "{addr}", "--title", "Added"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item at an item in a declared section",
        args: &["doc", "add-item", "{addr}#releases/1-3-0", "--title", "Added"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item at an item in an undeclared section",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section/1-3-0",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a bare doc address (no item hop at all)",
        args: &["doc", "remove-item", "{addr}"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a declared section",
        args: &["doc", "remove-item", "{addr}#releases"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at an undeclared section",
        args: &["doc", "remove-item", "{addr}#no-such-section"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a leaf on a live item in a declared section",
        args: &["doc", "remove-item", "{addr}#releases/1-3-0/summary"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a leaf on an item in an undeclared section",
        args: &["doc", "remove-item", "{addr}#no-such-section/9-9-9/summary"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a bare doc address (no item hop at all)",
        args: &["doc", "retitle-item", "{addr}", "--title", "Fixed"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a declared section",
        args: &["doc", "retitle-item", "{addr}#releases", "--title", "Fixed"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at an undeclared section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#no-such-section",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a leaf on a live item in a declared section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/1-3-0/summary",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a leaf on an item in an undeclared section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#no-such-section/9-9-9/summary",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    // ---- The address-shape column's **section-level** arm (M50 Increment 9, T2). The
    // thirteen rows above are the *item*-addressing resolvers; these seven are the other
    // two, `field_target` and `slot_target` — the resolvers M49 reached for the
    // undeclared-**section** miss alone. Their remaining bare arms answered every other
    // unmappable shape with the same code-less, route-less `{"error": "no field addressed
    // by …"}` / `{"error": "no slot addressed by …"}` envelope: a bare `<type>:<slug>`
    // with no fragment at all at either verb, a single-hop `#<name>` matching no declared
    // field at BOTH `set-field` flags, and — at `set-slot` — a declared section that hosts
    // no prose slot, whether because it is repeatable or because it declares none, plus an
    // item address with no slot leaf. `jigc doc author` inherits every one of them through
    // `apply_leaf`'s `SetField` / `SetSlot` arms, so this is seven cells at three doors.
    //
    // The declaredness dimension crosses this arm **once**, not twice: the rank-1
    // `undeclared_section_guard` M49 installed already answers the undeclared leading hop
    // at both hop-bearing forms, so what is left here is the declared side alone —
    // `write.wrong-shape`, the genuine declared-shape defect. The one exception is the
    // **single-hop** `#<name>` at `set-field`, which names no section at any hop: it is a
    // field-id search across every declared section, so `write.unknown-section` would be a
    // law-1 lie about sections that all exist. It is an undeclared **leaf** —
    // `write.unknown-field`, the contract's one member for *the schema declares no such
    // leaf here* — and the bound `validation.md` carried over it (*"it keeps the
    // resolver's own no field addressed by sentence"*) is rewritten there rather than
    // left standing false.
    Cell {
        what: "set-field at a bare doc address (no field hop at all)",
        args: &["doc", "set-field", "{addr}", "--value", "x"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --value at a single hop no section declares as a field",
        args: &["doc", "set-field", "{addr}#no-such-leaf", "--value", "x"],
        stdin: None,
        code: "write.unknown-field",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --unset at a single hop no section declares as a field",
        args: &["doc", "set-field", "{addr}#no-such-leaf", "--unset"],
        stdin: None,
        code: "write.unknown-field",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a bare doc address (no section hop at all)",
        args: &["doc", "set-slot", "{addr}", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a repeatable section (no section-level prose slot)",
        args: &["doc", "set-slot", "{addr}#releases", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        // The ingredient cell: a **declared** section that hosts no prose slot at all —
        // the shipped fields-only shape (a header section), which no other row reaches.
        what: "set-slot at a declared section that declares no prose slot",
        args: &["doc", "set-slot", "{addr}#meta", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a live item with no slot leaf hop",
        args: &["doc", "set-slot", "{addr}#releases/1-3-0", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    // ---- The **cross-term** the widened fence bought (M50 Increment 9, T3). Every row
    // above holds one dimension fixed: the address-shape rows all sit under a *declared*
    // leading hop, and the undeclared-section rows all stop at three hops. These six are
    // the corner the two columns share — a four-hop-or-deeper address whose LEADING hop is
    // declared nowhere, at all five addressed verbs, plus `set-slot`'s `#<undeclared>/<hop>`
    // (the one two-hop undeclared cell no verb but `set-slot` had left undriven). Rank 1
    // answers before the id chain is walked at all, so every one of them is
    // `write.unknown-section` however deep the address goes — which is what they assert,
    // and what nothing asserted before the cross demanded a witness per cell
    // (`MISS_SHAPES`; the fence's direction 1).
    Cell {
        what: "add-item at a nested destination under an undeclared leading hop",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section/9-9-9/bogus/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a nested item under an undeclared leading hop",
        args: &["doc", "remove-item", "{addr}#no-such-section/9-9-9/bogus/xyz"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a nested item under an undeclared leading hop",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#no-such-section/9-9-9/bogus/xyz",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field at a nested field under an undeclared leading hop",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/9-9-9/bogus/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a nested slot under an undeclared leading hop",
        args: &[
            "doc",
            "set-slot",
            "{addr}#no-such-section/9-9-9/bogus/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"Prose.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a leaf under an undeclared section (no item hop)",
        args: &["doc", "set-slot", "{addr}#no-such-section/xyz", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    // ---- The **title-miss** column (M48 Increment 2, T2). The matrix's other columns are
    // all *address* misses; these two are the miss a **doc-minting** verb can make, and
    // until M48 neither was a miss at all — both exited 0. They split on whether the
    // identity moves, because the recovery does not: a call that would mint a DIFFERENT
    // identity beside the one this task holds is `write.identity-change` (a second
    // document, not a correction), while a call that lands on the SAME identity and
    // merely drops the title is identity-**stable** (`design/storage.md` → Identity) and
    // earns `write.title-ignored`. Both route at `jigc doc rename`, the in-task title
    // change — a route that is itself a write, hence `MutatingWrite`.
    Cell {
        what: "create a second identity while the gate's role is already bound",
        args: &["doc", "create", "changelog", "--title", "Release Log"],
        stdin: None,
        code: "write.identity-change",
        route: RouteCheck::MutatingWrite(
            "jigc doc rename {addr} --to 'Release Log' --task log-the-release",
        ),
    },
    Cell {
        what: "create at the held identity with a title that would be dropped",
        args: &["doc", "create", "changelog", "--title", "Changelog!"],
        stdin: None,
        code: "write.title-ignored",
        route: RouteCheck::MutatingWrite(
            "jigc doc rename {addr} --to 'Changelog!' --task log-the-release",
        ),
    },
    // ---- The **collision** column (M49 Increment 5, T5). Not a miss: the section is
    // declared, the shape is right, and the id the title mints is simply taken — the
    // fixture already holds release `1-3-0`, and the distinct title `1.3.0` mints the same
    // id. The shipped route said *"edit it in place"*, which is the answer for a
    // correction and the wrong answer for a second, distinct entry — and is not runnable,
    // so from inside a `doc author` payload (rejected whole, nothing staged) it terminated
    // nowhere. Both doors carry the same row because both funnel through the one
    // `apply_add_item_target` enrichment; a fix at only one of them leaves the other lying.
    Cell {
        what: "add-item at a title that mints a taken id",
        args: &["doc", "add-item", "{addr}#releases", "--title", "1.3.0"],
        stdin: None,
        code: "write.already-present",
        route: RouteCheck::Collides {
            minted: "1-3-0",
            argv: "jigc doc add-item {addr}#releases --title 1.3.0 --slug 1-3-0-2 \
                   --task log-the-release",
            section: "releases",
            landed: "1-3-0-2",
        },
    },
    Cell {
        what: "doc author batch carrying an item title that mints a taken id",
        args: &["doc", "author", "changelog", "--from-file", "-"],
        stdin: Some(
            b"title: Changelog\nsections:\n  - id: releases\n    items:\n      - title: \"1.3.0\"\n",
        ),
        code: "write.already-present",
        route: RouteCheck::Collides {
            minted: "1-3-0",
            argv: "jigc doc add-item {addr}#releases --title 1.3.0 --slug 1-3-0-2 \
                   --task log-the-release",
            section: "releases",
            landed: "1-3-0-2",
        },
    },
    Cell {
        // The omitting context, driven rather than reasoned about: `#staged`'s `id-from`
        // is an **enum**, so the same slug-alike collision (`Changed` is live; `changed`
        // mints the same id) must NOT be handed a `--slug` — that override is refused as
        // an identity change, so the route would block when run.
        what: "add-item at a taken enum id-from member (the omitting context)",
        args: &["doc", "add-item", "{addr}#staged", "--title", "changed"],
        stdin: None,
        code: "write.already-present",
        route: RouteCheck::StaysHuman(
            "the target already exists — edit it in place (`set-field`/`set-slot`) \
             instead of re-creating it",
        ),
    },
];

#[test]
fn every_write_miss_names_its_own_miss_and_routes_the_recovery() {
    let fx = provision();
    let addr = format!("changelog:{}", fx.slug);
    let staged = fx.staged();
    let before = fs::read_to_string(&staged).expect("read the staged changelog");
    // Every cell is adjudicated, then the whole axis is reported at once — a per-cell
    // panic would hide the rest of the matrix behind the first broken row.
    let mut broken: Vec<String> = Vec::new();

    for cell in CELLS {
        let args: Vec<String> = cell
            .args
            .iter()
            .map(|arg| arg.replace("{addr}", &addr))
            .chain(["--format".to_owned(), "json".to_owned()])
            .collect();
        let out = fx.run(
            &args.iter().map(String::as_str).collect::<Vec<&str>>(),
            cell.stdin,
        );

        assert!(
            !out.status.success(),
            "`{}` must block (non-zero exit); stdout:\n{}\nstderr:\n{}",
            cell.what,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        let report: serde_json::Value = serde_json::from_str(stderr.trim())
            .unwrap_or_else(|e| panic!("`{}` stderr is JSON: {e}; got:\n{stderr}", cell.what));
        let got = report["findings"][0]["code"].as_str().unwrap_or("<absent>");
        let named_its_miss = got == cell.code;

        // **The other half of the pinned key.** A driver keys on `(code, target)`, so a
        // cell that names its miss correctly while pointing the key at some *other*
        // address is still unusable — and the nested column's whole subject is an address
        // the six doors read to different depths. Asserted as a property of the matrix
        // rather than as a per-row column: wherever the cell's argv carries a write
        // address, the finding's key target and its located address are **that** address,
        // verbatim and in full.
        if let Some(addressed) = args.iter().find(|arg| arg.contains('#')) {
            for (half, got) in [
                ("key.target", &report["findings"][0]["key"]["target"]),
                (
                    "location.address",
                    &report["findings"][0]["location"]["address"],
                ),
            ] {
                assert_eq!(
                    got.as_str(),
                    Some(addressed.as_str()),
                    "`{}`: {half} names the full write address",
                    cell.what,
                );
            }
        }

        // A **collision** must name the colliding minted id in its own message: an
        // `already-present` inside a batch is otherwise silent about WHICH of the
        // payload's items collided, and the whole payload is what got rejected.
        if let RouteCheck::Collides { minted, .. } = &cell.route {
            let message = report["findings"][0]["message"].as_str().unwrap_or("");
            assert!(
                message.contains(&format!("{minted:?}")),
                "`{}`: the reject names the colliding minted id {minted:?}; got:\n{message}",
                cell.what,
            );
        }
        if !named_its_miss {
            broken.push(format!(
                "  {}: expected `{}`, got `{}` (route: {})",
                cell.what,
                cell.code,
                got,
                report["findings"][0]["route"].as_str().unwrap_or("<none>"),
            ));
        }

        // **The route column.** An item-id miss must hand back the containing section's
        // `doc show`, with the doc's real slug and the resolved task id substituted — the
        // engine's `<address>` / `<task-id>` fallback reaching an agent is the dead end
        // this axis exists to close. Only adjudicated once the cell named its own miss:
        // a wrong-code cell has already been recorded, and its foreign route shape must
        // not panic the run and hide the rest of the matrix.
        //
        // The complement of the item-id column: a **shape** cell must route at the schema
        // read that answers a shape question — never at a `jigc doc show` of the very
        // section the address got wrong, which is the un-followable route (it exits 1) an
        // unranked declaredness check hands back. The CLI seam resolves the `<doctype>`
        // placeholder, so that route is run **verbatim** too: it must exit 0 and name the
        // doctype's real sections. And a **title-miss** cell's route is itself a write, so
        // it is proven followable on a private fixture in the same state (running it here
        // would move the bytes the next row diffs against).
        if named_its_miss {
            let route = report["findings"][0]["route"]
                .as_str()
                .unwrap_or_else(|| panic!("`{}` carries a route; got:\n{stderr}", cell.what));
            // The one non-command row shape: the omitting context, whose route must stay
            // the shipped human direction. Adjudicated on the WHOLE route string, before
            // `backticked` — which would panic on it, since a human route carries no
            // leading backticked command (that panic is exactly the red this column
            // opened with, and it must not fire on the row that is correct as it stands).
            if let RouteCheck::StaysHuman(expected) = cell.route {
                if route != expected {
                    broken.push(format!(
                        "  {}: expected the human route `{}`, got `{}`",
                        cell.what, expected, route
                    ));
                }
                let after = fs::read_to_string(&staged).expect("read the staged changelog");
                assert_eq!(
                    after, before,
                    "`{}` persisted nothing — the staged bytes are unchanged",
                    cell.what,
                );
                continue;
            }
            let cmd = backticked(route, cell.what);
            let expected = match cell.route {
                RouteCheck::Show(section) => {
                    format!("jigc doc show {addr}#{section} --task {TASK_ID}")
                }
                RouteCheck::Schema => "jigc doc schema changelog".to_owned(),
                RouteCheck::MutatingWrite(argv) => argv.replace("{addr}", &addr),
                RouteCheck::Collides { argv, .. } => argv.replace("{addr}", &addr),
                RouteCheck::StaysHuman(_) => unreachable!("adjudicated above"),
            };
            if cmd != expected {
                broken.push(format!(
                    "  {}: expected route `{}`, got `{}`",
                    cell.what, expected, cmd
                ));
            } else {
                match cell.route {
                    RouteCheck::Show(section) => {
                        // Followability: the emitted argv, run **verbatim**, must exit 0
                        // and print the section's live item ids — the address the agent
                        // should have used.
                        let shown = fx.run_route(cmd, cell.what);
                        let live = live_item(section);
                        assert!(
                            shown.contains(live),
                            "`{}`: the emitted route reveals the section's live item ids; got:\n{shown}",
                            cell.what,
                        );
                    }
                    RouteCheck::Schema => {
                        let shown = fx.run_route(cmd, cell.what);
                        assert!(
                            shown.contains("releases"),
                            "`{}`: the emitted route reveals the doctype's declared sections; got:\n{shown}",
                            cell.what,
                        );
                    }
                    RouteCheck::MutatingWrite(_) => {
                        provision().run_route(cmd, cell.what);
                    }
                    RouteCheck::Collides {
                        section, landed, ..
                    } => {
                        assert!(
                            cmd.contains(" --slug "),
                            "`{}`: the collision route names `--slug` — the flag that \
                             decouples the id from the title; got `{cmd}`",
                            cell.what,
                        );
                        // Run verbatim on a private fixture in the same state, then read
                        // the section back: the second item must have LANDED beside the
                        // first, not merely exited 0.
                        let private = provision();
                        private.run_route(cmd, cell.what);
                        let shown = private.run_route(
                            &format!("jigc doc show {addr}#{section} --task {TASK_ID}"),
                            cell.what,
                        );
                        for anchor in [live_item(section), landed] {
                            assert!(
                                shown.contains(&format!("{{#{anchor}}}")),
                                "`{}`: after the route ran, `{{#{anchor}}}` stands in \
                                 `#{section}`; got:\n{shown}",
                                cell.what,
                            );
                        }
                    }
                    RouteCheck::StaysHuman(_) => unreachable!("adjudicated above"),
                }
            }
        }

        let after = fs::read_to_string(&staged).expect("read the staged changelog");
        assert_eq!(
            after, before,
            "`{}` persisted nothing — the staged bytes are unchanged",
            cell.what,
        );
    }

    assert!(
        broken.is_empty(),
        "{} defect(s) across {} cells (the miss named, or the route handed back):\n{}",
        broken.len(),
        CELLS.len(),
        broken.join("\n"),
    );
}

/// **How much of a fragment an address carries** — the manufactured half of the fence's
/// cross.
///
/// # This is a MANUFACTURED SHAPE SPACE, not a registry enumeration
///
/// The idiom is `crates/cli/tests/support/shape_space.rs`'s, and the reason is the same
/// one. `engine::address::Fragment` supplies the *hop counts* a fragment can take, and
/// nothing more: which of those shapes a given resolver **rejects** is a property of four
/// hand-written `match` arms (`add_item_target` · `remove_item_target` · `field_target` ·
/// `slot_target`), and **declaredness is a property of a fixture**, not of any table in
/// the codebase — no registry anywhere knows that `releases` is declared and
/// `no-such-section` is not. A fence that looped a registry here would enumerate the
/// hop counts, find no verdict attached to any of them, and prove nothing. So the cross
/// is **built**: [`Shape`] × [`Leading`] is written down, [`MISS_SHAPES`] carries the
/// verdict per cell, and [`address_cell`] computes a `CELLS` row's coordinates from its
/// own argv so a row cannot be filed under a cell it does not drive.
///
/// The **verb** half is not manufactured and must not be: it is derived from
/// [`VERB_KINDS`], the table already fenced against the clap tree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Shape {
    /// `<type>:<slug>` — no fragment at all.
    Bare,
    /// `#a` — one hop.
    Unit,
    /// `#a/b` — two hops (`Fragment::UnitLeaf`; the parser mints no `UnitItem`).
    UnitLeaf,
    /// `#a/b/c` — three hops.
    UnitItemLeaf,
    /// `#a/b/…` — four hops or more (`Fragment::Deep`).
    Deep,
}

impl Shape {
    /// The shape a fragment of `hops` hops takes (`0` = no fragment at all) — the one
    /// place the hop-count → shape mapping is written, mirroring
    /// `engine::address::Fragment`'s own structural split.
    fn of_hops(hops: usize) -> Shape {
        match hops {
            0 => Shape::Bare,
            1 => Shape::Unit,
            2 => Shape::UnitLeaf,
            3 => Shape::UnitItemLeaf,
            _ => Shape::Deep,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Shape::Bare => "bare `<type>:<slug>`",
            Shape::Unit => "`#a`",
            Shape::UnitLeaf => "`#a/b`",
            Shape::UnitItemLeaf => "`#a/b/c`",
            Shape::Deep => "`#a/b/…` (≥4 hops)",
        }
    }
}

/// **Whether the address's leading hop names a section the schema declares** — the second
/// dimension, and the one that decides the *code*: rank 1 asks exactly this question at
/// every door, so an undeclared leading hop is `write.unknown-section` whatever shape the
/// rest of the address takes, and only over a declared one is what is left a genuine
/// declared-shape defect.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Leading {
    /// The address carries no fragment, so there is no leading hop to judge.
    Absent,
    /// The leading hop names a section [`CHANGELOG_SCHEMA`] declares.
    Declared,
    /// It names one the schema declares nowhere.
    Undeclared,
    /// **The declared bound, as a coordinate.** The form reads its leading hop as
    /// something other than a section, so its declaredness is not a question the reject
    /// may answer: `set-field`'s single-hop `#<field>` searches every declared section for
    /// a field of that id, and minting `write.unknown-section` there would be a law-1 lie
    /// about sections that all exist. It is an undeclared **leaf**.
    NotASectionHop,
}

impl Leading {
    fn label(self) -> &'static str {
        match self {
            Leading::Absent => "no leading hop",
            Leading::Declared => "declared",
            Leading::Undeclared => "undeclared",
            Leading::NotASectionHop => "not a section hop",
        }
    }
}

/// The sections [`CHANGELOG_SCHEMA`] declares — what makes a leading hop *declared* for
/// [`address_cell`]. Asserted against the schema text itself in the fence, so a section
/// renamed there cannot leave this list quietly wrong.
const FIXTURE_SECTIONS: &[&str] = &["meta", "overview", "staged", "releases"];

/// The three codes that answer an **address-shape or declaredness** question — the
/// columns this fence governs.
///
/// `write.not-present` is deliberately not among them: it answers **presence**, which is
/// the M47 column and a different question (nothing has been looked for in the corpus when
/// a shape reject prints). Nor does the fence govern the **trailing-leaf** declaredness
/// axis — an undeclared leaf under a declared section and a live item — which is
/// `crates/cli/tests/undeclared_address_writes.rs`'s subject; `write.unknown-field`
/// appears here only where the *leading* hop is what the form got wrong.
const SHAPE_CODES: &[&str] = &[
    "write.unknown-section",
    "write.wrong-shape",
    "write.unknown-field",
];

/// One cell of the **address-shape × leading-hop-declaredness** cross, per verb: a
/// coordinate the verb **rejects**, and the code it must earn there.
///
/// A cell may be reached by more than one row of [`CELLS`] (two witnesses, one verdict),
/// and a coordinate may carry more than one row when the verdict is not a function of the
/// coordinate alone — `#a/b/c` under a *declared* leading hop is `write.unknown-section`
/// when a later hop is declared nowhere and `write.not-present` when the item was never
/// minted, and only the first is this fence's business. That is why the row carries the
/// **code**: the coordinate names the question, the code names the answer.
struct MissShape {
    /// The verb, as it appears in [`VERB_KINDS`].
    verb: &'static [&'static str],
    shape: Shape,
    leading: Leading,
    /// The code a `CELLS` row at this coordinate must earn.
    code: &'static str,
    /// Why the verb rejects here — the sentence that makes the row a claim rather than a
    /// restatement of the assertion.
    why: &'static str,
}

/// **The cross.** Thirty-nine cells over the five `doc` write verbs that take an address
/// argument — every `(shape, leading)` coordinate at which the verb refuses on shape or
/// declaredness grounds, each carrying the code it must earn.
///
/// The old fence had the **verb** axis alone and was satisfied by any one
/// `write.unknown-section` row per verb, which is how eighteen cells stayed invisible
/// under a green fence for a whole wave. Adding the cross is what makes a cell a row to
/// add rather than a rediscovery.
const MISS_SHAPES: &[MissShape] = &[
    // ---- `jigc doc add-item` — `add_item_target` maps `#section` and the nested
    // `#section/item/…/nested-section`; every other shape names no mint destination.
    MissShape {
        verb: &["doc", "add-item"],
        shape: Shape::Bare,
        leading: Leading::Absent,
        code: "write.wrong-shape",
        why: "a mint needs a destination section; a bare doc address names none",
    },
    MissShape {
        verb: &["doc", "add-item"],
        shape: Shape::Unit,
        leading: Leading::Declared,
        code: "write.wrong-shape",
        why: "the form is right, so only a non-repeatable destination refuses — the \
              genuine shape question the M47 flip left standing",
    },
    MissShape {
        verb: &["doc", "add-item"],
        shape: Shape::Unit,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1: no section of that id is declared at all",
    },
    MissShape {
        verb: &["doc", "add-item"],
        shape: Shape::UnitLeaf,
        leading: Leading::Declared,
        code: "write.wrong-shape",
        why: "`#section/item` addresses an item, which is not a destination to mint into",
    },
    MissShape {
        verb: &["doc", "add-item"],
        shape: Shape::UnitLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1 outranks the form question: the section is wrong first",
    },
    MissShape {
        verb: &["doc", "add-item"],
        shape: Shape::UnitItemLeaf,
        leading: Leading::Declared,
        code: "write.unknown-section",
        why: "the trailing hop names the nested repeatable to mint into, and it is \
              declared nowhere in the parent block (M49's nested arm)",
    },
    MissShape {
        verb: &["doc", "add-item"],
        shape: Shape::UnitItemLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1, at the depth the nested mint reads",
    },
    MissShape {
        verb: &["doc", "add-item"],
        shape: Shape::Deep,
        leading: Leading::Declared,
        code: "write.unknown-section",
        why: "a nested-section segment below the parent chain, declared nowhere",
    },
    MissShape {
        verb: &["doc", "add-item"],
        shape: Shape::Deep,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1 answers before the chain is walked, however deep the address goes",
    },
    // ---- `jigc doc remove-item` — `remove_item_target` maps `#section/item` and the
    // nested id chain; a section, a leaf and a bare address name no item.
    MissShape {
        verb: &["doc", "remove-item"],
        shape: Shape::Bare,
        leading: Leading::Absent,
        code: "write.wrong-shape",
        why: "a bare doc address names no item to remove",
    },
    MissShape {
        verb: &["doc", "remove-item"],
        shape: Shape::Unit,
        leading: Leading::Declared,
        code: "write.wrong-shape",
        why: "a section is not an item — and nothing has been looked for yet, so an \
              absence claim here would be a lie about a document unread",
    },
    MissShape {
        verb: &["doc", "remove-item"],
        shape: Shape::Unit,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1: the section is wrong before the form is",
    },
    MissShape {
        verb: &["doc", "remove-item"],
        shape: Shape::UnitLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "the one item-addressing shape the resolver accepts, refused at rank 1 \
              because the section it names is declared nowhere",
    },
    MissShape {
        verb: &["doc", "remove-item"],
        shape: Shape::UnitItemLeaf,
        leading: Leading::Declared,
        code: "write.wrong-shape",
        why: "`#section/item/leaf` addresses a leaf, not the item that holds it",
    },
    MissShape {
        verb: &["doc", "remove-item"],
        shape: Shape::UnitItemLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1 outranks the form question",
    },
    MissShape {
        verb: &["doc", "remove-item"],
        shape: Shape::Deep,
        leading: Leading::Declared,
        code: "write.unknown-section",
        why: "a nested-section segment in the id chain, declared nowhere (M49)",
    },
    MissShape {
        verb: &["doc", "remove-item"],
        shape: Shape::Deep,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1, however deep the chain",
    },
    // ---- `jigc doc retitle-item` — the same resolver as `remove-item`, so the same
    // eight cells; the two verbs are one axis and must not answer differently.
    MissShape {
        verb: &["doc", "retitle-item"],
        shape: Shape::Bare,
        leading: Leading::Absent,
        code: "write.wrong-shape",
        why: "a bare doc address names no item to retitle (`jigc doc rename` is the \
              whole-doc verb)",
    },
    MissShape {
        verb: &["doc", "retitle-item"],
        shape: Shape::Unit,
        leading: Leading::Declared,
        code: "write.wrong-shape",
        why: "a section is not an item",
    },
    MissShape {
        verb: &["doc", "retitle-item"],
        shape: Shape::Unit,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1",
    },
    MissShape {
        verb: &["doc", "retitle-item"],
        shape: Shape::UnitLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "the accepted item shape, refused at rank 1 on the section",
    },
    MissShape {
        verb: &["doc", "retitle-item"],
        shape: Shape::UnitItemLeaf,
        leading: Leading::Declared,
        code: "write.wrong-shape",
        why: "a leaf is not an item",
    },
    MissShape {
        verb: &["doc", "retitle-item"],
        shape: Shape::UnitItemLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1 outranks the form question",
    },
    MissShape {
        verb: &["doc", "retitle-item"],
        shape: Shape::Deep,
        leading: Leading::Declared,
        code: "write.unknown-section",
        why: "a nested-section segment in the id chain, declared nowhere (M49)",
    },
    MissShape {
        verb: &["doc", "retitle-item"],
        shape: Shape::Deep,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1, however deep the chain",
    },
    // ---- `jigc doc set-field` — `field_target` maps the single-hop field search, the
    // two-hop `#section/field` and the item-leaf forms; the single hop is the one cell on
    // the whole cross whose leading hop is not read as a section at all.
    MissShape {
        verb: &["doc", "set-field"],
        shape: Shape::Bare,
        leading: Leading::Absent,
        code: "write.wrong-shape",
        why: "a bare doc address names no field leaf",
    },
    MissShape {
        verb: &["doc", "set-field"],
        shape: Shape::Unit,
        leading: Leading::NotASectionHop,
        code: "write.unknown-field",
        why: "the single hop is a field-id search across every declared section, so it \
              names no section to have got wrong — what it got wrong is a leaf",
    },
    MissShape {
        verb: &["doc", "set-field"],
        shape: Shape::UnitLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1: there is no block on which the addressed field could be declared",
    },
    MissShape {
        verb: &["doc", "set-field"],
        shape: Shape::UnitItemLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1, at the item-field depth",
    },
    MissShape {
        verb: &["doc", "set-field"],
        shape: Shape::Deep,
        leading: Leading::Declared,
        code: "write.unknown-section",
        why: "a nested-section segment in the id chain, declared nowhere (M49)",
    },
    MissShape {
        verb: &["doc", "set-field"],
        shape: Shape::Deep,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1, however deep the chain",
    },
    // ---- `jigc doc set-slot` — `slot_target` maps `#section` over a simple section that
    // declares a slot, and the per-item `#section/item/…/slot`; the declared-but-slotless
    // and repeatable cells are the shapes no other row in this fixture reaches.
    MissShape {
        verb: &["doc", "set-slot"],
        shape: Shape::Bare,
        leading: Leading::Absent,
        code: "write.wrong-shape",
        why: "a bare doc address names no prose slot",
    },
    MissShape {
        verb: &["doc", "set-slot"],
        shape: Shape::Unit,
        leading: Leading::Declared,
        code: "write.wrong-shape",
        why: "the form is right and the section exists, so what refuses is the declared \
              shape it landed on — a repeatable section, or a fields-only one that hosts \
              no prose slot at all",
    },
    MissShape {
        verb: &["doc", "set-slot"],
        shape: Shape::Unit,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1, at the form the batch lowering gives a payload section's own slot",
    },
    MissShape {
        verb: &["doc", "set-slot"],
        shape: Shape::UnitLeaf,
        leading: Leading::Declared,
        code: "write.wrong-shape",
        why: "over a repeatable section the second hop is an item id with no slot leaf, \
              so the address stops short of any prose",
    },
    MissShape {
        verb: &["doc", "set-slot"],
        shape: Shape::UnitLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1 runs before the section's own shape is consulted",
    },
    MissShape {
        verb: &["doc", "set-slot"],
        shape: Shape::UnitItemLeaf,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1, at the per-item slot depth",
    },
    MissShape {
        verb: &["doc", "set-slot"],
        shape: Shape::Deep,
        leading: Leading::Declared,
        code: "write.unknown-section",
        why: "a nested-section segment in the id chain, declared nowhere (M49)",
    },
    MissShape {
        verb: &["doc", "set-slot"],
        shape: Shape::Deep,
        leading: Leading::Undeclared,
        code: "write.unknown-section",
        why: "rank 1, however deep the chain",
    },
];

/// The `(shape, leading)` coordinate a [`CELLS`] row drives, computed from **its own
/// argv** — so a row cannot be filed under a cell it does not reach. `None` for a row that
/// carries no `{addr}` argument at all (the batch and doc-minting rows).
fn address_cell(cell: &Cell) -> Option<(Shape, Leading)> {
    let addressed = cell.args.iter().find(|arg| arg.contains("{addr}"))?;
    let hops: Vec<&str> = match addressed.split_once('#') {
        None => Vec::new(),
        Some((_, fragment)) => fragment.split('/').collect(),
    };
    let shape = Shape::of_hops(hops.len());
    let verb = &cell.args[..2];
    let leading = match shape {
        Shape::Bare => Leading::Absent,
        // The declared bound, applied where it is true: `set-field`'s single hop is a
        // field-id search, never a section hop, so its declaredness is not a coordinate.
        Shape::Unit if verb == ["doc", "set-field"] => Leading::NotASectionHop,
        _ if FIXTURE_SECTIONS.contains(&hops[0]) => Leading::Declared,
        _ => Leading::Undeclared,
    };
    Some((shape, leading))
}

/// The `doc` write verbs whose address carries **no leading section hop to judge**, so the
/// cross can hold no cell for them — each with the reason it cannot, stated rather than
/// left as an absence, and re-dispositioned against the widened axis.
const NO_SECTION_HOP: &[(&[&str], &str)] = &[
    (
        &["doc", "create"],
        "takes a bare doctype id, never an address — its own misses are the title-miss \
         column, which is a different question entirely",
    ),
    (
        &["doc", "rename"],
        "addresses a whole doc and refuses any `#fragment` outright, so no fragment of \
         any shape reaches a resolver. Driven at HEAD, that refusal is routed and \
         followable (`route: run `jigc doc rename <doc> --to <title>``) but carries NO \
         code and rides the `{\"error\": …}` envelope — the last bare-sentence write \
         reject on the `doc` surface, and Increment 10's, not this cross's",
    ),
];

/// The `doc` write verbs that reach the cross **through another verb's resolver**, so
/// their cells are covered by lowering rather than by rows of their own — the third
/// disposition, added when the axis widened past *does this verb drive one
/// undeclared-section row*.
const COVERED_BY_LOWERING: &[(&[&str], &str)] = &[(
    &["doc", "author"],
    "carries no address argv at all: a batch payload's section ids and `set:` keys are \
     lowered onto the very resolvers the five addressed verbs call (`apply_leaf`'s \
     `SetSlot` / `SetField` arms, `apply_add_item_target`), so its coordinates are those \
     verbs' coordinates one door up. Its `CELLS` rows are the witness that the lowering \
     carries the code through rather than re-deriving it",
)];

/// **The fence over the cross.** The rows of [`CELLS`] are cells of a matrix whose other
/// axis is the *write-verb surface*, and that surface is code — [`VERB_KINDS`], the table
/// already fenced against the clap tree. So the verb axis is derived from it rather than
/// hand-counted, and every `doc` write verb takes exactly one of three dispositions:
/// it carries [`MISS_SHAPES`] rows, or it is named in [`NO_SECTION_HOP`] with the reason
/// it takes no address, or it is named in [`COVERED_BY_LOWERING`] with the resolver its
/// cells reach through.
///
/// The predecessor of this fence had the verb axis **alone**, and was satisfied by any one
/// `write.unknown-section` row per verb — which is exactly how eighteen address-shape
/// cells stayed invisible underneath a green fence until M50. So it gains the cross, and
/// the cross totals against `CELLS` **in both directions**: no row of [`MISS_SHAPES`] may
/// claim a cell nothing drives, and no `CELLS` row carrying a [`SHAPE_CODES`] verdict at
/// an address may sit outside the table.
#[test]
fn every_doc_write_verb_answers_its_address_shape_and_declaredness_cells() {
    // The fixture's declared section set is what makes a leading hop *declared*, and it
    // is read from a `const` — so pin it against the schema text it claims to describe.
    for section in FIXTURE_SECTIONS {
        assert!(
            CHANGELOG_SCHEMA.contains(&format!("- id: {section}\n")),
            "`FIXTURE_SECTIONS` names {section:?}, which `CHANGELOG_SCHEMA` does not \
             declare as a section",
        );
    }
    assert_eq!(
        CHANGELOG_SCHEMA.matches("\n  - id: ").count(),
        FIXTURE_SECTIONS.len(),
        "`FIXTURE_SECTIONS` lists every top-level section `CHANGELOG_SCHEMA` declares",
    );

    let surface: Vec<&[&str]> = VERB_KINDS
        .iter()
        .filter(|(path, kind)| path.first() == Some(&"doc") && *kind == VerbKind::Write)
        .map(|(path, _)| *path)
        .collect();

    // Every `doc` write verb takes exactly one disposition.
    for verb in &surface {
        let crossed = MISS_SHAPES.iter().any(|row| row.verb == *verb);
        let addressless = NO_SECTION_HOP.iter().any(|(path, _)| path == verb);
        let lowered = COVERED_BY_LOWERING.iter().any(|(path, _)| path == verb);
        assert_eq!(
            [crossed, addressless, lowered]
                .iter()
                .filter(|b| **b)
                .count(),
            1,
            "`jigc {}`: a `doc` write verb carries `MISS_SHAPES` rows, or states in \
             `NO_SECTION_HOP` why it takes no address, or states in \
             `COVERED_BY_LOWERING` which resolver its cells reach through — exactly one \
             (crossed: {crossed}, addressless: {addressless}, lowered: {lowered})",
            verb.join(" "),
        );
    }
    for (verb, _) in NO_SECTION_HOP.iter().chain(COVERED_BY_LOWERING) {
        assert!(
            surface.contains(verb),
            "`jigc {}` is dispositioned here but is not a `doc` write verb in \
             `VERB_KINDS`",
            verb.join(" "),
        );
    }
    for row in MISS_SHAPES {
        assert!(
            surface.contains(&row.verb),
            "`MISS_SHAPES` carries a row for `jigc {}`, which is not a `doc` write verb \
             in `VERB_KINDS`",
            row.verb.join(" "),
        );
        assert!(
            SHAPE_CODES.contains(&row.code),
            "`jigc {}` at {} / {}: `{}` is not an address-shape verdict — this cross \
             governs {SHAPE_CODES:?} and leaves presence and trailing-leaf declaredness \
             to their own axes",
            row.verb.join(" "),
            row.shape.label(),
            row.leading.label(),
            row.code,
        );
    }

    // The witness `COVERED_BY_LOWERING` claims: the batch verb drives cells of this cross
    // without an address argv of its own.
    for (verb, _) in COVERED_BY_LOWERING {
        let rows: Vec<&Cell> = CELLS
            .iter()
            .filter(|cell| cell.args.starts_with(verb))
            .collect();
        assert!(
            rows.iter().any(|cell| SHAPE_CODES.contains(&cell.code)),
            "`jigc {}` is covered by lowering, so `CELLS` must witness the lowering \
             carrying an address-shape verdict through",
            verb.join(" "),
        );
        assert!(
            rows.iter().all(|cell| address_cell(cell).is_none()),
            "`jigc {}` is covered by lowering because it carries no address argv — a row \
             that does carry one belongs in `MISS_SHAPES`",
            verb.join(" "),
        );
    }

    // **Direction 1 — no row claims a cell nothing drives.**
    let mut unwitnessed: Vec<String> = Vec::new();
    for row in MISS_SHAPES {
        let driven = CELLS.iter().any(|cell| {
            cell.args.starts_with(row.verb)
                && cell.code == row.code
                && address_cell(cell) == Some((row.shape, row.leading))
        });
        if !driven {
            unwitnessed.push(format!(
                "  `jigc {}` at {} / {} → `{}` ({}) — no `CELLS` row drives it",
                row.verb.join(" "),
                row.shape.label(),
                row.leading.label(),
                row.code,
                row.why,
            ));
        }
    }

    // **Direction 2 — no driven cell sits outside the table.**
    let mut untabled: Vec<String> = Vec::new();
    for cell in CELLS {
        if !SHAPE_CODES.contains(&cell.code) {
            continue;
        }
        let Some((shape, leading)) = address_cell(cell) else {
            continue;
        };
        let tabled = MISS_SHAPES.iter().any(|row| {
            cell.args.starts_with(row.verb)
                && row.code == cell.code
                && (row.shape, row.leading) == (shape, leading)
        });
        if !tabled {
            untabled.push(format!(
                "  {} — {} / {} → `{}` is driven but is in no `MISS_SHAPES` row",
                cell.what,
                shape.label(),
                leading.label(),
                cell.code,
            ));
        }
    }

    assert!(
        unwitnessed.is_empty() && untabled.is_empty(),
        "the address-shape × declaredness cross does not total against `CELLS` \
         ({} row(s) unwitnessed, {} cell(s) untabled):\n{}",
        unwitnessed.len(),
        untabled.len(),
        unwitnessed
            .into_iter()
            .chain(untabled)
            .collect::<Vec<String>>()
            .join("\n"),
    );
}

/// **The envelope property, over the whole matrix.** Naming the miss and routing the
/// recovery are both properties *of a finding*, and the defect this increment closes was
/// one seam earlier: a reject that never became a finding at all. A bare `with_context`
/// dressed as `{"error": …}` exits 1 the same way, prints a sentence the same way, and is
/// **outside the contract** — no `schema_version`, no `findings` array, so a driver keying
/// on `(code, target)` sees nothing and every fence above it is vacuous.
///
/// So it is asserted as a property of every row rather than as a column of any of them:
/// each `CELLS` row's `--format json` reject carries the pinned envelope
/// (`engine::result::ValidationReport` — `schema_version` + a non-empty `findings`) and
/// **no top-level `error` key**. A row added later inherits it, and a resolver that
/// regresses to `anyhow` reddens here whatever code the row expects.
#[test]
fn every_write_miss_answers_inside_the_finding_envelope() {
    let fx = provision();
    let addr = format!("changelog:{}", fx.slug);
    let mut broken: Vec<String> = Vec::new();

    for cell in CELLS {
        let args: Vec<String> = cell
            .args
            .iter()
            .map(|arg| arg.replace("{addr}", &addr))
            .chain(["--format".to_owned(), "json".to_owned()])
            .collect();
        let out = fx.run(
            &args.iter().map(String::as_str).collect::<Vec<&str>>(),
            cell.stdin,
        );
        assert!(
            !out.status.success(),
            "`{}` must block (non-zero exit)",
            cell.what,
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        let Ok(report) = serde_json::from_str::<serde_json::Value>(stderr.trim()) else {
            broken.push(format!("  {}: stderr is not JSON:\n{stderr}", cell.what));
            continue;
        };
        if report.get("error").is_some() {
            broken.push(format!(
                "  {}: answered through the operational-error funnel — a top-level \
                 `error` key, outside the finding envelope",
                cell.what,
            ));
            continue;
        }
        if report["schema_version"].as_u64() != Some(u64::from(engine::result::SCHEMA_VERSION)) {
            broken.push(format!(
                "  {}: the reject carries no `schema_version` {} stamp; got:\n{stderr}",
                cell.what,
                engine::result::SCHEMA_VERSION,
            ));
            continue;
        }
        match report["findings"].as_array() {
            Some(findings) if !findings.is_empty() => {}
            _ => broken.push(format!(
                "  {}: the envelope carries no findings; got:\n{stderr}",
                cell.what,
            )),
        }
    }

    assert!(
        broken.is_empty(),
        "{} of {} cells answered outside the finding envelope:\n{}",
        broken.len(),
        CELLS.len(),
        broken.join("\n"),
    );
}

/// A fixture in the state the collision column never drove: `setup` + `start` and **no
/// `doc create`**, so the `doc author` batch's own `create` is the doc's first write.
/// This is the shape every `migrate-*` workflow drives — the pack's own step prose says
/// *"the CLI creates the record and places every field and prose slot over a single
/// staged buffer"* (`crates/cli/pack/steps/author-migration-spec.yaml`) — and it is the
/// arm the batch's whole-or-nothing rollback **discards**: nothing is staged when the
/// reject prints, so a route naming that doc cannot run.
///
/// `slug` is the id `--title Changelog` mints (the same derivation [`provision`] reads
/// back off `doc create`), so [`Fixture::staged`] addresses the file the rollback must
/// have removed.
fn unprovisioned() -> Fixture {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = fixture_pack();
    init_repo(repo.path());
    let fx = Fixture {
        repo,
        home,
        pack,
        slug: "changelog".to_owned(),
    };
    ok_stdout(fx.run(&["setup"], None), "jigc setup");
    ok_stdout(
        fx.run(
            &["start", "--workflow", "log-change", "log the release"],
            None,
        ),
        "jigc start --workflow log-change",
    );
    fx
}

/// The leading backticked command of a route (`` `<cmd>`<tail> ``), or `None` for a
/// route that presents no command at all — the non-panicking [`backticked`], because
/// *whether* the surface presents a command is the question here, not what it says.
fn leading_command(route: &str) -> Option<&str> {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
}

/// One cell of the batch collision's **destination axis** — the two questions that
/// together decide what the reject leaves behind, and therefore what its route may say.
struct BatchCell {
    what: &'static str,
    /// The corpus state the batch runs against (**provisioning**: does a doc survive the
    /// whole-or-nothing rollback at all?).
    fixture: fn() -> Fixture,
    /// The payload carrying the collision (**duplicate origin**: does the id it collides
    /// with come from the corpus, or from an earlier item of this same payload?).
    payload: &'static [u8],
    /// The id the colliding title mints — the subject the reject's message must name.
    minted: &'static str,
    /// **Whether that id stands in the doc the reject leaves behind.** This is the fact
    /// the enriched route asserts in as many words (*"beside the item already there"*)
    /// and the fact its `--slug <minted>-N` suffix is derived from, so it is asserted
    /// here against the product's own read-back rather than assumed from the payload.
    minted_survives: bool,
    /// Whether a staged instance survives the reject at all.
    staged_after: bool,
}

/// **The batch collision's destination axis** (M49 Increment 5, the T5 triage and its
/// validation fix). The `write.already-present` enrichment hands back a mint *into a doc,
/// beside an item*, and both halves of that sentence are claims about a state the batch
/// has already unwound by the time the route prints. Two independent questions decide it,
/// so the axis is their product rather than either one alone:
///
///   * **provisioning** — over a doc the task already staged the whole-or-nothing
///     rollback restores the pre-image, while on the **create** arm (the batch's own
///     `create` is the doc's first write — the shape every `migrate-*` workflow drives)
///     it removes the file the create provisioned, so nothing at all is staged;
///   * **duplicate origin** — the id may be taken by the **corpus** (an item that is
///     genuinely still there afterwards), or by an **earlier item of this same payload**
///     (which the rollback takes with it, so nothing is there afterwards).
///
/// The column shipped over one cell of the four (staged × corpus-taken) and the
/// implementation probed the **in-progress batch buffer** — the one state that is
/// discarded — so the deliverable's own headline case, *890 titles minting 884 slugs*,
/// got a route asserting an item that is not there and a `-2` suffix around a **free**
/// bare id: run verbatim it exits 0 and lands a mis-suffixed orphan while the rest of the
/// payload stays unauthored (`design/surface-contract.md` law 1: nothing lies).
///
/// The property is stated over the axis rather than over either fix: **the route
/// describes the state the reject leaves behind.** Where the colliding id survives, a
/// command is offered and it must run *and land beside that item*; where it does not,
/// no command is offered — the recovery is the payload's, and a single mint would be a
/// partial repair under an id nothing asked for.
///
/// One cell of the 2×2 is **excluded, with reason**: *create arm × corpus-taken* cannot
/// exist, because the doc the create provisions is empty — there is no corpus item in it
/// for a payload title to collide with. The three reachable cells are driven.
#[test]
fn a_batch_collision_route_describes_the_state_it_leaves_behind() {
    const CELLS: &[BatchCell] = &[
        BatchCell {
            what: "staged doc × an id the corpus already holds",
            fixture: provision,
            payload: b"title: Changelog\nsections:\n  - id: releases\n    items:\n      - title: \"1.3.0\"\n",
            minted: "1-3-0",
            minted_survives: true,
            staged_after: true,
        },
        BatchCell {
            what: "staged doc × a duplicate inside the payload itself",
            fixture: provision,
            payload: b"title: Changelog\nsections:\n  - id: releases\n    items:\n      - title: \"2.0.0\"\n      - title: \"2-0-0\"\n",
            minted: "2-0-0",
            minted_survives: false,
            staged_after: true,
        },
        BatchCell {
            what: "the batch's own create × a duplicate inside the payload itself",
            fixture: unprovisioned,
            payload: b"title: Changelog\nsections:\n  - id: releases\n    items:\n      - title: \"1.3.0\"\n      - title: \"1-3-0\"\n",
            minted: "1-3-0",
            minted_survives: false,
            staged_after: false,
        },
    ];

    for cell in CELLS {
        let fx = (cell.fixture)();
        let out = fx.run(
            &[
                "doc",
                "author",
                "changelog",
                "--from-file",
                "-",
                "--format",
                "json",
            ],
            Some(cell.payload),
        );
        assert!(
            !out.status.success(),
            "`{}` must block (non-zero exit); stdout:\n{}",
            cell.what,
            String::from_utf8_lossy(&out.stdout),
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        let report: serde_json::Value = serde_json::from_str(stderr.trim())
            .unwrap_or_else(|e| panic!("`{}` stderr is JSON: {e}; got:\n{stderr}", cell.what));
        assert_eq!(
            report["findings"][0]["code"].as_str(),
            Some("write.already-present"),
            "`{}` blocks on the collision; got:\n{stderr}",
            cell.what,
        );
        let message = report["findings"][0]["message"].as_str().unwrap_or("");
        assert!(
            message.contains(&format!("{:?}", cell.minted)),
            "`{}`: the reject names the colliding minted id {:?}; got:\n{message}",
            cell.what,
            cell.minted,
        );
        // The facts the route is adjudicated against, read back through the product: the
        // batch persists nothing, and on the create arm "nothing" includes the doc the
        // create provisioned — so the id it collided with is there afterwards only when
        // the **corpus** held it.
        let addr = format!("changelog:{}", fx.slug);
        assert_eq!(
            fx.staged().exists(),
            cell.staged_after,
            "`{}`: a staged instance {} the reject",
            cell.what,
            if cell.staged_after {
                "survives"
            } else {
                "must not survive"
            },
        );
        if cell.staged_after {
            let shown = fx.run_route(
                &format!("jigc doc show {addr}#releases --task {TASK_ID}"),
                cell.what,
            );
            assert_eq!(
                shown.contains(&format!("{{#{}}}", cell.minted)),
                cell.minted_survives,
                "`{}`: after the reject, `{{#{}}}` {} `#releases`; got:\n{shown}",
                cell.what,
                cell.minted,
                if cell.minted_survives {
                    "stands in"
                } else {
                    "is absent from"
                },
            );
        }
        let route = report["findings"][0]["route"]
            .as_str()
            .unwrap_or_else(|| panic!("`{}` carries a route; got:\n{stderr}", cell.what));
        match leading_command(route) {
            Some(cmd) => {
                // A command is offered only where there is an item to mint beside — and
                // then law 1 binds twice: it must run, and it must land the second entry
                // **next to** the one the route says is already there.
                assert!(
                    cell.minted_survives,
                    "`{}`: no command is offered where the colliding id {:?} is not in \
                     the doc the reject leaves behind — the duplicate is inside the \
                     payload, and one mint would be a partial repair under an id nothing \
                     asked for; got the route: {route}",
                    cell.what, cell.minted,
                );
                fx.run_route(cmd, cell.what);
                let shown = fx.run_route(
                    &format!("jigc doc show {addr}#releases --task {TASK_ID}"),
                    cell.what,
                );
                for anchor in [cell.minted.to_owned(), format!("{}-2", cell.minted)] {
                    assert!(
                        shown.contains(&format!("{{#{anchor}}}")),
                        "`{}`: after the route ran, `{{#{anchor}}}` stands in \
                         `#releases`; got:\n{shown}",
                        cell.what,
                    );
                }
            }
            None => {
                assert!(
                    !cell.minted_survives,
                    "`{}`: the colliding id {:?} is still there, so the mint beside it \
                     is a command the agent can be handed; got the route: {route}",
                    cell.what, cell.minted,
                );
                // What is left is the payload's own recovery, and it must be named: the
                // batch landed nothing, so revising the payload and re-running the whole
                // `doc author` is the complete repair.
                let said = route.to_lowercase();
                assert!(
                    said.contains("revise the payload") && said.contains("re-run"),
                    "`{}`: with no doc-side mint to offer, the route names the payload \
                     revision and the re-run that completes it; got: {route}",
                    cell.what,
                );
            }
        }
    }
}
