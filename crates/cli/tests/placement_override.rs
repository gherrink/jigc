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

// -------------------------------------------------------------------------------------
// T3 — the 14-site placement census, re-walked against a home that can now DIFFER from its
// declaration.
//
// Every row of the census (`design/storage.md` → Placement — the census) was written when a
// placement doctype's home was a constant: the literal `placement.file` its schema declares.
// A site reading that literal off a raw pack read and a site reading it off the cascade were
// therefore **indistinguishable** — the two could never disagree, so the census could not tell
// them apart and never had to. `placement-root` (T1) makes them able to disagree, which gives
// every row a new axis to answer: **declared, or resolved?**
//
// The arms below drive the axis through the real binary rather than reading it off the source.
// Arm 1 walks the whole door set at an overridden home; arm 2 the compose-time `{{store.<ty>}}`
// resolver (the census's 14th site); arm 3 the store-scope conformance sweep (its 11th); arm 4
// the strand — the state the doctype's home moves and the committed instance does not — with
// the knob written **straight into the manifest**, so T2's move floor never runs and the
// question is only what the doors say about a doc at the other home.
//
// **What arm 4 found, and this task repairs.** The `location:` twin of that state has been
// detected since M36: a `docs-root` re-point that strands `docs/decisions/x.md` draws
// `file-state.orphaned-doc` from `jigc validate` (the self-discovery arm keyed on the
// location-dir basename — the component a `docs-root` re-point leaves invariant). The
// `placement:` twin drew **nothing**: `jigc validate` printed *the committed store validates
// clean* over a stranded, baselined managed doc. That is the very asymmetry this increment
// exists to close, one layer down — the knob shipped, the detector did not follow — so the
// placement self-discovery arm lands here, keyed on the part a `placement-root` re-point leaves
// invariant: the **declared remainder** (`docs/roadmap.md` → `roadmap.md`). A placement doctype
// whose declared home carries no leading directory component (`VISION.md`, `CHANGELOG.md`) is
// not re-rootable and contributes no arm at all — the ecosystem-idiomatic rule's own
// derivation, fenced by the last arm below rather than asserted in a comment.
// -------------------------------------------------------------------------------------

/// The `roadmap` singleton's home as **declared** (`packs/methodology/schemas/roadmap.yaml`).
const DECLARED_HOME: &str = "docs/roadmap.md";
/// The same home as **resolved** under `placement-root: notes` — the only home any door may name.
const RESOLVED_HOME: &str = "notes/roadmap.md";

impl Corpus {
    /// `jigc <args>` with `stdin` piped — the prose write path (`--from-file -`).
    fn jigc_stdin(&self, args: &[&str], stdin: &[u8]) -> std::process::Output {
        use std::io::Write;
        use std::process::Stdio;
        let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the jigc binary");
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(stdin)
            .expect("write stdin");
        child.wait_with_output().expect("wait for jigc")
    }

    /// Set one prose slot through the binary, asserting exit 0.
    fn set_slot(&self, addr: &str, prose: &[u8]) {
        let out = self.jigc_stdin(&["doc", "set-slot", addr, "--from-file", "-"], prose);
        assert!(
            out.status.success(),
            "set-slot {addr} must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }

    /// Both streams of an invocation, joined — the doors below route on stdout *or* stderr.
    fn streams(&self, args: &[&str]) -> String {
        let out = self.jigc(args);
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        )
    }

    /// Overwrite the project layer's `manifest.yaml` verbatim — the hand-edited re-point that
    /// bypasses `jigc config set`, and with it T2's move floor.
    fn write_manifest(&self, body: &str) {
        let config = self.repo.path().join(".jigc").join("config");
        fs::create_dir_all(&config).expect("create the project layer");
        fs::write(config.join("manifest.yaml"), body).expect("write manifest.yaml");
    }

    /// Every path in the `HEAD` tree.
    fn head_paths(&self) -> Vec<String> {
        self.git(&["ls-tree", "-r", "--name-only", "HEAD"])
            .lines()
            .map(str::to_owned)
            .collect()
    }

    /// Author the `planning` workflow's `roadmap` singleton and finalize it — the
    /// create → author → finalize third of the door walk. Returns the task id.
    fn author_and_finalize_roadmap(&self) -> String {
        let task = "plan-the-first-milestone";
        self.ok(&[
            "start",
            "--workflow",
            "planning",
            "plan the first milestone",
        ]);
        assert_eq!(
            self.ok(&[
                "doc", "create", "roadmap", "--title", "Roadmap", "--task", task
            ])
            .trim(),
            "roadmap:roadmap",
            "the singleton mints at the fixed slug",
        );
        let item = self
            .ok(&[
                "doc",
                "add-item",
                "roadmap:roadmap#milestones",
                "--title",
                "M-One",
                "--task",
                task,
            ])
            .trim()
            .to_owned();
        self.set_slot(&format!("{item}/proves"), b"The home follows the knob.\n");
        self.set_slot(&format!("{item}/decomposition"), b"Inc 1, as prose.\n");
        self.ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "docs",
        ]);
        self.ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            "planning",
        ]);
        self.set_slot(&format!("commit:{task}#summary"), b"record the roadmap\n");
        self.set_slot(&format!("commit:{task}#body"), b"One home, everywhere.\n");
        self.ok(&["task", "finalize", task]);
        task.to_owned()
    }
}

