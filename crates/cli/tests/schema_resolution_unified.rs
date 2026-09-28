//! **One schema resolution** — no two doors answer differently about the same document
//! (M49 Increment 3, T2).
//!
//! The cascade resolves *all* customization, schemas included: a project-layer whole-file
//! shadow at `.jigc/config/schemas/<ty>.yaml` wins over the pack's copy at file
//! granularity (`design/overrides.md` → Resolution algorithm phase 2;
//! `design/storage.md` → Config layout). Exactly one production surface obeyed that —
//! `CascadeDefs::all_schemas`, which `describe`, compose, `doc show`, `doc schema` and the
//! store sweep read through. **Seven others read the pack directly**, shadow-blind:
//! `ActiveTask::schema` / `::schemas`, `doc::is_singleton_type`, `TaskCtx::schemas`,
//! `ingest::load_schemas`, `milestone::shipped_schemas` and `start::load_commit_schema`.
//!
//! The consequence is not a missing feature; it is **two doors of one binary giving
//! contradictory answers about one file**, and each arm below drives that contradiction
//! through the real binary rather than asserting a resolution result in-process:
//!
//! - `jigc validate` adjudicates the fields of a document `jigc ingest` reports as
//!   matching no schema at all (the headline);
//! - `jigc doc create` over a doc the store already holds mints a **blank skeleton**
//!   instead of copying the committed prose in;
//! - `jigc task finalize` promotes to a home `jigc doc show` then cannot read, while
//!   `jigc validate` calls that same store *clean*;
//! - `jigc milestone add-from-spec` cannot read the spec `jigc doc show` just printed;
//! - the provisioned commit form omits a field `jigc doc schema commit` lists;
//! - a bare address is refused for a doctype the resolved cascade makes a singleton.
//!
//! **The pack under test ships no freeze manifest.** A `location:` (and a `placement:`)
//! is inside the `schema-hash` since M38, so re-shaping a *manifest-governed* doctype
//! from the project layer is refused at pack-load since T1 — the freeze binds at every
//! layer that can change a schema. What survives, and is what these arms exercise, is the
//! cascade itself: a pack that declares nothing frozen freezes nothing at either layer,
//! so its doctypes' homes and shapes are the project's to change
//! ([`manifest_less_dev_pack`](crate::support::frozen_pack::manifest_less_dev_pack)).
//! **The order is forced**: unified before T1, a shape-changing shadow would have driven
//! `doc create`, the finalize conformance sweep and the promote destination before
//! anything refused it.
//!
//! The last arm is the **source-level fence**: the property is checked where membership
//! is decided, so the eighth shadow-blind surface reddens when it is written rather than
//! when a trial finds it (the `test_target_registration` / `temp_mint_fence` precedent).

use crate::support::frozen_pack;
use crate::support::run_then_parse::stdout_json;
use crate::support::rust_source;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-schema-resolution-{tag}-{}-{:?}",
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

/// A set-up corpus over a **manifest-less** on-disk dev pack: a real git repo with one
/// commit, `jigc setup` run, and a private `$HOME`.
struct Corpus {
    repo: TempDir,
    home: TempDir,
    pack: TempDir,
}

