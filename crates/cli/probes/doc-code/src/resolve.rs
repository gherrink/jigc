//! Tree-sitter symbol resolution — resolve a `#symbol` against a file's AST.
//!
//! The grammar is chosen by file extension ([`grammar_for`]): `.rs`→Rust;
//! `.ts`/`.mts`/`.cts`→TypeScript; `.tsx`→**TSX** (a *distinct* grammar — the plain
//! TypeScript grammar parses JSX with errors); `.js`/`.jsx`/`.mjs`/`.cjs`→JavaScript
//! (one grammar; JSX parses natively). An extension with no shipped grammar
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
    /// JavaScript (incl. JSX). Reuses the TypeScript allowlist verbatim — the citable
    /// node-kinds and value-position semantics are identical, and the TS-only kinds
    /// (`abstract_class_declaration`, `interface_declaration`, …) never appear in a JS AST.
    /// JSX parses natively under this one grammar (no separate TSX-style split).
    JavaScript,
    /// Python. Citable kinds are `function_definition` / `class_definition` at any nesting
    /// (nested defs are idiomatic and citable; methods are defs in a class body); an
    /// `import` carries a `name` field and is excluded (the field-walk over-match).
    Python,
    /// PHP. Citable kinds are `function_definition`, `class_declaration` (incl. an abstract
    /// class — the same kind with an `abstract_modifier` child), `method_declaration`,
    /// `interface_declaration`, `trait_declaration`, `enum_declaration`; a `simple_parameter`
    /// carries a `name` field and is excluded (the field-walk over-match).
    Php,
    /// bash. Citable kind is `function_definition` **only** — both declaration forms
    /// (`function f { }` and `f() { }`) parse to it; a called-but-never-defined function
    /// reaches the AST as a `command` node carrying a `name` and is excluded (functions-only,
    /// the field-walk over-match). Non-function bash symbols (variables, aliases) stay
    /// unverified. bash is the only grammar a POSIX sh-family shebang dispatches to.
    Bash,
}

