//! M49 Increment 6 / T1 — **a pack can be extended without losing the methodology**:
//! `compose-embedded-methodology: true` composes *with* a `packs:` list, at a declared
//! precedence (`design/multi-pack.md` → Embedded second pack; the M49 settle record → D5).
//!
//! Until this task the combination was a loud refusal — the M42 repair of an even worse
//! silent drop — which left an adopter with no way to add a house doctype while using the
//! methodology pack: `jigc setup` writes the marker into *every* project, so declaring one
//! project pack bricked every door. The composition is now
//! `[listed… ▸ dev ▸ methodology]` with **one demotion**: for a doctype the embedded
//! pair's manifests declare frozen, the listed packs sort **below** the pair — *a project
//! pack may not shadow a doctype the freeze governs*. Without that demotion a manifest-less
//! listed pack would shadow a frozen dev doctype while `assert_schema_freeze` is
//! skip-on-absent for it: the freeze invariant, falsified from the layer this task opens.
//!
//! Driven through the **real binary** over a repo carrying the marker *and* a listed pack
//! that ships both a new `note` doctype and a divergent `commit`:
//!   (i)   `jigc doc schema note` exits 0 rendering the **project** doctype — extension works.
//!   (ii)  `jigc doc schema commit` exits 0 rendering **dev's frozen shape**, with none of
//!         the project's divergent fields — the demotion, on the emitted bytes.
//!   (iii) `jigc doc schema idea` still resolves — the methodology pack is not lost, which
//!         is the whole point of the combination.
//!   (iv)  `jigc validate` exits clean over the composed set.
//!
//! The precedence *within* the un-demoted id-space is proven by (i): a listed pack still
//! wins what the freeze does not govern. The unit-level statement that the demotion holds
//! for **every** governed id (all 15, read from the two manifests, never hand-listed) lives
//! with the ordering function it fences, in `pack::tests`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-project-pack-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting
/// `JIGC_PACK_DIR` (which supersedes the marker — the arm this suite is not about).
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
}

/// The house pack an adopter would write: a **new** `note` doctype (what the freeze does
/// not govern) and a **divergent `commit`** (what it does). Manifest-less, exactly as a
/// project pack authored by hand is — which is what makes the demotion load-bearing.
fn write_house_pack(root: &Path) {
    let schemas = root.join("schemas");
    fs::create_dir_all(&schemas).expect("mk schemas/");
    fs::write(
        schemas.join("note.yaml"),
        b"type: note\nlocation: notes/\nid-from: title\n\
          description: A house note.\n\
          usage: you want a short house-local note.\n\
          \nsections:\n  - id: meta\n    header: true\n    fields:\n      \
          - { id: topic, type: string }\n  - id: body\n    slot: { hint: \"The note.\" }\n",
    )
    .expect("write note.yaml");
    fs::write(
        schemas.join("commit.yaml"),
        b"type: commit\n\
          description: A house commit shape.\n\
          usage: the house wants its own commit fields.\n\
          \nsections:\n  - id: header\n    header: true\n    fields:\n      \
          - { id: type, type: enum, of: [feat, fix, chore] }\n      \
          - { id: ticket, type: string, optional: true }\n  \
          - id: summary\n    slot: { hint: \"The subject line.\" }\n",
    )
    .expect("write commit.yaml");
}

/// The house pack minus its divergent `commit` — the arm where the highest-precedence
/// pack ships **neither** the colliding doctype **nor** any workflow, so every pack
/// attribution `--explain` prints must name a pack *below* it.
fn write_extension_only_pack(root: &Path) {
    write_house_pack(root);
    fs::remove_file(root.join("schemas").join("commit.yaml")).expect("drop the divergent commit");
}

/// The house pack's own identity (`config/defaults.yaml`) — `pack-id: house`,
/// `version: 0.0.1`. Present only in the provenance arms: without it the composite's
/// precedence read of `config/defaults` falls through to dev and the misattribution
/// this task fixes is invisible on the emitted bytes.
fn write_house_identity(root: &Path) {
    let config = root.join("config");
    fs::create_dir_all(&config).expect("mk config/");
    fs::write(
        config.join("defaults.yaml"),
        b"pack-id: house\nversion: 0.0.1\n",
    )
    .expect("write the house pack's defaults.yaml");
}

