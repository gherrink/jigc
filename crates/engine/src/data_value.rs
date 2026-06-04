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
use crate::result::CatalogEntry;

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
    /// The bound `task.id` scalar — the task's frozen minted slug.
    pub id: String,
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
///
/// `task` is `Some` for a `creates-task: true` workflow and `None` for a
/// `creates-task: false` one (the router and its kind), which composes with **no
/// task context** — any `task.*` reference against a `None` task is the blocking
/// `workflow-refs.task-ref-in-no-task-workflow` conformance error
/// ([write-commands.md](../../../design/write-commands.md) → Task origination).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposeContext {
    /// The engine-native `task` root binding — `None` in a `creates-task: false`
    /// (no-task) composition.
    pub task: Option<TaskRoot>,
    /// The engine-native `catalog` root — the live list of **selectable** work-
    /// workflows (`creates-task: true`) with their `when` hints, the router's
    /// input ([workflow-dialect.md](../../../design/workflow-dialect.md) → Workflow
    /// selection). Engine-native machinery that enumerates whatever work-workflows
    /// the cascade supplies, so "engine ships empty" holds; empty when unfed.
    pub catalog: Vec<CatalogEntry>,
    /// The engine-native `store` root — the committed managed store, keyed by
    /// doctype id. `store.<doctype-id>` enumerates every committed instance of
    /// that doctype as a **collection** of addresses (e.g. `store.specs` → every
    /// committed `spec` — pure navigation, no filtering;
    /// [workflow-dialect.md](../../../design/workflow-dialect.md) → data-value
    /// roots). The CLI feeds this list (it enumerates the committed
    /// `<location>/<slug>.md`); the resolver does **no** committed-store I/O (the
    /// determinism boundary), mirroring how `catalog` is fed. A doctype absent
    /// from the map (no committed instances) resolves to the empty collection —
    /// empty text, not a finding.
    pub store: BTreeMap<String, Vec<Address>>,
    /// The engine-native `milestone` work-unit root — the live sub-task ids of the
    /// milestone being composed (`{{milestone.tasks}}`, the `fan-out` step's
    /// list-source; [workflow-dialect.md](../../../design/workflow-dialect.md)
    /// → data-value roots). The CLI feeds the `TaskList::enumerate()` output
    /// (already canonically id-sorted) and the resolver does **no** work-unit I/O
    /// (the determinism boundary), mirroring how `catalog`/`store` are fed; the
    /// resolver also **sorts on resolve** so the emitted `fan-out` directive
    /// sequence is id-ordered regardless of feed order — the engine's determinism
    /// does not depend on the caller pre-sorting (Validation hardening #7). Empty
    /// when the composition has no milestone (no fan-out) or the milestone has no
    /// sub-tasks — the empty collection, not a finding.
    pub milestone: Vec<String>,
}

