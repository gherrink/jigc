//! The product's **refusal sites**, read from production source — one scan, for the one
//! committed list (`crates/cli/tests/route_catalog/sites.tsv`) that
//! `refusal_site_list.rs` holds equal to it.
//!
//! A finding's code is a free `String`: no enum, no registry, only the constructor calls
//! that mint one, spread over two crates. So *the refusals of a command* cannot be read
//! from any table. What can be read is where a finding is **constructed**, and that is
//! what a site is here:
//!
//!   * a call of `Finding::graded(`, `Finding::block(` or `Finding::blocking(`, and
//!   * a `Finding { … }` literal that sets `severity` — `Self { … }` inside an
//!     `impl Finding` block counts, a pattern (`Finding { severity, .. }`) does not —
//!
//! in production code of `crates/cli/src` and `crates/engine/src`.
//!
//! **Built on [`rust_source`], never beside it.** Comments, strings, `#[cfg(test)]`
//! regions, the enclosing function and every literal's value come from the shared reader;
//! this module adds only what a *site* needs — the call's arguments and the literal's
//! fields. Nothing is done to a file before the reader sees it: a character literal that
//! holds a quote or a bracket — production files hold both — is the reader's to read as a
//! literal, and `a_site_after_a_quote_character_literal_is_still_listed` holds that the
//! scan gets that reading.
//!
//! **A row.**
//!
//! | column | what it holds |
//! |---|---|
//! | `command` | the command the code's prefix is assigned to by [`FAMILIES`]; [`UNASSIGNED`] for a dynamic site |
//! | `file` · `function` · `code` · `ordinal` | **the site's id** ([`Site::id`]) — the function is the reader's `enclosing_fn`, the ordinal counts the function's sites of that code in source order. No line: an edit above a site moves no row |
//! | `form` | `graded` · `block` · `blocking` · `literal` |
//! | `severity` | `blocking` · `warning` · `advisory` as written, [`DYNAMIC`] where it is handed in |
//! | `code-written` | `literal`, or the expression the code was written as — a constant's name beside its resolved value, or what makes the site dynamic |
//! | `route` | the route's **kind as written**: `mechanical` · `human` · `informational` (the three `Route` constructors) · `prose` (a bare string or a `format!`, which `From<String>` turns into a human route) · `variable` (built elsewhere — kind not read) · `none` |
//! | `route-text` | the string literals that stand in the route expression, values joined by a space — a template is not filled, an operand that is not a literal is not there |
//!
//! **A code** is a string literal at the call, a constant resolved to its value (a
//! production `const NAME: &str = "…"`, found in the site's own file first and otherwise
//! only if every definition of that name agrees), or [`DYNAMIC`].
//!
//! **What the scan cannot see — declared, not hidden.**
//!
//!   * **A code that reaches a constructor through a helper, a `code()` method or a door
//!     table.** Its site is listed — dynamic and unassigned, never dropped — but the list
//!     does not say which codes it mints. Whole families are built that way (`rename`,
//!     `relocate`, `repo`, most of `workflow-refs`), so a command may list no site at all.
//!     A fence over a family holds the **code vocabulary** as well as the sites, or it
//!     misses those.
//!   * **Which command prints a code.** [`FAMILIES`] is a table from prefix to the command
//!     a prefix *mostly* belongs to. It is no reachability fact: the gate's families are
//!     printed by three doors, `store.*` by every door that takes an address.
//!   * **Causes.** A site is one row however many distinct states reach it.
//!   * **A route set after construction** (`finding.route = …`), and the kind of a
//!     `variable` route.
//!   * **A finding constructed any other way** — through a macro, an alias of `Finding`, or
//!     a `Deserialize`. None was found on this tree; nothing here would notice one.
//!   * **Nesting.** The reader's `enclosing_fn` is the last `fn` declared before the site,
//!     so a site after a nested `fn` reports the inner name.
//!
//! The three constructors' own bodies in `engine/src/finding.rs` are literals that set
//! `severity`, so they are rows too — dynamic, like every site whose code is a parameter.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::rust_source::{self, StringLiteral};

/// The `code` of a site whose code is not spelled at the call, and the `severity` of one
/// whose severity is handed in.
pub const DYNAMIC: &str = "dynamic";

