//! Reading Rust source **as a fence reads it** — the scanner the source-level fences
//! share.
//!
//! A source-level fence checks a property where membership is decided rather than over
//! a swept list ([dev-workflow.md](../../../../implementation/dev-workflow.md) → *a grep
//! is not a fence*), and every such fence needs the same two things before it can look
//! at anything: source with its **comments and string literals blanked out**, so prose
//! that merely *mentions* the banned idiom is not an offence, and the byte ranges of the
//! **`#[cfg(test)]` module bodies**, so a rule written for production code does not fire
//! on the unit tests interleaved with it.
//!
//! Both are subtle enough that a second copy would be a second set of bugs — this
//! workspace interleaves inline test modules with production code rather than putting
//! them last (`engine/src/validate.rs` opens one at line 969 and resumes production code
//! after it), so the naive "first `#[cfg(test)]` to EOF" rule is wrong in both
//! directions, and a `{` inside a string literal throws brace matching off. The
//! implementation landed with the temp-mint fence (M45) and moved here when the schema
//! resolution fence (M49 Increment 3) needed the same reading.

use std::path::Path;

/// The file with every comment and string literal blanked to spaces, byte offsets and
/// line breaks preserved.
///
/// Everything a fence looks at reads this rather than the raw bytes: prose that
/// *mentions* the idiom is not an instance of it, an assertion message quoting it is not
/// an instance of it, and a brace inside a string literal must not be counted when
/// matching a module body.
pub fn code_only(body: &str) -> String {
    blank(body, false)
}

/// The file with every **comment** blanked and string literals **kept**, byte offsets
/// and line breaks preserved — the reading a fence needs when the thing it looks for
/// *is* a literal (M51 Increment 8: the ambush registry's site leg asks whether a
/// production function really mints a finding **code**, which exists in source only as
/// a `"…"`).
///
/// It delegates to [`code_only`]'s own scanner rather than approximating one, because a
/// second reader would be a second set of bugs on exactly the cases this module's
/// header enumerates: a doc-comment mentioning the code is still not a mint, and a
/// `#[cfg(test)]` region located over [`code_only`] lines up byte-for-byte with this.
pub fn code_and_strings(body: &str) -> String {
    blank(body, true)
}

/// The shared scanner: blank comments always, string literals only when `keep_strings`
/// is false.
fn blank(body: &str, keep_strings: bool) -> String {
    #[derive(Clone, Copy, PartialEq)]
    enum Mode {
        Code,
        Line,
        Block(u32),
        Str,
        Raw(usize),
    }

    let src = body.as_bytes();
    let mut out = vec![b' '; src.len()];
    let mut mode = Mode::Code;
    let mut i = 0;

    while i < src.len() {
        let b = src[i];
        if b == b'\n' {
            out[i] = b'\n';
            if mode == Mode::Line {
                mode = Mode::Code;
            }
            i += 1;
            continue;
        }
        match mode {
            Mode::Code => {
                if b == b'/' && src.get(i + 1) == Some(&b'/') {
                    mode = Mode::Line;
                } else if b == b'/' && src.get(i + 1) == Some(&b'*') {
                    mode = Mode::Block(1);
                    i += 2;
                    continue;
                } else if b == b'"' {
                    mode = Mode::Str;
                    if keep_strings {
                        out[i] = b;
                    }
                } else if b == b'r' && matches!(src.get(i + 1), Some(b'"') | Some(b'#')) {
                    // `r"…"` or `r#"…"#` — count the hashes so the closer matches.
                    let mut hashes = 0;
                    while src.get(i + 1 + hashes) == Some(&b'#') {
                        hashes += 1;
                    }
                    if src.get(i + 1 + hashes) == Some(&b'"') {
                        mode = Mode::Raw(hashes);
                        if keep_strings {
                            out[i..i + 2 + hashes].copy_from_slice(&src[i..i + 2 + hashes]);
                        }
                        i += 2 + hashes;
                        continue;
                    }
                    out[i] = b;
                } else {
                    out[i] = b;
                }
            }
            Mode::Line => {}
            Mode::Block(depth) => {
                if b == b'/' && src.get(i + 1) == Some(&b'*') {
                    mode = Mode::Block(depth + 1);
                    i += 2;
                    continue;
                }
                if b == b'*' && src.get(i + 1) == Some(&b'/') {
                    mode = if depth == 1 {
                        Mode::Code
                    } else {
                        Mode::Block(depth - 1)
                    };
                    i += 2;
                    continue;
                }
            }
            Mode::Str => {
                if b == b'\\' {
                    if keep_strings {
                        out[i..(i + 2).min(src.len())]
                            .copy_from_slice(&src[i..(i + 2).min(src.len())]);
                    }
                    i += 2;
                    continue;
                }
                if keep_strings {
                    out[i] = b;
                }
                if b == b'"' {
                    mode = Mode::Code;
                }
            }
            Mode::Raw(hashes) => {
                if b == b'"' && src[i + 1..].starts_with(&vec![b'#'; hashes][..]) {
                    mode = Mode::Code;
                    if keep_strings {
                        out[i..i + 1 + hashes].copy_from_slice(&src[i..i + 1 + hashes]);
                    }
                    i += 1 + hashes;
                    continue;
                }
                if keep_strings {
                    out[i] = b;
                }
            }
        }
        i += 1;
    }

    String::from_utf8(out).expect("blanking preserves utf-8 boundaries")
}

