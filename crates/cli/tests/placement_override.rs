//! **`placement-root` — the one home no knob could reach** (M49 Increment 7 — T1 the
//! knob, T2 the detect + route + move floor).
//!
//! A `location:` doctype's home resolves through the `docs-root` knob, so an adopter
//! whose managed docs do not live under `docs/` re-points every located doctype with one
//! `jigc config set`. A `placement:` doctype's home resolves through **nothing**: it is
//! the literal `placement.file` its schema declares, and since M38 that literal is inside
//! the doctype's frozen `schema-hash`, so `jigc relocate` refuses to move it. The adopter
//! who wants `notes/roadmap.md` instead of `docs/roadmap.md` had **no path at all** —
//! not a knob, not a verb, not a shadow (`design/storage.md` → Placement;
//! `completions/artifacts/M49/settle-record.md` → D6, which argues the override *solely*
//! on that asymmetry).
//!
//! `placement-root` closes it, and **the rule is also the scope answer**: a declared home
//! carrying a leading directory component (`docs/roadmap.md`) resolves to
//! `<placement-root>/<remainder>`; a home declared **at the repo root** (`VISION.md`,
//! `CHANGELOG.md`) is never re-rooted. The ecosystem-idiomatic files are therefore
//! unburiable **by derivation** — there is no doctype allow-list to go stale, which is
//! what the third arm below drives.
//!
//! Composition-invariance survives, which is the property the placement design exists to
//! protect (`storage.md` → Placement — *the resolved path is composition-invariant*): a
//! project override is **one fixed value**, so `docs/roadmap.md` still resolves to the
//! same home under methodology-alone and under `[dev ▸ methodology]`. That is why the
//! knob is declared **identically in both shipped packs** rather than in one — a knob
//! present in only one file is invisible under the whole-file `knobs.yaml` shadow, and
//! the resolved home would then vary by *which pack won*. That half is pinned in-process
//! by `cli::pack::placement_root_is_declared_in_both_packs_identically`.
//!
//! **What is driven here is the emitted artifact**, never a reconstruction: each arm runs
//! `jigc workflow <id> --preview` through the built binary and reads the home out of the
//! `{{ schema:<doctype> }}` projection line the composed step actually carries — the
//! sentence an agent reads before it authors.
//!
//! **Scope bound, stated rather than hidden:** the *literal home strings hand-written into
//! six pack step bodies* (`author-migration-roadmap.yaml`'s "the canonical
//! `docs/roadmap.md`", …) are **not** touched by this task — they are prose, they still
//! say the declared home, and T4 of this increment is where they stop lying. These arms
//! therefore assert on the **generated** projection line, which is the surface the seam
//! feeds.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-placement-override-{tag}-{}-{:?}",
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

/// A real git repo with `jigc setup` run — which wires the embedded methodology pack over
/// the dev base, so the three `migrate-*` workflows below compose.
struct Corpus {
    repo: TempDir,
    home: TempDir,
}

impl Corpus {
    fn new(tag: &str) -> Self {
        let repo = TempDir::new(tag);
        let home = TempDir::new(&format!("{tag}-home"));
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(repo.path())
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr),
            );
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "test@example.com"]);
        git(&["config", "user.name", "Test"]);
        fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
        git(&["add", "."]);
        git(&["commit", "-q", "-m", "initial"]);

        let corpus = Self { repo, home };
        corpus.ok(&["setup"]);
        corpus
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

    /// `jigc <args>`, asserting exit 0 and returning stdout.
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

    /// The composed `jigc workflow <id> --preview` bytes.
    fn preview(&self, workflow: &str) -> String {
        self.ok(&["workflow", workflow, "--preview"])
    }
}

/// The home the `{{ schema:<ty> }}` projection line of an emitted preview names — read out
/// of the composed bytes themselves, never rebuilt from the schema in test code.
///
/// The projection's home line is `` The `<ty>` schema — the managed singleton at `<path>`. ``
/// (`engine::compose::projection_home_line`).
fn projected_home(preview: &str, ty: &str) -> String {
    let lead = format!("The `{ty}` schema — the managed singleton at `");
    let line = preview
        .lines()
        .find(|l| l.starts_with(&lead))
        .unwrap_or_else(|| {
            panic!(
                "the `{ty}` projection home line is absent from the composed preview:\n{preview}"
            )
        });
    line[lead.len()..]
        .split('`')
        .next()
        .expect("the home line closes its backtick")
        .to_owned()
}

