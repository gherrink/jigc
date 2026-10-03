//! **The seed ledger is the count, and it is held to its sources** (M55 Increment 9, T1;
//! `DECISIONS.md` → *2026-10-03 — M55 Increment 9 planning*, P1 and P2).
//!
//! [`completions/artifacts/M55/seed-ledger.md`](../../../completions/artifacts/M55/seed-ledger.md)
//! carries one row per distinct finding the seed files, and each row names every source row
//! it was recorded at. The scope's *first, the count* is only a count if nothing is lost and
//! nothing is counted twice, so this suite reads **the sources themselves**, never a list of
//! them written here:
//!
//! - the planning register's `F*`, `T*` and `D*` ids (`D*` → `inconsistency`);
//! - the tier-2/3 row heads of the M52 per-axis review and of M53's four re-reviews, from
//!   `### Tier 2` to the findings' `## B` section;
//! - the `(D) (a)`–`(f)` rows under *Deferred at the M53 Settle*;
//! - the owed rows under *Owed after M53's post-review arcs* and the CI rows under the
//!   `**CI — …**` paragraph, each by its opening bold label;
//! - the two declared bounds of `design/findings-channel.md` → 6.
//!
//! Every source row sits in exactly one ledger row's sources, or in the ledger's
//! exclusions with a reason; no ledger source names a row its source does not carry; the
//! keys are unique, filesystem-safe and prefixed by their first source's tag, which the
//! re-drive batches glob on. A bold label is matched verbatim, which is why the pointer
//! rewrite of the `decisions-pending.md` rows (T8) keeps each row's opening label.
//!
//! **The verdicts are held to the filing inputs** (T2, P3 and P4): a row with a verdict has
//! an input directory under `seed-filing/input/<doctype>/<key>/` whose `fields` `status` is
//! that verdict, a row without one has none, and no input names a row the ledger lacks.
//!
//! **The filed seed is the ledger, row for row** (T7): every row carries a verdict; the
//! rows' seed docs and the files under `seed/jigc-feedback/` and `seed/inconsistencies/`
//! (the only two entries of `seed/`) are a bijection, each doc titled as its row's input;
//! and each seed doc's `status` is its row's verdict.
//!
//! **The seeded sources point at the seed** (T8): each register row's disposition links its
//! seed doc, and each seeded `decisions-pending.md` row is one pointer line — its label,
//! the link, and its trigger.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

/// The ledger, repo-relative.
const LEDGER: &str = "completions/artifacts/M55/seed-ledger.md";

/// The heading the ledger's table sits under.
const LEDGER_HEADING: &str = "## The ledger";

const REGISTER: &str = "completions/artifacts/M55/planning-findings.md";
const PENDING: &str = "implementation/decisions-pending.md";
const CHANNEL: &str = "design/findings-channel.md";

/// Each source tag: the file its link must name, and the key prefix a row whose **first**
/// source carries the tag takes. The register's prefix is completed by the row's own id
/// (`F6` → `m55-f6`).
const TAGS: &[(&str, &str, &str)] = &[
    ("register", REGISTER, "m55-"),
    (
        "M52",
        "completions/artifacts/M52/per-axis-review/README.md",
        "rc16-",
    ),
    (
        "rc.17",
        "completions/artifacts/M53/per-axis-review/README.md",
        "m53-rc17-",
    ),
    (
        "rc.18",
        "completions/artifacts/M53/per-axis-review-rc18/README.md",
        "m53-rc18-",
    ),
    (
        "rc.19",
        "completions/artifacts/M53/per-axis-review-rc19/README.md",
        "m53-rc19-",
    ),
    (
        "rc.20",
        "completions/artifacts/M53/per-axis-review-rc20/README.md",
        "m53-rc20-",
    ),
    ("M53 Settle", PENDING, "m53-settle-"),
    ("owed", PENDING, "owed-"),
    ("CI", PENDING, "ci-"),
    ("bound", CHANNEL, "m55-bound-"),
    // Not a seed source (S12): read only to prove the ledger's exclusion of its row (a)
    // names a real row. Its rows (b) and (c) stay milestone-keyed deferrals there.
    ("M52 Settle", PENDING, ""),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

fn read(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel))
        .unwrap_or_else(|e| panic!("`{rel}` must be readable: {e}"))
}