/// The byte ranges of every `#[cfg(test)]` module body in `code` (which must already be
/// [`code_only`]).
///
/// Found by brace-matching from the module's opening `{`, so an inline test module
/// followed by more production code (the `validate.rs` shape) bounds correctly instead
/// of swallowing the rest of the file.
pub fn cfg_test_regions(code: &str) -> Vec<(usize, usize)> {
    let bytes = code.as_bytes();
    let mut regions = Vec::new();

    for (at, _) in code.match_indices("#[cfg(test)]") {
        let Some(open) = code[at..].find('{').map(|o| at + o) else {
            continue;
        };
        let mut depth = 0usize;
        for (i, b) in bytes.iter().enumerate().skip(open) {
            match b {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        regions.push((at, i + 1));
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    regions
}

/// Whether the site at `offset` is test code: always, for a file in a `tests/` tree or
/// under a suite home outside the crates ([`super::test_homes`] — the repository's
/// tooling suites, whose directory is not named `tests`); otherwise only inside a
/// `#[cfg(test)]` module. A file under such a home is recognised by the path
/// [`super::test_homes::homes_outside_the_crates`] spells it with, which is the root a
/// fence walks it from.
pub fn is_test_domain(path: &Path, regions: &[(usize, usize)], offset: usize) -> bool {
    static OUTSIDE: std::sync::OnceLock<Vec<std::path::PathBuf>> = std::sync::OnceLock::new();
    path.components().any(|c| c.as_os_str() == "tests")
        || OUTSIDE
            .get_or_init(super::test_homes::homes_outside_the_crates)
            .iter()
            .any(|home| path.starts_with(home))
        || regions.iter().any(|(lo, hi)| offset >= *lo && offset < *hi)
}

/// Every `.rs` file under `dir`, recursively, skipping `target/` trees. Sorted, so a
/// fence's offender list is deterministic. Read through [`super::root_walk`], so a
/// missing `dir`, or one holding no `.rs` file, panics rather than scanning nothing.
pub fn rust_files(dir: &Path) -> Vec<std::path::PathBuf> {
    super::root_walk::files(dir, super::root_walk::ext("rs"))
}

/// The name of the function whose body encloses `at` — the last `fn <name>` declared
/// before it, which is what a fence names when it reports an offending site and what an
/// allow-list keys on.
///
/// Nesting is not modelled: a site inside a closure or an inner `fn` reports that inner
/// name, which is the more specific answer and the one a reader can find.
pub fn enclosing_fn(code: &str, at: usize) -> Option<&str> {
    let head = &code[..at];
    let start = head.rfind("fn ")? + 3;
    let name: &str = code[start..]
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()?;
    (!name.is_empty()).then_some(name)
}

/// The identifier of the call whose argument list encloses `at` — `pack.read(…)` answers
/// `read`, `read_pack(pack, …)` answers `read_pack`, and a position inside a tuple or an
/// array literal answers `None`.
///
/// Walks back from `at` to the nearest **unmatched** `(`, then takes the identifier
/// immediately before it, so a fence can ask *what is being done with this value* rather
/// than pattern-matching one spelling of the call.
pub fn enclosing_callee(code: &str, at: usize) -> Option<&str> {
    let bytes = code.as_bytes();
    let mut depth = 0i32;
    let mut i = at;
    let open = loop {
        if i == 0 {
            return None;
        }
        i -= 1;
        match bytes[i] {
            b')' => depth += 1,
            b'(' => {
                if depth == 0 {
                    break i;
                }
                depth -= 1;
            }
            _ => {}
        }
    };
    let head = &code[..open];
    let name_start = head
        .rfind(|c: char| !(c.is_alphanumeric() || c == '_'))
        .map_or(0, |i| i + 1);
    let name = &code[name_start..open];
    (!name.is_empty()).then_some(name)
}

/// One decoded string literal: where it starts in the file, and the **value** the
/// compiler will bake into the binary — escapes resolved, `\`-newline continuations
/// already collapsed.
pub struct StringLiteral {
    pub offset: usize,
    pub value: String,
}

/// Every string literal in `body`, decoded.
///
/// A fence that checks *what a message says* needs the exact opposite of [`code_only`]:
/// the literal values rather than the code around them. Reading the raw source lines
/// instead is not equivalent — a `\n` written in source is one newline in the value, a
/// `\`-newline has already eaten the indentation that follows it, and a run of spaces
/// inside a comment or an aligned doc-table is not part of any message at all.
///
/// Unlike [`code_only`] this also models **char literals**: `'"'` occurs ten times in
/// this workspace, and treating its quote as a string opener would report a stretch of
/// real code as some literal's value.
pub fn string_literals(body: &str) -> Vec<StringLiteral> {
    let src = body.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < src.len() {
        let b = src[i];

        // Comments carry no literals.
        if b == b'/' && src.get(i + 1) == Some(&b'/') {
            i = body[i..].find('\n').map_or(src.len(), |o| i + o);
            continue;
        }
        if b == b'/' && src.get(i + 1) == Some(&b'*') {
            let mut depth = 1u32;
            i += 2;
            while i < src.len() && depth > 0 {
                if src[i] == b'/' && src.get(i + 1) == Some(&b'*') {
                    depth += 1;
                    i += 2;
                } else if src[i] == b'*' && src.get(i + 1) == Some(&b'/') {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }

        // A `'` opens a char literal or a lifetime; only the former can hide a quote.
        if b == b'\'' {
            i = skip_char_or_lifetime(body, i);
            continue;
        }

        // `r"…"`, `r#"…"#`, `br"…"`, `br#"…"#` — raw, so the value is verbatim.
        let raw_at = if b == b'b' { i + 1 } else { i };
        if src.get(raw_at) == Some(&b'r') {
            let mut hashes = 0usize;
            while src.get(raw_at + 1 + hashes) == Some(&b'#') {
                hashes += 1;
            }
            if src.get(raw_at + 1 + hashes) == Some(&b'"') {
                let start = raw_at + 2 + hashes;
                let closer = format!("\"{}", "#".repeat(hashes));
                let end = body[start..].find(&closer).map_or(src.len(), |o| start + o);
                out.push(StringLiteral {
                    offset: i,
                    value: body[start..end.min(body.len())].to_string(),
                });
                i = (end + closer.len()).min(src.len());
                continue;
            }
        }

        // `"…"` / `b"…"` — escaped.
        let str_at = if b == b'b' && src.get(i + 1) == Some(&b'"') {
            i + 1
        } else {
            i
        };
        if src[str_at] == b'"' {
            let (value, end) = decode_escaped(body, str_at + 1);
            out.push(StringLiteral { offset: i, value });
            i = end;
            continue;
        }

        i += 1;
    }

    out
}

/// Past the `'` at `at`: a char literal (`'x'`, `'\n'`, `'\u{1f}'`) is skipped whole, a
/// lifetime is skipped by one byte so the tick itself cannot open anything.
fn skip_char_or_lifetime(body: &str, at: usize) -> usize {
    let src = body.as_bytes();
    if src.get(at + 1) == Some(&b'\\') {
        return body[at + 2..]
            .find('\'')
            .map_or(src.len(), |o| at + 2 + o + 1);
    }
    let Some(c) = body[at + 1..].chars().next() else {
        return at + 1;
    };
    let after = at + 1 + c.len_utf8();
    if src.get(after) == Some(&b'\'') {
        after + 1
    } else {
        at + 1
    }
}

/// The value of a normal string literal whose body starts at `from`, and the offset just
/// past its closing quote.
fn decode_escaped(body: &str, from: usize) -> (String, usize) {
    let src = body.as_bytes();
    let mut value = String::new();
    let mut i = from;
    while i < src.len() {
        match src[i] {
            b'"' => return (value, i + 1),
            b'\\' => {
                let Some(next) = src.get(i + 1).copied() else {
                    return (value, src.len());
                };
                match next {
                    b'n' => value.push('\n'),
                    b't' => value.push('\t'),
                    b'r' => value.push('\r'),
                    b'0' => value.push('\0'),
                    b'\\' | b'"' | b'\'' => value.push(next as char),
                    b'\n' => {
                        // A line continuation eats the newline and the indentation after
                        // it — the whole point of the escape, and the shape the fence
                        // below asks authors to use.
                        i += 2;
                        while matches!(src.get(i), Some(b' ') | Some(b'\t')) {
                            i += 1;
                        }
                        continue;
                    }
                    b'x' => {
                        let hex = body.get(i + 2..i + 4).unwrap_or("20");
                        value.push(u8::from_str_radix(hex, 16).unwrap_or(b' ') as char);
                        i += 4;
                        continue;
                    }
                    b'u' => {
                        let close = body[i..].find('}').map_or(src.len(), |o| i + o);
                        let hex = body.get(i + 3..close).unwrap_or("20");
                        if let Some(c) = u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
                        {
                            value.push(c);
                        }
                        i = close + 1;
                        continue;
                    }
                    other => value.push(other as char),
                }
                i += 2;
            }
            _ => {
                let c = body[i..].chars().next().unwrap_or(' ');
                value.push(c);
                i += c.len_utf8();
            }
        }
    }
    (value, src.len())
}
