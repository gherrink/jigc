//! The URI-shaped address grammar and the id-hop newtypes.
//!
//! Every addressable target has a URI-shaped address — *which resource*, then
//! *where inside it* ([design/structural-grammar.md](../../../design/structural-grammar.md)
//! → Addressing):
//!
//! ```text
//! address  := ref ( "#" fragment )?
//! ref      := type ":" slug                 # the container (a doc or workflow)
//! fragment := unit ( "/" leaf )?            # in a non-repeatable unit
//!           | unit "/" item ( "/" leaf )?   # in a repeatable unit
//! ```
//!
//! The grammar *permits* every depth uniformly; the schema *determines* which
//! depths are valid for a given unit. So this parser is purely structural: it
//! splits a fragment by hop count (1 = unit, 2 = unit/leaf, 3 = unit/item/leaf, and from
//! 4 up to [`MAX_FRAGMENT_HOPS`] a role-less nested path) and records the typed hops,
//! without consulting any schema. Parse → `Display`
//! reproduces the input byte-for-byte for every grammar form.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Macro: define a thin `String` newtype for one author-named id hop.
macro_rules! id_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            /// Borrow the inner id text.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                $name(s)
            }
        }
    };
}

id_newtype!(
    /// The container dialect/type hop — author-named in the schema (e.g. `adr`).
    Type
);
id_newtype!(
    /// The container instance slug hop (e.g. `single-node-cache`).
    Slug
);
id_newtype!(
    /// A unit (section / step) hop — author-named in the schema.
    Unit
);
id_newtype!(
    /// A repeatable-item hop — minted at item-add time.
    Item
);
id_newtype!(
    /// A leaf hop (field, slot, placeholder) — author-named in the schema.
    Leaf
);

/// The fragment after `#` — where inside the container an address points.
///
/// Variants mirror the grammar's structural depth, distinguished by hop count.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fragment {
    /// `#unit`
    Unit(Unit),
    /// `#unit/leaf`
    UnitLeaf(Unit, Leaf),
    /// `#unit/item`
    UnitItem(Unit, Item),
    /// `#unit/item/leaf`
    UnitItemLeaf(Unit, Item, Leaf),
    /// `#unit/item/…/leaf` — a **nested** (≥4-hop) path into a repeatable-inside-a
    /// -repeatable (the M22 multi-level lift; review finding S1). The parser is purely
    /// structural, so this carries the raw hop strings in order, *without* assigning
    /// unit/item/leaf roles (the write-side parent-scoped locator + the schema do that
    /// when the address is resolved). The depth is bounded by [`MAX_FRAGMENT_HOPS`],
    /// which is in turn what bounds the schema loader's nesting cap
    /// ([`crate::schema::MAX_NESTING_DEPTH`]).
    Deep(Vec<String>),
}

/// A parsed URI-shaped address: a container `ref`, and an optional `fragment`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address {
    /// The container type hop.
    pub r#type: Type,
    /// The container instance slug hop.
    pub slug: Slug,
    /// The optional in-container fragment.
    pub fragment: Option<Fragment>,
}

/// A failure to parse an address string against the grammar.
///
/// The variants split 3/3 on **whose fault it is**, and the CLI's verb boundary answers
/// each half differently (`crates/cli/src/doc.rs` → `address_parse_guidance`): the first
/// three are faults of the `<type>:<slug>` **head**, the last three faults of the `#…`
/// **fragment** over a head that already parsed. Adding a variant here obliges an author to
/// give it a message and a route there — that mapping is an exhaustive match.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    /// The `type` hop before `:` was empty.
    #[error("empty type")]
    EmptyType,
    /// No `:` separating type from slug.
    #[error("missing ':' between type and slug")]
    MissingColon,
    /// The `slug` hop was empty.
    #[error("empty slug")]
    EmptySlug,
    /// The fragment after `#` was empty.
    #[error("empty fragment")]
    EmptyFragment,
    /// A hop within the fragment was empty (e.g. a trailing or doubled `/`).
    #[error("empty fragment hop")]
    EmptyHop,
    /// The fragment carried more than [`MAX_FRAGMENT_HOPS`] hops — the depth budget, not a
    /// fixed three: the chain alternates item id and nested-section id, so the ceiling this
    /// breaks is a **nesting depth** ([`crate::schema::MAX_NESTING_DEPTH`], derived from it).
    #[error("too many fragment hops")]
    TooManyHops,
}