impl Corpus {
    fn new(tag: &str) -> Self {
        let repo = TempDir::new(tag);
        let home = TempDir::new(&format!("{tag}-home"));
        let pack = TempDir::new(&format!("{tag}-pack"));
        frozen_pack::manifest_less_dev_pack(pack.path());

        git(repo.path(), &["init", "-q"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-q", "-m", "initial"]);

        let corpus = Self { repo, home, pack };
        let out = corpus.jigc(&["setup"]);
        assert_ok(&out, "`jigc setup`");
        corpus
    }

    fn repo(&self) -> &Path {
        self.repo.path()
    }

    /// The pack's own on-disk copy of doctype `ty`'s schema — the body a shadow starts
    /// from, so a shadow is the shipped schema with one thing changed rather than a
    /// hand-copied approximation that can drift from it.
    fn pack_schema(&self, ty: &str) -> String {
        fs::read_to_string(self.pack.path().join("schemas").join(format!("{ty}.yaml")))
            .expect("read the pack schema")
    }

    /// Write the project-layer whole-file schema shadow for `ty`.
    fn shadow_schema(&self, ty: &str, body: &str) {
        let dir = self.repo().join(".jigc").join("config").join("schemas");
        fs::create_dir_all(&dir).expect("create the project schemas dir");
        fs::write(dir.join(format!("{ty}.yaml")), body).expect("write the schema shadow");
    }

    /// Commit `path` (repo-relative) with `body`, creating parent directories.
    fn commit_file(&self, path: &str, body: &str) {
        let full = self.repo().join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).expect("create parent dir");
        }
        fs::write(&full, body).expect("write the file");
        git(self.repo(), &["add", "-A"]);
        git(self.repo(), &["commit", "-q", "-m", "seed"]);
    }

    /// Run `jigc <args>` against this corpus, with the pack selected by `JIGC_PACK_DIR`.
    fn jigc(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo())
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", self.pack.path())
            .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
            .output()
            .expect("run the jigc binary")
    }

    /// `jigc <args>`, asserting success and returning stdout.
    fn ok(&self, args: &[&str]) -> String {
        let out = self.jigc(args);
        assert_ok(&out, &format!("`jigc {}`", args.join(" ")));
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    /// The single active task's id.
    fn task_id(&self) -> String {
        let tasks = self.repo().join(".jigc").join("tasks");
        let mut ids: Vec<String> = fs::read_dir(&tasks)
            .expect("read the tasks dir")
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        ids.sort();
        assert_eq!(
            ids.len(),
            1,
            "expected exactly one active task, got {ids:?}"
        );
        ids.remove(0)
    }
}

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path.
fn doc_code_probe() -> &'static Path {
    use std::sync::OnceLock;
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
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

fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A conformant `adr` body — every required section of the shipped schema filled.
fn adr_body(title: &str, context: &str) -> String {
    format!(
        "---\n\
         status: accepted\n\
         date: 2026-06-14\n\
         ---\n\
         \n\
         # {title}\n\
         \n\
         ## Context\n\
         {context}\n\
         \n\
         ## Options\n\
         Weighed.\n\
         \n\
         ## Decision\n\
         Decided.\n\
         \n\
         ## Consequences\n\
         Effects.\n"
    )
}

/// Fill an adr's three author-required slots and the task's commit doc, so a finalize
/// over the task validates clean.
fn author_adr_and_commit(corpus: &Corpus, slug: &str, task: &str) {
    let prose = corpus.repo().join(".prose.txt");
    fs::write(&prose, "Prose.\n").expect("write the slot payload");
    let prose = prose.to_string_lossy().into_owned();
    for section in ["context", "decision", "consequences"] {
        corpus.ok(&[
            "doc",
            "set-slot",
            &format!("adr:{slug}#{section}"),
            "--from-file",
            &prose,
            "--task",
            task,
        ]);
    }
    corpus.ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#header/type"),
        "--value",
        "docs",
        "--task",
        task,
    ]);
    corpus.ok(&[
        "doc",
        "set-slot",
        &format!("commit:{task}#summary"),
        "--from-file",
        &prose,
        "--task",
        task,
    ]);
    fs::remove_file(corpus.repo().join(".prose.txt")).expect("drop the slot payload");
}

/// The `adr` schema with its home relocated `decisions/` → `adrs/` — the shadow four
/// arms share, and the one a manifest-less pack makes legal.
fn adr_relocated(corpus: &Corpus) -> String {
    let body = corpus.pack_schema("adr");
    let shadow = body.replace("location: decisions/", "location: adrs/");
    assert_ne!(body, shadow, "the shadow must actually relocate `adr`");
    shadow
}

// ---------------------------------------------------------------------------------
// Arm 1 — ingest classification (`ingest::load_schemas`), the headline divergence
// ---------------------------------------------------------------------------------

