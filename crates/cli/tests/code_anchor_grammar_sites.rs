//! The **`code-anchor` grammar site fence** (M50 Increment 12 / T4 — RC-m50 F-1; joined
//! by the `doc-code` probe at T5, where a value that is **not** an anchor names the grammar
//! it failed to match — RC-m50 F-2).
//!
//! **What F-1 measured.** A blind worker had to reverse-engineer `path#Symbol` from
//! two failed attempts, because every surface that named the type named *only* the
//! type: `jigc doc schema adr` printed `cites-code: code-anchor`, the
//! `{{schema:adr}}` authoring projection printed `code-anchor`, `jigc doc set-field
//! --help` never said the word *anchor* at all, and the shipped guide named the type
//! once in a sentence about probes. The value's shape — the one thing an author must
//! know to write the field — was stated in `design/validation.md` and nowhere a
//! worker looks.
//!
//! **The rule this pins, and why it is a source scan.** The grammar is stated
//! **verbatim, in one spelling, at every home that carries it**, and no home carries
//! it that is not declared here. Both directions are asserted:
//!
//!   - forward — every entry of [`SOURCE_SITES`] contains [`GRAMMAR`] byte-for-byte;
//!   - reverse — walking the shipping roots ([`SCANNED_ROOTS`]), the set of files
//!     containing [`GRAMMAR`] is **exactly** the declared set, so a fifth home cannot
//!     appear un-declared and drift into a second spelling.
//!
//! A shared `const` would be the stronger fence and is **refused here**: the grammar's
//! adjudicator is the bundled `doc-code` probe (`crates/cli/src/doc_code_probe/`, in the
//! `jigc` bin), whose wire types stay independently declared — it imports neither
//! `engine` nor `cli` (M54 S4), so no constant can reach both it and the surfaces below.
//! A source-scanning token table, total in both directions, is what is available; it is
//! asserted rather than assumed.
//!
//! **The rendered arms are separate, and drive the real binary**, because a source
//! site carrying the token proves nothing about whether the surface fed from it still
//! renders it: `doc schema` (over the code-anchor doctypes **derived** from the shipped
//! pack, never hand-listed), the `{{schema:<doctype>}}` compose seam, `doc set-field
//! --help`, and the guide artifact `jigc setup` installs.
//!
//! **The recorded non-addition** (the last arm): the pinned `--format json`
//! projection carries **no grammar key**. The grammar is a
//! property of the declared `type` the envelope already carries, so nothing is
//! withheld from a driver — the same disposition `text_json_parity_axis.rs` records
//! for the `doc schema` row. *(M50 stated this as "`contract-version` stays 6". The
//! version has since moved — M52's `identity`/`home` pair took it to 7 — so the claim
//! is restated over what M50 actually decided: no version moved **for the grammar**.
//! Pinning the numeral pinned the wrong thing; the arm asserts the absent key, and
//! the version it asserts beside it is the current one.)*
//!
//! **Declared bound:** the reverse scan covers the *shipping* surfaces (engine + CLI
//! sources, both packs, both guides). `design/` states the grammar in its own
//! normative spelling (`<repo-relative-path>#<symbol>`, the design home) and is
//! deliberately outside the scan — a design doc is not a surface a worker is routed to.

use crate::support::root_walk;
use cli::pack::FilesystemPack;
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{Field, Leaf, Schema, SectionBody};
use engine::target_surface::is_code_anchor;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The grammar, in the one spelling every site carries. The optional-bracket form is
/// load-bearing: a bare `<repo-relative-path>` is legal and degrades to a
/// file-existence check (`design/validation.md` → The anchor grammar + resolution).
const GRAMMAR: &str = "<repo-relative-path>[#<symbol>]";

/// One declared home of [`GRAMMAR`] — a repo-relative path plus the surface it feeds,
/// so the table reads as the reason each file is here rather than a bare path list.
struct Site {
    path: &'static str,
    feeds: &'static str,
}

/// The declared homes. The pack declaration is the one **authored** home of the surfaces a
/// worker is routed to: it feeds `doc schema`'s listing, the `{{schema:}}` compose seam and
/// `doc set-field --help` alike, so the grammar is authored once and rendered three times
/// rather than typed four times. The guide states it in prose, and the `doc-code` probe
/// re-declares it because its wire types stay independent of `engine` and `cli` (M54 S4),
/// which is exactly why this table is a source scan rather than a shared `const`.
const SOURCE_SITES: &[Site] = &[
    Site {
        path: cli::pack_path!(dev, "config/field-types.yaml"),
        feeds: "the `code-anchor` declaration — read by `doc schema`, the `{{schema:<doctype>}}` \
                compose seam, and `doc set-field --help`",
    },
    Site {
        path: "QUICKSTART.md",
        feeds: "the shipped guide `jigc setup` installs at `.claude/skills/jigc/SKILL.md`",
    },
    Site {
        path: "crates/cli/src/doc_code_probe/mod.rs",
        feeds: "the `doc-code` finding a value that is not an anchor takes (M50 / T5, RC-m50 \
                F-2) — the probe re-declares the token because it imports neither `engine` \
                nor `cli`",
    },
];

