//! M49 Increment 5, T4 — **`jigc doc add-item --slug`: an item's id can be decoupled
//! from its title**, driven through the real binary.
//!
//! The limit this closes was found by driving: an item's `{#id}` was `slugify(title)`
//! and nothing else, so two genuinely different headings that slug alike could not
//! both exist — the second mint hit `write.already-present` and the whole batch it
//! rode in was refused. `jigc start`, `jigc doc create` and `jigc migrate` have all
//! carried a `--slug` override since M39; `add-item`, the fourth mint door, did not
//! (`design/write-commands.md` → `jigc rename`'s `--slug` precedent).
//!
//! Every arm drives the **emitted** address verbatim — the address `add-item` prints
//! is the one the follow-up read runs, never a reconstructed equivalent, because the
//! acked address was itself derived from `slugify(title)` and is exactly what an
//! override must move.
//!
//! The arms:
//!
//! 1. **The headline** — two genuinely different component titles that slug alike are
//!    both minted (the second with `--slug`), the collision between them is shown to
//!    be real (the un-overridden second mint is refused), and each item reads back at
//!    ITS OWN address through `doc show --format json` with `id` equal to the override
//!    and its own heading text intact.
//! 2. **The nesting cap** — an override mints at nesting **depth 2**
//!    ([`engine::schema::MAX_NESTING_DEPTH`], the number T1 of this increment derived
//!    from the address-hop budget). No shipped doctype declares a depth-2 block with a
//!    non-enum `id-from` (`changelog`'s only nested block is `id-from: category`, which
//!    arm 4 refuses by design), so this arm drives a **mutated dev-pack copy** whose
//!    `arch-doc` component block gains a nested `notes` repeatable — the
//!    `corpus_migration_backstop` pattern, with the manifest re-pinned through the
//!    tool's own loader rather than a hand-typed hash.
//! 3. **The malformed override** — rejected before any bytes move (the staged doc is
//!    byte-identical afterwards), never silently re-slugified: the discipline
//!    `doc create --slug` / `migrate --slug` / `start --slug` already carry.
//! 4. **The enum `id-from`** — refused with a route. Where the block's id-source is an
//!    enum (`changelog#unreleased-changes`, `id-from: category`) the heading IS the
//!    member and the anchor equals it, so an override is an identity change, not an
//!    id: it converges on the shipped `write.identity-change`, the code
//!    `retitle-item`'s enum refusal already produces.
//! 5. **Retitle stays retitle** — `doc retitle-item` over an override-minted item
//!    leaves the anchor frozen at the override, so the identity `--slug` chose
//!    survives the one verb that moves heading text.
//!
//! No external test crates: the binary comes from `CARGO_BIN_EXE_jigc`, the repo is a
//! real `git init`, and the `TempDir`s clean themselves up.

use crate::support::run_then_parse::stdout_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-add-item-slug-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — the faithful source arm 2's mutated copy mirrors.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path —
/// `arch-doc`'s `implemented-by` anchors resolve through it (mirrors
/// `retitle_item::doc_code_probe`).
fn doc_code_probe() -> &'static Path {
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

/// A real git repo with one commit, a `src/lib.rs` carrying a resolvable symbol, and
/// the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.join("src")).expect("mk src");
    fs::write(
        repo.join("src").join("lib.rs"),
        "pub fn present_symbol() -> u32 {\n    42\n}\n",
    )
    .expect("write lib.rs");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the doc-code probe selected and
