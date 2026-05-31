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

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::address::{self, Address, Fragment, Slug, Type};
use crate::finding::{Finding, Location};

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

/// The live-state binding of one engine-native `task` root, against which a
/// data-value [`Path`] resolves during composition.
///
/// MVP carries the only engine-native root: `task`. It holds the bound scalar
/// `intent` and the workflow-declared **context roles**
/// ([write-commands.md](../../../design/write-commands.md) → Task origination —
/// "A task carries context roles its workflow declares; the agent binds them
/// explicitly"). A role present in [`roles`](TaskRoot::roles) is **declared**
/// (structurally valid); its value is `Some(address)` when **bound** and `None`
/// when **declared-but-unbound**. The declared/undeclared split is what
/// separates an *absent* value (empty text) from a *structural* error
/// ([workflow-dialect.md](../../../design/workflow-dialect.md) → Empty vs
/// unresolvable): a declared-but-unbound role yields [`Resolution::Absent`]; an
/// undeclared root or role is a `workflow-refs` [`Finding`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskRoot {
    /// The bound `task.intent` scalar — the human intent that minted the task.
    pub intent: String,
    /// The workflow-declared context roles. A present key is declared; its value
    /// is `Some` when bound to a document address, `None` when unbound.
    pub roles: BTreeMap<String, Option<Address>>,
}

/// The composition context a data-value [`Path`] resolves against — the live
/// state feed (`overrides.md` → Resolution algorithm, the read path).
///
/// MVP carries exactly the engine-native `task` root; later roots (`store`,
/// `milestone`) join as fields here without changing the resolver's contract.
/// Resolution is a **pure function of `(path, ctx)`** — no I/O, no clock, no LLM
/// (the determinism boundary; same resolved cascade in → same workflow out).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposeContext {
    /// The engine-native `task` root binding.
    pub task: TaskRoot,
}

/// What a data-value [`Path`] resolves to against a [`ComposeContext`] — the
/// four emitted classes ([workflow-dialect.md](../../../design/workflow-dialect.md)
/// → Leaves: Address vs content; Empty vs unresolvable).
///
/// - [`Scalar`](Resolution::Scalar) — a bare scalar string (`task.intent`); has
///   no address/content distinction, so `@` on it is a structural error.
/// - [`Address`](Resolution::Address) — the **reference** a bare path resolves
///   to (`task.commit#summary` → `commit:<slug>#summary`).
/// - [`Content`](Resolution::Content) — the **content** at a resolved address,
///   reached via the `@` dereference marker. MVP carries the dereferenced
///   address as the content handle; the byte-read against the store lands when
///   the store is wired.
/// - [`Absent`](Resolution::Absent) — a structurally-valid path that currently
///   resolves to nothing (a declared-but-unbound role); emits **empty text**,
///   never a finding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Resolution {
    /// A bare scalar string value.
    Scalar {
        /// The scalar text.
        value: String,
    },
    /// The reference (address) a bare path resolves to.
    Address {
        /// The resolved reference.
        address: Address,
    },
    /// The content at a resolved address (reached via `@`).
    Content {
        /// The address whose content is dereferenced.
        address: Address,
    },
    /// A structurally-valid path that currently resolves to nothing (empty text).
    Absent,
}

