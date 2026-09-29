//! M52 Increment 4 / T4 — **the milestone boundary keeps them too**
//! (`completions/artifacts/M52/settle-record.md` → §18, the human's (A) decision; §8's key
//! shape; `design/storage.md` → The per-task working area).
//!
//! T3 taught `jigc task finalize` to move a working area's complement aside instead of
//! destroying it. The **fifth** destroying door reaches the very same bytes one verb over:
//! a landed `jigc milestone finalize` removes each sub-task's `.jigc/tasks/<sub-id>/` area
//! through [`cleanup_subtask_areas`], which was a bare `remove_dir_all` at **both**
//! `finalize.fan-out.squash` arms. Driven at `83bed573`, a sub-task's `NOTES.md` and
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
//!
//! **M53 Increment 2 / T3 — the removal is conditioned on the move.** Until then the teardown
//! after the displacement was a `remove_dir_all`, so an entry whose move failed was destroyed
//! by the very removal the move existed to spare it from. The cells below drive that claim at
//! this door over the axis `{all · some · none} × {clean · fault on the pin · fault on a later
//! member}`, at **both** `finalize.fan-out.squash` arms, applying each manufacture to **both**
//! kinds of area the boundary settles — its own `milestones/<id>/` and every
//! `tasks/<sub-id>/`. The two kinds ride one run because they are one boundary: separating
//! them would be a second fan-out fixture asserting the identical predicate, and each cell
//! asserts them apart (one advisory per area, each keyed at its own work unit). Three named
//! cells sit beside the axis: **D1×D2** (a foreign byte under `merged/` whose move failed
//! survives a landed boundary — the cell that makes Increment 1's `unwind_merged` arm
//! reachable), the **`Take` carve-out** (`milestone discard --force` still takes and still
//! acks), and §13's post-join control.

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

/// The omitting cell — **and §13's third zero-false-fire control, driven end to end** (M53
/// Increment 2 / T3): an ordinary post-join milestone area, `merged/` included, unwinds whole
/// at a landed boundary, mints no advisory and leaves nothing behind. Its sibling controls (an
/// ordinary task lifecycle, a migrate task) are discharged at the per-task door, in
/// `crates/cli/tests/finalize_displacement.rs` →
/// `the_section_13_controls_mint_no_advisory_end_to_end`.
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

    // §13's control, the half that is this task's: every area the boundary settled unwound
    // WHOLE and no advisory was minted about any of them. The regression it catches is the
    // opposite of every cell above — `unwind_area` answering `Foreign` over an ordinary area
    // would leave every finalized milestone's workbench standing.
    for path in all_areas(repo.path()) {
        assert!(
            !path.exists(),
            "an area holding nothing but jigc's own writes unwinds whole: `{}` is gone;\n\
             stderr:\n{stderr}",
            path.display(),
        );
    }
    assert!(
        !stderr.contains(KEPT_AREA_CODE),
        "…and the ordinary boundary mints no `{KEPT_AREA_CODE}` at all;\nstderr:\n{stderr}",
    );
}

// ---------------------------------------------------------------------------
// M53 Increment 2 / T3 — the removal is conditioned on the move
// ---------------------------------------------------------------------------

/// The marker every T3 plant carries, so survival is claimed by a real `grep` over the whole
/// repository rather than by looking where the test expects the bytes to be — a before-control
/// runs the identical scan before the boundary, so a green cannot come from a scan that finds
/// nothing anywhere.
const KEEP_MARKER: &str = "JIGC-M53-KEEP";

/// The advisory this pass mints.
const KEPT_AREA_CODE: &str = "finalize.foreign-bytes";

/// A plant at an area's **root** — the depth every door has reached since M52.
const ROOT_KEEP: (&str, &str) = ("root-kept.txt", "JIGC-M53-KEEP root\n");

/// A plant one directory down, whose complement **entry** is the directory: it moves whole.
const NESTED_KEEP: (&str, &str) = ("analysis/perf.txt", "JIGC-M53-KEEP nested\n");

