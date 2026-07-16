//! The committed-store reader — resolve a `<type>:<slug>` address to its canonical
//! path, parse the committed `.md` against its schema, and slice a `#fragment` to
//! its content (a section slot's prose, or a repeatable section's rendered items).
//!
//! This is the byte-read the compose-path [`Resolution::Content`](crate::data_value::Resolution::Content)
//! handle deferred and the superseding-decision flow mandates
//! ([worked-examples.md](../../../design/worked-examples.md) → Superseding decision):
//! reaching `{{@task.decision.supersedes#decision}}` re-reads the committed
//! `decisions/single-node-cache.md` and parses it losslessly to extract the section.
//!
//! ## Identity is the path ([storage.md](../../../design/storage.md) → Identity is the path)
//!
//! A managed doc's identity *is* its path: `decisions/single-node-cache.md` → type
//! `adr`, id `single-node-cache`. So resolving `adr:single-node-cache` to a file is
//! purely `<repo-root>/<schema.location>/<slug>.md` — no redundant id is read from
//! the file. A type with **no `location:`** (a transient sink type like `commit`,
//! whose sink is the git message) has no committed path and is not
//! committed-store-readable — its **staged** working copy in an open task is, via
//! [`read_slice_staged`] (M43, the staged arm).
//!
//! ## The round-trip on a committed file ([parsing.md](../../../implementation/parsing.md) → Round-trip guarantees)
//!
//! The slice is taken over the committed bytes via [`crate::parse::parse_sections`]
//! and the recorded opaque [`Span`](crate::parse::Span) — the same byte-stable read
//! path the writer is the inverse of. Slicing re-reads the source over the span, so
//! the returned prose is byte-for-byte the committed file's slot bytes. A
//! missing/unparseable target is a blocking [`Finding`] carrying a route — never a
//! panic, never a silent empty (the settled block-payload shape, `DECISIONS.md`
//! 2026-05-31).

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use crate::address::{Address, Fragment};
use crate::finding::{Finding, Location, Route, Severity};
use crate::parse::{self, Document, ParsedItem};
use crate::schema::Schema;
use crate::write;

/// The canonical on-disk path a committed `<type>:<slug>` instance lives at, when
/// its schema declares a persisted home.
///
/// `<repo_root>/<location>/<slug>.md` — identity is the path ([storage.md](../../../design/storage.md)).
/// A **placement** doctype (`design/storage.md` → Placement) instead lives at its one
/// literal `placement.file` repo-root-relative path (case-preserved, bypassing docs-root
/// and the slug), so this returns `<repo_root>/<placement.file>` regardless of `slug`. A
/// type with **neither** (a transient sink type) has no committed path, so this returns
/// `None`.
pub fn canonical_path(repo_root: &Path, schema: &Schema, slug: &str) -> Option<PathBuf> {
    if let Some(placement) = &schema.placement {
        return Some(repo_root.join(&placement.file));
    }
    let location = schema.location.as_deref()?;
    Some(repo_root.join(location).join(format!("{slug}.md")))
}

/// Lexically normalize a path — drop `.` components and resolve `..` against the
/// accumulated prefix — **without touching the filesystem**.
///
/// The path-collision guards (`design/auto-migration.md` → Path-collision guard)
/// compare a recorded foreign source path against a canonical managed path; this
/// pass makes those comparisons spelling-insensitive (review F2/C1) so a path
/// recorded as `./changelog/changelog.md` (or with redundant `..` components) still
/// compares equal to the clean canonical `changelog/changelog.md`. Shared so the
/// retire-side (finalize) and create-side (state) guards normalize identically.
pub fn lexical_normalize(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in p.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Read the committed managed doc named by `address` and render it — the whole doc
/// when the address carries no fragment, or the addressed `#fragment` slice.
///
/// Resolves the address's `type` to a [`Schema`] in `schemas`, computes the
/// canonical path under `repo_root`, reads and parses the committed file against the
/// schema, then:
///
/// - **no fragment** → the whole committed doc, byte-for-byte over the committed
///   source (the canonical render is the file's own bytes — the round-trip guarantee);
/// - **`#unit`** → the section's content: a slot's opaque prose (byte-exact), a
///   repeatable section's rendered items, or a **fields-only** (header) section's field
///   group, canonically re-emitted (M42);
/// - **`#unit/item`** (a 2-hop fragment over a repeatable section) → that item
///   rendered;
/// - **deeper hops** → the **section-qualified write grammar** (the chain alternates
///   `item, nested-section, item, …` — M40, one canonical address): a chain ending on
///   a declared **nested section** slices to its nested items rendered, on an **item**
///   to that item rendered complete, and a trailing **leaf** hop to the addressed leaf
///   — a declared slot's opaque prose byte-for-byte, the block's `id-from` leaf (the
///   item's heading), or a per-item field's rendered value (see [`slice_fragment`]).
///
/// Every navigation hop is resolved over the already-parsed
/// [`crate::parse::ParsedSection`]/[`ParsedItem`] span data — the writer's
/// byte-stable inverse (the nested-section hops are validated against the schema, the
/// same walk the writer's locator uses).
///
/// Every failure is a blocking [`Finding`] carrying a `route`: an unknown type, a
/// transient (location-less) type, a missing file, an unparseable file, or a fragment
/// naming a section/item/leaf that does not exist in the committed doc.
pub fn read_slice(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    address: &Address,
) -> Result<String, Finding> {
    let address_str = address.to_string();
    let type_name = address.r#type.as_str();
    let slug = address.slug.as_str();
    let schema = resolve_read_schema(schemas, address, &address_str)?;

    // Identity is the path: `<repo_root>/<location>/<slug>.md`. A transient
    // (location-less) type has no committed path and is not committed-store-readable
    // — its staged working copy in an open task is ([`read_slice_staged`], M43).
    let Some(path) = canonical_path(repo_root, schema, slug) else {
        return Err(block(
            "store.transient-type",
            format!(
                "doctype `{type_name}` is transient (no `location:`); `{address_str}` is not committed"
            ),
            &address_str,
            Route::mechanical(
                [
                    "jigc",
                    "doc",
                    "show",
                    address_str.as_str(),
                    "--task",
                    "<task-id>",
                ],
                " — a transient doc is readable only as a task's staged working copy",
            ),
        ));
    };

    read_parse_slice(
        schema,
        &path,
        address,
        &address_str,
        "fix the committed file so it conforms to its schema",
        |err| {
            block(
                "store.not-found",
                format!(
                    "could not read `{address_str}` at `{}`: {err}",
                    path.display()
                ),
                &address_str,
                format!(
                    "create the referenced doc, or fix the reference to an existing one; a doc \
                     staged in an open task is not committed yet — read it with \
                     `jigc doc show {address_str} --task <task-id>`"
                ),
            )
        },
    )
}

/// Read the **staged** working copy of `address` in an open task — the instance at
/// `<task_dir>/docs/<type>:<slug>.md` ([`crate::state::instance_path`], the one owner
/// of the working-area layout) — through the **identical** parse/slice/render path
/// the committed read uses (M43, [surface-contract.md](../../../design/surface-contract.md)
/// → law 2: the staged read; the source-selection extraction).
///
/// Differences from the committed arm are confined to source selection:
///
/// - **no `canonical_path` gate** — a **transient** doctype (`commit:<task-id>`) is
///   legal here: its staged working copy is a real file even though it never commits
///   (the B9 staging-key leak's sanctioned read address, closed from the read side);
///   the singleton-slug guard stays (a staged singleton is still the one instance);
/// - an **absent staged instance** blocks `store.not-staged`, routed on the **real
///   state** under `repo_root`: a committed sibling exists → read it task-less
///   (`jigc doc show <addr>`); nothing exists anywhere → nothing to read yet.
pub fn read_slice_staged(
    repo_root: &Path,
    task_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
    address: &Address,
) -> Result<String, Finding> {
    let address_str = address.to_string();
    let schema = resolve_read_schema(schemas, address, &address_str)?;
    let path =
        crate::state::instance_path(task_dir, address.r#type.as_str(), address.slug.as_str());
    read_parse_slice(
        schema,
        &path,
        address,
        &address_str,
        "fix the staged working copy so it conforms to its schema",
        |_| not_staged_block(repo_root, schema, address, &address_str),
    )
}

/// Resolve `address`'s doctype to its schema — shared by both read arms: the
/// unknown-type block and the singleton-slug guard live here, so the staged arm
/// inherits them unchanged.
fn resolve_read_schema<'a>(
    schemas: &'a BTreeMap<String, Schema>,
    address: &Address,
    address_str: &str,
) -> Result<&'a Schema, Finding> {
    let type_name = address.r#type.as_str();
    let slug = address.slug.as_str();

    // Resolve the type to its schema.
    let Some(schema) = schemas.get(type_name) else {
        return Err(block(
            "store.unknown-type",
            format!("unknown doctype `{type_name}` for `{address_str}`"),
            address_str,
            "list the available doctypes with `jigc describe`".to_string(),
        ));
    };

    // A placement doctype is a singleton homed at its one literal `placement.file`, so
    // `canonical_path` resolves it regardless of `slug`. On the read path the only valid
    // address is the singleton's canonical slug (= the type id, mirroring how `create`
    // fixes a singleton's slug to `schema.ty`); any other slug names no committed doc, so
    // route a not-found block rather than silently returning the singleton's content for
    // an invalid reference (M39 read-surface hardening — a read verb must route a bad ref
    // like every other). This guard is read-scoped: the write/promote/reconcile callers
    // use `canonical_path` directly and are unaffected.
    if schema.placement.is_some() && slug != schema.ty {
        return Err(block(
            "store.not-found",
            format!(
                "`{address_str}` names no committed doc: `{type_name}` is a singleton, so its only address is `{type_name}:{}`",
                schema.ty
            ),
            address_str,
            format!(
                "read `{type_name}:{}` — a singleton doctype has one instance at a fixed slug",
                schema.ty
            ),
        ));
    }
    Ok(schema)
}

