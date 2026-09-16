//! Orphan detection — committed managed-looking docs **stranded** at a doctype's prior home
//! after its home moved (M36 same-shape `docs-root` re-point; M39 the general relocation
//! floor). `design/validation.md` → Orphan detection; `design/storage.md` → docs-root.
//!
//! **The strand discriminator (M39 re-key).** A doc is stranded when it sits at a doctype's
//! **prior** home yet no longer at its **current** home — [`is_stranded`] over the
//! generalized [`Home`] vocabulary (`location` dir **or** `placement` file), which the two
//! arms share:
//!
//! - **found-stranded** — the prior home is *self-discovered* from the doc's own path, keyed
//!   on the part the home's knob leaves invariant: a `location:` **dir basename** under
//!   `docs-root`, or (since M49 Increment 7) a re-rootable `placement:` home's **declared
//!   remainder** under `placement-root`. Backs the store-scope `file-state.orphaned-doc`
//!   advisory ([`orphaned_docs`], `jigc validate`). The placement half exists only because
//!   the knob does: before it, a `placement.file` was a constant and no re-point could
//!   strand its instance. A placement home declared **at** the repo root (`VISION.md`) is
//!   not re-rootable and stays outside the arm — only the recorded arm sees a strand there.
//! - **recorded-prior-home** — the prior home is *supplied/recorded* (the M39 relocation +
//!   `config set docs-root` fire-points), so a placement→root, root→`docs`, `location`
//!   basename **rename**, or shape change (`location`↔`placement`) all become detectable —
//!   relocations the old location-dir-basename-only key was blind to.
//!
//! Matching by the `.md` extension alone is explicitly wrong — it would flag `README.md`
//! and every stray markdown file (a front-door false-positive). Enumeration is over
//! `git ls-files` (committed truth that survives a fresh clone, where the finalize-gated
//! file-state map does not). A `git` failure yields none — a best-effort advisory, never
//! an error. CLI-side because the engine ships empty by invariant and never shells to git
//! for tracked status.
//!
//! [`docs_root_would_orphan`] backs the pre-write `jigc config set docs-root` warning — the
//! live docs a re-point to a *different* root would strand.
//!
//! **The sibling condition, and why it is not a strand (M51 Increment 8 / T3).** Everything
//! above is about a *home* that moved while the doctype stayed. [`orphaned_instances`] is the
//! other half: a committed stamped doc **no resolved doctype claims** — the pack that defined
//! its type left the composition, or the file sits where no doctype homes — so there is no
//! home to compare it against and no schema to parse it with. It is
//! `schema-conformance.orphaned-instance`: blocking, and a member of
//! [`crate::render::STORE_EXIT_FLIPS`]. The two are partitioned by the strand walk's own
//! verdict — a path it can name a doctype for is the strand advisory's, and what it cannot
//! name stays the sibling's (`completions/artifacts/M51/settle-record.md` → §18). It lands
//! here rather than in the engine for the reason the strand arms do: the subject is the
//! **committed** set, which only `git ls-files` knows — narrowed by [`Territory`] to the
//! committed set **inside jigc's declared homes**, for the reason that type states.

use std::path::Path;

use engine::schema::Schema;

/// The `location:` directory **basename** of a persisted schema (`decisions` for a
/// resolved `docs/decisions/`), or `None` for a transient (location-less) schema. Stable
/// across a `docs-root` change — the discriminator the predicate keys on.
pub(crate) fn location_basename(schema: &Schema) -> Option<&str> {
    let dir = schema.location.as_deref()?.trim_end_matches('/');
    Path::new(dir).file_name()?.to_str()
}

/// Whether `rel` (a repo-relative path) sits directly under `schema`'s **resolved**
/// `location:` dir — mirrors `unmanage::under_location`. A transient (location-less)
/// schema is never a home.
pub(crate) fn under_location(rel: &str, schema: &Schema) -> bool {
    let Some(dir) = schema.location.as_deref().map(|l| l.trim_end_matches('/')) else {
        return false;
    };
    !dir.is_empty() && rel.starts_with(&format!("{dir}/"))
}

/// A doctype's **home** — the shape of where its instance(s) live, generalized off the
/// location-dir-basename key so the strand discriminator sees every relocation kind
/// (M39; `design/validation.md` → Orphan detection): a **`location`** directory (instances
/// at `<dir>/<slug>.md`) *or* a **`placement`** literal file (the one root/`docs` instance
/// at an exact path, bypassing `docs-root`). This is the vocabulary a **recorded/supplied
/// prior home** is expressed in, so a placement→root, root→`docs`, or `location`-basename
/// **rename** all become first-class — not just the M36 same-shape `docs-root` re-point.
pub(crate) enum Home {
    /// Instances live directly under this **normalized** directory (no trailing slash).
    Location(String),
    /// The one instance lives at this exact repo-root-relative file (a placement doctype).
    Placement(String),
}

impl Home {
    /// A location home from a `location:` string, normalized (trailing slash stripped);
    /// `None` for an empty/flat-root location (never a doctype home).
    pub(crate) fn location(dir: &str) -> Option<Self> {
        let dir = dir.trim_end_matches('/');
        (!dir.is_empty()).then(|| Home::Location(dir.to_string()))
    }

    /// A placement home at an exact repo-root-relative file.
    pub(crate) fn placement(file: &str) -> Self {
        Home::Placement(file.to_string())
    }

    /// Whether a committed repo-relative `.md` path sits **at** this home — directly under a
    /// `Location` dir, or exactly equal to a `Placement` file.
    pub(crate) fn contains(&self, rel: &str) -> bool {
        match self {
            Home::Location(dir) => rel.starts_with(&format!("{dir}/")),
            Home::Placement(file) => rel == file,
        }
    }
}