/// The lines of `text` after the first line starting with `start`, up to the first later
/// line `stop` accepts.
fn section<'a>(text: &'a str, start: &str, stop: impl Fn(&str) -> bool) -> Vec<&'a str> {
    let mut lines = text.lines().skip_while(|l| !l.starts_with(start));
    assert!(lines.next().is_some(), "no line starting `{start}`");
    lines.take_while(|l| !stop(l)).collect()
}

/// The text between `open` at the start of `s` and the next `close`, and what follows it.
fn delimited<'a>(s: &'a str, open: &str, close: &str) -> Option<(&'a str, &'a str)> {
    let body = s.strip_prefix(open)?;
    let end = body.find(close)?;
    Some((&body[..end], &body[end + close.len()..]))
}

/// A source row's own id as its source spells it: a bold label's text, a backticked span's
/// text, or a bare word. Returns the id and what follows it.
fn source_id(s: &str) -> (&str, &str) {
    delimited(s, "**", "**")
        .or_else(|| delimited(s, "`", "`"))
        .unwrap_or_else(|| s.split_at(s.find(char::is_whitespace).unwrap_or(s.len())))
}

/// The bold label opening a `- **…**` list row.
fn row_label(line: &str) -> Option<&str> {
    delimited(line.strip_prefix("- ")?, "**", "**").map(|(label, _)| label)
}

/// A review's tier-2/3 row heads: from `### Tier 2` to the findings' `## B` section, each a
/// line opening with `marker` and a backticked `(<axis>, <id>)`, read as that first id.
fn review_heads(rel: &str, marker: &str) -> BTreeSet<String> {
    let text = read(rel);
    section(&text, "### Tier 2", |l| l.starts_with("## B "))
        .into_iter()
        .filter_map(|l| l.strip_prefix(marker))
        .filter(|l| l.starts_with("`("))
        .map(|l| source_id(l).0.to_owned())
        .collect()
}

/// The `- **(D) (x) …**` rows under a decisions-pending heading, as `(D) (x)`.
fn settle_rows(pending: &str, heading: &str) -> BTreeSet<String> {
    section(pending, heading, |l| l.starts_with("### "))
        .into_iter()
        .filter_map(row_label)
        .filter_map(|label| label.get(..7).filter(|id| id.starts_with("(D) (")))
        .map(str::to_owned)
        .collect()
}

/// Every source row each tag's source carries, read from the sources themselves.
fn source_rows() -> BTreeMap<&'static str, BTreeSet<String>> {
    let mut rows = BTreeMap::new();

    let register = read(REGISTER);
    rows.insert(
        "register",
        register
            .lines()
            .filter_map(|l| l.strip_prefix("| ")?.split(" |").next())
            .filter(|id| {
                id.len() > 1
                    && id.starts_with(['F', 'T', 'D'])
                    && id[1..].bytes().all(|b| b.is_ascii_digit())
            })
            .map(str::to_owned)
            .collect(),
    );

    // M52 sets its row heads in bold, M53's re-reviews as `####` headings.
    for (tag, rel, _) in TAGS {
        let marker = match *tag {
            "M52" => "**",
            t if t.starts_with("rc.") => "#### ",
            _ => continue,
        };
        rows.insert(*tag, review_heads(rel, marker));
    }

    let pending = read(PENDING);
    rows.insert(
        "M53 Settle",
        settle_rows(&pending, "### Deferred at the M53 Settle"),
    );
    rows.insert(
        "M52 Settle",
        settle_rows(&pending, "### Deferred at the M52 Settle"),
    );
    let labels = |lines: Vec<&str>| -> BTreeSet<String> {
        lines
            .into_iter()
            .filter_map(row_label)
            .map(str::to_owned)
            .collect()
    };
    rows.insert(
        "owed",
        labels(section(
            &pending,
            "### Owed after M53's post-review arcs",
            |l| l.starts_with("### "),
        )),
    );
    rows.insert(
        "CI",
        labels(section(&pending, "**CI — ", |l| {
            l.starts_with("**") || l.starts_with("### ")
        })),
    );

    let channel = read(CHANNEL);
    let bounds = channel
        .lines()
        .find(|l| l.starts_with("**Declared bounds, seeded open as"))
        .expect("findings-channel.md → 6 states its declared bounds");
    // The bold spans after the paragraph's own bold lead-in are the bounds' labels.
    let bold = bounds.split("**").skip(1).step_by(2).skip(1);
    rows.insert("bound", bold.map(str::to_owned).collect());

    for (tag, ids) in &rows {
        assert!(
            !ids.is_empty(),
            "the `{tag}` source read no rows — its reader is stale"
        );
    }
    rows
}