/// The shared parse/slice tail of both read arms: read the bytes at `path` (`missing`
/// shapes the arm-specific block when they cannot be read), strip a leading BOM,
/// parse against `schema`, and serve the whole doc or the addressed `#fragment` —
/// **one path**, so a staged read can never diverge from the committed read's
/// parse/slice/render semantics (M43 — the source-selection extraction).
fn read_parse_slice(
    schema: &Schema,
    path: &Path,
    address: &Address,
    address_str: &str,
    fix_route: &str,
    missing: impl FnOnce(std::io::Error) -> Finding,
) -> Result<String, Finding> {
    // Read the bytes; a missing file is a located block, not a panic.
    let mut source = std::fs::read_to_string(path).map_err(missing)?;
    // Tolerate a leading BOM on read (Windows-editor edits) before parse + slice.
    parse::strip_leading_bom(&mut source);

    // Parse the file against its schema; conformance failures surface the
    // first blocking finding (re-located onto the address for the caller).
    let doc = parse::parse_sections(schema, &source).map_err(|findings| {
        let why = findings
            .first()
            .map(|f| f.message.clone())
            .unwrap_or_else(|| "unparseable".to_string());
        block(
            "store.unparseable",
            format!(
                "`{address_str}` at `{}` does not parse: {why}",
                path.display()
            ),
            address_str,
            fix_route.to_string(),
        )
    })?;

    // No fragment → the whole doc, byte-for-byte (the parse above already
    // enforced conformance / surfaced the unparseable block).
    match &address.fragment {
        None => Ok(source),
        Some(fragment) => slice_fragment(schema, &doc, &source, fragment, address_str),
    }
}

/// The staged arm's absent-instance block, routed on the **real state** under
/// `repo_root` (blocking ⇒ route, the M43 Inc-1 floor): a committed sibling exists →
/// a [`Route::mechanical`] task-less read; nothing exists anywhere → nothing to read
/// yet (a transient doctype never has a committed sibling, so it lands here too).
fn not_staged_block(
    repo_root: &Path,
    schema: &Schema,
    address: &Address,
    address_str: &str,
) -> Finding {
    let committed_sibling =
        canonical_path(repo_root, schema, address.slug.as_str()).is_some_and(|p| p.is_file());
    if committed_sibling {
        block(
            "store.not-staged",
            format!("`{address_str}` is not staged in this task — only its committed copy exists"),
            address_str,
            Route::mechanical(
                ["jigc", "doc", "show", address_str],
                " — the task-less read serves the committed copy",
            ),
        )
    } else {
        block(
            "store.not-staged",
            format!(
                "`{address_str}` is not staged in this task and has no committed copy — nothing to read yet"
            ),
            address_str,
            "create or author the doc in this task first — a staged copy exists only after a write"
                .to_string(),
        )
    }
}

/// Build a blocking store-read [`Finding`] with a located message and a route
/// (a plain-`String` route is a [`Route`]-`Human` direction; the transient-type
/// block and the staged arm's committed-sibling block pass a [`Route::mechanical`]).
fn block(code: &str, message: String, address: &str, route: impl Into<Route>) -> Finding {
    Finding::graded(
        Severity::Blocking,
        code,
        message,
        Some(Location::addressed(address, 1, 1)),
        Some(route.into()),
    )
}

/// Slice the `#fragment` of a parsed `doc` to its content over `source`, or a
/// blocking finding when the fragment names no target that exists.
///
/// The first hop is always a section id. With **no further hops** the section slices
/// as before: a **slot** section (flow #5's `#decision`) returns its slot prose
/// byte-for-byte over the recorded opaque [`Span`](crate::parse::Span); a
/// **repeatable** section (no slot — e.g. the spec's `#criteria`) returns its items
/// rendered as a Content list (see [`render_item`]), so `{{@task.spec#criteria}}`
/// resolves to the criteria the implementer reads (`worked-examples.md` → flow 6).
/// Zero items yields empty content (the absent-value case, not an error).
///
/// **Further hops** resolve over the **section-qualified write grammar** (review
/// finding S1, `design/changelog.md` — the one canonical address; M40,
/// `design/doc-read-surface.md` → Nested repeatables join the pin): the chain
/// alternates `item, nested-section, item, …`, shared verbatim from the writer via
/// [`write::physical_item_chain`] — no second grammar. Two interpretations, tried
/// in order:
///
/// 1. the **whole** hop chain is an item path — its terminus an **item** (odd chain,
///    rendered complete via [`render_item`]) or a declared **nested section** (even
///    chain — its nested items rendered, self-rooted like a section-level slice);
/// 2. the chain **minus a trailing leaf hop** is an item path — the terminus is that
///    **leaf** on the chain's item: a declared slot's opaque prose byte-for-byte,
///    the item template's `id-from` leaf (the item's heading, its stable id-source
///    — the one schema-role consult on the read path), or a per-item field's
///    rendered canonical value.
///
/// Each hop that names no existing section/item/leaf is an **honest located block**
/// — never a wrong node with exit 0 (the M40 rule): a mistyped nested-section
/// segment is rejected, never treated as an item id, so the segment-less physical
/// shortcut (`#releases/<id>/<gid>`) blocks exactly like the write side.
fn slice_fragment(
    schema: &Schema,
    doc: &Document,
    source: &str,
    fragment: &Fragment,
    address: &str,
) -> Result<String, Finding> {
    let (section_id, rest) = fragment_hops(fragment);

    let Some(section) = doc.sections.iter().find(|s| s.id == section_id) else {
        return Err(block(
            "store.no-such-section",
            format!("`{address}` names no section `{section_id}`"),
            address,
            "name a section that exists in the committed doc".to_string(),
        ));
    };

    let declared = schema.sections.iter().find(|s| s.id == section_id);

    // Section-level (no further hops): a **fields-only** (header) section slices to its
    // field group; a slot section to its prose span; a repeatable section to its rendered
    // items. The fields-only arm is M42: the slice used to fall through to `render_items`
    // over a section that holds none, so a header carrying real fields answered the EMPTY
    // STRING at exit 0 — the wrong-node-exit-0 `doc-read-surface.md` forbids by name.
    if rest.is_empty() {
        if let Some(crate::schema::SectionBody::Simple { slot: None, .. }) =
            declared.map(|s| &s.body)
        {
            return Ok(render_field_group(
                &section.fields,
                declared.is_some_and(|s| s.header),
            ));
        }
        return Ok(match &section.slot {
            Some(span) => span.slice(source).to_string(),
            None => render_items(&section.items, source),
        });
    }

    // A **leaf inside a non-repeatable section** (`#section/<leaf>`) — the grammar's
    // `unit/leaf` depth (`structural-grammar.md` → Addressing: *"one grammar, every
    // reference"*), which the write path (`doc set-field`) and `validate`'s emitted
    // `#<section>/<field>` targets already honour. A simple section holds no items, so
    // its one legal deeper hop is a leaf on its own field group (M42 — the read path
    // stops rejecting an address the tool itself emits).
    let is_simple =
        declared.is_some_and(|s| matches!(s.body, crate::schema::SectionBody::Simple { .. }));
    if is_simple && rest.len() == 1 {
        return resolve_section_leaf(section, rest[0], address);
    }

    // 1. The whole chain as an item path: a valid chain of even length ends on a
    //    nested-section segment (the alternation starts at an item), so it slices to
    //    that item's nested items; an odd chain ends on the item itself. The parsed
    //    children are the one physical nested list (a nested section is a purely
    //    logical schema hop — its items render directly under the parent).
    if let Some(physical) = write::physical_item_chain(schema, section_id, &rest) {
        let item = descend_items(section, &physical, address)?;
        return Ok(if rest.len().is_multiple_of(2) {
            render_items(&item.items, source)
        } else {
            render_item(item, source)
        });
    }

    // 2. The chain minus a trailing leaf hop: the last hop addresses a leaf on the
    //    chain's item, resolved against the item template the chain bottoms out in
    //    (the same schema walk as the writer — `write::chain_repeatable`).
    if rest.len() >= 2
        && let (Some(physical), Some(template)) = (
            write::physical_item_chain(schema, section_id, &rest[..rest.len() - 1]),
            write::chain_repeatable(schema, section_id, &rest[..rest.len() - 1]),
        )
    {
        let item = descend_items(section, &physical, address)?;
        return resolve_leaf(item, template, source, rest[rest.len() - 1], address);
    }

    // Neither interpretation resolves. On a repeatable section the failure is a
    // nested-section-position hop naming no declared nested repeatable at its level
    // (S1: rejected, never treated as an item id) — locate the first offender. A
    // simple section has no items, so any deeper hop is the plain item miss.
    let is_repeatable = schema.sections.iter().any(|s| {
        s.id == section_id && matches!(s.body, crate::schema::SectionBody::Repeatable { .. })
    });
    if is_repeatable {
        let bad = rest
            .iter()
            .enumerate()
            .skip(1)
            .step_by(2)
            .find(|(i, _)| write::physical_item_chain(schema, section_id, &rest[..=*i]).is_none())
            .map_or(rest[rest.len() - 1], |(_, seg)| *seg);
        return Err(block(
            "store.no-such-section",
            format!("`{address}` names no nested section `{bad}` in section `{section_id}`"),
            address,
            "qualify nested items with their declared nested-section id (the section-qualified write address)".to_string(),
        ));
    }
    Err(block(
        "store.no-such-item",
        format!(
            "`{address}` names no item `{}` in section `{section_id}`",
            rest[0]
        ),
        address,
        "name an item that exists in the committed section".to_string(),
    ))
}