impl Grammar {
    /// The tree-sitter [`Language`] backing this grammar.
    fn language(self) -> Language {
        match self {
            Grammar::Rust => tree_sitter_rust::LANGUAGE.into(),
            Grammar::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Grammar::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Grammar::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
            Grammar::Python => tree_sitter_python::LANGUAGE.into(),
            Grammar::Php => tree_sitter_php::LANGUAGE_PHP.into(),
            Grammar::Bash => tree_sitter_bash::LANGUAGE.into(),
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
/// TypeScript grammar parses JSX with errors); `.js`/`.jsx`/`.mjs`/`.cjs`→JavaScript (one
/// grammar — JSX parses natively, no TSX-style split). An extension with no shipped grammar yields
/// `None` and the caller keeps the M10 silent-skip (the `unsupported-language` advisory is
/// a later increment).
pub fn grammar_for(path: &Path) -> Option<Grammar> {
    match path.extension().and_then(|e| e.to_str())? {
        "rs" => Some(Grammar::Rust),
        "ts" | "mts" | "cts" => Some(Grammar::TypeScript),
        "tsx" => Some(Grammar::Tsx),
        "js" | "jsx" | "mjs" | "cjs" => Some(Grammar::JavaScript),
        "py" | "pyi" => Some(Grammar::Python),
        "php" | "phtml" => Some(Grammar::Php),
        "sh" | "bash" => Some(Grammar::Bash),
        _ => None,
    }
}

/// The grammar to resolve a file's symbols against when its **extension** maps to no grammar,
/// chosen by **shebang sniff** of the source's first line. A shebang naming an interpreter in
/// the **POSIX sh-compatible shell family** (`sh` / `bash` / `dash` / `zsh` / `ash` / `ksh`)
/// maps to bash — the dominant extensionless-script case; bash is the only grammar a shebang
/// resolves to. Non-POSIX shells (`fish`, `csh`/`tcsh`) are *not* in that family and yield
/// `None` (they fall through to "no grammar", the same as any unsupported file) — they merely
/// happen to end in `sh`, and parsing a fish/csh function under the bash grammar would
/// mis-resolve. Any other (or no) shebang yields `None`. This reads only the file's own bytes
/// (the first line) — no wall-clock, network, or build — so the static-parse determinism
/// contract is intact.
///
/// Precedence is the caller's: [`grammar_for`] (extension) is tried first and **always
/// wins**; this sniff fires only for a file whose extension maps to nothing (a `.py` file
/// carrying a bash shebang stays Python).
pub fn grammar_for_shebang(src: &str) -> Option<Grammar> {
    let first_line = src.lines().next()?;
    let interpreter = first_line.strip_prefix("#!")?;
    // Extract the interpreter basename — the final path component of the last token, so both
    // `#!/bin/bash` and `#!/usr/bin/env bash` reduce to `bash`.
    let token = interpreter.split_whitespace().last()?;
    let command = token.rsplit('/').next()?;
    // Fold only the POSIX sh-compatible family to bash; `fish`/`csh`/`tcsh` (which also end in
    // `sh`) deliberately fall through to `None`.
    matches!(command, "sh" | "bash" | "dash" | "zsh" | "ash" | "ksh").then_some(Grammar::Bash)
}

/// The names a node declares as a citable symbol under `grammar` (usually one; none if the
/// node is not a citable declaration). The policy is **per-language**:
///
/// - **Rust** is field-driven — only declarations carry a `name` field, so any node
///   exposing one is a citable item (robust across Rust item kinds without enumeration).
/// - **TypeScript / TSX / JavaScript** uses a node-kind allowlist ([`is_ts_citable`]) — the
///   field walk over-matches in TS (imports, calls, value-position declarators carry a
///   `name`), so only declaration kinds resolve, with two value-position kinds constrained to
///   a declaration position.
/// - **Python** uses its own node-kind allowlist ([`is_py_citable`]) — `function_definition` /
///   `class_definition` at any nesting; an `import` carries a `name` and is excluded.
/// - **PHP** uses its own node-kind allowlist ([`is_php_citable`]) — the six declaration kinds
///   (function/class/method/interface/trait/enum) at any nesting; a `simple_parameter` carries
///   a `name` and is excluded.
fn item_names(node: &Node, src: &str, grammar: Grammar) -> Vec<String> {
    let citable = match grammar {
        Grammar::Rust => true,
        // JavaScript reuses the TS allowlist verbatim — identical node-kinds and
        // value-position semantics; the TS-only kinds never appear in a JS AST.
        Grammar::TypeScript | Grammar::Tsx | Grammar::JavaScript => is_ts_citable(node),
        Grammar::Python => is_py_citable(node),
        Grammar::Php => is_php_citable(node),
        Grammar::Bash => is_bash_citable(node),
    };
    if !citable {
        return Vec::new();
    }
    node.child_by_field_name("name")
        .and_then(|n| n.utf8_text(src.as_bytes()).ok())
        .map(|name| vec![name.to_string()])
        .unwrap_or_default()
}

/// Whether `node` is a citable TypeScript / TSX / JavaScript declaration (the node-kind
/// allowlist — [validation.md] → Multi-language resolution). JavaScript reuses this verbatim:
/// its citable kinds and value-position semantics are identical, and the TS-only kinds
/// (`abstract_class_declaration`, `interface_declaration`, `type_alias_declaration`,
/// `enum_declaration`) never appear in a JS AST. Declaration kinds resolve at any nesting,
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
        // A module/export/top-level binding only — not a function-local `let`/`const` (inside
        // a `statement_block`) and not a loop-local induction variable (inside a
        // `for_statement` / `for_in_statement`). The declarator's parent is the
        // `lexical_declaration`/`variable_declaration`; positively require that declaration to
        // sit directly at program top-level or under an `export_statement` (the robust reading
        // of "module/export/top-level only" — it closes function bodies *and* loop headers
        // without enumerating every negative scope).
        "variable_declarator" => node
            .parent()
            .and_then(|decl| decl.parent())
            .is_some_and(|scope| matches!(scope.kind(), "program" | "export_statement")),
        _ => false,
    }
}

/// Whether `node` is a citable Python declaration (the node-kind allowlist — [validation.md]
/// → Multi-language resolution). `function_definition` / `class_definition` resolve at any
/// nesting — a top-level def/class, a nested `def` (idiomatic and citable in Python), or a
/// method (a `def` in a class body). An `import` reaches the AST as an
/// `import_statement`/`import_from_statement` carrying a `name` field; it is not a declaration
/// kind and is excluded, closing the field-walk over-match.
fn is_py_citable(node: &Node) -> bool {
    matches!(node.kind(), "function_definition" | "class_definition")
}

/// Whether `node` is a citable PHP declaration (the node-kind allowlist — [validation.md] →
/// Multi-language resolution). The six declaration kinds resolve at any nesting,
/// visibility/abstract-modifier ignored: a `function_definition`, a `class_declaration` (an
/// abstract class is the *same* kind with an `abstract_modifier` child), a
/// `method_declaration`, and — the review's false-block fix — an `interface_declaration`,
/// `trait_declaration`, and `enum_declaration`. A `simple_parameter` reaches the AST with a
/// `name` field but is not a declaration kind and is excluded, closing the field-walk
/// over-match.
fn is_php_citable(node: &Node) -> bool {
    matches!(
        node.kind(),
        "function_definition"
            | "class_declaration"
            | "method_declaration"
            | "interface_declaration"
            | "trait_declaration"
            | "enum_declaration"
    )
}

/// Whether `node` is a citable bash declaration (the node-kind allowlist — [validation.md] →
/// Multi-language resolution). bash is **functions-only**: `function_definition` is the sole
/// citable kind, and both declaration forms (`function f { }` and `f() { }`) parse to it. A
/// called-but-never-defined function reaches the AST as a `command` node carrying a `name`
/// field; it is not a declaration kind and is excluded, closing the field-walk over-match.
/// Non-function bash symbols (variables, aliases) are intentionally unverified.
fn is_bash_citable(node: &Node) -> bool {
    node.kind() == "function_definition"
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
    fn ts_for_loop_induction_variable_does_not_resolve() {
        // `variable_declarator` is module/export/top-level-only: a for-loop induction
        // variable is a loop-local binding (its `lexical_declaration` sits in a
        // `for_statement`, not at program top-level / under an `export_statement`), so it
        // must NOT resolve — while a genuine top-level `const` still does.
        assert!(!symbol_exists(
            "for (let i = 0; i < 10; i++) {}",
            "i",
            Grammar::TypeScript
        ));
        assert!(!symbol_exists(
            "for (const x of xs) {}",
            "x",
            Grammar::TypeScript
        ));
        assert!(symbol_exists(
            "const realExport = 1;",
            "realExport",
            Grammar::TypeScript
        ));
    }

    #[test]
    fn js_for_loop_induction_variable_does_not_resolve() {
        // The same tightening on the JS allowlist (mirrored verbatim): a for-loop / for-of
        // induction variable is loop-local, not a top-level/module declaration.
        assert!(!symbol_exists(
            "for (let i = 0; i < 10; i++) {}",
            "i",
            Grammar::JavaScript
        ));
        assert!(!symbol_exists(
            "for (const x of xs) {}",
            "x",
            Grammar::JavaScript
        ));
        assert!(symbol_exists(
            "const realExport = 1;",
            "realExport",
            Grammar::JavaScript
        ));
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

    // A real JavaScript module exercising the JS allowlist (the TS allowlist minus the
    // TS-only kinds, which never appear in a JS AST) and the value-position constraints:
    // a class with a body method, a free function with a function-local `let`, an
    // object-literal method, a module-level + exported `const`, and an imported-but-never-
    // defined name reaching the AST only as an import + call.
    const JS: &str = "\
export class Shape {
    area() {
        return 0;
    }
}

function freeFn() {
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
";

    #[test]
    fn js_dispatches_by_extension() {
        // The dispatch: every JS extension maps to the JavaScript grammar; a non-JS
        // extension is unaffected.
        for ext in ["js", "jsx", "mjs", "cjs"] {
            let path = std::path::PathBuf::from(format!("src/app.{ext}"));
            assert_eq!(grammar_for(&path), Some(Grammar::JavaScript), "ext .{ext}");
        }
    }

    #[test]
    fn js_resolves_real_symbols_and_blocks_vanished() {
        // A real function/class resolves; a vanished one does not (block-on-rename).
        assert!(symbol_exists(JS, "freeFn", Grammar::JavaScript));
        assert!(symbol_exists(JS, "Shape", Grammar::JavaScript));
        assert!(!symbol_exists(JS, "freeFnRenamed", Grammar::JavaScript));
    }

    #[test]
    fn js_class_method_resolves_but_object_method_does_not() {
        // `method_definition` is class-body-member-only: the class method resolves, the
        // object-literal method (same node kind, parent `object`) does not.
        assert!(symbol_exists(JS, "area", Grammar::JavaScript));
        assert!(!symbol_exists(JS, "objMethod", Grammar::JavaScript));
    }

    #[test]
    fn js_top_level_const_resolves_but_function_local_let_does_not() {
        // `variable_declarator` is module/export/top-level-only: a module-level + exported
        // `const` resolve, a function-local `let` (inside a `statement_block`) does not.
        assert!(symbol_exists(JS, "exported", Grammar::JavaScript));
        assert!(symbol_exists(JS, "topVar", Grammar::JavaScript));
        assert!(!symbol_exists(JS, "localVar", Grammar::JavaScript));
    }

    #[test]
    fn js_imported_but_undefined_does_not_resolve() {
        // A called-but-never-defined name reaches the AST only as an `import_specifier`
        // (and a call `identifier`) — neither is in the allowlist.
        assert!(!symbol_exists(
            JS,
            "calledButUndefined",
            Grammar::JavaScript
        ));
    }

    #[test]
    fn js_jsx_parses_and_resolves_without_error_wipeout() {
        // JSX parses natively under the single JavaScript grammar (no separate TSX-style
        // split): the `.jsx`-shaped component declaration must resolve, not be wiped out by
        // an error tree.
        let src = "\
export function Button() {
    return <div className=\"btn\">click</div>;
}
";
        assert!(symbol_exists(src, "Button", Grammar::JavaScript));
        assert!(!symbol_exists(src, "Missing", Grammar::JavaScript));
    }

    #[test]
    fn js_hostile_input_does_not_panic() {
        // The panic-free property carries to JS: garbage / truncated / BOM / non-ASCII
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
            let _ = symbol_exists(src, "x", Grammar::JavaScript);
        }
    }

