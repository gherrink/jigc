//! M50 Increment 4, T1 — **the two root knobs get one home** (`settle-record.md` → D8;
//! `roadmap.md` → Milestone 50, Increment 4).
//!
//! `docs-root` and `placement-root` are the two knobs that re-point where every managed
//! doc lives, and both then **move** the committed docs the re-point strands. Every rule
//! that binds one binds the other, and until now that fact lived twice, hand-written, as
//! a `matches!` over the two key literals — the shape M45's complete-fix lens is named
//! for: a third rule (T2's home predicate, T3's value predicate) applied at one arm and
//! not the other is a rule that is not applied at all.
//!
//! Two arms, each naming the kind of set it iterates:
//!
//!   * **The source fence** — a *derivation over one file's text*: the hand-written pair
//!     literal appears **nowhere** in `crates/cli/src/config.rs`, in either ordering, so a
//!     future per-knob rule cannot be re-spelled inline beside the registry that exists to
//!     carry it. The fence lives in a different file from the one it scans: a fence whose
//!     own assertion text carries the forbidden literal could never go green.
//!   * **The declaration derivation** — every [`cli::config::ROOT_KNOBS`] member is a
//!     **declared knob** in the pack's `config/knobs.yaml`, read through the same
//!     `engine::knobs::load_knobs` the cascade seeds itself from (never a hand list here).
//!     A registry member that is not on the closed surface would name a key `jigc config
//!     set` rejects before any root rule could reach it.

use engine::knobs::load_knobs;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};

/// The module the registry governs, read at compile time so the fence cannot drift from
/// the file it is about.
const CONFIG_RS: &str = include_str!("../src/config.rs");

/// Both spellings of the hand-written pair — the fence is about the *shape* (a per-call-site
/// `matches!` over the two root-knob keys), not about one authoring order.
fn pair_literals() -> [String; 2] {
    let docs = format!("{:?}", "docs-root");
    let placement = format!("{:?}", "placement-root");
    [
        format!("{docs} | {placement}"),
        format!("{placement} | {docs}"),
    ]
}

/// **The source fence.** The two root-knob keys are never matched as a hand-written pair
/// inside `config.rs` — the registry is the one home, so a rule added at one knob is added
/// at both by construction.
#[test]
fn config_rs_carries_no_hand_written_root_knob_pair() {
    let hits: usize = pair_literals()
        .iter()
        .map(|lit| CONFIG_RS.matches(lit.as_str()).count())
        .sum();
    assert_eq!(
        hits, 0,
        "`crates/cli/src/config.rs` still matches the two root-knob keys as a hand-written \
         pair ({hits} occurrence(s)) — every root-knob rule reads the one registry \
         (`cli::config::ROOT_KNOBS`) so a rule added at one knob is added at both",
    );
}

/// **The declaration derivation.** Every [`cli::config::ROOT_KNOBS`] member is a declared
/// knob on the pack's closed surface, read through the same `engine::knobs::load_knobs` the
/// cascade seeds itself from — never a hand list here, which would prove only that this file
/// agrees with itself. A member that is not declared would name a key `jigc config set`
/// rejects with `config.undeclared-key` before any root rule could reach it, so the registry
/// would carry a rule for a knob nobody can set.
#[test]
fn every_root_knob_is_a_declared_knob() {
    let pack = cli::pack::EmbeddedPack::new();
    let bytes = pack
        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
        .expect("the embedded pack ships `config/knobs`");
    let knobs = load_knobs(&bytes).expect("`config/knobs` parses");
    let declared: Vec<&str> = knobs.keys().collect();

    assert!(
        !cli::config::ROOT_KNOBS.is_empty(),
        "the root-knob registry is the subject of this derivation — an empty one would make \
         every arm below vacuous",
    );
    for key in cli::config::ROOT_KNOBS {
        assert!(
            knobs.field(key).is_some(),
            "`{key}` is a `ROOT_KNOBS` member but not a declared knob in the pack's \
             `config/knobs.yaml`; declared: {declared:?}",
        );
    }
}