/// `rel` resolved against `base`'s directory, lexically, as a repo-relative path.
fn resolve(base: &str, rel: &str) -> String {
    let joined = Path::new(base).parent().unwrap().join(rel);
    let mut out: Vec<&str> = Vec::new();
    for c in joined.components() {
        match c {
            Component::ParentDir => {
                out.pop();
            }
            Component::Normal(s) => out.push(s.to_str().unwrap()),
            _ => {}
        }
    }
    out.join("/")
}

/// A source row: its tag and its own id.
type Source = (String, String);

/// One `[<tag>](<file>) <id>` source; returns it and what follows the id.
fn parse_source(s: &str) -> (Source, String) {
    let s = s.trim();
    let (tag, rest) =
        delimited(s, "[", "](").unwrap_or_else(|| panic!("`{s}` opens with no `[<tag>](`"));
    let (link, rest) = rest
        .split_once(')')
        .unwrap_or_else(|| panic!("`{s}`: unclosed link"));
    let (_, file, _) = TAGS
        .iter()
        .find(|(t, ..)| *t == tag)
        .unwrap_or_else(|| panic!("`{s}`: unknown source tag `{tag}`"));
    assert_eq!(
        resolve(LEDGER, link),
        *file,
        "`{s}`: the `{tag}` link must name its source file"
    );
    let (id, after) = source_id(rest.trim_start());
    ((tag.to_owned(), id.to_owned()), after.trim().to_owned())
}

struct Row {
    key: String,
    doctype: String,
    sources: Vec<Source>,
    /// The re-drive's verdict, empty until the row is re-driven.
    verdict: String,
    /// The seed doc cell, empty until the seed is filed.
    seed_doc: String,
}

/// The ledger's rows and its exclusions.
fn ledger() -> (Vec<Row>, Vec<(Source, String)>) {
    let text = read(LEDGER);
    let (head, table) = text
        .split_once(&format!("\n{LEDGER_HEADING}\n"))
        .unwrap_or_else(|| panic!("the ledger has no `{LEDGER_HEADING}` heading"));
    let exclusions = head
        .lines()
        .filter(|l| l.starts_with("- ["))
        .map(|l| parse_source(&l[2..]))
        .collect();
    let rows = table
        .lines()
        .filter(|l| l.starts_with("| `"))
        .map(|l| {
            let cells: Vec<&str> = l.trim_matches('|').split('|').map(str::trim).collect();
            assert_eq!(cells.len(), 5, "a ledger row has five cells: `{l}`");
            let unquote = |c: &str| c.trim_matches('`').to_owned();
            Row {
                key: unquote(cells[0]),
                doctype: unquote(cells[1]),
                sources: cells[2]
                    .split("<br>")
                    .map(|s| {
                        let (source, rest) = parse_source(s);
                        assert!(
                            rest.is_empty(),
                            "`{}`: text after the source id: `{rest}`",
                            cells[0]
                        );
                        source
                    })
                    .collect(),
                verdict: cells[3].to_owned(),
                seed_doc: cells[4].to_owned(),
            }
        })
        .collect();
    (rows, exclusions)
}