    // A real Python module exercising the allowlist (`function_definition`,
    // `class_definition`) and the over-match exclusion: a top-level function and class, a
    // nested `def` inside another def's `block`, a method inside a class body, and an
    // imported name that reaches the AST only as an `import_statement` `name`.
    const PY: &str = "\
import os
from collections import named_import

def free_fn():
    def nested_fn():
        return 1
    return nested_fn()

class Shape:
    def area(self):
        return 0
";

    #[test]
    fn py_dispatches_by_extension() {
        // The dispatch: every Python extension maps to the Python grammar.
        for ext in ["py", "pyi"] {
            let path = std::path::PathBuf::from(format!("src/app.{ext}"));
            assert_eq!(grammar_for(&path), Some(Grammar::Python), "ext .{ext}");
        }
    }

    #[test]
    fn py_resolves_top_level_and_blocks_vanished() {
        // A top-level def/class resolves; a vanished one does not (block-on-rename).
        assert!(symbol_exists(PY, "free_fn", Grammar::Python));
        assert!(symbol_exists(PY, "Shape", Grammar::Python));
        assert!(!symbol_exists(PY, "free_fn_renamed", Grammar::Python));
    }

    #[test]
    fn py_resolves_nested_def() {
        // A nested `def` (def inside a def's `block`) is idiomatic and citable in Python.
        assert!(symbol_exists(PY, "nested_fn", Grammar::Python));
    }