/// A sub-task area's plant inside the registry's one tree member.
const SUB_DOCS_KEEP: (&str, &str) = ("docs/non-md.txt", "JIGC-M53-KEEP under docs\n");

/// The **milestone** area's twin of [`SUB_DOCS_KEEP`], one level deeper: the join's staging
/// tree, beside the bodies `materialize` actually writes. This is D1's walk and D2's removal
/// meeting — cell **D1×D2**.
const MERGED_KEEP: (&str, &str) = (
    "merged/docs/deep.txt",
    "JIGC-M53-KEEP beside a materialized body\n",
);

/// How much of an area's complement reached `.jigc/displaced/<unit-id>/`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Moves {
    All,
    Partial,
    None,
}

/// What `engine::state::unwind_area` did with jigc's own members.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unwind {
    Clean,
    /// The removal failed on the registry row's member 0, the base pin — the area stands
    /// **whole**.
    FaultOnPin,
    /// The removal failed on a later member, after the pin was already gone: the task row's
    /// `docs/`, the milestone row's `merged/`.
    FaultOnLater,
}

/// One coordinate of the manufactured axis, and the manufacture it needs, applied to **both**
/// kinds of area this door settles — its own `milestones/<id>/` and every `tasks/<sub-id>/`.
///
/// **Manufactured, and it says so**: the two failure points are decided (the pin is member 0
/// of each registry row; the tree member is a later one), and the move axis is a property of
/// the parking home's state, which no registry enumerates.
struct BoundaryCell {
    name: &'static str,
    moves: Moves,
    unwind: Unwind,
    /// What is planted in each **sub-task** area.
    sub_plants: &'static [(&'static str, &'static str)],
    /// What is planted in the **milestone** area.
    milestone_plants: &'static [(&'static str, &'static str)],
    /// When set, every unit's parking home `.jigc/displaced/<unit-id>` is occupied by a
    /// regular file before the boundary runs, so no entry's parent can be created.
    block_parking: bool,
}

/// **The reachable coordinates** — seven of nine; [`UNREACHABLE`] names the other two.
const BOUNDARY_CELLS: &[BoundaryCell] = &[
    BoundaryCell {
        name: "all move × unwind clean",
        moves: Moves::All,
        unwind: Unwind::Clean,
        sub_plants: &[ROOT_KEEP, NESTED_KEEP],
        milestone_plants: &[ROOT_KEEP, MERGED_KEEP],
        block_parking: false,
    },
    BoundaryCell {
        name: "some move × unwind clean",
        moves: Moves::Partial,
        unwind: Unwind::Clean,
        sub_plants: &[ROOT_KEEP, SUB_DOCS_KEEP],
        milestone_plants: &[ROOT_KEEP, MERGED_KEEP],
        // Manufactured per area by `occupy_partial`: one entry's parking PARENT is a regular
        // file, so exactly that entry's move cannot land.
        block_parking: false,
    },
    BoundaryCell {
        name: "none move × unwind clean",
        moves: Moves::None,
        unwind: Unwind::Clean,
        sub_plants: &[ROOT_KEEP, NESTED_KEEP],
        milestone_plants: &[ROOT_KEEP, MERGED_KEEP],
        block_parking: true,
    },
    BoundaryCell {
        name: "all move × fault on a later member",
        moves: Moves::All,
        unwind: Unwind::FaultOnLater,
        sub_plants: &[ROOT_KEEP, NESTED_KEEP],
        milestone_plants: &[ROOT_KEEP],
        block_parking: false,
    },
    BoundaryCell {
        name: "some move × fault on a later member",
        moves: Moves::Partial,
        unwind: Unwind::FaultOnLater,
        // No parking game: the plant inside the tree member cannot move because the tree
        // member is the directory the fault is made in, and a rename needs the SOURCE
        // directory writable.
        sub_plants: &[ROOT_KEEP, SUB_DOCS_KEEP],
        milestone_plants: &[ROOT_KEEP, MERGED_KEEP],
        block_parking: false,
    },
    BoundaryCell {
        name: "none move × fault on a later member",
        moves: Moves::None,
        unwind: Unwind::FaultOnLater,
        sub_plants: &[ROOT_KEEP, NESTED_KEEP],
        milestone_plants: &[ROOT_KEEP],
        block_parking: true,
    },
    BoundaryCell {
        name: "none move × fault on the pin",
        moves: Moves::None,
        unwind: Unwind::FaultOnPin,
        // No parking game either: an area that cannot be written to is an area no entry can
        // be renamed out of, which is the same bit the pin's removal needs.
        sub_plants: &[ROOT_KEEP, NESTED_KEEP],
        milestone_plants: &[ROOT_KEEP],
        block_parking: false,
    },
];

