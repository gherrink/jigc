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

/// A repo whose project layer carries the marker (written by a normal `jigc setup`)
/// **and** a `packs:` list naming the house pack — the combination this task composes.
fn marker_plus_listed_repo(tag: &str) -> (TempDir, TempDir) {
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
    write_house_pack(&house);

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