/// A repo whose project layer carries the marker (written by a normal `jigc setup`)
/// **and** a `packs:` list naming the house pack — the combination this task composes.
fn marker_plus_listed_repo(tag: &str) -> (TempDir, TempDir) {
    marker_plus_pack_repo(tag, &write_house_pack)
}

/// [`marker_plus_listed_repo`] over an arbitrary house-pack writer — the seam the
/// provenance arms use to vary what the highest-precedence pack ships.
fn marker_plus_pack_repo(tag: &str, write_pack: &dyn Fn(&Path)) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    init_repo(repo.path());
    let home = TempDir::new("home");
    let setup = run(repo.path(), home.path(), &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    let house = repo.path().join("housepack");
    write_pack(&house);

    let packs_yaml = repo.path().join(".jigc").join("config").join("packs.yaml");
    let existing = fs::read_to_string(&packs_yaml).expect("read packs.yaml after setup");
    assert!(
        existing.contains("compose-embedded-methodology: true"),
        "`jigc setup` must write the compose marker; got:\n{existing}",
    );
    fs::write(
        &packs_yaml,
        format!(
            "compose-embedded-methodology: true\npacks:\n  - {}\n",
            house.display()
        ),
    )
    .expect("write packs.yaml carrying the marker AND the list");

    (repo, home)
}

#[test]
fn a_listed_pack_adds_a_doctype_the_freeze_does_not_govern() {
    // (i) Extension: the house `note` doctype resolves through the composed set and
    // `doc schema note` renders the PROJECT pack's shape. This is the capability the
    // refusal denied — with the marker written into every setup-initialized project,
    // an adopter could not add a house doctype at all.
    let (repo, home) = marker_plus_listed_repo("note");

    let out = run(repo.path(), home.path(), &["doc", "schema", "note"]);
    assert!(
        out.status.success(),
        "(i) `jigc doc schema note` over marker + a listed pack must exit 0; got {:?}\n\
         stderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        rendered.contains("doctype: note"),
        "(i) the rendered schema must be the house `note` doctype; got:\n{rendered}",
    );
    assert!(
        rendered.contains("topic: string"),
        "(i) the rendered `note` must carry the house pack's own field; got:\n{rendered}",
    );
}

#[test]
fn a_listed_pack_may_not_shadow_a_doctype_the_freeze_governs() {
    // (ii) The demotion, on the emitted bytes: `commit` is declared in the embedded
    // pair's manifests, so the listed pack sorts BELOW the pair for it and dev's frozen
    // shape wins. Before this task the listed pack would have won it — a manifest-less
    // shadow of a frozen doctype, invisible to the skip-on-absent freeze assertion.
    let (repo, home) = marker_plus_listed_repo("commit");

    let out = run(repo.path(), home.path(), &["doc", "schema", "commit"]);
    assert!(
        out.status.success(),
        "(ii) `jigc doc schema commit` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        rendered.contains("implements: ref"),
        "(ii) `commit` must resolve to DEV's frozen shape (which carries \
         `implements→spec`); got:\n{rendered}",
    );
    assert!(
        !rendered.contains("ticket"),
        "(ii) the project pack's divergent `commit` field must NOT reach the composed \
         schema — a project pack may not shadow a doctype the freeze governs; \
         got:\n{rendered}",
    );
    assert!(
        rendered.contains("trailers: repeatable"),
        "(ii) dev's whole frozen shape wins, not a merge of the two; got:\n{rendered}",
    );
}

#[test]
fn the_methodology_pack_is_not_lost_when_a_project_pack_is_listed() {
    // (iii) The point of the combination: the embedded methodology pack still composes,
    // so a project that adds a house doctype keeps vision/roadmap/idea authoring.
    let (repo, home) = marker_plus_listed_repo("idea");

    let out = run(repo.path(), home.path(), &["doc", "schema", "idea"]);
    assert!(
        out.status.success(),
        "(iii) `jigc doc schema idea` must exit 0 — the methodology pack must not be \
         lost when a project pack is listed; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        rendered.contains("doctype: idea"),
        "(iii) the methodology `idea` doctype must render; got:\n{rendered}",
    );
}

