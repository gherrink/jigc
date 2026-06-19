//! Tree-sitter symbol resolution — resolve a `#symbol` against a file's AST.
//!
//! The grammar is chosen by file extension ([`grammar_for`]): `.rs`→Rust;
//! `.ts`/`.mts`/`.cts`→TypeScript; `.tsx`→**TSX** (a *distinct* grammar — the plain
//! TypeScript grammar parses JSX with errors). An extension with no shipped grammar
//! resolves to `None` and the caller keeps the M10 silent-skip (the
//! `unsupported-language` advisory is a later increment). A bare `<path>` (no `#`) is a
//! pure file-existence check and never reaches here. For a `<path>#<symbol>` anchor the
//! file is parsed statically (no `cargo`, no build, no wall-clock — the determinism
//! contract's rules 1/3/5/6) and `<symbol>` is matched against the named items of the
//! parsed tree **at any nesting** — top level, nested bodies, or as a member.
//!
//! What counts as a symbol is **per-language** ([validation.md] → Multi-language
//! resolution). **Rust** stays the field-driven walk (only declarations carry a `name`
//! field, so any named item is citable). **TypeScript / TSX** uses a node-kind
//! **allowlist** — M10's any-`name`-node walk over-matches in TS (`import_specifier`,
//! a function-local `variable_declarator`, an object-literal `method_definition` all
//! carry a `name`), so only declaration kinds resolve, with two value-position kinds
//! constrained to a declaration position (a `variable_declarator` only at
//! module/export/top-level, a `method_definition` only as a class-body member).
//!
//! The is-a-test predicate (`#[test]`) adjudicates `criterion-maps-to-test`: a resolved
//! symbol additionally satisfies it when it is a `fn` carrying the canonical Rust
//! `#[test]` attribute at any nesting ([validation.md] → The `doc-code` probe). It stays
//! **Rust-only** — `#[test]` has no portable cross-language signature.

use std::path::Path;
use tree_sitter::{Language, Node, Parser};

/// A source grammar this probe can resolve symbols against — the result of the
/// extension→grammar dispatch ([`grammar_for`]). The named-symbol policy is keyed on
/// this: Rust is field-driven, TypeScript / TSX use the node-kind allowlist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grammar {
    Rust,
    /// TypeScript and TSX share one symbol policy (the allowlist) and differ only in the
    /// underlying tree-sitter language — TSX is a distinct grammar that parses JSX.
    TypeScript,
    Tsx,
}

impl Grammar {
    /// The tree-sitter [`Language`] backing this grammar.
    fn language(self) -> Language {
        match self {
            Grammar::Rust => tree_sitter_rust::LANGUAGE.into(),
            Grammar::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Grammar::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
        }
    }
}