/// The `command` of a dynamic site: no prefix, so no family.
pub const UNASSIGNED: &str = "unassigned";

/// **Code prefix → command.** The grouping is the one the count of 2026-10-10 made
/// (`completions/artifacts/M55/stabilization-build/reshape/route-registry-facts.md` §7),
/// re-read against this tree by the scan itself: [`scan`] refuses a code whose prefix has
/// no row, so a new family is assigned here, by a person, before its sites can be listed —
/// and `refusal_site_list.rs` refuses a row whose prefix no call spells.
///
/// A command is in the CLI's own words. Where a family is not one verb's, the word stands
/// for a group, and nothing here establishes which door prints which code: `start` is
/// composition, by `start` and `workflow`; `task finalize` is the gate, whose families
/// `task validate`, `task finalize` and `milestone finalize` print alike; `doc write` and
/// `doc read` are the `doc` verbs that write and that read, the second with every other
/// door that takes an address; `config` and `milestone` are those verbs' sub-commands;
/// `validate` is the store-scope sweep.
pub const FAMILIES: &[(&str, &str)] = &[
    ("adapter-guide", "setup"),
    ("amend", "task finalize"),
    ("changelog-recording", "task finalize"),
    ("combine", "milestone"),
    ("commit-recording", "task finalize"),
    ("config", "config"),
    ("conformance", "task finalize"),
    ("create", "doc write"),
    ("doc-code", "task finalize"),
    ("file-state", "task finalize"),
    ("finalize", "task finalize"),
    ("ingest", "ingest"),
    ("join", "milestone"),
    ("migrate", "migrate"),
    ("migrate-corpus", "migrate-corpus"),
    ("milestone", "milestone"),
    ("overrides", "config"),
    ("owner-artifact", "task finalize"),
    ("pack", "start"),
    ("pack-probe-integrity", "validate"),
    ("reconciliation", "task finalize"),
    ("schema-completeness", "validate"),
    ("schema-conformance", "task finalize"),
    ("setup", "setup"),
    ("store", "doc read"),
    ("store-version", "validate"),
    ("task", "task finalize"),
    ("task-bind", "task bind"),
    ("task-discard", "task discard"),
    ("uninstall", "uninstall"),
    ("work-unit", "doc read"),
    ("workflow", "start"),
    ("workflow-refs", "start"),
    ("write", "doc write"),
];

/// The three constructors a call site is found by.
pub const CONSTRUCTORS: [&str; 3] = ["Finding::graded(", "Finding::block(", "Finding::blocking("];

/// The production trees the scan walks, relative to the repository root.
pub const ROOTS: [&str; 2] = ["crates/cli/src", "crates/engine/src"];

/// One source file: its path relative to the repository root, `/`-separated, and its text.
pub struct Source {
    pub path: String,
    pub body: String,
}

impl Source {
    pub fn new(path: &str, body: &str) -> Self {
        Source {
            path: path.to_string(),
            body: body.to_string(),
        }
    }
}

/// One refusal site. The first four fields are its identity and its sort order.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Site {
    pub file: String,
    pub function: String,
    pub code: String,
    pub ordinal: usize,
    pub command: &'static str,
    pub form: &'static str,
    pub severity: &'static str,
    pub written: String,
    pub route: &'static str,
    pub route_text: String,
}

impl Site {
    /// `file::function::code#ordinal` — what a case, a reason or a hunter's count names a
    /// site by.
    pub fn id(&self) -> String {
        format!(
            "{}::{}::{}#{}",
            self.file, self.function, self.code, self.ordinal
        )
    }
}

/// `body` as the scan reads it: comments and literals blanked
/// ([`rust_source::code_only`]), and the `#[cfg(test)]` regions of that view. Offsets are
/// `body`'s own.
pub fn production_code(body: &str) -> (String, Vec<(usize, usize)>) {
    let code = rust_source::code_only(body);
    let regions = rust_source::cfg_test_regions(&code);
    (code, regions)
}

/// A file, read once by every view the scan needs.
struct File<'a> {
    path: &'a str,
    /// Comments and literals blanked.
    code: String,
    /// Comments blanked, literals kept — what an expression was *written* as.
    kept: String,
    regions: Vec<(usize, usize)>,
    literals: Vec<StringLiteral>,
}