/// `jigc ingest` and `jigc validate` reach the **same verdict** about the document at the
/// project-shadowed home.
///
/// Before this task they did not, and the split was maximal: `validate`, which resolves
/// schemas through the cascade, walked `adrs/` and adjudicated the file *as an `adr`*
/// (its fields, its sections, its baseline), while `ingest`, which read the pack
/// directly, reported the very same path as `unmanaged — parses against no schema`. One
/// binary, one file, one committed corpus, two contradictory answers — and the `ingest`
/// half is the one an adopter is told to trust when deciding whether a file is managed.
#[test]
fn ingest_and_validate_agree_about_the_document_at_the_shadowed_home() {
    let corpus = Corpus::new("ingest-agree");
    corpus.shadow_schema("adr", &adr_relocated(&corpus));
    corpus.commit_file(
        "docs/adrs/cache.md",
        &adr_body("Cache", "The committed forces."),
    );

    let validate = corpus.ok(&["validate"]);
    assert!(
        validate.contains("committed doc `docs/adrs/cache.md`"),
        "`jigc validate` must adjudicate the doc at the shadowed home as a managed doc; \
         stdout:\n{validate}",
    );

    let ingest = corpus.ok(&["ingest"]);
    assert!(
        ingest.contains("adoptable docs/adrs/cache.md \u{2192} adr"),
        "`jigc ingest` must reach `jigc validate`'s verdict about the same file — a \
         conformant `adr` at its resolved home, adopted register-only; ingest:\n{ingest}\n\
         validate:\n{validate}",
    );
    assert!(
        !ingest.contains("ingest.wrong-location"),
        "`jigc ingest` must not route the operator to move a document OUT of the home the \
         resolved cascade declares for it — the route, followed, would move it out of the \
         store `validate` and `doc show` read; ingest:\n{ingest}",
    );
}

// ---------------------------------------------------------------------------------
// Arm 2 — create / write (`ActiveTask::schema`)
// ---------------------------------------------------------------------------------

/// `jigc doc create` over a doc the committed store already holds **copies it in**, at
/// the resolved home.
///
/// Copy-on-first-touch resolves the committed `<location>/<slug>.md` through the doc
/// verb's own schema read (`design/write-commands.md` → copy-on-first-touch; M43's
/// `existed` ack). Shadow-blind, that read looked in the pack's `decisions/`, missed, and
/// acked a **fresh mint** — so the task staged a blank skeleton over a document whose
/// authored prose the store still held, and every later write in that task edited the
/// blank one.
#[test]
fn create_over_a_committed_doc_at_the_shadowed_home_copies_it_in() {
    let corpus = Corpus::new("create-copy-in");
    corpus.shadow_schema("adr", &adr_relocated(&corpus));
    corpus.commit_file(
        "docs/adrs/cache.md",
        &adr_body("Cache", "The committed forces."),
    );

    corpus.ok(&["start", "record a decision", "--workflow", "single-task"]);
    let task = corpus.task_id();
    let ack: serde_json::Value = stdout_json(
        &corpus.jigc(&[
            "doc", "create", "adr", "--title", "Cache", "--task", &task, "--format", "json",
        ]),
        &[0],
        "`jigc doc create adr --format json`",
    );
    assert_eq!(
        ack["existed"], true,
        "`doc create` must see the committed doc at the resolved home; ack:\n{ack:#}",
    );
    let staged = corpus.ok(&["doc", "show", "adr:cache", "--task", &task]);
    assert!(
        staged.contains("The committed forces."),
        "the staged copy must carry the committed prose, not a blank skeleton — a create \
         that misses the committed doc stages a blank one over it and every later write \
         in the task edits the blank; staged:\n{staged}",
    );
}

// ---------------------------------------------------------------------------------
// Arm 3 — task validate + the promote destination (`TaskCtx::schemas`)
// ---------------------------------------------------------------------------------