// ---------------------------------------------------------------------------------------
// T2 — **jigc's own workbench is not a home for managed docs.**
//
// Both root knobs re-point where every managed doc lives, and both then MOVE the committed
// docs the re-point strands. Driven on the shipped binary, before the guard existed:
//
// ```text
// $ jigc config set docs-root .jigc
// relocating 1 committed doc(s) stranded by the `docs-root` re-point to `.jigc` …
//   - docs/research/root-knob-probe.md → .jigc/research/root-knob-probe.md
// $ echo $?
// 0
// ```
//
// …and the placement knob reaches the identical state (`docs/roadmap.md → .jigc/roadmap.md`).
// That home is the tree `jigc uninstall` removes **whole** (`setup::uninstall` step 1 is
// `remove_dir_all(.jigc)`), so a managed doc homed there is one teardown from gone.
//
// The third value is driven, not inferred: `jigc config set placement-root .JIGC` stages
// `docs/roadmap.md -> .JIGC/roadmap.md` while the bytes land in the **real** `.jigc/` — the
// index and the worktree then name different directories for one file, permanently.
//
// The set iterated is the code-side registry `cli::config::ROOT_KNOBS` crossed with the three
// spellings that reach the workbench (the directory itself, a path inside it, and the
// case-folded form), and each cell is adjudicated on all three facts a half-acting door would
// split: the routed finding is raised, **nothing moved**, and the knob did **not** land.
// ---------------------------------------------------------------------------------------

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Every spelling that **reaches** jigc's own workbench: the directory itself, a path inside
/// it, the case-folded form a case-insensitive filesystem resolves onto it — and each of those
/// again behind a `..` hop, which is where the first cut of the rule was evadable.
///
/// The hop cells are driven, not imagined: at `6551d49` the literal three were refused
/// (`config.workbench-root`, rc=1, both knobs) while
/// `jigc config set placement-root docs/../.jigc` exited **0** and staged
/// `R docs/decisions-log.md -> .jigc/decisions-log.md` — git normalizes the path it records,
/// so the hop reached the identical directory the literal spelling is refused for. The set is
/// therefore the *resolved destination*, not the typed prefix: `{the workbench, a child of it}`
/// × `{as typed, behind a hop}` × `{as spelled, case-folded}`, plus the leading `./` and the
/// multi-hop forms that fold the same way.
const WORKBENCH_VALUES: [&str; 8] = [
    ".jigc",
    ".jigc/displaced",
    ".JIGC",
    "docs/../.jigc",
    "./docs/../.jigc",
    "docs/../.jigc/displaced",
    "docs/../.JIGC",
    "notes/deep/../../.jigc",
];

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-root-knob-rules-{tag}-{}-{:?}",
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

/// A conformant, v1-stamped `roadmap` at its declared **placement** home — what
/// `placement-root` moves.
const ROADMAP: &str = "\
---
schema-version: 1
---

# Roadmap

## Milestones

### First milestone  {#first-milestone}

#### Proves

The loop closes.

#### Decomposition

One increment.
";

/// A conformant, v1-stamped `research` doc at its declared **`location:`** home — what
/// `docs-root` moves. Both doctypes are planted so neither knob's cells are vacuous: each
/// knob's own subject is in the corpus it must leave untouched.
const RESEARCH: &str = "\
---
date: 2026-01-01
schema-version: 1
---

# Root Knob Probe

## Question

Where do managed docs live?

## Findings

Not in jigc's workbench.

## Sources

The driven repro in this module's header.
";

/// The two managed docs the corpus commits — one per knob's subject.
const MANAGED: [(&str, &str); 2] = [
    ("docs/roadmap.md", ROADMAP),
    ("docs/research/root-knob-probe.md", RESEARCH),
];

/// A real git repo with `jigc setup` run and both managed docs committed + ingested.
struct Corpus {
    repo: TempDir,
    home: TempDir,
}

