//! M53 Increment 1 / T2 + T3 — **the milestone area's `merged/` membership rule, both
//! halves**: the probe walks `merged/` and `staged_doc_id` alone decides inside it (T2), and
//! `engine::state::unwind_area`'s milestone arm honours the identical walk instead of
//! `remove_dir_all` (T3)
//! (`completions/artifacts/M53/settle-record.md` → D1.2 as amended by §2, and D1.4;
//! `design/team-ready-state.md` → The working area's two populations;
//! `implementation/roadmap.md` → M53 Increment 1).
//!
//! **Why one suite.** The probe and the removal are the two halves of one membership rule,
//! and `foreign_area_paths`' own doc-comment claims they *cannot disagree about a file*. A
//! claim of that shape is worth only the fixture both halves are asked over, so they are
//! asked over the same one here.
//!
//! `engine::state::MILESTONE_AREA_FILES` carried `merged/` as a **tree member nothing inside
//! was walked** — *"the join's staging area, jigc's wholesale"* — so every byte under
//! `.jigc/milestones/<id>/merged/` was jigc's by declaration. The declaration rests on a
//! predicate, and the predicate is false four ways: a finalize blocked at exit 3 leaves the
//! materialized bodies standing where a human can edit beside them; a succeeding `pre-commit`
//! hook writes into the area during the commit (M52's own recorded writer,
//! `engine/state.rs`); `materialize` took an editor `.swp` on a finalize that committed
//! nothing; and the carve-out **defeats the consent gate** at the two doors that have one —
//! driven at planning, `jigc milestone discard` with no `--force` over `merged/top.txt` and
//! `merged/docs/deep.txt` exited 0, printed *"workbench removed"* and took both, while the
//! identical byte at the area **root** refused with `milestone.foreign-bytes`. One run, two
//! answers, from one carve-out.
//!
//! **The rule inside `merged/` is `engine::state::staged_doc_id` ALONE** (§2). The task
//! branch's predicate is `TASK_DOCS_FILES.contains(name) || staged_doc_id(name)`;
//! `engine::milestone::materialize` writes **only** `<type>:<slug>.md` bodies, so inheriting
//! `TASK_DOCS_FILES` would call a foreign `merged/docs/provenance.json` jigc's own and hand it
//! to the destroying doors.
//!
//! **The zero-false-fire control is the load-bearing half.** A probe one level too wide turns
//! every `jigc uninstall` on a corpus holding a joined milestone into a refusal — so the
//! control runs over a merged tree this suite did not write: a **real** fan-out whose
//! `materialize` produced the bodies, read back through the probe and through the door.
//!
//! What drives what: the complement itself is asserted engine-facing, because the probe is
//! where the rule lives and a `PathBuf` vector is the rule's own shape; the consent gate is
//! driven through the **real binary** at both doors that have one, because *"the door refuses
//! and `--force` narrates"* is a claim about the emitted surface.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// The milestone every driven arm builds.
const MILESTONE: &str = "cache-rework";

/// **The six plant loci** — one per cell of D1's acceptance axis, each the relative path the
/// complement must report, with the reason it is its own cell.
///
/// `merged/sub` is the **whole-directory** unit rule: a foreign directory is one entry, never
/// descended, because the entry is what a door names, refuses over and moves, and moving the
/// top of a subtree preserves it.
const PLANTS: &[(&str, &str)] = &[
    (
        "scratch.txt",
        "the area ROOT — the shipped cell, jigc's own registry row decides it, and the one \
         that already refused while its `merged/` siblings did not",
    ),
    (
        "merged/top.txt",
        "`merged/` top — inside the tree member, beside `docs/`",
    ),
    (
        "merged/sub",
        "a foreign DIRECTORY under `merged/`, returned whole and never descended",
    ),
    (
        "merged/docs/adr.md",
        "a `.md` under `merged/docs/` whose stem carries no `:` — jigc's own writer emits \
         `<type>:<slug>.md` at every site (`engine::state::instance_path`), so a colon-less \
         `.md` is something else's",
    ),
    (
        "merged/docs/deep.txt",
        "a foreign non-`.md` file beside a materialized body",
    ),
    (
        "merged/docs/provenance.json",
        "§2's cell — the milestone rule is `staged_doc_id` ALONE, so the task branch's \
         `TASK_DOCS_FILES` member is a third party's file here",
    ),
];

