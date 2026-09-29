//! Acceptance — **the snapshot store's first two-snapshot doctype** (M49 Increment 9, T4;
//! `design/corpus-migration.md` → Prior-schema sourcing — the versioned snapshot store, and
//! → Snapshot hashing + `jigc upgrade`; `implementation/decisions-pending.md` → Prior-schema
//! snapshot integrity + `jigc upgrade`).
//!
//! `jigc migrate-corpus` sources a below-version doc's `from` **at that doc's own stamp** —
//! `load_prior_schema(ty, k)` over `schema-snapshots/<ty>.v<k>.yaml`. The mechanism has been
//! general in *shape* since M34, but until this wave **no doctype had ever shipped snapshots
//! at two versions**: every prior chain was one hop, so `k` had exactly one legal value and
//! "keyed by the stamp" and "the only snapshot there is" were indistinguishable. M49 bumps
//! `milestone-record` 2 → 3, so the store now holds `.v1.yaml` **beside** `.v2.yaml` and the
//! selection is real. This suite is where that is driven rather than reasoned.
//!
//! **What the two stamped arms can and cannot witness — stated up front, because the
//! deferral this suite re-adjudicates rests on it.** Every link of this doctype's chain is a
//! *byte no-op* on the corpus (v1 → v2 widened an enum; v2 → v3 added a `default`-less,
//! machine-maintained item leaf), so a v1-stamped record and a v2-stamped record with the
//! same body migrate to **byte-identical** results — which is asserted here, deliberately.
//! The corollary is the honest one: on a byte-neutral chain the migration's *outcome cannot
//! discriminate which snapshot sourced it*, so the fence against sourcing the wrong one is
//! not the fold's bytes but (a) the store being **complete** — every version below current
//! has a snapshot, so no stamp resolves to nothing — and (b) the snapshots being **distinct
//! and each the shape its own version declares**. Both are asserted below, over the real
//! embedded packs. That is the driven basis the ledger entry's re-disposition cites.
//!
//! The two migration arms drive the **shipped binary** against a throwaway
//! `[dev ▸ methodology]` repo — the real embedded packs, the real freeze manifest, the real
//! snapshot store — and assert the **bytes on disk** and the **emitted JSON**, never a
//! reconstruction. The third read site of the store, the placement **prior-home union**
//! (`candidate_docs`), is not reachable for this doctype and is not faked here:
//! `milestone-record` declares a `location:`, so the walk takes the directory branch and
//! reads no snapshot at all.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cli::pack::{EmbeddedPack, load_prior_schema};
use engine::schema::{Leaf, Schema, SectionBody};

// ---------------------------------------------------------------------------------------------
// The shipped packs, read the way production reads them.
// ---------------------------------------------------------------------------------------------

/// The two shipped pack trees, each paired with the **embedded** pack built from it: the
/// disk tree is where the snapshot *files* are enumerated, the embedded pack is what
/// `migrate-corpus` actually reads. Asserting over both is the point — a snapshot present
/// on disk but absent from the binary is exactly the failure a user would hit.
fn shipped_packs() -> Vec<(&'static str, PathBuf, EmbeddedPack)> {
    vec![
        (
            "dev",
            Path::new(cli::pack_path!(dev)).to_path_buf(),
            EmbeddedPack::new(),
        ),
        (
            "methodology",
            Path::new(cli::pack_path!(methodology)).to_path_buf(),
            EmbeddedPack::methodology(),
        ),
    ]
}

/// A pack tree's shipped freeze manifest.
fn manifest_of(tree: &Path) -> engine::manifest::Manifest {
    let bytes = fs::read(tree.join("config").join("schema-manifest.yaml"))
        .unwrap_or_else(|e| panic!("read the shipped manifest at {tree:?}: {e}"));
    serde_yaml_ng::from_slice(&bytes)
        .unwrap_or_else(|e| panic!("the manifest at {tree:?} deserializes: {e}"))
}

