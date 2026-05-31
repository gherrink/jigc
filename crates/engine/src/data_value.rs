//! The data-value path grammar — `{{path}}` / `{{@path}}` graph navigation.
//!
//! A data-value is a **start** + `.relation` hops + an optional `#fragment`
//! slice ([design/workflow-dialect.md](../../../design/workflow-dialect.md) →
//! Leaves / Data-values are full graph navigation):
//!
//! ```text
//! data-value := "@"? head ( "." relation )* ( "#" fragment )?
//! head       := root | type:name          # a live-state root, or a literal doc id
//! fragment   := unit ( "/" item )? ( "/" leaf )?   # the addressing fragment
//! ```
//!
//! This module is **parse-only**: it splits the path text into the typed shape
//! and records the optional `@` marker and the ordered relation hops. It does
//! **no** live-state resolution — what each hop resolves to (scalar / address /
//! content / collection) is composition's job, not the parser's. The `#fragment`
//! reuses the one [`crate::address::Fragment`] recognizer (the addressing
//! grammar), so the two grammars never drift. Parse → [`Display`](std::fmt::Display)
//! reproduces the input byte-for-byte for every grammar form.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::address::{self, Fragment, Slug, Type};

/// One `.relation` hop — an author-named relation edge crossed during navigation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Relation(String);

impl Relation {
    /// Borrow the inner relation name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for Relation {
    fn from(s: String) -> Self {
        Relation(s)
    }
}

/// The path's start: a live-state `root` (`task`, `store`, …) or a literal
/// `type:name` document id.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Head {
    /// A live-state root — a single bare name (`task`, `store`, `milestone`).
    Root(String),
    /// A literal document id — `type:name` (`prd:billing`).
    Doc(Type, Slug),
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Head::Root(root) => f.write_str(root),
            Head::Doc(ty, slug) => write!(f, "{ty}:{slug}"),
        }
    }
}

/// A parsed data-value path: the optional `@` content marker, the [`Head`] start,
/// the ordered `.relation` hops, and the optional `#fragment` slice.
///
/// `marker` records whether the path was `@`-prefixed (dereference to **content**
/// at the resolved address) versus bare (resolve to the **address**). The
/// distinction is recorded structurally here; the read/dereference itself happens
/// in composition.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Path {
    /// `true` when the path was written `@path` (content), `false` for bare `path`.
    pub marker: bool,
    /// The start of navigation.
    pub head: Head,
    /// The ordered `.relation` hops crossed after the head (may be empty).
    pub hops: Vec<Relation>,
    /// The optional `#fragment` slice into the final document.
    pub fragment: Option<Fragment>,
}

/// A failure to parse a data-value path against the grammar.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    /// The head before the first `.` / `#` was empty (e.g. `@`, `#summary`).
    #[error("empty head")]
    EmptyHead,
    /// A `.relation` hop was empty (e.g. a trailing or doubled `.`, as in `task.`).
    #[error("empty relation hop")]
    EmptyHop,
    /// The `type:name` head had an empty type before `:`.
    #[error("empty head type")]
    EmptyHeadType,
    /// The `type:name` head had an empty slug after `:`.
    #[error("empty head slug")]
    EmptyHeadSlug,
    /// The `#fragment` slice was malformed against the addressing grammar.
    #[error("malformed fragment: {0}")]
    Fragment(#[from] address::ParseError),
}

impl Path {
    /// Parse a data-value path string against the grammar.
    ///
    /// Strips an optional leading `@`, splits off an optional `#fragment` at the
    /// first `#` (parsed via the shared [`Fragment`] recognizer), then splits the
    /// remaining `head ( "." relation )*` on `.`: the first segment is the
    /// [`Head`] (a bare `root`, or a `type:name` doc id), the rest are
    /// [`Relation`] hops. An empty head or an empty relation hop is a typed error.
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let (marker, rest) = match s.strip_prefix('@') {
            Some(rest) => (true, rest),
            None => (false, s),
        };

        let (nav, fragment) = match rest.split_once('#') {
            Some((nav, frag)) => (nav, Some(Fragment::parse(frag)?)),
            None => (rest, None),
        };

        let mut segments = nav.split('.');
        let head_text = segments.next().expect("split yields at least one segment");
        if head_text.is_empty() {
            return Err(ParseError::EmptyHead);
        }
        let head = match head_text.split_once(':') {
            Some((ty, slug)) => {
                if ty.is_empty() {
                    return Err(ParseError::EmptyHeadType);
                }
                if slug.is_empty() {
                    return Err(ParseError::EmptyHeadSlug);
                }
                Head::Doc(Type::from(ty.to_string()), Slug::from(slug.to_string()))
            }
            None => Head::Root(head_text.to_string()),
        };

        let mut hops = Vec::new();
        for hop in segments {
            if hop.is_empty() {
                return Err(ParseError::EmptyHop);
            }
            hops.push(Relation::from(hop.to_string()));
        }

        Ok(Path {
            marker,
            head,
            hops,
            fragment,
        })
    }
}