impl Corpus {
    fn new(tag: &str) -> Self {
        let corpus = Self {
            repo: TempDir::new(tag),
            home: TempDir::new(&format!("{tag}-home")),
        };
        corpus.git(&["init", "-q"]);
        corpus.git(&["config", "user.email", "test@example.com"]);
        corpus.git(&["config", "user.name", "Test"]);
        fs::write(corpus.repo.path().join("README.md"), "hello\n").expect("write file");
        corpus.git(&["add", "."]);
        corpus.git(&["commit", "-q", "-m", "initial"]);
        corpus.ok(&["setup"]);
        for (rel, body) in MANAGED {
            let abs = corpus.repo.path().join(rel);
            fs::create_dir_all(abs.parent().expect("a parent")).expect("create parent dir");
            fs::write(&abs, body).expect("write the managed doc");
            corpus.git(&["add", rel]);
            corpus.git(&["commit", "-q", "-m", "plant a managed doc"]);
        }
        corpus.ok(&["ingest"]);
        corpus
    }

    fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(self.repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout).expect("utf-8 git stdout")
    }

    fn jigc(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    }

    fn ok(&self, args: &[&str]) -> String {
        let out = self.jigc(args);
        assert!(
            out.status.success(),
            "`jigc {}` must exit 0; stdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout).expect("utf-8 stdout")
    }

    /// Every path git currently tracks — the set a clone would receive.
    fn tracked(&self) -> Vec<String> {
        self.git(&["ls-files"]).lines().map(str::to_owned).collect()
    }

    /// Plant the on-disk shapes [`UNUSABLE_VALUES`] names: a symlink `linked` → the real
    /// directory `real/`, which also carries `sub/` so the **ancestor** cell addresses an
    /// existing path through the link. `README.md` is already committed by [`Corpus::new`],
    /// which is what makes `README.md/sub` — the file-shaped **ancestor** cell — a value whose
    /// leaf exists nowhere while a component of it is a file; and the absolute cell needs
    /// nothing on disk.
    ///
    /// Deliberately left untracked: what the refusal is about is the *destination shape*, and
    /// `git add`-ing the link would only add a second reason git dislikes it.
    fn plant_unusable_shapes(&self) {
        fs::create_dir_all(self.repo.path().join("real").join("sub")).expect("create real/sub");
        std::os::unix::fs::symlink("real", self.repo.path().join("linked")).expect("link real");
    }

    /// The project manifest's raw bytes (`.jigc/config/manifest.yaml`), or `""`.
    fn manifest(&self) -> String {
        fs::read_to_string(
            self.repo
                .path()
                .join(".jigc")
                .join("config")
                .join("manifest.yaml"),
        )
        .unwrap_or_default()
    }
}

