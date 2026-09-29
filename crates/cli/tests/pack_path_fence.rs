//! The **production pack-path fence** — no production code outside the embed seam names
//! a pack directory (M54 S2; [module-layout.md](../../../implementation/module-layout.md)
//! → *The seam is prepared now, the crate still is not*).
//!
//! A published crate carries only its own directory, so the packs move into
//! `crates/cli/` and the day a second frontend lands the embed seam becomes a
//! `pack-builtin` crate of its own. Both moves stay mechanical only while exactly one
//! module knows where a pack sits: [`SEAM`], which owns both `include_dir!` roots. A
//! production string anywhere else that names a pack directory — a row that points a
//! reader at `crates/cli/packs/dev/steps/…`, a runtime `join("pack")` — is a second copy of
//! that knowledge, and a move leaves it behind. So the fence reads every string literal
//! the binary could carry and fails on one that names a pack directory.
//!
//! **Scope.** Non-`#[cfg(test)]` code in `crates/*/src`: the `#[cfg(test)]` item bodies
//! [`support::rust_source::cfg_test_regions`](crate::support::rust_source) finds, and
//! every file an out-of-line `#[cfg(test)] mod x;` declares, are test code and out.
//! Comments are out too — they carry no literal — so a doc comment that *describes* the
//! pack home is not an offence; moving those is prose work, not this fence's.
//!
//! **Subject.** Each literal's **decoded value** ([`string_literals`]), matched against
//! [`PACK_DIR_TOKENS`] anywhere in it, and against the crate-relative forms
//! [`names_pack_dir`] reads at its head (`"pack"`, `"/pack/"`, `"../packs/…"` — the
//! pieces a `join` or a `concat!` builds a pack root from). A pack-**relative** path —
//! `steps/amend-message.yaml` — names a file *inside* a pack, not where the pack sits,
//! and is what production code should say.
//!
//! **Exemptions are named, never swept.** [`EXEMPT`] lists each exempt file with its
//! reason, and the fence asserts each one still exists and still carries a literal the
//! predicate matches — an exemption that exempts nothing is a stale one.

use crate::support::rust_source::{
    StringLiteral, cfg_test_regions, code_only, rust_files, string_literals,
};
use std::fs;
use std::path::{Path, PathBuf};

/// The embed seam: the one module that owns both `include_dir!` roots and so names both
/// pack directories.
const SEAM: &str = "crates/cli/src/pack_builtin.rs";

/// The files exempt from the fence, each with its reason. Repo-relative.
const EXEMPT: &[(&str, &str)] = &[(
    SEAM,
    "the embed seam (S2): it owns both `include_dir!` roots, and `include_dir!` takes a \
     literal, so its two pack directories are named here and nowhere else",
)];

/// A literal containing any of these names a pack directory, wherever the token sits in
/// it. `crates/cli/pack` also covers `crates/cli/packs/…`, and `$CARGO_MANIFEST_DIR/pack`
/// covers `…/packs`.
const PACK_DIR_TOKENS: &[&str] = &[
    "crates/cli/pack",
    "packs/dev",
    "packs/methodology",
    "$CARGO_MANIFEST_DIR/pack",
];

/// The workspace root — two levels up from `crates/cli`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize the workspace root")
}

/// The token or form `value` names a pack directory by, or `None` for a literal that
/// names none.
///
/// Beyond [`PACK_DIR_TOKENS`], the crate-relative forms: after any leading `/`, `./` or
/// `../` segments, a value that **is** `pack` or `packs`, or **starts** with `pack/` or
/// `packs/` — the argument of `Path::join("pack")`, or the `"/pack/"` piece of a
/// `concat!(env!("CARGO_MANIFEST_DIR"), …)`.
fn names_pack_dir(value: &str) -> Option<&'static str> {
    if let Some(token) = PACK_DIR_TOKENS.iter().find(|t| value.contains(**t)) {
        return Some(token);
    }
    let mut rest = value;
    loop {
        let before = rest;
        rest = rest.trim_start_matches('/');
        rest = rest.strip_prefix("./").unwrap_or(rest);
        rest = rest.strip_prefix("../").unwrap_or(rest);
        if rest == before {
            break;
        }
    }
    match rest {
        "pack" | "packs" => Some("a crate-relative pack root"),
        _ if rest.starts_with("pack/") || rest.starts_with("packs/") => {
            Some("a crate-relative pack root")
        }
        _ => None,
    }
}

