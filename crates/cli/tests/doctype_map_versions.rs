//! The doctype map states each doctype's **shipped** `schema-version` (M46
//! Increment 9, T1 — rider **R9**).
//!
//! [`implementation/doctype-map.md`](../../../implementation/doctype-map.md) is the
//! artifact the milestone-planning *detect gaps* phase reads to size a doctype
//! change ([`milestone-planning-workflow.md`](../../../implementation/milestone-planning-workflow.md)
//! → Where this sits). At the rider's discovery it recorded `adr` as *"frozen v1"*
//! while `crates/cli/pack/config/schema-manifest.yaml` declared **2**, and every
//! methodology row was version-blind — so the map produced a wrong sizing estimate
//! inside this wave's own planning. A prose file that no test reads drifts from the
//! manifests silently; this suite is that file's first fence.
//!
//! **The subject is derived, never listed.** Both shipped manifests are read and
//! *every* declared entry is enumerated — the map's row for a doctype must state
//! that doctype's current `schema-version`, and the map's row set must equal the
//! declared doctype set. Adding, removing or bumping a doctype therefore reddens
//! here until the map is corrected, which is the whole point: the map's job is to be
//! current, and currency is exactly what prose cannot promise on its own.
//!
//! **Keyed per (pack, doctype), and red on divergence.** `commit` is declared in
//! *both* packs (the dev pack's persisted-set shadow and the methodology pack's), at
//! the same version today, and the map carries **one** row for it. If the two ever
//! diverge, this suite fails rather than silently picking one — a single row cannot
//! state two versions, and the honest signal is that the map's shape no longer fits
//! the manifests ([`corpus-migration.md`](../../../design/corpus-migration.md) → The
//! freeze-exempt sibling, the M40 revision: a doctype's governing entry resolves
//! per-origin-pack).
//!
//! **The freeze scope pin is fenced against its own staleness.** `:41` described the
//! M40 state — ten methodology schemas, all at v1, `schema-snapshots/` absent —
//! which two later bumps (`deferral-ledger` at M41 F4, `milestone-record` at M42)
//! falsified. The arm derives the above-v1 set from the manifests and requires the
//! pin to name each member *with* its version, and forbids any sentence that both
//! mentions `schema-snapshots/` and calls it absent.

use engine::manifest::Manifest;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// The dev pack's frozen doctype-set manifest.
const DEV_MANIFEST: &str = "crates/cli/pack/config/schema-manifest.yaml";

/// The methodology pack's own manifest (M40 A1 — each pack declares its own set).
const METHODOLOGY_MANIFEST: &str = "packs/methodology/config/schema-manifest.yaml";

/// Both shipped manifests, in read order.
const MANIFESTS: [&str; 2] = [DEV_MANIFEST, METHODOLOGY_MANIFEST];

/// The planning aid under test.
const MAP: &str = "implementation/doctype-map.md";

/// The map heading whose table carries one row per doctype.
const TABLE_HEADING: &str = "## The doctypes";

/// The column whose cell must state the version.
const STATUS_COLUMN: &str = "Status";

/// The paragraph that scopes the freeze — `doctype-map.md:41`.
const SCOPE_PIN_PREFIX: &str = "**Scope pin";

/// This repo — the map and the manifests are both checked-in, so every arm runs
/// against the checkout rather than a fabricated tree.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

fn read(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("{rel} is readable: {e}"))
}

fn manifest(rel: &str) -> Manifest {
    serde_yaml_ng::from_str(&read(rel)).unwrap_or_else(|e| panic!("{rel} deserializes: {e}"))
}

/// Every declared entry, keyed **per (pack, doctype)**: `doctype -> manifest ->
/// schema-version`. Nothing is collapsed here — collapsing is where a divergence
/// would be silently resolved.
fn declared_per_pack() -> BTreeMap<String, BTreeMap<&'static str, u32>> {
    let mut per_pack: BTreeMap<String, BTreeMap<&'static str, u32>> = BTreeMap::new();
    for rel in MANIFESTS {
        for entry in manifest(rel).doctypes {
            let prior = per_pack
                .entry(entry.ty.clone())
                .or_default()
                .insert(rel, entry.schema_version);
            assert!(
                prior.is_none(),
                "{rel} declares `{}` twice — the manifest's own set-equality assert owns this, \
                 but the map fence must not average two entries",
                entry.ty
            );
        }
    }
    assert!(
        !per_pack.is_empty(),
        "both manifests parsed to an empty doctype set — the fence would then assert nothing"
    );
    per_pack
}

