//! Tree-sitter symbol resolution (T2) — resolve a `#symbol` against a file's AST.
//!
//! The grammar is chosen by file extension (`.rs` → the Rust grammar; M10 ships and
//! proves Rust only — [validation.md] → The anchor grammar + resolution). A bare
//! `<path>` (no `#`) is a pure file-existence check and never reaches here. For a
//! `<path>#<symbol>` anchor, the file is parsed statically (no `cargo`, no build, no
//! wall-clock — the determinism contract's rules 1/3/5/6) and `<symbol>` is matched
//! against the **top-level** named items of the parsed tree.
//!
//! The is-a-test predicate (`#[test]`) is **T3** — `symbol-exists` has no test predicate,
//! so this resolves any top-level named item, not only test fns.

use std::path::Path;
use tree_sitter::{Node, Parser};

/// Resolve `symbol` against the AST of the Rust source `src`. Returns `true` when a
/// **top-level** named item of that name exists. A parse failure (no language / a tree
/// the parser couldn't build) resolves to `false` — a symbol cannot be proven present.
pub fn symbol_exists_in_rust(src: &str, symbol: &str) -> bool {
    let mut parser = Parser::new();
    if parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .is_err()
    {
        return false;
    }
    let Some(tree) = parser.parse(src, None) else {
        return false;
    };
    let root = tree.root_node();
    let mut cursor = root.walk();
    root.children(&mut cursor)
        .any(|child| item_names(&child, src).into_iter().any(|n| n == symbol))
}

/// Whether the file at `path` has a `.rs` extension — the only grammar M10 ships.
pub fn is_rust_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some("rs")
}

/// The names a top-level item declares (usually one). Covers the named-item node kinds a
/// `code-anchor` can cite at file top level; an item with no `name` field contributes
/// none. The Rust grammar exposes the declared identifier as the `name` field on each
/// item, so resolution is field-driven (not kind-enumerated) and stays robust across
/// item kinds.
fn item_names(node: &Node, src: &str) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(name) = node
        .child_by_field_name("name")
        .and_then(|n| n.utf8_text(src.as_bytes()).ok())
    {
        names.push(name.to_string());
    }
    names
}