/// **The door walk.** With `placement-root: notes` a full create → author → finalize →
/// `doc show` → `doc list` → `validate` → `ingest` → `unmanage` → `migrate-corpus` names
/// `notes/roadmap.md` at **every** door and the declared `docs/roadmap.md` at **none** — the
/// census's `declared or resolved?` axis answered by driving each door rather than by reading
/// which field its source line touches.
#[test]
fn every_door_resolves_the_overridden_placement_home() {
    let corpus = Corpus::new("doors");
    corpus.ok(&["config", "set", "placement-root", "notes"]);
    let task = corpus.author_and_finalize_roadmap();

    // finalize — `finalize::plan_promotions` promoted to the resolved home, and the commit
    // carries it there.
    assert!(
        corpus.head_paths().iter().any(|p| p == RESOLVED_HOME),
        "finalize promotes the created singleton to the RESOLVED home; HEAD holds:\n{:?}",
        corpus.head_paths(),
    );
    assert!(
        !corpus.exists(DECLARED_HOME),
        "nothing is left at the declared home",
    );
    let _ = task;

    // `doc show` — `store::canonical_path`.
    let shown = corpus.ok(&["doc", "show", "roadmap:roadmap"]);
    assert!(
        shown.contains("The home follows the knob."),
        "`doc show` reads the committed singleton at the resolved home; got:\n{shown}",
    );

    // `doc list` — the contract-pinned index read prints the path it resolved.
    let listed = corpus.ok(&["doc", "list"]);
    assert!(
        listed.contains(&format!("roadmap:roadmap  {RESOLVED_HOME}  managed")),
        "`doc list` names the resolved home beside the identity; got:\n{listed}",
    );

    // `validate` — the store-scope sweep (conformance + file-state) over the resolved home.
    let report = corpus.ok(&["validate"]);
    assert!(
        report.contains("validates clean"),
        "the store validates clean with the doc at its resolved home; got:\n{report}",
    );

    // `ingest` — the census surface adopts at the resolved home, no move.
    let census = corpus.ok(&["ingest"]);
    assert!(
        census.contains(&format!("adoptable {RESOLVED_HOME} → roadmap")),
        "`ingest` classifies the doc adoptable AT the resolved home; got:\n{census}",
    );

    // `migrate-corpus` — its placement destination + `exists_in` probe read the resolved home.
    let migrated = corpus.ok(&["migrate-corpus"]);
    assert!(
        migrated.contains(&format!("current    {RESOLVED_HOME}")),
        "`migrate-corpus` reports the singleton already-current AT the resolved home; got:\n{migrated}",
    );

    // `unmanage` — the second path→identity pair (`unmanage::{identity_of, is_placement_file}`).
    let unmanaged = corpus.ok(&["unmanage", RESOLVED_HOME]);
    assert!(
        unmanaged.contains(&format!("unmanaged {RESOLVED_HOME} (roadmap:roadmap)")),
        "`unmanage` recognizes the resolved home as the `roadmap` singleton's identity; got:\n{unmanaged}",
    );

    // No door anywhere named the declaration.
    for door in [
        vec!["doc", "show", "roadmap:roadmap"],
        vec!["doc", "list"],
        vec!["validate"],
        vec!["ingest"],
        vec!["migrate-corpus"],
    ] {
        let out = corpus.streams(&door);
        assert!(
            !out.contains(DECLARED_HOME),
            "`jigc {}` must never name the DECLARED home once the knob re-roots it; got:\n{out}",
            door.join(" "),
        );
    }
}