/// The shipping roots the reverse scan walks — engine + CLI sources, both packs, both
/// guides. A file under any of these carrying [`GRAMMAR`] must be a declared site.
const SCANNED_ROOTS: &[&str] = &[
    "crates/engine/src",
    "crates/cli/src",
    cli::pack_path!(dev),
    cli::pack_path!(methodology),
    "QUICKSTART.md",
    "MIGRATING.md",
];

/// The workspace root (`crates/cli/../..`).
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves")
}

/// The embedded dev pack tree.
fn dev_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-anchor-grammar-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run jigc")
}

/// The stdout of a successful invocation, with both streams surfaced on failure.
fn ok_stdout(out: &std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
}

/// A minimal jigc-readable repo: a git repo with a committer identity and the project
/// config directory. Enough for the read-only arms (`doc schema`) and for the migrate
/// door, which commits its foreign source and mints into the workbench it creates.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// The composed `{{schema:adr}}` seam is reached through `jigc migrate <path> --as adr`
/// — `migrate-adr` is verb-routed, so `jigc workflow migrate-adr --preview` refuses it
/// (M52 Increment 9 / T2): the verb stages the foreign source the author step rewrites,
/// and the compose-by-name doors stage none. This commits that source and returns the
/// door's argv.
fn migrate_adr_door(repo: &Path) -> Vec<&'static str> {
    fs::write(
        repo.join(FOREIGN_ADR),
        "# Foreign ADR\n\n## Status\n\nAccepted\n\n## Context\n\nWhy.\n",
    )
    .expect("write the foreign adr");
    // Committed, not merely written: `jigc migrate` refuses a source git has never
    // recorded.
    git(repo, &["add", FOREIGN_ADR]);
    git(repo, &["commit", "-q", "-m", "the foreign adr"]);
    vec!["migrate", FOREIGN_ADR, "--as", "adr"]
}

/// The foreign source [`migrate_adr_door`] commits.
const FOREIGN_ADR: &str = "foreign-adr.md";

// ---------------------------------------------------------------------------
// The derivation — the code-anchor field set, read from the shipped pack.
// ---------------------------------------------------------------------------

/// Every field of a schema, descending into repeatable item blocks (and their nested
/// repeatables), so an item-level anchor is reached.
fn schema_fields(schema: &Schema) -> Vec<&Field> {
    fn walk<'a>(leaves: &'a [Leaf], out: &mut Vec<&'a Field>) {
        for leaf in leaves {
            match leaf {
                Leaf::Field(field) => out.push(field),
                Leaf::Repeatable { repeatable, .. } => walk(&repeatable.block, out),
                Leaf::Slot { .. } => {}
            }
        }
    }
    let mut out = Vec::new();
    for section in &schema.sections {
        match &section.body {
            SectionBody::Simple { fields, .. } => out.extend(fields.iter()),
            SectionBody::Repeatable { repeatable } => walk(&repeatable.block, &mut out),
        }
    }
    out
}