impl<'a> File<'a> {
    fn read(source: &'a Source) -> Self {
        let (code, regions) = production_code(&source.body);
        File {
            path: &source.path,
            code,
            kept: rust_source::code_and_strings(&source.body),
            regions,
            literals: rust_source::string_literals(&source.body),
        }
    }

    fn is_test(&self, at: usize) -> bool {
        rust_source::is_test_domain(Path::new(self.path), &self.regions, at)
    }

    /// The span's code with all whitespace removed — strings are blank in it, so what is
    /// left is the expression's shape.
    fn shape(&self, (lo, hi): (usize, usize)) -> String {
        self.code[lo..hi].split_whitespace().collect()
    }

    /// The span as it was written, whitespace collapsed.
    fn written(&self, (lo, hi): (usize, usize)) -> String {
        self.kept[lo..hi]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The offset of the span's first byte that is not whitespace or comment.
    fn lead(&self, (lo, hi): (usize, usize)) -> usize {
        hi - self.kept[lo..hi].trim_start().len()
    }

    /// The values of the string literals that start inside the span, in order.
    fn literals_in(&self, (lo, hi): (usize, usize)) -> Vec<&str> {
        self.literals
            .iter()
            .filter(|l| l.offset >= lo && l.offset < hi)
            .map(|l| l.value.as_str())
            .collect()
    }
}

fn is_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The offset of the bracket that closes the one at `open`.
fn closing(code: &str, open: usize) -> usize {
    let mut depth = 0usize;
    for (i, b) in code.bytes().enumerate().skip(open) {
        match b {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
    }
    panic!("refusal_sites: the bracket at byte {open} never closes — the source view regressed")
}

/// The spans between the bracket at `open` and the one that closes it, split at the commas
/// of that depth; a trailing comma leaves no empty span behind. Emptiness is read from the
/// kept view — a string literal is an argument, and blank in the other.
fn parts(file: &File, open: usize) -> Vec<(usize, usize)> {
    let close = closing(&file.code, open);
    let mut out = Vec::new();
    let (mut depth, mut from) = (0usize, open + 1);
    for (i, b) in file.code.bytes().enumerate().take(close).skip(open + 1) {
        match b {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b',' if depth == 0 => {
                out.push((from, i));
                from = i + 1;
            }
            _ => {}
        }
    }
    if !file.kept[from..close].trim().is_empty() {
        out.push((from, close));
    }
    out
}

/// Every production `const NAME: &str = "…";`, by name: `(file index, value)`.
fn constants(files: &[File]) -> BTreeMap<String, Vec<(usize, String)>> {
    let mut out: BTreeMap<String, Vec<(usize, String)>> = BTreeMap::new();
    for (index, file) in files.iter().enumerate() {
        for (at, keyword) in file.code.match_indices("const ") {
            if file.is_test(at) || at > 0 && is_ident(file.code.as_bytes()[at - 1]) {
                continue;
            }
            let rest = &file.code[at + keyword.len()..];
            let Some((declaration, _)) = rest.split_once(';') else {
                continue;
            };
            let Some((head, value)) = declaration.split_once('=') else {
                continue;
            };
            let Some((name, ty)) = head.split_once(':') else {
                continue;
            };
            let ty: String = ty.split_whitespace().collect();
            if !matches!(ty.as_str(), "&str" | "&'staticstr") || !value.trim().is_empty() {
                continue;
            }
            let lo = at + keyword.len() + head.len() + 1;
            if let [literal] = file.literals_in((lo, lo + value.len()))[..] {
                out.entry(name.trim().to_string())
                    .or_default()
                    .push((index, literal.to_string()));
            }
        }
    }
    out
}

/// The conversions a code or a route may carry without becoming another expression.
const CONVERSIONS: [&str; 3] = [".to_string()", ".into()", ".to_owned()"];

fn without_conversion(shape: &str) -> &str {
    CONVERSIONS
        .iter()
        .find_map(|suffix| shape.strip_suffix(suffix))
        .unwrap_or(shape)
}

/// A code expression read: `(code, how it was written)`.
fn read_code(
    files: &[File],
    index: usize,
    span: (usize, usize),
    constants: &BTreeMap<String, Vec<(usize, String)>>,
) -> (String, String) {
    let file = &files[index];
    let shape = file.shape(span);
    let bare = without_conversion(&shape);
    let literals = file.literals_in(span);

    if let [literal] = literals[..]
        && matches!(bare, "" | "String::from()")
    {
        return (literal.to_string(), "literal".to_string());
    }

    let written = file.written(span);
    let name = bare.strip_prefix('&').unwrap_or(bare);
    let last = name.rsplit("::").next().unwrap_or(name);
    let is_constant = literals.is_empty()
        && name.bytes().all(|b| is_ident(b) || b == b':')
        && last.starts_with(|c: char| c.is_ascii_uppercase())
        && last
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_');
    if is_constant && let Some(defined) = constants.get(last) {
        // Its own file's, when the name is written bare; otherwise only a value every
        // definition agrees on.
        let own: Vec<&String> = defined
            .iter()
            .filter(|(at, _)| *at == index && last == name)
            .map(|(_, value)| value)
            .collect();
        let candidates = if own.is_empty() {
            defined.iter().map(|(_, value)| value).collect()
        } else {
            own
        };
        if candidates.iter().all(|value| *value == candidates[0]) {
            return (candidates[0].clone(), written);
        }
    }
    (DYNAMIC.to_string(), written)
}

/// A severity expression read as written.
fn read_severity(shape: &str) -> &'static str {
    for (name, word) in [
        ("Blocking", "blocking"),
        ("Warning", "warning"),
        ("Advisory", "advisory"),
    ] {
        let path = format!("Severity::{name}");
        if shape == path || shape.ends_with(&format!("::{path}")) {
            return word;
        }
    }
    DYNAMIC
}

/// A route expression read: `(kind as written, the literals that stand in it)`.
/// `optional` is an `Option<Route>` position, where `None` is no route and anything but
/// `Some(…)` is built elsewhere.
fn read_route(file: &File, span: (usize, usize), optional: bool) -> (&'static str, String) {
    let text = file.literals_in(span).join(" ");
    let mut shape = file.shape(span);
    let mut lead = file.lead(span);
    if optional {
        if shape == "None" {
            return ("none", String::new());
        }
        let Some(inner) = shape.strip_prefix("Some(") else {
            return ("variable", text);
        };
        shape = inner.to_string();
        let open = lead + file.code[lead..span.1].find('(').expect("`Some(` opens");
        lead = file.lead((open + 1, span.1));
    }

    if file.literals.iter().any(|l| l.offset == lead) || shape.starts_with("format!(") {
        return ("prose", text);
    }
    // `Route::human(…)`, under any path: `engine::finding::Route::human(…)`.
    if let Some(at) = shape.find("Route::")
        && shape[..at].bytes().all(|b| is_ident(b) || b == b':')
    {
        let constructor = &shape[at + "Route::".len()..];
        for (name, kind) in [
            ("mechanical(", "mechanical"),
            ("human(", "human"),
            ("human_echoing_caller_token(", "human"),
            ("informational(", "informational"),
        ] {
            if constructor.starts_with(name) {
                return (kind, text);
            }
        }
    }
    ("variable", text)
}

/// What was read at one site, before it has an ordinal and a command.
struct Found {
    at: usize,
    form: &'static str,
    severity: &'static str,
    code: (String, String),
    route: (&'static str, String),
}

fn calls(
    files: &[File],
    index: usize,
    constants: &BTreeMap<String, Vec<(usize, String)>>,
) -> Vec<Found> {
    let file = &files[index];
    let mut out = Vec::new();
    for constructor in CONSTRUCTORS {
        for (at, _) in file.code.match_indices(constructor) {
            if file.is_test(at) || at > 0 && is_ident(file.code.as_bytes()[at - 1]) {
                continue;
            }
            let args = parts(file, at + constructor.len() - 1);
            let arg = |n: usize| {
                *args.get(n).unwrap_or_else(|| {
                    panic!(
                        "refusal_sites: `{constructor}…)` in {} (fn `{}`) has no argument {n} \
                         — the constructor's signature moved, or the source view regressed",
                        file.path,
                        rust_source::enclosing_fn(&file.code, at).unwrap_or("?"),
                    )
                })
            };
            // `graded(severity, code, message, location, route)` ·
            // `block(code, message, route)` · `blocking(code, message, location)`.
            let (form, severity, code, route) = match constructor {
                "Finding::graded(" => (
                    "graded",
                    read_severity(&file.shape(arg(0))),
                    arg(1),
                    read_route(file, arg(4), true),
                ),
                "Finding::block(" => ("block", "blocking", arg(0), read_route(file, arg(2), false)),
                _ => ("blocking", "blocking", arg(0), ("none", String::new())),
            };
            out.push(Found {
                at,
                form,
                severity,
                code: read_code(files, index, code, constants),
                route,
            });
        }
    }
    out
}

/// The byte ranges of every `impl Finding { … }` body — where `Self { … }` is a
/// `Finding` literal.
fn impl_finding_bodies(code: &str) -> Vec<(usize, usize)> {
    code.match_indices("impl Finding {")
        .filter(|(at, _)| *at == 0 || !is_ident(code.as_bytes()[*at - 1]))
        .map(|(at, head)| {
            let open = at + head.len() - 1;
            (open, closing(code, open))
        })
        .collect()
}

fn literals(
    files: &[File],
    index: usize,
    constants: &BTreeMap<String, Vec<(usize, String)>>,
) -> Vec<Found> {
    let file = &files[index];
    let code = &file.code;
    let in_impl = impl_finding_bodies(code);
    let mut out = Vec::new();

    for head in ["Finding {", "Self {"] {
        for (at, _) in code.match_indices(head) {
            if file.is_test(at) || at > 0 && is_ident(code.as_bytes()[at - 1]) {
                continue;
            }
            if head == "Self {" && !in_impl.iter().any(|(lo, hi)| at > *lo && at < *hi) {
                continue;
            }
            // The declaration of a struct of that name sets no field.
            if code[..at].trim_end().ends_with("struct") {
                continue;
            }

            let open = at + head.len() - 1;
            let fields = parts(file, open);
            let named = |want: &str| {
                fields.iter().copied().find_map(|(lo, hi)| {
                    let field = code[lo..hi].trim();
                    let lead = hi - code[lo..hi].trim_start().len();
                    match field.split_once(':') {
                        // `name: value` — but not a path that merely starts with the name.
                        Some((name, value)) if name.trim() == want && !value.starts_with(':') => {
                            Some((lead + name.len() + 1, hi))
                        }
                        None if field == want => Some((lead, hi)),
                        _ => None,
                    }
                })
            };
            let Some(severity) = named("severity") else {
                continue;
            };
            // A pattern destructures: its last field is a bare `..`, or what follows the
            // closing brace binds it (`=`, `=>`).
            let rest = fields.last().map(|(lo, hi)| code[*lo..*hi].trim());
            let after = code[closing(code, open) + 1..].trim_start();
            if rest == Some("..") || after.starts_with('=') && !after.starts_with("==") {
                continue;
            }

            let base = fields
                .iter()
                .copied()
                .find(|(lo, hi)| code[*lo..*hi].trim().starts_with(".."));
            out.push(Found {
                at,
                form: "literal",
                severity: read_severity(&file.shape(severity)),
                // A shorthand field (`code`, `route`) reads as the variable it is.
                code: match (named("code"), base) {
                    (Some(span), _) => read_code(files, index, span, constants),
                    (None, Some(base)) => (DYNAMIC.to_string(), file.written(base)),
                    (None, None) => (DYNAMIC.to_string(), String::new()),
                },
                route: match named("route") {
                    Some(span) => read_route(file, span, true),
                    None => ("variable", String::new()),
                },
            });
        }
    }
    out
}

/// The command a code's prefix is assigned to.
fn command_of(code: &str) -> Option<&'static str> {
    if code == DYNAMIC {
        return Some(UNASSIGNED);
    }
    let prefix = code.split_once('.').map_or(code, |(prefix, _)| prefix);
    FAMILIES
        .iter()
        .find(|(family, _)| *family == prefix)
        .map(|(_, command)| *command)
}

/// Every refusal site of `sources`, sorted by id. Constants resolve across the whole set.
///
/// # Panics
///
/// When a code's prefix has no row in [`FAMILIES`] — every such prefix is named, with one
/// site that mints it — and when a site stands in no function.
pub fn scan(sources: &[Source]) -> Vec<Site> {
    let files: Vec<File> = sources.iter().map(File::read).collect();
    let constants = constants(&files);
    let mut sites = Vec::new();
    let mut rowless: BTreeMap<String, String> = BTreeMap::new();

    for (index, file) in files.iter().enumerate() {
        let mut found = calls(&files, index, &constants);
        found.extend(literals(&files, index, &constants));
        found.sort_by_key(|f| f.at);

        let mut ordinals: BTreeMap<(String, String), usize> = BTreeMap::new();
        for found in found {
            let function = rust_source::enclosing_fn(&file.code, found.at)
                .unwrap_or_else(|| {
                    panic!(
                        "refusal_sites: a `{}` site in {} stands in no function the reader can \
                         name, so it has no id",
                        found.form, file.path,
                    )
                })
                .to_string();
            let (code, written) = found.code;
            let ordinal = ordinals
                .entry((function.clone(), code.clone()))
                .or_default();
            *ordinal += 1;
            let mut site = Site {
                file: file.path.to_string(),
                function,
                code,
                ordinal: *ordinal,
                command: UNASSIGNED,
                form: found.form,
                severity: found.severity,
                written,
                route: found.route.0,
                route_text: found.route.1,
            };
            match command_of(&site.code) {
                Some(command) => site.command = command,
                None => {
                    let prefix = site.code.split_once('.').map_or(&*site.code, |(p, _)| p);
                    rowless.entry(prefix.to_string()).or_insert(site.id());
                }
            }
            sites.push(site);
        }
    }

    assert!(
        rowless.is_empty(),
        "refusal_sites: `FAMILIES` holds no row for the code prefix {} — a family is assigned \
         to its command there, by hand, before its sites are listed:\n{}",
        rowless
            .keys()
            .map(|prefix| format!("`{prefix}`"))
            .collect::<Vec<_>>()
            .join(", "),
        rowless
            .iter()
            .map(|(prefix, id)| format!("  {prefix}  (first minted at {id})"))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    sites.sort();
    sites
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("the repository root is two levels above crates/cli")
        .to_path_buf()
}

/// Every `.rs` file of [`ROOTS`], read.
pub fn tree_sources() -> Vec<Source> {
    let root = repo_root();
    ROOTS
        .iter()
        .flat_map(|tree| rust_source::rust_files(&root.join(tree)))
        .map(|path| Source {
            path: path
                .strip_prefix(&root)
                .expect("a walked file is under the repository root")
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/"),
            body: std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {}: {e}", path.display())),
        })
        .collect()
}

/// The scan of this tree's production source.
pub fn scan_tree() -> Vec<Site> {
    scan(&tree_sources())
}

/// Where the committed list lives.
pub fn list_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/route_catalog/sites.tsv")
}

/// A field as a row holds it: one line, no tab, and `-` where there is nothing.
fn field(text: &str) -> String {
    if text.is_empty() {
        return "-".to_string();
    }
    text.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// The committed list's bytes: a header, then one site a row, tab-separated, sorted by id.
pub fn render(sites: &[Site]) -> String {
    let mut out = String::from(
        "# The product's refusal sites: every place production source constructs a finding.\n\
         # Generated from source by `crates/cli/tests/support/refusal_sites.rs`, which says what\n\
         # each column holds and what the scan cannot see. Never edited by hand:\n\
         #   UPDATE_GOLDENS=1 cargo test -p jigc refusal_site_list::\n\
         # A site's id is file::function::code#ordinal. `dynamic`: not spelled at the call.\n\
         # command\tfile\tfunction\tcode\tordinal\tform\tseverity\tcode-written\troute\troute-text\n",
    );
    for site in sites {
        let ordinal = site.ordinal.to_string();
        let row = [
            site.command,
            &site.file,
            &site.function,
            &site.code,
            &ordinal,
            site.form,
            site.severity,
            &site.written,
            site.route,
            &site.route_text,
        ];
        out.push_str(&row.map(field).join("\t"));
        out.push('\n');
    }
    out
}