/// What a data-value [`Path`] resolves to against a [`ComposeContext`] — the
/// emitted classes ([workflow-dialect.md](../../../design/workflow-dialect.md)
/// → Leaves: Address vs content; Empty vs unresolvable; the `catalog` collection).
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
    /// The live **collection** of selectable work-workflows the engine-native
    /// `catalog` root resolves to (`workflow-dialect.md` → data-value roots /
    /// Workflow selection). A collection leaf: it carries the entries but is not
    /// navigable — a hop / `#fragment` / `@` past it is a structural error.
    Catalog {
        /// The selectable work-workflows, in cascade order, each with its `when`.
        entries: Vec<CatalogEntry>,
    },
    /// The live **collection** of committed instances the engine-native `store`
    /// root resolves to (`store.<doctype-id>`; `workflow-dialect.md` → data-value
    /// roots — "`store.<doctype-id>` enumerates the committed instances of that
    /// doctype as a collection of addresses"). A collection leaf like
    /// [`Catalog`](Resolution::Catalog): it carries the instance addresses but is
    /// not navigable — a `.relation` hop, `#fragment`, or `@` past it is a
    /// structural error. A doctype with no committed instances resolves to the
    /// empty collection (empty text, not a finding).
    Store {
        /// The committed instance addresses of one doctype, in fed order.
        entries: Vec<Address>,
    },
    /// The live **collection** of sub-task ids the engine-native `milestone` root
    /// resolves to (`milestone.tasks`; [workflow-dialect.md](../../../design/workflow-dialect.md)
    /// → data-value roots — "its `.tasks` resolves to the milestone's sub-task
    /// collection"). A collection leaf like [`Catalog`](Resolution::Catalog) /
    /// [`Store`](Resolution::Store): it carries the bare sub-task ids (the
    /// `fan-out` step fans over them) but is not navigable — a further `.relation`
    /// hop, `#fragment`, or `@` past it is a structural error. An empty milestone
    /// resolves to the empty collection (no finding).
    Milestone {
        /// The milestone's sub-task ids in **canonical id-sorted order** — the
        /// resolver sorts here, so the order is a pure function of the id *set* and
        /// never depends on the caller's feed order (Validation hardening #7).
        ids: Vec<String>,
    },
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
    /// - `store.<doctype-id>` (one bare doctype hop) → [`Resolution::Store`], the
    ///   committed instances of that doctype (empty when none); a `.relation` hop,
    ///   `#fragment`, or `@` past it is the `workflow-refs.store-not-navigable`
    ///   structural error (a collection root, like `catalog`).
    /// - `milestone.tasks` (the fixed `.tasks` leaf hop) → [`Resolution::Milestone`],
    ///   the milestone's sub-task ids (empty when none); `milestone` alone, a
    ///   non-`tasks` hop, a further hop past `.tasks`, a `#fragment`, or `@` is the
    ///   `workflow-refs.milestone-not-navigable` structural error (a collection leaf).
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

        // The engine-native `catalog` root is a **collection leaf**: a bare
        // `catalog` resolves to the fed entries; any `.relation` hop, `#fragment`,
        // or `@` content marker is a structural error — there is nothing to
        // navigate into, slice, or dereference past the collection
        // (workflow-dialect.md → Workflow selection).
        if root == "catalog" {
            if !self.hops.is_empty() || self.fragment.is_some() || self.marker {
                return Err(Finding::blocking(
                    "workflow-refs.catalog-not-navigable",
                    "`catalog` is a collection leaf — it takes no `.relation` hop, \
                     `#fragment`, or `@` content marker"
                        .to_owned(),
                    Location::at(1, 1),
                ));
            }
            return Ok(Resolution::Catalog {
                entries: ctx.catalog.clone(),
            });
        }

        // The engine-native `store` root is a **collection root**: `store.<doctype>`
        // (exactly one `.relation` hop = the doctype id, no `#fragment`, no `@`
        // marker) resolves to the committed instances of that doctype as a
        // collection. The doctype id is the lone hop, so `store` alone, a `@`
        // marker, a `#fragment`, or any hop past the doctype is a structural error
        // — there is nothing to navigate into, slice, or dereference past the
        // collection (workflow-dialect.md → data-value roots). A doctype with no
        // committed instances resolves to the **empty** collection (empty text,
        // not a finding).
        if root == "store" {
            let is_bare_doctype = self.hops.len() == 1 && self.fragment.is_none() && !self.marker;
            if !is_bare_doctype {
                return Err(Finding::blocking(
                    "workflow-refs.store-not-navigable",
                    "`store.<doctype-id>` is a collection root — it takes exactly one \
                     doctype-id hop and no further `.relation` hop, `#fragment`, or \
                     `@` content marker"
                        .to_owned(),
                    Location::at(1, 1),
                ));
            }
            let doctype = self.hops[0].as_str();
            return Ok(Resolution::Store {
                entries: ctx.store.get(doctype).cloned().unwrap_or_default(),
            });
        }

        // The engine-native `milestone` work-unit root is a **collection leaf**
        // reached by the fixed `.tasks` hop: `milestone.tasks` (exactly that one
        // hop, no `#fragment`, no `@` marker) resolves to the milestone's sub-task
        // ids as a collection. The `.tasks` hop is the lone navigation; `milestone`
        // alone, a `@` marker, a `#fragment`, any non-`tasks` first hop, or a
        // further hop past `.tasks` is a structural error — there is nothing to
        // navigate into, slice, or dereference past the collection
        // (workflow-dialect.md → data-value roots). An empty milestone resolves to
        // the **empty** collection (empty text, not a finding), mirroring `store`.
        if root == "milestone" {
            let is_bare_tasks = self.hops.len() == 1
                && self.hops[0].as_str() == "tasks"
                && self.fragment.is_none()
                && !self.marker;
            if !is_bare_tasks {
                return Err(Finding::blocking(
                    "workflow-refs.milestone-not-navigable",
                    "`milestone.tasks` is the only navigable milestone path — it takes \
                     exactly the `.tasks` hop and no further `.relation` hop, \
                     `#fragment`, or `@` content marker"
                        .to_owned(),
                    Location::at(1, 1),
                ));
            }
            // Sort here so the emitted `fan-out` directive sequence is id-ordered
            // regardless of the feed order the caller hands in — the engine's
            // determinism does not silently depend on the caller pre-sorting
            // (Validation hardening #7). The CLI feeds `TaskList::enumerate()`
            // output (already id-sorted), so this is a no-op on the production path;
            // it closes the order leak when any other caller feeds an unsorted set.
            let mut ids = ctx.milestone.clone();
            ids.sort();
            return Ok(Resolution::Milestone { ids });
        }

        if root != "task" {
            return Err(Finding::blocking(
                "workflow-refs.undeclared-root",
                format!("`{root}` is not a declared live-state root"),
                Location::at(1, 1),
            ));
        }

        // A `creates-task: false` workflow composes with no task context, so any
        // `task.*` reference is a conformance error — not an absent value — *before*
        // any per-hop classification (write-commands.md → Task origination).
        let task = match &ctx.task {
            Some(task) => task,
            None => {
                return Err(Finding::blocking(
                    "workflow-refs.task-ref-in-no-task-workflow",
                    "`task.*` cannot be referenced by a `creates-task: false` workflow \
                     (it composes with no task context)"
                        .to_owned(),
                    Location::at(1, 1),
                ));
            }
        };

        // `task` alone (no hop) is not an addressable value — there is nothing to
        // emit. The MVP data-values all name `intent` or a role.
        let Some(first) = self.hops.first() else {
            return Err(Finding::blocking(
                "workflow-refs.undeclared-role",
                "`task` must be followed by `.intent` or a declared role".to_owned(),
                Location::at(1, 1),
            ));
        };

        // The engine-native scalars (`id`, `intent`): no address/content
        // distinction, so `@` is an error (`command-catalog.md` → `task.id` is a
        // scalar; `workflow-dialect.md` → Leaves).
        if let Some(scalar) = match first.as_str() {
            "id" => Some(task.id.clone()),
            "intent" => Some(task.intent.clone()),
            _ => None,
        } {
            if self.marker {
                return Err(Finding::blocking(
                    "workflow-refs.at-marker-on-non-scalar",
                    format!(
                        "the `@` content marker cannot apply to scalar `task.{}`",
                        first.as_str()
                    ),
                    Location::at(1, 1),
                ));
            }
            return Ok(Resolution::Scalar { value: scalar });
        }

        // A context role: declared (present in the map) vs undeclared.
        match task.roles.get(first.as_str()) {
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
            task: Some(TaskRoot {
                id: "resolve-the-data-value".to_owned(),
                intent: "resolve the data-value path against live state".to_owned(),
                roles,
            }),
            catalog: Vec::new(),
            store: BTreeMap::new(),
            milestone: Vec::new(),
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
    /// - a structurally-invalid undeclared root (`milestone.x`) → structural `Err`.
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
        // (`milestone` is now a declared root, so this uses a still-undeclared one.)
        let undeclared = resolve("sprint.x", &ctx).expect_err("undeclared root is structural");
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

    /// A `creates-task: false` workflow composes with **no task context**
    /// (`ctx.task == None`), so any `task.*` reference is a conformance error, not
    /// an absent value ([write-commands.md](../../../design/write-commands.md) →
    /// Task origination; [validation.md](../../../design/validation.md) →
    /// workflow-refs). The finding is blocking and coded
    /// `workflow-refs.task-ref-in-no-task-workflow`, and the branch fires *before*
    /// any per-hop classification (scalar / role) so even `task.intent` blocks.
    #[test]
    fn task_ref_in_no_task_workflow_is_blocking() {
        let ctx = ComposeContext {
            task: None,
            catalog: Vec::new(),
            store: BTreeMap::new(),
            milestone: Vec::new(),
        };

        for path in [
            "task.intent",
            "task.id",
            "task.commit#summary",
            "task.decision.supersedes#decision",
            "@task.intent",
        ] {
            let err = resolve(path, &ctx)
                .expect_err(&format!("`{path}` against a no-task ctx must be a finding"));
            assert_eq!(
                err.code, "workflow-refs.task-ref-in-no-task-workflow",
                "`{path}` should be the no-task conformance code"
            );
            assert_eq!(err.severity, crate::finding::Severity::Blocking);
        }
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

    /// A no-task composition (`creates-task: false`, the router) carrying the live
    /// selectable-work-workflow catalog — the engine-native `catalog` root the
    /// router interpolates ([workflow-dialect.md](../../../design/workflow-dialect.md)
    /// → Workflow selection).
    fn router_ctx() -> ComposeContext {
        ComposeContext {
            task: None,
            catalog: vec![
                CatalogEntry::new("single-task", "Implement one well-scoped change."),
                CatalogEntry::new("quick-fix", "A small, localized fix."),
            ],
            store: BTreeMap::new(),
            milestone: Vec::new(),
        }
    }

    /// Core done-criterion: a **bare** `catalog` head (no `.` hops, no `#fragment`,
    /// no `@` marker) resolves to [`Resolution::Catalog`] carrying the fed entries
    /// verbatim — the collection leaf the router's option list renders from
    /// ([workflow-dialect.md](../../../design/workflow-dialect.md) → data-value roots:
    /// the engine-native `catalog` root).
    #[test]
    fn bare_catalog_resolves_to_catalog_collection() {
        let ctx = router_ctx();

        assert_eq!(
            resolve("catalog", &ctx).expect("bare `catalog` resolves"),
            Resolution::Catalog {
                entries: vec![
                    CatalogEntry::new("single-task", "Implement one well-scoped change."),
                    CatalogEntry::new("quick-fix", "A small, localized fix."),
                ],
            }
        );
    }

    /// The catalog is a **collection leaf**, not a navigable root: a `.relation`
    /// hop, a `#fragment`, and the `@` content marker are *each* a blocking
    /// `workflow-refs.catalog-not-navigable` conformance error — there is nothing to
    /// hop into, slice, or dereference past the collection.
    #[test]
    fn navigated_catalog_is_blocking() {
        let ctx = router_ctx();

        for path in ["catalog.tasks", "catalog#summary", "@catalog"] {
            let err =
                resolve(path, &ctx).expect_err(&format!("`{path}` navigates the collection leaf"));
            assert_eq!(
                err.code, "workflow-refs.catalog-not-navigable",
                "`{path}` should be the catalog-not-navigable conformance code"
            );
            assert_eq!(err.severity, crate::finding::Severity::Blocking);
        }
    }

    /// A composition context fed a committed `store` of two `spec` instances — the
    /// `store` root the `implement-from-spec` `locate-from-spec` step interpolates
    /// (`{{store.specs}}`; [workflow-dialect.md](../../../design/workflow-dialect.md)
    /// → data-value roots). The CLI feeds this list; the resolver never reads the
    /// committed store itself (the determinism boundary).
    fn two_spec_store_ctx() -> ComposeContext {
        // The `store` root is keyed by the doctype's path-facing collection name —
        // the schema location stem (`specs/` → `specs`), exactly the `store.specs`
        // the design uses (`DECISIONS.md` 2026-06-01). The resolver is agnostic: it
        // looks up whatever string follows `store.`; the CLI owns the keying.
        let mut store = BTreeMap::new();
        store.insert(
            "specs".to_owned(),
            vec![
                Address::parse("spec:gateway-rate-limiting").expect("valid address"),
                Address::parse("spec:auth-token-rotation").expect("valid address"),
            ],
        );
        ComposeContext {
            task: None,
            catalog: Vec::new(),
            store,
            milestone: Vec::new(),
        }
    }

    /// Core done-criterion: a **bare** `store.<doctype-id>` (one doctype hop, no
    /// `#fragment`, no `@` marker) resolves to [`Resolution::Store`] carrying the
    /// fed committed instances verbatim — the collection the `locate-from-spec`
    /// step renders as a Content list. An unfed doctype resolves to the **empty**
    /// collection (empty text, not a finding).
    #[test]
    fn bare_store_doctype_resolves_to_store_collection() {
        let ctx = two_spec_store_ctx();

        assert_eq!(
            resolve("store.specs", &ctx).expect("bare `store.specs` resolves"),
            Resolution::Store {
                entries: vec![
                    Address::parse("spec:gateway-rate-limiting").expect("valid"),
                    Address::parse("spec:auth-token-rotation").expect("valid"),
                ],
            }
        );

        // A doctype with no committed instances → the empty collection, not a finding.
        assert_eq!(
            resolve("store.adrs", &ctx).expect("an unfed doctype resolves to empty"),
            Resolution::Store {
                entries: Vec::new()
            }
        );
    }

    /// `store.<doctype-id>` is a **collection root**, not a navigable one: a further
    /// `.relation` hop, a `#fragment`, and the `@` content marker are *each* a
    /// blocking `workflow-refs.store-not-navigable` conformance error — there is
    /// nothing to hop into, slice, or dereference past the collection (mirroring the
    /// `catalog` collection leaf).
    #[test]
    fn navigated_store_is_blocking() {
        let ctx = two_spec_store_ctx();

        for path in ["store.specs.x", "store.specs#goal", "@store.specs", "store"] {
            let err =
                resolve(path, &ctx).expect_err(&format!("`{path}` navigates the collection root"));
            assert_eq!(
                err.code, "workflow-refs.store-not-navigable",
                "`{path}` should be the store-not-navigable conformance code"
            );
            assert_eq!(err.severity, crate::finding::Severity::Blocking);
        }
    }

    /// A composition context fed the milestone's id-sorted sub-task list — the
    /// engine-native `milestone` work-unit root the `fan-out` step's
    /// `{{milestone.tasks}}` reads ([workflow-dialect.md](../../../design/workflow-dialect.md)
    /// → data-value roots). The CLI feeds the `TaskList::enumerate()` output (already
    /// id-sorted); the resolver does **no** work-unit I/O (the determinism boundary).
    fn two_task_milestone_ctx() -> ComposeContext {
        ComposeContext {
            milestone: vec!["alpha-fix".to_owned(), "zebra-fix".to_owned()],
            ..ComposeContext::default()
        }
    }

    /// Core done-criterion: a bare `milestone.tasks` (the fixed `.tasks` leaf hop,
    /// no `#fragment`, no `@` marker) resolves to [`Resolution::Milestone`] carrying
    /// the sub-task ids in **canonical id-sorted order** — the collection the
    /// `fan-out` step fans over. An empty milestone resolves to the **empty**
    /// collection (no finding).
    #[test]
    fn bare_milestone_tasks_resolves_to_milestone_collection() {
        let ctx = two_task_milestone_ctx();

        assert_eq!(
            resolve("milestone.tasks", &ctx).expect("bare `milestone.tasks` resolves"),
            Resolution::Milestone {
                ids: vec!["alpha-fix".to_owned(), "zebra-fix".to_owned()],
            }
        );

        // The resolver sorts: an unsorted feed resolves id-ordered, so the emit
        // path's order is a pure function of the id set, not the caller's feed
        // order (Validation hardening #7).
        let unsorted = ComposeContext {
            milestone: vec!["zebra-fix".to_owned(), "alpha-fix".to_owned()],
            ..ComposeContext::default()
        };
        assert_eq!(
            resolve("milestone.tasks", &unsorted).expect("unsorted milestone resolves"),
            Resolution::Milestone {
                ids: vec!["alpha-fix".to_owned(), "zebra-fix".to_owned()],
            }
        );

        // An empty milestone → the empty collection, not a finding.
        let empty = ComposeContext::default();
        assert_eq!(
            resolve("milestone.tasks", &empty).expect("empty milestone resolves to empty"),
            Resolution::Milestone { ids: Vec::new() }
        );
    }

    /// `milestone.tasks` is a **collection leaf** reached by the fixed `.tasks` hop,
    /// not a navigable root: `milestone` alone, the `@` content marker, a `#fragment`,
    /// a further `.relation` hop, and any non-`tasks` first hop are *each* a blocking
    /// `workflow-refs.milestone-not-navigable` conformance error.
    #[test]
    fn navigated_milestone_is_blocking() {
        let ctx = two_task_milestone_ctx();

        for path in [
            "milestone",
            "@milestone.tasks",
            "milestone.tasks#x",
            "milestone.tasks.y",
            "milestone.other",
        ] {
            let err = resolve(path, &ctx)
                .expect_err(&format!("`{path}` is not a navigable milestone path"));
            assert_eq!(
                err.code, "workflow-refs.milestone-not-navigable",
                "`{path}` should be the milestone-not-navigable conformance code"
            );
            assert_eq!(err.severity, crate::finding::Severity::Blocking);
        }
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