    #[test]
    fn py_resolves_method() {
        // A method (`def` inside a `class_definition` body) resolves at any nesting.
        assert!(symbol_exists(PY, "area", Grammar::Python));
    }

    #[test]
    fn py_imported_name_does_not_resolve() {
        // An imported name reaches the AST only via an `import_statement`/`import_from_statement`
        // carrying a `name` field — not in the allowlist, so the field-walk over-match is closed.
        assert!(!symbol_exists(PY, "os", Grammar::Python));
        assert!(!symbol_exists(PY, "named_import", Grammar::Python));
    }

    #[test]
    fn py_hostile_input_does_not_panic() {
        // The panic-free property carries to Python: garbage / truncated / BOM / non-ASCII /
        // mixed-indent source resolves to false, never crashes.
        let bom = "\u{feff}def f():\n    pass\n";
        for src in [
            "",
            "def",
            "}{)(",
            "class Shape",
            "def f(:\n  return",
            bom,
            "def ☃():\n    π = 1\n",
            "def a():\n\tx = 1\n        y = 2\n",
        ] {
            let _ = symbol_exists(src, "x", Grammar::Python);
        }
    }

    // A real PHP file exercising the allowlist (`function_definition`, `class_declaration`
    // incl. an abstract class, `method_declaration`, `interface_declaration`,
    // `trait_declaration`, `enum_declaration`) and the over-match exclusion: a function with
    // a parameter (a `simple_parameter` carrying a `name`, which must NOT resolve).
    const PHP: &str = "\
<?php

function free_fn($passed_param) {
    return $passed_param;
}

abstract class Shape {
    public function area(): int {
        return 0;
    }
}

interface Drawable {}

trait Loggable {}

enum Suit {
    case Hearts;
}
";