/// `jigc task validate` names, and `jigc task finalize` promotes to, the **resolved**
/// home — so the doc the finalize lands is the doc `jigc doc show` reads back.
///
/// This was the most destructive split of the seven. `task validate` reported the staged
/// copy as `docs/decisions/<slug>.md`, finalize promoted it there, and the read surfaces
/// then looked in `docs/adrs/` — `doc show` blocked with `store.not-found` on a document
/// that had just been committed, and `jigc validate` reported *"the committed store
/// validates clean"* over a store holding a managed document it could not see.
#[test]
fn finalize_promotes_to_the_resolved_home_the_read_surfaces_use() {
    let corpus = Corpus::new("promote-home");
    corpus.shadow_schema("adr", &adr_relocated(&corpus));
    git(corpus.repo(), &["add", "-A"]);
    git(corpus.repo(), &["commit", "-q", "-m", "the schema shadow"]);

    corpus.ok(&["start", "record a decision", "--workflow", "single-task"]);
    let task = corpus.task_id();
    corpus.ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Retry policy",
        "--task",
        &task,
    ]);
    author_adr_and_commit(&corpus, "retry-policy", &task);

    let preview = corpus.ok(&["task", "validate", &task]);
    assert!(
        preview.contains("docs/adrs/retry-policy.md"),
        "`jigc task validate` must name the resolved home; stdout:\n{preview}",
    );
    assert!(
        !preview.contains("docs/decisions/"),
        "`jigc task validate` must not name the shadowed-away pack home; stdout:\n{preview}",
    );

    let finalized = corpus.ok(&["task", "finalize", &task]);
    assert!(
        finalized.contains("promoted docs/adrs/retry-policy.md"),
        "finalize must promote to the resolved home; stdout:\n{finalized}",
    );
    assert!(
        corpus.repo().join("docs/adrs/retry-policy.md").is_file(),
        "the promoted file must land at the resolved home",
    );

    // The read surfaces answer about the same file the write path just landed.
    let shown = corpus.ok(&["doc", "show", "adr:retry-policy"]);
    assert!(
        shown.contains("# Retry policy"),
        "`jigc doc show` must read back the doc finalize just promoted; stdout:\n{shown}",
    );
}

// ---------------------------------------------------------------------------------
// Arm 4 — the milestone shipped set (`milestone::shipped_schemas`)
// ---------------------------------------------------------------------------------

/// `jigc milestone add-from-spec` reads the spec through the same resolution
/// `jigc doc show` reads it through.
///
/// The milestone surface resolves a committed doc by address for the seeding read (and
/// for the join's committed-index rebuild). Shadow-blind, `doc show spec:widget` printed
/// the document while `milestone add-from-spec … spec:widget` blocked with
/// `store.not-found` at the pack home — the same address, the same binary, two answers.
#[test]
fn milestone_add_from_spec_reads_the_spec_at_the_resolved_home() {
    let corpus = Corpus::new("milestone-spec");
    let body = corpus.pack_schema("spec");
    let shadow = body.replace("location: specs/", "location: specifications/");
    assert_ne!(body, shadow, "the shadow must actually relocate `spec`");
    corpus.shadow_schema("spec", &shadow);

    corpus.commit_file(
        "docs/specifications/widget.md",
        "---\n\
         derived-from: \n\
         ---\n\
         \n\
         # Widget\n\
         \n\
         ## Goal\n\
         Ship a widget.\n\
         \n\
         ## Context\n\
         Because.\n\
         \n\
         ## Criteria\n\
         \n\
         ### The widget renders {#the-widget-renders}\n\
         \n\
         maps-to-test: \n\
         \n\
         The widget renders on screen.\n",
    );

    // The read surface resolves the address.
    corpus.ok(&["doc", "show", "spec:widget"]);

    corpus.ok(&["milestone", "create", "Widget wave"]);
    let seeded = corpus.ok(&["milestone", "add-from-spec", "widget-wave", "spec:widget"]);
    assert!(
        seeded.contains("seeded 1 sub-task(s)"),
        "`milestone add-from-spec` must read the spec `doc show` just printed; stdout:\n{seeded}",
    );
}