/// The files the out-of-line `#[cfg(test)] mod <name>;` declarations in `path` bring in —
/// `<dir>/<name>.rs` and everything under `<dir>/<name>/`, where `<dir>` is the declaring
/// file's own directory for a `lib.rs`/`main.rs`/`mod.rs` and its stem's directory
/// otherwise. `code` must already be [`code_only`].
fn out_of_line_test_modules(path: &Path, code: &str) -> Vec<PathBuf> {
    let parent = path.parent().expect("a source file has a parent");
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let dir = if matches!(stem, "lib" | "main" | "mod") {
        parent.to_path_buf()
    } else {
        parent.join(stem)
    };
    let mut out = Vec::new();
    for (at, attr) in code.match_indices("#[cfg(test)]") {
        let after = code[at + attr.len()..].trim_start();
        let after = after
            .strip_prefix("pub(crate)")
            .or_else(|| after.strip_prefix("pub"))
            .map_or(after, str::trim_start);
        let Some(after) = after.strip_prefix("mod ") else {
            continue;
        };
        let name: String = after
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let tail = after.trim_start()[name.len()..].trim_start();
        if !name.is_empty() && tail.starts_with(';') {
            out.push(dir.join(format!("{name}.rs")));
            out.push(dir.join(&name));
        }
    }
    out
}

/// One production literal that names a pack directory, rendered for the failure message.
fn offenders_in(rel: &str, body: &str) -> Vec<String> {
    let code = code_only(body);
    let regions = cfg_test_regions(&code);
    string_literals(body)
        .into_iter()
        .filter(|lit| {
            !regions
                .iter()
                .any(|(lo, hi)| lit.offset >= *lo && lit.offset < *hi)
        })
        .filter_map(|StringLiteral { offset, value }| {
            names_pack_dir(&value).map(|token| {
                let line = body[..offset].matches('\n').count() + 1;
                format!("{rel}:{line} — {value:?} names a pack directory ({token})")
            })
        })
        .collect()
}

/// Every `.rs` file under `crates/*/src`, repo-relative, each paired with whether an
/// out-of-line `#[cfg(test)] mod` makes it test code.
fn production_sources(root: &Path) -> Vec<(String, PathBuf)> {
    let crates = root.join("crates");
    let files: Vec<PathBuf> = rust_files(&crates)
        .into_iter()
        .filter(|path| {
            let rel = path.strip_prefix(&crates).expect("under crates/");
            rel.components()
                .nth(1)
                .is_some_and(|c| c.as_os_str() == "src")
        })
        .collect();
    let test_modules: Vec<PathBuf> = files
        .iter()
        .flat_map(|path| {
            let body = fs::read_to_string(path).expect("read a source file");
            out_of_line_test_modules(path, &code_only(&body))
        })
        .collect();
    files
        .into_iter()
        .filter(|path| !test_modules.iter().any(|m| path.starts_with(m)))
        .map(|path| {
            let rel = path
                .strip_prefix(root)
                .expect("a source file sits under the workspace root")
                .to_string_lossy()
                .replace('\\', "/");
            (rel, path)
        })
        .collect()
}