/// One refused re-point, on the three facts a half-acting door would split: the routed
/// finding is raised, **nothing moved** (both docs still at their homes and still tracked —
/// the index is the assertion, since the loss shape is a staged move), and the knob did not
/// land (a landed knob with no move leaves the store pointing at a home no doc is at).
///
/// `code` is the caller's, because each root rule asks a **different question about the same
/// path** and therefore carries its own code (`design/storage.md` → Placement) — asserting a
/// shape is refused without saying *which* rule refused it would let either rule cover for the
/// other's absence.
///
/// Returns the refusal's `stderr`, so an arm whose rule earns a sentence of its own can
/// adjudicate what the reason *says* without re-driving the door (M51 Increment 1 / T5).
fn assert_refused_and_inert(corpus: &Corpus, key: &str, value: &str, code: &str) -> String {
    let before = corpus.manifest();
    let out = corpus.jigc(&["config", "set", key, value]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

    assert!(
        !out.status.success(),
        "`jigc config set {key} {value}` must NOT exit 0 — a root that leaves the store lying \
         is refused before it is written; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains(code),
        "the refusal carries `{code}`, the rule that refused it; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(value),
        "the refusal names the value it refused; stderr:\n{stderr}",
    );

    for (rel, _) in MANAGED {
        assert!(
            corpus.repo.path().join(rel).exists(),
            "nothing moved — `{rel}` is still at its home",
        );
        assert!(
            corpus.tracked().iter().any(|path| path == rel),
            "`{rel}` is still TRACKED: the loss shape is a staged move, so the index is the \
             assertion, not the filesystem; tracked:\n{:#?}",
            corpus.tracked(),
        );
    }
    assert_eq!(
        corpus.manifest(),
        before,
        "a refused set records nothing — a landed knob with no move leaves the store pointing \
         at a home no doc is at",
    );
    stderr
}

/// **The sixteen cells.** Every [`cli::config::ROOT_KNOBS`] member × every spelling that
/// reaches the workbench, each refused before anything moves and before the knob lands.
///
/// One corpus for all sixteen: a refused cell leaves the store byte-identical, so reuse is not
/// a shortcut but a further assertion — the last cell adjudicates the same untouched corpus the
/// first one did.
#[test]
fn no_root_knob_accepts_jigcs_own_workbench_as_a_home() {
    let corpus = Corpus::new("workbench");
    for key in cli::config::ROOT_KNOBS {
        for value in WORKBENCH_VALUES {
            assert_refused_and_inert(&corpus, key, value, "config.workbench-root");
        }
    }
}

// ---------------------------------------------------------------------------------------
// T3 — **a root the store cannot describe is refused before it is written.**
//
// The home rule above asks *whose* directory the value names. This one asks whether the value
// is a **usable root at all** — a repo-relative directory the store can go on describing after
// the re-point. Three shapes are not, all three driven at HEAD on a `committed-singletons` rig:
//
// ```text
// $ jigc config set placement-root README.md
//   - docs/decisions-log.md: could not relocate (creating the destination dir for
//     README.md/decisions-log.md: File exists (os error 17)) — move it by hand
//   - docs/roadmap.md: could not relocate (…) — move it by hand
// config: set `placement-root` = `README.md`  … $? = 0
// $ jigc doc list          # both re-rooted docs are GONE from the store surface
// changelog:changelog  CHANGELOG.md  managed
// vision:vision        VISION.md     managed
// ```
//
// …which is worse than N5 records: the knob lands with every move failed **and** the store stops
// listing the docs at all. The absolute value is silently reinterpreted (N6) — `placement-root
// /tmp/elsewhere` stages `R docs/roadmap.md -> tmp/elsewhere/roadmap.md` while `jigc config get`
// echoes `/tmp/elsewhere`, so the knob and the store name different homes. And a symlinked root
// (N7) stages `RD`: `git status` after `placement-root linked` reports
// `RD docs/roadmap.md -> linked/roadmap.md` — the index records a path the worktree does not
// have, because git records the link and not a path through it, so `jigc doc list`'s path and
// git's recorded path disagree permanently.
//
// **The positional subject of both on-disk shapes is every existing component of the value,
// not its leaf.** A symlinked ANCESTOR (`linked/sub`) reaches the identical `RD` state, and a
// file-shaped ANCESTOR (`README.md/sub`) reaches the identical vanished-store state — driven at
// `c231a6d`, when the file-shaped arm was asked once, after the walk, of the leaf alone:
//
// ```text
// $ jigc config set placement-root README.md/sub
//   - docs/decisions-log.md: could not relocate (creating the destination dir for
//     README.md/sub/decisions-log.md: Not a directory (os error 20)) — move it by hand
//   - docs/roadmap.md: could not relocate (…) — move it by hand
// config: set `placement-root` = `README.md/sub`  … $? = 0
// $ jigc doc list          # both re-rooted docs are GONE from the store surface
// ```
//
// So the two shapes are asked of the same subject, in the same walk, and this arm iterates
// {leaf, ancestor} × {symlink, file} rather than the symlink half of that axis alone. The walk
// starts at the repo root and steps the value's components — it must NOT canonicalize the root
// itself, since macOS corpora live under a symlinked `/var` and an ancestor-canonicalizing form
// would refuse every fixture. `""` and `.` carry no named components at all and stay admitted,
// which `docs_root::` / `placement_override::` / `corpus_migration_backstop::` all require.
//
// Two sets, each named: the refusing arm iterates the code-side registry
// `cli::config::ROOT_KNOBS` × the five shapes; the admitting arm iterates the same registry ×
// the five values a root legitimately takes (unset, the repo root, an existing directory, one
// jigc must create, and one spelled through a `..` hop that reaches none of the refused homes)
// — the over-refusal guard, without which "refuse three shapes" is satisfied by refusing
// everything.
// ---------------------------------------------------------------------------------------

/// The five values that are not usable roots — an absolute path, and each of the two on-disk
/// shapes at each of the two positions the walk can meet it: a symlinked leaf, a symlinked
/// ancestor, a file-shaped leaf, a file-shaped ancestor. Planted by
/// [`Corpus::plant_unusable_shapes`].
const UNUSABLE_VALUES: [&str; 5] = [
    "/tmp/jigc-root-knob-elsewhere",
    "linked",
    "linked/sub",
    "README.md",
    "README.md/sub",
];

/// The five values a root legitimately takes, each paired with **the value that lands**: unset
/// (`""`, canonicalized to `.`), the repo root, an existing directory, one that does not exist
/// yet (jigc creates it on the move) — and one spelled **through a `..` hop that does not reach
/// the workbench**, the over-refusal guard on the home rule's normalization: folding `..` must
/// not turn every hopping value into a refusal, only the ones that land in `.jigc/`.
///
/// The pair is spelled out per cell rather than computed, so this arm cannot re-implement the
/// fold it is checking (M49's *statement == constant* lesson): `docs/../notes` lands as `notes`
/// because the write door folds a root value once, so that one spelling reaches every reader
/// and the store renders the path git recorded (`cli::config::normalize_root_value`).
const USABLE_VALUES: [(&str, &str); 5] = [
    ("", "."),
    (".", "."),
    ("docs", "docs"),
    ("notes", "notes"),
    ("docs/../notes", "notes"),
];

/// **The ten refusing cells.** Every [`cli::config::ROOT_KNOBS`] member × every shape the store
/// cannot describe, each refused before anything moves and before the knob lands.
///
/// One corpus for all ten, on the same reasoning as the workbench arm: a refused cell leaves
/// the store byte-identical, so the tenth cell adjudicates the corpus the first one did.
#[test]
fn no_root_knob_accepts_a_root_the_store_cannot_describe() {
    let corpus = Corpus::new("unusable");
    corpus.plant_unusable_shapes();
    for key in cli::config::ROOT_KNOBS {
        for value in UNUSABLE_VALUES {
            assert_refused_and_inert(&corpus, key, value, "config.unusable-root");
        }
    }
}

/// **The ten admitting cells — the over-refusal guard.** A predicate that refuses everything
/// satisfies the arm above, so each legitimate root is driven to exit 0 on its own corpus, the
/// knob reads back the value it was set to, and **the store still describes both managed docs**
/// — the property N5 broke.
#[test]
fn every_usable_root_is_still_admitted_and_the_store_still_describes_its_docs() {
    for key in cli::config::ROOT_KNOBS {
        for (cell, (value, expected)) in USABLE_VALUES.iter().enumerate() {
            let corpus = Corpus::new(&format!("usable-{key}-{cell}"));
            corpus.ok(&["config", "set", key, value]);

            let reading = corpus.ok(&["config", "get", key]);
            assert!(
                // The whole `<key> = <value>` reading, never a substring of it: `docs-root`
                // contains `docs`, so a bare `contains(expected)` would pass on a knob that
                // never landed at all.
                reading.contains(&format!("{key} = {expected}")),
                "`jigc config set {key} {value:?}` must read back as `{expected}`; got:\n{reading}",
            );

            let listing = corpus.ok(&["doc", "list"]);
            for id in ["roadmap:roadmap", "research:root-knob-probe"] {
                assert!(
                    listing.contains(id),
                    "the store still describes `{id}` after `{key}` = {value:?} — a root the \
                     store cannot describe is exactly what the refusing arm exists to catch; \
                     listing:\n{listing}",
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------------------
// T4 — **a root value is stored in the one spelling the store renders** (M50 Increment 4
// validation, N8).
//
// Every rule above asks a question about the home a value *reaches*, and each folds `..`
// and `./` privately to find it. The value itself was then stored **as typed**, so the
// move floors and the read surfaces resolved a spelling no rule had folded — and a value
// naming byte-for-byte the CURRENT home was admitted at exit 0 as a re-point. Driven at
// `71c8f7a`, on a `committed-singletons` rig:
//
// ```
// $ jigc config set placement-root 'docs/../docs'
// relocating the committed doc(s) stranded by the `placement-root` re-point to `docs/../docs`:
//   - docs/decisions-log.md: could not relocate (reading the stranded doc …: No such file …)
//   - docs/roadmap.md: could not relocate (…) — move it by hand
// config: set `placement-root` = `docs/../docs`                                       rc=0
// $ ls docs            -> (empty)
// $ ls .jigc/displaced -> decisions-log.md  roadmap.md
// $ git status --porcelain -> D docs/decisions-log.md ; D docs/roadmap.md
// $ jigc doc list      -> roadmap + decisions-log GONE from the store surface
// ```
//
// The destination *was* the source, so the mover read both committed docs as foreign
// squatters, displaced them into the gitignored `.jigc/displaced/` and then could not move
// them back — two staged deletions with no matching adds, the knob landed, and the bytes
// then sat in the one tree `jigc uninstall` removes whole. Five spellings of the identical
// home reach that state on **both** knobs (`./docs`, `docs/./`, `docs/../docs`,
// `.jigc/../docs`, and every multi-hop fold of them); only the trailing-slash form escaped,
// because `trim_matches('/')` is the one fold the storage path already did.
//
// The softer half of the same class is the **admitted** re-point: `docs-root docs/../notes`
// moved the doc and staged `R docs/research/x.md -> notes/research/x.md` while `jigc doc
// list` printed `docs/../notes/research/x.md` — the knob and git naming different homes for
// one doc, which is exactly the state the absolute-value rule (N6) is refused for. The
// shipped admitting arm did not catch it because it asserts the doc **id** is still listed
// and never the path it is listed at.
//
// Two sets, each named:
//
//   * **A manufactured shape space** — [`cli::config::ROOT_KNOBS`] × the five ways one home
//     is spelled non-canonically (a leading `./`, a trailing separator, an interior `./`, a
//     `..` hop back onto itself, a `..` hop in from a sibling tree). Manufactured rather
//     than enumerated because no registry of spellings exists to read: the class is the
//     *grammar* of a path, and the shipped corpus populates it not at all.
//   * **The store's own rendering** — the path column `jigc doc list` prints for each
//     planted doc, asked of the canonical form after a genuine hop-spelled move.
// ---------------------------------------------------------------------------------------

/// The five non-canonical spellings of the corpus's **current** home (`docs`, the home both
/// knobs already resolve every planted doc to). Setting a root to any of them names the home
/// the docs are already at, so the only correct outcome is a no-op.
const HOP_SPELLINGS: [&str; 5] = [
    "./docs",
    "docs/",
    "docs/./",
    "docs/../docs",
    ".jigc/../docs",
];

/// The path `jigc doc list` prints for `id` — the second whitespace-separated field of its
/// row. Read as a field rather than with `contains`, because `docs/roadmap.md` is a
/// **substring** of `./docs/roadmap.md`, so a substring assertion passes on exactly the
/// un-normalized rendering this arm exists to catch.
fn listed_path(listing: &str, id: &str) -> String {
    listing
        .lines()
        .find(|line| line.split_whitespace().next() == Some(id))
        .unwrap_or_else(|| panic!("`jigc doc list` has no row for `{id}`; listing:\n{listing}"))
        .split_whitespace()
        .nth(1)
        .expect("a path column")
        .to_owned()
}

/// **The ten identity cells.** A root spelled non-canonically that names the home the docs
/// are already at is a **no-op**: nothing moves, nothing is staged, git tracks the same set,
/// and the store still renders both docs at the paths they are committed at.
#[test]
fn a_root_respelled_onto_its_own_home_moves_nothing() {
    for key in cli::config::ROOT_KNOBS {
        for (cell, value) in HOP_SPELLINGS.iter().enumerate() {
            let corpus = Corpus::new(&format!("identity-{key}-{cell}"));
            let before = corpus.tracked();

            corpus.ok(&["config", "set", key, value]);

            assert_eq!(
                corpus.tracked(),
                before,
                "`jigc config set {key} {value:?}` names the home both docs are already at — \
                 it must move nothing; the tracked set changed",
            );
            let status = corpus.git(&["status", "--porcelain"]);
            assert!(
                !status.lines().any(|line| line.contains(".md")),
                "`jigc config set {key} {value:?}` must leave no staged move or deletion of a \
                 managed doc; `git status --porcelain`:\n{status}",
            );
            let listing = corpus.ok(&["doc", "list"]);
            for (rel, _) in MANAGED {
                let id = if rel.contains("roadmap") {
                    "roadmap:roadmap"
                } else {
                    "research:root-knob-probe"
                };
                assert_eq!(
                    listed_path(&listing, id),
                    rel,
                    "the store still renders `{id}` at `{rel}` after `{key}` = {value:?}; \
                     listing:\n{listing}",
                );
            }
        }
    }
}

/// **The two moving cells — the over-refusal guard's other half.** A hop-spelled root that
/// reaches a *different* home still moves, once, and the store then renders the moved doc at
/// the **canonical** path git recorded — never at the spelling the operator typed.
#[test]
fn a_hop_spelled_move_lands_the_canonical_path_in_the_store() {
    // Each knob's own subject, and where a `notes` root puts it: `docs-root` re-roots the
    // located `research` doc, `placement-root` re-roots the placement `roadmap`.
    for (key, id, moved_to) in [
        (
            "docs-root",
            "research:root-knob-probe",
            "notes/research/root-knob-probe.md",
        ),
        ("placement-root", "roadmap:roadmap", "notes/roadmap.md"),
    ] {
        let corpus = Corpus::new(&format!("hop-move-{key}"));
        corpus.ok(&["config", "set", key, "docs/../notes"]);

        let status = corpus.git(&["status", "--porcelain"]);
        assert!(
            status.contains(&format!("-> {moved_to}")),
            "`{key} docs/../notes` stages the move to `{moved_to}`; \
             `git status --porcelain`:\n{status}",
        );
        let listing = corpus.ok(&["doc", "list"]);
        assert_eq!(
            listed_path(&listing, id),
            moved_to,
            "the store renders `{id}` at the path git recorded, never at the typed spelling; \
             listing:\n{listing}",
        );
        let reading = corpus.ok(&["config", "get", key]);
        assert!(
            reading.contains(&format!("{key} = notes")),
            "`jigc config get {key}` reads back the value that landed; got:\n{reading}",
        );
    }
}

// ---------------------------------------------------------------------------------------
// M51 Increment 1 / T5 — **a root value that does not read back as itself is refused**
// (EC-27; `settle-record.md` → D1 part 5; `design/storage.md` → What a root knob refuses).
//
// The three rules above all ask a question about the *home* a value reaches. This one asks a
// question about the **value**: can the operator read it back and see what they set? Driven at
// `b9e13cf` on a `committed-singletons` rig, all three cells at exit 0:
//
// ```text
// $ jigc config set docs-root '   '     # three spaces
// config: set `docs-root` = `   ` — written to `.jigc/config/` …                       rc=0
// $ jigc config get docs-root
// docs-root =      (project)            # visually identical to unset
// $ jigc config set docs-root '  x  '
// $ jigc config get docs-root
// docs-root =   x    (project)          # a directory literally named `  x  `
// ```
//
// …and every managed doc then finalizes under a directory whose name nobody can see or type
// back. `cli::config::normalize_root_value` folds `.` and `..` and never trims, so a whitespace
// run survives the fold as an ordinary `Component::Normal` and reaches the store as a home.
//
// **The class joins `config.unusable-root` rather than minting a code**, on that code's own
// five-reasons-one-code precedent: the operator's fix is the same as for a file-shaped,
// absolute or symlinked root — supply a different root. `ROOT_KNOBS` is the one home, so the
// rule lands at both knobs by construction, which is what this arm's outer loop asserts.
//
// **The subject is every component, not the leaf** — the correction M50 already had to make to
// the on-disk walk beside it (`README.md/sub`), applied here before it could be driven wrong:
// `docs/ notes` names an invisible directory exactly as `  x  ` does.
//
// **The predicate is edge whitespace, not any whitespace.** A tab never reaches this leg —
// `engine::write::check_value`'s control-character floor refuses it two steps earlier under
// `config.value-rejected` — and an interior space (`my notes`) reads back as itself, so
// refusing it would refuse a home the operator can see and type.
//
// The set iterated is the code-side registry `cli::config::ROOT_KNOBS` × the six spellings of
// the class: the whole value as whitespace (two runs, since one space and three spaces read
// back as the same nothing), padded on both sides, padded on one side each way, and a padded
// component that is **not** the first. The over-refusal guard is the shipped admitting arm
// `every_usable_root_is_still_admitted_and_the_store_still_describes_its_docs`, whose first
// two cells are `""` and `.` at both knobs — the two values that carry no named component at
// all, and the ones a trim-shaped rule would be most likely to swallow.
// ---------------------------------------------------------------------------------------

/// The six values that do not read back as themselves — every position edge whitespace can
/// take, in the value and in a component.
const WHITESPACE_VALUES: [&str; 6] = ["   ", " ", "  x  ", "notes ", " notes", "docs/ notes"];

/// **The twelve cells.** Every [`cli::config::ROOT_KNOBS`] member × every spelling of the
/// whitespace class, each refused before anything moves and before the knob lands, each
/// refusal naming the value and carrying exactly one route.
///
/// One corpus for all twelve, on the same reasoning as the two arms above: a refused cell
/// leaves the store byte-identical, so the twelfth cell adjudicates the corpus the first did.
#[test]
fn no_root_knob_accepts_a_value_that_does_not_read_back_as_itself() {
    let corpus = Corpus::new("whitespace");
    for key in cli::config::ROOT_KNOBS {
        for value in WHITESPACE_VALUES {
            let stderr = assert_refused_and_inert(&corpus, key, value, "config.unusable-root");
            assert!(
                stderr.contains(&format!("`{value}`")),
                "the refusal quotes the value the operator typed, so the one thing they cannot \
                 see on the read surface is visible on the refusal; stderr:\n{stderr}",
            );
            assert!(
                stderr.contains("whitespace"),
                "the reason names the whitespace — `config.unusable-root` carries four reasons \
                 under one code, so a refusal that does not say which one it is leaves the \
                 operator reading about files and symlinks; stderr:\n{stderr}",
            );
            let routes = stderr
                .lines()
                .filter(|line| line.trim_start().starts_with("route:"))
                .count();
            assert_eq!(
                routes, 1,
                "one refusal, one route (`design/surface-contract.md` → the route floor); \
                 stderr:\n{stderr}",
            );
        }
    }
}