impl Path {
    /// Resolve this path against a [`ComposeContext`]'s live state.
    ///
    /// A **pure function** of `(self, ctx)` — same inputs always yield the same
    /// [`Resolution`] (the determinism boundary). MVP resolves against the one
    /// engine-native `task` root:
    ///
    /// - `task.intent` → [`Resolution::Scalar`]; `@task.intent` is the
    ///   `at-marker-on-non-scalar` structural error (a scalar has no content to
    ///   dereference — [validation.md](../../../design/validation.md) → Severity
    ///   inventory).
    /// - `task.<role>` for a **declared, bound** role → [`Resolution::Address`]
    ///   built from the role's binding plus this path's own `#fragment`; with the
    ///   `@` marker → [`Resolution::Content`].
    /// - `task.<role>` for a **declared, unbound** role → [`Resolution::Absent`]
    ///   (empty text), *before* any further `.relation` hops — an unbound role
    ///   short-circuits to absent regardless of the navigation past it
    ///   ([workflow-dialect.md](../../../design/workflow-dialect.md) → Empty vs
    ///   unresolvable).
    /// - an **undeclared** root, or an undeclared role under `task`, is a
    ///   blocking `workflow-refs.undeclared-*` [`Finding`] — a *structural* error,
    ///   not an absent value.
    ///
    /// A literal `type:name` [`Head::Doc`] head is a committed-store read that
    /// needs the wired store/edge-index; it is out of MVP resolution scope and
    /// surfaces as an `unresolved` finding here.
    pub fn resolve(&self, ctx: &ComposeContext) -> Result<Resolution, Finding> {
        let root = match &self.head {
            Head::Root(root) => root,
            Head::Doc(ty, slug) => {
                return Err(unresolved(format!(
                    "literal document head `{ty}:{slug}` is not resolvable at MVP compose scope"
                )));
            }
        };

        if root != "task" {
            return Err(Finding::blocking(
                "workflow-refs.undeclared-root",
                format!("`{root}` is not a declared live-state root"),
                Location::at(1, 1),
            ));
        }

        // `task` alone (no hop) is not an addressable value — there is nothing to
        // emit. The MVP data-values all name `intent` or a role.
        let Some(first) = self.hops.first() else {
            return Err(Finding::blocking(
                "workflow-refs.undeclared-role",
                "`task` must be followed by `.intent` or a declared role".to_owned(),
                Location::at(1, 1),
            ));
        };

        // The `intent` scalar: no address/content distinction, so `@` is an error.
        if first.as_str() == "intent" {
            if self.marker {
                return Err(Finding::blocking(
                    "workflow-refs.at-marker-on-non-scalar",
                    "the `@` content marker cannot apply to scalar `task.intent`".to_owned(),
                    Location::at(1, 1),
                ));
            }
            return Ok(Resolution::Scalar {
                value: ctx.task.intent.clone(),
            });
        }

        // A context role: declared (present in the map) vs undeclared.
        match ctx.task.roles.get(first.as_str()) {
            None => Err(Finding::blocking(
                "workflow-refs.undeclared-role",
                format!("`task.{}` is not a declared context role", first.as_str()),
                Location::at(1, 1),
            )),
            // Declared but unbound → absent (empty text), regardless of any
            // further `.relation` hops navigating past it.
            Some(None) => Ok(Resolution::Absent),
            // Declared and bound: build the address from the binding plus this
            // path's own `#fragment`. Cross-doc `.relation` hops past a bound role
            // need the edge index (post-MVP); the MVP data-values do not walk past
            // a bound role.
            Some(Some(binding)) => {
                let address = Address {
                    r#type: binding.r#type.clone(),
                    slug: binding.slug.clone(),
                    fragment: self.fragment.clone().or_else(|| binding.fragment.clone()),
                };
                Ok(if self.marker {
                    Resolution::Content { address }
                } else {
                    Resolution::Address { address }
                })
            }
        }
    }
}