/// The `schema-version` the shipped **methodology** manifest declares for `ty` — read, never
/// spelled, so the next bump of some other doctype does not redden arms that are not about it.
fn declared_schema_version(ty: &str) -> u32 {
    let tree = Path::new(cli::pack_path!(methodology));
    manifest_of(tree)
        .doctypes
        .iter()
        .find(|entry| entry.ty == ty)
        .unwrap_or_else(|| panic!("the methodology manifest declares `{ty}`"))
        .schema_version
}

/// The `tasks` item block's leaf ids, in declared order.
fn item_leaf_ids(schema: &Schema, section: &str) -> Vec<String> {
    let sec = schema
        .sections
        .iter()
        .find(|s| s.id == section)
        .unwrap_or_else(|| panic!("the schema declares the `{section}` section"));
    match &sec.body {
        SectionBody::Repeatable { repeatable } => repeatable
            .block
            .iter()
            .map(|leaf| match leaf {
                Leaf::Field(field) => field.id.clone(),
                Leaf::Slot { id, .. } | Leaf::Repeatable { id, .. } => id.clone(),
            })
            .collect(),
        SectionBody::Simple { .. } => {
            panic!("`{section}` is a repeatable section in every shipped version")
        }
    }
}

/// The `of:` members of an `enum` leaf on a repeatable section's item block.
fn item_enum_members(schema: &Schema, section: &str, leaf_id: &str) -> Vec<String> {
    let sec = schema
        .sections
        .iter()
        .find(|s| s.id == section)
        .unwrap_or_else(|| panic!("the schema declares the `{section}` section"));
    let SectionBody::Repeatable { repeatable } = &sec.body else {
        panic!("`{section}` is a repeatable section in every shipped version")
    };
    repeatable
        .block
        .iter()
        .find_map(|leaf| match leaf {
            Leaf::Field(field) if field.id == leaf_id => field.of.clone(),
            _ => None,
        })
        .unwrap_or_else(|| panic!("`{section}`'s item block declares the `{leaf_id}` enum"))
}

/// The `of:` members of an `enum` field on a header (or any simple) section.
fn header_enum_members(schema: &Schema, section: &str, field_id: &str) -> Vec<String> {
    let sec = schema
        .sections
        .iter()
        .find(|s| s.id == section)
        .unwrap_or_else(|| panic!("the schema declares the `{section}` section"));
    let SectionBody::Simple { fields, .. } = &sec.body else {
        panic!("`{section}` is a simple section in every shipped version")
    };
    fields
        .iter()
        .find(|field| field.id == field_id)
        .unwrap_or_else(|| panic!("`{section}` declares the `{field_id}` field"))
        .of
        .clone()
        .unwrap_or_else(|| panic!("`{section}.{field_id}` is an enum"))
}

// ---------------------------------------------------------------------------------------------
// (1) The store is COMPLETE — every stamp a committed doc can carry resolves to a snapshot.
// ---------------------------------------------------------------------------------------------