/// — when `pack` is `Some` — an explicit `JIGC_PACK_DIR`. `None` runs the **shipped**
/// embedded pack, so every arm but the nesting one is proven against what ships.
fn jigc(repo: &Path, home: &Path, pack: Option<&Path>, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(dir) = pack {
        command.env("JIGC_PACK_DIR", dir);
    }
    let child = command.spawn().expect("spawn the jigc binary");
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing both streams on failure, returning
/// trimmed stdout.
fn ok_stdout(out: Output, what: &str) -> String {
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

/// Assert a `jigc` invocation was refused, returning its combined streams (findings
/// print on stderr; the shape assertions read both).
fn refused(out: Output, what: &str) -> String {
    assert!(
        !out.status.success(),
        "`{what}` must exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// The staged bytes of `<type>:<slug>` in `task`'s working area — the before/after
/// witness for "no bytes moved".
fn staged_doc(repo: &Path, task: &str, addr: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("{addr}.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// `doc show <addr> --task <task> --format json`, parsed — the read-back every mint arm
/// runs over the address `add-item` EMITTED.
fn show_json(
    repo: &Path,
    home: &Path,
    pack: Option<&Path>,
    addr: &str,
    task: &str,
) -> serde_json::Value {
    stdout_json(
        &jigc(
            repo,
            home,
            pack,
            &["doc", "show", addr, "--task", task, "--format", "json"],
        ),
        &[0],
        &format!("doc show {addr} --task {task} --format json"),
    )
}

/// The arch-doc task both shipped-pack arms open: start the architecture workflow and
/// create `arch-doc:gateway`. Returns the task id.
fn open_arch_doc(repo: &Path, home: &Path, pack: Option<&Path>) -> &'static str {
    let task = "document-the-gateway";
    ok_stdout(
        jigc(
            repo,
            home,
            pack,
            &[
                "start",
                "--workflow",
                "architecture-documentation",
                "document the gateway",
            ],
        ),
        "jigc start --workflow architecture-documentation",
    );
    assert!(
        repo.join(".jigc").join("tasks").join(task).is_dir(),
        "the intent must mint the task id this suite drives (`{task}`)",
    );
    let created = ok_stdout(
        jigc(
            repo,
            home,
            pack,
            &["doc", "create", "arch-doc", "--title", "Gateway"],
        ),
        "jigc doc create arch-doc",
    );
    assert_eq!(created, "arch-doc:gateway", "the minted arch-doc address");
    task
}

// ---------------------------------------------------------------------------
// Arm 1 — the headline
// ---------------------------------------------------------------------------

/// Two genuinely different component titles that slug alike are both minted — the
/// second through `--slug` — and each reads back at ITS OWN emitted address with `id`
/// equal to the override and its own heading text intact.
///
/// The collision is **shown, not assumed**: the un-overridden second mint is driven
/// first and refused at the id the first mint took, so the arm proves the override
/// removed a real limit rather than decorating a case that never collided.
#[test]
fn two_titles_that_slug_alike_both_mint_when_the_second_supplies_a_slug() {
    let repo = TempDir::new("headline-repo");
    let home = TempDir::new("headline-home");
    init_repo(repo.path());
    let (repo, home) = (repo.path(), home.path());
    let task = open_arch_doc(repo, home, None);

    let first = ok_stdout(
        jigc(
            repo,
            home,
            None,
            &[
                "doc",
                "add-item",
                "arch-doc:gateway#components",
                "--title",
                "Retry policy (v2)",
            ],
        ),
        "add-item `Retry policy (v2)`",
    );
    assert_eq!(
        first, "arch-doc:gateway#components/retry-policy-v2",
        "the first mint's emitted address",
    );

    // The limit itself: a DIFFERENT title that slugs to the same id is refused at that id.
    let collision = refused(
        jigc(
            repo,
            home,
            None,
            &[
                "doc",
                "add-item",
                "arch-doc:gateway#components",
                "--title",
                "Retry policy v2",
            ],
        ),
        "add-item `Retry policy v2` (no override)",
    );
    assert!(
        collision.contains("retry-policy-v2"),
        "the collision names the id both titles slug to; got:\n{collision}",
    );

    let second = ok_stdout(
        jigc(
            repo,
            home,
            None,
            &[
                "doc",
                "add-item",
                "arch-doc:gateway#components",
                "--title",
                "Retry policy v2",
                "--slug",
                "retry-policy-v2-plain",
            ],
        ),
        "add-item `Retry policy v2` --slug retry-policy-v2-plain",
    );
    assert_eq!(
        second, "arch-doc:gateway#components/retry-policy-v2-plain",
        "the acked address carries the OVERRIDE, not `slugify(title)`",
    );

    // Each item reads back at its own EMITTED address, driven verbatim.
    let one = show_json(repo, home, None, &first, task);
    assert_eq!(one["id"], "retry-policy-v2");
    assert_eq!(one["title"], "Retry policy (v2)");
    let two = show_json(repo, home, None, &second, task);
    assert_eq!(
        two["id"], "retry-policy-v2-plain",
        "the read-back id IS the override",
    );
    assert_eq!(
        two["title"], "Retry policy v2",
        "the heading keeps its own text — the override moved the id, not the title",
    );
}

// ---------------------------------------------------------------------------
// Arm 2 — the nesting cap
// ---------------------------------------------------------------------------

/// A dev-pack copy whose `arch-doc` component block gains a nested `notes` repeatable
/// with a **string** `id-from` — a depth-2 block an override can mint into. The shipped
/// registry has no such block (`changelog`'s only nested block is `id-from: category`,
/// which arm 4 refuses by design), so the depth-2 override is unreachable without one.
///
/// The manifest is re-pinned through the tool's **own** loader
/// ([`cli::pack::load_pack_schema`] + [`engine::manifest::schema_hash`], the exact pair
/// the pack-load freeze gate runs), never a hand-typed hash — a copied literal is how a
/// fixture silently stops being the shape it claims.
fn pack_with_a_nested_arch_doc_block(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&dev_pack(), dir.path());

    let schema_path = dir.path().join("schemas").join("arch-doc.yaml");
    let current = fs::read_to_string(&schema_path).expect("read the copied arch-doc.yaml");
    let anchor = "        - { id: implemented-by, type: code-anchor, title-names-symbol: true }\n";
    assert!(
        current.contains(anchor),
        "arch-doc must declare the component code-anchor leaf this fixture nests under",
    );
    const NESTED_BLOCK: &str = concat!(
        "        - id: notes\n",
        "          repeatable:\n",
        "            id-from: title\n",
        "            block:\n",
        "              - { id: title, type: string }\n",
        "              - { id: detail, slot: { hint: \"One note.\" } }\n",
    );
    fs::write(
        &schema_path,
        current.replace(anchor, &format!("{anchor}{NESTED_BLOCK}")),
    )
    .expect("write the nested arch-doc.yaml");

    let bytes = fs::read(&schema_path).expect("re-read the mutated arch-doc.yaml");
    let pack = cli::pack::FilesystemPack::new(dir.path().to_path_buf());
    let schema = cli::pack::load_pack_schema(&pack, &bytes).expect("the nested arch-doc loads");
    let hash = engine::manifest::schema_hash(&schema);

    let manifest_path = dir.path().join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let head = "  - type: arch-doc\n    schema-version: 1\n    schema-hash: ";
    let at = manifest
        .find(head)
        .expect("the manifest declares arch-doc at schema-version 1");
    let start = at + head.len();
    let end = start + manifest[start..].find('\n').expect("the hash line ends");
    fs::write(
        &manifest_path,
        format!("{}{hash}{}", &manifest[..start], &manifest[end..]),
    )
    .expect("write the re-pinned manifest");

    dir
}

/// An override mints a **nested** item at nesting depth 2 — the addressable cap — and
/// the emitted section-qualified chain reads back with `id` equal to the override.
#[test]
fn an_override_mints_a_nested_item_at_the_depth_cap() {
    assert_eq!(
        engine::schema::MAX_NESTING_DEPTH,
        2,
        "this arm drives the cap itself; if the derived cap moves, so must the fixture",
    );
    let pack = pack_with_a_nested_arch_doc_block("nested-pack");
    let repo = TempDir::new("nested-repo");
    let home = TempDir::new("nested-home");
    init_repo(repo.path());
    let (repo, home, pack) = (repo.path(), home.path(), Some(pack.path()));
    let task = open_arch_doc(repo, home, pack);

    let component = ok_stdout(
        jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                "arch-doc:gateway#components",
                "--title",
                "Retry policy (v2)",
            ],
        ),
        "add-item the parent component",
    );

    let note = ok_stdout(
        jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                &format!("{component}/notes"),
                "--title",
                "Retry policy v2",
                "--slug",
                "retry-policy-v2-plain",
            ],
        ),
        "nested add-item --slug",
    );
    assert_eq!(
        note,
        format!("{component}/notes/retry-policy-v2-plain"),
        "the nested acked chain carries the OVERRIDE as its leaf-most id",
    );

    let json = show_json(repo, home, pack, &note, task);
    assert_eq!(json["id"], "retry-policy-v2-plain");
    assert_eq!(json["title"], "Retry policy v2");
}