#[test]
fn the_ledger_carries_every_source_row_exactly_once() {
    let sources = source_rows();
    let (rows, exclusions) = ledger();
    assert!(!rows.is_empty(), "the ledger carries no rows");

    let mut seen: BTreeMap<&Source, Vec<&str>> = BTreeMap::new();
    for row in &rows {
        for source in &row.sources {
            seen.entry(source).or_default().push(&row.key);
        }
    }
    let excluded: BTreeSet<&Source> = exclusions.iter().map(|(s, _)| s).collect();
    for ((tag, id), reason) in &exclusions {
        assert!(
            !reason.trim_start_matches('—').trim().is_empty(),
            "the exclusion `{tag} {id}` states no reason"
        );
    }

    let mut problems = Vec::new();
    let seeded = |tag: &str| {
        TAGS.iter()
            .any(|(t, _, prefix)| *t == tag && !prefix.is_empty())
    };
    for (tag, ids) in sources.iter().filter(|(tag, _)| seeded(tag)) {
        for id in ids {
            let source = (tag.to_string(), id.clone());
            match (
                seen.get(&source).map(Vec::len).unwrap_or(0),
                excluded.contains(&source),
            ) {
                (1, false) | (0, true) => {}
                (0, false) => problems.push(format!(
                    "`{tag}` row `{id}` is in no ledger row and not excluded"
                )),
                (n, _) => problems.push(format!(
                    "`{tag}` row `{id}` sits in {n} ledger rows ({:?}){}",
                    seen[&source],
                    if excluded.contains(&source) {
                        " and is excluded"
                    } else {
                        ""
                    }
                )),
            }
        }
    }
    for (tag, id) in seen.keys().chain(excluded.iter()).copied() {
        if !sources
            .get(tag.as_str())
            .is_some_and(|ids| ids.contains(id))
        {
            problems.push(format!(
                "`{tag}` carries no row `{id}`, which the ledger names"
            ));
        }
    }
    assert!(
        problems.is_empty(),
        "the ledger and its sources disagree:\n  {}",
        problems.join("\n  ")
    );
}

#[test]
fn every_key_is_unique_filesystem_safe_and_prefixed_by_its_first_source() {
    let (rows, _) = ledger();
    let mut keys = BTreeSet::new();
    for row in &rows {
        let key = &row.key;
        assert!(keys.insert(key), "the key `{key}` is used twice");
        assert!(
            key.split('-').all(|w| !w.is_empty()
                && w.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())),
            "`{key}` is not `a-z0-9` words joined by `-`"
        );
        let (tag, id) = &row.sources[0];
        let (_, _, prefix) = TAGS.iter().find(|(t, ..)| t == tag).unwrap();
        let expected = match tag.as_str() {
            "register" => format!("{prefix}{}", id.to_lowercase()),
            _ => (*prefix).to_owned(),
        };
        let fits = if tag == "register" {
            *key == expected
        } else {
            key.starts_with(&expected) && key.len() > expected.len()
        };
        assert!(
            fits,
            "`{key}`: its first source is `{tag}`, so it is `{expected}…`"
        );

        for (tag, id) in &row.sources {
            let want = if tag == "register" && id.starts_with('D') {
                "inconsistency"
            } else {
                "jigc-feedback"
            };
            assert_eq!(
                row.doctype, want,
                "`{key}`: its source `{tag} {id}` files as `{want}`"
            );
        }
    }
}

#[test]
fn the_ledger_states_its_own_count() {
    let (rows, _) = ledger();
    let text = read(LEDGER);
    let line = text
        .lines()
        .find(|l| l.starts_with("**The count: "))
        .expect("the ledger states its count on a `**The count: ` line");
    let numbers: Vec<usize> = line
        .split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse().unwrap())
        .collect();
    let of = |ty: &str| rows.iter().filter(|r| r.doctype == ty).count();
    let entries: usize = rows.iter().map(|r| r.sources.len()).sum();
    assert_eq!(
        numbers[..4],
        [
            rows.len(),
            of("jigc-feedback"),
            of("inconsistency"),
            entries
        ],
        "`{line}` must state the rows, the two doctypes' rows and the source entries the table carries"
    );
}

/// The per-row filing inputs, repo-relative: `input/<doctype>/<key>/` (P4).
const INPUTS: &str = "completions/artifacts/M55/seed-filing/input";

/// The verdicts a re-drive may give each doctype (P3): an `inconsistency` may also be
/// `intended`.
fn verdicts(doctype: &str) -> &'static [&'static str] {
    match doctype {
        "inconsistency" => &["open", "resolved", "refuted", "intended"],
        _ => &["open", "resolved", "refuted"],
    }
}