/// Split a [`Fragment`] into its section id and the remaining navigation hops, as
/// plain strings — the read-path is purely structural (no schema roles), so it
/// resolves each hop over the parsed data. A [`Fragment::Deep`] always carries ≥4
/// hops (the parser only mints it past three), so its first element is present.
fn fragment_hops(fragment: &Fragment) -> (&str, Vec<&str>) {
    match fragment {
        Fragment::Unit(u) => (u.as_str(), Vec::new()),
        Fragment::UnitLeaf(u, l) => (u.as_str(), vec![l.as_str()]),
        Fragment::UnitItem(u, i) => (u.as_str(), vec![i.as_str()]),
        Fragment::UnitItemLeaf(u, i, l) => (u.as_str(), vec![i.as_str(), l.as_str()]),
        Fragment::Deep(hops) => (
            hops[0].as_str(),
            hops[1..].iter().map(String::as_str).collect(),
        ),
    }
}

/// Descend the parsed `section`'s items by the **physical** item-id chain (each id
/// matched within its parent's items — parent-scoped, the model the write-side byte
/// locator enforces on disk, so a same-anchor item under a different parent is never
/// returned) to the addressed [`ParsedItem`]. An absent id at any level is an honest
/// `store.no-such-item` block naming the missing id and its scope.
fn descend_items<'a>(
    section: &'a parse::ParsedSection,
    physical: &[&str],
    address: &str,
) -> Result<&'a ParsedItem, Finding> {
    let mut items = &section.items;
    let mut scope = format!("section `{}`", section.id);
    let mut found: Option<&ParsedItem> = None;
    for id in physical {
        let Some(item) = items.iter().find(|it| it.id == *id) else {
            return Err(block(
                "store.no-such-item",
                format!("`{address}` names no item `{id}` in {scope}"),
                address,
                "name an item that exists in the committed doc".to_string(),
            ));
        };
        scope = format!("item `{}`", item.id);
        items = &item.items;
        found = Some(item);
    }
    // The chain is non-empty by construction (every caller passes ≥1 hop); the
    // defensive block keeps the read path panic-free regardless.
    found.ok_or_else(|| {
        block(
            "store.no-such-item",
            format!("`{address}` carries an empty item path"),
            address,
            "name an item that exists in the committed section".to_string(),
        )
    })
}

/// Resolve a **leaf** hop on a **non-repeatable** `section`: the value of the field of
/// that id in the section's own field group (the header front-matter or a body
/// section's sentinelled trailing group), rendered canonically — the same value shape
/// the whole-doc `fields` project.
///
/// A simple section's addressable leaves are exactly its fields: its prose slot *is*
/// the section (`#<section>`), and the parser records no sub-labelled section slots. So
/// any other name — including an optional field the committed doc does not carry — is an
/// honest `store.no-such-leaf` block with a route (the item-leaf precedent above: the
/// doc answers for the leaves it holds, never a wrong node at exit 0).
fn resolve_section_leaf(
    section: &parse::ParsedSection,
    leaf: &str,
    address: &str,
) -> Result<String, Finding> {
    section
        .fields
        .iter()
        .find(|f| f.key == leaf)
        .map(|f| f.value.render())
        .ok_or_else(|| {
            block(
                "store.no-such-leaf",
                format!(
                    "`{address}` names no leaf `{leaf}` in section `{}`",
                    section.id
                ),
                address,
                "name a field the committed section carries (a section's prose slot is \
                 the section itself — address it as `#<section>`)"
                    .to_string(),
            )
        })
}

/// Render a **fields-only** section's field group as the writer emits it (M42): the bare
/// `key: value` front-matter form for the **header** section, the
/// `<!-- fields -->`-sentinelled bullet form for a **body** field group — through the
/// writer's own emitters, never a second field format. Empty for a section the committed
/// doc carries no fields for (every field optional and absent).
///
/// This slice is a **canonical re-emit, not a byte slice**: a
/// [`parse::ParsedSection`] carries no field-group span (the parser records the fields
/// themselves, not a byte range over them), so the group is rendered *from the parse* —
/// which is what the plain path promises throughout, the canonical render of the node you
/// addressed (`design/doc-read-surface.md` → the retired byte-exactness claim). Trailing
/// newline trimmed, so the value composes like every other slice.
fn render_field_group(fields: &[crate::field_block::Field], header: bool) -> String {
    if fields.is_empty() {
        return String::new();
    }
    let rendered = if header {
        write::emit_bare_fields(fields)
    } else {
        let mut out = String::new();
        write::append_field_group(&mut out, fields);
        out
    };
    rendered.trim_matches('\n').to_string()
}

/// Resolve the trailing **leaf** hop on the chain's `item`, against the `template`
/// (the [`crate::schema::Repeatable`]) its item chain bottoms out in: a **declared
/// slot**'s opaque prose byte-for-byte, the template's **`id-from`** leaf → the
/// item's heading (its stable id-source — the one schema-role consult on the read
/// path, and it works at every nesting depth now that nested content is pinned), or
/// a per-item **field**'s rendered canonical value.
///
/// An unknown leaf is an honest `store.no-such-leaf` block — when it names a
/// physical nested item, that is the segment-less shortcut, rejected with a route
/// naming the declared nested section(s) (S1: never treated as an item id, matching
/// the write-side semantics).
fn resolve_leaf(
    item: &ParsedItem,
    template: &crate::schema::Repeatable,
    source: &str,
    leaf: &str,
    address: &str,
) -> Result<String, Finding> {
    let declares_slot = template
        .block
        .iter()
        .any(|l| matches!(l, crate::schema::Leaf::Slot { id, .. } if id == leaf));
    if declares_slot && let Some(span) = item.slot_span(leaf) {
        return Ok(span.slice(source).to_string());
    }
    if template.id_from == leaf {
        return Ok(item.title.trim().to_string());
    }
    if let Some(field) = item.fields.iter().find(|f| f.key == leaf) {
        return Ok(field.value.render());
    }
    let nested_ids: Vec<&str> = template
        .block
        .iter()
        .filter_map(|l| match l {
            crate::schema::Leaf::Repeatable { id, .. } => Some(id.as_str()),
            _ => None,
        })
        .collect();
    let route = if !nested_ids.is_empty() && item.items.iter().any(|it| it.id == leaf) {
        format!(
            "`{leaf}` is a nested item — address it through its declared nested section ({})",
            nested_ids.join(", ")
        )
    } else {
        "name a leaf that exists in the committed item".to_string()
    };
    Err(block(
        "store.no-such-leaf",
        format!("`{address}` names no leaf `{leaf}` on item `{}`", item.id),
        address,
        route,
    ))
}

