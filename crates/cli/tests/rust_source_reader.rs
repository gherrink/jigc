//! The shared source reader's own arms: what `support::rust_source` makes of a
//! **character literal**, a lifetime, a raw string and a nested block comment, checked
//! against source it is handed rather than the tree its fences sweep.
//!
//! Every source-level fence reads through that one reader, so a misreading there is a
//! green over code no fence looked at. Until 2026-10-11 its blanking views did not model
//! character literals: a `'"'` opened a string that the next real string closed — the
//! code between read as prose, the prose after it as code — and a `'{'` or a `'('` was
//! counted as a bracket by the module-body and the argument-list matching built on the
//! view. The arms below hold the repair where the reader is, so no fence has to hold it
//! for itself:
//!
//!   * a character literal is a literal — blanked whole in the code view, kept whole
//!     beside the strings in the other, and never the start of a string;
//!   * `'\''` and `'\\'` close at their own last tick, and a `b'…'` is the same literal
//!     behind a `b`;
//!   * a lifetime and a loop label are code: the tick that opens one closes nothing;
//!   * raw strings and nested block comments read as they did, and a tick inside a
//!     string or a comment opens nothing;
//!   * and on this tree, over every file a fence can be handed, the code view balances its
//!     brackets and holds no quote — what a file read inside out cannot do.
//!
//! **The code view is a type** (2026-10-11, `rust_source::CodeOnly`), and the helpers that
//! match brackets take nothing else. Until then three fences matched their test regions over
//! the view that keeps strings, where a brace in a string is a brace: it found no test
//! module at all in four production files and a wrong one in a fifth, and their 195 unit
//! tests were scanned as production code. Handing that view to `cfg_test_regions` no longer
//! compiles, which no arm here can show; what the last arm holds is what the code view's
//! regions are on this tree — every unit test of a production file lies in one. Each of the
//! three fences holds its own reading on a fixture, in its own suite.

use std::path::Path;

use crate::support::rust_source::{
    cfg_test_regions, code_and_strings, code_only, enclosing_callee, rust_files, string_literals,
};
use crate::support::test_homes;

/// `source` with every occurrence of each of `spans` blanked to spaces, byte for byte —
/// what the code view holds of a file whose only comments and literals are those.
fn with_blank(source: &str, spans: &[&str]) -> String {
    let mut out = source.to_string();
    for span in spans {
        assert!(out.contains(span), "the fixture holds `{span}`");
        out = out.replace(span, &" ".repeat(span.len()));
    }
    out
}

/// The character literals the reader is held to: the two delimiters a string has, every
/// bracket, the two escapes that end in the characters a naive reading closes on, the
/// escapes that carry a brace or a quote by number, the separators an argument list and a
/// declaration are split at, a character of more than one byte, and a plain one.
const CHARACTER_LITERALS: [&str; 17] = [
    "'\"'",
    "'\\\"'",
    "'{'",
    "'}'",
    "'('",
    "')'",
    "'['",
    "']'",
    "'\\''",
    "'\\\\'",
    "'\\n'",
    "'\\u{7b}'",
    "'\\x22'",
    "','",
    "';'",
    "'é'",
    "'a'",
];

#[test]
fn a_character_literal_is_blanked_whole_and_what_follows_it_is_still_code() {
    let mut misread = Vec::new();
    for literal in CHARACTER_LITERALS {
        let source = format!(
            "fn f(c: char) -> bool {{ c == {literal} || after({literal}, 1) }}\n\
             fn g() {{ tail(); }}\n"
        );
        let code = code_only(&source);
        if *code != with_blank(&source, &[literal]) {
            misread.push(format!("{literal} -> {:?}", &*code));
        }
        // Kept beside the strings, and nothing else of the file touched.
        if code_and_strings(&source) != source {
            misread.push(format!(
                "{literal}, strings kept -> {:?}",
                code_and_strings(&source)
            ));
        }
    }
    assert!(
        misread.is_empty(),
        "{} of {} character literals are not read as one literal:\n  {}",
        misread.len(),
        CHARACTER_LITERALS.len(),
        misread.join("\n  ")
    );
}

/// The shape three production files hold: a quote character literal, then code, then a
/// real string.
const AFTER_A_QUOTE: &str = r#"
fn unquoted(s: &str) -> &str {
    s.trim_matches('"') // a remark
}

fn between() { seen(); }

fn worded() -> &'static str { "prose" }

fn after() { also_seen(); }
"#;

