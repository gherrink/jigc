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
                } else if b == b'r' && matches!(src.get(i + 1), Some(b'"') | Some(b'#')) {
                    // `r"…"` or `r#"…"#` — count the hashes so the closer matches.
                    let mut hashes = 0;
                    while src.get(i + 1 + hashes) == Some(&b'#') {
                        hashes += 1;
                    }
                    if src.get(i + 1 + hashes) == Some(&b'"') {
                        mode = Mode::Raw(hashes);
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
                    i += 2;
                    continue;
                }
                if b == b'"' {
                    mode = Mode::Code;
                }
            }
            Mode::Raw(hashes) => {
                if b == b'"' && src[i + 1..].starts_with(&vec![b'#'; hashes][..]) {
                    mode = Mode::Code;
                    i += 1 + hashes;
                    continue;
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

/// Whether the site at `offset` is test code: always, for a file in a `tests/` tree;
/// otherwise only inside a `#[cfg(test)]` module.
pub fn is_test_domain(path: &Path, regions: &[(usize, usize)], offset: usize) -> bool {
    path.components().any(|c| c.as_os_str() == "tests")
        || regions.iter().any(|(lo, hi)| offset >= *lo && offset < *hi)
}

/// Every `.rs` file under `dir`, recursively, skipping `target/` trees. Sorted, so a
/// fence's offender list is deterministic.
pub fn rust_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    collect(dir, &mut out);
    out.sort();
    out
}

fn collect(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            collect(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
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
