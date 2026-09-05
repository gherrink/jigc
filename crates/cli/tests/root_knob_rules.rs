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

/// The three spellings that reach jigc's own workbench: the directory itself, a path inside
/// it, and the case-folded form a case-insensitive filesystem resolves onto it.
const WORKBENCH_VALUES: [&str; 3] = [".jigc", ".jigc/displaced", ".JIGC"];

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
    /// and the absolute cell needs nothing on disk.
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
fn assert_refused_and_inert(corpus: &Corpus, key: &str, value: &str, code: &str) {
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
}

/// **The six cells.** Every [`cli::config::ROOT_KNOBS`] member × every spelling that reaches
/// the workbench, each refused before anything moves and before the knob lands.
///
/// One corpus for all six: a refused cell leaves the store byte-identical, so reuse is not a
/// shortcut but a further assertion — the sixth cell adjudicates the same untouched corpus the
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
// The **symlink subject is every existing component of the value, not its leaf**: a symlinked
// ANCESTOR (`linked/sub`) reaches the identical `RD` state. The walk therefore starts at the
// repo root and steps the value's components — it must NOT canonicalize the root itself, since
// macOS corpora live under a symlinked `/var` and an ancestor-canonicalizing form would refuse
// every fixture. `""` and `.` carry no named components at all and stay admitted, which
// `docs_root::` / `placement_override::` / `corpus_migration_backstop::` all require.
//
// Two sets, each named: the refusing arm iterates the code-side registry
// `cli::config::ROOT_KNOBS` × the four shapes; the admitting arm iterates the same registry ×
// the four values a root legitimately takes (unset, the repo root, an existing directory, and
// one jigc must create) — the over-refusal guard, without which "refuse three shapes" is
// satisfied by refusing everything.
// ---------------------------------------------------------------------------------------

/// The four values that are not usable roots — a file, an absolute path, a symlinked leaf, and
/// a symlinked ancestor. Planted by [`Corpus::plant_unusable_shapes`].
const UNUSABLE_VALUES: [&str; 4] = [
    "README.md",
    "/tmp/jigc-root-knob-elsewhere",
    "linked",
    "linked/sub",
];

/// The four values a root legitimately takes: unset (`""`, canonicalized to `.`), the repo root,
/// an existing directory, and one that does not exist yet (jigc creates it on the move).
const USABLE_VALUES: [&str; 4] = ["", ".", "docs", "notes"];

/// **The eight refusing cells.** Every [`cli::config::ROOT_KNOBS`] member × every shape the store
/// cannot describe, each refused before anything moves and before the knob lands.
///
/// One corpus for all eight, on the same reasoning as the workbench arm: a refused cell leaves
/// the store byte-identical, so the eighth cell adjudicates the corpus the first one did.
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

/// **The eight admitting cells — the over-refusal guard.** A predicate that refuses everything
/// satisfies the arm above, so each legitimate root is driven to exit 0 on its own corpus, the
/// knob reads back the value it was set to, and **the store still describes both managed docs**
/// — the property N5 broke.
#[test]
fn every_usable_root_is_still_admitted_and_the_store_still_describes_its_docs() {
    for key in cli::config::ROOT_KNOBS {
        for (cell, value) in USABLE_VALUES.iter().enumerate() {
            let corpus = Corpus::new(&format!("usable-{key}-{cell}"));
            corpus.ok(&["config", "set", key, value]);

            // `""` is canonicalized to the flat sentinel `.` before it lands (both knobs).
            let expected = if value.is_empty() { "." } else { *value };
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