/// **The two coordinates no filesystem produces**, named rather than silently absent.
const UNREACHABLE: &[(&str, &str)] = &[
    (
        "all move × fault on the pin",
        "the pin's `remove_file` and every entry's `fs::rename` out of the area draw on the \
         SAME bit — write permission on the area — so an area whose pin cannot be removed is \
         an area no entry can leave",
    ),
    (
        "some move × fault on the pin",
        "the same bit, for the same reason: with the area writable the pin goes, and without \
         it no entry moves",
    ),
];

/// Every file under `repo` carrying [`KEEP_MARKER`], repo-relative and sorted — the survival
/// scan, run through the real `grep`.
fn marker_files(repo: &Path) -> Vec<String> {
    let out = Command::new("grep")
        .args(["-rlE", KEEP_MARKER, "."])
        .current_dir(repo)
        .output()
        .expect("run grep");
    let mut found: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|line| line.trim_start_matches("./").to_owned())
        .filter(|line| !line.is_empty())
        .collect();
    found.sort();
    found
}

/// Whether `path` — a file, or a directory that moved whole — carries [`KEEP_MARKER`]
/// somewhere under it. Every plant does; nothing jigc writes does.
fn holds_marker(path: &Path) -> bool {
    Command::new("grep")
        .args(["-rqE", KEEP_MARKER])
        .arg(path)
        .output()
        .expect("run grep")
        .status
        .success()
}

/// **Every complement entry of a standing area is a third party's byte** — the assertion the
/// arm was missing, and the one that would have caught the M53 completion audit's fix 1.
///
/// The cells above all ask what happens to a *plant*. The complement is also what the
/// advisory **names** and what every later destroying door **refuses over**, so a file jigc
/// itself wrote that is absent from its own registry row is a lie on one surface and a dead
/// end at three doors. Driven at `f664863a`, the `fault on the pin` cell left
/// `finalize-message.tmp` — the shared finalize executor's commit-message transient, whose
/// `msg_tmp_dir` **is** this very area — in the complement of all three standing areas, at
/// exit 0, named as *a path jigc did not write*.
///
/// The predicate is the marker rather than a per-cell expected set: the plants carry it and
/// jigc's own writes never do, so this stays exact as the cells' plant lists change.
fn assert_complement_is_all_planted(area: &Path, kind: engine::state::WorkArea, label: &str) {
    let complement =
        engine::state::foreign_area_paths(area, kind).expect("a standing area enumerates");
    let intruders: Vec<String> = complement
        .iter()
        .filter(|rel| !holds_marker(&area.join(rel)))
        .map(|rel| rel.display().to_string())
        .collect();
    assert!(
        intruders.is_empty(),
        "[{label}] the complement of `{}` holds {intruders:?}, which carry no plant marker — \
         a file jigc wrote that is on neither registry row is named on the advisory as a path \
         jigc did not write, and refuses every later destroying door over jigc's own bytes",
        area.display(),
    );
}

