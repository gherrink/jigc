//! The **closure caveat fence** (M46 inc-7, T2): a step that solicits a write to a
//! field the pack declares under the `criterion-maps-to-test` check must state the
//! two facts that make a *correct* anchor authorable — that a **closure**-based test
//! framework registers its tests with no named symbol, and that the **file-only**
//! anchor (`<path>` with no `#<test-fn>`) is accepted.
//!
//! M45 landed that rider in `locate-from-spec` and stopped there, while
//! `author-migration-spec` went on soliciting `<path>#<test-fn>` and telling the agent
//! to *"carry over verbatim"* — one of the two soliciting steps swept, which is the
//! incomplete-fix shape this wave exists to end. Pinning the second step by name would
//! repeat the mistake one step later, so this suite pins the **rule**: the owe-set is
//! **derived from the shipped pack**, never hand-listed —
//!
//!   - the checked-field set is every schema field whose *resolved* predicate is
//!     `criterion-maps-to-test`, read through the production selector
//!     (`engine::target_surface::resolve_check_id`, i.e. `field.check ??` the
//!     pack-declared type's `check`) over the tree's own schemas;
//!   - a step **solicits a write** to such a field when its body names the field id
//!     *and* carries a write signal: a literal `jigc doc <write-verb>` line naming the
//!     field (the write-verb partition is the production one, `cli::doc::doc_write_verbs`),
//!     or a `{{schema:<T>}}` authoring-payload projection of the field's own doctype
//!     (the migrate author templates solicit their whole write that way and carry no
//!     literal command line at all).
//!
//! Both arms are load-bearing and each is proven to reach a member the other does not.
//! `arch-doc`'s `implemented-by` anchor is excluded **at the predicate** — its resolved
//! check is `symbol-exists`, so its two soliciting steps never enter the owe-set — and
//! that exclusion is asserted here rather than assumed, because an exclusion by hand is
//! the hand-list wearing a different hat.
//!
//! The fence's own teeth are then driven on the `read_back_fence.rs` mould: the caveat
//! is withdrawn from a **copied** step tree, member by member over the derived owe-set,
//! and the fence must name exactly that member. Without that arm the fence would be
//! green forever after the shipped text is fixed, with nothing showing it can fail.
//!
//! Declared bound: this tier reads the shipped **bytes**, so a step that solicits the
//! write only through the projection *without naming the field* carries no signal here
//! and is outside the derived set — the same bound-by-structural-signal the read-back
//! fence records for its own literal-command case.

use cli::pack::FilesystemPack;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::schema::{Field, Leaf, Schema, SectionBody};
use engine::target_surface::{is_code_anchor, resolve_check_id};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// The check the caveat is owed for — the predicate `spec.criteria/maps-to-test`
/// declares, and the only one whose resolution accepts a bare `<path>`.
const CHECK_ID: &str = "criterion-maps-to-test";

/// The two facts a soliciting step owes, as the tokens that carry them. The same
/// two words `spec_loop_prompts.rs` asserts on the **composed** output, so the fence
/// and the composed-surface pin state one rule.
const CAVEAT_TOKENS: [&str; 2] = ["closure", "file-only"];

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-caveat-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The embedded dev pack tree.
fn dev_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Every shipped pack tree — both constituents of the composed `[dev ▸ methodology]`
/// pair, so a checked field declared in either pack and a step soliciting it from
/// either pack are both reached.
fn shipped_pack_trees() -> Vec<PathBuf> {
    vec![dev_pack_tree(), methodology_pack_tree()]
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

// ---------------------------------------------------------------------------
// The derivation — recomputed from the trees' own bytes through the production
// check selector and the production write-verb partition.
// ---------------------------------------------------------------------------

/// Every field of a schema, in declaration order, descending into repeatable item
/// blocks (and their nested repeatables) so an item-level anchor is reached.
fn schema_fields(schema: &Schema) -> Vec<&Field> {
    fn walk<'a>(leaves: &'a [Leaf], out: &mut Vec<&'a Field>) {
        for leaf in leaves {
            match leaf {
                Leaf::Field(field) => out.push(field),
                Leaf::Repeatable { repeatable, .. } => walk(&repeatable.block, out),
                Leaf::Slot { .. } => {}
            }
        }
    }
    let mut out = Vec::new();
    for section in &schema.sections {
        match &section.body {
            SectionBody::Simple { fields, .. } => out.extend(fields.iter()),
            SectionBody::Repeatable { repeatable } => walk(&repeatable.block, &mut out),
        }
    }
    out
}

/// The tree's `(doctype, field)` pairs whose **resolved** predicate is [`CHECK_ID`],
/// read through the production selector so a field inheriting its type's check and a
/// field overriding it are adjudicated exactly as the probe seam adjudicates them.
fn checked_fields(tree: &Path) -> BTreeSet<(String, String)> {
    let pack = FilesystemPack::new(tree.to_path_buf());
    let mut out = BTreeSet::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .expect("the pack serves the schema it lists");
        let schema = cli::pack::load_pack_schema(&pack, &bytes).expect("the schema parses");
        for field in schema_fields(&schema) {
            if is_code_anchor(&field.ty) && resolve_check_id(field) == CHECK_ID {
                out.insert((schema.ty.clone(), field.id.clone()));
            }
        }
    }
    out
}