/// Resolve `symbol` against the AST of `src` parsed under `grammar`. Returns `true` when a
/// citable named item of that name exists **at any nesting** (the per-language symbol
/// policy — Rust field-driven, TS/TSX the node-kind allowlist). A parse failure (no
/// language / a tree the parser couldn't build) resolves to `false` — a symbol cannot be
/// proven present. Matching is over real AST named items, so a name appearing only in a
/// string, comment, call, or import never false-resolves.
pub fn symbol_exists(src: &str, symbol: &str, grammar: Grammar) -> bool {
    with_root(src, grammar, |root| {
        find_named_item(&root, &|node| {
            item_names(node, src, grammar)
                .into_iter()
                .any(|n| n == symbol)
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
    with_root(src, Grammar::Rust, |root| {
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

/// Parse `src` under `grammar` and run `f` over the tree's root node. A missing language or
/// an unbuildable tree resolves to `false` (no AST to adjudicate against).
fn with_root(src: &str, grammar: Grammar, f: impl FnOnce(Node) -> bool) -> bool {
    let mut parser = Parser::new();
    if parser.set_language(&grammar.language()).is_err() {
        return false;
    }
    let Some(tree) = parser.parse(src, None) else {
        return false;
    };
    f(tree.root_node())
}

/// The grammar to resolve a file's symbols against, chosen by extension. `.rs`→Rust;
/// `.ts`/`.mts`/`.cts`→TypeScript; `.tsx`→**TSX** (a distinct grammar — the plain
/// TypeScript grammar parses JSX with errors). An extension with no shipped grammar yields
/// `None` and the caller keeps the M10 silent-skip (the `unsupported-language` advisory is
/// a later increment).
pub fn grammar_for(path: &Path) -> Option<Grammar> {
    match path.extension().and_then(|e| e.to_str())? {
        "rs" => Some(Grammar::Rust),
        "ts" | "mts" | "cts" => Some(Grammar::TypeScript),
        "tsx" => Some(Grammar::Tsx),
        _ => None,
    }
}

/// The names a node declares as a citable symbol under `grammar` (usually one; none if the
/// node is not a citable declaration). The policy is **per-language**:
///
/// - **Rust** is field-driven — only declarations carry a `name` field, so any node
///   exposing one is a citable item (robust across Rust item kinds without enumeration).
/// - **TypeScript / TSX** uses a node-kind allowlist ([`is_ts_citable`]) — the field walk
///   over-matches in TS (imports, calls, value-position declarators carry a `name`), so
///   only declaration kinds resolve, with two value-position kinds constrained to a
///   declaration position.
fn item_names(node: &Node, src: &str, grammar: Grammar) -> Vec<String> {
    let citable = match grammar {
        Grammar::Rust => true,
        Grammar::TypeScript | Grammar::Tsx => is_ts_citable(node),
    };
    if !citable {
        return Vec::new();
    }
    node.child_by_field_name("name")
        .and_then(|n| n.utf8_text(src.as_bytes()).ok())
        .map(|name| vec![name.to_string()])
        .unwrap_or_default()
}

/// Whether `node` is a citable TypeScript / TSX declaration (the node-kind allowlist —
/// [validation.md] → Multi-language resolution). Declaration kinds resolve at any nesting,
/// visibility/export ignored. The two **value-position** kinds are constrained to a
/// declaration position, else the over-match the allowlist exists to close re-opens:
/// `method_definition` only as a class-body member (not an object-literal method);
/// `variable_declarator` only at module/export/top-level (not a function-local `let`/`const`).
fn is_ts_citable(node: &Node) -> bool {
    match node.kind() {
        "function_declaration"
        | "class_declaration"
        | "abstract_class_declaration"
        | "interface_declaration"
        | "type_alias_declaration"
        | "enum_declaration" => true,
        // A class member — not an object-literal method (whose parent is `object`).
        "method_definition" => node.parent().is_some_and(|p| p.kind() == "class_body"),
        // A module/export/top-level binding — not a function-local `let`/`const` (whose
        // declaration sits inside a `statement_block`). The declarator's parent is the
        // `lexical_declaration`/`variable_declaration`; that declaration's parent is the
        // scope — `statement_block` is a function body, anything else (`program`,
        // `export_statement`, …) is module/top-level.
        "variable_declarator" => node
            .parent()
            .and_then(|decl| decl.parent())
            .is_none_or(|scope| scope.kind() != "statement_block"),
        _ => false,
    }
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
        assert!(symbol_exists(NESTED, "TokenBucket", Grammar::Rust));
    }

    #[test]
    fn symbol_exists_resolves_impl_method() {
        // An `impl` method is invisible to a top-level-only walk — it must still resolve.
        assert!(symbol_exists(NESTED, "refill", Grammar::Rust));
    }

    #[test]
    fn symbol_exists_resolves_fn_nested_in_mod() {
        // A fn inside `#[cfg(test)] mod tests` — the dominant layout — must resolve.
        assert!(symbol_exists(NESTED, "burst_rejected", Grammar::Rust));
        assert!(symbol_exists(NESTED, "helper", Grammar::Rust));
    }

    #[test]
    fn symbol_absent_still_blocks() {
        // A genuinely-absent symbol must not resolve — descending must not false-resolve.
        assert!(!symbol_exists(NESTED, "TokenBucketRenamed", Grammar::Rust));
        assert!(!symbol_exists(
            NESTED,
            "burst_rejected_renamed",
            Grammar::Rust
        ));
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
        assert!(!symbol_exists(src, "burst_rejected", Grammar::Rust));
    }

    #[test]
    fn hostile_input_does_not_panic() {
        // The panic-free property: garbage / truncated source must resolve to false, not crash.
        for src in ["", "fn", "}{)(", "#[test]\nfn", "mod tests { #[test] fn"] {
            let _ = symbol_exists(src, "x", Grammar::Rust);
            let _ = test_fn_exists_in_rust(src, "x");
        }
    }

    // A real TypeScript module exercising every allowlist kind and the value-position
    // constraints — an exported abstract class with a method, a free function with a
    // function-local `let`, an object-literal method, a module-level + exported `const`,
    // an imported-but-undefined name, and the interface/type-alias/enum declarations.
    const TS: &str = "\
export abstract class Shape {
    area(): number {
        return 0;
    }
}

function freeFn(): number {
    let localVar = 1;
    return localVar;
}

const obj = {
    objMethod() {
        return 1;
    },
};

export const exported = 2;
const topVar = 3;

import { calledButUndefined } from \"./x\";
calledButUndefined();

interface Iface {}
type Alias = number;
enum Color {
    Red,
}
";

    #[test]
    fn ts_resolves_real_symbols_and_blocks_vanished() {
        // A real symbol resolves; a vanished one does not (the headline block-on-rename).
        assert!(symbol_exists(TS, "freeFn", Grammar::TypeScript));
        assert!(!symbol_exists(TS, "freeFnRenamed", Grammar::TypeScript));
    }

    #[test]
    fn ts_resolves_exported_abstract_class() {
        // The review's false-block: an exported abstract class IS citable.
        assert!(symbol_exists(TS, "Shape", Grammar::TypeScript));
    }

    #[test]
    fn ts_resolves_declaration_kinds() {
        // Every plain declaration kind in the allowlist resolves at any nesting.
        for sym in ["Iface", "Alias", "Color", "exported", "topVar", "obj"] {
            assert!(
                symbol_exists(TS, sym, Grammar::TypeScript),
                "expected `{sym}` to resolve"
            );
        }
    }

    #[test]
    fn ts_class_method_resolves_but_object_method_does_not() {
        // `method_definition` is class-body-member-only: the class method resolves, the
        // object-literal method (same node kind, parent `object`) does not.
        assert!(symbol_exists(TS, "area", Grammar::TypeScript));
        assert!(!symbol_exists(TS, "objMethod", Grammar::TypeScript));
    }

    #[test]
    fn ts_function_local_let_does_not_resolve() {
        // `variable_declarator` is module/export/top-level-only: a function-local `let`
        // (declaration inside a `statement_block`) is not a citable symbol.
        assert!(!symbol_exists(TS, "localVar", Grammar::TypeScript));
    }

    #[test]
    fn ts_imported_but_undefined_does_not_resolve() {
        // A called-but-never-defined name reaches the AST only as an `import_specifier`
        // (and a call `identifier`) — neither is in the allowlist, so the M10 field walk's
        // over-match is closed.
        assert!(!symbol_exists(
            TS,
            "calledButUndefined",
            Grammar::TypeScript
        ));
    }

    #[test]
    fn tsx_jsx_parses_and_resolves_without_error_wipeout() {
        // A `.tsx` JSX component must parse under the TSX grammar (the plain TS grammar
        // errors on JSX) and its declaration must resolve, not be wiped out by an error tree.
        let src = "\
export function Button() {
    return <div className=\"btn\">click</div>;
}
";
        assert!(symbol_exists(src, "Button", Grammar::Tsx));
        assert!(!symbol_exists(src, "Missing", Grammar::Tsx));
    }

    #[test]
    fn ts_hostile_input_does_not_panic() {
        // The panic-free property carries to TS/TSX: garbage / truncated / BOM / non-ASCII
        // source resolves to false, never crashes.
        let bom = "\u{feff}export function f() {}";
        for src in [
            "",
            "function",
            "}{)(",
            "export class",
            "const x = {",
            bom,
            "function ☃() { let π = 1; }",
        ] {
            let _ = symbol_exists(src, "x", Grammar::TypeScript);
            let _ = symbol_exists(src, "x", Grammar::Tsx);
        }
    }
}
