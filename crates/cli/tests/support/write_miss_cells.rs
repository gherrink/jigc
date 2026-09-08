//! **The write-verb × miss-shape axis** — the cell set two suites now read, lifted here so
//! there is exactly one of it.
//!
//! The rows below are M50 Increment 9's subject: every `doc` write verb's answer to an
//! address that names something the doc does not have. They lived in
//! `crates/cli/tests/write_miss_shape_axis.rs` until M50 Increment 13, when flow 51's
//! composite acceptance needed the same set — and a flow suite compiles into `g_flow`
//! while the axis suite compiles into `g_finalize`, so a cross-group `use` does not exist.
//! The choice at that point is a **copy** or a **lift**, and a copy of an axis is the
//! failure the complete-fix contract is about: two lists that agree today and disagree the
//! first time a row is added to one of them.
//!
//! This is [`super::shape_space`]'s precedent applied a second time (M49 Increment 1 lifted
//! the item-region shape generator here for exactly the same reason). The axis is one axis;
//! its consumers are two suites at two altitudes — the per-verb mechanism
//! (`write_miss_shape_axis.rs`: the code each row earns, the route each row hands back run
//! verbatim, and the address-shape × declaredness cross that totals against these rows) and
//! the wave's done picture (`flow51_acceptance.rs`: the whole set driven over one corpus,
//! with the corpus asserted untouched afterwards).
//!
//! **What a row is.** The `args` carry `{addr}` where the doc address goes, so one row
//! serves any *corpus of the right shape*; `code` is the finding the reject must carry;
//! `route` says how the emitted recovery is adjudicated.
//!
//! **The schema came with the rows, because a row addresses a shape.** A cell naming
//! `#releases/1-3-0/changes/no-such-group` is a cell *of a corpus shape*, so
//! [`CHANGELOG_SCHEMA`] and [`live_item`] are the other half of the same artifact and live
//! here too. What each consuming suite keeps is the way *it* builds a corpus out of that
//! shape — `write_miss_shape_axis.rs` a hand-rolled pack and repo, flow 51 the shared
//! [`super::trial_corpus`] substrate — which is what keeps two arms over one axis from
//! being one arm run twice.