// ---------------------------------------------------------------------------------
// Arm 5 — commit-doc provisioning (`start::load_commit_schema`)
// ---------------------------------------------------------------------------------

/// The commit form `jigc start` provisions carries the fields `jigc doc schema commit`
/// says the commit doctype has.
///
/// `commit` is transient — no home to relocate — so the split shows on **shape**: a
/// project shadow adding a header field was projected by `doc schema` and absent from the
/// form the agent is handed, which is the one surface that decides whether the field can
/// ever be filled (front-matter has no generate-a-line path, so an un-stamped field is
/// unwritable).
#[test]
fn the_provisioned_commit_form_carries_the_resolved_commit_schema() {
    let corpus = Corpus::new("commit-form");
    let body = corpus.pack_schema("commit");
    let anchor = "      - { id: scope, type: string, optional: true }";
    assert!(
        body.contains(anchor),
        "the pack commit schema declares `scope`"
    );
    let shadow = body.replace(
        anchor,
        &format!("{anchor}\n      - {{ id: ticket, type: string, optional: true }}"),
    );
    corpus.shadow_schema("commit", &shadow);

    let projected = corpus.ok(&["doc", "schema", "commit"]);
    assert!(
        projected.contains("ticket:"),
        "`doc schema commit` projects the shadowed field; stdout:\n{projected}",
    );

    corpus.ok(&["start", "add a widget", "--workflow", "single-task"]);
    let task = corpus.task_id();
    let form = corpus.ok(&["doc", "show", &format!("commit:{task}"), "--task", &task]);
    assert!(
        form.contains("ticket:"),
        "the provisioned commit form must carry the field `doc schema` projects — \
         front-matter has no generate-a-line path, so a field absent from the form can \
         never be filled; form:\n{form}",
    );
}

// ---------------------------------------------------------------------------------
// Arm 6 — address normalization (`doc::is_singleton_type`)
// ---------------------------------------------------------------------------------

/// A bare address expands for a doctype the **resolved** cascade makes a singleton.
///
/// The bare-singleton expansion asks whether the doctype homes at a literal file; that
/// question was answered from the pack alone, so a project shadow that made a doctype a
/// placement singleton left `jigc doc show adr` refused as a malformed address while
/// `jigc doc show adr:adr` — the spelling the expansion exists to save the caller from —
/// printed the document.
#[test]
fn a_bare_address_expands_for_a_doctype_the_cascade_makes_a_singleton() {
    let corpus = Corpus::new("bare-address");
    let body = corpus.pack_schema("adr");
    let shadow = body.replace(
        "location: decisions/",
        "placement: { file: docs/DECISION.md }\nsingleton: true\ndisplay-title: The decision",
    );
    assert_ne!(
        body, shadow,
        "the shadow must actually make `adr` a singleton"
    );
    corpus.shadow_schema("adr", &shadow);
    corpus.commit_file(
        "docs/DECISION.md",
        &adr_body("The decision", "The committed forces."),
    );

    let explicit = corpus.ok(&["doc", "show", "adr:adr"]);
    let bare = corpus.jigc(&["doc", "show", "adr"]);
    assert!(
        bare.status.success(),
        "a bare address must expand for a doctype the resolved cascade makes a \
         singleton — `adr:adr` reads it; stderr:\n{}",
        String::from_utf8_lossy(&bare.stderr),
    );
    assert_eq!(
        String::from_utf8_lossy(&bare.stdout),
        explicit,
        "the bare and explicit spellings must read the same document",
    );
}

// ---------------------------------------------------------------------------------
// Arm 7 — the source-level fence
// ---------------------------------------------------------------------------------

/// The `crates/cli` production tree.
fn cli_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Calls that read a pack resource's **bytes** — the act that resolves a schema, as
/// opposed to `list`ing ids or naming the kind in a const.
const READ_CALLEES: &[&str] = &["read", "read_pack", "read_text"];