#[test]
fn the_composed_set_validates_clean() {
    // (iv) The whole pack-set loads at every door, not just the read surfaces: the store
    // sweep runs over the composed set and finds nothing.
    let (repo, home) = marker_plus_listed_repo("validate");

    let out = run(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "(iv) `jigc validate` over marker + a listed pack must exit 0; got {:?}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
        out.status,
    );
    assert!(
        stdout.contains("no findings"),
        "(iv) the composed set must validate clean; got:\n{stdout}",
    );
}

/// The embedded packs' version — `EmbeddedPack::pack_version` is the binary's own, so
/// the provenance assertions name a *pack* rather than pinning a release string.
const BINARY_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Every `--explain` line of `out` that attributes something to a pack — the workflow
/// line and each `collision:` line. Extracted from the **emitted bytes** so the
/// assertions run on what an agent reads, never on a reconstruction.
fn attribution_lines(out: &str) -> Vec<String> {
    out.lines()
        .filter(|l| l.starts_with("workflow:") || l.trim_start().starts_with("collision:"))
        .map(|l| l.trim_start().to_string())
        .collect()
}

#[test]
fn explain_names_the_pack_that_actually_provided_each_resolved_id() {
    // T3 — the provenance half of the composition. Over `[house ▸ dev ▸ methodology]`
    // where the house pack ships NEITHER the colliding `commit` doctype NOR any
    // workflow, every pack attribution `--explain` prints must name a pack BELOW the
    // top of the precedence order. Verified live at HEAD, which printed all three
    // attributions as `house`:
    //   workflow:single-task    (pack-default · house/v0.0.1)   [the pre-M50 label]
    //   collision: default-workflow → won by house/0.0.1
    //   collision: doctype:commit → won by house/0.0.1
    // `design/overrides.md` makes cascade provenance a hard determinism contract; a
    // header naming a pack that provided nothing is exactly the hidden variance it
    // exists to prevent.
    let (repo, home) = marker_plus_pack_repo("explain-owner", &|root: &Path| {
        write_extension_only_pack(root);
        write_house_identity(root);
    });

    let out = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--explain",
            "--workflow",
            "single-task",
            "add a thing",
        ],
    );
    assert!(
        out.status.success(),
        "`jigc start --explain` over the three-pack composition must exit 0; got {:?}\n\
         stderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let text = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // The house pack IS in the composed set — the misattribution was possible precisely
    // because it composes, so its absence would make the arm vacuous.
    assert!(
        text.contains("Pack input: house/0.0.1 = "),
        "the house pack must be in the composed set; got:\n{text}",
    );

    // Every attribution, on the emitted bytes: the workflow line names DEV (whose pack
    // ships `single-task`), and each collision winner names the pack that owns the id.
    assert_eq!(
        attribution_lines(&text),
        vec![
            format!("workflow:single-task    (pack-default · dev/{BINARY_VERSION})"),
            format!("collision: doctype:commit → won by dev/{BINARY_VERSION}"),
            format!("collision: config:knobs → won by dev/{BINARY_VERSION}"),
        ],
        "every pack attribution must name the pack that provided the thing; got:\n{text}",
    );

    // Derived, not hand-listed — and filtered: `note` is owned by one pack, so it
    // adjudicates nothing and prints no winner line.
    assert!(
        !text.contains("doctype:note"),
        "an id only one pack owns is no collision and must print no winner line; \
         got:\n{text}",
    );
}