/// A doctype's **current** home — its `placement` file (bypassing `docs-root`) or its
/// resolved `location:` dir. `None` for a transient (home-less) doctype. `schemas` must
/// carry the `docs-root`-applied `location:` (the resolved roots).
pub(crate) fn home_of(schema: &Schema) -> Option<Home> {
    if let Some(p) = &schema.placement {
        return Some(Home::placement(&p.file));
    }
    Home::location(schema.location.as_deref()?)
}

/// **The strand discriminator** — a committed doc `rel` is stranded when it sits at a
/// doctype's **prior** home yet is no longer at its **current** home. Both the
/// **found-stranded** arm (prior home *self-discovered* from the doc's own dir, keyed on a
/// `location:` basename — [`orphaned_docs`]) and the **recorded-prior-home** arm (prior home
/// *supplied/recorded* — the M39 relocation + `config set docs-root` fire-points) route
/// through this one predicate, which sees placement/root/basename-rename relocations that
/// the old location-dir-basename key was blind to.
pub(crate) fn is_stranded(rel: &str, prior: &Home, current: &Home) -> bool {
    prior.contains(rel) && !current.contains(rel)
}

/// The immediate-parent directory (full repo-relative path) of a `.md` path —
/// `docs/decisions` for `docs/decisions/foo.md`. `None` for a root-level file (`README.md`),
/// which is never a `location`-homed managed doc.
fn parent_dir(rel: &str) -> Option<&str> {
    rel.rsplit_once('/').map(|(parent, _file)| parent)
}

/// The immediate-parent directory basename of a repo-relative `.md` path — `decisions`
/// for `docs/decisions/foo.md`. `None` for a root-level file (`README.md`), which is never
/// a managed doc — the false-positive a bare extension match would trip.
fn parent_basename(rel: &str) -> Option<&str> {
    parent_dir(rel).map(|parent| parent.rsplit('/').next().unwrap_or(parent))
}

/// Enumerate `git ls-files` under `repo_root`, returning the committed `.md` paths,
/// address-sorted. A `git` failure yields an empty listing (best-effort advisory). Shared
/// with the freeze-exempt relocation path (`crate::relocate`), which walks the same committed
/// truth to find the instances stranded at a supplied prior home.
pub(crate) fn committed_markdown(repo_root: &Path) -> Vec<String> {
    // `-z` NUL-terminates the listing so git never C-quotes a non-ASCII pathname
    // (default `core.quotepath=true` — the same mis-parse the ingest candidate set
    // hardened against, M40 F8); a quoted line would never match `record.get(rel)`
    // or the location-basename discriminator.
    let Ok(listing) = crate::task::git_capture(repo_root, &["ls-files", "-z"]) else {
        return Vec::new();
    };
    let mut out: Vec<String> = listing
        .split('\0')
        .filter(|l| l.ends_with(".md"))
        .map(str::to_string)
        .collect();
    out.sort();
    out
}

/// One committed doc found **stranded** — sitting at a shape that looks like a doctype's home
/// while the doctype's *current* home is elsewhere. Carries the three things the two-tier
/// advisory needs: the path, the **matched doctype** (the handle `jigc migrate <path> --as
/// <doctype>` needs on the unregistered tier — [`unregistered_route`]), and that doctype's
/// **current** home, which is both the destination a repair moves to and the discriminator the
/// finding's wording keys on (a `location:` home was moved by `docs-root`, a `placement:` home
/// by `placement-root` — naming the wrong knob would be a law-1 lie).
pub(crate) struct Strand {
    /// The committed repo-relative `.md` path sitting outside its doctype's current home.
    pub(crate) rel: String,
    /// The doctype whose home shape the path matched.
    pub(crate) doctype: String,
    /// That doctype's current resolved home.
    pub(crate) current: Home,
}

/// The committed `.md`s that **look like** managed docs yet fall **outside** the matched
/// doctype's current resolved home — the strands a home re-point leaves behind, one hit per
/// path, address-sorted (the walk is over the sorted [`committed_markdown`] listing).
///
/// Two self-discovery arms, one per home shape, each keyed on **the part its knob leaves
/// invariant** — that is what makes a self-discovered prior home possible at all:
///
/// - **`location:`** (M36) — the location-dir **basename**. `docs-root` is a whole-path prefix,
///   so `decisions` survives every re-point; a committed `<anywhere>/decisions/x.md` whose
///   doctype now homes elsewhere is stranded.
/// - **`placement:`** (M49 Increment 7 / T3) — the declared home's **remainder past its first
///   path component**. `placement-root` replaces exactly that first component
///   (`crate::start::reroot_placement_file`), so `roadmap.md` survives every re-point; a
///   committed `<anywhere>/roadmap.md` (or a root-level `roadmap.md`) that is not the doctype's
///   current home is stranded. Until the knob shipped this arm could not exist and was not
///   needed: a placement home was the literal its schema declared, so it could never move.
///
/// A placement doctype whose **declared** home carries no leading directory component
/// (`VISION.md`, `CHANGELOG.md`) is **not re-rootable** — the ecosystem-idiomatic rule is a
/// derivation, not an allow-list — so it contributes no arm at all and a stray `docs/VISION.md`
/// is never called a strand. That is why `declared` is a parameter: once the root is `.` a
/// *resolved* placement home is a bare filename whether it was declared nested or at the root,
/// and the two are then indistinguishable.
///
/// `resolved` must carry the `docs-root`-applied `location:` and the `placement-root`-applied
/// `placement.file`; `declared` the same schemas before either was applied
/// (`crate::start::CascadeDefs::{all_schemas, declared_schemas}`). Both are keyed by doctype.
pub(crate) fn orphaned_docs(
    repo_root: &Path,
    declared: &std::collections::BTreeMap<String, Schema>,
    resolved: &std::collections::BTreeMap<String, Schema>,
) -> Vec<Strand> {
    let schemas: Vec<&Schema> = resolved.values().collect();
    // The placement arm's key: per RE-ROOTABLE placement doctype, the remainder that survives
    // the re-root, paired with the current home the doc must no longer be at.
    let rerootable: Vec<(&Schema, &str)> = resolved
        .values()
        .filter_map(|schema| {
            schema.placement.as_ref()?;
            let file = &declared.get(&schema.ty)?.placement.as_ref()?.file;
            let (_, remainder) = file.split_once('/')?;
            Some((schema, remainder))
        })
        .collect();
    let mut hits = Vec::new();
    for rel in committed_markdown(repo_root) {
        // The `location:` arm first, so its pre-existing verdicts are untouched.
        if let Some(base) = parent_basename(&rel)
            // The doctype whose resolved location basename matches — at most one (basenames
            // are unique per doctype). No match → ordinary prose, never flagged.
            && let Some(schema) = schemas.iter().find(|s| location_basename(s) == Some(base))
            && let (Some(current), Some(parent)) = (home_of(schema), parent_dir(&rel))
            // Found-stranded self-discovery: the doc's own parent dir *looks like* a home (its
            // basename matched), so it is the self-discovered prior home; stranded iff the doc
            // is no longer at the doctype's CURRENT home.
            && is_stranded(&rel, &Home::Location(parent.to_string()), &current)
        {
            hits.push(Strand {
                rel,
                doctype: schema.ty.clone(),
                current,
            });
            continue;
        }
        // The `placement:` arm: the path carries a re-rootable doctype's invariant remainder.
        let Some((schema, _)) = rerootable
            .iter()
            .find(|(_, remainder)| at_remainder(&rel, remainder))
        else {
            continue;
        };
        let Some(current) = home_of(schema) else {
            continue; // a placement match always has a home — defensive.
        };
        // The self-discovered prior home of a placement strand is the doc's own path (a
        // placement home IS one file), so `is_stranded` reduces to "not at the current home" —
        // routed through the shared discriminator rather than re-spelled.
        if is_stranded(&rel, &Home::placement(&rel), &current) {
            hits.push(Strand {
                rel,
                doctype: schema.ty.clone(),
                current,
            });
        }
    }
    hits
}

