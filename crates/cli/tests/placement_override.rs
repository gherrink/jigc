//! **`placement-root` — the one home no knob could reach** (M49 Increment 7, T1).
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