/// **The snapshot store holds exactly `1..current` for every frozen doctype, in both shipped
/// packs, and every one of them resolves out of the embedded binary.**
///
/// This is the fence that makes stamp-keyed sourcing *total*. `migrate-corpus` blocks a doc
/// whose stamp `k` has no `schema-snapshots/<ty>.v<k>.yaml` (`missing-snapshot`, deliberately
/// never a silent `already-current`), so a bump that ships its version but forgets its
/// snapshot strands every corpus still at an intermediate version — and until this wave no
/// doctype had an *intermediate* version to forget. The check iterates the manifests rather
/// than a hand-list, so the next doctype to bump is covered by construction.
///
/// Both directions are asserted: no **missing** snapshot below the current version, and no
/// **surplus** one at or above it (a snapshot at `current` would claim the present as a prior
/// shape, and `load_prior_schema` is only ever asked for `k < current`).
#[test]
fn every_version_below_current_ships_a_snapshot_and_nothing_above_it_does() {
    for (pack_name, tree, embedded) in shipped_packs() {
        let manifest = manifest_of(&tree);

        // What the manifest OWES: `<ty>.v<k>` for every `1 <= k < current`.
        let owed: BTreeSet<String> = manifest
            .doctypes
            .iter()
            .flat_map(|entry| {
                let ty = entry.ty.clone();
                (1..entry.schema_version).map(move |k| format!("{ty}.v{k}"))
            })
            .collect();

        // What the pack tree SHIPS.
        let dir = tree.join("schema-snapshots");
        let shipped: BTreeSet<String> = match fs::read_dir(&dir) {
            Ok(entries) => entries
                .map(|e| e.expect("read a snapshot dir entry").file_name())
                .filter_map(|name| {
                    name.to_str()
                        .and_then(|n| n.strip_suffix(".yaml"))
                        .map(str::to_owned)
                })
                .collect(),
            Err(_) => BTreeSet::new(), // a pack that has never bumped ships no store at all
        };

        assert_eq!(
            shipped, owed,
            "the `{pack_name}` pack's snapshot store must hold exactly one snapshot per \
             version below each doctype's current one — a MISSING one blocks every committed \
             doc still at that stamp with `migrate-corpus.missing-snapshot`, and a SURPLUS \
             one at or above current claims the present as a prior shape",
        );

        // And every owed snapshot resolves out of the EMBEDDED pack — what production reads.
        for id in &owed {
            let (ty, version) = id.rsplit_once(".v").expect("a `<ty>.v<k>` snapshot id");
            let version: u32 = version.parse().expect("a numeric snapshot version");
            load_prior_schema(&embedded, ty, version).unwrap_or_else(|e| {
                panic!("the embedded `{pack_name}` pack resolves the `{id}` snapshot: {e}")
            });
        }
    }
}

// ---------------------------------------------------------------------------------------------
// (2) The two snapshots are DISTINCT, and each is the shape its own version declares.
// ---------------------------------------------------------------------------------------------

/// **`load_prior_schema(ty, k)` is keyed on `k`, and for the first time `k` has two legal
/// values.** The two `milestone-record` snapshots are read out of the embedded methodology
/// pack and each is checked against what its version actually declared:
///
///   * **v1** — the pre-`discarded` lifecycle: `status` closed at `[active, joined]` at
///     **both** loci, and no `workflow` leaf on the `tasks` item block;
///   * **v2** — `discarded` admitted at both loci, still no `workflow` leaf.
///
/// Neither substitutes for the other, and on this doctype's byte-neutral chain the *fold*
/// cannot tell them apart (arm 3) — so this is the assertion that the store is keyed at all.
#[test]
fn the_two_milestone_record_snapshots_are_distinct_shapes_keyed_by_version() {
    let pack = EmbeddedPack::methodology();
    let current = declared_schema_version("milestone-record");
    assert!(
        current >= 3,
        "this arm is about a doctype with TWO prior snapshots; `milestone-record` is at \
         {current}"
    );

    let v1 = load_prior_schema(&pack, "milestone-record", 1).expect("the `.v1.yaml` snapshot");
    let v2 = load_prior_schema(&pack, "milestone-record", 2).expect("the `.v2.yaml` snapshot");
    assert_ne!(
        v1, v2,
        "two snapshots that compare equal would make the store's keying unobservable"
    );

    // v1 — the pre-`discarded` enum, at BOTH loci.
    assert_eq!(
        header_enum_members(&v1, "meta", "status"),
        vec!["active".to_string(), "joined".to_string()],
        "the v1 snapshot is the pre-`discarded` header lifecycle",
    );
    assert_eq!(
        item_enum_members(&v1, "tasks", "status"),
        vec!["active".to_string(), "joined".to_string()],
        "the v1 snapshot is the pre-`discarded` item lifecycle",
    );

    // v2 — `discarded` admitted at both loci (the M42 widening), and NOT the v1 shape.
    assert_eq!(
        header_enum_members(&v2, "meta", "status"),
        vec![
            "active".to_string(),
            "joined".to_string(),
            "discarded".to_string()
        ],
        "the v2 snapshot carries the widened header lifecycle",
    );
    assert_eq!(
        item_enum_members(&v2, "tasks", "status"),
        vec![
            "active".to_string(),
            "joined".to_string(),
            "discarded".to_string()
        ],
        "the v2 snapshot carries the widened item lifecycle",
    );

    // Neither prior shape carries the M49 leaf — that is what makes each of them a PRIOR
    // shape of the current one.
    for (label, prior) in [("v1", &v1), ("v2", &v2)] {
        let leaves = item_leaf_ids(prior, "tasks");
        assert!(
            !leaves.contains(&"workflow".to_string()),
            "the {label} snapshot predates the `workflow` leaf; got {leaves:?}",
        );
    }
}