/// The plants sorted as `foreign_area_paths` returns them.
fn expected_complement() -> Vec<PathBuf> {
    let mut all: Vec<PathBuf> = PLANTS.iter().map(|(cell, _)| PathBuf::from(cell)).collect();
    all.sort();
    all
}

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-merged-complement-{tag}-{}-{:?}",
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

fn git_ok(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A real git repo with one commit plus the project cascade layer (`jigc milestone`'s
/// door-top precondition, M52 Inc 8 / T1).
fn init_repo(root: &Path) {
    git_ok(root, &["init", "-q"]);
    git_ok(root, &["config", "user.email", "test@example.com"]);
    git_ok(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write README.md");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
    crate::support::mint_project_layer(root);
}

/// Build the pack's real `doc-code` probe once (process-wide) — the merged-state gate's
/// anchor arm shells out to it (the `milestone_boundary_gate` idiom).
fn real_doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
}

/// Run the real binary in `repo`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", real_doc_code_probe())
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

fn expect_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

fn both_streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area, the two
/// inputs the by-task-id join reads.
fn stage_doc(repo: &Path, sub: &str, address: &str, body: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk docs/");
    fs::write(docs.join(format!("{address}.md")), body).expect("write staged body");

    let manifest = docs.join("provenance.json");
    let mut record: serde_json::Value = match fs::read_to_string(&manifest) {
        Ok(s) => serde_json::from_str(&s).expect("provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = serde_json::Value::String("created".to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize manifest"),
    )
    .expect("write provenance manifest");
}

/// A conformant, ref-free ADR body.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// A **non-conformant** ADR body — the required `## Decision` is missing, so the merged-state
/// gate blocks at exit 3. That block is what leaves `merged/` standing to be read, and it is
/// the boundary's **own** cause, never one of the plants.
fn adr_missing_decision(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

fn milestone_area(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("milestones").join(MILESTONE)
}

/// **A real post-join milestone area**: a two-sub-task fan-out whose `materialize` wrote the
/// merged bodies, left standing by a finalize blocked at exit 3 on its own conformance cause.
///
/// Returns the repo and the HOME the binary ran under, both alive for the caller's lifetime.
fn post_join_area(tag: &str) -> (TempDir, TempDir) {
    let repo_dir = TempDir::new(tag);
    let home_dir = TempDir::new(&format!("{tag}-home"));
    let (repo, home) = (repo_dir.path(), home_dir.path());
    init_repo(repo);

    expect_ok(&jigc(repo, home, &["setup"]), "jigc setup");
    expect_ok(
        &jigc(repo, home, &["milestone", "create", "Cache rework"]),
        "milestone create",
    );
    for intent in ["Doc area", "Code area"] {
        expect_ok(
            &jigc(repo, home, &["milestone", "add-task", MILESTONE, intent]),
            "milestone add-task",
        );
    }
    stage_doc(
        repo,
        "doc-area",
        "adr:broken-policy",
        &adr_missing_decision("Broken policy"),
    );
    stage_doc(
        repo,
        "code-area",
        "adr:code-policy",
        &adr_plain("Code policy"),
    );
    expect_ok(
        &jigc(repo, home, &["milestone", "provision", MILESTONE]),
        "milestone provision",
    );

    let blocked = jigc(repo, home, &["milestone", "finalize", MILESTONE]);
    assert_eq!(
        blocked.status.code(),
        Some(3),
        "the fixture's boundary must block at exit 3 on its OWN conformance cause — that \
         block is what leaves `merged/` standing; got:\n{}",
        both_streams(&blocked),
    );

    let docs = milestone_area(repo).join("merged").join("docs");
    let bodies: Vec<String> = fs::read_dir(&docs)
        .expect("the join's staging docs dir exists after a blocked boundary")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        bodies.iter().any(|name| name.contains(':')),
        "the fixture must really hold materialized bodies, else the control proves nothing; \
         `merged/docs/` holds {bodies:?}",
    );

    (repo_dir, home_dir)
}

/// Plant the six loci into a milestone area. Returns nothing — [`expected_complement`] is the
/// set, so the plant and the assertion cannot drift.
fn plant_six(area: &Path) {
    fs::create_dir_all(area.join("merged").join("docs")).expect("mk merged/docs/");
    fs::create_dir_all(area.join("merged").join("sub")).expect("mk merged/sub/");
    fs::write(area.join("merged").join("sub").join("nested.txt"), "deep\n").expect("nested");
    for (cell, _) in PLANTS {
        if *cell == "merged/sub" {
            continue;
        }
        fs::write(area.join(cell), format!("FOREIGN — {cell}\n")).expect("plant");
    }
}

// ---------------------------------------------------------------------------
// The zero-false-fire control — the load-bearing half
// ---------------------------------------------------------------------------

/// **An ordinary post-join area has nothing for a door to refuse over.** The merged tree is one
/// `materialize` wrote, read back through the probe and through a door that would refuse over
/// it. (What a *landed* `milestone finalize` displaces is T4's cell, not this one.)
#[test]
fn an_ordinary_post_join_area_has_an_empty_complement() {
    let (repo_dir, home_dir) = post_join_area("control");
    let (repo, home) = (repo_dir.path(), home_dir.path());
    let area = milestone_area(repo);

    let on_disk: Vec<String> = fs::read_dir(&area)
        .expect("read the milestone area")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        engine::state::foreign_area_paths(&area, engine::state::WorkArea::Milestone)
            .expect("the area enumerates"),
        Vec::<PathBuf>::new(),
        "a probe one level too wide turns every `jigc uninstall` on a corpus holding a joined \
         milestone into a refusal — the area holds {on_disk:?}",
    );

    // …and the door agrees. `uninstall` refuses over this corpus for its own reasons (the
    // sub-task areas hold staged prose), so the assertion is that it never reaches for THIS
    // code — a false fire would be a refusal nothing can clear.
    let out = jigc(repo, home, &["uninstall"]);
    let text = both_streams(&out);
    assert!(
        !text.contains("uninstall.foreign-bytes"),
        "`jigc uninstall` must not refuse over jigc's OWN merged bodies; got:\n{text}",
    );
}

// ---------------------------------------------------------------------------
// The probe — engine-facing, over the six loci
// ---------------------------------------------------------------------------

/// **Every plant locus is foreign, a foreign directory comes back whole, and nothing jigc
/// wrote joins them.**
#[test]
fn every_plant_locus_under_merged_is_foreign_and_returned_whole() {
    let (repo_dir, _home_dir) = post_join_area("loci");
    let area = milestone_area(repo_dir.path());
    plant_six(&area);

    let foreign = engine::state::foreign_area_paths(&area, engine::state::WorkArea::Milestone)
        .expect("the area enumerates");

    assert_eq!(
        foreign,
        expected_complement(),
        "the complement is exactly the six loci, sorted — cells:\n{}",
        PLANTS
            .iter()
            .map(|(cell, why)| format!("  {cell}: {why}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    assert!(
        !foreign.contains(&PathBuf::from("merged/sub/nested.txt")),
        "a foreign directory is the unit a door names, refuses over and moves — it is \
         returned whole and never descended; got {foreign:?}",
    );
    for member in engine::state::MILESTONE_AREA_FILES {
        assert!(
            !foreign.contains(&PathBuf::from(*member)),
            "`{member}` is a registry member, written here in the shape jigc writes it, and \
             must never be in the complement",
        );
    }
    assert!(
        foreign.iter().all(|p| !p.to_string_lossy().contains(':')),
        "and no materialized body — `staged_doc_id` is what makes a name one jigc's own \
         writer could have produced; got {foreign:?}",
    );
}

/// **Shape is part of membership at both levels**, read without following symlinks — the L-3
/// lesson asked at the door's own question, one level deeper than the task branch asks it.
#[test]
fn shape_is_membership_at_both_levels_under_merged() {
    let root = TempDir::new("shapes");
    let outside = root.path().join("outside");
    fs::create_dir_all(&outside).expect("outside root");
    fs::write(outside.join("elsewhere.md"), "elsewhere\n").expect("outside file");

    // (a) a plain FILE wearing the tree member's name is not the join's staging area.
    let flat = root.path().join("flat");
    fs::create_dir_all(&flat).expect("area");
    fs::write(flat.join("merged"), "not a tree\n").expect("file named merged");
    assert_eq!(
        engine::state::foreign_area_paths(&flat, engine::state::WorkArea::Milestone)
            .expect("enumerates"),
        vec![PathBuf::from("merged")],
    );

    // (b) inside `merged/`, a plain FILE named `docs` is nothing `materialize` wrote either,
    //     and it is returned whole rather than descended into.
    let flat_docs = root.path().join("flat-docs");
    fs::create_dir_all(flat_docs.join("merged")).expect("merged");
    fs::write(flat_docs.join("merged").join("docs"), "not a tree\n").expect("file named docs");
    assert_eq!(
        engine::state::foreign_area_paths(&flat_docs, engine::state::WorkArea::Milestone)
            .expect("enumerates"),
        vec![PathBuf::from("merged/docs")],
    );

    // (b′) …and a SYMLINK named `docs`, even one pointing at a real directory. Following it
    //      would have this walk enumerating **somebody else's** directory and reporting its
    //      entries as paths inside a milestone area the doors are about to destroy.
    let linked_docs = root.path().join("linked-docs");
    fs::create_dir_all(linked_docs.join("merged")).expect("merged");
    fs::create_dir_all(outside.join("real-docs")).expect("a real directory elsewhere");
    fs::write(
        outside.join("real-docs").join("adr:theirs.md"),
        "# Theirs\n",
    )
    .expect("their body");
    std::os::unix::fs::symlink(
        outside.join("real-docs"),
        linked_docs.join("merged").join("docs"),
    )
    .expect("symlink named docs");
    assert_eq!(
        engine::state::foreign_area_paths(&linked_docs, engine::state::WorkArea::Milestone)
            .expect("enumerates"),
        vec![PathBuf::from("merged/docs")],
        "a link is not the tree `materialize` writes, and its target is nobody's business \
         here — the entry comes back whole and undescended",
    );

    // (c) under `merged/docs/`, a DIRECTORY and a SYMLINK wearing a staged identity's name
    //     are both foreign — jigc writes regular files there and never a link.
    let shaped = root.path().join("shaped");
    let docs = shaped.join("merged").join("docs");
    fs::create_dir_all(&docs).expect("merged/docs");
    fs::write(docs.join("adr:real-one.md"), "# Real\n").expect("a real body");
    fs::create_dir_all(docs.join("adr:a-directory.md")).expect("dir wearing an identity");
    std::os::unix::fs::symlink(outside.join("elsewhere.md"), docs.join("adr:a-link.md"))
        .expect("symlink wearing an identity");
    assert_eq!(
        engine::state::foreign_area_paths(&shaped, engine::state::WorkArea::Milestone)
            .expect("enumerates"),
        vec![
            PathBuf::from("merged/docs/adr:a-directory.md"),
            PathBuf::from("merged/docs/adr:a-link.md"),
        ],
        "a directory named `<type>:<slug>.md` was a staged identity at four surfaces until \
         M52 Increment 4 / T1 — the shape question is asked here too",
    );
}

// ---------------------------------------------------------------------------
// The consent gate, driven through the real binary at both doors that have one
// ---------------------------------------------------------------------------

/// **`jigc milestone discard` refuses over every locus and narrates every one under
/// `--force`** — at its shipped `milestone.foreign-bytes` identity, no new code.
#[test]
fn milestone_discard_refuses_over_every_merged_plant_and_narrates_under_force() {
    let (repo_dir, home_dir) = post_join_area("discard");
    let (repo, home) = (repo_dir.path(), home_dir.path());
    let area = milestone_area(repo);
    plant_six(&area);

    let refused = jigc(repo, home, &["milestone", "discard", MILESTONE]);
    let text = both_streams(&refused);
    assert!(
        !refused.status.success(),
        "the abandon door must refuse over bytes jigc did not write; got:\n{text}",
    );
    assert!(
        text.contains("milestone.foreign-bytes"),
        "…under its shipped identity, not a new one; got:\n{text}",
    );
    for (cell, why) in PLANTS {
        assert!(
            text.contains(&format!(".jigc/milestones/{MILESTONE}/{cell}")),
            "the refusal names `{cell}` ({why}); got:\n{text}",
        );
    }
    for (cell, _) in PLANTS {
        assert!(
            area.join(cell).exists(),
            "a refusal takes nothing — `{cell}` is gone",
        );
    }

    let forced = jigc(repo, home, &["milestone", "discard", MILESTONE, "--force"]);
    let stderr = String::from_utf8_lossy(&forced.stderr).into_owned();
    assert!(
        forced.status.success(),
        "the printed consent runs as printed; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&forced.stdout),
    );
    for (cell, why) in PLANTS {
        assert!(
            stderr.contains(&format!(".jigc/milestones/{MILESTONE}/{cell}")),
            "`--force` narrates every path it took — `{cell}` ({why}) is missing from:\n{stderr}",
        );
    }
    assert!(
        !area.exists(),
        "and the consented abandon really took the area"
    );
}

/// **`jigc uninstall` refuses over every locus and narrates every one under `--force`** — at
/// its shipped `uninstall.foreign-bytes` identity.
#[test]
fn uninstall_refuses_over_every_merged_plant_and_narrates_under_force() {
    let (repo_dir, home_dir) = post_join_area("uninstall");
    let (repo, home) = (repo_dir.path(), home_dir.path());
    let area = milestone_area(repo);
    plant_six(&area);

    let refused = jigc(repo, home, &["uninstall"]);
    let text = both_streams(&refused);
    assert!(
        !refused.status.success(),
        "the teardown must refuse over bytes jigc did not write; got:\n{text}",
    );
    assert!(
        text.contains("uninstall.foreign-bytes"),
        "…under its shipped identity; got:\n{text}",
    );
    for (cell, why) in PLANTS {
        assert!(
            text.contains(&format!(".jigc/milestones/{MILESTONE}/{cell}")),
            "the refusal names `{cell}` ({why}); got:\n{text}",
        );
    }
    for (cell, _) in PLANTS {
        assert!(
            area.join(cell).exists(),
            "a refusal takes nothing — `{cell}` is gone",
        );
    }

    let forced = jigc(repo, home, &["uninstall", "--force"]);
    let stderr = String::from_utf8_lossy(&forced.stderr).into_owned();
    assert!(
        forced.status.success(),
        "the printed consent runs as printed; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&forced.stdout),
    );
    for (cell, why) in PLANTS {
        assert!(
            stderr.contains(&format!(".jigc/milestones/{MILESTONE}/{cell}")),
            "`--force` narrates every path it took — `{cell}` ({why}) is missing from:\n{stderr}",
        );
    }
    assert!(
        !repo.join(".jigc").exists(),
        "and the consented teardown really took `.jigc/`",
    );
}

// ---------------------------------------------------------------------------
// T3 — the removal half: `unwind_area`'s milestone arm honours the same walk
// ---------------------------------------------------------------------------

/// Copy a tree verbatim. The pristine post-join area is expensive to build — a real
/// two-sub-task fan-out through the binary — and the unwind under test destroys what it is
/// handed, so each plant cell gets its own copy of the same area.
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mk copy root");
    for entry in fs::read_dir(from).expect("read the source tree") {
        let entry = entry.expect("entry");
        let shape = entry.file_type().expect("file type");
        let target = to.join(entry.file_name());
        if shape.is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).expect("copy file");
        }
    }
}