/// Set `path`'s mode, for the permission games the fault cells need.
///
/// **Declared bound, the one `leftover_probe_fail_closed.rs` already carries:** run as `root`,
/// permission bits do not bind and the fault cells would not fault — the assertions below then
/// fail loudly rather than passing vacuously, because each one names the area it expects to
/// still be there.
fn set_mode(path: &Path, mode: u32) {
    let mut perms = fs::metadata(path)
        .unwrap_or_else(|err| panic!("stat {}: {err}", path.display()))
        .permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, mode);
    fs::set_permissions(path, perms)
        .unwrap_or_else(|err| panic!("chmod {}: {err}", path.display()));
}

/// Plant `plants` into `area`.
fn plant_into(area: &Path, plants: &[(&str, &str)]) {
    for (rel, body) in plants {
        let at = area.join(rel);
        if let Some(parent) = at.parent() {
            fs::create_dir_all(parent).expect("mkdir plant parent");
        }
        fs::write(&at, body).expect("write plant");
    }
}

/// Occupy `<repo>/.jigc/displaced/<unit>` with a regular file, so no entry's parking parent
/// can be created and **nothing** moves.
fn block_parking_home(repo: &Path, unit: &str) {
    let home = repo.join(".jigc").join("displaced");
    fs::create_dir_all(&home).expect("mk parking root");
    fs::write(home.join(unit), "not a directory\n").expect("occupy the parking home");
}

/// Occupy one entry's parking **parent** with a regular file, so exactly that entry's move
/// cannot land while every other entry's can — the `some` value of the move axis.
fn occupy_partial(repo: &Path, unit: &str, parent_rel: &str) {
    let at = repo
        .join(".jigc")
        .join("displaced")
        .join(unit)
        .join(parent_rel);
    fs::create_dir_all(at.parent().expect("the occupant has a parent")).expect("mk parking home");
    fs::write(&at, "not a directory\n").expect("occupy the parking parent");
}

/// Install a `pre-commit` hook that chmods each `target` to 0555 **inside** the transaction —
/// the only moment at which the fault can be made, since the finalize writes its message temp
/// file into the milestone area before the commit and a mode set beforehand would fail the run
/// instead of the teardown.
fn install_faulting_hook(repo: &Path, targets: &[PathBuf]) {
    let hooks = repo.join(".git").join("hooks");
    fs::create_dir_all(&hooks).expect("mk hooks dir");
    let mut body = String::from("#!/bin/sh\n");
    for target in targets {
        body.push_str(&format!("chmod 0555 {} 2>/dev/null\n", target.display()));
    }
    body.push_str("exit 0\n");
    let hook = hooks.join("pre-commit");
    fs::write(&hook, body).expect("install the faulting hook");
    set_mode(&hook, 0o755);
}

/// The tree member whose removal is the later-member fault, per kind.
fn later_member(area: &Path, milestone: bool) -> PathBuf {
    if milestone {
        area.join("merged").join("docs")
    } else {
        area.join("docs")
    }
}

/// Every area this boundary settles — the milestone's own and each sub-task's.
fn all_areas(repo: &Path) -> Vec<PathBuf> {
    let mut areas = vec![milestone_area(repo)];
    areas.extend(SUBS.iter().map(|sub| area(repo, sub)));
    areas
}