/// The version the map's single row per doctype must state. Panics — deliberately —
/// when the two packs disagree: one row cannot state two versions.
fn declared_versions() -> BTreeMap<String, u32> {
    declared_per_pack()
        .into_iter()
        .map(|(ty, per_pack)| {
            let versions: BTreeSet<u32> = per_pack.values().copied().collect();
            assert_eq!(
                versions.len(),
                1,
                "`{ty}` is declared at diverging schema-versions across packs ({per_pack:?}) — \
                 {MAP} carries ONE row for it, so the map's shape no longer fits the manifests; \
                 split the row before re-pinning either version"
            );
            let version = *versions.iter().next().expect("one distinct version");
            (ty, version)
        })
        .collect()
}

/// One row of the map's doctype table.
struct Row {
    doctype: String,
    status: String,
    line: usize,
}

/// The map's doctype table, parsed from the `## The doctypes` section. The status
/// column is located by **header name**, not by position.
fn map_rows(text: &str) -> Vec<Row> {
    let mut lines = text
        .lines()
        .enumerate()
        .skip_while(|(_, l)| !l.starts_with(TABLE_HEADING));
    assert!(
        lines.next().is_some(),
        "{MAP} has no `{TABLE_HEADING}` section — the fence's subject moved"
    );
    let table: Vec<(usize, &str)> = lines
        .map(|(i, l)| (i + 1, l.trim()))
        .skip_while(|(_, l)| !l.starts_with('|'))
        .take_while(|(_, l)| l.starts_with('|'))
        .collect();
    assert!(table.len() > 2, "{MAP}: `{TABLE_HEADING}` carries no rows");

    let header = cells(table[0].1);
    let status_at = header
        .iter()
        .position(|c| *c == STATUS_COLUMN)
        .unwrap_or_else(|| panic!("{MAP}: the doctype table has no `{STATUS_COLUMN}` column"));

    table[2..]
        .iter()
        .map(|(line, raw)| {
            let cells = cells(raw);
            assert_eq!(
                cells.len(),
                header.len(),
                "{MAP}:{line}: row has {} cells, header has {}",
                cells.len(),
                header.len()
            );
            Row {
                doctype: cells[0].trim_matches('`').to_string(),
                status: cells[status_at].to_string(),
                line: *line,
            }
        })
        .collect()
}

fn cells(row: &str) -> Vec<&str> {
    row.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(str::trim)
        .collect()
}

/// Every schema-version this text **states**, in either accepted spelling: a `v<N>`
/// token (not word-internal, so `M40`/`rev1` are not versions) and a `version <N>`
/// phrase (which is how `schema-version 2` reads). A cell that states none is
/// version-blind; a cell that states one that disagrees with the manifest is a lie —
/// both are the rider's defect.
fn stated_versions(text: &str) -> Vec<u32> {
    let bytes: Vec<char> = text.chars().collect();
    let mut found = Vec::new();
    for (i, c) in bytes.iter().enumerate() {
        let boundary = i == 0 || !bytes[i - 1].is_alphanumeric();
        if boundary
            && (*c == 'v' || *c == 'V')
            && let Some(n) = digits_at(&bytes, i + 1)
        {
            found.push(n);
        }
    }
    let lower: Vec<char> = text.to_lowercase().chars().collect();
    let needle: Vec<char> = "version".chars().collect();
    for start in 0..lower.len() {
        if lower[start..].starts_with(needle.as_slice()) {
            let mut at = start + needle.len();
            while at < lower.len() && (lower[at] == ' ' || lower[at] == ':' || lower[at] == '-') {
                at += 1;
            }
            if let Some(n) = digits_at(&lower, at) {
                found.push(n);
            }
        }
    }
    found
}

/// Whether `text` contains `word` as a **word**, not as a substring — `ten` must not
/// be found inside `content` or `written`, or the aggregate guard below would fire on
/// prose that names no count at all.
fn contains_word(text: &str, word: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let needle: Vec<char> = word.chars().collect();
    (0..chars.len()).any(|start| {
        chars[start..].starts_with(needle.as_slice())
            && (start == 0 || !chars[start - 1].is_alphanumeric())
            && chars
                .get(start + needle.len())
                .is_none_or(|c| !c.is_alphanumeric())
    })
}

fn digits_at(chars: &[char], from: usize) -> Option<u32> {
    let digits: String = chars[from..]
        .iter()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().ok()
}

/// The scope-pin paragraph, split into sentence-ish fragments so a claim is judged in
/// the clause that makes it rather than across the whole paragraph.
fn scope_pin_fragments() -> Vec<String> {
    let text = read(MAP);
    let pins: Vec<&str> = text
        .lines()
        .filter(|l| l.trim_start().starts_with(SCOPE_PIN_PREFIX))
        .collect();
    assert_eq!(
        pins.len(),
        1,
        "{MAP}: expected exactly one `{SCOPE_PIN_PREFIX}` paragraph, found {}",
        pins.len()
    );
    pins[0]
        .split(". ")
        .flat_map(|s| s.split("; "))
        .map(str::to_string)
        .collect()
}

