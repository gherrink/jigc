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
//! splits a fragment by hop count (1 = unit, 2 = unit/leaf, 3 = unit/item/leaf)
//! and records the typed hops, without consulting any schema. Parse → `Display`
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
    /// The fragment had more than the three permitted hops.
    #[error("too many fragment hops")]
    TooManyHops,
}

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

        let fragment = match fragment {
            None => None,
            Some(frag) => {
                if frag.is_empty() {
                    return Err(ParseError::EmptyFragment);
                }
                let hops: Vec<&str> = frag.split('/').collect();
                if hops.iter().any(|h| h.is_empty()) {
                    return Err(ParseError::EmptyHop);
                }
                Some(match hops.as_slice() {
                    [unit] => Fragment::Unit(Unit::from(unit.to_string())),
                    [unit, leaf] => Fragment::UnitLeaf(
                        Unit::from(unit.to_string()),
                        Leaf::from(leaf.to_string()),
                    ),
                    [unit, item, leaf] => Fragment::UnitItemLeaf(
                        Unit::from(unit.to_string()),
                        Item::from(item.to_string()),
                        Leaf::from(leaf.to_string()),
                    ),
                    _ => return Err(ParseError::TooManyHops),
                })
            }
        };

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
            ("adr:foo#a/b/c/d", ParseError::TooManyHops),
        ] {
            assert_eq!(
                Address::parse(input),
                Err(want.clone()),
                "for input {input:?}"
            );
        }
    }
}