#[test]
fn a_quote_character_literal_opens_no_string() {
    let code = code_only(AFTER_A_QUOTE);
    assert_eq!(
        *code,
        with_blank(AFTER_A_QUOTE, &["'\"'", "// a remark", "\"prose\""]),
        "the code between the literal and the next string is code, and the string is not"
    );
    assert!(code.contains("seen();") && code.contains("also_seen();"));
    assert!(!code.contains("prose") && !code.contains("remark"));

    // With the strings kept, the comment behind the literal is still a comment.
    assert_eq!(
        code_and_strings(AFTER_A_QUOTE),
        with_blank(AFTER_A_QUOTE, &["// a remark"]),
    );
}

#[test]
fn a_brace_character_literal_does_not_move_a_test_modules_end() {
    for brace in ["'{'", "'}'"] {
        let source = format!(
            "fn production_before() {{}}\n\n\
             #[cfg(test)]\n\
             mod tests {{\n    \
                 fn first_test() -> Option<usize> {{ \"x\".find({brace}) }}\n    \
                 fn second_test() {{}}\n\
             }}\n\n\
             fn production_after() {{}}\n"
        );
        let code = code_only(&source);
        let regions = cfg_test_regions(&code);
        let bodies: Vec<&str> = regions.iter().map(|(lo, hi)| &source[*lo..*hi]).collect();
        assert_eq!(
            bodies.len(),
            1,
            "one test module behind {brace}: {bodies:?}"
        );
        assert!(
            bodies[0].starts_with("#[cfg(test)]")
                && bodies[0].contains("first_test")
                && bodies[0].contains("second_test")
                && bodies[0].ends_with("fn second_test() {}\n}")
                && !bodies[0].contains("production"),
            "the module's body behind {brace} is the module's: {:?}",
            bodies[0]
        );
    }
}

#[test]
fn a_parenthesis_character_literal_does_not_move_a_calls_argument_list() {
    for parenthesis in ["'('", "')'"] {
        let source = format!("fn f() {{ outer(s.find({parenthesis}), here) }}\n");
        let code = code_only(&source);
        let at = source.find("here").expect("the fixture names the position");
        assert_eq!(
            enclosing_callee(&code, at),
            Some("outer"),
            "the call whose argument list holds the position behind {parenthesis}"
        );
    }
}

#[test]
fn a_byte_character_literal_is_the_same_literal_behind_a_b() {
    for literal in ["'\"'", "'{'", "'\\''", "'\\\\'", "'a'"] {
        let source = format!("fn f(c: u8) -> bool {{ c == b{literal} || after() }}\n");
        assert_eq!(
            *code_only(&source),
            with_blank(&source, &[literal]),
            "b{literal}"
        );
        assert_eq!(
            code_and_strings(&source),
            source,
            "b{literal}, strings kept"
        );
    }
}

#[test]
fn adjacent_character_literals_each_close_at_their_own_tick() {
    let source = "fn f() -> [char; 5] { ['\\'','\"','\\\\','{','}'] }\nfn g() { tail(); }\n";
    assert_eq!(
        *code_only(source),
        with_blank(source, &["'\\''", "'\"'", "'\\\\'", "'{'", "'}'"]),
    );
    assert_eq!(code_and_strings(source), source);
}