/// Plant **one** locus — [`plant_six`]'s single-cell twin, because the unwind's answer has to
/// be `Foreign` for each cell *on its own*: six plants at once would let any one cell carry
/// the other five, and the cell that actually matters is whichever one a real hook wrote.
fn plant_one(area: &Path, cell: &str) {
    let path = area.join(cell);
    if cell == "merged/sub" {
        fs::create_dir_all(&path).expect("plant a foreign directory");
        fs::write(path.join("nested.txt"), "deep\n").expect("nested");
        return;
    }
    fs::create_dir_all(path.parent().expect("a plant has a parent")).expect("mk plant parent");
    fs::write(&path, format!("FOREIGN — {cell}\n")).expect("plant");
}

/// Every regular file still under `area`, relative and sorted — what the unwind left.
fn remaining_files(area: &Path) -> Vec<PathBuf> {
    fn walk(root: &Path, at: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(at) else {
            return;
        };
        for entry in entries {
            let entry = entry.expect("entry");
            let shape = entry.file_type().expect("file type");
            if shape.is_dir() {
                walk(root, &entry.path(), out);
            } else {
                out.push(
                    entry
                        .path()
                        .strip_prefix(root)
                        .expect("under the area")
                        .to_path_buf(),
                );
            }
        }
    }
    let mut out = Vec::new();
    walk(area, area, &mut out);
    out.sort();
    out
}