/// **The 14th census site — the compose-time `{{store.<placement-type>}}` resolver**
/// (`cli::start::committed_store`). It derives a placement doctype's collection key from the
/// type id and its one-element collection from the home, and its failure mode is silence: an
/// absent key renders **empty text**, never a finding. So the arm drives the emitted step body
/// of a project-layer step that names `{{store.roadmap}}` — with the doc at the resolved home
/// the collection carries `roadmap:roadmap`; with the identical doc at the declared home it is
/// empty, which is what a resolver reading the declaration would produce in the mirror image.
#[test]
fn the_compose_time_store_collection_follows_the_overridden_home() {
    let corpus = Corpus::new("store-root");
    corpus.ok(&["config", "set", "placement-root", "notes"]);
    corpus.commit_file(RESOLVED_HOME, ROADMAP);
    corpus.ok(&["ingest"]);

    let step = corpus.home.path().join("probe-store.yaml");
    fs::write(
        &step,
        "The committed roadmap collection:\n\n{{ store.roadmap }}\n",
    )
    .expect("write the probe step");
    corpus.ok(&[
        "config",
        "insert-step",
        "--workflow",
        "planning",
        "--after",
        "plan-scope",
        step.to_str().expect("utf-8 path"),
    ]);

    let preview = corpus.preview("planning");
    assert!(
        preview.contains("The committed roadmap collection:\n\n> roadmap:roadmap\n"),
        "`{{{{store.roadmap}}}}` resolves the singleton at the RESOLVED home; got:\n{preview}",
    );

    // The mirror image: the same doc at the DECLARED home resolves to nothing. Asserted so the
    // arm above cannot pass on a resolver that ignores the home entirely.
    let stray = Corpus::new("store-root-declared");
    stray.ok(&["config", "set", "placement-root", "notes"]);
    stray.commit_file(DECLARED_HOME, ROADMAP);
    fs::write(
        &step,
        "The committed roadmap collection:\n\n{{ store.roadmap }}\n",
    )
    .expect("write the probe step");
    stray.ok(&[
        "config",
        "insert-step",
        "--workflow",
        "planning",
        "--after",
        "plan-scope",
        step.to_str().expect("utf-8 path"),
    ]);
    let stray_preview = stray.preview("planning");
    assert!(
        stray_preview.contains("The committed roadmap collection:"),
        "the probe step still composes; got:\n{stray_preview}",
    );
    // `> <address>` is the collection's own render — the surrounding step bodies name
    // `roadmap:roadmap` as a write address many times over, so the assertion is scoped to the
    // rendered entry, not to the string.
    assert!(
        !stray_preview.contains("> roadmap:roadmap"),
        "a doc at the declared home contributes no `store.roadmap` entry once the knob \
         re-roots the type — the empty case renders empty text, never a fabricated entry; \
         got:\n{stray_preview}",
    );
}

/// **The 11th census site — `validate::schema_conformance_store`**, the detect-half of the
/// corpus migration, enumerated through `index::committed_instances`. A non-conformant file at
/// the **resolved** home draws its `schema-conformance.*` break there and flips the exit; the
/// same bytes at the declared home are, correctly, no longer at a managed home at all — they
/// are a **strand**, and the arm below is where that is adjudicated.
#[test]
fn the_store_conformance_sweep_adjudicates_at_the_overridden_home() {
    let corpus = Corpus::new("conformance");
    corpus.ok(&["config", "set", "placement-root", "notes"]);
    corpus.commit_file(
        RESOLVED_HOME,
        "# Roadmap\n\n## Milestones\n\nnot the schema.\n",
    );

    let out = corpus.jigc(&["validate"]);
    let report = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        report.contains("schema-conformance.") && report.contains(RESOLVED_HOME),
        "the fifth store family walks the RESOLVED home; got:\n{report}",
    );
    assert!(
        !report.contains(DECLARED_HOME),
        "and only the resolved one — a sweep reading the declaration would adjudicate \
         a file that is not there; got:\n{report}",
    );
    assert!(
        !out.status.success(),
        "a non-conformant doc at a managed home flips the exit — the sweep must not \
         green over it",
    );
}