/// The map's row for every declared doctype states that doctype's current
/// `schema-version` — the rider's headline.
#[test]
fn every_doctype_row_states_its_declared_schema_version() {
    let declared = declared_versions();
    let text = read(MAP);
    let rows = map_rows(&text);
    let by_type: BTreeMap<&str, &Row> = rows.iter().map(|r| (r.doctype.as_str(), r)).collect();

    let mut wrong = Vec::new();
    for (ty, version) in &declared {
        let Some(row) = by_type.get(ty.as_str()) else {
            continue; // set equality is the arm below's subject
        };
        let stated = stated_versions(&row.status);
        if stated.is_empty() {
            wrong.push(format!(
                "{MAP}:{}: `{ty}` states no schema-version (`{}`), while the manifests declare {version}",
                row.line, row.status
            ));
        } else if stated.iter().any(|s| *s != *version) {
            wrong.push(format!(
                "{MAP}:{}: `{ty}` states {stated:?} (`{}`), while the manifests declare {version}",
                row.line, row.status
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "the doctype map is the artifact `detect gaps` sizes a doctype change against, \
         and these rows misstate the shipped version:\n{}",
        wrong.join("\n")
    );
}

/// Every declared doctype has a row, and every row is a declared doctype — so a
/// doctype cannot join or leave a manifest while the planning aid keeps its old set.
#[test]
fn the_map_row_set_equals_the_declared_doctype_set() {
    let declared: BTreeSet<String> = declared_versions().into_keys().collect();
    let text = read(MAP);
    let rows: BTreeSet<String> = map_rows(&text).into_iter().map(|r| r.doctype).collect();
    assert_eq!(
        rows, declared,
        "{MAP}'s doctype table and the two shipped manifests describe different doctype sets"
    );
}

/// `commit` is declared in both packs; the map carries one row. A divergence must
/// **redden**, never be silently collapsed to one of the two.
#[test]
fn a_doctype_declared_in_both_packs_is_keyed_per_pack() {
    let per_pack = declared_per_pack();
    let shared: Vec<&String> = per_pack
        .iter()
        .filter(|(_, packs)| packs.len() > 1)
        .map(|(ty, _)| ty)
        .collect();
    assert!(
        !shared.is_empty(),
        "no doctype is declared in both packs — the per-pack keying this fence relies on \
         has become untestable, and the divergence guard is now dead weight"
    );
    // The collapse refuses a divergence rather than picking a side.
    let collapsed = declared_versions();
    for ty in shared {
        let packs = &per_pack[ty];
        for (rel, version) in packs {
            assert_eq!(
                collapsed[ty], *version,
                "`{ty}` is declared at {version} in {rel} but the map fence resolved {} — \
                 a divergence must fail, not average",
                collapsed[ty]
            );
        }
    }
}

/// The freeze scope pin no longer describes the M40 ten-at-v1 state: it names each
/// methodology doctype that has moved past v1 **with** its version, states no
/// aggregate version over the set while the set is mixed, and does not call
/// `schema-snapshots/` absent.
#[test]
fn the_freeze_scope_pin_states_the_current_methodology_state() {
    let methodology: BTreeMap<String, u32> = manifest(METHODOLOGY_MANIFEST)
        .doctypes
        .into_iter()
        .map(|e| (e.ty, e.schema_version))
        .collect();
    let above_v1: Vec<(&String, &u32)> = methodology.iter().filter(|(_, v)| **v > 1).collect();
    assert!(
        !above_v1.is_empty(),
        "no methodology doctype is past v1 — the pin's ten-at-v1 description would be current \
         again, and this arm asserts nothing"
    );
    let fragments = scope_pin_fragments();

    let mut wrong = Vec::new();
    for (ty, version) in &above_v1 {
        let stated = fragments
            .iter()
            .filter(|f| f.contains(ty.as_str()))
            .any(|f| stated_versions(f).contains(version));
        if !stated {
            wrong.push(format!(
                "the scope pin never states `{ty}` at v{version} — a reader sizing a methodology \
                 change still reads the M40 all-at-v1 state"
            ));
        }
    }
    for fragment in &fragments {
        if fragment.contains("schema-snapshots") && fragment.contains("absent") {
            wrong.push(format!(
                "the scope pin still calls `schema-snapshots/` absent, and it is not: {fragment}"
            ));
        }
        if contains_word(fragment, "ten") && !stated_versions(fragment).is_empty() {
            wrong.push(format!(
                "the scope pin states one version over the whole ten-schema set, which is mixed \
                 ({above_v1:?}): {fragment}"
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "{MAP}'s freeze scope pin has drifted from the shipped manifests:\n{}",
        wrong.join("\n")
    );
}