/// **The asymmetry closes.** With `placement-root: notes` the `roadmap` singleton — whose
/// declared home `docs/roadmap.md` carries a leading directory component — composes at
/// `notes/roadmap.md`, while the two repo-root doctypes are untouched: `VISION.md` and
/// `CHANGELOG.md` are root **by the ecosystem-idiomatic rule**, and the re-root derivation
/// (leading component or nothing) is what makes them unburiable without an allow-list.
#[test]
fn a_placement_root_reroots_a_nested_home_and_never_a_root_one() {
    let corpus = Corpus::new("nested");
    corpus.ok(&["config", "set", "placement-root", "notes"]);

    assert_eq!(
        projected_home(&corpus.preview("migrate-roadmap"), "roadmap"),
        "notes/roadmap.md",
        "a declared home with a leading directory component re-roots under `placement-root`",
    );
    assert_eq!(
        projected_home(&corpus.preview("migrate-vision"), "vision"),
        "VISION.md",
        "a home declared AT the repo root is never re-rooted — `VISION.md` is unburiable \
         by derivation, not by an allow-list",
    );
    assert_eq!(
        projected_home(&corpus.preview("migrate-changelog"), "changelog"),
        "CHANGELOG.md",
        "the dev pack's root-declared `changelog` is re-rooted by nothing either — the \
         rule reads the DECLARATION, so it holds across both packs",
    );
}

/// **The repo-root value.** `.` re-roots a nested home to the repo root
/// (`docs/roadmap.md` → `roadmap.md`), and it is reachable by setting the empty string —
/// `config set` canonicalizes `""` to `.` exactly as it does for `docs-root`, so the
/// intuitive spelling is not an error. Root-declared homes stay put here too.
#[test]
fn the_repo_root_value_flattens_a_nested_home_and_is_reachable_as_the_empty_string() {
    let corpus = Corpus::new("root");
    corpus.ok(&["config", "set", "placement-root", ""]);

    let resolved = corpus.ok(&["config", "get", "placement-root"]);
    assert!(
        resolved.contains("placement-root = ."),
        "`config set placement-root \"\"` canonicalizes to the `.` repo-root sentinel; got:\n{resolved}",
    );
    assert_eq!(
        projected_home(&corpus.preview("migrate-roadmap"), "roadmap"),
        "roadmap.md",
        "`.` is the repo root: the leading `docs/` component is dropped, nothing prepended",
    );
    assert_eq!(
        projected_home(&corpus.preview("migrate-vision"), "vision"),
        "VISION.md",
        "a root-declared home has no leading component to drop, so `.` leaves it alone",
    );
}

/// **Unset is off, and off is a no-op by VALUE.**
///
/// The default `""` means *unset* — every declared home stands. Asserting only that would
/// prove a skipped branch, not a correct one, so the arm also drives the **fixed point**:
/// `placement-root: docs` re-roots `docs/roadmap.md` to `docs/roadmap.md`, so its whole
/// composed preview must be **byte-identical** to the unset one. A re-root that mangled
/// the remainder, doubled the root, or dropped the trailing segment would pass the
/// contains-check and fail here.
///
/// (The *other* half of the done-criterion — that the unset previews are byte-identical to
/// **HEAD** — is the compose-golden sweep's, which captures these exact invocations against
/// committed goldens; a knob that leaked into the unset path moves those files.)
#[test]
fn the_unset_knob_leaves_every_declared_home_standing() {
    let corpus = Corpus::new("unset");

    let roadmap = corpus.preview("migrate-roadmap");
    let vision = corpus.preview("migrate-vision");
    let changelog = corpus.preview("migrate-changelog");
    assert_eq!(projected_home(&roadmap, "roadmap"), "docs/roadmap.md");
    assert_eq!(projected_home(&vision, "vision"), "VISION.md");
    assert_eq!(projected_home(&changelog, "changelog"), "CHANGELOG.md");

    corpus.ok(&["config", "set", "placement-root", "docs"]);
    assert_eq!(
        corpus.preview("migrate-roadmap"),
        roadmap,
        "re-rooting `docs/roadmap.md` under `docs` is a fixed point — the composed bytes \
         must be identical to the unset composition, not merely contain the same home",
    );
    assert_eq!(corpus.preview("migrate-vision"), vision);
    assert_eq!(corpus.preview("migrate-changelog"), changelog);
}

