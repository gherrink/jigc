//! M52 Increment 4 / T4 — **the milestone boundary keeps them too**
//! (`completions/artifacts/M52/settle-record.md` → §18, the human's (A) decision; §8's key
//! shape; `design/storage.md` → The per-task working area).
//!
//! T3 taught `jigc task finalize` to move a working area's complement aside instead of
//! destroying it. The **fifth** destroying door reaches the very same bytes one verb over:
//! a landed `jigc milestone finalize` removes each sub-task's `.jigc/tasks/<sub-id>/` area
//! through [`cleanup_subtask_areas`], which was a bare `remove_dir_all` at **both**
//! `finalize.fan-out.squash` arms. Driven at `af0b5903`, a sub-task's `NOTES.md` and
//! `analysis/perf.txt` died there at **exit 0 with an empty stderr** — byte-identical bytes
//! to the ones the task door now keeps, on the door next to it.
//!
//! The landed-boundary warrant the *narrating* doors use (*the staged set is already in
//! git*) is as unavailable here as it was at T3: the boundary commit carries the promoted
//! docs, the merged record and the sub-agents' staged code, and takes nothing at all out of
//! a sub-task's working area. So the disposition is the same one, through the same
//! primitive — **displace**.
//!
//! **The axis this iterates is the landed-arm axis**, because it is the axis the sink has:
//! `cleanup_subtask_areas` has three call sites on two kinds of path, and the two *landed*
//! ones (`squash: true`'s single combine commit and `squash: false`'s N+1 chain) are the
//! two this task disposes. The third — `run_discard`'s abandon — keeps destroying and is
//! guarded at its own door (T5), which is why the disposition is a **parameter of the
//! call** and never a property of the function.
//!
//! …crossed with the output-surface axis, since the pairs ride **two** channels: stderr
//! under both formats (the loss-shaped side channel, so `--format json`'s document still
//! owns stdout undiluted) and the landed envelope's `committed.displaced`.
//!
//! **The omitting cell is the one that catches a subject cut too wide**: a boundary whose
//! sub-task areas hold nothing but jigc's own files carries `displaced: []` — present
//! always, so a driver never has to guess which of the two it is reading — and says nothing
//! on stderr at all.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The plants, in **every** sub-task area: one file at the area root and one nested a
/// directory deep. The nested one is the cell that proves the move is tree-preserving —
/// the complement's unit is the *entry*, so `analysis` moves whole and `perf.txt` survives
/// inside it.
const PLANTS: &[(&str, &str)] = &[
    ("NOTES.md", "keep me — the sub-agent's own scratch\n"),
    ("analysis/perf.txt", "p99 = 41ms\n"),
];

/// The two sub-tasks of the fixture fan-out.
const SUBS: &[&str] = &["area-low", "area-zed"];

/// The milestone the fixture mints.
const MILESTONE: &str = "cache-rework";

/// The plants in the **milestone's own** working area (M53 Increment 1 / T4;
/// `completions/artifacts/M53/settle-record.md` → D1.1) — one per shape the area's
/// complement has, because the milestone row's walk is two levels deep where the task row's
/// is one:
///
/// * the area **root**, the only depth M52 ever reached at the doors that guard this area;
/// * **`merged/` top**, the join's staging tree — jigc writes exactly one entry there, and
///   this is not it;
/// * **`merged/docs/adr.md`**, a `.md` with no `:` in it, so no call of
///   `engine::state::instance_path` could have produced the name;
/// * **`merged/docs/provenance.json`**, the *task* row's member deliberately not inherited
///   here — `materialize` writes no manifest, so this file is a third party's;
/// * **`merged/docs/deep.txt`**, beside the bodies `materialize` actually writes; and
/// * **`merged/scratch/perf.txt`**, a directory that must move **whole** with its subtree
///   preserved by the move itself.
const MILESTONE_PLANTS: &[(&str, &str)] = &[
    ("NOTES.md", "the operator's own milestone scratch\n"),
    ("merged/top.txt", "a third party at the join staging top\n"),
    ("merged/docs/adr.md", "# not a staged address\n"),
    ("merged/docs/provenance.json", "{\"docs\": {}}\n"),
    ("merged/docs/deep.txt", "beside a materialized body\n"),
    ("merged/scratch/perf.txt", "p99 = 41ms\n"),
];