impl FromStr for Path {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Path::parse(s)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.marker {
            f.write_str("@")?;
        }
        write!(f, "{}", self.head)?;
        for hop in &self.hops {
            write!(f, ".{hop}")?;
        }
        if let Some(frag) = &self.fragment {
            write!(f, "#{frag}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Core done-criterion: every canonical form parses into the typed shape with
    /// the `@` marker and the ordered hop list recorded, and the golden pins the
    /// full `Debug` projection so a representation change breaks it.
    #[test]
    fn data_value_path_parses_every_form() {
        let forms = [
            "task.intent",
            "task.commit#summary",
            "@task.spec#criteria",
            "@task.decision.supersedes#decision",
            "prd:billing#goal",
        ];
        let parsed: Vec<Path> = forms
            .iter()
            .map(|s| Path::parse(s).unwrap_or_else(|e| panic!("{s:?} should parse: {e}")))
            .collect();

        // The `@` marker and the relation hops are recorded.
        assert!(!parsed[0].marker);
        assert_eq!(parsed[0].hops, vec![Relation::from("intent".to_string())]);
        assert!(parsed[2].marker, "`@task.spec#criteria` carries the marker");
        assert_eq!(
            parsed[3].hops,
            vec![
                Relation::from("decision".to_string()),
                Relation::from("supersedes".to_string()),
            ],
            "`@task.decision.supersedes#decision` records both hops in order"
        );
        assert_eq!(
            parsed[4].head,
            Head::Doc(
                Type::from("prd".to_string()),
                Slug::from("billing".to_string())
            ),
            "`prd:billing` is a literal doc head"
        );

        let labelled: Vec<String> = forms
            .iter()
            .zip(&parsed)
            .map(|(input, path)| format!("{input}\n  => {path:?}"))
            .collect();
        insta::assert_snapshot!(labelled.join("\n"), @r###"
        task.intent
          => Path { marker: false, head: Root("task"), hops: [Relation("intent")], fragment: None }
        task.commit#summary
          => Path { marker: false, head: Root("task"), hops: [Relation("commit")], fragment: Some(Unit(Unit("summary"))) }
        @task.spec#criteria
          => Path { marker: true, head: Root("task"), hops: [Relation("spec")], fragment: Some(Unit(Unit("criteria"))) }
        @task.decision.supersedes#decision
          => Path { marker: true, head: Root("task"), hops: [Relation("decision"), Relation("supersedes")], fragment: Some(Unit(Unit("decision"))) }
        prd:billing#goal
          => Path { marker: false, head: Doc(Type("prd"), Slug("billing")), hops: [], fragment: Some(Unit(Unit("goal"))) }
        "###);
    }

    /// Every canonical form round-trips parse → `Display` byte-for-byte.
    #[test]
    fn round_trips_every_canonical_form() {
        for input in [
            "task.intent",
            "task.commit#summary",
            "@task.spec#criteria",
            "@task.decision.supersedes#decision",
            "prd:billing#goal",
        ] {
            let path = Path::parse(input).expect("valid path");
            assert_eq!(path.to_string(), input, "round-trip mismatch for {input:?}");
        }
    }

    /// An empty hop, a bare marker, and a malformed/empty fragment are typed
    /// errors — not panics, not silent acceptance.
    #[test]
    fn rejects_malformed() {
        assert_eq!(Path::parse("task."), Err(ParseError::EmptyHop));
        assert_eq!(Path::parse("@"), Err(ParseError::EmptyHead));
        assert_eq!(Path::parse("#summary"), Err(ParseError::EmptyHead));
        // `task#` is an empty fragment, surfaced through the shared recognizer.
        assert_eq!(
            Path::parse("task#"),
            Err(ParseError::Fragment(address::ParseError::EmptyFragment))
        );
        // A doubled relation hop is empty.
        assert_eq!(Path::parse("task..intent"), Err(ParseError::EmptyHop));
        // A four-hop fragment overflows the addressing grammar.
        assert_eq!(
            Path::parse("task#a/b/c/d"),
            Err(ParseError::Fragment(address::ParseError::TooManyHops))
        );
        // An empty type / slug in a literal head.
        assert_eq!(Path::parse(":billing"), Err(ParseError::EmptyHeadType));
        assert_eq!(Path::parse("prd:"), Err(ParseError::EmptyHeadSlug));
    }

    proptest::proptest! {
        /// Round-trip property: any generated valid path string parses, and its
        /// `Display` reproduces the source byte-for-byte. The generator covers the
        /// optional `@`, a root or `type:name` head, 0..4 `.relation` hops, and an
        /// optional 1..3-hop `#fragment`.
        #[test]
        fn display_round_trips_generated_paths(
            marker in proptest::bool::ANY,
            head_is_doc in proptest::bool::ANY,
            root in "[a-z][a-z0-9-]{0,7}",
            ty in "[a-z][a-z0-9-]{0,7}",
            slug in "[a-z][a-z0-9-]{0,7}",
            hops in proptest::collection::vec("[a-z][a-z0-9-]{0,7}", 0..4),
            frag in proptest::collection::vec("[a-z][a-z0-9-]{0,7}", 0..3),
        ) {
            let mut s = String::new();
            if marker {
                s.push('@');
            }
            if head_is_doc {
                s.push_str(&format!("{ty}:{slug}"));
            } else {
                s.push_str(&root);
            }
            for hop in &hops {
                s.push('.');
                s.push_str(hop);
            }
            if !frag.is_empty() {
                s.push('#');
                s.push_str(&frag.join("/"));
            }

            let path = Path::parse(&s).expect("generated path is valid");
            proptest::prop_assert_eq!(path.to_string(), s);
        }
    }
}