    #[test]
    fn php_dispatches_by_extension() {
        // The dispatch: every PHP extension maps to the PHP grammar.
        for ext in ["php", "phtml"] {
            let path = std::path::PathBuf::from(format!("src/app.{ext}"));
            assert_eq!(grammar_for(&path), Some(Grammar::Php), "ext .{ext}");
        }
    }

    #[test]
    fn php_resolves_function_class_method_and_blocks_vanished() {
        // A function/class/method resolves; a vanished one does not (block-on-rename).
        assert!(symbol_exists(PHP, "free_fn", Grammar::Php));
        assert!(symbol_exists(PHP, "Shape", Grammar::Php));
        assert!(symbol_exists(PHP, "area", Grammar::Php));
        assert!(!symbol_exists(PHP, "free_fn_renamed", Grammar::Php));
    }

    #[test]
    fn php_resolves_interface_trait_enum() {
        // The review's false-block fix: interface/trait/enum declarations ARE citable.
        assert!(symbol_exists(PHP, "Drawable", Grammar::Php));
        assert!(symbol_exists(PHP, "Loggable", Grammar::Php));
        assert!(symbol_exists(PHP, "Suit", Grammar::Php));
    }

    #[test]
    fn php_resolves_abstract_class() {
        // An abstract class is the SAME `class_declaration` kind (an `abstract_modifier`
        // child) — no separate kind needed; it must resolve.
        assert!(symbol_exists(PHP, "Shape", Grammar::Php));
    }

    #[test]
    fn php_parameter_does_not_resolve() {
        // A non-declaration `name`-carrying node (a `simple_parameter`) is not in the
        // allowlist, so the field-walk over-match is closed.
        assert!(!symbol_exists(PHP, "passed_param", Grammar::Php));
    }

    #[test]
    fn php_hostile_input_does_not_panic() {
        // The panic-free property carries to PHP: garbage / truncated / BOM / non-ASCII /
        // missing `<?php` source resolves to false, never crashes.
        let bom = "\u{feff}<?php function f() {}";
        for src in [
            "",
            "<?php function",
            "}{)(",
            "<?php class Shape",
            "function free_fn() {}",
            bom,
            "<?php function ☃() { $π = 1; }",
        ] {
            let _ = symbol_exists(src, "x", Grammar::Php);
        }
    }