#[test]
fn explain_names_the_demoted_project_doctype_as_the_loss_it_is() {
    // T3 — the demotion, seen from the provenance surface. The house pack DOES ship a
    // divergent `commit`, so all three packs own it and the freeze demotion (T1) makes
    // the house pack LOSE. `--explain` must say so: the winner is dev, the pack whose
    // frozen shape `doc schema commit` actually renders. At HEAD the line named
    // `house/0.0.1` — the surface asserted the opposite of what the loader resolved,
    // which is the worst reading of a determinism header.
    let (repo, home) = marker_plus_pack_repo("explain-demoted", &|root: &Path| {
        write_house_pack(root);
        write_house_identity(root);
    });

    let out = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--explain",
            "--workflow",
            "single-task",
            "add a thing",
        ],
    );
    assert!(
        out.status.success(),
        "`jigc start --explain` over the demoted composition must exit 0; got {:?}\n\
         stderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let text = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        text.contains("Pack input: house/0.0.1 = "),
        "the house pack must be in the composed set; got:\n{text}",
    );
    assert!(
        text.contains(&format!(
            "collision: doctype:commit → won by dev/{BINARY_VERSION}"
        )),
        "the demoted project doctype's loss must be named: dev wins `commit`; got:\n{text}",
    );
    assert!(
        !text.contains("won by house/"),
        "the house pack wins nothing here — it must not be named as any winner; \
         got:\n{text}",
    );

    // The named winner is the one the loader actually resolved — the surface and the
    // schema read cannot disagree.
    let schema = run(repo.path(), home.path(), &["doc", "schema", "commit"]);
    let rendered = String::from_utf8_lossy(&schema.stdout).into_owned();
    assert!(
        schema.status.success() && !rendered.contains("ticket"),
        "`doc schema commit` must render DEV's frozen shape, matching the named \
         winner; got {:?}\n{rendered}",
        schema.status,
    );
}

// ---------------------------------------------------------------------------
// M50 Increment 12 / T6 (W-6) — **one pack is named one way on every surface that
// names one.** Four surfaces render a `(pack-id, version)` pair: the orientation
// `Pack:` header, the `--explain` workflow label, its `collision:` lines and its
// `Pack input:` lines. Driven at the increment base, one `--explain` screen named the
// SAME pack twice, two ways:
//
//     workflow:single-task    (pack-default · dev/vfs-local)
//       Pack input: dev/fs-local = <…>/crates/cli/pack  (blake3 …)
//
// The label glued a literal `v` onto whatever the pack reported as its version
// (`start.rs`'s `format!("{pack_id}/v{version}")`), so a reader is asked to believe
// one pack ships two versions. `design/multi-pack.md` → Provenance states the segment
// as `Pack: <id>/<version>`, and `design/surface-contract.md` law 1 forbids a surface
// that says something untrue — a pack named two ways on one screen is exactly that.
// ---------------------------------------------------------------------------

/// The four `--explain` / orientation line kinds that name a pack. Matched on the
/// **trimmed** line so the two-space-indented `--explain` body lines are reached.
const PACK_NAMING_PREFIXES: [&str; 4] = ["Pack: ", "workflow:", "collision:", "Pack input:"];

/// Every `(surface, spelling)` pair in `out` that names `pack_id` — scanned off the
/// **emitted bytes**, never a reconstruction, so the assertion runs on what a reader
/// actually sees. `surface` is the [`PACK_NAMING_PREFIXES`] entry the line matched;
/// `spelling` is `<pack_id>/<token>` with the token taken as the run of version
/// characters that follows.
///
/// A `Pack input:` line is cut at its ` = ` first: everything after is the pack's
/// resolving directory path, which may itself contain the pack id as a path segment
/// and would otherwise be scanned as a naming.
fn pack_namings(out: &str, pack_id: &str) -> Vec<(&'static str, String)> {
    let needle = format!("{pack_id}/");
    let mut found = Vec::new();
    for line in out.lines() {
        let line = line.trim_start();
        let Some(surface) = PACK_NAMING_PREFIXES
            .iter()
            .find(|prefix| line.starts_with(**prefix))
        else {
            continue;
        };
        let scanned = match line.split_once(" = ") {
            Some((head, _)) if *surface == "Pack input:" => head,
            _ => line,
        };
        let mut rest = scanned;
        while let Some(at) = rest.find(&needle) {
            let tail = &rest[at + needle.len()..];
            let end = tail
                .find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+')))
                .unwrap_or(tail.len());
            found.push((*surface, format!("{needle}{}", &tail[..end])));
            rest = &tail[end..];
        }
    }
    found
}