/// The names of the entries of `dir`, or none when it does not exist yet.
fn entries(dir: &Path) -> BTreeSet<String> {
    match fs::read_dir(dir) {
        Ok(read) => read
            .map(|e| {
                e.expect("a readable entry")
                    .file_name()
                    .into_string()
                    .unwrap()
            })
            .collect(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => BTreeSet::new(),
        Err(e) => panic!("`{}` must be readable: {e}", dir.display()),
    }
}

/// A re-driven row's verdict is its input's `status`, and only a re-driven row has an input
/// (P3, P4): a ledger row with a verdict has `input/<doctype>/<key>/`, whose `fields` carries
/// exactly one `status <value>` line equal to the verdict; a row without one has no input; and
/// every input directory is a ledger key under its own doctype. The driver files what the
/// input says, so this is what holds the filed status to the ledger before T7's bijection.
#[test]
fn every_redriven_row_has_an_input_carrying_its_verdict() {
    let (rows, _) = ledger();
    let inputs = repo_root().join(INPUTS);
    let mut problems = Vec::new();

    for row in &rows {
        let dir = inputs.join(&row.doctype).join(&row.key);
        let rel = format!("{INPUTS}/{}/{}", row.doctype, row.key);
        if row.verdict.is_empty() {
            if dir.exists() {
                problems.push(format!("`{rel}` exists, but its ledger row has no verdict"));
            }
            continue;
        }
        if !verdicts(&row.doctype).contains(&row.verdict.as_str()) {
            problems.push(format!(
                "`{}`: the verdict `{}` is not one of {:?}",
                row.key,
                row.verdict,
                verdicts(&row.doctype)
            ));
        }
        let fields = match fs::read_to_string(dir.join("fields")) {
            Ok(fields) => fields,
            Err(e) => {
                problems.push(format!(
                    "`{}` is re-driven (`{}`), but `{rel}/fields` is unreadable: {e}",
                    row.key, row.verdict
                ));
                continue;
            }
        };
        let status: Vec<&str> = fields
            .lines()
            .filter_map(|l| l.strip_prefix("status "))
            .collect();
        if status != [row.verdict.as_str()] {
            problems.push(format!(
                "`{rel}/fields` sets status {status:?}; its ledger verdict is `{}`",
                row.verdict
            ));
        }
    }

    let keys: BTreeSet<(&str, &str)> = rows
        .iter()
        .map(|r| (r.doctype.as_str(), r.key.as_str()))
        .collect();
    for doctype in entries(&inputs) {
        if !["jigc-feedback", "inconsistency"].contains(&doctype.as_str()) {
            problems.push(format!(
                "`{INPUTS}/{doctype}` is not a doctype's input home"
            ));
            continue;
        }
        for key in entries(&inputs.join(&doctype)) {
            if !keys.contains(&(doctype.as_str(), key.as_str())) {
                problems.push(format!(
                    "`{INPUTS}/{doctype}/{key}` is no `{doctype}` row of the ledger"
                ));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "the inputs and the ledger's verdicts disagree:\n  {}",
        problems.join("\n  ")
    );
}

/// The committed seed, repo-relative (P1).
const SEED: &str = "completions/artifacts/M55/seed";

/// Each doctype's home under `seed/`, as the doctype lays its instances out.
fn home(doctype: &str) -> &'static str {
    match doctype {
        "inconsistency" => "inconsistencies",
        _ => "jigc-feedback",
    }
}

/// A row's seed doc, repo-relative: its cell is `[<home>/<slug>.md](seed/<home>/<slug>.md)`,
/// the link's text its target under `seed/`, and the target a `.md` under its doctype's home.
fn seed_doc(row: &Row) -> Result<String, String> {
    let cell = &row.seed_doc;
    let (text, rest) = delimited(cell, "[", "](")
        .ok_or_else(|| format!("`{}`: the seed doc cell `{cell}` is no link", row.key))?;
    let target = rest
        .strip_suffix(')')
        .ok_or_else(|| format!("`{}`: the seed doc cell `{cell}` is no link", row.key))?;
    if target.strip_prefix("seed/") != Some(text) {
        return Err(format!(
            "`{}`: the seed doc link `{cell}` must read its own target under `seed/`",
            row.key
        ));
    }
    let path = resolve(LEDGER, target);
    let slug = path
        .strip_prefix(&format!("{SEED}/{}/", home(&row.doctype)))
        .and_then(|rest| rest.strip_suffix(".md"))
        .filter(|slug| !slug.is_empty() && !slug.contains('/'));
    match slug {
        Some(_) => Ok(path),
        None => Err(format!(
            "`{}`: the seed doc `{path}` is no `.md` under `{SEED}/{}/`",
            row.key,
            home(&row.doctype)
        )),
    }
}

/// Every re-driven row carries its verdict, so every row is filed (T7).
#[test]
fn every_row_carries_a_verdict() {
    let (rows, _) = ledger();
    let blank: Vec<&str> = rows
        .iter()
        .filter(|r| r.verdict.is_empty())
        .map(|r| r.key.as_str())
        .collect();
    assert!(blank.is_empty(), "rows without a verdict: {blank:?}");
}

/// **The number of filed docs equals the recorded set, row for row** (T7): `seed/` holds
/// only the two homes; each row names one seed doc under its doctype's home, which exists,
/// is named by no other row, and is titled as the row's input; and every file under the two
/// homes is some row's seed doc.
#[test]
fn the_ledger_rows_and_the_seed_files_are_a_bijection() {
    let (rows, _) = ledger();
    let root = repo_root();
    let seed = root.join(SEED);
    let mut problems = Vec::new();

    let homes: BTreeSet<String> = ["inconsistencies", "jigc-feedback"]
        .map(str::to_owned)
        .into();
    if entries(&seed) != homes {
        problems.push(format!(
            "`{SEED}` holds {:?}; it holds only the two homes {homes:?}",
            entries(&seed)
        ));
    }
    let mut on_disk = BTreeSet::new();
    for home in &homes {
        for name in entries(&seed.join(home)) {
            on_disk.insert(format!("{SEED}/{home}/{name}"));
        }
    }

    let mut named: BTreeMap<String, &str> = BTreeMap::new();
    for row in &rows {
        let path = match seed_doc(row) {
            Ok(path) => path,
            Err(problem) => {
                problems.push(problem);
                continue;
            }
        };
        if let Some(other) = named.insert(path.clone(), &row.key) {
            problems.push(format!(
                "`{path}` is the seed doc of `{other}` and `{}`",
                row.key
            ));
        }
        let Ok(doc) = fs::read_to_string(root.join(&path)) else {
            problems.push(format!(
                "`{}`: its seed doc `{path}` is unreadable",
                row.key
            ));
            continue;
        };
        let input = format!("{INPUTS}/{}/{}/title", row.doctype, row.key);
        let title = read(&input);
        let h1 = doc.lines().find_map(|l| l.strip_prefix("# "));
        if h1 != Some(title.trim_end()) {
            problems.push(format!(
                "`{path}` is titled {h1:?}; `{}`'s input `{input}` is {:?}",
                row.key,
                title.trim_end()
            ));
        }
    }
    for path in on_disk.iter().filter(|p| !named.contains_key(*p)) {
        problems.push(format!("`{path}` is no ledger row's seed doc"));
    }
    assert!(
        problems.is_empty(),
        "the ledger and the filed seed disagree:\n  {}",
        problems.join("\n  ")
    );
}

/// Each seed doc's `status` is its row's verdict (T7): the filed record says what the
/// re-drive found.
#[test]
fn each_seed_doc_status_equals_its_verdict() {
    let (rows, _) = ledger();
    let mut problems = Vec::new();
    for row in &rows {
        let path = match seed_doc(row) {
            Ok(path) => path,
            Err(problem) => {
                problems.push(problem);
                continue;
            }
        };
        let doc = fs::read_to_string(repo_root().join(&path)).unwrap_or_default();
        let front: Vec<&str> = doc
            .lines()
            .skip(1)
            .take_while(|l| *l != "---")
            .filter_map(|l| l.strip_prefix("status: "))
            .collect();
        if front != [row.verdict.as_str()] {
            problems.push(format!(
                "`{path}` carries status {front:?}; `{}`'s verdict is `{}`",
                row.key, row.verdict
            ));
        }
    }
    assert!(
        problems.is_empty(),
        "the seed docs and the ledger's verdicts disagree:\n  {}",
        problems.join("\n  ")
    );
}

/// The sources a seeded row points back from (§7, T8): a `decisions-pending.md` row and a
/// register row each link the seed doc that now carries them. A review README is dated and
/// never edited, and a declared bound stays stated in its design home, so neither points.
const POINTING: &[&str] = &["M53 Settle", "owed", "CI", "register"];

/// The relative link targets on `line`, each resolved against `base` (repo-relative).
fn link_targets(base: &str, line: &str) -> Vec<String> {
    line.split("](")
        .skip(1)
        .filter_map(|rest| rest.split_once(')'))
        .map(|(target, _)| resolve(base, target))
        .collect()
}

/// The one line carrying a source row, read where [`source_rows`] reads it.
fn source_line<'a>(pending: &'a str, register: &'a str, tag: &str, id: &str) -> Option<&'a str> {
    let rows = |heading: &str, stop: fn(&str) -> bool| section(pending, heading, stop);
    let lines = match tag {
        "register" => register.lines().collect(),
        "M53 Settle" => rows("### Deferred at the M53 Settle", |l| l.starts_with("### ")),
        "owed" => rows("### Owed after M53's post-review arcs", |l| {
            l.starts_with("### ")
        }),
        "CI" => rows("**CI — ", |l| {
            l.starts_with("**") || l.starts_with("### ")
        }),
        _ => return None,
    };
    let label = |l: &&str| match tag {
        "register" => l.starts_with(&format!("| {id} |")),
        "M53 Settle" => row_label(l).is_some_and(|label| label.starts_with(id)),
        _ => row_label(l) == Some(id),
    };
    lines.into_iter().find(label)
}

/// **A seeded row points at its seed doc, and a `decisions-pending.md` row is only the
/// pointer** (§1.6, §7; T8). Each register row's disposition cell links its seed doc. Each
/// seeded `decisions-pending.md` row collapses to one line — its opening bold label, kept
/// verbatim so [`source_rows`] still finds it, then `Seeded as [<home>/<slug>.md](<link>).`
/// and the row's `*Trigger:*` clause where it had one — so the finding's text has one home,
/// the seed doc, and the row keeps only what makes it a deferral.
#[test]
fn every_seeded_pending_and_register_row_points_at_its_seed_doc() {
    let (rows, _) = ledger();
    let pending = read(PENDING);
    let register = read(REGISTER);
    let mut problems = Vec::new();
    for row in &rows {
        let Ok(doc) = seed_doc(row) else { continue };
        for (tag, id) in row
            .sources
            .iter()
            .filter(|(t, _)| POINTING.contains(&t.as_str()))
        {
            let Some(line) = source_line(&pending, &register, tag, id) else {
                problems.push(format!("`{tag}` row `{id}` is not found"));
                continue;
            };
            if tag == "register" {
                let disposition = line.trim_end_matches('|').rsplit(" | ").next().unwrap();
                if !link_targets(REGISTER, disposition).contains(&doc) {
                    problems.push(format!(
                        "register row `{id}`'s disposition cell does not link `{doc}`"
                    ));
                }
                continue;
            }
            let text = doc.strip_prefix(&format!("{SEED}/")).unwrap();
            let link = format!("[{text}]({})", relative_from(PENDING, &doc));
            let label = row_label(line).unwrap();
            let pointer = format!("- **{label}** Seeded as {link}.");
            let rest = line.strip_prefix(&pointer);
            if !rest.is_some_and(|r| r.is_empty() || r.starts_with(" *Trigger:* ")) {
                problems.push(format!(
                    "`{tag}` row `{id}` is not one pointer line `{pointer}[ *Trigger:* …]`: `{line}`"
                ));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "the seeded rows and their seed docs disagree:\n  {}",
        problems.join("\n  ")
    );
}

/// `target` (repo-relative) as a link from `base`'s directory.
fn relative_from(base: &str, target: &str) -> String {
    let ups = base.matches('/').count();
    format!("{}{target}", "../".repeat(ups))
}