/// The shipped pack's `(doctype, field)` pairs typed `code-anchor` — derived from the
/// pack's own bytes through the production predicate, so a doctype that gains an
/// anchor field joins this fence with no edit here.
fn code_anchor_fields() -> BTreeSet<(String, String)> {
    let tree = dev_pack_tree();
    let pack = FilesystemPack::new(tree);
    let mut out = BTreeSet::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .expect("the pack serves the schema it lists");
        let schema = cli::pack::load_pack_schema(&pack, &bytes).expect("the schema parses");
        for field in schema_fields(&schema) {
            if is_code_anchor(&field.ty) {
                out.insert((schema.ty.clone(), field.id.clone()));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// The fence — both directions.
// ---------------------------------------------------------------------------

/// The predicate both the forward arm and its teeth run — one home, so the teeth
/// prove the same check the shipped bytes are held to.
fn states_grammar(text: &str) -> bool {
    text.contains(GRAMMAR)
}

/// Forward: every declared site carries the grammar, byte-for-byte.
#[test]
fn every_declared_site_states_the_grammar_verbatim() {
    let root = repo_root();
    for site in SOURCE_SITES {
        let path = root.join(site.path);
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("declared site `{}` must be readable: {e}", site.path));
        assert!(
            states_grammar(&text),
            "`{}` is a declared home of the `code-anchor` grammar ({}), and must state it \
             verbatim as `{GRAMMAR}`",
            site.path,
            site.feeds,
        );
    }
}

/// **The fence's teeth, per member.** Withdraw the grammar from each declared site's
/// own bytes and the forward predicate must fail for exactly that site — otherwise
/// this suite would be green forever after the shipped text is fixed, with nothing
/// showing it can fail (the `maps_to_test_caveat_fence` mould). The shipped files are
/// never touched: the withdrawal is over a copy of their text.
#[test]
fn withdrawing_the_grammar_from_any_one_site_reddens_the_fence() {
    let root = repo_root();
    for site in SOURCE_SITES {
        let text = fs::read_to_string(root.join(site.path)).expect("a declared site reads");
        let withdrawn = text.replace(GRAMMAR, "");
        assert!(
            !states_grammar(&withdrawn),
            "the fence must go red when `{}` stops stating the grammar",
            site.path,
        );
        assert!(
            states_grammar(&text),
            "…and green while it states it — `{}`",
            site.path,
        );
    }
}

/// Reverse: no un-declared shipping file states the grammar, so the spelling cannot
/// fork into a second home nobody is fencing.
#[test]
fn no_undeclared_shipping_file_states_the_grammar() {
    let root = repo_root();
    let declared: BTreeSet<PathBuf> = SOURCE_SITES
        .iter()
        .map(|s| root.join(s.path))
        .map(|p| p.canonicalize().expect("a declared site exists"))
        .collect();

    let mut found = BTreeSet::new();
    for scanned in SCANNED_ROOTS {
        for file in root_walk::files(&root.join(scanned), |_| true) {
            let Ok(text) = fs::read_to_string(&file) else {
                continue; // a non-UTF-8 shipping asset carries no prose
            };
            if text.contains(GRAMMAR) {
                found.insert(file.canonicalize().expect("a scanned file exists"));
            }
        }
    }

    let undeclared: Vec<_> = found.difference(&declared).collect();
    assert!(
        undeclared.is_empty(),
        "these shipping files state the `code-anchor` grammar without being declared \
         sites — add them to `SOURCE_SITES` (with the surface each feeds) or route them \
         through the pack declaration: {undeclared:?}",
    );
}

// ---------------------------------------------------------------------------
// The rendered arms — the real binary, one per surface a worker reads.
// ---------------------------------------------------------------------------

/// `jigc doc schema <doctype>` names the grammar on the line naming the field, for
/// **every** code-anchor field the shipped pack declares.
#[test]
fn doc_schema_names_the_grammar_beside_every_code_anchor_field() {
    let repo = TempDir::new("schema-repo");
    let home = TempDir::new("schema-home");
    init_repo(repo.path());

    let fields = code_anchor_fields();
    assert!(
        !fields.is_empty(),
        "the shipped pack must declare at least one `code-anchor` field for this fence \
         to mean anything",
    );

    for (doctype, field) in &fields {
        let out = jigc(repo.path(), home.path(), &["doc", "schema", doctype]);
        let text = ok_stdout(&out, &format!("`jigc doc schema {doctype}`"));
        let line = text
            .lines()
            .find(|l| l.trim_start().starts_with(&format!("- {field}:")))
            .unwrap_or_else(|| {
                panic!("`jigc doc schema {doctype}` must list `{field}`; got:\n{text}")
            });
        assert!(
            line.contains(GRAMMAR),
            "`jigc doc schema {doctype}`'s `{field}` line must state the anchor grammar \
             `{GRAMMAR}`; got:\n{line}",
        );
    }
}

/// The `{{schema:<doctype>}}` authoring projection — the seam a migrate author step
/// composes — names the grammar beside the anchor field.
#[test]
fn the_compose_schema_seam_names_the_grammar() {
    let repo = TempDir::new("compose-repo");
    let home = TempDir::new("compose-home");
    init_repo(repo.path());

    let door = migrate_adr_door(repo.path());
    let out = jigc(repo.path(), home.path(), &door);
    let text = ok_stdout(&out, "`jigc migrate foreign-adr.md --as adr`");
    let line = text
        .lines()
        .find(|l| l.contains("`cites-code`"))
        .unwrap_or_else(|| panic!("the composed `{{{{schema:adr}}}}` must render `cites-code`"));
    assert!(
        line.contains(GRAMMAR),
        "the composed `{{{{schema:adr}}}}` projection must state the anchor grammar \
         `{GRAMMAR}` beside `cites-code`; got:\n{line}",
    );
}

/// `jigc doc set-field --help` — the door the value is typed at — names the type and
/// its grammar. F-1's own probe (`| grep -c -i anchor`) returned 0.
#[test]
fn set_field_long_help_names_the_type_and_its_grammar() {
    let repo = TempDir::new("help-repo");
    let home = TempDir::new("help-home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["doc", "set-field", "--help"]);
    let text = ok_stdout(&out, "`jigc doc set-field --help`");
    assert!(
        text.contains("code-anchor"),
        "`doc set-field --help` must name the `code-anchor` type; got:\n{text}",
    );
    assert!(
        text.contains(GRAMMAR),
        "`doc set-field --help` must state the anchor grammar `{GRAMMAR}`; got:\n{text}",
    );
}

/// The guide artifact `jigc setup` installs names the grammar — the fourth surface,
/// and the one an agent meets before it has typed anything.
#[test]
fn the_installed_guide_states_the_grammar() {
    let repo = TempDir::new("guide-repo");
    let home = TempDir::new("guide-home");
    init_repo(repo.path());
    git(repo.path(), &["config", "user.email", "t4@example.com"]);
    git(
        repo.path(),
        &["config", "user.name", "Anchor Grammar Fence"],
    );
    fs::write(repo.path().join("README.md"), "anchor grammar fence\n").expect("write README");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo.path(), home.path(), &["setup"]);
    ok_stdout(&out, "`jigc setup`");

    let guide = repo
        .path()
        .join(".claude")
        .join("skills")
        .join("jigc")
        .join("SKILL.md");
    let text = fs::read_to_string(&guide).expect("the installed guide reads back");
    assert!(
        text.contains(GRAMMAR),
        "the installed guide must state the anchor grammar `{GRAMMAR}`",
    );
}

/// **The generated surfaces really read the declaration.** Withdraw the `hint:` line
/// from a **copy** of the dev pack and point the binary at it (`JIGC_PACK_DIR`): the
/// `doc schema` listing and the `{{schema:<doctype>}}` compose seam both stop naming
/// the grammar. Without this arm the three rendered surfaces could be hardcoding the
/// string and the fence would never know — which is the whole reason the grammar is
/// pack-declared rather than typed four times.
#[test]
fn the_rendered_surfaces_are_fed_by_the_pack_declaration() {
    let pack = TempDir::new("stripped-pack");
    copy_tree(&dev_pack_tree(), pack.path());
    let decl_path = pack.path().join("config").join("field-types.yaml");
    let decl = fs::read_to_string(&decl_path).expect("the copied declaration reads");
    let stripped: String = decl
        .lines()
        .filter(|line| !line.trim_start().starts_with("hint:"))
        .map(|line| format!("{line}\n"))
        .collect();
    assert_ne!(
        decl, stripped,
        "the copy must have had a `hint:` line to remove"
    );
    fs::write(&decl_path, &stripped).expect("write the stripped declaration");

    let repo = TempDir::new("stripped-repo");
    let home = TempDir::new("stripped-home");
    init_repo(repo.path());

    for args in [vec!["doc", "schema", "adr"], migrate_adr_door(repo.path())] {
        let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(&args)
            .current_dir(repo.path())
            .env("HOME", home.path())
            .env("JIGC_PACK_DIR", pack.path())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("run jigc against the stripped pack");
        let text = ok_stdout(
            &out,
            &format!("`jigc {}` over the stripped pack", args.join(" ")),
        );
        assert!(
            text.contains("code-anchor"),
            "the surface must still name the type; got:\n{text}",
        );
        assert!(
            !text.contains(GRAMMAR),
            "`jigc {}` must render the grammar from the pack declaration, not a hardcoded \
             string; got:\n{text}",
            args.join(" "),
        );
    }
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read source dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy file");
        }
    }
}