/// Lifetimes and labels of every shape this workspace writes: declared, bounded, used,
/// `'static`, anonymous, on a struct, and a loop label declared and named.
const LIFETIMES: &str = r#"
fn pick<'a, 'b: 'a>(x: &'a str, _y: &'b str) -> &'a str { x }
static NAME: &'static str = "n";
struct Held<'s>(&'s str);
fn spin() { 'outer: loop { 'i: for _ in 0..1 { continue 'outer; } break 'outer; } }
fn anon(x: &'_ str) -> impl Iterator<Item = char> + '_ { x.chars() }
"#;

#[test]
fn a_lifetime_and_a_loop_label_are_code_and_nothing_of_them_is_eaten() {
    assert_eq!(*code_only(LIFETIMES), with_blank(LIFETIMES, &["\"n\""]));
    assert_eq!(code_and_strings(LIFETIMES), LIFETIMES);
}

#[test]
fn a_lifetime_beside_a_character_literal_of_its_own_name_is_still_a_lifetime() {
    let source = "fn f<'a>(s: &'a str, c: char) -> &'a str {\n    \
                      if c == 'a' { s.trim_matches('\"') } else { 'a: loop { break 'a s; } }\n\
                  }\n\
                  fn g<'b>(x: &'b str) -> &'b str { x }\n";
    assert_eq!(*code_only(source), with_blank(source, &["'a'", "'\"'"]));
    assert_eq!(code_and_strings(source), source);
}

/// Raw strings, a byte raw string, a nested block comment and a line comment, each holding
/// a tick or a quote character literal that is prose there and opens nothing.
const RAW_AND_NESTED: &str = r###"
fn f() {
    let a = r#"a "quoted" { brace 'x"#;
    let b = br"y'";
    /* one /* two '"' */ still { */ kept();
    let c = r##"z"# not yet"##;
    done(); // it's a tail '"'
}
fn g() { let s = "don't { '"; end(); }
"###;

#[test]
fn raw_strings_and_nested_block_comments_read_as_they_did() {
    let comments = ["/* one /* two '\"' */ still { */", "// it's a tail '\"'"];
    let strings = [
        "r#\"a \"quoted\" { brace 'x\"#",
        "r\"y'\"",
        "r##\"z\"# not yet\"##",
        "\"don't { '\"",
    ];
    let code = code_only(RAW_AND_NESTED);
    assert_eq!(
        *code,
        with_blank(RAW_AND_NESTED, &[&comments[..], &strings[..]].concat())
    );
    assert!(code.contains("kept();") && code.contains("done();") && code.contains("end();"));
    assert_eq!(
        code_and_strings(RAW_AND_NESTED),
        with_blank(RAW_AND_NESTED, &comments)
    );
}

#[test]
fn a_tick_character_literal_before_a_quote_one_opens_no_phantom_string() {
    let source = "fn f() -> [char; 2] { ['\\'','\"'] }\nfn g() -> &'static str { \"real\" }\n";
    let values: Vec<String> = string_literals(source)
        .into_iter()
        .map(|literal| literal.value)
        .collect();
    assert_eq!(values, ["real"]);
}

/// Whether every bracket of `code` closes the one it was opened by.
fn balances(code: &str) -> bool {
    let mut open = Vec::new();
    for byte in code.bytes() {
        match byte {
            b'(' | b'[' | b'{' => open.push(byte),
            b')' if open.pop() != Some(b'(') => return false,
            b']' if open.pop() != Some(b'[') => return false,
            b'}' if open.pop() != Some(b'{') => return false,
            _ => {}
        }
    }
    open.is_empty()
}

/// The reader's reading of the tree itself, not of a fixture: Rust that compiles balances
/// its brackets outside its literals and comments, and holds no `"` there. A file whose
/// code view does not is one the reader took a literal's delimiter for code in, or code for
/// a literal — whatever the syntax that did it, known here or not.
#[test]
fn on_this_tree_every_files_code_view_balances_its_brackets_and_holds_no_quote() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("the repository root is two levels above crates/cli");
    let mut trees = vec![root.join("crates/cli/src"), root.join("crates/engine/src")];
    trees.extend(test_homes::suite_homes());

    let mut read = 0usize;
    let mut inside_out = Vec::new();
    for path in trees.iter().flat_map(|tree| rust_files(tree)) {
        let body = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let code = code_only(&body);
        read += 1;
        if !balances(&code) || code.contains('"') {
            inside_out.push(path.display().to_string());
        }
    }
    assert!(read > 0, "the reader was handed no file");
    assert!(
        inside_out.is_empty(),
        "the code view of {} of {read} files does not balance its brackets, or holds a quote — \
         the reader took a literal for code there, or code for a literal:\n  {}",
        inside_out.len(),
        inside_out.join("\n  ")
    );
}

/// The test regions on the tree itself, not on a fixture. [`cfg_test_regions`] takes the
/// code view and nothing else, so these are the regions every fence over production code
/// works with; and a production file's unit tests are what such a fence must not read, so
/// every `#[test]` of one lies in a region — a module left unclosed, or closed early, leaves
/// its tests outside. Matched over the view that keeps strings, 195 of them were.
#[test]
fn on_this_tree_every_unit_test_of_a_production_file_lies_in_a_test_region_of_its_code_view() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("the repository root is two levels above crates/cli");
    let trees = [root.join("crates/cli/src"), root.join("crates/engine/src")];

    let mut tests = 0usize;
    let mut outside = Vec::new();
    for path in trees.iter().flat_map(|tree| rust_files(tree)) {
        let body = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let code = code_only(&body);
        let regions = cfg_test_regions(&code);
        for (at, _) in code.match_indices("#[test]") {
            tests += 1;
            if !regions.iter().any(|(lo, hi)| (*lo..*hi).contains(&at)) {
                outside.push(format!(
                    "{}:{}",
                    path.display(),
                    body[..at].matches('\n').count() + 1
                ));
            }
        }
    }
    assert!(tests > 0, "the reader was handed no unit test");
    assert!(
        outside.is_empty(),
        "{} of {tests} unit tests of production files lie outside every `#[cfg(test)]` region — \
         a fence over production code reads them as production code:\n  {}",
        outside.len(),
        outside.join("\n  ")
    );
}