/// Drive one cell through a real landed `jigc milestone finalize` at `squash`, and assert it.
fn assert_boundary_cell(cell: &BoundaryCell, squash: bool) {
    let label = format!("{} · squash: {squash}", cell.name);
    let root = TempDir::new("t3");
    let repo = root.path();
    init_repo(repo);
    let home = TempDir::new("home");
    setup_fanout(repo, home.path(), squash, false);

    for sub in SUBS {
        plant_into(&area(repo, sub), cell.sub_plants);
    }
    plant_into(&milestone_area(repo), cell.milestone_plants);

    if cell.block_parking {
        block_parking_home(repo, MILESTONE);
        for sub in SUBS {
            block_parking_home(repo, sub);
        }
    }
    if cell.moves == Moves::Partial && cell.unwind == Unwind::Clean {
        for sub in SUBS {
            occupy_partial(repo, sub, "docs");
        }
        occupy_partial(repo, MILESTONE, "merged/docs");
    }

    let faulted: Vec<PathBuf> = match cell.unwind {
        Unwind::Clean => Vec::new(),
        Unwind::FaultOnPin => all_areas(repo),
        Unwind::FaultOnLater => {
            let mut targets = vec![later_member(&milestone_area(repo), true)];
            targets.extend(SUBS.iter().map(|sub| later_member(&area(repo, sub), false)));
            targets
        }
    };
    if !faulted.is_empty() {
        install_faulting_hook(repo, &faulted);
    }

    let planted = cell.sub_plants.len() * SUBS.len() + cell.milestone_plants.len();
    let before = marker_files(repo);
    assert_eq!(
        before.len(),
        planted,
        "[{label}] the before-control must FIND the plants — a scan that finds nothing before \
         proves nothing after; got {before:?}",
    );

    let finalized = run_milestone(
        repo,
        home.path(),
        &["finalize", MILESTONE, "--format", "json"],
    );
    // Hand the write bits back before anything else reads or removes the tree.
    for target in &faulted {
        if target.exists() {
            set_mode(target, 0o755);
        }
    }
    for path in all_areas(repo) {
        if path.exists() {
            set_mode(&path, 0o755);
        }
    }
    let stdout = String::from_utf8_lossy(&finalized.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&finalized.stderr).into_owned();

    // (1) The commit is truth and the teardown is best-effort, so the boundary lands at exit 0
    //     whatever its areas do.
    assert!(
        finalized.status.success(),
        "[{label}] the boundary must land at exit 0;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // (2) Every planted byte is still on disk — moved aside or left standing, never taken.
    let after = marker_files(repo);
    assert_eq!(
        after.len(),
        before.len(),
        "[{label}] every planted byte survives the landed boundary; before {before:?}, after \
         {after:?}\nstderr:\n{stderr}",
    );

    // (3) The cell's own MOVE coordinate, asserted rather than assumed: a manufacture that
    //     silently stopped working would otherwise let a fault cell pass while testing a
    //     different coordinate. Each plant is exactly one complement entry here.
    let expected_moves = match cell.moves {
        Moves::All => planted,
        // One root entry per area lands; the one inside each area's tree member does not.
        Moves::Partial => SUBS.len() + 1,
        Moves::None => 0,
    };
    assert_eq!(
        envelope_pairs(&stdout).len(),
        expected_moves,
        "[{label}] the cell's move coordinate must really be the one it claims;\n\
         stdout:\n{stdout}",
    );

    // (4) Exactly one advisory per area left standing, on stderr — this door's landed arm
    //     carries no `findings` key.
    let standing: Vec<PathBuf> = all_areas(repo).into_iter().filter(|a| a.exists()).collect();
    let expected_standing = if cell.moves == Moves::All && cell.unwind == Unwind::Clean {
        0
    } else {
        1 + SUBS.len()
    };
    assert_eq!(
        standing.len(),
        expected_standing,
        "[{label}] an area jigc could not empty of a third party's bytes is LEFT, and one it \
         emptied is gone; standing: {standing:?}\nstderr:\n{stderr}",
    );
    assert_eq!(
        stderr.matches(KEPT_AREA_CODE).count(),
        expected_standing,
        "[{label}] exactly one `{KEPT_AREA_CODE}` per area left standing, on stderr;\n\
         stderr:\n{stderr}",
    );
    for path in &standing {
        let unit = path
            .file_name()
            .expect("an area has a name")
            .to_string_lossy();
        let target = if path.starts_with(repo.join(".jigc").join("milestones")) {
            format!("milestone:{unit}")
        } else {
            format!("task:{unit}")
        };
        assert!(
            stderr.contains(&target),
            "[{label}] …each keyed at its OWN work unit (`{target}`), so a boundary that \
             settles three areas hands a reader three keys;\nstderr:\n{stderr}",
        );
    }

    // (5) The pinned landed envelope does not move for the advisory.
    let envelope: serde_json::Value =
        serde_json::from_str(&stdout).expect("the landed envelope is one JSON value");
    let mut keys: Vec<String> = envelope
        .as_object()
        .expect("the landed envelope is an object")
        .keys()
        .cloned()
        .collect();
    keys.sort();
    assert_eq!(
        keys,
        vec!["committed".to_string()],
        "[{label}] the landed `milestone finalize` envelope stays \
         `Object(&[\"committed\"])`;\nstdout:\n{stdout}",
    );

    // (6) The pin's fate is what separates the two fault cells, and it is what a by-id door
    //     reads: a whole area with its pin, or a residual.
    for path in &standing {
        assert_eq!(
            path.join("base.json").exists(),
            cell.unwind == Unwind::FaultOnPin,
            "[{label}] a fault ON the pin leaves `{}` whole; a clean unwind and a fault on a \
             LATER member both take the pin first",
            path.display(),
        );
    }

    // (7) …and nothing JIGC wrote is in the complement of an area left standing
    //     (M53 completion audit, fix 1 — see `assert_complement_is_all_planted`).
    for path in &standing {
        let kind = if path.starts_with(repo.join(".jigc").join("milestones")) {
            engine::state::WorkArea::Milestone
        } else {
            engine::state::WorkArea::Task
        };
        assert_complement_is_all_planted(path, kind, &label);
    }
}

/// The axis's gaps are named — the seven driven cells read as seven of nine.
#[test]
fn the_boundary_axis_names_its_unreachable_coordinates() {
    assert_eq!(
        BOUNDARY_CELLS.len() + UNREACHABLE.len(),
        9,
        "the manufactured space is `{{all · partial · none}} × {{clean · fault on the pin · \
         fault on a later member}}` — nine coordinates, each either driven or declared \
         unreachable",
    );
    for (name, why) in UNREACHABLE {
        assert!(
            !why.trim().is_empty(),
            "`{name}` must carry the reason it cannot be manufactured",
        );
        assert!(
            !BOUNDARY_CELLS.iter().any(|cell| cell.name == *name),
            "`{name}` is declared unreachable and driven — it is one or the other",
        );
    }
}

/// Drive one cell at **both** `finalize.fan-out.squash` arms — the axis the sink has.
fn assert_both_arms(cell: &BoundaryCell) {
    assert_boundary_cell(cell, true);
    assert_boundary_cell(cell, false);
}

#[test]
fn boundary_areas_all_move_unwind_clean() {
    assert_both_arms(&BOUNDARY_CELLS[0]);
}

#[test]
fn boundary_areas_some_move_unwind_clean() {
    assert_both_arms(&BOUNDARY_CELLS[1]);
}

#[test]
fn boundary_areas_none_move_unwind_clean() {
    assert_both_arms(&BOUNDARY_CELLS[2]);
}

#[test]
fn boundary_areas_all_move_fault_on_a_later_member() {
    assert_both_arms(&BOUNDARY_CELLS[3]);
}

#[test]
fn boundary_areas_some_move_fault_on_a_later_member() {
    assert_both_arms(&BOUNDARY_CELLS[4]);
}

#[test]
fn boundary_areas_none_move_fault_on_a_later_member() {
    assert_both_arms(&BOUNDARY_CELLS[5]);
}

#[test]
fn boundary_areas_none_move_fault_on_the_pin() {
    assert_both_arms(&BOUNDARY_CELLS[6]);
}

/// **Cell D1×D2** (M53 Increment 2 / T3; `completions/artifacts/M53/settle-record.md` → D1.4,
/// D2's acceptance axis) — *a foreign byte under `merged/` whose move failed survives a landed
/// `milestone finalize`*.
///
/// This is where Increment 1's widened walk and Increment 2's conditioned removal meet, and
/// it is the cell that makes Increment 1 / T3's `unwind_merged` arm **reachable**: that arm
/// was conceded unreachable at HEAD, and true only until this task replaced the boundary's
/// `remove_dir_all` with the unwind. Before it, a `merged/docs/deep.txt` whose move could not
/// land was taken by the teardown a statement later, at exit 0, out of a gitignored tree with
/// no second copy.
///
/// The plant is *only* under `merged/`, so nothing at the area root can carry the claim.
#[test]
fn a_failed_merged_move_survives_a_landed_boundary() {
    let root = TempDir::new("d1xd2");
    let repo = root.path();
    init_repo(repo);
    let home = TempDir::new("home");
    setup_fanout(repo, home.path(), true, false);

    plant_into(&milestone_area(repo), &[MERGED_KEEP]);
    // The move cannot land: the entry's parking parent is a regular file.
    occupy_partial(repo, MILESTONE, "merged/docs");

    let before = marker_files(repo);
    assert_eq!(
        before,
        vec![format!(".jigc/milestones/{MILESTONE}/{}", MERGED_KEEP.0)],
        "the before-control finds the plant exactly where it was planted",
    );

    let finalized = run_milestone(
        repo,
        home.path(),
        &["finalize", MILESTONE, "--format", "json"],
    );
    let stdout = String::from_utf8_lossy(&finalized.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&finalized.stderr).into_owned();
    assert!(
        finalized.status.success(),
        "the boundary lands at exit 0;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert_eq!(
        marker_files(repo),
        before,
        "the byte is still exactly where it was — its move failed, so the teardown left it \
         rather than taking it;\nstderr:\n{stderr}",
    );
    assert!(
        milestone_area(repo).join(MERGED_KEEP.0).exists(),
        "…and `merged/` itself survives with it, because `remove_dir` refuses a non-empty \
         directory;\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains(KEPT_AREA_CODE) && stderr.contains(&format!("milestone:{MILESTONE}")),
        "one advisory names the milestone's own area;\nstderr:\n{stderr}",
    );
    assert!(
        !milestone_area(repo).join("tasks.json").exists(),
        "jigc's own members are still removed — the unwind is conditioned on the MOVE, not \
         skipped wholesale",
    );
}

/// **The `Take` carve-out** (`settle-record.md` → §3) — `jigc milestone discard --force` still
/// removes a sub-task area's foreign byte and still acks *workbench removed*.
///
/// `cleanup_subtask_areas` has three callers on two kinds of path, and the disposition is read
/// off the **call**, never off the function: the two landed boundary arms displace-then-unwind,
/// and this one takes. Its own doc-comment says why — *"a disposition read off the function
/// instead of the call would silently re-decide `jigc milestone discard`"* — and `--force` at
/// that door **is** the consent for exactly these bytes, given after its guard named them.
/// Unwinding here instead would leave the operator's abandoned workbench standing after they
/// consented to its removal.
#[test]
fn milestone_discard_force_still_takes_a_sub_task_areas_foreign_byte() {
    let root = TempDir::new("take");
    let repo = root.path();
    init_repo(repo);
    let home = TempDir::new("home");
    setup_fanout(repo, home.path(), true, false);
    for sub in SUBS {
        plant_into(&area(repo, sub), &[ROOT_KEEP]);
    }

    let before = marker_files(repo);
    assert_eq!(
        before.len(),
        SUBS.len(),
        "the before-control finds one plant per sub-task area; got {before:?}",
    );

    let discarded = run_milestone(repo, home.path(), &["discard", MILESTONE, "--force"]);
    let stdout = String::from_utf8_lossy(&discarded.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&discarded.stderr).into_owned();
    assert!(
        discarded.status.success(),
        "the discard must settle the record;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("workbench removed"),
        "…and it still acks the teardown it performed;\nstdout:\n{stdout}",
    );
    for sub in SUBS {
        assert!(
            !area(repo, sub).exists(),
            "`{sub}`'s area is gone — this door TAKES the complement under its own consent",
        );
    }
    assert!(
        marker_files(repo).is_empty(),
        "the consented bytes are gone, and none was parked: the `Displace` arm's keep is not \
         this caller's disposition",
    );
    assert!(
        !stderr.contains(KEPT_AREA_CODE),
        "…and no landed-boundary advisory is minted here at all;\nstderr:\n{stderr}",
    );
}