/// **The fence.** No string literal in production code outside [`EXEMPT`] names a pack
/// directory.
///
/// The repair: read the pack through the seam (`EmbeddedPack`), or name the file
/// pack-relative and say which pack by its `pack-id` — never by where it sits.
#[test]
fn no_production_literal_outside_the_seam_names_a_pack_directory() {
    let root = workspace_root();
    let sources = production_sources(&root);

    let mut offenders = Vec::new();
    let mut scanned_literals = 0usize;
    for (rel, path) in &sources {
        if EXEMPT.iter().any(|(exempt, _)| exempt == rel) {
            continue;
        }
        let body = fs::read_to_string(path).expect("read a source file");
        scanned_literals += string_literals(&body).len();
        offenders.extend(offenders_in(rel, &body));
    }

    // Green must mean "nothing named a pack", never "nothing was read"
    // (dev-workflow.md → the vacuous-pass family).
    assert!(
        sources.len() > 50 && scanned_literals > 10_000,
        "the fence measured almost nothing: {} files, {scanned_literals} literals",
        sources.len(),
    );

    assert!(
        offenders.is_empty(),
        "{} production literal(s) outside the embed seam (`{SEAM}`) name a pack directory. \
         Read the pack through `EmbeddedPack`, or name the file pack-relative and the pack \
         by its `pack-id`:\n{}",
        offenders.len(),
        offenders.join("\n"),
    );

    // Each exemption is live: its file exists, is in scope, and names a pack directory in
    // production code — otherwise it exempts nothing and hides the next one to land there.
    for (exempt, reason) in EXEMPT {
        assert!(
            !reason.trim().is_empty(),
            "`{exempt}` is exempt with no reason"
        );
        let (_, path) = sources
            .iter()
            .find(|(rel, _)| rel == exempt)
            .unwrap_or_else(|| panic!("the exempt file `{exempt}` is not a production source"));
        let body = fs::read_to_string(path).expect("read the exempt file");
        assert!(
            !offenders_in(exempt, &body).is_empty(),
            "`{exempt}` is exempt but names no pack directory in production code — a \
             stale exemption; remove it",
        );
    }
}

/// The fence's own reading, checked against source it is handed rather than the tree it
/// sweeps — so a green above is a measurement, not a scanner that stopped seeing.
///
/// Every planted pack path is assembled at run time, so this file carries no literal the
/// fence would read as one (it is a test file, out of scope anyway, but the discipline
/// keeps a copy of these arms honest wherever it lands).
#[test]
fn the_predicate_reads_each_pack_path_form_and_leaves_pack_relative_paths_alone() {
    let cli = ["crates", "cli", "pack"].join("/");
    let named = |value: &str| names_pack_dir(value).is_some();

    for value in [
        format!("{cli}/steps/x.yaml"),
        format!("{cli}s/dev/config/defaults.yaml"),
        format!("../../{}", ["packs", "methodology"].join("/")),
        ["packs", "dev", "schemas"].join("/"),
        format!("${}/pack", "CARGO_MANIFEST_DIR"),
        "pack".to_owned(),
        "packs".to_owned(),
        "/pack/".to_owned(),
        "../packs/methodology/steps".to_owned(),
    ] {
        assert!(named(&value), "`{value}` names a pack directory");
    }
    for value in [
        "steps/amend-message.yaml",
        "config/schema-manifest.yaml",
        "packs.yaml",
        "pack-id",
        "dev",
        "a pack",
    ] {
        assert!(!named(value), "`{value}` names no pack directory");
    }

    let rel = "crates/cli/src/sample.rs";
    let production = format!("fn f() -> &'static str {{ \"{cli}/steps/x.yaml\" }}\n");
    assert_eq!(
        offenders_in(rel, &production).len(),
        1,
        "a production literal"
    );

    let in_test = format!(
        "fn f() {{}}\n#[cfg(test)]\nmod tests {{ fn g() -> &'static str {{ \"{cli}/x\" }} }}\n"
    );
    assert!(
        offenders_in(rel, &in_test).is_empty(),
        "a `#[cfg(test)]` body"
    );

    let in_comment = format!("// see {cli}/steps\n/// and {cli}/config\nfn f() {{}}\n");
    assert!(offenders_in(rel, &in_comment).is_empty(), "a comment");

    // An out-of-line `#[cfg(test)] mod` makes its file test code.
    let lib = Path::new("/w/crates/engine/src/lib.rs");
    let modules = out_of_line_test_modules(lib, "pub mod a;\n#[cfg(test)]\nmod root_walk;\n");
    assert!(
        modules.contains(&PathBuf::from("/w/crates/engine/src/root_walk.rs")),
        "{modules:?}"
    );
    let nested = Path::new("/w/crates/cli/src/pack.rs");
    let modules = out_of_line_test_modules(nested, "#[cfg(test)] pub(crate) mod fixtures;");
    assert!(
        modules.contains(&PathBuf::from("/w/crates/cli/src/pack/fixtures.rs")),
        "{modules:?}"
    );
    // An inline module is not out-of-line.
    assert!(out_of_line_test_modules(lib, "#[cfg(test)]\nmod tests { }").is_empty());
}