// -------------------------------------------------------------------------------------
// T2 — the detect + route + MOVE floor: a `placement-root` re-point moves the docs it
// would otherwise strand.
//
// T1 shipped the knob, not the safety. A re-point re-resolves every nested placement
// home while the committed instance stays where it was, so the doc is **stranded**: the
// store goes quiet (`jigc validate` reports nothing — the record still baselines the old
// path and the file still matches it), and every read of the doctype's home now points at
// a file that does not exist. That is the same shape the M39 `config set docs-root` floor
// closed for `location:` doctypes (`crate::config::route_docs_root_repoint_orphans`), and
// this is its placement sibling: same place in `run_set` (BEFORE the knob lands, so the
// prior home resolves off the old cascade), same best-effort posture (a hiccup never fails
// the write), same solicit/act honesty pair.
//
// The arms below drive the **real binary**: the move is asserted on the filesystem, on
// `git status --porcelain`, and by reading the doc back through `jigc doc show` at the
// address that must keep working.
// -------------------------------------------------------------------------------------

/// A conformant, v1-stamped `roadmap` — the byte form a committed methodology corpus
/// carries at the placement home. Bytes are asserted **identical** across the move: a
/// relocation preserves the file, it does not re-author it.
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

/// A conformant, v1-stamped `vision` at the repo-root literal home.
const VISION: &str = "\
---
schema-version: 1
---

# Vision

## Thesis

Structure belongs to the tool.

## Invariants

Prose belongs to the model.

## Open questions

None yet.
";

impl Corpus {
    /// `git <args>` in the corpus repo, asserting success and returning stdout.
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

    /// Write `rel` (creating its parents) and commit it.
    fn commit_file(&self, rel: &str, body: &str) {
        let abs = self.repo.path().join(rel);
        if let Some(parent) = abs.parent() {
            fs::create_dir_all(parent).expect("create parent dir");
        }
        fs::write(&abs, body).expect("write file");
        self.git(&["add", rel]);
        self.git(&["commit", "-q", "-m", &format!("add {rel}")]);
    }

    fn exists(&self, rel: &str) -> bool {
        self.repo.path().join(rel).exists()
    }

    fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.repo.path().join(rel))
            .unwrap_or_else(|err| panic!("reading {rel}: {err}"))
    }

    /// `git status --porcelain`, as lines.
    fn status_lines(&self) -> Vec<String> {
        self.git(&["status", "--porcelain"])
            .lines()
            .map(str::to_owned)
            .collect()
    }
}

/// **The floor.** A committed, baselined `docs/roadmap.md`, then
/// `jigc config set placement-root notes`: the doc is **at** the new home, the move is a
/// staged `git mv` (one `R` line, and only one — the two repo-root placement doctypes
/// contribute none), the read address still resolves, and the store is not quietly wrong.
///
/// The last assertion is the one that separates a *move* from a *rename of the resolved
/// home*: `move_doc` re-keys the file-state record, so `jigc validate` reports **no**
/// file-state finding at the new path. Moving the file without re-keying would leave the
/// recorded path missing — quiet in a different way, and still broken.
#[test]
fn a_repoint_moves_the_committed_doc_it_would_otherwise_strand() {
    let corpus = Corpus::new("strand");
    corpus.commit_file("docs/roadmap.md", ROADMAP);
    corpus.commit_file("VISION.md", VISION);
    corpus.ok(&["ingest"]);

    corpus.ok(&["config", "set", "placement-root", "notes"]);

    assert!(
        corpus.exists("notes/roadmap.md"),
        "the re-point must MOVE the committed instance to the new resolved home",
    );
    assert!(
        !corpus.exists("docs/roadmap.md"),
        "the prior home must be emptied — a copy left behind is a second source of truth",
    );
    assert_eq!(
        corpus.read("notes/roadmap.md"),
        ROADMAP,
        "a relocation preserves the bytes — it never re-authors the doc",
    );
    assert!(
        corpus.exists("VISION.md") && !corpus.exists("notes/VISION.md"),
        "a home declared AT the repo root is not re-rooted, so it is never moved either",
    );

    let status = corpus.status_lines();
    let renames: Vec<&String> = status.iter().filter(|l| l.starts_with('R')).collect();
    assert_eq!(
        renames.len(),
        1,
        "the move lands as ONE staged `git mv` (the operator commits it next); status:\n{}",
        status.join("\n"),
    );
    assert!(
        renames[0].contains("docs/roadmap.md") && renames[0].contains("notes/roadmap.md"),
        "the staged rename names the prior and the new home; got: {}",
        renames[0],
    );

    let shown = corpus.jigc(&["doc", "show", "roadmap:roadmap"]);
    assert!(
        shown.status.success(),
        "the doc must still read back at its address after the re-point; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&shown.stdout),
        String::from_utf8_lossy(&shown.stderr),
    );
    assert!(
        String::from_utf8_lossy(&shown.stdout).contains("The loop closes."),
        "the read-back serves the moved doc's prose",
    );

    let report = corpus.ok(&["validate"]);
    assert!(
        !report.contains("file-state."),
        "the move re-keys the file-state record, so the store carries no file-state \
         finding at the new home; report:\n{report}",
    );
}

