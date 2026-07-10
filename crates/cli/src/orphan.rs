//! Orphan detection — committed managed-looking docs **stranded** at a doctype's prior home
//! after its home moved (M36 same-shape `docs-root` re-point; M39 the general relocation
//! floor). `design/validation.md` → Orphan detection; `design/storage.md` → docs-root.
//!
//! **The strand discriminator (M39 re-key).** A doc is stranded when it sits at a doctype's
//! **prior** home yet no longer at its **current** home — [`is_stranded`] over the
//! generalized [`Home`] vocabulary (`location` dir **or** `placement` file), which the two
//! arms share:
//!
//! - **found-stranded** — the prior home is *self-discovered* from the doc's own dir, keyed
//!   on a `location:` basename match. Backs the store-scope `file-state.orphaned-doc`
//!   advisory ([`orphaned_docs`], `jigc validate`). Here the location-dir basename is the
//!   *only* signal available (no prior-home record in hand), so a placement/root strand — a
//!   file with no dir pattern — is **not** self-discoverable; only the recorded arm sees it.
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
    let Ok(listing) = crate::task::git_capture(repo_root, &["ls-files"]) else {
        return Vec::new();
    };
    let mut out: Vec<String> = listing
        .lines()
        .filter(|l| l.ends_with(".md"))
        .map(str::to_string)
        .collect();
    out.sort();
    out
}

/// The committed `.md`s that **look like** managed docs (their immediate-parent dir
/// basename matches some persisted doctype's resolved `location:` basename) yet fall
/// **outside** that doctype's current resolved root — the `docs-root`-changed orphans.
/// Each hit carries the **matched doctype** (`schema.ty`) beside the path, the handle the
/// M40 two-tier route needs to name `jigc migrate <path> --as <doctype>` on the
/// unregistered tier ([`unregistered_route`]). `schemas` must carry the
/// `docs-root`-applied `location:` (the resolved roots). Address-sorted.
pub(crate) fn orphaned_docs<'a>(
    repo_root: &Path,
    schemas: impl IntoIterator<Item = &'a Schema>,
) -> Vec<(String, String)> {
    let schemas: Vec<&Schema> = schemas.into_iter().collect();
    let mut hits = Vec::new();
    for rel in committed_markdown(repo_root) {
        let Some(base) = parent_basename(&rel) else {
            continue; // a root-level file — never a `location`-homed managed doc.
        };
        // The doctype whose resolved location basename matches — at most one (basenames
        // are unique per doctype). No match → ordinary prose, never flagged.
        let Some(schema) = schemas.iter().find(|s| location_basename(s) == Some(base)) else {
            continue;
        };
        let (Some(current), Some(parent)) = (home_of(schema), parent_dir(&rel)) else {
            continue; // a location match always has both — defensive.
        };
        // Found-stranded self-discovery: the doc's own parent dir *looks like* a home (its
        // basename matched), so it is the self-discovered prior home; stranded iff the doc is
        // no longer at the doctype's CURRENT home (directly under the current resolved root).
        if is_stranded(&rel, &Home::Location(parent.to_string()), &current) {
            let ty = schema.ty.clone();
            hits.push((rel, ty));
        }
    }
    hits
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
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
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

    /// (M38 inc-2 T3) Characterization — orphan detection keys on the immediate-parent
    /// **directory basename** matching a doctype's `location:` basename, so a **placement**
    /// doctype's root literal (`VISION.md`) is **never mis-orphaned**: a root file has no
    /// parent dir to match, and a placement doctype carries no `location` basename to match
    /// against (`design/storage.md` → Placement — census verification `orphan`). A sibling
    /// root `.md` (`README.md`) is likewise **not** the placement doctype's instance. A live
    /// `decisions/live.md` under its current root proves the sweep actually enumerates and
    /// stays quiet only where it should.
    #[test]
    fn placement_root_file_and_sibling_root_md_are_never_orphaned() {
        let repo = TempRepo::new();
        repo.commit_file("VISION.md", "# Vision\n");
        repo.commit_file("README.md", "# Readme\n");
        repo.commit_file("decisions/live.md", "# A decision\n");

        let schemas = vec![
            placement_schema("vision", "VISION.md"),
            adr_schema("decisions/"),
        ];
        assert!(
            orphaned_docs(repo.path(), &schemas).is_empty(),
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
        let schemas = vec![adr_schema("docs/decisions/")];
        assert_eq!(
            orphaned_docs(repo.path(), &schemas),
            vec![("old/decisions/cache.md".to_string(), "adr".to_string())],
            "a stranded hit names both the path and the doctype whose basename matched",
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