/// Where a production schema read is allowed to live.
///
/// `pack.rs` is the pack layer itself — the loader, the manifest reads and the freeze
/// gate, which by construction hash *each pack's own bytes* and must not go through the
/// cascade. Everything else resolves a schema for a **document**, and there is exactly
/// one function for that.
const ALLOWED: &[(&str, &str)] = &[("pack.rs", "*"), ("start.rs", "read_one")];

/// Every production schema read in `crates/cli/src` goes through the one resolver.
///
/// The seven surfaces this task unified were not written carelessly — each was a local,
/// obvious `pack.read(Schemas, …)`, and the next one would be too. So the property is
/// checked where membership is decided rather than by the list of sites that happened to
/// exist ([dev-workflow.md] → *a grep is not a fence*): a new direct read reddens here,
/// naming itself, instead of shipping and being found by a trial.
#[test]
fn every_production_schema_read_goes_through_the_shared_resolver() {
    const KIND: &str = "PackResourceKind::Schemas";
    let root = cli_src();
    let mut offenders = Vec::new();
    let mut inspected = 0usize;

    for path in rust_source::rust_files(&root) {
        let body = fs::read_to_string(&path).expect("read a cli source");
        let code = rust_source::code_only(&body);
        let regions = rust_source::cfg_test_regions(&code);
        let file = path
            .file_name()
            .expect("a source file has a name")
            .to_string_lossy()
            .into_owned();

        for (at, _) in code.match_indices(KIND) {
            if rust_source::is_test_domain(&path, &regions, at) {
                continue;
            }
            let Some(callee) = rust_source::enclosing_callee(&code, at) else {
                continue;
            };
            if !READ_CALLEES.contains(&callee) {
                continue;
            }
            inspected += 1;
            let owner = rust_source::enclosing_fn(&code, at).unwrap_or("<top level>");
            if ALLOWED
                .iter()
                .any(|(f, fun)| *f == file && (*fun == "*" || *fun == owner))
            {
                continue;
            }
            let line = code[..at].lines().count();
            offenders.push(format!("  {file}:{line}: in `{owner}` via `{callee}(…)`"));
        }
    }

    assert!(
        inspected >= 3,
        "the fence must actually find the production schema reads; it saw only \
         {inspected} — the read discriminator has drifted",
    );
    assert!(
        offenders.is_empty(),
        "a production schema read must go through the shared cascade resolver \
         (`start::CascadeDefs`, reached by `start::resolved_schema` / \
         `start::resolved_schemas`) — reading the pack directly is shadow-blind, so the \
         reading door disagrees with every door that resolves.\n\
         {} offending read(s):\n{}",
        offenders.len(),
        offenders.join("\n"),
    );
}

/// The fence's two discriminators really discriminate — calibrated on this repo's own
/// shapes, so a fence that had gone vacuous fails here rather than passing silently.
#[test]
fn the_fence_discriminators_separate_reads_from_mentions() {
    let code = rust_source::code_only(
        "const C: &[(&str, PackResourceKind, &str)] = &[(\"x\", PackResourceKind::Schemas, \"c\")];\n\
         fn resolve(pack: &dyn P) { let _ = pack.list(PackResourceKind::Schemas); }\n\
         fn read_one(pack: &dyn P) { let _ = read_pack(pack, PackResourceKind::Schemas, id); }\n",
    );
    let sites: Vec<(Option<&str>, Option<&str>)> = code
        .match_indices("PackResourceKind::Schemas")
        .map(|(at, _)| {
            (
                rust_source::enclosing_callee(&code, at),
                rust_source::enclosing_fn(&code, at),
            )
        })
        .collect();

    assert_eq!(sites.len(), 3, "three mentions in the fixture");
    assert_eq!(
        sites[0].0, None,
        "a const tuple naming the kind is not a read",
    );
    assert_eq!(
        sites[1].0,
        Some("list"),
        "`list` enumerates ids, it reads no bytes"
    );
    assert_eq!(
        (sites[2].0, sites[2].1),
        (Some("read_pack"), Some("read_one")),
        "a real read is attributed to its callee and its owning function",
    );
}