/// A blocking `workflow-refs.unresolved` finding for a structurally-valid path
/// whose target cannot be reached at MVP compose scope (a committed-store read).
fn unresolved(message: impl Into<String>) -> Finding {
    Finding::blocking("workflow-refs.unresolved", message, Location::at(1, 1))
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

    /// The MVP `single-task` composition context: `task.intent` bound to a real
    /// intent, `commit` bound (a `creates-task` workflow's commit doc is the
    /// task's own slug), and `decision` **declared but unbound** (the agent has
    /// not yet created the ADR). `spec`/`stranger` are deliberately absent — an
    /// undeclared role.
    fn single_task_ctx() -> ComposeContext {
        let mut roles = BTreeMap::new();
        roles.insert(
            "commit".to_owned(),
            Some(Address::parse("commit:resolve-the-data-value").expect("valid address")),
        );
        roles.insert("decision".to_owned(), None);
        ComposeContext {
            task: TaskRoot {
                intent: "resolve the data-value path against live state".to_owned(),
                roles,
            },
        }
    }

    fn resolve(s: &str, ctx: &ComposeContext) -> Result<Resolution, Finding> {
        Path::parse(s).expect("valid path").resolve(ctx)
    }

    /// Core done-criterion (`data_value_resolution_cases`): the four resolution
    /// classes plus the two structural-error splits.
    ///
    /// - `task.intent` → `Scalar(the intent)`.
    /// - `task.commit#summary` (bare) → `Address` (`commit:<slug>#summary`).
    /// - an unbound `task.decision…` path → `Absent` (empty), distinct from
    /// - a structurally-invalid undeclared root (`store.x`) → structural `Err`.
    /// - `@task.intent` → `at-marker-on-non-scalar` structural `Err`.
    #[test]
    fn data_value_resolution_cases() {
        let ctx = single_task_ctx();

        // Scalar.
        assert_eq!(
            resolve("task.intent", &ctx).expect("scalar resolves"),
            Resolution::Scalar {
                value: "resolve the data-value path against live state".to_owned()
            }
        );

        // Address: the bare `task.commit#summary` resolves to the reference.
        assert_eq!(
            resolve("task.commit#summary", &ctx).expect("address resolves"),
            Resolution::Address {
                address: Address::parse("commit:resolve-the-data-value#summary")
                    .expect("valid address")
            }
        );

        // Absent: a declared-but-unbound role yields empty, even with hops past it.
        assert_eq!(
            resolve("task.decision.supersedes#decision", &ctx).expect("absent, not error"),
            Resolution::Absent
        );

        // Structural error: an undeclared root is *not* absent — it is a finding.
        let undeclared = resolve("store.x", &ctx).expect_err("undeclared root is structural");
        assert_eq!(undeclared.code, "workflow-refs.undeclared-root");
        assert_eq!(undeclared.severity, crate::finding::Severity::Blocking);

        // Structural error: `@` on the `intent` scalar is at-marker-on-non-scalar.
        let at_scalar = resolve("@task.intent", &ctx).expect_err("@-on-scalar is structural");
        assert_eq!(at_scalar.code, "workflow-refs.at-marker-on-non-scalar");
        assert_eq!(at_scalar.severity, crate::finding::Severity::Blocking);
    }

    /// The unbound-role-Absent vs undeclared-role-Err split, sharpened: the same
    /// `task.<role>` shape resolves to `Absent` when the role is declared-but-
    /// unbound (`decision`) and to a structural `Err` when the role is undeclared
    /// (`spec`, never declared by this workflow). The distinction is *binding*, not
    /// *syntax* ([workflow-dialect.md](../../../design/workflow-dialect.md) → Empty
    /// vs unresolvable).
    #[test]
    fn unbound_role_is_absent_undeclared_role_is_error() {
        let ctx = single_task_ctx();

        assert_eq!(
            resolve("task.decision", &ctx).expect("unbound declared role is absent"),
            Resolution::Absent
        );

        let err = resolve("task.spec#criteria", &ctx).expect_err("undeclared role is structural");
        assert_eq!(err.code, "workflow-refs.undeclared-role");
    }

    /// The `@` content marker dereferences a bound role to `Content`; the bare path
    /// to `Address`. Same path, marker flipped — the two emitted classes.
    #[test]
    fn at_marker_dereferences_bound_role_to_content() {
        let ctx = single_task_ctx();
        let addr = Address::parse("commit:resolve-the-data-value#summary").expect("valid");

        assert_eq!(
            resolve("task.commit#summary", &ctx).expect("bare → address"),
            Resolution::Address {
                address: addr.clone()
            }
        );
        assert_eq!(
            resolve("@task.commit#summary", &ctx).expect("@ → content"),
            Resolution::Content { address: addr }
        );
    }

    /// Golden over the resolution variants: a table of `(path → Resolution)` pins
    /// the four-class emitted contract (the `serde` projection, tagged `kind`), so
    /// a variant rename or a resolution-rule change breaks it.
    #[test]
    fn resolution_variants_golden() {
        let ctx = single_task_ctx();
        let cases = [
            "task.intent",
            "task.commit#summary",
            "@task.commit#summary",
            "task.decision.supersedes#decision",
        ];
        let rows: Vec<String> = cases
            .iter()
            .map(|p| {
                let r = resolve(p, &ctx).expect("resolves");
                format!(
                    "{p}\n  => {}",
                    serde_json::to_string(&r).expect("serializes")
                )
            })
            .collect();
        insta::assert_snapshot!(rows.join("\n"), @r#"
        task.intent
          => {"kind":"scalar","value":"resolve the data-value path against live state"}
        task.commit#summary
          => {"kind":"address","address":{"type":"commit","slug":"resolve-the-data-value","fragment":{"Unit":"summary"}}}
        @task.commit#summary
          => {"kind":"content","address":{"type":"commit","slug":"resolve-the-data-value","fragment":{"Unit":"summary"}}}
        task.decision.supersedes#decision
          => {"kind":"absent"}
        "#);
    }

    proptest::proptest! {
        /// Determinism: resolving the same `(path, ctx)` twice yields an identical
        /// `Resolution` (or an identical error). Resolution is a pure function of
        /// its inputs — no I/O, no clock (the determinism boundary). The generator
        /// covers the `@` marker, the `task` root and an undeclared root, and the
        /// `intent`/bound/unbound/undeclared role hops, each with optional fragment.
        #[test]
        fn resolution_is_pure_function_of_path_and_ctx(
            marker in proptest::bool::ANY,
            root in proptest::sample::select(vec!["task", "store"]),
            hop in proptest::sample::select(vec!["intent", "commit", "decision", "stranger"]),
            frag in proptest::option::of("[a-z][a-z0-9-]{0,5}"),
        ) {
            let ctx = single_task_ctx();
            let mut s = String::new();
            if marker {
                s.push('@');
            }
            s.push_str(root);
            s.push('.');
            s.push_str(hop);
            if let Some(f) = &frag {
                s.push('#');
                s.push_str(f);
            }
            let path = Path::parse(&s).expect("generated path parses");

            let first = path.resolve(&ctx);
            let second = path.resolve(&ctx);
            proptest::prop_assert_eq!(first, second);
        }
    }
}