/// **An ordinary post-join area unwinds whole.** The zero-false-fire control of the removal
/// half: a walk that kept one of jigc's own bodies would strand every rejected mint under an
/// id its identical re-run then refuses (M47 Inc 2, the reason this is not `remove_dir_all`
/// *and* the reason it must still remove everything jigc wrote).
#[test]
fn an_ordinary_post_join_area_unwinds_to_removed() {
    let (repo_dir, _home_dir) = post_join_area("unwind-control");
    let area = milestone_area(repo_dir.path());
    assert!(
        area.join("merged").join("docs").is_dir(),
        "the fixture must really hold the join's staging tree, else this control proves \
         nothing about the arm that walks it",
    );

    assert_eq!(
        engine::state::unwind_area(&area, engine::state::WorkArea::Milestone)
            .expect("the unwind reports"),
        engine::state::AreaUnwind::Removed,
        "an area holding nothing but jigc's own writes unwinds whole, `merged/` included; \
         what is left: {:?}",
        remaining_files(&area),
    );
    assert!(!area.exists(), "…and the area itself is gone");
}

/// **Each of the six loci survives the unwind on its own, and the area answers `Foreign`** —
/// the removal half of T2's membership rule, asked cell by cell over the same fixture.
///
/// Three properties ride together, because one without the others is the shape that shipped:
/// the plant is byte-intact (`remove_dir_all` took it), every `<type>:<slug>.md` body and
/// every registry-row file is gone (a walk that refuses too much strands the mint), and a
/// directory survives **iff** the plant is at or below it — one file, one answer, and the
/// area's own non-recursive `remove_dir` is where that answer is produced.
///
/// The `scratch.txt` cell is also done-criterion **(c)**: `merged/` holding only jigc's
/// bodies is fully removed while a plant at the area *root* still answers `Foreign`.
#[test]
fn every_merged_plant_survives_the_unwind_and_leaves_the_area_foreign() {
    let (repo_dir, _home_dir) = post_join_area("unwind-plants");
    let pristine = milestone_area(repo_dir.path());
    let cases = repo_dir.path().join("unwind-cases");

    for (index, (cell, why)) in PLANTS.iter().enumerate() {
        let area = cases.join(format!("case-{index}"));
        copy_tree(&pristine, &area);
        plant_one(&area, cell);

        let verdict = engine::state::unwind_area(&area, engine::state::WorkArea::Milestone)
            .expect("the unwind reports");
        let left = remaining_files(&area);
        assert_eq!(
            verdict,
            engine::state::AreaUnwind::Foreign,
            "`{cell}` ({why}) is a byte jigc did not write — the area survives and the \
             caller is told; what is left: {left:?}",
        );

        // (1) the plant is byte-intact, at the path it was planted at.
        if *cell == "merged/sub" {
            assert_eq!(
                fs::read_to_string(area.join(cell).join("nested.txt"))
                    .expect("the foreign directory's own file survives"),
                "deep\n",
                "a foreign directory is the unit a door names — it is never descended and \
                 never taken",
            );
        } else {
            assert_eq!(
                fs::read_to_string(area.join(cell)).expect("the plant survives"),
                format!("FOREIGN — {cell}\n"),
                "`{cell}` ({why}) must come through the unwind unread and unwritten",
            );
        }

        // (2) everything jigc wrote is gone — the bodies and the registry row alike.
        assert!(
            !left.iter().any(|p| p.to_string_lossy().contains(':')),
            "every materialized `<type>:<slug>.md` body is jigc's own and must go; left: \
             {left:?}",
        );
        for member in engine::state::MILESTONE_AREA_FILES {
            if *member == "merged" {
                continue;
            }
            assert!(
                !area.join(member).exists(),
                "`{member}` is a registry member — a mint this unwind refuses to clear is a \
                 mint whose identical re-run blocks on the id it already took",
            );
        }

        // (3) a directory survives iff the plant is at or below it.
        let under_merged = cell.starts_with("merged/");
        let under_docs = cell.starts_with("merged/docs/");
        assert_eq!(
            area.join("merged").exists(),
            under_merged,
            "`merged/` survives iff `{cell}` is inside it — done-criterion (c) is the \
             `scratch.txt` cell of this same rule; left: {left:?}",
        );
        assert_eq!(
            area.join("merged").join("docs").exists(),
            under_docs,
            "`merged/docs/` survives iff `{cell}` is inside it; left: {left:?}",
        );
    }
}