/// **The trap the sibling floor cannot hit, and this one can.** Once `placement-root` is
/// `.` the resolved home has **no leading directory component left** (`docs/roadmap.md`
/// → `roadmap.md`), so re-rooting the *old resolved* home under the new value computes a
/// **no-op destination** — the re-root rule reads the declaration, and a root-level file
/// declares nothing to re-root. Both homes must therefore be computed from the DECLARED
/// `placement.file`, never by re-rooting the prior resolved one. The two look equivalent
/// and are not: get it wrong and the doc is silently stranded at the repo root, which is
/// the exact silent-loss shape this floor exists to prevent.
#[test]
fn a_repoint_away_from_the_repo_root_still_finds_the_doc() {
    let corpus = Corpus::new("from-root");
    corpus.commit_file("docs/roadmap.md", ROADMAP);
    corpus.ok(&["ingest"]);

    corpus.ok(&["config", "set", "placement-root", "."]);
    assert!(
        corpus.exists("roadmap.md") && !corpus.exists("docs/roadmap.md"),
        "the flatten re-point moves the doc to the repo root",
    );

    corpus.ok(&["config", "set", "placement-root", "notes"]);
    assert!(
        corpus.exists("notes/roadmap.md"),
        "a re-point AWAY from the repo root must move the doc it strands — computing the \
         destination by re-rooting the prior RESOLVED home yields `roadmap.md`, a silent \
         no-op that leaves the doc behind",
    );
    assert!(
        !corpus.exists("roadmap.md"),
        "the prior (root) home is emptied",
    );

    let shown = corpus.jigc(&["doc", "show", "roadmap:roadmap"]);
    assert!(
        shown.status.success(),
        "the address resolves at the second home too; stderr:\n{}",
        String::from_utf8_lossy(&shown.stderr),
    );
}

/// **A foreign file squatting the destination is displaced, never clobbered.** The move
/// primitive's collision resolution (`design/reconciliation.md` → Relocation collisions)
/// parks an untracked/unmanaged squatter in the gitignored `.jigc/displaced/` workbench so
/// the managed instance can land and no working file is lost — the arm the `docs-root`
/// loop, which calls `move_doc` directly, does not have.
#[test]
fn a_foreign_squatter_at_the_new_home_is_displaced_into_the_workbench() {
    let corpus = Corpus::new("squatter");
    corpus.commit_file("docs/roadmap.md", ROADMAP);
    corpus.ok(&["ingest"]);

    const SQUATTER: &str = "# not the managed roadmap\n";
    let squatter = corpus.repo.path().join("notes/roadmap.md");
    fs::create_dir_all(squatter.parent().expect("parent")).expect("create notes/");
    fs::write(&squatter, SQUATTER).expect("write the squatter");

    corpus.ok(&["config", "set", "placement-root", "notes"]);

    assert_eq!(
        corpus.read("notes/roadmap.md"),
        ROADMAP,
        "the managed instance lands at the destination",
    );
    assert_eq!(
        corpus.read(".jigc/displaced/roadmap.md"),
        SQUATTER,
        "the foreign squatter is moved into the gitignored workbench intact — never \
         clobbered, never committed",
    );
}