/// **The strand — and the door that used to lie about it.**
///
/// `scalar: {placement-root: notes}` written straight into `.jigc/config/manifest.yaml`
/// bypasses `jigc config set`, so T2's move floor never runs: the doctype's home moves and the
/// committed, baselined instance stays at `docs/roadmap.md`. Before this task `jigc validate`
/// printed *the committed store validates clean* over exactly that state — while the
/// `location:` twin of it (a `docs-root` re-point stranding `docs/decisions/x.md`) has drawn
/// `file-state.orphaned-doc` since M36. Same asymmetry as the knob itself, one layer down.
///
/// Every door is asserted: the strand is detected and routed by `validate` **and** `ingest`,
/// the read surface blocks rather than inventing content, and no door reports a clean store.
#[test]
fn a_hand_edited_repoint_strands_the_committed_doc_and_no_door_reports_it_clean() {
    let corpus = Corpus::new("strand-manifest");
    corpus.commit_file(DECLARED_HOME, ROADMAP);
    corpus.ok(&["ingest"]);
    corpus.write_manifest("scalar:\n  placement-root: notes\n");

    assert!(
        corpus.exists(DECLARED_HOME) && !corpus.exists(RESOLVED_HOME),
        "the hand edit moves nothing — the doc stays where the floor would have moved it from",
    );

    // `validate` — the strand is a finding with a route, never a clean store.
    let report = corpus.ok(&["validate"]);
    assert!(
        !report.contains("validates clean"),
        "the store must NOT validate clean over a baselined doc sitting at the other home; \
         got:\n{report}",
    );
    assert!(
        report.contains("file-state.orphaned-doc") && report.contains(DECLARED_HOME),
        "the strand draws the same registered-tier advisory its `location:` twin draws; \
         got:\n{report}",
    );
    assert!(
        report.contains("placement-root") && report.contains(RESOLVED_HOME),
        "the finding names the knob that moved the home and the home it moved to — a \
         `docs-root` diagnosis here would be a lie; got:\n{report}",
    );
    let route = report
        .lines()
        .find(|l| l.trim_start().starts_with("route:"))
        .unwrap_or_else(|| panic!("the strand advisory carries a route; got:\n{report}"));
    assert!(
        route.contains(RESOLVED_HOME) && route.contains("jigc unmanage"),
        "the route names the home to move to and the way to drop it; got: {route}",
    );

    // `ingest` — the census surface routes the same state at its own door.
    let census = corpus.ok(&["ingest"]);
    assert!(
        census.contains("ingest.wrong-location") && census.contains(RESOLVED_HOME),
        "`ingest` blocks the conformant doc sitting outside the resolved home; got:\n{census}",
    );

    // `doc show` — the read surface blocks at the resolved home rather than serving the
    // stranded bytes from the declaration.
    let shown = corpus.streams(&["doc", "show", "roadmap:roadmap"]);
    assert!(
        shown.contains("store.not-found") && shown.contains(RESOLVED_HOME),
        "the read surface blocks at the resolved home; got:\n{shown}",
    );
}

/// **The never-adopted tier, and the fence that keeps the ecosystem-idiomatic files out of it.**
///
/// Two facts in one arm, because they are the same derivation seen from both sides. A committed
/// file carrying a re-rootable placement doctype's home remainder but **no** baseline is the
/// two-tier route's *unregistered* tier — a coincidence or an un-ingested foreign doc, routed to
/// adoption, never called a tracked strand. And a **root-declared** placement home (`VISION.md`)
/// is not re-rootable at all, so a stray `docs/VISION.md` is not this arm's business: no
/// `placement-root` value could ever have stranded it, and saying one did would be a lie.
#[test]
fn the_unregistered_tier_routes_adoption_and_a_root_declared_home_is_never_a_strand() {
    let corpus = Corpus::new("unregistered");
    corpus.commit_file(DECLARED_HOME, ROADMAP);
    corpus.commit_file("docs/VISION.md", VISION);
    corpus.write_manifest("scalar:\n  placement-root: notes\n");

    let report = corpus.ok(&["validate"]);
    assert!(
        report.contains("file-state.unregistered-doc") && report.contains(DECLARED_HOME),
        "a never-baselined file at a re-rootable placement doctype's prior home is the \
         unregistered tier, not a tracked strand; got:\n{report}",
    );
    assert!(
        report.contains("jigc migrate docs/roadmap.md --as roadmap"),
        "the unregistered tier routes at the adoption verb; got:\n{report}",
    );
    assert!(
        !report.contains("docs/VISION.md"),
        "a root-declared placement home is not re-rootable, so nothing sitting at a \
         `VISION.md`-shaped path anywhere else is a `placement-root` strand; got:\n{report}",
    );
}