/// The complement **entries** [`MILESTONE_PLANTS`] makes — the unit the door names, refuses
/// over or moves, sorted as `engine::state::foreign_area_paths` returns them. `merged/scratch`
/// is one entry, never `merged/scratch/perf.txt`: a foreign directory moves whole.
const MILESTONE_ENTRIES: &[&str] = &[
    "NOTES.md",
    "merged/docs/adr.md",
    "merged/docs/deep.txt",
    "merged/docs/provenance.json",
    "merged/scratch",
    "merged/top.txt",
];

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-boundary-displace-{tag}-{}-{:?}",
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

/// Run `git <args>` in `cwd`, asserting success.
fn git_ok(cwd: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// A real git repo with one commit (the milestone mint reads HEAD).
fn init_repo(root: &Path) {
    git_ok(root, &["init", "-q"]);
    git_ok(root, &["config", "user.email", "test@example.com"]);
    git_ok(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
    // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
    crate::support::mint_project_layer(root);
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`.
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("milestone")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// `.jigc/config/manifest.yaml` opting the project into per-sub-task commits.
fn set_squash_false(repo: &Path) {
    let config = repo.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("mk config layer");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  finalize.fan-out.squash: false\n",
    )
    .expect("write manifest");
}

fn area(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(sub)
}

fn displaced_home(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("displaced").join(sub)
}

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area.
fn stage_doc(repo: &Path, sub: &str, address: &str, body: &str) {
    let docs = area(repo, sub).join("docs");
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

/// A plain, ref-free ADR body.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a sub-task's authored `commit:<sub>` doc — the prose the per-sub-task render reads.
fn stage_subtask_commit(repo: &Path, sub: &str, summary: &str) {
    let body = format!(
        "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\n{summary}\n\n## Body\n\n\n\n## Trailers\n"
    );
    stage_doc(repo, sub, &format!("commit:{sub}"), &body);
}

/// Write + `git add` a file in a provisioned fan-out worktree, so the boundary has code to
/// land under both arms (the `squash: false` chain mints one commit per code-carrying
/// sub-task).
fn stage_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree parent");
    }
    fs::write(&p, body).expect("write worktree file");
    git_ok(&wt, &["add", rel]);
}

/// Plant [`PLANTS`] in one sub-task's working area.
fn plant_foreign(repo: &Path, sub: &str) {
    for (rel, body) in PLANTS {
        let p = area(repo, sub).join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).expect("mkdir plant parent");
        }
        fs::write(&p, body).expect("write plant");
    }
}

/// The **milestone's own** working area — the executor's `cleanup_dir` at both boundary arms.
fn milestone_area(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("milestones").join(MILESTONE)
}

/// Plant [`MILESTONE_PLANTS`] in the milestone area. Written **before** the boundary runs,
/// so each one also has to survive the join's own `merged/docs/` rebuild (M53 Increment 1 /
/// T1) on its way to the teardown that is this task's subject.
fn plant_milestone_foreign(repo: &Path) {
    for (rel, body) in MILESTONE_PLANTS {
        let p = milestone_area(repo).join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).expect("mkdir milestone plant parent");
        }
        fs::write(&p, body).expect("write milestone plant");
    }
}

/// The fan-out fixture: a milestone, two sub-tasks each with a persisted ADR + its authored
/// commit doc staged and one staged code file in its worktree — and, when `plant`, the
/// foreign bytes in **every** sub-task area.
fn setup_fanout(repo: &Path, home: &Path, squash: bool, plant: bool) {
    if !squash {
        set_squash_false(repo);
    }
    assert!(
        run_milestone(repo, home, &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in ["Area zed", "Area low"] {
        assert!(
            run_milestone(repo, home, &["add-task", "cache-rework", intent])
                .status
                .success(),
            "add-task `{intent}` must exit 0",
        );
    }
    stage_doc(repo, "area-low", "adr:low-policy", &adr_plain("Low policy"));
    stage_subtask_commit(repo, "area-low", "rework the low cache path");
    stage_doc(repo, "area-zed", "adr:zed-policy", &adr_plain("Zed policy"));
    stage_subtask_commit(repo, "area-zed", "rework the zed cache path");

    let provisioned = run_milestone(repo, home, &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    stage_code(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
    stage_code(repo, "area-zed", "src/zed.rs", "pub fn zed() {}\n");

    if plant {
        for sub in SUBS {
            plant_foreign(repo, sub);
        }
    }
}

/// Every `{from, to}` pair the landed envelope carries, as `(from, to)` strings.
fn envelope_pairs(stdout: &str) -> Vec<(String, String)> {
    let envelope: serde_json::Value =
        serde_json::from_str(stdout).expect("the landed envelope is valid JSON");
    envelope["committed"]["displaced"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("the landed envelope must carry `committed.displaced` as an array:\n{stdout}")
        })
        .iter()
        .map(|pair| {
            (
                pair["from"]
                    .as_str()
                    .expect("`from` is a string")
                    .to_owned(),
                pair["to"].as_str().expect("`to` is a string").to_owned(),
            )
        })
        .collect()
}

#[test]
fn a_landed_milestone_boundary_keeps_every_sub_task_area_byte_it_did_not_write() {
    // The landed-arm axis × the output-surface axis. Each cell is a fresh repo: a landed
    // boundary is terminal.
    for (squash, json) in [(true, false), (true, true), (false, false), (false, true)] {
        let label = format!(
            "squash: {squash}, format: {}",
            if json { "json" } else { "agent" }
        );
        let repo = TempDir::new(if squash { "squash" } else { "chain" });
        init_repo(repo.path());
        let home = TempDir::new("home");
        setup_fanout(repo.path(), home.path(), squash, true);

        let mut args = vec!["finalize", "cache-rework"];
        if json {
            args.extend(["--format", "json"]);
        }
        let finalized = run_milestone(repo.path(), home.path(), &args);
        let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
        let stderr = String::from_utf8(finalized.stderr).expect("utf-8 stderr");

        // (1) The boundary landed — this is the SUCCESS path, not a refusal.
        assert!(
            finalized.status.success(),
            "[{label}] the finalize must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
        );

        for sub in SUBS {
            // (2) The teardown still ran — the area is gone, so `jigc task list` cannot
            // report an active sub-task that finalized.
            assert!(
                !area(repo.path(), sub).exists(),
                "[{label}] the sub-task area `{sub}` must still be removed",
            );

            // (3) Every planted byte survives, byte-intact, with its relative path.
            for (rel, body) in PLANTS {
                let kept = displaced_home(repo.path(), sub).join(rel);
                let got = fs::read_to_string(&kept).unwrap_or_else(|err| {
                    panic!(
                        "[{label}] `{sub}`'s `{rel}` must survive at {kept:?}: {err}\n\
                         stderr:\n{stderr}"
                    )
                });
                assert_eq!(
                    &got, body,
                    "[{label}] `{sub}`'s `{rel}` must survive BYTE-INTACT",
                );
            }

            // (4) Each move is named on stderr — the same channel under both formats.
            for entry in ["NOTES.md", "analysis"] {
                let from = format!(".jigc/tasks/{sub}/{entry}");
                let to = format!(".jigc/displaced/{sub}/{entry}");
                assert!(
                    stderr.contains(&from) && stderr.contains(&to),
                    "[{label}] stderr must name the move `{from}` → `{to}`; got:\n{stderr}",
                );
            }
        }

        // (5) The envelope carries the identical pairs — repo-relative, sorted by `from`,
        // one per complement ENTRY (`analysis`, never `analysis/perf.txt`).
        if json {
            let pairs = envelope_pairs(&stdout);
            let expected: Vec<(String, String)> = SUBS
                .iter()
                .flat_map(|sub| {
                    ["NOTES.md", "analysis"].into_iter().map(move |entry| {
                        (
                            format!(".jigc/tasks/{sub}/{entry}"),
                            format!(".jigc/displaced/{sub}/{entry}"),
                        )
                    })
                })
                .collect();
            let mut expected_sorted = expected.clone();
            expected_sorted.sort();
            assert_eq!(
                pairs, expected_sorted,
                "[{label}] `committed.displaced` must carry every move, sorted by `from`; \
                 stdout:\n{stdout}",
            );
        }

        // (6) jigc's OWN files are never moved aside — a subject cut one member too wide
        // would park `docs/` or `provenance.json` here.
        for sub in SUBS {
            assert!(
                !displaced_home(repo.path(), sub).join("docs").exists(),
                "[{label}] jigc's own `docs/` tree must never be displaced",
            );
        }
    }
}

/// **M53 Increment 1 / T4 — the boundary answers for its OWN area too**
/// (`completions/artifacts/M53/settle-record.md` → D1.1).
///
/// M52's T4 disposed the sub-task areas and passed `None` for the executor's own
/// `cleanup_dir` on the stated ground that *"this door must not answer for a subject it was
/// not given"*. The subject **was** given: `cleanup_dir` at both milestone call sites IS the
/// milestone area, and phase 7 `remove_dir_all`s it. So the sentence bought a landed boundary
/// destroying an operator's bytes at exit 0, with an empty stderr, while printing
/// `"displaced": []` on a 1.0-pinned envelope — the same loss the task door next to it had
/// just been taught not to commit.
///
/// The axes are the two the sink has — the landed-arm axis (`finalize.fan-out.squash`, both
/// arms reaching the same executor) crossed with the output-surface axis — over the area's
/// **shape** axis, [`MILESTONE_PLANTS`], which is the one that is genuinely new: the
/// milestone row's walk descends two levels, so a plant at `merged/docs/` is as reachable as
/// one at the root and neither was kept.
///
/// The **union** is the other half: one boundary settles its own area and N sub-task areas,
/// and `committed.displaced` is one key over all of them, sorted by `from`.
#[test]
fn a_landed_milestone_boundary_keeps_every_byte_of_its_own_area_it_did_not_write() {
    for (squash, json) in [(true, false), (true, true), (false, false), (false, true)] {
        let label = format!(
            "squash: {squash}, format: {}",
            if json { "json" } else { "agent" }
        );
        let repo = TempDir::new(if squash { "own-squash" } else { "own-chain" });
        init_repo(repo.path());
        let home = TempDir::new("home");
        // Both populations planted: the union assertion below is only honest if the
        // sub-task pairs the boundary already carried are still in it.
        setup_fanout(repo.path(), home.path(), squash, true);
        plant_milestone_foreign(repo.path());

        let mut args = vec!["finalize", MILESTONE];
        if json {
            args.extend(["--format", "json"]);
        }
        let finalized = run_milestone(repo.path(), home.path(), &args);
        let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
        let stderr = String::from_utf8(finalized.stderr).expect("utf-8 stderr");

        // (1) The SUCCESS path: the plants are kept by a boundary that landed, and they did
        // not block it on their way — a refusal here would prove nothing about the teardown.
        assert!(
            finalized.status.success(),
            "[{label}] the finalize must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
        );

        // (2) The teardown still ran: the milestone area is gone.
        assert!(
            !milestone_area(repo.path()).exists(),
            "[{label}] the milestone area must still be removed",
        );

        // (3) Every planted byte survives, byte-intact, with its relative path preserved —
        // `merged/scratch/perf.txt` included, which rides its parent entry's move.
        let parked = repo.path().join(".jigc").join("displaced").join(MILESTONE);
        for (rel, body) in MILESTONE_PLANTS {
            let kept = parked.join(rel);
            let got = fs::read_to_string(&kept).unwrap_or_else(|err| {
                panic!("[{label}] `{rel}` must survive at {kept:?}: {err}\nstderr:\n{stderr}")
            });
            assert_eq!(&got, body, "[{label}] `{rel}` must survive BYTE-INTACT");
        }

        // (4) Each move is named on stderr, under both formats — the loss-shaped side
        // channel, so `--format json`'s document still owns stdout undiluted.
        for entry in MILESTONE_ENTRIES {
            let from = format!(".jigc/milestones/{MILESTONE}/{entry}");
            let to = format!(".jigc/displaced/{MILESTONE}/{entry}");
            assert!(
                stderr.contains(&from) && stderr.contains(&to),
                "[{label}] stderr must name the move `{from}` → `{to}`; got:\n{stderr}",
            );
        }

        // (5) The envelope carries the UNION — this area's entries beside every sub-task's,
        // one key for the whole boundary, sorted by `from`.
        if json {
            let mut expected: Vec<(String, String)> = MILESTONE_ENTRIES
                .iter()
                .map(|entry| {
                    (
                        format!(".jigc/milestones/{MILESTONE}/{entry}"),
                        format!(".jigc/displaced/{MILESTONE}/{entry}"),
                    )
                })
                .chain(SUBS.iter().flat_map(|sub| {
                    ["NOTES.md", "analysis"].into_iter().map(move |entry| {
                        (
                            format!(".jigc/tasks/{sub}/{entry}"),
                            format!(".jigc/displaced/{sub}/{entry}"),
                        )
                    })
                }))
                .collect();
            expected.sort();
            assert_eq!(
                envelope_pairs(&stdout),
                expected,
                "[{label}] `committed.displaced` must be the union over the milestone area \
                 and every sub-task area, sorted by `from`; stdout:\n{stdout}",
            );
        }

        // (6) jigc's OWN milestone-area files are never parked — a subject cut one member
        // too wide would move the join's materialized bodies or the task list out of the
        // area and call it a rescue.
        assert!(
            !parked.join("tasks.json").exists(),
            "[{label}] the task list is jigc's own and is never parked",
        );
        assert!(
            !parked
                .join("merged")
                .join("docs")
                .join("adr:low-policy.md")
                .exists(),
            "[{label}] a materialized body is jigc's own and is never parked",
        );
    }
}

#[test]
fn a_landed_boundary_with_no_foreign_byte_carries_an_empty_displaced_key() {
    let repo = TempDir::new("clean");
    init_repo(repo.path());
    let home = TempDir::new("home");
    setup_fanout(repo.path(), home.path(), true, false);

    let finalized = run_milestone(
        repo.path(),
        home.path(),
        &["finalize", "cache-rework", "--format", "json"],
    );
    let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(finalized.stderr).expect("utf-8 stderr");
    assert!(
        finalized.status.success(),
        "the finalize must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // Present always — an absent key would leave a driver guessing which of the two states
    // it is reading.
    assert_eq!(
        envelope_pairs(&stdout),
        Vec::<(String, String)>::new(),
        "an ordinary boundary carries `displaced: []`; stdout:\n{stdout}",
    );
    assert!(
        !stderr.contains("moved aside"),
        "the omitting context prints nothing at all; stderr:\n{stderr}",
    );
    assert!(
        !repo.path().join(".jigc").join("displaced").exists(),
        "nothing moved, so the parking home is never created",
    );
}