/// Render a list of repeatable items as a Content list for the store-read path:
/// each item via [`render_item`], joined by a blank line, mirroring on-disk order.
/// The composer wraps the whole string as a `> ` Content blockquote, so
/// `{{@task.spec#criteria}}` reads as the titled, prose-carrying list the implementer
/// needs.
fn render_items(items: &[ParsedItem], source: &str) -> String {
    items
        .iter()
        .map(|item| render_item(item, source))
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// The field-group sentinel line (mirrors the parser/writer's private const).
const FIELD_SENTINEL: &str = "<!-- fields -->";

/// Render a single repeatable item — the depth-1 entry point over
/// [`render_item_at`], which carries the depth-aware recursion.
fn render_item(item: &ParsedItem, source: &str) -> String {
    render_item_at(item, source, 1)
}

/// Render one repeatable item at nesting `depth` — the read-path mirror of the
/// writer's `render_item_at` body shape: the `<#…> <title>  {#id}` heading at level
/// `2 + depth` (depth 1 → `###`; the schema loader caps nesting at H6), then — for a
/// multi-slot template — each slot under its `<#…> <Leaf-Title>` sub-heading one level
/// deeper, or — for a single bare-prose slot — the slot prose beneath the heading; then
/// the item's sentinelled per-item **field group** (only when fields are present), then
/// — recursively — its **nested** items one level deeper (the M22 multi-level shape;
/// M40 — nested content joins the plain slice, `design/doc-read-surface.md` → Nested
/// repeatables join the pin).
///
/// The heading carries the item's frozen **`{#id}` anchor**, two spaces before it — the
/// writer's exact canonical form (M42 — `design/doc-read-surface.md` → the retired
/// byte-exactness claim). The render was anchorless by design until M42, which withheld
/// from every plain read (and every compose-time `{{@task.spec#criteria}}` deref) the one
/// value an agent needs in order to *address the item back*: its id, which is not
/// derivable from the heading (a release titled `1.0.0` mints `100`, and a retitle
/// diverges the two permanently by design). The re-rooting of a nested slice is *not*
/// repaired here and stays — lifting a sub-tree out of its document re-heads it — so this
/// is the canonical render of the addressed node, never a byte slice.
fn render_item_at(item: &ParsedItem, source: &str, depth: usize) -> String {
    let hashes = "#".repeat(2 + depth);
    let heading = format!("{hashes} {}  {{#{}}}", item.title.trim(), item.id);
    let mut out = if !item.slots.is_empty() {
        let leaf_hashes = "#".repeat(3 + depth);
        let mut out = heading;
        for (leaf_id, span) in &item.slots {
            out.push_str(&format!(
                "\n\n{leaf_hashes} {}\n\n{}",
                title_case_label(leaf_id),
                span.slice(source).trim()
            ));
        }
        out
    } else {
        match &item.slot {
            Some(span) => {
                format!("{heading}\n\n{}", span.slice(source).trim())
            }
            None => heading,
        }
    };
    if !item.fields.is_empty() {
        out.push_str("\n\n");
        out.push_str(FIELD_SENTINEL);
        for field in &item.fields {
            out.push_str(&format!("\n- {}: {}", field.key, field.value.render()));
        }
    }
    for child in &item.items {
        out.push_str("\n\n");
        out.push_str(&render_item_at(child, source, depth + 1));
    }
    out
}

/// Title-case a single-word slot leaf id for its `#### <Leaf-Title>` sub-heading in
/// the store-read view (mirrors the writer's single-word heading rendering).
fn title_case_label(id: &str) -> String {
    let mut chars = id.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");
    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");
    const CHANGELOG_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/changelog.yaml");

    #[test]
    fn lexical_normalize_collapses_curdir_and_parentdir() {
        // The shared path-collision normalization (review C1): a `./`-prefixed or
        // redundant-`..` spelling of the canonical path collapses to the clean form,
        // so retire-side and create-side guards compare equal regardless of spelling.
        assert_eq!(
            lexical_normalize(Path::new("./changelog/changelog.md")),
            PathBuf::from("changelog/changelog.md"),
        );
        assert_eq!(
            lexical_normalize(Path::new("changelog/../changelog/changelog.md")),
            PathBuf::from("changelog/changelog.md"),
        );
    }

    /// (M38 inc-1 T2) `canonical_path` resolves a **placement** doctype to its literal
    /// `placement.file` repo-root-relative path — case-preserved, bypassing docs-root
    /// and the `<location>/<slug>.md` join — so a root `FOO.md` and a `docs/bar.md` are
    /// each reachable exactly as written (store-readable). The passed slug is ignored:
    /// a placement singleton's home is the literal file, not a slug-derived path.
    /// (`design/storage.md` → Placement — census site `canonical_path`.)
    #[test]
    fn canonical_path_resolves_a_placement_doctype_to_its_literal_file() {
        let root_home = crate::schema::load_schema(
            b"\
type: foo
placement: { file: FOO.md }
sections: []
",
        )
        .expect("root-placement schema loads");
        assert_eq!(
            canonical_path(Path::new("/repo"), &root_home, "foo"),
            Some(PathBuf::from("/repo/FOO.md")),
            "a root placement resolves to the case-preserved literal, bypassing docs-root",
        );

        let under_docs = crate::schema::load_schema(
            b"\
type: bar
placement: { file: docs/bar.md }
sections: []
",
        )
        .expect("docs-placement schema loads");
        assert_eq!(
            canonical_path(Path::new("/repo"), &under_docs, "bar"),
            Some(PathBuf::from("/repo/docs/bar.md")),
            "a direct file under docs/ resolves exactly as written (the docs/ is in the literal)",
        );
    }

    /// A throwaway directory that removes itself on drop — keeps store-read tests
    /// off any real repo tree.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-store-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp root");
            TempRoot(path)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "adr".to_string(),
            crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
                .expect("adr.yaml loads"),
        );
        m.insert(
            "commit".to_string(),
            crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads"),
        );
        m.insert(
            "spec".to_string(),
            crate::schema::load_schema_with_types(
                SPEC_YAML,
                &crate::schema::dev_pack_field_types(),
            )
            .expect("spec.yaml loads"),
        );
        m.insert(
            "changelog".to_string(),
            crate::schema::load_schema(CHANGELOG_YAML).expect("changelog.yaml loads"),
        );
        m
    }

    /// A committed `spec` whose `criteria` is a **repeatable** section (the shipped
    /// shape) — the read target `implement-from-spec`'s `locate-from-spec` step reads
    /// via `{{@task.spec#criteria}}`. Human-editable, conformant bytes.
    const COMMITTED_SPEC: &str = "\
# Gateway rate limiting

## Goal

Bound per-client request volume at the gateway.

## Context

