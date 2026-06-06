//! Tree-sitter symbol resolution (T2) — resolve a `#symbol` against a file's AST.
//!
//! The grammar is chosen by file extension (`.rs` → the Rust grammar; M10 ships and
//! proves Rust only — [validation.md] → The anchor grammar + resolution). A bare
//! `<path>` (no `#`) is a pure file-existence check and never reaches here. For a
//! `<path>#<symbol>` anchor, the file is parsed statically (no `cargo`, no build, no
//! wall-clock — the determinism contract's rules 1/3/5/6) and `<symbol>` is matched
//! against the **top-level** named items of the parsed tree.
//!
//! The is-a-test predicate (`#[test]`, T3) adjudicates `criterion-maps-to-test`: a
//! resolved symbol additionally satisfies it when it is a `fn` carrying the canonical
//! Rust `#[test]` attribute ([validation.md] → The `doc-code` probe). `symbol-exists` has
//! no test predicate, so it resolves any top-level named item, not only test fns.

use std::path::Path;
use tree_sitter::{Node, Parser};

/// Resolve `symbol` against the AST of the Rust source `src`. Returns `true` when a
/// **top-level** named item of that name exists. A parse failure (no language / a tree
/// the parser couldn't build) resolves to `false` — a symbol cannot be proven present.
pub fn symbol_exists_in_rust(src: &str, symbol: &str) -> bool {
    with_rust_root(src, |root| {
        let mut cursor = root.walk();
        root.children(&mut cursor)
            .any(|child| item_names(&child, src).into_iter().any(|n| n == symbol))
    })
}

/// Resolve `symbol` against the AST of the Rust source `src` and require it to be a
/// **`#[test]`-attributed top-level `fn`** — the is-a-test predicate behind
/// `criterion-maps-to-test` (symbol existence + the test predicate). Returns `true` only
/// when a top-level `function_item` named `symbol` is immediately preceded by an
/// `attribute_item` whose attribute path is exactly `test` (the canonical Rust `#[test]`;
/// `#[cfg(test)]` and `#[tokio::test]` are not matched — M10 proves Rust's `#[test]` only).
/// A parse failure resolves to `false`. Static parse only — no `cargo`, build, network, or
/// wall-clock (the determinism contract's rules 1/3/5/6).
pub fn test_fn_exists_in_rust(src: &str, symbol: &str) -> bool {
    with_rust_root(src, |root| {
        let mut cursor = root.walk();
        let children: Vec<Node> = root.children(&mut cursor).collect();
        children.iter().enumerate().any(|(i, child)| {
            child.kind() == "function_item"
                && child
                    .child_by_field_name("name")
                    .and_then(|n| n.utf8_text(src.as_bytes()).ok())
                    == Some(symbol)
                && i > 0
                && is_test_attribute(&children[i - 1], src)
        })
    })
}

/// Whether `node` is an `attribute_item` for the canonical Rust `#[test]` — its `attribute`
/// child's text is exactly `test` (so `#[cfg(test)]`, whose attribute path is `cfg`, and
/// `#[tokio::test]`, a `scoped_identifier`, are excluded).
fn is_test_attribute(node: &Node, src: &str) -> bool {
    if node.kind() != "attribute_item" {
        return false;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|child| {
        child.kind() == "attribute" && child.utf8_text(src.as_bytes()).map(str::trim) == Ok("test")
    })
}

/// Parse `src` as Rust and run `f` over the tree's root node. A missing language or an
/// unbuildable tree resolves to `false` (no AST to adjudicate against).
fn with_rust_root(src: &str, f: impl FnOnce(Node) -> bool) -> bool {
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
    f(tree.root_node())
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