impl Fragment {
    /// Parse the text *after* `#` into a typed [`Fragment`].
    ///
    /// Splits by `/` into 1–3 hops (unit, unit/leaf, unit/item/leaf), or — from 4 up to
    /// [`MAX_FRAGMENT_HOPS`] — a role-less [`Fragment::Deep`] nested path, per the addressing
    /// grammar; an empty input, an empty hop, or a hop count over that budget is a
    /// [`ParseError`]. This is the single fragment recognizer the data-value
    /// path grammar ([`crate::data_value`]) also reuses for its `#fragment` slice.
    pub fn parse(frag: &str) -> Result<Self, ParseError> {
        if frag.is_empty() {
            return Err(ParseError::EmptyFragment);
        }
        let hops: Vec<&str> = frag.split('/').collect();
        if hops.iter().any(|h| h.is_empty()) {
            return Err(ParseError::EmptyHop);
        }
        Ok(match hops.as_slice() {
            [unit] => Fragment::Unit(Unit::from(unit.to_string())),
            [unit, leaf] => {
                Fragment::UnitLeaf(Unit::from(unit.to_string()), Leaf::from(leaf.to_string()))
            }
            [unit, item, leaf] => Fragment::UnitItemLeaf(
                Unit::from(unit.to_string()),
                Item::from(item.to_string()),
                Leaf::from(leaf.to_string()),
            ),
            // ≥4 hops is a **nested** path (the M22 multi-level lift), bounded by
            // `MAX_FRAGMENT_HOPS`; anything deeper overflows the addressable grammar.
            _ if hops.len() <= MAX_FRAGMENT_HOPS => {
                Fragment::Deep(hops.iter().map(|h| h.to_string()).collect())
            }
            _ => return Err(ParseError::TooManyHops),
        })
    }
}

/// The maximum number of `/`-separated fragment hops the grammar admits. Deeper than
/// this overflows ([`ParseError::TooManyHops`]).
///
/// **Each nesting level costs two hops, not one.** The chain the write path walks
/// (`write::physical_item_chain`) **alternates** an item id and the nested
/// section id that declares the next block, so an address into a repeatable at nesting
/// depth `D` is:
///
/// ```text
/// section / item / nested-section / item / … / leaf
/// │        └──────── 2D − 1 hops ────────┘   └ 1 hop
/// └ 1 hop
/// ```
///
/// — a **leaf write** at depth `D` is `1 + (2D − 1) + 1 = 2D + 1` hops, and a mint or
/// retitle at depth `D` is `2D`. Against this budget of six that is `D ≤ 2` for leaf
/// writes: the number [`crate::schema::MAX_NESTING_DEPTH`] is **derived** from, so the
/// loader can never again admit a depth whose addresses this grammar rejects. (This
/// comment previously modelled the budget as *"a section hop, up to four item hops, and
/// a leaf"* — one hop per level — which is how a loader cap of 4 and a write path of 2
/// sat side by side unnoticed; `DECISIONS.md` → 2026-08-28.)
pub const MAX_FRAGMENT_HOPS: usize = 6;

impl Address {
    /// Parse an address string against the URI grammar.
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let (reference, fragment) = match s.split_once('#') {
            Some((reference, frag)) => (reference, Some(frag)),
            None => (s, None),
        };

        let (ty, slug) = reference.split_once(':').ok_or(ParseError::MissingColon)?;
        if ty.is_empty() {
            return Err(ParseError::EmptyType);
        }
        if slug.is_empty() {
            return Err(ParseError::EmptySlug);
        }

        let fragment = fragment.map(Fragment::parse).transpose()?;

        Ok(Address {
            r#type: Type::from(ty.to_string()),
            slug: Slug::from(slug.to_string()),
            fragment,
        })
    }
}

impl FromStr for Address {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Address::parse(s)
    }
}