Downstream services were each enforcing limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Recovers after the window  {#recovers}

The next window admits requests again.
";

    /// The canonical committed-ADR fixture flow #5 re-reads
    /// (`decisions/single-node-cache.md`). Human-editable, conformant bytes.
    const COMMITTED_ADR: &str = "\
---
status: accepted
date: 2026-05-23
---

# Single-node session cache

## Context
Session lookups must stay sub-millisecond.

## Options
Alternatives were weighed and rejected.

## Decision
A single in-memory node keeps session lookups sub-millisecond and avoids a
network hop; acceptable because sessions are cheap to reconstruct on a cold node.

## Consequences
A cold node loses its sessions; clients re-authenticate.
";

    /// Write the committed ADR fixture to its canonical path under `repo_root`.
    fn write_committed_adr(repo_root: &Path) {
        let path = repo_root.join("decisions").join("single-node-cache.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk decisions/");
        std::fs::write(&path, COMMITTED_ADR).expect("write committed ADR");
    }

    /// GOLDEN: a committed ADR at `decisions/single-node-cache.md` is resolved from
    /// `adr:single-node-cache#decision`, parsed against the schema, and its
    /// `#decision` section slot sliced to its prose — byte-exact over the committed
    /// file. The snapshot pins the sliced prose bytes.
    #[test]
    fn store_reads_and_slices_a_committed_adr_section() {
        let root = TempRoot::new("read-slice");
        write_committed_adr(root.path());

        let address = Address::parse("adr:single-node-cache#decision").expect("valid address");
        let prose =
            read_slice(root.path(), &schemas(), &address).expect("committed ADR slice resolves");

        insta::assert_snapshot!(prose, @r"
        A single in-memory node keeps session lookups sub-millisecond and avoids a
        network hop; acceptable because sessions are cheap to reconstruct on a cold node.
        ");
    }

    /// GOLDEN: a committed `spec`'s **repeatable** `criteria` section is sliced via
    /// `spec:gateway-rate-limiting#criteria` — the section has no slot, so the slice
    /// renders its items (each `### <title>` + its `statement` slot prose) as the
    /// Content list `{{@task.spec#criteria}}` reads (`worked-examples.md` → flow 6).
    /// This is the read path the shipped repeatable `criteria` needs; the slot-only
    /// path (flow #5's `#decision`) never exercised it.
    #[test]
    fn store_slices_a_committed_spec_repeatable_criteria_section() {
        let root = TempRoot::new("spec-criteria");
        let path = root.path().join("specs").join("gateway-rate-limiting.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk specs/");
        std::fs::write(&path, COMMITTED_SPEC).expect("write committed spec");

        let address = Address::parse("spec:gateway-rate-limiting#criteria").expect("valid address");
        let rendered = read_slice(root.path(), &schemas(), &address)
            .expect("committed spec criteria slice resolves");

        insta::assert_snapshot!(rendered, @r"
        ### Rejects the 101st request  {#rejects-burst}

        The gateway rejects the 101st request in a rolling 60s window.

        ### Recovers after the window  {#recovers}

        The next window admits requests again.
        ");
    }

    /// Read-tolerance: a committed ADR saved with a leading UTF-8 BOM (common from
    /// Windows editors — the file is an editable channel) still reads and slices,
    /// per `parsing.md` → Round-trip guarantees ("BOM — tolerate on read (skip)").
    /// The BOM is stripped before both parse and slice, so the prose stays aligned.
    #[test]
    fn store_tolerates_a_leading_bom_on_a_committed_adr() {
        let root = TempRoot::new("bom");
        let path = root.path().join("decisions").join("single-node-cache.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk decisions/");
        std::fs::write(&path, format!("\u{feff}{COMMITTED_ADR}")).expect("write BOM'd ADR");

        let address = Address::parse("adr:single-node-cache#decision").expect("valid address");
        let prose = read_slice(root.path(), &schemas(), &address)
            .expect("a BOM-prefixed committed ADR still resolves");
        assert!(
            prose.starts_with("A single in-memory node"),
            "sliced prose is byte-aligned despite the BOM: {prose:?}"
        );
    }

    /// A missing target file yields a blocking [`Finding`] with a located message
    /// and a route — never a panic, never a silent empty.
    #[test]
    fn store_read_of_a_missing_file_is_a_blocking_finding() {
        let root = TempRoot::new("missing");
        // No file written.
        let address = Address::parse("adr:does-not-exist#decision").expect("valid address");
        let err = read_slice(root.path(), &schemas(), &address)
            .expect_err("a missing committed file blocks");

        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "store.not-found");
        assert!(
            err.message.contains("adr:does-not-exist"),
            "block names the unresolved address: {err:?}"
        );
        assert!(err.location.is_some(), "the block is located");
        assert!(err.route.is_some(), "the block carries a route");
    }

    /// (M43 inc-5 T2, supersedes the M40 F12 pin) The `store.not-found` route names
    /// the **staged read** for the staged-in-an-open-task case: a doc minted but not
    /// yet finalized has no committed file, and its sanctioned read is now the very
    /// address that missed, task-scoped — `jigc doc show <addr> --task <task-id>`.
    /// The retired `jigc task diff` route showed a *changeset*, never the doc.
    #[test]
    fn store_not_found_route_points_staged_docs_at_the_staged_read() {
        let root = TempRoot::new("missing-route");
        let address = Address::parse("adr:does-not-exist#decision").expect("valid address");
        let err = read_slice(root.path(), &schemas(), &address)
            .expect_err("a missing committed file blocks");

        assert_eq!(err.code, "store.not-found");
        let route = err.route.expect("the block carries a route");
        assert!(
            route.contains("jigc doc show adr:does-not-exist#decision --task <task-id>"),
            "route names the staged read of the missed address: {route}"
        );
        assert!(
            !route.contains("task diff"),
            "the superseded task-diff route is retired: {route}"
        );
    }

    /// (V13) An unknown doctype blocks `store.unknown-type` and its route names the
    /// real discovery verb `jigc describe` — never a nonexistent `doc types`
    /// subcommand (mirrors the `doc create`/`author` remediation precedent).
    #[test]
    fn store_unknown_type_route_names_jigc_describe() {
        let root = TempRoot::new("unknown-type");
        let address = Address::parse("wormhole:whatever#decision").expect("valid address");
        let err =
            read_slice(root.path(), &schemas(), &address).expect_err("an unknown doctype blocks");

        assert_eq!(err.code, "store.unknown-type");
        let route = err.route.expect("the block carries a route");
        assert!(
            route.contains("jigc describe"),
            "route names the real discovery verb: {route}"
        );
        assert!(
            !route.contains(&["jigc doc", "types"].join(" ")),
            "route must not name the nonexistent subcommand: {route}"
        );
    }

    /// (V13 grep-guard) No engine source names the nonexistent `doc types`
    /// subcommand — the two former sites (`store.rs`, `milestone.rs`) route to
    /// `jigc describe` and must stay that way. Walks `crates/engine/src/**/*.rs`.
    #[test]
    fn no_engine_source_names_the_nonexistent_doc_types_subcommand() {
        // Built at runtime so this guard's own body doesn't match itself.
        let needle = ["jigc doc", "types"].join(" ");
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut stack = vec![src];
        let mut offenders = Vec::new();
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read engine src dir") {
                let path = entry.expect("dir entry").path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    let text = std::fs::read_to_string(&path).expect("read rs file");
                    if text.contains(&needle) {
                        offenders.push(path.display().to_string());
                    }
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "engine source still names the nonexistent `{needle}`: {offenders:?}"
        );
    }

    /// (M39 inc-1 T1) A **fragmentless** address reads the whole committed doc
    /// byte-for-byte — the `jigc doc show <ref>` whole-doc read. The parse enforces
    /// conformance first, then the committed source is returned verbatim (the
    /// round-trip guarantee: the file's own bytes are its canonical render).
    #[test]
    fn store_reads_a_whole_committed_doc_with_no_fragment() {
        let root = TempRoot::new("whole-doc");
        write_committed_adr(root.path());

        let address = Address::parse("adr:single-node-cache").expect("valid address");
        let whole =
            read_slice(root.path(), &schemas(), &address).expect("whole committed ADR resolves");
        assert_eq!(
            whole, COMMITTED_ADR,
            "whole-doc read is byte-for-byte the file"
        );

        // A committed spec (repeatable body) reads whole just the same.
        let path = root.path().join("specs").join("gateway-rate-limiting.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk specs/");
        std::fs::write(&path, COMMITTED_SPEC).expect("write committed spec");
        let address = Address::parse("spec:gateway-rate-limiting").expect("valid address");
        let whole = read_slice(root.path(), &schemas(), &address).expect("whole committed spec");
        assert_eq!(
            whole, COMMITTED_SPEC,
            "whole-doc spec read is byte-for-byte"
        );
    }

    /// (M39 inc-1 T1) A **2-hop item slice** over a repeatable section
    /// (`spec:…#criteria/<id>`) renders that single item — its `### <title>` heading
    /// and its `statement` slot prose — the read-path inverse of the writer's item.
    #[test]
    fn store_slices_a_committed_spec_item() {
        let root = TempRoot::new("spec-item");
        let path = root.path().join("specs").join("gateway-rate-limiting.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk specs/");
        std::fs::write(&path, COMMITTED_SPEC).expect("write committed spec");

        let address =
            Address::parse("spec:gateway-rate-limiting#criteria/rejects-burst").expect("valid");
        let item = read_slice(root.path(), &schemas(), &address).expect("item slice resolves");
        insta::assert_snapshot!(item, @r"
        ### Rejects the 101st request  {#rejects-burst}

        The gateway rejects the 101st request in a rolling 60s window.
        ");
    }

    /// (M42 inc-8 T4) The plain slice renders each item heading in the writer's **exact
    /// canonical form** — `<#…> <title>  {#id}`, two spaces before the frozen anchor —
    /// so the identity an agent needs in order to address the item back survives the
    /// read. Before M42 `render_item_at` was anchorless by design, and the id was
    /// reachable only from the raw markdown the adapter rule forbids reading
    /// (`design/doc-read-surface.md` → the retired byte-exactness claim: "M42 restores
    /// the `{#id}` anchor on rendered item headings"). The anchor rides at **every**
    /// depth: a top-level criteria item and a nested changelog change-group alike.
    #[test]
    fn store_item_slices_carry_the_frozen_id_anchor() {
        let root = TempRoot::new("item-anchor");
        let spec = root.path().join("specs").join("gateway-rate-limiting.md");
        std::fs::create_dir_all(spec.parent().unwrap()).expect("mk specs/");
        std::fs::write(&spec, COMMITTED_SPEC).expect("write committed spec");
        write_committed_changelog(root.path());

        let item = Address::parse("spec:gateway-rate-limiting#criteria/rejects-burst").expect("ok");
        assert_eq!(
            read_slice(root.path(), &schemas(), &item).expect("item slice resolves"),
            "### Rejects the 101st request  {#rejects-burst}\n\nThe gateway rejects the 101st \
             request in a rolling 60s window.",
            "the item slice heads with the writer's canonical `<title>  {{#id}}` form",
        );

        let release = Address::parse("changelog:changelog#releases/1-0-0").expect("ok");
        let rendered =
            read_slice(root.path(), &schemas(), &release).expect("release slice resolves");
        assert!(
            rendered.starts_with("### 1.0.0  {#1-0-0}\n"),
            "the release item heads with its frozen anchor, got:\n{rendered}",
        );
        assert!(
            rendered.contains("#### Added  {#added}\n"),
            "the nested change-group carries its anchor one level deeper, got:\n{rendered}",
        );
    }

    /// (M39 inc-1 T1) A **3-hop leaf slice** (`spec:…#criteria/<id>/statement`) returns
    /// the addressed leaf's prose **byte-for-byte** over its opaque span — no heading,
    /// no rendering, just the committed slot bytes.
    #[test]
    fn store_slices_a_committed_spec_item_leaf_byte_for_byte() {
        let root = TempRoot::new("spec-item-leaf");
        let path = root.path().join("specs").join("gateway-rate-limiting.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk specs/");
        std::fs::write(&path, COMMITTED_SPEC).expect("write committed spec");

        let address = Address::parse("spec:gateway-rate-limiting#criteria/rejects-burst/statement")
            .expect("valid");
        let leaf = read_slice(root.path(), &schemas(), &address).expect("leaf slice resolves");
        assert_eq!(
            leaf, "The gateway rejects the 101st request in a rolling 60s window.",
            "the leaf slice is the committed statement prose, byte-for-byte",
        );
    }

    /// (M39 inc-1 T1) A fragment naming a **missing section** still blocks with
    /// `store.no-such-section` + a route; a missing **item** blocks with
    /// `store.no-such-item` — every miss is an honest located block (M40 tightened
    /// the leaf hop to the declared names only; see
    /// `store_nested_read_misses_block_honestly`).
    #[test]
    fn store_read_misses_block_with_a_route() {
        let root = TempRoot::new("misses");
        let path = root.path().join("specs").join("gateway-rate-limiting.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk specs/");
        std::fs::write(&path, COMMITTED_SPEC).expect("write committed spec");
        let schemas = schemas();

        let no_section = Address::parse("spec:gateway-rate-limiting#nope").expect("valid");
        let err =
            read_slice(root.path(), &schemas, &no_section).expect_err("missing section blocks");
        assert_eq!(err.code, "store.no-such-section");
        assert!(
            err.route.is_some(),
            "the no-such-section block carries a route"
        );

        let no_item =
            Address::parse("spec:gateway-rate-limiting#criteria/no-such-item").expect("valid");
        let err = read_slice(root.path(), &schemas, &no_item).expect_err("missing item blocks");
        assert_eq!(err.code, "store.no-such-item");
        assert!(
            err.route.is_some(),
            "the no-such-item block carries a route"
        );
    }

    /// (M42 inc-8 T1) A **leaf inside a non-repeatable section** — the grammar's
    /// `unit/leaf` depth (`structural-grammar.md` → Addressing: *"one grammar, every
    /// reference"*), which the write path and `validate`'s emitted targets both honour
    /// and the read path did not: `adr:…#status/status` blocked `store.no-such-item`
    /// while `doc set-field` on the same string exited 0. The header's field leaves now
    /// resolve to their canonical rendered values, and an absent leaf name is an honest
    /// `store.no-such-leaf` block with a route — never a wrong node at exit 0.
    #[test]
    fn store_slices_a_field_leaf_in_a_simple_section() {
        let root = TempRoot::new("simple-leaf");
        write_committed_adr(root.path());
        let schemas = schemas();

        for (leaf, want) in [("status", "accepted"), ("date", "2026-05-23")] {
            let address =
                Address::parse(&format!("adr:single-node-cache#status/{leaf}")).expect("valid");
            let value = read_slice(root.path(), &schemas, &address)
                .unwrap_or_else(|err| panic!("`#status/{leaf}` resolves; got {err:?}"));
            assert_eq!(value, want, "`#status/{leaf}` slices to its field value");
        }

        let bad = Address::parse("adr:single-node-cache#status/nope").expect("valid");
        let err = read_slice(root.path(), &schemas, &bad)
            .expect_err("an absent leaf name blocks honestly");
        assert_eq!(err.code, "store.no-such-leaf");
        assert!(
            err.message.contains("nope"),
            "the block names the absent leaf: {err:?}"
        );
        assert!(err.route.is_some(), "the block carries a route");
    }

    /// A doctype with **both** fields-only shapes: a `meta` **header** (the front-matter
    /// group, the shape 11 sections across 10 shipped doctypes carry) and a `pins`
    /// **body** field group (the `<!-- fields -->`-sentinelled bullet form) — plus a slot
    /// section, the context that must stay untouched.
    const FIELDS_ONLY_YAML: &[u8] = b"\
type: board
location: boards/
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: owner, type: string }
      - { id: created, type: date }
  - id: pins
    fields:
      - { id: base, type: string }
  - id: notes
    slot: { hint: \"Anything else.\" }
";

    /// The committed `board` — the canonical bytes its writer mints.
    const COMMITTED_BOARD: &str = "\
---
owner: maurice
created: 2026-07-13
---

# Pin board

## Pins

<!-- fields -->
- base: 2f0c1d9

## Notes

Nothing pinned yet.
";

    /// (M42 inc-8 T2) A **fields-only (header) section** slice serves its **fields**.
    /// Before: `adr:…#status` returned the **empty string at exit 0** — the
    /// wrong-node-exit-0 `doc-read-surface.md` forbids by name, over a header that
    /// genuinely carries `status`/`date`. The slice is a **canonical re-emit** (a
    /// `ParsedSection` carries no field-group span): the writer's own emitters render
    /// the group — the bare front-matter form for a header, the sentinelled bullet form
    /// for a body field group — never a second field format.
    #[test]
    fn store_slices_a_fields_only_section_to_its_field_group() {
        let root = TempRoot::new("fields-only");
        let path = root.path().join("boards").join("pin-board.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk boards/");
        std::fs::write(&path, COMMITTED_BOARD).expect("write committed board");
        let mut board = BTreeMap::new();
        board.insert(
            "board".to_string(),
            crate::schema::load_schema(FIELDS_ONLY_YAML).expect("board.yaml loads"),
        );

        // The header group → the bare `key: value` front-matter form.
        let header = Address::parse("board:pin-board#meta").expect("valid");
        assert_eq!(
            read_slice(root.path(), &board, &header).expect("`#meta` resolves"),
            "owner: maurice\ncreated: 2026-07-13",
            "a header slice serves its field lines as the writer renders them",
        );

        // A body field group → the `<!-- fields -->`-sentinelled bullet form.
        let body = Address::parse("board:pin-board#pins").expect("valid");
        assert_eq!(
            read_slice(root.path(), &board, &body).expect("`#pins` resolves"),
            "<!-- fields -->\n- base: 2f0c1d9",
            "a body field group slices to the sentinelled bullet form",
        );

        // The omitting context: a slot section still slices to its prose, unchanged.
        let slot = Address::parse("board:pin-board#notes").expect("valid");
        assert_eq!(
            read_slice(root.path(), &board, &slot).expect("`#notes` resolves"),
            "Nothing pinned yet.",
            "a slot section is untouched by the fields-only branch",
        );

        // And the shipped header the wave names: the adr's `status` section.
        write_committed_adr(root.path());
        let adr = Address::parse("adr:single-node-cache#status").expect("valid");
        assert_eq!(
            read_slice(root.path(), &schemas(), &adr).expect("`#status` resolves"),
            "status: accepted\ndate: 2026-05-23",
            "the adr header slice serves its fields, never the empty string",
        );
    }

    /// A committed changelog (the shipped nested-repeatable doctype) at its literal
    /// placement home — releases carry a per-item `date` field group and nested
    /// `#### <group>` change-group items, the exact on-disk shape the writer mints.
    const COMMITTED_CHANGELOG: &str = "\
# Changelog

## Unreleased Changes

## Releases

### 1.1.0  {#1-1-0}

<!-- fields -->
- date: 2026-07-01

#### Added  {#added}

- OAuth device-code flow

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14

#### Added  {#added}

- initial release

#### Fixed  {#fixed}

- session fixation on logout
";

    /// Write the committed changelog fixture to its placement home (`CHANGELOG.md`).
    fn write_committed_changelog(repo_root: &Path) {
        std::fs::write(repo_root.join("CHANGELOG.md"), COMMITTED_CHANGELOG)
            .expect("write committed changelog");
    }

    /// GOLDEN (M40 inc-6 T1): a committed changelog's `#releases` **section slice**
    /// carries each release's `date` field group plus its nested `#### <group>`
    /// headings and notes — the plain slice mirrors the writer's item body shape, down
    /// to the frozen `{#id}` anchor on every heading (M42 inc-8 T4)
    /// ([doc-read-surface.md](../../../design/doc-read-surface.md) → Nested repeatables
    /// join the pin). Before M40 the fields + nested content were silently dropped (only
    /// the whole-doc plain read was complete).
    #[test]
    fn store_slices_a_changelog_releases_section_with_fields_and_nested_groups() {
        let root = TempRoot::new("changelog-releases");
        write_committed_changelog(root.path());

        let address = Address::parse("changelog:changelog#releases").expect("valid address");
        let rendered = read_slice(root.path(), &schemas(), &address)
            .expect("committed changelog releases slice resolves");

        insta::assert_snapshot!(rendered, @r"
        ### 1.1.0  {#1-1-0}

        <!-- fields -->
        - date: 2026-07-01

        #### Added  {#added}

        - OAuth device-code flow

        ### 1.0.0  {#1-0-0}

        <!-- fields -->
        - date: 2026-06-14

        #### Added  {#added}

        - initial release

        #### Fixed  {#fixed}

        - session fixation on logout
        ");
    }

    /// GOLDEN (M40 inc-6 T1): a single **release-item slice** (`#releases/<id>`)
    /// carries the item's `date` field group and its nested change-groups one heading
    /// level deeper — complete, not just the bare `### <title>` heading.
    #[test]
    fn store_slices_a_changelog_release_item_with_fields_and_nested_groups() {
        let root = TempRoot::new("changelog-release-item");
        write_committed_changelog(root.path());

        let address = Address::parse("changelog:changelog#releases/1-0-0").expect("valid address");
        let rendered = read_slice(root.path(), &schemas(), &address)
            .expect("committed changelog release-item slice resolves");

        insta::assert_snapshot!(rendered, @r"
        ### 1.0.0  {#1-0-0}

        <!-- fields -->
        - date: 2026-06-14

        #### Added  {#added}

        - initial release

        #### Fixed  {#fixed}

        - session fixation on logout
        ");
    }

    /// GOLDEN (M40 inc-6 T2): the **section-qualified write address** resolves on read
    /// — `#releases/<id>/changes` (the canonical nested address, review finding S1)
    /// slices to the release's nested change-group items rendered, self-rooted at
    /// `###` like every section-level slice. Before M40 this canonical address falsely
    /// blocked (`store.no-such-leaf`) because the read path only spoke the physical
    /// shortcut ([doc-read-surface.md](../../../design/doc-read-surface.md) → Nested
    /// repeatables join the pin).
    #[test]
    fn store_slices_a_changelog_nested_section_via_the_write_address() {
        let root = TempRoot::new("changelog-nested-section");
        write_committed_changelog(root.path());

        let address =
            Address::parse("changelog:changelog#releases/1-0-0/changes").expect("valid address");
        let rendered = read_slice(root.path(), &schemas(), &address)
            .expect("the canonical nested-section address resolves on read");

        insta::assert_snapshot!(rendered, @r"
        ### Added  {#added}

        - initial release

        ### Fixed  {#fixed}

        - session fixation on logout
        ");
    }

    /// GOLDEN (M40 inc-6 T2): a nested **item** via the write grammar —
    /// `#releases/<id>/changes/<gid>` — renders that change-group complete. Before
    /// M40 the `changes` hop falsely blocked `store.no-such-item` (the false-block
    /// defect the M40 pin kills).
    #[test]
    fn store_slices_a_changelog_nested_item_via_the_write_address() {
        let root = TempRoot::new("changelog-nested-item");
        write_committed_changelog(root.path());

        let address = Address::parse("changelog:changelog#releases/1-0-0/changes/added")
            .expect("valid address");
        let rendered = read_slice(root.path(), &schemas(), &address)
            .expect("the canonical nested-item address resolves on read");

        insta::assert_snapshot!(rendered, @r"
        ### Added  {#added}

        - initial release
        ");
    }

    /// (M40 inc-6 T2) Nested **leaf** hops via the write grammar: the change-group's
    /// slot bytes byte-for-byte (`…/changes/<gid>/notes`), the nested block's own
    /// `id-from` leaf → the item's heading (`…/changes/<gid>/category`), and a
    /// release-level field value through the same grammar (`…/<id>/date`).
    #[test]
    fn store_slices_changelog_nested_leaves_via_the_write_address() {
        let root = TempRoot::new("changelog-nested-leaf");
        write_committed_changelog(root.path());
        let schemas = schemas();

        let notes = Address::parse("changelog:changelog#releases/1-0-0/changes/added/notes")
            .expect("valid address");
        assert_eq!(
            read_slice(root.path(), &schemas, &notes).expect("nested slot leaf resolves"),
            "- initial release",
            "the nested slot slice is the committed bytes, byte-for-byte",
        );

        let category = Address::parse("changelog:changelog#releases/1-0-0/changes/fixed/category")
            .expect("valid address");
        assert_eq!(
            read_slice(root.path(), &schemas, &category).expect("nested id-from leaf resolves"),
            "Fixed",
            "the nested block's own id-from leaf resolves to the item's heading",
        );

        let date =
            Address::parse("changelog:changelog#releases/1-1-0/date").expect("valid address");
        assert_eq!(
            read_slice(root.path(), &schemas, &date).expect("release field leaf resolves"),
            "2026-07-01",
            "a per-item field resolves to its rendered canonical value",
        );
    }

    /// The canonical staged transient `commit` doc (`docs/commit:<task-id>.md`) — the
    /// writer's exact byte form (the `write.rs` commit golden): front matter (`type`),
    /// the H1, the `## Summary`/`## Body` slots, the empty `## Trailers` repeatable.
    const STAGED_COMMIT: &str = "\
---
type: feat
---

# Add gateway rate limiting

## Summary

Add a per-client rate limit at the gateway.

## Body

Centralize limiting at the gateway.

## Trailers
";

    /// (M43 inc-5 T1) The **staged arm**: [`read_slice_staged`] serves a task's staged
    /// working copy at `<task_dir>/docs/<type>:<slug>.md` — whole-doc, byte-for-byte —
    /// while the committed arm still misses the same address (the proof the serve came
    /// from the staged source, never a committed sibling).
    #[test]
    fn staged_read_serves_a_whole_staged_doc_byte_for_byte() {
        let repo = TempRoot::new("staged-whole-repo");
        let task = TempRoot::new("staged-whole-task");
        let staged = crate::state::instance_path(task.path(), "adr", "single-node-cache");
        std::fs::create_dir_all(staged.parent().unwrap()).expect("mk task docs/");
        std::fs::write(&staged, COMMITTED_ADR).expect("stage the adr");

        let address = Address::parse("adr:single-node-cache").expect("valid address");
        let whole = read_slice_staged(repo.path(), task.path(), &schemas(), &address)
            .expect("the staged whole-doc read serves");
        assert_eq!(
            whole, COMMITTED_ADR,
            "the staged read is byte-for-byte the staged copy"
        );

        let committed_miss = read_slice(repo.path(), &schemas(), &address)
            .expect_err("no committed sibling exists — the staged arm alone served");
        assert_eq!(committed_miss.code, "store.not-found");
    }

    /// (M43 inc-5 T1) Every slice depth serves through the **identical
    /// parse/slice/render path**: over the same bytes — committed in the repo root,
    /// staged in a task's working area — the `#section`, `#section/item`, and
    /// `#section/item/leaf` slices are byte-identical across the two arms (source
    /// selection is the only difference between them).
    #[test]
    fn staged_slices_are_byte_identical_to_committed_slices_over_the_same_bytes() {
        let repo = TempRoot::new("staged-slice-repo");
        let task = TempRoot::new("staged-slice-task");
        write_committed_adr(repo.path());
        let spec = repo.path().join("specs").join("gateway-rate-limiting.md");
        std::fs::create_dir_all(spec.parent().unwrap()).expect("mk specs/");
        std::fs::write(&spec, COMMITTED_SPEC).expect("write committed spec");
        for (ty, slug, bytes) in [
            ("adr", "single-node-cache", COMMITTED_ADR),
            ("spec", "gateway-rate-limiting", COMMITTED_SPEC),
        ] {
            let staged = crate::state::instance_path(task.path(), ty, slug);
            std::fs::create_dir_all(staged.parent().unwrap()).expect("mk task docs/");
            std::fs::write(&staged, bytes).expect("stage the doc");
        }

        let schemas = schemas();
        for addr in [
            "adr:single-node-cache#decision",
            "spec:gateway-rate-limiting#criteria",
            "spec:gateway-rate-limiting#criteria/rejects-burst",
            "spec:gateway-rate-limiting#criteria/rejects-burst/statement",
        ] {
            let address = Address::parse(addr).expect("valid address");
            let committed = read_slice(repo.path(), &schemas, &address)
                .unwrap_or_else(|err| panic!("committed `{addr}` resolves; got {err:?}"));
            let staged = read_slice_staged(repo.path(), task.path(), &schemas, &address)
                .unwrap_or_else(|err| panic!("staged `{addr}` resolves; got {err:?}"));
            assert_eq!(
                staged, committed,
                "`{addr}` slices byte-identically through both arms"
            );
        }

        // One depth pinned directly, so both arms can't be broken in unison.
        let leaf = Address::parse("spec:gateway-rate-limiting#criteria/rejects-burst/statement")
            .expect("valid address");
        assert_eq!(
            read_slice_staged(repo.path(), task.path(), &schemas, &leaf)
                .expect("staged leaf resolves"),
            "The gateway rejects the 101st request in a rolling 60s window.",
        );
    }

    /// (M43 inc-5 T1) A **transient** doctype is legal on the staged arm — no
    /// `canonical_path` gate: `commit:<task-id>` staged at `docs/commit:<id>.md`
    /// serves whole and at slice depth (the B9 staging-key leak's sanctioned read
    /// address, closed from the read side) — while the committed arm keeps its
    /// transient gate untouched.
    #[test]
    fn staged_read_serves_a_transient_commit_doc() {
        let repo = TempRoot::new("staged-transient-repo");
        let task = TempRoot::new("staged-transient-task");
        let staged =
            crate::state::instance_path(task.path(), "commit", "add-gateway-rate-limiting");
        std::fs::create_dir_all(staged.parent().unwrap()).expect("mk task docs/");
        std::fs::write(&staged, STAGED_COMMIT).expect("stage the commit doc");
        let schemas = schemas();

        let whole = Address::parse("commit:add-gateway-rate-limiting").expect("valid address");
        assert_eq!(
            read_slice_staged(repo.path(), task.path(), &schemas, &whole)
                .expect("a staged transient commit doc serves"),
            STAGED_COMMIT,
            "the staged transient read is byte-for-byte the staged copy"
        );

        let summary =
            Address::parse("commit:add-gateway-rate-limiting#summary").expect("valid address");
        assert_eq!(
            read_slice_staged(repo.path(), task.path(), &schemas, &summary)
                .expect("a staged transient slice serves"),
            "Add a per-client rate limit at the gateway.",
        );

        let gate = read_slice(repo.path(), &schemas, &whole)
            .expect_err("the committed arm keeps the transient gate");
        assert_eq!(gate.code, "store.transient-type");
    }

    /// (M43 inc-5 T1) An **absent staged instance** blocks `store.not-staged`, routed
    /// on the **real state**: a committed sibling exists → the route names the
    /// task-less read (`jigc doc show <addr>`); nothing exists anywhere → nothing to
    /// read yet. Blocking ⇒ route present (the Inc-1 floor).
    #[test]
    fn staged_read_of_an_absent_instance_blocks_on_the_real_state() {
        let repo = TempRoot::new("not-staged-repo");
        let task = TempRoot::new("not-staged-task");
        write_committed_adr(repo.path());
        let schemas = schemas();

        // A committed sibling exists → read it task-less.
        let addr = Address::parse("adr:single-node-cache").expect("valid address");
        let err = read_slice_staged(repo.path(), task.path(), &schemas, &addr)
            .expect_err("an absent staged instance blocks");
        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "store.not-staged");
        assert!(err.location.is_some(), "the block is located");
        let route = err.route.as_deref().expect("the block carries a route");
        assert!(
            route.contains("jigc doc show adr:single-node-cache"),
            "the route names the task-less read of the committed copy: {route}"
        );
        assert!(
            !route.contains("--task"),
            "the recovery is the task-less read: {route}"
        );

        // Nothing exists anywhere → nothing to read yet.
        let ghost = Address::parse("adr:ghost").expect("valid address");
        let err = read_slice_staged(repo.path(), task.path(), &schemas, &ghost)
            .expect_err("an absent staged instance with no committed sibling blocks");
        assert_eq!(err.code, "store.not-staged");
        assert!(
            err.message.contains("nothing to read yet"),
            "the block states the real state: {err:?}"
        );
        let route = err
            .route
            .as_deref()
            .expect("blocking ⇒ route (the Inc-1 floor)");
        assert!(
            !route.contains("doc show"),
            "no committed copy exists to point at: {route}"
        );
    }

    /// (M43 inc-5 T1) The **singleton-slug guard stays** on the staged arm: a
    /// placement doctype answers only its canonical `<type>:<type>` address, staged
    /// or committed — any other slug blocks rather than serving the singleton's
    /// content for an invalid reference.
    #[test]
    fn staged_read_keeps_the_singleton_slug_guard() {
        let repo = TempRoot::new("staged-singleton-repo");
        let task = TempRoot::new("staged-singleton-task");
        let addr = Address::parse("changelog:wrong-slug").expect("valid address");
        let err = read_slice_staged(repo.path(), task.path(), &schemas(), &addr)
            .expect_err("a non-canonical singleton slug blocks on the staged arm too");
        assert_eq!(err.code, "store.not-found");
        assert!(
            err.message.contains("singleton"),
            "the block names the singleton rule: {err:?}"
        );
    }

    /// (M40 inc-6 T2) The nested read grammar's **honest blocks** — never a wrong
    /// node with exit 0: a mistyped nested-section hop blocks `store.no-such-section`
    /// (S1: never treated as an item id), an absent change-group id blocks
    /// `store.no-such-item`, and the segment-less **physical shortcut**
    /// (`#releases/<id>/<gid>`) blocks `store.no-such-leaf` with a route naming the
    /// declared nested section — matching the write-side rejection semantics.
    #[test]
    fn store_nested_read_misses_block_honestly() {
        let root = TempRoot::new("changelog-nested-misses");
        write_committed_changelog(root.path());
        let schemas = schemas();

        let bad_hop =
            Address::parse("changelog:changelog#releases/1-0-0/typo/added").expect("valid address");
        let err = read_slice(root.path(), &schemas, &bad_hop)
            .expect_err("a bad nested-section hop blocks");
        assert_eq!(err.code, "store.no-such-section");
        assert!(
            err.message.contains("typo"),
            "the block names the offending hop: {err:?}"
        );
        assert!(err.location.is_some(), "the block is located");
        assert!(err.route.is_some(), "the block carries a route");

        let absent_gid = Address::parse("changelog:changelog#releases/1-0-0/changes/removed")
            .expect("valid address");
        let err =
            read_slice(root.path(), &schemas, &absent_gid).expect_err("an absent group id blocks");
        assert_eq!(err.code, "store.no-such-item");
        assert!(
            err.message.contains("removed"),
            "the block names the absent item: {err:?}"
        );
        assert!(err.route.is_some(), "the block carries a route");

        let shortcut =
            Address::parse("changelog:changelog#releases/1-0-0/added").expect("valid address");
        let err = read_slice(root.path(), &schemas, &shortcut)
            .expect_err("the segment-less physical shortcut blocks, never a wrong node");
        assert_eq!(err.code, "store.no-such-leaf");
        assert!(
            err.route.as_deref().is_some_and(|r| r.contains("changes")),
            "the shortcut's route names the declared nested section: {err:?}"
        );
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use crate::write::{self, Instance, SectionContent};
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert("adr".to_string(), adr_schema());
        m
    }

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-store-prop-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp root");
            TempRoot(path)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// Heading-free slot prose: arbitrary paragraphs and `####`-deep headings, but
    /// never a `##`/`###` ATX heading or a Setext underline (the ceiling), and never
    /// a `<!-- fields -->` sentinel. Mirrors the parse-side slot strategy so the
    /// generated ADR is conformant.
    fn slot_prose_strategy() -> impl Strategy<Value = String> {
        let line = prop_oneof![
            "[a-zA-Z][a-zA-Z0-9 .,]{0,40}",
            "#### [a-zA-Z][a-zA-Z0-9 ]{0,20}",
        ];
        prop::collection::vec(line, 1..5).prop_map(|lines| lines.join("\n\n"))
    }

    proptest! {
        /// PARSE→SLICE round-trip: for any conformant ADR built via [`write::render`],
        /// a store-read of each slot fragment slices **byte-for-byte** to that slot's
        /// source prose. The committed bytes are written to the canonical path under a
        /// repo root, then `read_slice` re-reads and re-parses them over the recorded
        /// span — the round-trip on a committed, human-editable file
        /// ([parsing.md](../../../implementation/parsing.md) → Round-trip guarantees).
        #[test]
        fn read_slice_round_trips_every_slot_byte_for_byte(
            context in slot_prose_strategy(),
            decision in slot_prose_strategy(),
            consequences in slot_prose_strategy(),
        ) {
            let schema = adr_schema();
            let instance = Instance {
                title: "Single-node session cache".to_string(),
                sections: vec![
                    SectionContent { id: "status".to_string(), ..Default::default() },
                    SectionContent {
                        id: "context".to_string(),
                        slot: Some(context.clone()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "options".to_string(),
                        slot: Some("Alternatives were weighed and rejected.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "decision".to_string(),
                        slot: Some(decision.clone()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "consequences".to_string(),
                        slot: Some(consequences.clone()),
                        ..Default::default()
                    },
                ],
            };
            let bytes = write::render(&schema, &instance);

            // Commit it at its canonical path.
            let root = TempRoot::new();
            let path = root.path().join("decisions").join("single-node-cache.md");
            std::fs::create_dir_all(path.parent().unwrap()).expect("mk decisions/");
            std::fs::write(&path, &bytes).expect("write committed ADR");

            let schemas = schemas();
            for (section, expected) in [
                ("context", &context),
                ("decision", &decision),
                ("consequences", &consequences),
            ] {
                let address = Address::parse(&format!("adr:single-node-cache#{section}"))
                    .expect("valid address");
                let got = read_slice(root.path(), &schemas, &address)
                    .expect("conformant ADR slice resolves");
                // The writer `trim_end`s prose and the parser trims the span, so the
                // recorded slot bytes are the trimmed prose; the canonical render
                // places it contiguously, so trimmed == the prose itself.
                prop_assert_eq!(got, expected.trim());
            }
        }
    }
}