/// The fixture `changelog` schema — a two-level repeatable (so a nested `add-item`
/// under an absent parent has a home), carrying a `summary` slot and an **optional**
/// `link` field (the only unset-eligible shape: a required or defaulted field is
/// refused by the eligibility guard before the item is ever adjudicated), plus a
/// **non-repeatable** `overview` section so the genuine shape question has a target.
///
/// **The id-from topology mirrors the shipped dev pack's changelog, deliberately** (M47
/// Increment 6, T3 — the fixture-topology mask): the two CLI-side, schema-only
/// pre-checks that outranked item presence (`write.id-from-field` /
/// `write.identity-change`) are reached only through an id-from leaf, and the
/// **`write.identity-change`** arm only through an **enum** id-from. A fixture that
/// declares its change-group `category` as a plain `string` — as this one first did —
/// sidesteps that arm entirely and greens a cell the shipped pack false-fails. So both
/// `category` blocks are enums (`crates/cli/pack/schemas/changelog.yaml`), and a
/// single-level `staged` repeatable mirrors the pack's `unreleased-changes` so the
/// **top-level** enum id-from is on the axis too, not only the nested one.
///
/// It declares a **`location:`** for the same class of reason (M48): the title-miss rows
/// route at `jigc doc rename`, which refuses a doctype with no committed home as a
/// *transient sink* — so a home-less fixture would emit a route it cannot follow, and the
/// new column would green on a dead end. The nine shipped slug-identity doctypes those
/// rows model all declare one.
///
/// **The slotless `meta` section is the M50 T2 ingredient**, and it is a **header**
/// section because that is the shape the corpus actually has: every fields-only section
/// in both shipped packs (`adr` `status`, `spec`/`commit`/`arch-doc` headers) declares
/// `header: true`. `set-slot` at a declared section that hosts no prose slot is a cell of
/// the address-shape column with no ingredient anywhere else in this fixture —
/// `overview` declares a slot, `staged` / `releases` are repeatable — so a fixture
/// without it would leave that cell driven nowhere, which is how a column ships over part
/// of itself.
pub const CHANGELOG_SCHEMA: &str = "\
type: changelog
location: changelogs/
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: owner, type: string, optional: true }
  - id: overview
    slot: { hint: \"What this changelog covers.\", optional: true }
  - id: staged
    repeatable:
      id-from: category
      block:
        - { id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }
        - { id: notes, slot: { hint: \"One bullet per staged change.\" } }
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: link, type: string, optional: true }
        - { id: summary, slot: { hint: \"One-line release summary.\" } }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
";

/// The item a consuming suite mints in each showable section — what the emitted route, run
/// verbatim, must reveal. A property of the fixture (not of the cell), so a new row only
/// declares *which* section its route must strip to.
pub fn live_item(section: &str) -> &'static str {
    match section {
        "releases" => "1-3-0",
        "staged" => "Changed",
        other => panic!("no live item provisioned in section `{other}`"),
    }
}

/// One cell of the write-verb × miss-shape matrix.
pub struct Cell {
    /// What the cell is, for the assertion messages.
    pub what: &'static str,
    /// The `jigc` argv after the binary, with `{addr}` standing for the doc address.
    pub args: &'static [&'static str],
    /// Optional stdin payload (the `--from-file -` cells).
    pub stdin: Option<&'static [u8]>,
    /// The finding `code` the reject must carry.
    pub code: &'static str,
    /// How the emitted route is adjudicated — see [`RouteCheck`].
    pub route: RouteCheck,
}

/// What a cell's emitted route must be, and how it is proven followable.
pub enum RouteCheck {
    /// An **item-id miss**: the top showable section the route must show
    /// (`jigc doc show <type>:<slug>#<section> --task <id>`), run verbatim on the shared
    /// fixture — a read moves nothing.
    Show(&'static str),
    /// A **shape / declaredness** question, which the schema answers: `jigc doc schema
    /// <doctype>`, run verbatim on the shared fixture.
    Schema,
    /// A route that is itself a **write** (`{addr}` substituted): proven followable on a
    /// **private** fixture in the same state, because running it on the shared one would
    /// move the very bytes the next row diffs against.
    MutatingWrite(&'static str),
    /// A **collision** (M49): the minted item id is already taken, so the recovery is the
    /// same mint under a distinct `--slug`. Proven on a **private** fixture in the same
    /// state, and proven to *land* — a route that exits 0 without minting would pass a
    /// bare followability check while leaving the agent exactly where it was.
    Collides {
        /// The minted id the reject's message must name — which of the payload's items
        /// collided, the question the batch door left unanswered.
        minted: &'static str,
        /// The expected route argv (`{addr}` substituted).
        argv: &'static str,
        /// The section the landing is read back from …
        section: &'static str,
        /// … and the `{#id}` anchor the second item must carry there afterwards.
        landed: &'static str,
    },
    /// The collision column's **omitting context**: a destination whose `id-from` is an
    /// **enum**, where the heading IS the member and `--slug` is refused as an identity
    /// change. The collision must stay on its shipped human route — "edit it in place",
    /// which is the true answer when the id *is* the category — so this asserts the whole
    /// route string verbatim. A mechanical `--slug` route here would name a command that
    /// blocks when run, which is the un-followable route this suite exists to forbid.
    StaysHuman(&'static str),
}

/// **The axis.** Six item-id misses, the undeclared-section miss, and the one genuine
/// declared-shape defect the flip leaves standing.
pub const CELLS: &[Cell] = &[
    Cell {
        what: "set-slot at a nonexistent item",
        args: &[
            "doc",
            "set-slot",
            "{addr}#releases/9-9-9/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "remove-item at a nonexistent item",
        args: &["doc", "remove-item", "{addr}#releases/9-9-9"],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --value at a nonexistent item",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/9-9-9/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --unset at a nonexistent item, declared field",
        args: &["doc", "set-field", "{addr}#releases/9-9-9/link", "--unset"],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --unset at a nonexistent item, undeclared field",
        args: &["doc", "set-field", "{addr}#releases/9-9-9/bogus", "--unset"],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "retitle-item at a nonexistent item",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/9-9-9",
            "--title",
            "9.9.9",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "nested add-item under an absent parent item",
        args: &[
            "doc",
            "add-item",
            "{addr}#releases/9-9-9/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        // The **deep** strip at one of the newly-enriched verbs: a real parent release
        // (`1-3-0`), an absent nested change-group — the route must still strip past two
        // hops to the top *showable* section (the N2 pin, re-asserted at `retitle-item`).
        what: "retitle-item at a nonexistent nested item under a real parent",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/1-3-0/changes/no-such-group",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    // ---- The **id-from leaf** strip (M47 Increment 6, T3). Two CLI-side, schema-only
    // pre-checks sit in front of the engine's presence adjudication at two of the six
    // enriched dispatch sites — `write.id-from-field` (`set-field` at an id-from leaf)
    // and `write.identity-change` (`retitle-item` under an **enum** id-from). Both
    // assert a property of an item and hand back a route whose first verb blocks at an
    // item that was never minted, so item presence must outrank them: the same
    // shape → presence → leaf order the engine's own item-field doors already keep.
    Cell {
        what: "set-field --value at a nonexistent item's id-from leaf",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/9-9-9/version",
            "--value",
            "9.9.9",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --value at a nonexistent item's enum id-from leaf",
        args: &[
            "doc",
            "set-field",
            "{addr}#staged/no-such-group/category",
            "--value",
            "fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("staged"),
    },
    Cell {
        what: "set-field --value at a nonexistent nested item's enum id-from leaf",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/1-3-0/changes/no-such-group/category",
            "--value",
            "fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "retitle-item at a nonexistent item under an enum id-from",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#staged/no-such-group",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("staged"),
    },
    // ---- The **undeclared-section** column. The row below (`add-item` at a bare
    // undeclared section) was for a long time the column's only cell, so the matrix had
    // 1 of N there while the item-id-miss column had every verb — and five of the six
    // item-addressing verbs disagreed underneath it: `set-slot` / `remove-item` claimed
    // `write.not-present` (and handed back a `jigc doc show <doc>#<undeclared>` that
    // **exits 1** — a route that does not answer), `retitle-item` / nested `add-item`
    // claimed `write.wrong-shape`, and `set-field --unset` claimed `write.unknown-field`
    // about a field on an item in a section that does not exist. Shape outranks presence
    // and presence outranks the leaf (`design/write-commands.md` → Adjudication order),
    // so an undeclared section is `write.unknown-section` at **every** door, whose route
    // is the schema read that genuinely answers it.
    Cell {
        what: "set-slot at an item in an undeclared section",
        args: &[
            "doc",
            "set-slot",
            "{addr}#no-such-section/9-9-9/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at an item in an undeclared section",
        args: &["doc", "remove-item", "{addr}#no-such-section/9-9-9"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --value at an item in an undeclared section",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/9-9-9/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --unset at an item in an undeclared section",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/9-9-9/link",
            "--unset",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at an item in an undeclared section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#no-such-section/9-9-9",
            "--title",
            "9.9.9",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "nested add-item under an item in an undeclared section",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section/9-9-9/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item into an undeclared section",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    // ---- The **section-level** arm of the same column (M49 Increment 11, T3). Every row
    // above carries an **item hop**, so the address reaches a write door and the engine's
    // rank-1 `section_undeclared` answers it. These do not: `set-slot` at `#<undeclared>`
    // and `set-field` at `#<undeclared>/<leaf>` bottom out in the CLI's own target
    // resolvers, which searched the schema for a *slot* / a *field* and — finding neither,
    // because the section they would live in does not exist — refused with a bare
    // `{"error": "no slot addressed by …"}`: exit 1, **code-less, route-less, outside the
    // finding envelope**, so a driver keying on `(code, target)` sees nothing at all and an
    // agent is handed no recovery. `jigc doc author` reaches the same two resolvers through
    // its batch lowering, one arm each (a payload section's `set:` key is a field hop; the
    // section's own id is its slot, key-less), so it produced the bare form twice more. The
    // miss is identical to the rows above — the schema declares no such section — so it
    // earns the same `write.unknown-section` and the same `jigc doc schema <doctype>` read,
    // asked at rank 1 in the resolver rather than inferred from a missing leaf
    // (`design/validation.md` → The `write.*` route split, whose universal this closes —
    // M47 increment 6, advisory 2).
    Cell {
        what: "set-slot at an undeclared section (no item hop)",
        args: &[
            "doc",
            "set-slot",
            "{addr}#no-such-section",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --value at a field in an undeclared section (no item hop)",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --unset at a field in an undeclared section (no item hop)",
        args: &["doc", "set-field", "{addr}#no-such-section/link", "--unset"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        // The batch door's **field** arm: a payload `set:` key under an undeclared section
        // lowers to `#<undeclared>/<key>`, the same address the per-leaf `set-field` above
        // carries — and `author.rs`'s own lowering comment already claimed the engine's
        // `write.unknown-section` handled it.
        what: "doc author batch naming an undeclared section carrying a field",
        args: &["doc", "author", "changelog", "--from-file", "-"],
        stdin: Some(
            b"title: Changelog\nsections:\n  - id: no-such-section\n    set:\n      link: \"https://x\"\n",
        ),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        // The batch door's **slot** arm: the section's own id is its slot key, which
        // lowers key-less to `#<undeclared>` — the per-leaf `set-slot` address, through the
        // other resolver. Both arms are driven, because one fix reaching only one of them
        // leaves the same payload answering two ways.
        what: "doc author batch naming an undeclared section carrying a slot",
        args: &["doc", "author", "changelog", "--from-file", "-"],
        stdin: Some(
            b"title: Changelog\nsections:\n  - id: no-such-section\n    set:\n      no-such-section: \"<<A summary.>>\"\n",
        ),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    // ---- The **nested** arm of the same column (M49 Increment 8, T1). Every row above
    // gets the section wrong at the address's FIRST hop; these get it wrong at a later one
    // — `#releases/1-3-0/bogus/xyz…`, where `releases` is declared, `1-3-0` is live, and
    // the nested-section segment `bogus` is declared nowhere in the release block. It is
    // the same miss one level down, and it came back four different ways: `write.not-
    // present` at `set-slot` / `remove-item` (whose route is a `jigc doc show …#releases`
    // that exits 0 and answers nothing, because the item ids it lists are not what the
    // address got wrong), `write.wrong-shape` at `set-field --value` / `retitle-item` /
    // both `add-item` shapes, and `write.unknown-field` at `set-field --unset` — a field
    // question about an item in a nested section that does not exist. Shape outranks
    // presence outranks the leaf at every depth, so all seven converge on the shipped
    // `write.unknown-section` and its schema read (`design/command-output-contract.md` →
    // Evolution posture, the M49 paragraph authorizing the flip on the pinned key).
    Cell {
        what: "set-slot at an item in an undeclared nested section",
        args: &[
            "doc",
            "set-slot",
            "{addr}#releases/1-3-0/bogus/xyz/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at an item in an undeclared nested section",
        args: &["doc", "remove-item", "{addr}#releases/1-3-0/bogus/xyz"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --value at an item in an undeclared nested section",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/1-3-0/bogus/xyz/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --unset at an item in an undeclared nested section",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/1-3-0/bogus/xyz/link",
            "--unset",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at an item in an undeclared nested section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/1-3-0/bogus/xyz",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "nested add-item under an item in an undeclared nested section",
        args: &[
            "doc",
            "add-item",
            "{addr}#releases/1-3-0/bogus/xyz/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        // The nested dual of `add-item into an undeclared section` directly above: the
        // destination IS the undeclared nested section, with no trailing item hop, so the
        // segment names a section rather than a leaf and the top-level row's answer is the
        // right one one level down.
        what: "add-item into an undeclared nested section",
        args: &[
            "doc",
            "add-item",
            "{addr}#releases/1-3-0/bogus",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item into a non-repeatable section (the genuine shape question)",
        args: &["doc", "add-item", "{addr}#overview", "--title", "Added"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    // ---- The **address-shape** column (M50 Increment 9, T1). Every column above reaches a
    // write door; these thirteen never did. `add-item` resolves its destination through
    // `add_item_target` and `remove-item` / `retitle-item` theirs through
    // `remove_item_target`, and an address whose *shape* the resolver cannot map — a bare
    // `<type>:<slug>` with no fragment at all, an `add-item` at an item, a `remove-item` /
    // `retitle-item` at a section or at a leaf — fell out of the resolver as `None` and was
    // dressed by a bare `with_context`: exit 1 carrying `{"error": "no section addressed by
    // …"}`, **code-less, route-less, outside the finding envelope**, exactly the shape M49's
    // section-level arm closed one seam over. A driver keying on `(code, target)` saw
    // nothing and the agent was handed no recovery, at three of the five `doc` write verbs.
    //
    // The column has **two dimensions**, because the right answer is not a property of the
    // address shape alone: rank 1 asks whether the *leading* hop names a declared section at
    // all (`engine::write::undeclared_section_splice`, the same rank-1 predicate the six
    // item-addressing doors and M49's section-level resolvers already ask), so an
    // undeclared leading hop is `write.unknown-section` whatever shape the rest of the
    // address takes, and only a **declared** one leaves a genuine declared-shape defect:
    // `write.wrong-shape`, whose sentence names the form the verb takes and never claims an
    // absence. `write.not-present` earns no row here at all — nothing has been looked for in
    // the corpus yet, so naming an item absent would be a law-1 lie about a document the
    // resolver has not read.
    //
    // Both codes route the same `jigc doc schema <doctype>` read, which is the point: a
    // shape question and a declaredness question are both answered by the schema, and it is
    // the *code* a driver keys on that had to stop being absent.
    Cell {
        what: "add-item at a bare doc address (no section hop at all)",
        args: &["doc", "add-item", "{addr}", "--title", "Added"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item at an item in a declared section",
        args: &["doc", "add-item", "{addr}#releases/1-3-0", "--title", "Added"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item at an item in an undeclared section",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section/1-3-0",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a bare doc address (no item hop at all)",
        args: &["doc", "remove-item", "{addr}"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a declared section",
        args: &["doc", "remove-item", "{addr}#releases"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at an undeclared section",
        args: &["doc", "remove-item", "{addr}#no-such-section"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a leaf on a live item in a declared section",
        args: &["doc", "remove-item", "{addr}#releases/1-3-0/summary"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a leaf on an item in an undeclared section",
        args: &["doc", "remove-item", "{addr}#no-such-section/9-9-9/summary"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a bare doc address (no item hop at all)",
        args: &["doc", "retitle-item", "{addr}", "--title", "Fixed"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a declared section",
        args: &["doc", "retitle-item", "{addr}#releases", "--title", "Fixed"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at an undeclared section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#no-such-section",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a leaf on a live item in a declared section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/1-3-0/summary",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a leaf on an item in an undeclared section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#no-such-section/9-9-9/summary",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    // ---- The address-shape column's **section-level** arm (M50 Increment 9, T2). The
    // thirteen rows above are the *item*-addressing resolvers; these seven are the other
    // two, `field_target` and `slot_target` — the resolvers M49 reached for the
    // undeclared-**section** miss alone. Their remaining bare arms answered every other
    // unmappable shape with the same code-less, route-less `{"error": "no field addressed
    // by …"}` / `{"error": "no slot addressed by …"}` envelope: a bare `<type>:<slug>`
    // with no fragment at all at either verb, a single-hop `#<name>` matching no declared
    // field at BOTH `set-field` flags, and — at `set-slot` — a declared section that hosts
    // no prose slot, whether because it is repeatable or because it declares none, plus an
    // item address with no slot leaf. `jigc doc author` inherits every one of them through
    // `apply_leaf`'s `SetField` / `SetSlot` arms, so this is seven cells at three doors.
    //
    // The declaredness dimension crosses this arm **once**, not twice: the rank-1
    // `undeclared_section_guard` M49 installed already answers the undeclared leading hop
    // at both hop-bearing forms, so what is left here is the declared side alone —
    // `write.wrong-shape`, the genuine declared-shape defect. The one exception is the
    // **single-hop** `#<name>` at `set-field`, which names no section at any hop: it is a
    // field-id search across every declared section, so `write.unknown-section` would be a
    // law-1 lie about sections that all exist. It is an undeclared **leaf** —
    // `write.unknown-field`, the contract's one member for *the schema declares no such
    // leaf here* — and the bound `validation.md` carried over it (*"it keeps the
    // resolver's own no field addressed by sentence"*) is rewritten there rather than
    // left standing false.
    Cell {
        what: "set-field at a bare doc address (no field hop at all)",
        args: &["doc", "set-field", "{addr}", "--value", "x"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --value at a single hop no section declares as a field",
        args: &["doc", "set-field", "{addr}#no-such-leaf", "--value", "x"],
        stdin: None,
        code: "write.unknown-field",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --unset at a single hop no section declares as a field",
        args: &["doc", "set-field", "{addr}#no-such-leaf", "--unset"],
        stdin: None,
        code: "write.unknown-field",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a bare doc address (no section hop at all)",
        args: &["doc", "set-slot", "{addr}", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a repeatable section (no section-level prose slot)",
        args: &["doc", "set-slot", "{addr}#releases", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        // The ingredient cell: a **declared** section that hosts no prose slot at all —
        // the shipped fields-only shape (a header section), which no other row reaches.
        what: "set-slot at a declared section that declares no prose slot",
        args: &["doc", "set-slot", "{addr}#meta", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a live item with no slot leaf hop",
        args: &["doc", "set-slot", "{addr}#releases/1-3-0", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    // ---- The **cross-term** the widened fence bought (M50 Increment 9, T3). Every row
    // above holds one dimension fixed: the address-shape rows all sit under a *declared*
    // leading hop, and the undeclared-section rows all stop at three hops. These six are
    // the corner the two columns share — a four-hop-or-deeper address whose LEADING hop is
    // declared nowhere, at all five addressed verbs, plus `set-slot`'s `#<undeclared>/<hop>`
    // (the one two-hop undeclared cell no verb but `set-slot` had left undriven). Rank 1
    // answers before the id chain is walked at all, so every one of them is
    // `write.unknown-section` however deep the address goes — which is what they assert,
    // and what nothing asserted before the cross demanded a witness per cell
    // (`MISS_SHAPES`; the fence's direction 1).
    Cell {
        what: "add-item at a nested destination under an undeclared leading hop",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section/9-9-9/bogus/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at a nested item under an undeclared leading hop",
        args: &["doc", "remove-item", "{addr}#no-such-section/9-9-9/bogus/xyz"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at a nested item under an undeclared leading hop",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#no-such-section/9-9-9/bogus/xyz",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field at a nested field under an undeclared leading hop",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/9-9-9/bogus/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a nested slot under an undeclared leading hop",
        args: &[
            "doc",
            "set-slot",
            "{addr}#no-such-section/9-9-9/bogus/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"Prose.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-slot at a leaf under an undeclared section (no item hop)",
        args: &["doc", "set-slot", "{addr}#no-such-section/xyz", "--from-file", "-"],
        stdin: Some(b"Prose.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    // ---- The **title-miss** column (M48 Increment 2, T2). The matrix's other columns are
    // all *address* misses; these two are the miss a **doc-minting** verb can make, and
    // until M48 neither was a miss at all — both exited 0. They split on whether the
    // identity moves, because the recovery does not: a call that would mint a DIFFERENT
    // identity beside the one this task holds is `write.identity-change` (a second
    // document, not a correction), while a call that lands on the SAME identity and
    // merely drops the title is identity-**stable** (`design/storage.md` → Identity) and
    // earns `write.title-ignored`. Both route at `jigc doc rename`, the in-task title
    // change — a route that is itself a write, hence `MutatingWrite`.
    Cell {
        what: "create a second identity while the gate's role is already bound",
        args: &["doc", "create", "changelog", "--title", "Release Log"],
        stdin: None,
        code: "write.identity-change",
        route: RouteCheck::MutatingWrite(
            "jigc doc rename {addr} --to 'Release Log' --task log-the-release",
        ),
    },
    Cell {
        what: "create at the held identity with a title that would be dropped",
        args: &["doc", "create", "changelog", "--title", "Changelog!"],
        stdin: None,
        code: "write.title-ignored",
        route: RouteCheck::MutatingWrite(
            "jigc doc rename {addr} --to 'Changelog!' --task log-the-release",
        ),
    },
    // ---- The **collision** column (M49 Increment 5, T5). Not a miss: the section is
    // declared, the shape is right, and the id the title mints is simply taken — the
    // fixture already holds release `1-3-0`, and the distinct title `1.3.0` mints the same
    // id. The shipped route said *"edit it in place"*, which is the answer for a
    // correction and the wrong answer for a second, distinct entry — and is not runnable,
    // so from inside a `doc author` payload (rejected whole, nothing staged) it terminated
    // nowhere. Both doors carry the same row because both funnel through the one
    // `apply_add_item_target` enrichment; a fix at only one of them leaves the other lying.
    Cell {
        what: "add-item at a title that mints a taken id",
        args: &["doc", "add-item", "{addr}#releases", "--title", "1.3.0"],
        stdin: None,
        code: "write.already-present",
        route: RouteCheck::Collides {
            minted: "1-3-0",
            argv: "jigc doc add-item {addr}#releases --title 1.3.0 --slug 1-3-0-2 \
                   --task log-the-release",
            section: "releases",
            landed: "1-3-0-2",
        },
    },
    Cell {
        what: "doc author batch carrying an item title that mints a taken id",
        args: &["doc", "author", "changelog", "--from-file", "-"],
        stdin: Some(
            b"title: Changelog\nsections:\n  - id: releases\n    items:\n      - title: \"1.3.0\"\n",
        ),
        code: "write.already-present",
        route: RouteCheck::Collides {
            minted: "1-3-0",
            argv: "jigc doc add-item {addr}#releases --title 1.3.0 --slug 1-3-0-2 \
                   --task log-the-release",
            section: "releases",
            landed: "1-3-0-2",
        },
    },
    Cell {
        // The omitting context, driven rather than reasoned about: `#staged`'s `id-from`
        // is an **enum**, so the same slug-alike collision (`Changed` is live; `changed`
        // mints the same id) must NOT be handed a `--slug` — that override is refused as
        // an identity change, so the route would block when run.
        what: "add-item at a taken enum id-from member (the omitting context)",
        args: &["doc", "add-item", "{addr}#staged", "--title", "changed"],
        stdin: None,
        code: "write.already-present",
        route: RouteCheck::StaysHuman(
            "the target already exists — edit it in place (`set-field`/`set-slot`) \
             instead of re-creating it",
        ),
    },
];