// ---------------------------------------------------------------------------------------------
// (3) The headline — a v1-stamped and a v2-stamped record, one corpus, ONE pass.
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-snapshot-store-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A real git repo with one commit and the `[dev ▸ methodology]` compose marker — the exact
/// project-layer key that composes the embedded methodology pack over the dev pack, so the
/// `milestone-record` under test is the **shipped** one.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation exited 0, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

const RECORDS: &str = "docs/milestone-records";

/// Mint a milestone and one sub-task through the **real** verbs and return the committed
/// record's bytes — never a hand-written fixture, so the shape under migration is the shape
/// the binary actually writes.
fn minted_record(repo: &Path, home: &Path) -> String {
    assert_ok(
        &jigc(repo, home, &["milestone", "create", "Cache rework"]),
        "jigc milestone create",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &["milestone", "add-task", "cache-rework", "Zebra fix"],
        ),
        "jigc milestone add-task",
    );
    fs::read_to_string(repo.join(RECORDS).join("cache-rework.md")).expect("read the record")
}

/// The current mint wound back to the byte form a record committed **at version `to`** holds:
/// the `workflow` bullet the v3 mint writes removed (no prior version had the leaf) and the
/// stamp moved to `to`.
fn wind_back(at_current: &str, current: u32, to: u32) -> String {
    let stripped: String = at_current
        .lines()
        .filter(|line| !line.starts_with("- workflow: "))
        .map(|line| format!("{line}\n"))
        .collect();
    assert_ne!(
        stripped, at_current,
        "the mint must carry the `workflow` bullet for the wind-back to have anything to strip"
    );
    let wound = stripped.replace(
        &format!("schema-version: {current}"),
        &format!("schema-version: {to}"),
    );
    assert_ne!(
        wound, stripped,
        "the mint must carry a `schema-version: {current}` stamp to wind back"
    );
    wound
}