impl fmt::Display for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fragment::Unit(u) => write!(f, "{u}"),
            Fragment::UnitLeaf(u, l) => write!(f, "{u}/{l}"),
            Fragment::UnitItem(u, i) => write!(f, "{u}/{i}"),
            Fragment::UnitItemLeaf(u, i, l) => write!(f, "{u}/{i}/{l}"),
            Fragment::Deep(hops) => write!(f, "{}", hops.join("/")),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.r#type, self.slug)?;
        if let Some(frag) = &self.fragment {
            write!(f, "#{frag}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every grammar form round-trips parse → Display byte-for-byte.
    #[test]
    fn round_trips_every_form() {
        for input in [
            "adr:foo",                      // container
            "adr:foo#decision",             // #unit
            "adr:foo#decision/supersedes",  // #unit/leaf
            "adr:foo#criteria/item-a",      // #unit/item
            "adr:foo#criteria/item-a/text", // #unit/item/leaf
        ] {
            let addr = Address::parse(input).expect("valid address");
            assert_eq!(addr.to_string(), input, "round-trip mismatch for {input:?}");
        }
    }

    /// The two-hop fragment parses as unit/leaf with the expected typed hops.
    #[test]
    fn parses_typed_hops() {
        let addr = Address::parse("adr:foo#decision/supersedes").expect("valid");
        assert_eq!(addr.r#type, Type::from("adr".to_string()));
        assert_eq!(addr.slug, Slug::from("foo".to_string()));
        assert_eq!(
            addr.fragment,
            Some(Fragment::UnitLeaf(
                Unit::from("decision".to_string()),
                Leaf::from("supersedes".to_string()),
            ))
        );
    }

    /// The three-hop fragment parses as unit/item/leaf.
    #[test]
    fn parses_item_leaf_hops() {
        let addr = Address::parse("spec:s#criteria/c1/text").expect("valid");
        assert_eq!(
            addr.fragment,
            Some(Fragment::UnitItemLeaf(
                Unit::from("criteria".to_string()),
                Item::from("c1".to_string()),
                Leaf::from("text".to_string()),
            ))
        );
    }

    /// The `#unit/item` form is byte-identical to `#unit/leaf`, so a bare
    /// parser yields `UnitLeaf`; the `UnitItem` variant is reached only with
    /// schema knowledge. It still renders to the same two-hop bytes.
    #[test]
    fn unit_item_renders_two_hops() {
        let frag = Fragment::UnitItem(
            Unit::from("criteria".to_string()),
            Item::from("item-a".to_string()),
        );
        let addr = Address {
            r#type: Type::from("adr".to_string()),
            slug: Slug::from("foo".to_string()),
            fragment: Some(frag),
        };
        assert_eq!(addr.to_string(), "adr:foo#criteria/item-a");
    }

    /// Malformed inputs return a parse error rather than panicking.
    #[test]
    fn rejects_malformed() {
        for (input, want) in [
            (":foo", ParseError::EmptyType),
            ("adr", ParseError::MissingColon),
            ("adr:", ParseError::EmptySlug),
            ("adr:foo#", ParseError::EmptyFragment),
            ("adr:foo#u/", ParseError::EmptyHop),
            ("adr:foo#/u", ParseError::EmptyHop),
            // The depth cap stays real: `MAX_FRAGMENT_HOPS` is six (two hops per
            // nesting level, plus the section and leaf hops), so a seventh overflows.
            ("adr:foo#a/b/c/d/e/f/g", ParseError::TooManyHops),
        ] {
            assert_eq!(
                Address::parse(input),
                Err(want.clone()),
                "for input {input:?}"
            );
        }
    }

    /// T5 done-criterion (a): a 4-hop and a 5-hop nested fragment parse to the
    /// variable-depth [`Fragment::Deep`] path and round-trip parse → Display
    /// byte-identical; the pre-extension 1–3-hop forms still parse to their original
    /// variants (the lift is additive). The parser is purely structural — it splits by
    /// hop count without consulting any schema, so both nested-address conventions (the
    /// T4 `section/item/child/leaf` form and the design's `section/item/nested/child/leaf`
    /// form) parse identically.
    #[test]
    fn nested_deep_fragments_round_trip_and_are_additive() {
        // The 4-hop nested form T4's validate emits + the worked binary drives
        // (`#section/release/change-group/leaf`, no nested-section hop).
        let four = "changelog:cl#releases/1-2-0/added/notes";
        let addr = Address::parse(four).expect("4-hop nested address parses");
        assert_eq!(
            addr.fragment,
            Some(Fragment::Deep(vec![
                "releases".to_string(),
                "1-2-0".to_string(),
                "added".to_string(),
                "notes".to_string(),
            ])),
        );
        assert_eq!(addr.to_string(), four, "4-hop round-trips byte-identical");

        // The 5-hop design form carrying the nested-section hop (`changes`).
        let five = "changelog:cl#releases/1-2-0/changes/added/notes";
        let addr = Address::parse(five).expect("5-hop nested address parses");
        assert_eq!(
            addr.fragment,
            Some(Fragment::Deep(vec![
                "releases".to_string(),
                "1-2-0".to_string(),
                "changes".to_string(),
                "added".to_string(),
                "notes".to_string(),
            ])),
        );
        assert_eq!(addr.to_string(), five, "5-hop round-trips byte-identical");

        // Additive: the pre-extension 1–3-hop forms still parse to their original
        // (non-`Deep`) variants, byte-identical.
        for (input, want) in [
            (
                "adr:foo#decision",
                Fragment::Unit(Unit::from("decision".to_string())),
            ),
            (
                "adr:foo#decision/supersedes",
                Fragment::UnitLeaf(
                    Unit::from("decision".to_string()),
                    Leaf::from("supersedes".to_string()),
                ),
            ),
            (
                "spec:s#criteria/c1/text",
                Fragment::UnitItemLeaf(
                    Unit::from("criteria".to_string()),
                    Item::from("c1".to_string()),
                    Leaf::from("text".to_string()),
                ),
            ),
        ] {
            let addr = Address::parse(input).expect("≤3-hop parses");
            assert_eq!(addr.fragment, Some(want), "for input {input:?}");
            assert_eq!(addr.to_string(), input, "≤3-hop round-trips for {input:?}");
        }
    }
}