/// The distinct spellings in a [`pack_namings`] result, sorted — the set the
/// one-spelling assertion is about.
fn distinct(namings: &[(&'static str, String)]) -> Vec<String> {
    let mut spellings: Vec<String> = namings.iter().map(|(_, s)| s.clone()).collect();
    spellings.sort();
    spellings.dedup();
    spellings
}

/// The surfaces present in a [`pack_namings`] result, sorted — asserted so the
/// one-spelling claim can never pass vacuously over a surface that rendered nothing.
fn surfaces(namings: &[(&'static str, String)]) -> Vec<&'static str> {
    let mut kinds: Vec<&'static str> = namings.iter().map(|(k, _)| *k).collect();
    kinds.sort_unstable();
    kinds.dedup();
    kinds
}

#[test]
fn one_pack_is_named_one_way_on_every_surface_that_names_one() {
    // T6 — one `(pack_id, version)` pair, `dev`'s, rendered through all four
    // pack-naming surfaces of one composition, asserted to have exactly ONE spelling.
    // The three-pack shape is the provenance arm's: the house pack ships neither the
    // workflow nor the colliding doctype, so `dev` is the workflow line's pack, the
    // collision winner, a `Pack input:` entry and a segment of the orientation header
    // — all four surfaces name the same pair, which is what makes them comparable.
    let (repo, home) = marker_plus_pack_repo("one-spelling", &|root: &Path| {
        write_extension_only_pack(root);
        write_house_identity(root);
    });

    let explain = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--explain",
            "--workflow",
            "single-task",
            "add a thing",
        ],
    );
    assert!(
        explain.status.success(),
        "`jigc start --explain` must exit 0; got {:?}\nstderr:\n{}",
        explain.status,
        String::from_utf8_lossy(&explain.stderr),
    );
    let explain_text = String::from_utf8(explain.stdout).expect("utf-8 stdout");

    let orient = run(repo.path(), home.path(), &["start"]);
    assert!(
        orient.status.success(),
        "bare `jigc start` (orientation) must exit 0; got {:?}\nstderr:\n{}",
        orient.status,
        String::from_utf8_lossy(&orient.stderr),
    );
    let orient_text = String::from_utf8(orient.stdout).expect("utf-8 stdout");

    let mut namings = pack_namings(&explain_text, "dev");
    namings.extend(pack_namings(&orient_text, "dev"));

    // Non-vacuity: every one of the four surfaces actually rendered a `dev` naming.
    assert_eq!(
        surfaces(&namings),
        vec!["Pack input:", "Pack: ", "collision:", "workflow:"],
        "all four pack-naming surfaces must name `dev`, else the one-spelling claim \
         passes over a surface that rendered nothing; explain:\n{explain_text}\n\
         orientation:\n{orient_text}",
    );

    // The claim: one pack, one spelling — `<id>/<version>`, the segment
    // `design/multi-pack.md` → Provenance states and the orientation goldens carry.
    assert_eq!(
        distinct(&namings),
        vec![format!("dev/{BINARY_VERSION}")],
        "one pack must be named ONE way on every surface that names one; \
         explain:\n{explain_text}\norientation:\n{orient_text}",
    );
}

/// The dev pack **as a directory** — the same tree the binary embeds, reached through
/// the explicit `JIGC_PACK_DIR` channel. It declares no `version:` in
/// `config/defaults.yaml`, so a `FilesystemPack` reports the `fs-local` sentinel; the
/// embedded copy answers `CARGO_PKG_VERSION` instead, so this is the only way to drive
/// a version-less pack through the real binary.
fn dev_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// Like [`run`] but with `JIGC_PACK_DIR` **set** — the explicit channel, used only by
/// the version-less arm below (every other arm in this suite is about the marker path
/// and requires the variable absent).
fn run_with_pack_dir(
    repo: &Path,
    home: &Path,
    pack_dir: &Path,
    args: &[&str],
) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

#[test]
fn a_version_less_packs_label_does_not_read_as_a_version() {
    // T6, the arm that shows WHY a glued prefix is a lie rather than a style: a pack
    // that declares no `version:` reports the `fs-local` sentinel, and `v` + `fs-local`
    // reads as a version — `vfs-local` — that no pack anywhere declares. Both spellings
    // shipped on the SAME screen at the increment base (the workflow line against its
    // own `Pack input:` line), which is the driven observation this arm pins.
    let repo = TempDir::new("version-less");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = dev_pack_tree();

    let setup = run_with_pack_dir(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` under JIGC_PACK_DIR must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    let explain = run_with_pack_dir(
        repo.path(),
        home.path(),
        &pack,
        &[
            "start",
            "--explain",
            "--workflow",
            "single-task",
            "add a thing",
        ],
    );
    assert!(
        explain.status.success(),
        "`jigc start --explain` under JIGC_PACK_DIR must exit 0; got {:?}\nstderr:\n{}",
        explain.status,
        String::from_utf8_lossy(&explain.stderr),
    );
    let text = String::from_utf8(explain.stdout).expect("utf-8 stdout");

    let namings = pack_namings(&text, "dev");
    assert!(
        surfaces(&namings).contains(&"workflow:") && surfaces(&namings).contains(&"Pack input:"),
        "both the workflow label and the `Pack input:` line must name `dev`, else the \
         same-screen comparison is vacuous; got:\n{text}",
    );
    assert_eq!(
        distinct(&namings),
        vec!["dev/fs-local".to_string()],
        "a version-less pack reports the `fs-local` sentinel; no surface may dress it \
         as the version `vfs-local`, which no pack declares; got:\n{text}",
    );
}

/// A second, minimal house pack — present only so the preserved `packs:` list has an
/// **order** to preserve. It ships one more doctype the freeze does not govern.
fn write_second_pack(root: &Path) {
    let schemas = root.join("schemas");
    fs::create_dir_all(&schemas).expect("mk schemas/");
    fs::write(
        schemas.join("memo.yaml"),
        b"type: memo\nlocation: memos/\nid-from: title\n\
          description: A house memo.\n\
          usage: you want a short house-local memo.\n\
          \nsections:\n  - id: meta\n    header: true\n    fields:\n      \
          - { id: audience, type: string }\n  - id: body\n    slot: { hint: \"The memo.\" }\n",
    )
    .expect("write memo.yaml");
}

/// `git add` + `git commit` one repo-relative path — the state a hand-authored project
/// layer is in when it is part of the repository.
fn commit_path(root: &Path, relative: &str) {
    for args in [
        vec!["add", "--", relative],
        vec!["commit", "-q", "-m", "the operator's project layer"],
    ] {
        let out = Command::new("git")
            .args(&args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

/// The `packs:` entries of a `packs.yaml`, in file order — the sequence this task must
/// carry through `jigc setup` untouched. Line-based on purpose: the assertion is about
/// the emitted bytes, not about a re-parse agreeing with itself.
fn packs_entries(text: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if line.trim_end() == "packs:" {
            inside = true;
            continue;
        }
        if !inside {
            continue;
        }
        let trimmed = line.trim_start();
        if let Some(entry) = trimmed.strip_prefix("- ") {
            entries.push(entry.trim().to_string());
        } else if !trimmed.is_empty() {
            break;
        }
    }
    entries
}

#[test]
fn setup_wires_the_marker_over_a_projects_own_pack_list() {
    // T2 — the second half of the refusal. The loader now composes the marker WITH a
    // `packs:` list (T1), but `jigc setup` still vetoed the marker whenever a list was
    // present and said so on stderr. Verified live at HEAD: setup exits 0 and leaves
    // `packs.yaml` byte-identical, so "adding any doctype costs the entire methodology
    // pack" was true through SETUP, not only through the loader.
    //
    // Over a repo whose project layer ALREADY declares two house packs, `jigc setup`
    // must: exit 0 · print no "left unwired" line · preserve both entries in order and
    // content · add the marker · leave the project composing BOTH surfaces · and write
    // no bytes on a second run.
    let repo = TempDir::new("setup-over-list");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let house = repo.path().join("housepack");
    write_house_pack(&house);
    let other = repo.path().join("otherpack");
    write_second_pack(&other);

    let config_dir = repo.path().join(".jigc").join("config");
    fs::create_dir_all(&config_dir).expect("mk .jigc/config/");
    let packs_yaml = config_dir.join("packs.yaml");
    let authored = format!("packs:\n  - {}\n  - {}\n", house.display(), other.display());
    fs::write(&packs_yaml, &authored).expect("write the operator's hand-authored packs.yaml");
    // **Committed, because that is where an operator's own project layer lives** — and
    // because since M51 Increment 3 `jigc setup` refuses its install commit over an
    // install path carrying bytes it did not write, and `packs.yaml` is one of the host
    // files it **merges into** rather than rewrites (`design/validation.md` →
    // `setup.dirty-install-path`). Leaving it untracked here would make this test's
    // subject that refusal instead of pack composition.
    commit_path(repo.path(), ".jigc/config/packs.yaml");
    let authored_entries = packs_entries(&authored);
    assert_eq!(authored_entries.len(), 2, "the fixture declares two packs");

    let setup = run(repo.path(), home.path(), &["setup"]);
    let setup_err = String::from_utf8_lossy(&setup.stderr).into_owned();
    let setup_out = String::from_utf8_lossy(&setup.stdout).into_owned();
    assert!(
        setup.status.success(),
        "`jigc setup` over a project that lists its own packs must exit 0; got {:?}\n\
         stdout:\n{setup_out}\nstderr:\n{setup_err}",
        setup.status,
    );
    assert!(
        !setup_err.contains("left unwired") && !setup_out.contains("left unwired"),
        "setup must no longer report the embedded methodology pack as left unwired; \
         got stderr:\n{setup_err}\nstdout:\n{setup_out}",
    );

    let after = fs::read_to_string(&packs_yaml).expect("read packs.yaml after setup");
    assert!(
        after.contains("compose-embedded-methodology: true"),
        "setup must wire the compose marker over the operator's own pack list; got:\n{after}",
    );
    assert_eq!(
        packs_entries(&after),
        authored_entries,
        "the operator's `packs:` entries must survive setup byte-identical, in order; \
         got:\n{after}",
    );

    // The project composes BOTH: a methodology workflow reaches the catalog, and the
    // listed pack's own doctype resolves. Either one alone is the old either/or.
    let describe = run(repo.path(), home.path(), &["describe"]);
    let described = String::from_utf8_lossy(&describe.stdout).into_owned();
    assert!(
        describe.status.success(),
        "`jigc describe` must exit 0 over the composed set; got {:?}\nstderr:\n{}",
        describe.status,
        String::from_utf8_lossy(&describe.stderr),
    );
    assert!(
        described.contains("park-idea"),
        "a methodology-only workflow must appear in `jigc describe` — setup must wire \
         the embedded pair over the listed packs; got:\n{described}",
    );

    let schema = run(repo.path(), home.path(), &["doc", "schema", "note"]);
    let rendered = String::from_utf8_lossy(&schema.stdout).into_owned();
    assert!(
        schema.status.success(),
        "`jigc doc schema note` must exit 0 — the listed pack must still be loaded; \
         got {:?}\nstderr:\n{}",
        schema.status,
        String::from_utf8_lossy(&schema.stderr),
    );
    assert!(
        rendered.contains("doctype: note") && rendered.contains("topic: string"),
        "the listed pack's own doctype must render; got:\n{rendered}",
    );

    // A second setup writes no bytes: same content AND an untouched mtime.
    let bytes_before = fs::read(&packs_yaml).expect("read packs.yaml bytes");
    let mtime_before = fs::metadata(&packs_yaml)
        .and_then(|m| m.modified())
        .expect("packs.yaml mtime");
    let again = run(repo.path(), home.path(), &["setup"]);
    assert!(
        again.status.success(),
        "a second `jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        again.status,
        String::from_utf8_lossy(&again.stderr),
    );
    assert_eq!(
        fs::read(&packs_yaml).expect("re-read packs.yaml bytes"),
        bytes_before,
        "a second setup must leave `packs.yaml` byte-identical",
    );
    assert_eq!(
        fs::metadata(&packs_yaml)
            .and_then(|m| m.modified())
            .expect("packs.yaml mtime after"),
        mtime_before,
        "a second setup must write no bytes at all — the file's mtime must not move",
    );
}