// ---------------------------------------------------------------------------
// Arm 3 — the malformed override
// ---------------------------------------------------------------------------

/// A malformed `--slug` is rejected **before any bytes move**: the staged doc is
/// byte-identical afterwards, and the value is never silently re-slugified into
/// something the agent did not ask for.
#[test]
fn a_malformed_slug_is_rejected_before_any_bytes_move() {
    let repo = TempDir::new("malformed-repo");
    let home = TempDir::new("malformed-home");
    init_repo(repo.path());
    let (repo, home) = (repo.path(), home.path());
    let task = open_arch_doc(repo, home, None);

    let before = staged_doc(repo, task, "arch-doc:gateway");
    let out = refused(
        jigc(
            repo,
            home,
            None,
            &[
                "doc",
                "add-item",
                "arch-doc:gateway#components",
                "--title",
                "Retry policy v2",
                "--slug",
                "Retry Policy V2",
            ],
        ),
        "add-item --slug `Retry Policy V2`",
    );
    assert!(
        out.contains("--slug"),
        "the reject names the flag it is about; got:\n{out}",
    );
    assert_eq!(
        staged_doc(repo, task, "arch-doc:gateway"),
        before,
        "a malformed override moves no bytes",
    );
    assert!(
        !before.contains("retry-policy-v2"),
        "and mints nothing under a re-slugified spelling either",
    );
}