/// Whether a committed repo-relative path sits **at** `remainder` under some parent — i.e. it
/// is `<anything>/<remainder>` or exactly `<remainder>` (the repo-root case a
/// `placement-root: .` produces). The path-boundary check is load-bearing: a bare
/// `ends_with(remainder)` would also match `docs/old-roadmap.md`.
fn at_remainder(rel: &str, remainder: &str) -> bool {
    rel == remainder || rel.ends_with(&format!("/{remainder}"))
}

/// **The directory trees jigc has been told are its own** — the subject bound of the
/// orphaned-instance sweep ([`orphaned_instances`]), and the answer to a question the stamp
/// itself cannot answer.
///
/// **Why a location rule and not a bytes rule (M51 completion audit, HIGH).** The stamp jigc
/// writes is the bare key `schema-version: N` in a document's YAML front matter — no
/// namespace, no producer, nothing that distinguishes jigc's stamp from the same key written
/// by anyone else for any other purpose. The sweep's first shipped subject was *every*
/// committed `.md` in the repository, so a plain team document carrying that key — at the repo
/// root, in `notes/`, anywhere — was named a jigc instance and flipped `jigc validate` to
/// exit 1 over bytes jigc never wrote. Since the stamp cannot discriminate, the **home** does:
/// jigc speaks only for stamped files sitting inside the trees an adopter's own configuration
/// has handed it.
///
/// **The three sources, each resolved the way the rest of the code resolves it** — never a raw
/// `schema.location`, and the placement branch handled (`CLAUDE.md` → the cross-cutting
/// gotcha):
///
/// - the resolved **`docs-root`** tree (`crate::start::docs_root_prefix`);
/// - the resolved **`placement-root`**, when it is not the repo root
///   (`crate::start::placement_root`);
/// - each resolved doctype's **home directory** — a `location:` doctype's resolved dir, and a
///   `placement:` doctype's resolved *parent* directory ([`home_of`]).
///
/// **Two residuals, stated rather than hidden** (`design/validation.md` → The M51
/// registrations — Increment 8):
///
/// 1. A **foreign** stamped `.md` *inside* the docs home still fires. That is accepted: the
///    docs tree is the directory the adopter handed jigc, and the finding's route already
///    names `jigc unmanage <path>` for a file that is not meant to be managed.
/// 2. An orphan at a **root-level placement home** — a departed doctype homed like `VISION.md`
///    — goes **unflagged**, because a repo-root home contributes no directory that is not the
///    whole repository. That one cell keeps its pre-M51 exit-0 status quo; widening back to it
///    needs a stamp that says *jigc* (`implementation/decisions-pending.md` → the namespaced
///    stamp key), not a wider directory rule.
///
/// The same reasoning applies to a **repo-root `docs-root`** (`""` / `.`): it contributes no
/// tree of its own, since *the whole repository* is precisely the subject this type exists to
/// stop the sweep from claiming. A doctype's own home directories still contribute, so the
/// homes a flat layout actually uses stay covered.
pub(crate) struct Territory {
    /// Repo-relative directories, normalized (no trailing slash, never empty). A committed
    /// path is inside the territory iff it sits under one of them.
    dirs: std::collections::BTreeSet<String>,
}

impl Territory {
    /// Derive the territory from the **resolved** cascade knobs and the **resolved** schema
    /// map. `docs_root` is [`crate::start::docs_root_prefix`]'s normalized value (`""` being
    /// the repo root) and `placement_root` is [`crate::start::placement_root`]'s (`None`
    /// unset, `Some("")` the repo root) — passed as values rather than resolved here so the
    /// knob vocabulary keeps its single home in `start.rs`.
    pub(crate) fn resolve(
        docs_root: &str,
        placement_root: Option<&str>,
        resolved: &std::collections::BTreeMap<String, Schema>,
    ) -> Self {
        let mut dirs = std::collections::BTreeSet::new();
        let mut add = |dir: &str| {
            let dir = dir.trim_matches('/');
            if !dir.is_empty() && dir != "." {
                dirs.insert(dir.to_string());
            }
        };
        add(docs_root);
        if let Some(root) = placement_root {
            add(root);
        }
        for schema in resolved.values() {
            match home_of(schema) {
                // A located doctype's resolved home dir — which a project shadow may place
                // outside `docs-root` entirely, so it is read rather than assumed.
                Some(Home::Location(dir)) => add(&dir),
                // A placement doctype's home is one file; its *directory* is the territory.
                // A home at the repo root has none — residual 2 above.
                Some(Home::Placement(file)) => {
                    if let Some(parent) = parent_dir(&file) {
                        add(parent);
                    }
                }
                None => {} // a transient doctype homes nowhere.
            }
        }
        Territory { dirs }
    }

