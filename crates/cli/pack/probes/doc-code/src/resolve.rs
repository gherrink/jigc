//! Tree-sitter symbol resolution (T2) — resolve a `#symbol` against a file's AST.
//!
//! The grammar is chosen by file extension (`.rs` → the Rust grammar; M10 ships and
//! proves Rust only — [validation.md] → The anchor grammar + resolution). A bare
//! `<path>` (no `#`) is a pure file-existence check and never reaches here. For a
//! `<path>#<symbol>` anchor, the file is parsed statically (no `cargo`, no build, no
//! wall-clock — the determinism contract's rules 1/3/5/6) and `<symbol>` is matched
//! against the named items of the parsed tree **at any nesting** — top level, inside a
//! `mod` body (incl. the dominant `#[cfg(test)] mod tests`), or as an `impl` method.
//!
//! The is-a-test predicate (`#[test]`, T3) adjudicates `criterion-maps-to-test`: a
//! resolved symbol additionally satisfies it when it is a `fn` carrying the canonical
//! Rust `#[test]` attribute at any nesting ([validation.md] → The `doc-code` probe).
//! `symbol-exists` has no test predicate, so it resolves any named item, not only test fns.

use std::path::Path;
use tree_sitter::{Node, Parser};

/// Resolve `symbol` against the AST of the Rust source `src`. Returns `true` when a named
/// item of that name exists **at any nesting** — top level, inside a `mod` body (incl.
/// `#[cfg(test)] mod tests`), or as an `impl` method — the dominant Rust layouts. A parse
/// failure (no language / a tree the parser couldn't build) resolves to `false` — a symbol
/// cannot be proven present. Matching is over real AST named items (the `name` field), so a
/// name appearing only in a string or comment never false-resolves.
pub fn symbol_exists_in_rust(src: &str, symbol: &str) -> bool {
    with_rust_root(src, |root| {
        find_named_item(&root, &|node| {
            item_names(node, src).into_iter().any(|n| n == symbol)
        })
    })
}

/// Resolve `symbol` against the AST of the Rust source `src` and require it to be a
/// **`#[test]`-attributed `fn`** — the is-a-test predicate behind `criterion-maps-to-test`
/// (symbol existence + the test predicate). Returns `true` when a `function_item` named
/// `symbol` carries the canonical Rust `#[test]` attribute **at any nesting** — top level
/// or inside a `mod` body (the dominant `#[cfg(test)] mod tests` layout). The attribute path
/// must be exactly `test` (`#[cfg(test)]` and `#[tokio::test]` are not matched — M10 proves
/// Rust's `#[test]` only). A parse failure resolves to `false`. Static parse only — no
/// `cargo`, build, network, or wall-clock (the determinism contract's rules 1/3/5/6).
pub fn test_fn_exists_in_rust(src: &str, symbol: &str) -> bool {
    with_rust_root(src, |root| {
        find_named_item(&root, &|node| {
            node.kind() == "function_item"
                && node
                    .child_by_field_name("name")
                    .and_then(|n| n.utf8_text(src.as_bytes()).ok())
                    == Some(symbol)
                && has_preceding_test_attribute(node, src)
        })
    })
}

/// Whether any descendant of `node` (or `node` itself) satisfies `pred`. Walks the full AST
/// so a named item is found at any nesting — `mod` bodies and `impl` blocks included.
fn find_named_item(node: &Node, pred: &dyn Fn(&Node) -> bool) -> bool {
    if pred(node) {
        return true;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|child| find_named_item(&child, pred))
}

/// Whether `node`'s immediately-preceding sibling is the canonical `#[test]` attribute. The
/// Rust grammar emits an item's outer attributes as `attribute_item` siblings *before* the
/// item within its container's child list (the `source_file` at top level, the
/// `declaration_list` inside a `mod` body), so the predicate is preceding-sibling at every
/// nesting — not a child of the `function_item`.
fn has_preceding_test_attribute(node: &Node, src: &str) -> bool {
    node.prev_sibling()
        .is_some_and(|prev| is_test_attribute(&prev, src))
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

/// The names an item declares (usually one). Covers the named-item node kinds a
/// `code-anchor` can cite at any nesting; an item with no `name` field contributes
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

#[cfg(test)]
mod tests {
    use super::*;

    // The dominant Rust unit-test layout: `#[test]` fns inside `#[cfg(test)] mod tests`.
    const NESTED: &str = "\
pub struct TokenBucket {
    capacity: u32,
}

impl TokenBucket {
    pub fn refill(&mut self) {
        self.capacity += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burst_rejected() {
        assert!(true);
    }

    fn helper() {}
}
";

    #[test]
    fn symbol_exists_resolves_top_level() {
        assert!(symbol_exists_in_rust(NESTED, "TokenBucket"));
    }

    #[test]
    fn symbol_exists_resolves_impl_method() {
        // An `impl` method is invisible to a top-level-only walk — it must still resolve.
        assert!(symbol_exists_in_rust(NESTED, "refill"));
    }

    #[test]
    fn symbol_exists_resolves_fn_nested_in_mod() {
        // A fn inside `#[cfg(test)] mod tests` — the dominant layout — must resolve.
        assert!(symbol_exists_in_rust(NESTED, "burst_rejected"));
        assert!(symbol_exists_in_rust(NESTED, "helper"));
    }

    #[test]
    fn symbol_absent_still_blocks() {
        // A genuinely-absent symbol must not resolve — descending must not false-resolve.
        assert!(!symbol_exists_in_rust(NESTED, "TokenBucketRenamed"));
        assert!(!symbol_exists_in_rust(NESTED, "burst_rejected_renamed"));
    }

    #[test]
    fn test_fn_resolves_nested_in_mod_tests() {
        // The headline case: a `#[test]` fn nested in `mod tests` must satisfy is-a-test.
        assert!(test_fn_exists_in_rust(NESTED, "burst_rejected"));
    }

    #[test]
    fn test_fn_rejects_non_test_nested_fn() {
        // A nested fn that resolves but carries no `#[test]` is not a test.
        assert!(!test_fn_exists_in_rust(NESTED, "helper"));
        assert!(!test_fn_exists_in_rust(NESTED, "refill"));
    }

    #[test]
    fn test_fn_resolves_top_level() {
        // The integration-test layout (top-level `#[test]` fn) must still resolve.
        let top = "#[test]\nfn burst_rejected() {\n    assert!(true);\n}\n";
        assert!(test_fn_exists_in_rust(top, "burst_rejected"));
    }

    #[test]
    fn test_fn_absent_still_blocks() {
        assert!(!test_fn_exists_in_rust(NESTED, "missing_test"));
    }

    #[test]
    fn non_attribute_text_does_not_false_resolve() {
        // A symbol name appearing only in a string/comment is not an AST named item.
        let src = "// burst_rejected lives in the comment\nfn other() {\n    let _ = \"burst_rejected\";\n}\n";
        assert!(!symbol_exists_in_rust(src, "burst_rejected"));
    }

    #[test]
    fn hostile_input_does_not_panic() {
        // The panic-free property: garbage / truncated source must resolve to false, not crash.
        for src in ["", "fn", "}{)(", "#[test]\nfn", "mod tests { #[test] fn"] {
            let _ = symbol_exists_in_rust(src, "x");
            let _ = test_fn_exists_in_rust(src, "x");
        }
    }
}