    // A real bash script exercising the allowlist (`function_definition` only) in both
    // declaration forms (`function f { }` and `f() { }`) and the over-match exclusion: a
    // called-but-never-defined function reaches the AST only as a `command` node carrying a
    // `name` — it must NOT resolve (functions-only).
    const BASH: &str = "\
#!/usr/bin/env bash

function with_keyword {
    echo keyword
}

posix_form() {
    echo posix
}

called_but_undefined
";

    #[test]
    fn bash_dispatches_by_extension() {
        // The dispatch: every bash extension maps to the bash grammar.
        for ext in ["sh", "bash"] {
            let path = std::path::PathBuf::from(format!("scripts/run.{ext}"));
            assert_eq!(grammar_for(&path), Some(Grammar::Bash), "ext .{ext}");
        }
    }

    #[test]
    fn bash_resolves_both_function_forms_and_blocks_vanished() {
        // Both declaration forms parse to `function_definition` and resolve; a vanished one
        // does not (block-on-rename).
        assert!(symbol_exists(BASH, "with_keyword", Grammar::Bash));
        assert!(symbol_exists(BASH, "posix_form", Grammar::Bash));
        assert!(!symbol_exists(BASH, "posix_form_renamed", Grammar::Bash));
    }

    #[test]
    fn shebang_folds_posix_sh_family_to_bash() {
        // The POSIX sh-compatible shell family (sh / bash / dash / zsh / ash / ksh) folds to
        // the bash grammar via the shebang sniff — both `#!/bin/X` and `#!/usr/bin/env X`.
        for shebang in [
            "#!/bin/bash",
            "#!/usr/bin/env bash",
            "#!/bin/sh",
            "#!/bin/zsh",
            "#!/usr/bin/env dash",
            "#!/bin/ash",
            "#!/bin/ksh",
        ] {
            let src = format!("{shebang}\nf() {{ echo hi; }}\n");
            assert_eq!(
                grammar_for_shebang(&src),
                Some(Grammar::Bash),
                "expected `{shebang}` to fold to bash"
            );
            // And a function in the folded script resolves.
            assert!(
                symbol_exists(&src, "f", grammar_for_shebang(&src).unwrap()),
                "expected `f` to resolve under `{shebang}`"
            );
        }
    }

    #[test]
    fn shebang_does_not_fold_non_posix_shells_to_bash() {
        // fish and csh/tcsh are NOT in the POSIX sh-compatible family — they end with `sh`
        // but must NOT fold to bash; they fall through to "no grammar" (None), the same as
        // any unsupported file. An extensionless fish/csh script then silently skips rather
        // than mis-parsing a fish/csh function under the bash grammar.
        for shebang in [
            "#!/usr/bin/env fish",
            "#!/bin/csh",
            "#!/bin/tcsh",
            "#!/usr/bin/env tcsh",
        ] {
            let src = format!("{shebang}\nfunction f\n    echo hi\nend\n");
            assert_eq!(
                grammar_for_shebang(&src),
                None,
                "expected `{shebang}` to NOT fold to bash"
            );
        }
    }

    #[test]
    fn bash_called_but_undefined_does_not_resolve() {
        // A called-but-never-defined function reaches the AST only as a `command` node
        // carrying a `name` — not in the allowlist (functions-only), so the field-walk
        // over-match is closed.
        assert!(!symbol_exists(BASH, "called_but_undefined", Grammar::Bash));
    }

    #[test]
    fn bash_hostile_input_does_not_panic() {
        // The panic-free property carries to bash: garbage / truncated / BOM / non-ASCII
        // source resolves to false, never crashes.
        let bom = "\u{feff}#!/bin/bash\nfunction f { :; }";
        for src in [
            "",
            "function",
            "}{)(",
            "function f {",
            "f() {",
            bom,
            "function ☃ { local π=1; }",
        ] {
            let _ = symbol_exists(src, "x", Grammar::Bash);
        }
    }
}