/// Parse a `migrate-corpus --format json` report's `migrated` / `already_current` / `blocked`
/// path lists.
fn triage(report: &serde_json::Value, key: &str) -> Vec<String> {
    report[key]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `{key}[]`; got:\n{report:#}"))
        .iter()
        .map(|v| match v {
            serde_json::Value::String(s) => s.clone(),
            // `blocked` carries findings, not paths — surface the whole entry.
            other => other.to_string(),
        })
        .collect()
}

/// **The two stamped arms, in one corpus and ONE pass.** A committed `schema-version: 1`
/// record and a committed `schema-version: 2` record — same body, so the only difference the
/// verb sees is the stamp — both migrate to the current version in a single
/// `jigc migrate-corpus`, each byte-identical except its own stamp.
///
/// That is the whole claim of a two-snapshot store: the v1 record sources `.v1.yaml` and folds
/// the **whole remaining chain** in one hop (the added `.v2.yaml` neither intercepts it nor
/// strands it), while the v2 record sources its own `.v2.yaml`. Neither is blocked
/// (`missing-snapshot` never fires), neither is reported `already-current`, and the re-run is
/// idempotent.
///
/// The final assertion is the one the deferral ledger cites: with the slug normalized the two
/// migrated records are **byte-identical**, so on this doctype's byte-neutral chain the fold's
/// output does not reveal which snapshot sourced it. If a future link on this chain is *not*
/// byte-neutral this assertion reddens — which is the signal to re-open the snapshot-integrity
/// entry, not a licence to weaken it.
///
/// RED before `schema-snapshots/milestone-record.v2.yaml` shipped: the v2 record was blocked
/// `migrate-corpus.missing-snapshot`, `migrated[]` held only the v1 record, and the pass
/// exited non-zero.
#[test]
fn a_v1_and_a_v2_stamped_record_both_migrate_to_current_in_one_pass() {
    let repo = TempDir::new("one-pass");
    let home = TempDir::new("one-pass-home");
    init_repo(repo.path());
    let current = declared_schema_version("milestone-record");

    // Two committed records with the SAME body, differing only in slug and stamp: one at v1
    // (the pre-`discarded` era) and one at v2 (the pre-`workflow` era).
    let at_current = minted_record(repo.path(), home.path());
    let at_v1 = wind_back(&at_current, current, 1);
    let at_v2 = wind_back(&at_current, current, 2).replace("cache-rework", "queue-rework");
    fs::write(repo.path().join(RECORDS).join("cache-rework.md"), &at_v1)
        .expect("seed the v1 record");
    fs::write(repo.path().join(RECORDS).join("queue-rework.md"), &at_v2)
        .expect("seed the v2 record");
    git(repo.path(), &["add", RECORDS]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "seed a v1 and a v2 record"],
    );

    // ONE pass.
    let out = jigc(
        repo.path(),
        home.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let stdout = assert_ok(
        &out,
        "jigc migrate-corpus over a v1 AND a v2 milestone-record",
    );
    let report: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the report is JSON ({e}); stdout:\n{stdout}"));
    assert_eq!(
        triage(&report, "migrated"),
        vec![
            format!("{RECORDS}/cache-rework.md"),
            format!("{RECORDS}/queue-rework.md"),
        ],
        "both stamped records migrate in the SAME pass; report:\n{report:#}",
    );
    assert_eq!(
        triage(&report, "blocked"),
        Vec::<String>::new(),
        "every stamp below current resolves a snapshot — nothing is blocked; report:\n{report:#}",
    );
    assert_eq!(
        triage(&report, "already_current"),
        Vec::<String>::new(),
        "neither record was already current; report:\n{report:#}",
    );

    // Each record: its own stamp, and nothing else, moved.
    let after_v1 =
        fs::read_to_string(repo.path().join(RECORDS).join("cache-rework.md")).expect("read v1");
    let after_v2 =
        fs::read_to_string(repo.path().join(RECORDS).join("queue-rework.md")).expect("read v2");
    assert_eq!(
        after_v1,
        at_v1.replace("schema-version: 1", &format!("schema-version: {current}")),
        "the v1 record folds the WHOLE remaining chain in one pass, with the stamp 1 → \
         {current} as its only byte delta",
    );
    assert_eq!(
        after_v2,
        at_v2.replace("schema-version: 2", &format!("schema-version: {current}")),
        "the v2 record sources its own snapshot, with the stamp 2 → {current} as its only \
         byte delta",
    );

    // The ledger's basis: two different `from` snapshots, one identical result.
    assert_eq!(
        after_v1.replace("cache-rework", "queue-rework"),
        after_v2,
        "every link of this doctype's chain is a byte no-op, so the fold's OUTPUT cannot \
         discriminate which snapshot sourced it — the fence is the store's completeness and \
         the snapshots' distinctness, not the migrated bytes",
    );

    // Idempotent: a second pass reports both current and touches neither.
    let rerun = jigc(
        repo.path(),
        home.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let stdout = assert_ok(&rerun, "the re-run");
    let report: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the re-run report is JSON ({e}); stdout:\n{stdout}"));
    assert_eq!(
        triage(&report, "already_current"),
        vec![
            format!("{RECORDS}/cache-rework.md"),
            format!("{RECORDS}/queue-rework.md"),
        ],
        "the re-run reports both records current; report:\n{report:#}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join(RECORDS).join("cache-rework.md")).expect("read v1"),
        after_v1,
        "the re-run touches no byte",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join(RECORDS).join("queue-rework.md")).expect("read v2"),
        after_v2,
        "the re-run touches no byte",
    );
}

// ---------------------------------------------------------------------------------------------
// (4) The store's OTHER stamp-keyed read — the v0 arm, now a genuine choice.
// ---------------------------------------------------------------------------------------------

/// **An unstamped (v0-era) record still folds in one pass, and the earliest snapshot is what
/// sources it.** `v0_prior_shape` picks the doctype's **earliest** shipped snapshot
/// (`find_map` over `1..current`) — a selection that was trivial while every chain had one
/// snapshot and is a real choice for the first time here. A v0 doc predates *every* shipped
/// version, so the shape it must be diffed against is `.v1.yaml`, never the immediately-prior
/// `.v2.yaml`; picking the wrong one would strand the doc's earliest links, and — on this
/// byte-neutral chain — would do so **silently** (arm 3), which is exactly why this site is
/// driven rather than assumed.
///
/// The record is also the store sweep's managed-vs-foreign case: an unstamped record parses
/// against a shape jigc once shipped, so it is read as a **managed, v0-era** doc and never as
/// a foreign file to adopt (`unadopted[]` stays empty).
#[test]
fn an_unstamped_record_folds_to_current_against_the_earliest_snapshot() {
    let repo = TempDir::new("v0");
    let home = TempDir::new("v0-home");
    init_repo(repo.path());
    let current = declared_schema_version("milestone-record");

    // The v0 byte form: the mint with the `workflow` bullet AND the stamp line removed.
    let at_current = minted_record(repo.path(), home.path());
    let at_v0: String = at_current
        .lines()
        .filter(|line| !line.starts_with("- workflow: ") && !line.starts_with("schema-version: "))
        .map(|line| format!("{line}\n"))
        .collect();
    assert!(
        !at_v0.contains("schema-version"),
        "the v0 seed carries no stamp; got:\n{at_v0}"
    );
    fs::write(repo.path().join(RECORDS).join("cache-rework.md"), &at_v0)
        .expect("seed the v0 record");
    git(repo.path(), &["add", RECORDS]);
    git(repo.path(), &["commit", "-q", "-m", "seed a v0 record"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let stdout = assert_ok(
        &out,
        "jigc migrate-corpus over an unstamped milestone-record",
    );
    let report: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the report is JSON ({e}); stdout:\n{stdout}"));
    assert_eq!(
        triage(&report, "migrated"),
        vec![format!("{RECORDS}/cache-rework.md")],
        "the unstamped record migrates in one pass; report:\n{report:#}",
    );
    assert_eq!(
        triage(&report, "blocked"),
        Vec::<String>::new(),
        "the v0 → current union is byte-neutral on this doctype; report:\n{report:#}",
    );
    assert_eq!(
        triage(&report, "unadopted"),
        Vec::<String>::new(),
        "an unstamped record that parses against a shipped prior shape is MANAGED, not a \
         foreign file to adopt; report:\n{report:#}",
    );

    // The stamp is ADDED at the current version (the v0 arm's add-field path), and no other
    // byte moves.
    let after =
        fs::read_to_string(repo.path().join(RECORDS).join("cache-rework.md")).expect("read v0");
    assert_eq!(
        after,
        at_v0.replace(
            "status: active\n---",
            &format!("status: active\nschema-version: {current}\n---")
        ),
        "the v0 record gains the stamp at the CURRENT version and moves no other byte",
    );
}