/// The tree's step bodies, keyed by step id and sorted.
fn step_bodies(tree: &Path) -> Vec<(String, String)> {
    let pack = FilesystemPack::new(tree.to_path_buf());
    let mut ids = pack.list(PackResourceKind::Steps);
    ids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    ids.into_iter()
        .map(|id| {
            let bytes = pack
                .read(PackResourceKind::Steps, &id)
                .expect("the pack serves the step it lists");
            let def = engine::compose::load_step_def(id.as_str(), &bytes)
                .expect("step front-matter parses");
            (id.as_str().to_owned(), def.body)
        })
        .collect()
}

/// The inner text of every `{{ … }}` placeholder in a step body.
fn placeholder_refs(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(open) = rest.find("{{") {
        rest = &rest[open + 2..];
        let Some(close) = rest.find("}}") else { break };
        out.push(rest[..close].trim().to_owned());
        rest = &rest[close + 2..];
    }
    out
}

/// Arm 1 — the body carries a literal `jigc doc <write-verb>` line naming the field.
fn solicits_by_command(body: &str, field: &str) -> bool {
    let write_verbs = cli::doc::doc_write_verbs();
    body.lines().any(|line| {
        let line = line.trim().trim_start_matches('`');
        let Some(rest) = line.strip_prefix("jigc doc ") else {
            return false;
        };
        let verb = rest.split_whitespace().next().unwrap_or_default();
        write_verbs.iter().any(|write| write == verb) && line.contains(field)
    })
}

/// Arm 2 — the body projects the field's own doctype as an authoring payload.
fn solicits_by_schema_projection(body: &str, doctype: &str) -> bool {
    placeholder_refs(body)
        .iter()
        .filter_map(|inner| inner.strip_prefix("schema:"))
        .any(|ty| ty.trim() == doctype)
}

/// The two arms of the derivation over all shipped trees, kept apart so the union
/// can be shown to be a real union.
fn owe_set_arms(trees: &[PathBuf]) -> (BTreeSet<String>, BTreeSet<String>) {
    let fields: BTreeSet<(String, String)> = trees.iter().flat_map(|t| checked_fields(t)).collect();
    let mut by_command = BTreeSet::new();
    let mut by_projection = BTreeSet::new();
    for tree in trees {
        for (id, body) in step_bodies(tree) {
            for (doctype, field) in &fields {
                if !body.contains(field.as_str()) {
                    continue;
                }
                if solicits_by_command(&body, field) {
                    by_command.insert(id.clone());
                }
                if solicits_by_schema_projection(&body, doctype) {
                    by_projection.insert(id.clone());
                }
            }
        }
    }
    (by_command, by_projection)
}

/// The owe-set: every step of the shipped trees that solicits a write to a
/// `criterion-maps-to-test` field.
fn owe_set(trees: &[PathBuf]) -> BTreeSet<String> {
    let (by_command, by_projection) = owe_set_arms(trees);
    by_command.union(&by_projection).cloned().collect()
}

/// The fence itself: every owed step, with the caveat tokens it fails to state.
/// Empty over a conforming tree set.
fn caveat_violations(trees: &[PathBuf]) -> Vec<(String, Vec<&'static str>)> {
    let owed = owe_set(trees);
    let mut out = Vec::new();
    for tree in trees {
        for (id, body) in step_bodies(tree) {
            if !owed.contains(&id) {
                continue;
            }
            let lowered = body.to_lowercase();
            let missing: Vec<&'static str> = CAVEAT_TOKENS
                .into_iter()
                .filter(|token| !lowered.contains(token))
                .collect();
            if !missing.is_empty() {
                out.push((id, missing));
            }
        }
    }
    out
}

/// Which shipped tree ships a given step.
fn owning_tree(step: &str) -> PathBuf {
    shipped_pack_trees()
        .into_iter()
        .find(|tree| tree.join("steps").join(format!("{step}.yaml")).is_file())
        .unwrap_or_else(|| panic!("a shipped tree must ship the `{step}` step"))
}

/// Withdraw the caveat from a copied step: drop every line stating either token,
/// leaving the rest of the solicit standing — the mutation the fence must catch,
/// and narrower than emptying the body.
fn withdraw_caveat(tree: &Path, step: &str) {
    let path = tree.join("steps").join(format!("{step}.yaml"));
    let text = fs::read_to_string(&path).expect("read the copied step");
    let kept: Vec<&str> = text
        .lines()
        .filter(|line| {
            let lowered = line.to_lowercase();
            !CAVEAT_TOKENS
                .into_iter()
                .any(|token| lowered.contains(token))
        })
        .collect();
    assert!(
        kept.len() < text.lines().count(),
        "the shipped `{step}` step must state the caveat for its withdrawal to be a real \
         mutation",
    );
    fs::write(&path, format!("{}\n", kept.join("\n"))).expect("write the mutated step");
}