// ---------------------------------------------------------------------------
// Arm 4 — the enum `id-from`
// ---------------------------------------------------------------------------

/// Where the destination block's `id-from` is an **enum**, the heading IS the member
/// and the anchor equals it — so an override is an identity change, not an id, and is
/// refused with a route that runs. The shipped witness is `changelog`'s
/// `#unreleased-changes` (`id-from: category`).
#[test]
fn an_enum_id_from_refuses_an_override_with_a_route() {
    let repo = TempDir::new("enum-repo");
    let home = TempDir::new("enum-home");
    init_repo(repo.path());
    let (repo, home) = (repo.path(), home.path());

    ok_stdout(jigc(repo, home, None, &["setup"]), "jigc setup");
    ok_stdout(
        jigc(
            repo,
            home,
            None,
            &["start", "--workflow", "record-change", "cut 1.0.0"],
        ),
        "jigc start --workflow record-change",
    );
    let task = "cut-1-0-0";
    assert_eq!(
        ok_stdout(
            jigc(
                repo,
                home,
                None,
                &["doc", "create", "changelog", "--title", "Changelog"],
            ),
            "jigc doc create changelog",
        ),
        "changelog:changelog",
    );

    let before = staged_doc(repo, task, "changelog:changelog");
    let out = refused(
        jigc(
            repo,
            home,
            None,
            &[
                "doc",
                "add-item",
                "changelog:changelog#unreleased-changes",
                "--title",
                "Added",
                "--slug",
                "staged-additions",
            ],
        ),
        "add-item into an enum id-from with --slug",
    );
    assert!(
        out.contains("write.identity-change"),
        "the refusal converges on the shipped identity-change code; got:\n{out}",
    );
    assert!(
        out.contains("route:"),
        "the refusal carries a route (the route floor); got:\n{out}",
    );
    assert!(
        out.contains("jigc doc add-item changelog:changelog#unreleased-changes --title Added"),
        "the route re-runs the same mint without the override; got:\n{out}",
    );
    assert_eq!(
        staged_doc(repo, task, "changelog:changelog"),
        before,
        "the refusal moves no bytes",
    );

    // The omitting context: the very same mint WITHOUT `--slug` is inert — the enum
    // refusal is about the override, never about the door.
    let added = ok_stdout(
        jigc(
            repo,
            home,
            None,
            &[
                "doc",
                "add-item",
                "changelog:changelog#unreleased-changes",
                "--title",
                "Added",
            ],
        ),
        "add-item `Added` (no override)",
    );
    assert_eq!(added, "changelog:changelog#unreleased-changes/added");
}

// ---------------------------------------------------------------------------
// Arm 5 — retitle stays retitle
// ---------------------------------------------------------------------------

/// `doc retitle-item` over an **override-minted** item leaves the anchor frozen at the
/// override: the identity `--slug` chose survives the one verb that moves heading text
/// (the retitle-without-reslug invariant, seen from the override's side).
#[test]
fn retitle_item_over_an_override_minted_item_leaves_the_anchor_frozen() {
    let repo = TempDir::new("retitle-repo");
    let home = TempDir::new("retitle-home");
    init_repo(repo.path());
    let (repo, home) = (repo.path(), home.path());
    let task = open_arch_doc(repo, home, None);

    let item = ok_stdout(
        jigc(
            repo,
            home,
            None,
            &[
                "doc",
                "add-item",
                "arch-doc:gateway#components",
                "--title",
                "Retry policy v2",
                "--slug",
                "retry-policy-v2-plain",
            ],
        ),
        "add-item --slug",
    );

    ok_stdout(
        jigc(
            repo,
            home,
            None,
            &[
                "doc",
                "retitle-item",
                &item,
                "--title",
                "Retry policy, second generation",
            ],
        ),
        "retitle-item over an override-minted item",
    );

    let staged = staged_doc(repo, task, "arch-doc:gateway");
    assert!(
        staged.contains("### Retry policy, second generation  {#retry-policy-v2-plain}"),
        "the heading text moves and the OVERRIDE anchor stays frozen; got:\n{staged}",
    );
    let json = show_json(repo, home, None, &item, task);
    assert_eq!(json["id"], "retry-policy-v2-plain");
    assert_eq!(json["title"], "Retry policy, second generation");
}