    /// Whether a committed repo-relative path sits inside one of the territory's trees. The
    /// separator is part of the match, so `docs-archive/x.md` is not inside `docs`.
    pub(crate) fn contains(&self, rel: &str) -> bool {
        self.dirs
            .iter()
            .any(|dir| rel.starts_with(&format!("{dir}/")))
    }
}

/// The check id of the **orphaned-instance** break (M51 Increment 8 / T3) — a committed doc
/// **inside jigc's declared homes** ([`Territory`]) carrying a `schema-version:` stamp that
/// **no resolved doctype claims**. Named once so
/// the producer below, [`crate::render::STORE_EXIT_FLIPS`]' matcher and witness, and every
/// fence over either cannot drift apart on a string.
pub const ORPHANED_INSTANCE_CODE: &str = "schema-conformance.orphaned-instance";

/// The committed `.md`s **inside jigc's declared homes** ([`Territory`]) that carry a
/// **`schema-version:` stamp** yet are claimed by **no
/// resolved doctype** — the instances a doctype leaves behind when it leaves the composition,
/// and their sibling, a stamped file sitting where no doctype homes
/// (`design/validation.md` → The M51 registrations — Increment 8;
/// `completions/artifacts/M51/settle-record.md` → §18). Address-sorted: the walk is over the
/// sorted [`committed_markdown`] listing and the claimed set is a `BTreeSet`, so the output
/// is a function of the corpus and never of an enumeration order.
///
/// **Three legs, and each is the honest one available.** The stamp is read by
/// [`engine::validate::schema_version_from_front_matter`] — no parse and **no schema**, which
/// is the point: the type the stamp names is defined by nothing, so every reader that takes a
/// `&Schema` is unavailable here by construction (`crate::migrate_corpus`'s
/// `is_unadopted_foreign` among them). The claim is the union of
/// [`engine::index::committed_instances`] over the resolved schemas — the same census every
/// other committed-instance consumer reads, so "claimed" means claimed by the doors that act
/// on it, not by a second opinion. And the **subject** is the committed set inside
/// [`Territory`], not the repository: a stamped file outside jigc's declared homes is not this
/// condition's subject and is never named by it.
///
/// **That third leg replaced a false one (M51 completion audit, HIGH).** This comment used to
/// read *"a file jigc never stamped … is not this condition's subject and is never named by
/// it"* — true of the *stamp* leg and false as a statement about the sweep, because the stamp
/// is the unnamespaced key `schema-version:` and nothing makes it jigc's. A committed team
/// document carrying that key anywhere in the repository was named, blocking, and flipped
/// `jigc validate` to exit 1. The roadmap's *"needs `JIGC_PACK_DIR` or a PB-1 project pack"*
/// reachability bound was falsified by the same datum. [`Territory`] carries the narrowed
/// subject and both residuals it leaves.
///
/// `spoken_for` is the strand set [`orphaned_docs`] already reported. It is the **partition**,
/// not an optimization: a strand is a *self-discovered prior home* — a path the walk can name
/// a doctype for — and that doctype resolves, so the two conditions would otherwise claim one
/// path twice with different diagnoses (`design/validation.md` → Orphan detection, whose
/// subject this is not). What the strand walk cannot name it does not exclude: a stamped file
/// at a path matching no home shape stays this condition's, which is why the message states
/// the claim it computed rather than the cause it cannot know
/// ([`orphaned_instance_finding`]).
pub(crate) fn orphaned_instances(
    repo_root: &Path,
    resolved: &std::collections::BTreeMap<String, Schema>,
    territory: &Territory,
    spoken_for: &std::collections::BTreeSet<String>,
) -> Vec<String> {
    let claimed: std::collections::BTreeSet<String> = resolved
        .iter()
        .flat_map(|(ty, schema)| engine::index::committed_instances(repo_root, ty, schema))
        .map(|(_identity, path)| {
            path.strip_prefix(repo_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    committed_markdown(repo_root)
        .into_iter()
        .filter(|rel| territory.contains(rel))
        .filter(|rel| !claimed.contains(rel) && !spoken_for.contains(rel))
        .filter(|rel| carries_stamp(repo_root, rel))
        .collect()
}

/// Whether the committed file at `rel` carries a `schema-version:` stamp — *a* stamp, not
/// provably *jigc's*: the key is unnamespaced, which is why [`Territory`] bounds the subject.
/// A file git tracks
/// but the worktree no longer holds reads as **unstamped** — the sweep is a statement about
/// bytes it could read, never an assertion built on an absent file.
fn carries_stamp(repo_root: &Path, rel: &str) -> bool {
    std::fs::read_to_string(repo_root.join(rel))
        .ok()
        .and_then(|source| engine::validate::schema_version_from_front_matter(&source))
        .is_some()
}

/// The **blocking** store-scope finding for one orphaned instance, located at its path.
///
/// **The message states what was computed, not what was inferred.** Two things were computed
/// and the wording carries exactly those: the file sits **inside a home jigc was handed**
/// ([`Territory`]), and it carries a **`schema-version:` stamp** that **no resolved doctype
/// claims**. What was *not* computed is whose stamp it is — the key is unnamespaced, so
/// `a jigc schema-version stamp` (the wording this message shipped with until the M51
/// completion audit) was a law-1 overclaim about bytes jigc may never have written. It is
/// likewise not computed *why* nothing claims it: the pack that defined the type may have left
/// the composition, or the file may be a hand-placed copy or a bad merge sitting where no
/// doctype homes. Both are the one condition this code names, and the message says that much
/// and no more.
///
/// **The route names both directions of repair, and says what its exit leaves behind.**
/// Restoring what claims the file is the first — re-adding the pack that defines its type, or
/// moving the file to a resolved doctype's home, which is the exit that fits the population
/// actually reaching this finding (it is already inside jigc's homes; what it is not at is any
/// doctype's). `jigc unmanage` is the alternative — it runs cleanly here (a path at no
/// doctype's home drops its file-state baseline and exits 0) but **leaves the bytes on disk**,
/// so on its own it does not clear this finding. A route that, followed exactly, changes
/// nothing is the defect M46's PT-1 closed at another door, so that exit names the act that
/// finishes it (`DECISIONS.md` → 2026-09-15 M51 Increment 8 / T3).
pub(crate) fn orphaned_instance_finding(rel: &str) -> engine::finding::Finding {
    engine::finding::Finding::graded(
        engine::finding::Severity::Blocking,
        ORPHANED_INSTANCE_CODE,
        format!(
            "committed doc `{rel}` sits at a jigc-managed home and carries a \
             `schema-version:` stamp, but no resolved doctype claims it — no schema in the \
             composed set says what this file is"
        ),
        Some(engine::finding::Location::addressed(rel, 1, 1)),
        Some(engine::finding::Route::human(format!(
            "restore what claims it — re-add the pack that defines its type, or move it to a \
             resolved doctype's home — or take it out of jigc's world: `jigc unmanage \
             {token}`, then delete the file or its `schema-version:` stamp (`unmanage` drops \
             the baseline and leaves the bytes, so the stamp alone keeps this finding alive)",
            token = crate::task::shell_token(rel)
        ))),
    )
}

/// The route for the **unregistered** tier of the two-tier orphan advisory (M40;
/// `design/validation.md` → M40 two-tier route). A never-adopted basename-coincidence
/// file is not a tracked strand, so `jigc unmanage` (a clean no-op on a never-registered
/// doc — the adoption trial's proven no-op loop) is the wrong verb. When the
/// `migrate-<doctype>` workflow ships (`migratable`), the route names the real adoption
/// verb; when it does not, it falls back to ignore-or-human — never a command that
/// hard-errors.
pub(crate) fn unregistered_route(rel: &str, doctype: &str, migratable: bool) -> String {
    if migratable {
        format!(
            "adopt it with `jigc migrate {rel} --as {doctype}`, or ignore it if it is not \
             meant to be managed"
        )
    } else {
        format!(
            "ignore it if it is not meant to be managed, or route it to a human (no \
             `migrate-{doctype}` workflow ships to adopt it)"
        )
    }
}

/// The committed managed docs currently under an **old** resolved root that a `docs-root`
/// re-point to a *different* root would strand. `old_schemas` must carry the *current*
/// (`old_docs_root`-applied) `location:`. When the normalized root does not actually
/// change, nothing orphans (an empty list). Address-sorted.
pub(crate) fn docs_root_would_orphan<'a>(
    repo_root: &Path,
    old_schemas: impl IntoIterator<Item = &'a Schema>,
    old_docs_root: &str,
    new_docs_root: &str,
) -> Vec<String> {
    // Normalize both to `apply_docs_root`'s rule: strip surrounding slashes; `""` / `.`
    // are the flat repo-root layout.
    let norm = |v: &str| {
        let trimmed = v.trim_matches('/');
        if trimmed == "." { "" } else { trimmed }.to_string()
    };
    if norm(old_docs_root) == norm(new_docs_root) {
        return Vec::new(); // the root does not move — nothing orphans.
    }
    let old_schemas: Vec<&Schema> = old_schemas.into_iter().collect();
    let mut hits = Vec::new();
    for rel in committed_markdown(repo_root) {
        // A live managed doc under a current resolved root moves out from under it when
        // the root re-points → it would orphan.
        if old_schemas.iter().any(|s| under_location(&rel, s)) {
            hits.push(rel);
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn adr_schema(location: &str) -> Schema {
        let yaml = format!(
            "type: adr\nlocation: {location}\nid-from: title\nsections:\n  - id: body\n    slot: {{ hint: x }}\n"
        );
        engine::schema::load_schema(yaml.as_bytes()).expect("adr schema loads")
    }

    fn placement_schema(ty: &str, file: &str) -> Schema {
        let yaml = format!("type: {ty}\nplacement: {{ file: {file} }}\nsections: []\n");
        engine::schema::load_schema(yaml.as_bytes()).expect("placement schema loads")
    }

    /// A throwaway git repo that removes itself on drop (the project's no-tempfile pattern).
    struct TempRepo(PathBuf);

    impl TempRepo {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-orphan-unit-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create temp repo");
            let repo = TempRepo(path);
            repo.git(&["init", "-q"]);
            repo.git(&["config", "user.email", "t@t"]);
            repo.git(&["config", "user.name", "t"]);
            repo
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn git(&self, args: &[&str]) {
            let out = std::process::Command::new("git")
                .arg("-C")
                .arg(&self.0)
                .args(args)
                .env("GIT_CONFIG_GLOBAL", "/dev/null")
                .env("GIT_CONFIG_SYSTEM", "/dev/null")
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr),
            );
        }

        fn commit_file(&self, rel: &str, body: &str) {
            let abs = self.0.join(rel);
            if let Some(parent) = abs.parent() {
                std::fs::create_dir_all(parent).expect("create parent dir");
            }
            std::fs::write(&abs, body).expect("write file");
            self.git(&["add", "--", rel]);
            self.git(&["commit", "-q", "-m", "add"]);
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A doctype map keyed the way `CascadeDefs::{all_schemas, declared_schemas}` key theirs.
    fn map(schemas: Vec<Schema>) -> std::collections::BTreeMap<String, Schema> {
        schemas.into_iter().map(|s| (s.ty.clone(), s)).collect()
    }

    /// `(rel, doctype)` pairs — the shape the two callers below assert on.
    fn pairs(strands: Vec<Strand>) -> Vec<(String, String)> {
        strands.into_iter().map(|s| (s.rel, s.doctype)).collect()
    }

    /// (M38 inc-2 T3) Characterization — orphan detection keys on the immediate-parent
    /// **directory basename** matching a doctype's `location:` basename, so a **placement**
    /// doctype's root literal (`VISION.md`) is **never mis-orphaned**: a root file has no
    /// parent dir to match, and a placement doctype carries no `location` basename to match
    /// against (`design/storage.md` → Placement — census verification `orphan`). A sibling
    /// root `.md` (`README.md`) is likewise **not** the placement doctype's instance. A live
    /// `decisions/live.md` under its current root proves the sweep actually enumerates and
    /// stays quiet only where it should.
    ///
    /// (M49 inc-7 T3) Still true with the placement arm added, and now for a **derived**
    /// reason rather than an absent one: `VISION.md` is declared at the repo root, so it has
    /// no leading path component for `placement-root` to replace and contributes no arm.
    #[test]
    fn placement_root_file_and_sibling_root_md_are_never_orphaned() {
        let repo = TempRepo::new();
        repo.commit_file("VISION.md", "# Vision\n");
        repo.commit_file("README.md", "# Readme\n");
        repo.commit_file("decisions/live.md", "# A decision\n");

        let schemas = map(vec![
            placement_schema("vision", "VISION.md"),
            adr_schema("decisions/"),
        ]);
        assert!(
            orphaned_docs(repo.path(), &schemas, &schemas).is_empty(),
            "a placement root file, a sibling root .md, and a live decision are none orphaned",
        );
    }

    /// (M40 inc-3 T5) `orphaned_docs` carries the matched doctype beside each stranded
    /// path — the handle the two-tier route needs to name `jigc migrate <path> --as
    /// <doctype>` on the unregistered tier.
    #[test]
    fn orphaned_docs_surfaces_the_matched_doctype() {
        let repo = TempRepo::new();
        repo.commit_file("old/decisions/cache.md", "# A decision\n");
        let schemas = map(vec![adr_schema("docs/decisions/")]);
        assert_eq!(
            pairs(orphaned_docs(repo.path(), &schemas, &schemas)),
            vec![("old/decisions/cache.md".to_string(), "adr".to_string())],
            "a stranded hit names both the path and the doctype whose basename matched",
        );
    }

    /// (M49 inc-7 T3) **The placement arm's key is the DECLARED remainder, and the
    /// declared-vs-resolved distinction is load-bearing at exactly one value of the knob.**
    ///
    /// Under `placement-root: .` a nested-declared home (`docs/roadmap.md`) and a
    /// root-declared one (`VISION.md`) both resolve to a bare filename, so the *resolved*
    /// schemas alone cannot tell them apart. Feeding the declarations does: `notes/roadmap.md`
    /// is a strand of the re-rootable `roadmap`, while `docs/VISION.md` is not a strand of the
    /// unburiable `vision` — no `placement-root` value could have put it there, so saying one
    /// did would be a lie. Passing `resolved` for both arguments (the pre-knob world) flips
    /// the second assertion, which is what makes this a test of the parameter and not of the
    /// walk.
    #[test]
    fn the_placement_arm_keys_on_the_declared_remainder_not_the_resolved_home() {
        let repo = TempRepo::new();
        repo.commit_file("notes/roadmap.md", "# Roadmap\n");
        repo.commit_file("docs/VISION.md", "# Vision\n");

        let declared = map(vec![
            placement_schema("roadmap", "docs/roadmap.md"),
            placement_schema("vision", "VISION.md"),
        ]);
        // `placement-root: .` — the nested home flattens to the repo root, the root one stands.
        let resolved = map(vec![
            placement_schema("roadmap", "roadmap.md"),
            placement_schema("vision", "VISION.md"),
        ]);

        assert_eq!(
            pairs(orphaned_docs(repo.path(), &declared, &resolved)),
            vec![("notes/roadmap.md".to_string(), "roadmap".to_string())],
            "the re-rootable doctype's instance is stranded off the repo root; the \
             root-declared `vision` contributes no arm, so `docs/VISION.md` is not a strand",
        );
    }

    /// (M49 inc-7 T3) The remainder match is **path-boundary anchored**: a doc whose filename
    /// merely *ends with* the home's is ordinary prose, never a strand.
    #[test]
    fn a_filename_that_merely_ends_with_the_home_is_not_a_strand() {
        let repo = TempRepo::new();
        repo.commit_file("docs/old-roadmap.md", "# Not the roadmap\n");

        let declared = map(vec![placement_schema("roadmap", "docs/roadmap.md")]);
        let resolved = map(vec![placement_schema("roadmap", "notes/roadmap.md")]);
        assert!(
            orphaned_docs(repo.path(), &declared, &resolved).is_empty(),
            "`docs/old-roadmap.md` is not `<anywhere>/roadmap.md`",
        );
    }

    /// (M40 inc-3 T5) The unregistered-tier route builder names the real adoption verb —
    /// `jigc migrate <path> --as <doctype>` — when the `migrate-<doctype>` workflow ships,
    /// with ignore as the alternative.
    #[test]
    fn unregistered_route_names_the_migrate_verb_when_the_workflow_ships() {
        let route = unregistered_route("old/decisions/cache.md", "adr", true);
        assert!(
            route.contains("`jigc migrate old/decisions/cache.md --as adr`"),
            "the migratable route must carry the verbatim migrate invocation; got: {route}",
        );
        assert!(
            route.contains("ignore"),
            "ignore stays an offered alternative; got: {route}",
        );
    }

    /// (M40 inc-3 T5) Without a shipped `migrate-<doctype>` workflow the route falls back
    /// to ignore-or-human — it must never command a verb that hard-errors.
    #[test]
    fn unregistered_route_falls_back_to_ignore_or_human_without_a_migrate_workflow() {
        let route = unregistered_route("old/research/notes.md", "research", false);
        assert!(
            route.contains("route it to a human"),
            "the fallback route offers the human hand-off; got: {route}",
        );
        assert!(
            !route.contains("jigc migrate"),
            "no shipped workflow → no migrate invocation in the route; got: {route}",
        );
    }

    /// (M51 inc-8 T3; the subject leg added at the M51 completion audit) **The
    /// orphaned-instance predicate's legs, each driven against a real committed tree.** A
    /// stamped doc inside jigc's territory that no resolved doctype claims is the hit; the same
    /// doctype's live instance is claimed and silent; an unstamped file (`README.md`, and the
    /// adapter's own guide, whose front matter carries no `schema-version:` line) is not this
    /// condition's subject at all. The last is the leg that keeps the exit flip affordable —
    /// without it every prose file in a repo would flip `jigc validate`.
    #[test]
    fn orphaned_instances_names_the_stamped_unclaimed_docs_and_nothing_else() {
        let repo = TempRepo::new();
        repo.commit_file(
            "decisions/live.md",
            "---\nschema-version: 1\n---\n\n# Live\n",
        );
        repo.commit_file(
            "docs/roadmap.md",
            "---\nschema-version: 1\n---\n\n# Roadmap\n",
        );
        repo.commit_file("README.md", "# Readme\n");
        repo.commit_file("guide.md", "---\nname: guide\n---\n\n# Guide\n");

        let schemas = map(vec![adr_schema("decisions/")]);
        let territory = Territory::resolve("docs", None, &schemas);
        assert_eq!(
            orphaned_instances(repo.path(), &schemas, &territory, &Default::default()),
            vec!["docs/roadmap.md".to_string()],
            "the stamped doc no resolved doctype claims is the orphan; the adr's own committed \
             instance is claimed, and the two unstamped files were never jigc's to speak for",
        );
    }

    /// (M51 completion audit, HIGH) **The subject is jigc's declared territory, not the
    /// repository — driven over both shapes the audit reproduced.** A team document carrying a
    /// `schema-version:` key at the repo root and another in an arbitrary nested directory are
    /// both outside every home this configuration hands jigc, so neither is named; the same
    /// bytes inside the docs-root tree are. The stamp is an unnamespaced key, so the *home* is
    /// the only honest discriminator available (see [`Territory`]).
    #[test]
    fn a_stamped_doc_outside_the_territory_is_never_an_orphaned_instance() {
        let repo = TempRepo::new();
        let stamped = "---\ntitle: API notes\nschema-version: 3\n---\n\n# API notes\n";
        repo.commit_file("api-notes.md", stamped);
        repo.commit_file("notes/deep/api.md", stamped);
        repo.commit_file("docs/gone.md", stamped);

        let schemas = map(vec![adr_schema("decisions/")]);
        let territory = Territory::resolve("docs", None, &schemas);
        assert_eq!(
            orphaned_instances(repo.path(), &schemas, &territory, &Default::default()),
            vec!["docs/gone.md".to_string()],
            "a plain team document outside every jigc home is not this condition's subject; \
             the stamped file inside the docs home still is (the stated residual)",
        );
    }

    /// (M51 completion audit, HIGH) **The self-migration datum.** This repository's own
    /// committed test fixtures include stamped managed-doc bodies
    /// (`crates/cli/tests/fixtures/author-batch-scaling/spec-800-items.md`). Once jigc is
    /// pointed at its own corpus, a sweep whose subject is *every committed `.md`* blocks
    /// `jigc validate` on those fixtures forever. They sit under no docs-root, no
    /// `placement-root` and no doctype home, so the territory rule is what keeps them silent.
    #[test]
    fn this_repos_own_committed_test_fixtures_are_never_orphaned_instances() {
        let repo = TempRepo::new();
        repo.commit_file(
            "crates/cli/tests/fixtures/author-batch-scaling/spec-800-items.md",
            "---\nschema-version: 1\n---\n\n# A spec fixture\n",
        );

        let schemas = map(vec![adr_schema("decisions/")]);
        let territory = Territory::resolve("docs", None, &schemas);
        assert!(
            orphaned_instances(repo.path(), &schemas, &territory, &Default::default()).is_empty(),
            "a committed test fixture carrying a stamped body is not a managed instance, and a \
             sweep that claims it makes jigc unable to validate its own repository",
        );
    }

    /// (M51 completion audit, HIGH) **The territory's three sources, and the two homes that
    /// contribute nothing.** The docs-root tree, a non-root `placement-root` and every resolved
    /// doctype home directory are in — including a located doctype a project shadow homed
    /// outside `docs-root`. A **root-level** placement home (`VISION.md`) and a **repo-root**
    /// `docs-root` contribute no directory, because the only tree either names is the whole
    /// repository, which is the subject this type exists to refuse.
    #[test]
    fn the_territory_is_the_two_knobs_plus_every_resolved_doctype_home() {
        let schemas = map(vec![
            adr_schema("docs/decisions/"),
            adr_schema("elsewhere/specs/"), // a shadow homing a doctype off docs-root
            placement_schema("vision", "VISION.md"),
            placement_schema("roadmap", "papers/roadmap.md"),
        ]);
        let territory = Territory::resolve("docs", Some("papers"), &schemas);
        for inside in [
            "docs/gone.md",
            "docs/decisions/x.md",
            "elsewhere/specs/x.md",
            "papers/stray.md",
        ] {
            assert!(
                territory.contains(inside),
                "`{inside}` is inside a declared home"
            );
        }
        for outside in [
            "VISION.md",
            "api-notes.md",
            "notes/deep/api.md",
            "docs-archive/x.md",
        ] {
            assert!(
                !territory.contains(outside),
                "`{outside}` sits under no home this configuration declares",
            );
        }

        // A repo-root `docs-root` and an unset `placement-root` leave only the doctype homes —
        // never the whole repository.
        let flat = map(vec![
            adr_schema("decisions/"),
            placement_schema("vision", "VISION.md"),
        ]);
        let flat_territory = Territory::resolve("", None, &flat);
        assert!(flat_territory.contains("decisions/x.md"));
        assert!(
            !flat_territory.contains("api-notes.md") && !flat_territory.contains("notes/x.md"),
            "a flat layout does not make the whole repository jigc's to speak for",
        );
    }

    /// (M51 inc-8 T3) **The partition.** A path the strand walk already spoke for is not an
    /// orphaned instance: its doctype resolves and only its home moved, so naming it here
    /// would say the pack defining its type is gone while `jigc describe` still lists it.
    #[test]
    fn a_path_the_strand_walk_spoke_for_is_never_an_orphaned_instance() {
        let repo = TempRepo::new();
        // The strand sits INSIDE the docs-root tree, so the territory bound cannot be what
        // excludes it — otherwise this arm would prove the subject narrowing, not the partition.
        repo.commit_file(
            "docs/old/decisions/cache.md",
            "---\nschema-version: 1\n---\n\n# A\n",
        );

        let schemas = map(vec![adr_schema("docs/decisions/")]);
        let territory = Territory::resolve("docs", None, &schemas);
        let strands: Vec<(String, String)> = pairs(orphaned_docs(repo.path(), &schemas, &schemas));
        assert_eq!(
            strands,
            vec![("docs/old/decisions/cache.md".to_string(), "adr".to_string())],
            "the fixture must really be a strand, or the exclusion below proves nothing",
        );
        let spoken_for = strands.into_iter().map(|(rel, _)| rel).collect();
        assert!(
            orphaned_instances(repo.path(), &schemas, &territory, &spoken_for).is_empty(),
            "a strand is reported by `file-state.orphaned-doc` and by this condition never",
        );
        assert_eq!(
            orphaned_instances(repo.path(), &schemas, &territory, &Default::default()),
            vec!["docs/old/decisions/cache.md".to_string()],
            "without the exclusion the same path is claimed twice — which is what makes the \
             `spoken_for` argument the partition rather than an optimization",
        );
    }

    #[test]
    fn location_basename_is_the_trailing_dir_regardless_of_docs_root() {
        assert_eq!(
            location_basename(&adr_schema("decisions/")),
            Some("decisions")
        );
        assert_eq!(
            location_basename(&adr_schema("docs/decisions/")),
            Some("decisions")
        );
        assert_eq!(
            location_basename(&adr_schema("archive/decisions/")),
            Some("decisions")
        );
    }

    #[test]
    fn parent_basename_is_none_for_a_root_level_file() {
        assert_eq!(parent_basename("README.md"), None);
        assert_eq!(parent_basename("docs/guide.md"), Some("docs"));
        assert_eq!(
            parent_basename("docs/decisions/cache.md"),
            Some("decisions")
        );
    }

    /// (M39 inc-5 T2) `home_of` reads a doctype's home in the generalized vocabulary — a
    /// `placement` doctype homes at its literal file, a `location` doctype at its resolved
    /// dir (trailing slash stripped), a transient doctype has no home.
    #[test]
    fn home_of_reads_placement_and_location_homes() {
        assert!(matches!(
            home_of(&placement_schema("vision", "VISION.md")),
            Some(Home::Placement(f)) if f == "VISION.md"
        ));
        assert!(matches!(
            home_of(&adr_schema("docs/decisions/")),
            Some(Home::Location(d)) if d == "docs/decisions"
        ));
        // A transient (location-less, placement-less) schema has no home.
        let transient = engine::schema::load_schema(
            b"type: commit\nsections:\n  - id: body\n    slot: { hint: x }\n",
        )
        .expect("transient schema loads");
        assert!(home_of(&transient).is_none());
    }

    /// (M39 inc-5 T2) The recorded-prior-home discriminator detects a **placement/root**
    /// strand: a `vision` doctype relocated from `docs/vision.md` to root `VISION.md` leaves
    /// the pre-existing `docs/vision.md` stranded, while the instance already at the current
    /// home is not — a relocation the old location-dir-basename key could not see.
    #[test]
    fn is_stranded_detects_a_placement_root_strand_given_a_prior_home() {
        let prior = Home::placement("docs/vision.md");
        let current = Home::placement("VISION.md");
        assert!(
            is_stranded("docs/vision.md", &prior, &current),
            "the instance left at the prior placement home is stranded",
        );
        assert!(
            !is_stranded("VISION.md", &prior, &current),
            "the instance at the current placement home is not stranded",
        );
    }

    /// (M39 inc-5 T2) The discriminator detects a **`location` basename-rename** strand: a
    /// doctype whose home dir was renamed `docs/decisions/` → `docs/adrs/` leaves docs under
    /// the old dir stranded, while a doc under the new dir is not.
    #[test]
    fn is_stranded_detects_a_location_basename_rename_strand() {
        let prior = Home::location("docs/decisions/").expect("prior home");
        let current = Home::location("docs/adrs/").expect("current home");
        assert!(
            is_stranded("docs/decisions/cache.md", &prior, &current),
            "a doc under the renamed-away dir is stranded",
        );
        assert!(
            !is_stranded("docs/adrs/live.md", &prior, &current),
            "a doc under the current dir is not stranded",
        );
    }

    /// (M39 inc-5 T2) The discriminator also spans a **shape change** — `location` → root
    /// `placement` (the freeze-exempt relocation kind): a doc at the old `location` dir is
    /// stranded once the home becomes a root file; the root file itself is not.
    #[test]
    fn is_stranded_detects_a_location_to_placement_relocation() {
        let prior = Home::location("decisions/").expect("prior home");
        let current = Home::placement("DECISIONS.md");
        assert!(is_stranded("decisions/cache.md", &prior, &current));
        assert!(!is_stranded("DECISIONS.md", &prior, &current));
    }
}