// ---------------------------------------------------------------------------
// The fence
// ---------------------------------------------------------------------------

/// The derivation: the owe-set is non-empty, is exactly the two soliciting steps at
/// HEAD, and is a genuine **union** — each arm reaches a member the other does not,
/// so neither can be dropped without silently shrinking the set.
#[test]
fn the_owe_set_derives_to_exactly_the_two_soliciting_steps() {
    let trees = shipped_pack_trees();
    let (by_command, by_projection) = owe_set_arms(&trees);
    let owed = owe_set(&trees);

    assert!(
        !owed.is_empty(),
        "the shipped packs must solicit at least one `{CHECK_ID}` write for the fence to \
         have a subject",
    );
    assert!(
        by_command.difference(&by_projection).next().is_some(),
        "the literal-command arm must reach a step the projection arm does not, or the union \
         is not a union; by-command {by_command:?}, by-projection {by_projection:?}",
    );
    assert!(
        by_projection.difference(&by_command).next().is_some(),
        "the projection arm must reach a step the command arm does not; by-command \
         {by_command:?}, by-projection {by_projection:?}",
    );
    assert_eq!(
        owed,
        BTreeSet::from([
            "author-migration-spec".to_owned(),
            "locate-from-spec".to_owned(),
        ]),
        "the derived owe-set must be exactly the shipped steps soliciting a `{CHECK_ID}` \
         write — a third one joins by construction, and must then state the caveat",
    );
}

/// The exclusion is at the **predicate**, not by name: `arch-doc`'s `implemented-by`
/// is a `code-anchor` too, and its two soliciting steps are outside the owe-set only
/// because the field resolves to `symbol-exists` — the check whose bare-path degrade
/// buys nothing, so no caveat is owed there.
#[test]
fn a_symbol_exists_anchor_is_excluded_at_the_check_not_by_step_name() {
    let dev = dev_pack_tree();
    let pack = FilesystemPack::new(dev.clone());
    let bytes = pack
        .read(PackResourceKind::Schemas, &ResourceId::from("arch-doc"))
        .expect("the dev pack ships the arch-doc schema");
    let schema = cli::pack::load_pack_schema(&pack, &bytes).expect("the schema parses");
    let anchor = schema_fields(&schema)
        .into_iter()
        .find(|field| field.id == "implemented-by")
        .expect("arch-doc carries the `implemented-by` anchor");

    assert!(
        is_code_anchor(&anchor.ty),
        "`implemented-by` must be a code-anchor, or the exclusion proves nothing",
    );
    assert_ne!(
        resolve_check_id(anchor),
        CHECK_ID,
        "`implemented-by` must resolve to a different check — that resolution, not a name \
         filter, is what keeps its soliciting steps out of the owe-set",
    );

    let owed = owe_set(&shipped_pack_trees());
    for step in ["author-arch-doc", "author-migration-arch-doc"] {
        assert!(
            !owed.contains(step),
            "`{step}` solicits `implemented-by` under a different check and must stay out of \
             the owe-set; got {owed:?}",
        );
    }
}

/// The fence over the shipped packs: every step that solicits a `criterion-maps-to-test`
/// write states the closure case **and** the file-only fallback.
#[test]
fn every_soliciting_step_states_the_closure_and_file_only_caveat() {
    let violations = caveat_violations(&shipped_pack_trees());
    assert!(
        violations.is_empty(),
        "a step soliciting a `{CHECK_ID}` anchor must state that a closure-based framework \
         names no symbol and that a file-only anchor is accepted; missing: {violations:?}",
    );
}

/// The mutation arm, on the `read_back_fence.rs` mould: withdrawing the caveat from a
/// **copied** tree, member by member over the derived owe-set, makes the fence name
/// that member and only that member — so the fence's silence over the shipped packs is
/// a fact about them, not about a fence with no teeth.
#[test]
fn withdrawing_the_caveat_from_a_copied_tree_reddens_the_fence() {
    for step in owe_set(&shipped_pack_trees()) {
        let source = owning_tree(&step);
        let copy = TempDir::new(&step);
        copy_tree(&source, copy.path());
        withdraw_caveat(copy.path(), &step);

        let violations = caveat_violations(&[copy.path().to_path_buf()]);
        let named: Vec<&str> = violations.iter().map(|(id, _)| id.as_str()).collect();
        assert_eq!(
            named,
            vec![step.as_str()],
            "withdrawing `{step}`'s caveat must redden the fence for `{step}` alone; got \
             {violations:?}",
        );
        assert_eq!(
            violations[0].1,
            CAVEAT_TOKENS.to_vec(),
            "the fence must name both withdrawn facts; got {:?}",
            violations[0].1,
        );
    }
}