/// **The recorded non-addition.** The pinned `--format json` projection carries **no
/// grammar key**. The grammar is a
/// property of the declared `type` the envelope already names, so a driver is told
/// nothing less than before — and Increment 8's *one bump, one wave* stands.
///
/// The version assertion beside it is a **currency check, not the claim**: it reads 7
/// since M52's `identity`/`home` pair, and it is asserted here so that a projection
/// change arriving without a bump reddens. What M50 decided — and what this test was
/// named for — is that no version moved **for the grammar**, which a frozen numeral
/// could never express (M52 Increment 6 / T7).
#[test]
fn the_pinned_json_projection_carries_no_grammar_key() {
    let repo = TempDir::new("json-repo");
    let home = TempDir::new("json-home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "json"],
    );
    let text = ok_stdout(&out, "`jigc doc schema adr --format json`");
    assert!(
        text.contains("\"contract-version\": 7"),
        "the schema read contract carries its current version; got:\n{text}",
    );
    assert!(
        !text.contains(GRAMMAR),
        "the grammar is a text-only rendering of the declared `type` the envelope \
         already carries — it must NOT appear as a json key or value; got:\n{text}",
    );
    assert!(
        text.contains("\"type\": \"code-anchor\""),
        "the envelope must still declare the field's type; got:\n{text}",
    );
}
